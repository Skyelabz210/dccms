# HCRM Iridescent natal-chart prototype

`[UNRELIABLE] — UI prototype only.`

This is the browser prototype for the Human-Celestial Register Map (HCRM)
console: a natal chart treated as the human-domain projection of the same
cyclic basis the Dresden Codex investigation tracks. It was uploaded as a
concept surface, not a research-grade engine.

## What this is

`prototype/` contains the unpacked app: HTML entry points (`HCRM Console.html`,
`Resonance Spread.html`, `Test Harness.html`, and standalone/print variants),
the `.jsx` component sources, CSS, and reference screenshots under
`prototype/screens/`.

## Why it is not load-bearing yet

The prototype computes chart positions in **floating-point JavaScript**
(`parseFloat`, `Math.sin/cos`, decimal longitudes). The project prohibits
floating-point arithmetic in any load-bearing path, and the integration report
explicitly flags the required correction:

> Replace synthetic or decimal chart calculations with integer arcsecond
> ephemeris input. All longitudes, aspects, house cusps, and separations must
> be stored as integers or rational pairs.

Until that rewrite exists, treat the prototype as a UI mock. Its numeric output
must not feed any codex-comparison claim.

## Target pipeline (from the field checklist)

```
birth event
  -> integer arcsecond celestial coordinates
  -> sign / house / aspect / dignity / body-domain map
  -> prime residue basis {2,3,5,7,11,13}
  -> shadow (11) and boundary (13) signatures
  -> codex operator comparison
```

The exhaustive natal data-field schema (including the full residue column set it
records) is in `../dresden_hcrm/03_skills/codex_natal_fields_checklist.md`.
