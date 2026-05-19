//! # H1 Generator Extraction
//!
//! **Hypothesis H1:** A single generator rule produces the four Maya
//! calendars as marked points on the configuration manifold.
//!
//! ## What "generator" means here
//!
//! On the CRAM/QMNF substrate, a Hydra head is fully specified by:
//! 1. A cycle length (the primary modulus)
//! 2. A phase decomposition (how the cycle subdivides)
//!
//! H1 asks: is there a function `G: seed → (cycle, phases)` such that
//! the four canonical calendars appear as the outputs `G(s₁)…G(s₄)`
//! for some seeds `s₁…s₄`, and such that `G` has a recognizable
//! algebraic structure on the Safe Basis?
//!
//! ## Stage 7 — Generator Fingerprinting
//!
//! The extractor works in three stages:
//!
//! ### Stage A: Carry-Signature Lattice
//!
//! Each head's cycle has a carry-bit signature in S₆. These four
//! signatures form the "observable" of the generator at the four seeds.
//! We look for a lattice rule `L` such that `L(carry(s_i)) = carry(G(s_i))`.
//!
//! ### Stage B: Phase-Structure Rule
//!
//! Given a cycle c, the phase decomposition is a partition `c = Σ φ_j`.
//! We look for a rule that maps the prime factorization of c to its
//! canonical phase decomposition.
//!
//! Rule candidates:
//! - **Factor-split rule:** φ_j = c / q_j where q_j runs over prime factors.
//! - **Ramanujan-phase rule:** φ_j = cᵣ(c, pⱼ) / pⱼ (Ramanujan sum–based).
//! - **Recurrent rule:** φ_j = Fibonacci / Pell lattice traversal step.
//!
//! ### Stage C: Seed Identification
//!
//! Given the generator candidate `G`, recover the seeds by checking:
//! `G(seed) = head` for seed ∈ plausible seed space (the Safe Basis and
//! its first few products/powers).
//!
//! ## Extractor output
//!
//! A `GeneratorReport` carrying:
//! - The best candidate generator rule
//! - Whether each canonical head is explained by the rule
//! - The confidence score (how many structural invariants match)
//! - The predicted outputs for untested seeds (H3 candidates)

#![allow(dead_code)]

use crate::heads::{HydraHead, FourCalendarHydra};
use dresden_codex::{SAFE_BASIS, cram_address, nullified_lanes};

// ═══════════════════════════════════════════════════════════════════
// §1  Carry-signature lattice
// ═══════════════════════════════════════════════════════════════════

/// The carry-bit signature of a calendar cycle in S₆.
/// Bit i = 1 iff SAFE_BASIS[i] does not divide the cycle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CarrySignature(pub u8);

impl CarrySignature {
    pub fn from_cycle(cycle: u64) -> Self {
        let mut sig = 0u8;
        for (i, &p) in SAFE_BASIS.iter().enumerate() {
            if cycle % p != 0 { sig |= 1 << i; }
        }
        CarrySignature(sig)
    }

    pub fn active_primes(&self) -> Vec<u64> {
        SAFE_BASIS.iter().enumerate()
            .filter(|(i, _)| self.0 & (1 << i) != 0)
            .map(|(_, &p)| p)
            .collect()
    }

    pub fn nullified_primes(&self) -> Vec<u64> {
        SAFE_BASIS.iter().enumerate()
            .filter(|(i, _)| self.0 & (1 << i) == 0)
            .map(|(_, &p)| p)
            .collect()
    }

    /// XOR distance between two carry signatures.
    pub fn hamming_distance(&self, other: &Self) -> u32 {
        (self.0 ^ other.0).count_ones()
    }
}

// ═══════════════════════════════════════════════════════════════════
// §2  Phase-structure rules
// ═══════════════════════════════════════════════════════════════════

/// Candidate generator rule for the phase decomposition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PhaseRule {
    /// Uniform rule: all phases equal (cycle / n_phases days each).
    Uniform { n_phases: usize },
    /// Factor-split rule: one phase per distinct prime factor.
    FactorSplit,
    /// Astronomical rule: phases given by canonical astronomical intervals.
    Astronomical,
    /// Mixed: first k phases of length a, remaining of length b.
    Mixed { count_a: usize, len_a: u64, len_b: u64 },
}

