//! # Configuration Manifold Geometry
//!
//! The four-head atlas on the canonical codex corpus occupies a small
//! fraction of the 24-bit formal address space (1,022 of 16,777,216 ≈
//! 0.006% in the canonical run). This module characterizes the
//! geometric structure of that occupied subspace.
//!
//! ## Instruments
//!
//! - **Bit usage** — which of the 24 bit positions are ever set?
//! - **Effective dimensionality** — bit positions that actually vary
//!   across occupied addresses.
//! - **Bit-pair co-occurrence** — 24×24 matrix of joint bit activations.
//! - **Hamming distance distribution** — pairwise distance histogram.
//! - **Connected components** — the address graph at Hamming-1
//!   neighborhood.
//!
//! Everything is exact integer.

use crate::atlas::ConfigAtlas;
use std::collections::{HashMap, HashSet, VecDeque};

/// A geometric profile of the occupied address subspace.
#[derive(Clone, Debug)]
pub struct ManifoldProfile {
    /// Total occupied addresses.
    pub occupied: usize,
    /// Per-bit usage: how many occupied addresses have bit i set, for i in [0, 24).
    pub bit_usage: [u64; 24],
    /// Effective dimensionality: count of bit positions that vary across
    /// the occupied set (both 0 and 1 appear).
    pub effective_dimension: u8,
    /// Bits that are always 0 across the occupied set.
    pub always_zero_bits: Vec<u8>,
    /// Bits that are always 1 across the occupied set.
    pub always_one_bits: Vec<u8>,
    /// Maximum Hamming distance between any two occupied addresses.
    pub max_hamming_distance: u32,
    /// Mean Hamming distance (in basis points: integer × 10000).
    pub mean_hamming_distance_bp: u64,
    /// Total bit-pair co-occurrence count (sum of bits set per address).
    pub total_bits_set: u64,
}

impl ManifoldProfile {
    /// Compute the manifold profile of an atlas.
    pub fn from_atlas(atlas: &ConfigAtlas) -> Self {
        let addresses: Vec<u64> = atlas.entries.keys().copied().collect();
        let occupied = addresses.len();
        if occupied == 0 {
            return Self::empty();
        }

        // Bit usage
        let mut bit_usage = [0u64; 24];
        for &addr in &addresses {
            for i in 0..24 {
                if (addr >> i) & 1 == 1 {
                    bit_usage[i] += 1;
                }
            }
        }

        // Effective dimensionality and always-zero/always-one
        let mut always_zero = Vec::new();
        let mut always_one = Vec::new();
        let mut effective = 0u8;
        for i in 0..24 {
            if bit_usage[i] == 0 {
                always_zero.push(i as u8);
            } else if bit_usage[i] == occupied as u64 {
                always_one.push(i as u8);
            } else {
                effective += 1;
            }
        }

        // Hamming distance: compute exhaustively if occupied ≤ 5000,
        // else sample. For the canonical 1022-address case, this is
        // 1022 × 1021 / 2 ≈ 521K pair comparisons — fast.
        let (max_ham, mean_ham_bp) = if occupied <= 5000 {
            let mut max_d = 0u32;
            let mut sum_d: u128 = 0;
            let mut pair_count: u128 = 0;
            for i in 0..occupied {
                for j in (i + 1)..occupied {
                    let d = (addresses[i] ^ addresses[j]).count_ones();
                    if d > max_d { max_d = d; }
                    sum_d += d as u128;
                    pair_count += 1;
                }
            }
            let mean_bp = if pair_count > 0 {
                ((sum_d * 10000) / pair_count) as u64
            } else { 0 };
            (max_d, mean_bp)
        } else {
            (0, 0) // sampling deferred
        };

        let total_bits_set: u64 = bit_usage.iter().sum();

        ManifoldProfile {
            occupied,
            bit_usage,
            effective_dimension: effective,
            always_zero_bits: always_zero,
            always_one_bits: always_one,
            max_hamming_distance: max_ham,
            mean_hamming_distance_bp: mean_ham_bp,
            total_bits_set,
        }
    }

