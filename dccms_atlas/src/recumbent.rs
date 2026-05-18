//! # Recumbent State — Residue Plus Winding
//!
//! A flat residue alone loses traversal history:
//!
//! ```text
//! x mod 584 = r
//! ```
//!
//! does not distinguish:
//!
//! ```text
//! x = r,  x = r + 584,  x = r + 2·584,  ...
//! ```
//!
//! The recumbent state preserves both:
//!
//! ```text
//! Recumbent(x, m) = (x mod m, floor(x / m))
//! ```
//!
//! For DCCMS, every event carries a recumbent state across the
//! Venus conductor moduli {260, 365, 584, 2920, 37960}, giving the
//! atlas strictly more discriminating power than residue alone at
//! zero additional compute cost (the winding is already implicit in
//! the date arithmetic; we just preserve it).
//!
//! ## Why this matters
//!
//! Two events that share a residue but live in different windings are
//! NOT the same configuration state — they are repeated occurrences
//! of the same address on a closed manifold. Preserving winding
//! distinguishes them.
//!
//! ## From the NS workspace
//!
//! The same recumbent primitive is implemented in `ns_v5/src/recumbent.rs`
//! where winding monotonicity is what gives the L∞ bound for vorticity.
//! For DCCMS, winding gives us *traversal identity* — which time through
//! the cycle this event represents.

use crate::VENUS_CONDUCTOR_MODULI;

/// A residue-and-winding pair for a single modulus.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RecumbentPair {
    pub modulus: u64,
    pub residue: u64,
    pub winding: u64,
}

impl RecumbentPair {
    /// Construct from an absolute day count.
    pub fn from_days(modulus: u64, days_since_epoch: u64) -> Self {
        RecumbentPair {
            modulus,
            residue: days_since_epoch % modulus,
            winding: days_since_epoch / modulus,
        }
    }

    /// Reconstruct the absolute day count from this pair.
    pub fn to_days(&self) -> u64 {
        self.winding * self.modulus + self.residue
    }
}

/// Recumbent state across all five Venus conductor moduli.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VenusConductorState {
    pub pairs: [RecumbentPair; 5],
}

impl VenusConductorState {
    /// Construct from an absolute day count.
    pub fn from_days(days_since_epoch: u64) -> Self {
        let mut pairs = [
            RecumbentPair { modulus: 0, residue: 0, winding: 0 }; 5
        ];
        for (i, &m) in VENUS_CONDUCTOR_MODULI.iter().enumerate() {
            pairs[i] = RecumbentPair::from_days(m, days_since_epoch);
        }
        VenusConductorState { pairs }
    }

    /// All five recumbent pairs.
    pub fn pairs(&self) -> &[RecumbentPair; 5] {
        &self.pairs
    }

    /// Recumbent pair for a specific modulus (if present in the conductor).
    pub fn pair_for_modulus(&self, m: u64) -> Option<RecumbentPair> {
        self.pairs.iter().find(|p| p.modulus == m).copied()
    }

    /// Verify internal consistency: every pair reconstructs to the same
    /// absolute day count.
    pub fn verify(&self) -> bool {
        let reconstructed = self.pairs[0].to_days();
        self.pairs.iter().all(|p| p.to_days() == reconstructed)
    }

    /// Sum of all winding counts (a coarse traversal-depth indicator).
    pub fn total_winding(&self) -> u64 {
        self.pairs.iter().map(|p| p.winding).sum()
    }

    /// Maximum winding across moduli (the slowest-clock count).
    pub fn max_winding(&self) -> u64 {
        self.pairs.iter().map(|p| p.winding).max().unwrap_or(0)
    }

    /// Concatenate residues into a flat tuple (for hashing/comparison).
    pub fn residue_tuple(&self) -> [u64; 5] {
        [
            self.pairs[0].residue,
            self.pairs[1].residue,
            self.pairs[2].residue,
            self.pairs[3].residue,
            self.pairs[4].residue,
        ]
    }

    /// Concatenate windings into a flat tuple.
    pub fn winding_tuple(&self) -> [u64; 5] {
        [
            self.pairs[0].winding,
            self.pairs[1].winding,
            self.pairs[2].winding,
            self.pairs[3].winding,
            self.pairs[4].winding,
        ]
    }
}

/// A complete recumbent state across an arbitrary basis.
#[derive(Clone, Debug)]
pub struct RecumbentState {
    pub days_since_epoch: u64,
    pub pairs: Vec<RecumbentPair>,
}

