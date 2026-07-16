<!--
GOVERNANCE INTENT — the session synthesis (fill target)
========================================================
Written by the /mochiko:setup lead at the close of the interrogation session and ratified at the
synthesis-confirmation checkpoint BEFORE any authoring. Lives durably at
`.mochiko/memory/governance-intent.md`, beside the constitution it governs.

This artifact is a TRACEABLE CONTRACT on the constitution producer, not a brief:
- Every principle in the authored constitution MUST trace to exactly one element here
  (its GI-ID), and every principle-bearing element here MUST be realized in the constitution
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
-->

# Governance Intent — [PROJECT_NAME]

**Session date:** [YYYY-MM-DD] · **Mode:** [greenfield | brownfield | amend]
**Confirmed at synthesis checkpoint:** [YYYY-MM-DD] by [user]
**Governs:** `.mochiko/memory/constitution.md` v[X.Y.Z]

## Tier declaration

- **GI-001 — Tier:** `[poc | internal | production | regulated]` *(labels soft — record the
  project's own name if renamed)*
- **Graduation path:** [expected next tier + trigger, or "none expected"]
- **Rationale:** [why this tier — from dimensions 1, 2, 4]

## Project identity & type

- **GI-002 — Type:** [frontend | backend | fullstack | cli | library | service] → shelves dealt:
  [list]
- **Identity:** [what's being built, for whom, lifespan — 2-3 sentences]
- **Risk surface:** [what failure costs — the honest strictness driver]
- **Team reality:** [solo/team, review culture — what enforcement can lean on]

## Pruned dimensions

[Named skips only — "dimension 7 (deployment) skipped: poc tier, no release process." Or "none".]

## Real commands (dimension 6/7 → the validator's placeholder bar)

| Purpose | Command | Source |
|---------|---------|--------|
| Lint | `[actual command]` | [detected / declared] |
| Test | `[actual command]` | [detected / declared] |
| [etc.] | | |

## Deck rulings

One element per dealt card — **dropped cards are rulings too**:

| GI-ID | Card | Ruling | Adjustment (if any) |
|-------|------|--------|---------------------|
| GI-0XX | FLOOR-SEC | kept | [tightened/loosened how, or "at tier preset"] |
| GI-0XX | BE-HEX | dropped | [user's stated reason] |
| … | | | |

## Minted principle intents

One element per minted intent — traced to the elicited answer, never to prompting:

- **GI-0XX — [Intent name]:** [what to enforce, how strictly — the user's words distilled]
  *Elicited from:* [dimension 8 answer, quoted or closely paraphrased]

## Waivers

Only at tiers whose waiver posture permits it; every waiver is auditable:

| GI-ID | Floor category | Waiving tier | Revisit trigger |
|-------|----------------|--------------|-----------------|
| GI-0XX | [e.g. Observability] | [poc] | [tier graduation to internal, or explicit re-run] |

## Module selections

Template modules the synthesis selects (the validator checks core + exactly these):

| GI-ID | Module | Selected because |
|-------|--------|------------------|
| GI-0XX | [layer-rules] | [BE-HEX kept] |
| … | | |

## Deliberate exclusions (dimension 9)

- **GI-0XX:** [what governance will not cover at this tier + why]

## Amendment Log

[Empty on first ratification. Amend runs append one entry per delta:]
- [YYYY-MM-DD] — [GI-IDs added/edited/superseded] — [one-line rationale] — re-confirmed at
  synthesis checkpoint [date]
