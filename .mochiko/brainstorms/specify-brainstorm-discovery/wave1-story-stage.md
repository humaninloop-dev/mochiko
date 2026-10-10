# Wave 1 — the specify story-discovery stage (`specify-brainstorm-discovery` D1–D7)

**Opened:** 2026-10-10 · **Lead:** the brainstorm session lead · **Branch:** `specify-story-stage`
(from `main` @ `ea57167`, carrying the uncommitted close ritual of this session and the open
sibling session's directory) · **Target version:** 0.119.0 (MINOR — new rules and rewords, one
`kind: fail` minted, no floor or fail retired or lowered) · **Ruling:** `DECISIONS.md` row
2026-10-10 · **Record:** `.mochiko/brainstorms/specify-brainstorm-discovery/record.md` — the seven
cards and the *Defaults batch — confirmed* section are the source for every rule text; this plan
names carriers, never restates a decision.

One wave. Specify and the five primitives D5-specify-own-rules names; brainstorm's rules, the
architecture desk and `mochiko:analysis-iterative` are untouched (D5).

## 0. Sound-loop wiring

The trigger fires on every item: judgment-authored writes to plugin primitives.

- **Leg 1.** Each producer is a persona-less `general-purpose` seat, `model: opus`, spawned once
  with a plan-only brief. The lead snapshots `git status --porcelain` before the dispatch and
  diffs it after the seat returns; any new or changed path is a FAIL, `dirty`. The plan comes back
  verbatim. A fresh generic peer (`general-purpose`, `model: opus`) grades it default-FAIL per
  `mochiko:review-seat-plan`, the render pasted into its brief. The lead approves only a PASS and
  resumes the same seat with the plan quoted. Re-plan bound: one, on a counter the three producers
  share; a second consumption goes to the user.
