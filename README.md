# DCCMS — Dresden Codex Configuration Manifold Study

**Version:** 0.8.0-dev (Tier 1 + hardening + B-7 operator fabric)
**Tests:** 479 passing, 0 failing (408 dccms_atlas + 69 dresden_codex + 2 prime_hunt)
**Author:** Anthony Diaz (Acid) — HackFate.us / Skyelabz210
**Computational collaborator:** Claude (Anthropic)

A computational decoder for the **Moon Goddess section** of the Dresden
Codex (pages 16–23), built as an exact-integer Rust library on top of the
QMNF/CRAM Safe Basis substrate. Zero floating-point arithmetic anywhere
in the verification path — every claim is certified by integer congruence.

The Goddess section is the *target*; the rest of the workspace is the
apparatus required to read it without leaving exact arithmetic.

---

## What it does

Given the nine-interval almanac on pages 16–23 of the Dresden Codex:

```
[148, 177, 148, 177, 148, 177, 148, 177, 148]   total = 1448 days = 49 synodic months
```

DCCMS computes the CRAM (Configurable Residue Arithmetic Machine) address
of every page boundary, the carry-signature of the total, the K-Elimination
level at p=11, the entropy of the interval sequence, and the carry-class
disjointness against the three Tzolk'in-aligned sections (Tzolk'in proper,
the Venus Table, and the Eclipse Table).

The headline structural result:

| Section | Carry class (active lanes) | Tzolk'in aligned |
|---|---|---|
| Tzolk'in / Venus Table / Eclipse Table | {3, 7, 11} | yes |
| **Moon Goddess (1448 days)** | **{2, 5, 7, 11, 13}** | **no — remainder 148** |
| Haab (365) | {2, 3, 7, 11, 13} | no |

The Goddess section activates lanes **2 and 5** — exactly the lanes the
Tzolk'in nullifies. It carries information the rest of the codex is
arithmetically blind to. And `1448 mod 260 = 148` — the remainder after
stripping full Tzolk'ins is itself the first interval. Not a coincidence:
an arithmetic seal.

Prime **11** is active in every section. This is the H5 result: prime 11
is the universal navigation coordinate across the entire codex.

---

## v0.6.0 — CRAM-ENHANCE integration

v0.6 adds three exact-integer constructs that came out of the Python
`cram_codex_decoder.py` analysis:

- **Shadow16(A(n)) = (A(n) mod 11) × (A(n) mod 13)** — the
  coordinate × boundary lane product. Drops to zero exactly when
  A(n) is divisible by 13. Peak 110 at step 14 (A(2078) = 10×11×...).
- **Fifth-operator rhythm** — Venus kernel accumulated mod 11.
  Period 44 transitions = 11 Venus synodic cycles. Kernel lane-11
  signature: [5, 2, 8, 8] = [236%11, 90%11, 250%11, 8%11].
- **Heterogeneous carry vector** — per-lane carry rate over 20
  Venus transitions. Lane 2 (parity) never fires; lane 7 (traversal)
  fires 60%. Each prime lane carries information at its own rate.

**Grand synchronization:** `A(260) = 37,960 days = 146 × 260 = 104 × 365 = 65 × 584`.
All three calendars synchronize simultaneously at the Full Conductor.
Exact integers, no floats.

### Substrate role taxonomy

| Prime | Role | Function |
|---|---|---|
| 2 | parity / parking | scale; Venus kernel values are even → silent |
| 3 | fabric / triadic | rhythmic base |
| 5 | content / calendar-embedded | Haab structure |
| 7 | traversal / internal | navigation within a configuration |
| **11** | **coordinate / position** | universal navigation index |
| **13** | **boundary / configuration-region** | region demarcation |

---

## Hypothesis table

| H | Hypothesis | Status |
|---|---|---|
| H1 | Single generator (SEED=20 bilinear G) | SUPPORTED |
| H2 | Four heads add genuine resolution | SUPPORTED |
| H3-a | Saturn-11² (121d) independent | SUPPORTED (43%) |
| H3-b | Temperaments-4fold (28d) | SUPPORTED (39%) |
| H3-c | EclipseAlt (325d) | SUPPORTED (32%) |
| H3-ext | 73-family + intercalary family | SUPPORTED |
| H4 | Goddess section encodes metadata | **SUPPORTED** (v0.7.0-dev — visual transducer closes the diagram) |
| H5 | Prime 11 universal coordinate | SUPPORTED (Level-2/3+ theorem) |
| Binding | Venus/Eclipse = Tzolk'in carry class | THEOREM (all 5 parts) |

