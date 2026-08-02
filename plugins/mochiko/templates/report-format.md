# Report Format — the mochiko report envelope

The single authoritative home of the shape every mochiko workflow **report** follows —
cycle reports, verification reports, reviewer reports, and producer disclosures. Report
templates and report-producing skills reference this file for the envelope and the shared
rules; each carries only its own payload schema. (Deliverables — spec.md, plan.md,
tasks.md, slices.md, the working code — are not reports and are not governed here.)

## Who reads a report

The lead, forming a verdict or relaying a failure/gap list — a model, not a human. The
user receives lead-authored summaries, never report files. So reports are **machine-first**:
every consumed datum is a structured frontmatter field; prose exists only where structure
cannot carry the content (see the conditional-prose rule). Do not write for a human-style
reader; write fields a verdict can be formed from.

## The envelope

Every report is a `.md` file opening with YAML frontmatter:

```yaml
---
report: cycle | verification | final-validation | review | feasibility | disclosure
feature: <feature-id>
slice: <s#>            # only when the run is slice-scoped
round: <n>             # the lead's round counter (cycle reports carry cycle: + attempt: instead)
---
```

The payload fields per report type live in that type's own home (the report's template or
skill reference), never here. Unknown extra fields are permitted; missing required payload
fields are not.

## Shared rules

1. **Machine-first.** Every datum a verdict, gate, or relay consumes is a structured
   field. A fact that fits a field never appears as prose.
2. **Conditional prose — the failure exception.** Prose sections appear only when they
   carry content structure cannot: a **failure narrative** (mandatory when the report
   carries a FAIL / blocking finding: what failed, why, what was tried — debug value
   concentrates exactly there); **notes of note** (non-obvious decisions, difficulties,
   flagged blockers — only when non-empty); a **null-exit reasoning** block where a
   workflow defines one. A clean/passing report is frontmatter-only. **The set is closed
   for cycle and verification reports** — these three, nothing else. A body section outside
   them is a defect there, not an author's judgment call; a fact that wants one belongs in
   a field. Each payload home names which of its own sections carry the three.
3. **Omit empty.** An empty section or list-valued field with nothing to say is omitted
   (or `[]`), never written as "None" prose.
4. **No self-verdict (producer disclosures).** A `disclosure` report records what the
   producer did and what it is unsure of; it carries **no done-state and no PASS/FAIL**.
   The recommended verdict lives in reviewer reports; the clearing decision is the
   lead's. A disclosure must not gain a "ready"/"complete" field.
5. **No restatement.** Cite upstream IDs (`T4.2`, `FR-003`, `C-012`, `SC-005`) — never
   re-quote their text. One-line context is allowed only where a bare ID would be
   unreadable at the consumption point.
6. **Findings schema (reviewer reports).** Each finding is one structured entry:

   ```yaml
   findings:
     - {id: G1, type: <taxonomy>, sev: Critical|Important|Minor,
        at: "<artifact + ID/anchor>", gap: "<the defect, one line>",
        fix: "<the proposed resolution, one line>"}
   ```

   A report type may add fields a gate renders (e.g. `impact:`); it never drops these.
   `strengths:` is a **single line** (comma-brief) — the reviewer's evidence of engaging
   with what works, not a prose section.
7. **Evidence is captured, not narrated.** Slim reports do not reduce evidence *capture*
   — full output lives in logs/scratch with pointers from the report; the report carries
   results and bounded excerpts only where a failure requires them.
8. **Register — `ultra`, except the failure narrative.** Reports write `ultra`. The
   **failure narrative writes `full`** — it is what a lead scopes a retry from, so what
   failed, why, what was tried, and the state things were left in survive whole, never
   thinned to `ultra`. Levels, the clause manifest, the ambiguity guardrail and the switch:
   `templates/output-style.md`.
9. **Prose on a clean report is a defect.** `status: pass` plus body content outside rule
   2's set is **not a clean report**: it fails the deterministic-and-clean clearing
   conditions (the dispatching command's devolved-branch terms — `commands/implement.md`'s
   cycle checkpoint) and returns to the lead instead of devolving.
   Read mechanically — status and section headings, never prose quality. This is the
   report layer's twin of `artifact-format.md` rule 8: on a report, prose the envelope
   does not sanction *is* a finding.

---

**Format version:** v2 (2026-08-01) · **Consumed by:** the report templates in this
directory, `executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md`,
`testing-end-user/references/REPORT-TEMPLATES.md`.
