---
report: review
wave: 2
seat: R1
subject: >-
  R1's review of the census table (reports/w2-census-table.md, post_s4_retake included) and its
  facts (reports/w2-census-facts.md, post_s4_correction included) against wave plan §2 and §4,
  the records it cites and the last eight build-log entries. R1 wrote nothing in this wave.
verdict: FAIL
counts: {blocking: 7, advisory: 12}
pins:
  mochiko: "joint-hook-delta at 91b3838 plus migrations 0024–0029; the six sha256s match the table's post_s4_retake.pin"
  drafts: "S3's s3/probe/log903b = the live 0001–0029 byte-for-byte plus 9032–9035; migrate validate: 0 rejecting · 113 advisory"
  kinako: "6e73bd5; git --no-optional-locks status --porcelain -- .mochiko printed nothing before and after every probe batch"
  binary: "target/debug/mochiko-cli 0.3.0 · grammar 1..2, as built; no cargo; the 0.2.0 on PATH unused"
  views: "fresh emits: post-S4 log 80 documents, its 21 home views byte-identical to .mochiko/schema-views; log903b 85 documents, only the 9 replaced and 5 new homes differ"
  method: >-
    today = check under plugins/mochiko/migrations (0001–0029); after = check under log903b. Every
    probe is a dry run (check --path --content -, or --hook-json for shell payloads); nothing was
    written under any .mochiko/ path. Scratch: r1/probe.sh, r1/dest.py, r1/shell.py, r1/entries.py
    and their .out files.
checks:
  c1_every_pattern_has_a_row: "PASS. An independent find over both trees found no directory or file pattern without a row (kinako: the 9 non-evidence archive files, B53, B61, FEAT-006/reviews, the epic reviews/ and landing/, both evidence trees; mochiko: archive, benchmarks, schema-views, strips)"
  c2_home_sets_match_post_delta_write_sets: "FAIL. Feature, epic, spec and product sets match the post-S4 rules; the product-lane set does not (B1)"
  c3_budgets_admit_largest_entry: "FAIL on one row. Independent counts match the facts (data-model M 153, constraints MI 77 and ML 169, quickstart 28, concerns 84, spine 92 at ##) and every candidate allows under 177 with no fault. S5 keeps 150, below the 177 floor (B4). M1 carries the floor as the lead ruled"
  c4_seams_2_to_5_rows: "FAIL on seam 3. Seam 2 is N1c, seam 4 E1, seam 5 Q1 and Q2. Seam 3's text-outside-entries statement is missing for S3, S4 and Q1 and incomplete for all five stores (B2)"
  c5_contracts_scope_stated: "PASS. Row C1"
  c6_no_row_decides_the_ruled: "FAIL. S5 is labelled ruled on a ruling D2 re-keyed (B4). F2 departs from R1 and from RA8 (B3, B5). Otherwise every choice row carries a recommendation and a reason, and the ruled rows cite correctly (H1, H2, H3, H4, N2, R1, X1); R2's citation is incomplete (A2)"
  c7_consequence_column: "PASS on every verdict sampled (below), FAIL on completeness where a user choice turns on it (B7)"
rulings_carried:
  q1_as_corrected: "carried by H2: new files deny with 'Declared: none'; existing files allow under the amnesty; home prints the withdrawn title (probed)"
  f_carrier_0025_recut_bold: "partly. F1 and F2 name the 0025 re-cut and F1 spells **Lifecycle:** in bold, but post_s4_retake.text_moved still calls the carrier the lead's open choice, and F2 does not spell Raised and Weighed in the ruled form (B3)"
  s11_vs_i9: "carried by R2 as ruled I9; the lead's reading is not cited (A2)"
  undeclared_subdirectories_deny_outright: "carried by R1, R2, L1, L2 and L3; probed on evidence, epic reviews/, B53 and benchmarks"
  r8: "carried; F1 cites it and no size figure turns on it"
  r9: "carried by F1 and E1 (a stub counts 1 line under E1 (a)); its record is .mochiko/decisions/2026-09-29-landed-removal-stub.md, cited as 'seams R9'"
  archive_frozen_declare_only: "carried by N1b"
