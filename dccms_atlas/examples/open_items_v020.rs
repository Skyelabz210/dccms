//! v0.2.0 Comprehensive Findings — Three Open Items Closed
//!
//! This example runs all instruments from v0.1.0 plus the three new
//! modules targeting the open items from the v0.1.0 findings report:
//!
//! - **H1 generator extraction** (Stage 7 — first pass)
//! - **H5 refinements** (triple K-Elim depth analysis, κ₃ bifurcation, cross-head correlation)
//! - **H4 non-visual instruments** (four instruments requiring no imagery)

use dccms_atlas::events::EventSet;
use dccms_atlas::heads::{FourCalendarHydra};
use dccms_atlas::h1_generator::extract_generator;
use dccms_atlas::h5_refined::compute_h5_refined;
use dccms_atlas::h4_non_visual::compute_h4_non_visual;

fn main() {
    println!("════════════════════════════════════════════════════════════════════");
    println!("  DCCMS v0.2.0 — Open Items Report");
    println!("════════════════════════════════════════════════════════════════════");
    println!();

    let corpus = EventSet::canonical_corpus(0, 200_000);
    let hydra = FourCalendarHydra::canonical();

    println!("Corpus: {} events over 200,000 days", corpus.len());
    println!();

    // ── H1: Generator extraction ─────────────────────────────────────────
    println!("══ H1: GENERATOR EXTRACTION ════════════════════════════════════════");
    println!();
    let h1 = extract_generator(&hydra);

    println!("  Head invariants:");
    for inv in &h1.head_invariants {
        println!("  {:>14} | cycle={:>7} | {}-prime factors | {} phases | uniform={} | lane_ent={} bp",
            inv.name, inv.cycle, inv.prime_factor_count,
            inv.phase_count, inv.uniform_phases, inv.lane_entropy_bp);
    }
    println!();

    println!("  Carry-signature lattice (each head's S₆ carry-bit signature):");
    for inv in &h1.head_invariants {
        println!("  {:>14} | nullified={:?} | active_count={}",
            inv.name, inv.nullified, 6 - inv.nullified.len());
    }
    println!();

    println!("  Generator candidates evaluated: {}", h1.candidates.len());
    println!();
    println!("  {:>42} | matched | inv_score", "Candidate");
    println!("  {}", "─".repeat(65));
    for (cand, matched, inv_m, total) in &h1.candidates {
        let confidence = inv_m * 100 / (*total).max(1);
        println!("  {:>42} | {:>5}   | {}/{} ({}%)",
            &cand.label[..cand.label.len().min(42)],
            matched, inv_m, total, confidence);
    }
    println!();

    if let Some(ref best) = h1.best_candidate {
        println!("  Best candidate: {}", best.label);
        println!("  Phase rule: {}", best.phase_rule.label());
        println!("  Heads explained: {}", h1.heads_explained);
        println!("  Confidence: {} bp", h1.confidence_bp);
    }
    println!();

    println!("  Seed identification:");
    for (seed, head_name) in &h1.seed_map {
        println!("  seed={:>7} → head={}", seed, head_name);
    }
    println!();

    println!("  Configuration walk from 260 (6 steps):");
    let walk = dccms_atlas::h1_generator::configuration_walk(260, 6);
    for (cycle, sig, label) in &walk {
        println!("    {} | carry_sig={:#08b}", label, sig.0);
    }
    println!();
    println!("  H1 verdict: {}", h1.verdict.label());
    println!();

    // ── H5 refinements ────────────────────────────────────────────────────
    println!("══ H5 REFINED: TRIPLE K-ELIM DEPTH ANALYSIS ═══════════════════════");
    println!();
    let h5r = compute_h5_refined(&corpus, &hydra, 1000);

    println!("  Original: across-uniform max deviation = {} bp (confirmed 10 bp)", 
             h5r.across_uniform_max_dev_bp);
    println!();

    println!("  Refinement A — Triple K-Elimination depth (threshold 1000 bp):");
    println!("  {:>14} | κ₀ dev | κ₁ dev | κ₂ dev | κ₃ dev | depth-stratified?",
             "Head");
    println!("  {}", "─".repeat(75));
    for d in &h5r.depth_analyses {
        let strat = if d.depth_stratified() { "✓ YES" } else { " no" };
        println!("  {:>14} | {:>5} | {:>5} | {:>5} | {:>5} | {}",
            &d.head_name,
            d.depth_deviations[0], d.depth_deviations[1],
            d.depth_deviations[2], d.depth_deviations[3],
            strat);
    }
    println!();

    println!("  Refinement B — κ₃ bifurcation (solar vs eclipse/CalRound):");
    for (name, bp) in &h5r.kappa3.kappa3_zero_bp {
        println!("  {:>14} | κ₃=0 fraction = {} bp (expected: ~909 bp for uniform)",
            name, bp);
    }
    println!("  Bifurcation detected: {}", h5r.kappa3.bifurcation_detected);
    println!();

    println!("  Refinement C — Cross-head 11-lane correlation:");
    for c in &h5r.cross_correlations {
        println!("  {:>14} × {:>14} | agreement = {} bp | excess = {:+} bp",
            c.head_a, c.head_b, c.agreement_bp, c.excess_agreement_bp);
    }
    println!();
    println!("  Refined H5 verdict: {}", h5r.verdict.label());
    println!();

    // ── H4 non-visual ─────────────────────────────────────────────────────
    println!("══ H4 NON-VISUAL: GODDESS SECTION (pages 16-23) ═══════════════════");
    println!();
    let h4nv = compute_h4_non_visual(&hydra);

    println!("  Instrument 1 — Period-sequence entropy:");
    println!("    Null model entropy:     {} nbp", h4nv.entropy.null_entropy_nbp);
    println!("    Observed entropy:       {} nbp", h4nv.entropy.observed_entropy_nbp);
    println!("    Excess structure:       {} nbp", h4nv.entropy.excess_structure_nbp);
    println!("    Structured (≥3000 nbp): {}", h4nv.entropy.structured);
    println!();

    println!("  Instrument 2 — Residue-pattern signature:");
    println!("    Page boundary CRAM addresses (cumulative totals):");
    for (total, addr) in &h4nv.residue.page_signatures {
        println!("      day {:>5}: addr={:?}", total, addr);
    }
    println!("    Distinct addresses: {}", h4nv.residue.distinct_addresses);
    println!("    Max consecutive Hamming: {}", h4nv.residue.max_consecutive_hamming);
    println!("    Structured: {}", h4nv.residue.structured);
    println!();

    println!("  Instrument 3 — Cycle alignment:");
    println!("    Page-cycle alignments:");
    for (head_name, aligns) in &h4nv.alignment.alignments {
        let count = aligns.iter().filter(|&&a| a).count();
        let marks: String = aligns.iter().map(|&a| if a { '●' } else { '·' }).collect();
        println!("    {:>14}: [{}] ({} alignments)", head_name, marks, count);
    }
    println!("    Total alignments: {}", h4nv.alignment.total_alignments);
    println!("    Alignment density: {} bp", h4nv.alignment.alignment_density_bp);
    println!();

    println!("  Instrument 4 — Carry-flux monotonicity:");
    for (p, diff) in &h4nv.carry_flux.common_differences {
        let label = match diff {
            Some(d) => format!("AP (d={:+})", d),
            None => "non-AP".to_string(),
        };
        println!("    prime {:>2}: {}", p, label);
    }
    println!("    Primes with AP residue sequences: {}/6", h4nv.carry_flux.ap_prime_count);
    println!("    Carry flux structured: {}", h4nv.carry_flux.structured);
    println!();
    println!("  Non-visual instruments passing: {}/4", h4nv.instruments_passing);
    println!("  H4 non-visual verdict: {}", h4nv.verdict.label());
    println!();

    println!("════════════════════════════════════════════════════════════════════");
    println!("  v0.2.0 open items report complete.");
    println!("════════════════════════════════════════════════════════════════════");
}
