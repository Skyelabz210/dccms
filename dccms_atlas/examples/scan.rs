//! # Dresden Codex scanner — v0.9.6.
//!
//! Real pixel measurements for any subset of the codex,
//! plus a comparison against the vault model predictions.
//!
//! Usage:
//! ```text
//! # All 74 pages
//! cargo run --release --features slub --example scan
//!
//! # Specific pages or ranges
//! cargo run --release --features slub --example scan -- 52
//! cargo run --release --features slub --example scan -- 46 47 48
//! cargo run --release --features slub --example scan -- 46-50
//! cargo run --release --features slub --example scan -- 51-58
//!
//! # Write a CSV alongside the table
//! DCCMS_CSV=/tmp/scan.csv cargo run --release --features slub --example scan
//!
//! # Point at a custom image directory (overrides hard-coded path)
//! DCCMS_IMPORTS_ROOT=/my/images cargo run --release --features slub --example scan
//! ```
//!
//! ## Reading the output
//!
//! - `ink‰`   : fraction of pixels classified as ink (max channel < 180)
//! - `red‰`   : fraction classified as red (R ≥ 164, dominance ≥ 24 over G and B)
//! - `entropy` : Shannon entropy of luminance histogram, in millibits (8000 = max)
//! - `comps`   : connected-component count from closing-threshold segmenter
//! - `max_area`: largest component area in pixels
//! - `bars`    : red horizontal barrier rows (register structure)
//! - `ok?`     : whether the measurements are consistent with vault predictions
//!
//! ## Vault predictions are UNRELIABLE
//!
//! All `[VAULT]` predictions were written by AI in prior sessions and
//! have not been verified. A `✗` in `ok?` is a finding, not a failure.
//! All thresholds are uncalibrated estimates (see calibration note at end).

#![cfg(feature = "slub")]

use dccms_atlas::paths::{slub_page, SLUB_PAGE_RANGE};
use dccms_atlas::segmenter::{
    crosscheck::{
        crosscheck, CrosscheckResult, ENTROPY_HIGH_THRESHOLD, INK_DAMAGED_THRESHOLD,
        INK_DENSE_THRESHOLD, RED_ELEVATED_THRESHOLD,
    },
    scan::{scan_page, PageScan},
    slub::load_slub_page,
};
use std::io::Write as IoWrite;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let pages = parse_pages(&args);
    let csv_path = std::env::var("DCCMS_CSV").ok();

    let img_dir = slub_page(1);
    let img_dir = img_dir.parent().unwrap_or_else(|| std::path::Path::new("."));

    println!("══════════════════════════════════════════════════════════════════════════════════");
    println!(" DCCMS Scanner — real pixel measurements + vault prediction comparison");
    println!(" Image source: {}", img_dir.display());
    println!("══════════════════════════════════════════════════════════════════════════════════");
    println!();
    println!(" pg | ink‰ | red‰ | entropy | comps | max_area  | bars | vault section          | ok?");
    println!(" ---+------+------+---------+-------+-----------+------+------------------------+----");

    let mut rows: Vec<(PageScan, CrosscheckResult)> = Vec::new();
    let mut missing: u32 = 0;

    for &page in &pages {
        let path = slub_page(page as u32);
        if !path.exists() {
            println!(
                "  {:>2} | (image not present at {})",
                page,
                path.display()
            );
            missing += 1;
            continue;
        }
        let img = match load_slub_page(&path) {
            Ok(i) => i,
            Err(e) => {
                println!("  {:>2} | ERROR: {:?}", page, e);
                missing += 1;
                continue;
            }
        };
        let scan = scan_page(page, &img);
        let check = crosscheck(&scan);

        let ok_mark = if check.consistent { "✓" } else { "✗" };
        let section = trunc(check.prediction.section, 22);

        println!(
            "  {:>2} | {:>4} | {:>4} | {:>7} | {:>5} | {:>9} | {:>4} | {:<22} | {}",
            page,
            scan.ink_density_permille,
            scan.red_permille,
            scan.entropy_millibits,
            scan.component_count,
            scan.max_component_area,
            scan.barrier_rows,
            section,
            ok_mark
        );

        rows.push((scan, check));
    }

    let scanned = rows.len() as u32;
    let consistent: u32 = rows.iter().filter(|(_, c)| c.consistent).count() as u32;
    let inconsistent = scanned - consistent;

    println!();
    println!(
        " Scanned: {} / {} requested ({} missing).",
        scanned,
        pages.len(),
        missing
    );
    println!(
        " Vault consistency: {} consistent, {} inconsistent.",
        consistent, inconsistent
    );

    if inconsistent > 0 {
        println!();
        println!(" Pages where measurements CONTRADICT vault predictions:");
        for (scan, check) in rows.iter().filter(|(_, c)| !c.consistent) {
            println!(
                "   page {:>2}  section: {}",
                scan.page, check.prediction.section
            );
            println!(
                "            matching={} contradicting={}",
                check.matching_predictions, check.contradicting_predictions
            );
            println!(
                "            measured: ink={}‰ red={}‰ entropy={}mb",
                scan.ink_density_permille, scan.red_permille, scan.entropy_millibits
            );
            println!(
                "            vault role: {}",
                check.prediction.vault_role
            );
        }
    }

    // Calibration note — always shown so the user remembers what "ok?" means
    println!();
    println!(" ── Calibration note ────────────────────────────────────────────────────────────");
    println!(
        " Thresholds used: RED_ELEVATED={}, INK_DENSE={}, ENTROPY_HIGH={}, INK_DAMAGED={}",
        RED_ELEVATED_THRESHOLD, INK_DENSE_THRESHOLD, ENTROPY_HIGH_THRESHOLD, INK_DAMAGED_THRESHOLD
    );
    println!(" These are uncalibrated initial estimates. Run all 74 pages and set thresholds");
    println!(" from the distribution (e.g. bottom-quartile for 'elevated red', etc.).");
    println!(" All vault predictions are AI-penned hypotheses [VAULT] — contradictions are");
    println!(" findings, not program errors.");

    // Optional CSV
    if let Some(ref path) = csv_path {
        write_csv(path, &rows);
    }

    println!("══════════════════════════════════════════════════════════════════════════════════");
}

