//! # Tzolkin — Engine 2 (B-7.2)
//!
//! Sacred Maya calendar as the CRT product `ℤ/13 × ℤ/20`. The two lanes
//! are heterogeneous: the tone lane (mod 13) tracks one phenomenon (the
//! consciousness / energy state in traditional readings), the day-sign
//! lane (mod 20) tracks another (identity / archetype). CRT fusion gives
//! 260 = 13 · 20 unique combined states.
//!
//! ## Lane structure
//!
//! - **Tone** (mod 13): `α(13) = 7`, `π(13) = 28`. Every 7th Fibonacci
//!   number is divisible by 13.
//! - **Sign** (mod 20): `π(20) = 60` (= lcm of π(4)=6 and π(5)=20).
//!   The 20 day-signs are typed as [`DayNameGlyph`] — canonical modern
//!   Mayan orthography.
//!
//! ## Bridge to existing `dayname.rs`
//!
//! This module does **not** introduce a parallel day-name spelling set
//! (per plan §9 D-3). It re-uses `dccms_atlas::h4_visual::dayname::DayNameGlyph`
//! and [`ALL_DAY_NAMES`] for the canonical 20-element alphabet. The
//! vault's `Mayas Engine.md` used colonial-orthography spellings
//! (Ik, Cimi, Oc, Ahau, …); dccms uses the post-1980s scholarly
//! standard (IkPrime, Kimi, Ok, Ajaw, …). The latter is the codebase
//! canonical and is preserved here.
//!
//! ## Omitted from the vault spec
//!
//! - `Tzolkin::above_consciousness_threshold` — the φ³-threshold framing
//!   is synthesis O4, do-not-mechanize.
//!
//! Source: vault `Mayas Engine.md` §ENGINE 2, adapted.

#![allow(dead_code)]

use super::lane::{Lane, MayaState, MayaStateError};
use super::pisano::{fibonacci_mod_sequence, PisanoError};
use crate::h4_visual::dayname::{DayNameGlyph, from_ordinal};
#[cfg(test)]
use crate::h4_visual::dayname::ALL_DAY_NAMES;

/// Errors for Tzolkin operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TzolkinError {
    /// Tone outside the valid 1..=13 range.
    ToneOutOfRange(u64),
    /// Day-sign ordinal outside the 0..=19 range.
    GlyphOutOfRange(u64),
    /// State has wrong lane structure.
    MalformedState,
    /// Pisano helper failed.
    Pisano(PisanoError),
    /// Underlying `MayaState` constructor failed.
    State(MayaStateError),
}

impl From<MayaStateError> for TzolkinError {
    fn from(e: MayaStateError) -> Self { TzolkinError::State(e) }
}

impl From<PisanoError> for TzolkinError {
    fn from(e: PisanoError) -> Self { TzolkinError::Pisano(e) }
}

/// Engine 2: Tzolkin sacred calendar fabric over `ℤ/13 × ℤ/20`.
pub struct Tzolkin;

impl Tzolkin {
    /// Tone lane: mod 13, consciousness / energy state.
    pub const TONE_LANE: Lane = Lane {
        name: "tone",
        modulus: 13,
        domain: "consciousness",
    };

    /// Sign lane: mod 20, day-sign identity / archetype.
    pub const SIGN_LANE: Lane = Lane {
        name: "sign",
        modulus: 20,
        domain: "identity",
    };

    /// Construct from 1-indexed tone (1..=13) and 0-indexed glyph ordinal (0..=19).
    pub fn new(tone: u64, glyph_ordinal: u64) -> Result<MayaState, TzolkinError> {
        if !(1..=13).contains(&tone) {
            return Err(TzolkinError::ToneOutOfRange(tone));
        }
        if glyph_ordinal >= 20 {
            return Err(TzolkinError::GlyphOutOfRange(glyph_ordinal));
        }
        let residues = vec![(tone - 1) % 13, glyph_ordinal % 20];
        let lanes = vec![Self::TONE_LANE, Self::SIGN_LANE];
        Ok(MayaState::new(residues, lanes)?)
    }

    /// Construct from a typed [`DayNameGlyph`] plus a tone.
    pub fn from_glyph(tone: u64, glyph: DayNameGlyph) -> Result<MayaState, TzolkinError> {
        // DayNameGlyph::ordinal goes via the GlyphAlphabet<6> impl which has
        // its own trait import path.
        use crate::h4_visual::alphabet::GlyphAlphabet;
        let glyph_ordinal = <DayNameGlyph as GlyphAlphabet<6>>::ordinal(&glyph);
        Self::new(tone, glyph_ordinal)
    }

