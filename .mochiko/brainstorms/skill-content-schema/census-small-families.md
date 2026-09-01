# Skill-content schema — wave-2 census: small families (testing · analysis · singletons)

**Seat:** wave-2 census (small families) · **Date:** 2026-09-01 · **Status:** delivered
**Referent law:** `record.md` D1–D9 as amended (skill-content-schema) · command-content-schema D12/D15 ·
near-dup ADR R1–R6 (`.mochiko/decisions/2026-08-28-near-dup-convergence.md`) · the wave-1 census
(`census.md`, this directory) as structural referent, its §K build corrections included.
**Corpus:** 8 skills in three micro-families — **testing** (`testing-end-user` TEU ·
`testing-gap-finding` TGF · `testing-governance-injection` TGI), **analysis** (`analysis-codebase` AC ·
`analysis-iterative` AI), **singletons** (`grooming-operating-docs` GOD · `brownfield-integration` BI ·
`executing-tdd-cycle` ETC). The `mochiko` router is out of scope (D1 as amended). Every `SKILL.md`
read whole; every `references/*.md` and extra in-dir file read whole; every `.mochiko/strips/<name>.md`
read whole; `DECISIONS.md` grepped for traceable lines. Paper exercise — this file is the only write.
**Measurement:** characters of the parsed value per the canonical snippet in
`.mochiko/memory/primitive-cost-budgets.md`, taken 2026-09-01 against the quiesced tree; all eight
figures reconcile exactly with the ledger + latest strip entries.

All rule IDs provisional (wave-1 R-a idiom); filename-stem prefixes assumed (R-b). Dispositions use
the D3-as-amended vocabulary: **body-stays-prose** / **moves-to-schema** (move) / **reference-stub**
(stub). Every move of protected content is a D8/C4 supersession-transfer, recorded at conversion.

---

## A. Protected-set reconciliation (FIRST, per D9/C4)

All 8 members have strip files. Reconciliation is at the unit of **live protection** (the wave-1 J-1
posture): KEPT/RETURNED survivor rulings, keep-deliberately sets riding recorded supersessions, and
`DECISIONS.md`-traceable ruled machinery. Ended protections listed with their ending ruling. Nothing
below is dispositioned "delete".

### Testing family

#### TEU — testing-end-user (strips: 10 entries)

| Protected unit | Lineage | Status | Census disposition |
|---|---|---|---|
| KEPT: envelope register + prose-on-clean check restated in `references/REPORT-TEMPLATES.md` — "Cut it only with a ruling that also re-homes the check" | [v0.44.0] KEPT entry; verbosity-caveman D4/S2, `DECISIONS.md` 2026-08-01 | live, in reference | **reference-stub** — the stub's `pointer:` names REPORT-TEMPLATES.md; the sanctioned dual-homing (this payload home + `templates/report-format.md`) is ruled, and the stub must not read as the anti-dual-homing violation (see J2-10) |
| Whole parsing algorithm · field extraction · legacy-marker normalization · grammar-owner banner; legacy task-line form stays parseable by design | [v0.49.0] supersession keep-set; `DECISIONS.md` 2026-08-02 plan-task-granularity D2 | live | grammar-ownership banner rule **moves-to-schema** (binding); parsing algorithm stays in TASK-PARSING.md (**reference-stub** for the grammar-owner-wins line) |
| Guardrails keep-set: Overview + letter/spirit epigraph, When NOT to Use, entire Core Process incl. browser-flow exception, Quality Gates + Execution, Red Flags, Rationalizations, Mistakes, Reference Files | [v0.64.0] supersession keep-set; guardrails Wave 2, `DECISIONS.md` 2026-08-11 | live, compressed | obligations **move-to-schema** per §B; teaching tables (Red Flags, Rationalizations, Mistakes) **body-stays-prose**; the STOP-meta-rule moves |
| Quality-gate source = `tasks.md` `## Quality Gates` + project build configuration; always-auto-resolve (deterministic ground truth, never judgment); exit-code classification; `quality_gates` frontmatter slot | [v0.91.0] supersession keep-set; plan-stage retirement D4, `DECISIONS.md` 2026-08-26 | live | **moves-to-schema** (gate-source binding + auto-resolve floor) |
| Browser-flow classification exception (FLOW-XXX Playwright walk → CLI; binding surface is the flow, pixels advisory, never an assert target) | UX-D9, `DECISIONS.md` 2026-08-02 "UX mocking in specify" row ("testing-end-user browser-flow classification") | live | **moves-to-schema**, wording preserved in substance |
| `**TEST:**` grammar stays with `patterns-vertical-tdd`; TEU consumes | `DECISIONS.md` 2026-08-16 vertical-TDD row ("`**TEST:**` grammar stays with `patterns-vertical-tdd`") | live | **moves-to-schema** (binding + boundary) |

#### TGF — testing-gap-finding (strips: 3 entries, all supersession-by-ruling; born by ruling v0.79.0)

No KEPT lines — a birth-by-ruling body (the RSUF class, wave-1 J-2 idiom). The live protection is the
DECISIONS-traceable D1–D10 machinery of the 2026-08-19 "QA gap-finding verification ruled" row, plus
the v0.79.0 build's V2/F2 fence-delegation guard and the v0.81.0/v0.91.0 re-key keep-sets:

| Protected unit | Lineage | Status | Census disposition |
|---|---|---|---|
| Blindness fence: explicit inclusion list (as re-keyed — sufficiency report + design-phase deltas, those two artifacts only; store concern rows only, spine outside) + structural exclusions (code · cards · `**TEST:**` cases · cycle reports · builder's tests) | D3 + v0.81.0 D12 re-point + v0.91.0 A6 guard (strip keep-sets verbatim) | live, complete | **moves-to-schema** (fence floor; the two-artifacts-only and spine-exclusion guards ride the rule text verbatim) |
| Delegated reads inherit the inclusion list | v0.79.0 V2/F2 guard (DECISIONS build clause "fence delegation guard (V2 F2) added to the skill") | live | **moves-to-schema** (floor) |
| Two-message dispatch, expectations-before-probing | D3 | live | **moves-to-schema** (floor) |
| Finding split by kind never severity; spec-violation blocks with evidence + clause; lead confirms blocking; disputed kind defaults advisory to the user; finder never gates alone | D6 | live | **moves-to-schema** (constraint + reservation) |
| Rework bound (whole-run default 2, redeclarable only at run open; exhaustion reserved to the user); out-of-territory routes to `/mochiko:feature` delta card | D6 | live | **moves-to-schema** (bound + reservation + routing) |
| Fold-back: `gates.md` contract (mint at first fold or card authoring inside the implement run; survives graduation; union read), QA craft authors, never the exploratory seat; as-designed not folded | D7 + v0.91.0 mint-moment re-key keep-set | live | **moves-to-schema** (binding + independence) |
| Mutation lens: alongside-never-inside, HIGH depth only, diff only, timeboxed, absent/flaky = skipped AND noted, survivors advisory; tool is an advisory post-hoc exit-code checker — never gates, never dispatches, never holds judgment | D5 + D10 (GI-019 carve-out language) | live | **moves-to-schema** |
| Done condition + disclosure (probed-or-unprobeable-with-reason, counts disclosed); zero findings is a clean pass | D8 | live | **moves-to-schema** (duty + constraint; note the inverse-of-default-FAIL polarity, J2-6) |
| Seat = `mochiko:devils-advocate` | D4 as amended (I1 reseat) | live | **moves-to-schema** (binding) |
| Scope carve: selection + epic (once over union); delta/lane skipped, skip stated — silent no-op is a defect | D2 | live | **moves-to-schema** |

#### TGI — testing-governance-injection (strips: 3 entries)

| Protected unit | Lineage | Status | Census disposition |
|---|---|---|---|
| [v0.25.0] KEPT: entire remaining body (whole-skill survivor ruling) | strip survivor entry, batch-2 ratification 2026-07-25 | **partially ended** at v0.63.0 — probe-plan bounds + two-probe-types definitions recorded superseded there | per D8/C4: the surviving KEPT behaviors (next row) carry the live protection; the whole-body claim ended in form. No re-home ambiguity — the v0.63.0 entry enumerates the survivors |
| Surviving KEPT behaviors: contaminated-lead rule (probes always fresh subagents) · zero-expectations brief · both-matrix-directions findings · unconditional cleanup + `git status` verification · version-stamped findings | [v0.63.0] protected-content reconciliation, survivors enumerated | live | **move-to-schema**, each as its own rule |
| Versioned-harness-behavior-never-doctrine floor; findings route to amend run / BACKLOG, never hand-fixed (setup-owned surfaces) | v0.63.0 keep-set ("the unconditional-cleanup floor…; the versioned-harness-behavior-never-doctrine floor") | live | **move-to-schema** (floors) |

### Analysis family

#### AC — analysis-codebase (strips: 8 entries)

| Protected unit | Lineage | Status | Census disposition |
|---|---|---|---|
| [v0.24.0] KEPT: four Essential-Floor check tables | strip survivor entry | **ended** at v0.63.0 (recorded supersession; canonical definitions live in `authoring-constitution/references/ESSENTIAL-FLOOR.md`) | n/a — historical. Residual: the dangling "using the indicators below" pointer, an accepted defect shipped byte-faithful to the ruled variant (J2-9) |
| [v0.24.0] KEPT: Common Mistakes table (incl. CODEOWNERS relocation target) | strip survivor entry, pilot ratification 2026-07-25 | live | rows are teaching **body-stays-prose**; the two obligation-shaped rows (only-report-what-is-found; never-redefine-the-Floor) **move-to-schema** |
| Assess-status contract: canonical-definition boundary ("Do not redefine the categories here"), intent-blind + waiver-blind rule (never soften absent, never mark waived) | v0.63.0 guardrails keep-set; also the 2026-08-13 architect-role row (tech-lead carries `analysis-codebase`) | live | **moves-to-schema** (floors) |
| Two-arm output binding: `mochiko-cli template codebase-analysis` or Read `plugins/mochiko/schemas/codebase-analysis.yaml` raw | [v0.76.0] strip, schema-based-template-guidance D1/D8 (GI-020) | live | **moves-to-schema**, both arms preserved verbatim (the RPA two-arm precedent) |
| Parked-mode carve-outs (collision/JSON inventory = spec/design-cluster, not wired), meaning preserved | [v0.91.0] strip keep-set, plan-stage retirement D1 vocabulary re-key | live | **moves-to-schema** (routing) |

#### AI — analysis-iterative (strips: 6 entries)

| Protected unit | Lineage | Status | Census disposition |
|---|---|---|---|
| [v0.25.0] KEPT: four question-format templates + ratification-streak doctrine | strip survivor entry | **ended** at v0.63.0 (recorded supersession; intent carried as Common-Mistakes guardrails) | n/a — historical |
| [v0.25.0] KEPT: confidence-signals table · asymmetry principle · two-output-shapes dispatch | strip survivor entry | **partially ended** at v0.63.0 — signals table + asymmetry superseded; **two-output-shapes dispatch survives intact** | the surviving dispatch seam: shape/engine claim **moves-to-schema** (binding); the shape narratives **body-stays-prose** |
| Unknown-surfacing floor: "surface every elicited unknown as an open question … never silently omitted" | v0.63.0 floor-line addition, `DECISIONS.md` 2026-08-11 Wave-1 row ("four floor lines … unknown-surfacing (`analysis-iterative`)") | live | **moves-to-schema** (`class: floor`, wording verbatim) |
| Confidence-indicator vocabulary (Confident/Assumed/Contested/Unsure/Deferred, conversation-observation assignment) | v0.63.0 keep-set (Output section kept) | live | **moves-to-schema** (constraint) |

### Singletons

#### GOD — grooming-operating-docs (strips: 3 entries)

| Protected unit | Lineage | Status | Census disposition |
|---|---|---|---|
| Whole body = owned craft (v0.64.0 audited **body no-op** — "the Overview, the 8-step Procedure, and the Boundaries are all owned craft whose obligations survive nowhere else") | [v0.64.0] strip; `DECISIONS.md` 2026-08-11 Wave-2 row names the no-op | live | obligations **move-to-schema** per §B; the 8-step procedure **body-stays-prose** (D3) |
| Delivery-sweep contract: stated-obligation verification, adjacent-work-never-counts, run-gated-stay-open, delivered claims independently re-verified, closures user-ratified | `DECISIONS.md` 2026-08-01 row + ADR `.mochiko/decisions/2026-08-01-groom-delivery-sweep.md` | live | **moves-to-schema** (duty + reservation) |
| Compress-and-move-never-delete; never edit session/decision records; trail append-only | v0.64.0 keep-set + OD-D11 lineage (`DECISIONS.md` 2026-07-25) | live | **moves-to-schema** (floors) |
| v0.44.0 leak-test keep-set (four adopter-valid sweep sources) | [v0.44.0] strip keep-deliberately | live | rides the delivery-sweep rule's text |

#### BI — brownfield-integration (strips: 6 entries)

| Protected unit | Lineage | Status | Census disposition |
|---|---|---|---|
| [v0.25.0] KEPT: EXTEND/MODIFY consumption table · Read-Before-Write checklist · Conflict Detection · When to Flag · Rationalizations table | strip survivor entry, batch-2 ratification 2026-07-25 | live, all sections surviving (v0.64.0 reconciliation: "The When-to-Use cut removes NONE of these") | interface MUST-NOTs + read-full-file + flag-routing **move-to-schema**; checklist steps 2–5 and Conflict-Detection items ride two set-rules; Rationalizations table **body-stays-prose** |
| [v0.49.0] KEPT: entire consumption discipline (read-before-write, interface preservation, EXTEND-never-silently-becomes-MODIFY, conflict escalation) | strip keep-set, plan-task-granularity D2.1, `DECISIONS.md` 2026-08-02 | live | **moves-to-schema** (floors) |
| Consumes-the-classification boundary (builder classifies at decomposition; declared design-time by `patterns-vertical-tdd`) | [v0.49.0] supersession; PT-D7 (rung-zero thin form pointers here), `DECISIONS.md` 2026-08-05 | live | **moves-to-schema** (boundary) |

#### ETC — executing-tdd-cycle (strips: 11 entries)

| Protected unit | Lineage | Status | Census disposition |
|---|---|---|---|
| KEPT: envelope register + prose-on-clean check restated in `references/CYCLE-REPORT-FORMAT.md` ("Cut it only with a ruling that also re-homes the check") | [v0.44.0] KEPT entry; verbosity-caveman D4/S2 | live, in reference | **reference-stub** (same J2-10 dual-homing note as TEU's twin) |
| [v0.49.0] keep-set: cycle-boundary restriction (does not add/remove/re-scope/reorder cycles — decomposition unlocked, slicing not) · red/green/refactor strict order · rework-only-failed-tasks · fix-pass scoping · verifier boundary | plan-task-granularity D2.1, `DECISIONS.md` 2026-08-02 | live | **move-to-schema** (floors + constraints) |
| [v0.53.0] keep-set: self-disclosure framing (report is not a verdict), lead's verdict ownership, verifier-grades-independently | PT-D8, `DECISIONS.md` 2026-08-05 | live | **moves-to-schema** (reservation) |
| Pre-code ladder at decompose, rung disclosed; ladder single-sourced at `mochiko:patterns-code-minimalism` | PT-D4, `DECISIONS.md` 2026-08-05 | live | **moves-to-schema** (floor-pointer duty) |
| "Whether to rework … and when to stop are the lead's routing decisions" — the line the 2026-08-07 bounds ruling pinned as staying true | `DECISIONS.md` 2026-08-07 row ("`executing-tdd-cycle`'s 'lead's routing decision' line stays true") | live | **moves-to-schema** (reservation, wording preserved in substance) |
| Test-case-bundle card reading + `Covers` citation contract (reference) | 2026-08-16 vertical-TDD D1/D3, [v0.75.0] strip | live, in TASK-PARSING.md | **reference-stub** |
| `domain_deps_added` visibility floor: non-empty always forces a human checkpoint — never auto-approved (reference) | CYCLE-REPORT-FORMAT.md field table | live, in reference | **reference-stub** (user-gate) |

**Reconciliation totals:** 6 distinct KEPT/RETURNED survivor rulings across the corpus (TEU ×1 ·
TGI ×1 · AC ×2 · AI ×2 · BI ×1 — counting the two ETC/TEU reference-home KEPT twins once each per
member makes 8 KEPT entries in strip files), of which **3 are fully ended** by recorded supersession
(AC check-tables · AI format-templates · TGI's whole-body claim ended in form with survivors
enumerated) and 2 partially ended with survivors enumerated. **~30 live protected units** enumerated
above, every one carrying a named disposition; zero stay-silent; no disposition is "delete". TGF and
GOD are birth-/craft-by-ruling bodies with no KEPT lines — their moves cite the owning `DECISIONS.md`
rows per rule (the wave-1 J-2/R-c idiom; ride-along confirmation at the wave gate).

---

## B. Obligation census at D12 grain

One row per independently-citable obligation; `class` floor/must/advisory; `kind` from the
**eight-kind skill set** (`fail`/`enforces:` retired, checker-enforced); `constraint` = omitted
default. Procedure stays prose (D3): TEU's execution-sequence steps, TGF's probe-kit walk, TGI's
matrix mechanics, AC's context-gathering sub-procedure, AI's questioning engine, GOD's 8-step
procedure, BI's checklist narrative, ETC's r/g/r phase walkthroughs, and all Red-Flags /
Rationalizations / Mistakes teaching tables are NOT inventoried (their obligation-shaped meta-rules
are).

`when:` dimensions observed live: TGF run-scope (selection/epic/delta/lane — entry-derived) + depth
level (entry-derived) + mutation-tool-present (surface-presence) · TEU task-classification is
content-derived at runtime (no `when:` needed — it is the rule's own subject) · AC brownfield-setup
is the invoking context (entry-derived, arguably unconditional within the skill) · AI output-shape
(entry-derived from the caller's brief). All resolvable with the existing two resolution kinds; no
new kind needed.

### Testing family

#### TEU — testing-end-user (21 rules: 16 body + 5 reference stubs)

| # | Obligation (verbatim fragment) | class | kind | disp. |
|---|---|---|---|---|
| 1 | grammar ownership: `**TEST:**` construct "authored and owned by `patterns-vertical-tdd` in TEST-GRAMMAR.md. This skill **consumes** that grammar; it does not redefine it" — this skill owns the runtime | must | binding | move |
| 2 | "Execute in strict order. No skipping steps. No reordering." | floor | constraint | move |
| 3 | "Fail fast if any setup fails — a setup failure blocks action execution" | must | constraint | move |
| 4 | modifier execution semantics owned here: background PID-tracked for cleanup pass-or-fail; timeout enforced, kill + mark `TIMEOUT`; `(in path)` honored | must | duty | move |
| 5 | assert evaluation semantics owned here (substring / `test -f` / status compare / Playwright checks); "Any other assert text is a custom assertion for human evaluation" | must | duty + routing | move (5a semantics, 5b custom-to-human) |
| 6 | "Each assert MUST receive an explicit pass/fail evaluation. No default to PASS — an unevaluated assert is a failure" | floor | constraint | move |
| 7 | report machine-first per REPORT-TEMPLATES.md: all-PASS = frontmatter only; any FAIL/PARTIAL/TIMEOUT/ERROR = `## Failures` | must | binding | move |
| 8 | "The human decision gates completion — no proceeding without explicit human approval" | floor | gate | move |
| 9 | runtime classification owned by this skill; CLI / GUI / SUBJECTIVE table decides auto-approve vs human checkpoint | must | constraint | move |
| 10 | browser-flow exception: FLOW-XXX-cited Playwright walk with machine-evaluable asserts only classifies CLI; a subjective/custom assert anywhere keeps SUBJECTIVE/GUI; "the binding surface is the flow … visual appearance stays advisory and is never an assert target" | must | constraint | move (protected, UX-D9) |
| 11 | "Default to SUBJECTIVE if uncertain … Ambiguity is a reason to escalate to a human, never a reason to auto-approve. Any failure, on any classification, forces a checkpoint" | floor | constraint | move |
| 12 | result vocabulary: PASS / FAIL / PARTIAL / TIMEOUT / ERROR | must | constraint | move |
| 13 | pre-checkpoint completion set (setup done · actions run · asserts evaluated · evidence captured · report generated); "No presenting partial results. No skipping evidence capture" | must | duty | move |
| 14 | quality gates "always auto-resolve … the answer is an exit code, not a judgment"; "MUST NOT be softened into an LLM judgment call"; gate source = `tasks.md` `## Quality Gates` + the project's own build configuration | floor + must | constraint + binding | move (14a auto-resolve floor, 14b source binding — both protected) |
| 15 | no-exceptions + Red-Flags meta-rule: "Rationalization in progress. Return to the execution sequence. Follow every step" | floor | constraint | move (tables stay prose) |
| 16 | evidence capture per EVIDENCE-CAPTURE.md (types, PID tracking, cleanup) | must | binding | move |
| 17 | grammar-owner-wins: "when the two would differ, the grammar owner wins" (TASK-PARSING.md) | must | constraint | stub |
| 18 | missing `**Action**`/`**Assert**` = parsing error — "Do not attempt execution. Ask the human how to proceed" (TASK-PARSING.md) | must | routing | stub |
| 19 | sanctioned-set closure + prose-on-clean-is-a-defect + register rule (REPORT-TEMPLATES.md — the [v0.44.0] KEPT restatement) | floor | constraint | stub (protection transfers, J2-10) |
| 20 | truncation bounds + full-log pointers; checkpoint presentation never persisted (REPORT-TEMPLATES.md) | must | constraint | stub |
| 21 | cleanup protocol: on failure keep logs, report locations, cleanup after review (EVIDENCE-CAPTURE.md) | must | duty | stub |

#### TGF — testing-gap-finding (28 rules, all body; no references directory)

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | boundary: declared `**TEST:**` execution is `mochiko:testing-end-user`'s; the construct owned by `mochiko:patterns-vertical-tdd` — "consumed here when folding findings back, never redefined" | must | routing + binding | move (1a/1b) |
| 2 | one pass at final validation, whole built feature; selection-scope runs run; epic "once, over the union of member territories" | must | constraint | move |
| 3 | delta-scope + product-lane "skipped, and the final-validation report states the skip explicitly. A silent no-op is a defect" | floor | duty | move |
| 4 | "Probing runs against real infrastructure, never mocks" | floor | constraint | move |
| 5 | blindness fence: "Admissible inputs are an explicit inclusion list, not a layer label" — the enumerated list incl. "those two artifacts only, never the `FEAT-XXX/` run-output directory at large" and "concern rows only" | floor | binding | move (protected, D3 + A6) |
| 6 | "The store's spine deep view stays outside the fence" | must | constraint | move |
| 7 | structural exclusions: "the code · the cycle cards (`tasks.md`) · the `**TEST:**` cases · cycle reports · the builder's tests" | floor | constraint | move |
| 8 | "Delegated reads inherit the inclusion list … never delegated" | floor | constraint | move (protected, V2/F2) |
| 9 | two-message dispatch: message 1 inclusion-list refs only; expectations stated on the record before message 2 opens probing | floor | duty | move |
| 10 | seat: `mochiko:devils-advocate` — "Persona carries the judgment, this skill carries the procedure" | must | binding | move |
| 11 | expectations enumerated as a **numbered list** before probing, across the five families | must | duty | move |
| 12 | observability findings "Advisory-only findings, always" | must | constraint | move |
| 13 | probe kit runs at both depth levels; "breadth is invariant" | must | constraint | move |
| 14 | mutation lens "runs alongside, never inside, the blind explorer — on the existing verification seat" | floor | constraint | move |
| 15 | mutation lens: HIGH depth only · this feature's diff only · timeboxed | must | bound | move |
| 16 | "Tool absent = lens skipped AND noted — never silent"; flaky suite = skipped, skip noted | must | duty | move |
| 17 | "Surviving mutants are beyond-spec advisory findings — never blocking" | must | constraint | move |
| 18 | the tool "never gates progress, never dispatches agents, never holds judgment this skill owns" (GI-019 posture) | floor | constraint | move |
| 19 | findings "split by kind, never by severity"; spec-violation blocking "with evidence captured and the spec clause cited"; broken NFR-XXX qualifies | must | constraint | move |
| 20 | adjudication: finder proposes; "the lead confirms the blocking classification at the checkpoint verdict"; "a disputed kind defaults advisory and goes to the user, who rules. The finder never gates alone" | floor | reservation | move |
| 21 | rework bound: "whole-run bound, default 2 rounds, redeclarable only at run open"; cycle-localized findings charge that cycle's attempts; "Bound exhaustion or a no-progress round halts the run; the disposition is reserved to the user" | must | bound + reservation | move (21a/21b) |
| 22 | out-of-territory gaps route to a `/mochiko:feature` delta card, cited in the report — "not this run's rework" | must | routing | move |
| 23 | done condition: "every derived expectation has been probed, or explicitly marked unprobeable with a reason, within the charter's timebox" | must | duty | move |
| 24 | report discloses expectation count, probed count, per-unprobeable reasons, findings by kind | must | binding | move |
| 25 | "Zero findings is a clean pass — no never-zero rule, no quota"; padding corrupts the disclosure | must | constraint | move (J2-6 polarity note) |
| 26 | fold-back: fix-now/backlog gaps authored as `**TEST:**` cases "by the QA craft (`mochiko:qa-engineer`), never the exploratory seat"; as-designed does not fold | must | constraint | move (independence-shaped) |
| 27 | `gates.md` artifact contract: minted at first fold (or card authoring inside the implement run), "surviving graduation"; the named read source; later reads = union of territory `gates.md` + cards' cases | must | binding | move (protected, D7 + v0.91.0 re-key) |
| 28 | quality checklist = the reviewer's mirror of rules 2–27 | must | duty | move as one set-rule (items dedup against the rules above at conversion) |

#### TGI — testing-governance-injection (13 rules, all body)

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | scope: tests what the harness DOES; "static structure/trace grading is `validation-constitution`, not this skill" (+ not-a-substitute When-NOT bullet, dedup at conversion) | must | routing | move |
| 2 | "stamp every finding with the Claude Code version and date, and never promote an observation into a skill or template as a timeless fact" | floor | constraint | move (protected) |
| 3 | never fix what it finds: "rules files and the region are setup-owned; findings route to an amend run (or land as `BACKLOG.md` empirical items), never hand-fixed around the ownership boundary" | floor | routing | move (protected; author≠fixer shape — evidence note vs review-common.author-grader, §C) |
| 4 | "probe an accepted set" — never mid-authoring | must | constraint | move |
| 5 | "Both directions are findings" — under-injection and over-injection | must | constraint | move (protected) |
| 6 | known-behavior verify-not-rediscover: injection fires on Read not Write; "confirm the line is present and that the read-back it instructs actually injects" | must | duty | move |
| 7 | unconditional cleanup: "Delete every stub the run created — even when probes fail or the run is interrupted — then verify with `git status`" | floor | duty | move (protected) |
| 8 | report contract: trigger matrix, per-finding expected-vs-observed + severity + probe, harness version + date, token cost | must | binding | move |
| 9 | routing: `paths`/region changes → `/mochiko:setup` amend; harness surprises → BACKLOG; behavioral-probe failures → both | must | routing | move |
| 10 | "Probes are always fresh subagents" (contaminated-lead false positive) | must | constraint | move (protected) |
| 11 | new-file probes "Write first, report, then Read back" | must | duty | move |
| 12 | introspection briefs "ask for raw context contents only" — zero expectations | must | constraint | move (protected) |
| 13 | "Always one surface expected to inject nothing" (negative control) | must | duty | move (protected obligation; the ×-per-glob-set quantification stays ended) |

### Analysis family

#### AC — analysis-codebase (13 rules: 12 body + 1 reference stub)

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | deliverable binding: `.mochiko/memory/codebase-analysis.md` per the `codebase-analysis` schema — "invoke `mochiko-cli template codebase-analysis` when the binary is available; otherwise Read `plugins/mochiko/schemas/codebase-analysis.yaml` raw" | must | binding | move (both arms verbatim, GI-020 — protected) |
| 2 | artifact-format envelope, "slimmed but legible": findings in tables with file-cited evidence; judgment prose stays prose | must | binding | move |
| 3 | Essential-Floor categories "defined canonically" in `authoring-constitution/references/ESSENTIAL-FLOOR.md`; "Do not redefine the categories here"; this skill owns assess-status only | floor | binding + routing | move (cross-dir pointer, J2-7) |
| 4 | "The assessment is intent-blind and waiver-blind by design. Report what IS — the same codebase gets the same status" | floor | constraint | move |
| 5 | "Do not soften an `absent` to `partial` because the project 'is young', and do not mark a category 'waived' — waivers are governance rulings, not codebase facts" | floor | constraint | move |
| 6 | downstream interpretation "belongs to the session and the constitution, never to this analysis" | must | reservation | move |
| 7 | collision/JSON inventory out of scope — spec/design-cluster mode, not wired; deeper inventory "not produced here" | must | routing | move (protected re-key) |
| 8 | "Only report what is found" — no inventing findings, no assuming frameworks without evidence | must | constraint | move |
| 9 | determinism boundary: `detect-stack.sh` deterministic layer vs model-judgment layer — "Keep the boundary explicit" | must | constraint | move |
| 10 | Setup-Brownfield quality checklist (12 items incl. all-four-categories-assessed, file-cited, min-2-3 strengths, output path) | must | duty | move as one set-rule |
| 11 | capability signals seed setup's feature-map reconstruction — "map machinery: `mochiko:authoring-feature-map`" | must | binding | move |
| 12 | When-NOT set (greenfield → `authoring-constitution`; single-file; docs-only; complete-context skip) | must | routing | move |
| 13 | CONTEXT-GATHERING.md scope note: standalone report mode not wired; findings land in codebase-analysis, not a separate report file | must | routing | stub |

#### AI — analysis-iterative (12 rules: 10 body + 2 stubs into in-dir template files)

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | "**Floor (non-waivable)** — surface every elicited unknown as an open question in the produced artifacts; a vague zone the principal could not resolve is never silently omitted" | floor | constraint | move (protected, verbatim) |
| 2 | synthesis output binding: generate per `SYNTHESIS.md` (in-dir top-level file) | must | binding | move |
| 3 | confidence indicators "assign based on conversation observation" — the 5-value vocabulary | must | constraint | move |
| 4 | "One question per turn — always" | must | constraint | move |
| 5 | "Nudge, never force; the user has final say" (wrap-up) | must | reservation | move |
| 6 | "Output depth matches conversation depth" — never pad | must | constraint | move |
| 7 | When-NOT routing: spec review → `mochiko:review-specifications` (post-draft vs pre-spec seam); quick clarifications; clear-direction fast wrap; implementation → planning skills | must | routing | move |
| 8 | two output shapes, one engine: "the engine is identical and only the agenda and the concluding artifact differ"; enrichment variant per SPECIFICATION-INPUT.md → ENRICHMENT.md artifact | must | binding | move (protected dispatch seam) |
| 9 | caller-side dispatch: how a caller asks for the shape is "carried in the caller's own brief, not something this skill parses or owns" | must | constraint | move |
| 10 | multiple-questions / disconnected-questions / premature-synthesis guardrails (Common-Mistakes obligations beyond rows 4–6) | must | constraint | move as one set-rule; table rows stay prose |
| 11 | enrichment completion: "do NOT ask any follow-up questions; the enrichment is complete … This skill does not decide what runs next" (SPECIFICATION-INPUT.md) | must | constraint | stub |
| 12 | "Always preserve the original" input verbatim in the enrichment artifact (ENRICHMENT.md) | must | constraint | stub |

### Singletons

#### GOD — grooming-operating-docs (8 rules, all body)

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | source of truth: "read it first" — `.mochiko/memory/knowledge-management.md`; "this skill carries the craft, never the numbers" | must | binding | move |
| 2 | delivery-sweep contract: verify each open item's "stated obligation" against delivered state; "Adjacent work never counts; run-gated items stay open without run evidence"; per-item verdict delivered/partial/open with citations | must | duty | move (protected, 2026-08-01 ADR) |
| 3 | "every delivered claim independently re-verified before it is presented" | must | duty | move |
| 4 | "closure candidates the user ratifies"; partial/stale folds fix-on-sight | must | reservation | move |
| 5 | "Compress and move, never delete — the trail and archives are append-only" (incl. Now-item drops "back to BACKLOG, never deleted") | floor | constraint | move |
| 6 | "Never edit session records or decision records; only indexes, views, and stamps" | floor | constraint | move |
| 7 | judgment calls the docs can't settle "go to the user; everything mechanical is fix-on-sight" | must | reservation | move |
| 8 | expansion-heavy-surface watch: a hit is "log it as a BACKLOG item for the user — never act on it here" | must | reservation | move |

#### BI — brownfield-integration (11 rules, all body)

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | "This skill does not decide the classification — it is the implement-time discipline of consuming one safely" | must | constraint | move (boundary, protected) |
| 2 | `[EXTEND]`: "MUST NOT change existing function signatures, exports, or type contracts" | floor | constraint | move (protected) |
| 3 | `[MODIFY]`: specified sections only; MAY change internals; "MUST NOT change signatures unless the task explicitly says so" | floor | constraint | move (protected) |
| 4 | "Never treat an `[EXTEND]` task as a `[MODIFY]` … surface it as a blocker — do not silently rewrite" | floor | constraint | move (protected) |
| 5 | read-before-write: "Read the full file — not just the section you plan to change" before writing any code | floor | duty | move (protected) |
| 6 | follow the file's conventions exactly (naming · error handling · imports · test patterns — checklist steps 2–5) | must | duty | move as one set-rule |
| 7 | interface preservation: no renames of existing identifiers; add alongside; follow established patterns | must | constraint | move |
| 8 | conflict-detection set (name collisions · import shadowing · test-file alignment · circular deps) before adding code | must | duty | move as one set-rule |
| 9 | When-to-Flag: the five blockers "belong in the cycle report the run produces (owned by `executing-tdd-cycle`), not in a quiet workaround" | must | routing + binding | move (protected escalation seam) |
| 10 | refactoring "out of scope for an extend/modify task; note the opportunity, do not act on it" | must | constraint | move |
| 11 | letter-IS-spirit + no-exceptions meta-rule (Red Flags rebutted by the Rationalizations table, which stays prose) | floor | constraint | move |

#### ETC — executing-tdd-cycle (26 rules: 21 body + 5 reference stubs)

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | slicing boundary: cycles are design-time, `mochiko:patterns-vertical-tdd`'s — "it does not add, remove, re-scope, or reorder cycles" | floor | routing | move (protected) |
| 2 | verifier boundary: quality gates + the `**TEST:**` gate are `testing-end-user`'s, "never this skill's" | floor | routing | move (protected) |
| 3 | "the lead Reads the reports and owns that verdict. This skill produces its own report; it does not grade one" | floor | reservation | move (C4-shaped — evidence note, §C) |
| 4 | routing reservation: which cycle next / retry / fix pass / attempts "are the lead's routing decisions — not this skill's" | floor | reservation | move (protected, 2026-08-07 pin) |
| 5 | no loop/orchestration state — one cycle or one rework, one report | must | constraint | move |
| 6 | "Execute in strict order. No skipping steps. No reordering." | floor | constraint | move (protected) |
| 7 | card reading per `references/TASK-PARSING.md`; current cycle = first unchecked card in order | must | binding | move |
| 8 | decompose at build time "with the code in view"; tasks sized to a single reviewable change, paths named, tests precede implementation | must | duty | move |
| 9 | "decompose exactly what the card's acceptance criteria require — nothing the card didn't ask for" | must | constraint | move |
| 10 | pre-code ladder per `mochiko:patterns-code-minimalism` "Before any red-phase test"; each task's rung disclosed in the report | floor | duty | move (protected, PT-D4) |
| 11 | decomposition "disclosed in the cycle report … not written back into `tasks.md`" | must | constraint | move (protected, D2.1) |
| 12 | red phase: run the test, "verify the failure reason matches expectations"; a test passing without implementation is rewritten | floor | duty | move |
| 13 | green phase: "the minimum code to make the failing test pass"; no unrequired features/abstractions | must | constraint | move |
| 14 | brownfield co-fire: extend/modify tasks read the existing file first and invoke `brownfield-integration` | must | binding | move |
| 15 | refactor scope: this cycle's duplication only; no previous-cycle refactors; no "for the future" abstractions; re-run tests | must | constraint | move |
| 16 | flip the card only after tasks complete + tests pass; the flip is a self-report "verified independently" | must | duty | move |
| 17 | report binding: `cycle-report.md` per `references/CYCLE-REPORT-FORMAT.md`, machine-first, decomposition in structured fields; clean pass = frontmatter-only | must | binding | move |
| 18 | rework: only the reported-failing tasks, traced to decomposition IDs; "Leave passing code untouched"; attempt incremented | must | constraint | move (protected) |
| 19 | fix pass: "Reproduce each one with a failing test before changing any code"; narrowest change, scoped strictly to the reported failure; `cycle: fix` | must | duty | move (protected) |
| 20 | rationalization-STOP meta-rule + no-exceptions ("Not even if the user says 'just write the code'") | floor | constraint | move |
| 21 | Common-Mistakes obligations beyond rows above (failure-reason verification: "a `ModuleNotFoundError` is not a test failure") | must | constraint | move as one set-rule; table stays prose |
| 22 | report field contract: required frontmatter set, IDs local `T{cycle}.{n}`, rung per task (CYCLE-REPORT-FORMAT.md) | must | binding | stub |
| 23 | `domain_deps_added` non-empty "always forces a human checkpoint — never auto-approved" (CYCLE-REPORT-FORMAT.md) | floor | gate | stub (user-gate) |
| 24 | sanctioned-set-of-two + prose-on-clean-is-a-defect + register (CYCLE-REPORT-FORMAT.md — the [v0.44.0] KEPT restatement) | floor | constraint | stub (protection transfers, J2-10) |
| 25 | verifier-owned-task-is-not-an-execution-failure: listed in `failed_tasks:` with reason, triggers no narrative (CYCLE-REPORT-FORMAT.md) | must | constraint | stub |
| 26 | card-parse boundary: "Parse only to know what the cycle must prove; running the cases is `testing-end-user`'s work"; gates read "only to know they exist" (TASK-PARSING.md) | must | routing | stub |

**Census totals:** ≈ **132 rules** at D12 grain — **119 body-borne moves · 13 reference/in-dir-file
stubs · 0 lifts** (every reference-borne obligation sits inside surrounding procedure or a protected
restatement home; stub-default holds corpus-wide). Per family: testing 62 (TEU 21 · TGF 28 · TGI 13)
· analysis 25 (AC 13 · AI 12) · singletons 45 (GOD 8 · BI 11 · ETC 26). Floors: ≈ 33 corpus-wide
(TEU 6 · TGF 8 · TGI 3 · AC 3 · AI 1 · GOD 2 · BI 6 · ETC 10 — counting a/b limbs once).

**Section-set fit (against the wave-1 six-set `independence · scope · inputs · verdict · output ·
reserved`)** — the misfit flag the brief asked for:

| Member | independence | scope | inputs | verdict | output | reserved | Empty markers needed |
|---|---|---|---|---|---|---|---|
| TEU | **empty** | fits | fits | fits (classification + result vocab + auto-resolve) | fits | fits (human gate, custom-asserts-to-human) | 1 |
| TGF | fits (blind seat, fold-back authorship, lens seat-split) | fits | fits (the fence IS the intent) | fits (kind split, zero-is-clean) | fits | fits (lead confirms, user rules disputes/bounds) | 0 |
| TGI | fits thinly (fresh probes, contaminated-lead) | fits | fits thinly | **empty or repurposed** (both-directions classification) | fits | fits thinly (setup-owned routing) | 1–2 |
| AC | **empty** | fits | fits | fits loosely (present/partial/absent grammar + intent-blind posture) | fits | fits (interpretation reserved) | 1 |
| AI | **empty** | fits | thin | loose (confidence vocabulary) | fits | fits (user final say) | 1–2 |
| GOD | **empty** | fits | fits | thin (delivered/partial/open per-item verdict) | fits | strong | 1 |
| BI | **empty** | fits | fits (read-full-file-first) | **empty** | thin (flag into cycle report) | thin (blockers surfaced, never resolved) | 2 |
| ETC | thin (self-report-not-verdict, verified-independently) | strong | fits | **empty or thin** | strong | strong | 0–1 |

The six-set was minted from the grader lifecycle (wave-1 §H: "who-I-am → … → what-I-never-decide").
TGF — the one grader-shaped member — fits 6/6. The executor/producer/facilitator members leave
`independence` empty in 5 of 8 and `verdict` empty or strained in 3. **This is a user ruling the
wave gate must take** (see §J, J2-4): reuse the six-set with empty markers (uniform tooling,
checker unchanged) vs mint a second section skeleton for non-grader skills. Recommendation:
**reuse the six-set with empty markers** — D4's uniform-set + explicit-empty-marker machinery was
built for exactly this, a second skeleton fragments the grammar for ~6 empty sections' savings, and
`verdict` reads naturally as "the skill's outcome grammar" (classification vocabularies, status
grammars) in every strained case.

---

## C. Common-block candidates (R1–R6 screen, WITHIN each micro-family only)

Manual screen (no schemas exist pre-conversion). Bar: **3+ members, near-identical wording**,
strongest-wording-wins (R2), member-specific extras keep local text with allowlist edges (R6).
Cross-family sharing is recorded-rejected (D5); resemblances to wave-1 `skill-review-common.yaml`
blocks are recorded as **evidence only**.

### Testing family (3 members — the only family that can clear the bar)

| # | Candidate | Members | Evidence | Verdict |
|---|---|---|---|---|
| T-1 | consume-never-redefine the `**TEST:**` grammar (owner `patterns-vertical-tdd`) | TEU · TGF (2) | near-identical boundary language in both descriptions and bodies | **BELOW BAR** — TGI has no TEST-grammar contact. Keep local; allowlist edge |
| T-2 | real-infrastructure-never-mocks | TEU · TGF (2) | "verifies against real infrastructure, never mocks" (TEU) · "against real infrastructure, never mocks" (TGF) | **BELOW BAR** — TGI probes the harness, not infrastructure. Keep local; allowlist edge |
| T-3 | skip-noted-never-silent | TGF (mutation lens + delta skip) · TGI arguably (cleanup is unconditional, not a skip rule) | wordings diverge ("skipped AND noted" vs cleanup floor) | **NOT A FAMILY** — different obligations |
| T-4 | rationalization-STOP meta-rule | TEU (1 in-family; ETC and VC are other families) | "STOP immediately … Return to the execution sequence" | **BELOW BAR** in-family; cross-family evidence only (J2-5) |
| T-5 | human-checkpoint gates completion | TEU · TGF partially (checkpoint verdict is the lead's) | different reservations (human approval vs lead+user adjudication) | **NOT NEAR-IDENTICAL** |

**Testing verdict: NO common block clears the 3+ bar. No `skill-testing-common.yaml` is warranted.**
The three members are three different crafts (deterministic executor · blind explorer · harness
prober); their overlaps are pairwise and member-inflected. The 2-member edges (T-1, T-2) are
pre-recorded here for `scripts/similar-rules-allowlist.yaml` at build.

### Analysis family (2 members)

**Structurally incapable of a common file** — the R1 bar requires 3+ members and the family has 2.
Screened anyway for allowlist edges: AC and AI share essentially nothing obligation-shaped (producer
of a repo artifact vs conversational facilitator). Zero edges anticipated; the detector run at build
confirms.

### Singletons (3 members)

**No common by definition** (the brief's own framing; they are not a family). Screened for
cross-member edges the detector will hit:

- *strict-order execution* — TEU + ETC near-verbatim ("Execute in strict order. No skipping steps.
  No reordering."): cross-family (testing/singleton). **Allowlist edge**, keep-distinct — each
  binds a different sequence.
- *letter-IS-spirit epigraph* — TEU + BI + ETC (+ RPA/VC wave-1, already a kept-distinct allowlist
  family): now a 5-member cross-family resemblance. **Evidence only**; extraction across families
  is recorded-rejected (D5). Pre-record the three new allowlist rows.
- *rationalization-STOP* — TEU + ETC (+ VC's restart floor): same treatment.
- *lead-owns-verdict / output-is-input* — ETC rule 3 + TGF rule 20 resemble
  `review-common.verdict-is-input` (C4): **evidence only**, allowlist edges vs the review block.
- *never-fix-what-you-find* — TGI rule 3 resembles `review-common.author-grader` (C3): **evidence
  only**, allowlist edge.
- *single-source-never-restate* — AC rule 3 ("Do not redefine the categories here") + TEU rule 1 +
  TGF rule 1 + BI's marker-vocabulary seam resemble `review-common.its-command-states-them` (C5) in
  shape, not wording: likely below the detector's similarity threshold; note only.

**Projected allowlist additions for wave 2: ~10–14 edges**, all keep-distinct, none suppressing a
genuine extraction.

---

## D. Abort check (the wave-1 test, applied exactly)

The D9-I3 abort test: *"if it yields materially fewer common blocks than the drift driver assumes —
fewer than three clearing the 3+ bar — the wave returns to the user with the evidence before any
conversion begins."*

**Result: 0 candidate blocks clear the bar, in any micro-family and in the corpus as a whole. The
abort condition TRIPS.** Per family: testing **0** (tripped) · analysis **0, structurally
unreachable** (2 members) · singletons **n/a by definition**. Wave 1's headline — "abort check NOT
tripped — 6 common blocks clear the 3+ bar" — inverts exactly here: the primary driver (drift
control via family common blocks) is **absent from this corpus**. A wave-2 conversion stands only on
the secondary drivers the user ranked real at Q1 — deterministic checkability (B) and ceremony
addressability (C) — plus library uniformity (one grammar, one checker, one audit form across all
converted skills).

Per-member convert-worthiness on those B/C grounds:

| Member | Rules | Floors | Protected density | Verdict |
|---|---|---|---|---|
| TGF | 28 | 8 | very high (D1–D10 machinery, 3 ruled re-keys) | **converts well** — the strongest wave-2 case: dense obligations, heavy strip/DECISIONS traffic, real `when:` dimensions |
| ETC | 26 | 10 | very high (5 rulings traceable, 2 KEPT twins) | **converts well** |
| TEU | 21 | 6 | high (KEPT + 4 rulings) | **converts well** |
| BI | 11 | 6 | high (2 live KEPT sets) | converts — moderate pay |
| AC | 13 | 3 | moderate | converts — moderate pay |
| TGI | 13 | 3 | moderate (survivor-ruling remnant) | marginal — small body, thin obligations; pays mainly for uniformity |
| AI | 12 | 1 | low (1 floor line + 1 dispatch seam) | **weak pay** — most body is engine craft the boundary keeps as prose; payload ~doubles for ~12 rules |
| GOD | 8 | 2 | moderate (ADR + OD-D11) | **weak pay** — smallest body (2,666), highest relative multiplier; strong `reserved` content is the best argument |

**Plain statement per the brief:** no member's obligation content is so thin that a pair is
*meaningless* — every member carries at least one non-waivable floor and DECISIONS-traceable
machinery — but **AI and GOD do not pay for a pair on their own merits**; they pay only if the user
values a fully-uniform two-grammar library over per-invoke payload. The evidence-backed options at
the gate: (a) convert all 8 for uniformity, eyes-open on the multiplier; (b) convert the dense five
(TGF · ETC · TEU · BI · AC) and leave AI · GOD · TGI prose with their floors protected as today;
(c) hold the whole wave. This census recommends **(b)** on the record's own economics, with (a) a
defensible user preference.

---

## F. Read-cost projection (D8/C1 · D9/I5)

Current figures: canonical-snippet counts, this tree, reconciled against the ledger. Estimation
method **recalibrated from the wave-1 as-built** (not the wave-1 census's under-estimates): measured
per-rule all-in schema cost across the 8 built pairs = 352–460 chars/rule, mean **≈ 415** (+500
header); post-conversion body ≈ 0.65 × pre-body + ~900 load-first block for these procedure-heavy
bodies (the graders shed ~50% of body; these members keep more prose because the boundary keeps
procedure). **No family common file exists in wave 2 (§C), so delivered-at-invoke = budgeted
payload** — no +1.9k common-read rider, and no member has an RF-style obligated reference read in
its load-first action. ±30% bands; wave-1's census under-shot its own family by ~40% (est ×2.3 →
measured ×3.24), so read these as floors, not ceilings.

| Member | Body now | Desc | Refs (exempt) | Rules | Est. schema | Est. post body | Est. payload = delivered | vs body now | Current budget |
|---|---|---|---|---|---|---|---|---|---|
| TEU | 13,123 | 500 | 24,683 | 21 | ~9,200 | ~9,400 | **~18,600** | ×1.4 | 16,407 |
| TGF | 11,053 | 709 | — | 28 | ~12,100 | ~8,100 | **~20,200** | ×1.8 | unbudgeted |
| TGI | 3,540 | 483 | — | 13 | ~5,900 | ~3,200 | **~9,100** | ×2.6 | 4,425 |
| **Testing** | **27,716** | | | **62** | | | **~47,900** | **×1.7** | |
| AC | 6,613 | 349 | 5,621 (+ script) | 13 | ~5,900 | ~5,200 | **~11,100** | ×1.7 | 8,137 |
| AI | 4,120 | 476 | 18,436 + 11,401 in-dir | 12 | ~5,500 | ~3,600 | **~9,100** | ×2.2 | 4,928 |
| **Analysis** | **10,733** | | | **25** | | | **~20,200** | **×1.9** | |
| GOD | 2,666 | 490 | — | 8 | ~3,800 | ~2,600 | **~6,400** | ×2.4 | 3,333 |
| BI | 6,342 | 491 | — | 11 | ~5,100 | ~5,000 | **~10,100** | ×1.6 | 7,928 |
| ETC | 9,678 | 498 | 13,703 | 26 | ~11,300 | ~7,200 | **~18,500** | ×1.9 | 12,095 |
| **Singletons** | **18,686** | | | **45** | | | **~35,000** | **×1.9** | |
| **Corpus** | **57,135** | | | **132** | | | **~103,100** | **×1.8** | |

Read plainly: **the conversion roughly * 1.8's the corpus's per-invoke delivered payload** (~57k →
~103k est.; a wave-1-style miss would put the measured figure nearer **×2.3–2.5, ~130k**). The
multiplier is lower than wave 1's ×3.24 chiefly because there is no common-file rider and these
bodies keep more prose — but the *absolute* growth (~46–75k chars across 8 skills) buys **no drift
control** here (§C/§D), only IDs + checkability. TEU, ETC, TGF and BI are mounted inside build/verify
seats that run per-cycle — the F4 mid-run multiplier bites hardest exactly on this corpus's three
largest payloads. Budget mechanics per C1 unchanged: each converted member re-seeds to the measured
payload, no headroom, third seeding path; every member lands above its current cap (the designed
re-seed case); TGF/AI in-dir template files and all `references/` stay exempt. Descriptions all
≤1,536 and untouched (max TGF 709, ruled HOLDS at v0.79.0).

---

## I. Labels (`skill-labels.yaml` fit)

The existing 9-label registry covers wave 2 **without new mints**:

- **fence** — TGF's inclusion list/exclusions (the label's paradigm case), ETC/BI read-first duties.
- **verdict** — TEU classification + result vocabulary + auto-resolve posture; TGF kind split +
  zero-is-clean; AC present/partial/absent grammar; AI confidence vocabulary; GOD
  delivered/partial/open.
- **boundary** — grammar-ownership seams (TEU/TGF), slicing/verifier seams (ETC), classification
  consumption (BI), canonical-Floor-definition (AC), pre/post-spec seam (AI), records-never-edited
  (GOD).
- **user-gate** — TEU human checkpoint · TGF disputed-kind/bound-exhaustion · GOD closure
  ratification · ETC `domain_deps_added` forced checkpoint (stub 23).
- **independence** — TGF blind seat + fold-back authorship split · TGI fresh-probe rule.
- **evidence** — TEU capture duties · TGF evidence-captured-clause-cited · TGI version-stamped
  findings · AC file-cited findings.
- **binding · reporting · floor-pointer** — as in wave 1 (report contracts, artifact paths, the
  ETC→`patterns-code-minimalism` and BI→`patterns-vertical-tdd` pointers).

**Considered, recommended against:** a `determinism` label (TEU exit-code auto-resolve · AC
detect-stack boundary · TGF mutation-tool exit-code — 3 carriers, but the rules already link
through `evidence`/`boundary`, and a label minted for 3 rules re-opens registry ceremony for no
consumer). Named here as a graduation candidate if a fourth carrier appears. Registry-edit-first
ceremony unchanged; zero edits proposed.

---

## J. Anomalies (numbered, each with a recommended disposition)

- **J2-1 — abort tripped, wave-shape ruling owed.** §D is the census's headline: zero common
  blocks anywhere. *Recommendation:* present §D's three options at the gate; this census
  recommends the dense-five sub-wave (b).
- **J2-2 — analysis-iterative's three in-dir top-level files.** `SYNTHESIS.md` (4,223) ·
  `SPECIFICATION-INPUT.md` (4,773) · `ENRICHMENT.md` (2,405) live beside `SKILL.md` — neither body
  nor `references/`. The budget exemption's letter names `references/` and `scripts/` only; these
  are exempt by spirit (never auto-loaded, on-demand output templates) but not by letter.
  They carry 2 obligation-shaped lines (§B AI-11/12) → stubs with in-dir top-level `pointer:`
  values. *Recommendation:* one clarifying sentence rides the wave's ledger edit ("in-skill
  template files loaded on demand share the `references/` exemption"); checker pointer-resolution
  already handles in-dir non-references paths.
- **J2-3 — reference mass vs delivered payload.** TEU's references (24,683) and ETC's (13,703) are
  consumed mid-run on their own trigger points (report authoring, card parsing) — not obligated
  load-first reads, so they stay out of the payload figure, unlike wave-1 RF's lens. No J-3-class
  correction needed; stated so the gate isn't surprised by real-run reads exceeding §F.
- **J2-4 — six-section skeleton strains on non-grader skills.** `independence` empty in 5/8,
  `verdict` empty/strained in 3/8 (§B fit table). *Recommendation:* reuse the six-set with explicit
  empty markers (rationale in §B); the alternative — minting a producer/executor skeleton — is a
  named option for the user, priced at a second uniform section grammar and checker surface for
  ~6 empty markers saved.
- **J2-5 — cross-family near-dup families for the allowlist.** letter-IS-spirit (TEU · BI · ETC +
  wave-1 RPA/VC) · strict-order (TEU · ETC) · rationalization-STOP (TEU · ETC + VC) ·
  verdict-is-input shape (ETC-3 · TGF-20 vs `review-common.verdict-is-input`) ·
  author≠fixer (TGI-3 vs `review-common.author-grader`). All keep-distinct under D5;
  ~10–14 allowlist rows pre-recorded in §C. *Recommendation:* lead seeds the rows at build, the
  wave-1 three-pass pattern.
- **J2-6 — TGF's inverted default.** "Zero findings is a clean pass — no never-zero rule" is the
  deliberate polar opposite of the review family's `default-fail` block (the D8 disclosure-based
  honesty mechanism vs the grader's earned-verdict posture). The detector may cluster it against
  `review-common.default-fail`. *Recommendation:* pre-record the keep-distinct edge with this
  polarity note so no build seat "fixes" it.
- **J2-7 — cross-directory pointers (wave-1 J-7 class).** TEU/ETC →
  `../patterns-vertical-tdd/references/TEST-GRAMMAR.md`; AC →
  `../authoring-constitution/references/ESSENTIAL-FLOOR.md`; skill-name pointers
  (`mochiko:patterns-code-minimalism`, `mochiko:patterns-adopt-first` precedent). Already priced by
  the wave-1 checker build (check: pointer resolution accepts climbs); the first-live-run watch's
  M1 installed-cache probe covers these paths too. No new work; named for completeness.
- **J2-8 — TEU/ETC protected dual-homing twins.** The [v0.44.0] KEPT register + prose-on-clean
  restatements in `REPORT-TEMPLATES.md` and `CYCLE-REPORT-FORMAT.md` are RULED restatements of
  `templates/report-format.md` ("Cut it only with a ruling that also re-homes the check"). Their
  stubs must carry the pointer to the payload home, and the audit must not read the ruled
  dual-homing as a D6 anti-dual-homing violation. *Recommendation:* the stub's provenance-sidecar
  entry anchors the v0.44.0 ruling; protection transfers per D8/C4.
- **J2-9 — AC's accepted dangling pointer.** "assess against it using the indicators below" still
  has no indicators below (v0.63.0 residual, shipped byte-faithful to the ruled variant).
  Conversion relocates neighboring text; fixing the sentence is OUTSIDE the conversion's mandate.
  *Recommendation:* flag to the user at the gate as a one-line ruled repair riding the wave (or
  explicitly left as-is); never silently fixed.
- **J2-10 — TGI's whole-body survivor remnant.** The v0.25.0 whole-body KEPT was partially ended
  at v0.63.0 with survivors enumerated; per D8/C4 the five surviving behaviors' protection
  transfers onto their schema rule IDs, recorded once — the RF-style whole-body re-home in
  miniature. No per-line ambiguity: the v0.63.0 strip entry is the enumeration.
- **J2-11 — GOD sits 2,666 chars from the smallest viable pair.** Its projected multiplier (~×2.4)
  is the corpus's worst per rule delivered; its `reserved` content (4 of 8 rules) is the corpus's
  richest per capita. Both facts belong in the §D option-(b) ruling.

---

*End of census. Per D9-I3 this inventory returns to the user at the wave gate with the §D
**abort-tripped** result, the §F read costs, and the §C zero-common evidence before any conversion
begins.*
