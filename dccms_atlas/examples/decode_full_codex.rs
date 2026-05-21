//! Full-codex sweep — v0.9.5 N20.
//!
//! Runs the v0.9.3+v0.9.4 segmenter machinery against every SLUB page
//! 1..=74 on disk (and the FAMSI counterpart where available) and
//! reports the cross-source WWII-damage corroboration table for the
//! vault-known damage list [2, 4, 24, 28, 34, 38, 71, 72].
//!
//! Each Goddess page (16..=24) additionally runs through the
//! `decode_goddess_page` pipeline to report `cram_match` /
//! `figure_match` per v0.9.4 N13.
//!
//! Build/run:
//! ```text
//! cargo run --release --features slub --example decode_full_codex
//! ```

#![cfg(feature = "slub")]

use dccms_atlas::paths::{slub_page, famsi_page, SLUB_PAGE_RANGE};
use dccms_atlas::segmenter::{
    closing::ClosingThresholdSegmenter,
    register::RegisterAwareSegmenter,
    slub::load_slub_page,
    threshold::DarknessThresholdSegmenter,
    comparison::{compare_two_jpegs, is_wwii_damaged, slub_signals_damage, WWII_DAMAGED_PAGES},
    pipeline::decode_goddess_page_from_path,
    Segmenter,
};

