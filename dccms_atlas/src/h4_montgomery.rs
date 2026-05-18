//! # H4 Instrument 6 — Montgomery Quotient Shadow
//!
//! The sixth H4 instrument: the Montgomery Quotient Shadow.
//!
//! ## Background
//!
//! Montgomery multiplication computes `x × y × R⁻¹ mod N` where `R = 2^k`.
//! For the CRAM substrate with Transport Core {3,7,11,13}, the
//! "Montgomery quotient" for a given day count `x` at primes {11,13} is:
//!
//! ```text
//!     Q₁₁(x) = floor(x × 11⁻¹ mod 13)   (the 11-shadow on lane 13)
//!     Q₁₃(x) = floor(x × 13⁻¹ mod 11)   (the 13-shadow on lane 11)
//! ```
//!
//! These are "byproduct signatures" — they appear as deterministic outputs
//! of the CRAM multiplication algorithm, not primary inputs. For the
//! goddess-section pages, if the page intervals encode CRAM operations,
//! the byproduct pairs (Q₁₁, Q₁₃) should cluster deterministically.
//!
//! ## Implementation
//!
//! For each page boundary T_k in the goddess section:
//! - Compute `r₁₁ = T_k mod 11` (lane 11 residue)
//! - Compute `r₁₃ = T_k mod 13` (lane 13 residue)
//! - Compute `q₁₁ = (T_k × inv11_mod13) mod 13`   (Montgomery quotient)
//! - Compute `q₁₃ = (T_k × inv13_mod11) mod 11`   (Montgomery quotient)
//! - Record the pair (q₁₁, q₁₃)
//!
//! The "deterministic byproduct signature" for a valid CRAM encoding:
//! the sequence of (q₁₁, q₁₃) pairs should follow a fixed pattern
//! determined by the substrate's inner product structure.
//!
//! ## Null model
//!
//! For random day counts, (q₁₁, q₁₃) is approximately uniform over
//! [0,13) × [0,11). The goddess section's pairs should show clustering
//! if the intervals encode CRAM operations.

#![allow(dead_code)]

use crate::h4_non_visual::GODDESS_SECTION_INTERVALS;
use crate::h3_mi::entropy_nbp;

// ═══════════════════════════════════════════════════════════════════
// §1  Modular arithmetic
// ═══════════════════════════════════════════════════════════════════

/// Extended GCD: returns (gcd, x, y) such that a*x + b*y = gcd.
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 { return (a, 1, 0); }
    let (g, x1, y1) = extended_gcd(b, a % b);
    (g, y1, x1 - (a / b) * y1)
}

/// Modular inverse of a mod m (assuming gcd(a,m) = 1).
pub fn mod_inverse(a: i64, m: i64) -> i64 {
    let (_, x, _) = extended_gcd(a % m, m);
    ((x % m) + m) % m
}

/// 11⁻¹ mod 13.
pub const INV11_MOD13: u64 = 6; // 11 × 6 = 66 = 5×13 + 1 ✓

/// 13⁻¹ mod 11.
pub const INV13_MOD11: u64 = 6; // 13 × 6 = 78 = 7×11 + 1 ✓

// ═══════════════════════════════════════════════════════════════════
// §2  Montgomery quotient shadow computation
// ═══════════════════════════════════════════════════════════════════

/// Compute the (r₁₁, r₁₃, q₁₁, q₁₃) quad for a day count.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MontgomeryQuad {
    /// Residue mod 11.
    pub r11: u64,
    /// Residue mod 13.
    pub r13: u64,
    /// Montgomery quotient on lane 13: (x × 11⁻¹) mod 13.
    pub q11: u64,
    /// Montgomery quotient on lane 11: (x × 13⁻¹) mod 11.
    pub q13: u64,
}

impl MontgomeryQuad {
    pub fn from_day(x: u64) -> Self {
        let r11 = x % 11;
        let r13 = x % 13;
        let q11 = (x % 13 * INV11_MOD13) % 13;
        let q13 = (x % 11 * INV13_MOD11) % 11;
        MontgomeryQuad { r11, r13, q11, q13 }
    }

    /// The (q₁₁, q₁₃) byproduct pair.
    pub fn byproduct_pair(&self) -> (u64, u64) {
        (self.q11, self.q13)
    }
}

// ═══════════════════════════════════════════════════════════════════
// §3  Goddess-section Montgomery analysis
// ═══════════════════════════════════════════════════════════════════

