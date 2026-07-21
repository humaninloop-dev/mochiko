# Session Record — Domain-Layer Dependency Allowlist in Setup

**Opened:** 2026-07-21 · **Status:** accepted 2026-07-21 · bare session (direct
`mochiko:analysis-iterative` invocation) · record reviewed at user request: solo cold
devils-advocate subagent, lead pressure-tested every finding against cited files before
presenting — 9 raised → 7 confirmed / 1 weakened / 1 reframed / 0 refuted → 9/9 dispositioned
(3 user rulings)

## Problem statement

The dogfooding Flutter project's authored governance restricts the domain layer to the Dart SDK
only. The user wants a pragmatic posture: trusted, gold-standard libraries (e.g., Flutter
Favorites for Flutter; equivalents vary by ecosystem) should be admissible in the domain layer,
and `/mochiko:setup` should ensure an allowlist is discussed with the user — or otherwise
ensured — rather than defaulting to strict purity.

## Grounding facts (repo, checked at open)

- `plugins/mochiko/skills/authoring-constitution/references/catalog/backend-service.md` BE-HEX
  (lines 35–40) already carries **Approved Domain Dependencies**: qualification criteria
  (ubiquity >80% adoption + domain-relevance / no-I/O), example libraries for Python / TypeScript
  / Go / Rust — **no Dart/Flutter examples** — and "Projects SHOULD maintain their own registry
  as a project-level document."
- `plugins/mochiko/templates/constitution-modules/layer-rules.md` line 37 carries the **Domain
  Layer Note**: domain MAY import from the approved registry; registry is a project-level doc
  with per-dependency justification.
- The catalog has **no mobile/frontend shelf** — only `backend-service.md` and
  `universal-floor.md`; BE-HEX's type tags are backend / service / fullstack-api.
- **Nothing in the setup flow elicits or populates the registry** — no interrogation dimension,
  no arbitration sub-step; the card's registry concept is authored as a pointer, never
  discussed. BE-HEX's MAY-Import column reads "Standard library + approved domain deps" — with
  the registry never populated, a producer instantiating it collapses to "SDK only".

So the defect is not "purity is mandated" — the registry concept exists on paper. The gaps:
(1) no elicitation moment anywhere in setup, (2) no ecosystem trust-source grounding (Flutter
Favorites and equivalents), (3) no Dart/Flutter presence in the catalog at all.

## Decisions

