# Question ledger

The best next questions are not the broadest ones. They are the questions whose
answers would either unlock several branches at once or force the program to
change direction. The first twelve below are the highest-leverage set.

## Tier 0 — questions that define the machine

### Q0.1 — What is the source state?

Is `X` a day offset, an event count, a physical state, a vector of astronomical
phases, a path/history class, a profinite element, or a typed tuple containing
several of these? Which part is primary and which parts are queries?

**Why it matters:** every kernel, ambiguity, reconstruction bound, and claim of
universality depends on the admitted source. A theorem quantified over an
abstract source does not choose one for an application.

**Deliverable:** a typed source schema with units, epoch, frame, range, and
equality rules.

### Q0.2 — Where does each authentic reading come from?

For every lane used by a decoder—especially mod 584 and mod 23—what independent
producer emits it? Is it measured, copied from a manuscript field, advanced by
a transition law, or derived from another reading? How are same-source,
same-frame, and same-epoch conditions certified?

**Why it matters:** a constructor can be correct while its inputs are unrelated.
Range and GCD-compatibility checks do not prove provenance.

**Deliverable:** producer IDs and frame/epoch certificates in the input type.

### Q0.3 — What is the transition law?

What exact operation advances the base state by one unit? Are only forward
successors admitted, or are pauses, reversals, branching, merging, birth/death,
and variable-rate paths admitted? What is the state when a transition occurs?

**Why it matters:** orientation comes from a chosen action, not from a residue
set alone. Winding is then a cocycle of that action.

**Deliverable:** a deterministic or stochastic transition system and its
invariants.

### Q0.4 — Which query is the machine answering?

For each proposed output, is the goal a phase, quotient class, comparison,
exact integer, event predicate, future transition, or decoded symbol? What
distinctions is the output allowed to forget?

**Why it matters:** many apparent reconstruction problems are smaller query
problems. The universal theorem is most powerful when the target is explicit.

**Deliverable:** one source/observation/query triple per claim.

## Tier 1 — questions that can unlock or falsify several branches

### Q1.1 — What exactly are the four stacked structures?

Are they Tzolk'in, Haab, Calendar Round, and Long Count; four star tracks; four
operator layers; or four manuscript tables? Are their values full phases,
mixed-radix digits, carry predicates, or page-indexed labels?

**Test:** write the four types and their update laws before matching any page.

### Q1.2 — What operation in the manuscript produces the 584 bridge?

The arithmetic makes 584 a highly specific hinge: `584=8*73` provides both the
missing 2-adic bit and the next star anchor 73. Which sign, interval sequence,
or table relation independently supplies `X mod 584` at the moment of the
`36/37 -> 72/73` transfer?

**Decisive evidence:** a predeclared page/field rule that predicts unseen
transitions, not a period selected after observing the target.

### Q1.3 — Where does the mod-23 observation come from?

The 23-way ambiguity is exact for the chosen `P=37,960`, `D=11,960` model. Does
the manuscript carry an independently readable 23-state channel, an operator
that generates it, or only the factor 23 in a modern decomposition?

**Decisive evidence:** a source-bound 23-valued reading that resolves held-out
11,960 phases.

### Q1.4 — Is the target 11,959 or 11,960?

Recent scholarship distinguishes the surviving 11,959-day table from the
common idealized 11,960-day model. Which object is the claimed machine acting
on: written stations, interval sum, restart model, or ideal cycle?

**Why it matters:** 11,959 is prime and coprime to 37,960. The 23-closure result
does not transfer to it.

### Q1.5 — What does “star jump” mean?

At least five different operations were in play:

1. advancing one unit on a shell;
2. selecting sparse star-number positions `S_n`;
3. selecting the star-prime subsequence;
4. moving between adjacent shells `S_n-1 / S_n`;
5. multiplying an anchor-product ladder.

Which is the manuscript operation, and which is only an index or search
accelerator?

### Q1.6 — Why star anchors, rather than arbitrary adjacent anchors?

