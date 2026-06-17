//! # Real pixel measurements for one Dresden Codex page (v0.9.6).
//!
//! `scan_page` computes honest, image-derived statistics for any page
//! 1–74: Shannon entropy, ink density, red-ink fraction, and segmenter
//! blob counts. These are the Class-B inputs listed in `EVIDENCE_AND_THESIS.md`.
//!
//! ## Float discipline
//!
//! Shannon entropy requires logarithms. The crate-wide
//! `#![deny(clippy::float_arithmetic)]` is **overridden here** via the
//! module-level allow below, but floats do NOT cross the public API
//! boundary: every field of `PageScan` is an integer-scaled value
//! (permille for ratios, millibits for entropy). The exact-integer core
//! is unaffected.

#![allow(dead_code)]
#![allow(clippy::float_arithmetic)]

use super::closing::ClosingThresholdSegmenter;
use super::register::RegisterAwareSegmenter;
use super::threshold::DarknessThresholdSegmenter;
use super::{ImageBuffer, Segmenter};

// ── Pixel-classification thresholds ──────────────────────────────────────────

/// Luminance (max channel) below which a pixel counts as "ink."
/// Value 180 captures dark red barrier ink (calibrated median max ≈ 164)
/// and pure-black glyph ink; cream paper (max ≈ 230+) is excluded.
const INK_THRESHOLD: u8 = 180;

/// Minimum red channel for "red ink" classification.
/// Matches `RegisterAwareSegmenter::red_min` (calibrated against real SLUB
/// barrier pixels in `examples/calibrate.rs` Job 1).
const RED_MIN: u8 = 164;

/// Minimum excess of R over G and B for "red ink."
/// Matches `RegisterAwareSegmenter::red_excess` (calibrated same source).
const RED_EXCESS: u8 = 24;

// ── Public types ─────────────────────────────────────────────────────────────

/// Real pixel measurements for one page of the Dresden Codex.
///
/// Object contract: all fields are integer-scaled — no floats escape the
/// module. Ratios use permille (× 1000); entropy uses millibits (× 1000).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageScan {
    /// Förstemann page number (1–74).
    pub page: u8,
    /// Image width in pixels.
    pub width: u32,
    /// Image height in pixels.
    pub height: u32,
    /// Ink-covered pixels as a fraction of total pixels, in permille (0–1000).
    /// A pixel is "ink" when max(R,G,B) < 180.
    pub ink_density_permille: u32,
    /// Red pixels as a fraction of total pixels, in permille (0–1000).
    /// A pixel is "red" when R ≥ 164 AND R–G ≥ 24 AND R–B ≥ 24.
    /// Captures barrier rows AND red numerals (the K-elimination signal).
    pub red_permille: u32,
    /// Shannon entropy of the per-pixel luminance (max channel) histogram,
    /// scaled to millibits (entropy_bits × 1000). Range 0–8000.
    pub entropy_millibits: u32,
    /// Connected-component count from `ClosingThresholdSegmenter`.
    pub component_count: usize,
    /// Largest component area in pixels.
    pub max_component_area: u64,
    /// Red horizontal barrier row count from `RegisterAwareSegmenter`.
    pub barrier_rows: usize,
}

// ── Public functions ──────────────────────────────────────────────────────────

/// Compute real pixel measurements for a decoded codex page image.
///
/// No I/O. Caller loads the image (e.g. `slub::load_slub_page`).
/// Works for any page 1–74 — no Goddess-section restriction.
pub fn scan_page(page: u8, image: &ImageBuffer) -> PageScan {
    let total_pixels = (image.width as u64) * (image.height as u64);
    let bpp = image.bytes_per_pixel as usize;

    let mut lum_hist = [0u64; 256];
    let mut ink_pixels: u64 = 0;
    let mut red_pixels: u64 = 0;

    // Single-pass over the flat pixel buffer: luminance histogram + ink/red counts.
    // Iterating chunks is much faster than calling image.pixel() per pixel.
    for chunk in image.data.chunks(bpp) {
        let (r, g, b) = if bpp >= 3 {
            (chunk[0], chunk[1], chunk[2])
        } else {
            let v = chunk[0];
            (v, v, v)
        };
        let lum = r.max(g).max(b);
        lum_hist[lum as usize] += 1;

        if lum < INK_THRESHOLD {
            ink_pixels += 1;
        }

        // Red detection: channel dominance, brightness-agnostic.
        if r >= RED_MIN
            && (r as i16 - g as i16) >= RED_EXCESS as i16
            && (r as i16 - b as i16) >= RED_EXCESS as i16
        {
            red_pixels += 1;
        }
    }

    let ink_density_permille = if total_pixels > 0 {
        (ink_pixels * 1000 / total_pixels) as u32
    } else {
        0
    };

    let red_permille = if total_pixels > 0 {
        (red_pixels * 1000 / total_pixels) as u32
    } else {
        0
    };

    let entropy_millibits = entropy_millibits(&lum_hist, total_pixels);

    // Reuse existing segmenter infrastructure for blob stats.
    let closing = ClosingThresholdSegmenter::default();
    let bboxes = closing.segment(image);
    let component_count = bboxes.len();
    let max_component_area = bboxes.iter().map(|b| b.area()).max().unwrap_or(0);

    let reg = RegisterAwareSegmenter::new(DarknessThresholdSegmenter::default());
    let barrier_rows = reg.barrier_rows(image).len();

    PageScan {
        page,
        width: image.width,
        height: image.height,
        ink_density_permille,
        red_permille,
        entropy_millibits,
        component_count,
        max_component_area,
        barrier_rows,
    }
}

