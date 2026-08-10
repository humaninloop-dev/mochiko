# Feature Map Layer — Decision Record

**Status:** accepted (2026-08-10)
**When:** 2026-08-09/10
**Session:** `/mochiko:brainstorm` — feature map between user stories and slices in specify/plan; whether slices survive; feature graduation into repo-level artifacts.

## Driver (user's framing, verbatim intent)

The user wants to change the specify and plan workflows to understand where a **features map** based on user stories is best created. Their read: it is a big gap to jump straight from user stories to slices without an understanding of what the actual features are. They are considering why slices are even needed if a feature map can be created. Additionally: features need to be managed and graduated into repo-level artifacts once delivered. Big change — many angles wanted.

## Ground facts (read at session open)

- **F1** — `specify.md` (v8 goal+harness form): pipeline inside specify is Intent stage → P1/P2/P3 user stories + FR-XXX + SC-XXX → Screens & Flows (SCR/FLOW manifest) → **Delivery Slices section** (graduation-slice decomposition or single-slice line). No "feature" unit exists between story and slice; the feature IS the spec (`.mochiko/specs/<feature>/spec.md`, one kebab-case slug per specify run).
- **F2** — `authoring-slices` SKILL.md: graduation slices group **user stories** into ordered pipeline units (exactly-one-home, dependency closure, foundation slice, Feature-Done map with SC coverage + seams, Graduation contract). Vocabulary table explicitly distinguishes graduation slices (spec level) from vertical-slice cycles (implementation level, `patterns-vertical-tdd`).
- **F3** — `plan.md`: slice-scoped runs land under `slices/<slice>/`; the Graduation contract is the single home for slice resolution/scope/layout. Architecture delta seeds from accumulated feature-root `architecture.md` / repo `ARCHITECTURE.md`.
- **F4** — Repo-level graduation today is **architecture-only**: `ARCHITECTURE.md` accumulates via slice-landing folds + an In-flight pointer list (added at plan sign-off, removed at feature close — `architecture-tieback` session, AT-D1–D6). No repo-level artifact tracks delivered *features* as features; no FEATURES.md-analog exists anywhere in the plugin or KM module.
- **F5** — Prior ruling context: `plan-task-granularity` (2026-08-02) dissolved `/mochiko:slice` into specify and made slices a spec section; `production-only-focus` scopes mochiko to customer-facing product applications.

## Decisions

### D1 — Feature altitude: both levels (map in specify, graduating to repo level) — `Confident`

**Statement:** the feature concept lives at both altitudes (option C): a features map is authored during specify (below/alongside the spec), and delivered features graduate into a repo-level artifact at feature close.

