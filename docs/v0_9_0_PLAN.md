# v0.9.0 Plan — There Is No Outside Scope

**Per executioner Directive 0:** when analysis hits an apparent limit, convert the limit into a build target. Specify the instrument; build it; run it; report. The only acceptable "cannot" is demonstrated impossibility.

In v0.8.0 I listed four items as "outside scope." That was a category error — none of them are. Each is a buildable instrument with the math fully specified in the vault:

| Previously bracketed | Actually | Source |
|---|---|---|
| Recombinant CRT for Long Count > M_SAFE | Vault §07 fully specifies the winding-counter algebra | `07_recombinant_crt_winding.md` |
| Canonical K-Elim winding-extraction | Vault §06 fully specifies the phase-differential operation | `06_k_elimination_theorem.md` |
| Pages 13c-15 figure decoder | Needs DATA, not theory — the framework that consumes the data is buildable now | structural |
| SEG02/SEG03 pixel ingestion | Needs IMAGES, not pixels — the segmenter framework over arbitrary byte buffers is buildable now | structural |

## Phase C-1 — Recombinant CRT (`dccms_atlas/src/engines/recombinant.rs`)

Per vault §07: lossless integer accumulation via winding-counter tracking. Operations preserve `x = r + K·M` exactly.

```rust
pub struct RecombinantState {
    state: MayaState,
    tuple_winding: u64,    // K = ⌊x / M⌋ where M = product of lane moduli
}

impl RecombinantState {
    pub fn from_integer(x: u64, lanes: Vec<Lane>) -> Result<Self, _>;
    pub fn to_integer(&self) -> Result<u128, _>;        // exact, u128 for headroom
    pub fn modulus(&self) -> u128;                        // M = ∏ lane.modulus
    pub fn add(&self, other: &Self) -> Self;
    pub fn add_integer(&self, n: u64) -> Self;
    pub fn winding(&self) -> u64;
}
```

**Tests:** round-trip for values both below and above M; advance through Long Count (1,872,000 days, winding ≥ 62 over M_SAFE = 30,030); addition associativity; winding monotone-increasing under positive additions.

## Phase C-2 — K-Elim Division (`dresden_codex/src/k_elim_divide.rs`)

Per vault §06: exact RNS division for the coprime case `gcd(b, M) = 1, b | a`.

```rust
pub struct KElimResult {
    pub quotient_address: [u64; 6],
    pub witness_winding: u64,    // extracted K
}

pub enum KElimError {
    NotCoprime,                  // gcd(b, M) > 1
    DivisorZero,                 // b == 0
    NotDivisible,                // b ∤ a (verified after the fact)
}

pub fn k_elim_divide(a: u64, b: u64) -> Result<KElimResult, KElimError>;
```

**Tests:** divide canonical Maya cycles. `11960 / 260 = 46`, `37960 / 584 = 65`, `1872000 / 144000 = 13`, etc. Verify quotient by multiplying back. Reject non-coprime divisors. Reject non-divisible cases.

## Phase C-3 — Extended Goddess Decoder Framework (`dccms_atlas/src/extended_goddess.rs`)

Build the **consumer** of glyph-spec data for pages 13c-15. When Barnhart 2005 data arrives, plug it in. Until then, the framework runs against synthetic test data and structurally-verified placeholder specs.

```rust
pub struct GlyphSpec {
    pub page: u8,
    pub register: PageRegister,    // a/b/c/d sub-page registers
    pub position: u8,
    pub kind: GlyphKind,
    pub bar_dot_value: Option<u8>,
}

pub enum PageRegister { A, B, C, D }
pub enum GlyphKind {
    DayName(DayNameGlyph),
    Numeral(u8),
    Figure(/* TBD */),
}

pub fn decode_extended_section(specs: &[GlyphSpec]) -> ExtendedSectionAnalysis;
```

**Tests:** synthetic spec for pages 13c-15 round-trips; the structural extension `[13..=23]` is reachable through the API; an empty spec yields a typed "no data" response (not a panic).

## Phase C-4 — Pixel Ingestion Framework (`dccms_atlas/src/segmenter/`)

Build the **framework** for pixel→glyph segmentation without committing to a specific image format. Operates over `&[u8]` byte buffers with explicit width/height. Real imagery downloads (SLUB Dresden) become drop-in adapters later.

```rust
// segmenter/mod.rs

pub struct ImageBuffer {
    pub width: u32,
    pub height: u32,
    pub bytes_per_pixel: u32,
    pub data: Vec<u8>,
}

pub struct BoundingBox {
    pub x: u32, pub y: u32,
    pub w: u32, pub h: u32,
}

pub trait Segmenter {
    fn segment(&self, img: &ImageBuffer) -> Vec<BoundingBox>;
}

pub trait GlyphClassifier {
    type Glyph;
    fn classify(&self, img: &ImageBuffer, bbox: BoundingBox) -> Option<Self::Glyph>;
}

// segmenter/null.rs — trivial implementations for testing
pub struct NullSegmenter;
pub struct NullClassifier;
```

**Tests:** ImageBuffer round-trip; bbox math (intersect, contains, area); NullSegmenter on a synthetic buffer returns predictable output; the Object-contract from h4_visual is preserved (segmenter output is discrete `Vec<BoundingBox>`, classifier output is discrete `Option<Glyph>` — no float crosses the API).

## Build order

```
C-1 Recombinant CRT          ─┐
C-2 K-Elim Division          ─┼─→ independent
C-3 Extended Goddess         ─┤
C-4 Pixel Ingestion          ─┘
```

All four are independent. Ship together; commit at the end.

## Test count projection

| Phase | Estimated tests |
|---|---:|
| C-1 Recombinant CRT | 8 |
| C-2 K-Elim Division | 8 |
| C-3 Extended Goddess | 5 |
| C-4 Pixel Ingestion | 7 |
| **Total** | **28** |

Projected: 588 → ~616.

## A1 compliance

All four phases are pure integer arithmetic:
- Recombinant CRT: winding counters are u64; reconstruction uses u128 for overflow headroom.
- K-Elim Division: only mod, mul, gcd, modular inverse.
- Extended Goddess: pure typed-data manipulation.
- Pixel Ingestion: u8 byte buffers; u32 coordinates; no floats anywhere.
