# Strip notes — `skills/review-governance-intent/`

Entry formats: `strips/README.md`. Wave context: skill-succinctness pilot wave — R1 live-defect
repair batch (design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified
2026-07-25). Description ledger — separate accounting from SKILL.md body lines per R2. Repair
context: R1 measurement (2026-07-25) proved delivery truncates at exactly 1,536 chars; the
shipped description was losing its TAIL — the bounded delta-pass SHOULD trigger and the
never-session-lead / FAIL-posture boundary clauses. The rewrite preserves all MUST/SHOULD
triggers, restores the tail under the cap, and is strict-YAML-safe. Delivery verification
deferred: the in-session skill listing is a session-start snapshot (probe received pre-edit
text), so the probe re-runs in a fresh session.

<!-- Wave context: wave 5 of the CLI schema-delivery build (v0.106.0) — the review family
converts: each member's rules are rendered at fire by `mochiko-cli` from the migration log the
plugin carries at `plugins/mochiko/migrations/`, and the skill reads no schema file. Ruling for
every [v0.106.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D3 as amended
(the skill-side form — `!` runs in `SKILL.md` and at subagent preload), D7 (the `PreToolUse`
`Skill` limb), D9 (families in the arc's order), and D10 clause 6 (the budgeted quantity re-keys
to body + rendered output), with the wave-open rulings in that session's `wave5-plan.md` and the
`DECISIONS.md` 2026-09-04 row. Pre-edit verbatim text:
`git show 7d098b9:plugins/mochiko/skills/review-governance-intent/SKILL.md`. -->

## [v0.106.0] the Rules block — raw schema Read superseded by CLI delivery

- **Disposition:** superseded → `## Rules — delivered by mochiko-cli`: the positive-confirmation
  halt clause plus seven `!` lines, one per rendered block (the preamble and the six sections),
  and the read-back sentence.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3 as amended, the
  skill-side form)
- **Content:** verbatim —

  ```
  ## Rules — load the schema first

  Your first action, before any protocol step: **Read `schema.yaml` (this skill's own
  directory) and `../../schemas/skill-review-common.yaml` raw, in full, in the same first
  action.** The schema is the source of truth for this skill's binding rules, nested in six
  sections, each addressable by its section ID:
  `review-governance-intent.sec.independence` · `review-governance-intent.sec.scope` ·
  `review-governance-intent.sec.inputs` · `review-governance-intent.sec.verdict` ·
  `review-governance-intent.sec.output` · `review-governance-intent.sec.reserved`. Interpret
  it live: a rule's `kind:` names what it is, and an absent `kind:` reads `constraint`; a
  rule carrying `when:` binds only where its terms hold against the schema's declared
  `conditions:`, except that a `class: floor` rule is always read and always delivered —
  `when:` gates when its obligation applies, never whether it reaches you; a `pointer:` rule
  binds you to that file's or skill's procedure, referenced never restated; `${var}`
  substitutes from this schema's `vars:` at read time; labels come from
  `plugins/mochiko/schemas/skill-labels.yaml`. A rule carrying
  `extends: review-common.<slug>` inherits text/labels/pointer from
  `skill-review-common.yaml` only — `class` and every absence-meaningful field are local —
  and the stub's `review-governance-intent.*` ID stays the citable ID. The floor pin:
  the 16 rules of `class: floor` are non-waivable. Before the first protocol step, state the floor
  count back — a skipped or partial read leaves that count blank: halt and surface it, and
  halt likewise if the schema's `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** everything outside this section, byte for byte — the title, the opening
  paragraph, and every procedural section. The block's substance survives in the render: the six
  section IDs are now the six `--section` arguments, and the interpretation grammar the block
  taught is printed as the preamble's `legend` with every fire. The `extends:` stub resolution
  and the family common-file co-Read are discharged by the render, which resolves every stub
  before the model sees it.
- **Consumers assessed:** the family common file
  `plugins/mochiko/schemas/skill-review-common.yaml` is unchanged and still bound by every
  unconverted consumer; nothing shared leaves. The block was this skill's own text.

## [v0.106.0] the hand-pinned `class: floor` count

- **Disposition:** superseded → the CLI-printed pin. The count is the `- class: floor · N rules`
  line under `pins` in the preamble block, and the `floors:` line beneath it lists the ids; the
  read-back sentence now cites both rather than carrying a number of its own.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3, "the counts are computed
  and printed by the CLI, never hand-pinned"; the wave-4 re-key ruling adding the `floors:` index)
