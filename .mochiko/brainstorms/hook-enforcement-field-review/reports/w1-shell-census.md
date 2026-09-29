---
report: disclosure
wave: 1
seat: S2
subject: "the shell leg's write-position parse — record D5 as amended at S12, V1, N2; wave plan §3.3"
extraction:
  predicate: "a tool_result carrying 'a second deny on this path halts' whose text opens with the gate's reason, bare or behind 'PreToolUse:<Tool> hook error:'"
  sources: "main thread plus every subagents/*.jsonl of kinako sessions b0eddd5e, 8b0d2f34, d87ac459 (run 3) and abdf4115 (run 4); this repo's cf3ef5ad (field review) and 678e0789 (wave-1 lead)"
  coverage: "the scripts' globs reach every .jsonl find lists: 57, 2, 39 and 125 files"
  run_3: "2026-09-19T07:40 to 2026-09-22T02:56"
  run_4: "2026-09-22T02:56 to 2026-09-23T01:29"
  worktree_cross_check:
    kinako--claude-worktrees-mochiko-run3: "2 sessions, 2026-09-19 11:34-11:36, inside run 3; 0 shell denies"
    kinako--claude-worktrees-brainstorm-sparring-conduct: "402841a2, 2026-09-17 21:15 to 09-18 03:18, wholly before run 3"
    kinako--claude-worktrees-fix-sandbox-check: "5cde9fb6 (2026-09-23 03:25 to 09-24 13:39) and e28f9544 (09-28), wholly after run 4; its 7 shell denies are out of set"
    mochiko worktree dirs: "no shell deny"
  totals: "79 denies (run 3: Write 20, Bash 15, Edit 5; run 4: Write 26, Edit 7, Bash 6) · 21 shell · 13 false · 8 true · 4 added"
  false_by_shape: "(a) sed with a later -i: 6, plus the 4 added · (b) cp source: 5 · (c) home named, write elsewhere: 2"
  secrets_scan: "no credential in the 25 commands, bodies included; 4 non-credential hits (a SHA-256 digest twice, 'credential' in prose twice), all in trimmed body text; re-run on this file and tests/shell.rs: clean"
