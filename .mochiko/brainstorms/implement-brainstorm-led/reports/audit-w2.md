---
report: review
round: 1
grader: w2-gate-grader
tier: opus
wave: 2
units: [1, 2, 3, 4, 4b, 5, 6, 7, 8]
---

## Shared pre-pass — units 1–4

Every command below was run first-hand by this seat from the repo root, against the working tree
on `engine-desk-build` (HEAD `963a3d6` plus the untracked W2 P1 writes; `git status` identical at
the start and the end of the grade). The baseline is a HEAD copy of `plugins/mochiko/` and
`scripts/similar-rules-allowlist.yaml` (`git archive HEAD`) in the session scratchpad, replayed by
the same binary.

```
$ mochiko-cli --version
mochiko-cli 0.4.0 · grammar 1..2

$ mochiko-cli migrate validate --report --plugin-root plugins/mochiko
pointer resolution: 89 checked against plugins/mochiko
=== similar-rule clusters (threshold 0.60) ===
none — no pair clears the threshold
rules scanned: 1230 · in-kind pairs scored: 211343 · clusters: 0 (none)
allowlist-suppressed edges: 184
mochiko-cli migrate validate · 0 rejecting · 116 advisory

$ mochiko-cli migrate status --plugin-root plugins/mochiko
log plugins/mochiko/migrations · grammar 2 · sequences 1..69 (65 migrations)
state sha256:46145796047479de34aaad447f0e045e0045555e740f72c793f7c56cc7e94279 · 92 documents · 1230 rules

$ mochiko-cli views emit --plugin-root plugins/mochiko --out <scratch>/w2grade/views.foNz
mochiko-cli views emit · 92 documents · <scratch>/w2grade/views.foNz
$ diff -r <scratch>/w2grade/views.foNz .mochiko/schema-views
(no output: the committed views equal the replay)

$ mochiko-cli ids --check --plugin-root plugins/mochiko plugins/mochiko/migrations/0065-…0069-…
mochiko-cli ids --check · 0 bare · 0 drift      (exit 0)
```

Baseline at HEAD (same binary, HEAD allowlist): `sequences 1..62 (60 migrations)` · `state
sha256:a978b6d1…` · `88 documents · 1211 rules` · `0 rejecting · 115 advisory` · `allowlist-suppressed
edges: 184`. The sorted advisory diff, HEAD to now, is exactly: + `condition-coverage ·
command/architecture · visit_kind · value "other" is declared but named by no rule's when:` (the
lead's disclosed Q1 advisory) · `enforces-coverage · command/architecture` 21 → 23 rules (gains
`arch.thinking-stage`, `arch.fail.write-outside-focus`) · `budget · command/architecture` 49 rules ·
11,255 → 64 rules · 16,610 · `budget · skill/authoring-architecture-store` 30 rules · 8,311 → 34
rules · 9,958 (advisory, rule text only). Nothing else moved. Arithmetic holds: +19 rules (0067 four
mints, 0068 fourteen, 0069 one) and +4 documents (two homes, two templates).

Renders, all exit 0, every section end-line count equal to the preamble's section line:
`mochiko-cli rules architecture|authoring-architecture-store|analysis-iterative --section <id>` for
`preamble` and every listed section; `mochiko-cli template architecture-visit|architecture-diagram|
architecture-store`. Pins: architecture `kind: fail · 2 rules` · `class: floor · 25 rules` (HEAD 1 ·
23; the floor index differs by exactly `arch.thinking-stage` and `arch.fail.write-outside-focus`);
authoring-architecture-store `class: floor · 10 rules`, the `floors:` index line byte-identical to
HEAD's.

Homes (`mochiko-cli home --plugin-root plugins/mochiko <path>`): `.mochiko/architecture/desk/2026-10-11-sample`
and `…/2026-10-11-feat-001-sample` both resolve to `architecture-desk` (`visit.md` · template
`architecture-visit` · reports under `report-envelope`); `…/2026-10-11-FEAT-001-sample` resolves to
no home (`architecture/` is NOT a declared sub-directory of `operating-docs-nested`) — the user's Q2;
`.mochiko/product/architecture` lists `sub-directories: concerns, diagrams`;
`.mochiko/product/architecture/diagrams/tenant-provisioning.md` resolves to `product-diagrams` ·
`entries · one ## per entry · 177 lines per entry · not counted: **Lifecycle:** lines`.
Check probes (`mochiko-cli check --path … --content -`): a skeleton-shaped `visit.md` → `allow`; the
same file padded to 317 lines → `deny` "`visit.md` is 317 lines against a whole-file bound of 300"
(exit 4), so `check` enforces the bound the `home` printer does not show (the lead's Q7 quirk,
confirmed); `focus.md` in the visit directory → `deny` "not a declared deliverable"; a
skeleton-shaped diagram file → `allow`.

Char budgets (canonical snippet, `.mochiko/memory/primitive-cost-budgets.md`): no budgeted class is
edited in units 1–4 — no `SKILL.md` body, skill or agent `description:` moved (store `SKILL.md`
body 5,767 · desc 492, unchanged); commands carry no per-primitive budget. Figures noted for unit 5:
the store skill's seven rendered blocks 16,112 → 18,208 characters (+2,096; the HEAD figure equals
the ledger's render 16,112), so the payload stands at 23,975 against the 19,733 budget before P2's
body edit — +2,096 above the standing ruled-HOLDS +2,146, to be argued at unit 5. Architecture's
seven blocks 20,379 → 27,998 characters (unbudgeted).

Plan fidelity: the five files, `hash:` lines aside, differ from the approved plan's §2.1–§2.5 text
(sha256 `250490ec…6caa`, verified) only by stamp re-wrapping and the two disclosed deviations — the
0065 intent's added sentence naming its D3 header anchor (outside the hash) and the `visit_kind`
note's added re-derivation sentence.

## Unit 1 — 0065 and the new architecture-desk home and architecture-visit template views

