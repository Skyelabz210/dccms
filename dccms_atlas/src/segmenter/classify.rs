//! # Page-context glyph classifier + iconographic verification (Area B)
//!
//! ## What this is and isn't
//!
//! A real glyph classifier (template-match or trained ML) would
//! distinguish "is this bbox a MoonSign vs a WaterPot?" without
//! reference to the page number. We don't have templates or a labeled
//! corpus yet. What we DO have:
//!
//! - A typed expected-figure-per-page mapping
//!   ([`h4_visual::IconographicFigure::from_page`])
//! - Real segmenter output against real codex pages
//!
//! So instead of a true classifier, this module ships:
//!
//! 1. **`PageContextClassifier`** — a `GlyphClassifier` impl that maps
//!    any bbox to the expected figure for the page being analyzed.
//!    Tracking accountability: the classifier is honest about being
//!    page-context-based, not glyph-content-based.
//! 2. **`verify_page_iconography`** — a verification function that
//!    consumes segmenter output and reports whether the expected
//!    iconographic figure has a plausible structural footprint. This
//!    is the real value: it tests whether our framework's assertions
//!    about page-N's iconography survive contact with the actual page-N
//!    pixels.

#![allow(dead_code)]

use crate::h4_visual::iconographic::IconographicFigure;
use super::{BoundingBox, GlyphClassifier, ImageBuffer};

/// Classifier that returns the expected `IconographicFigure` for the
/// page it was constructed for, ignoring the bbox contents.
///
/// Useful for: (a) wiring the consumer side of a pipeline against the
/// existing alphabet, (b) verification tests that check whether the
/// expected figure shows up in the expected place. Not useful for:
/// (a) distinguishing between figures within a single page,
/// (b) classifying off-codex glyphs.
pub struct PageContextClassifier {
    pub page: u8,
}

impl PageContextClassifier {
    pub fn new(page: u8) -> Self { Self { page } }
}

impl GlyphClassifier for PageContextClassifier {
    type Glyph = IconographicFigure;
    fn classify(&self, _image: &ImageBuffer, _bbox: BoundingBox) -> Option<IconographicFigure> {
        IconographicFigure::from_page(self.page)
    }
}

/// Verification report for a page's iconography.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerificationReport {
    pub page: u8,
    /// The figure the H4 visual transducer says lives on this page,
    /// per `IconographicFigure::from_page`.
    pub expected_figure: Option<IconographicFigure>,
    /// Total bboxes the segmenter produced for this page.
    pub total_bboxes: usize,
    /// Count of bboxes large enough and square-ish enough to plausibly
    /// be a main iconographic figure (vs glyph fragments or barriers).
    pub figure_class_bboxes: usize,
    /// Position + size of the largest figure-class bbox, if any.
    pub largest_figure_bbox: Option<BoundingBox>,
    /// Whether the page is the BlankBridge (page 24) — different
    /// expectations apply.
    pub is_blank_bridge: bool,
    /// Whether the report is consistent with the expectation
    /// (≥ 1 figure-class bbox for figure-pages; ≤ 1 for BlankBridge).
    pub consistent: bool,
}

/// Whether a bbox plausibly belongs to a main iconographic figure.
///
/// Heuristic: area ≥ `min_figure_area` (default 50k pixels), aspect
/// ratio in `[0.4, 2.5]` (figures are roughly square-ish to portrait).
pub fn is_figure_class(b: &BoundingBox, min_figure_area: u64) -> bool {
    if b.area() < min_figure_area { return false; }
    // Aspect ratio in [0.4, 2.5]: w/h between 0.4 and 2.5, computed via
    // integer cross-multiplication to avoid floats.
    let (w, h) = (b.w as u64, b.h as u64);
    if w == 0 || h == 0 { return false; }
    // w/h >= 0.4 ⟺ 10*w >= 4*h ⟺ 5*w >= 2*h
    // w/h <= 2.5 ⟺ 2*w <= 5*h
    5 * w >= 2 * h && 2 * w <= 5 * h
}

