//! # Vault prediction vs measured pixel evidence (v0.9.6).
//!
//! For each page, `vault_prediction` states what the AI-penned vault
//! documents (Decoded.md, HULTA report) claim should be visible in the
//! image. `crosscheck` compares those claims to a real `PageScan`.
//!
//! ## Reliability warning
//!
//! All vault predictions are **UNRELIABLE** — they were written by AI in
//! prior sessions, encode the same assumptions they were meant to test,
//! and have not been verified against the physical manuscript. They are
//! hypotheses to challenge, not established findings.
//!
//! A "consistent" verdict means the measurement does not *contradict* the
//! prediction. It does NOT confirm the vault model. A contradiction IS a
//! finding — report it, do not explain it away.
//!
//! ## Threshold calibration
//!
//! All thresholds (`RED_ELEVATED_THRESHOLD`, etc.) are initial estimates.
//! They must be calibrated from real scan data once imagery is available.
//! The calibration note in `examples/scan.rs` makes this explicit.

#![allow(dead_code)]

use super::scan::PageScan;

// ── Thresholds (all uncalibrated — see module doc) ────────────────────────────

/// Vault prediction: red permille above this → "elevated red ink."
/// Uncalibrated initial estimate. Run all-74 scan to set from data.
pub const RED_ELEVATED_THRESHOLD: u32 = 20;

/// Vault prediction: ink density permille above this → "dense ink coverage."
pub const INK_DENSE_THRESHOLD: u32 = 100;

/// Vault prediction: entropy millibits above this → "high complexity layout."
pub const ENTROPY_HIGH_THRESHOLD: u32 = 6_000;

/// Vault prediction: ink density permille below this → "near-blank / damaged."
pub const INK_DAMAGED_THRESHOLD: u32 = 50;

// ── Public types ──────────────────────────────────────────────────────────────

/// What the vault model (UNRELIABLE — AI-penned) predicts for one page.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VaultPrediction {
    /// Förstemann page number.
    pub page: u8,
    /// Human-readable section label from the vault page model.
    pub section: &'static str,
    /// Functional role claim from Decoded.md / HULTA report.
    pub vault_role: &'static str,
    /// Vault predicts elevated red-ink density (K-elimination carry numbers).
    pub predicts_elevated_red: bool,
    /// Vault predicts dense ink (numerical tables, not figurative almanac).
    pub predicts_dense_ink: bool,
    /// Vault predicts high Shannon entropy (complex multi-register layout).
    pub predicts_high_entropy: bool,
    /// Vault predicts near-blank page (WWII damage — ink-loss mode).
    pub predicts_damaged_or_blank: bool,
}

/// Result of comparing one vault prediction to one `PageScan`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CrosscheckResult {
    /// The vault prediction that was tested.
    pub prediction: VaultPrediction,
    /// Whether the measured red_permille exceeded `RED_ELEVATED_THRESHOLD`.
    pub measured_elevated_red: bool,
    /// Whether the measured ink_density_permille exceeded `INK_DENSE_THRESHOLD`.
    pub measured_dense_ink: bool,
    /// Whether the measured entropy_millibits exceeded `ENTROPY_HIGH_THRESHOLD`.
    pub measured_high_entropy: bool,
    /// Whether the measured ink_density_permille fell below `INK_DAMAGED_THRESHOLD`.
    pub measured_damaged_or_blank: bool,
    /// True iff no activated prediction contradicts its measurement.
    pub consistent: bool,
    /// Number of activated predictions that agree with measurements.
    pub matching_predictions: u8,
    /// Number of activated predictions that contradict measurements.
    pub contradicting_predictions: u8,
}

// ── Public functions ──────────────────────────────────────────────────────────

