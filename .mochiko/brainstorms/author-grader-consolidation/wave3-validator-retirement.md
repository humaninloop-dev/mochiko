# Wave 3 — the `validator` retirement (`producer-plan-enforcement` D3 · D8 item 5 · D9 wave 2), audited under the gate form

**Opened:** 2026-09-19 · **Lead:** the brainstorm session lead · **Target version:** 0.113.0 (MINOR —
one persona retired, three rule rewords, no protected exit without its supersession row; precedent:
the `product-engineer` → `product-designer` rename ruled MINOR, the `mochiko:explorer` deletion at
v0.78.0) · **Sequenced by:** `author-grader-consolidation` D10 (this is that path's wave 3, the
last) · **Audited under:** the gate form (D3 · D6 · D7 · D11) — one plain seat, rendered contract,
one re-audit by the same seat. No double-grade this wave (D9 took its baseline at wave 2).

Ruling anchors: `2026-09-03 producer-plan-enforcement D3` (the retirement and its consumer list,
the axis-5 and mount-doctrine wordings) · `2026-09-03 producer-plan-enforcement D8` item 5 (the
landing set) · `2026-09-19 author-grader-consolidation D7` (the carrier: a plain fresh seat running
the rendered `validation-primitive-edit` contract; `setup.validate-seat-form` for the setup loop).

**Gate satisfied (D3 / D9):** the carrier is live at v0.111.0 and has graded 14 units at wave 2; the
plan-grade figures are in (`plans: P1:PASS(1) · P2:PASS(0) · P3:PASS(1)`, no dirty tree). The
installed copy is 0.112.0 (`/reload-plugins --force` → 48 skills), so `review-seat-plan` reaches the
peer graders both ways.

## 0. Sound-loop wiring — this wave runs under the plan-QA leg it shipped

Trigger fired on every item. **Leg 1:** each producer seat is spawned once in the main tree with a
plan-only brief; the lead snapshots `git status --porcelain` before the dispatch and diffs after the
seat returns (any new or changed path = FAIL, `dirty`); the plan text comes back verbatim; a fresh
generic peer (`general-purpose`, `model: opus` — the producers are persona-less, PPE D3/X3) grades it
default-FAIL per `mochiko:review-seat-plan`, the brief naming the skill and carrying its render
(two-way delivery, D4); the lead approves only a PASS and resumes the same seat with "approved" and
the plan quoted (D7). Re-plan bound one on a shared counter; a second consumption goes to the user
(D6). **Watch (D8 item 7):** plan FAILs > half the producers, any second re-plan, any dirtied tree →
halt and report. **Leg 2:** §4. **Leg 3:** the user gates the bump. Transport: subagents, disjoint
ownership, no cross-seat messaging.

## 1. Sequence allocation

Migration **`0010-validator-retirement.yaml`** — sequence 10, assigned here. Header anchor
`2026-09-03 producer-plan-enforcement D3`. Rewords only: no id moves, no floor pin change, the rule
count stays 1,082.

## 2. Seats and ownership (disjoint)

