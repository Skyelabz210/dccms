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

pub use lane::{Lane, MayaState, MayaStateError};
pub use pisano::{
    PisanoError, PISANO_MAX_MODULUS,
    pisano_period, fibonacci_entry_point, fibonacci_mod_sequence,
};
pub use ramanujan::{euler_totient, mobius, ramanujan_sum};
pub use vigesimal::{Vigesimal, VigesimalError};
pub use tzolkin::{Tzolkin, TzolkinError};
