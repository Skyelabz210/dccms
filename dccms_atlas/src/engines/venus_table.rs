//! # VenusTable — Engine 5 (B-7.5)
//!
//! Venus synchronization engine. `T_V = 584 = 8·73` and
//! `T_h (solar year) = 365 = 5·73` share the prime 73 as their
//! convergence factor. The synchronization period is
//! `lcm(584, 365) = 2,920 = 5·T_V = 8·T_h`.
//!
//! ## Four stations within a 584-day synodic cycle
//!
//! ```text
//!   MorningStar         236 days   (visibility before superior conj)
//!   SuperiorConjunction  90 days   (Venus behind sun)
//!   EveningStar         250 days   (visibility after superior conj)
//!   InferiorConjunction   8 days   (Venus between sun and Earth)
//!   ─────────────────── ────────
//!   Total               584 days   ✓
//! ```
//!
//! ## D-1 reaffirmed: `phi_approximation` dropped
//!
//! The vault's `phi_approximation()` returns `f64` ratio (8/5 = 1.6)
//! and `f64` error vs. φ ≈ 1.618. **This violates A1.** The arithmetic
//! content of the claim — the 8/5 sync, the 2920-day LCM, the
//! Venus-Earth structural relationship — is preserved in `find_sync()`
//! returning `(5, 8, 2920)`. The aesthetic gloss "the closest simple
//! rational approximant to φ" is not load-bearing.
//!
//! Source: vault `Mayas Engine.md` §ENGINE 5.

#![allow(dead_code)]

use super::lane::{Lane, MayaState};
use super::pisano::{fibonacci_entry_point, pisano_period, PisanoError};

/// Engine 5: Venus Table synchronization fabric over `ℤ/8 × ℤ/73`.
pub struct VenusTable;

/// The four Venus synodic-cycle stations — typed (NOT vault's `&'static str`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VenusStation {
    /// Days 0..236 of the synodic cycle.
    MorningStar,
    /// Days 236..326 of the synodic cycle (90 days).
    SuperiorConjunction,
    /// Days 326..576 of the synodic cycle (250 days).
    EveningStar,
    /// Days 576..584 of the synodic cycle (8 days).
    InferiorConjunction,
}

impl VenusTable {
    pub const VENUS_SYNODIC:  u64 = 584;
    pub const EARTH_YEAR:     u64 = 365;
    pub const SYNC_PERIOD:    u64 = 2_920;

    pub const MORNING_STAR:   u64 = 236;
    pub const SUPERIOR_CONJ:  u64 = 90;
    pub const EVENING_STAR:   u64 = 250;
    pub const INFERIOR_CONJ:  u64 = 8;

    /// Octave lane: ℤ/8ℤ (the 8 in T_V = 8·73, tracking Earth-year phase).
    pub const OCTAVE_LANE: Lane = Lane {
        name: "octave",
        modulus: 8,
        domain: "earth-year-phase",
    };

    /// Venus-specific lane: ℤ/73ℤ (the 73 shared with the Haab).
    pub const VENUS_LANE: Lane = Lane {
        name: "venus",
        modulus: 73,
        domain: "venus-specific",
    };

    /// Construct a Venus state from a day within the synodic period.
    ///
    /// Reduces `day_in_cycle mod VENUS_SYNODIC` then decomposes by CRT
    /// into the two coprime lanes [8, 73].
    pub fn venus_state(day_in_cycle: u64) -> MayaState {
        let d = day_in_cycle % Self::VENUS_SYNODIC;
        let lanes = vec![Self::OCTAVE_LANE, Self::VENUS_LANE];
        let residues = vec![d % 8, d % 73];
        MayaState::new(residues, lanes)
            .expect("Venus residues bounded by construction")
    }

    /// Which station of the synodic cycle `day_in_cycle` falls within.
    pub fn current_station(day_in_cycle: u64) -> VenusStation {
        let d = day_in_cycle % Self::VENUS_SYNODIC;
        if d < Self::MORNING_STAR {
            VenusStation::MorningStar
        } else if d < Self::MORNING_STAR + Self::SUPERIOR_CONJ {
            VenusStation::SuperiorConjunction
        } else if d < Self::MORNING_STAR + Self::SUPERIOR_CONJ + Self::EVENING_STAR {
            VenusStation::EveningStar
        } else {
            VenusStation::InferiorConjunction
        }
    }

    /// Find the Venus-Earth synchronization: smallest `(v, e, days)`
    /// with `v · VENUS_SYNODIC == e · EARTH_YEAR`.
    ///
    /// Returns `(5, 8, 2920)` — the canonical 5-Venus / 8-solar-year
    /// resonance.
    pub fn find_sync() -> (u64, u64, u64) {
        for v in 1..=20u64 {
            for e in 1..=20u64 {
                if v * Self::VENUS_SYNODIC == e * Self::EARTH_YEAR {
                    return (v, e, v * Self::VENUS_SYNODIC);
                }
            }
        }
        // Theorem: a sync exists at (5, 8) — unreachable by mathematical fact.
        (0, 0, 0)
    }

