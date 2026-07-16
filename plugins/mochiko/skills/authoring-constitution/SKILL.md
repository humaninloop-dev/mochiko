---
name: authoring-constitution
description: This skill MUST be invoked when authoring or amending a project constitution (the governance document at `.mochiko/memory/constitution.md`) — formulating enforceable principles from a ratified session synthesis (`.mochiko/memory/governance-intent.md`), assembling the universal-core template plus selected modules, stamping trace-IDs, recording tier declaration and floor waivers, and establishing an amendment/version policy. Handles BOTH modes in one place: greenfield (authoring from the synthesis's deck rulings and minted intents) and brownfield (the same, additionally codifying an existing codebase's patterns — Essential Floor assessed against the code plus an Emergent Ceiling, informed by `.mochiko/memory/codebase-analysis.md`). SHOULD also invoke when the authoring work concerns principle enforcement, testability, rationale, the Three-Part Rule, RFC 2119 keywords, a SYNC IMPACT report, trace stamps, tier declarations, floor waivers, module assembly, an Essential Floor, or an Emergent Ceiling. This is the single constitution-authoring skill for both new and existing projects — there is no separate brownfield skill.
---

# Authoring Constitution

## Overview

Write project constitutions that teams actually follow. Every principle must be enforceable,
testable, and justified; vague aspirations are rejected in favor of actionable constraints with
measurable criteria. The artifact lives at `.mochiko/memory/constitution.md`.

Constitutions are authored **from a ratified session synthesis** —
`.mochiko/memory/governance-intent.md`, produced by the setup lead's interrogation session and
confirmed by the user before authoring begins. The synthesis is a **traceable contract, not a
brief**: it owns *selection* (which principles, at what tier, with which waivers and modules);
this skill owns *formulation* (wording, enforcement mechanics, structuring intent into three-part
principles with real commands).

This skill produces a **reviewable** governance document. Its quality is graded by an
**independent validator** (a separate agent running `validation-constitution`), and accepted at a
named human gate — the sequencing, the produce→validate→revise loop, and the human gates are owned
by the command lead that drives this skill, not by this skill.

## The synthesis contract (selection vs. formulation)

The non-negotiable discipline of this skill:

- **Every principle traces.** Each authored principle carries a trace stamp naming exactly one
  principle-bearing synthesis element: `**Trace**: GI-XXX (deck-kept: CARD-ID | minted |
  floor-preset: CARD-ID)`.
- **Every element is realized or flagged.** Each principle-bearing synthesis element becomes a
  principle, or is surfaced as a flagged proposal — never silently dropped.
- **No unsanctioned selection.** Do not add, remove, merge, or reinterpret principles beyond the
  synthesis. If authoring reveals a genuine problem with the selection (a contradiction, a missing
  principle the project clearly needs, elicited intent that **resists enforceable formulation**),
  do not fold a fix in silently and do not author vagueness — emit a **flagged proposal**: what
  you propose, why, and which synthesis element (if any) it touches. Flagged proposals are ruled
  on by the user at the acceptance gate.
- **Waivers are authored, not skipped.** A waived floor category gets a waiver record in the
  constitution (category, waiving tier, revisit trigger, trace stamp) — absence is always
  deliberate and auditable.

## Two modes, one shared core

Constitution authoring runs in one of two modes. **Pick the mode first**; both then converge on
the same shared core. Both modes consume the synthesis.

| Mode | Use when | Adds on top of the shared core |
|------|----------|--------------------------------|
| **greenfield** | A new project with no existing code to honor. | Principles formulated from the synthesis's deck rulings + minted intents; floor cards at their tier parameterization. |
| **brownfield** | An existing codebase — *codify what is already there*. Requires `.mochiko/memory/codebase-analysis.md` (produced upstream by `analysis-codebase`). | Essential Floor *assessed against the code* (present/partial/absent) **+** an Emergent Ceiling codifying good existing patterns **+** the `evolution-notes` module. |

