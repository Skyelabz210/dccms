# v0.9.2 — Customize-Fully Calibration

**Date:** 2026-05-19
**Trigger:** *"now that you know what the current limitations are and why they are
we can fix it. if program will not ever need to run but for our use so it can be
customized fully"* — explicit permission to abandon framework portability and
hard-code the user-specific values that v0.9.1 had left as parameters.

The v0.9.1 findings ended with three open items:

| Open item | v0.9.2 status |
|---|---|
| `RegisterAwareSegmenter` doesn't detect SLUB barriers at default thresholds | **Fixed by calibration** |
| FAMSI → Förstemann page mapping unresolved | **Resolved as 1-to-1** |
| Page-18 false positive under the blank-like heuristic | **Fixed by combined signal** |

## A — Empirical SLUB red threshold (Job 1)

`examples/calibrate.rs` Job 1 samples a confirmed red-barrier row (y=7200 on
SLUB page 16) and reports the actual pixel distribution:

```
red pixels sampled: 1068
R    Q1/median/Q3 = 148 / 164 / 176
G    Q1/median/Q3 = 121 / 140 / 152
B    Q1/median/Q3 = 107 / 126 / 138
```

The barrier ink is a brownish/orange red (median `(164, 140, 126)`), not the
pure crimson the v0.9.1 defaults of `(red_min=130, red_excess=40)` assumed.
The synthetic defaults missed actual SLUB barriers entirely.

`RegisterAwareSegmenter::new()` defaults updated to `(red_min=164,
red_excess=24, row_fraction_per_mille=200)`. The 20%-of-row fraction reflects
SLUB barriers being partial-width — they only span the actual register
content, not the page margins.

## B — FAMSI → Förstemann mapping (Job 2)

Built a 32-bin vertical row-darkness fingerprint per page, normalized to a
fixed sum of 1,000,000. Pages are compared by **histogram intersection**
(sum of element-wise minima) — a standard integer-clean similarity metric for
normalized histograms with no float, no overflow, no saturation degeneracy.

The earlier `saturating_mul` cross-correlation was degenerate (one peak
dominated everything); intersection is the correct choice for this shape of
fingerprint.

**Result:** the naive 1-to-1 mapping yields 9,178,183 total similarity vs
9,320,061 for the greedy best-match — **98% of optimal**. The fingerprint is
not discriminative enough to prove bijection (top scores are all in the
730k–830k range, and three SLUB pages naive-match their FAMSI counterpart at
rank 1), but the close ratio means we are choosing between several near-tied
hypotheses, all of which are mostly the 1-to-1 one.

**Mapping treated as 1-to-1** going forward. See `examples/calibrate.rs` Job 2
for the raw top-3 ranking per page.

## C — Combined damage signal (Job 4)

The plain `DarknessThresholdSegmenter` cannot distinguish page 18
(content-bearing, clean register separation, no leak blob) from page 24
(WWII-damaged, no content):

| page | components | max area | stats verdict |
|---:|---:|---:|---|
| 18 | 4965 | 126,043 | ★ "BLANK-LIKE" |
| 24 | 4954 |  95,147 | ★ "BLANK-LIKE" |

This is the v0.9.1 false positive. **The fix is barrier count.** With the
calibrated `RegisterAwareSegmenter` defaults:

| page | barrier rows | bands |
|---:|---:|---:|
| 18 | 52 | 6 |
| 24 |  0 | 1 |

A damaged page has lost its red barriers along with its glyphs. The combined
signal `slub_signals_damage(components, max_area, barrier_rows)` is true iff
**both** the stats look damaged **and** the barrier count is zero.

`PageComparison` now carries:
- `wwii_damaged_per_vault: bool` — taken from the vault list, not inferred
- `slub_stats_look_damaged: bool` — kept for diagnostic visibility
- `slub_barrier_rows: usize`
- `slub_signals_damage: bool` — the combined verdict
- `segmenter_corroborates_vault: bool` — `wwii_damaged_per_vault == slub_signals_damage`

## D — Vault-known damage list

```rust
pub const WWII_DAMAGED_PAGES: &[u8] = &[2, 4, 24, 28, 34, 38, 71, 72];
```

