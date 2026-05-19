# DCCMS — Dresden Codex Configuration Manifold Study

**Version:** 0.9.2-dev — customize-fully calibration
**Tests:** 648 passing, 0 failing (532 `dccms_atlas` + 94 `dresden_codex` + 22 `prime_hunt`)
**Author:** Anthony Diaz (Acid) — [HackFate.us](https://hackfate.us) / [Skyelabz210](https://github.com/Skyelabz210)
**Computational collaborator:** Claude (Anthropic)

A computational decoder for the **Moon Goddess section** of the Dresden
Codex (Förstemann pages 16–23), built as an exact-integer Rust library on
top of the QMNF / CRAM Safe Basis substrate. **Zero floating-point
arithmetic anywhere on the verification path** — every claim is certified
by integer congruence.

The Goddess section is the *target*. The rest of the workspace is the
apparatus required to read it without leaving exact arithmetic.

---

## What the program does

Given the nine-interval almanac on Förstemann pages 16–23:

```
[148, 177, 148, 177, 148, 177, 148, 177, 148]    total = 1448 days = 49 synodic months
```

DCCMS computes:

- The **CRAM (Configurable Residue Arithmetic Machine) address** of every
  page boundary in the Safe Basis `{2, 3, 5, 7, 11, 13}` modulo
  `M_SAFE = 30,030`.
- The **carry-signature** of the 1448-day total and each interval.
- The **K-Elimination level** at prime `p = 11` (the navigation lane).
- The **carry-class disjointness** of the Goddess section against the
  three Tzolk'in-aligned sections of the codex (Tzolk'in proper, the
  Venus Table, the Eclipse Table).
- The **mutual information** between candidate "head" hypotheses
  (Saturn-11², 4-fold temperaments, 12-fold zodiac, Venus phase, …)
  and the event sequence.
- The **iconographic verification** of each Goddess page against the
  vault-known sequence (MoonSign, RabbitSign, …, BlankBridge).
- The **cross-source corroboration** of post-WWII SLUB photographs
  against the pre-1880 Förstemann / Schele FAMSI chromolithograph —
  damaged pages lose their red barriers, intact pages keep them.

### The headline structural result

| Section | Carry class (active lanes) | Tzolk'in aligned? |
|---|---|---|
| Tzolk'in / Venus Table / Eclipse Table | `{3, 7, 11}` | yes |
| **Moon Goddess (1448 days)** | **`{2, 5, 7, 11, 13}`** | **no — remainder 148** |
| Haab (365) | `{2, 3, 7, 11, 13}` | no |

The Goddess section activates lanes **2 and 5** — exactly the lanes the
Tzolk'in nullifies. It carries information the rest of the codex is
arithmetically blind to. And `1448 mod 260 = 148`: the remainder after
stripping full Tzolk'ins is itself the first interval of the almanac.
Not a coincidence — an arithmetic seal.

Prime **11** is active in every section: the H5 result. Prime 11 is the
universal navigation coordinate across the entire codex.

---

## Architecture

A three-crate Rust workspace, layered:

