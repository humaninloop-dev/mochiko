# Command `.md` scaffold standardization — decision record

**Status:** accepted (2026-08-27) — solo cold review (verdict critical-gaps), 19/19 survivors dispositioned and folded, verify round 1 verified-not-blocking with 6 nits lead-repaired same round; user accepted
**Opened:** 2026-08-27 · **Lead:** session lead (inline questioning via `mochiko:analysis-iterative`)

**Topic:** after the command-content-schema rollout (six `.md` + schema pairs, v0.96.0), the schema side follows one pattern — nested section nodes, dotted-slug IDs, labels, `vars:`, a `fail-conditions` section, a deterministic checker. The `.md` scaffold around those schemas does not: the six files vary in section set, order, and idiom. The user wants the `.md` scaffold governed by a pattern of similar strength.

## Ground facts

**F1 — The library is six pairs and the schema side is already standardized.** `plugins/mochiko/commands/{architecture,brainstorm,feature,implement,setup,specify}.md` each pair with `plugins/mochiko/schemas/<cmd>.yaml` (command-content-schema D1–D16, v0.92.0–v0.96.0): one rule grammar, nested `sections:` nodes, mint-once IDs, the ten-label registry, `vars:` injection, an advisory checker.

**F2 — Part of the `.md` scaffold is already uniform.** All six share: frontmatter (`description` + `disable-model-invocation: true`) · a `# <Name> — <epithet>` title · a `## Rules — load the schema first` block whose text is near-identical boilerplate varying only in the per-command anchor phrase and the enumerated section IDs · a count-pinned `**Not done — default FAIL:**` line keyed to the schema's `fail-condition` label set.

**F3 — The biggest variance is a *ruled* two-form split.** Three commands are charter-form — `## Identity & Mission` + `## Adaptive Goal Protocol` (architecture, feature, implement) — and three are goal-form — a bold `**Goal:**` opener line + a `## Goal` done-condition section (brainstorm, setup, specify). The split is protected content. Carriers *(as completed at review, C1/M3)*: pm-role-and-feature-derivation D10 (v0.68.0) · ADR 2026-08-13-charter-plan-implement · charter-ritual-balance D3 · command-content-schema D9 (the pair-grading block) · **ADR 2026-08-02-doctrine-purge-wave-1** (see F7) · the dual criteria blocks in `.claude/rules/mochiko/primitive-edits.md`. Collapsing the split is a supersession-by-ruling, not a tidy-up.

**F4 — On top of the ruled split there is unruled drift.** Observed at v0.96.0: `specify.md` alone puts `## Goal` before `## Rules` · Not-done placement varies (specify mid-file) · `$ARGUMENTS` home varies (bold opener vs protocol body) · `setup.md` carries a hybrid `**You are the lead.**` identity line neither form defines · `implement.md` uses `###`-numbered protocol subsections where the desks use a bold numbered list · the two desks duplicate near-identical Adaptive Goal Protocol prose · grammar defect "the 1 rules labeled" (`architecture.md:55`, `feature.md:59`).

**F5 — The stage-2 door interacts.** command-content-schema D2 rules stage 1 as rules-move/narrative-stays, with a benefit-keyed absorption trigger to stage 2 ("narrative absorbs into the schema, `.md` thins to scaffold") and a user-reserved retreat branch. The framing — is this session a stage-1.5 tidy of what stays prose, or the stage-2 on-ramp — matters to what gets ruled here. *Ruled at review: D7 (stage-1.5 tidy).*

**F6 — The two-form split lives on both surfaces of every pair.** The schema section sets mirror the `.md` forms: goal-form commands nest `<cmd>.sec.harness` · `<cmd>.sec.bindings` · `<cmd>.sec.fail-conditions`; charter-form commands nest `roles` · `tools` · `ways-of-working` · `boundaries` · `fail-conditions`.

**F6a — Correction and sharpening of F6, read from the schemas.** (i) Provenance anchors are keyed by **rule ID**, not section (command-content-schema D16), so section-set unification does not re-key anchors; the real cost is section-node tombstones (D11/D14), rule relocation, and re-keying the checker plus each `.md` enumeration. (ii) There are **three** section vocabularies, not two: goal-form three · desk five · implement six (`impl.sec.reserved` first-class; the desks carry reserved rulings inside `roles`).

