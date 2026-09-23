---
report: review
round: 1
session: delta-files-vs-direct-baseline-edits
reviewed: >-
  .mochiko/brainstorms/delta-files-vs-direct-baseline-edits/record.md (365 lines, frozen
  2026-09-23 21:06, D1–D6, F1–F12, OQ1–OQ2, Q1–Q6)
reviewer: cold-reviewer — mochiko:devils-advocate seat, mochiko:review-brainstorm, solo, cold pass
pairing: solo — both lenses carried (decision-quality and record-integrity); the pair protocol is skipped
blind_map: reports/angle-map.md (47 angles, 30 load-bearing, 8 classes; returned before record contact)
recommended_status: critical-gaps
status_basis: >-
  Two broken load-bearing claims, each of which would likely change its decision. C1: D1/D3c/D4
  move baseline and store writes ahead of the design checkpoint while claiming continuity with the
  store's sign-off carve, and a user-gate floor falls without a ruling. C2: D5 deletes FEAT-001's
  feature-dir copies on the premise that their headers record the fold. Run 4's fold never reached
  any product baseline, and D3a's next id collides with ids already cited.
tally: "25 raised, 17 survived — 2 Critical, 9 Important, 6 Minor; 6 coverage survivors among them (C1, I1, I5, I6, I9, M5)"
verdict_is_input: >-
  This status is a recommendation. The clearing verdict is the lead's, and every challenge to a
  user ruling goes back to the user. It is never folded by the lead's pen alone.
cross_exam: >-
  Solo, so the pair protocol is skipped. Eight questions were put to the record and answered from
  its text alone. Q-a: when does a design-phase seat write the baseline relative to the checkpoint?
  D3c answers "in the design phase", before the review pair and so before sign-off, and no clause
  moves the store's "no sign-off, no store write" floor (C1). Q-b: what base does "git diff" read?
  Not in the record (I1). Q-c: who is "the merge reviewer" in D3a? Not in the record; no rule
  seats one (I2). Q-d: are the run-4 owed folds disposed? F5 names them "owed by the principal's
  pen", and D5 is silent (C2). Q-e: does D2's marker reach a count, a summary table or a
  validation-rule list? Not in the record (I3). Q-f: what key does a product-lane run mark with?
  Not in the record (I5). Q-g: is design/ in D1's design-phase write set? Yes, by the statement,
  against the design baseline's as-built rule (I6). Q-h: is the field review still open? The record
  says open, but it was accepted and landed at 21:13, after this record froze (I8).
