# Research Canvas — Extended Prime Basis (17, 19) and State-Space Extension

**Status:** `[USER-STATED]` — recorded verbatim from the project owner (session
2026-06-17). Tagged per the reliability discipline in `../README.md`.

This canvas proposes a **two-tier prime architecture**: the safe basis
`{2,3,5,7,11,13}` (verified, integer, closed) and an extended basis `{17,19}`
treated as a separate, high-variance tier. Recording it here does **not** promote
it. Per the canvas's own rule (§3) and the G0–G5 gates, every 17/19-dependent
claim enters at **G0 (observational)** and carries a **High-Variance** flag until
it survives the falsification protocol.

---

## Part A — Canvas, recorded verbatim `[USER-STATED]`

> **Core Conjecture:** The foundational stability of the CRAM framework relies on
> an immutable "safe basis" of lower primes (2, 3, 5, 7, 11, 13) that provide
> predictable modular convergence and deterministic transduction. Extending this
> architecture to incorporate primes 17 and 19 triggers a critical phase
> transition from stable, closed-loop computational cycles into a more complex,
> high-entropy regime of state-space dynamics. We must formalize this bifurcation
> to prevent the dilution of the system's core mathematical integrity while still
> capturing the full information density of the manuscript.
>
> **1. The Stability-Chaos Tradeoff:**
> - **Safe Basis (2–13):** Functions as the primary anchor for all lanewise
>   homogeneous and heterogeneous polyunitary operations. This range ensures that
>   transduction remains strictly reversible and that operators maintain exact
>   skew-symmetry, allowing for arithmetic closure without the introduction of
>   computational noise.
> - **Extended Basis (17, 19):** Acts as a catalyst for non-linear coupling.
>   While these primes are essential for covering the complete product space,
>   they generate "turbulence"—instances where residue state transitions deviate
>   from linear expectations. These points effectively function as entropy
>   sources or "chaos-events" within the machine, which cannot be modeled by
>   simple modular arithmetic alone.
>
> **2. Functional Extension Requirements:**
> - **Polyunitary Mapping:** Research must rigorously characterize how primes 17
>   and 19 modify the existing gate logic. Rather than attempting direct
>   integration, these higher-order primes should be treated as "volatile
>   registers" that trigger specialized, multi-stage validation cycles.
> - **Transduction Scaling:** As the computational system expands, the existing
>   skew-symmetric operators must be recalibrated to account for the increased
>   density of the residue mapping, specifically to prevent "state-overflow" into
>   unauthorized or uninterpretable domains.
>
> **3. Strategic Implementation in Research Canvas:**
> - **Turbulence Thresholds:** Implement a dedicated ledger column for "Basis
>   Complexity Level." Any analytical observation involving primes 17 or 19 must
>   be tagged with a "High-Variance" flag to prevent the premature promotion of
>   theoretical hypotheses.
> - **Boundary Management:** The transition from the Safe Basis to the Extended
>   Basis must be codified as an explicit structural shift in the Codex's internal
>   logic. This ensures that when the agent encounters 17/19 signals, it
>   automatically switches from deductive verification to stochastic pattern
>   mapping.
> - **Prediction/Falsification Protocol:** All claims involving 17/19 must be
>   subjected to a "Chaos-Resistance Test." If the residue distribution of a
>   17/19-dependent operator fails to reconcile with the shadow-witness signal
>   provided by prime 11, the claim must be automatically downgraded to "G0:
>   Observational," regardless of how often the signal appears to recur.
>
> **4. Research Objective:** To systematically demonstrate that the Dresden Codex
> operates as a multi-tiered, hierarchical computational engine: the lower primes
> govern routine, cyclical maintenance ("The Safe Basis"), while the higher primes
> (17, 19) act as the primary drivers for complex, non-repeating, or chaotic
> operations. This dual-layer architectural approach preserves the core mechanical
> integrity of the CRAM framework while providing the necessary overhead to manage
> the high information density required for accurate celestial and ritual cycle
> mapping.

---

## Part B — What is arithmetically checkable (and verified)

