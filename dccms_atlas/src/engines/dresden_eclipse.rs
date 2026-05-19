//! # DresdenEclipse — Engine 4 (B-7.4)
//!
//! Eclipse prediction through commensuration. `PERIOD = 11,960 days =
//! 46 · Tzolk'in = 405 lunations (≈)`. Eclipse windows alternate at
//! `HALF_ECLIPSE_SHORT = 148` (5 lunar months) and `HALF_ECLIPSE_LONG
//! = 177` (6 lunar months) — the same 148/177 alternation the Moon
//! Goddess almanac records.
//!
//! ## Commensuration
//!
//! - `PERIOD mod 260 = 0` ✓ (Tzolk'in invariance across eclipse periods)
//! - `PERIOD mod 13  = 0` ✓
//! - `PERIOD mod 20  = 0` ✓
//! - `PERIOD mod 365 ≠ 0` (= 280; **the Haab does NOT divide**)
//! - `PERIOD mod 584 ≠ 0` (= 280; **Venus does NOT divide**)
//!
//! The vault catches this honestly: "11,960 mod 365 ≠ 0." Our
//! `verify_commensuration` reports each commensuration relationship
//! truthfully without inflating the claim.
//!
//! ## D-6 — unused `eclipse_number` arg dropped
//!
//! Vault's `tzolkin_at_eclipse(reference, eclipse_number)` ignores
//! `eclipse_number` (Tzolk'in is invariant across ANY number of
//! eclipse periods since `PERIOD mod 13 = 0` and `PERIOD mod 20 = 0`).
//! We drop the unused parameter — the function is just `clone`.
//!
//! Source: vault `Mayas Engine.md` §ENGINE 4.

#![allow(dead_code)]

use super::lane::MayaState;

/// Engine 4: Dresden Eclipse Table.
pub struct DresdenEclipse;

impl DresdenEclipse {
    /// The fundamental eclipse-table period.
    pub const PERIOD: u64 = 11_960;

    /// Long eclipse half-year: 6 lunar months = 177 days.
    pub const HALF_ECLIPSE_LONG: u64 = 177;

    /// Short eclipse half-year: 5 lunar months = 148 days.
    pub const HALF_ECLIPSE_SHORT: u64 = 148;

    /// Eclipse warning-stations within one period.
    ///
    /// Starts at day 0 and alternates `HALF_ECLIPSE_LONG` (177) and
    /// `HALF_ECLIPSE_SHORT` (148) increments. Only stations strictly
    /// less than `PERIOD` are included.
    pub fn warning_stations() -> Vec<u64> {
        let mut stations = vec![0u64];
        let mut day = 0u64;
        let mut use_short = false;
        loop {
            let interval = if use_short {
                Self::HALF_ECLIPSE_SHORT
            } else {
                Self::HALF_ECLIPSE_LONG
            };
            day += interval;
            if day >= Self::PERIOD { break; }
            stations.push(day);
            use_short = !use_short;
        }
        stations
    }

    /// Whether `day_in_period` falls within `tolerance` of any warning station.
    pub fn is_eclipse_window(day_in_period: u64, tolerance: u64) -> bool {
        let d = day_in_period % Self::PERIOD;
        Self::warning_stations().iter().any(|&s| {
            let diff = if d >= s { d - s } else { s - d };
            diff <= tolerance
        })
    }

    /// Next eclipse window after `reference_day` — adds one full PERIOD.
    pub fn next_eclipse_from(reference_day: u64) -> u64 {
        reference_day + Self::PERIOD
    }

    /// Tzolk'in position is INVARIANT across eclipse periods because
    /// `PERIOD mod 13 = 0` AND `PERIOD mod 20 = 0` — so this is just
    /// the identity on `MayaState`. D-6: unused vault `eclipse_number`
    /// parameter is dropped.
    pub fn tzolkin_at_eclipse(reference_tzolkin: &MayaState) -> MayaState {
        reference_tzolkin.clone()
    }