**v0.8.0-dev Tier 1: DPM-PRIME mechanized arithmetic certificate suite.**
The 10-theorem stack from `The Dresden Codex.md` is rendered as
`cargo test`-decidable Rust assertions in [`dccms_atlas::dpm_prime`](dccms_atlas/src/dpm_prime.rs):
9 theorems certified by direct integer computation, 1 theorem (T4: Long
Count = covering space of Calendar Round) returns Conditional pending
the FSM-PRIME `M_Fib` covering-morphism construction. All 14 published
validation identities (V1–V14) pass; one extension (V14_strict) added
during v0.8.0 precision hardening to align V14 with Theorem T2's
exact predicate. **This is an arithmetic certificate suite, not a
Lean/Coq formal proof artifact** — the latter exists upstream in the
vault's `TUDPBoundary.lean` and FSM-PRIME infrastructure. Companion
module [`dresden_codex::sr_distribution`](dresden_codex/src/sr_distribution.rs)
exposes the planetary-displacement table (Mars, Venus, Saturn, Jupiter,
Mercury) and the `t10_s_r_union` predicate. New constants in
[`dresden_codex`](dresden_codex/src/lib.rs): `JUPITER_SYNODIC=399`,
`SATURN_SYNODIC=378`, `MERCURY_SYNODIC=116`, `MARS_SYNODIC=780`,
`CYCLE_819`, `ECLIPSE_TABLE_DAYS=11_960`, `ECLIPSE_CORRECTION=93`,
`EPOCH_33_YEAR=12_053`, `VENUS_HAAB_LCM=2_920`,
`LONG_COUNT_13_BAKTUN=1_872_000`, `LUNAR_NODAL=6_793`,
`RAMANUJAN_S_R=[5,7,11]`.

**v0.7.0-dev: H4 closed.** The visual transducer is built — see
[executioner_dag.md](executioner_dag.md). The new
`h4_visual` module (10 files, 50+ tests) provides:

