//! # Null implementations for the segmenter framework
//!
//! Trivial-but-typed `Segmenter` and `GlyphClassifier` implementations
//! useful as test scaffolding and as the "no-op" reference. Real
//! segmenter implementations (e.g., a connected-components detector or
//! a neural-net classifier) will live in sibling modules when they
//! arrive — these stay as the reference null behavior.

#![allow(dead_code)]

use super::{BoundingBox, GlyphClassifier, ImageBuffer, Segmenter};

/// A segmenter that returns one bounding box covering the entire image.
pub struct NullSegmenter;

impl Segmenter for NullSegmenter {
    fn segment(&self, image: &ImageBuffer) -> Vec<BoundingBox> {
        if image.width == 0 || image.height == 0 {
            Vec::new()
        } else {
            vec![BoundingBox {
                x: 0, y: 0, w: image.width, h: image.height,
            }]
        }
    }
}

/// A classifier that always returns `None` (no glyph identified).
///
/// Useful as a typed default. Real classifiers replace this when
/// trained data is available.
pub struct NullClassifier<G> {
    _phantom: std::marker::PhantomData<G>,
}

impl<G> Default for NullClassifier<G> {
    fn default() -> Self {
        Self { _phantom: std::marker::PhantomData }
    }
}

impl<G> GlyphClassifier for NullClassifier<G> {
    type Glyph = G;
    fn classify(&self, _image: &ImageBuffer, _bbox: BoundingBox) -> Option<G> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::h4_visual::dayname::DayNameGlyph;

    #[test]
    fn null_segmenter_returns_one_full_box() {
        let img = ImageBuffer::new(10, 20, 3, vec![0u8; 600]).unwrap();
        let bbs = NullSegmenter.segment(&img);
        assert_eq!(bbs.len(), 1);
        assert_eq!(bbs[0], BoundingBox { x: 0, y: 0, w: 10, h: 20 });
    }

    #[test]
    fn null_segmenter_on_empty_image() {
        let img = ImageBuffer::new(0, 0, 3, vec![]).unwrap();
        let bbs = NullSegmenter.segment(&img);
        assert!(bbs.is_empty());
    }

    #[test]
    fn null_classifier_always_returns_none() {
        let img = ImageBuffer::new(4, 4, 1, vec![0u8; 16]).unwrap();
        let bbox = BoundingBox { x: 0, y: 0, w: 4, h: 4 };
        let classifier = NullClassifier::<DayNameGlyph>::default();
        assert_eq!(classifier.classify(&img, bbox), None);
    }
}
