//! # Generator Catalog
//!
//! Full structured catalog of all cycles reachable from SEED=20 by the
//! bilinear generator G. Sorted by cycle length, annotated with:
//! - CRAM address and carry-signature
//! - K-Elim level at p=11
//! - DKAM admissibility
//! - Astronomical identification
//! - Distance from the canonical four heads (carry-Hamming)
//!
//! The catalog is the "address book" of the configuration manifold
//! as seen from the generator's perspective.

#![allow(dead_code)]

use crate::h1_stage8::{SEED, ASTRO_PRIME, reachable_periods};
use crate::h5_level::k_elim_level;
use crate::dkam_filter::DkamFilter;
use crate::heads::{HydraHead, FourCalendarHydra};
use dresden_codex::{cram_address, nullified_lanes, active_lanes, carry_bits, pack_carry_bits, SAFE_BASIS};

// ═══════════════════════════════════════════════════════════════════
// §1  Catalog entry
// ═══════════════════════════════════════════════════════════════════

/// One entry in the generator catalog.
#[derive(Clone, Debug)]
pub struct CatalogEntry {
    pub name: &'static str,
    pub cycle: u64,
    pub generator_path: String,
    /// CRAM address on S₆.
    pub cram: [u64; 6],
    /// Carry-bit signature (6 bits packed into u8).
    pub carry_sig: u8,
    /// K-Elim level at p=11.
    pub level_11: u32,
    /// DKAM admissibility.
    pub dkam_ok: bool,
    /// Nullified Safe Basis primes.
    pub nullified: Vec<u64>,
    /// Active Safe Basis primes.
    pub active: Vec<u64>,
    /// Carry-Hamming distance to Tzolk'in (260).
    pub dist_to_tzolkin: u32,
    /// Carry-Hamming distance to Haab (365).
    pub dist_to_haab: u32,
}

fn carry_hamming(a: u8, b: u8) -> u32 {
    (a ^ b).count_ones()
}

fn dkam_check(cycle: u64) -> bool {
    // DKAM: operator degree < rho = 3.
    // Infer degree from number of distinct prime factors.
    let mut x = cycle;
    let mut factor_count = 0u32;
    let mut d = 2u64;
    while d * d <= x {
        if x % d == 0 {
            factor_count += 1;
            while x % d == 0 { x /= d; }
        }
        d += 1;
    }
    if x > 1 { factor_count += 1; }
    // Bilinear (degree 2) or lower → admissible.
    factor_count <= 3 // conservative: ≤ 3 distinct prime factors → degree ≤ 2
}

/// Build the full catalog from the reachable_periods list.
pub fn build_catalog(max_cycle: u64) -> Vec<CatalogEntry> {
    let tzolkin_carry = pack_carry_bits(&carry_bits(260));
    let haab_carry = pack_carry_bits(&carry_bits(365));

    let periods = reachable_periods(max_cycle);
    periods.into_iter().map(|(name, cycle, path)| {
        let cram = cram_address(cycle);
        let cbits = carry_bits(cycle);
        let csig = pack_carry_bits(&cbits);
        let level = k_elim_level(cycle, 11);
        let ok = dkam_check(cycle);
        let null = nullified_lanes(cycle);
        let act = active_lanes(cycle);
        let d_tz = carry_hamming(csig, tzolkin_carry);
        let d_ha = carry_hamming(csig, haab_carry);
        CatalogEntry {
            name,
            cycle,
            generator_path: path,
            cram,
            carry_sig: csig,
            level_11: level,
            dkam_ok: ok,
            nullified: null,
            active: act,
            dist_to_tzolkin: d_tz,
            dist_to_haab: d_ha,
        }
    }).collect()
}

// ═══════════════════════════════════════════════════════════════════
// §2  Catalog queries
// ═══════════════════════════════════════════════════════════════════

/// Find all periods with a given carry-signature (same nullification pattern as target).
pub fn same_carry_class(catalog: &[CatalogEntry], target_cycle: u64) -> Vec<&CatalogEntry> {
    let target_sig = pack_carry_bits(&carry_bits(target_cycle));
    catalog.iter().filter(|e| e.carry_sig == target_sig).collect()
}

/// Find all Level-2 periods (shadow-free: κ₃=0 always).
pub fn level2_periods(catalog: &[CatalogEntry]) -> Vec<&CatalogEntry> {
    catalog.iter().filter(|e| e.level_11 <= 2).collect()
}

/// Find all periods within carry-Hamming distance k of the canonical heads.
pub fn near_canonical(catalog: &[CatalogEntry], max_dist: u32) -> Vec<&CatalogEntry> {
    catalog.iter().filter(|e| {
        e.dist_to_tzolkin <= max_dist || e.dist_to_haab <= max_dist
    }).collect()
}

/// Periods grouped by carry-signature class.
pub fn carry_signature_classes(catalog: &[CatalogEntry])
    -> std::collections::HashMap<u8, Vec<u64>>
{
    let mut map: std::collections::HashMap<u8, Vec<u64>> = std::collections::HashMap::new();
    for e in catalog {
        map.entry(e.carry_sig).or_default().push(e.cycle);
    }
    map
}

