//! Integration tests for the CLI surface: the command tree, the exit codes, and the resolution
//! order for the plugin root, the log directory and the plugin version.
//!
//! Every fixture is written under `CARGO_TARGET_TMPDIR`, inside `target/`. Dispatch is driven
//! in-process through [`mochiko_cli::cli::dispatch`], which writes to caller-supplied sinks so a
//! test can assert on the exact bytes each stream carried without spawning the binary.

use mochiko_cli::cli;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// The repository root, anchored at the crate directory.
const REPO_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

fn scratch(tag: &str) -> PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("cli-{tag}-{n}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir is creatable");
    dir
}

fn write_migration(dir: &Path, name: &str, body: &str) {
    let stamped = mochiko_cli::migration::with_hash(name, body)
        .unwrap_or_else(|e| panic!("fixture {name} is not a well-formed migration: {e}"));
    std::fs::write(dir.join(name), stamped).expect("fixture migration is writable");
}

/// A minimal but valid corpus: one command carrying the six command sections, its registry, and
/// one template. Enough for every exit-code path without restating the render fixture.
const LOG: &str = r#"
grammar: 1
id: 0001-genesis
sequence: 1
intent: A minimal corpus for the CLI surface tests.
changes:
  - op: import-document
    kind: command-labels
    name: command-labels
    content:
      kind: command-labels
      labels:
        seats: Seat wiring.
  - op: import-document
    kind: command
    name: demo
    content:
      kind: command
      command: demo
      sections:
        - id: demo.sec.roles
          title: Roles
          intent: Seat wiring.
          rules:
            - id: demo.lead
              labels: [seats]
              class: must
              text: The lead plans the run.
        - id: demo.sec.reserved
          title: Reserved
          intent: Reserved to the user.
          note: Nothing reserved.
          rules: []
        - id: demo.sec.tools
          title: Tools
          intent: The skills reached for.
          note: No skills.
          rules: []
        - id: demo.sec.ways-of-working
          title: Ways of working
          intent: How the run proceeds.
          note: Nothing yet.
          rules: []
        - id: demo.sec.boundaries
          title: Boundaries
          intent: The floor.
          rules:
            - id: demo.boundary
              labels: [seats]
              class: floor
              text: The user rules acceptance.
        - id: demo.sec.fail-conditions
          title: Not done
          intent: The fail set.
          rules:
            - id: demo.fail.unaccepted
              labels: [seats]
              class: floor
              kind: fail
              enforces: [demo.boundary]
              text: An unaccepted record.
  - op: import-document
    kind: template
    name: demo-template
    content:
      template: demo-template
      title: Demo Template
      form: markdown
      register: full
      overview: A template carried as log data.
      sections:
        - name: Purpose
          required: true
          contract: State the purpose.
          check: Is the purpose stated?
      skeleton: |
        # Demo
"#;

/// A log directory holding [`LOG`].
fn log(tag: &str) -> PathBuf {
    let dir = scratch(tag);
    write_migration(&dir, "0001-genesis.yaml", LOG);
    dir
}

struct Run {
    code: i32,
    out: String,
    err: String,
}

fn run(args: &[&str]) -> Run {
    let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    let mut out = Vec::new();
    let mut err = Vec::new();
    let code = cli::dispatch(&owned, &mut out, &mut err);
    Run {
        code,
        out: String::from_utf8(out).expect("stdout is utf-8"),
        err: String::from_utf8(err).expect("stderr is utf-8"),
    }
}

