//! # DKAM Tier Mapping (B-8 / Tier 3)
//!
//! Mechanizes vault `The Dresden Codex.md` §DKAM Window section and
//! T-UDP-BOUNDARY Axiom A5: the four-tier failure structure for primes
//! against the Fibonacci-index function and the DKAM admissibility
//! condition `deg(F) < ρ(B)`.
//!
//! ## Tier classification
//!
//! | Tier | Name | Condition | Examples |
//! |---|---|---|---|
//! | 0 | Face13 | post-boundary collapse, prime = F(7) | 13 |
//! | 1 | Turbulent | ℓ > 11, outside the DKAM clean window | 17, 19, 23, 31, 73 |
//! | 2 | Boundary | ρ = deg, transition point | (basis-dependent) |
//! | 3 | Exact | S_R + DKAM exact regime, inside clean window | 5, 7, 11; also 3 |
//!
//! Note: this is **distinct from** `dccms_atlas::dkam_filter` (existing
//! module) which implements an admissibility filter over a prime basis.
//! `dkam_tier` is the per-prime classification side of DKAM.
//!
//! Source: vault `The Dresden Codex.md` lines 540-625.

#![allow(dead_code)]

/// Tier classification of a prime against the DKAM boundary structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tier {
    /// Tier 0 — Face 13. Post-boundary collapse. The single prime 13,
    /// which is `F(7)` (the Fibonacci index of the 7th composite).
    Face13,
    /// Tier 1 — Turbulent. Outside the DKAM clean window: ℓ > 11.
    Turbulent,
    /// Tier 2 — Boundary. Transition point where `ρ = deg`.
    Boundary,
    /// Tier 3 — Exact. Inside the clean window: the S_R primes {5, 7, 11}
    /// plus the stability-floor prime 3.
    Exact,
}

/// Classify a prime by its DKAM tier.
///
/// Per vault Axiom A5:
/// - 13 → Face13 (the unique post-boundary prime via F(7))
/// - {5, 7, 11} → Exact (the Ramanujan / S_R set)
/// - 3 → Exact (stability-floor prime; ρ=3 > deg=2)
/// - 2 → Boundary (parking-lane prime, ρ = deg at the smallest scale)
/// - ℓ > 13 → Turbulent
pub fn tier_of(prime: u64) -> Tier {
    match prime {
        13 => Tier::Face13,
        5 | 7 | 11 | 3 => Tier::Exact,
        2 => Tier::Boundary,
        p if p > 13 => Tier::Turbulent,
        _ => Tier::Turbulent,  // catch-all for unexpected (non-prime) inputs
    }
}

/// Fibonacci-index function `F(k)`: the k-th prime in the order
/// induced by Fibonacci indices. Per vault Axiom A5: F(4) = 3, F(7) = 13.
///
/// The Maya CRT framework uses this enumeration to identify the
/// stability-floor prime (F(4) = 3) and the boundary prime (F(7) = 13).
/// Other indices are mechanically computable but not load-bearing for
/// the four-tier classification.
///
/// Returns Some for k in {1..10} (the documented range); None otherwise.
pub fn fibonacci_index_prime(k: u64) -> Option<u64> {
    match k {
        1 => Some(2),
        2 => Some(3),
        3 => Some(5),
        4 => Some(3),   // vault A5: F(4) = 3, the stability-floor prime
        5 => Some(7),
        6 => Some(11),
        7 => Some(13),  // vault A5: F(7) = 13, the boundary prime
        8 => Some(17),
        9 => Some(19),
        10 => Some(23),
        _ => None,
    }
}

/// DKAM clean-window primes for stability index `rho`.
///
/// Returns the primes admissible under `deg(F) < ρ(B)` at the given ρ.
/// Per vault §DKAM Window:
/// - ρ = 2 → parking-lane only: {2}
/// - ρ = 3 → stability floor: {2, 3}
/// - ρ = 4 → adds boundary: {2, 3, 13}
/// - ρ = 5 → full accessible S_R: {2, 3, 5, 7, 11, 13}
/// - ρ = 7 → 819's three-tier product: {2, 3, 5, 7, 11, 13}
/// - ρ = 9 → Long Count regime: {2, 3, 5, 7, 11, 13, 17, 19}
pub fn dkam_window(rho: u64) -> Vec<u64> {
    match rho {
        2 => vec![2],
        3 => vec![2, 3],
        4 => vec![2, 3, 13],
        5 | 6 | 7 => vec![2, 3, 5, 7, 11, 13],
        8 | 9 => vec![2, 3, 5, 7, 11, 13, 17, 19],
        _ => vec![2, 3, 5, 7, 11, 13, 17, 19],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier_of_13_is_face13() {
        assert_eq!(tier_of(13), Tier::Face13);
    }

    #[test]
    fn tier_of_s_r_is_exact() {
        for &p in &[5u64, 7, 11] {
            assert_eq!(tier_of(p), Tier::Exact, "prime {} should be Exact tier", p);
        }
    }

    #[test]
    fn tier_of_stability_floor_3_is_exact() {
        assert_eq!(tier_of(3), Tier::Exact);
    }

    #[test]
    fn tier_of_2_is_boundary() {
        assert_eq!(tier_of(2), Tier::Boundary);
    }

    #[test]
    fn tier_of_larger_primes_is_turbulent() {
        for &p in &[17u64, 19, 23, 29, 31, 73, 89, 101] {
            assert_eq!(tier_of(p), Tier::Turbulent, "prime {} should be Turbulent", p);
        }
    }

    #[test]
    fn fibonacci_index_prime_load_bearing_values() {
        // Vault Axiom A5: F(4) = 3 (stability-floor), F(7) = 13 (boundary).
        assert_eq!(fibonacci_index_prime(4), Some(3));
        assert_eq!(fibonacci_index_prime(7), Some(13));
        // F(out_of_range) = None.
        assert_eq!(fibonacci_index_prime(100), None);
    }

    #[test]
    fn dkam_window_scales_with_rho() {
        // At ρ=2: only parking-lane.
        assert_eq!(dkam_window(2), vec![2u64]);
        // At ρ=3: stability floor added.
        assert!(dkam_window(3).contains(&3));
        // At ρ=5: full Safe Basis available.
        let w = dkam_window(5);
        for &p in &[2u64, 3, 5, 7, 11, 13] {
            assert!(w.contains(&p), "Safe Basis prime {} should be in ρ=5 window", p);
        }
        // At ρ=9: extended Safe Basis (with 17, 19).
        let w9 = dkam_window(9);
        assert!(w9.contains(&17));
        assert!(w9.contains(&19));
    }
}
