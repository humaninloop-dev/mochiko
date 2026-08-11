---
name: authoring-constitution
description: This skill MUST be invoked when authoring or amending a project's governance surface set from a ratified session synthesis (`.mochiko/memory/governance-intent.md`), landing principles on native Claude Code surfaces; there is NO constitution.md. SHOULD also invoke when the work concerns principle enforcement, compliance modules, floor waivers, or an Essential Floor. The single governance-authoring skill for BOTH greenfield and brownfield projects — no separate brownfield skill.
---

# Authoring Constitution — Governance on Native Surfaces

## Overview

Write project governance that teams — and Claude Code sessions — actually follow. Every principle
must be enforceable, testable, and justified; vague aspirations are rejected in favor of
actionable constraints with measurable criteria. **There is no `constitution.md`.** Governance
lands on the surfaces Claude Code natively loads, each at its disclosure level:

| Surface | Carries | Disclosure |
|---------|---------|------------|
| `CLAUDE.md` **governance region** (between `<!-- mochiko:governance:begin -->` / `<!-- mochiko:governance:end -->`) | Ratified stamp · principle index · **universal principles as short imperative lines** · tech stack · quality-gates summary · module pointers | Always-on, every session and every spawned agent |
| `.claude/rules/mochiko/*.md` | Scope-bound principles, one file per concern, `paths` frontmatter | On matching-file reads (plus the dispatch-brief obligated read for authoring producers) |
| Skill pointers | Procedure-shaped standards → the index/rule points at the skill; mint a new skill only when the session minted a procedure | On trigger / when a brief names it |
| `.mochiko/memory/governance-ledger.md` | Per-principle **Three-Part records** keyed by GI-ID · floor + attached compliance modules · waivers · amendment policy · exceptions · amendment log | Read only by setup/amend runs and the validator |

Authoring also emits the **trace summary** — the manifest mapping every GI element to its primary
enforceable home + companion entries (index line, ledger entry). The independent validator grades
trace closure over it; write it as part of the output, not as an afterthought.

Governance is authored **from a ratified session synthesis** —
`.mochiko/memory/governance-intent.md`, produced by the setup lead's interrogation session and
confirmed by the user before authoring begins. The synthesis is a **traceable contract, not a
brief**: it owns *selection* (which principles, with which waivers and modules — the floor is
asserted, its expression session-shaped);
this skill owns *formulation* (wording, enforcement mechanics, surface routing).

This skill produces a **reviewable** surface set. Its quality is graded by an **independent
validator** (a separate agent running `validation-constitution` — never co-mounted with this
skill), and accepted at a named human gate — the sequencing, the produce→validate→revise loop,
and the human gates are owned by the command lead that drives this skill, not by this skill.

## The synthesis contract (selection vs. formulation)

The non-negotiable discipline of this skill:

