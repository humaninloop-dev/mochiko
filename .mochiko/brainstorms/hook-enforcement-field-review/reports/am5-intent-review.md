---
report: review
round: 1
session: hook-enforcement-field-review
reviewed: .mochiko/memory/governance-intent.md — the AM-5 entry at the top of `## Amendment Log` plus the header `Governs:` line (frozen 2026-09-24, Q1–Q5, pending ratification as v3.2.0 MINOR); AM-1–AM-4 read as context only
reviewer: intent-reviewer — cold intent seat (opus), mochiko:review-governance-intent, solo, blind-map two-message dispatch
blind_map: "sent to the lead as message 1 before synthesis contact (25 angles, 4 classes — A gate re-description · B owed PATCH · C breach · D cross-cutting); not written to disk"
floor_read_back: "class: floor · 16 rules — never-a-participant · author-grader · authored-surfaces-out · formulation-quality-excluded · its-command-states-them · default-fail · too-thin-first-finding · contested-audit-first · echo-rationales-outrank · declared-level-discipline · yardstick-never-taste · no-in-session-confirmation · evidence-floor · verdict-is-input · ratification-user-owned · findings-through-leads-pen"
recommended_status: critical-gaps
status_basis: >-
  One unrecorded ruling on a bright-line element (R1): the amend widens what the write-time gate
  reads (`.gitignore`, the `.git` pointer file) and checks (the ignore guard, the worktree `runs/`
  refusal) past the GI-019 admission paragraph's "decidable by string and count against data the
  log carries", with no ruling on whether clause (iv) covers it and the admission paragraph left
  off the moving-surface list. Resolvable in session by one user ruling; every other survivor is
  needs-revision class.
tally: "14 raised, 9 survived — 1 Critical, 3 Important, 5 Minor; 8 of the 9 trace to blind-map angles (coverage); 5 fell"
cross_exam: >-
  Solo; the pair protocol is skipped. 8 questions went to the lead, answered from the synthesis
  only, nothing withdrawn: Q1 (clause iv / ignore guard) not in the synthesis; Q2 (strike trigger)
  not chosen, no wave-2 or build-check mention; Q3 (breach mechanism) not recorded; Q4 (exposure,
  lighter mitigation, consumer set) not in the synthesis, only B and C were put; Q5 (backstop) none;
  Q6 (expiry strike set) amendment-policy paragraph and Testability limbs not named; Q7 (2026-09-24
  supersession, "product baselines") not accounted for, the phrase conceded as a possible lead
  error; Q8 bundling rationale unwritten (given in the room), Q5 unmarked, fact profile / modules /
  depth not stated unchanged.
facts_verified:
  - "2f92b09 = PR #36 merge, 2026-09-19 12:21 +1000; plugin.json and marketplace.json read 0.109.0 there; hooks.json carries artifact-gate.sh on three matchers"
  - "0.109.0 bump commit 5d8fc69 (2026-09-15, 'bump staged, not landed') is not an ancestor of main before 2f92b09 — it reached main only through PR #36"
  - "six bumps shipped the hooks: 0.109.0–0.114.0 (CHANGELOG headings)"
  - "no mochiko-cli-v* tag on origin (only eval-evidence-2026-09-19); crates.io API 404; release.yml:100 `if: false`; crates/mochiko-cli/Cargo.toml `publish = false`; installed binary 0.2.0, grammar 1..1"
  - "README.md:25 consumer install is `cargo install --git https://github.com/humaninloop-dev/mochiko mochiko-cli` — unpinned main HEAD"
  - "amnesty re-measurement (Q1 c) has evidence: hook-enforced-artifact-schema build-log.md:316, record.md:653"
  - "ledger header depth: high; modules compliance none — unchanged by AM-5 (declared-level discipline: no finding)"