/// Run the built binary as a child process, with its working directory and environment set on the
/// child alone.
///
/// The two resolution limbs below depend on ambient state — an environment variable and the
/// working directory — and neither can be set for one in-process test without setting it for every
/// other test running beside it on another thread. `set_var` in a test binary is a race whose
/// safety is a reading of every sibling test rather than a property of the code, and it becomes
/// `unsafe` under edition 2024. A child process carries its own copy of both, so the ambient state
/// is real for the code under test and invisible to the rest of the suite. It also puts `main.rs`
/// on a tested path for the first time.
///
/// `env` entries with `None` are removed from the child rather than set, so a variable already
/// present in the developer's own shell cannot decide the outcome.
fn run_binary(args: &[&str], cwd: &Path, env: &[(&str, Option<&str>)]) -> Run {
    let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_mochiko-cli"));
    command.args(args).current_dir(cwd);
    for (key, value) in env {
        match value {
            Some(value) => command.env(key, value),
            None => command.env_remove(key),
        };
    }
    let output = command.output().expect("the built binary runs");
    Run {
        code: output.status.code().unwrap_or(-1),
        out: String::from_utf8(output.stdout).expect("stdout is utf-8"),
        err: String::from_utf8(output.stderr).expect("stderr is utf-8"),
    }
}

/// A directory holding a `migrations/` subdirectory with [`LOG`] in it, for the limbs that resolve
/// a log relative to something rather than from a flag.
fn dir_with_log(tag: &str) -> PathBuf {
    let root = scratch(tag);
    let log = root.join(LOG_DIR_NAME);
    std::fs::create_dir_all(&log).expect("log dir is creatable");
    write_migration(&log, "0001-genesis.yaml", LOG);
    root
}

/// The log directory's name under a plugin root, and the working-directory last resort.
const LOG_DIR_NAME: &str = "migrations";

/// The variable naming the log, read after an explicit flag and after a plugin root.
const LOG_DIR_ENV: &str = "MOCHIKO_MIGRATIONS";

// ---------------------------------------------------------------------------
// the version contract
// ---------------------------------------------------------------------------

#[test]
fn version_prints_the_binary_semver_and_the_supported_grammar_range() {
    let r = run(&["--version"]);
    assert_eq!(r.code, 0);
    assert_eq!(
        r.out,
        format!(
            "mochiko-cli {} · grammar {}..{}\n",
            env!("CARGO_PKG_VERSION"),
            mochiko_cli::migration::GRAMMAR_RANGE.0,
            mochiko_cli::migration::GRAMMAR_RANGE.1
        )
    );
}

#[test]
fn a_log_written_in_an_out_of_range_grammar_exits_3_with_the_install_line() {
    let dir = scratch("skew");
    std::fs::write(
        dir.join("0001-genesis.yaml"),
        "grammar: 99\nid: 0001-genesis\nsequence: 1\nintent: A log from the future.\nchanges: []\n",
    )
    .expect("fixture is writable");

    for args in [
        vec!["rules", "demo", "--section", "preamble"],
        vec!["template", "demo-template"],
        vec!["migrate", "status"],
        vec!["migrate", "validate"],
    ] {
        let mut full = args.clone();
        full.push("--log-dir");
        let dir_string = dir.display().to_string();
        full.push(&dir_string);
        let r = run(&full);
        assert_eq!(r.code, 3, "{args:?} should exit 3, got:\n{}", r.err);
        assert!(
            r.err.contains(mochiko_cli::migration::INSTALL_COMMAND),
            "{args:?}: the halt names the install command:\n{}",
            r.err
        );
        assert!(
            r.err.contains("grammar 99"),
            "{args:?}: the halt names the log's grammar:\n{}",
            r.err
        );
    }
}

// ---------------------------------------------------------------------------
// usage and unknown names (exit 2)
// ---------------------------------------------------------------------------

#[test]
fn help_and_no_arguments_exit_2_or_0_without_touching_the_log() {
    assert_eq!(run(&["--help"]).code, 0);
    // clap treats a missing subcommand as a usage error, which is exit 2 by the §4 table.
    assert_eq!(run(&[]).code, 2);
}

#[test]
fn an_unknown_subcommand_exits_2() {
    let r = run(&["frobnicate"]);
    assert_eq!(r.code, 2);
    assert!(!r.err.is_empty(), "a usage error explains itself");
}

