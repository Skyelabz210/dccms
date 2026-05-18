//! # DPM-PRIME — Mechanized Arithmetic Certificate Suite
//!
//! Direct-integer-computation checks of the 10-theorem stack in vault
//! `The Dresden Codex.md` §AXIOMS, §DEFINITIONS, §LEMMAS, §THEOREMS,
//! §VALIDATION IDENTITIES.
//!
//! Each theorem returns a [`TheoremResult`] with an explicit witness. The
//! arithmetic claims are `native_decide`-equivalent in Lean 4 per the
//! vault's verification status; this module renders them
//! `cargo test`-decidable in Rust. **This is an arithmetic certificate
//! suite, not a Lean/Coq formal proof artifact** — the latter lives
//! upstream (vault references `TUDPBoundary.lean` and the FSM-PRIME
//! infrastructure). The Rust tests are executable evidence that the
//! integer arithmetic underlying each theorem holds; they do not
//! replace the formal proofs and do not constitute one.
//!
//! **Source:** `~/Agents/imports/github/HackFate/The Dresden Codex.md`
//! lines 35-423 (DPM-PRIME stack).
//!
//! **Status:**
//! - 9 of 10 theorems certified by integer arithmetic in this module.
//! - 1 of 10 theorems (T4: Long Count = covering space of Calendar Round)
//!   returns [`TheoremResult::Conditional`] because the explicit Long
//!   Count → M_Fib covering-morphism is structural per the vault and
//!   depends on infrastructure (`M_Fib`) that lives outside dccms.
//! - 14 of 14 validation identities (V1–V14) certified verbatim from
//!   the vault.
//! - 1 additional companion identity (V14_strict) added during
//!   v0.8.0 precision hardening to align with Theorem T2's exact
//!   predicate (V14 as published is strictly weaker than T2).

#![allow(dead_code)]

use dresden_codex::{
    BAKTUN, CALENDAR_PRIME, CALENDAR_ROUND, COUNT_819, ECLIPSE_CORRECTION,
    ECLIPSE_TABLE_DAYS, EPOCH_33_YEAR, JUPITER_SYNODIC, LONG_COUNT_13_BAKTUN,
    MARS_SYNODIC, M_SAFE, RAMANUJAN_S_R, SATURN_SYNODIC, VENUS_HAAB_LCM,
    VENUS_SYNODIC,
};

// ═══════════════════════════════════════════════════════════════════
// Theorem result type
// ═══════════════════════════════════════════════════════════════════

/// The outcome of a DPM-PRIME theorem evaluation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TheoremResult {
    /// All claims hold by direct integer arithmetic.
    Pass {
        /// One-line summary of the verified content.
        witness: String,
    },
    /// The arithmetic claims pass but the theorem also depends on
    /// structural/categorical infrastructure outside this codebase
    /// (e.g., Lean 4 morphism construction).
    Conditional {
        /// What the arithmetic verifies.
        verified: String,
        /// What remains pending elsewhere.
        pending: String,
    },
    /// A claim does not hold by direct arithmetic.
    Fail {
        /// The failing assertion.
        reason: String,
    },
}

impl TheoremResult {
    /// Helper for the success path with a string witness.
    pub fn pass(witness: impl Into<String>) -> Self {
        Self::Pass { witness: witness.into() }
    }

    /// Whether this result is a clean Pass.
    pub fn is_pass(&self) -> bool {
        matches!(self, Self::Pass { .. })
    }

    /// Whether this result is conditionally accepted.
    pub fn is_conditional(&self) -> bool {
        matches!(self, Self::Conditional { .. })
    }
}

// ═══════════════════════════════════════════════════════════════════
// Helper arithmetic (gcd, lcm, factorization)
// ═══════════════════════════════════════════════════════════════════

const fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

const fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

// ═══════════════════════════════════════════════════════════════════
// §LEMMAS — L1 through L10
// ═══════════════════════════════════════════════════════════════════

pub mod lemmas {
    use super::*;

    /// **L1** — Calendar Round factorization: `lcm(260, 365) = 18,980 = 2² · 5 · 13 · 73`.
    pub fn l1_calendar_round() -> TheoremResult {
        let cr = lcm(260, 365);
        if cr != 18_980 { return TheoremResult::Fail { reason: format!("lcm(260,365) = {}", cr) }; }
        if cr != 4 * 5 * 13 * 73 { return TheoremResult::Fail { reason: "factorization".into() }; }
        if gcd(260, 365) != 5 { return TheoremResult::Fail { reason: "gcd".into() }; }
        TheoremResult::pass("18,980 = 2²·5·13·73; gcd(260,365)=5")
    }

