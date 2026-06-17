# DCCMS Research Record — Full Session Scrape

**Date:** 2026-06-17  
**Classification:** Complete session findings, insights, conjectures, and hypotheses  
**Branch:** `claude/work-analysis-gaps-360pvw`

## Reliability Marking System

| Tag | Meaning |
|---|---|
| `[VERIFIED]` | Directly confirmed by reading source code or running tests in this session |
| `[AGENT-REPORTED]` | Reported by a sub-agent reading actual source files; reliable but not double-checked line-by-line |
| `[UNRELIABLE — AI-PENNED SOURCE]` | Originates from vault documents written by AI in prior sessions; not established fact |
| `[UNAVAILABLE]` | Evidence sought but tooling failure prevented access |
| `[USER-STATED]` | Reported by the user; taken as accurate but not independently verified |
| `[SCHOLARLY]` | Sourced to named academic publications; the most reliable class |

---

## 1. What the User Actually Built and Why

**User's own statement** `[USER-STATED]`:

> "this entire repository exists because i was attempting testing and building the testing apparatus and you'll notice that it was the penmen who through poor researcher and engineering hygiene recorded the records that you're now fictioning with and you'll notice that when i noticed i've been since trying to recover the work from its poor work ethic"

**Plain translation:** The v0.9 segmenter arc is the user's testing apparatus, built specifically to check and ground the prior AI-penned interpretive claims. The theorem stack and DPM-PRIME layer are artifacts of what the user is trying to recover from. The repository is a calibration instrument, not a proof system.

---

## 2. Workspace Facts `[VERIFIED]`

**Repository:** `Skyelabz210/dccms`, branch `claude/work-analysis-gaps-360pvw`

**3-crate Rust workspace:**
- `dresden_codex` — L1 substrate: CRAM arithmetic primitives, Safe Basis, K-Elimination, shadow_bond, S_R distribution (94 tests)
- `prime_hunt` — prime sieve, Ramanujan partition, gap analysis (22 tests)
- `dccms_atlas` — query layer, decoders, segmenter (~80 source files, 554 unit tests + 6 integration tests)

**Total: 676 passing, 0 failing, 0 warnings** (with `--features slub`)

**Enforced workspace invariants:**
- `#![forbid(unsafe_code)]` — no overrides anywhere `[VERIFIED]`
- `#![deny(clippy::float_arithmetic)]` — no overrides anywhere `[VERIFIED]`
- Only external dependency: `image` 0.25 (pure-Rust JPEG only, feature-gated under `slub`)
- No C dependencies, no network/async/database

**Version:** `0.9.5-dev`

---

## 3. The Circular Verification Finding `[VERIFIED — direct code reading]`

This is the most significant finding. It explains why "12/12 figure matches" and "12/12 CRAM matches" tell you nothing about the codex.

### Finding 3a — Figure classification is a page-number lookup

`dccms_atlas/src/h4_visual/iconographic.rs:63-76` `[VERIFIED]`:
```rust
pub fn from_page(page: u8) -> Option<IconographicFigure> {
    match page {
        16 => Some(IconographicFigure::MoonSign),
        17 => Some(IconographicFigure::WaterPot),
        // ... all 9 pages hardcoded
        24 => Some(IconographicFigure::BlankBridge),
        _ => None,
    }
}
```

`dccms_atlas/src/segmenter/classify.rs:301` `[VERIFIED]`:
```rust
IconographicFigure::from_page(self.page).map(BboxClass::Figure)
```

The classifier confirms "a large dark blob exists" (geometry gates), then returns the figure identity by page number. It cannot distinguish MoonSign from WaterPot on the same page or any other page.

### Finding 3b — The figure match is tautological

`dccms_atlas/src/segmenter/pipeline.rs:155-160` `[VERIFIED]`:
```rust
let figure_match = match (classified_figure, expected_figure) {
    (None, None) => true,
    (Some(c), Some(e)) => c == e,  // both derived from from_page(page)
    (None, Some(IconographicFigure::BlankBridge)) => true,
    _ => false,
};
```