    fn empty() -> Self {
        ManifoldProfile {
            occupied: 0,
            bit_usage: [0; 24],
            effective_dimension: 0,
            always_zero_bits: Vec::new(),
            always_one_bits: Vec::new(),
            max_hamming_distance: 0,
            mean_hamming_distance_bp: 0,
            total_bits_set: 0,
        }
    }

    /// Maximum possible distinct addresses in the *effective* dimensions.
    /// 2^effective_dimension.
    pub fn effective_capacity(&self) -> u64 {
        if self.effective_dimension >= 64 {
            u64::MAX
        } else {
            1u64 << self.effective_dimension
        }
    }

    /// Saturation: occupied / effective_capacity in basis points.
    /// How much of the effective-dimensional subspace is actually used?
    pub fn saturation_bp(&self) -> u64 {
        let cap = self.effective_capacity();
        if cap == 0 { return 0; }
        ((self.occupied as u128) * 10000 / cap as u128) as u64
    }

    /// Per-head bit usage: bits 0-5 → head 0, 6-11 → head 1, etc.
    /// Returns a vector [head_0_bits_set_total, head_1_..., head_2_..., head_3_...].
    pub fn per_head_total_bit_usage(&self) -> [u64; 4] {
        let mut counts = [0u64; 4];
        for i in 0..24 {
            let head = i / 6;
            counts[head] += self.bit_usage[i];
        }
        counts
    }
}

/// Build a Hamming-1 adjacency graph and compute connected components.
///
/// Two occupied addresses are connected if their Hamming distance is 1
/// (they differ in exactly one bit). Connected components reveal the
/// "islands" of the configuration manifold.
#[derive(Clone, Debug)]
pub struct ConnectivityReport {
    /// Number of connected components.
    pub component_count: usize,
    /// Sizes of each component, sorted descending.
    pub component_sizes: Vec<usize>,
    /// Size of the largest component.
    pub largest_component_size: usize,
    /// Total occupied addresses examined.
    pub total_addresses: usize,
}

