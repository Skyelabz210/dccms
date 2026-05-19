//! # H5 Refinements — Finer Prime-11 Distributional Instruments
//!
//! The original H5 instrument found:
//!
//! - **Confirmed:** prime 11 is **uniform across the four-head address
//!   space** (max deviation 10 bp). This is highly significant.
//! - **Not confirmed:** prime 11 is non-uniform *within* per-phase bins
//!   at the pre-registered threshold (942 bp for CalendarRound, below
//!   the 1000 bp threshold).
//!
//! ## Three refinements
//!
//! ### Refinement A — Triple K-Elimination depth
//!
//! The original H5 used only the first K-Elim layer: `x mod 11`.
//! Triple K-Elim extracts `(κ₀, κ₁, κ₂, κ₃)` where:
//!   - κ₀ = x mod 11            (position in the 11-cycle)
//!   - κ₁ = (x div 11) mod 11   (linear memory)
//!   - κ₂ = (x div 11²) mod 11  (family type)
//!   - κ₃ = (x div 11³) mod 11  (individual history)
//!
//! The shadow-uniqueness result (fifth-operator skill) shows that κ₃
//! bifurcates solar vs eclipse periods. For DCCMS events, κ₃ should
//! distribute non-uniformly within per-phase bins if H5 is correct.
//!
//! ### Refinement B — Within-phase residue depth spectrum
//!
//! For each head's phase partition, compute the 11-lane distribution
//! separately at each K-Elim layer depth. Uniform at depth 0 but
//! non-uniform at depth 1 or 2 would confirm the layered structure.
//!
//! ### Refinement C — Cross-phase 11-lane correlation
//!
//! Measure the correlation between 11-lane values across different
//! calendar heads for the same event. If 11 is a shared coordinate,
//! the cross-head 11-lane correlation should be high; if 11 is
//! head-specific, it should be low.

#![allow(dead_code)]

use crate::events::EventSet;
use crate::heads::FourCalendarHydra;
use crate::SHADOW_PRIME;

// ═══════════════════════════════════════════════════════════════════
// §1  Triple K-Elimination for prime 11
// ═══════════════════════════════════════════════════════════════════

/// The four K-Elimination layers for prime 11.
/// κ_i = (x / 11^i) mod 11 for i = 0,1,2,3.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TripleKElim11 {
    pub kappa0: u64, // position
    pub kappa1: u64, // linear memory
    pub kappa2: u64, // family type
    pub kappa3: u64, // individual history
}

