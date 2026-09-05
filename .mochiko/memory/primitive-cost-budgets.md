# Primitive cost budgets — the D7 char-budget ledger

Provenance: `validator-scope-and-verbosity` D7 (record:
`.mochiko/brainstorms/validator-scope-and-verbosity/record.md`), the guardrails-vs-detail
benchmark verdict (`DECISIONS.md` 2026-08-10 benchmark-verdict row;
`.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`), the Wave 1 build
(v0.63.0), and the Wave 2 editorial extension (v0.64.0 — budgets seeded from the audited cut
results per the Wave 2 sketch in `report/build-plan.md`). Cited by the char-budget pre-assert
in `.claude/rules/mochiko/primitive-edits.md`.

**The rule.** A budget is the measured winning-variant character count **+25% headroom**
(rounded up). **A re-asserted or re-measured figure is a current measurement, never a
re-derivation base** — running ×1.25 over a grown body would ratchet the budget up every time a
primitive is edited, which is exactly the drift the budget exists to catch. A budget moves only by
the seeding paths below. Budgets are always measured, never invented: a new budget enters this
ledger only from a benchmark-measured winner, a ruled editorial cut shipped through the edit
ceremony, or **a ruled schema conversion** (skill-content-schema D8/C1: the budget re-seeds to
the measured post-conversion delivered-at-invoke payload with **no +25% headroom** — a stated
departure from the headroom rule; the conversion is a relocation, never a measured winner, so
the first post-conversion character takes the argued-overage path)
(Wave 2 seeded its budgets from its own audited cut results this way at v0.64.0). All counts
are **characters of the parsed value — never `wc -c` bytes** (byte counts over-reject
unicode-bearing text that is under the char cap; a v1 census defect was exactly this).

**Classes measured:** skill body (content after the closing frontmatter `---`) · skill
frontmatter `description:` value · agent frontmatter `description:` value. The exemption keys on
**never auto-loaded**: `references/` and `scripts/` files stay exempt (on-demand data), but a
schema whose read is obligated at invoke is budgeted — a converted skill's budgeted quantity is
its delivered-at-invoke payload, `SKILL.md` body plus its own `schema.yaml`, one number
(skill-content-schema D8 as amended by C1, `DECISIONS.md` 2026-09-01). Command schema data
files stay unbudgeted with their commands.

**Last authoritative sweep: v0.81.0 release gate** (2026-08-19) — every wave-touched skill and
agent re-measured with the canonical snippet in one pass, against the quiesced tree, after all
validator verdicts sealed. 15 primitives measured, 14 matched their recorded figures exactly, 1
had drifted (`authoring-architecture-store`, +54, corrected below). Sweeping at the gate rather
than trusting per-seat reports is what caught it: a figure reported mid-fix-round is a snapshot,
and the tree kept moving after several of this wave's were taken.

## Skill bodies

Wave 1 rows (benchmark winners, v0.63.0) and Wave 2 rows (audited editorial-cut results,
v0.64.0) share the same rule. `authoring-architecture` and `grooming-operating-docs` were
audited Wave 2 body no-ops (nothing D4-cuttable); their budgets are their surviving bodies
+25%, same as every other row.

