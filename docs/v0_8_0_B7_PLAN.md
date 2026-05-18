# v0.8.0 Tier 2 — Phase B-7 Plan

**Status:** PROPOSAL (awaiting user sign-off)
**Goal:** Give Tier 2 (B-5, B-6) and Tier 3 (B-8, B-9) a stable type vocabulary by porting the canonical operator-fabric specification from vault `Mayas Engine.md` into dccms, adapted to the project's A1 / no-predicate-drift discipline.

## 1. Scope

Per user direction: B-7 must produce *enough* type vocabulary that B-5 (shadow_bond) and B-6 (ramanujan_partition) can consume it cleanly. It does **not** need to mechanize all five engines from the vault — that can phase out across later commits. The minimum useful B-7 is:

- The two core types (`Lane`, `MayaState`) with their six methods
- Helper primitives the type methods depend on (`pisano_period`, `fibonacci_entry_point`, `fibonacci_mod_sequence`, `ramanujan_sum` — small-prime versions sufficient for our basis range)
- **Two reference engines** that exercise the type vocabulary end-to-end and provide working examples for B-5/B-6:
  - **Vigesimal** (small, illustrates the heterogeneous-lane pattern `ℤ/4 × ℤ/5`)
  - **Tzolkin** (medium, reconciles with existing `dayname.rs` and uses both lanes)
- Tests for everything new

LongCount / DresdenEclipse / VenusTable / MayaFabric **defer to later phases** (B-7.3 .. B-7.6 — separate commits when B-5 and B-6 are done and we have proven the vocabulary works for them).

## 2. Adaptation decisions — choices the user should approve before any code

These are the precision-hardening decisions that must be made up-front to avoid predicate drift downstream.

### D-1. A1 violations in the vault spec — replacement strategy

The vault's pseudo-Rust contains float arithmetic in four places. None survive into dccms; each needs an integer/rational replacement.

| Vault location | Float use | Replacement |
|---|---|---|
| `MayaState::pisano_profile` line 78 | `distinct.len() as f64 / lane.modulus as f64` returning `f64` coverage ratio | Return `(distinct: u64, modulus: u64)` pair; let the caller compute ratios in its own representation |
| `VenusTable::phi_approximation` lines 565-571 | `1.618033988749895`, `(ratio - phi).abs()` | Return `(num: u64, den: u64)` for the integer ratio (8/5); compute φ-error as `|num · 1000 - den · 1618|` or similar exact-integer scaled form, OR drop this method as ornamental |
| `VenusTable::correction_schedule` print line 770 | `days as f64 / 365.25` | Print as integer days; no float |
| `print_engines_report` line 698 | `as f64 / 20.0 * 100.0` | Print `distinct.len()` over `20` as a fraction or `(num, den)` pair |

**Recommendation:** drop `phi_approximation` from the port entirely — it's an ornamental aesthetic claim about Venus/Earth ≈ φ. The arithmetic content (8/5 sync, 2920-day LCM) is preserved in the other methods.

### D-2. Out-of-scope methods per synthesis register

| Vault method | Why omit |
|---|---|
| `Tzolkin::above_consciousness_threshold` | Implements the φ³ consciousness threshold (synthesis O4 — flagged do-not-mechanize) |
| The "tone ≥ 4 = consciousness" framing | Same |

The Tzolkin lane structure stays; only the φ³ interpretation gets cut.

### D-3. Day-name orthography reconciliation

This is the one decision that needs an explicit pick. The two sources disagree on 11 of 20 day names:

| Vault `Mayas Engine.md` | dccms `dayname.rs` | Difference |
|---|---|---|
| Ik | IkPrime | typographic (`'` vs `Prime`) |
| Chicchan | Chikchan | c/k mix |
| Cimi | Kimi | c/k |
| Muluc | Muluk | c/k |
| Oc | Ok | c/k |
| Chuen | Chuwen | u/uw |
| Cib | Kib | c/k |
| Caban | Kaban | c/k |
| Cauac | Kawak | c/k, u/w |
| Ahau | Ajaw | h/j, u/w |
| (others identical: Imix, Akbal, Kan, Manik, Lamat, Eb, Ben, Ix, Men, Etznab) | — | — |

