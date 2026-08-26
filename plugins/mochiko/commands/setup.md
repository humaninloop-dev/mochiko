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
trace summary in hand. The feature map exists at close: brownfield reconstructed and
user-confirmed, greenfield an empty scaffold (feature-map binding below). `Assumed`
(feature-sizing record, open thread 4 — reconstruction burden, partial-baseline poisoning):
brownfield close also carries the bootstrapped product baselines at `.mochiko/product/`;
greenfield leaves **the baselines** to seed at the first implement run's design phase. The architecture store's
`spine.md` stub and its `Scope:` line are outside that split — written on **both** paths
(feature-map binding below).

**Not done — default FAIL:** a surface authored before the intent was ratified · an unclosed
trace · the set never graded by anyone but its author · a floor category with neither
principle nor recorded waiver · user acceptance not given · no feature map at close
(brownfield reconstruction unconfirmed, or greenfield index unscaffolded).

## Harness

- **You are the lead.** Plan the run and orchestrate it toward the Goal; teammates or
  subagents per seat is your call. Run the interrogation yourself, inline — the agenda's
  dimensions worked adaptively via `mochiko:analysis-iterative`, then the catalog deck,
  card by card, recommend-then-arbitrate.
- **Transport floor.** When the run composes more than one seat,
  `mochiko:patterns-transport-floor` governs its composition and messaging under a split
  trigger — message legs on any multi-seat messaging, topology legs on shared writes —
  non-waivable once triggered; referenced, never restated.
- **Model tiering.** Exploration and fact-finding dispatches ride
  `mochiko:patterns-model-tiering`'s class key — locate/enumerate reads to a native
  `Explore` subagent spawned `model: haiku`, interpretive or absence-driven reads on the session
  tier — and every seat brief carries the routing rule; referenced, never restated.
- **Plan approval:** any seat that writes artifacts plans first and works only on a plan you
  approved; grading and fact-finding seats are exempt.
- **Independence:** no output is cleared by its author; grading reads the authored surfaces
  themselves — never the author's report — default FAIL. The synthesis is your pen, so its
  pre-ratification stress-test is always a cold seat, or the user's recorded waiver.
- **Blind-map dispatch:** the pre-ratification stress-test seat is spawned in two messages —
  first the setup topic / project identity and goal only, *never* the synthesis path, so it
  builds its Phase 0 angle map blind to what the interrogation concluded; its map returns before
  you send the synthesis path and the cold read begins. In a pair, both seats build their maps
  independently.
- **Coverage-survivor routing:** a surviving coverage finding questions the setup's scope, not a
  card — present each gap as a candidate topic; **the user** rules the path: **explore now** (re-enter
  `mochiko:analysis-iterative` on that angle; the re-elicited intent lands in the `GI-XXX`
  namespace), **rule inline**, or **defer**. A gap that overlaps an agenda dimension keeps the
  ordinary interrogation-follow-up path.
- **Reserved to the user:** the mode when ambiguous (greenfield / brownfield / amend) · every
  interrogation card ruling, module ruling, and waiver · synthesis ratification · detected
  reality vs declared intent conflicts (brownfield — confronted in the open, never silently
  resolved) · confirmation of the reconstructed feature map, entry by entry (brownfield) ·
  final acceptance of the surface set, flagged proposal by flagged proposal.
- Suggest commits; never run git mutations, never push. User acceptance is plain blocking
  text, never a timed prompt.

## Bindings

- **The surface set:** the marked governance region in `CLAUDE.md`
  (`<!-- mochiko:governance:begin/end -->`) · `paths`-scoped `.claude/rules/mochiko/*.md` ·
  skill pointers · `.mochiko/memory/governance-ledger.md` · the trace summary. Composition
  and read scope: `mochiko:authoring-constitution`.
- **The synthesis:** `.mochiko/memory/governance-intent.md`, `GI-XXX` namespace, from the
  governance-intent template (rendered by `mochiko-cli template governance-intent`, or its schema
  `plugins/mochiko/schemas/governance-intent.yaml` Read raw when the binary is absent — the
  shipped schema is the first-class source of truth) — durable amend baseline; never offer to
  delete it or the ledger.
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
- **Feature map:** brownfield → the analysis extends into a feature-map reconstruction —
  delivered capabilities derived from the code (routes, UI surfaces, services), confirmed
  with the user, landing as the initial `FEATURES.md` + `.mochiko/features/` entries with
  `delivered` status and the reconstructed-from-code mark (shape: the features-index and
  feature-entry templates — `mochiko-cli template features-index` and
  `mochiko-cli template feature-entry`, or their schemas
  `plugins/mochiko/schemas/features-index.yaml` and
  `plugins/mochiko/schemas/feature-entry.yaml` Read raw when the binary is absent, the shipped
  schemas being the first-class source of truth; map machinery and the first-touch re-verify
  obligation: `mochiko:authoring-feature-map`).
  Greenfield → scaffold the empty index. The never-overwrite floor covers both writes.
  `Assumed` (feature-sizing record, open thread 4 — setup reconstruction burden +
  partial-baseline poisoning risk): the brownfield analysis also bootstraps the product
  baselines — `data-model.md` · `contracts/` · `constraints-and-decisions.md` ·
  `quickstart.md` — at
  `.mochiko/product/` (`ARCHITECTURE.md` stays repo root, now as the store's derived index) —
  from the delivered code; greenfield seeds **those baselines** at the first implement run's design phase instead.
  **The architecture store's scaffold is unconditional** — on both paths, `architecture/` is
  created with a `spine.md` stub whose header carries the `Scope:` line (below) and an empty
  `concerns.md` beside it, so the store's layout is complete from birth; greenfield's
  stub is simply a header with no topology under it. The store's **ruled content is never
  authored here** on either path: a file holding only a `Scope:` header is scaffold, not ruled
  content, and the first `/mochiko:architecture` visit is what elicits it (greenfield) or
  reconstructs it from what exists and confirms it with the user (brownfield).
- **Architecture scope handoff:** the run reads which surface types the product carries —
  `backend-service` · `frontend-web` · `mobile` · `desktop`, composed for a full-stack or
  monorepo product — and declares them on the **`Scope:` line in the header of
  `.mochiko/product/architecture/spine.md`**, written as a stub by the unconditional store
  scaffolding above — so **every** setup run, greenfield or brownfield, leaves the scope
  declared; the derived index renders it. The store carries the scope — setup does not hold it. It
  is a handoff, not a ruling: the desk deals the shelves that scope selects, and the user may
  **override it at the desk** by an ordinary store write to that same line. No architecture
  stance is taken here.
- **Register:** user-facing prose per `templates/output-style.md`.
- **Next step:** `/mochiko:specify` for the first feature, and `/mochiko:architecture` for the
  product's architecture baseline — peer doors, neither ahead of the other (+
  `/mochiko:brainstorm` when knowledge-management was adopted).