- **Content:** verbatim —

  ```
  The floor pin:
  the 16 rules of `class: floor` are non-waivable. Before the first protocol step, state the floor
  count back — a skipped or partial read leaves that count blank: halt and surface it, and
  halt likewise if the schema's `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** the read-back obligation itself (skill-content-schema D6 as amended),
  re-keyed from a hand-pinned number to the printed pin and its id list — the skill still states
  the floor set back before its first procedural step, and a blank or partial read-back still
  halts. The independent second number the `.md` used to hold is booked as a loss, per D3's own
  rationale; the contract suite's `assert_floor_delivery` now carries what it used to check.
- **Consumers assessed:** `.claude/rules/mochiko/primitive-edits.md` skill-pair criterion 3, which
  demanded the hand-pinned count — amended in this same wave to branch on a converted skill.

## [v0.100.0] Schema conversion — census-row → minted-ID map (skill-content-schema wave)

Ruling for every entry below: skill-content-schema D3 (boundary) / D8/C4 (protected
transfers), `DECISIONS.md` 2026-09-01 row; census:
`.mochiko/brainstorms/skill-content-schema/census.md` §B (RGI). Schema home:
`plugins/mochiko/skills/review-governance-intent/schema.yaml`. Minted IDs carry the
`review-governance-intent.` prefix (omitted below). Map — census row → minted ID:
1 `frozen-window` · 2 `pre-ratification-timing` · 3 `never-a-participant` ·
4a `author-grader` (C3 stub) + 4b `verdict-is-input` (C4 stub) + 4c
`ratification-user-owned` (lettered splits — one row bound two common blocks and carried a
user-reservation tail; 4c minted so the tail survives as its own floor, deviation reported
to the wave lead) · 5a `lens-depth-never-jurisdiction` + 5b `report-out-of-lens-trips` ·
6 `solo-and-verify-routing` (both arms live in the text; no `when:` — extraction would
falsify, criterion-11 DECLARE latitude, reported) · 7 `authored-surfaces-out` ·
8 `formulation-quality-excluded` · 9 `sequestration` · 10a `read-set-binding` + 10b
`brownfield-analysis-read` (`when: {analysis: present}` — the brownfield leg split out so
the census-named dimension is genuinely used; reported) · 11 `finding-contract` ·
12 `unresolvable-is-commentary` · 13 `over-governance-admissibility` · 14 `never-excess`
(C6 stub) · 15 `cross-exam-binding` (`when: {pairing: pair}`, cross-dir pointer, census
J-7) · 16 `substrate-bindings` · 17 `reality-facts-checked` · 18 `user-facts-flagged` ·
19 `external-facts-binding` · 20 `survivor-report-form` · 21
`status-vocabulary-and-criteria` · 22a `default-fail` (C2 stub, `${verdict}` = `ready`;
the "never by looking reasonable" tail R2-absorbed by the block's "earned only by a
completed hunt", verbatim preserved below) + 22b `too-thin-first-finding` · 23a
`findings-through-leads-pen` (kept-distinct edge, RB+RGI) + 23b `evidence-floor` (C1
stub) · 24 `contested-audit-first` · 25 `echo-rationales-outrank` · 26
`declared-level-discipline` · 27a `yardstick-never-taste` + 27b
`no-in-session-confirmation` + 27c `its-command-states-them` (C5 stub).
Accounting (V1 fix round): body 5,562 → 2,662 (−2,900) + schema 12,001 = payload 14,663;
the delta over the pre-conversion body is structural overhead (IDs, keys, section
scaffolding, reading grammar) — no content growth claimed.

## [v0.100.0] Jurisdiction floor — protection transfers (census RGI-7; v0.63.0 keep-set)
- **Disposition:** superseded — protection transfers to schema rule `review-governance-intent.authored-surfaces-out` (class: floor), per skill-content-schema D8/C4; provenance sidecar carries the protected status.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 skill-content-schema row).
- **Content:** "**Permanently out of jurisdiction:** the authored surface set and its Tier-2 grading (`mochiko:validation-constitution`, downstream — different artifact, different family)"
- **Consumers assessed:** `mochiko:validation-constitution` named as the other family — untouched (P3's member).

## [v0.100.0] D1 exclusion — protection transfers (census RGI-8; v0.26.0 surviving element)
- **Disposition:** superseded — protection transfers to `review-governance-intent.formulation-quality-excluded` (class: floor), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "formulation/enforceability quality of authored principles (closed `Contested`, D1 — never re-raise)"
- **Consumers assessed:** none restate it (v0.89.0 map stands).

## [v0.100.0] Pair-protocol-by-reference + substrate bindings — protection transfers (census RGI-15/RGI-16; v0.26.0 surviving element)
- **Disposition:** superseded — protection transfers to `review-governance-intent.cross-exam-binding` (`when: {pairing: pair}`, cross-directory pointer per census J-7) and `review-governance-intent.substrate-bindings`, per D8/C4. `CROSS-EXAM.md` untouched — the single source, shared with `mochiko:review-brainstorm`. V1 fix round (RB-2 counterpart): the rule text KEEPS its pair/solo wording beside the `when:` — a DECLARE, not a double-home: the solo branch carries its own behavior ("solo skips to the report"), so extracting the guard would falsify the text (criterion-11 DECLARE latitude).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "cross-examination (pair only; solo skips to the report): after the lead introduces the counterpart, the one-shot four-message exchange per [../review-brainstorm/references/CROSS-EXAM.md] (the single source, shared with `mochiko:review-brainstorm`); bindings — *artifact*: the frozen synthesis; *fact substrate*: `codebase-analysis.md` + detect-stack baseline (brownfield; the files otherwise)"
- **Consumers assessed:** the shared file's charter ("An edit here changes both skills") — file untouched, both binders now schema rules.

## [v0.100.0] Verdict criteria incl. missing-declaration arm — protection transfers (census RGI-21; v0.63.0 keep-set + v0.65.0 arm)
- **Disposition:** superseded — protection transfers to `review-governance-intent.status-vocabulary-and-criteria`, per D8/C4; the v0.65.0 missing/unrecorded-depth-level-declaration arm carried verbatim in substance.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "a recommended status: `ready` (every hunt class actively worked, nothing blocking survived) / `needs-revision` (survivors resolvable by the session — an interrogation follow-up, a re-dealt card, a fixable fold) / `critical-gaps` (a fact profile contradicted by its own risk declaration or detected reality, an unrecorded ruling — a missing or unrecorded depth-level declaration among them — or a synthesis too thin to review)"
- **Kept deliberately:** the needs-revision examples ("an interrogation follow-up, a re-dealt card, a fixable fold") compress out of the rule text — teaching, not obligation; preserved verbatim here.
- **Consumers assessed:** none restate the criteria (v0.89.0 map stands).

## [v0.100.0] Never-default-ready + too-thin — protection transfers, lettered split (census RGI-22a/22b; v0.63.0 keep-set)
- **Disposition:** superseded — protection transfers to `review-governance-intent.default-fail` (22a, `extends: review-common.default-fail`, class: floor, `${verdict}` = `ready`) + `review-governance-intent.too-thin-first-finding` (22b, local floor), per D8/C4 and near-dup R2 (the "never by looking reasonable" tail absorbed by the block's completed-hunt wording).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "**Never default to `ready`** — earned by a completed hunt, never by looking reasonable; too thin to attack (rulings without rationale, marks without basis) is itself the first finding."
- **Consumers assessed:** five family members bind the same C2 block this wave.

## [v0.100.0] Lead's-pen + evidence floor — protection transfers, lettered split (census RGI-23a/23b; v0.63.0 keep-set + v0.64.0 floor line)
- **Disposition:** superseded — protection transfers to `review-governance-intent.findings-through-leads-pen` (23a, local floor — kept-distinct edge, RB+RGI only) + `review-governance-intent.evidence-floor` (23b, `extends: review-common.evidence-floor`, class: floor), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "findings enter through the lead's pen, dispositions in the Review section — verdict and dispositions land in the reviewed artifacts themselves; review evidence only in conversation is a floor violation"
- **Consumers assessed:** six family members bind the same C1 block this wave.

## [v0.100.0] Contested-shield audit — protection transfers (census RGI-24; v0.63.0 keep-set)
- **Disposition:** superseded — protection transfers to `review-governance-intent.contested-audit-first` (class: floor), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "marks are lead-self-reported: honor `Contested` only after its rationale audit (an unaudited `Contested` is a shield; a shield is a finding)"
- **Consumers assessed:** none (skill-local clause). The lead-self-reported opener rides RGI-25's rule.

## [v0.100.0] Echo-rationales rule — protection transfers (census RGI-25; v0.63.0 keep-set)
- **Disposition:** superseded — protection transfers to `review-governance-intent.echo-rationales-outrank` (class: floor), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "echo-rationales and adoption streaks outrank any mark"
- **Consumers assessed:** none (skill-local clause).

## [v0.100.0] Declared-level row — protection transfers (census RGI-26; v0.65.0 protected, complete)
- **Disposition:** superseded — protection transfers to `review-governance-intent.declared-level-discipline` (class: floor), per D8/C4; the D6 no-watcher fence preserved verbatim in the rule text per census §A.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "the declared depth level is the user's recorded ruling (D1/D2): challenge expression, waivers, fact consistency; verify it exists in the ledger, was recommend-then-arbitrated, greenfield got the low recommendation — never flag it against real users or deployment state (D6 no-watcher), never grade stricter than the declared level"
- **Consumers assessed:** none restate the row (v0.65.0 entry's grep stands).

## [v0.100.0] Yardstick + self-confirmation + its-command-states-them — protection transfers, lettered split (census RGI-27a/27b/27c; v0.63.0 keep-set + v0.46.0 clause)
- **Disposition:** superseded — protection transfers to `review-governance-intent.yardstick-never-taste` (27a, floor) + `review-governance-intent.no-in-session-confirmation` (27b, floor) + `review-governance-intent.its-command-states-them` (27c, `extends: review-common.its-command-states-them`, class: floor), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "yardstick = the agenda, the asserted floor, the synthesis's own internal consistency — never your governance taste · a session confirming its own synthesis is the gap this review closes; never soften on \"the user already confirmed it in session\" · your status is input; the lead owns the clearing verdict and survivor routing, the user owns ratification — its command states them."
- **Kept deliberately:** the status-is-input limb deduped into 4b's C4 stub (the body stated it twice); the ratification limb into 4c.
- **Consumers assessed:** RB and RF bind the same C5 block this wave.

## [v0.100.0] Frozen window relocated (census RGI-1)
- **Disposition:** relocated → `plugins/mochiko/skills/review-governance-intent/schema.yaml` `review-governance-intent.frozen-window`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "Frozen from your spawn until dispositions land."

## [v0.100.0] Pre-ratification timing relocated (census RGI-2)
- **Disposition:** relocated → schema.yaml `review-governance-intent.pre-ratification-timing`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "You run **before ratification**, spawned at the sizing gate"

## [v0.100.0] Never-a-participant floor relocated (census RGI-3)
- **Disposition:** relocated → schema.yaml `review-governance-intent.never-a-participant`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "never a participant — the session is recommend-then-arbitrate and unchallenged; **you are the challenge**" + Floors' "never in the session, out of its framing — the counterpart's included — until the cold read is done; in the session = not a reviewer"

## [v0.100.0] Never-author/revise/ratify relocated, lettered split (census RGI-4a/4b/4c)
- **Disposition:** relocated → schema.yaml `review-governance-intent.author-grader` (4a, C3 stub, floor) + `review-governance-intent.verdict-is-input` (4b, C4 stub, floor) + `review-governance-intent.ratification-user-owned` (4c, local floor — the user-reservation tail the two blocks do not carry; mint reported to the wave lead).
- **Tier failed:** n/a — supersession by ruling (D3/D5; `DECISIONS.md` 2026-09-01 row).
- **Content:** "You recommend; the **lead owns every verdict, the user owns ratification** — never author, revise, or ratify."

## [v0.100.0] Lens depth + out-of-lens trips relocated, lettered split (census RGI-5a/5b)
- **Disposition:** relocated → schema.yaml `review-governance-intent.lens-depth-never-jurisdiction` (5a) + `review-governance-intent.report-out-of-lens-trips` (5b). The two lens definitions stay body prose (teaching) per D3.
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "— depth, never jurisdiction: report real out-of-lens trips"

## [v0.100.0] Solo/verify routing relocated (census RGI-6)
- **Disposition:** relocated → schema.yaml `review-governance-intent.solo-and-verify-routing`. No `when:` — both arms live in the text; extracting the pair/solo guard would falsify it (criterion-11 DECLARE latitude; census annotation resolved this way, reported).
- **Tier failed:** n/a — supersession by ruling (D3/D4; `DECISIONS.md` 2026-09-01 row).
- **Content:** "solo, the whole surface is yours. The verify pass belongs to the coherence lens in a pair, to you automatically when solo."

## [v0.100.0] Sequestration relocated (census RGI-9)
- **Disposition:** relocated → schema.yaml `review-governance-intent.sequestration`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "sequestration — the entire attack formed before counterpart contact; the lead withholds the name until findings are formed"

## [v0.100.0] Read set relocated, lettered split (census RGI-10a/10b)
- **Disposition:** relocated → schema.yaml `review-governance-intent.read-set-binding` (10a, agenda pointer `../authoring-constitution/references/INTERROGATION-AGENDA.md`, census J-7) + `review-governance-intent.brownfield-analysis-read` (10b, `when: {analysis: present}` — the census-named brownfield dimension declared and used; split reported).
- **Tier failed:** n/a — supersession by ruling (D3/D4; `DECISIONS.md` 2026-09-01 row).
- **Content:** "read the frozen synthesis, the agenda ([../authoring-constitution/references/INTERROGATION-AGENDA.md] — its ten dimensions are the coverage yardstick), and, brownfield, `.mochiko/memory/codebase-analysis.md`"

## [v0.100.0] Finding contract relocated (census RGI-11)
- **Disposition:** relocated → schema.yaml `review-governance-intent.finding-contract`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "every finding: severity, the GI element(s) touched, a **concrete failure scenario or cited contradiction**, and a resolution path (the one question or check that settles it…)"

## [v0.100.0] Unresolvable-is-commentary relocated (census RGI-12)
- **Disposition:** relocated → schema.yaml `review-governance-intent.unresolvable-is-commentary`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "unresolvable = commentary, not a finding"

## [v0.100.0] Over-governance admissibility relocated (census RGI-13)
- **Disposition:** relocated → schema.yaml `review-governance-intent.over-governance-admissibility`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "the over-governance hunt is in-jurisdiction (remove-shaped): a minted principle or intent no elicited fact justifies is a finding like any absence — admissible only naming the fact it fails to trace to or the lighter surface carrying the real need; … never reaches formulation quality"

## [v0.100.0] Never-excess carve relocated to C6 stub (census RGI-14)
- **Disposition:** relocated → schema.yaml `review-governance-intent.never-excess` (`extends: review-common.never-excess`, class: must)
- **Tier failed:** n/a — supersession by ruling (D3/D5; `DECISIONS.md` 2026-09-01 row).
- **Content:** "a floor-, compliance-module-, or NFR-derived obligation is never excess"

## [v0.100.0] Three fact routes relocated (census RGI-17/18/19)
- **Disposition:** relocated → schema.yaml `review-governance-intent.reality-facts-checked` (17) + `review-governance-intent.user-facts-flagged` (18) + `review-governance-intent.external-facts-binding` (19, cross-dir pointer, census J-7).
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "a **reality-surface fact** is checked against the analysis or files, never argued · a **user-declared fact** (team size, risk posture, lifespan, values) is checkable against nothing on disk — flag for the lead to route to the user as confirmation, never to argument · an **external-sourced fact** (a floor-class claim fed from outside the repo) runs per [../review-brainstorm/references/EXTERNAL-CLAIMS.md], never argued."

## [v0.100.0] Survivor report form relocated (census RGI-20)
- **Disposition:** relocated → schema.yaml `review-governance-intent.survivor-report-form`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "**Survivor report** — a message to the lead, no report files: your own survivors, severity-classified, each with failure scenario, resolution path, unresolved counterpart objections, flagged counterpart duplicates; a tally (\"N raised, M survived\" — the fallen stay retrievable on ask)"

## [v0.89.0] User-ruled true-deletion body cut — body 8,150 → 5,562 chars (−31.8%)

- **Disposition:** superseded → a single-file five-paragraph body (identity+contract ·
  Lens+jurisdiction · Protocol · Survivor report · Floors) — true deletion, no relocation,
  no new file; the `description:` (483 chars) and the three shared reference pointers
  (INTERROGATION-AGENDA.md · CROSS-EXAM.md · EXTERNAL-CLAIMS.md) untouched. Every
  behavioral rule of the baseline body survives as a compressed clause. The floor is
  shallow by structure, disclosed at the gate: the v0.63.0 benchmark wave already
  strip-cut this skill −46%, so the baseline IS the keep-set plus the v0.65.0
  adaptive-depth ruling — this pass deletes the remaining rationale prose (Overview
  essays, the cost-asymmetry framing) and the section/table forms. The user ruled **ship
  the rule-complete cut**; deeper (~−55%) declined with the deaths named (verdict-criteria,
  fact-route, and declared-level rules degrade to labels; four mistake-row floors die).
- **Tier failed:** n/a — supersession by ruling (in-session user ruling 2026-08-26 at the
  `compressing-skills` ratification gate, on the v0.82.0–v0.88.0 "cut now, eval validates
  later" precedent; ADR
  `.mochiko/decisions/2026-08-26-review-governance-intent-true-deletion-cut.md`;
  `DECISIONS.md` 2026-08-26 row). Evidence per the ceremony: the 70-entry rule inventory
  `evals/review-governance-intent/rules.json` (non-compressor-authored; 3 restorations
  pre-gate: R-005 traceable-contract, R-025 lead-introduces + four-message, R-031
  floor-class qualifier) and the disposition map in
  `evals/review-governance-intent/pass-report.md`.
- **Disposition map (baseline section → new home; verbatim home: git history pre-v0.89.0):**
  - *Overview ¶1–3* → the opening paragraph: artifact definition (GI-IDs, the five
    lead-assigned marks, traceable contract), before-ratification timing, sizing-gate
    spawn, never-a-participant, you-are-the-challenge, frozen window, recommend-only
    (lead owns verdicts, user owns ratification, never author/revise/ratify). The
    gap-cost-asymmetry rationale dies.
  - *Overview lens ¶ + jurisdiction ¶* → Lens + jurisdiction: both lens scopes verbatim-in-
    substance, depth-never-jurisdiction, out-of-lens reporting, solo whole-surface,
    verify-pass assignment, both permanent exclusions (surface set + Tier-2 downstream;
    formulation quality D1).
  - *Independent cold read* → Protocol legs 1–3: sequestration with the withheld name, the
    three reads with the ten-dimension yardstick pointer, the four-element finding contract
    + unresolvable-is-commentary, the over-governance hunt with its full calibration.
  - *Cross-examination* → Protocol leg 4: pair-only/solo-skips, after-the-lead-introduces,
    one-shot four-message, CROSS-EXAM.md single-source pointer, the three substrate
    bindings, the three fact routes with their three dispositions.
  - *Survivor report + verdict table* → the Survivor report paragraph: message-no-files,
    report fields, tally + fallen-retrievable, all three verdict criteria (incl. the
    v0.65.0 missing-declaration critical-gaps arm), never-default-ready,
    too-thin-is-the-first-finding.
  - *Independence (4 bullets) + Common Mistakes (9 rows)* → Floors: never-in-session/
    own-session-disqualifies, lead's-pen + review-evidence floor line (wording superseded,
    substance intact), Contested-shield rationale audit, marks-self-reported +
    echo-rationales/streaks-outrank, the full declared-level row (D1/D2 · three-way
    verification · D6 no-watcher · never-grade-stricter), yardstick-never-taste,
    session-confirms-itself, status-is-input + its-command-states-them (v0.46.0 clause).
    Rows restating protocol content (user-declared route, resolution paths) ride those
    clauses.
- **MANDATORY KEPT reconciliation:** [v0.26.0] KEPT whole-body — its two elements
  surviving the v0.63.0 ruling (pair-protocol-by-reference with substrate bindings; D1
  exclusion) survive here compressed, no rule deleted. [v0.63.0] guardrails keep-set —
  every member's obligation survives per the map; forms end by this ruling. [v0.65.0]
  declared-level row + critical-gaps arm — survive complete. [v0.46.0]
  its-command-states-them — survives.
- **Consumers assessed:** `agents/devils-advocate.md` (mount — intact) · router `:44`
  (cold intent reviewer / frozen synthesis / coverage-coherence pair or solo / survivors +
  tally + recommended status — all survive) and `:141` · `commands/setup.md` (dispatch
  mechanics live in the command; coverage-survivor and ratified-before-authoring
  vocabulary survives body-side) · `CROSS-EXAM.md:4` + `EXTERNAL-CLAIMS.md:94` (shared
  single sources — pointers survive, files untouched). No dead pointers created.

## [v0.65.0] Adaptive-depth two-row form — level DECLARATION becomes reviewable; missing declaration is a critical gap
- **Disposition:** superseded → the Common-Mistakes row flips from "never review the level" to "review the level DECLARATION (exists / recorded / recommend-then-arbitrated / greenfield-got-low), never the level-vs-reality" (D6 no-watcher); strictness-beyond-the-declared-level stays non-negotiable
- **Tier failed:** n/a — supersession by ruling (production-floor-adaptive-depth, `DECISIONS.md` 2026-08-11 row; record `.mochiko/brainstorms/production-floor-adaptive-depth/record.md`, D1 / D2 / D6; PO-D2 amended, PO-D7 superseded)
- **Content (superseded line, verbatim old → new).** SKILL.md ~:103, Common Mistakes table. Protected lineage: descends from the `[v0.26.0] KEPT` "tier-consistency vs tier-choice" survivor row (PO-D2-reworded to "the floor's *level*"), and sits inside the `[v0.63.0]` guardrails Common-Mistakes keep-set:
  - OLD: `| Re-litigating the floor's *level* | The level is the library's, asserted (PO-D2) — challenge expression, waivers, and fact consistency, never the level itself. |`
  - NEW: `| Arguing the declared level is wrong for the project | The declared level is the user's recorded ruling (D1/D2) — challenge its expression, waivers, and fact consistency, and verify it exists in the ledger, was recommend-then-arbitrated, and greenfield got the low recommendation (D2). Never flag it against real users or deployment state (D6 no-watcher), and never grade a check stricter than the declared level sets. |`
- **Added (pure addition — rides the decision row, no supersession):** SKILL.md ~:79 critical-gaps verdict criterion gains the missing/unrecorded depth-level declaration case ("(a missing or unrecorded depth-level declaration among them)").
- **D6 fence honored:** the new row makes the DECLARATION reviewable — its existence, its ledger record, recommend-then-arbitrate protocol fidelity, and greenfield low-recommendation fidelity (all `/mochiko:setup` process facts, per ruling keyed to greenfield/brownfield mode) — while explicitly forbidding any level-vs-reality flag ("(D6 no-watcher)") and preserving per-check strictness as non-negotiable ("never grade a check stricter than the declared level sets"). No watcher of any kind added.
- **Rationale (PO-D2 → D2).** Under PO-D2 the level was the library's single asserted row, hence off-limits to challenge; under D1/D2 the level is a user-declared, ledger-recorded ruling elicited by a recommend-then-arbitrate protocol, so the DECLARATION's existence and protocol fidelity become reviewable — its correctness for the project does not.
- **Body budget:** 7,273 → 7,592 chars (budget 8,862). Description untouched (483).
- **Kept deliberately:** the v0.63.0 guardrails keep-set intact, incl. the rest of the Common Mistakes table; the row's positive jurisdiction (expression, waivers, fact consistency) is preserved and extended, never dropped.
- **Consumers assessed:** no command references this skill (grep `plugins/mochiko/commands/` clean). `agents/devils-advocate.md` declares it in `skills:`; the extended row leaves that composition intact.

## [v0.63.0] Guardrails cut — body deletions + slim description (benchmark verdict)
- **Disposition:** superseded → benchmark-ruled guardrails body + slim description (`.mochiko/benchmarks/guardrails-vs-detail/variants/`)
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark verdict — `DECISIONS.md` 2026-08-10 benchmark-verdict row; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`)
- **Content (faithfully compressed).** Body 13,374 → 7,273 chars (−6,101, −46%; this figure is net of the +~185-char review-evidence floor line added as a pure addition, so the deletion alone is larger). Description 1,499 → 483 chars. Sections removed or shortened:
  - **## Phase 0 — Blind angle map** (removed whole) — the topic-only blind angle-map pre-read, its free-repo-grounding rules, and the augments-hunt-class-1 governance (v0.60.0 cold-review gap-challenge addition).
  - **Phase 1 five-hunt-classes table** (removed) — the missed-dimension / unchallenged-fact-profile / passive-card-acceptance / too-easily-resolved-reality-conflict / thin-rationale-echo-hunt table, the sweep-remaining-elements paragraph (waivers · modules · minted intents · exclusions), and the "Use the marks to prioritize, never to skip" doctrine paragraph. Heading kept, renamed **## Independent cold read**; its sequestration and finding-shape prose kept.
  - **Coverage-findings paragraph** (removed) — the beyond-agenda blind-map-diff finding class and its materiality bar (v0.60.0 addition).
  - **## The verify pass — and the post-review-edit delta-pass** (removed whole) — the verify-pass fold-check, the bounded post-review-edit delta-pass, and reopen-born-intents handling.
  - **Phase framing dropped** — Phase 1/2/3 headings renamed to plain **## Independent cold read** / **## Cross-examination** / **## Survivor report**; Phase 3's cross-set-merge / survivor-routing detail trimmed to the survivor-report essentials.
  - Old description verbatim: "This skill MUST be invoked when serving as a cold INTENT REVIEWER in a `/mochiko:setup` run — stress-testing the frozen, confidence-marked interrogation synthesis (`.mochiko/memory/governance-intent.md`) BEFORE the user ratifies it at setup's synthesis-ratification checkpoint — spawned at the sizing gate (one of a coverage/coherence lens-briefed pair by default, or solo when sized down), never a participant in the interrogation session. Protocol — independent cold read FIRST; the five setup hunt classes (missed dimensions against the ten-dimension agenda, unchallenged fact-profile calls, passive card acceptances, too-easily-resolved reality conflicts, thin-rationale echo hunts); reality-grounding against `codebase-analysis.md` in brownfield. Then CROSS-EXAMINE the counterpart per the single-sourced pair protocol (`review-brainstorm`'s `references/CROSS-EXAM.md`) and return survivors severity-classified (Critical/Important/Minor) with a tally and a RECOMMENDED status (ready / needs-revision / critical-gaps) — survivor routing, the clearing verdict, and ratification are the lead's and the user's. SHOULD also invoke for the verify pass over the synthesis's folded dispositions (the coherence lens in a pair, automatic when solo) or the bounded delta-pass on a material post-review edit. Run by an independent reviewer, never the session lead; defaults to a FAIL posture — zero findings means hunt harder, and every finding needs a concrete failure scenario or cited contradiction."
  - Verbatim removed text survives in three places: (a) git history of the original `plugins/mochiko/skills/review-governance-intent/SKILL.md`; (b) the before/after pair in this tree — `.mochiko/benchmarks/guardrails-vs-detail/variants/body/review-governance-intent/SKILL.md` (after) and the pre-edit original (before, in git); (c) archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately (the guardrails keep-set):** goal/output contract (Overview + the frozen-synthesis contract); the permanent out-of-jurisdiction floor (authored surface set + the D1 formulation-quality exclusion); the pair-protocol-by-reference with skill-specific substrate bindings (the reality-surface vs user-declared vs external-sourced fact-authority split; the `references/CROSS-EXAM.md` and `EXTERNAL-CLAIMS.md` pointers); the verdict table (ready / needs-revision / critical-gaps) with "never default to `ready`"; the Common Mistakes table; the FAIL-posture floor. **Added (pure addition):** the review-evidence floor line in ## Independence.
- **Protected-content reconciliation.** The `[v0.26.0] KEPT: the entire remaining body` survivor ruling named five elements as kept; this guardrails cut REMOVES three of them, recorded here as superseded-by-this-ruling — never silently dropped:
  - "the five setup hunt classes" (the Phase-1 hunt-class table) — REMOVED. Superseded.
  - "the marks-prioritize-never-skip doctrine" — REMOVED. Superseded.
  - "the G3-edit delta-pass" (now the post-review-edit delta-pass) — REMOVED. Superseded.
  The other two v0.26.0-KEPT elements survive: the pair-protocol-by-reference with substrate bindings, and the D1 jurisdiction exclusion; the verdict-table criteria also survive. The v0.60.0 cold-review gap-challenge additions removed here (blind angle map, coverage findings, verify/delta/reopen passes) are DECISIONS-traceable (`4e2f1b3`, `baf67d7`) and are likewise superseded by this same benchmark ruling. The prior `[v0.24.0]`/`[v0.46.0]` DESCRIPTION entries concern the earlier description; the slim description supersedes that earlier description in full.
- **Consumers assessed:** no command references this skill (grep `plugins/mochiko/commands/` clean). `agents/devils-advocate.md` declares it in `skills:`; the kept goal/contract/floor/`references` pointers leave that composition intact.

## [v0.46.0] loop-discipline pointer reworded
- **Disposition:** superseded → "its command states them"
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** "— see `loop-discipline`; this skill does not restate them." → "— its command states them; this skill does not restate them."
- **Consumers assessed:** setup command briefs unchanged.

## [v0.26.0] Three pair-protocol / in-file-restatement Common-Mistakes rows deleted (body 138 → 135, −2.2%)
- **Disposition:** deleted as pure restatements of verified single-sourced homes — the
  counterpart-contact row restates this file's own Phase-1 sequestration line + the shared
  `review-brainstorm/references/CROSS-EXAM.md` withholding rule; the tally-merging row restates
  Phase 3's "the cross-set merge and the combined count are the lead's" + CROSS-EXAM's
  flagged-not-merged standard; the lens-dropping row restates the Overview lens paragraph. All
  homes Read and confirmed before landing (mirror of the same-day `review-brainstorm` strip).
  Wave context: skill-succinctness wave 2 (review-\* cluster), batch-3 proposal ratified
  2026-07-25 — user directed continuation on the recommended dispositions
- **Tier failed:** 1
- **Content:** the three table rows; the nine setup-specific rows (mark-audit, `Contested`
  rationale audit, tier-consistency vs tier-choice, user-declared facts, D1 jurisdiction,
  governance-taste, resolution paths, session-confirmation softening, own-session grading) kept
- **Consumers assessed:** wave-open enumeration — 5 citing files, none reference the rows

## [v0.26.0] KEPT: the entire remaining body (whole-skill survivor ruling, 2.2% vs the 30–70 band)
- **Tier-2 evidence:** contested as a whole at the under-band pass and kept — authored
  post-doctrine at altitude: the pair protocol runs by reference with skill-specific substrate
  bindings (the reality-surface vs user-declared fact-authority split is unique load-bearing
  content); the five setup hunt classes, the marks-prioritize-never-skip doctrine, the G3-edit
  delta-pass, and the D1 jurisdiction exclusion each name their failure mode; the verdict-table
  criteria are setup-specific. The description is the fresh v0.24.0 repair (1,500 chars, ≤1,536) —
  untouched, boundary clauses intact. Sixth whole-skill survivor of the pass. Session ruling:
  batch-3, 2026-07-25.

## [v0.24.0] DESCRIPTION: cut 1,778 → 1,500 chars (delivery cap measured at exactly 1,536)
- **Disposition:** deleted (description ledger)
- **Tier failed:** 2 — every dropped clause is doctrine restated from the SKILL.md body's hunt-class protocol; no trigger or boundary behavior lost
- **Content:** dropped clauses — the confidence-marks-are-lead-self-reported / echo-rationales-stay-independent rationale clause; the `Contested`-mark-is-no-unaudited-shield guard (body-owned); the "interrogation" qualifier on the ten-dimension agenda; the long solo-sizing phrasing ("or solo when the user sized the review down" → "or solo when sized down"); "before any contact with the counterpart reviewer" (substance preserved by "independent cold read FIRST")
- **Consumers assessed:** delivery-side only — no file consumes description text