**The shared core is authored once, identically, in both modes** — the Three-Part Principle Rule,
RFC 2119 keywords, the SYNC IMPACT REPORT, the Mandatory Constitution Sections, and module
assembly below. **Brownfield does NOT restate the shared core; it defers to this common path.**
The content sources:

- **both → [references/catalog/](references/catalog/README.md)** — the tier/type-tagged principle
  deck (universal floor + type shelves). The synthesis's deck rulings name which cards were kept
  and how they were adjusted; the cards carry the principle material to formulate from.
- **both → [references/ESSENTIAL-FLOOR.md](references/ESSENTIAL-FLOOR.md)** — the canonical
  definition of the four floor categories (greenfield formulates them at tier strictness;
  brownfield assesses the codebase against them). Tier strictness and waiver posture per category
  live on the floor cards in the catalog.
- **brownfield → [references/EMERGENT-CEILING-PATTERNS.md](references/EMERGENT-CEILING-PATTERNS.md)**
  for the existing-pattern library.

> **Mode prerequisites (lead-owned).** Authoring consumes a **ratified** synthesis — the setup
> lead runs the interrogation and the synthesis-confirmation checkpoint before dispatching this
> skill. Brownfield mode additionally consumes `.mochiko/memory/codebase-analysis.md`. This skill
> assumes both are present when invoked; if the synthesis is missing, say so and stop — authoring
> without it reproduces exactly the producer-decides-selection failure the synthesis exists to
> prevent.

## When to Use

- Authoring a constitution from a ratified synthesis, for a **new** project (greenfield mode).
- Authoring for an **existing** codebase, codifying its conventions without disrupting working
  code (brownfield mode).
- Amending an existing constitution against a delta-updated synthesis: adding/redefining
  principles, waivers, or tier, with a SYNC IMPACT bump.
- Establishing enforcement mechanisms, testability criteria, and amendment/version policy.

## When NOT to Use

- **Reviewing/grading an existing constitution** → that is the independent validator's job
  (`validation-constitution`), run by a different agent.
- **Eliciting what the constitution should contain** → that is the interrogation session's job
  (lead-conducted, upstream); this skill formulates a ratified synthesis, it does not interview.
- **Analyzing the codebase** → run `analysis-codebase` first; brownfield mode consumes its output.

## Common Mistakes (shared core)

| Mistake | Problem | Fix |
|---------|---------|-----|
| Vague principles | "Code should be clean" has no enforcement | Add specific, measurable criteria: "Functions MUST be ≤40 lines" |
| Missing enforcement | Principles without verification become suggestions | Every principle needs CI automation, code review checklist, or audit process |
| Untestable criteria | "Good architecture" can't be verified | Define binary pass/fail: "No domain imports from infrastructure layer" |
| No rationale | Future maintainers don't know why rules exist | Explain the failure mode prevented and success enabled |
| Selecting beyond the synthesis | Producer defaults silently override elicited intent — the failure this contract exists to prevent | Formulate only what the synthesis selects; everything else is a flagged proposal |
| Authoring vagueness for hard intent | Elicited intent that resists enforcement gets watered into an aspiration | Flag it as a proposal; the user rules at acceptance |
| Missing/fabricated trace stamps | Traceability check fails; or a plausible stamp hides a deviation | Stamp every principle with its real GI source; deviations go through flagged proposals |
| One tier's numbers in another tier's constitution | A poc graded like production, or production loosened to poc | Take tier parameterization from the floor cards; honor session overrides |
| Copying misfitting shelf examples | Backend-flavored examples (RFC 7807, `/health`) land in an SPA constitution | Formulate from the category definition, fitted to the declared type |
| Skipping SYNC IMPACT | Constitution changes without audit trail | Always update the SYNC IMPACT REPORT header with version changes |
| CLAUDE.md drift | AI assistants operate with outdated guidance | Include the CLAUDE.md Synchronization section; keep both files versioned together |

