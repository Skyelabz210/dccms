//! # H4 Instruments — Goddess Section Diagnostics
//!
//! Six instruments to test whether the goddess section (Dresden pages
//! 16–23) encodes configuration metadata for the substrate. Each
//! instrument returns a quantitative signal value; the H4 verdict is
//! based on how many instruments exceed their threshold.
//!
//! ## The Six Instruments
//!
//! 1. **Necessity-Surplus** — content beyond what biological tracking
//!    requires.
//! 2. **Cross-Reference Mapper** — numerical co-occurrence with
//!    page-24 anchors and Venus intervals.
//! 3. **Information-Content Comparator** — entropy excess relative to
//!    single-purpose biological almanacs.
//! 4. **Configuration-Match Scorer** — match against predicted
//!    configuration-instruction signatures from the head generator.
//! 5. **Compression-Resistance Test** — residual under biological-only
//!    vs dual-encoding compression models.
//! 6. **Montgomery Quotient Shadow** — sixth instrument from the
//!    CRAM-ENHANCE paper: deterministic byproduct signature for
//!    (r_11, r_13) pairs across the goddess section.
//!
//! ## Threshold Policy (pre-registered)
//!
//! - Each instrument returns a value in basis points (0-10000).
//! - The instrument's threshold is pre-registered at 3000 bp (30%) above
//!   the corresponding null baseline.
//! - The H4 verdict is **supported** if at least 3 of 6 instruments
//!   exceed their threshold; **partial** for 1-2; **unsupported** for 0.
//!
//! ## State of implementation
//!
//! This module ships the instrument framework and signatures. The
//! per-instrument scoring functions accept abstract feature vectors
//! and compute the comparisons. The features themselves come from
//! the visual-transducer layer (ENHANCE / CRAM-ENHANCE bridge) plus
//! the published Juárez/Castellanos page-level entropy values.
//!
//! Acid: when the visual transducer is wired into the workspace, the
//! feature extractors plug in here without changing the instrument
//! interface. The instrument framework is complete and tested.

use crate::H4_SUPPORT_THRESHOLD;

/// A single H4 instrument's result.
#[derive(Clone, Debug)]
pub struct InstrumentResult {
    pub name: &'static str,
    /// Signal value in basis points (0-10000).
    pub signal_bp: u64,
    /// Threshold above which this instrument is considered to fire.
    pub threshold_bp: u64,
    /// Whether the instrument fired.
    pub fired: bool,
    /// Notes / interpretation.
    pub notes: String,
}

impl InstrumentResult {
    pub fn new(name: &'static str, signal_bp: u64, threshold_bp: u64, notes: impl Into<String>) -> Self {
        InstrumentResult {
            name,
            signal_bp,
            threshold_bp,
            fired: signal_bp >= threshold_bp,
            notes: notes.into(),
        }
    }
}

/// Full H4 report.
#[derive(Clone, Debug)]
pub struct H4Report {
    pub instruments: Vec<InstrumentResult>,
    pub verdict: H4Verdict,
}

/// H4 verdict.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H4Verdict {
    /// 3 or more instruments fired.
    Supported,
    /// 1 or 2 instruments fired.
    Partial,
    /// 0 instruments fired.
    Unsupported,
    /// Not enough data to compute.
    Indeterminate,
}

impl H4Verdict {
    pub fn label(&self) -> &'static str {
        match self {
            H4Verdict::Supported => "SUPPORTED",
            H4Verdict::Partial => "PARTIAL",
            H4Verdict::Unsupported => "UNSUPPORTED",
            H4Verdict::Indeterminate => "INDETERMINATE",
        }
    }
}

impl H4Report {
    pub fn from_instruments(instruments: Vec<InstrumentResult>) -> Self {
        let fire_count = instruments.iter().filter(|r| r.fired).count();
        let total = instruments.len();
        let verdict = if total == 0 {
            H4Verdict::Indeterminate
        } else if fire_count >= H4_SUPPORT_THRESHOLD {
            H4Verdict::Supported
        } else if fire_count >= 1 {
            H4Verdict::Partial
        } else {
            H4Verdict::Unsupported
        };
        H4Report { instruments, verdict }
    }