| skill | winner chars | budget (+25%) |
|---|---|---|
| analysis-iterative | 3,942 | 4,928 |
| analysis-codebase | 13,776 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 4,688 + render 9,088; was 12,098 — re-seeded [v0.103.0] schema conversion — payload: body 3,814 + schema 8,284; was 6,509/8,137) | 13,776 (no headroom) |
| authoring-constitution | 29,614 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 7,695 + render 21,919; was 30,387 — re-seeded [v0.101.0] schema conversion — payload: body 7,255 + schema 23,132, re-measured post-fix-round; was 17,240/21,550) | 29,614 (no headroom) |
| authoring-feature-map | 22,323 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 5,936 + render 16,387; was 21,636 — re-seeded [v0.101.0] schema conversion — payload: body 5,505 + schema 16,131; was 12,330/15,413, standing +562 overage dissolved — see the [v0.101.0] note below) | 22,323 (no headroom) |
| review-governance-intent | 16,274 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 3,160 + render 13,114; was 14,663 — re-seeded [v0.100.0] schema conversion — payload: body 2,662 + schema 12,001; was 5,562/6,953) | 16,274 (no headroom) |
| validation-constitution | 15,017 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 3,263 + render 11,754; was 13,285 — re-seeded [v0.100.0] schema conversion — payload: body 2,800 + schema 10,485, re-measured post-fix-round; was 5,103/6,379) | 15,017 (no headroom) |
| testing-governance-injection | 3,540 | 4,425 |
| authoring-requirements | 12,373 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 3,439 + render 8,934; was 10,796 — re-seeded [v0.101.0] schema conversion — payload: body 2,761 + schema 8,035; was 4,101/5,127) | 12,373 (no headroom) |
| authoring-user-stories | 13,444 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 4,529 + render 8,915; was 11,668 — re-seeded [v0.101.0] schema conversion — payload: body 3,775 + schema 7,893; was 5,361/6,702) | 13,444 (no headroom) |
| authoring-prototype | 14,980 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 4,601 + render 10,379; was 13,943 — re-seeded [v0.101.0] schema conversion — payload: body 4,353 + schema 9,590; was 8,898/11,123) | 14,980 (no headroom) |
| review-specifications | 16,174 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 3,441 + render 12,733; was 15,600 — re-seeded [v0.100.0] schema conversion — payload: body 3,182 + schema 12,418; was 6,187/7,734) | 16,174 (no headroom) |
| ~~authoring-architecture~~ | 5,250 | 6,563 |
| authoring-technical-requirements | 20,775 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 4,038 + render 16,737; was 19,946 — re-seeded [v0.101.0] schema conversion — payload: body 3,204 + schema 16,742; was 10,628/13,285) | 20,775 (no headroom) |
| brownfield-integration | 12,854 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 4,831 + render 8,023; was 10,577 — re-seeded [v0.103.0] schema conversion — payload: body 3,903 + schema 6,674; was 6,342/7,928) | 12,854 (no headroom) |
| executing-tdd-cycle | 20,063 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 7,340 + render 12,723; was 18,951 — re-seeded [v0.103.0] schema conversion — payload: body 6,464 + schema 12,487; was 9,676/12,095) | 20,063 (no headroom) |
| grooming-operating-docs | 2,666 | 3,333 |
| testing-end-user | 21,710 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 9,563 + render 12,147; was 20,791 — re-seeded [v0.103.0] schema conversion — payload: body 8,696 + schema 12,095; was 13,125/16,407) | 21,710 (no headroom) |
| patterns-api-contracts | 10,729 | 13,412 |
| patterns-code-minimalism | 10,318 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 3,259 + render 7,059; was 8,024 — re-seeded [v0.102.0] schema conversion — payload: body 2,235 + schema 5,789; was 3,455/4,319) | 10,318 (no headroom) |
| patterns-entity-modeling | 13,468 | 16,835 |
| patterns-system-design | 9,304 (re-asserted v0.81.0) | 11,047 |
| patterns-technical-decisions | 4,626 | 5,783 |
| patterns-vertical-tdd | 15,775 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 5,896 + render 9,879; was 14,325 — re-seeded [v0.102.0] schema conversion — payload: body 5,145 + schema 9,180, re-measured post-fix-round; was 5,189/6,487 with a standing +294 ruled-HOLDS overage at body 6,781 — absorbed and superseded by this re-seed, trail in `.mochiko/strips/patterns-vertical-tdd.md` [v0.102.0] and the v0.80.0/v0.91.0 paragraphs below) | 15,775 (no headroom) |
| review-brainstorm | 12,824 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 2,833 + render 9,991; was 11,470 — re-seeded [v0.100.0] schema conversion — payload: body 2,328 + schema 9,142, re-measured post-fix-round; was 2,497/3,122) | 12,824 (no headroom) |
| review-feasibility | 12,220 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 2,721 + render 9,499; was 10,572 — re-seeded [v0.100.0] schema conversion — payload: body 2,195 + schema 8,377; was 1,893/2,367) | 12,220 (no headroom) |
| review-plan-artifacts | 18,013 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 3,363 + render 14,650; was 17,890 — re-seeded [v0.100.0] schema conversion — payload: body 3,170 + schema 14,720; was 4,901/6,127) | 18,013 (no headroom) |
| review-code-minimalism | 10,664 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 3,711 + render 6,953; was 8,449 — re-seeded [v0.100.0] schema conversion — payload: body 2,971 + schema 5,478, re-measured post-fix-round; was 3,689/4,612) | 10,664 (no headroom) |
| review-sufficiency | 15,423 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 3,262 + render 12,161; was 14,950 — first budget row, seeded [v0.100.0] schema conversion — payload: body 2,966 + schema 11,984, re-measured post-fix-round; the +82 over the first seed is the lead-ruled F5 restored clause + the F4 `kind:` line; unbudgeted at birth, body 6,652 at v0.91.0) | 15,423 (no headroom) |
| skill-review-common.yaml | 1,627 (family common file, budgeted once — seeded [v0.100.0] schema conversion; a log document, not a shipped file, from [v0.107.0] — see the note) | 1,627 (no headroom) |
| authoring-architecture-store | 19,733 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 5,391 + render 14,342; was 18,876 — first budget row, seeded [v0.101.0] schema conversion — payload: body 4,904 + schema 13,972, re-measured post-fix-round; unbudgeted at birth, v0.81.0 hard-cap-only) | 19,733 (no headroom) |
| authoring-epic | 14,062 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 3,129 + render 10,933; was 13,044 — first budget row, seeded [v0.101.0] schema conversion — payload: body 2,569 + schema 10,475; unbudgeted at birth, v0.72.0 hard-cap-only) | 14,062 (no headroom) |
| skill-authoring-common.yaml | 1,285 (family common file, budgeted once — seeded [v0.101.0] schema conversion, re-measured after the epigraph-block label-line drop; a log document, not a shipped file, from [v0.107.0] — see the note) | 1,285 (no headroom) |
| patterns-adopt-first | 14,242 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 3,061 + render 11,181; was 12,910 — first budget row, seeded [v0.102.0] schema conversion — payload: body 2,253 + schema 10,657; unbudgeted at birth, v0.73.0 build-cap-only) | 14,242 (no headroom) |
| patterns-architecture-shelves | 14,493 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 4,003 + render 10,490; was 12,361 — first budget row, seeded [v0.102.0] schema conversion — payload: body 2,923 + schema 9,438, re-measured post-fix-round; unbudgeted at birth, v0.81.0 hard-cap-only) | 14,493 (no headroom) |
| patterns-map-minimalism | 11,245 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 3,288 + render 7,957; was 9,362 — first budget row, seeded [v0.102.0] schema conversion — payload: body 2,470 + schema 6,892; unbudgeted at birth, v0.68.0 hard-cap-only) | 11,245 (no headroom) |
| patterns-model-tiering | 10,852 (standing **+3,844 ruled-HOLDS overage at [v0.108.0]** — payload 14,696: body 2,873 + render 11,823; the sonnet-worker-rung ruling's six minted rules and four rewords, ADR `2026-09-05-sonnet-worker-rung`, audit V1 + its delta ruled the growth a genuine new obligation set; re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 2,633 + render 8,219; was 8,793 — first budget row, seeded [v0.102.0] schema conversion — payload: body 1,614 + schema 7,179, re-measured post-fix-round; unbudgeted at birth, v0.77.0 hard-cap-only) | 10,852 (no headroom) |
| patterns-plan-minimalism | 10,824 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 3,834 + render 6,990; was 8,594 — first budget row, seeded [v0.102.0] schema conversion — payload: body 2,812 + schema 5,782; unbudgeted at birth, v0.67.0 hard-cap-only) | 10,824 (no headroom) |
| patterns-sound-loop | 12,388 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 2,812 + render 9,576; was 10,933 — first budget row, seeded [v0.102.0] schema conversion — payload: body 2,022 + schema 8,911, re-measured post-fix-round; unbudgeted at birth, v0.70.0 hard-cap-only) | 12,388 (no headroom) |
| patterns-transport-floor | 12,564 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 3,114 + render 9,450; was 10,556 — first budget row, seeded [v0.102.0] schema conversion — payload: body 2,412 + schema 8,144; unbudgeted at birth, v0.71.0 hard-cap-only) | 12,564 (no headroom) |
| testing-gap-finding | 19,977 (re-seeded [v0.106.0] CLI delivery, D10.6 — payload: body 5,954 + render 14,023; was 19,382 — first budget row, seeded [v0.103.0] schema conversion — payload: body 5,271 + schema 14,111; unbudgeted at birth, v0.79.0 hard-cap-only) | 19,977 (no headroom) |