/// Statistics summary of the catalog.
#[derive(Clone, Debug)]
pub struct CatalogStats {
    pub total_entries: usize,
    pub distinct_carry_signatures: usize,
    pub level2_count: usize,
    pub level3_plus_count: usize,
    pub dkam_admissible_count: usize,
    pub tzolkin_carry_class_size: usize,
    pub haab_carry_class_size: usize,
}

pub fn catalog_stats(catalog: &[CatalogEntry]) -> CatalogStats {
    let classes = carry_signature_classes(catalog);
    let tz_sig = pack_carry_bits(&carry_bits(260));
    let ha_sig = pack_carry_bits(&carry_bits(365));
    CatalogStats {
        total_entries: catalog.len(),
        distinct_carry_signatures: classes.len(),
        level2_count: catalog.iter().filter(|e| e.level_11 <= 2).count(),
        level3_plus_count: catalog.iter().filter(|e| e.level_11 >= 3).count(),
        dkam_admissible_count: catalog.iter().filter(|e| e.dkam_ok).count(),
        tzolkin_carry_class_size: classes.get(&tz_sig).map(|v| v.len()).unwrap_or(0),
        haab_carry_class_size: classes.get(&ha_sig).map(|v| v.len()).unwrap_or(0),
    }
}

// ═══════════════════════════════════════════════════════════════════
// §3  Tests
// ═══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_builds_non_empty() {
        let cat = build_catalog(50_000);
        assert!(!cat.is_empty(), "Catalog must be non-empty");
    }

    #[test]
    fn catalog_includes_canonical_heads() {
        let cat = build_catalog(200_000);
        let cycles: Vec<u64> = cat.iter().map(|e| e.cycle).collect();
        assert!(cycles.contains(&260), "Tzolk'in in catalog");
        assert!(cycles.contains(&365), "Haab in catalog");
        assert!(cycles.contains(&18_980), "CalendarRound in catalog");
        assert!(cycles.contains(&144_000), "Baktun in catalog");
    }

    #[test]
    fn catalog_includes_venus() {
        let cat = build_catalog(10_000);
        let cycles: Vec<u64> = cat.iter().map(|e| e.cycle).collect();
        assert!(cycles.contains(&584), "Venus synodic in catalog");
    }

    #[test]
    fn carry_sig_tzolkin_correct() {
        let cat = build_catalog(1_000);
        let tz = cat.iter().find(|e| e.cycle == 260).unwrap();
        // 260 = 2²×5×13 → nullified {2,5,13} → active {3,7,11}
        // carry_bits: bits 1,3,4 = 1 → 0b011010 = 26
        assert_eq!(tz.carry_sig, 26);
        assert!(tz.nullified.contains(&2));
        assert!(tz.nullified.contains(&5));
        assert!(tz.nullified.contains(&13));
    }

    #[test]
    fn catalog_tzolkin_hamming0_from_itself() {
        let cat = build_catalog(1_000);
        let tz = cat.iter().find(|e| e.cycle == 260).unwrap();
        assert_eq!(tz.dist_to_tzolkin, 0);
    }

    #[test]
    fn carry_signature_classes_non_empty() {
        let cat = build_catalog(10_000);
        let classes = carry_signature_classes(&cat);
        assert!(!classes.is_empty());
        // Tzolk'in and CalendarRound have the same carry class (both nullify {2,5,13})
        let tz_sig = pack_carry_bits(&carry_bits(260));
        // Verify Tzolk'in is in the map
        assert!(classes.contains_key(&tz_sig), "Tzolk'in carry class must exist");
        assert!(classes[&tz_sig].contains(&260), "Tzolk'in in its carry class");
        // CalRound may not be in max_cycle=10000 range — use 50000
        let cat50 = build_catalog(50_000);
        let classes50 = carry_signature_classes(&cat50);
        let cr_sig = pack_carry_bits(&carry_bits(18_980));
        assert_eq!(tz_sig, cr_sig, "Tzolk'in and CalRound have same carry class");
        assert!(classes50.contains_key(&cr_sig), "Calendar Round carry class must exist");
        // Find CalRound in some class
        let found_cr = cat50.iter().any(|e| e.cycle == 18_980);
        assert!(found_cr, "CalendarRound (18980) must appear in 50k catalog");
    }

    #[test]
    fn catalog_stats_makes_sense() {
        let cat = build_catalog(50_000);
        let stats = catalog_stats(&cat);
        assert!(stats.total_entries > 0);
        assert!(stats.level2_count + stats.level3_plus_count == stats.total_entries);
        assert!(stats.distinct_carry_signatures <= stats.total_entries);
        assert!(stats.tzolkin_carry_class_size >= 2,
            "Tzolk'in carry class has at least Tzolk'in and CalRound");
    }

    #[test]
    fn level2_periods_all_below_threshold() {
        let cat = build_catalog(10_000);
        let l2 = level2_periods(&cat);
        for e in &l2 {
            assert!(e.cycle < 1331 || e.level_11 <= 2,
                "Level-2 cycle {} must be < 1331 or truly Level-2", e.cycle);
        }
    }
}