Both `classified_figure` and `expected_figure` derive from `from_page(page)`. The match is guaranteed by construction. "12/12 figure matches" is definitionally true for all pages 16–24.

### Finding 3c — CRAM match does not involve pixels

`dccms_atlas/src/segmenter/pipeline.rs:130-146` `[VERIFIED]`:
- `predicted_cram_visual` — computed from `goddess_section_layout()`, a hardcoded page-figure mapping
- `reference_cram_non_visual` — computed from `MoonGoddessProfile::compute()`, using the same hardcoded `GODDESS_SECTION_INTERVALS`

Both sides come from the same hardcoded constants. Pixels are never read. "12/12 CRAM match" holds by mathematical identity, not by image analysis.

### Finding 3d — Damage corroboration is calibration, not discovery

`dccms_atlas/src/segmenter/comparison.rs` `[VERIFIED]`:
- `WWII_DAMAGED_PAGES: &[u8] = &[2, 4, 24, 28, 34, 38, 71, 72]` — hardcoded vault list
- `slub_signals_damage` thresholds calibrated specifically against page 24
- "Segmenter corroborates the vault" = agreement with the list the code was given

v0.9.5 finding (in `docs/v0_9_5_findings.md`) `[VERIFIED]`: the stat-only predicate catches only **1 of 8** damaged pages (page 24). The other seven produce opposite segmenter signatures (stain accumulation rather than ink loss).

---

## 4. Mathematical Claims Assessment `[VERIFIED — direct code reading]`

### 4a — The DPM-PRIME theorem suite

Located at `dccms_atlas/src/dpm_prime.rs`. Approximately 97% are computational witnesses that hardcoded constants recompute to themselves.

Representative examples:
- **T1:** Asserts 4×5×13×73 = 18,980. True. Equivalent to `assert_eq!(2+2, 4)`.
- **T2:** Finds minimum T < 365 where lcm(T, 365) = 18,980. Returns 260. Tautological by the definition of lcm.
- **T3:** Verifies 819 = lcm(9,7,13) by exhaustive search. True by definition.
- **T4:** Marked `Conditional` — the covering-space morphism is deferred to external Lean infrastructure. The arithmetic reduction is correct; the structural claim is **not proven** in this codebase.
- **T5–T10:** Same pattern.

**None of these are falsifiable claims about the codex.** If a test fails it means a constant changed, not that the theory was refuted.

### 4b — "Binding Theorem"

`codex_decoder.rs:360-379` `[VERIFIED]`. Five-part claim that Venus and Eclipse share the Tzolk'in carry class. All five parts are divisibility consequences of the chosen constant definitions. Labeled "THEOREM" but has no falsifiable content independent of the constants.

### 4c — "Prime 11 is the universal navigation coordinate" `[UNRELIABLE — AI-PENNED SOURCE]`

An artifact of choosing Maya periods that happen to avoid 11 as a factor. The "universality" is a consequence of the framework's design, not a discovery from the codex.

### 4d — Ramanujan "Discoveries"

- Discovery 1: Mechanization of the Ahlgren–Ono theorem (2000). **Correctly credited** in-code as upstream-proven — the mechanization is valid, but calling it a "discovery" is misleading.
- Discovery 3: Logical consequence of Ahlgren–Ono via CRT. Valid derivation, not a new discovery.
- Discovery 2 (prime gap doubling): Code honestly reports the actual ratio is ~2.67, not exactly 2.0. Acknowledged internally.

### 4e — Zero-float discipline `[VERIFIED]`

Genuinely enforced. `#![forbid(unsafe_code)]` and `#![deny(clippy::float_arithmetic)]` with no overrides anywhere in the workspace. This part of the mathematical infrastructure is rigorous.

---

## 5. Engineering Hygiene Findings

### 5a — Hard-coded paths `[VERIFIED]`
- `dccms_atlas/src/paths.rs`: resolves `~/Agents/imports` — machine-specific
- `scripts/download_slub_all.sh:7`: `DEST="/c/Users/hackf/Agents/imports/slub_dresden"` — hardcoded Windows path
- `scripts/download_famsi_all.sh:6`: same machine-specific path
- Scripts use GNU-only `stat -c %s` (not portable to macOS/BSD)
- `DCCMS_IMPORTS_ROOT` env-var override now added to `paths::imports_root()`