```text
┌─────────────────────────────────────────────────────────────┐
│  dccms_atlas (query layer — ~62 modules, 532 tests)         │
│  ┌──────────┬──────────┬──────────┬──────────┬──────────┐   │
│  │ heads    │ atlas    │ events   │ h*_*     │ engines  │   │
│  │ moon_    │ codex_   │ venus_   │ dpm_     │ segmenter│   │
│  │ goddess  │ decoder  │ kernel   │ prime    │          │   │
│  └──────────┴──────────┴──────────┴──────────┴──────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│  L1 substrate (verified, exact-integer)                     │
│  ┌──────────────────────┐    ┌──────────────────────────┐   │
│  │ dresden_codex        │    │ prime_hunt               │   │
│  │ — Safe Basis ops     │    │ — sieves, gap analysis,  │   │
│  │ — CRAM address       │    │   Ramanujan partition    │   │
│  │ — K-Elim divide      │    │ (22 tests)               │   │
│  │ — shadow_bond        │    │                          │   │
│  │ — S_R distribution   │    │                          │   │
│  │ (94 tests)           │    │                          │   │
│  └──────────────────────┘    └──────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

### Mathematical foundations

| Concept | Source crate | Use |
|---|---|---|
| Safe Basis `S₆ = {2, 3, 5, 7, 11, 13}`, `M_SAFE = 30030` | `dresden_codex` | residue arithmetic for all codex periods |
| CRAM address | `dresden_codex::cram_address` | the canonical Safe-Basis fingerprint of any integer ≤ M_SAFE |
| Active / nullified lanes | `dresden_codex::{active_lanes, nullified_lanes}` | "which primes the integer touches" |
| K-Elimination at prime `p` | `dresden_codex::k_elim_divide` | navigation-level classification (Level 0–4 at p=11) |
| Shadow-bond detector | `dresden_codex::shadow_bond` | T10-style content view vs anchor-in-period |
| S_R distribution (Mars, Venus, Saturn, Jupiter, Mercury) | `dresden_codex::sr_distribution` | planetary-displacement coverage |
| Ramanujan partition | `prime_hunt::ramanujan_partition` | discovery 1 + 3 (S_R = {5, 7, 11}) |
| Prime-gap statistics | `prime_hunt::prime_gap_analysis` | discovery 5 (gap-doubling) |

A1 (zero-float) is enforced at every API boundary. Internal float use is
disallowed — every computation is integer or rational with explicit
denominators.

### Hypothesis stack

| H | Hypothesis | Status |
|---|---|---|
| H1 | A single bilinear generator (SEED = 20) produces the four Maya calendars as marked points on the configuration manifold | SUPPORTED |
| H2 | The four calendars correspond to four heterogeneous Hydra heads; the atlas adds resolution over any single head | SUPPORTED |
| H3-a | Saturn-11² (121 d) is independent of the four canonical heads | SUPPORTED (43%) |
| H3-b | 4-fold temperaments (28 d) | SUPPORTED (39%) |
| H3-c | EclipseAlt (325 d) | SUPPORTED (32%) |
| H3-ext | 73-family + intercalary family | SUPPORTED |
| H4 | The Goddess section encodes configuration metadata | **SUPPORTED** (v0.7 visual transducer + v0.9 segmentation close it) |
| H5 | Prime 11 is the universal navigation coordinate | SUPPORTED (Level-2/3+ theorem) |
| Binding | Venus Table and Eclipse Table share the Tzolk'in carry class | THEOREM (all 5 parts certified) |

---

## Layer-by-layer

### `dresden_codex` — L1 substrate (94 tests)

Pure CRAM arithmetic, no codex semantics.

| Module | Provides |
|---|---|
| `lib.rs` | `SAFE_BASIS`, `M_SAFE`, period constants (`TZOLKIN_PERIOD=260`, `HAAB_PERIOD=365`, `VENUS_SYNODIC=584`, `MARS_SYNODIC=780`, `JUPITER_SYNODIC=399`, `SATURN_SYNODIC=378`, `MERCURY_SYNODIC=116`, `CYCLE_819`, `ECLIPSE_TABLE_DAYS=11_960`, `EPOCH_33_YEAR=12_053`, `VENUS_HAAB_LCM=2_920`, `LONG_COUNT_13_BAKTUN=1_872_000`, `LUNAR_NODAL=6_793`, `RAMANUJAN_S_R=[5,7,11]`), `cram_address`, `active_lanes`, `nullified_lanes`, `carry_bits`, `pack_carry_bits`, `MultiPhaseSchema`, `VENUS_PHASES` |
| `k_elim_divide.rs` | K-Elimination division (phase-differential), navigation-level certificate |
| `shadow_bond.rs` | T10 S_R-content view, anchor-in-period detector |
| `sr_distribution.rs` | Planetary-displacement table + `t10_s_r_union` predicate |

### `prime_hunt` — sieves and partitions (22 tests)

| Module | Provides |
|---|---|
| `lib.rs` | basic prime sieve interfaces |
| `ramanujan_partition.rs` | Discovery 1 (Ramanujan congruence content) + Discovery 3 (S_R = {5, 7, 11}) |
| `prime_gap_analysis.rs` | Discovery 5 (prime-gap doubling near the Safe Basis) |

### `dccms_atlas` — query layer (532 tests)

The decoder proper. Organized by hypothesis and by feature area.

#### Core (v0.1 – v0.5)

| Module | Role |
|---|---|
| `heads`, `events`, `atlas` | Hydra-head construction, 2967-event codex corpus, four-head address-space atlas |
| `recumbent`, `adelic_index` | Multi-modulus winding state, dual-track adelic indexing |
| `h3_mi`, `h3_extended` | Mutual-information report, 163 extended candidates |
| `h4_instruments`, `h4_non_visual`, `h4_montgomery` | Six Goddess-section instruments, Montgomery shadow |
| `h5_navigator`, `h5_refined`, `h5_level`, `h5_katun` | 11-lane distributor, triple K-Elim, level theorem, Katun partition |
| `dkam_filter`, `dkam_tier` | DKAM admissibility filter, tier mapping |
| `manifold_geometry`, `manifold_upgrade` | Manifold profile, Hamming-2 bridges, connectivity |
| `h1_generator`, `h1_stage8` | Generator extraction (Stage 7 + 8) |
| `cross_validation`, `generator_catalog` | Cross-validation corpus, catalog stats |
| `lunar`, `moon_goddess`, `codex_decoder` | ★ Moon Goddess decoder, full codex decoder (v0.5) |

#### Substrate roles (v0.6 — CRAM-ENHANCE)

| Module | Role |
|---|---|
| `venus_kernel` | Venus kernel under K-Elim, lane-11 signature `[5, 2, 8, 8]`, fifth-operator rhythm |
| `substrate_roles` | Per-prime role taxonomy (parity / fabric / content / traversal / coordinate / boundary) |

#### Visual transducer (v0.7 — H4 closed)

`h4_visual/` (10 files): basis-parameterized `lift<const K>`, the `GlyphAlphabet<const K>` trait with implementors for `BarDotNumeral`, `DayNameGlyph` (20 Tzolk'in days), `MonthGlyph` (18 Haab months + Wayeb), `IconographicFigure` (9 Goddess pages), and a `PageLayout` whose cumulative CRAM addresses match `MoonGoddessProfile::page_cram_addresses` exactly. Five-Contract test gates (Object / Topology / Uniformity / Operator Consistency / Discharge).

Two structural facts surfaced by the fifth-operator gate:
- **Eb and Keh are lane-11 zeros** of the Tzolk'in and Haab cycles — the navigation coordinate resets at the *structural midpoint* of each cycle, not at its endpoint.
- **The Goddess-section total (1448 days) is K-Elim Level 3 at p = 11** — clears the `11³ = 1331` long-cycle threshold by 117 days (8.8% margin).

#### Operator fabric + DPM-PRIME (v0.8)

| Module | Role |
|---|---|
| `codex_topology` | Codex-topology framework |
| `dpm_prime` | DPM-PRIME 10-theorem arithmetic certificate suite (T1–T10) — `cargo test`-decidable, with V14_strict precision-hardening extension |
| `engines/` | Operator-fabric engines: `Lane`, `MayaState`, `Vigesimal`, `Tzolkin`, `LongCount`, `DresdenEclipse`, `VenusTable`, `MayaFabric`, `Pisano`, `Ramanujan`, `Recombinant` |
| `ramanujan_alignment` | Ramanujan-alignment instruments |
| `page_arithmetic`, `maya_date` | Page-level arithmetic, Maya date utilities |
| `gini_stratification` | Gini stratification of the event corpus |
| `goddess_extension`, `extended_goddess` | Glyph specs for pages 13c–15 (extension framework) |

#### Pixel ingestion (v0.9 — image segmenter)

`segmenter/` is feature-gated under `slub` (pure-Rust JPEG decode via `image` 0.25, `jpeg` feature only).

| Module | Role |
|---|---|
| `mod.rs` | `ImageBuffer`, `BoundingBox`, `Segmenter` trait, `GlyphClassifier` trait — discrete typed contracts at all API boundaries |
| `null.rs` | `NullSegmenter`, `NullClassifier` for tests |
| `slub.rs` | SLUB JPEG adapter (`load_slub_page`) — produces `ImageBuffer` |
| `threshold.rs` | `DarknessThresholdSegmenter` — pixel-sum threshold + connected-components flood fill, 4-connectivity |
| `closing.rs` | `ClosingThresholdSegmenter` — morphological closing (separable dilate→erode) before connected-components; eliminates register-leak under-segmentation |
| `register.rs` | `RegisterAwareSegmenter<S>` — detects red horizontal barriers, partitions image into bands, runs the inner segmenter per band |
| `classify.rs` | `PageContextClassifier`, `verify_page_iconography` — vault-known page → expected `IconographicFigure` |
| `comparison.rs` | SLUB ↔ FAMSI cross-source comparison, vault-known `WWII_DAMAGED_PAGES = [2, 4, 24, 28, 34, 38, 71, 72]`, combined damage signal (stats AND zero barriers) |

#### Hard-coded paths (v0.9.2)

| Module | Role |
|---|---|
| `paths.rs` | `slub_page(n)`, `famsi_page(n)`, `famsi_pdf()`, `FAMSI_PAGE_RANGE = 13..=24` — hard-coded `~/Agents/imports/` layout for HackFate's machine |

---

## SLUB + FAMSI imagery pipeline (v0.9.0 → v0.9.2)

Two independent renderings of the same Dresden Codex pages are used:

| Source | Resolution | Origin | What it shows |
|---|---|---|---|
| **SLUB Dresden** | 3874 × 7649 @ 300 DPI RGB JPEG | post-WWII high-resolution photograph by the Sächsische Landesbibliothek, the codex's vault | the codex *as it is today* — with WWII water damage |
| **FAMSI** | 1552 × 3332 RGB JPEG | Förstemann 1880 / Linda Schele color edition, distributed by the Foundation for the Advancement of Mesoamerican Studies | the codex *before 1945* — chromolithograph captures pre-damage content |

### What we built for it

- **No PDF tooling available** on this machine. v0.9.0 built a pure-Rust byte-scanner ([`examples/famsi_extract.rs`](dccms_atlas/examples/famsi_extract.rs)) that locates JPEG SOI/EOI markers (`FF D8 FF` … `FF D9`) inside the FAMSI PDF directly. Worked first try. Extracted 12 JPEGs.
- **FAMSI → Förstemann mapping** verified at 98% of greedy best-match total similarity by histogram intersection over 32-bin vertical row-darkness fingerprints ([`examples/calibrate.rs`](dccms_atlas/examples/calibrate.rs) Job 2). Treated as 1-to-1 throughout.
- **SLUB barrier-color empirical calibration**: red-barrier ink samples to brownish/orange (median R=164, G=140, B=126), not pure crimson. `RegisterAwareSegmenter::new()` defaults `(red_min=164, red_excess=24, row_fraction_per_mille=200)` — barriers now fire on 10 of 12 pages 13–24.
- **Combined damage signal** breaks the v0.9.1 page-18 false positive. The plain `DarknessThresholdSegmenter` cannot distinguish page 18 (content-bearing, clean register separation) from page 24 (WWII-damaged) by stats alone — both look "blank-like". Barrier count discriminates: page 18 has 52 detected barriers, page 24 has 0.
- **Mode shift**: the segmenter no longer *discovers* damaged pages from stats. The vault-known `WWII_DAMAGED_PAGES = [2, 4, 24, 28, 34, 38, 71, 72]` list is authoritative; the segmenter *verifies* by corroboration.

End-to-end result on pages 13–24:

```
 page | vault    | barriers | combined signal | verdict
 -----+----------+----------+-----------------+----------------
   13 | intact   |       18 | intact          | ✓ corroborated
   14 | intact   |       22 | intact          | ✓ corroborated
   15 | intact   |        0 | intact          | ✓ corroborated
   16 | intact   |       39 | intact          | ✓ corroborated
   17 | intact   |       17 | intact          | ✓ corroborated
   18 | intact   |       52 | intact          | ✓ corroborated    ← false-pos fixed
   19 | intact   |       12 | intact          | ✓ corroborated
   20 | intact   |       94 | intact          | ✓ corroborated
   21 | intact   |       25 | intact          | ✓ corroborated
   22 | intact   |       22 | intact          | ✓ corroborated
   23 | intact   |       52 | intact          | ✓ corroborated
   24 | DAMAGED  |        0 | DAMAGE          | ✓ corroborated
