//! Iconographic figure alphabet for the H4 Goddess-section visual transducer.
//!
//! This alphabet gives one typed figure to each Dresden Codex Goddess-section
//! page, pages 16 through 24. The values are discrete glyph identities; pixel
//! extraction and layout sequencing remain separate transducer layers.
//!
//! ## Page 24 and the BlankBridge variant
//!
//! Per vault `Dresden Coprime.md` line 39, **page 24 contains no painted
//! iconography whatsoever — it is "a deliberate blank space [that] acts as
//! a physical and mathematical bridge"** between the Moon Goddess section
//! (pages 16-23) and the New Year ceremonies section (pages 25-28). The
//! `BlankBridge` variant of this alphabet represents that structural-blank
//! page. It carries `SemanticRole::Boundary` because a blank-bridge IS a
//! region demarcation in the codex's structural language. Its
//! `associated_interval` is 148 (the near eclipse half-year), maintaining
//! the 1448-day Goddess-section total without altering the H4 closure.

#![allow(dead_code)]

use super::alphabet::*;

/// One iconographic figure per Goddess-section page, Dresden pages 16-24.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IconographicFigure {
    /// Page 16: lunar disc / crescent.
    MoonSign,
    /// Page 17: Ix Chel as water-bearer.
    WaterPot,
    /// Page 18: weaving / textile attribute.
    WeavingShuttle,
    /// Page 19: serpent-coif identifying ornament.
    SnakeHeaddress,
    /// Page 20: eclipse mark for the 148/177 alternation.
    EclipseGlyph,
    /// Page 21: childbirth / midwifery attribute.
    BirthGlyph,
    /// Page 22: medicine / healing.
    HealingGlyph,
    /// Page 23: flood / overflow.
    FloodGlyph,
    /// Page 24: structural blank — no painted iconography. The deliberate
    /// blank space that closes the Moon Goddess section as a physical
    /// and mathematical bridge to pages 25-28 (New Year ceremonies).
    /// Carries `SemanticRole::Boundary`.
    BlankBridge,
}

/// All Goddess-section figures in codex page order, pages 16-24.
pub const ALL_FIGURES: [IconographicFigure; 9] = [
    IconographicFigure::MoonSign,
    IconographicFigure::WaterPot,
    IconographicFigure::WeavingShuttle,
    IconographicFigure::SnakeHeaddress,
    IconographicFigure::EclipseGlyph,
    IconographicFigure::BirthGlyph,
    IconographicFigure::HealingGlyph,
    IconographicFigure::FloodGlyph,
    IconographicFigure::BlankBridge,
];

/// Map a Dresden Codex page number to its Goddess-section figure.
pub fn from_page(page: u8) -> Option<IconographicFigure> {
    match page {
        16 => Some(IconographicFigure::MoonSign),
        17 => Some(IconographicFigure::WaterPot),
        18 => Some(IconographicFigure::WeavingShuttle),
        19 => Some(IconographicFigure::SnakeHeaddress),
        20 => Some(IconographicFigure::EclipseGlyph),
        21 => Some(IconographicFigure::BirthGlyph),
        22 => Some(IconographicFigure::HealingGlyph),
        23 => Some(IconographicFigure::FloodGlyph),
        24 => Some(IconographicFigure::BlankBridge),
        _ => None,
    }
}

impl IconographicFigure {
    /// Ordinal position in the Goddess-section figure alphabet.
    pub fn ordinal(&self) -> u64 {
        match self {
            IconographicFigure::MoonSign => 0,
            IconographicFigure::WaterPot => 1,
            IconographicFigure::WeavingShuttle => 2,
            IconographicFigure::SnakeHeaddress => 3,
            IconographicFigure::EclipseGlyph => 4,
            IconographicFigure::BirthGlyph => 5,
            IconographicFigure::HealingGlyph => 6,
            IconographicFigure::FloodGlyph => 7,
            IconographicFigure::BlankBridge => 8,
        }
    }

    /// Semantic role of this figure in the visual transducer.
    ///
    /// `EclipseGlyph` and `BlankBridge` carry `Boundary`. The eclipse mark
    /// demarcates the 148/177 half-year alternation; the blank bridge
    /// demarcates the Goddess-section closure (page 24). All other
    /// figures carry `Content`.
    pub fn semantic_role(&self) -> SemanticRole {
        match self {
            IconographicFigure::EclipseGlyph
            | IconographicFigure::BlankBridge => SemanticRole::Boundary,
            _ => SemanticRole::Content,
        }
    }

    /// Map a Dresden Codex page number to its Goddess-section figure.
    pub fn from_page(page: u8) -> Option<IconographicFigure> {
        from_page(page)
    }