/// Compare a vault prediction to a real `PageScan`.
///
/// Contradictions are findings, not failures. Report them honestly.
pub fn crosscheck(scan: &PageScan) -> CrosscheckResult {
    let pred = vault_prediction(scan.page);

    let elevated_red = scan.red_permille > RED_ELEVATED_THRESHOLD;
    let dense_ink = scan.ink_density_permille > INK_DENSE_THRESHOLD;
    let high_entropy = scan.entropy_millibits > ENTROPY_HIGH_THRESHOLD;
    let damaged = scan.ink_density_permille < INK_DAMAGED_THRESHOLD;

    let mut matching = 0u8;
    let mut contradicting = 0u8;

    let mut tally = |predicted: bool, measured: bool| {
        if predicted {
            if measured {
                matching += 1;
            } else {
                contradicting += 1;
            }
        }
    };
    tally(pred.predicts_elevated_red, elevated_red);
    tally(pred.predicts_dense_ink, dense_ink);
    tally(pred.predicts_high_entropy, high_entropy);
    tally(pred.predicts_damaged_or_blank, damaged);

    CrosscheckResult {
        measured_elevated_red: elevated_red,
        measured_dense_ink: dense_ink,
        measured_high_entropy: high_entropy,
        measured_damaged_or_blank: damaged,
        consistent: contradicting == 0,
        matching_predictions: matching,
        contradicting_predictions: contradicting,
        prediction: pred,
    }
}

