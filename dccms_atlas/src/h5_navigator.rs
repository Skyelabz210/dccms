//! # H5 Navigator — Prime 11 as Configuration Index
//!
//! Hypothesis: prime 11 is the navigation mechanism between
//! configurations, not a structural prime of any single configuration.
//!
//! Diagnostic signature:
//! - 11-lane distribution across the four-head address space should
//!   be **uniform** (because 11 labels which configuration is active,
//!   and configurations are equipopulated when sampled across all
//!   codex events).
//! - 11-lane distribution within any single head's projection should
//!   be **non-uniform** (because 11 tracks drift within that head's
//!   configuration).
//!
//! This module implements the distributional test as a Kolmogorov-
//! Smirnov-style comparison: the 11-lane residue histogram against
//! the uniform distribution, computed exactly in integer arithmetic.

use crate::events::EventSet;
use crate::heads::{FourCalendarHydra, HydraHead};
use crate::SHADOW_PRIME;
use std::collections::HashMap;

/// Per-residue count for a single prime lane.
#[derive(Clone, Debug)]
pub struct LaneHistogram {
    pub prime: u64,
    /// Count for each residue class (0..p).
    pub counts: Vec<u64>,
    pub total: u64,
}

impl LaneHistogram {
    pub fn new(prime: u64) -> Self {
        LaneHistogram {
            prime,
            counts: vec![0; prime as usize],
            total: 0,
        }
    }

    pub fn add(&mut self, residue: u64) {
        let idx = (residue % self.prime) as usize;
        self.counts[idx] += 1;
        self.total += 1;
    }

    /// Expected proportion per class under uniform distribution, in basis points.
    /// This is the proportion any single class should have (1/prime expressed as bp).
    pub fn expected_per_class_bp(&self) -> u64 {
        if self.prime == 0 {
            return 0;
        }
        10_000 / self.prime
    }

    /// Maximum absolute deviation from uniform, in basis points.
    /// This is the Kolmogorov-Smirnov-like statistic.
    ///
    /// For each class, compute the observed proportion (basis points)
    /// and compare to the uniform-expected proportion. Return the max
    /// absolute deviation across all classes.
    pub fn max_deviation_bp(&self) -> u64 {
        if self.total == 0 {
            return 0;
        }
        let expected_bp = self.expected_per_class_bp();
        let mut max_dev: u64 = 0;
        for &count in &self.counts {
            let observed_bp = (count * 10_000) / self.total;
            let dev = if observed_bp > expected_bp {
                observed_bp - expected_bp
            } else {
                expected_bp - observed_bp
            };
            if dev > max_dev {
                max_dev = dev;
            }
        }
        max_dev
    }

    /// Chi-square-like statistic (sum of squared deviations, scaled).
    /// Returns (numerator, denominator) as exact integers.
    pub fn chi_sq_exact(&self) -> (u128, u128) {
        if self.total == 0 || self.prime == 0 {
            return (0, 1);
        }
        // Expected count: total / prime
        // Numerator of chi-sq: Σ (observed - expected)² × prime
        // Denominator: total
        let total = self.total as u128;
        let prime = self.prime as u128;
        let mut num: u128 = 0;
        for &count in &self.counts {
            // expected × prime = total
            let observed_p = (count as u128) * prime;
            let diff = if observed_p > total {
                observed_p - total
            } else {
                total - observed_p
            };
            num += diff * diff;
        }
        // Final: num / (total × prime)
        (num, total * prime)
    }

    pub fn is_uniform(&self, deviation_threshold_bp: u64) -> bool {
        self.max_deviation_bp() <= deviation_threshold_bp
    }
}

/// H5 Verdict.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H5Verdict {
    /// 11-lane uniform across four-head space AND non-uniform within
    /// single heads. The navigator hypothesis is supported.
    Supported,
    /// One of the two signatures holds but not both.
    Partial,
    /// Neither signature holds.
    Unsupported,
    /// Not enough data.
    Indeterminate,
}