    /// **L2** — Tzolk'in factorization: `260 = 2² · 5 · 13`.
    pub fn l2_tzolkin() -> TheoremResult {
        if 260 != 4 * 5 * 13 { return TheoremResult::Fail { reason: "260 != 4·5·13".into() }; }
        TheoremResult::pass("260 = 4·5·13")
    }

    /// **L3** — 819-day three-tier product: `819 = 3² · 7 · 13`.
    pub fn l3_three_tier_819() -> TheoremResult {
        if COUNT_819 != 9 * 7 * 13 { return TheoremResult::Fail { reason: "819".into() }; }
        if COUNT_819 != 3 * 3 * 7 * 13 { return TheoremResult::Fail { reason: "tier".into() }; }
        TheoremResult::pass("819 = 3²·7·13 (stability_floor² · last_S_R · boundary)")
    }

    /// **L4** — Eclipse table & Tzolk'in commensurability: `11,960 = 46 · 260`.
    pub fn l4_eclipse_tzolkin() -> TheoremResult {
        if ECLIPSE_TABLE_DAYS % 260 != 0 { return TheoremResult::Fail { reason: "not divisible".into() }; }
        if ECLIPSE_TABLE_DAYS != 46 * 260 { return TheoremResult::Fail { reason: "not 46·260".into() }; }
        TheoremResult::pass("11,960 = 46·260 (Tzolk'in-aligned)")
    }

    /// **L5** — 33-year correction: `T_C = 93 = 3·31`, `gcd(11,960, 93) = 1`.
    pub fn l5_correction() -> TheoremResult {
        let t_c = EPOCH_33_YEAR - ECLIPSE_TABLE_DAYS;
        if t_c != ECLIPSE_CORRECTION { return TheoremResult::Fail { reason: "T_C".into() }; }
        if t_c != 3 * 31 { return TheoremResult::Fail { reason: "93=3·31".into() }; }
        if gcd(ECLIPSE_TABLE_DAYS, t_c) != 1 {
            return TheoremResult::Fail { reason: "not coprime".into() };
        }
        TheoremResult::pass("93 = 3·31, gcd(11960,93) = 1 (coprime two-phase lift)")
    }

    /// **L6** — Venus-Sun shared factor: `gcd(584, 365) = 73`, `lcm = 2920`.
    pub fn l6_venus_sun() -> TheoremResult {
        if gcd(VENUS_SYNODIC, 365) != CALENDAR_PRIME {
            return TheoremResult::Fail { reason: "gcd != 73".into() };
        }
        if lcm(VENUS_SYNODIC, 365) != VENUS_HAAB_LCM {
            return TheoremResult::Fail { reason: "lcm != 2920".into() };
        }
        if VENUS_HAAB_LCM != 8 * 5 * 73 {
            return TheoremResult::Fail { reason: "factorization".into() };
        }
        TheoremResult::pass("gcd(584,365)=73; lcm=2920=8·5·73")
    }

    /// **L7** — No standard synodic period divisible by 11.
    pub fn l7_no_synodic_carries_11() -> TheoremResult {
        for &p in &[VENUS_SYNODIC, MARS_SYNODIC, JUPITER_SYNODIC, SATURN_SYNODIC] {
            if p % 11 == 0 {
                return TheoremResult::Fail { reason: format!("11 | {}", p) };
            }
        }
        TheoremResult::pass("11 ∤ {584, 780, 399, 378}")
    }

    /// **L8** — Saturn displacement carries 11²: `11,960 mod 378 = 242 = 2·11²`.
    pub fn l8_saturn_eleven_squared() -> TheoremResult {
        let delta = ECLIPSE_TABLE_DAYS % SATURN_SYNODIC;
        if delta != 242 { return TheoremResult::Fail { reason: format!("Δ_S = {}", delta) }; }
        if delta != 2 * 121 { return TheoremResult::Fail { reason: "242 != 2·121".into() }; }
        if delta % (11 * 11) != 0 { return TheoremResult::Fail { reason: "11² ∤ 242".into() }; }
        TheoremResult::pass("Δ_S = 242 = 2·11²")
    }