/// The vault model's prediction for each Förstemann page.
///
/// Source: AI-penned Decoded.md and the page-by-page HULTA report.
/// All predictions tagged `[VAULT]` in `vault_role`.
pub fn vault_prediction(page: u8) -> VaultPrediction {
    match page {
        1..=15 => VaultPrediction {
            page,
            section: "Almanac (pre-Goddess)",
            vault_role: "260-day Tzolk'in ritual almanacs — figurative [VAULT]",
            predicts_elevated_red: false,
            predicts_dense_ink: false,
            predicts_high_entropy: false,
            predicts_damaged_or_blank: false,
        },
        16..=23 => VaultPrediction {
            page,
            section: "Moon Goddess almanacs",
            vault_role: "Biological domain; Moon Goddess figures + Tzolk'in intervals [VAULT]",
            predicts_elevated_red: true, // distance numbers in red
            predicts_dense_ink: false,   // figurative, not numerical table
            predicts_high_entropy: false,
            predicts_damaged_or_blank: false,
        },
        24 => VaultPrediction {
            page,
            section: "Venus preface / blank bridge",
            vault_role: "State-vector handoff; WWII ink-loss damage [VAULT]",
            predicts_elevated_red: false,
            predicts_dense_ink: false,
            predicts_high_entropy: false,
            predicts_damaged_or_blank: true, // vault: page 24 ink-loss mode
        },
        25..=28 => VaultPrediction {
            page,
            section: "New Year pages",
            vault_role: "Calendar register flush / New Year ceremonies [VAULT]",
            predicts_elevated_red: false,
            predicts_dense_ink: false,
            predicts_high_entropy: false,
            predicts_damaged_or_blank: false,
        },
        29..=45 => VaultPrediction {
            page,
            section: "Mars / agriculture",
            vault_role: "780-day Mars stride; 2-lane {7,11} processor [VAULT]",
            predicts_elevated_red: false,
            predicts_dense_ink: false,
            predicts_high_entropy: false,
            predicts_damaged_or_blank: false,
        },
        46..=50 => VaultPrediction {
            page,
            section: "Venus table",
            vault_role: "37,960-day Venus torus; dense numerical table [VAULT]",
            predicts_elevated_red: false,
            predicts_dense_ink: true,   // numeric table → dense ink
            predicts_high_entropy: true, // complex multi-column layout
            predicts_damaged_or_blank: false,
        },
        51..=58 => VaultPrediction {
            page,
            section: "Eclipse table",
            vault_role: "Lunar K-elim engine; red numbers = 148/177 corrections [VAULT]",
            predicts_elevated_red: true, // THE key K-elimination signal
            predicts_dense_ink: true,
            predicts_high_entropy: true,
            predicts_damaged_or_blank: false,
        },
        59..=61 => VaultPrediction {
            page,
            section: "Multi-thread sync",
            vault_role: "Calendar-wheel synchronization registers [VAULT]",
            predicts_elevated_red: false,
            predicts_dense_ink: false,
            predicts_high_entropy: false,
            predicts_damaged_or_blank: false,
        },
        62..=73 => VaultPrediction {
            page,
            section: "Serpent numbers",
            vault_role: "Deep-time long-count scaffold; dense glyph content [VAULT]",
            predicts_elevated_red: false,
            predicts_dense_ink: true,
            predicts_high_entropy: true,
            predicts_damaged_or_blank: false,
        },
        74 => VaultPrediction {
            page,
            section: "Great Deluge",
            vault_role: "Total reset; Chak Chel flood figure [VAULT]",
            predicts_elevated_red: false,
            predicts_dense_ink: false,
            predicts_high_entropy: false,
            predicts_damaged_or_blank: false,
        },
        _ => VaultPrediction {
            page,
            section: "Out of codex range",
            vault_role: "No prediction",
            predicts_elevated_red: false,
            predicts_dense_ink: false,
            predicts_high_entropy: false,
            predicts_damaged_or_blank: false,
        },
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::segmenter::scan::PageScan;

    fn make_scan(page: u8, ink: u32, red: u32, entropy: u32) -> PageScan {
        PageScan {
            page,
            width: 100,
            height: 100,
            ink_density_permille: ink,
            red_permille: red,
            entropy_millibits: entropy,
            component_count: 100,
            max_component_area: 1_000,
            barrier_rows: 3,
        }
    }

    #[test]
    fn eclipse_table_consistent_when_all_signals_present() {
        // Page 52: vault predicts red + dense + high-entropy
        let r = crosscheck(&make_scan(52, 200, 50, 7_000));
        assert!(r.consistent, "eclipse with red+dense+high-entropy should be consistent");
        assert_eq!(r.contradicting_predictions, 0);
        assert_eq!(r.matching_predictions, 3);
    }

    #[test]
    fn eclipse_table_inconsistent_when_no_red() {
        // Missing red contradicts the K-elimination prediction
        let r = crosscheck(&make_scan(52, 200, 0, 7_000));
        assert!(!r.consistent, "eclipse with no red should be inconsistent");
        assert!(r.contradicting_predictions >= 1);
    }

    #[test]
    fn page_24_consistent_when_blank() {
        let r = crosscheck(&make_scan(24, 10, 0, 100));
        assert!(r.consistent, "page 24 blank scan should be consistent");
        assert_eq!(r.matching_predictions, 1); // predicts_damaged_or_blank
    }

    #[test]
    fn page_24_inconsistent_when_dense() {
        // Dense ink contradicts the blank-bridge prediction
        let r = crosscheck(&make_scan(24, 500, 0, 7_000));
        assert!(!r.consistent);
        assert!(r.contradicting_predictions >= 1);
    }

    #[test]
    fn venus_table_consistent_when_dense_and_high_entropy() {
        // Pages 46-50: predicts dense + high entropy (no red prediction)
        let r = crosscheck(&make_scan(47, 200, 5, 7_000));
        assert!(r.consistent);
        assert_eq!(r.matching_predictions, 2);
    }

    #[test]
    fn almanac_pages_make_no_strong_predictions() {
        // Pages 1-15: no predictions activated → always consistent, nothing to match
        let r = crosscheck(&make_scan(1, 80, 5, 5_000));
        assert!(r.consistent);
        assert_eq!(r.matching_predictions, 0);
        assert_eq!(r.contradicting_predictions, 0);
    }

    #[test]
    fn vault_prediction_covers_every_codex_page() {
        for page in 1u8..=74 {
            let p = vault_prediction(page);
            assert_eq!(p.page, page);
            assert!(!p.section.is_empty());
            assert!(!p.vault_role.is_empty());
        }
    }

    #[test]
    fn moon_goddess_pages_predict_elevated_red() {
        for page in 16u8..=23 {
            let p = vault_prediction(page);
            assert!(
                p.predicts_elevated_red,
                "page {}: vault should predict red distance numbers",
                page
            );
        }
    }

    #[test]
    fn eclipse_table_pages_predict_red_and_dense() {
        for page in 51u8..=58 {
            let p = vault_prediction(page);
            assert!(p.predicts_elevated_red, "page {}: eclipse should predict red", page);
            assert!(p.predicts_dense_ink, "page {}: eclipse should predict dense", page);
        }
    }
}
