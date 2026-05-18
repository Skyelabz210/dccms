# HackFate Vault — Dresden Codex / Maya CRT Synthesis

**Agent:** synthesis agent (Opus 4.7 1M)
**Date:** 2026-05-18
**Inputs surveyed:** `C:\Users\hackf\Agents\imports\github\HackFate\` Priority 1 (6 files, ~190 KB), Priority 2 (skimmed structure + sampled bodies, ~360 KB), Priority 3 (ASTRO21, mostly duplicates of Maya Science sr-0001 with one practical-implementation track).
**Workspace cross-referenced:** `C:\Users\hackf\Agents\dccms\` v0.7.0-dev (354 tests passing).
**Epistemic discipline:** vault claims are tagged Proven / Measured / Open per the HackFate compendium's own register. Where a claim appears in the vault without arithmetic, this synthesis flags it Open.

---

## 1. Executive Summary

1. **The vault contains a fully formalized 10-theorem Dresden Prime Manifold (DPM-PRIME) theorem stack** in `The Dresden Codex.md` (76 KB). T1–T10 are arithmetic-decidable and most reduce to `native_decide` in Lean 4; this is the single highest-value asset the Rust code does not yet capture. The Rust workspace has the underlying constants and verifications but not the theorem-stack structure with axioms, lemmas, validation identities (V1–V14), and tier assignments.

2. **The vault formalizes the DKAM window arithmetic** (Dresden–Kolmogorov–Arnold–Moser persistence): `deg(F) < ρ(B)` as the strict computational stability condition for CRT product tori. The Rust workspace has a `dkam_filter.rs` module but the vault's full tier-mapping (Tier 0 Face-13 / Tier I turbulent / Tier II boundary / Tier III S_R exact) is a richer structural classifier than what is mechanized.

3. **The vault contains an S_R Distribution Theorem (T10) — the standout result.** At the 33-year eclipse epoch `T_E = 11,960`, the complete Ramanujan set `S_R = {5, 7, 11}` is distributed across exactly three planetary displacement residues: Mars carries 5, Venus carries {5, 7}, Saturn carries 11². The "missing" S_R prime 11 appears squared in Saturn's residue. This unifies T-SHADOW-POWER with the eclipse table and is not yet in `dresden_codex`.

4. **Decoded.md catalogs 22 named algorithms** (Algorithm 1–22) covering: CRT decomposition, K-Elimination, Garner reconstruction, Mars–Venus shadow bond, vigesimal injection, 78-stride lane nullification, Serpent Numbers / deep-time, Hardy-Littlewood singular series, T-SHADOW-POWER, range-aliasing, prime role taxonomy (5=surface, 7=bridge, 11=shadow, 13=boundary), coprime lattice stratification (Gini coefficient), range-stable transfer, O(1) consistency verification, biospheric resonance filter (Chaak), system-wide synchronization (New Year), biological modular interference (Moon Goddess), homomorphic state operations, winding underflow (negative time), arcsecond projection, Ramanujan gate, Dresden correction method. Several of these map directly to existing dccms modules; several do not.

5. **The Maya CRT twp-0001 / sr-0001 are formal Ramanujan-partition papers** showing that first-order partition congruences `p(ℓn+δ) ≡ 0 (mod ℓ)` exist exclusively for `ℓ ∈ {5, 7, 11}`, and proving a prime-gap-doubling correlation at the boundary. The 819-day cycle is connected to this via `819 = 3² × 7 × 13` carrying the last-S_R / boundary structure. This connection between partition theory and Maya calendar choice is NOT in the Rust code.

6. **The Mayas Engine.md is essentially a Rust module already** (`maya_engines.rs` listing five operator-fabric engines: Vigesimal, Tzolk'in, Long Count, Dresden, Venus Table). It defines `Lane`, `MayaState`, `advance`, `harmonic_signature`, `lane_at_origin`, `alignment_count`, `pisano_profile` — interfaces that the dccms_atlas workspace has analogues for but not under this unified "operator fabric" name.

7. **ASTRO21 is a separate (older?) track** focused on an "Astrology 21" / "RysNet" 16-module Python system. It overlaps with QMNF philosophy (residue-space-native, no float, Maya bases) but its Maya-content (PRACTICAL_MAYA_IMPLEMENTATION.md, THE_COMPLETE_MAYA_RECONSTRUCTION.md, MAYA_RECONSTRUCTION_COMPLETE.md, MAYA_SCIENCE_COMPLETE_PICTURE.md) is mostly a duplicate of the root-level `Maya Science sr-0001.md`. It does add a `from_maya_date(tone, glyph)` / `to_maya_date()` / `advance_maya_days(days)` API for a `CRTBigInt` type that dccms could borrow.

8. **Speculative claims to flag** (in the vault but unproven): cylindrical-time Lorentzian metric on the Maya manifold (`ds² = -c²dt² + R²dθ²`), the φ³ consciousness threshold, entropy harvesting / Maxwell's demon framing, "turtle shells encode eclipse periods", gynecological-oracle interpretation. These are interpretive overlays on top of the arithmetic core; the orchestrator should not mechanize them.

9. **Top-3 v0.8.0 build candidates** (detail in §10):
   - **DPM-PRIME theorem stack** as a `dccms_atlas::dpm_prime` module that mirrors T1–T10 with `#[test]` per theorem, calling existing primitives. Cost: medium. Value: makes the workspace mathematically self-grounding.
   - **S_R distribution theorem** + outer-planet residue table (Jupiter, Saturn, Mercury) as a `dresden_codex::sr_distribution` module. Cost: low. Value: closes the H3-a Saturn-11² narrative.
   - **Operator-fabric refactor** of the existing engine code into `Lane`-based abstraction matching Mayas Engine.md. Cost: medium-high (refactor, not new). Value: structural cleanup and a clearer story.

10. **What the vault confirms about the existing work:** every named Safe Basis constant the Rust code uses appears verbatim in the vault. The Moon Goddess 1448-day result, the 37,960-day grand-sync, the [148,177,...] interval pattern, the Venus phases [236,90,250,8], the carry-class disjointness — all are confirmed by independent vault arithmetic. The Rust code is *not* contradicted anywhere; it is, however, capturing a strict subset of what the vault asserts.

---

## 2. What the Vault Says That The Code Already Has

