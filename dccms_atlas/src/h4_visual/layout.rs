//! # PageLayout — ordered glyph sequence per codex page (NODE-VT06)
//!
//! A `PageLayout` represents one codex page as: page index in the
//! Goddess section (16..=24) + the iconographic figure painted on that
//! page + the day-count interval the page encodes (148 or 177).
//!
//! The cumulative residue address sequence produced by
//! `cumulative_addresses(&goddess_section_layout())` is the
//! **Operator Consistency certificate** for the visual transducer: it
//! must equal the cumulative CRAM addresses computed by the existing
//! [`MoonGoddessProfile`](crate::moon_goddess::MoonGoddessProfile) decoder
//! over [`GODDESS_SECTION_INTERVALS`](crate::h4_non_visual::GODDESS_SECTION_INTERVALS).

#![allow(dead_code)]

use super::iconographic::{IconographicFigure, ALL_FIGURES};
use dresden_codex::cram_address;

/// One codex page in the Goddess section.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PageLayout {
    /// Codex page number (16..=24 for the Moon Goddess section).
    pub page_index: u8,
    /// Iconographic figure painted on this page.
    pub figure: IconographicFigure,
    /// Day-count interval this page encodes (148 or 177 for Goddess pages).
    pub interval: u64,
}

impl PageLayout {
    /// Construct a page from an iconographic figure. The interval is
    /// derived from the figure's `associated_interval()` — 148 for the
    /// near eclipse half-year, 177 for the far half-year.
    pub fn from_figure(page_index: u8, figure: IconographicFigure) -> Self {
        Self {
            page_index,
            figure,
            interval: figure.associated_interval(),
        }
    }

    /// CRAM address of this page's interval in canonical Safe Basis.
    pub fn cram_address(&self) -> [u64; 6] {
        cram_address(self.interval)
    }
}

/// The canonical Moon Goddess section layout — pages 16..=24, each
/// figure mapped to its `associated_interval`.
pub fn goddess_section_layout() -> Vec<PageLayout> {
    ALL_FIGURES
        .iter()
        .enumerate()
        .map(|(i, &fig)| PageLayout::from_figure(16 + i as u8, fig))
        .collect()
}

/// Cumulative running-total addresses across a page sequence.
///
/// Returns `Vec<[u64; 6]>` of length `layout.len()`, where entry `k` is
/// `cram_address(sum of intervals from page 0..=k)`. The final entry is
/// the CRAM address of the full Goddess-section total (1448 days for
/// the canonical layout).
pub fn cumulative_addresses(layout: &[PageLayout]) -> Vec<[u64; 6]> {
    let mut running = 0u64;
    layout
        .iter()
        .map(|p| {
            running += p.interval;
            cram_address(running)
        })
        .collect()
}

/// The cumulative integer total at each page boundary.
pub fn cumulative_totals(layout: &[PageLayout]) -> Vec<u64> {
    let mut running = 0u64;
    layout
        .iter()
        .map(|p| {
            running += p.interval;
            running
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::h4_non_visual::{GODDESS_SECTION_INTERVALS, GODDESS_SECTION_TOTAL};
    use crate::moon_goddess::MoonGoddessProfile;

    #[test]
    fn canonical_layout_has_9_pages() {
        let layout = goddess_section_layout();
        assert_eq!(layout.len(), 9);
    }

    #[test]
    fn canonical_layout_intervals_match_non_visual_constant() {
        // NODE-VT06 core gate: the visual layout MUST recover the canonical
        // interval sequence. This is the visual-to-non-visual bridge.
        let layout = goddess_section_layout();
        let intervals: Vec<u64> = layout.iter().map(|p| p.interval).collect();
        assert_eq!(intervals.as_slice(), &GODDESS_SECTION_INTERVALS[..]);
    }

    #[test]
    fn canonical_layout_total_is_1448() {
        let layout = goddess_section_layout();
        let total: u64 = layout.iter().map(|p| p.interval).sum();
        assert_eq!(total, GODDESS_SECTION_TOTAL);
        assert_eq!(total, 1_448);
    }

    #[test]
    fn cumulative_addresses_match_moon_goddess_profile() {
        // NODE-VT06 load-bearing gate (Operator Consistency at the page
        // sequence level): the cumulative CRAM addresses produced by
        // the visual layout MUST equal those computed by the existing
        // moon_goddess decoder. If these diverge the visual transducer
        // is hallucinating; mark BLOCKED.
        let layout = goddess_section_layout();
        let visual_cum = cumulative_addresses(&layout);
        let profile = MoonGoddessProfile::compute();
        assert_eq!(
            visual_cum,
            profile.page_cram_addresses,
            "visual cumulative addresses must equal MoonGoddessProfile::page_cram_addresses"
        );
    }

    #[test]
    fn page_indices_are_16_through_24() {
        let layout = goddess_section_layout();
        let pages: Vec<u8> = layout.iter().map(|p| p.page_index).collect();
        assert_eq!(pages, (16u8..=24u8).collect::<Vec<_>>());
    }

    #[test]
    fn cumulative_totals_increasing_and_reach_1448() {
        let layout = goddess_section_layout();
        let totals = cumulative_totals(&layout);
        assert_eq!(totals.len(), 9);
        for w in totals.windows(2) {
            assert!(w[1] > w[0], "cumulative totals must strictly increase");
        }
        assert_eq!(*totals.last().unwrap(), GODDESS_SECTION_TOTAL);
    }
}
