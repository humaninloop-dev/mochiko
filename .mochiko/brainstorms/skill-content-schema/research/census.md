# Skill-content schema — census inventory (build step 0)

**Seat:** P0 (census producer) · **Date:** 2026-09-01 · **Status:** delivered, awaiting user gate
**Referent law:** `record.md` D1–D9 as amended (this directory) · command-content-schema D12/D15 ·
near-dup ADR R1–R6 (`.mochiko/decisions/2026-08-28-near-dup-convergence.md`) · lead rulings R-a/R-b/R-c
(P0 plan approval, 2026-09-01).
**Corpus:** the 8 review-family skills, `SKILL.md` whole + every `references/*.md` whole + strips
(`review-sufficiency` has none). Paper exercise — this file is the only write.
**Measurement:** characters of the parsed value per the canonical snippet in
`.mochiko/memory/primitive-cost-budgets.md`, never `wc -c`. All figures below are canonical-snippet
counts taken 2026-09-01 against the quiesced tree.

Member shorthand used in tables (filename stems remain the proposed ID prefixes per R-b — the
shorthand is presentation only): RB `review-brainstorm` · RCM `review-code-minimalism` ·
RF `review-feasibility` · RGI `review-governance-intent` · RPA `review-plan-artifacts` ·
RSPEC `review-specifications` · RSUF `review-sufficiency` · VC `validation-constitution`.

All rule IDs in this census are **provisional** (R-a): mint-once fires at conversion, never here.
Dispositions use the D3-as-amended vocabulary: **body-stays-prose** / **moves-to-schema** /
**reference-stub** (stub is the default for reference-borne obligations; lift only where the
reference home is incidental). Every move of protected content is a supersession-transfer per
D8/C4 — recorded once at conversion, protection re-homing onto the schema rule ID in the
provenance sidecar.

---

## A. Protected-set reconciliation (FIRST, per D9/C4)

**Counting note (see J-1).** The record's "19 `KEPT:` lines across 7 members" is the count of
strip-file lines containing the string `KEPT:` — reconciliation mentions included. The distinct
KEPT/RETURNED **survivor rulings** number 9, several already superseded-in-form by the 2026-08-26
true-deletion cuts; the live protection sits on the compressed keep-sets and the DECISIONS-traceable
ruled machinery. This section reconciles at the unit of **live protection**, which is what the D8/C4
supersession-transfer must carry. Nothing here is silently excluded; historical (ended) protections
are listed with their ending ruling.

### RB — review-brainstorm (strips: 8 entries)

| Protected unit | Status | Census disposition |
|---|---|---|
| [v0.26.0] KEPT: entire remaining body | **ended** at v0.83.0 ("the whole-body survivor status ends here") | n/a — historical |
| [v0.60.0] blind-map / coverage-diff / reopen-born machinery (DECISIONS 2026-08-10) | live, compressed | **moves-to-schema** (blind-map duty · coverage severity + materiality · dismissed-angle rule · reopen-born rule); procedure legs stay prose |
| [v0.88.0] RETURNED eval-demanded re-adds: blind-map-as-own-deliverable (behavioral floor R-001) · commentary clause · tally "N raised, M survived" form · `critical-gaps` criteria | live | **moves-to-schema** — the blind-map rule as `class: floor`; tally + criteria into the output/verdict rules |
| [v0.67.0] class-6 row + calibration clause | live, compressed | **moves-to-schema**; calibration clause → common candidate C6 (stub) |
| [v0.64.0] review-evidence floor line | live, compressed | **moves-to-schema** via common stub (C1) |
| [v0.52.0] EXTERNAL-CLAIMS carve-out (CROSS-EXAM delegation) | live, in references | **reference-stub** — files untouched, binding rule stubs the pointer |
| [v0.46.0] its-command-states-them clause | live | **moves-to-schema** → common candidate C5 |

### RCM — review-code-minimalism (strips: 2 entries; no KEPT lines)

| Protected unit | Status | Census disposition |
|---|---|---|
| PT-D1–D10 core: rung-2/3/5 codebase-read obligation · advisory-never-cycle-failing posture (DECISIONS 2026-08-05) | live | **moves-to-schema** (codebase-read as `class: floor`; advisory posture as floor) |
| [v0.91.0] design-phase carve-out wording (plan-stage retirement D1/D5) | live | **moves-to-schema** (routing rule) |
| [v0.64.0] recorded floor-line SKIP (equivalent present in step 5 + checklist) | live as recorded skip | keep-distinct allowlist edge on common C1 — see §C |

### RF — review-feasibility (strips: 9 entries)

| Protected unit | Status | Census disposition |
|---|---|---|
| [v0.26.0] KEPT: entire remaining body | **ended as body-residency claim** at v0.82.0; named cores live across body + lens | per D8/C4's explicit clause: the survivor protection **re-homes onto the pair** (body + schema jointly), recorded once at conversion — no per-line ambiguity |
| AD-D7 architecture pass (DECISIONS 2026-07-30) — lens A1–A3 + body governance floor | live | **reference-stub** for the pass rules (mandatory-on-store-delta gate, A3 two-exits, boundary watches); the body's never-silently-approved floor **moves-to-schema** |
| [v0.64.0] review-evidence floor line | live (body floor) | **moves-to-schema** via common stub (C1) |
| [v0.67.0] class 7 (blocking-capable · never-alone-`infeasible` · calibration · interrogatory round) | live, in lens + body verdict line | **reference-stub** (calibration → common C6); never-alone-`infeasible` limb rides the verdict rule (**moves-to-schema**) |
| [v0.81.0] element-not-file narrowing + D14 floor-asserted bullet | live, in lens | **reference-stub** |
| v0.82.0 additions: hunt-coverage one-line-per-class disclosure floor · pathed report bindings · canonical gate-fuel field names | live | **moves-to-schema** (disclosure floor, report binding); field names **reference-stub** |

### RGI — review-governance-intent (strips: 6 entries)

| Protected unit | Status | Census disposition |
|---|---|---|
| [v0.26.0] KEPT whole-body — two elements surviving v0.63.0: pair-protocol-by-reference with substrate bindings · D1 formulation-quality exclusion | live, compressed | **moves-to-schema** (cross-exam binding + fact-authority split; D1 exclusion as scope floor) |
| [v0.63.0] guardrails keep-set (verdict table + never-default-`ready` · Common-Mistakes obligations · FAIL posture · jurisdiction floor) | live, compressed at v0.89.0 | **moves-to-schema** per the v0.89.0 disposition map |
| [v0.65.0] declared-level row + missing-declaration critical-gaps arm (adaptive-depth D1/D2/D6) | live, complete | **moves-to-schema** (floor; D6 no-watcher fence preserved verbatim in the rule text) |
| [v0.64.0] review-evidence floor line | live, compressed | **moves-to-schema** via common stub (C1) |
| [v0.46.0] its-command-states-them | live | **moves-to-schema** → common C5 |

### RPA — review-plan-artifacts (strips: 15 entries)

