//! # prime_hunt — Exact-integer sieve and prime utilities

#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic)]

pub mod ramanujan_partition;

/// Sieve of Eratosthenes up to limit. Returns sorted list of primes.
pub fn sieve(limit: usize) -> Vec<u64> {
    if limit < 2 { return Vec::new(); }
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut p = 2;
    while p * p <= limit {
        if is_prime[p] {
            let mut k = p * p;
            while k <= limit { is_prime[k] = false; k += p; }
        }
        p += 1;
    }
    (2..=limit).filter(|&i| is_prime[i]).map(|i| i as u64).collect()
}

/// Distinct prime factors of n, sorted ascending.
pub fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            factors.push(d);
            while n % d == 0 { n /= d; }
        }
        d += 1;
    }
    if n > 1 { factors.push(n); }
    factors
}

/// Returns true iff n is prime.
pub fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut d = 3u64;
    while d * d <= n { if n % d == 0 { return false; } d += 2; }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sieve_first_ten() {
        let p = sieve(30);
        assert_eq!(p, vec![2,3,5,7,11,13,17,19,23,29]);
    }
    #[test]
    fn prime_factors_260() {
        assert_eq!(prime_factors(260), vec![2,5,13]);
    }
}
