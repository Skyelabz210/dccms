//! # Hydra Heads — Calendar Projections as Sieve Heads
//!
//! Each Maya calendar is one Hydra head: a projection of the underlying
//! adelic object onto a specific subset of the Safe Basis through a
//! specific operator topology. The four canonical calendars give four
//! canonical heads.
//!
//! ## Head Signature
//!
//! A head is defined by:
//! - `lane_primes` — which primes carry the head's discriminating signal
//! - `operator_schema` — the carry-pattern atlas the head computes
//! - `phase_decomposition` — how the head subdivides its cycle
//! - `nullification_pattern` — which Safe Basis lanes the head silences
//!
//! ## The Four Canonical Heads
//!
//! | Calendar | Cycle | Nullified | Active | Phase decomposition |
//! |----------|-------|-----------|--------|---------------------|
//! | Tzolk'in | 260   | {2,5,13}  | {3,7,11} | 13 × 20 trecenas |
//! | Haab     | 365   | {5}       | {2,3,7,11,13} | 18 × 20 + 5 Wayeb |
//! | CR       | 18,980| {2,5,13}  | {3,7,11} | 73 × 260 (Tzolk'in long-arm) |
//! | Long Count| positional | varies | varies | 1, 20, 360, 7200, 144000 |

use dresden_codex::{
    MultiPhaseSchema, cram_address, nullified_lanes, active_lanes,
    VENUS_SYNODIC, SAFE_BASIS,
};

/// A Hydra head's compact signature.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeadSignature {
    /// Display name of the head.
    pub name: &'static str,
    /// Primary cycle length.
    pub cycle: u64,
    /// Safe Basis primes nullified by this cycle (residue = 0).
    pub nullified_lanes: Vec<u64>,
    /// Safe Basis primes active in this cycle (residue != 0).
    pub active_lanes: Vec<u64>,
    /// CRAM address of the cycle (residue tuple on Safe Basis).
    pub address: [u64; 6],
    /// Carry-bit signature: 1 if lane active, 0 if nullified.
    pub carry_signature: [u8; 6],
}

impl HeadSignature {
    /// Build a signature from a cycle length.
    pub fn from_cycle(name: &'static str, cycle: u64) -> Self {
        let null = nullified_lanes(cycle);
        let act = active_lanes(cycle);
        let addr = cram_address(cycle);
        let mut carry = [0u8; 6];
        for (i, &p) in SAFE_BASIS.iter().enumerate() {
            carry[i] = if cycle % p == 0 { 0 } else { 1 };
        }
        HeadSignature {
            name,
            cycle,
            nullified_lanes: null,
            active_lanes: act,
            address: addr,
            carry_signature: carry,
        }
    }

    /// Number of active discriminating lanes (excludes the parking
    /// lanes {2, 5} which are not coprime to the standard SCALE).
    pub fn active_discriminating_lanes(&self) -> Vec<u64> {
        self.active_lanes.iter()
            .filter(|&&p| p != 2 && p != 5)
            .copied()
            .collect()
    }
}

/// A complete Hydra head.
#[derive(Clone, Debug)]
pub struct HydraHead {
    /// The head's compact signature.
    pub signature: HeadSignature,
    /// Phase decomposition of the cycle (the multi-phase schema).
    pub schema: MultiPhaseSchema,
    /// Per-lane activation schedule across phases:
    /// for each Safe Basis prime p, the list of phase indices where it is active.
    pub lane_schedule: Vec<(u64, Vec<usize>)>,
}

impl HydraHead {
    /// Construct a head from a name and a phase-interval list.
    /// The schema's total must equal the cycle being projected.
    pub fn new(name: &'static str, phase_intervals: &[u64]) -> Self {
        let schema = MultiPhaseSchema::new(name, phase_intervals);
        let signature = HeadSignature::from_cycle(name, schema.total);
        let lane_schedule = schema.lane_schedule();
        HydraHead { signature, schema, lane_schedule }
    }

    /// Verify the head's structural invariants.
    pub fn verify(&self) -> bool {
        self.schema.verify_composition()
            && self.schema.verify_address_composition()
            && self.signature.cycle == self.schema.total
    }

    /// Number of phases.
    pub fn phase_count(&self) -> usize {
        self.schema.phases.len()
    }

    /// Active phases for a given Safe Basis prime.
    pub fn phases_active_in_lane(&self, p: u64) -> Vec<usize> {
        self.lane_schedule.iter()
            .find(|(prime, _)| *prime == p)
            .map(|(_, phases)| phases.clone())
            .unwrap_or_default()
    }

    /// Number of phases where the given prime is active.
    pub fn lane_activation_count(&self, p: u64) -> usize {
        self.phases_active_in_lane(p).len()
    }