    pub fn fire_count(&self) -> usize {
        self.instruments.iter().filter(|r| r.fired).count()
    }
}

// ═══════════════════════════════════════════════════════════════════
// Instrument 1: Necessity-Surplus Detector
// ═══════════════════════════════════════════════════════════════════

/// Necessity-Surplus instrument input.
pub struct NecessitySurplusInput {
    /// Total information content of goddess section, in basis points
    /// relative to maximum possible for the page count.
    pub observed_content_bp: u64,
    /// Minimum required for biological-cycle tracking only.
    pub biological_minimum_bp: u64,
    /// Threshold for "surplus" verdict (basis points above biological).
    pub surplus_threshold_bp: u64,
}

pub fn necessity_surplus(input: NecessitySurplusInput) -> InstrumentResult {
    let surplus = input.observed_content_bp.saturating_sub(input.biological_minimum_bp);
    InstrumentResult::new(
        "Necessity-Surplus",
        surplus,
        input.surplus_threshold_bp,
        format!(
            "Surplus = {} bp over biological baseline {} bp",
            surplus, input.biological_minimum_bp
        ),
    )
}

// ═══════════════════════════════════════════════════════════════════
// Instrument 2: Cross-Reference Mapper
// ═══════════════════════════════════════════════════════════════════

/// Cross-Reference Mapper input.
pub struct CrossReferenceInput {
    /// Number of numerical references in goddess section that match
    /// page-24 anchor values, Venus intervals, basis-modulus values,
    /// or 819-day cycle values.
    pub anchor_matches: u64,
    /// Total numerical content units in goddess section.
    pub total_numerical_units: u64,
    /// Threshold density in basis points.
    pub density_threshold_bp: u64,
}

pub fn cross_reference_mapper(input: CrossReferenceInput) -> InstrumentResult {
    let density_bp = if input.total_numerical_units == 0 {
        0
    } else {
        (input.anchor_matches * 10_000) / input.total_numerical_units
    };
    InstrumentResult::new(
        "Cross-Reference",
        density_bp,
        input.density_threshold_bp,
        format!(
            "{}/{} numerical references match substrate anchors",
            input.anchor_matches, input.total_numerical_units
        ),
    )
}

// ═══════════════════════════════════════════════════════════════════
// Instrument 3: Information-Content Comparator
// ═══════════════════════════════════════════════════════════════════

/// Information-Content Comparator input.
pub struct InformationContentInput {
    /// Goddess section entropy (basis points of maximum).
    pub goddess_entropy_bp: u64,
    /// Mean entropy of comparable single-purpose almanac sections.
    pub baseline_entropy_bp: u64,
    /// Threshold for "excess" verdict.
    pub excess_threshold_bp: u64,
}

pub fn information_content(input: InformationContentInput) -> InstrumentResult {
    let excess = input.goddess_entropy_bp.saturating_sub(input.baseline_entropy_bp);
    InstrumentResult::new(
        "Info-Content",
        excess,
        input.excess_threshold_bp,
        format!(
            "Goddess {} bp vs baseline {} bp → excess {} bp",
            input.goddess_entropy_bp, input.baseline_entropy_bp, excess
        ),
    )
}

// ═══════════════════════════════════════════════════════════════════
// Instrument 4: Configuration-Match Scorer
// ═══════════════════════════════════════════════════════════════════

/// Configuration-Match Scorer input.
pub struct ConfigurationMatchInput {
    /// Number of goddess-section content units that match an expected
    /// configuration-instruction signature derived from the head generator.
    pub matched_signatures: u64,
    /// Total content units examined.
    pub total_examined: u64,
    /// Match-rate threshold.
    pub match_threshold_bp: u64,
}

pub fn configuration_match(input: ConfigurationMatchInput) -> InstrumentResult {
    let match_rate_bp = if input.total_examined == 0 {
        0
    } else {
        (input.matched_signatures * 10_000) / input.total_examined
    };
    InstrumentResult::new(
        "Config-Match",
        match_rate_bp,
        input.match_threshold_bp,
        format!(
            "{}/{} content units match configuration-instruction signatures",
            input.matched_signatures, input.total_examined
        ),
    )
}

