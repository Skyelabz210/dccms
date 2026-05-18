//! # dresden_codex — Safe Basis Substrate for DCCMS
//!
//! The substrate-level crate for the Dresden Codex Configuration
//! Manifold Study. All arithmetic is exact integer. No float.
//!
//! ## Safe Basis (S₆)
//!
//! The six-prime basis used by DCCMS:
//!
//! ```text
//! S₆ = {2, 3, 5, 7, 11, 13}    M_SAFE = 30,030 = 2×3×5×7×11×13
//! ```
//!
//! Every Maya calendar period factors into this basis or has
//! a resonance pattern against it.
//!
//! ## Key periods and their S₆ addresses
//!
//! | Period | Factorization       | Nullified | Active          |
//! |--------|---------------------|-----------|-----------------|
//! | 260    | 2²×5×13             | {2,5,13}  | {3,7,11}        |
//! | 365    | 5×73                | {5}       | {2,3,7,11,13}   |
//! | 584    | 2³×73               | {2}       | {3,5,7,11,13}   |
//! | 18,980 | 2²×5×13×73          | {2,5,13}  | {3,7,11}        |
//! | 144,000| 2⁷×3²×5³            | {2,3,5}   | {7,11,13}       |
//! | 121    | 11²                 | {11}      | {2,3,5,7,13}    |
//! | 819    | 3²×7×13             | {3,7,13}  | {2,5,11}        |
//!
//! ## Venus synodic period
//!
//! 584 days = one Venus synodic period.
//! The Dresden Venus table runs 65×584 = 37,960 days.
//!
//! ## Multi-phase schemas
//!
//! A `MultiPhaseSchema` decomposes a cycle into consecutive
//! sub-intervals, each with its own CRAM address signature.
//! The schema encodes the head's internal phase structure.

#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic)]

// ═══════════════════════════════════════════════════════════════════
// §1  Constants
// ═══════════════════════════════════════════════════════════════════

/// Safe Basis S₆ = {2, 3, 5, 7, 11, 13}.
///
/// DCCMS uses S₆, not the full S₈. The outer primes {17, 19} are
/// not resolved by the Maya corpus at the canonical-corpus scale.
pub const SAFE_BASIS: [u64; 6] = [2, 3, 5, 7, 11, 13];

/// Product of Safe Basis: M_SAFE = 2×3×5×7×11×13 = 30,030.
pub const M_SAFE: u64 = 30_030;

/// Venus synodic period in days.
pub const VENUS_SYNODIC: u64 = 584;

/// Venus table length: 65 synodic periods.
pub const VENUS_TABLE_DAYS: u64 = 37_960;

/// Eclipse near-half-year: 148 days (5 synodic months).
pub const ECLIPSE_NEAR: u64 = 148;

/// Eclipse far-half-year: 177 days (6 synodic months).
pub const ECLIPSE_FAR: u64 = 177;

/// Venus synodic decomposition: morning star, superior, evening, inferior.
pub const VENUS_PHASES: [u64; 4] = [236, 90, 250, 8];

/// Calendar Round: LCM(260, 365) = 18,980 days.
pub const CALENDAR_ROUND: u64 = 18_980;

/// Long Count baktun: 144,000 days.
pub const BAKTUN: u64 = 144_000;

// ═══════════════════════════════════════════════════════════════════
// §2  CRAM address arithmetic
// ═══════════════════════════════════════════════════════════════════

/// Compute the CRAM address of x: residue tuple (x mod p) for each p ∈ S₆.
///
/// Returns `[x%2, x%3, x%5, x%7, x%11, x%13]`.
pub fn cram_address(x: u64) -> [u64; 6] {
    let mut addr = [0u64; 6];
    for (i, &p) in SAFE_BASIS.iter().enumerate() {
        addr[i] = x % p;
    }
    addr
}

/// Safe Basis primes that divide x (residue = 0).
pub fn nullified_lanes(x: u64) -> Vec<u64> {
    SAFE_BASIS.iter().filter(|&&p| x % p == 0).copied().collect()
}

/// Safe Basis primes that do not divide x (residue ≠ 0).
pub fn active_lanes(x: u64) -> Vec<u64> {
    SAFE_BASIS.iter().filter(|&&p| x % p != 0).copied().collect()
}

/// Carry-bit signature of x: 1 for active lanes, 0 for nullified.
pub fn carry_bits(x: u64) -> [u8; 6] {
    let mut bits = [0u8; 6];
    for (i, &p) in SAFE_BASIS.iter().enumerate() {
        bits[i] = if x % p != 0 { 1 } else { 0 };
    }
    bits
}

