//! Haab month glyph alphabet for the H4 visual transducer.

use super::alphabet::*;

/// Month glyphs for the 18 Haab winals plus the Wayeb boundary period.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MonthGlyph {
    /// Pop.
    Pop,
    /// Wo.
    Wo,
    /// Sip.
    Sip,
    /// Sotz'.
    Sotz,
    /// Sek.
    Sek,
    /// Xul.
    Xul,
    /// Yaxk'in.
    Yaxkin,
    /// Mol.
    Mol,
    /// Ch'en.
    Chen,
    /// Yax.
    Yax,
    /// Sak.
    Sak,
    /// Keh.
    Keh,
    /// Mak.
    Mak,
    /// K'ank'in.
    Kankin,
    /// Muwan.
    Muwan,
    /// Pax.
    Pax,
    /// K'ayab.
    Kayab,
    /// Kumk'u.
    Kumku,
    /// Wayeb, the five-day year-end boundary period.
    Wayeb,
}

/// All Haab month glyphs in ordinal order.
pub const ALL_MONTHS: [MonthGlyph; 19] = [
    MonthGlyph::Pop,
    MonthGlyph::Wo,
    MonthGlyph::Sip,
    MonthGlyph::Sotz,
    MonthGlyph::Sek,
    MonthGlyph::Xul,
    MonthGlyph::Yaxkin,
    MonthGlyph::Mol,
    MonthGlyph::Chen,
    MonthGlyph::Yax,
    MonthGlyph::Sak,
    MonthGlyph::Keh,
    MonthGlyph::Mak,
    MonthGlyph::Kankin,
    MonthGlyph::Muwan,
    MonthGlyph::Pax,
    MonthGlyph::Kayab,
    MonthGlyph::Kumku,
    MonthGlyph::Wayeb,
];

impl MonthGlyph {
    /// Ordinal position in the Haab month alphabet.
    pub fn ordinal(&self) -> u64 {
        match self {
            MonthGlyph::Pop => 0,
            MonthGlyph::Wo => 1,
            MonthGlyph::Sip => 2,
            MonthGlyph::Sotz => 3,
            MonthGlyph::Sek => 4,
            MonthGlyph::Xul => 5,
            MonthGlyph::Yaxkin => 6,
            MonthGlyph::Mol => 7,
            MonthGlyph::Chen => 8,
            MonthGlyph::Yax => 9,
            MonthGlyph::Sak => 10,
            MonthGlyph::Keh => 11,
            MonthGlyph::Mak => 12,
            MonthGlyph::Kankin => 13,
            MonthGlyph::Muwan => 14,
            MonthGlyph::Pax => 15,
            MonthGlyph::Kayab => 16,
            MonthGlyph::Kumku => 17,
            MonthGlyph::Wayeb => 18,
        }
    }

    /// Semantic role of this month glyph in the visual transducer.
    pub fn semantic_role(&self) -> SemanticRole {
        match self {
            MonthGlyph::Wayeb => SemanticRole::Boundary,
            _ => SemanticRole::Navigation,
        }
    }
}

/// Convert a Haab month ordinal into its glyph.
pub fn from_ordinal(n: u64) -> Option<MonthGlyph> {
    match n {
        0 => Some(MonthGlyph::Pop),
        1 => Some(MonthGlyph::Wo),
        2 => Some(MonthGlyph::Sip),
        3 => Some(MonthGlyph::Sotz),
        4 => Some(MonthGlyph::Sek),
        5 => Some(MonthGlyph::Xul),
        6 => Some(MonthGlyph::Yaxkin),
        7 => Some(MonthGlyph::Mol),
        8 => Some(MonthGlyph::Chen),
        9 => Some(MonthGlyph::Yax),
        10 => Some(MonthGlyph::Sak),
        11 => Some(MonthGlyph::Keh),
        12 => Some(MonthGlyph::Mak),
        13 => Some(MonthGlyph::Kankin),
        14 => Some(MonthGlyph::Muwan),
        15 => Some(MonthGlyph::Pax),
        16 => Some(MonthGlyph::Kayab),
        17 => Some(MonthGlyph::Kumku),
        18 => Some(MonthGlyph::Wayeb),
        _ => None,
    }
}

impl GlyphAlphabet<6> for MonthGlyph {
    fn ordinal(&self) -> u64 {
        MonthGlyph::ordinal(self)
    }

    fn semantic_role(&self) -> SemanticRole {
        MonthGlyph::semantic_role(self)
    }
}

impl GlyphAlphabet<7> for MonthGlyph {
    fn ordinal(&self) -> u64 {
        MonthGlyph::ordinal(self)
    }

    fn semantic_role(&self) -> SemanticRole {
        MonthGlyph::semantic_role(self)
    }
}

impl GlyphAlphabet<8> for MonthGlyph {
    fn ordinal(&self) -> u64 {
        MonthGlyph::ordinal(self)
    }

    fn semantic_role(&self) -> SemanticRole {
        MonthGlyph::semantic_role(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dresden_codex::{cram_address, SAFE_BASIS};

    #[test]
    fn enumeration_has_19_variants() {
        assert_eq!(ALL_MONTHS.len(), 19);
    }

    #[test]
    fn ordinals_match_position() {
        for (i, month) in ALL_MONTHS.iter().enumerate() {
            assert_eq!(month.ordinal(), i as u64);
        }
    }

    #[test]
    fn semantic_roles_correct() {
        for month in ALL_MONTHS.iter().take(18) {
            assert_eq!(month.semantic_role(), SemanticRole::Navigation);
        }
        assert_eq!(MonthGlyph::Wayeb.semantic_role(), SemanticRole::Boundary);
    }

    #[test]
    fn from_ordinal_round_trip() {
        for (i, month) in ALL_MONTHS.iter().enumerate() {
            assert_eq!(from_ordinal(i as u64), Some(*month));
        }
        assert_eq!(from_ordinal(19), None);
    }

    #[test]
    fn operator_consistency() {
        for month in ALL_MONTHS {
            assert_eq!(
                <MonthGlyph as GlyphAlphabet<6>>::address(&month, &SAFE_BASIS),
                cram_address(month.ordinal())
            );
        }
    }

    #[test]
    fn wayeb_ordinal_is_18() {
        assert_eq!(MonthGlyph::Wayeb.ordinal(), 18);
    }
}