| Seat | Owns | Depends on |
|---|---|---|
| **P1 schema + tiering pair** | `plugins/mochiko/migrations/0010-validator-retirement.yaml` · `.mochiko/schema-views/**` (re-emit) · `plugins/mochiko/skills/patterns-model-tiering/SKILL.md` (description + Overview count) · `.mochiko/strips/patterns-model-tiering.md` (one supersession-by-ruling entry) · `.mochiko/memory/primitive-cost-budgets.md` — the `patterns-model-tiering` rows **and the `authoring-constitution` payload row** (amended at approval: the two rewords add 66 render chars; ruled-HOLDS restamp, argued under unit 1) · `scripts/similar-rules-allowlist.yaml` (rows only for edges the sweep reports). **Sole writer of the budget ledger until it reports done.** | — |
| **P2 prose sweep** | `plugins/mochiko/agents/validator.md` (delete) · `plugins/mochiko/.claude-plugin/plugin.json` (the agents entry only — the version is the lead's) · `plugins/mochiko/skills/mochiko/SKILL.md` (four persona lines **plus the `patterns-model-tiering` row's roster count, line 71** — amended at approval) · `plugins/mochiko/skills/validation-constitution/SKILL.md` · `plugins/mochiko/skills/review-brainstorm/references/EXTERNAL-CLAIMS.md` (one pointer) · `CLAUDE.md` (lines naming `mochiko:validator` + axis 5, **title aligned to "Producer↔grader pairing"** — amended at approval; nothing in the `## Governance` region unless the user rules A on the amendment-log row, which then extends this scope to the `Ratified:` line and the ledger's version log) · `.claude/rules/mochiko/primitive-edits.md` (one parenthetical) · `ARCHITECTURE.md` (legacy hand-maintained index, six lines) · `.mochiko/memory/governance-ledger.md` (**GI-004's two detail lines, 111 and 126, + one dated editorial note — corrected at approval: line 126 sits under GI-004, GI-005 names no persona and takes no note**) · `.mochiko/memory/primitive-cost-budgets.md` — the `validator` agent row and the `validation-constitution` rows only, **written after P1 reports done** · strips: `.mochiko/strips/validator.md` · `mochiko.md` · `validation-constitution.md` · `review-brainstorm.md` | P1's rule ids only (§3.1, for cross-reference); its budget rows on P1's landing |
| **P3 suite · crate fixtures · eval kit** | the frozen-census tests (`fidelity.rs` · `validate.rs` · `matrix_similar.rs` · `views.rs` · `render.rs`) re-keyed after 0010 · `evals/contract/run.py` and `evals/contract/README.md` only if a roster or name assertion reaches `validator` (the preload case is `devils-advocate` × `review-specifications` — expected untouched) · `evals/agents/validator/preregistration.md` (retirement stamp at the top) · `evals/README.md` (one line: the kit is frozen provenance) · `evals/plan/agents.py` only if it enumerates every persona directory (expected untouched) | P1 landed (for the re-key) |
| **Lead** | `CHANGELOG.md` `[0.113.0]` · both manifests → 0.113.0 · `DECISIONS.md` supersession annotations (§6) · ADR and older-record status lines · BACKLOG / ROADMAP / record / index / `build-log.md` | after §4 for the landing; the CHANGELOG + manifests before the gate audit |

Order: P1 ∥ P2 plan → peer grade → approve → execute · P3 plan → peer grade → approve → execute after
P1 lands · lead's CHANGELOG + manifests · §4 audits · §5 gates · §6 landing.

## 3. Content

### 3.1 Migration `0010-validator-retirement.yaml` (P1) — header anchor `2026-09-03 producer-plan-enforcement D3`

Three `reword-rule` ops, each keeping its id, class, kind and labels (the verbatim prior text rides
the log as the supersession record, as 0009 did for leg 1):

1. `skill/patterns-model-tiering` · `patterns-model-tiering.seat-default-key` (floor) — the
   `strong` list drops `validator`: five names (`devils-advocate · tech-lead · qa-engineer ·
   staff-engineer · principal-architect`); `down` unchanged. Everything else in the text stands.
2. `skill/authoring-constitution` · `authoring-constitution.never-co-mounted` — "The grading
   validator runs as a separate agent, never co-mounted with this skill" → the grader is a plain
   fresh seat that authored no surface, never co-mounted with this skill (per
   `setup.validate-seat-form`); the sequencing/ownership clause stands.
3. `skill/authoring-constitution` · `authoring-constitution.grading-routing` — "the independent
   validator's job (`validation-constitution`), run by a different agent" → the independent grader's
   job (`validation-constitution`), run by a fresh seat that authored none of the set.

