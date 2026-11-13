# Missed connections and newly exposed structure

Stepping back across the session reveals several connections stronger than any
single numerical coincidence. Each item below includes what it unlocks and the
remaining test that prevents overextension.

## 1. The periods form an observation lattice

The correct common object is the divisibility-ordered family of finite cyclic
observations:

```text
              X mod lcm(a,b)
                 /       \
            X mod a     X mod b
                 \       /
               X mod gcd(a,b)
```

The upper object is realized by compatible pairs, not arbitrary pairs. This
turns “stacked and braided calendars” into a precise diagram of refinements and
forgetful maps.

**Unlocks:** a common language for calendar tracks, star frames, CRAM bases,
frame transduction, and partial observations.

**Next test:** identify which manuscript fields carry full phase readings and
which carry only compressed predicates.

## 2. A minimal-observation compiler unifies the 584 and 23 discoveries

For current joint period `P` and requested target period `D`, the missing raw
residue modulus is obtained prime-power by prime-power: for every prime whose
exponent in `D` exceeds its exponent in `P`, the new modulus must carry the
**full target prime power**. This is why:

- `P=1,332`, `D=5,256` requires `8*73=584`;
- `P=37,960`, `D=11,960` requires `23`;
- `P=18,980`, `D=11,960` requires `8*23=184`.

The gcd-saturation theorem gives a factorization-free route to the same fixed
point.

**Unlocks:** automatic channel design, counterexample generation, proof
certificates, and a systematic search for manuscript-supplied missing lanes.

**Missed before:** `D/gcd(P,D)` is not always itself a sufficient raw modulus;
when a higher prime-power exponent is missing, the full target power is needed.

## 3. Query plus complement lane is a general isomorphism pattern

Kernel inclusion yields a possibly many-to-one target query. If a retained lane
distinguishes the collapsed fibers, the combined output can have the same
kernel as the input and become reversible.

Concrete instances:

```text
(mod 36, mod 37, mod 584) <-> (mod 72, mod 73, mod 37)
(mod 260, mod 584, mod 23) <-> (mod 11960, mod 73)
```

**Unlocks:** a precise transduction design rule. Instead of demanding that
every target alone be an isomorphism, synthesize the smallest complement that
preserves the full frame.

**Next test:** formalize complement synthesis for arbitrary finite cyclic
observation frames and expose it in the CRAM API.

## 4. The number 584 is a three-way hinge

`584=8*73` simultaneously:

1. supplies the missing `2^3` and `73` information for the next-star target;
2. shares the 73 lane with `365=5*73`; and
3. doubles the full-phase Calendar Round resolution from 18,980 to 37,960.

This is a real structural seam, not three independent confirmations.

**Unlocks:** a focused manuscript search: look for a transition where a Venus
phase carries the parity bit needed to move from a 36 shell to a 72 shell.

**Required control:** show that the 584 reading is independently present at the
operation, rather than chosen because it satisfies the desired algebra.

## 5. The 73 lane is the shared coordinate hinge

The same lane appears in:

```text
365 = 5*73
584 = 8*73
S_4 = 73
M_4 = 72 = 73-1
```

The next-star constructor outputs mod 73 directly from mod 584. The reversible
calendar constructor retains mod 73 as its complement lane. Thus 73 links the
star, solar, Venus, and calendar-phase frames at the exact representation
level.

**Unlocks:** one shared typed lane can serve several downstream queries without
reconstructing a scalar source.

**Next test:** trace whether DCCMS/NINE65 actually shares that authenticated
lane or merely recomputes 73-valued residues from a scalar day count.

## 6. The 23 closure and the complete 873,080-state frame

The joint 260/365/584 observation has period 37,960. Against target 11,960 it
leaves `11,960/gcd(37,960,11,960)=23` phases. Adding mod 23 gives

```text
lcm(37,960, 23) = lcm(11,960, 73) = 873,080.
```

That equality explains why the calendar constructor and retained 73 lane form
a complete reversible frame.

**Unlocks:** a concrete target for a 23-state manuscript channel and a precise
roundtrip test.

**Critical sensitivity:** the result is specific to the ideal 11,960 modulus;
the surviving 11,959-day table has different arithmetic.

## 7. The 13-baktun boundary lands exactly on the ideal table phase

```text
1,872,000 = 98*18,980 + 11,960.
```

This is stronger than merely noting that the numbers share factors: a full
13-baktun span has phase 11,960 modulo the Calendar Round.

**Unlocks:** a sharply stated historical question: is that remainder used as a
boundary alignment, restart, or table-selection operation?

**Required control:** ordinary positional-calendar arithmetic predicts the
same remainder. The proposed machine reading must predict an additional sign,
placement, or operation on held-out material.

## 8. Forward time is naturally a monoid action; winding is a cocycle

A forward-only successor is an action of the nonnegative integers. If the base
phase is `r_n = n mod M`, then the wrap indicator accumulates into
`K_n=floor(n/M)`. Algebraically, winding is an additive cocycle over the
transition action.

**Unlocks:** a rigorous separation among:

- instantaneous phase;
- admitted path/history;
- operational tick count;
- winding/topological class; and
- elapsed duration after calibration.

This same pattern connects to topological pumping, where a cyclic control path
can return while a transported observable accumulates.

**Next test:** specify a physical system whose proposed winding law predicts a
different measured result from conventional dynamics.

