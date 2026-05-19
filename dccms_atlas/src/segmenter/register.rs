//! # Register-aware segmenter wrapper (Area A-2)
//!
//! Detects red horizontal barriers — the structural register dividers
//! used throughout the Dresden Codex — and partitions the page into
//! bands. The inner `Segmenter` runs once per band, with output
//! coordinates translated back to the original image.
//!
//! ## Red detection
//!
//! A pixel `(R, G, B)` is "red" iff:
//! - `R >= red_min`, AND
//! - `R >= G + red_excess`, AND
//! - `R >= B + red_excess`.
//!
//! A row is a "barrier row" iff at least `row_fraction_per_mille / 1000`
//! of its pixels are red. Contiguous barrier rows are merged into a
//! single barrier strip.
//!
//! ## Effect
//!
//! Prevents whole-register connected-component leaks. The under-
//! segmentation pattern documented in [docs/slub_segmentation_findings.md]
//! (multi-million-pixel components on pages 15, 16, 19, 20, 23) is
//! specifically the case this wrapper addresses.

#![allow(dead_code)]

use super::{BoundingBox, ImageBuffer, Segmenter};

/// Barrier-aware segmenter wrapper.
#[derive(Clone, Debug)]
pub struct RegisterAwareSegmenter<S: Segmenter> {
    pub inner: S,
    pub red_min: u8,
    pub red_excess: u8,
    /// Threshold (parts per thousand) of red pixels in a row required
    /// to count as a barrier row.
    pub row_fraction_per_mille: u32,
}

impl<S: Segmenter> RegisterAwareSegmenter<S> {
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            red_min: 130,
            red_excess: 40,
            row_fraction_per_mille: 500,  // > 50% of row must be red
        }
    }

    /// Whether a pixel at the given byte-offset qualifies as "red".
    fn pixel_is_red(&self, data: &[u8], off: usize, bpp: usize) -> bool {
        if bpp < 3 { return false; }
        let r = data[off];
        let g = data[off + 1];
        let b = data[off + 2];
        r >= self.red_min
            && r as i16 - g as i16 >= self.red_excess as i16
            && r as i16 - b as i16 >= self.red_excess as i16
    }

    /// Identify red-barrier row indices.
    pub fn barrier_rows(&self, image: &ImageBuffer) -> Vec<u32> {
        if image.width == 0 || image.height == 0 { return Vec::new(); }
        let w = image.width as usize;
        let h = image.height as usize;
        let bpp = image.bytes_per_pixel as usize;
        let row_threshold = (w as u32 * self.row_fraction_per_mille) / 1000;
        let mut barrier_rows = Vec::new();
        for y in 0..h {
            let mut red_count: u32 = 0;
            for x in 0..w {
                let off = (y * w + x) * bpp;
                if self.pixel_is_red(&image.data, off, bpp) {
                    red_count += 1;
                }
            }
            if red_count >= row_threshold {
                barrier_rows.push(y as u32);
            }
        }
        barrier_rows
    }

    /// Identify register bands. Each band is `(y_start, y_end_exclusive)`.
    /// Barrier rows are excluded; bands are the gaps between barrier strips.
    pub fn register_bands(&self, image: &ImageBuffer) -> Vec<(u32, u32)> {
        let h = image.height;
        if h == 0 { return Vec::new(); }
        let barriers = self.barrier_rows(image);
        if barriers.is_empty() {
            return vec![(0, h)];
        }
        // Identify barrier-strip ranges (contiguous barrier rows).
        let mut barrier_strips: Vec<(u32, u32)> = Vec::new();
        let mut iter = barriers.into_iter();
        let first = iter.next().unwrap();
        let mut strip_start = first;
        let mut strip_end = first;
        for r in iter {
            if r == strip_end + 1 {
                strip_end = r;
            } else {
                barrier_strips.push((strip_start, strip_end + 1));
                strip_start = r;
                strip_end = r;
            }
        }
        barrier_strips.push((strip_start, strip_end + 1));
        // Bands are the gaps.
        let mut bands = Vec::new();
        let mut cursor: u32 = 0;
        for (bs, be) in &barrier_strips {
            if *bs > cursor {
                bands.push((cursor, *bs));
            }
            cursor = *be;
        }
        if cursor < h {
            bands.push((cursor, h));
        }
        bands
    }

    /// Slice the image to a vertical band; copy bytes (small overhead at
    /// our scale, simplicity preferred).
    fn slice_band(&self, image: &ImageBuffer, y_start: u32, y_end: u32) -> ImageBuffer {
        let w = image.width as usize;
        let bpp = image.bytes_per_pixel as usize;
        let row_bytes = w * bpp;
        let band_height = (y_end - y_start) as usize;
        let start = (y_start as usize) * row_bytes;
        let end = (y_end as usize) * row_bytes;
        ImageBuffer::new(
            image.width,
            band_height as u32,
            image.bytes_per_pixel,
            image.data[start..end].to_vec(),
        ).expect("band slice valid by construction")
    }
}