**F7 — A live contrary ruling exists and is dealt with in D1** *(review fold, C1)*. ADR 2026-08-02-doctrine-purge-wave-1 (DECISIONS.md row, status ruled) deleted `templates/command-shape.md` (v0.46.0) and re-keyed the command audit to "the command's own text" (its decision 4); its Alternatives-considered pre-rejects a shared shape artifact: *"Mint a thin replacement checklist for audits — rejected: recreates a shared artifact, against the purge intent."* Its rationale — per-command evolution without a shape-version ceremony rippling across six files — argues directly against this session's direction. Honest reopener, on record: the schema rollout itself already partly reversed purge intent (shared label registry, shared rule grammar, shared checker, shared audit branch).

**F8 — First-live-run evidence is n=0** *(review fold, C3)*. The command-content-schema first-live-run watch (BACKLOG) is open and unfired — no `/mochiko:*` has executed under the pair form. Its probes measure exactly what this session's rewrite perturbs ("schema read? read fully? before first action?").

## Decisions

### D1 — One canonical `.md` scaffold for all six commands; the form split is superseded — layout and vocabulary, not contracts — `Contested` *(amended at review: C1, C2, I7)*

**Statement:** this session designs **one** canonical `.md` scaffold that all six command files follow. The ruled charter/goal-form split is superseded — **scope narrowed (C2): the supersession covers scaffold layout (section set, order, idiom) and the section vocabulary (D4), not the done-condition contracts.** The per-visit (desk) vs fixed (run) done-condition distinction survives inside the single scaffold and the single audit criteria block (D6-R2 as amended). The build lands supersession-by-ruling entries against the F3 carriers — the owed clause inventory may legitimately return "no clause superseded" for a carrier whose clauses are contract-only (N4: the C2 narrowing leaves contract clauses standing) — **explicitly including purge-ADR decision 4 (C1)**: the audit bar "the command's own text" becomes "the command's own pair, graded against the canonical scaffold." Re-argument against the purge's rejected-alternative, on record: the purge rejected a shared shape artifact to keep per-command evolution cheap; the schema rollout has since made the shared-artifact move deliberately and by ruling (registry, grammar, checker), and the observed result of leaving the `.md` ungoverned is the F4 drift — the purge's benefit (cheap per-command evolution) demonstrably produced the variance this session exists to close. The session ratifies the partial reversal already made and extends it to the `.md`.

**Rationale:** the lead recommended keeping the two ruled forms and standardizing only the unruled drift — lighter, no supersession; steelman *(I7)*: drift-only fixes every F4 item, costs one hygiene wave, supersedes nothing, and preserves the purge's per-command-evolution benefit; its cost is that the form split itself — the largest variance — survives, and every future command must choose a form. The user ruled for full collapse: the variance complaint targets the split itself. `Contested` — deliberate choice against the recommendation.

### D2 — The canonical scaffold is the superset: every command carries every section — `Confident` (structure) / `Assumed` (delivery legs) *(amended at review: C3, I6, I9, M2)*

**Statement:** the single scaffold, in order:

1. `# <Name> — <epithet>` title. **Frontmatter (I6):** `description` + `disable-model-invocation: true` unchanged — `description` is discovery surface, not scaffold, so its register stays out of scope by reason — **plus `argument-hint` becomes canonical on all six**, the native frontmatter surface of the `$ARGUMENTS` contract fixed in step 4.
2. `## Identity & Mission` — who leads this surface and what it stewards; brainstorm, setup, and specify author theirs new; setup's stray `**You are the lead.**` line absorbs here. **Capped short (I9): identity prose is a single tight section, never delaying the Rules block materially.**
3. `## Rules — load the schema first` — the uniform boilerplate, varying only in the per-command anchor phrase and the section enumeration, byte-identical in shape per D5 (prefixes still differ per command).
4. `## Adaptive Goal Protocol` *(M2: "Adaptive" kept — graded-against language; it names the protocol family — Entry → Goal → Not-done with per-command convergence semantics — not a claim that every command negotiates its done condition)* — three fixed steps: **Entry** (`$ARGUMENTS` handling and gating) · **Goal** (the done condition — fixed for runs, per-visit converged for the desks; both contracts survive per D1-as-narrowed) · **Not done — default FAIL** (the count-pinned line, always last).

Nothing ruled dies except the split itself: the charters keep their Identity & Mission and protocol content; the goal-form commands gain the two sections and lose the bold `**Goal:**` opener — its content moves into protocol steps 1–2. The `$ARGUMENTS` home is protocol step 1 everywhere; the Not-done home is protocol step 3 everywhere.

