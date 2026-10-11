---
report: review
round: 2
grader: w3-gate-grader
tier: opus
wave: 3
units: [1, 2, 3, 4a, 4b, 4c, 4d, 5, 6a, 6b, 6c, 6d, 7, 7b, 8]
---

## Shared pre-pass — units 1–4

Every command below was run first-hand by this seat from the repo root, against the working tree
on `engine-desk-build` (HEAD `7d9d29e` plus the untracked W3 P1 writes 0075–0083 and their
regenerated views). The baseline is a HEAD copy of `plugins/mochiko/migrations/` (`git archive
HEAD`) in the session scratchpad with the HEAD allowlist beside it (the allowlist is unchanged in
the tree), replayed by the same binary with `--log-dir` and `--plugin-root plugins/mochiko`.

```
$ mochiko-cli --version
mochiko-cli 0.4.0 · grammar 1..2

$ mochiko-cli migrate validate --report --plugin-root plugins/mochiko
pointer resolution: 89 checked against plugins/mochiko
=== similar-rule clusters (threshold 0.60) ===
none — no pair clears the threshold
rules scanned: 1243 · in-kind pairs scored: 213340 · clusters: 0 (none)
allowlist-suppressed edges: 184
mochiko-cli migrate validate · 0 rejecting · 116 advisory        (exit 0)

$ mochiko-cli migrate status --plugin-root plugins/mochiko
log plugins/mochiko/migrations · grammar 2 · sequences 1..83 (75 migrations)
state sha256:68e3074fd6b70a95fdfa6cd7958acb9ae331f9799c82b3a9ef3e72f28a9e4ce7 · 93 documents · 1243 rules

$ mochiko-cli views emit --plugin-root plugins/mochiko --out <scratch>/w3gate/views
$ diff -r <scratch>/w3gate/views .mochiko/schema-views
(no output: the committed views equal the replay)

$ mochiko-cli migrate stamp <scratch copy of each of 0075–0083, its hash: line removed>
all nine re-stamped hashes equal the committed hash: lines — 0075 238d849d… · 0076 8f88f832… ·
0077 f0bdf701… · 0078 d78415d8… · 0079 49954382… · 0080 cf6ed47a… · 0081 3fd38bcc… ·
0082 8f59e963… · 0083 6c1ca199…

$ mochiko-cli ids --check plugins/mochiko/migrations/0075-… … 0083-…   (all nine files)
mochiko-cli ids --check · 0 bare · 0 drift        (exit 0)
```

