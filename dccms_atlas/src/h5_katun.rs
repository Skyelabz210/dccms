//! # H5 Katun Depth Analysis
//!
//! The Katun (7200 days) is Level-3 at p=11 — the FIRST period in the
//! Long Count hierarchy where κ₃ can be nonzero. This makes it the
//! critical transition point between "shadow-free" and "depth-active"
//! periods.
//!
//! ## The Katun partition
//!
//! Within one Katun (7200 days), positions split into two zones:
//!
//! ```text
//! Shadow-free zone:  positions 0..1330   (1331 days, 18.5%)
//!     κ₃(position) = 0  always
//!     The substrate sees only (κ₀, κ₁, κ₂) — three K-Elim layers
//!
//! Depth zone:        positions 1331..7199  (5869 days, 81.5%)
//!     κ₃(position) ≠ 0 possible
//!     The substrate sees all four K-Elim layers (κ₀, κ₁, κ₂, κ₃)
//! ```
//!
//! ## Structural prediction
//!
//! For events from the canonical corpus, the Long Count head partitions
//! its 7200-day cycle into the two zones. Events landing in the depth
//! zone are "seen" differently by the substrate than events in the
//! shadow-free zone.
//!
//! Within the depth zone, κ₃ should be approximately uniform over
//! {1, 2, ..., 10} (since κ₃=0 is impossible there). Within the
//! shadow-free zone, κ₃ = 0 always.
//!
//! ## Connection to the Long Count
//!
//! The Long Count Katun has 20 sub-periods of 360 days (= Tun) each.
//! The shadow-free zone contains approximately 3.7 Tuns (1331/360 ≈ 3.7).
//! So: the first four Tuns of each Katun are shadow-free; the remaining
//! 16 Tuns are in the depth zone.
//!
//! This is the most precise structural prediction the K-Elim Level
//! theorem makes about the Long Count calendar.

#![allow(dead_code)]

use crate::events::EventSet;
use crate::heads::FourCalendarHydra;
use crate::h5_level::THRESHOLD_11_3;
use crate::SHADOW_PRIME;

// ═══════════════════════════════════════════════════════════════════
// §1  Katun zone classification
// ═══════════════════════════════════════════════════════════════════

/// The Katun cycle length.
pub const KATUN_DAYS: u64 = 7_200;

/// Tun (360-day sub-unit of the Katun).
pub const TUN_DAYS: u64 = 360;

/// Number of Tuns in the Katun.
pub const TUNS_PER_KATUN: u64 = 20;

/// The depth-activation threshold (11³ = 1331).
pub const DEPTH_THRESHOLD: u64 = THRESHOLD_11_3; // 1331

/// Number of shadow-free Tuns: floor(1331/360) = 3.
pub const SHADOW_FREE_TUNS: u64 = 3;

/// Zone classification for a position within the Katun.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KatunZone {
    /// Position < 1331: κ₃ = 0 always.
    ShadowFree,
    /// Position ≥ 1331: κ₃ can be nonzero.
    DepthActive,
}

pub fn classify_katun_position(within_katun: u64) -> KatunZone {
    if within_katun < DEPTH_THRESHOLD {
        KatunZone::ShadowFree
    } else {
        KatunZone::DepthActive
    }
}

/// Which Tun (0-indexed) within the Katun does position p fall in?
pub fn tun_index(within_katun: u64) -> u64 {
    within_katun / TUN_DAYS
}

// ═══════════════════════════════════════════════════════════════════
// §2  κ₃ distribution by zone
// ═══════════════════════════════════════════════════════════════════

/// κ₃ distribution for events in one Katun zone.
#[derive(Clone, Debug)]
pub struct KappaThreeZoneDistribution {
    pub zone: KatunZone,
    pub event_count: u64,
    /// κ₃ histogram: counts[k] = number of events with κ₃ = k.
    pub histogram: Vec<u64>,
    /// Max deviation from the zone's predicted distribution (bp).
    pub deviation_from_predicted_bp: u64,
}

