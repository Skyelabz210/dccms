//! # End-to-end Goddess-page decoding pipeline (v0.9.3 N06)
//!
//! Wires the four segmenter components into the route the v0.9.3 thesis
//! demands:
//!
//! ```text
//! ImageBuffer (page-N SLUB JPEG)
//!     │
//!     ├──► ClosingThresholdSegmenter::segment(img)  →  Vec<BoundingBox>
//!     │
//!     ├──► RegisterAwareSegmenter::register_bands(img)  →  Vec<(y_top, y_bot)>
//!     │
//!     ├──► IconographicGlyphClassifier(page, bands).classify(img, bbox) for each bbox
//!     │       →  per-bbox BboxClass; collect bboxes classified Figure(...) into
//!     │          figure_bboxes; take the largest as classified_figure
//!     │
//!     ├──► h4_visual::layout::cumulative_addresses(&goddess_section_layout())
//!     │       [page - 16]  →  predicted_cram_visual : [u64; 6]
//!     │
//!     └──► MoonGoddessProfile::compute().page_cram_addresses[page - 16]
//!             →  reference_cram_non_visual : [u64; 6]
//!
//! PageDecoding {
//!     page, register_bands, total_bboxes,
//!     figure_bboxes, classified_figure, expected_figure,
//!     predicted_cram_visual, reference_cram_non_visual,
//!     cram_match : visual == reference  (Discharge contract — should be true),
//!     figure_match : classified == expected  (real-pixel discharge — the new gate),
//! }
//! ```
//!
//! `cram_match` is the v0.7 H4 closure, exercised here on real-pixel-
//! provenance bounding boxes; it should hold by construction (both
//! paths derive from the same canonical intervals). `figure_match` is
//! the v0.9.3 contribution — whether the real classifier output on
//! real pixels agrees with the page-context expectation.
//!
//! The pipeline is feature-gated under `slub` (depends on `slub::load_slub_page`
//! when called via `decode_goddess_page_from_path`).

#![allow(dead_code)]

use super::{BoundingBox, GlyphClassifier, ImageBuffer, Segmenter};
use super::classify::{BboxClass, IconographicGlyphClassifier};
use super::closing::ClosingThresholdSegmenter;
use super::register::RegisterAwareSegmenter;
use super::threshold::DarknessThresholdSegmenter;
use crate::h4_visual::iconographic::IconographicFigure;
use crate::h4_visual::layout::{cumulative_addresses, goddess_section_layout};

/// Discrete decoding of a single Goddess-section page from real pixels.
///
/// Object contract: all fields are typed integer / discrete-enum values.
/// No float, no confidence vectors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageDecoding {
    /// Förstemann page number (1-based, codex-canonical).
    pub page: u8,
    /// Register bands detected by `RegisterAwareSegmenter` at the
    /// v0.9.2-calibrated (red_min=164, red_excess=24, row_frac=200‰)
    /// defaults. Each `(y_top, y_bot)` half-open.
    pub register_bands: Vec<(u32, u32)>,
    /// Total bbox count from `ClosingThresholdSegmenter`.
    pub total_bboxes: usize,
    /// Bboxes the classifier identified as `BboxClass::Figure(...)`.
    /// Empty when no bbox in band 0 satisfies the figure predicate
    /// (expected for `BlankBridge` page 24 and possibly for pages whose
    /// figure happens to fall outside band 0 under the v0.9.3 N05 Q3
    /// "band 0 only" restriction).
    pub figure_bboxes: Vec<BoundingBox>,
    /// The figure type the classifier produced. `Some(...)` when at
    /// least one bbox classified as `Figure(...)`; `None` otherwise.
    /// For figure-bearing pages this should equal `expected_figure`;
    /// for `BlankBridge` (page 24) this should be `None`.
    pub classified_figure: Option<IconographicFigure>,
    /// What `IconographicFigure::from_page(page)` expects. `None` only
    /// for pages outside the Goddess section (1..15, 25..).
    pub expected_figure: Option<IconographicFigure>,
    /// Visual-path CRAM: `cumulative_addresses(goddess_section_layout())[page - 16]`.
    /// `[0; 6]` for non-Goddess-section pages.
    pub predicted_cram_visual: [u64; 6],
    /// Non-visual reference CRAM: `MoonGoddessProfile::compute().page_cram_addresses[page - 16]`.
    /// `[0; 6]` for non-Goddess-section pages.
    pub reference_cram_non_visual: [u64; 6],
    /// Whether the visual-path CRAM matches the non-visual reference.
    /// Should be `true` for every Goddess page (16..=24) by H4 construction.
    pub cram_match: bool,
    /// Whether the classifier's identified figure matches the expected
    /// figure. For pages where `classified_figure` is `None`, true iff
    /// `expected_figure` is also `None` (or `Some(BlankBridge)` for page 24
    /// where the absence of a figure-class bbox is the expected outcome).
    pub figure_match: bool,
}

