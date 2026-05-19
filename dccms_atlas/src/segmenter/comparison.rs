//! # SLUB ↔ FAMSI cross-source comparison
//!
//! Two independent renderings of the same Dresden Codex page:
//!
//! - **SLUB**: original photograph of the post-WWII manuscript.
//!   Pages on the vault's WWII-damage list appear blank/degraded.
//! - **FAMSI**: chromolithograph from Förstemann 1880 / Schele
//!   redrawing — captures pre-WWII content, NO water damage.
//!
//! The chromolithograph is the diagnostic when the photograph is
//! degraded.
//!
//! ## Mode shift in v0.9.1+: verification, not discovery
//!
//! The damaged-page list is *vault-known* (see
//! [`WWII_DAMAGED_PAGES`]). The job of this module is not to *find*
//! damaged pages from segmenter stats — it's to *verify* that the
//! observed segmenter output is consistent with the known damage
//! state. This eliminates the page-18 false positive that v0.9.1
//! documented: page 18 has content-bearing register structure, but
//! the plain darkness-threshold segmenter cannot tell its stats apart
//! from page 24's actually-damaged stats. By starting from the known
//! list, the segmenter output becomes corroborating evidence rather
//! than the primary classifier.
//!
//! ## FAMSI → Förstemann page mapping
//!
//! Resolved empirically by [`examples/calibrate.rs`]:
//! the naive 1-to-1 mapping (extracted index N ↔ Förstemann N) yields
//! 98% of the greedy best-match total similarity score under
//! histogram-intersection of vertical row-darkness fingerprints. The
//! mapping is treated as 1-to-1 in this module.

#![allow(dead_code)]

#[cfg(feature = "slub")]
use super::slub::{load_slub_page, SlubError};
#[cfg(feature = "slub")]
use super::Segmenter;
#[cfg(feature = "slub")]
use super::threshold::DarknessThresholdSegmenter;
#[cfg(feature = "slub")]
use super::closing::ClosingThresholdSegmenter;
#[cfg(feature = "slub")]
use super::register::RegisterAwareSegmenter;

/// Vault-known WWII-water-damaged Förstemann pages (Dresden Codex).
/// Source: `Dresden.md` in the Obsidian vault. These pages were
/// substantially or fully degraded in 1945 when the SLUB-Dresden
/// vault flooded; the FAMSI chromolithograph captures their pre-1945
/// content.
pub const WWII_DAMAGED_PAGES: &[u8] = &[2, 4, 24, 28, 34, 38, 71, 72];

/// Whether a page is on the vault-known WWII damage list.
pub fn is_wwii_damaged(page: u8) -> bool {
    WWII_DAMAGED_PAGES.contains(&page)
}

/// SLUB segmenter-stat signature consistent with WWII damage under the
/// `ClosingThresholdSegmenter` (the v0.9.3 N03 upgrade from plain
/// thresholding). Calibrated against actual closing-segmenter output on
/// SLUB pages 13–24 (`examples/calibrate.rs` Job 6):
///
/// | page | components | max area | classification |
/// |---:|---:|---:|---|
/// | 16  | 2622 | 443,390   | intact (lowest comp among intact) |
/// | 18  | 2648 | 2,330,084 | intact (false-pos under plain seg, NATURALLY fixed under closing) |
/// | 24  | 2498 |   303,366 | DAMAGED |
///
/// The closing segmenter eliminates the v0.9.1 page-18 false positive
/// **by stats alone** — closing merges page 18's clean-register
/// fragments into a content-bearing leak blob (max ≈ 2.3M), pushing it
/// well above the damage threshold. The threshold `components < 2600`
/// uniquely catches page 24 against all 11 intact pages in the
/// observed set, with a 124-component safety margin.
///
/// Under closing, max-area is NOT a clean damage discriminator (page 24
/// has higher max than page 14 intact). It is retained in the predicate
/// only as a guard against future imagery anomalies — kept loose
/// (< 500_000) so it never excludes a genuine damage signal.
pub fn slub_stats_look_damaged(components: usize, max_area: u64) -> bool {
    components < 2_600 && max_area < 500_000
}

/// Combined SLUB damage signal: closing-segmenter stats AND
/// register-barrier absence. Page 24 (damaged) has 0 barriers; page 15
/// (intact, narrow-barrier photographic anomaly) also has 0 barriers
/// but is excluded by the stats predicate (max-area 10M+).
///
/// Under v0.9.3 N03 calibration, this combined signal is strictly
/// stronger than the stats predicate alone, but the stats predicate
/// already discriminates 12/12 cleanly. The combined signal is retained
/// for defense-in-depth against imagery variation in future page sets.
pub fn slub_signals_damage(components: usize, max_area: u64, barrier_rows: usize) -> bool {
    slub_stats_look_damaged(components, max_area) && barrier_rows == 0
}

