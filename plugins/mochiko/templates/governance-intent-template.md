<!--
GOVERNANCE INTENT — the session synthesis (fill target)
========================================================
Written by the /mochiko:setup lead at the close of the interrogation session and ratified at the
synthesis-confirmation checkpoint BEFORE any authoring. Lives durably at
`.mochiko/memory/governance-intent.md`, beside the governance ledger of the surface set it
governs (there is no constitution.md — governance lands on native surfaces).

This artifact is a TRACEABLE CONTRACT on the surface-set producer, not a brief:
- Every principle in the authored surface set MUST trace to exactly one element here
  (its GI-ID), and every principle-bearing element here MUST be realized on a surface
  or surfaced as a flagged proposal at the acceptance gate.
- Producer latitude is confined to FORMULATION (wording, enforcement mechanics, three-part
  structure with real commands) — never SELECTION.
- The validator string-matches trace-IDs both ways (deterministic). Semantic fidelity of a
  stamped trace is judgment-grade residual risk, mitigated by the synthesis-confirmation
  checkpoint and the acceptance gate's trace summary.

AMEND RUNS UPDATE THIS FILE DELTA-WISE: touched elements are edited or superseded (never
renumbered), new elements take fresh IDs, untouched elements keep their IDs, and the Amendment
Log records each delta. The traceability cross-check always runs against this persisted,
updated file. LEGACY MIGRATION: an amend run meeting a tier declaration (the retired axis)
supersedes it — never renumbered — elicits the fact profile once, and re-records existing
waivers under the D4 model.

GI-ID rule: sequential GI-001, GI-002, … — unique forever within this file; never reuse a
retired ID.

CONFIDENCE MARKS (setup-adversarial-review D2): every GI element carries a mark from
`Confident / Assumed / Contested / Unsure / Deferred`, assigned by the lead at assembly from the
session's own indicators — updated DELTA-WISE on amend (untouched elements keep their marks).
The marks are the G3 intent review's prioritization substrate; they are lead-self-reported, so
the review audits them rather than trusting them. `Contested` = the user overruled a recorded
challenge with the steelman in view — the recorded basis is required, or the mark is a defect.

THE REVIEW SECTION is this file's durable record of the sized pre-G3 review (or its waiver) —
recovery keys off its state. The reviewers produce lead-adjudicated input; the Tier-2 validator
downstream issues the authoritative grade on the authored surface set (both still human-gated).
-->

# Governance Intent — [PROJECT_NAME]

**Session date:** [YYYY-MM-DD] · **Mode:** [greenfield | brownfield | amend]
**Confirmed at synthesis checkpoint:** [YYYY-MM-DD] by [user]
**Governs:** the governance surface set v[X.Y.Z] (CLAUDE.md governance region · `.claude/rules/mochiko/` · `.mochiko/memory/governance-ledger.md`)

## Fact profile

The module-driving facts (interrogation dimension 2), each with its consequence-stated
confirmation — negatives are recorded facts too (the S4 fail-safe, per
`authoring-constitution/references/COMPLIANCE-MODULES.md`):

- **GI-001 — Facts:** industry: [x] · data classes: [x] · jurisdictions/markets: [x] ·
  contractual commitments: [x] · **Mark:** [Confident | Assumed | Contested | Unsure | Deferred]