**Confidence (C3):** structure `Confident`; the **delivery legs are `Assumed`** — the claim that the Read obligation at position 3, behind newly-authored identity prose, still executes before first action has never run (n=0, F8). The first-live-run watch is extended to measure exactly this (build item 8); the order leg (Identity → Rules → Protocol) also remains `Assumed` from the accepted preview, revisited and kept at review (I9) with the cap as mitigation.

**Rationale:** lead recommended; user accepted. Superset preserves every ruled responsibility while collapsing the split; subset shapes owed heavy supersessions of charter content.

### D3 — Schema section sets unify in this same session and build — `Contested` *(amended at review: I7, M6)*

**Statement:** the session's scope covers both surfaces of every pair: six `.md` files adopt the D2 scaffold AND six schemas adopt the unified vocabulary (D4). Mechanics: old section nodes tombstone (D11/D14), rules relocate with IDs and texts unchanged (D14 precedent; verified per I8's build check), checker and `.md` enumerations re-key; provenance untouched (rule-ID-keyed, F6a). **Retreat branch (M6):** the surgery is structurally one-way (tombstones never reuse); if live evidence shows the unified vocabulary harming — mis-addressed rules, desk/run semantics blurring — retreat to per-form vocabularies is a named option **reserved to the user**, reconstructible from strips per GI-006.

**Rationale:** the lead recommended deferring schema-side unification with a revisit trigger; steelman *(I7)*: defer costs nothing now, the split vocabulary harms only cross-command queries (none observed yet), and unify-now spends tombstones + relocation + checker rework on symmetry; its cost is a second full wave later and a scaffold whose `.md` enumeration cannot be made uniform (D5) while vocabularies differ — the deferred road forfeits exactly the byte-identical enumeration the user is buying. The user ruled unify-now: full pair symmetry in one move. `Contested`.

### D4 — Unified vocabulary is implement's six-set; `reserved` first-class everywhere — `Confident` (choice) / `Assumed` (in-use benefit) *(amended at review: C3, M4)*

**Statement:** all six schemas adopt `<cmd>.sec.roles` · `reserved` · `tools` · `ways-of-working` · `boundaries` · `fail-conditions`. Desks extract reserved rulings out of `roles`; goal-form commands redistribute `harness`/`bindings` across the six. Rule IDs and texts unchanged; old nodes tombstone. **Prefix policy (M4):** existing rule-ID prefixes (`impl`/`feat`/`arch`/`spec`/`setup`/`brainstorm`) are **frozen** — re-prefixing would re-mint ~320 mint-once rule IDs, which is not the "nearly free" move it appears; the known `spec.*`/`spec.yaml`/`.mochiko/specs/` collision is accepted as-is. A **derivation rule for future commands** is recorded: a new command's prefix is its command filename stem, abbreviated only when a recorded collision forces it, the choice noted in the schema header.

**Rationale:** lead recommended, user accepted — same superset logic as D2; every command holds user-reserved decisions, so `reserved` earns first-class standing; richest set means zero rule-content compression. Choice `Confident`; the benefit of six-section addressing in live use is `Assumed` at n=0 (C3).

### D5 — Breadth invariant on the schemas: all six sections always present; empty is explicit — `Confident` (choice) / `Assumed` (runtime cost) *(amended at review: C3)*

**Statement:** every schema carries all six section nodes. A section with no rules carries one deliberate empty marker (grammar detail owed at build: explicit `rules: []` plus a one-line `note:` naming the emptiness deliberate — never a silent omission). The `.md` enumeration is therefore byte-identical in shape across all six commands; the checker asserts all six sections present per schema.

**Rationale:** the user first ruled present-when-populated, then reopened the question unprompted and ruled always-present on the fuller three-option framing (hybrid included). Deciding weight: enumeration variance is exactly what this session closes. Cost accepted eyes-open: dead sections read on every run — **unmeasurable at n=0, so the cost leg is `Assumed` (C3)** and rides the extended watch.

### D6 — Batch ruling: checker lint · ceremony re-key · one-wave build · hygiene rider — `Confident` (ship) / `Assumed` (lint efficacy, n=0) *(amended at review: C2, I2, I3, I8, M3; mark split at verify, N6, per the record's own C3 rule — D13 precedent)*

**Statement (user-ruled "adopt all as recommended"):**