| Protected unit | Status | Census disposition |
|---|---|---|
| [v0.26.0] KEPT: Red Flags · Common Rationalizations · Incremental Review Mode | long-form ended at v0.87.0; obligations live compressed | floors **move-to-schema**; incremental-mode rules move, its narrative stays prose |
| [v0.15.0] KEPT: incremental report-shape block | form ended at v0.87.0; fields live as clauses | **moves-to-schema** (output binding — the `consistency_checks:` field set) |
| [v0.64.0] review-evidence floor line | live, compressed | **moves-to-schema** via common stub (C1) |
| [v0.67.0] three-lens machinery + material-divergence precedence override + class-7 seam sentence | live (re-keyed v0.91.0 to the gap list) | **moves-to-schema** (conformance gate + override as floor-class) |
| [v0.53.0] code-review carve-out (`review-code-minimalism` sole exception) | live | **moves-to-schema** (routing) |
| [v0.75.0] oracle-semantics check (ruled wording; no time anchor, no foundation word) | live in the cycle-card set | **moves-to-schema** inside the cycle-card rule, wording verbatim |
| [v0.76.0] two-arm `--check` citation (`mochiko-cli template tasks --check` / raw Read) | live | **moves-to-schema**, both arms preserved (GI-020) |
| [v0.81.0] store-delta re-key incl. the kept-verbatim qualifying-flow guard ("a P1 journey is the floor, never the cap") and deployment-view row | live — guard + checklists in ARTIFACT-CHECKLISTS.md | **reference-stub** (checklists stay; body lens clause moves) |
| [v0.91.0] gap-list floor + BLOCKING strength (plan-stage retirement D4/D5) | live | **moves-to-schema** (floor) |

### RSPEC — review-specifications (strips: 10 entries)

| Protected unit | Status | Census disposition |
|---|---|---|
| [v0.26.0] KEPT: severity table + Core Principle table | table-form KEPT ended at v0.88.0; substance live | severity grammar + product-not-implementation rule **move-to-schema** (spec-specific wording verbatim) |
| [v0.25.0] RETURNED: five-class canonical-home taxonomy (`devils-advocate` anchor) | live, compressed | canonical-home claim **moves-to-schema** (binding); class descriptors **body-stays-prose** (teaching content) |
| [v0.67.0] class-6 excess row + calibration | live | **moves-to-schema**; calibration → common C6 |
| [v0.63.0] guardrails keep-set + review-evidence floor line | live, compressed at v0.88.0 | floors **move-to-schema**; C1 stub for the evidence line |
| [v0.58.0] feature-layer 10 checks + R13 git-baseline rule | live, complete | baseline rule + same-reviewer-same-report **move-to-schema**; the 10 checks stay in the two check rules' text (**moves-to-schema** as two set-rules — see §B) |
| [v0.50.0] S&F 8 checks + serve-and-click + authority split | live, complete | serve-and-click duty + authority split + two-legal-shapes **move-to-schema**; checks as two set-rules |
| [v0.53.0] carve-out | live | **moves-to-schema** (routing) |
| [v0.82.0] envelope wording (density-never-a-gap · rule-8 advisory) | live | **moves-to-schema** (floor) |

### RSUF — review-sufficiency (no strips)

Birth-by-ruling body (plan-stage-utility D2 as amended + Addendum; v0.91.0). Per lead ruling R-c:
**not wholesale-protected** — no KEPT line exists. The D2-ruled machinery (the ten clauses, the
fence + clause-10 carve, delta-scope collapse, default-FAIL, report binding) is DECISIONS-traceable
content: each such rule's move is recorded at conversion citing the `DECISIONS.md` 2026-08-26
plan-stage-retirement row, same ceremony class, no whole-body claim. Flagged as J-2 for
confirmation ride-along at the wave gate.

### VC — validation-constitution (strips: 9 entries)

| Protected unit | Status | Census disposition |
|---|---|---|
| [v0.25.0] KEPT: Red Flags + Common Rationalizations pair | table-form ended at v0.90.0; every rule live as a Floors clause | rationalization-family floor + STOP-and-restart meta-rule **move-to-schema** |
| [v0.25.0] KEPT: Step-7 placeholder pattern list | **ended** at v0.63.0 (recorded supersession) | n/a — historical; section-bound placeholder items live in QUALITY-CHECKLIST.md (**reference-stub**) |
| [v0.65.0] adaptive-depth extensions (declared-level accounting · MAJOR low→high flip) | live | **moves-to-schema** (verdict-block + bump rules, verbatim-in-substance) |
| [v0.76.0] governance-surfaces schema re-key (QUALITY-CHECKLIST header, two-arm) | live, in reference | **reference-stub** |
| [v0.63.0]/[v0.90.0] keep-sets (set-not-file · every-set-MUST-pass · from-file inputs · VALIDATION RESULT contract) | live, compressed | **move-to-schema** per the v0.90.0 disposition map |

**Reconciliation totals:** 9 distinct KEPT/RETURNED survivor rulings (3 ended by recorded
supersession — RB v0.26.0, VC Step-7, RF v0.26.0-as-body-residency; the RF protection re-homes onto
the pair per D8/C4). ~34 live protected units enumerated above; every one carries a named
disposition; zero stay-silent. No disposition is "delete".

---

## B. Obligation census at D12 grain

One row per independently-citable obligation. `class`: floor / must / advisory. `kind` from the
command nine-kind set (`constraint` = omitted default; written here for legibility).
Disposition: **move** = moves-to-schema · **stub** = reference-stub · **prose** = body-stays-prose.
Quotes are verbatim fragments (trimmed with `…`). Procedure and sequencing prose is NOT
inventoried (D3): RB's protocol chain, RCM's five-step procedure, RF's lens worked examples,
RSPEC's category walk, VC's assembly narrative, RSUF's branch narratives stay prose, as do all
teaching tables in references not listed as obligation-bearing.

`when:` dimensions observed live in the corpus (D4 validation): RB pair/solo + lens +
verify-pass · RGI pair/solo + brownfield + lens · RF store-delta-carried · RPA incremental +
delta-carried · RSUF scope=selection/delta + manifest-present. All resolvable as entry-derived or
surface-presence; no new resolution kind needed.

### RB — review-brainstorm (26 rules; source: body unless noted)

