# `formal/` — machine-checked proofs

This tree holds formal-verification artifacts: claims discharged in a proof
assistant, with **zero `sorry` and zero axioms**, so the mathematics is
machine-checkable rather than asserted.

## `k-elimination-lean4/` — K-Elimination theorem `[FORMAL-VERIFIED]`

Lean 4 (+ Coq cross-validation) formalization of exact division in residue
number systems — the K-Elimination identity:

```
k = (v_A − v_M) · M⁻¹ (mod A)     for X ∈ [0, M·A), gcd(M, A) = 1
```

where `v_M = X mod M`, `v_A = X mod A`, `k = ⌊X/M⌋`.

| System | Theorems / lemmas | `sorry` | axioms |
|---|---|---|---|
| Lean 4 (4.27.0) | 27 | 0 | 0 |
| Coq (8.20.1) | 10 | 0 | 0 |

The core lemma `key_congruence` (`v_A ≡ v_M + k·M (mod A)`) and the basic
`fundamental` identity (`X = v_M + k·M`) are present and complete. The formula
was additionally numerically exercised in-session across 367,698 coprime
`(M, A, X)` cases with 0 failures — consistent with the proof.

### Scope — what is and is not verified

- **Verified `[FORMAL-VERIFIED]`:** the RNS division mathematics. K-Elimination
  recovers the overflow count `k` exactly, in O(k), for any coprime `M, A`. This
  is a self-contained number-theoretic result and stands on its own.
- **Not claimed here:** anything about the Dresden Codex. The codex work *uses*
  K-Elimination (the prime-11 navigation lane, `M_SAFE = 2·3·5·7·11·13 = 30030`),
  but the proof says nothing about whether the manuscript implements it. That
  remains a separate, manuscript-falsifiable question governed by the gates in
  `research/dresden_hcrm/01_dag/INVESTIGATION_DAG.md`.

This separation is the methodology in `../METHODOLOGY.md` in action: an ambitious
result (a 60-year RNS division problem solved exactly) admitted to the record
**because** it is machine-checked, with its scope stated precisely.

### Build artifacts

Compiled outputs (`*.vo/.vok/.vos/.glob`, caches) and the prebuilt FHE benchmark
binary/tarball were stripped before import; only sources and documentation are
tracked. Rebuild Lean with `lake build`, Coq with `coqc coq/K_Elimination.v`.
