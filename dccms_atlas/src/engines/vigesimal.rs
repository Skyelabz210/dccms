//! # Vigesimal — Engine 1 (B-7.1)
//!
//! Maya base-20 number system as the CRT product `ℤ/4 × ℤ/5`. The
//! Pisano period `π(5) = 20` exactly fills the vigesimal cycle —
//! Fibonacci arithmetic mod 5 has a natural period of length 20,
//! structurally aligned with the base.
//!
//! Lane structure:
//!
//! - **Quad** (mod 4): binary-pair structure, four spatial quadrants
//! - **Pent** (mod 5): pentadic structure, five elemental/sensory states
//!
//! `gcd(4, 5) = 1` ⇒ CRT factors uniquely: every value in `[0, 20)`
//! corresponds to exactly one `(quad, pent)` residue pair.
//!
//! Source: vault `Mayas Engine.md` §ENGINE 1, adapted (no float).

#![allow(dead_code)]

use super::lane::{Lane, MayaState, MayaStateError};
use super::pisano::{fibonacci_mod_sequence, pisano_period, PisanoError};

/// Errors for Vigesimal operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VigesimalError {
    /// Encoding value ≥ 20.
    OutOfRange(u64),
    /// State has wrong lane structure (not `[QUAD_LANE, PENT_LANE]`).
    MalformedState,
    /// Pisano helper failed (only for `fibonacci_orbit`).
    Pisano(PisanoError),
    /// Underlying `MayaState` constructor failed.
    State(MayaStateError),
}

impl From<MayaStateError> for VigesimalError {
    fn from(e: MayaStateError) -> Self { VigesimalError::State(e) }
}

impl From<PisanoError> for VigesimalError {
    fn from(e: PisanoError) -> Self { VigesimalError::Pisano(e) }
}

/// Engine 1: Vigesimal base-20 fabric over `ℤ/4 × ℤ/5`.
pub struct Vigesimal;

impl Vigesimal {
    /// Quadrant lane: `ℤ/4ℤ`, four spatial quadrants.
    pub const QUAD_LANE: Lane = Lane {
        name: "quad",
        modulus: 4,
        domain: "spatial-quadrant",
    };

    /// Pentadic lane: `ℤ/5ℤ`, five elemental states.
    pub const PENT_LANE: Lane = Lane {
        name: "pent",
        modulus: 5,
        domain: "elemental",
    };

    /// Encode a value `0..=19` as a 2-lane `MayaState`.
    pub fn encode(value: u64) -> Result<MayaState, VigesimalError> {
        if value >= 20 {
            return Err(VigesimalError::OutOfRange(value));
        }
        let residues = vec![value % 4, value % 5];
        let lanes = vec![Self::QUAD_LANE, Self::PENT_LANE];
        Ok(MayaState::new(residues, lanes)?)
    }

    /// Validate that `state` has the canonical Vigesimal lane structure.
    pub fn validate(state: &MayaState) -> Result<(), VigesimalError> {
        if state.lane_count() != 2
            || state.lanes()[0] != Self::QUAD_LANE
            || state.lanes()[1] != Self::PENT_LANE
        {
            return Err(VigesimalError::MalformedState);
        }
        Ok(())
    }

    /// CRT reconstruction of the 0..=19 value.
    ///
    /// `x ≡ r1 (mod 4), x ≡ r2 (mod 5)`. By Garner / CRT with
    /// `5 ≡ 1 (mod 4)` and `4·4 = 16 ≡ 1 (mod 5)` so `inv(4, 5) = 4`:
    /// `x = (r1 · 5 · 1 + r2 · 4 · 4) mod 20 = (5·r1 + 16·r2) mod 20`.
    pub fn decode(state: &MayaState) -> Result<u64, VigesimalError> {
        Self::validate(state)?;
        let r1 = state.residues()[0];
        let r2 = state.residues()[1];
        Ok((r1 * 5 + r2 * 16) % 20)
    }

    /// Lane-parallel addition: `(a + b) mod 4` in lane 1, `(a + b) mod 5`
    /// in lane 2. Result lane structure preserved.
    pub fn add(a: &MayaState, b: &MayaState) -> Result<MayaState, VigesimalError> {
        Self::validate(a)?;
        Self::validate(b)?;
        let residues = vec![
            (a.residues()[0] + b.residues()[0]) % 4,
            (a.residues()[1] + b.residues()[1]) % 5,
        ];
        let lanes = vec![Self::QUAD_LANE, Self::PENT_LANE];
        Ok(MayaState::new(residues, lanes)?)
    }

    /// Lane-parallel multiplication.
    pub fn mul(a: &MayaState, b: &MayaState) -> Result<MayaState, VigesimalError> {
        Self::validate(a)?;
        Self::validate(b)?;
        let residues = vec![
            (a.residues()[0] * b.residues()[0]) % 4,
            (a.residues()[1] * b.residues()[1]) % 5,
        ];
        let lanes = vec![Self::QUAD_LANE, Self::PENT_LANE];
        Ok(MayaState::new(residues, lanes)?)
    }

