# DCCMS Formal Thesis — Recorded Verbatim

**Status:** `[USER-STATED]` — transcribed from user's own statement in session
2026-06-17. Arithmetic claims independently verified with pure Python (290/290
checks pass — see verification note at end). Interpretive claims remain
hypotheses pending empirical confirmation against the Dresden Codex manuscript.

**Author:** Acid / HackFate (project: DCCMS — Dresden Codex Configuration
Manifold Study)

---

## Preamble

> "this is 1 of the handful of parallel research and analysis angles im pursuing"

---

## 1. Forward Generator

The Dresden Codex calendar cycles are produced by a single integer generator:

```
G(n_reg, base, n_inter, inter) = n_reg · base + n_inter · inter
```

| Cycle | Parameters | Result |
|---|---|---|
| Tzolk'in | G(13, 20, 0, 0) | 260 |
| Haab | G(18, 20, 1, 5) | 365 |
| Tun | G(20, 18, 0, 0) | 360 |

The intercalary slot (n_inter · inter) absorbs the 5-day Wayeb without
breaking the 18-register count. The generator is exact-integer throughout.

---

## 2. Winding Recovery Formula

For a CRAM machine with 13 lanes indexed 0–12 and 20 parameters indexed 0–19:

```
w(l, p) = 2 · (p − l − 1)  mod  13
```

where `l` = lane index, `p` = parameter index.

**Verified:** Produces a well-defined integer in [0,12] for all 260 (l,p) pairs.
No floating-point arithmetic required.

---

## 3. Substrate Role Map

| Prime | Role | Codex anchoring |
|---|---|---|
| 5 | Content / intercalary slot | Wayeb 5-day sink; Haab intercalary |
| 7 | Bridge / courier / transition | 7×52=364; 7×13=91 quarter-year |
| 11 | Shadow coordinate / substrate navigator | 7592 mod 11 = 2; Venus boundary walk |
| 13 | Boundary / obstruction / parameter-cycle wall | 13-lane CRAM wall; 13×20=260 |
| 73 | Solar-Venus phase-lock | 584 = 8×73 |
| 37 | Lunar 5-month prime | 37×4 = 148 (eclipse interval) |
| 59 | Lunar 6-month prime | 59×3 = 177 (eclipse interval) |
| 23 | Eclipse-table scaling prime | 11960 = 8×5×13×23 |

---

## 4. Venus 11-Lane Page-Boundary Orbit

```
13 × 584 = 7592
7592 mod 11 = 2
```

Venus pages 46–50 walk mod-11 states:

| Page | State (mod 11) |
|---|---|
| 46 | 2 |
| 47 | 4 |
| 48 | 6 |
| 49 | 8 |
| 50 | 10 |

The walk touches only even residues — the "shadow lane" (11) is never zero on
these pages. This is Hypothesis H-VENUS-11.

---

## 5. Eclipse Table Arithmetic

```
11960 = 8 × 5 × 13 × 23       (eclipse table total, days)
11960 mod 378 = 242
242 = 2 × 11²                  (Saturn synodic 378; shadow metric 242)
1448 mod 260 = 148             (148 = eclipse short interval)
59 × 3 = 177                   (177 = eclipse long interval A)
37 × 4 = 148                   (148 = eclipse long interval B)
```

---

## 6. Node Cycle / 819 Facts

```
819 = 9 × 91 = 9 × 7 × 13
819 mod 91 = 0
819 mod 20 = 19
```

---

## 7. Complete Evidence Register

### Section A — Arithmetic (computable, internally consistent)

| ID | Claim | Arithmetic status |
|---|---|---|
| A1 | G(13,20,0,0)=260; G(18,20,1,5)=365; G(20,18,0,0)=360 | VERIFIED |
| A2 | w(l,p)=2(p−l−1) mod 13 over 260 pairs | VERIFIED |
| A3 | lcm(260,365)=18980; 2×18980=37960=65×584=104×365=146×260 | VERIFIED |
| A4 | 7592=13×584; 7592 mod 11=2 | VERIFIED |
| A5 | 11960 mod 378=242; 242=2×11² | VERIFIED |
| A6 | 1448 mod 260=148; 59×3=177; 37×4=148 | VERIFIED |
| A7 | 819 mod 91=0; 819 mod 20=19; 819=7×13×9 | VERIFIED |
| A8 | 260=4×5×13; 78=2×3×13; 780=4×3×5×13; 584=8×73; 11960=8×5×13×23 | VERIFIED |

### Section B — DCCMS Atlas (code-level)

| ID | Claim | Status |
|---|---|---|
| B1 | `dccms_atlas` CRAM lane decomposition correctly maps 260/365/584 to residues | `[REPORTED]` — code passes 676 tests |
| B2 | Winding formula `w(l,p)` implemented in `dccms_atlas` | `[REPORTED]` — needs code-read to confirm |
| B3 | DPM-PRIME arithmetic checks T1–T10 | `[REPORTED]` — tautological (checks its own constants) |

### Section C — Visual / Provenance (Dresden Codex manuscript)

| ID | Claim | Status |
|---|---|---|
| C1 | Red and black numerals both present and distinguishable | `[DIRECT-OBS]` — WDL scan pages 46, 74 |
| C2 | Bar-dot numeral system throughout | `[DIRECT-OBS]` — all numbered pages |
| C3 | Red horizontal register barriers divide pages into bands | `[DIRECT-OBS]` — page 16 and throughout |
| C4 | Three-register layout is the standard almanac format | `[DIRECT-OBS]` — pages 16–23 |
| C5 | Deity figures in painted panels (blue/turquoise + red pigments) | `[DIRECT-OBS]` — almanac section |
| C6 | Page 24 partially damaged (visible stain) but retains side columns | `[DIRECT-OBS]` — WDL page 24 |
| C7 | Pages 1–3 near-total WWII content loss | `[DIRECT-OBS]` — WDL pages 1–3 |
| C8 | Astronomical sections (Venus/eclipse) visually denser than almanac | `[DIRECT-OBS]` — pages 46–52 vs 16–23 |