survivors:
  - id: R1
    severity: Critical
    coverage: true
    map_angles: [A4, A6]
    elements: [AM-5 Q1, AM-5 Q4, GI-019 admission paragraph (ledger ~385–399), GI-019 clause (iv), .claude/rules/mochiko/rust-cli.md bright-line bullet]
    finding: >-
      The admission paragraph admits the gate on "a mechanical check" from a closed list (path ·
      file set · headings · frontmatter/enums · placeholders · per-section budgets · shell write
      operator) and asserts "Every check is decidable by string and count against data the log
      carries"; clause (iv) argues the gate is "structural validity of an artifact against the
      store's own declared shape". AM-5 adopts checks outside both predicates — the ignore guard
      (write refused unless `.gitignore` carries `.mochiko/runs/`, a repo-configuration read) and
      the worktree `runs/` refusal (reads the `.git` pointer file) — and a new size kind (per-entry).
      Q4 says reach and reads "widen" under "significantly expanded"; the driver's S8 said "no
      admission widened". The synthesis rules neither reading (cross-exam Q1) and the admission
      paragraph is not in the moving-surface list, so the producer would leave it false.
    failure_scenario: >-
      Ratified as drafted, the ledger's GI-019 carries a reads paragraph naming `.gitignore` and the
      `.git` pointer beside an admission paragraph saying every check is decided against log data —
      two homes contradicting each other on a bright-line element whose admission is valid ONLY by
      recorded ruling. The next audit of an admitting change (GI-019 Enforcement bullet 2) has no
      argument on record to grade the ignore guard against.
    resolution: >-
      One user ruling in session: does clause (iv) admit repo-state preconditions on a write (ignore
      guard, worktree refusal) — record the one-line argument either way (inside: a structural
      precondition of the declared `runs/` home; outside: a new admission) — and add the admission
      paragraph's check-kind list and "data the log carries" sentence, plus rust-cli.md's
      "per-section size" / "shape the log declares" bullet, to the surfaces expected to move.
  - id: R2
    severity: Important
    coverage: true
    map_angles: [A5, D1]
    elements: [AM-5 Q2 (Contested), build-state line]
    finding: >-
      Q2's `Contested` passes its rationale audit (trade-off put, roads A and C named and rejected,
      simplicity given) and is honored; its strike trigger is not. The line is struck at "the
      wave-1 release", but the paragraphs describe a gate that needs the wave-2 home migration too
      (`runs/`, `archive/`, `strips/`, `schema-views/` are declared at wave 2, driver § Build
      surface 2), and the driver still carries OQ1, OQ2, OQ3, OQ5 open on details the paragraphs
      state (run-key form, archive set, deletion owner). No step checks the paragraphs against what
      was built before the strike (cross-exam Q2). Consumers install unpinned main (README:25), so
      a merged wave-1 crate reaches them before any tag.
    failure_scenario: >-
      Wave 1 tags; the line is struck; the plugin still carries the pre-census log, so the ledger
      states a `runs/` home and closed world the installed plugin does not declare — or wave 2's
      census re-rules a control (D4 was already reversed twice) and the struck ledger silently
      disagrees with the gate (GI-005 record-layer integrity).
    resolution: >-
      Re-key the strike to the plugin bump that carries both the wave-1 binary range and the wave-2
      home migration, and make the strike PATCH include a read of the re-worded paragraphs against
      the released binary and log, correcting any divergence in the same row.
  - id: R3
    severity: Important
    coverage: true
    map_angles: [C1, C5]
    elements: [AM-5 Q3, exception-registry row]
    finding: >-
      Goal (3) is to record the breach honestly. The entry records that 0.109.0 reached main at
      2f92b09 but not how: git shows the bump was committed 2026-09-15 as "bump staged, not
      landed", with the CHANGELOG 0.109.0 "Release precondition" hold in the tree, and reached main
      inside PR #36 (primitive-evals-v2), an unrelated merge four days later; two governance events
      (v3.1.1, AM-4) then passed over it. The AM-3 guard on this gate was path-scoped globs on
      rust-cli.md, which inject on Read and cannot reach a PR merge. No recurrence measure is
      recorded (cross-exam Q3).
    failure_scenario: >-
      The next held bump staged on a working branch rides the next unrelated merge the same way;
      the exception row, read later, reads as a deliberate early release rather than a process gap.
    resolution: >-
      Add one mechanism sentence to the exception row (held bump, merged inside PR #36; the
      read-time guard cannot fire at merge) and ask the user for a light recurrence measure or an
      explicit decline — e.g. a held bump never staged in `plugin.json` on a shared branch, or a
      merge-time checklist line — recorded either way. Keep it light (no new machinery).
  - id: R4
    severity: Important
    coverage: true
    map_angles: [C3, C4, D3]
    elements: [AM-5 Q3, GI-002 risk surface (AM-2/AM-3 annotations), exception-registry row]
    finding: >-
      The exception excuses a supply-chain MUST NOT but never states the exposure it accepts. The
      live consumer route (README:25) builds unpinned main HEAD — no tag, no checksum, no signature,
      no approval — and runs it on every gated write and every fire. Only B (freeze) and C (pull)
      were put; no lighter mitigation (e.g. pin the README install to a reviewed `--rev`) was
      offered, and whether any consumer besides the maintainer exists is not recorded (cross-exam
      Q4). That consumer count is a user-declared fact, checkable against nothing on disk.
    failure_scenario: >-
      A third-party consumer installs after any main merge and runs code no gate reviewed at every
      artifact write; the ledger shows an exception "granted" with no stated risk, so ratification
      accepted an exposure nobody named — the review evidence of it lives only in conversation.
    resolution: >-
      Route to the user as confirmation: are there consumers besides the maintainer? Then state
      the accepted exposure in one line on the row (or a GI-002 AM-5 risk annotation), and put one
      lighter road (a pinned `--rev` install line) beside B and C for the user's ruling.
  - id: R5
    severity: Minor
    coverage: true
    map_angles: [C3]
    elements: [AM-5 Q3, exception-registry row]
    finding: >-
      The exception expires only at an event (the wave-1 publish carrying all four controls) and
      wave 1 has not started; no revisit date or backstop (cross-exam Q5).
    failure_scenario: >-
      Wave 1 slips indefinitely; the exception becomes a permanent waiver of the hook-ship gate by
      inertia, never re-ruled.
    resolution: >-
      Add a revisit trigger to the row's Expires/revisit cell (a date, or "at the next amend run")
      at which the user re-rules the exception if the publish has not landed.
  - id: R6
    severity: Minor
    coverage: true
    map_angles: [C6]
    elements: [AM-5 Q3, ledger amendment policy (~56–64), GI-012 Testability]
    finding: >-
      The expiry strike set names the region gates line and "GI-012's two paragraphs". The clause
      also lives in the amendment-policy first-publish paragraph and in GI-012 Testability's Pass
      limb ("the wave-4 hook-shipping bump lands only after a first publish") and Fail limb; the
      synthesis does not say those are struck or rewritten at expiry (cross-exam Q6). GI-012 holds
      one precondition paragraph; "two paragraphs" at AM-3 meant the policy one plus GI-012's.
    failure_scenario: >-
      At expiry the strike PATCH clears the region and GI-012 prose but leaves a live precondition
      in the amendment policy and a Testability limb testing a clause that no longer exists.
    resolution: >-
      Enumerate the full strike set in Q3: region gates line · amendment-policy paragraph · GI-012
      precondition paragraph · GI-012 Testability Pass and Fail limbs.
  - id: R7
    severity: Minor
    coverage: true
    map_angles: [A1, A8]
    elements: [AM-5 Q1 (clause-iv field-result note), AM-5 driver line]
    finding: >-
      The note says the table failed on "nine whole-file denies on product baselines and feature
      entry-class files". F2's nine are on `baseline-delta.md`, the feature
      `constraints-and-decisions.md`, `architecture.md`, `plan.md`, and a desk `derivation.md` —
      none a product baseline (F4's product baselines stand over bound at HEAD; their folds went
      through the user's shell, not a deny). `baseline-delta.md`'s entry-class candidacy and OQ1's
      fold-shape clause were superseded 2026-09-24 by `delta-files-vs-direct-baseline-edits`
      D1/D6a, which the driver line ("accepted 2026-09-23") does not note (cross-exam Q7).
    failure_scenario: >-
      A mis-stated field result enters the ledger as the discharge annotation of a ratified
      condition — a quiet record-layer error (GI-005) that the next reader of clause (iv) inherits.
    resolution: >-
      Re-word from F2/F4 as they stand (nine whole-file denies on feature-home and desk
      deliverables; product baselines 2–4× over bound, folded outside the gate) and add the
      2026-09-24 partial supersession to the driver line.
  - id: R8
    severity: Minor
    coverage: true
    map_angles: [D1, D3, D6]
    elements: [AM-5 Q1, AM-5 Q5, AM-5 entry as a whole]
    finding: >-
      Too-thin in three spots: Q1's bundling ruling carries no rationale (the reason given in the
      room — both drivers edit the same GI-019 paragraphs and the owed PATCH's trigger already
      fired — is unwritten); Q5 carries no mark; and AM-5, unlike AM-2 and AM-3, does not state
      the fact profile, modules, and depth level unchanged (cross-exam Q8). Four of five questions
      closed "as recommended"; the marks are the lead's own.
    failure_scenario: >-
      A later amend cannot tell whether AM-5 considered the fact profile at all while excusing a
      supply-chain gate, and the bundling that retired v3.1.3 has no reason on record.
    resolution: >-
      Write the room's bundling reason into Q1, mark Q5, and add one line: fact profile, modules,
      and depth level (high) unchanged — no module attaches.
  - id: R9
    severity: Minor
    coverage: false
    elements: [AM-5 Q1 (owed PATCH folded), wave5-bump-patch.md]
    finding: >-
      The entry calls the folded items "the AM-3 pre-authorized PATCH", but AM-3 pre-authorized
      only (a) the Testability activation and (b) the precondition strike; (c) the amnesty
      correction and (d) the stated-limits line are the lead draft's own additions (the draft's
      Authority paragraph says so). Harmless now, since AM-5 rules them, but the provenance is
      mis-stated.
    failure_scenario: >-
      A future reader reconstructing GI-006 provenance credits AM-3 with authorizing (c) and (d),
      text it never saw.
    resolution: >-
      Say "(a) and (b) pre-authorized at AM-3; (c) and (d) added by the draft and ruled here".
