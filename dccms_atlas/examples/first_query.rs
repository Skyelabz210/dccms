//! Exploratory query: run the canonical four-head atlas over a corpus
//! of synthetic codex events spanning the Maya operational range, and
//! report the atlas structure.

use dccms_atlas::events::EventSet;
use dccms_atlas::heads::{
    FourCalendarHydra, saturn_11_squared_head, zodiac_topology_head,
    temperaments_4fold_head, venus_phase_head, planetary_council_head,
    eclipse_alternation_head,
};
use dccms_atlas::atlas::{
    ConfigAtlas, head_progression, test_candidate_head,
};
use dccms_atlas::dkam_filter::DkamFilter;
use dccms_atlas::h5_navigator::compute_h5_report;
use dccms_atlas::H3_THRESHOLD_BP;

fn main() {
    println!("═══════════════════════════════════════════════════════════════");
    println!("  DCCMS — First Exploratory Query");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    // Build a canonical corpus over a substantial codex range.
    // 200,000 days ≈ 547 Haab years ≈ 4× Calendar Round
    let corpus = EventSet::canonical_corpus(0, 200_000);
    println!("Canonical event corpus: {} events over 200,000 days", corpus.len());
    println!();

    // ── Four-head Hydra ─────────────────────────────────────────────
    let hydra = FourCalendarHydra::canonical();
    println!("Canonical Four-Calendar Hydra verified: {}", hydra.verify_all());
    println!();
    for head in hydra.heads() {
        println!(
            "  {:<14} cycle={:<7} nullified={:?} active={:?}",
            head.signature.name,
            head.signature.cycle,
            head.signature.nullified_lanes,
            head.signature.active_lanes,
        );
    }
    println!();

    // ── Head progression: 1 → 2 → 3 → 4 heads ───────────────────────
    println!("── Head Progression (Gini in basis points) ──");
    let progression = head_progression(&corpus, &hydra);
    for (k, atlas) in &progression {
        println!(
            "  {}-head atlas: {} occupied addresses, Gini = {} bp",
            k,
            atlas.occupied_addresses(),
            atlas.gini_basis_points(),
        );
    }
    println!();

    // ── Four-head atlas structure ───────────────────────────────────
    let four_head_atlas = ConfigAtlas::build_four_head(&corpus, &hydra);
    println!("── Four-Head Atlas ──");
    println!("  Total events: {}", four_head_atlas.total_events);
    println!("  Occupied addresses: {}", four_head_atlas.occupied_addresses());
    println!("  Max address: {}", four_head_atlas.address_space.max_address());
    println!("  Dark strata: {}", four_head_atlas.dark_strata_count());
    println!("  Gini: {} bp", four_head_atlas.gini_basis_points());
    println!("  Mean occupancy: {} bp", four_head_atlas.mean_occupancy_bp());
    println!();

    // Top 5 brightest addresses
    println!("  Top 5 brightest addresses:");
    for entry in four_head_atlas.top_addresses(5) {
        println!(
            "    address {:#024b} → {} events",
            entry.address, entry.event_indices.len()
        );
    }
    println!();

    // ── DKAM admissibility for candidates ───────────────────────────
    println!("── DKAM Admissibility for Candidate Heads ──");
    let filter = DkamFilter::canonical();
    let candidates = [
        saturn_11_squared_head(),
        zodiac_topology_head(),
        temperaments_4fold_head(),
        venus_phase_head(),
        planetary_council_head(),
        eclipse_alternation_head(),
    ];
    for c in &candidates {
        let rep = filter.evaluate(c);
        println!(
            "  {:<24} cycle={:<5} deg={:<10} admissible={}",
            rep.head_name,
            rep.head_cycle,
            rep.inferred_degree.label(),
            rep.admissible,
        );
    }
    println!();

    // ── H3: candidate head resolution gain ──────────────────────────
    println!("── H3: Candidate-Head Resolution Gain (threshold {} bp) ──",
             H3_THRESHOLD_BP);
    for c in &candidates {
        let report = test_candidate_head(&corpus, &hydra, c, H3_THRESHOLD_BP);
        let marker = if report.admissible { "★" } else { " " };
        println!(
            "  {} {:<24} baseline={:>4} bp, extended={:>4} bp, gain={:>+5} bp",
            marker,
            c.signature.name,
            report.baseline_gini_bp,
            report.extended_gini_bp,
            report.gain_bp,
        );
    }
    println!();

    // ── H5: 11-lane distributional test ─────────────────────────────
    println!("── H5: Prime 11 as Navigator (uniform threshold 500 bp, nonuniform 1000 bp) ──");
    let h5 = compute_h5_report(&corpus, &hydra, 500, 1000);
    println!("  Four-head 11-lane max deviation: {} bp", h5.four_head_histogram.max_deviation_bp());
    println!("  Four-head uniform: {}", h5.four_head_uniform);
    println!("  Verdict: {}", h5.verdict.label());
    println!();
    println!("  11-lane class counts (four-head address-space partitioning):");
    for r in 0..11u64 {
        let count = h5.four_head_histogram.counts[r as usize];
        let bar = "▰".repeat(((count as usize) * 40 / h5.four_head_histogram.total.max(1) as usize).max(1));
        println!("    r₁₁ = {:>2}: {:>4} events  {}", r, count, bar);
    }
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("  First-pass query complete.");
    println!("═══════════════════════════════════════════════════════════════");
}
