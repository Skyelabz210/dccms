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
// §0  Submodules
// ═══════════════════════════════════════════════════════════════════

pub mod sr_distribution;
pub mod shadow_bond;

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

/// Mars synodic period: 780 days. Maya cycle for the Chaak/Mars tables
/// (Dresden Codex pages 29–45). Decomposition: 780 = 2²·3·5·13.
///
/// **Status:** Proven (vault A-10 §A.10.8 residue table, cross-validated
/// against `cram_address` in this crate's test suite).
///
/// **Significance:** the 780-day stride is selective-lane-nullifying on
/// {2,3,5,13}; only lanes 7 and 11 advance under this stride (see
/// `dccms_atlas::substrate_roles` for role taxonomy). This is the
/// canonical Maya stride for isolating the Bridge (7) and Shadow (11)
/// lanes from the surface calendar lanes.
pub const MARS_SYNODIC: u64 = 780;

/// The 819-day count. Maya cycle attested at multiple inscriptions
/// (Tortuguero, Palenque, Yaxchilán). Decomposition: 819 = 3²·7·13.
///
/// **Status:** Proven (vault A-10 §A.10.8 residue table).
///
/// **Significance:** the 819-day count is the **unique** Maya cycle that
/// brings prime 7 into the period surface. Without 819, the Maya calendar
/// system never directly activates lane 7 — it is the "Bridge" cycle in
/// Decoded.md's Ramanujan-gate analysis. Residue signature
/// `(1, 0, 4, 0, 5, 0)` mod Safe Basis.
pub const COUNT_819: u64 = 819;

/// Maya "coprime core" — the subset of the Safe Basis the Maya cycle
/// system explicitly activates: `{2² (=4), 3, 5, 13}`.
///
/// The Maya did not natively address primes 7 and 11 except via the
/// 819-day count (lane 7) and via shadow-displacement detection on
/// lane 11 (the H5 universal-coordinate result). This 4-prime subset
/// covers Tzolk'in (260), Haab (365 via lane 5), Calendar Round (18,980),
/// and Mars (780).
///
/// **Status:** Measured / structurally observed in vault A-10 §A.10.2.1.
pub const MAYA_COPRIME_CORE: [u64; 4] = [4, 3, 5, 13];

/// "Calendar Prime" 73. Appears in Haab (365 = 5·73) and Calendar Round
/// (18,980 = 2²·5·13·73) but is NOT in the canonical Safe Basis.
///
/// **Status:** Proven (factorization). Architectural decision pending:
/// whether to extend `SAFE_BASIS` to include 73, or leave 73 as a
/// "Calendar Prime" sidecar without modifying the canonical 6-lane basis.
/// The current code keeps the 6-lane Safe Basis canonical and exposes
/// `CALENDAR_PRIME` as a separate constant for downstream consumers that
/// need to address Haab/Calendar-Round arithmetic exactly.
pub const CALENDAR_PRIME: u64 = 73;

// ─────────────────────────────────────────────────────────────────────
// Shadow Disambiguator (SD-11) — per vault §09
// ─────────────────────────────────────────────────────────────────────

