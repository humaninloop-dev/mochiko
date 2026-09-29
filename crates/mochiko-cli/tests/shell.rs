//! The shell leg's write-position parse: which paths a `Bash` or `PowerShell` command writes.
//!
//! The scan answers targets only; whether a target sits in a home, and the `runs/` carve, are the
//! hook's decision. So every assertion here is on the target list the scan returns.

use mochiko_cli::shell::{powershell_write_targets, write_targets};

const HOME: &str = ".mochiko/features/FEAT-001/gates.md";
const DIR: &str = ".mochiko/features/FEAT-001";

// ---------------------------------------------------------------------------
// the cases carried from the hook tests
// ---------------------------------------------------------------------------

// `tests/hook.rs` drives these through the whole decision. Here they pin the scan underneath, so
// the call-site swap in `hook.rs` cannot change a verdict those tests hold.

#[test]
fn every_carried_hook_write_still_names_its_home() {
    for command in [
        format!("printf 'x' > {HOME}"),
        format!("echo x >> {HOME}"),
        format!("echo x | tee {HOME}"),
        format!("sed -i '' 's/a/b/' {HOME}"),
        format!("cat <<'EOF' > {HOME}\nbody\nEOF"),
        format!("cp /tmp/other.md {HOME}"),
        format!("mv /tmp/other.md {HOME}"),
        format!("printf x >| {HOME}"),
        format!("printf x >|{HOME}"),
        format!("dd of={HOME} if=/dev/null"),
        format!("install -m 644 /tmp/other.md {HOME}"),
        format!("echo x | tee -a {HOME}"),
        format!("some-tool 2> {HOME}"),
        format!("some-tool &> {HOME}"),
        format!("printf 'retry probe' > \"/tmp/cwd/{HOME}\""),
        format!("mv {HOME} /tmp/elsewhere.md"),
    ] {
        let targets = write_targets(&command);
        assert!(
            targets.iter().any(|t| t.ends_with(HOME)),
            "a write into a home must name it: {command} — {targets:?}"
        );
    }
}

#[test]
fn every_carried_hook_read_names_no_home() {
    for command in [
        "git status --porcelain".to_string(),
        "cargo test -q".to_string(),
        format!("grep -rn foo {HOME}"),
        format!("cat {HOME}"),
        format!("ls -la {DIR}/"),
        "printf 'x' > /tmp/scratch.md".to_string(),
        format!("grep -rn foo {HOME} > /tmp/hits.txt"),
        // The vocabularies are keyed to the tool: a cmdlet name in a Bash line means nothing.
        format!("echo 'Set-Content -Path {HOME}'"),
    ] {
        let targets = write_targets(&command);
        assert!(
            !targets.iter().any(|t| t.contains(".mochiko")),
            "a read names no home: {command} — {targets:?}"
        );
    }
}

#[test]
fn every_carried_powershell_write_still_names_its_home() {
    for command in [
        format!("Set-Content -Path {HOME} -Value 'x'"),
        format!("Set-Content {HOME} 'x'"),
        format!("Out-File -FilePath {HOME}"),
        format!("'x' | Out-File {HOME}"),
        format!("Add-Content -Path {HOME} -Value 'x'"),
        format!("New-Item -Path {HOME} -ItemType File"),
        format!("Tee-Object -FilePath {HOME}"),
        format!("Copy-Item /tmp/other.md {HOME}"),
        format!("Move-Item /tmp/other.md -Destination {HOME}"),
        format!("Move-Item {HOME} /tmp/elsewhere.md"),
        format!("'x' > {HOME}"),
        format!("'x' >> {HOME}"),
        format!("set-content -path {HOME} -value 'x'"),
    ] {
        let targets = powershell_write_targets(&command);
        assert!(
            targets.iter().any(|t| t == HOME),
            "a PowerShell write into a home must name it: {command} — {targets:?}"
        );
    }
}

