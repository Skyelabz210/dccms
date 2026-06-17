# Dresden Codex Project — Thesis and Evidence Register

**Status:** working hypothesis under test. Nothing here is established fact about
the manuscript yet. This document separates what we *claim*, what is merely
*internally consistent*, and what would count as *real evidence* gathered from the
physical artifact.

---

## PART 1 — What we think is happening (the thesis)

### Plain statement

We think the Dresden Codex is not just a calendar but a working piece of
exact-integer mathematics — a hand-operated residue-number machine. Concretely:

1. **Residue tracking.** The Maya tracked celestial cycles (Tzolk'in 260, Haab 365,
   Venus 584, Mars 780, the 148/177-day lunar-eclipse intervals, etc.) not as single
   running totals but as a set of *independent remainders* against the small primes
   {2, 3, 5, 7, 11, 13}. In modern terms: a Residue Number System / Chinese
   Remainder Theorem machine ("CRAM").
2. **Exact correction, no fractions.** The red vs black numbers (especially in the
   eclipse table) are an exact-integer winding-extraction mechanism
   ("K-elimination") that recovers how many times a cycle has wrapped without ever
   using a fraction or decimal.
3. **The whole codex is one machine.** All 74 pages are domains of that machine:
   biological/ritual almanacs (1–23), a handoff/"bootloader" page (24),
   Mars/agriculture (25–45), the Venus table (46–50), the eclipse table (51–58),
   deep-time "serpent" counts and the flood reset (59–74).
4. **Prime 11 is a hidden coordinate.** 11 does not divide the obvious cycle lengths
   but is claimed to govern their *displacements* (Mars–Venus alignments, Saturn's
   11² = 121 resonance) — a "shadow prime."

### The honest caveat (must stay attached to the thesis)

- This thesis was produced by **AI-assisted analysis**, and the existing software
  **encodes the same assumptions it is supposed to test**. The software's tests
  passing only proves the code faithfully copied the thesis — it does **not** prove
  the thesis is true of the real manuscript.
- The **plain calendar arithmetic is correct** (which numbers divide which — e.g.
  260 = 2²·5·13). That part is not in doubt.
- The **interpretive leap is unproven**: that the scribes intended a residue
  computer, that red/black = K-elimination, that 11 is a deliberate shadow
  coordinate, that pages map to machine "domains."
- **Mainstream scholarship** (Thompson, Taube, Vail, Aveni, Bassie-Sweet) describes
  these pages as divinatory almanacs and astronomical tables and does **not** assert
  the computer model. Our own Moon Goddess research file agrees with them and files
  the CRAM idea under a single "researcher note."

So: this is a **hypothesis we intend to test**, not a finding. The scanner exists to
try to **break** it. We are prepared for parts of it to fail.

---

## PART 2 — The complete evidence register

Two classes. **Class A** is already computable and already "confirmed" — but
circularly, because the code checks numbers it was handed. **Class B** is the real
evidence: things that must be measured from the actual page images or from external
astronomy, which the project does **not** yet do. Genuine confirmation or refutation
lives in Class B.

### CLASS A — Arithmetic predictions (computable; currently circular)

| # | Claim | Verifiable by | Status |
|---|---|---|---|
| A1 | Cycle factorizations: 260=2²·5·13; 365=5·73; 584=8·73; 780=2²·3·5·13; 819=3²·7·13; 11960=2³·5·13·23; 37960=2³·5·13·73 | factoring | TRUE (but trivial) |
| A2 | Grand sync: 37960 = 65·584 = 104·365 = 146·260 | multiplication | TRUE |
| A3 | Calendar Round = lcm(260,365) = 18,980 | LCM | TRUE |
| A4 | Stride nullification: 78 zeros lanes {2,3,13}; 780 zeros {2,3,5,13} | mod | TRUE |
| A5 | Shadow displacements: Saturn 242 = 2·11²; Mars–Venus 11-lane zeros | mod | TRUE given inputs |
| A6 | Ramanujan partition congruences exist only for {5,7,11} | Ahlgren–Ono (2000) | TRUE (real theorem) |
| A7 | 1448 mod 260 = 148 (the "seal") | mod | TRUE (but trivial) |

