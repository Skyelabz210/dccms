//! # SLUB Dresden JPEG adapter (feature `slub`)
//!
//! Decodes a SLUB Dresden Codex JPEG (per the URL pattern in
//! [docs/imagery_sources.md](../../../../docs/imagery_sources.md)) into
//! the framework's typed [`super::ImageBuffer`].
//!
//! Public-domain imagery — per the METS record, license is PDM 1.0 with
//! no restrictions on commercial or derivative use.
//!
//! ## Floats discipline
//!
//! The `image` crate uses floats internally for some decoding paths;
//! those floats DO NOT cross the dccms API boundary. This adapter
//! produces a `Vec<u8>` byte buffer plus integer width/height —
//! discrete by construction. Object contract preserved per the
//! `segmenter::mod` doc.

#![cfg(feature = "slub")]
#![allow(dead_code)]

use super::ImageBuffer;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SlubError {
    Io(String),
    Decode(String),
    UnsupportedColorType(String),
}

/// Load a SLUB-Dresden JPEG file into an [`ImageBuffer`] as 3-byte RGB pixels.
///
/// The JPEG decoder is the pure-Rust `image` crate; no native C dependency.
/// All floats stay inside the decoder; the returned buffer is `Vec<u8>`.
pub fn load_slub_page(path: impl AsRef<Path>) -> Result<ImageBuffer, SlubError> {
    let img = image::open(path.as_ref())
        .map_err(|e| SlubError::Io(format!("could not open SLUB JPEG: {}", e)))?;
    let rgb = img.into_rgb8();
    let width = rgb.width();
    let height = rgb.height();
    let data = rgb.into_raw();
    ImageBuffer::new(width, height, 3, data)
        .map_err(|e| SlubError::Decode(format!("ImageBuffer construction failed: {:?}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// Return the canonical SLUB imports path if it exists, else None.
    /// Test functions skip themselves cleanly when imagery is absent.
    fn slub_path(page: u32) -> Option<PathBuf> {
        let home = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).ok()?;
        let p = PathBuf::from(home)
            .join("Agents").join("imports").join("slub_dresden")
            .join(format!("page_{:08}.jpg", page));
        if p.exists() { Some(p) } else { None }
    }

    #[test]
    fn load_real_slub_page_16_if_present() {
        let Some(path) = slub_path(16) else {
            eprintln!("SKIP: SLUB page 16 not present at expected path");
            return;
        };
        let img = load_slub_page(&path).expect("decode page 16");
        // Synthesis prediction: ~3874x7649. Verify rough order.
        assert!(img.width >= 3000 && img.width <= 5000,
            "unexpected width {}", img.width);
        assert!(img.height >= 6000 && img.height <= 9000,
            "unexpected height {}", img.height);
        assert_eq!(img.bytes_per_pixel, 3);
        // Buffer size sanity.
        let expected = (img.width as usize) * (img.height as usize) * 3;
        assert_eq!(img.data.len(), expected);
    }
}
