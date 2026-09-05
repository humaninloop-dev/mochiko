# Strip notes — `skills/review-brainstorm/`

Entry formats: `strips/README.md`. Wave context: skill-succinctness pilot wave — R1 live-defect
repair batch (design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified
2026-07-25). Description ledger — separate accounting from SKILL.md body lines per R2. Repair
context: R1 measurement (2026-07-25) proved delivery truncates descriptions at exactly 1,536
chars; this description was shipping with its TAIL silently cut — the negative boundary +
FAIL-posture clauses never reached any session. The rewrite preserves all MUST/SHOULD triggers,
restores the boundary clauses under the cap, and is strict-YAML-safe. Delivery verification
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
`git show 7d098b9:plugins/mochiko/skills/review-brainstorm/SKILL.md`. -->

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
  sections, each addressable by its section ID: `review-brainstorm.sec.independence` ·
  `review-brainstorm.sec.scope` · `review-brainstorm.sec.inputs` ·
  `review-brainstorm.sec.verdict` · `review-brainstorm.sec.output` ·
  `review-brainstorm.sec.reserved`. Interpret it live: a rule's `kind:` names what it is, and
  an absent `kind:` reads `constraint`; a rule carrying `when:` binds only where its terms
  hold against the schema's declared `conditions:`, except that a `class: floor` rule is
  always read and always delivered — `when:` gates when its obligation applies, never whether
  it reaches you; a `pointer:` rule binds you to that file's or skill's procedure, referenced
  never restated; labels come from `plugins/mochiko/schemas/skill-labels.yaml`. A rule
  carrying `extends: review-common.<slug>` inherits text/labels/pointer from
  `skill-review-common.yaml` only — `class` and every absence-meaningful field are local —
  and the stub's `review-brainstorm.*` ID stays the citable ID. The floor pin:
  the 9 rules of `class: floor` are non-waivable. Before the first protocol step, state the floor count
  back — a skipped or partial read leaves that count blank: halt and surface it, and halt
  likewise if the schema's `class: floor` count disagrees with the pin.
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
  the 9 rules of `class: floor` are non-waivable. Before the first protocol step, state the floor count
  back — a skipped or partial read leaves that count blank: halt and surface it, and halt
  likewise if the schema's `class: floor` count disagrees with the pin.
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
transfers), `DECISIONS.md` 2026-09-01 row; census: `.mochiko/brainstorms/skill-content-schema/census.md`
§B (RB). Schema home: `plugins/mochiko/skills/review-brainstorm/schema.yaml`. All minted IDs
carry the `review-brainstorm.` prefix (omitted below). Map — census row → minted ID:
1 `never-in-the-room` · 2 `lens-depth-never-jurisdiction` · 3 `verdict-is-input` (C4 stub;
body's twice-stated form deduped here) · 4 `blind-map-before-record-contact` ·
5 `grounding-excludes-session-artifacts` · 6 `cold-read-before-counterpart` ·
7 `hunt-classes-per-decision` · 8a `excess-names-cheaper-shape` + 8b `never-excess` (C6 stub;
lettered split — one row, two obligations) · 9 `verify-load-bearing-claims` ·
10 `map-sample-audit` · 11 `external-claims-binding` · 12 `record-fitness-binding` ·
13 `coverage-severity-mapping` · 14 `dismissed-angle-is-ruling` ·
15 `findings-formed-count-only` · 16 `cross-exam-binding` · 17 `survivor-report-form` ·
18 `status-vocabulary-and-criteria` · 19 `unresolvable-is-commentary` ·
20 `verify-pass-grade` · 21 `synthesis-fidelity-sample` · 22 `reopen-born-verify-grade` ·
23a `author-grader` (C3 stub) + 23b `findings-through-leads-pen` (lettered split) ·
24 `evidence-floor` (C1 stub) · 25 `contested-needs-new-angle` · 26a `never-default-ready`
(LOCAL rule by lead ruling — no C2 stub; keep-distinct edge vs `review-common.default-fail`
reported for the allowlist) + 26b `unverifiable-claim-is-finding` · C5 mint (no census §B
row; §A v0.46.0 + census §C) `its-command-states-them`.
Accounting (V1 fix round): pre-conversion body 2,748; relocated content measures 3,265
across the pair — the +517 content growth is grammatical expansion only (compressed body
fragments rewritten as standalone, referentially-closed rule sentences per
command-content-schema D12/D15), no restored playbook prose — the argued-overage path per
skill-content-schema D8/C1; the remainder of the pair's payload over the body figure is
structural overhead (IDs, keys, section scaffolding, reading grammar).

## [v0.100.0] Blind-map floor — protection transfers (census RB-4; v0.60.0 + v0.88.0 protected)
- **Disposition:** superseded — protection transfers to schema rule `review-brainstorm.blind-map-before-record-contact` (class: floor), per skill-content-schema D8/C4; provenance sidecar carries the protected status.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 skill-content-schema row).
- **Content:** "Phase 0 blind angle map from the topic only, produced as its own deliverable before record contact"
- **Kept deliberately:** the Protocol's sequencing prose (map first, then cold read) stays in the body per D3.
- **Consumers assessed:** the dispatching command's two-message blind-map dispatch is command-side, unaffected.

