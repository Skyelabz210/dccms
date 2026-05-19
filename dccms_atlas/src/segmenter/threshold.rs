//! # Darkness-threshold connected-components segmenter
//!
//! A classical CV segmenter for codex page imagery. No ML, no floats,
//! pure-integer pipeline:
//!
//! 1. **Darken-test**: a pixel `(R, G, B)` is **dark** iff
//!    `R + G + B < DARKNESS_THRESHOLD_SUM`.
//! 2. **Connected-components labeling**: BFS flood-fill over the dark mask,
//!    4-connectivity. Each connected dark region becomes one candidate.
//! 3. **Size filter**: drop components with `area < MIN_AREA` (noise) or
//!    `area > MAX_AREA` (background bleed / page border).
//! 4. **Bounding box**: each surviving component produces one
//!    [`super::BoundingBox`].
//!
//! All output is discrete integer coordinates — Object contract preserved.

#![allow(dead_code)]

use super::{BoundingBox, ImageBuffer, Segmenter};
use std::collections::VecDeque;

/// Default darkness threshold (sum of R+G+B). Dresden Codex pages have
/// cream/yellow background pixels summing ~600-720; dark ink pixels
/// typically sum below 350.
pub const DEFAULT_DARKNESS_THRESHOLD_SUM: u32 = 350;

/// Default minimum component area (pixels). Drops single-pixel noise.
pub const DEFAULT_MIN_AREA: u64 = 25;

/// Default maximum component area (pixels). Drops the page-frame
/// background should it ever pass the threshold.
pub const DEFAULT_MAX_AREA: u64 = 2_000_000;

/// Connected-components segmenter parameters.
#[derive(Clone, Copy, Debug)]
pub struct DarknessThresholdSegmenter {
    pub darkness_threshold_sum: u32,
    pub min_area: u64,
    pub max_area: u64,
}

impl Default for DarknessThresholdSegmenter {
    fn default() -> Self {
        Self {
            darkness_threshold_sum: DEFAULT_DARKNESS_THRESHOLD_SUM,
            min_area: DEFAULT_MIN_AREA,
            max_area: DEFAULT_MAX_AREA,
        }
    }
}

impl DarknessThresholdSegmenter {
    /// Sum of the first three bytes at pixel `(x, y)` (R+G+B for typical
    /// RGB images; the same byte-sum predicate works for any 3-channel
    /// representation where dark pixels have low component values).
    fn pixel_sum(img: &ImageBuffer, x: u32, y: u32) -> u32 {
        let off = match img.pixel_offset(x, y) {
            Some(o) => o,
            None => return u32::MAX,  // out of bounds: treat as bright
        };
        let bpp = img.bytes_per_pixel as usize;
        if bpp < 3 {
            // Greyscale fallback: triple the single channel value.
            let v = img.data[off] as u32;
            return v * 3;
        }
        let r = img.data[off]     as u32;
        let g = img.data[off + 1] as u32;
        let b = img.data[off + 2] as u32;
        r + g + b
    }
}

impl Segmenter for DarknessThresholdSegmenter {
    fn segment(&self, image: &ImageBuffer) -> Vec<BoundingBox> {
        if image.width == 0 || image.height == 0 {
            return Vec::new();
        }
        let w = image.width as usize;
        let h = image.height as usize;
        let total = w * h;
        // Build dark mask (one byte per pixel).
        let mut dark = vec![0u8; total];
        for y in 0..image.height {
            for x in 0..image.width {
                if Self::pixel_sum(image, x, y) < self.darkness_threshold_sum {
                    dark[(y as usize) * w + (x as usize)] = 1;
                }
            }
        }
        // Connected-components via BFS flood-fill. visited[i] tracks pixels
        // already assigned to a component.
        let mut visited = vec![false; total];
        let mut boxes = Vec::new();
        let mut queue: VecDeque<(u32, u32)> = VecDeque::new();
        for start_y in 0..image.height {
            for start_x in 0..image.width {
                let idx = (start_y as usize) * w + (start_x as usize);
                if dark[idx] == 0 || visited[idx] { continue; }
                // Flood-fill from (start_x, start_y).
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
                    // 4-connectivity neighbors.
                    let neighbors: [(i64, i64); 4] = [
                        (x as i64 - 1, y as i64),
                        (x as i64 + 1, y as i64),
                        (x as i64, y as i64 - 1),
                        (x as i64, y as i64 + 1),
                    ];
                    for (nx, ny) in neighbors {
                        if nx < 0 || ny < 0 { continue; }
                        if nx >= image.width as i64 || ny >= image.height as i64 { continue; }
                        let nidx = (ny as usize) * w + (nx as usize);
                        if dark[nidx] != 0 && !visited[nidx] {
                            visited[nidx] = true;
                            queue.push_back((nx as u32, ny as u32));
                        }
                    }
                }
                // Size-filter, then emit bbox.
                if area >= self.min_area && area <= self.max_area {
                    boxes.push(BoundingBox {
                        x: min_x,
                        y: min_y,
                        w: max_x - min_x + 1,
                        h: max_y - min_y + 1,
                    });
                }
            }
        }
        boxes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a small synthetic test image: a white 16x16 RGB canvas
    /// with two dark 4x4 squares at (2,2) and (10,10).
    fn synthetic_two_blobs() -> ImageBuffer {
        let mut data = vec![255u8; 16 * 16 * 3];
        for y in 2..6 {
            for x in 2..6 {
                let off = (y * 16 + x) * 3;
                data[off] = 0; data[off + 1] = 0; data[off + 2] = 0;
            }
        }
        for y in 10..14 {
            for x in 10..14 {
                let off = (y * 16 + x) * 3;
                data[off] = 0; data[off + 1] = 0; data[off + 2] = 0;
            }
        }
        ImageBuffer::new(16, 16, 3, data).unwrap()
    }