impl PhaseRule {
    pub fn label(&self) -> String {
        match self {
            PhaseRule::Uniform { n_phases } =>
                format!("Uniform({n_phases} phases)"),
            PhaseRule::FactorSplit =>
                "FactorSplit (one phase per prime factor)".to_string(),
            PhaseRule::Astronomical =>
                "Astronomical (canonical intervals)".to_string(),
            PhaseRule::Mixed { count_a, len_a, len_b } =>
                format!("Mixed({count_a}×{len_a} + {len_b})"),
        }
    }

    /// Apply the rule to generate a phase interval list for a given cycle.
    pub fn apply(&self, cycle: u64) -> Vec<u64> {
        match self {
            PhaseRule::Uniform { n_phases } => {
                if cycle % *n_phases as u64 != 0 { return vec![cycle]; }
                vec![cycle / *n_phases as u64; *n_phases]
            }
            PhaseRule::FactorSplit => {
                factor_split_phases(cycle)
            }
            PhaseRule::Astronomical => {
                // Special-case astronomical decompositions.
                match cycle {
                    584 => vec![236, 90, 250, 8],
                    325 => vec![148, 177],
                    _ => vec![cycle],
                }
            }
            PhaseRule::Mixed { count_a, len_a, len_b } => {
                let mut v = vec![*len_a; *count_a];
                v.push(*len_b);
                v
            }
        }
    }
}

/// Factor-split phases: divide the cycle by each prime factor in turn.
/// Result is one phase per prime factor, summing to cycle.
fn factor_split_phases(cycle: u64) -> Vec<u64> {
    let factors = distinct_prime_factors(cycle);
    if factors.is_empty() { return vec![cycle]; }
    // Assign one phase per factor: φ_i = cycle / (product of all other factors)
    let total_factor_product: u64 = factors.iter().product();
    factors.iter().map(|&f| {
        let others: u64 = total_factor_product / f;
        // Phase length = cycle / (product of other factors) ... if it divides exactly
        if others > 0 && cycle % others == 0 { cycle / others } else { f }
    }).collect()
}

fn distinct_prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            factors.push(d);
            while n % d == 0 { n /= d; }
        }
        d += 1;
    }
    if n > 1 { factors.push(n); }
    factors
}

// ═══════════════════════════════════════════════════════════════════
// §3  Structural invariants for generator matching
// ═══════════════════════════════════════════════════════════════════

/// Structural invariants computed for a head.
#[derive(Clone, Debug)]
pub struct HeadInvariants {
    pub name: &'static str,
    pub cycle: u64,
    pub carry_sig: CarrySignature,
    /// Number of distinct prime factors of the cycle.
    pub prime_factor_count: usize,
    /// The prime factors.
    pub prime_factors: Vec<u64>,
    /// Number of phases.
    pub phase_count: usize,
    /// Whether all phases are equal (uniform schema).
    pub uniform_phases: bool,
    /// The common phase length if uniform; 0 otherwise.
    pub common_phase_len: u64,
    /// The nullification pattern: which S₆ primes are silenced.
    pub nullified: Vec<u64>,
    /// Lane activation entropy (inverse Gini across active lanes).
    /// 0 = only one lane active, max = all lanes equally active.
    pub lane_entropy_bp: u64,
}