    /// Construct from absolute Tzolk'in day number 0..=259.
    ///
    /// Reduces modulo the cycle: any u64 input yields a valid state.
    pub fn from_day(day: u64) -> MayaState {
        let residues = vec![day % 13, day % 20];
        let lanes = vec![Self::TONE_LANE, Self::SIGN_LANE];
        // Bound by construction: residues are reductions modulo the lane moduli.
        MayaState::new(residues, lanes).expect("from_day residues bounded by construction")
    }

    /// Validate that `state` has the canonical Tzolkin lane structure.
    pub fn validate(state: &MayaState) -> Result<(), TzolkinError> {
        if state.lane_count() != 2
            || state.lanes()[0] != Self::TONE_LANE
            || state.lanes()[1] != Self::SIGN_LANE
        {
            return Err(TzolkinError::MalformedState);
        }
        Ok(())
    }

    /// Reconstruct the absolute Tzolk'in day 0..=259 from a state via CRT.
    ///
    /// `x ≡ tone_0 (mod 13), x ≡ sign (mod 20)` with `inv(20, 13) = 2`
    /// (since `20 ≡ 7 mod 13` and `7·2 = 14 ≡ 1 mod 13`) and
    /// `inv(13, 20) = 17` (since `13·17 = 221 ≡ 1 mod 20`). So
    /// `x = (tone_0 · 20 · 2 + sign · 13 · 17) mod 260
    ///    = (40·tone_0 + 221·sign) mod 260`.
    pub fn to_day_number(state: &MayaState) -> Result<u64, TzolkinError> {
        Self::validate(state)?;
        let tone_0 = state.residues()[0];  // 0..=12
        let sign = state.residues()[1];    // 0..=19
        Ok((tone_0 * 40 + sign * 221) % 260)
    }

    /// Render as `"<tone> <glyph-name>"` using canonical modern Mayan
    /// orthography (via `DayNameGlyph`'s `Debug` rendering).
    pub fn display(state: &MayaState) -> Result<String, TzolkinError> {
        Self::validate(state)?;
        let tone = state.residues()[0] + 1;  // 1-indexed for display
        let sign_idx = state.residues()[1];
        let glyph = from_ordinal(sign_idx)
            .ok_or(TzolkinError::GlyphOutOfRange(sign_idx))?;
        Ok(format!("{} {:?}", tone, glyph))
    }