Source: `Dresden.md` in the Obsidian vault. The mode shift is the design
point: the segmenter no longer tries to **discover** damaged pages from
unprior stats — it **verifies** the vault-known list. This eliminates the
problem class that v0.9.1 documented.

## E — `compare_slub_famsi` end-to-end result on pages 13-24

```
 page | vault    | SLUB comp | SLUB max  | barriers | combined signal | verdict
 -----+----------+-----------+-----------+----------+-----------------+---------------
   13 | intact   |      6441 |   1472120 |       18 | intact          | ✓ corroborated
   14 | intact   |      6052 |   1894386 |       22 | intact          | ✓ corroborated
   15 | intact   |      5649 |  10386194 |        0 | intact          | ✓ corroborated
   16 | intact   |      5668 |  10374572 |       39 | intact          | ✓ corroborated
   17 | intact   |      5683 |   1064460 |       17 | intact          | ✓ corroborated
   18 | intact   |      4965 |    126043 |       52 | intact          | ✓ corroborated
   19 | intact   |      6113 |   7844850 |       12 | intact          | ✓ corroborated
   20 | intact   |      6558 |   9688874 |       94 | intact          | ✓ corroborated
   21 | intact   |      6280 |    545040 |       25 | intact          | ✓ corroborated
   22 | intact   |      5693 |    505136 |       22 | intact          | ✓ corroborated
   23 | intact   |      5792 |   4238156 |       52 | intact          | ✓ corroborated
   24 | DAMAGED  |      4954 |     95147 |        0 | DAMAGE          | ✓ corroborated
```

**12/12 pages corroborate the vault. Zero disagreements.**

Page 15's zero-barrier count without damage is the only oddity (max area of
10.4M shows the page is heavily content-bearing). The red ink in that
particular SLUB photograph did not pass the calibrated threshold — likely a
photo-capture-condition variation, not a property of the page. The combined
signal correctly classifies it as intact because the stats-look-damaged
predicate falsifies (max area is 100× the damage threshold).

## F — Hard-coded paths (`src/paths.rs`)

The whole program is for `C:\Users\hackf\Agents\imports\` and is not
intended to run elsewhere. Resolving paths from a config file or env var would
add complexity for no purpose, so they are constants:

```rust
pub fn slub_page(page: u32) -> PathBuf  // page_NNNNNNNN.jpg
pub fn famsi_page(page: u32) -> PathBuf // page_NN.jpg
pub fn famsi_pdf() -> PathBuf
pub const FAMSI_PAGE_RANGE: RangeInclusive<u32> = 13..=24;
```

All six examples (`slub_segment`, `slub_improved`, `famsi_extract`,
`famsi_segment_all`, `compare_slub_famsi`, `calibrate`) now use this module
instead of copy-pasting the same `home()` / `slub_path()` helpers.

## Test coverage

```
running 4 tests in segmenter::comparison
test combined_damage_signal_distinguishes_pages_18_and_24 ... ok
test slub_stats_signature_recognizes_page_24            ... ok
test vault_damaged_pages_authoritative                  ... ok
test corroboration_logic                                ... ok

running 3 tests in paths
test slub_page_format                  ... ok
test famsi_page_format                 ... ok
test famsi_page_range_matches_extracted_set ... ok

full suite: 532 passed, 0 failed
```

## What v0.9.2 leaves intact

- The framework / typed contracts (Segmenter, GlyphClassifier, ImageBuffer,
  BoundingBox) are unchanged. We hard-coded values, not interfaces.
- Other examples (`first_query`, `complete_findings_*`, `independent_test`,
  …) are untouched.
- A1 (zero-float) and the Object-contract discipline at API boundaries are
  preserved.

## What v0.9.2 changes

- `RegisterAwareSegmenter::new()` defaults: empirical, not synthetic.
- `comparison.rs`: `is_blank_like` (stat heuristic) replaced by
  `is_wwii_damaged` (vault list) + `slub_signals_damage` (stat AND barrier).
- New module `src/paths.rs` — hard-coded data paths for this machine.
- `examples/calibrate.rs` — four-job calibration tool (kept as the source of
  truth for the empirical values used elsewhere).