impl RecumbentState {
    /// Construct from a day count and a basis of moduli.
    pub fn from_basis(days_since_epoch: u64, basis: &[u64]) -> Self {
        let pairs = basis.iter()
            .map(|&m| RecumbentPair::from_days(m, days_since_epoch))
            .collect();
        RecumbentState { days_since_epoch, pairs }
    }

    /// Construct using the canonical Venus conductor moduli.
    pub fn venus_conductor(days_since_epoch: u64) -> Self {
        Self::from_basis(days_since_epoch, &VENUS_CONDUCTOR_MODULI)
    }

    /// Verify that all pairs reconstruct to the same absolute count.
    pub fn verify(&self) -> bool {
        self.pairs.iter().all(|p| p.to_days() == self.days_since_epoch)
    }

    /// Winding-aware key for use in hashmaps when traversal identity matters.
    /// Concatenates (residue, winding) tuples in basis order.
    pub fn full_key(&self) -> Vec<(u64, u64, u64)> {
        self.pairs.iter()
            .map(|p| (p.modulus, p.residue, p.winding))
            .collect()
    }

    /// Residue-only key (loses traversal identity, equivalent to flat residue).
    pub fn residue_only_key(&self) -> Vec<(u64, u64)> {
        self.pairs.iter()
            .map(|p| (p.modulus, p.residue))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recumbent_pair_round_trip() {
        let pair = RecumbentPair::from_days(584, 1750);
        assert_eq!(pair.residue, 1750 % 584);
        assert_eq!(pair.winding, 1750 / 584);
        assert_eq!(pair.to_days(), 1750);
    }

    #[test]
    fn recumbent_pair_preserves_winding_distinction() {
        let p1 = RecumbentPair::from_days(584, 236);          // first cycle, day 236
        let p2 = RecumbentPair::from_days(584, 236 + 584);    // second cycle, day 236
        let p3 = RecumbentPair::from_days(584, 236 + 2 * 584); // third cycle, day 236
        // Same residue
        assert_eq!(p1.residue, p2.residue);
        assert_eq!(p2.residue, p3.residue);
        // Different windings
        assert_eq!(p1.winding, 0);
        assert_eq!(p2.winding, 1);
        assert_eq!(p3.winding, 2);
    }

    #[test]
    fn venus_conductor_state_verifies() {
        let s = VenusConductorState::from_days(50_000);
        assert!(s.verify());
    }

    #[test]
    fn venus_conductor_state_day_zero() {
        let s = VenusConductorState::from_days(0);
        assert_eq!(s.residue_tuple(), [0u64; 5]);
        assert_eq!(s.winding_tuple(), [0u64; 5]);
        assert_eq!(s.total_winding(), 0);
        assert!(s.verify());
    }

    #[test]
    fn venus_conductor_state_after_one_full_table() {
        let s = VenusConductorState::from_days(37_960);
        // 37,960 % 37,960 = 0 with winding 1
        let p = s.pair_for_modulus(37_960).unwrap();
        assert_eq!(p.residue, 0);
        assert_eq!(p.winding, 1);
    }

    #[test]
    fn recumbent_state_general_basis() {
        let s = RecumbentState::from_basis(1000, &[7, 11, 13]);
        assert_eq!(s.pairs.len(), 3);
        assert_eq!(s.pairs[0].residue, 1000 % 7);
        assert_eq!(s.pairs[0].winding, 1000 / 7);
        assert!(s.verify());
    }

    #[test]
    fn full_key_distinguishes_what_residue_only_does_not() {
        let s1 = RecumbentState::venus_conductor(584);
        let s2 = RecumbentState::venus_conductor(584 + 37_960);
        // Both have residue 584 mod 584 = 0, and 584 mod 37960 = 584
        // For modulus 37960: s1 has winding 0, s2 has winding 1
        assert_ne!(s1.full_key(), s2.full_key());
        // Residue-only keys also differ here, but the winding distinction is
        // what survives address aliasing on a closed manifold.
    }

    #[test]
    fn winding_monotone_across_time() {
        let s1 = VenusConductorState::from_days(100);
        let s2 = VenusConductorState::from_days(200);
        for i in 0..5 {
            assert!(s2.pairs[i].to_days() >= s1.pairs[i].to_days());
        }
    }
}
