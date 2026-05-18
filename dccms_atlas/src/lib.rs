//! # DCCMS — Dresden Codex Configuration Manifold Study
//!
//! **Anthony Diaz (Acid) — HackFate.us / Skyelabz210**
//!
//! ## Mission
//!
//! Characterize the configuration manifold of the QMNF/CRAM substrate
//! by triangulating between three independent observations of it:
//! the Dresden Codex (the artifact the Maya left), the QMNF library
//! (the artifact Acid built), and the NS workspace (the artifact
//! Acid constructed as a falsifier and which now stands as proven
//! infrastructure).
//!
//! All three speak about the same object.
//!
//! ## Directive 0
//!
//! When the analysis encounters an apparent limit, the limit is
//! converted into a build target. Specify what instrument would
//! resolve the limit. Build the instrument. Run it. Report.
//! The only acceptable "cannot" is one where the instrument itself
//! has been demonstrated impossible — not difficult, not out of scope.
//!
//! ## Architecture
//!
//! `dccms_atlas` is a thin query layer on top of the verified L1
//! workspace. It does not duplicate or replace any substrate code.
//! It composes existing primitives into a configuration-space
//! navigation engine for the codex.
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │  DCCMS query layer (this crate)                             │
//! │  ┌──────────┬──────────┬──────────┬──────────┬──────────┐  │
//! │  │ heads    │ atlas    │ events   │ h4_inst  │ h5_dist  │  │
//! │  └──────────┴──────────┴──────────┴──────────┴──────────┘  │
//! └─────────────────────────────────────────────────────────────┘
//!                              │
//!                              ▼
//! ┌─────────────────────────────────────────────────────────────┐
//! │  L1 substrate (verified, 991 tests, 54K Rust lines)         │
//! │  ┌─────────────┬─────────────┬─────────────┬─────────────┐ │
//! │  │ dresden_    │ prime_hunt  │ qmnf_       │ qcid_ns_    │ │
//! │  │ codex       │             │ primitives  │ bridge      │ │
//! │  └─────────────┴─────────────┴─────────────┴─────────────┘ │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Hypotheses Under Test
//!
//! - **H1:** A single generator produces the four Maya calendars as
//!   marked points on the configuration manifold.
//! - **H2:** The four calendars correspond to four heterogeneous
//!   Hydra heads. The four-head atlas explains codex events better
//!   than any single-head atlas.
//! - **H3:** Additional configurations exist within the same generator.
//!   Candidates: 11²-shadow navigation (Saturn), 12-fold zodiac topology,
//!   4/16-fold temperaments. Each tested via T13 resolution gain.
//! - **H4:** The goddess section encodes configuration metadata. Tested
//!   via six instruments (see `h4_instruments` module).
//! - **H5:** Prime 11 is the navigation index between configurations,
//!   not a structural prime of any single configuration. 11-lane
//!   distribution should be uniform across the four-head address
//!   space and non-uniform within any single head.
//!
//! ## Module Map
//!
//! - [`heads`] — Hydra-head construction from Maya calendars
//! - [`events`] — Codex-event encoding (Venus risings, eclipses, etc.)
//! - [`atlas`]  — Address-space atlas built over the four-head signature
//! - [`recumbent`] — Recumbent state with multi-modulus winding
//! - [`h4_instruments`] — The six goddess-section instruments
//! - [`h5_navigator`]   — The 11-lane distributional test
//! - [`dkam_filter`]    — Admissibility filter for candidate heads
//! - [`adelic_index`]   — Dual-track adelic indexing for events

#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic)]

pub mod heads;
pub mod events;
pub mod atlas;
pub mod recumbent;
pub mod h3_mi;
pub mod h4_instruments;
pub mod h5_navigator;
pub mod dkam_filter;
pub mod adelic_index;
pub mod manifold_geometry;

// ── v0.2.0 additions ─────────────────────────────────────────────────────
pub mod h1_generator;
pub mod h5_refined;
pub mod h4_non_visual;

// ── v0.3.0 additions ─────────────────────────────────────────────────────
pub mod h1_stage8;
pub mod h5_level;
pub mod cross_validation;
pub mod h4_montgomery;

// ── v0.4.0 additions ─────────────────────────────────────────────────────
pub mod h3_extended;
pub mod h5_katun;
pub mod manifold_upgrade;
pub mod generator_catalog;

// ── v0.5.0 — Dresden Codex decoder ───────────────────────────────────────
pub mod lunar;
pub mod moon_goddess;
pub mod codex_decoder;

// ── v0.6.0 — CRAM-ENHANCE Venus decoder ──────────────────────────────────
pub mod venus_kernel;
pub mod substrate_roles;

// ═══════════════════════════════════════════════════════════════════
// Re-exports — convenience for downstream consumers
// ═══════════════════════════════════════════════════════════════════