/// Montgomery quad for each goddess-section page boundary.
pub fn goddess_section_quads() -> Vec<(u64, MontgomeryQuad)> {
    let mut total = 0u64;
    GODDESS_SECTION_INTERVALS.iter().map(|&interval| {
        total += interval;
        (total, MontgomeryQuad::from_day(total))
    }).collect()
}

/// Count distinct byproduct pairs in the goddess section.
pub fn distinct_byproduct_pairs() -> usize {
    let quads = goddess_section_quads();
    let mut seen = std::collections::HashSet::new();
    for (_, q) in &quads {
        seen.insert(q.byproduct_pair());
    }
    seen.len()
}

/// Entropy of the byproduct-pair distribution over goddess-section pages.
/// Returns entropy in nat-basis-points.
pub fn byproduct_entropy_nbp() -> i64 {
    let quads = goddess_section_quads();
    let mut counts: std::collections::HashMap<(u64, u64), u64> = std::collections::HashMap::new();
    for (_, q) in &quads {
        *counts.entry(q.byproduct_pair()).or_insert(0) += 1;
    }
    let count_vec: Vec<u64> = counts.values().copied().collect();
    entropy_nbp(&count_vec)
}

/// Maximum entropy for n_pages pages over 11×13 = 143 possible pairs.
pub fn max_byproduct_entropy_nbp(n_pages: usize) -> i64 {
    // Max entropy: uniform over min(n_pages, 143) bins
    let n_bins = n_pages.min(143);
    entropy_nbp(&vec![1u64; n_bins])
}

// ═══════════════════════════════════════════════════════════════════
// §4  Deterministic clustering test
// ═══════════════════════════════════════════════════════════════════

/// Whether the byproduct sequence shows clustering:
/// distinct pairs < 50% of possible entropy.
#[derive(Clone, Debug)]
pub struct MontgomeryReport {
    /// Per-page (day, quad) tuples.
    pub page_quads: Vec<(u64, MontgomeryQuad)>,
    /// Count of distinct byproduct pairs.
    pub distinct_pairs: usize,
    /// Entropy of pair distribution (nbp).
    pub pair_entropy_nbp: i64,
    /// Max possible entropy (nbp).
    pub max_entropy_nbp: i64,
    /// Entropy reduction vs maximum (= max - observed, in nbp).
    pub entropy_reduction_nbp: i64,
    /// Whether the reduction exceeds 3000 nbp (30% of max).
    pub clustered: bool,
    /// The (q₁₁, q₁₃) sequence across pages.
    pub byproduct_sequence: Vec<(u64, u64)>,
    /// Whether consecutive pairs share a common q₁₁ value (line pattern).
    pub q11_line_pattern: bool,
}

pub fn compute_montgomery_report() -> MontgomeryReport {
    let quads = goddess_section_quads();
    let n = quads.len();
    let distinct = distinct_byproduct_pairs();
    let entropy = byproduct_entropy_nbp();
    let max_entropy = max_byproduct_entropy_nbp(n);
    let reduction = max_entropy - entropy;
    let clustered = reduction >= 3_000;

    let byproduct_seq: Vec<(u64, u64)> = quads.iter().map(|(_, q)| q.byproduct_pair()).collect();

    // Check for q₁₁ line pattern: if q₁₁ values form an AP
    let q11_vals: Vec<u64> = byproduct_seq.iter().map(|&(q11, _)| q11).collect();
    let q11_line = if q11_vals.len() < 2 {
        false
    } else {
        let d = q11_vals[1] as i64 - q11_vals[0] as i64;
        q11_vals.windows(2).all(|w| (w[1] as i64 - w[0] as i64) == d)
    };

    MontgomeryReport {
        page_quads: quads,
        distinct_pairs: distinct,
        pair_entropy_nbp: entropy,
        max_entropy_nbp: max_entropy,
        entropy_reduction_nbp: reduction,
        clustered,
        byproduct_sequence: byproduct_seq,
        q11_line_pattern: q11_line,
    }
}

// ═══════════════════════════════════════════════════════════════════
// §5  Full period Montgomery sweep
// ═══════════════════════════════════════════════════════════════════

