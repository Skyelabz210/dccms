//! # Codex Topology — the Dresden Codex's sectional structure
//!
//! Per vault `Dresden Coprime.md` lines 27-71, the Dresden Codex divides
//! into 11 named sections across its 74 pages. This module models that
//! topology so downstream code can route page numbers to the appropriate
//! section without rediscovering the layout each time.
//!
//! The Moon Goddess section (pages 16-23) is currently the dccms target;
//! the other 10 sections are catalogued here for future expansion
//! (Chaak, Venus Tables, Eclipse Tables, Mars-78, K'atun, Serpent Numbers,
//! Great Deluge). The `BlankBridge` section is page 24 alone — the
//! deliberate structural blank between Goddess and New Year sections.
//!
//! Pages with WWII water damage (per vault `Dresden.md` Moon Goddess
//! audit): 2, 4, 24, 28, 34, 38, 71, 72. The astronomical core is intact;
//! the damage falls on introduction, blank-bridge, mid-Chaak, mid-Serpent
//! pages. See [`is_water_damaged`].

#![allow(dead_code)]

/// The eleven structural sections of the Dresden Codex.
///
/// Page-range source: vault `Dresden Coprime.md` §Page-by-page
/// topographical decoding.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CodexSection {
    /// Pages 1-14b: introduction / invocation; 52 almanacs in this range.
    Introduction,
    /// Pages 15-23 (vault) / 16-23 (dccms current): Moon Goddess /
    /// Divinatory Almanacs. The H4 visual transducer's current target.
    MoonGoddess,
    /// Page 24: deliberate structural blank. The "physical and
    /// mathematical bridge" between Moon Goddess and New Year sections.
    BlankBridge,
    /// Pages 25-28: New Year ceremonies (state-vector synchronization
    /// per Decoded.md §Algorithm 16).
    NewYear,
    /// Pages 29-45: Farmer's Almanacs / Chaak tables, anchored on the
    /// 780-day Mars synodic period (Decoded.md §Algorithm 6, selective
    /// lane nullification on {2, 3, 5, 13}).
    Chaak,
    /// Pages 46-50: Venus Tables — 65 synodic periods, 37,960-day grand
    /// synchronization (codex_decoder.rs).
    Venus,
    /// Pages 51-58: Lunar / Eclipse Tables — 405 lunations = 11,960 days
    /// = 46 Tzolk'in. The 148/177 alternation engine.
    Eclipse,
    /// Pages 58-59: Mars × 78 tables (page 58 overlaps with Eclipse;
    /// the vault places both there).
    Mars78,
    /// Page 60: K'atun prophecy (Decoded.md §Algorithm 10, range
    /// aliasing on M_SAFE).
    Katun,
    /// Pages 61-73: Rain Tables / Serpent Numbers (deep time, 32,000+
    /// years; Decoded.md §Algorithm 7).
    Serpent,
    /// Page 74: Great Deluge (final page of the codex).
    Deluge,
}

impl CodexSection {
    /// The first codex page in this section.
    pub fn first_page(&self) -> u8 {
        match self {
            Self::Introduction => 1,
            Self::MoonGoddess  => 15,  // vault canonical range start; dccms h4_visual is the 16-23 subset
            Self::BlankBridge  => 24,
            Self::NewYear      => 25,
            Self::Chaak        => 29,
            Self::Venus        => 46,
            Self::Eclipse      => 51,
            Self::Mars78       => 58,
            Self::Katun        => 60,
            Self::Serpent      => 61,
            Self::Deluge       => 74,
        }
    }

    /// The last codex page in this section (inclusive).
    pub fn last_page(&self) -> u8 {
        match self {
            Self::Introduction => 14,
            Self::MoonGoddess  => 23,
            Self::BlankBridge  => 24,
            Self::NewYear      => 28,
            Self::Chaak        => 45,
            Self::Venus        => 50,
            Self::Eclipse      => 58,
            Self::Mars78       => 59,
            Self::Katun        => 60,
            Self::Serpent      => 73,
            Self::Deluge       => 74,
        }
    }

    /// Iterator over all sections in codex order.
    pub fn all() -> &'static [CodexSection] {
        &[
            Self::Introduction,
            Self::MoonGoddess,
            Self::BlankBridge,
            Self::NewYear,
            Self::Chaak,
            Self::Venus,
            Self::Eclipse,
            Self::Mars78,
            Self::Katun,
            Self::Serpent,
            Self::Deluge,
        ]
    }
}

