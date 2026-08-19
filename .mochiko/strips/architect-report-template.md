# Strip notes — `templates/architect-report-template.md` (formerly `sysarchitect-report-template.md`, renamed v0.67.0)

Entry formats: `strips/README.md`. This log file retains its original name
(`strips/sysarchitect-report-template.md`) as the primitive's history anchor; the primitive it
tracks is now `templates/architect-report-template.md`.

---

## [v0.81.0] RETIRED — the template is deleted; its disclosure duty moves into the plan package's delta + sign-off trail

**The primitive `plugins/mochiko/templates/architect-report-template.md` no longer exists.** Whole-file
supersession by ruling; the retired file is quoted verbatim below.

- **Disposition:** superseded → the plan package's drafted delta + the sign-off trail (D10: the delta
  is drafted in the plan package with the store untouched, and **user sign-off on the rendered
  diagram + the named AX-row changes is the write gate**). The disclosure this template carried is
  made by the delta itself and the gate it passes through, not by a parallel report.
- **Tier failed:** n/a — supersession by ruling (record
  `.mochiko/brainstorms/product-architecture-schema/record.md` — Build surface, Retired/dying:
  "`architect-report-template.md` (dies or reshapes)", **settled at build as DIES** by the session
  lead; the Open-threads entry "`architect-report-template.md` 'dies or reshapes' — unruled either
  way; settled at build" is hereby discharged. Governed by **D3** (the artifact the report disclosed
  is gone), **D10** (sign-off as the write gate), **D12** (structural-origin D-XXX die into store
  deltas); `DECISIONS.md` 2026-08-19).

### Content — the retired file, verbatim and whole

`````markdown
# Architect Report Template

The architecture producer's (principal-architect) self-disclosure report — authored alongside `architecture.md` and the
structural-decision rows on each architecture round; read by the lead and the reviewer(s) to
follow what the delta proposes and what changed. Envelope + shared rules (machine-first, no
self-verdict, conditional prose): `templates/report-format.md` — this file carries only the
payload.

---

````markdown
---
report: disclosure
feature: {{feature_id}}
round: {{round}}
produced: [architecture.md, constraints-and-decisions.md#structural-decisions]
baseline: "{{seeded-from-ARCHITECTURE.md | reconstructed (confidence: high|medium|low) | greenfield-empty}}"
delta:                         # the proposed structural change, one line each; ["no structural change"] for a no-delta feature
  - "{{+ worker: settlement-processor (new box), request/response arrow from api}}"
scope: "{{full-system | delta-neighborhood (N components) — full view linked, not inlined}}"
structural_decisions: []       # the D-XXX IDs written into the structural-decisions section this round
changed_this_round:            # round 1: [initial]; later: the reviewer gap IDs addressed
  - "{{coverage: added sequence diagram for the async settlement flow}}"
governance_alignment: "{{respects BE-HEX layering per GI-XXX | the conflict + its chosen exit (redesign | ledger amendment), one line}}"
assumptions: []                # assumptions made this round, one line each with the rationale compressed in
open_questions:                # producer-surfaced unknowns feeding the clarification loop / gap routing
  - "{{the unknown, one line}}"
handoff: "{{what to grade (topology feasibility, coverage) + known soft spots, one line}}"
---
````

---

## Usage Notes

1. **This is a self-disclosure report, not a verdict.** No done-state, no PASS/FAIL, and
   deliberately no "signed-off"/"approved" field — the producer must not self-assert a state
   the lead and the user own (the envelope's no-self-verdict rule; the sign-off is the lead's
   gate, the approval the user's). The clearing verdict lives in the reviewer reports.
2. **Machine-first; the map lives in the deliverable.** `architecture.md` is where the diagram,
   the sequence diagrams, and the component register live — the reviewer grades them *there*.
   Never restate the topology here; cite component names and D-/NFR- IDs.
3. **`baseline`** discloses the current-state seed and, when reconstructed (no `ARCHITECTURE.md`),
   its confidence — the input to the baseline-confirmation gate; a reconstructed baseline is
   confirmed by the user before any delta is designed on it.
4. **`delta` + `scope`** carry the proposed change and the size-bound scoping (delta neighborhood
   vs full system); a no-delta feature discloses `["no structural change"]`, still shown for
   sign-off, never silently asserted.
5. **`changed_this_round`** — first round: `[initial]`. Later rounds: the specific reviewer gaps
   addressed by ID, so the lead can see progress and detect a stalled, no-change round.
6. **`assumptions` + `open_questions` are the producer-authored uncertainty carrier** (the
   dispatching command's producer-authored branch); **`governance_alignment`** discloses conformance to the
   CLAUDE.md governance region + its `.claude/rules/mochiko/` files, or the conflict and its
   chosen exit, in one line.
7. **`handoff`** is a *pointer*, not a claim — what to grade and where the soft spots are;
   **not** a "ready" assertion. The rendered-diagram presentation and any "presented un-rendered"
   record are the lead's, set at the sign-off gate, not disclosed here.
8. **Output location** — `.mochiko/specs/<feature>/architect-report.md`, seeded and collected
   by the lead, alongside `architecture.md`.
9. **This is a reference template** — the architecture producer (principal-architect) fills in actual content following this
   structure.
`````

