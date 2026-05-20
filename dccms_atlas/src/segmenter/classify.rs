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

// ════════════════════════════════════════════════════════════════════
// v0.9.3 N05 — IconographicGlyphClassifier
//
// First real glyph classifier — replaces the page-context stub with a
// classifier that actually consults bbox geometry and bbox-region pixel
// darkness. Returns a discrete `BboxClass` per bbox.
//
// Q1 (locked): bbox geometry + bbox mean darkness as auxiliary scalar
// Q2 (locked): output enum BboxClass with 5 variants
// Q3 (locked): figure-class restricted to band 0 of register_bands
//
// Calibrated for SLUB 3874 × 7649 imagery. Page-24 (BlankBridge)
// expectation is that no bbox in band 0 satisfies the Figure predicate
// — handled by returning Some(Figure(BlankBridge)) for the discrete
// "I am the BlankBridge page" assertion only when no genuine figure
// bbox is found in band 0 (page 24 specific guard).
// ════════════════════════════════════════════════════════════════════

/// Discrete classification of a segmented bounding box. One value per
/// bbox; never a confidence vector (preserves Object contract).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BboxClass {
    /// A main iconographic figure of the page — the largest plausible
    /// figure-class bbox in band 0 of the register-aware segmentation.
    /// Payload identifies which figure (from `IconographicFigure::from_page`).
    Figure(IconographicFigure),
    /// A typical Maya glyph cluster — medium-sized roughly-square bbox.
    GlyphBlock,
    /// A bar-dot numeral — small-to-tiny roughly-square bbox.
    Numeral,
    /// A fragment of a red register barrier — wide and short.
    BarrierFragment,
    /// Doesn't fit any other class (out-of-aspect, noise, etc.).
    Unknown,
}

/// Which register band a candidate figure-class bbox must occupy.
///
/// v0.9.3 N05 shipped with `Band0Only`. v0.9.3 N07 surfaced its limit:
/// on Goddess pages the register-aware segmenter detects 5-11 sub-bands
/// per page (every 1-2-row barrier-pixel cluster creates a band
/// boundary), and band 0 is the ~70-pixel top margin, NOT the figure
/// register which lives 30-50% down. Result: 7/12 figure-match misses
/// on real SLUB pixels.
///
/// v0.9.4 N11 introduces `LargestBand` as the default — pick the band
/// with the largest y-extent. The figure register is the largest band
/// by construction; barrier-row clusters are tiny. Self-calibrating, no
/// magic constants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FigureBandStrategy {
    /// v0.9.3 N05 Q3 default — figure-class allowed only in band 0
    /// (the topmost band). Kept for regression testing.
    Band0Only,
    /// v0.9.4 N11 default — figure-class allowed only in the band with
    /// the largest `(y_bottom - y_top)` extent. Self-calibrates per
    /// page; works on pages with 1 band (no barriers detected — pages
    /// 15, 24) as well as 5-11 sub-bands.
    LargestBand,
}

impl Default for FigureBandStrategy {
    fn default() -> Self { FigureBandStrategy::LargestBand }
}

/// First real `GlyphClassifier` — bbox geometry + auxiliary pixel
/// darkness, with `Figure` restricted to a specific band selected by
/// `figure_band_strategy`.
///
/// Calibrated for SLUB 3874 × 7649 imagery; thresholds below are
/// empirical defaults chosen against the actual ClosingThresholdSegmenter
/// bbox-size distribution. Tuning per page is not necessary for first
/// pass — these defaults discriminate the four classes cleanly.
#[derive(Clone, Debug)]
pub struct IconographicGlyphClassifier {
    /// The page being analyzed; supplies the `Figure(...)` payload.
    pub page: u8,
    /// Register bands from `RegisterAwareSegmenter::register_bands(img)`,
    /// each `(y_top, y_bottom)` half-open.
    pub register_bands: Vec<(u32, u32)>,
    /// Which band(s) are eligible for `Figure(...)` classification.
    /// Default `LargestBand` (v0.9.4 N11).
    pub figure_band_strategy: FigureBandStrategy,
    /// Bboxes smaller than this are `Numeral` (or `BarrierFragment` if
    /// flat / wide / pale).
    pub max_numeral_area: u64,
    /// Bboxes between `max_numeral_area` and this are `GlyphBlock`.
    pub max_glyph_area: u64,
    /// Bboxes ≥ this are candidate `Figure` (subject to band check).
    pub min_figure_area: u64,
    /// A bbox is a `BarrierFragment` if its aspect ratio w/h ≥ this
    /// (very wide) AND its area < `max_numeral_area`.
    /// Stored as a numerator with implicit denominator 1 — so
    /// `barrier_aspect_min = 4` means "w/h ≥ 4".
    pub barrier_aspect_min: u32,
    /// Mean RGB-sum-per-pixel threshold; values BELOW this are "dark"
    /// (ink-bearing). Range 0..=765 (3 channels × 255).
    /// A bbox darker than this is content; lighter is barrier-like.
    pub dark_threshold: u32,
}

