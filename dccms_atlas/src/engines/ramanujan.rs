//! # Ramanujan sums — exact integer implementation (NODE-B7-04)
//!
//! Computes `c_q(n) = Σ_{a coprime to q, 1≤a≤q} e^{2πi a n / q}` as an
//! exact integer via the Möbius/Euler-totient formula
//!
//! ```text
//!   c_q(n) = μ(q/d) · φ(q) / φ(q/d),   d = gcd(q, n)
//! ```
//!
//! All arithmetic stays in `i64`. No roots of unity, no floats. Source:
//! vault `Mayas Engine.md` declares the function signature; the formula
//! is standard (Hardy & Wright, *An Introduction to the Theory of Numbers*,
//! §16.6).

#![allow(dead_code)]

/// Greatest common divisor (binary GCD adequate for u64).
const fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Euler totient φ(n): count of k in [1, n] with gcd(k, n) = 1.
///
/// Computed via prime factorization: φ(p_1^a · ... · p_k^a) =
/// n · ∏ (1 - 1/p_i). For n ≤ ~10⁶ the trial-division is fast and
/// stays in integer arithmetic.
pub fn euler_totient(mut n: u64) -> u64 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    let mut result = n;
    let mut p: u64 = 2;
    while p.saturating_mul(p) <= n {
        if n % p == 0 {
            while n % p == 0 { n /= p; }
            result -= result / p;
        }
        p += 1;
    }
    if n > 1 { result -= result / n; }
    result
}

/// Möbius function μ(n).
///
/// - μ(1) = 1
/// - μ(n) = 0 if n has a squared prime factor
/// - μ(p_1 · ... · p_k) = (-1)^k for distinct primes p_i
pub fn mobius(mut n: u64) -> i64 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    let mut prime_count: u32 = 0;
    let mut p: u64 = 2;
    while p.saturating_mul(p) <= n {
        if n % p == 0 {
            n /= p;
            if n % p == 0 { return 0; }
            prime_count += 1;
        }
        p += 1;
    }
    if n > 1 { prime_count += 1; }
    if prime_count % 2 == 0 { 1 } else { -1 }
}

