//! # H4 Visual Transducer (NODE-VT* / NODE-WIRE* / NODE-CTR* / NODE-FO*)
//!
//! Discrete glyph → CRAM-address transducer that closes the H4 hypothesis
//! ("Goddess section encodes configuration metadata") from PARTIAL to
//! SUPPORTED. Built per the executioner DAG in
//! [executioner_dag.md](../../../executioner_dag.md).
//!
//! ## Architecture
//!
//! ```text
//!   glyph (finite alphabet)
//!         │ GlyphAlphabet::ordinal
//!         ▼
//!   u64 ordinal
//!         │ cram_address (Operator Consistency contract)
//!         ▼
//!   [u64; 6]  ← Safe Basis address
//!         │ BasisLift<const K>
//!         ▼
//!   [u64; K]  ← arbitrary-precise extension
//! ```
//!
//! Pixels never reach this module. The Object contract (NODE-CTR01)
//! firewall is enforced by the trait domain being a finite enumeration.

pub mod alphabet;
pub mod bardot;        // NODE-VT02 — BarDotNumeral
pub mod dayname;       // NODE-VT03 — DayNameGlyph
pub mod month;         // NODE-VT04 — MonthGlyph
pub mod iconographic;  // NODE-VT05 — IconographicFigure
pub mod layout;        // NODE-VT06 — PageLayout
pub mod lift;          // NODE-VT07 — BasisLift<const K>
pub mod wire;          // NODE-WIRE01..WIRE03 — adapters

#[cfg(test)] mod contracts;  // NODE-CTR01..CTR05 — Five-Contract gates
#[cfg(test)] mod fifth_op;   // NODE-FO01..FO02 — fifth-operator coherence

pub use alphabet::{GlyphAlphabet, SemanticRole, safe_address};
pub use bardot::BarDotNumeral;
pub use dayname::{DayNameGlyph, ALL_DAY_NAMES};
pub use month::{MonthGlyph, ALL_MONTHS};
pub use iconographic::{IconographicFigure, ALL_FIGURES};
pub use layout::{PageLayout, goddess_section_layout, cumulative_addresses, cumulative_totals};
pub use lift::{lift, SAFE_BASIS_K7, SAFE_BASIS_K8, SAFE_BASIS_K10, basis_product};
pub use wire::{page_intervals, verify_against_non_visual_pipeline, residue_pattern_from_layout};
//
// Tier 1+ (orchestrator-side):
// pub mod layout;        // NODE-VT06 — PageLayout
// pub mod lift;          // NODE-VT07 — BasisLift<const K>
// pub mod wire;          // NODE-WIRE01..03 — adapters
//
// Test-only:
// #[cfg(test)] mod contracts;  // NODE-CTR01..05
// #[cfg(test)] mod fifth_op;   // NODE-FO01..02
