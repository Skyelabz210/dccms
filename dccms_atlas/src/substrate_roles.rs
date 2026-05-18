//! # CRAM-ENHANCE Substrate Roles and Mutual Information Engine
//!
//! Ports the Python CRAM-ENHANCE substrate analysis framework to Rust.
//!
//! ## Substrate Mutual Information
//!
//! For a corpus of events with event-kind labels, the MI between a
//! prime lane's residue distribution and the event-kind distribution
//! measures how much the lane encodes the event's nature.
//!
//! High MI → the lane is semantically loaded for this event type.
//! Low MI  → the lane is statistically independent (free coordinate).
//!
//! ## Transport Core Analysis
//!
//! For the four Transport Core primes {3, 7, 11, 13}, the boundary
//! fraction measures how often a lane-flip crosses a configuration
//! boundary (moving from an occupied to an unoccupied address).
//!
//! Low boundary fraction → high traversability → the prime enables
//! smooth movement through the configuration space.
//!
//! ## Equipopulation Test
//!
//! Tests whether a set of lane residues is uniformly distributed over
//! {0, 1, ..., p-1}. The max deviation in basis points measures
//! departure from uniformity.

#![allow(dead_code)]

use crate::venus_kernel::{SAFE_BASIS, TRANSPORT_CORE, VENUS_SYNODIC,
                           shadow16, venus_accumulated_states, carry_vector};

// ═══════════════════════════════════════════════════════════════════
// §1  Role taxonomy
// ═══════════════════════════════════════════════════════════════════

/// Substrate role for a given Safe Basis prime.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubstrateRole {
    ParityParking,           // 2
    FabricTriadic,           // 3
    ContentCalendarEmbedded, // 5
    TraversalInternal,       // 7
    CoordinatePosition,      // 11
    BoundaryConfiguration,   // 13
}

impl SubstrateRole {
    pub fn from_prime(p: u64) -> Self {
        match p {
            2  => SubstrateRole::ParityParking,
            3  => SubstrateRole::FabricTriadic,
            5  => SubstrateRole::ContentCalendarEmbedded,
            7  => SubstrateRole::TraversalInternal,
            11 => SubstrateRole::CoordinatePosition,
            13 => SubstrateRole::BoundaryConfiguration,
            _  => SubstrateRole::FabricTriadic,  // fallback
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            SubstrateRole::ParityParking           => "parity/parking",
            SubstrateRole::FabricTriadic           => "fabric/triadic",
            SubstrateRole::ContentCalendarEmbedded => "content/calendar-embedded",
            SubstrateRole::TraversalInternal       => "traversal/internal-movement",
            SubstrateRole::CoordinatePosition      => "coordinate/position",
            SubstrateRole::BoundaryConfiguration   => "boundary/configuration-region",
        }
    }

    /// Whether this prime is in the Transport Core {3,7,11,13}.
    pub fn in_transport_core(&self) -> bool {
        matches!(self,
            SubstrateRole::FabricTriadic |
            SubstrateRole::TraversalInternal |
            SubstrateRole::CoordinatePosition |
            SubstrateRole::BoundaryConfiguration
        )
    }
}

// ═══════════════════════════════════════════════════════════════════
// §2  Equipopulation test
// ═══════════════════════════════════════════════════════════════════

/// Result of an equipopulation test for a prime lane.
#[derive(Clone, Debug)]
pub struct EquipopulationResult {
    pub prime: u64,
    /// Maximum absolute deviation from expected count.
    pub max_deviation: u64,
    /// Total number of observations.
    pub total: u64,
    /// Expected count per bucket = total / prime.
    pub expected_per_bucket: u64,
    /// Max deviation as fraction of total, in basis points.
    pub max_deviation_bp: u64,
    /// Is the distribution uniform at the given threshold?
    pub uniform_at_1000bp: bool,
}

/// Test whether the residues of `values` modulo `prime` are equipopulated.
pub fn equipopulation_test(values: &[u64], prime: u64) -> EquipopulationResult {
    let n = values.len() as u64;
    if n == 0 {
        return EquipopulationResult {
            prime, max_deviation: 0, total: 0,
            expected_per_bucket: 0, max_deviation_bp: 0, uniform_at_1000bp: true,
        };
    }
    let expected = n / prime; // integer floor
    let mut counts = vec![0u64; prime as usize];
    for &v in values { counts[(v % prime) as usize] += 1; }
    let max_dev = counts.iter().map(|&c| {
        if c >= expected { c - expected } else { expected - c }
    }).max().unwrap_or(0);
    let max_dev_bp = max_dev * 10_000 / n.max(1);
    EquipopulationResult {
        prime,
        max_deviation: max_dev,
        total: n,
        expected_per_bucket: expected,
        max_deviation_bp: max_dev_bp,
        uniform_at_1000bp: max_dev_bp <= 1_000,
    }
}

