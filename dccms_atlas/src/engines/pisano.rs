//! # Pisano periods + Fibonacci entry points (NODE-B7-03)
//!
//! Minimal exact-integer implementations of:
//!
//! - `pisano_period(m)` — π(m), the period of `F_n mod m`
//! - `fibonacci_entry_point(m)` — α(m), the least n > 0 with `m | F_n`
//! - `fibonacci_mod_sequence(m)` — one full Pisano period of values
//!
//! All computed naively (no matrix exponentiation): O(π(m)) time, O(1)
//! space for `pisano_period` and `fibonacci_entry_point`. Scope clamp
//! [`PISANO_MAX_MODULUS`] covers every Maya cycle dccms touches; larger
//! moduli return [`PisanoError::ModulusTooLarge`].
//!
//! ## Foundational claims (mechanized in tests)
//!
//! - `pisano_period(5) == 20` — the Pisano period of 5 is the vigesimal
//!   base. The Maya base-20 numeral system is structurally aligned with
//!   the Fibonacci cycle mod 5.
//! - `fibonacci_entry_point(13) == 7` — α(13) = 7, so every 7th Fibonacci
//!   number is divisible by 13. This connects to the 819-day cycle's
//!   `3² · 7 · 13` structure (DPM-PRIME Lemma L3).
//!
//! Source: vault `Mayas Engine.md` mentions but does not implement these;
//! upstream lives in `crate::pisano` of the CRAM Explorer monorepo per
//! vault CLAUDE.md.

#![allow(dead_code)]

/// Maximum modulus for which Pisano helpers will compute.
///
/// Covers all Maya cycles in dccms (Calendar Round 18_980, Eclipse Table
/// 11_960, Venus 584, Tzolk'in 260, Haab 365). For Pisano periods,
/// π(m) ≤ 6m, so the work bound is O(6 · 100_000) = O(600_000) integer
/// operations — well within the test-time budget.
pub const PISANO_MAX_MODULUS: u64 = 100_000;

/// Errors for Pisano helpers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PisanoError {
    /// `m == 0` — Pisano period undefined.
    ModulusZero,
    /// `m > PISANO_MAX_MODULUS` — out of scope.
    ModulusTooLarge(u64),
}

/// Pisano period π(m): the least n > 0 with `(F_n, F_{n+1}) ≡ (0, 1) (mod m)`.
pub fn pisano_period(m: u64) -> Result<u64, PisanoError> {
    if m == 0 { return Err(PisanoError::ModulusZero); }
    if m > PISANO_MAX_MODULUS { return Err(PisanoError::ModulusTooLarge(m)); }
    if m == 1 { return Ok(1); } // mod 1, everything is 0; period 1 by convention
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    // π(m) ≤ 6m for all m, with equality only at specific m (theorem of Wall).
    let upper_bound = 6u64.saturating_mul(m);
    for n in 1..=upper_bound {
        let next = (a + b) % m;
        a = b;
        b = next;
        if a == 0 && b == 1 {
            return Ok(n);
        }
    }
    // Unreachable for m ≤ PISANO_MAX_MODULUS — π(m) ≤ 6m always holds.
    // Returning a clear error rather than panicking in case the bound
    // is ever exceeded.
    Err(PisanoError::ModulusTooLarge(m))
}

/// Fibonacci entry point α(m): least n > 0 with `m | F_n`.
pub fn fibonacci_entry_point(m: u64) -> Result<u64, PisanoError> {
    if m == 0 { return Err(PisanoError::ModulusZero); }
    if m > PISANO_MAX_MODULUS { return Err(PisanoError::ModulusTooLarge(m)); }
    if m == 1 { return Ok(1); } // F_1 = 1 ≡ 0 mod 1
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let upper_bound = 6u64.saturating_mul(m);
    for n in 1..=upper_bound {
        let next = (a + b) % m;
        a = b;
        b = next;
        if a == 0 {
            return Ok(n);
        }
    }
    Err(PisanoError::ModulusTooLarge(m))
}

