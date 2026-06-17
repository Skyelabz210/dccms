//! # Pixel Ingestion Framework (C-4)
//!
//! Per executioner Directive 0: convert the apparent limit "needs SLUB
//! Dresden imagery" into a build target. The segmenter framework over
//! arbitrary byte buffers is buildable now; image-format adapters
//! (PNG/JPEG/TIFF) become drop-in implementations later.
//!
//! ## Layering
//!
//! - [`ImageBuffer`] — opaque byte-array representation of an image
//!   with width, height, and channels-per-pixel. No image-format
//!   dependency.
//! - [`BoundingBox`] — integer-pixel-coordinate region.
//! - [`Segmenter`] trait — input: image; output: `Vec<BoundingBox>`.
//! - [`GlyphClassifier`] trait — input: image + bbox; output:
//!   `Option<Glyph>` (discrete, never a confidence vector).
//!
//! ## Object-contract preservation
//!
//! Per the H4 visual transducer's discipline (see
//! `dccms_atlas::h4_visual` for the closure): the segmenter output
//! crosses the API boundary as `Vec<BoundingBox>` (discrete, integer
//! coordinates), and the classifier output crosses as `Option<Glyph>`
//! (discrete, typed). No float at any API boundary. Internal segmenter
//! arithmetic may use floats per implementation, but they must NOT
//! escape.
//!
//! ## What lives here now vs. later
//!
//! Now: the type vocabulary, the traits, a [`NullSegmenter`] and
//! [`NullClassifier`] for tests, and `ImageBuffer` round-trip.
//!
//! Later: PNG/JPEG decoders (adapters into `ImageBuffer`), a connected-
//! components segmenter for glyph detection, a template-match or
//! neural-net classifier. These are implementations of the existing
//! traits — they require external imagery + classifier-training data
//! that will arrive separately.

#![allow(dead_code)]

pub mod null;
pub mod threshold;
pub mod closing;
pub mod register;
pub mod classify;
pub mod comparison;
pub mod pipeline;     // v0.9.3 N06: end-to-end page→bbox→glyph→CRAM verification
pub mod scan;         // v0.9.6: real pixel measurements (entropy, ink density, red fraction)
pub mod crosscheck;   // v0.9.6: vault prediction vs measured pixel evidence

#[cfg(feature = "slub")]
pub mod slub;

/// An opaque byte-buffer image. Bytes are interpreted by the consumer
/// (typically `bytes_per_pixel * width` per row, no padding).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageBuffer {
    pub width: u32,
    pub height: u32,
    pub bytes_per_pixel: u32,
    pub data: Vec<u8>,
}

impl ImageBuffer {
    /// Construct from explicit dimensions and bytes.
    pub fn new(width: u32, height: u32, bytes_per_pixel: u32, data: Vec<u8>)
        -> Result<Self, ImageError>
    {
        let expected = (width as usize)
            .checked_mul(height as usize)
            .and_then(|a| a.checked_mul(bytes_per_pixel as usize))
            .ok_or(ImageError::DimensionOverflow)?;
        if data.len() != expected {
            return Err(ImageError::BufferSizeMismatch {
                expected,
                actual: data.len(),
            });
        }
        Ok(Self { width, height, bytes_per_pixel, data })
    }

    /// Byte offset for a pixel `(x, y)`. Returns `None` if out of bounds.
    pub fn pixel_offset(&self, x: u32, y: u32) -> Option<usize> {
        if x >= self.width || y >= self.height { return None; }
        let row = (y as usize) * (self.width as usize) * (self.bytes_per_pixel as usize);
        Some(row + (x as usize) * (self.bytes_per_pixel as usize))
    }

    /// Read the bytes of pixel `(x, y)`. Length = `bytes_per_pixel`.
    pub fn pixel(&self, x: u32, y: u32) -> Option<&[u8]> {
        let off = self.pixel_offset(x, y)?;
        let bpp = self.bytes_per_pixel as usize;
        Some(&self.data[off..off + bpp])
    }
}

