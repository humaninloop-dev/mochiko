# Build log — setup-product-agnostic

## 2026-09-24 · wave 1 opened — plan-only dispatch

- Ruling accepted and landed the same day (`DECISIONS.md` row 2026-09-24; `BACKLOG.md` build item).
  User: "lets jump into implementation after".
- Branch `setup-product-agnostic` off `main` at `bbe303f` (plugin 0.114.0; installed cache 0.112.0 —
  seats resolve the installed personas; `review-seat-plan` and `validation-primitive-edit` in both).
  The landing's uncommitted edits ride the branch's working tree; commits suggested, never run.
- Plan: [wave1-setup-product-agnostic.md](wave1-setup-product-agnostic.md). Sequence range 0019–0023
  allocated to S1.
- Sound loop fired on every item (judgment-authored writes on plugin primitives). Three producer
  seats spawned plan-only in the main tree, tree snapshotted before: S1 `tech-lead` (schema content +
  `setup.md`) · S2 `tech-lead` (prose references + strips) · S3 `staff-engineer` (evals + crate census).
  All at persona default `opus`; no deviation.
- Plan-grade path: fresh peers of each persona type invoke `mochiko:review-seat-plan` (installed
  0.112.0 copy); plans read by verbatim extract path in the scratchpad.
- Transport: lead-relayed messaging, no cross-seat mesh; single writer per file per § 1 of the plan.

## 2026-09-24 · plan round — S3 v1 → v2 → FAIL(1,5) → v3; S2 v1 → v2; S1 pending

- Tree diffed clean after every plan return (only the landing and session files changed; no `dirty`).
- S3 (`staff-engineer`) plan v1 returned with four questions, all ruled YES by the lead before the
  grade (not a re-plan): `validate.rs` + `matrix_similar.rs` join its set (halt on clusters > 0) ·
  field-scoped `expected-skills.json` + README :273 re-key licensed if a skill floor moves · three
  pre-existing uncovered ids added to `observable.yaml` as observable (a rubric defect from 0008/0014,
  fixed on sight) · `evals.json` s2 golden re-keyed (a § 7 build defect). v2 sha `461a3900…`.
- P3 (fresh `staff-engineer`, `review-seat-plan` render 0.112.0, 5 floors read back) graded v2
  **FAIL** items 1 and 5: the s1 golden grades the struck greenfield baselines limb (Q5, ruled YES,
  § 7 defect) · the plan replaced the wave plan's re-freeze instruction on its own authority — the
  lead withdrew that sentence by ruling (`baseline_bytes` is a historical constant; `run.py`
  untouched unless the floor set moves) · no attempt bound stated — ruled: two in-write-set fix
  attempts on a red gate then halt; the audit round under `common.gate-loop-bound`. Notes taken:
  `evals.json` `assertions` key · real check-rubric output shape · E7 pass check must test the struck
  claims · the 0003 tuple disclosed under § 7 · E5 also reads specify's `floors:` line (advisory).
  Re-plan round 1 on the shared counter; v3 pending.
- S2 (`tech-lead`) plan v1 returned with four defaulted questions, all ruled to the default before
  the grade: the three module-citing consumers outside the closed list (`EXTERNAL-CLAIMS.md:89`,
  `ANTI-PATTERNS.md:16`, `FEASIBILITY-LENS.md:89`) join its set as § 7 defects · `release-gates.md:39`
  deleted · one strip entry per file per decision · `COMPLIANCE-MODULES.md` header takes a supersession
  entry. Budget pre-assert (parsed chars): every S2 delta negative; headroom relayed to S1 —
  `validation-constitution` 220 (its new rule points at QUALITY-CHECKLIST for the worked cases),
  `review-governance-intent` 19, `authoring-constitution` +201 net after S2's −477. v2 pending.
- Sweep finds beyond the wave plan's line list (S2): `authoring-constitution` description trigger
  and :62–65; `review-governance-intent` :14; router :49 and :158. Hand-offs H1–H9 to S1 ride its GO.
- Tiering: S3 and P3 read first-hand, no explorer spawned (P3 disclosed); S2's dispatches undisclosed
  in its return — asked at GO.

## 2026-09-24 · plan grades — S3 PASS(1) approved and held · S2 PASS(1) GO · S1 to P1 · E8/H4 addendum

