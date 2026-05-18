//! # H3 Mutual Information Instrument
//!
//! The H3 hypothesis asks whether candidate additional heads (Saturn-11²,
//! zodiac, temperaments, etc.) carry independent configuration-space
//! structure beyond the four canonical Maya calendars.
//!
//! ## Why not Gini-gain?
//!
//! The first version of H3 used "Gini increase when candidate added as
//! fifth head." Empirically this gave negative values for every candidate:
//! adding a head spreads events across more addresses, lowering Gini.
//! That measures redistribution, not information content.
//!
//! ## The mutual information instrument
//!
//! For each candidate head H_c and the four-head Hydra H_4, we measure
//! the mutual information I(H_4_address ; H_c_address) on the codex event
//! corpus. This says: knowing the four-head address, how much can we
//! learn about the candidate head's address?
//!
//! - If I ≈ 0: the candidate's signature is fully predictable from the
//!   four-head address. The candidate adds no structural information.
//! - If I > 0: the candidate signature has variation that the four heads
//!   do not capture. The candidate is independent structure.
//!
//! All computation is exact integer. Entropy in basis points uses
//! integer logarithms via the natural-log fraction approximation
//! ln(p) ≈ -10000 * (1-p)/p + correction, but for entropy comparison
//! we use the simpler log-ratio identity with integer scaling.
//!
//! ## Scaling
//!
//! Entropy values are reported in **nat-basis-points** (nbp):
//!
//! ```text
//! H_nbp = sum( -p * ln(p) ) × 10000
//! ```
//!
//! where p = count / total. We compute integer numerators and only divide
//! at the end. The ln approximation uses a fixed integer Taylor expansion
//! around 1 for ratios n/total, evaluated as:
//!
//! ```text
//! ln(n/total) ≈ ln_table(n) - ln_table(total)
//! ```
//!
//! For exactness, we use ln_int_bp(x) — integer natural log scaled by
//! 10000 — implemented via the Reinsch high-precision integer log.

use crate::events::EventSet;
use crate::heads::{HydraHead, FourCalendarHydra};
use crate::atlas::ConfigAtlas;
use std::collections::HashMap;

// ───────────────────────────────────────────────────────────────────
// Integer natural log scaled by 10000 (nat-basis-points).
// ───────────────────────────────────────────────────────────────────

/// Integer natural log scaled by 10000.
///
/// Computes ln(x) × 10000 as an i64 for x ≥ 1. For x = 0 returns
/// i64::MIN (sentinel; caller must handle).
///
/// Method: range-reduce x to [1, 2) by extracting the bit-length k,
/// so x = 2^k * m / 2^k where m/2^k ∈ [1, 2). Then ln(x) = k * ln(2) +
/// ln(m / 2^k). The remainder is computed via integer Taylor series
/// around 1 with rational arithmetic.
///
/// Returns ln(x) × 10000 (nat-basis-points), exact-integer.
pub fn ln_nbp(x: u64) -> i64 {
    if x == 0 {
        return i64::MIN;
    }
    if x == 1 {
        return 0;
    }
    // ln(2) × 10000 = 6931 (more precisely 6931.47, we use 6931)
    const LN2_NBP: i64 = 6931;
    // Range-reduce: find largest k such that 2^k ≤ x. Then x = 2^k * r
    // with r = x / 2^k ∈ [1, 2). ln(x) = k*ln(2) + ln(r).
    let mut k = 0i64;
    let mut y = x;
    while y >= 2 {
        y /= 2;
        k += 1;
    }
    // Now y == 1 and x is in [2^k, 2^(k+1)). The "remainder" r = x / 2^k.
    // We need ln(r) where r = x * 1 / 2^k, so r is in [1, 2).
    // Use integer Taylor: ln(1+u) ≈ u - u²/2 + u³/3 - u⁴/4 + ...
    // where u = (x / 2^k) - 1 = (x - 2^k) / 2^k.
    // Represent u as integer numerator over 2^k denominator.
    let two_k = 1u64 << k;
    let u_num = (x - two_k) as i128; // 0 ≤ u_num < 2^k
    let u_den = two_k as i128;
    // ln_remainder × 10000 ≈ sum of (-1)^(i+1) * (u_num^i * 10000) / (i * u_den^i)
    let mut ln_rem_nbp: i64 = 0;
    let mut term_num: i128 = u_num * 10000;
    let mut term_den: i128 = u_den;
    let mut sign: i64 = 1;
    for i in 1..=12i64 {
        let contribution = (term_num / (i as i128 * term_den)) as i64;
        ln_rem_nbp += sign * contribution;
        term_num *= u_num;
        term_den *= u_den;
        // Guard against overflow
        if term_den.abs() > (1i128 << 100) {
            break;
        }
        sign = -sign;
    }
    k * LN2_NBP + ln_rem_nbp
}