/// Errors for image-buffer construction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImageError {
    DimensionOverflow,
    BufferSizeMismatch { expected: usize, actual: usize },
}

/// An axis-aligned bounding box in integer pixel coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BoundingBox {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl BoundingBox {
    /// Area in square pixels.
    pub fn area(&self) -> u64 {
        (self.w as u64) * (self.h as u64)
    }

    /// Whether `(px, py)` lies within this box.
    pub fn contains(&self, px: u32, py: u32) -> bool {
        px >= self.x && px < self.x + self.w
            && py >= self.y && py < self.y + self.h
    }

    /// Whether two boxes overlap at all.
    pub fn intersects(&self, other: &Self) -> bool {
        self.x < other.x + other.w
            && other.x < self.x + self.w
            && self.y < other.y + other.h
            && other.y < self.y + self.h
    }
}

/// A segmenter divides an image into glyph-candidate bounding boxes.
///
/// Implementations may use any algorithm internally; the API output
/// must be discrete integer-coordinate boxes (no confidence scores,
/// no floats).
pub trait Segmenter {
    fn segment(&self, image: &ImageBuffer) -> Vec<BoundingBox>;
}

/// A glyph classifier maps a bounding-box region of an image to a
/// discrete glyph identifier of some type. The Object-contract from
/// `h4_visual` requires the output to be a single value, not a
/// confidence vector.
pub trait GlyphClassifier {
    /// The discrete glyph alphabet this classifier produces.
    type Glyph;

    /// Classify the bbox region. `None` means "no glyph confidently
    /// identified" — the discreteness is preserved by returning a
    /// single `Glyph` value or `None`, never a probability distribution.
    fn classify(&self, image: &ImageBuffer, bbox: BoundingBox) -> Option<Self::Glyph>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small_buffer() -> ImageBuffer {
        // 2x2 RGB image (4 pixels · 3 bytes).
        ImageBuffer::new(2, 2, 3, vec![
            255, 0, 0,    // (0,0) red
            0, 255, 0,    // (1,0) green
            0, 0, 255,    // (0,1) blue
            255, 255, 0,  // (1,1) yellow
        ]).unwrap()
    }

    #[test]
    fn image_buffer_construction_validates_size() {
        // Correct size.
        let _ = small_buffer();
        // Wrong size should error.
        let bad = ImageBuffer::new(2, 2, 3, vec![0u8; 11]);
        assert!(matches!(bad, Err(ImageError::BufferSizeMismatch { .. })));
    }

    #[test]
    fn image_buffer_pixel_access() {
        let img = small_buffer();
        assert_eq!(img.pixel(0, 0), Some(&[255u8, 0, 0][..]));
        assert_eq!(img.pixel(1, 0), Some(&[0u8, 255, 0][..]));
        assert_eq!(img.pixel(0, 1), Some(&[0u8, 0, 255][..]));
        assert_eq!(img.pixel(1, 1), Some(&[255u8, 255, 0][..]));
        // Out of bounds.
        assert_eq!(img.pixel(2, 0), None);
        assert_eq!(img.pixel(0, 2), None);
    }

    #[test]
    fn bounding_box_area_contains_intersects() {
        let a = BoundingBox { x: 10, y: 20, w: 30, h: 40 };
        assert_eq!(a.area(), 1200);
        assert!(a.contains(10, 20));
        assert!(a.contains(39, 59));
        assert!(!a.contains(40, 59));  // edge exclusive
        assert!(!a.contains(10, 60));

        let b = BoundingBox { x: 35, y: 25, w: 10, h: 10 };
        assert!(a.intersects(&b));
        assert!(b.intersects(&a));

        let c = BoundingBox { x: 100, y: 100, w: 5, h: 5 };
        assert!(!a.intersects(&c));
    }

    #[test]
    fn bounding_box_zero_area() {
        let z = BoundingBox { x: 0, y: 0, w: 0, h: 0 };
        assert_eq!(z.area(), 0);
        assert!(!z.contains(0, 0));   // w=0 means no point is contained
    }
}