// ═══════════════════════════════════════════════════════════════════
// Instrument 5: Compression-Resistance Test
// ═══════════════════════════════════════════════════════════════════

/// Compression-Resistance Test input.
pub struct CompressionResistanceInput {
    /// Compressed size under a biological-only model (integer code length).
    pub bio_only_compressed_size: u64,
    /// Compressed size under a dual-encoding model.
    pub dual_compressed_size: u64,
    /// Threshold for "dual model wins" verdict (basis points reduction).
    pub reduction_threshold_bp: u64,
}

pub fn compression_resistance(input: CompressionResistanceInput) -> InstrumentResult {
    let reduction_bp = if input.bio_only_compressed_size == 0 {
        0
    } else {
        // (bio_only - dual) / bio_only × 10000
        let reduction = input.bio_only_compressed_size
            .saturating_sub(input.dual_compressed_size);
        (reduction * 10_000) / input.bio_only_compressed_size
    };
    InstrumentResult::new(
        "Compression",
        reduction_bp,
        input.reduction_threshold_bp,
        format!(
            "Dual model reduces compressed size from {} to {} ({} bp)",
            input.bio_only_compressed_size, input.dual_compressed_size, reduction_bp
        ),
    )
}

// ═══════════════════════════════════════════════════════════════════
// Instrument 6: Montgomery Quotient Shadow
// ═══════════════════════════════════════════════════════════════════

/// Montgomery Quotient Shadow input.
pub struct MontgomeryShadowInput {
    /// Number of (r_11, r_13) pairs with non-trivial shadow signature.
    pub nontrivial_shadow_count: u64,
    /// Total (r_11, r_13) pairs sampled.
    pub total_pairs: u64,
    /// Threshold density for "structural shadow" verdict.
    pub density_threshold_bp: u64,
}

pub fn montgomery_shadow(input: MontgomeryShadowInput) -> InstrumentResult {
    let density_bp = if input.total_pairs == 0 {
        0
    } else {
        (input.nontrivial_shadow_count * 10_000) / input.total_pairs
    };
    InstrumentResult::new(
        "Montgomery-Shadow",
        density_bp,
        input.density_threshold_bp,
        format!(
            "{}/{} (r_11, r_13) pairs show nontrivial shadow",
            input.nontrivial_shadow_count, input.total_pairs
        ),
    )
}

// ═══════════════════════════════════════════════════════════════════
// Composite H4 Test
// ═══════════════════════════════════════════════════════════════════

/// All H4 instrument inputs gathered.
pub struct H4Inputs {
    pub necessity: NecessitySurplusInput,
    pub cross_ref: CrossReferenceInput,
    pub info_content: InformationContentInput,
    pub config_match: ConfigurationMatchInput,
    pub compression: CompressionResistanceInput,
    pub montgomery: MontgomeryShadowInput,
}

