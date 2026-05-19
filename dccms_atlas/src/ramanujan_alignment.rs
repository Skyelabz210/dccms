//! # Ramanujan-alignment bridge (NODE-B6-03)
//!
//! Bridges [`dresden_codex::shadow_bond::ShadowBond`] (the B-5 detector)
//! with [`prime_hunt::ramanujan_partition`] (B-6 decision procedure).
//!
//! A shadow bond is **Ramanujan-aligned** iff its anchor prime has a
//! first-order Ramanujan partition congruence (i.e., the anchor prime
//! is in `{5, 7, 11}`). This combines the **selectivity** of the
//! shadow-bond view (the anchor prime must be absent from the synodic
//! period) with the **Ramanujan boundary** structure (the prime must
//! be in the Ahlgren-Ono-complete first-order set).
//!
//! ## Saturn case (the headline)
//!
//! Saturn at T_E = 11,960 with anchor 11:
//! - `ShadowBond::Deep { prime: 11, power: 2, displacement: 242 }`
//! - 11 ∈ {5, 7, 11} ⇒ **Ramanujan-aligned and Deep**.
//!
//! This is the strongest classification: a deep shadow bond at a
//! first-order Ramanujan prime. The vault's T-SHADOW-POWER signature
//! and the Ahlgren-Ono boundary converge on this single instance.
//!
//! ## Lives in dccms_atlas because
//!
//! The bridge needs `dresden_codex::shadow_bond::ShadowBond` AND
//! `prime_hunt::ramanujan_partition::first_order_congruence`. `dccms_atlas`
//! already depends on both crates; placing the bridge here avoids
//! adding a `prime_hunt → dresden_codex` edge or vice versa.

#![allow(dead_code)]

use dresden_codex::shadow_bond::ShadowBond;
use prime_hunt::ramanujan_partition::first_order_congruence;

/// Whether a shadow bond is Ramanujan-aligned.
///
/// True iff the bond is `Standard` or `Deep` AND its anchor prime has a
/// first-order Ramanujan partition congruence. Non-bond variants
/// (`AnchorInPeriod`, `NoDisplacement`, `NoBond`) all return `false`.
pub fn is_ramanujan_aligned(bond: &ShadowBond) -> bool {
    let prime = match bond {
        ShadowBond::Standard { prime, .. } | ShadowBond::Deep { prime, .. } => *prime,
        _ => return false,
    };
    first_order_congruence(prime).is_some()
}

/// Whether a shadow bond is Ramanujan-aligned AND Deep — the strongest
/// classification combining T-SHADOW-POWER with the Ahlgren-Ono boundary.
pub fn is_ramanujan_aligned_deep(bond: &ShadowBond) -> bool {
    is_ramanujan_aligned(bond) && bond.is_deep()
}

#[cfg(test)]
mod tests {
    use super::*;
    use dresden_codex::shadow_bond;
    use dresden_codex::{
        ECLIPSE_TABLE_DAYS, JUPITER_SYNODIC, MARS_SYNODIC, MERCURY_SYNODIC,
        SATURN_SYNODIC, VENUS_SYNODIC,
    };

    #[test]
    fn saturn_deep_eleven_bond_is_ramanujan_aligned() {
        // The headline: Saturn's Deep bond at 11.
        let bond = shadow_bond::detect(SATURN_SYNODIC, ECLIPSE_TABLE_DAYS, 11);
        assert!(bond.is_deep());
        assert!(is_ramanujan_aligned(&bond),
            "11 ∈ FIRST_ORDER_PRIMES, so Saturn's bond should be Ramanujan-aligned");
        assert!(is_ramanujan_aligned_deep(&bond),
            "Saturn's bond is BOTH Ramanujan-aligned AND Deep");
    }

