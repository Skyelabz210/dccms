//! # Shadow Bond Detector (B-5)
//!
//! Mechanizes vault `Decoded.md` §Algorithm 4 (Mars-Venus Shadow Prime Bond)
//! as a typed Rust predicate over a synodic period, an epoch, and an anchor
//! prime. Replaces the vault's Python `True`/string returns with a typed
//! [`ShadowBond`] enum exposing the exact divisibility power.
//!
//! ## What a shadow bond is
//!
//! Given a synodic period `T_X` and an epoch `T_E`, the displacement
//! `Δ = T_E mod T_X` is the residue of the epoch within the synodic
//! cycle. A **shadow bond** at anchor prime `ℓ` exists when:
//!
//! 1. `ℓ ∤ T_X` — the prime is **absent** from the synodic period itself
//!    (otherwise the bond reduces to ordinary divisibility, not "shadow"),
//! 2. `Δ ≠ 0` — the displacement is non-trivial (the epoch doesn't land
//!    exactly on a cycle boundary),
//! 3. `ℓ | Δ` — the prime divides the displacement.
//!
//! The bond is **Deep** when `ℓ² | Δ` (or higher power) — this is the
//! T-SHADOW-POWER signature from vault Decoded.md §Algorithm 9 / DPM-PRIME
//! T8. Saturn's 33-year case `Δ_S = 242 = 2 · 11²` is the headline
//! instance.
//!
//! ## Source provenance
//!
//! - Vault `Decoded.md` §Algorithm 4 (Python pseudocode, lines 61-86)
//! - Vault `The Dresden Codex.md` §T8 (Saturn displacement theorem)
//! - DPM-PRIME §LEMMA L8 (`Δ_S = 242 = 2 · 11²`)
//! - Vault returns Python strings `"STANDARD SHADOW BOND"` / `"DEEP SHADOW
//!   BOND DETECTED (Power >= 2)"`; this module returns typed variants
//!   with the **exact** power as `u32` (D-2 hardening).

#![allow(dead_code)]

/// Classification of the shadow-bond status between a synodic period
/// and an epoch at a given anchor prime.
///
/// Variants represent the disjoint outcomes of vault Algorithm 4's
/// condition checks, with two named "no-bond" cases for clarity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShadowBond {
    /// The anchor prime divides the synodic period itself
    /// (vault condition (i) fails: `ℓ | T_X`).
    ///
    /// The prime is **not absent** from the period surface, so any
    /// alignment is ordinary divisibility, not a shadow phenomenon.
    AnchorInPeriod,

    /// The displacement is exactly zero — the epoch lands on a
    /// synodic cycle boundary. Trivial alignment, no bond.
    /// (vault condition (iii) fails: `Δ = 0`).
    NoDisplacement,

    /// The anchor prime does NOT divide the displacement.
    /// No shadow bond at this prime for this period/epoch pair.
    NoBond {
        /// The anchor prime tested.
        prime: u64,
        /// The displacement `Δ = epoch mod period`.
        displacement: u64,
    },

    /// **Standard shadow bond**: `ℓ | Δ` but `ℓ² ∤ Δ`. The anchor prime
    /// divides the displacement to exact power 1.
    Standard {
        prime: u64,
        displacement: u64,
    },

    /// **Deep shadow bond** (T-SHADOW-POWER): `ℓ^k | Δ` with `k ≥ 2`.
    /// The `power` field reports the **exact** k such that `ℓ^k | Δ`
    /// but `ℓ^(k+1) ∤ Δ` — strictly more informative than the vault's
    /// "Power ≥ 2" string return.
    Deep {
        prime: u64,
        /// Exact divisibility power. Always `≥ 2` for the `Deep` variant.
        power: u32,
        displacement: u64,
    },
}

impl ShadowBond {
    /// Whether this is any kind of shadow bond (Standard or Deep).
    pub fn is_bond(&self) -> bool {
        matches!(self, Self::Standard { .. } | Self::Deep { .. })
    }

