//! The hook payload, the shell write-operator parse, and the decision JSON the wrapper prints.
//!
//! The wrapper is a pipe: it hands the raw `PreToolUse` payload to the binary on stdin and prints
//! what comes back. So the parse and the emit are the binary's, and the shell script holds no rule
//! — the same posture as `dependency-halt.sh`.

use mochiko_cli::hook::{self, Outcome};
use mochiko_cli::migration;
use mochiko_cli::replay::{self, State};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// A scratch git tree: a directory holding its own `.git`, so its `.mochiko/` is a home tree.
///
/// `target/` sits inside this repository, so a scratch directory without its own `.git` would
/// resolve against this repository's root and its `.mochiko/` would be a nested, never-home tree
/// (field review D7 as amended).
fn scratch(tag: &str) -> PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("hook-{tag}-{n}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join(".git")).expect("scratch tree is creatable");
    dir
}

/// A linked worktree of `main` at `main/.claude/worktrees/<name>`, carrying the `.git` pointer file
/// git writes: `gitdir: <main>/.git/worktrees/<name>`.
fn worktree(main: &Path, name: &str) -> PathBuf {
    let dir = main.join(".claude/worktrees").join(name);
    std::fs::create_dir_all(&dir).expect("worktree dir is creatable");
    let gitdir = main.join(".git/worktrees").join(name);
    std::fs::create_dir_all(&gitdir).expect("worktree gitdir is creatable");
    std::fs::write(dir.join(".git"), format!("gitdir: {}\n", gitdir.display()))
        .expect("pointer file is writable");
    dir
}

fn write_payload(cwd: &str, file_path: &str, content: &str) -> String {
    let content = content
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n");
    format!(
        r##"{{"cwd":"{cwd}","tool_name":"Write","tool_input":{{"file_path":"{file_path}","content":"{content}"}}}}"##
    )
}

const LOG: &str = r#"
grammar: 2
id: 0001-hook
sequence: 1
intent: One home, the raw-output home, and a template, for the hook surface.
changes:
  - op: import-document
    kind: home
    name: runs
    content:
      home: runs
      title: The run folder
      path: [".mochiko", "runs", "<run-id>"]
      raw_output: true
      bounds: elsewhere
      bounds_cite: hook-enforcement-field-review D4 (raw output, shape and size unchecked)
      deliverables:
        - file: implement-log.md
          bound_reason: ephemeral run log, deleted at acceptance
  - op: import-document
    kind: home
    name: feature
    content:
      home: feature
      title: Feature work home
      path: [".mochiko", "features", "<FEAT-ID>"]
      bounds: whole-file
      deliverables:
        - file: gates.md
          max_lines: 6
      subdirs: ["prototype"]
      reports:
        envelope: report-envelope
  - op: import-document
    kind: template
    name: report-envelope
    content:
      template: report-envelope
      title: Report envelope
      form: report-format.md
      register: full
      overview: The envelope every report opens with.
      conformance:
        frontmatter:
          required: [report]
          enum:
            report: [cycle, review]
      sections: []
      skeleton: |
        ---
        report: cycle
        ---
"#;

fn state(tag: &str) -> (State, PathBuf) {
    let dir = scratch(tag);
    let log = dir.join("log");
    std::fs::create_dir_all(&log).expect("log dir");
    let stamped = migration::with_hash("0001-hook.yaml", LOG).expect("well-formed fixture");
    std::fs::write(log.join("0001-hook.yaml"), stamped).expect("fixture is writable");
    let state = replay::load(&log).unwrap_or_else(|findings| {
        let lines: Vec<String> = findings.iter().map(ToString::to_string).collect();
        panic!("fixture log replays:\n{}", lines.join("\n"))
    });
    (state, dir)
}

fn decide(state: &State, json: &str) -> Outcome {
    let payload = hook::parse(json).expect("the payload parses");
    hook::decide(state, &payload)
}

fn is_deny(outcome: &Outcome) -> bool {
    matches!(outcome, Outcome::Deny { .. })
}

fn is_allow(outcome: &Outcome) -> bool {
    matches!(outcome, Outcome::Allow { .. })
}

fn reason(outcome: &Outcome) -> String {
    match outcome {
        Outcome::Deny { reason } => reason.clone(),
        Outcome::Allow { .. } => String::new(),
    }
}

// ---------------------------------------------------------------------------
// the payload
// ---------------------------------------------------------------------------

#[test]
fn a_write_payload_with_multi_line_escaped_content_parses() {
    let json = r##"{"session_id":"s","cwd":"/repo","hook_event_name":"PreToolUse",
        "tool_name":"Write","tool_input":{"file_path":"/repo/a.md",
        "content":"# T\n\n## S\n\nsay \"hi\" and C:\\path\n"}}"##;
    let payload = hook::parse(json).expect("the payload parses");
    assert_eq!(payload.tool_name.as_deref(), Some("Write"));
    assert_eq!(payload.cwd.as_deref(), Some("/repo"));
    assert_eq!(
        payload.tool_input.content.as_deref(),
        Some("# T\n\n## S\n\nsay \"hi\" and C:\\path\n"),
        "the shell wrapper's grep/sed field() cannot carry this, which is why the binary parses"
    );
}

#[test]
fn an_escaped_forward_slash_in_a_path_parses_to_the_plain_path() {
    // Legal JSON that plain YAML rejects. The probe at wave 1 confirmed the parser handles it.
    let json = r##"{"tool_name":"Write","tool_input":{"file_path":"\/repo\/a.md"}}"##;
    let payload = hook::parse(json).expect("the payload parses");
    assert_eq!(payload.tool_input.file_path.as_deref(), Some("/repo/a.md"));
}

#[test]
fn unknown_payload_fields_are_ignored_rather_than_rejected() {
    let json = r##"{"permission_mode":"bypassPermissions","agent_id":"a1","agent_type":"x",
        "tool_name":"Bash","tool_input":{"command":"ls","description":"d"}}"##;
    let payload = hook::parse(json).expect("a payload with extra fields still parses");
    assert_eq!(payload.tool_name.as_deref(), Some("Bash"));
}

