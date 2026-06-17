# `research/` — investigation materials

This tree holds research-control documents, seed data, and prototypes for the
parallel analysis angles the project is pursuing. It is **not** part of the Rust
workspace build. Nothing here compiles, and nothing here is load-bearing for any
claim until it is promoted through the gating rules below.

## Reliability discipline

These materials were authored in prior sessions (some by AI penmen) and uploaded
by the project owner. They are recorded here as **inputs to investigation**, not
as findings. Tag meanings match `RESEARCH_RECORD.md` and `FORMAL_THESIS.md`:

| Tag | Meaning |
|---|---|
| `[USER-PROVIDED]` | Uploaded by the project owner as a research input |
| `[UNRELIABLE]` | Interpretive claim; not independently verified against the manuscript |
| `[DIRECT-OBS]` | Confirmed against the Dresden Codex facsimile (`docs/facsimile/`) |

The promotion gates G0–G5 (visible observation → exact arithmetic → local
alignment → repeated alignment → cross-section recurrence → predictive
confirmation) are defined in `dresden_hcrm/01_dag/INVESTIGATION_DAG.md` and the
integration report. **Strong claims require G4 or G5.**

## Layout

```
research/
  dresden_hcrm/
    00_project_control/
      SESSION_INTEGRATION_REPORT_20260616.md   # session launch packet [USER-PROVIDED]
    01_dag/
      INVESTIGATION_DAG.md                      # gating DAG G0–G6 [USER-PROVIDED]
    03_skills/
      codex_natal_fields_checklist.md           # exhaustive data-field checklist [USER-PROVIDED]
    data/
      seed/
        period_prime_residue_seed.csv           # period → prime-residue seed table [USER-PROVIDED]
  hcrm_app/
    prototype/                                  # browser natal-chart prototype [UNRELIABLE]
    README.md
```

## Relationship to the rest of the repository

- The **primary source** is the Dresden Codex facsimile at
  `docs/facsimile/Dresden_Codex_Facsimile_WDL_11621.pdf` (WDL 11621).
- The **verified arithmetic** and the project thesis live in `FORMAL_THESIS.md`
  (290 checks, integer-only).
- The **session scrape** and source-reliability separation live in
  `RESEARCH_RECORD.md`.
- This `research/` tree holds the broader investigation framework that those
  documents draw on.

## Hard constraints (carried from the workspace)

- No floating-point arithmetic in any load-bearing path. All residue and cycle
  arithmetic must be integer / rational-pair / symbolic.
- The HCRM browser prototype under `hcrm_app/prototype/` is float-bearing
  JavaScript. It is a **UI concept only** and must be reimplemented with
  integer-arcsecond ephemeris input before any of its output is treated as
  load-bearing. See `hcrm_app/README.md`.
