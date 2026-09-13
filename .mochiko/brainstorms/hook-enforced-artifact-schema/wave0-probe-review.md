---
report: review
feature: hook-enforced-artifact-schema
round: 0
verdict: PASS
verdict_history:
  round_0: FAIL — 15 findings, 1 Critical, 5 Important, 9 Minor
  round_1: PASS — all 15 held after the QA seat's fix pass; see the `## Re-grade` section at the end of this file
note_on_item_grades: >
  The `item_grades` and `findings` blocks below are the round-0 record and are left unrevised; the
  per-finding round-1 dispositions live in the `## Re-grade` section.
abort_reading_safe_to_rule: yes
graded_artifact: wave0-probe-report.md
graded_against: record.md D8 (legs + abort criterion) + D1 + D9 + OQ3 + OQ4; wave0-probe-plan.md
reviewer_role: standing non-author reviewer; authored nothing in this build
platform_reverified: Claude Code 2.1.258; --permission-mode choices acceptEdits, auto, bypassPermissions, manual, dontAsk, plan (no `default`) — both report claims hold
independent_work_done: >
  Opened the hook log, 14 per-fire payload files, 6 run summaries, 2 subagent sidechain sets,
  4 run streams, the 8 probe hooks.json files and the probe wrapper; re-ran the field() extraction
  against the real false-allow payload; recounted both cost sessions' tool calls from the
  transcripts and sidechains; re-timed the per-call proxy; re-fetched both hook doc pages and
  verified 7 quotes verbatim.

