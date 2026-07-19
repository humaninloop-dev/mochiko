---
name: validation-constitution
description: This skill MUST be invoked to grade a DRAFTED governance surface set against the quality checklist — there is NO constitution.md; the graded set is the CLAUDE.md governance region (between the mochiko:governance markers), the `paths`-scoped `.claude/rules/mochiko/` files, and the governance ledger (`.mochiko/memory/governance-ledger.md`), judged against the session synthesis (`.mochiko/memory/governance-intent.md`) and the producer's trace summary manifest. Checks: two-way trace closure over the manifest (every GI element → one primary home + companion entries; every surface element → GI), region-marker integrity, index→home existence, per-principle three-part structure in the ledger (enforcement/testability/rationale), tier-declaration and waiver-format checks, module-parameterized checks, anti-pattern scan, placeholder scan, quantification enforcement, and semantic version-bump determination — emitting a binary PASS/FAIL verdict plus a fix list. SHOULD also invoke whenever the setup loop's validate step needs an independent grade of a surface set produced by mochiko:authoring-constitution (greenfield or brownfield mode), or when re-validating after a FAIL-loop revision. The validator-side skill of the governance producer↔validator pair; defaults to FAIL; run by an independent validator (a different agent than the author), never the author.
---

# Validating Constitution

## Overview

Governance validation ensures the surface set is enforceable, testable, trace-closed, and free of
anti-patterns before finalization. **The graded artifact is a set, not a file** — the CLAUDE.md
governance region, the rules files, and the ledger are one deliverable and are graded together;
grading only the region is partial validation, which is not validation. Every set MUST pass — no
exceptions for "simple projects" or "tight deadlines."

**Violating the letter of the rules is violating the spirit of the rules.**

Skipping validation because "the constitution looks fine" or "it's mostly complete" is not following the spirit of quality assurance—it is abandoning it.

## When to Use

- After drafting a constitution with mochiko:authoring-constitution (greenfield or brownfield mode)
- Before presenting a constitution to users for approval
- When updating an existing constitution (any change requires re-validation)
- When user explicitly requests constitution review or quality check
- When determining appropriate version bump for constitution changes
- When auditing existing constitution for anti-patterns

## When NOT to Use

- During initial constitution drafting (validate AFTER drafting, not during)
- For documents that are not constitutions (specs, plans, code)
- When user only wants to READ a constitution without validation
- For informal project notes or temporary governance sketches

## Core Process

The inputs, all read from file, never from the author's report: the **surface set** — the
CLAUDE.md governance region (between `<!-- mochiko:governance:begin -->` /
`<!-- mochiko:governance:end -->`), every rules file under `.claude/rules/mochiko/`, and the
ledger (`.mochiko/memory/governance-ledger.md`) — plus the session synthesis
(`.mochiko/memory/governance-intent.md`, the traceable contract) and the producer's **trace
summary manifest**. A missing synthesis when the set carries trace keys, a missing manifest, or a
missing member of the set — each is itself a FAIL. A `.mochiko/memory/constitution.md` on disk is
a superseded artifact the lead should have deleted — flag it in the fix list.

### Step 1: Load Quality Checklist and Assemble It

Read [references/QUALITY-CHECKLIST.md](references/QUALITY-CHECKLIST.md). The structure checks are
**module-parameterized**: read the synthesis's module selections, then assemble the working
checklist as universal core + the checklist fragment embedded in each selected module's file
(`templates/constitution-modules/*.md`), applied to the module's **routed** content (region
pointer / rules files / ledger section — per the authoring skill's routing table). Verify every
item. Do not skip items because they "seem obvious" or "clearly pass" — and do not check module
fragments the synthesis did not select.

### Step 2: Region and Surface Integrity (deterministic)

- Exactly one governance region in `CLAUDE.md`, both markers present, correctly ordered.
- The region carries the ratified stamp (version · date · tier, one line), the principle index,
  universal principles as short imperative lines, the technology stack, and the quality-gates
  summary — and stays **short-form** (detail belongs to the ledger; a region restating ledger
  detail is a fix-list item).
- Every index line's pointer resolves: the named rules file exists under `.claude/rules/mochiko/`,
  the named skill exists, or the principle's home is the region itself.
- Every rules file carries `paths` frontmatter honest to its concern and operative rules in the
  body. **A universal principle homed in a rules file is a FAIL** (delivery to spawned producers
  is unproven; universal content belongs in the region).
- **Scope coverage** (per rules file): the `paths` globs cover every path whose code can violate
  the concern — cross-check against the layer-rules Import Rules table when that module is
  attached (a layer that MAY invoke the governed operation but matches no glob is a fix-list
  item), otherwise against the region's technology-stack and module pointers.