    /// Fibonacci orbit through vigesimal residue space.
    ///
    /// Length = `π(20) = 60`. Each element is `(F_k mod 4, F_k mod 5)`.
    pub fn fibonacci_orbit() -> Result<Vec<MayaState>, VigesimalError> {
        let pi = pisano_period(20)?;
        let seq = fibonacci_mod_sequence(20)?;
        debug_assert_eq!(seq.len() as u64, pi);
        let mut orbit = Vec::with_capacity(pi as usize);
        for &v in &seq {
            orbit.push(Self::encode(v)?);
        }
        Ok(orbit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_rejects_out_of_range() {
        assert_eq!(Vigesimal::encode(20), Err(VigesimalError::OutOfRange(20)));
        assert_eq!(Vigesimal::encode(255), Err(VigesimalError::OutOfRange(255)));
    }

    #[test]
    fn round_trip_all_values() {
        for v in 0u64..20 {
            let state = Vigesimal::encode(v).unwrap();
            let decoded = Vigesimal::decode(&state).unwrap();
            assert_eq!(v, decoded, "round-trip failed for {}", v);
        }
    }

    #[test]
    fn encode_lane_structure() {
        let state = Vigesimal::encode(7).unwrap();
        assert_eq!(state.lane_count(), 2);
        assert_eq!(state.lanes()[0], Vigesimal::QUAD_LANE);
        assert_eq!(state.lanes()[1], Vigesimal::PENT_LANE);
        assert_eq!(state.residues(), &[7 % 4, 7 % 5]);
        assert_eq!(state.residues(), &[3u64, 2]);
    }

    #[test]
    fn add_matches_modular_arithmetic() {
        for a in 0u64..20 {
            for b in 0u64..20 {
                let sa = Vigesimal::encode(a).unwrap();
                let sb = Vigesimal::encode(b).unwrap();
                let sum = Vigesimal::add(&sa, &sb).unwrap();
                let decoded = Vigesimal::decode(&sum).unwrap();
                assert_eq!(decoded, (a + b) % 20, "{} + {} mod 20", a, b);
            }
        }
    }

    #[test]
    fn mul_matches_modular_arithmetic() {
        for a in 0u64..20 {
            for b in 0u64..20 {
                let sa = Vigesimal::encode(a).unwrap();
                let sb = Vigesimal::encode(b).unwrap();
                let prod = Vigesimal::mul(&sa, &sb).unwrap();
                let decoded = Vigesimal::decode(&prod).unwrap();
                assert_eq!(decoded, (a * b) % 20, "{} · {} mod 20", a, b);
            }
        }
    }

    #[test]
    fn fibonacci_orbit_length_is_pisano_20() {
        // π(20) = 60.
        let orbit = Vigesimal::fibonacci_orbit().unwrap();
        assert_eq!(orbit.len(), 60);
    }

    #[test]
    fn fibonacci_orbit_first_three_values() {
        let orbit = Vigesimal::fibonacci_orbit().unwrap();
        // F_0 mod 20 = 0; F_1 mod 20 = 1; F_2 mod 20 = 1.
        assert_eq!(Vigesimal::decode(&orbit[0]).unwrap(), 0);
        assert_eq!(Vigesimal::decode(&orbit[1]).unwrap(), 1);
        assert_eq!(Vigesimal::decode(&orbit[2]).unwrap(), 1);
        // F_3 = 2, F_4 = 3, F_5 = 5, F_6 = 8, F_7 = 13.
        assert_eq!(Vigesimal::decode(&orbit[3]).unwrap(), 2);
        assert_eq!(Vigesimal::decode(&orbit[7]).unwrap(), 13);
    }

    #[test]
    fn validate_rejects_wrong_lane_structure() {
        // A state with the right shape but wrong lane identities should fail.
        let wrong_lanes = vec![
            Lane { name: "foo", modulus: 4, domain: "" },
            Lane { name: "bar", modulus: 5, domain: "" },
        ];
        let state = MayaState::new(vec![0u64, 0], wrong_lanes).unwrap();
        assert_eq!(Vigesimal::validate(&state), Err(VigesimalError::MalformedState));
    }

    #[test]
    fn cram_lane_coverage_observation() {
        // The vigesimal {4, 5} lanes are the first two prime-power
        // components of the Safe Basis {2, 3, 5, 7, 11, 13}: 4 = 2² and 5.
        // This is the Maya's own selection of "core" lanes.
        assert_eq!(Vigesimal::QUAD_LANE.modulus, 4);
        assert_eq!(Vigesimal::PENT_LANE.modulus, 5);
        // Both appear in dresden_codex::MAYA_COPRIME_CORE = [4, 3, 5, 13].
        let core = dresden_codex::MAYA_COPRIME_CORE;
        assert!(core.contains(&4));
        assert!(core.contains(&5));
    }
}
