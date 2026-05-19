//! # Ramanujan partition congruences — boundary decision procedure (B-6)
//!
//! Mechanizes vault `Maya CRT twp-0001.md` Discoveries 1 and 3:
//!
//! - **Discovery 1**: first-order Ramanujan partition congruences
//!   `p(ℓn + δ) ≡ 0 (mod ℓ)` exist exclusively for `ℓ ∈ {5, 7, 11}`,
//!   with `δ` given by `24δ ≡ 1 (mod ℓ)` → (5, 4), (7, 5), (11, 6).
//!   This is the Ahlgren-Ono theorem (2000); we take it as upstream-proven
//!   and mechanize the decision procedure plus a spot-check verifier.
//!
//! - **Discovery 3**: composite-CRT congruence-lifting decision —
//!   `p(Nn + δ) ≡ 0 (mod N)` for composite N iff every prime factor of N
//!   has a first-order congruence (i.e., is in {5, 7, 11}). So `35 = 5·7`,
//!   `55 = 5·11`, `77 = 7·11`, and `385 = 5·7·11` succeed; `819 = 9·7·13`,
//!   `91 = 7·13`, and `65 = 5·13` fail (13 is not in {5, 7, 11}).
//!
//! - **Discovery 2** (prime gap doubling at the boundary, ~2.20 → 4.40)
//!   is **deferred** to Tier 3 — Measured per vault, not load-bearing for
//!   B-6's coverage/exclusion structure.
//!
//! ## T3 / 819 tension (recorded for downstream)
//!
//! DPM-PRIME T3 frames `819 = 3² · 7 · 13` as the minimum
//! (stability_floor² · last-S_R · boundary) product. T3's structural-product
//! claim stands. But by Discovery 3, **819 has no composite Ramanujan
//! congruence** because the boundary prime 13 has no first-order
//! congruence. The "Ramanujan-carrying" gloss on 819 is therefore weaker
//! than T3's prose may suggest. Test
//! `composite_supports_congruence_819_false` in this module encodes the
//! distinction explicitly; T3's structural-product test continues to pass
//! unchanged in `dccms_atlas::dpm_prime`.

#![allow(dead_code)]

/// The three primes with first-order Ramanujan partition congruences.
///
/// **Status:** Proven (Ahlgren-Ono 2000). Inherited from upstream
/// number theory; not re-proven here.
pub const FIRST_ORDER_PRIMES: [u64; 3] = [5, 7, 11];

/// First-order Ramanujan partition congruence offsets:
/// `(ℓ, δ)` such that `p(ℓn + δ) ≡ 0 (mod ℓ)` for all `n ≥ 0`.
///
/// `δ` is the unique solution to `24δ ≡ 1 (mod ℓ)`.
pub const FIRST_ORDER_DELTAS: [(u64, u64); 3] = [(5, 4), (7, 5), (11, 6)];

/// Decide whether prime `ell` has a first-order Ramanujan partition
/// congruence, and return its offset `δ` if so.
///
/// `Some(δ)` iff `ell ∈ {5, 7, 11}` (Ahlgren-Ono).
pub fn first_order_congruence(ell: u64) -> Option<u64> {
    FIRST_ORDER_DELTAS.iter()
        .find(|&&(p, _)| p == ell)
        .map(|&(_, d)| d)
}

/// Trial-division factorization for small n.
///
/// Returns the distinct prime factors of `n`. Used to decide
/// `composite_supports_congruence` per Discovery 3. For `n` up to ~10⁴
/// this is fast; for larger inputs callers should use a proper sieve.
fn distinct_prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    if n < 2 { return factors; }
    let mut p: u64 = 2;
    while p.saturating_mul(p) <= n {
        if n % p == 0 {
            factors.push(p);
            while n % p == 0 { n /= p; }
        }
        p += 1;
    }
    if n > 1 { factors.push(n); }
    factors
}

/// Decide whether composite `n` supports a Ramanujan-style congruence
/// `p(Nn' + δ) ≡ 0 (mod N)` for some `δ` (Discovery 3).
///
/// Returns `true` iff every distinct prime factor of `n` is in
/// `FIRST_ORDER_PRIMES`. For `n < 2`, returns `false` (no nontrivial
/// composite congruence possible).
pub fn composite_supports_congruence(n: u64) -> bool {
    if n < 2 { return false; }
    let factors = distinct_prime_factors(n);
    factors.iter().all(|p| FIRST_ORDER_PRIMES.contains(p))
}