| # | Obligation (verbatim fragment) | class | kind | disp. |
|---|---|---|---|---|
| 1 | "never in the room" | floor | constraint | move |
| 2 | lens "sets depth, never jurisdiction" | must | constraint | move |
| 3 | "the lead owns every verdict" + "your status is input" (one rule; body states it twice — dedup at conversion) | floor | reservation | move → C4 |
| 4 | "Phase 0 blind angle map from the topic only, produced as its own deliverable before record contact" | floor | duty | move |
| 5 | "free repo grounding, session artifacts excluded" | must | constraint | move |
| 6 | "cold read before counterpart contact" | must | constraint | move |
| 7 | scenario stress + six hunt classes hunted per decision (class list stays prose) | must | duty | move |
| 8 | "name the cheaper shape; floor/compliance/NFR never excess" | must | constraint | move → C6 |
| 9 | "verify load-bearing claims against the fact-checker map or the files" | must | duty | move |
| 10 | record-integrity lens: "sample-audit the map itself against the files" (`when:` lens) | must | duty | move |
| 11 | "outside-repo claims per `references/EXTERNAL-CLAIMS.md` (owned here)" | must | binding | move |
| 12 | "fitness per `references/RECORD-FITNESS.md`" | must | binding | move |
| 13 | coverage severity "by whether a ruling would likely / plausibly / not have changed" | must | constraint | move |
| 14 | "a dismissed angle is a ruling, not a gap" | must | constraint | move |
| 15 | "findings-formed = count only" | must | constraint | move |
| 16 | "cross-examination per `references/CROSS-EXAM.md` (pair only…)" (`when:` paired) | must | binding | move |
| 17 | survivor report as message: fields + tally ("N raised, M survived") | must | binding | move |
| 18 | status vocabulary + `critical-gaps` criteria (four named arms) | must | constraint | move |
| 19 | "A finding nothing could resolve is commentary, not a finding" | must | constraint | move |
| 20 | verify pass: "grade each fold against the updated record, quoting evidence; new surface only for fold-introduced contradictions" (`when:` verify-pass) | must | constraint | move |
| 21 | fidelity sample: "every ruling present, no confidence mark inflated, no rejected alternative resurrected" | must | duty | move |
| 22 | "Reopen-born decisions get this grade, never a fresh cold read" | must | constraint | move |
| 23 | "never author or revise the record — findings enter through the lead's pen" | floor | constraint | move |
| 24 | "verdict and dispositions left in the reviewed artifacts themselves, never only in conversation" | floor | constraint | move → C1 |
| 25 | "never raise a `Contested` decision unless the angle is genuinely new to the ruling" | floor | constraint | move |
| 26 | "never default to `ready` — zero findings means hunt harder, never manufacture" + "an unverifiable claim is a finding" (two rules, counted as 26a/26b) | floor | constraint | move → C2 / move |

### RCM — review-code-minimalism (12 rules)

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | grading standard is `mochiko:patterns-code-minimalism` — "never a copy of the ladder" | must | binding | move |
| 2 | "runs inside the per-cycle verification … no separate stage, no final-pass sweep" | must | constraint | move |
| 3–5 | three carve-outs: general code review out · TEST/quality gates → `mochiko:testing-end-user` · design-time → design-phase and spec reviewers (sibling = rung-honesty grade in `review-plan-artifacts`) | must | routing ×3 | move |
| 6 | "Read the report file itself, never a relay of it" (+ diff AND report both read) | must | constraint | move |
| 7 | "A missing rung note is itself a finding" | must | constraint | move |
| 8 | rung-2/3/5 codebase-read obligation — "Never take reuse claims on trust" (greps · stdlib · manifest) | floor | duty | move |
| 9 | floor-line check: cut past a floor/accessibility obligation is a finding | must | duty | move |
| 10 | output: "One `minimalism:` entry per finding … empty block; never narrate a clean grade" | must | binding | move |
| 11 | "A `minimalism:` finding never fails a cycle" — advisory posture | floor | constraint | move |
| 12 | rung dispute "escalates to the user only at the checkpoint, never as a mid-cycle stop" | must | reservation | move |

### RF — review-feasibility (23 rules: 13 body + 10 reference)

Body:

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | sibling split: sibling owns "coverage / measurability / consistency / presence; you own contradiction / impossibility / buildability" | must | routing | move |
| 2 | "Never author or fix what you grade" | floor | constraint | move → C3 |
| 3 | "Load references/FEASIBILITY-LENS.md before hunting" | must | duty | move (folds into the D6 load-first sequencing at conversion — see J-3) |
| 4 | "Never default to `feasible` — earned only by a completed hunt; absence of looking is not evidence" | floor | constraint | move → C2 |
| 5 | hunt coverage "discloses as one line per class in the report, never a narrative" | floor | duty | move |
| 6 | "`infeasible` never flattens into `needs-revision` … escalates to the human" | floor | constraint | move |
| 7 | "Governance is never silently approved — two exits only: redesign to conform, or a user-ruled amendment/waiver via `governance-ledger.md`" | floor | routing | move |
| 8 | "Verdict + per-finding dispositions land in the reviewed artifacts and the filled report" | floor | constraint | move → C1 |
| 9 | report binding: `templates/feasibility-report-template.md` under `templates/report-format.md` | floor | binding | move |
| 10 | "Findings cite the IDs in tension … with the four gate-fuel fields" | must | constraint | move |
| 11 | "external premises verify per `../review-brainstorm/references/EXTERNAL-CLAIMS.md`" | must | binding | move |
| 12 | "Your verdict is input — the lead owns clearing, loops, and the human gate; G1: design-phase artifacts only, never the constitution" (two rules 12a/12b) | floor | reservation / constraint | move → C4 / move |
| 13 | verdict vocabulary: "`feasible` · `needs-revision` (resolvable) · `infeasible` (fundamental)" | must | constraint | move |

Reference (`FEASIBILITY-LENS.md`, 17,514 chars — obligation-shaped lines only; lenses/worked
examples stay untouched):

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 14 | "a skipped lens is not a clean lens" | must | constraint | stub |
| 15 | "the seam is between two *elements*, never merely between two files" | must | constraint | stub |
| 16 | single-artifact flaw = sibling's, "class 7 … the one remove-shaped class and may fire from a single artifact" | must | routing | stub |
| 17 | class 7 "blocks `feasible` … but never alone at `infeasible`" | must | bound | stub |
| 18 | class-7 calibration: "admissible only when it names the cheaper alternative or the specific bar breached; a floor-, compliance-module-, or NFR-derived obligation is never excess" | must | constraint | stub → C6 |
| 19 | adopt-first limb (hand-built commodity with no rationale beating a named candidate = excess; trigger/floor at `mochiko:patterns-adopt-first`, never restated) | must | constraint | stub |
| 20 | interrogatory round — "you may put questions to the producer … the verdict stays yours alone" | advisory | latitude | stub |
| 21 | architecture pass "Fires when the design-phase package carries a store delta" (`when:` surface-presence) | must | gate | stub |
| 22 | A3 boundary watch: conformance graded, "never whether the governance itself is well-formed" | must | routing | stub |
| 23 | gate-fuel field names are the template's own (`gap / at / impact / fix`) | must | binding | stub |

Lens content that duplicates body floors (verdict recap, guardrail rows restating never-default /
evidence / G1) is a dedup note at conversion, not extra rules.

