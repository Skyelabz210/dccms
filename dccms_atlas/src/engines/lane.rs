//! # Lane + MayaState — core operator-fabric primitives (NODE-B7-02)
//!
//! Heterogeneous CRT residue-tuple representation. Distinct from
//! [`dresden_codex::cram_address`] (which returns fixed `[u64; 6]` for the
//! canonical Safe Basis) — `MayaState` carries arbitrary lane configurations
//! including the engine-specific heterogeneous fabrics:
//!
//! - **Vigesimal**: `[ℤ/4, ℤ/5]`
//! - **Tzolkin**: `[ℤ/13, ℤ/20]`
//! - **Long Count (cyclic part)**: `[ℤ/20, ℤ/18, ℤ/20, ℤ/20]`
//! - **Venus Table**: `[ℤ/8, ℤ/73]`
//!
//! Adapters [`MayaState::from_cram_address`] and [`MayaState::to_cram_address`]
//! bridge the canonical 6-lane Safe Basis representation.
//!
//! Source: vault `Mayas Engine.md` §CORE TYPES, adapted per DAG decisions
//! D-1, D-4 (see [`docs/v0_8_0_B7_PLAN.md`](../../../../docs/v0_8_0_B7_PLAN.md)).

#![allow(dead_code)]

use dresden_codex::SAFE_BASIS;

/// A single lane in an operator-fabric CRT representation.
///
/// `modulus` is the lane's prime or prime power. `name` is a stable
/// short identifier (e.g. `"tone"`, `"sign"`, `"quad"`). `domain` is
/// free-form metadata describing what the lane semantically tracks;
/// it is NOT load-bearing for arithmetic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Lane {
    pub name: &'static str,
    pub modulus: u64,
    pub domain: &'static str,
}

/// Errors that arise from constructing or operating on a [`MayaState`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MayaStateError {
    /// `residues.len() != lanes.len()`.
    LaneResidueMismatch { lanes: usize, residues: usize },
    /// A residue is ≥ its lane's modulus.
    ResidueOutOfRange { lane_idx: usize, residue: u64, modulus: u64 },
    /// `same_position` invoked with non-matching lane configurations.
    LaneMismatch,
    /// A lane has modulus 0.
    ZeroModulus { lane_idx: usize },
}

/// A Maya state as a CRT residue tuple over a heterogeneous lane set.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MayaState {
    residues: Vec<u64>,
    lanes: Vec<Lane>,
}

impl MayaState {
    /// Construct a `MayaState` from explicit residues and lanes.
    ///
    /// Validates: same length, all residues < their lane's modulus, no
    /// zero modulus.
    pub fn new(residues: Vec<u64>, lanes: Vec<Lane>) -> Result<Self, MayaStateError> {
        if residues.len() != lanes.len() {
            return Err(MayaStateError::LaneResidueMismatch {
                lanes: lanes.len(),
                residues: residues.len(),
            });
        }
        for (i, lane) in lanes.iter().enumerate() {
            if lane.modulus == 0 {
                return Err(MayaStateError::ZeroModulus { lane_idx: i });
            }
            if residues[i] >= lane.modulus {
                return Err(MayaStateError::ResidueOutOfRange {
                    lane_idx: i,
                    residue: residues[i],
                    modulus: lane.modulus,
                });
            }
        }
        Ok(Self { residues, lanes })
    }

    /// Construct without bounds-checking residues (caller asserts invariants).
    /// Validates lengths and zero modulus only.
    pub(crate) fn new_unchecked(
        residues: Vec<u64>,
        lanes: Vec<Lane>,
    ) -> Result<Self, MayaStateError> {
        if residues.len() != lanes.len() {
            return Err(MayaStateError::LaneResidueMismatch {
                lanes: lanes.len(),
                residues: residues.len(),
            });
        }
        for (i, lane) in lanes.iter().enumerate() {
            if lane.modulus == 0 {
                return Err(MayaStateError::ZeroModulus { lane_idx: i });
            }
        }
        Ok(Self { residues, lanes })
    }

    /// Reduce each residue mod its lane's modulus — convenience for
    /// callers building states from possibly-overflowing values.
    pub fn from_unbounded(values: Vec<u64>, lanes: Vec<Lane>) -> Result<Self, MayaStateError> {
        if values.len() != lanes.len() {
            return Err(MayaStateError::LaneResidueMismatch {
                lanes: lanes.len(),
                residues: values.len(),
            });
        }
        for (i, lane) in lanes.iter().enumerate() {
            if lane.modulus == 0 {
                return Err(MayaStateError::ZeroModulus { lane_idx: i });
            }
        }
        let residues = values.iter().zip(lanes.iter())
            .map(|(&v, lane)| v % lane.modulus)
            .collect();
        Ok(Self { residues, lanes })
    }

    /// Borrow the residue vector.
    pub fn residues(&self) -> &[u64] { &self.residues }