- **New-file read line** (when any rules file exists): the region's Governance operations carries
  the standing line — rules inject on Read, not Write; read the matching rules file (or read back
  the created file) before creating a file under a scoped path. Absence is a fix-list item.

### Step 3: Check Each Principle (three-part, in the ledger)

Every principle in the manifest MUST have a ledger entry keyed by its GI-ID carrying the
three-part structure:

| Part | Purpose | Verification |
|------|---------|--------------|
| **Enforcement** | How is compliance verified? | CI check, code review rule, or audit process named |
| **Testability** | What does pass/fail look like? | Concrete pass and fail conditions defined |
| **Rationale** | Why does this rule exist? | Business or technical justification present |
| **Home** | Where does its operative text live? | Named home matches the index and the actual surface |

If any principle lacks any part, the set FAILS validation.

### Step 4: Trace Closure Cross-Check (deterministic, both ways, over the manifest)

String-match against the synthesis and the surfaces:

1. Every manifest row's GI-ID exists in `governance-intent.md` and points at a principle-bearing
   element (deck-kept / minted / floor-preset).
2. Every principle-bearing element in the synthesis appears in the manifest — realized on a
   surface — or in the producer's flagged-proposals list. Unrealized-and-unflagged = FAIL.
3. Every manifest row closes on the surfaces: **one primary enforceable home** (region line,
   rules file, or skill pointer) **plus its companion entries** (index line + ledger entry), each
   actually present. A missing companion is a FAIL — this is the check that catches an index line
   pointing at a file that doesn't exist, or a rule with no ledger metadata.
4. Waiver records and routed module content match the synthesis's waiver and module-selection
   elements one-for-one.

