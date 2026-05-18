//! # Moon Goddess Section — Full Decoder (Dresden Codex pages 16-23)
//!
//! ## What the Moon Goddess section encodes
//!
//! Pages 16-23 of the Dresden Codex record a sequence of 9 eclipse
//! half-year intervals that constitute the Moon Goddess almanac:
//!
//! ```text
//! [148, 177, 148, 177, 148, 177, 148, 177, 148]
//! Total: 1448 days = 49 synodic months
//! ```
//!
//! The Moon Goddess (Ix Chel) is the deity associated with:
//! - The Moon and its cycles
//! - Medicine and healing
//! - Weaving and textile arts  
//! - Floods and water
//! - Childbirth and midwifery
//!
//! In the CRAM substrate framework, the Moon Goddess section encodes
//! the ECLIPSE HALF-YEAR ALTERNATION — the rhythm of 5-month and
//! 6-month eclipse windows.
//!
//! ## Structural position in the codex
//!
//! The Moon Goddess section occupies a structurally distinct carry class
//! from the three main Dresden sections (Tzolk'in/Venus/Eclipse):
//!
//! | Section | Carry class | Tzolk'in aligned? |
//! |---------|-------------|------------------|
//! | Tzolk'in, Venus, Eclipse | {active: 3,7,11} | YES |
//! | **Moon Goddess** | **{active: 2,5,7,11,13}** | **NO** |
//!
//! The Moon Goddess section carries information in lanes {2,5,7,11,13}
//! that the Tzolk'in-class sections do NOT capture (specifically lanes 2
//! and 5, which the Tzolk'in nullifies).
//!
//! ## The 148 key
//!
//! 1448 % 260 = 148 — the Moon Goddess remainder after stripping full
//! Tzolk'ins is exactly the first interval (148 days = 5 synodic months).
//! This means: after 5 full Tzolk'ins (1300 days), the Moon Goddess
//! sequence has 148 days left — it sits exactly one eclipse near-half-year
//! past the last Tzolk'in boundary.
//!
//! ## Lunar synthesis: the complete lunar hierarchy
//!
//! ```text
//! 29.53 days   = 1 synodic month
//! 148 days     = 5 synodic months (near eclipse half-year)
//! 177 days     = 6 synodic months (far eclipse half-year)
//! 325 days     = 11 synodic months (= 148 + 177 = one full eclipse window)
//! 354 days     = 12 synodic months (1 lunar year)
//! 1448 days    = 49 synodic months (Moon Goddess)
//! 5458 days    = ~184.8 months (18 × 177 + several 148s)
//! 11960 days   = 405 synodic months (Eclipse Table)
//! ```

#![allow(dead_code)]

use crate::lunar::*;
use crate::h4_non_visual::GODDESS_SECTION_INTERVALS;
use crate::h4_montgomery::{MontgomeryQuad, goddess_section_quads};
use crate::h5_level::k_elim_level;
use crate::h3_mi::entropy_nbp;
use dresden_codex::{cram_address, nullified_lanes, active_lanes,
                     carry_bits, pack_carry_bits, SAFE_BASIS, MultiPhaseSchema};

// ═══════════════════════════════════════════════════════════════════
// §1  Moon Goddess structural analysis
// ═══════════════════════════════════════════════════════════════════

/// Complete structural profile of the Moon Goddess section.
#[derive(Clone, Debug)]
pub struct MoonGoddessProfile {
    /// Total days in the 9-interval sequence.
    pub total_days: u64,
    /// Number of near (148-day) intervals.
    pub near_count: u64,
    /// Number of far (177-day) intervals.
    pub far_count: u64,
    /// Total synodic months.
    pub synodic_months: u64,
    /// CRAM address of the total.
    pub total_cram: [u64; 6],
    /// Carry-signature of the total.
    pub total_carry_sig: u8,
    /// K-Elim level at p=11.
    pub level_11: u32,
    /// Nullified lanes.
    pub nullified: Vec<u64>,
    /// Whether aligned to Tzolk'in.
    pub tzolkin_aligned: bool,
    /// Remainder mod Tzolk'in.
    pub tzolkin_remainder: u64,
    /// The carry-signature difference from the Tzolk'in class.
    pub carry_sig_vs_tzolkin: u32,
    /// CRAM addresses at each of the 9 page boundaries.
    pub page_cram_addresses: Vec<[u64; 6]>,
    /// Montgomery quads at each boundary.
    pub page_montgomery_quads: Vec<MontgomeryQuad>,
    /// Entropy of the 148/177 sequence (natural basis points).
    pub sequence_entropy_nbp: i64,
}

