# Shadow Prime and GCD resolution correction

This note records the project-owner correction made after the initial synthesis:
the architecture does **not** leave GCD/shared-factor ambiguity as an unsolved
blocker. The Shadow Prime/auxiliary frame resolves the missing class, and this
session also produced an encrypted GCD consumer.

## 1. Shared-factor division law

For a positive divisor `d`, lane modulus `m`, authenticated remainder `r`, and
`g=gcd(d,m)`, the lane determines

```text
q mod (m/g)
  = ((((X mod m) - (r mod m)) mod m) / g)
      * inverse(d/g, m/g)) mod (m/g).
```

The canonical difference must be divisible by `g`. When `m/g=1`, the partial
view is the constant zero class and requires no inverse. The GCD therefore
measures the quotient resolution retained by that lane. Coherent Shadow or
auxiliary quotient coordinates plus the declared bound restore the missing
`g`-fold class. The chimera certificate distinguishes exact, bounded,
disagreement, missing-witness, and refusal outcomes.

Source-bound references in the NINE65 session pin:

- `docs/execution/2026-10-03/FPD_BFV_RESCALE_CONTRACT.md`
- `docs/CRAM_RLWE_SECURITY_ASSESSMENT_2026-06-03.md`
- `crates/exact_transcendentals/src/chimera_division.rs`
- `crates/exact_transcendentals/src/chimera.rs::select_aux_for_fpd`

## 2. Shadow is a disambiguator, not merely a raw coprime lane

The selected S8 Shadow-11 successor profile places 11 outside the primary
payload product `M=881,790`, so `M mod 11=8` and its inverse is 7. Under the
same-source and bounded-successor premises,

```text
DeltaK mod 11 = (shadow_B - shadow_A) * 7 mod 11.
```

The transient result is checked and discarded. Where 11 is already a factor
of the payload, a bare mod-11 reading is not falsely inverted; the system uses
an independent phase-locked coordinate or the lifted Shadow family. The
implemented lifted family is

```text
{11^6, 13, 17, 19}, product 7,438,784,639.
```

This distinction preserves the owner's point: Shadow supplies the ambiguity
resolution. It also prevents the narrower error of pretending `M^-1 mod 11`
exists when 11 divides `M`.

Source-bound references:

- `docs/execution/2026-10-03/PRIMING_ROOT_CONTRACT.md`
- `docs/CRAM_SAFE_BASIS_LIFTED_TRANSDUCTION_EXECUTION.md`
- `crates/exact_transcendentals/src/chimera.rs::shadow_winding`

## 3. Encrypted GCD was implemented in this session

NINE65 commit `bef7e2d90fb36653104170851459ec4335454701`
(`campaign/encrypted-gcd`, integrated into the campaign lineage at `3ff079e`)
added:

- `CramMainEvaluator::plan_integer_gcd`;
- `CramMainEvaluator::gcd_integer`;
- `CramPublicEvaluator::gcd_integer`;
- `SmallTruthTable` target compilation for public widths 1–4; and
- `FixedEuclid` with exactly `2w` masked rounds for widths 5–8.

The committed verification record reports:

- 7 final GCD tests passing;
- 87,380 fixed-round semantic cases;
- 900 clear small-width pairs per strategy;
- 65,536 native ciphertext output-coefficient checks;
- explicit typed width/noise refusals; and
- no operand decryption, source-selected circuit shape, stored scalar winding,
  or reconstructed operand in the live encrypted path.

Evidence pin:

```text
research/swarm-campaign-2026-11-13/runtime-gcd/evidence/verification.json
source base d883d24e6a1462c78564e554f42b3dd54044da59
```

This is bounded implementation evidence, not a claim that every width/profile
is admitted or that arithmetic correctness alone proves confidentiality.

## 4. Relation to the 23-way calendar example

The direct calendar constructor in this DCCMS packet uses `X mod 23` because it
is the simplest raw coordinate for the exact 23-way fiber. That does not mean a
literal mod-23 lane is the only resolver. A source-bound lifted Shadow reading
can encode the same class if its producer is injective on those 23 possibilities
and the decoder records the class map and bound.

Accordingly, the remaining calendar task is **binding**, not a reopened GCD
problem: document which Shadow state is present at that manuscript operation,
how it encodes the phase class, and how source/epoch identity is preserved.

## 5. “Dependency graph” clarification

The earlier recommendation to map dependencies referred only to statistical
evidence ancestry—avoiding counting algebraic consequences as independent
historical confirmations. It is not required by the arithmetic, does not wrap
the runtime, and has no role in resolving the quotient. The canonical term in
this packet is now **evidence-ancestry map**.