    /// Carry-bit signature of a date under this head's projection.
    ///
    /// The head's projection takes the absolute day count, reduces it
    /// modulo the head's cycle (which is what makes it specific to *this*
    /// head), and then computes the Safe Basis residue signature of
    /// that local-cycle position.
    ///
    /// For each Safe Basis prime p, returns 1 if the local-cycle position's
    /// residue mod p is nonzero, 0 if zero.
    ///
    /// This is genuinely head-specific: different heads with different
    /// cycles produce different local positions and therefore different
    /// signatures for the same absolute day.
    pub fn carry_bits(&self, days_since_epoch: u64) -> [u8; 6] {
        let local = days_since_epoch % self.signature.cycle;
        let addr = cram_address(local);
        let mut bits = [0u8; 6];
        for (i, &r) in addr.iter().enumerate() {
            bits[i] = if r == 0 { 0 } else { 1 };
        }
        bits
    }

    /// CRAM residues of the head's local-cycle position (not bits).
    /// Useful when we need richer information than the binary carry.
    pub fn local_residues(&self, days_since_epoch: u64) -> [u64; 6] {
        let local = days_since_epoch % self.signature.cycle;
        cram_address(local)
    }

    /// Phase index this head is currently in for a given absolute day count.
    pub fn current_phase(&self, days_since_epoch: u64) -> usize {
        let within_cycle = days_since_epoch % self.signature.cycle;
        let mut accumulated = 0u64;
        for (i, phase) in self.schema.phases.iter().enumerate() {
            accumulated += phase.interval;
            if within_cycle < accumulated {
                return i;
            }
        }
        // Fallback (should not be reached if schema is valid)
        self.schema.phases.len() - 1
    }
}

// ═══════════════════════════════════════════════════════════════════
// The four canonical heads
// ═══════════════════════════════════════════════════════════════════

/// Build the canonical Tzolk'in head.
/// Decomposition: 13 trecenas of 20 days each (13 × 20 = 260).
pub fn tzolkin_head() -> HydraHead {
    HydraHead::new("Tzolkin", &[20; 13])
}

/// Build the canonical Haab head.
/// Decomposition: 18 winals of 20 days + Wayeb of 5 (18 × 20 + 5 = 365).
pub fn haab_head() -> HydraHead {
    let mut intervals = vec![20u64; 18];
    intervals.push(5);
    HydraHead::new("Haab", &intervals)
}

/// Build the canonical Calendar Round head.
/// Decomposition: 73 repetitions of Tzolk'in (73 × 260 = 18,980).
pub fn calendar_round_head() -> HydraHead {
    let intervals = vec![260u64; 73];
    HydraHead::new("CalendarRound", &intervals)
}

/// Build the canonical Long Count head.
/// Decomposition: positional, smallest-to-largest place values.
/// 1 kin, 20 kins/winal, 18 winals/tun, 20 tuns/katun, 20 katuns/baktun.
/// One full baktun period = 144,000 days.
pub fn long_count_head() -> HydraHead {
    // Use the baktun structure: 20 katuns of 7200 days each
    // (this is the natural unit at the Maya operational scale)
    HydraHead::new("LongCount", &[7200u64; 20])
}

// ═══════════════════════════════════════════════════════════════════
// Additional heads (candidates for H3)
// ═══════════════════════════════════════════════════════════════════

/// Build the Venus-table head (the 65-cycle conductor).
/// Decomposition: 65 Venus synodic periods (65 × 584 = 37,960).
pub fn venus_table_head() -> HydraHead {
    let intervals = vec![VENUS_SYNODIC; 65];
    HydraHead::new("VenusTable", &intervals)
}

/// Build the Venus-phase head — the 236/90/250/8 decomposition of
/// one synodic period. Already encoded in `dresden_codex`.
pub fn venus_phase_head() -> HydraHead {
    let schema = MultiPhaseSchema::venus_corrections();
    let signature = HeadSignature::from_cycle("VenusPhase", schema.total);
    let lane_schedule = schema.lane_schedule();
    HydraHead { signature, schema, lane_schedule }
}

/// Build the Eclipse-alternation head — the 148/177 coprime pair.
pub fn eclipse_alternation_head() -> HydraHead {
    let schema = MultiPhaseSchema::eclipse_alternation();
    let signature = HeadSignature::from_cycle("EclipseAlt", schema.total);
    let lane_schedule = schema.lane_schedule();
    HydraHead { signature, schema, lane_schedule }
}

/// Build the 819-day Planetary Council head.
/// Decomposition: 9 × 91 (where 91 = 7 × 13).
pub fn planetary_council_head() -> HydraHead {
    let intervals = vec![91u64; 9];
    HydraHead::new("PlanetaryCouncil_819", &intervals)
}

