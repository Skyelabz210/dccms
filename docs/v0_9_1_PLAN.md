# v0.9.1 Plan — Three Areas, Complete Execution

**Per user direction:** all three areas covered, planned completely, completed entirely.

## Area A — Improved Segmenter

Move the segmenter from baseline (5,600 components/page, mixed under-/over-segmentation) toward glyph-block-level granularity. Two new `Segmenter` impls plugging into the existing trait.

### A-1: `ClosingThresholdSegmenter`

Morphological **closing** (dilate-then-erode) before connected-components. Merges bar/dot fragments that 4-connectivity splits.

```rust
pub struct ClosingThresholdSegmenter {
    pub inner: DarknessThresholdSegmenter,
    pub closing_radius: u32,    // default 4
}
```

Implementation: separable horizontal+vertical max-filter (dilate) then min-filter (erode). All u8 mask ops. Sub-second per 30 MP page.

### A-2: `RegisterAwareSegmenter<S: Segmenter>`

Detects red horizontal barriers (the codex's structural register dividers), partitions the page into bands between barriers, runs the inner segmenter on each band independently. Wraps any `Segmenter`.

```rust
pub struct RegisterAwareSegmenter<S: Segmenter> {
    pub inner: S,
    pub red_min: u8,           // R must exceed this absolutely
    pub red_excess: u8,        // R must exceed G and B by this much
    pub row_fraction_per_mille: u32,  // barrier-row threshold (parts per 1000)
}
```

Output bboxes are translated back to the original image's coordinate system.

## Area B — Iconographic Verification

A typed classifier-style API that consumes segmenter output + page number and verifies whether the expected `IconographicFigure` (per H4 visual transducer) has a structural footprint in the bboxes.

### B-1: `PageContextClassifier`

Maps page number to expected `IconographicFigure` via existing `from_page`. Tests page-16 → MoonSign etc.

```rust
pub struct PageContextClassifier { pub page: u8 }

impl GlyphClassifier for PageContextClassifier {
    type Glyph = IconographicFigure;
    fn classify(&self, image: &ImageBuffer, bbox: BoundingBox) -> Option<IconographicFigure>;
}
```

### B-2: `verify_page_iconography(page, bboxes) -> VerificationReport`

Reports:
- Expected figure for page
- Count of "figure-class" bboxes (large enough to be the main figure, aspect ratio in `[0.5, 2.0]`)
- Largest figure-class bbox's position + size
- Whether bbox-count is non-trivially > 0 (sanity)

Used in tests against real SLUB pages: each page 16-23 should report ≥ 1 figure-class bbox; page 24 should report 0 (consistent with `BlankBridge`).

## Area C — FAMSI Förstemann-Schele Comparison

Download the FAMSI page-13-24 chromolithograph PDF, extract page 24 (the WWII-damaged page in SLUB), and run structural comparison.

### C-1: Download

`https://www.famsi.org/mayawriting/codices/pdf/2_dresden_fors_schele_pp13-24.pdf` (16 MB, public domain by age). Stored at `~/Agents/imports/famsi_dresden/`.

### C-2: PDF→image extraction

Strategy: enumerate PDF tooling already available in our env (Git Bash, PowerShell):
- Check for `pdftoppm` (poppler) — most likely available
- Check for `mutool` (mupdf)
- Fallback: use a lightweight Rust crate (`lopdf` for parsing + raw stream extraction)

If extraction succeeds, page 24 becomes a JPEG/PNG we load via our existing `ImageBuffer`. If not, document the comparison framework and the extraction blocker honestly per discipline.

### C-3: Side-by-side comparison

```rust
pub struct PageComparison {
    pub slub_components: usize,
    pub famsi_components: usize,
    pub slub_max_area: u64,
    pub famsi_max_area: u64,
    pub same_page_index: bool,
}

pub fn compare_slub_vs_famsi(page: u8) -> Result<PageComparison, ComparisonError>;
```

Tests verify that page 24's comparison shows BOTH sources at low component counts (the WWII-damaged page is blank in both representations — independent confirmation across photograph AND chromolithograph).

## Build order

```
A-1 closing.rs   ─┐
A-2 register.rs  ─┼─→ B classifier ─→ C comparison ─→ tests + commit
```

## A1 / discipline

- All new code: zero floats at API boundaries.
- Tests use synthetic data; real-imagery tests skip cleanly if files absent.
- Provenance for FAMSI extraction documented honestly (which tool, which fallback if any).
- The `verify_page_iconography` API does NOT silently classify — it returns a `VerificationReport` the caller interprets.

## Projected tests: +20 minimum