impl H5Verdict {
    pub fn label(&self) -> &'static str {
        match self {
            H5Verdict::Supported => "SUPPORTED",
            H5Verdict::Partial => "PARTIAL",
            H5Verdict::Unsupported => "UNSUPPORTED",
            H5Verdict::Indeterminate => "INDETERMINATE",
        }
    }
}

/// Full H5 report.
#[derive(Clone, Debug)]
pub struct H5Report {
    /// 11-lane histogram across the four-head address-space partitioning.
    pub four_head_histogram: LaneHistogram,
    /// 11-lane histogram within each single head's address space
    /// (events after the head's first cycle completion).
    pub single_head_histograms: HashMap<String, LaneHistogram>,
    /// Per-phase histograms for each head (key: head name, value: histograms by phase).
    pub per_phase_histograms: HashMap<String, Vec<LaneHistogram>>,
    /// Maximum per-phase deviation per head — H5 expects this to exceed
    /// the nonuniform threshold.
    pub per_phase_max_deviation_bp: HashMap<String, u64>,
    /// Whether the four-head distribution is uniform (low max deviation).
    pub four_head_uniform: bool,
    /// Whether per-phase distributions are non-uniform (high deviation).
    pub per_phase_nonuniform: HashMap<String, bool>,
    pub verdict: H5Verdict,
}

/// Compute the 11-lane distribution across the four-head address
/// partitioning. Each event's 11-residue is recorded.
pub fn four_head_lane_histogram(events: &EventSet) -> LaneHistogram {
    let mut hist = LaneHistogram::new(SHADOW_PRIME);
    for event in &events.events {
        // event.address[4] is the residue mod 11 (Safe Basis index 4)
        hist.add(event.address[4]);
    }
    hist
}

/// Compute the H5 report.
pub fn compute_h5_report(
    events: &EventSet,
    hydra: &FourCalendarHydra,
    uniform_threshold_bp: u64,
    nonuniform_threshold_bp: u64,
) -> H5Report {
    let four_head = four_head_lane_histogram(events);
    let four_head_uniform = four_head.is_uniform(uniform_threshold_bp);

    let mut single_head_histograms = HashMap::new();
    let mut per_phase_histograms = HashMap::new();
    let mut per_phase_max_deviation_bp = HashMap::new();
    let mut per_phase_nonuniform = HashMap::new();

    for head in hydra.heads() {
        let name = head.signature.name.to_string();
        let single_hist = single_head_lane_histogram(events, head);
        single_head_histograms.insert(name.clone(), single_hist);

        let phase_hists = per_phase_lane_histograms(events, head);
        let max_dev = phase_hists.iter()
            .filter(|h| h.total > 0)
            .map(|h| h.max_deviation_bp())
            .max()
            .unwrap_or(0);
        per_phase_max_deviation_bp.insert(name.clone(), max_dev);
        per_phase_nonuniform.insert(name.clone(), max_dev >= nonuniform_threshold_bp);
        per_phase_histograms.insert(name, phase_hists);
    }

    let all_nonuniform = per_phase_nonuniform.values().all(|&n| n);
    let any_nonuniform = per_phase_nonuniform.values().any(|&n| n);

    let verdict = if events.is_empty() {
        H5Verdict::Indeterminate
    } else if four_head_uniform && all_nonuniform {
        H5Verdict::Supported
    } else if four_head_uniform || any_nonuniform {
        H5Verdict::Partial
    } else {
        H5Verdict::Unsupported
    };

    H5Report {
        four_head_histogram: four_head,
        single_head_histograms,
        per_phase_histograms,
        per_phase_max_deviation_bp,
        four_head_uniform,
        per_phase_nonuniform,
        verdict,
    }
}

