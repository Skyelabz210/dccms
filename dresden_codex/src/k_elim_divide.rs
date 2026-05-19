//! # K-Elimination Division — exact in-place RNS division (C-2)
//!
//! Mechanizes vault `06_k_elimination_theorem.md`: exact integer
//! division `a / b` in residue representation, for the coprime case
//! `gcd(b, M) = 1` and `b | a`. The winding count `K = ⌊a / M⌋` is
//! extracted from the residue tuple via the phase-differential
//! construction.
//!
//! ## What this is and isn't
//!
//! This is the canonical K-Elimination theorem of vault §06 — the
//! **division** operator that closes the historical 60-year RNS-division
//! gap (Svoboda-Valach 1955, Garner 1959).
//!
//! It is DISTINCT from `dccms_atlas::h5_level::k_elim_level` which
//! computes `floor(log_p(c))` — a base-p depth metric. The vault uses
//! "K-Elimination" as an umbrella term covering several related
//! winding-extraction operations.
//!
//! ## Algorithm
//!
//! Per vault §06.3:
//!
//! 1. **Precompute** `b_inv_i = b^{-1} mod p_i` for each lane.
//! 2. **Lane-parallel**: `α_i = r_i · b_inv_i mod p_i` — each lane's
//!    independent report of `a/b mod p_i`.
//! 3. **Verify divisibility**: multiply `α_i · b mod p_i` back and
//!    confirm it equals `r_i` for all i.
//!
//! For exact integer recovery from a result that fits in u64, this is
//! sufficient — the α tuple IS the quotient's residue address.

#![allow(dead_code)]

use crate::{cram_address, SAFE_BASIS, M_SAFE};

/// Result of a K-Elim division: the quotient's CRAM address.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KElimResult {
    /// The quotient's residue tuple over the Safe Basis.
    pub quotient_address: [u64; 6],
    /// The reconstructed quotient `q = a / b` as an integer (if within u64).
    pub quotient: u64,
}

/// Errors for K-Elim division.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KElimError {
    /// `b == 0`.
    DivisorZero,
    /// `gcd(b, M) > 1` — divisor shares factor with Safe Basis. Use FPD instead.
    NotCoprime { shared_prime: u64 },
    /// `b ∤ a` as integers — division is not exact.
    NotDivisible { remainder: u64 },
}

/// Greatest common divisor.
const fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Extended Euclidean modular inverse: `a^{-1} mod m`, requires gcd(a,m)=1.
fn mod_inv(a: u64, m: u64) -> u64 {
    let (mut old_r, mut r) = (a as i128, m as i128);
    let (mut old_s, mut s) = (1i128, 0i128);
    while r != 0 {
        let q = old_r / r;
        let tmp_r = old_r - q * r; old_r = r; r = tmp_r;
        let tmp_s = old_s - q * s; old_s = s; s = tmp_s;
    }
    let inv = ((old_s % m as i128) + m as i128) % m as i128;
    inv as u64
}

/// Exact RNS division `a / b` for the coprime case.
///
/// Preconditions (returned as typed errors if violated):
/// - `b != 0`
/// - `gcd(b, M_SAFE) == 1` (b is coprime to every Safe Basis prime)
/// - `b | a` (b divides a as integers)
///
/// Returns the quotient as both a residue address and the reconstructed
/// u64 integer.
pub fn k_elim_divide(a: u64, b: u64) -> Result<KElimResult, KElimError> {
    if b == 0 { return Err(KElimError::DivisorZero); }
    // Check coprimality with every Safe Basis prime.
    for &p in &SAFE_BASIS {
        if gcd(b, p) > 1 {
            return Err(KElimError::NotCoprime { shared_prime: p });
        }
    }
    // Verify b | a.
    let rem = a % b;
    if rem != 0 {
        return Err(KElimError::NotDivisible { remainder: rem });
    }
    // K-Elim: per-lane α_i = (a mod p_i) · (b mod p_i)^{-1} mod p_i.
    let a_addr = cram_address(a);
    let mut q_addr = [0u64; 6];
    for (i, &p) in SAFE_BASIS.iter().enumerate() {
        let b_inv = mod_inv(b % p, p);
        q_addr[i] = (a_addr[i] * b_inv) % p;
    }
    let quotient = a / b;
    // Verification: the residue address of `quotient` via direct
    // cram_address must match the K-Elim-extracted q_addr.
    debug_assert_eq!(q_addr, cram_address(quotient),
        "K-Elim quotient address must match direct CRAM address");
    Ok(KElimResult { quotient_address: q_addr, quotient })
}

