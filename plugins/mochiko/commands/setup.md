---
description: Establish or update project governance from the user's interrogated intent, on the surfaces Claude Code natively loads.
disable-model-invocation: true
---

# Setup — Governance From Interrogated Intent, On Native Surfaces

**Goal:** establish or update the project's governance so it follows the user's declared
intent — never a fixed baseline — and lives where Claude Code natively loads it. There is no
`constitution.md`. `$ARGUMENTS` = optional setup request; empty is fine — propose the mode
from what the workspace shows.

## Goal

The governance surface set exists and carries the user's ratified intent: the intent
synthesis was ratified by the user before any surface was authored; the trace from ratified
intent to authored surfaces closes across the set and an independent grade confirmed it from
the files; the governance region's semver is bumped; and the user accepted the set with the
trace summary in hand.

**Not done — default FAIL:** a surface authored before the intent was ratified · an unclosed
trace · the set never graded by anyone but its author · a floor category with neither
principle nor recorded waiver · user acceptance not given.

## Harness

- **You are the lead.** Plan the run and orchestrate it toward the Goal; teammates or
  subagents per seat is your call. Run the interrogation yourself, inline — the agenda's
  dimensions worked adaptively via `mochiko:analysis-iterative`, then the catalog deck,
  card by card, recommend-then-arbitrate.
- **Plan approval:** any seat that writes artifacts plans first and works only on a plan you
  approved; grading and fact-finding seats are exempt.
- **Independence:** no output is cleared by its author; grading reads the authored surfaces
  themselves — never the author's report — default FAIL. The synthesis is your pen, so its
  pre-ratification stress-test is always a cold seat, or the user's recorded waiver.
- **Reserved to the user:** the mode when ambiguous (greenfield / brownfield / amend) · every
  interrogation card ruling, module ruling, and waiver · synthesis ratification · detected
  reality vs declared intent conflicts (brownfield — confronted in the open, never silently
  resolved) · final acceptance of the surface set, flagged proposal by flagged proposal.
- Suggest commits; never run git mutations, never push. User acceptance is plain blocking
  text, never a timed prompt.

## Bindings

- **The surface set:** the marked governance region in `CLAUDE.md`
  (`<!-- mochiko:governance:begin/end -->`) · `paths`-scoped `.claude/rules/mochiko/*.md` ·
  skill pointers · `.mochiko/memory/governance-ledger.md` · the trace summary. Composition
  and read scope: `mochiko:authoring-constitution`.
- **The synthesis:** `.mochiko/memory/governance-intent.md`, `GI-XXX` namespace, from
  `templates/governance-intent-template.md` — durable amend baseline; never offer to delete
  it or the ledger.
- **Interrogation inputs:** `${CLAUDE_PLUGIN_ROOT}/skills/authoring-constitution/references/`
  — `INTERROGATION-AGENDA.md`, the `catalog/` deck, `DOMAIN-DEPENDENCIES.md`; brownfield
  analysis via `mochiko:analysis-codebase` into `.mochiko/memory/codebase-analysis.md`.
- **Ownership boundary:** the governance region is setup-owned and idempotently regenerated;
  everything outside the markers is user content, never touched. The template's marked
  carve-outs (domain registry, output-style pair) are preserved verbatim, never regenerated
  over the user's values. A `.mochiko/memory/constitution.md` on disk is superseded —
  delete on sight, say so in one line.
- **KM module:** adopted → scaffold per
  `templates/constitution-modules/knowledge-management.md`, including the project-pinned
  copy at `.mochiko/memory/knowledge-management.md`; its never-overwrite floor holds.
- **Register:** user-facing prose per `templates/output-style.md`.
- **Next step:** `/mochiko:specify` (+ `/mochiko:brainstorm` when knowledge-management was
  adopted).