    /// Borrow the lane vector.
    pub fn lanes(&self) -> &[Lane] { &self.lanes }

    /// Number of lanes.
    pub fn lane_count(&self) -> usize { self.lanes.len() }

    /// Advance by `days` — pure modular arithmetic, stays in residue space.
    ///
    /// Per-lane: `r' = (r + days mod p) mod p`. Note the inner `days mod p`
    /// keeps the intermediate addition bounded even for very large `days`.
    pub fn advance(&self, days: u64) -> MayaState {
        let residues = self.residues.iter()
            .zip(self.lanes.iter())
            .map(|(&r, lane)| {
                let d = days % lane.modulus;
                (r + d) % lane.modulus
            })
            .collect();
        MayaState { residues, lanes: self.lanes.clone() }
    }

    /// Two states are in the same CRT equivalence class iff they have
    /// matching lanes AND matching residues.
    pub fn same_position(&self, other: &MayaState) -> bool {
        if self.lanes != other.lanes { return false; }
        self.residues == other.residues
    }

    /// Whether lane `idx` is at residue 0.
    ///
    /// Panics if `idx >= lane_count()`.
    pub fn lane_at_origin(&self, idx: usize) -> bool {
        self.residues[idx] == 0
    }

    /// Count lanes simultaneously at residue 0.
    pub fn alignment_count(&self) -> usize {
        self.residues.iter().filter(|&&r| r == 0).count()
    }

    /// Pisano coverage profile — one row per lane: `(modulus, pisano_period, distinct_count)`.
    ///
    /// **D-1 mitigation:** returns integer counts only; the coverage ratio
    /// is intentionally not computed here to avoid float arithmetic at
    /// the API boundary. Callers may compute `distinct_count / modulus`
    /// in whatever representation they need (rational, integer-scaled, etc.).
    pub fn pisano_profile(&self) -> Result<Vec<(u64, u64, u64)>, super::pisano::PisanoError> {
        let mut out = Vec::with_capacity(self.lanes.len());
        for lane in &self.lanes {
            let pi = super::pisano::pisano_period(lane.modulus)?;
            let seq = super::pisano::fibonacci_mod_sequence(lane.modulus)?;
            let distinct: std::collections::HashSet<u64> = seq.into_iter().collect();
            out.push((lane.modulus, pi, distinct.len() as u64));
        }
        Ok(out)
    }

    /// Ramanujan harmonic signature across lanes at point `n`.
    pub fn harmonic_signature(&self, n: u64) -> Vec<i64> {
        self.lanes.iter()
            .map(|lane| super::ramanujan::ramanujan_sum(lane.modulus, n))
            .collect()
    }

    // ─────────────────────────────────────────────────────────────────
    // Adapters to/from the canonical 6-lane Safe Basis address
    // ─────────────────────────────────────────────────────────────────

    /// Construct a `MayaState` representing `addr` over the canonical
    /// Safe Basis `[2, 3, 5, 7, 11, 13]`.
    pub fn from_cram_address(addr: [u64; 6]) -> MayaState {
        let lanes: Vec<Lane> = SAFE_BASIS.iter().enumerate().map(|(i, &p)| Lane {
            name: SAFE_BASIS_LANE_NAMES[i],
            modulus: p,
            domain: SAFE_BASIS_LANE_DOMAINS[i],
        }).collect();
        // Bound by construction: addr[i] = x % p, so addr[i] < p.
        MayaState { residues: addr.to_vec(), lanes }
    }

    /// Project a `MayaState` whose lanes are exactly the canonical
    /// Safe Basis (in order) into a `[u64; 6]` address. Returns `None`
    /// if the lane configuration does not match the Safe Basis.
    pub fn to_cram_address(&self) -> Option<[u64; 6]> {
        if self.lanes.len() != 6 { return None; }
        for (i, &p) in SAFE_BASIS.iter().enumerate() {
            if self.lanes[i].modulus != p { return None; }
        }
        let mut out = [0u64; 6];
        for i in 0..6 { out[i] = self.residues[i]; }
        Some(out)
    }
}

/// Canonical Safe-Basis lane names (used by `from_cram_address`).
const SAFE_BASIS_LANE_NAMES: [&str; 6] =
    ["p2", "p3", "p5", "p7", "p11", "p13"];

/// Canonical Safe-Basis lane domains (per `substrate_roles`).
const SAFE_BASIS_LANE_DOMAINS: [&str; 6] = [
    "parity",
    "fabric",
    "content",
    "traversal",
    "coordinate",
    "boundary",
];

