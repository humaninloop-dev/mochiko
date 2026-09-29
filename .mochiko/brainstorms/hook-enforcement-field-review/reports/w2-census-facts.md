---
report: disclosure
wave: 2
seat: S3
subject: >-
  the census facts, wave plan §2a items 1–3. Plan: s3/plan-v3.md (581 lines, frozen; P3 PASS at
  re_review_round_1, 8 advisories). Lead GO for phase A (A0–A7). The table is its sibling,
  reports/w2-census-table.md.
pins:
  mochiko: "91b3838; migration log 0001–0023; S4's 0024–0031 not yet landed; S4's reports/w2-delta-sweep.md read"
  kinako: "6e73bd5; git --no-optional-locks status --porcelain -- .mochiko printed nothing at A0 and after each probe batch"
  binary: "target/debug/mochiko-cli 0.3.0, grammar 1..2, as built; no cargo build or test; the 0.2.0 on PATH unused"
  views: "a fresh views emit of the branch log: 80 documents, equal to .mochiko/schema-views at 91b3838"
  where: >-
    every probe ran in the scratchpad under s3/ (scratch logs s3/probe/log-L2, log-L3, log903,
    log903alt, log903noroot; scratch tree s3/probe/tree with an empty .git). Kinako was only read:
    find over the working tree, git show HEAD: for contents, and dry runs whose on-disk baseline is
    the working-tree file (equal to HEAD under .mochiko/). Every scratch log validates 0 rejecting;
    log903, log903alt and log903noroot each print 113 advisory findings (the similarity clusters).