    /// **L9** — Mars displacement is Tzolk'in: `11,960 mod 780 = 260`.
    pub fn l9_mars_tzolkin() -> TheoremResult {
        let delta = ECLIPSE_TABLE_DAYS % MARS_SYNODIC;
        if delta != 260 { return TheoremResult::Fail { reason: format!("Δ_Ma = {}", delta) }; }
        TheoremResult::pass("Δ_Ma = 260 = T_tz")
    }

    /// **L10** — Venus displacement carries {5, 7}: `11,960 mod 584 = 280 = 2³·5·7`.
    pub fn l10_venus_five_seven() -> TheoremResult {
        let delta = ECLIPSE_TABLE_DAYS % VENUS_SYNODIC;
        if delta != 280 { return TheoremResult::Fail { reason: format!("Δ_V = {}", delta) }; }
        if delta != 8 * 5 * 7 { return TheoremResult::Fail { reason: "280 != 8·5·7".into() }; }
        TheoremResult::pass("Δ_V = 280 = 2³·5·7 (carries 5 and 7)")
    }
}

// ═══════════════════════════════════════════════════════════════════
// §THEOREMS — T1 through T10
// ═══════════════════════════════════════════════════════════════════

pub mod theorems {
    use super::*;

    /// **T1** — Calendar Round as CRT product torus over `{4, 5, 13, 73}`.
    pub fn t1_calendar_round_as_crt_torus() -> TheoremResult {
        // L1 establishes the factorization.
        let l1 = lemmas::l1_calendar_round();
        if !l1.is_pass() { return l1; }
        // The four prime-powers are pairwise coprime — required for CRT.
        let components = [4u64, 5, 13, 73];
        for i in 0..components.len() {
            for j in (i + 1)..components.len() {
                if gcd(components[i], components[j]) != 1 {
                    return TheoremResult::Fail {
                        reason: format!("gcd({}, {}) != 1", components[i], components[j])
                    };
                }
            }
        }
        // Conductor: product of the four components.
        let conductor: u64 = components.iter().product();
        if conductor != CALENDAR_ROUND {
            return TheoremResult::Fail { reason: "conductor != 18,980".into() };
        }
        TheoremResult::pass("CR = 18,980 = product of pairwise-coprime {4, 5, 13, 73}")
    }

    /// **T2** — `T_tz = 260` is the minimum CRT-calendar-complete integer.
    pub fn t2_tzolkin_minimum() -> TheoremResult {
        // Conditions: (i) contains S_R prime, (ii) contains 13,
        // (iii) lcm(T, 365) has 4-prime CRT factorization {4, 5, 13, 73}.
        // Search multiples of lcm(5,13) = 65 below 365.
        let mut candidates = Vec::new();
        for t in (65u64..365).step_by(65) {
            // Check whether lcm(t, 365) factors as 4 · 5 · 13 · 73 = 18,980.
            if lcm(t, 365) == 18_980 {
                candidates.push(t);
            }
        }
        if candidates.is_empty() {
            return TheoremResult::Fail { reason: "no T found".into() };
        }
        let minimum = *candidates.iter().min().unwrap();
        if minimum != 260 {
            return TheoremResult::Fail {
                reason: format!("minimum is {} not 260", minimum)
            };
        }
        TheoremResult::pass("260 is the unique minimum multiple of 65 < 365 with lcm·365 = 18,980")
    }

