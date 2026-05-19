use super::alphabet::*;

/// Tzolk'in day-name glyphs in canonical 20-day order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DayNameGlyph {
    /// Imix
    Imix,
    /// Ik'
    IkPrime,
    /// Akb'al
    Akbal,
    /// Kan
    Kan,
    /// Chikchan
    Chikchan,
    /// Kimi
    Kimi,
    /// Manik'
    Manik,
    /// Lamat
    Lamat,
    /// Muluk
    Muluk,
    /// Ok
    Ok,
    /// Chuwen
    Chuwen,
    /// Eb
    Eb,
    /// Ben
    Ben,
    /// Ix
    Ix,
    /// Men
    Men,
    /// Kib
    Kib,
    /// Kaban
    Kaban,
    /// Etz'nab
    Etznab,
    /// Kawak
    Kawak,
    /// Ajaw
    Ajaw,
}

pub const ALL_DAY_NAMES: [DayNameGlyph; 20] = [
    DayNameGlyph::Imix,
    DayNameGlyph::IkPrime,
    DayNameGlyph::Akbal,
    DayNameGlyph::Kan,
    DayNameGlyph::Chikchan,
    DayNameGlyph::Kimi,
    DayNameGlyph::Manik,
    DayNameGlyph::Lamat,
    DayNameGlyph::Muluk,
    DayNameGlyph::Ok,
    DayNameGlyph::Chuwen,
    DayNameGlyph::Eb,
    DayNameGlyph::Ben,
    DayNameGlyph::Ix,
    DayNameGlyph::Men,
    DayNameGlyph::Kib,
    DayNameGlyph::Kaban,
    DayNameGlyph::Etznab,
    DayNameGlyph::Kawak,
    DayNameGlyph::Ajaw,
];

pub fn from_ordinal(n: u64) -> Option<DayNameGlyph> {
    match n {
        0 => Some(DayNameGlyph::Imix),
        1 => Some(DayNameGlyph::IkPrime),
        2 => Some(DayNameGlyph::Akbal),
        3 => Some(DayNameGlyph::Kan),
        4 => Some(DayNameGlyph::Chikchan),
        5 => Some(DayNameGlyph::Kimi),
        6 => Some(DayNameGlyph::Manik),
        7 => Some(DayNameGlyph::Lamat),
        8 => Some(DayNameGlyph::Muluk),
        9 => Some(DayNameGlyph::Ok),
        10 => Some(DayNameGlyph::Chuwen),
        11 => Some(DayNameGlyph::Eb),
        12 => Some(DayNameGlyph::Ben),
        13 => Some(DayNameGlyph::Ix),
        14 => Some(DayNameGlyph::Men),
        15 => Some(DayNameGlyph::Kib),
        16 => Some(DayNameGlyph::Kaban),
        17 => Some(DayNameGlyph::Etznab),
        18 => Some(DayNameGlyph::Kawak),
        19 => Some(DayNameGlyph::Ajaw),
        _ => None,
    }
}

macro_rules! impl_day_name_glyph_alphabet {
    ($k:expr) => {
        impl GlyphAlphabet<$k> for DayNameGlyph {
            fn ordinal(&self) -> u64 {
                match self {
                    DayNameGlyph::Imix => 0,
                    DayNameGlyph::IkPrime => 1,
                    DayNameGlyph::Akbal => 2,
                    DayNameGlyph::Kan => 3,
                    DayNameGlyph::Chikchan => 4,
                    DayNameGlyph::Kimi => 5,
                    DayNameGlyph::Manik => 6,
                    DayNameGlyph::Lamat => 7,
                    DayNameGlyph::Muluk => 8,
                    DayNameGlyph::Ok => 9,
                    DayNameGlyph::Chuwen => 10,
                    DayNameGlyph::Eb => 11,
                    DayNameGlyph::Ben => 12,
                    DayNameGlyph::Ix => 13,
                    DayNameGlyph::Men => 14,
                    DayNameGlyph::Kib => 15,
                    DayNameGlyph::Kaban => 16,
                    DayNameGlyph::Etznab => 17,
                    DayNameGlyph::Kawak => 18,
                    DayNameGlyph::Ajaw => 19,
                }
            }

            fn semantic_role(&self) -> SemanticRole {
                SemanticRole::Navigation
            }
        }
    };
}

