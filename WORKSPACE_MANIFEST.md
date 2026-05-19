# DCCMS Workspace Manifest

**Version:** 0.8.0-dev (Tiers 1, 2, 3 ALL ENTIRELY COMPLETE)
**Date:** 2026-05-18
**Tests:** 588 passing, 0 failing (481 dccms_atlas + 85 dresden_codex + 22 prime_hunt)
**Architecture:** 3-crate Rust workspace, exact-integer (zero float, `#![deny(clippy::float_arithmetic)]` enforced in dccms_atlas/src/lib.rs)

This document is the ground truth of what exists. Read it before creating any new module.

---

## Crates

| Crate | Status | Tests | Purpose |
|---|---|---|---|
| `dresden_codex` | PRODUCTION | 24 | L1 substrate — CRAM primitives, Safe Basis, phase schemas |
| `dccms_atlas` | PRODUCTION (v0.6.0) | 262 | Query layer — Hydra, atlases, hypothesis instruments, decoders |
| `prime_hunt` | PRODUCTION | 2 | Prime sieve + factorization helpers |

---

## L1 substrate — `dresden_codex` (24 tests)

Exported constants:
- `SAFE_BASIS: [u64; 6] = [2, 3, 5, 7, 11, 13]`
- `M_SAFE: u64 = 30_030`
- `VENUS_SYNODIC = 584`, `VENUS_TABLE_DAYS = 37_960`
- `ECLIPSE_NEAR = 148`, `ECLIPSE_FAR = 177`
- `VENUS_PHASES = [236, 90, 250, 8]`, `CALENDAR_ROUND = 18_980`, `BAKTUN = 144_000`

Exported functions (canonical CRAM API — DO NOT REIMPLEMENT):
- `cram_address(x: u64) -> [u64; 6]` — integer → residue address
- `nullified_lanes(x: u64) -> Vec<u64>` — primes where x ≡ 0
- `active_lanes(x: u64) -> Vec<u64>` — primes where x ≢ 0
- `carry_bits(x: u64) -> [u8; 6]` — per-lane carry indicators
- `pack_carry_bits(&[u8; 6]) -> u8` — bitpacked signature

Exported types:
- `Phase`, `MultiPhaseSchema`
- Schema builders: `tzolkin_schema`, `haab_schema`, `calendar_round_schema`, `long_count_schema`

---

## dccms_atlas modules (25 modules)