---

# Shared core (both modes)

## The Three-Part Principle Rule

Every principle MUST have three components. A principle without all three is incomplete and should
not be accepted.

### 1. Enforcement

How compliance is verified. Without enforcement, a principle is a suggestion.

```markdown
**Enforcement**:
- CI runs `ruff check .` and blocks merge on violations
- Code review MUST verify test files accompany new functionality
- Quarterly audit checks exception registry for staleness
```

**Enforcement Types**:

| Type | Examples | Strength |
|------|----------|----------|
| **CI Automated** | Linting, tests, coverage gates | Strongest—no human judgment needed |
| **Code Review** | Architecture compliance, security review | Strong—explicit checklist item |
| **Tooling** | Pre-commit hooks, IDE plugins | Medium—can be bypassed |
| **Audit** | Quarterly review, compliance check | Weaker—periodic, not continuous |

Enforcement MUST fit the team reality recorded in the synthesis — a solo project cannot lean on
"code review MUST verify"; give it tooling and CI instead.

### 2. Testability

What pass/fail looks like. A principle without testable criteria is merely an aspiration.

```markdown
**Testability**:
- Pass: `flutter analyze` exits with code 0
- Pass: All functions have ≤10 cyclomatic complexity
- Fail: Any file exceeds 400 lines without documented exception
```

**Testability Requirements**: binary outcome; measurable threshold where applicable; observable
without subjective judgment; reproducible by any team member.

### 3. Rationale

Why this constraint exists. Future maintainers need this to evaluate if the rule is still
relevant.

```markdown
**Rationale**: Tests written after implementation tend to validate what was built rather than
what was intended. Test-first ensures requirements drive implementation, catches defects early,
and produces inherently testable, modular code.
```

**Rationale Requirements**: explains the failure mode prevented; describes the success enabled;
provides context for future evaluation; justifies the enforcement overhead.

## Principle Writing Format

```markdown
### I. [Principle Name]

[Declarative statement of the constraint using RFC 2119 keywords]

- [Specific rule 1]
- [Specific rule 2]

**Enforcement**:
- [How compliance is verified — specific commands or processes]

**Testability**:
- [Pass/fail criteria, measurable thresholds]

**Rationale**: [Why this constraint exists]

**Trace**: GI-XXX (deck-kept: CARD-ID | minted | floor-preset: CARD-ID)
```

The trace stamp is part of the format — a principle without one is incomplete, exactly like a
principle without a rationale.

## RFC 2119 Keywords

Use precise language for requirements:

| Keyword | Meaning | Example |
|---------|---------|---------|
| **MUST** | Absolute requirement; no exceptions | "Tests MUST pass before merge" |
| **MUST NOT** | Absolute prohibition | "Secrets MUST NOT be committed" |
| **SHOULD** | Recommended; valid exceptions exist | "Functions SHOULD be under 40 lines" |
| **SHOULD NOT** | Discouraged; valid exceptions exist | "Magic numbers SHOULD NOT appear" |
| **MAY** | Optional; implementation choice | "Teams MAY adopt additional linting rules" |

See [references/RFC-2119-KEYWORDS.md](references/RFC-2119-KEYWORDS.md) for detailed usage.

## Mandatory Constitution Sections

Every constitution MUST include these sections, regardless of mode, using
[`constitution-template.md`](../../templates/constitution-template.md) (the universal core):

### 1. SYNC IMPACT REPORT (Header)

Track changes as an HTML comment at file top — version change with semver rationale, modified
principles, added/removed sections, configuration changes, templates requiring updates, follow-up
TODOs, and a rolling log of previous versions. See
[references/SYNC-IMPACT-FORMAT.md](references/SYNC-IMPACT-FORMAT.md) for the complete format.

### 2. Governance Tier

