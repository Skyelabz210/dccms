//! # Adelic Indexing
//!
//! From NS proof Step 5: the CRT torus and the physical torus are two
//! projections of the adele ring 𝔸 = ℝ × ∏_p ℤ_p via
//!
//! ```text
//! 0 → ℤ̂ → 𝔸 → ℝ → 0
//! ```
//!
//! For DCCMS, every event carries both projections:
//!
//! - **Non-Archimedean address** (CRT residue tuple + winding) — the
//!   substrate's view of where the event lives on the infinite product
//!   of p-adic places.
//! - **Archimedean witness** (continuous time and angle) — the
//!   astronomical view.
//!
//! These two views are not "model vs reality" — they are the same
//! adelic object viewed from different places. Strong approximation
//! says ℚ is dense in 𝔸, so consistency at all finite places plus
//! consistency at the Archimedean place gives global consistency.
//!
//! ## Implementation correspondence
//!
//! The `qcid_ns_bridge::DualTrackState` already implements this pattern
//! for NS: physics lives on (a, b) ∈ F_p², contraction lives on the
//! norm. The two tracks do not interfere. DCCMS uses the same pattern.

use crate::recumbent::VenusConductorState;
use dresden_codex::cram_address;

/// The non-Archimedean address: residue tuple plus winding.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NonArchimedeanAddress {
    /// CRAM address on the Safe Basis.
    pub cram_residues: [u64; 6],
    /// Recumbent state on the Venus conductor.
    pub recumbent: VenusConductorState,
}

impl NonArchimedeanAddress {
    pub fn from_days(days_since_epoch: u64) -> Self {
        NonArchimedeanAddress {
            cram_residues: cram_address(days_since_epoch),
            recumbent: VenusConductorState::from_days(days_since_epoch),
        }
    }
}

/// The Archimedean witness: continuous (real-valued) parameters
/// represented as integer-scaled approximations for storage.
///
/// We keep these as integers scaled by a fixed factor to maintain
/// the no-float discipline. Conversion to angle is done by the caller.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArchimedeanWitness {
    /// Days since epoch (Long Count day count, integer).
    pub days_since_epoch: u64,
    /// Optional ecliptic longitude in arcseconds (0 to 1,296,000).
    /// 0 means unspecified.
    pub ecliptic_arcseconds: u64,
    /// Optional ecliptic latitude in arcseconds (-324,000 to 324,000, biased).
    pub ecliptic_latitude_arcseconds_biased: i64,
}

impl ArchimedeanWitness {
    pub fn from_days(days_since_epoch: u64) -> Self {
        ArchimedeanWitness {
            days_since_epoch,
            ecliptic_arcseconds: 0,
            ecliptic_latitude_arcseconds_biased: 0,
        }
    }

    pub fn with_ecliptic(mut self, longitude_arcsec: u64, latitude_arcsec_biased: i64) -> Self {
        self.ecliptic_arcseconds = longitude_arcsec;
        self.ecliptic_latitude_arcseconds_biased = latitude_arcsec_biased;
        self
    }
}

/// The complete adelic index for an event.
#[derive(Clone, Debug)]
pub struct AdelicIndex {
    pub non_arch: NonArchimedeanAddress,
    pub archimedean: ArchimedeanWitness,
}

impl AdelicIndex {
    pub fn from_days(days_since_epoch: u64) -> Self {
        AdelicIndex {
            non_arch: NonArchimedeanAddress::from_days(days_since_epoch),
            archimedean: ArchimedeanWitness::from_days(days_since_epoch),
        }
    }

    pub fn from_days_with_ecliptic(
        days_since_epoch: u64,
        longitude_arcsec: u64,
        latitude_arcsec_biased: i64,
    ) -> Self {
        let mut idx = Self::from_days(days_since_epoch);
        idx.archimedean = idx.archimedean
            .with_ecliptic(longitude_arcsec, latitude_arcsec_biased);
        idx
    }

    /// Verify the two views agree on the day count.
    pub fn verify(&self) -> bool {
        self.archimedean.days_since_epoch == self.non_arch.recumbent.pairs[0].to_days()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adelic_index_views_agree() {
        let idx = AdelicIndex::from_days(12345);
        assert!(idx.verify());
    }

    #[test]
    fn non_arch_address_round_trip() {
        let addr = NonArchimedeanAddress::from_days(584);
        assert_eq!(addr.cram_residues, cram_address(584));
        // 584 should give residue 0 in the 584-lane
        let pair = addr.recumbent.pair_for_modulus(584).unwrap();
        assert_eq!(pair.residue, 0);
        assert_eq!(pair.winding, 1);
    }

    #[test]
    fn ecliptic_witness_carries_position() {
        let idx = AdelicIndex::from_days_with_ecliptic(
            1000,
            108_000, // 30 degrees, i.e., 0° Taurus
            0,
        );
        assert_eq!(idx.archimedean.ecliptic_arcseconds, 108_000);
        assert!(idx.verify());
    }
}