- A `GlyphAlphabet<const K: usize>` trait with implementors for
  `BarDotNumeral`, `DayNameGlyph` (20 Tzolk'in days), `MonthGlyph`
  (18 Haab months + Wayeb), and `IconographicFigure` (9 Goddess pages).
- A `PageLayout` whose cumulative CRAM addresses match
  `MoonGoddessProfile::page_cram_addresses` **exactly** (Operator
  Consistency contract closed).
- A basis-parameterized `lift<const K>` that scales from K=6 (canonical
  Safe Basis) to K=7, K=8, K=10 with the first six lanes invariant —
  *arbitrary-precise scalability exhibited per-K, not asserted.*
- Five-Contract test gates (Object / Topology / Uniformity / Operator
  Consistency / Discharge) per the ns-continuum-bridge discipline.
- Two structural facts surfaced by the fifth-operator gate:
  * **Eb and Keh are lane-11 zeros of the Tzolk'in and Haab cycles** —
    the navigation coordinate resets at the structural midpoint of each
    cycle, not at its endpoint.
  * **The Goddess-section total (1448 days) is K-Elim Level 3 at p=11**
    (clears the 11³=1331 long-cycle threshold by 117 days, 8.8% margin).

The next phase (SEG01–SEG03) ingests actual codex pixels. By the Object
contract this is firewalled off the H4 closure path; the transducer is
correct independent of pixel quality. See
[docs/imagery_sources.md](docs/imagery_sources.md) for the public-domain
imagery survey.

---

## Workspace layout

```
dccms/
├── Cargo.toml                 ← workspace root (3 members)
├── README.md
├── .gitignore
├── dccms_atlas/               ← query layer (this is the decoder)
│   ├── Cargo.toml
│   ├── src/                   ← 25 modules, ~11,800 lines
│   │   ├── lib.rs
│   │   ├── heads.rs           ← four-calendar Hydra
│   │   ├── events.rs          ← 2967-event corpus
│   │   ├── atlas.rs           ← ConfigAtlas, Gini
│   │   ├── recumbent.rs       ← winding state
│   │   ├── adelic_index.rs
│   │   ├── h3_mi.rs           ← mutual information
│   │   ├── h4_instruments.rs  ← Goddess-section instruments (6)
│   │   ├── h4_non_visual.rs   ← non-visual H4
│   │   ├── h4_montgomery.rs   ← Montgomery shadow
│   │   ├── h5_navigator.rs    ← prime-11 distributor
│   │   ├── h5_refined.rs      ← triple K-Elim
│   │   ├── h5_level.rs        ← K-Elim level theorem
│   │   ├── h5_katun.rs        ← Katun partition
│   │   ├── dkam_filter.rs     ← DKAM admissibility
│   │   ├── manifold_geometry.rs
│   │   ├── manifold_upgrade.rs← Hamming-2 bridges
│   │   ├── h1_generator.rs    ← generator extraction
│   │   ├── h1_stage8.rs       ← bilinear generator
│   │   ├── h3_extended.rs     ← 163 candidates
│   │   ├── cross_validation.rs
│   │   ├── generator_catalog.rs
│   │   ├── lunar.rs
│   │   ├── moon_goddess.rs    ← ★ Moon Goddess decoder
│   │   ├── codex_decoder.rs   ← Venus / Eclipse / Binding
│   │   ├── venus_kernel.rs    ← v0.6.0 CRAM-ENHANCE kernel
│   │   └── substrate_roles.rs ← v0.6.0 role taxonomy + MI
│   └── examples/              ← 8 runnable examples
├── dresden_codex/             ← L1 substrate (CRAM primitives)
│   ├── Cargo.toml
│   └── src/lib.rs             ← 24 tests
└── prime_hunt/                ← prime sieve / factorization
    ├── Cargo.toml
    └── src/lib.rs             ← 2 tests
```

---

## Build and test

This repo lives under `C:\Users\hackf\Agents\dccms\` and uses the portable
Rust toolchain in `C:\Users\hackf\Agents\_toolchains\cargo\`.

```powershell
# In a fresh PS7 session
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
. $HOME\Agents\use-agents.ps1

Set-Location $HOME\Agents\dccms

cargo build --workspace --release
cargo test  --workspace --release      # 288 passing, 0 failing

# Run the v0.6.0 CRAM-ENHANCE decoder end-to-end
cargo run --example cram_enhance_decoder --release

# Other examples
cargo run --example codex_decoder           --release   # v0.5.0 Dresden decoder
cargo run --example complete_findings_v040  --release   # extended catalog
cargo run --example complete_findings_v030  --release   # generator + level
cargo run --example open_items_v020         --release   # κ₃ bifurcation
cargo run --example findings_query          --release   # v0.1.0 baseline
cargo run --example first_query             --release
cargo run --example independent_test        --release
```

---

## Provenance and what was stripped

The original release archive
(`dccms_atlas_v0_6_0_FINAL.tar.gz`, 116 MB compressed, 497 MB uncompressed,
1,089 entries) was packaged hot from a Linux build directory and shipped
with:

- **`dccms_work/target/`** — 496 MB of Rust incremental-build cache plus
  Linux ELF binaries unusable on Windows. Stripped.
- **`dccms_work/dccms_atlas_v0_1_0_tar.gz`** — a nested historical-version
  archive of the same source in older form. Stripped.
- **`dccms_work/qmnf_primitives` → `/home/crates/qmnf_primitives`**
  and **`dccms_work/qcid_ns_bridge` → `/home/crates/qcid_ns_bridge`** —
  dangling symlinks pointing at paths inside the build container that
  did not survive into the tarball. Both were declared as workspace
  members and as dependencies of `dccms_atlas`, but neither was actually
  `use`d anywhere in the source (only one doc-comment in
  `adelic_index.rs:25` mentions `qcid_ns_bridge`). Removed from
  `Cargo.toml`. The link against them in the original `target/` had been
  a 4 KB stub.

After stripping, the source-only working tree is 609 KB. A native Windows
rebuild produces the same test counts and the same numeric outputs as the
Linux-side build originally reported.

The full original archive remains untouched at
`C:\Users\hackf\Downloads\dccms_atlas_v0_6_0_FINAL.tar.gz` if any of the
stripped material is later needed.

---

## License

Specify before publishing to GitHub.

---

## Contact

Anthony Diaz · skyelabz210@gmail.com · HackFate.us