fn main() {
    println!("══════════════════════════════════════════════════════════════════════");
    println!(" Full-codex sweep — v0.9.5 N20");
    println!(" Vault-known WWII-damaged pages: {:?}", WWII_DAMAGED_PAGES);
    println!("══════════════════════════════════════════════════════════════════════");
    println!();
    println!(" page | vault    | SLUB comp | SLUB max  | barriers | signal   | FAMSI? | match?");
    println!(" -----+----------+-----------+-----------+----------+----------+--------+-------");

    let close_seg = ClosingThresholdSegmenter::default();
    let reg_seg = RegisterAwareSegmenter::new(DarknessThresholdSegmenter::default());

    let mut slub_present: u32 = 0;
    let mut famsi_present: u32 = 0;
    let mut signal_damaged: Vec<u32> = Vec::new();
    let mut corroborated: u32 = 0;
    let mut disagreements: Vec<(u32, bool, bool)> = Vec::new();

    for page in SLUB_PAGE_RANGE {
        let sp = slub_page(page);
        if !sp.exists() { continue; }
        slub_present += 1;
        let img = match load_slub_page(&sp) {
            Ok(i) => i,
            Err(_) => { continue; }
        };
        let bboxes = close_seg.segment(&img);
        let max_area = bboxes.iter().map(|b| b.area()).max().unwrap_or(0);
        let barriers = reg_seg.barrier_rows(&img).len();
        let signals = slub_signals_damage(bboxes.len(), max_area, barriers);
        let vault = is_wwii_damaged(page as u8);
        let famsi_exists = famsi_page(page).exists();
        if famsi_exists { famsi_present += 1; }

        if signals { signal_damaged.push(page); }
        let agrees = vault == signals;
        if agrees { corroborated += 1; }
        else { disagreements.push((page, vault, signals)); }

        let vault_label = if vault { "DAMAGED " } else { "intact  " };
        let signal_label = if signals { "DAMAGE  " } else { "intact  " };
        let famsi_mark = if famsi_exists { "✓" } else { " " };
        let match_mark = if agrees { "✓" } else { "✗" };
        println!("  {:>3} | {} | {:>9} | {:>9} | {:>8} | {} |   {}    |   {}",
            page, vault_label, bboxes.len(), max_area, barriers,
            signal_label, famsi_mark, match_mark);
    }

    println!();
    println!(" ── SLUB / FAMSI coverage ────────────────────────────────────────────");
    println!(" SLUB pages on disk:  {} / 74", slub_present);
    println!(" FAMSI pages on disk: {} / 74", famsi_present);

    println!();
    println!(" ── WWII-damage cross-source corroboration ───────────────────────────");
    println!(" Vault-known damaged pages: {:?}", WWII_DAMAGED_PAGES);
    let vault_in_coverage: Vec<u8> = WWII_DAMAGED_PAGES.iter().copied()
        .filter(|&p| slub_page(p as u32).exists()).collect();
    println!(" In SLUB coverage:          {:?}", vault_in_coverage);
    println!(" Signal flagged damaged:    {:?}", signal_damaged);
    println!(" Corroboration: {}/{} pages agree with vault",
        corroborated, slub_present);
    if disagreements.is_empty() {
        println!(" ★ Every page on disk corroborates the vault.");
    } else {
        println!(" Disagreements (page, vault, signal):");
        for (p, v, s) in &disagreements {
            println!("   page {}: vault={} signal={}", p,
                if *v { "damaged" } else { "intact" },
                if *s { "damaged" } else { "intact" });
        }
    }

    // Cross-source comparison on every vault-damaged page that has
    // FAMSI available. The stat-only damage signal was calibrated on
    // page 24 and doesn't generalize — but cross-source comparison via
    // SLUB↔FAMSI does, page by page.
    println!();
    println!(" ── Cross-source on vault-damaged pages (SLUB ↔ FAMSI) ────────────────");
    println!(" page | SLUB comp | SLUB max  | SLUB bar | FAMSI comp | FAMSI max  | sig agrees?");
    println!(" -----+-----------+-----------+----------+------------+------------+------------");
    let mut famsi_in_damage_set: u32 = 0;
    let mut cross_corroborated: u32 = 0;
    for &page in WWII_DAMAGED_PAGES {
        let sp = slub_page(page as u32);
        let fp = famsi_page(page as u32);
        if !sp.exists() || !fp.exists() {
            println!("  {:>3} | (FAMSI not in coverage)", page);
            continue;
        }
        famsi_in_damage_set += 1;
        match compare_two_jpegs(page, &sp, &fp) {
            Ok(pc) => {
                let agree_mark = if pc.segmenter_corroborates_vault { "✓" } else { "✗" };
                if pc.segmenter_corroborates_vault { cross_corroborated += 1; }
                println!("  {:>3} | {:>9} | {:>9} | {:>8} | {:>10} | {:>10} |     {}",
                    page,
                    pc.slub_components, pc.slub_max_area, pc.slub_barrier_rows,
                    pc.famsi_components, pc.famsi_max_area, agree_mark);
            }
            Err(e) => println!("  {:>3} | error: {:?}", page, e),
        }
    }
    println!();
    println!(" Vault-damaged pages with FAMSI:  {} / {}",
        famsi_in_damage_set, WWII_DAMAGED_PAGES.len());
    println!(" Cross-source corroborates vault: {} / {}",
        cross_corroborated, famsi_in_damage_set);
    if cross_corroborated < famsi_in_damage_set {
        println!();
        println!(" The damage-stat signature is calibrated on page 24 only.");
        println!(" Pages 2, 4, 28, 34, 38, 71, 72 have stats inconsistent with");
        println!(" 'near-total content loss' — they show SOME readable content");
        println!(" surviving the WWII water damage. Cross-source comparison");
        println!(" doesn't auto-corroborate from stats alone; the FAMSI side");
        println!(" is the diagnostic for content the photograph cannot recover.");
    }

    // Goddess pipeline run on 16..=24 (real-pixel discharge).
    println!();
    println!(" ── Goddess pipeline (16..=24) — H4 real-pixel discharge ─────────────");
    println!(" page | classified | expected | fig? | CRAM?");
    println!(" -----+------------+----------+------+------");
    let mut cram_ok = 0u32;
    let mut fig_ok = 0u32;
    let mut goddess_runs = 0u32;
    for page in 16u8..=24 {
        let sp = slub_page(page as u32);
        if !sp.exists() { continue; }
        goddess_runs += 1;
        match decode_goddess_page_from_path(page, &sp) {
            Ok(d) => {
                let cls = match d.classified_figure {
                    Some(f) => format!("{:?}", f),
                    None => "—".to_string(),
                };
                let exp = match d.expected_figure {
                    Some(f) => format!("{:?}", f),
                    None => "—".to_string(),
                };
                if d.cram_match { cram_ok += 1; }
                if d.figure_match { fig_ok += 1; }
                println!("  {:>3} | {:<10} | {:<8} |   {}  |   {}",
                    page,
                    truncate(&cls, 10),
                    truncate(&exp, 8),
                    if d.figure_match { "✓" } else { "✗" },
                    if d.cram_match { "✓" } else { "✗" }
                );
            }
            Err(e) => println!("  {:>3} | ERROR: {:?}", page, e),
        }
    }
    println!();
    println!(" CRAM matches:   {} / {}", cram_ok, goddess_runs);
    println!(" figure matches: {} / {}", fig_ok, goddess_runs);
    if goddess_runs > 0 && cram_ok == goddess_runs && fig_ok == goddess_runs {
        println!(" ★ Full Goddess-section discharge holds on real SLUB imagery.");
    }
    println!("══════════════════════════════════════════════════════════════════════");
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max { s.to_string() }
    else {
        let mut out: String = s.chars().take(max - 1).collect();
        out.push('…');
        out
    }
}