    /// Whether this is specifically a Deep shadow bond.
    pub fn is_deep(&self) -> bool {
        matches!(self, Self::Deep { .. })
    }

    /// Extract the anchor prime, if the variant carries one.
    pub fn prime(&self) -> Option<u64> {
        match self {
            Self::AnchorInPeriod | Self::NoDisplacement => None,
            Self::NoBond { prime, .. }
            | Self::Standard { prime, .. }
            | Self::Deep { prime, .. } => Some(*prime),
        }
    }

    /// Extract the displacement, if the variant carries one.
    pub fn displacement(&self) -> Option<u64> {
        match self {
            Self::AnchorInPeriod | Self::NoDisplacement => None,
            Self::NoBond { displacement, .. }
            | Self::Standard { displacement, .. }
            | Self::Deep { displacement, .. } => Some(*displacement),
        }
    }

    /// Extract the divisibility power (1 for Standard, ≥ 2 for Deep,
    /// `None` for non-bond variants).
    pub fn power(&self) -> Option<u32> {
        match self {
            Self::Standard { .. } => Some(1),
            Self::Deep { power, .. } => Some(*power),
            _ => None,
        }
    }
}

/// Detect a shadow bond from a synodic period, an epoch, and an anchor prime.
///
/// Mechanizes vault `Decoded.md` §Algorithm 4 exactly:
///
/// 1. If `anchor_prime | period` → [`ShadowBond::AnchorInPeriod`].
/// 2. Compute `displacement = epoch mod period`.
/// 3. If `displacement == 0` → [`ShadowBond::NoDisplacement`].
/// 4. If `anchor_prime ∤ displacement` → [`ShadowBond::NoBond`].
/// 5. Otherwise compute the exact power `k` such that `prime^k | displacement`
///    but `prime^(k+1) ∤ displacement` and return [`ShadowBond::Standard`]
///    (if `k == 1`) or [`ShadowBond::Deep`] (if `k ≥ 2`).
///
/// `anchor_prime` is not required to be prime by the function itself;
/// the function's identity depends on the caller's choice. Passing a
/// non-prime anchor is mathematically meaningful (the same divisibility
/// chain still applies) but the "shadow" name presumes primality.
pub fn detect(period: u64, epoch: u64, anchor_prime: u64) -> ShadowBond {
    if anchor_prime == 0 || period == 0 {
        // Degenerate: defined as NoBond with displacement 0.
        return ShadowBond::AnchorInPeriod; // 0 divides 0 → treat as anchor-in-period
    }
    if period % anchor_prime == 0 {
        return ShadowBond::AnchorInPeriod;
    }
    let displacement = epoch % period;
    for_displacement(displacement, anchor_prime)
}