**These are all true, but they are facts about numbers we already know. They do not
require the codex and do not by themselves support the "computer" interpretation.**

### CLASS B — Empirical evidence (must be MEASURED; NOT yet built) — the real list

| # | Claim / prediction | Where | What to measure in the image | Confirms if… | Refutes if… |
|---|---|---|---|---|---|
| **B1** | The numbers painted on the pages ARE the cycle constants and their multiples (Venus table = multiples of 584; eclipse chain = 148/177/178; Serpent = deep-time counts; flood = 5.1.0 = 1820; Venus phases 236/90/250/8) | 24, 46–58, 61–73, 74 | **Bar-dot numeral OCR**: bars=5, dots=1, decode vigesimal | read values match predicted constants | numerals differ / absent where predicted |
| **B2** | Red ("intercalated") numbers mark exact corrections; red/black alternates through the eclipse table; red intensifies on pp 16–23 and 6–10; pp 25–45 are black-dominated | 6–10, 16–23, 25–45, 51–58 | **Red-pixel vs black-pixel fraction** per page/band | the predicted spatial pattern appears | distribution is flat / random vs prediction |
| **B3** | Glyph/event density varies strongly across pages (max/min > 12:1); astronomical tables (24, 46–58) are numerically dense; almanacs are figurative | all 74 | **Shannon entropy + numeral/glyph counts** per page | tables show high numeric density; spread matches | density roughly uniform |
| **B4** | Red horizontal barriers partition pages into registers; page 24 is a structural boundary | all 74; 24 | **Barrier detection** (already partly built) | consistent banding; page 24 distinct | no consistent structure |
| **B5** | 148/177/178-day distance numbers are the stopping points linking the Moon Goddess almanacs to the eclipse table; the eclipse chain sums to 11,960 over 69 intervals | 16–23, 51–58 | **OCR the distance numbers**; sum the chain | reads as predicted, sums to 11,960 | intervals differ / don't sum |
| **B6** | Specific figures per page (Goddess I / Moon Goddess on 16–23; Chak Chel pouring the flood on 74; spearing Venus deities on 46–50) | 16–23, 46–50, 74 | **Classify the figure from pixels** (not by page number) | matches scholarly identification | classifier can't tell figures apart |
| **B7** | 8 pages WWII-damaged, in two modes (ink loss vs stain accumulation) | 2,4,24,28,34,38,71,72 | **SLUB photo vs FAMSI drawing ratio** (partly built) | damaged pages separate cleanly into two modes | no separation |
| **B8** | The tables actually predict **real** eclipses / Venus events at the stated intervals | 46–58 | **Compare decoded dates to a modern astronomical ephemeris** / known historical eclipses | predicted dates align with real sky events | no better than chance |
| **B9** | The codex's specific numbers are "special" (not numerology) | n/a | **Statistical null test**: run the same coincidence checks on random/invented period sets | codex numbers stand out vs controls | random sets score the same |

**B1 and B8 are the strongest tests.** B1 (read the actual numerals and check they
match) is fully internal to the images. B8 (do the predicted intervals hit real
eclipses?) is fully external to both the documents and the code — it is the one test
that cannot be faked by copying.

### What is built today vs. what is missing

- **Built:** the Class-A arithmetic; barrier detection (B4, partial); SLUB/FAMSI
  comparison (B7, partial); figure lookup-by-page-number (NOT B6 — it does not read
  pixels).
- **Missing (the real evidence):** bar-dot numeral OCR (B1, B5), red/black pigment
  measurement (B2), entropy/density (B3), real figure classification (B6), the
  astronomical cross-check (B8), and the statistical null test (B9).
