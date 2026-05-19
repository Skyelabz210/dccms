//! # Goddess Section Extension (B-12 / Tier 3)
//!
//! The H4 visual transducer currently models Goddess-section pages
//! **16-24** (with page 24 as the structural blank-bridge). Per vault
//! `Dresden.md` Moon Goddess audit (referencing Barnhart 2005), the
//! Moon Goddess section actually begins at page **13c** — the lower
//! register of page 13 — and extends through page 23.
//!
//! ## What this module mechanizes
//!
//! - The **extended page range** [13c, 14, 15, 16, ..., 23] as a
//!   typed constant.
//! - The current dccms coverage [16..=23] documented as a **subset**
//!   of the extended range.
//! - The **un-decoded subset** [13c, 14, 15] named explicitly with
//!   `SourceMaterialStatus::Pending` — synthesis O / B-12.
//!
//! ## What this module does NOT mechanize
//!
//! Figure-by-figure iconographic decoder for pages 13c-15 requires
//! source material (Barnhart 2005 reference) which is not currently
//! available in the workspace. This module **documents** the open
//! question without making any false claim of completeness.
//!
//! Per synthesis discipline: name the gap explicitly, ship the
//! structural extension, do not silently omit.

#![allow(dead_code)]

/// Status of source material for a given Goddess-section page range.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SourceMaterialStatus {
    /// Source material is downloaded and decoder is implemented.
    Available,
    /// Source material is known to exist (named reference) but not
    /// yet in the workspace.
    Pending,
    /// Source material has been searched for and not found.
    NotFound,
}

/// Pages covered by the H4 visual transducer (dccms current coverage).
pub const H4_COVERED_PAGES: &[u8] = &[16, 17, 18, 19, 20, 21, 22, 23, 24];

/// Pages in the extended Moon Goddess range per Barnhart 2005.
///
/// Note: the actual range starts at "page 13c" — the lower register
/// of page 13. Represented as integer 13 in our u8-based page system;
/// the "c" register distinction is documented but not modeled
/// numerically (would require a register sub-field on the page type).
pub const GODDESS_EXTENDED_PAGES: &[u8] = &[13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23];

/// Pages in the extended range that are NOT yet covered by dccms.
pub const UNDECODED_GODDESS_PAGES: &[u8] = &[13, 14, 15];

/// The structural gap between dccms's current H4 coverage and the
/// canonical extended Moon Goddess range.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GoddessExtensionGap {
    /// Pages not yet decoded.
    pub undecoded_pages: Vec<u8>,
    /// Pages currently covered by the H4 visual transducer.
    pub h4_covered_pages: Vec<u8>,
    /// The structural blank-bridge page within H4 coverage.
    pub blank_bridge_page: u8,
    /// Source-material availability for the un-decoded subset.
    pub source_material_status: SourceMaterialStatus,
    /// Named reference for the extended range (Barnhart 2005).
    pub canonical_reference: &'static str,
}

impl GoddessExtensionGap {
    /// Return the current state of the gap analysis.
    pub fn current() -> Self {
        Self {
            undecoded_pages: UNDECODED_GODDESS_PAGES.to_vec(),
            h4_covered_pages: H4_COVERED_PAGES.to_vec(),
            blank_bridge_page: 24,
            source_material_status: SourceMaterialStatus::Pending,
            canonical_reference: "Barnhart 2005 (via vault Dresden.md Moon Goddess audit)",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn h4_coverage_is_subset_of_extended() {
        // Every H4-covered page (except 24, which is the blank bridge
        // outside the Goddess section proper) should be in the
        // extended range.
        for &p in H4_COVERED_PAGES {
            if p == 24 { continue; }  // BlankBridge — closes the section
            assert!(GODDESS_EXTENDED_PAGES.contains(&p),
                "H4 page {} should be in GODDESS_EXTENDED_PAGES", p);
        }
    }

    #[test]
    fn undecoded_pages_disjoint_from_h4() {
        for &p in UNDECODED_GODDESS_PAGES {
            assert!(!H4_COVERED_PAGES.contains(&p),
                "page {} is both un-decoded and H4-covered", p);
        }
    }

    #[test]
    fn extended_range_is_exactly_13_through_23() {
        assert_eq!(GODDESS_EXTENDED_PAGES, &[13u8, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23]);
    }

    #[test]
    fn gap_carries_pending_source_status() {
        let gap = GoddessExtensionGap::current();
        assert_eq!(gap.source_material_status, SourceMaterialStatus::Pending);
        assert_eq!(gap.undecoded_pages, vec![13u8, 14, 15]);
        assert!(gap.canonical_reference.contains("Barnhart"));
    }
}