## [v0.100.0] Coverage severity + materiality — protection transfers (census RB-13; v0.60.0 protected)
- **Disposition:** superseded — protection transfers to `review-brainstorm.coverage-severity-mapping`, per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "coverage findings: blind-map-vs-record diff + materiality, severity by whether a ruling would likely / plausibly / not have changed (Critical / Important / Minor)"
- **Kept deliberately:** the diff+materiality clause survives in `survivor-report-form`'s coverage parenthetical.
- **Consumers assessed:** none restate the severity test (census sweep).

## [v0.100.0] Dismissed-angle rule — protection transfers (census RB-14; v0.60.0 protected)
- **Disposition:** superseded — protection transfers to `review-brainstorm.dismissed-angle-is-ruling`, per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "a dismissed angle is a ruling, not a gap"
- **Consumers assessed:** none (skill-local clause).

## [v0.100.0] Reopen-born grade — protection transfers (census RB-22; v0.60.0 protected)
- **Disposition:** superseded — protection transfers to `review-brainstorm.reopen-born-verify-grade`, per D8/C4. "this grade" deixis resolved to "the verify-pass grade" (command-content-schema D15).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "Reopen-born decisions get this grade, never a fresh cold read."
- **Consumers assessed:** `commands/brainstorm.md` "reopen-born verify" vocabulary — survives in the rule ID and text.

## [v0.100.0] Tally form — protection transfers (census RB-17; v0.88.0 RETURNED)
- **Disposition:** superseded — protection transfers to `review-brainstorm.survivor-report-form`, per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "survivor report as a message: survivors with severity, the decision(s) touched, failure scenario or cited contradiction (coverage: the diff + materiality), resolution path; tally (\"N raised, M survived\")"
- **Consumers assessed:** `review-governance-intent` carries its own tally clause (kept-distinct, 2-member — below the common bar).

## [v0.100.0] `critical-gaps` criteria — protection transfers (census RB-18; v0.88.0 RETURNED)
- **Disposition:** superseded — protection transfers to `review-brainstorm.status-vocabulary-and-criteria`, per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "recommended status (`ready / needs-revision / critical-gaps` — critical-gaps: a broken load-bearing claim, an unowned decision, a Critical coverage gap, or a record too thin to review)"
- **Consumers assessed:** none restate the criteria (census sweep).

## [v0.100.0] Commentary clause — protection transfers (census RB-19; v0.88.0 RETURNED)
- **Disposition:** superseded — protection transfers to `review-brainstorm.unresolvable-is-commentary`, per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "A finding nothing could resolve is commentary, not a finding."
- **Consumers assessed:** `review-governance-intent` carries its own copy (kept-distinct, 2-member).

## [v0.100.0] Class-6 calibration — protection transfers, lettered split (census RB-8a/8b; v0.67.0 protected)
- **Disposition:** superseded — protection transfers to `review-brainstorm.excess-names-cheaper-shape` (8a, local) + `review-brainstorm.never-excess` (8b, `extends: review-common.never-excess` — the carve's family block), per D8/C4 and near-dup R2.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "excess machinery (name the cheaper shape; floor/compliance/NFR never excess)"
- **Kept deliberately:** the six-class list stays body prose per census §B note; the class-6 name rides it.
- **Consumers assessed:** the carve wording now single-homes in `skill-review-common.yaml`; RGI/RSPEC/VC/RF bind the same block.

## [v0.100.0] EXTERNAL-CLAIMS binding — protection transfers (census RB-11; v0.52.0 carve-out protected)
- **Disposition:** superseded — protection transfers to `review-brainstorm.external-claims-binding` (pointer: `references/EXTERNAL-CLAIMS.md`), per D8/C4. The reference file is untouched (D3/C2 — stub points, never duplicates).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "outside-repo claims per `references/EXTERNAL-CLAIMS.md` (owned here)"
- **Consumers assessed:** nine consumers named in the reference's Single-source header — file untouched, pointers hold.

## [v0.100.0] CROSS-EXAM binding — protection transfers (census RB-16; v0.52.0-adjacent protected)
- **Disposition:** superseded — protection transfers to `review-brainstorm.cross-exam-binding` (`when: {pairing: pair}`, pointer: `references/CROSS-EXAM.md`), per D8/C4. File untouched. V1 fix round (RB-2): the "pair only" text limb DROPPED from the rule text — the guard is single-homed in `when:` (criterion-11 MOVE); the verbatim clause is the Content line below.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "cross-examination per `references/CROSS-EXAM.md` (pair only; fact substrate: the record's fact-checker map; fact authority: the seated fact-checker, else the files)"
- **Consumers assessed:** `review-governance-intent` shares the file (its own binding rule, same wave).

## [v0.100.0] Evidence floor — protection transfers to C1 stub (census RB-24; v0.64.0 protected)
- **Disposition:** superseded — protection transfers to `review-brainstorm.evidence-floor` (`extends: review-common.evidence-floor`, class: floor), per D8/C4; strongest wording now single-homed in `skill-review-common.yaml`.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "verdict and dispositions left in the reviewed artifacts themselves, never only in conversation"
- **Consumers assessed:** six other family members bind the same block this wave.

## [v0.100.0] its-command-states-them — protection transfers to C5 stub (§A v0.46.0; no census §B row — mint reported)
- **Disposition:** superseded — protection transfers to `review-brainstorm.its-command-states-them` (`extends: review-common.its-command-states-them`), per D8/C4. Census §B carried no RB row for this clause; minted per §A + §C, deviation reported to the wave lead, never silent.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 row).
- **Content:** "— its command states them."
- **Consumers assessed:** RGI and RF bind the same block this wave.