    /// Return the page's associated eclipse half-year interval.
    ///
    /// Even ordinals 0, 2, 4, 6, 8 are associated with the near interval
    /// of 148 days; odd ordinals 1, 3, 5, 7 are associated with the far
    /// interval of 177 days.
    pub fn associated_interval(&self) -> u64 {
        // BlankBridge (page 24, ordinal 8, even) keeps the near-eclipse 148
        // value to preserve the canonical 1448-day total. The interpretation
        // is that the structural-blank page closes a 148-day near-eclipse
        // window into the next section.
        match self {
            IconographicFigure::MoonSign
            | IconographicFigure::WeavingShuttle
            | IconographicFigure::EclipseGlyph
            | IconographicFigure::HealingGlyph
            | IconographicFigure::BlankBridge => 148,
            IconographicFigure::WaterPot
            | IconographicFigure::SnakeHeaddress
            | IconographicFigure::BirthGlyph
            | IconographicFigure::FloodGlyph => 177,
        }
    }
}

impl GlyphAlphabet<6> for IconographicFigure {
    fn ordinal(&self) -> u64 {
        IconographicFigure::ordinal(self)
    }

    fn semantic_role(&self) -> SemanticRole {
        IconographicFigure::semantic_role(self)
    }
}

impl GlyphAlphabet<7> for IconographicFigure {
    fn ordinal(&self) -> u64 {
        IconographicFigure::ordinal(self)
    }

    fn semantic_role(&self) -> SemanticRole {
        IconographicFigure::semantic_role(self)
    }
}

impl GlyphAlphabet<8> for IconographicFigure {
    fn ordinal(&self) -> u64 {
        IconographicFigure::ordinal(self)
    }

    fn semantic_role(&self) -> SemanticRole {
        IconographicFigure::semantic_role(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::h4_non_visual::GODDESS_SECTION_INTERVALS;
    use dresden_codex::{cram_address, SAFE_BASIS};
    use std::collections::HashSet;

    #[test]
    fn enumeration_has_9_variants() {
        assert_eq!(ALL_FIGURES.len(), 9);
    }

    #[test]
    fn ordinals_0_through_8() {
        let ordinals: Vec<u64> = ALL_FIGURES
            .iter()
            .map(<IconographicFigure as GlyphAlphabet<6>>::ordinal)
            .collect();
        let distinct: HashSet<u64> = ordinals.iter().copied().collect();

        assert_eq!(distinct.len(), 9);
        assert!(ordinals.iter().all(|&ord| ord <= 8));
        for expected in 0..=8 {
            assert!(distinct.contains(&expected));
        }
    }

    #[test]
    fn boundary_figures_are_eclipse_and_blankbridge() {
        // EclipseGlyph and BlankBridge are the only Boundary figures.
        assert_eq!(
            IconographicFigure::EclipseGlyph.semantic_role(),
            SemanticRole::Boundary
        );
        assert_eq!(
            IconographicFigure::BlankBridge.semantic_role(),
            SemanticRole::Boundary
        );
        // All other figures are Content.
        for figure in ALL_FIGURES {
            let is_boundary_figure = matches!(
                figure,
                IconographicFigure::EclipseGlyph | IconographicFigure::BlankBridge
            );
            if !is_boundary_figure {
                assert_eq!(figure.semantic_role(), SemanticRole::Content);
            }
        }
    }

    #[test]
    fn page_24_is_blank_bridge() {
        // Vault Dresden Coprime line 39: page 24 is iconographically blank.
        // Our alphabet honors this by representing page 24 as BlankBridge
        // rather than a painted figure.
        assert_eq!(from_page(24), Some(IconographicFigure::BlankBridge));
        assert_eq!(
            IconographicFigure::BlankBridge.semantic_role(),
            SemanticRole::Boundary
        );
    }

    #[test]
    fn from_page_round_trip() {
        for page in 16..=24 {
            assert!(from_page(page).is_some());
        }

        assert_eq!(from_page(15), None);
        assert_eq!(from_page(25), None);
    }

    #[test]
    fn operator_consistency() {
        for figure in ALL_FIGURES {
            assert_eq!(
                figure.address(&SAFE_BASIS),
                cram_address(figure.ordinal())
            );
        }
    }

    #[test]
    fn associated_intervals_sum_to_1448() {
        let figure_sum: u64 = ALL_FIGURES
            .iter()
            .map(IconographicFigure::associated_interval)
            .sum();
        let interval_sum: u64 = GODDESS_SECTION_INTERVALS.iter().sum();

        assert_eq!(figure_sum, 1_448);
        assert_eq!(figure_sum, interval_sum);
    }
}
