# Methodological Stance

## Rigor is the enabler of ambition, not a limit on it

This project's firm commitment to absolute scientific rigor and methodical
discipline must **not** be conflated with a lack of ambition or an absence of
significant, potentially groundbreaking conclusions.

The investigation pursues large claims — that the Dresden Codex encodes an
exact-integer residue machine; that its prime substrate carries specific,
non-interchangeable computational roles; that a 60-year open problem in residue
number system division (Szabó & Tanaka, 1967) admits an exact O(k) solution.
These are ambitious, unconventional, and expansive in scope, and the project
states them as such.

The distinguishing feature of the methodology is **not** the modesty of its
conclusions. It is the standard of proof behind them:

> Every claim — regardless of whether it appears grandiose, unconventional, or
> expansive in scope — is grounded strictly in, and supported by, irrefutable
> and verifiable evidence-based data that withstands rigorous falsification
> testing.

A bold conclusion and a rigorously-supported conclusion are not opposites. The
project's purpose is to hold both at once: to reach for significant results, and
to admit them into the record **only** once they survive exact verification.

## What "supported by verifiable evidence" means here

A claim is promoted only when it is anchored to at least one of the following,
and never on the strength of recurrence or rhetorical force alone:

1. **Exact integer arithmetic** — no floating point on any verification path.
   Modular and cyclic facts are checked by integer congruence, reproducibly
   (e.g. the 290-check verification behind `FORMAL_THESIS.md`).
2. **Machine-checked formal proof** — where the mathematics permits, claims are
   discharged in a proof assistant with **zero `sorry` and zero axioms**, and
   cross-validated across independent systems (see
   `formal/k-elimination-lean4/`: 27 Lean 4 theorems + 10 Coq lemmas).
3. **Direct observation of the primary source** — measured against the Dresden
   Codex facsimile (`docs/facsimile/`), not against secondary interpretation.
4. **Explicit falsification** — a stated prediction with a defined failure
   condition, and a null/control comparison. Failed predictions are recorded,
   not deleted.

## The gating discipline

Claims move through the promotion gates defined in
`research/dresden_hcrm/01_dag/INVESTIGATION_DAG.md`:

```
G0 observation → G1 exact arithmetic → G2 local alignment
→ G3 repeated alignment → G4 cross-section recurrence → G5 predictive confirmation
```

A **strong claim requires G4 or G5.** Anything that has not cleared the gates is
tagged (`[UNRELIABLE]`, `[USER-STATED]`, High-Variance, …) and held at its actual
level — visible in the record, but not mistaken for a result. This is how the
project keeps the ambition of its hypotheses fully separate from the certainty of
its conclusions, without suppressing either.

## Why this document exists

The repository was rebuilt to recover from an earlier body of work in which
interpretive narrative was recorded as if it were established fact. The
correction was never to lower ambition. It was to require that every claim, large
or small, carry its evidence with it. Rigor is what makes the ambitious claims
worth stating.
