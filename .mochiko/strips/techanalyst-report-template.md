# Strip notes — `templates/techanalyst-report-template.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (rulings
ratified 2026-07-23: producer disclosures machine-first).

## [v0.77.0] Template superseded by schema — `schemas/techanalyst-report.yaml` + `mochiko-cli template techanalyst-report`
- **Disposition:** superseded → `schemas/techanalyst-report.yaml` + `mochiko-cli template techanalyst-report` (D8 raw-Read fallback when the binary is absent)
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D3 later-ratchet + user ruling 2026-08-16; `DECISIONS.md` "Template-schema ratchet" row; record `.mochiko/brainstorms/schema-based-template-guidance/record.md`)
- **Ratchet context:** the D3 later-ratchet exercised over the Class A seat report templates per the user ruling 2026-08-16 (against DM scope-breadth caution + the open n=0 first-live-run watch). Same mechanism as the v0.76.0 first wave: the schema data file is the source of truth and the binary renders the producer + `--check` views over it. **Source-file deletion is PHASE-2-gated** — the source file remains on disk until the fidelity audit (V1) PASSES; this entry records the supersession now so the record is complete before deletion.
- **Schema mapping (M1):** `sections` = the payload frontmatter field-clusters (NOT prose chapters); the Usage Notes fold into the per-section `contract:` + the `overview:`; `skeleton:` = the frontmatter block + the conditional `## Notes of note` prose block; `form: report-format.md` stays the single home for the shared report doctrine (register / findings-schema / conditional-prose / no-self-verdict) — pointer only, NOT restated as per-section checks. The `--check` lines are authored NET-NEW (reports have no validator/checklist consumer; disclosed).
- **Content (VERBATIM — the superseded source, reproduced for GI-006 reconstruction before the phase-2 deletion):**
~~~markdown
# Technical Analyst Report Template

The technical-analyst's self-disclosure report — authored alongside the analysis/design
artifacts on each round; read by the lead and the reviewer(s) to follow what was produced
and what changed. Envelope + shared rules (machine-first, no self-verdict, conditional
prose): `templates/report-format.md` — this file carries only the payload.

---

```markdown
---
report: disclosure
feature: {{feature_id}}
phase: analysis | design       # which phase this round produced — discloses, never drives the sequence
round: {{round}}
produced: [requirements.md, constraints-and-decisions.md, nfrs.md]   # design rounds: [data-model.md, contracts/api.yaml, quickstart.md]
changed_this_round:            # round 1 of a phase: [initial]; later: the reviewer gap IDs addressed
  - "{{G2: NFR-003 given a numeric latency target}}"
governance_alignment: "{{aligned | the rules/GI IDs touched and any exception, one line}}"
assumptions: []                # assumptions made this round, one line each with the rationale compressed in
open_questions:                # producer-surfaced unknowns feeding the clarification loop / gap routing
  - "{{the unknown, one line}}"
handoff: "{{which artifacts to grade + known soft spots, one line}}"
---
```

---

## Usage Notes

1. **This is a self-disclosure report, not a verdict.** No done-state, no PASS/FAIL, and
   deliberately no "Completion"/"ready" field — the producer must not self-assert a state
   the lead owns (the envelope's no-self-verdict rule). The clearing verdict lives in the
   reviewer report(s); the lead owns the loop decision.
2. **Machine-first; the lead and reviewer(s) read the fields.** A routine round is
   frontmatter-only. Add `## Notes of note` only for genuine difficulties or non-obvious
   calls. The artifacts themselves are what the reviewer(s) grade — never restate their
   content here (cite TR-/C-/NFR-/D- IDs).
3. **`phase`** discloses which phase this round produced (`analysis` or `design`); the lead
   owns the two-phase sequence.
4. **`changed_this_round`** — first round of a phase: `[initial]`. Later rounds: the
   specific reviewer gaps addressed by ID, so the lead can see progress and detect a
   stalled, no-change round.
5. **`handoff`** is a *pointer*, not a claim — what to grade and where the soft spots are;
   **not** a "ready" assertion.
6. **`assumptions` + `open_questions` are the producer-authored uncertainty carrier** (the
   dispatching command's producer-authored branch): assumptions record what was assumed and why;
   open questions feed the clarification loop, where the reviewer stress-tests them.
   **`governance_alignment`** discloses alignment to the CLAUDE.md governance region + its
   `.claude/rules/mochiko/` files in one line.
7. **Output location** — `.mochiko/specs/<feature>/techanalyst-report.md`, seeded and
   collected by the lead, alongside the analysis/design artifacts.
8. **This is a reference template** — the technical-analyst fills in actual content
   following this structure.
~~~
- **Kept deliberately:** nothing dropped — every frontmatter field and every Usage Note has a home in the schema (fields in `skeleton:` + section `contract:`; notes folded into `contract:`/`overview:`). The `form: report-format.md` envelope pointer is preserved. The no-self-verdict + no-Completion/ready-field emphasis (Note 1) and phase-disclosure semantics (Note 3) are preserved (overview + Phase & production contract).
- **Consumers assessed:** router row `skills/mochiko/SKILL.md:86` (techanalyst). No other static consumer named (plan §4). Re-points are owned by seat P5 (router/consumer rows, two-arm `mochiko-cli template techanalyst-report; if absent Read plugins/mochiko/schemas/techanalyst-report.yaml`) — NOT this seat; each re-pointed consumer appends its own supersession strip.

## [v0.46.0] "the shape's producer-authored branch" re-pointed (audit finding 1)
- **Disposition:** superseded → "the dispatching command's producer-authored branch"
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row) — audit-caught consumer, fixed at re-grade
- **Content:** rule 6's parenthetical "(the shape's producer-authored branch)" — the shape home was deleted this wave; the ADR's re-point list missed this consumer, caught by the wave audit.
- **Consumers assessed:** the producing seat's briefs unchanged.

## [v0.22.0] Prose disclosure sections → frontmatter fields
- **Disposition:** contracted in place (template rewritten)
- **Tier failed:** consumption evidence (epic F-c part 2)
- **Content:** `## What Was Produced` prose → the `produced:` artifact list (the artifacts themselves are what reviewers grade); `## What Changed This Round` prose → `changed_this_round:` ID-cited list; `## Governance Alignment` prose → `governance_alignment:` one-liner; `## Open Questions` prose → `open_questions:` list; `## Handoff to Review` prose → `handoff:` one-line pointer; the optional `## Artifacts Produced` table (folded into `produced:`). The "Foreground prose… no parser" usage note reversed to machine-first. Preserved: no-self-verdict (no Completion/ready field), phase disclosure semantics, handoff-is-a-pointer-not-a-claim, output location.