These are the only parts of the canvas grounded in exact integer arithmetic.
Verified in session 2026-06-17 (pure Python, no floats):

| Fact | Status |
|---|---|
| 17 and 19 are prime and exceed the safe-basis ceiling 13 | VERIFIED |
| Gear modulus 323 = 17 × 19; 323 mod 17 = 0; 323 mod 19 = 0 | VERIFIED |
| 11, 17, 19 are pairwise coprime — so their residues are independent, which is the precondition for the §3 "Chaos-Resistance Test" of an 17/19 operator against the prime-11 shadow witness | VERIFIED |
| All `mod_17` / `mod_19` (and every residue column) in `data/seed/period_prime_residue_seed.csv` re-derive correctly across all 15 rows | VERIFIED |

The data substrate for this tier **already exists**: the seed table carries
`mod_17`, `mod_19`, `mod_23`, `mod_29` columns and a dedicated `gear_17x19, 323`
row. No new arithmetic machinery is required to begin logging 17/19 residues.

## Part C — Interpretation: stated derivation vs. current verification status

The vocabulary in the canvas — "phase transition", "turbulence",
"chaos-events", "high-entropy regime", "non-linear coupling", "polyunitary
operations", "skew-symmetry", "transduction scaling", "volatile registers" — is
**interpretive framing** of an empirical finding, not a consequence of the
arithmetic in Part B. Its standing in this record:

### Stated provenance `[USER-STATED]`

The project owner reports that the 17/19 layer was **found empirically, by
reverse-engineering astrology from the Maya material** — not posited a priori.
That is a real derivation path with a stated source, and it is recorded as such.
It means the conjecture is a **claim about the content of the codex and the
astrological system it encodes**, which is exactly the falsifiable form a claim
needs to take. It is not idle speculation, and this record does not treat it as
such.

### Two notes to keep the recording honest (not a rebuttal of the finding)

1. **Locate the "turbulence" in the data, not in the primes.** Residues mod 17
   and mod 19 are, as pure arithmetic, exactly as deterministic and reversible
   as residues mod 11 or 13. So when the canvas says 17/19 behavior "cannot be
   modeled by simple modular arithmetic alone," the precise statement is that the
   *manuscript / astrological values* behave irregularly under 17/19 (e.g. period
   values that do not divide evenly), not that the moduli themselves are chaotic.
   Stated that way, the finding becomes testable against the page measurements
   and the natal data — which is where the empirical reverse-engineering lives.
2. **Verification status here is "stated, not yet re-derived in this record."**
   The owner has the derivation; this repository has not yet reproduced it
   end-to-end against the facsimile and the HCRM natal data. Until it does, the
   claim sits at G0–G1 in *this* ledger and carries the High-Variance flag — a
   statement about what this record has independently confirmed, not a judgment
   of the underlying work. The path to promotion is the Chaos-Resistance Test in
   §3 (reconcile a 17/19 operator against the prime-11 shadow witness) plus the
   natal cross-check in the field checklist.

## Part D — Relationship to `FORMAL_THESIS.md`

`FORMAL_THESIS.md` defines the safe basis `{2,3,5,7,11,13}` and the role map
(5=content, 7=bridge, 11=shadow, 13=boundary). This canvas does not contradict
it; it adds a **second tier above the boundary prime 13**:

| Tier | Primes | Role (claimed) | Verification status |
|---|---|---|---|
| Safe basis | 2, 3, 5, 7, 11, 13 | Routine cyclic maintenance; closed, reversible | Arithmetic VERIFIED (`FORMAL_THESIS.md`, 290 checks) |
| Extended basis | 17, 19 | Drivers of complex / non-repeating operations | Arithmetic VERIFIED (Part B); **roles are conjecture (Part C)** |
| (Seed data also tracks) | 23, 29 | 23 = eclipse-scaling prime (thesis); 29 unassigned | Residues VERIFIED; roles unverified |

This keeps the two as the **separate research angles** the owner described, with a
clean boundary at prime 13: everything at or below 13 is the verified core;
everything above is recorded, residue-logged, and held at G0 until it survives a
falsification test against the manuscript.