#[test]
fn rules_without_a_section_exits_2() {
    let dir = log("nosection");
    let r = run(&["rules", "demo", "--log-dir", &dir.display().to_string()]);
    assert_eq!(r.code, 2, "--section is required:\n{}", r.err);
}

#[test]
fn rules_for_an_unknown_primitive_exits_2() {
    let dir = log("unknownprim");
    let r = run(&[
        "rules",
        "does-not-exist",
        "--section",
        "preamble",
        "--log-dir",
        &dir.display().to_string(),
    ]);
    assert_eq!(r.code, 2);
    assert!(
        r.err.contains("does-not-exist"),
        "the error names the primitive:\n{}",
        r.err
    );
}

#[test]
fn rules_for_an_unknown_section_exits_2_and_names_the_sections_that_exist() {
    let dir = log("unknownsec");
    let r = run(&[
        "rules",
        "demo",
        "--section",
        "not-a-section",
        "--log-dir",
        &dir.display().to_string(),
    ]);
    assert_eq!(r.code, 2);
    assert!(
        r.err.contains("demo.sec.roles"),
        "the error lists the real sections:\n{}",
        r.err
    );
}

#[test]
fn an_unknown_template_exits_2() {
    let dir = log("unknowntpl");
    let r = run(&[
        "template",
        "does-not-exist",
        "--log-dir",
        &dir.display().to_string(),
    ]);
    assert_eq!(r.code, 2);
}

// ---------------------------------------------------------------------------
// an unsound or absent log (exit 1)
// ---------------------------------------------------------------------------

#[test]
fn an_absent_log_exits_1_and_names_the_directory_it_looked_in() {
    let dir = scratch("absent").join("no-such-dir");
    let r = run(&[
        "rules",
        "demo",
        "--section",
        "preamble",
        "--log-dir",
        &dir.display().to_string(),
    ]);
    assert_eq!(r.code, 1);
    assert!(
        r.err.contains(&dir.display().to_string()),
        "the halt names the directory:\n{}",
        r.err
    );
}

#[test]
fn an_empty_log_directory_exits_1_rather_than_rendering_nothing() {
    let dir = scratch("emptylog");
    let r = run(&[
        "rules",
        "demo",
        "--section",
        "preamble",
        "--log-dir",
        &dir.display().to_string(),
    ]);
    assert_eq!(r.code, 1, "an empty log is a delivery failure:\n{}", r.err);
    assert!(r.err.contains("empty"), "the halt says so:\n{}", r.err);
}

/// Every path treats an empty log the same way. `migrate validate` reporting green on a
/// mis-pointed `--log-dir` would make it useless as a gate — the one job it has.
#[test]
fn every_subcommand_exits_1_on_an_empty_log_directory_and_names_it() {
    let dir = scratch("emptyall");
    let dir_string = dir.display().to_string();
    for args in [
        vec!["rules", "demo", "--section", "preamble"],
        vec!["template", "demo-template"],
        vec!["migrate", "status"],
        vec!["migrate", "validate"],
    ] {
        let mut full = args.clone();
        full.push("--log-dir");
        full.push(&dir_string);
        let r = run(&full);
        assert_eq!(r.code, 1, "{args:?} should exit 1:\n{}{}", r.out, r.err);
        assert!(
            r.err.contains(&dir_string),
            "{args:?}: the halt names the directory:\n{}",
            r.err
        );
        assert!(
            r.err.contains("empty"),
            "{args:?}: the halt says the log is empty:\n{}",
            r.err
        );
    }
}

