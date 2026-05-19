# Tier 3 — Complete Execution Plan

**Status:** PROPOSAL → EXECUTING
**Source:** synthesis report §3.2, §3.5, §3.6, §3.9, §3.13, §3.14, §10
**Prior state:** B-7 entirely complete, 555 tests, HEAD 139c518

Per user directive: this plan covers **every** remaining Tier 3 phase comprehensively. Execution will complete the whole plan, not a subset.

## Phase B-8 — DKAM Tier Mapping

**Source:** vault `The Dresden Codex.md` §DKAM Window, T-UDP-BOUNDARY Axiom A5.

```rust
// dccms_atlas/src/dkam_tier.rs

pub enum Tier {
    Face13,      // Tier 0: post-boundary collapse (e.g., 13 = F(7))
    Turbulent,   // Tier 1: ℓ > 11 (outside DKAM window)
    Boundary,    // Tier 2: ρ = deg (transition)
    Exact,       // Tier 3: S_R + DKAM exact regime (5, 7, 11, possibly 3)
}

pub fn fibonacci_index_prime(k: u64) -> Option<u64>;  // F(k) for the k-th Fibonacci-index prime
pub fn tier_of(prime: u64) -> Tier;
pub fn dkam_window(rho: u64) -> Vec<u64>;            // primes admissible at given ρ
```

**Tests:** F(4) = 3, F(7) = 13 (Fibonacci-index-of-prime); tier_of for each Safe-Basis prime; DKAM windows for ρ ∈ {2, 3, 4, 5, 7, 9}.

## Phase B-9 — Page Arithmetic

**Source:** vault `A-10_dresden_codex.md` §A.10.3.1, §A.10.3.2. **Provenance: Measured** (Kimi-observed, Claude-arithmetic-verified per A.10.4.1).

```rust
// dccms_atlas/src/page_arithmetic.rs

pub enum CodexPage { Page8Jaguar, Page52aRedBarrier }

pub struct PageArithmetic {
    pub page: CodexPage,
    pub provenance: Provenance,  // Measured (Kimi → Claude verification chain)
    pub kappa: u64,
    pub witness: String,
}

pub fn page_8_jaguar() -> PageArithmetic;
    // 13 + 8 = 21; 21 mod 7 = 0; 6⁻¹ mod 7 = 6; κ = 1

pub fn page_52a_red_barrier() -> PageArithmetic;
    // 21 · 0⁻¹ mod 13 = 7; κ = 12 · 7 mod 13 = 6
```

**Tests:** verify each arithmetic step in isolation; verify provenance tagging is Measured (not Proven) per discipline.

## Phase B-10 — Maya-Date API Ergonomics

**Source:** synthesis §3.5 (lifted from ASTRO21 PRACTICAL_MAYA_IMPLEMENTATION.md).

```rust
// dccms_atlas/src/maya_date.rs

pub struct MayaDate {
    pub tone: u8,            // 1..=13
    pub glyph: DayNameGlyph,
}

impl MayaDate {
    pub fn new(tone: u8, glyph: DayNameGlyph) -> Result<Self, MayaDateError>;
    pub fn from_day_number(day: u64) -> Self;
    pub fn to_day_number(&self) -> u64;
    pub fn advance_days(&self, days: u64) -> Self;
}
```

**Tests:** round-trip 0..=259; equivalence with `Tzolkin::from_day`; advance_days additive.

## Phase B-11 — Gini Stratification Verification

**Source:** synthesis §3.9 (Decoded.md §Algorithm 12 — "max/min density ratio exceeding 12:1"). **Status: Open** per synthesis O7 — **verify before mechanizing**.

```rust
// dccms_atlas/src/gini_stratification.rs

pub struct DensityRatio {
    pub max_density: u64,
    pub min_density: u64,
    pub ratio_num: u64,
    pub ratio_den: u64,
}

pub fn cram_address_density_per_carry_bit_signature() -> Vec<(u8, u64)>;
    // (carry_bit_signature, count) for all 30030 values in [0, M_SAFE)

pub fn density_max_min_ratio() -> DensityRatio;
```

**Verify before mechanize:** the test computes the actual ratio empirically. If it equals or exceeds 12:1, document; if not, document the actual value found and adjust the framing. **No drift from vault to code.**

## Phase B-12 — Goddess Section Extended (pages 13c–15)

**Source:** synthesis §6.3 (`Dresden.md` audit references pages 13c-15 per Barnhart 2005). **Status: Open** — figure-by-figure decoder requires source material we don't yet have.

```rust
// Extend dccms_atlas::codex_topology::CodexSection::MoonGoddess
// range to 13..=23 (currently 15..=23). Document that the H4 visual
// transducer's iconographic alphabet covers 16..=24 (with page 24 as
// BlankBridge) — a SUBSET of the extended MoonGoddess range.

// dccms_atlas/src/goddess_extension.rs

pub const GODDESS_EXTENDED_PAGES: &[u8] = &[13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23];

pub struct GoddessExtensionGap {
    pub pages: Vec<u8>,             // [13, 14, 15] — un-decoded
    pub h4_covered_pages: Vec<u8>,  // [16..=23] from h4_visual
    pub blank_bridge_page: u8,      // 24
    pub source_material_status: SourceMaterialStatus,
}

pub enum SourceMaterialStatus { Available, Pending, NotFound }
```

The open question is documented; the extended page range is exposed; the un-decoded subset is named explicitly. No silent omission.

## Discovery 2 — Prime Gap Doubling at Boundary

**Source:** synthesis §3.11, Maya CRT twp-0001 Discovery 2 ("average gap goes 2.20 → 4.40 at ℓ > 11").

```rust
// prime_hunt/src/prime_gap_analysis.rs

pub fn s_r_prime_gap_sum() -> (u64, u64);   // (numerator, denominator) for S_R gap
pub fn above_eleven_gap_average(end: u64) -> (u64, u64);

pub fn boundary_doubling_ratio() -> (u64, u64);
```

**Verify-before-mechanize:** compute the actual gap averages, compare to vault's 2.20 / 4.40 claim.

## Build order

```
B-8 DKAM tier mapping       ─┐
B-9 page_arithmetic         ─┤
B-10 Maya-date API          ─┼─→ (all independent; ship together)
B-11 Gini stratification    ─┤
B-12 Goddess extended       ─┤
Discovery 2 prime gaps      ─┘
```

## Test count projection

| Phase | Estimated tests |
|---:|---:|
| B-8 | 7 |
| B-9 | 6 |
| B-10 | 6 |
| B-11 | 5 |
| B-12 | 4 |
| Discovery 2 | 4 |
| **Total** | **32** |

Projected: 555 → ~587.

## Honest reporting policy

Per discipline:
- **B-9**: page_arithmetic carries `provenance: Measured` (NOT Proven) — observation source is Kimi, not human re-verified.
- **B-11**: verify the 12:1 ratio empirically; if it fails, the test documents the actual ratio rather than asserting the vault claim.
- **B-12**: figure-by-figure decoder for 13c-15 stays Open with explicit `SourceMaterialStatus::Pending`. No silent omission, no false claim of completeness.
- **Discovery 2**: verify the 2.20/4.40 numbers; document actual computed values.
