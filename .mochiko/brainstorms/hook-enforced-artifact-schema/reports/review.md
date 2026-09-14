---
report: review
feature: hook-enforced-artifact-schema
round: 1
skill: mochiko:review-brainstorm
lens: solo cold review (decision-quality + record-integrity, unsplit)
pass: cold
seat: cold-review (teammate, session tier)
date: 2026-09-13
graded: .mochiko/brainstorms/hook-enforced-artifact-schema/record.md (frozen 2026-09-13, 470 lines, D1-D11, F1-F13, OQ1-OQ4, Q1-Q7)
dispatch: blind-map two-message; map sent before record contact, fence held (no read of the session dir or the brainstorms index before message 2)
status: critical-gaps
tally: {raised: 30, survived: 25, critical: 5, important: 12, minor: 8}
strengths: F10/F13 are real field evidence with the drift class named, not asserted; D7 routes the supersession through an amend run instead of landing a carve in a script; D4 draws the mechanical/judgment line explicitly against GI-019; D6 refuses to derive budgets from observed sizes; D8 carries an abort criterion and absorbs the backlog's owed channel probe; F8 marks its own inferences unverified; the adoption streak is disclosed, not hidden.
---

## Failure narrative

The record is a competent, well-evidenced design that rules on the wrong step first. Its
diagnosis (F10, F13) shows two distinct causes with very different cures: one delivered rule
read and overridden (n=1), and roughly twenty-two of thirty artifact kinds whose home, file set,
and shape were **never declared anywhere** — not in the log, not in a template, not in a rule.
D1 treats the first as proof that declaration cannot work and jumps to a write-time deny, and no
decision or rejected road covers the road the evidence actually points at: declare the homes,
sets, and kinds in the log, deliver them at fire through the channel that already exists, and
measure what still drifts. Four further Criticals are mechanical: the exit-code mapping D3
specifies contradicts the fail-open floor D7(c) re-ratifies and the posture of the script it
claims to copy; existing live artifacts get no first-touch amnesty, so the first Edit to
kinako's oversized files is denied and work stops; the deny text instructs the user to declare a
name through a surface that does not exist for a consumer; and a `Write|Edit`-matched gate does
not see a file written through the Bash tool, while D9's two-strike halt is an instruction to a
model the record itself says can read a deny and route around it.

Three load-bearing figures are wrong (checked against the trees): kinako `.mochiko/` is 288
files / 115,757 lines, not 284 / ~114,800; `FEAT-002/tasks.md` is 1,731 lines, not 1,452;
`FEAT-001/tasks.md` is 1,003, not 836. Two of F10's seven named EPIC-002 spine files
(`plan.md`, `design-closure.md`) are not in that tree at all. The census and the budget table are
told to build on these numbers. One platform fact is wrong in the other direction: hook merge
order **is** documented, along with `disableAllHooks` and `allowManagedHooksOnly`, which bound
what any deny can guarantee in a consumer repo.

Confirmed as claimed: F1 (hooks.json, three registrations), F2 (`cli-schema-delivery`
record.md:579 "if ever wanted it takes its own ruling from zero"; D7(c) maintainer-side
PostToolUse at :571), F3 (GI-019 wording), F4 (eight templates, no `check`/`validate`
subcommand), F6 (eval-v2 F12, probe 2026-09-05), F9 (four `paths:`-scoped rules files; no
machine path check anywhere), F10's EPIC-002 file set, the `## EPIC-002 cycles — 2026-09-10`
append, the `C2-n` card ids (144 occurrences), the missing `## Header` / `## Cycle Cards`,
`B53` with nine cycle reports, both desk-output homes, all five undeclared brainstorm review
names, F12's ADR (`.mochiko/decisions/2026-08-22-verbosity-envelope-enforcement.md`), and
`impl.reports-envelope`'s text. Also checked and **not** found: any mochiko primitive that
prescribes seeding from a prior run's artifacts (0 hits for seed-source / prior-epic language in
the log or skills) — the sibling-seeding in kinako's review artifacts was run-authored, so D1(a)
aims at the right moment.

## Phase 0 blind angle map (55 angles, as sent before record contact)

### A. Diagnosis of the reported failure
1. The substitution is reported, not demonstrated — a record needs an observed trace.
2. Which channel fails: main session, `Agent` subagent, teammate, headless `-p`.
3. Which surface is substituted: rules, templates, docs, or location.
4. Skip versus supplement: CLI never fired, or fired and overridden.
5. Location may be unstated rather than unenforced — no template carries an output path.
6. Attractor inventory: `.mochiko/schema-views/**`, `plugins/mochiko/templates/*.md`, prior artifacts, stale `.specify/` or `docs/templates/`.
7. Sibling mimicry is indistinguishable at tool level from legitimate reading.
8. Legitimate reads the fence must not break: reviewers, groomers, producers reading prior facts.

### B. Platform facts
9. PostToolUse cannot block or undo — post-write checking is corrective only.
10. PreToolUse is the only preventive channel (deny, or exit 2 with stderr).
11. PreToolUse can rewrite via `updatedInput` — a distinct, riskier design than denying.
12. Matchers key on tool name only; path-shaped gating needs the `if` field.
13. `if` is best-effort for Bash — a shell write evades a Write/Edit gate.
14. Hooks fire inside subagents with `agent_id` / `agent_type` on input.
15. Skill and subagent frontmatter hooks are a scoped alternative to plugin-wide hooks.
16. Hooks merge across settings levels; a consumer can disable them.
17. mochiko's ratified posture is fail-open, absence-or-skew only, 5-s timeout (D7).
18. `.claude/rules/mochiko/rust-cli.md` forbids hooks blocking on behavior.
19. `producer-plan-enforcement` D1 already declined behavior gating.
20. Which output channel carries the finding: `additionalContext` / `systemMessage` / exit-2 stderr.
21. Non-blocking hook failure is near-silent; a timeout discards output.
22. Timeout budget against a per-write replay.
23. Headless, policy-disabled, and bypass modes.
24. Windows Git Bash and non-interactive shell constraints for a new script.

