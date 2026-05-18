//! # BasisLift — arbitrary-precise residue lift (NODE-VT07)
//!
//! Generic residue map `u64 → [u64; K]` parameterized by `const K: usize`.
//! This is the **load-bearing scalability node** of the H4 visual
//! transducer: adding primes to the basis extends the addressable range
//! *without altering the first 6 lanes* and without introducing
//! floating-point precision.
//!
//! ## The arbitrary-precise scalability claim
//!
//! For any `x: u64` and any basis `b` extending `SAFE_BASIS` (i.e.,
//! `b[0..6] == SAFE_BASIS`):
//!
//! ```text
//!   lift::<K>(x, &b)[0..6]  ==  cram_address(x)        for every K ≥ 6
//!   lift::<K>(x, &b)[i]     ==  x % b[i]               for every i
//! ```
//!
//! The first identity is the *preservation* claim — extension never
//! changes what's already there. The second is the *refinement* claim —
//! every new lane is a true residue modulo a true prime.
//!
//! Adding primes scales the addressable range geometrically: at `K=6`,
//! `M_SAFE = 30_030`; at `K=7` (+17), `≈ 510_510`; at `K=8` (+19),
//! `≈ 9_699_690`. The lift is the canonical CRT extension — no rounding,
//! no precision budget shared with anything else.

#![allow(dead_code)]

/// Lift an integer to a `K`-lane residue address.
///
/// Every lane is a true `u64 % prime` residue. The function is
/// branch-free over the basis size.
pub fn lift<const K: usize>(x: u64, basis: &[u64; K]) -> [u64; K] {
    let mut addr = [0u64; K];
    for (i, &p) in basis.iter().enumerate() {
        addr[i] = x % p;
    }
    addr
}

/// Extended Safe Basis at K=7: canonical Safe Basis ∪ {17}.
///
/// `prod(SAFE_BASIS_K7) = 510_510`.
pub const SAFE_BASIS_K7: [u64; 7] = [2, 3, 5, 7, 11, 13, 17];

/// Extended Safe Basis at K=8: canonical Safe Basis ∪ {17, 19}.
///
/// `prod(SAFE_BASIS_K8) = 9_699_690`.
pub const SAFE_BASIS_K8: [u64; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

/// Extended Safe Basis at K=10: canonical Safe Basis ∪ {17, 19, 23, 29}.
pub const SAFE_BASIS_K10: [u64; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

/// Product of a basis. Saturating to avoid overflow at large K.
pub fn basis_product<const K: usize>(basis: &[u64; K]) -> u128 {
    let mut prod: u128 = 1;
    for &p in basis.iter() {
        prod = prod.saturating_mul(p as u128);
    }
    prod
}

#[cfg(test)]
mod tests {
    use super::*;
    use dresden_codex::{SAFE_BASIS, M_SAFE, cram_address};

    /// Sample integers spanning the Goddess section and beyond.
    const SAMPLES: [u64; 14] = [
        0, 1, 5, 11, 13, 19, 148, 177, 260, 365, 584, 1_448, 11_960, 37_960,
    ];

    #[test]
    fn k6_lift_matches_cram_address() {
        for &x in SAMPLES.iter() {
            assert_eq!(lift::<6>(x, &SAFE_BASIS), cram_address(x),
                "K=6 lift must equal cram_address for x={}", x);
        }
    }

    #[test]
    fn k7_extension_preserves_first_six_lanes() {
        for &x in SAMPLES.iter() {
            let k6 = lift::<6>(x, &SAFE_BASIS);
            let k7 = lift::<7>(x, &SAFE_BASIS_K7);
            for i in 0..6 {
                assert_eq!(k6[i], k7[i],
                    "lane {} must agree between K=6 and K=7 for x={}", i, x);
            }
            assert_eq!(k7[6], x % 17, "lane 6 (prime 17) for x={}", x);
        }
    }

    #[test]
    fn k8_extension_preserves_first_seven_lanes() {
        for &x in SAMPLES.iter() {
            let k7 = lift::<7>(x, &SAFE_BASIS_K7);
            let k8 = lift::<8>(x, &SAFE_BASIS_K8);
            for i in 0..7 {
                assert_eq!(k7[i], k8[i],
                    "lane {} must agree between K=7 and K=8 for x={}", i, x);
            }
            assert_eq!(k8[7], x % 19, "lane 7 (prime 19) for x={}", x);
        }
    }

    #[test]
    fn k10_extension_preserves_first_eight_lanes() {
        // The arbitrary-precise scalability proof at K=10.
        for &x in SAMPLES.iter() {
            let k8 = lift::<8>(x, &SAFE_BASIS_K8);
            let k10 = lift::<10>(x, &SAFE_BASIS_K10);
            for i in 0..8 {
                assert_eq!(k8[i], k10[i],
                    "lane {} must agree between K=8 and K=10 for x={}", i, x);
            }
            assert_eq!(k10[8], x % 23);
            assert_eq!(k10[9], x % 29);
        }
    }

    #[test]
    fn every_lane_is_bounded_by_its_prime() {
        // NODE-CTR03 (Uniformity contract) applied at the lift level.
        for &x in SAMPLES.iter() {
            let addr = lift::<10>(x, &SAFE_BASIS_K10);
            for (i, &p) in SAFE_BASIS_K10.iter().enumerate() {
                assert!(addr[i] < p, "addr[{}]={} must be < {} for x={}",
                    i, addr[i], p, x);
            }
        }
    }

    #[test]
    fn basis_product_values_match_documentation() {
        assert_eq!(basis_product(&SAFE_BASIS), M_SAFE as u128);
        assert_eq!(basis_product(&SAFE_BASIS_K7), 510_510);
        assert_eq!(basis_product(&SAFE_BASIS_K8), 9_699_690);
        // SAFE_BASIS_K10 = [2,3,5,7,11,13,17,19,23,29]
        // Product chain: M_SAFE=30030 → ·17=510510 → ·19=9699690
        //              → ·23=223092870 → ·29=6469693230
        assert_eq!(basis_product(&SAFE_BASIS_K10), 6_469_693_230);
    }
}