    /// Commensuration table — honest reporting per modulus.
    ///
    /// Returns `(name, modulus, divides)` for each cycle examined.
    /// `divides == true` iff `PERIOD mod modulus == 0`. Haab (365) and
    /// Venus (584) correctly report `false`.
    pub fn verify_commensuration() -> Vec<(&'static str, u64, bool)> {
        let checks: &[(&'static str, u64)] = &[
            ("Tzolk'in",        260),
            ("Sacred 13",        13),
            ("Uinal",            20),
            ("Haab (365)",      365),
            ("Venus synodic",   584),
        ];
        checks.iter()
            .map(|&(name, m)| (name, m, Self::PERIOD % m == 0))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::tzolkin::Tzolkin;

    #[test]
    fn period_is_46_tzolkins() {
        assert_eq!(DresdenEclipse::PERIOD, 46 * 260);
        assert_eq!(DresdenEclipse::PERIOD, 11_960);
    }

    #[test]
    fn half_eclipse_constants_match_goddess_section_intervals() {
        // The 148/177 alternation is the same as the Moon Goddess almanac's
        // [148, 177, 148, 177, ...] interval sequence.
        assert_eq!(DresdenEclipse::HALF_ECLIPSE_SHORT, dresden_codex::ECLIPSE_NEAR);
        assert_eq!(DresdenEclipse::HALF_ECLIPSE_LONG, dresden_codex::ECLIPSE_FAR);
    }

    #[test]
    fn warning_stations_start_at_zero_and_alternate() {
        let stations = DresdenEclipse::warning_stations();
        assert_eq!(stations[0], 0);
        // After day 0, the first interval is 177 (long), then 148 (short), etc.
        assert_eq!(stations[1], 177);
        assert_eq!(stations[2], 177 + 148);  // 325
        assert_eq!(stations[3], 177 + 148 + 177);  // 502
        assert_eq!(stations[4], 177 + 148 + 177 + 148);  // 650
        // Every station is strictly less than PERIOD.
        for &s in &stations {
            assert!(s < DresdenEclipse::PERIOD);
        }
    }

    #[test]
    fn is_eclipse_window_zero_day_zero_tolerance() {
        assert!(DresdenEclipse::is_eclipse_window(0, 0));
        // Day 1 with zero tolerance — between stations, not a window.
        assert!(!DresdenEclipse::is_eclipse_window(1, 0));
        // Day 1 with tolerance 3 — still close enough to day 0.
        assert!(DresdenEclipse::is_eclipse_window(1, 3));
    }

    #[test]
    fn next_eclipse_advances_by_one_period() {
        assert_eq!(DresdenEclipse::next_eclipse_from(0), 11_960);
        assert_eq!(DresdenEclipse::next_eclipse_from(100), 100 + 11_960);
    }

    #[test]
    fn tzolkin_invariant_across_eclipse_period() {
        // Pick a Tzolk'in state; advance by PERIOD; same residues.
        let start = Tzolkin::from_day(91);  // Eb-tone-1 from earlier session
        let advanced = start.advance(DresdenEclipse::PERIOD);
        assert!(start.same_position(&advanced));
        // And tzolkin_at_eclipse is the identity (D-6 verified).
        let same = DresdenEclipse::tzolkin_at_eclipse(&start);
        assert!(start.same_position(&same));
    }

    #[test]
    fn verify_commensuration_honest_about_haab_and_venus() {
        let table = DresdenEclipse::verify_commensuration();
        // Tzolk'in, Sacred 13, Uinal all divide PERIOD.
        for &(name, m, divides) in &table {
            match name {
                "Tzolk'in" | "Sacred 13" | "Uinal" => {
                    assert!(divides, "{} ({}) should divide PERIOD", name, m);
                }
                "Haab (365)" => {
                    assert!(!divides,
                        "Haab does NOT divide PERIOD (vault Decoded.md catches this)");
                    // Actual remainder: 11_960 mod 365 = 280.
                    assert_eq!(DresdenEclipse::PERIOD % m, 280);
                }
                "Venus synodic" => {
                    assert!(!divides,
                        "Venus synodic does NOT divide PERIOD");
                    assert_eq!(DresdenEclipse::PERIOD % m, 280);
                }
                _ => panic!("unexpected commensuration row: {}", name),
            }
        }
    }

    #[test]
    fn warning_stations_alternation_correct_count() {
        // Stations are at sums of alternating 177/148. Since PERIOD =
        // 11960, and the average interval ≈ 162.5, we expect roughly
        // 11960 / 162.5 ≈ 73 stations. Exact value depends on alternation.
        let stations = DresdenEclipse::warning_stations();
        assert!(stations.len() >= 70 && stations.len() <= 80,
            "expected ~73 stations, got {}", stations.len());
    }
}