/// Verify that a page's segmenter output is consistent with the
/// expected iconographic figure.
///
/// `min_figure_area` is the area threshold for "could plausibly be the
/// main figure" — sensible defaults for SLUB 3874×7649 imagery are
/// 50k-200k pixels.
pub fn verify_page_iconography(
    page: u8,
    bboxes: &[BoundingBox],
    min_figure_area: u64,
) -> VerificationReport {
    let expected = IconographicFigure::from_page(page);
    let is_blank_bridge = expected == Some(IconographicFigure::BlankBridge);
    let figure_class: Vec<&BoundingBox> = bboxes.iter()
        .filter(|b| is_figure_class(b, min_figure_area))
        .collect();
    let largest = figure_class.iter().copied().max_by_key(|b| b.area()).copied();
    // Consistency:
    //   - BlankBridge: expect 0 figure-class bboxes (the blank page).
    //     1 is tolerable (figure-bleed from adjacent registers can leak).
    //   - Figure page: expect ≥ 1 figure-class bbox.
    let consistent = if is_blank_bridge {
        figure_class.len() <= 1
    } else {
        figure_class.len() >= 1
    };
    VerificationReport {
        page,
        expected_figure: expected,
        total_bboxes: bboxes.len(),
        figure_class_bboxes: figure_class.len(),
        largest_figure_bbox: largest,
        is_blank_bridge,
        consistent,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_context_classifier_returns_expected_figure() {
        let img = ImageBuffer::new(10, 10, 3, vec![0u8; 300]).unwrap();
        let bbox = BoundingBox { x: 0, y: 0, w: 10, h: 10 };
        // Page 16 → MoonSign.
        let c = PageContextClassifier::new(16);
        assert_eq!(c.classify(&img, bbox), Some(IconographicFigure::MoonSign));
        // Page 24 → BlankBridge.
        let c = PageContextClassifier::new(24);
        assert_eq!(c.classify(&img, bbox), Some(IconographicFigure::BlankBridge));
        // Out-of-range page → None.
        let c = PageContextClassifier::new(99);
        assert_eq!(c.classify(&img, bbox), None);
    }

    #[test]
    fn is_figure_class_aspect_filter() {
        // 200×200 square: aspect 1.0, area 40k.
        let square = BoundingBox { x: 0, y: 0, w: 200, h: 200 };
        assert!(is_figure_class(&square, 10_000));
        assert!(!is_figure_class(&square, 100_000));   // below min_area

        // 300×100 wide (3:1 aspect): out of range.
        let wide = BoundingBox { x: 0, y: 0, w: 300, h: 100 };
        assert!(!is_figure_class(&wide, 10_000));

        // 100×300 tall (1:3 aspect): out of range.
        let tall = BoundingBox { x: 0, y: 0, w: 100, h: 300 };
        assert!(!is_figure_class(&tall, 10_000));

        // 150×100 (1.5:1): in range.
        let portrait = BoundingBox { x: 0, y: 0, w: 150, h: 100 };
        assert!(is_figure_class(&portrait, 10_000));
    }

    #[test]
    fn verify_page_16_with_synthetic_figure() {
        // Synthetic bbox set: 50 tiny glyph fragments + 1 figure-sized bbox.
        let mut bboxes = Vec::new();
        for i in 0..50 {
            bboxes.push(BoundingBox { x: i * 10, y: 0, w: 8, h: 8 });
        }
        bboxes.push(BoundingBox { x: 500, y: 1000, w: 300, h: 400 });

        let r = verify_page_iconography(16, &bboxes, 50_000);
        assert_eq!(r.expected_figure, Some(IconographicFigure::MoonSign));
        assert_eq!(r.figure_class_bboxes, 1);
        assert!(r.consistent);
        assert!(!r.is_blank_bridge);
        assert!(r.largest_figure_bbox.is_some());
    }

    #[test]
    fn verify_page_24_blank_bridge_with_zero_figures() {
        // BlankBridge expectation: zero (or at most 1) figure-class bboxes.
        let bboxes: Vec<BoundingBox> = (0..100).map(|i|
            BoundingBox { x: i * 10, y: 0, w: 8, h: 8 }  // all tiny
        ).collect();

        let r = verify_page_iconography(24, &bboxes, 50_000);
        assert_eq!(r.expected_figure, Some(IconographicFigure::BlankBridge));
        assert!(r.is_blank_bridge);
        assert_eq!(r.figure_class_bboxes, 0);
        assert!(r.consistent);
    }

    #[test]
    fn verify_page_24_inconsistent_if_figures_present() {
        // BlankBridge with multiple figure-sized bboxes should flag inconsistent.
        let bboxes = vec![
            BoundingBox { x: 0, y: 0, w: 300, h: 300 },
            BoundingBox { x: 400, y: 0, w: 300, h: 300 },
        ];
        let r = verify_page_iconography(24, &bboxes, 50_000);
        assert_eq!(r.figure_class_bboxes, 2);
        assert!(!r.consistent);
    }

    #[test]
    fn verify_page_16_inconsistent_if_zero_figures() {
        // Figure page with NO figure-class bboxes is inconsistent.
        let bboxes: Vec<BoundingBox> = (0..50).map(|i|
            BoundingBox { x: i * 10, y: 0, w: 8, h: 8 }
        ).collect();
        let r = verify_page_iconography(16, &bboxes, 50_000);
        assert_eq!(r.figure_class_bboxes, 0);
        assert!(!r.consistent);
    }

    #[test]
    fn out_of_range_page_returns_no_expected_figure() {
        let r = verify_page_iconography(99, &[], 50_000);
        assert_eq!(r.expected_figure, None);
    }
}