pub use heads::{HydraHead, FourCalendarHydra, HeadSignature};
pub use events::{CodexEvent, CodexEventKind, EventSet};
pub use atlas::{ConfigAtlas, AtlasEntry, AddressSpace};
pub use recumbent::{RecumbentState, VenusConductorState};
pub use h3_mi::{H3MIReport, h3_mi_report, mutual_information_nbp};
pub use h4_instruments::{H4Report, H4Verdict};
pub use h5_navigator::{H5Report, H5Verdict};
pub use dkam_filter::{DkamFilter, AdmissibilityReport};
pub use adelic_index::{AdelicIndex, ArchimedeanWitness, NonArchimedeanAddress};
pub use manifold_geometry::{ManifoldProfile, ConnectivityReport, BitCorrelationMatrix, hamming_1_connectivity};

// ═══════════════════════════════════════════════════════════════════
// Project constants — pre-registered at DCCMS declaration
// ═══════════════════════════════════════════════════════════════════

/// Safe Basis for DCCMS (matches the L1 substrate's Safe Basis).
pub const SAFE_BASIS: [u64; 6] = dresden_codex::SAFE_BASIS;

/// Safe Basis product.
pub const M_SAFE: u64 = dresden_codex::M_SAFE;

/// Shadow Prime — the navigation prime (H5 candidate).
pub const SHADOW_PRIME: u64 = 11;

/// Boundary Prime — the first post-Ramanujan prime.
pub const BOUNDARY_PRIME: u64 = 13;

/// Transport Core — primes admissible for substrate operations.
pub const TRANSPORT_CORE: [u64; 4] = [3, 7, 11, 13];

/// Ramanujan primes — congruences live here.
pub const RAMANUJAN_PRIMES: [u64; 3] = [5, 7, 11];

/// Venus conductor moduli for recumbent indexing.
pub const VENUS_CONDUCTOR_MODULI: [u64; 5] = [260, 365, 584, 2920, 37960];

/// DKAM resonance order for the Transport Core.
pub const RHO_TRANSPORT: u64 = 3;

/// DKAM maximum admissible operator degree.
pub const DKAM_MAX_DEGREE: u64 = 2;

/// H3 resolution-gain threshold (Gini increase, basis points).
/// 1000 bp = 10% — pre-registered at project declaration.
pub const H3_THRESHOLD_BP: u64 = 1000;

/// H4 instrument count — must reach this many for "supported" verdict.
pub const H4_SUPPORT_THRESHOLD: usize = 3;

/// H4 total instrument count.
pub const H4_INSTRUMENT_COUNT: usize = 6;

// ═══════════════════════════════════════════════════════════════════
// Project version
// ═══════════════════════════════════════════════════════════════════

pub const DCCMS_VERSION: &str = "0.6.0";
pub const DCCMS_DECLARATION_DATE: &str = "2026-05-16";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_basis_matches_substrate() {
        assert_eq!(SAFE_BASIS, dresden_codex::SAFE_BASIS);
        assert_eq!(M_SAFE, 30030);
    }

    #[test]
    fn shadow_prime_in_basis() {
        assert!(SAFE_BASIS.contains(&SHADOW_PRIME));
    }

    #[test]
    fn boundary_prime_in_basis() {
        assert!(SAFE_BASIS.contains(&BOUNDARY_PRIME));
    }

    #[test]
    fn transport_core_is_subset_of_safe_basis() {
        for p in TRANSPORT_CORE {
            assert!(SAFE_BASIS.contains(&p),
                "Transport core prime {} not in Safe Basis", p);
        }
    }

    #[test]
    fn dkam_subcriticality_holds_for_transport_core() {
        // deg(operator) < rho(basis) is the admissibility condition
        assert!(DKAM_MAX_DEGREE < RHO_TRANSPORT);
    }

    #[test]
    fn venus_conductor_moduli_share_structure() {
        // 37960 = 65 × 584 = 146 × 260 = 104 × 365
        assert_eq!(VENUS_CONDUCTOR_MODULI[4], 65 * VENUS_CONDUCTOR_MODULI[2]);
        assert_eq!(VENUS_CONDUCTOR_MODULI[4], 146 * VENUS_CONDUCTOR_MODULI[0]);
        assert_eq!(VENUS_CONDUCTOR_MODULI[4], 104 * VENUS_CONDUCTOR_MODULI[1]);
        // 2920 = 5 × 584 = 8 × 365
        assert_eq!(VENUS_CONDUCTOR_MODULI[3], 5 * VENUS_CONDUCTOR_MODULI[2]);
        assert_eq!(VENUS_CONDUCTOR_MODULI[3], 8 * VENUS_CONDUCTOR_MODULI[1]);
    }
}
