//! # Five-Contract Gates (NODE-CTR01..CTR05)
//!
//! Test-only module enforcing the ns-continuum-bridge Five Contracts on
//! the H4 visual transducer:
//!
//! - **CTR01 — Object:** every implementor's domain is a finite enumeration.
//! - **CTR02 — Topology:** the codomain is discrete `[u64; K]`.
//! - **CTR03 — Uniformity:** every lane is bounded by its prime.
//! - **CTR04 — Operator Consistency** *(load-bearing)*: the transducer
//!   commutes with `cram_address` on every glyph.
//! - **CTR05 — Discharge:** the visual pipeline produces an H4 verdict
//!   not weaker than the non-visual pipeline.
//!
//! Each contract failure marks the affected node BLOCKED and stops
//! downstream advancement.

#![cfg(test)]

use super::alphabet::{GlyphAlphabet, SemanticRole};
use super::bardot::BarDotNumeral;
use super::dayname::{DayNameGlyph, ALL_DAY_NAMES};
use super::month::{MonthGlyph, ALL_MONTHS};
use super::iconographic::{IconographicFigure, ALL_FIGURES};
use super::layout::{goddess_section_layout, cumulative_addresses};
use super::wire::verify_against_non_visual_pipeline;
use super::lift::{lift, SAFE_BASIS_K7, SAFE_BASIS_K8, SAFE_BASIS_K10};
use dresden_codex::{SAFE_BASIS, cram_address, carry_bits};

// ─────────────────────────────────────────────────────────────────────
// CTR01 — Object contract
// ─────────────────────────────────────────────────────────────────────

#[test]
fn ctr01_object_contract_bardot_finite() {
    // BarDotNumeral has 20 valid values (0..=19). new() rejects > 19.
    let valid: Vec<_> = (0..=19).filter_map(|v| BarDotNumeral::new(v)).collect();
    assert_eq!(valid.len(), 20);
    assert!(BarDotNumeral::new(20).is_none());
    assert!(BarDotNumeral::new(255).is_none());
}

#[test]
fn ctr01_object_contract_dayname_finite() {
    assert_eq!(ALL_DAY_NAMES.len(), 20);
}

#[test]
fn ctr01_object_contract_month_finite() {
    assert_eq!(ALL_MONTHS.len(), 19);
}

#[test]
fn ctr01_object_contract_iconographic_finite() {
    assert_eq!(ALL_FIGURES.len(), 9);
}

// ─────────────────────────────────────────────────────────────────────
// CTR02 — Topology contract (discrete codomain)
// ─────────────────────────────────────────────────────────────────────
//
// Type-level: GlyphAlphabet::address returns [u64; K]. The Rust type
// system guarantees discreteness. We only need a smoke test confirming
// no path produces a float or NaN-coded sentinel.

#[test]
fn ctr02_topology_codomain_is_u64_array() {
    let g = BarDotNumeral::new(13).unwrap();
    let _addr: [u64; 6] = <BarDotNumeral as GlyphAlphabet<6>>::address(&g, &SAFE_BASIS);
    // If this compiles, the type is correct.
}

// ─────────────────────────────────────────────────────────────────────
// CTR03 — Uniformity contract (lane-bounded by prime)
// ─────────────────────────────────────────────────────────────────────

#[test]
fn ctr03_uniformity_dayname_all_lanes_bounded() {
    for &glyph in ALL_DAY_NAMES.iter() {
        let addr = <DayNameGlyph as GlyphAlphabet<6>>::address(&glyph, &SAFE_BASIS);
        for (i, &p) in SAFE_BASIS.iter().enumerate() {
            assert!(addr[i] < p, "DayName {:?} lane {} = {} not < {}", glyph, i, addr[i], p);
        }
    }
}

#[test]
fn ctr03_uniformity_month_all_lanes_bounded() {
    for &glyph in ALL_MONTHS.iter() {
        let addr = <MonthGlyph as GlyphAlphabet<6>>::address(&glyph, &SAFE_BASIS);
        for (i, &p) in SAFE_BASIS.iter().enumerate() {
            assert!(addr[i] < p);
        }
    }
}

#[test]
fn ctr03_uniformity_iconographic_all_lanes_bounded() {
    for &glyph in ALL_FIGURES.iter() {
        let addr = <IconographicFigure as GlyphAlphabet<6>>::address(&glyph, &SAFE_BASIS);
        for (i, &p) in SAFE_BASIS.iter().enumerate() {
            assert!(addr[i] < p);
        }
    }
}