- **Leg 2.** §4, one gate grader for the wave.
- **Leg 3.** The user gates the commit and the merge. The lead never commits unasked, never pushes.
- **Transport.** Seats are teammates (the user's runs seat teammates, F3); disjoint ownership (§2);
  no cross-seat messaging; every hand-off is lead-relayed. Every brief carries the routing line:
  locate and enumerate reads go to a native `Explore` subagent spawned `model: haiku`.

## 1. Sequence allocation

- **`0051-specify-story-stage.yaml`** — `command/specify`.
- **`0052-stories-from-slate.yaml`** — `skill/authoring-user-stories`.
- **`0053-review-slate-checks.yaml`** — `skill/review-specifications`.
- **`0054-fr-cites-story.yaml`** — `skill/authoring-requirements`.
- **`0055-map-slate-touchpoint.yaml`** — `skill/authoring-feature-map`.
- **`0056-spec-template-slate.yaml`** — `template/spec` (`replace-document`).
- `0057`–`0058` held for a fix round; a gap is legal if unused.

Header anchor on all six: `2026-10-10 specify-brainstorm-discovery`. A minted rule that carries one
ruled decision takes that decision as its rule-level anchor (`… D1`); a rule carrying a confirmed
default takes the bare session anchor. The minted fail carries its anchor by construction.

## 2. Seats and ownership (disjoint)

| Seat | Owns | Depends on |
|---|---|---|
| **P1 schema** | the six migration files · `.mochiko/schema-views/**` (re-emit only) · `scripts/similar-rules-allowlist.yaml` (rows only for edges the sweep reports) | — |
| **P2 prose** | `plugins/mochiko/commands/specify.md` · `plugins/mochiko/skills/authoring-user-stories/SKILL.md` · `skills/review-specifications/SKILL.md` · `skills/authoring-requirements/SKILL.md` · `skills/authoring-feature-map/SKILL.md` · the strip files (`.mochiko/strips/specify.md` · `authoring-user-stories.md` · `review-specifications.md` · `authoring-requirements.md` · `authoring-feature-map.md` · `spec-template.md`) · ripple lines the build makes false (`README.md`, the router `skills/mochiko/SKILL.md`, `docs/`, `ARCHITECTURE.md` — found by grep, reworded only where false) · `.mochiko/memory/primitive-cost-budgets.md` · `CHANGELOG.md` 0.119.0 entry | P1 landed (budgets are measured against the render) |
| **P3 tests and kits** | the crate's frozen-census tests under `crates/mochiko-cli/tests/` (no crate source, no crate version bump) · `evals/contract/run.py` `Expected["specify"]` and README figures where a frozen id or count moved · `evals/plan/specify/` (the D8 partition re-keyed: every schema rule id in exactly one list) · `evals/review-specifications/` if it freezes rule ids | P1 landed |

The manifests (`plugin.json`, `marketplace.json`) are the lead's last mechanical step, after the
audit passes and before the gates run (GI-004-primitive-audit-ratchet: no bump before the audit).

Order: P1 plan → peer grade → approve → build · then P2 and P3 in parallel, each plan → peer
grade → approve → build · §4 audit · manifests · §5 gates · §6 landing.

## 3. Content

### 3.0 What every rule text must respect

- **Guardrails, never a sequence.** A rule says what must be true before something else happens;
  moments anchor, they never order.
- **Short.** One or two sentences per minted rule, in plain words. P1 reports each render's size
  before and after.
- **One fail minted, nothing retired or lowered.** `spec.fail.story-outside-slate` joins the nine;
  the 17 floors of `specify` stand, every other floor and fail across the five skills stands.
- **Texts come from the cards and the confirmed defaults.** Verbatim where a card or default
  phrases a rule; every changed-at-review and verify-round note is part of its decision.
- **The precedence clause reaches `mochiko:analysis-iterative` only** (D5); every other primitive
  takes a ruled edit of its own (D5 changed-at-review).

### 3.1 Migration 0051 — `command/specify`

The ids below are the lead's proposal. P1 may merge or split in its plan, with the reason; the
approved plan fixes the set. Every decision and default in §3.7 must keep a carrier.

**Declarations.** `set-moment story-stage` (where, after the intent synthesis is confirmed and
before any story is authored, the story set is worked with the user) · `set-moment slate-confirm`
(the user's confirmation of the slate — every candidate row ruled, the blind list's differences
ruled, nothing open) · `set-condition slate_source` (values `forks|handed-in`; resolution
moment-resolved(intent); note: handed-in when the user supplies stories or a brainstorm record
names them — the forks are skipped, the close is not).

**Rewords (ids survive).**

| Rule | Change | Source |
|---|---|---|
| `spec.reserved-to-user` | adds: each slate row's ruling (in / out, and a strike of a floor-obligated row) · the slate's confirmation | D1, D6, defaults *Floor-obligated stories* |
| `spec.lockstep-prototyping` | the skeleton nav frame is built from the confirmed slate; a click that changes a story's line or test returns to the user as a one-line amendment to its row | default *UX-bearing work* |
| `spec.filter-rejections-recorded` | the PM vets the slate once at its close (map fit — duplicates of pending rows or stubs, extend-vs-mint), returned beside the blind journey list; after authoring the filter keeps its homing and dedup mechanics; a rejection of an `in` row then escalates (`spec.filter-disagreement-escalates` stands) | default *Filter* |
| `spec.intent-probe-discipline` | the streak flag and the vague-answer re-ask also bind the story stage | default *Mechanisms carried* |

**Mints.** `class: must` unless marked.

| Section | Proposed id | Carries |
|---|---|---|
| roles | `spec.lead-holds-story-stage` | D3 — the lead runs the stage inline, main pane; the slate transcribed; the analyst authors from it |
| roles | `spec.blind-journey-list` | D4 — one fresh persona-less teammate, `model: opus`, given intent synthesis and frame only, never the slate; differences become candidate rows |
| roles | `spec.analyst-brief-and-pen` | default *Hand-off and slate writing* — the lead writes the slate at confirm, the pen passes explicitly; the brief's contents; later row changes landed as content-pinned mechanical execution |
| reserved | `spec.slate-rulings-users` (`kind: reservation`) | D1 — "which stories" is the user's; the PM floor on which capabilities untouched |
| reserved | `spec.gate-slate-confirm` (`kind: gate`) | D6 — the slate confirm is the user's, after the intent confirm, before authoring |
| tools | `spec.story-stage` | D1 — the stage, its close in a ruled slate, the slate rules what the spec carries never what builds |
| tools | `spec.slate-unit` | D2 as changed — line + priority + seed; the seed expanded without change of meaning, shown beside; `test: open` |
| tools | `spec.slate-home` (`kind: binding`) | default *Slate home and binding* — the User Stories index, columns, two-phase Disposition, the struck sub-table, the header line |
| tools | `spec.slate-binds-authoring` | default — files for `in` rows only; a missing story proposed as a row, never silent; the 2–5 bound yields |
| tools | `spec.stage-inputs-in-hand` | defaults *Inputs already in hand* and *Size and skip* — handed-in rows, provenance `handed in`, the close still runs |
| tools | `spec.slate-not-the-frame` | default *Ordering against the capability frame* |
| tools | `spec.floor-obligated-rows` | default *Floor-obligated stories* as repaired (N2) — pointer, narrowing, waiver, the held row |
| ways-of-working | `spec.story-forks` | default *Mechanisms carried* — one fork per turn · two real options, one the do-less shape · fact in hand first · own-turn-or-batch · a one-row mock · the status line |
| ways-of-working | `spec.slate-provenance` | the per-row provenance values |
| ways-of-working | `spec.story-stage-stop-rule` (`kind: bound`) | every candidate in or out, nothing open; the stall clause |
| ways-of-working | `spec.two-confirm-screens` | D6 |
| ways-of-working | `spec.own-rules-win` | D5 — where these rules and `mochiko:analysis-iterative` differ, these win |
| fail-conditions | `spec.fail.story-outside-slate` (`class: floor`, `kind: fail`) | a story authored with no slate confirmed, or outside the confirmed slate — enforces `spec.story-stage`, `spec.slate-binds-authoring`, `spec.gate-slate-confirm` |

Untouched: every `extends: common.*` stub, every existing floor and fail, `spec.capability-frame-
at-intent` (F-S3's "never enumerating stories" stands — the slate is declared not the frame),
`spec.frame-hypothesis-not-anchor`, `spec.selection-card`, `spec.gate-selection`.

### 3.2 Migration 0052 — `skill/authoring-user-stories`

| Op | Rule | Change | Source |
|---|---|---|---|
| reword | `authoring-user-stories.story-structure` | one story per `in` row of a confirmed slate where one exists — the slate's count is the feature's; 2–5 per feature only for a caller with no slate; the rest of the structure stands | default *Slate home and binding* (the 2–5 bound yields) |
| reword | `authoring-user-stories.independent-test-required` | adds: where the row carries a seed, the Independent Test is its expansion — setup or data, pass and fail — without change of meaning, the seed kept beside it | D2 as changed |
| mint | `authoring-user-stories.slate-fidelity` | no story outside the slate; a story the slate lacks is proposed as a row to the user, never authored; a `test: open` row becomes an Open Questions entry plus a provisional test marked provisional | defaults, D2 |

P1 places the mint in the family section it fits (the `artifact` or `scope` section) and names it.

### 3.3 Migration 0053 — `skill/review-specifications`

| Op | Rule | Change | Source |
|---|---|---|---|
| mint | `review-specifications.slate-checks` (`sec.verdict`) | the four slate checks, each a blocking gap on failure: (1) every story file maps to an `in` row and no `in` row lacks a file, a row `in — waiver pending` not yet due; (2) each file's story line and Independent Test keep the meaning of its row's line and seed; (3) every FR cites an `in` row's story; (4) scenario count within the story skill's bound | default *Slate home and binding* as repaired (N2, N3) |
| reword | `review-specifications.complete-coverage` | adds "every slate row" to the coverage list | same |

`review-specifications.no-scope-creep` (floor) stands.

### 3.4 Migration 0054 — `skill/authoring-requirements`

| Op | Rule | Change | Source |
|---|---|---|---|
| reword | `authoring-requirements.fr-format` | each FR line cites the story it serves (`US-<n>`), the carrier of slate check (3) | N3 |

### 3.5 Migration 0055 — `skill/authoring-feature-map`

| Op | Rule | Change | Source |
|---|---|---|---|
| reword | `authoring-feature-map.four-touchpoints` | specify's touchpoint reads: frame at intent, the ruled slate (the PM's one map-fit vet at slate close), then confirm-frame, cut rows, the filter's homing and dedup mechanics, and selection after stories | default *Filter* |

`authoring-feature-map.frame-first`, `complete-disposition` (floor), `stories-inform-never-define`
stand: slate values are not story statuses.

### 3.6 Migration 0056 — `template/spec`

`replace-document` with the current content (from `0046-joined-id-templates.yaml`) changed only
where the defaults say: the `## User Stories` guidance — a header line recording the slate's source
(forks · handed in) and the blind list's seat; columns ID (linked for `in` rows) · Story (one
breath) · Priority · Test (the seed) · Feature · Disposition (slate phase `in` / `in — waiver
pending` / `out — <why>`, then `homed | rejected — why`); a `### Struck candidates` sub-table for
`out` rows, IDs unlinked; the "only story-native status is `rejected`" line kept, with the slate
values named as pre-authoring dispositions, not statuses. The `## Functional Requirements`
guidance — each FR line cites its `US-<n>`. The Conformance block, headings and budgets unchanged.

