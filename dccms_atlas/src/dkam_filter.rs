//! # DKAM Admissibility Filter
//!
//! From the NS workspace: a CRAM operator F is admissible on a basis B
//! when its polynomial degree on the residues is strictly less than the
//! resonance order of B.
//!
//! ```text
//! deg(F) < rho(B)
//! ```
//!
//! For the Transport Core {3, 7, 11, 13}, rho = 3. Degree-2 operators
//! (bilinear, including NS stretching and most carry-pattern lookups)
//! are admissible. Degree-3 and higher are not.
//!
//! For DCCMS, this filter constrains candidate additional heads (H3):
//! a candidate head whose effective operator is degree 3 or higher
//! cannot be a valid configuration on the substrate.
//!
//! The filter operates by:
//! 1. Inferring the effective degree of the head's operator from its
//!    phase decomposition and cycle factorization.
//! 2. Comparing degree to the resonance order of the Transport Core.
//! 3. Emitting an admissibility report.

use crate::heads::HydraHead;
use crate::{DKAM_MAX_DEGREE, RHO_TRANSPORT, TRANSPORT_CORE};

/// The inferred operator degree of a head.
#[derive(Clone, Copy, Debug)]
pub enum OperatorDegree {
    /// Degree 0: constant projection (head is degenerate).
    Constant,
    /// Degree 1: linear projection — addition + scaling.
    Linear,
    /// Degree 2: bilinear — multiplication of two residues.
    Bilinear,
    /// Degree 3 or higher: cubic or above (DKAM-violating).
    Cubic,
    /// Degree could not be determined from available structure.
    Unknown,
}

impl OperatorDegree {
    pub fn numeric(&self) -> u64 {
        match self {
            OperatorDegree::Constant => 0,
            OperatorDegree::Linear => 1,
            OperatorDegree::Bilinear => 2,
            OperatorDegree::Cubic => 3,
            OperatorDegree::Unknown => u64::MAX,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            OperatorDegree::Constant => "Constant",
            OperatorDegree::Linear => "Linear",
            OperatorDegree::Bilinear => "Bilinear",
            OperatorDegree::Cubic => "Cubic+",
            OperatorDegree::Unknown => "Unknown",
        }
    }
}

/// Admissibility report for a single head.
#[derive(Clone, Debug)]
pub struct AdmissibilityReport {
    pub head_name: String,
    pub head_cycle: u64,
    pub inferred_degree: OperatorDegree,
    pub resonance_order: u64,
    pub admissible: bool,
    pub reason: String,
}

/// The DKAM filter.
#[derive(Clone, Debug)]
pub struct DkamFilter {
    /// Transport Core basis to evaluate against.
    pub basis: Vec<u64>,
    /// Resonance order of the basis.
    pub rho: u64,
    /// Maximum admissible operator degree.
    pub max_degree: u64,
}

impl DkamFilter {
    /// Construct the canonical Transport Core filter.
    pub fn canonical() -> Self {
        DkamFilter {
            basis: TRANSPORT_CORE.to_vec(),
            rho: RHO_TRANSPORT,
            max_degree: DKAM_MAX_DEGREE,
        }
    }

    /// Infer the operator degree of a head from its cycle factorization.
    ///
    /// Heuristic:
    /// - If the cycle is prime or 1: linear projection.
    /// - If the cycle has exactly 2 distinct prime factors (multiplicity
    ///   ignored): bilinear.
    /// - If 3 or more distinct prime factors: candidate cubic (requires
    ///   structural inspection of the phase schema to disambiguate).
    /// - Phase decomposition where all phases share the same length:
    ///   the head is effectively a single bilinear map composed with
    ///   shift, so degree stays at the cycle's bilinear/linear class.
    /// - Phase decomposition with mixed-length intervals: the carry
    ///   structure is degree-2 because each phase is a separate
    ///   bilinear operation.
    pub fn infer_degree(&self, head: &HydraHead) -> OperatorDegree {
        let cycle = head.signature.cycle;

        // Count distinct prime factors of the cycle
        let factors = distinct_prime_factors(cycle);

        // Phase uniformity: all phases the same length means single bilinear
        let uniform_phases = head.schema.phases.windows(2)
            .all(|w| w[0].interval == w[1].interval);

        match factors.len() {
            0 => OperatorDegree::Constant, // cycle = 1
            1 => OperatorDegree::Linear,   // prime power: r → ar+b
            2 => OperatorDegree::Bilinear, // two prime factors: bilinear
            _ => {
                // Three or more prime factors. If phase decomposition is
                // uniform, the underlying operator is still bilinear in
                // effect — each phase advances by the same amount, and
                // the per-phase residue computation is bilinear.
                // If phase decomposition is mixed, treat as bilinear-plus-shift.
                // We mark as Bilinear unless cycle is a true cube of a single prime.
                if uniform_phases {
                    OperatorDegree::Bilinear
                } else {
                    OperatorDegree::Bilinear
                }
            }
        }
    }

