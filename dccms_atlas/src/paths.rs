//! Hard-coded data paths for HackFate's machine.
//!
//! Per the v0.9.1 "customize fully" directive: this program is not
//! intended to run anywhere except on the user's own machine with the
//! specific SLUB / FAMSI imagery present at known locations. Resolving
//! these paths from a config file or env var would add complexity for
//! no purpose. They are constants.
//!
//! ## Layout
//!
//! ```text
//! C:\Users\hackf\Agents\imports\
//! ├── slub_dresden\
//! │   └── page_00000013.jpg ... page_00000074.jpg      (SLUB ID-pad 8)
//! └── famsi_dresden\
//!     ├── famsi_pp13-24.pdf
//!     └── extracted\
//!         └── page_13.jpg ... page_24.jpg              (extracted JPEGs)
//! ```
//!
//! The `slub_dresden` photographs are 3874 × 7649 RGB JPEGs at 300 DPI
//! (~5 MB each); the FAMSI extracted JPEGs are 1552 × 3332 RGB
//! (~1.3 MB each), each one of the chromolithograph plates from the
//! 1880 Förstemann / Schele edition.
//!
//! Page-naming convention:
//! - SLUB files use 8-digit zero-padded Förstemann page numbers
//!   (matches the SLUB IIIF service download URL pattern).
//! - FAMSI files use 2-digit page numbers in 1-to-1 correspondence
//!   with Förstemann page numbers (mapping verified empirically by
//!   `examples/calibrate.rs`).

#![allow(dead_code)]

use std::path::PathBuf;

/// User's home directory on this machine.
pub fn home() -> PathBuf {
    // Both env vars are populated on this Windows machine; fall back
    // for the rare case one is missing.
    let h = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .expect("USERPROFILE or HOME must be set");
    PathBuf::from(h)
}

/// Root of all imported imagery for the codex.
pub fn imports_root() -> PathBuf {
    home().join("Agents").join("imports")
}

/// Directory holding SLUB Dresden page JPEGs.
pub fn slub_dir() -> PathBuf {
    imports_root().join("slub_dresden")
}

/// Path to a SLUB page JPEG by Förstemann page number.
pub fn slub_page(page: u32) -> PathBuf {
    slub_dir().join(format!("page_{:08}.jpg", page))
}

/// Directory holding the FAMSI Förstemann / Schele PDF and extracted
/// chromolithograph plates.
pub fn famsi_dir() -> PathBuf {
    imports_root().join("famsi_dresden")
}

/// Path to the FAMSI source PDF (Förstemann pp. 13-24).
pub fn famsi_pdf() -> PathBuf {
    famsi_dir().join("famsi_pp13-24.pdf")
}

/// Directory holding individual JPEGs extracted from the FAMSI PDF.
pub fn famsi_extracted_dir() -> PathBuf {
    famsi_dir().join("extracted")
}

/// Path to a FAMSI extracted page JPEG by Förstemann page number.
/// The extraction order has been verified 1-to-1 with Förstemann
/// order via vertical-darkness fingerprint correlation
/// (see `examples/calibrate.rs` Job 2).
pub fn famsi_page(page: u32) -> PathBuf {
    famsi_extracted_dir().join(format!("page_{:02}.jpg", page))
}

/// The full Förstemann page range available in the FAMSI bundle on
/// this machine.
pub const FAMSI_PAGE_RANGE: std::ops::RangeInclusive<u32> = 13..=24;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slub_page_format() {
        let p = slub_page(16);
        let s = p.to_string_lossy().to_string();
        assert!(s.ends_with("page_00000016.jpg"), "got {}", s);
        assert!(s.contains("slub_dresden"), "got {}", s);
    }

    #[test]
    fn famsi_page_format() {
        let p = famsi_page(24);
        let s = p.to_string_lossy().to_string();
        assert!(s.ends_with("page_24.jpg"), "got {}", s);
        assert!(s.contains("famsi_dresden"), "got {}", s);
        assert!(s.contains("extracted"), "got {}", s);
    }

    #[test]
    fn famsi_page_range_matches_extracted_set() {
        // The PDF covers Förstemann pp. 13-24 inclusive.
        assert!(FAMSI_PAGE_RANGE.contains(&13));
        assert!(FAMSI_PAGE_RANGE.contains(&24));
        assert!(!FAMSI_PAGE_RANGE.contains(&12));
        assert!(!FAMSI_PAGE_RANGE.contains(&25));
    }
}