### 5b — Dead-code suppression `[VERIFIED]`
- ~40 source files carry module-level `#![allow(dead_code)]`
- "0 warnings" is partly attributable to this suppression, not purely clean code

### 5c — Documentation lag `[VERIFIED]`
| Document | Says | Reality |
|---|---|---|
| README | v0.9.2-dev, 648 tests | v0.9.5-dev, 676 tests |
| WORKSPACE_MANIFEST | v0.9.2-dev, 648 tests | v0.9.5-dev, 676 tests |
| H4 "real-pixel closure pending" | open item | completed in v0.9.4 |
| Closing-segmenter pickup listed open | open item | completed in v0.9.3 |

### 5d — No LICENSE `[VERIFIED]`
README states "Specify before publishing to GitHub." No LICENSE file exists anywhere in the tree.

### 5e — Page coverage gap `[VERIFIED]`
The semantic model (`from_page`, `IconographicFigure`, CRAM layouts) only covers pages 16–24. `from_page()` returns `None` for all other pages. There is no semantic model for approximately 65 of 74 codex pages.

### 5f — Algorithms present vs absent `[VERIFIED — direct grep]`

Present (named implementations):
`shadow_bond`, `k_elim`, `vigesimal`, `gini`, `ramanujan`, `garner`, `katun`, `winding`

Absent (mentioned in vault docs but no implementation):
`new_year`, `homomorphic`, `deep_time`, `range_alias`, `lane_null`

Present only as comments/stubs:
`arcsecond`, `chaak`, `serpent`, `biological`

---

## 6. What Mainstream Scholarship Actually Says `[SCHOLARLY]`

From the Moon Goddess research file (which cites real academics):
- Pages 16–23 contain the final twenty almanacs of the first 52 almanacs in the codex
- The deity is **Goddess I** (Ixik Kab, earth/fertility) and **Goddess O** (Chak Chel, aged creator crone) — **two distinct deities**, not a single Moon Goddess
- Thompson's single-Moon-Goddess synthesis (dominant 20th century) is challenged by Taube (1992), Vail & Stone (2002), Ardren (2006)
- Distance numbers 177, 178, and 148 connect Moon Goddess almanacs to the eclipse table — **this IS established** `[SCHOLARLY]`
- Eclipse table covers 11,960 days (405 synodic months) — **this IS established** `[SCHOLARLY]`
- The CRAM/computer interpretation is not asserted by any cited scholar; it appears only under a single "researcher note" in the vault file

---

## 7. Vault Document Claims `[UNRELIABLE — AI-PENNED SOURCE]`

The following were written by AI in prior sessions. Not established findings.

### 7a — Core hypothesis
The Dresden Codex is a hand-operated exact-integer residue machine. The Maya tracked celestial cycles as independent remainders against {2,3,5,7,11,13} (CRAM lanes), used red/black numbers for fraction-free winding extraction (K-elimination), organized all 74 pages as functional machine domains, and treated prime 11 as a hidden shadow coordinate.

### 7b — 22 named algorithms from Decoded.md

1. CRT substrate — decompose epoch into 6 lane residues
2. K-elimination engine — exact winding extraction (red/black numerals on p.52a)
3. Garner reconstruction — Venus tables as inverse CRT map
4. Mars-Venus shadow bond — 11-lane zeros as alignment events
5. Vigesimal injection — bar-dot numerals as CRAM lane inputs
6. Selective lane nullification — 78-day stride silences {2,3,13} lanes
7. Zero-drift epoch traversal — serpent numbers as torus addresses mod 30,030
8. Divisibility enhancement — Hardy-Littlewood prime constellation density
9. T-SHADOW-POWER — Saturn 242 = 2×11² deep space metric
10. Range aliasing — K'atun prophecy as torus wrap
11. Prime role taxonomy — {5=surface, 7=bridge, 11=shadow, 13=boundary}
12. Coprime lattice stratification — Gini coefficient prime-rich vs prime-poor
13. Range-stable transfer — 32,000-year proof via integer torus
14. O(1) consistency verification — SafeAnchor residue-space error checking
15. Biospheric resonance filter — 780-day Chaak as 2-lane processor ({7,11} only)
16. State vector synchronization — New Year ceremonies as SafeAnchor protocol
17. Biological modular interference — Moon Goddess medical almanacs as phase matching
18. Homomorphic state operations — planetary states added in residue space
19. Winding underflow — pre-creation dates as negative torus traversal
20. Arcsecond projection — spatial alignments as integer arcsecond residues
21. Ramanujan gate — S_R = {5,7,11} as codex filter
22. Dresden correction method — calendar drift absorbed by torus periodicity