#[test]
fn a_payload_that_is_not_json_does_not_parse() {
    assert!(hook::parse("not json at all {").is_none());
    assert!(hook::parse("").is_none());
}

// ---------------------------------------------------------------------------
// the explicit allow (wave-0 fold)
// ---------------------------------------------------------------------------

#[test]
fn every_non_deny_outcome_renders_an_explicit_allow_decision() {
    let (state, dir) = state("explicit-allow");
    let cwd = dir.display().to_string();
    let json = format!(
        r##"{{"cwd":"{cwd}","tool_name":"Write","tool_input":{{"file_path":"{cwd}/docs/x.md","content":"# x\n"}}}}"##
    );
    let outcome = decide(&state, &json);
    assert!(is_allow(&outcome), "a path outside every home is allowed");
    let rendered = hook::render(&outcome);
    assert!(
        rendered.contains(r#""permissionDecision":"allow""#),
        "the platform denies a background subagent's call when no hook returns a decision, so an \
         allow is always explicit: {rendered}"
    );
    assert!(rendered.contains(r#""hookEventName":"PreToolUse""#));
}

#[test]
fn a_deny_renders_its_reason_and_a_conformance_exit_code_of_four() {
    let (state, dir) = state("deny-render");
    let cwd = dir.display().to_string();
    let json = format!(
        r##"{{"cwd":"{cwd}","tool_name":"Write","tool_input":{{"file_path":"{cwd}/.mochiko/features/FEAT-001/invented.md","content":"# x\n"}}}}"##
    );
    let outcome = decide(&state, &json);
    assert!(is_deny(&outcome));
    let rendered = hook::render(&outcome);
    assert!(rendered.contains(r#""permissionDecision":"deny""#));
    assert!(rendered.contains("invented.md"));
    assert_eq!(hook::EXIT_CONFORMANCE, 4);
    assert_eq!(outcome.exit_code(), 4);
}

#[test]
fn an_amnestied_allow_carries_its_standing_overage_as_additional_context() {
    let (state, dir) = state("amnesty-context");
    let cwd = dir.display().to_string();
    let file = dir.join(".mochiko/features/FEAT-001/gates.md");
    std::fs::create_dir_all(file.parent().expect("parent")).expect("dirs");
    let over = "a\nb\nc\nd\ne\nf\ng\nh\n";
    std::fs::write(&file, over).expect("baseline is writable");
    let json = format!(
        r##"{{"cwd":"{cwd}","tool_name":"Write","tool_input":{{"file_path":"{}","content":"a\nb\nc\nd\ne\nf\ng\n"}}}}"##,
        file.display()
    );
    let outcome = decide(&state, &json);
    assert!(
        is_allow(&outcome),
        "improving a standing overage is allowed"
    );
    let rendered = hook::render(&outcome);
    assert!(
        rendered.contains("additionalContext"),
        "the standing overage is reported, not hidden: {rendered}"
    );
}

#[test]
fn an_existing_file_at_an_undeclared_name_is_editable_while_a_new_one_still_denies() {
    // D4e as ratified at AM-3 — the file set is relaxable, the path is not. Three cells plus the
    // control that keeps the set binding on anything that does not exist yet.
    let (state, dir) = state("file-set-amnesty");
    let cwd = dir.display().to_string();
    let home = dir.join(".mochiko/features/FEAT-001");
    std::fs::create_dir_all(&home).expect("dirs");
    let stray = home.join("notes.md");
    std::fs::write(&stray, "a\nb\n").expect("baseline is writable");

    // The control: a *different* undeclared name in the same home has no baseline of its own, so
    // it is denied — an amnestied neighbour excuses nothing.
    let fresh = format!(
        r##"{{"cwd":"{cwd}","tool_name":"Write","tool_input":{{"file_path":"{}","content":"x\n"}}}}"##,
        home.join("invented.md").display()
    );
    let outcome = decide(&state, &fresh);
    assert!(is_deny(&outcome), "a new undeclared name still denies");
    assert!(reason(&outcome).contains("invented.md"));

    // A Write over the existing undeclared name is allowed, and names the violation.
    let over = format!(
        r##"{{"cwd":"{cwd}","tool_name":"Write","tool_input":{{"file_path":"{}","content":"a\nb\nc\n"}}}}"##,
        stray.display()
    );
    let outcome = decide(&state, &over);
    assert!(
        is_allow(&outcome),
        "an existing mis-homed file is not wedged"
    );
    let rendered = hook::render(&outcome);
    assert!(
        rendered.contains("additionalContext") && rendered.contains("notes.md"),
        "the allow carries the standing violation: {rendered}"
    );

    // An Edit over it likewise.
    let edit = format!(
        r##"{{"cwd":"{cwd}","tool_name":"Edit","tool_input":{{"file_path":"{}","old_string":"b\n","new_string":"b\nc\n"}}}}"##,
        stray.display()
    );
    let outcome = decide(&state, &edit);
    assert!(
        is_allow(&outcome),
        "Edit is relaxed on the same terms as Write"
    );
    let rendered = hook::render(&outcome);
    assert!(
        rendered.contains("additionalContext") && rendered.contains("notes.md"),
        "the Edit allow names the standing violation, which is the limb the ledger's gap \
         describes: {rendered}"
    );
}

#[test]
fn the_rendered_json_escapes_every_character_json_requires() {
    let outcome = Outcome::Deny {
        reason: "quote \" backslash \\ newline \n tab \t control \u{1} em—dash".to_string(),
    };
    let rendered = hook::render(&outcome);
    assert!(rendered.contains(r#"\""#), "{rendered}");
    assert!(rendered.contains(r"\\"), "{rendered}");
    assert!(rendered.contains(r"\n"), "{rendered}");
    assert!(rendered.contains(r"\t"), "{rendered}");
    assert!(rendered.contains(r"\u0001"), "{rendered}");
    assert!(rendered.contains("em—dash"), "multi-byte UTF-8 is literal");
    assert!(
        !rendered.contains('\n'),
        "the wrapper prints one line: {rendered}"
    );
}

// ---------------------------------------------------------------------------
// the Edit leg
// ---------------------------------------------------------------------------

#[test]
fn an_edit_is_graded_on_the_result_of_applying_it_to_the_file_on_disk() {
    let (state, dir) = state("edit-leg");
    let cwd = dir.display().to_string();
    let file = dir.join(".mochiko/features/FEAT-001/gates.md");
    std::fs::create_dir_all(file.parent().expect("parent")).expect("dirs");
    std::fs::write(&file, "a\nb\nc\n").expect("baseline is writable");

    // An append that crosses the whole-file bound of 6 is denied before it lands.
    let json = format!(
        r##"{{"cwd":"{cwd}","tool_name":"Edit","tool_input":{{"file_path":"{}","old_string":"c\n","new_string":"c\nd\ne\nf\ng\n"}}}}"##,
        file.display()
    );
    assert!(is_deny(&decide(&state, &json)));

    // An append that stays inside it is allowed.
    let json = format!(
        r##"{{"cwd":"{cwd}","tool_name":"Edit","tool_input":{{"file_path":"{}","old_string":"c\n","new_string":"c\nd\n"}}}}"##,
        file.display()
    );
    assert!(is_allow(&decide(&state, &json)));
}

#[test]
fn an_edit_whose_old_string_is_not_in_the_file_is_not_denied() {
    let (state, dir) = state("edit-nomatch");
    let cwd = dir.display().to_string();
    let file = dir.join(".mochiko/features/FEAT-001/gates.md");
    std::fs::create_dir_all(file.parent().expect("parent")).expect("dirs");
    std::fs::write(&file, "a\n").expect("baseline is writable");
    let json = format!(
        r##"{{"cwd":"{cwd}","tool_name":"Edit","tool_input":{{"file_path":"{}","old_string":"nowhere","new_string":"x"}}}}"##,
        file.display()
    );
    // The platform rejects it on its own; the gate must not add a second, confusing denial.
    assert!(is_allow(&decide(&state, &json)));
}

// ---------------------------------------------------------------------------
// the shell write-operator parse (D1c)
// ---------------------------------------------------------------------------

fn shell(state: &State, cwd: &str, command: &str) -> Outcome {
    // A newline is escaped as JSON escapes it, as the platform's payload does: a raw one inside the
    // string would let a heredoc body's `---` line read as a YAML document marker.
    let escaped = command
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n");
    let json =
        format!(r##"{{"cwd":"{cwd}","tool_name":"Bash","tool_input":{{"command":"{escaped}"}}}}"##);
    decide(state, &json)
}

#[test]
fn every_write_operator_aimed_at_a_home_is_denied() {
    let (state, dir) = state("shell-deny");
    let cwd = dir.display().to_string();
    let target = ".mochiko/features/FEAT-001/gates.md";
    let cases = [
        format!("printf 'x' > {target}"),
        format!("echo x >> {target}"),
        format!("echo x | tee {target}"),
        format!("sed -i '' 's/a/b/' {target}"),
        format!("cat <<'EOF' > {target}\nbody\nEOF"),
        format!("cp /tmp/other.md {target}"),
        format!("mv /tmp/other.md {target}"),
        // Round 1's G5: the clobber redirect was a one-character evasion of the `>` arm.
        format!("printf x >| {target}"),
        format!("printf x >|{target}"),
        // Implemented but previously untested.
        format!("dd of={target} if=/dev/null"),
        format!("install -m 644 /tmp/other.md {target}"),
        // Shapes the plan never named, confirmed at review round 1.
        format!("echo x | tee -a {target}"),
        format!("some-tool 2> {target}"),
        format!("some-tool &> {target}"),
        // The exact line the wave-0 probe false-allowed through the wrapper's grep/sed field().
        format!("printf 'retry probe' > \"{cwd}/{target}\""),
    ];
    for command in cases {
        let outcome = shell(&state, &cwd, &command);
        assert!(
            is_deny(&outcome),
            "a shell write into a home must be denied: {command}"
        );
        assert!(
            reason(&outcome).contains("Write/Edit"),
            "the reason names the real route: {}",
            reason(&outcome)
        );
    }
}

#[test]
fn a_move_out_of_a_home_is_denied_because_the_home_is_the_destination_of_record() {
    let (state, dir) = state("shell-move-out");
    let cwd = dir.display().to_string();
    let outcome = shell(
        &state,
        &cwd,
        "mv .mochiko/features/FEAT-001/gates.md /tmp/elsewhere.md",
    );
    assert!(
        is_deny(&outcome),
        "a re-home under the gate is done with Write, by design (D11 wave 5)"
    );
}

#[test]
fn an_ordinary_shell_command_is_allowed() {
    let (state, dir) = state("shell-allow");
    let cwd = dir.display().to_string();
    for command in [
        "git status --porcelain",
        "cargo test -q",
        "grep -rn foo .mochiko/features/FEAT-001/gates.md",
        "cat .mochiko/features/FEAT-001/gates.md",
        "ls -la .mochiko/features/FEAT-001/",
        // A redirect aimed outside every home is not the gate's business.
        "printf 'x' > /tmp/scratch.md",
        "grep -rn foo .mochiko/features/FEAT-001/gates.md > /tmp/hits.txt",
    ] {
        let outcome = shell(&state, &cwd, command);
        assert!(
            is_allow(&outcome),
            "an ordinary command must pass: {command} — {}",
            reason(&outcome)
        );
    }
}

fn pwsh(state: &State, cwd: &str, command: &str) -> Outcome {
    let escaped = command.replace('\\', "\\\\").replace('"', "\\\"");
    let json = format!(
        r##"{{"cwd":"{cwd}","tool_name":"PowerShell","tool_input":{{"command":"{escaped}"}}}}"##
    );
    decide(state, &json)
}

#[test]
fn every_powershell_write_cmdlet_aimed_at_a_home_is_denied() {
    // The arm reached `check` before this table existed, and allowed every cmdlet below: the
    // vocabulary was POSIX-shaped, so `Set-Content` resolved to no target at all.
    let (state, dir) = state("shell-pwsh-cmdlets");
    let cwd = dir.display().to_string();
    let target = ".mochiko/features/FEAT-001/gates.md";
    let denied = [
        format!("Set-Content -Path {target} -Value 'x'"),
        format!("Set-Content {target} 'x'"),
        format!("Out-File -FilePath {target}"),
        format!("'x' | Out-File {target}"),
        format!("Add-Content -Path {target} -Value 'x'"),
        format!("New-Item -Path {target} -ItemType File"),
        format!("Tee-Object -FilePath {target}"),
        format!("Copy-Item /tmp/other.md {target}"),
        format!("Move-Item /tmp/other.md -Destination {target}"),
        // Both ends of a move: a move *out* of a home names the home as its source.
        format!("Move-Item {target} /tmp/elsewhere.md"),
        // The redirects PowerShell shares with the POSIX shells.
        format!("'x' > {target}"),
        format!("'x' >> {target}"),
        // Case-insensitive, because PowerShell is.
        format!("set-content -path {target} -value 'x'"),
    ];
    for command in denied {
        let outcome = pwsh(&state, &cwd, &command);
        assert!(
            is_deny(&outcome),
            "a PowerShell write into a home must be denied: {command}"
        );
        assert!(
            reason(&outcome).contains("Write/Edit"),
            "the reason names the real route: {}",
            reason(&outcome)
        );
    }

    let allowed = [
        // Reading a home writes nothing.
        format!("Get-Content {target}"),
        "Get-ChildItem .mochiko/features/FEAT-001".to_string(),
        // The false-deny control: a home path as a *value* is content, not a destination.
        format!("Set-Content -Path notes.txt -Value '{target}'"),
    ];
    for command in allowed {
        assert!(
            is_allow(&pwsh(&state, &cwd, &command)),
            "this PowerShell command writes nothing into a home: {command}"
        );
    }
}

#[test]
fn a_bash_command_naming_a_powershell_cmdlet_is_not_measured_against_it() {
    // The vocabularies are keyed to the tool. A Bash line mentioning a cmdlet name means nothing,
    // and reading it as a write would deny ordinary work.
    let (state, dir) = state("shell-pwsh-crosstalk");
    let cwd = dir.display().to_string();
    let command = "echo 'Set-Content -Path .mochiko/features/FEAT-001/gates.md'";
    assert!(is_allow(&shell(&state, &cwd, command)));
}

#[test]
fn a_powershell_redirect_into_a_home_is_denied_on_the_same_table() {
    let (state, dir) = state("shell-powershell");
    let cwd = dir.display().to_string();
    let json = format!(
        r##"{{"cwd":"{cwd}","tool_name":"PowerShell","tool_input":{{"command":"'x' > .mochiko/features/FEAT-001/gates.md"}}}}"##
    );
    assert!(
        is_deny(&decide(&state, &json)),
        "matching Bash alone leaves the Windows shell tool uncovered (record D1c, V6)"
    );
}

#[test]
fn a_tool_outside_the_matcher_set_is_allowed_untouched() {
    let (state, dir) = state("other-tool");
    let cwd = dir.display().to_string();
    for tool in ["Read", "Grep", "NotebookEdit", "Glob"] {
        let json = format!(
            r##"{{"cwd":"{cwd}","tool_name":"{tool}","tool_input":{{"file_path":"{cwd}/.mochiko/features/FEAT-001/invented.md"}}}}"##
        );
        assert!(
            is_allow(&decide(&state, &json)),
            "{tool} is knowingly outside the matcher set (record D1c)"
        );
    }
}

#[test]
fn a_path_in_no_git_tree_resolves_outside_every_home() {
    // No ancestor of `/elsewhere/...` holds `.git`, so there is no home tree to resolve against —
    // an `impl.cold-verification` snapshot is the real case (delta-check N4).
    let (state, dir) = state("outside-cwd");
    let cwd = dir.display().to_string();
    let json = format!(
        r##"{{"cwd":"{cwd}","tool_name":"Write","tool_input":{{"file_path":"/elsewhere/.mochiko/features/FEAT-001/invented.md","content":"x"}}}}"##
    );
    assert!(is_allow(&decide(&state, &json)));
}

#[test]
fn the_out_of_home_sniff_runs_on_a_write_the_homes_do_not_govern() {
    let (state, dir) = state("sniff-through-hook");
    let cwd = dir.display().to_string();
    let json = format!(
        r##"{{"cwd":"{cwd}","tool_name":"Write","tool_input":{{"file_path":"{cwd}/docs/smuggled.md","content":"---\nreport: cycle\n---\n\nBody.\n"}}}}"##
    );
    assert!(
        is_deny(&decide(&state, &json)),
        "a report smuggled outside every home is caught by the sniff (D9)"
    );
}

// ---------------------------------------------------------------------------
// resolution against the tree root (field review D7 as amended at S3)
// ---------------------------------------------------------------------------

/// One shape of the resolution matrix: where the seat stands, the path it names, and whether the
/// write must land in the feature home (and so be denied for its undeclared name).
struct Shape {
    name: &'static str,
    cwd: PathBuf,
    file_path: String,
    in_home: bool,
}

#[test]
fn home_resolution_keys_on_the_tree_root_never_on_the_cwd() {
    let (state, tree) = state("resolve-matrix");
    let other = scratch("resolve-other-tree");
    let sub = tree.join("crates");
    std::fs::create_dir_all(&sub).expect("sub dir");
    let wt = worktree(&tree, "seat");
    let wt_home = wt.join(".mochiko/features/FEAT-001");
    std::fs::create_dir_all(&wt_home).expect("worktree home dir");
    let nested = tree.join("evals/demo/fixtures");
    std::fs::create_dir_all(&nested).expect("nested fixture dir");
    let target = ".mochiko/features/FEAT-001/invented.md";
    let abs = |root: &Path| root.join(target).display().to_string();

    let shapes = [
        Shape {
            name: "root-relative",
            cwd: tree.clone(),
            file_path: target.to_string(),
            in_home: true,
        },
        Shape {
            name: "absolute from a sub-directory cwd (F15)",
            cwd: sub.clone(),
            file_path: abs(&tree),
            in_home: true,
        },
        Shape {
            name: "relative from a sub-directory cwd",
            cwd: sub.clone(),
            file_path: format!("../{target}"),
            in_home: true,
        },
        Shape {
            name: "relative from inside a worktree's home",
            cwd: wt_home.clone(),
            file_path: "invented.md".to_string(),
            in_home: true,
        },
        Shape {
            name: "absolute from the root",
            cwd: tree.clone(),
            file_path: abs(&tree),
            in_home: true,
        },
        Shape {
            name: "a nested fixture tree is never a home",
            cwd: nested.clone(),
            file_path: target.to_string(),
            in_home: false,
        },
        Shape {
            name: "a path in another tree resolves against that tree",
            cwd: tree.clone(),
            file_path: abs(&other),
            in_home: true,
        },
        Shape {
            name: "a path in no git tree",
            cwd: tree.clone(),
            file_path: format!("/nonexistent-mochiko-probe/{target}"),
            in_home: false,
        },
    ];
    for shape in shapes {
        let cwd = shape.cwd.display().to_string();
        let outcome = decide(&state, &write_payload(&cwd, &shape.file_path, "# x\n"));
        assert_eq!(
            is_deny(&outcome),
            shape.in_home,
            "{}: cwd {cwd}, path {} — {}",
            shape.name,
            shape.file_path,
            reason(&outcome)
        );
        if shape.in_home {
            assert!(
                reason(&outcome).contains("not a declared deliverable"),
                "{}: denied as the feature home's file set: {}",
                shape.name,
                reason(&outcome)
            );
        }
    }
}

// ---------------------------------------------------------------------------
// the closed world under `.mochiko/` (field review D3)
// ---------------------------------------------------------------------------

fn edit_payload(cwd: &str, file_path: &str, old: &str, new: &str) -> String {
    format!(
        r##"{{"cwd":"{cwd}","tool_name":"Edit","tool_input":{{"file_path":"{file_path}","old_string":"{old}","new_string":"{new}"}}}}"##
    )
}

/// The route-1 and route-2 text every closed-world deny carries.
fn assert_both_routes(reason: &str, main: &Path, what: &str) {
    assert!(
        reason.contains("declared deliverable"),
        "{what}: route 1: {reason}"
    );
    assert!(
        reason.contains(&format!("{}/.mochiko/runs/", main.display())),
        "{what}: route 2 names the main tree's run folder by absolute path: {reason}"
    );
    assert!(
        reason.contains("ephemeral"),
        "{what}: route 2 is ephemeral: {reason}"
    );
}

#[test]
fn a_write_under_mochiko_to_no_home_is_refused_on_every_leg_with_both_routes() {
    let (state, tree) = state("closed-world");
    let cwd = tree.display().to_string();
    let archived = tree.join(".mochiko/archive/notes.md");
    std::fs::create_dir_all(archived.parent().expect("parent")).expect("archive dir");
    std::fs::write(&archived, "old\n").expect("seed");

    let write = decide(
        &state,
        &write_payload(&cwd, ".mochiko/evidence/console.log", "raw\n"),
    );
    // The path is not a relaxable measure: an existing file in no home is refused too.
    let edit = decide(
        &state,
        &edit_payload(&cwd, &archived.display().to_string(), "old", "new"),
    );
    let bash = shell(&state, &cwd, "printf x > .mochiko/evidence/console.log");
    for (what, outcome) in [("Write", &write), ("Edit", &edit), ("Bash", &bash)] {
        assert!(is_deny(outcome), "{what} must be refused");
        let reason = reason(outcome);
        assert!(
            reason.contains("a write here is refused"),
            "{what}: {reason}"
        );
        assert_both_routes(&reason, &tree, what);
    }
}

#[test]
fn a_closed_world_refusal_in_a_worktree_names_the_main_trees_run_folder() {
    let (state, main) = state("closed-world-worktree");
    let wt = worktree(&main, "seat");
    let cwd = wt.display().to_string();
    let outcome = decide(
        &state,
        &write_payload(&cwd, ".mochiko/evidence/console.log", "raw\n"),
    );
    assert!(is_deny(&outcome));
    let reason = reason(&outcome);
    assert_both_routes(&reason, &main, "worktree");
    assert!(
        !reason.contains(&format!("{}/.mochiko/runs/", wt.display())),
        "never the worktree's own run folder: {reason}"
    );
}

#[test]
fn an_undeclared_subdir_deny_through_the_hook_names_both_routes() {
    let (state, tree) = state("closed-world-subdir");
    let cwd = tree.display().to_string();
    let outcome = decide(
        &state,
        &write_payload(
            &cwd,
            ".mochiko/features/FEAT-001/evidence/console.log",
            "raw\n",
        ),
    );
    let reason = reason(&outcome);
    assert!(reason.contains("not a declared sub-directory"), "{reason}");
    assert_both_routes(&reason, &tree, "subdir");
}

#[test]
fn outside_mochiko_the_closed_world_does_not_reach() {
    let (state, tree) = state("closed-world-outside");
    let cwd = tree.display().to_string();
    for (what, path, content) in [
        // The root operating docs are unchanged: not watched (D7 as amended).
        (
            "a root operating doc",
            "ROADMAP.md",
            "# Roadmap\n\n## Now\n",
        ),
        ("a plain doc", "docs/notes.md", "# notes\n"),
        (
            "a nested tree's closed-world path",
            "evals/demo/fixtures/.mochiko/evidence/console.log",
            "raw\n",
        ),
    ] {
        let outcome = decide(&state, &write_payload(&cwd, path, content));
        assert!(
            is_allow(&outcome),
            "{what} is allowed: {}",
            reason(&outcome)
        );
    }
}

// ---------------------------------------------------------------------------
// the run folder's controls (field review D4 as amended, V2, V3, V7)
// ---------------------------------------------------------------------------

const RUN: &str = ".mochiko/runs/FEAT-001-run5";
const REPORT: &str = "---\nreport: cycle\n---\n\nBody.\n";

/// A tree whose `.gitignore` carries the run folder, as run-open leaves it.
fn ignoring(tag: &str) -> (State, PathBuf) {
    let (state, tree) = state(tag);
    std::fs::write(tree.join(".gitignore"), "target/\n.mochiko/runs/\n").expect(".gitignore");
    (state, tree)
}

#[test]
fn a_write_to_the_run_folder_is_refused_until_gitignore_carries_it() {
    let (state, tree) = state("runs-guard");
    let cwd = tree.display().to_string();
    let path = format!("{RUN}/console.log");
    let refused = decide(&state, &write_payload(&cwd, &path, "raw\n"));
    assert!(is_deny(&refused), "nothing here may reach a commit");
    let text = reason(&refused);
    assert!(
        text.contains(".gitignore") && text.contains("`.mochiko/runs/`"),
        "the reason names the line and the file: {text}"
    );

    std::fs::write(tree.join(".gitignore"), ".mochiko/runs/\n").expect(".gitignore");
    let allowed = decide(&state, &write_payload(&cwd, &path, "raw\n"));
    assert!(is_allow(&allowed), "{}", reason(&allowed));

    // The run folder resolves against the tree root from any cwd inside the tree.
    let sub = tree.join("crates");
    std::fs::create_dir_all(&sub).expect("sub dir");
    let from_sub = decide(
        &state,
        &write_payload(&sub.display().to_string(), &format!("../{path}"), "raw\n"),
    );
    assert!(is_allow(&from_sub), "{}", reason(&from_sub));
}

#[test]
fn the_run_folder_takes_any_raw_file_but_refuses_a_report() {
    let (state, tree) = ignoring("runs-content");
    let cwd = tree.display().to_string();
    for (what, name, content, denied) in [
        ("a report-shaped `.md`", "cycle-report.md", REPORT, true),
        ("a plain `.md`", "notes.md", "# notes\n\nraw\n", false),
        ("a report-shaped non-`.md`", "capture.json", REPORT, false),
        (
            "a nested raw file",
            "cycle-2/screens/home.png",
            "png",
            false,
        ),
        // V7: the ephemeral run log is allowed by name, whatever it opens with.
        ("the run log", "implement-log.md", REPORT, false),
    ] {
        let outcome = decide(
            &state,
            &write_payload(&cwd, &format!("{RUN}/{name}"), content),
        );
        assert_eq!(is_deny(&outcome), denied, "{what}: {}", reason(&outcome));
        if denied {
            assert!(
                reason(&outcome).contains("`report: cycle`"),
                "{what}: {}",
                reason(&outcome)
            );
        }
    }
}

#[test]
fn a_declared_subdir_without_a_home_document_refuses_a_report_but_takes_raw_material() {
    // V2, as ruled at plan Q2 (B): every declared sub-directory with no home document of its own.
    let (state, tree) = state("deferred-sniff");
    let cwd = tree.display().to_string();
    let base = ".mochiko/features/FEAT-001/prototype";
    for (name, content, denied) in [
        ("notes.md", REPORT, true),
        ("notes.md", "# notes\n", false),
        ("index.html", REPORT, false),
        ("screens/flow.md", REPORT, true),
    ] {
        let outcome = decide(
            &state,
            &write_payload(&cwd, &format!("{base}/{name}"), content),
        );
        assert_eq!(is_deny(&outcome), denied, "{name}: {}", reason(&outcome));
    }
}

#[test]
fn a_write_to_a_worktrees_own_run_folder_is_refused_with_the_main_trees_path() {
    let (state, main) = ignoring("runs-worktree");
    let wt = worktree(&main, "seat");
    std::fs::write(wt.join(".gitignore"), ".mochiko/runs/\n").expect(".gitignore");
    let cwd = wt.display().to_string();
    let outcome = decide(
        &state,
        &write_payload(&cwd, &format!("{RUN}/console.log"), "raw\n"),
    );
    assert!(
        is_deny(&outcome),
        "the run folder is always the main tree's (V3)"
    );
    let reason = reason(&outcome);
    assert!(
        reason.contains(&format!("{}/.mochiko/runs/<run-id>/", main.display())),
        "{reason}"
    );
}

#[test]
fn a_folder_under_runs_not_named_by_the_run_key_is_refused_and_says_why() {
    let (state, tree) = ignoring("runs-key");
    let cwd = tree.display().to_string();
    let outcome = decide(
        &state,
        &write_payload(&cwd, ".mochiko/runs/scratch/console.log", "raw\n"),
    );
    assert!(is_deny(&outcome));
    let reason = reason(&outcome);
    assert!(reason.contains("`scratch` is not a run key"), "{reason}");
    assert!(reason.contains("`<owner>-run<n>`"), "{reason}");
    assert!(reason.contains("FEAT-001-run5"), "{reason}");
}

#[test]
fn a_shell_write_into_a_home_names_the_run_folder_for_raw_output() {
    // Field review 2.7: every deny inside `.mochiko/` that could be raw output names the run
    // folder's absolute path — and a `tee` into a home is the raw-capture shape (F11).
    let (state, tree) = state("shell-raw-route");
    let cwd = tree.display().to_string();
    let outcome = shell(
        &state,
        &cwd,
        "cargo test 2>&1 | tee .mochiko/features/FEAT-001/gates.md",
    );
    assert!(is_deny(&outcome));
    let text = reason(&outcome);
    assert!(text.contains("Write/Edit"), "{text}");
    assert!(
        text.contains(&format!("{}/.mochiko/runs/<run-id>/", tree.display()))
            && text.contains("ephemeral"),
        "{text}"
    );
}

#[test]
fn a_log_that_declares_no_home_closes_no_world() {
    // The closed world is the log's declaration made total; a log carrying no home at all has
    // declared nothing to close, on either leg.
    let dir = scratch("no-homes");
    let log = dir.join("log");
    std::fs::create_dir_all(&log).expect("log dir");
    let body = "grammar: 1\nid: 0001-empty\nsequence: 1\nintent: No homes.\nchanges:\n  \
                - op: import-document\n    kind: template\n    name: t\n    content:\n      \
                template: t\n      title: T\n      form: f.md\n      register: full\n      \
                overview: O.\n      sections: []\n      skeleton: \"x\"\n";
    let stamped = migration::with_hash("0001-empty.yaml", body).expect("well-formed");
    std::fs::write(log.join("0001-empty.yaml"), stamped).expect("writable");
    let state = replay::load(&log).expect("replays");
    let cwd = dir.display().to_string();
    let write = decide(
        &state,
        &write_payload(&cwd, ".mochiko/evidence/x.log", "raw\n"),
    );
    assert!(is_allow(&write), "{}", reason(&write));
    let bash = shell(&state, &cwd, "printf x > .mochiko/evidence/x.log");
    assert!(is_allow(&bash), "{}", reason(&bash));
}

// ---------------------------------------------------------------------------
// the shell leg on `crate::shell`, the run-folder carve, and the evasions (T9)
// ---------------------------------------------------------------------------

#[test]
fn a_read_whose_line_carries_a_later_in_place_flag_is_not_a_write() {
    // This session's own denied read (build-log, 2026-09-29): the old scan read `grep -i` as
    // `sed -i` over every later token, and the glued `;` kept the ledger path in `sed`'s run.
    let (state, tree) = state("shell-glued-read");
    let cwd = tree.display().to_string();
    for command in [
        "sed -n 276,312p .mochiko/memory/governance-ledger.md; echo ----; grep -n -i x .mochiko/memory/governance-ledger.md",
        "sed -n 1,5p .mochiko/features/FEAT-001/gates.md; grep -i x .mochiko/features/FEAT-001/gates.md",
    ] {
        let outcome = shell(&state, &cwd, command);
        assert!(is_allow(&outcome), "a read, not a write: {command} — {}", reason(&outcome));
    }
}

#[test]
fn a_non_markdown_shell_write_into_the_run_folder_is_admitted() {
    // Field review V1/N2: raw capture is shell output by nature (F11's `cmd 2>&1 | tee …`).
    let (state, tree) = ignoring("shell-carve");
    let cwd = tree.display().to_string();
    for command in [
        format!("cargo test 2>&1 | tee {RUN}/console.log"),
        format!("printf x > {RUN}/out.txt"),
        format!("some-tool >> {RUN}/cycle-2/trace.json"),
        format!("cp target/report.html {RUN}/report.html"),
        // Moving raw output out of the run folder is a write to it, and admitted like one.
        format!("mv {RUN}/a.log /tmp/a.log"),
    ] {
        let outcome = shell(&state, &cwd, &command);
        assert!(is_allow(&outcome), "{command} — {}", reason(&outcome));
    }
}

#[test]
fn a_shell_write_of_markdown_or_to_a_directory_under_the_run_folder_is_refused() {
    let (state, tree) = ignoring("shell-carve-refused");
    let cwd = tree.display().to_string();
    // N2: a heredoc could otherwise smuggle a report past the content rule — a `.md` goes through
    // Write, where the sniff reads it.
    let heredoc = shell(
        &state,
        &cwd,
        &format!("cat <<'EOF' > {RUN}/r.md\n---\nreport: cycle\n---\nEOF"),
    );
    assert!(is_deny(&heredoc));
    assert!(reason(&heredoc).contains("Write"), "{}", reason(&heredoc));
    let log = shell(&state, &cwd, &format!("echo x >> {RUN}/implement-log.md"));
    assert!(
        is_deny(&log),
        "the run log is markdown too: {}",
        reason(&log)
    );
    // Plan Q3: a directory target hides the file name the carve's own test needs.
    for command in [
        format!("cp build.log {RUN}/"),
        format!("cp build.log {RUN}/sub/"),
        format!("cp -t {RUN} report.md"),
    ] {
        let outcome = shell(&state, &cwd, &command);
        assert!(is_deny(&outcome), "{command}");
        assert!(
            reason(&outcome).contains("name the destination file"),
            "{command}: {}",
            reason(&outcome)
        );
    }
}

#[test]
fn a_shell_write_into_the_run_folder_keeps_the_ignore_guard_and_the_main_tree_rule() {
    let (state, tree) = state("shell-carve-guard");
    let cwd = tree.display().to_string();
    let unguarded = shell(&state, &cwd, &format!("printf x > {RUN}/out.txt"));
    assert!(is_deny(&unguarded));
    assert!(
        reason(&unguarded).contains(".gitignore"),
        "{}",
        reason(&unguarded)
    );

    std::fs::write(tree.join(".gitignore"), ".mochiko/runs/\n").expect(".gitignore");
    let wt = worktree(&tree, "seat");
    let from_worktree = shell(
        &state,
        &wt.display().to_string(),
        &format!("printf x > {RUN}/out.txt"),
    );
    assert!(is_deny(&from_worktree));
    assert!(
        reason(&from_worktree).contains(&format!("{}/.mochiko/runs/<run-id>/", tree.display())),
        "{}",
        reason(&from_worktree)
    );
}

#[test]
fn the_prior_builds_disclosed_evasions_three_closed_one_closed_for_a_literal_cd_two_open() {
    let (state, tree) = state("shell-evasions");
    let home = tree.join(".mochiko/features/FEAT-001");
    std::fs::create_dir_all(&home).expect("home dir");

    // Evasion 3, closed (lead ruling at plan round 1): the target is joined to the payload's cwd,
    // so a relative write from a cwd inside a home resolves into that home.
    let inside = shell(&state, &home.display().to_string(), "printf x > x.md");
    assert!(
        is_deny(&inside) && home_shell_deny(&inside),
        "evasion 3 is closed: {}",
        reason(&inside)
    );

    let root = tree.display().to_string();
    // Evasion 1, closed for an absolute literal `cd` (S2's fix round): the later relative target
    // is joined to that directory, so the write resolves into the home.
    let literal = shell(
        &state,
        &root,
        &format!("cd {} && printf x > x.md", home.display()),
    );
    assert!(
        is_deny(&literal) && home_shell_deny(&literal),
        "an absolute literal `cd` into a home is closed: {}",
        reason(&literal)
    );
    // Evasion 1, still open and disclosed for a relative operand: the scan does not stack a
    // relative `cd` on the cwd, so the later relative target is dropped rather than guessed.
    let relative = shell(
        &state,
        &root,
        "cd .mochiko/features/FEAT-001 && printf x > x.md",
    );
    assert!(
        is_allow(&relative),
        "evasion 1 stays open for a relative `cd`: {}",
        reason(&relative)
    );
    // ... and for an expansion operand, where the directory is unknown to the scan ...
    let expansion = shell(&state, &root, "cd $D && printf x > x.md");
    assert!(
        is_allow(&expansion),
        "evasion 1 stays open for `cd $D`: {}",
        reason(&expansion)
    );
    // ... except a relative target that opens with `.mochiko`: it is kept, joined to the
    // payload's cwd, and so resolves into the home.
    let kept = shell(
        &state,
        &root,
        "cd $D && printf x > .mochiko/features/FEAT-001/x.md",
    );
    assert!(
        is_deny(&kept) && home_shell_deny(&kept),
        "a `.mochiko`-first target after `cd $D` is kept: {}",
        reason(&kept)
    );
    // Evasion 2, still open and disclosed: a path held in a variable is an expansion the scan
    // cannot resolve, so `crate::shell` drops it rather than guessing (`resolvable`) — the
    // assignment is no target and `"$f"` returns none, confirmed against the landed parse.
    let command = "f=.mochiko/features/FEAT-001/x.md; printf x > \"$f\"";
    assert!(
        mochiko_cli::shell::write_targets(command).is_empty(),
        "{:?}",
        mochiko_cli::shell::write_targets(command)
    );
    let variable = shell(&state, &root, command);
    assert!(
        is_allow(&variable),
        "evasion 2 stays open: {}",
        reason(&variable)
    );
}

/// Whether a denial is the home shell deny, not the closed-world or a run-folder refusal.
fn home_shell_deny(outcome: &Outcome) -> bool {
    let text = reason(outcome);
    text.contains("never through a shell redirect")
        && text.contains("resolves under a declared artifact home")
}

// ---------------------------------------------------------------------------
// G1 round 1, fix round: A2 · A5 · A9
// ---------------------------------------------------------------------------

#[test]
fn the_run_folder_named_as_a_file_gets_no_empty_run_key_sentence() {
    // A2: for `.mochiko/runs` itself and a run folder named as a file there is no segment past
    // the run key to blame, so the run-key sentence would print empty backticks.
    let (state, tree) = ignoring("a2-run-key");
    let cwd = tree.display().to_string();
    for path in [".mochiko/runs", RUN] {
        let outcome = decide(&state, &write_payload(&cwd, path, "raw\n"));
        assert!(
            is_deny(&outcome),
            "{path}: nothing may be written in place of the folder"
        );
        let text = reason(&outcome);
        assert!(!text.contains("is not a run key"), "{path}: {text}");
        assert!(!text.contains("``"), "{path}: no empty code span: {text}");
        assert!(text.contains("a write here is refused"), "{path}: {text}");
    }
}

#[test]
fn a_path_in_no_git_tree_takes_no_report_sniff_and_a_path_in_a_tree_still_does() {
    // A5: outside `<root>/.mochiko/` nothing changes (D3) — a path in no git tree at all (the
    // session scratchpad, `~/.claude/`) is not the gate's, as a path outside the cwd was at HEAD.
    let (state, tree) = state("a5-no-tree-sniff");
    let cwd = tree.display().to_string();
    let no_tree = decide(
        &state,
        &write_payload(&cwd, "/nonexistent-mochiko-probe/scratch/report.md", REPORT),
    );
    assert!(is_allow(&no_tree), "{}", reason(&no_tree));
    let in_tree = decide(&state, &write_payload(&cwd, "docs/report.md", REPORT));
    assert!(
        is_deny(&in_tree),
        "the sniff still reads a path in the tree outside `.mochiko/`"
    );
}

#[cfg(unix)]
#[test]
fn a_write_the_amnesty_does_not_need_never_reads_the_file_on_disk() {
    // A9: the on-disk file is read only after resolution, and only where it is needed — for an
    // Edit, and for the amnesty inside a home. A named pipe makes an unneeded read observable: a
    // read of one blocks until a writer opens it, so a gate that read it would never answer.
    use std::sync::mpsc;
    use std::time::Duration;
    let (state, tree) = state("a9-no-read");
    let docs = tree.join("docs");
    std::fs::create_dir_all(&docs).expect("docs dir");
    let pipe = docs.join("pipe.md");
    let made = std::process::Command::new("mkfifo")
        .arg(&pipe)
        .status()
        .expect("mkfifo runs");
    assert!(made.success(), "the fifo is creatable");
    let payload = write_payload(
        &tree.display().to_string(),
        &pipe.display().to_string(),
        "# plain\n",
    );
    let (sent, received) = mpsc::channel();
    std::thread::spawn(move || {
        let outcome = decide(&state, &payload);
        let _ = sent.send(is_allow(&outcome));
    });
    let answered = received
        .recv_timeout(Duration::from_secs(5))
        .expect("the gate answers without reading a file it does not need");
    assert!(answered, "a plain `.md` outside `.mochiko/` is allowed");
}