- **R1 — checker `.md` lint, as widened at review:** the D13 advisory checker gains a scaffold lint over the `.md` side — canonical headings present and in D2 order · the Rules-block section enumeration asserted **set-wise against the schema's section IDs, not by count** (I3 — the count-vs-count check goes vacuous under D5) · **every `<cmd>.sec.*` token anywhere in the `.md` resolves to a live node** (I2 — catches `setup.md:35`/`setup.md:40`, which sit outside the Rules block) · **no rule text references a tombstoned or re-homed node** (I8 — the D14 precedent was same-vocabulary relocation; this wave is not) · the Not-done count-pin matches the schema's `fail-condition` count. New assertions **negative-tested** (house precedent: the D14/D15 guards). Advisory exit-code only; GI-019 bright line holds.
- **R2 — audit ceremony re-key:** the dual criteria blocks in `.claude/rules/mochiko/primitive-edits.md` collapse to one canonical-scaffold criteria block **that internally branches on done-condition class (C2): desk commands graded on per-visit convergence, run commands on the fixed-done-condition contract (entry gating, named user gate, attempt bounds where applicable)** — the branch is inside one block, on record, not two blocks pretending to be one. Supersession-by-ruling entries land against the full F3 carrier set *(M3: command-content-schema D9 included; C1: the purge ADR included)*, exact clause inventory owed at build (house precedent: the pm-role D6 inventory).
- **R3 — build shape:** one wave — six `.md` rewrites + six schema unifications + checker rework + ceremony re-key + strips, **plus the ripple, tooling, and watch items per the build surface (items 6–8)**; author ≠ grader audit per pair; release gates as governed.
- **R4 — hygiene rider:** the "the 1 rules labeled" grammar defect dies in the canonical Not-done line (correct pluralization).

**Rationale:** all four are consequences of D1–D5; the user adopted the batch in one ruling; the review's lint-hardening folds (I2/I3/I8) and the C2 branch clause were user-ruled in the disposition rounds.

### D7 — Framing ruled: this session is a stage-1.5 tidy, not the stage-2 on-ramp — `Confident` *(review fold, C4; user-ruled)*

**Statement:** the canonical scaffold is a tidy of what stays prose under command-content-schema D2 stage 1. It neither advances nor retards the absorption trigger. **Rider, eyes-open:** if the absorption trigger later fires, the three newly-authored Identity & Mission sections (brainstorm, setup, specify) are strip candidates by that ruling's own ceremony — they are authored now knowing that, and the sound-loop cost of authoring them is accepted against the value of a uniform scaffold in the interim.

**Rationale:** F5 declared the framing decisive; the reviewer caught that no decision answered it while D2 thickens the `.md` against the pre-authorized thinning trajectory. The user ruled tidy — the alternative (on-ramp) would have reopened D2's superset shape.

## Cost line *(review fold, I5)*

Priced and accepted: **build** — 6 producer-class `.md` rewrites (3 containing new Identity & Mission judgment writes under the sound-loop floor) + 6 schema unifications + checker rework + ceremony re-key + ripple (README, ARCHITECTURE, conversion skill); per-pair author ≠ grader audits — comparable D10 wave ran 5 producers + 5 validators with 2 round-1 FAILs; expect that order. **Runtime** — a few deliberately-empty sections read on every run (unmeasured, n=0, rides the watch). **Budgets** — commands carry no per-primitive char budget by explicit standing user ruling; that stance is unchanged here, so the identity-cap (D2 step 2) is a scaffold rule, not a budget.

## Session trail

- Opened 2026-08-27. Ground facts F1–F6a read from the tree and the command-content-schema record before Q1.
- Q1 (scope): drift-only vs collapse-forms vs stage-2 thin → user ruled **collapse forms** → D1. F6 surfaced while recording.
- Q2 (merged shape): superset vs minimal core vs compressed-identity → user accepted the recommended **superset** → D2.
- Q3 (schema sets): defer vs unify-now vs never → user ruled **unify now** → D3. F6a recorded (anchor-cost correction; three vocabularies).
- Q4 (target vocabulary): six-set vs five-set vs three-set → user accepted the recommended **six-set** → D4.
- Q5 (empty-section rule): first pass ruled present-when-populated; the user reopened unprompted and, on the fuller three-option framing, ruled **always present, explicit empty** → D5.
- Q6 (convergence batch R1–R4): user ruled **adopt all as recommended** → D6.
- Review sizing gate: lead recommended solo; user ruled **solo cold review**.

## Review + disposition trail