fn write_csv(path: &str, rows: &[(PageScan, CrosscheckResult)]) {
    match std::fs::File::create(path) {
        Err(e) => eprintln!("CSV: could not create {}: {}", path, e),
        Ok(f) => {
            let mut w = std::io::BufWriter::new(f);
            let _ = writeln!(
                w,
                "page,ink_permille,red_permille,entropy_millibits,\
                 component_count,max_component_area,barrier_rows,\
                 section,consistent,matching,contradicting"
            );
            for (scan, check) in rows {
                let _ = writeln!(
                    w,
                    "{},{},{},{},{},{},{},{},{},{},{}",
                    scan.page,
                    scan.ink_density_permille,
                    scan.red_permille,
                    scan.entropy_millibits,
                    scan.component_count,
                    scan.max_component_area,
                    scan.barrier_rows,
                    check.prediction.section,
                    check.consistent,
                    check.matching_predictions,
                    check.contradicting_predictions
                );
            }
            eprintln!("CSV written to {}", path);
        }
    }
}

fn trunc(s: &str, max: usize) -> String {
    if s.len() <= max {
        format!("{:<width$}", s, width = max)
    } else {
        format!("{}…", &s[..max.saturating_sub(1)])
    }
}

/// Parse command-line arguments into a sorted, deduplicated list of page numbers.
///
/// Accepts: integers (`52`), space-separated integers (`46 47 48`),
/// and ranges (`51-58`). Empty → all 74 pages.
fn parse_pages(args: &[String]) -> Vec<u8> {
    if args.is_empty() {
        return SLUB_PAGE_RANGE.map(|p| p as u8).collect();
    }
    let mut pages: Vec<u8> = Vec::new();
    for arg in args {
        if let Some((a, b)) = arg.split_once('-') {
            if let (Ok(lo), Ok(hi)) = (a.parse::<u8>(), b.parse::<u8>()) {
                for p in lo..=hi {
                    pages.push(p);
                }
                continue;
            }
        }
        match arg.parse::<u8>() {
            Ok(p) => pages.push(p),
            Err(_) => eprintln!("warning: unrecognised argument {:?}, skipped", arg),
        }
    }
    pages.sort_unstable();
    pages.dedup();
    pages
}