- **VALIDATE:** schema content · `plugins/mochiko/migrations/0065-desk-visit-home.yaml` with its
  regenerated views `.mochiko/schema-views/homes/architecture-desk.yaml` and
  `.mochiko/schema-views/templates/architecture-visit.yaml` (both new).
- **Checklist run:** the AM-2 five for schema content (`validation-primitive-edit.judgment-items-schema`).
- **Evidence read:** the migration in full; both views in full; the rendered template; the
  `feature-desk` home view (the shape it copies); `architecture-brainstorm-interview` record D3 (S2,
  S5, V3), D4, D5 (S17), the *Defaults batch*; the plan's §2.1.
- **Pre-pass:** shared block above — 0 rejecting; views ≡ replay; `ids --check` 0 bare · 0 drift;
  home and check probes quoted there.
- **Intent stated — PASS.** The intent names what the file imports and why — D3 as changed at S5 and
  V3, the user's `visit.md`, a 300-line whole-file bound, `reports/` under the envelope, the feature
  desk's shape — and the changes do exactly that.
- **Anchor present where required — PASS.** No protected exit (two `import-document` ops); the header
  anchor `2026-10-11 architecture-brainstorm-interview D3` is well-formed and names the one ruling
  executed; a `DECISIONS.md` row (2026-10-11, line 14) carries the session.
- **ID lifecycle right — PASS.** `template/architecture-visit` and `home/architecture-desk` enter once;
  no rule id touched; the skeleton's id placeholders are joined (`FEAT-001-<slug>`, `AX-004-<slug>`,
  `SPN-002-<slug>`) and `ids --check` reports none bare.
- **Floor and fail survival — PASS.** No rule-bearing document is touched; nothing leaves.
- **Register — PASS.** `register: full`, present tense, plain words. The sections carry D3's content
  item for item (goal and done condition · focus set with part, the user's words, the know-sentence
  or `open`, provenance in the engine's vocabulary · defaults batch with default, cost and case
  against (S10) · blind-list differences, with the single-row skip line (D4) · outside claims (the
  default) · rows written, linked (D3), none on a delta visit (D5) · verdict); the header carries the
  FEAT id on a delta visit and the handed-in inputs (default *No size gate*). The `Rows written`
  link `../../../product/architecture/concerns.md` resolves from the visit directory. Grammar 1 is
  right: no grammar-2 home field is used.
- **VERDICT: PASS**
- **Issues requiring fix:** none.

audit: unit 1 (schema content · 0065 + architecture-desk and architecture-visit views) · w2-gate-grader · opus · 3 files · 1 rounds · 0 blocking

## Unit 2 — 0066 and its home and template view diffs

- **VALIDATE:** schema content · `plugins/mochiko/migrations/0066-store-diagram-set.yaml` with the new
  views `homes/product-diagrams.yaml`, `templates/architecture-diagram.yaml` and the view diffs of
  `homes/product-architecture.yaml`, `templates/architecture-store.yaml`.
- **Checklist run:** the AM-2 five for schema content.
- **Evidence read:** the migration in full; the two new views; `git diff` of the two changed views in
  full; the `product` and `product-concerns` home views (the exempt-field and sub-home precedents);
  record D7 (S1), D8 (S3, S4, S6, V2), the *Defaults batch* (*Readers*); the plan's §2.2 and §2.9.
- **Pre-pass:** shared block above.
- **Intent stated — PASS.** It names D7 and D8 as changed at S1, S3, S4 and V2, every document
  imported or replaced, and "Every other template line stands" — the `architecture-store` view diff
  confirms it: the only hunks are the layout table's diagrams row and its two sentences, the index's
  header commit, its fourth part (Diagrams), the sixth health count, and the matching skeleton
  lines. `product-architecture` changes only `subdirs: [concerns] → [concerns, diagrams]`.
- **Anchor present where required — PASS.** No protected exit (imports and two `replace-document`
  ops on a home and a template); the bare session header anchor is well-formed.
- **ID lifecycle right — PASS.** Two documents enter once; two are replaced; no rule id touched;
  the skeleton's ids are joined.
- **Floor and fail survival — PASS.** No rule-bearing document is touched; every pre-existing
  template contract and check line survives (diff read line by line).
- **Register — PASS.** `register: full`, the skeleton constraining shape, never voice. Carriers match
  the rulings: one file per diagram, one `##` entry each at the store's 177-line bound with
  `entry_exempt_fields: [Lifecycle]` exactly as the `product` baselines carry it (S3); one Mermaid
  block, a one-line caption, a `**Draws:**` line citing store targets by joined id and data-model
  entities by file and heading (S1); the store's seven values with one open key at most and no stale
  flag in the file (S3, V2); the index embedding none and linking each with a one-line summary
  (D7), its header naming the last regeneration commit (V2), six health counts (S4). Grammar 2 is
  required and declared (`form: entries`, `entry_exempt_fields`).
- **VERDICT: PASS**
- **Issues requiring fix:** none.

audit: unit 2 (schema content · 0066 + product-diagrams and architecture-diagram views, product-architecture and architecture-store view diffs) · w2-gate-grader · opus · 5 files · 1 rounds · 0 blocking

## Unit 3 — 0067 and the store skill's view diff

- **VALIDATE:** schema content · `plugins/mochiko/migrations/0067-store-diagram-rules.yaml` with the
  view diff of `.mochiko/schema-views/skills/authoring-architecture-store.yaml`.
- **Checklist run:** the AM-2 five for schema content.
- **Evidence read:** the migration in full; the view diff in full; the store skill's render, every
  section; word-level diffs of the five reworded rules (HEAD view against the working view); the log
  history of `sign-off-flips-proposed` (`0029-delta-proposed-until-signoff.yaml`) and the earlier
  re-anchor precedents (`0009-plan-qa-leg.yaml`, `0013-design-baseline-home.yaml`); record D7 (S1 and
  its accepted-risk readability line), D8 (S3, S4, V2), D9 (S9), the *Defaults batch* (*Readers*,
  V8); the plan's §2.3 and §2.8.