Not touched, with reason: `validation-constitution.sec.independence`'s intent line and the
`governance-surfaces` / `governance-intent` template lines use "the validator" as a role word for
setup's grading seat, not the persona (D3: reworded only where the persona is named); every
`anchor: … validator-scope-and-verbosity` string is a historic anchor and stays. P1 re-emits the
views, runs `migrate validate --report` (0 rejecting expected; advisory count unchanged unless a
budget line moves) and the full similarity sweep, and confirms `migrate status` still prints 1,082
rules over 75 documents.

**`patterns-model-tiering/SKILL.md`:** description "six `opus` and four `sonnet`" → "five `opus` and
four `sonnet`"; Overview "six on `opus`, four on `sonnet`" likewise. Strip entry: supersession by
ruling (D3), the prior wording verbatim. Budget: re-measure description and payload (body + render);
restamp the rows on the ledger's ruled path where a figure moves — the render shrinks, so no overage
is expected.

### 3.2 Prose sweep (P2)

- **`agents/validator.md`** — deleted whole. Strip entry in `.mochiko/strips/validator.md`:
  supersession by ruling (`producer-plan-enforcement` D3 · `author-grader-consolidation` D7), the
  file's responsibilities named and where each now lives — the generic default-FAIL grade →
  `validation-primitive-edit` on a plain fresh seat (primitive-edit gate) and
  `setup.validate-seat-form` (setup's validate step); the evidence-hierarchy and source re-read
  clauses → the `from-file-floor` / `pre-pass-first-hand` floors of the gate skill; the
  `skills:` mount of `validation-constitution` → no persona preloads it, the setup lead names it
  in the seat's brief. `plugin.json`: the `./agents/validator.md` entry removed.
- **Router `skills/mochiko/SKILL.md`** — the family sentence ("on the `validator` persona (today:
  `validation-constitution` …)") → the `validation-*` grade runs on a plain fresh seat with an
  explicit `model:` alias, `validation-constitution` at setup's validate step and
  `validation-primitive-edit` at the primitive-edit gate; the `validator` roster row deleted; the
  mount-doctrine line → the ruled text, **"never mount producing and grading skills for the same
  artifact on one seat"** (the persona may hold both; the seat never grades what it produced); the
  "producer↔validator round" hygiene line aligned to "producer↔grader" — a role-noun alignment
  beside a reworded line, disclosed in the strip as such. Strip entry in `mochiko.md`.
- **`validation-constitution/SKILL.md`** — description: "Validator-side skill of the governance
  producer↔validator pair … run by an independent validator, never the author" → grader-side
  wording, run by a fresh seat that authored none of the set; body: "(never co-mounted; the
  validator is a different agent)" → the grader is a fresh seat that authored no surface. Procedure
  and rules untouched. Strip entry; description re-measured, row restamped if it moves.
- **`review-brainstorm/references/EXTERNAL-CLAIMS.md`** — the consumer line naming
  `agents/validator.md` gains "(retired v0.113.0; the clause now lives in
  `validation-primitive-edit`)" so the pointer is not dead. One-line strip entry.
- **`CLAUDE.md`** — "Starting new work" item 3 and the "Editing a shipped primitive" paragraph: the
  audit is by a plain fresh seat running the rendered `mochiko:validation-primitive-edit` contract
  against the unit (command pair · skill pair · prose primitive · schema content), held to the
  criteria in `primitive-edits.md`; the "(the matching `validation-*`/`review-*` skill otherwise)"
  clause goes — the gate skill keys by unit. **Axis 5** → the ruled text: every reviewable artifact
  is **graded by a structurally independent grader: a fresh seat that authored nothing it grades,
  running a different skill from the author's**; the mirror-checklist / adversarial-critique clause
  stands. Nothing inside `## Governance` changes.
- **`primitive-edits.md`** — the parenthetical "(no validator-for-skills exists, and a pilot member
  never grades itself)" → "(no skill-specific grader exists, and a pilot member never grades itself)".
