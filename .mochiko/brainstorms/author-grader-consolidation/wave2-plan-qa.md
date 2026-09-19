# Wave 2 — the plan-QA leg (`producer-plan-enforcement` D1–D9 · D8 items 0–4, 5′, 6–10), audited under the new gate form

**Opened:** 2026-09-19 · **Lead:** the brainstorm session lead · **Target version:** 0.112.0 (MINOR —
one new skill, floor rewords, no protected exits) · **Sequenced by:** `author-grader-consolidation`
D10 (this is that path's wave 2) · **Audited under:** the NEW gate form (D3 · D6 · D7 · D11) — one
plain seat, rendered contract, one re-audit by the same seat — plus the D9 double-grade of one unit.

Ruling anchors: `2026-09-03 producer-plan-enforcement D8` (the landing set; D1–D7 as amended) ·
`2026-09-19 author-grader-consolidation D9/D10` (the double-grade and the sequencing).

## 0. Sound-loop wiring — this wave runs under the leg it ships (PPE D9)

Trigger fired on every item. **Leg 1, the new form:** each producer seat is spawned once in the
main tree with a plan-only brief; the lead snapshots `git status --porcelain` before the dispatch
and diffs after the seat returns (any new or changed path = FAIL, `dirty`); the plan text comes back
verbatim; a **fresh generic peer** (`general-purpose`, `model: opus` — the producers are
persona-less, PPE D3/X3) grades it default-FAIL against D5's seven items carried verbatim in its
brief (the skill this wave builds does not yet exist — the pre-install path, PPE D4/X15, recorded
per grade); the lead approves only a PASS and resumes the same seat with "approved" and the plan
quoted (D7). Re-plan bound one on a shared counter; a second consumption goes to the user (D6).
**Watch with stop condition (D8 item 7):** plan FAILs > half the producers, any second re-plan, any
dirtied tree → halt and report. **Leg 2:** §4. **Leg 3:** the user gates the bump. Transport:
subagents, disjoint ownership, no cross-seat messaging.

**Install refresh (D8 item 10 / D9 abort gate):** the installed copy is 0.110.0 from the marketplace
checkout at `9f256f0`; the repo is at 0.111.0 (`2ec189b`, pushed: no). Landing the refresh needs a
push and a marketplace update — user actions (the lead never pushes). Until the user rules (refresh
or waive with the pre-install path), no planning dispatch runs. Recorded in the log either way.

## 1. Sequence allocation

Migration **`0009-plan-qa-leg.yaml`** — sequence 9, assigned here.

## 2. Seats and ownership (disjoint)

| Seat | Owns | Depends on |
|---|---|---|
| **P1 schema** | `plugins/mochiko/migrations/0009-plan-qa-leg.yaml` · `.mochiko/schema-views/**` (re-emit) · `evals/contract/expected-skills.json` (new member `review-seat-plan`, post-freeze row; floor fields elsewhere only if a pin moves — none expected: every reword keeps its id) · `evals/contract/run.py` (`PROBE_ARGUMENTS` row) · `evals/contract/README.md` figures (+1 skill: 91 cases, +4 sessions) · `scripts/similar-rules-allowlist.yaml` (rows only for edges the sweep reports) | — |
| **P2 prose** | `plugins/mochiko/skills/review-seat-plan/SKILL.md` (new) · router `skills/mochiko/SKILL.md` (new "Seat-plan QA" table row; nothing else — the `validator`-bearing lines are wave 3) · `.mochiko/memory/primitive-cost-budgets.md` (birth-seed rows) · `.claude/rules/mochiko/primitive-edits.md` (criterion 6 only, D8 item 5′) · `CHANGELOG.md` 0.112.0 · both manifests 0.112.0 | P1's section ids (pinned in §3.1) |
| **P3 crate tests** | the frozen-census tests (`fidelity.rs` · `validate.rs` · `matrix_similar.rs` · `views.rs` · `render.rs`) re-keyed after 0009; no crate source, no version bump (0.2.0 stands) | P1 landed |

Order: P1 plan → peer grade → approve → execute · (P2 ∥ P3 after P1's plan) plan → peer grade →
approve → execute · §4 audits · §5 gates · §6 landing.

## 3. Content

### 3.1 Migration `0009-plan-qa-leg.yaml` (P1) — header anchor `2026-09-03 producer-plan-enforcement D8`

1. **`reword-rule` `patterns-sound-loop.leg-1-seat-produces`** (floor; id survives; pin stays 6;
   rule-level anchor `… D8`) — the user-authored text, verbatim from D8 item 1: *"Leg 1 — the
   producing seat plans first, read-only, in a plan-only dispatch, and stops; a fresh peer of its
   persona type grades the plan per `mochiko:review-seat-plan`; the lead approves only a passed
   plan and resumes the seat to work. The plan grade is additive to leg 2, never a substitute."*
   Ceremony note: under the current regime the migration carries the verbatim prior text and the
   anchor — that IS the supersession-by-ruling record D8 item 1 asked for; no `.mochiko/strips/`
   entry is owed for schema content (`primitive-edits.md`, "Schema content").
2. **`reword-rule` `patterns-sound-loop.disclosure-line`** (D8 item 4): the pinned grammar gains one
   segment after the seats — `floor: tripped|clear · seats: <who produced> / <who reviewed> ·
   plans: <seat>:PASS|FAIL(n)[ dirty]…` — `n` the re-plan rounds consumed on the shared counter,
   `dirty` when the seat changed the tree while planning; `floor: clear` stays bare.
3. **`reword-rule` `common.plan-approval-producers`** (three stubs inherit — brainstorm · setup ·
   specify; "the lead", never DM): *"Any seat that writes artifacts plans first, read-only, in a
   plan-only dispatch, and stops; a fresh peer of its persona type — a generic seat for a
   persona-less producer — grades the plan per `mochiko:review-seat-plan`; the lead approves only a
   passed plan and resumes the seat to work; grading and fact-finding seats are exempt."*
4. **Command-local rewords** (D8 item 2; each keeps its id; floors carry the rule-level anchor):
   `impl.plan-approval-producers` (same mechanism; exemption set keeps `verification`; DM wording)
   · `impl.design-gaps-only` and `impl.builder-decompose-disclose` ("on a plan a fresh peer passed
   and you approved") · `arch.seat-architect-producer` · `arch.author-grader-separation` (floor) ·
   `feat.author-grader` · the three pointer floors `arch.sound-loop-floor` / `feat.sound-loop-floor`
   / `impl.sound-loop-floor` ("a seat produces on a plan a fresh peer passed and you approved").
   Near-dup: the three pointer texts already differ by command; if any reword makes 3+ identical,
   converge via `common.yaml`, else keep local and allowlist the edges the sweep reports.
5. **`reword-rule` `patterns-plan-minimalism.grading-routing`** (D8 item 9): *"`mochiko:review-seat-plan`
   grades a seat plan's rung-claim presence — blocking on presence; `mochiko:review-plan-artifacts`
   grades the design package's disclosed rung honesty at review — advisory, gap-list conformance
   blocking."*
6. **`import-document` kind `skill`, name `review-seat-plan`** — review six-set, `vars: {verdict:
   PASS}`. Rule set (ids fixed; P1 writes texts from PPE D2–D6, verbatim where the record phrases):
   - `sec.independence`: `.author-grader` (floor, extends `review-common.author-grader`) ·
     `.fresh-peer-grader` (floor, binding, `[independence, seats]`, anchor D3: the grader is a fresh
     spawn of the author's persona type, never the author's context, never the lead; a persona-less
     producer's plan is graded by a fresh generic seat carrying this skill's render; devils-advocate
     is not a seat in this loop) · `.two-way-delivery` (must, binding, `[binding]`, anchor D4: model-
     invoked by description AND named in every grader brief with the render pasted; pre-install, the
     grader reads the repo pair by path and the run records which path).
   - `sec.scope`: `.plan-not-artifact` (must, routing, `[boundary]`, anchor D5/X9: grades the seat's
     plan before work; additive to sound-loop leg 2, never a substitute — the produced artifact is
     still graded by a non-author seat) · `.its-command-states-them` (must, extends
     `review-common.its-command-states-them`) · `.never-excess` (must, extends `review-common.never-excess`).
   - `sec.inputs`: `.plan-verbatim` (floor, `[fence, evidence]`, anchor D2: the plan reaches the
     grader as the seat's verbatim text in the brief, never a summary; the brief also names the
     seat's assigned scope and the other seats' declared write sets) · `.no-on-trust` (must,
     `[evidence]`: a claim the plan makes about the tree is checked against the tree — the grader
     reads what the plan says it read).
   - `sec.verdict`: `.default-fail` (floor, extends `review-common.default-fail`) · `.seven-items`
     (must, binding, `[verdict]`, anchor D5: the seven criteria verbatim — 1 scope fidelity · 2 write
     set declared · 3 reads named · 4 rung claims present (design seats; builders name the test
     first) · 5 stops and hand-offs named · 6 no self-clearing step · 7 size bound, advisory; 1–6
     blocking) · `.fail-cites-fix` (must, `[verdict]`: a FAIL cites the item and gives the fix) ·
     `.same-grader-regrades` (must, `[independence]`, anchor D6: after a FAIL the same grader seat
     re-grades the revision, resumed; fresh only if gone — the bound and the escalation are the
     command's, never restated here).
   - `sec.output`: `.verdict-to-lead` (must, binding, `[reporting, binding]`, anchor D2: the verdict
     returns to the lead as `PLAN GRADE: <seat> · PASS|FAIL · <items failed: fixes>`; the lead
     carries it into the disclosure line's `plans:` segment — the plan itself is never persisted) ·
     `.verdict-is-input` (must, extends `review-common.verdict-is-input`).
   - `sec.reserved`: `.approval-is-the-leads` (floor, reservation, `[user-gate]`, anchor D6: the
     lead approves only a PASS and may bounce a PASS with feedback; a second consumption of the
     re-plan bound goes to the user — re-plan, re-staff, or narrow — never the grader's call).
   Floors by construction: **5** — `author-grader` · `fresh-peer-grader` · `plan-verbatim` ·
   `default-fail` · `approval-is-the-leads`. `review-common.evidence-floor` is deliberately NOT
   extended: the plan is transient, so the verdict lands with the lead and in the disclosure
   line, not in a reviewed artifact. P1 confirms the pin from the render. *(Line corrected at the
   P3 peer grade, which flagged the earlier self-contradictory count.)*
7. stamp → validate --report (0 rejecting; clusters block) → views emit → `rules review-seat-plan
   --section preamble` (pin) → status → pre-registration edits → allowlist rows if reported.

### 3.2 Prose (P2)

- **`skills/review-seat-plan/SKILL.md`** — the `validation-constitution` shape: description with
  MUST/SHOULD + triggers ("grade the plan", "seat plan", "plan-only dispatch", "plan QA", "re-plan")
  ≤ 1,536 measured; `allowed-tools: Bash(mochiko-cli *)`; Rules section with the seven `!` lines
  (`preamble` + the six); the read-back sentence; a short Procedure (read the plan verbatim; check
  the tree for each on-trust claim; walk items 1–7; emit `PLAN GRADE:`; on FAIL wait to be resumed).
- **Router** — a new table **"Seat-plan QA (model-invoked — every producing seat's plan-only
  dispatch, all six commands)"** with the one row; nothing else (the `Operating rules` mount-doctrine
  line and the `validator` row are wave 3).
- **Budget** birth-seed rows for the new skill (payload + description), the ledger's fourth path.
- **`primitive-edits.md` criterion 6** — "Plan approval before any producing seat works" → "a
  producing seat's plan graded by a fresh peer per `mochiko:review-seat-plan` and approved by the
  lead only on PASS before it works" (D8 item 5′); nothing else in the file moves.
- **CHANGELOG** `## [0.112.0] — 2026-09-19`; manifests 0.111.0 → 0.112.0.

### 3.3 Crate tests (P3)

Census re-key after 0009 (documents 75, rules +N, sequences 1..9, `review-seat-plan` floors, any
render/views fixture count), figures read from `migrate status` and the render; full sweep
re-baselined after confirming clusters 0 (suppressed zero disclosed). No source, no version bump.

## 4. Audits — the NEW gate form (leg 2)

**One gate grader for the wave** (D11): a plain `general-purpose` seat, `model: opus` explicit,
persona-less. Its brief carries: the verbatim render of `mochiko-cli rules validation-primitive-edit`
(all seven blocks — pasted by the lead, never paraphrased), the unit list with file paths, and the
pre-pass commands. It runs the pre-pass itself and quotes it (D3). Units, one verdict block and
one D9 line each: `patterns-sound-loop` pair · `patterns-plan-minimalism` pair · the six command
pairs (arch · feat · impl locally reworded; brainstorm · setup · specify via the common stub) ·
`review-seat-plan` pair · schema content (0009 + view diff, AM-2 five) · pre-registration diff ·
router · budget ledger · `primitive-edits.md` · CHANGELOG + manifests · crate fixtures. It splits
into a second seat only if the files will not fit, and says so in the lines. **Re-audit (D6):** on a
FAIL the owning seat fixes and the SAME grader seat is resumed to read only the delta; a second FAIL
halts to the user.

**Double-grade (author-grader-consolidation D9/C4):** the **`review-seat-plan` pair** is also graded
by a fresh `mochiko:validator` under the old form (full read, skill-pair criteria) — the two finding
sets recorded side by side in that unit's line; this is the baseline later lines read against.
**Cost read (I2):** the plain seat's sidechain transcript
(`~/.claude/projects/<project>/<session>/subagents/agent-<name>-*.jsonl`) is read for its first
`cache_creation_input_tokens` and the validator's likewise; both figures land beside the two sets;
D7's cost limb moves from `Assumed` on that reading.

## 5. Gates before the bump

`migrate validate --report` 0 rejecting · views ≡ replay · `cargo test -p mochiko-cli` + fmt + clippy
+ audit · full similarity sweep · **contract suite full run** (91 cases; the new skill's two cases
included) · char-budget pre-assert on every touched budgeted primitive (release-gate sweep) ·
CHANGELOG · manifests synced · strips: asserted (schema content in the log; the router edit is a
pure addition; `primitive-edits.md` is a repo rule).

## 6. Landing

`DECISIONS.md`: the 2026-08-02 `command-architecture-realignment` row gains its supersession
annotation (D2's "native plan approval" clause; D8 item 3); the 2026-09-03 producer-plan row and the
2026-09-19 consolidation row → "wave 2 BUILT at v0.112.0" · BACKLOG: the producer-plan item's wave
1 → trail (one line), the item re-cut to the wave-3 retirement only (≤ 15 lines); the D9 watch item
gains the double-grade figures · ROADMAP row · record + index Landed lines · `build-log.md` entry
closed with the `floor:` line carrying the new `plans:` segment and every D9 line · commit suggested.

## 7. Stop conditions

PPE D8 item 7's watch (FAIL rate > ½ · any second re-plan · any dirty tree) · any rejecting
`migrate validate` finding a seat cannot resolve inside its ownership · a second FAIL on any gate
unit (D6 — to the user) · contract suite not green after one fix round · any crate source touched.