consequence_sample:
  - "kinako archive/evidence/FEAT-001-run4/C1-15/console-rows.txt (R1, evidence): expected deny closed world, then deny undeclared sub-directory; got both"
  - "kinako features/FEAT-001/reports/evidence/C1-7/a2-build.txt (R1, evidence): expected deny, deny (sub-directory); got both"
  - "kinako epics/EPIC-001/reviews/architecture-feasibility.md (R2, I9): expected deny, deny (sub-directory); got both"
  - "kinako epics/EPIC-002/data-model.md and implement-log.md (H3, R2, I9): expected allow (standing faults frozen), then allow (undeclared name); got both"
  - "kinako features/B53/cycle-01-report.md (L1, legacy): expected deny, deny; got both. Destination archive/b53-cycle-01-report.md: today deny, after allow"
  - "kinako product/data-model.md (S1, store): expected allow (1206 against 300 frozen), then allow clean; got both. Plus a 45-line entity: today deny (1251 against 300), after allow. S3's D5 candidate M: today deny (1273), after allow"
  - "kinako product/constraints-and-decisions.md (S2): allow frozen, then allow clean; S3's MI and ML candidates: today deny (765), after allow"
  - "kinako product/architecture/spine.md (S3): allow frozen, then allow clean. Plus one element row: today deny ('## Elements is 61 lines against a budget of 40'), after allow"
  - "kinako features/FEAT-001/baseline-delta.md (H1, N1c): allow frozen, then allow (undeclared name). Destination archive/ledgers/FEAT-001/baseline-delta.md: today deny, after allow. Shell leg: git rm allows, git mv denies"
  - "kinako features/FEAT-001/gates.md (H1g): allow, allow. Plus a 14-line gate: today deny (161 against 150), after allow"
  - "kinako features/FEAT-002/reports/driver-fix-report.md (X1): deny, deny ('## Notes of note is 52 lines against a budget of 15'); got both"
  - "kinako archive/feat-001-entry-groom-2026-09-22.md (N1a) and archive/product-baselines/2026-09-22/spine-groom.md (N1): today deny, after allow; got both"
  - "kinako features/FEAT-001/contracts/ipc.md (H2): allow, then allow (undeclared name). A new contracts/new-contract.md: today allow, after deny 'Declared: none'"
  - "mochiko strips/README.md (N5), schema-views/homes/product.yaml (N4), archive/REGISTRY.md (N1b): today deny closed world, after allow; got all three"
  - "mochiko benchmarks/guardrails-vs-detail/report/final-verdict.md (L3, N3): deny closed world, then deny undeclared sub-directory of .mochiko/; got both"
  - "mochiko product/architecture/spine.md (S3): allow with three required headings missing, frozen, under both; got both"
  - "also probed as the rows state: H3 a new epic implement-log.md denies · N2 kinako runs/FEAT-001-run5/implement-log.md denies on the missing .gitignore line · H5 spec data-model.md denies, spec constraints-and-decisions.md allows · N3 kinako .mochiko/ROADMAP.md: today deny, after allow · R1 git rm -r and rm -r allow · N4 views emit passes the shell leg"