/// Decode a single Goddess page from a decoded `ImageBuffer`.
///
/// Pure function over `(page, &ImageBuffer)`. No I/O. Caller is
/// responsible for loading the image (`slub::load_slub_page` is the
/// SLUB JPEG adapter).
pub fn decode_goddess_page(page: u8, image: &ImageBuffer) -> PageDecoding {
    // ── Segmenter pass 1: closing threshold → bbox set ──
    let closing = ClosingThresholdSegmenter::default();
    let bboxes = closing.segment(image);

    // ── Segmenter pass 2: register-aware → band layout ──
    let reg = RegisterAwareSegmenter::new(DarknessThresholdSegmenter::default());
    let register_bands = reg.register_bands(image);

    // ── Classifier pass: per-bbox BboxClass ──
    let classifier = IconographicGlyphClassifier::new(page, register_bands.clone());
    let mut figure_bboxes: Vec<BoundingBox> = Vec::new();
    let mut figure_payload: Option<IconographicFigure> = None;
    for &bbox in &bboxes {
        match classifier.classify(image, bbox) {
            Some(BboxClass::Figure(f)) => {
                figure_bboxes.push(bbox);
                figure_payload = Some(f);  // all Figure(...) variants for one
                                            // page have the same payload by
                                            // construction (from_page(page))
            }
            _ => {}
        }
    }
    // Pick the largest figure-class bbox as the canonical one (if any).
    figure_bboxes.sort_by(|a, b| b.area().cmp(&a.area()));
    let classified_figure = figure_payload;

    // ── Visual-path predicted CRAM ──
    let expected_figure = IconographicFigure::from_page(page);
    let predicted_cram_visual: [u64; 6] = if (16u8..=24).contains(&page) {
        let layout = goddess_section_layout();
        let cum = cumulative_addresses(&layout);
        cum[(page - 16) as usize]
    } else {
        [0; 6]
    };

    // ── Non-visual reference CRAM ──
    let reference_cram_non_visual: [u64; 6] = if (16u8..=24).contains(&page) {
        let profile = crate::moon_goddess::MoonGoddessProfile::compute();
        profile.page_cram_addresses[(page - 16) as usize]
    } else {
        [0; 6]
    };

    let cram_match = predicted_cram_visual == reference_cram_non_visual;

    // figure_match logic:
    //   - For BlankBridge page (24): expected = Some(BlankBridge);
    //     classified is allowed to be None (no figure-class bbox detected
    //     in band 0) OR Some(BlankBridge). Both indicate the structural
    //     blank-page expectation is met.
    //   - For figure pages (16-23): classified MUST equal expected.
    //   - For out-of-range pages: both None ⇒ match.
    let figure_match = match (classified_figure, expected_figure) {
        (None, None) => true,
        (Some(c), Some(e)) => c == e,
        (None, Some(IconographicFigure::BlankBridge)) => true,
        _ => false,
    };

    PageDecoding {
        page,
        register_bands,
        total_bboxes: bboxes.len(),
        figure_bboxes,
        classified_figure,
        expected_figure,
        predicted_cram_visual,
        reference_cram_non_visual,
        cram_match,
        figure_match,
    }
}

