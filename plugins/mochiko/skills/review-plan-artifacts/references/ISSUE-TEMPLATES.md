# Issue Templates and Classification

Issue-documentation formats, severity classification rules, and the working report shape for plan
artifact reviews. These are the reviewer's working formats; the **assembled deliverable** the lead
reads is the `mochiko:advocate-report-template` (see [Assembled report](#assembled-report) below) —
do not restate that template here.

## Severity Levels

| Severity | Definition | Action |
|----------|------------|--------|
| **Critical** | Blocks progress; must resolve | Return to the responsible producer |
| **Important** | Significant gap; should resolve | Flag for this round |
| **Minor** | Polish item; can defer | Note for later |

## Classification Rules

```
CRITICAL if:
- Missing entity/endpoint from requirements
- Unresolved NEEDS CLARIFICATION marker
- No alternatives considered for major decision
- Security/privacy issue (unmarked PII, etc.)
- Broken traceability (can't trace FR to implementation)
- Design contradicts a decided approach (consistency failure)

IMPORTANT if:
- Missing validation rules
- Incomplete error handling
- Weak rationale documentation
- State machine gaps
- Brownfield misalignment

MINOR if:
- Naming inconsistency
- Missing examples
- Documentation formatting
- Non-critical missing details
```

---

## Issue Documentation Format

### Individual Issue Template

```markdown
### Issue: [Short description]

**Severity**: Critical | Important | Minor
**Artifact**: [Which file has the issue]
**Check Failed**: [Which review check]

**Problem**:
[What is wrong or missing]

**Evidence**:
[Where this gap appears - cite requirements or artifacts]

**Impact**:
[What happens if not resolved]

**Suggested Fix**:
[How the responsible producer should address this]
```

### Issue Table Format

```markdown
## Issues Found

| # | Severity | Artifact | Description | Suggested Action |
|---|----------|----------|-------------|------------------|
| 1 | Critical | data-model.md | Session entity missing expiresAt | Add expiresAt timestamp |
| 2 | Important | contracts/api.yaml | No 429 error for login | Add rate limit response |
| 3 | Minor | contracts/api.yaml | Inconsistent path naming | Standardize to kebab-case |
```

---

## Working Report Shape

While executing the review, organize findings with this working shape. The final report handed to
the lead is assembled with the `advocate-report-template` — see below.

```markdown
## Plan Artifact Review: {artifact set reviewed}

**Artifacts Reviewed**: {file paths}

---

### Verdict: [ready | needs-revision | critical-gaps]

---

### Critical Issues

[Issues that MUST be resolved before proceeding]

| # | Issue | Evidence | Suggested Fix |
|---|-------|----------|---------------|
| 1 | [Description] | [Source] | [Action] |

---

### Important Issues

[Issues that SHOULD be addressed in this round]

| # | Issue | Evidence | Suggested Fix |
|---|-------|----------|---------------|

---

### Minor Issues

[Polish items that can be deferred]

| # | Issue | Suggested Fix |
|---|-------|---------------|

---

### Cross-Artifact Concerns

[Issues that span multiple artifacts]
```

### Assembled report

The deliverable report the lead reads is structured with `mochiko:advocate-report-template`
(the shared reviewer shape, machine-first: verdict + basis, severity-classified `findings:` YAML,
clarifying questions, the one-line `strengths:` field). The one plan-specific addition is the
incremental-mode `consistency_checks:` frontmatter block (SKILL.md → Incremental Review Mode),
which is this skill's, not the template's. Reference the template for everything else rather than
restating its structure.

---

## Verdict Criteria

The verdict is derived **mechanically** from the issue counts (no judgment in the mapping itself):

### ready

- Zero Critical issues
- Zero Important issues (or all auto-resolved)
- Minor issues documented for future

### needs-revision

- 1–3 Important issues to resolve
- Minor issues affecting usability
- Gaps that can be fixed in one round

### critical-gaps

- 1+ Critical issues
- 4+ Important issues
- Missing major artifact section
- Unrecoverable in a single round

---

## Anti-Patterns to Avoid

- **Implementation focus**: Review design, not code details
- **Vague issues**: "Could be better" - specify what and how
- **Severity inflation**: Not everything is Critical
- **Missing evidence**: Issues need citations to artifacts
- **No suggestions**: Every issue needs a path to resolution
- **Ignoring strengths**: Acknowledge good work
- **Rubber stamping**: Being too agreeable defeats the purpose
- **Reaching into feasibility**: Buildability/contradiction findings belong to
  `mochiko:review-feasibility`; note them as cross-artifact concerns and let that reviewer own them
