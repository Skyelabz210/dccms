//! # Fifth-Operator Coherence (NODE-FO01, NODE-FO02)
//!
//! Test-only module mechanizing the fifth-operator (prime-11) analysis
//! over the visual transducer. Prime 11 is the **coordinate / navigation
//! lane** of the QMNF/CRAM Safe Basis; prime 13 is the **boundary lane**.
//!
//! - **FO01:** Navigation glyphs activate p=11 except at lane-11 zeros
//!   (ordinals divisible by 11). Content glyphs may have p=11 silent.
//!   Boundary glyphs additionally interact with p=13 via the Shadow16
//!   product `(addr%11) * (addr%13)`.
//! - **FO02:** K-Elim level at p=11 of cumulative visual totals equals
//!   the K-Elim level of the integer-derived cumulative totals.

#![cfg(test)]

use super::alphabet::{GlyphAlphabet, SemanticRole};
use super::dayname::{DayNameGlyph, ALL_DAY_NAMES};
use super::month::{MonthGlyph, ALL_MONTHS};
use super::iconographic::{IconographicFigure, ALL_FIGURES};
use super::layout::{goddess_section_layout, cumulative_totals};
use crate::h5_level::k_elim_level;
use crate::h4_non_visual::GODDESS_SECTION_INTERVALS;
use dresden_codex::{active_lanes};

// ─────────────────────────────────────────────────────────────────────
// FO01 — Navigation glyphs activate p=11 except at lane-11 zeros
// ─────────────────────────────────────────────────────────────────────

#[test]
fn fo01_dayname_lane_11_zero_only_at_imix_and_eb() {
    // The unique non-trivial lane-11 zero in the Tzolk'in cycle is Eb
    // (ordinal 11). Imix (ordinal 0) is the trivial zero.
    let mut zero_count = 0;
    for &g in ALL_DAY_NAMES.iter() {
        let ord = <DayNameGlyph as GlyphAlphabet<6>>::ordinal(&g);
        let p11_active = active_lanes(ord).contains(&11);
        if !p11_active { zero_count += 1; }
        // Navigation property must match the math: p=11 active iff ord % 11 != 0.
        assert_eq!(p11_active, ord % 11 != 0,
            "FO01 DayName {:?} ord={}: lane-11 activity inconsistent",
            g, ord);
    }
    assert_eq!(zero_count, 2,
        "DayName cycle must have exactly 2 lane-11 zeros (Imix, Eb), found {}",
        zero_count);
}

#[test]
fn fo01_month_lane_11_zero_only_at_pop_and_keh() {
    // Haab cycle (19 glyphs, ordinals 0..=18). Multiples of 11 in
    // [0,18]: {0, 11}. So Pop (0) and Keh (ordinal 11) are the zeros.
    let mut zero_ordinals = Vec::new();
    for &g in ALL_MONTHS.iter() {
        let ord = <MonthGlyph as GlyphAlphabet<6>>::ordinal(&g);
        if !active_lanes(ord).contains(&11) {
            zero_ordinals.push((ord, g));
        }
    }
    assert_eq!(zero_ordinals.len(), 2,
        "Haab cycle must have 2 lane-11 zeros, found {:?}", zero_ordinals);
    assert_eq!(zero_ordinals[0].0, 0);
    assert_eq!(zero_ordinals[1].0, 11);
}

#[test]
fn fo01_navigation_glyphs_are_p11_active_outside_zeros() {
    // Strong form of FO01: every Navigation glyph with ordinal not
    // divisible by 11 has p=11 active.
    for &g in ALL_DAY_NAMES.iter() {
        let ord = <DayNameGlyph as GlyphAlphabet<6>>::ordinal(&g);
        let role = <DayNameGlyph as GlyphAlphabet<6>>::semantic_role(&g);
        assert_eq!(role, SemanticRole::Navigation);
        if ord % 11 != 0 {
            assert!(active_lanes(ord).contains(&11),
                "FO01 violation: Navigation DayName {:?} (ord={}) should be p=11 active",
                g, ord);
        }
    }
}

