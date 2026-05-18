//! # Lunar Arithmetic — Dresden Codex Lunar Framework
//!
//! ## The Lunar Backbone
//!
//! The Dresden Codex encodes three interlocking astronomical cycles:
//!
//! ```text
//! Synodic month    ≈  29.53 days  (Moon phase cycle)
//! Eclipse half-year = 148 or 177 days (5 or 6 synodic months)
//! Saros cycle      ≈  6585 days  (18 yr eclipse repeat)
//! Metonic cycle    ≈  6940 days  (19 yr lunar-solar sync)
//! Tzolk'in         =  260 days   (the substrate clock)
//! ```
//!
//! ## The Grand Structural Fact
//!
//! Three Dresden sections share the SAME carry-signature class:
//!
//! | Section       | Days   | Nullified | Active     |
//! |--------------|--------|-----------|-----------|
//! | Tzolk'in      | 260    | {2,5,13}  | {3,7,11}  |
//! | Eclipse Table | 11,960 | {2,5,13}  | {3,7,11}  |
//! | Venus Table   | 37,960 | {2,5,13}  | {3,7,11}  |
//!
//! All three are multiples of 260 (Tzolk'in).
//! They are the "Tzolk'in carry class" — the substrate operations that
//! preserve the {active: 3,7,11} signature.
//!
//! The Moon Goddess section (1,448 days) does NOT belong to this class:
//! 1448 % 13 = 5 (≠ 0), so lane 13 is active → different carry class.
//!
//! ## The Eclipse-Lunar Identity
//!
//! 11,960 = 405 synodic months (within 0.001%)
//!
//! This is the fundamental commensurability that makes eclipse prediction
//! possible: exactly 405 synodic months fit into the Tzolk'in-aligned
//! 11,960-day Eclipse Table.
//!
//! ## Integer arithmetic
//!
//! Synodic month = 29531/1000 days (exact rational from Babylonian/Maya tables).
//! All computations are exact integer.

#![allow(dead_code)]

use dresden_codex::{cram_address, nullified_lanes, active_lanes,
                     carry_bits, pack_carry_bits, SAFE_BASIS};
use crate::h5_level::k_elim_level;

// ═══════════════════════════════════════════════════════════════════
// §1  Fundamental constants
// ═══════════════════════════════════════════════════════════════════

/// Synodic month in exact integer thousandths of a day.
/// True: 29.53059 days. Dresden approximation: 29531/1000.
pub const SYNODIC_MONTH_NUM: u64 = 29_531;
pub const SYNODIC_MONTH_DEN: u64 = 1_000;

/// Eclipse half-year: 5 synodic months = 147.653 days → 148 days (Maya integer).
pub const ECLIPSE_NEAR_DAYS: u64 = 148;

/// Eclipse half-year: 6 synodic months = 177.184 days → 177 days (Maya integer).
pub const ECLIPSE_FAR_DAYS: u64 = 177;

/// Saros cycle in days (exact integer approximation used by Maya).
pub const SAROS_DAYS: u64 = 6_585;

/// Metonic cycle: 19 solar years ≈ 235 synodic months ≈ 6940 days.
pub const METONIC_DAYS: u64 = 6_940;

/// Tzolk'in: 260 days (the substrate clock).
pub const TZOLKIN_DAYS: u64 = 260;

/// Haab: 365 days (the solar year approximation).
pub const HAAB_DAYS: u64 = 365;

/// Venus synodic: 584 days.
pub const VENUS_SYNODIC_DAYS: u64 = 584;

/// Eclipse Table total: 11960 days = 46 × Tzolk'in = 405 synodic months.
pub const ECLIPSE_TABLE_DAYS: u64 = 11_960;

/// Venus Table total: 37960 days = 146 × Tzolk'in = 65 × Venus.
pub const VENUS_TABLE_DAYS: u64 = 37_960;

/// Moon Goddess section total: 1448 days = 49 synodic months.
pub const MOON_GODDESS_DAYS: u64 = 1_448;

/// Lunar year: 12 synodic months ≈ 354 days.
pub const LUNAR_YEAR_DAYS: u64 = 354;

// ═══════════════════════════════════════════════════════════════════
// §2  Lunar arithmetic
// ═══════════════════════════════════════════════════════════════════

/// Compute the number of synodic months in `days` days, scaled by 1000.
///
/// Returns (months × 1000, integer). Exact.
pub fn synodic_months_x1000(days: u64) -> u64 {
    days * SYNODIC_MONTH_DEN / SYNODIC_MONTH_NUM
}

/// Nearest integer number of synodic months to `days` days.
pub fn nearest_synodic_months(days: u64) -> u64 {
    (days * SYNODIC_MONTH_DEN + SYNODIC_MONTH_NUM / 2) / SYNODIC_MONTH_NUM
}

/// Residual: |days - n_months × SYNODIC_MONTH| in thousandths of a day.
pub fn lunar_residual_milli(days: u64) -> u64 {
    let n = nearest_synodic_months(days);
    let approx_days_milli = n * SYNODIC_MONTH_NUM; // n × 29531
    let days_milli = days * SYNODIC_MONTH_DEN;
    if days_milli >= approx_days_milli {
        days_milli - approx_days_milli
    } else {
        approx_days_milli - days_milli
    }
}