/// Pack a carry-bit signature into a 6-bit integer (bit 0 = lane 2).
pub fn pack_carry_bits(bits: &[u8; 6]) -> u8 {
    let mut out = 0u8;
    for (i, &b) in bits.iter().enumerate() {
        if b != 0 { out |= 1 << i; }
    }
    out
}

// ═══════════════════════════════════════════════════════════════════
// §3  MultiPhaseSchema
// ═══════════════════════════════════════════════════════════════════

/// A single phase interval within a multi-phase schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Phase {
    /// Length of this phase in days.
    pub interval: u64,
}

impl Phase {
    pub fn new(interval: u64) -> Self {
        Phase { interval }
    }

    /// CRAM address of this phase's interval length.
    pub fn cram_address(&self) -> [u64; 6] {
        cram_address(self.interval)
    }

    /// Whether a given prime p is active (not nullified) for this phase.
    pub fn prime_active(&self, p: u64) -> bool {
        self.interval % p != 0
    }
}

/// A multi-phase decomposition of a calendar cycle.
///
/// Encodes the internal structure of a Hydra head: how the cycle
/// is subdivided into consecutive intervals, each with its own
/// carry-bit signature.
#[derive(Clone, Debug)]
pub struct MultiPhaseSchema {
    /// Name of the head this schema belongs to.
    pub name: String,
    /// Ordered phase intervals.
    pub phases: Vec<Phase>,
    /// Total cycle length = sum of all intervals.
    pub total: u64,
}

impl MultiPhaseSchema {
    // ─── Construction ────────────────────────────────────────────

    /// Build from a name and a list of phase interval lengths.
    pub fn new(name: &str, intervals: &[u64]) -> Self {
        let phases: Vec<Phase> = intervals.iter().map(|&i| Phase::new(i)).collect();
        let total: u64 = intervals.iter().sum();
        MultiPhaseSchema { name: name.to_string(), phases, total }
    }

    /// Venus corrections schema: 236/90/250/8 (one synodic period).
    pub fn venus_corrections() -> Self {
        MultiPhaseSchema::new("VenusPhase", &VENUS_PHASES)
    }

    /// Eclipse alternation schema: 148/177 (near/far half-year).
    pub fn eclipse_alternation() -> Self {
        MultiPhaseSchema::new("EclipseAlt", &[ECLIPSE_NEAR, ECLIPSE_FAR])
    }

    /// Uniform n-phase schema: n phases each of length interval_len.
    pub fn uniform(name: &str, count: usize, interval_len: u64) -> Self {
        MultiPhaseSchema::new(name, &vec![interval_len; count])
    }

    // ─── Verification ────────────────────────────────────────────

    /// Verify that the sum of phase intervals equals the total.
    pub fn verify_composition(&self) -> bool {
        self.phases.iter().map(|p| p.interval).sum::<u64>() == self.total
    }

    /// Verify that all phase intervals are positive.
    pub fn verify_address_composition(&self) -> bool {
        !self.phases.is_empty() && self.phases.iter().all(|p| p.interval > 0)
    }

    // ─── Lane schedule ───────────────────────────────────────────

    /// For each Safe Basis prime, list the phase indices where it is active.
    ///
    /// Returns a Vec of (prime, [active_phase_indices]).
    pub fn lane_schedule(&self) -> Vec<(u64, Vec<usize>)> {
        SAFE_BASIS.iter().map(|&p| {
            let active: Vec<usize> = self.phases.iter().enumerate()
                .filter(|(_, phase)| phase.prime_active(p))
                .map(|(i, _)| i)
                .collect();
            (p, active)
        }).collect()
    }

    /// Number of phases where a given prime is active.
    pub fn lane_activation_count(&self, p: u64) -> usize {
        self.phases.iter().filter(|phase| phase.prime_active(p)).count()
    }

    /// CRAM address of the i-th phase interval.
    pub fn phase_address(&self, i: usize) -> Option<[u64; 6]> {
        self.phases.get(i).map(|p| p.cram_address())
    }

    /// Total CRAM address of the full cycle (= cram_address(total)).
    pub fn cycle_address(&self) -> [u64; 6] {
        cram_address(self.total)
    }

    /// Whether all phases have the same interval length.
    pub fn is_uniform(&self) -> bool {
        self.phases.windows(2).all(|w| w[0].interval == w[1].interval)
    }

    /// Carry-bit pattern of each phase (for fingerprinting).
    pub fn phase_carry_patterns(&self) -> Vec<[u8; 6]> {
        self.phases.iter().map(|p| {
            let addr = p.cram_address();
            let mut bits = [0u8; 6];
            for (i, &r) in addr.iter().enumerate() {
                bits[i] = if r != 0 { 1 } else { 0 };
            }
            bits
        }).collect()
    }
}

// ═══════════════════════════════════════════════════════════════════
// §4  Calendar-specific schemas (pre-built)
// ═══════════════════════════════════════════════════════════════════

