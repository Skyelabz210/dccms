# Claim ledger

Labels in this file separate mathematical truth, executed code, repository
behavior, and proposed interpretation.

| Claim | Status | Consequence / next gate |
|---|---|---|
| A query is exactly recoverable iff it is constant on observation fibers | `[FORMAL-VERIFIED]` | Use as the universal decision rule for every proposed decoder |
| For additive observations, recoverability is equivalent to kernel inclusion | `[FORMAL-VERIFIED]` | Supplies the algebraic factor theorem |
| Adjacent readings modulo `M` and `M+1` determine `floor(X/M) mod (M+1)` | `[FORMAL-VERIFIED]` | StarLift needs subtraction only because `M=-1 mod M+1` |
| A finite adjacent pair determines the unbounded quotient without a window | `[REFUTED-AS-STATED]` | It determines a residue class; exact integer recovery needs an admissible interval or more observations |
| StarLift computes `(-K) mod A` by reversing operands | `[FORMAL-VERIFIED]` | “Additive inverse of infinity” has a precise bounded-query interpretation |
| The star family is `S_n=6n(n-1)+1` and each nondegenerate shell is adjacent | `[SOURCE-OBSERVED] [EXECUTED]` | Primality is unnecessary for adjacent lifting |
| Every consecutive star transition uses 584 | `[REFUTED]` | 584 is least only for the chosen `36/37 -> 72/73` transition |
| An authentic mod-584 reading transports `36/37 -> 72/73` | `[PROVED-CONSTRUCTIVE] [EXECUTED]` | Candidate bridge into the Venus lane; historical producer still needed |
| Retaining mod 37 turns that star query into a reversible full-frame map | `[PROVED-CONSTRUCTIVE] [EXECUTED]` | General pattern: query plus complement lane can produce an isomorphism |
| Full calendar phase readings combine by generalized CRT | `[PROVED-STANDARD]` | Their joint resolution is an LCM, with GCD compatibility checks |
| The named periods are pairwise coprime lanes | `[REFUTED]` | They overlap at 4, 5, 13, 73 and must be treated as a compatible lattice |
| `260/365/584` leaves 23 phases modulo 11,960 | `[PROVED] [EXECUTED]` | An authentic mod-23 reading is sufficient from this particular source frame |
| A raw mod-23 lane is the only possible resolver of those 23 classes | `[REFUTED]` | Any authenticated observation injective on the 23 classes suffices; the direct constructor uses mod 23, while a lifted Shadow frame may carry the class through a different encoding |
| Factor 23 alone supplies the reading | `[REFUTED]` | A factor is not an observation producer |
| The surviving eclipse table is exactly a modulus of 11,960 | `[HISTORICALLY-SENSITIVE]` | Recent scholarship distinguishes a 11,959-day surviving table from the common 11,960 model |
| The 11,960 constructor predicts eclipses | `[NOT-ESTABLISHED]` | It predicts a modeled modular phase only; an astronomical event query needs a physical model |
| `1,872,000 = 98*18,980 + 11,960` | `[PROVED]` | Strong boundary identity; historical operational meaning remains a testable hypothesis |
| The Long Count is exactly `floor(X/18,980)` | `[REFUTED-AS-STATED]` | It is a mixed-radix day count from which that winding can be derived |
| A forward successor law induces phase plus winding | `[PROVED-MODEL] [EXECUTED]` | Gives an operational tick and orientation once the transition is chosen |
| Phase plus net winding determines elapsed physical duration without calibration | `[REFUTED-IN-GENERAL]` | Rate, pauses, and admitted histories matter |
| Winding-sensitive transport occurs in an unrelated physical domain | `[EXTERNAL-EVIDENCE]` | Thouless pumps supply an application, while conventional theory predicts it too |
| Physical time emerges from winding | `[OPEN-PHYSICAL-HYPOTHESIS]` | Needs explicit dynamics and a measurement that differs from conventional predictions |
| A shared factor makes exact division impossible | `[REFUTED]` | The lane yields `q mod (m/g)` for `g=gcd(d,m)`; Shadow/auxiliary coordinates and a bound restore the missing class, or the certified dispatcher refuses |
| GCD remained an unsolved runtime blocker after this session | `[REFUTED]` | NINE65 implemented bounded encrypted GCD with direct-target and fixed-round Euclid strategies and recorded native ciphertext evidence |
| Shadow Prime is merely another coprime CRT lane | `[REFUTED-AS-STATED]` | It is a typed integrity/disambiguation coordinate; when 11 is already in the payload, an independent phase-locked anchor or lifted Shadow family supplies the required invertible coordinate |
| DCCMS currently performs observation-only K-Elimination end to end | `[NOT-YET]` | Current division accepts scalars and emits `a/b`; see implementation audit |
| DCCMS visual H4 has been independently discharged by image content | `[REFUTED-BY-CURRENT-PATH]` | Current classification remains page-conditioned; a held-out pixel-derived classifier is required |
| The universal theorem proves the Codex implemented the mechanism | `[NOT-IMPLIED]` | Requires a source-bound sign/operator correspondence and held-out prediction |
| Profinite means finite | `[REFUTED]` | Finite quotients can form an infinite, uncountable inverse limit |
| The mathematics is merely a restatement of Garner reconstruction | `[TOO-COARSE]` | Particular constructions can overlap prior CRT techniques; the query-factor criterion and architecture must be compared at the exact claim level |

## Novelty should be stated at the right layer

Potential contribution layers are independently assessable:

1. a theorem or proof formulation;
2. a constructive minimal-observation compiler;
3. a no-persistent-scalar-winding architecture;
4. a reversible typed transduction protocol;
5. a noncoprime exact-division construction;
6. a manuscript decoding with predictive success;
7. a physical law.

A known algebraic identity in one component does not settle novelty of the
whole architecture, just as using common grammar does not settle authorship of
a work. Conversely, architectural novelty does not make every component a new
theorem. Each claim should identify its layer and closest exact comparator.
