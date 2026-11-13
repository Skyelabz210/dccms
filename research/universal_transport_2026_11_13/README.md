# Universal transport session synthesis — 2026-11-13

This directory is the canonical DCCMS record for the session that connected
K-Elimination, StarLift, the star-number family, calendar-period transport,
winding-sensitive dynamics, the Dresden Codex investigation, CRAM-DSP, and the
astrology remaster. It is intentionally bold about the mathematical structure
that survived exact tests and equally exact about what still needs an authentic
producer, a historical identification, or an empirical experiment.

## The central result

The common mechanism is **exact query transport through an observation**.
For a source set `X`, observation `B : X -> O`, and requested query
`C : X -> T`, an exact decoder `D` with `D(B(x)) = C(x)` exists exactly when

```text
B(x) = B(y)  implies  C(x) = C(y).
```

For additive maps this is the kernel condition `ker B <= ker C`. For integer
residue observations `X mod m_i`, the observation fibers differ by the joint
period `P = lcm(m_i)`. The query `X mod d` is therefore recoverable exactly
when `d | P`. This is the theorem-level core behind adjacent StarLift, bounded
calendar-phase constructors, least-extra-anchor synthesis, and complete-frame
transductions.

This is more general than one CRT reconstruction recipe. It tells us:

1. which answer is determined by the available observations;
2. what ambiguity remains when it is not;
3. the least additional raw residue observation that closes the ambiguity;
4. when a many-to-one query becomes a reversible frame after retaining a
   complementary lane; and
5. which historical or physical claims require evidence not present in the
   arithmetic theorem.

Shared factors are not a terminal obstruction in this architecture. They tell
us the resolution of the partial quotient view: on a lane `m`, division by
`d` leaves a view modulo `m/gcd(d,m)`. The Shadow/auxiliary frame supplies the
missing class, and the certified FPD/division dispatcher either returns the
exact quotient, a bounded result, or a typed refusal. Separately, this session
implemented an encrypted GCD consumer in NINE65. See
`evidence/SHADOW_GCD_RESOLUTION.md` for the source-bound correction.

## Strongest constructive results retained here

| Result | Status | Exact scope |
|---|---|---|
| Adjacent StarLift `K mod (M+1) = (v_M-v_(M+1)) mod (M+1)` | `[FORMAL-VERIFIED]` | Authentic same-source readings; full integer `K` additionally needs a range window |
| Universal observation/query factor criterion | `[FORMAL-VERIFIED]` | Conditional theorem; does not identify a physical source or manuscript operation |
| Least sufficient raw added modulus | `[FORMAL-VERIFIED]` general theorem, `[EXECUTED]` instances | Restricted to one additional observation of the form `X mod a` |
| `36/37 -> 72/73` using authentic `X mod 584` | `[PROVED-CONSTRUCTIVE] [EXECUTED]` | 194,472 complete source states and roundtrips |
| `260/584/23 -> X mod 11,960`, retaining mod 73 for reversibility | `[PROVED-CONSTRUCTIVE] [EXECUTED]` | 873,080 complete source states and roundtrips; idealized phase, not eclipse occurrence |
| Forward successor produces a phase and winding cocycle | `[PROVED-MODEL] [EXECUTED]` | Operational tick coordinate; elapsed duration requires a rate/calibration law |
| Star-number divisor classes | `[PROVED-NUMBER-THEORY] [EXECUTED]` | Structural sieve; not a complete arbitrary-size primality certificate |
| DCCMS default workspace | `[EXECUTED]` | 669 tests passed at source pin before this research-only commit |

## The observation lattice

The most important session-level abstraction is that the periods form an
**observation lattice**, not merely a list of calendars.

- `X mod a` refines `X mod b` when `b | a`.
- Combining full readings modulo `a` and `b` gives resolution `lcm(a,b)` after
  enforcing compatibility modulo `gcd(a,b)`.
- Forgetting detail to the greatest common quotient gives a `gcd` map.
- A requested period `d` is a valid exact query when its distinctions are
  constant on the current observation fibers.