/// Tzolk'in schema: 13 trecenas of 20 days (13 × 20 = 260).
pub fn tzolkin_schema() -> MultiPhaseSchema {
    MultiPhaseSchema::uniform("Tzolkin", 13, 20)
}

/// Haab schema: 18 winals of 20 + Wayeb of 5 (18×20 + 5 = 365).
pub fn haab_schema() -> MultiPhaseSchema {
    let mut intervals = vec![20u64; 18];
    intervals.push(5);
    MultiPhaseSchema::new("Haab", &intervals)
}

/// Calendar Round schema: 73 × 260.
pub fn calendar_round_schema() -> MultiPhaseSchema {
    MultiPhaseSchema::uniform("CalendarRound", 73, 260)
}

/// Long Count schema: 20 katuns of 7200 days (= one baktun).
pub fn long_count_schema() -> MultiPhaseSchema {
    MultiPhaseSchema::uniform("LongCount", 20, 7200)
}

// ═══════════════════════════════════════════════════════════════════
// §5  Tests
// ═══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ── cram_address ──────────────────────────────────────────────

    #[test]
    fn cram_address_260() {
        let a = cram_address(260);
        // 260 = 2² × 5 × 13
        assert_eq!(a[0], 0); // 260 % 2 = 0
        assert_eq!(a[1], 2); // 260 % 3 = 2
        assert_eq!(a[2], 0); // 260 % 5 = 0
        assert_eq!(a[3], 1); // 260 % 7 = 1
        assert_eq!(a[4], 7); // 260 % 11 = 7
        assert_eq!(a[5], 0); // 260 % 13 = 0
    }

    #[test]
    fn cram_address_365() {
        let a = cram_address(365);
        // 365 = 5 × 73
        assert_eq!(a[2], 0); // 365 % 5 = 0
        assert_ne!(a[0], 0); // 365 % 2 = 1
        assert_ne!(a[1], 0); // 365 % 3 ≠ 0
    }

    #[test]
    fn cram_address_584() {
        let a = cram_address(584);
        // 584 = 2³ × 73
        assert_eq!(a[0], 0); // 584 % 2 = 0
        assert_ne!(a[1], 0); // 584 % 3 ≠ 0
    }

    #[test]
    fn cram_address_zero() {
        let a = cram_address(0);
        assert_eq!(a, [0u64; 6]);
    }

    #[test]
    fn nullified_lanes_260() {
        let n = nullified_lanes(260);
        assert!(n.contains(&2));
        assert!(n.contains(&5));
        assert!(n.contains(&13));
        assert!(!n.contains(&3));
        assert!(!n.contains(&7));
        assert!(!n.contains(&11));
    }

    #[test]
    fn active_lanes_260() {
        let a = active_lanes(260);
        assert!(a.contains(&3));
        assert!(a.contains(&7));
        assert!(a.contains(&11));
        assert!(!a.contains(&2));
    }

    #[test]
    fn nullified_365_is_only_5() {
        let n = nullified_lanes(365);
        assert_eq!(n, vec![5]);
    }

    #[test]
    fn nullified_584_is_only_2() {
        let n = nullified_lanes(584);
        assert_eq!(n, vec![2]);
    }

    #[test]
    fn nullified_121_is_only_11() {
        let n = nullified_lanes(121);
        assert_eq!(n, vec![11]);
    }

    #[test]
    fn nullified_819_is_3_7_13() {
        let n = nullified_lanes(819);
        assert!(n.contains(&3));
        assert!(n.contains(&7));
        assert!(n.contains(&13));
    }

    #[test]
    fn m_safe_is_product_of_safe_basis() {
        let prod: u64 = SAFE_BASIS.iter().product();
        assert_eq!(prod, M_SAFE);
    }

    // ── MultiPhaseSchema ──────────────────────────────────────────

    #[test]
    fn tzolkin_schema_verifies() {
        let s = tzolkin_schema();
        assert!(s.verify_composition());
        assert!(s.verify_address_composition());
        assert_eq!(s.total, 260);
        assert_eq!(s.phases.len(), 13);
    }

    #[test]
    fn haab_schema_verifies() {
        let s = haab_schema();
        assert!(s.verify_composition());
        assert_eq!(s.total, 365);
        assert_eq!(s.phases.len(), 19);
    }

    #[test]
    fn calendar_round_schema_verifies() {
        let s = calendar_round_schema();
        assert!(s.verify_composition());
        assert_eq!(s.total, 18_980);
        assert_eq!(s.phases.len(), 73);
    }

    #[test]
    fn long_count_schema_verifies() {
        let s = long_count_schema();
        assert!(s.verify_composition());
        assert_eq!(s.total, 144_000);
    }

    #[test]
    fn venus_corrections_total_is_584() {
        let s = MultiPhaseSchema::venus_corrections();
        assert_eq!(s.total, VENUS_SYNODIC);
        assert_eq!(s.phases.len(), 4);
        assert!(s.verify_composition());
    }

    #[test]
    fn eclipse_alternation_total_is_325() {
        let s = MultiPhaseSchema::eclipse_alternation();
        assert_eq!(s.total, ECLIPSE_NEAR + ECLIPSE_FAR);
        assert_eq!(s.phases.len(), 2);
        assert!(s.verify_composition());
    }

    #[test]
    fn lane_schedule_identifies_active_primes() {
        let s = tzolkin_schema();
        let schedule = s.lane_schedule();
        // 20 = 2² × 5; so for each phase (interval=20):
        // prime 2: active if 20 % 2 ≠ 0 → FALSE (20 % 2 = 0) → no active phases
        // prime 3: active if 20 % 3 ≠ 0 → TRUE → all 13 phases
        // prime 11: active if 20 % 11 ≠ 0 → TRUE → all 13 phases
        let prime_3 = schedule.iter().find(|(p,_)| *p == 3).unwrap();
        assert_eq!(prime_3.1.len(), 13, "all 13 phases active for prime 3 in Tzolk'in");
        let prime_2 = schedule.iter().find(|(p,_)| *p == 2).unwrap();
        assert_eq!(prime_2.1.len(), 0, "no phases active for prime 2 in Tzolk'in (20 = 2²×5)");
    }

    #[test]
    fn lane_schedule_haab_has_mixed_activity_for_5() {
        let s = haab_schema();
        let schedule = s.lane_schedule();
        // Haab: 18 phases of 20 + 1 phase of 5.
        // 20 % 5 = 0 (nullified), 5 % 5 = 0 (nullified) → prime 5 never active
        let prime_5 = schedule.iter().find(|(p,_)| *p == 5).unwrap();
        assert_eq!(prime_5.1.len(), 0, "prime 5 never active in Haab");
        // 20 % 7 ≠ 0, 5 % 7 ≠ 0 → prime 7 active in all 19 phases
        let prime_7 = schedule.iter().find(|(p,_)| *p == 7).unwrap();
        assert_eq!(prime_7.1.len(), 19, "prime 7 active in all Haab phases");
    }

    #[test]
    fn phase_carry_patterns_reflect_nullification() {
        let s = MultiPhaseSchema::venus_corrections();
        let patterns = s.phase_carry_patterns();
        // Phase 0: 236 = 2² × 59; 236 % 2 = 0 → bit 0 = 0
        assert_eq!(patterns[0][0], 0, "236: prime 2 nullified");
        // Phase 1: 90 = 2 × 3² × 5; 90 % 2 = 0, 90 % 3 = 0, 90 % 5 = 0
        assert_eq!(patterns[1][0], 0, "90: prime 2 nullified");
        assert_eq!(patterns[1][1], 0, "90: prime 3 nullified");
        assert_eq!(patterns[1][2], 0, "90: prime 5 nullified");
        // Phase 2: 250 = 2 × 5³; 250 % 2 = 0, 250 % 5 = 0
        assert_eq!(patterns[2][0], 0, "250: prime 2 nullified");
        assert_eq!(patterns[2][2], 0, "250: prime 5 nullified");
        // Phase 3: 8 = 2³; 8 % 2 = 0
        assert_eq!(patterns[3][0], 0, "8: prime 2 nullified");
    }

    #[test]
    fn uniform_schema_is_detected() {
        let s = tzolkin_schema();
        assert!(s.is_uniform(), "Tzolk'in is uniform (all 20-day trecenas)");
        let s2 = haab_schema();
        assert!(!s2.is_uniform(), "Haab is not uniform (last phase = 5, not 20)");
    }

    #[test]
    fn carry_bits_and_pack_round_trip() {
        let bits = carry_bits(260);
        let packed = pack_carry_bits(&bits);
        // 260 = 2² × 5 × 13: nullified at indices 0,2,5; active at 1,3,4
        // bits = [0,1,0,1,1,0]
        // packed = 0 + 2 + 0 + 8 + 16 + 0 = 26 = 0b011010
        assert_eq!(bits, [0,1,0,1,1,0]);
        assert_eq!(packed, 26); // 2^1 + 2^3 + 2^4 = 2 + 8 + 16 = 26
    }

    #[test]
    fn venus_table_is_65_synodic_periods() {
        assert_eq!(VENUS_TABLE_DAYS, 65 * VENUS_SYNODIC);
    }

    #[test]
    fn calendar_round_is_lcm_260_365() {
        fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a % b) } }
        let lcm = 260 * 365 / gcd(260, 365);
        assert_eq!(CALENDAR_ROUND, lcm);
    }
}
