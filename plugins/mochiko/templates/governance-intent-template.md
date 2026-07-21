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
updated file.

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

## Tier declaration

- **GI-001 — Tier:** `[poc | internal | production | regulated]` *(labels soft — record the
  project's own name if renamed)* · **Mark:** [Confident | Assumed | Contested | Unsure | Deferred]
- **Graduation path:** [expected next tier + trigger, or "none expected"]
- **Rationale:** [why this tier — from dimensions 1, 2, 4]

## Project identity & type

- **GI-002 — Type:** [frontend | backend | fullstack | cli | library | service] → shelves dealt:
  [list] · **Mark:** [Confident | Assumed | Contested | Unsure | Deferred]
- **Identity:** [what's being built, for whom, lifespan — 2-3 sentences]
- **Risk surface:** [what failure costs — the honest strictness driver]
- **Team reality:** [solo/team, review culture — what enforcement can lean on]

## Pruned dimensions

[Named skips only — "dimension 8 (deployment) skipped: poc tier, no release process." Or "none".]

## Real commands (dimension 6/8 → the validator's placeholder bar)

| Purpose | Command | Source |
|---------|---------|--------|
| Lint | `[actual command]` | [detected / declared] |
| Test | `[actual command]` | [detected / declared] |
| [etc.] | | |

## Deck rulings

One element per dealt card — **dropped cards are rulings too**:

| GI-ID | Card | Ruling | Adjustment (if any) | Mark |
|-------|------|--------|---------------------|------|
| GI-0XX | FLOOR-SEC | kept | [tightened/loosened how, or "at tier preset"] | [Confident] |
| GI-0XX | BE-HEX | dropped | [user's stated reason] | [Confident] |
| … | | | | |

*(On a tier-bump amend, tier-loosened rulings are re-dealt on the bump's agenda slice or
force-re-marked — a stale `Confident` never carries a lower-tier loosening upward unexamined.)*

## Minted principle intents

One element per minted intent — traced to the elicited answer, never to prompting:

- **GI-0XX — [Intent name]:** [what to enforce, how strictly — the user's words distilled]
  · **Mark:** [Confident | Assumed | Contested | Unsure | Deferred]
  *Elicited from:* [dimension 9 answer, quoted or closely paraphrased]

## Waivers

Only at tiers whose waiver posture permits it; every waiver is auditable:

| GI-ID | Floor category | Waiving tier | Revisit trigger | Mark |
|-------|----------------|--------------|-----------------|------|
| GI-0XX | [e.g. Observability] | [poc] | [tier graduation to internal, or explicit re-run] | [Confident] |

## Module selections

Template modules the session ruled on — **declines are rulings too** (recorded, durable: amend
runs offer only modules with *no* recorded ruling here; a recorded decline is never re-asked
until the user reopens it). The validator checks core + exactly the adopted modules:

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

- **GI-0XX:** [what governance will not cover at this tier + why]
  · **Mark:** [Confident | Assumed | Contested | Unsure | Deferred]

## Review

<!-- The durable record of the sized pre-G3 intent review — one block per run (amend runs append
a new block, event-scaled). Recovery keys off this section's state: no sizing ruling → open the
gate; sized but survivors undispositioned → respawn/continue the review; folded but unverified →
verify pass; waived or verified → G3. -->

**[YYYY-MM-DD] — [first ratification | amend]**

- **Sizing:** lead stated weight [element count · mark mix · reality-surface load]; recommended
  [pair | single | none] ([tier-keyed per D3 | tier × delta weight per D6]); **user ruled:**
  [pair | single | none]
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