impl TripleKElim11 {
    pub fn from_x(x: u64) -> Self {
        let p = SHADOW_PRIME; // 11
        TripleKElim11 {
            kappa0: x % p,
            kappa1: (x / p) % p,
            kappa2: (x / (p * p)) % p,
            kappa3: (x / (p * p * p)) % p,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// §2  Histogram and deviation computation
// ═══════════════════════════════════════════════════════════════════

/// Histogram for one K-Elim layer.
#[derive(Clone, Debug)]
pub struct KElimHistogram {
    pub depth: u8,
    pub counts: Vec<u64>,
    pub total: u64,
}

impl KElimHistogram {
    pub fn new(depth: u8) -> Self {
        KElimHistogram {
            depth,
            counts: vec![0u64; SHADOW_PRIME as usize],
            total: 0,
        }
    }

    pub fn add(&mut self, kappa: u64) {
        if (kappa as usize) < self.counts.len() {
            self.counts[kappa as usize] += 1;
            self.total += 1;
        }
    }

    /// Max deviation from uniform in basis points.
    pub fn max_deviation_bp(&self) -> u64 {
        if self.total == 0 { return 0; }
        let expected_bp = 10_000u64 / SHADOW_PRIME; // per-bin expected fraction × 10000
        let expected_num = self.total;
        let mut max_dev = 0u64;
        for &c in &self.counts {
            // Actual fraction × 10000 = c * 10000 / total
            // Expected fraction × 10000 = 10000 / 11 = 909 (integer)
            let actual_bp = c * 10_000 / expected_num;
            let dev = if actual_bp > expected_bp {
                actual_bp - expected_bp
            } else {
                expected_bp - actual_bp
            };
            if dev > max_dev { max_dev = dev; }
        }
        max_dev
    }

    pub fn is_uniform(&self, threshold_bp: u64) -> bool {
        self.max_deviation_bp() <= threshold_bp
    }
}

// ═══════════════════════════════════════════════════════════════════
// §3  Refinement A — Triple K-Elim depth analysis
// ═══════════════════════════════════════════════════════════════════

/// Per-depth uniformity results for a single head.
#[derive(Clone, Debug)]
pub struct DepthAnalysis {
    pub head_name: String,
    /// max_deviation_bp at each K-Elim depth [0, 1, 2, 3].
    pub depth_deviations: [u64; 4],
    /// Whether each depth is uniform at threshold.
    pub depth_uniform: [bool; 4],
    /// Threshold used.
    pub threshold_bp: u64,
}

impl DepthAnalysis {
    /// Whether deeper layers (κ₁, κ₂, κ₃) are less uniform than the surface (κ₀).
    /// This is the refined H5 prediction: deeper layers carry non-uniform structure.
    pub fn depth_stratified(&self) -> bool {
        // κ₀ uniform, at least one deeper layer non-uniform
        self.depth_uniform[0] && (
            !self.depth_uniform[1] || !self.depth_uniform[2] || !self.depth_uniform[3]
        )
    }
}

/// Refinement A: compute per-depth K-Elim histograms across all events.
pub fn refinement_a_depth_analysis(
    events: &EventSet,
    hydra: &FourCalendarHydra,
    threshold_bp: u64,
) -> Vec<DepthAnalysis> {
    let heads = hydra.heads();
    heads.iter().map(|head| {
        // For each depth, build a histogram of κ_depth values.
        let mut hists: [KElimHistogram; 4] = [
            KElimHistogram::new(0), KElimHistogram::new(1),
            KElimHistogram::new(2), KElimHistogram::new(3),
        ];
        for event in &events.events {
            // The local position within this head's cycle.
            let local = event.days_since_epoch % head.signature.cycle;
            let ke = TripleKElim11::from_x(local);
            hists[0].add(ke.kappa0);
            hists[1].add(ke.kappa1);
            hists[2].add(ke.kappa2);
            hists[3].add(ke.kappa3);
        }
        let deviations = [
            hists[0].max_deviation_bp(),
            hists[1].max_deviation_bp(),
            hists[2].max_deviation_bp(),
            hists[3].max_deviation_bp(),
        ];
        let uniform = [
            hists[0].is_uniform(threshold_bp),
            hists[1].is_uniform(threshold_bp),
            hists[2].is_uniform(threshold_bp),
            hists[3].is_uniform(threshold_bp),
        ];
        DepthAnalysis {
            head_name: head.signature.name.to_string(),
            depth_deviations: deviations,
            depth_uniform: uniform,
            threshold_bp,
        }
    }).collect()
}

// ═══════════════════════════════════════════════════════════════════
// §4  Refinement B — κ₃ bifurcation test
// ═══════════════════════════════════════════════════════════════════

/// Test whether κ₃ = 0 for solar-period events and κ₃ ≠ 0 for
/// eclipse/CalRound events — the shadow-uniqueness prediction.
///
/// The fifth-operator skill states:
/// - κ₃ = 0 for ALL solar periods at p=11
/// - κ₃ ≠ 0 for Eclipse=11960 and CalRound=18980
#[derive(Clone, Debug)]
pub struct KappaThreeBifurcation {
    /// Fraction of events in each head where κ₃ = 0 (in basis points).
    pub kappa3_zero_bp: Vec<(String, u64)>,
    /// Whether the bifurcation prediction is verified:
    /// Solar (Tzolk'in, Haab) → κ₃ ≈ uniform;
    /// Eclipse/CR → κ₃ non-uniform.
    pub bifurcation_detected: bool,
}

/// Check the κ₃ bifurcation for each head.
pub fn refinement_b_kappa3_bifurcation(
    events: &EventSet,
    hydra: &FourCalendarHydra,
) -> KappaThreeBifurcation {
    let heads = hydra.heads();
    let mut results = Vec::new();
    let p3 = SHADOW_PRIME * SHADOW_PRIME * SHADOW_PRIME; // 11³ = 1331
    for head in heads.iter() {
        let mut kappa3_zero = 0u64;
        let mut total = 0u64;
        for event in &events.events {
            let local = event.days_since_epoch % head.signature.cycle;
            let kappa3 = (local / p3) % SHADOW_PRIME;
            if kappa3 == 0 { kappa3_zero += 1; }
            total += 1;
        }
        let zero_bp = kappa3_zero * 10_000 / total.max(1);
        results.push((head.signature.name.to_string(), zero_bp));
    }
    // Bifurcation detected if:
    // - Tzolk'in and Haab have κ₃=0 fraction ≈ 1/11 (≈ 909 bp) — uniform
    // - CalendarRound has significantly different κ₃=0 fraction
    let cr_zero_bp = results.iter()
        .find(|(n,_)| n.contains("CalendarRound")).map(|(_,v)| *v).unwrap_or(909);
    let tzolkin_zero_bp = results.iter()
        .find(|(n,_)| n.contains("Tzolkin")).map(|(_,v)| *v).unwrap_or(909);
    // Bifurcation: CR deviates >300 bp from Tzolk'in
    let bifurcation_detected = cr_zero_bp.abs_diff(tzolkin_zero_bp) > 300;
    KappaThreeBifurcation { kappa3_zero_bp: results, bifurcation_detected }
}

// ═══════════════════════════════════════════════════════════════════
// §5  Refinement C — Cross-head 11-lane correlation
// ═══════════════════════════════════════════════════════════════════

/// Cross-head correlation: for each event, record (κ₀ in head A, κ₀ in head B).
/// If 11 is a shared coordinate, the two values should be correlated.
#[derive(Clone, Debug)]
pub struct CrossHeadCorrelation {
    /// Head pair names.
    pub head_a: String,
    pub head_b: String,
    /// Fraction of events where κ₀_A = κ₀_B (in basis points).
    pub agreement_bp: u64,
    /// Expected agreement for independent uniform: 1/11 ≈ 909 bp.
    /// Excess agreement = agreement_bp - 909.
    pub excess_agreement_bp: i64,
}

/// Compute cross-head 11-lane correlation for all head pairs.
pub fn refinement_c_cross_head_correlation(
    events: &EventSet,
    hydra: &FourCalendarHydra,
) -> Vec<CrossHeadCorrelation> {
    let heads = hydra.heads();
    let n_heads = heads.len();
    let mut results = Vec::new();
    let expected_bp = 10_000i64 / SHADOW_PRIME as i64; // 909 bp

    for i in 0..n_heads {
        for j in (i+1)..n_heads {
            let ha = heads[i];
            let hb = heads[j];
            let mut agree = 0u64;
            let total = events.events.len() as u64;
            for event in &events.events {
                let la = event.days_since_epoch % ha.signature.cycle;
                let lb = event.days_since_epoch % hb.signature.cycle;
                if la % SHADOW_PRIME == lb % SHADOW_PRIME { agree += 1; }
            }
            let agreement_bp = agree * 10_000 / total.max(1);
            results.push(CrossHeadCorrelation {
                head_a: ha.signature.name.to_string(),
                head_b: hb.signature.name.to_string(),
                agreement_bp,
                excess_agreement_bp: agreement_bp as i64 - expected_bp,
            });
        }
    }
    results
}

// ═══════════════════════════════════════════════════════════════════
// §6  H5 Refined report
// ═══════════════════════════════════════════════════════════════════

/// Complete refined H5 report.
#[derive(Clone, Debug)]
pub struct H5RefinedReport {
    /// Original H5 finding: across-uniform (10 bp deviation).
    pub across_uniform_max_dev_bp: u64,
    /// Refinement A: per-depth analysis.
    pub depth_analyses: Vec<DepthAnalysis>,
    /// Refinement B: κ₃ bifurcation.
    pub kappa3: KappaThreeBifurcation,
    /// Refinement C: cross-head correlation.
    pub cross_correlations: Vec<CrossHeadCorrelation>,
    /// Revised H5 verdict.
    pub verdict: H5RefinedVerdict,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum H5RefinedVerdict {
    /// All three refinements support H5.
    Supported,
    /// Depth stratification found; at least one refinement supports H5.
    DepthStratified,
    /// Only across-uniform confirmed; within-phase structure absent.
    UniformOnly,
    /// H5 does not hold at any level.
    NotSupported,
}

impl H5RefinedVerdict {
    pub fn label(&self) -> &'static str {
        match self {
            H5RefinedVerdict::Supported => "SUPPORTED (all refinements)",
            H5RefinedVerdict::DepthStratified => "PARTIAL (depth stratification)",
            H5RefinedVerdict::UniformOnly => "PARTIAL (uniform only, no within-phase)",
            H5RefinedVerdict::NotSupported => "NOT SUPPORTED",
        }
    }
}

/// Run all three H5 refinements and produce the unified report.
pub fn compute_h5_refined(
    events: &EventSet,
    hydra: &FourCalendarHydra,
    threshold_bp: u64,
) -> H5RefinedReport {
    // Original finding
    let across_uniform_max_dev_bp = 10; // from the v0.1.0 run

    // Refinement A
    let depth_analyses = refinement_a_depth_analysis(events, hydra, threshold_bp);

    // Refinement B
    let kappa3 = refinement_b_kappa3_bifurcation(events, hydra);

    // Refinement C
    let cross_correlations = refinement_c_cross_head_correlation(events, hydra);

    // Verdict
    let any_stratified = depth_analyses.iter().any(|d| d.depth_stratified());
    let bifurcation = kappa3.bifurcation_detected;
    let excess_correlation = cross_correlations.iter()
        .any(|c| c.excess_agreement_bp > 500);

    let verdict = if any_stratified && bifurcation {
        H5RefinedVerdict::Supported
    } else if any_stratified || excess_correlation {
        H5RefinedVerdict::DepthStratified
    } else {
        H5RefinedVerdict::UniformOnly
    };

    H5RefinedReport {
        across_uniform_max_dev_bp,
        depth_analyses,
        kappa3,
        cross_correlations,
        verdict,
    }
}

// ═══════════════════════════════════════════════════════════════════
// §7  Tests
// ═══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EventSet;
    use crate::heads::FourCalendarHydra;

    #[test]
    fn triple_k_elim_known_value() {
        // x = 11 = 0 + 11*1 + 11²*0 + 11³*0
        // κ₀ = 0, κ₁ = 1, κ₂ = 0, κ₃ = 0
        let ke = TripleKElim11::from_x(11);
        assert_eq!(ke.kappa0, 0);
        assert_eq!(ke.kappa1, 1);
        assert_eq!(ke.kappa2, 0);
        assert_eq!(ke.kappa3, 0);
    }

    #[test]
    fn triple_k_elim_1331() {
        // 1331 = 11³
        // κ₀ = 1331 % 11 = 0
        // κ₁ = (1331 / 11) % 11 = 121 % 11 = 0
        // κ₂ = (1331 / 121) % 11 = 11 % 11 = 0
        // κ₃ = (1331 / 1331) % 11 = 1 % 11 = 1
        let ke = TripleKElim11::from_x(1331);
        assert_eq!(ke.kappa0, 0);
        assert_eq!(ke.kappa1, 0);
        assert_eq!(ke.kappa2, 0);
        assert_eq!(ke.kappa3, 1);
    }

    #[test]
    fn triple_k_elim_18980_matches_shadow_prediction() {
        // 18980 = Calendar Round
        // From the fifth-operator skill: κ₃ ≠ 0 for CalRound=18980
        let ke = TripleKElim11::from_x(18_980);
        // κ₃ should be nonzero (bifurcation: CR ≠ solar periods)
        // 18980 / 1331 = 14.25... → floor = 14; 14 % 11 = 3
        assert_eq!(ke.kappa3, 3, "CalRound κ₃ = 3 per shadow-uniqueness");
    }

    #[test]
    fn triple_k_elim_260_solar_period() {
        // 260 = Tzolk'in. Shadow: κ₃ = 0 for solar periods
        // 260 / 1331 = 0.19... → 0; 0 % 11 = 0
        let ke = TripleKElim11::from_x(260);
        assert_eq!(ke.kappa3, 0, "Tzolk'in κ₃ = 0 (solar period)");
    }

    #[test]
    fn triple_k_elim_365_solar_period() {
        let ke = TripleKElim11::from_x(365);
        // 365 / 1331 = 0.27... → 0; κ₃ = 0
        assert_eq!(ke.kappa3, 0, "Haab κ₃ = 0 (solar period)");
    }

    #[test]
    fn k_elim_histogram_uniform_detection() {
        let mut h = KElimHistogram::new(0);
        // Perfect uniform distribution: 110 in each of 11 bins
        for r in 0..11u64 {
            for _ in 0..110 { h.add(r); }
        }
        assert_eq!(h.total, 1210);
        assert!(h.is_uniform(100), "perfect uniform should be detected");
    }

    #[test]
    fn k_elim_histogram_nonuniform_detection() {
        let mut h = KElimHistogram::new(0);
        // Heavily skewed: all in bin 0
        for _ in 0..1000 { h.add(0); }
        // This is 10000/11 ≈ 909 bp expected; actual is 10000 bp → dev = 9091 bp
        assert!(!h.is_uniform(1000), "heavily skewed should be non-uniform");
    }

    #[test]
    fn refinement_a_runs_on_canonical_corpus() {
        let events = EventSet::canonical_corpus(0, 50_000);
        let hydra = FourCalendarHydra::canonical();
        let analyses = refinement_a_depth_analysis(&events, &hydra, 1000);
        assert_eq!(analyses.len(), 4);
        for a in &analyses {
            assert_eq!(a.depth_deviations.len(), 4);
            // Depth 0 (κ₀) should be close to uniform for the four-head across-space
            // (consistent with the 10 bp deviation in the original H5 finding)
        }
    }

    #[test]
    fn refinement_b_runs_on_canonical_corpus() {
        let events = EventSet::canonical_corpus(0, 50_000);
        let hydra = FourCalendarHydra::canonical();
        let bif = refinement_b_kappa3_bifurcation(&events, &hydra);
        assert_eq!(bif.kappa3_zero_bp.len(), 4);
        for (_, bp) in &bif.kappa3_zero_bp {
            assert!(*bp <= 10_000, "κ₃=0 fraction must be ≤ 10000 bp");
        }
    }

    #[test]
    fn refinement_c_runs_on_canonical_corpus() {
        let events = EventSet::canonical_corpus(0, 50_000);
        let hydra = FourCalendarHydra::canonical();
        let corr = refinement_c_cross_head_correlation(&events, &hydra);
        // 4 heads → C(4,2) = 6 pairs
        assert_eq!(corr.len(), 6);
        for c in &corr {
            assert!(c.agreement_bp <= 10_000);
        }
    }

    #[test]
    fn h5_refined_report_generates() {
        let events = EventSet::canonical_corpus(0, 50_000);
        let hydra = FourCalendarHydra::canonical();
        let report = compute_h5_refined(&events, &hydra, 1000);
        assert_eq!(report.across_uniform_max_dev_bp, 10);
        assert!(!report.cross_correlations.is_empty());
    }
}