/// Partition function `p(n)` — number of partitions of `n`.
///
/// Computed via Euler's pentagonal-number recurrence:
///
/// ```text
///   p(n) = Σ_{k=1,2,3,...} (-1)^(k+1) · [p(n - g_k) + p(n - g'_k)]
/// ```
///
/// where `g_k = k(3k-1)/2` and `g'_k = k(3k+1)/2` are the generalized
/// pentagonal numbers, with `p(0) = 1` and `p(m) = 0` for `m < 0`.
///
/// Returns `u128` because `p(n)` grows quickly: `p(100) = 190_569_292`
/// is fine, but values above n ≈ 200 begin exceeding `u128` and must be
/// computed in higher precision (out of scope here). For `n > 200` this
/// function returns `0` to fail safely.
pub fn partition_count(n: u64) -> u128 {
    const MAX_N: u64 = 200;
    if n > MAX_N { return 0; }
    let n = n as usize;
    let mut p = vec![0u128; n + 1];
    p[0] = 1;
    for m in 1..=n {
        let mut sum: i128 = 0;
        // Generalized pentagonal numbers: g_k = k(3k±1)/2 for k = 1, 2, 3, ...
        // (-1)^(k+1) sign alternates: +, +, -, -, +, +, ...
        let mut k: i64 = 1;
        loop {
            let g1 = (k * (3 * k - 1) / 2) as i64;
            let g2 = (k * (3 * k + 1) / 2) as i64;
            if g1 > m as i64 { break; }
            let sign: i128 = if k % 2 == 1 { 1 } else { -1 };
            sum += sign * p[m - g1 as usize] as i128;
            if g2 <= m as i64 {
                sum += sign * p[m - g2 as usize] as i128;
            }
            k += 1;
        }
        // Sum should be non-negative; cast back to u128.
        debug_assert!(sum >= 0, "p({}) recurrence produced negative sum {}", m, sum);
        p[m] = sum as u128;
    }
    p[n]
}