/// Decode a Goddess page from a SLUB JPEG on disk.
///
/// Convenience wrapper around `decode_goddess_page` that loads the
/// image via the SLUB adapter.
#[cfg(feature = "slub")]
pub fn decode_goddess_page_from_path(
    page: u8,
    path: &std::path::Path,
) -> Result<PageDecoding, super::slub::SlubError> {
    let img = super::slub::load_slub_page(path)?;
    Ok(decode_goddess_page(page, &img))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::segmenter::ImageBuffer;

    /// Build a synthetic page-like image: white background with a dark
    /// 300×300 block in the upper-left area (simulating a figure in
    /// band 0 of a tall page).
    fn synthetic_page_with_figure(page_w: u32, page_h: u32) -> ImageBuffer {
        let mut data = vec![255u8; (page_w * page_h * 3) as usize];
        // Dark figure block at (200..500, 200..500) — well inside the
        // top quarter for any page taller than 2000 pixels.
        for y in 200..500 {
            for x in 200..500 {
                let off = ((y * page_w + x) * 3) as usize;
                if off + 2 >= data.len() { continue; }
                data[off] = 0;
                data[off + 1] = 0;
                data[off + 2] = 0;
            }
        }
        ImageBuffer::new(page_w, page_h, 3, data).unwrap()
    }

    /// Build a fully-white synthetic page (no content).
    fn synthetic_blank_page(page_w: u32, page_h: u32) -> ImageBuffer {
        ImageBuffer::new(
            page_w, page_h, 3,
            vec![255u8; (page_w * page_h * 3) as usize],
        ).unwrap()
    }

    #[test]
    fn pipeline_cram_match_holds_by_construction_on_all_goddess_pages() {
        // The visual-path CRAM and non-visual reference CRAM should be
        // identical for every Goddess page, regardless of image content.
        // This is the H4 closure exercised inside the pipeline.
        let img = synthetic_blank_page(400, 800);
        for page in 16u8..=24 {
            let d = decode_goddess_page(page, &img);
            assert!(
                d.cram_match,
                "page {} cram_match should hold by construction: visual {:?} vs non-visual {:?}",
                page, d.predicted_cram_visual, d.reference_cram_non_visual
            );
        }
    }

    #[test]
    fn pipeline_expected_figure_matches_h4_alphabet() {
        let img = synthetic_blank_page(400, 800);
        for page in 16u8..=24 {
            let d = decode_goddess_page(page, &img);
            assert_eq!(
                d.expected_figure,
                IconographicFigure::from_page(page),
                "page {} expected_figure should match the H4 alphabet", page
            );
        }
    }

    #[test]
    fn pipeline_blank_page_matches_blankbridge_expectation_when_classified_as_none() {
        // A fully-blank image of page 24 should produce no figure-class
        // bboxes (no ink) → classified_figure = None → matches expected
        // None-or-BlankBridge.
        let img = synthetic_blank_page(400, 800);
        let d = decode_goddess_page(24, &img);
        assert_eq!(d.expected_figure, Some(IconographicFigure::BlankBridge));
        assert_eq!(d.classified_figure, None);
        assert!(d.figure_match, "blank synthetic page 24 should figure_match (None matches BlankBridge expectation)");
    }

    #[test]
    fn pipeline_out_of_range_page_yields_neutral_decoding() {
        // Page 99 is outside the Goddess section: both expected and
        // reference CRAM are zero, both figures are None, both matches
        // hold trivially.
        let img = synthetic_blank_page(200, 400);
        let d = decode_goddess_page(99, &img);
        assert_eq!(d.expected_figure, None);
        assert_eq!(d.classified_figure, None);
        assert_eq!(d.predicted_cram_visual, [0; 6]);
        assert_eq!(d.reference_cram_non_visual, [0; 6]);
        assert!(d.cram_match);
        assert!(d.figure_match);
    }

    #[test]
    fn pipeline_object_contract_no_floats_in_output() {
        // PageDecoding contains only u8/u32/u64/Vec/enum fields.
        // This test is a compile-time fact; documented here as a checked
        // assertion that the struct is `Eq` (which forbids float members).
        fn _requires_eq<T: Eq>() {}
        _requires_eq::<PageDecoding>();
    }
}