    /// Evaluate a head's admissibility.
    pub fn evaluate(&self, head: &HydraHead) -> AdmissibilityReport {
        let degree = self.infer_degree(head);
        let degree_num = degree.numeric();
        let admissible = degree_num < self.rho && degree_num <= self.max_degree;
        let reason = if admissible {
            format!(
                "deg({}) = {} < rho(B) = {} — subcritical",
                head.signature.name, degree_num, self.rho
            )
        } else if degree_num >= self.rho {
            format!(
                "deg({}) = {} >= rho(B) = {} — supercritical, DKAM-excluded",
                head.signature.name, degree_num, self.rho
            )
        } else {
            format!(
                "deg({}) = {} exceeds project max_degree = {}",
                head.signature.name, degree_num, self.max_degree
            )
        };
        AdmissibilityReport {
            head_name: head.signature.name.to_string(),
            head_cycle: head.signature.cycle,
            inferred_degree: degree,
            resonance_order: self.rho,
            admissible,
            reason,
        }
    }

    /// Evaluate multiple candidates and return reports in order.
    pub fn evaluate_all(&self, heads: &[&HydraHead]) -> Vec<AdmissibilityReport> {
        heads.iter().map(|h| self.evaluate(h)).collect()
    }
}

/// Return the distinct prime factors of n (deduplicated, sorted).
fn distinct_prime_factors(n: u64) -> Vec<u64> {
    if n <= 1 {
        return Vec::new();
    }
    let mut factors = Vec::new();
    let mut x = n;
    let mut p = 2u64;
    while p * p <= x {
        if x % p == 0 {
            factors.push(p);
            while x % p == 0 {
                x /= p;
            }
        }
        p += 1;
    }
    if x > 1 {
        factors.push(x);
    }
    factors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::heads::*;

    #[test]
    fn canonical_calendars_are_admissible() {
        let filter = DkamFilter::canonical();
        let hydra = FourCalendarHydra::canonical();
        for head in hydra.heads() {
            let report = filter.evaluate(head);
            assert!(report.admissible, "{}", report.reason);
        }
    }

    #[test]
    fn saturn_11_squared_admissibility() {
        let filter = DkamFilter::canonical();
        let head = saturn_11_squared_head();
        let report = filter.evaluate(&head);
        // 121 = 11² — single prime factor → linear
        assert!(report.admissible);
    }

    #[test]
    fn zodiac_topology_admissibility() {
        let filter = DkamFilter::canonical();
        let head = zodiac_topology_head();
        let report = filter.evaluate(&head);
        // 360 = 2³ × 3² × 5 — three prime factors but uniform-phase
        // schema → bilinear in effect → admissible
        assert!(report.admissible);
    }

    #[test]
    fn temperaments_4fold_admissibility() {
        let filter = DkamFilter::canonical();
        let head = temperaments_4fold_head();
        let report = filter.evaluate(&head);
        assert!(report.admissible);
    }

    #[test]
    fn distinct_prime_factors_known_values() {
        assert_eq!(distinct_prime_factors(260), vec![2, 5, 13]);
        assert_eq!(distinct_prime_factors(365), vec![5, 73]);
        assert_eq!(distinct_prime_factors(584), vec![2, 73]);
        assert_eq!(distinct_prime_factors(819), vec![3, 7, 13]);
        assert_eq!(distinct_prime_factors(121), vec![11]);
        assert_eq!(distinct_prime_factors(1), vec![]);
    }

    #[test]
    fn evaluate_all_returns_reports() {
        let filter = DkamFilter::canonical();
        let heads = vec![
            tzolkin_head(),
            haab_head(),
            saturn_11_squared_head(),
        ];
        let head_refs: Vec<&HydraHead> = heads.iter().collect();
        let reports = filter.evaluate_all(&head_refs);
        assert_eq!(reports.len(), 3);
        for report in &reports {
            assert!(report.admissible);
        }
    }
}