/// Spot-check that `p(ℓn + δ) ≡ 0 (mod ℓ)` holds for `n = 0..=n_max`.
///
/// Returns `false` if any check fails OR if `ell` is not in
/// `FIRST_ORDER_PRIMES` (defensive — there's no `δ` to check). Use
/// `n_max ≤ 15` for reasonable runtime.
pub fn verify_first_order_for_small_n(ell: u64, n_max: u64) -> bool {
    let delta = match first_order_congruence(ell) {
        Some(d) => d,
        None => return false,
    };
    for n in 0..=n_max {
        let arg = ell * n + delta;
        if arg > 200 { return false; }  // out of partition_count's exact range
        let pn = partition_count(arg);
        if pn % (ell as u128) != 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─────────────────────────────────────────────────────────────────
    // Discovery 1 — first-order congruence decision procedure
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn first_order_congruence_5_7_11() {
        assert_eq!(first_order_congruence(5), Some(4));
        assert_eq!(first_order_congruence(7), Some(5));
        assert_eq!(first_order_congruence(11), Some(6));
    }

    #[test]
    fn first_order_congruence_24_delta_equiv_1_mod_ell() {
        // Verify the δ values satisfy 24δ ≡ 1 (mod ℓ) — the canonical
        // formula. This is a sanity check on the FIRST_ORDER_DELTAS table.
        for &(ell, delta) in FIRST_ORDER_DELTAS.iter() {
            assert_eq!((24 * delta) % ell, 1,
                "24·{} mod {} should be 1, got {}", delta, ell, (24 * delta) % ell);
        }
    }

    #[test]
    fn no_first_order_congruence_outside_5_7_11() {
        // Ahlgren-Ono: no first-order congruence for any other prime.
        for ell in [2u64, 3, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73] {
            assert_eq!(first_order_congruence(ell), None,
                "prime {} should have no first-order congruence", ell);
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Partition function p(n)
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn partition_count_small_values() {
        // OEIS A000041: 1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, 77, 101, 135, 176, 231, 297, 385, 490, 627, ...
        let expected: [u128; 21] = [
            1, 1, 2, 3, 5, 7, 11, 15, 22, 30,
            42, 56, 77, 101, 135, 176, 231, 297, 385, 490, 627,
        ];
        for (n, &p_n) in expected.iter().enumerate() {
            assert_eq!(partition_count(n as u64), p_n, "p({}) mismatch", n);
        }
    }

    #[test]
    fn partition_count_at_100() {
        // p(100) = 190_569_292 (well-known value)
        assert_eq!(partition_count(100), 190_569_292);
    }

    #[test]
    fn partition_count_above_max_returns_zero() {
        // Out of exact-u128 range we fail safely.
        assert_eq!(partition_count(201), 0);
        assert_eq!(partition_count(1000), 0);
    }

    // ─────────────────────────────────────────────────────────────────
    // Spot-check the three Ramanujan congruences hold
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn ramanujan_congruence_mod_5_spot_check() {
        // p(5n + 4) ≡ 0 (mod 5) for n = 0..15.
        // E.g., p(4) = 5, p(9) = 30, p(14) = 135, p(19) = 490, ...
        for n in 0u64..=15 {
            let arg = 5 * n + 4;
            let pn = partition_count(arg);
            assert_eq!(pn % 5, 0, "p({}) = {} should be divisible by 5", arg, pn);
        }
        assert!(verify_first_order_for_small_n(5, 15));
    }

    #[test]
    fn ramanujan_congruence_mod_7_spot_check() {
        // p(7n + 5) ≡ 0 (mod 7) for n = 0..15.
        // E.g., p(5) = 7, p(12) = 77, p(19) = 490, ...
        for n in 0u64..=15 {
            let arg = 7 * n + 5;
            let pn = partition_count(arg);
            assert_eq!(pn % 7, 0, "p({}) = {} should be divisible by 7", arg, pn);
        }
        assert!(verify_first_order_for_small_n(7, 15));
    }

    #[test]
    fn ramanujan_congruence_mod_11_spot_check() {
        // p(11n + 6) ≡ 0 (mod 11) for n = 0..10.
        // E.g., p(6) = 11, p(17) = 297, ...
        for n in 0u64..=10 {
            let arg = 11 * n + 6;
            if arg > 200 { break; }  // out of partition_count exact range
            let pn = partition_count(arg);
            assert_eq!(pn % 11, 0, "p({}) = {} should be divisible by 11", arg, pn);
        }
        assert!(verify_first_order_for_small_n(11, 10));
    }

    // ─────────────────────────────────────────────────────────────────
    // Discovery 3 — composite CRT decision procedure
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn composite_supports_congruence_pure_s_r_products() {
        // All combinations of primes within {5, 7, 11} support congruence.
        assert!(composite_supports_congruence(5 * 7));         // 35
        assert!(composite_supports_congruence(5 * 11));        // 55
        assert!(composite_supports_congruence(7 * 11));        // 77
        assert!(composite_supports_congruence(5 * 7 * 11));    // 385
        // Prime powers in {5,7,11} also work (single distinct factor).
        assert!(composite_supports_congruence(25));            // 5²
        assert!(composite_supports_congruence(49));            // 7²
        assert!(composite_supports_congruence(121));           // 11²
        assert!(composite_supports_congruence(5 * 49));        // 5·7²
    }

    #[test]
    fn composite_supports_congruence_819_false() {
        // T3 / 819 tension explicitly encoded.
        //
        // 819 = 3² · 7 · 13. Although 7 ∈ {5,7,11}, the prime factor 13
        // has no first-order congruence. By Discovery 3, the composite
        // 819 inherits no first-order congruence.
        //
        // This does NOT contradict DPM-PRIME T3, which asserts 819 is
        // the minimum (stability_floor² · last-S_R · boundary) STRUCTURAL
        // PRODUCT. T3's structural claim stands; the additional
        // "Ramanujan-carrying" framing is weakened by Discovery 3.
        assert!(!composite_supports_congruence(819),
            "819 should NOT support a Ramanujan congruence — \
             its prime factor 13 has no first-order congruence");
    }

    #[test]
    fn composite_supports_congruence_other_failures() {
        // Any composite containing a prime outside {5, 7, 11} fails.
        assert!(!composite_supports_congruence(7 * 13));       // 91
        assert!(!composite_supports_congruence(5 * 13));       // 65
        assert!(!composite_supports_congruence(2 * 5));        // 10
        assert!(!composite_supports_congruence(3 * 7));        // 21
        assert!(!composite_supports_congruence(11 * 17));      // 187
        // Maya cycles that aren't pure-S_R products:
        assert!(!composite_supports_congruence(260));          // 4·5·13
        assert!(!composite_supports_congruence(365));          // 5·73
        assert!(!composite_supports_congruence(584));          // 8·73
        assert!(!composite_supports_congruence(780));          // 4·3·5·13
    }

    #[test]
    fn composite_supports_congruence_trivial_cases() {
        // 0 and 1 don't carry congruences (no nontrivial composite).
        assert!(!composite_supports_congruence(0));
        assert!(!composite_supports_congruence(1));
    }

    #[test]
    fn composite_supports_individual_first_order_primes() {
        // Each first-order prime trivially "supports" itself.
        for &p in &FIRST_ORDER_PRIMES {
            assert!(composite_supports_congruence(p),
                "single prime {} should support its own congruence", p);
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // verify_first_order_for_small_n exclusion behavior
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn verify_first_order_false_for_non_s_r_primes() {
        // The verifier defensively returns false for primes without
        // first-order congruences.
        for ell in [2u64, 3, 13, 17, 19] {
            assert!(!verify_first_order_for_small_n(ell, 5),
                "verify_first_order should be false for ell = {}", ell);
        }
    }
}