## [v0.100.0] Never-in-the-room floor relocated (census RB-1)
- **Disposition:** relocated → `plugins/mochiko/skills/review-brainstorm/schema.yaml` `review-brainstorm.never-in-the-room` (class: floor)
- **Tier failed:** n/a — supersession by ruling (skill-content-schema D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "paired or solo, **never in the room**"

## [v0.100.0] Lens depth-never-jurisdiction relocated (census RB-2)
- **Disposition:** relocated → schema.yaml `review-brainstorm.lens-depth-never-jurisdiction`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "A lens brief (*decision-quality*: scenario stress, classes 1–4, steelmans · *record-integrity*: inconsistencies, fitness, the map audit) sets depth, never jurisdiction."

## [v0.100.0] Verdict-is-input relocated to C4 stub (census RB-3; body stated it twice)
- **Disposition:** relocated → schema.yaml `review-brainstorm.verdict-is-input` (`extends: review-common.verdict-is-input`, class: floor)
- **Tier failed:** n/a — supersession by ruling (D3/D5; `DECISIONS.md` 2026-09-01 row).
- **Content:** "You recommend; **the lead owns every verdict**." + "your status is input; the lead owns the clearing verdict" (one obligation, deduped at conversion per census).

## [v0.100.0] Grounding fence relocated (census RB-5)
- **Disposition:** relocated → schema.yaml `review-brainstorm.grounding-excludes-session-artifacts`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "(free repo grounding, session artifacts excluded)"

## [v0.100.0] Cold-read-before-counterpart relocated (census RB-6)
- **Disposition:** relocated → schema.yaml `review-brainstorm.cold-read-before-counterpart`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "cold read before counterpart contact"

## [v0.100.0] Hunt duty relocated; class list stays prose (census RB-7)
- **Disposition:** relocated → schema.yaml `review-brainstorm.hunt-classes-per-decision`; the six-class list stays in the body Protocol per census §B note.
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "scenario stress + six hunt classes per decision"

## [v0.100.0] Verify-claims duty relocated (census RB-9)
- **Disposition:** relocated → schema.yaml `review-brainstorm.verify-load-bearing-claims`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "verify load-bearing claims against the fact-checker map or the files"

## [v0.100.0] Integrity-lens map audit relocated with `when:` (census RB-10)
- **Disposition:** relocated → schema.yaml `review-brainstorm.map-sample-audit` (`when: {lens: record-integrity}` — guard moved out of the text, criterion-11 MOVE)
- **Tier failed:** n/a — supersession by ruling (D3/D4; `DECISIONS.md` 2026-09-01 row).
- **Content:** "(record-integrity lens: sample-audit the map itself against the files)"

## [v0.100.0] Record-fitness binding relocated (census RB-12)
- **Disposition:** relocated → schema.yaml `review-brainstorm.record-fitness-binding` (pointer: `references/RECORD-FITNESS.md`; file untouched)
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "fitness per `references/RECORD-FITNESS.md`"

## [v0.100.0] Findings-formed count-only relocated (census RB-15)
- **Disposition:** relocated → schema.yaml `review-brainstorm.findings-formed-count-only`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "findings-formed = count only"

## [v0.100.0] Verify-pass grade relocated with `when:` (census RB-20)
- **Disposition:** relocated → schema.yaml `review-brainstorm.verify-pass-grade` (`when: {pass: verify}`)
- **Tier failed:** n/a — supersession by ruling (D3/D4; `DECISIONS.md` 2026-09-01 row).
- **Content:** "**Verify pass** (record-integrity lens, or solo): grade each fold against the updated record, quoting evidence; new surface only for fold-introduced contradictions."

## [v0.100.0] Fidelity sample relocated (census RB-21)
- **Disposition:** relocated → schema.yaml `review-brainstorm.synthesis-fidelity-sample`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "Fidelity-sample a requested `synthesis.md` — every ruling present, no confidence mark inflated, no rejected alternative resurrected."

## [v0.100.0] Author≠grader + lead's-pen relocated, lettered split (census RB-23a/23b)
- **Disposition:** relocated → schema.yaml `review-brainstorm.author-grader` (23a, `extends: review-common.author-grader`, class: floor) + `review-brainstorm.findings-through-leads-pen` (23b, local floor — the lead's-pen limb is a kept-distinct edge, RB+RGI only, below the common bar).
- **Tier failed:** n/a — supersession by ruling (D3/D5; `DECISIONS.md` 2026-09-01 row).
- **Content:** "never author or revise the record — findings enter through the lead's pen"

## [v0.100.0] Contested-new-angle floor relocated (census RB-25)
- **Disposition:** relocated → schema.yaml `review-brainstorm.contested-needs-new-angle`
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 row).
- **Content:** "never raise a `Contested` decision unless the angle is genuinely new to the ruling"

## [v0.100.0] Default-FAIL floor kept LOCAL + unverifiable-claim floor relocated (census RB-26a/26b)
- **Disposition:** relocated → schema.yaml `review-brainstorm.never-default-ready` (26a — LOCAL full wording by lead ruling: the "zero findings means hunt harder, never manufacture" tail is protected lineage the C2 common block deliberately excludes; keep-distinct allowlist edge vs `review-common.default-fail` reported at fan-in) + `review-brainstorm.unverifiable-claim-is-finding` (26b).
- **Tier failed:** n/a — supersession by ruling (D3/D5 + near-dup R2/R6; `DECISIONS.md` 2026-09-01 row).
- **Content:** "never default to `ready` — zero findings means hunt harder, never manufacture · an unverifiable claim is a finding"

## [v0.88.0] RETURNED: blind map as its own deliverable (floor) + the commentary clause — eval-demanded re-adds, body 2,497 → 2,599; verify round added the tally form + critical-gaps criteria, final body 2,747 (−76.6% vs pre-cut)
- **Evidence:** the post-cut regression grid `evals/review-brainstorm/runs/20260826-110222/`
  (27 runs, 3 arms; verdict in `evals/review-brainstorm/pass-report.md`). The cut arm failed
  the ratified bar on one root cause: 5 of 9 cut runs produced **no blind-map artifact**
  (pre-cut produced `phase0-blind-map.md` in 9/9) — "Return the map to the lead as a message"
  had been deleted as prose but was a behavioral floor (R-001), taking three map-quality
  rules (R-010/R-011/R-012) and the "Phase 0 angle map" consumer vocabulary (R-059/R-060)
  with it. R-035 (a finding nothing could resolve is commentary, not a finding) was lost
  independently. Re-added: the Protocol first leg now reads "Phase 0 blind angle map from
  the topic only, produced as its own deliverable before record contact"; the survivor-report
  leg gains the commentary sentence. User ruled "re-add + verify" 2026-08-26; the verify
  re-run (9 fresh cut-arm sessions appended to the same grid) came back **floor-clean: 9/9
  runs produced the blind-map deliverable** (was 4/9) and the R-001 map cluster fully
  recovered. Three residual losses surfaced: the tally's "N raised, M survived" two-number
  form (R-060 consumer vocab, real — cut runs reported survivor counts only), the
  `critical-gaps` criteria (R-049 — died with the verdict table, dispositioned at
  ratification but consumer-graded), and a 1-of-9 replicate miss on the freshly re-added
  commentary clause (R-035, pass^k single-replicate kill, flake-adjacent). The first two
  were re-added under the same eval-demanded ruling pattern (+148 chars: tally `"N raised,
  M survived"`; `critical-gaps` = broken load-bearing claim · unowned decision · Critical
  coverage gap · record too thin) — **these two micro re-adds ship unverified** (no third
  arm-run; the resume command is the standing lever). Budget: 2,747 rides inside the seeded
  3,122 cap — headroom absorbs re-adds by design, no re-seed. Per D7 the version bump
  carrying these re-adds triggers the validator audit on this primitive; version
  roll-forward is owned by the parallel landing session holding the counter (tree at
  0.93.0 mid-flight when this shipped).

## [v0.83.0] User-ruled true-deletion body cut — body 11,754 → 2,497 chars (−78.8%)

- **Disposition:** superseded → a single-file floors-and-dispatch body (true deletion — no
  relocation, no new reference file; a drafted breakup into a `references/REVIEW-PROTOCOL.md`
  was rejected by the user mid-pass as verbosity-shifting, not verbosity-reduction). Every
  behavioral rule of the old body survives as a compressed line; all rationale prose, worked
  framing, and the two tables' long-form text are deleted. The user targeted −90% and ruled
  −83.3% at the ratification gate with the trade named: the last ~800 chars to −90.0% required
  deleting ruled machinery outright (reopen-born verify grading, synthesis fidelity sample,
  class-6 calibration clause, coverage materiality gate, blind-map grounding fence, cross-exam
  substrate binding) — declined; every one of those rules is kept in compressed form.
  Post-gate repairs kept the every-rule-survives claim honest: the integrity-lens sample-audit
  clause restored first (+71 chars), then the author≠grader audit (FAIL round 1, 7 blocking)
  forced back the lens taxonomy definitions, the coverage-severity test clause, the full
  cross-exam substrate binding (fact authority), the decision(s)-touched contract field, the
  fidelity-sample criteria, and the v0.46.0 its-command-states-them clause (+461 chars).
  Ratified draft 1,965 → landed 2,497.
  `description:` untouched (490 chars). All three `references/` files untouched.
- **Tier failed:** n/a — supersession by ruling (in-session user ruling 2026-08-26 at the
  `compressing-skills` ratification gate, on the `review-feasibility` v0.82.0 "cut now, eval
  validates later" precedent; ADR
  `.mochiko/decisions/2026-08-26-review-brainstorm-true-deletion-cut.md`; `DECISIONS.md`
  2026-08-26 row). Pass artifacts: `evals/review-brainstorm/pass-report.md`.
- **Disposition map (old body section → new home; verbatim home for all removed text: git
  history of this SKILL.md, pre-v0.83.0):**
  - *Overview ¶1 (record/confidence marks, pair-vs-solo sizing, counterpart purpose, solo
    bar)* — deleted; the cold/paired-or-solo/never-in-the-room identity survives in the
    opening line, the solo-bar and sizing detail dies (the dispatching command sizes the
    review).
  - *Overview ¶2 (lens split)* — compressed: the two lens definitions (*decision-quality* /
    *record-integrity*) and depth-never-jurisdiction survive as one parenthetical clause
    (taxonomy audit-restored after the ratified draft deleted it while both gate uses
    remained).
  - *Overview ¶3 (lead owns verdicts)* — compressed to the opening line + a Floors clause.
  - *Phase 0 — blind angle map (v0.60.0 PROTECTED)* — compressed to the Protocol chain's first
    leg: blind map from the topic only, before record contact, free repo grounding, session
    artifacts excluded. Anti-anchoring rationale and pair-both-build prose deleted (the
    dispatching command's two-message blind-map dispatch carries the mechanics).
  - *Phase 1 — sequestration, scenario stress, six-class table (v0.67.0 class-6 row)* —
    compressed to the second leg: cold-read-before-counterpart-contact + the six class names,
    class 2 keeping its intra-decision qualifier and class 6 keeping its calibration clause
    ("name the cheaper shape; floor/compliance/NFR never excess"). Long-form class questions
    deleted.
  - *Phase 1 — reality-grounding + EXTERNAL-CLAIMS ownership* — compressed: fact-checker-map
    substrate, files fallback, the integrity-lens sample-audit rule, and the ownership claim
    ("owned here") all survive as clauses; the failed-sample-is-a-finding-against-everyone's-
    substrate consequence dies as prose.
  - *Phase 1 — finding contract, `Contested` rule, coverage findings + severity table + two
    gates + findings-formed (v0.60.0 PROTECTED)* — compressed: contract fields, `Contested`
    new-angle exception, diff + materiality, dismissed-angle-is-a-ruling, and
    findings-formed-count-only all survive as clauses; the Critical/Important/Minor severity
    test table survives compressed to one clause (severity by whether a ruling would likely /
    plausibly / not have changed — audit-restored after the ratified draft dropped it).
  - *Phase 2 — cross-exam binding* — compressed to one leg: `references/CROSS-EXAM.md`, pair
    only, fact substrate and fact authority named.
  - *Phase 3 — survivor report + verdict table + never-default-`ready`* — compressed: message
    form, per-survivor fields, tally, the three status words, and the never-default-`ready` /
    hunt-harder / never-manufacture floor all survive.
  - *The verify pass + synthesis fidelity sample + reopen-born decisions (v0.60.0
    PROTECTED)* — compressed to the Verify-pass paragraph: fold-grading with quoted evidence,
    fold-introduced-contradictions-only, fidelity sample, reopen-born-never-a-fresh-cold-read.
    The one-level recursion stop dies as prose (implied by never-a-fresh-cold-read).
  - *Independence (4 bullets incl. the v0.64.0 floor line)* — compressed into Floors; the
    v0.64.0 line's verbatim form is superseded by its compressed form ("verdict and
    dispositions left in the reviewed artifacts themselves, never only in conversation") —
    substance intact, wording superseded by this same ruling.
  - *Common Mistakes (5 rows)* — deleted; each row's rule already lives in the Protocol chain
    or Floors (resolution paths, map-checking, `Contested`, cold-spawn independence,
    never-grade-own-session via never-in-the-room).
- **MANDATORY KEPT reconciliation:** [v0.26.0] KEPT (entire remaining body) — **superseded by
  this ruling**, the whole-body survivor status ends here. [v0.60.0] protected machinery —
  survives compressed (blind map · coverage diff/materiality/dismissed-angle · reopen-born
  verify), no rule deleted. [v0.67.0] class-6 row — survives compressed with its calibration
  clause. [v0.64.0] floor line — survives compressed (wording superseded, substance intact).
  [v0.52.0] carve-out — untouched (`CROSS-EXAM.md`/`EXTERNAL-CLAIMS.md` not edited).
  [v0.46.0] its-command-states-them pointer clause — survives compressed in the Floors close
  ("— its command states them"; audit-restored after the ratified draft dropped it).
- **Consumers assessed:** `agents/devils-advocate.md` (declares the skill; description
  untouched, routing intact) · `agents/validator.md` (points at
  `references/EXTERNAL-CLAIMS.md` — untouched) · `review-governance-intent` (shares
  `CROSS-EXAM.md` — untouched; its own protocol unaffected) · `commands/brainstorm.md`
  ("Phase 0 angle map" wording: the body no longer numbers phases, but "blind angle map"
  survives verbatim and the two-message dispatch mechanics live in the command itself;
  "coverage-survivor" / "reopen-born verify" vocabulary survives in the body) ·
  grep for body-vocabulary quotes elsewhere: all external "hunt class" hits are
  `review-feasibility`'s class 7. No dead pointers created.

## [v0.67.0] Sixth hunt class (excess machinery / unpaid decision) added — three "five hunt classes" counts re-keyed
- **Disposition:** superseded → the excess posture from the architect-role ruling: a sixth, remove-shaped hunt class is added, so the three "five hunt classes" counts become "six".
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/architect-role-pushback-and-abstraction/record.md`, D3 as amended by its F3 calibration clause; DECISIONS.md combined-wave build row).
- **Content (superseded, verbatim — count reconciliations):**
  - Phase 1 item 2 — OLD: "2. **The five hunt classes**, per decision:" → "2. **The six hunt classes**, per decision:".
  - Coverage-findings paragraph — OLD: "a first-class finding beside the five hunt classes above." → "...beside the six hunt classes above.".
  - Description — OLD: "independent cold read, the five hunt classes, then cross-examination" → "...the six hunt classes...".
- **Kept deliberately:** hunt classes 1–5 (Unchallenged assumption, Missing dimension, Passive acceptance, Rejected-road steelman, Inconsistency) untouched; the coverage-findings class and the Phase 0 blind-map machinery untouched. Pure addition riding the decision row (no strip): the class-6 table row carrying the calibration clause in one breath.
- **Consumers assessed:** no reference file or command carries the "five hunt classes" count (grep confirmed the three sites are all inside this SKILL.md). Description stays under the 1,536 delivery cap (490 chars) and within budget (614).

## [v0.64.0] Guardrails Wave 2 — slim description + review-evidence floor line (no body deletions)
- **Disposition:** superseded → the guardrails-vs-detail Wave 2 editorial cut (D4 cut line). Description slimmed; body carries no deletion — only the sanctioned floor-line pure addition below.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md` 2026-08-11 build row [its Wave 2 residual authorization] + user rulings 2026-08-10/11; method warrant: benchmark verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md` — guardrails held across all four skill natures).
- **Content (faithfully compressed).** Description 1,506 → 491 chars (−67%). Body 11,326 → 11,508 chars (+182, +2% — the floor-line addition; **zero body deletions**). Description cut: the full protocol enumeration (blind-map-diff mechanics, the five named hunt classes spelled out, reality-grounding-via-fact-checker-map, the CROSS-EXAM owner-withdrawal detail, the severity buckets, the three verdict-state names, the SHOULD-trigger production detail) compressed. Kept in the slim description: the MUST cold-END-STAGE-REVIEWER trigger, `record.md`, the blind-angle-map-yielding-coverage-findings phrase (routing to the v0.60.0 protected machinery), the five-hunt-classes phrase, cross-examination, the paired/solo + never-in-the-room independence triggers, the SHOULD triggers (verify pass · synthesis fidelity sample · one-shot cold review), and the never-a-co-author / defaults-to-FAIL posture.
  - **Old description (verbatim):** "This skill MUST be invoked when serving as a cold END-STAGE REVIEWER of a collaborative thinking session's decision record (`record.md`) — spawned at convergence (one of a lens-briefed pair by default, or solo when the user sized the review down), never in the room during the session. Protocol — a blind angle map (topic alone) whose diff yields coverage findings for never-visited angles; then independent cold read; scenario stress per decision; the five hunt classes (unchallenged assumptions, missing dimensions, passive acceptances, steelman-able rejected alternatives, inconsistencies); reality-grounding of load-bearing claims via the record's fact-checker map (no map → the files directly); the standalone-record fitness checklist. Then CROSS-EXAMINE the counterpart per the one-shot protocol (`references/CROSS-EXAM.md` — owner-withdrawal only; fact disputes route to the fact-checker, never to argument) and return survivors severity-classified (Critical/Important/Minor) with a tally and a RECOMMENDED status (ready / needs-revision / critical-gaps) — the cross-set merge and clearing verdict are lead-owned. SHOULD also invoke for the verify pass over a record's folded resolutions, the fidelity sample of a requested synthesis, or a one-shot cold review of a decision record outside a live team. Run by an independent reviewer, never a session co-author; defaults to a FAIL posture — zero findings means hunt harder, and every finding needs a concrete failure scenario or cited contradiction."
  - Verbatim homes for the removed description text: git history of this SKILL.md (pre-v0.64.0); archive branch `worktree-brainstorm-validator-scope`.
- **Floor line added (pure addition, cross-cutting finding 1 / F-X1 mitigation):** "The independent review leaves its verdict and per-finding dispositions in the reviewed artifacts themselves — review evidence that lives only in conversation is a floor violation." Placed in `## Independence`, as a new bullet after the "Findings enter the record through the lead's pen, with dispositions" bullet — mirroring the Wave-1 `review-governance-intent` placement (which carries the same line alongside its lead's-pen bullet). Rides the same decision row.
- **Kept deliberately (body — the entire body survives):** no body prose was deleted. The [v0.26.0] KEPT whole-body survivor ruling is honored intact — every phase, table, and pointer stands. The v0.60.0 DECISIONS-traceable machinery (Phase 0 blind angle map, the coverage-findings map-vs-record diff, the reopen-born-decision verify grading) is protected, ruled content and survives whole. The five-hunt-class table, the two verdict tables, the CROSS-EXAM/EXTERNAL-CLAIMS/RECORD-FITNESS pointers, and the Common Mistakes table are untouched.
- **MANDATORY KEPT reconciliation:** grep of this strip for `KEPT`/protected/DECISIONS-traceable entries — [v0.26.0] KEPT (entire remaining body) and [v0.60.0] supersession (hunt-class-2 re-key) and [v0.52.0] (CROSS-EXAM carve-out). This cut removes **no** body line, so no prior KEPT or protected content is touched. The v0.60.0 blind-map/coverage/reopen content (RULED, protected) fully survives — verified present after the edit.
- **Consumers assessed:** commands `plugins/mochiko/commands/` — grep clean (the brainstorm command orchestrates it by dispatch, not by name in body). Agents — `plugins/mochiko/agents/devils-advocate.md` declares it in `skills:` and carries a when-to-reach bullet ("cold end-stage review of a thinking session's `record.md`"); the slim description preserves that cold-review role and every routing trigger, so the composition is intact. `plugins/mochiko/agents/validator.md` points at `skills/review-brainstorm/references/EXTERNAL-CLAIMS.md` — a `references/` file, untouched by this cut. `review-governance-intent` shares `references/CROSS-EXAM.md` — substrate-agnostic, untouched. Contract intact.

## [v0.60.0] Hunt class 2 narrowed to intra-decision scope; topic-level coverage moves to the new coverage class
- **Disposition:** superseded → the new "Coverage findings — the map-vs-record diff" class (topic-level never-visited dimensions) + a re-keyed class 2 (intra-decision missing factors)
- **Tier failed:** n/a — supersession by ruling (record `.mochiko/brainstorms/cold-review-gap-challenge/record.md` D10; DECISIONS.md 2026-08-10 row)
- **Content:** `| 2 | **Missing dimension** | What angle (cost, failure mode, actor, timescale) was never visited? |`
- **Kept deliberately:** the intra-decision reading — a decision that never weighed cost/failure-mode/actor/timescale is a real miss the map-diff will not catch, so class 2 survives re-keyed rather than deleted (keep-both-unbounded and full-supersession both rejected in D10).
- **Consumers assessed:** `review-governance-intent` keeps its own hunt class 1 (agenda-diff) — a different class, unaffected; the shared `CROSS-EXAM.md` is substrate-agnostic and carries no class text. No other consumer references class 2 text.

## [v0.52.0] CROSS-EXAM.md fact-dispute rule gains an external-claim carve-out
- **Disposition:** superseded → `references/EXTERNAL-CLAIMS.md` (external-claim disputes only;
  every other fact dispute keeps routing to the session's fact authority exactly as shipped)
- **Tier failed:** n/a — supersession by ruling (DECISIONS.md 2026-08-04 external-research row,
  ER-D4 as amended at review fold F1; record
  `.mochiko/brainstorms/external-research-in-review/record.md`)
- **Content:** the fact-dispute bullet's totalizing reading — "route it to the session's fact
  authority… one route per fact" as the *only* route, with no path for a fact that authority
  holds no jurisdiction over (the fact-checker never fetches; an external-claim dispute
  dead-ended).
- **Kept deliberately:** the fact-dispute bullet's own text verbatim; the four-message exchange
  and all other attack/defense standards untouched. The carve-out lands as an added bullet, not
  a rewrite — CROSS-EXAM.md remains the single pair-protocol home, delegating exactly one
  dispute class by pointer.
- **Consumers assessed:** `mochiko:review-brainstorm` and `mochiko:review-governance-intent`
  (the file's charter: "An edit here changes both skills") — neither restates the fact-dispute
  rule locally; both pick up the carve-out through the shared file. Both also gained their own
  EXTERNAL-CLAIMS.md binders in the same v0.52.0 build, so the routes agree.

## [v0.46.0] loop-discipline pointer reworded
- **Disposition:** superseded → "its command states them"
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** "— see `loop-discipline`; this skill does not restate them." → "— its command states them; this skill does not restate them."
- **Consumers assessed:** brainstorm command briefs unchanged.

## [v0.26.0] Six pair-protocol / in-file-restatement Common-Mistakes rows deleted (body 78 → 72, −7.7%)
- **Disposition:** deleted as pure restatements of verified single-sourced homes: three rows
  (consensus-seeking, withdrawal-under-pressure, fact re-routing) restate `references/CROSS-EXAM.md`'s
  Attack-and-defense standards near-verbatim — the file Phase 2 mandates loading at exactly the
  temptation moment; three rows (counterpart-contact, tally-merging, lens-dropping) restate this
  file's own Phase-1 sequestration line, Phase-3 tally line, and Overview lens paragraph
  (26–60 lines up). All homes Read and confirmed before landing. Wave context: skill-succinctness
  wave 2 (review-\* cluster), batch-3 proposal ratified 2026-07-25 — user directed continuation
  on the recommended dispositions
- **Tier failed:** 1 (distinct from the R4b aphorism ruling, which protected copies with **no**
  home — these had two verified homes each)
- **Content:** the six table rows; the five skill-specific rows (resolution paths, map-trusting,
  `Contested`-raising, softening, own-session grading) kept
- **Consumers assessed:** wave-open enumeration — 6 citing files, none reference the rows.
  Shared-home audit this wave: `references/CROSS-EXAM.md` (4 consumers) is a clean single source —
  no dead pointers, no duplication-only content, untouched; `references/RECORD-FITNESS.md` is
  single-consumer but at correct altitude (checklist detail — inlining would add body lines), no action

## [v0.26.0] KEPT: the entire remaining body (whole-skill survivor ruling, 7.7% vs the 30–70 band)
- **Tier-2 evidence:** contested as a whole at the under-band pass and kept — authored
  post-doctrine at altitude: the pair protocol is run by reference (never restated), every
  Overview and Phase paragraph names its failure mode (framing-inheritance, captured-by-the-room,
  unverifiable-claim-is-a-finding), the hunt-class and verdict tables are the skill's core unique
  content. The sentences shared with `review-governance-intent` ("lens sets depth, not
  jurisdiction"; "every finding carries…"; "findings-formed — count only") are KEPT by design:
  CROSS-EXAM's header assigns per-skill substrate bindings, no canonical home exists for them,
  and pointer economics are negative at 1–3 lines each. Fifth whole-skill survivor of the pass.
  Session ruling: batch-3, 2026-07-25.

## [v0.24.0] DESCRIPTION: cut 1,795 → 1,413 chars (delivery cap measured at exactly 1,536)
- **Disposition:** deleted (description ledger)
- **Tier failed:** 2 — every dropped clause is protocol detail restated from the SKILL.md body / `references/CROSS-EXAM.md`; no trigger or boundary behavior lost
- **Content:** dropped clauses — the sample-audit-the-map record-integrity-lens instruction; the attack/defend, persuades-never-vetoes, cited-never-re-routed exchange detail; the tally example string ("N raised, M survived"); the `RECORD-FITNESS.md` path (checklist still named); "before any contact with the counterpart reviewer" (substance preserved by "independent cold read FIRST")
- **Consumers assessed:** delivery-side only — no file consumes description text