findings:
  - id: B1
    severity: blocking
    row: H4
    defect: >-
      The product-lane home's set misses the run's drawing. After S4, impl.design-outputs-home (no
      scope condition) says "the run's home keeps only architecture.md, the drawing the user
      signs", and impl.artifact-home names the lane home as a lane run's home. The design phase
      fires on any sufficiency gap (impl.design-phase-fires-on-gap, no scope condition), and store
      elements may be keyed `proposed (lane-<slug>)` (templates/architecture-store, 0025
      lifecycle-statuses). H4 declares only sufficiency-report.md. Probed: kinako
      product/lane-probe/architecture.md denies after ("not a declared deliverable of
      `.mochiko/product/<slug>/`. Declared: `sufficiency-report.md`"). Delta D4 names the feature
      and epic drawings and is silent on the lane's, so this is a choice no row puts.
    fix: >-
      Add a choice row for the lane home: (a) declare architecture.md (300, as in the feature and
      epic homes) or (b) scope impl.design-outputs-home so a lane run keeps no drawing, with the
      rule change routed to S4 or wave 3. H4's consequence then names the lane drawing's verdict.
  - id: B2
    severity: blocking
    row: S1, S2, S3, S4, Q1 (seam 3)
    defect: >-
      Text before a store's first heading escapes every budget: the crate counts it nowhere
      (conform.rs entry_spans: "text before the first heading is not counted at all"), and a
      `##`-entry store has no section bound at all. Probed under log903b: 300 lines added to the
      preamble of kinako spine.md, data-model.md and quickstart.md each allow. Today's preambles
      are data-model 29 lines, constraints 25, quickstart 12, spine 42 and concerns 63, and
      spine's is already accreting (an EPIC-002 checkpoint annotation). Wave plan §2b requires
      section_max_lines "or the reason there is none"; S3, S4 and Q1 state neither, and S1 and S2
      state no bound for the preamble.
    fix: >-
      Each store row states how its preamble and non-entry text are bounded. For the ##-entry
      stores, the reason there is no section bound (every line after the first heading is inside
      an entry). For every store, that the preamble is unbounded, with its size today. Then either
      accept that with a reason, or put a crate bound on the preamble to the user as a choice
      routed to wave 3.
  - id: B3
    severity: blocking
    row: F2 (and post_s4_retake.text_moved, E1)
    defect: >-
      The lead's RA8 ruling (build-log, 2026-09-29 post-S4 re-take entry) aligns every marker field
      in S4's files to the bold `**Name:**` form (`**Lifecycle:**`, `**Raised:**`, `**Weighed:**`)
      in the one 0025 re-cut. F2 gives concerns rows `- **Raised**:` and `- **Weighed**:` (colon
      outside the bold), does not spell the baseline Raised and Weighed lines at all, and
      text_moved still says the re-cut-versus-new-file choice "is the lead's", although the lead
      has ruled it.
    fix: >-
      F2 spells `**Raised:** <cycle>` and `**Weighed:** <one line>` on the baselines. For concerns
      rows it either uses `- **Raised:**` and `- **Weighed:**` or names the exception (the row's
      existing `- **Status**:` convention) for the lead to rule. text_moved and E1 state RA8 as
      ruled: the one 0025 re-cut, bold spelling.
  - id: B4
    severity: blocking
    row: S5
    defect: >-
      S5 is labelled ruled on 0005, but field D2 re-keyed the whole architecture store per entry
      after 0005, with the 177 floor (OQ1, S6, lead Q4). Keeping 150 puts the one-entry graduated
      file below the floor, and graduation is by depth (authoring-architecture-store
      graduation-by-depth), so the deepest concerns get the smallest bound. Probed under log903b: a
      160-line concern allows as a concerns.md entry and denies as concerns/AX-099-probe.md
      ("162 lines against a whole-file bound of 150").
    fix: >-
      Make S5 a choice row: (a) keep 150; (b) whole-file max_lines 177; (c) form: entries at `##`,
      177, Lifecycle exempt. State the inversion probe, and recommend (b) or (c) to match the
      floor.
  - id: B5
    severity: blocking
    row: F2
    defect: >-
      "spine rows carry none: … the why rides the element's concern row or the cycle report"
      decides against seams R1 (user-ruled): a ruling that changes the architecture store is
      "recorded on that entry and nowhere else, its reason in the entry's own fields". A concern
      row is another entry. Under delta D3b the cycle report discloses the decision, but it does not
      carry the reason.
    fix: >-
      Give a build-raised spine element its why on its own row, for example in its `Derived from`
      cell or a Raised/Weighed pair the spine template admits. Or put the exception to the user as
      an explicit supersession in part of R1, with that option named.
  - id: B6
    severity: blocking
    row: F1, F2 (contracts/*)
    defect: >-
      F1 places '**Lifecycle:** <status>' as "the first line under the entry heading" for
      contracts/*, but no contracts file has a declared entry heading (C1: outside the size scope,
      no form). kinako's six contracts mix `##` and `###` clauses (ipc.md: 9 and 18), and
      product-contracts declares api.yaml, where a bold markdown line would break the YAML. Delta D2
      routes exactly this, the fields contracts/* entries gain, to this table.
    fix: >-
      Name what a contracts entry is: the heading level of one contract clause in a `.md` file, and
      the marker form for api.yaml (for example an `x-lifecycle` key on the operation or schema).
      Or state that api.yaml carries no marker and how the diff read finds its changes.
  - id: B7
    severity: blocking
    row: N1, N1a, N1c (and L1, L2, which depend on N1a)
    defect: >-
      The recommended archive shape reopens the displacement door that field D3 closed, and no
      consequence line says so. (i) The root `<slug>.md` pattern, with bounds elsewhere, admits a
      new 400-line report-frontmatter file at archive/feat-009-run1-console.md (probed allow). The
      same body in the feature's reports/ denies ("## Notes of note is 402 lines against a budget
      of 15"). (ii) product-baselines/ and ledgers/ are allowed by location, so they admit the raw
      evidence R1 deletes: kinako's console-rows.txt allows at
      archive/product-baselines/FEAT-009-run1/C1-15/ and at archive/ledgers/FEAT-009-run1/, and the
      archive/evidence deny itself prints "Declared: `product-baselines`, `ledgers`". N1a's stated
      cost ("admits any slug-named .md") names neither.
    fix: >-
      Declare the two sub-directories as homes with file sets rather than by location: ledgers at
      `[.mochiko, archive, ledgers, <FEAT-ID>]` holding baseline-delta.md, and product-baselines
      holding the absorbed-source names arch.tools-brownfield-reconstruction writes, `.md` only.
      Then either narrow the root to literal names, or put N1a's full cost to the user and re-weigh
      (b). Re-probe L1 and L2 (a) against the result.
  - id: A1
    severity: advisory
    row: Q2, E1, F1, F2
    defect: "post_s4_retake.text_moved holds corrections (Q2's reason, the RA8 status) in the frontmatter, while the rows the user reads keep the old text"
    fix: "fold each correction into its row in the revision round and drop the text_moved list"
  - id: A2
    severity: advisory
    row: R2
    defect: "field S11 lists epics/EPIC-001/reviews, EPIC-001/landing and EPIC-002/reviews as 'declared by the census or moved out'. R2 cites I9 but not the lead's reading that I9 governs over S11 (build-log, plan-grades entry, P3 B1), so a reader of S11 sees a conflict"
    fix: "cite the lead's reading in R2's reason; cite R9 by its own record path in F1 and E1"
  - id: A3
    severity: advisory
    row: S2
    defect: "option I re-levels 2 base headings (D-025, D-037) against D5(iv)'s 'text unchanged, nothing else moved'; the row calls it the user's call but not a supersession in part of a ruled clause"
    fix: "say that choosing I amends D5(iv) in part, and name where the annotation lands (the delta record's DECISIONS.md row or kinako's wave-4 item)"
  - id: A4
    severity: advisory
    row: N4, N5
    defect: "both homes ship to every consumer. In kinako after the drafts, strips/feat-009-run1-console.md with report frontmatter allows, and schema-views/skills/console.txt (raw) allows (probed). N4's shape (sub-directories allowed by location) is S3's design, labelled ruled with D3's declaration"
    fix: "state the consumer-side consequence. Consider sub-homes declaring `<slug>.yaml` for schema-views; label N4's shape a choice"
  - id: A5
    severity: advisory
    row: S3, S4, E1
    defect: "entry_exempt_fields [Lifecycle] is inert on spine and concerns, whose lifecycle is the Status column or '- **Status**:' field (F1). E1's reason says so; S3 and S4 recommend it anyway"
    fix: "drop it from the two store entries, or say in S3 and S4 that it is inert"
  - id: A6
    severity: advisory
    row: H1g
    defect: "(a) keeps a whole-file cap on a file D2's own definition classes as cumulative (it grows across runs), so its growth ends in the same deny D2 was ruled to end"
    fix: "say so in the reason, with the size at which (c) must follow"
  - id: A7
    severity: advisory
    row: S1, S2, E1, X1, N1c, F1, F2, N1
    defect: "readability: S1 and S2 cite S3's internal variant codes (M, P, Mu, ML, MI, PL, PI, MId, MIs, MLs, 'sensitivity d') and RA1, RA3, RA8, V2, D5(v), which no user can read without the facts. F1, F2 and N1 are choice rows with no options listed. 21 choices sit among 33 rows and a 137-line list"
    fix: "one plain clause per code (for example 'every merge variant fits'); options on every choice row; a short choices list at the top (s3/phase-a-summary.md has one)"
  - id: A8
    severity: advisory
    row: H0
    defect: "H0 holds an unstated choice: kinako's 18 legacy files under undeclared names (13 brainstorm review-*.md, probes.md, stress-test.md, desk review.md, two .gitkeep) are left in place under the amnesty"
    fix: "a one-line row, or H0's reason saying that no ruling moves them (S11 names directories) and that leaving them is the recommendation"
  - id: A9
    severity: advisory
    row: X1 (facts ra3)
    defect: "the facts say the first ## Notes of note is 52 lines. The four sections are 29, 26, 52 and 62 lines, and the deny names the third. X1 is unaffected"
    fix: "correct the facts line"
  - id: A10
    severity: advisory
    row: F1
    defect: "'A removed entry: its heading plus **Lifecycle:** removed' reads as one form for every store, but the landed 0025 lifecycle-statuses gives the store its own removed forms (a concern keeps its heading and Status: removed; a spine row is cut to id, kind, name and status)"
    fix: "cite 0025's store forms beside the baseline stub"
  - id: A11
    severity: advisory
    row: S3
    defect: "under S3 (a), authoring-architecture-store.store-home ('Take the spine's per-section budgets from mochiko-cli template architecture-spine') tells seats to read budgets the gate no longer applies"
    fix: "route the reword to wave 3 in S3's consequence"
  - id: A12
    severity: advisory
    row: N1
    defect: "field D3 and OQ3 name a brainstorm archive among the archive's contents. N1 declines to declare one (no writer) inside a single recommendation, with no option the user can see"
    fix: "make it an explicit option of N1, with the reason 'no writer; a future writer's migration adds it'"
gate_denies: "none from the installed gate on any command. The branch binary's dry run of this report's first draft denied (## Failure narrative 17 lines against 15); the draft was trimmed and the next dry run allowed before the one Write"
re_review_round_1:
  subject: >-
    The one re-review of the revised table (reports/w2-census-table.md, 823 lines, sha256
    9aadd497d19fc07a…, modified 14:19) and the facts' one Edit (sha256 122e13510231fb07…), against
    this report's 19 findings, the lead's store-field ruling (build-log, R1-review entry) and S3's
    drafts in s3/probe/log904 and the nine option logs s3/probe/v-*.
  verdict: FAIL
  counts: {blocking_closed: 7, blocking_new: 1, advisory_folded: 12, advisory_new: 5}
  pins: "log904 = the live 0001–0029 byte-for-byte plus the revised 9032–9035; log904 and v-h4-b, v-s5-a, v-s5-c, v-n1a-c, v-n4-a each validate 0 rejecting · 113 advisory; kinako 6e73bd5, .mochiko status empty before and after; no cargo; no installed-gate deny"
  blocking_closed:
    B1: "closed by H4, now a choice: (a) architecture.md in the lane home. Re-probed: a lane drawing allows under log904 and is refused under v-h4-b"
    B2: "closed by P1, a choice whose recommended option is a wave-3 crate bound at 177, and by S1–S4 and Q1, which each state their preamble size and section treatment. Re-probed: 300 preamble lines still allow on all five stores under log904, as P1 says until the crate change"
    B3: "closed. F2 spells `**Raised:**` and `**Weighed:**` on the baselines and `- **Raised**:` and `- **Weighed**:` in the store, per the lead's store-field ruling. E1 states RA8 as ruled; text_moved is gone"
    B4: "closed by S5, now a choice; (b), a whole-file 177, is recommended. Re-probed: a 160-line concern file is refused under v-s5-a, allows under log904 and v-s5-c; a 400-line file with no ## heading is refused under log904 and allows under v-s5-c, as the row says"
    B5: "closed by F3, a choice: (a) Raised and Weighed columns on the element row; (c) named as a supersession of seams R1 in part"
    B6: "closed by F4, a choice: (a) the changed clause's own heading, and an x-lifecycle key in api.yaml"
    B7: "closed. The archive root declares literal names only; ledgers/<FEAT-ID>/ and product-baselines/<slug>/ are homes with file sets. Re-probed under log904: a 400-line report dump at the root is refused; raw console output is refused at product-baselines/FEAT-009-run1/C1-15/, product-baselines/feat-009-run1/, its C1-15/, ledgers/FEAT-009-run1/ and its C1-15/; the archive/evidence refusal now prints 'Declared: none'. Each FEAT-001 and FEAT-002 ledger allows at its destination. L1 and L2 (a) are refused in place, as closed record; L1 (c) is refused under log904 and allows under v-n1a-c. git rm allows, git mv is refused. All 80 views and 101 strips allow unchanged"
  advisory_folded: "A1–A12, each where revision_r1 says. Checked: text_moved gone; R2 cites the lead's I9 reading; S2 names the D5(iv) amendment; N4 is a choice (raw console.txt refused under log904, allows under the (b) log); spine and concerns carry no exempt field; H1g says when (c) must follow; every choice row lists options; H0a added (one review-map.md re-probed: refused at reports/, as the row says); the facts read 29, 26, 52 and 62; F1 cites the store's removed forms; S3 routes the store-home reword; N1 offers the brainstorm archive"
  findings:
    - id: RB1
      severity: blocking
      row: N1a, N1d
      defect: >-
        Five of kinako's nine non-evidence archive files end neither declared nor moved out: the
        four groom snapshots under N1a (a), and spine-groom.md, which N1d (a ruled row) leaves
        undeclared. Field D9(b), user-ruled and corrected to 9 files at S14, says the archive files
        "are sorted into the declared archive/ set or out of .mochiko/" (record.md:369). So N1a (a)
        amends D9(b) in part, and N1d carries an amendment inside a ruled row. Neither says so,
        while L1 and L2 name their S11 amendment.
      fix: >-
        N1a says that (a) amends field D9(b) in part, and that (b), or the four literal names,
        keeps to it. spine-groom.md's disposition becomes a choice: declare it in N1d's set, delete
        it, or keep it with the amendment named. choices_at_a_glance flags it too (RA1).
    - id: RA1
      severity: advisory
      row: choices_at_a_glance
      defect: "the list the user reads first does not mark the recommendations that amend an earlier user ruling in part: L1 and L2 (field S11), S2 (delta D5(iv)), and N1a (field D9(b), RB1). The rows say so; the list does not"
      fix: "add 'amends field S11 in part' (and the like) to those four entries"
    - id: RA2
      severity: advisory
      row: N1d
      defect: "the name set (ARCHITECTURE-prose.md, <FEAT-ID>-architecture.md, nfrs.md) is S3's pick from one kinako snapshot, in a ruled row. The rule archives 'repo ARCHITECTURE.md prose' under no named file, so a reconstruction that keeps a source's own name is refused: ARCHITECTURE.md and architecture.md in a new dated folder are both refused (probed). The refusal lists the declared names, so renaming is the route"
      fix: "say so in N1d's consequence, or add ARCHITECTURE.md to the set; the lead decides whether the name set is ruled or a choice"
    - id: RA3
      severity: advisory
      row: N5, N4
      defect: "N5 keeps in every consumer's strips/ the <slug>.md door that N1a closes at the archive root: a 400-line report dump allows in kinako's strips/ (re-probed under log904). Neither N5 option closes it. N4 (a) admits a report-shaped dump.yaml in schema-views/skills/ (probed allow), a narrow door"
      fix: "add an option that closes it (a wave-3 crate change running the report check on a pattern-named file in a home whose bounds are cited elsewhere), or say that none exists short of crate work"
    - id: RA4
      severity: advisory
      row: terms (H2, H3, M1, N1b)
      defect: "'lead Qn' and 'RAn' are ambiguous in build-log.md: S3's Q1–Q4 and S4's Q1–Q9 are both logged there, and P3's RA6 (the frozen archive) differs from P4's RA6 (sequencing). H2, H3, M1 and N1b cite bare Q1, Q2, Q4 and RA6. 'closed record', used in nine rows, has no terms entry"
      fix: "qualify by seat (S3's Q1, P3's RA6) and add one terms line for closed record"
    - id: RA5
      severity: advisory
      row: how_to_read
      defect: "'the committed migration log 0001–0029': 0024–0029 are untracked on the branch. 'every other option was checked under a scratch log of its own' overstates: F1–F4, P1 and M1 have no verdict to probe, and some options were probed under the earlier logs"
      fix: "'the branch's migration log 0001–0029 (0024–0029 uncommitted)'; 'every option with a verdict to check'"
  readability: "the new top sections work: choices_at_a_glance gives all 27 choices with the pick, ruled_at_a_glance names the 11 ruled rows, and terms glosses nearly every code the rows use. Every choice row lists its options. Taken on its own, each choice row can be understood without the records, except where RB1, RA1 and RA4 note a gap"
  gate_denies: "none from the installed gate or on any probe; this block was dry-run before its one Edit"
---

## Failure narrative

FAIL on seven blocking findings, each a row the user would rule on while it is wrong or missing:

- B1: the lane home misses the drawing that the post-S4 rules put there.
- B2: every store leaves its preamble unbounded, and seam 3 does not say so.
- B3: F2 conflicts with the lead's RA8 spelling.
- B4: S5 sits below the 177 floor under a stale "ruled" label.
- B5: F2's spine clause decides against seams R1.
- B6: F1 and F2 give contracts/* no entry and api.yaml no form.
- B7: the archive shape re-admits the displacement D3 closed, unstated.

Every verdict R1 sampled matched the table; the gaps are in what the rows decide and say.

## Notes of note

- Every blocking fix is a row change or an added row. Only B4 and B7 need new probes: their
  options, each a Write at its destination, plus B7's re-probe of L1 and L2.
- B1 and B6 may need rule text beyond the census (impl.design-outputs-home, the contracts marker).
  That text is S4's re-cut or wave 3's; the lead routes it.
- `migrate validate` on log903b took 0.3 s here, not the facts' 160 s; the result matched
  (0 rejecting · 113 advisory).