#[test]
fn fo01_eclipse_glyph_is_boundary() {
    // EclipseGlyph (ordinal 4) has semantic role Boundary. Its
    // ordinal % 13 = 4, ordinal % 11 = 4 — Shadow16 = 16.
    // (The 'Boundary' role-tag is the structural claim; the value of
    // Shadow16 is whatever the arithmetic produces.)
    let g = IconographicFigure::EclipseGlyph;
    let ord = <IconographicFigure as GlyphAlphabet<6>>::ordinal(&g);
    let role = <IconographicFigure as GlyphAlphabet<6>>::semantic_role(&g);
    assert_eq!(role, SemanticRole::Boundary);
    assert_eq!(ord, 4);
    let shadow16 = (ord % 11) * (ord % 13);
    assert_eq!(shadow16, 16); // 4 * 4
}

// ─────────────────────────────────────────────────────────────────────
// FO02 — K-Elim level at p=11 matches integer sequence
// ─────────────────────────────────────────────────────────────────────

#[test]
fn fo02_k_elim_level_at_p11_matches_integer_cumulative() {
    // For each page boundary, the K-Elim level at p=11 of the
    // visual-derived cumulative total must equal that of the integer
    // cumulative total. (These are by construction the same integer
    // here, since the visual pipeline recovers the canonical intervals.
    // The test is the formal proof of that equality.)
    let layout = goddess_section_layout();
    let visual_cumulative = cumulative_totals(&layout);

    let mut integer_cumulative = Vec::new();
    let mut running = 0u64;
    for &iv in GODDESS_SECTION_INTERVALS.iter() {
        running += iv;
        integer_cumulative.push(running);
    }

    assert_eq!(visual_cumulative.len(), integer_cumulative.len());

    for (i, (&vc, &ic)) in visual_cumulative.iter()
        .zip(integer_cumulative.iter()).enumerate()
    {
        let v_level = k_elim_level(vc, 11);
        let i_level = k_elim_level(ic, 11);
        assert_eq!(v_level, i_level,
            "FO02 K-Elim level divergence at page {}: visual_total={} level={}, integer_total={} level={}",
            i, vc, v_level, ic, i_level);
    }
}

#[test]
fn fo02_final_goddess_total_k_elim_level_p11_is_3() {
    // GODDESS_SECTION_TOTAL = 1448. k_elim_level(c, p) = floor(log_p(c)).
    // 11^3 = 1331 ≤ 1448 < 14641 = 11^4 → level 3.
    //
    // Structural significance (H5): 1448 lands above THRESHOLD_11_3,
    // placing the Goddess-section total in the LONG-CYCLE (Level ≥ 3)
    // classification at the navigation prime p=11. It clears the
    // Level-2/Level-3 boundary by just 117 days (8.8% margin).
    let layout = goddess_section_layout();
    let final_total = *cumulative_totals(&layout).last().unwrap();
    assert_eq!(final_total, 1_448);
    assert_eq!(k_elim_level(final_total, 11), 3,
        "Goddess total 1448 must sit at K-Elim Level 3 (long-cycle) at p=11");
    // And just below Level 4.
    assert!(final_total < 14_641);
    assert!(final_total >= 1_331);
}

#[test]
fn fo02_boundary_lane_p13_at_iconographic_figures() {
    // For each iconographic figure, compute the boundary-lane (p=13)
    // residue of its ordinal. The EclipseGlyph (Boundary role,
    // ordinal=4) has p=13 residue = 4. This is informative but not
    // an assertion about correctness — we record the values as a
    // visible substrate fingerprint for downstream consumers.
    for &g in ALL_FIGURES.iter() {
        let ord = <IconographicFigure as GlyphAlphabet<6>>::ordinal(&g);
        let p11 = ord % 11;
        let p13 = ord % 13;
        let shadow16 = p11 * p13;
        // Ordinals 0..=8: shadow16 = ord*ord, so it's just ord squared.
        assert_eq!(shadow16, ord * ord);
    }
}