cases:
  - id: K01
    session: "b0eddd5e"
    thread: "main"
    at: "2026-09-19T07:50:43.978Z"
    class: "false positive"
    shape: "a"
    denied_target_then: ".mochiko/product/contracts/README.md;"
    verdict_then: deny
    expected_verdict_now: allow
    targets_now: []
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K01"
    command: "echo \"=====plugin-bridge prompt/turn clauses=====\"; grep -n -i -E \"every (completed )?turn|prompt half|UserPromptSubmit|leaderText|turn record|ordinal|slash command|bind exchange|B69\" .mochiko/product/contracts/plugin-bridge.md | head -40; echo \"=====contracts README=====\"; sed -n 1,40p .mochiko/product/contracts/README.md; echo \"=====data-model Turn=====\"; grep -n -i -E \"^## |^### |leaderText|kinakoText|ordinal\" .mochiko/product/data-model.md | head -40"
  - id: K02
    session: "b0eddd5e"
    thread: "agent-a56d776d94732e40f"
    at: "2026-09-19T08:38:08.379Z"
    class: "true positive"
    shape: "heredoc cat >"
    denied_target_then: ".mochiko/features/desk/2026-09-19-feat-001-r1-4-and-b69/reports/review-3.md"
    verdict_then: deny
    expected_verdict_now: deny
    targets_now: ["/Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/desk/2026-09-19-feat-001-r1-4-and-b69/reports/review-3.md"]
    trimmed: "heredoc body: 7 of 40 lines kept"
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K02"
    command: "cd /Users/deepeshadmin/Documents/GitHub/kinako\ncat > .mochiko/features/desk/2026-09-19-feat-001-r1-4-and-b69/reports/review-3.md <<'REPORT'\n---\nreport: review\nfeature: FEAT-001\nREPORT\nwc -l .mochiko/features/desk/2026-09-19-feat-001-r1-4-and-b69/reports/review-3.md"
  - id: K03
    session: "b0eddd5e"
    thread: "main"
    at: "2026-09-19T08:49:18.569Z"
    class: "false positive"
    shape: "c"
    denied_target_then: ".mochiko/features/desk/2026-09-19-feat-001-r1-4-and-b69/derivation.md;"
    verdict_then: deny
    expected_verdict_now: allow
    targets_now: []
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K03"
    command: "M=/Users/deepeshadmin/.claude/projects/-Users-deepeshadmin-Documents-GitHub-kinako/memory/feat-001-desk-visit-2026-09-19.md; sed -i '' 's|^\\*\\*Why:\\*\\* the next `/mochiko:implement FEAT-001` at delta scope reads \"the entry'\"'\"'s marked delta\" at$|**Why:** the next `/mochiko:implement FEAT-001` run reads \"the entry'\"'\"'s marked delta\" at its|' $M; sed -n 21,22p $M; echo \"=====\"; wc -l .mochiko/features/desk/2026-09-19-feat-001-r1-4-and-b69/derivation.md; grep -c \"RULED 2026-09-19, over this recommendation\" .mochiko/features/desk/2026-09-19-feat-001-r1-4-and-b69/derivation.md; git diff --stat | tail -1"
  - id: K04
    session: "b0eddd5e"
    thread: "agent-ae9f4bca01fed5d63"
    at: "2026-09-19T09:47:54.953Z"
    class: "false positive"
    shape: "b"
    denied_target_then: ".mochiko/features/FEAT-001/architecture.md"
    verdict_then: deny
    expected_verdict_now: allow
    targets_now: ["/private/tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-kinako/b0eddd5e-22e7-4744-b91d-9aac80f4e7c8/scratchpad/fix1.md"]
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K04"
    command: "cd /Users/deepeshadmin/Documents/GitHub/kinako; cp .mochiko/features/FEAT-001/architecture.md /private/tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-kinako/b0eddd5e-22e7-4744-b91d-9aac80f4e7c8/scratchpad/fix1.md; echo copied"
  - id: K05
    session: "b0eddd5e"
    thread: "agent-acf0d5e0b266b9873"
    at: "2026-09-19T10:07:58.756Z"
    class: "true positive"
    shape: "sed -i"
    denied_target_then: ".mochiko/features/FEAT-001/contracts/plugin-bridge.md"
    verdict_then: deny
    expected_verdict_now: deny
    targets_now: ["/Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/contracts/plugin-bridge.md"]
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K05"
    command: "cd /Users/deepeshadmin/Documents/GitHub/kinako\nsed -i '' 's/:818-820/:817-820/g' .mochiko/features/FEAT-001/contracts/plugin-bridge.md\nwc -l .mochiko/features/FEAT-001/contracts/plugin-bridge.md\ngrep -n \"817-820\" .mochiko/features/FEAT-001/contracts/plugin-bridge.md"
  - id: K06
    session: "b0eddd5e"
    thread: "agent-a3ef5ab702391c4bd"
    at: "2026-09-19T10:16:52.612Z"
    class: "true positive"
    shape: "heredoc cat >"
    denied_target_then: ".mochiko/features/FEAT-001/reports/design-feasibility-3-2026-09-19.md"
    verdict_then: deny
    expected_verdict_now: deny
    targets_now: [".mochiko/features/FEAT-001/reports/design-feasibility-3-2026-09-19.md"]
    trimmed: "heredoc body: 6 of 162 lines kept"
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K06"
    command: "cat > .mochiko/features/FEAT-001/reports/design-feasibility-3-2026-09-19.md <<'REPORT'\n---\nreport: feasibility\nfeature: FEAT-001\nREPORT\nwc -l .mochiko/features/FEAT-001/reports/design-feasibility-3-2026-09-19.md"
  - id: K07
    session: "b0eddd5e"
    thread: "main"
    at: "2026-09-19T10:20:21.452Z"
    class: "false positive"
    shape: "a"
    denied_target_then: ".mochiko/features/FEAT-001/plan.md"
    verdict_then: deny
    expected_verdict_now: allow
    targets_now: []
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K07"
    command: "wc -l .mochiko/features/FEAT-001/data-model.md .mochiko/features/FEAT-001/constraints-and-decisions.md .mochiko/features/FEAT-001/contracts/harness-store.md .mochiko/features/FEAT-001/contracts/plugin-bridge.md .mochiko/features/FEAT-001/plan.md .mochiko/features/FEAT-001/architecture.md .mochiko/product/quickstart.md | cat\necho \"=====M-1 plan.md:270=====\"; sed -n 270p .mochiko/features/FEAT-001/plan.md | cut -c1-120\necho \"=====M-3 anchors=====\"; grep -n \"settings.rs:806\" .mochiko/features/FEAT-001/constraints-and-decisions.md .mochiko/features/FEAT-001/contracts/harness-store.md | cut -c1-160\necho \"=====empty-id refusal in contract=====\"; grep -n -i \"empty\\|absent\" .mochiko/features/FEAT-001/contracts/harness-store.md | head -4 | cut -c1-160"
  - id: K08
    session: "b0eddd5e"
    thread: "agent-a972930afe2c76714"
    at: "2026-09-19T11:11:54.397Z"
    class: "true positive"
    shape: "heredoc cat >>"
    denied_target_then: ".mochiko/features/FEAT-001/contracts/plugin-bridge.md"
    verdict_then: deny
    expected_verdict_now: deny
    targets_now: [".mochiko/features/FEAT-001/contracts/plugin-bridge.md"]
    trimmed: "heredoc body: 6 of 78 lines kept"
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K08"
    command: "cat >> .mochiko/features/FEAT-001/contracts/plugin-bridge.md <<'MD'\n\n## Build-time delta (run 3) — 2026-09-19, cycle C1-11 (`B69`)\n\nMD\nwc -l .mochiko/features/FEAT-001/contracts/plugin-bridge.md"
  - id: K09
    session: "b0eddd5e"
    thread: "agent-a9230381dda9b5aca"
    at: "2026-09-19T11:41:28.805Z"
    class: "false positive"
    shape: "c"
    denied_target_then: ".mochiko/features/FEAT-001/reports/cycle-C1-3-verification.md;"
    verdict_then: deny
    expected_verdict_now: allow
    targets_now: ["/tmp/kinako-probe-settings-before.sha256"]
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K09"
    command: "shasum -a 256 ~/.claude/settings.json | tee /tmp/kinako-probe-settings-before.sha256; echo \"=== C1-3 :558-568 ===\"; sed -n '558,568p' .mochiko/features/FEAT-001/reports/cycle-C1-3-verification.md; echo \"=== C1-3 :320-326 (marker bootstrap) ===\"; sed -n '320,326p' .mochiko/features/FEAT-001/reports/cycle-C1-3-verification.md"
  - id: K10
    session: "b0eddd5e"
    thread: "agent-a9230381dda9b5aca"
    at: "2026-09-19T13:02:06.459Z"
    class: "true positive"
    shape: "sed -i"
    denied_target_then: ".mochiko/features/FEAT-001/tasks.md"
    verdict_then: deny
    expected_verdict_now: deny
    targets_now: [".mochiko/features/FEAT-001/tasks.md"]
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K10"
    command: "sed -i '' 's/^### - \\[ \\] Cycle C1-13: Confirm and delete/### - [x] Cycle C1-13: Confirm and delete/' .mochiko/features/FEAT-001/tasks.md && sed -n '1213p' .mochiko/features/FEAT-001/tasks.md"
  - id: K11
    session: "b0eddd5e"
    thread: "agent-a46aa72bfc56bad31"
    at: "2026-09-19T13:39:43.006Z"
    class: "true positive"
    shape: "heredoc cat >"
    denied_target_then: ".mochiko/features/FEAT-001/reports/blind-gap-finding-2026-09-19.md"
    verdict_then: deny
    expected_verdict_now: deny
    targets_now: ["/Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/reports/blind-gap-finding-2026-09-19.md"]
    trimmed: "heredoc body: 6 of 381 lines kept"
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K11"
    command: "cd /Users/deepeshadmin/Documents/GitHub/kinako && cat > .mochiko/features/FEAT-001/reports/blind-gap-finding-2026-09-19.md <<'REPORT'\n---\nreport: final-validation\nkind: gap-finding\nREPORT\nwc -l .mochiko/features/FEAT-001/reports/blind-gap-finding-2026-09-19.md"
  - id: K12
    session: "d87ac459"
    thread: "agent-a3cfd9d256ce0eb5d"
    at: "2026-09-21T23:27:28.705Z"
    class: "false positive"
    shape: "a"
    denied_target_then: ".mochiko/product/architecture/spine.md"
    verdict_then: deny
    expected_verdict_now: allow
    targets_now: []
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K12"
    command: "cd /Users/deepeshadmin/Documents/GitHub/kinako && sed -n '52,80p' .mochiko/product/architecture/spine.md | nl -ba -v52 | grep -i \"logs\\|cli\\|LOG\" "
  - id: K13
    session: "d87ac459"
    thread: "agent-a3590822ac1f65eeb"
    at: "2026-09-22T00:39:39.468Z"
    class: "false positive"
    shape: "a"
    denied_target_then: ".mochiko/features/FEAT-001/architecture.md"
    verdict_then: deny
    expected_verdict_now: allow
    targets_now: []
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K13"
    command: "echo \"=== FLOW-032 qualification sentence ===\" && sed -n '142,145p' .mochiko/features/FEAT-001/architecture.md && echo && echo \"=== does FLOW-032 draw any log participant? ===\" && sed -n '146,182p' .mochiko/features/FEAT-001/architecture.md | grep -n -i \"participant\\|logs\\|SPN-005\" && echo && echo \"=== amendment note count claim ===\" && sed -n '293,297p' .mochiko/features/FEAT-001/architecture.md | grep -o \"three count sentences\""
  - id: K14
    session: "d87ac459"
    thread: "agent-ababf31c25bc8e1f6"
    at: "2026-09-22T02:26:05.350Z"
    class: "false positive"
    shape: "a"
    denied_target_then: ".mochiko/product/data-model.md"
    verdict_then: deny
    expected_verdict_now: allow
    targets_now: []
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K14"
    command: "echo \"=== appended data-model block: headings + table lines ===\"; sed -n '1080,1225p' .mochiko/product/data-model.md | grep -n \"^#\\|^| \" | cut -c1-140 | head -40; echo; echo \"=== staged text's own note on rendering (grep 'rendering\\|prose\\|table' in data_model_fold region) ===\"; awk '/^data_model_fold:/,/^constraints_decisions_fold:/' .mochiko/features/FEAT-001/reports/landing-fold-texts-2026-09-22.md | grep -n -i \"rendering\\|title-framing\\|prose form\\|table form\" | cut -c1-220 | head -8"
  - id: K15
    session: "d87ac459"
    thread: "agent-ababf31c25bc8e1f6"
    at: "2026-09-22T02:40:07.974Z"
    class: "false positive"
    shape: "b"
    denied_target_then: "/Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/reports/landing-verification-2-2026-09-22.md"
    verdict_then: deny
    expected_verdict_now: allow
    targets_now: ["/private/tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-kinako/d87ac459-3911-45a6-86aa-0f974e0c1d67/scratchpad/lv2-draft.md"]
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K15"
    command: "cp /Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/reports/landing-verification-2-2026-09-22.md /private/tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-kinako/d87ac459-3911-45a6-86aa-0f974e0c1d67/scratchpad/lv2-draft.md && wc -l /private/tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-kinako/d87ac459-3911-45a6-86aa-0f974e0c1d67/scratchpad/lv2-draft.md"
  - id: K16
    session: "abdf4115"
    thread: "main"
    at: "2026-09-22T04:56:21.762Z"
    class: "false positive"
    shape: "a"
    denied_target_then: ".mochiko/features/FEAT-001/baseline-delta.md;"
    verdict_then: deny
    expected_verdict_now: allow
    targets_now: []
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K16"
    command: "sed -n '1,25p' .mochiko/features/FEAT-001/baseline-delta.md; echo '--- headings'; grep -n '^## ' .mochiko/features/FEAT-001/baseline-delta.md | head -30; echo '--- folded markers'; grep -n -i 'folded\\|landing' .mochiko/features/FEAT-001/baseline-delta.md | head -12; echo '--- git log'; git log --oneline -5 -- .mochiko/features/FEAT-001/baseline-delta.md"
  - id: K17
    session: "abdf4115"
    thread: "agent-a948756ab8624db7b"
    at: "2026-09-22T07:04:04.528Z"
    class: "false positive"
    shape: "b"
    denied_target_then: ".mochiko/features/FEAT-001-sparring-sessions.md"
    verdict_then: deny
    expected_verdict_now: allow
    targets_now: []
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K17"
    command: "D=/private/tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-kinako/abdf4115-2570-4b6e-af0c-c555d279af3c/scratchpad && mkdir -p $D && git show HEAD:.mochiko/features/FEAT-001-sparring-sessions.md > $D/pre.md && cp .mochiko/features/FEAT-001-sparring-sessions.md $D/post.md && cp .mochiko/archive/feat-001-entry-groom-2026-09-22.md $D/arch.md && wc -l $D/pre.md $D/post.md $D/arch.md"
  - id: K18
    session: "abdf4115"
    thread: "main"
    at: "2026-09-22T07:58:33.556Z"
    class: "false positive"
    shape: "b"
    denied_target_then: ".mochiko/features/FEAT-001/reports/run-log-run4-2026-09-22.md"
    verdict_then: deny
    expected_verdict_now: allow
    targets_now: []
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K18"
    command: "S=/private/tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-kinako/abdf4115-2570-4b6e-af0c-c555d279af3c/scratchpad; cp .mochiko/features/FEAT-001/reports/run-log-run4-2026-09-22.md $S/runlog-test.md && ruby -ryaml -e '\nf=File.read(ARGV[0]); fm=f.split(/^---\\s*$/)[1]\nbegin; d=YAML.safe_load(fm); puts \"CURRENT parses: report=#{d[\"report\"]} keys=#{d.keys.size}\"; rescue => e; puts \"CURRENT FAILS: #{e.message}\"; end\n' $S/runlog-test.md"
  - id: K19
    session: "abdf4115"
    thread: "agent-ab56f9524cbbd3dcf"
    at: "2026-09-22T22:36:43.736Z"
    class: "false positive"
    shape: "b"
    denied_target_then: ".mochiko/product/architecture/spine.md"
    verdict_then: deny
    expected_verdict_now: allow
    targets_now: []
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K19"
    command: "S=/private/tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-kinako/abdf4115-2570-4b6e-af0c-c555d279af3c/scratchpad && mkdir -p $S/g && git show 1788d97:.mochiko/product/architecture/spine.md > $S/g/pre.md && cp .mochiko/product/architecture/spine.md $S/g/post.md && cp .mochiko/archive/product-baselines/2026-09-22/spine-groom.md $S/g/arch.md && wc -l $S/g/*.md && echo \"=== PRE ## headings ===\" && grep -n '^## ' $S/g/pre.md && echo \"=== POST ## headings ===\" && grep -n '^## ' $S/g/post.md && echo \"=== ARCH headings ===\" && grep -n '^#\\{1,3\\} ' $S/g/arch.md"
  - id: K20
    session: "abdf4115"
    thread: "agent-a3b5f433879f54d42"
    at: "2026-09-22T23:46:16.599Z"
    class: "true positive"
    shape: "cp into home"
    denied_target_then: "/Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/reports/gap-rework-run4-1b-verification.md"
    verdict_then: deny
    expected_verdict_now: deny
    targets_now: ["/Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/reports/gap-rework-run4-1b-verification.md"]
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K20"
    command: "cp /tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-kinako/abdf4115-2570-4b6e-af0c-c555d279af3c/scratchpad/verif-1b/hookprobe-body.txt /Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/reports/gap-rework-run4-1b-verification.md && echo COPIED && wc -l /Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/reports/gap-rework-run4-1b-verification.md"
  - id: K21
    session: "abdf4115"
    thread: "agent-a4619d17d5a2db18b"
    at: "2026-09-23T00:12:58.237Z"
    class: "true positive"
    shape: "cp into home"
    denied_target_then: "/Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/reports/built-vs-signed-run4-2026-09-23.md"
    verdict_then: deny
    expected_verdict_now: deny
    targets_now: ["/Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/reports/built-vs-signed-run4-2026-09-23.md"]
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row K21"
    command: "cp /tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-kinako/abdf4115-2570-4b6e-af0c-c555d279af3c/scratchpad/r1.md /Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/reports/built-vs-signed-run4-2026-09-23.md && wc -l /Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/reports/built-vs-signed-run4-2026-09-23.md"
  - id: F01
    session: "cf3ef5ad"
    thread: "main"
    at: "2026-09-23T08:25:13.100Z"
    class: "false positive"
    shape: "a"
    denied_target_then: ".mochiko/memory/knowledge-management.md"
    verdict_then: deny
    expected_verdict_now: allow
    targets_now: []
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row F01"
    command: "cd /Users/deepeshadmin/Documents/GitHub/mochiko; grep -n -i -E 'important|high priority|priority|\\(high\\)|\\*\\*high' BACKLOG.md | head -20; sed -n '848,870p' BACKLOG.md; grep -n -i 'count\\|open items\\|bound' .mochiko/memory/knowledge-management.md | head -20"
  - id: F02
    session: "cf3ef5ad"
    thread: "main"
    at: "2026-09-23T08:32:28.609Z"
    class: "false positive"
    shape: "a"
    denied_target_then: ".mochiko/brainstorms/index.md;"
    verdict_then: deny
    expected_verdict_now: allow
    targets_now: []
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row F02"
    command: "cd /Users/deepeshadmin/Documents/GitHub/mochiko; wc -l .mochiko/brainstorms/index.md; sed -n '1,40p' .mochiko/brainstorms/index.md; grep -n -i 'hook\\|home\\|artifact-schema\\|location' .mochiko/brainstorms/index.md | head -30"
  - id: F03
    session: "cf3ef5ad"
    thread: "main"
    at: "2026-09-23T08:59:23.432Z"
    class: "false positive"
    shape: "a"
    denied_target_then: ".mochiko/features/FEAT-001/reports/x.md"
    verdict_then: deny
    expected_verdict_now: allow
    targets_now: []
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row F03"
    command: "cd /Users/deepeshadmin/Documents/GitHub/kinako; R=/Users/deepeshadmin/Documents/GitHub/mochiko/plugins/mochiko; mochiko-cli home .mochiko/features/FEAT-001/implement-log.md --plugin-root $R 2>&1 | sed -n '3,14p'; echo ---; mochiko-cli home .mochiko/features/FEAT-001/reports/x.md --plugin-root $R 2>&1 | grep -i 'report' | head -4"
  - id: L01
    session: "678e0789"
    thread: "main"
    at: "2026-09-28T21:46:04.621Z"
    class: "false positive"
    shape: "a"
    denied_target_then: ".mochiko/memory/governance-ledger.md;"
    verdict_then: deny
    expected_verdict_now: allow
    targets_now: []
    test: "tests/shell.rs every_census_command_returns_exactly_its_write_targets, row L01"
    command: "sed -n 276,312p .mochiko/memory/governance-ledger.md; echo ----; grep -n -i \"publish\" .claude/rules/mochiko/rust-cli.md | head -20"
