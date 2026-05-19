//! # Recombinant CRT — winding-preserving residue arithmetic (C-1)
//!
//! Mechanizes vault `07_recombinant_crt_winding.md`: lossless integer
//! accumulation via explicit winding-counter tracking. A `RecombinantState`
//! represents `x = r + K · M` exactly, where:
//!
//! - `r` is the standard CRT residue tuple (carried in a [`MayaState`])
//! - `K` is the **tuple winding** — the number of full `M`-cycles
//! - `M` is the product of the lane moduli
//!
//! Operations (add, add_integer) update the winding count explicitly so
//! reconstruction to integer form is exact even when the result exceeds
//! `M`. This is the substrate's answer to the standard RNS information
//! loss problem.
//!
//! ## Long Count usage
//!
//! Over the canonical Safe Basis `M_SAFE = 30,030`, the 13-Baktun Long
//! Count value `1,872,000` has winding `62`. `RecombinantState::from_integer(1_872_000)`
//! exposes this directly; `to_integer()` reconstructs `1,872,000` exactly.
//!
//! Source: vault `07_recombinant_crt_winding.md`.

#![allow(dead_code)]

use super::lane::{Lane, MayaState, MayaStateError};

/// Errors for Recombinant CRT operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecombinantError {
    /// Underlying MayaState construction failed.
    State(MayaStateError),
    /// Lane configurations don't match for a binary operation.
    LaneMismatch,
    /// Reconstruction would overflow u128.
    Overflow,
}

impl From<MayaStateError> for RecombinantError {
    fn from(e: MayaStateError) -> Self { Self::State(e) }
}

/// A winding-tracking residue state. Lossless under integer accumulation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecombinantState {
    state: MayaState,
    /// `K` — the number of full lane-moduli-product cycles. `x = r + K·M`.
    tuple_winding: u64,
}

impl RecombinantState {
    /// Construct from an explicit integer over the given lane set.
    ///
    /// Computes `K = ⌊x / M⌋` and `r = x mod M`, decomposes `r` into
    /// per-lane residues, and packages everything.
    pub fn from_integer(x: u64, lanes: Vec<Lane>) -> Result<Self, RecombinantError> {
        let m: u128 = product_of_moduli(&lanes)?;
        let m_u64 = m as u64;
        if m_u64 == 0 {
            return Err(RecombinantError::State(MayaStateError::ZeroModulus { lane_idx: 0 }));
        }
        let r = x % m_u64;
        let k = x / m_u64;
        let residues: Vec<u64> = lanes.iter().map(|l| r % l.modulus).collect();
        let state = MayaState::new(residues, lanes)?;
        Ok(Self { state, tuple_winding: k })
    }

    /// The lane-modulus product `M = ∏ lane.modulus`. Returns u128 for headroom.
    pub fn modulus(&self) -> u128 {
        // Always succeeds by construction (state was built from valid lanes).
        product_of_moduli(self.state.lanes()).unwrap_or(0)
    }

    /// Reconstruct the exact integer represented by this state: `r + K·M`.
    ///
    /// Returns u128 because the result can exceed u64 for large windings.
    pub fn to_integer(&self) -> Result<u128, RecombinantError> {
        let m = self.modulus();
        // r — reconstruct from residues via CRT (Garner's algorithm). Since
        // we know we built the state from x % M, we can reconstruct via the
        // explicit Garner mixed-radix expansion. For small moduli (Safe
        // Basis), a simpler direct-search reconstruction works: r is the
        // unique x in [0, M) with x % lanes[i] == residues[i] for all i.
        // Direct search bounded by M.
        let r = self.garner_reconstruct_residue();
        let k = self.tuple_winding as u128;
        k.checked_mul(m)
            .and_then(|km| km.checked_add(r))
            .ok_or(RecombinantError::Overflow)
    }

    /// Borrow the underlying MayaState.
    pub fn state(&self) -> &MayaState { &self.state }

    /// The tuple winding count `K`.
    pub fn winding(&self) -> u64 { self.tuple_winding }

