---
name: authoring-constitution
description: This skill MUST be invoked when authoring or amending a project's governance surface set — formulating enforceable principles from a ratified session synthesis (`.mochiko/memory/governance-intent.md`) and landing them on native Claude Code surfaces; there is NO constitution.md. The set: a marked, setup-owned governance region in `CLAUDE.md` (ratified stamp, principle index, universal principles as short imperative lines, tech stack, quality-gates summary, module pointers), `paths`-scoped `.claude/rules/` files for scope-bound principles, skill pointers for procedure-shaped standards, and a governance ledger at `.mochiko/memory/governance-ledger.md` (Three-Part metadata keyed by GI-ID, waivers, amendment/version policy, exception registry) — plus the trace summary manifest. Handles BOTH modes in one place: greenfield (authoring from the synthesis's deck rulings and minted intents) and brownfield (the same, additionally codifying an existing codebase's patterns — Essential Floor assessed against the code plus an Emergent Ceiling, informed by `.mochiko/memory/codebase-analysis.md`). SHOULD also invoke when the authoring work concerns principle enforcement, testability, rationale, the Three-Part Rule, RFC 2119 keywords, trace stamps, tier declarations, floor waivers, surface routing, the governance region, module assembly, an Essential Floor, or an Emergent Ceiling. The single governance-authoring skill for both new and existing projects — there is no separate brownfield skill.
---

# Authoring Constitution — Governance on Native Surfaces

## Overview

Write project governance that teams — and Claude Code sessions — actually follow. Every principle
must be enforceable, testable, and justified; vague aspirations are rejected in favor of
actionable constraints with measurable criteria. **There is no `constitution.md`.** Governance
lands on the surfaces Claude Code natively loads, each at its disclosure tier
(design record: `.mochiko/brainstorms/constitution-native-surfaces/record.md`, D1–D8):

| Surface | Carries | Disclosure |
|---------|---------|------------|
| `CLAUDE.md` **governance region** (between `<!-- mochiko:governance:begin -->` / `<!-- mochiko:governance:end -->`) | Ratified stamp (version · date · tier, one line) · principle index (one line each + pointer) · **universal principles as short imperative lines** · technology stack · quality-gates summary · module pointers | Always-on, every session and every spawned agent |
| `.claude/rules/mochiko/*.md` | Scope-bound principles, one file per concern, `paths` frontmatter | On matching-file reads (plus the dispatch-brief obligated read for authoring producers) |
| Skill pointers | Procedure-shaped standards → the index/rule points at the skill; mint a new skill only when the session minted a procedure | On trigger / when a brief names it |
| `.mochiko/memory/governance-ledger.md` | Per-principle **Enforcement / Testability / Rationale** keyed by GI-ID · tier + graduation path · waiver table with revisit triggers · amendment process + semver policy · exception registry · amendment log | Read only by setup/amend runs and the validator |

Authoring also emits the **trace summary** — the manifest mapping every GI element to its primary
enforceable home + companion entries (index line, ledger entry). The independent validator grades
trace closure over it; write it as part of the output, not as an afterthought.

Governance is authored **from a ratified session synthesis** —
`.mochiko/memory/governance-intent.md`, produced by the setup lead's interrogation session and
confirmed by the user before authoring begins. The synthesis is a **traceable contract, not a
brief**: it owns *selection* (which principles, at what tier, with which waivers and modules);
this skill owns *formulation* (wording, enforcement mechanics, surface routing).

This skill produces a **reviewable** surface set. Its quality is graded by an **independent
validator** (a separate agent running `validation-constitution`), and accepted at a named human
gate — the sequencing, the produce→validate→revise loop, and the human gates are owned by the
command lead that drives this skill, not by this skill.

## The synthesis contract (selection vs. formulation)

The non-negotiable discipline of this skill:

- **Every principle traces.** Each authored principle carries its GI trace: on `CLAUDE.md`, as an
  HTML comment beside the line (comments are stripped from context — zero cost); in a rules file
  and always in the **ledger**, as the `GI-XXX (deck-kept: CARD-ID | minted | floor-preset:
  CARD-ID)` key. The ledger entry is the canonical trace record (rules-file comment handling is
  undocumented — never rely on it alone).
- **Every element is realized or flagged.** Each principle-bearing synthesis element becomes a
  principle on some surface, or is surfaced as a flagged proposal — never silently dropped.
- **No unsanctioned selection.** Do not add, remove, merge, or reinterpret principles beyond the
  synthesis. If authoring reveals a genuine problem (a contradiction, a missing principle the
  project clearly needs, elicited intent that **resists enforceable formulation**), do not fold a
  fix in silently and do not author vagueness — emit a **flagged proposal**: what you propose,
  why, and which synthesis element (if any) it touches. Flagged proposals are ruled on by the
  user at the acceptance gate.
- **Waivers are authored, not skipped.** A waived floor category gets a waiver record in the
  ledger (category, waiving tier, revisit trigger, trace) — absence is always deliberate and
  auditable.

## Surface routing (which content lands where)

Route each synthesis element by its scope; the routing IS part of formulation quality:

- **Universal** (governs every session's work) → a **short imperative line in the CLAUDE.md
  region**. Keep it one to a few lines; the always-on budget is the scarcest resource — detail
  and metadata go to the ledger. Universal principles do NOT go to unconditional rules files
  (empirically, rules delivery to spawned producers cannot be assumed; CLAUDE.md is doc-confirmed
  for every spawn path).
- **Scope-bound** (governs work on a path-identifiable slice: layers, API surface, tests,
  frontend) → a **`paths`-scoped rules file** under `.claude/rules/mochiko/`, one concern per
  file, operative rules in the body, `paths` globs honest to the concern. Honest cuts both ways:
  the globs must cover **every path whose code can violate the concern** — including layers that
  orchestrate the governed operation through ports/interfaces — not just the layer that
  implements the mechanism. Test each glob set against the kept architecture card, layer by
  layer: "could code in this layer violate this rule?" — yes → its glob belongs in `paths`.
- **Scope-bound delivery caveat** (observed, kinako dogfood 2026-07-19): rules files inject on
  **Read** of a matching file, not on Write — an agent creating a new file under a scoped path
  never sees the rule, which is exactly the greenfield-scaffolding case. Whenever the set
  includes any rules file, emit the standing new-file read line in the region's Governance
  operations (per the template), naming the actual scoped paths. Word it as observed behavior,
  not doctrine — platform delivery may change.
- **Procedure-shaped** (a how-to, not a constraint) → a **pointer to the skill** that carries the
  procedure; the index line names it. Mint a new skill only for a session-minted procedure.
- **Every principle, regardless of home** → an **index line** in the region and a **ledger
  entry** (Three-Part + trace). Index → home → ledger must close; the validator checks it.

## Two modes, one shared core

| Mode | Use when | Adds on top of the shared core |
|------|----------|--------------------------------|
| **greenfield** | A new project with no existing code to honor. | Principles formulated from the synthesis's deck rulings + minted intents; floor cards at their tier parameterization. |
| **brownfield** | An existing codebase — *codify what is already there*. Requires `.mochiko/memory/codebase-analysis.md` (produced upstream by `analysis-codebase`). | Essential Floor *assessed against the code* (present/partial/absent) **+** an Emergent Ceiling codifying good existing patterns **+** the `evolution-notes` module. |

**The shared core is authored once, identically, in both modes** — the Three-Part Principle Rule,
RFC 2119 keywords, surface routing, the mandatory content inventory, and module assembly below.
The content sources:

- **both → [references/catalog/](references/catalog/README.md)** — the tier/type-tagged principle
  deck (universal floor + type shelves). The synthesis's deck rulings name which cards were kept
  and how they were adjusted; the cards carry the principle material to formulate from.
- **both → [references/ESSENTIAL-FLOOR.md](references/ESSENTIAL-FLOOR.md)** — the canonical
  definition of the four floor categories. Tier strictness and waiver posture per category live
  on the floor cards in the catalog.
- **brownfield → [references/EMERGENT-CEILING-PATTERNS.md](references/EMERGENT-CEILING-PATTERNS.md)**
  for the existing-pattern library.

Artifact shapes (region block, rules file, ledger):
[`governance-surfaces-template.md`](../../templates/governance-surfaces-template.md).

> **Mode prerequisites (lead-owned).** Authoring consumes a **ratified** synthesis. Brownfield
> additionally consumes `.mochiko/memory/codebase-analysis.md`. If the synthesis is missing, say
> so and stop — authoring without it reproduces exactly the producer-decides-selection failure
> the synthesis exists to prevent.

> **Ownership boundary (D8).** The governance region is regenerated **in place, idempotently**:
> replace only what sits between the markers; never touch user content outside them. Rules files
> under `.claude/rules/mochiko/` and the ledger are setup-owned and regenerated whole. In amend,
> preserve untouched principles verbatim (their GI-IDs are stable) and bump the region's semver.
> A `.mochiko/memory/constitution.md` on disk is superseded — the lead deletes it; never author
> into it.

## When to Use

- Authoring the governance surface set from a ratified synthesis, for a **new** project
  (greenfield mode).
- Authoring for an **existing** codebase, codifying its conventions without disrupting working
  code (brownfield mode).
- Amending an existing surface set against a delta-updated synthesis: adding/redefining
  principles, waivers, or tier, with a semver bump and an amendment-log entry.
- Establishing enforcement mechanisms, testability criteria, and amendment/version policy.

## When NOT to Use

- **Reviewing/grading an existing surface set** → that is the independent validator's job
  (`validation-constitution`), run by a different agent.
- **Eliciting what governance should contain** → that is the interrogation session's job
  (lead-conducted, upstream); this skill formulates a ratified synthesis, it does not interview.
- **Analyzing the codebase** → run `analysis-codebase` first; brownfield mode consumes its output.

## Common Mistakes (shared core)

| Mistake | Problem | Fix |
|---------|---------|-----|
| Vague principles | "Code should be clean" has no enforcement | Add specific, measurable criteria: "Functions MUST be ≤40 lines" |
| Missing enforcement | Principles without verification become suggestions | Every principle's ledger entry needs CI automation, code review checklist, or audit process |
| Untestable criteria | "Good architecture" can't be verified | Define binary pass/fail: "No domain imports from infrastructure layer" |
| No rationale | Future maintainers don't know why rules exist | Explain the failure mode prevented and success enabled |
| Selecting beyond the synthesis | Producer defaults silently override elicited intent — the failure this contract exists to prevent | Formulate only what the synthesis selects; everything else is a flagged proposal |
| Authoring vagueness for hard intent | Elicited intent that resists enforcement gets watered into an aspiration | Flag it as a proposal; the user rules at acceptance |
| Missing/fabricated trace stamps | Trace closure fails; or a plausible stamp hides a deviation | Key every ledger entry to its real GI source; deviations go through flagged proposals |
| Fat governance region | A verbose region re-creates the always-on cost the dissolution exists to fix | Region entries stay short-form; detail lives in the ledger or behind pointers |
| Universal principle in a rules file | Rules delivery to spawned producers is unproven; the principle silently misses authoring agents | Universal → CLAUDE.md region lines; rules files are for `paths`-scoped concerns only |
| Rule scoped to the mechanism's home layer only | Layers that orchestrate the operation (e.g. application use cases persisting through ports) never trigger the rule | Run the per-layer violation test; every layer that can violate the concern gets a glob |
| Rules files emitted without the new-file read line | Rules inject on Read, not Write — scaffolding writes governed code without ever seeing the rules | Emit the region's standing read-before-create line whenever any rules file exists |
| Editing outside the markers | Setup clobbers user-authored CLAUDE.md content | Regenerate only the region; everything outside the markers is untouchable |
| Index without a home, home without an index line | Trace closure fails; a principle exists nowhere or invisibly | Every principle = index line + primary home + ledger entry; write the trace summary as you author |
| One tier's numbers in another tier's governance | A poc graded like production, or production loosened to poc | Take tier parameterization from the floor cards; honor session overrides |
| Copying misfitting shelf examples | Backend-flavored examples (RFC 7807, `/health`) land in an SPA's governance | Formulate from the category definition, fitted to the declared type |

---

# Shared core (both modes)

## The Three-Part Principle Rule

Every principle MUST have three components, recorded in its **ledger entry**. A principle without
all three is incomplete and should not be accepted.

### 1. Enforcement

How compliance is verified. Without enforcement, a principle is a suggestion.

```markdown
**Enforcement**:
- CI runs `ruff check .` and blocks merge on violations
- Code review MUST verify test files accompany new functionality
- Quarterly audit checks exception registry for staleness
```

| Type | Examples | Strength |
|------|----------|----------|
| **CI Automated** | Linting, tests, coverage gates | Strongest—no human judgment needed |
| **Code Review** | Architecture compliance, security review | Strong—explicit checklist item |
| **Tooling** | Pre-commit hooks, IDE plugins | Medium—can be bypassed |
| **Audit** | Quarterly review, compliance check | Weaker—periodic, not continuous |

Enforcement MUST fit the team reality recorded in the synthesis — a solo project cannot lean on
"code review MUST verify"; give it tooling and CI instead. (Native surfaces are context, not
enforcement — the docs say so explicitly; the teeth are CI, hooks, review, audit, same as ever.)

### 2. Testability

What pass/fail looks like. A principle without testable criteria is merely an aspiration.

```markdown
**Testability**:
- Pass: `flutter analyze` exits with code 0
- Fail: Any file exceeds 400 lines without documented exception
```

Binary outcome; measurable threshold where applicable; observable without subjective judgment;
reproducible by any team member.

### 3. Rationale

Why this constraint exists. Explains the failure mode prevented, the success enabled, and
justifies the enforcement overhead.

## Principle Writing Format

**On the surface** (region line or rules-file rule) — the operative constraint only, RFC 2119
keywords, short:

```markdown
- API errors MUST use RFC 7807 problem+json. <!-- GI-007 -->
```

**In the ledger** — the full record:

```markdown
### GI-XXX — [Principle Name] · [home: CLAUDE.md | rules/mochiko/<file>.md | skill:<name>]

**Enforcement**: [how compliance is verified — specific commands or processes]
**Testability**: [pass/fail criteria, measurable thresholds]
**Rationale**: [why this constraint exists]
**Trace**: GI-XXX (deck-kept: CARD-ID | minted | floor-preset: CARD-ID)
```

## RFC 2119 Keywords

| Keyword | Meaning |
|---------|---------|
| **MUST / MUST NOT** | Absolute requirement / prohibition; no exceptions |
| **SHOULD / SHOULD NOT** | Recommended / discouraged; valid exceptions exist |
| **MAY** | Optional; implementation choice |

See [references/RFC-2119-KEYWORDS.md](references/RFC-2119-KEYWORDS.md) for detailed usage.

## Mandatory content inventory

Every governance set MUST include, per
[`governance-surfaces-template.md`](../../templates/governance-surfaces-template.md):

1. **Ratified stamp** (region, one line): version · ratified date · tier. The semver policy:
   MAJOR — principle removal / incompatible redefinition / tier change · MINOR — new principle or
   waiver change · PATCH — clarification.
2. **Principle index** (region): one line per principle — name, imperative gist, pointer to its
   home when the home is not the region itself.
3. **Universal principles** (region): the short imperative lines, floor principles first. At
   `production`/`regulated` tier, floor principles are marked `(NON-NEGOTIABLE)`.
4. **Technology stack** (region): mandated choices — from the synthesis's real-commands and
   existing-practices elements; brownfield populates from the codebase analysis.
5. **Quality-gates summary** (region): the blocking checks with **actual commands** (never
   placeholder tokens); coverage pre-seeds tier-parameterized from the FLOOR-TEST card unless the
   session overrode them. Gates for waived categories are omitted; the waiver record covers the
   absence.
6. **Scope-bound rules files**: per the routing; `paths` frontmatter honest to the concern
   (violation-coverage tested, per the routing's per-layer test). When any rules file is
   emitted, the region's Governance operations carries the standing new-file read line.
7. **Governance ledger**: tier + graduation path · waiver table (category, waiving tier, revisit
   trigger, trace; "None." when nothing is waived — waivers only at tiers whose posture permits,
   per [catalog/universal-floor.md](references/catalog/universal-floor.md)) · per-principle
   Three-Part entries · amendment process (tier bumps and un-waives are governance events routed
   back through setup's amend mode) · exception registry · amendment log.
8. **Trace summary**: the manifest — every GI element → primary home + companions; every surface
   element → its GI element.

> Every governance set MUST **account for** all four Essential Floor categories
> ([references/ESSENTIAL-FLOOR.md](references/ESSENTIAL-FLOOR.md)) — with a principle or, where
> the tier permits, a recorded waiver. A floor category with neither is a defect in either mode.

There is **no CLAUDE.md-synchronization section**: governance lives ON CLAUDE.md — the
sync-a-copy problem died with the dissolved artifact.

## Module assembly

Modules from [`templates/constitution-modules/`](../../templates/constitution-modules/) attach
per the synthesis's module selections — but **module content routes by surface, like everything
else** (any in-file attach instructions that predate the dissolution are superseded by this
table):

| Module | Attach when | Routes to |
|--------|-------------|-----------|
| `layer-rules` | A layered-architecture card was kept | `paths`-scoped rules files (one per layer concern) + index lines + ledger entries |
| `release-gates` | The deployment dimension elicited a real release process | Region: one summary line + pointer; detail in the ledger |
| `evolution-notes` | Mode is brownfield (always) | Ledger section (floor status, gap references, confrontation rulings) + region pointer |
| `knowledge-management` | The KM dimension elicited adoption (default-on, whole; a recorded decline is durable) | Region: the operating-manual **pointer** + index line; the bundle scaffolding and command carriers are unchanged (lead-executed at finalize) |

**Never route module content the synthesis didn't select** — an unselected module attached "to be
safe" is unsanctioned selection. Each module's validator checklist fragment still applies to its
routed content.

---

# Greenfield branch (formulate the synthesis)

No fixed default principle set: the synthesis's deck rulings and minted intents ARE the
selection. The job is formulation + routing quality.

1. **Floor principles** — for each floor card the synthesis keeps: formulate at the card's tier
   parameterization, fitted to the declared project type; route (floor principles are typically
   universal → region lines). Worked examples in ESSENTIAL-FLOOR.md are backend-flavored; for
   other types, formulate from the category definition instead of copying the example.
2. **Type principles** — for each kept shelf card: formulate from the card's content, honoring
   every recorded tighten/loosen ruling; these are typically scope-bound → rules files.
3. **Minted principles** — for each minted intent: structure the elicited intent into a
   three-part principle with real commands, routed by scope. Intent that resists enforceable
   formulation goes back as a flagged proposal, never as an aspiration.

# Brownfield branch (codify existing patterns)

Codify the **Essential Floor + Emergent Ceiling**: existing codebases have implicit conventions
worth preserving (Emergent Ceiling) but may lack foundational governance (Essential Floor).
Reuses the entire shared core above without restating it. Driven by **two** inputs: the ratified
synthesis and `.mochiko/memory/codebase-analysis.md` — read it for **"Strengths to Preserve"**
(ceiling candidates) and the **Essential-Floor status** (present / partial / absent per category).

**Floor, assessed against the code, waiver-aware:** category present → the principle codifies the
existing pattern with enforcement; absent → the principle states "MUST implement" and references
a roadmap gap; waived → a ledger waiver record, not a pretend-principle and not a gap. The
session already confronted detected-reality-vs-declared-intent conflicts — author the ruling the
synthesis records, never re-litigate it.

**Emergent Ceiling:** codify existing good patterns (naming conventions, architecture patterns,
error formats) as principles with enforcement — see
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

- **Independent validation** — after authoring, the surface set is graded by
  `validation-constitution`, run by a **separate validator agent** (never co-mounted with this
  skill). The produce→validate→revise loop and the human gates are owned by the command lead.
- **`analysis-codebase`** (in-cluster) — produces `.mochiko/memory/codebase-analysis.md`, the
  brownfield-mode input. Run before brownfield authoring (lead-sequenced).
- **`authoring-roadmap` / `evolution-roadmap`** (documented stub, *moved-to-other-cluster*) — turn
  the brownfield gap status into an improvement plan at `.mochiko/memory/evolution-roadmap.md`.
  **Not ported.** The Evolution Notes module spec stays; the roadmap producer is a planned stub.
- *(Retired 2026-07-18: the `syncing-claude-md` stub. Governance lives on CLAUDE.md — there is no
  constitution→CLAUDE.md copy left to synchronize.)*
