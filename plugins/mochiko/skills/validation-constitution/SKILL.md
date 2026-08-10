---
name: validation-constitution
description: This skill MUST be invoked to grade a DRAFTED governance surface set against the quality checklist — there is NO constitution.md; the graded set is the CLAUDE.md governance region, the `.claude/rules/mochiko/` files, and the governance ledger. SHOULD also invoke for the setup loop's validate step, or when re-validating after a FAIL-loop revision. Validator-side skill of the governance producer↔validator pair; defaults to FAIL; run by an independent validator, never the author.
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

The independent review leaves its verdict and per-finding dispositions in the reviewed artifacts themselves — review evidence that lives only in conversation is a floor violation.

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

### Step 8: Determine Version Bump

| Bump | Trigger | Example |
|------|---------|---------|
| **MAJOR** | Principle removed or incompatibly redefined; floor-level change; module attach/detach | Removing "Test-First" principle; attaching `hipaa` |
| **MINOR** | New principle added or significant expansion; waiver added/removed | Adding "Observability" principle; un-waiving Testing |
| **PATCH** | Clarification or non-semantic change | Rewording for clarity; typo fixes; formatting |

### Step 9: Document Validation Result

Produce explicit validation verdict:

```
VALIDATION RESULT: [PASS/FAIL]

Checklist items: [X/Y passed] (core + [N] module fragments: [names])
Surface integrity: [region markers OK · index→home resolution X/X · rules files paths-scoped Y/Y · scope coverage Z/Z · new-file read line present/absent/n-a · universal-in-rules violations: none/list]
Trace closure: [manifest rows closed X/X (primary home + companions) · synthesis elements realized-or-flagged Y/Y · waivers matched · modules matched]
Floor/module accounting: [floor asserted (region stamp = ledger) · modules matched to the fact profile · floor categories principled/waived, e.g. 3 principled + 1 waived]
Anti-patterns found: [list or "none"]
Version bump: [MAJOR/MINOR/PATCH] (if changes made)

Issues requiring fix:
- [list each failure]
Advisory (judgment-grade, non-blocking):
- [suspected trace-fidelity mismatches, or "none"]
```

## Quantification Requirements

Vague language MUST be replaced with measurable criteria — the patterns and quantified examples
live in [references/ANTI-PATTERNS.md](references/ANTI-PATTERNS.md) (*Vague Principle*, *Generic
Thresholds*, *Missing Enforcement*).

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
| "User asked to skip validation" | User requests do not override process. If the user insists, document that validation was skipped against recommendation — never claim a validated set when validation was skipped. |
| "I'll add the missing parts later" | Missing parts = FAIL. Return to authoring; never sign off incomplete governance. |

## Related Skills

- **OPTIONAL:** mochiko:authoring-constitution - Core authoring for constitutions; greenfield mode for new projects, brownfield mode for existing codebases