- **Pre-pass:** shared block above; floors 10, index byte-identical to HEAD.
- **Intent stated — PASS.** It names the four mints, the two re-anchored rewords with the ruling each
  now carries and the ruling superseded, the three made-false rewords, and "No floor retired or
  lowered" — confirmed by the identical floor index and by no `class`, `kind` or `when` value moving.
- **Anchor present where required — PASS.** The one protected exit is the floor
  `sign-off-flips-proposed`'s clause "on the rendered diagram", reworded (word diff: that phrase
  alone becomes "on the diagram as Mermaid text that parses (rendered beside it where the session
  can)"; the gate's force and every other clause unchanged), in an anchored migration, its rule
  anchor set to `… D9`. On the lead's Q5: D9's changed-at-review S9 names exactly this text — "
  `authoring-architecture-store.sign-off-flips-proposed`'s 'on the rendered diagram'" — as
  superseded, so the ruling covers the reword. Replacing the rule's `… delta-files-vs-direct-baseline-edits
  D7` anchor neither clears nor lowers protection; D7's provenance stays in 0029, and the log's
  precedent (0009, 0013) anchors a reworded rule to the ruling the reword carries.
  `health-view-binding` gains `… D8`, a raise (D8 S4 names "amended from five counts to six"); the
  floor `artifact-home`'s reword is purely additive and keeps its D1 anchor.
- **ID lifecycle right — PASS.** Four ids minted once; five rewords keep their ids; no tombstone,
  none owed; no surviving text cites a dead node.
- **Floor and fail survival — PASS.** Floors 10 → 10, same set; no fail exists in a skill.
- **Register — PASS.** Skill grammar holds (no `kind: fail`, kinds from the eight); one or two
  sentences per mint, plain words. Each mint traces to its ruling — `diagram-set-grammar` (D7, S1, S3;
  two rulings, so the bare session anchor per the wave's convention), `diagram-redraw-timing` (D8,
  V8), `stale-compare` (S4, V2: the index-header commit, the "no drawing behind it" test, unresolved
  citations, one home, a model's read), `index-diagrams-section` (D7); `readability-bar` now reads
  "from the index and the diagrams it points at", D7's own accepted-risk wording.
- **VERDICT: PASS**
- **Issues requiring fix:** none.

audit: unit 3 (schema content · 0067 + authoring-architecture-store view diff) · w2-gate-grader · opus · 2 files · 1 rounds · 0 blocking

## Unit 4 — 0068, 0069 and the architecture view diff

- **VALIDATE:** schema content · `plugins/mochiko/migrations/0068-desk-focus-stage.yaml` and
  `0069-desk-on-engine.yaml` with the view diff of `.mochiko/schema-views/commands/architecture.yaml`.
- **Checklist run:** the AM-2 five for schema content.
- **Evidence read:** both migrations in full; the view diff in full; the architecture render, every
  section; word-level diffs of the six rewords; the `analysis-iterative` render, every section; the
  `common.thinking-stage` block and the `brainstorm`/`spec` stubs; record D1–D5 as review-amended
  (S2, S10–S13, S17–S19, S21) and the *Defaults batch*; `implement-brainstorm-led` D3, *Defaults
  batch* (*Precedence*, *Slots declared where*) and *Supersessions owed at the build*; wave plan §3.4
  and §3.6; the plan's §2.4–§2.5.
- **Pre-pass:** shared block above; pins `kind: fail · 2` · `class: floor · 25`.
- **Intent stated — PASS.** 0068 names the condition, the moment, thirteen domain rules plus the floor
  fail, and six rewords — counted: thirteen mints, one fail, six rewords. 0069 names the one stub,
  the third extender of `common.thinking-stage` (brainstorm and specify carry the same three-field
  stub). "No asking mechanic is restated" holds on read: the desk rules fill the engine's slots
  (`analysis-iterative.caller-slots`) — opening and contract on (`arch.focus-stage`), the unit
  (`arch.focus-unit`), the deliverable (`arch.visit-file`), the blind list's cadence, inputs and job
  (`arch.focus-blind-list`, `arch.focus-context`), the independent leg (`arch.rows-from-visit-file`) —
  and carry no fork, options, streak, stop or provenance rule; no desk-wins precedence clause ships
  (the flipped default).
- **Anchor present where required — PASS.** No protected exit: every reword is additive except
  `arch.seat-architect-producer`'s "delta authoring" (a `must`, unanchored; the intent ties its drop
  to D5, "the desk writing no store delta"); the two floor rewords' anchors are unchanged. Rule
  anchors follow the wave's convention — single-ruling rules carry `… D1`–`… D5`, `… D8`, `… D9`;
  defaults carry the bare session anchor (`gate-focus-confirm`, `first-visit-stage`,
  `outside-claims`, the fail); the stub carries `implement-brainstorm-led D3` under 0069's own header
  (the lead's Q6).
- **ID lifecycle right — PASS.** Fifteen ids minted once; the fail id's `.fail.` segment matches
  `kind: fail`, its `enforces:` naming four live local rules; six rewords keep their ids; no
  tombstone; the prose cite `(arch.focus-context)` resolves.
- **Floor and fail survival — PASS.** Floors 23 → 25 and fails 1 → 2, additions only; the desk gates
  S2 keeps (`arch.user-reserved-rulings`, `arch.truth-user-ruling`, `arch.sound-loop-floor`) and
  `arch.dm-converge-goal`, `arch.dm-close-verdict`, `arch.fail.no-verdict` are byte-identical.
- **Register — PASS.** Second person for the lead, `${producer_seat}`/`${grader_seat}` where a seat
  is named, plain words. The disclosed `visit_kind` re-derivation sentence agrees with S13's own
  first line (the stage fires on any visit that rules an amendment) and with `arch.focus-batch`'s
  drift-finding pull-in (S11), so it closes a gap rather than widening the frame.
- **VERDICT: PASS**
- **Issues requiring fix:** none.

audit: unit 4 (schema content · 0068 + 0069 + architecture view diff) · w2-gate-grader · opus · 3 files · 1 rounds · 0 blocking

## Unit 4b — 0070 and its view diffs

- **VALIDATE:** schema content · `plugins/mochiko/migrations/0070-diagram-write-reach.yaml` with the
  view diffs of `.mochiko/schema-views/commands/architecture.yaml` and
  `.mochiko/schema-views/skills/authoring-architecture-store.yaml`, read against the views as they
  stood after 0069 (this seat's own emit, `<scratch>/w2grade/views.foNz`).
- **Checklist run:** the AM-2 five for schema content.
- **Evidence read:** the migration in full; `diff -r` of the post-0069 emit against the working
  views (three hunks, nothing else); word-level diffs of the three rewords with their class, kind,
  anchor, when and labels; `review-sufficiency.lifecycle-marker-read` and implement's landing
  orphan check (the orphan readers); record D5, D8 (S3); the plan `w2-p1-plan-0070.md` (sha256
  `0075fe63…282c1c`, verified), whose §1 text equals the built file, hash aside.
- **Pre-pass:** run first-hand —

  ```
  $ mochiko-cli migrate validate --report --plugin-root plugins/mochiko
  rules scanned: 1230 · in-kind pairs scored: 211343 · clusters: 0 (none)
  allowlist-suppressed edges: 184
  mochiko-cli migrate validate · 0 rejecting · 116 advisory
  $ mochiko-cli migrate status --plugin-root plugins/mochiko
  log plugins/mochiko/migrations · grammar 2 · sequences 1..70 (66 migrations)
  state sha256:805d25974e6a777ede0be6c46d1bb82ffbdb9442d6e396d413c3dd91c721f3c6 · 92 documents · 1230 rules
  $ mochiko-cli views emit … --out <scratch>/w2grade/views70.mH53 ; diff -r … .mochiko/schema-views
  (no output: the committed views equal the replay)
  $ mochiko-cli ids --check --plugin-root plugins/mochiko plugins/mochiko/migrations/0070-diagram-write-reach.yaml
  mochiko-cli ids --check · 0 bare · 0 drift      (exit 0)
  ```

  The producer's figures match. Against the post-0069 report, only two advisory lines moved:
  `budget · command/architecture` 16,610 → 16,612, and `budget · skill/authoring-architecture-store`
  9,958 → 10,064. Every render exits 0. Pins are unchanged: architecture fail 2 · floor 25, store
  floor 10. The store's `floors:` line is byte-identical to HEAD's. Rendered blocks: store
  18,208 → 18,314 characters (+106, for unit 5), architecture 27,998 → 28,000.
- **Intent stated — PASS.** It names the three rewords, the ruling behind each (D8 · D5 and D8 ·
  the lead's reading of S3, "not a new ruling"), and "no floor retired or lowered". The word diffs
  show exactly those changes:
  - `sound-loop-on-judgment-writes` gains "drawing or redrawing a store diagram,";
  - `author-grader-separation` reads "diagram" where it read "delta";
  - `orphan-rule` adds "and every diagram whose `**Lifecycle:**` carries a key" and "or diagram".
- **Anchor present where required — PASS.** The only protected text that leaves is the word "delta"
  in the floor `arch.author-grader-separation`. It leaves in an anchored migration whose intent ties
  it to D5, "the desk writes no store delta", which D5 and the default *A `kind: fail`* (S18,
  "'delta' dropped") rule. The rule keeps `producer-plan-enforcement D8`, the ruling for the plan-first
  clause that still carries its force. This differs from 0067's re-anchor of the sign-off. The
  plan states the line between them: re-anchor only where the anchor's ruling was replaced. That
  line holds on read. The other two rewords are additions, and their anchors are unchanged.
- **ID lifecycle right — PASS.** Three rewords keep their ids; nothing minted or tombstoned.
- **Floor and fail survival — PASS.** No `class`, `kind`, `when` or anchor moves. The two floors
  gain reach and lose nothing the rulings keep.
- **Register — PASS.** Each rule changes by one list member or subject, with every other word
  unchanged. The orphan reach agrees with S3: each diagram file carries its own `**Lifecycle:**`,
  with one open key. At run-open `review-sufficiency.lifecycle-marker-read` already lists any
  orphaned store-entry marker, diagrams included. The landing verifier's limb waits on implement's
  orphan check naming diagrams, the plan's advisory, routed to wave 3.
- **VERDICT: PASS**
- **Issues requiring fix:** none.
- Closes three carry-forwards above: the unit 5 sound-loop line, the unit 6 "delta" line and the
  unruled orphan thread. Still open: the health view still labels a diagram orphan "orphan
  elements" (plan Q2, routed), and `arch.sound-loop-floor`'s list (plan Q1) stays true through its
  first sentence.

audit: unit 4b (schema content · 0070 + architecture and authoring-architecture-store view diffs) · w2-gate-grader · opus · 3 files · 1 rounds · 0 blocking

## Shared pre-pass — units 5, 6 and 7

Run first-hand from the repo root, on the working tree with P2's prose writes in it. P3 had
written nothing yet: no `evals/**` or `crates/**` path was dirty.

```
$ mochiko-cli migrate validate --report --plugin-root plugins/mochiko
rules scanned: 1230 · in-kind pairs scored: 211343 · clusters: 0 (none)
allowlist-suppressed edges: 184
mochiko-cli migrate validate · 0 rejecting · 116 advisory      (the sorted report is identical to unit 4b's)
$ mochiko-cli migrate status --plugin-root plugins/mochiko
log plugins/mochiko/migrations · grammar 2 · sequences 1..70 (66 migrations)
state sha256:805d25974e6a777ede0be6c46d1bb82ffbdb9442d6e396d413c3dd91c721f3c6 · 92 documents · 1230 rules
```

Char budgets, with the ledger's canonical snippet (characters of the parsed value), HEAD →
working tree:
- `authoring-architecture-store`: body 5,767 → 5,878, description 492 → 492.
- `patterns-system-design`: body 9,991 → 10,073, description 625 → 625.
- the router `mochiko`: body 48,037 → 48,637, description 206.

Renders, summed over the seven blocks:
- `authoring-architecture-store`: 16,112 at HEAD, 18,208 after 0069, 18,314 now (preamble 2,452 ·
  independence 919 · scope 1,317 · inputs 764 · artifact 8,166 · output 2,357 · reserved 2,339).
- `architecture`: 20,379 at HEAD, 28,000 now.

Per-rule render deltas came from stepped replays (a HEAD copy, a log copy without 0070, the tree).
`0067` totals +2,096 and `0070` totals +106, matching the ledger exactly. Four per-rule figures
differ from the ledger's by one character, where a section's trailing newline is attributed
(`artifact-home` 124 vs 123 · `diagram-redraw-timing` 290 vs 291 · `index-diagrams-section` 228
vs 229 · `readability-bar` 25 vs 24). The totals do not move.

## Unit 5 — the authoring-architecture-store skill pair

- **VALIDATE:** skill pair · `plugins/mochiko/skills/authoring-architecture-store/SKILL.md` with
  `mochiko-cli rules authoring-architecture-store --section <id>` for the preamble and all six
  sections, and the budget row `.mochiko/memory/primitive-cost-budgets.md:82`.
- **Checklist run:** the skill-pair judgment items (`validation-primitive-edit.judgment-items-pair`),
  against `.claude/rules/mochiko/primitive-edits.md`'s skill-pair criteria.
- **Evidence read:** `SKILL.md` in full and its `git diff`; all seven rendered blocks; the budget
  row (its HEAD text and its working text); record D7, D8 (S3, S4, V2), D9 (S9); units 3 and 4b
  above.
- **Pre-pass:** shared block above — 0 rejecting; pointer resolution 89; skill grammar holds.
- **Scaffold headings and order — PASS.** The frontmatter carries `name`, the `description` and
  `allowed-tools: Bash(mochiko-cli *)`. Then come the title, `## Overview`, `## Rules — delivered
  by mochiko-cli` with its delivery and no-raw-Read clause, and seven `!` lines: the preamble, then
  the six authoring-family sections in the preamble's order. The read-back cites the `class: floor`
  pin and the `floors:` line with no number hard-coded. None of this moved this wave.
- **Preserved responsibilities — PASS.** The body's one change adds the
  `diagrams/<slug>.md` row to the store-layout table. Nothing leaves, so no strip is owed. The
  `description:` is byte-identical (492 characters, under the 1,536 cap). No body line is made
  false: the lifecycle diagram, the landing-duty table and the drift paragraph still read true.
- **Floor survival — PASS.** There are 10 floors and the `floors:` line is byte-identical to
  HEAD's. The one floor reword that drops text, `sign-off-flips-proposed`, is covered by D9's S9
  (unit 3). `artifact-home` and `sound-loop-on-judgment-writes` only gain text (units 3 and 4b).
- **Independence, no seat grading its own row — PASS.** `sound-loop-on-judgment-writes` now names
  drawing or redrawing a store diagram, so a diagram write takes the review leg like any other
  judgment write.
- **Reserved-to-user content in the reserved section — PASS.** `sec.reserved` holds
  `sign-off-flips-proposed`, `diff-user-dispositioned`, `drift-user-disposition` and
  `bootstrap-confirmation-gate`.
- **Done-condition branch — n/a.** This item applies to commands only.
- **Argued overage — PASS: the argument HOLDS.** The overage was named in the brief, and every
  figure re-measures as stated: payload 24,192 = body 5,878 + render 18,314, which is +4,459 over
  19,733. This wave's +2,313 has three parts:
  - render +2,096 from `0067`: four mints carrying D7 and D8 (the diagram set and its grammar,
    the stale compare, redraw timing, the index's diagrams section) and five rewords D7, D8 and
    D9 make necessary;
  - render +106 from `0070`: D8's diagram drawing as a judgment write, plus S3's own Lifecycle
    field brought under the orphan rule;
  - body +111: one table row that makes the layout table true.

  Each part is a ruled obligation and none is restored playbook prose. The row carries the
  prior entries word for word (checked), and only rows `:73` and `:82` changed.
- **VERDICT: PASS**
- **Issues requiring fix:** none.
- Closes unit 1–4's note on this payload.

audit: unit 5 (skill pair · authoring-architecture-store) · w2-gate-grader · opus · 3 files · 1 rounds · 0 blocking

## Unit 6 — the architecture command pair

- **VALIDATE:** command pair · `plugins/mochiko/commands/architecture.md` with `mochiko-cli rules
  architecture --section <id>`, the preamble and all six sections.
- **Checklist run:** the command-pair judgment items, against the canonical-scaffold criteria
  (desk done-condition branch). Two extra checks: change control (the engine's render against the
  desk's) and the delivery check for `arch.thinking-stage`.
- **Evidence read:**
  - `architecture.md` in full and its `git diff`;
  - all seven `architecture` blocks;
  - all seven `analysis-iterative` blocks, and its `SKILL.md` `!` lines;
  - `plugins/mochiko/hooks/scripts/dependency-halt.sh`;
  - record D1–D5 and the *Defaults batch*; `implement-brainstorm-led` *Defaults batch*.
- **Pre-pass:** shared block above. Pins: `kind: fail · 2 rules` and `class: floor · 25 rules`.
  Every section's end-line count equals the preamble's section line (16 · 2 · 19 · 13 · 12 · 2).
- **Scaffold headings and order — PASS.**
  - Frontmatter: `description`, `argument-hint`, `disable-model-invocation: true`,
    `allowed-tools: Bash(mochiko-cli *)`.
  - Headings: `# Architecture — The Product Architecture Desk`, `## Identity & Mission`,
    `## Rules — delivered by mochiko-cli` with seven `!` lines matching the six-section set, then
    `## Adaptive Goal Protocol` with Entry, Goal and Not done last.
  - Entry takes `$ARGUMENTS` (a demand, a store query, a brainstorm record, or a `<FEAT-ID>`) and
    the empty case. Not done cites the `kind: fail` pin and carries the halt clause, with no
    hard-coded count.
  - The one `arch.sec.*` token in the prose resolves.
- **Preserved responsibilities — PASS.**
  - The edit is additive: the `argument-hint` gains `brainstorm record | <FEAT-ID>` (plan §3.5),
    and Entry gains those two inputs, diagram orphans (`0070`) and the sixth health count (`0067`).
    Nothing leaves.
  - No `.md` line is made false. "A crisp ask … converges in a single exchange" still holds,
    because a crisp ask is a focus set of one, handed in.
- **Floor survival — PASS.** Floors 23 → 25 and fails 1 → 2, additions only (unit 4). The
  sound-loop pointer floor `arch.sound-loop-floor` stands on this DM-chartered command, and the
  `dm-*` floors are unchanged.
- **Independence, no seat grading its own row — PASS.** `arch.author-grader-separation` (now
  naming diagrams), `arch.seat-tech-lead-grader` (a drawing graded against its `Draws:` targets)
  and the sameness grade in `arch.rows-from-visit-file`.
- **Reserved-to-user content in the reserved section — PASS.** `arch.user-reserved-rulings` and
  the new `arch.gate-focus-confirm`.
- **Done-condition branch, desk per-visit contract — PASS.** Goal converges each visit to a
  one-line goal and its done condition and closes with a verdict. `arch.focus-stage` puts the
  set's close inside that condition, `arch.fail.no-verdict` holds, and the visit template ends in
  a Verdict. No fixed done condition is demanded.
- **Argued overage — n/a.** Commands are unbudgeted (render 28,000 characters).
- **Change control — PASS. No desk rule loosens an engine mechanic.** The desk fills each slot
  in `analysis-iterative.caller-slots`:
  - opening and contract `on`: `arch.focus-stage`;
  - the unit: `arch.focus-unit`;
  - the deliverable and where the set is saved: `arch.visit-file`;
  - the blind list's cadence, inputs and job: `arch.focus-blind-list` and `arch.focus-context`;
  - the independent leg: `arch.rows-from-visit-file`.

  The gate and the fail stay command grammar, as the slot rule says. Where the desk adds to an
  engine rule, it only tightens:
  - `arch.outside-claims` adds "live source, never memory" to `fact-before-fork`;
  - `arch.fail.write-outside-focus` grants no small-extra carve-out under `contract-inside`;
  - `arch.diagram-signoff` waits on the user's yes before any publish.

  The desk carries no fork, options, streak, stall, stop or provenance rule, and no own-rules-win
  clause. On the note from units 1–4: the slot rule lets a caller replace the blind list's inputs.
  The desk's input list is closed ("you read — and the blind seat is given — …"), so the seat
  never gets the lead's options, the conversation or the ruled set. The fence holds.
- **Delivery — PASS. `arch.thinking-stage` reaches the engine's seven blocks.**
  - It renders `pointer: mochiko:analysis-iterative`, and that skill's `SKILL.md` carries
    `name: analysis-iterative`.
  - The seven `!` lines of `analysis-iterative/SKILL.md` were run verbatim with
    `CLAUDE_PLUGIN_ROOT` set to the tree's plugin. Each exits 0, and the end lines read 0 · 2 · 2 ·
    2 · 9 · 3 · 3 with 21 `###` headings.
  - `architecture.md`'s own seven lines exit 0, with end lines 0 · 16 · 2 · 19 · 13 · 12 · 2.
  - The hook, given simulated payloads, returns "mochiko-cli present · rules delivered by the
    skill's own render" for `PreToolUse`/`Skill` on `mochiko:analysis-iterative`, and "… by the
    command's own render" for `UserPromptExpansion` on `mochiko:architecture`. With the binary off
    `PATH` it returns `deny` ("mochiko-cli is not installed").
- **VERDICT: PASS**
- **Issues requiring fix:** none.
- Still open, a nit: the `focus` moment's text names `principal-architect` literally.

audit: unit 6 (command pair · architecture, with change control and delivery) · w2-gate-grader · opus · 4 files · 1 rounds · 0 blocking

## Unit 7 — patterns-system-design, router, strips, ripple, ledger, changelog

- **VALIDATE:** prose primitives and records: `plugins/mochiko/skills/patterns-system-design/SKILL.md` and
  `references/DIAGRAM-CONVENTIONS.md` · the router `plugins/mochiko/skills/mochiko/SKILL.md` ·
  `.mochiko/strips/patterns-system-design.md` · `.mochiko/strips/mochiko.md` · `README.md` ·
  `ARCHITECTURE.md` · `.mochiko/memory/primitive-cost-budgets.md:73`, `:82` · `CHANGELOG.md` (the
  W2 part).
- **Checklist run:** the prose items (`validation-primitive-edit.judgment-items-prose`):
  coherence and preserved responsibilities. The strips are read against the strip README's
  supersession-entry form.
- **Evidence read:**
  - the `git diff` of every file above, and `patterns-system-design/SKILL.md` lines 1–70 and its
    checklist;
  - every `render`, `sign-off` and `raw code` line in both system-design files;
  - the cross-references *Scale bound in practice* and `BACKLOG.md` *Security-depth build*;
  - the HEAD text of each superseded line, checked against the strip quote;
  - the repo's own `.mochiko/product/architecture/`: a scaffold, so `ARCHITECTURE.md` is still
    hand-kept and editable (`.claude/rules/mochiko/operating-docs.md`);
  - `DECISIONS.md:179`; record D9 (S9), the *Defaults batch* (*Taste*, S23) and OQ3.
- **Pre-pass:** the shared block above, budgets included.
- **Coherence — PASS.**
  - **Sign-off lines.** The three rewritten `SKILL.md` lines (`:54`, `:68`, the checklist item)
    and the *Rendering* paragraph all state D9's surface: Mermaid text that parses, rendered
    wherever it is viewed, the picture beside it where the session can. No line still demands a
    render at sign-off.
  - **Taste section.** It carries the default *Taste* item for item, and its cross-references
    resolve.
  - **The init directive.** The line claims nothing about GitHub: it "holds only where the
    renderer honors it", which leaves OQ3 to the user's push-time check.
  - **Router.** The store row gains the diagram set and the stale count. The system-design row
    takes D9's wording. The desk row describes the focus stage, the visit file and the no-stance
    defaults batch (S11, S13). The `tech-lead` row matches `0070`.
  - **Ripple.** `README.md` and `ARCHITECTURE.md` lose their stale lines ("delta authoring", the
    five-item health list).
  - **Figures.** The ledger rows and `CHANGELOG.md` re-measure as stated, including the six
    `[v0.120.0]` strip entries.
- **Preserved responsibilities — PASS.**
  - Every removal has a `[v0.120.0]` supersession-by-ruling entry, newest first: four in
    `patterns-system-design.md` (`:54`, `:67–68`, `:128`, the *Rendering* line) and two in
    `mochiko.md` (the desk row's two passages, the system-design row's clause).
  - Each entry quotes the superseded text word for word against HEAD and cites D9 (or D1–D4)
    with its `DECISIONS.md` row. Each says what it keeps — "This diagram is what the reader
    approves", "valid mermaid" kept as "parses", "the signed state where no commit is made" — and
    which consumers it assessed.
  - The additions are recorded in each file's wave-context comment. No `KEPT:` line in either
    strip file is touched.
  - Budgets: `patterns-system-design` body 10,073 is inside its 11,047 budget, its description 625
    unchanged; `references/` is exempt and the router is unbudgeted.
  - The `ARTIFACT-CHECKLISTS.md:80`/`:89` wording and the `DECISIONS.md:179` annotation for
    `architecture-design-primitive` D8-rendered-diagram-signoff are owed later: the first in wave
    3, the second at the landing, as the strips say.
- **VERDICT: PASS**
- **Issues requiring fix:** none.
- Advisory, non-blocking:
  - `CHANGELOG.md:125`: "20,379 → 28,000 characters per block" reads as a per-block figure. It is
    the sum over the seven blocks.
  - `patterns-system-design/SKILL.md`'s overview and description scope the skill to the delta, but
    its taste section now governs every diagram, the store's diagram set included. A diagram-set
    drawer reaches it through `arch.tools-system-design` and the template, not through this skill.
  - The router's desk row does not name the `<FEAT-ID>` delta visit, which writes no rows.

audit: unit 7 (prose primitives and records · patterns-system-design + DIAGRAM-CONVENTIONS, router, two strips, README, ARCHITECTURE, budget ledger, CHANGELOG) · w2-gate-grader · opus · 9 files · 1 rounds · 0 blocking

## Unit 8 — crate tests, contract expectations, the architecture plan kit

- **VALIDATE:** gate files and tests, sixteen paths:
  - `crates/mochiko-cli/tests/{fidelity,validate,views,matrix_similar,render}.rs`;
  - `tests/fixtures/template/architecture-store.{producer,check}.txt` (modified), and
    `architecture-visit.*` and `architecture-diagram.*` (new);
  - `evals/contract/run.py` and `evals/contract/README.md`;
  - `evals/plan/README.md`, `evals/plan/architecture/observable.yaml` and `preregistration.md`.

  This unit is also the crate's independent non-author review (`.claude/rules/mochiko/rust-cli.md`).
- **Checklist run:**
  - every pinned figure is read from a tool;
  - `EXPECTED["architecture"]` matches the render's floors;
  - the store skill's frozen floor set holds;
  - every live architecture rule sits in exactly one `observable.yaml` list, each mint with its why;
  - the preregistration recount and its amendment line are right;
  - the two disclosed deviations are acceptable.
- **Evidence read:** the `git diff` of each modified file and each new fixture in full; the
  fixture comparison test (`render.rs:1236–1258`); the g1–g3 fixtures and `evals.json` arguments,
  spot-read where a `why` names them; build log lines 660–672 and 772–790.
- **Pre-pass:** run first-hand, quoting the shortest decisive line of each:

  ```
  $ git diff --stat -- crates/mochiko-cli/src crates/mochiko-cli/Cargo.toml Cargo.toml Cargo.lock scripts/similar-rules-allowlist.yaml
  (empty — no crate source, manifest, lock or allowlist change)
  $ mochiko-cli migrate validate --report --plugin-root plugins/mochiko
  mochiko-cli migrate validate · 0 rejecting · 116 advisory
  $ cargo test --all
  20 `test result: ok` lines · passed 765 · failed 0 · ignored 0
  $ MOCHIKO_FULL_SIMILAR=1 cargo test --test matrix_similar
  test result: ok. 48 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 235.46s
  $ cargo fmt --all --check ; cargo clippy --all-targets -- -D warnings
  (fmt exit 0) · Finished `dev` profile … (clippy exit 0)
  $ uv run evals/contract/run.py --list ; uv run evals/contract/run.py --host-only
  FILTERED — 7 of 99 declared cases. Not a gate run.
  contract suite: 7/7 cases passed, 7 ran, 5 measurement(s) recorded and not asserted
  $ uv run evals/run.py command check-rubric architecture ; … check-fixtures architecture
  rubric OK: 58 observable, 6 out-of-instrument, 64 total (exit 0) · fixture consistency: OK (exit 0)
  ```
- **Fixtures — PASS.** For all six (`architecture-store`, `-visit` and `-diagram`, producer and
  `--check`), `mochiko-cli template <name> [--check] --plugin-root plugins/mochiko` with its final
  `schemas: replayed from …` line removed — the form the fixture test reads — is byte-identical
  to the file (`cmp`). `SHIPPED_TEMPLATES` (14) equals the log's template set, and so does
  `run.py`'s `TEMPLATE_NAMES`: the `deliverables` host case reports "every template in the log is
  one this case walks".
- **Figures come from tools — PASS.**
  - **Corpus counts.** 92 documents and 1,230 rules agree with `migrate status`; 390 command and
    840 skill rules, 124 command floors, 38 fail nodes and 266 skill floors are asserted by passing
    tests.
  - **Similarity pins.** 1,230 rules and 211,343 pairs equal `migrate validate`'s stats line.
    390 rules and 18,471 pairs are what the opt-in sweep asserts and passes.
  - **By-kind breakdowns.** Each one recomputes from n(n−1)/2: command +1,003 · +162 · +138 ·
    +37 · +11 = 1,351; corpus +4,058 · +865 · +775 · +37 · +29 = 5,764. The fifteen command
    mints split into 6 constraint, 3 binding, 4 duty, 1 fail and 1 gate, matching the units 4
    and 4b reads.
  - **Raw cluster listings.** The claim that 1..62 and 1..70 give identical raw listings was
    re-run with an empty allowlist. Both report 77 clusters, and the 551-line listings are
    identical.
  - **Prose counts.** The comments' counts agree with the migrations: fourteen rewords, two
    anchor `set-rule-field`s, `0063`–`0064` a legal gap.
- **`EXPECTED["architecture"]` — PASS.** The set gains `arch.fail.write-outside-focus` and
  `arch.thinking-stage`, each with its migration and anchor, and its 25 ids equal the render's
  `floors:` line exactly. `evals/contract/README.md` reads 124 floor ids across six and names
  both mints. `expected-skills.json` is untouched, and `authoring-architecture-store`'s frozen
  `floor_ids` (pin 10) equal the render's `floors:` line.
- **Plan kit — PASS.**
  - **Partition.** `check-rubric` asserts the 58 · 6 partition of the live 64. The two lists
    share no id, and their union equals the live set `partition` reports.
  - **The fifteen mints.** Each carries a `why`: twelve observable, two declared conditional
    (`arch.feature-delta-visit`, `arch.desk-redraw`, each with the plant that would make it
    gradable), and the fail beside `arch.fail.no-verdict`.
  - **Spot-read whys.** The ones that name fixtures hold: g2's four `open` rows,
    g1's Twilio p99 and A2P constraint, g1's empty argument as a first visit.
  - **The `visit_kind` header.** It reads ruling for g1 and g2 and other for g3.
  - **Recount.** `uv run evals/run.py command partition architecture --old-ref 794cea8`,
    crossed with `observable.yaml`, gives exactly unchanged 38 (35) · changed 9 (9) · removed 0 ·
    added 17 (14). The intermediate 41 (38) · 6 (6) · 0 · 2 (2) at `963a3d6` follows from the
    changed and added lists.
  - **Arithmetic.** 3 × 58 = 174 pairs per arm, and the regression denominator is 35.
  - **Amendment line.** It cites the lead's ruling (build log line 666, option A): the
    authoring-time shape kept as history, `--old-ref 794cea8`, and the `arch.artifact-home`
    control graded on its current text. Since no `runs/` exists, the re-key count stays 0.
  - **`PROBE_ARGUMENTS["architecture"]`** still reads true: its Entry quote is the head of the new
    Entry line.
- **Deviation 1 (fixtures written with Write and Edit) — ACCEPTABLE.** How the files were written
  does not matter here, because each one equals the binary's own output byte for byte.
- **Deviation 2 (`post` names `0070-diagram-write-reach.yaml`) — ACCEPTABLE, and the plan's
  `0069` would have been wrong.** The working log ends at 70, and `0070` rewords the architecture
  rule `arch.author-grader-separation`, so a `post` arm ending at `0069` would misstate its own
  pair. This follows §5.4's substitution principle.
- **Crate review (non-author) — PASS.** The change is tests only: constants, one manifest entry
  pair, four new fixtures. No assertion is loosened. Each moved pin carries a comment naming the
  migrations that move it. No crate source, manifest or lock changed, so `cargo audit`'s input is
  unchanged. The sandbox cases of the contract suite did not run: they are the bump's gate, where
  a SKIPPED suite is not green.
- **VERDICT: PASS**
- **Issues requiring fix:** none.
- Advisory, for the first grid: g3's argument opens "FEAT-006 shipped last week — flip AX-008 to
  built …", and the new `argument-hint` reads a `<FEAT-ID>` as a delta visit. The kit expects
  `visit_kind` other on g3, which is right under the converged goal. Watch the first grid for the
  stage firing there.

audit: unit 8 (crate tests, contract expectations, the architecture plan kit · crate non-author review) · w2-gate-grader · opus · 16 files · 1 rounds · 0 blocking

## Notes of note

- Unit 6 (change control): `arch.focus-blind-list` states one exclusion, "never the visit file", and
  leans on `arch.focus-context`'s closed input list for the rest of D4's fence (never the focus set
  or the conversation). Re-read it against the engine's default fence.
- Unit 6 (coherence): the floor `arch.author-grader-separation` still lists "delta" after D5 and
  names no diagram. It stays true as written; the `focus` moment names `principal-architect`
  literally.
- Unit 5: the payload (23,975 before P2) needs its +2,096 render growth argued under D7/D8/D9;
  `sound-loop-on-judgment-writes` names no diagram write.
- Wave 3: the D8 landing flip of a diagram (delta styling, `**Lifecycle:**`) has no store-skill
  carrier (`fold-duty` names elements only); the D9 degrade note at implement's checkpoint.
- Unruled thread: `orphan-rule` keys elements only, so nothing flags a diagram left open under a
  closed run's key.