/// Fibonacci sequence mod m for one full Pisano period.
///
/// Returns `[F_0 mod m, F_1 mod m, ..., F_{π(m)-1} mod m]` — length π(m).
pub fn fibonacci_mod_sequence(m: u64) -> Result<Vec<u64>, PisanoError> {
    let pi = pisano_period(m)?;
    let mut seq = Vec::with_capacity(pi as usize);
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    seq.push(a);
    for _ in 1..pi {
        seq.push(b);
        let next = (a + b) % m;
        a = b;
        b = next;
    }
    Ok(seq)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modulus_zero_errors() {
        assert_eq!(pisano_period(0), Err(PisanoError::ModulusZero));
        assert_eq!(fibonacci_entry_point(0), Err(PisanoError::ModulusZero));
        assert_eq!(fibonacci_mod_sequence(0), Err(PisanoError::ModulusZero));
    }

    #[test]
    fn modulus_too_large_errors() {
        let m = PISANO_MAX_MODULUS + 1;
        assert_eq!(pisano_period(m), Err(PisanoError::ModulusTooLarge(m)));
        assert_eq!(fibonacci_entry_point(m), Err(PisanoError::ModulusTooLarge(m)));
    }

    #[test]
    fn pisano_period_foundational_values() {
        // π(2) = 3:  0,1,1,0,1,1,...
        assert_eq!(pisano_period(2).unwrap(), 3);
        // π(3) = 8:  0,1,1,2,0,2,2,1,0,1,...
        assert_eq!(pisano_period(3).unwrap(), 8);
        // π(4) = 6:  0,1,1,2,3,1,0,1,...
        assert_eq!(pisano_period(4).unwrap(), 6);
        // **π(5) = 20 — the foundational claim: vigesimal base IS Pisano-of-5.**
        assert_eq!(pisano_period(5).unwrap(), 20);
        // π(7) = 16
        assert_eq!(pisano_period(7).unwrap(), 16);
        // π(8) = 12
        assert_eq!(pisano_period(8).unwrap(), 12);
        // π(10) = 60
        assert_eq!(pisano_period(10).unwrap(), 60);
        // π(11) = 10
        assert_eq!(pisano_period(11).unwrap(), 10);
        // π(13) = 28
        assert_eq!(pisano_period(13).unwrap(), 28);
        // π(20) = lcm(π(4), π(5)) = lcm(6, 20) = 60
        assert_eq!(pisano_period(20).unwrap(), 60);
    }

    #[test]
    fn fibonacci_entry_point_foundational_values() {
        // α(2) = 3:   F_3 = 2
        assert_eq!(fibonacci_entry_point(2).unwrap(), 3);
        // α(3) = 4:   F_4 = 3
        assert_eq!(fibonacci_entry_point(3).unwrap(), 4);
        // α(5) = 5:   F_5 = 5
        assert_eq!(fibonacci_entry_point(5).unwrap(), 5);
        // α(7) = 8:   F_8 = 21 = 3·7
        assert_eq!(fibonacci_entry_point(7).unwrap(), 8);
        // **α(13) = 7 — every 7th Fibonacci is divisible by 13.**
        // F_7 = 13.
        assert_eq!(fibonacci_entry_point(13).unwrap(), 7);
        // α(11) = 10: F_10 = 55 = 5·11
        assert_eq!(fibonacci_entry_point(11).unwrap(), 10);
    }

    #[test]
    fn entry_point_divides_pisano_period() {
        // Theorem: α(m) | π(m) for all m ≥ 1.
        for m in [2u64, 3, 4, 5, 7, 8, 11, 13, 17, 19, 23, 29, 31] {
            let alpha = fibonacci_entry_point(m).unwrap();
            let pi = pisano_period(m).unwrap();
            assert_eq!(pi % alpha, 0, "α({}) = {} should divide π({}) = {}", m, alpha, m, pi);
        }
    }

    #[test]
    fn fibonacci_mod_sequence_starts_with_zero_then_one() {
        let seq = fibonacci_mod_sequence(13).unwrap();
        assert_eq!(seq[0], 0);
        assert_eq!(seq[1], 1);
        // Length equals π(13) = 28.
        assert_eq!(seq.len(), 28);
        // F_7 mod 13 = 0 (α(13) = 7).
        assert_eq!(seq[7], 0);
    }

    #[test]
    fn fibonacci_mod_sequence_for_vigesimal_base() {
        // π(20) = 60: 60-element sequence in vigesimal residue space.
        let seq = fibonacci_mod_sequence(20).unwrap();
        assert_eq!(seq.len(), 60);
        assert_eq!(seq[0], 0);
        assert_eq!(seq[1], 1);
        // The sequence wraps: F_60 ≡ 0 mod 20, F_61 ≡ 1 mod 20.
        // We don't append F_60 (length is exactly π), but the next would
        // close the cycle by definition of Pisano period.
    }

    #[test]
    fn pisano_period_handles_maya_cycles() {
        // Smoke-test for the Maya cycle moduli — these all return.
        for m in [260u64, 365, 584, 780, 819, 1448, 2920, 11_960, 18_980] {
            let pi = pisano_period(m).unwrap();
            assert!(pi <= 6 * m, "π({}) = {} should be ≤ 6m = {}", m, pi, 6 * m);
        }
    }
}
