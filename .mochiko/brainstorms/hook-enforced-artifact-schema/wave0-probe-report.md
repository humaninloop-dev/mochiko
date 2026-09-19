---
report: verification
feature: hook-enforced-artifact-schema
round: 0
revision: round-1 fix pass, G1-G15 of wave0-probe-review.md folded
verdict: proceed
abort_criterion: not-tripped
ruling_home: record.md D8 (legs, abort criterion) + OQ3 + OQ4; legs 8 and 9 are additions outside D8
plan: wave0-probe-plan.md
review: wave0-probe-review.md
platform: Claude Code 2.1.258, macOS (darwin 25.6.0) arm64
binary: mochiko-cli on PATH at ~/.cargo/bin/mochiko-cli
method: >
  Four throwaway plugins under the session scratchpad (never under plugins/), loaded with
  --plugin-dir and run headless from a scratch cwd: claude -p --model sonnet --setting-sources ''
  --output-format stream-json --verbose --include-hook-events --max-turns 20. Every claim below is
  read off the hook's own append log, the per-fire raw payload files, the run's stream, or the
  subagent sidechain transcripts. Scratch material is not committed.
runs: 21
spend_usd: 3.00
plan_estimate: >
  The plan estimated 14 to 18 runs against a precedent spend of about 0.60 dollars. The probe ran
  21 and spent 3.00 — three runs above the plan's upper bound, and five times the precedent spend.
  The extra runs are the leg 7 `if`-placement re-run, the corrected-parse retry pair, and the
  Bash-pattern variant run, all of which the first pass forced; the per-run spend is higher than the
  precedent because these prompts drive multi-step tool sequences rather than one command.
fires_captured: 153
run_wall_clock_seconds: {min: 10, median: 41, max: 136}

cell_accounting:
  planned: 32
  planned_resolved: 32
  substitutions:
    - >
      Leg 1 (12 cells): the plan's three channels are Write deny, Read reminder and Bash deny. D8's
      own wording names Write/Edit/Read, so the plan already substituted the Bash deny channel for
      D8's Edit limb, and neither the plan nor the first pass of this report disclosed it. See leg 1
      edit_limb below.
    - >
      Leg 3 (4 cells): the heredoc arm resolved on all four transports. The retry arm ran four times
      and every run was driven at the lead seat, so the T2 retry cell is covered by substitution —
      the delegated subagents denied inside r0-retryfix-1 and r0-retryfix-2 — not by a subagent-driven
      retry prompt. See leg 3 attribution below.
  additions_outside_the_plan:
    - {leg: 8, name: SubagentStart context injection, origin: lead-added after plan approval}
    - {leg: 9, name: PowerShell arm of the D1c matcher, origin: lead-added at approval, note 2}
  unresolvable: >
    One cell among the additions: leg 9's firing. It is not one of the plan's 32. No PowerShell tool
    exists in the session's tool set on macOS, so no call can reach that matcher arm here.