### 3.7 Decision and default coverage

| Decision / default | Carrier |
|---|---|
| D1-story-discovery-stage | `spec.story-stage` · moment `story-stage` · `spec.slate-rulings-users` · `spec.fail.story-outside-slate` · the command's Goal step |
| D2-line-plus-test | `spec.slate-unit` · `authoring-user-stories.independent-test-required` · `authoring-user-stories.slate-fidelity` |
| D3-lead-holds-stage | `spec.lead-holds-story-stage` · `spec.analyst-brief-and-pen` |
| D4-blind-journey-list | `spec.blind-journey-list` · `spec.slate-provenance` |
| D5-specify-own-rules | `spec.own-rules-win` · the six migrations as the touch-set · the convergence BACKLOG item (no carrier in the plugin) |
| D6-two-confirm-screens | `spec.two-confirm-screens` · `spec.gate-slate-confirm` · moment `slate-confirm` |
| D7-dogfood-criteria-deferred | no carrier: the BACKLOG build item's "then the first ai-fileops run" |
| Mechanisms carried | `spec.story-forks` · `spec.story-stage-stop-rule` · `spec.slate-provenance` · `spec.intent-probe-discipline` reworded |
| Slate home and binding | `spec.slate-home` · `spec.slate-binds-authoring` · `spec.fail.story-outside-slate` · `review-specifications.slate-checks` · `authoring-user-stories.story-structure` · template 0056 |
| Ordering against the frame | `spec.slate-not-the-frame` |
| Filter | `spec.filter-rejections-recorded` reworded · `authoring-feature-map.four-touchpoints` reworded |
| Floor-obligated stories | `spec.floor-obligated-rows` |
| Inputs in hand · Size and skip | `spec.stage-inputs-in-hand` · condition `slate_source` |
| Hand-off and slate writing | `spec.analyst-brief-and-pen` |
| UX-bearing work | `spec.lockstep-prototyping` reworded |
| Acceptance scenarios after | `spec.slate-unit` |
| FR cites its story (N3) | `authoring-requirements.fr-format` · template 0056 · check (3) |