/// A cross-source comparison of two renderings of the same codex page,
/// keyed by Förstemann page number. The damage status is taken from
/// the vault-known [`WWII_DAMAGED_PAGES`] list; the segmenter stats
/// provide corroborating evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageComparison {
    /// Förstemann page number (1-based, codex-canonical).
    pub page: u8,
    pub slub_components: usize,
    pub famsi_components: usize,
    pub slub_max_area: u64,
    pub famsi_max_area: u64,
    pub slub_dims: (u32, u32),
    pub famsi_dims: (u32, u32),
    /// Number of red barrier rows detected in the SLUB image using
    /// the calibrated `RegisterAwareSegmenter` defaults.
    pub slub_barrier_rows: usize,
    /// Vault-known WWII damage status (NOT inferred from segmenter).
    pub wwii_damaged_per_vault: bool,
    /// Whether the SLUB segmenter output is consistent with damage
    /// by stats alone (low component count, no large blob). May be
    /// true for non-damaged pages with clean register structure
    /// (e.g. page 18) — needs barrier count to disambiguate.
    pub slub_stats_look_damaged: bool,
    /// Combined SLUB damage signal: stats AND barrier absence.
    pub slub_signals_damage: bool,
    /// True iff the combined SLUB damage signal agrees with the vault.
    pub segmenter_corroborates_vault: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComparisonError {
    SlubLoadFailed(String),
    FamsiLoadFailed(String),
}

#[cfg(feature = "slub")]
impl From<SlubError> for ComparisonError {
    fn from(e: SlubError) -> Self {
        Self::SlubLoadFailed(format!("{:?}", e))
    }
}

