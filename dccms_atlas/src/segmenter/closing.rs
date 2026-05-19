//! # Morphological closing + threshold segmenter (Area A-1)
//!
//! Wraps the basic `DarknessThresholdSegmenter` with a morphological-
//! closing pre-pass that dilates the dark mask then erodes it. Net
//! effect: small gaps within glyph clusters fill in (bar+dot fragments
//! merge into single components) while overall component shapes are
//! preserved.
//!
//! All operations are u8/u32 integer; no floats.

#![allow(dead_code)]

use super::{BoundingBox, ImageBuffer, Segmenter};
use super::threshold::DarknessThresholdSegmenter;
use std::collections::VecDeque;

/// Closing-then-connected-components segmenter.
#[derive(Clone, Copy, Debug)]
pub struct ClosingThresholdSegmenter {
    pub inner: DarknessThresholdSegmenter,
    /// Radius of the closing kernel (dilate and erode each use a
    /// `(2·radius + 1)`-wide separable box kernel). Reasonable
    /// values for the SLUB Dresden 3874×7649 images: 3-8.
    pub closing_radius: u32,
}

impl Default for ClosingThresholdSegmenter {
    fn default() -> Self {
        Self {
            inner: DarknessThresholdSegmenter::default(),
            closing_radius: 4,
        }
    }
}

impl ClosingThresholdSegmenter {
    /// Build the binary darkness mask from the image.
    fn build_mask(&self, image: &ImageBuffer) -> Vec<u8> {
        let w = image.width as usize;
        let h = image.height as usize;
        let mut mask = vec![0u8; w * h];
        let bpp = image.bytes_per_pixel as usize;
        let threshold = self.inner.darkness_threshold_sum;
        for y in 0..h {
            for x in 0..w {
                let off = (y * w + x) * bpp;
                let sum: u32 = if bpp >= 3 {
                    image.data[off] as u32
                        + image.data[off + 1] as u32
                        + image.data[off + 2] as u32
                } else if bpp == 1 {
                    (image.data[off] as u32) * 3
                } else {
                    u32::MAX
                };
                if sum < threshold { mask[y * w + x] = 1; }
            }
        }
        mask
    }

    /// Separable horizontal max-filter (dilate horizontally).
    fn dilate_horizontal(mask: &[u8], w: usize, h: usize, radius: usize) -> Vec<u8> {
        let mut out = vec![0u8; w * h];
        for y in 0..h {
            let row_base = y * w;
            for x in 0..w {
                let xs = x.saturating_sub(radius);
                let xe = (x + radius + 1).min(w);
                let mut any = 0u8;
                for xi in xs..xe {
                    if mask[row_base + xi] != 0 { any = 1; break; }
                }
                out[row_base + x] = any;
            }
        }
        out
    }

    /// Separable vertical max-filter (dilate vertically).
    fn dilate_vertical(mask: &[u8], w: usize, h: usize, radius: usize) -> Vec<u8> {
        let mut out = vec![0u8; w * h];
        for x in 0..w {
            for y in 0..h {
                let ys = y.saturating_sub(radius);
                let ye = (y + radius + 1).min(h);
                let mut any = 0u8;
                for yi in ys..ye {
                    if mask[yi * w + x] != 0 { any = 1; break; }
                }
                out[y * w + x] = any;
            }
        }
        out
    }

    /// Separable horizontal min-filter (erode horizontally).
    fn erode_horizontal(mask: &[u8], w: usize, h: usize, radius: usize) -> Vec<u8> {
        let mut out = vec![0u8; w * h];
        for y in 0..h {
            let row_base = y * w;
            for x in 0..w {
                let xs = x.saturating_sub(radius);
                let xe = (x + radius + 1).min(w);
                let mut all = 1u8;
                for xi in xs..xe {
                    if mask[row_base + xi] == 0 { all = 0; break; }
                }
                out[row_base + x] = all;
            }
        }
        out
    }

    /// Separable vertical min-filter (erode vertically).
    fn erode_vertical(mask: &[u8], w: usize, h: usize, radius: usize) -> Vec<u8> {
        let mut out = vec![0u8; w * h];
        for x in 0..w {
            for y in 0..h {
                let ys = y.saturating_sub(radius);
                let ye = (y + radius + 1).min(h);
                let mut all = 1u8;
                for yi in ys..ye {
                    if mask[yi * w + x] == 0 { all = 0; break; }
                }
                out[y * w + x] = all;
            }
        }
        out
    }