The v0.63.0 floor-line additions (analysis-iterative, review-governance-intent,
validation-constitution, review-specifications) and the v0.64.0 ones (review-brainstorm,
review-feasibility, review-plan-artifacts) ride inside these budgets — the +25% headroom
absorbed them by design.

`review-governance-intent` re-seeded at v0.89.0 and `validation-constitution` at v0.90.0
per the compression rule (R11): user-ruled true-deletion cuts landed the bodies at 5,562
(was 8,150 against 8,862; budget 6,953) and 5,103 (was 7,630 against 8,418; budget 6,379).
Both floors shallow by structure — the v0.63.0 benchmark wave had already strip-cut both
(−46%/−44%), so these passes compressed keep-sets. Rulings + disposition maps: the
[v0.89.0]/[v0.90.0] strip entries and the two 2026-08-26 ADRs. Both eval slots run as
post-cut regression checks; inventories non-compressor-authored under `evals/`.

`review-specifications` re-seeded at v0.88.0 per the compression rule (R11): the user-ruled
true-deletion cut landed the body at 6,187 (was 12,184 against 14,089); budget 6,187 × 1.25 =
7,734 (rounded up). Ruling + disposition map: `.mochiko/strips/review-specifications.md`
[v0.88.0] and `.mochiko/decisions/2026-08-26-review-specifications-true-deletion-cut.md`.
Its compression-eval slot runs as a post-cut regression check; the 81-entry rule inventory is
non-compressor-authored at `evals/review-specifications/rules.json`.

`review-plan-artifacts` re-seeded at v0.87.0 per the compression rule (R11): the user-ruled
true-deletion cut landed the body at 4,901 (was 13,521 against 13,569); budget 4,901 × 1.25 =
6,127 (rounded up). Ruling + disposition map: `.mochiko/strips/review-plan-artifacts.md`
[v0.87.0] and `.mochiko/decisions/2026-08-26-review-plan-artifacts-true-deletion-cut.md`.
Its compression-eval slot runs as a post-cut regression check (the standing v0.82.0 pattern);
the 113-entry rule inventory is already non-compressor-authored at
`evals/review-plan-artifacts/rules.json`.

`review-feasibility` re-seeded at v0.82.0 per the compression rule (R11 — headroom does not
survive a cut): the user-ruled 90% body cut with breakup into `references/` landed the body at
1,893 (was 18,959 against 19,058); budget 1,893 × 1.25 = 2,367. Ruling + disposition map:
`.mochiko/strips/review-feasibility.md` [v0.82.0] and
`.mochiko/decisions/2026-08-22-verbosity-envelope-enforcement.md`. The skill-compression eval
pilot for this skill is re-purposed as a post-cut regression check (cut-vs-baseline rule
coverage); a lost load-bearing rule re-adds via the strips re-add path.

**[v0.100.0] skill-content-schema conversion re-seeds — the third seeding path's first use.**
The eight review-family rows above (`review-sufficiency` gaining its first row) and the
`skill-review-common.yaml` row are conversion re-seeds per skill-content-schema D8 as amended by
C1 (`DECISIONS.md` 2026-09-01): the budgeted quantity for a converted skill is its
**delivered-at-invoke payload** — canonical-snippet `SKILL.md` body chars plus whole-file
`schema.yaml` chars, one number — and the budget equals the measured payload exactly, **no +25%
headroom** (the stated departure: a conversion is a relocation, never a measured winner, so the
first post-conversion character takes the argued-overage path). All nine figures are
canonical-snippet counts taken 2026-09-01 against the quiesced post-conversion tree, after the
full `check-skill-schema.py` sweep passed. The common file is budgeted once as its own
primitive, never per binding skill. Descriptions are byte-untouched by the conversion (RB 490 ·
RCM 492 · RF 599 · RGI 483 · RPA 598 · RSPEC 490 · RSUF 686 · VC 481 — all under their existing
description budgets and the 1,536 cap). Prior winner/budget figures are kept visible in each
row as history, per the `system-architect`/`authoring-architecture` precedent. Rulings +
disposition maps: the [v0.100.0] entries in each member's `.mochiko/strips/` file and the
2026-09-01 skill-content-schema `DECISIONS.md` row.

