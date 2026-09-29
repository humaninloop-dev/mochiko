---
report: disclosure
wave: 2
seat: S3
subject: >-
  The census table (wave plan §2b), revised once after R1's review (reports/w2-census-review.md).
  It is the one artifact of this wave the user rules on: every row carries a recommendation and its
  reason, and every choice the user makes is a row. The facts behind it are in
  reports/w2-census-facts.md.
how_to_read: >-
  Each row is either ruled (earlier rulings already decide it; it is shown so the set is complete)
  or a choice (yours). A choice row lists its options, the recommended one and why. "Today" is what
  the write gate does now: the branch binary (mochiko-cli 0.3.0) on the committed migration log
  0001–0029. "After" is what it does under this table's recommended rows, checked with the same
  binary against scratch drafts of the four home migrations; every other option was checked under a
  scratch log of its own. "Carrier" names what lands a row: a home migration (0032–0035, written
  after your ruling), S4's one re-cut of migration 0025, a crate change in wave 3, or the wave-4
  kinako pass. Each row's consequence says what the row does to existing files and to the next
  honest write, and consequence_by_pattern at the end covers every existing file in both repos.
choices_at_a_glance:
  - "H0a · 14 kinako files under names their homes do not declare (plus two .gitkeep): keep them in place, editable (they cannot move to reports/ as they are)"
  - "H1g · the feature's gates.md bound: whole file, 300 lines"
  - "H4 · the product-lane home: add architecture.md, the drawing a lane run signs"
  - "H5 · the spec home: drop data-model.md and quickstart.md, keep constraints-and-decisions.md"
  - "N1 · the archive root: the backlog trail and mochiko's three frozen files only; no brainstorm archive until something writes one"
  - "N1a · kinako's four groom snapshots: no pattern at the archive root; the four stay, editable"
  - "N1c · the archived ledgers: a home of their own, archive/ledgers/<FEAT-ID>/baseline-delta.md"
  - "N3 · operating docs nested under .mochiko/: declare the five names"
  - "N4 · the derived views: one home that admits only slug-named .yaml files"
  - "N5 · the strip records: README.md and slug-named .md, bounds cited to strips/README.md"
  - "L1 · kinako's B53 and B61 cycle reports: keep in place as closed record"
  - "L2 · kinako's FEAT-006 reviews/: keep in place as closed record"
  - "L3 · mochiko's benchmarks/: leave undeclared, closed record"
  - "S1 · product data-model.md: one ### entry per entity, 177 lines; section text also 177"
  - "S2 · product constraints-and-decisions.md: re-level every id to ###, 177 lines per entry"
  - "S3 · the architecture spine: ## entries of 177 lines, template kept for shape"
  - "S4 · the concern ledger: ## entries of 177 lines"
  - "S5 · a graduated concern file: whole file, 177 lines"
  - "P1 · text above a store's first heading: a wave-3 crate bound at the entry budget, 177"
  - "E1 · which marker lines are not counted: Lifecycle only, on the three baselines"
  - "Q1 · product quickstart.md: ## entries of 177 lines"
  - "Q2 · product design/design.md: keep the template's section budgets"
  - "F1 · where the Lifecycle marker sits: first line under the entry heading"
  - "F2 · where Raised and Weighed sit: right after the Lifecycle line"
  - "F3 · a spine element's why: two columns, Raised and Weighed, on its own row"
  - "F4 · what a contracts entry is: the changed clause's own heading; api.yaml takes an x-lifecycle key"
  - "M1 · the 177-line floor rests on a mis-measure: keep 177"
ruled_at_a_glance: "H0, H1, H2, H3, N1b, N1d, N2, R1, R2, C1 and X1 restate earlier rulings (the field review, the delta session, the seams record, the lead's build-time rulings); nothing in them is yours to choose"
terms:
  - "field Dn, Sn, OQn, Vn: a ruling, supplementary ruling, open question or verify item of the hook-enforcement field review (.mochiko/brainstorms/hook-enforcement-field-review/record.md)"
  - "delta Dn, In: a ruling or invariant of the delta-files session (.mochiko/brainstorms/delta-files-vs-direct-baseline-edits/record.md). Delta D5 is the wave-4 kinako cleanup: (i) run 4's fold text applied, (ii) the three baseline-delta.md ledgers moved to the archive unchanged, (iii) the feature copies of the baselines deleted, (iv) the two landing-fold blocks moved into the entries they extend, (v) every link re-pointed"
  - "seams Rn: the joint build's seam rulings (.mochiko/decisions/2026-09-29-joint-hook-delta-build-seams.md); R9, the removed-entry stub, has its own record (.mochiko/decisions/2026-09-29-landed-removal-stub.md)"
  - "lead Qn, Bn, RAn: the session lead's build-time rulings, logged in build-log.md"
  - "the amnesty: a file that already breaks a rule may be rewritten if the write does not make it worse; a file under a name its home does not declare stays editable, while a new file of that name is refused. A sub-directory its home does not declare gets no amnesty: every write into it is refused"
  - "allowed by location: a sub-directory a home names without a file set; it admits any file except one carrying report frontmatter (field V2, the report sniff)"
  - "the floor, 177: the smallest per-entry budget the rulings allow (field D2; lead Q4)"
  - "rung n: the plan-minimalism ladder step where the recommendation stops (1 required, 2 simpler shape, 3 already exists, 4 minimum now, 5 builder's room)"
pins: >-
  mochiko: joint-hook-delta at 91b3838 plus migrations 0024–0029, sha256 0024
  86a0350ae31efe3dbdcbfa74df047f5765197ae92abdcf9fa575a4eb3d0e6c33 · 0025
  6faccc7c74b43b6d24a5b30c12e0f1b6599e5d5eb3b83c38aefa85318b6bbf82 · 0026
  45e41eb8386a612c2c1d6c5a4f81d474497f9c5c196fbe7bfd3a411dd48ff884 · 0027
  a3384a244be42857029fbee9544a14551b6d9adf12723abece462a3df3e1cfe4 · 0028
  1db2a01e21a37441a0ac10f9f8bb10c1fd6e5c5c0432635f42f0962c96601174 · 0029
  7a9f7ebe462847280168f9d509791517b10db0bba485a11536111825b55dcb97 (lead-verified state
  sha256:6836ba966e8d10a4662ca338f5fbeccaa0132ac36dd34846e35f4b2921f16dbe). kinako: 6e73bd5, its
  .mochiko status empty after every probe batch. Binary: target/debug/mochiko-cli 0.3.0, as built,
  no cargo. Drafts: s3/probe/log904 (0001–0029 plus the revised 9032–9035) and nine option logs
  (s3/probe/v-*), each validating 0 rejecting · 113 advisory. Files: the census pin, 1,342 (mochiko
  550, kinako 792).
post_s4_retake: >-
  Second write, after S4's 0024–0029 landed: no figure moved (21 home views byte-identical; the
  unchanged rewrite of all 1,342 files byte-identical, 737 allow · 605 deny today and 931 · 411 under
  the drafts; 194 destination checks identical), and H5 held. Its two text items, Q2's reason and
  the Lifecycle spelling, are folded into rows Q2, E1, F1 and F2 in this round.