- **Every principle traces.** Each authored principle carries its GI trace: on `CLAUDE.md`, as an
  HTML comment beside the line; always in the **ledger**, as the `GI-XXX (floor-asserted: CARD-ID |
  deck-kept: CARD-ID | minted | module: <module>-<obligation>)` key — the canonical trace record
  (why the ledger is canonical: the template's comment block).
- **Every element is realized or flagged.** Each principle-bearing synthesis element becomes a
  principle on some surface, or is surfaced as a flagged proposal — never silently dropped.
- **No unsanctioned selection.** Do not add, remove, merge, or reinterpret principles beyond the
  synthesis. If authoring reveals a genuine problem (a contradiction, a missing principle the
  project clearly needs, elicited intent that **resists enforceable formulation**), do not fold a
  fix in silently and do not author vagueness — emit a **flagged proposal**: what you propose,
  why, and which synthesis element (if any) it touches. Flagged proposals are ruled on by the
  user at the acceptance gate.
- **Waivers are authored, not skipped.** A waived standard gets a waiver record in the ledger
  (standard, justification, optional revisit trigger, trace — D4: permanent pending the D4.1
  revisit; legal-mandate module obligations are never waivable, D4.2) — absence is always
  deliberate and auditable.

## Surface routing (which content lands where)

Route each synthesis element by its scope; the routing IS part of formulation quality:

- **Universal** (governs every session's work) → a **short imperative line in the CLAUDE.md
  region**. Keep it one to a few lines; the always-on budget is the scarcest resource — detail
  and metadata go to the ledger. Universal principles do NOT go to unconditional rules files
  (empirically, rules delivery to spawned producers cannot be assumed; CLAUDE.md is doc-confirmed
  for every spawn path).
- **Scope-bound** (governs work on a path-identifiable slice: layers, API surface, tests,
  frontend) → a **`paths`-scoped rules file** under `.claude/rules/mochiko/`, one concern per
  file, operative rules in the body, `paths` globs covering **every path whose code can violate
  the concern** — run the per-layer violation test (worked reasoning + kinako example: the
  template's Shape 2 preamble).
- **Scope-bound delivery caveat**: rules files inject on **Read**, not Write — whenever the set
  includes any rules file, emit the region's standing new-file read line, naming the actual
  scoped paths (Shape 1 carries the line and its observed-behavior wording).
- **Procedure-shaped** (a how-to, not a constraint) → a **pointer to the skill** that carries the
  procedure; the index line names it. Mint a new skill only for a session-minted procedure.
- **Every principle, regardless of home** → an **index line** in the region and a **ledger
  entry** (Three-Part + trace). Index → home → ledger must close; the validator checks it.

## Two modes, one shared core

| Mode | Use when | Adds on top of the shared core |
|------|----------|--------------------------------|
| **greenfield** | A new project with no existing code to honor. | Principles formulated from the synthesis's deck rulings + minted intents; floor cards authored at the synthesis's declared depth level (the two-row `low`/`high` card form). |
| **brownfield** | An existing codebase — *codify what is already there*. Requires `.mochiko/memory/codebase-analysis.md` (produced upstream by `analysis-codebase`). | Essential Floor *assessed against the code* (present/partial/absent) **+** an Emergent Ceiling codifying good existing patterns **+** the `evolution-notes` module. |

**The shared core is authored once, identically, in both modes** — the Three-Part Principle Rule,
RFC 2119 keywords, surface routing, the mandatory content inventory, and module assembly below.
The content sources:

- **both → [references/catalog/](references/catalog/README.md)** — the type-shelved principle
  deck (asserted universal floor + arbitrated type shelves). The synthesis's floor-expression
  and deck rulings name what was shaped, kept, and adjusted; the cards carry the principle
  material to formulate from.
- **both → [references/ESSENTIAL-FLOOR.md](references/ESSENTIAL-FLOOR.md)** — the canonical
  definition of the four floor categories. The two-row `low`/`high` depth form and the waiver
  posture per category live on the floor cards in the catalog — author the row the synthesis's
  declared depth level selects (adaptive-depth, 2026-08-11); audit-evidence variants live in
  [references/COMPLIANCE-MODULES.md](references/COMPLIANCE-MODULES.md) and attach via the
  synthesis's fact profile.
- **brownfield → [references/EMERGENT-CEILING-PATTERNS.md](references/EMERGENT-CEILING-PATTERNS.md)**
  for the existing-pattern library.

Artifact shapes (region block, rules file, ledger):
[`governance-surfaces-template.md`](../../templates/governance-surfaces-template.md).

> **Mode prerequisites (lead-owned).** Authoring consumes a **ratified** synthesis. Brownfield
> additionally consumes `.mochiko/memory/codebase-analysis.md`. If the synthesis is missing, say
> so and stop — authoring without it reproduces exactly the producer-decides-selection failure
> the synthesis exists to prevent.

> **Ownership boundary (D8).** Regenerate only what sits between the markers — user content
> outside them is untouchable. Rules files and the ledger are setup-owned and regenerated whole,
> except the two preserved blocks: `mochiko:domain-registry` (the template's comment block +
> `references/DOMAIN-DEPENDENCIES.md`) and the `mochiko:output-style` switch line in the
> region's Governance operations. Write the style line default-on when the region is first
> authored; on every later run **read the existing values and re-emit them unchanged** — the
> user sets their register there, and regenerating the defaults over it silently reverts them.
> The same preservation covers the Shape-5 output-style rules file: refresh it only to track those
> values, and keep any line the user added to it.
> In amend, preserve untouched principles verbatim (their
> GI-IDs are stable) and bump the region's semver. A `.mochiko/memory/constitution.md` on disk
> is superseded — the lead deletes it; never author into it.

## When NOT to Use

- **Reviewing/grading an existing surface set** → that is the independent validator's job
  (`validation-constitution`), run by a different agent.
- **Eliciting what governance should contain** → that is the interrogation session's job
  (lead-conducted, upstream); this skill formulates a ratified synthesis, it does not interview.
- **Analyzing the codebase** → run `analysis-codebase` first; brownfield mode consumes its output.

---

# Shared core (both modes)

## The Three-Part Principle Rule

Every principle MUST have three components, recorded in its **ledger entry**. A principle without
all three is incomplete and should not be accepted. Worked Three-Part examples: the four floor
principles in [references/ESSENTIAL-FLOOR.md](references/ESSENTIAL-FLOOR.md).

### 1. Enforcement

How compliance is verified. Without enforcement, a principle is a suggestion.

Enforcement MUST fit the team reality recorded in the synthesis — a solo project cannot lean on
"code review MUST verify"; give it tooling and CI instead. (Native surfaces are context, not
enforcement — the docs say so explicitly; the teeth are CI, hooks, review, audit, same as ever.)

### 2. Testability

What pass/fail looks like. A principle without testable criteria is merely an aspiration.
Binary outcome; measurable threshold where applicable; observable without subjective judgment;
reproducible by any team member.

### 3. Rationale

Why this constraint exists. Explains the failure mode prevented, the success enabled, and
justifies the enforcement overhead.

## Principle Writing Format

The surface (region line or rules-file rule) carries the **operative constraint only** — RFC 2119
keywords, short, GI trace comment. The ledger carries the full Three-Part record keyed by GI-ID.
Both shapes live in the template (Shape 1 line, Shape 3 record); do not restate them.

## RFC 2119 Keywords

MUST / MUST NOT (absolute; no exceptions) · SHOULD / SHOULD NOT (recommended / discouraged; valid
exceptions exist) · MAY (optional). Detailed usage:
[references/RFC-2119-KEYWORDS.md](references/RFC-2119-KEYWORDS.md).

## Mandatory content inventory

Every governance set MUST include, per
[`governance-surfaces-template.md`](../../templates/governance-surfaces-template.md):

1. **Ratified stamp** (region, one line): version · ratified date · floor · declared depth level ·
   attached modules; semver per the template's amendment policy (Shape 3).
2. **Principle index** (region): one line per principle — name, imperative gist, pointer to its
   home when the home is not the region itself.
3. **Universal principles** (region): the short imperative lines, floor principles first.
   Floor principles are marked `(NON-NEGOTIABLE)`. Author each floor principle at the row the
   synthesis's declared depth level selects; a `high`-only check is absent at `low`.
4. **Technology stack** (region): mandated choices — from the synthesis's real-commands and
   existing-practices elements; brownfield populates from the codebase analysis.
5. **Quality-gates summary** (region): the blocking checks with **actual commands** (never
   placeholder tokens); coverage pre-seeds from the FLOOR-TEST card's coverage threshold unless the
   session overrode them. Gates for waived categories are omitted; the waiver record covers the
   absence.
6. **Scope-bound rules files**: per the routing — globs violation-coverage tested, the standing
   new-file read line emitted. **Plus one unconditional file every run**, routed from no
   principle: the output-style rules file per the template's Shape 5, scoped over the
   deliverable-authoring paths — edit-time reinforcement of the register, never the carrier that
   reaches creates.
7. **Governance ledger**: sections per the template's Shape 3. Riders: waiver table says "None."
   when nothing is waived; every waiver carries its recorded justification per the D4 model
   ([catalog/universal-floor.md](references/catalog/universal-floor.md)); legal-mandate module
   obligations are never waived (D4.2 —
   [references/COMPLIANCE-MODULES.md](references/COMPLIANCE-MODULES.md)); fact-profile changes
   (module attach/detach) and un-waives are governance events routed back through setup's amend
   mode.
8. **Trace summary**: the manifest — every GI element → primary home + companions; every surface
   element → its GI element.

> Every governance set MUST **account for** all four Essential Floor categories
> ([references/ESSENTIAL-FLOOR.md](references/ESSENTIAL-FLOOR.md)) — with a principle or a
> recorded waiver (D4). A floor category with neither is a defect in either mode. (A `high`-only
> check absent at `low` is not a missing category — its category is still present through its
> other rows.)

There is **no CLAUDE.md-synchronization section**: governance lives ON CLAUDE.md — the
sync-a-copy problem died with the dissolved artifact.

## Module assembly

Modules from [`templates/constitution-modules/`](../../templates/constitution-modules/) attach
per the synthesis's module selections — but **module content routes by surface, like everything
else** (any in-file attach instructions that predate the dissolution are superseded by this
table):

| Module | Attach when | Routes to |
|--------|-------------|-----------|
| `layer-rules` | A layered-architecture principle was kept **or minted** (the module ruling lands in the synthesis either way — the interrogation's layered-architecture beat) | `paths`-scoped rules files (one per layer concern; the domain file carries the preserved registry block + policy preamble — `references/DOMAIN-DEPENDENCIES.md`) + index lines + ledger entries (incl. the Domain-dependency policy section) |
| `release-gates` | Always offered (a deployed/operated target class — PO-D1); content from the always-interrogated deployment dimension | Region: one summary line + pointer; detail in the ledger |
| `evolution-notes` | Mode is brownfield (always) | Ledger section (floor status, gap references, confrontation rulings) + region pointer |
| `knowledge-management` | The KM dimension elicited adoption (default-on, whole; a recorded decline is durable) | Region: the operating-manual **pointer** + index line; the bundle scaffolding and command carriers are unchanged (lead-executed at finalize) |
| **compliance modules** (`hipaa`, `pci-dss`, … — [references/COMPLIANCE-MODULES.md](references/COMPLIANCE-MODULES.md)) | The fact profile triggered them (mechanical attachment, recorded in the synthesis's Fact profile — never a session choice) | Obligations formulated as principles at their stratum (legal-mandate = unwaivable), routed by scope like any principle; the ledger records module + stratum per obligation |

**Never route module content the synthesis didn't select** — an unselected module attached "to be
safe" is unsanctioned selection. Each module's validator checklist fragment still applies to its
routed content.

---

# Greenfield branch (formulate the synthesis)

No fixed default principle set: the synthesis's deck rulings and minted intents ARE the
selection. The job is formulation + routing quality.

# Brownfield branch (codify existing patterns)

Codify the **Essential Floor + Emergent Ceiling**: existing codebases have implicit conventions
worth preserving (Emergent Ceiling) but may lack foundational governance (Essential Floor).
Reuses the entire shared core above without restating it. Driven by **two** inputs: the ratified
synthesis and `.mochiko/memory/codebase-analysis.md` — read it for **"Strengths to Preserve"**
(ceiling candidates) and the **Essential-Floor status** (present / partial / absent per category).

**Emergent Ceiling:** codify existing good patterns as principles with enforcement — see
[references/EMERGENT-CEILING-PATTERNS.md](references/EMERGENT-CEILING-PATTERNS.md). Ceiling
principles trace like any other; a ceiling pattern no synthesis element sanctions is a **flagged
proposal**. Only codify patterns that are intentionally good: "Would I recommend this for a new
project?" — if no, it is technical debt, not ceiling.

Attach the `evolution-notes` module (always, in brownfield); future maintainers must be able to
tell codified-existing-capability from aspirational MUST-implement targets — that distinction
lives in its ledger section.

> **Roadmap stub (moved-to-other-cluster).** Producing `evolution-roadmap.md` (the improvement
> plan) is the roadmap cluster's job, not ported yet — reference
> `.mochiko/memory/evolution-roadmap.md` as a documented stub.

---

## Related (cross-cluster; referenced, not mounted)

- **`analysis-codebase`** (in-cluster) — produces `.mochiko/memory/codebase-analysis.md`, the
  brownfield-mode input. Run before brownfield authoring (lead-sequenced).
