//! Comprehensive findings query: run all completed DCCMS instruments
//! on the canonical event corpus and report what emerges.

use dccms_atlas::events::EventSet;
use dccms_atlas::heads::{
    FourCalendarHydra, saturn_11_squared_head, zodiac_topology_head,
    temperaments_4fold_head, venus_phase_head, planetary_council_head,
    eclipse_alternation_head,
};
use dccms_atlas::atlas::{ConfigAtlas, head_progression};
use dccms_atlas::h3_mi::h3_mi_report;
use dccms_atlas::h5_navigator::compute_h5_report;
use dccms_atlas::manifold_geometry::{
    ManifoldProfile, BitCorrelationMatrix, hamming_1_connectivity,
};

fn main() {
    println!("════════════════════════════════════════════════════════════════════");
    println!("  DCCMS — Comprehensive Findings Query");
    println!("════════════════════════════════════════════════════════════════════");
    println!();

    let corpus = EventSet::canonical_corpus(0, 200_000);
    let hydra = FourCalendarHydra::canonical();
    let atlas = ConfigAtlas::build_four_head(&corpus, &hydra);

    println!("Corpus: {} canonical events over 200,000 days", corpus.len());
    println!("Four-head atlas: {} occupied addresses (of 16,777,216)", atlas.occupied_addresses());
    println!();

    // ── Head progression ─────────────────────────────────────────────
    println!("══ H2: HEAD PROGRESSION ═══════════════════════════════════════════");
    println!();
    println!("  Heads | Occupied | Gini (bp) | Mean occ (bp)");
    println!("  ──────┼──────────┼───────────┼──────────────");
    for (k, atlas) in head_progression(&corpus, &hydra) {
        println!("    {}   |  {:>6}  |   {:>5}   |   {:>6}",
                 k, atlas.occupied_addresses(),
                 atlas.gini_basis_points(),
                 atlas.mean_occupancy_bp());
    }
    println!();
    println!("  Reading: occupied addresses grow with head count (29→179→327→1022),");
    println!("  confirming that each calendar adds genuine resolution. The Gini");
    println!("  redistributes as the address space becomes more populated.");
    println!();

    // ── H3 mutual information ────────────────────────────────────────
    println!("══ H3: CANDIDATE HEAD INDEPENDENCE (Mutual Information) ═══════════");
    println!();
    println!("  Independence ratio: fraction of candidate's entropy that is");
    println!("  NOT predictable from the four-head address. Higher = more");
    println!("  structurally independent (more admissible as configuration).");
    println!();
    println!("  Candidate              | H(four)| H(cand)| I(4;c) | Indep | Ratio");
    println!("  ───────────────────────┼────────┼────────┼────────┼───────┼──────");

    let candidates = [
        saturn_11_squared_head(),
        zodiac_topology_head(),
        temperaments_4fold_head(),
        venus_phase_head(),
        planetary_council_head(),
        eclipse_alternation_head(),
    ];

    for cand in &candidates {
        let r = h3_mi_report(&corpus, &hydra, cand, 3000);
        let marker = if r.admissible { "★" } else { " " };
        println!(
            "  {} {:<22}| {:>5} | {:>5} | {:>5} | {:>5} | {:>3}%",
            marker,
            r.candidate_name,
            r.four_head_entropy_nbp / 10,  // displayed as natural-log × 100
            r.candidate_entropy_nbp / 10,
            r.mutual_information_nbp / 10,
            r.independent_information_nbp / 10,
            r.independence_ratio_bp / 100,
        );
    }
    println!();
    println!("  ★ = admissible at independence ratio ≥ 30%");
    println!();

    // ── H5 navigator with per-phase tests ────────────────────────────
    println!("══ H5: PRIME 11 AS CONFIGURATION NAVIGATOR ════════════════════════");
    println!();
    let h5 = compute_h5_report(&corpus, &hydra, 500, 1000);
    println!("  Four-head 11-lane max deviation: {} bp", h5.four_head_histogram.max_deviation_bp());
    println!("  Four-head uniform: {}", h5.four_head_uniform);
    println!();
    println!("  Per-head per-phase 11-lane deviations (nonuniform threshold: 1000 bp):");
    println!("  Head           | Max per-phase dev (bp) | Non-uniform?");
    println!("  ───────────────┼────────────────────────┼─────────────");
    for head in hydra.heads() {
        let name = head.signature.name;
        let dev = h5.per_phase_max_deviation_bp.get(name).copied().unwrap_or(0);
        let nu = h5.per_phase_nonuniform.get(name).copied().unwrap_or(false);
        let marker = if nu { "✓" } else { " " };
        println!("  {} {:<13}|        {:>5}           |     {}",
                 marker, name, dev, nu);
    }
    println!();
    println!("  Verdict: {}", h5.verdict.label());
    println!();
    println!("  H5 prediction: 11-lane should be UNIFORM across the four-head");
    println!("  address space (because 11 labels configuration identity, not");
    println!("  position) AND NON-UNIFORM within per-phase bins (because 11");
    println!("  tracks drift within a configuration).");
    println!();

    // ── Manifold geometry ───────────────────────────────────────────
    println!("══ CONFIGURATION MANIFOLD GEOMETRY ════════════════════════════════");
    println!();
    let profile = ManifoldProfile::from_atlas(&atlas);
    println!("  Occupied addresses: {}", profile.occupied);
    println!("  Effective dimensionality: {} bits (of 24 formal)", profile.effective_dimension);
    println!("  Always-zero bits: {:?}", profile.always_zero_bits);
    println!("  Always-one bits: {:?}", profile.always_one_bits);
    println!("  Effective capacity: {}", profile.effective_capacity());
    println!("  Saturation: {} bp ({}%)",
             profile.saturation_bp(),
             profile.saturation_bp() / 100);
    println!();
    println!("  Per-head bit usage totals:");
    let per_head = profile.per_head_total_bit_usage();
    let head_names = ["Tzolkin", "Haab", "CalendarRound", "LongCount"];
    for (i, &count) in per_head.iter().enumerate() {
        let pct = (count * 10000) / (profile.occupied as u64 * 6);
        println!("    {:<14} → {:>5} bits set ({}% of head's max)",
                 head_names[i], count, pct / 100);
    }
    println!();
    println!("  Hamming geometry of occupied set:");
    println!("    Max pairwise Hamming distance: {}", profile.max_hamming_distance);
    println!("    Mean pairwise Hamming distance: {}.{:02}",
             profile.mean_hamming_distance_bp / 10000,
             (profile.mean_hamming_distance_bp % 10000) / 100);
    println!();

    // ── Connectivity ────────────────────────────────────────────────
    println!("══ HAMMING-1 CONNECTIVITY ═════════════════════════════════════════");
    println!();
    let conn = hamming_1_connectivity(&atlas);
    println!("  Connected components (at Hamming distance 1): {}", conn.component_count);
    println!("  Largest component: {} addresses", conn.largest_component_size);
    println!("  Top 10 component sizes: {:?}",
             conn.component_sizes.iter().take(10).collect::<Vec<_>>());
    println!();
    println!("  Reading: if the 1022-address set is one connected component,");
    println!("  it forms a single manifold. If many components, the substrate");
    println!("  partitions into isolated configuration islands.");
    println!();

    // ── Bit correlations ────────────────────────────────────────────
    println!("══ TOP BIT-PAIR CORRELATIONS ══════════════════════════════════════");
    println!();
    let corr = BitCorrelationMatrix::from_atlas(&atlas);
    println!("  Top 10 co-occurring bit pairs:");
    println!("  Bit i (h/lane) | Bit j (h/lane) | Co-occur | of {} occupied",
             profile.occupied);
    println!("  ───────────────┼────────────────┼──────────┼────────────────");
    for (i, j, count) in corr.top_correlations(10) {
        let hi = i / 6;
        let li = i % 6;
        let hj = j / 6;
        let lj = j % 6;
        let bp_lanes = [2, 3, 5, 7, 11, 13];
        println!(
            "  {} (h{}/lane {:>2})  |  {} (h{}/lane {:>2})  |   {:>4}   |     {} bp",
            i, hi, bp_lanes[li],
            j, hj, bp_lanes[lj],
            count,
            ((count as u128) * 10000 / profile.occupied as u128) as u64,
        );
    }
    println!();
    println!("  Reading: high co-occurrence between bits in different heads");
    println!("  for the SAME lane prime suggests substrate redundancy.");
    println!("  High co-occurrence between bits in the SAME head for");
    println!("  different primes suggests structural lane coupling.");
    println!();

    println!("════════════════════════════════════════════════════════════════════");
    println!("  Findings query complete.");
    println!("════════════════════════════════════════════════════════════════════");
}
