# Strip notes — `templates/architect-report-template.md` (formerly `sysarchitect-report-template.md`, renamed v0.67.0)

Entry formats: `strips/README.md`. This log file retains its original name
(`strips/sysarchitect-report-template.md`) as the primitive's history anchor; the primitive it
tracks is now `templates/architect-report-template.md`.

---

## [v0.77.0] Template superseded by schema — `schemas/architect-report.yaml` + `mochiko-cli template architect-report`
- **Disposition:** superseded → `schemas/architect-report.yaml` + `mochiko-cli template architect-report` (D8 raw-Read fallback when the binary is absent)
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D3 later-ratchet + user ruling 2026-08-16; `DECISIONS.md` "Template-schema ratchet" row; record `.mochiko/brainstorms/schema-based-template-guidance/record.md`)
- **Ratchet context:** the D3 later-ratchet exercised over the Class A seat report templates per the user ruling 2026-08-16 (against DM scope-breadth caution + the open n=0 first-live-run watch). Same mechanism as the v0.76.0 first wave: the schema data file is the source of truth and the binary renders the producer + `--check` views over it. **Source-file deletion is PHASE-2-gated** — the source file remains on disk until the fidelity audit (V1) PASSES; this entry records the supersession now so the record is complete before deletion.
- **Schema mapping (M1):** `sections` = the payload frontmatter field-clusters (NOT prose chapters); the Usage Notes fold into the per-section `contract:` + the `overview:`; `skeleton:` = the frontmatter block (architect is frontmatter-only — its Usage Notes define no sanctioned prose block, so the skeleton is the frontmatter block alone); `form: report-format.md` stays the single home for the shared report doctrine (register / findings-schema / conditional-prose / no-self-verdict) — pointer only, NOT restated as per-section checks. The `--check` lines are authored NET-NEW (reports have no validator/checklist consumer; disclosed).
- **Content (VERBATIM — the superseded source, reproduced for GI-006 reconstruction before the phase-2 deletion):**
~~~markdown
# Architect Report Template

The architecture producer's (principal-architect) self-disclosure report — authored alongside `architecture.md` and the
structural-decision rows on each architecture round; read by the lead and the reviewer(s) to
follow what the delta proposes and what changed. Envelope + shared rules (machine-first, no
self-verdict, conditional prose): `templates/report-format.md` — this file carries only the
payload.

---

```markdown
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
```

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
~~~
- **Kept deliberately:** nothing dropped — every frontmatter field and every Usage Note has a home in the schema (fields in `skeleton:` + section `contract:`; notes folded into `contract:`/`overview:`). The `form: report-format.md` envelope pointer is preserved. All 9 Usage Notes homed (no-self-verdict + no signed-off field → overview; baseline/delta/scope/governance/uncertainty/handoff → their sections). The rendered-diagram/sign-off-is-lead's note (Note 7) rides the Handoff `contract:`.
- **Consumers assessed:** NO static consumer pointer — `grep -rn` over `plugins/` is clean (the architect report is dispatched via the `plan.md` brief, not a named pointer); no router row exists for it. The template's own `templates/architect-report-template.md:58` output-location self-reference dies with the file. No P5 re-point owed for this schema.

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