```

**12/12 pages corroborate the vault. Zero disagreements.**

---

## Workspace layout

```
dccms/
├── Cargo.toml                  # workspace root (3 members)
├── Cargo.lock
├── README.md                   # this file
├── WORKSPACE_MANIFEST.md       # crate inventory + status flags
├── executioner_dag.md          # DAG of build nodes through tiers
├── docs/
│   ├── imagery_sources.md
│   ├── slub_segmentation_findings.md
│   ├── hackfate_dresden_synthesis.md
│   ├── v0_8_0_TIER3_PLAN.md
│   ├── v0_8_0_B7_PLAN.md
│   ├── v0_8_0_B7_COMPLETION_PLAN.md
│   ├── v0_9_0_PLAN.md
│   ├── v0_9_1_PLAN.md
│   ├── v0_9_1_findings.md
│   └── v0_9_2_findings.md      # latest
├── dccms_atlas/                # query layer (54 .rs source files + 14 examples)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs              # module exports + project constants
│   │   ├── heads.rs            # four-calendar Hydra
│   │   ├── events.rs           # 2967-event corpus
│   │   ├── atlas.rs            # ConfigAtlas, Gini
│   │   ├── recumbent.rs        # winding state
│   │   ├── adelic_index.rs
│   │   ├── h1_generator.rs
│   │   ├── h1_stage8.rs
│   │   ├── h3_mi.rs
│   │   ├── h3_extended.rs
│   │   ├── h4_instruments.rs
│   │   ├── h4_non_visual.rs
│   │   ├── h4_montgomery.rs
│   │   ├── h4_visual.rs        # H4 visual-transducer root
│   │   ├── h4_visual/          # 10 files — alphabet, bardot, dayname, month, …
│   │   ├── h5_navigator.rs
│   │   ├── h5_refined.rs
│   │   ├── h5_level.rs
│   │   ├── h5_katun.rs
│   │   ├── dkam_filter.rs
│   │   ├── dkam_tier.rs
│   │   ├── manifold_geometry.rs
│   │   ├── manifold_upgrade.rs
│   │   ├── cross_validation.rs
│   │   ├── generator_catalog.rs
│   │   ├── lunar.rs
│   │   ├── moon_goddess.rs     # ★ Moon Goddess decoder
│   │   ├── codex_decoder.rs    # Venus / Eclipse / Binding
│   │   ├── codex_topology.rs
│   │   ├── venus_kernel.rs
│   │   ├── substrate_roles.rs
│   │   ├── dpm_prime.rs        # T1–T10 certificate suite
│   │   ├── engines/            # 11 files — Lane, MayaState, Vigesimal, …
│   │   ├── ramanujan_alignment.rs
│   │   ├── page_arithmetic.rs
│   │   ├── maya_date.rs
│   │   ├── gini_stratification.rs
│   │   ├── goddess_extension.rs
│   │   ├── extended_goddess.rs
│   │   ├── segmenter/          # 8 files — image ingestion, segmenter, comparison
│   │   └── paths.rs            # hard-coded data paths
│   └── examples/               # 14 runnable examples
├── dresden_codex/              # L1 substrate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── k_elim_divide.rs
│       ├── shadow_bond.rs
│       └── sr_distribution.rs
└── prime_hunt/                 # sieves + Ramanujan partition
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── ramanujan_partition.rs
        └── prime_gap_analysis.rs