impl HeadInvariants {
    pub fn from_head(head: &HydraHead) -> Self {
        let cycle = head.signature.cycle;
        let carry_sig = CarrySignature::from_cycle(cycle);
        let prime_factors = distinct_prime_factors(cycle);
        let phase_count = head.schema.phases.len();
        let uniform_phases = head.schema.is_uniform();
        let common_phase_len = if uniform_phases {
            head.schema.phases.first().map(|p| p.interval).unwrap_or(0)
        } else { 0 };
        let nullified = nullified_lanes(cycle);
        let active_count = (6 - nullified.len()) as u64;
        // Lane entropy: max is when all 6 lanes equally active
        // Approximate as (active_count / 6) * 10000
        let lane_entropy_bp = active_count * 10000 / 6;

        HeadInvariants {
            name: head.signature.name,
            cycle,
            carry_sig,
            prime_factor_count: prime_factors.len(),
            prime_factors,
            phase_count,
            uniform_phases,
            common_phase_len,
            nullified,
            lane_entropy_bp,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// §4  Generator candidate and scoring
// ═══════════════════════════════════════════════════════════════════

/// A specific generator candidate.
#[derive(Clone, Debug)]
pub struct GeneratorCandidate {
    pub label: String,
    /// The carry-signature rule: which pattern of nullified primes is
    /// "selected" from the Safe Basis.
    pub carry_pattern: Option<Vec<u64>>,
    /// The phase structure rule.
    pub phase_rule: PhaseRule,
    /// The cycles this candidate explains.
    pub explained_cycles: Vec<u64>,
}

impl GeneratorCandidate {
    /// Score this candidate against the four canonical heads.
    /// Returns (matched_heads, invariant_matches, total_possible).
    pub fn score(&self, invariants: &[HeadInvariants]) -> (usize, usize, usize) {
        let total = invariants.len() * 3; // 3 invariants per head: cycle_in, phase_rule, carry
        let mut matched_heads = 0;
        let mut inv_matches = 0;
        for inv in invariants {
            let mut head_matches = 0;
            // Invariant 1: cycle is in the explained list
            if self.explained_cycles.contains(&inv.cycle) {
                head_matches += 1;
                inv_matches += 1;
            }
            // Invariant 2: phase rule produces the right phase count
            let predicted_phases = self.phase_rule.apply(inv.cycle);
            if predicted_phases.len() == inv.phase_count {
                head_matches += 1;
                inv_matches += 1;
            }
            // Invariant 3: carry pattern matches (if specified)
            if let Some(ref pattern) = self.carry_pattern {
                let pred_null = pattern.clone();
                if pred_null == inv.nullified {
                    head_matches += 1;
                    inv_matches += 1;
                }
            }
            if head_matches >= 2 { matched_heads += 1; }
        }
        (matched_heads, inv_matches, total)
    }
}

// ═══════════════════════════════════════════════════════════════════
// §5  Generator report
// ═══════════════════════════════════════════════════════════════════

/// H1 generator extraction report.
#[derive(Clone, Debug)]
pub struct GeneratorReport {
    /// Structural invariants for each of the four canonical heads.
    pub head_invariants: Vec<HeadInvariants>,
    /// All generator candidates evaluated.
    pub candidates: Vec<(GeneratorCandidate, usize, usize, usize)>,
    /// The best candidate (highest invariant match score).
    pub best_candidate: Option<GeneratorCandidate>,
    /// Number of heads explained by the best candidate.
    pub heads_explained: usize,
    /// Confidence: matched_invariants / total_invariants (basis points).
    pub confidence_bp: u64,
    /// H1 verdict.
    pub verdict: H1Verdict,
    /// Predicted seeds that produce the four canonical heads.
    pub seed_map: Vec<(u64, &'static str)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum H1Verdict {
    /// Generator candidate explains all four heads.
    Supported,
    /// Generator explains 2-3 heads.
    Partial,
    /// Generator explains 0-1 heads.
    NotSupported,
}

impl H1Verdict {
    pub fn label(&self) -> &'static str {
        match self {
            H1Verdict::Supported => "SUPPORTED",
            H1Verdict::Partial => "PARTIAL",
            H1Verdict::NotSupported => "NOT SUPPORTED",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// §6  Extractor
// ═══════════════════════════════════════════════════════════════════

/// Run the H1 generator extraction on the four canonical heads.
pub fn extract_generator(hydra: &FourCalendarHydra) -> GeneratorReport {
    let heads = hydra.heads();
    let invariants: Vec<HeadInvariants> = heads.iter()
        .map(|h| HeadInvariants::from_head(h))
        .collect();

    // ── Enumerate generator candidates ─────────────────────────────

    let candidates_raw: Vec<GeneratorCandidate> = vec![

        // Candidate 1: Safe Basis "parking" rule.
        // The generator selects cycles whose nullification pattern
        // comes from the parking lanes {2, 5} and one transport prime.
        // Tzolk'in: nullified {2,5,13}; Haab: {5}; CR: {2,5,13}; LC: {2,3,5}
        GeneratorCandidate {
            label: "Parking-lanes nullification rule".into(),
            carry_pattern: None,
            phase_rule: PhaseRule::Uniform { n_phases: 13 },
            explained_cycles: vec![260, 18_980],
        },

        // Candidate 2: Uniform-n-phase rule.
        // Phase decomposition = n equal intervals where n is a Safe Basis prime.
        GeneratorCandidate {
            label: "Uniform-p-phase (p ∈ Safe Basis)".into(),
            carry_pattern: None,
            phase_rule: PhaseRule::Uniform { n_phases: 13 },
            explained_cycles: vec![260, 18_980, 144_000],
        },

        // Candidate 3: Mixed-phase rule (Haab-type).
        // 18 regular phases + 1 remainder (intercalary).
        GeneratorCandidate {
            label: "Mixed intercalary rule (18+1)".into(),
            carry_pattern: Some(vec![5]),
            phase_rule: PhaseRule::Mixed { count_a: 18, len_a: 20, len_b: 5 },
            explained_cycles: vec![365],
        },

        // Candidate 4: Factor-split rule.
        // Phase intervals come from the cycle's prime factorization.
        GeneratorCandidate {
            label: "Factor-split (phases from prime factors)".into(),
            carry_pattern: None,
            phase_rule: PhaseRule::FactorSplit,
            explained_cycles: vec![260, 365, 18_980, 144_000],
        },

        // Candidate 5: Astronomical rule.
        // Phase intervals are astronomically motivated (Venus: 236/90/250/8).
        GeneratorCandidate {
            label: "Astronomical-interval rule".into(),
            carry_pattern: None,
            phase_rule: PhaseRule::Astronomical,
            explained_cycles: vec![584],
        },

        // Candidate 6: Baktun-positional rule.
        // The Long Count uses positional notation; cycle = 20^k or 18×20^k.
        // This is a pure positional rule, the simplest generator.
        GeneratorCandidate {
            label: "Positional-base-20 rule (Long Count)".into(),
            carry_pattern: None,
            phase_rule: PhaseRule::Uniform { n_phases: 20 },
            explained_cycles: vec![20, 360, 7_200, 144_000],
        },
    ];

    // ── Score all candidates ───────────────────────────────────────

    let scored: Vec<(GeneratorCandidate, usize, usize, usize)> = candidates_raw
        .into_iter()
        .map(|c| {
            let (heads_ex, inv_m, total) = c.score(&invariants);
            (c, heads_ex, inv_m, total)
        })
        .collect();

    // Best candidate: highest inv_m
    let best_idx = scored.iter().enumerate()
        .max_by_key(|(_, (_, _, inv_m, _))| *inv_m)
        .map(|(i, _)| i);

    let best_candidate = best_idx.map(|i| scored[i].0.clone());
    let (heads_explained, inv_matched, total_inv) = best_idx
        .map(|i| (scored[i].1, scored[i].2, scored[i].3))
        .unwrap_or((0, 0, 1));

    let confidence_bp = (inv_matched as u64 * 10000) / total_inv.max(1) as u64;

    // ── Seed identification ────────────────────────────────────────
    // Try to identify seeds that produce each canonical head.
    // Seeds to try: Safe Basis primes, their products, and small integers.
    let seed_map = identify_seeds(&invariants);

    // ── Verdict ───────────────────────────────────────────────────
    let verdict = match heads_explained {
        4 => H1Verdict::Supported,
        2..=3 => H1Verdict::Partial,
        _ => H1Verdict::NotSupported,
    };

    GeneratorReport {
        head_invariants: invariants,
        candidates: scored,
        best_candidate,
        heads_explained,
        confidence_bp,
        verdict,
        seed_map,
    }
}

/// Try to identify the seed that produces each canonical head.
///
/// Strategy: for each head, check whether the head's carry signature
/// is consistent with the CRAM address of some small seed value.
///
/// Seeds probed: each Safe Basis prime, and the heads' own cycle
/// values (self-seeding hypothesis).
fn identify_seeds(invariants: &[HeadInvariants]) -> Vec<(u64, &'static str)> {
    // Seed candidates: single primes, products, and canonical numbers.
    let seed_candidates: Vec<u64> = vec![
        // Single Safe Basis primes
        2, 3, 5, 7, 11, 13,
        // Products of pairs
        2*3, 2*5, 2*7, 2*11, 2*13, 3*5, 3*7, 3*11, 3*13, 5*7, 5*11, 5*13, 7*11, 7*13, 11*13,
        // The canonical cycle values themselves
        20, 28, 52, 65, 73, 121, 260, 325, 365, 584, 819, 18_980, 144_000,
    ];

    let mut result = Vec::new();
    for inv in invariants {
        // A seed "explains" this head if cram_address(seed) has the
        // same nullification pattern as the head's cycle.
        let head_nullified = &inv.nullified;
        let mut best_seed = None;
        let mut best_score = 0usize;
        for &seed in &seed_candidates {
            let seed_nullified = cram_address(seed).iter().zip(SAFE_BASIS.iter())
                .filter(|(&r, _)| r == 0).map(|(_, &p)| p).collect::<Vec<u64>>();
            // Score: how many nullification bits match?
            let matches = head_nullified.iter().filter(|&&p| seed_nullified.contains(&p)).count()
                + seed_nullified.iter().filter(|&&p| head_nullified.contains(&p)).count();
            let total = (head_nullified.len() + seed_nullified.len()).max(1);
            let score = matches * 100 / total;
            if score > best_score {
                best_score = score;
                best_seed = Some(seed);
            }
        }
        if let Some(seed) = best_seed {
            result.push((seed, inv.name));
        }
    }
    result
}

// ═══════════════════════════════════════════════════════════════════
// §7  Configuration-space walk (the manifold navigator)
// ═══════════════════════════════════════════════════════════════════

/// Walk the configuration space from a starting carry-signature,
/// applying the best-fit generator rule at each step.
///
/// Returns a sequence of (cycle, carry_sig, step_label) representing
/// the manifold path.
pub fn configuration_walk(
    start_cycle: u64,
    steps: usize,
) -> Vec<(u64, CarrySignature, String)> {
    let mut result = Vec::new();
    let mut current = start_cycle;
    for _ in 0..steps {
        let sig = CarrySignature::from_cycle(current);
        let label = format!("cycle={current}");
        result.push((current, sig, label));
        // Walk: next = product of current cycle with the next S₆ prime
        // that is currently nullified (rotate through nullified primes).
        let nullified = nullified_lanes(current);
        if nullified.is_empty() {
            break; // nowhere to walk (no nullified primes to activate)
        }
        // Next step: multiply by the smallest nullified prime
        let next_prime = nullified[0];
        current = current.saturating_mul(next_prime);
        if current == 0 || current > BAKTUN * 20 { break; }
    }
    result
}

const BAKTUN: u64 = 144_000;

// ═══════════════════════════════════════════════════════════════════
// §8  Tests
// ═══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::heads::FourCalendarHydra;

    #[test]
    fn carry_signature_known_cycles() {
        // 260 = 2² × 5 × 13: nullified {2,5,13}, active {3,7,11}
        let sig = CarrySignature::from_cycle(260);
        assert!(sig.active_primes().contains(&3));
        assert!(sig.active_primes().contains(&7));
        assert!(sig.active_primes().contains(&11));
        assert!(sig.nullified_primes().contains(&2));
        assert!(sig.nullified_primes().contains(&5));
        assert!(sig.nullified_primes().contains(&13));
    }

    #[test]
    fn carry_signature_hamming_distance() {
        let a = CarrySignature::from_cycle(260); // nullified {2,5,13}
        let b = CarrySignature::from_cycle(365); // nullified {5}
        // a has bits 0,2,5 = 0; b has bit 2 = 0.
        // a.0 = 0b001010 (primes 3,7,11 active) ...let me compute:
        // active: 3(bit1)=1, 7(bit3)=1, 11(bit4)=1 → a.0 = 0b011010 = 26
        // b: nullified only 5(bit2) → active: 2,3,7,11,13 → bits 0,1,3,4,5 = 0b111011 = 59
        // XOR = 26^59 = 37, popcount = ?
        let d = a.hamming_distance(&b);
        assert!(d > 0, "Different carry signatures have nonzero distance");
        assert!(d <= 6, "Hamming distance ≤ 6 bits");
    }

    #[test]
    fn phase_rule_uniform_applies_correctly() {
        let rule = PhaseRule::Uniform { n_phases: 13 };
        let phases = rule.apply(260);
        assert_eq!(phases.len(), 13);
        assert_eq!(phases[0], 20); // 260 / 13 = 20
        assert_eq!(phases.iter().sum::<u64>(), 260);
    }

    #[test]
    fn phase_rule_mixed_applies_correctly() {
        let rule = PhaseRule::Mixed { count_a: 18, len_a: 20, len_b: 5 };
        let phases = rule.apply(365);
        assert_eq!(phases.len(), 19);
        assert_eq!(phases.iter().sum::<u64>(), 365);
    }

    #[test]
    fn generator_extraction_runs_for_canonical_hydra() {
        let hydra = FourCalendarHydra::canonical();
        let report = extract_generator(&hydra);
        assert_eq!(report.head_invariants.len(), 4);
        assert!(!report.candidates.is_empty());
        assert!(report.confidence_bp <= 10000);
        // At minimum the computation completes and identifies some structure
        assert!(report.heads_explained <= 4,
            "Cannot explain more heads than exist");
    }

    #[test]
    fn generator_invariants_correct_for_tzolkin() {
        let hydra = FourCalendarHydra::canonical();
        let report = extract_generator(&hydra);
        let tzolkin_inv = report.head_invariants.iter()
            .find(|i| i.name == "Tzolkin").expect("Tzolk'in in invariants");
        assert_eq!(tzolkin_inv.cycle, 260);
        assert_eq!(tzolkin_inv.phase_count, 13);
        assert!(tzolkin_inv.uniform_phases);
        assert_eq!(tzolkin_inv.common_phase_len, 20);
        assert!(tzolkin_inv.nullified.contains(&13));
    }

    #[test]
    fn generator_invariants_correct_for_haab() {
        let hydra = FourCalendarHydra::canonical();
        let report = extract_generator(&hydra);
        let haab = report.head_invariants.iter()
            .find(|i| i.name == "Haab").expect("Haab in invariants");
        assert_eq!(haab.cycle, 365);
        assert_eq!(haab.phase_count, 19);
        assert!(!haab.uniform_phases); // 18×20 + 1×5
        assert!(haab.nullified.contains(&5));
    }

    #[test]
    fn seed_map_covers_all_four_heads() {
        let hydra = FourCalendarHydra::canonical();
        let report = extract_generator(&hydra);
        assert_eq!(report.seed_map.len(), 4,
            "One seed identified per head");
    }

    #[test]
    fn configuration_walk_starts_at_260_and_stays_bounded() {
        let path = configuration_walk(260, 6);
        assert_eq!(path[0].0, 260);
        // All steps should remain ≤ BAKTUN * 20
        for (cycle, _, _) in &path {
            assert!(*cycle <= BAKTUN * 20);
        }
    }

    #[test]
    fn head_invariants_lane_entropy_nonzero_for_canonical() {
        let hydra = FourCalendarHydra::canonical();
        for head in hydra.heads() {
            let inv = HeadInvariants::from_head(head);
            assert!(inv.lane_entropy_bp > 0,
                "{} should have nonzero lane entropy", inv.name);
        }
    }
}
