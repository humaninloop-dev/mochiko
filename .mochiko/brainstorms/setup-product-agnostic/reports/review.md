---
report: review
pass: cold
pairing: solo
lenses: decision-quality + record-integrity
reviewed: .mochiko/brainstorms/setup-product-agnostic/record.md (frozen 2026-09-24)
blind_map: .mochiko/brainstorms/setup-product-agnostic/reports/angle-map.md (record_contact none at build)
recommended_status: critical-gaps
status_criterion: >-
  An unowned decision. OQ1's owner is "a later session or the build", yet the build surface
  ships the strip as one wave. The interim state breaks a specify floor and leaves the
  legal-mandate stratum held nowhere. See S1.
tally: 21 raised, 16 survived (1 Critical, 9 Important, 6 Minor)
fact_verification:
  F1: verified (INTERROGATION-AGENDA.md ten-dimension table)
  F2: verified (setup section counts 7/7/13/6/7/6 against mochiko-cli rules setup, plugin 0.114.0)
  F3: >-
    Verified. The rules files total 476 lines and engine-port.md has 154. The spine rows cite
    governance as follows: SPN-015 cites "GI-004 boundary 1", SPN-016 cites GI-008, SPN-017
    cites "GI-036 prose.md" and SPN-019 cites GI-032.
  F4: >-
    Quotes verified at kinako DECISIONS.md lines 77 and 80. Two defects: the amend history
    omits v1.1.0 (2026-08-16), and the spine.md:207 pointer is historical, since the text now
    sits at spine.md:173 and :233. See S16.
  F5: >-
    Partly verified. GI-001 rests on the vision record's D12, and the cold review's C1 found
    dimensions 1 and 4 skipped. The count of "37" was not reproduced: a phrase count gives 39.
    See S16.
  F6: verified (COMPLIANCE-MODULES.md)
  F7: verified (mochiko-cli template design-baseline)
  F8: verified (mochiko governance-intent GI-001 and GI-017)
  F9: verified (setup.md Adaptive Goal Protocol step 2)
  D5_counts: >-
    Broken. Twelve ids leave, not thirteen. Three floors leave, not two, because
    setup.fail.no-feature-map is class floor. Floors go from 20 to 17, not 18. See S6.
fitness:
  self_contained: pass (topic, F1-F9, D1-D5, Q1-Q6 trail)
  decisions_attackable: pass, except D2's "sorted cleanly" claim, which is a bare assertion (S3)
  decision_trail: pass (each D ties to a Q, and the Q1 leaves-list is carried Assumed until D5)
  confidence_marks: >-
    Pass with a caveat. All five decisions are Confident on explicit user rulings. But D5's
    "yes" confirmed a miscounted list (S6), and D2's confirmed sort covered only 7 of the
    region's rows (S3).
  rejected_roads: fail for D1, D5 and D4 (S2, S7, S15)
  honest_about_open: >-
    Fail. OQ1 has no owner (S1). OQ3 is listed as open, but D4's prose absorbs it as settled
    with "never leaves governance stale" (S5).
  provenance: pass (status line gives the lead, the review plan, the dates and the transport)
