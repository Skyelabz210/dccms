# Formal proof scope

## Authoritative proof pin

The expanded Lean work is pinned to:

```text
repository: https://github.com/Skyelabz210/k-elimination-lean4
working proof commit: dd9c6cfb341ad3747f148c336a015e2b80664e87
branch used during the campaign: campaign/universal-proof
Mathlib pin: 3bdc7047b97538f795b725dc0713c28c0f53ed10
```

That working tree contained **223 unique declarations** in the audited proof
inventory, with no `sorry`, custom axiom, or admission found. The smaller
snapshot vendored under `formal/k-elimination-lean4/` in this repository
predates that campaign and must not be mistaken for the full proof pin.

## Theorems that carry the current synthesis

1. **Exact observation criterion:** a target query is recoverable exactly when
   equal observations imply equal target values.
2. **Additive factor criterion:** an additive target factors through an
   additive observation image exactly when `ker B <= ker C`.
3. **Uniqueness on the authentic image:** the transported decoder is unique on
   readings actually emitted by the observation map.
4. **Adjacent StarLift:** for `A=M+1`, authentic readings modulo `M` and `A`
   yield `K mod A` by subtraction, and operand reversal yields `(-K) mod A`.
5. **Range lifting:** a residue class determines the full integer coordinate
   only after an admissible interval of sufficient width is supplied.
6. **Target-query transport:** knowing `K mod R` suffices for `X mod b` exactly
   under the stated divisibility/kernel condition.
7. **Least sufficient anchor:** gcd iteration reaches the least raw added
   modulus sufficient for a finite cyclic target without requiring an external
   factorization oracle.
8. **Reflection/involution laws:** swapping adjacent observations transports
   the additive inverse query and is involutive at the bounded level.

## Existence versus executable construction

The general set-theoretic factor theorem can establish that a decoder exists;
it does not automatically supply an efficient program. The 584 and 23 formulas
in this package are separate direct constructors. Each implementation must
also specify canonical representatives, compatibility checks, refusal cases,
bit widths, and producer authenticity.

## What the proofs do not establish by themselves

The formal results do not assert:

- that the physical universe is an integer or profinite state;
- that time emerges from winding;
- that a Dresden Codex glyph denotes a particular map or lane;
- that 584 or 23 is independently observed in a manuscript operation;
- that an ideal modular phase is a real eclipse occurrence;
- that an RNS implementation is cryptographically secure;
- that all division cases, especially noncoprime FPD, are complete;
- that the current DCCMS Rust path avoids scalar reconstruction; or
- that a theorem or architecture has no prior art.

Those are legitimate downstream hypotheses, but their extra premises must be
supplied and tested outside the abstract theorem.

## Formal work still worth doing

1. Import the campaign proof pin into a clean, canonical repository history.
2. Prove the concrete 584 and 23 constructors and inverses in Lean.
3. Formalize compatible cyclic frames and complement-lane synthesis.
4. Extract an executable minimal-observation compiler with proof certificates.
5. State and prove the exact noncoprime FPD success/refusal invariant.
6. Formalize source/frame/epoch authentication as types or indexed structures.
7. Prove packed-signature sufficiency only for explicitly listed queries.
8. Connect NINE65's actual encrypted execution path to the same interfaces and
   range/noise conditions.
