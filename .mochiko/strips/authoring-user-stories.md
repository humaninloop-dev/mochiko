# Strip notes — `skills/authoring-user-stories/`

Entry formats: `strips/README.md`. Wave context: [v0.28.0] entries — skill-succinctness wave 4
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25);
[v0.23.0] entries — workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` D4 + the wave-2 rulings R1–R4/T1;
ratified 2026-07-24).

## [v0.101.0] Schema conversion — census-row → minted-ID map (skill-content-schema wave 2A)

Ruling for every [v0.101.0] entry: skill-content-schema D3 (three-home boundary) / D8/C4
(protected transfers), `DECISIONS.md` 2026-09-01 rows (skill-content-schema + the wave-2
family-door ruling); census: `.mochiko/brainstorms/skill-content-schema/census-authoring.md`
§B (AUS). Schema home: `plugins/mochiko/skills/authoring-user-stories/schema.yaml`. Minted
IDs carry the `authoring-user-stories.` prefix (omitted below). Map — census row →
minted ID:
1 `letter-is-spirit` (C-A1 stub) · 2 `independently-testable` · 3a `pm-frame-boundary` ·
3b `boundary-dispute-escalates` · 4a `technical-tasks-routing` · 4b `bug-reports-routing` ·
4c `already-story-form` · 4d `architecture-decisions-routing` · 4e
`api-contracts-routing` · 5a-i `deliverable-envelope` (C-A4 stub — the wave-lead F2
ruling: AUS takes the envelope stub, restoring the census ×5 membership) + 5a-ii
`story-structure` (lettered split; the member-specific story form + 2–5 cap, R6) · 5b
`density-fields` · 6 `priority-justified` · 7a `scenario-count-bound` · 7b
`scenarios-independently-verifiable` · 7c `observable-outcomes` · 8
`independent-test-required` · 9 `validation-script` (advisory binding, pointer
`scripts/validate-user-stories.py`, census J-3) · 10 `rationalization-stop`.
Build count 19 vs census 18: the approved 5a lettered split — disclosed for the audit.
Sections: independence, inputs, and output carry explicit empty markers (census fit —
AUS independence 0).
Accounting: body 5,351 → 3,775 + schema 7,893 = payload 11,668 (census estimate ~11,800);
the retained body is the load-first Rules block plus teaching prose (the T1 format block,
priorities + G/W/T intro with both reference pointers, Common Rationalizations table);
every removed line is a relocation or transfer recorded below — no content growth
claimed. Description byte-untouched at 425.

## [v0.101.0] T1-ruled story form — protection transfers (census AUS §A row 1; "untouchable")

- **Disposition:** superseded — the [v0.23.0] T1 ruling's obligations transfer per D8/C4:
  the 2–3 scenario cap with the compound-story rationale → `scenario-count-bound`
  (kind: bound) · the one-line scenario rule with its hiding-several-scenarios rationale +
  the journey ≤ 2 lines / one-line justification and independent-test densities →
  `density-fields`. The fenced format block itself stays prose in the body — the T1 form's
  **teaching rendering**, kept deliberately whole (census: "the fenced format block stays
  prose").
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01
  skill-content-schema row; original protection: 2026-07-24 token-reduction-wave-2 T1,
  user-ruled).
- **Content (verbatim, the load-bearing lines):** "Each story needs 2-3 acceptance
  scenarios — the happy path plus the key edge case(s); more than 3 means the story is
  compound or the scenarios overlap" · "Each scenario is **one line** — a scenario that
  needs a paragraph is hiding several scenarios or restating context the Given already
  carries" · "the journey ≤ 2 lines, the priority justification and independent test one
  line each".
- **Kept deliberately:** the format block fence verbatim in the body, its "— one line"
  field annotations included; `references/EXAMPLES.md` (the dense-form rewrite) untouched.
- **Consumers assessed:** `scripts/validate-user-stories.py` checks the authored artifact
  — its numbered-scenario + G/W/T regexes match the unchanged form (the [v0.23.0] check
  below stands); `review-specifications`' density-is-not-a-gap note unaffected.

## [v0.101.0] [v0.28.0] KEPT set — protection transfers (census AUS §A row 2)

- **Disposition:** superseded — protection transfers per D8/C4: letter/spirit epigraph →
  `letter-is-spirit` (C-A1 stub, floor) · the Independent Test bullets ("only home") →
  `independent-test-required`, the three bullets' content carried whole in the rule text
  (isolation method · data/setup · pass/fail) · the Quality Checklist → dedup-to-schema
  (lead F4 ruling), each row mirroring a minted rule, no extra rule minted — row → ID map:
  clear title + exact structure → `story-structure` · priority with justification →
  `priority-justified` · journey ≤ 2 lines → `density-fields` · independent test one line
  → `density-fields` + `independent-test-required` · 2-3 one-line G/W/T scenarios →
  `scenario-count-bound` + `density-fields` · no implementation details / technology
  references → `observable-outcomes` · observable, measurable outcomes →
  `observable-outcomes` + `independently-testable`.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row; original
  protection: 2026-07-25 skill-succinctness-strip).
- **Content (verbatim, the checklist):**

  ```
  - [ ] Has a clear, descriptive title
  - [ ] Priority is assigned with justification (one line)
  - [ ] User journey is described in plain language (≤ 2 lines)
  - [ ] Independent test is specified (one line)
  - [ ] 2-3 acceptance scenarios using Given/When/Then, one line each
  - [ ] No implementation details or technology references
  - [ ] Outcomes are observable and measurable
  ```

  And the Independent Test bullets: "How QA can verify this story in isolation · What
  data or setup is required · What constitutes passing/failing. This enables parallel
  testing and clear verification."
- **Kept deliberately:** the format block and the Common Rationalizations table stay in
  the body whole (teaching, census §B preamble).
- **Consumers assessed:** the [v0.28.0] wave-open enumeration stands (7 citing files, no
  section anchors); the router row unchanged.

## [v0.101.0] [v0.91.0] design-track carve-outs — protection transfers (census AUS §A row 3)

- **Disposition:** superseded — protection transfers per D8/C4, both carve-outs entire:
  `architecture-decisions-routing` + `api-contracts-routing`.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row; original
  protection: 2026-08-26 plan-stage-utility D1, kept entire at the [v0.91.0] entry below).
- **Content (verbatim):** "**Architecture decisions** - Capture technical-decision
  rationale in the design track instead; this skill authors user stories, not technical
  choices" · "**API contract design** - Define endpoints and schemas in the design track
  instead; this skill authors user stories, not interface contracts".
- **Consumers assessed:** `mochiko:authoring-requirements` keeps its vocabulary-aligned
  twin (P3's member this wave); nothing links these bullets by anchor.

## [v0.101.0] Remaining body obligations relocated to schema (D3 — grouped)

- **Disposition:** relocated → `plugins/mochiko/skills/authoring-user-stories/schema.yaml`,
  per the map entry above: the PM-frame boundary paragraph (3a/3b — the boundary floor and
  its escalates-to-the-user reservation) · the remaining When-NOT-to-Use bullets (4a–c) ·
  the format-intro obligations (5a-i/5a-ii/5b — envelope binding, 2–5 stories, exact
  structure, density fields) · the priority rule (6) · the scenario guidelines (7a/7b/7c) ·
  the Independent Test requirement (8) · the Validation Script section (9 — the command
  rides the rule text verbatim: `python scripts/validate-user-stories.py path/to/spec.md`) ·
  the overview's independently-testable line (2) · the Red-Flags STOP paragraph (10,
  floor, quoted verbatim in the rule text: "No exceptions: not for 'simple' features,
  'we'll refine later', 'tight deadlines', nor even if the user says 'just give me quick
  stories'").
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** faithfully compressed — each line's operative text survives in its mapped
  rule's `text`; pre-conversion body verbatim in git history of `SKILL.md` at v0.100.0.
- **Kept deliberately:** the format block, the priorities one-line enumeration with its
  `references/PRIORITY-DEFINITIONS.md` pointer, the G/W/T pattern intro with the
  `references/EXAMPLES.md` pointer, the Common Rationalizations table, and the overview
  identity voice stay in the body; `references/` and `scripts/` untouched.
- **Consumers assessed:** `requirements-analyst` mounts the skill (contract intact); the
  router row unchanged; no consumer links a removed section anchor.

## [v0.91.0] Two When-NOT-to-Use carve-outs: "the design/plan track" → "the design track" (advisory)

- **Disposition:** superseded → "the design track" at both sites.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1). Raised as an **advisory** by the
  v0.91.0 wave audit; this skill entered the wave's scope at the fix round, having carried no
  plan-stage vocabulary the earlier sweeps' terms matched — the pair reads as ordinary prose, so
  only the wider fix-round sweep surfaced it.
- **Content (superseded fragments, verbatim — two sites):**

  1. `- **Architecture decisions** - Capture technical-decision rationale in the design/plan track instead; this skill authors user stories, not technical choices`
  2. `- **API contract design** - Define endpoints and schemas in the design/plan track instead; this skill authors user stories, not interface contracts`

- **Kept deliberately:** both carve-outs entire — this skill authors **user stories, not
  technical choices** and **not interface contracts**; technical-decision rationale and
  endpoint/schema design both belong downstream. Only the downstream track's name lost its
  retired half.
- **Budget:** body **5,351** against the 6,702 budget; description unchanged at 425 against 532.
  Both inside.
- **Consumers assessed:** `mochiko:authoring-requirements` carried the identical phrasing at two
  sites and was re-keyed in the same round — the two spec-layer authoring skills stay
  vocabulary-aligned. Specify's FR/SC and story layers are otherwise untouched by this wave: D3
  explicitly declined to move the technical layer into specify.

## [v0.63.0] Guardrails body + slim description (guardrails-vs-detail benchmark verdict)
- **Disposition:** superseded → benchmark-ruled guardrails body + slim description
  (`.mochiko/benchmarks/guardrails-vs-detail/variants/body/authoring-user-stories/` and
  `variants/descriptions/authoring-user-stories/`; the shipped file is the deterministic merge
  of the two).
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark verdict,
  `DECISIONS.md` 2026-08-10 benchmark-verdict row; record
  `.mochiko/brainstorms/validator-scope-and-verbosity/record.md`, Benchmark execution;
  `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md` — body arm formal D6 PASS,
  descriptions arm 0 fire misses).
- **Content (faithfully compressed):** body 5,684 → 5,361 chars (−6%); description 562 → 425
  chars (−24%). Body cut: the **When to Use** section deleted whole (five bullets restating the
  description's invocation conditions — transforming descriptions into requirements, breaking
  down features, G/W/T structure, backlog items, defining "done"). Description cut: the SHOULD
  clause's trailing enumeration compressed (the trigger-phrase list and MUST/SHOULD grading kept;
  restated production detail dropped). Verbatim homes: git history of this file (pre-v0.63.0),
  the before/after pair under `variants/`, and archive branch
  `worktree-brainstorm-validator-scope`.
- **Kept deliberately:** the guardrails keep-set — the story format block + scenario rules
  (the [v0.23.0] T1-ruled form), Independent Test bullets, Quality Checklist, Common
  Rationalizations table, the letter/spirit epigraph, and all `references/` pointers
  (PRIORITY-DEFINITIONS, EXAMPLES). The [v0.28.0] kept-set below survives this cut in full —
  no prior KEPT or protected line is touched.
- **Consumers assessed:** 7 citing files per the wave-open enumeration below (requirements-analyst,
  specify, authoring-requirements, spec-template, artifact-format, mochiko router; authoring-slices
  retired v0.58.0); none links a section anchor; `scripts/validate-user-stories.py` checks the
  authored artifact, not the removed teaching bullets. Contract intact.

## [v0.28.0] Reference restatements, excuse-column red flags, and homed mistake rows stripped (body 179 → 116, −35%, in-band)
- **Disposition:** deduped → verified pre-existing homes, each Read before landing (nothing
  written to any reference this wave): the P1/P2/P3
  table → one-line enumeration + `references/PRIORITY-DEFINITIONS.md` pointer (richer:
  definitions, criteria, business signals, decision tree, distribution guidelines, its own
  mistakes table), the good/bad scenario example pair → `references/EXAMPLES.md` pointer (its
  Good-vs-Bad Comparisons section holds the same pair richer, plus journey / justification /
  independent-test pairs; pointer moved onto the scenario-rules section), the Given/When/Then
  definition bullets → folded into the pattern intro line, the script-checks bullets → a
  parenthetical on the intro line (command kept) · Red Flags trigger bullets + no-exceptions
  list → one STOP paragraph (the bullets map ~1:1 onto the kept Common Rationalizations Excuse
  column — "the user just wants quick stories" verbatim; batch-2 precedent) · **Common Mistakes
  deleted whole** (all 6 rows homed: technical-stories → When-NOT-to-Use + the format block's
  journey field; missing-justification → the format's one-line field + rationalizations rows
  1/6 + the script's justification check; implementation-details → scenario rule 4 + EXAMPLES'
  bad pairs; vague-outcomes → rule 4 + checklist; compound-stories → rule 1 ("more than 3 means
  the story is compound"); non-testable → rule 3 + checklist)
- **Tier failed:** 1 throughout (verified homes) · n/a for the fold-ins
- **Content:** one table, two fenced examples, three bullet lists, ten red-flag/no-exception
  bullets, six mistake subsections
- **Consumers assessed:** wave-open enumeration — 7 citing files (requirements-analyst, specify,
  authoring-requirements, authoring-slices, spec-template, artifact-format, mochiko router);
  none links a section anchor; `scripts/validate-user-stories.py` checks the authored artifact,
  not these teaching sections. Kept: the story format block + scenario rules ([v0.23.0] T1-ruled
  form, untouchable), Independent Test bullets (only home), Quality Checklist, Common
  Rationalizations table, the letter/spirit epigraph (R4b: anchored by the discipline paragraph
  below it). Session ruling: wave-4 batch-3 ratified 2026-07-25.

## [v0.23.0] Acceptance scenarios compressed to one line each, cap 2-4 → 2-3 (T1, user-ruled)
- **Disposition:** revised per the wave-2 T1 ruling (deleted prose replaced by the dense form; nothing relocated)
- **Tier failed:** artifact density (epic D4 extension): multi-line Given/When/Then prose re-paid ~10× per feature via mandated reads; the G/W/T grammar carries the testability, the line breaks carried nothing
- **Content:** the story-format block's multi-line scenario shape; the "2-4 scenarios" rule (now 2-3 with the compound-story rationale); the multi-line good example (now one line); quality-checklist counts. `references/EXAMPLES.md` rewritten to the dense form (3 examples: journeys ≤ 2 lines, one-line why/test/scenarios — same substance, same story content).
- **Consumers assessed:** spec.md producers (specify) + review-specifications (retargeted this wave: density-is-not-a-gap note) + `scripts/validate-user-stories.py` (checked: numbered-scenario + G/W/T keyword regexes match the one-line form — no script change needed).