```

---

## Build and test

This repo lives under `C:\Users\hackf\Agents\dccms\` and uses the portable
Rust toolchain in `C:\Users\hackf\Agents\_toolchains\cargo\`. PowerShell 7
is the supported shell.

```powershell
# In a fresh PS7 session
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
. $HOME\Agents\use-agents.ps1

Set-Location $HOME\Agents\dccms

# Build everything (without the slub image feature)
cargo build --workspace --release

# Build with SLUB / FAMSI image segmenter
cargo build --workspace --release --features dccms_atlas/slub

# Run the full test suite (532 + 94 + 22 = 648 tests)
cargo test --workspace --release --features dccms_atlas/slub
```

### Examples (14 runnable)

```powershell
# Pure-arithmetic decoders — no imagery feature needed
cargo run --release --example codex_decoder              # v0.5 full codex decoder
cargo run --release --example cram_enhance_decoder        # v0.6 Venus kernel + roles
cargo run --release --example complete_findings_v040      # extended catalog
cargo run --release --example complete_findings_v030      # generator + level
cargo run --release --example open_items_v020             # κ₃ bifurcation
cargo run --release --example findings_query              # baseline + extension
cargo run --release --example first_query                 # canonical four-head atlas
cargo run --release --example independent_test            # null / prime / Fibonacci corpora
cargo run --release --example famsi_extract               # FAMSI PDF → 12 JPEGs

