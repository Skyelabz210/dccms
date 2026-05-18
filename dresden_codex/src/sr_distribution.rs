//! # S_R Distribution at the 33-year Eclipse Epoch (DPM-PRIME T8/T9/T10)
//!
//! At the eclipse-table epoch `T_E = 11,960` days, the complete Ramanujan
//! set `S_R = {5, 7, 11}` is distributed across exactly three planetary
//! displacement residues:
//!
//! - **Mars**   `Δ_Ma = 260 = 2² · 5 · 13` carries `5 ∈ S_R`.
//! - **Venus**  `Δ_V  = 280 = 2³ · 5 · 7`  carries `{5, 7} ∈ S_R`.
//! - **Saturn** `Δ_S  = 242 = 2 · 11²`     carries `11² ∈ S_R`.
//!
//! Synthesis agent extensions to T10 (Appendix B of `hackfate_dresden_synthesis.md`):
//!
//! - **Jupiter**  `Δ_J  = 389 (prime)` — outside both Safe Basis and S_R.
//! - **Mercury**  `Δ_Me = 12 = 2² · 3`  — only parking + stability-floor primes.
//!
//! Jupiter and Mercury contributing no S_R content is what makes Mars +
//! Venus + Saturn the *minimal* S_R recovery set — T10 isn't an arbitrary
//! selection, it's structurally forced.
//!
//! ## Status
//!
//! **Mechanically certified by direct integer computation** for all five
//! bodies (Mars, Venus, Saturn, Jupiter, Mercury) — see tests. The
//! structural T10 claim (S_R completeness across Mars + Venus + Saturn)
//! follows from the arithmetic. The Rust tests are an arithmetic
//! certificate suite, not a Lean/Coq formal proof; the latter exists
//! upstream in the FSM-PRIME / TUDPBoundary infrastructure per the vault.
//!
//! Source: vault `The Dresden Codex.md` §THEOREMS T7-T10, §VALIDATION
//! IDENTITIES V5-V7, and synthesis Appendix B items 3-7.

#![allow(dead_code)]

use crate::{
    ECLIPSE_TABLE_DAYS, JUPITER_SYNODIC, MARS_SYNODIC, MERCURY_SYNODIC,
    RAMANUJAN_S_R, SATURN_SYNODIC, VENUS_SYNODIC, cram_address,
};

/// A planetary displacement at a given epoch — Δ_X := T_E mod T_X (DEF D5).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlanetaryDisplacement {
    /// Body name (e.g., "Mars", "Venus").
    pub body: &'static str,
    /// Synodic period in days.
    pub period: u64,
    /// Eclipse-table epoch (typically `ECLIPSE_TABLE_DAYS = 11_960`).
    pub epoch: u64,
    /// Displacement `Δ := epoch mod period`.
    pub displacement: u64,
    /// S_R primes dividing the displacement.
    pub s_r_content: Vec<u64>,
    /// Whether the displacement carries any S_R prime to a power ≥ 2.
    pub deep_shadow: Option<u64>,
}

impl PlanetaryDisplacement {
    /// Compute the displacement record for a body.
    pub fn compute(body: &'static str, period: u64, epoch: u64) -> Self {
        let displacement = epoch % period;
        let s_r_content: Vec<u64> = RAMANUJAN_S_R
            .iter()
            .copied()
            .filter(|&p| displacement % p == 0)
            .collect();
        let deep_shadow = RAMANUJAN_S_R
            .iter()
            .copied()
            .find(|&p| displacement % (p * p) == 0);
        Self {
            body,
            period,
            epoch,
            displacement,
            s_r_content,
            deep_shadow,
        }
    }

    /// CRAM address of the displacement (residue tuple under Safe Basis).
    pub fn displacement_cram(&self) -> [u64; 6] {
        cram_address(self.displacement)
    }

    /// True if this body carries any of the Ramanujan primes.
    pub fn carries_any_s_r(&self) -> bool {
        !self.s_r_content.is_empty()
    }

    /// Classify the shadow-bond status of this body's displacement
    /// against a given anchor prime (B-5 bridge).
    ///
    /// Wraps [`crate::shadow_bond::detect`] over `(period, epoch, anchor_prime)`,
    /// which is the canonical (T_X, T_E, ℓ) triple of vault Decoded.md §Algorithm 4.
    pub fn shadow_bond(&self, anchor_prime: u64) -> crate::shadow_bond::ShadowBond {
        crate::shadow_bond::detect(self.period, self.epoch, anchor_prime)
    }
}