These reflect the historical "Yucatec colonial" orthography (Mayas Engine.md) versus the modern unified Mayan orthography (dccms — what scholars now use). The modern spellings are the academic standard since the 1980s revision. **dccms is already correct.**

**Recommendation:** keep dccms's modern Mayan spellings as canonical. Document the colonial-orthography mapping for cross-referencing the vault but do not regress dccms. The B-7 `engines::tzolkin::DAY_SIGNS` constant should re-export `dayname::ALL_DAY_NAMES` (or use the same string identifiers), not introduce a new spelling.

### D-4. Type-shape decision: fixed array vs Vec

The vault uses `Vec<u64>` for `MayaState::residues` and `Vec<Lane>` for `MayaState::lanes`. Our existing CRAM addresses use `[u64; 6]` (fixed Safe Basis array). The B-7 types need to handle both:

- **Heterogeneous fabrics** (Vigesimal `ℤ/4 × ℤ/5`, Tzolkin `ℤ/13 × ℤ/20`) — variable lane count and moduli per engine → `Vec<Lane>` is natural
- **Safe Basis canonical** — fixed 6-lane `[2,3,5,7,11,13]` → `[u64; 6]` is what existing code uses

**Recommendation:** B-7's `MayaState` uses `Vec<u64>` / `Vec<Lane>` to faithfully port the vault spec. A separate adapter `MayaState::from_cram_address(addr: [u64; 6])` and `MayaState::to_cram_address() -> Option<[u64; 6]>` bridges to our existing types when the basis is the canonical Safe Basis. No silent type-widening.

### D-5. Pisano + Ramanujan helpers — scope

The vault depends on `pisano_period(m)`, `fibonacci_entry_point(m)`, `fibonacci_mod_sequence(m)`, `ramanujan_sum(m, n)`. dccms has none of these. We need to build minimal versions.

**Recommendation:** build them with a strict scope clamp — supports `m ≤ 100,000` (covers all Maya cycles we care about: 260, 365, 584, 780, 1448, 11960, 18980, etc.). For larger `m`, return `Err`. Naive O(m·π(m)) Fibonacci computation is fine at that scale — no need for matrix exponentiation. A1-compliant by construction (only integer ops).

### D-6. tzolkin_at_eclipse signature cleanup

Vault's `DresdenEclipse::tzolkin_at_eclipse(reference_tzolkin: &MayaState, eclipse_number: u64) -> MayaState` ignores `eclipse_number` entirely (line 470: just clones the reference, because Tzolkin is invariant across eclipse periods).

**Recommendation:** drop the unused parameter when we port to `dccms_atlas::engines::dresden_eclipse` (in a later phase). The fact that Tzolkin is invariant is the actual content; an unused arg is decoration.

## 3. File layout

```
dccms_atlas/src/
├── engines/
│   ├── mod.rs           ← re-exports, module-level doc
│   ├── lane.rs          ← Lane + MayaState + 6 methods
│   ├── pisano.rs        ← pisano_period, fibonacci_entry_point, fibonacci_mod_sequence
│   ├── ramanujan.rs     ← ramanujan_sum
│   ├── vigesimal.rs     ← Engine 1 (Vigesimal)
│   └── tzolkin.rs       ← Engine 2 (Tzolkin) — bridges to existing dayname.rs
```

LongCount / DresdenEclipse / VenusTable / MayaFabric come in **subsequent B-7.3 — B-7.6 commits**, not in this initial B-7.0–B-7.2 landing.

## 4. Type signatures (B-7.0 — foundation)

