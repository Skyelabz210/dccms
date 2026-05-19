//! Calibration tool — for our specific SLUB + FAMSI datasets only.
//!
//! Three jobs in one pass:
//!
//! 1. **SLUB red-barrier color sampling** — load SLUB page 16, sample
//!    pixels along a row where the visual scan confirmed a red barrier,
//!    report the (R, G, B) distribution. Tells us the correct thresholds
//!    for `RegisterAwareSegmenter` on the actual SLUB color palette.
//!
//! 2. **FAMSI → SLUB mapping by vertical-darkness fingerprint** —
//!    compute a 32-bin vertical row-darkness histogram for each page in
//!    both sets, normalize, compute cross-source correlations, and
//!    output the empirical mapping FAMSI extracted index → SLUB page.
//!
//! 3. **Refined blank-like classification** — apply the tightened
//!    threshold (components<3000 AND max<100000) and report which SLUB
//!    pages flag.
//!
//! Build/run:
//! ```text
//! cargo run --release --features slub --example calibrate
//! ```

#![cfg(feature = "slub")]

use dccms_atlas::paths::{slub_page as slub_path, famsi_page as famsi_path};
use dccms_atlas::segmenter::{
    slub::load_slub_page,
    threshold::DarknessThresholdSegmenter,
    register::RegisterAwareSegmenter,
    ImageBuffer, Segmenter,
};

/// Sample a horizontal row in a page; return per-pixel (r,g,b) triples.
fn sample_row(img: &ImageBuffer, y: u32, x_min: u32, x_max: u32) -> Vec<(u8, u8, u8)> {
    let mut out = Vec::new();
    if img.bytes_per_pixel < 3 { return out; }
    let bpp = img.bytes_per_pixel as usize;
    let w = img.width as usize;
    for x in x_min..x_max.min(img.width) {
        let off = ((y as usize) * w + (x as usize)) * bpp;
        if off + 2 < img.data.len() {
            out.push((img.data[off], img.data[off + 1], img.data[off + 2]));
        }
    }
    out
}

/// Find horizontal rows in a page that appear to be red-rich (more than
/// the given fraction of pixels are visually red). Returns the y-indices.
/// Uses a permissive initial filter to discover the actual red color.
fn find_visually_red_rows(img: &ImageBuffer, sample_every: u32) -> Vec<u32> {
    let bpp = img.bytes_per_pixel as usize;
    let w = img.width as usize;
    let mut out = Vec::new();
    let permissive = |r: u8, g: u8, b: u8| -> bool {
        // Permissive: R distinctly higher than G and B by SOMETHING, and
        // R not too dark. We'll narrow down by sampling.
        r > 100 && r > g + 20 && r > b + 20
    };
    for y in (0..img.height).step_by(sample_every as usize) {
        let mut red_count = 0u32;
        for x in 0..img.width {
            let off = ((y as usize) * w + (x as usize)) * bpp;
            if off + 2 >= img.data.len() { continue; }
            if permissive(img.data[off], img.data[off + 1], img.data[off + 2]) {
                red_count += 1;
            }
        }
        if red_count > (img.width / 3) {
            out.push(y);
        }
    }
    out
}

/// Quartile of a sorted slice.
fn q(sorted: &[u8], num: u32, den: u32) -> u8 {
    let idx = ((sorted.len() as u32 * num) / den) as usize;
    sorted[idx.min(sorted.len() - 1)]
}

/// 32-bin vertical row-darkness histogram (resampled, integer-only).
fn vertical_fingerprint(img: &ImageBuffer) -> [u64; 32] {
    let mut bins = [0u64; 32];
    let w = img.width as usize;
    let bpp = img.bytes_per_pixel as usize;
    let h = img.height as usize;
    if h == 0 { return bins; }
    for y in 0..h {
        let bin = ((y as u64) * 32) / (h as u64);
        let bin = bin.min(31) as usize;
        for x in 0..w {
            let off = (y * w + x) * bpp;
            if off + 2 >= img.data.len() { continue; }
            let sum = (img.data[off] as u32) + (img.data[off + 1] as u32)
                + (img.data[off + 2] as u32);
            if sum < 350 {
                bins[bin] += 1;
            }
        }
    }
    bins
}

/// Normalize a fingerprint so its bins sum to a fixed value (1,000,000)
/// — makes correlation comparable across pages of different sizes.
fn normalize_fingerprint(bins: [u64; 32]) -> [u64; 32] {
    let total: u64 = bins.iter().sum();
    if total == 0 { return [0; 32]; }
    let mut out = [0u64; 32];
    for i in 0..32 {
        out[i] = (bins[i] * 1_000_000) / total;
    }
    out
}

/// Histogram intersection similarity: sum of element-wise minima.
/// For two fingerprints each normalized to 1,000,000, the max possible
/// value is 1,000,000 (identical) and the min is 0 (disjoint support).
/// This is the standard integer-clean similarity metric for normalized
/// histograms — no float, no overflow, no saturation degeneracy.
fn similarity(a: &[u64; 32], b: &[u64; 32]) -> u64 {
    let mut s: u64 = 0;
    for i in 0..32 {
        s += a[i].min(b[i]);
    }
    s
}