Adjacent K-Elimination works for every `M,M+1`; composite `S_5=121` works too.
What additional invariant selects the centered-hexagonal/star family—geometry,
lane factorization, a manuscript diagram, a recurrence, or an implementation
cost optimum?

**Deliverable:** a property false for a reasonable control family.

### Q1.7 — What is the “triple pack”?

The remaster contains the concrete coprime basis `[4,9,37]` whose product shell
is `36*37=1,332`. Does the phrase refer to those three lanes, three rotations,
three stacked calendars, or a different object?

**Test:** map each member to a visible or executable role.

### Q1.8 — What observation would make us revise the Codex-machine reading?

List in advance the page patterns, operator failures, period mismatches, or
held-out decoding results that would lower confidence or select a competing
model.

**Why it matters:** a hypothesis that absorbs every outcome cannot identify a
machine.

## Tier 2 — formal and algorithmic questions

### Q2.1 — Can the least-observation theorem become a compiler?

Given source moduli and a target query, can software emit:

- the missing prime-power observation modulus;
- a direct residue decoder;
- compatibility predicates;
- a retained complement lane for reversibility;
- a Lean certificate; and
- cost/bit-width estimates?

This would unify the 584 and 23 constructions as instances of one tool.

### Q2.2 — Which existence proofs need executable refinements?

The set-theoretic factor theorem may use classical choice. Which DCCMS targets
need a computable decoder with explicit complexity, canonical representatives,
and refusal behavior?

### Q2.3 — What is the exact invariant for noncoprime division?

For FPD/coupled anchors, specify dividend domain, divisor, observation basis,
gcd classes, ambiguity certificate, and output query. Does the proposed fusion
always recover the exact quotient, recover a quotient class, or sometimes
correctly refuse?

### Q2.4 — Can winding remain derived rather than persisted?

For add, multiply, exact divide, comparison, and frame transfer, identify the
authentic readings from which each required winding query is derived. If a
scalar `K` is stored, state whether it is an oracle, boundary artifact, cache,
or architecturally required register.

### Q2.5 — When do transductions form a groupoid?

What are the objects: bases, typed observation frames, or source windows? Which
morphisms are total, partial, many-to-one, or invertible? Is equality of kernels
the condition for an isomorphism, while inclusion gives only a query map?

### Q2.6 — How are signed and unbounded domains represented?

Define canonical signed windows, overflow behavior, anchor extension, and the
conditions under which a residue class becomes an exact integer. Avoid treating
an unbounded coordinate as if a fixed finite tuple injected it.

### Q2.7 — What is the true cost model?

“One modular subtraction and multiplication” is a local decoder count. Include
the cost of authentic observation production, channel width, synchronization,
base changes, certification, and any retained complement state. State whether
`O(1)` means fixed channel count, parallel depth, or bit complexity.

## Tier 3 — implementation questions for DCCMS and NINE65

### Q3.1 — Which code path is authoritative?

Does production use the scalar `k_elim_divide`, the recombinant/Garner engine,
the formal observation-only constructor, or another NINE65 encrypted path?
Mark demonstrators, oracles, fallbacks, and cryptographic primary paths.

### Q3.2 — Can source authentication become a type-level invariant?

Introduce a frame token or producer-bound tuple so readings from different
epochs cannot be mixed accidentally. Compatibility modulo shared factors is
necessary but not sufficient.

### Q3.3 — Can the visual path be made genuinely pixel-derived?

Freeze training pages, held-out pages, segmentation rules, expected glyph
classes, and null images. Remove page number as a classifier input, then test
whether predictions survive.

### Q3.4 — What information do the packed Hydra heads retain?

The four full periods have 136,656,000 joint states, exceeding the 24-bit
packed address space. Which exact target queries remain recoverable from the
compressed signature, and which are intentionally discarded?

### Q3.5 — Where are noncoprime and width failures handled?