withdrawn_or_commentary:
  - "Date discrepancy (driver S10 '2026-09-15' vs AM-5 '2026-09-19'): AM-5 is right per git — the bump was committed 09-15 and landed on main 09-19. Fell."
  - "GI-002 tech-stack line lacks the hook role: pre-dates AM-5, not falsified by it; taste. Fell."
  - "Amnesty re-measurement in Q1(c) unevidenced: evidence found at build-log.md:316 and record.md:653. Fell."
  - "Lead checks before the questions (six bumps, 404, no tag, `if: false`, PR #36): all verified first-hand. Fell."
  - "rust-cli.md delegated to the producer: folded into R1. Fell as a separate item."
  - "Over-governance hunt: nothing minted beyond the three topics; the per-bump CHANGELOG citation is the lightest forcing surface. No survivor."
verify_round_1:
  date: 2026-09-24
  scope: "bounded — the 9 folds in the AM-5 entry (driver line, Q1, R1 bullet, Q2, Q3, Q5, Unchanged, Surfaces, Review, Ratified); no fresh angle hunt"
  status: "CLEAN on blocking — 9/9 folds landed faithful to their dispositions; 3 non-blocking residuals; no mark upgraded past its ruling"
  folds:
    - "R1 landed: the admission predicate widened to 'data the log carries and the repository's own layout, with no judgment', argument recorded (the bright line bars judging or ordering work, not reading plain repo facts), roads B/C rejected with reasons; the check-kind list and the rust-cli.md bullet are on the Surfaces list. Mark Confident = user-ruled 'as recommended'."
    - "R2 landed: strike at the plugin.json bump carrying the wave-1 binary range plus the wave-2 home migration, never the tag alone; text-vs-build check at the strike, drift corrected in the same row. Q2 keeps `Contested`, no upgrade."
    - "R3 landed: mechanism (5d8fc69 'bump staged, not landed', merged inside PR #36; Read-time rules cannot fire at a merge) and the held-bump sentence into GI-012, also on the Surfaces list; rejected roads named."
    - "R4 landed: user-confirmed fact dated 2026-09-24 (no installer besides the maintainer), exposure stated (unpinned main, no tag/checksum/signature), tripwire, the pinned --rev road put and rejected with a reason."
    - "R5 landed: backstop 2026-12-31, labelled 'go' on the lead's recommendation — honest about the batch."
    - "R6 landed: expiry strike set enumerates the region line, GI-012's paragraphs, the amendment-policy limb, and GI-012 Testability Pass and Fail."
    - "R7 landed: F2's nine denies named as feature-home and desk files, F4's baselines stated as folded through the user's shell; the 2026-09-24 partial supersession cited in the driver line."
    - "R8 landed: bundling reason with rejected roads in Q1, Q5 marked Confident (user-ruled 'as recommended'), Unchanged bullet (fact profile, modules, floor, waivers, depth high)."
    - "R9 landed: provenance line — AM-3 pre-authorized (a)/(b) only; (c)/(d)/(e) ruled in Q1."
  residuals:
    - id: N1
      severity: Minor
      finding: "R1 widens the admission paragraph's predicate, but the clause (iv) argument body (ledger :400, 'mechanical conformance is structural validity of an artifact against the store's own declared shape') is not on the Surfaces list — the Surfaces list names only the clause-iv C1 note. The ignore guard and worktree refusal are repo-layout preconditions, not artifact structural validity, so the producer could leave clause (iv) narrower than the predicate beside it."
      fix: "Add 'the clause (iv) argument body — carry R1's rationale sentence' to the Surfaces bullet."
    - id: N2
      severity: Minor
      finding: "The re-worded clause-iv note still opens 'the ratified table did not admit honest cumulative content' and then cites `plan.md` and a desk `derivation.md` among the nine denies — per-run deliverables, not cumulative stores."
      fix: "Open with 'did not admit honest content' (or split: cumulative stores per F4, per-run deliverables per F2)."
    - id: N3
      severity: Minor
      finding: "The Review bullet says a '37-angle map'; the map sent as message 1 held 25 angles in 4 classes (A9 · B4 · C6 · D6), as this report's `blind_map` field records."
      fix: "Correct to 25."
  new_contradictions: "none blocking — R1's widening is consistent with Q4's MINOR reading (a widened admission under an unchanged principle, the AM-3 precedent); N1 is the one seam the folds opened."
  mark_audit: "no upgrade past ruling: Q1, R1, Q3, Q4, Q5 Confident each rest on a recorded user ruling; Q3's backstop and the R6–R9 repairs are disclosed as a batched 'go' on the lead's recommendation after the user asked for less depth — recorded, not hidden."
  commentary: "The R4 tripwire ('anyone else starts installing') and the R5 backstop ('at the next setup run') both depend on the maintainer noticing — no telemetry exists. Both were ruled; this note is not a finding."
