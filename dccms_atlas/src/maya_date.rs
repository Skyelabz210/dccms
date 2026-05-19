//! # Maya-Date API (B-10 / Tier 3) — ergonomic Tzolk'in date type
//!
//! A typed Maya-calendar date as `(tone, day-name glyph)`. Wraps the
//! `engines::tzolkin::Tzolkin` constructors with a friendlier surface:
//! `from_day_number`, `to_day_number`, `advance_days`. No new
//! mathematical content — this is purely ergonomic, lifted in shape
//! from ASTRO21's `from_maya_date(tone, glyph)` API surface per
//! synthesis §3.5.
//!
//! Uses dccms's canonical modern Mayan orthography
//! (`h4_visual::dayname::DayNameGlyph`) — `Imix, IkPrime, Akbal, ...,
//! Ajaw` — not the vault's colonial spellings.

#![allow(dead_code)]

use crate::engines::tzolkin::Tzolkin;
use crate::h4_visual::alphabet::GlyphAlphabet;
use crate::h4_visual::dayname::{DayNameGlyph, from_ordinal};

/// Errors for Maya-date construction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MayaDateError {
    /// Tone outside 1..=13.
    ToneOutOfRange(u8),
}

/// A Maya Tzolk'in date as a tone (1..=13) and day-sign glyph.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MayaDate {
    pub tone: u8,
    pub glyph: DayNameGlyph,
}

impl MayaDate {
    /// Construct from a 1-indexed tone and a typed glyph.
    pub fn new(tone: u8, glyph: DayNameGlyph) -> Result<Self, MayaDateError> {
        if !(1..=13).contains(&tone) {
            return Err(MayaDateError::ToneOutOfRange(tone));
        }
        Ok(Self { tone, glyph })
    }

    /// Construct from a Tzolk'in absolute day number (0..=259, or any
    /// `u64` — wraps via the 260-day cycle).
    pub fn from_day_number(day: u64) -> Self {
        let tone_residue = day % 13;          // 0..=12
        let sign_residue = day % 20;          // 0..=19
        let glyph = from_ordinal(sign_residue)
            .expect("from_ordinal guaranteed for residue < 20");
        Self {
            tone: (tone_residue + 1) as u8,   // 1-indexed
            glyph,
        }
    }

    /// Absolute Tzolk'in day number (0..=259) for this date.
    pub fn to_day_number(&self) -> u64 {
        let tone_0 = (self.tone - 1) as u64;
        let glyph_ord = <DayNameGlyph as GlyphAlphabet<6>>::ordinal(&self.glyph);
        // CRT: x = (40·tone_0 + 221·glyph) mod 260.
        (tone_0 * 40 + glyph_ord * 221) % 260
    }

    /// Advance this date by `n` days, wrapping within the 260-day cycle.
    pub fn advance_days(&self, n: u64) -> Self {
        Self::from_day_number(self.to_day_number() + n)
    }

    /// Render as `<tone> <glyph-name>` using canonical modern Mayan orthography.
    pub fn display(&self) -> String {
        format!("{} {:?}", self.tone, self.glyph)
    }

    /// Get the equivalent Tzolk'in `MayaState` for fabric-level operations.
    pub fn as_tzolkin_state(&self) -> crate::engines::lane::MayaState {
        Tzolkin::from_day(self.to_day_number())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_validates_tone() {
        assert!(MayaDate::new(1, DayNameGlyph::Imix).is_ok());
        assert!(MayaDate::new(13, DayNameGlyph::Ajaw).is_ok());
        assert_eq!(MayaDate::new(0, DayNameGlyph::Imix),
            Err(MayaDateError::ToneOutOfRange(0)));
        assert_eq!(MayaDate::new(14, DayNameGlyph::Imix),
            Err(MayaDateError::ToneOutOfRange(14)));
    }

    #[test]
    fn day_zero_is_one_imix() {
        let d = MayaDate::from_day_number(0);
        assert_eq!(d.tone, 1);
        assert_eq!(d.glyph, DayNameGlyph::Imix);
    }

    #[test]
    fn round_trip_full_cycle() {
        for day in 0u64..260 {
            let date = MayaDate::from_day_number(day);
            let recovered = date.to_day_number();
            assert_eq!(day, recovered, "round-trip failed for day {}", day);
        }
    }

    #[test]
    fn advance_zero_is_identity() {
        let d = MayaDate::new(7, DayNameGlyph::Ix).unwrap();
        assert_eq!(d.advance_days(0), d);
    }

    #[test]
    fn advance_260_is_identity_modulo_cycle() {
        let d = MayaDate::from_day_number(100);
        let dd = d.advance_days(260);
        assert_eq!(d, dd);
    }

    #[test]
    fn equivalence_with_tzolkin_engine() {
        // For every day in 0..260, MayaDate's round-trip should agree
        // with the underlying engines::tzolkin::Tzolkin constructors.
        use crate::engines::tzolkin::Tzolkin;
        for day in 0u64..260 {
            let date = MayaDate::from_day_number(day);
            let from_engine = Tzolkin::from_day(day);
            assert_eq!(date.as_tzolkin_state(), from_engine,
                "MayaDate and Tzolkin disagree at day {}", day);
        }
    }

    #[test]
    fn display_uses_modern_orthography() {
        let d = MayaDate::from_day_number(0);
        assert_eq!(d.display(), "1 Imix");
        let d = MayaDate::from_day_number(1);
        assert_eq!(d.display(), "2 IkPrime");  // NOT colonial "Ik"
    }
}
