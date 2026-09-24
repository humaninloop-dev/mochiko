# Governance Interrogation Agenda

The agenda for the pre-authoring interrogation session that `/mochiko:setup` runs **before any
constitution is authored**. The session is conducted inline by the setup lead using the
`mochiko:analysis-iterative` questioning engine (one question per turn, format adapted to the
user's state) — this file supplies the *agenda*, the engine supplies the *craft*. Its output is the
session synthesis at `.mochiko/memory/governance-intent.md` (shaped by the `governance-intent`
schema — `mochiko-cli template governance-intent`).

**The interrogation leads, the deck follows.** No catalog card is dealt until the dimension that
selects and filters the deck (type) is elicited. Minted principles trace to elicited intent, never
to shallow prompting.

**The agenda test (PO-D3, adaptive-depth 2026-08-11, setup-product-agnostic 2026-09-24):** every
question elicits an **engineering fact** — one the codebase, the team, or the pipeline answers — and
no question negotiates a per-check standard or asks what the product is, whom it serves, or what its
failure costs. The **one** exception is a single project-wide **depth level** declaration
(`low` | `high`), offered recommend-then-arbitrate; it is the ONE strictness declaration in the
agenda, never a revived per-check tier ladder and never per-check haggling. The session sets the
floor's *depth level* (the single dial), its *shape* (type facts translate each floor category into
its correct expression), and its *path* (brownfield facts set the ratchet's starting point, never
its target). The one deliberately arbitrated card layer is the architecture-opinion card set
(PO-D3's S7 carve-out).

## The seven dimensions (in order, adaptively)

Work through these **adaptively, not as a fixed script** — one question per turn, skipping what an
answer has already settled, probing deeper where answers are vague.

The numbers are stable: dimensions 1 (project identity & intent), 2 (fact profile) and 4 (risk
surface) were struck by ruling (setup-product-agnostic D1, D3), and the seven that stay keep their
numbers.

| # | Dimension | What it elicits | Feeds |
|---|-----------|-----------------|-------|
| 3 | **Project type & shape** | frontend / backend / fullstack / service / mobile / desktop — read first from the code (brownfield) or from the architecture store's `Scope:` line when the store carries one; asked only when neither answers | Shelf selection |
| 5 | **Team reality** | Solo vs team, experience mix, review culture | Enforcement must fit who's enforcing |
| 6 | **Existing practices & tools** | Detected stack, CI, linters, tests; brownfield analysis feeds in here | The real commands the validator requires |
| 7 | **Knowledge management** | Adopt or decline the `knowledge-management` module — the operating-docs layer (brainstorms + `index.md`, open-only `BACKLOG.md` + trail, `ROADMAP.md`, the decisions layer, `ARCHITECTURE.md`, `GLOSSARY.md`, plus the enforcement surfaces), offered default-on — **core taken whole, electives (`CHANGELOG.md` / `RUNBOOK.md`) per-doc** | Module ruling (recorded either way); elective rulings; finalize scaffolding; collision rulings |
| 8 | **Deployment & release reality** | Target, environments, cadence, what blocks a release, rollback expectations · the **trust vectors** of what ships where — a binary, hooks, a public package, a deploy target's exposure — and the supply-chain controls they call for | Quality Gates; Observability/Error-Handling expression; deck presets (with dimension 9); the `release-gates` module offer — default-on for a deployed product, recorded either way — **always interrogated**: the target is software the team deploys and operates |
| 9 | **Values & non-negotiables** | What the user insists on enforcing and explicitly refuses — engineering values, minted under the product-instance test below | The preference gap only they can fill; primary source for minted principles |
| 10 | **Deliberate exclusions** | What governance will not cover, beyond the floor | Recorded exclusions — the floor itself leaves only by recorded waiver (D4), never by exclusion |

**Dimension 7 offers the knowledge-management module default-on: core whole, electives
per-doc.** Recommend adoption — the user must actively decline (the projects most needing
imposed structure are the least likely to ask for it). The **core** bundle
(`templates/constitution-modules/knowledge-management.md`) is adopted or declined as one unit —
a project for which the core feels heavy declines the module, not a fragment — while the
**electives** (`CHANGELOG.md` for release-shaped projects, `RUNBOOK.md` for deployed services)
are per-doc opt-ins elicited by project type; each elective ruling is recorded and durable
(re-ruled 2026-07-25, deliberately superseding the prior no-inner-menu clause). It sits here because
everything the call depends on (type, team, detected practices) is declared
by the time dimension 6 completes, and brownfield doc evidence is still fresh. The ruling is
recorded in the synthesis **either way**; a decline also memorializes in dimension 10.
**Brownfield collision beat** — fired only when dimension-6 evidence shows an existing doc
colliding with a bundle name: the hard floor is *never overwrite or rename existing content*; the
default is to **codify** an existing doc whose semantics fit into the module role; a true semantic
collision (name taken, different meaning — e.g. a product feature roadmap at `ROADMAP.md`) is put
to the user with the evidence on the table — adopt the existing doc into the role, or nest the
module's artifact under `.mochiko/` — and the ruling lands in the synthesis.

**Dimension 8 carries rules, not product values** (setup-product-agnostic D3). An SLO is declared
and measured — that rule stays in governance; the app-level SLO number set is a product value and
lives in the architecture store as a concern row, never as a governance element. The trust-vector
answers land in the synthesis's Trust vectors line.

**Dimension 9 phrasing pre-filters for enforceability and for product instances.** Probe for values
as *enforceable behavior* ("what should CI or review block?"), not moods ("quality matters"). A value
becomes a minted-principle intent only if it can be checked from the code, the pipeline or the stack
without knowing what the product does for its users (setup-product-agnostic D2): the rule stays, the
product's instance of it — its boundary list, its data's name, its behaviour toward its users —
leaves, and where governance needs the instance it points at its home. Elicited intent that fails
this test is neither authored nor flagged: it goes to the synthesis's **Handed off** list under
Deliberate exclusions, one line per intent in the template's shape. Elicited intent that passes the
test but still resists enforceable formulation is not dropped and not authored as vagueness — the
producer flags it as a proposal for the user to rule on at acceptance.

## No pruning license

The retired tier ladder's low-tier pruning license is gone (PO-D2): every project here is a
deployed, operated, customer-facing product, so no dimension is foreclosed by declaration —
deployment reality in particular is always interrogated. Adaptive convergence still applies —
skip what an answer has already settled, and say so — but a convergence skip is bookkeeping,
never a scope ruling.

A **struck** dimension is the one exception, and it is not a skip: dimensions 1, 2 and 4 left the
agenda by a single recorded scope ruling that binds every project (setup-product-agnostic D1, D3) —
not by declaration and not by convergence — and no session re-asks them.

The depth-level declaration (below) does **not** revive that license: it is one project-wide
dial, not a per-dimension prune. Facts are fully elicited at both `low` and `high` — the level
changes how strictly the asserted code standards are authored, never which dimensions are asked
or which floor categories are present. A misfit standard still leaves only by recorded waiver,
at either level.

## Depth per mode

The interrogation runs in **all three modes** — it covers only what the mode leaves unknown:

- **Greenfield** — the full seven dimensions, adaptively.
- **Brownfield** — the codebase analysis pre-fills dimension 6; the session focuses on what code
  cannot say: team, knowledge management, deployment reality, values, exclusions. **Confront detected-reality-vs-floor conflicts in the
  open** ("the floor requires tests; the codebase has none") — never silently resolve them. A
  confrontation's resolution (a recorded waiver with justification, a gap accepted into a roadmap
  as MUST-implement, a principle codifying the fix) is a session ruling, recorded in the
  synthesis. A declared fact the analysis contradicts (a declared type or toolchain the code
  belies) is confronted the same way.