/// Detect a shadow bond from a precomputed displacement and an anchor prime.
///
/// Identical to [`detect`] but skips the period-divisibility check
/// (caller is asserting `anchor_prime ∤ period`). Useful when the
/// displacement is already known.
pub fn for_displacement(displacement: u64, anchor_prime: u64) -> ShadowBond {
    if anchor_prime == 0 {
        return ShadowBond::AnchorInPeriod;
    }
    if displacement == 0 {
        return ShadowBond::NoDisplacement;
    }
    if displacement % anchor_prime != 0 {
        return ShadowBond::NoBond {
            prime: anchor_prime,
            displacement,
        };
    }
    // Compute exact power: divide repeatedly.
    let mut d = displacement;
    let mut power: u32 = 0;
    while d % anchor_prime == 0 {
        d /= anchor_prime;
        power += 1;
        // Safety: u32 max is 4.3B, dividing a u64 by anchor_prime ≥ 2 means
        // at most 64 iterations. Bound is overgenerous but explicit.
        if power > 64 { break; }
    }
    if power >= 2 {
        ShadowBond::Deep {
            prime: anchor_prime,
            power,
            displacement,
        }
    } else {
        // power == 1
        ShadowBond::Standard {
            prime: anchor_prime,
            displacement,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ECLIPSE_TABLE_DAYS, JUPITER_SYNODIC, MARS_SYNODIC, MERCURY_SYNODIC,
        SATURN_SYNODIC, VENUS_SYNODIC,
    };

    // ─────────────────────────────────────────────────────────────────
    // Variant coverage — every ShadowBond variant has a named test
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn anchor_in_period_when_prime_divides_period() {
        // 121 = 11² ⇒ 11 | 121.
        let bond = detect(121, 0, 11);
        assert_eq!(bond, ShadowBond::AnchorInPeriod);
        // Synodic period itself = anchor prime.
        let bond = detect(11, 100, 11);
        assert_eq!(bond, ShadowBond::AnchorInPeriod);
    }

    #[test]
    fn no_displacement_when_epoch_is_period_multiple() {
        // 260 | 1040 (= 4·260).
        let bond = detect(260, 1040, 7);
        assert_eq!(bond, ShadowBond::NoDisplacement);
    }

    #[test]
    fn no_bond_when_prime_does_not_divide_displacement() {
        // Mars at T_E: Δ_Ma = 260. 260 mod 11 = 7.
        let bond = detect(MARS_SYNODIC, ECLIPSE_TABLE_DAYS, 11);
        assert_eq!(bond, ShadowBond::NoBond {
            prime: 11,
            displacement: 260,
        });
    }

    #[test]
    fn standard_bond_when_power_exactly_one() {
        // Construct: displacement = 14 = 2·7 (power 1 of 7).
        let bond = for_displacement(14, 7);
        assert_eq!(bond, ShadowBond::Standard {
            prime: 7,
            displacement: 14,
        });
    }

    #[test]
    fn deep_bond_power_two_saturn() {
        // Saturn case — the headline. Δ_S = 242 = 2·11².
        let bond = detect(SATURN_SYNODIC, ECLIPSE_TABLE_DAYS, 11);
        assert_eq!(bond, ShadowBond::Deep {
            prime: 11,
            power: 2,
            displacement: 242,
        });
    }

    #[test]
    fn deep_bond_higher_power() {
        // Construct: displacement = 8 = 2³ (power 3 of 2).
        let bond = for_displacement(8, 2);
        assert_eq!(bond, ShadowBond::Deep {
            prime: 2,
            power: 3,
            displacement: 8,
        });
    }

    // ─────────────────────────────────────────────────────────────────
    // T10 cross-check: only Saturn carries a shadow bond at T_E with p=11
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn t10_only_saturn_carries_shadow_bond_at_t_e() {
        // Across all 5 planets at T_E = 11960 with anchor 11.
        let mars   = detect(MARS_SYNODIC,    ECLIPSE_TABLE_DAYS, 11);
        let venus  = detect(VENUS_SYNODIC,   ECLIPSE_TABLE_DAYS, 11);
        let saturn = detect(SATURN_SYNODIC,  ECLIPSE_TABLE_DAYS, 11);
        let jupiter = detect(JUPITER_SYNODIC, ECLIPSE_TABLE_DAYS, 11);
        let mercury = detect(MERCURY_SYNODIC, ECLIPSE_TABLE_DAYS, 11);

        // Only Saturn bonds; the rest are NoBond.
        assert!(saturn.is_bond());
        assert!(saturn.is_deep());
        assert!(!mars.is_bond());
        assert!(!venus.is_bond());
        assert!(!jupiter.is_bond());
        assert!(!mercury.is_bond());
    }

    #[test]
    fn t10_at_other_anchor_primes() {
        // Mars Δ = 260 = 2²·5·13.
        // → no bond at 11; standard bond at 5 (and 13);
        //   13 | 260 = 4·5·13, 13² ∤ 260, so power 1.
        let mars_at_13 = detect(MARS_SYNODIC, ECLIPSE_TABLE_DAYS, 13);
        // 13 ∤ 780? 780 = 4·3·5·13, so 13 | 780. → AnchorInPeriod.
        assert_eq!(mars_at_13, ShadowBond::AnchorInPeriod);

        // Venus Δ = 280 = 2³·5·7.
        // → check at 5: 5 | 280, but check 5 | 584 (Venus period) first.
        //   584 = 2³·73, so 5 ∤ 584. So condition (i) passes.
        //   280 mod 5 = 0; 280 / 5 = 56; 56 mod 5 = 1. So power 1 → Standard.
        let venus_at_5 = detect(VENUS_SYNODIC, ECLIPSE_TABLE_DAYS, 5);
        assert_eq!(venus_at_5, ShadowBond::Standard {
            prime: 5,
            displacement: 280,
        });

        // Venus at 7: 7 ∤ 584; 280 mod 7 = 0; 280 / 7 = 40; 40 mod 7 = 5.
        //   Power 1 → Standard.
        let venus_at_7 = detect(VENUS_SYNODIC, ECLIPSE_TABLE_DAYS, 7);
        assert_eq!(venus_at_7, ShadowBond::Standard {
            prime: 7,
            displacement: 280,
        });
    }

    // ─────────────────────────────────────────────────────────────────
    // Equivalence: detect ↔ for_displacement
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn detect_equals_for_displacement_when_anchor_not_in_period() {
        // For any (period, epoch, prime) with prime ∤ period,
        // detect should produce the same result as for_displacement(epoch % period, prime).
        for period in [13u64, 17, 19, 23, 73, 365] {
            for epoch in [100u64, 1000, 11_960, 18_980] {
                let prime = 11; // 11 ∤ any of the periods above
                if period % prime == 0 { continue; }
                let d = epoch % period;
                assert_eq!(detect(period, epoch, prime), for_displacement(d, prime));
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Helper-method coverage
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn shadow_bond_helper_methods() {
        let deep = ShadowBond::Deep { prime: 11, power: 2, displacement: 242 };
        assert!(deep.is_bond());
        assert!(deep.is_deep());
        assert_eq!(deep.prime(), Some(11));
        assert_eq!(deep.displacement(), Some(242));
        assert_eq!(deep.power(), Some(2));

        let std = ShadowBond::Standard { prime: 5, displacement: 280 };
        assert!(std.is_bond());
        assert!(!std.is_deep());
        assert_eq!(std.power(), Some(1));

        let none = ShadowBond::NoBond { prime: 11, displacement: 7 };
        assert!(!none.is_bond());
        assert!(!none.is_deep());
        assert_eq!(none.power(), None);

        let anchor = ShadowBond::AnchorInPeriod;
        assert!(!anchor.is_bond());
        assert_eq!(anchor.prime(), None);
        assert_eq!(anchor.displacement(), None);
    }

    // ─────────────────────────────────────────────────────────────────
    // Degenerate input handling
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn zero_anchor_yields_anchor_in_period() {
        assert_eq!(detect(100, 50, 0), ShadowBond::AnchorInPeriod);
        assert_eq!(for_displacement(50, 0), ShadowBond::AnchorInPeriod);
    }

    #[test]
    fn zero_period_yields_anchor_in_period() {
        // Degenerate: avoid division by zero in epoch % period.
        assert_eq!(detect(0, 50, 11), ShadowBond::AnchorInPeriod);
    }

    // ─────────────────────────────────────────────────────────────────
    // Power reporting precision (D-2 hardening)
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn power_reporting_is_exact() {
        // Vault returns "Power >= 2"; we report the exact power.
        // 2^5 = 32: power should be exactly 5.
        let bond = for_displacement(32, 2);
        assert_eq!(bond.power(), Some(5));
        // 2^5 · 3 = 96: power 5 of 2 (other factor doesn't divide).
        let bond = for_displacement(96, 2);
        assert_eq!(bond.power(), Some(5));
        // 11³ = 1331: power 3.
        let bond = for_displacement(1331, 11);
        assert_eq!(bond.power(), Some(3));
    }
}