The declared tier from the synthesis (trace-stamped), the graduation path, and the **Waivers
table** — one row per waived floor category: category, waiving tier, revisit trigger, trace.
"None." when nothing is waived. Waivers are only valid at tiers whose posture permits them
([catalog/universal-floor.md](references/catalog/universal-floor.md)).

### 3. Core Principles

Numbered principles (I, II, III…) with Enforcement/Testability/Rationale/Trace. Floor principles
first, then type and minted principles. Principle **count is driven by the synthesis**, not a
quota. Mark non-negotiable principles explicitly: `(NON-NEGOTIABLE)` — at `production`/`regulated`
tier, floor principles are always non-negotiable.

> Every constitution MUST **account for** all four Essential Floor categories
> ([references/ESSENTIAL-FLOOR.md](references/ESSENTIAL-FLOOR.md)) — with a principle or, where
> the tier permits, a recorded waiver. A floor category with neither is a defect in either mode.

### 4. Technology Stack

Mandated technology choices with rationale — from the synthesis's real-commands and
existing-practices elements. In brownfield mode this table is populated **from the codebase
analysis** rather than chosen fresh.

### 5. Quality Gates

Automated checks that block merge, with **actual commands** from the synthesis (never placeholder
tokens). Coverage pre-seeds are **tier-parameterized** — take them from the FLOOR-TEST card's tier
row unless the session overrode them. Gates for waived categories are omitted; the waiver record
covers the absence.

### 6. Governance

Amendment process (including the rule that tier bumps and un-waives are governance events routed
back through setup's amend mode), version policy (MAJOR: principle removal/incompatible
redefinition/tier change · MINOR: new principle or waiver change · PATCH: clarification),
exception registry, compliance review. Calibrate ceremony to the synthesis's team reality.

### 7. CLAUDE.md Synchronization

Define synchronization requirements — the mapping table, process, and enforcement. **This
section's SPEC is constitution content and stays here.** The *operational* act of propagating
constitution changes into CLAUDE.md is handled by a separate cluster's `syncing-claude-md` skill —
see Related, below. Writing the constitution's own synchronization section is part of authoring;
performing the sync is not.

## Module assembly

The universal core is fixed; everything type- or tier-specific attaches as a **module** from
[`templates/constitution-modules/`](../../templates/constitution-modules/):