#[test]
fn every_carried_powershell_read_names_no_home() {
    for command in [
        format!("Get-Content {HOME}"),
        format!("Get-ChildItem {DIR}"),
        // A home path as a *value* is content, not a destination.
        format!("Set-Content -Path notes.txt -Value '{HOME}'"),
    ] {
        let targets = powershell_write_targets(&command);
        assert!(
            !targets.iter().any(|t| t.contains(".mochiko")),
            "this PowerShell command writes nothing into a home: {command} — {targets:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// the census (record F3, S12; `reports/w1-shell-census.md`)
// ---------------------------------------------------------------------------

/// One real denied command, verbatim, and the write targets it has now.
struct Case {
    id: &'static str,
    command: &'static str,
    targets: &'static [&'static str],
}

/// Every `Bash` call the gate denied in kinako implement runs 3 and 4 (`K`), the field review's own
/// hits (`F`), and the wave-1 lead's (`L`), extracted by script from the transcripts. The 17 false
/// positives name no home; the 8 true positives keep exactly theirs, three of them joined to the
/// directory their `cd` names. Heredoc bodies are trimmed to three lines: a body is data, so its
/// length cannot move a target.
const CENSUS: &[Case] = &[
    Case {
        id: "K01",
        command: r##"echo "=====plugin-bridge prompt/turn clauses====="; grep -n -i -E "every (completed )?turn|prompt half|UserPromptSubmit|leaderText|turn record|ordinal|slash command|bind exchange|B69" .mochiko/product/contracts/plugin-bridge.md | head -40; echo "=====contracts README====="; sed -n 1,40p .mochiko/product/contracts/README.md; echo "=====data-model Turn====="; grep -n -i -E "^## |^### |leaderText|kinakoText|ordinal" .mochiko/product/data-model.md | head -40"##,
        targets: &[],
    },
    // heredoc body trimmed: 7 of 40 lines kept
    Case {
        id: "K02",
        command: r##"cd /Users/deepeshadmin/Documents/GitHub/kinako
cat > .mochiko/features/desk/2026-09-19-feat-001-r1-4-and-b69/reports/review-3.md <<'REPORT'
---
report: review
feature: FEAT-001
REPORT
wc -l .mochiko/features/desk/2026-09-19-feat-001-r1-4-and-b69/reports/review-3.md"##,
        targets: &["/Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/desk/2026-09-19-feat-001-r1-4-and-b69/reports/review-3.md"],
    },
    Case {
        id: "K03",
        command: r##"M=/Users/deepeshadmin/.claude/projects/-Users-deepeshadmin-Documents-GitHub-kinako/memory/feat-001-desk-visit-2026-09-19.md; sed -i '' 's|^\*\*Why:\*\* the next `/mochiko:implement FEAT-001` at delta scope reads "the entry'"'"'s marked delta" at$|**Why:** the next `/mochiko:implement FEAT-001` run reads "the entry'"'"'s marked delta" at its|' $M; sed -n 21,22p $M; echo "====="; wc -l .mochiko/features/desk/2026-09-19-feat-001-r1-4-and-b69/derivation.md; grep -c "RULED 2026-09-19, over this recommendation" .mochiko/features/desk/2026-09-19-feat-001-r1-4-and-b69/derivation.md; git diff --stat | tail -1"##,
        targets: &[],
    },
    Case {
        id: "K04",
        command: r##"cd /Users/deepeshadmin/Documents/GitHub/kinako; cp .mochiko/features/FEAT-001/architecture.md /private/tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-kinako/b0eddd5e-22e7-4744-b91d-9aac80f4e7c8/scratchpad/fix1.md; echo copied"##,
        targets: &["/private/tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-kinako/b0eddd5e-22e7-4744-b91d-9aac80f4e7c8/scratchpad/fix1.md"],
    },
    Case {
        id: "K05",
        command: r##"cd /Users/deepeshadmin/Documents/GitHub/kinako
sed -i '' 's/:818-820/:817-820/g' .mochiko/features/FEAT-001/contracts/plugin-bridge.md
wc -l .mochiko/features/FEAT-001/contracts/plugin-bridge.md
grep -n "817-820" .mochiko/features/FEAT-001/contracts/plugin-bridge.md"##,
        targets: &["/Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/contracts/plugin-bridge.md"],
    },
    // heredoc body trimmed: 6 of 162 lines kept
    Case {
        id: "K06",
        command: r##"cat > .mochiko/features/FEAT-001/reports/design-feasibility-3-2026-09-19.md <<'REPORT'
---
report: feasibility
feature: FEAT-001
REPORT
wc -l .mochiko/features/FEAT-001/reports/design-feasibility-3-2026-09-19.md"##,
        targets: &[".mochiko/features/FEAT-001/reports/design-feasibility-3-2026-09-19.md"],
    },
    Case {
        id: "K07",
        command: r##"wc -l .mochiko/features/FEAT-001/data-model.md .mochiko/features/FEAT-001/constraints-and-decisions.md .mochiko/features/FEAT-001/contracts/harness-store.md .mochiko/features/FEAT-001/contracts/plugin-bridge.md .mochiko/features/FEAT-001/plan.md .mochiko/features/FEAT-001/architecture.md .mochiko/product/quickstart.md | cat
echo "=====M-1 plan.md:270====="; sed -n 270p .mochiko/features/FEAT-001/plan.md | cut -c1-120
echo "=====M-3 anchors====="; grep -n "settings.rs:806" .mochiko/features/FEAT-001/constraints-and-decisions.md .mochiko/features/FEAT-001/contracts/harness-store.md | cut -c1-160
echo "=====empty-id refusal in contract====="; grep -n -i "empty\|absent" .mochiko/features/FEAT-001/contracts/harness-store.md | head -4 | cut -c1-160"##,
        targets: &[],
    },
    // heredoc body trimmed: 6 of 78 lines kept
    Case {
        id: "K08",
        command: r##"cat >> .mochiko/features/FEAT-001/contracts/plugin-bridge.md <<'MD'

## Build-time delta (run 3) — 2026-09-19, cycle C1-11 (`B69`)

MD
wc -l .mochiko/features/FEAT-001/contracts/plugin-bridge.md"##,
        targets: &[".mochiko/features/FEAT-001/contracts/plugin-bridge.md"],
    },
    Case {
        id: "K09",
        command: r##"shasum -a 256 ~/.claude/settings.json | tee /tmp/kinako-probe-settings-before.sha256; echo "=== C1-3 :558-568 ==="; sed -n '558,568p' .mochiko/features/FEAT-001/reports/cycle-C1-3-verification.md; echo "=== C1-3 :320-326 (marker bootstrap) ==="; sed -n '320,326p' .mochiko/features/FEAT-001/reports/cycle-C1-3-verification.md"##,
        targets: &["/tmp/kinako-probe-settings-before.sha256"],
    },
    Case {
        id: "K10",
        command: r##"sed -i '' 's/^### - \[ \] Cycle C1-13: Confirm and delete/### - [x] Cycle C1-13: Confirm and delete/' .mochiko/features/FEAT-001/tasks.md && sed -n '1213p' .mochiko/features/FEAT-001/tasks.md"##,
        targets: &[".mochiko/features/FEAT-001/tasks.md"],
    },
    // heredoc body trimmed: 6 of 381 lines kept
    Case {
        id: "K11",
        command: r##"cd /Users/deepeshadmin/Documents/GitHub/kinako && cat > .mochiko/features/FEAT-001/reports/blind-gap-finding-2026-09-19.md <<'REPORT'
---
report: final-validation
kind: gap-finding
REPORT
wc -l .mochiko/features/FEAT-001/reports/blind-gap-finding-2026-09-19.md"##,
        targets: &["/Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/reports/blind-gap-finding-2026-09-19.md"],
    },
    Case {
        id: "K12",
        command: r##"cd /Users/deepeshadmin/Documents/GitHub/kinako && sed -n '52,80p' .mochiko/product/architecture/spine.md | nl -ba -v52 | grep -i "logs\|cli\|LOG" "##,
        targets: &[],
    },
    Case {
        id: "K13",
        command: r##"echo "=== FLOW-032 qualification sentence ===" && sed -n '142,145p' .mochiko/features/FEAT-001/architecture.md && echo && echo "=== does FLOW-032 draw any log participant? ===" && sed -n '146,182p' .mochiko/features/FEAT-001/architecture.md | grep -n -i "participant\|logs\|SPN-005" && echo && echo "=== amendment note count claim ===" && sed -n '293,297p' .mochiko/features/FEAT-001/architecture.md | grep -o "three count sentences""##,
        targets: &[],
    },
    Case {
        id: "K14",
        command: r##"echo "=== appended data-model block: headings + table lines ==="; sed -n '1080,1225p' .mochiko/product/data-model.md | grep -n "^#\|^| " | cut -c1-140 | head -40; echo; echo "=== staged text's own note on rendering (grep 'rendering\|prose\|table' in data_model_fold region) ==="; awk '/^data_model_fold:/,/^constraints_decisions_fold:/' .mochiko/features/FEAT-001/reports/landing-fold-texts-2026-09-22.md | grep -n -i "rendering\|title-framing\|prose form\|table form" | cut -c1-220 | head -8"##,
        targets: &[],
    },
    Case {
        id: "K15",
        command: r##"cp /Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/reports/landing-verification-2-2026-09-22.md /private/tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-kinako/d87ac459-3911-45a6-86aa-0f974e0c1d67/scratchpad/lv2-draft.md && wc -l /private/tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-kinako/d87ac459-3911-45a6-86aa-0f974e0c1d67/scratchpad/lv2-draft.md"##,
        targets: &["/private/tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-kinako/d87ac459-3911-45a6-86aa-0f974e0c1d67/scratchpad/lv2-draft.md"],
    },
    Case {
        id: "K16",
        command: r##"sed -n '1,25p' .mochiko/features/FEAT-001/baseline-delta.md; echo '--- headings'; grep -n '^## ' .mochiko/features/FEAT-001/baseline-delta.md | head -30; echo '--- folded markers'; grep -n -i 'folded\|landing' .mochiko/features/FEAT-001/baseline-delta.md | head -12; echo '--- git log'; git log --oneline -5 -- .mochiko/features/FEAT-001/baseline-delta.md"##,
        targets: &[],
    },
    Case {
        id: "K17",
        command: r##"D=/private/tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-kinako/abdf4115-2570-4b6e-af0c-c555d279af3c/scratchpad && mkdir -p $D && git show HEAD:.mochiko/features/FEAT-001-sparring-sessions.md > $D/pre.md && cp .mochiko/features/FEAT-001-sparring-sessions.md $D/post.md && cp .mochiko/archive/feat-001-entry-groom-2026-09-22.md $D/arch.md && wc -l $D/pre.md $D/post.md $D/arch.md"##,
        targets: &[],
    },
    Case {
        id: "K18",
        command: r##"S=/private/tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-kinako/abdf4115-2570-4b6e-af0c-c555d279af3c/scratchpad; cp .mochiko/features/FEAT-001/reports/run-log-run4-2026-09-22.md $S/runlog-test.md && ruby -ryaml -e '
f=File.read(ARGV[0]); fm=f.split(/^---\s*$/)[1]
begin; d=YAML.safe_load(fm); puts "CURRENT parses: report=#{d["report"]} keys=#{d.keys.size}"; rescue => e; puts "CURRENT FAILS: #{e.message}"; end
' $S/runlog-test.md"##,
        targets: &[],
    },
    Case {
        id: "K19",
        command: r##"S=/private/tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-kinako/abdf4115-2570-4b6e-af0c-c555d279af3c/scratchpad && mkdir -p $S/g && git show 1788d97:.mochiko/product/architecture/spine.md > $S/g/pre.md && cp .mochiko/product/architecture/spine.md $S/g/post.md && cp .mochiko/archive/product-baselines/2026-09-22/spine-groom.md $S/g/arch.md && wc -l $S/g/*.md && echo "=== PRE ## headings ===" && grep -n '^## ' $S/g/pre.md && echo "=== POST ## headings ===" && grep -n '^## ' $S/g/post.md && echo "=== ARCH headings ===" && grep -n '^#\{1,3\} ' $S/g/arch.md"##,
        targets: &[],
    },
    Case {
        id: "K20",
        command: r##"cp /tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-kinako/abdf4115-2570-4b6e-af0c-c555d279af3c/scratchpad/verif-1b/hookprobe-body.txt /Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/reports/gap-rework-run4-1b-verification.md && echo COPIED && wc -l /Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/reports/gap-rework-run4-1b-verification.md"##,
        targets: &["/Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/reports/gap-rework-run4-1b-verification.md"],
    },
    Case {
        id: "K21",
        command: r##"cp /tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-kinako/abdf4115-2570-4b6e-af0c-c555d279af3c/scratchpad/r1.md /Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/reports/built-vs-signed-run4-2026-09-23.md && wc -l /Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/reports/built-vs-signed-run4-2026-09-23.md"##,
        targets: &["/Users/deepeshadmin/Documents/GitHub/kinako/.mochiko/features/FEAT-001/reports/built-vs-signed-run4-2026-09-23.md"],
    },
    Case {
        id: "F01",
        command: r##"cd /Users/deepeshadmin/Documents/GitHub/mochiko; grep -n -i -E 'important|high priority|priority|\(high\)|\*\*high' BACKLOG.md | head -20; sed -n '848,870p' BACKLOG.md; grep -n -i 'count\|open items\|bound' .mochiko/memory/knowledge-management.md | head -20"##,
        targets: &[],
    },
    Case {
        id: "F02",
        command: r##"cd /Users/deepeshadmin/Documents/GitHub/mochiko; wc -l .mochiko/brainstorms/index.md; sed -n '1,40p' .mochiko/brainstorms/index.md; grep -n -i 'hook\|home\|artifact-schema\|location' .mochiko/brainstorms/index.md | head -30"##,
        targets: &[],
    },
    Case {
        id: "F03",
        command: r##"cd /Users/deepeshadmin/Documents/GitHub/kinako; R=/Users/deepeshadmin/Documents/GitHub/mochiko/plugins/mochiko; mochiko-cli home .mochiko/features/FEAT-001/implement-log.md --plugin-root $R 2>&1 | sed -n '3,14p'; echo ---; mochiko-cli home .mochiko/features/FEAT-001/reports/x.md --plugin-root $R 2>&1 | grep -i 'report' | head -4"##,
        targets: &[],
    },
    Case {
        id: "L01",
        command: r##"sed -n 276,312p .mochiko/memory/governance-ledger.md; echo ----; grep -n -i "publish" .claude/rules/mochiko/rust-cli.md | head -20"##,
        targets: &[],
    },
];

#[test]
fn every_census_command_returns_exactly_its_write_targets() {
    let failures: Vec<String> = CENSUS
        .iter()
        .filter_map(|case| {
            let got = write_targets(case.command);
            (got != case.targets).then(|| format!("{}: {got:?}, want {:?}", case.id, case.targets))
        })
        .collect();
    assert!(
        failures.is_empty(),
        "{} of {} census rows:\n{}",
        failures.len(),
        CENSUS.len(),
        failures.join("\n")
    );
}

#[test]
fn the_census_holds_twenty_five_rows_of_which_eight_write_into_a_home() {
    assert_eq!(CENSUS.len(), 25);
    let writes = CENSUS
        .iter()
        .filter(|case| case.targets.iter().any(|t| t.contains(".mochiko/")))
        .count();
    assert_eq!(writes, 8, "the table itself drifted from the census");
}

// ---------------------------------------------------------------------------
// the write positions (record D5 as amended at S12, V1/N2)
// ---------------------------------------------------------------------------

fn assert_targets(command: &str, want: &[&str]) {
    assert_eq!(write_targets(command), want, "{command}");
}

#[test]
fn a_redirect_names_its_target_in_every_spelling() {
    for command in [
        format!("printf x > {HOME}"),
        format!("printf x >> {HOME}"),
        format!("printf x >| {HOME}"),
        format!("printf x >|{HOME}"),
        // zsh's clobber spellings, which the user's shell reads as `>|` and `>>`.
        format!("printf x >! {HOME}"),
        format!("printf x >>! {HOME}"),
        format!("some-tool 2> {HOME}"),
        format!("some-tool 2>>{HOME}"),
        format!("some-tool &> {HOME}"),
        format!("some-tool &>> {HOME}"),
        format!("printf x>{HOME}"),
        format!("some-tool >&{HOME}"),
        format!("printf x > \"{HOME}\""),
        format!("cat <<'EOF' > {HOME}\nbody\nEOF"),
    ] {
        assert_targets(&command, &[HOME]);
    }
}

#[test]
fn a_descriptor_copy_or_an_input_redirect_names_no_target() {
    assert_targets("some-tool 2>&1 | tee /tmp/log", &["/tmp/log"]);
    for command in [
        "some-tool >&2".to_string(),
        "some-tool 2>&-".to_string(),
        format!("wc -l < {HOME}"),
        format!("grep x <<< {HOME}"),
        "cat <&3".to_string(),
    ] {
        assert_targets(&command, &[]);
    }
}

#[test]
fn tee_writes_every_argument_of_its_own_command() {
    assert_targets(&format!("echo x | tee -a {HOME} /tmp/b"), &[HOME, "/tmp/b"]);
    assert_targets(&format!("echo x | tee /tmp/a; cat {HOME}"), &["/tmp/a"]);
}

#[test]
fn cp_and_install_write_their_destination_and_read_their_source() {
    assert_targets(&format!("cp {HOME} /tmp/x"), &["/tmp/x"]);
    assert_targets(&format!("cp -r {DIR} /tmp/x"), &["/tmp/x"]);
    assert_targets(&format!("cp /tmp/a /tmp/b {DIR}"), &[DIR]);
    assert_targets(&format!("install -m 644 /tmp/a {HOME}"), &[HOME]);
}

#[test]
fn a_target_directory_flag_names_the_destination() {
    assert_targets(&format!("cp -t {DIR} /tmp/a /tmp/b"), &[DIR]);
    assert_targets(&format!("cp -t{DIR} /tmp/a"), &[DIR]);
    assert_targets(&format!("cp --target-directory={DIR} /tmp/a"), &[DIR]);
    assert_targets(&format!("install --target-directory {DIR} /tmp/a"), &[DIR]);
    assert_targets(&format!("mv -t {DIR} /tmp/a"), &[DIR, "/tmp/a"]);
}

#[test]
fn mv_writes_every_argument_because_a_source_is_a_removal() {
    assert_targets(
        &format!("mv /tmp/a /tmp/b {DIR}"),
        &["/tmp/a", "/tmp/b", DIR],
    );
    assert_targets(&format!("mv {HOME} /tmp/x"), &[HOME, "/tmp/x"]);
}

#[test]
fn git_mv_is_read_as_mv_and_every_other_git_command_as_a_read() {
    assert_targets(&format!("git mv {HOME} /tmp/x"), &[HOME, "/tmp/x"]);
    assert_targets(&format!("git -C /repo mv /tmp/a {HOME}"), &["/tmp/a", HOME]);
    assert_targets(
        &format!("git -c core.x=y mv /tmp/a {HOME}"),
        &["/tmp/a", HOME],
    );
    assert_targets(
        &format!("git show HEAD:{HOME} > /tmp/pre.md"),
        &["/tmp/pre.md"],
    );
    assert_targets(&format!("git log -- {HOME}"), &[]);
    assert_targets(&format!("git grep -n mv {HOME}"), &[]);
}

#[test]
fn sed_writes_its_file_operands_only_in_place_and_never_its_script() {
    for command in [
        format!("sed -i '' 's/a/b/' {HOME}"),
        format!("sed -i .bak 's/a/b/' {HOME}"),
        format!("sed -i.bak -e 's/a/b/' {HOME}"),
        format!("sed -i -f fix.sed {HOME}"),
        format!("sed -I '' 's/a/b/' {HOME}"),
        format!("sed -Ei 's/a/b/' {HOME}"),
        format!("sed --in-place 's/a/b/' {HOME}"),
        format!("sed --in-place=.bak --expression='s/a/b/' {HOME}"),
        // A word with a `/` is a file, never a backup suffix — even one that opens with `.`.
        format!("sed -e s/a/b/ -i {HOME}"),
        format!("sed -f fix.sed -i {HOME}"),
    ] {
        assert_targets(&command, &[HOME]);
    }
    assert_targets(&format!("sed -n 's/a/b/p' {HOME}"), &[]);
    assert_targets(&format!("sed 's/a/b/' {HOME} > /tmp/o"), &["/tmp/o"]);
}

#[test]
fn perl_writes_its_file_operands_only_in_place_and_never_its_program() {
    for command in [
        format!("perl -pi -e 's/a/b/' {HOME}"),
        format!("perl -i.bak -pe 's/a/b/' {HOME}"),
        format!("perl -i -pe 's/a/b/' {HOME}"),
        // With no `-e`, the first operand is the program file: a read.
        format!("perl -i -p fix.pl {HOME}"),
    ] {
        assert_targets(&command, &[HOME]);
    }
    assert_targets(&format!("perl -Mstrict -ne 'print' {HOME}"), &[]);
    assert_targets(&format!("perl -ne 'print' {HOME}"), &[]);
    assert_targets(&format!("perl fix.pl {HOME}"), &[]);
}

#[test]
fn dd_writes_only_the_of_operand_of_its_own_command() {
    assert_targets(&format!("dd if={HOME} of=/tmp/x"), &["/tmp/x"]);
    assert_targets(&format!("dd if=/dev/zero of={HOME} bs=1"), &[HOME]);
    assert_targets(&format!("dd if=/tmp/a; echo of={HOME}"), &[]);
}

#[test]
fn an_operand_of_a_read_is_never_a_target() {
    for command in [
        format!("grep -n foo {HOME}"),
        format!("sed -n 1p {HOME}"),
        format!("wc -l {HOME}"),
        format!("cat {HOME}"),
        format!("git show HEAD:{HOME}"),
    ] {
        assert_targets(&command, &[]);
    }
}

// ---------------------------------------------------------------------------
// the command word
// ---------------------------------------------------------------------------

#[test]
fn an_arm_fires_on_the_command_word_only() {
    for command in [
        format!("grep -n tee {HOME}"),
        format!("grep -c cp {HOME}"),
        format!("echo mv {HOME}"),
        format!("grep -rn 'mv' {HOME}"),
        format!("echo install {HOME}"),
        format!("grep -n sed -i {HOME}"),
        format!("sudo grep -n tee {HOME}"),
        format!("find {DIR} -name '*.md' -exec grep -n cp {{}} \\;"),
    ] {
        assert_targets(&command, &[]);
    }
}

#[test]
fn a_wrapper_hands_the_command_word_on() {
    for command in [
        format!("sudo tee {HOME}"),
        format!("sudo -u root tee -a {HOME}"),
        format!("doas -u root tee {HOME}"),
        format!("env FOO=1 tee {HOME}"),
        format!("env -i tee {HOME}"),
        format!("FOO=1 BAR=2 tee {HOME}"),
        format!("command cp /tmp/a {HOME}"),
        format!("builtin command cp /tmp/a {HOME}"),
        format!("exec tee {HOME}"),
        format!("exec -a name tee {HOME}"),
        format!("nohup cp /tmp/a {HOME}"),
        format!("time cp /tmp/a {HOME}"),
        format!("time -p cp /tmp/a {HOME}"),
        format!("timeout 5 tee {HOME}"),
        format!("timeout -s KILL 10s cp /tmp/a {HOME}"),
        format!("timeout --signal=KILL 1m tee {HOME}"),
        format!("nice tee {HOME}"),
        format!("nice -n 10 tee {HOME}"),
        format!("stdbuf -oL tee {HOME}"),
        format!("stdbuf -o L tee {HOME}"),
        format!("find . -exec sed -i s/a/b/ {HOME} \\;"),
        format!("find . -name x -exec sudo tee {HOME} ';'"),
    ] {
        assert_targets(&command, &[HOME]);
    }
    for command in [
        format!("xargs cp -t {DIR}"),
        format!("ls | xargs -I {{}} cp {{}} {DIR}"),
        format!("ls | xargs -n1 -I{{}} mv {{}} {DIR}"),
        format!("find . -execdir mv {{}} {DIR} +"),
    ] {
        assert_targets(&command, &[DIR]);
    }
}

#[test]
fn a_reserved_word_is_skipped_before_the_command_word() {
    for command in [
        format!("for f in a b; do cp $f {HOME}; done"),
        format!("{{ tee {HOME}; }}"),
        format!("if true; then sed -i '' s/a/b/ {HOME}; fi"),
        format!("while read l; do echo $l | tee -a {HOME}; done"),
        format!("! tee {HOME}"),
    ] {
        assert_targets(&command, &[HOME]);
    }
    assert_targets(&format!("if grep -q x {HOME}; then echo ok; fi"), &[]);
}

// ---------------------------------------------------------------------------
// the tokenizer
// ---------------------------------------------------------------------------

#[test]
fn a_separator_glued_to_a_word_ends_the_command() {
    // Record S12: the shape behind 10 of the census's false positives.
    for command in [
        format!("sed -n 1p {HOME}; grep -i x y"),
        format!("sed -n 1p {HOME};grep -i x y"),
        format!("sed -n 1p {HOME}|grep -i x"),
        format!("sed -n 1p {HOME}&&grep -i x y"),
        format!("sed -n 1p {HOME}||grep -i x y"),
        format!("sed -n 1p {HOME}&grep -i x y"),
        format!("sed -n 1p {HOME}\ngrep -i x y"),
    ] {
        assert_targets(&command, &[]);
    }
    assert_targets(
        &format!("echo x | tee /tmp/a; sed -n 1p {HOME}"),
        &["/tmp/a"],
    );
}

#[test]
fn a_quoted_operator_is_a_word() {
    assert_targets(&format!("echo '>' {HOME}"), &[]);
    assert_targets(&format!("grep '|' {HOME}"), &[]);
    assert_targets(&format!("echo ';' ; tee {HOME}"), &[HOME]);
    assert_targets("echo \"a && b\" > /tmp/x", &["/tmp/x"]);
}

#[test]
fn a_parenthesis_is_an_operator_outside_quotes() {
    for command in [
        format!("(tee {HOME})"),
        format!("( tee {HOME} )"),
        format!("x=$(tee {HOME})"),
        format!("echo $(sed -i '' s/a/b/ {HOME})"),
    ] {
        assert_targets(&command, &[HOME]);
    }
    assert_targets(
        &format!("(cd /tmp && cp a {HOME})"),
        &[&format!("/tmp/{HOME}")],
    );
    assert_targets(&format!("grep -E '(a|b)' {HOME}"), &[]);
    assert_targets(&format!("echo \"$(cat {HOME})\""), &[]);
    assert_targets(&format!("x=$(cat {HOME}); echo $x"), &[]);
}

#[test]
fn a_heredoc_body_is_data_not_commands() {
    // The apostrophe used to open a quote that swallowed the `cp` after the terminator: a false
    // allow. A body is skipped whole, so neither its quotes nor its words reach the scan.
    assert_targets(
        &format!("cat > /tmp/a <<'EOF'\nit's\ncp /tmp/z {HOME}\nEOF\ncp /tmp/a {HOME}"),
        &["/tmp/a", HOME],
    );
    assert_targets(&format!("cat <<EOF > /tmp/a\n> {HOME}\nEOF"), &["/tmp/a"]);
    assert_targets(
        &format!("cat <<-EOF > /tmp/a\n\ttee {HOME}\n\tEOF\necho done"),
        &["/tmp/a"],
    );
    assert_targets(
        &format!("cat <<A <<\"B\" > /tmp/a\nx\nA\ny\nB\ntee {HOME}"),
        &["/tmp/a", HOME],
    );
}

#[test]
fn a_backslash_escapes_a_character_or_continues_a_line() {
    assert_targets(&format!("cp /tmp/a \\\n {HOME}"), &[HOME]);
    assert_targets(&format!("echo \"say \\\"hi\" > {HOME}"), &[HOME]);
    assert_targets("echo a\\;b > /tmp/x", &["/tmp/x"]);
    assert_targets("echo x > /tmp/my\\ file", &["/tmp/my file"]);
}

#[test]
fn an_expansion_the_scan_cannot_resolve_is_dropped() {
    assert_targets("echo x > $OUT", &[]);
    assert_targets("cp /tmp/a ${DIR}/x", &[]);
    assert_targets("echo x | tee ~/notes.md", &[]);
    assert_targets("find . -exec cp {} /tmp/x \\;", &["/tmp/x"]);
    assert_targets(&format!("find . -exec mv {{}} {HOME} \\;"), &[HOME]);
}

#[test]
fn disclosed_evasions_return_literal_targets() {
    // 1: a `cd` into a home before a relative write. The scan follows a `cd` only to an absolute
    // literal directory; after a relative one, like this, a relative target is dropped.
    assert_targets(&format!("cd {DIR} && echo x > gates.md"), &[]);
    // 2: a path held in a variable is an expansion, dropped.
    assert_targets(&format!("f={HOME}; echo x > $f"), &[]);
    // 3: a relative write from a working directory inside a home. The literal target is all the
    // scan can return; the hook joins it to the payload's `cwd`, which closes this one.
    assert_targets("echo x > gates.md", &["gates.md"]);
    // The residue: a substitution inside double quotes is one quoted word, never split, and a
    // backtick substitution is not split at its backticks, quoted or not.
    assert_targets(&format!("echo \"$(tee {HOME})\""), &[]);
    assert_targets(&format!("echo `tee {HOME}`"), &[]);
    assert_targets(&format!("echo \"`tee {HOME}`\""), &[]);
}

// ---------------------------------------------------------------------------
// PowerShell
// ---------------------------------------------------------------------------

#[test]
fn a_powershell_separator_glued_to_a_word_ends_the_cmdlet_run() {
    assert_eq!(
        powershell_write_targets(&format!(
            "Set-Content -Path notes.txt -Value x; Get-Content {HOME}"
        )),
        ["notes.txt"]
    );
    assert_eq!(
        powershell_write_targets(&format!(
            "Set-Content -Path notes.txt -Value x;Copy-Item /tmp/a {HOME}"
        )),
        ["notes.txt", "/tmp/a", HOME]
    );
}

#[test]
fn a_powershell_path_keeps_its_backslashes_and_its_parentheses() {
    let windows = r"C:\repo\.mochiko\features\FEAT-001\gates.md";
    assert_eq!(
        powershell_write_targets(&format!("Set-Content -Path {windows} -Value x")),
        [windows]
    );
    // A parenthesis is a sub-expression here, not a separator: the run reaches `-Path`.
    assert_eq!(
        powershell_write_targets(&format!("Set-Content -Value (Get-Date) -Path {HOME}")),
        [HOME]
    );
    assert_eq!(
        powershell_write_targets(&format!("Get-Content {HOME} 2>&1 > /tmp/log")),
        ["/tmp/log"]
    );
    assert!(powershell_write_targets("Set-Content -Path $p -Value x").is_empty());
}

// ---------------------------------------------------------------------------
// the code review's fix round (`reports/w1-code-review.md`)
// ---------------------------------------------------------------------------

#[test]
fn a_target_directory_flag_inside_a_cluster_names_the_destination() {
    // B1: the old scan denied these through its first-argument rule; a cluster must not lose them.
    assert_targets(&format!("cp -rt {DIR} a b"), &[DIR]);
    assert_targets(&format!("cp -vt{DIR} a"), &[DIR]);
    assert_targets(&format!("cp -at {DIR} a"), &[DIR]);
    assert_targets(&format!("install -Dt {DIR} a"), &[DIR]);
    assert_targets(&format!("mv -vt {DIR} /tmp/a"), &[DIR, "/tmp/a"]);
    // A flag that takes a value ends the cluster scan, and its value is never the directory.
    assert_targets(&format!("install -m 644 -t {DIR} a"), &[DIR]);
    assert_targets(&format!("install -mt {HOME}"), &[HOME]);
}

#[test]
fn a_process_substitution_is_a_command_of_its_own() {
    // B2(b): the group used to end `tee`'s run, so the home after it was lost.
    assert_targets(&format!("echo x | tee >(cat) {HOME}"), &[HOME]);
    assert_targets(
        &format!("echo x | tee >(cat > /tmp/copy) {HOME}"),
        &[HOME, "/tmp/copy"],
    );
    assert_targets(&format!("cat <(tee {HOME})"), &[HOME]);
    assert_targets(&format!("diff <(sort {HOME}) x"), &[]);
    assert_targets(
        &format!("diff <(sort {HOME}) <(sort x) > /tmp/d"),
        &["/tmp/d"],
    );
}

#[test]
fn a_wrapper_outside_the_named_set_hides_its_command_word() {
    // B2(a): recorded as a class, the set not widened. The pin makes a later change visible.
    assert_targets(&format!("watch tee {HOME}"), &[]);
}

#[test]
fn an_arithmetic_expression_is_one_word() {
    // Its `>` is a comparison and its `<<` a shift, never a redirect or a heredoc.
    assert_targets("(( 3 > 2 ))", &[]);
    assert_targets("x=$(( 1 > 0 )); echo $x", &[]);
    assert_targets(&format!("echo $((1<<2))\ntee {HOME}"), &[HOME]);
    assert_targets(&format!("echo $((1<<2)) > {HOME}"), &[HOME]);
}

#[test]
fn a_double_paren_that_closes_apart_is_two_subshells() {
    // G1 R2: bash and zsh read `((` as arithmetic only when the `)` closing its inner `(` is
    // followed at once by the outer `)`; otherwise it is a subshell inside a subshell.
    assert_targets(&format!("((echo a); tee {HOME})"), &[HOME]);
    assert_targets(&format!("((echo a) > {HOME})"), &[HOME]);
    assert_targets(&format!("$((echo a); tee {HOME})"), &[HOME]);
    // An inner `(` that never closes is still one word: bash refuses the line.
    assert_targets("(( 3 > 2", &[]);
    // The lookahead budget: 64 non-arithmetic answers per scan. Within it, arithmetic stays one
    // word; past it, `((` reads as two subshells, so a write is never missed (fail-closed) and a
    // comparison may be read as a redirect.
    assert_targets(&("((a); ".repeat(63) + "(( 3 > 2 ))"), &[]);
    assert_targets(&("((a); ".repeat(64) + "(( 3 > 2 ))"), &["2"]);
    assert_targets(
        &("((a); ".repeat(65) + &format!("((echo a); tee {HOME})")),
        &[HOME],
    );
    assert_targets(
        &("(".repeat(20_000) + &format!("tee {HOME}") + &";)".repeat(20_000)),
        &[HOME],
    );
    // G3 B1: a `((` read as two subshells exposes a `<<` shift, which the scan takes for a heredoc
    // that swallows every later line. Past the budget, or where a `case` pattern's `)` misleads the
    // close, that hid a write the reading before R2 saw. Collected first, so every row reports.
    let later = format!("\necho hi > {HOME}");
    let hidden: Vec<String> = [
        "((true); true); ".repeat(64) + "(( x = 1 << 2 ))",
        "(( x = $(case a in a) echo 1;; esac) << 2 ))".to_string(),
        "(( x = `case a in a) echo 1;; esac` << 2 ))".to_string(),
        "((true); true); ".repeat(63) + "(( x = 1 << 2 ))",
    ]
    .into_iter()
    .map(|shape| shape + &later)
    .filter(|command| write_targets(command) != [HOME])
    .collect();
    assert!(hidden.is_empty(), "a later write is hidden: {hidden:#?}");
}

#[test]
fn a_run_of_openers_past_the_lookahead_budget_stays_linear() {
    // Past the budget no `((` is looked ahead, so this 40,000-character run costs 64 scans, not
    // 20,000; it names no target under either reading.
    assert_targets(&("(".repeat(20_000) + "a" + &";)".repeat(20_000)), &[]);
}

#[test]
fn perl_l_and_0_take_digits_only() {
    assert_targets(&format!("perl -lpi -e s/a/b/ {HOME}"), &[HOME]);
    assert_targets(&format!("perl -0777pi -e s/a/b/ {HOME}"), &[HOME]);
    assert_targets(&format!("perl -l -ne print {HOME}"), &[]);
}

#[test]
fn a_nesting_past_the_depth_cap_fails_open_without_a_panic() {
    assert_targets(
        &("find . -exec ".repeat(3) + &format!("tee {HOME}")),
        &[HOME],
    );
    assert_targets(
        &("find . -exec ".repeat(20_000) + &format!("tee {HOME}")),
        &[],
    );
    assert_targets(&("tee >(".repeat(20_000) + HOME), &[]);
}

#[test]
fn a_comment_ends_the_line() {
    // An apostrophe in a comment used to open a quote that swallowed every later line.
    assert_targets(&format!("# don't\ntee {HOME}"), &[HOME]);
    assert_targets(&format!("echo x # > {HOME}"), &[]);
    assert_targets(&format!("echo x;# > {HOME}"), &[]);
    // A `#` inside a word is literal.
    assert_targets("echo a#b > /tmp/x", &["/tmp/x"]);
}

#[test]
fn a_cd_to_an_absolute_literal_directory_joins_the_relative_targets_after_it() {
    // A3, ruled (ii) with paren scoping: evasion 1 closes for an absolute literal `cd`.
    assert_targets("cd /tmp && echo x > y.log", &["/tmp/y.log"]);
    assert_targets(
        &format!("cd /abs/repo/{DIR} && echo > x.md"),
        &["/abs/repo/.mochiko/features/FEAT-001/x.md"],
    );
    assert_targets("pushd /tmp && tee a", &["/tmp/a"]);
    assert_targets("builtin cd /tmp/ && echo > y", &["/tmp/y"]);
    assert_targets("cd / && echo > y", &["/y"]);
    // An absolute target is untouched, and a later absolute `cd` moves the join.
    assert_targets("cd /tmp && cp a /var/b", &["/var/b"]);
    assert_targets(
        "cd /tmp; echo > a; cd /var; echo > b",
        &["/tmp/a", "/var/b"],
    );
    // A `cd` that is not the command word changes nothing.
    assert_targets("echo cd /tmp; echo > y", &["y"]);
}

#[test]
fn a_cd_the_scan_cannot_name_drops_the_relative_targets_after_it() {
    assert_targets(&format!("cd {DIR} && echo > x.md"), &[]);
    assert_targets("cd $D && echo > x.md", &[]);
    assert_targets("cd && echo > x.md", &[]);
    assert_targets("cd - && echo > x.md", &[]);
    assert_targets("cd -P /tmp && echo > x.md", &[]);
    assert_targets("cd /tmp/* && echo > x.md", &[]);
    assert_targets("cd ~/x && echo > x.md", &[]);
    assert_targets("pushd /tmp && popd && echo > x.md", &[]);
    // An absolute target still stands, and an absolute `cd` after restores the join.
    assert_targets("cd $D && echo > /tmp/x", &["/tmp/x"]);
    assert_targets("cd $D; cd /tmp; echo > y", &["/tmp/y"]);
}

#[test]
fn a_cd_to_an_expansion_keeps_only_the_targets_aimed_at_mochiko() {
    // A3 as refined: the field shape aims a `.mochiko/…` path at the repository the variable names,
    // so the target is kept as written for the hook to join; any other relative target is dropped.
    assert_targets(
        "MAIN=/r; cd $MAIN && cat >> .mochiko/epics/EPIC-001/implement-log.md",
        &[".mochiko/epics/EPIC-001/implement-log.md"],
    );
    assert_targets("cd $X && echo x > y.log", &[]);
    assert_targets("cd \"${W}\" && tee .mochiko/x.md y.md", &[".mochiko/x.md"]);
    assert_targets("pushd ~ && echo > .mochiko/x.md", &[".mochiko/x.md"]);
    assert_targets("cd ~/x && echo > .mochiko/a.md", &[".mochiko/a.md"]);
    assert_targets("cd $X && echo > ./.mochiko/x.md", &["./.mochiko/x.md"]);
    // The first segment must be `.mochiko` itself.
    assert_targets("cd $X && echo > sub/.mochiko/x.md", &[]);
    assert_targets("cd $X && echo > .mochikox/x.md", &[]);
    // A relative literal, a flag or a glob still drops every relative target.
    assert_targets("cd sub && echo > .mochiko/x.md", &[]);
    assert_targets("cd -P $X && echo > .mochiko/x.md", &[]);
    assert_targets("cd $X/* && echo > .mochiko/x.md", &[]);
    // A later relative `cd` drops them again, and a group restores what was before it.
    assert_targets("cd $X; cd sub; echo > .mochiko/x.md", &[]);
    assert_targets(
        "(cd $X && echo > .mochiko/a.md); echo > b",
        &[".mochiko/a.md", "b"],
    );
}

#[test]
fn a_cd_inside_parentheses_holds_only_to_its_close() {
    assert_targets("(cd /abs/.mochiko/x && ls); echo > y.log", &["y.log"]);
    assert_targets(
        "cd /tmp; (cd /var && echo > a); echo > b",
        &["/var/a", "/tmp/b"],
    );
    assert_targets("(cd $D); echo > y.log", &["y.log"]);
    assert_targets("x=$(cd /var && pwd); echo > y", &["y"]);
    // A process substitution is a group of its own too.
    assert_targets("cd /tmp; tee >(cd /var; cat > a) b", &["/tmp/b", "/var/a"]);
}