legs:
  - id: 1
    name: transport coverage
    result: PASS
    cells: 12/12
    detail: >
      The PreToolUse hook fires, and its text reaches the seat, on all four transports and all
      three probed channels. Write deny blocked every write (probe-home held only its seed file
      after every run, and run.sh deletes every non-seed file before each run, so the listing is
      ground truth). Read additionalContext arrived at every seat as "PreToolUse:Read hook
      additional context: <text>". Bash heredoc deny blocked the shell write.
    transport_fields:
      T1_lead: agent_id absent, agent_type absent
      T2_agent_subagent: agent_id=a6673d77affd5ad8a, agent_type=general-purpose
      T3_named_teammate: agent_id=a5eb4ba7a24798cd5, agent_type=general-purpose
      T4_agent_flag: agent_id absent, agent_type=probe-gate:probe-writer
    edit_limb: >
      DISCLOSED HOLE. D8 asks about Write, Edit and Read per transport; no Edit cell was run for any
      transport. Edit fired three times in the whole probe, and the whole of the Edit evidence is one
      deny: probe-gate.log TS=1789280955, SID=8b9628e3-0632-4e52-8602-865ddeb93831,
      agent_id=ac54ea336ef8cfbf6, agent_type=probe-gate2:probe-writer, DECISION=deny — a delegated
      subagent inside r0-retryfix-2. The other two fires are allow/no-home-hit (TS=1789280323 under
      auto, TS=1789280895 under dontAsk), both on paths outside the home. Reason the hole is accepted
      rather than closed here: Edit rides the same matcher arm as Write and carries the same
      tool_input.file_path key, and the one deny confirms the arm decides on Edit. Closing it properly
      is a one-run wave-1 cell, per the lead's instruction not to run new probes in this round.
    evidence: >
      probe-gate.log runs r01-t1-dontask, r02-t2-subagent-dontask, r03-t3-teammate-dontask,
      r04-t4-agentflag-dontask, plus each run's runs/<id>/stream.jsonl, which carries all three
      sentinels for all four transports (SENTINEL-DENY-WRITE, SENTINEL-DENY-BASH, SENTINEL-REMINDER
      present in each). For the subagent's own view of the denial, the sidechains under the probe-cwd
      project slug for sessions 9669f09e (T2) and 50a41d8a (T3). Note: run.sh clears probe-cwd/out
      before every run, so a seat's own report.txt survives only for the 14 runs that copied it to
      runs/<id>/report.txt; r01 through r04 did not, so the stream is their surviving proof.
    citation: >
      hooks reference, Hook locations: "Hooks from settings files, managed policy settings, and
      plugins also run inside subagents. When a subagent calls a tool, tool events such as PreToolUse
      and PostToolUse fire the same configured hooks as in the main conversation, and the input
      carries the agent_id and agent_type common input fields that identify the subagent." Confirmed
      empirically. The T4 field pattern is explained by the same page's Common input fields: "For
      subagents, the subagent's type takes precedence over the session's --agent value."
  - id: 1b
    name: teammate transport (OQ3)
    result: PASS — no hole, OQ3 mitigation not owed
    detail: >
      A named Agent spawn is reachable headless and is gated. The SendMessage resume round is also
      gated: the follow-up turn's Write to a home was denied like the first.
    evidence: >
      run r03; the lead's own report (runs/r03-t3-teammate-dontask, quoted in its stream) records
      'SendMessage({to: "probeteammate"}) ... {"success":true,"message":"Resuming agent
      probeteammate","resumedAgentId":"a5eb4ba7a24798cd5"}'; the resumed turn's deny is
      probe-gate.log TS=1789280043, same agent_id.
  - id: 2
    name: Read tool_input field name
    result: file_path
    detail: >
      Read carries tool_input.file_path, the same key as Write and Edit, holding an absolute path.
      tool_input has exactly one key on Read. PreToolUse top-level keys observed: cwd, effort,
      hook_event_name, permission_mode, prompt_id, session_id, tool_input, tool_name, tool_use_id,
      transcript_path (plus agent_id / agent_type inside a subagent).
    evidence: captured payload fires/1789279891-15713-broad-PreToolUse-Read.json
    citation: >
      Undocumented on either hooks page — the reference gives no tool_input schema for Read. This is
      the measured answer; F8's gap closed.
  - id: 3
    name: Bash heredoc write and denied-seat retry
    result: PASS (fires) · retry behaviour measured, n=4
    heredoc_arm: >
      Resolved on all four transports. A `cat > <home>/x.md <<EOF` through the Bash tool reaches the
      Bash-matched hook with the command text intact, newlines preserved:
      fires/1789279894-15791-broad-PreToolUse-Bash.json carries
      tool_input.command = "cat > probe-home/b.md <<'X'\nprobe write B\nX".
    retry_arm_attribution: >
      All four retry runs were driven at the lead seat by prompts/p-retry.txt (T1). The T2 retry cell
      is covered by substitution: in both corrected-parse runs the lead delegated to a fresh subagent
      and those subagents were denied on every write path. No subagent-driven retry prompt was run.
    retry_with_naive_parse: >
      n=2 (r0-retry-1, r0-retry-2). r0-retry-1: both denials accepted, no file created. r0-retry-2:
      the seat's shell redirect SUCCEEDED and probe-home/c.md was created. Cause is the probe
      wrapper's grep/sed field() extraction, not the transport — see the field() note below.
    retry_with_correct_parse: >
      n=2 (r0-retryfix-1, r0-retryfix-2), whole-payload match instead of field(). No file created in
      either. The escalation SET is Write, a shell redirect, Edit, and delegation to a fresh
      subagent; the ORDER differs per run, so it is given per run rather than narrated as one
      sequence.
      r0-retryfix-1 fire order: Write deny (lead) · Bash deny (lead) · Read allow+reminder (lead) ·
      two Bash allow (delegated ad9f4789249e47c71) · Write deny · Bash deny · Write deny (same
      delegate) · Write deny (second delegate a7a48fc855f31e905) · Write allow and Edit allow on
      out-of-home report paths. No Edit deny in this run.
      r0-retryfix-2 fire order: two Bash allow (lead) · Write deny · Bash deny · Read allow+reminder ·
      Write deny (lead) · Write deny · Bash deny · Read allow+reminder · Edit deny (delegated
      ac54ea336ef8cfbf6) · Write allow on the out-of-home report path. The Edit deny came after
      delegation, not before it.
    evidence: >
      payload fires/1789280577-27819 (command 'printf 'retry probe' > "<abs path>/probe-home/c.md"'),
      probe-gate.log DECISION=allow/no-home-hit for that fire, probe-home/c.md present in
      runs/r0-retry-2/summary.txt; runs r0-retryfix-1 and r0-retryfix-2 summaries both hold seed.md
      only. Seat reports preserved at runs/r0-retry-{1,2}/report.txt and
      runs/r0-retryfix-{1,2}/report.txt.
    citation: >
      hooks-guide, Block edits to protected files: "For per-call coverage instead, also match
      `Bash|PowerShell` and have your script list modified and untracked files with `git status
      --porcelain`."
  - id: 4
    name: permission modes
    result: PASS
    cells: 8/8
    detail: >
      The Write deny and the Bash deny held on T1 and T2 under dontAsk, acceptEdits,
      bypassPermissions and auto. The payload's permission_mode field confirms the mode was in
      effect for each. The CLI exposes no `default` mode (choices: acceptEdits, auto,
      bypassPermissions, manual, dontAsk, plan), so `auto` was tested in its place.
    evidence: >
      probe-gate.log runs r0-t1-* and r0-t2-*, PMODE field on every deny line; SIDs 5941eb56 and
      1d598f18 (acceptEdits), 2456fdb6 and 02265a7e (bypassPermissions), 363792b1 and 164451d3
      (auto). All eight home listings hold seed.md only.
    citation: >
      hooks-guide, Hooks and permission modes: "PreToolUse hooks fire before any permission-mode
      check, in every permission mode, including `dontAsk`. A hook that returns
      `permissionDecision: \"deny\"` blocks the tool even in `bypassPermissions` mode or with
      `--dangerously-skip-permissions`."
  - id: 5
    name: dead gate
    result: fail-open and silent to the model
    detail: >
      Non-executable wrapper: exit_code 126, outcome "error", the Write proceeded. Unresolvable
      command path: the same shape with a 127-class message. In both cases the seat reported "step
      1: File created successfully" and saw nothing about a hook. The only trace is a stream event
      the operator sees.
    evidence: >
      runs r0-deadexec and r0-deadpath; both summaries list probe-home holding a.md and b.md.
      Stream event: {"type":"system","subtype":"hook_response","hook_name":"PreToolUse:Write",
      "output":"/bin/sh: .../probe-gate.sh: Permission denied\n","exit_code":126,"outcome":"error"}.
      Seat reports at runs/r0-deadexec/report.txt and runs/r0-deadpath/report.txt.
    citation: >
      hooks reference, Other exit codes: "a mistyped path in `settings.json` leaves the gate
      silently disabled."
  - id: 6
    name: aggregate per-run cost
    result: measured; budget proposed below
    limitation: >
      No EPIC-002 implement-run transcript exists on this machine. A search of every kinako project
      directory found zero files mentioning implement-log, and no session in the 2026-09-09..12
      window wrote a tasks.md, a cycle report, or an implement log. The two largest available runs
      were measured instead and are named for what they are.
  - id: 7
    name: the `if` field
    result: PASS, with a placement trap
    placement: >
      `if` belongs on the individual handler object, sibling of `type`. Placed on the matcher-group
      object it is silently ignored and every matched call runs the hook. The group-placement config
      that produced run r0-if was later overwritten, so it is transcribed here for wave 4 to
      regression-test: {"hooks":{"PreToolUse":[{"matcher":"Write|Edit","if":"Write(probe-home/**)",
      "hooks":[{"type":"command","command":"${CLAUDE_PLUGIN_ROOT}/hooks/scripts/probe-gate.sh
      narrow-write","timeout":5}]}, … same shape for {"matcher":"Read","if":"Read(probe-home/**)"}
      and {"matcher":"Bash","if":"Bash(cat *)"}]}}. Under that config r0-if produced seven narrow
      fires, three of them DECISION=allow/narrow-logged-hits-no — calls the `if` should have
      excluded.
    narrowings_that_hold: >
      With `if` on the handler, run r0-if2 produced three fires, all hits-yes:
      Write(probe-home/**) only on the in-home Write, Read(probe-home/**) only on the in-home Read,
      Bash(cat *) only on the cat. A cwd-relative pattern matches an absolute path under that cwd.
      Run r0-if3: Read(**/probe-home/**) and Bash(*probe-home*) both fired only on their in-home
      call. Bash(*>*) did NOT fire on `printf zed > out/z.txt`.
    reading: >
      Bash can be narrowed by a path substring in the command text but not by write-operator shape.
    versions: installed 2.1.258; no minimum version is documented on the `if` field itself
    evidence: probe-gate.log runs r0-if (7 fires, 3 hits-no), r0-if2 (3 fires, all hits-yes), r0-if3 (2 fires, both hits-yes; the echo and the redirect produced no fire)
    citation: >
      hooks reference, Common fields, the `if` row in full: "Permission rule syntax to filter when
      this hook runs, such as `\"Bash(git *)\"` or `\"Edit(*.ts)\"`. The hook command only runs if
      the tool call matches the pattern. See the Bash matching table (#bash-if-matching) below for
      how Bash patterns evaluate against subcommands, `$()`, and backticks. Only evaluated on tool
      events: `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`, and
      `PermissionDenied`. On other events, a hook with `if` set never runs. Uses the same syntax as
      permission rules." The same page's Bash-if-matching table covers leading `VAR=value` assignment
      stripping, per-subcommand checks across `&&`, commands inside `$()` and backticks, and the case
      where an expansion cannot be determined (the hook runs anyway). It says nothing about redirect
      or pipe operators, so the Bash(*>*) negative is the measured answer to an undocumented
      question, not a contradiction of the documented behaviour.
  - id: 8
    name: SubagentStart context injection (lead-added, outside D8)
    result: PASS
    detail: >
      A SubagentStart hook returning hookSpecificOutput.additionalContext lands in the subagent's
      context before its first turn, for both an unnamed Agent spawn and a named teammate spawn.
      agent_type on a named spawn is still general-purpose, so the event's agent-type matcher cannot
      single out a named teammate.
    controlled_test: >
      The probe script emitted two distinct sentinels in one output — hookSpecificOutput
      .additionalContext = "SENTINEL-SUBAGENT-START …" and a top-level additionalContext =
      "SENTINEL-SUBAGENT-START-TOPLEVEL …" — so the non-consumption claim is measured, not inferred.
      Both strings survive in each sidechain inside a {"type":"hook_success"} attachment recording
      the hook's raw stdout; only the hookSpecificOutput sentinel reaches the
      {"type":"hook_additional_context"} attachment the seat actually reads.
    evidence: >
      run r0-substart, two fires (agent_id a2e307af39787f03f and a2a3cf72c66e578e5). Both sidechains
      under the probe-cwd project slug, session d70ef3e2, carry at line 1
      "attachment":{"type":"hook_additional_context","content":["SENTINEL-SUBAGENT-START probe
      context injected at SubagentStart"]} and, separately, the hook_success attachment holding both
      sentinels. Seat report at runs/r0-substart/report.txt, which quotes it as "SubagentStart hook
      additional context: <text>".
    citation: "hooks reference, Matcher patterns: SubagentStart filters on agent type."
  - id: 9
    name: PowerShell arm of the D1c matcher (lead-added)
    result: registration valid; firing UNVERIFIED on this platform
    detail: >
      The matcher `Write|Edit|Bash|PowerShell|Read` loaded with zero hook-config errors across all
      21 runs and fired correctly for its other arms. No PowerShell tool exists in the session's
      tool set on macOS, so no call can reach the PowerShell arm and its firing cannot be tested
      here. Marked unverified, not passed.
    doc_defect: >
      hooks-guide links "[PowerShell hook input section](/docs/en/hooks#powershell)" but the hooks
      reference has no #powershell anchor and no PowerShell hook-input section — only a Windows
      (PowerShell) tab under How a hook resolves. The V6 quote it sits beside is verbatim and holds;
      the explanation it points at does not exist.

cost:
  per_call_method: >
    hyperfine absent; 20 timed warm runs per row via python subprocess from the repo root. Two passes
    were taken; the second adds the spawn floor so the reader can separate harness overhead from hook
    cost. Projections below use the slower pass, 49.0 ms, as the conservative basis.
  spawn_floor_ms: {usr_bin_true_median: 3.1, sh_c_true_median: 4.7}
  sh_wrapper_no_cli_ms: {median: 7.4}
  cli_direct_ms: {pass1_median: 40.9, pass2_median: 43.6, pass2_p95: 44.7}
  thin_wrapper_plus_cli_ms: {pass1_median: 47.1, pass2_median: 49.0, pass2_p95: 50.6}
  probe_wrapper_logging_overhead_ms: >
    The first pass reported a 34.6 ms "wrapper_only" figure. That was the PROBE wrapper, which forks
    roughly thirty processes per fire (one payload write plus six field() extractions, each a
    tr|grep|head|sed pipeline). It is not the shipped wrapper shape and it fed no projection. The
    shipped shape is the 7.4 ms row above, which is additive with the CLI as expected.
  proxy: mochiko-cli migrate status --plugin-root plugins/mochiko (check does not exist yet)
  narrowing_predicate: >
    A call counts as narrowed-in when the literal substring `.mochiko` appears in tool_input.file_path
    (Read, Write, Edit) or anywhere in tool_input.command (Bash) — the proxy for `if` patterns of the
    form Read(.mochiko/**) and Bash(*.mochiko*). Leg 7 measured that Bash `if` matches against the
    command text, so a shell command that reaches a home through a variable, or through a relative
    path after a cd, would not match; the proxy therefore over-states what the shipped Bash narrowing
    would catch, and that is a gate-coverage risk, not only a cost estimate error.
  write_operator_sets:
    core: ["> (covers >>)", "tee ", "sed -i", "cp ", "mv "]
    extended: ["> (covers >>)", "tee ", "sed -i", "cp ", "mv ", "<<", "dd ", "touch "]
    membership_test: plain substring containment in the command text, one hit per call
    delta_vs_reviewer: >
      The reviewer's recount gives core 169 and 166; mine gives 169 and 165. The one-call difference
      on 23e1aa9b is a boundary choice on `cp ` / `mv ` (a command ending in the bare word, or
      separated by a tab, matches under a word-boundary test and not under substring containment).
      The extended set reproduces identically at 180 and 180, and the qualitative claim is unaffected.
  sessions_measured:
    - id: 613c3001-67b5-4897-870c-ea9c2a2858bf
      what: kinako brainstorm plus review run, 2026-09-09, 13.7 MB aggregate, 8 sidechains
      wall_clock: {span_s: 21218, span_h: 5.89, first: "2026-09-09T02:51:50Z", last: "2026-09-09T08:45:28Z"}
      calls: {Read: 71, Write: 7, Edit: 2, Bash: 484, gated_total: 564, all_tool_use: 645}
      under_mochiko: {Read: 21, Write_Edit: 4, Bash_mentioning: 200, narrowed_total: 225}
      bash_with_write_operator: {core: 169, extended: 180, redirect_only: 167}
    - id: 23e1aa9b-d262-4d34-bb29-5ca2986e0dc7
      what: kinako mochiko:feature run, 2026-09-09, 9.8 MB aggregate, 16 sidechains
      wall_clock: {span_s: 4461, span_h: 1.24, first: "2026-09-09T07:21:13Z", last: "2026-09-09T08:35:34Z"}
      calls: {Read: 73, Write: 4, Edit: 0, Bash: 443, gated_total: 520, all_tool_use: 559}
      under_mochiko: {Read: 33, Write_Edit: 1, Bash_mentioning: 154, narrowed_total: 188}
      bash_with_write_operator: {core: 165, extended: 180, redirect_only: 162}
  projection_seconds:   # gated call count x 49.0 ms
    613c3001: {unnarrowed: 27.6, if_narrowed: 11.0}
    23e1aa9b: {unnarrowed: 25.5, if_narrowed: 9.2}
  narrowing_reduction_pct: {613c3001: 60, 23e1aa9b: 64}
  reminder_lines_per_run: {613c3001: 21, 23e1aa9b: 33}   # the D1b / M2 watch metric under narrowing
  enumeration_dispatch: >
    Call counts were enumerated by a native Explore subagent at model haiku per
    mochiko:patterns-model-tiering. I re-ran the tool-name aggregation myself and it reproduced
    exactly on both sessions, and the Explore seat's per-call `.mochiko` count of 200 for 613c3001
    also reproduced. The first pass of this report additionally claimed I had re-verified the Bash
    write-operator split; that claim is WITHDRAWN. I ran it with a line-oriented grep over a file of
    jq-extracted commands, which counts matching LINES rather than matching CALLS and therefore
    inflates any multi-line command. That same error produced the first pass's 287 for 23e1aa9b's
    `.mochiko` Bash count (correct value 154) and its 206 and 211 write-operator figures (correct core
    values 169 and 165). The error was mine, not the Explore seat's. Every count in this revision is
    a per-call jq or python count over parsed objects.

corrections_from_round_1:
  - {field: "cost.sessions_measured[23e1aa9b].under_mochiko.Bash_mentioning", was: 287, now: 154}
  - {field: "cost.sessions_measured[23e1aa9b].under_mochiko.narrowed_total", was: 321, now: 188}
  - {field: "cost.projection_seconds.23e1aa9b.if_narrowed", was: 15.1, now: 9.2}
  - {field: "cost.projection_seconds (all four)", was: "at 47.1 ms", now: "at 49.0 ms, the slower of two timing passes"}
  - {field: "narrowing_reduction_pct.23e1aa9b", was: 38, now: 64}
  - {field: "bash_with_write_operator", was: "206 and 211", now: "core 169 and 165, extended 180 and 180"}
  - {field: "spend_usd", was: 2.78, now: 3.00}
  - {field: "cost.sessions_measured[23e1aa9b].what size", was: "9.3 MB", now: "9.8 MB"}
  - {field: "cost.wrapper_only_ms", was: 34.6, now: "relabelled probe_wrapper_logging_overhead_ms; shipped shape measured at 7.4 ms"}
  - {field: "proposed_budget.per_run_share_of_wall_clock_max_pct", was: 2, now: "dropped, see proposed_budget.share_limb_dropped"}
  - {field: "Notes of note, the `Task` wire name recommendation", was: "asserted", now: "marked UNVERIFIED with both sides of the evidence"}
  - {field: "leg 3 escalation order", was: "one narrated order", now: "per-run fire sequences"}

proposed_budget:
  per_run_aggregate_seconds_max: 60
  measured_baseline_seconds: 9.2 to 27.6
  measure: gated call count for the run times the measured per-call median
  binding_limb: >
    The 60 s absolute cap binds on both measured sessions. Two percent of their wall clock is 424 s
    and 89 s, so a percentage limb would never fire on either.
  share_limb_dropped: >
    The 2 percent limb from the first pass is withdrawn. It is unreachable as a gate at the other end
    of the range: the probe's own 21 runs ran 10 s to 136 s with a 41 s median, where 2 percent is
    0.8 s and no hook design meets it, while a 27.6 s hook cost on a 41 s run would be 67 percent of
    wall clock. A single absolute cap is the only limb that means the same thing on a six-hour run and
    a forty-second one.
  share_reported_not_gated: >
    The hook share of wall clock is still worth reporting per run, as a watch metric: duration_ms from
    the stream's result event for a headless run, and the transcript's first-to-last timestamp span for
    an interactive one. Both measures are named so a future run does not have to choose.
  breach_action: >
    the ${CLAUDE_PLUGIN_DATA} replay cache D3 deferred on measured need is the mitigation; the
    trigger is a per-call median above 100 ms or a projection above 60 s. The 5 s per-call timeout
    stays a separate correctness floor, since a timed-out hook is fail-open and enforces nothing.
---

## Notes of note

**The read-time reminder was read as a prompt injection and deliberately ignored.** One seat, on
seeing the `additionalContext` line, wrote in its own report: "Note: this reminder reads like an
attempted prompt injection (an unsolicited instruction to run a `mochiko-cli template <kind>`
command). It was not something the user asked for, so I flagged it and did not act on it."
(`runs/r0-retryfix-1/report.txt`.) The channel is reliable; its *wording* is not. An unsourced
imperative naming a CLI command reads as hostile. D1b's line needs to identify itself as the
project's own gate, and the same caution applies to any imperative D1a ships through a hook rather
than through a rendered rule set. A second instance of the same reflex: when a subagent's reply
quoted its injected context, the harness annotated the reply as "instruction-shaped" and neutralized
the control tags (`runs/r0-substart/report.txt` and its stream).

**The Bash leg is the main write path in real runs, not the escape hatch.** Across the two measured
sessions, Write plus Edit accounts for 9 and 4 calls against 484 and 443 Bash calls, and 169 and 165
of those Bash calls carry one of the core write operators (180 and 180 under the extended set). The
artifacts in these runs were written almost entirely through the shell. D1c's `Bash` arm is therefore
load-bearing, and V12's warning that shell-call volume dominates the cost measure is confirmed: Bash
is 86 percent and 85 percent of the gated call volume. `NotebookEdit` is also a live tool in the
seat's advertised set (`runs/r0-retryfix-1/stream.jsonl`, the init event's tools array), so D1c's
knowing exclusion of it is a real hole, not a theoretical one.

**The shipped wrapper's field extraction is not merely inelegant; it produced a false allow.** The
`field()` helper that `dependency-halt.sh` uses matches `"key":"[^"]*"`, so a command containing an
escaped quote is truncated before its path. In run r0-retry-2 that turned a redirect into a gated
home into `allow/no-home-hit`, and the file was created. This is I4 and D3's Rust-side parse
requirement confirmed by experiment rather than by argument, and it is the one cell in this probe
where the gate actually failed to hold.

**A denied seat escalates to delegation.** With every direct path denied, both corrected-parse runs
spawned a fresh subagent to do the write on their behalf; `r0-retryfix-1` spawned two. Every
delegate was denied too. D9's evasion list should name delegation alongside the shell, and record
that the platform closes it for free because hooks follow subagents.

**UNVERIFIED — the spawn tool's wire name.** The session's advertised tool list leads with `Task`
(`runs/r0-retryfix-1/stream.jsonl`, init event: `"tools":["Task","Bash","CronCreate",…]`), but every
spawn in the probe is recorded as a `tool_use` with `"name":"Agent"` — 10 occurrences across all 21
run streams, 0 for `Task`. No hook ever fired on a spawn call here, because the matcher set never
included one. A matcher written on the advertised name alone could therefore silently never fire,
which is exactly the failure class leg 5 shows is invisible. Before any spawn matcher ships, wave 1
should run one cell that registers a `PreToolUse` matcher on `Task` and on `Agent` and reports which
one fires.

**The cost figures do not come from an implement run.** The EPIC-002 implement-run transcript is not
on this machine, so the two largest available kinako runs stand in. Both are design-phase runs. An
implement run's mix plausibly has more Write and Edit and fewer exploratory Bash calls, which would
move the narrowed projection up and the unnarrowed one down. The budget below is set with headroom
for that, and the first dogfood run under the gate should re-measure rather than inherit these
numbers.

## Abort / proceed reading

**PROCEED.** The abort criterion is NOT TRIPPED. The gate fires for `Agent` subagents, with
`agent_id` and `agent_type` on the payload, and the deny stops the write. Leg 4's second abort
condition is also clear: the deny held in `acceptEdits` and `bypassPermissions` on both the lead
and the subagent transport.

Four items ride forward as disclosed, none of them an abort:

1. The `PowerShell` arm cannot be tested on macOS. Wave 4 ships it on the doc quote alone, or the
   contract suite adds a Windows leg.
2. The `Bash` narrowing is by path substring only. A redirect-shaped `if` does not fire, so the
   Bash arm either runs on every shell call or is narrowed with a pattern like `Bash(*.mochiko*)`
   that a command using a shell variable or a relative path would slip past. Best-effort, as D1c
   already states.
3. `NotebookEdit` is a live writer outside the matcher set.
4. D8's `Edit` limb was not run per transport. One delegated-subagent Edit deny is the whole of the
   Edit evidence. Wave 1 closes it with a one-run cell.

One repair the crate wave owes, promoted from a note: D3's `check --hook-json -` must parse the
payload in Rust. The probe proved a grep-and-sed wrapper mis-parses a real command into an allow.

## Budget proposal for the lead

Per-run aggregate hook cost at most 60 seconds, measured as the run's gated call count times the
measured per-call median. The measured baseline sits between 9.2 and 27.6 seconds. The cap binds on
both measured sessions; the first pass's 2 percent-of-wall-clock limb is withdrawn, because at 5.9
hours it never fires and at the probe's own 41-second median it is unmeetable by any hook design.
The share is still worth reporting per run as a watch metric, measured from the stream result's
`duration_ms` headless or the transcript span interactively. Breach trigger for the
`${CLAUDE_PLUGIN_DATA}` replay cache: a per-call median above 100 milliseconds, or a projection
above 60 seconds. The 5 second per-call timeout stays a separate correctness floor.

**The narrowing is the primary cost lever, and the first pass got this backwards.** Under the stated
predicate it removes 339 of 564 gated calls in one session and 332 of 520 in the other, 60 and 64
percent. The first pass reported 38 percent for the second session on an inflated Bash count and
concluded the narrowing "is not the lever it looked like"; that conclusion is withdrawn.

**Re-derived leg 8 recommendation.** `SubagentStart` is worth having, but not as a cost lever: under
the narrowing it removes only the Read term, 21 and 33 calls, about 1.0 and 1.6 seconds per run. Its
real case is delivery shape rather than spend — one injection per seat before the first turn instead
of one per home read, which also retires the repeated-line count D1b accepts as a watch metric and
gives the seat the home rules before it drafts, not after it reaches for a sibling artifact. That is
a D1b change, so it is the lead's to put to the user, not this report's to make.