```rust
// engines/lane.rs

/// A lane in the operator fabric: modulus + operator pipeline.
///
/// This is the minimal description of one coprime component in a
/// heterogeneous CRT fabric. `modulus` is the lane's prime or prime
/// power; `name` is a stable identifier; `domain` describes what the
/// lane semantically tracks (free-form metadata, not load-bearing).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Lane {
    pub name: &'static str,
    pub modulus: u64,
    pub domain: &'static str,
}

/// A Maya state as a CRT residue tuple — heterogeneous lane count.
///
/// Distinct from `dresden_codex::cram_address` which returns a fixed
/// `[u64; 6]` for the canonical Safe Basis. `MayaState` supports
/// arbitrary lane configurations including the heterogeneous engines
/// (Vigesimal `[ℤ/4, ℤ/5]`, Tzolkin `[ℤ/13, ℤ/20]`, etc.).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MayaState {
    residues: Vec<u64>,
    lanes:    Vec<Lane>,
}

impl MayaState {
    /// Construct from explicit residues + lanes. Returns Err if the
    /// vectors have different lengths or any residue exceeds its modulus.
    pub fn new(residues: Vec<u64>, lanes: Vec<Lane>) -> Result<Self, MayaStateError>;

    /// Accessors.
    pub fn residues(&self) -> &[u64];
    pub fn lanes(&self) -> &[Lane];
    pub fn lane_count(&self) -> usize;

    /// Advance by `days` — pure modular arithmetic, stays in residue space.
    pub fn advance(&self, days: u64) -> MayaState;

    /// Two states are in the same CRT equivalence class iff their
    /// residues agree position-by-position. Requires matching lanes.
    pub fn same_position(&self, other: &MayaState) -> bool;

    /// Whether the specified lane is at its zero/origin residue.
    pub fn lane_at_origin(&self, lane_idx: usize) -> bool;

    /// Count lanes simultaneously at origin.
    pub fn alignment_count(&self) -> usize;

    /// Compute the Ramanujan harmonic signature across all lanes.
    /// Returns Vec<i64> (Ramanujan sums can be negative integers).
    pub fn harmonic_signature(&self, n: u64) -> Vec<i64>;

    /// Pisano coverage profile per lane: returns (modulus, pisano_period,
    /// distinct_count). NOTE: no float ratio (D-1) — caller computes
    /// `distinct/modulus` in its own representation if needed.
    pub fn pisano_profile(&self) -> Vec<(u64, u64, u64)>;

    /// Adapter: convert a canonical-Safe-Basis address into a MayaState
    /// with six lanes [2, 3, 5, 7, 11, 13].
    pub fn from_cram_address(addr: [u64; 6]) -> MayaState;

    /// Adapter: project this MayaState into a canonical-Safe-Basis address.
    /// Returns Some only if the lanes are exactly the Safe Basis in order.
    pub fn to_cram_address(&self) -> Option<[u64; 6]>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MayaStateError {
    LaneResidueMismatch { lanes: usize, residues: usize },
    ResidueOutOfRange { lane_idx: usize, residue: u64, modulus: u64 },
    LaneMismatch,  // for same_position when lanes differ
}
```

## 5. Helper primitives (B-7.0 — same commit as foundation)

```rust
// engines/pisano.rs

/// Maximum modulus for which we compute Pisano periods.
/// Covers all Maya cycles in dccms (largest is Long Count 1,872,000 / Baktun-many; smaller naturally for π).
pub const PISANO_MAX_MODULUS: u64 = 100_000;

/// Pisano period π(m) — the period of the Fibonacci sequence mod m.
///
/// Computed naively by iterating until (F_n, F_{n+1}) ≡ (0, 1) (mod m).
/// Returns Err for m == 0 or m > PISANO_MAX_MODULUS.
pub fn pisano_period(m: u64) -> Result<u64, PisanoError>;

/// Fibonacci entry point α(m) — least n > 0 with m | F_n.
/// Returns Err similarly.
pub fn fibonacci_entry_point(m: u64) -> Result<u64, PisanoError>;

/// Fibonacci mod-m sequence for one full Pisano period.
/// Returns Err similarly.
pub fn fibonacci_mod_sequence(m: u64) -> Result<Vec<u64>, PisanoError>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PisanoError { ModulusZero, ModulusTooLarge(u64) }
```

