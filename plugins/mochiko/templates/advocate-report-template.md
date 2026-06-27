# Advocate Report Template

This template defines the structure for the Advocate Report — a reviewer's adversarial critique of a draft spec, recording the gaps found, clarifications needed, a verdict, and the spec's strengths.

---

```markdown
# Advocate Report

> Feature: {{feature_id}}
> Round: {{round}}
> Generated: {{timestamp}}

---

## Gaps Found

| ID | Type | Description | Severity |
|----|------|-------------|----------|
| G1 | {{type}} | {{description}} | {{severity}} |

**Severity levels**: Critical | Important | Minor

**Gap types**: Missing | Ambiguous | Edge Case | Assumption | Contradiction

---

## Clarifications Needed

### C1: {{question_title}}

**Related Gap**: {{gap_id}}

**Question**: {{question}}

**Options**:
1. {{option_1}}
2. {{option_2}}
3. {{option_3}}

**Why it matters**: {{impact}}

---

## Verdict

**Status**: {{verdict}}

**Verdict options**: ready | needs-revision | critical-gaps

**Rationale**: {{verdict_rationale}}

---

## What's Strong

{{strengths}}
```

---

## Usage Notes

1. **Placeholders**: `{{mustache}}` style — the reviewer replaces each with actual content; this is a reference shape, not literal output.
2. **Severity levels**: `Critical` (blocking), `Important` (should fix), `Minor` (cosmetic).
3. **Gap types**: `Missing`, `Ambiguous`, `Edge Case`, `Assumption`, `Contradiction` — the category recorded for each gap.
4. **Verdict values** — the reviewer's judgment of the spec under review:
   - `ready` — no blocking gaps; the spec is complete and internally consistent.
   - `needs-revision` — addressable gaps remain in an otherwise sound spec.
   - `critical-gaps` — fundamental problems with the spec's foundation, not just fixable gaps.

