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
