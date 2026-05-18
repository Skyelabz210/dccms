# DCCMS Imagery Sources — Dresden Codex Digital Editions

**Node:** NODE-SEG01 (executioner_dag.md)
**Date:** 2026-05-18
**Audience:** segmenter author (NODE-SEG02) and the upstream H4 visual transducer.
**Page focus:** Förstemann pages 16–24 (Moon Goddess / Ix Chel almanacs; H4 visual transducer targets).

---

## 1. Executive Summary

### Primary recommendation: SLUB Dresden digital facsimile
- **URL root:** http://digital.slub-dresden.de/id280742827
- **METS/OAI record:** https://digital.slub-dresden.de/oai/?verb=GetRecord&metadataPrefix=mets&identifier=oai:de:slub-dresden:db:id-280742827
- **Direct page-image URL pattern (verified):**
  `https://digital.slub-dresden.de/data/kitodo/codedrm_280742827/codedrm_280742827_tif/jpegs/{NNNNNNNN}.tif.original.jpg`
  where `{NNNNNNNN}` is the 8-digit zero-padded sheet index (01–78). Förstemann pages 16–24 correspond to sheet indices `00000016`–`00000024` in the SLUB ordering (note: SLUB page sequence follows Förstemann's canonical numbering; one-to-one for the front-of-codex pages 1–24).
- **License (verified from METS):** `dv:license = pdm` / `slubarchiv:copyrightStatus = publicdomain` / `mods:accessCondition xlink:href="http://creativecommons.org/publicdomain/mark/1.0/"`. This is **Public Domain Mark 1.0** with no legal restriction on commercial or derivative use; SLUB only requests citation of source.
- **Why primary:** original color photography of the actual manuscript; the highest-fidelity public source. The "ORIGINAL" file-group contains the full-resolution JPGs (single-page HEAD request on `00000020.tif.original.jpg` returns 5,446,491 bytes — consistent with multi-megapixel color page imagery). Pixel dimensions are not advertised on SLUB's pages; treat as "high resolution, exact dimensions unknown until first download".
- **Page numbering:** SLUB uses Förstemann's canonical 1–74 sequence. Matches the dccms_atlas convention already in code (`README.md` "pages 16–23").

### Backup recommendation: FAMSI Förstemann-Schele PDF set
- **URL (pages 13-24 segment):** https://www.famsi.org/mayawriting/codices/pdf/2_dresden_fors_schele_pp13-24.pdf (16.3 MB)
- **URL (full codex):** https://www.famsi.org/mayawriting/codices/pdf/dresden_fors_schele_all.pdf (95.7 MB)
- **Why backup, not primary:** Förstemann's 1880 chromolithograph facsimile is one historical generation removed from the original (it is itself a hand-redrawing). It does, however, **show the pre-1945-bombing state of the manuscript** for 8 pages with WWII water damage (pages 2, 4, 24, 28, 34, 38, 71, 72 per the user's `The Dresden Codex.md`). Page 24 sits inside the Moon Goddess target range, so the Förstemann edition is the disambiguator when the SLUB photograph is degraded. Schele's color-corrected version (the "Schele copy") is what FAMSI distributes, in color.
- **License:** public domain by age (Förstemann 1880); FAMSI itself does not assert rights restrictions on the PDFs.

### Compatibility ranking (top of full table in §3)
1. **SLUB Dresden — original photographs (color)** — 17/20
2. **FAMSI Förstemann-Schele color PDFs** — 14/20
3. **Wikimedia Commons Förstemann PDF (B&W)** — 13/20
4. **FAMSI Kingsborough PDFs (Aglio 1831 drawings)** — 11/20
5. **William Gates / Wayeb PDF** — 11/20
6. **Maya Codices Database (Vail / mayacodices.org)** — 11/20 (pre-segmented but low-res line drawings)
7. **Library of Congress / former World Digital Library** — 10/20 (record exists; deep links return 403/redirect)
8. **mayaexploration.org "First 23 pages" PDF** — 8/20
9. **ADEVA Graz facsimile via FAMSI mirror** — unranked (gallery viewer, license unclear, premium product)
10. **Villacorta 1930 line-drawings facsimile** — unranked (offline / out-of-print book, no canonical digital release found)

---

## 2. HackFate org / Skyelabz210 GitHub findings

The user's GitHub org is **not** an organization — `HackFate` is a user account with 0 public repos. The active developer account is **Skyelabz210**, which holds **`Skyelabz210/HackFate`** (a private "literature vault" repo) and the active **`Skyelabz210/dccms`** project. Both already contain detailed prior work on the imagery question.