fn main() {
    println!("══════════════════════════════════════════════════════════════════════");
    println!(" CALIBRATION: SLUB red, FAMSI mapping, blank-like threshold");
    println!("══════════════════════════════════════════════════════════════════════");

    // ── JOB 1: Sample SLUB red-barrier color from page 16 ───────────────
    println!();
    println!(" [1] Sampling SLUB red-barrier color from page 16");
    let img16 = load_slub_page(&slub_path(16)).expect("decode page 16");
    println!("     image: {} × {}", img16.width, img16.height);
    // Find rows that have significant "red-ish" content.
    let red_rows = find_visually_red_rows(&img16, 50);
    if red_rows.is_empty() {
        println!("     no red-rich rows found at permissive threshold");
        return;
    }
    println!("     red-rich rows (sampled every 50): {:?}", red_rows);

    // Pick a representative red row and sample its pixels.
    let target_y = red_rows[red_rows.len() / 2];
    let pixels = sample_row(&img16, target_y, 200, img16.width - 200);
    // Filter to permissively-red pixels only.
    let red_only: Vec<(u8,u8,u8)> = pixels.iter().copied()
        .filter(|&(r,g,b)| r > 100 && r > g + 20 && r > b + 20)
        .collect();
    if red_only.is_empty() {
        println!("     no red pixels in row {}", target_y);
        return;
    }
    let mut rs: Vec<u8> = red_only.iter().map(|p| p.0).collect();
    let mut gs: Vec<u8> = red_only.iter().map(|p| p.1).collect();
    let mut bs: Vec<u8> = red_only.iter().map(|p| p.2).collect();
    rs.sort(); gs.sort(); bs.sort();
    println!("     red pixels at row {}: {} samples", target_y, red_only.len());
    println!("     R    Q1/median/Q3 = {} / {} / {}",
        q(&rs,1,4), q(&rs,1,2), q(&rs,3,4));
    println!("     G    Q1/median/Q3 = {} / {} / {}",
        q(&gs,1,4), q(&gs,1,2), q(&gs,3,4));
    println!("     B    Q1/median/Q3 = {} / {} / {}",
        q(&bs,1,4), q(&bs,1,2), q(&bs,3,4));
    let r_med = q(&rs,1,2);
    let g_med = q(&gs,1,2);
    let b_med = q(&bs,1,2);
    println!("     SUGGESTED tuned values for RegisterAwareSegmenter (SLUB):");
    println!("       red_min     = {} (median R)", r_med);
    println!("       red_excess  = {} (min(R-G, R-B) = min({},{}))",
        (r_med as i16 - g_med as i16).min(r_med as i16 - b_med as i16),
        r_med as i16 - g_med as i16, r_med as i16 - b_med as i16);

    // ── JOB 2: FAMSI → SLUB mapping by vertical fingerprint ──────────────
    println!();
    println!(" [2] FAMSI → SLUB mapping by 32-bin vertical-darkness fingerprint");
    let mut slub_fps: Vec<(u32, [u64; 32])> = Vec::new();
    for n in 13..=24 {
        let path = slub_path(n);
        if !path.exists() { continue; }
        let img = load_slub_page(&path).expect("decode SLUB");
        let fp = normalize_fingerprint(vertical_fingerprint(&img));
        slub_fps.push((n, fp));
    }
    let mut famsi_fps: Vec<(u32, [u64; 32])> = Vec::new();
    for n in 13..=24 {
        let path = famsi_path(n);
        if !path.exists() { continue; }
        let img = load_slub_page(&path).expect("decode FAMSI");
        let fp = normalize_fingerprint(vertical_fingerprint(&img));
        famsi_fps.push((n, fp));
    }
    println!("     SLUB pages fingerprinted: {}", slub_fps.len());
    println!("     FAMSI pages fingerprinted: {}", famsi_fps.len());
    println!();
    println!("     SLUB page → top-3 matching FAMSI indices (histogram intersection / 1,000,000):");
    let mut mapping: Vec<(u32, u32, u64)> = Vec::new();
    for (slub_n, slub_fp) in &slub_fps {
        // Score against every FAMSI page, sort descending.
        let mut scores: Vec<(u32, u64)> = famsi_fps.iter()
            .map(|(n, fp)| (*n, similarity(slub_fp, fp))).collect();
        scores.sort_by(|a, b| b.1.cmp(&a.1));
        let s0 = scores[0]; let s1 = scores[1]; let s2 = scores[2];
        let id_score = similarity(slub_fp, slub_fp);  // self-similarity baseline
        let confidence = if s1.1 > 0 { (s0.1 * 100) / s1.1 } else { 0 };
        let mark = if *slub_n == s0.0 { "✓naive" } else { "≠naive" };
        mapping.push((*slub_n, s0.0, s0.1));
        println!("       SLUB {:>2} (self={:>6}): {:>2}={:>6}  {:>2}={:>6}  {:>2}={:>6}  ratio={}%  {}",
            slub_n, id_score, s0.0, s0.1, s1.0, s1.1, s2.0, s2.1, confidence, mark);
    }
    // Determine if mapping is 1-to-1 by checking uniqueness.
    let mut famsi_used: std::collections::HashSet<u32> = std::collections::HashSet::new();
    let mut conflicts = 0;
    for &(_, fm, _) in &mapping {
        if !famsi_used.insert(fm) { conflicts += 1; }
    }
    println!();
    if conflicts == 0 {
        println!("     MAPPING IS BIJECTIVE: every FAMSI page is best-match for exactly one SLUB page.");
    } else {
        println!("     {} FAMSI pages collide as best-match (non-bijective by similarity alone).", conflicts);
    }
    // How well does the naive 1-to-1 hypothesis hold? Compute its total score.
    let mut naive_total: u64 = 0;
    let mut bijective_total: u64 = 0;
    for (slub_n, slub_fp) in &slub_fps {
        if let Some((_, famsi_fp)) = famsi_fps.iter().find(|(n, _)| n == slub_n) {
            naive_total += similarity(slub_fp, famsi_fp);
        }
        if let Some((_, _, s)) = mapping.iter().find(|(n, _, _)| n == slub_n) {
            bijective_total += s;
        }
    }
    println!("     naive 1-to-1 total similarity   = {}", naive_total);
    println!("     greedy best-match total          = {}", bijective_total);
    println!("     ratio (naive/greedy)             = {}%",
        if bijective_total > 0 { (naive_total * 100) / bijective_total } else { 0 });

    // ── JOB 3: Data-driven blank-like threshold ─────────────────────────
    println!();
    println!(" [3] SLUB per-page stats — pick threshold from real data, not speculation");
    println!("     page | components |       max area | likely status");
    println!("     -----+------------+----------------+---------------");
    let seg = DarknessThresholdSegmenter::default();
    let mut stats: Vec<(u32, usize, u64)> = Vec::new();
    for n in 13..=24 {
        let path = slub_path(n);
        if !path.exists() { continue; }
        let img = load_slub_page(&path).expect("decode");
        let bbs = seg.segment(&img);
        let max = bbs.iter().map(|b| b.area()).max().unwrap_or(0);
        stats.push((n, bbs.len(), max));
    }
    // The "blank-like" signature is: SMALL max area (no register-leak blob).
    // Page 24 is known WWII-damaged. Look at its max relative to neighbors.
    let p24_max = stats.iter().find(|(n,_,_)| *n == 24).map(|(_,_,m)| *m).unwrap_or(0);
    for (n, comp, max) in &stats {
        let status = if *max < p24_max * 2 && *comp < 5_500 {
            "★ BLANK-LIKE"
        } else if *max < 500_000 {
            "low-density"
        } else {
            "content-bearing"
        };
        println!("     {:>4} | {:>10} | {:>14} | {}", n, comp, max, status);
    }
    println!();
    println!("     Page-24 max area = {}", p24_max);
    println!("     Recommended threshold: max < {} (= page-24 max × 1.5)",
        (p24_max * 3) / 2);
    let recommended_max = (p24_max * 3) / 2;
    let mut flagged: Vec<u32> = Vec::new();
    for (n, comp, max) in &stats {
        if *max < recommended_max && *comp < 5_500 { flagged.push(*n); }
    }
    println!("     At (components<5500 AND max<{}): flags pages {:?}",
        recommended_max, flagged);

    // ── JOB 4: Verify RegisterAwareSegmenter barriers fire on real SLUB ─
    println!();
    println!(" [4] RegisterAwareSegmenter barrier detection on real SLUB (new defaults)");
    let reg_seg = RegisterAwareSegmenter::new(DarknessThresholdSegmenter::default());
    println!("     defaults: red_min={}, red_excess={}, row_fraction_per_mille={}",
        reg_seg.red_min, reg_seg.red_excess, reg_seg.row_fraction_per_mille);
    println!("     page | barrier rows | bands | barrier-row y-spans");
    println!("     -----+--------------+-------+--------------------");
    for n in 13..=24 {
        let path = slub_path(n);
        if !path.exists() { continue; }
        let img = load_slub_page(&path).expect("decode");
        let rows = reg_seg.barrier_rows(&img);
        let bands = reg_seg.register_bands(&img);
        // Compact summary: count contiguous y-runs.
        let mut runs: Vec<(u32, u32)> = Vec::new();
        if !rows.is_empty() {
            let mut start = rows[0];
            let mut last = rows[0];
            for &y in &rows[1..] {
                if y == last + 1 { last = y; }
                else { runs.push((start, last)); start = y; last = y; }
            }
            runs.push((start, last));
        }
        let span_str: String = runs.iter().take(6)
            .map(|(a,b)| format!("{}-{}", a, b))
            .collect::<Vec<_>>().join(", ");
        println!("     {:>4} | {:>12} | {:>5} | {}",
            n, rows.len(), bands.len(), span_str);
    }

    println!();
    println!("══════════════════════════════════════════════════════════════════════");
}