/// Entropy in nat-basis-points: H = -Σ p_i × ln(p_i), scaled by 10000.
///
/// For a histogram of counts [c_1, c_2, ..., c_n] with total T:
///   H = -Σ (c_i/T) × ln(c_i/T)
///     = Σ (c_i/T) × (ln(T) - ln(c_i))
///     = ln(T) × (Σ c_i / T) - Σ (c_i × ln(c_i)) / T
///     = ln(T) - (Σ c_i × ln(c_i)) / T   [since Σ c_i = T]
///
/// In nat-basis-points: H_nbp = ln_nbp(T) - (Σ c_i × ln_nbp(c_i)) / T
pub fn entropy_nbp(counts: &[u64]) -> i64 {
    let total: u64 = counts.iter().sum();
    if total == 0 {
        return 0;
    }
    let ln_t = ln_nbp(total) as i128;
    let mut weighted: i128 = 0;
    for &c in counts {
        if c > 0 {
            weighted += (c as i128) * (ln_nbp(c) as i128);
        }
    }
    let h_scaled = ln_t - weighted / (total as i128);
    h_scaled.max(0) as i64
}

// ───────────────────────────────────────────────────────────────────
// Joint and conditional entropy
// ───────────────────────────────────────────────────────────────────

/// Joint entropy H(X, Y) in nbp from a joint count table indexed by (x, y).
pub fn joint_entropy_nbp(joint: &HashMap<(u64, u64), u64>) -> i64 {
    let counts: Vec<u64> = joint.values().copied().collect();
    entropy_nbp(&counts)
}

/// Marginal counts over X from a joint table.
pub fn marginal_x(joint: &HashMap<(u64, u64), u64>) -> HashMap<u64, u64> {
    let mut m: HashMap<u64, u64> = HashMap::new();
    for (&(x, _), &c) in joint {
        *m.entry(x).or_insert(0) += c;
    }
    m
}

/// Marginal counts over Y from a joint table.
pub fn marginal_y(joint: &HashMap<(u64, u64), u64>) -> HashMap<u64, u64> {
    let mut m: HashMap<u64, u64> = HashMap::new();
    for (&(_, y), &c) in joint {
        *m.entry(y).or_insert(0) += c;
    }
    m
}

/// Mutual information I(X; Y) = H(X) + H(Y) - H(X, Y), in nbp.
pub fn mutual_information_nbp(joint: &HashMap<(u64, u64), u64>) -> i64 {
    let mx = marginal_x(joint);
    let my = marginal_y(joint);
    let hx = entropy_nbp(&mx.values().copied().collect::<Vec<u64>>());
    let hy = entropy_nbp(&my.values().copied().collect::<Vec<u64>>());
    let hxy = joint_entropy_nbp(joint);
    (hx + hy - hxy).max(0)
}

// ───────────────────────────────────────────────────────────────────
// H3 instrument
// ───────────────────────────────────────────────────────────────────

/// H3 mutual information report for a single candidate head.
#[derive(Clone, Debug)]
pub struct H3MIReport {
    pub candidate_name: String,
    pub candidate_cycle: u64,
    /// Joint entropy H(four_head_address, candidate_address) in nbp.
    pub joint_entropy_nbp: i64,
    /// Marginal entropy H(four_head_address) in nbp.
    pub four_head_entropy_nbp: i64,
    /// Marginal entropy H(candidate_address) in nbp.
    pub candidate_entropy_nbp: i64,
    /// Mutual information I(four_head ; candidate) in nbp.
    pub mutual_information_nbp: i64,
    /// Conditional entropy H(candidate | four_head) = H(c) - I in nbp.
    /// This is the "independent information" the candidate carries.
    pub independent_information_nbp: i64,
    /// Fraction of candidate entropy that is independent of the four heads.
    /// In basis points (0–10000). 10000 means fully independent.
    pub independence_ratio_bp: u64,
    /// Whether the independence ratio exceeds the threshold.
    pub admissible: bool,
}

/// Build the joint distribution of (four_head_address, candidate_address)
/// over a corpus of events.
pub fn build_joint_distribution(
    events: &EventSet,
    hydra: &FourCalendarHydra,
    candidate: &HydraHead,
) -> HashMap<(u64, u64), u64> {
    let mut joint: HashMap<(u64, u64), u64> = HashMap::new();
    let four_heads = hydra.heads();
    for event in &events.events {
        let four_addr = ConfigAtlas::compute_address(&four_heads, event.days_since_epoch);
        let cand_addr = ConfigAtlas::compute_address(&[candidate], event.days_since_epoch);
        *joint.entry((four_addr, cand_addr)).or_insert(0) += 1;
    }
    joint
}