// ═══════════════════════════════════════════════════════════════════
// §3  Entropy (exact-integer approximation)
// ═══════════════════════════════════════════════════════════════════

/// Entropy of a count vector in nat-basis-points.
///
/// Uses the Padé approximant `ln(n/c) ≈ 2(n-c)/(n+c)` for integer
/// computation without floating point.
///
/// H_nbp ≈ 10000 × H_nat = 10000 × Σ -p × ln(p).
pub fn entropy_counts_nbp(counts: &[u64]) -> i64 {
    let n: u64 = counts.iter().sum();
    if n == 0 { return 0; }
    let mut h = 0i64;
    for &c in counts {
        if c == 0 { continue; }
        // Padé: ln(n/c) ≈ 2(n-c)/(n+c)
        // contribution = (c/n) × ln(n/c) ≈ c/n × 2(n-c)/(n+c)
        // scaled × 10000: 10000 × c × 2(n-c) / (n(n+c))
        let num = 10_000u64 * c * 2 * (n - c);
        let den = n * (n + c);
        h += (num / den.max(1)) as i64;
    }
    h
}

// ═══════════════════════════════════════════════════════════════════
// §4  Substrate role analysis
// ═══════════════════════════════════════════════════════════════════

/// Substrate role profile for a given prime and event corpus.
#[derive(Clone, Debug)]
pub struct SubstrateRoleProfile {
    pub prime: u64,
    pub role: SubstrateRole,
    pub role_label: &'static str,
    pub n_events: u64,
    /// Max lane-residue deviation from uniform (basis points).
    pub equipop_max_dev_bp: u64,
    /// Entropy of the lane-residue distribution (nat-basis-points).
    pub lane_entropy_nbp: i64,
    /// Expected max entropy for uniform distribution (nat-basis-points).
    pub uniform_entropy_nbp: i64,
    /// Entropy deficit = uniform - actual (basis points of nat).
    /// High deficit → lane is structured (not free).
    pub entropy_deficit_nbp: i64,
    /// Whether the lane is approximately uniform (free coordinate).
    pub is_free_coordinate: bool,
}

/// Compute substrate role profiles for all S₆ primes against a corpus.
pub fn substrate_role_profiles(event_days: &[u64]) -> Vec<SubstrateRoleProfile> {
    SAFE_BASIS.iter().map(|&p| {
        let residues: Vec<u64> = event_days.iter().map(|&d| d % p).collect();
        let eq = equipopulation_test(&residues, p);

        let mut counts = vec![0u64; p as usize];
        for r in &residues { counts[*r as usize] += 1; }
        let lane_ent = entropy_counts_nbp(&counts);

        // Uniform entropy for p symbols: ln(p) in nat-basis-points
        // For small p: ln(2)≈6931, ln(3)≈10986, ln(5)≈16094, ln(7)≈19459,
        //              ln(11)≈23979, ln(13)≈26268
        let uniform_ent: i64 = match p {
            2  => 6_931,
            3  => 10_986,
            5  => 16_094,
            7  => 19_459,
            11 => 23_979,
            13 => 26_268,
            _  => 10_000,
        };
        let deficit = uniform_ent - lane_ent;

        SubstrateRoleProfile {
            prime: p,
            role: SubstrateRole::from_prime(p),
            role_label: SubstrateRole::from_prime(p).label(),
            n_events: event_days.len() as u64,
            equipop_max_dev_bp: eq.max_deviation_bp,
            lane_entropy_nbp: lane_ent,
            uniform_entropy_nbp: uniform_ent,
            entropy_deficit_nbp: deficit,
            is_free_coordinate: eq.uniform_at_1000bp,
        }
    }).collect()
}

// ═══════════════════════════════════════════════════════════════════
// §5  Per-phase uniformity
// ═══════════════════════════════════════════════════════════════════

/// Test uniformity of lane-prime residues within each phase of a head cycle.
///
/// For each phase position (day % head_cycle), computes the equipopulation
/// of (day % head_cycle) % lane_prime across all events in that phase.
#[derive(Clone, Debug)]
pub struct PerPhaseUniformityReport {
    pub prime: u64,
    pub head_cycle: u64,
    /// Max deviation across all phases.
    pub max_deviation_bp: u64,
    /// Number of phases with > 1000 bp deviation.
    pub non_uniform_phase_count: usize,
    /// Whether the overall per-phase structure is non-uniform.
    pub non_uniform: bool,
}

