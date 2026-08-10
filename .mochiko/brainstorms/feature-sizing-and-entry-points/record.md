# Feature Sizing & Entry Points — Decision Record

**Status:** accepted (2026-08-10)
**When:** 2026-08-10
**Session:** `/mochiko:brainstorm` — two threads on top of the feature-map layer: (1) feature size and stacking depth; (2) dedicated entry points for bugs and feature improvements beyond `/mochiko:specify`.

## Driver (user's framing, verbatim intent)

The feature-map ability exists (per the `feature-map-layer` session, built v0.57.0–v0.59.0). The user wants to go deeper into the **size** of a feature and **how features are stacked**. Also: explore giving bugs and feature improvements their **own command/entry point** — not solely `/mochiko:specify` as the pipeline entrance.

## Ground facts (read at session open)

- **F1** — Feature sizing today is one guidance line (feature-map D6): "a capability a product person would name in one breath — bigger than a story, smaller than a product area; extent not statable in ~3 lines means it's two features." No effort/size axis exists on entries.
- **F2** — Oversized-feature risk is a recorded open thread (feature-map D4 named risk / open thread 1): a feature too big for one graduation pass; working answer is vertical-TDD cycles inside implement; revisit trigger is the first real run that hits it.
- **F3** — Stacking today: entries carry **relations** (depends-on / extends / composes-with, feature-map D6) and dependency-ordered delivery (feature-map D17 one plan/implement run per feature, map's dependency order); foundation softened to an ordering role (feature-map D22).
- **F4** — Entry points today: `/mochiko:specify` is the sole pipeline entrance for product work. Its Goal assumes a full feature description → intent stage → stories → derivation → selection. No bug or improvement path exists in any command.
- **F5** — R15 boundary (feature-map): product capabilities live on the map as `proposed` entries; defects/tooling/process stay in `BACKLOG.md`. *(Narrowed at review, finding 12-fact: KM-adopting target repos DO have BACKLOG.md as their defect queue; only a non-KM target product lacks a ruled defect home.)*

## Decisions

### D1 — Sizing pain: too coarse AND too flat, interlinked — `Confident`

**Statement:** the sizing problem is both (a) derived features come out too big for one graduation pass, and (b) the map is flat, with no way to express a big capability composed of smaller deliverable features. The user rules these interlinked: features derive too big *because* the map cannot express "big capability = stack of smaller features" — the deriver rounds up to the big capability name when no compositional structure exists to land the smaller units in.

**Rationale:** the one-breath guidance (F1) polices the entry's prose, not its delivery weight; with only flat peer relations available (F3), a naturally hierarchical capability has nowhere to decompose, so it stays one oversized entry — which is exactly the feature-map D4 open-thread risk (F2) arriving by construction rather than by accident.

### D2 — Hierarchy on the map: nested feature entries, leaf = pipeline unit — `Confident`

**Statement:** feature entries can nest — a parent feature (the capability a product person names) with child features (deliverable units). The **pipeline unit is the leaf**: plan/implement runs key to leaf features only; a parent is navigation + status roll-up and is never built directly. Parent status derives from children (`delivered` when all children delivered; `in-flight` when any child is). The sizing bar applies at the leaf. Map-integrity invariants (feature-map D16) extend to parent/child agreement: a parent whose status contradicts its children's roll-up is a defect, fix-on-sight.

**Amended at review (finding 3):** the roll-up yields to feature-map D8's sticky-delivered — a `delivered` parent gaining a new in-flight child (retroactive promotion, new sibling) **keeps `delivered`** and carries the child as a marked delta on the parent entry, folding when the child delivers. The roll-up invariant reads: delivered = all children delivered at the time delivered was earned; later children ride as deltas, never regress the parent.

**Rationale:** the pain is exactly "the big thing IS made of small things" — peer relations express that weakly (no visible roll-up), and a separate area/epic vocabulary adds a second entry class whose status would be decorative and stale. Nesting keeps one entry type; the one-breath guidance survives at the parent (it names the capability) while the ~3-line extent bar polices the leaf (it bounds the deliverable).

### D3 — Two levels, hard cap — `Confident`

**Statement:** the tree is capped at two levels: parent + leaf. A capability that outgrows two levels splits the parent into two parents. No third level, no escape hatch.

**Rationale:** the one-breath rule at the parent and the ~3-line extent bar at the leaf already bound the spread; deep trees are where maps die (every index in this repo stays flat-ish and readable). A product that genuinely outgrows two levels hits the map-scale revisit trigger (feature-map open thread 6, ~60 entries) anyway — that session owns the answer if it ever fires.

### D4 — Parents minted both directions, plus retroactive promotion — `Confident`

**Statement:** a parent can be minted three ways: (a) at derivation as a remedy, when a leaf fails the size bar (extent won't fit ~3 lines, or won't honestly fit one graduation pass); (b) at derivation capability-first, when the deriver sees the capability naturally decomposing — single-leaf parents allowed (parent = leaf collapsed until a second child arrives); (c) **retroactively**: a later spec can promote an existing flat feature into a parent when new related work arrives — the original delivered extent becomes the first child, new work lands as sibling children. Promotion on a delivered entry follows the feature-map D8 marked-delta discipline (status never regresses; the delivered extent becomes a delivered child).

**Rationale:** split-on-oversize alone loses the D1 insight — the rounding-up happens *because* structure is missing at derivation; capability-first alone forces ceremony on genuinely small features. Both-directions plus promotion keeps it deriver judgment; without the promote path, the map's earliest flat entries fossilize and new related leaves land as fake peers.

### D5 — Reframe: a feature-management command, not bug/improvement commands — `Confident` (user-initiated)

**Statement:** the entry-point question resolves by reframe. Posed the bug/improvement fork (new lightweight command vs specify-intake fork vs two dedicated commands — dealt at a flagged adoption streak, steelmans, no recommendation), the user redirected: what's wanted is a command **linked directly with managing features** — features are first-class citizens of the flow. Bugs and improvements become operations *on features* through that command, not their own entrances.

**Rationale (user):** the map is the product's capability source of truth; the flow should front it directly rather than routing every touch through a spec-shaped entrance or minting per-work-type commands.

### D6 — Remit: map steward + lightweight delivery lane — `Confident`

**Statement:** the feature command (working name `/mochiko:feature`) owns (a) **map stewardship** — view/query the map, add `proposed` entries, promote flat entries to parents (D4c), retire, groom integrity fix-on-sight; and (b) a **lightweight delivery lane** — small feature-keyed work (a bug on a delivered feature, an extent-grow improvement) enters through the command: marked delta on the entry, delivery per D8-as-amended (dispatch to the re-keyed pipeline, never an inline harness). `/mochiko:specify` remains the entrance for **new-capability** work only. The full-lifecycle option (specify demoted to an internal stage the feature command dispatches) was declined.

**One-command shape confirmed at review (finding 8, user-ruled):** the surface-area challenge was weighed; the command stays one command — but post-D8-amendment it shrinks to steward + triage + delta-card authoring + dispatch: the discipline floor lives only in plan/implement and the craft skills, **bound by reference, never restated**. The split-steward-from-delivery option was declined.

**Rationale:** stewardship alone still leaves bugs and improvements homeless — the driver pain. Full lifecycle ownership inverts the pipeline around one command and demotes specify's intent/story/derivation discipline to a callee. The middle rung gives the map a front door and small work a proportionate path while the heavy entrance keeps its own command.

### D7 — Lane boundary: the map-write test, with abort-and-reroute — `Confident`

**Statement:** the inline delivery lane is allowed only when the work needs **no new map entry and no status change** — a pure marked delta on an existing feature (extent grows, defect fixed within extent). Anything that would mint an entry, promote to parent, or flip status beyond a delta-fold routes to `/mochiko:specify`. Mid-run discovery that the work outgrew the lane (the fix turns out to need a new entry) **aborts and re-routes** — the lane never widens in place. The boundary is mechanical and file-checkable, audited from the map delta, never from a self-declared "small". **Grading seat (review finding 5, closing open thread 2):** the lane run's own verification seat — already independent of the producer under implement's harness (the D8 dispatch) — gains the map-delta boundary check in scope, per the D15 precedent of extending the landing verifier. No new seat.

**Invariant amendment (review finding 4):** feature-map R5's in-flight-agreement invariant re-words — every `in-flight` status/delta points at **an open spec or a live lane run** (a lane run is live from dispatch until its acceptance landing; a delta whose lane run ended without folding is a defect, fix-on-sight). Recorded supersession touch at build.

**Rationale:** a story-based test leaks (honest bugs and sneaky scope-creeps both leave stories untouched); pure judgment reintroduces the self-declared-small erosion the lane's critics predicted. The map-write test is the only boundary the existing integrity machinery can police without new surface.

### D8 — Lane rigor: one delta card, executed by dispatch to the re-keyed pipeline — `Confident` (as amended at review, finding 1, user-ruled)

**Statement (as amended):** lane work is captured as **one delta card** — a single cycle-card-shaped unit authored inside the feature command: a bug's acceptance is its reproduction-failing-test; an improvement carries 1–3 acceptance criteria on the delta. **Execution is by dispatch:** the lane hands the delta card to the D9-re-keyed `plan`/`implement` entry as feature-command-delta scope — the feature command runs **no inline harness** and restates no discipline floor. For delta scope the pipeline scales itself: plan collapses to confirming the card against the entry (no package authoring where no design surface changes); implement executes the card under its own bounds, verification seats, and evidence rules. The landing branches by scope type: delta scope = the feature-map D8 delta fold; selection scope = the graduation batch.

**Assessment findings that motivated the original inline reading (kept for the trail):** at assessment time plan blocked without an accepted `spec.md` and implement without the spec-folder package, and implement's landing was graduation-only — an inline harness looked like the only path. D9's entry re-key dissolved that premise minutes later.

**Superseded at review (finding 1, Critical, user-ruled option A):** the original "restated in the feature command's own harness / commands untouched" architecture is dead — it double-specified lane execution against D9's entry re-key and created a drift-prone second home for implement's bounds (finding 8). One discipline home; the feature command references, never restates.

### D9 — Plan/implement re-key to the feature; design surface goes two-altitude (product baseline + feature delta) — `Confident` (user-initiated challenge, user-shaped)

**Statement:** the user challenged spec-linkage directly: plan and implement should link to the **feature** — the map is the central source of truth. Ruled:

- **Entry re-key:** plan/implement gate on a feature with ratified scope on its entry — the scope source may be a spec's Feature Selection or a feature-command delta; the spec.md gate and the package-under-spec-folder gate are replaced. The Q8 lane problem dissolves: one uniform entry condition, no spec-less fork.
- **Artifact re-home:** per-feature artifacts live at `.mochiko/features/FEAT-XXX/` (working layout; fixed at build) — plan, architecture delta, data-model delta, contract delta, tasks, reports, **plus `requirements.md` (FR→TR is per-feature analysis; finding 2)**. D18's cross-spec reach dies; a run touching FEAT-X reads FEAT-X's own directory.
- **Orphan altitudes (review finding 2, batch-ruled):** `constraints-and-decisions.md` and `quickstart.md` join the **product-baseline set** — constraints/decisions accumulate product-wide (C-XXX/D-XXX/IP-XXX are product truths), quickstart describes the product's real external-integration surface; both fold per D15 like every baseline. `requirements.md` goes per-feature as above. No shared artifact is left homeless.
- **Two-altitude design surface (user ruling: "works at product level and feature level"):** the shared design artifacts (`data-model.md`, `contracts/`, `nfrs.md`) become **product-level accumulated baselines** beside `ARCHITECTURE.md` — describing what the product HAS; each feature's directory carries its **delta** against those baselines — what the feature CHANGES; feature acceptance **folds deltas into baselines** in the same landing that flips map status. The exact convention `ARCHITECTURE.md` + per-feature `architecture.md` already use, extended to every shared design surface.
- **Spec becomes a pure delivery-event record** (D9 of feature-map made fully honest): spec.md + stories; design artifacts no longer accumulate at spec root.

**Rationale:** spec-linkage was inherited, not chosen — feature-map D18 re-keyed the Graduation contract verbatim from slices, whose whole life was inside one spec; feature-map D3 had already made features durable and cross-spec, and cross-spec reach was the patch covering the mismatch. Re-homing removes the patch instead of extending it.

**Supersession note (extended at review, finding 11):** this amends feature-map D17/D18 (artifact layout `features/FEAT-XXX/` inside the spec folder; extend-mode-at-spec-root; cross-spec reach), **feature-map D10's plan-artifacts-in-spec-folder clause** (dies with the re-home), and **feature-map D19's read mechanics** (the obligated read into in-flight work now resolves the design half to `.mochiko/features/FEAT-XXX/` dirs; stories stay with the owning spec) — all recorded supersessions at build per the primitive-edit ceremony.

### D10 — No migration path: breaking change `Confident`; bootstrap mechanism `Assumed`

**Statement:** the re-homing ships as a breaking change. No freeze-old layer, no compatibility reads of the v0.57–0.59 layout (`features/FEAT-XXX/` inside spec folders). The new version assumes the new layout only.

**Bootstrap clause — `Assumed` (split at review, finding 14):** product baselines bootstrap from delivered code via setup's brownfield analysis (feature-map D12 precedent) or are seeded by the first plan run. Lead-derived, never user-ratified; its risk is open thread 4's (first-plan-seeding produces a partial baseline claiming to describe what the product HAS). Hardens with open thread 4 at build.

**Rationale (user, covering the breaking-change half):** "assume the new version of mochiko will have breaking change and no need to be backward compatible."

### D11 — Concurrent deltas: D19 machinery suffices at the fold — `Assumed`

**Statement:** two in-flight features carrying deltas against the same product baseline are governed by the existing machinery: feature-map D19 (in-flight work is a readable input, no locks; silent contradiction prohibited) plus feature-map D17 dependency ordering — the later-landing feature folds second and reconciles at the fold, escalating a real conflict to the user. No new fold-time machinery.

**Mark rationale:** recommended alongside the migration question; the user ruled only on migration. Carried as the working answer unless challenged.

### D12 — Feature command mints placeholder stubs only; selectability stays behind derivation — `Confident`

**Statement:** the feature command can add `proposed` entries only as **capability stubs** — name + one-breath hook, marked `unrefined`. Only specify's derivation fills extent/relations and makes an entry selectable for delivery. A stub is parking, never a spec-bypass; the `unrefined` mark makes the gap auditable (feature-map D16 review + map-integrity invariants).

**Rider (review finding 10):** the intent-stage map read treats `unrefined` stubs as **unratified hypotheses, never extension anchors** — specify's derivation ignores stub text and derives from stories per feature-map D7; a stub matching a derived feature is confirmation, a stub matching nothing is left parked or retired. Stories-first survives the stubs.

**Rationale:** two proposal paths now exist (specify's derivation, the feature command's stewardship); an unbounded second path makes the feature command a cheap side door for minting features without stories/intent discipline — D7's boundary leaking upstream. Parking an idea is legitimate stewardship; delivering it still earns derivation.

### D13 — Cross-cutting bugs: product-level defect lane; BACKLOG stays the queue — `Confident`

**Statement:** a defect with no single owning feature (infra, glue, cross-feature, performance) runs in a **product-level defect lane** inside the feature command: the same delta card and rigor, keyed to the product baselines / `ARCHITECTURE.md` instead of a feature entry, verification scoped by the affected surface. The map is untouched for these — no fake owners, no pseudo-features (feature-map D22's ban holds).

**BACKLOG/KM relation (user-asked; scoped at review, finding 12):** R15's boundary extends unchanged **for KM-adopting repos** — `BACKLOG.md` is the defect **queue** (a reported bug is a BACKLOG item until a lane run picks it up; the lane landing closes it per the KM landing ritual). A non-KM target product has **no queue**: lane runs accept direct requests, nothing queues between runs — stated as the degrade path, not silently assumed away. Deferred improvements split by nature: extent-growth ideas ride the map (`proposed` deltas / obligation lines — the map is the capability backlog per feature-map D9); non-capability improvements (refactor, tooling) are BACKLOG items where KM exists. The KM module gains one line: **lane acceptance is a landing event** — same ritual home as spec/implement acceptance. Map-integrity invariants stay pipeline-core (feature-map R7).

**Rationale:** nearest-feature homing pins fake owners on honest infra bugs and pollutes entries with foreign obligations; force-feature mints the pipeline-convenience pseudo-features feature-map D2/D22 forbid. Some code serves all features — its defects key to the product surface, which after D9 exists as a first-class artifact set.

### D14 — One stable-ground rule: the lane writes only surfaces no live run owns — `Confident` (user-composed coherence)

**Statement:** lane eligibility is one rule with two instances — **the lane writes only surfaces no live run owns**. Triage keys the bug to its surface, then applies the stable-ground test:

1. **Single owning feature, `delivered`** → feature lane (D7 delta card on the entry).
2. **Single owning feature, `in-flight`** → not lane work: the finding **files to the owning run** — its implement is live, its verification and regression gates own that territory.
3. **No single owner** → product lane (D13), keyed to baselines/`ARCHITECTURE.md`, under the same test at the product surface: an affected baseline surface under active delta by an in-flight run files to that run instead; the product lane proceeds only on baseline ground no live run is building on.

**Precision restated at review (finding 7):** the check **inputs** are files — entry status at the feature level, the in-flight feature dirs' enumerated baseline deltas at the product level (the D9 two-altitude structure makes them enumerable). The **keying** of a raw bug report to its surface is triage judgment, audited from the resulting delta, never claimed mechanical. The product lane gains D7's **abort-and-reroute**: mid-fix discovery that the run stands on an in-flight feature's territory files the finding to that run and aborts. **Lane-vs-lane (finding 15):** the product lane is **single-flight** — one live product-lane run at a time; concurrent product-lane runs are the two-writers-no-sequencer case verbatim.

**Delta-visibility obligation (review finding 6, batch-ruled — touches D6/D13/D15):** any lane run that touches a product baseline **authors a minimal enumerated delta document** (appliable form) alongside its delta card — making lane baseline writes visible to D14's stable-ground check and giving D15's fold audit its middle input. Authoring actor: at intake by the feature command when the touch is known up front; by the dispatched run at the moment a touch is discovered mid-fix. A lane run that writes no baseline carries no delta doc.

**Rationale:** an in-flight feature has a live implement run with its own verification, bounds, and landing — a parallel lane writer on the same surface is two writers with no sequencer (spec-vs-spec has feature-map D17's dependency order; lane-vs-implement has nothing). The user surfaced the D13/D14 seam — bugs ruled product-level, collision ruled feature-level — and ratified the generalization that dissolves it: stable ground is the invariant, the surface (entry vs baseline) is the instance.

### D15 — Baseline folds are graded, uniformly — `Confident`

**Statement:** every landing's fold into a product baseline (`data-model.md`, `contracts/`, `nfrs.md`, `constraints-and-decisions.md`, `quickstart.md`, `ARCHITECTURE.md`) is independently checked before acceptance completes: a three-way diff audit — pre-fold baseline + the feature's delta vs the folded result — confirming the delta applied whole, nothing else changed, no silent rewrites of unrelated sections. Uniform across surfaces; the grader is the landing's existing verification seat, scope-extended — no new seat.

**Delta-form requirement (review finding 9):** deltas against prose baselines (`nfrs.md`, `ARCHITECTURE.md` narrative, constraints prose) are written in **appliable form — exact before/after text** — keeping the three-way diff mechanical; structured surfaces (`contracts/api.yaml`, data-model tables) are appliable by nature. A delta not statable as before/after text is not yet a foldable delta.

**Rationale:** D9 made the fold the highest-blast-radius write in the pipeline — a corrupted baseline poisons every later extend-mode run silently. The check is cheap (bounded three-way diff), the failure expensive and late-discovered; a next-run guard finds corruption only after it propagated into a build.

## Build surface (derivation, not rulings — cold-buildable sketch; revised at review fold)

- **New command:** `/mochiko:feature` — map stewardship (view/query, stub minting per D12 with the finding-10 rider, promotion per D4c, retire, integrity grooming) + lane intake: triage per D14 (stable-ground test, both instances, product lane single-flight), delta-card authoring (+ the minimal enumerated delta doc when a baseline is touched, finding 6), then **dispatch to the re-keyed plan/implement** — discipline floor bound by reference, never restated (D8-as-amended, finding 8); lane acceptance is a KM landing event where KM exists.
- **Commands re-keyed (D9):** `plan.md` / `implement.md` entry gates re-key from spec to feature (ratified scope on entry: spec selection or feature-command delta; plan scales itself down for delta scope, landing branches by scope type per D8); artifact home moves to `.mochiko/features/FEAT-XXX/` incl. per-feature `requirements.md`; cross-spec reach and extend-mode-at-spec-root die; `specify.md` — spec becomes pure delivery-event record (spec.md + stories); `setup.md` — brownfield bootstrap extends to product baselines, scope per open thread 4.
- **Two-altitude design surface (D9):** product baselines (`data-model.md`, `contracts/`, `nfrs.md`, `constraints-and-decisions.md`, `quickstart.md`) beside `ARCHITECTURE.md`; per-feature deltas in feature dirs, prose deltas in appliable before/after form (D15); graded folds at landing (D15) — fold-check joins the landing verification seat's scope, as does the lane map-delta boundary check (D7).
- **Map machinery (D2–D4):** `authoring-feature-map` gains nesting (parent/leaf, two-level cap, roll-up with the sticky-delivered amendment), minting-both-directions + retroactive promotion with feature-map D8 delta discipline, parent/child integrity invariants, the R5 invariant re-wording (open spec OR live lane run); entry template + index template gain the parent/leaf shape and `unrefined` mark.
- **Supersessions owed at build (per the primitive-edit ceremony):** feature-map D17/D18 (artifact layout, extend-mode home, cross-spec reach) · feature-map D10's plan-artifacts-in-spec-folder clause · feature-map D19's read mechanics · feature-map R5's invariant wording · this record's D8 original inline-harness architecture.
- **Breaking change (D10):** no compatibility layer; version bump communicates it.

## Review

Cold review run 2026-08-10 (devils-advocate, solo, cold from the frozen file; respawned with the Fable 5 model override after the known model-name API error killed the first spawn — third occurrence of that error across sessions). Fact layer F1–F4 verified clean; F5 narrowed (finding-12 fact half). **15 findings raised, 15 survived** (2 Critical, 7 Important, 6 Minor). Verdict: FAIL — needs revision; blocking set 1, 2, 3, 4, 6.

**Dispositions (15/15):** finding 1 user-ruled option A → D8 amended (dispatch architecture, inline harness dead) · finding 8 user-ruled → D6 amended (one command, floor by reference) · findings 2–7, 9–15 user-confirmed as the recommended batch → folded as amendments into D2 (roll-up vs sticky-delivered), D7 (grading seat + R5 invariant), D9 (orphan altitudes + supersession list), D10 (mark split), D12 (stub rider), D13 (KM scoping), D14 (precision + product-lane reroute + single-flight + delta-visibility), D15 (delta-form requirement), F5 (narrowed), plus the finding-13 editorial prefix pass. Record revised in place; superseded statements amended at their source, not only noted here.

**Verify pass on the fold:** round 1 NOT CLEAN — 4 non-blocking fold-hygiene defects (stale open-thread pointer in D10 after renumbering · 4 unprefixed cross-record refs in F3/D9 · wrong finding citation on F5 · delta-doc authoring actor unnamed in D14), all repaired same round; bounded re-verify round 2 **CLEAN** — no new contradictions, one dispatch architecture confirmed across D6/D8/D9.

## Open threads

1. **Parent selection semantics** — user selects a parent = selects all undelivered children, or leaves only? Build-time design; selection UX ruling stays reserved to the user (feature-map open thread 5 adjacent).
2. ~~Lane delta-card grading detail~~ **Closed at review (finding 5):** the lane run's verification seat, scope-extended, grades the map-delta boundary — see D7.
3. **`unrefined` stub lifecycle** — staleness/expiry of parked stubs (D12); no ruled trigger.
4. **Setup reconstruction burden + bootstrap risk** — D10 baselines bootstrap via setup now means reconstructing data-model/contracts/NFRs/constraints/quickstart from code, not just the feature list; scope of that ask unruled; half-reconstructed baselines poison extend-mode, and first-plan-seeding (D10's other bootstrap arm) produces exactly a partial baseline. (D10's bootstrap clause is `Assumed` pending this.)
5. **Size/effort signal at selection** — the selection card carries no cost signal (Q1-C dropped at D1); revisit if selection decisions misfire in dogfood.
6. **D11 is `Assumed`** — concurrent-delta fold reconciliation rests on feature-map D19 machinery unchallenged; first real concurrent fold re-opens it if reconciliation proves under-specified.