    /// **T3** — 819 = three-tier structure product `3² · 7 · 13`.
    ///
    /// Minimality is established by **exhaustive bounded exclusion**: no
    /// integer `n` with `1 ≤ n < 819` simultaneously satisfies `9 ∣ n`,
    /// `7 ∣ n`, and `13 ∣ n`. This is equivalent to `lcm(9, 7, 13) = 819`
    /// since 9, 7, 13 are pairwise coprime.
    pub fn t3_819_three_tier() -> TheoremResult {
        let l3 = lemmas::l3_three_tier_819();
        if !l3.is_pass() { return l3; }
        // Exhaustive bounded exclusion over 1..819.
        for n in 1u64..COUNT_819 {
            if n % 9 == 0 && n % 7 == 0 && n % 13 == 0 {
                return TheoremResult::Fail {
                    reason: format!(
                        "n = {} < 819 satisfies 9|n ∧ 7|n ∧ 13|n — minimality violated",
                        n
                    )
                };
            }
        }
        // And 819 itself satisfies all three conditions.
        if !(COUNT_819 % 9 == 0 && COUNT_819 % 7 == 0 && COUNT_819 % 13 == 0) {
            return TheoremResult::Fail {
                reason: "819 should satisfy 9|n ∧ 7|n ∧ 13|n but does not".into()
            };
        }
        // And the lcm computation agrees: lcm(9,7,13) = 819.
        let lcm_9_7_13 = lcm(lcm(9, 7), 13);
        if lcm_9_7_13 != COUNT_819 {
            return TheoremResult::Fail {
                reason: format!("lcm(9,7,13) = {} != 819", lcm_9_7_13)
            };
        }
        TheoremResult::pass(
            "819 = lcm(9, 7, 13) = minimum n with 9|n ∧ 7|n ∧ 13|n \
             (exhaustively excluded over 1..819)"
        )
    }

    /// **T4** — Long Count as covering space of Calendar Round (Conditional).
    ///
    /// The arithmetic component is proven exactly without decimals:
    ///
    /// ```text
    ///   1,872,000 / 18,980  =  7200 / 73  =  98 + 46/73
    /// ```
    ///
    /// after dividing numerator and denominator by `gcd = 260`. Non-integrality
    /// is proven by `73 ∤ 7200` (since `7200 mod 73 = 46`).
    ///
    /// The covering-space morphism construction is structural per vault and
    /// depends on M_Fib infrastructure (FSM-PRIME) outside dccms.
    pub fn t4_long_count_covers_calendar_round() -> TheoremResult {
        // (a) Long Count = 13 · Baktun.
        if LONG_COUNT_13_BAKTUN != 13 * BAKTUN {
            return TheoremResult::Fail { reason: "Long Count != 13·Baktun".into() };
        }
        // (b) Reduce the ratio exactly: 1,872,000 / 18,980.
        let num: u64 = LONG_COUNT_13_BAKTUN;   // 1,872,000
        let den: u64 = CALENDAR_ROUND;         // 18,980
        let g = gcd(num, den);
        let reduced_num = num / g;             // 7,200
        let reduced_den = den / g;             // 73
        // Sanity: the reduction should yield 7200/73.
        if reduced_num != 7_200 || reduced_den != 73 {
            return TheoremResult::Fail {
                reason: format!("expected 7200/73 after gcd reduction, got {}/{}",
                    reduced_num, reduced_den)
            };
        }
        // (c) Non-integrality: 73 ∤ 7,200.
        let remainder = reduced_num % reduced_den;
        if remainder == 0 {
            return TheoremResult::Fail {
                reason: format!("expected 73 ∤ 7200, but 7200 mod 73 = 0")
            };
        }
        if remainder != 46 {
            return TheoremResult::Fail {
                reason: format!("expected 7200 mod 73 = 46, got {}", remainder)
            };
        }
        let quotient = reduced_num / reduced_den;  // 98
        if quotient != 98 {
            return TheoremResult::Fail {
                reason: format!("expected ⌊7200/73⌋ = 98, got {}", quotient)
            };
        }
        // (d) Equivalent expression: 1,872,000 = (Long Count) and
        //     CR · 98 + (gcd · 46) = 18,980 · 98 + 260 · 46 = LongCount.
        let reconstructed = den * quotient + g * remainder;
        if reconstructed != num {
            return TheoremResult::Fail {
                reason: format!("CRT reconstruction failed: {} != {}", reconstructed, num)
            };
        }
        TheoremResult::Conditional {
            verified: "1,872,000 = 13·Baktun; 1,872,000/18,980 = 7200/73 = 98 + 46/73; \
                       73 ∤ 7200 (7200 mod 73 = 46), so the ratio is non-integral and \
                       Long Count is a fiber bundle over CR, not a simple multiple"
                .into(),
            pending: "covering-space morphism Long Count → M_Fib requires FSM-PRIME \
                      infrastructure (Lean 4 / Coq) outside dccms"
                .into(),
        }
    }