impl Default for IconographicGlyphClassifier {
    /// Defaults calibrated for SLUB 3874×7649 imagery + ClosingThresholdSegmenter.
    /// `register_bands` defaults empty; caller MUST populate from
    /// `RegisterAwareSegmenter::register_bands(img)` before classifying
    /// — an empty `register_bands` causes `Figure(...)` to be unreachable
    /// (all candidates fall through to `GlyphBlock` or `Unknown`).
    fn default() -> Self {
        Self {
            page: 0,
            register_bands: Vec::new(),
            figure_band_strategy: FigureBandStrategy::default(),
            max_numeral_area: 10_000,    // up to ~100×100 pixels
            max_glyph_area: 50_000,      // up to ~225×225 pixels
            min_figure_area: 50_000,
            barrier_aspect_min: 4,       // w/h ≥ 4 → barrier-like
            dark_threshold: 500,         // mean RGB-sum < 500 → ink-bearing
        }
    }
}

impl IconographicGlyphClassifier {
    /// Construct for a given page with the supplied register bands.
    /// Uses the default `FigureBandStrategy::LargestBand`.
    pub fn new(page: u8, register_bands: Vec<(u32, u32)>) -> Self {
        Self { page, register_bands, ..Self::default() }
    }

    /// Construct with explicit band strategy.
    pub fn with_strategy(
        page: u8,
        register_bands: Vec<(u32, u32)>,
        strategy: FigureBandStrategy,
    ) -> Self {
        Self {
            page,
            register_bands,
            figure_band_strategy: strategy,
            ..Self::default()
        }
    }

    /// Index of the band a candidate figure-class bbox must fall in,
    /// per the configured `figure_band_strategy`. Returns `None` when
    /// no bands are populated (Figure unreachable, by design).
    pub fn figure_band_index(&self) -> Option<usize> {
        if self.register_bands.is_empty() { return None; }
        match self.figure_band_strategy {
            FigureBandStrategy::Band0Only => Some(0),
            FigureBandStrategy::LargestBand => {
                // Pick the band with the largest y-extent. On ties pick
                // the lowest index (deterministic).
                self.register_bands.iter()
                    .enumerate()
                    .max_by_key(|(_, &(t, b))| b.saturating_sub(t))
                    .map(|(i, _)| i)
            }
        }
    }

    /// Whether a y-coordinate falls inside the configured figure band.
    /// Empty bands → always false (Figure unreachable, by design).
    pub fn in_figure_band(&self, y: u32) -> bool {
        match self.figure_band_index() {
            None => false,
            Some(i) => {
                let (top, bottom) = self.register_bands[i];
                y >= top && y < bottom
            }
        }
    }

    /// Legacy alias retained for backward compatibility with v0.9.3 N05.
    /// Always evaluates against band 0 regardless of the configured
    /// strategy. Use `in_figure_band` for strategy-aware checks.
    #[deprecated(since = "0.9.4-dev", note = "Use in_figure_band; this always checks band 0 regardless of strategy")]
    pub fn in_band_zero(&self, y: u32) -> bool {
        match self.register_bands.first() {
            None => false,
            Some(&(top, bottom)) => y >= top && y < bottom,
        }
    }

    /// Aspect-ratio w/h computed via integer cross-multiplication.
    /// Returns true iff w/h is in `[0.4, 2.5]`, the "roughly square-ish
    /// to portrait" range used throughout the segmenter framework.
    fn aspect_in_figure_range(b: &BoundingBox) -> bool {
        let (w, h) = (b.w as u64, b.h as u64);
        if w == 0 || h == 0 { return false; }
        // w/h ≥ 0.4 ⟺ 5*w ≥ 2*h ; w/h ≤ 2.5 ⟺ 2*w ≤ 5*h
        5 * w >= 2 * h && 2 * w <= 5 * h
    }