### C. CLI-side design
25. No validating verb exists; `migrate validate` validates the log, not an artifact.
26. GI-019: grading a produced file is the nearest approach yet to the bright line.
27. The mechanical/judgment split drawn explicitly.
28. Where location truth is minted — a registry in the log, named.
29. Variable path segments (`<feature>`, `FEAT-XXX`, dated desks).
30. The enforcer must read the log, never a view (no-Read assert).
31. Exit codes 0-3 are spoken for; non-conformance needs a fifth or a documented reuse.
32. Cost per fire against the measured 35 ms / 66 B baseline.
33. Grammar skew versus unknown-subcommand at the hook.

### D. Cheaper shapes to steelman
34. Subtractive: delete or relocate the attractors.
35. The views' own "never hand-edited" marker — why a marker fails where a hook succeeds.
36. Rule-layer `fence` label — a read-boundary with zero new machinery.
37. Gating Read rather than Write.
38. PostToolUse advisory plus the existing landing/audit gate.
39. The null road: a prompt-precedence defect fixed in primitive text.
40. One delivered precedence rule: the render outranks any on-disk file.

### E. Governance interactions
41. GI-020: a new subcommand raises the required-binary floor and forces a coordinated bump.
42. GI-012: crate tests, contract cases, view-equals-replay, unchanged render shape.
43. Primitive-edits ceremony scope and the edit count committed to.
44. Path-scoped rules inject on Read, not Write — never fire for a first write.
45. Consumer reach: the plugin's hooks are the only channel that arrives with the plugin.
46. Self-hosting asymmetry between this repo and a consumer repo.
47. Who owns location as a skill — no named home today.

### F. Failure modes of the fix
48. False positives on scratch drafts, reports, desk cards, user-owned markdown.
49. Deadlock: denying the write the run was dispatched to produce.
50. Loop: deny, retry, deny — who breaks it.
51. Evasion taught by the gate.
52. Silent non-enforcement: unexecutable script, unresolved root, timeout, consumer disable.
53. Retrofit: artifacts already on disk that a new gate now rejects.

### G. Evidence and rollout
54. Probe matrix proving substitution before and absence after, plus a pilot.
55. What a record could omit and still look complete: the reproduction trace, the D7 fail-open supersession, the `rust-cli.md` strip, the `producer-plan-enforcement` precedent, the matcher-versus-`if` fact, the Bash hole, the consumer disable, the per-write cost, the subtractive road, and location's skill home.

## Survivors