    /// **T5** — Eclipse table as two-phase K-Elimination lift.
    pub fn t5_eclipse_two_phase_lift() -> TheoremResult {
        let l5 = lemmas::l5_correction();
        if !l5.is_pass() { return l5; }
        // Phase 1 closes the 5-channel: 5 | 11,960.
        if ECLIPSE_TABLE_DAYS % 5 != 0 {
            return TheoremResult::Fail { reason: "5 ∤ 11,960".into() };
        }
        // Phase 2 closes the 3-channel: 3 | 93.
        if ECLIPSE_CORRECTION % 3 != 0 {
            return TheoremResult::Fail { reason: "3 ∤ 93".into() };
        }
        // Combined coverage: lcm(11960, 93) spans the full ~3000-year range.
        let combined = lcm(ECLIPSE_TABLE_DAYS, ECLIPSE_CORRECTION);
        if combined != ECLIPSE_TABLE_DAYS * ECLIPSE_CORRECTION {
            return TheoremResult::Fail { reason: "lcm != product (so not coprime?)".into() };
        }
        TheoremResult::pass(format!(
            "Phase1 closes 5-channel (5|11960); Phase2 closes 3-channel (3|93); combined lcm = {}",
            combined
        ))
    }

    /// **T6** — Venus-Sun convergence via shared-73-factor.
    pub fn t6_venus_sun_73() -> TheoremResult {
        let l6 = lemmas::l6_venus_sun();
        if !l6.is_pass() { return l6; }
        // 73 > 11 → Tier 1 (turbulent) per T-UDP-BOUNDARY.
        if CALENDAR_PRIME <= 11 {
            return TheoremResult::Fail { reason: "73 should be > 11".into() };
        }
        // 73 ∉ S_R.
        for &p in &RAMANUJAN_S_R {
            if p == CALENDAR_PRIME {
                return TheoremResult::Fail { reason: "73 should not be in S_R".into() };
            }
        }
        TheoremResult::pass("Venus-Sun convergence: gcd=73, lcm=2920, regime=Tier-1 turbulent")
    }

    /// **T7** — Astronomical missing channel: 11 ∉ {T_Me, T_V, T_Ma, T_J, T_S, T_E, CR, T_tz, T_819, 365}.
    ///
    /// **v0.8.0 extension:** the vault's published T7 set covers
    /// `{T_V, T_Ma, T_J, T_S, T_E, CR, T_tz, T_819, T_h}`. Since the dccms
    /// substrate exposes `MERCURY_SYNODIC = 116` (vault §C3 confirms
    /// `116 mod 11 = 6 ≠ 0`), we extend the check to include Mercury.
    /// This strengthens but does not contradict the vault claim.
    pub fn t7_eleven_astronomically_absent() -> TheoremResult {
        use dresden_codex::MERCURY_SYNODIC;
        let periods = [
            (MERCURY_SYNODIC, "T_Me"),    // v0.8.0 extension; vault §C3
            (VENUS_SYNODIC, "T_V"),
            (MARS_SYNODIC, "T_Ma"),
            (JUPITER_SYNODIC, "T_J"),
            (SATURN_SYNODIC, "T_S"),
            (ECLIPSE_TABLE_DAYS, "T_E"),
            (CALENDAR_ROUND, "CR"),
            (260u64, "T_tz"),
            (COUNT_819, "T_819"),
            (365u64, "T_h"),
        ];
        for (p, name) in periods {
            if p % 11 == 0 {
                return TheoremResult::Fail {
                    reason: format!("11 | {} ({})", p, name)
                };
            }
        }
        TheoremResult::pass(
            "11 ∤ {116, 584, 780, 399, 378, 11960, 18980, 260, 819, 365} \
             (T_Me added v0.8.0)"
        )
    }

    /// **T8** — Saturn displacement = 2 · 11² (T-SHADOW-POWER).
    pub fn t8_saturn_shadow_power() -> TheoremResult {
        lemmas::l8_saturn_eleven_squared()
    }

    /// **T9** — Mars displacement = T_tz (one Tzolk'in advance).
    pub fn t9_mars_closes_tzolkin() -> TheoremResult {
        lemmas::l9_mars_tzolkin()
    }