impl_day_name_glyph_alphabet!(6);
impl_day_name_glyph_alphabet!(7);
impl_day_name_glyph_alphabet!(8);

#[cfg(test)]
mod tests {
    use super::*;
    use dresden_codex::{active_lanes, cram_address, SAFE_BASIS};

    #[test]
    fn enumeration_has_20_variants() {
        assert_eq!(ALL_DAY_NAMES.len(), 20);
    }

    #[test]
    fn ordinals_match_position() {
        for (i, glyph) in ALL_DAY_NAMES.iter().enumerate() {
            assert_eq!(<DayNameGlyph as GlyphAlphabet<6>>::ordinal(glyph), i as u64);
        }
    }

    #[test]
    fn semantic_role_navigation() {
        for glyph in ALL_DAY_NAMES {
            assert_eq!(
                <DayNameGlyph as GlyphAlphabet<6>>::semantic_role(&glyph),
                SemanticRole::Navigation
            );
        }
    }

    #[test]
    fn from_ordinal_round_trip() {
        for n in 0..20 {
            let glyph = from_ordinal(n).unwrap();
            assert_eq!(<DayNameGlyph as GlyphAlphabet<6>>::ordinal(&glyph), n);
        }
        assert_eq!(from_ordinal(20), None);
    }

    #[test]
    fn operator_consistency() {
        for glyph in ALL_DAY_NAMES {
            let ordinal = <DayNameGlyph as GlyphAlphabet<6>>::ordinal(&glyph);
            assert_eq!(
                <DayNameGlyph as GlyphAlphabet<6>>::address(&glyph, &SAFE_BASIS),
                cram_address(ordinal)
            );
        }
    }

    #[test]
    fn fifth_operator_p11_active_except_at_lane_zeros() {
        // Lane 11 nullifies exactly when ordinal % 11 == 0.
        // In the 20-day cycle this happens at ordinals {0, 11}:
        //   - Imix (0)  — trivial zero (all lanes nullified)
        //   - Eb (11)   — the unique non-trivial lane-11 zero of the cycle
        //
        // Eb is therefore the fifth-operator reset point inside the
        // day-name cycle — the navigation coordinate's structural midpoint,
        // NOT the cycle endpoint (Ajaw=19). This is intrinsic to the
        // QMNF/CRAM Safe Basis acting on the 20-element Tzolk'in alphabet.
        for glyph in ALL_DAY_NAMES {
            let ordinal = <DayNameGlyph as GlyphAlphabet<6>>::ordinal(&glyph);
            let expected_active = ordinal % 11 != 0;
            let actually_active = active_lanes(ordinal).contains(&11);
            assert_eq!(
                actually_active, expected_active,
                "lane-11 activity mismatch at ordinal {} ({:?})",
                ordinal, glyph
            );
        }
        // Positive assertion: Eb is the unique non-trivial lane-11 zero.
        let eb_ord = <DayNameGlyph as GlyphAlphabet<6>>::ordinal(&DayNameGlyph::Eb);
        assert_eq!(eb_ord, 11);
        assert!(!active_lanes(eb_ord).contains(&11));
    }

    /// Eb (ordinal 11) is the unique non-trivial lane-11 zero of the
    /// Tzolk'in day-name cycle. Provided as a documented constant for
    /// downstream consumers (NODE-FO01).
    #[allow(dead_code)]
    pub const LANE_11_ZERO_DAY: DayNameGlyph = DayNameGlyph::Eb;
}