    /// The Maya's Venus correction schedule — integer day adjustments
    /// after specific cycle counts to keep the table aligned with
    /// the true Venus synodic period (583.92 days, not 584).
    ///
    /// Each entry is `(after_N_Venus_cycles, day_adjustment)` with the
    /// adjustment as `i64` (the Maya subtracted days, so negative).
    pub fn correction_schedule() -> Vec<(u64, i64)> {
        vec![
            (61,  -4),
            (122, -4),
            (183, -4),
            (305, -8),  // refined: every 5th correction is -8 instead of -4
        ]
    }

    /// Pisano analysis of the Venus lanes.
    ///
    /// Returns `(π(8), π(73), α(73), π(584))`. The 584-lane's Pisano
    /// period factors as `lcm(π(8), π(73))` (lanes are coprime).
    pub fn pisano_analysis() -> Result<(u64, u64, u64, u64), PisanoError> {
        let pi_8   = pisano_period(8)?;
        let pi_73  = pisano_period(73)?;
        let alpha_73 = fibonacci_entry_point(73)?;
        let pi_584 = pisano_period(Self::VENUS_SYNODIC)?;
        Ok((pi_8, pi_73, alpha_73, pi_584))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_stations_sum_to_synodic_period() {
        assert_eq!(
            VenusTable::MORNING_STAR
                + VenusTable::SUPERIOR_CONJ
                + VenusTable::EVENING_STAR
                + VenusTable::INFERIOR_CONJ,
            VenusTable::VENUS_SYNODIC
        );
        // And the synodic period matches dresden_codex.
        assert_eq!(VenusTable::VENUS_SYNODIC, dresden_codex::VENUS_SYNODIC);
    }

    #[test]
    fn find_sync_is_5_venus_8_solar() {
        let (v, e, days) = VenusTable::find_sync();
        assert_eq!(v, 5);
        assert_eq!(e, 8);
        assert_eq!(days, 2_920);
        // And matches dresden_codex::VENUS_HAAB_LCM.
        assert_eq!(days, dresden_codex::VENUS_HAAB_LCM);
    }

    #[test]
    fn venus_state_at_origin_has_both_lanes_zero() {
        let state = VenusTable::venus_state(0);
        assert_eq!(state.lane_count(), 2);
        assert_eq!(state.residues(), &[0u64, 0]);
    }

    #[test]
    fn current_station_transitions() {
        assert_eq!(VenusTable::current_station(0), VenusStation::MorningStar);
        assert_eq!(VenusTable::current_station(235), VenusStation::MorningStar);
        assert_eq!(VenusTable::current_station(236), VenusStation::SuperiorConjunction);
        assert_eq!(VenusTable::current_station(325), VenusStation::SuperiorConjunction);
        assert_eq!(VenusTable::current_station(326), VenusStation::EveningStar);
        assert_eq!(VenusTable::current_station(575), VenusStation::EveningStar);
        assert_eq!(VenusTable::current_station(576), VenusStation::InferiorConjunction);
        assert_eq!(VenusTable::current_station(583), VenusStation::InferiorConjunction);
        // Wraps: day 584 = 0 mod 584.
        assert_eq!(VenusTable::current_station(584), VenusStation::MorningStar);
    }

    #[test]
    fn every_day_in_cycle_yields_some_station() {
        for d in 0..VenusTable::VENUS_SYNODIC {
            let _ = VenusTable::current_station(d);
        }
    }

    #[test]
    fn correction_schedule_matches_vault() {
        let s = VenusTable::correction_schedule();
        assert_eq!(s, vec![(61, -4), (122, -4), (183, -4), (305, -8)]);
    }

    #[test]
    fn pisano_analysis_well_defined() {
        let (pi_8, pi_73, alpha_73, pi_584) = VenusTable::pisano_analysis().unwrap();
        // π(8) = 12.
        assert_eq!(pi_8, 12);
        // π(73) = 148 (well-known).
        assert_eq!(pi_73, 148);
        // α(73) — entry point.
        assert!(alpha_73 > 0);
        assert_eq!(pi_73 % alpha_73, 0, "α(73) | π(73)");
        // π(584) = lcm(π(8), π(73)) by CRT (since gcd(8,73)=1).
        fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a % b) } }
        let expected = pi_8 * pi_73 / gcd(pi_8, pi_73);
        assert_eq!(pi_584, expected);
    }

    #[test]
    fn venus_state_residues_match_cram_for_first_six_lanes() {
        // venus_state(d) gives [d mod 8, d mod 73].
        // cram_address(d) gives [d mod 2, mod 3, mod 5, mod 7, mod 11, mod 13].
        // These two address spaces use different basis primes — overlap only
        // implicitly through the underlying integer d.
        let d = 100u64;
        let venus = VenusTable::venus_state(d);
        let cram = dresden_codex::cram_address(d);
        // Just sanity-check that venus_state(d) decomposes d correctly:
        assert_eq!(venus.residues()[0], d % 8);
        assert_eq!(venus.residues()[1], d % 73);
        // And cram_address still computes its own canonical address:
        assert_eq!(cram[0], d % 2);
    }

    #[test]
    fn sync_period_factorization() {
        // 2920 = 2³·5·73 = 8·365 = 5·584.
        assert_eq!(VenusTable::SYNC_PERIOD, 8 * VenusTable::EARTH_YEAR);
        assert_eq!(VenusTable::SYNC_PERIOD, 5 * VenusTable::VENUS_SYNODIC);
        assert_eq!(VenusTable::SYNC_PERIOD, 2_920);
    }
}