- **Amend** — a micro-session scoped to the delta. A governance event opens an amend and gets the
  agenda slice it touches; the event set is closed at six (setup-product-agnostic D4): (1) the
  depth-level flip `low`→`high` (the ceremony below) · (2) a waiver added, lifted or re-grounded ·
  (3) a stack, toolchain or layout change — a real command changes, a surface type is added and a
  new shelf deals, or the repo layout the `paths`-scoped rules files bind to moves · (4) an
  engineering principle minted, dropped or redefined under the dimension-9 test · (5) an
  engineering module (`knowledge-management`, `release-gates`, `layer-rules`) attached or
  detached · (6) the deliberate exclusions change. A wording-level amendment (a PATCH clarification) is not an event and may
  need no interrogation at all. **A product change opens no amend except through event (3):** a
  new boundary, data class or market, a feature-map change, an architecture-store or
  design-baseline write, a moved SLO number — none opens an amend, because governance cites no
  product instance; a surface type added is event (3) by its engineering face, surfaced at the
  architecture desk's `Scope:` write, never by a watcher. An amend run also offers, **once**, every
  module the synthesis records no ruling on — the answer is recorded either way, and a recorded
  decline is never re-asked (permanent until the user reopens it). **Legacy migration
  (forward-only):** an amend run meeting a synthesis that still carries a retired element
  supersedes it in place, once — never renumbered, never carried forward: a tier declaration (the
  retired axis), its existing waivers re-recorded under the PO-D4 waiver model; a fact profile,
  attached compliance modules, or product-instance principles (setup-product-agnostic D4 — no
  interim marker; the new agenda is the only agenda). A
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
   every category is present at both levels. Type facts shape each category's *expression*
   (translation, not waiver); tightening is always open; the only loosening route is a recorded
   waiver (step 5).
2. **Deal the arbitrated deck** — shelf cards selected by type (dimension 3), presets tuned by
   deployment reality and values (dimensions 8, 9). Architecture-opinion cards (BE-HEX and kin) are the
   deliberately arbitrated layer (PO-D3's S7 carve-out): present with recommendations; the user
   **keeps / tightens / drops / re-ranks** each. Use the recommend-then-arbitrate format from
   `analysis-iterative` — the user sorts and arbitrates supplied content, they are not asked to
   generate it. Every ruling is recorded.
3. **Mint** — turn dimension-9 intent that no card covers into minted-principle intents (what to
   enforce, how strictly — formulation is the producer's job later) — only intent that passes the
   dimension-9 test; the rest goes to the Handed off list.
4. **Layered-architecture beat** — fired when a layered-architecture card was kept **or** a
   layered intent was minted: record the `layer-rules` module ruling in the synthesis (either
   way), then run the domain-dependency seed arbitration per
   [DOMAIN-DEPENDENCIES.md](DOMAIN-DEPENDENCIES.md) — seeds and rulings land in the synthesis's
   Domain-dependency seeds table. An undiscussed registry degrades to "standard library only" —
   stricter than anyone ratified.
5. **Waiver rulings** — for any asserted standard the user chooses to deviate from: record the
   standard, the justification, and any revisit trigger (waivers are permanent pending the D4.1
   revisit — an expiry is the user's option, never a default).

The session closes with the assembled synthesis presented at the **synthesis-confirmation
checkpoint** (confirm / edit / reject) — the gate that ratifies synthesis↔intent before any
authoring. That checkpoint is the setup lead's gate, defined in the command, not here.