impl std::fmt::Display for MayaState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<String> = self.lanes.iter()
            .zip(self.residues.iter())
            .map(|(lane, &r)| format!("{}:{}", lane.name, r))
            .collect();
        write!(f, "({})", parts.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two_small_lanes() -> Vec<Lane> {
        vec![
            Lane { name: "a", modulus: 4, domain: "test-a" },
            Lane { name: "b", modulus: 5, domain: "test-b" },
        ]
    }

    #[test]
    fn new_validates_lengths() {
        let lanes = two_small_lanes();
        let r = MayaState::new(vec![0], lanes.clone());
        assert!(matches!(r, Err(MayaStateError::LaneResidueMismatch { lanes: 2, residues: 1 })));
    }

    #[test]
    fn new_validates_residue_range() {
        let lanes = two_small_lanes();
        let r = MayaState::new(vec![4, 0], lanes);
        assert!(matches!(r, Err(MayaStateError::ResidueOutOfRange { lane_idx: 0, residue: 4, modulus: 4 })));
    }

    #[test]
    fn new_validates_zero_modulus() {
        let bad = vec![Lane { name: "bad", modulus: 0, domain: "" }];
        let r = MayaState::new(vec![0], bad);
        assert!(matches!(r, Err(MayaStateError::ZeroModulus { lane_idx: 0 })));
    }

    #[test]
    fn from_unbounded_reduces() {
        let lanes = two_small_lanes();
        let s = MayaState::from_unbounded(vec![13, 17], lanes).unwrap();
        assert_eq!(s.residues(), &[13 % 4, 17 % 5]);
        assert_eq!(s.residues(), &[1u64, 2]);
    }

    #[test]
    fn advance_zero_is_identity() {
        let s = MayaState::new(vec![1, 2], two_small_lanes()).unwrap();
        let t = s.advance(0);
        assert_eq!(s, t);
    }

    #[test]
    fn advance_additive() {
        let lanes = two_small_lanes();
        let s = MayaState::new(vec![1, 2], lanes).unwrap();
        let t = s.advance(3).advance(7);
        let u = s.advance(10);
        assert_eq!(t, u);
    }

    #[test]
    fn advance_large_offsets_handled() {
        let s = MayaState::new(vec![1, 2], two_small_lanes()).unwrap();
        // 1_000_000_000 mod 4 = 0, mod 5 = 0
        let t = s.advance(1_000_000_000);
        assert_eq!(t.residues(), &[1u64, 2]);
    }

    #[test]
    fn same_position_reflexive_and_strict() {
        let s = MayaState::new(vec![1, 2], two_small_lanes()).unwrap();
        let t = MayaState::new(vec![1, 2], two_small_lanes()).unwrap();
        assert!(s.same_position(&t));
        let u = MayaState::new(vec![1, 3], two_small_lanes()).unwrap();
        assert!(!s.same_position(&u));
    }

    #[test]
    fn same_position_rejects_lane_mismatch() {
        let s = MayaState::new(vec![0, 0], two_small_lanes()).unwrap();
        let other_lanes = vec![
            Lane { name: "x", modulus: 4, domain: "" },
            Lane { name: "y", modulus: 5, domain: "" },
        ];
        let t = MayaState::new(vec![0, 0], other_lanes).unwrap();
        // Even with matching residues, different lane structs ⇒ different position.
        assert!(!s.same_position(&t));
    }

    #[test]
    fn alignment_count_correct() {
        let s = MayaState::new(vec![0, 3], two_small_lanes()).unwrap();
        assert_eq!(s.alignment_count(), 1);
        let t = MayaState::new(vec![0, 0], two_small_lanes()).unwrap();
        assert_eq!(t.alignment_count(), 2);
        let u = MayaState::new(vec![1, 3], two_small_lanes()).unwrap();
        assert_eq!(u.alignment_count(), 0);
    }

    #[test]
    fn lane_at_origin_correct() {
        let s = MayaState::new(vec![0, 3], two_small_lanes()).unwrap();
        assert!(s.lane_at_origin(0));
        assert!(!s.lane_at_origin(1));
    }

    #[test]
    fn safe_basis_adapter_round_trip() {
        // Test that from_cram_address ∘ to_cram_address = identity for canonical
        // Safe Basis tuples.
        for x in [0u64, 1, 5, 13, 19, 148, 177, 260, 365, 584, 1448, 11_960, 37_960] {
            let addr = dresden_codex::cram_address(x);
            let state = MayaState::from_cram_address(addr);
            let recovered = state.to_cram_address();
            assert_eq!(recovered, Some(addr));
        }
    }

    #[test]
    fn to_cram_address_rejects_non_safe_basis_lanes() {
        let s = MayaState::new(vec![0, 0], two_small_lanes()).unwrap();
        assert_eq!(s.to_cram_address(), None);
    }

    #[test]
    fn safe_basis_state_has_six_lanes_with_canonical_moduli() {
        let state = MayaState::from_cram_address([0, 0, 0, 0, 0, 0]);
        assert_eq!(state.lane_count(), 6);
        let moduli: Vec<u64> = state.lanes().iter().map(|l| l.modulus).collect();
        assert_eq!(moduli, vec![2u64, 3, 5, 7, 11, 13]);
    }
}