```rust
// engines/ramanujan.rs

/// Ramanujan sum c_q(n) = Σ_{a coprime to q, 1≤a≤q} e^{2πi a n / q}.
///
/// Computed via the exact integer formula: c_q(n) = μ(q/gcd(q,n)) · φ(q) / φ(q/gcd(q,n)).
/// All arithmetic stays in i64 — no roots of unity, no floats.
pub fn ramanujan_sum(q: u64, n: u64) -> i64;
```

## 6. Reference engines (B-7.1 and B-7.2)

```rust
// engines/vigesimal.rs — Engine 1

pub struct Vigesimal;

impl Vigesimal {
    /// Quadrant lane: ℤ/4ℤ, "spatial-quadrant" domain.
    pub const QUAD_LANE: Lane = Lane { name: "quad", modulus: 4, domain: "spatial-quadrant" };
    /// Pentadic lane: ℤ/5ℤ, "elemental" domain.
    pub const PENT_LANE: Lane = Lane { name: "pent", modulus: 5, domain: "elemental" };

    /// Encode 0..=19 into a 2-lane MayaState. Returns Err for value >= 20.
    pub fn encode(value: u64) -> Result<MayaState, VigesimalError>;

    /// Reconstruct the canonical 0..=19 value via CRT.
    pub fn decode(state: &MayaState) -> Result<u64, VigesimalError>;

    /// Lane-parallel addition mod 4 / mod 5.
    pub fn add(a: &MayaState, b: &MayaState) -> MayaState;

    /// Lane-parallel multiplication mod 4 / mod 5.
    pub fn mul(a: &MayaState, b: &MayaState) -> MayaState;

    /// Fibonacci orbit through vigesimal residue space.
    /// Length = π(20) = 60.
    pub fn fibonacci_orbit() -> Vec<MayaState>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VigesimalError { OutOfRange(u64), MalformedState }
```

```rust
// engines/tzolkin.rs — Engine 2

pub struct Tzolkin;

impl Tzolkin {
    pub const TONE_LANE: Lane = Lane { name: "tone", modulus: 13, domain: "consciousness" };
    pub const SIGN_LANE: Lane = Lane { name: "sign", modulus: 20, domain: "identity" };

    /// Construct from 1-indexed tone (1..=13) and 0-indexed day-sign ordinal (0..=19).
    /// Day-sign ordinal corresponds to `dayname::DayNameGlyph` in the canonical
    /// modern Mayan orthography (Imix=0, IkPrime=1, ..., Ajaw=19).
    pub fn new(tone: u64, glyph_ordinal: u64) -> Result<MayaState, TzolkinError>;

    /// Construct from a `DayNameGlyph` plus a tone.
    pub fn from_glyph(tone: u64, glyph: crate::h4_visual::DayNameGlyph)
        -> Result<MayaState, TzolkinError>;

    /// Construct from absolute Tzolk'in day number 0..=259.
    pub fn from_day(day: u64) -> MayaState;

    /// Project to absolute Tzolk'in day 0..=259 via CRT.
    pub fn to_day_number(state: &MayaState) -> Result<u64, TzolkinError>;

    /// Render as "<tone> <glyph-name>" using canonical modern Mayan orthography.
    pub fn display(state: &MayaState) -> Result<String, TzolkinError>;

    // NOTE: Vigesimal-spec method `above_consciousness_threshold` is intentionally
    // OMITTED per D-2 (φ³ threshold is synthesis O4, do-not-mechanize).

    /// Fibonacci-mod-13 sequence — the tone-lane's intrinsic Fibonacci pattern.
    pub fn fibonacci_tone_pattern() -> Vec<u64>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TzolkinError { ToneOutOfRange(u64), GlyphOutOfRange(u64), MalformedState }
```

## 7. Tests planned

Per-module test groups; all tests A1 zero-float.

