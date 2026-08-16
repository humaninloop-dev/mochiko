# Strip notes — `templates/advocate-report-template.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md`; wave rulings ratified 2026-07-23:
all report formats machine-first YAML, strengths → one-line field).

## [v0.77.0] Template superseded by schema — `schemas/advocate-report.yaml` + `mochiko-cli template advocate-report`
- **Disposition:** superseded → `schemas/advocate-report.yaml` + `mochiko-cli template advocate-report` (the D8 raw-Read fallback reads the schema directly when the binary is absent)
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D3 later-ratchet + user ruling 2026-08-16; `DECISIONS.md` "Template-schema ratchet" row; record `.mochiko/brainstorms/schema-based-template-guidance/record.md`)
- **Ratchet context:** the D3 later-ratchet exercised over the Class A seat report templates per the user ruling 2026-08-16 (against DM scope-breadth caution + the open n=0 first-live-run watch). Same mechanism as the v0.76.0 first wave: the schema data file is the source of truth and the binary renders the producer + `--check` views over it. **Source-file deletion is PHASE-2-gated** — the source file remains on disk until the fidelity audit (V1) PASSES; this entry records the supersession now so the record is complete before deletion.
- **Schema mapping (M1):** `sections` = the payload frontmatter field-clusters (NOT prose chapters); the Usage Notes fold into the per-section `contract:` + the `overview:`; `skeleton:` = the frontmatter block + the conditional Clarifications-needed prose block; `form: report-format.md` stays the single home for the shared report doctrine (register / findings-schema / conditional-prose / no-self-verdict) — pointer only, NOT restated as per-section checks. The `--check` lines are authored NET-NEW (reports have no validator/checklist consumer; disclosed).
- **Content (VERBATIM — the superseded source, reproduced for GI-006 reconstruction before the phase-2 deletion):**
~~~markdown
# Advocate Report Template

The reviewer's adversarial critique of an artifact under review — gaps found, clarifications
needed, and a recommended verdict. The shared emit shape for the specify / plan / tasks /
slice review seats. Envelope + shared rules (machine-first, findings schema, conditional
prose): `templates/report-format.md` — this file carries only the review payload.

---

```markdown
---
report: review
feature: {{feature_id}}
round: {{round}}
incremental: false            # true when this round reviews only a delta (e.g. plan Phase-2 incremental mode) — name the scope in `scope:`
verdict: ready | needs-revision | critical-gaps
verdict_basis: "{{one line — what drives the verdict}}"
strengths: "{{one line, comma-brief — what genuinely works}}"
findings:
  - {id: G1, type: Missing | Ambiguous | EdgeCase | Assumption | Contradiction,
     sev: Critical | Important | Minor,
     at: "{{artifact + ID/section anchor}}",
     gap: "{{the defect, one line}}",
     fix: "{{proposed resolution, one line}}"}
---

## Clarifications needed   <!-- only when user input is required; omit otherwise -->

### C1: {{question_title}}   ({{gap_id}})

**Question**: {{question}}
**Options**: 1. {{option_1}} · 2. {{option_2}} · 3. {{option_3}}
**Why it matters**: {{impact}}
```

---

## Usage Notes

1. **The findings list is the relay payload** — the lead carries it to the producer
   verbatim as the gap list. Every finding cites its location by ID/anchor (`at:`), never
   by re-quoting artifact text.
2. **Verdict values** — the reviewer's judgment of the artifact under review, a
   RECOMMENDED verdict (the clearing decision is the lead's):
   - `ready` — no blocking gaps; complete and internally consistent.
   - `needs-revision` — addressable gaps remain in an otherwise sound artifact.
   - `critical-gaps` — fundamental problems with the foundation, not just fixable gaps.
3. **Clarifications are gate fuel.** When a gap needs a user ruling, frame it as a
   product question with concrete options and why-it-matters — the lead renders these at
   the clarification gate. Omit the section when nothing needs the user.
4. **`strengths:` is one line, not a section** — evidence the review engaged with what
   works, kept comma-brief.
5. **`incremental:`/`scope:`** — set on rounds that review a delta rather than the full
   artifact set (e.g. plan's Phase-2 incremental mode), so the verdict's coverage is
   honest.
6. **Zero findings with a `ready` verdict** still requires `verdict_basis` and
   `strengths` — a clean report is frontmatter-only, never empty.
~~~
- **Kept deliberately:** nothing dropped — every frontmatter field and every Usage Note has a home in the schema (fields in `skeleton:` + section `contract:`; notes folded into `contract:`/`overview:`). The `form: report-format.md` envelope pointer is preserved.
- **Consumers assessed:** router row `skills/mochiko/SKILL.md:64` (advocate) + the `:89` reuse note; advocate-report consumers `review-plan-artifacts/SKILL.md:128,211`, `review-plan-artifacts/references/ISSUE-TEMPLATES.md:5,84,132`, `review-specifications/SKILL.md:39,132`. Re-points are owned by seat P5 (router/consumer rows, two-arm `mochiko-cli template advocate-report; if absent Read plugins/mochiko/schemas/advocate-report.yaml`) — NOT this seat; each re-pointed consumer appends its own supersession strip.

## [v0.22.0] Prose report shape → machine-first findings YAML; What's Strong → `strengths:` field
- **Disposition:** contracted in place (template rewritten); `What's Strong` prose section → the one-line `strengths:` frontmatter field (user-ruled: keep the anti-rubber-stamp discipline at one line)
- **Tier failed:** consumption evidence (epic F-c part 2): round reports are consumed in-round by the lead's verdict and relayed as gap lists; no downstream stage reads them
- **Content:** the markdown Gaps Found table (ID/Type/Description/Severity) → the `findings:` YAML list (same taxonomy: Missing/Ambiguous/EdgeCase/Assumption/Contradiction; severities unchanged); the `## Verdict` prose block (Status/Rationale) → `verdict:` + `verdict_basis:` fields (same three states: ready/needs-revision/critical-gaps); `## What's Strong` free prose → `strengths:` one-liner. Preserved: Clarifications-Needed with concrete options + why-it-matters (gate fuel), the recommended-not-clearing verdict doctrine. Added: `incremental:`/`scope:` fields (review-plan-artifacts' Phase-2 incremental mode, formerly an inline divergent shape in that skill).
- **Re-add trigger:** a lead verdict or producer revision demonstrably starved by the one-line findings compression (evidence-gated, marked override).