**[v0.101.0] authoring-family conversion re-seeds — the third seeding path's second use.**
The eight authoring-family rows above and the `skill-authoring-common.yaml` row are
conversion re-seeds per skill-content-schema D8/C1 and the wave-2 family-door ruling
(`DECISIONS.md` 2026-09-01): budget = the measured delivered-at-invoke payload exactly, no
+25% headroom. Six rows re-seed (AC · AFM · AP · AR · ATR · AUS); **two are first-seeds** —
`authoring-architecture-store` and `authoring-epic` were unbudgeted at birth under the
"budgets are never invented" clause, and gain their first rows here because the third
seeding path ("a ruled schema conversion") composes with that clause: the conversion is a
ruled measurement event, not an invention (census-authoring J-7; their unbudgeted-at-birth
paragraphs below stay as history). **`authoring-feature-map`'s standing ruled overage
(+562, ruled HOLDS at v0.91.0) dissolves into its re-seed** — the budget row no longer
carries it, and its ruling trail survives in `.mochiko/strips/authoring-feature-map.md`
([v0.91.0] byte-reconciliation + [v0.101.0] conversion entry) per census-authoring J-6
(GI-006 reconstruction). All nine figures are canonical-snippet counts taken 2026-09-01 by
the closer seat against the quiesced post-conversion tree (never seat-reported numbers —
the wave-1 V2/R1 lesson). The common file is budgeted once as its own primitive, never per
binding skill. Descriptions are byte-untouched by the conversion (AAS 492 · AC 481 ·
AE 496 · AFM 598 · AP 493 · AR 379 · ATR 598 · AUS 425 — all under their existing
description budgets and the 1,536 cap). Family delivered-at-invoke, measured (final, post-fix-round re-measure of AAS · AC · the
common file): 140,296 budgeted payload + 1,285 common ×8 reads = 150,576 vs the 81,896
pre-conversion family body — **×1.84 against the census estimate ×2.35** (the wave-1 estimate-to-measured drift
did not recur; figure returns to the user at the landing gate). Rulings + disposition
maps: the [v0.101.0] entries in each member's `.mochiko/strips/` file and the 2026-09-01
`DECISIONS.md` rows.

**[v0.102.0] patterns-family conversion re-seeds — the third seeding path's third use.**
The nine patterns-carrier rows above are conversion re-seeds and first-seeds per
skill-content-schema D8/C1 and the wave-2 patterns family-door ruling (`DECISIONS.md`
2026-09-01): budget = the measured delivered-at-invoke payload exactly, no +25% headroom.
Two rows re-seed (`patterns-code-minimalism` · `patterns-vertical-tdd`); **seven are
first-seeds** — `patterns-adopt-first` · `patterns-architecture-shelves` ·
`patterns-map-minimalism` · `patterns-model-tiering` · `patterns-plan-minimalism` ·
`patterns-sound-loop` · `patterns-transport-floor` were unbudgeted at birth under the
"budgets are never invented" clause and gain their first rows via **the third seeding
path ("a ruled schema conversion")**, which composes with that clause: the conversion is
a ruled measurement event, not an invention (the wave-2A AAS/AE precedent; their
unbudgeted-at-birth paragraphs below stay as history).
**`patterns-vertical-tdd`'s standing ruled overage (+294, declared at v0.80.0/v0.91.0
and ruled HOLDS) is absorbed and superseded by its re-seed** — the budget row no longer
carries it; the trail survives in `.mochiko/strips/patterns-vertical-tdd.md`
([v0.80.0]/[v0.91.0] declarations + the [v0.102.0] conversion entry) and the
v0.80.0/v0.91.0 paragraphs below (census-patterns J-P5; GI-006 reconstruction). No
common file exists for this family (census-patterns §C/§ROAD — the door ruling minted
none), so no common-file row rides this wave. All nine figures are canonical-snippet
counts taken 2026-09-01 by the closer seat against the tree as landed — **re-measured
once after the W1/W2 audit fix rounds landed** (the AS/MT/SL text repairs and the VT
sentence cut; four rows moved and carry "re-measured post-fix-round"), never
seat-reported numbers (the wave-1 V2/R1 lesson). Descriptions are byte-untouched by the
conversion (AF 610 · AS 473 · CM 564 · MM 499 · MT 655 · PM 600 · SL 505 ·
TF 450 · VT 497 — all under the 1,536 cap; CM's protected RETURNED clause stays
description-borne per census J-P3). Family delivered-at-invoke, measured (final,
post-fix-round): **95,858 vs the 50,379 pre-conversion nine-carrier bodies = ×1.90,
against the census estimate ×1.81 (~91.2k)** — +5.1% over estimate, well inside the
census's stated ±25% band; the measured figure returns to the user at the landing gate.
Rulings + disposition maps: the [v0.102.0] entries in each member's `.mochiko/strips/`
file and the 2026-09-01 `DECISIONS.md` rows.