/// Map a codex page number to its section.
///
/// For page 58 (the only ambiguous page — both Eclipse and Mars78 claim
/// it), Eclipse is returned because the eclipse table is the primary
/// astronomical engine at that page. Use [`page_sections`] if you need
/// the full set of sections claiming a page.
pub fn page_to_section(page: u8) -> Option<CodexSection> {
    match page {
        1..=14  => Some(CodexSection::Introduction),
        15      => Some(CodexSection::MoonGoddess),  // vault range start
        16..=23 => Some(CodexSection::MoonGoddess),  // dccms current target
        24      => Some(CodexSection::BlankBridge),
        25..=28 => Some(CodexSection::NewYear),
        29..=45 => Some(CodexSection::Chaak),
        46..=50 => Some(CodexSection::Venus),
        51..=58 => Some(CodexSection::Eclipse),       // page 58 also Mars78
        59      => Some(CodexSection::Mars78),
        60      => Some(CodexSection::Katun),
        61..=73 => Some(CodexSection::Serpent),
        74      => Some(CodexSection::Deluge),
        _       => None,
    }
}

/// Return all sections claiming a page. Most pages have one section;
/// page 58 has two (Eclipse and Mars78).
pub fn page_sections(page: u8) -> Vec<CodexSection> {
    let mut sections = Vec::new();
    for &s in CodexSection::all() {
        if page >= s.first_page() && page <= s.last_page() {
            sections.push(s);
        }
    }
    sections
}

/// Whether a page is documented as WWII-water-damaged.
///
/// Source: vault `Dresden.md` Moon Goddess audit. Damaged pages:
/// {2, 4, 24, 28, 34, 38, 71, 72}. The astronomical core is intact;
/// damage falls on introductory, blank-bridge, mid-Chaak, and
/// mid-Serpent pages.
pub fn is_water_damaged(page: u8) -> bool {
    matches!(page, 2 | 4 | 24 | 28 | 34 | 38 | 71 | 72)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_codex_page_maps_to_a_section() {
        for page in 1u8..=74u8 {
            assert!(
                page_to_section(page).is_some(),
                "page {} must have a section assignment",
                page
            );
        }
    }

    #[test]
    fn pages_outside_codex_have_no_section() {
        assert_eq!(page_to_section(0), None);
        assert_eq!(page_to_section(75), None);
        assert_eq!(page_to_section(255), None);
    }

    #[test]
    fn moon_goddess_pages_match_h4_visual_target() {
        // The dccms h4_visual iconographic alphabet currently covers
        // pages 16-23 as MoonGoddess and page 24 as BlankBridge.
        for page in 16u8..=23u8 {
            assert_eq!(page_to_section(page), Some(CodexSection::MoonGoddess));
        }
        assert_eq!(page_to_section(24), Some(CodexSection::BlankBridge));
    }

    #[test]
    fn page_15_is_moon_goddess_per_vault() {
        // Vault Decoded.md and Dresden Coprime use 15-23 for Moon Goddess
        // (i.e., page 15 begins the section). The dccms h4_visual
        // alphabet starts at page 16; this is a documented divergence.
        // The codex_topology recognizes the vault range.
        assert_eq!(page_to_section(15), Some(CodexSection::MoonGoddess));
    }

    #[test]
    fn page_58_belongs_to_both_eclipse_and_mars78() {
        // page_to_section prioritizes Eclipse for the ambiguous case.
        assert_eq!(page_to_section(58), Some(CodexSection::Eclipse));
        // page_sections returns both.
        let sections = page_sections(58);
        assert_eq!(sections.len(), 2);
        assert!(sections.contains(&CodexSection::Eclipse));
        assert!(sections.contains(&CodexSection::Mars78));
    }

    #[test]
    fn sections_cover_full_codex_without_gaps() {
        // First page of section N+1 = last page of section N + 1 (except
        // for the page-58 overlap, which is intentional).
        let sections = CodexSection::all();
        // First section starts at page 1.
        assert_eq!(sections[0].first_page(), 1);
        // Last section ends at page 74.
        assert_eq!(sections[sections.len() - 1].last_page(), 74);
        // No gaps when iterating; allow overlap at page 58.
        for w in sections.windows(2) {
            let gap = w[1].first_page() as i32 - w[0].last_page() as i32;
            // gap is either 1 (normal succession), 0 (page 58 overlap),
            // or 1 (also normal: e.g., section ends at p, next starts at p+1).
            assert!(
                gap == 0 || gap == 1,
                "unexpected section gap between {:?} (ends {}) and {:?} (starts {}): {}",
                w[0], w[0].last_page(), w[1], w[1].first_page(), gap
            );
        }
    }

    #[test]
    fn water_damage_pages() {
        let damaged = [2u8, 4, 24, 28, 34, 38, 71, 72];
        for &p in &damaged {
            assert!(is_water_damaged(p), "page {} should be marked damaged", p);
        }
        // Pages in the astronomical core (Venus, Eclipse, Katun) intact.
        for p in [46u8, 47, 48, 49, 50, 51, 60] {
            assert!(!is_water_damaged(p));
        }
        // Page 24 IS damaged AND a structural blank — same page.
        assert!(is_water_damaged(24));
        assert_eq!(page_to_section(24), Some(CodexSection::BlankBridge));
    }

    #[test]
    fn first_last_consistent_per_section() {
        for &s in CodexSection::all() {
            assert!(s.first_page() <= s.last_page(),
                "{:?}: first_page {} > last_page {}",
                s, s.first_page(), s.last_page());
        }
    }
}