    /// **T10** — S_R Distribution: Mars(5) + Venus(5,7) + Saturn(11²) = complete S_R.
    pub fn t10_s_r_distribution() -> TheoremResult {
        let l8 = lemmas::l8_saturn_eleven_squared();
        let l9 = lemmas::l9_mars_tzolkin();
        let l10 = lemmas::l10_venus_five_seven();
        if !l8.is_pass() { return l8; }
        if !l9.is_pass() { return l9; }
        if !l10.is_pass() { return l10; }
        // The union of S_R primes across Mars + Venus + Saturn = {5, 7, 11}.
        let delta_ma = ECLIPSE_TABLE_DAYS % MARS_SYNODIC;     // 260
        let delta_v  = ECLIPSE_TABLE_DAYS % VENUS_SYNODIC;    // 280
        let delta_s  = ECLIPSE_TABLE_DAYS % SATURN_SYNODIC;   // 242
        let mut union: Vec<u64> = Vec::new();
        for &p in &RAMANUJAN_S_R {
            if delta_ma % p == 0 || delta_v % p == 0 || delta_s % p == 0 {
                union.push(p);
            }
        }
        if union.as_slice() != &RAMANUJAN_S_R[..] {
            return TheoremResult::Fail {
                reason: format!("S_R union {:?} != {:?}", union, RAMANUJAN_S_R)
            };
        }
        TheoremResult::pass("Mars(5) ∪ Venus(5,7) ∪ Saturn(11²) = S_R = {5,7,11} (complete)")
    }
}

// ═══════════════════════════════════════════════════════════════════
// §VALIDATION IDENTITIES — V1 through V14
// ═══════════════════════════════════════════════════════════════════

pub mod validation {
    use super::*;

    pub fn v1()  -> bool { gcd(260, 365) == 5 }
    pub fn v2()  -> bool { lcm(260, 365) == 18_980 && 18_980 == 4 * 5 * 13 * 73 }
    pub fn v3()  -> bool { COUNT_819 == 9 * 91 && COUNT_819 == 9 * 7 * 13 && COUNT_819 == 3 * 3 * 7 * 13 }
    pub fn v4()  -> bool { ECLIPSE_TABLE_DAYS == 46 * 260 && ECLIPSE_TABLE_DAYS % 260 == 0 }
    pub fn v5()  -> bool { ECLIPSE_TABLE_DAYS % MARS_SYNODIC == 260 }
    pub fn v6()  -> bool { ECLIPSE_TABLE_DAYS % VENUS_SYNODIC == 280 && 280 == 8 * 5 * 7 }
    pub fn v7()  -> bool { ECLIPSE_TABLE_DAYS % SATURN_SYNODIC == 242 && 242 == 2 * 121 && 242 == 2 * 11 * 11 }
    pub fn v8()  -> bool { gcd(VENUS_SYNODIC, 365) == 73 }
    pub fn v9()  -> bool { lcm(VENUS_SYNODIC, 365) == 2_920 && 2_920 == 8 * 5 * 73 }
    pub fn v10() -> bool {
        260 % 11 != 0 && 365 % 11 != 0 && 584 % 11 != 0
            && 780 % 11 != 0 && 399 % 11 != 0 && 378 % 11 != 0
    }
    pub fn v11() -> bool {
        ECLIPSE_TABLE_DAYS % 11 != 0
            && CALENDAR_ROUND % 11 != 0
            && COUNT_819 % 11 != 0
    }
    pub fn v12() -> bool { ECLIPSE_CORRECTION == 3 * 31 && gcd(ECLIPSE_TABLE_DAYS, ECLIPSE_CORRECTION) == 1 }
    pub fn v13() -> bool {
        280 % 5 == 0 && 280 % 7 == 0  // 5 | 280 ∧ 7 | 280
            && 242 % 11 == 0 && 242 % (11 * 11) == 0  // 11 | 242 ∧ 11² | 242
    }
    /// **V14** — verbatim from vault `The Dresden Codex.md` §VALIDATION IDENTITIES.
    ///
    /// **Note:** this predicate is strictly weaker than the one verified by
    /// Theorem T2. The vault publishes V14 as the conjunction
    /// `5 ∣ T ∧ 13 ∣ T ∧ 4 ∣ lcm(T, 365)`, which is implied by but not
    /// equivalent to T2's full claim `lcm(T, 365) = 18,980`. Both
    /// predicates produce minimum `T = 260` over `T < 365`, but the
    /// stronger T2 predicate is verified by [`v14_strict`].
    pub fn v14() -> bool {
        // min{T < 365 : 5 ∣ T ∧ 13 ∣ T ∧ 4 ∣ lcm(T, 365)} = 260
        let mut min: Option<u64> = None;
        for t in 1u64..365 {
            if t % 5 == 0 && t % 13 == 0 && lcm(t, 365) % 4 == 0 {
                min = Some(min.map_or(t, |m| m.min(t)));
            }
        }
        min == Some(260)
    }

