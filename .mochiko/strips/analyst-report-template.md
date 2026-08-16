# Strip notes — `templates/analyst-report-template.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (rulings
ratified 2026-07-23: producer disclosures machine-first; "foreground prose / no parser"
doctrine reversed).

## [v0.77.0] Template superseded by schema — `schemas/analyst-report.yaml` + `mochiko-cli template analyst-report`
- **Disposition:** superseded → `schemas/analyst-report.yaml` + `mochiko-cli template analyst-report` (D8 raw-Read fallback when the binary is absent)
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D3 later-ratchet + user ruling 2026-08-16; `DECISIONS.md` "Template-schema ratchet" row; record `.mochiko/brainstorms/schema-based-template-guidance/record.md`)
- **Ratchet context:** the D3 later-ratchet exercised over the Class A seat report templates per the user ruling 2026-08-16 (against DM scope-breadth caution + the open n=0 first-live-run watch). Same mechanism as the v0.76.0 first wave: the schema data file is the source of truth and the binary renders the producer + `--check` views over it. **Source-file deletion is PHASE-2-gated** — the source file remains on disk until the fidelity audit (V1) PASSES; this entry records the supersession now so the record is complete before deletion.
- **Schema mapping (M1):** `sections` = the payload frontmatter field-clusters (NOT prose chapters); the Usage Notes fold into the per-section `contract:` + the `overview:`; `skeleton:` = the frontmatter block + the conditional `## Notes of note` prose block; `form: report-format.md` stays the single home for the shared report doctrine (register / findings-schema / conditional-prose / no-self-verdict) — pointer only, NOT restated as per-section checks. The `--check` lines are authored NET-NEW (reports have no validator/checklist consumer; disclosed).
- **Content (VERBATIM — the superseded source, reproduced for GI-006 reconstruction before the phase-2 deletion):**
~~~markdown
# Analyst Report Template

The producer's self-disclosure report — authored alongside `spec.md` on each round; the
lead reads it to follow what was produced and what changed. Envelope + shared rules
(machine-first, no self-verdict, conditional prose): `templates/report-format.md` — this
file carries only the analyst payload.

---

```markdown
---
report: disclosure
feature: {{feature_id}}
round: {{round}}
produced: [spec.md]
changed_this_round:            # round 1: [initial]; later rounds: the gap IDs / changes addressed
  - "{{G3: added expiry edge case to US2}}"
assumptions:                   # one line each, with the rationale compressed in
  - {id: A1, assumption: "{{assumption}}", why: "{{rationale, one line}}"}
open_questions: []             # producer-surfaced unknowns feeding the clarification loop
handoff: "{{what the critic should grade + any known soft spots, one line}}"
---
```

---

## Usage Notes

1. **This is a self-disclosure report, not a verdict.** It records what the producer
   authored and assumed; it carries no PASS/FAIL and no done-state (per the envelope's
   no-self-verdict rule). The clearing verdict lives in the critic's review, and the lead
   owns the loop decision — never read this report as a gate.
2. **Machine-first; the lead is the reader.** The frontmatter is the report — a round with
   nothing unusual needs no prose. Add a `## Notes of note` block only for genuine
   difficulties or non-obvious calls the fields can't carry.
3. **`round`** is the lead's bounded-loop round counter: round 1 is the first draft; later
   rounds are revisions following a critic review.
4. **`changed_this_round`** — round 1: `[initial]`. Later rounds: the specific gaps
   addressed (by the critic's gap IDs), so the lead can see progress and detect a stalled,
   no-change round. Cite IDs; never restate spec text (the spec is the artifact the critic
   and lead read directly).
5. **Output location** — `.mochiko/specs/<feature>/analyst-report.md`, seeded and collected
   by the lead.
6. **This is a reference template** — the producer fills in actual content following this
   structure.
~~~
- **Kept deliberately:** nothing dropped — every frontmatter field and every Usage Note has a home in the schema (fields in `skeleton:` + section `contract:`; notes folded into `contract:`/`overview:`). The `form: report-format.md` envelope pointer is preserved. The no-self-verdict emphasis (Usage Note 1) is carried in `overview:` as a pointer to report-format.md rule 4, not restated as a check.
- **Consumers assessed:** router row `skills/mochiko/SKILL.md:63` (analyst). No other static consumer named (plan §4). Re-points are owned by seat P5 (router/consumer rows, two-arm `mochiko-cli template analyst-report; if absent Read plugins/mochiko/schemas/analyst-report.yaml`) — NOT this seat; each re-pointed consumer appends its own supersession strip.

## [v0.22.0] Prose disclosure sections → frontmatter fields
- **Disposition:** contracted in place (template rewritten)
- **Tier failed:** consumption evidence (epic F-c part 2): read in-round by the lead (progress/stall) and relayed; terminal once the round closes
- **Content:** `## Summary` free prose (dropped — the spec is what the critic/lead read); `## Assumptions Made` table → `assumptions:` list ({id, assumption, why} one-liners); `## What Changed This Round` prose → `changed_this_round:` ID-cited list (stall detection preserved); `## Notes` free prose → the conditional `## Notes of note` block (only when non-empty); the optional `## What I Created` count table (dropped — a convenience disclosure of counts the critic reads from spec.md directly). The "Foreground prose; write for a human-style reader… no parser" usage note reversed to machine-first. Preserved: no-self-verdict doctrine (now via the envelope), round semantics, output location.