item_grades:
  - id: 1
    name: abort reading
    grade: PASS
    verdict: The abort criterion is genuinely not tripped; every limb is carried by evidence I opened myself.
    evidence_checked: >
      fires/1789279954-17190-broad-PreToolUse-Write.json — PreToolUse inside an Agent subagent,
      agent_id a6673d77affd5ad8a, agent_type general-purpose, permission_mode dontAsk,
      tool_input.file_path = <cwd>/probe-home/a.md. probe-gate.log SID=9669f09e shows
      DECISION=deny for that fire. runs/r02-t2-subagent-dontask/summary.txt: probe-home after run
      holds seed.md only, and run.sh deletes every non-seed file before each run, so the listing is
      ground truth. Sidechain
      ~/.claude/projects/<probe-cwd slug>/9669f09e-.../subagents/agent-a6673d77affd5ad8a.jsonl line 7:
      isSidechain true, tool_result is_error true carrying the deny text, toolDenialKind
      "permission-rule". acceptEdits: deny on Write and Bash for T1 (SID=5941eb56) and T2
      (SID=1d598f18, agent_id a0f6ffbc8cde66e1e); bypassPermissions: deny on Write and Bash for T1
      (SID=2456fdb6) and T2 (SID=02265a7e, agent_id a6f96e4fec2ab7f6f); PMODE field present on every
      one of those lines; all four home listings hold seed.md only.
  - id: 2
    name: plan conformance
    grade: FAIL
    verdict: Cell accounting misattributes the one unresolved cell and never states whether every planned cell resolved; D8's `Edit` limb was dropped without disclosure.
    findings: [G1, G2, G3]
  - id: 3
    name: leg 8 evidence
    grade: PASS
    verdict: The attachment proof is real in both sidechains, and the non-consumption claim is evidenced by a two-sentinel controlled test — but the report never discloses the discriminator.
    evidence_checked: >
      Both sidechains under 9a06.../d70ef3e2-.../subagents/ (agent-a2a3cf72c66e578e5.jsonl,
      agent-a2e307af39787f03f.jsonl) carry at line 1 an attachment
      {"type":"hook_additional_context","content":["SENTINEL-SUBAGENT-START probe context injected at
      SubagentStart"]}. probe-subagent/hooks/scripts/probe-gate.sh emits TWO distinct sentinels in one
      output: hookSpecificOutput.additionalContext = "SENTINEL-SUBAGENT-START ...", top-level
      additionalContext = "SENTINEL-SUBAGENT-START-TOPLEVEL ...". Both strings appear in the same
      sidechains inside a separate {"type":"hook_success"} attachment recording the hook's raw stdout;
      only the hookSpecificOutput sentinel appears in the hook_additional_context content array. So
      non-consumption is measured, not inferred.
    findings: [G14]
  - id: 4
    name: cost model honesty
    grade: FAIL
    verdict: The transcript substitution is disclosed everywhere, and the 60 s / 100 ms triggers do derive from the stated figures — but two stated counts do not reproduce, one of them flipping the report's own narrowing recommendation.
    reproduced_exactly: >
      613c3001 Read 71 / Write 7 / Edit 2 / Bash 484 / gated 564 / all_tool_use 645, 8 sidechains,
      13.7 MB; 23e1aa9b Read 73 / Write 4 / Edit 0 / Bash 443 / gated 520 / all_tool_use 559,
      16 sidechains; 613c3001 narrowed_total 225 (Read 21 + Write_Edit 4 + Bash mentioning .mochiko 200);
      Bash share 85.8% and 85.2%; all four projection_seconds figures at the 47.1 ms median.
    findings: [G4, G5, G6, G7, G15]
  - id: 5
    name: findings that bind later waves
    grade: FAIL
    verdict: Five of six carry real pointers and the field() false-allow is the strongest evidence in the report; the `Task` recommendation is unverified and locally contradicted.
    per_finding:
      field_false_allow: >
        VERIFIED, and it is a real created file. runs/r0-retry-2/summary.txt lists probe-home holding
        c.md and seed.md; probe-gate.log SID=f4b92293 TS=1789280577 records DECISION=allow/no-home-hit
        for that Bash fire; the payload's command is
        `printf 'retry probe' > "<abs>/probe-home/c.md"`. I re-ran the field() pipeline from
        plugins/mochiko/hooks/scripts/dependency-halt.sh against that payload: it extracts
        `printf 'retry probe' > \` — the path is gone. The shipped helper's regex is verbatim
        "\"$1\"[[:space:]]*:[[:space:]]*\"[^\"]*\"" as the report describes.
      bash_share: VERIFIED for the shares (86%/85%); see G5 for the write-operator counts.
      if_placement_trap: >
        VERIFIED by fire counts — r0-if 7 narrow fires including three DECISION=...hits-no
        (out-of-home calls the `if` should have excluded), r0-if2 3 fires all hits-yes. See G12.
      task_wire_name: >
        PARTIAL. runs/r0-retryfix-1/stream.jsonl system:init lists tools starting ["Task","Bash",...]
        and includes NotebookEdit, so both claims about the advertised set hold. But the same stream
        records the spawn tool_use as "name":"Agent" 10 times and 0 times as "Task", and no hook fire
        on a spawn exists anywhere in the probe (the matcher never included one). See G8.
      prompt_injection_reading: >
        VERIFIED verbatim in runs/r0-retryfix-1/report.txt; the "instruction-shaped" second instance
        is in runs/r0-substart/report.txt and its stream. Neither pointer appears in the report — G9.
      delegation_escalation: >
        VERIFIED. probe-gate.log SID=1b690108 shows two distinct delegated agent_ids
        (ad9f4789249e47c71, a7a48fc855f31e905) denied on Write and Bash; SID=8b9628e3 shows
        ac54ea336ef8cfbf6 denied on Write, Bash and Edit. Both runs' home listings hold seed.md only.
        The narrated escalation ORDER does not match either run — G10.
    findings: [G8, G9, G10, G11, G12]
  - id: 6
    name: citations
    grade: PASS
    verdict: Seven quotes verified verbatim against the live pages, the doc defect is real, and unverified items are marked as such; one citation is abridged in a way that hides the doc surface bearing on leg 7.
    verified_verbatim:
      - "hooks reference, Hook locations: subagents sentence, including the clause that the input carries agent_id and agent_type"
      - "hooks reference, Common fields: the `if` row"
      - "hooks reference, exit-code text: 'a mistyped path in `settings.json` leaves the gate silently disabled.'"
      - "hooks reference, Common input fields: the agent_id and agent_type rows, including 'For subagents, the subagent's type takes precedence over the session's --agent value' — which explains the report's T4 field table"
      - "hooks-guide: 'A hook that returns `permissionDecision: \"deny\"` blocks the tool even in `bypassPermissions` mode or with `--dangerously-skip-permissions`.' The same sentence adds that PreToolUse fires before any permission-mode check, in every mode including dontAsk."
      - "hooks-guide, Block edits to protected files: the `Bash|PowerShell` plus `git status --porcelain` remedy, and the PowerShell-hook-input link"
      - "hooks-guide, Limitations: 'Background subagents can't show a prompt in non-interactive mode. Claude Code still runs the hooks for their tool calls, and if no hook returns a decision, it denies the call.' — this is what makes the plan's explicit-allow design load-bearing, and it holds."
    doc_defect_confirmed: >
      The hooks reference has no #powershell anchor and no PowerShell hook-input section (only a
      Windows (PowerShell) tab under How a hook resolves), while the guide links
      /docs/en/hooks#powershell. The report's reading is accurate. The reference also gives no
      tool_input schema for Read, so leg 2's "undocumented" is accurate.
    findings: [G13]

findings:
  - {id: G4, type: measurement, sev: Critical,
     at: "wave0-probe-report.md cost.sessions_measured[23e1aa9b].under_mochiko.Bash_mentioning = 287 (and narrowed_total 321, projection_seconds.23e1aa9b.if_narrowed 15.1, the '38 percent' line in the budget proposal)",
     gap: "287 does not reproduce. The predicate that yields the report's own 200 for 613c3001 is `.mochiko` appearing in the command text — my recount hits 200 exactly. The same predicate yields 154 for 23e1aa9b, so narrowed_total is 188, the if_narrowed projection is 8.9 s, and the narrowing removes 64 percent of gated calls, not 38. That inverts the report's headline reading that the narrowing 'is not the lever it looked like' and weakens the case for promoting leg 8's SubagentStart channel over it.",
     fix: "State the predicate in one line, recount both sessions under it, correct Bash_mentioning / narrowed_total / if_narrowed / measured_baseline_seconds, and re-derive the narrowing verdict and the SubagentStart recommendation from the corrected pair (60 percent and 64 percent)."}
  - {id: G5, type: measurement, sev: Important,
     at: "cost.sessions_measured[*].bash_with_write_operator = 206 and 211; the Notes-of-note line '206 and 211 of those Bash calls carry a write operator'; cost.enumeration_dispatch's claim that the Bash write-operator split was re-run and matched",
     gap: "Neither figure reproduces. Counting Bash calls whose command carries any of > >> tee `sed -i` cp mv gives 169 and 166; adding heredoc `<<`, dd and touch gives 180 and 180; nothing I tried reaches 206 or 211. Since enumeration_dispatch states the author re-ran this split and it matched, the disclosed self-verification does not hold for the numbers it names.",
     fix: "Publish the operator set as a literal list, recount, correct both figures, and either re-state or withdraw the enumeration_dispatch sentence for this split. The qualitative claim (shell is the dominant write path) survives at 169-180 and needs no softening."}
  - {id: G6, type: derivation, sev: Important,
     at: "proposed_budget.per_run_share_of_wall_clock_max_pct = 2, and the Budget-proposal paragraph",
     gap: "No wall-clock figure for any measured run appears anywhere in the report, so the second budget limb has no denominator and cannot be checked. The transcripts give it: 613c3001 spans 5.9 h (21218 s) and 23e1aa9b 1.2 h (4461 s), so 2 percent is 424 s and 89 s respectively — both far above the 60 s aggregate cap, meaning the cap binds and the percentage limb never fires on either session.",
     fix: "State both spans, name which limb binds, and either keep the 2 percent as an explicit backstop for short runs or drop it. Also state which wall-clock measure a future run uses (transcript span vs the stream result's duration_ms — the probe's own 21 runs ran 10 s to 136 s, median 41 s, where a 26.6 s hook cost would be 65 percent of wall clock)."}
  - {id: G1, type: conformance, sev: Important,
     at: "frontmatter cells_planned 32 / cells_resolved 31 / cells_unresolvable 1 (# the PowerShell arm)",
     gap: "The plan's 32 cells are legs 1-7 (12+1+4+8+2+1+4) and none of them is a PowerShell cell. Leg 9 is an unplanned addition, like leg 8, so attributing the single unresolved cell to it leaves the reader unable to tell whether every planned cell resolved. On my read all 32 did resolve, one of them by substitution (G3).",
     fix: "Report 32/32 planned cells resolved with the substitutions named, and account for legs 8 and 9 separately as additions outside the plan — leg 9 marked lead-added the way leg 8 already is."}
  - {id: G2, type: conformance, sev: Important,
     at: "leg 1 (transport coverage) result PASS, cells 12/12, graded against record.md D8",
     gap: "D8 asks whether the hooks fire on Write/Edit/Read for each transport. The plan's leg 1 replaced Edit with the Bash deny channel, and neither the plan nor the report discloses the substitution. Edit was exercised 3 times in the whole probe: one DECISION=deny (r0-retryfix-2, on a plugin-typed delegated subagent) and two allow/no-home-hit. No Edit fire exists for T1, T2, T3 or T4 as a leg 1 cell.",
     fix: "Disclose that D8's Edit limb was not run per transport, cite the single Edit deny (probe-gate.log SID=8b9628e3, agent_id ac54ea336ef8cfbf6) as the whole of the Edit evidence, and either add a one-run Edit cell in wave 1 or record the omission as a disclosed hole with its reason (Edit shares tool_input.file_path and the same matcher arm as Write)."}
  - {id: G8, type: unverified-claim, sev: Important,
     at: "Notes of note, 'The spawn tool's wire name is `Task`, not `Agent`. ... Any future matcher on a spawn, including the `SubagentStart` alternative, should be written against that name.'",
     gap: "Half-measured, and the report does not disclose the conflict. The init event does advertise Task, but the same stream records every spawn tool_use as name Agent (10 occurrences, 0 for Task), and no hook ever fired on a spawn call in this probe because the matcher set excluded it. A matcher written on the advertised name alone could silently never fire — the exact failure class leg 5 shows is invisible.",
     fix: "Mark the recommendation UNVERIFIED, cite the runs/r0-retryfix-1/stream.jsonl init tools array and the 'name':'Agent' tool_use records side by side, and add a one-cell wave-1 probe that registers a PreToolUse matcher on Task and on Agent and reports which fires before any spawn matcher ships."}
  - {id: G11, type: evidence-pointer, sev: Minor,
     at: "leg 1 evidence 'each seat's own verbatim report at probe-cwd/out/report.txt'; the same path implied in leg 3",
     gap: "run.sh clears probe-cwd/out before every run, so that path now holds only the last run's files (outofhome.md, outside.md, z.txt — no report.txt at all). runs/r01..r04 preserved no report.txt, so the cited artifact for the abort leg does not exist. The surviving proof is each run's stream.jsonl, which does carry the sentinels (SENTINEL-DENY-WRITE, SENTINEL-DENY-BASH and SENTINEL-REMINDER all present in all four).",
     fix: "Repoint leg 1 and leg 3 at runs/<id>/stream.jsonl (and the r02/r03 sidechains for the subagent view), and note that out/ is reset per run so seat reports survive only for the runs that copied them."}
  - {id: G12, type: evidence-pointer, sev: Minor,
     at: "leg 7 'Placed on the matcher-group object it is silently ignored', evidence run r0-if",
     gap: "probe-if/hooks/hooks.json now holds the corrected handler-level `if`; the group-placement config that produced r0-if was overwritten, so the trap is reconstructible only from the log's three hits-no fires.",
     fix: "Transcribe the group-placement JSON into the report (four lines) so wave 4 can regression-test the trap without re-deriving it."}
  - {id: G13, type: citation, sev: Minor,
     at: "leg 7 citation of hooks reference, Common fields, the `if` row",
     gap: "The quote is abridged. The live row also says 'See the Bash matching table (#bash-if-matching) below for how Bash patterns evaluate against subcommands, $(), and backticks.' That table is the doc surface bearing directly on leg 7's finding that Bash(*>*) does not fire while Bash(*probe-home*) does, and the report neither cites nor reconciles it. I read it: it covers assignment stripping, per-subcommand checks, $() and backticks, and is silent on redirect and pipe operators — so the finding survives as the measured answer to an undocumented question.",
     fix: "Quote the full row, cite the Bash-if-matching table, and state that the table's rows do not explain the Bash(*>*) result, so the redirect-shaped narrowing is undocumented rather than contradicted."}
  - {id: G14, type: disclosure, sev: Minor,
     at: "leg 8 detail 'a top-level additionalContext in the same output was NOT consumed', evidence line",
     gap: "The evidence quoted is only the consumed form, so a reader cannot see this was a controlled two-sentinel test. The discriminator exists in the probe script (SENTINEL-SUBAGENT-START vs SENTINEL-SUBAGENT-START-TOPLEVEL) and both strings survive in each sidechain's hook_success stdout record while only one reaches the hook_additional_context attachment.",
     fix: "Name both sentinels and cite the hook_success attachment alongside the hook_additional_context one, in both sidechains."}
  - {id: G9, type: evidence-pointer, sev: Minor,
     at: "Notes of note, the prompt-injection quote and the 'instruction-shaped' second instance",
     gap: "Both are real and verbatim, but neither carries a run id. They are the two Notes-of-note items that change a D1b wording decision, so they are the ones a later wave will need to re-read.",
     fix: "Cite runs/r0-retryfix-1/report.txt and runs/r0-substart/report.txt."}
  - {id: G10, type: accuracy, sev: Minor,
     at: "leg 3 detail 'both seats escalated Write, then a shell redirect, then Edit, then delegation to a fresh subagent'",
     gap: "Neither run's fire order matches. r0-retryfix-1 has no Edit deny at all; r0-retryfix-2's Edit deny came from the delegated subagent, after delegation, not before it. The material claim — every path denied, no file created — holds in both.",
     fix: "Replace the single narrated order with each run's fire sequence from the log, or state the escalation set without an order."}
  - {id: G7, type: measurement, sev: Minor,
     at: "cost.wrapper_only_ms {median: 34.6}",
     gap: "It cannot coexist with cli_direct_ms 40.9 and thin_wrapper_plus_cli_ms 47.1 under any additive model. My re-timing through the same harness puts the spawn floor at 2.0 ms (/usr/bin/true) and 4.1 ms (sh -c true), with the CLI at 45.4 ms and thin.sh at 45.1 ms — so a wrapper without the CLI costs single-digit ms, not 34.6. The figure feeds nothing (projections use 47.1), which is why this is Minor.",
     fix: "Re-measure or drop wrapper_only_ms, and state the spawn floor measured the same way so the reader can see how much of the 47.1 ms is harness overhead rather than hook cost."}
  - {id: G3, type: conformance, sev: Minor,
     at: "leg 3 result 'retry behaviour measured, n=4'; the plan's leg 3 cells {T1, T2} x {heredoc, retry}",
     gap: "The heredoc arm is resolved on all four transports (I confirmed a heredoc payload with newlines intact for T1, T2, T3 and T4), but all four retry runs were lead-seat: prompts/p-retry.txt drives the lead, and the T2 retry cell is covered only indirectly, by the delegated subagent denied inside r0-retryfix-1 and r0-retryfix-2.",
     fix: "Attribute the retry runs by transport and state the T2 substitution explicitly."}
  - {id: G15, type: accuracy, sev: Minor,
     at: "frontmatter spend_usd 2.78; cost.sessions_measured[23e1aa9b].what '9.3 MB aggregate'",
     gap: "Summing total_cost_usd over every run's stream result events gives 3.00; the session's main transcript plus its 16 sidechains measure 9.8 MB. Also worth a line: the plan estimated 14-18 runs against a ~$0.60 precedent and the probe spent 21 runs and ~$3.",
     fix: "Correct both figures and add one line noting the run-count and spend overrun against the plan's estimate."}

strengths: abort leg independently reproducible end to end; the field() false-allow is experimental proof of a shipped-wrapper defect and I reproduced the truncation from the real payload; leg 7's `if` placement trap and the Bash(*>*) negative are both carried by fire-level evidence including the absence case; every doc quote I checked is verbatim and the PowerShell arm is honestly marked unverified rather than passed; leg 4 covers all four modes on both transports with PMODE on every line; the transcript substitution behind the cost figures is disclosed in three separate places; the tool-name aggregation reproduced exactly on both sessions.
---

## Failure narrative

The verdict is FAIL, and the abort/proceed reading is nonetheless safe for the user to rule on. Those
two statements do not conflict, and the split is the point of this review.

What is safe. The abort criterion in D8 has two limbs, and both are independently reproducible from the
captured evidence without trusting a single line of the report's prose. The `PreToolUse` hook fired
inside an `Agent` subagent: I opened the payload and it carries `agent_id` a6673d77affd5ad8a,
`agent_type` general-purpose, `permission_mode` dontAsk, and a `tool_input.file_path` under the gated
home. The deny stopped the write: the hook log records `DECISION=deny` for that fire, the run's summary
lists `probe-home` holding only its seed file, and `run.sh` deletes every non-seed file before each run,
so that listing cannot be stale. The deny reached the seat as text: the subagent's own sidechain carries
the deny string as a `tool_result` with `is_error` true and `toolDenialKind` "permission-rule". The
second abort limb holds too, on both transports and in both modes that matter: `acceptEdits` and
`bypassPermissions` each show a Write deny and a Bash deny on the lead and on a subagent, with the
`permission_mode` field on every line proving the mode was in effect, and with all four home listings
clean afterwards. Nothing I checked weakens the PROCEED reading, and the three disclosed holes the
report carries forward are honestly scoped.

What fails. Two of the six graded items fail, and a third fails on a single claim.

The cost model carries a count that does not reproduce and that inverts the report's own conclusion.
The report's narrowed figure for the first session, 225, reproduces exactly when the Bash term is
"commands whose text mentions `.mochiko`" — I recounted 200 Bash calls under that predicate, matching
the report to the call. Applying the same predicate to the second session gives 154, not the 287 the
report states. That moves the second session's narrowed total from 321 to 188, its narrowed projection
from 15.1 s to 8.9 s, and the narrowing's effect from "about 38 percent" to about 64 percent. The
report's closing argument to the lead rests on that 38 percent: it tells the lead the `if` narrowing
"is not the lever it looked like" and proposes leg 8's `SubagentStart` channel as the larger one. Under
the corrected pair, 60 percent and 64 percent, the narrowing is a consistent and substantial lever and
the recommendation has to be re-derived rather than patched. A second count, the 206 and 211 Bash calls
said to carry a write operator, reproduces nowhere: my most inclusive operator set reaches 180 for both
sessions. That one matters less for the conclusion, which survives at 169-180, and more for the report's
honesty claim, because `enumeration_dispatch` states the author re-ran the write-operator split and it
matched. It does not match.

Plan conformance fails on accounting rather than on work. I believe all 32 planned cells resolved, but
the report cannot be read to say so: it declares 31 of 32 resolved and names the unresolvable one as the
PowerShell arm, which was never one of the 32 planned cells. Leg 8 is correctly marked lead-added; leg 9
is not marked at all, so an addition occupies the slot of a planned cell and the reader learns nothing
about whether a planned cell went missing. Separately, D8 asks for `Write`/`Edit`/`Read` per transport,
the plan quietly replaced `Edit` with the Bash deny channel, and the report inherits the substitution
without disclosing it. `Edit` was exercised three times in the entire probe, and the single `Edit` deny
came from a delegated subagent in a retry run, not from any leg 1 cell.

The findings item fails on one claim of six. The report tells wave 4 to write any future spawn matcher
against the wire name `Task`. The advertised tool list does say `Task`, and I confirmed it in the init
event. But the same stream records every spawn as a tool_use named `Agent`, ten times, and never as
`Task`, and no hook in this probe ever fired on a spawn call because the matcher set excluded it. A
matcher built on the advertised name alone could silently never fire, which is precisely the failure
mode leg 5 proves is invisible to the model and nearly invisible to the operator. That recommendation
needs marking unverified and a one-cell probe before anything is built on it.

What to do with this. The user can rule PROCEED now on the strength of item 1. The corrections the
report owes are the budget number and the narrowing recommendation, which wave 1 does not need on day
one but the budget ruling does, plus the accounting and pointer fixes, which are cheap. Fifteen fix
items, one Critical, five Important, nine Minor; none of them touches the abort reading.

## Notes of note

Two envelope defects in the graded artifact, reported here rather than as findings because they are the
lead's to enforce under `templates/report-format.md` rule 9, not the probe's to re-litigate. The report
declares `report: verification`, for which rule 2 closes the body-section set at failure narrative,
notes of note and null-exit reasoning. It carries `## Abort / proceed reading` and `## Budget proposal
for the lead` as well, and both restate content that already sits in frontmatter fields (`verdict`,
`abort_criterion`, `proposed_budget`). The honest repair is to fold both into fields rather than to
delete the reasoning, since the abort reading is the decisive content the user rules on.

One platform fact worth carrying into wave 1's design, not a defect in the report. The hooks-guide's
Limitations section says that for background subagents in non-interactive mode, if no hook returns a
decision, Claude Code denies the call. The probe's explicit-allow design depends on that sentence and I
verified it verbatim. It means the shipped wrapper's fail-open posture is not uniform across contexts:
a wrapper that exits without emitting a decision is fail-open for the lead and fail-closed for a
background subagent. D9's fail-open assumption and F7 should be re-read against that asymmetry before
the crate wave ships the real wrapper.

One measurement caveat that survives every correction. My re-timing puts the spawn floor at 2.0 ms for
`/usr/bin/true` and 4.1 ms for `sh -c true` through the same kind of harness, with the proxy CLI at
45.4 ms. So essentially the whole of the 47.1 ms per-call median is the binary's own startup, not
harness overhead and not the wrapper. That is good news for the budget, because it means the figure is
real rather than inflated, and it makes the `>100 ms median` cache trigger a genuine 2x headroom rather
than an artifact of how the measurement was taken.

## Re-grade

Round-1 fix pass, bounded to the 15 findings of this review. No new angles and no new findings.
Each line is held / not held against the revised `wave0-probe-report.md`, its
`corrections_from_round_1` block and the raw evidence, re-verified where the fix asserted a number.

- **G4 (Critical, narrowing predicate and recount) — HELD.** `cost.narrowing_predicate` states the
  predicate in one line: the literal substring `.mochiko` in `tool_input.file_path` or anywhere in
  `tool_input.command`. My recount reproduces every corrected figure: Bash mentioning 200 and 154,
  narrowed totals 225 and 188, reductions 60 and 64 percent, and 339 of 564 and 332 of 520 removed.
  The projections recompute exactly at the new 49.0 ms basis (27.6 / 11.0 / 25.5 / 9.2). The "not the
  lever it looked like" conclusion is explicitly withdrawn and the leg 8 recommendation re-derived as
  a delivery-shape argument worth 1.0 and 1.6 seconds, which also recomputes. The revision goes
  further than the fix asked by disclosing that the predicate over-states what the shipped Bash
  narrowing would catch, since leg 7 measured that Bash `if` matches command text only — a
  gate-coverage risk, not just a cost-estimate risk. Correct call, and it belongs in wave 4's matcher
  design.
- **G5 (Important, write-operator sets and the withdrawn self-check) — HELD.** Both sets are
  published as literal lists, and the core set reproduces at 169 and 165 under the stated substring
  test, matching the revision to the call. `enumeration_dispatch` withdraws the claim that the
  write-operator split had been re-verified and names the root cause: a line-oriented grep counting
  matching lines instead of matching calls. I confirmed that root cause directly — line matches give
  287 for the second session's `.mochiko` Bash count and 206 for the first session's core operators,
  reproducing the first pass's two wrong figures exactly. One residual, immaterial: the
  `delta_vs_reviewer` note attributes our one-call gap to a `cp `/`mv ` word-boundary choice, and a
  word-boundary test changes neither session's count. The gap is my `tee` without a trailing space;
  the revision's 165 is the right number under the set it publishes.
- **G6 (Important, the 2 percent limb) — HELD.** The limb is dropped, not patched.
  `binding_limb` carries both wall-clock spans and the 424 s / 89 s figures, matching my computation,
  and `share_limb_dropped` adds the other end of the range (the probe's own 41 s median run, where
  2 percent is 0.8 s and no hook design meets it). The share survives as a named watch metric with
  both measurement methods stated.
- **G1 (Important, cell accounting) — HELD.** `cell_accounting` reports 32 planned and 32 resolved,
  names both substitutions, lists legs 8 and 9 as additions outside the plan, and states that the one
  unresolvable cell is leg 9's firing and "is not one of the plan's 32".
- **G2 (Important, D8's Edit limb) — HELD.** `leg 1.edit_limb` is marked DISCLOSED HOLE and carries
  the whole of the Edit evidence. All three Edit fires check out against the log: the deny at
  TS=1789280955 (`agent_id` ac54ea336ef8cfbf6, a delegated subagent in r0-retryfix-2) and the two
  allow/no-home-hit fires at TS=1789280323 under auto and TS=1789280895 under dontAsk. The hole also
  rides forward as item 4 of the abort reading, which the first pass did not have.
- **G8 (Important, the `Task` wire name) — HELD.** Marked UNVERIFIED with both sides cited (the init
  event's tools array against 10 `"name":"Agent"` tool_use records and 0 for `Task`), the reason the
  conflict matters stated, and a one-cell wave-1 probe proposed before any spawn matcher ships.
- **G3 (Minor, retry-cell attribution) — HELD.** `retry_arm_attribution` states all four retry runs
  were lead-seat under `prompts/p-retry.txt` and that the T2 cell is covered by substitution.
- **G7 (Minor, the wrapper-only figure) — HELD, and verified rather than taken on trust.** The 34.6 ms
  row is relabelled `probe_wrapper_logging_overhead_ms` with the cause given (7 `field()` extractions,
  each a multi-process pipeline). I timed the probe wrapper against a real payload: 32.8 ms median,
  consistent with the relabelling. The shipped shape is measured separately at 7.4 ms and is additive
  with the CLI, and the spawn floor is now stated so the reader can separate harness overhead from
  hook cost.
- **G9 (Minor, missing quote pointers) — HELD.** Both pointers added: `runs/r0-retryfix-1/report.txt`
  for the prompt-injection reading, `runs/r0-substart/report.txt` and its stream for the
  instruction-shaped instance.
- **G10 (Minor, escalation order) — HELD.** The single narrated order is replaced by per-run fire
  sequences. Both transcriptions match the log line for line, including the two distinct delegates in
  r0-retryfix-1, the absence of an Edit deny there, and the Edit deny landing after delegation in
  r0-retryfix-2.
- **G11 (Minor, dead evidence pointer) — HELD.** Leg 1 is repointed at `runs/<id>/stream.jsonl` with
  the sidechains named for the subagent view, and the `out/` reset is disclosed with the right count:
  14 runs preserved a seat report, and r01 through r04 did not. I counted 14 `report.txt` files.
- **G12 (Minor, overwritten group-placement config) — HELD.** The group-placement JSON is transcribed
  with `if` on the matcher-group object, alongside the seven-fire / three-hits-no outcome that proves
  the trap.
- **G13 (Minor, abridged `if` citation) — HELD.** The `if` row is quoted in full, matching the live
  page, and the Bash-if-matching table is now cited with its actual coverage and its silence on
  redirect and pipe operators — so the `Bash(*>*)` negative is framed as the measured answer to an
  undocumented question rather than a contradiction.
- **G14 (Minor, leg 8 discriminator) — HELD.** `controlled_test` names both sentinels and cites the
  `hook_success` stdout attachment beside the `hook_additional_context` one.
- **G15 (Minor, spend and size) — HELD.** Spend corrected to 3.00, matching my sum over the runs'
  result events; size to 9.8 MB; and a `plan_estimate` block states the three-run and five-times-spend
  overrun against the plan, which the fix asked for as one line.

Two items outside the 15, recorded so they are not mistaken for re-grade findings. The report still
carries `## Abort / proceed reading` and `## Budget proposal for the lead`, body sections its declared
`report: verification` envelope does not sanction; that was a note to the lead in round 0, never a G
item, and it is the lead's to enforce or waive. Second, my own G7 timing check ran the probe wrapper
20 times, which appended 20 log lines tagged `ENTRY=narrow-timingcheck` and 20 files to `fires/`. The
report's `fires_captured: 153` and the 195-line log were correct when written; the on-disk counts are
now 173 and 215, and the 20 extra entries are mine, not probe evidence. I left them in place rather
than edit the evidence trail.

verdict: PASS
items_not_held: 0