    /// **V14_strict** — companion identity matching Theorem T2's full predicate.
    ///
    /// `min{T < 365 : (5∣T ∨ 7∣T ∨ 11∣T) ∧ 13∣T ∧ lcm(T, 365) = 18,980} = 260`.
    ///
    /// This is the predicate Theorem T2 actually proves: T must (i)
    /// contain at least one S_R prime, (ii) contain the boundary prime 13,
    /// and (iii) achieve `lcm(T, 365) = 18,980`. The vault's V14 checks
    /// only the weaker `5∣T ∧ 13∣T ∧ 4∣lcm(T,365)` (which is consistent
    /// with V14_strict but does not imply it: e.g., T = 52 = 2²·13
    /// satisfies `lcm(52, 365) = 18,980` and `4∣18,980` but lacks any
    /// S_R prime, so it would pass a naïve `lcm = 18,980` check yet fails
    /// T2's full conjunctive predicate).
    ///
    /// Not part of the vault's V1-V14 list — added during v0.8.0 precision
    /// hardening to remove predicate drift between V-tier identities and
    /// the T-tier theorem.
    pub fn v14_strict() -> bool {
        // T2 condition (i): T contains at least one S_R prime.
        let has_s_r = |t: u64| -> bool {
            RAMANUJAN_S_R.iter().any(|&p| t % p == 0)
        };
        // T2 condition (ii): T contains the boundary prime 13.
        let has_boundary = |t: u64| -> bool { t % 13 == 0 };
        // T2 condition (iii): lcm(T, 365) = 18,980.
        let conductor_18980 = |t: u64| -> bool { lcm(t, 365) == 18_980 };

        let mut min: Option<u64> = None;
        for t in 1u64..365 {
            if has_s_r(t) && has_boundary(t) && conductor_18980(t) {
                min = Some(min.map_or(t, |m| m.min(t)));
            }
        }
        min == Some(260)
    }

    /// All 14 vault-published validation identities together.
    pub fn all() -> [bool; 14] {
        [v1(), v2(), v3(), v4(), v5(), v6(), v7(),
         v8(), v9(), v10(), v11(), v12(), v13(), v14()]
    }

    /// V14 plus the strict T2-aligned companion. Returns `(v14, v14_strict)`.
    pub fn v14_full_alignment() -> (bool, bool) {
        (v14(), v14_strict())
    }
}

// ═══════════════════════════════════════════════════════════════════
// Public entry point
// ═══════════════════════════════════════════════════════════════════

