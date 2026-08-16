# Governance Interrogation Agenda

The agenda for the pre-authoring interrogation session that `/mochiko:setup` runs **before any
constitution is authored**. The session is conducted inline by the setup lead using the
`mochiko:analysis-iterative` questioning engine (one question per turn, format adapted to the
user's state) — this file supplies the *agenda*, the engine supplies the *craft*. Its output is the
session synthesis at `.mochiko/memory/governance-intent.md` (shaped by the `governance-intent`
schema — `mochiko-cli template governance-intent`, or
`plugins/mochiko/schemas/governance-intent.yaml` read raw).

**The interrogation leads, the deck follows.** No catalog card is dealt until the dimensions that
select and filter the deck (fact profile, type) are elicited. Minted principles trace to elicited
intent, never to shallow prompting.

**The agenda test (PO-D3, adaptive-depth 2026-08-11):** every question elicits a **fact** — no
question negotiates a per-check standard. The **one** exception is a single project-wide **depth
level** declaration (`low` | `high`), offered recommend-then-arbitrate; it is the ONE strictness
declaration in the agenda, never a revived per-check tier ladder and never per-check haggling. The
session sets the floor's *depth level* (the single dial), its *shape* (product-kind facts
translate each floor category into its correct expression), its *triggers* (fact-profile facts
attach compliance modules mechanically), and its *path* (brownfield facts set the ratchet's
starting point, never its target). The one deliberately arbitrated card layer is the
architecture-opinion card set (PO-D3's S7 carve-out).

## The ten dimensions (in order, adaptively)

Work through these **adaptively, not as a fixed script** — one question per turn, skipping what an
answer has already settled, probing deeper where answers are vague.

| # | Dimension | What it elicits | Feeds |
|---|-----------|-----------------|-------|
| 1 | **Project identity & intent** | What's being built, for whom, expected lifespan | Framing for everything |
| 2 | **Fact profile** | Industry · data classes · jurisdictions/markets · contractual commitments | Module triggers per [COMPLIANCE-MODULES.md](COMPLIANCE-MODULES.md) — each module-driving fact gets a consequence-stated confirmation (the S4 fail-safe). The depth level is the user's single declaration (D1), never fact-derived; facts shape each category's expression, never the level |
| 3 | **Project type & shape** | frontend / backend / fullstack / service / mobile / desktop | Shelf selection |
| 4 | **Risk surface** | What failure costs: data loss, money, reputation, compliance, user trust | Honest context for the floor's expression and the arbitrated card layer |
| 5 | **Team reality** | Solo vs team, experience mix, review culture | Enforcement must fit who's enforcing |
| 6 | **Existing practices & tools** | Detected stack, CI, linters, tests; brownfield analysis feeds in here | The real commands the validator requires |
| 7 | **Knowledge management** | Adopt or decline the `knowledge-management` module — the operating-docs layer (brainstorms + `index.md`, open-only `BACKLOG.md` + trail, `ROADMAP.md`, the decisions layer, `ARCHITECTURE.md`, `GLOSSARY.md`, plus the enforcement surfaces), offered default-on — **core taken whole, electives (`CHANGELOG.md` / `RUNBOOK.md`) per-doc** | Module ruling (recorded either way); elective rulings; finalize scaffolding; collision rulings |
| 8 | **Deployment & release reality** | Target, environments, cadence, what blocks a release, rollback expectations | Quality Gates; Observability/Error-Handling expression; the `release-gates` module offer — default-on for a deployed product, recorded either way — **always interrogated**: the target is software the team deploys and operates |
| 9 | **Values & non-negotiables** | What the user insists on enforcing and explicitly refuses | The preference gap only they can fill; primary source for minted principles |
| 10 | **Deliberate exclusions** | What governance will not cover, beyond the floor | Recorded exclusions — the floor itself leaves only by recorded waiver (D4), never by exclusion |

**Dimension 7 offers the knowledge-management module default-on: core whole, electives
per-doc.** Recommend adoption — the user must actively decline (the projects most needing
imposed structure are the least likely to ask for it). The **core** bundle
(`templates/constitution-modules/knowledge-management.md`) is adopted or declined as one unit —
a project for which the core feels heavy declines the module, not a fragment — while the
**electives** (`CHANGELOG.md` for release-shaped projects, `RUNBOOK.md` for deployed services)
are per-doc opt-ins elicited by project type; each elective ruling is recorded and durable
(re-ruled 2026-07-25, deliberately superseding the prior no-inner-menu clause). It sits here because
everything the call depends on (the fact profile, type, risk, team, detected practices) is declared
by the time dimension 6 completes, and brownfield doc evidence is still fresh. The ruling is
recorded in the synthesis **either way**; a decline also memorializes in dimension 10.
**Brownfield collision beat** — fired only when dimension-6 evidence shows an existing doc
colliding with a bundle name: the hard floor is *never overwrite or rename existing content*; the
default is to **codify** an existing doc whose semantics fit into the module role; a true semantic
collision (name taken, different meaning — e.g. a product feature roadmap at `ROADMAP.md`) is put
to the user with the evidence on the table — adopt the existing doc into the role, or nest the
module's artifact under `.mochiko/` — and the ruling lands in the synthesis.

**Dimension 9 phrasing pre-filters for enforceability.** Probe for values as *enforceable
behavior* ("what should CI or review block?"), not moods ("quality matters"). Elicited intent that
still resists enforceable formulation is not dropped and not authored as vagueness — the producer
flags it as a proposal for the user to rule on at acceptance.

## No pruning license

The retired tier ladder's low-tier pruning license is gone (PO-D2): every project here is a
deployed, operated, customer-facing product, so no dimension is foreclosed by declaration —
deployment reality in particular is always interrogated. Adaptive convergence still applies —
skip what an answer has already settled, and say so — but a convergence skip is bookkeeping,
never a scope ruling.

The depth-level declaration (below) does **not** revive that license: it is one project-wide
dial, not a per-dimension prune. Facts are fully elicited at both `low` and `high` — the level
changes how strictly the asserted code standards are authored, never which dimensions are asked
or which floor categories are present. A misfit standard still leaves only by recorded waiver,
at either level.

## Depth per mode

The interrogation runs in **all three modes** — it covers only what the mode leaves unknown:

- **Greenfield** — the full ten dimensions, adaptively.
- **Brownfield** — the codebase analysis pre-fills dimension 6; the session focuses on what code
  cannot say: intent, facts, risk, values. **Confront detected-reality-vs-floor conflicts in the
  open** ("the floor requires tests; the codebase has none") — never silently resolve them. A
  confrontation's resolution (a recorded waiver with justification, a gap accepted into a roadmap
  as MUST-implement, a principle codifying the fix) is a session ruling, recorded in the
  synthesis. Declared module-driving facts are cross-checked against the analysis (the S4
  fail-safe, per [COMPLIANCE-MODULES.md](COMPLIANCE-MODULES.md)); a declared-fact-vs-detected-
  reality conflict is confronted the same way.