- **D1 — Seed eagerly, grow lazily** (`Confident`, adopted on recommendation): when a
  layered-architecture card is kept, setup seeds the approved-domain-dependencies registry with
  a small arbitrated set for the detected stack (recommended from the ecosystem's trust source,
  user keeps/drops/tightens), AND the authored principle carries the qualification criteria +
  an add-process so implementation-time additions don't require re-running setup. Rationale:
  the dogfood failure shows an unexercised registry pointer degrades to strict purity (eager
  discussion necessary); a setup-frozen list would force a governance amend for every later
  addition (lazy growth necessary). Rejected: A (eager-only — list can't be complete at setup),
  B (lazy-only — today's design, produced "Dart SDK only").
  **Re-ratified post-review (F7 disposition, user ruling):** weighed against the steelmanned
  lazy-guided alternative (no seed; guided just-in-time elicitation on first need) — kept:
  with F1's composition filter the seed pool is small (~3–5 libs) and cheap to arbitrate, and
  a non-empty day-one registry is what kills the empty-list-reads-as-prohibition defect.

- **D2 — Tier-keyed add-gate with a visibility floor** (`Confident`, adopted on recommendation;
  streak-watch: 2nd consecutive unelaborated adoption): implementation-time registry additions
  are self-serve-with-justification at `poc`/`internal` but MUST be reported in the cycle
  report (visible, contestable, non-blocking); at `production`/`regulated` the addition
  surfaces to the user as an explicit ruling BEFORE entering the registry (flagged-proposal
  idiom). Rationale: tier parameterization is the house idiom (BE-HEX enforcement, review
  sizing), but silent registry divergence is tier-independent — hence the reporting floor even
  where the gate is open. Rejected: A (pure self-serve — registry becomes a log of decisions
  already taken), B (always-gated — poc ceremony outweighs risk).

- **D3 — Codified signal hierarchy, table-seeded** (`Confident`, adopted on recommendation;
  streak-watch: 3rd consecutive — streak flagged to user after this ruling): the card codifies
  a durable hierarchy of trust signals — official curation > semi-official stewardship >
  credible community curation > quantitative proxies + qualification criteria — with a starter
  table of known instances (Flutter Favorites / golang.org/x / .NET Foundation / PyPA-partial /
  blessed.rs; table verified for freshness at build time). Seeding walks the hierarchy top-down
  and cites which level each seed rests on; unlisted ecosystems get live research constrained
  to filling hierarchy levels. Level-citation makes weak/hallucinated sources challengeable at
  the D1 arbitration gate. Rejected: A (pure table — fallback case is the common case and
  reproduces the unguided collapse), B (pure research — can confidently cite curated lists
  that don't exist).
  **Amended by review (F1 disposition, user-confirmed):** trust and domain-admissibility are
  orthogonal axes — **domain-relevance (the no-I/O qualification criterion) filters the
  candidate pool FIRST; the trust hierarchy ranks and grounds only what passes.** A trust-only
  walk surfaces mostly-inadmissible candidates (Flutter Favorites skews I/O/UI) and misses the
  admissible pure-Dart libs. **(F8 fold):** starter-table entries are pointers re-verified
  live at seed time, never copied as snapshots.

**Streak flag (after D3):** three consecutive unelaborated recommendation-adoptions →
per the ratification-streak discipline, Q4 posed with steelmanned options and NO marked
recommendation, forcing a genuine read.

- **D4 — Rules-file-as-registry with a policy/list ownership split** (`Confident` — user
  initially leaned B ledger-canonical, arbitrated to A after counter-argument; streak broken,
  genuine read given): the registry's **list** (dependency rows: dep, justification, D3
  signal-level citation, D2 add-gate provenance) lives ONLY in a marked, preserve-in-place
  block inside the `paths`-scoped domain-layer rules file — delivered natively to any agent
  editing domain code, writable at implement-time under D2's gate, preserved across setup/amend
  regeneration (marker idiom precedent: CLAUDE.md governance region, KM never-overwrite floor).
  The **policy** (qualification criteria, D3 hierarchy, D2 add-process + tier gate) is stable
  setup-authored content: ledger-recorded, regenerated into the rules file's preamble. No copy
  of the list exists anywhere → nothing can drift; the validator checks the block's rows for
  required metadata at the next setup/amend run. Rejected: B (projection drift is
  invisible-by-design between setup runs — re-creates the surface≠intent defect; no clean
  implement-time write path: relaxing ledger read-only or micro-amend ceremony both
  contradict D2), C (pointer delivery is the failure mode the session exists to kill).

- **D5 — Trigger keys to `layer-rules` module attachment, dealt or minted** (`Confident`,
  adopted on recommendation — first post-streak-break adoption): the allowlist sub-arbitration
  (D1) and registry authoring (D4) hang off `layer-rules` module attachment, whose attach
  condition widens from "the synthesis kept a layered-architecture card" to "kept **or
  minted**" — one choke point covering every path a layered architecture enters a session,
  including the minted-intent path the Flutter dogfood project actually took. The mobile/app
  shelf (properly-flavored clean-architecture card for Flutter/Compose/SwiftUI, selecting
  `layer-rules`) is recorded as a **BACKLOG item**, not built here — workflow-first: it
  inherits an acceptance test from the dogfood run (a Flutter setup session deals flavored
  layered material instead of minting from nothing). Rejected: A (widening BE-HEX tags deals
  backend-flavored material to app sessions — a quality defect), B-now (balloons this
  session's build surface).

## Review (user-sized: solo cold devils-advocate subagent; lead pressure-tested each finding
## against the cited files before presenting)

**Tally: 9 raised → 7 confirmed, 1 weakened, 1 reframed as a re-ratification question. 0 refuted.**
Lead verification: all file citations sampled and accurate (CYCLE-REPORT-FORMAT.md frontmatter;
implement.md confidence-gate auto-approve; authoring-constitution SKILL.md "regenerated whole" +
module table; governance-surfaces-template.md ledger-canonical note; governance-intent-template.md
module-selections table; setup.md amend-only module offer).

- **F1 (Critical, CONFIRMED — hits D1/D3):** trust and domain-admissibility are orthogonal axes
  the session never composed. D3's walk is ordered by trust only; Flutter Favorites skews I/O/UI
  (dio, riverpod, shared_preferences — fail the no-I/O criterion) while genuinely admissible
  pure-Dart libs (equatable, dartz, collection) are largely not Favorites. Repair is a
  composition rule, not a redesign: domain-relevance filters the candidate pool FIRST; the trust
  hierarchy ranks and grounds what passes. → amendment to D3's text, user to confirm.
- **F2 (Critical, CONFIRMED — hits D2):** D2's runtime half has no landing surface: cycle-report
  frontmatter has no dependency field, and implement's confidence gate auto-approves
  all-deterministic-CLI cycles — so the production-tier "ruling before entry" can never fire and
  the visibility floor can't be met. → build items on implement-side machinery (cycle-report
  field + gate hook), missing from the session's list.
- **F3 (Critical, CONFIRMED — hits D4):** SKILL.md says rules files are "regenerated **whole**";
  D4's preserve-in-place block inverts that invariant, its cited precedents are different
  semantics (setup-regenerated block in user file; never-touched scaffolding), and until
  SKILL.md is edited the producer follows SKILL.md — silent loss of implement-added rows on the
  next amend. Lead nuance: the ledger-canonical-metadata half is weaker than stated (that rule
  exists because rules-file *comments* are undocumented; registry rows are body content). →
  build items: SKILL.md invariant amendment + preserve-block mechanics + validator fragment.
- **F4 (Important, CONFIRMED — hits D5):** module attachment is driven by a synthesis GI ruling
  (module-selections table), not the attach comment; the minted path needs an
  interrogation-side module-offer trigger, and the condition lives in ≥3 places (layer-rules.md,
  SKILL.md module table, interrogation flow). → D5's build items expand.
- **F5 (Important, CONFIRMED — missing dimension):** single-stack assumption throughout (D1
  "the detected stack", D4 "the domain-layer rules file"); a monorepo with two domain layers in
  two ecosystems reproduces the SDK-only collapse for the second stack. → needs a user ruling:
  design for it now, or explicit out-of-scope.
- **F6 (Minor after pressure-test, WEAKENED):** wrong-paths-glob risk (src/ vs lib/) is
  mitigated for the brownfield dogfood case (codebase-analysis supplies real paths; validator
  fragment requires real names + per-layer paths coverage) but real for greenfield Flutter
  until the mobile shelf lands. → noted on the BACKLOG mobile-shelf item.
- **F7 (REFRAMED — re-ratification question on D1):** the rejected "lazy-only" was its weakest
  form; the steelman (no eager seed; guided just-in-time elicitation on first need + authored
  criteria/process) was never weighed. Lead counter: with F1's composition fix the eager pool
  is small and cheap, and a non-empty day-one block is what kills the
  empty-registry-reads-as-prohibition defect. → user re-ratifies D1 or switches.
- **F8 (Minor, CONFIRMED):** freshness inversion — listed ecosystems seed from a build-time
  snapshot, unlisted get live research. Fix: table entries are pointers re-verified live at
  seed time, never copied as snapshots. → fold into D3.
- **F9 (Minor, CONFIRMED):** proportionality never sized (~3–5 row expected list vs full
  machinery). Lead counter: plugin-side machinery is once-authored; per-project cost is one
  small sub-arbitration + a small block. → noted; no change proposed.

## Dispositions (9/9)

F1 → D3 amended, user-confirmed · F2 → implement-side build items · F3 → SKILL.md invariant
amendment + preserve-block mechanics build items · F4 → D5 build items expanded (3 edit sites +
interrogation trigger) · F5 → ruled out-of-scope, BACKLOG item (user ruling) · F6 → note on the
mobile-shelf BACKLOG item · F7 → D1 re-ratified (user ruling) · F8 → folded into D3 · F9 →
noted, no change.

## Out of scope (explicit)

- **Multi-stack / monorepo registries (F5, user ruling):** per-stack registries, `paths`
  scopes, and seeding for repos with more than one domain-layer ecosystem — deferred to
  BACKLOG (workflow-first: no current project demands it). The design as ruled covers the
  single detected stack.
- Drift detection between setup runs (inherited from setup's own out-of-scope posture).

## Build items

1. **BE-HEX card + `layer-rules` module — allowlist sub-arbitration (D1, D3):** seed procedure
   with the F1 composition rule (domain-relevance filters candidates first; trust hierarchy
   ranks and grounds survivors, level-cited); signal hierarchy + starter table (Flutter
   Favorites / golang.org/x / .NET Foundation / PyPA-partial / blessed.rs — entries are
   pointers re-verified live at seed time, F8); user arbitrates keep/drop/tighten.
2. **`layer-rules.md` (D4, D5):** attach condition widens to layered card **kept or minted**;
   registry-block spec — marked preserve-in-place list block (rows: dep · justification ·
   signal-level citation · add-gate provenance) + regenerated policy preamble (criteria,
   hierarchy, add-process, tier gate); validator checklist fragment adds marker integrity +
   row-metadata completeness.
3. **`authoring-constitution` SKILL.md (F3, F4):** amend the "rules files … regenerated
   whole" invariant with the registry preserve-block carve-out; module-table row for
   `layer-rules` becomes "kept **or minted**".
4. **Interrogation / synthesis (F4):** minting a layered-architecture intent triggers the
   module offer so a `layer-rules | adopted` GI ruling lands in the module-selections table;
   `governance-intent-template.md` "Because" column admits minted provenance.
5. **Ledger template (D4):** policy section (criteria, hierarchy, add-process, tier gate) as
   the setup-owned canonical policy record.
6. **Implement-side (D2, F2):** cycle-report frontmatter field for domain-dependency
   additions (`executing-tdd-cycle` CYCLE-REPORT-FORMAT.md — the `poc`/`internal` visibility
   floor); implement.md confidence-gate hook — a registry addition at `production`/`regulated`
   forces a human checkpoint regardless of deterministic-CLI pass.
7. **BACKLOG items:** mobile/app shelf (flavored clean-architecture card selecting
   `layer-rules`; acceptance test: a Flutter setup run deals flavored material instead of
   minting; carries the F6 greenfield `paths`-glob caution) · multi-stack registries (F5).

## Kinako (dogfood) path

After the build lands: run `/mochiko:setup` amend on kinako — the minted-layered module offer
(build item 4) fires the allowlist sub-arbitration, seeding the registry per D1/D3 and
authoring the D4 registry block. Interim, the domain rules can be hand-edited; the amend run
supersedes them.

## Landing

**Built 2026-07-21 at plugin v0.18.0** (user go-ahead at conclusion). ROADMAP Key Decisions row
(2026-07-21) · BACKLOG: `approved-domain-deps.md`-fate item closed as resolved-by-this-session;
Domain-allowlist follow-ups section added (mobile/app shelf w/ F6 caution · multi-stack
registries · kinako amend dogfood); module-elicitation-scaling item gains the beat-not-dimension
datapoint. Build surface: new `authoring-constitution/references/DOMAIN-DEPENDENCIES.md` (single
source) · INTERROGATION-AGENDA arbitration step 3 (layered-architecture beat) · BE-HEX card
composition-rule rewrite + Dart examples · `layer-rules.md` (kept-or-minted attach, registry
block spec, validator fragment) · `authoring-constitution/SKILL.md` (ownership carve-out, module
table) · `governance-intent-template.md` (minted provenance, Domain-dependency seeds section) ·
`governance-surfaces-template.md` (ownership carve-out, ledger policy section) ·
`CYCLE-REPORT-FORMAT.md` (`domain_deps_added` field) · `implement.md` (confidence-gate hook +
human-gates line) · `setup.md` (interrogation beat clause) · `plugin.json` 0.17.0 → 0.18.0.