| Repo : Path | Relevance |
|---|---|
| `Skyelabz210/HackFate : Dresden.md` | **Canonical user-authored source list.** Already names the SLUB persistent URL, FAMSI, the Wikimedia Förstemann PDF, and the Wayeb/Gates PDF. Body of the file is the master CRT-residue decoding write-up. |
| `Skyelabz210/HackFate : Dresden Coprime.md` | Establishes the **Förstemann vs. Kingsborough page-numbering history** (Aglio 1831 inverted ordering; Förstemann 1892 corrected it: K1=F44, K2=F45, K44=F1, K45=F2; obverse runs 24→46, reverse 25 follows 74). **The dccms code already uses Förstemann numbering** (`README.md` "pages 16–23"). Locks NODE-SEG01 to Förstemann ordering. |
| `Skyelabz210/HackFate : The Dresden Codex.md` | Names **the eight WWII-damaged pages** (2, 4, 24, 28, 34, 38, 71, 72). Page 24 overlaps the H4 target range — this is the technical reason to keep the Förstemann redrawing as backup. |
| `Skyelabz210/HackFate : Decoded.md` | "Algorithm 17: Biological Modular Interference (The Moon Goddess / Medical Almanacs)" — confirms downstream consumer expectations for the page-16–24 transducer. |
| `Skyelabz210/dccms : README.md`, `dccms_atlas/src/moon_goddess.rs`, `dccms_atlas/src/lunar.rs`, `dccms_atlas/examples/codex_decoder.rs` | Existing Rust code already framed around pages **16–23** (Förstemann numbering). The segmenter must match this convention. |
| `Skyelabz210/dccms : dresden_codex/` | Already-named substrate crate. Imagery enters at a higher layer — `dccms_atlas`. |
| `Skyelabz210/NS-Regularity : dresden_prime_hunt/`, `dresden_codex_bridge.rs`, `INSIGHT_MINING_REPORT.md` | Prior unrelated work on Dresden period arithmetic, not imagery. Mentioned here only because it confirms the Dresden ↔ CRAM coupling has been formalized; no imagery is referenced. |
| `Skyelabz210/prime-resonance : DresdenPanel.tsx`, `dresden_corrections.generated.ts` | UI / ephemeris layer. Not imagery. |
| `Skyelabz210/HackFate : possibly duplicate/HackFate/*` | Duplicates of the canonical Dresden notes; ignore. |

**Net finding:** the user has **already done the source-survey work** in `Skyelabz210/HackFate/Dresden.md`. NODE-SEG01 should treat that file as the upstream specification; this document operationalizes it.

The token "decoded" in the search returned mostly CRT decode/encode call sites in unrelated crypto crates (NINE65 / MYSTIC / BlindRoute) — **no prior imagery-segmentation work** appears in the HackFate org. NODE-SEG02 is greenfield.

---

## 3. Full source table with compatibility scores

Scoring axes (each 1–5, total /20):
- **R** = Resolution (5 = >4000 px page width, 1 = <1000 px)
- **S** = Segmentation friendliness (5 = pre-segmented glyphs with metadata, 1 = composite image only)
- **L** = License clarity (5 = explicit public domain mark / CC0, 1 = unclear)
- **P** = Page-numbering alignment (5 = standard Förstemann numbering, 1 = unclear / mixed)