*(Fence widths above are widened for nesting; the shipped original used ``` for the payload's
markdown fence.)*

### What dies with it, by ruling

- **`produced: [architecture.md, constraints-and-decisions.md#structural-decisions]`** — both
  targets are gone: the per-feature artifact by D3, the structural-decisions section by D12
  ("structural-origin D-XXX die into store deltas — the store ruling is the decision record").
- **`baseline:`** and its `seeded-from-ARCHITECTURE.md | reconstructed (confidence: …) |
  greenfield-empty` vocabulary, plus usage note 3's baseline-confirmation-gate framing — D3/D16 move
  reconstruction and confirmation to the `/mochiko:architecture` desk's first visit.
- **`structural_decisions: []`** — the D-XXX collection D12 dissolves.
- **Usage note 8's output path `.mochiko/specs/<feature>/architect-report.md`** — one limb of the
  **F5 path inconsistency** (`plan.md` said `.mochiko/features/FEAT-XXX/`, `patterns-system-design`
  said "the feature's spec dir", this template said `.mochiko/specs/<feature>/`). Per the lead's
  build note, the stale path **dies with the template rather than being repaired**; the
  `patterns-system-design` limb died in the same wave (see that file's [v0.81.0] entry B, span 1),
  leaving `plan.md` as the sole surviving authority. F5 is closed by subtraction.
- **Usage note 2's "`architecture.md` is where the diagram … lives"** — the store and the plan
  package's delta are where they live now.

### Kept deliberately

The obligations the template carried that outlive it, each with the home that now carries it:

- **The no-self-verdict rule** (usage note 1: no done-state, no PASS/FAIL, no "signed-off"/"approved"
  field; "the sign-off is the lead's gate, the approval the user's") — the shared envelope
  `templates/report-format.md` carries this rule for every producer report and is **untouched by
  this deletion**; the architecture producer's no-self-verdict obligation is not weakened, it simply
  stops being restated in a template of its own. D10 reinforces it: sign-off is the write gate, and
  a producer cannot self-assert it.
- **The no-delta disclosure** (usage note 4: "a no-delta feature discloses `["no structural
  change"]`, still shown for sign-off, never silently asserted") — the same protected obligation now
  lives as `patterns-system-design`'s relocated protected line ("The no-delta judgment is always
  shown, never made silently") plus D10 fold S13's one-line claim in the plan package, shown at
  gates. Recorded here so the audit reads the deletion as a relocation, not a drop.
- **The rendered-diagram presentation rule** (usage note 7: the presentation and any "presented
  un-rendered" record are the lead's, set at the sign-off gate) — already the lead's, held in
  `plan.md`'s sign-off gate, which D10 re-targets at the store delta (P2's re-key).
- **`assumptions` / `open_questions` as the producer-authored uncertainty carrier** — the dispatching
  command's producer-authored branch, named in usage note 6; that branch lives in the command, not
  here.

### Consumers assessed

**Zero live references.** `grep -rl "architect-report" plugins/ .claude/` (worktree copies excluded)
returns only the template itself; `plugins/mochiko/commands/plan.md` contains no occurrence of
"architect report", "architect-report", or "disclosure report"; the router
`skills/mochiko/SKILL.md` registers `analyst-report-template`, `advocate-report-template`,
`techanalyst-report-template`, and `feasibility-report-template` but has **no row for this one**;
`plugin.json` enumerates agents only (`"commands"`/`"skills"` are directory globs, templates
unlisted), so the deletion needs no manifest change. The template was already orphaned in the
shipped tree before this wave — the deletion removes dead weight as well as a superseded primitive.

**Prior-entry housekeeping:** the [v0.67.0] entry's flagged recommendation — "recommend `git mv` of
this log to `strips/architect-report-template.md` at ripple for filename-parity" — was in fact
carried out (this log lives at `.mochiko/strips/architect-report-template.md`); the flag is closed.
This log file is retained as the primitive's history anchor now that the primitive is gone.

## [v0.67.0] Seat rotation — renamed to `architect-report-template.md`; seat re-keyed system-architect → architecture producer (principal-architect)
- **Disposition:** superseded → `templates/architect-report-template.md` (git mv rename) with the seat naming re-keyed in place; the report machinery survives, only the retired seat name rotates.
- **Tier failed:** n/a — supersession by ruling (record `.mochiko/brainstorms/architect-role-pushback-and-abstraction/record.md` **D1** — the two-architect split dies; the architecture producer is now `principal-architect`; `DECISIONS.md` 2026-08-13 row L13).
- **Content (verbatim re-keys — retired-seat text → rotated text):**
  - Title: `# System Architect Report Template` → `# Architect Report Template`
  - Header: `The system-architect's self-disclosure report — authored alongside \`architecture.md\` and the …` → `The architecture producer's (principal-architect) self-disclosure report — authored alongside \`architecture.md\` and the …`
  - Usage note 8 (output location): `.mochiko/specs/<feature>/sysarchitect-report.md` → `.mochiko/specs/<feature>/architect-report.md`
  - Usage note 9: `the system-architect fills in actual content following this structure` → `the architecture producer (principal-architect) fills in actual content following this structure`
- **Kept deliberately:** the entire payload YAML block (report/feature/round/produced/baseline/delta/scope/structural_decisions/changed_this_round/governance_alignment/assumptions/open_questions/handoff) and every self-disclosure / no-self-verdict / machine-first usage rule — untouched; the payload names no seat, so the template's function is unchanged, only its producing seat rotated. The v0.46.0 entry below stands.
- **Consumers assessed (grep of `plugins/` for old filename + output name):** no live command/skill/template references `sysarchitect-report-template.md` or the output `sysarchitect-report.md` by name — `grep -rn` over `plugins/` returned only this template's own (now re-keyed) self-references; the plan-cluster harness collects the architect report generically. Out-of-cluster references left for the lead: `.mochiko/strips/plan.md` L844 (a historical [vX] entry naming the old filename + seat — a stale historical reference in the plan cluster's file, not amended here); router `skills/mochiko/SKILL.md` + `plugin.json` (the lead's ripple). `.mochiko/brainstorms/verbosity-caveman-ops-separation/record.md` L145 is a frozen point-in-time template census — not a live pointer, left as-is.
- **Strip-file rename (flagged):** recommend `git mv` of this log to `strips/architect-report-template.md` at ripple for filename-parity with the renamed primitive; left at the original path per the fix-round instruction.

## [v0.46.0] "the shape's producer-authored branch" re-pointed (audit finding 2)
- **Disposition:** superseded → "the dispatching command's producer-authored branch"
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row) — audit-caught consumer, fixed at re-grade
- **Content:** rule 6's parenthetical "(the shape's producer-authored branch)" — the shape home was deleted this wave; the ADR's re-point list missed this consumer, caught by the wave audit.
- **Consumers assessed:** the producing seat's briefs unchanged.