/// Run the full DPM-PRIME stack and return the result of each theorem.
pub fn run_full_stack() -> Vec<(&'static str, TheoremResult)> {
    vec![
        ("T1", theorems::t1_calendar_round_as_crt_torus()),
        ("T2", theorems::t2_tzolkin_minimum()),
        ("T3", theorems::t3_819_three_tier()),
        ("T4", theorems::t4_long_count_covers_calendar_round()),
        ("T5", theorems::t5_eclipse_two_phase_lift()),
        ("T6", theorems::t6_venus_sun_73()),
        ("T7", theorems::t7_eleven_astronomically_absent()),
        ("T8", theorems::t8_saturn_shadow_power()),
        ("T9", theorems::t9_mars_closes_tzolkin()),
        ("T10", theorems::t10_s_r_distribution()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─────────────────────────────────────────────────────────────────
    // Lemma tests
    // ─────────────────────────────────────────────────────────────────

    #[test] fn lemma_l1_passes()  { assert!(lemmas::l1_calendar_round().is_pass()); }
    #[test] fn lemma_l2_passes()  { assert!(lemmas::l2_tzolkin().is_pass()); }
    #[test] fn lemma_l3_passes()  { assert!(lemmas::l3_three_tier_819().is_pass()); }
    #[test] fn lemma_l4_passes()  { assert!(lemmas::l4_eclipse_tzolkin().is_pass()); }
    #[test] fn lemma_l5_passes()  { assert!(lemmas::l5_correction().is_pass()); }
    #[test] fn lemma_l6_passes()  { assert!(lemmas::l6_venus_sun().is_pass()); }
    #[test] fn lemma_l7_passes()  { assert!(lemmas::l7_no_synodic_carries_11().is_pass()); }
    #[test] fn lemma_l8_passes()  { assert!(lemmas::l8_saturn_eleven_squared().is_pass()); }
    #[test] fn lemma_l9_passes()  { assert!(lemmas::l9_mars_tzolkin().is_pass()); }
    #[test] fn lemma_l10_passes() { assert!(lemmas::l10_venus_five_seven().is_pass()); }

    // ─────────────────────────────────────────────────────────────────
    // Theorem tests
    // ─────────────────────────────────────────────────────────────────

    #[test] fn theorem_t1_passes()  { assert!(theorems::t1_calendar_round_as_crt_torus().is_pass()); }
    #[test] fn theorem_t2_passes()  { assert!(theorems::t2_tzolkin_minimum().is_pass()); }
    #[test] fn theorem_t3_passes()  { assert!(theorems::t3_819_three_tier().is_pass()); }
    #[test] fn theorem_t4_conditional() {
        // T4 is conditional per vault — the covering-space morphism is structural.
        let r = theorems::t4_long_count_covers_calendar_round();
        assert!(r.is_conditional(), "T4 should be Conditional, got {:?}", r);
    }
    #[test] fn theorem_t5_passes()  { assert!(theorems::t5_eclipse_two_phase_lift().is_pass()); }
    #[test] fn theorem_t6_passes()  { assert!(theorems::t6_venus_sun_73().is_pass()); }
    #[test] fn theorem_t7_passes()  { assert!(theorems::t7_eleven_astronomically_absent().is_pass()); }
    #[test] fn theorem_t8_passes()  { assert!(theorems::t8_saturn_shadow_power().is_pass()); }
    #[test] fn theorem_t9_passes()  { assert!(theorems::t9_mars_closes_tzolkin().is_pass()); }
    #[test] fn theorem_t10_passes() { assert!(theorems::t10_s_r_distribution().is_pass()); }

    // ─────────────────────────────────────────────────────────────────
    // Validation identity sweep
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn all_14_validation_identities_pass() {
        let results = validation::all();
        for (i, &result) in results.iter().enumerate() {
            assert!(result, "V{} failed", i + 1);
        }
        // Explicit per-identity to keep failure messages readable.
        assert!(validation::v1());
        assert!(validation::v2());
        assert!(validation::v3());
        assert!(validation::v4());
        assert!(validation::v5());
        assert!(validation::v6());
        assert!(validation::v7());
        assert!(validation::v8());
        assert!(validation::v9());
        assert!(validation::v10());
        assert!(validation::v11());
        assert!(validation::v12());
        assert!(validation::v13());
        assert!(validation::v14());
    }

    #[test]
    fn v14_strict_matches_theorem_t2_predicate() {
        // V14_strict checks T2's exact predicate (lcm = 18,980), not the
        // weaker vault V14 conditions. Both produce min = 260.
        assert!(validation::v14_strict());
        let (weak, strict) = validation::v14_full_alignment();
        assert!(weak, "V14 (weak / vault verbatim) failed");
        assert!(strict, "V14_strict (T2-aligned) failed");
    }

    // ─────────────────────────────────────────────────────────────────
    // Full-stack run
    // ─────────────────────────────────────────────────────────────────

    #[test]
    fn full_stack_nine_pass_one_conditional() {
        let results = run_full_stack();
        assert_eq!(results.len(), 10);
        let pass_count = results.iter().filter(|(_, r)| r.is_pass()).count();
        let cond_count = results.iter().filter(|(_, r)| r.is_conditional()).count();
        let fail_count = results.iter().filter(|(_, r)| !r.is_pass() && !r.is_conditional()).count();
        assert_eq!(pass_count, 9, "expected 9 Pass, got {}", pass_count);
        assert_eq!(cond_count, 1, "expected 1 Conditional (T4), got {}", cond_count);
        assert_eq!(fail_count, 0, "expected 0 Fail, got {}", fail_count);

        // T4 specifically is Conditional.
        let t4 = results.iter().find(|(name, _)| *name == "T4").unwrap();
        assert!(t4.1.is_conditional());
    }
}
