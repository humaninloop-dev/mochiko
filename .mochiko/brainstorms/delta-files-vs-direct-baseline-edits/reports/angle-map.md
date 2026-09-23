---
report: review
round: 0
session: delta-files-vs-direct-baseline-edits
phase: blind angle map (Phase 0, review-brainstorm.blind-map-before-record-contact)
reviewer: cold-reviewer — mochiko:devils-advocate seat, mochiko:review-brainstorm, solo, cold pass
record_contact: >-
  none — read, listed and searched nothing under .mochiko/brainstorms/ in either repo, index
  included; no search for this session's decisions
handed_back: 2026-09-23, message 1 of the two-message blind dispatch; reproduced here exactly as returned
floor_readback: >-
  9 floors. never-in-the-room · blind-map-before-record-contact · author-grader ·
  contested-needs-new-angle · never-default-ready · unverifiable-claim-is-finding · evidence-floor ·
  verdict-is-input · findings-through-leads-pen
routing_disclosure: >-
  I dispatched three Explore haiku seats. Two died at the SubagentHandback hook and one hit a 429
  rate limit, so none delivered. I took the locate/enumerate reads first-hand, as your fallback
  allowed.
grounding: >-
  plugin 0.114.0 working tree — migrations 0001-genesis.yaml (impl.* landing, boundaries and fail
  rules; authoring-architecture-store lifecycle, sign-off and fold rules; authoring-epic;
  patterns-transport-floor; authoring-feature-map), 0005-artifact-homes.yaml (feature, product,
  product-lane homes), grep hits in 0003/0009/0012/0013/0014/0018; hooks.json; BACKLOG.md audit item
  (2026-09-23) lines 40-55 and 865-880; DECISIONS.md row 13 and the rows naming the prior delta
  rulings. kinako — .mochiko/product/ line counts, FEAT-001/002/006 file lists, the FEAT-001
  ledger preamble and renumbering ledger, the run-4 landing fold text header, git log incl.
  commits 341bbb8, 2004abe, 0d87633, 5167f36, 76179e1.
legend: >-
  LB marks a load-bearing angle: a record that misses it is likely a Critical or Important
  coverage gap.
angle_count: 47
load_bearing_count: 30
class_counts:
  A-premise-and-evidence: 6
  B-rejected-roads: 7
  C-purpose-survival: 7
  D-scenario-stress: 7
  E-migration: 5
  F-cross-surface: 8
  G-governance-and-build: 5
  H-excess-watch: 2
  total: 47