- S3 v3 (sha `32337f91…`) re-graded by P3, same seat resumed: **PASS**, all seven items, re-plan round 1.
  Approved; GO deferred until S1 lands (its census figures are read from the landed log). Then the
  lead assigned S3 an addendum from P2's S2 grade: **E8** — two eval kits outside every write set
  grade retired behaviour (`evals/validation-constitution/evals.json` g1/g3 module accounting and a
  legal-mandate waiver FAIL; `evals/review-governance-intent/evals.json` g1–g3 fact-profile, risk,
  legal-mandate findings) — § 7 build defects; v4 (sha `9a78adc5…`) raised **H4**: each kit's
  `rules.json` carries the judge's rule text naming modules, and will lack the minted
  `no-product-instance` rule — ruled into E8 (re-key from the landed views per each kit's `rekey.md`
  precedent); v5 pending, then a bounded P3 grade of the addendum.
- S2 v2 (sha `308f7820…`) graded by P2 (fresh `tech-lead`): **FAIL** items 1 and 5 — IA-12's Amend
  bullet made every amend an event (D4 closes the set; the PATCH clarification class stays) · the two
  eval kits unnamed as consumers · the gate bound uncited. Ruled: IA-12 reworded as the grader
  proposed; kits to S3 (H10); `common.gate-loop-bound` cited. v3 (sha `4d7dceff…`) re-graded
  **PASS**, re-plan round 1. **GO S2 sent** with the branch rulings: branch A on
  `analysis-codebase/SKILL.md` fires (S1 tombstones `capability-signals-seed-feature-map`); QC-10/H2
  and § 2.10/H5 held for S1's landed text; H10 to S3.
- S1 plan (sha `43726582…`): five migrations 0019–0023, one anchor each (D1–D5); retires 8 ids
  (setup: 3 supersede + 2 tombstone; skills: `authoring-constitution.module-mechanical-attachment`
  superseded, `authoring-constitution.s4-fail-safe` and `analysis-codebase.capability-signals-seed-feature-map`
  tombstoned); mints 2 (`authoring-constitution.rule-not-instance`, `validation-constitution.no-product-instance`,
  both `must`); census hand-off 337 → 332 command · 805 → 804 skill · floors 119/264 · fails 36 · setup
  pins 20/6, sections 5/6/11/6/7/6. Five defaults confirmed by the lead before the grade: one
  `replace-document` per template in 0019 · `arch.tools-brownfield-reconstruction` fixed in 0023 as a
  § 7 defect (architecture joins the gate units) · `setup.interrogation-inputs` left alone (the record's
  "seven amended" is six in the log, disclosed) · the design-baseline overview keeps its
  `baseline-delta.md` path until the delta-files build retires it (deviation from § 2b disclosed) ·
  five decision-required additions beyond § 2. `.mochiko/provenance.yaml` does not exist (sidecar
  frozen in the archive) — no provenance write. S1's haiku explorer returned nothing; first-hand grep,
  disclosed. Graded by P1 (fresh `tech-lead`), verdict pending.

## 2026-09-24 · Q6/Q7 ruled — S1 rev 3 · S3 v5 corrected · grade orders to P1 and P3

- S1 rev 2 (sha `0c2e2fff…`) raised Q6 (mint versus clause: `validation-constitution` holds 220 chars
  of headroom) and Q7 (`authoring-architecture-store` +95 on a standing +222 overage). Ruled: Q6 no
  mint — one clause on `validation-constitution.excess-governance`, the worked cases in S2's
  QUALITY-CHECKLIST line; Q7 toward the cheaper shape — the D4 detector lands in `command/architecture`.
  S1 rev 3 (sha `72a522e0…`) rewords `arch.dm-store-integrity-close` (floor · duty · no `when:`, so
  delivered on every visit; `arch.dm-km-landing` is gated on `km_file`) and leaves the store skill
  byte-identical; the lead spot-checked both rules in the view. Census hand-off now 337 → 332 ·
  805 → 803 · 1142 → 1135 · floors 119/264 · fails 36; one mint (`authoring-constitution.rule-not-instance`),
  eight retirements. P1's hold lifted by a content-pinned grade order on rev 3's sha (first grade).
