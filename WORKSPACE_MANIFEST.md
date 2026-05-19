# DCCMS Workspace Manifest

**Version:** 0.9.2-dev — customize-fully calibration
**Date:** 2026-05-19
**Tests:** 648 passing, 0 failing (532 `dccms_atlas` + 94 `dresden_codex` + 22 `prime_hunt`)
**Architecture:** 3-crate Rust workspace, exact-integer (zero float, `#![deny(clippy::float_arithmetic)]` enforced in `dccms_atlas/src/lib.rs`)

This document is the ground truth of what exists. Read it before creating any new module.

---

## Crates

| Crate | Status | Tests | Purpose |
|---|---|---|---|
| `dresden_codex` | PRODUCTION | 94 | L1 substrate — CRAM primitives, Safe Basis, K-Elim divide, shadow_bond, S_R distribution |
| `dccms_atlas` | PRODUCTION (v0.9.2-dev) | 532 | Query layer — Hydra, atlas, hypothesis instruments, decoders, segmenter |
| `prime_hunt` | PRODUCTION | 22 | Prime sieves, Ramanujan partition, prime-gap analysis |

---

## L1 substrate — `dresden_codex` (94 tests)

### `lib.rs`

Exported constants:
- `SAFE_BASIS: [u64; 6] = [2, 3, 5, 7, 11, 13]`
- `M_SAFE: u64 = 30_030`
- `TZOLKIN_PERIOD = 260`, `HAAB_PERIOD = 365`
- `VENUS_SYNODIC = 584`, `VENUS_TABLE_DAYS = 37_960`
- `MARS_SYNODIC = 780`, `JUPITER_SYNODIC = 399`, `SATURN_SYNODIC = 378`, `MERCURY_SYNODIC = 116`
- `ECLIPSE_NEAR = 148`, `ECLIPSE_FAR = 177`, `ECLIPSE_TABLE_DAYS = 11_960`, `ECLIPSE_CORRECTION = 93`
- `EPOCH_33_YEAR = 12_053`, `VENUS_HAAB_LCM = 2_920`
- `CALENDAR_ROUND = 18_980`, `BAKTUN = 144_000`, `LONG_COUNT_13_BAKTUN = 1_872_000`
- `LUNAR_NODAL = 6_793`, `CYCLE_819`
- `VENUS_PHASES = [236, 90, 250, 8]`
- `RAMANUJAN_S_R: [u64; 3] = [5, 7, 11]`

Exported functions (canonical CRAM API — DO NOT REIMPLEMENT):
- `cram_address(x: u64) -> [u64; 6]` — integer → residue address
- `nullified_lanes(x: u64) -> Vec<u64>` — primes where x ≡ 0
- `active_lanes(x: u64) -> Vec<u64>` — primes where x ≢ 0
- `carry_bits(x: u64) -> [u8; 6]` — per-lane carry indicators
- `pack_carry_bits(&[u8; 6]) -> u8` — bitpacked signature

Exported types:
- `Phase`, `MultiPhaseSchema`
- Schema builders: `tzolkin_schema`, `haab_schema`, `calendar_round_schema`, `long_count_schema`

### `k_elim_divide.rs`

K-Elimination division (phase-differential) + navigation-level certificate.

### `shadow_bond.rs`

T10-style content view, anchor-in-period detector, planetary shadow-bond enum.

### `sr_distribution.rs`

`PlanetaryDisplacement` table (Mars, Venus, Saturn, Jupiter, Mercury) + `t10_s_r_union` predicate.

---

## `prime_hunt` (22 tests)

| Module | Provides |
|---|---|
| `lib.rs` | basic prime sieve interfaces |
| `ramanujan_partition.rs` | Discovery 1 (Ramanujan congruence content) + Discovery 3 (S_R = {5, 7, 11}) |
| `prime_gap_analysis.rs` | Discovery 5 (prime-gap doubling near the Safe Basis) |

---

## `dccms_atlas` modules — by version arc