impl KappaThreeZoneDistribution {
    /// Build from events and their Katun local positions.
    fn build(zone: KatunZone, local_positions: &[u64]) -> Self {
        let p3 = SHADOW_PRIME.pow(3); // 11³ = 1331
        let mut hist = vec![0u64; SHADOW_PRIME as usize];
        for &pos in local_positions {
            let k3 = (pos / p3) % SHADOW_PRIME;
            if (k3 as usize) < hist.len() {
                hist[k3 as usize] += 1;
            }
        }
        let total = local_positions.len() as u64;

        let dev = match zone {
            KatunZone::ShadowFree => {
                // Predicted: κ₃ = 0 always → hist[0] = total, rest = 0.
                // Deviation of bin 0 from "all there": always 0.
                // Deviation of any other bin from 0: if any non-zero bin exists.
                let non_zero: u64 = hist[1..].iter().sum();
                if total > 0 { non_zero * 10_000 / total } else { 0 }
            }
            KatunZone::DepthActive => {
                // Predicted: κ₃ uniform over {1..10} (bin 0 has prob 0 here,
                // since position ≥ 1331 means floor(pos/1331) ≥ 1, so κ₃ ≥ 1).
                // Actually: κ₃ = (pos/1331) % 11. For pos in [1331, 7199]:
                // floor(pos/1331) ∈ {1, 2, 3, 4, 5} → κ₃ ∈ {1,2,3,4,5}.
                // So κ₃ is NOT uniform over all 11 values — it's restricted to {1..5}.
                // Expected counts are roughly equal over {1..5}.
                let active: Vec<u64> = hist[1..6].to_vec(); // bins 1-5
                let active_total: u64 = active.iter().sum();
                if active_total == 0 { 0 } else {
                    let expected = active_total / 5;
                    active.iter().map(|&c| {
                        let d = if c > expected { c - expected } else { expected - c };
                        d * 10_000 / active_total.max(1)
                    }).max().unwrap_or(0)
                }
            }
        };

        KappaThreeZoneDistribution {
            zone,
            event_count: total,
            histogram: hist,
            deviation_from_predicted_bp: dev,
        }
    }

    pub fn kappa3_zero_fraction_bp(&self) -> u64 {
        if self.event_count == 0 { return 0; }
        self.histogram[0] * 10_000 / self.event_count
    }
}

// ═══════════════════════════════════════════════════════════════════
// §3  Per-Tun analysis
// ═══════════════════════════════════════════════════════════════════

/// Event distribution across the 20 Tuns of the Katun.
#[derive(Clone, Debug)]
pub struct TunDistribution {
    /// Event count per Tun (0-indexed).
    pub tun_counts: Vec<u64>,
    /// κ₃ = 0 fraction per Tun (bp).
    pub tun_kappa3_zero_bp: Vec<u64>,
    /// Whether each Tun is in the shadow-free zone.
    pub tun_shadow_free: Vec<bool>,
}

impl TunDistribution {
    fn build(local_positions: &[u64]) -> Self {
        let p3 = SHADOW_PRIME.pow(3);
        let n_tuns = TUNS_PER_KATUN as usize;
        let mut counts = vec![0u64; n_tuns];
        let mut k3_zero = vec![0u64; n_tuns];

        for &pos in local_positions {
            let tun = (pos / TUN_DAYS).min(TUNS_PER_KATUN - 1) as usize;
            counts[tun] += 1;
            let k3 = (pos / p3) % SHADOW_PRIME;
            if k3 == 0 { k3_zero[tun] += 1; }
        }

        let tun_kappa3_zero_bp = (0..n_tuns).map(|i| {
            if counts[i] == 0 { 0 } else { k3_zero[i] * 10_000 / counts[i] }
        }).collect();

        let tun_shadow_free = (0..n_tuns as u64).map(|t| {
            let tun_start = t * TUN_DAYS;
            tun_start < DEPTH_THRESHOLD
        }).collect();

        TunDistribution { tun_counts: counts, tun_kappa3_zero_bp, tun_shadow_free }
    }
}