survivors:
  - id: C1
    severity: Critical
    decisions: [D1, D3, D4, F2, F11]
    kind: broken load-bearing claim · passive acceptance
    finding: >-
      D1 claims the architecture store "already works this way" and D4 says the carve
      "impl.baselines-never-in-place already grants" in-place in-flight writes, "now the rule for
      every baseline". The carve grants one write only: "a store write at the design checkpoint's
      user sign-off" (0001-genesis:1780). Its companion floor,
      authoring-architecture-store.sign-off-is-write-gate (0001-genesis:3285), reads "the sign-off
      IS the write gate. No sign-off, no store write." D3c has the feasibility and plan-artifact
      reviewers read "git diff -- .mochiko/product/" in the design phase, and
      impl.design-review-pair grades "before the design checkpoint" (0001-genesis:1401). The
      baseline and store writes therefore land before the user signs. F1c's recorded reason for the
      gate ("sign-off-as-write-gate keeps the store ruled-truth-only") is quoted in the facts and
      then dropped without a ruling. The floor is not in D6d's supersession list, and the Q1 put
      did not show the user that unsigned proposals would enter the baselines and the store.
    failure_scenario: >-
      FEAT-001 run 4 left six design-review reports dated 2026-09-22 before its checkpoint. Under
      D1 each round's drafts sit in the store and baselines marked in-flight (FEAT-001), the same
      mark a signed element carries. A second feature's sufficiency check then reads them. By
      review-sufficiency.clause-in-flight it must "cite the planned contract, no gap" against an
      unsigned proposal. The user reshapes or rejects at the checkpoint, and the second feature is
      graded sufficient against a design that died. As built, the rules contradict each other: the
      old floor forbids the write that D3c's review needs.
    resolution: >-
      Put the timing to the user as its own ruling. Recommended: writes before sign-off carry a
      distinct pre-sign status, such as proposed (FEAT-XXX), and flip to in-flight at the
      checkpoint. sign-off-is-write-gate is then superseded by a recorded ruling that says "sign-off
      flips proposed to in-flight". The other road is to write only at sign-off, with the review
      pair grading drafts held elsewhere, which brings back a delta-shaped draft. Either way, add the
      floor to D6d and restate F11/D4's "same model" claim with the timing difference.
  - id: C2
    severity: Critical
    decisions: [D5, D3, F4, F5]
    kind: broken load-bearing claim · missing dimension
    finding: >-
      D5 deletes the feature-dir baseline copies, "their own headers having already recorded the
      fold and the commit their text persists at". That holds for FEAT-001's run-3 content only.
      The current FEAT-001 data-model.md and constraints-and-decisions.md are the run-4 package
      (header: "Fresh package for run 4 (R1-9/R1-10/R1-11)"). F5 records run 4's fold as "owed by
      the principal's pen", and it never landed. Product constraints-and-decisions.md stops at
      D-049/C-008. Product data-model.md, contracts/ipc.md and constraints-and-decisions.md hold
      zero R1-9/R1-10/R1-11 lines, and no product commit after 5167f36 touches data-model.md or
      constraints-and-decisions.md. The graded run-4 fold text assigned product ids C-009…C-011
      and D-050…D-061. The feature-map entry landed at 5167f36 already says run 4's amendments are
      "DISCHARGED, built into data-model.md, contracts/ipc.md and contracts/plugin-bridge.md's own
      product text (`D-061` corrected …)". FEAT-006's copies do not record a fold in their headers
      either ("Folds into the baseline only at the feature's landing").
    failure_scenario: >-
      The kinako wave deletes the only non-report copy of run 4's unfolded design, and the product
      baselines permanently lack run 4's entities, decisions and V-48…V-51 while the code and the
      map cite them. The next run applies D3a, reads high-water D-049 and writes D-050. The map
      entry's D-061 and the fold text's D-050…D-061 then name different decisions from the
      baseline's new D-050 onward.
    resolution: >-
      Add a clause to D5. Before the copies go, apply run 4's graded fold text
      (reports/landing-fold-text-run4-2026-09-23.md §A, PASS at revision 1) as a direct in-place
      edit, with entries in the sections they extend. The product high-water becomes C-011/D-061,
      and the same non-author diff read that grades the fold-block re-homing grades this edit.
      Correct D5's premise to "copies whose content folded, or is folded in this pass". Cite the
      epic landings as the fold evidence for FEAT-006's copies (EPIC-001, ac21ce8) and FEAT-002's
      (EPIC-002, 77ea3fb).
  - id: I1
    severity: Important
    decisions: [D1, D3, Q1, F10]
    kind: unchallenged assumption · inconsistency · coverage (A6, C2, C7 partial)
    finding: >-
      "The change is what git shows — git diff while uncommitted, the commit once landed" names no
      base. The record's own evidence defeats a plain working-tree diff. The kinako memory F10
      cites says the principal commits mid-run from their own client ("688627f checkpoint swept
      an in-progress cycle's code in under a one-word message"), and "an empty git diff on a file
      a seat just edited is not a lost edit". Q1's answer ("the commit is the user's acceptance,
      so the review reads the working-tree diff before it") contradicts D2's rationale ("runs
      commit on main mid-run and the lead commits at checkpoints"). D3b puts the run-level reason
      in the commit message, but that message is user-written or depends on the per-run commit
      carve F10 records as unruled (audit gap 5). D6d's new fail rule, "an unmarked baseline
      write", can only be checked against a pinned base. The built-vs-signed diff and
      impl.deviation-gate need the signed state frozen, and D1 keeps no artifact that freezes it.
    failure_scenario: >-
      The user commits mid-cycle. At landing, "git diff -- .mochiko/product/" shows only the
      uncommitted tail. An unmarked edit swept into the checkpoint commit is never read, which is
      the silent baseline corruption F1b named as the reason the fold was graded.
    resolution: >-
      The run records its base commit in its run log at run-open, and again at the design-checkpoint
      sign-off. Every baseline review reads the committed and uncommitted change set against that
      base with "git diff <base> -- .mochiko/product/". The unmarked-write fail is "a hunk in that
      diff outside an entry marked for this feature". Drop the commit message as a required
      rationale carrier, or rule audit gap 5's per-run commit carve here.
  - id: I2
    severity: Important
    decisions: [D3]
    kind: unchallenged assumption · rejected-road steelman
    finding: >-
      D3a says two runs "may mint the same id; the second to land renumbers its own entries, and the
      merge reviewer catches a collision the way a code merge does". No rule seats a merge reviewer:
      merges are the user's git operations under impl.no-git-mutations. Git flags only overlapping
      hunks. D5 re-homes entries into "the sections they extend", so ids are placed by topic, not by
      order. Local check: two branches each add "### D-050" under different sections of one file;
      "git merge --no-edit featB" printed "merge featB exit=0", and the merged file held "8:### D-050
      — featA …" and "16:### D-050 — featB …". The claim carries no disclosure line
      (EXTERNAL-CLAIMS: an undisclosed claim is itself a finding). D3a's worktree premise also
      conflicts with D2's premise that runs commit on main. No alternative was dealt: block
      reservation per run, or a feature-qualified provisional id flipped at landing.
    failure_scenario: >-
      Two worktree runs each mint D-050 in different sections. The merge is clean, both land, and
      citations in code and reports resolve to whichever entry a reader finds first. This repeats
      the BD-240 collision F6 records, now in the product sequence itself.
    resolution: >-
      Replace "the merge reviewer catches" with a mechanical check that owns the catch: an
      id-uniqueness pass over each baseline in the landing diff review (the non-author seat of
      D3c). Or rule block reservation for concurrent runs. Name the seat.
  - id: I3
    severity: Important
    decisions: [D2]
    kind: missing intra-decision dimension
    finding: >-
      D2 marks "an entry". A feature's change also edits structures that are not entries, and those
      carry no marker. kinako data-model.md opens "Eighteen entities are delivered. Seventeen are
      modelled here." (:32). It has an Entity summary table (:30), a Relationships table (:729) and
      "## Validation rules, as delivered" (:759). contracts/ipc.md holds dated count-reconciliation
      sections ("the closed set is **46**", :427). The run-4 fold text touched "the Relationships table, two
      Entity-summary rows and V-48…V-51" (F-07). An abandoned run's sweep finds marked entries only.
    failure_scenario: >-
      A run adds an entity, updates the count and the summary row, then is abandoned. The sweep
      removes the entity, and the count and summary row stay. The living view lies, which is
      exactly the failure F1a says the delta model existed to prevent.
    resolution: >-
      The entry grammar (OQ2 or the census table) either forbids derived counts and summaries in
      baselines (derive them) or requires a row-level marker on every table row and list item a run
      touches. The abandonment sweep then reads the pinned-base diff (I1), not markers alone.
  - id: I4
    severity: Important
    decisions: [D2, D3, F11]
    kind: broken supporting claim · inconsistency
    finding: >-
      D2 claims "the same vocabulary the architecture store's elements use today". The store's fold
      duty (authoring-architecture-store.fold-duty) says "flip in-flight-class elements to built,
      clear their FEAT-XXX keys", and the spine template's check asks for "none on ruled/built"
      (0001:10008). D2's form is "built (FEAT-XXX, <date>)".
      authoring-architecture-store.lifecycle-statuses keeps "status is the build-lifecycle axis
      only — stance is a separate axis". kinako C&D's existing **Status:** field holds the stance
      axis ("recommended, effectively forced", "new · Source:", "delivered, as amended"), and D2
      writes lifecycle words into it. data-model.md carries no Status field (0 hits); contracts/
      carries one, a ruling stamp ("Status: RULED, 2026-09-03", plugin-bridge.md:53). Source ·
      Shaped by · Impact appear 16 times in C&D and 0 times in data-model.md or contracts/, so D3b's
      "fields the baselines already carry" is true of C&D only.
    failure_scenario: >-
      The landing flip leaves FEAT keys on built entries, which the store's own template check
      rejects if the flip is shared. A C&D reader cannot tell "recommended" (stance) from
      "in-flight" (lifecycle). Build-raised data-model changes have no field for their why.
    resolution: >-
      Rule one form for built, keyed or not, across the store and the baselines. Put the lifecycle
      marker in its own field, separate from C&D's stance Status. Declare which new fields
      data-model and contracts entries gain, in the census table.
  - id: I5
    severity: Important
    decisions: [D2, D4, D6]
    kind: coverage (A4, D4, F1)
    finding: >-
      Runs with no FEAT-XXX are unaddressed. The product-lane home declares baseline-delta.md
      (0005:325). impl.landing-lane says "a product lane owns no capability entry". D4/D6 withdraw
      the ledger from the feature and epic homes only. D2's marker keys FEAT-XXX, and the orphan
      rule flags any element whose key "names a closed, retired, or nonexistent feature". Outside
      implement, kinako fixes B126/B129 (341bbb8, 2004abe) edited product contracts in place with
      code, under no marker. The record does not say whether the new floor binds them.
    failure_scenario: >-
      A lane run edits a baseline. Either it cannot mark (no key), or its lane-keyed mark is an
      orphan by definition. Its home still requires a file D1 abolished.
    resolution: >-
      Key markers by run owner (FEAT-XXX or lane-<slug>) and extend the orphan rule to lane keys.
      Withdraw baseline-delta.md from the product-lane home. State the floor's scope: implement
      runs mark; out-of-run fixes that ship doc and code in one commit write built directly.
  - id: I6
    severity: Important
    decisions: [D1, D6, Prior-session relations]
    kind: coverage (F4) · missed prior ruling
    finding: >-
      D1 puts design/ in the design-phase write set. The design baseline (impeccable-design-integration
      D6/D7, 2026-09-19; 0013 and 0018) is "written only from shipped code at a landing fold, never
      from intent". It has "one writer, product-designer, on exactly three graded paths", one of
      them "a build-time baseline-delta.md entry". impl.design-first-write (floor) makes "the fold's
      first system-part write" the user's at final acceptance. The live gap-finding fence (0012
      reword) admits the design baseline as "as-built system". The prior-relations list omits this
      ruling. D6d names "the 0013 design-baseline fold lines" only, while 0018's replace-document
      re-imports the template text naming baseline-delta.md and the landing fold.
    failure_scenario: >-
      A design-phase seat writes intended tokens into design/ marked in-flight. The blind
      gap-finder reads them as promised as-built look, and the first-write floor has no fold left
      to trigger on.
    resolution: >-
      Carve design/'s system part out of D1's design-phase writes. It keeps its landing write from
      shipped code, which is already an in-place graded write and compatible with D1, and the
      first-write floor stands. The truth part follows D1. Add the ruling to prior relations, and
      add 0018 and impl.design-first-write to D6d.
  - id: I7
    severity: Important
    decisions: [D6]
    kind: inconsistency (build-surface completeness)
    finding: >-
      D6d's supersession list misses rules whose text assumes delta files or the fold (all ids
      resolve in the log): impl.design-outputs-home ("Design outputs land at FEAT-XXX/ as deltas
      beside their baselines"), impl.epic-shared-baseline-single-pen, impl.verification-design-time-grades,
      feat.product-surface and impl.reports-envelope ("delta files overwrite only via the graded
      fold"), feat.author-grader (names baseline-delta.md), impl.landing-lane ("the graded folds"),
      authoring-feature-map.map-side-altitude ("the appliable before/after form"),
      review-sufficiency.clause-in-flight ("reading that feature's deltas"),
      testing-gap-finding.blindness-fence-inclusion-list (floor; "the feature's design-phase
      deltas"), authoring-architecture-store.sign-off-is-write-gate (floor; C1) and the 0018
      design-baseline template (I6). F1d also cites impl.epic-shared-baseline-single-pen, which D6d
      then leaves out.
    failure_scenario: >-
      The migration wave rewords the listed rules and ships the rest unchanged. Implement then
      binds design outputs to feature-home deltas that D4 withdrew from the home, and the gate
      denies the write the rule demands.
    resolution: >-
      Extend D6d with the twelve sites above. Before the migration op, run a full-text sweep of the
      log for "delta", "fold", "appliable" and "in place" as a build step, and require the audit
      to diff the list against it.
  - id: I8
    severity: Important
    decisions: [D6, Prior-session relations, Constraints carried in]
    kind: inconsistency · stale relation
    finding: >-
      The record calls hook-enforcement-field-review "open … frozen for cold review". D6a says its
      amendments "land when that session's dispositions run". That record's status line reads
      "accepted 2026-09-23 … Landed: DECISIONS.md row … BACKLOG.md build item". Its file time is
      21:13:50, after this record's freeze at 21:06:43. Its dispositions have run, so D6a's route
      is gone. Its accepted OQ1 now also rules "the fold shape (dated fold block appended vs
      in-place [EXTEND])", which D1/D5 answer. Its D2 as amended takes contracts/* out of scope
      ("no gate bound applies"), so D6a's "now governs the in-place writes D1 makes" overstates
      for contracts.
    failure_scenario: >-
      The field-review wave-2 census rules the feature/epic home sets and the fold shape on the
      delta-model assumption. This record's amendments then have no landing carrier.
    resolution: >-
      Restate D6a as a supersession-in-part of an accepted record. Annotate its DECISIONS row and its
      BACKLOG build item at this record's landing, carrying D4's home sets and the fold-shape
      answer into the census. Correct the contracts scope.
  - id: I9
    severity: Important
    decisions: [D5]
    kind: coverage (E3) · missing dimension
    finding: >-
      D5 moves the three ledgers to .mochiko/archive/ and names no pointer treatment. kinako carries
      6 markdown links to ledger paths, 3 of them "../../features/FEAT-001/baseline-delta.md".
      61 .mochiko files name the file. BD ids are cited in 113 code files and on 76 lines across 8
      product-baseline files. D5's lazy path ("booked only when a later run trips on it") has only
      those citations as its tripwire. D5 also leaves the epic-dir baseline copies undisposed
      (EPIC-001/002 data-model.md, constraints-and-decisions.md, quickstart.md, contracts/), which
      D6b withdraws from the epic home.
    failure_scenario: >-
      The move creates dead pointers, a knowledge-management defect, and every BD citation in code
      loses its path. The lazy path's trip then has nowhere to land.
    resolution: >-
      Leave a one-line stub at each old path pointing to the archive copy, or re-point the links
      mechanically in the same kinako wave. State that the epic dirs stay as closed record
      (authoring-epic.close-semantics) and are exempt from the census.
  - id: M1
    severity: Minor
    decisions: [D2, OQ2, D1]
    kind: inconsistency · excess machinery
    finding: >-
      D2's statement rules that an amended entry "keeps the prior text readable beside the new".
      OQ2 lists the same choice as open. Keeping prior text beside new is the "two copies of every
      entry" cost D1 rejected road C for.
    resolution: >-
      Cheaper shape: marker only, with the pinned-base diff (I1) showing prior text. Strike the
      clause from D2 or close OQ2 with it.
  - id: M2
    severity: Minor
    decisions: [D1]
    kind: rejected-road steelman
    finding: >-
      D1's rationale says "the fold failed at each purpose it was ruled for". Each of its three legs
      is an execution defect curable under road C. No seat could apply the fold because of the
      budget deny (F9), which field-review D2 cures and D1 does not (OQ1 shows D1 hits the same
      wall). The accreted blocks are a fold-shape choice, which the field review's OQ1 already
      asked about. The never-folded ledger is a fold that was never dispatched. C's real residual
      cost is the transcription step, the two copies and renumbering. The ruling stands on those.
    resolution: >-
      Restate D1's rationale on the residual costs. Record that D1 does not cure the gate problem
      and depends on field-review D2 and OQ1.
  - id: M3
    severity: Minor
    decisions: [Prior-session relations, D6]
    kind: wrong fact
    finding: >-
      feature-sizing-and-entry-points was ruled 2026-08-10 (DECISIONS row 81; the record's own
      status line says "accepted (2026-08-10)"), not 2026-08-13. D6d's annotation list says
      "2026-08-13 feature-sizing", and the 2026-08-13 rows belong to other sessions.
    resolution: Correct the date in the prior-relations list and in D6d.
  - id: M4
    severity: Minor
    decisions: [D3, D6]
    kind: record fitness
    finding: >-
      D3 and D6 carry no rejected roads. Both rest on batch rulings (Q3 put three mechanics as one
      question, Q6 four items). None of the sub-items' alternatives were dealt: block reservation,
      feature-qualified ids, a ledger-lite rationale file, a separate build sequence. Both still
      read Confident.
    resolution: >-
      Record the alternatives each sub-item beat, or mark the un-dealt sub-items Assumed.
  - id: M5
    severity: Minor
    decisions: [D1, Constraints carried in]
    kind: coverage (F8)
    finding: >-
      BACKLOG audit gap 3 cites impl.baselines-never-in-place: the B116 spine groom (f4c3ae5,
      646 → 227) edited the store mid-run, and the gap reads "carve or forbid grooms". D1
      supersedes that floor, and the record does not say whether gap 3 dissolves or how a
      compress-and-move groom is marked.
    resolution: Dispose gap 3 in D6, saying whether a groom is exempt from markers.
  - id: M6
    severity: Minor
    decisions: [D2]
    kind: missing dimension
    finding: >-
      D2 extends the orphan rule to every baseline "unchanged". The orphan check's only reader is
      the architecture store's health view, and no reader is named for data-model, C&D or
      contracts orphans.
    resolution: Name the reader, for example the sufficiency check at run-open, or the landing verifier.
withdrawn_or_commentary:
  - "G5 (git as a hard dependency): withdrawn. Git is already in the loop (impl.cold-verification uses git ls-files; the graded head uses git rev-parse), and F10 records it."
  - "B6/B7 hybrids undealt: commentary. Q1's roads B/C span the space, and no ruling would change."
  - "E5 cross-epic collision rule: commentary. The rule lives in the archived ledger, and with BD ids retired no new collision arises."
  - "D6c no governance amend: checked true. Neither mochiko's CLAUDE.md nor kinako's names the fold. Withdrawn."
  - "F6 sound-loop review leg: D3c places it. Withdrawn."
  - "D6 crash/resume: markers give recovery. Withdrawn (I1's base covers the rest)."
  - "Run-4 fold review '14 findings' vs the kinako report's own split '4 blocking, 7 minor, 2 advisory' (13): a kinako report inconsistency, not the record's. Commentary."
  - "Phase 0's '237 BD entries': the reviewer's own miscount. The record's 238 is right."
verification:
  confirmed:
    - "F1a–d quotes, against the four prior records (architecture-design-primitive:113, feature-sizing:141, product-architecture-schema:213-214, multi-feature-plan-implement:142)"
    - "F2 floor and carve text, verbatim (0001-genesis:1780-1789); fail rules :1946, :1973; graded fold :1697; baseline-delta grammar :1647"
    - "F4 headers of FEAT-001 data-model.md, constraints-and-decisions.md and architecture.md (point at 741585d and the run-3 fold text)"
    - "F5: run-3 fold text 823 lines, run-4 619 lines; commit ec0b17f +364/−19; 5167f36 writes concerns.md +28 and spine.md +18 only in product; the run-4 memory says the folds are owed"
    - "F7: fold blocks at C&D :371, :547 and data-model :907, :1080, after the named sections; BoundSession [NEW] at :646"
    - "F8: 2314 + 4744 + 1010 = 8068 lines; 62 + 144 + 32 = 238 entries (FEAT-001's header declares 62)"
    - "F9/S6b quote in the field review's review.md:141-144; its OQ1 and D2 text"
    - "F10: impl.cold-verification snapshot path; user-commits-during-runs.md; run commits on main's first-parent line"
    - "F11: 5167f36 stat; C&D Status field present"
    - "D6c: no fold in either governance surface"
    - "Every rule id D6d names resolves in the log"
  found_wrong:
    - "D5's premise that the feature-dir copies' headers record the fold is false for FEAT-001's current run-4 content and for FEAT-006 (C2)"
    - "D3a's 'the merge reviewer catches a collision the way a code merge does' fails for non-overlapping hunks (I2, local git check)"
    - "D2's 'same vocabulary the architecture store uses today' fails on the built form and on the axis (I4)"
    - "D3b's 'fields the baselines already carry (Source · Shaped by · Impact)' is true of C&D only (I4)"
    - "Prior relations and D6d date feature-sizing 2026-08-13; the ruling is 2026-08-10 (M3)"
    - "The field review is described as open and frozen; it is accepted and landed (I8)"
  not_verified:
    - "F8's 'groom's line shift broke frozen citations (cycle-C1-6-verification.md:337)' and the BD-220 fold-check claim: not re-read"
    - "F5's 'PASSed at revision 1': the pass report was not opened"
    - "D5's 'runs have built against the baselines without them since 2026-09-03': not measurable from the tree"
external_claims: >-
  One. D3a's claim about git merge behaviour, undisclosed in the record. It was checked by a local
  reproduction rather than a web source: a scratch repo, two branches adding "### D-050" under
  different headings of one file, then "git merge --no-edit featB". The quoted output is "merge
  featB exit=0", with lines 8 and 16 both holding D-050. The solo re-read clause applies: the lead
  re-runs the four-step reproduction before I2 survives.
fitness:
  self_contained: "pass — the topic, facts, rulings, roads and question trail stand alone"
  decisions_attackable: "pass — every decision cites F-facts concretely"
  decision_trail: "pass — Q1–Q6 map to D1–D6; the Q1 follow-up on why a diff is still needed is recorded"
  confidence_honest: "partial — D3/D6 read Confident on batch rulings whose sub-items had no alternatives dealt (M4); D3a's collision catch rests on an unchecked claim (I2)"
  rejected_roads: "partial — D1, D2, D4 and D5 name theirs; D3 and D6 none (M4); road C's steelman is weaker than stated (M2)"
  honest_about_open: "partial — OQ1/OQ2 are listed, but D2's statement pre-empts OQ2 (M1) and the field review's state is stale (I8)"
  provenance: "pass — the status line names the lead, the reviewer seat, the blind dispatch and the transport"
hunt_grid:
  D1: "assumption: a git diff with no base (I1) · dimension: timing against sign-off (C1); runs without FEAT (I5); design/ (I6) · passive: 'store already works this way' (C1) · steelman: road C (M2) · inconsistency: 'commit = acceptance' vs mid-run commits (I1) · excess: none, it is the cheaper shape · scenario: an abandoned and a rejected design (C1, I3)"
  D2: "assumption: every change is an entry (I3) · dimension: lane key (I5); field axis (I4) · passive: the vocabulary claim (I4) · steelman: road C (manifest) lost fairly · inconsistency: against OQ2 (M1) · excess: prior text beside new (M1) · scenario: an abandoned run (I3)"
  D3: "assumption: the merge catches collisions (I2) · dimension: who reviews the merge (I2) · passive: the rationale fields (I4) · steelman: block reservation undealt (I2, M4) · inconsistency: worktree vs main premise (I2) · excess: none · scenario: two worktree runs (I2), the next run after run 4 (C2)"
  D4: "assumption: the carve already grants the write (C1) · dimension: the product-lane home (I5) · passive: none further · steelman: B/C lost fairly · inconsistency: none further · excess: none · scenario: the checkpoint table as the built-vs-signed input holds if its base is pinned (I1)"
  D5: "assumption: headers record the fold (C2) · dimension: the owed run-4 fold (C2); pointers and epic copies (I9) · passive: the lazy-path tripwire (I9) · steelman: road B lost fairly on cost · inconsistency: none · excess: none · scenario: the kinako wave executed as written (C2)"
  D6: "assumption: the field review is open (I8) · dimension: rule-list completeness (I7); BACKLOG gap 3 (M5) · passive: none · steelman: no roads recorded (M4) · inconsistency: dates (M3) · excess: none · scenario: migration wave executed from the list (I7)"
coverage_diff: >-
  Of 30 load-bearing map angles, the record meets 21 in full. It meets 6 in part: A3 (M2),
  A6/C2/C7 (I1), F1 (I5, product-lane home), F2 (I8). It misses 3: A4 (I5), E3 (I9), and C4's
  write timing (C1; the drawing is met, the timing is not). Non-load-bearing misses that survived:
  F4 (I6) and F8 (M5). B6, B7 and E5 are dismissed as commentary; G5 is withdrawn.
verify_round_1:
  date: 2026-09-24
  scope: >-
    The folds of C1–C2, I1–I9 and M1–M6 in the re-frozen record (584 lines, 07:03), each graded
    against the updated record. Also checked: internal consistency of D1–D7, F2/F5/F9/F10/F11/F13,
    the build surface and the relations; and the fitness of what the folds introduced. There was no
    fresh cold read and no blind-map hunt against D7 (reopen-born, verify grade only). New surface
    is reported only where a fold introduced a contradiction.
  result: not clean
  recommended_status: needs-revision
  status_basis: >-
    All 17 folds landed where the disposition table says, in the form ruled. Both Critical gaps are
    closed in substance: D7 gives the sign-off a pre-sign word, and D5(i) applies run 4's fold
    before the copies go. One blocking residual remains, V1, and it comes from the I1 fold, which
    carries the reviewer's own recommendation. D3c/D7 pin a sign-off base commit that D6d's audit
    gap 5 line and impl.no-git-mutations leave nobody to make. Thirteen nits are pen fixes.
  tally: "17 folds checked, 17 landed; 14 residuals, 1 blocking, 13 nits"
  folds_landed:
    - "C1: D7 455–480 · F2 70–82 · F11 164–166 · D4 325–328 · D1 216 · D6d 406–407 · relations 32–33. Landed as road A: proposed before the checkpoint, flipped to in-flight at sign-off, the store floor superseded."
    - "C2: D5(i) 347–351 · D5(iii) 353–356 · F4 95–96 · F5 108–113 · D5 rejected roads 378–379. Landed: the fold is applied first, the high-water is C-011/D-061, the premise is corrected and the epic commits are cited."
    - "I1: D3c 297–307 · D3b 295 · D1 216–217 · F10 159–160 · Q1 note 562–563. Landed. It introduces V1."
    - "I2: D3a 286–291 · F10 161–163. Landed: a duplicate-id check by the non-author landing seat; alternatives recorded."
    - "I3: D2 261–264 · F7 131–134. Landed: the sweep reads the pinned-base diff."
    - "I4: D2 252–259 · D3b 292–294 · F11 168–174. Landed: Lifecycle sits in its own field, stance Status is untouched, and the built form clears the key."
    - "I5: D1 226–227 · D2 256–257 · D4 333–335 · F3 88–90. Landed. F3 over-states the epic home (N6)."
    - "I6: D1 222–225 · F13 186–193 · D6d 420–422 · relations 30–31. Landed. F13's quotes are altered (N8)."
    - "I7: D6d 405–425. Landed: all twelve sites are present; the sweep is a build step; the list is Assumed. The sweep terms miss the key-set sites (N3)."
    - "I8: relations 33–39 · F9 145–152 · Constraints 202–204 · D6a 388–397 · Build 539–540. Landed. The KM index leg is missing (N12)."
    - "I9: D5(v) 360–365 · F6 121–122. Landed with the re-point road and no stubs; the epic dirs are exempt. F6's code-file count is not reproducible (N7)."
    - "M1: D2 259–261 · OQ2 551–552. Landed."
    - "M2: D1 229–240. Landed."
    - "M3: relations 29 · D6d 431. Landed."
    - "M4: D3 290–291, 296 · D6d 440–442, 453. Landed."
    - "M5: D6d 433–436. Landed. The own-commit option makes a claim git does not bear out (N2)."
    - "M6: D2 265–267. Landed."
  blocking:
    - id: V1
      touches: [D3c, D7, D6d]
      finding: >-
        D3c: "The run pins its base commit at run-open and again at the design checkpoint's
        sign-off"; D7: "the sign-off base is pinned there". impl.deviation-gate and the
        built-vs-signed diff "read against the sign-off base". At the checkpoint, the design-phase
        writes and the lead's flip are working-tree edits, and a commit exists only if someone makes
        one. D6d 436–437 leaves audit gap 5 (the per-run commit carve) unruled, saying "D3b no longer
        depends on it". But impl.no-git-mutations bars the lead from committing. Where no commit is
        made, the pinned sign-off commit is the HEAD from before the design writes. The landing diff
        then mixes signed and build-time changes, and the two floor-backed checks read against an
        unsigned state. The run-open pin is sound; only the sign-off pin needs a commit. This
        residual originates in the reviewer's own I1 resolution.
      repair: >-
        D3c/D7: the sign-off pin is the commit the checkpoint card asks the user to make (suggested,
        never run, per impl.no-git-mutations). If none is made, the card records the flipped entry
        set, and the built-vs-signed diff and deviation gate read the checkpoint table (D4). D6d's
        gap-5 line names this dependency.
  nits:
    - id: N1
      touches: [D3c 306–307]
      finding: >-
        "An unmarked write … is a hunk in that diff outside an entry marked for the run's key"
        fires falsely in two cases. One is uncommitted edits already under .mochiko/product/ at
        run-open, which are not the run's. The other is another open run's marked entries in the
        same tree: runs commit on main (F10, D2).
      repair: >-
        Run-open records any pre-existing hunks as excluded, and a hunk inside another open key's
        entry belongs to that run.
    - id: N2
      touches: [D6d 433–436]
      finding: >-
        "lands in its own commit so the pinned-base diff can exclude it": git diff <base> includes
        every commit after the base (F10), so one intermediate commit cannot be excluded.
      repair: >-
        The review reads around the groom (base..groom^ and groom..tree). Otherwise drop the
        own-commit option and keep "deferred past the landing".
    - id: N3
      touches: [D6d 419–425, Build surface 532–537]
      finding: >-
        I5's key set (EPIC-XXX, lane-<slug>) and D7's proposed reach rules the four sweep terms
        ("delta", "fold", "appliable", "in place") cannot find. They are lifecycle-statuses' "MUST
        name the FEAT-XXX that owns the change" (0001:3182), the orphan rule (0001:3229, "keys an
        open feature"), and the spine template's status vocabulary and key check (0001:9999,
        :10008).
      repair: Add the three sites to D6d, and add "in-flight" and "FEAT-XXX" to the sweep terms.
    - id: N4
      touches: [D7 460–461]
      finding: >-
        "the lead performs the flip" writes the store. The store's derived ARCHITECTURE.md is
        regenerated by the store skill on every store write and is never hand-edited
        (arch.single-writer-store, impl.store-landing), and D7 names neither.
      repair: >-
        The flip is a mechanical transcription made through the store skill, which regenerates the
        index. It rides the landing audit.
    - id: N5
      touches: [D2 252–254, D5(i) 347–351, D5(iv) 357–360]
      finding: >-
        D2 defines the marker for entries written during a run, but gives no reading for an entry
        with no Lifecycle field. D5(i) and (iv) write entries outside any run.
      repair: >-
        State that an entry with no Lifecycle field reads built, and that D5's writes add no
        marker.
    - id: N6
      touches: [F3 88–90]
      finding: >-
        "a declared deliverable of the feature, epic and product-lane homes (… :166, :325)". The
        epic home (0005:215–246) declares no baseline-delta.md; only :166 (feature) and :325
        (product-lane) do. D4 itself is correct.
      repair: Drop "epic" from F3.
    - id: N7
      touches: [F6 121–122, § Review 501]
      finding: >-
        "108 kinako code files" cite BD ids, a figure no scope reproduces: crates/src/tools gives
        113, crates+src 110, and tracked files 112. The "63 .mochiko files" figure checks out with
        archive/ counted.
      repair: State 113 (crates, src, tools), or name the scope that yields 108.
    - id: N8
      touches: [F13 187–192]
      finding: >-
        The quotes are altered inside the quote marks. F13 has "the setup leg, a build-time
        baseline-delta.md entry, and the landing fold"; the source (0013:305–307) reads "the setup
        leg (the truth part, the brownfield system-part seed, the scaffold), a build-time
        baseline-delta.md entry, and this fold". F13 has "before the baseline takes it"; the source
        reads "takes them".
      repair: Quote verbatim or mark each elision.
    - id: N9
      touches: [F1 50, relations 27]
      finding: F1 says "ruled four times" and the relations say "five sessions".
      repair: F1 should read "four times for the baselines; the design baseline's path in a fifth (F13)".
    - id: N10
      touches: [D7 479–480]
      finding: >-
        D7's confidence line says "a coverage survivor ruled inline". The table (509) and the trail
        (579–581) record a one-by-one three-option put.
      repair: Change it to "user-ruled A on a three-option put, one by one, 2026-09-24".
    - id: N11
      touches: [Q1 561–562]
      finding: >-
        The historical answer was edited. "reads the working-tree diff before it" became "reads the
        diff before it". The italic I1 note already records the sharpening.
      repair: Restore "the working-tree diff" and keep the note.
    - id: N12
      touches: [D6a 388–391, Build surface 539–540]
      finding: >-
        The supersession-in-part lands as annotations on the sibling's DECISIONS row and BACKLOG
        item only. KM's landing ritual requires, on supersession, updating "both indexes (brainstorms
        + decisions) so statuses agree" (knowledge-management.md, landing step 3).
      repair: Add the field review's brainstorms-index entry to D6a and to Build item 2.
    - id: N13
      touches: [§ Review 488–489, 497–499]
      finding: >-
        The summary says "all twelve rule ids resolve": eleven are rule ids, and the twelfth is the
        0018 template. Its "not in the record" list names Q-h, which the record did answer (it said
        "open"), and omits Q-d (D5 silent).
      repair: Write "eleven rule ids plus the 0018 template", and put Q-d in place of Q-h.
  verification:
    re_checked:
      - "record lines cited above, read in full (584 lines)"
      - "0005 epic home 215–246 and feature/product-lane :166/:325 (N6)"
      - "0013:305–307 and impl.design-first-write text (N8)"
      - "0001:3182, :3229 key clauses (N3)"
      - "kinako counts: BD-cite files 113 (crates/src/tools), 110 (crates+src), 112 (tracked); ledger-path files 61 (excl. archive), 63 (incl.); product BD lines 76 (N7)"
      - "knowledge-management.md landing ritual step 3 (N12)"
    not_re_checked:
      - "the lead's I2 reproduction (lines 7 and 17): accepted as re-run; the reviewer's own run gave lines 8 and 16 on a different fixture"
  routing: >-
    One Explore haiku seat was dispatched with an explicit hand-back instruction. It ended without
    a hand-back, the fourth such failure this session. Every count above was taken first-hand.