| Module | Attach when (per the synthesis's module selections) |
|--------|------------------------------------------------------|
| `layer-rules` | A layered-architecture card (e.g. BE-HEX) was kept |
| `release-gates` | The deployment dimension elicited a real release process |
| `evolution-notes` | Mode is brownfield (always) |

Procedure: read the synthesis's **module selections**; attach exactly those module sections at the
marked point in the core template; fill each from the synthesis (and, brownfield, the analysis);
add a CLAUDE.md sync-table row per module the sync mandates. **Never inline module content the
synthesis didn't select** — an unselected module attached "to be safe" is unsanctioned selection.
Each module file carries its own validator checklist fragment; the validator checks core + exactly
the attached modules.

---

# Greenfield branch (formulate the synthesis)

Use when authoring for a **new** project. There is no fixed default principle set: the synthesis's
deck rulings and minted intents ARE the selection. The job is formulation quality.

1. **Floor principles** — for each floor card the synthesis keeps: formulate at the card's tier
   parameterization (as adjusted by the session), fitted to the declared project type. The worked
   examples in [references/ESSENTIAL-FLOOR.md](references/ESSENTIAL-FLOOR.md) are
   backend-flavored; for other types, formulate from the category definition instead of copying
   the example.
2. **Type principles** — for each kept shelf card: formulate from the card's content, honoring
   every recorded tighten/loosen ruling.
3. **Minted principles** — for each minted intent: structure the elicited intent into a three-part
   principle with real commands. This is the formulation craft at full strength — the intent says
   *what* to enforce and *how strictly*; you supply the enforceable *how*. Intent that resists
   enforceable formulation goes back as a flagged proposal, never as an aspiration.

# Brownfield branch (codify existing patterns)

Use when authoring for an **existing codebase**, with the **Essential Floor + Emergent Ceiling**
approach: existing codebases have implicit conventions worth preserving (Emergent Ceiling) but may
lack foundational governance (Essential Floor). Codify both — respecting what works while
establishing governance.

Brownfield mode reuses the entire shared core above **without restating it**. What follows is the
brownfield-unique layer only.

## Brownfield Input

Driven by **two** inputs: the ratified synthesis (selection, tier, waivers, module selections,
session confrontation rulings) and `.mochiko/memory/codebase-analysis.md` — read it for
**"Strengths to Preserve"** (Emergent Ceiling candidates) and the **Essential-Floor status**
(present / partial / absent per category).

## Essential Floor — assessed against the code, waiver-aware

For each floor category the synthesis keeps:

- Codebase **has** the capability → the principle codifies the existing pattern with enforcement.
- Codebase **lacks** it → the principle states "MUST implement" and references a roadmap gap.

For each floor category the synthesis **waives**: write the waiver record (Governance Tier
section), not a pretend-principle and not a gap. The session already confronted any
detected-reality-vs-declared-intent conflicts ("production tier, no tests") — author the ruling
the synthesis records, never re-litigate it.

## Emergent Ceiling (FROM CODEBASE)

Identify **existing good patterns** worth codifying: read the analysis's "Strengths to Preserve",
identify patterns (naming conventions, architecture patterns, error formats), and codify as
principles with enforcement. See
[references/EMERGENT-CEILING-PATTERNS.md](references/EMERGENT-CEILING-PATTERNS.md) for the pattern
library. Ceiling principles trace like any other — to a deck ruling or minted element; a ceiling
pattern no synthesis element sanctions is a **flagged proposal** ("the codebase consistently does
X; propose codifying it").

Only codify patterns that are **intentionally good**. Ask: "Would I recommend this pattern for a
new project?" If no, it is technical debt, not an Emergent Ceiling pattern.

## Brownfield structure notes

- Attach the `evolution-notes` module (always selected in brownfield): floor status table, gap
  references, session confrontation rulings.
- Never skip codebase analysis — without it, this work devolves into guessing; good patterns get
  missed and floor status assessments become inaccurate.
- Future maintainers must be able to tell codified-existing-capability from aspirational
  MUST-implement targets — that distinction lives in the Evolution Notes.

> **Roadmap stub (moved-to-other-cluster).** The Evolution Notes section *spec* stays here (via
> its module). Producing `evolution-roadmap.md` (the improvement plan) is the roadmap cluster's
> job, not ported yet — reference `.mochiko/memory/evolution-roadmap.md` as a documented stub.

---

## Related (cross-cluster stubs — referenced, not mounted)

- **Independent validation** — after authoring, the constitution is graded by
  `validation-constitution`, run by a **separate validator agent** (never co-mounted with this
  skill). The produce→validate→revise loop and the human gates are owned by the command lead.
- **`analysis-codebase`** (in-cluster) — produces `.mochiko/memory/codebase-analysis.md`, the
  brownfield-mode input. Run before brownfield authoring (lead-sequenced).
- **`syncing-claude-md`** (documented stub, *moved-to-other-cluster*) — performs the operational
  CLAUDE.md propagation. **Not ported.** The CLAUDE.md Synchronization *section spec* stays here
  as constitution content; the execution skill is referenced as a planned stub, not a live mount.
- **`authoring-roadmap` / `evolution-roadmap`** (documented stub, *moved-to-other-cluster*) — turn
  the brownfield gap status into an improvement plan at `.mochiko/memory/evolution-roadmap.md`.
  **Not ported.** The Evolution Notes module spec stays; the roadmap producer is a planned stub.