// ═══════════════════════════════════════════════════════════════════
// §4  Full Katun depth report
// ═══════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct KatunDepthReport {
    /// Total events in the LongCount head's cycle.
    pub total_events: u64,
    /// Events in the shadow-free zone.
    pub shadow_free_events: u64,
    /// Events in the depth-active zone.
    pub depth_active_events: u64,
    /// Fraction of events in shadow-free zone (bp).
    pub shadow_free_fraction_bp: u64,
    /// κ₃ distribution in each zone.
    pub shadow_free_dist: KappaThreeZoneDistribution,
    pub depth_dist: KappaThreeZoneDistribution,
    /// Per-Tun analysis.
    pub tun_dist: TunDistribution,
    /// Structural prediction confirmed:
    /// shadow-free κ₃=0 fraction ≥ 99% AND depth-active κ₃ variability detected.
    pub prediction_confirmed: bool,
}

/// Compute the full Katun depth analysis from the canonical corpus.
pub fn compute_katun_depth_report(events: &EventSet, hydra: &FourCalendarHydra) -> KatunDepthReport {
    let lc_head = &hydra.long_count;
    let cycle = lc_head.signature.cycle;

    // Collect local positions within the LongCount cycle.
    let mut shadow_free_pos: Vec<u64> = Vec::new();
    let mut depth_active_pos: Vec<u64> = Vec::new();

    for ev in &events.events {
        let local = ev.days_since_epoch % cycle;
        match classify_katun_position(local) {
            KatunZone::ShadowFree => shadow_free_pos.push(local),
            KatunZone::DepthActive => depth_active_pos.push(local),
        }
    }

    let total = events.len() as u64;
    let n_sf = shadow_free_pos.len() as u64;
    let n_da = depth_active_pos.len() as u64;
    let sf_frac = if total > 0 { n_sf * 10_000 / total } else { 0 };

    let sf_dist = KappaThreeZoneDistribution::build(KatunZone::ShadowFree, &shadow_free_pos);
    let da_dist = KappaThreeZoneDistribution::build(KatunZone::DepthActive, &depth_active_pos);

    let all_positions: Vec<u64> = shadow_free_pos.iter()
        .chain(depth_active_pos.iter()).copied().collect();
    let tun_dist = TunDistribution::build(&all_positions);

    // Prediction: shadow-free has κ₃=0 always, depth has κ₃ variable.
    let sf_ok = sf_dist.kappa3_zero_fraction_bp() >= 9_900; // ≥ 99%
    let da_ok = da_dist.kappa3_zero_fraction_bp() < 9_000; // < 90% (some non-zero)

    KatunDepthReport {
        total_events: total,
        shadow_free_events: n_sf,
        depth_active_events: n_da,
        shadow_free_fraction_bp: sf_frac,
        shadow_free_dist: sf_dist,
        depth_dist: da_dist,
        tun_dist,
        prediction_confirmed: sf_ok && da_ok,
    }
}

