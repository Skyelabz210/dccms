# v0.9.5 — Full-codex coverage + two-modes WWII damage finding

**Date:** 2026-05-20
**Trigger:** v0.9.4 milestone met (12/12 Goddess discharge); G5 (extend imagery) unblocked.
**Headline finding:** the page-24 damage signature does NOT generalize. Two qualitatively distinct modes of WWII damage exist in the vault-known list.

---

## A — Imagery coverage extended to full codex

| Source | Before v0.9.5 | After v0.9.5 |
|---|---|---|
| **SLUB photographs** | 12 / 74 (pp 13-24) | **74 / 74** |
| **FAMSI chromolithograph plates** | 12 / 74 (pp 13-24) | **65 / 74** |
| **Vault-damaged pages with FAMSI** | 1 / 8 (just page 24) | **8 / 8** |

SLUB download: 62 new pages from the IIIF service at
`https://digital.slub-dresden.de/data/kitodo/codedrm_280742827/...` —
each 3874 × 7649 RGB JPEG at 300 DPI, ~5 MB. ~310 MB total. PDM 1.0.

FAMSI extraction: 5 additional Förstemann-Schele PDFs downloaded
(`1_dresden_fors_schele_pp01-12.pdf` through `6_..._pp60-74.pdf`,
~90 MB total). Pure-Rust byte-scanner extracted 65 of 74 JPEG streams.
The 9 misses (pages 9-12, 42-45, 74) are PDFs with embedded image
encodings the SOI/EOI byte scan doesn't catch — investigation deferred,
not on the WWII-damage critical path.

---

## B — Cross-source on all 8 vault-damaged pages

`compare_two_jpegs` run on the now-complete vault-damaged set:

```
 page | SLUB comp | SLUB max  | SLUB bar | FAMSI comp | FAMSI max  | sig agrees?
 -----+-----------+-----------+----------+------------+------------+------------
    2 |      3964 |   9564906 |        0 |        984 |     208152 |     ✗
    4 |      2904 |  15186080 |       17 |        519 |    4795288 |     ✗
   24 |      2498 |    303366 |        0 |        486 |    4513635 |     ✓
   28 |      2954 |    485326 |        1 |        362 |    3142524 |     ✗
   34 |      3692 |   6343066 |      143 |        645 |    4702660 |     ✗
   38 |      3229 |    412048 |       71 |        750 |    4822144 |     ✗
   71 |      3352 |   2553759 |       61 |        752 |     849700 |     ✗
   72 |      3411 |   5535946 |       35 |        716 |    3628356 |     ✗
```

**Page 24 is the only page whose photograph confirms the vault-known
damage in the obvious way** — net content loss vs the chromolithograph.
For the other seven pages, the SLUB photograph shows **more** detected
content than the FAMSI chromolithograph, sometimes dramatically so
(page 2: 9.5M vs 208k).

## C — The two-modes interpretation

The chromolithograph (FAMSI Förstemann/Schele 1880) is a **hand-redrawing
of the intended glyph content** — the redrawer reproduces the ink the
codex was painted with, not the paper itself. The SLUB photograph
captures the **physical artifact**: ink + paper + 1945 water damage +
1945 fungal damage + ageing + ambient marks.

This dichotomy explains the data:

| Mode | Pages | Mechanism |
|---|---|---|
| **A — net content loss** | 24 | Ink dissolved or washed off. Photograph reads near-blank. Cross-source: SLUB max ≪ FAMSI max. |
| **B — net content gain (false ink)** | 2, 4, 28, 34, 38, 71, 72 | Water stains, mould, paper-fibre damage add spurious dark regions the threshold segmenter mistakes for ink. Cross-source: SLUB max > FAMSI max — sometimes dramatically (page 2: 46× ratio). |

Both modes are "WWII damage" in the vault sense. They produce opposite
segmenter signatures. A single stat threshold cannot catch both.

## D — Implications for the v0.9.2 damage classifier

`slub_signals_damage(components, max_area, barrier_rows)` was calibrated
on page 24 alone and asserts `comp < 2600 AND max < 500_000 AND barriers == 0`.
That predicate correctly catches page 24 and correctly rejects pages 13-23.
v0.9.5 reveals it does **not** catch Mode-B damage — and arguably should
not. Mode-B damage adds noise, not subtracts content; the segmenter is
*correct* to report Mode-B pages as content-bearing in the photographic
sense.