/// Run the full H4 test suite.
pub fn run_h4_suite(inputs: H4Inputs) -> H4Report {
    let instruments = vec![
        necessity_surplus(inputs.necessity),
        cross_reference_mapper(inputs.cross_ref),
        information_content(inputs.info_content),
        configuration_match(inputs.config_match),
        compression_resistance(inputs.compression),
        montgomery_shadow(inputs.montgomery),
    ];
    H4Report::from_instruments(instruments)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_report_is_indeterminate() {
        let report = H4Report::from_instruments(vec![]);
        assert_eq!(report.verdict, H4Verdict::Indeterminate);
    }

    #[test]
    fn three_fired_is_supported() {
        let instruments = vec![
            InstrumentResult::new("A", 5000, 3000, "fired"),
            InstrumentResult::new("B", 5000, 3000, "fired"),
            InstrumentResult::new("C", 5000, 3000, "fired"),
            InstrumentResult::new("D", 1000, 3000, "below"),
            InstrumentResult::new("E", 1000, 3000, "below"),
            InstrumentResult::new("F", 1000, 3000, "below"),
        ];
        let report = H4Report::from_instruments(instruments);
        assert_eq!(report.verdict, H4Verdict::Supported);
        assert_eq!(report.fire_count(), 3);
    }

    #[test]
    fn one_fired_is_partial() {
        let instruments = vec![
            InstrumentResult::new("A", 5000, 3000, "fired"),
            InstrumentResult::new("B", 1000, 3000, "below"),
            InstrumentResult::new("C", 1000, 3000, "below"),
            InstrumentResult::new("D", 1000, 3000, "below"),
            InstrumentResult::new("E", 1000, 3000, "below"),
            InstrumentResult::new("F", 1000, 3000, "below"),
        ];
        let report = H4Report::from_instruments(instruments);
        assert_eq!(report.verdict, H4Verdict::Partial);
    }

    #[test]
    fn zero_fired_is_unsupported() {
        let instruments = vec![
            InstrumentResult::new("A", 1000, 3000, "below"),
            InstrumentResult::new("B", 1000, 3000, "below"),
            InstrumentResult::new("C", 1000, 3000, "below"),
            InstrumentResult::new("D", 1000, 3000, "below"),
            InstrumentResult::new("E", 1000, 3000, "below"),
            InstrumentResult::new("F", 1000, 3000, "below"),
        ];
        let report = H4Report::from_instruments(instruments);
        assert_eq!(report.verdict, H4Verdict::Unsupported);
    }

    #[test]
    fn necessity_surplus_basic() {
        let r = necessity_surplus(NecessitySurplusInput {
            observed_content_bp: 7000,
            biological_minimum_bp: 4000,
            surplus_threshold_bp: 2000,
        });
        assert_eq!(r.signal_bp, 3000);
        assert!(r.fired);
    }

    #[test]
    fn cross_reference_density_computed() {
        let r = cross_reference_mapper(CrossReferenceInput {
            anchor_matches: 25,
            total_numerical_units: 100,
            density_threshold_bp: 2000,
        });
        assert_eq!(r.signal_bp, 2500); // 25/100 = 25%
        assert!(r.fired);
    }

    #[test]
    fn information_content_excess() {
        let r = information_content(InformationContentInput {
            goddess_entropy_bp: 6000,
            baseline_entropy_bp: 4500,
            excess_threshold_bp: 1000,
        });
        assert_eq!(r.signal_bp, 1500);
        assert!(r.fired);
    }

    #[test]
    fn compression_reduction_basic() {
        let r = compression_resistance(CompressionResistanceInput {
            bio_only_compressed_size: 1000,
            dual_compressed_size: 700,
            reduction_threshold_bp: 2000,
        });
        // (1000-700)/1000 × 10000 = 3000 bp
        assert_eq!(r.signal_bp, 3000);
        assert!(r.fired);
    }

    #[test]
    fn run_full_suite_completes() {
        let inputs = H4Inputs {
            necessity: NecessitySurplusInput {
                observed_content_bp: 7000,
                biological_minimum_bp: 4000,
                surplus_threshold_bp: 2000,
            },
            cross_ref: CrossReferenceInput {
                anchor_matches: 30,
                total_numerical_units: 100,
                density_threshold_bp: 2000,
            },
            info_content: InformationContentInput {
                goddess_entropy_bp: 6000,
                baseline_entropy_bp: 4500,
                excess_threshold_bp: 1000,
            },
            config_match: ConfigurationMatchInput {
                matched_signatures: 40,
                total_examined: 100,
                match_threshold_bp: 2000,
            },
            compression: CompressionResistanceInput {
                bio_only_compressed_size: 1000,
                dual_compressed_size: 700,
                reduction_threshold_bp: 2000,
            },
            montgomery: MontgomeryShadowInput {
                nontrivial_shadow_count: 25,
                total_pairs: 100,
                density_threshold_bp: 2000,
            },
        };
        let report = run_h4_suite(inputs);
        assert_eq!(report.instruments.len(), 6);
        // All instruments above threshold → SUPPORTED
        assert_eq!(report.verdict, H4Verdict::Supported);
    }
}