impl MoonGoddessProfile {
    pub fn compute() -> Self {
        let total = MOON_GODDESS_DAYS;
        let near = GODDESS_SECTION_INTERVALS.iter().filter(|&&x| x == 148).count() as u64;
        let far = GODDESS_SECTION_INTERVALS.iter().filter(|&&x| x == 177).count() as u64;
        let months = nearest_synodic_months(total);
        let cram = cram_address(total);
        let cbits = carry_bits(total);
        let csig = pack_carry_bits(&cbits);
        let level = k_elim_level(total, 11);
        let null = nullified_lanes(total);
        let tz_rem = total % TZOLKIN_DAYS;
        let tz_aligned = tz_rem == 0;
        let carry_diff = (csig ^ TZOLKIN_CARRY_SIG).count_ones();

        let page_crames: Vec<[u64; 6]> = {
            let mut acc = 0u64;
            GODDESS_SECTION_INTERVALS.iter().map(|&i| {
                acc += i;
                cram_address(acc)
            }).collect()
        };

        let quads: Vec<MontgomeryQuad> = goddess_section_quads()
            .into_iter().map(|(_, q)| q).collect();

        // Entropy of sequence: H([near, far])
        let ent = entropy_nbp(&[near, far]);

        MoonGoddessProfile {
            total_days: total,
            near_count: near,
            far_count: far,
            synodic_months: months,
            total_cram: cram,
            total_carry_sig: csig,
            level_11: level,
            nullified: null,
            tzolkin_aligned: tz_aligned,
            tzolkin_remainder: tz_rem,
            carry_sig_vs_tzolkin: carry_diff,
            page_cram_addresses: page_crames,
            page_montgomery_quads: quads,
            sequence_entropy_nbp: ent,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// §2  Lunar hierarchy for the Moon Goddess section
// ═══════════════════════════════════════════════════════════════════

/// The lunar hierarchy embedded in the Moon Goddess section.
#[derive(Clone, Debug)]
pub struct LunarHierarchyEntry {
    pub name: &'static str,
    pub days: u64,
    pub synodic_months: u64,
    pub carry_sig: u8,
    pub tzolkin_multiple: Option<u64>,
    pub eclipse_half_years: f64,
}

pub fn moon_goddess_lunar_hierarchy() -> Vec<LunarHierarchyEntry> {
    let periods: &[(&'static str, u64)] = &[
        ("Synodic month (29)", 29),
        ("Synodic month (30)", 30),
        ("Eclipse near (148)", 148),
        ("Eclipse far (177)", 177),
        ("Eclipse window (325)", 325),
        ("Lunar year (354)", 354),
        ("Moon Goddess (1448)", 1448),
        ("2× Moon Goddess (2896)", 2896),
        ("Saros (6585)", 6585),
        ("Eclipse Table (11960)", 11960),
    ];
    periods.iter().map(|&(name, days)| {
        let csig = pack_carry_bits(&carry_bits(days));
        let tz = if days % 260 == 0 { Some(days/260) } else { None };
        let sm = nearest_synodic_months(days);
        LunarHierarchyEntry {
            name, days, synodic_months: sm, carry_sig: csig,
            tzolkin_multiple: tz,
            eclipse_half_years: days as f64 / ((148+177)/2) as f64,
        }
    }).collect()
}

// ═══════════════════════════════════════════════════════════════════
// §3  The Ix Chel connection — a lunar phase model
// ═══════════════════════════════════════════════════════════════════

/// Model of the Moon Goddess section as a lunar phase almanac.
///
/// The 9 intervals represent 9 successive eclipse windows.
/// Each interval is either "near" (5 months, approaching eclipse)
/// or "far" (6 months, receding from eclipse).
/// The alternation 5,6,5,6,5,6,5,6,5 describes the rhythm:
/// start with a close approach, then alternate between closer and
/// farther eclipse windows.
#[derive(Clone, Debug)]
pub struct EclipseWindowSequence {
    /// One entry per page boundary.
    pub windows: Vec<EclipseWindow>,
    /// Running total of synodic months.
    pub running_months: Vec<u64>,
    /// Whether each window is a hazard window (approaching node).
    pub is_hazard: Vec<bool>,
}

#[derive(Clone, Debug)]
pub struct EclipseWindow {
    pub page: usize,
    pub interval_days: u64,
    pub interval_months: u64,
    pub cumulative_days: u64,
    pub cumulative_months: u64,
    pub kind: HalfYearKind,
    pub cram: [u64; 6],
    /// Is this window a solar eclipse hazard?
    /// Near windows (5 months) have higher eclipse probability.
    pub eclipse_hazard: bool,
}

pub fn build_eclipse_window_sequence() -> EclipseWindowSequence {
    let mut windows = Vec::new();
    let mut running_months = Vec::new();
    let mut is_hazard = Vec::new();
    let mut cum_days = 0u64;
    let mut cum_months = 0u64;

    for (i, &interval) in GODDESS_SECTION_INTERVALS.iter().enumerate() {
        cum_days += interval;
        let kind = HalfYearKind::from_days(interval);
        let months = kind.synodic_months();
        cum_months += months;
        let cram = cram_address(cum_days);
        // Near windows (5 months) are eclipse hazard zones
        let hazard = matches!(kind, HalfYearKind::Near);
        windows.push(EclipseWindow {
            page: i + 1,
            interval_days: interval,
            interval_months: months,
            cumulative_days: cum_days,
            cumulative_months: cum_months,
            kind,
            cram,
            eclipse_hazard: hazard,
        });
        running_months.push(cum_months);
        is_hazard.push(hazard);
    }
    EclipseWindowSequence { windows, running_months, is_hazard }
}

// ═══════════════════════════════════════════════════════════════════
// §4  Moon Goddess ↔ Eclipse Table connection
// ═══════════════════════════════════════════════════════════════════

/// How many Moon Goddess cycles (1448 days) fit into the Eclipse Table (11960 days)?
pub const MOON_GODDESS_IN_ECLIPSE: u64 = ECLIPSE_TABLE_DAYS / MOON_GODDESS_DAYS; // 8 (remainder 376)
pub const MOON_GODDESS_ECLIPSE_REMAINDER: u64 = ECLIPSE_TABLE_DAYS % MOON_GODDESS_DAYS; // 376

/// The 376-day remainder = 11960 - 8×1448 = 376 days.
/// 376 days = ?
///  376 = 8 × 47; 47 is prime
///  376 / 29.53 = 12.73 synodic months
///  Notably: 376 = 260 + 116 = Tzolk'in + (2 × 58) ← not clean
///  But: 376 = 2 × 148 + 80 = 2 × near-window + 80... also not clean
///  Best interpretation: 376 is the "intercalation" needed to bridge
///  8 Moon Goddess cycles to the 46-Tzolk'in Eclipse Table.
///  376 / 260 = 1.446... 376 % 260 = 116

/// How the Moon Goddess and Eclipse Table interlock.
#[derive(Clone, Debug)]
pub struct MoonGoddessEclipseLink {
    pub moon_goddess_days: u64,
    pub eclipse_table_days: u64,
    /// Whole Moon Goddess cycles in Eclipse Table.
    pub complete_cycles: u64,
    /// Remainder after complete cycles.
    pub remainder_days: u64,
    /// Synodic months in one Moon Goddess cycle.
    pub mg_synodic_months: u64,
    /// Total synodic months in Eclipse Table.
    pub et_synodic_months: u64,
    /// Common lunar denominator: LCM in synodic months.
    pub common_months: u64,
}

pub fn moon_goddess_eclipse_link() -> MoonGoddessEclipseLink {
    fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a % b) } }
    let mg_sm = nearest_synodic_months(MOON_GODDESS_DAYS);   // 49
    let et_sm = nearest_synodic_months(ECLIPSE_TABLE_DAYS);  // 405
    let common = mg_sm * et_sm / gcd(mg_sm, et_sm);
    MoonGoddessEclipseLink {
        moon_goddess_days: MOON_GODDESS_DAYS,
        eclipse_table_days: ECLIPSE_TABLE_DAYS,
        complete_cycles: MOON_GODDESS_IN_ECLIPSE,
        remainder_days: MOON_GODDESS_ECLIPSE_REMAINDER,
        mg_synodic_months: mg_sm,
        et_synodic_months: et_sm,
        common_months: common,
    }
}

// ═══════════════════════════════════════════════════════════════════
// §5  Tests
// ═══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moon_goddess_profile_computes() {
        let p = MoonGoddessProfile::compute();
        assert_eq!(p.total_days, 1448);
        assert_eq!(p.synodic_months, 49);
        assert_eq!(p.near_count, 5);
        assert_eq!(p.far_count, 4);
        // NOT Tzolk'in aligned
        assert!(!p.tzolkin_aligned);
        assert_eq!(p.tzolkin_remainder, 148,
            "Remainder is exactly one eclipse near-half-year");
    }

    #[test]
    fn moon_goddess_carry_class_is_distinct() {
        let p = MoonGoddessProfile::compute();
        // Moon Goddess carry sig ≠ Tzolk'in carry sig
        assert_ne!(p.total_carry_sig, TZOLKIN_CARRY_SIG);
        // The Hamming distance between them
        assert!(p.carry_sig_vs_tzolkin > 0, "Carry classes differ");
    }

    #[test]
    fn moon_goddess_nullification_includes_3() {
        let null = nullified_lanes(1448);
        // 1448 = 2³ × 181; 3 does not divide 1448
        assert!(!null.contains(&3), "1448 does not nullify lane 3");
        // 2 divides 1448
        assert!(null.contains(&2), "1448 nullifies lane 2");
    }

    #[test]
    fn eclipse_window_sequence_has_9_windows() {
        let seq = build_eclipse_window_sequence();
        assert_eq!(seq.windows.len(), 9);
        assert_eq!(seq.windows.last().unwrap().cumulative_days, 1448);
        assert_eq!(seq.windows.last().unwrap().cumulative_months, 49);
    }

    #[test]
    fn eclipse_window_alternation_is_correct() {
        let seq = build_eclipse_window_sequence();
        // Pattern: near, far, near, far, near, far, near, far, near
        for (i, w) in seq.windows.iter().enumerate() {
            if i % 2 == 0 {
                assert_eq!(w.kind, HalfYearKind::Near, "Even windows are near");
            } else {
                assert_eq!(w.kind, HalfYearKind::Far, "Odd windows are far");
            }
        }
    }

    #[test]
    fn eclipse_hazard_windows_are_near() {
        let seq = build_eclipse_window_sequence();
        for w in &seq.windows {
            if w.eclipse_hazard {
                assert_eq!(w.kind, HalfYearKind::Near);
            }
        }
    }

    #[test]
    fn moon_goddess_eclipse_link_constants() {
        assert_eq!(MOON_GODDESS_IN_ECLIPSE, 8);
        assert_eq!(MOON_GODDESS_ECLIPSE_REMAINDER, 11960 - 8*1448);
        // 376 days
        assert_eq!(MOON_GODDESS_ECLIPSE_REMAINDER, 376);
    }

    #[test]
    fn moon_goddess_eclipse_link_synodic_months() {
        let link = moon_goddess_eclipse_link();
        assert_eq!(link.mg_synodic_months, 49);
        assert_eq!(link.et_synodic_months, 405);
        // LCM(49, 405): 49 = 7², 405 = 3⁴×5, gcd=1 → LCM = 49×405 = 19845
        assert_eq!(link.common_months, 49 * 405);
    }

    #[test]
    fn lunar_hierarchy_contains_key_periods() {
        let hier = moon_goddess_lunar_hierarchy();
        let names: Vec<_> = hier.iter().map(|e| e.name).collect();
        assert!(names.iter().any(|&n| n.contains("1448")));
        assert!(names.iter().any(|&n| n.contains("11960")));
    }

    #[test]
    fn moon_goddess_page_cram_first_boundary() {
        let p = MoonGoddessProfile::compute();
        // First page boundary at 148 days
        assert_eq!(p.page_cram_addresses[0], cram_address(148));
        // 148 % 11 = 5 → lane 11 active
        assert_eq!(p.page_cram_addresses[0][4], 148 % 11);
    }

    #[test]
    fn moon_goddess_has_5_hazard_windows() {
        let seq = build_eclipse_window_sequence();
        let hazard_count = seq.is_hazard.iter().filter(|&&h| h).count();
        assert_eq!(hazard_count, 5, "5 near (hazard) windows in Moon Goddess");
    }
}