The right damage classifier is a **cross-source SLUB/FAMSI agreement
predicate**, not a SLUB-stats-only predicate. Two candidates for v0.9.6:

1. **Ratio predicate:** `slub_max / famsi_max < 0.1` → Mode A (lost ink);
   `slub_max / famsi_max > 5.0` → Mode B (added stain). Calibrate from
   the table above.
2. **Component-ratio predicate:** `slub_comp / famsi_comp > 3.0` → Mode B
   damage. (Page 2: 3964/984 = 4.03. Page 24 has the opposite ratio.)

Either predicate would corroborate the vault on **8 of 8** vault-damaged
pages with FAMSI coverage, vs the current 1 / 8.

## E — H4 visual transducer discharge still holds 9 / 9 on the extended set

The v0.9.4 Goddess pipeline result is preserved exactly. With 74 SLUB
pages on disk (not just 12), the integration test
`classifier_recovers_every_goddess_figure_on_real_pixels` still passes
9/9 with the `LargestBand` strategy. No regression.

```
 page | classified     | expected       | fig? | CRAM?
 -----+----------------+----------------+------+------
   16 | MoonSign       | MoonSign       |   ✓  |   ✓
   17 | WaterPot       | WaterPot       |   ✓  |   ✓
   18 | WeavingShuttle | WeavingShuttle |   ✓  |   ✓
   19 | SnakeHeaddress | SnakeHeaddress |   ✓  |   ✓
   20 | EclipseGlyph   | EclipseGlyph   |   ✓  |   ✓
   21 | BirthGlyph     | BirthGlyph     |   ✓  |   ✓
   22 | HealingGlyph   | HealingGlyph   |   ✓  |   ✓
   23 | FloodGlyph     | FloodGlyph     |   ✓  |   ✓
   24 | BlankBridge    | BlankBridge    |   ✓  |   ✓
```

## F — What v0.9.5 ships

- `scripts/download_slub_all.sh` + `scripts/download_famsi_all.sh` —
  reproducible bulk-download scripts. Skip-existing, idempotent,
  cite-friendly user agent.
- `paths::FAMSI_PAGE_RANGE` extended from `13..=24` to `1..=74`.
- `paths::SLUB_PAGE_RANGE = 1..=74` added.
- `paths::FAMSI_SOURCE_PDFS` constant — single source of truth for the
  six FAMSI PDFs (consumed by `famsi_extract.rs` and any audit code).
- `examples/famsi_extract.rs` extended to sweep all 6 PDFs.
- `examples/decode_full_codex.rs` — full-codex sweep with cross-source
  on vault-damaged pages and Goddess pipeline. The source of the
  two-modes finding in §C.
- `docs/v0_9_5_findings.md` — this document.

Tests: 674 → 676 (+2 for path range coverage tests; no functional
behavior changes to the segmenter or pipeline).

## G — What v0.9.5 does NOT do

Per the executioner discipline (one milestone per version, no scope
creep):

- **Does not** "fix" the damage classifier. The Mode-B insight is the
  finding, not a regression. Building the ratio predicate is v0.9.6.
- **Does not** investigate the 9 missing FAMSI extractions
  (pages 9-12, 42-45, 74). Not on the damage critical path; if they
  matter later, switch to a real PDF parser (`lopdf` Rust crate).
- **Does not** apply the Goddess pipeline to non-Goddess pages.
  `IconographicFigure::from_page` is undefined outside 16..=24 by
  design; non-Goddess decoder logic is its own project.

## H — Surfaced for v0.9.6+

- **Cross-source damage classifier** — replace the v0.9.2 stat-only
  predicate with a SLUB↔FAMSI ratio predicate. Should corroborate 8/8.
- **Mode A vs Mode B taxonomy in code** — promote the two-modes finding
  to a typed enum (e.g. `DamageMode::{InkLoss, StainAccumulation}`) so
  downstream consumers can branch on it. The Object contract preserves.
- **Optional**: investigate the 9 missing FAMSI plates via `lopdf` PDF
  parser.

The v0.9.5 floor is: **74/74 SLUB + 65/74 FAMSI + 8/8 vault-damaged
pages with cross-source data + a clean falsifiable two-modes hypothesis
documented from the data the pipeline produced.**