# Image-segmenter decoders — require `--features dccms_atlas/slub`
cargo run --release --features dccms_atlas/slub --example slub_segment       # one page
cargo run --release --features dccms_atlas/slub --example slub_improved      # closing + register-aware
cargo run --release --features dccms_atlas/slub --example famsi_segment_all  # all 12 FAMSI pages
cargo run --release --features dccms_atlas/slub --example compare_slub_famsi # cross-source verdict
cargo run --release --features dccms_atlas/slub --example calibrate          # 4-job calibration
```

### Imagery prerequisites

The image-segmenter examples expect:

```
~/Agents/imports/
├── slub_dresden/
│   └── page_00000013.jpg  …  page_00000024.jpg
└── famsi_dresden/
    ├── famsi_pp13-24.pdf
    └── extracted/
        └── page_13.jpg  …  page_24.jpg
```

SLUB pages are downloadable from the SLUB Dresden IIIF service
(`Mscr.Dresd.R.310` — Codex Maya Dresdensis). The FAMSI PDF is at
`https://www.famsi.org/mayawriting/codices/pdf/2_dresden_fors_schele_pp13-24.pdf`
(public domain — Förstemann 1880 + Schele color edition). Run
`cargo run --release --example famsi_extract` once to populate `extracted/`
from the PDF. Both sources are public domain by age.