/// Build a candidate Saturn-11² head (Acid's H3 hypothesis: 11² as
/// configuration-navigation prime). The cycle is the smallest interval
/// whose CRAM address has r_11 = 0 and r_11_squared_residue carries
/// nonzero structure. We model this as a 121-day cycle decomposed
/// as 11 × 11 phases.
pub fn saturn_11_squared_head() -> HydraHead {
    HydraHead::new("Saturn_11_squared", &[11u64; 11])
}

/// Build a candidate zodiac-topology head (12-fold structure).
/// 360 days = 12 signs × 30 days each.
pub fn zodiac_topology_head() -> HydraHead {
    HydraHead::new("Zodiac_12fold", &[30u64; 12])
}

/// Build a candidate temperaments head (4-fold humoral structure).
/// 28 days = 4 phases × 7 days each. The 7-day phase echoes the lunar
/// quarter and aligns with the Transport Core prime 7.
pub fn temperaments_4fold_head() -> HydraHead {
    HydraHead::new("Temperaments_4fold", &[7u64; 4])
}

// ═══════════════════════════════════════════════════════════════════
// The Four-Calendar Hydra — the canonical configuration
// ═══════════════════════════════════════════════════════════════════

/// The four canonical calendars assembled as a multi-head Hydra.
#[derive(Clone, Debug)]
pub struct FourCalendarHydra {
    pub tzolkin: HydraHead,
    pub haab: HydraHead,
    pub calendar_round: HydraHead,
    pub long_count: HydraHead,
}

impl FourCalendarHydra {
    /// Construct the canonical four-head Hydra.
    pub fn canonical() -> Self {
        FourCalendarHydra {
            tzolkin: tzolkin_head(),
            haab: haab_head(),
            calendar_round: calendar_round_head(),
            long_count: long_count_head(),
        }
    }

    /// Get all four heads as a slice (for iteration).
    pub fn heads(&self) -> [&HydraHead; 4] {
        [&self.tzolkin, &self.haab, &self.calendar_round, &self.long_count]
    }

    /// Verify all four heads pass structural invariants.
    pub fn verify_all(&self) -> bool {
        self.heads().iter().all(|h| h.verify())
    }

    /// Four-head carry-bit address for a given day count.
    /// Returns one [u8; 6] signature per head, packed in head order:
    /// [tzolkin_bits, haab_bits, cr_bits, long_count_bits].
    pub fn four_head_address(&self, days_since_epoch: u64) -> [[u8; 6]; 4] {
        [
            self.tzolkin.carry_bits(days_since_epoch),
            self.haab.carry_bits(days_since_epoch),
            self.calendar_round.carry_bits(days_since_epoch),
            self.long_count.carry_bits(days_since_epoch),
        ]
    }

    /// Concatenate the four-head address into a flat 24-bit signature.
    /// Bits packed: [tzolkin[0..6], haab[0..6], cr[0..6], lc[0..6]].
    pub fn flat_address(&self, days_since_epoch: u64) -> u32 {
        let addr = self.four_head_address(days_since_epoch);
        let mut bits: u32 = 0;
        for (head_idx, head_bits) in addr.iter().enumerate() {
            for (lane_idx, &bit) in head_bits.iter().enumerate() {
                if bit != 0 {
                    bits |= 1 << (head_idx * 6 + lane_idx);
                }
            }
        }
        bits
    }
}