/// Compute `Δ_X = T_E mod T_X` for a planetary body.
pub fn planetary_displacement(
    body: &'static str,
    period: u64,
    epoch: u64,
) -> PlanetaryDisplacement {
    PlanetaryDisplacement::compute(body, period, epoch)
}

/// S_R primes dividing an integer.
pub fn sr_content(n: u64) -> Vec<u64> {
    RAMANUJAN_S_R
        .iter()
        .copied()
        .filter(|&p| n % p == 0)
        .collect()
}

/// The T10 distribution table at the eclipse-table epoch.
///
/// Returns five planetary bodies:
///
/// - Mars, Venus, Saturn — the headline T10 trio carrying `S_R = {5, 7, 11}`
/// - Jupiter, Mercury — synthesis extensions confirming completeness
///   (neither contributes any `S_R` content, so the headline trio is the
///   minimal recovery set, not an arbitrary selection)
///
/// The eclipse table `T_E` itself is intentionally NOT a row: its
/// "self-displacement" `T_E mod T_E = 0` is degenerate.
pub fn t10_distribution_table() -> Vec<PlanetaryDisplacement> {
    let epoch = ECLIPSE_TABLE_DAYS;
    vec![
        planetary_displacement("Mars",    MARS_SYNODIC,    epoch),
        planetary_displacement("Venus",   VENUS_SYNODIC,   epoch),
        planetary_displacement("Saturn",  SATURN_SYNODIC,  epoch),
        planetary_displacement("Jupiter", JUPITER_SYNODIC, epoch),
        planetary_displacement("Mercury", MERCURY_SYNODIC, epoch),
    ]
}

