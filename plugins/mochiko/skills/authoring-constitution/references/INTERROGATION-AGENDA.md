# Governance Interrogation Agenda

The agenda for the pre-authoring interrogation session that `/mochiko:setup` runs **before any
constitution is authored**. The session is conducted inline by the setup lead using the
`mochiko:analysis-iterative` questioning engine (one question per turn, format adapted to the
user's state) — this file supplies the *agenda*, the engine supplies the *craft*. Its output is the
session synthesis at `.mochiko/memory/governance-intent.md` (see
`templates/governance-intent-template.md`).

**The interrogation leads, the deck follows.** No catalog card is dealt until the dimensions that
select and filter the deck (tier, type) are elicited. Minted principles trace to elicited intent,
never to shallow prompting.

## The nine dimensions (in order, adaptively)

Work through these **adaptively, not as a fixed script** — one question per turn, skipping what an
answer has already settled, probing deeper where answers are vague.

| # | Dimension | What it elicits | Feeds |
|---|-----------|-----------------|-------|
| 1 | **Project identity & intent** | What's being built, for whom, expected lifespan | Framing for everything |
| 2 | **Scope tier** | poc / internal / production / regulated + expected graduation path | Floor strictness, waiver defaults, deck filtering — and the pruning license below |
| 3 | **Project type & shape** | frontend / backend / fullstack / CLI / library / service | Shelf selection |
| 4 | **Risk surface** | What failure costs: data loss, money, reputation, compliance, user trust | Honest floor strictness — tunes the tier preset |
| 5 | **Team reality** | Solo vs team, experience mix, review culture | Enforcement must fit who's enforcing |
| 6 | **Existing practices & tools** | Detected stack, CI, linters, tests; brownfield analysis feeds in here | The real commands the validator requires |
| 7 | **Deployment & release reality** | Target, environments, cadence, what blocks a release, rollback expectations | Quality Gates; Observability/Error-Handling strictness; sharpens the tier declaration |
| 8 | **Values & non-negotiables** | What the user insists on enforcing and explicitly refuses | The preference gap only they can fill; primary source for minted principles |
| 9 | **Deliberate exclusions** | What governance will not cover at this tier | Recorded waivers and exclusions |

**Dimension 8 phrasing pre-filters for enforceability.** Probe for values as *enforceable
behavior* ("what should CI or review block?"), not moods ("quality matters"). Elicited intent that
still resists enforceable formulation is not dropped and not authored as vagueness — the producer
flags it as a proposal for the user to rule on at acceptance.

## Low-tier pruning license

An early low-tier declaration (dimension 2 = `poc`, often `internal`) **licenses explicit agenda
pruning**: dimensions the tier forecloses are skipped **and named as skipped** ("skipping
deployment-reality — poc tier, no release process to govern"), with adaptive convergence handling
the rest. A poc session may legitimately be five questions long. Never silently skip — a named
skip is auditable; a silent one is a gap.

## Depth per mode

The interrogation runs in **all three modes** — it covers only what the mode leaves unknown:

- **Greenfield** — the full nine dimensions, subject to the pruning license.
- **Brownfield** — the codebase analysis pre-fills dimension 6; the session focuses on what code
  cannot say: intent, tier, risk, values. **Confront detected-reality-vs-declared-intent conflicts
  in the open** ("you declared production tier; the codebase has no tests") — never silently
  resolve them. A confrontation's resolution (tier adjusted, gap accepted into a roadmap, principle
  written as MUST-implement) is a session ruling, recorded in the synthesis.
- **Amend** — a micro-session scoped to the delta. An amendment that bumps the tier or un-waives a
  floor category is a governance event and gets the relevant agenda slice (dimensions 2, 4, 9 at
  minimum); a wording-level amendment may need no interrogation at all.

## After the dimensions: deck arbitration, then waiver rulings

1. **Deal the deck** — shelves selected by type (dimension 3), cards filtered/parameterized by tier
   (dimension 2), presets tuned by risk/values (dimensions 4, 8). Present cards with
   recommendations per their tier defaults; the user **keeps / tightens / loosens / drops /
   re-ranks** each. Use the recommend-then-arbitrate format from `analysis-iterative` — the user
   sorts and arbitrates supplied content, they are not asked to generate it. Every ruling is
   recorded.
2. **Mint** — turn dimension-8 intent that no card covers into minted-principle intents (what to
   enforce, how strictly — formulation is the producer's job later).
3. **Waiver rulings** — for any floor category the tier permits waiving and the user chooses to
   waive: record the category, the waiving tier, and the revisit trigger (default: tier
   graduation).

The session closes with the assembled synthesis presented at the **synthesis-confirmation
checkpoint** (confirm / edit / reject) — the gate that ratifies synthesis↔intent before any
authoring. That checkpoint is the setup lead's gate, defined in the command, not here.