/// Extended Safe Basis 𝒮 = {2, 3, 5, 7, 11, 13, 17, 19}. The canonical
/// 8-prime CRAM basis of the framework. The Dresden Codex application
/// uses the 6-prime subset {2..13}; SD-11 and chimera disambiguation
/// require all 8.
pub const SAFE_BASIS_S8: [u64; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

/// Shadow anchor value 11⁶ = 1,771,561.
///
/// The vorticity-wraparound bound for the N=32 Navier-Stokes solver
/// per vault §09.2.3 (S3 property). For Dresden Codex usage it is the
/// scale at which the shadow lane (p=11) would wrap, well above any
/// Maya cycle period in the canonical corpus.
pub const SHADOW_ANCHOR_11_POW_6: u64 = 1_771_561;

/// Shadow Disambiguator anchor set {11⁶, 13, 17, 19}. Pairwise coprime
/// per vault §09.4.2. Forms a valid CRT decomposition with product
/// 11⁶ · 13 · 17 · 19 = 7,437,683,639.
///
/// This is the corrected anchor set (vault §09.4.3). The earlier
/// formulation {11², 11³, 11⁴} = {121, 1331, 14641} was internally
/// consistent but NOT pairwise coprime, so it did not form a CRT
/// decomposition.
pub const SHADOW_ANCHOR_SET: [u64; 4] = [SHADOW_ANCHOR_11_POW_6, 13, 17, 19];

/// Product of the shadow anchor set: 11⁶ · 13 · 17 · 19 = **7,438,784,639**.
///
/// **Status:** Proven by direct integer multiplication.
///
/// **Note on vault correction:** Vault §09.4.2 publishes this product
/// as 7,437,683,639. The published value does not match direct
/// computation; further, 7,437,683,639 mod 4199 = 3337 ≠ 0, so it
/// cannot be a multiple of 13·17·19 = 4199 — it is not a product of
/// those primes with any integer. The discrepancy of 1,101,000 looks
/// like a transcription error. The structural role of the anchor set
/// is unaffected; only the published numeric label was wrong.
pub const SHADOW_ANCHOR_PRODUCT: u64 = 7_438_784_639;

// ─────────────────────────────────────────────────────────────────────
// Extended planetary periods (per vault `The Dresden Codex.md` §A6, §C3,
// and synthesis report §3.3, §7)
// ─────────────────────────────────────────────────────────────────────

/// Jupiter synodic period: 399 days. Decomposition: 3 · 7 · 19.
///
/// **Status:** Proven (Maya astronomy; vault A6).
///
/// At the 33-year eclipse epoch T_E = 11,960, Jupiter's displacement is
/// 11,960 mod 399 = 389. **389 is prime** — outside both the Safe Basis
/// and the Ramanujan set S_R = {5, 7, 11}. Synthesis bonus finding:
/// Jupiter carries no S_R prime in its T_E displacement, which is why
/// T10's S_R-completeness result (Mars + Venus + Saturn) is the *minimal*
/// recovery set, not an artificial selection.
pub const JUPITER_SYNODIC: u64 = 399;

/// Saturn synodic period: 378 days. Decomposition: 2 · 3³ · 7.
///
/// **Status:** Proven (vault A6, T8).
///
/// At T_E = 11,960, Saturn's displacement is 242 = 2 · 11². This is the
/// **T8 / T-SHADOW-POWER** signature: the missing S_R prime 11 appears
/// **squared** in Saturn's displacement, providing the deepest "shadow
/// bond" (Decoded.md §Algorithm 9).
pub const SATURN_SYNODIC: u64 = 378;

/// Mercury synodic period: 116 days. Decomposition: 2² · 29.
///
/// **Status:** Proven (vault C3).
///
/// At T_E = 11,960, Mercury's displacement is 12 = 2² · 3 — only
/// parking-lane (2) and stability-floor (3) primes. Like Jupiter,
/// Mercury contributes no S_R content, supporting T10's completeness
/// claim for Mars/Venus/Saturn.
pub const MERCURY_SYNODIC: u64 = 116;

/// Eclipse table length: 11,960 days = 405 synodic months = 46 Tzolk'in.
/// Decomposition: 2³ · 5 · 13 · 23.
///
/// **Status:** Proven. The 33-year eclipse epoch `T_E` against which all
/// planetary displacements in T7-T10 are measured.
pub const ECLIPSE_TABLE_DAYS: u64 = 11_960;

/// Eclipse table correction term: 93 days. Decomposition: 3 · 31.
///
/// **Status:** Proven. Used in two-phase K-Elim lift over the coprime
/// pair (11960, 93). gcd(11960, 93) = 1 enables the lift (vault T5).
pub const ECLIPSE_CORRECTION: u64 = 93;

/// 33-year epoch in days: 12,053 ≈ 33 · 365.25.
///
/// **Status:** Proven.
///
/// Distinct from ECLIPSE_TABLE_DAYS (11,960): the 33-year solar epoch is
/// slightly longer than the eclipse table. The eclipse correction (93)
/// accounts for the difference: 11,960 + 93 = 12,053.
pub const EPOCH_33_YEAR: u64 = 12_053;

/// Venus-Haab LCM: 2,920 days = 5 · 584 = 8 · 365. Decomposition: 2³ · 5 · 73.
///
/// **Status:** Proven (vault T6).
///
/// The "Calendar Round of Venus" — 8 solar years = 5 Venus synodic periods.
/// Encodes the 8/5 ≈ φ Venus-Earth synchronization in exact integers.
pub const VENUS_HAAB_LCM: u64 = 2_920;

/// 13-Baktun Long Count: 1,872,000 days = 13 · 144,000. Decomposition: 2⁷ · 3² · 5³ · 13.
///
/// **Status:** Proven (vault T4).
///
/// The full Maya Long Count cycle. As a CRAM address this requires
/// winding > 0 (since 1,872,000 > M_SAFE = 30,030). Reconstruction needs
/// Recombinant CRT (vault §07) or careful u64 tracking.
pub const LONG_COUNT_13_BAKTUN: u64 = 1_872_000;

/// Lunar nodal cycle in days: 6,793 ≈ 18.6 · 365.
///
/// **Status:** Measured (vault OP-DRESDEN-1; synthesis confirmed
/// 6793 mod 11 = 6 ≠ 0). The lunar nodal cycle does NOT bring prime 11
/// into the period surface either, leaving prime 11 still confined to
/// the "displacement interior" of the planetary residue structure.
pub const LUNAR_NODAL: u64 = 6_793;

/// The three Ramanujan partition primes S_R = {5, 7, 11}.
///
/// **Status:** Proven (vault Maya CRT twp-0001 Theorem I).
///
/// First-order Ramanujan partition congruences `p(ℓn + δ) ≡ 0 (mod ℓ)`
/// exist exclusively for these three primes. The Dresden Codex's
/// astronomical use of {5, 7, 11} as the discriminating primes is the
/// reason these three appear in the Safe Basis with distinct roles:
/// 5 = surface (Tzolk'in), 7 = bridge (819-day), 11 = shadow.
pub const RAMANUJAN_S_R: [u64; 3] = [5, 7, 11];

/// Alias of [`COUNT_819`] using the synthesis-recommended name.
///
/// **Status:** Proven. Use either name; both refer to the 819-day count.
pub const CYCLE_819: u64 = COUNT_819;

// ─────────────────────────────────────────────────────────────────────
// SD-11 anchor constants (continued)
// ─────────────────────────────────────────────────────────────────────

/// **Verified** fingerprint of 11⁶ modulo the seven non-11 basis primes,
/// ordered as {mod 2, mod 3, mod 5, mod 7, mod 13, mod 17, mod 19}.
///
/// **Status:** Proven by direct computation (see `sd_11_fingerprint_*`
/// tests in this module).
///
/// **Note on vault correction:** Vault §09.2.4 publishes the fingerprint
/// as `{1, 1, 1, 5, 10, 13, 9}`. This codebase's direct integer
/// computation produces `{1, 1, 1, 1, 12, 8, 1}`. The discrepancy is
/// flagged for vault correction; the structural role of 11⁶ as anchor
/// is unaffected — only the published residue values were transcribed
/// incorrectly. By Fermat's little theorem, 11⁶ ≡ 1 mod 7 (since
/// 6 = 7−1) is forced, ruling out the vault's `5` value at lane 7. The
/// other corrections follow from direct computation.
pub const SD_11_FINGERPRINT: [u64; 7] = [1, 1, 1, 1, 12, 8, 1];

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

    // ─────────────────────────────────────────────────────────────────
    // Vault A-10 §A.10.8 cross-validation
    // ─────────────────────────────────────────────────────────────────
    //
    // The user's HackFate compendium addendum A-10 publishes a residue
    // table for six Maya cycles modulo the Safe Basis. The numbers were
    // independently produced (Kimi → Claude verification chain, per
    // A-10 §A.10.4.1). We cross-check them against `cram_address` here
    // as a second-source regression — divergence means either A-10 is
    // wrong or our substrate is. Both should fail safely if so.
    //
    // Source: ~/Agents/imports/github/HackFate/A-10_dresden_codex.md §A.10.8

    #[test]
    fn a10_residue_table_260_tzolkin() {
        // 260 = 4 · 5 · 13;  expected mod {2,3,5,7,11,13} = (0, 2, 0, 1, 7, 0)
        assert_eq!(cram_address(260), [0, 2, 0, 1, 7, 0]);
        assert_eq!(260, 4 * 5 * 13);
    }

    #[test]
    fn a10_residue_table_365_haab() {
        // 365 = 5 · 73;  expected (1, 2, 0, 1, 2, 1)
        assert_eq!(cram_address(365), [1, 2, 0, 1, 2, 1]);
        assert_eq!(365, 5 * CALENDAR_PRIME);
    }

    #[test]
    fn a10_residue_table_584_venus() {
        // 584 = 2³ · 73;  expected (0, 2, 4, 3, 1, 12)
        assert_eq!(cram_address(VENUS_SYNODIC), [0, 2, 4, 3, 1, 12]);
        assert_eq!(VENUS_SYNODIC, 8 * CALENDAR_PRIME);
    }

    #[test]
    fn a10_residue_table_780_mars() {
        // 780 = 4 · 3 · 5 · 13 = lcm(Maya coprime core);
        // expected (0, 0, 0, 3, 10, 0) — selective lane nullification on {2,3,5,13}
        assert_eq!(cram_address(MARS_SYNODIC), [0, 0, 0, 3, 10, 0]);
        // Mars stride is lcm of the Maya coprime core {4,3,5,13}.
        let core = MAYA_COPRIME_CORE;
        fn lcm(a: u64, b: u64) -> u64 {
            fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a % b) } }
            a / gcd(a, b) * b
        }
        let core_lcm = core.iter().fold(1u64, |acc, &p| lcm(acc, p));
        assert_eq!(MARS_SYNODIC, core_lcm);
        // Selective lane nullification: only lanes 7 and 11 are active.
        assert_eq!(active_lanes(MARS_SYNODIC), vec![7, 11]);
        assert_eq!(nullified_lanes(MARS_SYNODIC), vec![2, 3, 5, 13]);
    }

    #[test]
    fn a10_residue_table_819_count() {
        // 819 = 3² · 7 · 13;  expected (1, 0, 4, 0, 5, 0)
        // The unique Maya cycle that brings p=7 into the period surface.
        assert_eq!(cram_address(COUNT_819), [1, 0, 4, 0, 5, 0]);
        assert_eq!(COUNT_819, 9 * 7 * 13);
        // Bridge-lane signature: 819 is divisible by 7 (Bridge prime nullified
        // in this cycle's surface, meaning the cycle ITSELF acts on lane 7).
        assert!(nullified_lanes(COUNT_819).contains(&7));
    }

    #[test]
    fn a10_residue_table_18980_calendar_round() {
        // 18,980 = 2² · 5 · 13 · 73;  expected (0, 2, 0, 3, 5, 0)
        assert_eq!(cram_address(CALENDAR_ROUND), [0, 2, 0, 3, 5, 0]);
        assert_eq!(CALENDAR_ROUND, 4 * 5 * 13 * CALENDAR_PRIME);
        // CALENDAR_PRIME (73) is required to factor 18980 exactly,
        // but is NOT in the canonical 6-lane Safe Basis.
        assert!(!SAFE_BASIS.contains(&CALENDAR_PRIME));
    }

    #[test]
    fn maya_coprime_core_is_subset_of_safe_basis_or_powers() {
        // Maya coprime core = {4, 3, 5, 13}. The 4 is 2² — its prime
        // factor (2) is in the Safe Basis. The other three (3, 5, 13)
        // are direct members.
        assert_eq!(MAYA_COPRIME_CORE, [4u64, 3, 5, 13]);
        for &p in &MAYA_COPRIME_CORE {
            let is_prime_in_basis = SAFE_BASIS.contains(&p);
            let is_prime_power = p == 4; // 2²
            assert!(is_prime_in_basis || is_prime_power,
                "Maya-core element {} must be a Safe-Basis prime or its power", p);
        }
    }

    #[test]
    fn mars_stride_silences_safe_basis_minus_seven_eleven() {
        // Decoded.md §Algorithm 6 — 780-day stride nullifies {2,3,5,13}.
        // Quoted deltas: 780 mod 7 = 3, 780 mod 11 = 10.
        assert_eq!(MARS_SYNODIC % 7, 3);
        assert_eq!(MARS_SYNODIC % 11, 10);
        for &p in &[2u64, 3, 5, 13] {
            assert_eq!(MARS_SYNODIC % p, 0, "MARS_SYNODIC must be divisible by {}", p);
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Shadow Disambiguator (SD-11) — vault §09 verification
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn sd_11_safe_basis_s8_has_eight_primes() {
        assert_eq!(SAFE_BASIS_S8.len(), 8);
        assert_eq!(SAFE_BASIS_S8, [2u64, 3, 5, 7, 11, 13, 17, 19]);
        // The first six match the canonical SAFE_BASIS used for Dresden.
        for i in 0..6 {
            assert_eq!(SAFE_BASIS_S8[i], SAFE_BASIS[i]);
        }
    }

    #[test]
    fn sd_11_anchor_value_is_eleven_to_sixth() {
        let mut x: u64 = 1;
        for _ in 0..6 { x *= 11; }
        assert_eq!(x, SHADOW_ANCHOR_11_POW_6);
        assert_eq!(SHADOW_ANCHOR_11_POW_6, 1_771_561);
    }

    #[test]
    fn sd_11_anchor_set_is_pairwise_coprime() {
        // Vault §09.4.2: anchor set must form a valid CRT decomposition.
        fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a % b) } }
        for i in 0..SHADOW_ANCHOR_SET.len() {
            for j in (i + 1)..SHADOW_ANCHOR_SET.len() {
                let g = gcd(SHADOW_ANCHOR_SET[i], SHADOW_ANCHOR_SET[j]);
                assert_eq!(g, 1,
                    "anchor[{}]={} and anchor[{}]={} must be coprime (gcd={})",
                    i, SHADOW_ANCHOR_SET[i], j, SHADOW_ANCHOR_SET[j], g);
            }
        }
    }

    #[test]
    fn sd_11_anchor_product_matches_constant() {
        // Vault §09.4.2: anchor product = 11⁶ · 13 · 17 · 19.
        //
        // Computed directly:
        //   1,771,561 · 13         = 23,030,293
        //   23,030,293 · 17        = 391,514,981
        //   391,514,981 · 19       = 7,438,784,639
        //
        // The vault publishes 7,437,683,639. That value is not a
        // multiple of 4199 = 13·17·19, so it cannot be a product of
        // those primes with any integer. See SHADOW_ANCHOR_PRODUCT
        // doc comment for the transcription-error analysis.
        let mut prod: u64 = 1;
        for &p in &SHADOW_ANCHOR_SET {
            prod *= p;
        }
        assert_eq!(prod, SHADOW_ANCHOR_PRODUCT);
        assert_eq!(SHADOW_ANCHOR_PRODUCT,
            SHADOW_ANCHOR_11_POW_6 * 13 * 17 * 19);
        assert_eq!(SHADOW_ANCHOR_PRODUCT, 7_438_784_639);
        // The vault's published 7,437,683,639 is NOT a multiple of 4199.
        // Locked in as a Proven-by-arithmetic non-match (negative test).
        assert_ne!(7_437_683_639u64 % 4199, 0,
            "vault's published anchor product 7,437,683,639 is not a multiple of 13·17·19");
    }

    #[test]
    fn sd_11_fingerprint_matches_direct_computation() {
        // The corrected fingerprint per direct integer computation.
        // Vault §09.2.4 publishes {1, 1, 1, 5, 10, 13, 9}; this codebase's
        // computation produces {1, 1, 1, 1, 12, 8, 1}. By Fermat's little
        // theorem, 11⁶ mod 7 = 1 is forced (since 6 = 7-1), ruling out
        // the vault's `5` entry. The other lanes are confirmed below.
        let basis_minus_11 = [2u64, 3, 5, 7, 13, 17, 19];
        let computed: Vec<u64> = basis_minus_11.iter()
            .map(|&p| SHADOW_ANCHOR_11_POW_6 % p).collect();
        assert_eq!(computed.as_slice(), &SD_11_FINGERPRINT[..]);
    }

    #[test]
    fn sd_11_fingerprint_structural_properties() {
        // The corrected fingerprint exhibits a cleaner structure than
        // the vault's published values: 11⁶ acts as identity in five
        // lanes {2, 3, 5, 7, 19} and is non-trivial only in {13, 17}.
        let idx_2  = 0;  // basis-minus-11 ordering
        let idx_3  = 1;
        let idx_5  = 2;
        let idx_7  = 3;
        let idx_13 = 4;
        let idx_17 = 5;
        let idx_19 = 6;
        for i in [idx_2, idx_3, idx_5, idx_7, idx_19] {
            assert_eq!(SD_11_FINGERPRINT[i], 1,
                "lane index {} should be identity (≡1)", i);
        }
        // Lane 13: 11⁶ ≡ 12 ≡ −1 mod 13 (boundary-negation signature).
        assert_eq!(SD_11_FINGERPRINT[idx_13], 12);
        assert_eq!(SD_11_FINGERPRINT[idx_13] + 1, 13);
        // Lane 17: 11⁶ ≡ 8 = 2³ mod 17 (cubic-root structure).
        assert_eq!(SD_11_FINGERPRINT[idx_17], 8);
        assert_eq!(SD_11_FINGERPRINT[idx_17], 2u64.pow(3));
    }

    #[test]
    fn sd_11_property_s1_legendre_minus_one() {
        // S1: ((-1)/p) = -1 means -1 is a quadratic non-residue mod p.
        // Equivalent to p ≡ 3 mod 4.
        // 11 mod 4 = 3 → S1 holds for 11. [verified]
        // 13 mod 4 = 1 → S1 fails. [vault confirms]
        // 17 mod 4 = 1 → S1 fails. [vault confirms]
        // 19 mod 4 = 3 → S1 holds for 19. [vault notes 19 also passes]
        assert_eq!(11u64 % 4, 3);
        assert_eq!(13u64 % 4, 1);
        assert_eq!(17u64 % 4, 1);
        assert_eq!(19u64 % 4, 3);
    }

    // ─────────────────────────────────────────────────────────────────
    // Extended planetary period verification (B-1 + B-4)
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn jupiter_synodic_factorization() {
        assert_eq!(JUPITER_SYNODIC, 399);
        assert_eq!(JUPITER_SYNODIC, 3 * 7 * 19);
    }

    #[test]
    fn saturn_synodic_factorization() {
        assert_eq!(SATURN_SYNODIC, 378);
        assert_eq!(SATURN_SYNODIC, 2 * 3 * 3 * 3 * 7);
    }

    #[test]
    fn mercury_synodic_factorization() {
        assert_eq!(MERCURY_SYNODIC, 116);
        assert_eq!(MERCURY_SYNODIC, 4 * 29);
    }

    #[test]
    fn eclipse_table_factorization_and_lunations() {
        assert_eq!(ECLIPSE_TABLE_DAYS, 11_960);
        assert_eq!(ECLIPSE_TABLE_DAYS, 8 * 5 * 13 * 23);
        // 46 Tzolk'in periods of 260 days each.
        assert_eq!(ECLIPSE_TABLE_DAYS, 46 * 260);
        // 405 lunations of (mean) 29.53... — the integer table approximation.
    }

    #[test]
    fn eclipse_correction_coprime_with_eclipse_table() {
        assert_eq!(ECLIPSE_CORRECTION, 93);
        assert_eq!(ECLIPSE_CORRECTION, 3 * 31);
        fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a % b) } }
        // Vault T5: gcd(11960, 93) = 1 enables the two-phase K-Elim lift.
        assert_eq!(gcd(ECLIPSE_TABLE_DAYS, ECLIPSE_CORRECTION), 1);
    }

    #[test]
    fn epoch_33_year_is_eclipse_plus_correction() {
        assert_eq!(EPOCH_33_YEAR, 12_053);
        assert_eq!(EPOCH_33_YEAR, ECLIPSE_TABLE_DAYS + ECLIPSE_CORRECTION);
    }

    #[test]
    fn venus_haab_lcm_factors_correctly() {
        assert_eq!(VENUS_HAAB_LCM, 2_920);
        assert_eq!(VENUS_HAAB_LCM, 5 * VENUS_SYNODIC);
        assert_eq!(VENUS_HAAB_LCM, 8 * 365);
        assert_eq!(VENUS_HAAB_LCM, 8 * 5 * CALENDAR_PRIME);
    }

    #[test]
    fn long_count_13_baktun() {
        assert_eq!(LONG_COUNT_13_BAKTUN, 1_872_000);
        assert_eq!(LONG_COUNT_13_BAKTUN, 13 * BAKTUN);
        // Exceeds M_SAFE — requires Recombinant CRT for residue-only reconstruction.
        assert!(LONG_COUNT_13_BAKTUN > M_SAFE);
        // Winding count over M_SAFE.
        assert_eq!(LONG_COUNT_13_BAKTUN / M_SAFE, 62);
    }

    #[test]
    fn lunar_nodal_does_not_introduce_lane_11() {
        // Vault OP-DRESDEN-1 + synthesis bonus arithmetic.
        // 6793 = 617·11 + 6, so 6793 mod 11 = 6 ≠ 0.
        assert_eq!(LUNAR_NODAL, 6_793);
        assert_eq!(LUNAR_NODAL % 11, 6);
        // Full Safe Basis signature per synthesis Appendix B item 2.
        assert_eq!(cram_address(LUNAR_NODAL), [1, 1, 3, 3, 6, 7]);
        // Closes part of OP-DRESDEN-1: the lunar nodal cycle does NOT
        // restore the 11-channel into the period surface.
        assert!(active_lanes(LUNAR_NODAL).contains(&11));
        assert_ne!(LUNAR_NODAL % 11, 0);
    }

    #[test]
    fn ramanujan_s_r_is_5_7_11() {
        assert_eq!(RAMANUJAN_S_R, [5u64, 7, 11]);
        // All three are in the Safe Basis.
        for &p in &RAMANUJAN_S_R {
            assert!(SAFE_BASIS.contains(&p), "S_R prime {} must be in Safe Basis", p);
        }
    }

    #[test]
    fn cycle_819_alias() {
        assert_eq!(CYCLE_819, COUNT_819);
        assert_eq!(CYCLE_819, 819);
    }

    // ─────────────────────────────────────────────────────────────────
    // T10 — S_R Distribution at T_E = 11,960 (preview; full module in B-2)
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn t9_mars_displacement_at_t_e_is_tzolkin() {
        // Vault T9: Mars displacement at T_E = 11,960 equals exactly one Tzolk'in.
        let displacement = ECLIPSE_TABLE_DAYS % MARS_SYNODIC;
        assert_eq!(displacement, 260);
        assert_eq!(displacement, 4 * 5 * 13); // = T_tz factorization
        // Mars carries S_R prime 5 in its displacement.
        assert_eq!(displacement % 5, 0);
    }

    #[test]
    fn t10_venus_displacement_at_t_e_carries_5_and_7() {
        // Vault T10: Venus displacement at T_E is 280 = 2³·5·7.
        let displacement = ECLIPSE_TABLE_DAYS % VENUS_SYNODIC;
        assert_eq!(displacement, 280);
        assert_eq!(displacement, 8 * 5 * 7);
        // Venus carries S_R primes {5, 7} in its displacement.
        assert_eq!(displacement % 5, 0);
        assert_eq!(displacement % 7, 0);
    }

    #[test]
    fn t8_saturn_displacement_at_t_e_is_two_eleven_squared() {
        // Vault T8 / T-SHADOW-POWER: Saturn displacement = 242 = 2·11².
        let displacement = ECLIPSE_TABLE_DAYS % SATURN_SYNODIC;
        assert_eq!(displacement, 242);
        assert_eq!(displacement, 2 * 11 * 11);
        // Saturn carries 11² in its displacement — the deep shadow signature.
        assert_eq!(displacement % (11 * 11), 0);
    }

    #[test]
    fn jupiter_displacement_at_t_e_is_389_prime() {
        // Synthesis bonus discovery: Jupiter at T_E = 11,960 has
        // displacement 389, which is prime and outside both Safe Basis
        // and S_R. This supports T10's S_R-completeness for Mars/Venus/Saturn.
        let displacement = ECLIPSE_TABLE_DAYS % JUPITER_SYNODIC;
        assert_eq!(displacement, 389);
        // Verify 389 is prime: not divisible by any prime ≤ √389 ≈ 19.7.
        for p in [2u64, 3, 5, 7, 11, 13, 17, 19] {
            assert_ne!(displacement % p, 0,
                "if 389 mod {} == 0, then 389 isn't prime", p);
        }
        // Jupiter carries no S_R prime in its T_E displacement.
        for &p in &RAMANUJAN_S_R {
            assert_ne!(displacement % p, 0,
                "Jupiter displacement should not carry S_R prime {}", p);
        }
    }

    #[test]
    fn mercury_displacement_at_t_e_is_parking_plus_stability() {
        // Synthesis bonus: Mercury at T_E = 11,960 has displacement
        // 12 = 2²·3 — only parking (2) and stability-floor (3) primes,
        // no S_R content.
        let displacement = ECLIPSE_TABLE_DAYS % MERCURY_SYNODIC;
        assert_eq!(displacement, 12);
        assert_eq!(displacement, 4 * 3);
        // No S_R primes.
        for &p in &RAMANUJAN_S_R {
            assert_ne!(displacement % p, 0,
                "Mercury displacement should not carry S_R prime {}", p);
        }
    }

    #[test]
    fn t10_complete_s_r_distribution_at_t_e() {
        // The headline T10 claim: at T_E = 11,960, the complete
        // S_R = {5, 7, 11} is distributed across exactly Mars + Venus + Saturn,
        // with no overlap of "missing" or "extra" primes.
        let mars_d   = ECLIPSE_TABLE_DAYS % MARS_SYNODIC;     // 260
        let venus_d  = ECLIPSE_TABLE_DAYS % VENUS_SYNODIC;    // 280
        let saturn_d = ECLIPSE_TABLE_DAYS % SATURN_SYNODIC;   // 242

        // Mars contains 5 (and 13, but 13 is boundary not S_R).
        assert_eq!(mars_d % 5, 0);
        // Venus contains both 5 and 7 (the accessible S_R).
        assert_eq!(venus_d % 5, 0);
        assert_eq!(venus_d % 7, 0);
        // Saturn contains 11² (the deepest shadow-power signature).
        assert_eq!(saturn_d % (11 * 11), 0);

        // Union covers all of S_R = {5, 7, 11}.
        // (Note: lane 5 appears in both Mars and Venus, which is fine —
        // the claim is that S_R is *covered*, not partitioned.)
    }

    #[test]
    fn cycle_819_lane_7_active_via_lane_11_inactive() {
        // Vault T7 / Lemma L7: 819 brings p=7 into the period surface
        // (since 819 = 3²·7·13), but p=11 remains absent.
        assert_eq!(CYCLE_819 % 7, 0);   // 7 active in surface
        assert_eq!(CYCLE_819 % 11, 5);  // 11 absent (residue 5, not 0)
        // 11 ∤ 819 — vault Lemma L7 confirmed.
    }

    #[test]
    fn algo_20_arcsecond_check_278289_mod_11_zero() {
        // Decoded.md §Algorithm 20: 77°18' = 278,289 arcseconds, mod 11 = 0.
        // Arithmetic claim verified (cultural claim about Maya tracking
        // arcseconds remains Open per synthesis O8).
        let arcseconds: u64 = 278_289;
        assert_eq!(arcseconds % 11, 0);
        assert_eq!(arcseconds, 25_299 * 11);
    }

    #[test]
    fn sd_11_property_s2_two_is_primitive_root_mod_11() {
        // S2 (revised, per vault §09.2.2): 2 is a primitive root mod p.
        // For p=11 the multiplicative order of 2 must be exactly 10.
        let mut order = 0u64;
        let mut x = 1u64;
        for k in 1..=11 {
            x = (x * 2) % 11;
            if x == 1 { order = k; break; }
        }
        assert_eq!(order, 10, "order of 2 mod 11 must be 10");
        // Also verify the vault's intermediate residues sequence:
        // 2, 4, 8, 5, 10, 9, 7, 3, 6, 1
        let expected_orbit = [2u64, 4, 8, 5, 10, 9, 7, 3, 6, 1];
        let mut y = 1u64;
        for k in 0..10 {
            y = (y * 2) % 11;
            assert_eq!(y, expected_orbit[k]);
        }
    }
}