Baseline at HEAD: `sequences 1..70 (66 migrations)` · `state sha256:805d2597…` · `92 documents ·
1230 rules` · `0 rejecting · 116 advisory` · 211343 pairs · 184 suppressed. The sorted advisory
diff, HEAD to now, is exactly: `budget` lines for implement (114 rules · 34,627 → 125 · 42,178),
authoring-architecture-store (34 · 10,064 → 34 · 10,148), review-plan-artifacts (37 · 6,747 → 38 ·
7,583), review-sufficiency (28 · 7,099 → 29 · 7,745), testing-gap-finding (36 · 7,996 → 36 · 8,095)
· + `condition-coverage · skill/review-plan-artifacts · design_stage · value "absent"` ·
`enforces-coverage · command/implement` 31 → 33 (gains `impl.thinking-stage` and
`impl.fail.design-outside-ruled-set`) · − `unused-moment · command/implement · design-checkpoint`
(now named in `impl.index-regeneration`'s text). 116 stays 116 (+1, −1); nothing else moved.

Field diff (HEAD view against the regenerated view, parsed, over the five changed rule-bearing
documents): no rule vanished or moved; no `class`, `kind`, `when`, `pointer`, `extends`,
`enforces` or `labels` changed on an existing rule; no tombstone added. Anchors: one replaced
(`impl.gate-design-checkpoint`, `2026-08-26 plan-stage-utility` → `2026-10-11
implement-brainstorm-led D8`); five added where none stood (`impl.design-inputs` arch D5;
`impl.design-outputs-home`, `impl.deviation-gate`, `impl.store-landing` arch D8;
`impl.design-review-pair` implement D8). Implement pins: `kind: fail · 16 rules` · `class: floor ·
39 rules` (HEAD 15 · 37; the floor index gains exactly `impl.thinking-stage` and
`impl.fail.design-outside-ruled-set`). Skill floor counts unchanged: store 10 · review-plan-artifacts
11 · review-sufficiency 8 · testing-gap-finding 9.

Char budgets and sizes: no budgeted class — no `SKILL.md` body, no skill or agent `description:` —
is edited in units 1–4; those land at units 6a–6d. Measured for those units, characters of the
seven rendered blocks, HEAD → now: store 18,307 → 18,391 · review-plan-artifacts 14,902 → 16,038 ·
review-sufficiency 13,352 → 14,091 · testing-gap-finding 16,196 → 16,293. Implement
`impl.sec.tools` 25,955 → 27,396 characters, 27,651 bytes by `wc -c` (27,650 in the lead's
no-trailing-newline convention), under the 29,000 B ceiling. Other implement blocks (bytes):
preamble 5,016 · roles 5,322 · reserved 5,597 · ways-of-working 6,342 · boundaries 5,991 ·
fail-conditions 5,162.

Homes and check probes: `mochiko-cli home` on `.mochiko/features/FEAT-001-demo`,
`.mochiko/epics/EPIC-001-demo` and `.mochiko/product/lane-foo` each lists `design-stage.md ·
template design-stage`. `mochiko-cli check --path <home>/design-stage.md --content -`: a 3-line
draft exits 0 at all three; a 301-line draft exits 4 at all three with "301 lines against a
whole-file bound of 300". `mochiko-cli template design-stage` renders 4,279 B; `--check` renders
849 B with six items, one per section.

Plan fidelity: each file's `changes`, `anchor` and whitespace-normalized `intent` equal the
approved drafts (`w3p1/drafts/`, and `w3p1/drafts-rev1/` for 0078 and 0083), compared parsed; the
files differ textually only by the stamp's re-wrapping of `intent`. Plan files re-hashed: base
`9093c035…`, rev1 `d8aab3f9…`, as approved.

## Unit 1 — 0075 and the design-stage template and home views

- **VALIDATE:** schema content · `plugins/mochiko/migrations/0075-design-stage-home.yaml` with the
  new view `.mochiko/schema-views/templates/design-stage.yaml` and the view diffs of
  `homes/feature.yaml`, `homes/epic.yaml`, `homes/product-lane.yaml`.
- **Checklist run:** the AM-2 five for schema content (`validation-primitive-edit.judgment-items-schema`).
- **Evidence read:** the migration in full; the new view in full; the three home view diffs; the
  rendered template and its `--check`; the `architecture-visit` template view (the shape it
  copies) and `0065-desk-visit-home.yaml` (the bound posture); implement-brainstorm-led record D5
  (S9), D6 (S1), D7 (S8), the *Defaults batch* (*the set's home*, *the blind list*, *handed-in
  sets*, *One provenance vocabulary*); the build log's 2026-10-11 lane ruling; plan §2.1, rev1 R3.
- **Pre-pass:** shared block above — 0 rejecting; views ≡ replay; stamp and `ids --check` clean;
  home and check probes quoted there.
- **Intent stated — PASS.** It names the default, the vocabulary and the user's lane ruling, the
  item shape and the 300-line whole-file bound; "Nothing else in the three homes moves" holds —
  each home's view diff is one three-line deliverable entry.
- **Anchor present where required — PASS.** No protected content is touched (one `import-document`,
  three `replace-document` on homes); the header anchor `2026-10-11 implement-brainstorm-led` is
  well-formed; `DECISIONS.md` line 13 carries the session.
- **ID lifecycle right — PASS.** `template/design-stage` enters once; each home is replaced with its
  prior content verbatim plus one deliverable; the skeleton's ids are joined (`FEAT-001-<slug>`,
  `US-003-<slug>`, `C3-<slug>` in the tasks template's cycle form); `ids --check` reports none bare.
- **Floor and fail survival — PASS.** No rule-bearing document is touched; nothing leaves.
- **Register — PASS.** `register: full`, present tense, plain words. The sections carry the record
  item for item: Header (handed-in inputs by path, stage state for resume — *the set's home*) ·
  Ruled set (one entry per gap, per epic spine item — D5 S9, per user-added gap — D6 S1; the
  user's direction, the seed or `open` never filled in, each piece marked, the entry's own mark —
  D7 and the one vocabulary) · Defaults batch (default and cost — D7) · Blind list (the single-gap
  re-fire skip — the default) · Outside needs (split by kind, `/mochiko:specify` · `BACKLOG.md` ·
  a feature-desk card, none on a lane run — D6 S1) · Open (a stopped stage resumes from it). The
  bound — `template:` with no conformance block plus `max_lines: 300` — is the visit file's
  posture, and the probes show the gate enforcing it at all three homes.
- **VERDICT: PASS**
- **Issues requiring fix:** none.

audit: unit 1 (schema content · 0075 + design-stage template view, feature/epic/product-lane home view diffs) · w3-gate-grader · opus · 5 files · 1 rounds · 0 blocking

## Unit 2 — 0076, 0077 and the implement view diff

- **VALIDATE:** schema content · `0076-implement-design-stage.yaml` and
  `0077-implement-on-engine.yaml` with the view diff of `commands/implement.yaml` — the moment, the
  eight 0076 mints, `impl.thinking-stage`, the `impl.gate-design-checkpoint` reword and anchor, the
  `impl.midrun-refire` reword.
- **Checklist run:** the AM-2 five for schema content.
- **Evidence read:** both migrations in full; the implement view diff in full; the
  `analysis-iterative` view (the `contract` condition, `blind-second-list`, `caller-slots`,
  `inputs-in-hand`, `own-turn-or-batch`, `mock-in-message`, `contract-pen`, `contract-inside`);
  `common.thinking-stage`; `0069-desk-on-engine.yaml` (the stub precedent);
  `spec.fail.story-outside-slate` (the mirrored fail); `impl.escalation-batching`,
  `impl.baselines-in-place-marked`, `impl.design-absent-baseline-seed`; implement-brainstorm-led
  record D3, D5 (S9, V2), D6 (S1, V3), D7 (S8), D8 (S2), *Supersessions owed*, *Touch set*, the
  *Defaults batch*; architecture-brainstorm-interview D8 (S6, S7, V1) and D9 (S9); plan §2.2–§2.3,
  §6 D-9, rev1 R3.
- **Pre-pass:** shared block above.
- **Intent stated — PASS.** 0076 names the moment, the seven domain rules by section, the floor fail,
  the checkpoint reword re-anchored to D8 with the superseded rendered-diagram clause, and the
  re-fire reword — seven plus the fail is the eight mints the diff shows. Its "No asking mechanic
  is restated" holds against the engine render: `impl.design-stage-unit` cites "the stage's
  own-turn-or-batch rule" and adds only D7's domain part; `impl.design-stage-inputs` names only the
  slot's content; `impl.design-seat-additions` grants `contract-inside`'s latitude by name and
  narrows it ("or confirmed default"). 0077 states its one stub.
- **Anchor present where required — PASS.** No exit. The one replaced anchor rides the header anchor
  the binary requires for an anchor change, and D8 re-rules that floor ("stays a `class: floor`
  gate: no code before the user signs … The floor's wording is amended at the build, never its
  force"); D9's share is named in the intent, one ruling per anchor (D-9). Rule anchors follow the
  wave's convention: D5 · D6 · D7 · D8 on the rules carrying them, the bare session anchor on the
  defaults (blind list, inputs, file, fail); the stub takes D3, as 0069's does.
- **ID lifecycle right — PASS.** Eight mints with the `impl.` prefix, none in `impl.sec.tools`; the
  `design-stage` moment is named in `impl.design-stage`'s text (no `unused-moment` finding);
  rewords keep id, class and kind (field diff). `impl.thinking-stage` declares `class: floor`
  locally and extends `common.thinking-stage`, its fourth extender after brainstorm, specify and
  architecture.
- **Floor and fail survival — PASS.** Pins 37 → 39 floors, 15 → 16 fails, the index gaining exactly
  two. `impl.gate-design-checkpoint` keeps `class: floor`, `kind: gate`, "no code is written before
  the user signs the design and the store delta", the flip and the base pin; the clause that leaves
  — "signed on a rendered diagram … (no render surface: present the source plus the changed-element
  table, and record it)" — is the text D9 S9 names as superseded, replaced by D9's surface: Mermaid
  that parses, rendered where the session can, a local preview first and a hosted page only on the
  user's yes, else the degrade note naming which read confirmed the parse. The new fail's
  `enforces:` lists four live local rules.
- **Register — PASS.** Plain words; "you" is the lead, as elsewhere in implement. Each rule matches
  its ruling: D5 (after run-open, before any drafting; zero-gap, no stage; the epic's spine always
  gives an agenda) · D6 S1 (new behaviour never joins; a story-internal piece re-graded and marked
  user-added; a lane run adds none) · D7 S8 (direction and seed; the owning seat proposes read-only
  on its plan-only dispatch; own turn or batch) · D8 S2 (the size split, the by-name latitude) ·
  *the blind list* (once per design phase, persona-less, `model: opus`, fenced inputs, skipped on a
  single-gap re-fire) · *handed-in sets* · *the set's home* (as ruled, whole before the pen passes,
  later rulings, open candidates) · *mid-build re-fire* (interrupts as `impl.escalation-batching`'s
  own exception; the same grade and landing-table checkpoint) · *the fail* (narrowed per S2, the
  seed excepted per *Greenfield seed kept apart*). The fail's end-state text also carries 0083's
  second exception, graded at unit 3.
- **VERDICT: PASS**
- **Issues requiring fix:** none.

audit: unit 2 (schema content · 0076 + 0077 + implement view diff) · w3-gate-grader · opus · 3 files · 1 rounds · 0 blocking

## Unit 3 — 0078, 0083 and the implement view diff

- **VALIDATE:** schema content · `0078-implement-architecture-touches.yaml` and
  `0083-artifact-home-stage-file.yaml` (its three implement ops; its `review-sufficiency` op is
  graded at 4c) with the view diffs of `commands/implement.yaml` (the 0078 and 0083 hunks) and
  `skills/review-sufficiency.yaml` (0083's hunk, cross-listed).
- **Checklist run:** the AM-2 five for schema content.
- **Evidence read:** both migrations in full; the implement view diff; the HEAD text and fields of
  every touched rule (field diff); `0037-run-log-and-in-run-rulings.yaml`, where
  `impl.artifact-home`'s "only" landed, and `.mochiko/decisions/2026-09-29-joint-hook-delta-build-seams.md`
  R1, the ruling it executes; the `decisions` home view; `impl.design-map-assertion`,
  `impl.baselines-in-place-marked`; the store skill's `diagram-set-grammar` and `orphan-rule`; the
  `architecture-diagram` template; the architecture command's visit-slug text;
  `testing-gap-finding.spine-outside-fence`; architecture-brainstorm-interview D5 (S17, S20), D8
  (S3, S4, S6, S7, V1, V7, V9), the S8 list; implement-brainstorm-led D8 and the defaults *the
  set's home* and *the fail*; the build log's Q1–Q8; plan §2.4, §6 D-3 to D-8, rev1 R0, R1, R3.
- **Pre-pass:** shared block above.
- **Intent stated — PASS.** 0078 names each touch with its ruling, the two rewords beyond §3.3 with
  why, and (D-11) the deployment view's move under S7. 0083 names its four rewords; its "the force
  … kept" for `impl.artifact-home` is graded below.
- **Anchor present where required — PASS.** No exit. Anchors are added only where none stood
  (raising protection needs no authority): `impl.design-inputs` D5; `impl.design-outputs-home`,
  `impl.deviation-gate`, `impl.store-landing` D8; `impl.design-review-pair` implement D8 under the
  architecture header (D-8, legal). Standing anchors kept (`impl.baseline-entry-grammar`,
  `impl.gap-finding-blind-dispatch`, `impl.artifact-home`). The two mints carry D5 and D8. 0083 sets
  no field.
- **ID lifecycle right — PASS.** Two mints: `impl.desk-offer` in reserved, `when: {scope: [selection,
  delta, epic]}` on declared values, the lane case out per Q4; `impl.index-regeneration` in
  ways-of-working with its pointer, outside tools as §3.3 item 10 asks. Every reword keeps id,
  class, kind, `when`, pointer and labels. 0083's second op this wave on three ids is disclosed
  (D-10).
- **Floor and fail survival — FAIL.** `impl.artifact-home` (`class: floor`) carries seams ruling R1:
  a ruling that changes a baseline or the store "is recorded on that entry and nowhere else", and
  "the `.mochiko/decisions/<date>-<slug>.md` route … takes only standing in-run rulings that touch no
  baseline"; 0037 landed it as "recorded on that entry only". 0083 drops "only" and puts "never in a
  ledger" in its place. In implement's own text a ledger is `baseline-delta.md` or
  `governance-ledger.md`, and the `decisions` home is "Standalone decision records" — so the floor
  no longer excludes the `.mochiko/decisions/` route R1 was ruled to close, and implement-brainstorm-led
  rules nothing on R1. The cut is also unneeded: the new clause "the run's `design-stage.md` holds
  the design stage's ruled set as the run's input, never as that record" already keeps "only" true;
  and the lead's Q3 answer took the plan's ready text, which kept "only". The other floors hold:
  `impl.deviation-gate` keeps its class, kind and stop, its reach narrowed to the container and flow
  views by S7 ("ER, state, flowchart and other views … never as cycle stops"), the deployment
  view's move disclosed in the intent; the fail's 0083 exception (a post-sign build-time decision)
  matches *the fail*'s "design-phase write" scope, and its second limb still catches a build-time
  write that changes a ruling.
- **Register — PASS.** Each 0078 reword matches its ruling. `impl.design-inputs`: D5's visit file,
  the FEAT id lower-cased in its slug as the architecture command writes it, S20's read against the
  run's pinned base with conflicts at the checkpoint, and `design-stage.md`; S17's link stays with
  `impl.design-map-assertion`, which fills it when a store delta exists, as a structural visit-file
  run has. `impl.desk-offer` is D5's offer default. `impl.design-outputs-home`: drawn delta-styled
  into the store's own views before code, `architecture.md` the neighborhood view plus the table
  listing every edited diagram file (S6, V1, V9); 0083's key-flow clause matches
  `diagram-set-grammar`'s "the spine keeps its own container view and key-flow sequences".
  `impl.baseline-entry-grammar`: S3's per-file `**Lifecycle:**`, matching the
  `architecture-diagram` template's Draws and Lifecycle sections. `impl.store-landing`: the flip
  clears delta styling with no redraw, the orphan check names keyed diagrams (0070's
  `orphan-rule`), the built-vs-signed diff reads every listed file and view kind (S7).
  `impl.gap-finding-blind-dispatch`: the signed `architecture.md` with its listed diagram files
  (touch 8, under the one signed object S6 and V1 repaired). `impl.index-regeneration` is S4 and V7
  word for word. Staffing and the review pair gain the diagram edits and the meaning-kept grade.
- **VERDICT: FAIL**
- **Issues requiring fix:**
  1. Floor and fail survival · `impl.artifact-home` has lost R1's "nowhere else": the floor no
     longer excludes recording a baseline- or store-changing ruling in a `.mochiko/decisions/`
     record · Fix: a fix migration in the held range (0084, header anchor `2026-10-11
     implement-brainstorm-led`, the floor's own anchor kept) rewording `impl.artifact-home` to
     0083's text with "recorded on that entry," restored to "recorded on that entry only," and
     everything else byte-identical — "…is recorded on that entry only, its reason in the entry's
     own fields (impl.baseline-entry-grammar), never in a ledger; the run's `design-stage.md` holds
     the design stage's ruled set as the run's input, never as that record; …" — then re-emit the
     views.

audit: unit 3 (schema content · 0078 + 0083 + implement and review-sufficiency view diffs) · w3-gate-grader · opus · 4 files · 1 rounds · 1 blocking

### Unit 3, round 2 — the 0084 fix, re-audited (same seat, resumed)

- **VALIDATE:** schema content · `plugins/mochiko/migrations/0084-artifact-home-only-restored.yaml`
  with the `impl.artifact-home` hunk of `.mochiko/schema-views/commands/implement.yaml`.
- **Checklist run:** the AM-2 five for schema content, scoped per `common.gate-loop-bound` to what
  the fix touched and what it could have broken.
- **Evidence read:** 0084 in full; the regenerated views against round 1's replay; a word diff of
  `impl.artifact-home`'s text, 0083 → 0084 and 0037 (HEAD) → 0084; the seams R1 record; the
  rule's fields; every view for other rules citing `impl.artifact-home` (none); 0075–0083 re-hashed,
  each byte-identical to round 1.
- **Pre-pass:** run first-hand —
  `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` → `rules scanned: 1243 ·
  in-kind pairs scored: 213340 · clusters: 0 (none)` · `allowlist-suppressed edges: 184` ·
  `mochiko-cli migrate validate · 0 rejecting · 116 advisory` (exit 0); the sorted advisory set
  differs from round 1 by one line only, `budget · command/implement` 125 rules · 42,178 → 42,183.
  `mochiko-cli migrate status --plugin-root plugins/mochiko` → `sequences 1..84 (76 migrations)` ·
  `state sha256:58f165c81837e3343e6938a0c65655411a23b4942e79213baf8aab65213bdf33 · 93 documents ·
  1243 rules`. `migrate stamp` on a hash-stripped copy → `sha256:6bbee4f6…39fb9ad0`, equal to the
  committed line. `views emit` to scratch, `diff -r` against `.mochiko/schema-views` → empty; against
  round 1's replay only `commands/implement.yaml` differs, by the one line carrying "only". Pins
  unchanged: `kind: fail · 16 rules` · `class: floor · 39 rules`; ways-of-working 6,347 B.
- **Intent stated — PASS.** It names the round-1 finding, R1 and its 0037 landing, the one word
  restored, the two clauses kept, and the fields kept.
- **Anchor present where required — PASS.** No exit; one `reword-rule`, no `set-rule-field`; the floor
  keeps `2026-09-13 hook-enforced-artifact-schema D1`; header anchor well-formed.
- **ID lifecycle right — PASS.** The id survives; class `floor`, kind `binding`, labels unchanged.
- **Floor and fail survival — PASS.** 0083 → 0084 is one token, "entry," → "entry only,". Against
  HEAD (0037's text) the rule now only gains — the `design-stage` template binding, "never in a
  ledger", and the `design-stage.md` clause — and loses nothing, so R1's "on that entry and nowhere
  else" stands whole; "only" and "never in a ledger" agree, and "the run's input, never as that
  record" keeps "only" true beside `design-stage.md`.
- **Register — PASS.** Plain, as before; the sentence reads whole.
- **VERDICT: PASS**
- **Issues requiring fix:** none — round 1's issue 1 is closed by 0084.

audit: unit 3 (schema content · 0078 + 0083 + 0084 + implement and review-sufficiency view diffs) · w3-gate-grader · opus · 5 files · 2 rounds · 0 blocking

## Unit 4a — 0079 and the store skill and store template view diffs

- **VALIDATE:** schema content · `0079-store-landing-diagrams.yaml` with the view diffs of
  `skills/authoring-architecture-store.yaml` and `templates/architecture-store.yaml`.
- **Checklist run:** the AM-2 five for schema content.
- **Evidence read:** the migration in full; both view diffs; the store skill's `orphan-rule` and
  `diagram-set-grammar`; architecture-brainstorm-interview D8 (S3, S4); plan §2.5, §6 D-6; the
  build log's Q2.
- **Pre-pass:** shared block above.
- **Intent stated — PASS.** It names both rewords and the template replace, "every other template
  line stands" — the template view diff is exactly two hunks, the Health contract item and the
  skeleton's health line.
- **Anchor present where required — PASS.** No exit; the header carries the bare session anchor;
  `fold-duty` keeps `2026-08-19 product-architecture-schema D10`, `health-view-binding` keeps arch D8.
- **ID lifecycle right — PASS.** Two rewords keep their ids and fields; the template moves by
  `replace-document`, the only op a template takes.
- **Floor and fail survival — PASS.** Neither rule is a floor (store floors 10 → 10);
  `fold-duty` keeps `when: {structure_built: present}`.
- **Register — PASS.** `fold-duty` flips diagrams with the elements, keys cleared, delta styling to
  plain, no redraw (D8, S3's per-file Lifecycle); `health-view-binding` and the template's Health
  item and skeleton line name diagrams as orphans in the `orphan-rule`'s own terms. The crate
  fixture the replace changes is unit 8's (Q2).
- **VERDICT: PASS**
- **Issues requiring fix:** none.

audit: unit 4a (schema content · 0079 + authoring-architecture-store and architecture-store template view diffs) · w3-gate-grader · opus · 3 files · 1 rounds · 0 blocking

## Unit 4b — 0080 and the review-plan-artifacts view diff

- **VALIDATE:** schema content · `0080-plan-artifacts-meaning-kept.yaml` with the view diff of
  `skills/review-plan-artifacts.yaml`.
- **Checklist run:** the AM-2 five for schema content.
- **Evidence read:** the migration in full; the view diff; `material-divergence-autofail` and the
  `incremental` condition (the precedent); implement-brainstorm-led D6 (S1), D8 (S2); the build
  log's Q8; plan §2.6, §6 D-2.
- **Pre-pass:** shared block above.
- **Intent stated — PASS.** The condition, the mint and the floor reword, and why
  `material-divergence-autofail` is not reworded, are all stated.
- **Anchor present where required — PASS.** No exit; `gap-list-scope` keeps `2026-08-26
  plan-stage-utility`; `meaning-kept` carries implement D8.
- **ID lifecycle right — PASS.** `design_stage` is declared (presence, entry-derived) and named by
  `meaning-kept`'s `when:`; its `absent` coverage advisory is the `incremental` precedent's.
  `kind: gate` is in the skill's eight-kind set.
- **Floor and fail survival — PASS.** `gap-list-scope` stays `class: floor`, `kind: binding`, "never a
  fixed artifact set"; the list grows only by the user-added gap D6 rules in, and the unchanged
  floor `material-divergence-autofail` reads "the gap list" through it (D-2, agreed). Floors 11 → 11.
- **Register — PASS.** `meaning-kept`: the drafts against `design-stage.md` at conformance's strength
  (Q8); a ruled item changed with no post-pen ruling is blocking; additions split on D8 S2's line —
  a small extra listed for the user's OK, never a finding on its own; a ruling-changing or
  user-noticeable one with no ruling blocking; the result fills the landing table. It agrees with
  `impl.design-seat-additions` and `impl.fail.design-outside-ruled-set`.
- **VERDICT: PASS**
- **Issues requiring fix:** none.

audit: unit 4b (schema content · 0080 + review-plan-artifacts view diff) · w3-gate-grader · opus · 2 files · 1 rounds · 0 blocking

## Unit 4c — 0081, 0083's re-grade op and the review-sufficiency view diff

- **VALIDATE:** schema content · `0081-sufficiency-user-added.yaml` and 0083's
  `review-sufficiency.user-added-regrade` op with the view diff of `skills/review-sufficiency.yaml`.
- **Checklist run:** the AM-2 five for schema content.
- **Evidence read:** 0081 in full; 0083's fourth op; the view diff (end state);
  `fence-never-reads`, `every-applicable-clause`, `report-binding`; implement-brainstorm-led D6
  (S1); the build log's Q7; rev1 R1.
- **Pre-pass:** shared block above.
- **Intent stated — PASS.** 0081 states the mint and its fence; 0083 states that the never-read now
  covers `design-stage.md` in an epic's home as in a feature's.
- **Anchor present where required — PASS.** No exit; the rule carries D6 and 0083 keeps it.
- **ID lifecycle right — PASS.** One mint in `review-sufficiency.sec.scope` (`must`, `duty`); the 0083
  reword keeps the id (D-10 disclosed).
- **Floor and fail survival — PASS.** No floor touched (8 → 8). The rule keeps `fence-never-reads`
  whole and extends never-read to `design-stage.md` in either home; it grades "on the clause set
  like any unit", so `every-applicable-clause` binds it.
- **Register — PASS.** D6 S1: graded before it joins, each gap marked user-added, the seat the
  report's one writer, new behaviour back to the lead and never a gap; "or saying there that every
  clause holds" is Q7(b); a lane run never adds, so no lane case is owed.
- **VERDICT: PASS**
- **Issues requiring fix:** none.

audit: unit 4c (schema content · 0081 + 0083's user-added-regrade op + review-sufficiency view diff) · w3-gate-grader · opus · 3 files · 1 rounds · 0 blocking

## Unit 4d — 0082 and the testing-gap-finding view diff

- **VALIDATE:** schema content · `0082-gap-finding-signed-set.yaml` with the view diff of
  `skills/testing-gap-finding.yaml`.
- **Checklist run:** the AM-2 five for schema content.
- **Evidence read:** the migration in full; the view diff; `spine-outside-fence`; the store skill's
  `diagram-set-grammar`; `impl.gap-finding-blind-dispatch` at its 0078 end state;
  architecture-brainstorm-interview S8 item 8, D8 (S6, V1); the build log's Q5; plan §6 D-1.
- **Pre-pass:** shared block above.
- **Intent stated — PASS.** The one reword, the limbs that stand, and the spine kept outside.
- **Anchor present where required — PASS.** Header `2026-10-11 architecture-brainstorm-interview D8`
  (Q5); the floor keeps its own anchor; a fence widened by ruling is not an exit.
- **ID lifecycle right — PASS.** One reword, id and fields kept.
- **Floor and fail survival — PASS.** The floor keeps its class and kind, its blindness to code and
  cards and its exclusion of the `FEAT-XXX/` directory at large; the one widening — the store diagram
  files under `diagrams/` that the signed `architecture.md` lists — is touch 8's ruling; the
  Lifecycle admission (built and in-flight class, never `proposed`) applies to those files; the
  `diagrams/` path keeps the spine's views out, so `spine-outside-fence` stands. It matches the
  command's dispatch rule. Floors 9 → 9.
- **Register — PASS.** One clause, plain words, paths exact.
- **VERDICT: PASS**
- **Issues requiring fix:** none.

audit: unit 4d (schema content · 0082 + testing-gap-finding view diff) · w3-gate-grader · opus · 2 files · 1 rounds · 0 blocking

## Unit 5 — the implement command pair

- **VALIDATE:** command pair · `plugins/mochiko/commands/implement.md` (unchanged this wave:
  `git diff HEAD` empty) with its rules as rendered, `mochiko-cli rules implement --section <id>
  --plugin-root plugins/mochiko` for `preamble` and the six `impl.sec.*` sections; plus change
  control against the engine's render and the delivery check.
- **Checklist run:** `.claude/rules/mochiko/primitive-edits.md`'s canonical-scaffold criteria 1–11,
  the run-command done-condition branch and implement's own criterion
  (`validation-primitive-edit.judgment-items-pair`); the engine re-read; the tools ceiling.
- **Evidence read:** `implement.md` in full; all seven rendered blocks in full, and the same seven
  replayed from the HEAD log; the `analysis-iterative` render, every section, and its `SKILL.md`
  load-first lines; the implement view's fail-id and kind correspondence; the budget ledger (no
  command row); the record defaults *Precedence*, *Slots declared where*, *Change control*;
  architecture-brainstorm-interview D8 S7; units 2 and 3 above.
- **Pre-pass:** run first-hand — `mochiko-cli migrate validate --report --plugin-root
  plugins/mochiko` → `pointer resolution: 89 checked against plugins/mochiko` · `rules scanned:
  1243 · in-kind pairs scored: 213340 · clusters: 0 (none)` · `allowlist-suppressed edges: 184` ·
  `mochiko-cli migrate validate · 0 rejecting · 116 advisory` (exit 0); implement's own findings
  are advisory only — six `condition-coverage`, two `unused-moment` (`card-confirm`,
  `cycle-checkpoint`), `enforces-coverage` 33, `budget · 125 rules · 42183`, one
  `zero-member-label`. `migrate status` → `sequences 1..84 (76 migrations)` · `58f165c8…` ·
  `93 documents · 1243 rules`. Char budgets: commands carry no per-primitive budget, and
  `implement.md` is unchanged. Block sizes, bytes by `wc -c` (HEAD → now): preamble 4,734 → 5,016 ·
  roles 4,128 → 5,322 · reserved 3,139 → 5,597 · tools 26,201 → 27,651 (27,396 characters) ·
  ways-of-working 3,802 → 6,347 · boundaries 5,453 → 5,991 · fail-conditions 4,580 → 5,162.
- **Delivery, first-hand.** The seven `!` lines of `implement.md`, run verbatim with
  `CLAUDE_PLUGIN_ROOT` set to `plugins/mochiko`, each exit 0, each opening with `mochiko-cli rules
  implement · section <id> · binary 0.4.0 · grammar 2 · plugin 0.119.0` and closing with `mochiko-cli
  rules end · implement · <id> · <N> rules` (0 · 18 · 12 · 50 · 13 · 16 · 16; 125 in all, the
  validate count). `impl.thinking-stage` renders in boundaries as `[class: floor · labels:
  floor-pointer, user-gate · pointer: mochiko:analysis-iterative]` with `common.thinking-stage`'s
  text resolved. The pointer target's seven `!` lines in `analysis-iterative/SKILL.md`, run verbatim,
  each exit 0 with both lines (0 · 2 · 2 · 2 · 9 · 3 · 3 rules); that skill directory is unchanged
  against HEAD, and its seven-block render is byte-identical to the HEAD log's, the plugin field
  aside.
- **Scaffold conformance — PASS.** The frontmatter carries `description` · `argument-hint` ·
  `disable-model-invocation: true` · `allowed-tools: Bash(mochiko-cli *)`; `# Implement — the Single
  Downstream Run` · `## Identity & Mission` · `## Rules — delivered by mochiko-cli` · `## Adaptive
  Goal Protocol` with Entry → Goal → Not done last; `$ARGUMENTS` handled in Entry; no `**Goal:**`
  opener, no Harness or Bindings, no extra top-level section.
- **Rules-block enumeration — PASS.** The seven lines enumerate `preamble` and the six-set the
  preamble's `sections` lists, set-wise; every `impl.sec.*` token in the `.md` resolves.
- **FAIL survival — PASS.** The 15 HEAD fails survive and one is added
  (`impl.fail.design-outside-ruled-set`); the fail-conditions block holds 16 rules, all `kind:
  fail`, and every `.fail.` id is `kind: fail` and the reverse (no mismatch). The Not-done line
  cites the `kind: fail` pin and obliges a halt on an end-line disagreement, with no number;
  today's pin `kind: fail · 16 rules` equals the block's end line `16 rules`.
- **ID continuity — PASS.** No id vanished or moved (units 2 and 3); tombstones unchanged; no
  surviving text cites a retired node.
- **Floor survival — PASS.** All 37 HEAD floors stay `class: floor` (39 now). `impl.deviation-gate`
  keeps its class, kind, stop and "never design around it silently"; its cycle-stop set narrows
  to the run's signed `architecture.md` and the container and flow views of the listed diagram
  files, the deployment view leaving it for the landing's built-vs-signed diff, which
  `impl.store-landing` now holds to "every view kind included" — the narrowing is S7's ruling ("ER,
  state, flowchart and other views are checked at the landing diff, never as cycle stops"), and
  `impl.fail.deviation-unresolved` still enforces the gate. `impl.gate-design-checkpoint` keeps its
  force (unit 2); `impl.artifact-home` keeps R1 whole (unit 3, round 2).
- **Substance across the pair — PASS.** The P1 plan was peer-graded and approved only on PASS
  (build log, round 2). No seat grades its own row: the blind list is a fresh persona-less seat;
  the re-grade is the sufficiency seat, which authored none of the piece; the drafts are graded by
  the non-author review pair, now with the meaning-kept grade. The new user decisions sit in
  reserved (`impl.design-stage-agenda`, `impl.design-seat-additions`, `impl.desk-offer`, the
  checkpoint). The bindings are complete: `design-stage.md` and its template bind in
  `impl.artifact-home` and `impl.design-stage-file`, in ways-of-working beside the binding rule that
  already lived there, since the wave added no rule to tools. The floor pointer lines
  `impl.thinking-stage` and `impl.sound-loop-floor` sit in boundaries.
- **Done-condition branch (run command, implement) — PASS.** Entry carries the gating and the
  neither-source routing; Goal states the fixed done condition; Not done defaults to FAIL and is
  pin-cited. Convergence stays at the existing run-open gate — batch, scope type, both attempt bounds
  at their only redeclaration point, the done condition — and closes at final acceptance; the
  design stage runs after run-open and leaves it one confirmation (`impl.design-stage`, D5), so no
  new ceremony enters the goal protocol.
- **Preserved responsibilities — PASS.** No rule left; the clauses that changed meaning leave by
  ruling — the rendered-diagram clause (D9 S9), the drawing's contents relocated into the store's
  views (architecture D8, superseding `architecture-design-primitive` D5-plan-run-artifact in part),
  the deviation narrowing (S7). Their strip entries and `DECISIONS.md` annotations are unit 7's
  and the landing's.
- **Change control — PASS.** Each engine rule re-read against implement's render: the lead holds the
  conversation (seats propose read-only, the lead puts it to the user); `blind-second-list`'s slot
  is set by name — cadence once per design phase, inputs the spec, gap list, index and visit file
  "only" (never options, conversation or ruled set), job the hidden choices per gap, a fresh seat;
  `inputs-in-hand` is filled with content only; the asking grammar is cited, never restated, and
  where implement adds it tightens — own-turn pieces put as what the user would notice with a worked
  example and never a schema, batched pieces with suggestion and cost; `provenance-marks` and
  `open-unknowns` are carried by the template's vocabulary and open seed; `contract-pen` matches
  `impl.design-stage-file`; `contract-inside`'s latitude is granted by name in
  `impl.design-seat-additions` and narrowed ("or confirmed default"), its halt kept. The gate and
  the fail stay implement's, as `caller-slots` requires. No implement rule loosens an engine
  mechanic.
- **Tools ceiling — PASS.** `impl.sec.tools` renders 27,651 B against the 29,000 B line under the
  30,000-character `!` ceiling; its rule set (50 headings) is identical to HEAD's, so no rule was
  added — the growth is rewords only.
- **Ontology grammar — PASS.** The CLI asserts the grammar (0 rejecting). By judgment:
  `impl.desk-offer`'s guard is in `when:` alone (MOVE); the new fail's reach rides its subject
  noun, as `spec.fail.story-outside-slate`'s does; the stub meets the 3+-command bar (four
  extenders); the new moment is declared and named.
- **VERDICT: PASS**
- **Issues requiring fix:** none.

audit: unit 5 (command pair · implement, with change control, delivery and the tools ceiling) · w3-gate-grader · opus · 4 files · 1 rounds · 0 blocking

## Shared pre-pass — units 6a–6d and 7

Run first-hand from the repo root after P2 landed (P3's `crates/**/tests` and `evals/**` edits are
in the tree and were not read):

```
$ mochiko-cli migrate validate --report --plugin-root plugins/mochiko
pointer resolution: 89 checked against plugins/mochiko
rules scanned: 1243 · in-kind pairs scored: 213340 · clusters: 0 (none)
allowlist-suppressed edges: 184
mochiko-cli migrate validate · 0 rejecting · 116 advisory        (exit 0)

$ mochiko-cli migrate status --plugin-root plugins/mochiko
log plugins/mochiko/migrations · grammar 2 · sequences 1..84 (76 migrations)
state sha256:58f165c81837e3343e6938a0c65655411a23b4942e79213baf8aab65213bdf33 · 93 documents · 1243 rules
```

Char budgets, the ledger's canonical snippet (`.mochiko/memory/primitive-cost-budgets.md`, *How to
measure*) for body and `description:`, plus each `SKILL.md`'s seven `!` lines run verbatim with
`CLAUDE_PLUGIN_ROOT=plugins/mochiko`, full stdout counted in characters. Every `!` line exits 0
and carries its version-triple and end lines. HEAD is the same measure over `git show HEAD:` bodies
and the HEAD log copy (1..70):

| skill | HEAD body · render · payload | now body · render · payload | description | budget | over |
|---|---|---|---|---|---|
| authoring-architecture-store | 5,878 · 18,314 · 24,192 | 5,878 · 18,398 · 24,276 | 492 = 492 | 19,733 | +4,543 |
| review-plan-artifacts | 3,363 · 14,909 · 18,272 | 3,462 · 16,045 · 19,507 | 598 = 598 | 18,013 | +1,494 |
| review-sufficiency | 3,262 · 13,359 · 16,621 | 3,262 · 14,098 · 17,360 | 686 = 686 | 15,423 | +1,937 |
| testing-gap-finding | 7,005 · 16,203 · 23,208 | 7,005 · 16,300 · 23,305 | 770 = 770 | 19,977 | +3,328 |
| patterns-system-design (prose) | body 10,073 | body 10,298 | 625 = 625 | 11,047 | under |
| mochiko router (prose) | body 48,637 | body 48,744 | 206 | unbudgeted | — |

Stepped prefix replays of the log (1..79 · 1..80 · 1..82 · 1..83 · 1..84) attribute every render
move: the store +84 lands by 0079; `review-plan-artifacts` +1,136 at 0080 alone; `review-sufficiency`
+704 at 0081 and +35 at 0083; `testing-gap-finding` +97 at 0082; nothing else moves them. The four
ledger rows' figures, splits and attributions reproduce exactly. `git diff HEAD` is empty for the
`SKILL.md` and `references/` of `authoring-architecture-store`, `review-sufficiency` and
`testing-gap-finding`, for every agent and for every command. A word-level diff shows the ledger,
the strips and the router as additions only; CHANGELOG removes only its `[lead: Wave 3 …]`
placeholder.

## Unit 6a — the authoring-architecture-store skill pair

- **VALIDATE:** skill pair · `plugins/mochiko/skills/authoring-architecture-store/SKILL.md`
  (unchanged) with its seven-section render; the named overage in ledger row `:82`.
- **Checklist run:** the skill-pair criteria 1–12 (`primitive-edits.md`), budget item 8 the one
  judgment that moved.
- **Evidence read:** `SKILL.md` in full (empty `git diff`); the render's seven blocks; the ledger
  row's diff; unit 4a above (0079); architecture-brainstorm-interview D8 (S3) and 0070's `orphan-rule`.
- **Pre-pass:** shared block above.
- **Load-first, section enumeration, floor pin and read-back — PASS.** The body is unchanged: seven
  `!` lines over the authoring six-set, each exiting with both lines; the read-back cites the pin
  (`class: floor · 10 rules`, unchanged) and the `floors:` line.
- **Floor survival · ID continuity · `extends:` · pointers · skill grammar · anchors — PASS.** The
  CLI is clean (0 rejecting); two rewords kept their ids, classes and anchors (unit 4a); no
  `extends:` added.
- **`description:` untouched — PASS.** 492 = 492, under the 1,536 cap.
- **Budget — PASS, the overage HOLDS.** +84 this wave, render only, from 0079 — `fold-duty` +71 (the
  landing flip over diagrams as over elements, the delta styling cleared to plain with no redraw,
  D8 and S3) and `health-view-binding` +13 (orphans naming diagrams, 0070's ruled `orphan-rule`). Both are ruled obligations;
  the body is unchanged, so no prose returns. The standing +4,459 is wave 2's figure.
- **Preserved responsibilities — PASS.** No body line is made false: the fold table's "statuses
  flipped" covers diagrams; nothing leaves.
- **VERDICT: PASS**
- **Issues requiring fix:** none.

audit: unit 6a (skill pair · authoring-architecture-store, overage +4,543 HOLDS) · w3-gate-grader · opus · 3 files · 1 rounds · 0 blocking

## Unit 6b — the review-plan-artifacts skill pair

- **VALIDATE:** skill pair · `plugins/mochiko/skills/review-plan-artifacts/SKILL.md` (E1, `:42–43`)
  with its seven-section render; the named overage in ledger row `:78`.
- **Checklist run:** the skill-pair criteria 1–12.
- **Evidence read:** `SKILL.md` in full and its `git diff`; the render's seven blocks; the ledger
  row's diff; unit 4b above (0080); the strip's W3 comment; P2 plan §3 E1 (sha256 `3e0dbc1a…`,
  verified); implement-brainstorm-led D8 (S2), D6 (S1).
- **Pre-pass:** shared block above.
- **Load-first, section enumeration, floor pin and read-back — PASS.** The seven `!` lines over the
  review six-set and the read-back sentence are byte-identical to HEAD; the pin is `class: floor ·
  11 rules`, unchanged.
- **Floor survival · ID continuity · `extends:` · pointers · skill grammar · anchors — PASS.** The
  CLI is clean; `meaning-kept` is `kind: gate`, legal in the eight-kind set; `gap-list-scope` keeps
  its floor and anchor (unit 4b).
- **`description:` untouched — PASS.** 598 = 598.
- **Budget — PASS, the overage HOLDS.** +1,235 this wave: render +1,136 from 0080 —
  `meaning-kept` +854 and its `design_stage` guard +189 (D8 S2 names this skill as the
  meaning-kept grader), the `gap-list-scope` reword +93 (D6 S1's grown list) — and body +99, E1's
  procedure line, which names the meaning-kept lens beside conformance in the walk order. E1 is a
  pure insertion that sequences a gate the render now carries; it restates no rule text and
  restores no playbook prose. The standing +259 was ruled HOLDS at [v0.118.0].
- **Preserved responsibilities — PASS.** E1 only adds; the four lenses, their order and the
  checklist pointer stand.
- **VERDICT: PASS**
- **Issues requiring fix:** none.

audit: unit 6b (skill pair · review-plan-artifacts, overage +1,494 HOLDS) · w3-gate-grader · opus · 3 files · 1 rounds · 0 blocking

## Unit 6c — the review-sufficiency skill pair

- **VALIDATE:** skill pair · `plugins/mochiko/skills/review-sufficiency/SKILL.md` (unchanged) with
  its seven-section render; the named overage in ledger row `:80`.
- **Checklist run:** the skill-pair criteria 1–12.
- **Evidence read:** `SKILL.md` in full (empty `git diff`); the render's seven blocks; the ledger
  row's diff; unit 4c above (0081, 0083's op); implement-brainstorm-led D6 (S1).
- **Pre-pass:** shared block above.
- **Load-first, section enumeration, floor pin and read-back — PASS.** Unchanged body; seven `!` lines
  over the review six-set; the pin is `class: floor · 8 rules`, unchanged.
- **Floor survival · ID continuity · `extends:` · pointers · skill grammar · anchors — PASS.** The
  CLI is clean; one mint (`must`, `duty`), its 0083 reword keeping its id (unit 4c).
- **`description:` untouched — PASS.** 686 = 686.
- **Budget — PASS, the overage HOLDS.** +739 this wave, render only — 0081's `user-added-regrade` +704
  (D6 S1: the re-grade before a user-added piece joins, the seat the report's one writer) and
  0083's +35 (the same rule's read fence over an epic's home). These are ruled obligations; the body
  is unchanged. The standing +1,198 was ruled HOLDS at [v0.116.0].
- **Preserved responsibilities — PASS.** No body line is made false; the procedure's per-unit
  clause walk covers the re-graded piece as "any unit".
- **VERDICT: PASS**
- **Issues requiring fix:** none.

audit: unit 6c (skill pair · review-sufficiency, overage +1,937 HOLDS) · w3-gate-grader · opus · 3 files · 1 rounds · 0 blocking

## Unit 6d — the testing-gap-finding skill pair

- **VALIDATE:** skill pair · `plugins/mochiko/skills/testing-gap-finding/SKILL.md` (unchanged) with
  its seven-section render; the named overage in ledger row `:92`.
- **Checklist run:** the skill-pair criteria 1–12.
- **Evidence read:** `SKILL.md` in full (empty `git diff`); the render's seven blocks; the ledger
  row's diff; unit 4d above (0082); architecture-brainstorm-interview S8 item 8.
- **Pre-pass:** shared block above.
- **Load-first, section enumeration, floor pin and read-back — PASS.** Unchanged body; seven `!` lines
  over the review six-set (the small-families reuse); the pin is `class: floor · 9 rules`, unchanged.
- **Floor survival · ID continuity · `extends:` · pointers · skill grammar · anchors — PASS.** The
  CLI is clean; one floor reword, id and anchor kept (unit 4d).
- **`description:` untouched — PASS.** 770 = 770.
- **Budget — PASS, the overage HOLDS.** +97 this wave, render only, 0082's fence reword — the store
  diagram files the signed `architecture.md` lists join the admissible inputs (D8, the S8 list's
  eighth touch). It is a ruled obligation; the body is unchanged. The standing +3,231 was ruled HOLDS
  at [v0.118.0].
- **Preserved responsibilities — PASS.** The body names the fence and inclusion list without
  listing it; no line is made false.
- **VERDICT: PASS**
- **Issues requiring fix:** none.

audit: unit 6d (skill pair · testing-gap-finding, overage +3,328 HOLDS) · w3-gate-grader · opus · 3 files · 1 rounds · 0 blocking

## Unit 7 — prose primitives and records

- **VALIDATE:** prose primitives and records · `patterns-system-design/SKILL.md` (E4, E5) ·
  `review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md` (E2, E3) · the router
  `skills/mochiko/SKILL.md` (E6, E7) · strips `patterns-system-design.md`, `review-plan-artifacts.md`,
  `implement.md`, `mochiko.md` · `README.md` (`:108`, `:134`) · the budget ledger rows `:73`, `:78`,
  `:80`, `:82`, `:92` · `CHANGELOG.md`'s W3 slot.
- **Checklist run:** coherence and preserved responsibilities
  (`validation-primitive-edit.judgment-items-prose`); the strip contract (`.mochiko/strips/README.md`);
  the char budget for the budgeted prose skill.
- **Evidence read:** every file above in full where edited and by `git diff HEAD`; P2's plan §3–§7
  (sha256 `3e0dbc1a…`, verified); `patterns-system-design/references/` (grep); `ARCHITECTURE.md`'s
  implement section; architecture-brainstorm-interview D8 (S6, S7, V1, V9) and D9 (S9);
  implement-brainstorm-led D5, D8; units 2–6 above.
- **Pre-pass:** shared block above; `patterns-system-design` body 10,298 under its 11,047 budget,
  description 625 unchanged; the router is unbudgeted.
- **Coherence — PASS.**
  - `patterns-system-design`'s overview now says what 0078's `impl.design-outputs-home` says:
    the delta is drawn delta-styled into the store's own diagrams, and the run's `architecture.md`
    is the signed drawing (the neighborhood view plus the table listing the edited diagram files).
    §1's sign-off surface, §3's table "lists, by path, every store diagram file the change edited",
    the neighborhood scope and the D9 lines from wave 2 agree with it.
  - `ARTIFACT-CHECKLISTS.md`'s store-delta intro grades the pinned-base diff of every edited
    `diagrams/` file plus the table that lists them. Its `:80`/`:89` read Mermaid text that parses
    and "the container delta diagram" (D9). A grep for "rendered diagram" over
    `plugins/mochiko/**/*.md` returns nothing.
  - The router's rows mirror 0082 and the design stage, in `impl.design-stage`'s own wording.
  - `README.md:108` names the container and flow views as cycle stops, the other views going to
    the landing's diff (S7, matching `impl.deviation-gate`). `:134` places the design stage before
    the author ↔ grade ↔ sign-off loop.
  - The CHANGELOG slot's facts check against the tree: 13 mints (11 implement), 15 rewords,
    1,230 → 1,243 rules, pins 16 and 39, tools 27,650 B, implement 51,441 → 60,392 characters, the
    four payloads, 3 strip entries, and the deviation narrowing under S7 with its README ripple.
- **Preserved responsibilities — PASS.**
  - Each removal has its strip entry: psd's overview apposition, and the checklist's two "rendered
    diagram" clauses. Each entry carries the disposition, "Tier failed: n/a — supersession by
    ruling" citing D8 (S6, V1, V9) or D9, the superseded text verbatim (it matches the pre-edit
    lines), what was kept and which consumers were assessed.
  - Pure additions (E1, E5, E6, E7, E2's diff clause) ride the wave-context comments.
    `strips/implement.md`'s comment names D8, D9 and `architecture-design-primitive`
    D8-rendered-diagram-signoff and D5-plan-run-artifact. That closes the round-1 advisory on
    `impl.gate-design-checkpoint`'s single anchor.
  - The ledger rows only prefix new segments; every prior segment and every standing ruling is
    kept verbatim.
  - The lead's rulings stand: FEASIBILITY-LENS.md `:99–101` and ARTIFACT-CHECKLISTS.md `:91` are
    incomplete, not false, and go to BACKLOG at the landing.
- **VERDICT: PASS**
- **Issues requiring fix:** none.

audit: unit 7 (prose primitives and records · patterns-system-design, ARTIFACT-CHECKLISTS, router, four strips, README, budget ledger, CHANGELOG W3 slot) · w3-gate-grader · opus · 11 files · 1 rounds · 0 blocking

## Unit 8 — crate tests, contract expectations and the implement plan kit

- **VALIDATE:** `crates/mochiko-cli/tests/{fidelity,matrix_similar,render,validate,views}.rs` ·
  `tests/fixtures/template/design-stage.{producer,check}.txt` (new) and
  `architecture-store.producer.txt` (recaptured, lead ruling W3 P1 Q2) · `evals/contract/run.py` ·
  `evals/contract/README.md` · `evals/plan/README.md` (`:45`, lead ruling Q3) ·
  `evals/plan/implement/observable.yaml` · `evals/plan/implement/preregistration.md`. Also the
  crate's independent non-author code review (`.claude/rules/mochiko/rust-cli.md`), tests and
  fixtures only.
- **Checklist run:** wave plan §3.7 (figures by tool, fixtures from the binary,
  `Expected["implement"]`, frozen skill floor sets, `TEMPLATE_NAMES`, README counts, the kit
  re-key, one honest preregistration amendment, kit static checks, no grid); the lead's grade
  list; rust-cli.md's quality gate and non-author review; coherence and preserved
  responsibilities (`validation-primitive-edit.judgment-items-prose`) for the prose files.
- **Evidence read:** every file above by `git diff HEAD`, `fidelity.rs:539–665` in full; P3's
  plan (sha256 `51bcbfdb…`, verified) and the lead's Q1–Q3 answers; the implement view now and
  at HEAD; the 36 skill views now and at HEAD; the allowlist locator (`similar.rs:981`); raw
  no-allowlist sweeps over scratch copies of the log at `1..70` and `1..84`; the
  `rules implement --section preamble` render; the `partition --old-ref 7d9d29e` output.
- **Pre-pass** (first-hand, repo root):
  - `migrate validate --report`: `0 rejecting · 116 advisory` · `rules scanned: 1243 · in-kind
    pairs scored: 213340 · clusters: 0 (none)` · `allowlist-suppressed edges: 184`;
    `migrate status`: `sequences 1..84 (76 migrations)` · `sha256:58f165c8…` · `93 documents ·
    1243 rules`.
  - `cargo test --all`: 20 result lines, 765 passed · 0 failed summed.
    `MOCHIKO_FULL_SIMILAR=1 cargo test --test matrix_similar`: `48 passed; 0 failed … 237.04s`.
    `cargo fmt --all --check`: no output, rc 0. `cargo clippy --all-targets -- -D warnings`:
    `Finished`, rc 0.
  - `uv run evals/contract/run.py --list`: 99 declared cases. `--host-only`: `contract suite:
    7/7 cases passed` · `FILTERED — 7 of 99 declared cases. Not a gate run.`
  - `check-rubric implement`: `rubric OK: 71 observable, 54 out-of-instrument, 125 total`;
    `check-fixtures implement`: `fixture consistency: OK`.
  - Fixtures against `mochiko-cli template <n> [--check] --plugin-root plugins/mochiko | sed '$d'`:
    all 15 producers identical (`cmp`); the touched three hash `3a6a8f1e` (4,229 B), `82f093f5`
    (799 B), `f362df8a` (13,497 B), and the store's check view `3c64695d` stays identical — the
    plan's pins.
- **Crate scope — PASS.** `git diff HEAD -- crates/mochiko-cli/src crates/mochiko-cli/Cargo.toml
  Cargo.toml Cargo.lock scripts/similar-rules-allowlist.yaml` is empty, `git status` there too: no
  source, version, lock or allowlist row moved. `#[test]` counts hold (17 · 48 · 39 · 105 · 11,
  HEAD and now). Every edit is a pinned figure, a pinned list, its measured-why comment, or the
  one Q1 list.
- **Figures by tool — PASS.**
  - Census 93 docs · 401 command · 842 skill · 1,243 rules · 126 command floors · 39 fail nodes ·
    266 skill floors over 36 skills — `migrate status`, a kind count over the views, the floor
    walk of the skill views.
  - Kinds in the views: commands duty 41 · reservation 24 · binding 58 · fail 39 · no-kind 172;
    corpus duty 163 · reservation 67 · binding 178 · gate 31 · no-kind 585. The comments' pair
    deltas follow: command +539 to 19,010, corpus +1,997 to 213,340, matching validate.
  - Raw sweeps: `clusters: 77` · `allowlist: none (184 edges unsuppressed)` at `1..70` and
    `1..84`, sorted member lists identical; the review pair goes `[67] … best 0.67` to
    `[75] … best 0.61`, as the comment says. The floor-index pin `("implement", 1107, 1145)` holds
    by the passing `render.rs` test.
- **Fixtures — PASS.** Binary captures in the minus-footer form, byte-identical (pre-pass);
  `SHIPPED_TEMPLATES` is `[&str; 15]` with `design-stage`.
- **Re-key list honest to its test — PASS.** `fidelity.rs:566–570` asks for "a re-key here that
  names the rule"; `REANCHORED_SIDECAR_ANCHORS` (`:629–639`) names one rule, its migration and its
  new anchor, and the walk (`:656–661`) holds it to that anchor with `assert_eq!` — moved, not
  skipped; every other sidecar id stays on its genesis value, so the green walk itself proves only
  this anchor moved. Advisory: as with `RETIRED_SIDECAR_ANCHORS`, an entry naming a non-sidecar id
  would match nothing and fail nothing; the one entry is a sidecar id (T3 was red without it), so
  the doc comment's "or the walk below still fails" holds as written.
- **Contract expectations — PASS.** `Expected["implement"]` is 39 ids, set-equal to the render's
  `floors:` line (`class: floor · 39 rules`); the suite agrees ("the pre-registered floor set
  matches the implement render (39 ids)"); `44_266` stays the pre-registered baseline. Skill floor
  sets: 266 at HEAD and now, no set moved, `expected-skills.json` unchanged. `TEMPLATE_NAMES`
  gains `design-stage` (`deliverables`: "15 templates × 2 views"). `README.md:274` reads 126 and
  names both new floors under the implement-brainstorm-led ruling; the skill cell keeps 266.
- **The implement plan kit — PASS.**
  - `observable.yaml` places the implement view's 125 live ids exactly once (no duplicate, none
    not live; 5 tombstones and 6 section ids excluded): 71 observable · 54 out. The partition
    crossed with the buckets: unchanged 103 (54) · changed 11 (9) · removed 0 · added 11 (8), as
    the footer and the amendment say.
  - Each of the 11 mints carries a why its rule text and the fixtures bear out — `impl.desk-offer`'s
    `when: scope` includes `selection`; the agenda's why discloses the unplanted outside-need split;
    `impl.design-stage-inputs` is conditional on nothing handed in. All 16 new placements parse
    whole under YAML; the 18 pre-existing truncated whys are disclosed (plan Q5).
  - The 11 reworded ids keep their buckets; each why, read against its new text, names no dropped
    limb. The fail header reads "all 16" over 16 `impl.fail.*` entries. The five `0013` ids are
    each `when: {ux_bearing: present}` (Q2).
- **Preregistration and README line — PASS.** `:35` reads 71. The one `## Amendments` entry is
  true on every figure: HEAD's `observable.yaml` holds 63 · 46, so 58 was stale; 63 + 8 = 71,
  46 + 3 + 5 = 54, 125 in all; it is dated before any grid of this edit. Goldens, fixtures,
  `evals.json` and the wrapper show no diff, and `runs/` holds nothing newer than 2026-09-19 — no
  grid. `evals/plan/README.md:45` reads `implement 71 + 54`.
- **Preserved responsibilities — PASS.** Only superseded figures leave; every pinned assert
  survives with its new value, the clusters-0 and suppressed asserts unchanged.
  `mochiko-cli ids --check` over the ten text files: `1 bare · 0 drift` — `fidelity.rs:829` `D3`,
  present at HEAD `:805`, not this unit's.
- **Crate non-author review — PASS.** Test-only code; the new const mirrors
  `RETIRED_SIDECAR_ANCHORS`' shape (its migration element is documentation, as there); the
  `map_or` lookup keeps borrows plain; clippy clean. No assertion is weakened — each edit tightens
  to a measured figure or names the one expected move.
- **VERDICT: PASS**
- **Issues requiring fix:** none.

audit: unit 8 (crate tests, contract expectations, the implement plan kit · crate non-author review) · w3-gate-grader · opus · 13 files · 1 rounds · 0 blocking

## Unit 7b — the dependency-halt hook's prose-skill count and its strip entry

- **VALIDATE:** prose primitive · `plugins/mochiko/hooks/scripts/dependency-halt.sh` (`:76`, one
  comment word) · its strip entry `[v0.120.0]` in `.mochiko/strips/dependency-halt.md`
  (supersession by ruling, implement-brainstorm-led D2-iterative-becomes-engine). Lead-ruled
  widening of P2's write set (the build's Q-D item).
- **Checklist run:** coherence and preserved responsibilities
  (`validation-primitive-edit.judgment-items-prose`); the strip contract
  (`.mochiko/strips/README.md`, the supersession-by-ruling entry); the lead's first-hand list —
  the count, `bash -n`, file mode, behaviour unchanged, the entry's completeness.
- **Evidence read:** both files by `git diff HEAD`; the hook at `:66–95`; `:76` at `7d9d29e`
  (= HEAD); every `plugins/mochiko/skills/*/SKILL.md` grepped for `` !`mochiko-cli rules ``;
  `analysis-iterative/SKILL.md`'s `!` lines at `963a3d6^`, `963a3d6` and now;
  `evals/contract/README.md:371`; `.claude/rules/mochiko/primitive-edits.md:75`, `:242` and its
  `git diff HEAD`; record D2-iterative-becomes-engine (`:497`); `DECISIONS.md:13`;
  `0058-thinking-engine.yaml`; `strips/README.md:22–42`.
- **Pre-pass:** `migrate validate --report`: `0 rejecting · 116 advisory`; `migrate status`:
  `sha256:58f165c8… · 93 documents · 1243 rules`. `bash -n dependency-halt.sh`: rc 0. Mode:
  index `100755`, tree `-rwxr-xr-x`, `git diff --summary` empty. The count: seven `SKILL.md` lack
  the rules line — `grooming-operating-docs`, `mochiko` (the router), `patterns-api-contracts`,
  `patterns-entity-modeling`, `patterns-system-design`, `patterns-technical-decisions`,
  `testing-governance-injection` — six prose skills plus the router.
- **Coherence — PASS.** "the six prose skills and the router" now matches the count; "never had a
  rule set" stays true of the six (`analysis-iterative` left them: 0 `!` lines at `963a3d6^`, 7 at
  `963a3d6` and now). The contract README's "seven prose skills that carry no rules" lists the
  router among its seven — a count of primitives, consistent.
- **Behaviour unchanged — PASS.** The diff is one `#` line to one `#` line; the gate is still
  ``grep -q -F '!`mochiko-cli rules' "$primitive" … || exit 0`` at `:80`, keyed on the primitive's
  own file, never on the count. The unit-8 `--host-only` run (23:00, after the script's 22:51
  write) passed `hook-input` against this script.
- **Preserved responsibilities — PASS.** Only the count word leaves, and it has its entry; `:77–79`
  are byte for byte, no reflow, the line two characters shorter.
- **Strip entry — FAIL.** Newest-first, `[v0.120.0]` matching the wave's other stamps; Disposition
  names the rewrite; Tier failed cites D2-iterative-becomes-engine, `0058` and the `DECISIONS.md`
  row; Content is `:76` at `7d9d29e` verbatim; Kept deliberately is true. But *Consumers
  assessed* says `primitive-edits.md:75` and `:242` "carry the same stale "seven prose skills"
  count": the tree reads "the six prose skills" at both (`git diff HEAD`, the build's Q-C edit),
  so the entry records as open a defect its own landing closes.
- **VERDICT: FAIL**
- **Issues requiring fix:**
  1. `.mochiko/strips/dependency-halt.md`, `[v0.120.0]` *Consumers assessed*: reword the
     `primitive-edits.md:75`/`:242` clause to what the tree holds — both corrected from seven to
     six in this landing (Q-C, the lead's edit) — instead of "carry the same stale … count".

audit: unit 7b (prose primitive · dependency-halt.sh :76 count word + its v0.120.0 strip entry) · w3-gate-grader · opus · 2 files · 1 rounds · 1 blocking

### Unit 7b, round 2 — the Consumers-assessed clause fix, re-audited (same seat, resumed)

- **VALIDATE:** prose (strip record) · the `[v0.120.0]` entry's *Consumers assessed* clause in
  `.mochiko/strips/dependency-halt.md`.
- **Checklist run:** the strip contract's *Consumers assessed* field, scoped per
  `common.gate-loop-bound` to what the fix touched and what it could have broken.
- **Evidence read:** the entry now against round 1's text, rebuilt verbatim from round 1's
  `git diff HEAD` — only `:34–36` differ; `[v0.118.0]` and older, and the file header,
  byte-identical to HEAD; `.claude/rules/mochiko/primitive-edits.md:75` and `:242` now and by
  `git diff HEAD` ("seven" at both in HEAD); `build-log.md:326`, `:328`, `:1106`, `:1149` for the
  Q label.
- **Pre-pass:** `migrate validate --report`: `0 rejecting · 116 advisory`;
  `mochiko-cli ids --check .mochiko/strips/dependency-halt.md`: `0 bare · 0 drift`.
- **Consumers assessed — PASS.** "carried the same "seven prose skills" count, both corrected from
  seven to six in this landing by the lead's edit" is what the tree holds: `:75` reads "the six
  prose skills" and `:242` "The six prose skills". "(the build's Q-D item)" is the right label —
  `build-log.md:328` names Q-D as `primitive-edits.md:75`, `:241–242` and the hook comment, while
  Q-C (`:326`) is the declined agenda item. Round 1's "Q-C", in my block and the narrative, echoed
  the entry's old label; the narrative now reads Q-D.
- **Nothing else broken — PASS.** One clause rewritten at the file's wrap width; every other field
  byte-identical to round 1.
- **VERDICT: PASS**
- **Issues requiring fix:** none — round 1's issue 1 is closed.

audit: unit 7b (prose primitive · dependency-halt.sh :76 count word + its v0.120.0 strip entry) · w3-gate-grader · opus · 2 files · 2 rounds · 0 blocking

## Failure narrative

Unit 3, round 1 (cleared at round 2 by 0084): 0083 replaced the floor `impl.artifact-home`'s
"recorded on that entry only" — seams ruling R1, `.mochiko/decisions/2026-09-29-joint-hook-delta-build-seams.md`
"on that entry and nowhere else" — with "never in a ledger", which names `baseline-delta.md` and the
governance ledger, not a decisions record; R1's exclusion left the floor with no ruling behind it.
Read: 0037, the R1 record, the plan's Q3 text, the lead's Q3 answer, rev1 R1, the `decisions` home's
title. Fix: one word restored in a fix migration.

Unit 7b, round 1 (cleared at round 2): the `[v0.120.0]` strip entry is true on the hook edit, but
its *Consumers assessed* says `primitive-edits.md:75` and `:242` still carry "seven prose skills";
the tree reads "six" at both (the build's Q-D edit), so the record calls open a defect its own
landing closes. Read: both diffs, the rules file's two lines and its diff, the `SKILL.md` count.
Fix: reword that clause. Re-audit scope: that clause alone.

## Notes of note

- The HEAD baseline's validate needs `--plugin-root plugins/mochiko` beside `--log-dir`: with the
  scratch copy's own root every pointer reads unresolved (89 rejecting) — a scratch artifact.
- `mochiko-cli home` prints "carries its section budgets" for `design-stage.md`, whose template
  declares none (the lead's Q1, BACKLOG at landing); `check` enforces the 300-line bound.
- `impl.gate-design-checkpoint` now anchors to implement D8; D9 and
  `architecture-design-primitive` D8-rendered-diagram-signoff are named only in 0076's intent, so
  the unit-7 strip entries and the landing's `DECISIONS.md` annotations should name both, as the
  record's *Supersessions owed* asks.
- Outside W3, not a W3 defect: `ARCHITECTURE.md`'s implement section still says build-time
  `D-XXX` / `C-XXX` / `IP-XXX` writes "land as `baseline-delta.md` entries", false since the
  delta-files retirement (`impl.baselines-in-place-marked`); a BACKLOG candidate at the landing.
- Unit 8 ran the contract suite `--host-only` (`FILTERED — 7 of 99`, not a gate run): release
  gate 6 still needs the full set green at the `plugin.json` bump — SKIPPED is not green.