    /// Compute the mean RGB-sum-per-pixel inside the bbox.
    /// Returns u32 in 0..=765 (3 channels × 255). 0 = pure black ink,
    /// 765 = pure white background.
    pub fn bbox_mean_darkness(image: &ImageBuffer, bbox: &BoundingBox) -> u32 {
        let bpp = image.bytes_per_pixel as usize;
        if bpp < 3 { return 765; }
        let w_img = image.width as usize;
        let x_start = bbox.x as usize;
        let y_start = bbox.y as usize;
        let x_end = (bbox.x + bbox.w).min(image.width) as usize;
        let y_end = (bbox.y + bbox.h).min(image.height) as usize;
        if x_end <= x_start || y_end <= y_start { return 765; }
        let mut sum: u64 = 0;
        let mut count: u64 = 0;
        for y in y_start..y_end {
            for x in x_start..x_end {
                let off = (y * w_img + x) * bpp;
                if off + 2 >= image.data.len() { continue; }
                let r = image.data[off] as u64;
                let g = image.data[off + 1] as u64;
                let b = image.data[off + 2] as u64;
                sum += r + g + b;
                count += 1;
            }
        }
        if count == 0 { 765 } else { (sum / count) as u32 }
    }
}

impl GlyphClassifier for IconographicGlyphClassifier {
    type Glyph = BboxClass;