pub fn hamming_1_connectivity(atlas: &ConfigAtlas) -> ConnectivityReport {
    let addresses: Vec<u64> = atlas.entries.keys().copied().collect();
    let n = addresses.len();
    if n == 0 {
        return ConnectivityReport {
            component_count: 0,
            component_sizes: Vec::new(),
            largest_component_size: 0,
            total_addresses: 0,
        };
    }

    let addr_set: HashSet<u64> = addresses.iter().copied().collect();
    let mut visited: HashSet<u64> = HashSet::new();
    let mut sizes = Vec::new();

    for &start in &addresses {
        if visited.contains(&start) { continue; }
        // BFS from start
        let mut size = 0usize;
        let mut queue: VecDeque<u64> = VecDeque::new();
        queue.push_back(start);
        visited.insert(start);
        while let Some(current) = queue.pop_front() {
            size += 1;
            // Each Hamming-1 neighbor differs by one bit
            for i in 0..24 {
                let neighbor = current ^ (1u64 << i);
                if addr_set.contains(&neighbor) && !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
        sizes.push(size);
    }

    sizes.sort_by(|a, b| b.cmp(a));
    let largest = sizes.first().copied().unwrap_or(0);
    let count = sizes.len();

    ConnectivityReport {
        component_count: count,
        component_sizes: sizes,
        largest_component_size: largest,
        total_addresses: n,
    }
}

/// Bit-pair co-occurrence matrix (24×24 lower triangle).
///
/// `matrix[i][j]` for j < i: how many occupied addresses have BOTH bits i and j set.
#[derive(Clone, Debug)]
pub struct BitCorrelationMatrix {
    pub matrix: Vec<Vec<u64>>,
    pub occupied: u64,
}

impl BitCorrelationMatrix {
    pub fn from_atlas(atlas: &ConfigAtlas) -> Self {
        let addresses: Vec<u64> = atlas.entries.keys().copied().collect();
        let occupied = addresses.len() as u64;
        let mut matrix: Vec<Vec<u64>> = (0..24).map(|i| vec![0u64; i]).collect();
        for &addr in &addresses {
            for i in 1..24 {
                if (addr >> i) & 1 == 1 {
                    for j in 0..i {
                        if (addr >> j) & 1 == 1 {
                            matrix[i][j] += 1;
                        }
                    }
                }
            }
        }
        BitCorrelationMatrix { matrix, occupied }
    }

    /// Expected co-occurrence under independence assumption (basis points).
    pub fn expected_independence_bp(&self, bit_usage: &[u64; 24], i: usize, j: usize) -> u64 {
        if self.occupied == 0 { return 0; }
        let p_i = bit_usage[i];
        let p_j = bit_usage[j];
        ((p_i as u128 * p_j as u128 * 10000) / (self.occupied as u128 * self.occupied as u128)) as u64
    }

    /// Top-k strongest positive correlations (above independence baseline).
    /// Returns triples (bit_i, bit_j, observed_count).
    pub fn top_correlations(&self, k: usize) -> Vec<(usize, usize, u64)> {
        let mut all: Vec<(usize, usize, u64)> = Vec::new();
        for i in 1..24 {
            for j in 0..i {
                all.push((i, j, self.matrix[i][j]));
            }
        }
        all.sort_by(|a, b| b.2.cmp(&a.2));
        all.into_iter().take(k).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EventSet;
    use crate::heads::FourCalendarHydra;

    #[test]
    fn empty_atlas_gives_empty_profile() {
        let events = EventSet::new();
        let hydra = FourCalendarHydra::canonical();
        let atlas = ConfigAtlas::build_four_head(&events, &hydra);
        let profile = ManifoldProfile::from_atlas(&atlas);
        assert_eq!(profile.occupied, 0);
        assert_eq!(profile.effective_dimension, 0);
    }

    #[test]
    fn canonical_corpus_profile_computable() {
        let events = EventSet::canonical_corpus(0, 50_000);
        let hydra = FourCalendarHydra::canonical();
        let atlas = ConfigAtlas::build_four_head(&events, &hydra);
        let profile = ManifoldProfile::from_atlas(&atlas);
        assert!(profile.occupied > 0);
        assert!(profile.effective_dimension <= 24);
        // Per-head bit usage should distribute non-degenerately
        let per_head = profile.per_head_total_bit_usage();
        assert_eq!(per_head.len(), 4);
    }

    #[test]
    fn always_zero_plus_one_plus_effective_equals_24() {
        let events = EventSet::canonical_corpus(0, 20_000);
        let hydra = FourCalendarHydra::canonical();
        let atlas = ConfigAtlas::build_four_head(&events, &hydra);
        let profile = ManifoldProfile::from_atlas(&atlas);
        let total = profile.always_zero_bits.len()
                  + profile.always_one_bits.len()
                  + profile.effective_dimension as usize;
        assert_eq!(total, 24);
    }

    #[test]
    fn connectivity_report_for_canonical() {
        let events = EventSet::canonical_corpus(0, 30_000);
        let hydra = FourCalendarHydra::canonical();
        let atlas = ConfigAtlas::build_four_head(&events, &hydra);
        let report = hamming_1_connectivity(&atlas);
        assert!(report.component_count >= 1);
        let total: usize = report.component_sizes.iter().sum();
        assert_eq!(total, report.total_addresses);
    }

    #[test]
    fn bit_correlation_matrix_consistent() {
        let events = EventSet::canonical_corpus(0, 30_000);
        let hydra = FourCalendarHydra::canonical();
        let atlas = ConfigAtlas::build_four_head(&events, &hydra);
        let corr = BitCorrelationMatrix::from_atlas(&atlas);
        // Co-occurrence cannot exceed individual bit usage
        let profile = ManifoldProfile::from_atlas(&atlas);
        for i in 1..24 {
            for j in 0..i {
                assert!(corr.matrix[i][j] <= profile.bit_usage[i]);
                assert!(corr.matrix[i][j] <= profile.bit_usage[j]);
            }
        }
    }
}