---

## Notes of note

- The count holds: 79 real denies (F2), 21 shell (13 false, 8 true), plus 4 added. All 25 are rows of `tests/shell.rs`, each asserted on its exact targets.
- By mechanism, F3(c)'s `git show … > $S/pre.md` pair (K17, K19) was denied on the `cp` source, so it counts as shape (b); the `git show` spec keeps its own read row.
- Out of set, count only (Q4): 30 more real shell denies. 22 are in this repo's sessions `1bfb0a40` `1cc5644a` `af8c0bea` `7890a85e` `ea2dff98` `b6db47c7`, 1 is in kinako `af34e7c9`, 7 are in kinako `fix-sandbox-check` `5cde9fb6`.
- K02, K06, K08, K11: heredoc bodies trimmed to three lines; the redirect line, terminator and trailing `wc -l` are verbatim. The secrets scan found no credential, so nothing is redacted.
- Behaviour past §3.2's literal text, each lead-ruled: `-t`/`--target-directory` · zsh `>!`/`>>!` · `mv` every argument · BSD `sed -I` · sed and perl script words dropped · `git mv` as `mv` · reserved words and the wrapper set before the command word · unquoted `(`/`)` as separators (POSIX only; PowerShell tokenizes as before) · `$`, `{}` and `~` targets dropped · a heredoc body no longer lexed (closes the false allow where an apostrophe swallowed a later `cp`) · `\"` no longer flips quoting.
- Added at the code review's fix round (`w1-code-review.md`), each lead-ruled: a `-t` inside a flag cluster (`-rt <dir>`, `-vt<dir>`) · a process substitution `>(…)`/`<(…)` scanned as a command of its own · `((…))` and `$((…))` lexed as one word, so an arithmetic `>` or `<<` is never a redirect or a heredoc (this retires the `$((1<<2))` miss) · a word-initial `#` starts a comment · perl's `-l`/`-0` take digits only · nesting past depth 64 fails open without a panic.
- A `cd` is now followed (A3, lead-ruled, then refined): after `cd`/`pushd` with one absolute literal operand, later relative targets are joined to that directory, so K02, K05 and K11 name their home as an absolute path; after an expansion (`$X`, `${X}`, `~`), a relative target whose first segment is `.mochiko` is kept as written for the hook's cwd join and any other is dropped; after a relative operand, none, `-`, a flag or a glob, and after `popd`, every relative target is dropped; a `cd` inside `( … )` holds only to its `)`.
- Evasion 1 (`cd <home> && … > x.md`) closes for an absolute literal `cd` into a home and stays open for a relative or expanded operand. Evasion 2 (a path in a variable) stays open. Evasion 3 (a relative write from a cwd inside a home) closes in the hook's decision, by the cwd join, not here. Accepted false deny: a `cd $X` pointing outside the tree, then a `.mochiko/…` write, is joined to the payload's cwd all the same; no census row has that shape.
- A wrapper outside the named set (`watch`, `strace -o f`, `parallel`) hides the command word after it, so `watch tee <home>` names no target. The set is not widened; a test pins the miss.
- Two residual misses, the same before and after: a command substitution inside double quotes (`echo "$(tee <home>)"`) is one quoted word and is never split, and a backtick substitution (`` echo `tee <home>` ``) is not split at its backticks.
- Sweep: both scans run over all 48,284 `Bash`/`PowerShell` commands in the kinako and mochiko transcripts without a panic. Against the scan before the fix round, 261 commands keep their home targets, now joined to their `cd` directory; 27 gain one (26 by an absolute `cd` into a home, 1 by `perl -0pi`); none loses one. The 35 writes into homes after `cd $MAIN` (26), `cd $W` (8) and `cd $R` (1), which A3 before its refinement dropped, keep their targets unchanged.