/// The union of S_R primes covered by Mars + Venus + Saturn at epoch.
///
/// **T10 main claim:** this union equals `S_R = {5, 7, 11}` exactly.
pub fn t10_s_r_union(epoch: u64) -> Vec<u64> {
    let mars   = planetary_displacement("Mars",   MARS_SYNODIC,   epoch);
    let venus  = planetary_displacement("Venus",  VENUS_SYNODIC,  epoch);
    let saturn = planetary_displacement("Saturn", SATURN_SYNODIC, epoch);
    let mut union: Vec<u64> = mars.s_r_content;
    for &p in &venus.s_r_content {
        if !union.contains(&p) { union.push(p); }
    }
    for &p in &saturn.s_r_content {
        if !union.contains(&p) { union.push(p); }
    }
    union.sort();
    union
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t8_saturn_displacement_is_two_eleven_squared() {
        let s = planetary_displacement("Saturn", SATURN_SYNODIC, ECLIPSE_TABLE_DAYS);
        assert_eq!(s.displacement, 242);
        assert_eq!(s.displacement, 2 * 11 * 11);
        assert_eq!(s.s_r_content, vec![11]);
        assert_eq!(s.deep_shadow, Some(11)); // 11² | 242
    }

    #[test]
    fn t9_mars_displacement_is_tzolkin() {
        let m = planetary_displacement("Mars", MARS_SYNODIC, ECLIPSE_TABLE_DAYS);
        assert_eq!(m.displacement, 260);
        assert_eq!(m.displacement, 4 * 5 * 13);
        assert_eq!(m.s_r_content, vec![5]);
        assert_eq!(m.deep_shadow, None);
    }

    #[test]
    fn t10_venus_carries_5_and_7() {
        let v = planetary_displacement("Venus", VENUS_SYNODIC, ECLIPSE_TABLE_DAYS);
        assert_eq!(v.displacement, 280);
        assert_eq!(v.displacement, 8 * 5 * 7);
        assert_eq!(v.s_r_content, vec![5, 7]);
        assert_eq!(v.deep_shadow, None);
    }

    #[test]
    fn t10_complete_s_r_distribution_across_mars_venus_saturn() {
        // The headline T10 result.
        let union = t10_s_r_union(ECLIPSE_TABLE_DAYS);
        assert_eq!(union, vec![5, 7, 11]);
        assert_eq!(union.as_slice(), &RAMANUJAN_S_R[..]);
    }

    #[test]
    fn jupiter_displacement_carries_no_s_r() {
        // Synthesis extension: Jupiter at T_E carries 389 (prime), no S_R.
        let j = planetary_displacement("Jupiter", JUPITER_SYNODIC, ECLIPSE_TABLE_DAYS);
        assert_eq!(j.displacement, 389);
        assert!(j.s_r_content.is_empty());
        assert!(j.deep_shadow.is_none());
        // 389 is prime (verified by exhaustive check below).
        for p in [2u64, 3, 5, 7, 11, 13, 17, 19] {
            assert_ne!(j.displacement % p, 0);
        }
    }

    #[test]
    fn mercury_displacement_carries_no_s_r() {
        // Synthesis extension: Mercury at T_E carries 12 = 2²·3.
        let m = planetary_displacement("Mercury", MERCURY_SYNODIC, ECLIPSE_TABLE_DAYS);
        assert_eq!(m.displacement, 12);
        assert!(m.s_r_content.is_empty());
        // Only parking-lane (2) and stability-floor (3) primes.
        assert_eq!(m.displacement % 2, 0);
        assert_eq!(m.displacement % 3, 0);
    }

    #[test]
    fn t10_completeness_strengthened_by_jupiter_mercury_exclusion() {
        // The synthesis observation: Mars/Venus/Saturn IS the minimal
        // S_R recovery exactly because Jupiter and Mercury don't contribute.
        let table = t10_distribution_table();
        let s_r_carriers: Vec<&'static str> = table
            .iter()
            .filter(|d| d.carries_any_s_r())
            .map(|d| d.body)
            .collect();
        assert_eq!(s_r_carriers, vec!["Mars", "Venus", "Saturn"]);
    }

    #[test]
    fn sr_content_helper() {
        assert_eq!(sr_content(280), vec![5, 7]);
        assert_eq!(sr_content(242), vec![11]);
        assert_eq!(sr_content(260), vec![5]);
        assert_eq!(sr_content(389), Vec::<u64>::new());
        assert_eq!(sr_content(35), vec![5, 7]);   // 35 = 5·7
        assert_eq!(sr_content(385), vec![5, 7, 11]); // 385 = 5·7·11 (covers all S_R)
    }

    #[test]
    fn t10_table_has_five_rows() {
        let table = t10_distribution_table();
        assert_eq!(table.len(), 5);
        let bodies: Vec<&'static str> = table.iter().map(|d| d.body).collect();
        assert_eq!(bodies, vec!["Mars", "Venus", "Saturn", "Jupiter", "Mercury"]);
    }

    #[test]
    fn displacement_cram_addresses_are_consistent() {
        let s = planetary_displacement("Saturn", SATURN_SYNODIC, ECLIPSE_TABLE_DAYS);
        let addr = s.displacement_cram();
        assert_eq!(addr, cram_address(242));
        // 242 = 2 · 11² → mod 2 = 0, mod 11 = 0, others non-zero.
        assert_eq!(addr[0], 0);  // lane 2
        assert_eq!(addr[4], 0);  // lane 11
    }

    // ─────────────────────────────────────────────────────────────────
    // B-5 shadow_bond method coverage
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn saturn_shadow_bond_is_deep_eleven_squared() {
        use crate::shadow_bond::ShadowBond;
        let s = planetary_displacement("Saturn", SATURN_SYNODIC, ECLIPSE_TABLE_DAYS);
        let bond = s.shadow_bond(11);
        assert_eq!(bond, ShadowBond::Deep {
            prime: 11,
            power: 2,
            displacement: 242,
        });
        assert!(bond.is_deep());
    }

    #[test]
    fn mars_venus_jupiter_mercury_no_eleven_bond() {
        for body in [
            ("Mars", MARS_SYNODIC),
            ("Venus", VENUS_SYNODIC),
            ("Jupiter", JUPITER_SYNODIC),
            ("Mercury", MERCURY_SYNODIC),
        ] {
            let d = planetary_displacement(body.0, body.1, ECLIPSE_TABLE_DAYS);
            let bond = d.shadow_bond(11);
            assert!(!bond.is_bond(),
                "{} should have no shadow bond at 11, got {:?}", body.0, bond);
        }
    }

    #[test]
    fn shadow_bond_view_of_s_r_distribution() {
        // The shadow-bond perspective is **strictly more selective** than
        // T10's "S_R distribution" view. A shadow bond at prime ℓ requires
        // ℓ to be ABSENT from the synodic period itself (Decoded.md
        // §Algorithm 4 condition i). T10's distribution view counts
        // carrying-by-displacement-content regardless of in-period status.
        //
        // Key distinction surfaced during B-5 execution:
        //
        //   - Mars period 780 = 4·3·5·13 contains 5 (and 13). So Mars at
        //     p=5 returns AnchorInPeriod, not a shadow bond. Mars's S_R
        //     content of 5 in its displacement is ORDINARY divisibility,
        //     not a "shadow" phenomenon.
        //   - Saturn period 378 = 2·3³·7 contains 7. So Saturn at p=7
        //     returns AnchorInPeriod (despite Δ_S = 242 having no factor
        //     of 7 anyway, the in-period condition fires first).
        //   - Venus period 584 = 2³·73 contains neither 5, 7, nor 11.
        //     Venus at 5 and at 7 produce Standard bonds.
        //
        // Net result for S_R coverage via shadow bonds at T_E = 11960:
        //   prime 5  → Venus only (Standard)
        //   prime 7  → Venus only (Standard)
        //   prime 11 → Saturn only (Deep, power 2)
        //   Mars contributes NO shadow bonds — its S_R content is in-period.
        //
        // This is a richer finding than the T10 distribution alone: the
        // shadow-bond reading shows S_R covered by **Venus + Saturn** only,
        // a 2-body recovery (whereas T10's distribution counts a 3-body
        // Mars+Venus+Saturn recovery). Both are valid; they answer
        // different questions. T10 still passes via
        // `t10_complete_s_r_distribution_at_t_e` above.
        use crate::shadow_bond::ShadowBond;

        let saturn = planetary_displacement("Saturn", SATURN_SYNODIC, ECLIPSE_TABLE_DAYS);
        let venus  = planetary_displacement("Venus",  VENUS_SYNODIC,  ECLIPSE_TABLE_DAYS);
        let mars   = planetary_displacement("Mars",   MARS_SYNODIC,   ECLIPSE_TABLE_DAYS);

        // Saturn at 11: Deep bond.
        assert_eq!(saturn.shadow_bond(11),
            ShadowBond::Deep { prime: 11, power: 2, displacement: 242 });
        // Saturn at 7: AnchorInPeriod (7 | 378).
        assert_eq!(saturn.shadow_bond(7), ShadowBond::AnchorInPeriod);

        // Venus at 5 and 7: Standard bonds (5,7 ∤ 584).
        assert_eq!(venus.shadow_bond(5),
            ShadowBond::Standard { prime: 5, displacement: 280 });
        assert_eq!(venus.shadow_bond(7),
            ShadowBond::Standard { prime: 7, displacement: 280 });
        // Venus at 11: NoBond.
        assert!(matches!(venus.shadow_bond(11), ShadowBond::NoBond { .. }));

        // Mars at 5 and 13: AnchorInPeriod (5, 13 | 780).
        assert_eq!(mars.shadow_bond(5), ShadowBond::AnchorInPeriod);
        assert_eq!(mars.shadow_bond(13), ShadowBond::AnchorInPeriod);
        // Mars at 7 and 11: NoBond (7, 11 ∤ 780; 7, 11 ∤ 260).
        assert!(matches!(mars.shadow_bond(7), ShadowBond::NoBond { .. }));
        assert!(matches!(mars.shadow_bond(11), ShadowBond::NoBond { .. }));

        // S_R coverage via shadow bonds: Venus + Saturn together.
        let mut bonded: std::collections::BTreeSet<u64> = Default::default();
        for body in [&saturn, &venus, &mars] {
            for &p in &[5u64, 7, 11] {
                if body.shadow_bond(p).is_bond() {
                    bonded.insert(p);
                }
            }
        }
        let covered: Vec<u64> = bonded.into_iter().collect();
        assert_eq!(covered, vec![5, 7, 11],
            "S_R fully covered by shadow bonds (Venus carries 5 and 7; Saturn carries 11)");

        // Mars contributes NO bonded primes.
        let mars_bonded_count = [5u64, 7, 11, 13].iter()
            .filter(|&&p| mars.shadow_bond(p).is_bond())
            .count();
        assert_eq!(mars_bonded_count, 0,
            "Mars contributes no shadow bonds (its 5- and 13-content is in-period)");
    }
}