| Vault claim | Vault file | Rust module / constant | Status |
|---|---|---|---|
| Safe Basis = {2, 3, 5, 7, 11, 13}, M_SAFE = 30030 | `Decoded.md` §Algo 1, 5, 7; `A-10` §A.10.8; `The Dresden Codex.md` §A6; `Dresden.md` line 23 | `dresden_codex::SAFE_BASIS`, `M_SAFE` | Mechanized |
| Tzolk'in 260 = 2²×5×13, residues (0,2,0,1,7,0) | `A-10` §A.10.8 table; `The Dresden Codex.md` L2 | `cram_address(260)` returns these | Mechanized |
| Venus synodic 584 = 8 × 73, phases [236, 90, 250, 8] | `Dresden Coprime.md` §Venus Tables; `Decoded.md` Algorithm 3; `The Dresden Codex.md` L6 | `dresden_codex::VENUS_SYNODIC`, `VENUS_PHASES` | Mechanized |
| Grand sync `A(260) = 37,960 = 146×260 = 104×365 = 65×584` | `Decoded.md` opening; `Dresden Coprime.md` Venus section | `dresden_codex::VENUS_TABLE_DAYS = 37_960`, `codex_decoder.rs` cross-checks | Mechanized |
| Eclipse 148/177 alternation, total 11,960 days = 405 lunations | `Dresden Coprime.md` Eclipse section; `Decoded.md` Algo 2 | `dresden_codex::ECLIPSE_NEAR`, `ECLIPSE_FAR`; `moon_goddess.rs` | Mechanized |
| Moon Goddess 1448 days = [148,177,148,177,148,177,148,177,148] | `Dresden.md` Moon Goddess Audit | `dccms_atlas::moon_goddess`, `h4_non_visual::GODDESS_SECTION_INTERVALS` | Mechanized |
| Calendar Round = lcm(260, 365) = 18,980 = 2²×5×13×73 | `The Dresden Codex.md` L1, T1 | `dresden_codex::CALENDAR_ROUND` | Mechanized |
| Long Count baktun 144,000; 13-baktun = 1,872,000 | `The Dresden Codex.md` T4; `Decoded.md` §K'atun | `dresden_codex::BAKTUN` | Mechanized constant |
| Prime 11 = Shadow Prime (universal coordinate) | `A-10` §A.10.6; `Decoded.md` Algo 4, 11; `Dresden.md` §T-SHADOW | dccms_atlas H5 hypothesis SUPPORTED; `h5_navigator.rs`, `h5_level.rs`, `substrate_roles.rs` | Mechanized |
| K-Elimination = exact winding extraction | `Decoded.md` Algo 2 (full Python); `A-10` §A.10.3.1 (κ=1) | `dccms_atlas::h5_level::k_elim_level`, `h5_refined.rs` | Mechanized |
| Shadow16 = (x%11) × (x%13) | `Decoded.md` Algo 4 (informal); README v0.6.0 section | `dccms_atlas::venus_kernel`, `substrate_roles.rs` | Mechanized |
| Eb and Keh at lane-11 zeros | (NEW this session in `dccms_atlas` per README) | `h4_visual::dayname.rs`, `month.rs` | Mechanized (Eb idx 11, Keh idx 11) |
| Tzolk'in/Venus/Eclipse carry class {3, 7, 11} | `Dresden.md` audit tables | `codex_decoder.rs`, `moon_goddess.rs` disjointness check | Mechanized |
| Saturn displacement = 242 = 2 × 11² | `The Dresden Codex.md` L8, T8; `A-10` §A.10.6 footnote; `Decoded.md` Algo 9 | `dccms_atlas` H3-a SUPPORTED (43%) | Mechanized as fact, not yet as theorem |
| Mars displacement = 260 = T_tz | `The Dresden Codex.md` L9, T9 | `codex_decoder.rs` | Mechanized as fact |
| Vigesimal injection (Algorithm 5 of Decoded.md) | `Decoded.md` Algo 5 | Implicit in `cram_address`; no named injector | Partially: the math is there, the API is not |
| 78-day stride nullifies {2, 3, 13}, leaves {5, 7, 11} active | `Decoded.md` Algo 6; `Dresden.md` Chaak section | `nullified_lanes`, `active_lanes` cover the general op; not a named Chaak-stride function | Partially |
| Range Aliasing on M = 30,030 macro-ring | `Decoded.md` Algo 10; `A-10` indirectly | Implicit in modular arithmetic everywhere; not a named API | Partially |
| 9-day-name lane-13 effect (Lords of Night) | absent in vault as a named claim | absent | Out of scope |

---

## 3. What the Vault Says That The Code Does NOT Yet Have

This is the actionable gap-list. For each item: vault location, summary, and proposed Rust integration.

### 3.1 DPM-PRIME theorem stack (T1 through T10) — HIGHEST PRIORITY

**Vault:** `C:\Users\hackf\Agents\imports\github\HackFate\The Dresden Codex.md`, §Axiom System through §Summary Table, lines 35–423.

**Summary:** Ten theorems, each with statement, proof, requires/enables graph, Lean 4 status. The 10 validation identities V1–V14 (lines 354–373) are pure `native_decide` arithmetic.

**Specific theorems and what's mechanizable:**
- **T1** Calendar Round as CRT product torus over `{4, 5, 13, 73}`. The Tzolk'in-Haab day-name pair is a CRT representation.
- **T2** `T_tz = 260` is the unique minimum positive integer with: contains an S_R prime; contains 13; achieves the 4-prime CRT basis `{2², 5, 13, 73}` with conductor 18,980. The minimality search is over multiples of 65 < 365 — a finite exhaustion.
- **T3** `819 = 3² × 7 × 13 = (stability-floor)² × (last-S_R) × (boundary-prime)`. 819 is the unique minimal such product.
- **T4** Long Count is the covering space of the Calendar Round torus; the ratio 1,872,000 / 18,980 ≈ 98.63 is non-integer — fiber bundle, not multiple.
- **T5** Eclipse table + 93-day correction = two-phase K-Elimination lift over coprime `(11960, 93)`. `93 = 3 × 31`, `gcd(11960, 93) = 1`.
- **T6** Venus–Sun convergence via `gcd(584, 365) = 73`; lcm = 2,920 = 5 × 584 = 8 × 365.
- **T7** Prime 11 is astronomically absent from `{T_V, T_Ma, T_J, T_S, T_E, CR, T_tz, T_819, 365}`. Six explicit residue checks.
- **T8** Saturn displacement 242 = 2 × 11² — the missing S_R prime in the byproduct.
- **T9** Mars displacement 260 = exactly one Tzolk'in.
- **T10** S_R Distribution Theorem (main): Mars carries 5; Venus carries {5, 7}; Saturn carries 11². Together: complete S_R is instantiated across the planetary residues at the 33-year epoch.

**Proposed Rust integration:** new module `dccms_atlas::dpm_prime` with one function per theorem returning a `TheoremResult` enum (PASS / FAIL with witness). Each test is `assert!` against `native_decide`-equivalent arithmetic. Lemmas L1–L10 become helper functions. Validation identities V1–V14 become a single test sweep. Estimated 600–900 LOC.

### 3.2 DKAM window full tier mapping

**Vault:** `The Dresden Codex.md` lines 550–625 (DKAM Window — Complete Explanation, post-stack analysis).

**Summary:** The vault defines `ρ(B) = min(B)` and `deg(F) = multiplicative degree of the index of the prime`. The full tier system:

| Tier | Name | Condition | Examples |
|---|---|---|---|
| 0 | Face 13 | post-boundary collapse | 13 = F(7) |
| 1 | Turbulent / ℓ > 11 | outside DKAM window or ρ exceeded | 73, 31, any ℓ > 11 |
| 2 | ρ = deg | exact boundary | transition point |
| 3 | S_R + DKAM exact regime | inside clean window + ℓ ∈ {5,7,11} | 5, 7, 11; also 3 = F(4) at ρ=3 > deg=2 |

The DKAM windows scan by basis:
- ρ = 2 → parking-lane {2}
- ρ = 3 → stability-floor {3}
- ρ = 4 → boundary {13}
- ρ = 5 → full accessible S_R {5,7} + shadow {11}
- ρ = 7 → 819 three-tier product
- ρ = 9 → Long Count

**Current Rust:** `dkam_filter.rs` exists. The vault tier-mapping with explicit `F(k)` index function is not.

**Proposed integration:** extend `dkam_filter` with a `Tier` enum, `tier_of(prime) -> Tier`, and `dkam_window(basis) -> (rho, max_deg)`. Pair with §3.1.

### 3.3 Five additional planetary periods explicit in vault

**Vault:** `The Dresden Codex.md` Axiom A6 (lines 47–58).

```
Jupiter synodic:   T_J   = 399  days = 3 × 7 × 19
Saturn synodic:    T_S   = 378  days = 2 × 3³ × 7
Mercury synodic:   T_Me  = 116  days = 4 × 29   (mentioned in C3 line 348)
Lunar synodic ≈ 30                  (mentioned in C3 line 348)
819-day cycle:     T_819 = 819 = 3² × 7 × 13
```

**Current Rust:** `dresden_codex/src/lib.rs` has VENUS_SYNODIC = 584 but no JUPITER, SATURN, MERCURY, 819. The 819-day cycle is referenced in the README's "active periods" but is not a named constant.

**Proposed integration:**