**[v0.103.0] small-families dense-five conversion re-seeds — the third seeding path's
fourth use.** The five dense-five rows above are conversion re-seeds and a first-seed per
skill-content-schema D8/C1 and the 2026-09-01 small-families door ruling (`DECISIONS.md`
2026-09-01 — abort TRIPPED, zero common blocks; the dense five convert on the B/C
drivers): budget = the measured delivered-at-invoke payload exactly, no +25% headroom.
Four rows re-seed (`testing-end-user` · `executing-tdd-cycle` · `brownfield-integration` ·
`analysis-codebase`); **one is a first-seed** — `testing-gap-finding` was unbudgeted at
birth under the "budgets are never invented" clause and gains its first row via **the
third seeding path ("a ruled schema conversion")**, which composes with that clause: the
conversion is a ruled measurement event, not an invention (the wave-2A AAS/AE precedent;
its unbudgeted-at-birth paragraph below stays as history). **No common file rides this
wave** — census-small-families §C/§D found zero blocks clearing the 3+ bar in any
micro-family, so delivered-at-invoke = budgeted payload, no common-read rider. All five
figures are canonical-snippet counts taken 2026-09-01 by the closer seat against the
landed tree, never seat-reported numbers (the wave-1 V2/R1 lesson; the closer's counts
reconcile with the seat snapshots exactly). Descriptions are byte-untouched by the
conversion (TEU 500 · TGF 709 · ETC 498 · BI 491 · AC 349 — all under the 1,536 cap;
TGF's 709 stays the library's largest, ruled HOLDS at v0.79.0). Family
delivered-at-invoke, measured: **81,799 vs the 46,809 pre-conversion five-member bodies =
×1.75, against the census estimate ~78,500 (×1.68)** — +4.2% over estimate, well inside
the census's stated ±30% band; the wave-1 estimate-to-measured drift did not recur.
**This wave closes the wave-2 conversion arc: 30 converted pairs of 38 skill
directories** — 8 review (v0.100.0) + 8 authoring (v0.101.0) + 9 patterns carriers
(v0.102.0) + 5 dense-five (this wave); 7 stay prose by ruling (`analysis-iterative` ·
`grooming-operating-docs` · `testing-governance-injection` + the four patterns teachers);
the `mochiko` router out of scope (D1). Rulings + disposition maps: the [v0.103.0]
entries in each member's `.mochiko/strips/` file and the 2026-09-01 `DECISIONS.md` rows.

**[v0.106.0] wave-5 CLI-delivery re-seeds — the budgeted quantity itself re-keys.** The thirty
converted-skill rows above are re-seeded per `cli-schema-delivery` D10 clause 6 (`DECISIONS.md`
2026-09-04): a converted skill reads no schema file at invoke, so its delivered-at-invoke payload
is no longer body + `schema.yaml` but **the `SKILL.md` body plus the seven blocks `mochiko-cli`
renders** — the preamble and the six family sections, measured after the `floors:` index line
landed. The plugin's dependency-hook lines are excluded: they are the harness's output, not the
primitive's. Budget = the measured payload exactly, **no +25% headroom**, the third seeding path's
next use, unchanged in that respect from the [v0.100.0]–[v0.103.0] conversion re-seeds. Every row
keeps its prior figure and prior payload split as history, per the standing convention. The two
family common files (`skill-review-common.yaml` 1,627 · `skill-authoring-common.yaml` 1,285) are
**untouched**: they still ship, and they are still budgeted once as their own primitives, but no
converted skill reads them any more — the render resolves every `extends:` stub before the model
sees it, so their bytes have left the converted members' payloads. Descriptions are byte-untouched
by this wave; the only frontmatter change is the added `allowed-tools: Bash(mochiko-cli *)` grant,
which is not a budgeted class. Family aggregates against the record's F3 baselines: review 116,609
vs 119,895 (−2.7 %) · authoring 147,304 vs 150,576 (−2.2 %) · patterns 112,701 vs 95,858
(+17.6 %) · dense five 88,380 vs 81,799 (+8.0 %) — the two overages pre-stated at the wave open as
the eyes-open trade for deterministic delivery. Rulings + disposition maps: the [v0.106.0] entries
in each member's `.mochiko/strips/` file and the 2026-09-04 `cli-schema-delivery` `DECISIONS.md`
row.

**[v0.107.0] the two family common rows are no longer shipped files — figures unchanged.** The
wave-6 end state deleted every schema file from the plugin (`cli-schema-delivery` D9 wave 6,
`DECISIONS.md` 2026-09-05), `skill-review-common.yaml` and `skill-authoring-common.yaml` among
them. They are now **documents in the migration log**, resolved into every render before the model
sees them, so the `.yaml` suffix in their row names is a historical name rather than a path on
disk. This **supersedes the "they still ship" clause** in the [v0.106.0] block above, which was
true when written. Their rows stay exactly as they are — 1,627 and 1,285, no headroom — re-read as
**the budget of the shared block text**, not of a shipped primitive: the text still has a size
worth holding, it is still budgeted once rather than per binding skill, and it is still absent
from every member's payload because the render resolves each `extends:` stub. No figure on this
page changes at wave 6, and no member row is re-seeded: the wave moved no block text.

## Skill descriptions

Hard cap for every skill description, budgeted or not: **1,536 chars** (the delivery-truncation
cap).

| skill | winner chars | budget (+25%) |
|---|---|---|
| analysis-iterative | 476 | 595 |
| analysis-codebase | 349 | 437 |
| authoring-constitution | 481 | 602 |
| authoring-feature-map | 495 | 619 |
| review-governance-intent | 483 | 604 |
| validation-constitution | 481 | 602 |
| testing-governance-injection | 483 | 604 |
| authoring-requirements | 379 | 474 |
| authoring-user-stories | 425 | 532 |
| authoring-prototype | 493 | 617 |
| review-specifications | 490 | 613 |
| ~~authoring-architecture~~ | 488 | 610 |
| authoring-technical-requirements | 496 | 620 |
| brownfield-integration | 491 | 614 |
| executing-tdd-cycle | 498 | 623 |
| grooming-operating-docs | 490 | 613 |
| testing-end-user | 500 | 625 |
| patterns-api-contracts | 486 | 608 |
| patterns-code-minimalism | 564 | 705 |
| patterns-entity-modeling | 497 | 622 |
| patterns-system-design | 649 (re-asserted v0.81.0) | 677 |
| patterns-technical-decisions | 469 | 587 |
| patterns-vertical-tdd | 496 | 620 |
| review-brainstorm | 491 | 614 |
| review-feasibility | 500 | 625 |
| review-plan-artifacts | 500 | 625 |
| review-code-minimalism | 492 | 615 |