This check is deterministic — ID presence and closure, not meaning. **Semantic fidelity of a
stamped trace is judgment-grade residual risk**: a fabricated-but-plausible trace passes the
string match. Flag suspected content↔intent mismatches in the fix list as advisory findings; the
fidelity gates are the human checkpoints upstream (synthesis confirmation) and downstream (the
acceptance gate's trace summary), not this scan.

### Step 5: Tier and Waiver Checks

- Ledger carries the declared tier, graduation path, trace; the region stamp's tier matches.
- Every Essential Floor category has a principle **or a recorded waiver** — in any mode.
- Every waiver carries: category, waiving tier, revisit trigger, trace. A waiver at a
  tier whose posture forbids it (`production`/`regulated`) is a FAIL.
- Thresholds and gate strictness consistent with the declared tier, or covered by a recorded
  session override in the synthesis.

### Step 6: Scan for Anti-Patterns

Compare against [references/ANTI-PATTERNS.md](references/ANTI-PATTERNS.md). Common failures:

| Anti-Pattern | Detection |
|--------------|-----------|
| Vague principle | Contains words like "appropriate", "reasonable", "clean" without metrics |
| Missing enforcement | Principle states rule but no verification mechanism |
| Placeholder syndrome | Contains `[PLACEHOLDER]`, `[COMMAND]`, `[THRESHOLD]` syntax |
| Generic thresholds | Says "coverage must be measured" instead of "coverage ≥80%" |

### Step 7: Verify No Placeholders

This is the most commonly rationalized check. Search **every member of the set** (region, every
rules file, ledger, manifest) for:

- `[PLACEHOLDER]`
- `[COMMAND]`
- `[THRESHOLD]`
- `[TOOL]`
- `GI-XXX` (an unfilled trace stamp is a placeholder)
- Any `[BRACKETED_TEXT]` pattern

**No exceptions.** A surface set with placeholders is not ready for validation sign-off.

### Step 8: Determine Version Bump

| Bump | Trigger | Example |
|------|---------|---------|
| **MAJOR** | Principle removed or incompatibly redefined; tier change | Removing "Test-First" principle; poc → production |
| **MINOR** | New principle added or significant expansion; waiver added/removed | Adding "Observability" principle; un-waiving Testing |
| **PATCH** | Clarification or non-semantic change | Rewording for clarity; typo fixes; formatting |

### Step 9: Document Validation Result

Produce explicit validation verdict:

```
VALIDATION RESULT: [PASS/FAIL]

Checklist items: [X/Y passed] (core + [N] module fragments: [names])
Surface integrity: [region markers OK · index→home resolution X/X · rules files paths-scoped Y/Y · scope coverage Z/Z · new-file read line present/absent/n-a · universal-in-rules violations: none/list]
Trace closure: [manifest rows closed X/X (primary home + companions) · synthesis elements realized-or-flagged Y/Y · waivers matched · modules matched]
Tier/floor accounting: [tier declared (region stamp = ledger) · floor categories principled/waived, e.g. 3 principled + 1 waived]
Anti-patterns found: [list or "none"]
Version bump: [MAJOR/MINOR/PATCH] (if changes made)

Issues requiring fix:
- [list each failure]
Advisory (judgment-grade, non-blocking):
- [suspected trace-fidelity mismatches, or "none"]
```

## Quantification Requirements

Vague language MUST be replaced with measurable criteria:

| Vague | Quantified |
|-------|------------|
| "Code should be clean" | "Zero lint warnings from configured rules" |
| "Functions should be short" | "Functions MUST NOT exceed 40 lines" |
| "Tests should cover the code" | "Coverage MUST be ≥80% for new code" |
| "Response should be fast" | "API MUST respond in <200ms p95" |
| "Secure by default" | "All inputs MUST be validated; auth required on all endpoints" |

## Common Mistakes

| Mistake | Why It Happens | Fix |
|---------|----------------|-----|
| Skipping checklist items | "Obviously passes" | Run every item. Obvious failures happen. |
| Accepting placeholders | "User will fill in later" | Placeholders = incomplete. Return for completion. |
| Validating during drafting | Interrupts creative flow | Draft first, validate second. Separate phases. |
| Soft validation language | "Mostly looks good" | Binary verdict: PASS or FAIL. No middle ground. |
| Missing version bump | "Small change" | Every change needs version bump determination. |
| Validating non-constitutions | Skill triggered by similar keywords | Verify document IS a constitution before validating. |

## Red Flags - STOP and Restart Properly

If you notice yourself thinking any of these, STOP immediately:

- "The constitution looks complete enough"
- "This is just a minor update, doesn't need full validation"
- "I already reviewed it while writing"
- "User seems happy with it"
- "The checklist is too detailed for this simple project"
- "These anti-patterns don't apply to this case"
- "I can skip the placeholder check—I didn't use any"
- "Validation would be redundant since I wrote it carefully"

**All of these mean:** You are rationalizing. Restart validation from Step 1.

## Common Rationalizations

| Excuse | Reality |
|--------|---------|
| "Constitution looks complete" | Looking complete ≠ being complete. Run the checklist. |
| "Just a minor update" | Minor updates can introduce major anti-patterns. Full validation. |
| "Already reviewed while writing" | Authoring mode ≠ validation mode. Fresh review catches blind spots. |
| "User seems satisfied" | User satisfaction doesn't verify enforcement mechanisms exist. |
| "Too detailed for simple project" | Simple projects become complex. Governance debt compounds. |
| "Anti-patterns don't apply here" | Every rationalization claims uniqueness. They apply. |
| "I'm being pragmatic" | Pragmatic = following validation process. Skipping is not pragmatic. |
| "Can validate more thoroughly later" | "Later" rarely comes. Validate now or ship broken governance. |

## Explicit Loophole Closures

### "The constitution looks fine"

Looking fine is not validation. Run every checklist item. Document every result. A constitution is validated when all checks pass, not when it "looks fine."

### "This is a small change"

Small changes require validation. A one-line change can introduce vague language, remove enforcement, or add placeholders. Size does not determine validation necessity.

### "I'll add the missing parts later"

Constitutions with missing parts FAIL validation. Return to authoring. Do not sign off on incomplete governance.

### "User asked to skip validation"

User requests do not override process. Explain why validation matters. If user insists, document that validation was skipped against recommendation—but never claim a validated constitution when validation was skipped.

### "The project is just prototyping"

Prototypes become production. Governance established during prototyping persists. Validate now or inherit broken governance later.

## Testing Evidence

### Baseline Testing Results (RED Phase)

Pressure scenarios tested without skill loaded revealed these agent behaviors:

**Scenario 1: Time pressure + "looks complete"**
- Agent rationalized: "The constitution appears comprehensive and I wrote it carefully"
- Skipped checklist, missed placeholder in Quality Gates section
- Verdict: FAIL - proceeded without systematic validation

**Scenario 2: User satisfaction signal**
- Agent rationalized: "User already reviewed the draft and seemed satisfied"
- Skipped anti-pattern scan, missed vague "appropriate level" language
- Verdict: FAIL - treated user satisfaction as validation

**Scenario 3: Minor update context**
- Agent rationalized: "This is just updating one threshold, full validation is overkill"
- Skipped Steps 1-3, only checked the changed line
- Verdict: FAIL - partial validation is not validation

### Skill Effectiveness (GREEN Phase)

Same scenarios re-run with skill loaded:
- Agent cited "letter = spirit" principle when tempted to skip
- Agent ran full checklist despite time pressure
- Agent produced binary PASS/FAIL verdicts
- All placeholders and anti-patterns caught

## Related Skills

- **OPTIONAL:** mochiko:authoring-constitution - Core authoring for constitutions; greenfield mode for new projects, brownfield mode for existing codebases