---

## Provenance and what was stripped

The original release archive (`dccms_atlas_v0_6_0_FINAL.tar.gz`, 116 MB
compressed, 497 MB uncompressed, 1,089 entries) was packaged hot from a
Linux build directory and shipped with:

- **`dccms_work/target/`** — 496 MB of Rust incremental-build cache plus
  Linux ELF binaries unusable on Windows. Stripped.
- **`dccms_work/dccms_atlas_v0_1_0_tar.gz`** — a nested historical-version
  archive of the same source in older form. Stripped.
- **`dccms_work/qmnf_primitives` → `/home/crates/qmnf_primitives`** and
  **`dccms_work/qcid_ns_bridge` → `/home/crates/qcid_ns_bridge`** —
  dangling symlinks pointing at paths inside the build container that did
  not survive into the tarball. Both were declared as workspace members
  and as dependencies of `dccms_atlas`, but neither was actually `use`d
  anywhere in the source (only one doc-comment in `adelic_index.rs:25`
  mentions `qcid_ns_bridge`). Removed from `Cargo.toml`.

After stripping, the source-only working tree fits well under 1 MB. A
native Windows rebuild produces the same test counts and the same numeric
outputs as the Linux-side build originally reported.

---

## Version arc

| Version | Theme | Key landmarks |
|---|---|---|
| v0.1–v0.4 | Atlas substrate | Four-head Hydra, ConfigAtlas, H1/H3/H5 reports, manifold geometry |
| v0.5 | Codex decoder | `moon_goddess`, `codex_decoder` — Venus / Eclipse / Binding theorem |
| v0.6 | CRAM-ENHANCE | `venus_kernel` (lane-11 sig `[5, 2, 8, 8]`), `substrate_roles` taxonomy |
| v0.7 | H4 closed | `h4_visual` 10-file visual transducer, basis-parameterized `lift<const K>` |
| v0.8 | Tier 1–3 | DPM-PRIME 10-theorem certificate, operator fabric (8 engines), Ramanujan alignment, DKAM tier, Gini stratification, page arithmetic, Maya date, Goddess extension |
| v0.9.0 | "No outside scope" | SLUB / FAMSI ingestion, threshold segmenter, FAMSI byte-scan PDF extractor |
| v0.9.1 | Three-area improvements | Closing segmenter, register-aware framework, iconographic verification, cross-source comparison |
| **v0.9.2** | **Customize-fully calibration** | **Empirical SLUB red thresholds, vault-driven damage verification, 12/12 corroboration on pages 13–24, hard-coded paths** |

Each step is reflected in `docs/v0_X_Y_PLAN.md` (plan) → `docs/v0_X_Y_findings.md` (results) → a single Git commit per phase.

---

## Discipline rules in force throughout

- **A1 — Zero float.** No `f32` / `f64` / `float` anywhere on the API boundary. Internal float use is also disallowed; every computation is integer or rational with explicit denominators.
- **Object contract.** All segmenter output crosses the API as `Vec<BoundingBox>` (discrete, integer coordinates); all classifier output crosses as `Option<Glyph>` (discrete, typed). No confidence vectors.
- **Manifest first.** Never duplicate what the manifest says exists. The L1 substrate is wired into, never rebuilt.
- **No outside scope.** Apparent limits become build targets. The FAMSI PDF byte-scanner was built because no PDF tooling was available; the calibration tool was built because empirical SLUB color values were unknown.

---

## License

Specify before publishing to GitHub.

---

## Contact

Anthony Diaz · [skyelabz210@gmail.com](mailto:skyelabz210@gmail.com) · [HackFate.us](https://hackfate.us)