| # | Source | URL | R | S | L | P | Total | Notes |
|---|---|---|---|---|---|---|---|---|
| 1 | **SLUB Dresden — Codex Dresdensis original photographs** | http://digital.slub-dresden.de/id280742827 | 4 | 1 | 5 | 5 | **17** | Color photos of the actual manuscript. Pixel dimensions unstated but file sizes (≈5 MB JPEG per page) indicate multi-megapixel. METS+IIIF available. PDM 1.0 verified in METS. Förstemann numbering. **No glyph segmentation metadata.** |
| 2 | FAMSI — Förstemann-Schele color PDF, full | https://www.famsi.org/mayawriting/codices/pdf/dresden_fors_schele_all.pdf | 3 | 1 | 4 | 5 | **13** | Color chromolithograph. Pages 13-24 PDF segment (16.3 MB) covers Moon Goddess range. Public-domain-by-age. |
| 2b | FAMSI — Förstemann-Schele color, **pages 13-24 only** | https://www.famsi.org/mayawriting/codices/pdf/2_dresden_fors_schele_pp13-24.pdf | 3 | 1 | 4 | 5 | **13** | Same source as #2, the sub-PDF for the exact target range. **Recommended backup for H4 pages 16-24.** |
| 3 | Wikimedia Commons — Förstemann (Schele copy, B&W) | https://commons.wikimedia.org/wiki/File:F%C3%B6rstemann_Dresden_Codex.pdf | 2 | 1 | 5 | 5 | **13** | 775 × 1664 px per page (verified) = below the 1000-px-width "no-go" threshold for fine-glyph segmentation. B&W. Strong license clarity (Wikimedia PDM). 74 pages. |
| 4 | FAMSI — Kingsborough/Aglio drawings | https://www.famsi.org/mayawriting/codices/pdf/kings.pdf | 2 | 1 | 4 | 2 | **9** | 1831 Aglio drawings. **Kingsborough page ordering is the legacy inverted scheme**; pages 1/45, 2/44 swap. Color hand-tinted. Mostly of historical interest; do not use as primary. |
| 5 | William Gates / Wayeb PDF | https://www.wayeb.org/download/resources/dresden01.pdf | 2 | 1 | 3 | 4 | **10** | Another scholarly facsimile redrawing; resolution unknown. License inherited from Gates (1932, public domain by age) but Wayeb's hosting terms unstated. |
| 6 | Maya Codices Database — Vail / Hernández | http://mayacodices.org/ | 1 | 5 | 2 | 4 | **12** | **Only source with pre-segmented glyph blocks and frame-clause metadata.** Per the project: "low resolution line drawings of each glyph block with an image of the frame (clause with picture) within which each glyph block occurs." License is restrictive: copyrighted by Gabrielle Vail, LLC; usage agreement applies. Low pixel resolution means it cannot be the visual substrate for the segmenter, but it is a **gold-standard ground-truth label set** for evaluating NODE-SEG02 output. |
| 7 | Library of Congress (former World Digital Library) | https://www.loc.gov/item/2021667917/ | ? | 1 | 4 | 5 | **— (10 est.)** | LoC item page returns 403 to automated fetch; gallery referenced as `loc.gov/resource/gdcwdl.wdl_11621/?st=gallery`. Hosts a copy of the SLUB photographs under LoC public-domain rules. Inferior to going directly to SLUB. |
| 8 | mayaexploration.org "First 23 pages" PDF | https://mayaexploration.org/pdf/DresdenCodex1-23.pdf | 2 | 1 | 2 | 4 | **9** | 2.3 MB mixed-content PDF; embedded images "approximately 1500+ px wide" per fetch probe. Scholarly commentary doc rather than a clean facsimile. Use as cross-reference only. |
| 9 | ADEVA Graz 1975 facsimile (FAMSI mirror) | http://www.famsi.org/research/graz/dresdensis/img_page02.html | ? | 1 | 1 | 4 | **— (≤8)** | Gallery viewer over the Graz premium facsimile. Resolution and license not posted. Likely the highest-fidelity color reproduction in print, but cannot rely on the FAMSI mirror without an explicit rights statement. **Skip unless SLUB and FAMSI-Förstemann both fail.** |
| 10 | Villacorta & Villacorta 1930 *Códices Mayas* | (no canonical digital host found) | ? | 1 | 4 | 5 | **— (≤10)** | B&W line drawings, published Guatemala 1930. Public domain by age. **No reliable open digital edition surfaced.** Cited in scholarly references but not directly accessible online for download. Out of scope for an automated pipeline. |

**Interpretation of S=1 across the photographic sources:** none of the photographic editions ship per-glyph bounding boxes or alpha masks. Glyph-level segmentation is exactly what NODE-SEG02 is being built to produce. The only source that already has segmentation (mayacodices.org) trades resolution for it — and its restrictive license blocks redistribution of derivative crops.

---

## 4. Recommended decision

1. **Primary substrate:** SLUB Dresden `.tif.original.jpg` page images, fetched on-demand by the segmenter from the URL pattern above. Cache locally under `dccms_atlas/data/slub/00000016.jpg` … `00000024.jpg` for the H4 target range. Cite "SLUB Dresden, Mscr.Dresd.R.310, http://digital.slub-dresden.de/id280742827 (Public Domain Mark 1.0)" in any derived artifact.
2. **Visual backup for damaged pages (page 24 in the H4 range):** FAMSI Förstemann-Schele `2_dresden_fors_schele_pp13-24.pdf`, used as a pre-1945 reference where SLUB's photograph shows WWII water damage.
3. **Ground-truth labels for evaluating the segmenter:** mayacodices.org glyph-block frames, used **read-only** (no redistribution) per their usage agreement. This is the closest thing to a labeled corpus and is the right thing to compare NODE-SEG02 bounding boxes against, even though the images themselves cannot ship in this repo.
4. **Defer:** ADEVA Graz, Villacorta 1930, LoC/WDL, Gates/Wayeb — useful at most as disambiguation references; not part of the automated pipeline.