// ═══════════════════════════════════════════════════════════════════
// §5  Tests
// ═══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EventSet;
    use crate::heads::FourCalendarHydra;

    #[test]
    fn katun_partition_constants() {
        assert_eq!(KATUN_DAYS, 7_200);
        assert_eq!(TUN_DAYS, 360);
        assert_eq!(TUNS_PER_KATUN, 20);
        assert_eq!(DEPTH_THRESHOLD, 1_331);
        // Shadow-free fraction: 1331/7200 ≈ 18.5%
        let sf_frac = DEPTH_THRESHOLD * 10_000 / KATUN_DAYS;
        assert!(sf_frac > 1_800 && sf_frac < 1_900, "Shadow-free ≈ 18.5%");
    }

    #[test]
    fn katun_zone_classification() {
        assert_eq!(classify_katun_position(0), KatunZone::ShadowFree);
        assert_eq!(classify_katun_position(1330), KatunZone::ShadowFree);
        assert_eq!(classify_katun_position(1331), KatunZone::DepthActive);
        assert_eq!(classify_katun_position(7199), KatunZone::DepthActive);
    }

    #[test]
    fn tun_index_correct() {
        assert_eq!(tun_index(0), 0);
        assert_eq!(tun_index(359), 0);
        assert_eq!(tun_index(360), 1);
        assert_eq!(tun_index(1330), 3); // floor(1330/360) = 3
        assert_eq!(tun_index(1331), 3); // still Tun 3 (depth zone starts here)
        assert_eq!(tun_index(1440), 4); // floor(1440/360) = 4
    }

    #[test]
    fn shadow_free_tuns_count() {
        // Tuns 0,1,2,3 start at 0,360,720,1080. Tun 4 starts at 1440 > 1331.
        // But wait: Tun 3 starts at 1080, ends at 1439. The depth threshold is
        // at position 1331, which falls within Tun 3.
        // So the first three COMPLETE shadow-free Tuns are 0,1,2 (ends at 1079 < 1331).
        // Tun 3 is split: days 1080-1330 are shadow-free, 1331-1439 are depth-active.
        assert_eq!(SHADOW_FREE_TUNS, 3);
        // Verify: Tun 3 start < threshold < Tun 4 start
        assert!(3 * TUN_DAYS < DEPTH_THRESHOLD);  // 1080 < 1331 ✓
        assert!(4 * TUN_DAYS > DEPTH_THRESHOLD);  // 1440 > 1331 ✓
    }

    #[test]
    fn shadow_free_zone_kappa3_always_zero() {
        // All positions 0..1330 should have κ₃ = 0.
        let p3 = SHADOW_PRIME.pow(3);
        for pos in 0u64..1331 {
            let k3 = (pos / p3) % SHADOW_PRIME;
            assert_eq!(k3, 0, "Position {pos} in shadow-free zone must have κ₃ = 0");
        }
    }

    #[test]
    fn depth_zone_kappa3_range() {
        // Positions 1331..7199: κ₃ = floor(pos/1331) % 11.
        // floor(1331/1331) = 1, floor(7199/1331) = 5.
        // So κ₃ ∈ {1, 2, 3, 4, 5} in the depth zone of one Katun.
        let p3 = SHADOW_PRIME.pow(3);
        let kappas: std::collections::HashSet<u64> = (1331u64..7200)
            .map(|pos| (pos / p3) % SHADOW_PRIME)
            .collect();
        assert!(!kappas.contains(&0), "κ₃=0 never in depth zone");
        assert!(kappas.contains(&1), "κ₃=1 appears in depth zone");
        assert!(kappas.contains(&5), "κ₃=5 appears in depth zone");
        assert!(!kappas.contains(&6), "κ₃=6 not in one-Katun depth zone");
    }

    #[test]
    fn katun_depth_report_runs() {
        let events = EventSet::canonical_corpus(0, 100_000);
        let hydra = FourCalendarHydra::canonical();
        let report = compute_katun_depth_report(&events, &hydra);
        assert!(report.total_events > 0);
        assert_eq!(report.shadow_free_events + report.depth_active_events,
                   report.total_events);
    }

    #[test]
    fn shadow_free_prediction_holds() {
        let events = EventSet::canonical_corpus(0, 200_000);
        let hydra = FourCalendarHydra::canonical();
        let report = compute_katun_depth_report(&events, &hydra);
        // Shadow-free zone must have κ₃=0 always.
        assert_eq!(
            report.shadow_free_dist.kappa3_zero_fraction_bp(),
            10_000,
            "Shadow-free zone must have κ₃=0 for 100% of events"
        );
    }

    #[test]
    fn depth_zone_has_nonzero_kappa3() {
        let events = EventSet::canonical_corpus(0, 200_000);
        let hydra = FourCalendarHydra::canonical();
        let report = compute_katun_depth_report(&events, &hydra);
        if report.depth_active_events > 0 {
            // Depth zone must have some events with κ₃ ≠ 0.
            let k3_zero = report.depth_dist.kappa3_zero_fraction_bp();
            assert!(k3_zero < 9_000,
                "Depth zone should NOT be 100% κ₃=0 (got {} bp)", k3_zero);
        }
    }

    #[test]
    fn tun_distribution_has_20_tuns() {
        let events = EventSet::canonical_corpus(0, 50_000);
        let hydra = FourCalendarHydra::canonical();
        let report = compute_katun_depth_report(&events, &hydra);
        assert_eq!(report.tun_dist.tun_counts.len(), 20);
        assert_eq!(report.tun_dist.tun_shadow_free.len(), 20);
        // First 3 Tuns are fully shadow-free
        assert!(report.tun_dist.tun_shadow_free[0]);
        assert!(report.tun_dist.tun_shadow_free[1]);
        assert!(report.tun_dist.tun_shadow_free[2]);
        // Tun 4 starts at 1440 > 1331 → depth-active
        assert!(!report.tun_dist.tun_shadow_free[4]);
    }
}