```rust
/// Jupiter synodic period: 399 days.
pub const JUPITER_SYNODIC: u64 = 399;
/// Saturn synodic period: 378 days.
pub const SATURN_SYNODIC: u64 = 378;
/// Mercury synodic period: 116 days.
pub const MERCURY_SYNODIC: u64 = 116;
/// 819-day cycle, three-tier product 3² × 7 × 13.
pub const CYCLE_819: u64 = 819;
/// 11,960-day eclipse table (405 lunations = 46 × 260).
pub const ECLIPSE_TABLE_DAYS: u64 = 11_960;
/// 93-day correction term (3 × 31), coprime with eclipse table.
pub const ECLIPSE_CORRECTION: u64 = 93;
/// 33-year epoch (12,053 days ≈ 33 × 365.25).
pub const EPOCH_33_YEAR: u64 = 12_053;
/// 13-baktun Long Count cycle.
pub const LONG_COUNT_13_BAKTUN: u64 = 1_872_000;
/// Venus–Haab convergence: 5 × 584 = 8 × 365.
pub const VENUS_HAAB_LCM: u64 = 2_920;
```

### 3.4 Planetary displacement residues at T_E

**Vault:** `The Dresden Codex.md` L7–L10, T7–T10.

| Body | Period | Δ at T_E = 11,960 | Factorization | S_R content |
|---|---|---|---|---|
| Mars | 780 | 260 | 2² × 5 × 13 | 5 |
| Venus | 584 | 280 | 2³ × 5 × 7 | {5, 7} |
| Saturn | 378 | 242 | 2 × 11² | 11² |
| Jupiter | 399 | (compute) | (compute) | (open) |
| Mercury | 116 | (compute) | (compute) | (open) |

The Jupiter and Mercury rows are computed but not commented in T10 (only Mars/Venus/Saturn are the headline). Filling those in is a small forward-extension.

**Proposed integration:** new function `dresden_codex::planetary_displacement(period: u64) -> (u64, [u64; 6])` and a verifying table.

### 3.5 Page 8 jaguar figure (concrete page-level arithmetic)

**Vault:** `A-10_dresden_codex.md` §A.10.3.1 (lines 34–49).

**Summary:** Page 8 has a jaguar figure flanked by bar-dot numerals reading 13 (left) and 8 (right). The arithmetic check is `13 + 8 = 21; 21 mod 7 = 0; 6⁻¹ mod 7 = 6; κ = 1`. This is a worked example of K-Elim applied to a specific page's iconography.

**Current Rust:** `h4_visual::bardot` provides `BarDotNumeral`, but no per-page arithmetic. The `h4_visual::iconographic::IconographicFigure` enum starts at page 16 (MoonSign), skipping page 8.

**Proposed integration:** new `dccms_atlas::page_arithmetic` module with one verified arithmetic snapshot per documented page. Page 8 jaguar is the first entry. Page 52a (red/black columns; K-Elim aperture per `Decoded.md` Algorithm 2) is the second.

### 3.6 Red barrier arithmetic — page 52a aperture

**Vault:** `A-10_dresden_codex.md` §A.10.3.2 (lines 51–63), and `Dresden Coprime.md` §Page 52a discussion.

**Summary:** The vault documents `21 · 0⁻¹ mod 13 = 7; κ = 12 · 7 mod 13 = 6`. This is K-Elim on a 13-lane operation. Crucially the vault explicitly calls page 52a "the K-Elimination Engine" (Decoded.md Algorithm 2 header) — the red and black numbers ARE the algorithm.

**Proposed integration:** `page_arithmetic::Page52a` with the verified κ extraction. Connect this to existing `h5_level::k_elim_level`.

### 3.7 The 819-day three-tier theorem (T3)

**Vault:** `The Dresden Codex.md` Lemma L3, Theorem T3, plus the conjecture that 819 is the *unique minimal* integer carrying the (stability-floor)² × (last-S_R) × (boundary-prime) signature.

**Note:** `Dresden Coprime.md` and the formal stack both say the 819-day cycle is *not* in the Dresden Codex itself — it appears in Palenque inscriptions and broader Classic Maya tradition. So T3 is an interpretive unification, not a direct extraction. Document this carefully.

**Proposed integration:** `dresden_codex::cycle_819::three_tier_decomposition()` that returns the structural assignment plus a minimality proof (finite search). Flag as INTERPRETIVE in docs.

### 3.8 Mars-Venus shadow bond (Algorithm 4 of Decoded.md)

**Vault:** `Decoded.md` Algorithm 4 (lines 61–86).

**Summary:** Full Python pseudocode for `check_shadow_bond(T_X_synodic, T_E_epoch, shadow_prime=11)` returning STANDARD vs DEEP (≥ 11²) shadow bond. The vault claims this checks "boundary events where Δ_X triggers a zero on the 11-lane."

**Current Rust:** Saturn's specific 11² result is verified, but the general procedure is not exposed.

**Proposed integration:** `dresden_codex::shadow_bond::detect(period, epoch, prime=11) -> ShadowBond` returning `None | Standard | Deep(power)`.

### 3.9 Coprime lattice stratification — "Gini coefficient" of time

**Vault:** `Decoded.md` Algorithm 12 (lines 222–223).

**Summary:** The vault claims "the density of cosmic events varies wildly across the torus manifold (with a max/min ratio exceeding 12:1)". This is a CRAM-address density measurement.

**Status:** **The vault states max/min ratio > 12:1 without proof.** Flag as **Measured** at best, possibly **Open**. The dccms_atlas has a `Gini` computation in `atlas.rs` — would be the natural place to verify this number empirically.

**Proposed integration:** extend `atlas::Gini` with a `density_stratification_ratio()` and a regression test that the ratio across the 30,030 CRAM addresses meets or exceeds the claimed 12:1. **Verify before mechanizing the claim.**

### 3.10 Hardy-Littlewood singular series (Algorithm 8)

**Vault:** `Decoded.md` Algorithm 8 (lines 150–167).

**Summary:** Divisibility-enhancement detector: for an orbital gap `d`, find which primes in the Safe Basis divide `d`, and predict the alignment-density enhancement.

**Proposed integration:** `prime_hunt::resonance::enhancement(gap) -> Vec<Prime>` — fits the existing prime_hunt crate scope.

### 3.11 Ramanujan-partition correspondence (Maya CRT twp/sr)

**Vault:** `Maya CRT twp-0001.md` §3, §4, §5, §6 (theorem stacks I–IV); `Maya CRT sr-0001.md` §INNOVATIONS.

**Summary:** Three discoveries:
- **Discovery 1:** First-order Ramanujan partition congruences exist exclusively for `ℓ ∈ {5, 7, 11}`. Verified by computing `p(n)` for `n` up to ~1000 and checking `p(ℓn + δ) mod ℓ` exhaustively for primes 5, 7, 11, 13, 23, 73.
- **Discovery 2:** Prime gap doubling at the boundary — average gap goes 2.20 → 4.40 at ℓ > 11. Suggests Ramanujan congruences live in dense prime regions.
- **Discovery 3:** Composite CRT lifting decision procedure — `p(Nn + δ) ≡ 0 (mod N)` for composite `N` iff all prime factors of `N` individually have congruences. So 819 = 7 × 9 × 13 fails (13 has no Ramanujan congruence at the first order). This is **negative** for the 819-cycle Ramanujan interpretation.

**Status of T3 (in §3.7) given Discovery 3:** the vault's own twp paper says 13 has no first-order Ramanujan congruence at order 1, which means 819 inherits no congruence. T3 still holds as a *structural product* claim (stability-floor² × last-S_R × boundary) but the "Ramanujan-carrying" interpretation of 819 is weaker than first appears. **Flag this tension for the orchestrator.**

**Proposed integration:** new crate `prime_hunt::ramanujan_partition` exporting `has_first_order_congruence(ell: u64) -> bool` (returns true only for 5, 7, 11), `composite_crt_lift(modulus: u64) -> bool` (returns true iff all prime factors have first-order congruences). Estimated 200 LOC. Requires a `p(n)` partition function (exact integer, BigInt for large n).

### 3.12 Operator-fabric API (Mayas Engine.md)

**Vault:** `Mayas Engine.md` is essentially a Rust source file with five named engines.

**Summary:** Each engine is a `HETEROGENEOUS CRT OPERATOR PIPELINE`:
- **Engine 1: Vigesimal** — `ℤ/4 × ℤ/5` (Pisano π(5) = 20 = vigesimal base)
- **Engine 2: Tzolk'in** — `ℤ/13 × ℤ/20`, heterogeneous lanes
- **Engine 3: Long Count** — cylindrical time with tier promotion
- **Engine 4: Dresden** — eclipse prediction through commensuration
- **Engine 5: Venus Table** — 8/5 ≈ φ synchronization