### RGI — review-governance-intent (27 rules)

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | frozen-window contract: "Frozen from your spawn until dispositions land" | must | constraint | move |
| 2 | "You run before ratification, spawned at the sizing gate" | must | constraint | move |
| 3 | "never a participant … you are the challenge" | floor | constraint | move |
| 4 | "never author, revise, or ratify — the lead owns every verdict, the user owns ratification" | floor | reservation | move → C3/C4 |
| 5 | lens is "depth, never jurisdiction"; "report real out-of-lens trips" (two rules 5a/5b) | must | constraint | move |
| 6 | "solo, the whole surface is yours"; verify pass to coherence lens in a pair, to you when solo (`when:` solo/pair) | must | routing | move |
| 7 | permanently out: "the authored surface set and its Tier-2 grading (`mochiko:validation-constitution`, downstream…)" | floor | constraint | move |
| 8 | D1 exclusion: formulation/enforceability quality "closed `Contested`, D1 — never re-raise" | floor | constraint | move |
| 9 | sequestration: "the entire attack formed before counterpart contact; the lead withholds the name" | must | constraint | move |
| 10 | reads: frozen synthesis + agenda ("its ten dimensions are the coverage yardstick") + brownfield `codebase-analysis.md` (`when:` brownfield) | must | binding | move |
| 11 | finding contract: severity + GI element(s) + "concrete failure scenario or cited contradiction" + resolution path | must | constraint | move |
| 12 | "unresolvable = commentary, not a finding" | must | constraint | move |
| 13 | over-governance hunt in-jurisdiction, "admissible only naming the fact it fails to trace to or the lighter surface" | must | constraint | move |
| 14 | "a floor-, compliance-module-, or NFR-derived obligation is never excess" | must | constraint | move → C6 |
| 15 | cross-exam "pair only … after the lead introduces the counterpart, the one-shot four-message exchange per …CROSS-EXAM.md (the single source)" (`when:` paired) | must | binding | move |
| 16 | substrate bindings: artifact = frozen synthesis · fact substrate = analysis + detect-stack (brownfield; files otherwise) | must | binding | move |
| 17–19 | three fact routes: reality-surface "checked … never argued" · user-declared "flag for the lead to route to the user as confirmation, never to argument" · external-sourced per EXTERNAL-CLAIMS, "never argued" | must | routing ×3 | move |
| 20 | survivor report: "a message to the lead, no report files" + fields + tally + "the fallen stay retrievable on ask" | must | binding | move |
| 21 | verdict criteria 3-state incl. "a missing or unrecorded depth-level declaration among them" (v0.65.0 arm) | must | constraint | move |
| 22 | "Never default to `ready` — earned by a completed hunt"; "too thin to attack … is itself the first finding" (22a/22b) | floor | constraint | move → C2 / move |
| 23 | lead's-pen + "verdict and dispositions land in the reviewed artifacts themselves" | floor | constraint | move → C1 |
| 24 | "honor `Contested` only after its rationale audit (an unaudited `Contested` is a shield; a shield is a finding)" | floor | constraint | move |
| 25 | "echo-rationales and adoption streaks outrank any mark" | floor | constraint | move |
| 26 | declared-level rule: challenge expression/waivers/consistency; verify ledger record, recommend-then-arbitrate, greenfield-low; "never flag it against real users or deployment state (D6 no-watcher), never grade stricter than the declared level" | floor | constraint | move |
| 27 | "yardstick = the agenda, the asserted floor, the synthesis's own internal consistency — never your governance taste" + "a session confirming its own synthesis is the gap this review closes" + its-command-states-them (27a/27b/27c) | floor | constraint / constraint / routing | move (27c → C5) |

### RPA — review-plan-artifacts (33 rules: 26 body + 7 reference)