```yaml
findings:
  - id: C1
    sev: Critical
    class: rejected-road steelman (map angles 39, 40, 47)
    at: "D1 rationale + D11 (build order); silent on the road itself"
    gap: >-
      The record rejects "reminder everywhere, no deny" citing one disclosed override
      (impl.reports-envelope, F10) and generalizes it to all drift. But F9/F13 show roughly 22 of
      30 kinds have no template, no declared home, and no delivered rule at all — for the
      dominant share of the observed drift there was never a rule to override. The road the
      evidence points at is never put: mint the `home` registry (D3) and deliver it as rules at
      fire through the existing ratified channel, then measure the residual.
    fix: >-
      Split D11: a wave that lands the `home` kind and DELIVERS it (homes, file sets,
      kind-to-template bindings, per-section budgets) as rendered rules in every producing
      command and skill, plus the D1(a) read-time reminder, with one kinako run measured against
      it; the write-time deny scoped at the next ruling to the classes that still drift. Add the
      road to D1's rejected list only if the user declines it with the measurement in hand.
    disposition: reopen (D1 scope + D11 wave order)

  - id: C2
    sev: Critical
    class: inconsistency (map angles 21, 31, 33)
    at: "D3 ('exit code the hook's signal', 'the same posture as dependency-halt.sh') vs D7(c) fail-open floor"
    gap: >-
      Passing the CLI's exit code through as the hook's signal inverts the floor. Verified
      contract (crates/mochiko-cli/src/cli.rs): 0 ok, 1 log absent or unsound, 2 usage error or a
      name the log does not carry, 3 version contract. PreToolUse reads exit 2 as BLOCK with
      stderr as the reason — so a consumer whose binary predates `check` has every gated write
      denied with a clap usage message, while a genuinely unsound log (exit 1) and a skew (exit 3)
      fail open. dependency-halt.sh does the opposite on purpose: it emits a JSON decision, always
      exits 0, and leaves every non-zero CLI code alone but exit 3.
    fix: >-
      State the mapping in D3: the wrapper emits `permissionDecision` JSON and exits 0 always;
      only a conformance verdict from `check` produces `deny`; CLI exit 1, 2, and 3 are
      pass-through-silent (proceed) exactly as dependency-halt.sh treats them, with absence and
      skew already covered by the shipped halt hooks. Add a contract case per exit code.
    disposition: repair (D3 statement + D10 contract cases)

  - id: C3
    sev: Critical
    class: missing intra-decision dimension (map angle 53)
    at: "D6 ('the first dogfood run is expected to trip denies, which is the signal working') + D4(e) + D11 wave 5"
    gap: >-
      Nothing grandfathers artifacts already on disk. D4(e) evaluates an Edit against the
      in-memory result, so with tight per-section budgets every Edit to an existing oversized file
      is denied: kinako's FEAT-002 tasks.md is 1,731 lines, FEAT-001 1,003, EPIC-002's
      implement-log.md 3,653, the EPIC-002 tree 11,713. That is not a signal, it is a wedge — the
      live repo cannot be edited until each file is split or rewritten, and the gate offers no
      path to do the rewrite (the rewrite itself is a denied write).
    fix: >-
      Add to D6 or D4: a first-touch rule. On an Edit whose pre-edit file already violates a
      budget, the check compares the delta — a write that does not worsen the violated measure is
      allowed with the overage reported as context, and only a worsening write is denied. Name a
      migration act in D11 (a pass that splits or re-homes the existing violators) as a
      precondition for enabling size denies in a live repo.
    disposition: repair (D6 statement + D11 wave 5 precondition)

  - id: C4
    sev: Critical
    class: unchallenged assumption (map angles 44, 45)
    at: "D2 (deny reason ends 'ask the user to declare it') vs D3 (the registry lives in the plugin's migration log) + GI-020"
    gap: >-
      A consumer cannot declare anything. The `home` registry ships inside the plugin's log; no
      schema file ships (GI-020) and the CLI resolves one log directory (flag, then
      MOCHIKO_MIGRATIONS, then plugin root) with no merge of a consumer-local log. So the deny
      instructs the user to do the one thing the design forbids, and every new artifact name in a
      consumer repo blocks on an upstream migration plus a plugin release — with the crate itself
      not yet published (BACKLOG: the wave-2 publish tail, `mochiko-cli-v0.1.0`, is still owed).
    fix: >-
      Rule the escape explicitly in D2: either (a) a consumer-local declaration surface the CLI
      merges over the plugin log (a `.mochiko/homes.yaml` class file, which needs its own GI-020
      argument since it is a readable rule file), or (b) the deny text drops the instruction and
      names the real route — "this name is not declared; a new deliverable kind takes an upstream
      migration, so put the file under the home's `reports/` or raise it with the maintainer". Add
      the crate publish step to D11 as wave 1's exit condition.
    disposition: ruling (D2 deny text + D3 resolution order; user's call on the local surface)

  - id: C5
    sev: Critical
    class: unchallenged assumption (map angles 13, 51)
    at: "D1(b) matcher `Write|Edit`; D9 two-strike halt; silent on Bash"
    gap: >-
      The record never mentions the Bash tool. A matcher is a tool-name regex (F8's own fact), and
      a file written by `cat >`, a heredoc, `sed -i`, or `python -c` is not a Write or an Edit
      call, so the gate never fires. The docs state nothing either way, and F8 already records
      that a deny "reaches the model as text it can read and route around". D9's answer is a
      sentence in the deny reason asking the model to halt — the same obedience-dependent control
      F10 documents failing.
    fix: >-
      Add a wave-0 probe leg (D8): does a `Bash` heredoc write to a gated path fire any hook, and
      does a denied seat retry through Bash. Then rule the matcher set in D1(b) — either extend to
      `Bash` with the `if` narrowing (accepting its documented best-effort parsing) or record the
      hole with a stated mitigation (the reviewer's read of the file set at fan-in). Replace D9's
      halt instruction with a mechanism the hook owns, or state plainly that the halt is advisory.
    disposition: reopen (D1 matcher set + D8 probe legs + D9 halt mechanism)

  - id: I1
    sev: Important
    class: rejected-road steelman (map angle 38)
    at: "F8 ('the docs name outright' a Stop hook scanning the tree once per turn); D7(c) precedent; no decision weighs either"
    gap: >-
      The platform's own documented pattern for "must see every file change" is recorded as a
      ground fact and then never appears in any rejected-roads list, and `cli-schema-delivery`
      D7(c)'s maintainer-side PostToolUse advisory — already admitted as not kernel-class under
      GI-019 — is never weighed as the complement for the classes a deny cannot hold (existing
      files, Bash writes, hook-disabled consumers, teammate transports).
    fix: >-
      Add to D1's rejected roads, with reasons: a `Stop` per-turn tree scan and a PostToolUse
      advisory. If both are rejected on the no-generate-then-fix driver, say so; consider keeping
      one as the disclosed-hole mitigation rather than leaving the holes uncovered.
    disposition: fold (D1 rejected roads)

  - id: I2
    sev: Important
    class: missing intra-decision dimension (map angle 40)
    at: "D1 rationale ('a reminder there cannot shape the draft'); Q2 driver ('I dont want the files to be generated and then we fix it . token wastage')"
    gap: >-
      A PreToolUse deny does not prevent generation — the draft is already in
      `tool_input.content`, so the tokens are spent and a deny costs a second full draft. The
      channel that serves the stated driver is injection BEFORE authoring: the plugin already owns
      a `PreToolUse` hook on `Skill`, which fires before an authoring skill's procedure runs and
      could carry the kind's skeleton, file set, and section budgets. The record never considers
      it.
    fix: >-
      Add to D1: a third channel, the authoring-time injection of the target kind's conformance
      block at `PreToolUse:Skill` (or in the producing command's rendered rules), with the deny
      kept as the floor. State honestly in the rationale that the deny bounds persistence, not
      generation cost.
    disposition: fold (D1 statement + rationale)

  - id: I3
    sev: Important
    class: excess machinery / cost (map angles 22, 32)
    at: "D1(a) fires on every `Read|Glob|Grep` under a home; OQ4 measures per-call cost only"
    gap: >-
      No aggregate bound exists. In a repo with 288 artifact files and read-heavy review work,
      every read spawns a process and replays the whole log (wave-1 measured 35 ms cold, 599 KB
      genesis). OQ4's abort line is the 5-second timeout, which is the wrong bound: a 200 ms check
      firing on several hundred reads per run is a large tax well inside it. The
      `${CLAUDE_PLUGIN_DATA}` cache that `cli-schema-delivery` D1 deferred "on measured need" is
      never revisited, and the `if` field that would narrow the matcher to home paths is recorded
      in F8 and unused.
    fix: >-
      D8 measures aggregate cost per run, not per call, with a stated per-run budget as the abort
      line. D1 narrows both registrations with `if` path patterns. D3 rules whether `check` caches
      the replayed state; the cheaper shape for the reminder is a once-per-session fire.
    disposition: repair (D1 matcher narrowing + D8 measurement + OQ4 bound)

  - id: I4
    sev: Important
    class: inconsistency / buildability (map angle 24)
    at: "D3 ('thin POSIX sh wrappers around those two subcommands')"
    gap: >-
      The existing wrapper's JSON reader (hooks/scripts/dependency-halt.sh `field()`) is
      grep-and-sed over a newline-stripped payload, "deliberately not on jq", and extracts short
      bare identifiers. It cannot extract `tool_input.content` — multi-line, escaped quotes,
      escaped newlines — nor `old_string`/`new_string`. So the wrapper is not thin: it either
      takes a jq dependency in every consuming project or the content never arrives intact.
    fix: >-
      Change D3's interface: `mochiko-cli check --hook-json -` consumes the raw hook payload on
      stdin and does its own parsing in Rust, leaving the wrapper to pipe stdin through and emit
      the decision. The CLI already owns serde.
    disposition: repair (D3 subcommand interface)

  - id: I5
    sev: Important
    class: inconsistency (map angle 8)
    at: "D2 ('`reviews/` is not a home — reviews are reports and land in `reports/`') vs D5 ('a declared review home for the cold review's artifacts')"
    gap: >-
      The two decisions give reviews two different homes. The live case is this session: the
      dispatch writes `review.md` beside `record.md` in the brainstorm home, which under D2 is an
      undeclared name in a home with a closed set and would be denied; mochiko's own repo uses
      `review-lens-a.md` there, and kinako uses five other names (F13).
    fix: >-
      Rule one shape. Either brainstorm review artifacts are reports under
      `.mochiko/brainstorms/<slug>/reports/` (D2's rule, and D5 drops the separate review home),
      or the brainstorm home declares `review.md` (plus the pair/verify names) in its set and D2's
      sentence gains the exception. Whichever wins, declare it before wave 4 or the gate denies
      the review of its own session.
    disposition: repair (D2 + D5 harmonization)

  - id: I6
    sev: Important
    class: inconsistency with governance (map angle 18)
    at: "D5 (root operating docs in the registry, 'location + size') + D6 budget table"
    gap: >-
      The root operating docs' caps and bounds already have a home: the KM invariants at
      `.mochiko/memory/knowledge-management.md`, with a tripped cap routed to
      `mochiko:grooming-operating-docs`. A second numeric home in the budget table restates a
      constraint that exists, which GI-017 forbids ("Governance surfaces point at existing
      constraint homes; they MUST NOT restate them"), and leaves no rule for which surface wins.
    fix: >-
      In D5, scope the registry over the operating docs to LOCATION only, and have the budget
      table cite the KM invariants for their bounds rather than restating numbers. If a mechanical
      size check on those files is wanted, mint it as the KM cap's evaluator, not as a second cap.
    disposition: repair (D5 scope + D6 table sourcing)

  - id: I7
    sev: Important
    class: unchallenged assumption (map angle 51)
    at: "D4(a) ('`###` and deeper are free') + D6 (per-section budgets) + D2 (`reports/` open by name)"
    gap: >-
      The gate measures lines per `##` section while three escape routes stay open: nest the prose
      under a free `###`, split it into a new file in the open-by-name `reports/` dir, or append it
      to the per-entry-bounded log. Total artifact volume — the user's actual complaint (F13's
      115,757 lines) — is measured nowhere. Separately, whether a section's `max_lines` counts its
      nested `###` content is unstated, so the check is undefined as written.
    fix: >-
      State in D4 that a section's bound counts every line until the next `##` (nested content
      included), and add a per-home total-volume measure to the D10 dogfood watch so displacement
      is visible. Consider a report-count-per-run figure in the watch.
    disposition: repair (D4 counting rule + D10 watch metric)

  - id: I8
    sev: Important
    class: inconsistency / broken load-bearing claim (map angle 55)
    at: "F13 (284 files, ~114,800 lines) and F10 (FEAT-002 1175→1452, FEAT-001 651→836; `plan.md`, `design-closure.md` in the EPIC-002 spine)"
    gap: >-
      Counted at HEAD: kinako `.mochiko/` holds 288 markdown files / 115,757 lines; FEAT-002
      tasks.md is 1,731 lines; FEAT-001 tasks.md is 1,003. `plan.md` and `design-closure.md` are
      not in `.mochiko/epics/EPIC-002/` at all — they are per-feature files, so two of F10's seven
      named spine files are mis-homed. The per-dir splits, EPIC-002's 11,713, implement-log's
      3,653, the `C2-n` ids, the missing `## Header` / `## Cycle Cards`, B53's nine reports, and
      both desk homes all check out.
    fix: >-
      Correct the four figures and move the two file names to the feature-home sentence. D11's
      census re-counts at its own date rather than inheriting these numbers, and D6's budget table
      states the count date.
    disposition: repair (F10 + F13 figures)

  - id: I9
    sev: Important
    class: unverifiable claim / missing dimension (map angles 16, 45)
    at: "F8 ('merge order with project/user hooks *not in docs*'); D7(c) re-ratifies the floor"
    gap: >-
      The docs do state it: "Hook entries merge across settings levels rather than replacing each
      other", plus `disableAllHooks` to turn all hooks off and `allowManagedHooksOnly` for
      enterprise restriction. That bounds the whole design's guarantee — in a consumer repo the
      gate is one setting away from gone — and the record nowhere states what enforcement means
      under it.
    fix: >-
      Correct F8. Add one line to D7(c) or D9: the gate is a floor for consumers who keep the
      plugin's hooks enabled; a project that disables hooks keeps only the procedural
      author-not-grader ceremony, and that is a ratified consequence, not a defect.
    disposition: repair (F8 fact + D7/D9 scope line)

  - id: I10
    sev: Important
    class: inconsistency (map angle 45)
    at: "D5 (setup homes include `.claude/rules/mochiko/*.md` and 'the CLAUDE.md governance region's file') vs D9 rationale ('a plugin overreach the consuming project never ratified')"
    gap: >-
      Declaring those files as homes puts the gate on the consumer's own Claude Code
      configuration: every edit to CLAUDE.md, including edits with nothing to do with mochiko,
      hits a plugin-owned deny with a size budget attached. D9's own reasoning rejects exactly
      this reach for `docs/`.
    fix: >-
      In D5, scope the gate over `.claude/rules/mochiko/*.md` to location and file-name only, and
      exclude CLAUDE.md from the write-time gate entirely (the governance region has its own
      ceremony through `/mochiko:setup`). Keep them in the registry for the reminder channel if
      wanted.
    disposition: repair (D5 scope)

  - id: I11
    sev: Important
    class: passive acceptance (map angle on streaks; record's own disclosure)
    at: "D1, D2, D4, D6 confidence lines ('as recommended'; adoption streak 1-4); Q3-Q6"
    gap: >-
      Four consecutive recommended options adopted, no fork declined, no alternative chosen by the
      user, and the one refinement the user contributed (section-level limits) is the one whose
      numbers do not exist yet. The record discloses the streak and then treats each ruling as
      Confident on the strength of the adoption. A streak is a calibration risk, not evidence.
    fix: >-
      Put the strongest rejected road back to the user as a real fork with the cost of each side
      named — C1's declare-and-measure wave against D1's deny — and re-mark the affected
      confidences from the answer rather than from the adoption.
    disposition: ruling (lead re-puts C1's fork; confidences re-marked on the answer)

  - id: I12
    sev: Important
    class: missing intra-decision dimension (map angles 23, 52)
    at: "D8 probe legs (transports, Read/Glob/Grep field names, per-call cost)"
    gap: >-
      The probe covers who the hook fires for and omits what would make the gate real or dead:
      whether a deny still blocks under `bypassPermissions`, `acceptEdits`, and `auto` (the docs
      are silent, and `permission_mode` is on the hook input F7 already lists); whether a Bash
      write is seen at all (C5); aggregate per-run cost (I3); and the behavior when the wrapper is
      unexecutable or `${CLAUDE_PLUGIN_ROOT}` fails to resolve, which the platform treats as a
      non-blocking error that silently proceeds.
    fix: >-
      Add those four legs to D8, each with its own recorded result, and make the permission-mode
      leg part of the abort logic (a gate inert in the modes the runs actually use is the same
      failure as a gate inert for subagents).
    disposition: fold (D8 probe legs)

  - id: M1
    sev: Minor
    class: missing dimension
    at: "D1(a) ('whose target resolves under a declared artifact home')"
    gap: A Glob or Grep carries a pattern and a search root, not a target path; nothing defines resolution for a pattern matching many homes, one home, or none.
    fix: State the rule (fire when the search root or pattern prefix lies under a home; never attempt per-match resolution), or drop Glob/Grep from the matcher and keep Read.
    disposition: repair (D1a)

  - id: M2
    sev: Minor
    class: excess machinery
    at: "D1(a) reminder, D9 (hooks hold no state)"
    gap: The reminder fires on every read by every seat forever, including reviewers and the lead whose reads are not shape-shopping; stateless means no first-fire-only suppression, so identical context lines accumulate through a run.
    fix: Accept the repetition explicitly, or scope the reminder to the seats that author (the producing skills' own rendered rules already reach them) and measure line count per run in the D10 watch.
    disposition: fold (D1a)

  - id: M3
    sev: Minor
    class: unverifiable claim
    at: "F8 (`permissionDecision` `allow|deny|ask|defer`; PostToolUse `additionalContext` unverified)"
    gap: "`defer` appears nowhere in the hooks documentation; the doc page does not enumerate the value set at all. Not load-bearing (the design uses allow and deny only), but F8 is the block D8 is told to trust."
    fix: Trim F8 to the values the shipped script already proves work (`deny` via `hookSpecificOutput`, exit 0) and mark the rest unverified, or verify against the reference page before D8.
    disposition: repair (F8)

  - id: M4
    sev: Minor
    class: missing dimension
    at: "D4(c) (placeholder tokens absent, '`XXX` inside an id')"
    gap: Legitimate prose in these artifacts names `FEAT-XXX`, `EPIC-XXX`, and `AX-XXX` as patterns — this record does so repeatedly, and so do the delivered rules. An unscoped token check denies honest text.
    fix: Scope the check to frontmatter values and heading text, or require the exact template token spelling rather than a substring.
    disposition: repair (D4c)

  - id: M5
    sev: Minor
    class: inconsistency
    at: "Review block ('one review seat (`mochiko:devils-advocate` on `mochiko:review-brainstorm`, session tier)')"
    gap: The seat as dispatched is a session-tier teammate carrying `mochiko:review-brainstorm` without the devils-advocate persona; the disclosure names a composition that does not match the dispatch.
    fix: Record the composition as dispatched, or seat the persona for the verify pass if the persona was intended.
    disposition: repair (Review block)

  - id: M6
    sev: Minor
    class: missing dimension
    at: "D1(b) matcher (`Write|Edit`)"
    gap: Other write-shaped tools are unaddressed — NotebookEdit, and any MCP-provided file writer a consuming project has enabled. The matcher set is stated as a fact rather than as a decision with a boundary.
    fix: State the matcher set as deliberate and name what is knowingly outside it, alongside C5's Bash ruling.
    disposition: fold (D1b)

  - id: M7
    sev: Minor
    class: missing dimension
    at: "OQ4 ('the abort line for cost … is the lead's to propose at wave 0')"
    gap: The stated framing — a check slower than the 5-second timeout is fail-open and enforces nothing — is true and the wrong bound; the damaging case is a fast check firing constantly.
    fix: Re-state OQ4 as a per-run aggregate budget (see I3), with the timeout as a separate correctness floor.
    disposition: repair (OQ4)

  - id: M8
    sev: Minor
    class: inconsistency
    at: "D2 (reports open-by-name) vs `impl.reports-envelope` (an enumerated report set)"
    gap: >-
      The delivered rule already names its reports — sufficiency, cycle, verification,
      final-validation, built-vs-signed diff — while D2 makes report names free inside `reports/`.
      Two policies for the same files; also D2 re-keys that rule's path, so the rule's own text
      changes in wave 4 and the enum in `templates/report-format.md` becomes the binding surface.
    fix: Reconcile in D2: report NAMES are free, report `report:` TYPES come from the format enum, and note the `impl.reports-envelope` re-key as the text change it is (D11 wave 4 already carries the re-point).
    disposition: fold (D2)
```

## Status

**Recommended status: `critical-gaps`** — five Critical findings, of which C1 reopens the build
order, C5 reopens the matcher set, and C2/C3/C4 are mechanical defects that would break a
consumer's session, block a live repo's edits, or instruct the user to use a surface that does
not exist. Three load-bearing figures are wrong (I8) and one platform fact is wrong in the
direction that bounds the design's guarantee (I9).

Counts: **30 raised, 25 survived** — Critical 5 · Important 12 · Minor 8.

The verdict is input. The clearing decision, and every fold into `record.md`, is the lead's.

## Verify pass

Round 1 verify, 2026-09-13. Scope: internal consistency and record-fitness of the 25 folds in
`record.md` (634 lines) against the disposition table. No fresh cold read, no new blind map, no
new angles. Every disposition was located at the row's stated home; the folds are substantive,
not cosmetic, and three of the four heaviest repairs (C2 exit-code contract, C3 first-touch
amnesty, I4 hook-payload interface) are written as designs a builder could execute.

Fold-introduced facts re-checked independently, both CONFIRMED on
`code.claude.com/docs/en/hooks-guide`: "`PreToolUse` hooks fire before any permission-mode
check, in every permission mode, including `dontAsk`. A hook that returns
`permissionDecision: "deny"` blocks the tool even in `bypassPermissions` mode or with
`--dangerously-skip-permissions`" (line 967 of the fetched page), and `defer` as a fourth
`permissionDecision` value in non-interactive `-p` mode (line 641). So F8's two new claims and
M3's verification hold, and D9's substitute mechanism has its ground. `FileChanged` is also real
("To reformat a specific file however it changes, including when a `Bash` command rewrites it,
use a `FileChanged` hook instead"), with one fact the record does not carry: its `matcher` is
split into **literal filenames, not a regex or glob**, so it cannot watch a tree.

Corrected figures verified: F13's 288 files / 115,757 lines is right, and the per-dir splits plus
the 4-file / 942-line `archive/` remainder reconcile to the totals. F10's HEAD figures (FEAT-001
1,003 · FEAT-002 1,731) are right, and the card-writing-time qualifier on the superseded numbers
is the honest fix.

```yaml
residuals:
  - id: V1
    sev: Critical
    at: "D1(a) statement + D1 rationale ('(a) closes the second class') vs D4(f)"
    gap: >-
      Channel (a) delivers the conformance block "as part of the `mochiko-cli template <kind>`
      producer view", but D4(f) states ~22 of ~30 kinds have no template in the log. So (a)
      reaches only the eight templated kinds and delivers nothing for the class the rationale
      says it closes — and that claim is load-bearing for R1, where the declare-and-measure road
      was declined partly because "channel (a) ships beside the gate at no extra cost".
    fix: >-
      Extend D1(a): for a kind with no template, the producing command's or skill's rendered
      rules carry the home, file set, and whole-file bound from `mochiko-cli home <path>` (the
      subcommand D3 already mints), so every kind has an authoring-time channel. Alternatively
      state in the rationale that (a) covers the templated eight today and sequence templates for
      the rest in D11 — but then R1's basis should be re-stated to the user, since it rests on
      (a) covering the untemplated class.
  - id: V2
    sev: Important
    at: "D4(f) ('location + set + whole-file size checks only') vs D5 (root operating docs, 'location only')"
    gap: >-
      The root operating docs have no template, so D4(f) applies a whole-file size check to them
      while D5 scopes them to location only and sends their bounds to the KM invariants per
      GI-017. The two decisions disagree on the same files.
    fix: >-
      Add the exemption to D4(f): a kind whose bounds live in another named constraint home takes
      location + set only, with the home cited. Name the operating docs as that case.
  - id: V3
    sev: Important
    at: "D4(e) first-touch amnesty (scoped to `Edit`)"
    gap: >-
      The amnesty compares pre- and post-edit state for `Edit` only. A `Write` that overwrites an
      existing violating file — the ordinary way a 1,731-line `tasks.md` gets split or rewritten —
      is judged against the budget outright and denied while still over, so the rewrite the
      amnesty exists to allow is blocked when performed by `Write`.
    fix: >-
      State that the non-worsening comparison applies to any write over an existing file, `Write`
      and `Edit` alike (for `Write` the on-disk file is the baseline and `tool_input.content` the
      result), and that a new file at a declared name takes the budget outright.
  - id: V4
    sev: Important
    at: "D2 deny text ('Put it under this home's `reports/` (any name; report envelope required)')"
    gap: >-
      The escape route is only open to content that can honestly carry a `report:` value from
      `report-format.md`'s enum (cycle · verification · final-validation · review · feasibility ·
      disclosure). A non-report deliverable moved to `reports/` is then denied by the envelope
      check, so the deny routes to a second deny.
    fix: >-
      Either name `disclosure` as the catch-all type in the deny text, or drop the `reports/`
      clause for non-report content and let the deny end at the upstream ask, which R2b already
      accepts as the only route.
  - id: V5
    sev: Important
    at: "D11 wave 5 precondition (violator pass) vs D1(c) Bash deny + wave ordering"
    gap: >-
      The violator pass splits and re-homes kinako's live files, and it runs at wave 5 — after
      wave 4 ships the hooks. D1(c) denies a `Bash` command whose text carries a `cp`/`mv`
      destination under a declared home, so the re-homing act is denied by the gate it is a
      precondition for, and the split's `Write`s hit V3.
    fix: >-
      Move the violator pass to wave 3 or 4 (before the registrations land), or state the
      exemption: a re-home whose destination is a declared name in a declared home is allowed.
  - id: V6
    sev: Minor
    at: "F8 (the composite Bash/`Stop`/`FileChanged` quote)"
    gap: >-
      The paragraph is rendered as one verbatim quote with ellipses but names no page. Its sense
      is confirmed on the hooks guide; the clause "For per-call coverage instead, also match
      `Bash|PowerShell`" could not be located on either the guide or the reference page pulled at
      review.
    fix: Pin each quoted sentence to its page and section anchor, or mark the unlocated clause as the seat's paraphrase.
  - id: V7
    sev: Minor
    at: "F8 (`FileChanged` named) vs D1 rejected roads (Stop hook, PostToolUse advisory dispositioned; FileChanged not)"
    gap: >-
      F8 now names a hook that fires when a file changes on disk whatever wrote it — the closest
      platform fit for location enforcement — and no decision dispositions it.
    fix: >-
      Add `FileChanged` to D1's rejected roads with both reasons: post-write (the driver excludes
      it) and a `matcher` of literal filenames rather than patterns, so it cannot watch a tree of
      homes.
  - id: V8
    sev: Minor
    at: "D5 ('F13: 284 files'); D4 rationale ('FEAT-002 `tasks.md` 1175 → 1452')"
    gap: The I8 correction did not propagate: D5 still cites the superseded file count, and D4's rationale cites the card-writing-time line counts without F10's qualifier.
    fix: "D5: 288 files. D4 rationale: cite the append drift with the qualifier F10 now carries, or cite the HEAD figures."
  - id: V9
    sev: Minor
    at: "D9 title ('a two-strike halt instruction in every deny') and rationale ('the halt instruction routes it to the user at a bounded cost')"
    gap: The statement now marks that sentence advisory (C5), while the title and rationale still present it as the routing mechanism.
    fix: Re-title to name the owned mechanisms (the Bash-matched deny and the permission-mode floor), and reword the rationale to say the halt is advisory and the reviewer is the backstop.
  - id: V10
    sev: Minor
    at: "D8 title ('the reminder channel's `Read`/`Glob`/`Grep` field names verified there too')"
    gap: M1 dropped Glob and Grep from the reminder; the statement now names `Read` only.
    fix: Title says `Read`.
  - id: V11
    sev: Minor
    at: "D11 wave 3 ('the brainstorm review home declared')"
    gap: Reads against I5's ruling that there is no separate review home — brainstorm reviews are reports under the home's `reports/`.
    fix: "Wave 3 declares the brainstorm home's `reports/` dir and its report types."
  - id: V12
    sev: Minor
    at: "D10 contract cases; D8 aggregate-cost leg"
    gap: >-
      The crate matrix carries the Bash write-operator parse but the contract suite has no
      `Bash`-leg case, and the cost leg does not name Bash-call volume even though the `Bash`
      matcher fires on every shell command in a run, not only on writes.
    fix: Add a contract case for a denied Bash write and an allowed ordinary Bash command; name Bash-call volume in D8's aggregate measure.
  - id: V13
    sev: Minor
    at: "D6 confidence ('adoption streak 4')"
    gap: I11's repair re-marked D1 only; D6 still rests its confidence on the streak I11 named as not-evidence, though its section-level refinement is genuinely the user's.
    fix: Re-mark from the answer (the user's own wording carries D6), or state that the streak stands as the warrant knowingly.
  - id: V14
    sev: Minor
    at: "Header line 3-4 ('**Review:** pending (cold, blind-map two-message dispatch)')"
    gap: Stale — round 1 is folded and the verify pass is in progress; the same line's Status field already says so.
    fix: "Review: round 1 folded, verify in progress."
```

**Status: NOT CLEAN — 14 residuals** (Critical 1 · Important 4 · Minor 9).

V1 is the one that changes a user-facing claim: it is the fold that compensates for the declined
C1 road, and as written it does not reach the class the rationale assigns it. V2 through V5 are
cross-decision disagreements that would surface as build defects. The nine Minors are text that
did not follow its own fold.

The verdict stays input. Repairs are the lead's pen.

## Delta-check

Bounded to the fourteen verify repairs in `record.md` (676 lines), each read with its immediate
context. No new angles.

- **V1 — not held.** D1(a) and the rationale say exactly what the fix asked (template-less kinds
  get home, set, and whole-file bound from `mochiko-cli home <path>`), but no wave lands that arm:
  wave 1 builds only "the conformance block in the template producer view", wave 3 mints the
  `home` documents, and wave 4's primitive edits are scoped to "the `Not-done` and `Tools` lines
  … re-pointed where a home changed". The authoring-time rule that makes a producing command or
  skill deliver the home is neither a `home` document nor a re-point. **Repair:** name it in wave
  3 (the migration mints that rule into each producing primitive's rule set) and add its render
  golden to D10 beside the producer-view one.
- **V2 — not held.** The D4(f) exemption is in and names the operating docs, but D6 still reads
  "A kind with no template yet (D4f) takes a single whole-file bound as its placeholder", with no
  pointer to the exemption — a builder implementing size checks from D6 alone still applies a
  whole-file bound to `BACKLOG.md`. **Repair:** one clause in D6 citing D4's exemption.
- **V3 — held.** Amnesty now covers any write over an existing file, `Write` and `Edit` alike,
  with the baseline and result defined per tool and a new file taking the budget outright; D6 and
  D11 wave 5 both cite it consistently.
- **V4 — held.** The deny text routes only enum-typed report content to `reports/` and sends
  everything else upstream, and the later `reports/` clause agrees.
- **V5 — held.** Wave 5 is three ordered steps with the violator pass ahead of the consumer's
  plugin upgrade, and the later-re-home case is routed to D4(e) amnesty with the `Bash` `mv` deny
  named as by-design.
- **V6 — not held.** The pin is right and the hedge is wrong: the full paragraph, including the
  clause the record now carries as "not re-located by the verify seat", is verbatim on
  `code.claude.com/docs/en/hooks-guide` in the "Block edits to protected files" section — "For
  per-call coverage instead, also match `Bash|PowerShell` and have your script list modified and
  untracked files with `git status --porcelain`." Its next sentence is the substantive item: "The
  PowerShell hook input section explains why matching `Bash` alone is not enough." D1(c) matches
  `Write|Edit|Bash`, so on Windows the write-time gate has the hole C5 was raised to close.
  **Repair:** restore the clause as a pinned quote, and either extend the matcher to
  `Write|Edit|Bash|PowerShell` or record the Windows carve explicitly against GI-020's supported
  set (Windows with Git Bash is supported; PowerShell-only is not, which does not by itself
  remove the PowerShell tool from a supported session).
- **V7 — held.** `FileChanged` sits in D1's rejected roads with both reasons, post-write by
  construction and a literal-filename matcher.
- **V8 — held.** D5 reads 288 files, and D4's rationale now cites the HEAD figure with F10's
  card-writing-time qualifier.
- **V9 — held.** D9 is re-titled to the owned mechanisms and its rationale says the halt sentence
  is advisory, naming the `Bash` deny, the permission-mode floor, and the reviewer as the
  backstops.
- **V10 — held.** D8's title names the `Read` field name, the Bash leg, permission modes,
  dead-gate behavior, and per-run cost.
- **V11 — held.** Wave 3 declares the brainstorm home's `reports/` dir and its report types, with
  "no separate review home" stated.
- **V12 — held.** D10 gains a denied `Bash` heredoc write and an allowed ordinary `Bash` command,
  and D8's cost leg counts every `Bash` call with the reason stated.
- **V13 — held.** D6's confidence is marked from the answer, not the adoption.
- **V14 — held.** The header records round 1 folded, verify round 1 NOT CLEAN with 14 repaired,
  delta-check pending.

**Status: NOT CLEAN — 3 residuals** (V1, V2, V6).

V6 carries the only new substance: the docs sentence the record now quotes is followed by one
saying a `Bash` match alone is not enough, which leaves the Windows half of the C5 hole open in
D1(c). V1 is a sequencing gap in an otherwise correct repair, and V2 is a single missing clause.

The verdict stays input. Repairs are the lead's pen.