    #[test]
    fn venus_standard_bonds_at_5_and_7_are_ramanujan_aligned() {
        // Venus at p=5 → Standard, 5 ∈ {5,7,11} → aligned.
        let venus_5 = shadow_bond::detect(VENUS_SYNODIC, ECLIPSE_TABLE_DAYS, 5);
        assert!(venus_5.is_bond());
        assert!(!venus_5.is_deep());
        assert!(is_ramanujan_aligned(&venus_5));
        assert!(!is_ramanujan_aligned_deep(&venus_5));  // standard, not deep

        let venus_7 = shadow_bond::detect(VENUS_SYNODIC, ECLIPSE_TABLE_DAYS, 7);
        assert!(venus_7.is_bond());
        assert!(is_ramanujan_aligned(&venus_7));
    }

    #[test]
    fn mars_jupiter_mercury_no_bond_means_no_alignment() {
        // NoBond / AnchorInPeriod / NoDisplacement variants are never
        // Ramanujan-aligned (no bond to align with).
        for (period, name) in [
            (MARS_SYNODIC,    "Mars"),
            (JUPITER_SYNODIC, "Jupiter"),
            (MERCURY_SYNODIC, "Mercury"),
        ] {
            let bond = shadow_bond::detect(period, ECLIPSE_TABLE_DAYS, 11);
            assert!(!bond.is_bond(), "{} has no bond at 11", name);
            assert!(!is_ramanujan_aligned(&bond),
                "{} should not be Ramanujan-aligned (no bond)", name);
        }
    }

    #[test]
    fn hypothetical_bond_at_13_not_ramanujan_aligned() {
        // Construct a Standard bond at prime 13 (boundary prime, NOT in
        // {5,7,11}). It carries a shadow but no first-order congruence.
        // Displacement = 26 = 2·13 → Standard at p=13.
        let bond = shadow_bond::for_displacement(26, 13);
        assert!(matches!(bond, ShadowBond::Standard { prime: 13, displacement: 26 }));
        assert!(bond.is_bond());
        // But 13 has no first-order congruence → not Ramanujan-aligned.
        assert!(!is_ramanujan_aligned(&bond),
            "bond at p=13 is a shadow bond but not Ramanujan-aligned (13 ∉ {{5,7,11}})");
    }

    #[test]
    fn anchor_in_period_and_no_displacement_never_aligned() {
        assert!(!is_ramanujan_aligned(&ShadowBond::AnchorInPeriod));
        assert!(!is_ramanujan_aligned(&ShadowBond::NoDisplacement));
    }

    #[test]
    fn ramanujan_aligned_s_r_recovery_is_two_body() {
        // Re-deriving the shadow-bond-view S_R recovery from B-5, now
        // filtered through Ramanujan alignment:
        //
        //   Venus at 5  → Standard, aligned (5 in FIRST_ORDER)
        //   Venus at 7  → Standard, aligned (7 in FIRST_ORDER)
        //   Saturn at 11 → Deep, aligned (11 in FIRST_ORDER)
        //   Mars at *  → no bonds
        //
        // Every shadow bond at T_E with an S_R anchor is automatically
        // Ramanujan-aligned, because S_R = FIRST_ORDER_PRIMES. The
        // Ramanujan boundary structure exactly matches the shadow-bond
        // S_R structure. This is one direction of the deep connection
        // between Discovery 1 (Ahlgren-Ono) and the shadow phenomenon.
        let saturn = shadow_bond::detect(SATURN_SYNODIC, ECLIPSE_TABLE_DAYS, 11);
        let venus_5 = shadow_bond::detect(VENUS_SYNODIC, ECLIPSE_TABLE_DAYS, 5);
        let venus_7 = shadow_bond::detect(VENUS_SYNODIC, ECLIPSE_TABLE_DAYS, 7);

        let aligned_bonds: Vec<&ShadowBond> = [&saturn, &venus_5, &venus_7]
            .iter()
            .filter(|b| is_ramanujan_aligned(b))
            .copied()
            .collect();
        assert_eq!(aligned_bonds.len(), 3,
            "all three shadow bonds at T_E with S_R anchors should be Ramanujan-aligned");
    }
}