Body:

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | "The run's floor is the sufficiency report's gap list … never a fixed artifact set" | floor | binding | move |
| 2 | sibling handoff: feasibility findings "hand … off, never grade them" (boundary-table pointer) | must | routing | move |
| 3 | Not-for line (code review with RCM sole carve-out · specs · constitution · still-drafted) | must | routing | move |
| 4 | conformance lens BLOCKING: "every named gap closed, within the gap list's depth" | floor | gate | move |
| 5 | "material divergence … auto-FAILs the package (critical-gaps), overriding the count mapping" | floor | constraint | move |
| 6 | adopt-first disclosure lens BLOCKING (trigger and floor at `mochiko:patterns-adopt-first`; rationale-beats-candidate advisory) | must | gate | move |
| 7 | rung honesty "advisory, never drives the verdict … the standard, never restated" | must | constraint | move |
| 8 | completeness "over whichever sets the caller supplies" per ARTIFACT-CHECKLISTS | must | binding | move |
| 9 | "Tier-1 pre-assert first … a `failed` count is ground truth, folded straight into the issue list" | floor | gate | move |
| 10 | classify per ISSUE-TEMPLATES.md | must | binding | move |
| 11 | "verdict mechanically from the counts (its Verdict Criteria), subject to the divergence override" | must | constraint | move |
| 12 | report per `mochiko:advocate-report-template`; "the one-line `strengths:` field filled" | must | binding | move |
| 13 | cycle-card check set — the seven checks verbatim (incl. v0.75.0 oracle semantics), "Mirrors the `tasks` `--check` view (`mochiko-cli template tasks --check`, or Read `plugins/mochiko/schemas/tasks.yaml` when the binary is absent)" | must | duty + binding | move |
| 14 | incremental: "the caller names the {new} and {prior} sets — never you" (`when:` incremental) | must | reservation | move |
| 15 | incremental scope: full on new; consistency-only on prior; "flagging only inconsistencies *between* artifacts" | must | constraint | move |
| 16 | "no full re-read, 1–2 minutes per artifact" | must | bound | move |
| 17 | "a thorough prior review never waives it" | must | constraint | move |
| 18–20 | three escalations: 2+ issues → full re-read · contradiction → report, lead routes (design-vs-decided Critical here; requirements/constraints feasibility's) · unsure → recommend targeted review | must | routing ×3 | move |
| 21 | incremental report frontmatter: `incremental:` · scope lists · six `consistency_checks:` fields · "a fail also lands as a finding" | must | binding | move |
| 22 | "defaults to FAIL — good enough is never ready: evidence or rejection" | floor | constraint | move → C2 |
| 23 | "the letter IS the spirit: never skip a check, never downgrade a severity … an inapplicable check is flagged N/A with justification, never silently dropped" (23a/23b) | floor | constraint | move |
| 24 | "a Critical/Important issue blocks — never 'noted but not blocking'" | floor | constraint | move |
| 25 | "feature size, producer seniority, time pressure, found-enough never shrink the review; a vague spec is a gap to flag, not permission to propagate" | floor | constraint | move |
| 26 | evidence floor ("verdict and per-finding dispositions land in the reviewed artifacts themselves") | floor | constraint | move → C1 |

Reference (`ARTIFACT-CHECKLISTS.md` 23,051 + `ISSUE-TEMPLATES.md` 4,776 — checklist tables stay
untouched; the obligation-bearing frames gain stubs):

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 27 | severity levels + classification rules (ISSUE-TEMPLATES "Severity Levels" / "Classification Rules") | must | constraint | stub |
| 28 | verdict criteria — the mechanical count mapping (ready / needs-revision / critical-gaps) | must | constraint | stub |
| 29 | analysis checklist set (constraints-and-decisions; "There is no `requirements.md` checklist") | must | duty | stub |
| 30 | store-delta checklist set + no-delta claim checks (`when:` delta-carried), incl. the protected qualifying-flow guard ("a P1 journey is the floor, never the cap") and floor-precedence legality row | must | duty | stub |
| 31 | design + cross-artifact consistency checklist sets (incl. Architecture Conformance keyed to the signed store delta; disclosure-presence per EXTERNAL-CLAIMS) | must | duty | stub |
| 32 | boundary table — "This skill keeps the left column; `mochiko:review-feasibility` owns the right" | must | routing | stub |
| 33 | Tier-1 command forms + exit-code contract; "density is never itself a finding" envelope line | must | binding | stub |

### RSPEC — review-specifications (27 rules; no references directory)

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | "input, never a clearing PASS/FAIL verdict of its own" | floor | constraint | move |
| 2 | "Independent reviewer, never the author" | floor | constraint | move → C3 |
| 3 | "WHAT is missing, never HOW to implement" (+ product questions, never implementation questions) | must | constraint | move |
| 4 | Not-for line (six routes incl. RCM carve-out, `analysis-iterative` disjoint-triggers) | must | routing | move |
| 5 | "Coverage is complete, never sampled: every user story … every success criterion … edge cases hunted per main flow" | must | duty | move |
| 6 | question format: "2–3 concrete options, what each means for users, why it matters — specific, never vague" | must | constraint | move |
| 7 | "never presupposing a mechanism ('should we cache?' assumes caching)" | must | constraint | move |
| 8 | "in the Clarifications shape of `templates/advocate-report-template.md`, never a variant" | must | binding | move |
| 9 | six-class taxonomy is "the canonical hunt taxonomy `devils-advocate` leans on" (class descriptors stay prose) | must | binding | move |
| 10 | class-6 admissibility: "admissible only naming the driver it fails to trace to or the cheaper shape; a floor / compliance-module / NFR-derived obligation is never excess" | must | constraint | move → C6 |
| 11 | "Implementation posture smuggled into a constraint … is an assumption-gap finding" | must | constraint | move |
| 12 | "A regulatory/product-legal assertion is a floor-class external claim — verify per …EXTERNAL-CLAIMS.md; undisclosed is a gap" | must | binding | move |
| 13 | feature layer "graded with the spec, same reviewer, same report" | must | constraint | move |
| 14 | "grade staged writes against the git state of the map at run open, never a workspace copy" (R13) | floor | constraint | move |
| 15 | "map machinery single-sourced in `mochiko:authoring-feature-map`, this list is the reviewer's mirror" | must | binding | move |
| 16–17 | feature-layer check sets: Critical six (derivation honesty · disposition completeness · dedup at baseline · delta legality · SC re-homing · in-flight handling) · Important four (granularity · entry well-formedness · selection-card honesty · specs-index agreement) | must | duty ×2 | move |
| 18 | S&F "Two legal shapes: a SCR/FLOW manifest with its `prototype/` app, or the waiver line" (`when:` manifest-present) | must | constraint | move |
| 19 | "Serve the prototype and click it … adversarial, not ceremonial" | must | duty | move |
| 20 | authority split: "flows, screens, data shown are binding; layout and styling advisory — a cosmetic finding against a low-fi prototype is wrong-altitude" | must | constraint | move |
| 21–22 | S&F check sets: Critical five (reachability · walkability · scenario coverage · traceability · drift both directions) · Important three (data-shape honesty · FEAT tags · waiver second-guess) | must | duty ×2 | move |
| 23 | severity grammar: "Critical = cannot build without this answer … Important = will cause rework … Minor = polish, log and defer" | must | constraint | move |
| 24 | report structure "single-sourced at `templates/advocate-report-template.md`; no report path named → same structure inline" | must | binding | move |
| 25 | "density is never itself a gap … grade substance, never prose style; undisclosed overage … advisory per its rule 8" | floor | constraint | move |
| 26 | evidence floor + "5–7 Critical/Important gaps per round, related gaps grouped — never a 20-gap dump" (26a → C1; 26b bound) | floor | constraint / bound | move |
| 27 | "scope creep is not a gap … never add new ones as 'missing requirements'" + "check existing patterns and decisions first" (27a/27b) | floor | constraint / duty | move |

### RSUF — review-sufficiency (29 rules; no references directory)

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | binding pre-build gate: "`sufficient` … licenses cards and build directly; any gap scopes the in-run design phase to exactly the named gaps, nothing else" | must | gate | move |
| 2 | "you never author the fix — the design phase is a different seat" | floor | constraint | move → C3 |
| 3 | fence read-set: spec (S&F manifest included) + architecture store + product baselines + map entries | must | binding | move |
| 4 | fence never-set: "Never the code, `tasks.md`, `**TEST:**` cases, cycle reports, or this batch's own `FEAT-XXX/` run-output directory" | floor | constraint | move |
| 5 | clause-10 carve: "the sole run-output read this fence admits, scoped to the colliding surface" | must | latitude | move |
| 6 | unit: selection scope "per selected work row"; delta scope "per delta card" with the three-clause collapse (`when:` scope) | must | constraint | move |
| 7 | "Clause 9 does not apply under delta scope" (+ structural re-fire rule) | must | constraint | move |
| 8 | "sufficient only when every applicable clause holds … a clause that cannot be graded is a gap, never a pass" | floor | constraint | move |
| 9–18 | the ten clauses, each with its own gap form (testable criteria · contract exposure · data exposure · structural trigger incl. own-structure exclusion · NFR targets · commodity exposure · dependency order · UX trace · delivered-feature exposure (selection only) · in-flight exposure incl. no-locks rule) | must | duty ×10 | move |
| 19 | "An absent baseline file grades its touched surfaces new (gap), never n/a" + seed duty | must | constraint | move |
| 20 | "a store trip … never becomes a gap … dispositioned by the user at run-open; a recorded deferral is a legal escape, a silent skip is not" | must | reservation | move |
| 21 | verdict "Binding at entry — a gap list routes to the design phase, zero gaps routes to cards and build" | must | gate | move |
| 22 | "A disputed clause defaults to gap and the dispute goes to the user; the grader never clears alone" | floor | reservation | move |
| 23 | report binding: `sufficiency-report.md` in `.mochiko/features/FEAT-XXX/` under the envelope + contents list (incl. the `quickstart.md` null-path record) | must | binding | move |
| 24 | "Defaults to FAIL — a unit is insufficient until every applicable clause is graded. Absence of looking is never evidence" | floor | constraint | move → C2 |
| 25 | "Never author what you grade — you authored none of the three sources" | floor | constraint | move (dedup with 2 at conversion) |
| 26 | "Every clause graded, or flagged n/a with its justification — never silently dropped" (+ the two structural n/a's named) | floor | constraint | move |
| 27 | "Verdict and dispositions land in the report — evidence living only in conversation is a floor violation" | floor | constraint | move → C1 |
| 28 | "Your verdict is input to routing, never a clearing — the lead routes; the user rules trips, in-flight conflicts, and any disputed clause" | floor | reservation | move → C4 |
| 29 | never-reads restated in description ("Never reads code…") — description untouched (D3); no schema rule minted for the description copy | — | — | prose |

### VC — validation-constitution (23 rules: body + 2 reference stubs)

| # | Obligation | class | kind | disp. |
|---|---|---|---|---|
| 1 | "The graded artifact is a set, not a file … grading only the region is partial validation, which is not validation" | floor | constraint | move |
| 2 | "Every set MUST pass — no exceptions" | floor | constraint | move |
| 3 | "The letter of the rules IS the spirit" | floor | constraint | move (keep-distinct vs RPA's copy — §C edge) |
| 4 | Not-for + "verify the document IS a governance set before validating" | must | routing | move |
| 5 | "Inputs — all read from file, never from the author's report" | floor | constraint | move |
| 6 | input set enumerated (region markers verbatim · rules files · ledger · synthesis · trace manifest) | must | binding | move |
| 7 | "A missing synthesis when the set carries trace keys, a missing manifest, or a missing set member — each is itself a FAIL" | must | constraint | move |
| 8 | "A `.mochiko/memory/constitution.md` on disk is a superseded artifact … flag it" | must | duty | move |
| 9 | checklist assembly "module-parameterized: universal core + the checklist fragment embedded in each SELECTED module's file" | must | duty | move |
| 10 | "Verify every item — never skip one as 'obvious', never check a fragment the synthesis did not select" | floor | constraint | move |
| 11 | "vague language MUST be replaced with measurable criteria — patterns … single-sourced in references/ANTI-PATTERNS.md" | must | binding | move |
| 12 | "excess governance is an anti-pattern too, FAIL-eligible … admissible only naming the constraint's existing home … or the missing fact; a principle mandated by the asserted floor, an attached compliance module, or an NFR is never excess" | must | constraint | move → C6 |
| 13 | version bump determination (MAJOR incl. "a low→high depth-level flip" · MINOR · PATCH); "Every change gets a bump determination" | must | constraint | move |
| 14 | VALIDATION RESULT block contract — every line enumerated (verdict · counts · surface integrity six · trace closure four · floor/module accounting three · anti-patterns · bump · issues · advisory) | must | binding | move |
| 15 | "binary PASS or FAIL — no soft language, no middle ground" | floor | constraint | move → C2 |
| 16 | rationalization family → "STOP and restart from checklist assembly" (six named rationalizations) | floor | constraint | move |
| 17 | "placeholders = incomplete, return for completion" | floor | constraint | move |
| 18 | "missing parts = FAIL — return to authoring, never sign off incomplete governance" | floor | constraint | move |
| 19 | "authoring mode ≠ validation mode: 'I reviewed it while writing' is not validation" | floor | constraint | move → C3 |
| 20 | "user satisfaction verifies nothing — enforcement mechanisms are checked, not vibes" | floor | constraint | move |
| 21 | "if the user insists, document that validation was skipped against recommendation — never claim a validated set" | floor | reservation | move |
| 22 | evidence floor | floor | constraint | move → C1 |
| 23 | QUALITY-CHECKLIST.md checklist sets + two-arm governance-surfaces re-key · ANTI-PATTERNS scan list (two stubs; files untouched) | must | duty ×2 | stub |

**Census totals:** ≈ **200 rules** at D12 grain — 181 body-borne (moves-to-schema), 19
reference-borne (stubs), 0 lifts (no reference obligation judged home-incidental; every one sits
inside surrounding procedure, so stub-default holds family-wide).

---

## C. Common-block candidates (R1–R6 screen)

Manual screen — the detector cannot run pre-conversion (no schemas exist); stated per D9-I3. Bar:
3+ members, near-identical wording, strongest-wording-wins (R2), member-specific extras keep local
text with allowlist edges (R6).

| # | Candidate block | Members | Wording evidence | Verdict |
|---|---|---|---|---|
| C1 | **review-evidence floor** — "verdict and per-finding dispositions land in the reviewed artifacts themselves — review evidence living only in conversation is a floor violation" | RB · RF · RGI · RPA · RSPEC · RSUF · VC (7) | near-verbatim across all 7 (v0.63.0/v0.64.0 lineage); RSUF says "land in the report" | **CLEARS**. Strongest wording: the full two-limb form. Edges: RSUF's report-specific target keeps local tail; RCM excluded — its v0.64.0 recorded SKIP (equivalent via output rule) is the allowlist edge |
| C2 | **default-FAIL / never-default-to-clearing** — "never default to `<verdict>` — earned by a completed hunt; absence of looking is never evidence" | RB · RF · RGI · RPA · RSUF · VC (6) | RB "never default to `ready` — zero findings means hunt harder, never manufacture" · RF "Never default to `feasible` — earned only by a completed hunt; absence of looking is not evidence" · RGI "Never default to `ready` — earned by a completed hunt" · RPA "defaults to FAIL — good enough is never ready" · RSUF "Defaults to FAIL … Absence of looking is never evidence" · VC "binary PASS or FAIL" + "Every set MUST pass" | **CLEARS** with `${verdict}`-style var or strongest generic wording + local verdict word. Prime allowlist territory (D1 amendment): vocabularies differ (`ready` ×3 · `feasible` · `sufficient` · PASS/FAIL); RCM and RSPEC issue no clearing verdict — bind nothing here, recorded as the two structural non-members |
| C3 | **author≠grader** — "never author (or fix/revise) what you grade" | all 8 | RB "never author or revise the record" · RCM verification-seat-never-producer · RF "Never author or fix what you grade" · RGI "never author, revise, or ratify" · RPA "never the author" · RSPEC "never the author" · RSUF "you never author the fix" · VC "authoring mode ≠ validation mode" + producer-pair clause | **CLEARS** (8/8 — the family's strongest cluster). Edges: RGI's "or ratify" tail local; VC's fresh-review framing local |
| C4 | **verdict-is-input / reservation to lead+user** — "your verdict/status is input; the lead owns clearing; \<named user reservations\>" | RB · RF · RGI · RSUF · RCM (5) | RB "your status is input; the lead owns the clearing verdict" · RF "the lead owns clearing, loops, and the human gate" · RGI "+ the user owns ratification" · RSUF "the lead routes; the user rules trips, in-flight conflicts, and any disputed clause" · RCM findings "ride … to the lead's checkpoint verdict" | **CLEARS**. Member-specific reservation tails (ratification · trips/disputes · checkpoint) keep local text (R2/R6) — the block carries only the input-not-clearing core |
| C5 | **its-command-states-them** — loop/round-cap/human-gate mechanics never restated; "its command states them" | RB · RGI · RF (3 — RF's copy in the lens guardrails) | verbatim-identical clause, v0.46.0 lineage | **CLEARS** at exactly the 3+ bar. Note: RF's member sits in a reference (stub binds it) |
| C6 | **never-excess carve** — "a floor-, compliance-module-, or NFR-derived obligation is never excess" + the admissibility limb ("admissible only naming the \<driver/fact/cheaper shape\>…") | RB · RF · RGI · RSPEC · VC (5) | near-verbatim carve across all 5; admissibility limb's named object varies (fact / driver / constraint home) | **CLEARS**. The carve converges; the admissibility limb's object noun stays local per member (R2) |

**Screened and kept distinct (below bar or R5 net-reduction fail), recorded for the allowlist:**
- *unresolvable-is-commentary* — RB + RGI only (2).
- *findings-enter-through-the-lead's-pen* — RB + RGI only (2).
- *letter-IS-the-spirit* — RPA + VC only (2).
- *N/A-with-justification-never-silently-dropped* — RPA + RSUF near-identical (2); VC's
  "never skip as obvious" is a different obligation (skip vs n/a-flag), not a third member.
- *RCM sole carve-out line* — RPA + RSPEC (2).
- *sequestration / cold-read-first* — RB + RGI (2); CROSS-EXAM.md carries the pair-protocol copy
  as the sanctioned Single-source (D3/C2), not a schema block.
- *tally "N raised, M survived"* — RB + RGI (2).

**Known heterogeneity, restated for the wave:** verdict vocabularies are 4-way
(`ready/needs-revision/critical-gaps` ×3 · `feasible/needs-revision/infeasible` ·
`sufficient`/gap-list · PASS/FAIL), and RCM (advisory-only) and RSPEC (gap-finding input) issue no
clearing verdict — C2/C4 membership above is drawn accordingly.

**Cross-grammar note (J-5):** C1/C2/C3 are near-identical to command `common.yaml` blocks
(`author-grader-default-fail` family). D5 forbids cross-grammar sharing; these are separate
skill-side blocks with the drift edge recorded in `scripts/similar-rules-allowlist.yaml` at build.

---

## D. Numeric abort check

**6 candidate blocks clear the 3+ near-identical bar** (C1 ×7 · C2 ×6 · C3 ×8 · C4 ×5 · C5 ×3 ·
C6 ×5). The D9-I3 abort threshold is "fewer than three" — **not tripped; the wave proceeds** to the
user gate on the evidence here.

---

## E. `kind: fail` question (D9/M2)

**No member asserts a fail predicate.** Surveyed every rule in §B: the corpus's "FAIL" language is
uniformly the **artifact verdict** (default-FAIL posture, missing-input FAILs, auto-FAIL
overrides) — obligations about what the graded artifact earns, expressible as
`constraint`/`gate`. No skill states an end-state not-done condition of **its own run**, and skills
have no Not-done count-pin for `enforces:` to mirror (F5). The nearest shapes — RF's
"a skipped lens is not a clean lens", RSUF's "a clause that cannot be graded is a gap, never a
pass" — are verdict-earning constraints, not run-fail nodes.

**Recommendation: retire `fail` from the skill-side kind set** (eight kinds:
`constraint` · `duty` · `gate` · `reservation` · `binding` · `bound` · `routing` · `latitude`),
by census evidence per the D4/D9 symmetric retirement path. `enforces:` leaves with it (its only
carrier). Re-admission stays open by the same path if a future family shows a real run-fail
predicate.

---

## F. Per-member read-cost figures (D8/C1 · D9/I5)

Current figures: canonical-snippet counts, this tree. Estimates: coarse (±25%), method —
est. schema = 0.6×body (obligation text relocates) + 120 chars/rule structural overhead (id ·
class · kind · labels · indentation; calibrated against the command pairs: 278–385 chars/rule
all-in, brainstorm.yaml the leanest) + 500 header; est. post body = 0.4×body + 450 load-first
block; **budgeted payload** (the C1 quantity, re-seeded at conversion, no +25% headroom) = post
body + own schema; **delivered-at-invoke** adds `skill-review-common.yaml` (est. ~1,900:
6 blocks × ~230 + header — command `common.yaml` is 2,478 for 9 blocks) read in the same first
action wherever any stub binds (all 8 members bind ≥1).

| Member | Body now | Desc | Refs now (exempt) | Rules | Est. schema | Est. post body | Est. budgeted payload | Est. delivered/invoke | vs body now |
|---|---|---|---|---|---|---|---|---|---|
| RB | 2,748 | 490 | 9,799 | 26 | ~5,270 | ~1,550 | ~6,820 | ~8,720 | ×3.2 |
| RCM | 3,884 | 492 | — | 12 | ~4,270 | ~2,000 | ~6,270 | ~8,170 | ×2.1 |
| RF | 1,901 | 599 | 17,514 | 23 | ~4,400 | ~1,210 | ~5,610 | ~7,510 (+17,514 lens, obligated read today and after — J-3) | ×3.9 |
| RGI | 5,562 | 483 | — | 27 | ~7,080 | ~2,670 | ~9,750 | ~11,650 | ×2.1 |
| RPA | 4,938 | 598 | 27,827 | 33 | ~7,420 | ~2,430 | ~9,850 | ~11,750 | ×2.4 |
| RSPEC | 6,187 | 490 | — | 27 | ~7,450 | ~2,930 | ~10,380 | ~12,280 | ×2.0 |
| RSUF | 6,652 | 686 | — | 29 | ~7,970 | ~3,110 | ~11,080 | ~12,980 | ×2.0 |
| VC | 5,103 | 481 | 10,124 | 23 | ~6,320 | ~2,490 | ~8,810 | ~10,710 | ×2.1 |
| **Family** | **36,975** | | 65,264 | **200** | | | **~68,570** | **~83,770** | **×2.3** |

Read plainly: **the conversion roughly doubles each member's per-invoke delivered payload**
(family ×2.3), the purchase being single-point drift control + IDs + checkability, never size —
the F4 arithmetic, which bites harder for skills (mid-run, multi-seat). The user accepts this
eyes-open at the wave gate or sets a bound (I5); the figure joins the first-live-run watch as an
observable. Budget mechanics per C1: each member's budget re-seeds to the **measured**
post-conversion payload (these are estimates, not seeds); the common file is budgeted once as its
own primitive; existing budget rows (RB 3,122 · RCM 4,612 · RF 2,367 · RGI 6,953 · RPA 6,127 ·
RSPEC 7,734 · VC 6,379; RSUF unbudgeted) are all superseded by conversion re-seed — every one of
the eight lands above its current cap, which is exactly the case C1's re-seed path exists for.
Descriptions untouched (all ≤ 1,536; RSUF 686 the family max).

---

## G. Eval-gate spend estimate (D7/I2, D9)

Judged sample: **2 members minimum** — recommend **RB** (existing goldens + regression grid at
`evals/review-brainstorm/`, the family's highest conversion ratio ×3.2) and **RF** (existing
inventory at `evals/review-feasibility/` per v0.82.0; exercises the reference-stub home and the
obligated-lens read, the family's two structural novelties). Arm shape fixed at pre-registration
(D7-I2): pre-conversion baseline vs converted pair, staged as full skill-directory copies.
Estimated runs: 2 members × 4 arms × 3 replicates = **24 grid runs + ~48 opus judge calls**, plus
per-skill goldens ×3 authored by a non-author seat where not reusable, a settled probe run, and
the user-ratified pre-registered ship bar. Metered spend stated at the wave gate per I2; the
2026-08-28 side-finding applies (opus coverage-judge occasionally returns unparseable arrays —
retry belongs in the runner).

---

## H. Section-set proposal (D4 — minted from the corpus, not the command six-set)

Six sections, uniform across the family, `<skill>.sec.<slug>` IDs, explicit empty marker where a
member has no rules in a section (none forced empty in this corpus; the marker exists for future
members):

| Section | Intent (one line) | Corpus coverage |
|---|---|---|
| `independence` | who is never whom — author≠grader, never-in-the-room/-session, seat fences | 8/8 |
| `scope` | jurisdiction and routing — carve-outs, sibling boundaries, permanent exclusions | 8/8 |
| `inputs` | what is read and never read — substrates, fences, from-file-never-relay, load bindings | 8/8 |
| `verdict` | the clearing grammar and its posture — vocabulary, criteria, default-FAIL, overrides, escalation classes | 8/8 (RCM carries its advisory-semantics rules here; RSPEC its input-not-verdict rule) |
| `output` | report contracts — where evidence lands, template bindings, field sets, the evidence floor | 8/8 |
| `reserved` | decisions reserved to the lead or the user — verdict-is-input, dispute/trip routing, ratification | 8/8 |

Every §B rule homes in exactly one section (independence: C3 cluster + sequestration ·
scope: routing/carve-outs + C5 + C6 · inputs: fences/bindings/read duties · verdict: C2 +
grammars/criteria/overrides · output: C1 + report bindings · reserved: C4 + user gates). The
command six-set (`roles/reserved/tools/ways-of-working/boundaries/fail-conditions`) was tested and
rejected: no Not-done set exists (§E), "tools" and "ways-of-working" map to nothing
obligation-shaped here, and the corpus's natural cleavage is the grader lifecycle
(who-I-am → what-I-touch → what-I-read → what-I-rule → what-I-emit → what-I-never-decide).

---

## I. `skill-labels.yaml` seed proposal

Transfers from `command-labels.yaml` (meaning unchanged, per D4): **independence** ·
**user-gate** · **evidence** · **floor-pointer** · **reporting** · **binding**.

New, corpus-demanded (one line each):

- **verdict** — the clearing grammar: vocabulary, criteria, default posture, overrides.
- **boundary** — a sibling/jurisdiction line: what this skill consumes or routes but never grades.
- **fence** — a read-boundary: surfaces the skill must read, and surfaces it must never read.

Not seeded (considered, rejected): `attempt-economy` / `scope-entry` / `seats` / `landing` /
`stewardship` (command-run machinery with no skill-side carrier); a `taxonomy` label (the class
lists stay prose — nothing to label). Registry-edit-first ceremony applies unchanged; the command
registry's scope statement stays honest (no skill labels enter it).

---

## J. Anomalies (numbered, each with a recommended disposition)

- **J-1 — "19 KEPT lines" is a mention count, not a ruling count.** The 19 counts strip-file
  lines containing `KEPT:`, including reconciliation references; distinct survivor rulings number
  9, of which 3 are already ended by recorded supersession (§A). *Recommendation:* the D8/C4
  supersession-transfer executes at the unit of **live protection** as enumerated in §A; the
  census's table is the reconciliation artifact the audits cite. No record amendment needed — C4's
  figure was "at review count", and the reconciliation is exactly what C4 ordered.
- **J-2 — RSUF's birth-by-ruling body.** No strips, no KEPT lines; R-c confirms no wholesale
  protection. *Recommendation:* conversion moves cite the plan-stage-retirement `DECISIONS.md` row
  per rule; ride-along confirmation at the wave gate.
- **J-3 — RF's obligated reference read.** "Load FEASIBILITY-LENS.md before hunting" makes a
  17,514-char reference an obligated invoke-time read, while C1 keys the budget on body+schema
  and exempts `references/`. No contradiction — C1's exemption text ("never auto-loaded") was
  amended for schemas specifically — but the I5 **real** per-invoke read for RF is ~25k, not
  ~7.5k. *Recommendation:* state both figures at the wave gate (done in §F); the D6 load-first
  block for RF sequences schema + common + lens in one declared first action; no budget change.
- **J-4 — ID prefix verbosity (R-b follow-through).** Filename-stem prefixes on ~200 rules cost
  ≈ +25 chars/rule vs short mnemonics (~5k family-wide, inside schemas). Judged tolerable — no
  collision exists, and resolvable addresses beat short ones (the RPA slug precedent).
  *Recommendation:* stems stand as ruled; no user decision needed unless the wave gate wants the
  mnemonic alternative priced again.
- **J-5 — cross-grammar near-dups.** C1/C2/C3 nearly duplicate command `common.yaml` blocks. D5
  forbids sharing. *Recommendation:* separate skill-side blocks; allowlist edges recorded at
  build (the D5-I7 argued posture).
- **J-6 — verdict heterogeneity carriers.** RCM (advisory-only) and RSPEC (input-only) have real
  `verdict`-section content but bind neither C2 nor C4's clearing limb. *Recommendation:* both
  members carry local verdict-posture rules (RCM §B-11, RSPEC §B-1); the common blocks' member
  lists are drawn as in §C; allowlist notes the two structural non-members so detector reruns
  stay quiet.
- **J-7 — cross-directory stub pointers.** RGI's obligations bind references living in **other
  skills' directories** (`../review-brainstorm/references/CROSS-EXAM.md`, `EXTERNAL-CLAIMS.md`,
  `../authoring-constitution/references/INTERROGATION-AGENDA.md`); RF and RSPEC bind
  EXTERNAL-CLAIMS the same way. Stubs will carry cross-dir relative `pointer:` values.
  *Recommendation:* legal under D3/C2 (pointer names the reference file; the Single-source
  convention governs the files); the D7 checker's pointer-resolution check must resolve
  base-dir-relative paths that climb out of the skill directory — named here so the checker build
  prices it, and the D6/D9 first-live-run watch probes it from the installed plugin cache (the
  M1 path-resolution class).

---

*End of census. Next step per D9-I3: this inventory returns to the user at the wave gate (with
the §D non-abort result, the §F read costs, and the §G spend estimate) before any conversion
begins.*

---

## K. Build corrections (landing annotations, 2026-09-01 — the ontology-wave idiom)

The census above is the frozen audit referent; these are the as-built corrections applied
at the wave landing (v0.100.0):

- **§B preamble:** `manifest-present` belongs to RSPEC (row 18 family), never RSUF (P3
  flag 1, lead-ruled; RSPEC's row 18 itself built unconditional — waiver-arm
  reachability — with rows 19–22 gated, deviation note in its strip).
- **VC header:** 23 → 24 pre-split rows (arithmetic); built 26 with two ruled splits
  (`binary-verdict` · `excess-governance`); pin 14.
- **§C C2 membership ×6 → ×5:** RB keeps local `review-brainstorm.never-default-ready`
  (protected v0.83.0 tail; five allowlist rows). **C1:** RSUF's local rule DOES fire live
  detector edges — six adjudicated keep-distinct rows — contra §F's no-edge reading.
- **RPA:** fresh `author-grader` stub minted (C3's 8/8 held structurally); row 13 split
  duty + binding (+1); row 30 carries both arms in text, unconditional.
- **§H:** RSPEC's `input-not-verdict` homes in `sec.reserved`; the 8/8 coverage claim
  holds.
- **§E executed:** `kind: fail` + `enforces:` retired from the skill grammar
  (checker-enforced errors).
- **§F superseded by measurement:** family budgeted payload 106,879 chars (the ledger
  rows are the source of truth), delivered-at-invoke ≈ 119,895 — **×3.24** vs the ±25%
  estimates; user-accepted eyes-open at the landing gate, the read-cost observable on the
  first-live-run watch.
- **External-claims family** (post-census detector find): user-ruled **keep-distinct** at
  the landing gate; six allowlist rows, extraction reopens if wordings converge.