/// A name the log carries as both a command and a skill is ambiguous, not absent. Reporting it as
/// absent asserts the opposite of the truth and sends the reader hunting for a typo.
#[test]
fn a_name_carried_as_both_a_command_and_a_skill_is_reported_as_ambiguous() {
    let dir = scratch("ambiguous");
    write_migration(&dir, "0001-genesis.yaml", LOG);
    write_migration(
        &dir,
        "0002-twin.yaml",
        r#"
grammar: 1
id: 0002-twin
sequence: 2
intent: Import a skill sharing its name with a command.
changes:
  - op: import-document
    kind: skill-labels
    name: skill-labels
    content:
      kind: skill-labels
      labels:
        scope: What the skill covers.
  - op: import-document
    kind: skill
    name: demo
    content:
      kind: skill
      skill: demo
      sections:
        - id: demo.sec.independence
          title: Independence
          intent: Who may run this skill.
          note: Nothing beyond the standing floor.
          rules: []
        - id: demo.sec.scope
          title: Scope
          intent: What is graded.
          rules:
            - id: demo.scope-fence
              labels: [scope]
              class: must
              text: Grade the artifact.
        - id: demo.sec.inputs
          title: Inputs
          intent: What the skill reads.
          note: The artifact alone.
          rules: []
        - id: demo.sec.verdict
          title: Verdict
          intent: The verdict grammar.
          note: PASS or FAIL.
          rules: []
        - id: demo.sec.output
          title: Output
          intent: The report shape.
          note: One report.
          rules: []
        - id: demo.sec.reserved
          title: Reserved
          intent: Reserved to the user.
          note: Nothing reserved.
          rules: []
"#,
    );
    let r = run(&[
        "rules",
        "demo",
        "--section",
        "preamble",
        "--log-dir",
        &dir.display().to_string(),
    ]);
    assert_eq!(r.code, 2, "an ambiguous name is a usage error:\n{}", r.err);
    assert!(
        r.err.contains("ambiguous"),
        "the error says the name is ambiguous:\n{}",
        r.err
    );
    assert!(
        r.err.contains("command") && r.err.contains("skill"),
        "the error names both kinds:\n{}",
        r.err
    );
    assert!(
        !r.err.contains("no command or skill named"),
        "the error must not assert the name is absent:\n{}",
        r.err
    );
}

#[test]
fn a_rejecting_log_exits_1_with_one_finding_per_line() {
    let dir = scratch("rejecting");
    write_migration(&dir, "0001-genesis.yaml", LOG);
    write_migration(
        &dir,
        "0002-break.yaml",
        r#"
grammar: 1
id: 0002-break
sequence: 2
intent: Mint a rule carrying a label no registry declares.
changes:
  - op: mint-rule
    schema: command/demo
    section: demo.sec.roles
    rule:
      id: demo.unregistered
      labels: [not-a-label]
      class: must
      text: A rule with an unregistered label.
"#,
    );
    let r = run(&[
        "rules",
        "demo",
        "--section",
        "demo.sec.roles",
        "--log-dir",
        &dir.display().to_string(),
    ]);
    assert_eq!(r.code, 1, "an unsound log is never rendered from");
    assert!(
        r.err.contains("not-a-label"),
        "the finding names the offending label:\n{}",
        r.err
    );
}

// ---------------------------------------------------------------------------
// migrate
// ---------------------------------------------------------------------------

#[test]
fn migrate_status_prints_the_log_grammar_sequences_and_the_state_hash() {
    let dir = log("status");
    let r = run(&["migrate", "status", "--log-dir", &dir.display().to_string()]);
    assert_eq!(r.code, 0, "{}", r.err);
    let lines: Vec<&str> = r.out.lines().collect();
    assert_eq!(
        lines[0],
        format!(
            "log {} · grammar 1 · sequences 1..1 (1 migration)",
            dir.display()
        ),
        "status line 1:\n{}",
        r.out
    );
    assert!(
        lines[1].starts_with("state sha256:"),
        "status line 2 opens with the state hash:\n{}",
        r.out
    );
    assert!(
        lines[1].ends_with(" · 3 documents · 3 rules"),
        "status line 2 carries the census:\n{}",
        r.out
    );
}