survivors:
  - id: S1
    severity: Critical
    decisions: [D5, D1, "Build surface", OQ1]
    kind: unowned decision and unsafe interim
    finding: >-
      OQ1's owner is "a later session or the build", but the build surface lands the strip as
      a single wave with no sequencing gate. The out-of-scope ruling on rehoming stands. What is
      unsafe is shipping the cut before any rehome exists.
    failure_scenario: >-
      First, a greenfield project runs /mochiko:specify after the cut. The rule
      spec.missing-map-surfaced routes it to "/mochiko:setup, whose brownfield analysis
      reconstructs it", which D5 removes. The floor spec.fail.map-unread fails the run, so the
      pipeline dead-ends. Second, the legal-mandate stratum, unwaivable under PO-D4.2, is held
      nowhere. patterns-vertical-tdd TEST-GRAMMAR's accessibility assert then silently falls
      back to the floor line for a product under ADA or EAA.
    resolution: >-
      Name OQ1's owner as a BACKLOG item with a trigger. Add a build-surface gate: the D5
      migration and the D1 module strip do not ship before the rehome lands, or the same release
      carries a named interim. The interim can be kept empty scaffolds (S7) plus rewritten
      routing in specify.
  - id: S2
    severity: Important
    decisions: [D1]
    kind: unchallenged assumption, rejected-road steelman, inconsistency
    finding: >-
      D1's rationale says the rules the modules sharpen "are already floor content at both
      depth levels and stay". COMPLIANCE-MODULES.md says a module "only ever ADDS obligations on
      top of the asserted floor". Its seed obligations are additive, not floor: audit logging of
      auth events, key rotation, coverage at 90/80, log retention, access-controlled logs,
      license compliance, and medium-plus vulnerability blocking. They also pass D2's own test,
      because each can be checked from the code without knowing what the product does. D1
      strikes content that D2 would admit. The middle road was never put to the user: modules
      as user-declared engineering modules that attach and detach only by declaration, which is
      D4 event 5's shape, with no fact re-elicitation.
    failure_scenario: >-
      A SaaS product storing card data runs setup after the cut. Nothing asserts PCI's additive
      controls, and the user ruled on a reassurance that nothing engineering-shaped was lost.
    resolution: >-
      Correct the rationale sentence. Put the declared-module road to the user with its cost,
      or record it as rejected with a reason.
  - id: S3
    severity: Important
    decisions: [D2]
    kind: inconsistency, unchallenged assumption
    finding: >-
      The confirmed sort keeps GI-008, "the Claude engine MUST be reached only through a port",
      as a hexagonal rule. F3 shows that store row SPN-016 is the same boundary and cites
      GI-008, which is the two-homes pattern D2 exists to end. It also fails D2's own axis,
      "whether the principle changes when the product changes": the engine choice was itself a
      product ruling (B20). The claim that the region "sorted cleanly" covers 7 of about 17
      rows. GI-030 (UI never touches corpus or engine), GI-031 (corpus backup), GI-005
      ("critical path") and GI-007 ("transcript content" in logs) were never put. "Mechanical
      enough for the validate seat" is untested.
    failure_scenario: >-
      The validate seat applies the new no-product-instance check. It either strikes GI-008,
      contradicting the user-confirmed sort, or keeps it and GI-030 alike, so product
      architecture stays in governance.
    resolution: >-
      Re-run the sort over every region row and put GI-008 and GI-030 to the user against the
      stated axis. Reword the test so the grader has worked cases, or mark D2's mechanical
      claim Assumed.
  - id: S4
    severity: Important
    decisions: [D3]
    kind: missing intra-decision dimension, unchallenged assumption
    finding: >-
      D3 names the depth recommendation as dimension 4's only lost consumer. The agenda also
      feeds dimension 4 into the floor's expression ("honest context") and into step 2's deck
      presets ("presets tuned by risk/values (dimensions 4, 9)"). Mochiko's own synthesis shows
      the risk surface is not always product-shaped. Its AM-1 to AM-3 risk entries, covering the
      shipped-executable vector and the hook vector, produced the four named supply-chain
      controls that gate the hook ship (GI-012). That is a distribution and trust fact that D3's
      rationale, "what its failure costs", does not cover.
    failure_scenario: >-
      A project that starts shipping a binary or hooks has no dimension that surfaces the new
      trust vector. The supply-chain controls that mochiko itself needed would never be elicited.
    resolution: >-
      Re-put dimension 4 with this evidence, or split it: strike the product-harm half and keep
      the engineering trust-vector half under dimension 8. Record the deck-preset input's fate.
  - id: S5
    severity: Important
    decisions: [D4, OQ3]
    kind: inconsistency
    finding: >-
      D4 says "a product change is never a governance event … none leaves governance stale".
      Its own event (3) fires on "a surface type is added". Adding a mobile or web surface to
      the product is a product change. OQ3 concedes that the GI-002 type line goes stale when
      Scope moves at the desk. The goal line's last limb holds only by definition.
    failure_scenario: >-
      A desk visit adds frontend-web to Scope. Nothing fires an amend and no frontend shelf is
      dealt. UI is built under backend-only standards until someone notices, which is the
      out-of-sync pain again.
    resolution: >-
      Reword D4 honestly: product changes open no amend except a surface-type change, which is
      event (3). Name the detector, for example an implement-entry comparison of Scope against
      GI-002, or rule that no detector is wanted.
  - id: S6
    severity: Important
    decisions: [D5, "Build surface step 1"]
    kind: inconsistency (broken count claims in a cold-build instruction)
    finding: >-
      The listed strikes are 11 rules and 1 fail, 12 ids in all. D5 says "twelve rules and one
      fail", and build step 1 says "thirteen ids". The fail rule setup.fail.no-feature-map is
      class floor, so three floors leave, not "two of them floors". Floors go from 20 to 17, not
      18, yet step 1 would re-key the pins to "floors 18". The phrase "the other three floors
      of the boundaries section" lists five. The fail rule's floor removal is also not named as
      a supersession-by-ruling.
    failure_scenario: >-
      A cold builder hunts for a thirteenth id and writes a floor pin of 18 against 17 rendered
      floors. The delivery then fails its pin check, or a floor leaves without a recorded
      supersession.
    resolution: >-
      Correct the counts to 12 ids, 3 floors and 17 floors after. Name the fail rule's floor
      removal as supersession-by-ruling.
  - id: S7
    severity: Important
    decisions: [D5]
    kind: rejected-road steelman, excess
    finding: >-
      D5 strikes the empty scaffolds on the ground that each one "writes a product artifact":
      the greenfield FEATURES.md index, the spine.md stub and the design home's heading
      scaffold. An empty scaffold holds no product content, so it cannot go stale, and the
      out-of-sync rationale does not reach it. The keep-the-scaffolds road was not dealt. Only
      "keep the two floors" was considered.
    failure_scenario: >-
      Striking the scaffolds causes the S1 dead-end in specify and leaves the architecture
      desk's arch.shelf-scope-source reading a Scope line "declared there by /mochiko:setup"
      that no one declares. Both are avoidable at zero drift cost.
    resolution: >-
      Put "scaffolds stay, elicited product content goes" to the user as the steelman, or record
      it as rejected with a reason.
  - id: S8
    severity: Important
    decisions: ["Build surface step 5"]
    kind: record-integrity (the supersession chain is incomplete)
    finding: >-
      Step 5 misses rulings the cut supersedes. PO-D3 ("setup elicits facts; safety floor +
      modules asserted") is listed under the prior-session relations but not superseded. PO-D4's
      D4.2 legal-mandate unwaivable stratum is ruled Contested and made moot by D1. The PO
      record's S4 fact-validation fail-safe fold is struck by D1. impeccable-design-integration
      D11 routes the accessibility standard of record to the a11y module. Protected content
      leaves only by recorded ruling.
    failure_scenario: >-
      The landing strips content traceable to these rows with no supersession row. The
      primitive-edit audit's preserved-responsibilities check reads that as silent deletion.
    resolution: >-
      Add supersession-in-part rows for PO-D3, PO-D4 (D4.2), the PO record's S4 fold and
      impeccable D11.
  - id: S9
    severity: Important
    decisions: ["Build surface steps 2 and 4", OQ1]
    kind: coverage (blind-map angle G1)
    finding: >-
      The build surface's edit list misses consumers the cut breaks or leaves pointing at
      nothing. specify's spec.missing-map-surfaced routes to setup. architecture's
      arch.shelf-scope-source says setup declares Scope. patterns-vertical-tdd TEST-GRAMMAR's
      a11y assert names the "attached a11y compliance module". patterns-design-direction and
      migration 0011's no-accessibility-content rule route to the a11y module. The
      release-gates.md validator fragment has "Gates consistent with the attached compliance
      modules". evals/plan/setup/observable.yaml lists 9 of the struck ids, and
      evals/contract/run.py is named only generically. Dead pointers are defects under GI-005.
    failure_scenario: >-
      The build lands, and the contract suite, the plan-eval rubric and live runs hit rules
      that cite removed setup behavior.
    resolution: >-
      Add each named consumer to the build surface, or bind them into S1's sequencing gate.
      Materiality: plausibly changes the build surface and the single-wave shape.
  - id: S10
    severity: Important
    decisions: [D3, D4]
    kind: coverage (blind-map angle A5)
    finding: >-
      The record never reconciles with the ruled but queued ops-observability-hardening build.
      OO-D3 elicits app-level SLO numbers at dimension 8, which are user-owned and product-shaped
      values. OO-D4 makes RUNBOOK asserted with a setup writer moment. OO-D5 adds a fifth floor
      category, Operations. D3 keeps dimension 8 unexamined, and D4's closed six has no event
      for an SLO change. Kinako's release gate already names distribution detail: the tap
      formula and the plugin tag channel.
    failure_scenario: >-
      The OO build lands after this cut. SLO numbers and runbook content enter governance
      through dimension 8, and each product-driven SLO change is either an unlisted event or
      silent staleness. The drift returns through the kept dimension.
    resolution: >-
      Add ops-observability-hardening to the prior-session relations. Rule whether SLO numbers
      and runbook content pass D2's test, and fold the answer into D3 and D4. Materiality:
      plausibly changes D4's closed set.
  - id: S11
    severity: Minor
    decisions: [D3, D5]
    kind: inconsistency
    finding: >-
      The floor setup.blind-map-dispatch sends the stress-test seat "the setup topic / project
      identity and goal". D3 reduces identity to a project name, and D5 does not touch the
      floor.
    resolution: Reword the floor's text in the D5 migration, or say identity means the name.
  - id: S12
    severity: Minor
    decisions: [D5]
    kind: inconsistency
    finding: >-
      setup.km-module-scaffold stays, and the knowledge-management core scaffolds two
      product-bearing docs: ARCHITECTURE.md, the derived index over a store setup no longer
      scaffolds, and GLOSSARY.md, the domain language. D5's criterion, "every struck rule writes
      a product artifact", is not applied to them.
    resolution: Rule their place in the knowledge-management core, or state why they are exempt.
  - id: S13
    severity: Minor
    decisions: [D2, D4]
    kind: coverage (blind-map angle C4)
    finding: >-
      Path-scoped rules files carry repo-layout globs, and kinako's region line enumerates
      src/, src-tauri/, crates/ and components/. These go stale when the tree changes, even when
      the rule content is product-agnostic. No D4 event covers them.
    resolution: >-
      Say which D4 event covers a layout change, or that glob drift is tolerated. Materiality:
      would not change a ruling.
  - id: S14
    severity: Minor
    decisions: [D2]
    kind: missing intra-decision dimension
    finding: >-
      D2 says "setup records the hand-off, not the content", but names no artifact, section or
      template slot for that record.
    resolution: Name the slot, for example in the governance-intent template's Deliberate exclusions.
  - id: S15
    severity: Minor
    decisions: [D4]
    kind: rejected-road steelman
    finding: >-
      F4 shows the owed amends were booked but not taken. The record diagnoses the cause as
      instance restatement only. The ceremony cost of an amend, which is a full setup run, was
      not weighed as a cause, and a cheaper amend path for surviving events was never dealt.
      Events 2 to 4 still book full amends.
    resolution: >-
      Record the cheaper-amend road as considered and rejected with a reason. Materiality:
      would not likely change a ruling.
  - id: S16
    severity: Minor
    decisions: [F4, F5]
    kind: record-integrity (unverifiable and stale fact pointers)
    finding: >-
      F4's amend history jumps from v1.0.0 to v1.2.0 and omits v1.1.0 (2026-08-16, GI-035).
      F4's spine.md:207 is the historical line quoted from DECISIONS.md line 80, and the text
      now sits at spine.md:173 and :233. F5's "37 times" has no stated method: a phrase count
      gives 23 "vision record" plus 16 "integration record", which is 39.
    resolution: Add v1.1.0, cite the current lines, and state F5's counting method or drop the number.
dropped:
  - D2 enforceability (a pointer to the store keeps the input-validation rule testable; the record covers it via "points at its home")
  - re-entry through arbitrated cards and the domain registry (the registry names stack crates, which pass D2)
  - product-shaped waivers such as kinako GI-031 (they leave with their principles under the D4 legacy clause)
  - mochiko's own GI-019 and GI-020 against D2 (build step 6 defers them to the next amend, which is a ruling)
  - all-Confident marks (every decision rests on an explicit user ruling, with its caveats folded into S3 and S6)
verify_round_1:
  pass: verify (solo, over the record re-frozen 2026-09-24 with 16 of 16 survivors dispositioned)
  recommended_status: needs-revision
  status_criterion: >-
    Three Important findings introduced by the folds remain open (V1 to V3). None is Critical,
    and OQ1 now has an owner, so the unowned-decision criterion no longer holds.
  tally: 12 of 16 CLEAN · 4 NOT CLEAN (S1, S5, S7, S10) · 9 fold-introduced findings (3 Important, 6 Minor)
  d5_counts_checked: >-
    Five rules are struck: 2 from roles, 1 from reserved and 2 from tools. None is a floor. The
    section counts become 5/6/11/6/7/6, which is 41. Floors stay at 20 and fails stay at 6. The
    record's figures match, checked against mochiko-cli rules setup at plugin 0.114.0.
  folds:
    S1: >-
      NOT CLEAN. D6 gives OQ1 an owner and states the exposure window. But its statement that
      "every consumer that cites a compliance module is reworded" is not delivered by build
      step 4 (V2), and its rationale that "the window touches no live consumer" contradicts D4's
      legacy clause (V3).
    S2: CLEAN. The false sentence is replaced (record lines 127-133), and the declared-module road is recorded as rejected with a reason (141-145).
    S3: >-
      CLEAN. GI-008 and GI-030 are re-sorted (173-178), the sort is declared illustrative, the
      mechanical claim is marked Assumed (168), worked cases are owed in build step 4, and the
      rejected road is added (186-188). One minor side-effect is recorded as V9.
    S4: >-
      CLEAN. Dimension 8 gains the trust vectors (201-204), the deck presets move to dimensions
      8 and 9, and dimension 4 is split (209-211, 224-226). A minor gap in the template slot is
      recorded as V8.
    S5: >-
      NOT CLEAN. The body is reworded as "opens no amend except through event (3)" (246), the
      detector is named (251-253) and OQ3 is closed into it. The D4 heading (238) still reads "a
      product change is never one" (V6).
    S6: CLEAN. The counts are recomputed and correct, and build step 1's pins (20 floors, 6 fails) match.
    S7: >-
      NOT CLEAN. The scaffolds stay (D5, lines 280-303), but the fold left OQ1's consequence
      list stale (V1). The lead now writes the design headings, which contradicts the design
      baseline template's "one writer, product-designer", and the template is not in the build
      surface (V5).
    S8: CLEAN. Build step 5 gains rows for PO-D3, PO-D4's D4.2, the S4 fail-safe and impeccable D11, and product-architecture-schema is marked as standing.
    S9: CLEAN. Every consumer the cold review named is in build step 4, and observable.yaml has its own step 6.
    S10: >-
      NOT CLEAN. The ops-observability relations are added and dimension 8 gets its D2 clause.
      But D3 places the SLO number set in the store as a concern row (216-218), while D6 rejects
      "module content dealt as concern rows" as a rehoming out of scope by Q1 (357-359) (V7).
    S11: CLEAN. setup.blind-map-dispatch is amended to "project name and goal" (299).
    S12: CLEAN. No change, with the reason recorded; a no-change disposition is a ruling.
    S13: CLEAN. Event (3) is widened to layout (242-243), and build step 3 carries it.
    S14: CLEAN. A Handed off list is added under Deliberate exclusions (161-163), and build step 3 carries it.
    S15: CLEAN. The cheaper-amend road is recorded as rejected with a reason (265-268).
    S16: CLEAN. F4 gains v1.1.0 and the historical pointer note (74-78), and F5 states its count method (81-82).
  new_findings:
    - id: V1
      severity: Important
      touches: [OQ1, D6, D5]
      finding: >-
        After S7, OQ1 still lists "the store scaffold" and "the feature map at close" as things to
        rehome. Its named consequences still say that the store and design home "are no longer
        scaffolded by anyone" and that "the feature map's first write moves to whichever command
        first touches it". All three are false under D5 as amended, and OQ1 is now the brief of
        D6's queued session.
      scenario: The rehoming session scopes itself to rehoming scaffolds that setup still writes, and may re-strike them.
      resolution: Rewrite OQ1's list and consequences against D5 as amended.
    - id: V2
      severity: Important
      touches: [D6, "Build surface step 4", OQ1]
      finding: >-
        D6 says every module-citing consumer is reworded in the same release, but build step 4
        misses several:
        - the design-baseline template, whose Accessibility section, check text and skeleton
          line "Standard of record: [the a11y compliance module, or the Essential Floor line]"
          come from migration 0018, and the S7 scaffold now writes that heading;
        - validation-constitution QUALITY-CHECKLIST lines 26, 35, 39, 40 and 47, the module
          checks, where step 4 says only "floor-category check unchanged";
        - catalog/universal-floor.md line 16.
        OQ1 itself keeps "the design baseline's Accessibility heading points at a module" as a
        rehoming item, which contradicts D6 directly.
      scenario: >-
        After the cut, the validator checks for "attached compliance modules match the fact
        profile one-for-one" against a synthesis with no fact profile. The scaffolded design.md
        points at a module that nothing attaches.
      resolution: >-
        Add the design-baseline template, the module lines of QUALITY-CHECKLIST and
        universal-floor.md to build step 4. Then either drop the Accessibility item from OQ1 or
        narrow D6's "every consumer" claim.
    - id: V3
      severity: Important
      touches: [D6, D4, OQ2, "Build surface step 7"]
      finding: >-
        D6's rationale says "the window touches no live consumer" because existing consumers
        keep their modules "until their next amend under D4's legacy clause". That clause strips
        modules and product-instance principles at that amend. Kinako owes an amend now (F4), so
        its gdpr module and GI-011, GI-012 and GI-013 would leave governance before any rehome
        exists.
      scenario: >-
        Kinako runs its owed boundary amend next week. The legacy clause strips erasure, export
        and transparency obligations from a live product, and they are held nowhere.
      resolution: >-
        Defer the legacy clause's module and product-principle limb until the rehome lands.
        Otherwise, state kinako's exposure in D6 and have the user rule on it.
    - id: V4
      severity: Minor
      touches: [D6]
      finding: >-
        The BACKLOG trigger "before the next setup run on a project with legal-mandate exposure"
        cannot fire, because after D1 setup never elicits that fact. The trigger reduces to "at
        the queued session", which has no date, and that is the very objection D6 raised against
        the gate road.
      resolution: Give the trigger an observable signal, or state that it is date-less and accepted as such.
    - id: V5
      severity: Minor
      touches: [D5, S7, "Build surface step 5"]
      finding: >-
        The lead writes the empty design headings with no product-designer seat. The
        design-baseline template, from migration 0018, says "one writer, product-designer, on
        exactly three graded paths: the setup leg". The supersession row "its setup leg in part"
        does not name the template edit, and no build step carries it.
      resolution: Add the design-baseline template's writer sentence to the build surface under the impeccable supersession.
    - id: V6
      severity: Minor
      touches: [D4]
      finding: The D4 heading still reads "a product change is never one", which contradicts the S5-reworded body.
      resolution: Reword the heading.
    - id: V7
      severity: Minor
      touches: [D3, D6]
      finding: >-
        The S10 fold places SLO numbers in the store as concern rows. D6 rejects the same move
        for module content as out of scope by the Q1 ruling.
      resolution: >-
        Mark the SLO homing as a user-ruled exception to the Q1 fence, or move it into OQ1 and
        keep only "the numbers leave governance".
    - id: V8
      severity: Minor
      touches: [D3, "Build surface step 3"]
      finding: >-
        The governance-intent template's Risk surface line is struck. The trust-vector answers
        that dimension 8 now elicits then have no synthesis slot. Mochiko's own AM-1 to AM-3
        entries live on that line today.
      resolution: Name the slot for dimension 8's trust-vector answers in build step 3.
    - id: V9
      severity: Minor
      touches: [D2, "Build surface step 4"]
      finding: >-
        The GI-008 re-sort keeps "every external system through a port, BE-HEX's port
        requirement". Kinako narrowed that scheme out during arbitration: engine-port.md is
        "scoped to the one obligation this project ruled — the engine seam". The sort seeds the
        validator's worked cases, so this example widens a ratified scope.
      resolution: Word the kept rule as the ruled one, the external engine behind a port, or pick a different worked case.
  process_note: >-
    A probe that copied this file into the scratchpad was denied by the redirect hook as a
    write. That is the first deny on this path. This section was added with one Edit.
---

## Failure narrative

The record fails on one blocking finding, S1. Rehoming is out of scope by the user's ruling,
and that ruling holds. But the build surface ships the strip in a single wave, and OQ1 names no
owner. Between the cut and a rehome nobody owns, two things break. Specify's floor dead-ends
every greenfield project on a missing feature map that setup no longer writes. The legal-mandate
stratum is held nowhere. Nine Important findings sit behind it. D1's reassurance that module
rules are "already floor content" is false (S2). D2's confirmed sort keeps a product boundary
it should strike (S3). D3 strikes a dimension that produced mochiko's own supply-chain controls
(S4). D4's "never" contradicts its own event (3) (S5). D5's counts are wrong in a cold-build
instruction (S6). D5 strikes scaffolds that cannot drift (S7). The rest are an incomplete
supersession chain (S8), unlisted consumers (S9) and an unreconciled queued ops build (S10).

## Notes of note

- A PreToolUse hook denied one read-only Bash grep because it parsed kinako's spine.md as a
  write target. I did not retry through Bash on that path. I read the file with the Read tool.
- The blind-map diff over 23 angles found 9 covered clean: A1, A2, A3, C3, D2, D3, D4, E1 and
  E2. Eleven are covered but produced findings: A4 became S4, B1 and B2 became S6 and S12, B3
  became S7, C1 became S5, C2 and F3 became S2, D1 became S8, F1 and F2 became S15, and G2
  became S1. Three are uncovered gaps: A5 became S10, G1 became S9 and C4 became S13.
- Kinako is a local tree and was verified from its files, so no claim needed an outside-repo check.

## Verify round 1

Twelve of the 16 folds are clean, and four are not: S1, S5, S7 and S10. The per-fold evidence
and nine new findings are in the frontmatter field `verify_round_1`. The recommended status is
needs-revision, because three Important findings introduced by the folds are open. First, V1:
OQ1's list goes stale after S7. Second, V2: D6's claim that "every module-citing consumer is
reworded" is not met, because the design-baseline template, the module lines of
QUALITY-CHECKLIST and universal-floor.md are missing from build step 4. Third, V3: D4's legacy
clause strips kinako's gdpr obligations at its owed amend, before any rehome exists, which
contradicts D6's "no live consumer". D5's recomputed counts are verified at 41 rules, 20 floors
and 6 fails.

## Delta-check

This was a single bounded round over the fixes for V1 to V9 in the re-frozen record. Seven of
the nine are clean.

**The V3 regrade supersedes the V3 row, X1 and X2 below.** The user ruled at Q8 that
the clause applies forward only. X1 and X2 are void, because nothing is carried forward and D1
and OQ2 now agree with the clause. The forward-only clause sits consistently with D6's statement
("Existing consumers are not a design input (Q8) … nothing in this cut carries it for them"),
with OQ1 (where that content goes is OQ1's session), with OQ2 ("superseded at its next amend") and
with build step 7 ("superseded on sight under D4 (Q8)"). V3 is still NOT CLEAN on one Minor
contradiction that the reversal introduced:

- **Y1 (Minor, touches D6's Rationale).** The rationale still reads "the window touches no live
  consumer" and "a case no live project is in". Under the Q8 ruling, kinako's owed amend
  supersedes its gdpr module and product-instance principles with nothing carrying them. The
  window does touch a live consumer, and the ruling makes that acceptable, not absent. The fix is
  to replace the two phrases with "existing consumers are not a design input (Q8)". The ruling
  itself is not a finding.

The X3 repair is verified against the record as it now reads. D6's Touches line (377-379) reads
"no fact-based trigger, V4: it rides the ROADMAP.md Next queue", so V4 is now CLEAN. The final
tally is 8 of 9 CLEAN. The only open finding is Y1, which is Minor: D6's Rationale at lines
366-367 still says "touches no live consumer" and "no live project is in". **The final
recommended status is ready.** Y1 is a one-sentence repair through the lead's pen and needs no
further review pass. The record's decisions stand.

*Superseded in part by the regrade above, kept as the reviewer's record:*

| Fold | Grade | Evidence |
|---|---|---|
| V1 | CLEAN | OQ1 now lists five content items and says the scaffolds are not rehoming items. D6's list of five matches it. |
| V2 | CLEAN | Build step 3 carries the design-baseline Accessibility pointer. Build step 4 adds the checklist module checks and the universal-floor sentence, and binds D6's "every" to that closed list. See the note on backend-service.md. |
| V3 | NOT CLEAN | D4's legacy clause is split correctly, and D6 and build step 7 match it. Two problems remain. First, D2's new validator check for "no product instance in the set" has no exemption for rows marked `carried pending rehome` (X1). Second, OQ2 and D1's statement were not realigned (X2). |
| V4 | NOT CLEAN | D6's statement replaces the trigger, but D6's Touches line still reads "trigger: before the next setup run on a project with legal-mandate exposure" (X3). |
| V5 | CLEAN | Build step 3 amends the design-baseline writer sentence, which matches D5's lead-written empty headings. |
| V6 | CLEAN | D4 is retitled "opens no amend except through event (3)". |
| V7 | CLEAN | D6's alternatives record the distinction: the SLO set supersedes an existing governance home, while module content has none. |
| V8 | CLEAN | Build step 3 gives the template a Trust vectors line in dimension 8's slot. |
| V9 | CLEAN | D2's GI-008 row now reads "at whatever scope the project ratified — kinako ratified the engine seam only". |

Findings introduced by the folds:

- **X1 (Important, touches V3, D2 and build step 4).** Build step 4 adds a validation-constitution
  check for "no product instance in the set", with no exemption for rows the split legacy clause
  keeps. Kinako's owed amend carries GI-036, GI-037 and the corpus rows as `carried pending
  rehome`, so it fails validation. The run then halts to the user under the gate-loop bound, or
  the rows get stripped to pass, which is the void V3 was meant to prevent. The fix is to exempt
  rows marked `carried pending rehome` in the new check, and say so in build step 4.
- **X2 (Minor, touches V3, OQ2 and D1).** OQ2 still says the sorted rows and the gdpr module
  "leave at its next amend". D1 still says the governance surface set "carries no module". Both
  contradict the carried-forward limb. The fix is to reword both against the split.
- **X3 (Minor, touches V4 and D6).** The fix is to replace the stale trigger in D6's Touches line
  with the re-put in the Next queue.
- **Note, not a finding.** The `catalog/backend-service.md` sentence at line 110 cites
  COMPLIANCE-MODULES.md as "attached via the fact profile", and it is not on the closed list. The
  record's own clause makes it a build defect, not a new ruling. It is flagged here so the
  builder has it.
