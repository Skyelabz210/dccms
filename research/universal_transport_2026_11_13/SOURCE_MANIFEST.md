# Source manifest and publication boundary

This package consolidates repository-safe results. It does not copy private
Drive document bodies, user-uploaded unpublished papers, credentials, delivery
receipts, generated dependency trees, or complete mirrors of other repositories.
Those items are represented by immutable pins, hashes, and scoped descriptions.

## Repository pins inspected

| Repository | Commit | Use in this session |
|---|---|---|
| `Skyelabz210/dccms` | `40a76158970c41440ec7e4c10437fa1c2e271d03` | Baseline source, tests, manuscript-facing implementation audit |
| `Skyelabz210/cram-dsp` | `39fbe1ccdd0a31f3cb43cc97982fa196ed5c2b3b` | DSP comparison, independent and patch regression suites |
| `Skyelabz210/astrology_remastered` | `ad0c8380f02b1d0c4f05a3e00726be3bc9680058` | Star family, adjacent recovery, successor and winding primitives |
| `Skyelabz210/k-elimination-lean4` working proof tree | `dd9c6cfb341ad3747f148c336a015e2b80664e87` | Universal theorem campaign and least-anchor proofs |
| `Skyelabz210/NINE65_v7` cross-pollination workspace | `25bb6a7c464d97a439544d1d1e2d32f0baf3f6cb` | Original session packets and verification ledgers |

The DCCMS baseline was clean and equal to `origin/main` before this branch was
created.

## Primary manuscript sources

The public DCCMS facsimile remains at
`docs/facsimile/Dresden_Codex_Facsimile_WDL_11621.pdf`; its SHA-256 in the
baseline checkout is:

```text
239de6327effd7a5ae17f6d7da65f189da7fc260f697fb1417a84d27556e4ed4
```

The investigation independently downloaded the SLUB Dresden original from:

```text
https://digital.slub-dresden.de/data/kitodo/codedrm_280742827/
  codedrm_280742827_tif/jpegs/codedrm_280742827.pdf
sha256 ca18310851384f7ac03a765f2f3e797c6fcd9e4a5190ea100fd318a8e63317d9
```

The SLUB IIIF manifest was pinned at SHA-256
`27dca71fe46841cf316bb7abe9a3dd7174340890fad858188ee9569bb66907cf`.
Different PDF hashes refer to different public renderings/files and should not
be treated as a provenance contradiction.

## Session packets represented by this synthesis

The following original packets remain pinned in the NINE65 cross-pollination
repository at the commit above:

- `research/dccms-mechanism-investigation-2026-11-13/`
- `research/winding-prediction-test-2026-11-13/`
- `research/star-successor-clarification-2026-11-13/`
- `research/star-sequence-identification-2026-11-13/`
- `research/calendar-claim-audit-2026-11-13/`

This DCCMS directory is the canonical synthesis, question ledger, and compact
reproducer. The original packets retain detailed logs, external-source ledgers,
patches, and cross-repository probes without duplicating them here.

## Private or excluded inputs

Not republished in this public repository:

- unpublished user-uploaded PDF/DOCX/Markdown paper bodies;
- private Google Drive documents and account metadata;
- Drive delivery receipts and folder IDs;
- complete clones of `cram-dsp`, `astrology_remastered`, NINE65, or the Lean
  campaign tree;
- build directories, dependency caches, bytecode, and large generated logs;
- speculative text whose provenance could not be separated from private
  uploads.

Exclusion is a publication and duplication decision, not an assertion that the
materials are irrelevant. Claims drawn from them must be restated with public
evidence or explicit permission before becoming load-bearing here.

## Evidence classes

| Label | Meaning |
|---|---|
| `[FORMAL-VERIFIED]` | Checked in the pinned proof assistant environment |
| `[PROVED-CONSTRUCTIVE]` | Written finite arithmetic proof with an explicit decoder |
| `[EXECUTED]` | Code run over the stated finite domain or test suite |
| `[SOURCE-OBSERVED]` | Directly present in pinned source code or primary material |
| `[EXTERNAL-EVIDENCE]` | Reported by an independently published source with scope noted |
| `[HISTORICAL-HYPOTHESIS]` | Proposed manuscript interpretation requiring independent testing |
| `[PHYSICAL-HYPOTHESIS]` | Proposed law/model requiring measurement and comparison |
| `[REFUTED-AS-STATED]` | A concrete counterexample defeats the precise wording; a narrower claim may survive |

## Reproducibility policy

Every future addition should record:

1. source URL/repository and immutable revision or content hash;
2. exact command and environment needed to reproduce;
3. generated observation versus expected-answer oracle separation;
4. finite domain or sampling rule;
5. passed, failed, refused, and untested cases; and
6. whether the result is mathematical, implementation, historical, or physical.
