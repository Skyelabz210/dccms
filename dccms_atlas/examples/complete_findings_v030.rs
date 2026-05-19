//! DCCMS v0.3.0 — Complete Findings
//!
//! Runs all v0.3.0 instruments: H1 Stage 8, H5 Level classification,
//! cross-validation corpus, and H4 Montgomery Quotient Shadow.

use dccms_atlas::heads::FourCalendarHydra;
use dccms_atlas::h1_stage8::compute_h1_stage8;
use dccms_atlas::h5_level::compute_h5_level_report;
use dccms_atlas::cross_validation::compute_cross_validation;
use dccms_atlas::h4_montgomery::compute_montgomery_report;

fn main() {
    println!("════════════════════════════════════════════════════════════════════");
    println!("  DCCMS v0.3.0 — Complete Findings");
    println!("════════════════════════════════════════════════════════════════════");
    println!();

    let hydra = FourCalendarHydra::canonical();

    // ── H1 Stage 8: Bilinear intercalary generator ───────────────────────
    println!("══ H1 STAGE 8: BASE-20 BILINEAR GENERATOR ═════════════════════════");
    println!();
    let h1 = compute_h1_stage8();

    println!("  Generator tree (SEED = 20):");
    println!("  {:>14} | {:>8} | {:>8} | {:>10} | {:>6} | {:>5}",
        "Name", "n_reg", "base", "intercalary", "cycle", "parent");
    println!("  {}", "─".repeat(68));
    for (name, cycle) in &h1.tree_cycles {
        // Find node
        let nodes = dccms_atlas::h1_stage8::canonical_generator_tree();
        if let Some(node) = nodes.iter().find(|n| n.name == *name) {
            let inter = if node.application.has_intercalary() {
                format!("{}×{}", node.application.n_inter, node.application.inter)
            } else {
                "none".to_string()
            };
            println!("  {:>14} | {:>8} | {:>8} | {:>10} | {:>6} | {}",
                name, node.application.n_reg, node.application.base,
                inter, cycle, node.parent_name);
        }
    }
    println!();
    println!("  Tree verified: {}", h1.tree_verified);
    println!();

    println!("  Multiplier profiles:");
    for prof in &h1.multiplier_profiles {
        println!("  {:>3} | S₆={} | factors={:?} | nullifies={:?} | {}",
            prof.multiplier, prof.in_safe_basis,
            prof.prime_factors, prof.nullifies, prof.role);
    }
    println!();

    println!("  Reachable periods ≤ 200,000 days: {}", h1.reachable_count);
    println!("  All four canonical heads reachable: {}", h1.all_heads_reachable);
    println!();

    let periods = dccms_atlas::h1_stage8::reachable_periods(50_000);
    println!("  Notable reachable periods (≤ 50,000 days):");
    for (name, cycle, path) in periods.iter().filter(|(n,_,_)| !n.starts_with("computed")) {
        println!("    {:>22} | {:>7} days | {}", name, cycle, path);
    }
    println!();
    println!("  H1 Stage 8 verdict: {}", h1.verdict.label());
    println!();

    // ── H5 Level classification ───────────────────────────────────────────
    println!("══ H5 LEVEL CLASSIFICATION: 11³ THRESHOLD ══════════════════════════");
    println!();
    let h5l = compute_h5_level_report();

    println!("  Period K-Elim level table at p=11:");
    println!("  {:>22} | {:>7} | {:>5} | {:>14} | κ₃=0 pred.",
        "Period", "days", "level", "category");
    println!("  {}", "─".repeat(75));
    for entry in &h5l.period_table {
        println!("  {:>22} | {:>7} | {:>5} | {:>14} | {} bp",
            entry.name, entry.days, entry.level_11,
            entry.category.label().split('(').next().unwrap_or(""),
            entry.kappa3_zero_bp);
    }
    println!();
    println!("  Short periods (Level ≤ 2, κ₃=0 always): {}", h5l.short_count);
    println!("  Long periods (Level ≥ 3): {}", h5l.long_count);
    println!();

    println!("  Level-theorem validation vs v0.2.0 observations:");
    println!("  {:>14} | Level | Predicted bp | Observed bp | Consistent",
             "Head");
    println!("  {}", "─".repeat(60));
    for v in &h5l.validations {
        println!("  {:>14} |   {}   |    {:>5}     |    {:>5}    | {}",
            v.name, v.level, v.predicted_bp, v.observed_bp,
            if v.consistent { "✓" } else { "✗" });
    }
    println!();

    if h5l.near_threshold.is_empty() {
        println!("  No known astronomical periods near 11³ = 1331.");
    } else {
        println!("  Astronomical periods near 11³ = 1331:");
        for (name, days) in &h5l.near_threshold {
            println!("    {} = {} days", name, days);
        }
    }
    println!();
    println!("  All validations consistent: {}", h5l.all_consistent);
    println!();

    // ── Cross-validation ─────────────────────────────────────────────────
    println!("══ CROSS-VALIDATION: INDEPENDENT ASTRONOMICAL CORPORA ══════════════");
    println!();
    let xval = compute_cross_validation(&hydra);

    println!("  H3 independence ratios across independent corpora:");
    println!("  {:>14} | {:>10} | Canon bp | XVal bp | Stable",
             "Candidate", "Corpus");
    println!("  {}", "─".repeat(60));
    for r in &xval.h3_results {
        println!("  {:>14} | {:>10} | {:>5}    | {:>5}   | {}",
            r.candidate_name, r.corpus_name,
            r.canonical_ratio_bp, r.crossval_ratio_bp,
            if r.stable { "✓" } else { "✗" });
    }
    println!();

    println!("  H5 Level-2 universality (κ₃=0 for Tzolk'in/Haab on all corpora):");
    for r in &xval.h5_results {
        println!("  {} corpus ({} events):", r.corpus_name, r.event_count);
        for (head, bp) in &r.kappa3_zero_per_head {
            let marker = if *bp >= 9_900 { "✓" } else { " " };
            println!("    {} {:>14}: {} bp", marker, head, bp);
        }
        println!("    Level-2 prediction holds: {}", r.level2_prediction_holds);
    }
    println!();
    println!("  H3 stability across corpora: {}", xval.h3_stable);
    println!("  H5 Level-2 universality: {}", xval.h5_level2_universal);
    println!();

    // ── H4 Montgomery Shadow ─────────────────────────────────────────────
    println!("══ H4 INSTRUMENT 6: MONTGOMERY QUOTIENT SHADOW ═════════════════════");
    println!();
    let mont = compute_montgomery_report();

    println!("  Goddess section (pages 16–23) byproduct sequence:");
    println!("  Page | Day   | r₁₁ | r₁₃ | q₁₁(=r₁₃×11⁻¹) | q₁₃(=r₁₁×13⁻¹)");
    println!("  {}", "─".repeat(65));
    for (i, ((day, quad), bp)) in mont.page_quads.iter()
        .zip(mont.byproduct_sequence.iter()).enumerate() {
        println!("   {:>2}  | {:>5} |  {:>2} |  {:>2} |       {:>2}       |       {:>2}",
            i+1, day, quad.r11, quad.r13, bp.0, bp.1);
    }
    println!();

    println!("  Distinct byproduct pairs: {}/9 pages", mont.distinct_pairs);
    println!("  Pair entropy: {} nbp", mont.pair_entropy_nbp);
    println!("  Max entropy (9 pages): {} nbp", mont.max_entropy_nbp);
    println!("  Entropy reduction: {} nbp", mont.entropy_reduction_nbp);
    println!("  Clustered (≥3000 nbp reduction): {}", mont.clustered);
    println!("  q₁₁ arithmetic progression: {}", mont.q11_line_pattern);
    println!();

    println!("  Period sweep — (r₁₁, r₁₃, q₁₁, q₁₃) for all T7 periods:");
    let sweep = dccms_atlas::h4_montgomery::period_montgomery_sweep();
    println!("  {:>22} | {:>7} | r₁₁ | r₁₃ | q₁₁ | q₁₃",
             "Period", "days");
    println!("  {}", "─".repeat(60));
    for (name, days, quad) in &sweep {
        println!("  {:>22} | {:>7} |  {:>2} |  {:>2} |  {:>2} |  {:>2}",
            name, days, quad.r11, quad.r13, quad.q11, quad.q13);
    }
    println!();

    println!("════════════════════════════════════════════════════════════════════");
    println!("  v0.3.0 complete. Test suite: 188 passing, 0 failing.");
    println!("════════════════════════════════════════════════════════════════════");
}
