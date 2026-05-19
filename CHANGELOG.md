# Changelog

All notable changes to **DCCMS — Dresden Codex Configuration Manifold Study**.

Format based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning is internal; the project is currently pre-1.0 and ships as `0.X.Y-dev`.

A1 (zero-float) and the Object contract are invariants — not version-scoped — and
hold from v0.1 forward. Every entry below preserves them.

---

## [Unreleased] — v0.9.3-dev

### Added
- `WORKSPACE_MANIFEST.md` refreshed to reflect v0.9.2 reality (was stale at v0.9.0).
- `docs/v0_9_3_PLAN.md` — forward plan declaring the v0.9.3 thesis (Object diagram closes on real pixels).
- `CHANGELOG.md` (this file).
- `executioner_dag.md` § "v0.9.3 — The Object diagram closes on real pixels" — gap analysis + 10-node DAG.

### Changed
- Cleaned 30+ unused-import warnings across `dccms_atlas`, `dresden_codex`, and four examples (`open_items_v020`, `cram_enhance_decoder`, `complete_findings_v030`, plus the comparison module's auto-fixed redundancies). `cargo check --workspace --features dccms_atlas/slub` now produces zero warnings. No behavior change. 648 tests still passing.

### Pending (v0.9.3 critical path)
- `segmenter::classify::IconographicGlyphClassifier` — first real bbox→glyph classifier.
- `segmenter::pipeline` — end-to-end page→bbox→glyph→atlas pipeline.
- `examples/decode_goddess.rs` — runs the pipeline on SLUB pages 13–24.
- Integration test asserting `pipeline.predicted_cram == MoonGoddessProfile::page_cram_addresses()`.

---

## [0.9.2-dev] — 2026-05-19 — Customize-fully calibration

The program is for our use only and can be customized fully. Empirical SLUB values replace synthetic defaults; the vault-known damage list replaces the stat-inferred heuristic.

### Added
- `WWII_DAMAGED_PAGES = [2, 4, 24, 28, 34, 38, 71, 72]` vault-known damage list in `segmenter::comparison`.
- `slub_signals_damage(components, max_area, barrier_rows)` combined damage signature — fixes the v0.9.1 page-18 false positive (page 18 has 52 barriers, page 24 has 0).
- `src/paths.rs` — hard-coded data paths (`slub_page`, `famsi_page`, `famsi_pdf`, `famsi_extracted_dir`, `FAMSI_PAGE_RANGE = 13..=24`) for HackFate's machine.
- `examples/calibrate.rs` — four-job calibration tool (red sampling, FAMSI mapping, threshold tuning, barrier verification).
- `docs/v0_9_2_findings.md`.

### Changed
- `RegisterAwareSegmenter::new()` defaults from synthetic `(red_min=130, red_excess=40, row_fraction_per_mille=500)` to empirical `(164, 24, 200)` — sampled at SLUB page 16 row 7200 (1068 red pixels): median (R, G, B) = (164, 140, 126). The barrier ink is brownish/orange, not crimson.
- `PageComparison` struct: replaced `is_blank_like` stat heuristic with `wwii_damaged_per_vault` (authoritative) + `slub_stats_look_damaged` (diagnostic) + `slub_signals_damage` (combined) + `segmenter_corroborates_vault` (the gate).
- All six examples now use the centralized `paths` module instead of copy-pasting `home()`/`slub_path()` helpers.
- Crate version `0.9.0-dev` → `0.9.2-dev`.

### Result
**12/12 SLUB pages 13-24 corroborate the vault. Zero disagreements.** Cross-source verdict in commit `e19a1f4`; README rewrite in commit `2498ef6`.

---

## [0.9.1-dev] — 2026-05-19 — Three-area improvements

### Added
- `ClosingThresholdSegmenter` — morphological closing (separable dilate→erode) before connected-components. Reduces page-16 components 5,668 → 2,622 (54% drop), eliminates register-leak under-segmentation.
- `RegisterAwareSegmenter<S>` — detects red horizontal barriers, partitions image into bands, runs the inner segmenter per band. Framework only at this point — defaults missed actual SLUB barriers (calibration in v0.9.2).
- `PageContextClassifier` + `verify_page_iconography()` — page → expected `IconographicFigure`, returns `VerificationReport`. Honest minimal: ignores bbox content (real classifier is v0.9.3 work).
- `segmenter::comparison::compare_two_jpegs` — first-pass SLUB↔FAMSI cross-source comparison.
- `docs/v0_9_1_findings.md`.

### Result
SLUB page 24 flagged as genuine WWII-damage candidate (aligns with vault list). SLUB page 18 flagged as false positive (visual inspection shows full Moon Goddess content) — driver for v0.9.2 calibration.

---

## [0.9.0-dev] — 2026-05-18 — SLUB Dresden ingestion + "no outside scope"

### Added
- `segmenter/mod.rs` — `ImageBuffer`, `BoundingBox`, `Segmenter` trait, `GlyphClassifier` trait. Discrete typed contracts at all API boundaries.
- `segmenter/slub.rs` — SLUB JPEG adapter (`load_slub_page`) via pure-Rust `image` crate (jpeg feature only). Feature-gated under `slub`.
- `segmenter/threshold.rs` — `DarknessThresholdSegmenter` with BFS flood-fill connected components, 4-connectivity.
- `segmenter/null.rs` — `NullSegmenter`, `NullClassifier` for tests.
- `examples/famsi_extract.rs` — pure-Rust PDF byte-scanner (no PDF tooling available). Locates JPEG SOI/EOI markers (`FF D8 FF` … `FF D9`) directly inside the FAMSI PDF and extracts the 12 embedded streams.
- `examples/slub_segment.rs`, `examples/famsi_segment_all.rs` — segmentation against real imagery.
- 12 SLUB Dresden pages 13–24 downloaded (~58 MB).
- 12 FAMSI Förstemann / Schele chromolithograph plates extracted (1552 × 3332 RGB).
- `docs/imagery_sources.md`, `docs/slub_segmentation_findings.md`, `docs/v0_9_0_PLAN.md`.

### Philosophy
Apparent limits become build targets. PDF tooling absent → built the byte-scanner. No image-format adapter → built one. The directive "there is no outside scope" drove conversion of every bracketed deferred item into a buildable target.

---

## [0.8.0-dev] — 2026-05-18 — DPM-PRIME + operator fabric (Tier 1–3)

### Added
- `dpm_prime` — DPM-PRIME 10-theorem arithmetic certificate suite (T1–T10). 9 theorems certified by direct integer computation; T4 (Long Count = covering space of Calendar Round) returns `Conditional` pending FSM-PRIME `M_Fib` covering-morphism construction. V14_strict precision-hardening predicate added.
- `engines/` — operator-fabric engines (`Lane`, `MayaState`, `Vigesimal`, `Tzolkin`, `LongCount`, `DresdenEclipse`, `VenusTable`, `MayaFabric`, `Pisano`, `Ramanujan`, `Recombinant`).
- `dresden_codex::shadow_bond` — typed `ShadowBond` enum with five variants. Algorithm 4 from vault `Decoded.md`.
- `dresden_codex::sr_distribution` — planetary-displacement table (Mars, Venus, Saturn, Jupiter, Mercury) + `t10_s_r_union` predicate.
- `prime_hunt::ramanujan_partition` — Discovery 1 + Discovery 3 (S_R = {5, 7, 11}). Möbius/totient formula `c_q(n) = μ(q/gcd(q,n)) · φ(q) / φ(q/gcd(q,n))`.
- `prime_hunt::prime_gap_analysis` — Discovery 5 (prime-gap doubling).
- `ramanujan_alignment`, `dkam_tier`, `page_arithmetic`, `maya_date`, `gini_stratification`, `goddess_extension`, `extended_goddess`.
- New `dresden_codex` constants: `JUPITER_SYNODIC = 399`, `SATURN_SYNODIC = 378`, `MERCURY_SYNODIC = 116`, `MARS_SYNODIC = 780`, `CYCLE_819`, `ECLIPSE_TABLE_DAYS = 11_960`, `ECLIPSE_CORRECTION = 93`, `EPOCH_33_YEAR = 12_053`, `VENUS_HAAB_LCM = 2_920`, `LONG_COUNT_13_BAKTUN = 1_872_000`, `LUNAR_NODAL = 6_793`, `RAMANUJAN_S_R = [5, 7, 11]`.

### Fixed (precision hardening)
- V14_strict — initial draft only encoded `lcm = 18980`, admitting T = 52. Hardened to T2's full conjunctive predicate (S_R prime ∧ 13|T ∧ lcm = 18980).
- Test for `ramanujan_prime_split` (B-7): case-split was inverted in test prose; correct formula is `c_p(n) = -1 if p∤n else p-1`. Implementation was always right.
- T4 decimal witness — "≈ 98.63" replaced with exact `1872000/18980 = 7200/73 = 98 + 46/73`, proven by `73 ∤ 7200`.

---

## [0.7.0-dev] — 2026-05-18 — H4 closed (visual transducer)

### Added
- `h4_visual/` (10 files, 50+ tests) — closes the H4 visual transducer diagram.
  - `alphabet.rs` — `GlyphAlphabet<const K>` trait, `SemanticRole` enum, `safe_address` function.
  - `bardot.rs` — `BarDotNumeral` (Maya base-20 numerals 0..=19).
  - `dayname.rs` — `DayNameGlyph` (20 Tzolk'in days) with `LANE_11_ZERO_DAY = Eb`.
  - `month.rs` — `MonthGlyph` (18 Haab months + Wayeb) with lane-11 zero at Keh.
  - `iconographic.rs` — `IconographicFigure` (9 Goddess pages) with `associated_interval()`.
  - `layout.rs` — `PageLayout` whose cumulative CRAM addresses match `MoonGoddessProfile::page_cram_addresses` exactly.
  - `lift.rs` — basis-parameterized `lift<const K>` at K = 6, 7, 8, 10. First-K lanes invariant on extension.
  - `wire.rs` — verification against the non-visual pipeline.
  - `contracts.rs` (test) — CTR01..CTR05 Five-Contract gates per ns-continuum-bridge discipline.
  - `fifth_op.rs` (test) — FO01..FO02 fifth-operator coherence.
- Two structural facts surfaced:
  - **Eb and Keh are lane-11 zeros** of the Tzolk'in / Haab cycles. Navigation coordinate resets at structural midpoint, not endpoint.
  - **The Goddess-section total (1448 days) is K-Elim Level 3 at p = 11** — clears `11³ = 1331` by 117 days (8.8% margin).

### Status change
- H4: PARTIAL → SUPPORTED.

---

## [0.6.0] — CRAM-ENHANCE integration

### Added
- `venus_kernel` — Shadow16(A(n)) = (A(n) mod 11) × (A(n) mod 13) coordinate × boundary product. Peak 110 at step 14. Fifth-operator rhythm (period-44 transitions = 11 Venus synodic cycles). Heterogeneous carry vector — per-lane carry rate over 20 Venus transitions.
- `substrate_roles` — prime role taxonomy: 2 parity / 3 fabric / 5 content / 7 traversal / 11 coordinate / 13 boundary.

### Grand synchronization
`A(260) = 37,960 days = 146 × 260 = 104 × 365 = 65 × 584`. All three calendars synchronize simultaneously at the Full Conductor. Exact integers.

---

## [0.5.0] — Dresden Codex decoder

### Added
- `moon_goddess` — full Moon Goddess Section decoder. `MoonGoddessProfile::compute()` returns `page_cram_addresses` for the nine-interval almanac `[148, 177, 148, 177, 148, 177, 148, 177, 148] = 1448 days`.
- `codex_decoder` — Venus Table + Eclipse Table + Binding Theorem (Venus/Eclipse share the Tzolk'in carry class `{3, 7, 11}` — theorem certified in all 5 parts).
- `lunar` — synodic arithmetic, `MOON_GODDESS_DAYS = 1448`, `TZOLKIN_DAYS = 260`.

### Headline structural result
The Goddess section activates lanes `{2, 5, 7, 11, 13}` — exactly the lanes the Tzolk'in nullifies. `1448 mod 260 = 148` — the remainder after stripping full Tzolk'ins is itself the first interval of the almanac. Arithmetic seal.

---

## [0.4.0] — Extended hypotheses

### Added
- `h3_extended` — 163 extended candidates (`73-family + intercalary family`).
- `h5_katun` — Katun partition.
- `manifold_upgrade` — Hamming-2 bridges.
- `generator_catalog` — 86-entry generator catalog.

---

## [0.3.0] — Generator stage 8 + K-Elim level theorem

### Added
- `h1_stage8` — bilinear generator extraction (SEED = 20).
- `h5_level` — K-Elim level theorem (`k_elim_level(x, p)`).
- `cross_validation` — independent corpora.
- `h4_montgomery` — Montgomery shadow quad, goddess-section quads.

---

## [0.2.0] — Generator extraction + triple K-Elim

### Added
- `h1_generator` — generator extraction.
- `h5_refined` — triple K-Elimination.
- `h4_non_visual` — 4 Goddess instruments + `GODDESS_SECTION_INTERVALS = [148, 177, 148, 177, 148, 177, 148, 177, 148]`.

---

## [0.1.0] — Initial atlas substrate

### Added
- Four-crate workspace (later trimmed to three).
- `dresden_codex` substrate — Safe Basis `{2, 3, 5, 7, 11, 13}`, `M_SAFE = 30,030`, `cram_address`, `active_lanes`, `nullified_lanes`, `carry_bits`, `pack_carry_bits`, `MultiPhaseSchema`.
- `dccms_atlas` query layer — `heads`, `events` (2,967-event corpus), `atlas` (`ConfigAtlas`), `recumbent`, `adelic_index`, `h3_mi`, `h4_instruments`, `h5_navigator`, `dkam_filter`, `manifold_geometry`.
- A1 (zero-float) and Object contract invariants established at every API boundary.

---

## Pre-history

The dccms_atlas codebase originated as an analysis of the Dresden Codex Goddess section's nine-interval almanac. Each version represents a single instrument added — never a rewrite. Every test from v0.1 still passes in v0.9.2.

The mission, unchanged since v0.1: **decode the Moon Goddess section of the Dresden Codex using exact-integer arithmetic on the QMNF / CRAM Safe Basis substrate.** Zero floating-point on the verification path. Every claim certified by integer congruence.