    #[test]
    fn synthetic_two_blobs_detected() {
        let img = synthetic_two_blobs();
        let seg = DarknessThresholdSegmenter {
            darkness_threshold_sum: 100,
            min_area: 4,
            max_area: 1000,
        };
        let bbs = seg.segment(&img);
        assert_eq!(bbs.len(), 2, "expected exactly 2 connected dark components");
        // Each blob is 4x4 = 16 pixels.
        for b in &bbs {
            assert_eq!(b.w, 4);
            assert_eq!(b.h, 4);
            assert_eq!(b.area(), 16);
        }
    }

    #[test]
    fn min_area_filter_drops_noise() {
        let mut img = synthetic_two_blobs();
        // Add a single dark pixel at (8, 8).
        let off = (8 * 16 + 8) * 3;
        img.data[off] = 0; img.data[off + 1] = 0; img.data[off + 2] = 0;
        let seg = DarknessThresholdSegmenter {
            darkness_threshold_sum: 100,
            min_area: 4,
            max_area: 1000,
        };
        let bbs = seg.segment(&img);
        // Single pixel (area 1) dropped; two 4x4 blobs remain.
        assert_eq!(bbs.len(), 2);
    }

    #[test]
    fn max_area_filter_drops_background() {
        // Build an all-dark image (entire 16x16 is one giant component).
        let img = ImageBuffer::new(16, 16, 3, vec![0u8; 16 * 16 * 3]).unwrap();
        let seg = DarknessThresholdSegmenter {
            darkness_threshold_sum: 100,
            min_area: 1,
            max_area: 50,  // 256 > 50 → dropped
        };
        let bbs = seg.segment(&img);
        assert!(bbs.is_empty(), "single 256-pixel component should exceed max_area");
    }

    #[test]
    fn empty_image_yields_no_boxes() {
        let img = ImageBuffer::new(0, 0, 3, vec![]).unwrap();
        let seg = DarknessThresholdSegmenter::default();
        assert!(seg.segment(&img).is_empty());
    }

    #[test]
    fn all_bright_image_yields_no_boxes() {
        let img = ImageBuffer::new(20, 20, 3, vec![255u8; 20 * 20 * 3]).unwrap();
        let seg = DarknessThresholdSegmenter::default();
        assert!(seg.segment(&img).is_empty());
    }

    #[test]
    fn grayscale_fallback_works() {
        // 8x8 single-channel image with a dark cross.
        let mut data = vec![255u8; 64];
        // Vertical line at x=3.
        for y in 1..7 { data[y * 8 + 3] = 0; }
        // Horizontal line at y=3.
        for x in 1..7 { data[3 * 8 + x] = 0; }
        let img = ImageBuffer::new(8, 8, 1, data).unwrap();
        let seg = DarknessThresholdSegmenter {
            darkness_threshold_sum: 100,
            min_area: 1,
            max_area: 1000,
        };
        let bbs = seg.segment(&img);
        // The cross is one connected component.
        assert_eq!(bbs.len(), 1);
        assert_eq!(bbs[0].x, 1);
        assert_eq!(bbs[0].y, 1);
        assert_eq!(bbs[0].w, 6);
        assert_eq!(bbs[0].h, 6);
    }
}