angles:
  A-premise-and-evidence:
    - >-
      A1 (LB) Quantified cost, not asserted. The ledgers are FEAT-001 2314, FEAT-002 4744 and
      FEAT-006 1010 lines, against a 300-line bound. There are 237 BD entries, plus fold-review
      FAIL rounds in runs 3 and 4.
    - >-
      A2 (LB) Fold debt. The FEAT-001 ledger header says no BD entry has ever folded, so the
      baselines already lag built code. That makes "present-tense truth" false in practice today.
    - >-
      A3 (LB) Root-cause split. How much of the run 3/4 pain came from the delta model, and how much
      from the budget-deny gate? The product data-model.md is 1206 lines and C&D is 733, over their
      300 bound, so the fold was denied and the user ran APPLY.sh. Attributing gate pain to deltas
      would be a misattribution.
    - >-
      A4 (LB) Direct edits already happen. Commits 341bbb8 (B126) and 2004abe (B129) edited product
      contracts in place outside /mochiko:implement. The never-in-place floor is scoped to `impl.`
      only, so two regimes already coexist.
    - >-
      A5 Present-tense purity is already breached. Commit 0d87633 put accepted-but-unverified status
      into the baselines. Dated "## The FEAT-001 landing fold" blocks are appended at
      data-model.md:1080 and C&D:547.
    - >-
      A6 (LB) Does git actually carry the history? `impl.no-git-mutations` forbids mochiko from
      committing, so commit boundaries belong to the user. The diff base a grade needs is not
      guaranteed to exist as a commit.
  B-rejected-roads:
    - >-
      B1 (LB) Status quo, repaired. Keep the deltas and fix only execution: the budget gate, fold
      cadence, who holds the pen, and folding per run instead of never.
    - >-
      B2 Direct edit on the working tree with no markers.
    - >-
      B3 (LB) Direct edit with in-place lifecycle markers per entity or row, such as
      `in-flight (FEAT-XXX)`. The architecture store already works this way.
    - >-
      B4 (LB) Branch per feature. The branch holds proposals, the merge is the fold, and the PR diff
      is the sign-off. Kinako already works in PR branches, but mochiko cannot create branches.
    - >-
      B5 Hybrid by altitude. Build-time transcription edits go direct, and design-time structural
      proposals keep deltas.
    - >-
      B6 Hybrid by artifact. contracts/ goes direct, prose baselines keep deltas, and the store keeps
      its own lifecycle.
    - >-
      B7 Landing-time direct edit. Deltas stay as scratch, the landing edits the baselines, and the
      graded check is a diff against a run-open base. The ledger grammar drops.
  C-purpose-survival:
    - >-
      C1 (LB) Unbuilt proposals. Where does a signed but unbuilt design live? What does each reader
      see as truth mid-run: the sufficiency seat, the designer, another feature's builder?
    - >-
      C2 (LB) Graded high-blast write. What gets graded, when, and by whom? Who pins the diff base
      without git mutations?
    - >-
      C3 (LB) One pen. How do concurrent builders and epic members stay isolated on a shared
      baseline? Relevant rules are patterns-transport-floor.composition-steer and
      authoring-epic.shared-baseline-single-pen-holder.
    - >-
      C4 (LB) Sign-off surface. The design checkpoint signs a delta file today. Under direct edit,
      does the user sign a diff or a rendered view? How does this interact with the store's
      sign-off-is-write-gate carve?
    - >-
      C5 (LB) Id sequences. The fold renumbers feature-local ids (run 4: C-001 became C-009, D-001
      became D-050). Direct edit mints product ids at write time, so concurrent features race for
      the high-water mark. Collisions already happened, and FEAT-001 carries a renumbering ledger.
    - >-
      C6 (LB) Rationale home. Each BD entry records "what was weighed". Where does that go: a commit
      message mochiko cannot write, the baseline row, a report, or .mochiko/decisions/?
    - >-
      C7 (LB) Deviation gate and built-vs-signed diff. Both need a frozen signed artifact, and direct
      edits move the diff target.
  D-scenario-stress:
    - >-
      D1 (LB) A feature is abandoned or rejected after baselines were edited. What is the rollback,
      with no git mutations and mixed commits?
    - >-
      D2 The build diverges from a design already written into the baseline. Who reverts the
      unbuilt truth?
    - >-
      D3 (LB) Concurrent in-flight features. Feature B reads a baseline carrying A's unbuilt edits.
      Today review-sufficiency.clause-in-flight reads A's deltas; under direct edit, B cannot tell
      built from unbuilt.
    - >-
      D4 Scope. Does the rule govern every writer (fixes, the desk, /mochiko:feature,
      /mochiko:architecture) or only implement?
    - >-
      D5 Epic joint deltas. What replaces the spine's shared-baseline joint delta?
    - >-
      D6 A run crashes and resumes with half-applied direct edits and no ledger of intent.
    - >-
      D7 (LB) An over-budget baseline hits the same deny at a direct edit that it hit at the fold.
      There is no escape before per-entry budgets land.
  E-migration:
    - >-
      E1 (LB) Existing ledgers, about 8000 lines and never folded. Options: fold once and grade,
      archive, or declare historical. Protected status follows from the ledger's "sole surviving
      record until that fold" line.
    - >-
      E2 (LB) Existing dated fold blocks in the baselines. Integrate them into the present-tense body
      or leave them?
    - >-
      E3 (LB) BD ids are cited in product source (sandbox_reap.rs:40, sandbox_tool.rs:280,
      corpus_driver.rs:1920), in commit messages and in closed epic records. What keeps those
      citations intact after retirement?
    - >-
      E4 Feature-dir delta files. FEAT-006 contracts/ipc.md is 2150 lines, against 657 in product.
      Options are delete, archive or keep, and each carries dead-pointer risk.
    - >-
      E5 Cross-epic BD collisions stay qualified under the standing rule. Does that rule survive?
  F-cross-surface:
    - >-
      F1 (LB) Artifact gate homes. baseline-delta.md is a declared deliverable in both the feature
      and product-lane homes (0005). Removing it takes a migration op, and baselines must stay
      writable under the gate's closed world.
    - >-
      F2 (LB) hook-enforcement-field-review. It is ruled in DECISIONS row 2026-09-23, with a
      four-wave build still owed. It covers per-entry budgets on cumulative stores, D4e amnesty, and
      its D2 superseding the budget-blocked-fold item. Questions: sequencing, which wave carries
      what, and whether D2's fix goes moot.
    - >-
      F3 (LB) Architecture store. Does the decision align baselines with the store's in-flight model
      or leave two models? The never-in-place text binds both.
    - >-
      F4 Design baseline (0013/0018). It is written "only from shipped code at a landing fold". Is
      design/ in scope?
    - >-
      F5 Baseline readers: the sufficiency check, the gap-finding blindness fence and the regression
      sweep. What do they assume about baseline truth?
    - >-
      F6 (LB) Sound-loop floor. A judgment write to product baselines still needs a seat, a
      non-author review and a user ruling. Where does the review leg sit under direct edit?
    - >-
      F7 Sibling rules that name baseline-delta.md: patterns-adopt-first.baseline-delta-landing,
      authoring-technical-requirements, feature-map altitude, the store skill, and the epic.
    - >-
      F8 BACKLOG item 3: the B116 spine groom edited the store mid-run, and the item asks to carve or
      forbid grooms. Does this decision resolve it?
  G-governance-and-build:
    - >-
      G1 (LB) Floor supersession. The floor/fail rules are baselines-never-in-place, graded-fold,
      fail.baseline-in-place, fail.ungraded-fold and baseline-delta-grammar. They leave only by
      recorded ruling, strips and migration op, with every site enumerated.
    - >-
      G2 (LB) Prior rulings that need supersession traces: AD-D3 (2026-07-30), the 2026-08-19 store
      D10, the 2026-08-14 epic D10, the 2026-08-10 D9, and the 2026-08-26 D7.
    - >-
      G3 Contested handling. Is the user's challenge ruled as theirs, and does the rejected road get
      its steelman?
    - >-
      G4 (LB) Build surface: migration 0019+, audits per unit, the crate's home census, the contract
      suite, the plugin bump, and ordering against the field-review waves and the unmet 0.109.0
      publish gate.
    - >-
      G5 (LB) Git as a hard pipeline dependency. What happens to a repo not under git or a shallow
      clone? How does it square with GI-020's unsupported-environment list?
  H-excess-watch:
    - >-
      H1 (LB) The replacement must not rebuild the delta model under new names. The candidates are a
      marker grammar, a git-diff grade, id reservation and a rationale home.
    - >-
      H2 Cheaper shape. Is direct edit plus one graded landing diff enough? Is even that grade excess
      for transcription-class changes?
fence_note: >-
  DECISIONS row 13 and BACKLOG lines 40-55 and 865-880 summarize the other session,
  hook-enforcement-field-review. I read them as operating docs and opened neither record.
---

## Notes of note

The map went to the lead as a message before any record contact. This file reproduces the angles
and their text unchanged; only the envelope fields around them are new. The message stated no
counts; the counts above are tallied from the listed angles. A1's "237 BD entries" is the Phase 0
heading count; the verified figure is 238 (FEAT-001 declares 62 in its own header, one of which the
heading grep missed), corrected in review.md, not here.
