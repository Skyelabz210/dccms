//! Compare every SLUB page against its FAMSI counterpart.
//!
//! The FAMSI → Förstemann mapping is treated as 1-to-1 (extracted
//! page_NN.jpg ↔ Förstemann page NN). This was verified empirically
//! by `examples/calibrate.rs`: the naive 1-to-1 hypothesis yields
//! 98% of the greedy best-match total similarity score under
//! histogram-intersection of vertical row-darkness fingerprints. No
//! greater discriminating power is needed for this set of 12 pages.
//!
//! Damage status is taken from the vault-known list of WWII-damaged
//! Dresden Codex pages (see `WWII_DAMAGED_PAGES`); the segmenter
//! stats serve as corroboration, not as the source of truth.
//!
//! Build/run:
//! ```text
//! cargo run --release --features slub --example compare_slub_famsi
//! ```

#![cfg(feature = "slub")]

use dccms_atlas::paths::{slub_page as slub_path, famsi_page as famsi_path};
use dccms_atlas::segmenter::comparison::{compare_two_jpegs, is_wwii_damaged};

fn main() {
    println!("══════════════════════════════════════════════════════════════════════");
    println!(" Cross-source comparison: SLUB photograph ↔ FAMSI chromolithograph");
    println!(" (mode: verify vault-known WWII damage with segmenter evidence)");
    println!("══════════════════════════════════════════════════════════════════════");
    println!();
    println!(" page | vault    | SLUB comp | SLUB max  | barriers | combined signal | verdict");
    println!(" -----+----------+-----------+-----------+----------+-----------------+---------------");
    let mut disagreements: Vec<(u32, bool, bool)> = Vec::new();
    for n in 13..=24 {
        let sp = slub_path(n);
        let fp = famsi_path(n);
        if !sp.exists() || !fp.exists() { continue; }
        match compare_two_jpegs(n as u8, &sp, &fp) {
            Ok(pc) => {
                let vault_label = if pc.wwii_damaged_per_vault { "DAMAGED " } else { "intact  " };
                let signal_label = if pc.slub_signals_damage { "DAMAGE  " } else { "intact  " };
                let verdict = if pc.segmenter_corroborates_vault {
                    "✓ corroborated"
                } else if pc.wwii_damaged_per_vault {
                    "⚠ vault damaged, signal disagrees"
                } else {
                    "⚠ signal damaged, vault disagrees"
                };
                println!("  {:>3} | {} | {:>9} | {:>9} | {:>8} | {}        | {}",
                    n, vault_label, pc.slub_components, pc.slub_max_area,
                    pc.slub_barrier_rows, signal_label, verdict);
                if !pc.segmenter_corroborates_vault {
                    disagreements.push((n, pc.wwii_damaged_per_vault, pc.slub_signals_damage));
                }
            }
            Err(e) => println!("  {:>3} | ERROR: {:?}", n, e),
        }
    }
    println!();
    println!(" Vault-known WWII-damaged pages in 13-24: ");
    for n in 13..=24 {
        if is_wwii_damaged(n as u8) {
            println!("   page {}", n);
        }
    }
    println!();
    if disagreements.is_empty() {
        println!(" ★ Every page in 13-24 corroborates the vault.");
        println!(" The v0.9.1 page-18 false positive is fixed: barriers detected");
        println!(" at calibrated (red_min=164, red_excess=24) defaults break the");
        println!(" stats-only tie with page 24 (0 barriers).");
    } else {
        println!(" Segmenter ↔ vault disagreements ({} pages):", disagreements.len());
        for (n, vault, sig) in &disagreements {
            println!("   page {}: vault={}, signals-damage={}",
                n, if *vault { "damaged" } else { "intact" }, sig);
        }
    }
    println!("══════════════════════════════════════════════════════════════════════");
}