### v0.1.0 — base layer
- `heads` — four-calendar Hydra (Tzolk'in / Haab / Venus / Lunar)
- `events` — 2,967-event canonical corpus (`EventSet`, `CodexEvent`, `CodexEventKind`)
- `atlas` — ConfigAtlas, Gini coefficient
- `recumbent` — winding state with multi-modulus indexing
- `adelic_index` — dual-track index
- `h3_mi` — mutual information (natural basis points)
- `h4_instruments` — six typed Goddess-section instruments + `run_h4_suite`
- `h5_navigator` — prime-11 distributor
- `dkam_filter` — admissibility filter
- `manifold_geometry` — Hamming-1 components

### v0.2.0
- `h1_generator` — generator extraction
- `h5_refined` — triple K-Elimination
- `h4_non_visual` — **4 Goddess instruments + `GODDESS_SECTION_INTERVALS = [148,177,148,177,148,177,148,177,148]`**

### v0.3.0
- `h1_stage8` — bilinear generator (SEED=20)
- `h5_level` — K-Elim level theorem, `k_elim_level(x, p)`
- `cross_validation` — independent corpora
- `h4_montgomery` — Montgomery shadow, `MontgomeryQuad`, `goddess_section_quads`

### v0.4.0
- `h3_extended` — 163-candidate family
- `h5_katun` — Katun partition
- `manifold_upgrade` — Hamming-2 bridges
- `generator_catalog` — 86-entry catalog

### v0.5.0 — Dresden Codex decoder
- `lunar` — synodic arithmetic, `MOON_GODDESS_DAYS`, `TZOLKIN_DAYS`, etc.
- `moon_goddess` — **Moon Goddess Section full decoder** (`MoonGoddessProfile::compute`)
- `codex_decoder` — Venus Table + Eclipse Table + Binding Theorem

### v0.6.0 — CRAM-ENHANCE
- `venus_kernel` — Shadow16, fifth-operator rhythm, heterogeneous carry vector
- `substrate_roles` — prime role taxonomy (parity / fabric / content / traversal / coordinate / boundary)

### v0.7.0-dev — H4 visual transducer (NEW)
- `h4_visual` — module index
- `h4_visual::alphabet` — `GlyphAlphabet<const K>` trait + `SemanticRole` enum + `safe_address` fn
- `h4_visual::bardot` — `BarDotNumeral` (Maya base-20 numerals 0..=19)
- `h4_visual::dayname` — `DayNameGlyph` (20 Tzolk'in day names) + `LANE_11_ZERO_DAY = Eb`
- `h4_visual::month` — `MonthGlyph` (18 Haab months + Wayeb)
- `h4_visual::iconographic` — `IconographicFigure` (9 Goddess-page figures) with `associated_interval()`
- `h4_visual::layout` — `PageLayout` + `goddess_section_layout()` + `cumulative_addresses()` + `cumulative_totals()`
- `h4_visual::lift` — `lift<const K>(x, basis)` + `SAFE_BASIS_K7`/`K8`/`K10` + `basis_product`
- `h4_visual::wire` — `verify_against_non_visual_pipeline()` + `page_intervals` + `residue_pattern_from_layout`
- `h4_visual::contracts` (test-only) — CTR01..CTR05 Five-Contract gates
- `h4_visual::fifth_op` (test-only) — FO01..FO02 fifth-operator coherence

### Project constants (dccms_atlas/src/lib.rs)
- `SAFE_BASIS`, `M_SAFE` (re-exported)
- `SHADOW_PRIME = 11`, `BOUNDARY_PRIME = 13`
- `TRANSPORT_CORE = [3, 7, 11, 13]`, `RAMANUJAN_PRIMES = [5, 7, 11]`
- `VENUS_CONDUCTOR_MODULI = [260, 365, 584, 2920, 37960]`
- `RHO_TRANSPORT = 3`, `DKAM_MAX_DEGREE = 2`
- `H3_THRESHOLD_BP = 1000`, `H4_SUPPORT_THRESHOLD = 3`, `H4_INSTRUMENT_COUNT = 6`

---

## Hypothesis status

| H | Status | Where in code |
|---|---|---|
| H1 | SUPPORTED | `h1_generator`, `h1_stage8` |
| H2 | SUPPORTED | `heads`, `atlas` |
| H3-{a,b,c,ext} | SUPPORTED | `h3_extended`, `h3_mi` |
| **H4** | **SUPPORTED** (v0.7.0-dev) | `h4_instruments`, `h4_non_visual`, `h4_montgomery`, `h4_visual` (full visual side: glyph alphabets, layout, lift, wire, contracts, fifth-op) |
| H5 | SUPPORTED | `h5_navigator`, `h5_refined`, `h5_level`, `h5_katun` |
| Binding (Venus/Eclipse = Tzolk'in carry class) | THEOREM | `codex_decoder` |

---

## Examples (8)

| Example | Version | Purpose |
|---|---|---|
| `findings_query` | v0.1.0 | baseline atlas query |
| `first_query` | v0.1.0 | minimal smoke |
| `independent_test` | v0.1.0 | corpus-independent run |
| `open_items_v020` | v0.2.0 | κ₃ bifurcation |
| `complete_findings_v030` | v0.3.0 | generator + level |
| `complete_findings_v040` | v0.4.0 | extended catalog |
| `codex_decoder` | v0.5.0 | Venus/Eclipse/Binding |
| `cram_enhance_decoder` | v0.6.0 | Shadow16 + carry-vector + role taxonomy |

---

## Invariants enforced by the workspace

- **A1 ZERO FLOAT** — `#![deny(clippy::float_arithmetic)]` in `dccms_atlas/src/lib.rs`
- **NO UNSAFE** — `#![forbid(unsafe_code)]` in `dccms_atlas/src/lib.rs`
- **Safe Basis** — every CRAM operation uses `dresden_codex::SAFE_BASIS`; no ad-hoc residue moduli
- **Prime 11 universal** — verified active in every codex section (H5 theorem)

---

## Open infrastructure (not yet present)

These are the natural extension points. They do NOT exist yet:

- **Pixel ingestion (SEG01–SEG03)** — page-image → glyph-ID pipeline.
  Off the H4 critical path by Object-contract firewall. Imagery survey
  complete (`docs/imagery_sources.md`); SLUB Dresden public-domain
  color photos are the primary source. Segmenter design deferred.
- **Image dependencies** — no `image` / `imageproc` / `tiff` / `png` / `jpeg` crate in any Cargo.toml.
- **Codex page imagery on disk** — no SLUB/Förstemann scans downloaded yet.
- **Visual entropy ingestion** — `visual_entropy_by_winding_depth` exists but takes `&[u64]` events, not pixels.