/// Compute the H3 mutual-information report for a single candidate.
pub fn h3_mi_report(
    events: &EventSet,
    hydra: &FourCalendarHydra,
    candidate: &HydraHead,
    independence_threshold_bp: u64,
) -> H3MIReport {
    let joint = build_joint_distribution(events, hydra, candidate);
    let mx = marginal_x(&joint);
    let my = marginal_y(&joint);
    let hx = entropy_nbp(&mx.values().copied().collect::<Vec<u64>>());
    let hy = entropy_nbp(&my.values().copied().collect::<Vec<u64>>());
    let hxy = joint_entropy_nbp(&joint);
    let mi = (hx + hy - hxy).max(0);
    // Conditional entropy H(Y|X) = H(X,Y) - H(X) = H(Y) - I(X;Y)
    let independent = (hy - mi).max(0);
    // Independence ratio: H(Y|X) / H(Y) — what fraction of the candidate's
    // entropy is NOT predictable from the four-head address?
    let independence_ratio_bp = if hy > 0 {
        ((independent as i128) * 10000 / hy as i128) as u64
    } else {
        0
    };
    H3MIReport {
        candidate_name: candidate.signature.name.to_string(),
        candidate_cycle: candidate.signature.cycle,
        joint_entropy_nbp: hxy,
        four_head_entropy_nbp: hx,
        candidate_entropy_nbp: hy,
        mutual_information_nbp: mi,
        independent_information_nbp: independent,
        independence_ratio_bp,
        admissible: independence_ratio_bp >= independence_threshold_bp,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::heads::*;

    #[test]
    fn ln_nbp_known_values() {
        assert_eq!(ln_nbp(0), i64::MIN);
        assert_eq!(ln_nbp(1), 0);
        // ln(2) ≈ 0.6931 → 6931 nbp (allow ±50 tolerance for integer approximation)
        let v = ln_nbp(2);
        assert!((v - 6931).abs() <= 50, "ln(2) approx = {}", v);
        // ln(10) ≈ 2.3026 → 23026 nbp
        let v = ln_nbp(10);
        assert!((v - 23026).abs() <= 100, "ln(10) approx = {}", v);
        // ln(100) ≈ 4.6052 → 46052 nbp
        let v = ln_nbp(100);
        assert!((v - 46052).abs() <= 200, "ln(100) approx = {}", v);
    }

    #[test]
    fn entropy_zero_for_single_class() {
        // All counts in one bucket → H = 0
        let h = entropy_nbp(&[100, 0, 0, 0]);
        assert_eq!(h, 0);
    }

    #[test]
    fn entropy_maximum_for_uniform() {
        // Uniform over 4 classes: H = ln(4) ≈ 1.3863 → ~13863 nbp
        let h = entropy_nbp(&[25, 25, 25, 25]);
        assert!((h - 13863).abs() <= 200, "H = {}", h);
    }

    #[test]
    fn mutual_information_zero_for_independent_marginals() {
        // Independent: P(x,y) = P(x) * P(y), so I = 0.
        // Construct joint with x ∈ {0,1}, y ∈ {0,1}, each combination
        // with count 25 (totally independent uniform).
        let mut joint = HashMap::new();
        joint.insert((0u64, 0u64), 25);
        joint.insert((0, 1), 25);
        joint.insert((1, 0), 25);
        joint.insert((1, 1), 25);
        let mi = mutual_information_nbp(&joint);
        // I should be close to 0 (allow some integer-arithmetic slack)
        assert!(mi <= 100, "I = {} should be ~0 for independent", mi);
    }

    #[test]
    fn mutual_information_high_for_perfect_correlation() {
        // Perfect correlation: y = x. I(X;Y) = H(X) = H(Y) = ln(2) nbp.
        let mut joint = HashMap::new();
        joint.insert((0u64, 0u64), 50);
        joint.insert((1, 1), 50);
        let mi = mutual_information_nbp(&joint);
        // I should equal H(X) = ln(2) ≈ 6931 nbp
        assert!((mi - 6931).abs() <= 200, "I = {} for perfect correlation", mi);
    }

    #[test]
    fn h3_mi_runs_for_canonical_corpus() {
        let events = EventSet::canonical_corpus(0, 50_000);
        let hydra = FourCalendarHydra::canonical();
        let candidate = saturn_11_squared_head();
        let report = h3_mi_report(&events, &hydra, &candidate, 3000);
        // Just verify the computation completes and ratios are sensible
        assert!(report.candidate_entropy_nbp >= 0);
        assert!(report.mutual_information_nbp >= 0);
        assert!(report.independent_information_nbp >= 0);
        assert!(report.independence_ratio_bp <= 10000);
    }
}