    /// Add another RecombinantState (matching lanes). Tuple windings sum,
    /// plus a carry if the residue addition overflows M.
    pub fn add(&self, other: &Self) -> Result<Self, RecombinantError> {
        if self.state.lanes() != other.state.lanes() {
            return Err(RecombinantError::LaneMismatch);
        }
        // Reconstruct each side's r-mod-M, add them, separate the carry.
        let r_self = self.garner_reconstruct_residue();
        let r_other = other.garner_reconstruct_residue();
        let m = self.modulus();
        let sum = r_self.checked_add(r_other).ok_or(RecombinantError::Overflow)?;
        let carry = (sum / m) as u64;
        let new_r = (sum % m) as u64;
        let new_winding = self.tuple_winding
            .checked_add(other.tuple_winding)
            .and_then(|w| w.checked_add(carry))
            .ok_or(RecombinantError::Overflow)?;
        let lanes = self.state.lanes().to_vec();
        let residues: Vec<u64> = lanes.iter().map(|l| new_r % l.modulus).collect();
        let state = MayaState::new(residues, lanes)?;
        Ok(Self { state, tuple_winding: new_winding })
    }

    /// Add an integer (faster path than `add(&from_integer(n))`).
    pub fn add_integer(&self, n: u64) -> Result<Self, RecombinantError> {
        let lanes = self.state.lanes().to_vec();
        let other = Self::from_integer(n, lanes)?;
        self.add(&other)
    }

    /// CRT reconstruction of the residue part `r ∈ [0, M)`.
    ///
    /// Uses Garner's algorithm in ASCENDING j-loop order (per vault D-012
    /// "GARNER ORDER" anti-pattern guard — descending order silently
    /// corrupts).
    fn garner_reconstruct_residue(&self) -> u128 {
        let lanes = self.state.lanes();
        let residues = self.state.residues();
        let k = lanes.len();
        // Garner's mixed-radix coefficients v_i:
        //   v_0 = r_0
        //   v_i = ((r_i - mixed-radix-sum-so-far) · (prod_{j<i} p_j)^{-1}) mod p_i
        // Then x = sum_i v_i · prod_{j<i} p_j.
        let mut v = vec![0u128; k];
        for i in 0..k {  // ASCENDING per D-012
            let p_i = lanes[i].modulus as u128;
            let mut x_so_far: u128 = 0;
            let mut prod: u128 = 1;
            for j in 0..i {
                x_so_far += v[j] * prod;
                prod *= lanes[j].modulus as u128;
            }
            // We need v_i = (r_i - x_so_far) · prod^{-1} mod p_i
            let r_i = residues[i] as u128;
            let target_residue = (r_i + p_i * 10 - (x_so_far % p_i)) % p_i;
            let inv = mod_inv((prod % p_i) as u64, lanes[i].modulus);
            v[i] = ((target_residue as u64).wrapping_mul(inv) as u128) % p_i;
        }
        // Reconstruct x.
        let mut x: u128 = 0;
        let mut prod: u128 = 1;
        for i in 0..k {  // ASCENDING
            x += v[i] * prod;
            prod *= lanes[i].modulus as u128;
        }
        x
    }
}

/// Extended Euclidean modular inverse: returns `a^{-1} mod m`. Requires gcd(a,m)=1.
fn mod_inv(a: u64, m: u64) -> u64 {
    let (mut old_r, mut r) = (a as i128, m as i128);
    let (mut old_s, mut s) = (1i128, 0i128);
    while r != 0 {
        let q = old_r / r;
        let tmp_r = old_r - q * r; old_r = r; r = tmp_r;
        let tmp_s = old_s - q * s; old_s = s; s = tmp_s;
    }
    // old_s might be negative; reduce mod m.
    let inv = ((old_s % m as i128) + m as i128) % m as i128;
    inv as u64
}