### v0.1.0 — base layer
- `heads` — four-calendar Hydra (Tzolk'in / Haab / Venus / Lunar)
- `events` — 2,967-event canonical corpus (`EventSet`, `CodexEvent`, `CodexEventKind`)
- `atlas` — `ConfigAtlas`, Gini coefficient
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
- `h4_non_visual` — 4 Goddess instruments + `GODDESS_SECTION_INTERVALS = [148,177,148,177,148,177,148,177,148]`

### v0.3.0
- `h1_stage8` — bilinear generator (SEED = 20)
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
- `moon_goddess` — Moon Goddess Section full decoder (`MoonGoddessProfile::compute`)
- `codex_decoder` — Venus Table + Eclipse Table + Binding Theorem

### v0.6.0 — CRAM-ENHANCE
- `venus_kernel` — Shadow16, fifth-operator rhythm, heterogeneous carry vector
- `substrate_roles` — prime role taxonomy (parity / fabric / content / traversal / coordinate / boundary)

### v0.7.0 — H4 visual transducer
- `h4_visual/alphabet` — `GlyphAlphabet<const K>` trait + `SemanticRole` enum + `safe_address` fn
- `h4_visual/bardot` — `BarDotNumeral` (Maya base-20 numerals 0..=19)
- `h4_visual/dayname` — `DayNameGlyph` (20 Tzolk'in days) + `LANE_11_ZERO_DAY = Eb`
- `h4_visual/month` — `MonthGlyph` (18 Haab months + Wayeb)
- `h4_visual/iconographic` — `IconographicFigure` (9 Goddess-page figures) with `associated_interval()`
- `h4_visual/layout` — `PageLayout` + `goddess_section_layout()` + `cumulative_addresses()` + `cumulative_totals()`
- `h4_visual/lift` — `lift<const K>(x, basis)` + `SAFE_BASIS_K7`/`K8`/`K10` + `basis_product`
- `h4_visual/wire` — `verify_against_non_visual_pipeline()` + `page_intervals` + `residue_pattern_from_layout`
- `h4_visual/contracts` (test-only) — CTR01..CTR05 Five-Contract gates
- `h4_visual/fifth_op` (test-only) — FO01..FO02 fifth-operator coherence

### v0.8.0 — DPM-PRIME + operator fabric
- `codex_topology` — codex-topology framework
- `dpm_prime` — DPM-PRIME 10-theorem certificate suite (T1–T10, V14_strict precision hardening)
- `engines/` (11 files) — `Lane`, `MayaState`, `Vigesimal`, `Tzolkin`, `LongCount`, `DresdenEclipse`, `VenusTable`, `MayaFabric`, `Pisano`, `Ramanujan`, `Recombinant`
- `ramanujan_alignment` — Ramanujan-alignment instruments
- `dkam_tier` — DKAM tier mapping
- `page_arithmetic` — page-level arithmetic
- `maya_date` — Maya date utilities
- `gini_stratification` — Gini stratification of the event corpus
- `goddess_extension`, `extended_goddess` — glyph specs for pages 13c–15

### v0.9.0 — segmenter framework + SLUB ingestion
- `segmenter/mod` — `ImageBuffer`, `BoundingBox`, `Segmenter` trait, `GlyphClassifier` trait (discrete typed contracts at all API boundaries)
- `segmenter/null` — `NullSegmenter`, `NullClassifier` for tests
- `segmenter/slub` — SLUB JPEG adapter (`load_slub_page`) — feature-gated under `slub`
- `segmenter/threshold` — `DarknessThresholdSegmenter` — pixel-sum threshold + flood-fill connected components, 4-connectivity

### v0.9.1 — closing seg + register-aware + cross-source comparison
- `segmenter/closing` — `ClosingThresholdSegmenter` — morphological closing (separable dilate→erode) before CC; eliminates register-leak under-segmentation
- `segmenter/register` — `RegisterAwareSegmenter<S>` — detects red horizontal barriers, partitions into bands, runs inner segmenter per band
- `segmenter/classify` — `PageContextClassifier` (page → expected `IconographicFigure`, honest minimal — bbox-content-blind) + `verify_page_iconography`
- `segmenter/comparison` — SLUB ↔ FAMSI cross-source `compare_two_jpegs`, blank-like heuristic (later replaced in v0.9.2)

### v0.9.2 — empirical calibration + vault-driven verification (CURRENT)
- `segmenter/comparison` — `WWII_DAMAGED_PAGES = [2, 4, 24, 28, 34, 38, 71, 72]`, `slub_signals_damage = stats AND zero barriers`, `segmenter_corroborates_vault` field on `PageComparison`
- `segmenter/register` — `RegisterAwareSegmenter::new()` defaults bumped from synthetic (130, 40, 500‰) to empirical (164, 24, 200‰) from `calibrate.rs` Job 1
- `paths` — hard-coded data paths for HackFate's machine (`slub_page`, `famsi_page`, `famsi_pdf`, `famsi_extracted_dir`, `FAMSI_PAGE_RANGE = 13..=24`)

### Project constants (`dccms_atlas/src/lib.rs`)

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
| **H4** | **SUPPORTED** (v0.7 visual transducer closes diagram; v0.9 segmenter exists; **real-pixel closure pending v0.9.3 IconographicGlyphClassifier**) | `h4_instruments`, `h4_non_visual`, `h4_montgomery`, `h4_visual`, `segmenter` |
| H5 | SUPPORTED | `h5_navigator`, `h5_refined`, `h5_level`, `h5_katun` |
| Binding (Venus/Eclipse = Tzolk'in carry class) | THEOREM | `codex_decoder` |

---

## Examples (14 runnable)

| Example | Version | Feature | Purpose |
|---|---|---|---|
| `findings_query` | v0.1.0 | — | baseline atlas query |
| `first_query` | v0.1.0 | — | minimal smoke |
| `independent_test` | v0.1.0 | — | corpus-independent run |
| `open_items_v020` | v0.2.0 | — | κ₃ bifurcation |
| `complete_findings_v030` | v0.3.0 | — | generator + level |
| `complete_findings_v040` | v0.4.0 | — | extended catalog |
| `codex_decoder` | v0.5.0 | — | Venus / Eclipse / Binding decoder |
| `cram_enhance_decoder` | v0.6.0 | — | Shadow16 + carry-vector + role taxonomy |
| `famsi_extract` | v0.9.0 | — | FAMSI PDF byte-scan → 12 JPEGs |
| `slub_segment` | v0.9.0 | `slub` | one-page segmentation |
| `slub_improved` | v0.9.1 | `slub` | closing + register-aware against real SLUB |
| `famsi_segment_all` | v0.9.0 | `slub` | sweep all 12 FAMSI pages |
| `compare_slub_famsi` | v0.9.2 | `slub` | cross-source verdict (12/12 corroboration) |
| `calibrate` | v0.9.2 | `slub` | 4-job empirical calibration (red, mapping, threshold, register) |

---

## Imagery on disk

`~/Agents/imports/` layout:

```
slub_dresden/   page_00000013.jpg … page_00000024.jpg     (3874 × 7649 RGB)
famsi_dresden/
├── famsi_pp13-24.pdf
└── extracted/  page_13.jpg … page_24.jpg                  (1552 × 3332 RGB)
```

12 SLUB pages + 12 FAMSI pages on disk. **Coverage gap:** vault-known WWII-damaged pages include 2, 4, 28, 34, 38, 71, 72 — no SLUB or FAMSI imagery for any of these in the current download. Extension to the rest of the codex is a v0.9.3+ candidate.

---

## Invariants enforced by the workspace

- **A1 ZERO FLOAT** — `#![deny(clippy::float_arithmetic)]` in `dccms_atlas/src/lib.rs`
- **NO UNSAFE** — `#![forbid(unsafe_code)]` in `dccms_atlas/src/lib.rs`
- **Object contract** — `Segmenter::segment` returns `Vec<BoundingBox>` (discrete integer coords); `GlyphClassifier::classify` returns `Option<Glyph>` (single value or `None`, never a confidence vector)
- **Safe Basis** — every CRAM operation uses `dresden_codex::SAFE_BASIS`; no ad-hoc residue moduli
- **Prime 11 universal** — verified active in every codex section (H5 theorem)
- **Vault first** — for known facts (WWII damage list, page → iconographic figure), the vault is the source of truth and the segmenter verifies, not discovers

---

## Open infrastructure — v0.9.3+ extension points

These are the natural next nodes. They do NOT exist yet:

- **`segmenter/pipeline`** — end-to-end page→bbox→glyph→atlas wire. **Load-bearing for H4 real-pixel closure.** See `executioner_dag.md` § "v0.9.3 — The Object diagram closes on real pixels."
- **Real `GlyphClassifier` implementations** — the only existing classifier is `PageContextClassifier`, which is bbox-content-blind. A real `IconographicGlyphClassifier` using bbox geometry (area, aspect, band position) + auxiliary pixel-darkness summary is the v0.9.3 critical-path build.
- **Extended SLUB imagery** — pages 1–12, 25–74 not downloaded. The IIIF service URL pattern is known; extension is mechanical.
- **Closing-segmenter pickup in cross-source comparison** — `compare_two_jpegs` currently uses `DarknessThresholdSegmenter`. Switching to `ClosingThresholdSegmenter` would reduce noise in cross-source corroboration.
- **Lean 4 / Coq formal proofs** for the DPM-PRIME theorem stack — orthogonal artifact; the Rust arithmetic-certificate suite is canonical for this workspace.

These are not deferred; they are the surface of `executioner_dag.md § v0.9.3`.