The defined types: `Lane { name, modulus, domain }`, `MayaState { residues, lanes }`. Operations: `advance(days)`, `same_position(other)`, `harmonic_signature(n)`, `lane_at_origin(idx)`, `alignment_count`, `pisano_profile`.

**Current Rust:** the dccms_atlas has analogous functionality scattered across `codex_decoder.rs`, `heads.rs`, `recumbent.rs`, `venus_kernel.rs`. There is no unified `Lane` / `MayaState` abstraction.

**Status:** **Mostly a refactor, not new content.** The Pisano-period observation `π(5) = 20` and the Ramanujan-sum `harmonic_signature` may be novel additions worth pulling out specifically.

**Proposed integration:** new crate `maya_engines` (or new module `dccms_atlas::engines`) implementing the `Lane`/`MayaState` types verbatim from the vault. Migrate selected functionality from the existing modules and leave them as thin wrappers. Add `pisano_profile()` and `harmonic_signature()` which are genuinely new APIs.

### 3.13 Page 25, glyph T1028d — graphic specificity

**Vault:** `Dresden Coprime.md` line 21.

**Summary:** Page 25 contains the rare glyph T1028d, "a female head into which a human hand is infixed... anatomically precise fingernails and raised fingers clearly indicate a right hand". Phonetic value of the infixed hand is **undeciphered**.