- **`ARCHITECTURE.md`** — the two-families bullet (persona → plain fresh seat), the setup prose
  ("producer↔validator loop" → "producer↔grader loop"), the wiring table's `validator` row → grader ·
  plain fresh seat × `validation-constitution` (explicit `model:` alias), the mermaid node label, and
  the closing seat paragraph's `validator` sentence → the shipped-primitive and setup grades run on
  plain fresh seats carrying the rendered contracts.
- **Ledgers** — `governance-ledger.md` GI-004 detail "(`mochiko:validator` against the primitive's
  own text)" → a plain fresh seat running the rendered `mochiko:validation-primitive-edit` contract
  against the unit's own files; GI-005 detail "graded by `mochiko:validator` on five criteria" → by
  the primitive-edit gate seat on the same five; plus one dated editorial line under each: grader
  identity re-pointed 2026-09-19 at v0.113.0 by ruling, ratchet and criteria unchanged, no
  amendment. **P2 reads the ledger's own amendment policy first**; if it classes a detail edit as an
  amendment, P2 stops there and the lead raises it to the user (§8). `primitive-cost-budgets.md`: the
  `validator` agent row struck with a retirement note (the figure stays legible for provenance).

**Amendments at approval (2026-09-19, lead rulings on the seats' reserved questions):** (1) the
ledger's "graded by `mochiko:validator` on five criteria" line sits at line 126 **under GI-004**,
not GI-005 — both detail edits and the single editorial note are GI-004's; GI-005 names no persona
and takes nothing; (2) the router's `patterns-model-tiering` row (line 71) carries the roster count
and moves six → five with the same strip entry; (3) the axis-5 title aligns to
"**Producer↔grader pairing**" beside its reworded body — a role-noun alignment under this section's
licence, disclosed in the CLAUDE.md step; (4) the `authoring-constitution` payload row is P1's, the
+66 render chars argued under unit 1; (5) P2's budget rows wait for P1's landing (one writer per
file per window); (6) the ledger amendment-log row and the `Ratified:` header are the user's A/B
call — A adds a PATCH `3.1.1` row on the v3.0.1–v3.0.3 idiom and moves the header, B leaves both.

### 3.3 Suite, crate fixtures, eval kit (P3)

Re-key the five census tests to the post-0010 state (hash, byte totals; rule and document counts
unchanged); no crate source, no `Cargo.toml` change (0.2.0 stands). Read `run.py` for any assertion
that reaches the agent roster or the `validator` name — none expected; the preload case is
`devils-advocate` × `review-specifications`. Stamp `evals/agents/validator/preregistration.md` at the
top: persona retired at v0.113.0, kit frozen as provenance (its evidence lives on the
`eval-evidence-2026-09-19` ref), never re-run; one line in `evals/README.md` beside the kit layout.
Read `evals/plan/agents.py` for any all-persona enumeration that would trip on a kit without an
agent file; none expected (it resolves a named persona's file on demand).

## 4. Audits — the gate form (leg 2)

**One gate grader for the wave** (D11): a plain `general-purpose` seat, `model: opus` explicit,
persona-less. Its brief carries the verbatim render of `mochiko-cli rules validation-primitive-edit`
(all seven blocks, pasted by the lead), the unit list with file paths, and the pre-pass commands; it
runs the pre-pass itself and quotes it (D3). Units, one verdict block and one D9 line each:

1. schema content — `0010-validator-retirement.yaml` + the view diff (AM-2 five);
2. skill pair — `patterns-model-tiering` (SKILL.md + render · strip · budget rows);
3. skill pair — `validation-constitution` (SKILL.md + render · strip · budget rows);
4. prose primitive — `agents/validator.md` retirement + `plugin.json` + strip + the supersession
   rows the lead writes before the audit (preserved responsibilities: each named a new home);
5. prose primitive — router `mochiko/SKILL.md` + strip;
6. prose primitive — `review-brainstorm/references/EXTERNAL-CLAIMS.md` + strip;
7. repo prose — `CLAUDE.md` · `primitive-edits.md` · `ARCHITECTURE.md` (coherence, no persona
   name survives outside history, the ruled wordings verbatim);
8. ledgers — `governance-ledger.md` detail lines + editorial notes · `primitive-cost-budgets.md`;
9. suite + eval kit — `run.py` / `README.md` if touched · the preregistration stamp · `evals/README.md`;
10. crate fixtures — the five census tests;
11. `CHANGELOG.md` `[0.113.0]` + both manifests.

**Re-audit (D6):** on a FAIL the owning seat fixes and the SAME grader seat is resumed to read only
the delta; a second FAIL halts to the user. The grader splits into a second seat only if the files
will not fit, and says so in the lines.

## 5. Gates before the bump

`migrate validate --report` 0 rejecting · views ≡ replay · `cargo test -p mochiko-cli` + fmt +
clippy + audit · full similarity sweep · **contract suite full run** (91 cases — the case list is
unchanged) · char-budget pre-assert on `patterns-model-tiering` and `validation-constitution` ·
CHANGELOG · manifests synced · strips asserted: `validator` · `mochiko` · `validation-constitution` ·
`patterns-model-tiering` · `review-brainstorm` · no `mochiko:validator` spawn anywhere in
`plugins/mochiko/` or `.claude/rules/mochiko/` (grep, zero hits outside strips).

## 6. Landing

`DECISIONS.md`: the 2026-09-03 producer-plan row → "wave 2 (retirement) BUILT at v0.113.0 — path
complete"; the 2026-09-19 consolidation row → "wave 3 BUILT — all three waves built"; supersession
annotations on the 2026-08-10 `validator-scope-and-verbosity` row (persona limb only — the budget
and description discipline stands), the 2026-08-01 `validator-worktree-isolation` row (grader
identity only — the cold snapshot stands), and the two 2026-08-26 `validator` rows (whole: the ADR
`2026-08-26-validator-router-indexed-checklists` superseded, its Status line updated) · the two
older records' Status lines + index entries agree · BACKLOG: the producer-plan item → trail (OQ3
rides the D9 gate-form watch; OQ4 has no trigger and closes with the item) · ROADMAP "Floor builds"
row → all three waves BUILT · record Status + index Landed → wave 3 BUILT, path complete ·
`build-log.md` entry closed with the `floor:` line and every D9 line · commit suggested.

## 7. Stop conditions

PPE D8 item 7's watch (FAIL rate > ½ · any second re-plan · any dirty tree) · any rejecting
`migrate validate` finding a seat cannot resolve inside its ownership · a second FAIL on any gate
unit (D6 — to the user) · contract suite not green after one fix round · any crate source touched ·
any rule id or floor pin moved (rewords only) · P2's ledger halt (§3.2).

## 8. Out of scope, with reason

Role-noun uses of "validator" that do not name the persona stay: `testing-governance-injection`,
`analysis-codebase` ("an independent validator"), `authoring-constitution/SKILL.md`'s ledger table
cell, `tech-lead.md`, the two governance templates, `review-feasibility` / `review-plan-artifacts`
references (an OpenAPI validator), README and ROADMAP ("the human is the primary external
validator"). Frozen records stay: `governance-intent.md` (ratified synthesis — amend-run territory),
`governance-trace-summary.md` and `codebase-analysis.md` (dated), the benchmarks tree, migration
files 0001–0009, BACKLOG lines quoting delivered builds' seat figures. `.mochiko/provenance.yaml`
no longer exists (folded into the log at genesis), so D3's anchor item is moot.

**Lead's default, stated to the user at wave open:** the ledger GI-004/GI-005 detail lines are a
grader-identity re-point with the principle's meaning unchanged, handled as a detail edit under this
wave's audit, not a `/mochiko:setup` amend run; the user said "go" with that default in view.