// ═══════════════════════════════════════════════════════════════════
// §3  Dresden section characterisation
// ═══════════════════════════════════════════════════════════════════

/// Characterisation of one Dresden Codex section from the CRAM substrate.
#[derive(Clone, Debug)]
pub struct SectionProfile {
    pub name: &'static str,
    pub days: u64,
    /// CRAM address (residue tuple on S₆).
    pub cram: [u64; 6],
    /// Carry-bit signature (packed u8).
    pub carry_sig: u8,
    /// K-Elim level at p=11.
    pub level_11: u32,
    /// Nullified Safe Basis primes.
    pub nullified: Vec<u64>,
    /// Active Safe Basis primes.
    pub active: Vec<u64>,
    /// Is this section a multiple of the Tzolk'in (260)?
    pub tzolkin_aligned: bool,
    /// Tzolk'in multiple (if aligned).
    pub tzolkin_multiple: Option<u64>,
    /// Nearest integer number of synodic months.
    pub synodic_months: u64,
    /// Lunar residual in thousandths of a day.
    pub lunar_residual_milli: u64,
}

impl SectionProfile {
    pub fn compute(name: &'static str, days: u64) -> Self {
        let cram = cram_address(days);
        let cbits = carry_bits(days);
        let csig = pack_carry_bits(&cbits);
        let level = k_elim_level(days, 11);
        let null = nullified_lanes(days);
        let act = active_lanes(days);
        let tz_aligned = days % TZOLKIN_DAYS == 0;
        let tz_mult = if tz_aligned { Some(days / TZOLKIN_DAYS) } else { None };
        let sm = nearest_synodic_months(days);
        let lr = lunar_residual_milli(days);
        SectionProfile {
            name, days, cram, carry_sig: csig, level_11: level,
            nullified: null, active: act,
            tzolkin_aligned: tz_aligned, tzolkin_multiple: tz_mult,
            synodic_months: sm, lunar_residual_milli: lr,
        }
    }
}

/// Build profiles for all major Dresden sections.
pub fn all_section_profiles() -> Vec<SectionProfile> {
    vec![
        SectionProfile::compute("Tzolk'in",        TZOLKIN_DAYS),
        SectionProfile::compute("Haab",             HAAB_DAYS),
        SectionProfile::compute("Venus synodic",    VENUS_SYNODIC_DAYS),
        SectionProfile::compute("Moon Goddess",     MOON_GODDESS_DAYS),
        SectionProfile::compute("Saros",            SAROS_DAYS),
        SectionProfile::compute("Metonic",          METONIC_DAYS),
        SectionProfile::compute("Eclipse Table",    ECLIPSE_TABLE_DAYS),
        SectionProfile::compute("Venus Table",      VENUS_TABLE_DAYS),
        SectionProfile::compute("Eclipse near",     ECLIPSE_NEAR_DAYS),
        SectionProfile::compute("Eclipse far",      ECLIPSE_FAR_DAYS),
    ]
}

// ═══════════════════════════════════════════════════════════════════
// §4  Carry-class grouping
// ═══════════════════════════════════════════════════════════════════

/// The "Tzolk'in carry class": sections with carry_sig = carry_sig(260).
pub const TZOLKIN_CARRY_SIG: u8 = {
    // 260 = 2²×5×13 → nullified {2,5,13} → active bits 1,3,4 set
    // SAFE_BASIS = [2,3,5,7,11,13] → indices [0,1,2,3,4,5]
    // Active: 3(idx1)=1, 7(idx3)=1, 11(idx4)=1 → bits 1,3,4 set
    // packed = 2^1 + 2^3 + 2^4 = 2 + 8 + 16 = 26
    26u8
};

/// Determine which carry-class a section belongs to.
pub fn carry_class_label(carry_sig: u8) -> &'static str {
    match carry_sig {
        26 => "Tzolk'in class {active: 3,7,11}",
        58 => "Haab class {active: 2,3,7,11,13}",
        30 => "Venus {active: 3,5,7,11,13}",
        _  => "other",
    }
}

// ═══════════════════════════════════════════════════════════════════
// §5  Eclipse half-year sequence tools
// ═══════════════════════════════════════════════════════════════════

/// Classify a duration as a "near" (5-month) or "far" (6-month) eclipse half-year.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HalfYearKind { Near, Far, Other(u64) }

impl HalfYearKind {
    pub fn from_days(days: u64) -> Self {
        match days {
            148 => HalfYearKind::Near,
            177 => HalfYearKind::Far,
            d   => HalfYearKind::Other(d),
        }
    }
    pub fn synodic_months(&self) -> u64 {
        match self { HalfYearKind::Near => 5, HalfYearKind::Far => 6, _ => 0 }
    }
}

/// Total synodic months in a sequence of eclipse half-years.
pub fn total_synodic_months(intervals: &[u64]) -> u64 {
    intervals.iter().map(|&d| HalfYearKind::from_days(d).synodic_months()).sum()
}