// ── Private helpers ───────────────────────────────────────────────────────────

/// Shannon entropy of a 256-bin luminance histogram, scaled to millibits.
/// Float arithmetic is contained here and does not escape.
fn entropy_millibits(hist: &[u64; 256], total: u64) -> u32 {
    if total == 0 {
        return 0;
    }
    let n = total as f64;
    let mut h: f64 = 0.0;
    for &count in hist {
        if count == 0 {
            continue;
        }
        let p = count as f64 / n;
        h -= p * p.log2();
    }
    // h ∈ [0.0, 8.0] bits; scale to millibits and clamp
    ((h * 1000.0).round() as u32).min(8_000)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::segmenter::ImageBuffer;

    fn solid(w: u32, h: u32, r: u8, g: u8, b: u8) -> ImageBuffer {
        let data: Vec<u8> = std::iter::repeat([r, g, b])
            .take((w * h) as usize)
            .flatten()
            .collect();
        ImageBuffer::new(w, h, 3, data).unwrap()
    }

    fn checker_bw(w: u32, h: u32) -> ImageBuffer {
        let mut data = vec![0u8; (w * h * 3) as usize];
        for y in 0..h {
            for x in 0..w {
                let off = ((y * w + x) * 3) as usize;
                let v = if (x + y) % 2 == 0 { 255u8 } else { 0u8 };
                data[off] = v;
                data[off + 1] = v;
                data[off + 2] = v;
            }
        }
        ImageBuffer::new(w, h, 3, data).unwrap()
    }

    #[test]
    fn solid_white_zero_ink_zero_entropy() {
        let s = scan_page(1, &solid(50, 50, 255, 255, 255));
        assert_eq!(s.ink_density_permille, 0, "white: no ink");
        assert_eq!(s.entropy_millibits, 0, "uniform: zero entropy");
        assert_eq!(s.red_permille, 0, "white: no red");
    }

    #[test]
    fn solid_black_full_ink_zero_entropy() {
        let s = scan_page(1, &solid(50, 50, 0, 0, 0));
        assert_eq!(s.ink_density_permille, 1000, "black: full ink");
        assert_eq!(s.entropy_millibits, 0, "uniform: zero entropy");
        assert_eq!(s.red_permille, 0, "black is not red");
    }

    #[test]
    fn checker_half_ink_one_bit_entropy() {
        let s = scan_page(1, &checker_bw(100, 100));
        // Two equally-likely symbols → H = 1 bit = 1000 millibits
        assert!(
            s.ink_density_permille > 450 && s.ink_density_permille < 550,
            "checker: ~500‰ ink, got {}",
            s.ink_density_permille
        );
        assert!(
            s.entropy_millibits > 900 && s.entropy_millibits < 1100,
            "checker: ~1000 millibits entropy, got {}",
            s.entropy_millibits
        );
    }

    #[test]
    fn red_detection_matches_register_calibration() {
        // Dark brownish-red matching SLUB calibration: R=170, G=130, B=120
        // R(170) >= RED_MIN(164) ✓, R-G=40 >= RED_EXCESS(24) ✓, R-B=50 >= 24 ✓
        // lum=170 < INK_THRESHOLD(180) → also counted as ink
        let s = scan_page(1, &solid(40, 40, 170, 130, 120));
        assert!(
            s.red_permille > 900,
            "dark brownish-red: high red_permille, got {}",
            s.red_permille
        );
        assert!(
            s.ink_density_permille > 900,
            "dark brownish-red: also ink, got {}",
            s.ink_density_permille
        );

        // Pure black — not red (R=0 < RED_MIN=164)
        let s2 = scan_page(1, &solid(40, 40, 0, 0, 0));
        assert_eq!(s2.red_permille, 0, "black is not red");
    }

    #[test]
    fn bright_red_not_red_ink_when_below_red_min() {
        // R=100, G=10, B=10: dominance ratio fine but R < RED_MIN(164)
        let s = scan_page(1, &solid(40, 40, 100, 10, 10));
        assert_eq!(s.red_permille, 0, "R < RED_MIN → not classified as red");
    }

    #[test]
    fn page_number_preserved() {
        let s = scan_page(52, &solid(10, 10, 200, 200, 200));
        assert_eq!(s.page, 52);
    }

    #[test]
    fn dimensions_match_image() {
        let s = scan_page(1, &solid(60, 80, 200, 200, 200));
        assert_eq!(s.width, 60);
        assert_eq!(s.height, 80);
    }

    #[test]
    fn struct_is_eq_no_floats_in_output() {
        // PageScan is Eq, which requires all fields to be Eq.
        // float fields would violate this at compile time.
        fn _requires_eq<T: Eq>() {}
        _requires_eq::<PageScan>();
    }
}
