//! # H3 Extended — Generator-Predicted Candidate Heads
//!
//! Now that H1 Stage 8 has established the bilinear generator, we can
//! systematically enumerate ALL cycles the generator produces and test
//! each as a candidate H3 head.
//!
//! ## Generator-predicted cycles (not yet tested)
//!
//! From the Stage 8 tree, the generator with multiplier set
//! {2,3,4,5,7,8,11,13,18,20,73} and base set {20,73,260,365,360} yields:
//!
//! | Cycle | Generator path | Astronomical interpretation |
//! |-------|---------------|---------------------------|
//! | 140   | G(7, 20)      | 7 winals — Quarter-Tzolk'in |
//! | 220   | G(11, 20)     | 11 winals — shadow-prime multiple |
//! | 780   | G(3, 260)     | Mars synodic (already known) |
//! | 803   | G(11, 73)     | 11 × astro-prime |
//! | 949   | G(13, 73)     | 13 × astro-prime |
//! | 1460  | G(4, 365)     | 4-year cycle |
//! | 1820  | G(7, 260)     | 7 Tzolk'ins |
//! | 2860  | G(11, 260)    | 11 Tzolk'ins |
//! | 2920  | G(8, 365)     | Venus Great Round (8 years) |
//! | 4745  | G(13, 365)    | 13 years |
//!
//! ## DKAM pre-filter
//!
//! Before running H3 MI, each candidate is filtered through DKAM:
//! cycles whose effective operator degree ≥ 3 are excluded.
//!
//! ## H3 instrument: same as existing h3_mi module
//!
//! Independence ratio = H(candidate | four_head) / H(candidate).
//! Threshold: 30% (3000 bp) as pre-registered.

#![allow(dead_code)]

use crate::events::EventSet;
use crate::heads::{FourCalendarHydra, HydraHead};
use crate::h3_mi::h3_mi_report;
use crate::h1_stage8::{GeneratorApplication, SEED, ASTRO_PRIME};
use crate::dkam_filter::DkamFilter;
use dresden_codex::{cram_address, nullified_lanes, active_lanes, SAFE_BASIS};

// ═══════════════════════════════════════════════════════════════════
// §1  Enumerate generator-predicted cycles
// ═══════════════════════════════════════════════════════════════════

/// A generator-predicted candidate cycle with full provenance.
#[derive(Clone, Debug)]
pub struct GeneratorCandidate {
    pub cycle: u64,
    pub generator_path: String,
    /// Astronomical name if known.
    pub astro_name: &'static str,
    /// K-Elim level at p=11.
    pub level_11: u32,
    /// CRAM address.
    pub cram: [u64; 6],
    /// Nullified Safe Basis primes.
    pub nullified: Vec<u64>,
}

impl GeneratorCandidate {
    fn new(cycle: u64, path: impl Into<String>, name: &'static str) -> Self {
        use crate::h5_level::k_elim_level;
        GeneratorCandidate {
            cycle,
            generator_path: path.into(),
            astro_name: name,
            level_11: k_elim_level(cycle, 11),
            cram: cram_address(cycle),
            nullified: nullified_lanes(cycle),
        }
    }
}

/// Enumerate all generator-predicted cycles up to max_cycle.
///
/// Systematically applies G(n, base) for all multipliers in the
/// extended set and all known bases (including the astronomical prime 73).
pub fn enumerate_generator_candidates(max_cycle: u64) -> Vec<GeneratorCandidate> {
    // Previously tested cycles — skip them.
    let already_tested: &[u64] = &[
        121,  // Saturn-11²
        28,   // Temperaments-4fold
        325,  // EclipseAlt (148+177)
        584,  // VenusPhase
        819,  // PlanetaryCouncil
        360,  // Zodiac-12fold
        260, 365, 18_980, 144_000, // canonical heads
    ];

    let multipliers: &[(u64, &str)] = &[
        (2, "2"), (3, "3"), (4, "4"), (5, "5"), (6, "6"), (7, "7"),
        (8, "8"), (9, "9"), (11, "11"), (12, "12"), (13, "13"),
        (14, "14"), (17, "17"), (18, "18"), (19, "19"), (20, "20"),
        (73, "73"),
    ];

    let bases: &[(u64, &str)] = &[
        (SEED, "20"),
        (ASTRO_PRIME, "73"),
        (260, "260"),
        (365, "365"),
        (360, "360"),
        (520, "520"),
    ];

    let intercalary_units: &[u64] = &[3, 5, 7, 11, 13];

    let mut candidates: Vec<GeneratorCandidate> = Vec::new();
    let mut seen: std::collections::HashSet<u64> = already_tested.iter().copied().collect();

    // Uniform rule
    for &(n, nlabel) in multipliers {
        for &(base, blabel) in bases {
            let c = n.saturating_mul(base);
            if c == 0 || c > max_cycle || seen.contains(&c) { continue; }
            seen.insert(c);
            let name = astro_name(c);
            candidates.push(GeneratorCandidate::new(
                c,
                format!("G({}, {})", nlabel, blabel),
                name,
            ));
        }
    }

    // Intercalary rule: G(n, 20, 1, inter) for small n
    for &(n, nlabel) in multipliers.iter().filter(|(n,_)| *n <= 25) {
        for &inter in intercalary_units {
            let c = n.saturating_mul(SEED) + inter;
            if c == 0 || c > max_cycle || seen.contains(&c) { continue; }
            seen.insert(c);
            let name = astro_name(c);
            candidates.push(GeneratorCandidate::new(
                c,
                format!("G({}, 20, 1, {})", nlabel, inter),
                name,
            ));
        }
    }

    candidates.sort_by_key(|c| c.cycle);
    candidates
}