/// Per-phase 11-lane distribution within a single head.
///
/// For each phase index in the head, build a separate 11-lane histogram
/// of the events that fall in that phase. If H5 holds, these per-phase
/// histograms should be NON-uniform (11 tracks drift within a phase).
///
/// Returns a vector of histograms, one per phase.
pub fn per_phase_lane_histograms(events: &EventSet, head: &HydraHead) -> Vec<LaneHistogram> {
    let phase_count = head.phase_count();
    let mut histograms: Vec<LaneHistogram> = (0..phase_count)
        .map(|_| LaneHistogram::new(SHADOW_PRIME))
        .collect();
    for event in &events.events {
        let phase_idx = head.current_phase(event.days_since_epoch);
        if phase_idx < phase_count {
            histograms[phase_idx].add(event.address[4]);
        }
    }
    histograms
}

/// Compute the maximum deviation across per-phase histograms for a head.
/// If any phase shows non-uniform 11-distribution, this returns its
/// deviation; otherwise the maximum (least uniform) across all phases.
pub fn head_per_phase_max_deviation_bp(events: &EventSet, head: &HydraHead) -> u64 {
    let histograms = per_phase_lane_histograms(events, head);
    histograms.iter()
        .filter(|h| h.total > 0)
        .map(|h| h.max_deviation_bp())
        .max()
        .unwrap_or(0)
}

/// Single-head lane histogram: aggregate 11-lane residues across all
/// events that the head's primary cycle has "completed" at least once
/// (i.e., events occurring after day = cycle_length).
///
/// This is the within-head 11-distribution that H5 predicts should be
/// non-uniform.
pub fn single_head_lane_histogram(events: &EventSet, head: &HydraHead) -> LaneHistogram {
    let mut hist = LaneHistogram::new(SHADOW_PRIME);
    let cycle = head.signature.cycle;
    for event in &events.events {
        // Only count events that have completed at least one full cycle
        // of this head's primary length — that's where "within-head drift"
        // is observable.
        if event.days_since_epoch >= cycle {
            hist.add(event.address[4]);
        }
    }
    hist
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::CodexEventKind;
    use crate::events::CodexEvent;

    #[test]
    fn empty_histogram_is_uniform() {
        let h = LaneHistogram::new(11);
        assert_eq!(h.max_deviation_bp(), 0);
        assert!(h.is_uniform(0));
    }

    #[test]
    fn uniform_histogram_has_zero_deviation() {
        let mut h = LaneHistogram::new(11);
        // Add 11 events, one per residue class
        for r in 0..11u64 {
            h.add(r);
        }
        // Each class should have observed = expected
        assert_eq!(h.max_deviation_bp(), 0);
    }

    #[test]
    fn skewed_histogram_has_high_deviation() {
        let mut h = LaneHistogram::new(11);
        // All 11 events at residue 0
        for _ in 0..11 {
            h.add(0);
        }
        // Residue 0 has 100% of events; expected was 1/11 = ~909 bp
        // Deviation should be (10000 - 909) = 9091 bp
        let dev = h.max_deviation_bp();
        assert!(dev > 9000, "expected high deviation, got {}", dev);
    }

    #[test]
    fn h5_report_handles_empty_events() {
        let events = EventSet::new();
        let hydra = FourCalendarHydra::canonical();
        let report = compute_h5_report(&events, &hydra, 500, 1000);
        assert_eq!(report.verdict, H5Verdict::Indeterminate);
    }

    #[test]
    fn h5_report_builds_for_canonical_corpus() {
        let events = EventSet::canonical_corpus(0, 50_000);
        let hydra = FourCalendarHydra::canonical();
        let report = compute_h5_report(&events, &hydra, 500, 1000);
        // The verdict will depend on actual data; just verify it computed.
        assert_eq!(report.single_head_histograms.len(), 4);
        assert_eq!(report.four_head_histogram.prime, 11);
    }

    #[test]
    fn chi_sq_returns_exact_integers() {
        let mut h = LaneHistogram::new(11);
        for r in 0..22u64 {
            h.add(r);
        }
        let (num, den) = h.chi_sq_exact();
        // Perfectly uniform → numerator should be 0
        assert_eq!(num, 0);
        assert!(den > 0);
    }
}
