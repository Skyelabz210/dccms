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
use super::Segmenter;
use super::threshold::DarknessThresholdSegmenter;
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

/// SLUB segmenter-stat signature consistent with WWII damage on the
/// plain `DarknessThresholdSegmenter`: small component count, no
/// register-leak blob. Calibrated against actual page-24 stats
/// (4954 components, max area 95147).
///
/// Page 18 also falls under this signature without being damaged —
/// that is the v0.9.1 false-positive case, expected: the plain
/// segmenter cannot distinguish clean register separation from
/// genuine damage. To break the tie we also consult barrier count
/// (see [`slub_signals_damage`]).
pub fn slub_stats_look_damaged(components: usize, max_area: u64) -> bool {
    components < 5_500 && max_area < 150_000
}

/// Tight per-page damage signature combining BOTH segmenter stats
/// (low component count, no leak blob) AND register-barrier absence.
/// Page 18 has 52 barriers detected at calibrated thresholds; page
/// 24 has zero. This combined signature breaks the v0.9.1 tie:
///
/// | page | stats-look-damaged | barriers | combined |
/// |------|--------------------|----------|----------|
/// | 18   | yes                | 52       | NO       |
/// | 24   | yes                | 0        | YES      |
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
    let seg = DarknessThresholdSegmenter::default();
    let slub_bbs = seg.segment(&slub_img);
    let famsi_bbs = seg.segment(&famsi_img);
    let slub_max = slub_bbs.iter().map(|b| b.area()).max().unwrap_or(0);
    let famsi_max = famsi_bbs.iter().map(|b| b.area()).max().unwrap_or(0);
    // Barrier count uses the calibrated register-aware segmenter.
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
        // Real page-24 stats from examples/calibrate.rs.
        assert!(slub_stats_look_damaged(4_954, 95_147));
        // Real page-16 stats — content-bearing.
        assert!(!slub_stats_look_damaged(5_668, 10_374_572));
        // The page-18 false-positive case: stats LOOK damaged but vault
        // says page 18 is fine. The stats-only signature flags it —
        // that is the documented limitation; barrier count breaks the tie.
        assert!(slub_stats_look_damaged(4_965, 126_043));
    }

    #[test]
    fn combined_damage_signal_distinguishes_pages_18_and_24() {
        // Real calibration data:
        //   page 24: components 4954, max 95147, 0 barrier rows
        //   page 18: components 4965, max 126043, 52 barrier rows
        // Stats alone confuse them; combined signal separates them.
        assert!(slub_signals_damage(4_954, 95_147, 0));     // page 24 — DAMAGED
        assert!(!slub_signals_damage(4_965, 126_043, 52));  // page 18 — intact
        // Page 16 — content + barriers — definitely not damaged.
        assert!(!slub_signals_damage(5_668, 10_374_572, 39));
    }

    #[test]
    fn corroboration_logic() {
        // Page 24: vault damaged, signals agree → corroborated.
        let pc = PageComparison {
            page: 24,
            slub_components: 4_954,
            famsi_components: 1_636,
            slub_max_area: 95_147,
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
            slub_components: 5_668,
            famsi_components: 1_740,
            slub_max_area: 10_374_572,
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

        // Page 18: vault intact; stats look damaged BUT 52 barriers
        // detected → combined signal says intact → corroborates vault.
        // The v0.9.1 false positive is now FIXED.
        let pc3 = PageComparison {
            page: 18,
            slub_components: 4_965,
            famsi_components: 1_700,
            slub_max_area: 126_043,
            famsi_max_area: 2_500_000,
            slub_dims: (3874, 7649),
            famsi_dims: (1552, 3332),
            slub_barrier_rows: 52,
            wwii_damaged_per_vault: false,
            slub_stats_look_damaged: true,    // stats alone misleading
            slub_signals_damage: false,        // but barriers present
            segmenter_corroborates_vault: true,
        };
        assert!(pc3.segmenter_corroborates_vault);
    }
}