fn astro_name(c: u64) -> &'static str {
    match c {
        140 => "7-winal quarter",
        173 => "half-eclipse",
        220 => "11-winal",
        260 => "Tzolk'in",
        365 => "Haab",
        520 => "double-Tzolk'in",
        584 => "Venus synodic",
        780 => "Mars synodic",
        803 => "11×73",
        949 => "13×73",
        1460 => "4-year",
        1820 => "7 Tzolk'ins",
        2920 => "Venus Great Round",
        2860 => "11 Tzolk'ins",
        4015 => "11×365",
        4745 => "13×365",
        _ => "generator-predicted",
    }
}

// ═══════════════════════════════════════════════════════════════════
// §2  Build heads from generator candidates
// ═══════════════════════════════════════════════════════════════════

/// Build a HydraHead from a generator candidate.
///
/// Phase structure is inferred from the generator path:
/// uniform → n phases of base days each;
/// intercalary → n phases of base + 1 phase of inter.
pub fn head_from_generator_candidate(cand: &GeneratorCandidate) -> HydraHead {
    // Parse the generator path to recover n, base, inter.
    // Simple: for uniform G(n, base) → HydraHead::new(name, &[base; n])
    // For intercalary G(n, base, 1, inter) → &[base; n] + [inter]
    // Fallback: single-phase head = the cycle itself.
    let path = &cand.generator_path;
    if path.contains("1,") {
        // Intercalary: extract n, base, inter
        let nums: Vec<u64> = path
            .trim_start_matches("G(").trim_end_matches(")")
            .split(',')
            .filter_map(|s| s.trim().parse::<u64>().ok())
            .collect();
        if nums.len() >= 4 {
            let (n_reg, base, _, inter) = (nums[0], nums[1], nums[2], nums[3]);
            if n_reg > 0 && base > 0 && n_reg <= 100 {
                let mut intervals = vec![base; n_reg as usize];
                intervals.push(inter);
                return HydraHead::new(Box::leak(
                    cand.astro_name.to_string().into_boxed_str()),
                    &intervals);
            }
        }
    } else {
        // Uniform: extract n, base
        let nums: Vec<u64> = path
            .trim_start_matches("G(").trim_end_matches(")")
            .split(',')
            .filter_map(|s| s.trim().parse::<u64>().ok())
            .collect();
        if nums.len() >= 2 {
            let (n, base) = (nums[0], nums[1]);
            if n > 0 && base > 0 && n <= 500 {
                let intervals = vec![base; n as usize];
                return HydraHead::new(Box::leak(
                    cand.astro_name.to_string().into_boxed_str()),
                    &intervals);
            }
        }
    }
    // Fallback: single-phase
    HydraHead::new(Box::leak(cand.astro_name.to_string().into_boxed_str()),
                   &[cand.cycle])
}

// ═══════════════════════════════════════════════════════════════════
// §3  H3 extended report
// ═══════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct H3ExtendedEntry {
    pub cycle: u64,
    pub generator_path: String,
    pub astro_name: &'static str,
    pub level_11: u32,
    pub dkam_admissible: bool,
    /// H3 independence ratio in basis points (None if DKAM rejected).
    pub independence_ratio_bp: Option<u64>,
    pub h3_admissible: bool,
}

#[derive(Clone, Debug)]
pub struct H3ExtendedReport {
    pub entries: Vec<H3ExtendedEntry>,
    /// New cycles that pass both DKAM and H3 threshold.
    pub newly_admissible: Vec<(u64, String)>,
    /// Count of tested candidates.
    pub tested: usize,
    /// Count of DKAM-admissible candidates.
    pub dkam_pass: usize,
    /// Count of H3-admissible candidates.
    pub h3_pass: usize,
}