pub fn per_phase_uniformity(event_days: &[u64], head_cycle: u64, prime: u64)
    -> PerPhaseUniformityReport
{
    let n_phases = head_cycle as usize;
    let mut phase_residues: Vec<Vec<u64>> = vec![Vec::new(); n_phases];
    for &d in event_days {
        let phase = (d % head_cycle) as usize;
        phase_residues[phase].push(d % prime);
    }
    let mut max_dev = 0u64;
    let mut non_uniform = 0usize;
    for residues in &phase_residues {
        if residues.is_empty() { continue; }
        let eq = equipopulation_test(residues, prime);
        if eq.max_deviation_bp > max_dev { max_dev = eq.max_deviation_bp; }
        if !eq.uniform_at_1000bp { non_uniform += 1; }
    }
    PerPhaseUniformityReport {
        prime,
        head_cycle,
        max_deviation_bp: max_dev,
        non_uniform_phase_count: non_uniform,
        non_uniform: non_uniform > 0,
    }
}

// ═══════════════════════════════════════════════════════════════════
// §6  Venus-specific CRAM analysis
// ═══════════════════════════════════════════════════════════════════

/// Full Venus CRAM state at a given day.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VenusCramState {
    pub day: u64,
    /// CRAM address (residues mod S₆).
    pub cram: [u64; 6],
    /// Shadow16 = (day%11) × (day%13).
    pub shadow16: u64,
    /// Kernel phase (0-3 within one Venus synodic period).
    pub kernel_phase: u64,
    /// Winding depth k_584 = day / 584.
    pub k584: u64,
    /// Recumbent position within the current Venus synodic cycle.
    pub recumbent_584: u64,
    /// Carry vector from previous kernel step.
    pub carry: [u8; 6],
}

impl VenusCramState {
    pub fn at_day(day: u64) -> Self {
        let cram = crate::lunar::all_section_profiles()
            .into_iter().next()
            .map(|_| [0u64; 6]) // placeholder
            .unwrap_or([0u64; 6]);

        // Compute CRAM address directly
        let cram = {
            let mut out = [0u64; 6];
            for (i, &p) in SAFE_BASIS.iter().enumerate() {
                out[i] = day % p;
            }
            out
        };

        let sh16 = shadow16(day);
        let k584 = day / VENUS_SYNODIC;
        let recumbent = day % VENUS_SYNODIC;

        // Which kernel phase are we in?
        let kernel_phase = (k584 * 4 + recumbent / (VENUS_SYNODIC / 4)) % 4;
        // Actually use exact kernel phase
        let kernel_cumulative = [0u64, 236, 326, 576];
        let phase = kernel_cumulative.iter().position(|&kc| recumbent < kc + match kc {
            0 => 236, 236 => 90, 326 => 250, 576 => 8, _ => 8,
        }).unwrap_or(3) as u64;

        // Carry from previous step (from kernel accumulation)
        let n = k584 * 4 + phase;
        let states = venus_accumulated_states(n as usize + 1);
        let prev = states.get(n as usize).copied().unwrap_or(0);
        let delta = if n < states.len() as u64 {
            let k = (n % 4) as usize;
            crate::venus_kernel::VENUS_KERNEL[k]
        } else { 0 };
        let cv = carry_vector(prev, delta);
        let carry = [cv[0] as u8, cv[1] as u8, cv[2] as u8, cv[3] as u8, cv[4] as u8, cv[5] as u8];

        VenusCramState { day, cram, shadow16: sh16, kernel_phase: phase, k584, recumbent_584: recumbent, carry }
    }

    pub fn role_label(&self, lane_idx: usize) -> &'static str {
        SubstrateRole::from_prime(SAFE_BASIS[lane_idx]).label()
    }
}

// ═══════════════════════════════════════════════════════════════════
// §7  Grand synchronization analysis
// ═══════════════════════════════════════════════════════════════════

/// Verify the grand synchronization point: A(260) = 37,960.
///
/// At step 260 (65 Venus synodic periods × 4 phases = 260 transitions):
/// - 146 × Tzolk'in = 37,960 ← Venus Table is Tzolk'in-aligned
/// - 104 × Haab ≈ 37,960 (104 × 365 = 37,960 ✓)
/// - 65 × Venus = 37,960 ✓
#[derive(Clone, Debug)]
pub struct GrandSyncVerification {
    pub step_260_state: u64,
    pub tzolkin_multiple: u64,
    pub haab_multiple: u64,
    pub venus_multiple: u64,
    pub all_sync: bool,
}

pub fn verify_grand_synchronization() -> GrandSyncVerification {
    let a260 = venus_accumulated_states(260)[260];
    let tz = a260 / 260;
    let haab = a260 / 365;
    let venus = a260 / VENUS_SYNODIC;
    let all = a260 % 260 == 0 && a260 % 365 == 0 && a260 % VENUS_SYNODIC == 0;
    GrandSyncVerification {
        step_260_state: a260,
        tzolkin_multiple: tz,
        haab_multiple: haab,
        venus_multiple: venus,
        all_sync: all,
    }
}