/// Run the Montgomery shadow analysis over all Dresden T7 periods.
pub fn period_montgomery_sweep() -> Vec<(&'static str, u64, MontgomeryQuad)> {
    let periods: &[(&'static str, u64)] = &[
        ("Tzolk'in", 260),
        ("Haab", 365),
        ("Venus synodic", 584),
        ("Council-819", 819),
        ("Eclipse-148", 148),
        ("Eclipse-177", 177),
        ("Eclipse-11960", 11_960),
        ("Calendar Round", 18_980),
        ("Baktun", 144_000),
    ];
    periods.iter().map(|&(name, days)| {
        (name, days, MontgomeryQuad::from_day(days))
    }).collect()
}

// ═══════════════════════════════════════════════════════════════════
// §6  Tests
// ═══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_inverse_11_mod_13() {
        // 11 × 6 = 66 = 5×13 + 1 → 11⁻¹ mod 13 = 6
        assert_eq!(INV11_MOD13, 6);
        assert_eq!((11 * INV11_MOD13) % 13, 1);
    }

    #[test]
    fn mod_inverse_13_mod_11() {
        // 13 × 6 = 78 = 7×11 + 1 → 13⁻¹ mod 11 = 6
        assert_eq!(INV13_MOD11, 6);
        assert_eq!((13 * INV13_MOD11) % 11, 1);
    }

    #[test]
    fn extended_gcd_known_values() {
        let (g, x, y) = extended_gcd(11, 13);
        assert_eq!(g, 1); // gcd(11,13) = 1 (coprime)
        let result = 11 * x + 13 * y;
        assert_eq!(result, 1);
    }

    #[test]
    fn montgomery_quad_day_zero() {
        let q = MontgomeryQuad::from_day(0);
        assert_eq!(q.r11, 0);
        assert_eq!(q.r13, 0);
        assert_eq!(q.q11, 0); // 0 × anything mod 13 = 0
        assert_eq!(q.q13, 0);
    }

    #[test]
    fn montgomery_quad_day_148() {
        let q = MontgomeryQuad::from_day(148);
        assert_eq!(q.r11, 148 % 11); // 5
        assert_eq!(q.r13, 148 % 13); // 5
        // q11 = (148 % 13 * 6) % 13 = (5 * 6) % 13 = 30 % 13 = 4
        assert_eq!(q.q11, (148 % 13 * INV11_MOD13) % 13);
        assert_eq!(q.q13, (148 % 11 * INV13_MOD11) % 11);
    }

    #[test]
    fn goddess_section_quads_correct_count() {
        let quads = goddess_section_quads();
        assert_eq!(quads.len(), 9); // 9 page boundaries
        // Last total should be the goddess section total
        assert_eq!(quads.last().unwrap().0, 1_448);
    }

    #[test]
    fn goddess_section_first_boundary_at_148() {
        let quads = goddess_section_quads();
        assert_eq!(quads[0].0, 148);
        // Verify the quad at 148: r11 = 148%11 = 5, r13 = 148%13 = 5
        assert_eq!(quads[0].1.r11, 5);
        assert_eq!(quads[0].1.r13, 5);
    }

    #[test]
    fn distinct_pairs_at_most_n_pages() {
        let n = goddess_section_quads().len();
        assert!(distinct_byproduct_pairs() <= n);
        assert!(distinct_byproduct_pairs() >= 1);
    }

    #[test]
    fn montgomery_report_runs() {
        let report = compute_montgomery_report();
        assert_eq!(report.page_quads.len(), 9);
        assert!(report.pair_entropy_nbp >= 0);
        assert!(report.max_entropy_nbp >= report.pair_entropy_nbp);
        assert!(report.entropy_reduction_nbp >= 0);
    }

    #[test]
    fn period_sweep_known_values() {
        let sweep = period_montgomery_sweep();
        // Tzolk'in: 260 mod 11 = 7, 260 mod 13 = 0
        let tzolkin = sweep.iter().find(|(n,_,_)| *n == "Tzolk'in").unwrap();
        assert_eq!(tzolkin.2.r11, 260 % 11); // 7
        assert_eq!(tzolkin.2.r13, 260 % 13); // 0
        // Haab: 365 mod 11 = 2, 365 mod 13 = 1
        let haab = sweep.iter().find(|(n,_,_)| *n == "Haab").unwrap();
        assert_eq!(haab.2.r11, 365 % 11);  // 2
        assert_eq!(haab.2.r13, 365 % 13);  // 1
    }

    #[test]
    fn tzolkin_r13_is_zero() {
        // 260 = 20 × 13 → 260 mod 13 = 0 → lane 13 is nullified by Tzolk'in
        let q = MontgomeryQuad::from_day(260);
        assert_eq!(q.r13, 0, "Tzolk'in nullifies lane 13");
    }

    #[test]
    fn council_819_r11_r13_structure() {
        // 819 = 3² × 7 × 13 → r13 = 0; 819 mod 11 = 4
        let q = MontgomeryQuad::from_day(819);
        assert_eq!(q.r13, 0, "819 nullifies lane 13");
        assert_eq!(q.r11, 819 % 11); // 4
    }
}