/// Run the full H3 extended test.
///
/// `max_cycle`: upper bound for candidate cycles.
/// `h3_threshold_bp`: independence threshold (pre-registered: 3000 bp).
pub fn compute_h3_extended(
    events: &EventSet,
    hydra: &FourCalendarHydra,
    max_cycle: u64,
    h3_threshold_bp: u64,
) -> H3ExtendedReport {
    let candidates = enumerate_generator_candidates(max_cycle);
    let filter = DkamFilter::canonical();
    let mut entries: Vec<H3ExtendedEntry> = Vec::new();
    let mut newly_admissible: Vec<(u64, String)> = Vec::new();

    for cand in &candidates {
        let head = head_from_generator_candidate(cand);

        // DKAM filter
        let rep = filter.evaluate(&head);
        let dkam_ok = rep.admissible;

        let (ratio_bp, h3_ok) = if dkam_ok {
            let report = h3_mi_report(events, hydra, &head, h3_threshold_bp);
            (Some(report.independence_ratio_bp), report.admissible)
        } else {
            (None, false)
        };

        if h3_ok {
            newly_admissible.push((cand.cycle, cand.generator_path.clone()));
        }

        entries.push(H3ExtendedEntry {
            cycle: cand.cycle,
            generator_path: cand.generator_path.clone(),
            astro_name: cand.astro_name,
            level_11: cand.level_11,
            dkam_admissible: dkam_ok,
            independence_ratio_bp: ratio_bp,
            h3_admissible: h3_ok,
        });
    }

    let tested = entries.len();
    let dkam_pass = entries.iter().filter(|e| e.dkam_admissible).count();
    let h3_pass = entries.iter().filter(|e| e.h3_admissible).count();

    H3ExtendedReport {
        entries,
        newly_admissible,
        tested,
        dkam_pass,
        h3_pass,
    }
}

// ═══════════════════════════════════════════════════════════════════
// §4  Tests
// ═══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::heads::FourCalendarHydra;

    #[test]
    fn enumerate_candidates_non_empty() {
        let cands = enumerate_generator_candidates(10_000);
        assert!(!cands.is_empty(), "Generator must produce candidates");
    }

    #[test]
    fn enumerate_candidates_excludes_canonical() {
        let cands = enumerate_generator_candidates(50_000);
        let cycles: Vec<u64> = cands.iter().map(|c| c.cycle).collect();
        // Canonical heads must be excluded (already tested)
        assert!(!cycles.contains(&260), "Tzolk'in excluded");
        assert!(!cycles.contains(&365), "Haab excluded");
        assert!(!cycles.contains(&18_980), "CalendarRound excluded");
        assert!(!cycles.contains(&144_000), "Baktun excluded");
    }

    #[test]
    fn enumerate_candidates_includes_key_generator_cycles() {
        let cands = enumerate_generator_candidates(10_000);
        let cycles: Vec<u64> = cands.iter().map(|c| c.cycle).collect();
        // These should be produced by the generator
        assert!(cycles.contains(&140), "7×20 = 140 should be a candidate");
        assert!(cycles.contains(&220), "11×20 = 220 should be a candidate");
        assert!(cycles.contains(&780), "3×260 = 780 (Mars) should be a candidate");
        assert!(cycles.contains(&2920), "8×365 = 2920 (Venus GR) should be a candidate");
    }

    #[test]
    fn generator_candidate_cram_address_correct() {
        let cands = enumerate_generator_candidates(5_000);
        let c140 = cands.iter().find(|c| c.cycle == 140).unwrap();
        // 140 = 2² × 5 × 7 → nullified = {2, 5, 7}
        assert!(c140.nullified.contains(&7), "140 nullifies lane 7");
        assert_eq!(c140.cram, cram_address(140));
    }

    #[test]
    fn generator_candidate_level_correct() {
        let cands = enumerate_generator_candidates(5_000);
        // 140 < 1331 → Level-2
        let c140 = cands.iter().find(|c| c.cycle == 140).unwrap();
        assert_eq!(c140.level_11, 2, "140 is Level-2");
        // 1820 > 1331 → Level-3
        let c1820 = cands.iter().find(|c| c.cycle == 1820).unwrap();
        assert_eq!(c1820.level_11, 3, "1820 is Level-3");
    }

    #[test]
    fn head_from_candidate_produces_correct_cycle() {
        let cand = GeneratorCandidate::new(780, "G(3, 260)", "Mars synodic");
        let head = head_from_generator_candidate(&cand);
        assert_eq!(head.signature.cycle, 780);
        assert_eq!(head.schema.phases.len(), 3);
    }

    #[test]
    fn head_intercalary_from_candidate() {
        let cand = GeneratorCandidate::new(367, "G(18, 20, 1, 7)", "test-inter");
        let head = head_from_generator_candidate(&cand);
        assert_eq!(head.signature.cycle, 367);
        assert_eq!(head.schema.phases.len(), 19); // 18 + 1
    }

    #[test]
    fn h3_extended_runs_on_small_range() {
        let events = EventSet::canonical_corpus(0, 50_000);
        let hydra = FourCalendarHydra::canonical();
        let report = compute_h3_extended(&events, &hydra, 1_000, 3_000);
        assert!(report.tested > 0);
        assert!(report.dkam_pass <= report.tested);
        assert!(report.h3_pass <= report.dkam_pass);
    }

    #[test]
    fn mars_synodic_is_candidate() {
        let cands = enumerate_generator_candidates(5_000);
        let mars = cands.iter().find(|c| c.cycle == 780).unwrap();
        assert_eq!(mars.astro_name, "Mars synodic");
        assert_eq!(mars.generator_path, "G(3, 260)");
    }
}
