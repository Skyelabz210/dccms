# DCCMS implementation audit at `40a7615`

This audit describes the code that existed before this research-only branch.
It does not diminish the verified arithmetic modules; it identifies which
paths can currently serve as evidence for observation-only execution and which
remain demonstrators, scalar boundaries, or page-conditioned prototypes.

## Baseline

Command:

```text
cargo test --workspace --locked
```

Prior independent run: **669 passed, 0 failed** across the default three-crate
workspace. Optional SLUB/image and external proof-assistant execution were not
part of that count. This branch reruns the command before publication.

## Findings that affect theorem-to-system interpretation

### I1 — Scalar exact division is not an observation-only producer

File: `dresden_codex/src/k_elim_divide.rs`

`k_elim_divide(a,b)` accepts the full scalar dividend and divisor, checks
`a % b`, computes per-lane inverse products, and returns both a residue address
and `quotient = a / b`. Its residue tuple is correct for the admitted coprime,
exact-divisibility cases. It does not demonstrate recovery of an unknown
quotient from independently supplied residue observations.

**Action:** retain it as a scalar-boundary/reference implementation or add a
separate API whose inputs are authenticated lane readings plus an explicit
range/query contract.

### I2 — Recombinant state persists scalar winding and reconstructs with Garner

File: `dccms_atlas/src/engines/recombinant.rs`

`RecombinantState` stores `tuple_winding: u64`; `from_integer` computes it by
scalar division; `to_integer` and addition reconstruct the residue component
through an internal mixed-radix/Garner routine. This can be a useful exact
boundary representation, but it is not the no-persistent-`K`, no-reconstruction
compute path defined by the current CRAM verified pattern.

Independent probes also reproduced two boundaries:

- noncoprime lane sets can enter a modular-inverse assumption that is not valid;
- casting a lane product larger than `u64` truncates the product used by
  `from_integer`.

**Action:** distinguish boundary reconstruction from residue-compute APIs;
validate pairwise-coprime/LCM semantics and checked widths before construction.

### I3 — Recumbent and adelic views are scalar-derived

Files:

- `dccms_atlas/src/recumbent.rs`
- `dccms_atlas/src/adelic_index.rs`
- `dccms_atlas/src/venus_kernel.rs`

These structures are created from a scalar day count and retain per-modulus
quotients/windings. They show exact coordinate decompositions. They do not yet
show that a downstream value was produced solely from authentic bounded
observations. `AdelicIndex::verify` checks agreement of selected views, not all
fields of an externally supplied object.

**Action:** label scalar construction as an ingress boundary and add
producer-bound observation constructors for the residue-only path.

### I4 — The current H4 visual classifier remains page-conditioned

Files:

- `dccms_atlas/src/segmenter/classify.rs`
- `dccms_atlas/src/segmenter/pipeline.rs`
- `dccms_atlas/src/h4_visual/iconographic.rs`
- `dccms_atlas/tests/h4_real_pixel_discharge.rs`

The expected figure is selected through `IconographicFigure::from_page(page)`.
The test `h4_visual_path_independent_of_image_content` explicitly confirms that
the arithmetic path does not depend on pixel content. Independent all-white and
all-black 400x400 images both produced all nine CRAM matches; page assignment
still supplied the nine figure labels.

**Action:** do not cite this path as pixel-derived manuscript recognition.
Freeze a real segmentation/classification protocol, remove page identity as a
feature, and evaluate held-out pages plus blank/noise controls.

### I5 — Synthetic eclipse intervals are a fixture, not a transcription

The tested synthetic generator emits 74 intervals totaling 11,960 by
construction and includes an interval outside the commonly named 148/177/178
set. That is useful for arithmetic regression but cannot authenticate the
surviving manuscript table or choose between 11,959 and 11,960.

**Action:** maintain a separate, provenance-rich transcription dataset with
page/station/uncertainty fields and no target-forcing generator.

### I6 — Packed four-head addresses are compressed queries

File: `dccms_atlas/src/heads.rs`

The canonical heads use periods 260, 365, 18,980, and 144,000. Their full joint
phase period is 136,656,000, while the packed zero/nonzero signature occupies
24 bits. The packed signature therefore cannot be a full inverse of the phase
state. It may be sufficient for selected downstream queries.

**Action:** publish its equivalence classes and a query-sufficiency table rather
than calling the compressed address a lossless full-phase encoding.

### I7 — “Adelic” terminology is broader than the implemented type

The implemented object is a useful finite residue tuple plus a scalar witness.
Statements that the rationals are dense in the full adele ring need the usual
diagonal/quotient qualifications; a finite product of selected residue rings is
not by itself the full adele ring or profinite completion.

**Action:** name the implemented finite object exactly and reserve full adelic
claims for a defined mathematical construction.

## Positive implementation evidence

The audit also retained substantial working structure:

- exact integer/rational-only arithmetic on load-bearing paths;
- canonical period and Long Count engines with extensive tests;
- explicit typed errors in several division paths;
- source-separated research gates;
- full facsimile provenance in the repository;
- formal artifacts checked separately from historical interpretation; and
- testable lane, carry, star, and transduction primitives across the related
  repositories.

## Isolated repair not merged here

An isolated branch `research/bounded-lane-repair` at commit
`761f1abd8bf45ce36f119636b1130fc6aeaafae8` repaired a bounded lane-addition
overflow and added canonicalizing/strict constructors. It passed 671 tests and
received a narrow source review. It does **not** repair the recombinant
noncoprime/product-width issues or source authentication, and this synthesis
does not merge it implicitly.

## Integration rule

A production claim should name its exact code path:

```text
producer -> authenticated observations -> decoder -> bounded query -> consumer
```

Scalar reconstruction may exist at declared ingress/egress boundaries. It
should not be used as evidence that the internal residue path derived the same
answer without reconstruction.