- **Solo cold review** via blind-map two-message dispatch (40-angle map, topic-only spawn; fence held). Reviewer self-verified all load-bearing ground facts — F1/F2/F3/F4/F6/F6a held exactly.
- 32 raised · **19 survived — 4 Critical · 9 Important · 6 Minor** · 13 reviewer-killed with recorded reasons. Verdict: **critical-gaps**.
- Dispositions, all user-ruled: **C1** (unnamed purge supersession) ruled *supersede explicitly* → D1/F3/F7/D6-R2 folds · **C2** (nominal collapse) ruled *branch inside one block, narrow D1* → D1/D6-R2 folds · **C3** (n=0, open watch) ruled *proceed, re-mark `Assumed`, extend watch* → D2/D4/D5 confidence folds + build item 8 · **C4** (F5 unruled) ruled *stage-1.5 tidy* → D7. Important/Minor batch ruled **"adopt all as recommended"** (I9 keep-order variant): I1→build item 6 · I2/I3/I8→D6-R1 widening · I4→build item 7 · I5→cost line · I6→D2 frontmatter leg · I7→D1/D3 steelmans · I9→D2 identity cap + watch probe · M1→this structural repair · M2→"Adaptive" kept · M3→D6-R2 carrier · M4→D4 prefix policy · M5→open question · M6→D3 retreat branch.
- **Verify round 1: verified, not blocking** — 19/19 dispositions confirmed landed (purge-ADR quote, BACKLOG watch state, the 320-rule census, all ripple line numbers reviewer-re-verified against the tree); 6 non-blocking nits (N1 restored goal-form opener destination · N2 enumeration "identical in shape" wording · N3 "Adaptive" names the family · N4 no-clause-superseded carve for contract-only carriers · N5 R3 points at build items 6–8 · N6 D6 mark split `Confident`-ship/`Assumed`-efficacy per the record's own C3 rule) — all six lead-repaired same round. Reviewer-recommended status: **ready**.

## Build surface (cold-buildable)

1. **Six `.md` rewrites** to the D2 scaffold (Adaptive Goal Protocol heading kept; identity capped; `argument-hint` added). New Identity & Mission prose for brainstorm/setup/specify = judgment writes — sound-loop floor applies. Charter content relocates; goal-form bold-`**Goal:**` opener content relocates into protocol steps 1–2 (N1); nothing ruled dies except the split (D1 supersessions across the F3 carrier set, N4 carve honored).
2. **Six schema unifications** to the D4 six-set under D5: tombstone `{brainstorm,setup,spec}.sec.{harness,bindings}` · mint `sec.reserved` where absent · redistribute rules, IDs/texts unchanged — **with the I8 check that no surviving rule text references a tombstoned or re-homed node** · explicit empty markers (grammar in Open questions). Re-key **every** `<cmd>.sec.*` reference in every `.md`, not just Rules blocks (I2 — `setup.md:35`, `setup.md:40`).
3. **Checker rework** (D6-R1 as widened): set-wise section-ID assert · all-token resolution · tombstone-reference lint · scaffold headings/order · count-pin; negative tests for each new assertion; `cargo test` green (gate 6).
4. **Ceremony re-key** (D6-R2 as amended): one criteria block, internal done-condition-class branch; exhaustive clause inventory over all F3 carriers (incl. purge ADR decision 4, command-content-schema D9) before any supersession entry. The criteria block is the canonical scaffold's durable home — no new template file (purge precedent honored in form even as its decision 4 is superseded).
5. **Strips + supersessions** per `.mochiko/strips/README.md`; author ≠ grader audit per pair; gates 4/5/6.
6. **Ripple (I1):** `README.md:5` (states the two-form split to users) · `ARCHITECTURE.md` charter-form sites (95, 294, 296, 312 — hand-maintained legacy, D1 makes them wrong, not merely stale).
7. **Conversion-skill update (I4):** `.claude/skills/converting-command-to-schema/SKILL.md` — step 2 vocabulary, step 8 thin-`.md` spec, exemplar line — re-keyed to the canonical scaffold so a seventh command cannot reintroduce the superseded pattern.
8. **Watch + falsifier (C3):** the open first-live-run watch extends to the new scaffold — probes: schema Read executed fully **before first action from position 3, behind identity prose** (I9) · empty-section runtime cost observed (D5) · one concrete six-set addressing benefit (D4). Baseline perturbation named eyes-open: the pre-rewrite pair form will never fire live; delivery evidence lands only on the new scaffold.

## Open questions

- **Empty-marker grammar (D5):** exact YAML form (`rules: []` + `note:` vs a marker rule block) — build-time detail; the checker must recognize it.
- **Desks' one-clause FAIL sets (M5):** architecture and feature each carry a single `fail-condition` rule — possibly under-extracted at the D10 rollout. Trigger: the build's per-pair audit re-inventories FAIL-shaped content in both desks' schemas; a found under-extraction is a separate ruled addition, not a silent widening.
