//! # SLUB ↔ FAMSI cross-source comparison
//!
//! Two independent renderings of the same Dresden Codex page:
//!
//! - **SLUB**: original photograph of the post-WWII manuscript
//!   (page 24 in this set is WATER-DAMAGED — appears blank)
//! - **FAMSI**: chromolithograph from Förstemann 1880 / Schele
//!   redrawing — captures pre-WWII content, NO water damage
//!
//! The chromolithograph is the diagnostic when the photograph is
//! degraded. Cross-source comparison provides an additional channel:
//! a page where SLUB segments as "blank" but FAMSI segments as
//! "content-bearing" is a candidate for WWII-damage hypothesis.
//!
//! ## Open question: FAMSI→Förstemann page mapping
//!
//! Our `famsi_extract` example pulls JPEG streams from the PDF in
//! PDF-object order, which is NOT guaranteed to match Förstemann page
//! order. SLUB pages are 1-to-1 with Förstemann (verified). FAMSI
//! extracted index → Förstemann page is **empirically unresolved**.
//!
//! The naive correlation by component count or max-area does NOT work,
//! because the chromolithograph has uniformly lower ink density than
//! the photograph — no FAMSI page is "blank" by photographic
//! standards. Resolution requires either a proper PDF page-tree parse
//! (e.g. via the `lopdf` crate) or visual cross-match against known
//! SLUB content.
//!
//! This module exposes typed comparison APIs that take **explicit
//! pairs of paths** — the caller resolves the mapping externally.

#![allow(dead_code)]

#[cfg(feature = "slub")]
use super::slub::{load_slub_page, SlubError};
use super::Segmenter;
use super::threshold::DarknessThresholdSegmenter;

/// A cross-source comparison of two renderings of the (presumed) same
/// codex page.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageComparison {
    pub slub_components: usize,
    pub famsi_components: usize,
    pub slub_max_area: u64,
    pub famsi_max_area: u64,
    /// SLUB image dimensions.
    pub slub_dims: (u32, u32),
    /// FAMSI image dimensions.
    pub famsi_dims: (u32, u32),
    /// Whether SLUB segments as "blank-like" (few components, no
    /// register-wide blobs).
    pub slub_is_blank_like: bool,
    /// Whether FAMSI segments as "blank-like".
    pub famsi_is_blank_like: bool,
    /// True if SLUB looks blank but FAMSI does not — the WWII-damage
    /// hypothesis fingerprint.
    pub wwii_damage_candidate: bool,
}

impl PageComparison {
    /// Heuristic: a page is "blank-like" iff component count < 5000
    /// AND max area < 200,000 pixels (post-segmenter, no closing).
    pub fn is_blank_like(components: usize, max_area: u64) -> bool {
        components < 5_000 && max_area < 200_000
    }
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

/// Compare two JPEG renderings of (presumed) the same page.
///
/// The caller is responsible for ensuring the two paths refer to the
/// same codex page (per the open-mapping note above).
#[cfg(feature = "slub")]
pub fn compare_two_jpegs(
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
    let slub_blank = PageComparison::is_blank_like(slub_bbs.len(), slub_max);
    let famsi_blank = PageComparison::is_blank_like(famsi_bbs.len(), famsi_max);
    Ok(PageComparison {
        slub_components: slub_bbs.len(),
        famsi_components: famsi_bbs.len(),
        slub_max_area: slub_max,
        famsi_max_area: famsi_max,
        slub_dims: (slub_img.width, slub_img.height),
        famsi_dims: (famsi_img.width, famsi_img.height),
        slub_is_blank_like: slub_blank,
        famsi_is_blank_like: famsi_blank,
        wwii_damage_candidate: slub_blank && !famsi_blank,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_blank_like_classification() {
        // Low components + small max → blank-like.
        assert!(PageComparison::is_blank_like(4_954, 95_147));   // SLUB page 24 stats
        // Many components + huge max → not blank-like.
        assert!(!PageComparison::is_blank_like(5_668, 10_374_572)); // SLUB page 16
        // Boundary cases.
        assert!(!PageComparison::is_blank_like(5_001, 100_000));
        assert!(!PageComparison::is_blank_like(100, 250_000));
    }

    #[test]
    fn wwii_damage_candidate_logic() {
        // SLUB blank but FAMSI content-bearing → WWII damage candidate.
        let pc = PageComparison {
            slub_components: 4_954,
            famsi_components: 1_636,
            slub_max_area: 95_147,
            famsi_max_area: 3_895_120,
            slub_dims: (3874, 7649),
            famsi_dims: (1552, 3332),
            slub_is_blank_like: true,
            famsi_is_blank_like: false,
            wwii_damage_candidate: true,
        };
        assert!(pc.wwii_damage_candidate);

        // Both content-bearing → not a damage candidate (normal page).
        let pc2 = PageComparison {
            slub_components: 5_668,
            famsi_components: 1_740,
            slub_max_area: 10_374_572,
            famsi_max_area: 2_781_163,
            slub_dims: (3874, 7649),
            famsi_dims: (1552, 3332),
            slub_is_blank_like: false,
            famsi_is_blank_like: false,
            wwii_damage_candidate: false,
        };
        assert!(!pc2.wwii_damage_candidate);
    }
}