- **Modules triggered (mechanical):** [module list with trigger fact each, or "none — negatives
  confirmed: (each negative + its stated consequence)"]
- **Brownfield cross-check:** [declared facts vs `codebase-analysis.md` / DS-XXX / detected
  integrations — "consistent" or the confrontation's ruling] *(brownfield only)*

## Project identity & type

- **GI-002 — Type:** [frontend | backend | fullstack | service | mobile | desktop] → shelves
  dealt: [list] · **Mark:** [Confident | Assumed | Contested | Unsure | Deferred]
- **Identity:** [what's being built, for whom, lifespan — 2-3 sentences]
- **Risk surface:** [what failure costs — honest context for the floor's expression]
- **Team reality:** [solo/team, review culture — what enforcement can lean on]

## Convergence skips

[Named skips only — a dimension left unasked because an earlier answer settled it ("dimension 5
settled by dimension 1: solo founder"). A skip is convergence bookkeeping, never a scope ruling
— there is no pruning license. Or "none".]

## Real commands (dimension 6/8 → the validator's placeholder bar)

| Purpose | Command | Source |
|---------|---------|--------|
| Lint | `[actual command]` | [detected / declared] |
| Test | `[actual command]` | [detected / declared] |
| [etc.] | | |

## Floor expression & deck rulings

Floor cards enter asserted — their rows record *expression* (type translation), never a level
ruling; arbitrated cards (architecture-opinion and kin) record the user's ruling. **Dropped
arbitrated cards are rulings too:**

| GI-ID | Card | Layer | Ruling / Expression | Mark |
|-------|------|-------|---------------------|------|
| GI-0XX | FLOOR-SEC | floor-asserted | [expression shaped how, or "at floor level"] | [Confident] |
| GI-0XX | BE-HEX | arbitrated | [kept/tightened/dropped + user's stated reason] | [Confident] |
| … | | | | |

*(On a fact-profile or un-waive amend, affected rulings are re-dealt on the event's agenda
slice or force-re-marked — a stale `Confident` never carries an unexamined ruling forward.)*

## Minted principle intents

One element per minted intent — traced to the elicited answer, never to prompting:

- **GI-0XX — [Intent name]:** [what to enforce, how strictly — the user's words distilled]
  · **Mark:** [Confident | Assumed | Contested | Unsure | Deferred]
  *Elicited from:* [dimension 9 answer, quoted or closely paraphrased]

## Waivers

Any asserted standard may be waived with a recorded justification (D4) — **except legal-mandate
module obligations (D4.2)**. Waivers are permanent pending the D4.1 revisit; a revisit trigger
is the user's option, never a default. Every waiver is auditable:

| GI-ID | Standard (floor category / card / non-legal module obligation) | Justification | Revisit trigger (optional) | Mark |
|-------|---------------------------------------------------------------|---------------|---------------------------|------|
| GI-0XX | [e.g. FLOOR-TEST coverage gate] | [recorded reason] | [or "permanent (D4.1 pending)"] | [Confident] |

## Module selections

**Template modules** (the operating-docs layer — knowledge-management, layer-rules,
release-gates), ruled in session; *compliance modules attach mechanically in the Fact profile
above, never here.* **Declines are rulings too** (recorded, durable: amend runs offer only
modules with *no* recorded ruling here; a recorded decline is never re-asked until the user
reopens it). The validator checks core + exactly the adopted modules:

| GI-ID | Module | Ruling | Because | Mark |
|-------|--------|--------|---------|------|
| GI-0XX | [layer-rules] | adopted | [BE-HEX kept \| layered intent minted] | [Confident] |
| GI-0XX | [knowledge-management] | [adopted \| declined] | [offered default-on at dimension 7; a decline also memorializes in dimension 10] | [Confident] |
| … | | | | |

## Domain-dependency seeds (only when `layer-rules` is adopted)

Session-arbitrated registry seeds per `authoring-constitution/references/DOMAIN-DEPENDENCIES.md`
— domain-relevance filtered, trust-ranked, each level-cited; rejected seeds are rulings too:

| GI-ID | Dependency | Signal level | Ruling | Mark |
|-------|------------|--------------|--------|------|
| GI-0XX | [equatable] | [1 — Flutter Favorites, live-verified] | [kept \| dropped] | [Confident] |
| … | | | | |

## Deliberate exclusions (dimension 10)

- **GI-0XX:** [what governance will not cover + why — never a floor category (the floor leaves
  only by recorded waiver, above)]
  · **Mark:** [Confident | Assumed | Contested | Unsure | Deferred]

## Review

<!-- The durable record of the sized pre-G3 intent review — one block per run (amend runs append
a new block, event-scaled). Recovery keys off this section's state: no sizing ruling → open the
gate; sized but survivors undispositioned → respawn/continue the review; folded but unverified →
verify pass; waived or verified → G3. -->

**[YYYY-MM-DD] — [first ratification | amend]**

- **Sizing:** lead stated weight [element count · mark mix · reality-surface load]; recommended
  [pair | single | none] ([pair default on first ratification | event-scaled on amend:
  governance event → pair · lighter substantive delta → single · wording-level →
  none-with-recorded-waiver]); **user ruled:** [pair | single | none]
- *(if none)* **Waiver:** waived by [user] at the sizing gate — [stated reason]. The synthesis
  proceeds to G3 unreviewed; this record is the audit trail.
- *(otherwise)* **Review:** reviewer(s) [coverage / coherence lenses | solo]; **tally** [N raised
  → M merged survivors]; recommended status [ready | needs-revision | critical-gaps]
- **Survivor dispositions** (every survivor carries one):

  | # | Sev | GI element(s) | Finding | Disposition |
  |---|-----|---------------|---------|-------------|
  | S1 | [Critical] | [GI-0XX] | [one line] | [resolved / user-ruled / recorded-open — overruled → element marked `Contested`] |

- **Verify pass:** [PASS — folds confirmed by the coherence-lens/sole reviewer | pending]
- *(if any)* **G3-edit delta-pass:** [edited elements + the still-seated reviewer's outcome]

## Amendment Log

[Empty on first ratification. Amend runs append one entry per delta:]
- [YYYY-MM-DD] — [GI-IDs added/edited/superseded] — [one-line rationale] — re-confirmed at
  synthesis checkpoint [date]