revision_r1: >-
  R1 returned FAIL (7 blocking, 12 advisory); the lead folded all 19 into this one round and ruled
  the store field form (B3). Where each finding landed: B1 row H4 · B2 new row P1, and S1–S4 and Q1
  say how their text outside entries is bounded · B3 E1, F1, F2 spell the ruled forms · B4 S5 is a
  choice · B5 new row F3 · B6 new row F4 · B7 new row N1d, ledgers a home (N1c), the root narrowed
  (N1, N1a), L1 and L2 re-weighed and re-probed · A1 corrections folded into their rows, text_moved
  dropped · A2 R2 cites the lead's reading, R9 cited by its record · A3 S2 names the D5(iv) amendment
  · A4 N4 is a choice and N4, N5 state the consumer side · A5 no exempt field on spine or concerns ·
  A6 H1g says when (c) must follow · A7 plain words, options on every choice row, the two lists
  above · A8 new row H0a (and H0's count corrected, 11 review files not 13) · A9 the facts report
  corrected · A10 F1 cites the store's own removed forms · A11 S3 routes the store-home reword to
  wave 3 · A12 N1 lists the brainstorm archive as an option. The consequence list is recomputed
  under the revised drafts: 931 allow · 411 deny, as before.
no_fold_shape_row: "delta D1 answered it: there is no fold; entries are written in place"
stops_reached: "none. Every ruled move, merge and deletion has a checked route, every store fits one budget (so the per-entry-file split is not needed), and no row denies an existing file's next honest write without a route"
recommended_set: "0032 (field D3): the archive root, archive/ledgers/<FEAT-ID>/, archive/product-baselines/<slug>/, schema-views, strips, the nested-docs home, the spec replace. 0033 (field D4): runs. 0034 (field D2): product, product-architecture, product-concerns. 0035 (delta D4; intent names field D6): feature, epic, product-lane, the two contracts homes. S4's 0025 re-cut: rows E1 (spelling), F1–F4. Wave 3: P1's crate bound, N1d's rule wording"
rows:
  - id: H0
    subject: "the 13 homes the rulings leave alone: brainstorms-index, brainstorm-session, decisions, memory, features-index, feature-desk, product-contracts, product-design, spec-contracts, spec-map-delta, spec-stories, specs-index, and product-concerns up to its bound (row S5)"
    kind: ruled
    recommendation: "no change"
    reason: "every declared name is written or read by a live rule, or was declared from tree practice and has an instance (facts item 1: ten names have no writer). No ruling touches these sets. product-design also appears as row Q2, and the files under undeclared names in these homes as row H0a"
    rung: "rung 3: the existing homes already carry it"
    consequence: >-
      Every existing file allows an unchanged rewrite. Old faults stay frozen at their size, as
      today: template section budgets (spec.md, the feature map entries, tasks.md, map-delta
      entries, governance-intent.md), whole-file bounds (derivations, the map-delta README and
      selection card, five mochiko wave plans, several kinako feature and epic files), two
      build-log entries over 60 lines, and report-envelope faults in 86 kinako and 14 mochiko
      reports. One file denies for a reason outside this table: row X1
    carrier: "none"
  - id: H0a
    subject: "kinako files under names their home does not declare: 11 brainstorm review-*.md files and probes.md, specs/<slug>/stress-test.md, features/desk/<date-slug>/review.md, and two .gitkeep placeholders"
    kind: choice
    options:
      a: "keep them in place; each stays editable under the amnesty, and a new file of those names is refused"
      b: "move each into its home's reports/ at wave 4"
    recommendation: "a"
    reason: "No ruling moves them (field S11 names directories, not files). None of the 14 non-placeholder files carries report frontmatter, so under (b) every one is refused at reports/ ('the frontmatter is missing the required field `report`', probed ×14) and would need rewriting first. The .gitkeep files are placeholders either way"
    rung: "rung 1: nothing needs them moved"
    consequence: "(a): all 16 allow an unchanged rewrite (probed); a new review-*.md in a brainstorm folder is refused, with the home's reports/ as the route. (b): 14 of 14 refused as they stand"
    carrier: "none"
  - id: H1
    subject: "the feature home's set, .mochiko/features/<FEAT-ID>/"
    kind: ruled
    recommendation: "tasks.md (template tasks) · plan.md · requirements.md · design-closure.md · sufficiency-report.md · architecture.md · proposal.md · contest-brief.md (300 lines each) · gates.md (row H1g) · reports/. No sub-directory"
    reason: "delta D4 names this set and withdraws baseline-delta.md, data-model.md, constraints-and-decisions.md and contracts/. After S4's migrations, 11 rules and two tasks-template sections cite architecture.md, most as the run's signed drawing"
    rung: "rung 3: a replace of the existing home"
    consequence: >-
      kinako FEAT-001, FEAT-002 and FEAT-006: baseline-delta.md (3), data-model.md (2) and
      constraints-and-decisions.md (2) become undeclared names. Each stays editable under the
      amnesty until the wave-4 cleanup moves it (delta D5(ii), row N1c) or deletes it (D5(iii)); a new
      file of those names is refused. Their whole-file faults (930 to 4,744 lines against 300) leave
      with the name. reports/evidence/** is row R1, FEAT-006's reviews/ row L2. Every other file as
      today
    carrier: "0035"
  - id: H1g
    subject: "the feature home's gates.md bound (field OQ1: which feature-home files are entry-class)"
    kind: choice
    options:
      a: "whole file, 300 lines"
      b: "entries at ##, 177 lines each (the ## Gates section is one entry)"
      c: "one ### heading per gate and ### entries of 177: a gates-format rule change (testing-gap-finding, wave 3) plus a re-heading of FEAT-001's file (wave 4)"
      d: "keep the whole-file 150"
    recommendation: "a"
    reason: >-
      gates.md accumulates across a feature's runs and survives graduation
      (testing-gap-finding.gates-artifact-contract). FEAT-001's file is 146 of 150 lines, so under
      (d) its next gate is refused. Gates are **TEST:** lines, not headings, so (b) bounds the whole
      Gates section (108 lines now) and leaves less room than (a). (c) is the durable shape, since
      field D2 treats a file that grows across runs as cumulative, and under (a) such a file still
      ends at a whole-file refusal. (c) must therefore follow before any gates file passes about 250
      lines: FEAT-001's has 13 gates at about 15 lines each, so roughly seven more gates
    rung: "rung 4: sized to the one instance and its growth"
    consequence: "probed with FEAT-001's file grown by one 15-line gate: (a) allow; (d) refused ('`gates.md` is 161 lines against a whole-file bound of 150', as today), with no route but a re-rule; (b) allows until ## Gates passes 177"
    carrier: "0035"
  - id: H2
    subject: "the feature and epic contracts homes (lead ruling Q1, as corrected)"
    kind: ruled
    recommendation: "an empty file set in both, titled 'Interface contracts under a capability — withdrawn by delta D4, see product-contracts' and the same for an epic"
    reason: "delta D4 withdraws contracts/ from the feature and epic homes. The log has no step that deletes a home (migration.rs:154), so each stays with an empty set. The refusal prints 'Declared: none', never the title; mochiko-cli home prints the title"
    rung: "rung 3"
    consequence: >-
      A new file (features/FEAT-001/contracts/new-contract.md, probed) is refused: "`new-contract.md`
      is not a declared deliverable of `.mochiko/features/<FEAT-ID>/contracts/`. Declared: none …".
      The epic path is refused the same way. An existing file (features/FEAT-001/contracts/ipc.md,
      unchanged, probed) allows under the amnesty. All 14 kinako contracts files (8 feature, 6 epic)
      keep that until the wave-4 cleanup deletes the feature copies; the epic copies are closed
      record (R2). A wording defect for wave 3: the refusal names the withdrawn home itself as "the
      nearest home"
    carrier: "0035"
  - id: H3
    subject: "the epic home's set, .mochiko/epics/<EPIC-ID>/ (lead ruling Q2: one replace)"
    kind: ruled
    recommendation: "manifest.md · proposal.md · contest-brief.md · architecture.md · build-order.md · screens-and-flows.md (300 each) · reports/. No sub-directory"
    reason: "delta D4 and D6(b) withdraw the shared-baseline copies (data-model.md, constraints-and-decisions.md, quickstart.md) and contracts/; field D6 withdraws implement-log.md, whose place is the run folder (row N2)"
    rung: "rung 3"
    consequence: "kinako EPIC-001 and EPIC-002 are closed record (R2): their withdrawn files become undeclared names, editable under the amnesty, which no write will use. A new epic's implement-log.md is refused (probed: '`implement-log.md` is not a declared deliverable of `.mochiko/epics/<EPIC-ID>/`. Declared: `manifest.md`, …'). reports/* (7) allow as today"
    carrier: "0035, anchored delta D4; its intent names field D6"
  - id: H4
    subject: "the product-lane home's set, .mochiko/product/<slug>/"
    kind: choice
    options:
      a: "sufficiency-report.md and architecture.md (300 lines each) · reports/"
      b: "sufficiency-report.md (300) · reports/, with impl.design-outputs-home reworded in wave 3 so a lane run keeps no drawing"
    recommendation: "a"
    reason: >-
      Delta D4 and I5 withdraw baseline-delta.md and are silent on a lane run's drawing. After S4's
      migrations, impl.design-outputs-home says "the run's home keeps only architecture.md, the
      drawing the user signs" for every run, impl.artifact-home names the lane home as a lane run's
      home, the design phase fires on any sufficiency gap (impl.design-phase-fires-on-gap), and
      store elements may be keyed to a lane run (lane-<slug>). (a) gives that drawing its home, as
      the feature and epic homes already do. (b) needs a rule reword and leaves a lane run's sign-off
      with nothing to sign
    rung: "rung 3: the feature and epic homes already carry the name"
    consequence: "no lane folder exists in either tree. (a): a lane architecture.md allows (probed); today it is refused ('`architecture.md` is not a declared deliverable of `.mochiko/product/<slug>/`. Declared: `sufficiency-report.md`, `baseline-delta.md`'). (b): still refused. Under both, a new lane baseline-delta.md is refused"
    carrier: "0035 for (a); 0035 plus a wave-3 rule reword for (b)"
  - id: H5
    subject: "the spec home's baseline names, .mochiko/specs/<slug>/: data-model.md, constraints-and-decisions.md, quickstart.md"
    kind: choice
    options:
      a: "withdraw data-model.md and quickstart.md; keep constraints-and-decisions.md"
      b: "keep all three"
      c: "withdraw all three"
    recommendation: "a"
    reason: >-
      No ruling names the spec home's baselines. No rule writes a spec-home data-model.md or
      quickstart.md: spec.deliverable is spec.md plus stories/, and after delta D1 the design writes
      the product baselines in place. constraints-and-decisions.md is named by the floor rule
      authoring-technical-requirements.artifact-home, which S4's landed 0027 words as "a declared
      file in whichever home the run owns — the spec or the product baseline". Neither tree has any
      of the three under specs/. (c) would leave that floor rule naming a withdrawn file
    rung: "rung 1: two names have no writer"
    consequence: "(a): a new specs/<slug>/data-model.md is refused and a constraints-and-decisions.md allows (both probed). No existing file is affected"
    carrier: "0032"
  - id: N1
    subject: "the archive root's own set, .mochiko/archive/ (field D3, OQ3)"
    kind: choice
    options:
      a: "backlog-trail.md and mochiko's three frozen files (row N1b); no name pattern at the root (row N1a); ledgers/ and product-baselines/ as homes of their own (rows N1c, N1d); no evidence/ (row R1); no brainstorm archive"
      b: "as (a), plus a brainstorms/ sub-directory allowed by location for the brainstorm archive field D3 and OQ3 name"
    recommendation: "a"
    reason: >-
      Field D3 requires the archive declared. Its only writers today are the knowledge-management
      trail (KM module template :42) and the brownfield store reconstruction's dated snapshots (row
      N1d). No rule writes a brainstorm archive, so (b) would open a directory nothing fills, and a
      directory allowed by location admits any file but a report; a future writer's migration adds
      it. The archive is append-only and frozen, so its bounds are cited to the KM module, as the two
      index homes are
    rung: "rung 1: only named writers get a name"
    consequence: "today every archive write is refused (outside every home). After (a): backlog-trail.md and mochiko's four archive files allow; a new file at the root under any other name is refused, a 400-line file with report frontmatter included (probed); archive/evidence/** (166) is refused as an undeclared sub-directory (R1), its refusal now reading 'Declared: none'"
    carrier: "0032"
  - id: N1a
    subject: "groom snapshots (field S14): kinako's four FEAT-001 groom files at the archive root"
    kind: choice
    options:
      a: "no pattern at the root: the four stay where they are, editable under the amnesty; a future writer's migration declares its own name"
      b: "declare `<slug>-groom-<slug>.md` at the root"
      c: "declare `<slug>.md` at the root (the earlier recommendation)"
      d: "a grooms/ sub-directory allowed by location, for future snapshots"
    recommendation: "a"
    reason: >-
      No rule names a groom-snapshot path (the grooming skill names none, SKILL.md :46), so there is
      no writer to declare for, and the four are frozen records. (c) and (d) re-open the displacement
      door field D3 closed: with bounds cited elsewhere, any slug-named markdown at the root is
      admitted unchecked, and a 400-line file with report frontmatter allows there (probed) while the
      same body in a feature's reports/ is refused ('## Notes of note is 402 lines against a budget of
      15'). (b) narrows the door to names holding -groom- but does not close it: the same 400-line
      body named …-groom-….md allows (probed)
    rung: "rung 1: no writer"
    consequence: "(a): the four allow an unchanged rewrite (probed); any new root name is refused. (b): the four resolve as declared and allow; other new names are refused; groom-named files allow unchecked. (c): the four allow; any new slug-named .md allows unchecked. Rows L1 and L2 option (c) need (c)"
    carrier: "none for (a); 0032 for (b), (c) or (d)"
  - id: N1b
    subject: "mochiko's frozen archive files: REGISTRY.md, ROADMAP.md, provenance-frozen-2026-09-05.yaml"
    kind: ruled
    recommendation: "declare each by its literal name at the archive root"
    reason: "the lead's ruling RA6: they are frozen (CLAUDE.md History; .mochiko/archive/** is never edited), so no move is offered and the row declares them. Cost: three mochiko-only names ship in every consumer's archive set. Left undeclared, the amnesty would give the three the same verdict"
    rung: "rung 3"
    consequence: "allow (probed, unchanged rewrite); no next write"
    carrier: "0032"
  - id: N1c
    subject: "the archived-ledger shape (seam 2): the three baseline-delta.md ledgers, 8,068 lines, moved unchanged (delta D5(ii))"
    kind: choice
    options:
      a: "a home of its own, archive/ledgers/<FEAT-ID>/, declaring baseline-delta.md only (the lead's form for R1's B7)"
      b: "a declared name at the archive root, <FEAT-ID>-baseline-delta.md"
    recommendation: "a"
    reason: "(a) keeps the file name, so the 63 files that cite a ledger re-point by a prefix swap (delta D5(v)), and nothing else can land there: raw console output at archive/ledgers/FEAT-009-run1/ and at archive/ledgers/FEAT-009-run1/C1-15/ is refused (probed). (b) also takes the ledgers, but renames each file"
    rung: "rung 2"
    consequence: >-
      (a): a Write of each ledger at its destination allows (probed ×3). (b): allows (probed ×3).
      Move route for wave 4: a Write of the unchanged content at the destination, then git rm of the
      source (the shell check allows git rm, probed). git mv and cp are refused, because each
      touches a declared home as a write target (field S12; probed under the revised drafts). The user's own shell is the other route. A
      4,744-line Write must reproduce the file exactly, so the wave-4 pass checks it with a byte diff
    carrier: "0032 (shape); wave 4 (move)"
  - id: N1d
    subject: "the product-baseline snapshots, .mochiko/archive/product-baselines/<slug>/ (the lead's ruling on R1's B7)"
    kind: ruled
    recommendation: "a home of its own for each dated folder (the token set has no bare-date token; a date is a valid slug), declaring the names kinako's 2026-08-21 snapshot uses for the absorbed sources: ARCHITECTURE-prose.md, <FEAT-ID>-architecture.md and nfrs.md"
    reason: >-
      arch.tools-brownfield-reconstruction (and its skill twin) archives the sources a first store
      visit absorbs, repo ARCHITECTURE.md prose, per-feature architecture.md files and nfrs.md, to
      this folder, but names no file, so the names come from the one instance. The rule also lists
      "structural D-XXX rows" as a source, with no file name and no instance: a snapshot of them is
      refused until the rule names the file (routed to the lead for wave 3). spine-groom.md (the
      2026-09-22 folder) has no writer and stays editable under the amnesty
    rung: "rung 3: the rule's own sources, the tree's own names"
    consequence: "today all four files are refused (outside every home). After: the three 2026-08-21 files allow clean and spine-groom.md allows under the amnesty (probed); a new dated folder takes nfrs.md or FEAT-009-architecture.md (probed); decisions.md there, or raw output at archive/product-baselines/FEAT-009-run1/C1-15/, is refused (probed)"
    carrier: "0032; wave 3 for the rule's D-XXX file name"
  - id: N2
    subject: "the run folder home, .mochiko/runs/<run-id>/"
    kind: ruled
    recommendation: "raw output allowed, bounds cited to field D4, implement-log.md declared by name with its reason (the run log, deleted with the folder), no reports/"
    reason: "field D4 as amended (S2), D6 and V1; the run-id form per seams R5; the run-id token and the folder's controls landed in wave 1"
    rung: "rung 3"
    consequence: "neither repo's .gitignore carries .mochiko/runs/, so every run-folder write is refused (probed in both: 'the run folder is not git-ignored: … Add the line `.mochiko/runs/` …'). The line must land before a repo's first run: kinako at wave 4, this repo when a run first uses the folder. A bad run id is refused (probed); under N3 (a) its reason changes (see N3)"
    carrier: "0033"
  - id: N3
    subject: "operating docs the knowledge-management module nests under .mochiko/ on a collision ruling (field S7)"
    kind: choice
    options:
      a: "declare a home at .mochiko/ holding ROADMAP.md, BACKLOG.md, DECISIONS.md, ARCHITECTURE.md and GLOSSARY.md"
      b: "no home: such a write is refused until wave 3 re-points the nest target"
    recommendation: "a"
    reason: >-
      The KM module template (:168–173) lets setup nest a module doc at .mochiko/<NAME>.md when the
      repo root already has one. Without a home that write is refused with no route (probed: refused
      today and without the home, allowed with it). No instance exists in either tree. Costs of (a),
      probed: an undeclared top-level directory's refusal changes from "resolves to no declared home
      under `.mochiko/`, where the world is closed" to "`<dir>/` is not a declared sub-directory of
      `.mochiko/`. Declared: none" (both refuse, both name the run folder); a bad run id loses its
      own reason ("`not-a-run-id` is not a run key …" becomes "`runs/` is not a declared
      sub-directory of `.mochiko/`"), a crate wording fix for wave 3; seat S5's crate test pins on
      the closed-world text for a top-level path may move
    rung: "rung 1 holds (a live writer); rung 2: one home, five names"
    consequence: "(a): mochiko benchmarks/** (42) is refused as an undeclared sub-directory instead of as outside every home (row L3); nothing else changes. (b): no existing file changes"
    carrier: "0032"
  - id: N4
    subject: "the derived views, .mochiko/schema-views/ (field D3)"
    kind: choice
    options:
      a: "one home for each kind's folder, schema-views/<slug>/, declaring `<slug>.yaml` only"
      b: "schema-views/ with its seven sub-directories allowed by location (the earlier shape)"
    recommendation: "a"
    reason: >-
      Field D3 names schema-views; mochiko-cli views emit writes it, never a hand. The home ships to
      every consumer repo, where nothing writes views. Under (b) a sub-directory allowed by location
      admits any file but a report: a raw console.txt allows in schema-views/skills/ (probed in both
      repos). Under (a) only a slug-named .yaml is admitted, which is every view there is (80, all
      matching), and a new kind's folder needs no migration
    rung: "rung 2: one home, one name pattern"
    consequence: "today all 80 views are refused (outside every home). (a): all 80 allow, a new view allows, console.txt is refused, and views emit passes the shell check (probed). (b): the same, except console.txt allows"
    carrier: "0032"
  - id: N5
    subject: "the strip records, .mochiko/strips/ (field D3)"
    kind: choice
    options:
      a: "README.md and <slug>.md, bounds cited to strips/README.md"
      b: "the same names with a per-entry budget"
    recommendation: "a"
    reason: >-
      Field D3 names strips; the landing ritual writes them. The home ships to every consumer repo,
      where no rule writes strips, and there as here any slug-named markdown is admitted unchecked: a
      400-line file with report frontmatter allows in kinako's strips/ (probed). (b) would bound
      entries but not the text above a file's first heading (row P1), so it narrows that door only
      with P1 (b); it also needs a strip-entry measurement the census did not take (the largest file
      is implement.md, 3,197 lines), and a number invented for it would be enforced against work it
      cannot measure
    rung: "rung 4"
    consequence: "today all 101 files are refused (outside every home). After (a): allow (probed); a new strip file allows"
    carrier: "0032"
  - id: R1
    subject: "kinako's evidence trees: features/<FEAT-ID>/reports/evidence/** (161: FEAT-001 68, FEAT-002 93) and archive/evidence/** (166)"
    kind: ruled
    recommendation: "not declared; deleted at wave 4"
    reason: "field D9(b), S11"
    rung: "rung 1 fails for a home: nothing will write them"
    consequence: "every write is refused (an undeclared sub-directory, no amnesty). Deletion route: git rm -r and rm -r pass the shell check (probed)"
    carrier: "wave 4"
  - id: R2
    subject: "kinako's EPIC-001 and EPIC-002 directories: reviews/ (23: 15 and 8), landing/ (4), and their withdrawn top-level files"
    kind: ruled
    recommendation: "closed record, exempt from the census; not declared"
    reason: "delta D5 and I9 (authoring-epic.close-semantics): the two epics are closed record, untouched, exempt from the census. Field S11 lists their reviews/ and landing/ as 'declared by the census or moved out'; the lead read I9 as governing, since it is the later user ruling and names these directories (build-log.md, the plan-grades entry, on P3's B1)"
    rung: "rung 1 fails: closed record"
    consequence: "reviews/ and landing/: every write is refused (undeclared sub-directory); closed record, no next write. The withdrawn top-level files stay editable under the amnesty, unused. reports/* allow as today"
    carrier: "none"
  - id: L1
    subject: "kinako features/B53/** (9 cycle reports) and features/B61/** (1)"
    kind: choice
    options:
      a: "keep them in place as closed record, not declared"
      b: "delete them at wave 4; git history keeps them, and the links to them are annotated 'in history at <commit>' (field S11's own pointer clause)"
      c: "move each to the archive root as b53-<name>.md or b61-<name>.md (needs N1a (c))"
      d: "declare B53 and B61 as sub-directories of the feature map's home"
      e: "move each into a lane home's reports/ (for example product/lane-b53/reports/)"
    recommendation: "a"
    reason: >-
      Nothing writes these finished reports, so a refusal on every write is the right verdict for
      them, as for the closed epic directories (R2) and mochiko's benchmarks (L3). No move, no link
      rewrite. Choosing (a) amends field S11 in part: S11 says "declared by the census or moved out",
      and (a) adds "or kept in place as closed record" for these two directories, the reading the
      lead applied to the epic directories under I9. (b) keeps to S11's letter but removes the
      reports from the tree. (c) needs the archive-root door N1a (a) closes. (d) ships kinako's names
      in the plugin and still freezes 8 of the 10, since a directory allowed by location refuses
      report frontmatter. (e) fails the report envelope for all 10 (Notes of note over 15 lines; two
      lack a report field)
    rung: "rung 1: nothing needs them moved"
    consequence: "(a): every write refused (undeclared sub-directory), no next write (probed ×10). (b): git rm -r passes the shell check (probed). (c): refused ×10 under N1a (a), allowed ×10 under N1a (c) (probed). (d): 8 refused, 2 allow. (e): refused ×10"
    carrier: "none for (a); wave 4 for (b), (c) or (e); 0032 for (d)"
  - id: L2
    subject: "kinako features/FEAT-006/reviews/** (4 review reports)"
    kind: choice
    options:
      a: "keep them in place as closed record, not declared"
      b: "delete them at wave 4, links annotated as in L1 (b)"
      c: "move each to the archive root as feat-006-<name>.md (needs N1a (c))"
      d: "move each into features/FEAT-006/reports/"
      e: "declare reviews as a sub-directory of the feature home"
    recommendation: "a"
    reason: "as L1: no writer, so a refusal on every write is right, and (a) amends field S11 in part for this directory. (d) fails the report envelope for 3 of 4 (a missing report field; ## Failure narrative 62 and 65 lines against 15). (e) opens a reviews/ directory by location in every feature home, the door field D3 closed, and still freezes 3 of 4"
    rung: "rung 1"
    consequence: "(a): every write refused, no next write (probed ×4). (b): git rm -r passes the shell check (probed). (c): refused ×4 under N1a (a), allowed ×4 under N1a (c). (d): 3 refused, 1 allows (plan-artifacts-review.md). (e): 3 refused, 1 allows"
    carrier: "none for (a); wave 4 for (b), (c) or (d); 0035 for (e)"
  - id: L3
    subject: "mochiko .mochiko/benchmarks/** (42 files, the guardrails-vs-detail benchmark, last commit 2026-08-11)"
    kind: choice
    options:
      a: "leave undeclared, as closed record"
      b: "declare benchmarks as a sub-directory of the N3 home (needs N3 (a))"
    recommendation: "a"
    reason: "no rule writes it; 48 files in this repo cite the path, so a move breaks citations; it is finished work. Field S11 covers kinako's tree only, so (a) amends nothing"
    rung: "rung 1 fails for a declaration"
    consequence: "(a): every write refused, no next write. (b): allow ×42 (probed)"
    carrier: "none for (a); 0032 for (b)"
  - id: S1
    subject: "product data-model.md (seam 3)"
    kind: choice
    options:
      a: "one entry per ### (one entity or read model), 177 lines each, and the text of each ## section outside its entries bounded at 177 too"
      b: "as (a), with no bound on the section text"
      c: "as (a), with a tighter section bound such as 60 (4 lines over today's largest section)"
    recommendation: "a, with the Lifecycle line not counted (row E1)"
    reason: >-
      ### is where one entity sits; at ## the whole Entities section, 927 lines after the cleanup,
      would be one entry. The largest entity after the wave-4 cleanup's merges is 153 lines
      (BoundSession, an upper bound), 24 under the floor, and every merge shape the census measured
      fits. The text outside entries is the registers (Validation rules 56, Data sensitivity 47,
      Entity summary 36, Relationships 36), which grow with each entity; with no section bound (b), a
      ## section with no ### under it escapes every budget. (a) reuses the one number rather than
      inventing a second. Text above the first heading (29 lines today) is row P1
    rung: "rung 4: sized to the largest honest entry and the ruled floor"
    consequence: "the kinako file at HEAD allows an unchanged rewrite with no fault (today it allows only because its 1,206 lines against a whole-file 300 are frozen). Each merge shape of the cleanup, written at the real path, allows (probed); today each is refused ('`data-model.md` is 1273 lines against a whole-file bound of 300')"
    carrier: "0034"
  - id: S2
    subject: "product constraints-and-decisions.md, and whether its #### ids move to ###"
    kind: choice
    options:
      I: "re-level the 17 constraint and decision headings written at #### to ###, so every id is its own entry: 177 lines each; largest entry 77 (Live rows, the decision table), or 104 if a base decision written only as a table row is counted in it"
      L: "keep the heading levels as they are: 177 lines each; largest entry 169 (one ### heading holding six #### decisions), 8 lines of headroom"
    recommendation: "I, with the ## section text also bounded at 177 and the Lifecycle line not counted (row E1)"
    reason: >-
      Under L the #### decisions D-039 and D-045…D-049 are counted inside one ### group of 169 lines,
      so the next decision written after them at #### is refused (probed: 182 against 177), and so is
      any amendment of more than 8 lines to one of them; a new decision written at ### allows (probed).
      Under I every id is its own entry and the next decision allows (probed). Choosing I amends delta
      D5(iv) in part: D5(iv) says the fold-block move leaves "text unchanged, nothing else moved",
      and I also changes 17 heading levels, 2 of them outside the fold blocks (D-025, D-037). The
      annotation lands on the delta session's DECISIONS.md row at the joint landing, and the
      re-level rides the wave-4 kinako pass. Sections outside entries: Infrastructure provisioning
      61, Declarations 28, NFR 20, Where this baseline is deliberately silent 15. The stance line
      (`**Status:**`) stays counted. Text above the first heading (25 lines) is row P1
    rung: "rung 4"
    consequence: "the kinako file at HEAD allows an unchanged rewrite with no fault (today 733 lines against 300, frozen). Every merge shape of the cleanup at the real path allows under I and under L (probed); today they are refused (765 against 300). The duplicate-heading defect (X1) does not bite at 177"
    carrier: "0034 (budget); wave 4 (re-level)"
  - id: S3
    subject: "the architecture store's spine.md"
    kind: choice
    options:
      a: "entries at ##, 177 lines each; the architecture-spine template kept for shape (its section budgets are replaced, conform.rs:403)"
      b: "keep the template's section budgets (Container diagram 30, Elements 40, Key flows 30, As-built notes 15)"
    recommendation: "a, with no exempt field"
    reason: >-
      Field D2 bounds the architecture store per entry. Its four ## sections are its units, and the
      element rows and the diagram grow with the product: kinako's Container diagram is 92 lines and
      Elements 60, already over (b)'s numbers. Every line after the first ## is inside an entry, so
      no section bound applies; text above the first heading (42 lines) is row P1. The spine's
      lifecycle is its Status column, a cell inside each row (row F1), so an exempt field would match
      nothing
    rung: "rung 4"
    consequence: "(a): kinako's unchanged rewrite allows with no fault, and a new element row allows (probed). (b): a new element row is refused (probed today: '`## Elements` is 61 lines against a budget of 40'), with no route but a re-rule. mochiko's 8-line stub lacks three required headings, frozen under either option, as today. Under (a), authoring-architecture-store.store-home ('Take the spine's per-section budgets from mochiko-cli template architecture-spine') points seats at budgets the gate no longer applies; its reword is routed to wave 3"
    carrier: "0034"
  - id: S4
    subject: "the architecture store's concerns.md"
    kind: choice
    options:
      a: "entries at ##, 177 lines each; the architecture-concerns template kept for shape"
      b: "no size bound, as today (the template declares none)"
    recommendation: "a, with no exempt field"
    reason: "field D2 bounds the store per entry, and one concern row is one ## entry; the largest is 84 (AX-012), counting its `- **Status**:` line, which 33 of kinako's 34 entries carry. The template objects to a cap on the ledger's length, which (a) does not impose. No section bound applies (every line after the first ## is an entry); text above the first heading (63 lines) is row P1"
    rung: "rung 4"
    consequence: "(a) and (b): kinako's and mochiko's unchanged rewrites allow with no fault"
    carrier: "0034"
  - id: S5
    subject: "graduated concern files, concerns/<AX-ID>.md"
    kind: choice
    options:
      a: "keep the whole-file 150 (set in migration 0005)"
      b: "whole file, 177 lines"
      c: "entries at ##, 177 lines each"
    recommendation: "b"
    reason: >-
      Field D2 re-keyed the whole store per entry with the 177 floor after 0005 set 150. A concern
      graduates to its own file only on real depth (authoring-architecture-store.graduation-by-depth),
      so under (a) the deepest concerns get a smaller bound than a ledger row: a 160-line concern
      allows as a concerns.md entry and is refused as its own file (probed). The file holds one
      concern, so the whole file is its one entry, and (b) gives it the floor. (c) bounds each ##
      section, but a graduated file's depth may sit under its # title with no ## at all, which no
      entry budget counts: a 400-line file with no ## allows under (c) (probed; row P1)
    rung: "rung 4: the floor, one number"
    consequence: "no instance in either tree. A 160-line file: (a) refused ('`<AX-ID>.md` is 160 lines against a whole-file bound of 150', as today), (b) allow, (c) allow. A 400-line file with no ## heading: (a) and (b) refused, (c) allow (all probed)"
    carrier: "0034"
  - id: P1
    subject: "text above a store's first ## or ### heading (seam 3), for all five entry-bounded stores: S1–S4 and Q1"
    kind: choice
    options:
      a: "leave it unbounded, stated; the wave-4 watch (field S15) reads each store's preamble size at each landing"
      b: "a crate change in wave 3 that counts the preamble against the store's entry budget, 177, with the amnesty for any file already over"
      c: "the same crate change with a tighter number, such as 60"
    recommendation: "b"
    reason: >-
      The gate counts nothing above a store's first heading (conform.rs entry_spans: "text before the
      first heading is not counted at all"), and a ## store has no other text outside entries. So the
      preamble is the one place a store can still grow without limit: 300 lines added above the
      first heading of kinako's spine, concerns, data-model, constraints and quickstart each allow
      (probed). Today's preambles are spine 42 (already accreting: an EPIC-002 checkpoint note),
      concerns 63, data-model 29, constraints 25 and quickstart 12. (b) closes the gap with the one
      number every store already uses and freezes none of today's files; (c) would freeze concerns.md
      (63 over 60)
    rung: "rung 4"
    consequence: "no verdict moves until the crate change lands. Under (b) a 300-line preamble addition is then refused, and every file today stays under 177"
    carrier: "wave 3 (crate) for (b) or (c); none for (a)"
  - id: E1
    subject: "which marker lines do not count toward an entry's budget (seam 4)"
    kind: choice
    options:
      a: "the Lifecycle line only, on the three baselines: data-model, constraints-and-decisions, quickstart"
      b: "the Lifecycle, Raised and Weighed lines"
      c: "none"
    recommendation: "a"
    reason: >-
      Lifecycle is a one-line status every run flips, so it should not push an honest entry over;
      Raised and Weighed carry the entry's why (delta D3b), text the budget should weigh. A removed
      stub (seams R9) then counts 1 line. The spelling is `**Lifecycle:**` (bold, colon inside), the
      only form the gate's exempt matcher reads (conform.rs:916–921); the lead ruled that S4's one
      re-cut of 0025 aligns every baseline marker to that form (`**Lifecycle:**`, `**Raised:**`,
      `**Weighed:**`). The architecture store stays outside every option: its lifecycle is spine's
      Status column and concerns' `- **Status**:` field, which keeps the store's own `**Name**:` form
      (lead ruling on R1's B3), so the matcher never reads it. Exempting it would need a template form
      change or a crate matcher widening (wave 3), for one line per entry that today's budgets
      already count with room (concerns 84 of 177, spine 92 of 177). So spine and concerns carry no
      exempt field
    rung: "rung 2"
    consequence: "no store carries the fields today, so no verdict changes"
    carrier: "0034 (the exempt list); S4's 0025 re-cut (the spelling)"
  - id: Q1
    subject: "product quickstart.md (seam 5)"
    kind: choice
    options:
      a: "entries at ##, 177 lines each"
      b: "keep the whole-file 300"
    recommendation: "a, with the Lifecycle line not counted (row E1)"
    reason: "it is a product baseline that landings grow (187, then 197 lines). Field D2's reasoning holds: a whole-file bound only gets closer. Its 11 ## steps are the entries (largest 28), and one budget number serves every store. Every line after the first ## is inside an entry, so no section bound applies; text above the first heading (12 lines) is row P1"
    rung: "rung 4"
    consequence: "(a) and (b): kinako's unchanged rewrite allows. (b): the landing that takes it past 300 is refused"
    carrier: "0034"
  - id: Q2
    subject: "product design/design.md (seam 5)"
    kind: choice
    options:
      a: "keep the design-baseline template's section budgets"
      b: "entries"
    recommendation: "a"
    reason: >-
      No instance exists in either tree, so there is no honest entry to size, and the template's ten
      sections carry their own budgets. Since S4's migrations, design-phase and build-time decisions
      write the design truth part in place under the lifecycle marker (impl.baselines-in-place-marked,
      0024; impl.design-outputs-home, 0027), so marked entries will grow those sections. Who authors
      the truth part's content is still open (setup-product-agnostic OQ1, named in the template);
      that question is left to the rehoming brainstorm (seams record, routed items), and this row is
      re-ruled with it
    rung: "rung 4"
    consequence: "no existing file"
    carrier: "none for (a)"
  - id: F1
    subject: "where the Lifecycle marker sits in each baseline and store (delta D2; seams R8, R9)"
    kind: choice
    options:
      a: "the first line under the entry heading (details in the recommendation)"
      b: "the entry's last line"
      c: "on the heading line itself, for example `### Entity: BoundSession — proposed (FEAT-007)`"
    recommendation: >-
      a. data-model and quickstart: `**Lifecycle:** <status>` as the first line under the entry
      heading. constraints-and-decisions: the same first line, above the stance line (`**Class:** …
      · **Status:** …`), so the two never share a line. contracts/*: row F4. The architecture store
      takes no separate field: spine's Status column and concerns' `- **Status**:` field already
      carry the lifecycle (authoring-architecture-store.lifecycle-statuses). Removed forms: a
      baseline entry keeps its heading and `**Lifecycle:** removed` (seams R9,
      .mochiko/decisions/2026-09-29-landed-removal-stub.md); the store keeps 0025's own forms, a
      concern its heading and `- **Status**: removed`, a spine row cut to id, kind, name and status
      and not drawn
    reason: "one place across the baselines (delta D2, I4), with the stance kept separate; the first line is where a diff reader and the landing sweep look. (b) sits after a body of any length. (c) changes the heading text, which is the entry's key for the size check, so every flip would read as a new entry and lose its amnesty"
    rung: "rung 3: the store's own field is reused"
    consequence: "no verdict changes"
    carrier: "S4's one 0025 re-cut, after your ruling"
  - id: F2
    subject: "where Raised and Weighed sit on a build-raised entry (delta D3b)"
    kind: choice
    options:
      a: "directly after the Lifecycle line, one line each"
      b: "at the end of the entry"
    recommendation: "a. Baselines: `**Raised:** <cycle>` and `**Weighed:** <why, one line>`. Concern rows: `- **Raised**:` and `- **Weighed**:` fields in the store's own form (lead ruling on R1's B3). Spine: row F3"
    reason: "delta D3b puts the why on the entry; beside the marker a reader sees status and why together. The constraints file's Source · Shaped by · Impact stay where they are"
    rung: "rung 4"
    consequence: "no verdict changes; under E1 (a) each adds 2 counted lines"
    carrier: "S4's one 0025 re-cut"
  - id: F3
    subject: "where a build-raised spine element carries its why (seams R1)"
    kind: choice
    options:
      a: "two columns on the spine's element tables, Raised and Weighed, filled on a build-raised element and `—` elsewhere"
      b: "one Why column holding both, as `Raised <cycle> · Weighed <why>`"
      c: "no field on the spine: the why rides the element's concern row or the cycle report"
    recommendation: "a"
    reason: >-
      Seams R1 (user-ruled) records a ruling that changes the store "on that entry and nowhere else,
      its reason in the entry's own fields", and a spine element is one table row. (a) uses the same
      two names as every other entry and keeps one line per element; (b) is narrower but hides the
      names inside a cell. (c) was the earlier recommendation; it decides against seams R1, since a
      concern row is another entry and the cycle report discloses the decision without carrying its
      reason, so choosing (c) is an explicit supersession of R1 in part. kinako's spine already has a
      Derived from column the template does not declare; the gate reads no columns, so nothing is
      refused either way
    rung: "rung 3: the store's own row"
    consequence: "no verdict changes"
    carrier: "S4's one 0025 re-cut (the spine template's element table)"
  - id: F4
    subject: "what a contracts/* entry is and how it carries the marker (delta D2 routes this to the table)"
    kind: choice
    options:
      a: "a .md contract: the heading of the clause that changes, at the level its sibling clauses use (## or ###), with **Lifecycle:** as its first line. api.yaml: an `x-lifecycle` key on the changed operation or schema, with the same values; a removed one keeps only its summary and `x-lifecycle: removed`"
      b: ".md as (a); api.yaml carries no marker: the pinned-base diff finds its changes, and the run's architecture.md checkpoint table lists each changed operation with its status"
      c: "fix ### as the clause level in every .md contract (re-heading kinako's harness-store.md, which has no ###, at wave 4); api.yaml as (a)"
    recommendation: "a"
    reason: >-
      Contracts are outside the size scope (C1), so no budget depends on the heading level: the
      marker only has to sit where a reader and the landing sweep find it, directly under the
      clause's own heading. kinako's six contract files mix levels (ipc.md 9 ## and 18 ###;
      harness-store.md ## only), so a fixed level (c) forces re-heading for no gain. (b) leaves one
      baseline with no marker, so the sufficiency read and the orphan check cannot see an api.yaml
      change's status, against delta D2 ("every entry a run writes or amends carries a Lifecycle
      field"). An x- key is OpenAPI's own extension form, so the file stays a valid contract. Neither
      tree has an api.yaml today
    rung: "rung 3: the file's own clause headings; OpenAPI's own extension key"
    consequence: "no verdict changes: no size check reads contracts/*"
    carrier: "S4's one 0025 re-cut"
  - id: C1
    subject: "the size scope of contracts/* (delta D6a)"
    kind: ruled
    recommendation: "stated, unchanged: product-contracts keeps its bounds cited elsewhere, and no size check reads contracts/*"
    reason: "field S6; delta D6a"
    rung: "rung 3"
    consequence: "an in-place contract edit is not size-checked. kinako's product/contracts/ipc.md (657 lines) carries its own ## The EPIC-002 landing fold block (:420), which delta D5(iv) does not name; it gates nothing"
    carrier: "none"
  - id: M1
    subject: "the 177-line floor rests on a mis-measure (lead ruling Q4)"
    kind: choice
    options:
      a: "keep 177 as the floor"
      b: "restate the floor as the largest entry by the gate's own count: 153 after the cleanup (merged, an upper bound)"
    recommendation: "a"
    reason: "reports/review.md:142 counted BoundSession from line 646 to the next ### at line 823, across three ## sections; the gate counts 83 there. Per the lead's Q4 a mis-measure never lowers the floor, and 177 clears every measured entry, so neither option changes a recommended budget"
    rung: "rung 4"
    consequence: "none on any file"
    carrier: "none"
  - id: X1
    subject: "a known gate defect where it reaches a file (facts report, item 3)"
    kind: ruled
    recommendation: "no table change; the lead routes the crate fix to wave 3"
    reason: "when a file repeats a section heading, the gate compares a rewrite against the first standing fault of that name only, so a later, larger section of the same name makes an unchanged rewrite refused. kinako's features/FEAT-002/reports/driver-fix-report.md has four ## Notes of note (29, 26, 52 and 62 lines against 15); an unchanged rewrite is refused today and under this table alike"
    rung: "not applicable"
    consequence: "that file refuses every write until the crate fix; it is closed record, with no next write"
    carrier: "wave 3"
consequence_by_pattern:
  - "kinako · .mochiko/archive/backlog-trail.md · 1 · today deny (outside every home) · after allow · row N1"
  - "kinako · .mochiko/archive/evidence/** · 166 · today deny (outside every home) · after deny (undeclared sub-directory) · row R1"
  - "kinako · .mochiko/archive/feat-001-baseline-delta-groom-2026-09-22.md · 1 · today deny (outside every home) · after allow (undeclared name, kept editable) · row N1a"
  - "kinako · .mochiko/archive/feat-001-design-closure-groom-2026-09-22.md · 1 · today deny (outside every home) · after allow (undeclared name, kept editable) · row N1a"
  - "kinako · .mochiko/archive/feat-001-entry-groom-2026-09-22.md · 1 · today deny (outside every home) · after allow (undeclared name, kept editable) · row N1a"
  - "kinako · .mochiko/archive/feat-001-sufficiency-report-groom-2026-09-22.md · 1 · today deny (outside every home) · after allow (undeclared name, kept editable) · row N1a"
  - "kinako · .mochiko/archive/product-baselines/** · 3 · today deny (outside every home) · after allow · row N1d"
  - "kinako · .mochiko/archive/product-baselines/<slug>/spine-groom.md · 1 · today deny (outside every home) · after allow (undeclared name, kept editable) · row N1d"
  - "kinako · .mochiko/brainstorms/<slug>/probes.md · 1 · today allow (undeclared name, kept editable) · after allow (undeclared name, kept editable) · row H0a"
  - "kinako · .mochiko/brainstorms/<slug>/record.md · 8 · today allow · after allow · row H0"
  - "kinako · .mochiko/brainstorms/<slug>/review-A-crossexam.md · 1 · today allow (undeclared name, kept editable) · after allow (undeclared name, kept editable) · row H0a"
  - "kinako · .mochiko/brainstorms/<slug>/review-A-map.md · 1 · today allow (undeclared name, kept editable) · after allow (undeclared name, kept editable) · row H0a"
  - "kinako · .mochiko/brainstorms/<slug>/review-A-report.md · 1 · today allow (undeclared name, kept editable) · after allow (undeclared name, kept editable) · row H0a"
  - "kinako · .mochiko/brainstorms/<slug>/review-B-crossexam.md · 1 · today allow (undeclared name, kept editable) · after allow (undeclared name, kept editable) · row H0a"
  - "kinako · .mochiko/brainstorms/<slug>/review-B-map.md · 1 · today allow (undeclared name, kept editable) · after allow (undeclared name, kept editable) · row H0a"
  - "kinako · .mochiko/brainstorms/<slug>/review-B-report.md · 1 · today allow (undeclared name, kept editable) · after allow (undeclared name, kept editable) · row H0a"
  - "kinako · .mochiko/brainstorms/<slug>/review-map.md · 1 · today allow (undeclared name, kept editable) · after allow (undeclared name, kept editable) · row H0a"
  - "kinako · .mochiko/brainstorms/<slug>/review-report.md · 1 · today allow (undeclared name, kept editable) · after allow (undeclared name, kept editable) · row H0a"
  - "kinako · .mochiko/brainstorms/<slug>/review-survivors.md · 1 · today allow (undeclared name, kept editable) · after allow (undeclared name, kept editable) · row H0a"
  - "kinako · .mochiko/brainstorms/<slug>/review-verify.md · 2 · today allow (undeclared name, kept editable) · after allow (undeclared name, kept editable) · row H0a"
  - "kinako · .mochiko/brainstorms/index.md · 1 · today allow · after allow · row H0"
  - "kinako · .mochiko/decisions/.gitkeep · 1 · today allow (undeclared name, kept editable) · after allow (undeclared name, kept editable) · row H0a"
  - "kinako · .mochiko/decisions/<date-slug>.md · 23 · today allow · after allow · row H0"
  - "kinako · .mochiko/epics/<EPIC-ID>/architecture.md · 2 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/epics/<EPIC-ID>/build-order.md · 1 · today allow · after allow · row H0"
  - "kinako · .mochiko/epics/<EPIC-ID>/constraints-and-decisions.md · 2 · today allow (old faults frozen) · after allow (undeclared name, kept editable) · row H3, H2, R2"
  - "kinako · .mochiko/epics/<EPIC-ID>/contest-brief.md · 1 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/epics/<EPIC-ID>/contracts/README.md · 1 · today allow · after allow (undeclared name, kept editable) · row H3, H2, R2"
  - "kinako · .mochiko/epics/<EPIC-ID>/contracts/corpus-format.md · 1 · today allow · after allow (undeclared name, kept editable) · row H3, H2, R2"
  - "kinako · .mochiko/epics/<EPIC-ID>/contracts/engine-port.md · 1 · today allow · after allow (undeclared name, kept editable) · row H3, H2, R2"
  - "kinako · .mochiko/epics/<EPIC-ID>/contracts/ipc.md · 2 · today allow · after allow (undeclared name, kept editable) · row H3, H2, R2"
  - "kinako · .mochiko/epics/<EPIC-ID>/contracts/plugin-bridge.md · 1 · today allow · after allow (undeclared name, kept editable) · row H3, H2, R2"
  - "kinako · .mochiko/epics/<EPIC-ID>/data-model.md · 2 · today allow (old faults frozen) · after allow (undeclared name, kept editable) · row H3, H2, R2"
  - "kinako · .mochiko/epics/<EPIC-ID>/implement-log.md · 2 · today allow (old faults frozen) · after allow (undeclared name, kept editable) · row H3, H2, R2"
  - "kinako · .mochiko/epics/<EPIC-ID>/landing/** · 4 · today deny (undeclared sub-directory) · after deny (undeclared sub-directory) · row R2"
  - "kinako · .mochiko/epics/<EPIC-ID>/manifest.md · 2 · today allow · after allow · row H0"
  - "kinako · .mochiko/epics/<EPIC-ID>/proposal.md · 1 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/epics/<EPIC-ID>/quickstart.md · 2 · today allow · after allow (undeclared name, kept editable) · row H3, H2, R2"
  - "kinako · .mochiko/epics/<EPIC-ID>/reports/* · 5 · today allow · after allow · row H0"
  - "kinako · .mochiko/epics/<EPIC-ID>/reports/* · 2 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/epics/<EPIC-ID>/reviews/** · 23 · today deny (undeclared sub-directory) · after deny (undeclared sub-directory) · row R2"
  - "kinako · .mochiko/epics/<EPIC-ID>/screens-and-flows.md · 1 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/features/.gitkeep · 1 · today allow (undeclared name, kept editable) · after allow (undeclared name, kept editable) · row H0a"
  - "kinako · .mochiko/features/<FEAT-ID>.md · 6 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/features/<FEAT-ID>.md · 1 · today allow · after allow · row H0"
  - "kinako · .mochiko/features/<FEAT-ID>/architecture.md · 2 · today allow · after allow · row H0"
  - "kinako · .mochiko/features/<FEAT-ID>/baseline-delta.md · 3 · today allow (old faults frozen) · after allow (undeclared name, kept editable) · row H1"
  - "kinako · .mochiko/features/<FEAT-ID>/constraints-and-decisions.md · 1 · today allow · after allow (undeclared name, kept editable) · row H1"
  - "kinako · .mochiko/features/<FEAT-ID>/constraints-and-decisions.md · 1 · today allow (old faults frozen) · after allow (undeclared name, kept editable) · row H1"
  - "kinako · .mochiko/features/<FEAT-ID>/contest-brief.md · 1 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/features/<FEAT-ID>/contracts/corpus-format.md · 1 · today allow · after allow (undeclared name, kept editable) · row H2"
  - "kinako · .mochiko/features/<FEAT-ID>/contracts/engine-port.md · 2 · today allow · after allow (undeclared name, kept editable) · row H2"
  - "kinako · .mochiko/features/<FEAT-ID>/contracts/harness-store.md · 1 · today allow · after allow (undeclared name, kept editable) · row H2"
  - "kinako · .mochiko/features/<FEAT-ID>/contracts/ipc.md · 2 · today allow · after allow (undeclared name, kept editable) · row H2"
  - "kinako · .mochiko/features/<FEAT-ID>/contracts/plugin-bridge.md · 2 · today allow · after allow (undeclared name, kept editable) · row H2"
  - "kinako · .mochiko/features/<FEAT-ID>/data-model.md · 1 · today allow · after allow (undeclared name, kept editable) · row H1"
  - "kinako · .mochiko/features/<FEAT-ID>/data-model.md · 1 · today allow (old faults frozen) · after allow (undeclared name, kept editable) · row H1"
  - "kinako · .mochiko/features/<FEAT-ID>/design-closure.md · 2 · today allow · after allow · row H0"
  - "kinako · .mochiko/features/<FEAT-ID>/design-closure.md · 1 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/features/<FEAT-ID>/gates.md · 1 · today allow · after allow · row H1g"
  - "kinako · .mochiko/features/<FEAT-ID>/plan.md · 2 · today allow · after allow · row H0"
  - "kinako · .mochiko/features/<FEAT-ID>/plan.md · 1 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/features/<FEAT-ID>/proposal.md · 1 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/features/<FEAT-ID>/reports/* · 84 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/features/<FEAT-ID>/reports/* · 100 · today allow · after allow · row H0"
  - "kinako · .mochiko/features/<FEAT-ID>/reports/* · 1 · today deny (known gate defect, row X1) · after deny (known gate defect, row X1) · row X1"
  - "kinako · .mochiko/features/<FEAT-ID>/reports/evidence/** · 161 · today deny (undeclared sub-directory) · after deny (undeclared sub-directory) · row R1"
  - "kinako · .mochiko/features/<FEAT-ID>/requirements.md · 3 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/features/<FEAT-ID>/reviews/** · 4 · today deny (undeclared sub-directory) · after deny (undeclared sub-directory) · row L2"
  - "kinako · .mochiko/features/<FEAT-ID>/sufficiency-report.md · 1 · today allow · after allow · row H0"
  - "kinako · .mochiko/features/<FEAT-ID>/sufficiency-report.md · 2 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/features/<FEAT-ID>/tasks.md · 3 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/features/B53/** · 9 · today deny (undeclared sub-directory) · after deny (undeclared sub-directory) · row L1"
  - "kinako · .mochiko/features/B61/** · 1 · today deny (undeclared sub-directory) · after deny (undeclared sub-directory) · row L1"
  - "kinako · .mochiko/features/desk/<date-slug>/derivation.md · 1 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/features/desk/<date-slug>/derivation.md · 1 · today allow · after allow · row H0"
  - "kinako · .mochiko/features/desk/<date-slug>/reports/* · 3 · today allow · after allow · row H0"
  - "kinako · .mochiko/features/desk/<date-slug>/review.md · 1 · today allow (undeclared name, kept editable) · after allow (undeclared name, kept editable) · row H0a"
  - "kinako · .mochiko/memory/governance-intent.md · 1 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/memory/governance-ledger.md · 1 · today allow · after allow · row H0"
  - "kinako · .mochiko/memory/governance-trace-summary.md · 1 · today allow · after allow · row H0"
  - "kinako · .mochiko/memory/knowledge-management.md · 1 · today allow · after allow · row H0"
  - "kinako · .mochiko/product/architecture/concerns.md · 1 · today allow · after allow · row S4"
  - "kinako · .mochiko/product/architecture/spine.md · 1 · today allow (old faults frozen) · after allow · row S3"
  - "kinako · .mochiko/product/constraints-and-decisions.md · 1 · today allow (old faults frozen) · after allow · row S2"
  - "kinako · .mochiko/product/contracts/README.md · 1 · today allow · after allow · row C1"
  - "kinako · .mochiko/product/contracts/corpus-format.md · 1 · today allow · after allow · row C1"
  - "kinako · .mochiko/product/contracts/engine-port.md · 1 · today allow · after allow · row C1"
  - "kinako · .mochiko/product/contracts/harness-store.md · 1 · today allow · after allow · row C1"
  - "kinako · .mochiko/product/contracts/ipc.md · 1 · today allow · after allow · row C1"
  - "kinako · .mochiko/product/contracts/plugin-bridge.md · 1 · today allow · after allow · row C1"
  - "kinako · .mochiko/product/data-model.md · 1 · today allow (old faults frozen) · after allow · row S1"
  - "kinako · .mochiko/product/quickstart.md · 1 · today allow · after allow · row Q1"
  - "kinako · .mochiko/specs/<slug>/derivation.md · 2 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/specs/<slug>/map-delta/<FEAT-ID>.md · 3 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/specs/<slug>/map-delta/FEATURES.md · 1 · today allow · after allow · row H0"
  - "kinako · .mochiko/specs/<slug>/map-delta/README.md · 1 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/specs/<slug>/map-delta/selection-card.md · 1 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/specs/<slug>/map-delta/specs-index.md · 1 · today allow · after allow · row H0"
  - "kinako · .mochiko/specs/<slug>/prototype/** · 56 · today allow · after allow · row H0"
  - "kinako · .mochiko/specs/<slug>/spec.md · 3 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "kinako · .mochiko/specs/<slug>/stories/US-(n).md · 22 · today allow · after allow · row H0"
  - "kinako · .mochiko/specs/<slug>/stress-test.md · 1 · today allow (undeclared name, kept editable) · after allow (undeclared name, kept editable) · row H0a"
  - "kinako · .mochiko/specs/index.md · 1 · today allow · after allow · row H0"
  - "mochiko · .mochiko/archive/REGISTRY.md · 1 · today deny (outside every home) · after allow · row N1b"
  - "mochiko · .mochiko/archive/ROADMAP.md · 1 · today deny (outside every home) · after allow · row N1b"
  - "mochiko · .mochiko/archive/backlog-trail.md · 1 · today deny (outside every home) · after allow · row N1"
  - "mochiko · .mochiko/archive/provenance-frozen-2026-09-05.yaml · 1 · today deny (outside every home) · after allow · row N1b"
  - "mochiko · .mochiko/benchmarks/** · 42 · today deny (outside every home) · after deny (undeclared sub-directory) · row L3, N3"
  - "mochiko · .mochiko/brainstorms/<slug>/build-log.md · 5 · today allow · after allow · row H0"
  - "mochiko · .mochiko/brainstorms/<slug>/build-log.md · 1 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "mochiko · .mochiko/brainstorms/<slug>/inputs/** · 5 · today allow · after allow · row H0"
  - "mochiko · .mochiko/brainstorms/<slug>/record.md · 72 · today allow · after allow · row H0"
  - "mochiko · .mochiko/brainstorms/<slug>/referents/** · 5 · today allow · after allow · row H0"
  - "mochiko · .mochiko/brainstorms/<slug>/reports/* · 90 · today allow · after allow · row H0"
  - "mochiko · .mochiko/brainstorms/<slug>/reports/* · 14 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "mochiko · .mochiko/brainstorms/<slug>/research/** · 18 · today allow · after allow · row H0"
  - "mochiko · .mochiko/brainstorms/<slug>/synthesis.md · 8 · today allow · after allow · row H0"
  - "mochiko · .mochiko/brainstorms/<slug>/wave(n)-<slug>.md · 38 · today allow · after allow · row H0"
  - "mochiko · .mochiko/brainstorms/<slug>/wave(n)-<slug>.md · 5 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "mochiko · .mochiko/brainstorms/index.md · 1 · today allow · after allow · row H0"
  - "mochiko · .mochiko/decisions/<date-slug>.md · 53 · today allow · after allow · row H0"
  - "mochiko · .mochiko/memory/codebase-analysis.md · 1 · today allow · after allow · row H0"
  - "mochiko · .mochiko/memory/governance-intent.md · 1 · today allow (old faults frozen) · after allow (old faults frozen) · row H0"
  - "mochiko · .mochiko/memory/governance-ledger.md · 1 · today allow · after allow · row H0"
  - "mochiko · .mochiko/memory/governance-trace-summary.md · 1 · today allow · after allow · row H0"
  - "mochiko · .mochiko/memory/knowledge-management.md · 1 · today allow · after allow · row H0"
  - "mochiko · .mochiko/memory/primitive-cost-budgets.md · 1 · today allow · after allow · row H0"
  - "mochiko · .mochiko/product/architecture/concerns.md · 1 · today allow · after allow · row S4"
  - "mochiko · .mochiko/product/architecture/spine.md · 1 · today allow (old faults frozen) · after allow (old faults frozen) · row S3"
  - "mochiko · .mochiko/schema-views/commands/* · 6 · today deny (outside every home) · after allow · row N4"
  - "mochiko · .mochiko/schema-views/common/* · 3 · today deny (outside every home) · after allow · row N4"
  - "mochiko · .mochiko/schema-views/homes/* · 21 · today deny (outside every home) · after allow · row N4"
  - "mochiko · .mochiko/schema-views/labels/* · 2 · today deny (outside every home) · after allow · row N4"
  - "mochiko · .mochiko/schema-views/shelves/* · 1 · today deny (outside every home) · after allow · row N4"
  - "mochiko · .mochiko/schema-views/skills/* · 35 · today deny (outside every home) · after allow · row N4"
  - "mochiko · .mochiko/schema-views/templates/* · 12 · today deny (outside every home) · after allow · row N4"
  - "mochiko · .mochiko/strips/*.md · 101 · today deny (outside every home) · after allow · row N5"
---

## Notes of note

- The consequence list gives, per pattern, today's verdict and the verdict under the recommended
  rows. Today 605 of the 1,342 files are refused an unchanged rewrite; after, 411 are, each in row
  R1, R2, L1, L2, L3 or X1.
- Dependent options: L1 and L2 (c) need N1a (c); L3 (b) needs N3 (a). If you rule otherwise, the
  dependent row falls back to its recommendation.
- Routed to wave 3 by this table: P1's crate bound, N1d's rule wording, and S3's store-home reword;
  H2's and N3's refusal wording and X1's defect were already wave 3's.
- One installed-gate false deny in the post-S4 re-take, on a heredoc writing a scratch probe script
  whose payload held a `cp` string ("This command's write target resolves under a declared artifact
  home: `.mochiko/features/FEAT-001/baseline-delta.md`"). Not retried in shape; the script was
  written with Write (lead-accepted); nothing reached any .mochiko/ path. None in this round.