#[test]
fn ctr03_uniformity_extended_basis() {
    // At K=10 the property must also hold.
    for x in [0u64, 19, 148, 1_448, 37_960] {
        let addr = lift::<10>(x, &SAFE_BASIS_K10);
        for (i, &p) in SAFE_BASIS_K10.iter().enumerate() {
            assert!(addr[i] < p);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────
// CTR04 — Operator Consistency contract (LOAD-BEARING)
// ─────────────────────────────────────────────────────────────────────
//
// For every glyph g: g.address(&SAFE_BASIS) == cram_address(g.ordinal()).
// This is the commutative-diagram closure between the new transducer
// and the canonical L1 CRAM operator. If any case fails, the
// transducer is hallucinating and downstream nodes are BLOCKED.

#[test]
fn ctr04_operator_consistency_bardot() {
    for v in 0..=19 {
        let g = BarDotNumeral::new(v).unwrap();
        let visual = <BarDotNumeral as GlyphAlphabet<6>>::address(&g, &SAFE_BASIS);
        let canonical = cram_address(<BarDotNumeral as GlyphAlphabet<6>>::ordinal(&g));
        assert_eq!(visual, canonical,
            "Operator Consistency failure at BarDotNumeral({}): visual={:?} canonical={:?}",
            v, visual, canonical);
    }
}

#[test]
fn ctr04_operator_consistency_dayname() {
    for &g in ALL_DAY_NAMES.iter() {
        let visual = <DayNameGlyph as GlyphAlphabet<6>>::address(&g, &SAFE_BASIS);
        let canonical = cram_address(<DayNameGlyph as GlyphAlphabet<6>>::ordinal(&g));
        assert_eq!(visual, canonical, "OC failure at DayName {:?}", g);
    }
}

#[test]
fn ctr04_operator_consistency_month() {
    for &g in ALL_MONTHS.iter() {
        let visual = <MonthGlyph as GlyphAlphabet<6>>::address(&g, &SAFE_BASIS);
        let canonical = cram_address(<MonthGlyph as GlyphAlphabet<6>>::ordinal(&g));
        assert_eq!(visual, canonical, "OC failure at Month {:?}", g);
    }
}

#[test]
fn ctr04_operator_consistency_iconographic() {
    for &g in ALL_FIGURES.iter() {
        let visual = <IconographicFigure as GlyphAlphabet<6>>::address(&g, &SAFE_BASIS);
        let canonical = cram_address(<IconographicFigure as GlyphAlphabet<6>>::ordinal(&g));
        assert_eq!(visual, canonical, "OC failure at Figure {:?}", g);
    }
}

#[test]
fn ctr04_operator_consistency_carry_bits_derived_consistently() {
    // The carry-bits derived from the visual address must equal the
    // canonical carry_bits of the ordinal. This extends OC from the
    // address itself to the per-lane activation pattern.
    for &g in ALL_FIGURES.iter() {
        let ord = <IconographicFigure as GlyphAlphabet<6>>::ordinal(&g);
        let addr = <IconographicFigure as GlyphAlphabet<6>>::address(&g, &SAFE_BASIS);
        let derived_carry: [u8; 6] = std::array::from_fn(|i| if addr[i] != 0 { 1 } else { 0 });
        assert_eq!(derived_carry, carry_bits(ord),
            "carry-bit derivation diverges at Figure {:?}", g);
    }
}

#[test]
fn ctr04_operator_consistency_extends_to_k7_k8() {
    // OC must also hold under the basis extension.
    let test_ordinals = [0u64, 1, 5, 11, 13, 17, 19, 148, 177];
    for &ord in test_ordinals.iter() {
        let k6 = cram_address(ord);
        let k7 = lift::<7>(ord, &SAFE_BASIS_K7);
        let k8 = lift::<8>(ord, &SAFE_BASIS_K8);
        for i in 0..6 {
            assert_eq!(k6[i], k7[i]);
            assert_eq!(k6[i], k8[i]);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────
// CTR05 — Discharge contract
// ─────────────────────────────────────────────────────────────────────
//
// The visual layer must not WEAKEN the non-visual H4 result. If the
// non-visual pipeline produced a verdict on the canonical Goddess
// section, the visual pipeline operating on the same underlying
// integers must reproduce that verdict exactly.

#[test]
fn ctr05_discharge_visual_matches_non_visual() {
    let layout = goddess_section_layout();
    verify_against_non_visual_pipeline(&layout)
        .expect("Discharge contract: visual pipeline must match non-visual exactly");
}

#[test]
fn ctr05_discharge_cumulative_endpoints_match() {
    use crate::moon_goddess::MoonGoddessProfile;
    let layout = goddess_section_layout();
    let cum = cumulative_addresses(&layout);
    let profile = MoonGoddessProfile::compute();
    assert_eq!(cum.last(), profile.page_cram_addresses.last(),
        "final cumulative address must match MoonGoddessProfile");
}
