# B-7 Completion Plan — Engines 3, 4, 5 + MayaFabric

**Status:** PROPOSAL → EXECUTING (per user directive: plan completely, complete entirely)
**Source:** vault `Mayas Engine.md` §ENGINE 3, §ENGINE 4, §ENGINE 5, §UNIFIED FABRIC.
**Prior state:** `engines/` module has `lane`, `pisano`, `ramanujan`, `vigesimal`, `tzolkin`. Workspace at 519 tests, HEAD 8a4bded.

This plan is **comprehensive** — every remaining B-7 phase is included with type signatures, tests, and adaptation decisions. No further deferral.

## Phase B-7.3 — LongCount engine (`engines/long_count.rs`)

**Vault:** §ENGINE 3 — Cylindrical time `ℝ × (S¹)⁴`. Five tiers: baktun (linear), katun (mod 20), tun (mod 20), uinal (**mod 18, anomalous**), kin (mod 20).

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LongCount {
    pub baktun: u64,
    pub katun: u8,   // 0..20
    pub tun:   u8,   // 0..20
    pub uinal: u8,   // 0..18
    pub kin:   u8,   // 0..20
}

impl LongCount {
    pub fn new(baktun, katun, tun, uinal, kin) -> Self;  // reduces mod the tier moduli
    pub fn to_days(&self) -> u64;
    pub fn from_days(days: u64) -> Self;
    pub fn advance_one_day(&self) -> Self;     // tier-promotion through overflow
    pub fn advance(&self, days: u64) -> Self;
    pub fn cyclic_state(&self) -> MayaState;   // 4-lane heterogeneous fabric
    pub fn period_endings(&self) -> Vec<PeriodEnding>;  // typed (not strings)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PeriodEnding { UinalEnd, TunEnd, KatunEnd, BaktunEnd }
```

**Adaptation:**
- Vault returns `Vec<&'static str>` from `period_endings`; we return typed enum (no stringly-typed).
- Vault's tier moduli are u64; we use u8 because each lane is bounded < 20 (saves memory, exposes bounds in the type).
- 13-Baktun Long Count `1,872,000` → `LongCount { baktun: 13, ..0 }` round-trips.

**Tests** (≥ 8):
- Round-trip for the 12 vault test dates {0, 1, 19, 20, 359, 360, 7199, 7200, 143999, 144000, 1872000, 13·144000}
- `advance_one_day` from origin → kin=1
- `advance(20)` from origin → uinal=1
- `advance(360)` → tun=1
- `advance(7200)` → katun=1
- `advance(144000)` → baktun=1
- `period_endings` for baktun-end (all four lower tiers at 0)
- `cyclic_state()` produces a 4-lane MayaState with moduli [20, 18, 20, 20]
- `LongCount::from_days(LONG_COUNT_13_BAKTUN).to_days() == 1_872_000`

## Phase B-7.4 — DresdenEclipse engine (`engines/dresden_eclipse.rs`)

**Vault:** §ENGINE 4 — eclipse prediction through commensuration. `PERIOD = 11960 = 46·260`.

```rust
pub struct DresdenEclipse;

impl DresdenEclipse {
    pub const PERIOD: u64 = 11_960;
    pub const HALF_ECLIPSE_LONG: u64 = 177;   // 6 lunar months
    pub const HALF_ECLIPSE_SHORT: u64 = 148;  // 5 lunar months

    pub fn warning_stations() -> Vec<u64>;
    pub fn is_eclipse_window(day_in_period: u64, tolerance: u64) -> bool;
    pub fn next_eclipse_from(reference_day: u64) -> u64;
    pub fn tzolkin_at_eclipse(reference_tzolkin: &MayaState) -> MayaState;
    pub fn verify_commensuration() -> Vec<(&'static str, u64, bool)>;
}
```

**Adaptation:**
- D-6: drop unused `eclipse_number` param from `tzolkin_at_eclipse` (vault method ignores it).
- Vault's `verify_commensuration` includes Haab (365) and Venus (584); 11960 mod 365 = 280 ≠ 0, vault's own comment catches this — we keep the returns honest.

**Tests** (≥ 7):
- `PERIOD == 46 · 260`
- `warning_stations()` produces alternating 177/148 intervals starting from day 0
- `is_eclipse_window(0, 3)` true (at station 0)
- `next_eclipse_from(100) == 100 + PERIOD`
- `tzolkin_at_eclipse` preserves any Tzolkin state (invariance: 11960 mod 13 = 0, mod 20 = 0)
- `verify_commensuration` honestly reports Haab (365) as NOT dividing 11960
- Tzolkin invariance: starting Tzolkin state, advanced by PERIOD via `Tzolkin::from_day(d + PERIOD)`, returns same residues

## Phase B-7.5 — VenusTable engine (`engines/venus_table.rs`)

**Vault:** §ENGINE 5 — `8·73 = 584 = T_V`, `5·73 = 365 = T_h`, sync 2920. Four stations: 236 + 90 + 250 + 8.

```rust
pub struct VenusTable;

impl VenusTable {
    pub const VENUS_SYNODIC:  u64 = 584;
    pub const EARTH_YEAR:     u64 = 365;
    pub const SYNC_PERIOD:    u64 = 2_920;
    pub const MORNING_STAR:   u64 = 236;
    pub const SUPERIOR_CONJ:  u64 = 90;
    pub const EVENING_STAR:   u64 = 250;
    pub const INFERIOR_CONJ:  u64 = 8;

    pub fn venus_state(day_in_cycle: u64) -> MayaState;  // lanes [8, 73]
    pub fn current_station(day_in_cycle: u64) -> VenusStation;
    pub fn find_sync() -> (u64, u64, u64);                // (v_cycles, e_cycles, days)
    pub fn correction_schedule() -> Vec<(u64, i64)>;
    pub fn pisano_analysis() -> Result<(u64, u64, u64, u64), PisanoError>;
        // (π(8), π(73), α(73), π(584))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VenusStation { MorningStar, SuperiorConjunction, EveningStar, InferiorConjunction }
```

**Adaptation:**
- **D-1 (re-affirmed): `phi_approximation` dropped entirely.** Vault returned `f64` ratio + `f64` error — A1 violation. The 8/5 synchronization fact is preserved via `find_sync() = (5, 8, 2920)`.
- Vault's `current_station` returns `&'static str`; we return typed enum.
- Phase boundaries sum: `236 + 90 + 250 + 8 = 584` checked.

**Tests** (≥ 7):
- `MORNING_STAR + SUPERIOR_CONJ + EVENING_STAR + INFERIOR_CONJ == VENUS_SYNODIC`
- `find_sync() == (5, 8, 2920)`
- `venus_state(0)` has both lanes at 0
- `current_station(0) == MorningStar`; transitions at 236, 326, 576
- `pisano_analysis()` produces well-defined integer values (π(584) = lcm(π(8), π(73)))
- `correction_schedule` entries match vault (61, -4), (122, -4), (183, -4), (305, -8)
- All days 0..VENUS_SYNODIC produce a valid station

## Phase B-7.6 — MayaFabric (`engines/fabric.rs`)

**Vault:** §UNIFIED FABRIC — `MayaFabric` binding all five engines on shared temporal substrate.

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MayaFabric {
    pub day: u64,
    pub tzolkin: MayaState,
    pub long_count: LongCount,
    pub venus_day: u64,
    pub eclipse_day: u64,
}

impl MayaFabric {
    pub fn origin() -> Self;
    pub fn at_day(day: u64) -> Self;
    pub fn advance(&self, days: u64) -> Self;
    pub fn alignment_report(&self) -> FabricAlignment;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FabricAlignment {
    pub day: u64,
    pub tzolkin_origin: bool,
    pub venus_station_boundary: bool,
    pub eclipse_window: bool,
    pub long_count_endings: Vec<PeriodEnding>,
    pub alignment_score: u32,
}
```

**Adaptation:**
- All five engines plumb in cleanly; no new float concerns.

**Tests** (≥ 5):
- `origin()` produces day-0 state with all sub-fields at origin
- `at_day(0)` == `origin()`
- `at_day(1).advance(0)` == `at_day(1)`
- `at_day(d).advance(k)` == `at_day(d+k)`
- `alignment_score` at day 0 is at least 3 (Tzolkin origin + Venus boundary + eclipse window)
- 13-Baktun fabric: `at_day(1_872_000).long_count.baktun == 13`

## Test count projection

| Phase | Estimated tests |
|---|---:|
| B-7.3 LongCount | 9 |
| B-7.4 DresdenEclipse | 8 |
| B-7.5 VenusTable | 9 |
| B-7.6 MayaFabric | 6 |
| **Subtotal** | **32** |

Projected workspace: 519 → ~551.

## Build order

```
B-7.3 LongCount       ─┐
B-7.4 DresdenEclipse  ─┼─→ B-7.6 MayaFabric (depends on the others)
B-7.5 VenusTable      ─┘
```

Each engine independent within tier (no cross-references); MayaFabric composes all four.

## After B-7 completion

Tier 3 plan to follow in a separate completion cycle:
- B-8 DKAM tier mapping
- B-9 page_arithmetic (Page 8 jaguar; Page 52a red barriers — Measured provenance)
- B-10 Maya-date API ergonomics
- B-11 Gini stratification verification (verify-before-mechanize per synthesis O7)
- B-12 Goddess section extension to pages 13c-15
- Discovery 2 — prime-gap-doubling boundary verification

That whole list will be its own complete-plan-complete-execute cycle.