impl<S: Segmenter> Segmenter for RegisterAwareSegmenter<S> {
    fn segment(&self, image: &ImageBuffer) -> Vec<BoundingBox> {
        let bands = self.register_bands(image);
        let mut out = Vec::new();
        for (y_start, y_end) in bands {
            let band = self.slice_band(image, y_start, y_end);
            let bbs = self.inner.segment(&band);
            for b in bbs {
                out.push(BoundingBox {
                    x: b.x,
                    y: b.y + y_start,
                    w: b.w,
                    h: b.h,
                });
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::threshold::DarknessThresholdSegmenter;

    fn striped_image() -> ImageBuffer {
        // 30x30 RGB. White background, red barriers at y=10 and y=20
        // (3 px each), with one black square in each of the 3 resulting bands.
        let mut data = vec![255u8; 30 * 30 * 3];
        // Helper: set a pixel to color.
        let set = |data: &mut Vec<u8>, x: usize, y: usize, r: u8, g: u8, b: u8| {
            let off = (y * 30 + x) * 3;
            data[off] = r; data[off + 1] = g; data[off + 2] = b;
        };
        // Red barriers at y = 10..13 and y = 20..23.
        for y in 10..13 {
            for x in 0..30 { set(&mut data, x, y, 200, 30, 30); }
        }
        for y in 20..23 {
            for x in 0..30 { set(&mut data, x, y, 200, 30, 30); }
        }
        // Black squares: (5,5), (15,15), (25,25).
        for (cx, cy) in [(5usize, 5usize), (15, 15), (25, 25)] {
            for y in cy..cy + 2 {
                for x in cx..cx + 2 {
                    set(&mut data, x, y, 0, 0, 0);
                }
            }
        }
        ImageBuffer::new(30, 30, 3, data).unwrap()
    }

    #[test]
    fn detects_red_barriers() {
        let img = striped_image();
        let seg = RegisterAwareSegmenter::new(DarknessThresholdSegmenter::default());
        let rows = seg.barrier_rows(&img);
        // Should detect rows 10, 11, 12 and 20, 21, 22.
        assert_eq!(rows, vec![10u32, 11, 12, 20, 21, 22]);
    }

    #[test]
    fn partitions_into_three_bands() {
        let img = striped_image();
        let seg = RegisterAwareSegmenter::new(DarknessThresholdSegmenter::default());
        let bands = seg.register_bands(&img);
        // Bands: [0,10), [13,20), [23,30). Three bands.
        assert_eq!(bands, vec![(0, 10), (13, 20), (23, 30)]);
    }

    #[test]
    fn coordinate_translation_back_to_image_space() {
        let img = striped_image();
        let inner = DarknessThresholdSegmenter {
            darkness_threshold_sum: 100,
            min_area: 1,
            max_area: 1000,
        };
        let outer = RegisterAwareSegmenter::new(inner);
        let bbs = outer.segment(&img);
        // Expect 3 components (one per band), with original y-coordinates.
        assert_eq!(bbs.len(), 3);
        let mut by_y: Vec<&BoundingBox> = bbs.iter().collect();
        by_y.sort_by_key(|b| b.y);
        assert_eq!(by_y[0].y, 5);   // band 0, square at (5, 5)
        assert_eq!(by_y[1].y, 15);  // band 1, square at (15, 15)
        assert_eq!(by_y[2].y, 25);  // band 2, square at (25, 25)
    }

    #[test]
    fn no_barriers_yields_one_band() {
        let img = ImageBuffer::new(10, 10, 3, vec![255u8; 300]).unwrap();
        let seg = RegisterAwareSegmenter::new(DarknessThresholdSegmenter::default());
        let bands = seg.register_bands(&img);
        assert_eq!(bands, vec![(0u32, 10)]);
    }

    #[test]
    fn empty_image() {
        let img = ImageBuffer::new(0, 0, 3, vec![]).unwrap();
        let seg = RegisterAwareSegmenter::new(DarknessThresholdSegmenter::default());
        assert!(seg.segment(&img).is_empty());
    }
}