fn product_of_moduli(lanes: &[Lane]) -> Result<u128, RecombinantError> {
    let mut prod: u128 = 1;
    for lane in lanes {
        prod = prod.checked_mul(lane.modulus as u128)
            .ok_or(RecombinantError::Overflow)?;
    }
    Ok(prod)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dresden_codex::SAFE_BASIS;

    fn safe_basis_lanes() -> Vec<Lane> {
        SAFE_BASIS.iter().enumerate().map(|(i, &p)| Lane {
            name: ["p2","p3","p5","p7","p11","p13"][i],
            modulus: p,
            domain: "safe-basis",
        }).collect()
    }

    #[test]
    fn from_integer_below_m_safe_zero_winding() {
        // x = 260 < M_SAFE → winding = 0.
        let s = RecombinantState::from_integer(260, safe_basis_lanes()).unwrap();
        assert_eq!(s.winding(), 0);
        assert_eq!(s.to_integer().unwrap(), 260u128);
    }

    #[test]
    fn from_integer_at_m_safe_winding_one() {
        // x = M_SAFE → winding = 1, r = 0.
        let s = RecombinantState::from_integer(30_030, safe_basis_lanes()).unwrap();
        assert_eq!(s.winding(), 1);
        assert_eq!(s.to_integer().unwrap(), 30_030u128);
    }

    #[test]
    fn from_integer_above_m_safe_round_trips() {
        for x in [0u64, 1, 260, 30_029, 30_030, 30_031, 37_960, 144_000, 1_872_000] {
            let s = RecombinantState::from_integer(x, safe_basis_lanes()).unwrap();
            assert_eq!(s.to_integer().unwrap(), x as u128,
                "round-trip failed for x = {}", x);
        }
    }

    #[test]
    fn long_count_13_baktun_winding_is_62() {
        // 1,872,000 / 30,030 = 62 remainder 1,140.
        let s = RecombinantState::from_integer(1_872_000, safe_basis_lanes()).unwrap();
        assert_eq!(s.winding(), 62);
        assert_eq!(s.to_integer().unwrap(), 1_872_000u128);
    }

    #[test]
    fn modulus_equals_m_safe() {
        let s = RecombinantState::from_integer(0, safe_basis_lanes()).unwrap();
        assert_eq!(s.modulus(), 30_030u128);
    }

    #[test]
    fn add_below_m_safe() {
        let a = RecombinantState::from_integer(100, safe_basis_lanes()).unwrap();
        let b = RecombinantState::from_integer(200, safe_basis_lanes()).unwrap();
        let c = a.add(&b).unwrap();
        assert_eq!(c.to_integer().unwrap(), 300u128);
        assert_eq!(c.winding(), 0);
    }

    #[test]
    fn add_crossing_m_safe_increments_winding() {
        // 20000 + 20000 = 40000 > M_SAFE → r = 9970, winding = 1.
        let a = RecombinantState::from_integer(20_000, safe_basis_lanes()).unwrap();
        let b = RecombinantState::from_integer(20_000, safe_basis_lanes()).unwrap();
        let c = a.add(&b).unwrap();
        assert_eq!(c.to_integer().unwrap(), 40_000u128);
        assert_eq!(c.winding(), 1);
    }

    #[test]
    fn add_integer_helper_matches_full_add() {
        let a = RecombinantState::from_integer(100, safe_basis_lanes()).unwrap();
        let via_integer = a.add_integer(50).unwrap();
        assert_eq!(via_integer.to_integer().unwrap(), 150u128);
    }

    #[test]
    fn add_associativity_below_overflow() {
        let a = RecombinantState::from_integer(1_234, safe_basis_lanes()).unwrap();
        let b = RecombinantState::from_integer(5_678, safe_basis_lanes()).unwrap();
        let c = RecombinantState::from_integer(9_012, safe_basis_lanes()).unwrap();
        let ab_c = a.add(&b).unwrap().add(&c).unwrap();
        let a_bc = a.add(&b.add(&c).unwrap()).unwrap();
        assert_eq!(ab_c.to_integer().unwrap(), a_bc.to_integer().unwrap());
        assert_eq!(ab_c.to_integer().unwrap(), 15_924u128);
    }

    #[test]
    fn winding_monotone_under_positive_addition() {
        let mut s = RecombinantState::from_integer(0, safe_basis_lanes()).unwrap();
        let mut last_winding = 0u64;
        for step in 1..=70 {
            s = s.add_integer(M_SAFE_STEP).unwrap();
            assert!(s.winding() >= last_winding,
                "winding decreased at step {}: {} -> {}", step, last_winding, s.winding());
            last_winding = s.winding();
        }
    }

    const M_SAFE_STEP: u64 = 5_000;
}