### 3.8 Prose (P2)

- **`commands/specify.md`** — Identity & Mission gains the story stage in one clause; the Goal
  step adds the confirmed slate (the User Stories index rows with their dispositions, the struck
  sub-table) between the Intent section and the stories, and reads "stories as `stories/US-*.md`
  files for `in` rows". Entry and Not-done untouched (the Not-done line already cites the printed
  pin). Strip entry: supersession by ruling.
- **`skills/authoring-user-stories/SKILL.md`** — the body's format block and any "2–5" prose
  follow the reworded rules; the `description:` value stays byte-identical. Strip entries where
  content leaves or is superseded.
- **`skills/review-specifications/SKILL.md`**, **`authoring-requirements/SKILL.md`**,
  **`authoring-feature-map/SKILL.md`** — reword only what the new rules make false; descriptions
  byte-identical; strip entries.
- **Ripple** — `README.md`, the router skill, `docs/`, `ARCHITECTURE.md`: any line restating
  specify's flow as "intent then stories".
- **Budgets** — every touched skill stands at "no headroom" with a standing ruled overage; each
  growth is argued as a genuine new obligation (D1–D6, the confirmed defaults), measured with the
  ledger's canonical snippet, never derived from a prior figure; `specify`'s render size reported.
- **`CHANGELOG.md`** — `## [0.119.0] — 2026-10-10`, the gates paragraph filled at close.

