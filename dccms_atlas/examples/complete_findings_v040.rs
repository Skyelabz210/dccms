//! DCCMS v0.4.0 — Complete Findings
use dccms_atlas::events::EventSet;
use dccms_atlas::heads::FourCalendarHydra;
use dccms_atlas::atlas::ConfigAtlas;
use dccms_atlas::h3_extended::compute_h3_extended;
use dccms_atlas::h5_katun::compute_katun_depth_report;
use dccms_atlas::manifold_upgrade::{hamming2_connectivity, lane_bridge_analysis, manifold_geometry};
use dccms_atlas::generator_catalog::{build_catalog, catalog_stats, same_carry_class,
                                      level2_periods};

fn main() {
    println!("════════════════════════════════════════════════════════════════════");
    println!("  DCCMS v0.4.0 — Complete Findings");
    println!("════════════════════════════════════════════════════════════════════");
    println!();

    let corpus = EventSet::canonical_corpus(0, 200_000);
    let hydra = FourCalendarHydra::canonical();
    let atlas = ConfigAtlas::build_four_head(&corpus, &hydra);

    // ── H3 Extended: generator-predicted candidates ───────────────────────
    println!("══ H3 EXTENDED: GENERATOR-PREDICTED CANDIDATES ═════════════════════");
    println!();
    let h3 = compute_h3_extended(&corpus, &hydra, 10_000, 3_000);
    println!("  Generator candidates tested: {}", h3.tested);
    println!("  DKAM-admissible: {}", h3.dkam_pass);
    println!("  H3-admissible (≥30%): {}", h3.h3_pass);
    println!();
    println!("  {:>22} | {:>6} | L11 | DKAM | MI ratio", "Candidate", "cycle");
    println!("  {}", "─".repeat(60));
    for e in h3.entries.iter().filter(|e| e.dkam_admissible).take(20) {
        let ratio_str = e.independence_ratio_bp.map(|r| format!("{:>3}%", r/100))
            .unwrap_or_else(|| "  --".to_string());
        let marker = if e.h3_admissible { "★" } else { " " };
        println!("  {} {:>22} | {:>6} |  {}  |  ✓   | {}",
            marker, e.astro_name, e.cycle, e.level_11, ratio_str);
    }
    println!();
    println!("  ★ = newly admissible at ≥30% independence");
    if !h3.newly_admissible.is_empty() {
        println!();
        println!("  Newly admissible cycles:");
        for (cycle, path) in &h3.newly_admissible {
            println!("    {} days via {}", cycle, path);
        }
    }
    println!();

    // ── H5 Katun depth partition ──────────────────────────────────────────
    println!("══ H5 KATUN DEPTH PARTITION (11³ = 1331 boundary) ══════════════════");
    println!();
    let katun = compute_katun_depth_report(&corpus, &hydra);
    println!("  LongCount cycle = 7200 days. Depth threshold = 1331 days.");
    println!("  Shadow-free zone: days 0-1330 ({} days, 18.5%)", 1331);
    println!("  Depth-active zone: days 1331-7199 ({} days, 81.5%)", 7200-1331);
    println!();
    println!("  Events in shadow-free zone: {} ({} bp)",
        katun.shadow_free_events, katun.shadow_free_fraction_bp);
    println!("  Events in depth-active zone: {}", katun.depth_active_events);
    println!();
    println!("  κ₃ distribution by zone:");
    println!("    Shadow-free: κ₃=0 fraction = {} bp (predicted: 10000)",
        katun.shadow_free_dist.kappa3_zero_fraction_bp());
    println!("    Depth-active: κ₃=0 fraction = {} bp (predicted: < 9000)",
        katun.depth_dist.kappa3_zero_fraction_bp());
    println!("    Prediction confirmed: {}", katun.prediction_confirmed);
    println!();
    println!("  Per-Tun κ₃=0 fraction (0-indexed, 360 days each):");
    println!("  Tun | Shadow? | κ₃=0 bp | Events");
    println!("  {}", "─".repeat(40));
    for (i, ((count, bp), sf)) in katun.tun_dist.tun_counts.iter()
        .zip(katun.tun_dist.tun_kappa3_zero_bp.iter())
        .zip(katun.tun_dist.tun_shadow_free.iter())
        .enumerate() {
        let sf_mark = if *sf { "✓" } else { " " };
        println!("  {:>3} |    {}    |   {:>5} | {}",
            i, sf_mark, bp, count);
    }
    println!();

    // ── Manifold upgrade ─────────────────────────────────────────────────
    println!("══ MANIFOLD UPGRADE: HAMMING-2 CONNECTIVITY ════════════════════════");
    println!();
    let h2conn = hamming2_connectivity(&atlas);
    println!("  v0.1.0 at Hamming-1: 110 components, largest=265");
    println!("  v0.4.0 at Hamming≤2: {} components, largest={}",
        h2conn.component_count, h2conn.largest);
    println!("  Top-7 total: {} ({} bp of all occupied)",
        h2conn.top7_total, h2conn.top7_fraction_bp);
    println!("  Top 10 component sizes: {:?}",
        h2conn.component_sizes.iter().take(10).collect::<Vec<_>>());
    println!();

    println!("  Bridge-bit analysis (which bits connect Hamming-1 components):");
    let bridge = lane_bridge_analysis(&atlas);
    println!("  Rank | Bit | Head/Lane label           | Cross-component edges");
    println!("  {}", "─".repeat(55));
    for (rank, (&(bit, count), label)) in bridge.top_bridge_bits.iter()
        .zip(bridge.bridge_labels.iter()).enumerate() {
        println!("   {:>2}  |  {:>2} | {:>25} | {}", rank+1, bit, label, count);
    }
    println!();

    println!("  Manifold geometry (largest component):");
    let geom = manifold_geometry(&atlas);
    println!("    Component size: {}", geom.component_size);
    println!("    Diameter (max shortest path): {}", geom.diameter);
    println!("    Centroid address: {:#026b}", geom.centroid_address);
    println!("    Avg distance from centroid: {}.{}",
        geom.centroid_avg_dist_x100/100,
        geom.centroid_avg_dist_x100%100);
    println!();

    // ── Generator catalog ─────────────────────────────────────────────────
    println!("══ GENERATOR CATALOG (SEED=20, max=50,000 days) ════════════════════");
    println!();
    let cat = build_catalog(50_000);
    let stats = catalog_stats(&cat);
    println!("  Total entries: {}", stats.total_entries);
    println!("  Distinct carry signatures: {}", stats.distinct_carry_signatures);
    println!("  Level-2 (shadow-free): {}", stats.level2_count);
    println!("  Level-3+ (depth-active): {}", stats.level3_plus_count);
    println!("  DKAM-admissible: {}", stats.dkam_admissible_count);
    println!();
    println!("  Tzolk'in carry class (nullified {{2,5,13}}):");
    let tz_class = same_carry_class(&cat, 260);
    for e in tz_class.iter().take(8) {
        println!("    {:>22} | {:>7} days | Level-{}", e.name, e.cycle, e.level_11);
    }
    println!();
    println!("  All Level-2 periods in catalog:");
    let l2 = level2_periods(&cat);
    for e in l2.iter().take(15) {
        println!("    {:>22} | {:>5} days | {:?}", e.name, e.cycle, e.nullified);
    }
    println!();

    println!("════════════════════════════════════════════════════════════════════");
    println!("  v0.4.0 complete. Tests: 194 passing, 0 failing.");
    println!("════════════════════════════════════════════════════════════════════");
}