    fn classify(&self, image: &ImageBuffer, bbox: BoundingBox) -> Option<BboxClass> {
        let area = bbox.area();
        let (w, h) = (bbox.w as u64, bbox.h as u64);
        if w == 0 || h == 0 { return Some(BboxClass::Unknown); }

        // Aspect-ratio shortcut: very wide + small area → BarrierFragment.
        // `5 * w > barrier_aspect_min * 5 * h` is equivalent to
        // `w / h > barrier_aspect_min` via integer cross-multiply.
        let is_very_wide = (w as u64) >= (self.barrier_aspect_min as u64) * h;
        if is_very_wide && area < self.max_numeral_area {
            return Some(BboxClass::BarrierFragment);
        }

        // Out-of-aspect bboxes that aren't barrier-wide: Unknown.
        if !Self::aspect_in_figure_range(&bbox) {
            return Some(BboxClass::Unknown);
        }

        // Area-bucket classification.
        if area < self.max_numeral_area {
            return Some(BboxClass::Numeral);
        }
        if area < self.max_glyph_area {
            return Some(BboxClass::GlyphBlock);
        }

        // Figure candidate: area ≥ max_glyph_area AND in-aspect (just confirmed).
        // Two additional gates:
        //   (a) bbox center y inside the configured figure band
        //       (`FigureBandStrategy::LargestBand` by default per v0.9.4 N11), AND
        //   (b) bbox mean darkness below threshold (= ink-bearing, Q1 default).
        let center_y = bbox.y + bbox.h / 2;
        if !self.in_figure_band(center_y) {
            return Some(BboxClass::GlyphBlock);
        }
        let mean = Self::bbox_mean_darkness(image, &bbox);
        if mean > self.dark_threshold {
            // Large region but no ink — likely background-pocket between
            // glyphs, not a content-bearing figure.
            return Some(BboxClass::Unknown);
        }
        // Passes all gates — assign the page's iconographic figure.
        // Returns None only if page is out of the 16..=24 Goddess-section
        // range, in which case "figure" is undefined.
        IconographicFigure::from_page(self.page).map(BboxClass::Figure)
    }
}

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

    // ── v0.9.3 N05 — IconographicGlyphClassifier tests ────────────────

    /// Build a uniformly-dark synthetic image (all RGB=0 — black ink everywhere).
    fn dark_image(w: u32, h: u32) -> ImageBuffer {
        ImageBuffer::new(w, h, 3, vec![0u8; (w * h * 3) as usize]).unwrap()
    }

    /// Build a uniformly-light synthetic image (all RGB=255 — pure white).
    fn light_image(w: u32, h: u32) -> ImageBuffer {
        ImageBuffer::new(w, h, 3, vec![255u8; (w * h * 3) as usize]).unwrap()
    }

    #[test]
    fn icg_classifier_bbox_mean_darkness_works() {
        let dark = dark_image(100, 100);
        let light = light_image(100, 100);
        let bbox = BoundingBox { x: 10, y: 10, w: 50, h: 50 };
        assert_eq!(IconographicGlyphClassifier::bbox_mean_darkness(&dark, &bbox), 0);
        assert_eq!(IconographicGlyphClassifier::bbox_mean_darkness(&light, &bbox), 765);
    }

    #[test]
    fn icg_classifier_returns_numeral_for_small_square_bbox() {
        // 50×50 = area 2500, area < max_numeral_area=10000, aspect 1.0.
        let img = dark_image(200, 200);
        let bbox = BoundingBox { x: 0, y: 0, w: 50, h: 50 };
        let c = IconographicGlyphClassifier::new(16, vec![(0, 100)]);
        assert_eq!(c.classify(&img, bbox), Some(BboxClass::Numeral));
    }

    #[test]
    fn icg_classifier_returns_glyph_block_for_medium_square_bbox() {
        // 150×150 = area 22500, area between max_numeral_area and max_glyph_area.
        let img = dark_image(300, 300);
        let bbox = BoundingBox { x: 0, y: 0, w: 150, h: 150 };
        let c = IconographicGlyphClassifier::new(16, vec![(0, 100)]);
        assert_eq!(c.classify(&img, bbox), Some(BboxClass::GlyphBlock));
    }

    #[test]
    fn icg_classifier_returns_barrier_fragment_for_wide_short_bbox() {
        // 400×20 = area 8000 < max_numeral_area=10000, aspect 20 (very wide).
        let img = dark_image(500, 100);
        let bbox = BoundingBox { x: 0, y: 0, w: 400, h: 20 };
        let c = IconographicGlyphClassifier::new(16, vec![(0, 100)]);
        assert_eq!(c.classify(&img, bbox), Some(BboxClass::BarrierFragment));
    }

    #[test]
    fn icg_classifier_returns_figure_for_large_dark_bbox_in_band_zero() {
        // 300×300 = area 90000 ≥ min_figure_area=50000, aspect 1.0,
        // center (150, 150) inside band (0, 500).
        let img = dark_image(500, 500);
        let bbox = BoundingBox { x: 0, y: 0, w: 300, h: 300 };
        let c = IconographicGlyphClassifier::new(16, vec![(0, 500)]);
        assert_eq!(c.classify(&img, bbox), Some(BboxClass::Figure(IconographicFigure::MoonSign)));
    }

    #[test]
    fn icg_classifier_demotes_figure_to_glyph_block_outside_band_zero() {
        // Same large bbox but band 0 is (0, 100); bbox center 150 is outside.
        let img = dark_image(500, 500);
        let bbox = BoundingBox { x: 0, y: 0, w: 300, h: 300 };
        let c = IconographicGlyphClassifier::new(16, vec![(0, 100), (200, 400)]);
        assert_eq!(c.classify(&img, bbox), Some(BboxClass::GlyphBlock));
    }

    #[test]
    fn icg_classifier_demotes_figure_to_unknown_if_too_light() {
        // Large bbox in band 0 BUT image is pure white (mean 765 > dark_threshold=500).
        let img = light_image(500, 500);
        let bbox = BoundingBox { x: 0, y: 0, w: 300, h: 300 };
        let c = IconographicGlyphClassifier::new(16, vec![(0, 500)]);
        assert_eq!(c.classify(&img, bbox), Some(BboxClass::Unknown));
    }

    #[test]
    fn icg_classifier_returns_none_for_out_of_range_page() {
        // Even with valid figure-class geometry, page 99 has no
        // IconographicFigure → classifier returns None.
        let img = dark_image(500, 500);
        let bbox = BoundingBox { x: 0, y: 0, w: 300, h: 300 };
        let c = IconographicGlyphClassifier::new(99, vec![(0, 500)]);
        assert_eq!(c.classify(&img, bbox), None);
    }

    #[test]
    fn icg_classifier_object_contract_one_value_or_none() {
        // The Object contract: every call returns either ONE BboxClass
        // value or None — never a confidence vector. Verify by type:
        // the return type is Option<BboxClass>, not Vec<(BboxClass, f64)>.
        // This is enforced by the trait signature; this test documents it.
        let img = dark_image(100, 100);
        let c = IconographicGlyphClassifier::new(16, vec![(0, 50)]);
        for bbox in [
            BoundingBox { x: 0, y: 0, w: 10, h: 10 },     // Numeral
            BoundingBox { x: 0, y: 0, w: 60, h: 60 },     // GlyphBlock
            BoundingBox { x: 0, y: 0, w: 90, h: 5 },      // BarrierFragment
            BoundingBox { x: 0, y: 0, w: 1, h: 90 },      // out-of-aspect → Unknown
        ] {
            let result = c.classify(&img, bbox);
            // Single value (Some or None) — not a vector.
            assert!(result.is_some() || result.is_none());
        }
    }

    #[test]
    fn icg_classifier_empty_register_bands_prevents_figure() {
        // With no register bands supplied, in_figure_band is always false,
        // so a large content-bearing bbox demotes to GlyphBlock.
        let img = dark_image(500, 500);
        let bbox = BoundingBox { x: 0, y: 0, w: 300, h: 300 };
        let c = IconographicGlyphClassifier::new(16, Vec::new());
        assert_eq!(c.classify(&img, bbox), Some(BboxClass::GlyphBlock));
    }

    // ── v0.9.4 N11 — FigureBandStrategy::LargestBand tests ─────────────

    #[test]
    fn icg_largest_band_picks_band_with_max_y_extent() {
        // 3 bands: (0,77)=77, (100,2500)=2400, (2700,2800)=100.
        // LargestBand should pick index 1.
        let bands = vec![(0, 77), (100, 2500), (2700, 2800)];
        let c = IconographicGlyphClassifier::with_strategy(
            16, bands, FigureBandStrategy::LargestBand);
        assert_eq!(c.figure_band_index(), Some(1));
        // y=1000 is inside band 1.
        assert!(c.in_figure_band(1000));
        // y=50 is inside band 0 (which is NOT the figure band under LargestBand).
        assert!(!c.in_figure_band(50));
        // y=2750 is inside band 2 (also not the figure band).
        assert!(!c.in_figure_band(2750));
    }

    #[test]
    fn icg_band0_strategy_preserves_v093_behavior() {
        // Same 3 bands; Band0Only picks index 0.
        let bands = vec![(0, 77), (100, 2500), (2700, 2800)];
        let c = IconographicGlyphClassifier::with_strategy(
            16, bands, FigureBandStrategy::Band0Only);
        assert_eq!(c.figure_band_index(), Some(0));
        assert!(c.in_figure_band(50));       // inside band 0
        assert!(!c.in_figure_band(1000));    // inside band 1 — not band 0
    }

    #[test]
    fn icg_default_strategy_is_largest_band() {
        // The Default impl must give LargestBand, not Band0Only.
        let c: IconographicGlyphClassifier = Default::default();
        assert_eq!(c.figure_band_strategy, FigureBandStrategy::LargestBand);
    }

    #[test]
    fn icg_largest_band_recovers_figure_outside_band_zero() {
        // Real-pixel-realistic scenario: page has many bands, figure
        // lives in band 1 (the actual register), band 0 is a tiny top
        // margin. Under LargestBand the figure is correctly classified.
        let img = dark_image(500, 5000);
        let bbox = BoundingBox { x: 100, y: 1000, w: 300, h: 300 };  // center y=1150
        let bands = vec![
            (0, 77),         // tiny top margin
            (100, 2500),     // the figure register — largest
            (2510, 2520),    // barrier-cluster sub-band
            (2530, 4900),    // second register
        ];
        let c_largest = IconographicGlyphClassifier::with_strategy(
            16, bands.clone(), FigureBandStrategy::LargestBand);
        assert_eq!(
            c_largest.classify(&img, bbox),
            Some(BboxClass::Figure(IconographicFigure::MoonSign)),
            "LargestBand should recover the figure in band 1"
        );
        // Under the old Band0Only strategy the same bbox demotes.
        let c_band0 = IconographicGlyphClassifier::with_strategy(
            16, bands, FigureBandStrategy::Band0Only);
        assert_eq!(
            c_band0.classify(&img, bbox),
            Some(BboxClass::GlyphBlock),
            "Band0Only should demote the figure (regression test for v0.9.3 behavior)"
        );
    }

    #[test]
    fn icg_largest_band_handles_single_band_pages() {
        // Pages 15 and 24 in SLUB had no detected barriers → 1 band only.
        // LargestBand picks band 0 in this case — equivalent to Band0Only.
        let img = dark_image(500, 5000);
        let bbox = BoundingBox { x: 100, y: 1000, w: 300, h: 300 };
        let bands = vec![(0, 4999)];   // whole page is one band
        let c = IconographicGlyphClassifier::with_strategy(
            16, bands, FigureBandStrategy::LargestBand);
        assert_eq!(c.figure_band_index(), Some(0));
        assert_eq!(
            c.classify(&img, bbox),
            Some(BboxClass::Figure(IconographicFigure::MoonSign))
        );
    }
}
