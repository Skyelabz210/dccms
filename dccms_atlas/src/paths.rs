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
///
/// Override with `DCCMS_IMPORTS_ROOT` environment variable to run on any machine:
/// ```text
/// DCCMS_IMPORTS_ROOT=/my/images cargo run --features slub --example slub_segment
/// ```
/// Falls back to the hard-coded path `~/Agents/imports` when the variable is unset.
pub fn imports_root() -> PathBuf {
    if let Ok(root) = std::env::var("DCCMS_IMPORTS_ROOT") {
        return PathBuf::from(root);
    }
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

/// The full Förstemann page range covered by the FAMSI Förstemann/Schele
/// PDF set on this machine. v0.9.5 N17/N18 extends from the original
/// pp13-24 bundle to all six FAMSI PDFs (pp01-12, pp13-24, pp25-35,
/// pp36-45, pp46-59, pp60-74). After extraction the per-page JPEGs are
/// at `famsi_extracted_dir()/page_NN.jpg`.
pub const FAMSI_PAGE_RANGE: std::ops::RangeInclusive<u32> = 1..=74;

/// The full Förstemann page range available as SLUB photographs on
/// this machine. The Dresden Codex has 74 surviving pages; the SLUB
/// IIIF service publishes them all. v0.9.5 N16 downloaded the full
/// range.
pub const SLUB_PAGE_RANGE: std::ops::RangeInclusive<u32> = 1..=74;

/// FAMSI source PDFs, one per Förstemann page range. Sorted by first
/// page. Provided for `famsi_extract.rs` and any audit tooling.
pub const FAMSI_SOURCE_PDFS: &[(&str, u32, u32)] = &[
    ("1_dresden_fors_schele_pp01-12.pdf",  1, 12),
    ("2_dresden_fors_schele_pp13-24.pdf", 13, 24),
    ("3_dresden_fors_schele_pp25-35.pdf", 25, 35),
    ("4_dresden_fors_schele_pp36-45.pdf", 36, 45),
    ("5_dresden_fors_schele_pp46-59.pdf", 46, 59),
    ("6_dresden_fors_schele_pp60-74.pdf", 60, 74),
];

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
    fn famsi_page_range_covers_full_codex() {
        // v0.9.5 N17/N18: extended from pp13-24 to the full 1..=74 set.
        assert!(FAMSI_PAGE_RANGE.contains(&1));
        assert!(FAMSI_PAGE_RANGE.contains(&13));
        assert!(FAMSI_PAGE_RANGE.contains(&24));
        assert!(FAMSI_PAGE_RANGE.contains(&74));
        assert!(!FAMSI_PAGE_RANGE.contains(&0));
        assert!(!FAMSI_PAGE_RANGE.contains(&75));
    }

    #[test]
    fn slub_page_range_covers_full_codex() {
        // v0.9.5 N16: the Dresden Codex has 74 surviving pages; SLUB
        // publishes them all via IIIF.
        assert!(SLUB_PAGE_RANGE.contains(&1));
        assert!(SLUB_PAGE_RANGE.contains(&74));
        assert!(!SLUB_PAGE_RANGE.contains(&75));
    }

    #[test]
    fn famsi_source_pdfs_partition_full_range() {
        // The 6 PDFs together must cover pp1..=74 without gaps or
        // overlaps. Iterate page-by-page and verify exactly one PDF
        // claims each page.
        for page in 1u32..=74 {
            let claims: Vec<&str> = FAMSI_SOURCE_PDFS.iter()
                .filter(|(_, first, last)| page >= *first && page <= *last)
                .map(|(n, _, _)| *n)
                .collect();
            assert_eq!(claims.len(), 1,
                "page {} claimed by {} PDFs: {:?}", page, claims.len(), claims);
        }
        // No page outside 1..=74 claimed.
        let total_pages: u32 = FAMSI_SOURCE_PDFS.iter()
            .map(|(_, first, last)| last - first + 1).sum();
        assert_eq!(total_pages, 74);
    }
}