    /// Fibonacci-mod-13 sequence — the tone lane's intrinsic Fibonacci
    /// pattern. Length = `π(13) = 28`.
    pub fn fibonacci_tone_pattern() -> Result<Vec<u64>, TzolkinError> {
        Ok(fibonacci_mod_sequence(13)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::h4_visual::alphabet::GlyphAlphabet;

    #[test]
    fn new_validates_tone_range() {
        assert_eq!(Tzolkin::new(0, 0), Err(TzolkinError::ToneOutOfRange(0)));
        assert_eq!(Tzolkin::new(14, 0), Err(TzolkinError::ToneOutOfRange(14)));
    }

    #[test]
    fn new_validates_glyph_range() {
        assert_eq!(Tzolkin::new(1, 20), Err(TzolkinError::GlyphOutOfRange(20)));
        assert_eq!(Tzolkin::new(1, 255), Err(TzolkinError::GlyphOutOfRange(255)));
    }

    #[test]
    fn from_day_round_trip_full_cycle() {
        // 260 days, each producing a unique residue pair, all round-tripping.
        let mut seen: std::collections::HashSet<(u64, u64)> = std::collections::HashSet::new();
        for day in 0u64..260 {
            let state = Tzolkin::from_day(day);
            let pair = (state.residues()[0], state.residues()[1]);
            assert!(seen.insert(pair), "duplicate residue pair at day {}", day);
            let recovered = Tzolkin::to_day_number(&state).unwrap();
            assert_eq!(day, recovered, "round-trip failed for day {}", day);
        }
        assert_eq!(seen.len(), 260);
    }

    #[test]
    fn from_glyph_bridges_to_dayname() {
        // Imix (ordinal 0) at tone 1 → day 0.
        let state = Tzolkin::from_glyph(1, DayNameGlyph::Imix).unwrap();
        assert_eq!(state.residues(), &[0u64, 0]);
        assert_eq!(Tzolkin::to_day_number(&state).unwrap(), 0);

        // Eb (ordinal 11 — the lane-11 zero of the Tzolk'in cycle) at tone 1.
        // Imix=0 is at day 0, so Eb at tone 1 has tone_0 = 0, sign = 11
        // → day = (40·0 + 221·11) % 260 = 2431 % 260 = 91.
        let state = Tzolkin::from_glyph(1, DayNameGlyph::Eb).unwrap();
        assert_eq!(state.residues(), &[0u64, 11]);
        let recovered = Tzolkin::to_day_number(&state).unwrap();
        assert_eq!(recovered, 91);

        // Verify against the Eb-as-lane-11-zero fact: ordinal 11.
        assert_eq!(<DayNameGlyph as GlyphAlphabet<6>>::ordinal(&DayNameGlyph::Eb), 11);
    }

    #[test]
    fn display_uses_modern_mayan_orthography() {
        let state = Tzolkin::from_day(0);
        let rendered = Tzolkin::display(&state).unwrap();
        // tone 1, Imix.
        assert_eq!(rendered, "1 Imix");

        // Day 1: tone 2, IkPrime (NOT colonial "Ik").
        let state = Tzolkin::from_day(1);
        let rendered = Tzolkin::display(&state).unwrap();
        assert_eq!(rendered, "2 IkPrime");

        // Day 19: tone 7, Ajaw (NOT colonial "Ahau").
        let state = Tzolkin::from_day(19);
        let rendered = Tzolkin::display(&state).unwrap();
        // tone_0 = 19 mod 13 = 6 → tone 7. sign = 19 mod 20 = 19 → Ajaw.
        assert_eq!(rendered, "7 Ajaw");
    }

    #[test]
    fn fibonacci_tone_pattern_length_28() {
        let seq = Tzolkin::fibonacci_tone_pattern().unwrap();
        // π(13) = 28.
        assert_eq!(seq.len(), 28);
        // Starts with 0, 1.
        assert_eq!(seq[0], 0);
        assert_eq!(seq[1], 1);
        // F_7 mod 13 = 0 (α(13) = 7).
        assert_eq!(seq[7], 0);
    }

    #[test]
    fn validate_rejects_wrong_lane_structure() {
        let wrong_lanes = vec![
            Lane { name: "foo", modulus: 13, domain: "" },
            Lane { name: "bar", modulus: 20, domain: "" },
        ];
        let state = MayaState::new(vec![0u64, 0], wrong_lanes).unwrap();
        assert_eq!(Tzolkin::validate(&state), Err(TzolkinError::MalformedState));
    }

    #[test]
    fn cycle_has_exactly_260_unique_states() {
        // 13 × 20 = 260, all CRT-distinct by coprimality.
        let mut all_states = std::collections::HashSet::new();
        for day in 0u64..260 {
            let state = Tzolkin::from_day(day);
            all_states.insert((state.residues()[0], state.residues()[1]));
        }
        assert_eq!(all_states.len(), 260);
        // Day 260 wraps to (0, 0) — same as day 0.
        let day_260 = Tzolkin::from_day(260);
        let day_0 = Tzolkin::from_day(0);
        assert!(day_260.same_position(&day_0));
    }

    #[test]
    fn cram_address_relationship_at_day_0() {
        // Tzolk'in day 0 corresponds to integer 0. Its CRAM address
        // is [0, 0, 0, 0, 0, 0] (every prime divides 0).
        let _state = Tzolkin::from_day(0);
        assert_eq!(dresden_codex::cram_address(0), [0u64; 6]);
        // Tzolk'in day 1 corresponds to integer 1. CRAM address [1,1,1,1,1,1].
        assert_eq!(dresden_codex::cram_address(1), [1u64, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn full_glyph_alphabet_bridge() {
        // Every DayNameGlyph maps to a unique sign-lane residue.
        let mut seen = std::collections::HashSet::new();
        for &glyph in ALL_DAY_NAMES.iter() {
            let state = Tzolkin::from_glyph(1, glyph).unwrap();
            assert!(seen.insert(state.residues()[1]));
        }
        assert_eq!(seen.len(), 20);
    }
}