### Section D — Venus Page-Boundary (requires pixel measurement)

| ID | Claim | Status |
|---|---|---|
| D1 | Red-ink fraction increases on pages 46–50 vs almanac pages | `[UNRELIABLE]` — not yet measured |
| D2 | Page 46 shows the densest numerical grid of Venus table | `[DIRECT-OBS]` — visually denser; exact ratio unmeasured |

### Section E — Entropy (requires integer-approximation image analysis)

| ID | Claim | Status |
|---|---|---|
| E1 | Shannon entropy distinguishes table vs almanac pages | `[UNRELIABLE]` — not yet measured |
| E2 | Bit-complexity (integer approximation of log₂) is computable without floats | `[UNRELIABLE]` — method not yet implemented |

**Constraint:** The workspace prohibits float arithmetic (`#![deny(clippy::float_arithmetic)]`
with no overrides). Any entropy implementation must use integer approximations
(e.g., bit-length as integer log₂ proxy).

### Section F — Provenance

| ID | Claim | Status |
|---|---|---|
| F1 | Dresden Codex is WDL 11621 (World Digital Library, 2013-07-01, 78 pages) | `[DIRECT-OBS]` — rendered with poppler-utils |
| F2 | Vault documents (Decoded.md, HULTA, DPM-PRIME) are AI-penned from prior sessions | `[USER-STATED]` |
| F3 | The repository purpose is to build a testing apparatus, not record AI claims | `[USER-STATED]` |

### Section G — Classifier Circularity

| ID | Claim | Status |
|---|---|---|
| G1 | `from_page(page)` returns figure identity by lookup, not pixel analysis | `[CODE-READ]` — confirmed |
| G2 | "12/12 figure matches" is tautological (both sides from same function) | `[CODE-READ]` — confirmed |
| G3 | Damage predicate catches 1 of 8 vault-known damaged pages | `[REPORTED]` |

### Section H — NS/QMNF Bridge

| ID | Claim | Status |
|---|---|---|
| H1 | The CRAM substrate of the codex is structurally analogous to the QMNF (Quadratic Manifold Number Field) substrate | `[UNRELIABLE]` — interpretive |
| H2 | The forward generator G is a register-space projection of the NS (Number-Space) map | `[UNRELIABLE]` — interpretive |

---

## 8. Hypotheses

| Label | Statement | Testable? |
|---|---|---|
| H-VENUS-11 | Venus table pages 46–50 walk mod-11 states 2,4,6,8,10 — none hit the shadow zero | Yes — arithmetic confirmed A4; image correlation pending D1 |
| H-7-BRIDGE | Pages whose primary cycle count is divisible by 7 carry structural (not content) function | Yes — requires page-by-page cycle count + content classification |
| H-13-BOUNDARY | 13 acts as a parameter-cycle wall in the CRAM machine — the winding formula saturates at 13 lanes | Yes — winding formula verified A2; manifold interpretation pending |
| H-5-CONTENT | 5 is the intercalary content slot — the Wayeb/Haab slot in the generator | Yes — arithmetic confirmed A1 |
| H-PIN-OPERATOR | A page-index pin operator maps each page to a (lane, parameter) address in the CRAM manifold | Partially — G and w give the arithmetic; page→address map not yet implemented |
| H-PAGE24-PROVENANCE | Page 24 (visibly damaged) is the table-cover page, not a content page — damage is structural not accidental | Pending — requires comparison with undamaged facsimile sources |
| H-SHADOW-ENTROPY | Entropy (measured by integer approximation) is lower on shadow-lane pages (11, 22, 33, …) than on content pages | Pending — entropy method not yet implemented |

---

## 9. Final Canonical Statement

The Dresden Codex encodes an exact-integer residue machine. The machine's substrate
is the prime set {2, 3, 5, 7, 11, 13}. Its register-space is produced by the
forward generator G. Its winding structure is recovered by w(l,p). The astronomical
tables (Venus, eclipse, Mars) are the machine's output register — they record
celestial-cycle addresses in the residue space, not merely observational calendars.

The role of each prime is specific and non-interchangeable: 5 absorbs intercalary
content, 7 routes transitions, 11 navigates the shadow coordinate, 13 walls the
parameter cycle. The eclipse table (11960 days) encodes the scaling prime 23.
Saturn's synodic period (378) produces the shadow metric 242 = 2×11² when the
table is taken mod 378. The Venus table (7592 = 13×584) leaves remainder 2 mod 11
— the first step of an even-only mod-11 walk across the five Venus pages.

This thesis is `[USER-STATED]`. Its arithmetic is independently verified (290
checks, pure Python). Its interpretive claims about the manuscript remain
hypotheses pending direct measurement of the primary source.

---

## Arithmetic Verification Note

All arithmetic claims in this document were verified in session 2026-06-17 using
pure Python (no sympy, no floating-point). 290 checks, 0 failures.

Constraints carried into any implementation:
- `#![forbid(unsafe_code)]` — no overrides
- `#![deny(clippy::float_arithmetic)]` — no overrides; floats are prohibited
  throughout; any entropy computation must use integer-only approximation