- S3 v5 (sha `3388e19c…`) predated the Q6 correction and added `no-product-instance`; the corrected v5
  (sha `abe292a0…`, pre-correction copy kept beside it) re-keys seven texts across the two kits with no
  entry added, and the E8b stop now reads a minted validation-constitution rule in the view as a change
  beyond the named set. Bounded P3 grade order sent on the addendum (E8 · E8b · § 7–9) against the
  standing v3 PASS.
- S2 executing under GO: 16 plugin files and seven strip files changed on disk plus the new
  `.mochiko/strips/release-gates-module.md`; return pending.
- Fan-in: S1 and S3 went idle without a return message; deliverables pulled from disk per the transport
  floor's fan-in rule, both orders pinned to the on-disk shas.
- Docker daemon up; the `claude-mochiko` sandbox existed stopped and started on the reachability
  probe (`sbx exec claude-mochiko true`); Claude Code 2.1.259 and cargo 1.98.1 inside, credentials
  present. The suite's own preflight runs the auth probe at gate time.
- S1 and S3 return messages arrived after the orders went out and confirm the pinned shas
  (`72a522e0…`, `abe292a0…`).
- P1 had graded an intermediate on-disk state (sha `c87d61a5…`, neither rev 1 nor rev 2) **FAIL**
  items 1 and 5 before the hold reached it; P1 voided that verdict itself — not a grade round, not in
  the `plans:` segment — and grades rev 3 from scratch as the first grade. Two notes P1 will re-check
  against rev 3: the plan names no attempt bound (`common.gate-loop-bound` for G1, the re-plan bound
  for the plan) · the `review-governance-intent` rewords measure +14 by P1's count against the
  plan's "net negative" claim (19 chars of headroom).