delta_check:
  date: 2026-09-24
  scope: >-
    V1's fold and the repairs of N1–N13 in the re-frozen record (626 lines, 07:28). Each was
    checked against the user's V1 ruling or the one-line repair given, and the sentences around each
    repair were checked for a new inconsistency. Nothing else was re-read or hunted.
  result: clean on blocking — 3 nits
  final_recommended_status: ready
  status_basis: >-
    V1 landed as the user ruled it: D3c 303–308, D7 483–484, D6d 455–457 and trail 624–626. All
    thirteen nits landed at the locations named. Two repairs introduced a nit each (d1, d2), and
    one repair left a restatement elsewhere behind (d3). All three are pen fixes that move no
    ruling. With them folded, nothing blocks ready.
  landed:
    - "V1: D3c 303–308 carries the user's commit at the checkpoint (suggested, never run), or the card's flipped-entry set with the checkpoint table as the signed state; D7 483–484 defines the sign-off base as either; D6d 455–457 names the soft dependency on gap 5; trail 624–626 records the three-option put."
    - "N1: D3c 316–319. Pre-existing hunks recorded at run-open are excluded, and another open key's entries belong to that run."
    - "N2: D6d 452–454. The review reads around the groom commit. See d2 for the range notation."
    - "N3: D6d 434–437 (the lifecycle-statuses key set, the orphan rule, the spine legend and check) · sweep terms 441–442 · build item 1 574–575."
    - "N4: D7 480–483. The flip runs through mochiko:authoring-architecture-store for the store, the index is regenerated, and the landing audit reads it."
    - "N5: D2 264 (no Lifecycle field reads built) · D5 379–380 (steps i–v add no marker)."
    - "N6: F3 92–93. The epic home declares the copies, not the ledger."
    - "N7: F6 124–125 · § Review 526. The 108 figure reproduces for crates/ + src/ with three-digit ids. See d1."
    - "N8: F13 191–197. Checked verbatim against 0013:300–307 and impl.design-first-write."
    - "N9: F1 52–53."
    - "N10: D7 502–504."
    - "N11: Q1 601. The original 'the working-tree diff' is restored and the I1 note kept."
    - "N12: D6a 402–404 · build item 2 578–579. See d3 for the restatement left behind."
    - "N13: § Review 512–514 (Q-d in; Q-h marked as answered wrongly) · 522–525 (eleven ids plus the 0018 template)."
  nits:
    - id: d1
      touches: [F6 124–125, § Review 526]
      finding: >-
        "113 with tools/" mixes criteria. Three-digit ids across crates/, src/ and tools/ give 111;
        113 is the any-length count. § Review 526 drops the "three-digit" qualifier, and
        crates/ + src/ at any length gives 110.
      repair: >-
        Write "111 with tools/" (or "113 with tools/, ids of any length"), and add "three-digit
        ids" at § Review 526.
    - id: d2
      touches: [D6d 453–454]
      finding: >-
        "`<groom>..` the tree": in git, "A.." means A..HEAD and leaves out uncommitted changes.
        That contradicts D3c's "committed and uncommitted change since the base". The reviewer's
        own "groom..tree" shorthand in N2 seeded it.
      repair: >-
        Write `git diff <groom>` (the groom commit against the working tree) alongside
        `<base>..<groom>^`.
    - id: d3
      touches: [Constraints carried in 207–209]
      finding: >-
        This restates D6a's annotation set as "its DECISIONS.md row and BACKLOG.md build item",
        without the brainstorms-index entry the N12 repair added to D6a and build item 2.
      repair: Add "and its .mochiko/brainstorms/index.md entry".
  verification:
    re_checked:
      - "record 1–626 at every V1/N site and the sentences around each"
      - "kinako BD-cite files: crates+src three-digit 108 · crates+src+tools three-digit 111 · crates+src any 110 · crates+src+tools any 113 (d1)"
      - "0013:300–307 design-landing and impl.design-first-write wording (N8)"
  routing: >-
    No Explore seat was dispatched this round. The one count was a single targeted re-run of
    commands already used, taken first-hand after four hand-back failures.