/// Ramanujan sum c_q(n).
///
/// Returns `0` for `q == 0` (degenerate).
pub fn ramanujan_sum(q: u64, n: u64) -> i64 {
    if q == 0 { return 0; }
    let d = gcd(q, n);
    let m = q / d;          // = q / gcd(q, n)
    let mu_m = mobius(m);
    if mu_m == 0 { return 0; }
    let phi_q = euler_totient(q) as i64;
    let phi_m = euler_totient(m) as i64;
    // φ(m) | φ(q) for m | q (standard identity), so this is exact integer.
    mu_m * (phi_q / phi_m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euler_totient_small() {
        assert_eq!(euler_totient(1), 1);
        assert_eq!(euler_totient(2), 1);
        assert_eq!(euler_totient(3), 2);
        assert_eq!(euler_totient(4), 2);
        assert_eq!(euler_totient(5), 4);
        assert_eq!(euler_totient(6), 2);
        assert_eq!(euler_totient(7), 6);
        assert_eq!(euler_totient(8), 4);
        assert_eq!(euler_totient(9), 6);
        assert_eq!(euler_totient(10), 4);
        assert_eq!(euler_totient(11), 10);
        assert_eq!(euler_totient(12), 4);
        assert_eq!(euler_totient(13), 12);
        assert_eq!(euler_totient(20), 8);
        assert_eq!(euler_totient(260), 96);   // 4·2·12 = 96 (260 = 4·5·13; φ(4)=2, φ(5)=4, φ(13)=12)
    }

    #[test]
    fn mobius_small() {
        assert_eq!(mobius(1), 1);
        assert_eq!(mobius(2), -1);
        assert_eq!(mobius(3), -1);
        assert_eq!(mobius(4), 0);    // 2²
        assert_eq!(mobius(5), -1);
        assert_eq!(mobius(6), 1);    // 2·3
        assert_eq!(mobius(7), -1);
        assert_eq!(mobius(8), 0);    // 2³
        assert_eq!(mobius(9), 0);    // 3²
        assert_eq!(mobius(10), 1);   // 2·5
        assert_eq!(mobius(11), -1);
        assert_eq!(mobius(12), 0);   // 2²·3
        assert_eq!(mobius(13), -1);
        assert_eq!(mobius(30), -1);  // 2·3·5 (three primes)
    }

    #[test]
    fn ramanujan_identity_c_one_n_equals_one() {
        // c_1(n) = 1 for all n.
        for n in 0u64..20 {
            assert_eq!(ramanujan_sum(1, n), 1, "c_1({}) should equal 1", n);
        }
    }

    #[test]
    fn ramanujan_identity_c_q_zero_equals_totient() {
        // c_q(0) = φ(q) for all q ≥ 1 (gcd(q, 0) = q, so m = 1; μ(1) = 1; result = φ(q)).
        for q in 1u64..20 {
            let expected = euler_totient(q) as i64;
            assert_eq!(ramanujan_sum(q, 0), expected,
                "c_{}(0) should equal φ({}) = {}", q, q, expected);
        }
    }

    #[test]
    fn ramanujan_prime_split() {
        // For prime p, the Möbius/totient formula gives:
        //   p ∤ n  ⇒  c_p(n) = μ(p) · φ(p)/φ(p) = -1
        //   p | n  ⇒  c_p(n) = μ(1) · φ(p)/φ(1) = p - 1
        //
        // In particular: c_p(0) = p-1 (p | 0 by convention),
        // c_p(1) = -1 (p ∤ 1 for p > 1).
        //
        // Earlier draft of this test had the case-split inverted in three
        // of four assertions; verified against the formula and against the
        // other identity tests (c_q(0) = φ(q), c_1(n) = 1, c_4 with square
        // divisor) which constrain the same arithmetic.
        for p in [2u64, 3, 5, 7, 11, 13, 17] {
            // p ∤ 1 (for p > 1): c_p(1) = -1.
            assert_eq!(ramanujan_sum(p, 1), -1,
                "c_{}(1) = -1 because {} ∤ 1", p, p);
            // p | 0: c_p(0) = p - 1.
            assert_eq!(ramanujan_sum(p, 0), (p - 1) as i64,
                "c_{}(0) = {} because {} | 0", p, p - 1, p);
            // p | p: c_p(p) = p - 1.
            assert_eq!(ramanujan_sum(p, p), (p - 1) as i64,
                "c_{}({}) = {} because {} | {}", p, p, p - 1, p, p);
            // p | 2p: c_p(2p) = p - 1.
            assert_eq!(ramanujan_sum(p, 2 * p), (p - 1) as i64,
                "c_{}({}) = {} because {} | {}", p, 2 * p, p - 1, p, 2 * p);
        }
    }

    #[test]
    fn ramanujan_zero_q_yields_zero() {
        for n in 0u64..5 {
            assert_eq!(ramanujan_sum(0, n), 0);
        }
    }

    #[test]
    fn ramanujan_with_square_divisor() {
        // c_4(1): d = gcd(4, 1) = 1, m = 4, μ(4) = 0 ⇒ c_4(1) = 0.
        assert_eq!(ramanujan_sum(4, 1), 0);
        // c_4(2): d = gcd(4, 2) = 2, m = 2, μ(2) = -1, φ(4) = 2, φ(2) = 1
        //   ⇒ c_4(2) = -1 · (2 / 1) = -2.
        assert_eq!(ramanujan_sum(4, 2), -2);
        // c_4(4): d = 4, m = 1, μ(1) = 1, φ(4) = 2, φ(1) = 1
        //   ⇒ c_4(4) = 1 · 2 = 2.
        assert_eq!(ramanujan_sum(4, 4), 2);
    }
}