- **Amend** — a micro-session scoped to the delta. An amendment that changes the fact profile
  (module attach/detach) or un-waives a floor category is a governance event and gets the
  relevant agenda slice (dimensions 2, 4, 10 at minimum); a wording-level amendment may need no
  interrogation at all. An amend run also offers, **once**, every module the synthesis records no
  ruling on — the answer is recorded either way, and a recorded decline is never re-asked
  (permanent until the user reopens it). **Legacy migration:** an amend run meeting a synthesis
  that still carries a tier declaration (the retired axis) supersedes it — never renumbered —
  elicits the fact profile once, and re-records existing waivers under the D4 model. A
  **depth-level flip** (`low`→`high`) is itself a governance-event amend — the flip ceremony
  below; a legacy synthesis carrying no depth declaration defaults to `high` at this amend
  (review fold #7), recorded without a ceremony since it never ran at `low`.

## The depth-level flip ceremony (D6)

Raising the level is a **conscious act, never automatic** — there is no watcher of any kind (D6).
A project moves `low`→`high` only by a deliberate `/mochiko:setup` rerun **in high mode**, which:

- re-deals the floor cards at the **high row** of each card, surfacing the **adherence delta** —
  the checks the project does not yet meet at the high level — for the user to see and rule on;
- records the flip as a governance event (a MAJOR semver bump; an amendment-log row), the declared
  level moving to `high` in the ledger;
- files each not-yet-met high check as an **interim transition-delta waiver** (D4) — a bounded
  transition exception, transition-scoped, naming its delta item, dying when that item conforms;
- is **one-way**: `high` is terminal and never returns to `low`.

The flip surfaces the gap and lets the user stage adoption through waivers; it never silently
tightens live checks under the team.

## After the dimensions: the depth level, the asserted floor, deck arbitration, then waiver rulings

0. **Declare the depth level** — offer the single project-wide depth level
   (`low` | `high`) recommend-then-arbitrate: recommend `low` for greenfield, `high` where reality
   warrants it (D2); the user rules. This is the ONE strictness declaration in the agenda — never
   a per-check negotiation, never a revived tier ladder. Record it in the synthesis (its GI
   element) and the ledger; the level is one-way (`high` terminal, D1/D2).
1. **Assert the floor** — the four floor cards enter the session
   ([catalog/universal-floor.md](catalog/universal-floor.md)): presented, not negotiated. Each
   card carries a two-row `low`/`high` form; the declared depth level selects which row is
   authored — the **low row** or the **high row** of the floor card. Breadth is invariant —
   every category is present at both levels — and compliance modules are level-blind (D7). Type
   facts shape each category's *expression* (translation, not waiver); tightening is always
   open; the only loosening route is a recorded waiver (step 5). Compliance modules attach
   mechanically from the fact profile per [COMPLIANCE-MODULES.md](COMPLIANCE-MODULES.md), each
   attachment recorded in the synthesis.
2. **Deal the arbitrated deck** — shelf cards selected by type (dimension 3), presets tuned by
   risk/values (dimensions 4, 9). Architecture-opinion cards (BE-HEX and kin) are the
   deliberately arbitrated layer (PO-D3's S7 carve-out): present with recommendations; the user
   **keeps / tightens / drops / re-ranks** each. Use the recommend-then-arbitrate format from
   `analysis-iterative` — the user sorts and arbitrates supplied content, they are not asked to
   generate it. Every ruling is recorded.
3. **Mint** — turn dimension-9 intent that no card covers into minted-principle intents (what to
   enforce, how strictly — formulation is the producer's job later).
4. **Layered-architecture beat** — fired when a layered-architecture card was kept **or** a
   layered intent was minted: record the `layer-rules` module ruling in the synthesis (either
   way), then run the domain-dependency seed arbitration per
   [DOMAIN-DEPENDENCIES.md](DOMAIN-DEPENDENCIES.md) — seeds and rulings land in the synthesis's
   Domain-dependency seeds table. An undiscussed registry degrades to "standard library only" —
   stricter than anyone ratified.
5. **Waiver rulings** — for any asserted standard the user chooses to deviate from: record the
   standard, the justification, and any revisit trigger (waivers are permanent pending the D4.1
   revisit — an expiry is the user's option, never a default). Legal-mandate module obligations
   are unwaivable (D4.2); contractual/non-legal module obligations are waivable like any other
   standard (ruled 2026-07-30, `po-narrowing-build-scope`).

The session closes with the assembled synthesis presented at the **synthesis-confirmation
checkpoint** (confirm / edit / reject) — the gate that ratifies synthesis↔intent before any
authoring. That checkpoint is the setup lead's gate, defined in the command, not here.
