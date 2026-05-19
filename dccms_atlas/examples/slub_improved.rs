//! Improved-segmenter run: closing + register-aware against real SLUB pages.
//!
//! Build/run:
//! ```text
//! cargo run --release --features slub --example slub_improved 16
//! ```

#![cfg(feature = "slub")]

use dccms_atlas::segmenter::{
    classify::verify_page_iconography,
    closing::ClosingThresholdSegmenter,
    register::RegisterAwareSegmenter,
    slub::load_slub_page,
    threshold::DarknessThresholdSegmenter,
    Segmenter,
};
use std::path::PathBuf;

fn home_dir() -> PathBuf {
    let h = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .expect("home dir not set");
    PathBuf::from(h)
}

fn page_path(page: u32) -> PathBuf {
    home_dir().join("Agents").join("imports").join("slub_dresden")
        .join(format!("page_{:08}.jpg", page))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let page: u32 = if args.len() >= 2 {
        args[1].parse().expect("usage: slub_improved <page>")
    } else { 16 };
    let path = page_path(page);
    if !path.exists() {
        eprintln!("SLUB page {} not found at {}", page, path.display());
        std::process::exit(2);
    }
    let img = load_slub_page(&path).expect("decode JPEG");
    println!("Page {} loaded: {}×{}", page, img.width, img.height);
    println!();

    // Plain (baseline) — for comparison.
    let plain = DarknessThresholdSegmenter::default();
    let t0 = std::time::Instant::now();
    let plain_bbs = plain.segment(&img);
    let plain_t = t0.elapsed();

    // Closing alone.
    let closing = ClosingThresholdSegmenter::default();
    let t0 = std::time::Instant::now();
    let closing_bbs = closing.segment(&img);
    let closing_t = t0.elapsed();

    // Register-aware + plain (no closing in inner).
    let reg_plain = RegisterAwareSegmenter::new(DarknessThresholdSegmenter::default());
    let t0 = std::time::Instant::now();
    let reg_plain_bbs = reg_plain.segment(&img);
    let reg_plain_t = t0.elapsed();

    // Register-aware + closing inner (the strongest combination).
    let combined = RegisterAwareSegmenter::new(ClosingThresholdSegmenter::default());
    let t0 = std::time::Instant::now();
    let combined_bbs = combined.segment(&img);
    let combined_t = t0.elapsed();

    println!(" Segmenter                    | Components | Max area  | Time");
    println!(" -----------------------------+------------+-----------+--------");
    let report = |name: &str, bbs: &[dccms_atlas::segmenter::BoundingBox], t: std::time::Duration| {
        let max = bbs.iter().map(|b| b.area()).max().unwrap_or(0);
        println!(" {:<28} | {:>10} | {:>9} | {:.3}s",
            name, bbs.len(), max, t.as_secs_f64());
    };
    report("plain (baseline)", &plain_bbs, plain_t);
    report("closing only (r=4)", &closing_bbs, closing_t);
    report("register-aware + plain", &reg_plain_bbs, reg_plain_t);
    report("register-aware + closing", &combined_bbs, combined_t);
    println!();

    // Iconographic verification on the strongest result.
    let v = verify_page_iconography(page as u8, &combined_bbs, 50_000);
    println!(" Iconographic verification (combined segmenter):");
    println!("   expected_figure       = {:?}", v.expected_figure);
    println!("   total_bboxes          = {}", v.total_bboxes);
    println!("   figure_class_bboxes   = {}", v.figure_class_bboxes);
    println!("   largest_figure_bbox   = {:?}", v.largest_figure_bbox);
    println!("   is_blank_bridge       = {}", v.is_blank_bridge);
    println!("   consistent            = {}", v.consistent);
}
