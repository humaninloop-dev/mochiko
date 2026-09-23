---
report: review
round: 1
session: hook-enforcement-field-review
reviewed: .mochiko/brainstorms/hook-enforcement-field-review/record.md (405 lines, frozen 2026-09-23, D1–D9, F1–F16, OQ1–OQ4)
reviewer: cold-reviewer — mochiko:devils-advocate seat (opus), mochiko:review-brainstorm, solo, cold pass
blind_map: reports/angle-map.md (53 angles, 8 classes; written before record contact)
recommended_status: critical-gaps
status_basis: three broken load-bearing claims (S1 D6 "nothing reads the log after acceptance"; S2 D4's run-folder path is a protected Claude Code path; S3 D7's rejection of git-root resolution as "the same answer") — each would likely change its decision
tally: "24 raised, 16 survived — 3 Critical, 8 Important, 5 Minor; 4 coverage survivors among them"
cross_exam: >-
  Solo; the pair protocol is skipped. 10 questions went to the lead, answered from the record: 7
  "not in the record" (Q1 `.claude/` treatment and the ignore line, conceded as a lead error; Q2
  log citations; Q4 nested trees, the cwd rule, and root docs left unresolved; Q5 primitives'
  paths; Q6 in-place amendment and split stores; Q7 the ledger text; Q8 the publish gate), 1 "not
  measured" (Q3 honest headers), 1 "not tested" (Q9 directory render targets), 1 "partly" (Q10 full
  commands real, minimal forms not re-run). No finding was withdrawn; Q4 extends S3.