- The finite diagrams used in software are finite views of the divisibility
  system; a full profinite inverse limit is a different, infinite object.

This makes “stacked and braided lattices” mathematically precise enough to
implement and test. It also exposes the crucial distinction between a **full
phase reading** and a compressed predicate such as a zero/nonzero carry bit.

## Two complete-frame constructors

### Next-star transport

For authentic readings `r = X mod 36` and `t = X mod 584`, define

```text
e   = ((t-r)/4) mod 2
u72 = r + 36e
v73 = t mod 73
```

Then `(u72,v73) = (X mod 72, X mod 73)`. The old mod-37 lane is not needed
for this bounded query, but retaining it makes the full frame reversible:

```text
(X mod 36, X mod 37, X mod 584)
    <->
(X mod 72, X mod 73, X mod 37).
```

The value 584 is forced for this source/target pair because the target period
`5256 = 72*73` needs the full missing prime-power factors `8*73`. It is not a
universal constant of all consecutive star transitions.

### Calendar-phase transport

For authentic readings `r = X mod 260`, `v = X mod 584`, and `w = X mod 23`,
define

```text
e = ((v-r)/4) mod 2
u = r + 260e                  # X mod 520
q = 5(w-u) mod 23
y = u + 520q                  # X mod 11,960
```

Retaining `t = X mod 73` gives the reversible frame

```text
(X mod 260, X mod 584, X mod 23)
    <->
(X mod 11,960, X mod 73).
```

The preexisting `260/365/584` full-phase observation has period 37,960 and
leaves exactly 23 possible phases modulo 11,960. The additional `X mod 23`
reading closes that ambiguity. Naming the factor 23 does not manufacture its
authentic reading.

## What this package does not silently identify

The same algebra can support several layers of inquiry without collapsing
them into one claim:

- a theorem about maps and kernels;
- an executable constructor on authenticated readings;
- a software architecture using those constructors;
- a proposed decoding of manuscript signs as observations/operators;
- a physical model connecting a measured system to those signs or residues.

Success at one layer is reusable upstream and downstream, but it is not an
automatic proof of the next layer. The open work is to bind the layers with
specific producers, frames, epochs, operator traces, and differential tests.

## Directory guide

- [CLAIM_LEDGER.md](CLAIM_LEDGER.md) — what is proved, executed, observed,
  hypothesized, or refuted as currently phrased.
- [QUESTION_LEDGER.md](QUESTION_LEDGER.md) — the questions now worth asking,
  ordered by their ability to unlock or falsify branches of the program.
- [MISSED_CONNECTIONS.md](MISSED_CONNECTIONS.md) — structural links exposed by
  stepping back across the entire session.
- [IMPLEMENTATION_AUDIT.md](IMPLEMENTATION_AUDIT.md) — where current DCCMS code
  matches the observation-only model and where it still uses scalar or
  page-derived shortcuts.
- [FORMAL_SCOPE.md](FORMAL_SCOPE.md) — proof pins, theorem inventory, and exact
  non-implications.
- [SOURCE_MANIFEST.md](SOURCE_MANIFEST.md) — immutable pins and public/private
  publication boundaries.
- `experiments/verify_transport.py` — standard-library exhaustive verification.
- `evidence/verification.json` — generated counts, identities, and transition
  table.
- `evidence/WORKSPACE_TEST.md` — fresh Rust workspace test environment and
  per-crate counts.
- `evidence/SHADOW_GCD_RESOLUTION.md` — correction recording the existing
  Shadow/FPD ambiguity resolution and encrypted GCD implementation.

## Reproduce

From the DCCMS repository root:

```bash
python3 research/universal_transport_2026_11_13/experiments/verify_transport.py \
  --output research/universal_transport_2026_11_13/evidence/verification.json
cargo test --workspace --locked
```

The Python verifier never uses a reconstructed `X` inside either decoder. Its
loop variable is only the independent oracle used to generate authentic input
readings and expected outputs.