// ═══════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tzolkin_signature_has_expected_nullification() {
        let h = tzolkin_head();
        // 260 = 2² × 5 × 13 nullifies {2, 5, 13}
        assert!(h.signature.nullified_lanes.contains(&2));
        assert!(h.signature.nullified_lanes.contains(&5));
        assert!(h.signature.nullified_lanes.contains(&13));
        // 260 is active in {3, 7, 11}
        assert!(h.signature.active_lanes.contains(&3));
        assert!(h.signature.active_lanes.contains(&7));
        assert!(h.signature.active_lanes.contains(&11));
    }

    #[test]
    fn haab_signature_nullifies_only_5() {
        let h = haab_head();
        // 365 = 5 × 73 nullifies only {5}
        assert_eq!(h.signature.nullified_lanes, vec![5]);
    }

    #[test]
    fn calendar_round_matches_tzolkin_nullification_plus_more() {
        let h = calendar_round_head();
        // 18,980 = 2² × 5 × 13 × 73
        assert!(h.signature.nullified_lanes.contains(&2));
        assert!(h.signature.nullified_lanes.contains(&5));
        assert!(h.signature.nullified_lanes.contains(&13));
    }

    #[test]
    fn all_four_canonical_heads_verify() {
        let hydra = FourCalendarHydra::canonical();
        assert!(hydra.verify_all());
    }

    #[test]
    fn four_head_address_returns_distinct_addresses_for_different_days() {
        let hydra = FourCalendarHydra::canonical();
        let a = hydra.flat_address(0);
        let b = hydra.flat_address(1);
        // Address at day 0: every cycle is 0 mod cycle (zero residue everywhere)
        // Address at day 1: residue 1 in many lanes
        assert_ne!(a, b);
    }

    #[test]
    fn day_zero_has_zero_address() {
        // At day 0, every cycle has residue 0 in every lane
        let hydra = FourCalendarHydra::canonical();
        let addr = hydra.flat_address(0);
        assert_eq!(addr, 0);
    }

    #[test]
    fn tzolkin_returns_to_zero_in_its_own_lanes_after_260_days() {
        // The Tzolk'in head's discriminating lanes are {2, 5, 13} (the
        // lanes the cycle 260 nullifies). After 260 days, those lanes
        // return to zero. Other lanes (3, 7, 11) are not constrained
        // by Tzolk'in completion — the substrate moves on across
        // those lanes independently.
        let h = tzolkin_head();
        let bits_at_260 = h.carry_bits(260);
        // Tzolk'in's nullified lanes: {2, 5, 13} -> Safe Basis indices 0, 2, 5
        assert_eq!(bits_at_260[0], 0, "lane 2 (Tzolk'in null) should be 0");
        assert_eq!(bits_at_260[2], 0, "lane 5 (Tzolk'in null) should be 0");
        assert_eq!(bits_at_260[5], 0, "lane 13 (Tzolk'in null) should be 0");
        // The other lanes are NOT required to be zero — they reflect
        // the substrate's progression on lanes Tzolk'in does not silence.
    }

    #[test]
    fn haab_returns_to_zero_in_its_own_lanes_after_365_days() {
        // The Haab head's nullified lane is only {5} (since 365 = 5 × 73).
        // After 365 days, lane 5 returns to zero. Other lanes evolve.
        let h = haab_head();
        let bits_at_365 = h.carry_bits(365);
        assert_eq!(bits_at_365[2], 0, "lane 5 (Haab null) should be 0");
    }

    #[test]
    fn venus_phase_head_has_four_phases() {
        let h = venus_phase_head();
        assert_eq!(h.phase_count(), 4);
        assert_eq!(h.signature.cycle, 584);
    }

    #[test]
    fn eclipse_head_has_two_phases() {
        let h = eclipse_alternation_head();
        assert_eq!(h.phase_count(), 2);
        assert_eq!(h.signature.cycle, 148 + 177);
    }

    #[test]
    fn planetary_council_819_factorization() {
        let h = planetary_council_head();
        assert_eq!(h.signature.cycle, 819);
        // 819 = 3² × 7 × 13 → nullifies {3, 7, 13}
        assert!(h.signature.nullified_lanes.contains(&3));
        assert!(h.signature.nullified_lanes.contains(&7));
        assert!(h.signature.nullified_lanes.contains(&13));
    }

    #[test]
    fn saturn_11_squared_head_is_admissible() {
        let h = saturn_11_squared_head();
        assert_eq!(h.signature.cycle, 121);
        // 121 = 11² nullifies only {11}
        assert!(h.signature.nullified_lanes.contains(&11));
        assert_eq!(h.signature.nullified_lanes.len(), 1);
    }

    #[test]
    fn zodiac_12fold_head_structure() {
        let h = zodiac_topology_head();
        assert_eq!(h.signature.cycle, 360);
        // 360 = 2³ × 3² × 5 → nullifies {2, 3, 5}
        assert!(h.signature.nullified_lanes.contains(&2));
        assert!(h.signature.nullified_lanes.contains(&3));
        assert!(h.signature.nullified_lanes.contains(&5));
    }

    #[test]
    fn temperaments_4fold_head_structure() {
        let h = temperaments_4fold_head();
        assert_eq!(h.signature.cycle, 28);
        // 28 = 2² × 7 → nullifies {2, 7}
        assert!(h.signature.nullified_lanes.contains(&2));
        assert!(h.signature.nullified_lanes.contains(&7));
    }

    #[test]
    fn current_phase_advances_through_tzolkin() {
        let h = tzolkin_head();
        // Phase 0 covers days 0-19
        assert_eq!(h.current_phase(0), 0);
        assert_eq!(h.current_phase(19), 0);
        // Phase 1 covers days 20-39
        assert_eq!(h.current_phase(20), 1);
        // Phase 12 (last trecena) covers days 240-259
        assert_eq!(h.current_phase(259), 12);
        // Wraps back to phase 0 at 260
        assert_eq!(h.current_phase(260), 0);
    }
}