| Module | Tests |
|---|---|
| `lane.rs` | `new` validates residue ranges; `new` rejects mismatched lengths; `advance(0)` is identity; `advance(d).advance(e) == advance(d+e)`; `same_position` reflexive/symmetric/transitive; `alignment_count` matches manual count; `from_cram_address` / `to_cram_address` round-trip for any u64 |
| `pisano.rs` | `pisano_period(5) == 20` (foundational claim); `pisano_period(13) == 28`; `pisano_period(20) == 60`; `fibonacci_entry_point(13) == 7`; `fibonacci_entry_point(5) == 5`; error on m=0 and m > MAX |
| `ramanujan.rs` | `c_1(n) == 1` for all n (Ramanujan-1 identity); `c_q(0) == φ(q)` (totient identity); `c_p(n) == p-1` if p∤n, else `-1`, for primes p |
| `vigesimal.rs` | Round-trip for all 0..=19; `add` matches `(a+b)%20`; `mul` matches `(a*b)%20`; Fibonacci orbit length == 60; encoding rejects ≥ 20 |
| `tzolkin.rs` | Round-trip for all 0..=259 days; `from_glyph(1, Imix).to_day_number() == 0`; `display(from_day(0)) == "1 Imix"` (1-indexed tone); cross-check `dayname::DayNameGlyph::Eb` (ordinal 11) maps to a Tzolk'in day where sign-lane residue is 11; matrix test that 13·20-cyclic structure produces exactly 260 distinct days |

Total new tests projected: ~30–40. Should land workspace at 470+ passing.

## 8. What this commit does NOT do

- Does not touch `dayname.rs`, `month.rs`, `iconographic.rs`, or any other existing module. Existing CRAM addresses, h4_visual transducer, dpm_prime stack — all untouched.
- Does not migrate any existing module into the new engines. No deprecations. Migration is a separate later commit after the type vocabulary is proven stable through B-5 and B-6.
- Does not port LongCount, Dresden Eclipse, Venus Table, or MayaFabric. Those phases land separately.
- Does not change any existing test. Workspace test count grows; no regressions.

## 9. Open questions for sign-off

Before any code:

1. **D-1 — drop `phi_approximation` entirely, or keep it as `phi_approximation_integer_ratio() -> (u64, u64, u64)` returning `(num, den, error_in_thousandths)` to preserve the 8/5 ≈ φ observation in exact integers?**

2. **D-3 — confirm: keep dccms's modern Mayan orthography (Imix, IkPrime, …, Ajaw) as canonical, document the colonial-orthography mapping for vault cross-reference but do not introduce a parallel spelling set in the engines port?**

3. **D-5 — `PISANO_MAX_MODULUS = 100_000` is sufficient for all Maya cycles. Confirm or specify a different ceiling.**

4. **Scope confirmation — B-7 commit lands B-7.0 (Lane, MayaState, pisano, ramanujan), B-7.1 (Vigesimal), B-7.2 (Tzolkin). LongCount / Dresden / Venus / Fabric defer to later B-7.3 — B-7.6 commits. Sound, or pull more into this commit?**

5. **`MayaState::pisano_profile` — D-1 mitigation returns `(u64, u64, u64)` triples (modulus, pisano_period, distinct_count). Is that the right shape, or include the coverage ratio as a separate `(num, den)` pair?**

## 10. Once signed off

Execute in this order:

1. Create `dccms_atlas/src/engines/mod.rs`, `engines/lane.rs`, `engines/pisano.rs`, `engines/ramanujan.rs`. Wire into `dccms_atlas/src/lib.rs`.
2. Build tests for all three primitives. Verify foundational claims: `π(5) = 20`, `α(13) = 7`.
3. Build `engines/vigesimal.rs`. Tests round-trip 0..=19.
4. Build `engines/tzolkin.rs`. Bridge to `dayname::DayNameGlyph`. Tests round-trip 0..=259.
5. Full workspace test pass; commit + push.

No code is written until the open questions are resolved.