### 3.9 Tests and kits (P3)

- Crate frozen-census tests (`crates/mochiko-cli/tests/fidelity.rs` and any sibling): command
  rules 362 → the post-0051 count, skill rules 813 → the post-0055 count, totals, command floors
  119 → 120, the fail set gains `spec.fail.story-outside-slate`; figures read from `migrate
  status` and the render, never computed by hand. No crate source, no crate version bump.
- `evals/contract/run.py` — `Expected["specify"]` floor ids gain the new fail; the size figure and
  README figures only where a frozen count moved. `expected-skills.json` untouched (no skill floor
  moves) — P3 confirms from the render.
- `evals/plan/specify/` — `observable.yaml` (the D8 partition: every new rule id in exactly one
  list, with its why), `evals.json`, the preregistration; the kit's static check passes; no grid.

## 4. Audit (leg 2)

One gate grader for the wave: a plain `general-purpose` seat, `model: opus` explicit. Its brief
carries the verbatim render of `mochiko-cli rules validation-primitive-edit` (every block), the
unit list with file paths, the pre-pass commands, and the named budget overages. It runs the
pre-pass itself and quotes it. Units, one verdict block and one outcome line each:

1. schema content — 0051 and its view diff
2. schema content — 0052–0055 and their view diffs
3. schema content — 0056 and its view diff
4. the `specify` command pair
5. the `authoring-user-stories` skill pair
6. the `review-specifications` skill pair
7. the `authoring-requirements` and `authoring-feature-map` skill pairs
8. strips, ripple lines, the budget ledger, the changelog entry
9. crate fixtures, contract expectations, the specify kit

On a FAIL the owning seat fixes and the same grader is resumed to read only the delta. A second
FAIL on any unit halts to the user (`common.gate-loop-bound`).

Beside the audit, the lead reads the built rules against §3.7 decision by decision. That read is
the lead checking seat output; it clears nothing on its own.

## 5. Gates before the bump lands

`migrate validate --report` 0 rejecting, clusters 0 · views ≡ replay · `cargo test -p mochiko-cli`
+ fmt + clippy + audit · the full similarity sweep · the contract suite, full run in Docker (a
SKIPPED suite blocks; `sbx login` is the user's) · char budgets on every touched budgeted primitive ·
`CHANGELOG.md` entry · both manifests at 0.119.0 · strips recorded.

## 6. Landing

`DECISIONS.md` 2026-10-10 row → built at v0.119.0 · `BACKLOG.md`: the build item → the trail, the
convergence item stays · `ROADMAP.md` Now row · the record's and the index's Landed lines ·
`build-log.md` closed with the `floor:` line and every `audit:` line · a commit suggested to the
user. The installed plugin copy and the ai-fileops copy are upgraded by the user after the merge;
the first ai-fileops specify run follows (D7-dogfood-criteria-deferred).

## 7. Stop conditions

Plan FAILs from more than half the producers · any second re-plan · any tree dirtied during
planning · a rejecting `migrate validate` finding a seat cannot resolve inside its ownership · any
floor or fail retired or lowered, or a second fail minted · any crate source file touched · a
second FAIL on any gate unit · the grader not holding an overage argument · the contract suite not
green after one fix round. Each halts the wave and goes to the user.