survivors:
  - id: S1
    severity: Critical
    decisions: [D6, F16, D9d]
    kind: broken load-bearing claim · unchallenged assumption
    finding: >-
      D6 makes the run log ephemeral and deletes it at final acceptance on the premise that "no
      rule requires the log and nothing reads it after acceptance" (rejected B: "no later reader
      opens"). In kinako the log is the durable rationale home for in-run user rulings: 27
      DECISIONS.md rows cite a run log or an epic implement-log.md as their provenance pointer
      (11 to epic implement-log.md; run 4's rows cite run-log-run4-2026-09-22.md anchors such as
      `card_confirm`, `design_checkpoint`, `gap_rework_1`, `final_validation`), and ROADMAP.md and
      BACKLOG.md cite implement-log.md too. The implement rules name no other durable home for
      gate rulings (F16 is right that no rule requires the log; that is a gap, not proof of no
      need).
    failure_scenario: >-
      The next run's card-confirm and design-checkpoint rulings live only in a folder deleted at
      acceptance; the landing ritual's DECISIONS rows then point at nothing, and kinako's KM
      dead-pointer scan ("every ROADMAP.md / DECISIONS.md / BACKLOG.md pointer resolves") fails on
      every run. Withdrawing the epic log deliverable strands the 11 epic-log citations.
    resolution: >-
      Split the log's two jobs: the durable ruling record (gate rulings, checkpoint verdicts) keeps
      a declared home (the epic's log-form implement-log.md, and the same for the feature and lane
      homes, per-entry bounded — D6's rejected B), or the implement rules route each gate ruling
      to a `.mochiko/decisions/` record per the KM record-less path; only the working memory and
      resume list go to the run folder. Keep the epic deliverable until its citations are resolved.
  - id: S2
    severity: Critical
    decisions: [D4, D3, D9d]
    kind: broken load-bearing claim · unchallenged assumption
    finding: >-
      `.claude/` is a Claude Code protected directory; only `.claude/worktrees` is exempt. Writes to
      `.claude/mochiko-runs/<run>/` are "never auto-approved": prompted in default/acceptEdits,
      "Routed to the classifier" in auto (the mode every run-3 and run-4 transcript records),
      "Denied" in dontAsk, allowed only in bypassPermissions; "permissions.allow rules in settings
      files do not pre-approve protected-path writes", and Bash `>`/`>>`/`2>` and `tee` targets
      take the same protected-path check. verified: https://code.claude.com/docs/en/permission-modes
      (§ Protected paths: "`.claude`, except for `.claude/worktrees` where Claude stores its own
      git worktrees"). D4's rationale ("beside `.claude/worktrees/`") assumes the sibling inherits
      the exemption; it does not.
    failure_scenario: >-
      Run 4 wrote 166 raw evidence files. Under D4 each capture, by Write or by shell redirect,
      becomes a prompt a background seat cannot answer, a classifier call per file in auto mode, or
      a hard deny in dontAsk; D3's deny text then routes seats to a location they cannot write.
    resolution: >-
      Keep D4's principle (git-ignored, in-repo, cross-session) and re-pick the path outside every
      protected directory, e.g. a root-level ignored folder whose ignore entry run-open ensures;
      `.claude/worktrees/<run>/` is exempt and already ignored, but its cleanup behavior would need
      checking first. The source re-read clause applies: the lead re-reads the cited section.
  - id: S3
    severity: Critical
    decisions: [D7, D3]
    kind: broken load-bearing claim · rejected-road steelman
    finding: >-
      D7 resolves a home by the last `.mochiko/` segment anywhere in a path and rejects git-root
      resolution because it costs "a filesystem read on every call for the same answer". It isn't the
      same answer for nested fixture trees: this repo tracks 354 files under nested `.mochiko/` trees
      (evals/*/fixtures/**/.mochiko, crates/mochiko-cli/tests/fixtures/genesis-corpus/.mochiko), plus
      ignored eval workspaces and target/tmp crate fixtures. Today they sit outside every home (probe:
      a 400-line plan.md at evals/review-plan-artifacts/fixtures/g1-berth-waitlist/.mochiko/features/FEAT-001/
      is allowed). Under D7 they take live-home checks; under D3 any fixture path no home declares is
      denied outright.
    failure_scenario: >-
      A maintainer session writing a grader fixture with a planted structural defect (the input
      review-plan-artifacts and validation-constitution kits need) is denied, and the second deny
      halts; the eval corpus can then only be edited around the gate, which is the pattern D3 exists to stop.
    resolution: >-
      Resolve against the tree root the path belongs to (git toplevel, or the worktree root under
      `.claude/worktrees/<name>/`) rather than any segment, or state an explicit exclusion for
      fixture trees; add a nested-fixture case to D7's crate matrix. Also state whether today's
      "outside cwd is allowed" rule survives D7, and fix the root-docs clause: the binary has no
      by-name rule for root operating docs (`mochiko-cli home ROADMAP.md` → "no declared home
      governs this path"; prior R6 left them uncaught), and D7 leaves their resolution unstated
      (lead, Q4). Either rule it or drop the clause.
  - id: S4
    severity: Important
    decisions: [D9d, D3, F8]
    kind: coverage (map E1) · inconsistency
    finding: >-
      F8's defect is systemic, not brainstorm's alone. Every directory render target the
      artifact-home rules name resolves to the wrong home (0.112.0 and 0.114.0 roots, kinako tree):
      `.mochiko/features/FEAT-001` → features-index "NOT a declared deliverable";
      `.mochiko/specs/<slug>` → specs-index, same; `.mochiko/features/desk/<date-slug>` →
      features-index "declared sub-directory governed by its own home document, if one exists";
      `.mochiko/product/architecture` → product "NOT a declared deliverable"; `.mochiko/memory` and
      `.mochiko/epics/EPIC-001` → "no declared home governs this path". D9d fixes brainstorm only and
      rules `impl.artifact-home` "render target unchanged".
    failure_scenario: >-
      Under D3, `home` "says the same" as the closed-world deny for a path with no home, so a setup
      seat following setup.artifact-home is told `.mochiko/memory` is refused and pointed at run
      scratch, and an epic seat is told the same about its epic directory.
    resolution: >-
      One move in wave 1 or 3: `home` resolves a directory path to the home it names, or every
      artifact-home render target re-points to a file inside its home; add a matrix case per rule.
  - id: S5
    severity: Important
    decisions: [D6, OQ4, D4]
    kind: unchallenged assumption · excess machinery
    finding: >-
      The header cap cannot separate the log from honest reports. FEAT-001's reports carry their
      machine-first payload in the header by design: header medians are review 256, feasibility 162,
      final-validation 151, verification 130, cycle 89. Several honest headers match or exceed the
      384-line run log: s1-design-review 544, records-groom-review 534, sufficiency 396/383,
      spine-groom-review 356, verification up to 613, and the landing-fold-texts disclosure 809. The
      envelope's by_type is `{}`, so no type declares the payload OQ4 would size caps from. D4 also
      moves decisive evidence quotes into reports, so payloads grow.
    failure_scenario: >-
      A cap tight enough to stop a 384-line log denies ordinary review and verification reports. A
      cap loose enough to admit them does not stop the log, which can also split across several
      disclosures.
    resolution: >-
      With the log given a legal home, the loophole has no demonstrated user. Drop the cap
      (steelman rejected C), or first declare per-type payload schemas and size each cap from the
      observed distribution, handing OQ4 these numbers.
  - id: S6
    severity: Important
    decisions: [D2, F4, OQ1]
    kind: wrong fact · unchallenged assumption · coverage (map C1, G2, H)
    finding: >-
      (a) F4 lists `contracts/plugin-bridge.md` 1,276 as over "the product home's 300-line
      whole-file bound", but contracts live in the `product-contracts` home with `bounds: elsewhere`,
      so no gate size bound applies. D2 therefore adds a size check to contracts, including
      `api.yaml`, which has no markdown entry grammar, without field evidence. (b) Observed folds
      append a dated `## The FEAT-001 landing fold` block (127–187 lines) holding `[EXTEND]` entries
      for existing entities; the largest entity entry is 177 lines. "A landing fold that adds an
      entry is therefore a conforming write … no seat needs a carve" holds only for OQ1 numbers and
      a fold shape the record leaves open. An in-place amendment to an entry already over budget
      still worsens it and denies. (c) Two roads were never put: split each cumulative store into
      per-entry files (the `concerns/<AX-ID>.md` graduation pattern, bounded by the existing
      whole-file check, no new check kind), and the prior record's own route of splitting live
      violators by a named pass before size denies go live (prior D6/D11 wave 5), which kinako
      skipped (F1). F4 is partly that skipped precondition, not only the bound shape.
    failure_scenario: >-
      Wave 1 builds a per-entry check kind; the OQ1 table sets a tight entry budget; the next fold
      that extends BoundSession in place is denied, and the landing blocks again.
    resolution: >-
      Correct F4; drop `contracts/*` from D2 or justify it; make D2's guarantee explicitly
      conditional on OQ1 (the entry budget must admit the largest honest entry and the fold shape
      must be ruled); put the split-into-files road to the user with its cost to the store-authoring
      skills.
  - id: S7
    severity: Important
    decisions: [D3, D9d]
    kind: unchallenged assumption · coverage (map B12)
    finding: >-
      (a) The closed world's census is "of mochiko's own tree and kinako's", but shipped primitives
      write `.mochiko/` paths neither tree holds. The KM module template's collision ruling nests the
      module doc under `.mochiko/` ("the module's forward view lives at `.mochiko/ROADMAP.md`"). (b)
      Declared sub-directories with no home document are allowed unchecked today (probes: spec
      `prototype/index.html`, brainstorm `research/notes.md`), and the record does not say whether
      the closed world covers them. If it does, prototype and research writes break. If it does not,
      they are open doors inside `.mochiko/`, which is D3's own reason for rejecting C: "the next
      unplanned folder gets the same open door".
    failure_scenario: >-
      A brownfield consumer whose setup took the KM nesting ruling has every landing-ritual write to
      `.mochiko/ROADMAP.md` denied at first use.
    resolution: >-
      Take the census from the primitives' write sets (the rules and templates), with both trees as
      a cross-check, and rule the status of a declared sub-directory without a home document.
  - id: S8
    severity: Important
    decisions: [D9c, D3]
    kind: inconsistency (governance surface)
    finding: >-
      D9c says "the ledger pointer is untouched", but the ledger's GI-019 detail restates what D3
      changes. "Reach of the gate (D9, review C4): outside every declared home a `.md` write is gated
      only when its content opens with mochiko report frontmatter …", and the amnesty paragraph says
      "a write outside every declared home is ungated, not amnestied". Under D3 both are false for
      `.mochiko/`. The ledger also records condition C1 as DISCHARGED on a table the field shows does
      not admit honest content (F4, 9 whole-file denies in F2).
    failure_scenario: >-
      The build lands a governance surface that contradicts the gate it describes, a record-layer
      corruption under GI-005.
    resolution: >-
      Add a ledger text amendment (PATCH, recorded in the amendment log) to wave 3, or state in D9c
      why those paragraphs stay true.
  - id: S9
    severity: Important
    decisions: [D4]
    kind: wrong fact
    finding: >-
      "The same ignore line covers it" is false. impl.cold-verification ensures "the
      `/.claude/worktrees` ignore entry"; kinako ignores `.claude/worktrees/` and mochiko
      `/.claude/worktrees`, and `git check-ignore .claude/mochiko-runs/x/a.txt` finds no rule in
      either repo.
    failure_scenario: >-
      Raw output shows as untracked, gets copied into every cold-verification snapshot (`git ls-files
      -co --exclude-standard` copies untracked non-ignored files), and a broad `git add` commits it,
      which loses D4's "outside git" property.
    resolution: >-
      Run-open ensures an ignore entry for the run folder, the way impl.cold-verification does for
      worktrees, with a matrix or contract case.
  - id: S10
    severity: Important
    decisions: [F1, D9d]
    kind: inconsistency (governance floor GI-012)
    finding: >-
      mochiko-cli is not on crates.io (https://crates.io/api/v1/crates/mochiko-cli returns HTTP
      404), no `mochiko-cli-v*` tag exists, and the installed binary is a local-path `cargo install`.
      The hook bump (0.109.0, commit 5d8fc69, "bump staged, not landed") is on public main and
      reached kinako through the GitHub marketplace. The ledger makes the first publish with all four
      controls a hard precondition of that ship ("a maintainer break-glass install never substitutes
      for consumers"). F1 calls BACKLOG's "0.109.0 … not released" line stale, and D9d "corrects" it.
    failure_scenario: >-
      The bookkeeping overwrites the one honest trace of an unmet release gate, and wave 1's
      "mochiko-cli publish" goes out with the two owed controls unnamed.
    resolution: >-
      Record the gate status (a DECISIONS row or ledger note) instead of flipping the line; wave 1
      names signed tags and the crates-io approval rule as hard preconditions of the publish.
  - id: S11
    severity: Important
    decisions: [D3, D9b]
    kind: coverage (map B10, B8)
    finding: >-
      The closed world and the violator pass skip standing structure the field already shows.
      FEAT-002's `reports/evidence/` holds 93 files, beside FEAT-001's 68 and the archive's 166.
      Undeclared directories: `features/B53` (9), `features/B61` (1), `epics/EPIC-001/reviews` (15),
      `epics/EPIC-002/reviews` (8), `epics/EPIC-001/landing` (4), `features/FEAT-006/reviews` (4).
      Undeclared directories are never relaxed, so these files are already un-editable. There are
      also 47 pointers in 24 files into the evidence trees; the prior wave 5 priced 143 pointer
      repairs.
    failure_scenario: >-
      D9b deletes FEAT-001's evidence trees and leaves FEAT-002's, 47 pointers dangle, and legacy
      directories stay wedged after "the violator pass".
    resolution: >-
      Make D9b's pass a full inventory of the tree: every undeclared directory and every evidence
      tree, with pointer disposition (annotate "in history at <commit>" or repair).
  - id: S12
    severity: Minor
    decisions: [D5, F3]
    kind: wrong fact (mechanism)
    finding: >-
      F3's counts are exact (8 true and 13 false positives across the 21 denied commands), but two
      of D5's four matrix examples do not deny alone: `grep -n -i … <home>` → allow, and `git show
      <rev>:<home> > <scratch>` → allow (probes at 0.112.0 and 0.114.0). The real mechanisms are
      these. sed's `-i` test scans every later token in the whole command, so `sed -n … <home> |
      grep -i` denies. A separator glued to a word (`file;`) is not split, so a `tee` argument run
      continues across statements. cp and mv treat both ends as targets by deliberate prior design
      (the hook.rs comment on `mv` out of a home), and D5 does not engage that rationale for `mv`.
    failure_scenario: >-
      The crate matrix goes green without fixing the defect.
    resolution: >-
      Build the matrix from the 13 actual denied commands in the transcripts; rule `mv`'s source
      explicitly.
  - id: S13
    severity: Minor
    decisions: [D4]
    kind: wrong fact · inconsistency
    finding: >-
      "The gate does not watch `.claude/`" is false: the D9 sniff, which D3 keeps, denies a
      report-typed draft in the run folder (probe: `.claude/mochiko-runs/r1/draft-report.md` with
      `report: verification` → "sits under no declared home"). Seats whose cwd is a worktree resolve
      a relative `.claude/mochiko-runs/…` inside the worktree, so the folder is not "shared by every
      seat" unless the absolute path is printed. D4 also re-points reports' "full log" pointers into
      a folder deleted at acceptance, which reproduces the F11 tension instead of resolving it.
    resolution: >-
      State the sniff's reach over the run folder; the deny prints the main-tree absolute path;
      reports cite command and commit, not a full-log path.
  - id: S14
    severity: Minor
    decisions: [F5, D9b]
    kind: wrong fact
    finding: >-
      `.mochiko/archive/` holds 9 non-evidence files, not 22 (175 files, 166 of them evidence, all
      git-tracked). The 9 are 4 FEAT-001 groom snapshots, backlog-trail.md, 3 product-baselines
      from 2026-08-21, and spine-groom.md. D9b carries the wrong count. "History keeps them" is
      confirmed: every evidence file is tracked.
    resolution: Correct the count in F5 and D9b.
  - id: S15
    severity: Minor
    decisions: [D8, D9d]
    kind: unverifiable claim · excess machinery
    finding: >-
      "Trimmed to budget without loss" is asserted, not measured. Prior D10 made displacement
      "measured, not assumed away" (per-home total volume, reports-per-run, section sizes
      before/after), and the wave-4 watch drops those metrics and names no threshold or trigger. The
      dry-run rule makes every seat hand-build a PreToolUse JSON payload; the cheaper shape is a
      `check` mode taking path and content directly.
    resolution: >-
      Mark the no-loss claim as observed-not-measured, restore the displacement metrics and a
      trigger to the watch, and consider a direct dry-run flag.
  - id: S16
    severity: Minor
    decisions: [D9]
    kind: coverage (map A6, D4, B7) · passive acceptance
    finding: >-
      Three angles are unruled. First, the user's `!` shell stays the only override of a deny in
      every permission mode (F4 used it), and it is not recorded as sanctioned break-glass. Second,
      the four-wave build changes homes under any live run (run 3 already spanned 0.110.0 and
      0.112.0), and no run pins its log. Third, a flat open-name reports/ at 168 files per
      capability after two runs was not weighed against per-run namespacing. Separately, D9c's
      no-amendment call rode the Q10 batch rather than its own question.
    resolution: >-
      One line each in the record, ruling or dismissing them. A dismissed angle is a ruling.
withdrawn_or_commentary:
  - D1 root-cause framing — challenged, holds; the mechanism fixes (D5, D7) sit beside it without contradiction
  - D3 shell-leg evasions (interpreter writes, rsync, git restore) — beyond the three disclosed; the ledger already calls the gate "a floor, not the whole fence"; commentary
  - D9a phase-blind posture — a dismissed angle, ruled; no finding
  - feature-entry path grammar (map B11) — booked separately in BACKLOG; would not change a ruling here
  - advisory-checker and path-only roads (map H1, H2) — the user's direction was to harden; would not change a ruling
  - seat provenance — the record's reviewer line matches the team config (devils-advocate, opus)
  - F14's exact 385 — measured 384 header lines between delimiters; within counting convention
  - F2 per-kind split — totals reproduced (run 4 exactly 39 · Write 26 · Edit 7 · Bash 6; run 3 is_error Bash 15 · Write 20 · Edit 5); my kind classifier differs from the record's, so the split is unconfirmed, not wrong
verification:
  confirmed:
    - F2 totals per run and per tool (transcript script over main thread and every sidechain; hook-error results carrying the halt sentence)
    - F3 8 true / 13 false positives over the 21 denied commands; shapes (b) and (c) reproduced
    - F4 sizes (data-model.md 1,206 · constraints-and-decisions.md 733 · FEAT-001 baseline-delta.md 2,314) · APPLY.sh and APPLY-2.sh run as user `<bash-input>` · no run-4 commit touches those baselines
    - F5 `evidence/` undeclared in the feature home · 166 files in archive/evidence · 68 in FEAT-001 reports/evidence
    - F8 brainstorm render target misresolves (and five more; S4)
    - F9 commit f4c3ae5 "646 → 227"
    - F11 EVIDENCE-CAPTURE.md quote
    - F12 spot-check quotes ("book a plugin migration for `reports/evidence/`", "Evidence beside the builder's", the deny text "A new sub-directory takes a migration")
    - F13 feature home lacks implement-log.md at 0.114.0; epic home declares it
    - F15 cwd-dependent resolution reproduced (root cwd → sniff deny; worktree cwd → feature-home size deny)
    - F16 106 implement rules at 0.112.0 (111 at 0.114.0), implement-log named once
    - "hooks.json matchers: Write|Edit un-narrowed; Bash and PowerShell narrowed by an `if` on commands naming .mochiko; identical at 0.112.0 and 0.114.0"
    - both runs in permission mode `auto` (transcripts)
  wrong:
    - F4 contracts parenthetical (S6a)
    - F5 / D9b "22 further files" (S14)
    - F3(a) mechanism and two D5 matrix examples (S12)
    - D4 "the same ignore line covers it" (S9) · "the gate does not watch `.claude/`" (S13) · path viability (S2)
    - D6 / F16 "nothing reads it after acceptance" (S1)
    - D7 rejected-B "for the same answer" (S3)
    - F1 BACKLOG line "stale" on the GI-012 reading (S10)
  could_not_verify:
    - F7 edit counts (95 and 130)
    - F10 kinako memory files on report trims and usage-limit recoveries
    - F6 "4" worktree size denies (resolution reproduced, count not recounted)
    - D8 "without loss" (S15)
fitness:
  self_contained: pass — Topic, F1–F16, constraints, D1–D9, OQ1–OQ4 and the Q trail reconstruct the session
  decisions_attackable: pass — every D carries a rationale and named rejected roads
  decision_trail: pass for the session (Q1–Q10 map to D1–D9); the Review section is owed after this pass
  confidence_honest: partial — all nine read Confident on user rulings; D2's guarantee is stated as settled while it hangs on OQ1 (S6); D9c rode a batch (S16)
  rejected_roads: partial — missing the split-store road (S6), and git-root was rejected on a false premise (S3)
  honest_about_open: partial — OQ1–OQ4 are open; D2's and D6's dependencies on them are not marked
  provenance: pass — lead, reviewer seat, transport and blind dispatch stated
verify_round_1:
  date: 2026-09-23
  scope: folds of S1–S16 in the record as folded (607 lines), each graded against the updated record; no fresh cold read, no new blind-map hunt; new surface only where a fold introduced a contradiction
  result: not clean
  recommended_status: needs-revision
  status_basis: >-
    All 16 folds landed; S8 and S13 landed in part. The three Critical gaps are closed in
    substance. The folds introduced four blocking contradictions (V1–V4), each one a pen fix plus
    a bounded delta check. V2 must go back to the user: the Contested D4 ruling was given on a
    claim ("nowhere to spread") that S7's own fold makes false.
  tally: "16 folds checked, 16 landed (2 in part: S8, S13); 12 residuals, 4 blocking, 8 nits"
  folds_landed:
    - "S1: D6 299–305 · § Review 420–434 · Build 548–549. Landed; standing rulings go to .mochiko/decisions/, no row per checkpoint, the log stays ephemeral."
    - "S2: D4 246–251 · § Review 435–460 · Build 534–536 · OQ5 575–578. Landed; `Contested` is honest (the user overruled the lead's recommendation); source re-read 407–410."
    - "S3: D7 325–328 · § Review 461–472 · Build 532–533. Landed; tree-root resolution, nested fixtures never homes, root-docs clause dropped."
    - "S4: D9 amendment 396–397 · § Review 474–476 · Build 533, 540, 549–550. Landed; F8 88–91 not annotated (V6)."
    - "S5: D6 303–304 · § Review 477–479 · OQ4 572–574. Landed; stale D6 heading and statement (V5)."
    - "S6: D2 194–198 · F4 66–67 · § Review 480–488. Landed; OQ1 not updated (V9)."
    - "S7: § Review 489–493 · Build 542–544. Landed; introduces V2."
    - "S8: D9 amendment 391–393 · § Review 494–498 · Build 552–553. Landed in part (V4)."
    - "S9: D4 amendment 249–251 · § Review 446–448. Landed; closed structurally by the ignore guard."
    - "S10: F1 44–47 · D9 amendment 393–395 · § Review 499–504 · Build 540–541, 553, 556. Landed."
    - "S11: D9 amendment 389–391 · § Review 505–509 · Build 554–555. Landed."
    - "S12: D5 272–275 · § Review 510–514 · Build 536–537, 539–540. Landed; F3 55–62 not annotated (V6)."
    - "S13: § Review 449–450, 452–454 · Build 535. Landed in part (V3, V7)."
    - "S14: F5 76–78 · D9 amendment 391 · § Review 515–517. Landed."
    - "S15: D8 346–348 · § Review 518–522 · Build 538, 555. Landed."
    - "S16: § Review 523–528. Landed; three one-line rulings."
  blocking:
    - id: V1
      touches: [D4, D5, Build surface]
      lines: "D4 amendment 246–251 · § Review 442–455 · D5 255–264 · Build 534–537, 550"
      residual: >-
        `runs/` becomes a declared home, but the shell leg denies every shell write whose target
        resolves under any declared home (hook.rs `decide_shell`: "artifacts under declared homes
        are written with Write/Edit, never through a shell redirect"). The Build surface's
        write-position parse keeps that deny, and no fold carves out `runs/`. Raw capture is
        shell-born: F11's own shape is `command 2>&1 | tee …/verify-….log`, and testing-end-user
        re-keys to `runs/` (550). Every tee or redirect capture into the run folder denies, and
        the second deny halts the seat.
      resolution: >-
        Rule `runs/` as the one home that admits shell writes (a write-position target under
        `runs/` is allowed, and the ignore guard still applies). Every other home keeps the shell
        deny. Add matrix cases for `tee`, `>` and `2>&1` into `runs/`.
    - id: V2
      touches: [D4, D3, S7 fold]
      lines: "control 1 443–445 vs § Review S7 491–493 · Build 543–544 · the user's ask 440–442"
      residual: >-
        Control 1 says every other undeclared path under `.mochiko/` is denied, "so the pattern has
        nowhere to spread". S7's fold keeps `prototype/`, `inputs/`, `research/` and `referents/`
        inside the closed world, allowed by location with content unchecked. That leaves four
        doors where any raw dump conforms. The Contested C2 ruling was given on the answer that the
        CLI and hooks can hold the in-tree home.
      resolution: >-
        Either correct the claim to what the gate does, or give those four sub-directories content
        rules (e.g. `prototype/` its web file types; `inputs/`, `research/` and `referents/`
        markdown only). Then re-put the corrected claim to the user, since a Contested ruling
        leaned on it.
    - id: V3
      touches: [D4, D7]
      lines: "D4 230 (shared by every seat) · F12 107–115 · § Review 452–454"
      residual: >-
        "A worktree's own `.mochiko/runs/` resolves as that tree's home." So a seat whose cwd is a
        cold-verification worktree writes raw output to that worktree's `runs/`, not the run
        folder. That output is invisible to the lead and the other seats and is removed with the
        worktree. The snapshot copy (`git ls-files -co --exclude-standard`) skips the ignored
        main-tree `runs/`, so the worktree has no relative path to the run folder either.
      resolution: >-
        Make the run folder always the main tree's `.mochiko/runs/<run-id>/`, named by absolute
        path in seat briefs and in the deny and `home` texts. State whether a worktree's own
        `runs/` is used or refused, and add a matrix case.
    - id: V4
      touches: [D9c, S8 fold, D4 control 3, D7]
      lines: "D9 amendment 391–393 · § Review 494–498 · Build 552–553 · control 3 446–447 · D7 325–326, 464–466"
      residual: >-
        The PATCH re-words only the ledger's "Reach of the gate" and amnesty paragraphs. The
        GI-019 paragraph "What the gate reads (review C7): the raw PreToolUse payload … and, on
        Edit, the on-disk file" also becomes false: control 3 reads `.gitignore` on every write
        under `runs/`, and amended D7 walks ancestors for `.git`. This is S8's defect again, on a
        paragraph the folds themselves falsify.
      resolution: >-
        Add the "What the gate reads" paragraph to the wave-3 PATCH. State why the change is a
        PATCH clarification and not the amendment policy's MINOR "a principle significantly
        expanded" (ledger line 32; AM-3 itself was ruled MINOR).
  nits:
    - "V5: headings, statements and confidence lines still state reversed rulings without an inline pointer: D4 heading 226, 230–235, Confidence 243; D6 heading 277, 279 (\"never under a declared home\"), 284–287; D7 heading 307, 309–315, Confidence 323; D9 heading 350 (\"no governance amend\"); D9d 362–379 (segment resolution, frontmatter line budgets, header caps, `.claude/mochiko-runs/`, render target unchanged). D9's amendment block does not withdraw the header caps or segment resolution from the waves. Add an \"amended, see below\" marker on each."
    - "V6: F3 55–62 still gives the (a) mechanism and the `git show` example that S12 corrected, and F8 88–91 still reads brainstorm-only. § Review 511 says \"F3(a)'s mechanism corrected\", but F3 itself is not."
    - "V7: control 4 449–450 says \"no report or log hides in scratch\", but D6 puts the log there. Also, the D9 sniff runs only outside every home; inside the declared `runs/` home it is a new content rule and should be named as one (Build 535 \"sniff kept\")."
    - "V8: OQ2 567–568 still names `.claude/mochiko-runs/`, and 243–244 calls the run-id a build detail. Control 2 (446) makes the run-key form a mechanical check in the wave-1 crate, so OQ2 now gates a crate item; say so."
    - "V9: OQ1 561–566 is not updated with S6's two census obligations (an entry budget of at least 177 lines, and the fold shape). S6's \"rides D4e amnesty per entry (non-worsening allowed)\" (484–485) reads as a resolution, but an extension that grows an over-budget entry is a worsening and denies. Say so and name the split fallback as the route."
    - "V10: the Build surface numbers items 0–3 while D9d and the folds say waves 1–4 (348, 391, 399, 495, 521, 578). \"Wave 1\" reads as Build item 1 (migration) instead of the crate. Relabel one side."
    - "V11: \"re-ruled on its own line\" (393, 498) overstates it; D9c's re-ruling rode the review's I/M batch (473). Say so."
    - "V12: \"a SessionStart line naming run folders other than the current run's\" (451–452) needs state a stateless hook does not hold (Constraints 156). List every run folder instead."
  fitness_after_folds:
    self_contained: pass with the V5 caveat — superseded headings and statements still read as rulings without an inline pointer
    decisions_attackable: pass
    decision_trail: pass — § Review gives every disposition with its ruling form (re-put, three puts, batch)
    confidence_honest: pass with nits — D4 `Contested` is honest; 243 and 323 keep pre-review marks (V5); D9c's "own line" is overstated (V11)
    rejected_roads: pass — D4 456–460, D6 433–434, D7 472, D2 fallback 486–488
    honest_about_open: partial — OQ5 added and OQ4 closed; OQ1 and OQ2 not updated for the folds' new dependencies (V8, V9)
    provenance: pass — Status line 3–5 and § Review 401–416
  commentary:
    - "7 report-typed fixture files under nested `.mochiko/` trees (e.g. evals/review-plan-artifacts/fixtures/g1-berth-waitlist/.mochiko/features/FEAT-031/sufficiency-report.md) stay under the D9 sniff, as they are today; this is not fold-introduced (a same-content rewrite probes allow)."
delta_check:
  date: 2026-09-23
  scope: the V1–V12 repairs in the record as repaired (664 lines); each residual checked against its stated fold home, plus any contradiction the repairs introduced; no fresh cold read, no new hunt
  result: clean of blocking residuals; 7 nits remain
  recommended_status: ready once the lead's pen folds the 7 nits, with no further review round; if "log-shaped" (N3) means more than the existing sniff test, that one line goes to the user
  closed:
    - "V1: closed in substance — § Review control 4 460–462, S12 541–542, § Verify 560–562 (transcription gaps: N1; new tension: N2)"
    - "V2: closed — control 1 corrected 448–453, S7 511–513, user re-put § Verify 563–568; C2 stands on the corrected basis (wording: N3)"
    - "V3: closed for the worktree transport — 465–468, § Verify 569–571 (the cold snapshot case: N4)"
    - "V4: closed in § Review — S8 514–525 names the paragraph and the version class (Build surface not updated: N5)"
    - "V5: closed — amendment pointers on the D4 231, D6 282, D7 312 and D9 355 headings"
    - "V6: closed — F3 55–57, F8 91–93"
    - "V7: closed in § Review 457–459 (Build 587 and the D4 note 254 still say sniff: N1)"
    - "V8: closed — OQ2 622–624"
    - "V9: closed — S6 500–503, OQ1 618–621"
    - "V10: closed — Build surface 584, 594, 600, 606 carry wave 1–4 labels"
    - "V11: closed in S8 524–525 (the D9 amendment block 398 still says it: N6)"
    - "V12: closed — 464–465, consistent with OQ5 631–634"
  nits:
    - "N1 (V1/V7 transcription): the shell carve for `runs/` and the renamed content rule live in § Review only. D5 (258–280) does not state the carve. Build wave 1 lists the shell parse (588–589) without it and still says \"sniff kept\" (587). The D4 amendment note (254) still says \"the report sniff\". Add `· shell writes into runs/<run-id>/ admitted · runs/ content rule` to Build 587–589 and a one-line pointer in D5."
    - "N2 (new, V1 vs control 4): the gate cannot see what a shell write puts in a file. With shell writes admitted into `runs/`, `cat > .mochiko/runs/<id>/r.md <<EOF` with report frontmatter passes, so \"no report hides in scratch\" (458) holds for Write/Edit only. Cheapest fix: the carve admits non-`.md` targets only (raw captures are .log, .txt, .json); a `.md` goes through Write/Edit, where control 4 sees it. Or state that control 4 binds Write/Edit only."
    - "N3 (V2 wording): \"a report-typed or log-shaped `.md`\" (512, 566). \"Log-shaped\" is not a string-or-count test, and every other check in the record is (Constraints 157–158, GI-019 clause iv). Define it at the wave-1 matrix as the existing sniff test (report frontmatter, or a template's `## Header` signature, per the ledger's reach paragraph). If it means more, that line goes to the user. The Build surface (584–596) also omits the sniff's extension into the four sub-directories; add it to wave 1."
    - "N4 (new, V3 vs D7): V3's refusal presumes a git worktree, which carries a `.git` file (run 3's `git worktree add .claude/worktrees/mochiko-run3 -b run3/feat-001`). The impl.cold-verification snapshot is a plain copy with no `.git` (`git ls-files -co --exclude-standard … copied to .claude/worktrees/mochiko-<purpose>/`), so under D7 as amended its `.mochiko/` is a nested non-home tree. A write to its `runs/` is then neither refused nor redirected, and \"so a cold-verification seat's captures survive\" (467–468) holds for the worktree transport, not the cold snapshot. Say that the snapshot's tree is outside the gate and the seat brief names the main tree's absolute path, or refuse `.claude/worktrees/*/.mochiko/runs/` whatever the `.git` shape. Also, printing the main tree's path from a worktree reads the worktree's `.git` gitdir pointer, a third read for V4's paragraph."
    - "N5 (V4 transcription): Build wave 3 (604–605) still reads \"PATCH ledger amendment: GI-019 reach/amnesty paragraphs\". It omits \"What the gate reads\" and the version-class note (proposed PATCH, MINOR possible, ruled at the `/mochiko:setup` amend run, S8 518–522), and no wave lists that amend run."
    - "N6 (V11 residue): the D9 amendment block still says \"re-ruled on its own line\" (398)."
    - "N7 (Status line): lines 3–5 still read \"verify pass pending\"; update them for verify round 1 (not clean, repaired) and this delta check."
---

## Failure narrative

Status `critical-gaps`. Three load-bearing claims fail against the files and the platform docs.
First, D6 makes the run log ephemeral because nothing reads it after acceptance, yet 27 kinako
DECISIONS.md rows use run and epic logs as the rationale home for in-run user rulings (S1).
Second, D4's `.claude/mochiko-runs/` is inside a Claude Code protected directory, so every raw
capture prompts, goes to the classifier, or is denied (S2). Third, D7 rejects git-root resolution
as giving "the same answer", but last-segment resolution gates 354 tracked fixture files in this
repo that are outside every home today (S3). Eight Important survivors widen the brainstorm
render-target fix to six rules (S4), show the header cap cannot separate the log from honest
reports (S5), correct D2's contracts premise and its unconditional guarantee (S6), move the
closed-world census to the primitives' write sets (S7), and flag three missed surfaces: the
ledger's GI-019 text, the ignore line, and the GI-012 publish gate (S8–S10). Each survivor names a
resolution; none needs a new session.

## Notes of note

- Solo review. CROSS-EXAM.md's pair protocol is skipped. The lead's Q&A round is logged in `cross_exam`.
- S2 and S10 rest on fetched sources; per the source re-read clause the lead re-reads them cold
  before those findings survive.
- The blind map's 53 angles were diffed against D1–D9 and OQ1–OQ4. Coverage survivors are S4,
  S6 (in part), S11 and S16.
- Verify round 1 is in `verify_round_1`: not clean, with 4 blocking (V1–V4) and 8 nits; recommended
  `needs-revision`. V2 goes back to the user.
- Delta check is in `delta_check`: the V1–V12 repairs closed every blocking item; 7 nits remain.
  Recommended: `ready` once the lead's pen folds them.
