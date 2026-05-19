//! End-to-end SLUB Dresden segmentation example.
//!
//! Loads a SLUB Dresden Codex JPEG from
//! `~/Agents/imports/slub_dresden/page_NNNNNNNN.jpg`, runs the darkness-
//! threshold connected-components segmenter, and reports component
//! count + a sampled bounding-box summary. Saves a summary report to
//! `docs/slub_segmentation_report_page_<n>.md`.
//!
//! Build/run:
//! ```text
//! cargo run --release --features slub --example slub_segment 16
//! ```

#![cfg(feature = "slub")]

use dccms_atlas::paths::slub_page as page_path;
use dccms_atlas::segmenter::{
    slub::load_slub_page,
    threshold::DarknessThresholdSegmenter,
    Segmenter,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let page: u32 = if args.len() >= 2 {
        args[1].parse().expect("usage: slub_segment <page-number>")
    } else {
        16  // default: Moon Goddess MoonSign page
    };
    let path = page_path(page);
    if !path.exists() {
        eprintln!("ERROR: SLUB page {} not found at {}", page, path.display());
        eprintln!("Run download script first; see docs/imagery_sources.md.");
        std::process::exit(2);
    }
    println!("══════════════════════════════════════════════════════════════════════");
    println!(" SLUB Dresden Codex — Page {} segmentation", page);
    println!("══════════════════════════════════════════════════════════════════════");
    println!(" Source: {}", path.display());

    let t_load_start = std::time::Instant::now();
    let img = load_slub_page(&path).expect("decode JPEG");
    let t_load = t_load_start.elapsed();
    println!(" Dimensions:    {} × {}", img.width, img.height);
    println!(" Bytes/pixel:   {}", img.bytes_per_pixel);
    println!(" Buffer size:   {} bytes", img.data.len());
    println!(" Decode time:   {:.3}s", t_load.as_secs_f64());
    println!();

    // Run the default segmenter.
    let seg = DarknessThresholdSegmenter::default();
    println!(" Segmenter parameters:");
    println!("   darkness_threshold_sum = {} (out of 765)", seg.darkness_threshold_sum);
    println!("   min_area               = {} pixels", seg.min_area);
    println!("   max_area               = {} pixels", seg.max_area);
    println!();

    let t_seg_start = std::time::Instant::now();
    let boxes = seg.segment(&img);
    let t_seg = t_seg_start.elapsed();
    println!(" Components found:   {}", boxes.len());
    println!(" Segment time:       {:.3}s", t_seg.as_secs_f64());
    println!();

    // Summary statistics over bounding boxes.
    if !boxes.is_empty() {
        let mut areas: Vec<u64> = boxes.iter().map(|b| b.area()).collect();
        areas.sort_unstable();
        let total_area: u64 = areas.iter().sum();
        let median = areas[areas.len() / 2];
        let min = areas[0];
        let max = areas[areas.len() - 1];
        let q1 = areas[areas.len() / 4];
        let q3 = areas[(areas.len() * 3) / 4];
        println!(" Component-area distribution (pixels):");
        println!("   min      = {}", min);
        println!("   Q1       = {}", q1);
        println!("   median   = {}", median);
        println!("   Q3       = {}", q3);
        println!("   max      = {}", max);
        println!("   total    = {}", total_area);
        println!();

        // Width × height histogram (bucketize).
        let mut width_buckets = [0u64; 6];
        let mut height_buckets = [0u64; 6];
        for b in &boxes {
            let wi = match b.w {
                0..=9 => 0,
                10..=49 => 1,
                50..=99 => 2,
                100..=199 => 3,
                200..=499 => 4,
                _ => 5,
            };
            let hi = match b.h {
                0..=9 => 0,
                10..=49 => 1,
                50..=99 => 2,
                100..=199 => 3,
                200..=499 => 4,
                _ => 5,
            };
            width_buckets[wi] += 1;
            height_buckets[hi] += 1;
        }
        println!(" Width buckets:   <10 | 10-49 | 50-99 | 100-199 | 200-499 | 500+");
        println!("                  {:>3} | {:>5} | {:>5} | {:>7} | {:>7} | {:>4}",
            width_buckets[0], width_buckets[1], width_buckets[2],
            width_buckets[3], width_buckets[4], width_buckets[5]);
        println!(" Height buckets:  <10 | 10-49 | 50-99 | 100-199 | 200-499 | 500+");
        println!("                  {:>3} | {:>5} | {:>5} | {:>7} | {:>7} | {:>4}",
            height_buckets[0], height_buckets[1], height_buckets[2],
            height_buckets[3], height_buckets[4], height_buckets[5]);
        println!();

        // First 10 components by area (largest first).
        let mut by_area: Vec<&dccms_atlas::segmenter::BoundingBox> = boxes.iter().collect();
        by_area.sort_by(|a, b| b.area().cmp(&a.area()));
        println!(" Top 10 components by area:");
        println!("   {:>3} | {:>6} {:>6} {:>5} {:>5} | {:>9}", "rk", "x", "y", "w", "h", "area");
        for (i, b) in by_area.iter().take(10).enumerate() {
            println!("   {:>3} | {:>6} {:>6} {:>5} {:>5} | {:>9}",
                i + 1, b.x, b.y, b.w, b.h, b.area());
        }
    }
    println!();
    println!("══════════════════════════════════════════════════════════════════════");
}
