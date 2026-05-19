# v0.9.3 — "The Object diagram closes on real pixels"

**Date:** 2026-05-19 (planning)
**Source:** [executioner_dag.md § v0.9.3](../executioner_dag.md) gap analysis at HEAD `2498ef6`

## Thesis

v0.7.0 closed the H4 visual transducer **in theory**: the Five-Contract gates pass on `goddess_section_layout()`, the lift is exhibited at K = 6, 7, 8, 10, the fifth-operator coherence is certified. The diagram closes for **synthetic data shaped like Goddess pages**.

v0.9.0–v0.9.2 added the segmenter framework, the SLUB / FAMSI imagery, the empirical calibration, the cross-source corroboration. Real pixels are now on disk and segment correctly.

**The two pieces have never met.** No code path takes a real SLUB page, segments it, classifies the bboxes, and asks "does the predicted CRAM address sequence match `MoonGoddessProfile::compute()`?" The Object diagram has not closed on real pixels.

v0.9.3 is that single piece of integration work.

## Scope

In:
1. **`IconographicGlyphClassifier`** — first real `GlyphClassifier` (not bbox-content-blind). Maps bbox geometry + auxiliary pixel-darkness → `Option<IconographicFigure>` discretely.
2. **`segmenter::pipeline`** — wires `RegisterAwareSegmenter(ClosingThresholdSegmenter)` → `IconographicGlyphClassifier` → `h4_visual::layout::PageLayout` verification → `MoonGoddessProfile` discharge.
3. **`examples/decode_goddess.rs`** — runs the pipeline on SLUB pages 13–24 and reports the per-page corroboration table.
4. **Verification harness** — integration test asserting `pipeline.predicted_cram == MoonGoddessProfile::page_cram_addresses()` for pages 16–23 (the canonical Goddess section).

Out (deferred):
- Glyph classification beyond `IconographicFigure` (BarDotNumeral, DayNameGlyph, MonthGlyph all have existing alphabets but classifiers come later).
- Extension to non-Goddess pages of the codex.
- Per-glyph spatial accuracy beyond figure-class detection.

## Architecture sketch

```text
~/Agents/imports/slub_dresden/page_00000016.jpg
                              │
                              ▼
                  slub::load_slub_page()
                              │ ImageBuffer
                              ▼
              RegisterAwareSegmenter::new()    ← v0.9.2 calibrated thresholds
              (inner = ClosingThresholdSegmenter)
                              │ Vec<BoundingBox> + Vec<(u32,u32)> bands
                              ▼
                IconographicGlyphClassifier    ← NEW in v0.9.3 (N05)
                              │ Option<IconographicFigure>
                              ▼
                   h4_visual::layout::PageLayout::cumulative_addresses(page)
                              │ predicted [u64; 6]
                              ▼
              compare to MoonGoddessProfile::compute().page_cram_addresses[page-16]
                              │ integer equality
                              ▼
                          PageDecoding {
                              page: u8,
                              register_bands: Vec<(u32,u32)>,
                              figure_bboxes: Vec<BoundingBox>,
                              classified_figure: Option<IconographicFigure>,
                              expected_figure: IconographicFigure,    // from page lookup
                              predicted_cram: [u64; 6],
                              cram_match: bool,
                          }
```

The `cram_match` field is the v0.9.3 success metric. For pages 16–23 it should be `true`; for page 24 (BlankBridge) the classifier returns `None`, the predicted-CRAM is the cumulative-total-modulo-Safe-Basis (which is the same arithmetic whether or not a figure was found), so the equality should still hold but the `classified_figure` field will be `None`. Whether that's a failure or a pass depends on the test design — pinned in N08 gate.

## Load-bearing gaps this closes

| Gap | Severity | Closed by |
|---|---|---|
| G2 — no real glyph classifier | HIGH | N05 |
| G3 — segmenter ↔ atlas disconnected | HIGH | N06, N07, N08 |
| G4 — comparison uses plain threshold | MEDIUM | N03 |
| G6 — 34 compiler warnings | LOW | N04 |
| G7 — page 15 anomaly unresolved | LOW | N09 |
| G8 — no forward plan doc | LOW | this document |
| G9 — no CHANGELOG | LOW | N10 |

Gaps **NOT closed by v0.9.3** (left for v0.9.4+):
- **G1 — manifest stale** — closed in this v0.9.3 plan-write session (N01).
- **G5 — imagery only 13–24** — extending coverage requires a separate SLUB IIIF download pass. Not load-bearing for H4 real-pixel closure.

## Acceptance criteria

v0.9.3 ships when:

1. **N01 PASS** — `WORKSPACE_MANIFEST.md` reflects current state (already PASS this session)
2. **N05 PASS** — `IconographicGlyphClassifier` exists, ≥ 6 unit tests on synthetic bboxes
3. **N06 PASS** — `segmenter::pipeline::decode_goddess_page` exists, ≥ 4 unit tests
4. **N07 PASS** — `examples/decode_goddess.rs` runs end-to-end on SLUB 13–24
5. **N08 PASS** — integration test asserts `predicted_cram == MoonGoddessProfile` for pages 16–23
6. **Full suite ≥ 648 + (new tests) passing, 0 failing**
7. **A1 zero-float** preserved at every API boundary
8. **Object contract** preserved (`Option<Glyph>`, not confidence vector)

## Open decisions

Three locked-in defaults from the DAG (override before N05 execution if desired):

- **Q1** — Classifier consults pixels: bbox geometry + bbox-mean-darkness scalar.
- **Q2** — Classifier output: richer enum `{Figure(IconographicFigure), GlyphBlock, Numeral, BarrierFragment, Unknown}`.
- **Q3** — Figure-class bboxes hard-coded to band 0 (topmost register) only.

## Tier order (from DAG)

```
Tier 0 (parallel-ready, completed/in-flight this session):
  N01 — manifest refresh                   ← DONE this session
  N02 — this plan doc                       ← DONE this session
  N04 — warning cleanup
  N09 — page 15 diagnostic (adds Job 5 to calibrate.rs)
  N10 — CHANGELOG initial

Tier 1 (mechanical, single-session):
  N03 — comparison switches to closing seg

Tier 2 (load-bearing, single-session):
  N05 — IconographicGlyphClassifier

Tier 3 (sequential, may span sessions):
  N06 — segmenter::pipeline
    │
    ├─► N07 — examples/decode_goddess.rs
    └─► N08 — verification harness
```

Tier 0 nodes are independent — completing them all is a clean v0.9.3-prep commit.
Tier 1 is a tight 30-minute upgrade.
Tier 2+3 is the real v0.9.3 ship.
