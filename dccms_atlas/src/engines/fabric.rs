//! # MayaFabric — Unified five-engine substrate (B-7.6)
//!
//! Binds all five Maya computational engines on a single shared
//! temporal substrate. At any day `d`, a `MayaFabric` carries:
//!
//! - the raw day count (`day`),
//! - the Tzolk'in state ([`super::tzolkin::Tzolkin`]) at day `d`,
//! - the Long Count ([`super::long_count::LongCount`]) at day `d`,
//! - the position within the current Venus synodic cycle
//!   (`venus_day = d mod VENUS_SYNODIC`),
//! - the position within the current Dresden eclipse period
//!   (`eclipse_day = d mod PERIOD`).
//!
//! [`alignment_report`](MayaFabric::alignment_report) returns a typed
//! summary: which engines are simultaneously at a structurally
//! significant boundary at this day.
//!
//! Vigesimal is per-value, not per-day; it doesn't appear in `MayaFabric`.
//! That's faithful to vault `Mayas Engine.md` §UNIFIED FABRIC.
//!
//! Source: vault `Mayas Engine.md` §UNIFIED FABRIC.

#![allow(dead_code)]

use super::dresden_eclipse::DresdenEclipse;
use super::lane::MayaState;
use super::long_count::{LongCount, PeriodEnding};
use super::tzolkin::Tzolkin;
use super::venus_table::VenusTable;

/// The unified Maya computational fabric at one moment in time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MayaFabric {
    pub day: u64,
    pub tzolkin: MayaState,
    pub long_count: LongCount,
    pub venus_day: u64,    // day within current Venus synodic period
    pub eclipse_day: u64,  // day within current Dresden eclipse period
}

/// Alignment-significance report at a single `MayaFabric` day.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FabricAlignment {
    pub day: u64,
    /// Both Tzolk'in lanes at origin: tone=1 (residue 0) AND sign=Imix (residue 0).
    pub tzolkin_origin: bool,
    /// Venus position is exactly at a station boundary
    /// (0, 236, 326, or 576 within the synodic cycle).
    pub venus_station_boundary: bool,
    /// Within `eclipse_window_tolerance` days of an eclipse warning station.
    pub eclipse_window: bool,
    /// Period-ending markers from the Long Count.
    pub long_count_endings: Vec<PeriodEnding>,
    /// Sum of booleans (Tzolk'in origin, Venus boundary, Eclipse window)
    /// + number of Long Count tier-endings. Crude additive score.
    pub alignment_score: u32,
}

impl MayaFabric {
    /// Tolerance (in days) for the `eclipse_window` boolean.
    pub const ECLIPSE_WINDOW_TOLERANCE: u64 = 3;

    /// Long Count creation-date origin (day 0).
    pub fn origin() -> Self {
        Self::at_day(0)
    }

    /// State of all engines at an arbitrary day count.
    pub fn at_day(day: u64) -> Self {
        Self {
            day,
            tzolkin: Tzolkin::from_day(day),
            long_count: LongCount::from_days(day),
            venus_day: day % VenusTable::VENUS_SYNODIC,
            eclipse_day: day % DresdenEclipse::PERIOD,
        }
    }

    /// Advance the entire fabric by `n` days.
    pub fn advance(&self, n: u64) -> Self {
        Self::at_day(self.day + n)
    }

    /// Compute the alignment report at the current fabric day.
    pub fn alignment_report(&self) -> FabricAlignment {
        let tone_residue = self.tzolkin.residues()[0];
        let sign_residue = self.tzolkin.residues()[1];
        let tzolkin_origin = tone_residue == 0 && sign_residue == 0;

        let venus_boundaries = [
            0u64,
            VenusTable::MORNING_STAR,
            VenusTable::MORNING_STAR + VenusTable::SUPERIOR_CONJ,
            VenusTable::MORNING_STAR + VenusTable::SUPERIOR_CONJ + VenusTable::EVENING_STAR,
        ];
        let venus_station_boundary = venus_boundaries.contains(&self.venus_day);

        let eclipse_window = DresdenEclipse::is_eclipse_window(
            self.eclipse_day, Self::ECLIPSE_WINDOW_TOLERANCE,
        );

        let long_count_endings = self.long_count.period_endings();

        let alignment_score = (tzolkin_origin as u32)
            + (venus_station_boundary as u32)
            + (eclipse_window as u32)
            + (long_count_endings.len() as u32);

        FabricAlignment {
            day: self.day,
            tzolkin_origin,
            venus_station_boundary,
            eclipse_window,
            long_count_endings,
            alignment_score,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_equals_at_day_zero() {
        assert_eq!(MayaFabric::origin(), MayaFabric::at_day(0));
    }

    #[test]
    fn at_day_zero_has_all_subfields_at_origin() {
        let f = MayaFabric::origin();
        assert_eq!(f.day, 0);
        assert_eq!(f.long_count, LongCount::new(0, 0, 0, 0, 0));
        assert_eq!(f.tzolkin.residues(), &[0u64, 0]);
        assert_eq!(f.venus_day, 0);
        assert_eq!(f.eclipse_day, 0);
    }

    #[test]
    fn advance_zero_is_identity() {
        let a = MayaFabric::at_day(123);
        let b = a.advance(0);
        assert_eq!(a, b);
    }

    #[test]
    fn advance_additive() {
        let a = MayaFabric::at_day(100);
        let b = a.advance(50);
        let c = MayaFabric::at_day(150);
        assert_eq!(b, c);
    }

    #[test]
    fn thirteen_baktun_fabric() {
        let f = MayaFabric::at_day(1_872_000);
        assert_eq!(f.long_count.baktun, 13);
        assert_eq!(f.long_count.katun, 0);
        assert_eq!(f.long_count.tun, 0);
        assert_eq!(f.long_count.uinal, 0);
        assert_eq!(f.long_count.kin, 0);
    }

    #[test]
    fn alignment_report_at_day_zero_is_strong() {
        // Day 0 is special: Tzolk'in origin (1 Imix), Venus origin
        // (start of MorningStar), Eclipse origin (warning station at 0),
        // Long Count origin (all four tier-endings simultaneously).
        let f = MayaFabric::origin();
        let a = f.alignment_report();
        assert!(a.tzolkin_origin);
        assert!(a.venus_station_boundary);
        assert!(a.eclipse_window);
        assert_eq!(a.long_count_endings.len(), 4);
        // Tzolk'in + Venus + Eclipse + 4 endings = 7.
        assert_eq!(a.alignment_score, 7);
    }

    #[test]
    fn alignment_report_at_mid_cycle_is_weaker() {
        // Day 200: Tzolk'in not at origin (200 mod 13 = 5, mod 20 = 0;
        // sign==0 but tone != 0, so NOT both-at-origin). Mid-MorningStar
        // (day 200 < 236). Not an eclipse window. No Long Count endings
        // (kin = 0 here? 200 mod 20 = 0, so kin = 0 → UinalEnd).
        let f = MayaFabric::at_day(200);
        let a = f.alignment_report();
        assert!(!a.tzolkin_origin);
        assert!(!a.venus_station_boundary);
        // Day 200's eclipse-window status: 200 vs. nearest warning station.
        // Stations: 0, 177, 325, ... → nearest to 200 is 177 (diff=23) or
        // 325 (diff=125). Tolerance 3 → not a window.
        assert!(!a.eclipse_window);
        // 200 mod 20 = 0 → kin=0 → UinalEnd.
        assert_eq!(a.long_count_endings, vec![PeriodEnding::UinalEnd]);
        assert_eq!(a.alignment_score, 1);
    }
}