Test lane products beyond `u64`, repeated/noncoprime moduli, noncanonical
residues, and invalid inverse assumptions. Decide whether each is rejected,
reduced to an LCM basis, or handled by a certified alternate branch.

### Q3.6 — Is the NINE65 FHE use observation-only in the encrypted path?

Trace the actual ciphertext producer, readout, target query, noise/range bound,
and absence or presence of reconstruction. A clear data-flow proof matters more
than a similarly named helper in a different crate.

## Tier 4 — historical questions

### Q4.1 — What is the smallest candidate opcode dictionary?

Choose a limited set of signs with independently transcribed positions and
declare candidate operations before evaluating outcomes. A useful opcode must
predict arguments, outputs, and ordering, not only share a numerical motif.

### Q4.2 — Does the proposed machine beat a positional-calendar model?

Compare held-out predictive likelihood or error counts against ordinary
calendar arithmetic, scribal correction models, and known astronomical-table
procedures. The alternative must be implemented fairly.

### Q4.3 — Are numerical matches independent?

Build a dependency graph. For example, 365, 584, and 73 are not three
independent confirmations once `365=5*73` and `584=8*73` are selected. Count
independent source observations, not algebraic consequences.

### Q4.4 — Can an independent epigrapher reproduce the data extraction?

Provide page IDs, bounding boxes, glyph readings, damaged/ambiguous marks, and
transcription uncertainty without revealing the proposed computational label
until extraction is frozen.

### Q4.5 — What is the role of 819 under competing models?

Specify an operator-switch prediction that differs from planetary
commensuration accounts. Test both on held-out occurrences and report misses.

## Tier 5 — physical questions

### Q5.1 — What measurable state is proposed to be modular?

Give units, measurement procedure, noise model, evolution law, and how residues
are extracted. “Micro, macro, and celestial” must become three separate models
before universality across them is testable.

### Q5.2 — What prediction differs from conventional dynamics?

Topological pumps validate winding-sensitive observables but standard quantum
theory already predicts them. A physical-emergence claim needs a frozen result
on which the proposed model and the best conventional model disagree.

### Q5.3 — How do noninteger physical periods enter exact arithmetic?

Astronomical periods are measured quantities with variability and uncertainty.
Specify whether integers are table conventions, rational approximations,
quantized event counts, or exact postulates. Propagate uncertainty into phase
predictions.

### Q5.4 — What is “time” in the claim?

Separate:

- causal/transition order;
- operational tick count;
- elapsed duration;
- coordinate time in a physical theory; and
- a proposed ontological origin of time.

The first two can arise from a successor/winding system. The later claims need
calibration or physics beyond the residue theorem.

## Tier 6 — prime and number-theory questions

### Q6.1 — What exact superstar-prime conjecture is intended?

Define whether both a star number and its rank must be star numbers/primes, and
whether a lower twin, upper twin, or both are predicted. The examples 661 and
1,750,255,921 are evidence for a pattern, not yet a universal law.

### Q6.2 — Is the goal search acceleration or a primality theorem?

The congruence classes `p = 1 or 11 mod 12` for possible star-number divisors
give a sieve. Quantify its speedup and false-positive rate separately from a
proof of primality.

### Q6.3 — Does the least-anchor sequence have structure?

For consecutive star frames, the least raw extra moduli begin
`333, 584, 605, 1629, 1771, 5392, ...`; 584 is not constant. Factor this
sequence, derive a closed prime-power rule, and test whether any subfamily has
geometric, calendrical, or hardware significance.

## Recommended order

1. Freeze Q0.1–Q0.4 as typed definitions.
2. Resolve Q1.2–Q1.4 against actual manuscript data.
3. Build the compiler in Q2.1.
4. Repair/segregate the implementation paths in Q3.1–Q3.5.
5. Run a blinded historical test from Q4.1–Q4.4.
6. Only then design a physical differential test under Q5.1–Q5.4.

This order does not diminish the universal hypothesis. It makes every success
portable to other domains and makes every failure local enough to teach us
which layer needs revision.