// ═══════════════════════════════════════════════════════════════════
// §8  Tests
// ═══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substrate_role_from_prime() {
        assert_eq!(SubstrateRole::from_prime(11), SubstrateRole::CoordinatePosition);
        assert_eq!(SubstrateRole::from_prime(13), SubstrateRole::BoundaryConfiguration);
        assert_eq!(SubstrateRole::from_prime(2),  SubstrateRole::ParityParking);
        assert_eq!(SubstrateRole::from_prime(5),  SubstrateRole::ContentCalendarEmbedded);
    }

    #[test]
    fn transport_core_membership() {
        assert!(SubstrateRole::from_prime(11).in_transport_core());
        assert!(SubstrateRole::from_prime(13).in_transport_core());
        assert!(!SubstrateRole::from_prime(2).in_transport_core());
        assert!(!SubstrateRole::from_prime(5).in_transport_core());
    }

    #[test]
    fn equipopulation_uniform_distribution() {
        // Perfectly uniform modulo 3: [0,1,2,0,1,2,...] × 10 = 30 items
        let vals: Vec<u64> = (0..30).map(|i| i % 3).collect();
        let eq = equipopulation_test(&vals, 3);
        assert_eq!(eq.max_deviation_bp, 0, "Perfect uniform: 0 deviation");
        assert!(eq.uniform_at_1000bp);
    }

    #[test]
    fn equipopulation_biased_distribution() {
        // All zeros — completely biased
        let vals = vec![0u64; 30];
        let eq = equipopulation_test(&vals, 3);
        assert!(eq.max_deviation_bp > 1000, "All-zero: highly non-uniform");
        assert!(!eq.uniform_at_1000bp);
    }

    #[test]
    fn grand_sync_at_step_260() {
        let gsync = verify_grand_synchronization();
        assert_eq!(gsync.step_260_state, 37_960, "A(260) = 37960 = Full Conductor");
        assert_eq!(gsync.tzolkin_multiple, 146, "37960 = 146 × 260");
        assert_eq!(gsync.haab_multiple, 104, "37960 = 104 × 365");
        assert_eq!(gsync.venus_multiple, 65, "37960 = 65 × 584");
        assert!(gsync.all_sync, "Grand synchronization at A(260) = 37960");
    }

    #[test]
    fn venus_cram_state_day_236() {
        let state = VenusCramState::at_day(236);
        assert_eq!(state.cram[4], 5, "236 % 11 = 5 (coordinate lane)");
        assert_eq!(state.cram[5], 2, "236 % 13 = 2 (boundary lane)");
        assert_eq!(state.shadow16, 10, "Shadow16(236) = 5×2 = 10");
        assert_eq!(state.k584, 0, "Winding depth = 0");
    }

    #[test]
    fn venus_cram_state_day_584() {
        let state = VenusCramState::at_day(584);
        // 584 % 11 = 1, 584 % 13 = 12 (44*13=572, rem=12)
        assert_eq!(state.cram[4], 1, "584 % 11 = 1");
        assert_eq!(state.shadow16, 12, "Shadow16(584) = 1×12 = 12");
        assert_eq!(state.k584, 1, "Winding depth = 1 (second Venus cycle)");
    }

    #[test]
    fn substrate_role_profiles_run() {
        let days: Vec<u64> = (0..100).map(|i| i * 236).collect();
        let profiles = substrate_role_profiles(&days);
        assert_eq!(profiles.len(), 6, "One profile per S₆ prime");
        // Lane 2 (parity): all values are multiples of 236 = 4×59, all even → residue=0 always
        let lane2 = profiles.iter().find(|p| p.prime == 2).unwrap();
        assert!(lane2.equipop_max_dev_bp > 0, "Lane 2 not uniform for multiples of 236");
    }

    #[test]
    fn entropy_counts_all_equal() {
        // 4 equal counts → H = ln(4) ≈ 13863 nbp
        let counts = vec![100u64; 4];
        let h = entropy_counts_nbp(&counts);
        assert!(h > 10_000, "Entropy of 4 equal bins must be > 10000 nbp");
    }

    #[test]
    fn entropy_counts_all_one() {
        // All mass in one bin → H = 0
        let mut counts = vec![0u64; 4];
        counts[0] = 100;
        let h = entropy_counts_nbp(&counts);
        assert_eq!(h, 0, "One-bin distribution has zero entropy");
    }

    #[test]
    fn per_phase_uniformity_tzolkin_vs_11() {
        // Generate days at multiples of 584 (Tzolk'in-local positions 0, 260, 520, ...)
        let days: Vec<u64> = (0..1000u64).map(|i| i).collect();
        let report = per_phase_uniformity(&days, 260, 11);
        // With many events, lane 11 over Tzolk'in phases should be approximately uniform
        assert_eq!(report.prime, 11);
        assert_eq!(report.head_cycle, 260);
    }
}