- **S2 returned** under GO (sha `4d7dceff…`): 15 of 16 prose files edited, 18 of 19 strip entries
  written (`[v0.115.0]` supersession form; three Q1 entries carry the § 7 build-defect sentence; the
  new `.mochiko/strips/release-gates-module.md`). Budgets measured post-edit, parsed chars, all at
  the pre-assert: `authoring-constitution` body 7,219 (−477) · description 461 (−20);
  `review-governance-intent` body 3,141 (−19); router body 47,148 (−122, unbudgeted);
  `validation-constitution` SKILL.md untouched. Self-check grep leaves three intended hits
  (INTERROGATION-AGENDA :30, :125–126). Tiering: nothing dispatched, first-hand on the seat tier,
  disclosed. Held: QC-10/H2 (awaits `version-bump`'s landed text) and § 2.10/H5. Ruled on the
  ambiguity S2 named in the GO: branch A fires now — the `capability-signals-seed-feature-map`
  tombstone is a ruled item, not S1 wording — the "§ 2.10 waits" clause superseded; QC-10 stays held.
  Branch A then applied: `analysis-codebase/SKILL.md` :68–69 and one `[v0.115.0]` entry in
  `.mochiko/strips/analysis-codebase.md` citing the tombstone by id; body 4,698 (−28). S2 at 16/16
  files and 19/19 entries; only QC-10 open.
- **P1 graded rev 3** (sha `72a522e0…`, first grade, full read, 5 floors read back): **FAIL(5)** —
  items 1–4 and 6 PASS. Item 1: census agrees everywhere (332 · 803 · 1135), Q6 clause text written
  in full, Q7 home verified in the architecture view, D3/D4/D5 match the record. Item 3: every
  "Current:" quote verbatim against the views; exits match protection (four `supersede-rule` on
  anchored rules, four `tombstone-rule` on unanchored `must`, no floor tombstoned); P1's own grep
  confirms setup is the sole `Scope:` writer. Item 4 advisory: the `review-governance-intent` −26
  claim holds by P1's measure; the `excess-governance` clause measures +86 (net ≈ +19), not +71.
  Item 5: no attempt bound anywhere — neither S1's own bound on a red verification step, nor
  `common.gate-loop-bound` for G1, nor the re-plan bound. Fix order sent: rev 4 with one bound
  paragraph (two in-write-set fix attempts then halt · G1 under `common.gate-loop-bound` · one
  re-plan round per seat, a second FAIL to the user), optional tidies named; S1 re-plan round 1
  on the shared counter. P1 re-grades rev 4 resumed. P1 tiering: no explorer, first-hand
  known-path reads, disclosed.

## 2026-09-24 · P3 FAIL(3,4,5) on S3's E8b · `review-common.never-excess` ruled to S1 · S3 v6 order

- **P3 graded the addendum** (v5 corrected, sha `abe292a0…`, bounded to E8/E8b, first-hand reads,
  scratch kit-parity script under `uv`): **FAIL(3,4,5)**, all in E8b; E8 clean (every retired-token
  hit sits in a named golden sentence; no hits in `assertions` or unnamed goldens); items 1, 2, 6
  PASS; the v3 body unchanged (two-attempt bound, gate-loop line, halt shape). Findings:
  (a) `never-excess` in both kits is an `extends: review-common.never-excess` stub — no text in the
  skill view, text at `common/skill-review-common.yaml:27–31`; (b) S1's rev 3 moves four of the
  seven texts E8b named — `checklist-assembly` left alone with reason, `never-excess` untouched
  anywhere; (c) `review-governance-intent/rules.json` `findings-through-leads-pen` already drifted
  from the view at HEAD, so the file-wide parity check is red before the wave and the byte-identical
  fence forbids the fix; (d) the only E8b stop fires on over-movement — nothing halts on a named
  rule that fails to move. P3 verified the id-set, floors-line and tempts checks hold at HEAD
  (26 ids/14 floors · 35/16).
- **Outside S3's plan, put to the lead:** `review-common.never-excess` still reads "floor-,
  compliance-module-, or NFR-derived" after the wave and reaches seven review skills through stubs;
  four kits hold copies (`validation-constitution`, `review-governance-intent`, `review-brainstorm`,
  `review-specifications`). Ruled a § 7 build defect outside the closed list (same class as the three
  prose consumers S2 already landed): S1's rev 4 takes one `reword-rule` in 0020 to "A floor- or
  NFR-derived obligation is never excess, however heavy it looks." — matching S2's landed prose;
  the view diff gains the common view (thirteen files); seven review renders shrink. Supersession
  sent to S1 voiding the "no diff beyond these lines" clause of the rev 4 order. `checklist-assembly`
  stays: template-module fragments are engineering modules, which survive D1.
- **S3 v6 fix order:** stubs resolve from the landed common view · named set corrected to
  four + two (`checklist-assembly` dropped, `never-excess` in via the common reword) · write set
  widened by `review-brainstorm` and `review-specifications` `rules.json` + `rekey.md`, field-scoped
  to the one entry, their `evals.json` joining E8's grep (hits halt) · the `findings-through-leads-pen`
  drift repaired in the same re-key as a disclosed fix-on-sight integrity repair, fence amended,
  further drift halts · under-movement stop added, retired-term grep extended to the four kits'
  named entries. Counted as the addendum's round 1 — S3's second round on the shared counter, on
  lead-added scope after the v3 PASS, disclosed here rather than escalated: plan-only, repo
  untouched, and the lead's relay never told S3 which texts S1 moves. P3 re-grades v6 resumed.
- **S1 rev 4 final** (sha `19e49610…`; an intermediate `d3f133a8…` carried only the item 5 fix
  before the widening reached S1): § 8 bound paragraph (a)/(b)/(c) · +86/≈+19 figure with
  measure-before-claim · tidies · the `review-common.never-excess` reword placed in **0019 under the
  D1 anchor** (the lead's order said 0020 by slip; 0019 is D1 in this plan — placement confirmed) with
  a halt clause on a refused common-id reword (precedent 0009) · view diff thirteen files · seven
  review renders and the `skill-review-common.yaml` budget row shrink · § 6 disclosure · § 7 S3
  hand-off. Diff against rev 3: 64 lines, verified on disk. Re-grade order to P1, resumed.
- **S3 v6** (sha `2bd60a4e…`, 211-line diff against v5, verified on disk): all five rulings placed —
  stubs from the common view with `vars:` substituted and cross-checked against the render (S3's
  read-only `parity.py` at HEAD shows the kits carry substituted text) · named set four + two,
  `checklist-assembly` dropped · `review-brainstorm` and `review-specifications` kits joined,
  field-scoped, goldens in E8's grep · drift repair disclosed with old/new text · under-movement stop
  with the widened token list · § 9 records the rulings, old H4 line marked superseded. New
  observation **H6**, no action: `evals/review-specifications/rules.json` holds 30 ids against the
  view's 31 — `review-specifications.sf-direction-checks` (minted 0013) missing at HEAD. Ruled out
  of this wave (the kit is field-scoped to `never-excess` here); queued in `BACKLOG.md`. Re-grade
  order to P3, resumed.
- **P1 re-graded rev 4** (sha `19e49610…`, same seat resumed): **PASS** — item 5 closed by the three
  bounds; the `never-excess` reword verbatim against the common view (no class, kind or anchor on
  the block: no exit, no anchor needed; 0019/D1 placement correct); seven extenders confirmed by
  grep, skill views hold only the stub so five untouched review views stay byte-identical; 0009's
  `command-common/common` reword the precedent; the 1,627 budget row exists. Advisory: "three"
  untouched review skills is five; § 7 could name the common view diff in 0019's unit. Tiering:
  first-hand. **Lead approved; GO S1 sent** quoting the sha, with S2's H1–H9 as re-keyed by the
  rulings (H1 superseded by Q6, H5 done) and the return shape.

## 2026-09-24 · P3 FAIL(4) on S3 v6 · S1 landed · S2 complete · user ruled re-plan

- **P3 re-graded S3 v6** (sha `2bd60a4e…`, same seat resumed, own parity script): **FAIL(4)** only —
  items 1, 2, 3, 5, 6 PASS; stub source, `vars:` substitution and render cross-check verified sound
  (the render command needs `--plugin-root plugins/mochiko`). Item 4: the lead's ruling 2 named two
  `review-governance-intent` entries; S1's 0021 rewords four — `read-set-binding` and
  `user-facts-flagged` unnamed, so the parity check and drift stop would fire on every run and the
  judge would keep both retired terms. Root cause: the lead's named set, built from S2's H7 and
  P3's v5 finding, not S1's op list. Lead fact-check: the four kits with retired-term hits are S3's
  four; no other kit copies a rule S1 rewords. The addendum's second FAIL — reserved to the user by
  `review-seat-plan.approval-is-the-leads`; **user ruled re-plan** ("continue" on the lead's
  recommendation). v7 order: the two ids added; `--plugin-root` on the render command; two crate
  asserts S1's landing turned red join field-scoped — `fidelity.rs` `RETIRED_SIDECAR_ANCHORS` and
  the `validate.rs` pointer pin 84 → 82. P3 re-grades v7 resumed.
- **S1 landed** under GO (rev 4, sha `19e49610…`): migrations 0019–0023 stamped, every old text
  asserted against the live view before replacement. § 8 halt fired once —
  `patterns-plan-minimalism.floor-both-ways` (floor, anchored) still named compliance-module
  obligations; the lead's sweep of the landed views for every retired term found it the only live
  hit (the rest: the forward-only clause and supersession dispositions); ruled a § 7 defect,
  reworded in 0019 (16 changes), no prose consumer. Then `setup.md` step 2 and the
  `.mochiko/strips/setup.md` entry "[v0.115.0] Goal step — reconstruction, amend surfacing, and
  baselines sentences superseded". Lead verified first-hand: 0 rejecting · 113 advisory; 1135 rules
  (332/803); setup pins 20/6, sections 5/6/11/6/7/6; fourteen views, identical to a fresh replay;
  `authoring-architecture-store` byte-identical. Renders: `authoring-constitution` −275 (payload
  29,138 under 29,614), `validation-constitution` −2, `review-governance-intent` −47,
  `analysis-codebase` −236 (standing overage falls), `patterns-plan-minimalism` −32, five review
  skills −21 each. P1's two plan tidies applied post-GO. `cargo test`: nine failures, all in S3's
  files. Tiering: first-hand, disclosed.
- **S2 complete** (16/16 files, 19/19 entries): the lead relayed the H2/H3/H4/H8/H9 landed texts;
  QC-10 branch fired (semver line re-keyed, old line appended to VC-2); IA-12 numbered (1)–(6) to
  the ledger route (an addition, no strip); H4/H8/H9 no drift. Event (4) keeps the record's "under
  the dimension-9 test" qualifier. Spot-checked on disk by the lead.
- **S3 v7** (sha `8c7312af…`, 112-line diff against v6, verified on disk): the four-entry
  `review-governance-intent` set with the moved ids the GO must quote; `--plugin-root` on the render
  cross-check; `RETIRED_SIDECAR_ANCHORS` gains exactly the two sidecar-anchored retirements
  (`module-mechanical-attachment` 0019/D1, `setup.baselines-bootstrap` 0023/D5 — the 0014 mints are
  not in the sidecar); the pointer pin 84 → 82 read from the failure output (E2b, with
  `validate.rs`'s census); § 1 carries the landed state. Two of the lead's migration numbers
  corrected from the log by S3 (0019, not 0020/0023). **H7**, no action: three `observable.yaml`
  rationale lines sit on reworded ids (architecture :35/:51, specify :46) — ruled leave, ids
  survive and the rubric holds. Re-grade order to P3, resumed.
- **P3 re-graded v7** (same seat resumed; migrations, sidecar fixture and both crate sites checked
  first-hand): **PASS**, none failed; wording note — the GO list is nine moved ids, not ten, and
  E8b's stop compares against that list. Lead approved; **GO S3 sent** quoting the sha, the landed
  state (`156ab089…`) and the nine ids. S3 re-plan rounds: 2 on the shared counter (v2, v6), plus
  the user-ruled third (v7).

## 2026-09-24 · gate audit — G1 rounds 1–2, 23 units PASS · unit 7 pending S3

- G1 spawned plain `general-purpose` `model: opus`, the post-landing `validation-primitive-edit`
  render (11 floors) pasted verbatim; units: three command pairs, five schema-content units
  (migration + view diff), seven skill pairs, seven prose units; crate + evals (unit 7) deferred to
  S3's landing. G1 stopped once on the session limit before any write; resumed on reset with its
  context intact. Verdict blocks: `reports/gate-audit.md` (one envelope denial on a 19-line
  section, trimmed to fit — disclosed). Tiering: no explorer; first-hand at opus.
- **Round 1** — 21 PASS, 1 FAIL (0023: `impl.design-landing` still named the retired setup leg as
  a designer write path). Fix (S1, the one fix `common.gate-loop-bound` allows): one `reword-rule`
  appended to 0023 (19 changes; the rule keeps id, class, `when:` and its impeccable D7 anchor) and
  the dead allowlist edge on the tombstoned id deleted. Fifteen views. Lead verified: 0 rejecting,
  no `warning:` line, state `156ab089…`, views ≡ replay.
- **Round 2** — 0023 PASS, `implement` pair (new unit) PASS; 0 blocking. Advisory left: BE-DEP's
  missing blank line before **Content:**; "a fact-profile module" wording; the six-event list in
  both the region and the ledger (as before).
- Outcome lines:
  - audit: setup (command pair) · G1 · opus · 4 files · 1 rounds · 0 blocking
  - audit: specify (command pair) · G1 · opus · 3 files · 1 rounds · 0 blocking
  - audit: architecture (command pair) · G1 · opus · 4 files · 1 rounds · 0 blocking
  - audit: 0019-setup-agnostic-modules-out (schema content) · G1 · opus · 10 files · 1 rounds · 0 blocking
  - audit: 0020-setup-agnostic-rule-not-instance (schema content) · G1 · opus · 4 files · 1 rounds · 0 blocking
  - audit: 0021-setup-agnostic-seven-dimensions (schema content) · G1 · opus · 2 files · 1 rounds · 0 blocking
  - audit: 0022-setup-agnostic-closed-event-set (schema content) · G1 · opus · 3 files · 1 rounds · 0 blocking
  - audit: 0023-setup-agnostic-setup-rule-set (schema content) · G1 · opus · 4 files · 2 rounds · 0 blocking
  - audit: implement (command pair) · G1 · opus · 3 files · 1 rounds · 0 blocking
  - audit: authoring-constitution (skill pair) · G1 · opus · 3 files · 1 rounds · 0 blocking
  - audit: validation-constitution (skill pair) · G1 · opus · 3 files · 1 rounds · 0 blocking
  - audit: review-governance-intent (skill pair) · G1 · opus · 3 files · 1 rounds · 0 blocking
  - audit: analysis-codebase (skill pair) · G1 · opus · 3 files · 1 rounds · 0 blocking
  - audit: patterns-design-direction (skill pair) · G1 · opus · 2 files · 1 rounds · 0 blocking
  - audit: patterns-craft-floor (skill pair) · G1 · opus · 2 files · 1 rounds · 0 blocking
  - audit: patterns-plan-minimalism (skill pair) · G1 · opus · 2 files · 1 rounds · 0 blocking
  - audit: mochiko router (prose) · G1 · opus · 2 files · 1 rounds · 0 blocking
  - audit: release-gates module (prose) · G1 · opus · 2 files · 1 rounds · 0 blocking
  - audit: authoring-constitution references (prose) · G1 · opus · 7 files · 1 rounds · 0 blocking
  - audit: validation-constitution references (prose) · G1 · opus · 3 files · 1 rounds · 0 blocking
  - audit: TEST-GRAMMAR (prose) · G1 · opus · 2 files · 1 rounds · 0 blocking
  - audit: EXTERNAL-CLAIMS (prose) · G1 · opus · 2 files · 1 rounds · 0 blocking
  - audit: FEASIBILITY-LENS (prose) · G1 · opus · 2 files · 1 rounds · 0 blocking

## 2026-09-24 · S3 landed — crate census, template fixtures, evals re-keyed · contract suite running

- **S3 executed v7** (sha `8c7312af…`) — 21 files, the § 3 firm set exactly; no § 8 halt; E8 opened on
  the lead's confirmation that S2 is final. Crate: `fidelity.rs` (sequence list 1..23, census
  332/803/1135/264/119/36, the 0003 table 14 → 13 dropping the `setup.feature-map-brownfield` tuple,
  `RETIRED_SIDECAR_ANCHORS` 1 → 3), `validate.rs` (census; pointer pin 84 → 82 read from the failing
  test's `left: 82` first), `matrix_similar.rs` (command-family pin (332, 12_834, 0, 54), full corpus
  1135 / 178_230 / 0 / 184 — the two allowlisted pairs dropped by the `arch`/`spec` rewords, one
  `feat.user-reserved` pair returned, the removed allowlist row), six template fixtures recaptured
  (only 0019's template changes in the diffs; the other nine templates unmoved). Gates green:
  `cargo build --release` · `fmt --check` · `clippy -D warnings` · `cargo test --all` (16 binaries, 0
  failed) · `MOCHIKO_FULL_SIMILAR=1` sweep 0 clusters · `check-rubric setup` 33 observable · 8
  out-of-instrument · 41 total. `evals/contract/**` untouched: every `expected-skills.json` row equals
  the render (setup 20, specify 17).
- **E8b** (parity script with stubs resolved from the common view, `${var}` substituted, cross-checked
  against the render): four kits' `rules.json` — 11 `"rule":` lines changed, 0 other; id sets equal
  (26 · 35 · 30; `review-specifications` 30 vs view 31 = H6, not asserted), floors 14/16/9/8 equal the
  renders, tempts resolve; the `findings-through-leads-pen` drift repaired, old and new text in that
  kit's `rekey.md`; `never-excess` labels `[boundary]` carried from the common block, disclosed. Four
  `rekey.md` sections "## Re-key 2026-09-24 — setup-product-agnostic", additions only.
- **E7/E8 goldens** (§ 7 build defects, `expected_output` only, every assertion/prompt/tempts/fixture
  untouched): setup s1/s2 (empty index scaffolded, no product baselines by setup);
  `validation-constitution` g1/g3 (floor-accounting line; the ASV waiver re-keyed to the D2
  product-instance ground; module-stamp consistency dropped; the never-excess sentences to
  floor-derived); `review-governance-intent` g1–g3 (status criteria to the product-instance ground
  on GI-011/GI-012; the risk-posture and lifespan user-declared conflicts dropped; the GDPR/HIPAA
  module sentences dropped or re-keyed). Token grep over all twelve goldens: none. Open item ruled:
  g2's status sentence re-keyed to `critical-gaps` on the product-instance ground (E8 — a golden
  graded by the retired criteria), one sentence, before G1's unit. H6/H7 stay as ruled.
- **E4** `observable.yaml`: three struck ids removed; `blind-map-dispatch`, `feature-map-greenfield`,
  `map-never-overwrite` whys re-keyed; three pre-existing uncovered ids added as observable (rubric
  defect fixed on sight, disclosed). Tiering: no worker, no explorer, first-hand; scratch scripts
  read-only. Lead: crate gates re-run first-hand in the background; the contract suite (97 cases)
  running in the `claude-mochiko` sandbox, 39 passed at last read, none failed.
- Lead's first-hand crate gates: `cargo fmt --all --check` clean · `cargo clippy --all-targets -- -D warnings`
  clean · `cargo test --all` 16 binaries, 0 failed · `cargo audit` 31 dependencies, no findings.
- **G1 unit 7** (same seat, `rust-cli.md` lens for the crate, each kit's `rekey.md` precedent for the
  evals; `cargo test --all` 17 result lines ok, rubric OK 33/8/41, diff stat exactly 21 files,
  `evals/contract` clean, no `src`/`Cargo.*` change): **PASS**, 0 blocking. Pins equal the landed
  log's own figures; the sidecar fixture anchors exactly the two ids added; six fixtures equal the
  live render minus the `schemas:` trailer; `rules.json` parity by script 0 mismatches, floors equal;
  `rekey.md` additions-only; goldens `expected_output` only, no retired term, status grounds exist in
  the fixtures. Advisory: `validation-constitution` g3 reads the planted pci-dss waiver as a product
  instance — the loosest re-key in the set, defensible. Tiering: no explorer, first-hand at opus.
  - audit: crate tests + eval kits (unit 7) · G1 · opus · 21 files · 1 rounds · 0 blocking
  **Wave standing: 24 units graded, 24 PASS, 0 open blocking.**

## 2026-09-24 · wave 1 closed — v0.115.0 shipped

- **Contract suite** (`python3 evals/contract/run.py`, `claude-mochiko` sandbox, Claude Code 2.1.259,
  cargo 1.98.1, authenticated): `contract suite: 97/97 cases passed, 97 ran, 327 measurement(s)
  recorded and not asserted`, exit 0 — a full run, not filtered, not skipped. Started while S3
  executed (it reads nothing S3 owns; its binary built at preflight).
- **Release gates:** audits 24/24 PASS · strips recorded in ten files (`release-gates-module.md`
  new) · landing complete (below) · `CHANGELOG.md` entry `[0.115.0]` · `plugin.json` 0.114.0 → 0.115.0
  · `marketplace.json` synced · views ≡ replay re-checked at the bump under binary 0.2.0 ·
  `cargo test` 16 binaries 0 failed, fmt, clippy, audit clean · contract suite green. No crate source
  changed: no `mochiko-cli-v*` tag owed, 0.2.0 still suffices.
- **Landing (KM three-part move):** the `BACKLOG.md` build item → `.mochiko/archive/backlog-trail.md`
  (DONE 2026-09-24); the *Rehoming* brainstorm and the *Eval-kit drift* item stay open in the
  section · `ROADMAP.md` Production-only Next row: "build queued" → BUILT at v0.115.0 · index entry:
  build pending → BUILT, pointing here and at `reports/gate-audit.md` · `DECISIONS.md` row unchanged
  (the ruling landed at acceptance). Owed outside this build: kinako's and mochiko's own governance
  take the new agenda at their next amend (Q8); the rehoming brainstorm (OQ1); H6.
- **Disclosure line:** `floor: tripped · seats: S1 tech-lead · S2 tech-lead · S3 staff-engineer /
  P1 tech-lead · P2 tech-lead · P3 staff-engineer · G1 general-purpose (opus, explicit) · plans:
  S1:PASS(1) · S2:PASS(1) · S3:PASS(3)` — no seat dirty; S3's third round was the user's ruling at
  the second consumption (`review-seat-plan.approval-is-the-leads`), recorded above. Roster tiers:
  every persona seat at its pinned default (`opus`), no deviation; G1 pinned `opus` explicitly. Read
  tiering: no seat dispatched a haiku explorer this wave (each disclosed first-hand, grade-deciding
  or completeness-sensitive reads); no sonnet worker used. Transport: lead-relayed messaging, one
  pen per surface throughout; two supersessions issued (P1's hold, the rev 4 widening), both
  content-pinned. Commits suggested to the user, never run.
