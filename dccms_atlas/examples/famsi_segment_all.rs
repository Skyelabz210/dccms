//! Run the threshold segmenter against all 12 FAMSI-extracted JPEGs
//! and identify the blank-bridge candidate by signature.
//!
//! Build/run:
//! ```text
//! cargo run --release --features slub --example famsi_segment_all
//! ```

#![cfg(feature = "slub")]

use dccms_atlas::paths::famsi_page as famsi_path;
use dccms_atlas::segmenter::{
    slub::load_slub_page,  // works on any JPEG — function name is historical
    threshold::DarknessThresholdSegmenter,
    Segmenter,
};

fn main() {
    println!("══════════════════════════════════════════════════════════════════════");
    println!(" FAMSI 12-page segmentation sweep");
    println!("══════════════════════════════════════════════════════════════════════");
    println!();
    println!(" PDF-index | dims         | components | median area | max area");
    println!(" ----------+--------------+------------+-------------+----------");
    let seg = DarknessThresholdSegmenter::default();
    let mut by_max: Vec<(u32, usize, u64)> = Vec::new();
    for n in 13..=24 {
        let path = famsi_path(n);
        if !path.exists() { continue; }
        let img = load_slub_page(&path).expect("decode JPEG");
        let bbs = seg.segment(&img);
        let mut areas: Vec<u64> = bbs.iter().map(|b| b.area()).collect();
        areas.sort_unstable();
        let median = if areas.is_empty() { 0 } else { areas[areas.len() / 2] };
        let max = areas.last().copied().unwrap_or(0);
        println!("  page_{:02} | {:>5}×{:<6} | {:>10} | {:>11} | {:>9}",
            n, img.width, img.height, bbs.len(), median, max);
        by_max.push((n, bbs.len(), max));
    }
    println!();
    // Identify blank-bridge candidate: lowest max-area and component count.
    by_max.sort_by_key(|&(_, count, max)| (max, count));
    println!(" Blank-bridge candidates (sorted by max area ascending):");
    for (i, (n, count, max)) in by_max.iter().take(3).enumerate() {
        println!("   #{} → FAMSI page_{:02}: {} components, max area {}",
            i + 1, n, count, max);
    }
    println!();
    println!("══════════════════════════════════════════════════════════════════════");
}
