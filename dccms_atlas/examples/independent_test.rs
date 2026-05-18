//! Independent-event test: build atlases against event corpora that
//! were NOT generated from cycle-completion arithmetic. If the four-head
//! atlas still concentrates at the same Gini signature, the four
//! calendars are projections of one adelic object (Reading A). If the
//! atlas spreads to flat distribution, the prior test was circular
//! (Reading B).
//!
//! ## Independent corpora
//!
//! 1. **Random-day null** — dates sampled uniformly from the operational
//!    range. By construction, has no relationship to Safe Basis cycles.
//! 2. **Prime-day corpus** — dates that are prime integers. By
//!    construction, never divisible by any small prime including the
//!    Safe Basis primes.
//! 3. **Fibonacci corpus** — Fibonacci-number day counts. Famously has
//!    Pisano periods modulo every prime; structure is *different* from
//!    the codex cycles.

use dccms_atlas::events::{EventSet, CodexEvent, CodexEventKind};
use dccms_atlas::heads::FourCalendarHydra;
use dccms_atlas::atlas::ConfigAtlas;

/// Deterministic linear-congruential generator with no float operations.
struct DetRng {
    state: u64,
}

impl DetRng {
    fn new(seed: u64) -> Self { DetRng { state: seed } }
    fn next(&mut self) -> u64 {
        // Numerical Recipes LCG constants
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        self.state
    }
    fn next_in_range(&mut self, max: u64) -> u64 {
        self.next() % max
    }
}

/// Generate N random days uniformly distributed in [0, max_day).
fn random_day_corpus(n: usize, max_day: u64, seed: u64) -> EventSet {
    let mut rng = DetRng::new(seed);
    let mut set = EventSet::new();
    for _ in 0..n {
        let day = rng.next_in_range(max_day);
        set.add(CodexEvent::new(day, CodexEventKind::Custom("random"), ""));
    }
    set
}

/// Generate a corpus of prime-valued day counts up to max_day.
fn prime_day_corpus(max_day: u64) -> EventSet {
    let mut set = EventSet::new();
    let n = max_day as usize;
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    if n >= 1 { is_prime[1] = false; }
    let mut p = 2;
    while p * p <= n {
        if is_prime[p] {
            let mut k = p * p;
            while k <= n {
                is_prime[k] = false;
                k += p;
            }
        }
        p += 1;
    }
    for i in 2..=n {
        if is_prime[i] {
            set.add(CodexEvent::new(i as u64, CodexEventKind::Custom("prime"), ""));
        }
    }
    set
}

/// Generate Fibonacci-day corpus up to max_day.
fn fibonacci_day_corpus(max_day: u64) -> EventSet {
    let mut set = EventSet::new();
    let mut a: u64 = 1;
    let mut b: u64 = 1;
    set.add(CodexEvent::new(a, CodexEventKind::Custom("fib"), ""));
    while b <= max_day {
        set.add(CodexEvent::new(b, CodexEventKind::Custom("fib"), ""));
        let c = a.saturating_add(b);
        a = b;
        b = c;
        if c == u64::MAX { break; }
    }
    set
}

fn run_corpus(label: &str, events: &EventSet, hydra: &FourCalendarHydra) {
    let atlas = ConfigAtlas::build_four_head(events, hydra);
    println!(
        "{:<22} | n={:>6} | occupied={:>6} | Gini={:>5} bp | dark={}",
        label,
        events.len(),
        atlas.occupied_addresses(),
        atlas.gini_basis_points(),
        atlas.dark_strata_count(),
    );
}

fn main() {
    println!("═══════════════════════════════════════════════════════════════");
    println!("  Independent-Event Test");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("Reading A (substrate redundancy): all corpora should show");
    println!("  the same address-space concentration as the canonical corpus.");
    println!();
    println!("Reading B (circularity): the canonical corpus shows 49 addresses;");
    println!("  independent corpora should spread to many more addresses.");
    println!();

    let hydra = FourCalendarHydra::canonical();

    println!("{:<22} | {:>6} | {:>14} | {:>13} | {}",
             "Corpus", "events", "occupied addrs", "Gini (bp)", "dark strata");
    println!("{}", "─".repeat(76));

    // Baseline: canonical corpus
    let canonical = EventSet::canonical_corpus(0, 200_000);
    run_corpus("Canonical (cycles)", &canonical, &hydra);

    // Independent corpora
    let random_small = random_day_corpus(2967, 200_000, 12345);
    run_corpus("Random days (n=2967)", &random_small, &hydra);

    let random_large = random_day_corpus(30_000, 200_000, 67890);
    run_corpus("Random days (n=30K)", &random_large, &hydra);

    let primes = prime_day_corpus(200_000);
    run_corpus("Prime days <200K", &primes, &hydra);

    let fibs = fibonacci_day_corpus(200_000);
    run_corpus("Fibonacci days", &fibs, &hydra);

    println!();
    println!("── Address concentration comparison ──");
    println!("If the address space is naturally finite to ≤ 64 (one head can");
    println!("only produce 2^6 distinct signatures, and the four-head signature");
    println!("collisions reduce that further), then high concentration is");
    println!("structural, not circular.");
    println!();

    // Count distinct addresses per head independently
    println!("Per-head address-space saturation (n=30K random days):");
    let heads = hydra.heads();
    for head in heads {
        let single_atlas = ConfigAtlas::build(&random_large, &[head]);
        println!(
            "  {:<14} → {} distinct 6-bit addresses (of 64 possible)",
            head.signature.name,
            single_atlas.occupied_addresses(),
        );
    }
    println!();

    // Joint distinct addresses
    let four_head_random = ConfigAtlas::build_four_head(&random_large, &hydra);
    println!("Four heads jointly: {} distinct addresses (of 16,777,216 possible)",
             four_head_random.occupied_addresses());
    println!();

    println!("═══════════════════════════════════════════════════════════════");
}