**Rationale (user's words, compressed):** the pain is not knowing, from user stories alone, what exact feature will be built. Plan and tasks break down user *stories*, not features — so the built-unit vocabulary never matches the capability vocabulary anywhere in the pipeline.

### D2 — Feature = built capability, a standing artifact peer to architecture — `Confident`

**Statement:** a feature is the built thing — a core capability of the system describing what it *does* — not a cluster/regrouping of user stories. User stories inform *which* features get built; the feature description must stand on its own, together with the architecture, as a durable description of the system.

**Rationale:** stories carry user value in user language; they don't describe the shape of the delivered capability. Implication noted at ruling: this makes the repo-level artifact a living peer of `ARCHITECTURE.md`, not a delivery log.

**Amended at Q3 (user correction):** "what it does" was too narrow a reading. The feature map is the **broad view of the whole system expressed as features** — the primary capability lens on the product (capabilities, their extent, their relations, delivered vs in-flight), the way `ARCHITECTURE.md` is the system viewed as components. Not one behavioral appendix; a map.

### D3 — Feature map + ARCHITECTURE.md = central source of truth; feature delivery is independent of spec delivery — `Confident`

**Statement:** the feature map works together with `ARCHITECTURE.md` as the central source of truth of the product. User stories *inform* features — they help enhance and sharpen them — but do not define them. Feature delivery and spec delivery are **independent axes**: one spec can require multiple distinct features, and a run may deliberately build only a subset of those features because that subset aligns with the product need at that moment.

**Rationale:** features are durable product-level units that outlive any one spec; specs are delivery vehicles that touch features. Consequence flagged at ruling: "build a subset of the features a spec requires" is structurally the job the Delivery Slices section currently does with stories — the slice question (why slices, if a feature map exists?) now has a concrete shape: does slicing become *feature selection*?

### D4 — Slices die; the feature is the pipeline unit — `Confident`

**Statement:** graduation slices are retired. The delivery unit becomes the **feature selection**: the spec maps stories onto features, the user picks which features to build now, and the selected feature (or set) graduates through plan/implement. The slice machinery's invariants transfer onto features — dependency-closed ordering, foundation designation (a first feature that establishes the shared core AND delivers a testable journey), exactly-one-home for stories, and the Feature-Done map's SC-coverage + seams obligations re-keyed to features.

**Rationale:** features already carry the two properties slice invariants exist to guarantee — dependency structure and independent value. A second grouping layer would double ceremony for the same judgment. User: "maps exactly to what I had in mind."

**Named risk (open):** a single feature can still be too big for one graduation pass. Working answer: within-feature increments are `patterns-vertical-tdd` cycles at implement (the existing downstream mechanism); if a real run shows a feature too big even for that, revisit.

### D5 — One living repo-level feature map; no per-spec copy — `Confident`

**Statement:** the repo-level feature map is the **single working surface** — there is no per-spec feature artifact. Specify reads and writes the map directly: it proposes new or changed features (status `proposed`/`in-flight`) derived from the stories and intent; plan confirms and hardens them alongside architecture (components tied to features, intended-vs-designed drift surfaced back to the user); feature close flips status to `delivered`. The spec references feature IDs; it never carries its own feature descriptions.

**Rationale:** D3 makes features durable product units independent of specs — a per-spec fragment would have to fold into the map later, and the fold is where drift and duplicates creep in (the `architecture-tieback` lesson). One home, status-marked lifecycle (`proposed / in-flight / delivered`), three touchpoints (specify proposes · plan confirms · close graduates).

### D6 — Feature entry shape (starting point, refine at build) — `Assumed` (accepted as "very good starting point", not line-by-line ratified)

**Statement:** a feature entry carries: **ID + name** (`FEAT-XXX`, durable, referenced by specs / cycle cards / architecture) · **capability statement** (1–3 lines, system's own language) · **status** (`proposed / in-flight / delivered`, + date and owning-spec pointer while in flight) · **extent** (what's in and notably not in — where stories sharpen the feature over time) · **relations** (depends-on / extends / composes-with other FEAT-IDs) · **architecture link** (which `ARCHITECTURE.md` components realize it, navigable both directions) · **story trace** (which specs/stories informed it, accumulating provenance).

**Granularity guide:** a feature is a capability a product person would name in one breath — bigger than a story, smaller than a product area; extent not statable in ~3 lines means it's two features.

**Mark rationale:** user accepted the proposal as a starting point without per-line arbitration; the shape is expected to be refined during the build (template authoring) and first dogfood.

### D7 — Stories first, features derived — with the map-read-at-intent rider — `Confident`

**Statement:** inside specify the ordering is **stories first, features derived** (option A): Intent → stories drafted → features proposed/updated on the map from what the stories imply → selection. Rider (critical, user-ratified): the **existing feature map is an obligated read at the intent stage** — capability context is *input* to story drafting even though feature *writes* happen only after stories exist. Drafting stories blind to the map is the named failure mode (duplicate "new" features, ignored extension points).

**Rationale:** the pipeline's evidence flows user-first (intent, stories, prototype clicks); features derived from real stories inherit that honesty, while features sketched first are hypotheses stories get bent to confirm — the worse failure mode for a durable source-of-truth map. A's weakness (no capability picture mid-run) is transient within one run and resolved by the derivation step before selection. Asked at a flagged adoption-streak fork with steelmans and no recommendation; user chose A, then asked for and received an honest assessment confirming it with the rider.

**Corollary (user-added):** the derivation step is also a **filter** — the pipeline must be able to say **no** to user stories. Not every story becomes (or joins) a feature; a story that doesn't earn a place on the capability map can be rejected or deferred rather than silently inflating the system. Without the map read, "all stories become features" is the big weakness.

### D8 — Delivered is sticky; re-touch rides as a marked delta — `Confident`

**Statement:** when a later spec touches a `delivered` feature, the feature's status never regresses. The extension/change rides as a **marked delta on the entry** ("extent grows by X — in-flight, spec-N") until that work's feature close folds it into the extent lines. New features still enter as `proposed`.

**Rationale:** the map's source-of-truth role requires it to stay truthful about what production has — a regressed status would claim the product lacks a capability it demonstrably ships. Mirrors `ARCHITECTURE.md`'s existing convention (delivered baseline + In-flight pointer list), so the two peers behave identically and readers learn one convention. Versioned features (C) rejected as bookkeeping too heavy for an at-a-glance map.

### D9 — Specs are delivery events; the map carries the remainder — `Confident`

**Statement:** a spec closes when its **selected** features deliver. Features the spec surfaced but did not select stay on the map as `proposed`, with the spec pointer as provenance only. Building a `proposed` feature later means a fresh specify run — which re-reads the map at intent (D7 rider), picks up the `proposed` entries, and attaches fresh or inherited stories. Specs are records of delivery decisions, never standing containers.

**Rationale:** a spec held open as a container drifts stale against the living map — two sources of truth again, the exact failure D5 killed. Routing the remainder through BACKLOG duplicates the map (its `proposed` rows *are* the capability backlog). One rule: **the map is durable, specs are delivery events.** The `proposed` entry with story-trace provenance is the surviving thread.

*(D9 walkthrough: a two-run invoicing example was played end-to-end at session — filter rejecting a story, a proposed feature surviving six months and being picked up by a later run's map-read, a D8 delta on a delivered feature folding at close. User: matches broadly; triggered the D10 operational restructure.)*

### D10 — Spec workspace restructures brainstorms-style: index.md + per-story files; map owns status — `Confident`

**Statement:** the one-big-`spec.md` workspace restructures like `.mochiko/brainstorms/`:

- `.mochiko/specs/index.md` — one entry per spec folder (slug · status · FEAT-IDs touched with outcomes · one-line about), newest first, maintained by the same open/close contract as the brainstorms index.
- Per spec folder: `spec.md` keeps the core (intent, FRs, SCs, edge cases, Screens & Flows); **stories break out into `stories/US-*.md` files**, each carrying its text, acceptance scenarios, and its FEAT-ID mapping; plan artifacts land per-feature (keyed by FEAT, replacing `slices/<slice>/`).
- **Status ownership (sub-ruling, option A, "for sure"):** the feature map is the sole owner of capability status. A story's delivered/proposed status is *derived* by following its FEAT-ID — story files never carry their own copy. The only story-native status is `rejected` (the D7 filter's verdict, with the why recorded in the story file).

**Rationale:** the index gives specs the at-a-glance layer brainstorm sessions have; per-story files surface per-story disposition instead of burying it in a monolith. Status derivation kills the two-surfaces-lying-to-each-other drift before it exists — D5's one-status-home principle extended down to stories.

### D11 — FEATURES.md at repo root as a succinct index; entries live in per-feature files — `Confident`

**Statement:** the map's home is repo-root **`FEATURES.md`** — but as a **succinct index, never a monolith** (user counter-ruling over the flat-file option): one line per feature (FEAT-ID · name · status · one-breath capability hook), the same shape as the brainstorms/specs indexes. Full entries (D6 shape: extent, relations, architecture link, story trace, deltas) live in per-feature files in a features directory the index points at (working layout: `.mochiko/features/FEAT-XXX-<slug>.md`; exact home fixed at build). Repo root gains one succinct file; the top-level reservation amendment covers it, joining `ARCHITECTURE.md`/`GLOSSARY.md` in the KM core-when-content set.

**Rationale:** a massive single features file is unmaintainable and unreadable at a glance — the index-plus-records pattern is already the repo's proven answer (brainstorms, specs per D10, DECISIONS.md over records). Root visibility satisfies D2/D3's stands-on-its-own, central-source-of-truth ambition; the index form keeps it succinct enough to actually be read.

### D12 — Brownfield bootstrap: setup reconstructs the initial map — `Confident`

**Statement:** on an existing codebase, `/mochiko:setup`'s brownfield analysis gains a **feature-map reconstruction**: delivered capabilities derived from the code (routes, UI surfaces, services), confirmed with the user, landing as the initial `FEATURES.md` + per-feature entries with `delivered` status. Reconstructed entries carry a **reconstructed-from-code mark**; the first spec that touches such a feature re-verifies its extent before building on it. Specify's map-read (D7 rider) can then assume a map exists wherever setup has run; a missing map is surfaced like a missing governance region (offer `/mochiko:setup`), never silently tolerated.

**Rationale:** without bootstrap, D7's filter, dedup protection, and extension-point detection silently no-op on every existing product. Setup is where brownfield truth is already established, and the exact precedent exists — plan's architecture-baseline rule (absent → reconstructed, user-confirmed, landed). A lazily-grown partial map breaks the central-source-of-truth promise; a dedicated command adds surface the repo has deliberately shrunk.

### D13 — No feature-close stage; implement's acceptance landing absorbs it — `Confident`

**Statement:** there is **no distinct feature-close moment**. The user challenged the construct ("do we need a feature close? why?") and the lead's own test dissolved it — "feature close" was slice vocabulary carried over, holding nothing that needs a new stage:

- Status flip, D8 delta fold, `FEATURES.md` index touch, in-flight pointer clear — bookkeeping edits that join implement's existing acceptance landing (where `ARCHITECTURE.md` already folds, AT-D2).
- Per-feature SC coverage — already implement's TEST-gate verification.
- Cross-feature seams — owned by the **later-landing feature**: under D4's dependency ordering, a seam exists only when its second side lands, and that feature's TEST gates exercise it against the real delivered first side. No end-of-spec sweep.
- Spec index closure — derived state (spec closed when its selected FEAT-IDs read `delivered`); one index-line edit at the same landing.

**Reading rule:** wherever D5/D8/D9 say "feature close," read "the landing inside implement's acceptance."

**Rationale:** every obligation the phrase carried already has a home; a new ritual would be ceremony without a job. Recorded as amendment-by-reading-rule rather than rewriting D5/D8/D9, keeping the trail honest.

### D14 — New `product-manager` agent with the wide product-layer remit — `Contested`

**Statement:** a new **`product-manager`** agent joins the roster with the **wide remit** (option C, user-ruled over the lead's A recommendation): the whole product layer — feature-map derivation and writes, the D7 filter's verdicts, selection advice, intent-stage capability input, and spec-index stewardship. Persona carries capability thinking, portfolio judgment, and the discipline to say no; procedure lands in a new `authoring-feature-map` skill (derivation method, entry shape, map-write rules, the intent-stage map-read agenda).

**Rationale (user):** this is a big change; a big blast radius is acceptable — the product layer should have one owner, not judgment scattered across seats.

**Lead's contested points, preserved:** (1) A was purely additive while C reworks specify's intent stage — regression surface accepted by the user with eyes open; (2) structural fact unresolved to build time: the intent stage is a lead-run user conversation, so the PM's intent-stage ownership will in practice mean the PM's skill/brief shapes the lead's agenda (or the PM seat is consulted mid-intent), not the PM replacing the lead as facilitator — the build must pick the mechanism; (3) selection stays the **user's** ruling — the PM recommends, never selects.

**Consequence (user-raised, next):** the `requirements-analyst`'s continued existence is now in question — its story-authoring remit overlaps the PM's product layer.

### D15 — PM and requirements-analyst both stay, layered — `Confident`

**Statement:** the `requirements-analyst` survives alongside the new PM. Boundary: **PM owns *which*** — features, extents, the D7 filter, selection advice; **analyst owns *how well*** — story sharpness, measurable criteria, FR traceability, authored inside the PM's frame. Neither edits the other's verdicts; a disagreement (e.g. the analyst holds a filter-rejected story is load-bearing) escalates to the user. The seats interleave at stages (PM at intent and post-story derivation; analyst at authoring), not as a serial queue.

**Rationale:** the two crafts fail differently — PM failure builds the wrong thing, analyst failure builds the thing wrong; one persona self-checking across both failure modes deadens the tension quality comes from. The composition doctrine already rules it: two judgment modes = two personas; folding would mint the roster's first dual-discipline seat. User leaned both-stay; lead's independent recommendation agreed.

### D16 — Review: extended spec review for judgment + KM-style map-integrity invariants — `Confident`

**Statement:** the PM's feature work is graded two ways. **Judgment:** `review-specifications` extends — the independent spec reviewer grades spec + stories + feature derivation + map delta in one pass (derivation honesty, filter rejections justified, dedup against the actual map file, granularity guide respected, proposed entries well-formed). **Structure:** the map joins the KM module's fix-on-sight invariants — dangling FEAT-IDs, status/index disagreement, orphaned D8 deltas, spec-index rows contradicting the map — policed by any touching command, same as the brainstorms index today. No dedicated map-validator seat.

**Rationale:** the reviewer who already reads the stories is the only one who can see derivation dishonesty (a feature no story supports); splitting that context across two graders halves both. Per-run judgment review can't catch rot between runs — that is exactly what KM invariants exist for, and the map is now operating-doc-grade infrastructure. A seat for what a checklist line covers is surface without value.

### D17 — One plan/implement run per feature, dependency-ordered — `Confident`

**Statement:** each selected feature gets its **own plan/implement pass**, strictly sequential per the map's dependency order — never one plan run spanning a multi-feature selection. Plan artifacts land per feature (working layout: `features/FEAT-XXX/` inside the spec folder, replacing `slices/<slice>/`; exact layout fixed at build).

**Rationale:** everything ruled today — per-feature status (D5), per-feature deltas (D8), per-feature landing bookkeeping (D13), seams owned by the later-landing feature — assumes the feature is the atomic pipeline unit. A per-selection batch run blurs back toward slice-batch behavior and makes the per-feature architecture fold ambiguous. Sequential passes are the honest cost of D4.

## Build surface (derivation, not rulings — cold-buildable sketch)

- **New primitives:** `product-manager` agent (D14, roster 9→10) · `authoring-feature-map` skill (derivation method, D6 entry shape, map-write rules, D8 delta grammar, intent-stage map-read agenda, D7 filter procedure).
- **Retiring:** `authoring-slices` skill — the invariants that transfer (exactly-one-home, dependency closure, foundation designation, SC-coverage map, seams) re-key from stories/slices to features inside `authoring-feature-map`; retirement is a recorded supersession per the primitive-edit ceremony.
- **Commands touched (all five):** `setup.md` — brownfield feature-map reconstruction (D12); `specify.md` — map-read at intent (D7), PM seat, derivation + filter stage, selection replaces Delivery Slices, spec-workspace restructure (D10); `plan.md` — per-feature scoping (D17), feature hardening alongside architecture (D5), `slices/<slice>/` layout → `features/FEAT-XXX/`; `implement.md` — landing absorbs map bookkeeping (D13); `brainstorm.md` — untouched (verify at build).
- **Templates:** `spec-template.md` restructure (stories out to `stories/US-*.md`, Delivery Slices section dies, feature-selection section enters, SCR/FLOW slice-tags re-key to FEAT-tags) · new `FEATURES.md` index shape + per-feature entry template (D6/D11) · `tasks-template.md` cycle-card slice references re-key.
- **Skills touched:** `review-specifications` — extension per D16 (derivation honesty, filter justification, dedup, map delta) · `patterns-vertical-tdd` — vocabulary re-key (graduation slice → feature) · `authoring-user-stories` / `authoring-requirements` — analyst-inside-PM-frame boundary lines (D15) · `authoring-prototype` — slice-tag grammar re-key.
- **KM module:** map-integrity invariants (D16) · top-level reservation amendment adding `FEATURES.md` to the core-when-content set (D11) · specs `index.md` maintenance contract (D10).
- **Roster/doc ripple:** router skill, `ARCHITECTURE.md` (plugin), `plugin.json` agents count, governance region regeneration.

## Open threads

1. **Oversized feature** (D4): a feature too big for one graduation pass even with vertical-TDD cycles inside implement — revisit trigger: first real run that hits it.
2. **Intent-stage mechanism** (D14): ~~open~~ **ruled at build kickoff (2026-08-10, user):** the `authoring-feature-map` skill carries the intent-stage map-read agenda and shapes the lead's intent conversation; the PM seat spawns after stories for derivation. No mid-intent relay.
3. **D6 entry shape** is `Assumed` — line-by-line hardening happens at template authoring; first dogfood re-opens it if the shape fights real use. (Amended post-review: obligations line per D21, `retired` status per R3.)
4. **Migration:** ~~open~~ **ruled at build kickoff (2026-08-10, user): freeze old, new-form new.** Existing slice-form specs stay valid frozen history; only new specify runs use the feature surface; D12's bootstrap derives the map from delivered code, never from old specs. Zero conversion.
5. **Selection UX:** how the selection moment is presented (map delta + PM recommendation + deferred-SC list per D21) — build-time design, user ruling stays reserved.
6. **Map scale** (R11): revisit trigger when the map crosses ~60 entries — index grouping layer, scoped map-reads.

## Review

Cold review run 2026-08-09/10 (devils-advocate via `review-brainstorm`, solo, session-model override after the known model-name API error killed the first spawn). Fact layer F1–F5 verified clean. **15 findings raised, 15 survived** (1 Critical, 9 Important, 5 Minor). Verdict: FAIL — needs revision; blocking set R1 · R4 · R6 · R8. Dispositions below; R9's disposition reserved to the user (challenges D4).

### Dispositions

**R1 (Critical — Graduation contract's other half) → resolved by D18** (user-ruled).

### D18 — Graduation contract re-keys to features verbatim, plus the cross-spec extend reach — `Confident`

**Statement:** the full Graduation contract survives, re-keyed from slices to features:

- **Per-feature artifacts** under `features/FEAT-XXX/` in the spec folder: `plan.md`, `architecture.md` (delta), `tasks.md` — the trio that was per-slice.
- **Shared artifacts accumulate at spec root:** `requirements.md`, `constraints-and-decisions.md`, `nfrs.md`, `data-model.md`, `contracts/`, `quickstart.md`. Feature 1's plan run creates them scoped to its needs; every later feature's run opens in **extend-mode** — read first, extend in place, never re-derive parallel copies.
- **Breaking amendment:** a later feature breaking an earlier delivered feature's design = explicit `[MODIFY]` amendment — named in the plan, migration stated, surfaced at architecture sign-off, never designed around silently. The amendment also writes the D8 delta on the affected feature's map entry — the two mechanisms fuse.
- **Regression safety:** each feature's implement runs the accumulated TEST gates of previously delivered features in its territory before acceptance.
- **Cross-spec extend reach (new, beyond verbatim):** features are durable across specs (D3), so extend-mode's read obligation reaches cross-spec — a plan run touching FEAT-X reads the artifacts of the spec that last shipped FEAT-X, found via the map entry's owning-spec provenance. The slice world never had this seam; the build carries it into `plan.md`'s entry obligations.

**Rationale:** the contract's machinery was never slice-specific — it governs sequential units sharing one design surface, exactly what D17's per-feature runs are. Dropping any part re-opens the drift it was built to stop.

**R4 (Important — multi-spec concurrency) → resolved by D19** (user-shaped).

### D19 — In-flight features are readable inputs, not locked hazards — `Confident`

**Statement:** when a run touches a feature that is `in-flight` (or delta-carrying) under another spec, the owning-spec pointer obligates a **read into the owning spec's artifacts** — its stories, plan, architecture delta for that feature — so the touching run knows what the feature is *becoming*, not just that it's busy. The fork then resolves with information, not policy:

- Need already covered by the in-flight planned extent → reference the relation, build against the planned contract, no entry write.
- Need adjacent → record a `proposed` delta marked "extends in-flight work, spec-N," sequenced behind that delivery by the ordinary dependency machinery (D17 ordering + D18 cross-spec read) — never by a lock.
- Need conflicting with the in-flight direction → a real product decision, escalated to the user (amend the owning spec, or override) — the same never-designed-around-silently rule plan already applies to signed-off architecture.

No lock exists. What is prohibited is only **silent contradiction**. D16 gains the matching invariant: every delta names its spec; a delta whose spec closed without folding is a defect.

**Rationale:** the lead's three dealt options (advisory warning · lock-with-override · partition) all treated in-flight work as a hazard; the user's reframe — spec-B should *utilize* spec-A's in-flight design as input — dissolves most of the concurrency problem, because a run that has read the in-flight design cannot blindly harden the same entry against different assumptions. Matches the repo's doctrine arc: lead judgment + user rulings over machinery locks.

**R6 (Important — write timing vs rejection) → resolved by D20** (user-ruled).

### D20 — Map writes land at spec acceptance; reads any time — `Confident`

**Statement:** during a specify run, proposed features and deltas live in the **spec workspace** as derivation output (graded there by the D16 extended spec review). The map write executes at **spec acceptance** — specify's existing landing moment — as one atomic bookkeeping batch: new entries land (`proposed`; selected ones to `in-flight`), deltas attach, index lines update. A rejected spec never touched the map; a dead run leaves only workspace drafts, the truth layer clean. **D5 amends by reading rule:** "reads and writes the map directly" = reads any time, writes at acceptance.

**Corollary:** D19's read-into-in-flight-work sees another spec's feature work only once that spec is accepted — correct by design: pre-acceptance derivation is unratified thought no other run should build on.

**Rationale:** the live source of truth must never carry entries from a spec that never existed. Same landing pattern the repo uses everywhere (brainstorm records land at acceptance, architecture folds at landing).

**R8 (Important — partial-delivery orphans) → resolved by D21** (user-ruled). Also retro-closes R2's extend-obligation half via the obligations line.

### D21 — SCs scope to features at derivation; deferred obligations ride the proposed entry — `Confident`

**Statement:**

- **SC re-homing:** at derivation every SC-XXX maps to the feature(s) whose delivery verifies it (the D4 SC-coverage map, now consequential). At selection the SC set splits visibly: SCs covered by selected features are this delivery's done-condition; SCs covered only by unselected features **travel with the `proposed` entry** — folded into its extent/acceptance expectations, verified when it builds. Spec close verifies only the selected half — nothing silently unmet, nothing falsely open.
- **Seams:** both sides selected → owned by the later-landing feature (D13, unchanged). One side unselected → recorded on the `proposed` entry as an obligation ("when built, verify seam against FEAT-XXX").
- **Entry shape amendment (D6):** entries gain an **obligations line** — deferred SCs, deferred seams, and cross-cutting extend obligations (closing R2's gap with the same field).
- **Selection-card honesty:** the deferred-SC list is visible on the selection card — choosing a subset is choosing which success criteria wait, shown at the moment of choice, not discovered at close.

**Rationale:** the map entry becomes the single carrier of everything deferred — status, SCs, seams, obligations — the consistent extension of D9's "map durable, specs are delivery events."

**R9 (Important — challenged D4; disposition reserved to and ruled by the user) → resolved by D22.**

### D22 — Foundation softens to an ordering role on features — `Confident` (D4 amendment, user-ruled)

**Statement:** the foundation-legitimacy invariant re-keys as an **ordering role**: foundation = the selection's first feature per dependency order. "Establishes the shared core + delivers a testable journey" demotes from hard invariant to **guidance** for which feature goes first; plumbing lands in foundation *cycles* inside that feature's implement (the existing downstream mechanism). When the true shared core spans features, the first feature carries it only as far as its extent honestly reaches — minting a pipeline-convenience pseudo-feature stays forbidden (D2 holds).

**Rationale (reviewer's R9):** slices were composable, so the invariant was always satisfiable by regrouping; features are fixed capabilities — the map may contain no feature that is both shared-core and independent journey, making the verbatim invariant unsatisfiable.

### Remaining dispositions (user-confirmed batch)

- **R2** (cross-cutting stories) — **closed by D21's obligations line**: story homes to one FEAT; touched FEATs carry extend obligations on their entries.
- **R3** (no retirement path) — **accepted, D6 amendment:** terminal status **`retired`** joins the lifecycle (entry kept, dated, provenance intact; capability removed from product). Deletion never — dangling-reference protection stays whole.
- **R5** (abandoned in-flight rot) — **accepted, D16 amendment:** the KM-precedented **in-flight-agreement invariant** joins the integrity set — every `in-flight` status/delta points at an open spec; a closed-or-abandoned spec still pointed at is a defect, fix-on-sight.
- **R7** (KM-elective vs pipeline-core) — **accepted, D16 amendment:** the map is **pipeline-core**; integrity invariants are carried by the touching commands + `authoring-feature-map`, not the KM module. KM adds only index-agreement lines for KM-adopting repos; the FEATURES.md top-level reservation rides the pipeline's own doc set.
- **R10** (filter vs lockstep prototype) — **accepted, build lines in specify + `authoring-prototype`:** a filter-rejected story's screens are kept, greyed, marked rejected (coming-soon grammar); FEAT-tags land at derivation as a re-tag pass over the SCR/FLOW manifest.
- **R11** (scale) — open thread: revisit trigger when the map crosses ~60 entries (index grouping layer, scoped map-reads).
- **R12** (self-application) — build surface gains the dogfood repo's own `/mochiko:setup` governance-amend step for the KM/reservation changes.
- **R13** (review delta baseline) — build line: the reviewer's map-delta baseline is the git state at run open.
- **R14** (reconstructed-extent re-verify carrier) — build line: obligation carried in `authoring-feature-map` + specify's touch list.
- **R15** (two-backlog boundary) — boundary rule: product capabilities live on the map as `proposed` entries; everything else (defects, tooling, process) stays in `BACKLOG.md`. KM BACKLOG contract line at build.

**Review close:** 15/15 findings dispositioned — 4 blockers by D18–D21, R9 by D22 (user-ruled), R2 folded into D21, the rest accepted as amendments/build lines per the confirmed batch. Reviewer's verdict FAIL → record revised as above.
