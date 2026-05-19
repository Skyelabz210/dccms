//! # Extended Goddess Decoder Framework (C-3)
//!
//! Per executioner Directive 0: convert the apparent limit "needs
//! Barnhart 2005 source material" into a build target. The framework
//! that **consumes** glyph-spec data is buildable now; when the actual
//! Barnhart specs arrive, they plug in as data, not as new code.
//!
//! Pages 13c-15 (per vault `Dresden.md` Moon Goddess audit) belong to
//! the canonical extended Moon Goddess range. The H4 visual transducer
//! currently covers 16-23 (with page 24 as BlankBridge). This module
//! extends the consumption-side type vocabulary so future glyph specs
//! drop in cleanly.

#![allow(dead_code)]

use crate::h4_visual::dayname::DayNameGlyph;
use dresden_codex::cram_address;

/// The four typographic registers within a single codex page (a, b, c, d
/// from top to bottom). Page references in Maya scholarship use
/// "13c" to mean the c-register of page 13.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PageRegister { A, B, C, D }

/// Kinds of glyphs that can appear in a codex page spec.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GlyphKind {
    /// A 20-day Tzolk'in glyph.
    DayName(DayNameGlyph),
    /// A bar-and-dot numeral (0..=19).
    Numeral(u8),
    /// An iconographic figure with a textual identifier — typed handle
    /// for downstream resolution against an actual figure alphabet.
    NamedFigure(&'static str),
    /// A blank or eroded position.
    Blank,
}

/// A single glyph occurrence on a specific page+register+position.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GlyphSpec {
    pub page: u8,
    pub register: PageRegister,
    pub position: u8,
    pub kind: GlyphKind,
}

/// Result of analyzing a glyph-spec sequence for the extended Goddess section.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtendedSectionAnalysis {
    /// All specs sorted by (page, register, position).
    pub specs_sorted: Vec<GlyphSpec>,
    /// Numerals extracted, in spec order.
    pub numerals: Vec<u8>,
    /// Sum of all numerals.
    pub numeral_sum: u64,
    /// Cumulative CRAM address after consuming the numerals.
    pub cumulative_cram: [u64; 6],
    /// Pages touched by any spec.
    pub pages_covered: Vec<u8>,
    /// Whether the spec set is empty (no data yet — typed sentinel).
    pub empty: bool,
}

/// Decode a sequence of glyph specs into a structural analysis.
///
/// Never panics on empty input — returns an `ExtendedSectionAnalysis`
/// with `empty: true` and zeroed sums. The Barnhart specs, when they
/// arrive, will populate the same shape.
pub fn decode_extended_section(specs: &[GlyphSpec]) -> ExtendedSectionAnalysis {
    if specs.is_empty() {
        return ExtendedSectionAnalysis {
            specs_sorted: Vec::new(),
            numerals: Vec::new(),
            numeral_sum: 0,
            cumulative_cram: [0u64; 6],
            pages_covered: Vec::new(),
            empty: true,
        };
    }
    let mut sorted: Vec<GlyphSpec> = specs.to_vec();
    sorted.sort_by_key(|g| (g.page, g.register as u8, g.position));

    let numerals: Vec<u8> = sorted.iter().filter_map(|g| match &g.kind {
        GlyphKind::Numeral(n) => Some(*n),
        _ => None,
    }).collect();
    let numeral_sum: u64 = numerals.iter().map(|&n| n as u64).sum();
    let cumulative_cram = cram_address(numeral_sum);

    let mut pages_covered: Vec<u8> = sorted.iter().map(|g| g.page).collect();
    pages_covered.sort_unstable();
    pages_covered.dedup();

    ExtendedSectionAnalysis {
        specs_sorted: sorted,
        numerals,
        numeral_sum,
        cumulative_cram,
        pages_covered,
        empty: false,
    }
}

/// Convenience: a placeholder spec set for the un-decoded extended
/// range pages [13, 14, 15] tagged as `Blank` glyphs. When Barnhart
/// specs arrive, this function's caller substitutes the real glyphs.
pub fn placeholder_extended_specs() -> Vec<GlyphSpec> {
    let mut out = Vec::new();
    for page in [13u8, 14, 15] {
        for reg in [PageRegister::A, PageRegister::B, PageRegister::C] {
            out.push(GlyphSpec {
                page,
                register: reg,
                position: 0,
                kind: GlyphKind::Blank,
            });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_yields_typed_empty_response() {
        let r = decode_extended_section(&[]);
        assert!(r.empty);
        assert_eq!(r.numeral_sum, 0);
        assert_eq!(r.cumulative_cram, [0u64; 6]);
        assert!(r.pages_covered.is_empty());
    }

    #[test]
    fn placeholder_specs_have_correct_shape() {
        let specs = placeholder_extended_specs();
        assert_eq!(specs.len(), 9);
        let pages: std::collections::BTreeSet<u8> = specs.iter().map(|g| g.page).collect();
        assert_eq!(pages.into_iter().collect::<Vec<_>>(), vec![13u8, 14, 15]);
        for s in &specs {
            assert!(matches!(s.kind, GlyphKind::Blank));
        }
    }

    #[test]
    fn numerical_glyphs_accumulate() {
        let specs = vec![
            GlyphSpec { page: 14, register: PageRegister::A, position: 0,
                        kind: GlyphKind::Numeral(13) },
            GlyphSpec { page: 14, register: PageRegister::A, position: 1,
                        kind: GlyphKind::Numeral(8) },
        ];
        let r = decode_extended_section(&specs);
        assert_eq!(r.numeral_sum, 21);
        assert_eq!(r.cumulative_cram, cram_address(21));
        assert!(!r.empty);
    }

    #[test]
    fn specs_sort_by_page_then_register_then_position() {
        let specs = vec![
            GlyphSpec { page: 15, register: PageRegister::A, position: 0, kind: GlyphKind::Blank },
            GlyphSpec { page: 13, register: PageRegister::C, position: 0, kind: GlyphKind::Blank },
            GlyphSpec { page: 13, register: PageRegister::A, position: 0, kind: GlyphKind::Blank },
            GlyphSpec { page: 14, register: PageRegister::B, position: 2, kind: GlyphKind::Blank },
            GlyphSpec { page: 14, register: PageRegister::B, position: 1, kind: GlyphKind::Blank },
        ];
        let r = decode_extended_section(&specs);
        // Expected order: (13,A), (13,C), (14,B,1), (14,B,2), (15,A).
        assert_eq!(r.specs_sorted[0].page, 13);
        assert_eq!(r.specs_sorted[0].register, PageRegister::A);
        assert_eq!(r.specs_sorted[1].page, 13);
        assert_eq!(r.specs_sorted[1].register, PageRegister::C);
        assert_eq!(r.specs_sorted[3].position, 2);
    }

    #[test]
    fn pages_covered_dedups_and_sorts() {
        let specs = vec![
            GlyphSpec { page: 13, register: PageRegister::A, position: 0, kind: GlyphKind::Blank },
            GlyphSpec { page: 13, register: PageRegister::B, position: 0, kind: GlyphKind::Blank },
            GlyphSpec { page: 15, register: PageRegister::A, position: 0, kind: GlyphKind::Blank },
        ];
        let r = decode_extended_section(&specs);
        assert_eq!(r.pages_covered, vec![13u8, 15]);
    }
}
