//! The hook payload, the shell write-operator parse, and the decision JSON the wrapper prints.
//!
//! The wrapper is a pipe: it hands the raw `PreToolUse` payload to the binary on stdin and prints
//! what comes back. So the parse and the emit are the binary's, and the shell script holds no rule
//! — the same posture as `dependency-halt.sh`.

use mochiko_cli::hook::{self, Outcome};
use mochiko_cli::migration;
use mochiko_cli::replay::{self, State};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn scratch(tag: &str) -> PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("hook-{tag}-{n}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir is creatable");
    dir
}

const LOG: &str = r#"
grammar: 1
id: 0001-hook
sequence: 1
intent: One home and its template, for the hook surface.
changes:
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
    let escaped = command.replace('\\', "\\\\").replace('"', "\\\"");
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
fn a_write_outside_the_working_directory_is_allowed() {
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