#[test]
fn migrate_validate_exits_0_on_a_sound_log_and_reports_its_tally() {
    let dir = log("validclean");
    let r = run(&[
        "migrate",
        "validate",
        "--log-dir",
        &dir.display().to_string(),
    ]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    let tally = r.out.trim_end().lines().last().unwrap();
    assert!(
        tally.starts_with("mochiko-cli migrate validate · 0 rejecting · "),
        "a sound log rejects nothing:\n{}",
        r.out
    );
    // Advisory findings are never zero: the budget report is emitted per document unconditionally,
    // so the tally's second figure counts reports rather than problems.
    assert!(
        tally.ends_with(" advisory"),
        "the tally closes with the advisory count:\n{}",
        r.out
    );
    assert_eq!(
        r.out.lines().count(),
        1,
        "without --report only the tally prints:\n{}",
        r.out
    );
}

#[test]
fn migrate_validate_exits_1_on_a_rejecting_log_and_prints_every_finding() {
    let dir = scratch("validdirty");
    write_migration(&dir, "0001-genesis.yaml", LOG);
    write_migration(
        &dir,
        "0002-break.yaml",
        r#"
grammar: 1
id: 0002-break
sequence: 2
intent: Mint two rules, each wrong in its own way.
changes:
  - op: mint-rule
    schema: command/demo
    section: demo.sec.roles
    rule:
      id: demo.unregistered
      labels: [not-a-label]
      class: must
      text: A rule with an unregistered label.
  - op: mint-rule
    schema: command/demo
    section: demo.sec.roles
    rule:
      id: demo.unbound
      labels: [seats]
      class: must
      text: A rule naming ${nothing}.
"#,
    );
    let r = run(&[
        "migrate",
        "validate",
        "--log-dir",
        &dir.display().to_string(),
    ]);
    assert_eq!(r.code, 1);
    assert!(r.out.contains("not-a-label"), "finding 1:\n{}", r.out);
    assert!(r.out.contains("nothing"), "finding 2:\n{}", r.out);
    assert!(
        r.out.contains("· 2 rejecting ·"),
        "the tally counts both:\n{}",
        r.out
    );
}

#[test]
fn advisory_findings_print_only_under_report() {
    let dir = scratch("advisory");
    write_migration(&dir, "0001-genesis.yaml", LOG);
    write_migration(
        &dir,
        "0002-deixis.yaml",
        r#"
grammar: 1
id: 0002-deixis
sequence: 2
intent: Mint a rule whose text dangles outside its own block.
changes:
  - op: mint-rule
    schema: command/demo
    section: demo.sec.roles
    rule:
      id: demo.deictic
      labels: [seats]
      class: advisory
      text: Follow these rules as stated above.
"#,
    );
    let dir_string = dir.display().to_string();

    // The same corpus without the deictic rule, so the assertion is on the delta this rule makes
    // rather than on a figure the unconditional budget report would keep moving.
    let base = log("advisorybase");
    let baseline = advisory_count(&run(&[
        "migrate",
        "validate",
        "--log-dir",
        &base.display().to_string(),
    ]));

    let quiet = run(&["migrate", "validate", "--log-dir", &dir_string]);
    assert_eq!(quiet.code, 0, "an advisory finding never rejects");
    assert!(
        !quiet.out.contains("deictic"),
        "advisory findings stay quiet by default:\n{}",
        quiet.out
    );
    assert_eq!(
        advisory_count(&quiet),
        baseline + 1,
        "the tally counts the deictic finding even while withholding it:\n{}",
        quiet.out
    );

    let loud = run(&["migrate", "validate", "--report", "--log-dir", &dir_string]);
    assert_eq!(loud.code, 0);
    assert!(
        loud.out.contains("demo.deictic"),
        "--report prints them:\n{}",
        loud.out
    );
    assert_eq!(
        advisory_count(&loud),
        baseline + 1,
        "--report changes what is printed, never what is counted:\n{}",
        loud.out
    );
}

/// The advisory figure from a `migrate validate` tally line.
fn advisory_count(run: &Run) -> usize {
    let tally = run.out.trim_end().lines().last().expect("a tally line");
    tally
        .rsplit(" · ")
        .next()
        .and_then(|part| part.trim_end_matches(" advisory").parse().ok())
        .unwrap_or_else(|| panic!("no advisory count in: {tally}"))
}

// ---------------------------------------------------------------------------
// resolution: the plugin version and the log directory
// ---------------------------------------------------------------------------

/// A scratch plugin root: `.claude-plugin/plugin.json` plus its own `migrations/` directory.
fn plugin_root(tag: &str, version: &str) -> PathBuf {
    let root = scratch(tag);
    std::fs::create_dir_all(root.join(".claude-plugin")).expect("manifest dir is creatable");
    std::fs::write(
        root.join(".claude-plugin/plugin.json"),
        format!("{{\n  \"name\": \"mochiko\",\n  \"version\": \"{version}\"\n}}\n"),
    )
    .expect("manifest is writable");
    std::fs::create_dir_all(root.join("migrations")).expect("log dir is creatable");
    write_migration(&root.join("migrations"), "0001-genesis.yaml", LOG);
    root
}

#[test]
fn the_plugin_version_comes_from_the_plugin_root_and_reads_unknown_without_one() {
    let root = plugin_root("pluginver", "0.103.0");
    let with = run(&[
        "rules",
        "demo",
        "--section",
        "preamble",
        "--plugin-root",
        &root.display().to_string(),
    ]);
    assert_eq!(with.code, 0, "{}", with.err);
    assert!(
        with.out
            .lines()
            .next()
            .unwrap()
            .ends_with("· plugin 0.103.0"),
        "the head line carries the plugin version:\n{}",
        with.out
    );

    let dir = log("nopluginroot");
    let without = run(&[
        "rules",
        "demo",
        "--section",
        "preamble",
        "--log-dir",
        &dir.display().to_string(),
    ]);
    assert_eq!(without.code, 0, "{}", without.err);
    assert!(
        without
            .out
            .lines()
            .next()
            .unwrap()
            .ends_with("· plugin unknown"),
        "no plugin root reads unknown:\n{}",
        without.out
    );
}

#[test]
fn an_unreadable_plugin_manifest_reads_unknown_rather_than_halting() {
    let root = plugin_root("badmanifest", "0.0.0");
    std::fs::write(root.join(".claude-plugin/plugin.json"), "{ not json").expect("writable");
    let r = run(&[
        "rules",
        "demo",
        "--section",
        "preamble",
        "--plugin-root",
        &root.display().to_string(),
    ]);
    assert_eq!(
        r.code, 0,
        "a bad manifest never blocks delivery:\n{}",
        r.err
    );
    assert!(r.out.lines().next().unwrap().ends_with("· plugin unknown"));
}

#[test]
fn the_log_directory_resolves_flag_first_then_the_plugin_root() {
    // The plugin root carries a log; an explicit --log-dir must win over it.
    let root = plugin_root("orderflag", "1.2.3");
    let other = log("orderother");
    let r = run(&[
        "migrate",
        "status",
        "--plugin-root",
        &root.display().to_string(),
        "--log-dir",
        &other.display().to_string(),
    ]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(
        r.out.starts_with(&format!("log {}", other.display())),
        "--log-dir wins over the plugin root:\n{}",
        r.out
    );

    let from_root = run(&[
        "migrate",
        "status",
        "--plugin-root",
        &root.display().to_string(),
    ]);
    assert_eq!(from_root.code, 0, "{}", from_root.err);
    assert!(
        from_root
            .out
            .starts_with(&format!("log {}", root.join("migrations").display())),
        "the plugin root supplies the log directory:\n{}",
        from_root.out
    );
}

/// Limb 3 of 4: the environment beats the working directory.
///
/// The child's working directory carries a `migrations/` of its own, so this pins the ordering
/// rather than merely showing the variable is read at all — a test run from a directory with no
/// log would pass either way.
#[test]
fn the_environment_beats_the_working_directory() {
    let named = log("orderenv");
    let cwd = dir_with_log("orderenvcwd");
    let r = run_binary(
        &["migrate", "status"],
        &cwd,
        &[(LOG_DIR_ENV, Some(&named.display().to_string()))],
    );
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(
        r.out.starts_with(&format!("log {}", named.display())),
        "the environment names the log, not ./{LOG_DIR_NAME}:\n{}",
        r.out
    );
}

/// Limb 2 of 4: a plugin root beats the environment.
#[test]
fn the_plugin_root_beats_the_environment() {
    let root = plugin_root("orderrootenv", "1.2.3");
    let named = log("orderrootenvother");
    let r = run_binary(
        &[
            "migrate",
            "status",
            "--plugin-root",
            &root.display().to_string(),
        ],
        &scratch("orderrootenvcwd"),
        &[(LOG_DIR_ENV, Some(&named.display().to_string()))],
    );
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(
        r.out
            .starts_with(&format!("log {}", root.join(LOG_DIR_NAME).display())),
        "the plugin root names the log, not the environment:\n{}",
        r.out
    );
}

/// Limb 4 of 4: with no flag, no plugin root and no variable, `./migrations` is the last resort.
#[test]
fn the_working_directory_is_the_last_resort() {
    let cwd = dir_with_log("ordercwd");
    let r = run_binary(&["migrate", "status"], &cwd, &[(LOG_DIR_ENV, None)]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(
        r.out.starts_with(&format!("log {LOG_DIR_NAME} ")),
        "the working directory supplies the log as a relative path:\n{}",
        r.out
    );
}

/// With nothing to resolve from, the last resort still fails honestly rather than silently.
#[test]
fn the_last_resort_names_the_directory_it_looked_in_when_there_is_none() {
    let cwd = scratch("ordernone");
    let r = run_binary(&["migrate", "status"], &cwd, &[(LOG_DIR_ENV, None)]);
    assert_eq!(r.code, 1, "{}{}", r.out, r.err);
    assert!(
        r.err.contains(LOG_DIR_NAME),
        "the halt names the directory it looked in:\n{}",
        r.err
    );
}

// ---------------------------------------------------------------------------
// the shipped log (present from P3's genesis onward)
// ---------------------------------------------------------------------------

/// The repository's own log, or `None` before P3's genesis migration exists.
fn shipped_log() -> Option<PathBuf> {
    let dir = Path::new(REPO_ROOT).join(LOG_DIR_NAME);
    if dir.join("0001-genesis.yaml").is_file() {
        Some(dir)
    } else {
        eprintln!(
            "SKIPPED: {} does not exist yet — P3 generates it; this test is dark until then",
            dir.join("0001-genesis.yaml").display()
        );
        None
    }
}

/// Renders every section of every primitive in the repository's own log.
///
/// The walk goes through the library against **one** replayed state. Driving 252 renders through
/// `dispatch` would reload and revalidate the whole 618 KB log 252 times — about 25 seconds for a
/// property none of those reloads test, since one load either succeeds for every render or fails
/// for all of them. The dispatch path over this same log is covered by its own test below, on a
/// sample; here the subject is the render, so the state is loaded once and every section is
/// rendered from it.
#[test]
fn the_shipped_log_renders_every_section_of_every_primitive() {
    let Some(log_dir) = shipped_log() else {
        return;
    };
    let state = mochiko_cli::replay::load(&log_dir).unwrap_or_else(|findings| {
        let lines: Vec<String> = findings.iter().map(ToString::to_string).collect();
        panic!(
            "the shipped log should be deliverable:\n{}",
            lines.join("\n")
        )
    });
    // The values in the triple are pinned by `the_shipped_log_is_reachable_through_the_binary`
    // below, which resolves them the way a real fire does. Here they are fixed so the walk asserts
    // the render's shape rather than the resolution's.
    let ctx = mochiko_cli::render::Context {
        binary: "0.1.0".to_string(),
        grammar: 1,
        plugin: "0.103.0".to_string(),
    };

    let mut rendered = 0usize;
    for (doc, document) in &state.docs {
        let Some(schema) = document.as_rules() else {
            continue;
        };
        if !matches!(
            doc.kind,
            mochiko_cli::model::DocKind::Command | mochiko_cli::model::DocKind::Skill
        ) {
            continue;
        }
        for section in std::iter::once(None).chain(schema.sections.iter().map(Some)) {
            let (id, live) = match section {
                None => ("preamble".to_string(), 0),
                Some(section) => (section.id.clone(), section.rules.len()),
            };
            let out = mochiko_cli::render::section(&state, doc, &id, &ctx)
                .unwrap_or_else(|e| panic!("{} · {id}: {e}", doc.name));
            assert_eq!(
                out.lines().next().unwrap(),
                format!(
                    "mochiko-cli rules {} · section {id} · binary 0.1.0 · grammar 1 · plugin 0.103.0",
                    doc.name
                ),
                "{} · {id}: head line",
                doc.name
            );
            assert_eq!(
                out.trim_end().lines().last().unwrap(),
                format!("mochiko-cli rules end · {} · {id} · {live} rules", doc.name),
                "{} · {id}: tail line",
                doc.name
            );
            rendered += 1;
        }
    }
    assert!(rendered > 0, "the shipped log carries no primitives");
    eprintln!("rendered {rendered} sections from the shipped log");
}

/// The same log through the whole binary — argument parsing, both resolutions, the render, the
/// exit code — on a sample rather than a walk.
///
/// One command and one skill, each preamble and first section, is enough to pin what the library
/// walk cannot: that a real invocation resolves the plugin version off the manifest and the
/// grammar off the log, and prints the triple the `.md` halt clause keys on.
#[test]
fn the_shipped_log_is_reachable_through_the_binary() {
    let Some(log_dir) = shipped_log() else {
        return;
    };
    let plugin_root = Path::new(REPO_ROOT).join("plugins/mochiko");
    let expected_plugin = std::fs::read_to_string(plugin_root.join(".claude-plugin/plugin.json"))
        .ok()
        .and_then(|text| serde_norway::from_str::<serde_norway::Value>(&text).ok())
        .and_then(|value| {
            value
                .get("version")
                .and_then(|v| v.as_str())
                .map(String::from)
        })
        .expect("the shipped plugin manifest declares a version");

    let state = mochiko_cli::replay::load(&log_dir).expect("the shipped log is deliverable");
    let root = plugin_root.display().to_string();
    let log = log_dir.display().to_string();
    let mut sampled = 0usize;

    for kind in [
        mochiko_cli::model::DocKind::Command,
        mochiko_cli::model::DocKind::Skill,
    ] {
        let (doc, schema) = state
            .docs
            .iter()
            .filter(|(doc, _)| doc.kind == kind)
            .find_map(|(doc, document)| document.as_rules().map(|schema| (doc, schema)))
            .unwrap_or_else(|| panic!("the shipped log carries at least one {kind}"));

        for id in ["preamble".to_string(), schema.sections[0].id.clone()] {
            let r = run(&[
                "rules",
                &doc.name,
                "--section",
                &id,
                "--plugin-root",
                &root,
                "--log-dir",
                &log,
            ]);
            assert_eq!(r.code, 0, "{} · {id}:\n{}", doc.name, r.err);
            assert_eq!(
                r.out.lines().next().unwrap(),
                format!(
                    "mochiko-cli rules {} · section {id} · binary {} · grammar 1 · plugin {expected_plugin}",
                    doc.name,
                    env!("CARGO_PKG_VERSION")
                ),
                "{} · {id}: the resolved version triple",
                doc.name
            );
            assert!(
                r.out
                    .trim_end()
                    .lines()
                    .last()
                    .unwrap()
                    .starts_with(&format!("mochiko-cli rules end · {} · {id} · ", doc.name)),
                "{} · {id}: tail line",
                doc.name
            );
            sampled += 1;
        }
    }
    assert_eq!(
        sampled, 4,
        "one command and one skill, preamble and first section"
    );
}