---

## Failure narrative

Recommended critical-gaps on two broken load-bearing claims. C1: D1, D3c and D4 put baseline and
store writes before the design checkpoint while claiming the store's sign-off carve already grants
them. The carve and its floor grant only a write at sign-off, so a user-gate floor falls with no
ruling and no D6d entry. C2: D5 deletes FEAT-001's feature-dir copies on a header premise that is
true for run 3 only. Run 4's graded fold never reached a product baseline. The map entry already
cites D-061, and D3a's next minted id restarts at D-050. Both fixes are small and the rest of the
direction holds. C1 needs a user ruling on write timing. C2 needs one clause in D5.

Verify round 1: not clean on one blocking residual, V1, which comes from the reviewer's own I1
resolution. D3c/D7 pin a sign-off base commit that nobody is authorized to make. The repair is to
ask for it at the checkpoint card, falling back to the checkpoint table.

## Notes of note

The field review was accepted and landed after this record froze, so D6a's landing route no longer
exists (I8). This affects sequencing: its wave-2 census is where D4's home sets and OQ1's fold
shape must land. The Explore haiku seats failed in Phase 0, so every read in the cold pass was
taken first-hand. The enumerations it needed were completeness-sensitive, which keeps them on the
seat tier in any case.

Delta-check: clean on blocking, with three nits (d1 to d3), each a pen fix. The final recommended
status is ready.