---

## 5. Open questions for NODE-SEG02

- **Pixel dimensions of SLUB originals.** Filed as unknown — confirm by fetching the first page header on first segmenter run. If width is <2000 px, raise concern; if ≥3000 px, proceed.
- **IIIF endpoint.** SLUB advertises an IIIF manifest in its viewer UI but the METS does not embed the IIIF Image API service URI. Worth fetching the manifest JSON once (the URL is exposed from the workview UI) — IIIF would allow region cropping at fetch time, which is a free speedup for the segmenter.
- **mayacodices.org ground-truth.** The usage agreement (TLS cert error at fetch time) needs a human read before NODE-SEG02 publishes any evaluation metric that uses Vail's glyph IDs.

---

## 6. Machine-readable URL list

```
http://digital.slub-dresden.de/id280742827
https://digital.slub-dresden.de/en/workview/dlf/2967/1
https://digital.slub-dresden.de/oai/?verb=GetRecord&metadataPrefix=mets&identifier=oai:de:slub-dresden:db:id-280742827
https://digital.slub-dresden.de/data/kitodo/codedrm_280742827/codedrm_280742827_tif/jpegs/00000016.tif.original.jpg
https://digital.slub-dresden.de/data/kitodo/codedrm_280742827/codedrm_280742827_tif/jpegs/00000017.tif.original.jpg
https://digital.slub-dresden.de/data/kitodo/codedrm_280742827/codedrm_280742827_tif/jpegs/00000018.tif.original.jpg
https://digital.slub-dresden.de/data/kitodo/codedrm_280742827/codedrm_280742827_tif/jpegs/00000019.tif.original.jpg
https://digital.slub-dresden.de/data/kitodo/codedrm_280742827/codedrm_280742827_tif/jpegs/00000020.tif.original.jpg
https://digital.slub-dresden.de/data/kitodo/codedrm_280742827/codedrm_280742827_tif/jpegs/00000021.tif.original.jpg
https://digital.slub-dresden.de/data/kitodo/codedrm_280742827/codedrm_280742827_tif/jpegs/00000022.tif.original.jpg
https://digital.slub-dresden.de/data/kitodo/codedrm_280742827/codedrm_280742827_tif/jpegs/00000023.tif.original.jpg
https://digital.slub-dresden.de/data/kitodo/codedrm_280742827/codedrm_280742827_tif/jpegs/00000024.tif.original.jpg
https://digital.slub-dresden.de/data/kitodo/codedrm_280742827/codedrm_280742827_tif/jpegs/codedrm_280742827.pdf
https://www.famsi.org/mayawriting/codices/dresden.html
https://www.famsi.org/mayawriting/codices/pdf/dresden_fors_schele_all.pdf
https://www.famsi.org/mayawriting/codices/pdf/1_dresden_fors_schele_pp01-12.pdf
https://www.famsi.org/mayawriting/codices/pdf/2_dresden_fors_schele_pp13-24.pdf
https://www.famsi.org/mayawriting/codices/pdf/3_dresden_fors_schele_pp25-35.pdf
https://www.famsi.org/mayawriting/codices/pdf/4_dresden_fors_schele_pp36-45.pdf
https://www.famsi.org/mayawriting/codices/pdf/5_dresden_fors_schele_pp46-59.pdf
https://www.famsi.org/mayawriting/codices/pdf/6_dresden_fors_schele_pp60-74.pdf
https://www.famsi.org/mayawriting/codices/pdf/kings.pdf
https://commons.wikimedia.org/wiki/File:F%C3%B6rstemann_Dresden_Codex.pdf
https://www.wayeb.org/download/resources/dresden01.pdf
http://mayacodices.org/
http://mayacodices.org/agreement.htm
https://www.loc.gov/item/2021667917/
https://www.slub-dresden.de/en/explore/manuscripts/the-dresden-maya-codex
https://nutzungshinweis.slub-dresden.de/en
http://creativecommons.org/publicdomain/mark/1.0/
```
