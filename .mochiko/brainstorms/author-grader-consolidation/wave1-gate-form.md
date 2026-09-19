# Wave 1 — the gate form (`author-grader-consolidation` D3–D7, D9, D11)

**Opened:** 2026-09-19 · **Lead:** the brainstorm session lead · **Target version:** 0.111.0 (MINOR —
one new skill, schema additions, no protected exits) · **Audited under:** the OLD form
(`mochiko:validator`, full read, one fresh seat per cluster — the old form's last full wave, D10).

Ruling anchors: `2026-09-19 author-grader-consolidation D2` (job split) · `D3` (gate contract) ·
`D4` (loop-bound home) · `D5` (scope) · `D6` (one re-audit, second FAIL to the user) · `D7`
(plain seat + rendered contract + explicit tier) · `D9` (outcome line) · `D11` (one seat per wave).

## 0. Sound-loop wiring

Trigger fired on every item below (judgment-authored, governing surface). Legs: **1** each producer
seat plans first, plan-only dispatch, the lead approves, the seat is resumed to execute (the
`producer-plan-enforcement` peer-grade leg is wave 2, not yet in force — the lead approves) ·
**2** non-author audits under the old form (§4) · **3** the user gates the `plugin.json` bump.
Transport: subagents, disjoint file ownership, no cross-seat messaging — the transport floor's
message legs do not fire; the topology leg is satisfied by ownership (§2).

Model routing: every producer and every grader `model: opus` explicit on the spawn (persona-less
seats — D7's own rule, applied to this wave before it ships); Explore reads `model: haiku`.

## 1. Sequence allocation

Migration **`0008-gate-form.yaml`** — one file, sequence 8, assigned here. No seat allocates.

## 2. Seats and ownership (disjoint)

| Seat | Owns | Depends on |
|---|---|---|
| **P1 schema** | `plugins/mochiko/migrations/0008-gate-form.yaml` · `.mochiko/schema-views/**` (re-emit only) · `evals/contract/expected-skills.json` (new member row + `patterns-model-tiering` floor re-key) · `evals/contract/run.py` (`EXPECTED["setup"]` floor re-key · `PROBE_ARGUMENTS` row for the new skill · the zero-baseline guard at the delivery-case percentage, mirroring the summary printer's) · `evals/contract/README.md` floor-count prose · `scripts/similar-rules-allowlist.yaml` (rows only for edges the sweep actually reports, stub-vs-local shape) — ownership widened at P1 plan approval, lead-ruled | — |
| **P2 prose** | `plugins/mochiko/skills/validation-primitive-edit/SKILL.md` (new) · `plugins/mochiko/skills/mochiko/SKILL.md` (router row + `validation-*` paragraph) · `.mochiko/memory/primitive-cost-budgets.md` (first-seed row) · `.claude/rules/mochiko/primitive-edits.md` (Check section) · `CHANGELOG.md` · `plugins/mochiko/.claude-plugin/plugin.json` + `.claude-plugin/marketplace.json` (0.111.0) | P1's section ids (pinned in §3, so P2 may start once P1's plan is approved) |
| **P3 crate tests** | `crates/mochiko-cli/tests/fidelity.rs` · `validate.rs` · `matrix_similar.rs` (census re-key after 0008: sequences, live-rule totals, `patterns-model-tiering` floor pin, similarity sweep baseline) · **widened at P3 plan approval, lead-ruled:** `tests/views.rs` and `tests/render.rs` census assertions (document count 73 → 74; "thirty skills" → thirty-one; checked 36 → 37) · `crates/mochiko-cli/Cargo.toml` + `Cargo.lock` (0.1.0 → 0.2.0, the grammar widening) | P1 landed (`migrate status` figures) |
| **P4 crate op** (added 2026-09-19 — §7 halt fired at P1's C1, user-ruled "widen the grammar") | `crates/mochiko-cli/src/**` (the one change: `mint-rule` with no `section:` on a `command-common` / `skill-common` document appends a block at the top level; every existing rejection on blocks stands) · its unit/integration tests · `plugins/mochiko/migrations/README.md` grammar table row for `mint-rule` | — (P1's C1–C3 resume after P4 lands; P3 after P1) |

Order: P1 plan → approve → P1 execute → (P2 ∥ P3) plan → approve → execute → §4 audits → §5 gates
→ landing.

## 3. Content — what each seat produces

### 3.1 Migration `0008-gate-form.yaml` (P1)

`grammar: 1 · id: 0008-gate-form · sequence: 8 · intent:` one line · header `anchor: 2026-09-19
author-grader-consolidation D7` · hash by `mochiko-cli migrate stamp`. Changes, in this order:

1. **`mint-rule` on `command-common/common`** — `common.gate-loop-bound`, labels `[user-gate,
   independence]`, anchor `2026-09-19 author-grader-consolidation D6`. Text (the number lives
   here and nowhere else): *"A gate verdict of FAIL allows one fix and one re-audit — by the same
   grader seat resumed, reading only what the fix touched and what it could have broken; a second
   FAIL halts the landing and goes to the user with both fix lists, fix again or drop; no run
   raises this bound. Beyond the extending command, `.claude/rules/mochiko/primitive-edits.md` and
   `validation-primitive-edit.gate-loop-bound` cite this id — a rename or tombstone sweeps them."*
   (Single extender by ruling — record OQ1; the near-dup 3+ bar is for extraction, not for a ruled
   home.)
2. **`mint-rule` on `command/setup`**, section `setup.sec.boundaries` — `setup.gate-loop-bound`,
   `class: floor`, `extends: common.gate-loop-bound` (class local, per the extends rule).
3. **`mint-rule` on `command/setup`**, section `setup.sec.roles` — `setup.validate-seat-form`,
   labels `[independence, seats]`, `class: must`, `kind: binding`, anchor `… D7`. Text: *"The
   validate step's grader is a fresh seat that authored no surface, running
   `mochiko:validation-constitution` with its floors unchanged, spawned with an explicit `model:`
   alias never below the producer's tier; its FAIL→fix→re-grade loop is bounded by
   setup.gate-loop-bound."*
4. **`mint-rule` on `skill/patterns-model-tiering`**, section `patterns-model-tiering.sec.discipline`
   — `patterns-model-tiering.persona-less-grader-pin`, labels `[boundary]`, `class: floor`,
   `kind: bound`, anchor `… D7`. Text: *"A persona-less grader or reviewer spawn carries an explicit
   `model:` alias, never omitted — the tier the graded work was produced at, never below, `opus`
   when the lead itself produced it; a bare spawn inherits the session tier and has failed this
   floor."*
5. **`import-document` kind `skill`, name `validation-primitive-edit`** — the gate contract as
   rules, **review six-set** (`independence · scope · inputs · verdict · output · reserved`),
   `vars: {verdict: PASS}`, every rule labelled from the live registry, no `kind: fail`, no
   `moments`. Rule set (ids fixed here; P1 writes the text from the record's D3/D6/D7/D9/D11
   statements, verbatim where the record already phrases the obligation):

   - `sec.independence`: `.author-grader` (floor, extends `review-common.author-grader`) ·
     `.plain-seat-explicit-tier` (floor, binding, `[independence, binding]`, anchor D7: persona-less
     fresh seat, authored nothing in the unit, explicit `model:` = producer tier never below, `opus`
     when the lead produced; omitted alias = floor miss) · `.rendered-contract-only` (floor,
     binding, `[independence, fence]`, anchor D7: these rendered rules ARE the contract; the
     dispatcher writes only unit · paths · pre-pass command; a hand-written contract section is a
     floor miss; the grader grades against the render, never a paraphrase) · `.one-seat-per-wave`
     (must, binding, `[independence, binding]`, anchor D11: one seat takes every unit of a wave,
     per-unit verdict block and outcome line tagged with the seat, split only on context fit and
     said so).
   - `sec.scope`: `.gate-job` (must, routing, `[boundary, verdict]`, anchor D2: the gate job —
     binary, the lead cannot ship past it — at the primitive-edit gate only, a shipped
     `plugins/mochiko/` primitive before the `plugin.json` bump (GI-004); input-job reviews are the
     `review-*` family's, setup's governance set is `validation-constitution`'s) · `.unit-keyed`
     (must, binding, `[boundary, binding]`: the unit is keyed by kind — command pair · skill pair ·
     prose primitive · schema content (migration + regenerated view diff); the criteria follow the
     unit) · `.never-excess` (must, extends `review-common.never-excess`).
   - `sec.inputs`: `.from-file-floor` (floor, `[fence, evidence]`: inputs read from the files —
     the edited primitive, its render via `mochiko-cli rules <primitive>`, the migration and its
     view diff, the strip entry where owed — never the author's report) · `.pre-pass-first-hand`
     (floor, duty, `[evidence]`, anchor D3: the grader runs `mochiko-cli migrate validate --report`
     and the char-budget measurement itself and quotes the output; a pre-pass result quoted from
     the brief is not evidence; nothing that output asserts is re-derived by judgment) ·
     `.brief-carries-unit` (must, binding, `[binding]`: the brief carries unit kind + name, file
     paths, pre-pass commands — nothing else of the contract).
   - `sec.verdict`: `.binary-verdict` (floor, `[verdict]`) · `.default-fail` (floor, extends
     `review-common.default-fail`) · `.tamper-proof-clause` (floor, `[verdict, evidence]`, anchor
     D3: no evidence-read line naming this run's files ⇒ FAIL automatically — the unit is not done
     until read) · `.judgment-items-pair` (must, binding, `[verdict]`, anchor D3: for a command or
     skill pair, each confirmed once with one evidence line — preserved responsibilities · floor
     survival · independence (no self-grading seat row) · reserved-to-user in the reserved section
     · the matching done-condition branch · an argued overage where the pre-pass shows one; the
     mechanical items — scaffold headings/order, section-set enumeration, `kind: fail` ↔ `.fail.*`,
     id continuity/tombstones, ontology grammar, `extends:` conformance, pointer resolution — are
     the pre-pass's, read from its output) · `.judgment-items-schema` (must, binding, `[verdict]`,
     anchor D3: for schema content the AM-2 five — intent stated · anchor present where required ·
     ID lifecycle right · floor and fail survival · register) · `.judgment-items-prose` (must,
     `[verdict]`: prose primitive — coherence + preserved responsibilities; a kitted persona edit
     also reads the advisory grid the brief cites, never a gate) · `.gate-loop-bound` (floor,
     bound, `[verdict, user-gate]`, anchor D6: one re-audit by the SAME seat resumed, delta read;
     second FAIL to the user; no run raises it; command-side home `common.gate-loop-bound`).
   - `sec.output`: `.verdict-block` (must, binding, `[reporting, binding]`: per unit `VALIDATE:` ·
     `Checklist run:` · `Evidence read:` (absent ⇒ FAIL) · `Pre-pass: <command> → <quoted>` ·
     per-item PASS/FAIL + evidence · `VERDICT: PASS | FAIL` · `Issues requiring fix:` item · missing
     thing · concrete fix) · `.outcome-line` (must, duty, `[reporting, evidence]`, anchor D9:
     `audit: <unit> · <seat> · <tier> · <n> files · <n> rounds · <n> blocking[ · cost: $<x>]` into
     the wave's `build-log.md` entry or the defect-close ADR; `cost:` for launched-session audits
     only) · `.evidence-floor` (floor, extends `review-common.evidence-floor`).
   - `sec.reserved`: `.second-fail-user` (floor, reservation, `[user-gate]`, anchor D6: the
     disposition after a second FAIL is the user's — fix again or drop; overruling a grader rides
     the ledger's waiver path, never a run's own call).

   Floor count by construction: **11** (`author-grader` · `plain-seat-explicit-tier` ·
   `rendered-contract-only` · `from-file-floor` · `pre-pass-first-hand` · `binary-verdict` ·
   `default-fail` · `tamper-proof-clause` · `gate-loop-bound` · `evidence-floor` ·
   `second-fail-user`). P1 confirms from the render pin.

Then: `mochiko-cli migrate stamp` → `migrate validate --report` (0 rejecting; advisories noted) →
`views emit --out .mochiko/schema-views` → `migrate status` (figures for P3).

**Contract-suite pre-registration (P1, same unit, derived under ruling — `evals/contract/README.md`
"When a migration legitimately moves a floor set"):** `expected-skills.json` gains the member
`validation-primitive-edit` (family `review`, `floor_ids` + `floor_pin` from the render; the four
byte columns `0` with a `note`: *post-freeze member, 2026-09-19 author-grader-consolidation D7 —
no pre-conversion baseline exists, never measured*) and `families.review.members` lists it;
`patterns-model-tiering` moves `floor_ids`/`floor_pin` only (+1). `run.py` `EXPECTED["setup"]`
gains `setup.gate-loop-bound`, `baseline_bytes` untouched. README floor-count prose updated.

### 3.2 Prose (P2)

- **`skills/validation-primitive-edit/SKILL.md`** — the `validation-constitution` shape: frontmatter
  `name` · `description` (MUST/SHOULD + exact trigger phrases — "gate audit", "primitive-edit
  audit", "grade the pair", "author≠grader audit"; ≤ 1,536 chars, measured) · `allowed-tools:
  Bash(mochiko-cli *)`; body: title, a short overview (the gate job, the plain seat, the rendered
  contract), the `## Rules — delivered by mochiko-cli` section with the seven `!` lines in the
  family order (`preamble` then the six), the floor-count read-back sentence, a `## Procedure`
  (run the pre-pass first-hand → read the unit's files → walk the judgment items for the unit's
  kind → emit the verdict block → write the outcome line; on FAIL wait to be resumed for the one
  delta re-audit). No restated rule text in the body.
- **Router** (`skills/mochiko/SKILL.md`): the `validation-*` paragraph lists
  `validation-primitive-edit` beside `validation-constitution` (persona wording untouched — wave
  3's sweep); a new table **"Primitive-edit gate (model-invoked — reached by the maintainer
  ceremony in `.claude/rules/mochiko/primitive-edits.md`)"** with the one row.
- **Budget** first-seed row for the new skill: measured delivered-at-invoke payload (body + the
  seven renders, characters of the parsed value, canonical snippet), no headroom, stamped
  `[v0.111.0]`, note "first budget row at birth — argued-overage path from here".
- **`primitive-edits.md` Check section** — rewritten to the record's build item 2, verbatim
  obligations: deterministic pre-pass first, run by the grader itself; the criteria lists keep every
  numbered item but each is tagged **[CLI]** or **[judgment]** from the inventory in §3.4; the gate
  grader is a plain fresh seat carrying `mochiko:validation-primitive-edit`'s render pasted verbatim
  (dispatcher writes unit · paths · pre-pass command only; hand-written = floor miss); explicit
  `model:` alias per the tiering rule; one seat per wave (D11); one re-audit by the same seat
  resumed on the fix delta, second FAIL to the user — fix again or drop (bound cited by id
  `common.gate-loop-bound`, number lives there); the D9 outcome line named as part of the landing;
  "restart from the checklist" and the full-cluster re-read removed; `mochiko:validator` →
  "the gate grader" in this file (the persona retires at wave 3). Skill-pair and command-pair
  criteria blocks, the persona-grid paragraph, the protected-content paragraph: content
  preserved, only the tags and the grader/loop sentences change.
- **CHANGELOG** `## [0.111.0] — 2026-09-19` entry (record + DECISIONS pointers, the D-list in
  prose); **manifests** 0.110.0 → 0.111.0, both files.

### 3.3 Crate tests (P3)

Re-key the three frozen censuses to the post-0008 state read from `migrate status` and the
render: `fidelity.rs` sequences `[…, 7, 8]`, live-rule total, `patterns-model-tiering` floor pin
(+1), any needle comment naming this migration; `validate.rs` the same numbers; `matrix_similar.rs`
scanned/scored/clusters/suppressed re-baselined after confirming clusters are genuinely 0.
`cargo test -p mochiko-cli` green, `cargo fmt --check`, `cargo clippy` clean. No production code
touched. Independent non-author review at §4 (rust-cli.md).

### 3.4 CLI-asserted vs judgment inventory (P2 derives it; A3 checks it)

From `crates/mochiko-cli/src/validate.rs` `Code`: **CLI-asserted** — section set (`SectionSet`),
id format/prefix/duplicate/mint-once/tombstone integrity, labels, vars, `extends` resolution +
cross-family + class-local, `when`/conditions/moments, `enforces` resolution/required/misplaced,
fail-segment ↔ kind, skill grammar, class/kind sets, text presence, protected exit + anchor
format, depth, citations, pointers, superseded/unknown fields, flat rules, home shape, and the
advisory set (budget, coverage, similar-rule clusters). **Judgment** — everything in D3's pair set,
the AM-2 five for schema content, coherence + preserved responsibilities for prose. The `!`-line
enumeration and the literal `allowed-tools` grant are checked by the contract suite's
`converted-shape` host case — tag **[suite]**; the `.md` scaffold headings and their order are
checked by nothing mechanical — tag **[judgment]** (corrected at P2 plan approval, lead-ruled).

## 4. Audits — old form, last full wave (leg 2)

Three fresh `mochiko:validator` seats (persona default `opus`), full read, one per cluster:

- **A1 — pairs:** `setup` pair (`commands/setup.md` + `mochiko-cli rules setup`) on the command
  criteria; `patterns-model-tiering` pair and `validation-primitive-edit` pair on the skill-pair
  criteria (12 items) — the new skill's `description` byte-identity item reads "≤ 1,536 chars" only.
- **A2 — schema content + pre-registration:** `0008-gate-form.yaml` + the view diff on the AM-2
  five; `expected-skills.json` / `run.py` / README changes as a field-scoped diff against the
  render (floor fields only moved; byte columns untouched on existing rows; the new row's note).
- **A3 — prose + crate:** `primitive-edits.md` (coherence, preserved responsibilities, the §3.4
  tags against `validate.rs`), router, budget row, CHANGELOG, manifests; the three crate test
  files as the independent non-author review (rust-cli.md) — a fixture refresh, not a logic change.

Each audit writes its D9 line into `build-log.md` (`audit: … · validator · opus · …`) — the
baseline's first old-form samples. FAIL → fix by the owning seat → re-audit (old form: full
re-read by a fresh seat is the old rule; this wave follows the old rule as ruled).

## 5. Gates before the bump

`mochiko-cli migrate validate --report` 0 rejecting · views ≡ replay (`views emit` produces no
diff) · `cargo test -p mochiko-cli` green + `fmt` + `clippy` · **contract suite full run green**
(`python3 evals/contract/run.py`, Docker sandbox up; the two new cases for the new skill included;
a SKIPPED suite is not green) · char-budget pre-assert on every touched budgeted primitive ·
CHANGELOG entry present · manifests synced · strips: none owed (pure additions; schema content
in the log; `primitive-edits.md` is a repo rule) — asserted, not assumed.

## 6. Landing

`build-log.md` entry closed with the D9 lines and the roster/disclosure line (`floor: tripped ·
seats: P1/P2/P3 / A1/A2/A3`) · record Status line + index entry → "wave 1 BUILT at v0.111.0" ·
BACKLOG gate-form item → trail (one-line DONE + pointer) · ROADMAP row touched · DECISIONS row
status → "wave 1 built" · suggested commit (never run by the lead).

## 7. Stop conditions

Any rejecting `migrate validate` finding the seat cannot resolve inside its ownership · any audit
FAIL that would need a change to a ruled decision · contract suite not green after one fix round ·
any crate production code touched — each halts to the user.