    /// Connected-components flood-fill over a closed mask.
    fn ccomp(&self, mask: &[u8], w: usize, h: usize) -> Vec<BoundingBox> {
        let total = w * h;
        let mut visited = vec![false; total];
        let mut boxes = Vec::new();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        for start_y in 0..h {
            for start_x in 0..w {
                let idx = start_y * w + start_x;
                if mask[idx] == 0 || visited[idx] { continue; }
                queue.clear();
                queue.push_back((start_x, start_y));
                visited[idx] = true;
                let (mut min_x, mut max_x) = (start_x, start_x);
                let (mut min_y, mut max_y) = (start_y, start_y);
                let mut area: u64 = 0;
                while let Some((x, y)) = queue.pop_front() {
                    area += 1;
                    if x < min_x { min_x = x; }
                    if x > max_x { max_x = x; }
                    if y < min_y { min_y = y; }
                    if y > max_y { max_y = y; }
                    let try_push = |nx: i64, ny: i64, q: &mut VecDeque<(usize, usize)>,
                                       visited: &mut [bool]| {
                        if nx < 0 || ny < 0 { return; }
                        let nx = nx as usize;
                        let ny = ny as usize;
                        if nx >= w || ny >= h { return; }
                        let ni = ny * w + nx;
                        if mask[ni] != 0 && !visited[ni] {
                            visited[ni] = true;
                            q.push_back((nx, ny));
                        }
                    };
                    try_push(x as i64 - 1, y as i64, &mut queue, &mut visited);
                    try_push(x as i64 + 1, y as i64, &mut queue, &mut visited);
                    try_push(x as i64, y as i64 - 1, &mut queue, &mut visited);
                    try_push(x as i64, y as i64 + 1, &mut queue, &mut visited);
                }
                if area >= self.inner.min_area && area <= self.inner.max_area {
                    boxes.push(BoundingBox {
                        x: min_x as u32,
                        y: min_y as u32,
                        w: (max_x - min_x + 1) as u32,
                        h: (max_y - min_y + 1) as u32,
                    });
                }
            }
        }
        boxes
    }
}

impl Segmenter for ClosingThresholdSegmenter {
    fn segment(&self, image: &ImageBuffer) -> Vec<BoundingBox> {
        if image.width == 0 || image.height == 0 { return Vec::new(); }
        let w = image.width as usize;
        let h = image.height as usize;
        let r = self.closing_radius as usize;
        let mask = self.build_mask(image);
        // Closing = dilate then erode (separable in each direction).
        let m = Self::dilate_horizontal(&mask, w, h, r);
        let m = Self::dilate_vertical(&m, w, h, r);
        let m = Self::erode_horizontal(&m, w, h, r);
        let m = Self::erode_vertical(&m, w, h, r);
        self.ccomp(&m, w, h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fragmented_glyph() -> ImageBuffer {
        // 20x20 RGB. Build a "fragmented" glyph: 4 separate 2x2 dark squares
        // arranged in a square, with 2px gaps. Naive threshold sees 4
        // components; closing should merge them into 1.
        let mut data = vec![255u8; 20 * 20 * 3];
        for (cx, cy) in [(4, 4), (4, 10), (10, 4), (10, 10)] {
            for y in cy..cy + 2 {
                for x in cx..cx + 2 {
                    let off = (y * 20 + x) * 3;
                    data[off] = 0; data[off + 1] = 0; data[off + 2] = 0;
                }
            }
        }
        ImageBuffer::new(20, 20, 3, data).unwrap()
    }

    #[test]
    fn closing_merges_fragments() {
        let img = fragmented_glyph();
        // Without closing: 4 fragments.
        let plain = DarknessThresholdSegmenter {
            darkness_threshold_sum: 100,
            min_area: 1,
            max_area: 1000,
        };
        let plain_bbs = plain.segment(&img);
        assert_eq!(plain_bbs.len(), 4, "plain should find 4 fragments");

        // With closing radius 3: fragments merge into 1.
        let closing = ClosingThresholdSegmenter {
            inner: DarknessThresholdSegmenter {
                darkness_threshold_sum: 100,
                min_area: 1,
                max_area: 1000,
            },
            closing_radius: 3,
        };
        let closed_bbs = closing.segment(&img);
        assert_eq!(closed_bbs.len(), 1, "closing should merge into 1 component");
    }

    #[test]
    fn closing_zero_radius_matches_plain() {
        let img = fragmented_glyph();
        let plain = DarknessThresholdSegmenter {
            darkness_threshold_sum: 100,
            min_area: 1,
            max_area: 1000,
        };
        let closing = ClosingThresholdSegmenter {
            inner: plain,
            closing_radius: 0,
        };
        let plain_count = plain.segment(&img).len();
        let closing_count = closing.segment(&img).len();
        assert_eq!(plain_count, closing_count);
    }

    #[test]
    fn closing_on_empty_image() {
        let img = ImageBuffer::new(0, 0, 3, vec![]).unwrap();
        let seg = ClosingThresholdSegmenter::default();
        assert!(seg.segment(&img).is_empty());
    }
}