`patterns-system-design` (541) and `patterns-code-minimalism` (564) include the v0.64.0
RETURNED clauses (fire-rate probe evidence, user-ruled — see their strips); their budgets
derive from the restored values.

## Agent descriptions

| agent | winner chars | budget (+25%) |
|---|---|---|
| principal-architect | 936 (re-asserted v0.81.0) | 945 |
| validator | 269 | 337 |
| devils-advocate | 316 (re-measured 384 at v0.86.0 — remit-coverage description fix, `.mochiko/strips/devils-advocate.md`; budget unchanged) | 395 |
| requirements-analyst | 303 | 379 |
| product-manager | 438 | 548 |
| product-engineer | 392 | 490 |
| qa-engineer | 299 | 374 |
| staff-engineer | 274 | 343 |
| tech-lead | 888 (re-asserted v0.81.0) | 899 |
| technical-analyst | 402 | 503 |

Wave 2 agent rows (v0.64.0) use the audited measurements (auditor's reproducible canonical-snippet
counts, +1 over the shipper's block-scalar parse — the audited number is authoritative).

`qa-engineer` re-seeded 251→299 at the v0.77.0 audit's drift catch: the v0.75.0 vertical-TDD
build's ruled description rewrite (design-time test-case authoring added) shipped without its
ledger re-measure; 299 is the canonical-snippet count at both the v0.75.0 and v0.77.0 trees,
budget re-derived (+25% → 374). Bookkeeping correction, not a fresh cut.

v0.67.0 rotation (architect-role restructure, audit-measured per the Wave-2 seeding precedent):
`system-architect` row retired with the persona (was 482/603 — strip: `.mochiko/strips/system-architect.md`);
`tech-lead` seeded from its audited description (719); `principal-architect` re-measured at its
rewritten description (756 — the rotation was a ruled edit shipped through the ceremony, audit PASS).
`patterns-plan-minimalism` (new skill, v0.67.0) deliberately unbudgeted — ships hard-cap-only
(desc ≤1,536); audited at body 4,079 / desc 499; a body budget may seed from a future ruled cut
or benchmark, never invented.
`patterns-map-minimalism` (new skill, v0.68.0) likewise unbudgeted — hard-cap-only (desc
≤1,536); audited at body 4,662 / desc 499 (five-guardrail carrier, disclosed size class above
its two siblings — justification ruled HOLDS at audit); a body budget may seed from a future
ruled cut or benchmark, never invented. Standing caution from the same wave's audit, updated at
v0.76.0: `authoring-feature-map` body now measures 15,863 against its 15,413 budget — a
**+450-char total overage** declared and ruled HOLDS at the v0.76.0 V3 audit (the mandated
two-arm schema re-point per schema-based-template-guidance §5 — the feature-entry/features-index
read-pointers swap to the `invoke mochiko-cli template <name>; if absent, Read
plugins/mochiko/schemas/<name>.yaml` form; no restored prose). Overage history: +248 at v0.72.0
(the epic `[EPIC-XXX]` marker grammar + within-epic seam-owner rule, multi-feature-plan-implement
D8/D13) + 202 more at v0.76.0 (the two-arm re-point). Its description 598 against 619 (21 chars,
unchanged). Any further body addition must re-justify its own overage or cut — the budget itself
is unchanged. Same-wave correction at v0.80.0: the slice-vocabulary purge SHRANK the body to
15,846 (overage narrows +450 → +433). **v0.81.0: 15,847 (+434)** — the
product-architecture-schema wave's three architecture-pointer re-keys; the +1 is arithmetically
forced by the `authoring-architecture` → `authoring-architecture-store` slug re-point, **no
prose added or restored** (two of the three re-keyed fragments shrank). Declared and ruled HOLDS
at the v0.81.0 audit; budget unchanged.
`patterns-vertical-tdd` body overage, recorded at v0.80.0 (the slice-vocabulary-purge audit):
measures 6,555 against its 6,487 budget (**+68**). History: 6,457 (inside budget) at v0.75.0;
**+86 silent drift at the v0.76.0 mochiko-cli merge's two-arm schema re-point, never ledgered**
— the same re-point whose `authoring-feature-map` +450 WAS declared and ruled HOLDS; this file's
share was missed and is recorded here retroactively. +12 at v0.80.0, declared and ruled HOLDS
at audit (arithmetically forced by the purge's replacement words: "increments" +8 across two
sites, "BUNDLE-" +4 across four link sites; no prose added or restored; description
simultaneously shrank 512 → 497). Budget unchanged — any further body addition must re-justify
its own overage or cut.
**v0.91.0 (plan-stage retirement): 6,781 against the 6,487 budget — +294 total, of which +226 is
this wave's.** Declared at the edit and **ruled HOLDS by the v0.91.0 wave audit (V2)**. The
growth is D1 mechanic (c), a ruled new obligation with no other home — this skill single-sources
card authorship, so the rule lives here or nowhere: naming the card-authoring home (~+128 — cards
are authored inside the `/mochiko:implement` run, after the design phase or directly on a
zero-gap sufficiency verdict, never a separate plan run) and the card-author-is-never-the-
executing-builder rule (~+82), plus ~+7 across three vocabulary re-keys. No playbook prose was
restored; a rationale clause drafted alongside the mechanic-(c) rule was cut before landing
precisely because it was prose rather than obligation, which is what brought the figure down from
+390. Description unchanged at 497. Budget unchanged — any further body addition must re-justify
its own overage or cut. Ruling + disposition map: `.mochiko/strips/patterns-vertical-tdd.md`
[v0.91.0] and the `DECISIONS.md` 2026-08-26 plan-stage-retirement row.

`authoring-feature-map` body overage widens again at **v0.91.0: 15,975 against the 15,413 budget
(+562)** — **+128 this wave**, on top of the +434 standing at v0.81.0. Declared at the edit for
the wave audit. The growth is D1 mechanic (e)'s **zero-gap branch**, a ruled new obligation: the
map's four-touchpoint line previously read "plan confirms and hardens", and mechanic (e) splits
that duty in two — the design phase asserts the design-implied dependency relations and sharpened
extent when it runs, and **on the zero-gap path, where no design phase runs at all, the
card-authoring seat performs the assertion instead**. Without the second clause the zero-gap path
would leave the map-entry hardening duty unowned, which is the exact defect the record's verify
round 1 caught as V1 and repaired. Two same-line simplifications partly offset it ("plan and
implement's surface" → "implement's surface", −12; "one plan/implement run" → "one implement
run", −9); no prose added or restored. Description unchanged at 598 against its 619 budget.
**Ruled HOLDS by the v0.91.0 wave audit (V2), byte-reconciled to the ruled obligation exactly.**
Budget unchanged — any further body addition must re-justify its own overage or cut.
`patterns-sound-loop` (new skill, v0.70.0) likewise unbudgeted — hard-cap-only (desc ≤1,536);
audited at body 5,729 / desc 500; re-measured body 5,849 at the v0.71.0 neutrality-line
amendment (ruled edit, strip + audit PASS); a body budget may seed from a future ruled cut or
benchmark, never invented.
`patterns-transport-floor` (new skill, v0.71.0) likewise unbudgeted — hard-cap-only (desc
≤1,536); audited at body 5,398 / desc 450; a body budget may seed from a future ruled cut or
benchmark, never invented.
`authoring-epic` (new skill, v0.72.0) likewise unbudgeted — hard-cap-only (desc ≤1,536);
audited at body 7,503 / desc 497 (single-sources a whole delivery object across eight
D-rulings, disclosed above its floor siblings — justification ruled HOLDS at audit); a body
budget may seed from a future ruled cut or benchmark, never invented.
`patterns-adopt-first` (new skill, v0.73.0) likewise unbudgeted — hard-cap-only (desc ≤1,536);
authored to a plan-set build cap of body ≤6,500 / desc ≤500 and measured at birth at body 6,493
/ desc 497 (single-sources three rulings of the build-vs-off-the-shelf discipline — D2's
two-part obligation with its two-sided limb and external-claim binding, D3's constraint-challenge
route-back, D4's retrofit-cost gate — so it sits above its floor siblings and inside the build
cap); the build cap is a plan-time bound, not a budget: a body budget may seed from a future
ruled cut or benchmark, never invented.

`patterns-model-tiering` (new skill, v0.77.0) likewise unbudgeted — hard-cap-only (desc
≤1,536); measured at birth at body 4,160 / desc 643 (the description carries the class key's
both tiers plus the D5 boundary — above the ~500 norm, disclosed for the audit's ruling); a
body budget may seed from a future ruled cut or benchmark, never invented. The `explorer`
agent (new, v0.77.0) description measured at birth at 425; no budget until a ruled cut or
benchmark seeds one. (Row historical: the agent was deleted at the v0.78.0 explorer
retarget — strip `.mochiko/strips/explorer.md`; no live budget obligation remains.)

v0.81.0 rotation (the product-architecture-schema Stage-1 wave — one store, `/mochiko:architecture`
desk, D12 absorb): the `authoring-architecture` rows above are **struck through as historical** —
the skill was retired with the wave (D7; strip `.mochiko/strips/authoring-architecture.md`), so
neither its body 5,250/6,563 nor its description 488/610 carries a live obligation. Kept visible
rather than deleted, in the `system-architect` (v0.67.0) and `explorer` (v0.78.0) precedent.
`patterns-system-design` (transformed — altitude and diagram craft now serving store deltas) and
`principal-architect` (recharted as desk lead / store steward) were **re-asserted at v0.81.0**,
both measured at the transformed text and both **inside budget**: `patterns-system-design` body
9,304 against 11,047 (was 8,837) and description 649 against 677 · `principal-architect`
description 936 against 945 (was 756 — the recharter carries the desk-lead and empirical-drift
duties, 9 chars of headroom left). `tech-lead` description 888 against 899 (was 719 — the
store-write grading duty per D11-as-narrowed). No overage anywhere in the rotation, so no
justification is owed.

`authoring-architecture-store` and `patterns-architecture-shelves` (new skills, v0.81.0) are
**unbudgeted at birth** — hard-cap-only (desc ≤1,536); a body budget may seed from a future ruled
cut or benchmark, never invented. Both grew across the wave's validator-ruled fix rounds, so the
birth figure and the landed figure differ and both are recorded:

| skill | birth | landed (v0.81.0) | growth |
|---|---|---|---|
| authoring-architecture-store | body 8,879 / desc 486 | body 10,810 / desc 492 | +1,931 body |
| patterns-architecture-shelves | body 6,584 / desc 473 | body 6,927 / desc 473 | +343 body |

Every added char is a ruled obligation from a fix round — no restored playbook prose. Store: the
validator-ruled rounds, closing with a +5 NA2 reword. Shelves: the V1-A1 rephrase, the stance
suffix lead-in, and the scope-read line, closing with a +37 NB1 check line. The birth figures are
kept because an unbudgeted primitive's birth measurement is what a future ruled cut or benchmark
would seed a budget *from*; the landed figures are what the next edit is measured against.

How these settled, recorded so a later auditor reads the spread as process rather than drift: each
skill's size was reported more than once while its fix rounds were still running — the store at
8,857 before a +22 hyphenation fix gave the 8,879 birth figure, then at 10,751 before the +5 NA2
reword, then at 10,756 before a final +54 (the V4 no-ruled-content phrasing alignment, applied
wave-wide and reaching this skill last); the shelves at 6,890 before the +37 NB1 check line. **The
landed column is the canonical-snippet count taken at the release-gate sweep** — the authoritative
measurement, which is why it supersedes every relayed figure above — with the descriptions
(492 / 473) stable across every report. This is the standing rule working as designed: the gate
re-measures before the `plugin.json` bump, and it caught one figure that had moved after its seat
reported. The new
`/mochiko:architecture` command carries **no per-primitive budget**, like every other command
(both waves excluded commands by user ruling) — the hard cap and the justified-exemption path are
its only bar.

`testing-gap-finding` (new skill, v0.79.0, QA gap-finding build) likewise unbudgeted —
hard-cap-only (desc ≤1,536); measured at birth at body 10,559 / desc 709. The 709
description is the library's largest (prior high `patterns-model-tiering` 643) — ruled
HOLDS at the v0.79.0 audit (V2/F5): ownership set + scope carve + the six-item fence
inclusion list + the `testing-end-user`/`patterns-vertical-tdd` boundary, all
routing-load-bearing, no playbook prose. Body trail on the audit record: 9,938 at the V1
grade → +219 advisory alignments → +270 F2 fence guard → +132 final alignments + reflow. A
body budget may seed from a future ruled cut or benchmark, never invented. **Re-measured 10,929
at v0.81.0** (+370 — the D12 runtime-NFR re-point to store concern rows plus its spine-exclusion
fence guard); still unbudgeted, still hard-cap-only, description unchanged at 709.

`review-sufficiency` (new skill, v0.91.0, the plan-retirement wave — the sufficiency check's
single source, `plan-stage-utility` D2 as amended by verify V5/V7 and the Addendum A1–A6)
likewise **unbudgeted at birth** — hard-cap-only (desc ≤1,536); measured at birth at body
6,652 / desc 686 — the landed figures, taken after the V3 fix round. Body trail, recorded so a
later auditor reads the spread as process rather than drift: 6,254 at first draft → **+108**
naming `sufficiency-report.md` and its path explicitly (the vocabulary pin, aligning with
`implement.md`) → 6,362 at the V3 grade → **+290** the V3 B1 repair (the fence's flat
`FEAT-XXX/` exclusion contradicted clause 10's obligation to read an in-flight feature's
deltas; scoped to the batch's own run-output directory plus an explicit bounded carve).
Description 681 → **686** across the same round, net of two opposing edits: the "Consumes spec
+ store + baselines only" clause was inaccurate once the fence admitted map entries (A6) and
the clause-10 carve, and was replaced by the operative exclusion alone (−35); the delta-scope
parenthetical then gained the three-clause form, which the prior wording left readable as ten
clauses per card (+40). Both figures sit above the review-family norm and are disclosed rather than
trimmed: the body carries ten clauses each with its own explicit gap form, plus the
delta-scope collapse, the absent-baseline and trips branches, and the report contents — no
rationale prose (the record is the rationale home), no `references/`, no `scripts/`. Size band
for a single-source carrier of this class: `patterns-adopt-first` 6,493 · `authoring-epic`
7,503 · `testing-gap-finding` 10,559. The 686 description carries the entry site, the grading
unit with its delta-scope collapse, the three sources, four SHOULD triggers, the fence's
operative exclusion, and the default-FAIL and independence lines; precedent for a description
above the ~500 norm is `testing-gap-finding`
709 and `patterns-model-tiering` 643. A body budget may seed from a future ruled cut or
benchmark, never invented.

## Unbudgeted primitives

The `mochiko` router skill (body deliberately unbudgeted — its body IS the router index, the
discoverability surface; **38,884 chars at v0.81.0**, grown by the wave's Architecture desk
cluster and its architecture row re-keys; its description is 206 chars) and all **commands** have no
per-primitive budget: no measured winner or ruled cut exists for them (commands excluded from
both waves by user ruling). They are covered only by the standing hard cap (skill
`description:` ≤ 1,536) plus the justified-exemption path. The former M1 near-cap risk
(four description skills within ~30 chars of the cap) was retired at v0.64.0 — all four are
now budgeted rows above, slimmed to ≤500 and probe-verified (14/14 routing hits).

## How to measure

Canonical measurement (python3, from the repo root; identical results required of every grader):

```python
import re

text = open(PATH).read()
fm, body = re.match(r'^(---\n.*?\n---\n)(.*)$', text, re.S).groups()

# skill body: characters after the closing frontmatter fence
body_chars = len(body)

# description (skill or agent): the parsed value — label stripped,
# folded continuation lines joined with single spaces
m = re.search(r'^description:\s*(.*(?:\n  .*)*)', fm, re.M)
desc_chars = len(' '.join(l.strip() for l in m.group(1).splitlines() if l.strip()))
```