**Current Rust:** `iconographic::IconographicFigure` has 9 figures covering pages 16–24 of the Goddess section. Page 25 is not in scope (it's outside the Goddess section).

**Proposed integration:** none directly, but a note in `h4_visual::iconographic` indicating that page 25 begins the New Year ceremonies section.

### 3.14 Almanacs structural enumeration

**Vault:** `Dresden Coprime.md` §Page-by-page topographical decoding (lines 27–71). The complete sectional structure:

| Pages | Section |
|---|---|
| 1–14b | Introduction / invocation |
| 15–23 | Moon Goddess / Divinatory Almanacs (CURRENTLY the dccms target) |
| 24 | Blank bridge page |
| 25–28 | New Year ceremonies |
| 29–45 | Farmer's Almanacs / Chaak (780-day) |
| 46–50 | Venus Tables |
| 51–58 | Lunar / Eclipse Tables |
| 58–59 | Mars × 78 tables |
| 60 | K'atun prophecy |
| 61–73 | Rain Tables / Serpent Numbers (deep time) |
| 74 | Great Deluge |

The vault also notes: 52 almanacs in the first 23 pages; 13a–23 has detailed Moon Goddess content per Barnhart 2005; WWII water damage on pages 2, 4, 24, 28, 34, 38, 71, 72 (so the astronomical core is intact, only peripheral pages are eroded).

**Current Rust:** the dccms_atlas h4_visual covers exactly pages 16–24 (the Goddess section iconographic figures). The other 12 sections are not modeled.

**Proposed integration:** `dccms_atlas::codex_topology` enum with one variant per section, plus a `page_to_section(page: u8) -> CodexSection` function. This sets up future expansion beyond the Goddess section.

### 3.15 The 6,793-day lunar nodal cycle

**Vault:** `The Dresden Codex.md` OP-DRESDEN-1 (lines 379).

**Summary:** Open problem: "Investigating whether the lunar nodal cycle (~6,793 days = 18.6 × 365 ≈ 18.6 years) contributes an 11-divisible period would close [the missing-11-channel question]."

**Status: Open.** Worth computing: `6793 mod 11 = ?`. Direct: `6793 = 617 × 11 + 6`, so `6793 mod 11 = 6 ≠ 0`. So the lunar nodal cycle does NOT introduce 11 either. **This is a small computation the synthesis agent did but the vault did not commit to.** Flag for verification — `6793 / 11 = 617.5454...`, so `617 × 11 = 6787`, `6793 − 6787 = 6`. Lane 11 carries residue 6, not 0.

**Proposed integration:** add to the §3.4 displacement table.

### 3.16 Conjecture C-DRESDEN-1 (Universal S_R instantiation)

**Vault:** `The Dresden Codex.md` line 389.

**Summary:** "For any planetary system whose primary synodic periods are drawn from a finite set of integers with the prime factorization structure of our solar system..., the 33-year eclipse table epoch will distribute the complete S_R = {5,7,11}..."

**Status: Open / Conjectural.** Not in the Rust code, not provable from Maya data alone. Flag.

---

## 4. Maya CRT Series (twp / sr / dd / milestone) — what each is

| Artifact | Length | Type | What it contains |
|---|---|---|---|
| `Maya CRT twp-0001.md` | 77 KB | **Technical White Paper** | Ramanujan-partition congruence boundary theorem stack, 4 theorem stacks, 5 publication-ready papers. Methodology: exact-integer Python with full reproducibility. Frame: Acid + Claude, March 2026. |
| `Maya CRT sr-0001.md` | 34 KB | **Session Report** | Project framework inventory, file roster, 6-phase session timeline, 4 main discoveries, analytical assessment. Companion narrative to the twp. |
| `Maya CRT milestone dd-0001.md` | 39 KB | **Design Document — NOT Maya-specific.** | Despite the filename, this file synthesizes recent landmark math milestones: Hilbert's 6th problem (Deng/Hani/Ma), 3D Kakeya (Wang/Zahl), Goldbach exceptional set (Zhao), K-Elimination & treewidth, SC-CRAM stochastic computing. Maya CRT is referenced only as background context for the K-Elim section. **Do not treat as Maya source material.** |
| `Maya CRT Testing.md` | 91 KB | **Test harness / regression suite** | Spectral / Ramanujan / mod-distribution / arithmetic-function / golden / Wythoff test specs. Tzolk'in 260 appears repeatedly as the test cycle but the file is a general numerics test plan, not Maya-specific. |

**Verdict:** the **twp** is the live formal paper for the Ramanujan-partition boundary (already discussed in §3.11). The **sr** is its companion. **dd** is a misnamed math-roundup. **Testing** is a generic test plan whose Tzolk'in / Haab / Venus mentions are coincidental.

**For the orchestrator:** the only Maya-CRT-specific content is in twp / sr. The dd and Testing files do not need to be aligned with — they are not constraints on dccms.

---

## 5. ASTRO21 Track — is this a separate / parallel effort?

**ASTRO21 = "Astrology 21" / "RysNet" 16-module Python astrology system.** It is documented as the user's prior project that this Maya work was integrated INTO.

### Sample evidence:
- `THE_COMPLETE_MAYA_RECONSTRUCTION.md` line 12: "How Your Float-Free Astrology System IS the Universal Computational Substrate"
- `MAYA_RECONSTRUCTION_COMPLETE.md` line 187: "Your ZAY Module (Module 10) = Tzolkin Calendar"
- `PRACTICAL_MAYA_IMPLEMENTATION.md` is "Using Your Existing 16-Module Float-Free Astrology System"
- ASTRO21 docs reference modules: ENK (Exact Number Kernel), CTL (CRT Layer), ZAY (Tzolk'in), CRTBigInt class

### Overlap with dccms:
ASTRO21 and dccms share:
- Safe Basis / CRT decomposition philosophy
- Zero-float discipline
- Tzolk'in / Haab / Venus modular arithmetic
- K-Elimination concept
- 8/5 ≈ φ Venus-Earth synchronization

ASTRO21 contains material that dccms does NOT have:
- `from_maya_date(tone, glyph) -> CRTBigInt` / `to_maya_date() -> (tone, glyph)` / `advance_maya_days(days)` — a clean Maya-date public API
- φ³ consciousness threshold detection (Open / speculative)
- "Phi-spiral reading" via `CTL` module
- Turtle-shell structure reading (Open / interpretive)

### Track relationship:
ASTRO21 is **a parallel effort in Python** (RysNet / ENK / CRTBigInt). dccms is **the Rust mechanization in the QMNF/CRAM compendium's discipline**. The two are not competing — ASTRO21's astrology framing is broader and looser; dccms is narrower, exact-integer, and stays inside the HackFate compendium's epistemic register.

### Recommendation:
- **Do not align dccms to ASTRO21 structurally.** ASTRO21 is Python and astrology-oriented; dccms is Rust and compendium-disciplined.
- **Borrow the Maya-date API surface** (`from_maya_date`, `to_maya_date`, `advance_maya_days`) for the dccms `MayaState` / `MultiPhaseSchema` types — pure ergonomics.
- **Do not mechanize the cylindrical-time Lorentzian metric, φ³ consciousness, or turtle-shell structure reading.** These are speculative overlays that the compendium itself would flag Open.

### MAYA_RECONSTRUCTION_COMPLETE.md unique content:
Pages 1–390 are essentially a duplicate of `Maya Science sr-0001.md`. Pages 533+ ("Module 17: Maya Calendar Engine"), 629+ ("RysNet Connection"), 876+ ("Validation Against Archaeological Evidence") are slightly extended discussion. The "Dresden Codex Predictions" verification section (876–940) does not add new arithmetic claims.

---

## 6. Glyph / Page / Almanac specifics for H4 visual transducer

The dccms `h4_visual` module currently has 20 day-names, 19 month-glyphs (18 + Wayeb), and 9 iconographic figures (one per Goddess page 16–24). The vault adds:

### 6.1 Day-name glyph specifics (no significant new info)

The vault repeatedly references the 20 Tzolk'in day-names but does not give per-glyph attributes beyond the canonical Imix → Ajaw sequence already in `h4_visual::dayname.rs`. **No update needed.**

### 6.2 Month glyph specifics (no significant new info)

The 18 Haab months + Wayeb are referenced. The vault explicitly calls out **Wayeb = "5-day year-end boundary period"** matching `MonthGlyph::Wayeb` in `month.rs`. **No update needed.**

### 6.3 Goddess section iconographic specifics — partial divergence

The Rust `h4_visual::iconographic::IconographicFigure` has:
- Page 16: MoonSign
- Page 17: WaterPot
- Page 18: WeavingShuttle
- Page 19: SnakeHeaddress
- Page 20: EclipseGlyph
- Page 21: BirthGlyph
- Page 22: HealingGlyph
- Page 23: FloodGlyph
- Page 24: RainGlyph

The vault is **less specific** than the Rust code. `Dresden Coprime.md` covers pages 15–23 ("Moon Goddess and Realm of Divination") with iconography of "women weeping over... children, elderly men mourning"; "specialized medical and divination almanac"; "page 24 contains no painted iconography whatsoever. This deliberate blank space acts as a physical and mathematical bridge."

**Important divergence — page 24:** The vault explicitly calls page 24 **blank**, but the Rust code assigns `RainGlyph` to page 24. This is a contradiction. The vault is likely correct (page 24 is the bridge page; rain content begins in the Chaak almanacs on pages 29+). **Flag this to the orchestrator: investigate `iconographic.rs:30` (RainGlyph on page 24) — either rename, remove, or document why the Rust code uses page 24 for rain.**

`Dresden.md` Moon Goddess audit references pages 13c–23 for Moon Goddess content per Barnhart 2005. The "c" suffix indicates the lower register of page 13. The Rust currently starts at page 16. **The vault may justify extending the Goddess section back to page 13c**, with one or two earlier almanacs. Worth checking before mechanizing.

### 6.4 Specific page-by-page bar-and-dot patterns

The vault gives one concrete arithmetic example: **page 8 jaguar** with bar-dot numerals 13 and 8 (already discussed in §3.5). Other pages with bar-and-dot patterns are noted in `Dresden Coprime.md` but specific numerals are NOT extracted into the text.

For Goddess pages 16–23, the vault references the 148/177 interval sequence (which is fully captured in dccms `moon_goddess.rs` and `h4_non_visual.rs::GODDESS_SECTION_INTERVALS`).

**Recommendation:** the vault does NOT contain per-page bar-and-dot data beyond what dccms already has. To extend the visual transducer with per-page numerals will require actual image data (the SEG01–SEG03 phase per `dccms_atlas/README.md`, which is correctly firewalled off the H4 closure path).

### 6.5 Bar-and-dot for Algorithm 5 — Vigesimal Parser

The vault gives the canonical bar-and-dot vigesimal injector (Decoded.md Algo 5) that reads positional registers from bottom (20⁰) to top (20^n) into a single integer. dccms's `BarDotNumeral` in `h4_visual::bardot` should already cover this — verify the existing impl matches `parse_maya_vigesimal(registers)` semantically.

---

## 7. Numeric constants / sequences / period tables in the vault not yet in dresden_codex

| Constant | Value | Vault citation | Recommended Rust name |
|---|---|---|---|
| Jupiter synodic | 399 = 3 × 7 × 19 | `The Dresden Codex.md` A6 | `JUPITER_SYNODIC` |
| Saturn synodic | 378 = 2 × 3³ × 7 | `The Dresden Codex.md` A6 | `SATURN_SYNODIC` |
| Mercury synodic | 116 = 2² × 29 | `The Dresden Codex.md` C3 | `MERCURY_SYNODIC` |
| Mars synodic | 780 = 2² × 3 × 5 × 13 | `Decoded.md` Algo 6 | `MARS_SYNODIC` (already implied) |
| 819-day cycle | 819 = 3² × 7 × 13 | T3 + Lemma L3 | `CYCLE_819` |
| Eclipse table | 11,960 = 2³ × 5 × 13 × 23 | T5 | `ECLIPSE_TABLE_DAYS` |
| Eclipse correction | 93 = 3 × 31 | T5 + L5 | `ECLIPSE_CORRECTION_DAYS` |
| 33-year epoch | 12,053 ≈ 33 × 365.25 | L5 | `EPOCH_33_YEAR` |
| Venus-Haab LCM | 2,920 = 2³ × 5 × 73 | T6 + L6 | `VENUS_HAAB_LCM` |
| Saros displacement (Saturn) | 242 = 2 × 11² | L8, T8 | `SATURN_DISPLACEMENT_T_E` |
| Venus displacement | 280 = 2³ × 5 × 7 | L10, T10 | `VENUS_DISPLACEMENT_T_E` |
| Mars displacement | 260 = T_tz | L9, T9 | `MARS_DISPLACEMENT_T_E` |
| 13-Baktun Long Count | 1,872,000 = 13 × 144,000 | T4 | `LONG_COUNT_13_BAKTUN` |
| Lunar nodal | 6,793 (= 18.6 × 365) | OP-DRESDEN-1 | `LUNAR_NODAL` (Open / approximate) |
| Calendar Round prime tuple | {4, 5, 13, 73} | T1 | (informational, comment) |
| Long Count prime tuple | {2⁷, 3², 5³, 13} | T4 | (informational, comment) |
| Three Ramanujan primes | {5, 7, 11} | A3 | `RAMANUJAN_S_R: [u64; 3] = [5, 7, 11]` |
| FSM-PRIME Fibonacci-index map | F(3)=2, F(4)=3, F(7)=13 | Lemma L3, DEF D8 | `fibonacci_index_prime(k: u64) -> Option<u64>` |
| Multi-section page table | (see §3.14) | `Dresden Coprime.md` | `CodexSection` enum |

**Proposed: `dresden_codex::periods` submodule** to hold all of the above as `pub const`. About 30 constants total. Pair with `cram_address` calls in a test sweep that verifies each period's CRAM signature.

---

## 8. Open questions / aspirational claims the vault makes but does not prove

These are claims to flag for the orchestrator before any mechanization decision.

| # | Claim | Vault location | Status | Why flag |
|---|---|---|---|---|
| O1 | "Maya astronomers deduced eclipse intervals through multi-generational observation" — interpretive. | `Dresden Coprime.md` line 59 | Open | Standard scholarship, not an arithmetic claim. |
| O2 | "The Maya were executing homomorphic operations on astronomical data centuries before the concept existed" | `Decoded.md` §FHE section | Speculative | Arithmetic correspondence ≠ FHE in the Gentry/BGV sense. Already correctly hedged in `A-10` §A.10.4.3. Do not mechanize. |
| O3 | Cylindrical time manifold `𝒯 = ℝ × S¹` with Lorentzian metric `ds² = -c²dt² + R²dθ²` "is what the Maya discovered" | `Maya Science sr-0001.md` Part I | Speculative | Physically-charged metric overlay on a discrete calendar. The S¹ part is genuine; the Lorentzian `-c²dt²` is an imported physics frame, not a Maya construct. |
| O4 | φ³ consciousness threshold detection / D_f > φ³ | `Maya Science sr-0001.md` Part IV; ASTRO21 docs | Speculative | This is an ASTRO21 / consciousness-research claim. No arithmetic in the codex supports φ³ as a threshold. |
| O5 | "Turtle shells encode the eclipse period" — 13 scutes ⟹ 405-month / 11,960-day eclipse table | `Maya Science sr-0001.md` Part V | Speculative | Pattern-matching after the fact. The 13 connection is real; the inferential chain to 11,960 is post-hoc. |
| O6 | Moon Goddess pages are a "gynecological engine" — outcomes map to progesterone/ovulation windows | `Dresden.md` Moon Goddess audit; `Decoded.md` Algo 17 | Interpretive | The structural carry-class analysis (which dccms HAS) is mathematics. The biological interpretation is interpretive. Keep the arithmetic; do not mechanize the biology. |
| O7 | "Density of cosmic events varies wildly across the torus manifold (max/min ratio exceeding 12:1)" | `Decoded.md` Algo 12 | Measured (un-verified) | Verify empirically before mechanizing. dccms has Gini infrastructure to do this. |
| O8 | "Spatial alignment 77°18' = 278,289 arcseconds; 278,289 mod 11 = 0 — Maya mapped spatial residue classes" | `Decoded.md` Algo 20 | Open (unverified) | `278289 = 25299 × 11`, so `mod 11 = 0` is true. But the claim that Maya tracked arcseconds is speculative. Verify the arithmetic; do not mechanize the cultural claim. |
| O9 | Conjecture C-DRESDEN-1 (universal S_R instantiation across planetary systems) | `The Dresden Codex.md` line 389 | Open / Conjectural | Tested only against our solar system. Cannot be mechanized without exoplanetary data. |
| O10 | Conjecture: "Saturn's 11² is structurally forced, not an accident specific to T_E = 11,960" | OP-DRESDEN-2 | Open | Worth testing computationally: try varying T_E to find other 11²-carrying displacements. |
| O11 | "Maya understood U(1) gauge symmetry from S¹ dimension" | `Maya Science sr-0001.md` Part I | Speculative | Anachronism. U(1) gauge is a 20th-century concept. The cyclic phase structure is real; the U(1) framing is interpretive. |
| O12 | "Entropy harvesting via phase gates extracts work — Maxwell's demon" | `Maya Science sr-0001.md` Part VI | Speculative | The agricultural-timing-improves-yield observation is real; the Maxwell's demon framing is overclaim. |
| O13 | Page 24 is iconographically blank (vault) vs. assigned RainGlyph (Rust code) | `Dresden Coprime.md` line 39; `iconographic.rs:30` | **Likely vault is correct.** Fix Rust. | See §6.3. |
| O14 | "Maya did not need continuous observation; mathematics guaranteed coherence" | repeated in `Dresden.md`, `Decoded.md` | Interpretive | Once epoch-initialized, modular arithmetic propagates exactly. This is mathematically true. The claim that this is *how the Maya operated* is interpretive. |
| O15 | "32,000-year Serpent Numbers prove the codex does not use floating points" | `Decoded.md` Algo 7 | Interpretive | The deep-time Serpent Numbers exist. The claim they "prove" no floating point is post-hoc since the Maya had no floating point anyway. |

---

## 9. Cross-reference table

| Claim | Vault file | Section/page | In dccms code? Where? |
|---|---|---|---|
| Safe Basis {2,3,5,7,11,13} | A-10, Decoded, Dresden, The Dresden Codex | various | YES: `dresden_codex::SAFE_BASIS` |
| M_SAFE = 30,030 | Decoded, A-10 | Algo 5, 7, 10 | YES: `dresden_codex::M_SAFE` |
| Tzolk'in 260 = 2²×5×13 | A-10 §A.10.8; The Dresden Codex L2 | L2, T2 | YES (used throughout codex_decoder, atlas) |
| Haab 365 = 5×73 | The Dresden Codex L1 | L1 | YES (constant in codex_decoder) |
| Venus 584, phases [236,90,250,8] | Dresden Coprime §Venus | T6 | YES: `dresden_codex::VENUS_PHASES` |
| Calendar Round 18,980 | T1 | T1 | YES: `CALENDAR_ROUND` |
| Long Count 13×144,000 | T4 | T4 | YES: `BAKTUN` (partial) |
| Mars 780 = 2²×3×5×13 | Decoded Algo 6 | — | Implicit; **no named constant** |
| Jupiter 399 = 3×7×19 | The Dresden Codex A6 | — | **NOT in code** |
| Saturn 378 = 2×3³×7 | The Dresden Codex A6 | — | **NOT in code** |
| Mercury 116 = 4×29 | C3 (line 348) | — | **NOT in code** |
| 819-day cycle 3²×7×13 | T3, L3 | T3 | **NOT in code** as constant |
| 11,960 = 405 lunations = 46×260 | L4 | L4 | Used in `moon_goddess.rs`; **no named constant** in dresden_codex |
| 93-day correction = 3×31 | L5 | L5 | **NOT in code** |
| 33-year epoch ≈ 12,053 | L5 | T5 | **NOT in code** |
| Venus-Haab LCM 2,920 = 5×584 = 8×365 | T6 | T6 | **NOT in code** |
| Saturn displacement 242 = 2×11² | L8, T8 | T8 | Verified in `h3_extended.rs` H3-a; **no named constant** |
| Venus displacement 280 = 2³×5×7 | L10, T10 | T10 | **NOT in code** |
| Mars displacement 260 = T_tz | L9, T9 | T9 | Verified; **no named constant** |
| Eclipse 148/177 alternation | Dresden Coprime §Eclipse; Decoded Algo 2 | — | YES: `ECLIPSE_NEAR`, `ECLIPSE_FAR`, `moon_goddess.rs` |
| Moon Goddess 1448 = 49 lunations | — | — | YES: `MoonGoddessProfile` |
| Grand sync 37,960 | Decoded; T6 | — | YES: `VENUS_TABLE_DAYS` |
| K-Elim winding extraction | Decoded Algo 2 | — | YES: `h5_level::k_elim_level` |
| Shadow16 = (x%11)×(x%13) | — | — | YES: `substrate_roles`, `venus_kernel` |
| Eb / Keh at lane-11 zeros | — | — | YES: `h4_visual::dayname`, `month` |
| Prime 11 as universal coordinate | A-10 §A.10.6.1; T-SHADOW kill #115 (Dresden.md) | — | YES: H5 hypothesis closed |
| Ramanujan S_R = {5,7,11} | The Dresden Codex A3; Maya CRT twp Theorem I | — | **NOT in code** as named const |
| Prime gap doubling at boundary | Maya CRT twp §4 | Theorem II | **NOT in code** |
| Composite CRT congruence-lifting decision procedure | Maya CRT twp §5 | Theorem III | **NOT in code** |
| DPM-PRIME T1–T10 theorem stack | The Dresden Codex | T1–T10 | **NOT in code** structurally; arithmetic underlies |
| DKAM 4-tier mapping (Face 13 / Turbulent / Boundary / Exact) | The Dresden Codex §DKAM | — | Partial: `dkam_filter.rs` exists |
| 78-stride lane nullification → isolates {5,7,11} | Decoded Algo 6 | — | Partial: `nullified_lanes` is general |
| Mars-Venus shadow bond detector | Decoded Algo 4 | — | **NOT in code** |
| Page 8 jaguar arithmetic 13+8=21, κ=1 | A-10 §A.10.3.1 | — | **NOT in code** |
| Page 52a as K-Elim engine (red/black columns) | Decoded Algo 2; A-10 §A.10.3.2 | — | **NOT in code** as page-level fact |
| Codex topology — 13 sections, pages 1-74 | Dresden Coprime §page-by-page | — | Partial: only Goddess section is modeled |
| Hardy-Littlewood singular series detector | Decoded Algo 8 | — | **NOT in code** |
| Range-aliasing on M = 30,030 | Decoded Algo 10 | — | Implicit |
| Lunar nodal 6,793-day cycle | OP-DRESDEN-1 | — | **NOT in code** |
| Universal S_R-instantiation conjecture | C-DRESDEN-1 | — | Open |
| Mayas Engine operator-fabric API | Mayas Engine.md | — | Partial: scattered |
| Pisano π(5) = 20 = vigesimal base | Mayas Engine.md | — | **NOT in code** as named property |
| Cylindrical time Lorentzian metric | Maya Science sr-0001 Part I | — | Out of scope (Open / speculative) |
| φ³ consciousness threshold | ASTRO21 docs | — | Out of scope (Open / speculative) |
| Turtle shell eclipse-period reading | Maya Science sr-0001 Part V | — | Out of scope (interpretive) |
| Moon Goddess as gynecological oracle | Dresden.md audit; Decoded Algo 17 | — | Out of scope (interpretive) |
| Maya FHE | Decoded.md §FHE | — | Out of scope (overclaim) |

---

## 10. Proposed v0.8.0+ build priorities

Ranking dimensions: **(S)upport** = how well the vault grounds the claim arithmetically; **(M)echanizability** = how cleanly it maps to Rust; **(E)xtension** = how much it extends what's already proven.

### Tier 1 — Build first (S/M/E all high)

#### B-1: `dresden_codex::periods` constants module
- **S:** Proven (every constant has arithmetic backing)
- **M:** Trivial — `pub const` declarations + `native_decide`-style tests
- **E:** Foundational for B-2..B-4
- **Effort:** XS (1–2 hours)
- **Files:** new `dresden_codex/src/periods.rs`
- **Adds:** JUPITER_SYNODIC=399, SATURN_SYNODIC=378, MERCURY_SYNODIC=116, MARS_SYNODIC=780, CYCLE_819=819, ECLIPSE_TABLE_DAYS=11_960, ECLIPSE_CORRECTION=93, EPOCH_33_YEAR=12_053, VENUS_HAAB_LCM=2_920, LONG_COUNT_13_BAKTUN=1_872_000, LUNAR_NODAL=6_793, RAMANUJAN_S_R=[5,7,11], plus per-period CRAM signature tests.

#### B-2: `dresden_codex::sr_distribution` — S_R Distribution Theorem (T10)
- **S:** Proven (T10 entirely arithmetic)
- **M:** Direct (residue + factorization + classification)
- **E:** Closes H3-a Saturn-11² as a *theorem*, not just a hypothesis
- **Effort:** S (3–5 hours)
- **Files:** new `dresden_codex/src/sr_distribution.rs`
- **Adds:** `fn planetary_displacement(period, epoch) -> Displacement`; `fn sr_content(displacement) -> Vec<u64>`; `fn t10_distribution_table()` returning the full Mars/Venus/Saturn/Jupiter/Mercury/lunar-nodal table; tests assert all of T7, T8, T9, T10 entries.

#### B-3: `dccms_atlas::dpm_prime` — DPM-PRIME theorem stack T1–T10
- **S:** Proven (10 theorems, 14 validation identities, all `native_decide`)
- **M:** Direct (each theorem → one test function returning a `Theorem::Pass(witness)` enum)
- **E:** Gives the workspace a published-paper-equivalent backbone
- **Effort:** M (1–2 days)
- **Files:** new `dccms_atlas/src/dpm_prime.rs` with submodules `axioms`, `lemmas`, `theorems`, `validation`
- **Adds:** the 10 theorems as `pub fn theorem_t1() -> TheoremResult` etc., one test per V1–V14, doc-comments citing `The Dresden Codex.md`.

### Tier 2 — Build next (high value, moderate cost)

#### B-4: Page 24 RainGlyph fix + codex_topology enum
- **S:** Vault contradicts code (page 24 is blank per Dresden Coprime line 39)
- **M:** Trivial fix + small extension
- **E:** Sets up future expansion beyond the Goddess section
- **Effort:** XS
- **Files:** edit `dccms_atlas/src/h4_visual/iconographic.rs`; new `dccms_atlas/src/codex_topology.rs`
- **Adds:** remove or rename `RainGlyph` on page 24; add `CodexSection` enum with `Introduction`, `MoonGoddess`, `BlankBridge`, `NewYear`, `Chaak`, `Venus`, `Eclipse`, `Mars78`, `Katun`, `Serpent`, `Deluge`; add `page_to_section(page: u8) -> CodexSection`.

#### B-5: `dresden_codex::shadow_bond` detector (Decoded Algo 4)
- **S:** Proven (algorithm is exact integer)
- **M:** Trivial port from the vault's Python
- **E:** Provides the named API for the Saturn-11² and other 11²-carrying alignments
- **Effort:** S
- **Adds:** `fn detect_shadow_bond(period, epoch, anchor_prime=11) -> ShadowBond` returning `None | Standard(prime) | Deep(prime, power)`.

#### B-6: `prime_hunt::ramanujan_partition` boundary
- **S:** Proven (Maya CRT twp 4 theorem stacks)
- **M:** Moderate — requires exact `p(n)` partition function (BigInt for n > ~400)
- **E:** Links partition theory to calendar choice; supports T3 and the 819-day analysis
- **Effort:** M
- **Adds:** `fn first_order_congruence(ell: u64) -> Option<Congruence>` returning Some(δ) for ℓ ∈ {5,7,11}, None otherwise; `fn composite_supports_congruence(modulus: u64) -> bool`; tests for {5,7,11} positive, {13, 23, 73} negative, composite {819, 65, 91, etc.}.

#### B-7: `dccms_atlas::engines` — operator-fabric refactor (Mayas Engine.md)
- **S:** Proven (the abstraction is sound; the constituent pieces already work in dccms)
- **M:** Mostly refactor of existing scattered code
- **E:** Structural cleanup; adds `pisano_profile`, `harmonic_signature` as genuinely new APIs
- **Effort:** M–L (refactor risk)
- **Adds:** `Lane`, `MayaState` types; five engine constructors (vigesimal, tzolkin, long_count, dresden, venus); Pisano/Ramanujan signature methods.

### Tier 3 — Build later (supplementary)

#### B-8: `dccms_atlas::dkam` — full tier mapping with Fibonacci index
- **S:** Proven (DKAM window section of The Dresden Codex.md)
- **M:** Moderate (`fibonacci_index_prime` + `Tier` enum)
- **E:** Augments existing `dkam_filter.rs`
- **Effort:** S
- **Adds:** `enum Tier { Face13, Turbulent, Boundary, Exact }`; `fn tier_of(prime) -> Tier`; `fn dkam_window(basis) -> (rho, max_deg)`.

#### B-9: `dccms_atlas::page_arithmetic` — per-page worked examples
- **S:** Proven (vault gives Page 8 jaguar and Page 52a explicitly)
- **M:** Direct (one fn per page entry)
- **E:** Builds out the page-level mathematical commentary
- **Effort:** S
- **Adds:** Page 8 jaguar `13 + 8 = 21; κ = 1`; Page 52a red/black columns `κ = 6` via K-Elim on lane 13.

#### B-10: Maya-date API (`from_maya_date`, `to_maya_date`, `advance_maya_days`)
- **S:** Engineering convenience (lifted from ASTRO21 PRACTICAL_MAYA_IMPLEMENTATION.md)
- **M:** Direct
- **E:** Improves ergonomics; no new mathematical claims
- **Effort:** XS
- **Adds:** methods on `MayaState` or a new `MayaDate` type.

### Tier 4 — Investigate before mechanizing

#### B-11: Empirical verification of the "12:1 Gini stratification" claim
- **Status:** Open / Measured (vault claim un-verified)
- **Action:** before mechanizing, compute the actual max/min density ratio across 30,030 CRAM addresses using existing dccms `Gini` infrastructure. **If the ratio is < 12:1, document and do not mechanize.**

#### B-12: Goddess section extension to pages 13c–15
- **Status:** Open (vault suggests this; Rust code starts at 16)
- **Action:** examine Barnhart 2005 reference or the SLUB facsimile to determine if pages 13c–15 contain Moon Goddess almanacs that should be part of the dccms target.

### Out-of-scope / do NOT mechanize

- Cylindrical-time Lorentzian metric (O3)
- φ³ consciousness threshold (O4)
- Turtle-shell eclipse reading (O5)
- Moon Goddess gynecological-oracle interpretation (O6)
- Maya FHE (O2)
- U(1) gauge framing (O11)
- Entropy harvesting / Maxwell's demon (O12)

These are interpretive overlays. The HackFate compendium's own discipline (CLAUDE.md in the vault) requires Open status for unproven claims and reserves "approximation" for classical limits — none of these survive that bar.

---

## Appendix A: Tabular index of vault files surveyed

| File | Path | Read depth | Synthesis tier |
|---|---|---|---|
| Decoded.md | `imports/github/HackFate/Decoded.md` (38 KB) | Full | P1 — 22 named algorithms |
| The Dresden Codex.md | `imports/github/HackFate/The Dresden Codex.md` (76 KB) | First 250 lines fully + grep | P1 — DPM-PRIME theorem stack |
| A-10_dresden_codex.md | `imports/github/HackFate/A-10_dresden_codex.md` (10 KB) | Full | P1 — addendum, page 8 + red-barrier arithmetic |
| Dresden Coprime.md | `imports/github/HackFate/Dresden Coprime.md` (24 KB) | Full | P1 — physical / historical / page-by-page topology |
| Dresden.md | `imports/github/HackFate/Dresden.md` (21 KB) | Full | P1 — CRAM-torus decoding + Moon Goddess audit + 64 hexagrams (off-topic but cross-domain) |
| Maya CRT milestone dd-0001.md | `imports/github/HackFate/Maya CRT milestone dd-0001.md` (39 KB) | First 100 lines | P1 — **mis-categorized; not Maya-specific** |
| Maya CRT twp-0001.md | `imports/github/HackFate/Maya CRT twp-0001.md` (77 KB) | First 100 lines + heading grep | P2 — Ramanujan partition formal paper |
| Maya CRT sr-0001.md | `imports/github/HackFate/Maya CRT sr-0001.md` (34 KB) | Heading grep | P2 — session report companion to twp |
| Maya Science sr-0001.md | `imports/github/HackFate/Maya Science sr-0001.md` (37 KB) | Sampled Parts I, V | P2 — speculative overlay on QMNF |
| Mayas Engine.md | `imports/github/HackFate/Mayas Engine.md` (35 KB) | First 100 lines | P2 — Rust-pseudocode operator-fabric API |
| Maya CRT Testing.md | `imports/github/HackFate/Maya CRT Testing.md` (91 KB) | Grep | P2 — generic test suite, not Maya-specific |
| A-07_dual_codex_dcbigint.md | `imports/github/HackFate/A-07_dual_codex_dcbigint.md` (10 KB) | Skipped (not Dresden-specific) | P2 |
| ASTRO21/MAYA_SCIENCE_COMPLETE_PICTURE.md | `imports/github/HackFate/ASTRO21/...` (37 KB) | First 200 lines | P3 — duplicate of Maya Science sr-0001 |
| ASTRO21/MAYA_RECONSTRUCTION_COMPLETE.md | `imports/github/HackFate/ASTRO21/...` (36 KB) | Heading grep | P3 — ASTRO21 integration narrative |
| ASTRO21/PRACTICAL_MAYA_IMPLEMENTATION.md | `imports/github/HackFate/ASTRO21/...` (17 KB) | First 240 lines | P3 — CRTBigInt Maya-date API |
| ASTRO21/THE_COMPLETE_MAYA_RECONSTRUCTION.md | `imports/github/HackFate/ASTRO21/...` (14 KB) | Heading grep | P3 — README-style index |

Priority 4 (`HackFate/Enhance CRAM edition.md`, `HackFate/Operator.md`) was not opened — budget allocated to higher-value Priority 1 and 2 sources.

---

## Appendix B: Concrete numeric checks performed by this synthesis agent

These are arithmetic checks I performed inline (not just lifted from the vault) — flagged so the orchestrator can re-verify:

1. **Lunar nodal cycle modulo 11** (closes OP-DRESDEN-1 partially): `6793 = 617 × 11 + 6`, so `6793 mod 11 = 6 ≠ 0`. The lunar nodal cycle does **not** restore the 11-channel either. Confirms the vault's "astronomically missing" claim at one more period.

2. **Lunar nodal full Safe Basis residues**: `(6793 mod 2, mod 3, mod 5, mod 7, mod 11, mod 13) = (1, 1, 3, 3, 6, 7)`. None of the residues is zero — full activation, but no S_R closure. Add this row to the §3.4 table.

3. **Confirmation of T10 Saturn**: `11960 / 378 = 31.6402...`, `31 × 378 = 11718`, `11960 − 11718 = 242 = 2 × 121 = 2 × 11²`. Verified.

4. **Confirmation of T10 Venus**: `11960 / 584 = 20.4794...`, `20 × 584 = 11680`, `11960 − 11680 = 280 = 8 × 35 = 2³ × 5 × 7`. Verified — both 5 and 7 (the accessible S_R) appear in Venus's residue.

5. **Confirmation of T10 Mars**: `11960 / 780 = 15.333...`, `15 × 780 = 11700`, `11960 − 11700 = 260 = T_tz`. Verified.

6. **Confirmation Jupiter residue**: `11960 / 399 = 29.97...`, `29 × 399 = 11571`, `11960 − 11571 = 389`. `389 = ?`. `389` is prime (not 2, 3, 5, 7, 11, 13, 17, 19, 23 divisor; 19² = 361, 23² = 529; checked: 389 / 13 = 29.92..., /17 = 22.88..., /19 = 20.47...; so 389 is prime). **Jupiter's 33-year displacement = 389, a prime outside the entire Safe Basis and S_R.** This is mathematically interesting and not commented on in the vault — Jupiter carries *no* S_R prime, *no* Safe Basis prime, in its displacement. The T10 Mars/Venus/Saturn triple is therefore the "complete" S_R recovery exactly because Jupiter and Mercury do not contribute.

7. **Confirmation Mercury residue**: `11960 / 116 = 103.103...`, `103 × 116 = 11948`, `11960 − 11948 = 12 = 2² × 3`. Mercury carries only the parking-lane 2 and stability-floor 3 — no S_R content. Again, this excludes it from T10's main result.

8. **Page 8 jaguar arithmetic** (independently re-verified per A-10): `13 + 8 = 21`. `21 / 7 = 3` exactly, so `21 mod 7 = 0`. `6⁻¹ mod 7`: `6 × 6 = 36 = 5 × 7 + 1`, so `6⁻¹ = 6`. Verified. The κ = 1 step depends on the specific K-Elim formulation the vault uses; not re-derived here.

9. **278289 mod 11 check** (Algo 20 spatial arcseconds): `278289 / 11 = 25299`, `25299 × 11 = 278289`. So `278289 mod 11 = 0` exactly. Arithmetic claim verified; cultural claim (that Maya tracked arcseconds) remains Open.

10. **819 mod 11 verification** (Lemma L7 / Theorem T7): `819 / 11 = 74.45...`, `74 × 11 = 814`, `819 − 814 = 5`. So `819 mod 11 = 5 ≠ 0`. Verified — 11 is absent from 819.

---

*End of synthesis. The vault contains roughly 200 KB of grounded arithmetic that directly extends what dccms has, plus another 150 KB of speculative overlay that should not be mechanized. The highest-leverage v0.8.0 candidate is the DPM-PRIME theorem stack: 10 theorems, all `native_decide`-equivalent, that turn the workspace into a formal artifact rather than a hypothesis register.*