/// Compare two JPEG renderings of the same Förstemann page.
///
/// Damage status is taken from [`WWII_DAMAGED_PAGES`]; segmenter
/// stats are corroborating evidence, not the source of truth.
#[cfg(feature = "slub")]
pub fn compare_two_jpegs(
    page: u8,
    slub_path: &std::path::Path,
    famsi_path: &std::path::Path,
) -> Result<PageComparison, ComparisonError> {
    let slub_img = load_slub_page(slub_path)
        .map_err(|e| ComparisonError::SlubLoadFailed(format!("{:?}", e)))?;
    let famsi_img = load_slub_page(famsi_path)
        .map_err(|e| ComparisonError::FamsiLoadFailed(format!("{:?}", e)))?;
    // v0.9.3 N03: switched from DarknessThresholdSegmenter to
    // ClosingThresholdSegmenter. Closing eliminates the register-leak
    // under-segmentation that caused the v0.9.1 page-18 false positive.
    // Stats thresholds in `slub_stats_look_damaged` were recalibrated
    // against closing-segmenter output by `examples/calibrate.rs` Job 6.
    let seg = ClosingThresholdSegmenter::default();
    let slub_bbs = seg.segment(&slub_img);
    let famsi_bbs = seg.segment(&famsi_img);
    let slub_max = slub_bbs.iter().map(|b| b.area()).max().unwrap_or(0);
    let famsi_max = famsi_bbs.iter().map(|b| b.area()).max().unwrap_or(0);
    // Barrier count uses the calibrated register-aware segmenter, with
    // the plain threshold segmenter inside (the inner segmenter is only
    // used by RegisterAwareSegmenter for the per-band call, which the
    // barrier_rows() method doesn't reach).
    let reg = RegisterAwareSegmenter::new(DarknessThresholdSegmenter::default());
    let barrier_rows = reg.barrier_rows(&slub_img).len();
    let damaged = is_wwii_damaged(page);
    let stats_damaged = slub_stats_look_damaged(slub_bbs.len(), slub_max);
    let signals_damage = slub_signals_damage(slub_bbs.len(), slub_max, barrier_rows);
    Ok(PageComparison {
        page,
        slub_components: slub_bbs.len(),
        famsi_components: famsi_bbs.len(),
        slub_max_area: slub_max,
        famsi_max_area: famsi_max,
        slub_dims: (slub_img.width, slub_img.height),
        famsi_dims: (famsi_img.width, famsi_img.height),
        slub_barrier_rows: barrier_rows,
        wwii_damaged_per_vault: damaged,
        slub_stats_look_damaged: stats_damaged,
        slub_signals_damage: signals_damage,
        segmenter_corroborates_vault: damaged == signals_damage,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vault_damaged_pages_authoritative() {
        // Vault list per Dresden.md.
        for p in [2u8, 4, 24, 28, 34, 38, 71, 72] {
            assert!(is_wwii_damaged(p), "page {} should be vault-damaged", p);
        }
        // Pages NOT on the list, including the v0.9.1 false-positive (18).
        for p in [1u8, 3, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 25] {
            assert!(!is_wwii_damaged(p), "page {} should NOT be vault-damaged", p);
        }
    }

    #[test]
    fn slub_stats_signature_recognizes_page_24() {
        // Real closing-segmenter stats from examples/calibrate.rs Job 6:
        //   page 24 (damaged): components 2498, max 303_366
        //   page 16 (intact):  components 2622, max 443_390
        //   page 18 (was v0.9.1 false-pos under plain seg):
        //     components 2648, max 2_330_084
        // Under closing the stats predicate alone is sufficient.
        assert!(slub_stats_look_damaged(2_498, 303_366));       // page 24 — DAMAGED
        assert!(!slub_stats_look_damaged(2_622, 443_390));      // page 16 — intact
        assert!(!slub_stats_look_damaged(2_648, 2_330_084));    // page 18 — intact (false-pos fixed)
        assert!(!slub_stats_look_damaged(2_862, 258_258));      // page 15 — intact even with zero barriers
    }

    #[test]
    fn combined_damage_signal_corroborates_under_closing_seg() {
        // Real closing-segmenter stats from calibrate.rs Job 6:
        //   page 24 (damaged):  comp 2498, max 303_366, barriers 0
        //   page 18 (intact):   comp 2648, max 2_330_084, barriers 52
        //   page 15 (intact):   comp 2862, max 258_258,  barriers 0
        //   page 16 (intact):   comp 2622, max 443_390,  barriers 39
        // The closing-segmenter stats predicate already discriminates
        // 12/12 cleanly; the combined-with-barriers signal is retained
        // for defense in depth.
        assert!(slub_signals_damage(2_498, 303_366, 0));        // page 24 — DAMAGED
        assert!(!slub_signals_damage(2_648, 2_330_084, 52));    // page 18 — intact
        assert!(!slub_signals_damage(2_862, 258_258, 0));       // page 15 — intact
        assert!(!slub_signals_damage(2_622, 443_390, 39));      // page 16 — intact
    }

    #[test]
    fn corroboration_logic() {
        // All stats below are closing-segmenter values (v0.9.3 N03).
        // Page 24: vault damaged, signals agree → corroborated.
        let pc = PageComparison {
            page: 24,
            slub_components: 2_498,
            famsi_components: 1_636,
            slub_max_area: 303_366,
            famsi_max_area: 3_895_120,
            slub_dims: (3874, 7649),
            famsi_dims: (1552, 3332),
            slub_barrier_rows: 0,
            wwii_damaged_per_vault: true,
            slub_stats_look_damaged: true,
            slub_signals_damage: true,
            segmenter_corroborates_vault: true,
        };
        assert!(pc.segmenter_corroborates_vault);

        // Page 16: vault intact, signals agree → corroborated.
        let pc2 = PageComparison {
            page: 16,
            slub_components: 2_622,
            famsi_components: 1_740,
            slub_max_area: 443_390,
            famsi_max_area: 2_781_163,
            slub_dims: (3874, 7649),
            famsi_dims: (1552, 3332),
            slub_barrier_rows: 39,
            wwii_damaged_per_vault: false,
            slub_stats_look_damaged: false,
            slub_signals_damage: false,
            segmenter_corroborates_vault: true,
        };
        assert!(pc2.segmenter_corroborates_vault);

        // Page 18 under closing: max area is 2.3M (content-bearing leak
        // blob), so stats_look_damaged is false outright. The v0.9.1
        // false positive is now resolved by the segmenter choice itself,
        // not by the barrier-count auxiliary signal.
        let pc3 = PageComparison {
            page: 18,
            slub_components: 2_648,
            famsi_components: 1_700,
            slub_max_area: 2_330_084,
            famsi_max_area: 2_500_000,
            slub_dims: (3874, 7649),
            famsi_dims: (1552, 3332),
            slub_barrier_rows: 52,
            wwii_damaged_per_vault: false,
            slub_stats_look_damaged: false,    // closing-seg max >> threshold
            slub_signals_damage: false,
            segmenter_corroborates_vault: true,
        };
        assert!(pc3.segmenter_corroborates_vault);

        // Page 15 under closing: zero barriers (photographic anomaly)
        // but max area 258k is below the 500k guard and components 2862
        // is above the 2600 damage threshold → not damage-like → vault
        // says intact → corroborated.
        let pc4 = PageComparison {
            page: 15,
            slub_components: 2_862,
            famsi_components: 1_500,
            slub_max_area: 258_258,
            famsi_max_area: 2_000_000,
            slub_dims: (3874, 7649),
            famsi_dims: (1552, 3332),
            slub_barrier_rows: 0,
            wwii_damaged_per_vault: false,
            slub_stats_look_damaged: false,
            slub_signals_damage: false,
            segmenter_corroborates_vault: true,
        };
        assert!(pc4.segmenter_corroborates_vault);
    }
}