## 9. Information sufficiency, not reconstruction, is the central resource

A 24-bit packed Hydra signature cannot invert a 136,656,000-state full phase
frame because `2^24 < 136,656,000`. That does not make it useless. It may still
be sufficient for a smaller target query.

**Unlocks:** evaluate every compressed representation by the queries it
preserves, rather than by whether it reconstructs the full source.

**Next test:** enumerate the exact equivalence classes of the packed signature
and compute which DCCMS outputs are constant on them.

## 10. Star geometry, adjacency, and primality are three different layers

Every nondegenerate star number gives an adjacent pair `S_n-1,S_n`, including
composites such as 121. Star-number primality is a separate multiplicative
filter. The divisor-class theorem provides a sieve but is not needed for
StarLift correctness.

**Unlocks:** avoid imposing prime structure where adjacency alone suffices;
reserve primality claims for search/security properties that actually use it.

**New question:** does the intended machine select star geometry, prime star
anchors, superstar primes, or different classes for different operators?

## 11. The 36 coincidence is unique in the basic star recurrence

For `S_n=6n(n-1)+1`, the next-star gap is `12n`. Equality

```text
S_n-1 = S_(n+1)-S_n
```

holds only at `n=3`, giving 36. Thus the 36/37 seed is not an arbitrary member:
its shell size equals its jump to the next star number.

**Unlocks:** a defensible reason to study 36/37 as a special transition.

**Boundary:** this uniqueness does not make every later transition reuse the
same 584 bridge.

## 12. The least extra modulus across star successors is itself a new sequence

For successive frames `(S_n-1,S_n) -> (S_(n+1)-1,S_(n+1))`, the least single
raw added moduli for `n=2..15` begin:

```text
333, 584, 605, 1629, 1771, 5392, 11691,
2705, 7271, 57096, 937, 7651, 4365, 46112
```

They are obtained from missing target prime powers, not by a constant-step
rule. The abrupt changes expose where consecutive star frames share or lose
prime-power information.

**Unlocks:** a new arithmetic object to factor, classify, and optimize for
hardware channel reuse.

**Next test:** derive a symbolic rule from the factorizations of `S_n` and
`S_(n+1)`, then compare it to manuscript or architecture lane selections.

## 13. Calendar observations behave like a presheaf over divisibility

Each modulus has a reading set; divisibility induces restriction maps; compatible
readings agree on GCD overlaps. A complete compatible family is the inverse-limit
idea behind profinite coordinates. Reversible frame changes are natural
isomorphisms between presentations with the same kernel.

**Unlocks:** a clean categorical formulation of transduction without claiming
that every query map is invertible. It may also guide Lean interfaces and typed
producer composition.

## 14. Many apparent confirmations share the same algebraic root

The numbers 365, 584, 73, 37,960, and the next-star output are linked by chosen
factorizations. They cannot be counted as statistically independent hits.

**Unlocks:** an evidence-ancestry map can separate:

- independent manuscript observations;
- independent astronomical measurements;
- definitions/model choices; and
- deterministic algebraic consequences.

That will make any remaining convergence substantially stronger because it
will identify which coincidences survive dependency correction.

This map is statistical bookkeeping, not an arithmetic dependency wrapper and
not a prerequisite for Shadow/GCD resolution.

## 15. The most valuable negative control is 11,959 versus 11,960

The factor-23 story is exact at 11,960 and disappears at prime 11,959. This is
an unusually strong discriminating control supplied by the historical record
itself.

**Unlocks:** test whether the manuscript mechanism tracks the written table,
an ideal cycle, a restart procedure, or a later arithmetic normalization.

**Proposed test:** freeze both decoders, score them against independently
transcribed stations/intervals, and do not change the target modulus after
seeing which fits better.

## 16. The architecture can be novel even when a local identity is known

Garner/MRC, CRT base extension, and modular inverses are relevant prior art for
specific computations. They do not by themselves settle the contribution of a
system that treats source-bound observations, derived winding queries,
minimal-channel synthesis, complement-lane reversibility, and certified
operator dispatch as one architecture.

**Unlocks:** write novelty claims component by component, with the closest
algorithmic comparator and the new composition/invariant stated explicitly.

**Required discipline:** do not call a familiar identity new; do not let that
identity erase a new theorem formulation, protocol, or architecture without
comparison at the same level.

## 17. Shared factors are resolution data, not a dead end

For division by `d` on a lane `m`, put `g=gcd(d,m)`. After subtracting the
authenticated remainder, reduction by `g` exposes the quotient modulo `m/g`.
The missing `g`-fold class is exactly what the Shadow/auxiliary frame resolves.
Thus GCD does not block the operation; it quantifies how much auxiliary
resolution is required.

The session also implemented GCD itself as an encrypted target query in NINE65:
a bounded direct truth-table circuit for widths 1–4 and fixed-round encrypted
Euclid for widths 5–8, with typed noise/width refusals. This is a downstream
example of the universal principle: compile the consumer's requested answer
instead of reconstructing every intermediate scalar state.

**Unlocks:** treat nonunit reciprocal failure as a dispatcher signal to the
Shadow/FPD/word-circuit route, not as a claim that arbitrary-modulus division or
GCD is unavailable.

**Integration task:** bind the existing route and certificate to each DCCMS and
NINE65 profile; do not reopen the solved arithmetic question.