### 7c — Page-by-page functional model (HULTA report)

| Pages | Vault label |
|---|---|
| 1–23 | Biological domain (260-day Tzolk'in, ritual almanacs) |
| 24 | Venus-preface "bootloader" — state vector handoff buffer |
| 25–28 | New Year re-initialization ("register flush") |
| 29–45 | 780-day Mars stride nullification (2-lane processor) |
| 46–50 | Venus torus execution (37,960-day macro-modulus) |
| 51–58 | Lunar K-elimination engine (148/177 intervals) |
| 59–61 | Multi-threaded synchronization |
| 62–73 | Serpent numbers — infinite-scaffold traversal |
| 74 | Great Deluge — total register flush / system reset |

### 7d — Arithmetic claims (partially reliable)

These are arithmetically true. The interpretations placed on them are what is unreliable.

| Claim | Arithmetic | Interpretation |
|---|---|---|
| 37,960 = 65×584 = 104×365 = 146×260 | TRUE | [UNRELIABLE] |
| Calendar Round = lcm(260,365) = 18,980 | TRUE | [UNRELIABLE] |
| 78 = 2×3×13 → nullifies {2,3,13} lanes | TRUE | [UNRELIABLE] |
| 780 = 2²×3×5×13 → nullifies {2,3,5,13} lanes | TRUE | [UNRELIABLE] |
| Saturn displacement 242 = 2×11² | TRUE | [UNRELIABLE] |
| 819 = 3²×7×13 (does NOT support Ramanujan congruence) | TRUE | [UNRELIABLE] |
| 1448 mod 260 = 148 | TRUE | [UNRELIABLE] |
| 260 = 2²×5×13 → Tzolk'in nullifies {2,5,13} lanes | TRUE | [UNRELIABLE] |

---

## 8. Evidence Register — Class A and Class B

### Class A — Arithmetic predictions (computable; currently circular)

| # | Claim | Status |
|---|---|---|
| A1 | Cycle factorizations: 260=2²·5·13; 365=5·73; 584=8·73; 780=2²·3·5·13 | TRUE (trivial) |
| A2 | Grand sync: 37,960 = 65·584 = 104·365 = 146·260 | TRUE |
| A3 | Calendar Round = lcm(260,365) = 18,980 | TRUE |
| A4 | Stride nullification: 78 zeros {2,3,13}; 780 zeros {2,3,5,13} | TRUE |
| A5 | Shadow displacements: Saturn 242 = 2·11²; Mars-Venus 11-lane zeros | TRUE given inputs |
| A6 | Ramanujan congruences exist only for {5,7,11} (Ahlgren–Ono 2000) | TRUE (upstream theorem) |
| A7 | 1448 mod 260 = 148 | TRUE (trivial) |

**These are facts about numbers the code was handed. They do not require the codex.**

### Class B — Empirical evidence (must be measured; NOT yet built)

| # | What to measure | Strongest test? |
|---|---|---|
| B1 | Bar-dot numeral OCR — read painted values; check they match constants | ★★ (internal) |
| B2 | Red vs black ink fraction per page/band | measurable |
| B3 | Shannon entropy + ink density per page | measurable |
| B4 | Red horizontal barrier detection | partially built |
| B5 | OCR the 148/177/178 distance numbers; verify sum = 11,960 | ★ |
| B6 | Real pixel-based figure classification (not page-number lookup) | critical gap |
| B7 | SLUB/FAMSI ratio per WWII-damaged page (two-modes finding) | partially built |
| B8 | Compare decoded dates against real astronomical ephemeris | ★★ (external) |
| B9 | Statistical null test — same checks on random/invented period sets | not built |

**B1 and B8 are the strongest tests.** B8 cannot be faked by copying — if the predicted intervals hit real historical eclipses, that is genuine evidence.

---

## 9. Tooling Failures and Evidence Contamination

### 9a — PDF reading failure `[UNAVAILABLE]`
Three PDFs uploaded by user. Text extraction failed in this environment via every available method (`pdftoppm`, `pdftotext`, Python `pypdf`, manual zlib decompression). Content of all three PDFs is unavailable until read by other means.

### 9b — Prior-session AI penmanship (the core contamination)

The core contamination: prior AI sessions penned documents (Decoded.md, dpm_prime.rs theorem stack, HULTA model, synthesis document) with confident, formal styling that made interpretive claims appear established. The code was then built to encode those same claims. Tests confirm the code matches the documents — not that the documents are true of the codex.

The user identified this and the v0.9 segmenter arc is the recovery mechanism. This research record confirms the contamination is real, characterizes its scope, and gives the Class-B test list needed to escape it.

---

## 10. Two-Modes WWII Damage Finding `[VERIFIED — v0.9.5 docs]`

From `docs/v0_9_5_findings.md`:

| Mode | Pages | Mechanism |
|---|---|---|
| A — net content loss | 24 | Ink dissolved/washed off; SLUB photograph reads near-blank |
| B — net content gain | 2, 4, 28, 34, 38, 71, 72 | Water stains/mould add spurious dark regions the segmenter misreads as ink |

Both modes are WWII damage. They produce **opposite** segmenter signatures. The v0.9.2 damage classifier (calibrated on page 24) catches only Mode A. The proposed fix (v0.9.6 candidate): a SLUB↔FAMSI ratio predicate instead of a SLUB-stats-only predicate.

---

## 11. What Is Built vs What Is Missing

### Built and working
- Exact-integer CRAM arithmetic (Safe Basis, K-Elimination, residue arithmetic)
- Closing-threshold and register-aware segmenters (real pixel operations)
- SLUB JPEG ingestion (`slub::load_slub_page`)
- SLUB↔FAMSI cross-source comparison (`compare_two_jpegs`)
- Barrier detection (red horizontal rows)
- Damage corroboration predicate (calibrated on page 24 only)
- Download scripts for 74 SLUB pages and 65/74 FAMSI plates
- EVIDENCE_AND_THESIS.md — thesis with honest caveat + Class A/B evidence register
- `DCCMS_IMPORTS_ROOT` env-var override for portable image paths

### Not built (the real evidence — Class B)
- Bar-dot numeral OCR (B1, B5)
- Red/black pigment measurement (B2)
- Shannon entropy / ink density per page (B3) — **Note: requires float arithmetic; prohibited by workspace invariant. Must use integer approximation or fixed-point when implemented.**
- Real pixel-based figure classification (B6)
- Astronomical cross-check against real eclipse ephemeris (B8)
- Statistical null test vs random period sets (B9)

**Note on entropy (B3):** Shannon entropy requires logarithms. The workspace prohibits float arithmetic. Any implementation must use a fixed-point or integer-approximation method (e.g. integer log₂ via bit-length, scaled integer arithmetic) — floats are not an option in this framework.

---

## 12. Files Created in This Session

| File | Description |
|---|---|
| `EVIDENCE_AND_THESIS.md` | Thesis statement + complete evidence register (Classes A and B) |
| `RESEARCH_RECORD.md` | This document — full session scrape with reliability markings |
| `dccms_atlas/src/paths.rs` | Updated: `DCCMS_IMPORTS_ROOT` env-var override added |

Files created and then removed (scanner idea scrapped; float arithmetic prohibited):
- `dccms_atlas/src/segmenter/scan.rs` (removed)
- `dccms_atlas/src/segmenter/crosscheck.rs` (removed)
- `dccms_atlas/examples/scan.rs` (removed)