post_s4_correction:
  why: >-
    A fact-correction note, the second write. S4's 0024–0029 landed after this report was written
    (the lead's LOG SETTLED). The facts that the post-S4 log moves are corrected here; the text below
    this note stands as it was written at the first pin. Figures and method are in
    reports/w2-census-table.md, post_s4_retake.
  pins_mochiko: >-
    91b3838 plus migrations 0024–0029; the full hashes are in the table's post_s4_retake.pin. The
    re-emit to s3/views2 has 80 documents. The 16 documents S4 touched differ from .mochiko/schema-views
    at 91b3838: 3 commands, 9 skills and 4 templates. The 21 home views are byte-identical.
  item_1_method: >-
    The hit count goes from 567 to 574 (s3/a1/hits2.tsv): 18 hits left 12 rules and 25 entered 16
    rules. The scan also reads superseded rules' tombstones.
  item_1_withdrawn_names_still_cited: >-
    Four citations become one. First, authoring-technical-requirements.artifact-home: the landed 0027
    puts the file in "the spec or the product baseline", and says a delivery run writes the product
    file "never a feature or epic copy". Second, authoring-epic.artifact-home: the landed 0027 drops
    the contracts/ and implement-log.md clauses, and the closed set now reads "The manifest and the
    spine artifacts … no product-baseline copy among them"; routing item (a) is closed. Fourth, the
    D6d sites: 0024–0029 remove every write of baseline-delta.md or of a feature copy. baseline-delta.md
    remains only in impl.baselines-in-place-marked ("no baseline-delta.md ledger") and in the tombstone
    of patterns-adopt-first.baseline-delta-landing. The one citation left is the third,
    impl.artifact-home's implement-log.md, which belongs to wave 3.
  observations_for_the_lead: "the fourth observation (the implement-log clause had no owner) is closed by 0027"
  item_1_unchanged: >-
    after_the_rulings stands, and the post-S4 rules agree with it. impl.design-outputs-home keeps only
    architecture.md in the run's home. authoring-epic.artifact-home's closed set carries no baseline
    copy. impl.base-pins and review-sufficiency.lifecycle-marker-read write only into
    sufficiency-report.md and reports/, which are already declared. No rule names .mochiko/runs/ yet.
    None of the ten names in declared_with_no_writer gained a citation.
  dispositions_ra8: >-
    0025 has landed. impl.baseline-entry-grammar spells the field `Lifecycle:` with no bold and no
    placement, while the matcher reads **Lifecycle:** (conform.rs:916–921). The table's
    post_s4_retake.text_moved carries what this means for E1, F1 and F2.
  items_2_and_3: >-
    Unchanged. The home views did not move, so no path verdict could move. The unchanged rewrite of
    all 1342 files is byte-identical under both logs, and the store measurements read kinako only.

item_1_write_sets:
  method: >-
    Every .mochiko path and home-relative file name in the replayed commands, skills, templates and
    common documents, cited doc · rule id (567 hits, s3/a1/hits.tsv); the plugin prose outside the
    log (commands, skills, agents, templates, hooks) searched for archive, nesting and trail paths;
    each hit joined to the declared sets (mochiko-cli home); then the rulings applied.
  after_the_rulings:
    feature: "tasks · plan · requirements · design-closure · sufficiency-report · architecture · proposal · contest-brief · gates, plus reports/ (delta D4). Out: baseline-delta, data-model, constraints-and-decisions, contracts/"
    epic: "manifest · proposal · contest-brief · architecture · build-order · screens-and-flows, plus reports/ (delta D4, D6(b)). Out: data-model, constraints-and-decisions, quickstart, contracts/ (D4) and implement-log.md (field D6)"
    product-lane: "sufficiency-report, plus reports/ (delta D4, I5). Out: baseline-delta"
    runs: "new: .mochiko/runs/<run-id>/, raw output, implement-log.md by name (field D4 as amended, D6; seams R5)"
    archive: "new destination for the three baseline-delta.md ledgers, moved unchanged (delta D5(ii)); the shape is table row N1c (seam 2)"
    evidence: "kinako's evidence trees are deleted at wave 4 (field D9(b), S11)"
    closed_epics: "EPIC-001 and EPIC-002 are closed record, untouched and exempt from the census (delta D5, I9)"
  declared_with_no_writer: >-
    Ten declared names no rule, template or command names as a target: proposal.md and
    contest-brief.md (feature and epic), build-order.md and screens-and-flows.md (epic), plan.md and
    design-closure.md (feature), governance-trace-summary.md and primitive-cost-budgets.md (memory),
    selection-card.md and specs-index.md (spec-map-delta). The prior census declared them from tree
    practice (0005). Each has an instance in one tree or the other. No row changes them.
  written_but_undeclared_today:
    - ".mochiko/archive/backlog-trail.md: the KM module template :42, append-only at every item close"
    - ".mochiko/archive/product-baselines/<date>/: arch.tools-brownfield-reconstruction and its skill twin (views commands/architecture.yaml:216, skills/authoring-architecture-store.yaml:208)"
    - ".mochiko/<ROADMAP|BACKLOG|DECISIONS|ARCHITECTURE|GLOSSARY>.md: the KM module's collision nest (template :168–173); no instance in either tree"
    - ".mochiko/strips/<primitive>.md: the landing ritual (CLAUDE.md, strips/README.md); this repo only"
    - ".mochiko/schema-views/<kind>/*.yaml: mochiko-cli views emit; this repo only"
    - ".mochiko/runs/<run-id>/: field D4 and D6; no rule names it yet (wave 3 rewords impl.artifact-home)"
  not_found:
    brainstorm_archive: "no rule, template, command or prose writes a brainstorm archive. Field D3 :219–220 and OQ3 name one; nothing writes it"
    groom_snapshot_path: "grooming-operating-docs says the trail and archives are append-only (SKILL.md :46) and names no snapshot path. Kinako's four FEAT-001 groom files at the archive root have no named writer"
    other: ".mochiko/memory/constitution.md and evolution-roadmap.md are flagged as superseded or cited as a stub (setup.constitution-superseded, authoring-constitution.roadmap-stub, validation-constitution.superseded-artifact-flag), never written"
  open_epic_check: >-
    authoring-epic.artifact-home reads "a landing or review directory is not a sub-home — its files
    are reports". No live rule, template or command targets epics/<EPIC-ID>/reviews/ or landing/.
    The I9 rows cover EPIC-001 and EPIC-002 only, and no H row is needed.
  withdrawn_names_still_cited:
    - "authoring-technical-requirements.artifact-home (floor) names constraints-and-decisions.md in the spec, feature, epic and product homes. S4's sweep carries it: 0027 reword, the D4 clause (feature and epic leave; spec and product stay). My earlier note that D6d missed it is answered there"
    - "authoring-epic.artifact-home (floor) names contracts/ (S4's 0027 reword) and 'implement-log.md is bounded per entry'. The implement-log clause is field D6's and sits in no seat's set I found. Routed to the lead"
    - "impl.artifact-home (floor) names implement-log.md 'bounded per entry, never per file'. Wave 3 (wave plan §3c)"
    - "the D6d sites naming baseline-delta.md or the feature copies (feat.delta-cards, impl.baseline-delta-grammar, impl.design-landing, impl.design-outputs-home and the rest): S4, 0024–0029, per its sweep"

item_2_trees:
  method: >-
    find over each .mochiko/ tree; git ls-files for the tracked flag; mochiko-cli home on every file
    against today's log (s3/a2/today.tsv, 1342 files); ids folded to tokens (s3/a2/today-patterns.tsv).
    Counts are at A2 time, before S4's sweep report landed.
  totals: "mochiko 550 files (2 untracked: this wave's plan and the R9 record) · kinako 792 (all tracked)"
  buckets_today: "mochiko: file 191 · report 104 · deferred 28 · outside 227. kinako: file 148 · report 195 · deferred 56 · undeclared file 16 · undeclared sub-directory 202 · outside 175"
  unchanged_rewrite: >-
    Under the 0.3.0 closed world and today's log, an unchanged rewrite of 605 of the 1342 files
    denies. Among them are all 101 strip files and all 80 views in this repo. Under the table's
    recommended set (scratch log log903) 411 deny, each one an evidence tree (327), an I9 epic
    directory (27), a legacy directory (B53 9, B61 1, FEAT-006/reviews 4, benchmarks 42) or the RA3
    file below (1). Per file: s3/a6/rewrite-today.tsv and s3/a6/rewrite.tsv.
  classes: >-
    declared (it resolves) · ruled (a ruling places it) · undeclared but legitimate (a writer in item
    1) · legacy (choice): the lead's list · legacy (amnestied): an undeclared name at a home's top
    level with no writer, which the file-set amnesty keeps editable while a new file of that name
    denies. (n) below stands for the home's n token, which the envelope's placeholder check
    refuses in a frontmatter value.
  patterns:
    - "kinako · .mochiko/archive/backlog-trail.md · 1 · outside (-) · undeclared but legitimate — KM module :42 (append-only trail)"
    - "kinako · .mochiko/archive/evidence/** · 166 · outside (-) · ruled — deleted at wave 4 (field D9(b), S11)"
    - "kinako · .mochiko/archive/feat-001-baseline-delta-groom-2026-09-22.md · 1 · outside (-) · legacy (choice) — S14 groom snapshot, no writer names a path"
    - "kinako · .mochiko/archive/feat-001-design-closure-groom-2026-09-22.md · 1 · outside (-) · legacy (choice) — S14 groom snapshot, no writer names a path"
    - "kinako · .mochiko/archive/feat-001-entry-groom-2026-09-22.md · 1 · outside (-) · legacy (choice) — S14 groom snapshot, no writer names a path"
    - "kinako · .mochiko/archive/feat-001-sufficiency-report-groom-2026-09-22.md · 1 · outside (-) · legacy (choice) — S14 groom snapshot, no writer names a path"
    - "kinako · .mochiko/archive/product-baselines/** · 4 · outside (-) · undeclared but legitimate — arch.tools-brownfield-reconstruction"
    - "kinako · .mochiko/brainstorms/<slug>/probes.md · 1 · undeclared-file (brainstorm-session) · legacy (amnestied) — no rule writes the name; file-set amnesty keeps it editable"
    - "kinako · .mochiko/brainstorms/<slug>/record.md · 8 · file (brainstorm-session) · declared"
    - "kinako · .mochiko/brainstorms/<slug>/review-A-crossexam.md · 1 · undeclared-file (brainstorm-session) · legacy (amnestied) — no rule writes the name; file-set amnesty keeps it editable"
    - "kinako · .mochiko/brainstorms/<slug>/review-A-map.md · 1 · undeclared-file (brainstorm-session) · legacy (amnestied) — no rule writes the name; file-set amnesty keeps it editable"
    - "kinako · .mochiko/brainstorms/<slug>/review-A-report.md · 1 · undeclared-file (brainstorm-session) · legacy (amnestied) — no rule writes the name; file-set amnesty keeps it editable"
    - "kinako · .mochiko/brainstorms/<slug>/review-B-crossexam.md · 1 · undeclared-file (brainstorm-session) · legacy (amnestied) — no rule writes the name; file-set amnesty keeps it editable"
    - "kinako · .mochiko/brainstorms/<slug>/review-B-map.md · 1 · undeclared-file (brainstorm-session) · legacy (amnestied) — no rule writes the name; file-set amnesty keeps it editable"
    - "kinako · .mochiko/brainstorms/<slug>/review-B-report.md · 1 · undeclared-file (brainstorm-session) · legacy (amnestied) — no rule writes the name; file-set amnesty keeps it editable"
    - "kinako · .mochiko/brainstorms/<slug>/review-map.md · 1 · undeclared-file (brainstorm-session) · legacy (amnestied) — no rule writes the name; file-set amnesty keeps it editable"
    - "kinako · .mochiko/brainstorms/<slug>/review-report.md · 1 · undeclared-file (brainstorm-session) · legacy (amnestied) — no rule writes the name; file-set amnesty keeps it editable"
    - "kinako · .mochiko/brainstorms/<slug>/review-survivors.md · 1 · undeclared-file (brainstorm-session) · legacy (amnestied) — no rule writes the name; file-set amnesty keeps it editable"
    - "kinako · .mochiko/brainstorms/<slug>/review-verify.md · 2 · undeclared-file (brainstorm-session) · legacy (amnestied) — no rule writes the name; file-set amnesty keeps it editable"
    - "kinako · .mochiko/brainstorms/index.md · 1 · file (brainstorms-index) · declared"
    - "kinako · .mochiko/decisions/.gitkeep · 1 · undeclared-file (decisions) · legacy (amnestied) — no rule writes the name; file-set amnesty keeps it editable"
    - "kinako · .mochiko/decisions/<date-slug>.md · 23 · file (decisions) · declared"
    - "kinako · .mochiko/epics/<EPIC-ID>/architecture.md · 2 · file (epic) · declared"
    - "kinako · .mochiko/epics/<EPIC-ID>/build-order.md · 1 · file (epic) · declared"
    - "kinako · .mochiko/epics/<EPIC-ID>/constraints-and-decisions.md · 2 · file (epic) · ruled — withdrawn (delta D4); closed record (I9)"
    - "kinako · .mochiko/epics/<EPIC-ID>/contest-brief.md · 1 · file (epic) · declared"
    - "kinako · .mochiko/epics/<EPIC-ID>/contracts/README.md · 1 · file (epic-contracts) · ruled — withdrawn (delta D4); closed record (I9)"
    - "kinako · .mochiko/epics/<EPIC-ID>/contracts/corpus-format.md · 1 · file (epic-contracts) · ruled — withdrawn (delta D4); closed record (I9)"
    - "kinako · .mochiko/epics/<EPIC-ID>/contracts/engine-port.md · 1 · file (epic-contracts) · ruled — withdrawn (delta D4); closed record (I9)"
    - "kinako · .mochiko/epics/<EPIC-ID>/contracts/ipc.md · 2 · file (epic-contracts) · ruled — withdrawn (delta D4); closed record (I9)"
    - "kinako · .mochiko/epics/<EPIC-ID>/contracts/plugin-bridge.md · 1 · file (epic-contracts) · ruled — withdrawn (delta D4); closed record (I9)"
    - "kinako · .mochiko/epics/<EPIC-ID>/data-model.md · 2 · file (epic) · ruled — withdrawn (delta D4); closed record (I9)"
    - "kinako · .mochiko/epics/<EPIC-ID>/implement-log.md · 2 · file (epic) · ruled — withdrawn (field D6); closed record (I9)"
    - "kinako · .mochiko/epics/<EPIC-ID>/landing/** · 4 · undeclared-subdir (epic) · ruled — closed record, exempt (delta I9)"
    - "kinako · .mochiko/epics/<EPIC-ID>/manifest.md · 2 · file (epic) · declared"
    - "kinako · .mochiko/epics/<EPIC-ID>/proposal.md · 1 · file (epic) · declared"
    - "kinako · .mochiko/epics/<EPIC-ID>/quickstart.md · 2 · file (epic) · ruled — withdrawn (delta D4); closed record (I9)"
    - "kinako · .mochiko/epics/<EPIC-ID>/reports/* · 7 · report (epic) · declared"
    - "kinako · .mochiko/epics/<EPIC-ID>/reviews/** · 23 · undeclared-subdir (epic) · ruled — closed record, exempt (delta I9)"
    - "kinako · .mochiko/epics/<EPIC-ID>/screens-and-flows.md · 1 · file (epic) · declared"
    - "kinako · .mochiko/features/.gitkeep · 1 · undeclared-file (features-index) · legacy (amnestied) — no rule writes the name; file-set amnesty keeps it editable"
    - "kinako · .mochiko/features/<FEAT-ID>.md · 7 · file (features-index) · declared"
    - "kinako · .mochiko/features/<FEAT-ID>/architecture.md · 2 · file (feature) · declared"
    - "kinako · .mochiko/features/<FEAT-ID>/baseline-delta.md · 3 · file (feature) · ruled — withdrawn (delta D4); moved or deleted at wave 4 (D5(ii)/(iii))"
    - "kinako · .mochiko/features/<FEAT-ID>/constraints-and-decisions.md · 2 · file (feature) · ruled — withdrawn (delta D4); moved or deleted at wave 4 (D5(ii)/(iii))"
    - "kinako · .mochiko/features/<FEAT-ID>/contest-brief.md · 1 · file (feature) · declared"
    - "kinako · .mochiko/features/<FEAT-ID>/contracts/corpus-format.md · 1 · file (feature-contracts) · ruled — withdrawn (delta D4); moved or deleted at wave 4 (D5(ii)/(iii))"
    - "kinako · .mochiko/features/<FEAT-ID>/contracts/engine-port.md · 2 · file (feature-contracts) · ruled — withdrawn (delta D4); moved or deleted at wave 4 (D5(ii)/(iii))"
    - "kinako · .mochiko/features/<FEAT-ID>/contracts/harness-store.md · 1 · file (feature-contracts) · ruled — withdrawn (delta D4); moved or deleted at wave 4 (D5(ii)/(iii))"
    - "kinako · .mochiko/features/<FEAT-ID>/contracts/ipc.md · 2 · file (feature-contracts) · ruled — withdrawn (delta D4); moved or deleted at wave 4 (D5(ii)/(iii))"
    - "kinako · .mochiko/features/<FEAT-ID>/contracts/plugin-bridge.md · 2 · file (feature-contracts) · ruled — withdrawn (delta D4); moved or deleted at wave 4 (D5(ii)/(iii))"
    - "kinako · .mochiko/features/<FEAT-ID>/data-model.md · 2 · file (feature) · ruled — withdrawn (delta D4); moved or deleted at wave 4 (D5(ii)/(iii))"
    - "kinako · .mochiko/features/<FEAT-ID>/design-closure.md · 3 · file (feature) · declared"
    - "kinako · .mochiko/features/<FEAT-ID>/gates.md · 1 · file (feature) · declared"
    - "kinako · .mochiko/features/<FEAT-ID>/plan.md · 3 · file (feature) · declared"
    - "kinako · .mochiko/features/<FEAT-ID>/proposal.md · 1 · file (feature) · declared"
    - "kinako · .mochiko/features/<FEAT-ID>/reports/* · 185 · report (feature) · declared"
    - "kinako · .mochiko/features/<FEAT-ID>/reports/evidence/** · 161 · undeclared-subdir (feature) · ruled — deleted at wave 4 (field D9(b), S11)"
    - "kinako · .mochiko/features/<FEAT-ID>/requirements.md · 3 · file (feature) · declared"
    - "kinako · .mochiko/features/<FEAT-ID>/reviews/** · 4 · undeclared-subdir (feature) · legacy (choice) — S11"
    - "kinako · .mochiko/features/<FEAT-ID>/sufficiency-report.md · 3 · file (feature) · declared"
    - "kinako · .mochiko/features/<FEAT-ID>/tasks.md · 3 · file (feature) · declared"
    - "kinako · .mochiko/features/B53/** · 9 · undeclared-subdir (features-index) · legacy (choice) — S11"
    - "kinako · .mochiko/features/B61/** · 1 · undeclared-subdir (features-index) · legacy (choice) — S11"
    - "kinako · .mochiko/features/desk/<date-slug>/derivation.md · 2 · file (feature-desk) · declared"
    - "kinako · .mochiko/features/desk/<date-slug>/reports/* · 3 · report (feature-desk) · declared"
    - "kinako · .mochiko/features/desk/<date-slug>/review.md · 1 · undeclared-file (feature-desk) · legacy (amnestied) — no rule writes the name; file-set amnesty keeps it editable"
    - "kinako · .mochiko/memory/governance-intent.md · 1 · file (memory) · declared"
    - "kinako · .mochiko/memory/governance-ledger.md · 1 · file (memory) · declared"
    - "kinako · .mochiko/memory/governance-trace-summary.md · 1 · file (memory) · declared"
    - "kinako · .mochiko/memory/knowledge-management.md · 1 · file (memory) · declared"
    - "kinako · .mochiko/product/architecture/concerns.md · 1 · file (product-architecture) · declared"
    - "kinako · .mochiko/product/architecture/spine.md · 1 · file (product-architecture) · declared"
    - "kinako · .mochiko/product/constraints-and-decisions.md · 1 · file (product) · declared"
    - "kinako · .mochiko/product/contracts/README.md · 1 · file (product-contracts) · declared"
    - "kinako · .mochiko/product/contracts/corpus-format.md · 1 · file (product-contracts) · declared"
    - "kinako · .mochiko/product/contracts/engine-port.md · 1 · file (product-contracts) · declared"
    - "kinako · .mochiko/product/contracts/harness-store.md · 1 · file (product-contracts) · declared"
    - "kinako · .mochiko/product/contracts/ipc.md · 1 · file (product-contracts) · declared"
    - "kinako · .mochiko/product/contracts/plugin-bridge.md · 1 · file (product-contracts) · declared"
    - "kinako · .mochiko/product/data-model.md · 1 · file (product) · declared"
    - "kinako · .mochiko/product/quickstart.md · 1 · file (product) · declared"
    - "kinako · .mochiko/specs/<slug>/derivation.md · 2 · file (spec) · declared"
    - "kinako · .mochiko/specs/<slug>/map-delta/<FEAT-ID>.md · 3 · file (spec-map-delta) · declared"
    - "kinako · .mochiko/specs/<slug>/map-delta/FEATURES.md · 1 · file (spec-map-delta) · declared"
    - "kinako · .mochiko/specs/<slug>/map-delta/README.md · 1 · file (spec-map-delta) · declared"
    - "kinako · .mochiko/specs/<slug>/map-delta/selection-card.md · 1 · file (spec-map-delta) · declared"
    - "kinako · .mochiko/specs/<slug>/map-delta/specs-index.md · 1 · file (spec-map-delta) · declared"
    - "kinako · .mochiko/specs/<slug>/prototype/** · 56 · deferred (spec) · declared"
    - "kinako · .mochiko/specs/<slug>/spec.md · 3 · file (spec) · declared"
    - "kinako · .mochiko/specs/<slug>/stories/US-(n).md · 22 · file (spec-stories) · declared"
    - "kinako · .mochiko/specs/<slug>/stress-test.md · 1 · undeclared-file (spec) · legacy (amnestied) — no rule writes the name; file-set amnesty keeps it editable"
    - "kinako · .mochiko/specs/index.md · 1 · file (specs-index) · declared"
    - "mochiko · .mochiko/archive/REGISTRY.md · 1 · outside (-) · legacy (declare only) — frozen, CLAUDE.md History"
    - "mochiko · .mochiko/archive/ROADMAP.md · 1 · outside (-) · legacy (declare only) — frozen, CLAUDE.md History"
    - "mochiko · .mochiko/archive/backlog-trail.md · 1 · outside (-) · undeclared but legitimate — KM module :42 (append-only trail)"
    - "mochiko · .mochiko/archive/provenance-frozen-2026-09-05.yaml · 1 · outside (-) · legacy (declare only) — frozen, CLAUDE.md History"
    - "mochiko · .mochiko/benchmarks/** · 42 · outside (-) · legacy (choice) — mochiko own; no writer since 2026-08-11"
    - "mochiko · .mochiko/brainstorms/<slug>/build-log.md · 6 · file (brainstorm-session) · declared"
    - "mochiko · .mochiko/brainstorms/<slug>/inputs/** · 5 · deferred (brainstorm-session) · declared"
    - "mochiko · .mochiko/brainstorms/<slug>/record.md · 72 · file (brainstorm-session) · declared"
    - "mochiko · .mochiko/brainstorms/<slug>/referents/** · 5 · deferred (brainstorm-session) · declared"
    - "mochiko · .mochiko/brainstorms/<slug>/reports/* · 104 · report (brainstorm-session) · declared"
    - "mochiko · .mochiko/brainstorms/<slug>/research/** · 18 · deferred (brainstorm-session) · declared"
    - "mochiko · .mochiko/brainstorms/<slug>/synthesis.md · 8 · file (brainstorm-session) · declared"
    - "mochiko · .mochiko/brainstorms/<slug>/wave(n)-<slug>.md · 43 · file (brainstorm-session) · declared" # 42 tracked
    - "mochiko · .mochiko/brainstorms/index.md · 1 · file (brainstorms-index) · declared"
    - "mochiko · .mochiko/decisions/<date-slug>.md · 53 · file (decisions) · declared" # 52 tracked
    - "mochiko · .mochiko/memory/codebase-analysis.md · 1 · file (memory) · declared"
    - "mochiko · .mochiko/memory/governance-intent.md · 1 · file (memory) · declared"
    - "mochiko · .mochiko/memory/governance-ledger.md · 1 · file (memory) · declared"
    - "mochiko · .mochiko/memory/governance-trace-summary.md · 1 · file (memory) · declared"
    - "mochiko · .mochiko/memory/knowledge-management.md · 1 · file (memory) · declared"
    - "mochiko · .mochiko/memory/primitive-cost-budgets.md · 1 · file (memory) · declared"
    - "mochiko · .mochiko/product/architecture/concerns.md · 1 · file (product-architecture) · declared"
    - "mochiko · .mochiko/product/architecture/spine.md · 1 · file (product-architecture) · declared"
    - "mochiko · .mochiko/schema-views/commands/* · 6 · outside (-) · undeclared but legitimate — views emit (field D3 names it)"
    - "mochiko · .mochiko/schema-views/common/* · 3 · outside (-) · undeclared but legitimate — views emit (field D3 names it)"
    - "mochiko · .mochiko/schema-views/homes/* · 21 · outside (-) · undeclared but legitimate — views emit (field D3 names it)"
    - "mochiko · .mochiko/schema-views/labels/* · 2 · outside (-) · undeclared but legitimate — views emit (field D3 names it)"
    - "mochiko · .mochiko/schema-views/shelves/* · 1 · outside (-) · undeclared but legitimate — views emit (field D3 names it)"
    - "mochiko · .mochiko/schema-views/skills/* · 35 · outside (-) · undeclared but legitimate — views emit (field D3 names it)"
    - "mochiko · .mochiko/schema-views/templates/* · 12 · outside (-) · undeclared but legitimate — views emit (field D3 names it)"
    - "mochiko · .mochiko/strips/*.md · 101 · outside (-) · undeclared but legitimate — strips/README.md (field D3 names it)"

item_3_stores:
  method: >-
    Primary counter: the binary's own amnesty listing. Each copy sits in the scratch tree and is
    dry-run unchanged under a scratch log whose product and product-architecture homes carry a zero
    budget at the level under test (log-L2, log-L3), so the allow lists every entry and section with
    its count. Cross-check: s3/entry_spans.py, a port of heading_of, heading_scan and entry_spans
    (conform.rs :811, :867, :910). Only ## and ### open an entry; #### is body text; fenced lines are
    skipped; text before the first heading is not counted. Binary and replica agree heading by
    heading on every copy and level, except where RA3 makes the binary deny (below).
  sources: >-
    kinako HEAD 6e73bd5 via git show. Run 4's fold text (reports/landing-fold-text-run4-2026-09-23.md
    §A1 :80–201, §A2 :202–288) is applied first (D5(i)); then the two ## … landing fold blocks of each
    file are re-homed (D5(iv)): data-model :907 and :1080, constraints-and-decisions :371 and :547.
  mapping_rule: >-
    First match wins. 1 extends: the heading's first id names a base entry. 2 row-only base: the id's
    only base form is a table row, so it is its own entry (sensitivity d adds it to Live rows). 3 new
    id, marked or not: a new entry in its kind's section. 4 register: the lines join that ## section's
    text (M drops the register heading; P keeps it as its own entry). 5 unmapped: its own entry
    (sensitivity u appends it to the entry before). 6 (added for RA2) a Part n body with no heading of
    its own: its lines join what the Part names; Part 2's decision rows join the Live rows entry
    (sensitivity s: the Decisions section text), Parts 1, 3 and 4 join their sections. Scaffolding
    removed: each ## … landing fold heading and preamble, every Part n heading, the §A headers and
    Traceability trailers. Clause 4 no longer lists Part 4; the Part heading is scaffolding and its
    body is clause 6's.
  variants: >-
    M merged: an extension's body joins its target entry and its heading goes; every heading from
    clauses 2, 3 and 5 is its own entry; the honest future shape under delta D1. P placed: each
    extension stays its own entry right after its target; D5(iv)'s literal move. For the
    constraints file also L (levels as they stand, #### ids read inside the preceding ### entry) and
    I (every C-, D- and IP- block at ###). M figures for [MODIFY] and [EXTEND] units are upper bounds:
    those units restate base text.
  mapping:
    data-model:
      - "fold-text :83–92 · clause - · scaffolding, removed · ## The FEAT-001 landing fold *(2026-09-23)*"
      - "fold-text :93–105 · clause 1 · extends :523 ### Entity: AppSettings **[NEW]** — the  · ### `[MODIFY]` Amendment 1 — `AppSettings` (`SPN-028`) gains `ConductCon"
      - "fold-text :106–119 · clause 3 · new entry in Entities · ### Entity: ConductConfiguration `[NEW]` — nested on AppSettings (`SPN-0"
      - "fold-text :120–140 · clause 3 · new entry in Entities · ### Entity: SessionConduct `[NEW]` — nested on BoundSession, via the bin"
      - "fold-text :141–152 · clause 1 · extends :646 ### Entity: BoundSession **[NEW]** — the · ### `BoundSession` `[MODIFY]` / `SpoolBindRecord.conduct` — gains one at"
      - "fold-text :153–162 · clause 3 · new entry in Read models · ### Read model — `BoundSessionConductView` (serving surface, computed, n"
      - "fold-text :163–169 · clause 4 · register text of Relationships · ### Relationships — two rows added"
      - "fold-text :170–176 · clause 4 · register text of Entity summary · ### Entity summary — two rows added"
      - "fold-text :177–187 · clause 4 · register text of Data sensitivity · ### Data Sensitivity Summary — rows added"
      - "fold-text :188–195 · clause 4 · register text of Validation rules · ### Validation rules — continuing the shared sequence at V-48"
      - "fold-text :196–199 · clause - · scaffolding, removed · "
      - "data-model :907–919 · clause - · scaffolding, removed · ## The EPIC-002 landing fold *(2026-09-12)*"
      - "data-model :920–945 · clause 1 · extends :523 ### Entity: AppSettings **[NEW]** — the  · ### Entity: AppSettings `[MODIFY]` — schemaVersion 2, and **ten** attrib"
      - "data-model :946–980 · clause 5 · own entry in Entities · ### The v1→v2 migrate-on-read rule"
      - "data-model :981–1019 · clause 1 · extends :608 ### Entity: DisclosureAcknowledgement ** · ### Entity: DisclosureAcknowledgement `[MODIFY]` — the digest's schema-v"
      - "data-model :1020–1048 · clause 1 · extends :823 ### HomeState — `home.state`'s response  · ### Read model: HomeState `[EXTEND]` — **two** attributes added, not one"
      - "data-model :1049–1056 · clause 1 · extends :445 ### Entity: ExtractionRun **[NEW]** — th · ### Entity: ExtractionRun `[EXTEND]` — two named cause classes"
      - "data-model :1057–1063 · clause 1 · extends :361 ### OperationsLogEntry — `<app-data-root · ### Entity: OperationsLogEntry `[EXTEND]` — the in-VM engine version"
      - "data-model :1064–1079 · clause 5 · own entry in Entities · ### The three keychain items — an external store at a boundary"
      - "data-model :1080–1089 · clause - · scaffolding, removed · ## The FEAT-001 landing fold *(2026-09-22)*"
      - "data-model :1090–1149 · clause 1 · extends :646 ### Entity: BoundSession **[NEW]** — the · ### Entity: BoundSession `[EXTEND]` — the derived `harnessSessionFile` r"
      - "data-model :1150–1200 · clause 3 · new entry in Entities · ### Entity: HarnessSessionFile `[NEW]` — external, unmodelled record"
      - "data-model :1201–1206 · clause 4 · register text of Validation rules · ### Validation rules — continuing the shared sequence"
    constraints:
      - "fold-text :205–215 · clause - · scaffolding, removed · ## The FEAT-001 landing fold *(2026-09-23)*"
      - "fold-text :216–216 · clause - · scaffolding, removed · ### Part 1 · Constraints"
      - "fold-text :217–217 · clause 6 · register text of Constraints · "
      - "fold-text :218–223 · clause 3 · new entry in Constraints · #### `C-009` — conduct crosses as values, never composed text"
      - "fold-text :224–229 · clause 3 · new entry in Constraints · #### `C-010` — every settable integer is floor-checked at the write boun"
      - "fold-text :230–238 · clause 3 · new entry in Constraints · #### `C-011` — absence reads as one named state, never as a fault or a s"
      - "fold-text :239–239 · clause - · scaffolding, removed · ### Part 2 · Decisions — one line each, full ADR text at the feature fil"
      - "fold-text :240–275 · clause 6 · Live rows entry (s: Decisions section text) · "
      - "fold-text :276–276 · clause - · scaffolding, removed · ### Part 3 · Infrastructure provisioning — none"
      - "fold-text :277–279 · clause 6 · register text of Infrastructure provisioning · "
      - "fold-text :280–280 · clause - · scaffolding, removed · ### Part 4 · Declarations — none"
      - "fold-text :281–283 · clause 6 · register text of Declarations · "
      - "fold-text :284–286 · clause - · scaffolding, removed · "
      - "constraints :370–378 · clause - · scaffolding, removed · "
      - "constraints :379–379 · clause - · scaffolding, removed · ### Part 1 · Constraints"
      - "constraints :380–380 · clause 6 · register text of Constraints · "
      - "constraints :381–393 · clause 1 · extends :55 ### C-002 — the leader's installed harne · #### `C-002` · **AMENDED** — the harness floor narrows to api-key, gatew"
      - "constraints :394–424 · clause 3 · new entry in Constraints · #### `C-004` · **NEW** — no seat and no gate writes the leader's host `a"
      - "constraints :425–425 · clause - · scaffolding, removed · ### Part 2 · Decisions"
      - "constraints :426–439 · clause 6 · Live rows entry (s: Decisions section text) · "
      - "constraints :440–466 · clause 2 · own entry in Decisions · #### `D-039` · **AMENDED 2026-09-12 — a premise proved false, not a codi"
      - "constraints :467–467 · clause - · scaffolding, removed · ### Part 3 · Infrastructure provisioning"
      - "constraints :468–508 · clause 6 · register text of Infrastructure provisioning · "
      - "constraints :509–509 · clause - · scaffolding, removed · ### Part 4 · Declarations — product-level"
      - "constraints :510–524 · clause 6 · register text of Declarations · "
      - "constraints :525–539 · clause 4 · register text of Non-functional requirements · ### Non-functional requirements — no number minted, and the deferrals ar"
      - "constraints :540–546 · clause 4 · register text of Where this baseline is deliberately silent · ### Routed, not settled"
      - "constraints :547–560 · clause - · scaffolding, removed · ## The FEAT-001 landing fold *(2026-09-22)*"
      - "constraints :561–561 · clause - · scaffolding, removed · ### Part 1 · Constraints"
      - "constraints :562–562 · clause 6 · register text of Constraints · "
      - "constraints :563–571 · clause 3 · new entry in Constraints · #### `C-005` — read-never-write except one consented delete"
      - "constraints :572–579 · clause 3 · new entry in Constraints · #### `C-006` — the verb never reads session-file content"
      - "constraints :580–597 · clause 3 · new entry in Constraints · #### `C-007` — seat verification under a relocated HOME, seeded throwawa"
      - "constraints :598–606 · clause 3 · new entry in Constraints · #### `C-008` — the store root resolves from the environment, never hardc"
      - "constraints :607–607 · clause - · scaffolding, removed · ### Part 2 · Decisions"
      - "constraints :608–608 · clause 6 · Live rows entry (s: Decisions section text) · "
      - "constraints :609–626 · clause 3 · new entry in Decisions · #### `D-045` — the bound-session SET and the sessionId→file locator"
      - "constraints :627–639 · clause 3 · new entry in Decisions · #### `D-046` — the turn-count source rendered per file on SCR-038"
      - "constraints :640–691 · clause 3 · new entry in Decisions · #### `D-047` — confirmation mechanics: a two-invocation gate carrying a "
      - "constraints :692–704 · clause 3 · new entry in Decisions · #### `D-048` — delete mechanics: unlink, continue-on-partial-failure"
      - "constraints :705–714 · clause 3 · new entry in Decisions · #### `D-049` — enumeration scope: the whole store, not the current proje"
      - "constraints :715–715 · clause - · scaffolding, removed · ### Part 3 · Infrastructure provisioning"
      - "constraints :716–716 · clause 6 · register text of Infrastructure provisioning · "
      - "constraints :717–724 · clause 3 · new entry in Infrastructure provisioning · #### `IP-012` — a seeded throwaway harness-store fixture on every verifi"
      - "constraints :725–725 · clause - · scaffolding, removed · ### Part 4 · Declarations — product-level"
      - "constraints :726–733 · clause 6 · register text of Declarations · "
  conservation:
    data-model: "base 906 kept; 417 fold and fold-text lines = 380 placed + 37 scaffolding; output M 1273 (13 merged headings dropped), P 1286, Mu 1271 (15 dropped); closes, every source line placed once"
    constraints-and-decisions: "base 369 kept; 446 = 397 placed + 49 scaffolding; 2 synthetic lines (## Declarations (INT-XXX · DS-XXX), which the base lacks); output M 765 (3 dropped), P 768; closes for ML, MI, PL, PI, MIu, MId, MIs, MLs"
  figures:
    data-model:
      level: "### (one entity or read model per entry). At ## the whole Entities section is one entry: 632 at HEAD, 927 M, 934 P"
      HEAD: "28 entries · max 85 (Entity: AppSettings) · median 28.5 · p90 60 · over 60: 3 · over 100: 0. Sections: Validation rules 44, Data sensitivity 37, Entity summary 30, Relationships 30, Divergences 30, Read models 20, Delivered but out of scope 8, the two fold blocks 13 and 10"
      M: "24 entries · max 153 (Entity: BoundSession, upper bound) · AppSettings 122 (upper bound) · ExtractionRun 85 · median 27 · p90 85 · over 60: 5 · over 100: 2 · over 150: 1 · over 177: 0. Sections: Validation rules 56, Data sensitivity 47, Entity summary 36, Relationships 36, Divergences 30, Read models 20, Delivered 8"
      P: "37 entries · max 85 · median 24 · p90 53 · over 60: 3. Sections as HEAD, less the fold blocks"
      Mu: "sensitivity only: 22 entries, max 156 (AppSettings with the migrate-on-read rule appended)"
    constraints-and-decisions:
      level: "###. Ids sit at ### (base C-001, C-002), at #### (base D-025 :175 and D-037 :247, every fold id) and in table rows (Live rows). At ## the Decisions section is one entry: 217 at HEAD, 401 M"
      HEAD: "replica (RA3): 19 entries · max 126 (### 2a · Transcribed at the architect's pen, a grouping holding #### D-025 and D-037) · Part 2 · Decisions 108 and 42, Part 1 · Constraints 46 twice (duplicate keys). Sections: IP 16, fold blocks 14 and 8"
      L: "M: 9 entries · max 169 (### `D-037` — the ruled mechanism extends …, a grouping holding #### D-039 and D-045…D-049) · 126 (2a · Transcribed) · 115 (`C-002` — the version floor, holding #### C-009…C-011 and C-004…C-008) · Live rows 77. P: 11 entries, max 169"
      I: "M: 26 entries · max 77 (Live rows, the decision table, one row per decision) · D-025 72 · D-047 52 · median 17.5 · p90 43 · over 60: 2 · over 100: 0. P: 29 entries, max 77"
      sections: "both levels, M: Infrastructure provisioning 61, Declarations 28 (synthetic heading), NFR 20, Where this baseline is deliberately silent 15, Constraints 7, Decisions 4. P: IP 61, Declarations 28, silent 9, NFR 6"
      sensitivities: "MId: Live rows 104 · MIs: max 72, Decisions section text 55 · MLs: max 169, Decisions section text 55 · MIu equals MI"
      relevel: "L and I differ by 17 heading lines, #### to ###: 15 in fold text, 2 in the base (D-025 :175, D-037 :247)"
    quickstart: "HEAD (D5 leaves it alone): 197 lines · ## 11 entries, max 28 (2 · Install the plugin …), median 16 · ### 1 entry (12). Whole-file bound 300 today. Growth: 187 then 197 lines over its two commits"
    spine: "kinako HEAD: ## 4 entries, Container diagram 92, Elements 60, Key flows 26, As-built notes 13. The template's section budgets are 30, 40, 30 and 15, so two stand over today. ### 8 entries, max 26, with the diagram as 92 lines of section text. mochiko: an 8-line stub with no entry"
    concerns: "kinako HEAD: ## 34 entries (one per AX-XXX), max 84 (AX-012), median 23.5, p90 51. No size bound today; the template declares none. mochiko: a 4-line stub"
    no_instance: "design/design.md and concerns/<AX-ID>.md exist in neither tree"
    gates_md: "kinako features/FEAT-001/gates.md: 146 lines against the feature home's whole-file 150; ## Standing Setup 9, Gates 108, Pending 26; gates are **TEST:** lines, not headings. It accumulates across runs (testing-gap-finding.gates-artifact-contract) and is the one feature-home file OQ1 asks about that survives D4"
  trace_177: >-
    Source: reports/review.md:142 ("the largest entity entry is 177 lines"), carried into the
    record at :206, :516 and :654. It is kinako data-model.md :646 (### Entity: BoundSession
    **[NEW]**) counted to the next ### heading at :823 (### HomeState), across ## Relationships
    :729, ## Validation rules :759 and ## Read models :803; 823 − 646 = 177 at 2abf8ad, ec0b17f and
    76179e1 (HEAD). The gate ends an entry at the next heading at its level or above, so it counts
    :646–:728, 83 lines. By the gate's count the largest entity at HEAD is AppSettings (85), and
    after D5 BoundSession (153, M, upper bound). Per Q4 the floor stays 177; the mis-measure is table
    row M1.
  ra3: >-
    settle compares a candidate fault with the first standing fault of its key (conform.rs:274–296),
    so a later, larger duplicate makes an unchanged rewrite deny. Kinako's constraints file at HEAD,
    at ### under a zero budget, denies (### Part 2 · Decisions: 42 lines, then 108), so its HEAD ###
    figures above are the replica's. It does not bite at the table's budgets. One live instance:
    kinako features/FEAT-002/reports/driver-fix-report.md carries four ## Notes of note, of 29, 26,
    52 and 62 lines against 15, and the refusal names the third (corrected at R1's A9: this line
    first said the first section was 52); an unchanged rewrite denies under today's log and under
    the table's set alike. The lead routes the defect to wave 3.
  markers: >-
    No kinako store carries a **Lifecycle:**, **Raised:** or **Weighed:** line today. The
    constraints file's 11 **Status:** stance values sit inline on **Class:** … lines, never on a
    line of their own. concerns.md's 33 row fields read "- **Status**:", colon outside the bold,
    which the exempt matcher (**<Name>:**) never matches. Lifecycle adds 1 line to each entry a run
    touches; Raised and Weighed add 2 more to a build-raised entry (delta D2, D3b; seams R8). An R9
    stub is its heading plus **Lifecycle:** removed: 2 lines, 1 counted if Lifecycle is exempt. None
    exists today; the stub's heading keeps its amnesty key.
  contracts_information_only: >-
    D6a keeps contracts/* outside the size scope. kinako product/contracts/ipc.md (657 lines) has
    dated reconciliation sections at :117, :427 and :508, and a ## The EPIC-002 landing fold block
    of its own at :420 (238 lines at ##). D5(iv) names only the data-model and constraints fold
    blocks.

observations_for_the_lead:
  - "D5(i): run 4's §A2 Part 2 heading reads 'full ADR text at the feature file's own D-XXX blocks'. That file is FEAT-001's copy, which D5(iii) deletes; after D5 the product file carries the one-line forms of D-050…D-061 only (history keeps the blocks)"
  - "D5(iv) does not name product/contracts/ipc.md's own ## The EPIC-002 landing fold block (:420). No size check reads it (D6a)"
  - "the branch binary's closed world denies every strip and view write in this repo until 0032's homes land; they must ship in the same bump"
  - "authoring-epic.artifact-home's 'implement-log.md is bounded per entry' has no owner I found (field D6); S4's 0027 edits the same rule for D4"

dispositions:
  - "RA1 taken: L and I variants measured; table row S2 carries one budget per option, each checked with the next honest decision"
  - "RA2 taken: clause 6 places the Part 2 body (fold text :240–275, 36 lines under the removed :239 heading) into the Live rows entry; conservation closes; the clause-4 overlap is settled as above"
  - "RA3 routed by the lead to wave 3; disclosed here and in the table's consequence column"
  - "RA4 withdrawn by P3 on the frozen file"
  - "RA5 taken: the single table revision round after R1 (plan A11) is my own disposition, not a lead ruling; the lead folds or rejects R1's findings (wave plan §4)"
  - "RA6 taken as the lead ruled: mochiko's archive files are frozen; their row offers declare only (table N1b)"
  - "RA7 taken: every option of every archive and legacy row is checked at its destination (table rows N1a, N1c, L1–L3)"
  - "RA8 handed to R1 and the lead: 0025 has not landed. The table's exempt field is Lifecycle and the matcher reads **Lifecycle:**, R9's spelling; 0034 and 0025 must agree at the re-cut"
  - "RA9 taken: the seams record's routed items are :89–100 and its R9 pointer :86–87"
---

## Notes of note

- Nothing was written in either repository before this report; kinako's `.mochiko/` status stayed empty.
- One gate deny early in phase A: the installed 0.2.0 gate read a `cp` source as its write target
  (F3 shape b). Reported to the lead then, not retried; no second deny on any path.
- No §7 stop: every ruled move, merge and deletion has a checked route (table, consequence column),
  and every store fits one budget, so the per-entry-file split is not reached.
- `migrate validate` on a debug build takes about 160 s per log; every run completed.
