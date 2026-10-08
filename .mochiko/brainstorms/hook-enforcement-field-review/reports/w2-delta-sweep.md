---
report: disclosure
wave: 2
seat: S4
subject: >-
  the delta-files sweep and rule supersessions — wave plan §3a/§3b; plan s4/plan-v3.md (409 lines, sha256
  1986314c…e2ec), P4 PASS at re_review_round_1, lead GO
corpus: >-
  the replayed state as views, .mochiko/schema-views/ at 91b3838 (a fresh branch-binary emit, diff -rq
  empty, 80 documents); a raw-log grep as cross-check
terms: >-
  delta · fold · appliable · in place · in-flight · FEAT-XXX — word-bounded, case-insensitive (FEAT-XXX
  exact)
counts: >-
  234 hits on 204 nodes (re-run at execution: identical; P4's independent sweep matched one for one) ·
  D6d-listed 27 rule + 10 document nodes · D6d-untouched 6 · missed in scope 9 rule + 8 document nodes
  · other-sense 144: 2 in (B1), 4 in by class, 2 in (R2), 136 stand
d6d_completeness: >-
  D6d was ruled Assumed complete until this sweep. The sweep, the lead's Q2 ruling (all 15 sites in) and
  the re-plan-1 class rulings discharge it, subject to the wave-3 gate audit's D6d-vs-sweep diff (RA8);
  this report does not clear itself
rulings_applied:
- 'Q1 no strip entries: schema content is recorded by the log'
- Q2 all 15 sites in; 0030-0031 unused
- >-
  Q4 the run lead pins at entry, before the sufficiency seat; the seat writes the pin; run-open restates
  it
- Q5 seed entries read proposed (<key>); the checkpoint card lists them apart
- Q6 a reword keeps every responsibility but what its ruling removes; each intent names what left
- Q7 a single-member epic write is keyed to that member's FEAT-XXX
- Q8 the brief clause in impl.baseline-entry-grammar
- Q9 sequential-ids reworded in 0026; RA5 (at execution) takes "no gaps" out of it
- 'R2 at execution: impl.sufficiency-report reworded in 0026 to name the run-open pin'
- >-
  field review D6 at execution (S3 census finding): the epic artifact-home reword in 0027 drops the implement-log
  clause
- >-
  census table ratified 2026-09-29 (.mochiko/decisions/2026-09-29-census-table-ratified.md): rows E1 and
  F1-F4 in the one 0025 re-cut; baseline markers in the bold **Name:** form (0025, 0026, 0029), the store
  in its own **Name**: form
- R8 flip by kind at sign-off (seams record :73-84)
- >-
  R9 a landed removal leaves a removed stub (2026-09-29-landed-removal-stub.md); table rows: the row reduced
  to id, kind, name and status removed, not drawn (A10, lead-confirmed)
- >-
  re-plan 1: classes (a)/(b)/(c); the grammar rule carries no per-store sentence until the relayed 0025
  re-cut
supplementary_sites:
- node: authoring-technical-requirements.store-write-at-sign-off
  doc: skills/authoring-technical-requirements
  found_by: a supplementary "package" read (Q2-14)
  disposition: >-
    0029 supersede (floor reservation, D7; P4 B2); successor authoring-technical-requirements.nfr-target-flips-at-sign-off
- node: authoring-technical-requirements.artifact-home
  doc: skills/authoring-technical-requirements
  found_by: each skill's artifact-home read (Q2-7)
  disposition: 0027 reword (floor), the D4 clause only — the feature and epic homes leave
- node: authoring-epic.artifact-home
  doc: skills/authoring-epic
  found_by: each skill's artifact-home read (Q2-8)
  disposition: >-
    0027 reword (floor): the D4 clause — the contracts/ sub-directory leaves — and, by the lead's ruling
    on S3's census finding, hook-enforcement-field-review D6-ephemeral-run-log — "implement-log.md is bounded per entry"
    leaves, the run log living in .mochiko/runs/<run-id>/
- node: review-sufficiency.lifecycle-marker-read
  doc: skills/review-sufficiency
  found_by: no reader of the markers existed (Q2-15)
  disposition: '0029 mint (must, inputs) — the reader sentence; orphan keys listed for the user'
- node: authoring-technical-requirements.sequential-ids
  doc: skills/authoring-technical-requirements
  found_by: raised at plan grade (P4 A9, Q9)
  disposition: >-
    0026 reword (lead ruling Q9) — the product file's own sequence at write, a removed stub's id taken;
    the 001 start leaves. RA5, lead-ruled at execution: "no gaps" leaves too (D3a) — a landed id is never
    reused, and a gap left when an abandoned run's reverted entries were minted over is legal; no stub
    for an abandoned run, its entries having been proposed
no_op_homes:
- >-
  review-sufficiency.absent-baseline and impl.design-absent-baseline-seed carry the graded fold's absent-file
  clause (A4); impl.baselines-in-place-marked creates the file in place
migrations:
  0024-delta-baselines-in-place: D1 · 2 supersede · 2 mint · 7 reword · 1 template replace (design-baseline)
  0025-delta-lifecycle-marker: >-
    D2 (R8, R9, Q8; re-cut after the census ratification: rows E1 spelling, F1-F4 placement, the per-store
    sentence, the spine Raised/Weighed columns) · 1 supersede · 1 mint · 6 reword · 2 template replace
    (architecture-store, architecture-spine)
  0026-delta-pinned-base-review: D3 (R2, R9, Q9, RA5, A4) · 3 supersede · 4 mint · 5 reword
  0027-delta-drawing-not-copies: >-
    D4 (+ field review D6 on the epic artifact-home reword) · 6 reword · 1 template replace (tasks, header
    and generated-from line)
  '0028-delta-epic-one-pen': D6b · 1 supersede · 3 reword
  '0029-delta-proposed-until-signoff': D7 (R8) · 2 supersede · 3 mint · 9 reword · 1 set-condition
  totals: >-
    9 supersede · 10 mint · 36 reword · 1 set-condition · 4 template replace · anchor 2026-09-24 delta-files-vs-direct-baseline-edits
    D<decision> on every header, supersede and mint
  floors: implement 4 out, 4 in (37 holds; render order moves) · store 1/1 · technical requirements 1/1
validation:
  scratch_final: >-
    target/debug/mochiko-cli migrate validate --report over the scratch log: 0 rejecting · 113 advisory
    (the same code mix as before; only budget counts and the enforces-coverage list moved); with scripts/similar-rules-allowlist.yaml
    in scope: clusters 0, allowlist-suppressed edges 184 to 183, no new unsuppressed edge
  scratch_incremental: >-
    through 0024: 5 rejecting (1 enforces-unresolved, 4 same-document cite-unresolved: forward cites to
    rules 0025-0026 mint); through 0025: 4; through 0026-0029: 0. The set lands whole
  landed: >-
    after the copy into plugins/mochiko/migrations/ (lead slot): target/debug/mochiko-cli migrate validate
    --report --plugin-root plugins/mochiko gives 0 rejecting · 113 advisory, clusters 0, allowlist-suppressed
    edges 183; migrate status gives grammar 1 · sequences 1..29 (29 migrations) · state sha256:6836ba966e8d10a4662ca338f5fbeccaa0132ac36dd34846e35f4b2921f16dbe
    · 80 documents · 1136 rules; after the 0025 re-cut (0025, 0026, 0029 replaced in a second lead slot):
    0 rejecting · 113 advisory, clusters 0, edges 183; status sequences 1..29 · state sha256:158a73a900405b4ccdff9415b6f04b9f71c2d0f96e0d8b98d2e0a222a873812f
  largest_section: impl.sec.tools renders 23,282 chars (was 17,892), under the 30,000 ceiling (tests/render.rs:1034)
view_diff: >-
  16 view files move: 3 commands (implement, feature, architecture), 9 skills, 4 templates. Rule level:
  9 ids out, 10 in, 36 texts changed, 1 condition re-set, tombstones added; nothing else moves (rule-by-rule
  diff of a scratch emit against the committed views)
pins_moved_for_S5:
- >-
  crates/mochiko-cli/tests/render.rs:1076-1113 IMPLEMENT_FLOORS (4 out: impl.graded-fold, impl.baselines-never-in-place,
  impl.fail.baseline-in-place, impl.fail.ungraded-fold; 4 in: impl.baseline-diff-review, impl.baselines-in-place-marked,
  impl.fail.unmarked-baseline-write, impl.fail.unreviewed-baseline-diff; render order)
- 'crates/mochiko-cli/tests/fidelity.rs: the sequence list and the counts (six files 0024-0029)'
- evals/contract/run.py:290-316
- evals/contract/expected-skills.json:96, :229
- evals/plan/implement/observable.yaml:64,75,97,128,131 (S5 set widened to evals/plan/**, A8)
advisories:
- >-
  RA1 the signed architecture.md drawing named where "design-phase deltas" left the fence, the dispatch
  mirror and clause10-carve (and clause-in-flight)
- >-
  RA2 in-flight-territory-read names the owning run's in-flight-class entries in the baselines and the
  store, and its drawing
- RA3 the set-condition row carries its rung (R3) on its own row
- >-
  RA4 clause-structural-trigger relabelled class (c); its dead `Derived from` reference noted on its row
- 'RA5 lead-ruled at execution: folded into the sequential-ids reword ("no gaps" leaves)'
- >-
  RA6 the lead's: the post-ratification 0025 re-cut re-runs stamp, validate and the view diff before S3
  regenerates views and S5 pins
- >-
  RA7 impl.baseline-diff-review's landing flip covers the store too, an NFR-only concern-row change included
  whether or not structure was built
- RA8 the D6d completeness line cites the Q2 ruling and defers to the gate audit
- RA9 plan-only (v3 frozen); this report points into no superseded plan
- RA10 noted on the impl.design-first-write row
- >-
  A2 flagged, not settled: impl.design-landing keeps "${designer_seat} is the only writer of its content"
  while the design-baseline template says who writes the truth part is open (setup-product-agnostic OQ1);
  the queued design-truth rehoming brainstorm owns it
stand_reasons:
  map: >-
    map, desk-card or delta-scope sense (map rows and status, map deltas, desk delta cards, delta scope);
    no ruling touches it
  id: FEAT-XXX as a path, id or pointer placeholder, not a lifecycle key; no ruling touches it
  signed: >-
    reads "the signed delta"; its direction is unchanged — impl.base-pins and the store gate successor
    define where the signed state is read (D3c, V1)
  arch: >-
    "delta" as the architecture change D4 keeps; the drawing and the store change still exist, their location
    carried by 0027/0029
  dfold: '"fold" as the design landing write I6 keeps'
  other: >-
    another sense — a review, brainstorm, finding or design-skill fold, a governance delta, a pen-holder
    routing, a tombstone, or "in place" in an unrelated sense
  class_a: class (a) candidate that routes or reserves on map status and reads no contract
  landing: >-
    names the class of landing folds, of which the map rows (impl.landing-selection), the gates (impl.gates-fold)
    and the design landing survive; directs no baseline fold; the label registry takes no reword op (README:117-118)
nodes:
  - { node: "feat.author-grader", doc: "commands/feature", terms: "delta", class: "d6d-listed", disposition: "0024 reword — the ledger item leaves" }
  - { node: "feat.delta-cards", doc: "commands/feature", terms: "delta,appliable", class: "d6d-listed", disposition: "0024 reword — the ledger item leaves; a known touch is named on the card" }
  - { node: "feat.product-surface", doc: "commands/feature", terms: "delta,fold", class: "d6d-listed", disposition: "0027 reword — \"delta files overwrite only via the graded fold\" leaves" }
  - { node: "impl.baseline-delta-grammar", doc: "commands/implement", terms: "delta,appliable,in place", class: "d6d-listed", disposition: "0025 supersede (D2); successor impl.baseline-entry-grammar" }
  - { node: "impl.baselines-never-in-place", doc: "commands/implement", terms: "in place,delta,fold,in-flight", class: "d6d-listed", disposition: "0024 supersede (floor, D1); successor impl.baselines-in-place-marked" }
  - { node: "impl.design-first-write", doc: "commands/implement", terms: "fold", class: "d6d-listed", disposition: "no op — no text names the ledger; I6 keeps the floor. Its \"the proposed write\" is plain English, not the lifecycle marker (RA10)" }
  - { node: "impl.design-landing", doc: "commands/implement", terms: "delta,fold", class: "d6d-listed", disposition: "0024 reword — the ledger route becomes a truth-part build-raised entry (A2)" }
  - { node: "impl.design-outputs-home", doc: "commands/implement", terms: "delta,appliable,FEAT-XXX", class: "d6d-listed", disposition: "0027 reword — outputs in place; the home keeps the signed drawing" }
  - { node: "impl.epic-shared-baseline-single-pen", doc: "commands/implement", terms: "delta", class: "d6d-listed", disposition: "0028 reword — one pen per file, EPIC-XXX" }
  - { node: "impl.fail.baseline-in-place", doc: "commands/implement", terms: "in place,delta", class: "d6d-listed", disposition: "0026 supersede (floor fail, D3); successor impl.fail.unmarked-baseline-write" }
  - { node: "impl.fail.ungraded-fold", doc: "commands/implement", terms: "fold", class: "d6d-listed", disposition: "0026 supersede (floor fail, D3); successor impl.fail.unreviewed-baseline-diff" }
  - { node: "impl.graded-fold", doc: "commands/implement", terms: "fold,delta", class: "d6d-listed", disposition: "0026 supersede (floor, D3); successor impl.baseline-diff-review" }
  - { node: "impl.landing-lane", doc: "commands/implement", terms: "delta,fold", class: "d6d-listed", disposition: "0024 reword — \"the graded folds\" leaves" }
  - { node: "impl.landing-verifier-folds", doc: "commands/implement", terms: "fold,delta", class: "d6d-listed", disposition: "0026 reword — checks the landing flip, a removing entry to its R9 stub" }
  - { node: "impl.reports-envelope", doc: "commands/implement", terms: "delta,fold,FEAT-XXX", class: "d6d-listed", disposition: "0027 reword — \"delta files overwrite only via the graded fold\" leaves" }
  - { node: "impl.verification-design-time-grades", doc: "commands/implement", terms: "delta", class: "d6d-listed", disposition: "0026 reword — the ledger's name leaves" }
  - { node: "authoring-architecture-store.fold-duty", doc: "skills/authoring-architecture-store", terms: "fold,in-flight,FEAT-XXX", class: "d6d-listed", disposition: "0025 reword — the landing flip (A3: the store's fold was never the retired three-way diff)" }
  - { node: "authoring-architecture-store.lifecycle-statuses", doc: "skills/authoring-architecture-store", terms: "in-flight,FEAT-XXX", class: "d6d-listed", disposition: "0025 reword — seven statuses, the run key set, the reader sentence, R9 stubs" }
  - { node: "authoring-architecture-store.orphan-rule", doc: "skills/authoring-architecture-store", terms: "in-flight,FEAT-XXX", class: "d6d-listed", disposition: "0025 reword — any open feature, epic or lane; readers per D2/M6" }
  - { node: "authoring-architecture-store.sign-off-is-write-gate", doc: "skills/authoring-architecture-store", terms: "delta,in place,in-flight", class: "d6d-listed", disposition: "0029 supersede (floor gate, D7); successor authoring-architecture-store.sign-off-flips-proposed" }
  - { node: "authoring-epic.member-deltas-stay-per-feature", doc: "skills/authoring-epic", terms: "delta,FEAT-XXX", class: "d6d-listed", disposition: "0028 supersede (D6b), no successor; manifest link lives on in manifest-required-fields" }
  - { node: "authoring-epic.shared-baseline-single-pen-holder", doc: "skills/authoring-epic", terms: "delta,fold", class: "d6d-listed", disposition: "0028 reword (floor) — in place, one pen per file, EPIC-XXX; a single member's write under its FEAT-XXX (Q7)" }
  - { node: "authoring-feature-map.map-side-altitude", doc: "skills/authoring-feature-map", terms: "delta,appliable", class: "d6d-listed", disposition: "0024 reword — \"appliable before/after\" leaves" }
  - { node: "patterns-adopt-first.baseline-delta-landing", doc: "skills/patterns-adopt-first", terms: "delta,in place", class: "d6d-listed", disposition: "0024 supersede (anchored, D1); successor patterns-adopt-first.build-time-entry-landing" }
  - { node: "review-plan-artifacts.store-delta-checklists", doc: "skills/review-plan-artifacts", terms: "delta", class: "d6d-listed", disposition: "no op — the register rows are prose at skills/review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md:85-98, wave 3" }
  - { node: "review-sufficiency.clause-in-flight", doc: "skills/review-sufficiency", terms: "in-flight,delta", class: "d6d-listed", disposition: "0029 reword — the reader sentence; the signed drawing read in place of the deltas" }
  - { node: "testing-gap-finding.blindness-fence-inclusion-list", doc: "skills/testing-gap-finding", terms: "delta,FEAT-XXX", class: "d6d-listed", disposition: "0029 reword (floor) — the signed drawing admitted in place of the deltas (RA1); built and the in-flight class admitted, never proposed" }
  - { node: "deliverables.4.file", doc: "homes/feature", terms: "delta", class: "d6d-listed (document)", disposition: "S3's: the feature home drops baseline-delta.md at the census (D4); not in S4's set" }
  - { node: "deliverables.1.file", doc: "homes/product-lane", terms: "delta", class: "d6d-listed (document)", disposition: "S3's: the lane home drops baseline-delta.md at the census (D4); not in S4's set" }
  - { node: "sections.1.check", doc: "templates/architecture-store", terms: "in-flight,FEAT-XXX", class: "d6d-listed (document)", disposition: "0025 replace — the run key on every keyed status, none on ruled/built/removed" }
  - { node: "sections.1.contract", doc: "templates/architecture-store", terms: "in-flight,FEAT-XXX", class: "d6d-listed (document)", disposition: "0025 replace — the legend: seven statuses, the key set, the flip, the stub" }
  - { node: "sections.2.contract", doc: "templates/architecture-store", terms: "FEAT-XXX", class: "d6d-listed (document)", disposition: "0025 replace — a row cut for removal and the removed stub; the Work pointers stand" }
  - { node: "sections.4.contract", doc: "templates/architecture-store", terms: "in-flight,FEAT-XXX", class: "d6d-listed (document)", disposition: "0025 replace — the health view's orphan line takes any open key" }
  - { node: "skeleton", doc: "templates/architecture-store", terms: "in-flight", class: "d6d-listed (document)", disposition: "0025 replace — diagram note; a removed row shown" }
  - { node: "overview", doc: "templates/design-baseline", terms: "delta,fold", class: "d6d-listed (document)", disposition: "0024 replace — the truth part's build-raised entry in place of the ledger path (A2)" }
  - { node: "sections.10.check", doc: "templates/design-baseline", terms: "FEAT-XXX", class: "d6d-listed (document)", disposition: "stands: FEAT-XXX names the feature that surfaced a divergence, not a lifecycle key" }
  - { node: "sections.10.contract", doc: "templates/design-baseline", terms: "fold,FEAT-XXX", class: "d6d-listed (document)", disposition: "stands: the Drift section is written by the design landing fold I6 keeps" }
  - { node: "impl.delta-reverification", doc: "commands/implement", terms: "delta", class: "d6d-untouched", disposition: "stands: on D6d's untouched list — its \"delta\" or \"fold\" is the map row or the desk card" }
  - { node: "impl.gates-fold", doc: "commands/implement", terms: "fold,FEAT-XXX", class: "d6d-untouched", disposition: "stands: on D6d's untouched list — its \"delta\" or \"fold\" is the map row or the desk card" }
  - { node: "impl.landing-delta", doc: "commands/implement", terms: "delta,fold", class: "d6d-untouched", disposition: "stands: on D6d's untouched list — its \"delta\" or \"fold\" is the map row or the desk card" }
  - { node: "authoring-feature-map.delivered-sticky-rows-fold", doc: "skills/authoring-feature-map", terms: "fold", class: "d6d-untouched", disposition: "stands: on D6d's untouched list — its \"delta\" or \"fold\" is the map row or the desk card" }
  - { node: "patterns-sound-loop.no-delta-card-exemption", doc: "skills/patterns-sound-loop", terms: "delta", class: "d6d-untouched", disposition: "stands: on D6d's untouched list — its \"delta\" or \"fold\" is the map row or the desk card" }
  - { node: "review-sufficiency.clause9-delta-inapplicable", doc: "skills/review-sufficiency", terms: "delta,fold", class: "d6d-untouched", disposition: "stands: on D6d's untouched list — its \"delta\" or \"fold\" is the map row or the desk card" }
  - { node: "arch.tools-store-skill", doc: "commands/architecture", terms: "fold,in-flight,FEAT-XXX", class: "missed-in-scope (Q2, ruled in)", disposition: "0025 reword (Q2-2) — the chain gains proposed and removed, the key set widens" }
  - { node: "impl.design-absent-baseline-seed", doc: "commands/implement", terms: "delta", class: "missed-in-scope (Q2, ruled in)", disposition: "0024 reword (Q2-1, Q5) — seed entries proposed, listed apart on the card" }
  - { node: "impl.design-inputs", doc: "commands/implement", terms: "delta,FEAT-XXX", class: "missed-in-scope (Q2, ruled in)", disposition: "0027 reword (Q2-6) — the drawing and the marked entries in place of the feature-dir deltas" }
  - { node: "impl.fail.store-landing-incomplete", doc: "commands/implement", terms: "delta,in-flight", class: "missed-in-scope (Q2, ruled in)", disposition: "0025 reword (Q2-4, floor fail) — any open key; a removal flips to its stub" }
  - { node: "impl.gap-finding-blind-dispatch", doc: "commands/implement", terms: "delta", class: "missed-in-scope (Q2, ruled in)", disposition: "0029 reword (Q2-12) — the dispatch mirror of the fence (RA1)" }
  - { node: "impl.gate-design-checkpoint", doc: "commands/implement", terms: "delta", class: "missed-in-scope (Q2, ruled in)", disposition: "0029 reword (Q2-11, floor gate) — the flip sentence and the sign-off pin" }
  - { node: "impl.store-landing", doc: "commands/implement", terms: "delta,fold,in-flight,FEAT-XXX", class: "missed-in-scope (Q2, ruled in)", disposition: "0025 reword (Q2-3) — the landing flip, any key" }
  - { node: "authoring-epic.transport-floor-binding", doc: "skills/authoring-epic", terms: "delta", class: "missed-in-scope (Q2, ruled in)", disposition: "0028 reword (Q2-10) — the shared baselines edited in place" }
  - { node: "review-sufficiency.clause10-carve", doc: "skills/review-sufficiency", terms: "delta,in-flight", class: "missed-in-scope (Q2, ruled in)", disposition: "0029 reword (Q2-13) — the signed drawing in place of the deltas (RA1)" }
  - { node: "overview", doc: "templates/architecture-spine", terms: "fold", class: "missed-in-scope (document)", disposition: "stands: the as-built notes are the store landing's (A3)" }
  - { node: "sections.1.check", doc: "templates/architecture-spine", terms: "in-flight", class: "missed-in-scope (document)", disposition: "0025 replace (Q2-5) — proposed and in-flight-class marked, removed not drawn" }
  - { node: "sections.1.contract", doc: "templates/architecture-spine", terms: "in-flight", class: "missed-in-scope (document)", disposition: "0025 replace (Q2-5) — the diagram contract: proposed and in-flight-class marked, removed not drawn" }
  - { node: "sections.4.contract", doc: "templates/architecture-spine", terms: "fold", class: "missed-in-scope (document)", disposition: "stands: the as-built notes are the store landing's (A3)" }
  - { node: "skeleton", doc: "templates/architecture-spine", terms: "fold,in-flight", class: "missed-in-scope (document)", disposition: "0025 replace (Q2-5) — diagram note; a removed row shown" }
  - { node: "sections.0.check", doc: "templates/tasks", terms: "FEAT-XXX", class: "missed-in-scope (document)", disposition: "stands: the FEAT-XXX title placeholder" }
  - { node: "sections.0.contract", doc: "templates/tasks", terms: "delta,FEAT-XXX", class: "missed-in-scope (document)", disposition: "0027 replace (Q2-9) — the header cites the drawing and the marked entries" }
  - { node: "skeleton", doc: "templates/tasks", terms: "delta,FEAT-XXX", class: "missed-in-scope (document)", disposition: "0027 replace (Q2-9) — the generated-from line cites the drawing and the marked entries" }
  - { node: "arch.author-grader-separation", doc: "commands/architecture", terms: "delta", class: "other-sense", disposition: "stands (arch)" }
  - { node: "arch.no-delivery-harness", doc: "commands/architecture", terms: "delta", class: "other-sense", disposition: "stands (arch)" }
  - { node: "arch.seat-architect-producer", doc: "commands/architecture", terms: "delta", class: "other-sense", disposition: "stands (arch)" }
  - { node: "arch.tools-system-design", doc: "commands/architecture", terms: "delta", class: "other-sense", disposition: "stands (arch)" }
  - { node: "brainstorm.coverage-survivor-routing", doc: "commands/brainstorm", terms: "fold", class: "other-sense", disposition: "stands (other)" }
  - { node: "brainstorm.non-coverage-survivors", doc: "commands/brainstorm", terms: "fold", class: "other-sense", disposition: "stands (other)" }
  - { node: "feat.artifact-home", doc: "commands/feature", terms: "FEAT-XXX", class: "other-sense", disposition: "stands (id)" }
  - { node: "feat.dispatch-scope-split", doc: "commands/feature", terms: "delta,fold", class: "other-sense", disposition: "stands (map)" }
  - { node: "feat.dm-map-integrity", doc: "commands/feature", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "feat.dm-route-honestly", doc: "commands/feature", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "feat.feature-map-binding", doc: "commands/feature", terms: "delta,fold", class: "other-sense", disposition: "stands (map)" }
  - { node: "feat.lane-never-widens", doc: "commands/feature", terms: "in place,in-flight", class: "other-sense", disposition: "stands (class_a)" }
  - { node: "feat.map-files", doc: "commands/feature", terms: "FEAT-XXX", class: "other-sense", disposition: "stands (id)" }
  - { node: "feat.no-delivery-harness", doc: "commands/feature", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "feat.sound-loop-floor", doc: "commands/feature", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "feat.stable-ground-triage", doc: "commands/feature", terms: "delta,in-flight", class: "other-sense", disposition: "stands (class_a)" }
  - { node: "conditions.scope.values", doc: "commands/implement", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "impl.artifact-home", doc: "commands/implement", terms: "FEAT-XXX", class: "other-sense", disposition: "stands (id)" }
  - { node: "impl.delivered-territory-routing", doc: "commands/implement", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "impl.design-audit-advisory", doc: "commands/implement", terms: "fold", class: "other-sense", disposition: "stands (dfold)" }
  - { node: "impl.design-finish", doc: "commands/implement", terms: "fold", class: "other-sense", disposition: "stands (dfold)" }
  - { node: "impl.design-map-assertion", doc: "commands/implement", terms: "delta", class: "other-sense", disposition: "stands (signed); also writes a marked map delta (map sense)." }
  - { node: "impl.design-seats-staffing", doc: "commands/implement", terms: "delta", class: "other-sense, ruled in", disposition: "in (B1): 0024 reword (D1) — \"design deltas\" / \"a store delta\" become the design-phase baseline writes, the store's proposed elements and the run's drawing" }
  - { node: "impl.deviation-gate", doc: "commands/implement", terms: "delta", class: "other-sense", disposition: "stands (signed)" }
  - { node: "impl.epic-card-sequence", doc: "commands/implement", terms: "FEAT-XXX", class: "other-sense", disposition: "stands (id)" }
  - { node: "impl.fail.design-skipped", doc: "commands/implement", terms: "delta", class: "other-sense", disposition: "stands (signed)" }
  - { node: "impl.fail.skip-unstated", doc: "commands/implement", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "impl.gap-finding-scope", doc: "commands/implement", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "impl.landing-selection", doc: "commands/implement", terms: "fold", class: "other-sense", disposition: "stands (map)" }
  - { node: "impl.lane-never-widens", doc: "commands/implement", terms: "in place,in-flight", class: "other-sense", disposition: "stands (class_a)" }
  - { node: "impl.midrun-refire", doc: "commands/implement", terms: "delta", class: "other-sense", disposition: "stands (signed)" }
  - { node: "impl.regression-sweep", doc: "commands/implement", terms: "FEAT-XXX", class: "other-sense", disposition: "stands (id)" }
  - { node: "impl.sufficiency-binding-verdict", doc: "commands/implement", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "impl.sufficiency-report", doc: "commands/implement", terms: "delta", class: "other-sense, ruled in", disposition: "in (seam R2, lead ruling at execution): 0026 reword — implement's own list of what the report carries names the run-open pin and the pre-existing list" }
  - { node: "impl.user-runopen-rulings", doc: "commands/implement", terms: "in-flight", class: "other-sense", disposition: "stands (class_a)" }
  - { node: "moments.design-checkpoint", doc: "commands/implement", terms: "delta", class: "other-sense", disposition: "stands (signed)" }
  - { node: "moments.landing", doc: "commands/implement", terms: "fold", class: "other-sense", disposition: "stands (landing)" }
  - { node: "setup.design-scaffold-unconditional", doc: "commands/setup", terms: "fold", class: "other-sense", disposition: "stands (dfold)" }
  - { node: "moments.derivation", doc: "commands/specify", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "spec.artifact-home", doc: "commands/specify", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "spec.feature-map-craft", doc: "commands/specify", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "spec.reserved-to-user", doc: "commands/specify", terms: "fold", class: "other-sense", disposition: "stands (other)" }
  - { node: "spec.stress-test-one-pass", doc: "commands/specify", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "subdirs.3", doc: "homes/spec", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "home", doc: "homes/spec-map-delta", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "path.3", doc: "homes/spec-map-delta", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "title", doc: "homes/spec-map-delta", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "labels.landing", doc: "labels/command-labels", terms: "fold", class: "other-sense", disposition: "stands (landing)" }
  - { node: "analysis-codebase.design-facts-seed", doc: "skills/analysis-codebase", terms: "fold", class: "other-sense", disposition: "stands (dfold)" }
  - { node: "authoring-architecture-store.diagram-craft-routing", doc: "skills/authoring-architecture-store", terms: "delta", class: "other-sense", disposition: "stands (arch)" }
  - { node: "authoring-architecture-store.diff-both-directions", doc: "skills/authoring-architecture-store", terms: "delta", class: "other-sense", disposition: "stands (signed)" }
  - { node: "authoring-architecture-store.landing-diff-on-delta", doc: "skills/authoring-architecture-store", terms: "delta", class: "other-sense", disposition: "stands (signed)" }
  - { node: "authoring-architecture-store.work-pointers-only", doc: "skills/authoring-architecture-store", terms: "FEAT-XXX", class: "other-sense", disposition: "stands (id)" }
  - { node: "conditions.delta.note", doc: "skills/authoring-architecture-store", terms: "delta", class: "other-sense", disposition: "stands (signed)" }
  - { node: "authoring-constitution.amend-preserves-verbatim", doc: "skills/authoring-constitution", terms: "in place", class: "other-sense", disposition: "stands (other)" }
  - { node: "authoring-epic.close-semantics", doc: "skills/authoring-epic", terms: "fold,in place", class: "other-sense", disposition: "stands (map)" }
  - { node: "authoring-epic.delta-batching-parked", doc: "skills/authoring-epic", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "authoring-epic.identity-grammar", doc: "skills/authoring-epic", terms: "FEAT-XXX", class: "other-sense", disposition: "stands (id)" }
  - { node: "authoring-epic.manifest-required-fields", doc: "skills/authoring-epic", terms: "FEAT-XXX", class: "other-sense", disposition: "stands (id); keeps the manifest link of the superseded member-deltas rule." }
  - { node: "authoring-epic.one-signed-store-delta", doc: "skills/authoring-epic", terms: "delta", class: "other-sense", disposition: "stands (signed)" }
  - { node: "authoring-epic.selection-scope-only", doc: "skills/authoring-epic", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "authoring-epic.transient-role", doc: "skills/authoring-epic", terms: "fold", class: "other-sense", disposition: "stands (map)" }
  - { node: "authoring-feature-map.acceptance-batch", doc: "skills/authoring-feature-map", terms: "fold,in-flight", class: "other-sense", disposition: "stands (map)" }
  - { node: "authoring-feature-map.artifact-home", doc: "skills/authoring-feature-map", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "authoring-feature-map.four-touchpoints", doc: "skills/authoring-feature-map", terms: "fold", class: "other-sense", disposition: "stands (map)" }
  - { node: "authoring-feature-map.growth-rides-work-row", doc: "skills/authoring-feature-map", terms: "in place", class: "other-sense", disposition: "stands (map)" }
  - { node: "authoring-feature-map.in-flight-territory-read", doc: "skills/authoring-feature-map", terms: "in-flight", class: "other-sense, ruled in", disposition: "in (B1): 0029 reword (D7) — names where the planned contract is read: the owning run's in-flight-class entries in the baselines and the store, and its signed architecture.md drawing (RA2)" }
  - { node: "authoring-feature-map.integrity-fix-on-sight", doc: "skills/authoring-feature-map", terms: "fold,in-flight", class: "other-sense", disposition: "stands (map)" }
  - { node: "authoring-feature-map.map-owns-status", doc: "skills/authoring-feature-map", terms: "in-flight", class: "other-sense", disposition: "stands (map)" }
  - { node: "authoring-feature-map.one-living-map", doc: "skills/authoring-feature-map", terms: "FEAT-XXX", class: "other-sense", disposition: "stands (id)" }
  - { node: "authoring-feature-map.retired-terminal", doc: "skills/authoring-feature-map", terms: "fold", class: "other-sense", disposition: "stands (map)" }
  - { node: "authoring-feature-map.selection-card", doc: "skills/authoring-feature-map", terms: "fold", class: "other-sense", disposition: "stands (map)" }
  - { node: "authoring-feature-map.work-rows-transient", doc: "skills/authoring-feature-map", terms: "fold", class: "other-sense", disposition: "stands (map)" }
  - { node: "vars.artifact", doc: "skills/authoring-feature-map", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "authoring-technical-requirements.entangled-decision-route", doc: "skills/authoring-technical-requirements", terms: "delta", class: "other-sense", disposition: "stands (arch)" }
  - { node: "authoring-technical-requirements.no-topology-decisions", doc: "skills/authoring-technical-requirements", terms: "delta", class: "other-sense", disposition: "stands (arch)" }
  - { node: "patterns-architecture-shelves.not-per-feature-design", doc: "skills/patterns-architecture-shelves", terms: "delta", class: "other-sense", disposition: "stands (arch)" }
  - { node: "patterns-craft-floor.sec.discipline", doc: "skills/patterns-craft-floor", terms: "fold", class: "other-sense", disposition: "stands (other)" }
  - { node: "patterns-design-direction.checklist-walked", doc: "skills/patterns-design-direction", terms: "fold", class: "other-sense", disposition: "stands (other)" }
  - { node: "patterns-design-direction.direction-block", doc: "skills/patterns-design-direction", terms: "fold", class: "other-sense", disposition: "stands (other)" }
  - { node: "patterns-design-direction.sec.discipline", doc: "skills/patterns-design-direction", terms: "fold", class: "other-sense", disposition: "stands (other)" }
  - { node: "patterns-map-minimalism.merge-preserves-mechanics", doc: "skills/patterns-map-minimalism", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "patterns-map-minimalism.vocabulary-boundary", doc: "skills/patterns-map-minimalism", terms: "fold", class: "other-sense", disposition: "stands (map)" }
  - { node: "patterns-model-tiering.rostered-seats-never-retier", doc: "skills/patterns-model-tiering", terms: "fold", class: "other-sense", disposition: "stands (other)" }
  - { node: "patterns-plan-minimalism.not-for-routes", doc: "skills/patterns-plan-minimalism", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "patterns-sound-loop.default-seat-wiring", doc: "skills/patterns-sound-loop", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "patterns-sound-loop.governing-surface-table", doc: "skills/patterns-sound-loop", terms: "delta", class: "other-sense, ruled in", disposition: "in by class (b): 0029 reword — the store row's \"design-phase deltas\" become a design phase's proposed writes" }
  - { node: "patterns-sound-loop.map-review-spec-less", doc: "skills/patterns-sound-loop", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "patterns-transport-floor.composition-steer", doc: "skills/patterns-transport-floor", terms: "delta", class: "other-sense", disposition: "stands (other)" }
  - { node: "patterns-vertical-tdd.walking-skeleton-first", doc: "skills/patterns-vertical-tdd", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "conditions.pass.note", doc: "skills/review-brainstorm", terms: "fold", class: "other-sense", disposition: "stands (other)" }
  - { node: "review-brainstorm.verify-pass-grade", doc: "skills/review-brainstorm", terms: "fold", class: "other-sense", disposition: "stands (other)" }
  - { node: "review-design-audit.baseline-divergence-drift", doc: "skills/review-design-audit", terms: "fold", class: "other-sense", disposition: "stands (dfold)" }
  - { node: "conditions.store_delta.note", doc: "skills/review-feasibility", terms: "delta", class: "other-sense, ruled in", disposition: "in by class (c): 0029 set-condition — values and resolution kept; the note names the run's proposed store changes, drawn in architecture.md and read in the pinned-base diff. Rung R3 (already exists; re-set, not minted) (RA3)" }
  - { node: "review-feasibility.architecture-pass-gate", doc: "skills/review-feasibility", terms: "delta", class: "other-sense, ruled in", disposition: "in by class (c): 0029 reword — \"a drafted store delta\" becomes the run's proposed store changes" }
  - { node: "review-plan-artifacts.design-cross-checklists", doc: "skills/review-plan-artifacts", terms: "delta", class: "other-sense", disposition: "stands (signed)" }
  - { node: "review-plan-artifacts.tier1-preassert", doc: "skills/review-plan-artifacts", terms: "fold", class: "other-sense", disposition: "stands (other)" }
  - { node: "review-specifications.feature-critical-checks", doc: "skills/review-specifications", terms: "delta,in-flight", class: "other-sense", disposition: "stands (class_a); reads the owning spec, which no ruling moves." }
  - { node: "conditions.scope.note", doc: "skills/review-sufficiency", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "conditions.scope.values", doc: "skills/review-sufficiency", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "review-sufficiency.clause-delivered-feature", doc: "skills/review-sufficiency", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "review-sufficiency.clause-structural-trigger", doc: "skills/review-sufficiency", terms: "delta", class: "other-sense, ruled in", disposition: "in by class (c), relabelled from (b) at P4 RA4: 0029 reword — elements keyed to the row's own feature or epic, proposed or in-flight class; the rule's `Derived from` named a field no store grammar carries (review-sufficiency.yaml:130 its only occurrence), so the key-based text also mends a dead reference" }
  - { node: "review-sufficiency.fence-never-reads", doc: "skills/review-sufficiency", terms: "FEAT-XXX", class: "other-sense", disposition: "stands (id)" }
  - { node: "review-sufficiency.fence-read-set", doc: "skills/review-sufficiency", terms: "FEAT-XXX", class: "other-sense", disposition: "stands (id)" }
  - { node: "review-sufficiency.na-justified", doc: "skills/review-sufficiency", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "review-sufficiency.report-binding", doc: "skills/review-sufficiency", terms: "delta,in-flight,FEAT-XXX", class: "other-sense, ruled in", disposition: "in (seam R2): 0026 reword — the run-open base commit and the pre-existing list join the report" }
  - { node: "review-sufficiency.sec.scope", doc: "skills/review-sufficiency", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "review-sufficiency.unit-and-collapse", doc: "skills/review-sufficiency", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "conditions.run_scope.values", doc: "skills/testing-gap-finding", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "testing-gap-finding.critique-finding-split", doc: "skills/testing-gap-finding", terms: "fold", class: "other-sense", disposition: "stands (dfold)" }
  - { node: "testing-gap-finding.delta-lane-skip-stated", doc: "skills/testing-gap-finding", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "testing-gap-finding.fold-back-authorship", doc: "skills/testing-gap-finding", terms: "fold", class: "other-sense", disposition: "stands (other)" }
  - { node: "testing-gap-finding.gates-artifact-contract", doc: "skills/testing-gap-finding", terms: "fold,FEAT-XXX", class: "other-sense", disposition: "stands (other); the gates fold, D6d-untouched sense (impl.gates-fold)." }
  - { node: "testing-gap-finding.out-of-territory-routing", doc: "skills/testing-gap-finding", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "testing-gap-finding.reviewer-mirror-checklist", doc: "skills/testing-gap-finding", terms: "fold", class: "other-sense", disposition: "stands (other)" }
  - { node: "testing-gap-finding.sec.independence", doc: "skills/testing-gap-finding", terms: "fold", class: "other-sense", disposition: "stands (other)" }
  - { node: "testing-gap-finding.sec.output", doc: "skills/testing-gap-finding", terms: "fold", class: "other-sense", disposition: "stands (other)" }
  - { node: "testing-gap-finding.test-grammar-consumed-never-redefined", doc: "skills/testing-gap-finding", terms: "fold", class: "other-sense", disposition: "stands (other)" }
  - { node: "overview", doc: "templates/feature-entry", terms: "fold,FEAT-XXX", class: "other-sense", disposition: "stands (map)" }
  - { node: "sections.0.check", doc: "templates/feature-entry", terms: "in-flight,FEAT-XXX", class: "other-sense", disposition: "stands (map)" }
  - { node: "sections.0.contract", doc: "templates/feature-entry", terms: "in-flight", class: "other-sense", disposition: "stands (map)" }
  - { node: "sections.2.contract", doc: "templates/feature-entry", terms: "fold", class: "other-sense", disposition: "stands (map)" }
  - { node: "sections.3.check", doc: "templates/feature-entry", terms: "fold", class: "other-sense", disposition: "stands (map)" }
  - { node: "sections.3.contract", doc: "templates/feature-entry", terms: "fold", class: "other-sense", disposition: "stands (map)" }
  - { node: "sections.7.contract", doc: "templates/feature-entry", terms: "FEAT-XXX", class: "other-sense", disposition: "stands (map)" }
  - { node: "skeleton", doc: "templates/feature-entry", terms: "in-flight", class: "other-sense", disposition: "stands (map)" }
  - { node: "overview", doc: "templates/features-index", terms: "fold", class: "other-sense", disposition: "stands (map)" }
  - { node: "sections.0.contract", doc: "templates/features-index", terms: "in-flight,FEAT-XXX", class: "other-sense", disposition: "stands (map)" }
  - { node: "sections.1.contract", doc: "templates/features-index", terms: "fold,in-flight,FEAT-XXX", class: "other-sense", disposition: "stands (map)" }
  - { node: "skeleton", doc: "templates/features-index", terms: "in-flight,FEAT-XXX", class: "other-sense", disposition: "stands (map)" }
  - { node: "overview", doc: "templates/governance-intent", terms: "delta,fold,in place", class: "other-sense", disposition: "stands (other)" }
  - { node: "sections.11.check", doc: "templates/governance-intent", terms: "delta", class: "other-sense", disposition: "stands (other)" }
  - { node: "sections.11.contract", doc: "templates/governance-intent", terms: "delta,fold", class: "other-sense", disposition: "stands (other)" }
  - { node: "sections.12.check", doc: "templates/governance-intent", terms: "delta", class: "other-sense", disposition: "stands (other)" }
  - { node: "sections.12.contract", doc: "templates/governance-intent", terms: "delta", class: "other-sense", disposition: "stands (other)" }
  - { node: "sections.7.check", doc: "templates/governance-intent", terms: "delta", class: "other-sense", disposition: "stands (other)" }
  - { node: "sections.7.contract", doc: "templates/governance-intent", terms: "delta", class: "other-sense", disposition: "stands (other)" }
  - { node: "skeleton", doc: "templates/governance-intent", terms: "delta", class: "other-sense", disposition: "stands (other)" }
  - { node: "sections.0.contract", doc: "templates/governance-surfaces", terms: "in place", class: "other-sense", disposition: "stands (other)" }
  - { node: "sections.2.contract", doc: "templates/governance-surfaces", terms: "delta", class: "other-sense", disposition: "stands (other)" }
  - { node: "sections.2.contract", doc: "templates/spec", terms: "FEAT-XXX", class: "other-sense", disposition: "stands (map)" }
  - { node: "sections.7.contract", doc: "templates/spec", terms: "FEAT-XXX", class: "other-sense", disposition: "stands (map)" }
  - { node: "sections.8.contract", doc: "templates/spec", terms: "delta", class: "other-sense", disposition: "stands (map)" }
  - { node: "skeleton", doc: "templates/spec", terms: "delta,FEAT-XXX", class: "other-sense", disposition: "stands (map)" }
---

## Notes of note

- Beyond the plan text, accepted by the lead as R8's direct consequence: the store template's
  concern ledger also exempts a row cut for removal before landing (`proposed` or `removing`, body
  cut) from the stance field; the plan named only the `removed` stub.
- Lead rulings at execution: in 0026 `sequential-ids` drops "no gaps" (RA5) and
  `impl.sufficiency-report` names the run-open pin (R2); in 0027 the epic artifact-home reword
  also drops the implement-log clause (field review D6). Reword total 35 to 36.
- "A landed id is never reused" scopes the lead's "an id is never reused": a reverted `proposed`
  id is invisible to the next high-water read, so the unscoped form cannot be followed.
- implement's resolved rule text grows 23,710 to 29,543 characters (budget advisory, no cap).
- Intermediate log states reject (5, then 4); only the six files together validate. S3's phase-A
  reads must see all six or none.
- The home documents still list baseline-delta.md and the epic's baseline copies until S3's
  phase B; the rewords here already describe the post-census sets.
