//! # Operator-Fabric Engines (v0.8.0 Tier 2 — B-7)
//!
//! Port of the canonical operator-fabric specification from vault
//! `Mayas Engine.md`, adapted to the DCCMS A1 / no-predicate-drift discipline.
//!
//! ## Architecture
//!
//! Each engine is a **heterogeneous CRT operator pipeline**: different
//! lanes with different moduli operating in parallel on the same
//! temporal/structural input, fused via CRT into a single residue tuple.
//!
//! The two foundational types are:
//!
//! - [`lane::Lane`] — one coprime lane with a modulus + identifier + domain
//! - [`lane::MayaState`] — a heterogeneous CRT residue tuple
//!
//! Adapters bridge `MayaState` to the canonical 6-lane Safe Basis
//! representation used by [`dresden_codex::cram_address`].
//!
//! ## Reference engines in this commit
//!
//! - [`vigesimal`] — Engine 1: base-20 number system as `ℤ/4 × ℤ/5`.
//!   Foundational claim: `π(5) = 20` makes the Pisano period of 5
//!   equal to the vigesimal base.
//! - [`tzolkin`] — Engine 2: sacred calendar `ℤ/13 × ℤ/20` with `α(13) = 7`
//!   structuring the tone lane. Bridges to existing
//!   [`crate::h4_visual::dayname::DayNameGlyph`].
//!
//! ## Deferred to subsequent commits (B-7.3 — B-7.6)
//!
//! - **LongCount** (Engine 3) — cylindrical time, tier-promotion through
//!   overflow (mod 20 / mod 18 / mod 20 / mod 20 / linear baktun)
//! - **DresdenEclipse** (Engine 4) — eclipse-window prediction through
//!   commensuration (11960 = 46 × Tzolk'in)
//! - **VenusTable** (Engine 5) — 8/5 Venus-Earth synchronization,
//!   four stations (236 + 90 + 250 + 8 = 584)
//! - **MayaFabric** — unified type binding all five engines on shared
//!   temporal substrate
//!
//! ## Source vs adaptation notes
//!
//! - Vault `Mayas Engine.md` is a pseudo-Rust draft. Its API is preserved
//!   in shape; A1 violations have been replaced with exact-integer forms.
//! - `phi_approximation` is **omitted** entirely (per plan §9 D-1).
//! - `Tzolkin::above_consciousness_threshold` is **omitted** (φ³ threshold
//!   is synthesis O4 — do-not-mechanize).
//! - Day-name orthography follows dccms's modern Mayan canonical spellings
//!   ([`crate::h4_visual::dayname::DayNameGlyph`]) — see plan §9 D-3.
//! - `pisano_profile` returns `Vec<(u64, u64, u64)>` integer triples,
//!   not `Vec<(u64, u64, f64)>` — D-1 mitigation.
//!
//! See [docs/v0_8_0_B7_PLAN.md](../../../../docs/v0_8_0_B7_PLAN.md) and the
//! B-7 section of [executioner_dag.md](../../../../executioner_dag.md).

pub mod lane;
pub mod pisano;
pub mod ramanujan;
pub mod vigesimal;
pub mod tzolkin;
pub mod long_count;       // B-7.3
pub mod dresden_eclipse;  // B-7.4
pub mod venus_table;      // B-7.5
pub mod fabric;           // B-7.6 — unifies all five engines
pub mod recombinant;      // C-1 — winding-preserving residue arithmetic

pub use lane::{Lane, MayaState, MayaStateError};
pub use pisano::{
    PisanoError, PISANO_MAX_MODULUS,
    pisano_period, fibonacci_entry_point, fibonacci_mod_sequence,
};
pub use ramanujan::{euler_totient, mobius, ramanujan_sum};
pub use vigesimal::{Vigesimal, VigesimalError};
pub use tzolkin::{Tzolkin, TzolkinError};
pub use long_count::{LongCount, PeriodEnding};
pub use dresden_eclipse::DresdenEclipse;
pub use venus_table::{VenusTable, VenusStation};
pub use fabric::{MayaFabric, FabricAlignment};
pub use recombinant::{RecombinantState, RecombinantError};

#[cfg(test)]
mod shadow_bond_consumption {
    //! B-5 vocabulary-consumption smoke test (NODE-B5-04).
    //!
    //! Demonstrates that the engines layer cleanly consumes
    //! `dresden_codex::shadow_bond::ShadowBond` for downstream analysis
    //! over `MayaState` — without inverting crate dependencies.
    //! `dresden_codex` produces the typed bond; `dccms_atlas::engines`
    //! lifts the displacement into a heterogeneous-fabric state.

    use super::MayaState;
    use dresden_codex::shadow_bond::{self, ShadowBond};
    use dresden_codex::{cram_address, ECLIPSE_TABLE_DAYS, SATURN_SYNODIC};

    #[test]
    fn saturn_deep_bond_displacement_lifts_to_maya_state() {
        let bond = shadow_bond::detect(SATURN_SYNODIC, ECLIPSE_TABLE_DAYS, 11);
        // Headline case: Δ_S = 242 = 2·11², Deep bond.
        match bond {
            ShadowBond::Deep { prime, power, displacement } => {
                assert_eq!(prime, 11);
                assert_eq!(power, 2);
                assert_eq!(displacement, 242);

                // Lift the displacement into a Safe-Basis MayaState for
                // downstream analysis (which lanes nullify, etc.).
                let state = MayaState::from_cram_address(cram_address(displacement));
                assert_eq!(state.lane_count(), 6);
                // 242 = 2·11² → lane 2 (idx 0) and lane 11 (idx 4) nullify.
                assert_eq!(state.residues()[0], 0);  // 242 mod 2 = 0
                assert_eq!(state.residues()[4], 0);  // 242 mod 11 = 0
                // Other lanes are non-zero:
                assert_ne!(state.residues()[1], 0);  // mod 3
                assert_ne!(state.residues()[2], 0);  // mod 5
                assert_ne!(state.residues()[3], 0);  // mod 7
                assert_ne!(state.residues()[5], 0);  // mod 13
                // Alignment count: exactly 2 lanes at origin (2 and 11).
                assert_eq!(state.alignment_count(), 2);
            }
            other => panic!("expected Deep bond, got {:?}", other),
        }
    }

    #[test]
    fn nobond_displacement_still_lifts_cleanly() {
        // Mars: NoBond at p=11 with displacement 260.
        let bond = shadow_bond::detect(
            dresden_codex::MARS_SYNODIC,
            ECLIPSE_TABLE_DAYS,
            11,
        );
        // 260 mod 11 = 7 → NoBond.
        let displacement = bond.displacement().expect("NoBond carries displacement");
        let state = MayaState::from_cram_address(cram_address(displacement));
        // Lane 11 (idx 4) is non-zero — that's why there's no bond.
        assert_ne!(state.residues()[4], 0);
        // But Mars displacement 260 = 4·5·13 → lanes {2, 5, 13} nullify.
        assert_eq!(state.alignment_count(), 3);
    }
}