---

## Failure narrative

Blocking: R1. AM-5 moves GI-019's reads and reach past the admission paragraph's own predicate
("every check is decidable by string and count against data the log carries") — the ignore guard
reads `.gitignore`, the worktree refusal reads the `.git` pointer — and neither the synthesis nor
the driver rules whether clause (iv) covers that. The driver claimed "no admission widened" (S8);
AM-5's Q4 says the reads widen; nothing reconciles them, and the admission paragraph is not on the
moving list. Under GI-019 an admission stands only by recorded ruling, so this is an unrecorded
ruling on a bright-line element: `critical-gaps` by the status criteria, resolvable by one user
ruling in session. Tried: cross-exam Q1 — confirmed not in the synthesis.

## Notes of note

- Contested audit: Q2's `Contested` holds its rationale (the trade-off was put, the rejected roads
  are named); honored. The strike-trigger defect (R2) lies in its execution, not in the choice.
- Declared level: depth high is unchanged in the ledger; nothing here is graded above it.
- R4 carries a user-declared fact (the consumer count): route it to the user as a confirmation,
  not as an argument.
- The skill's survivor-report rule says "no report files"; this file exists at the lead's
  explicit instruction, and the survivor message went to the lead as well. Dispositions belong in
  the synthesis's Review section, in the lead's hand.
- Probe disclosure: the crates.io check sent a User-Agent carrying the user's email address. It
  was a read-only GET. I disclosed it to the lead, and no further external probes were run.