/// Whether a sequence of eclipse half-years sums to an exact Tzolk'in multiple.
pub fn is_tzolkin_aligned(intervals: &[u64]) -> bool {
    let total: u64 = intervals.iter().sum();
    total % TZOLKIN_DAYS == 0
}

// ═══════════════════════════════════════════════════════════════════
// §6  Tests
// ═══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tzolkin_carry_sig_is_26() {
        let sig = pack_carry_bits(&carry_bits(260));
        assert_eq!(sig, 26);
        assert_eq!(sig, TZOLKIN_CARRY_SIG);
    }

    #[test]
    fn eclipse_table_carry_sig_equals_tzolkin() {
        let eclipse_sig = pack_carry_bits(&carry_bits(ECLIPSE_TABLE_DAYS));
        assert_eq!(eclipse_sig, TZOLKIN_CARRY_SIG,
            "Eclipse Table carry sig must equal Tzolk'in carry sig");
    }

    #[test]
    fn venus_table_carry_sig_equals_tzolkin() {
        let venus_sig = pack_carry_bits(&carry_bits(VENUS_TABLE_DAYS));
        assert_eq!(venus_sig, TZOLKIN_CARRY_SIG,
            "Venus Table carry sig must equal Tzolk'in carry sig");
    }

    #[test]
    fn moon_goddess_carry_sig_differs_from_tzolkin() {
        let mg_sig = pack_carry_bits(&carry_bits(MOON_GODDESS_DAYS));
        assert_ne!(mg_sig, TZOLKIN_CARRY_SIG,
            "Moon Goddess carry sig must DIFFER from Tzolk'in carry sig");
        // 1448 % 13 = 5 (≠ 0), so lane 13 is active in Moon Goddess
        assert_eq!(1448 % 13, 5, "Lane 13 is active in Moon Goddess");
    }

    #[test]
    fn eclipse_table_is_tzolkin_multiple() {
        assert_eq!(ECLIPSE_TABLE_DAYS % TZOLKIN_DAYS, 0);
        assert_eq!(ECLIPSE_TABLE_DAYS / TZOLKIN_DAYS, 46);
    }

    #[test]
    fn venus_table_is_tzolkin_multiple() {
        assert_eq!(VENUS_TABLE_DAYS % TZOLKIN_DAYS, 0);
        assert_eq!(VENUS_TABLE_DAYS / TZOLKIN_DAYS, 146);
    }

    #[test]
    fn moon_goddess_not_tzolkin_aligned() {
        assert_ne!(MOON_GODDESS_DAYS % TZOLKIN_DAYS, 0);
        assert_eq!(MOON_GODDESS_DAYS % TZOLKIN_DAYS, 148,
            "Moon Goddess remainder is exactly 148 — the first eclipse half-year");
    }

    #[test]
    fn eclipse_table_is_405_synodic_months() {
        // 11960 days ÷ 29.531 days/month ≈ 404.996 ≈ 405
        let months = nearest_synodic_months(ECLIPSE_TABLE_DAYS);
        assert_eq!(months, 405, "Eclipse Table = 405 synodic months");
    }

    #[test]
    fn moon_goddess_is_49_synodic_months() {
        let months = nearest_synodic_months(MOON_GODDESS_DAYS);
        assert_eq!(months, 49, "Moon Goddess = 49 synodic months");
    }

    #[test]
    fn eclipse_near_is_5_synodic_months() {
        assert_eq!(HalfYearKind::from_days(148).synodic_months(), 5);
    }

    #[test]
    fn eclipse_far_is_6_synodic_months() {
        assert_eq!(HalfYearKind::from_days(177).synodic_months(), 6);
    }

    #[test]
    fn moon_goddess_sequence_is_49_months() {
        use crate::h4_non_visual::GODDESS_SECTION_INTERVALS;
        let total = total_synodic_months(&GODDESS_SECTION_INTERVALS);
        assert_eq!(total, 49, "Moon Goddess = 5+6+5+6+5+6+5+6+5 = 49 synodic months");
    }

    #[test]
    fn section_profiles_all_computed() {
        let profiles = all_section_profiles();
        assert!(!profiles.is_empty());
        let eclipse = profiles.iter().find(|p| p.name == "Eclipse Table").unwrap();
        assert_eq!(eclipse.tzolkin_multiple, Some(46));
        assert_eq!(eclipse.synodic_months, 405);
        assert_eq!(eclipse.carry_sig, TZOLKIN_CARRY_SIG);
    }

    #[test]
    fn carry_class_label_tzolkin() {
        assert_eq!(carry_class_label(26), "Tzolk'in class {active: 3,7,11}");
    }

    #[test]
    fn eclipse_and_venus_tables_have_common_lcm() {
        // LCM(11960, 37960) = 873080
        fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a%b) } }
        let lcm = ECLIPSE_TABLE_DAYS * VENUS_TABLE_DAYS / gcd(ECLIPSE_TABLE_DAYS, VENUS_TABLE_DAYS);
        assert_eq!(lcm, 873_080);
        assert_eq!(lcm / ECLIPSE_TABLE_DAYS, 73);
        assert_eq!(lcm / VENUS_TABLE_DAYS, 23);
    }
}