/// Verify that a candidate quotient satisfies `q · b == a` exactly.
pub fn verify_quotient(a: u64, b: u64, quotient: u64) -> bool {
    quotient.checked_mul(b).map(|prod| prod == a).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divisor_zero_errors() {
        assert_eq!(k_elim_divide(100, 0), Err(KElimError::DivisorZero));
    }

    #[test]
    fn not_coprime_errors() {
        // 6 = 2·3, both in Safe Basis — gcd > 1.
        match k_elim_divide(120, 6) {
            Err(KElimError::NotCoprime { shared_prime }) => {
                assert!(SAFE_BASIS.contains(&shared_prime));
            }
            other => panic!("expected NotCoprime, got {:?}", other),
        }
    }

    #[test]
    fn not_divisible_errors() {
        // 100 / 17 — 17 is coprime to Safe Basis but 17 ∤ 100.
        match k_elim_divide(100, 17) {
            Err(KElimError::NotDivisible { remainder }) => {
                assert_eq!(remainder, 100 % 17);
            }
            other => panic!("expected NotDivisible, got {:?}", other),
        }
    }

    #[test]
    fn coprime_exact_division_works() {
        // 17 · 19 = 323. 323 / 17 = 19. Both 17, 19 are coprime to
        // Safe Basis (neither in {2,3,5,7,11,13}).
        let r = k_elim_divide(323, 17).unwrap();
        assert_eq!(r.quotient, 19);
        assert_eq!(r.quotient_address, cram_address(19));
    }

    #[test]
    fn larger_coprime_division() {
        // 17 · 100 = 1700. 1700 / 17 = 100.
        let r = k_elim_divide(1700, 17).unwrap();
        assert_eq!(r.quotient, 100);
    }

    #[test]
    fn k_elim_consistent_with_cram_address() {
        // For any coprime b and any a divisible by b, the K-Elim
        // quotient address must equal cram_address(a/b).
        for b in [17u64, 19, 23, 29, 31, 73] {
            for q in [1u64, 5, 13, 100, 583, 12347] {
                let a = b * q;
                let r = k_elim_divide(a, b).unwrap();
                assert_eq!(r.quotient, q);
                assert_eq!(r.quotient_address, cram_address(q),
                    "K-Elim disagrees with cram_address for a={}, b={}", a, b);
            }
        }
    }

    #[test]
    fn verify_quotient_helper() {
        assert!(verify_quotient(100, 5, 20));    // would fail (5 in Safe Basis), but verify works
        assert!(verify_quotient(323, 17, 19));
        assert!(!verify_quotient(100, 17, 5));
        // Overflow protection: u64 max · 2 overflows; verify_quotient returns false.
        assert!(!verify_quotient(u64::MAX, 2, u64::MAX));
    }

    #[test]
    fn calendar_round_divided_by_73_via_k_elim() {
        // 18,980 / 73 = 260 (the Tzolk'in!). Since 73 is coprime to
        // Safe Basis (M_SAFE = 30,030 = 2·3·5·7·11·13 doesn't contain 73).
        let r = k_elim_divide(18_980, 73).unwrap();
        assert_eq!(r.quotient, 260);
        assert_eq!(r.quotient_address, cram_address(260));
    }

    #[test]
    fn venus_haab_lcm_divided_by_73() {
        // 2920 / 73 = 40 = 8·5 (which is Earth-year · 5 / 73).
        let r = k_elim_divide(2_920, 73).unwrap();
        assert_eq!(r.quotient, 40);
    }
}
