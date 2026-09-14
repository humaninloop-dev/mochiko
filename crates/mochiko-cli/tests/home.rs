//! The `home` document kind: how it enters the log, how it replays, and how a path resolves
//! against the homes the log declares.
//!
//! A `home` document carries no grammar of its own in the store — it is opaque YAML like a
//! template, decoded into [`mochiko_cli::home::Home`] at the point of use. These tests grade the
//! decode, the segment-token vocabulary, and the resolution of a repository-relative path to one
//! home and one verdict about the name inside it.
//!
//! Every fixture is written under `CARGO_TARGET_TMPDIR`, inside `target/`.

use mochiko_cli::home::{self, Bounds, Homes, Resolution};
use mochiko_cli::migration;
use mochiko_cli::model::{DocKind, DocRef, Document};
use mochiko_cli::render;
use mochiko_cli::replay::{self, State};
use mochiko_cli::views;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn log_dir(tag: &str) -> PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("home-{tag}-{n}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("fixture log dir is creatable");
    dir
}

fn write(dir: &Path, name: &str, body: &str) {
    let stamped = migration::with_hash(name, body)
        .unwrap_or_else(|e| panic!("fixture {name} is not a well-formed migration: {e}"));
    std::fs::write(dir.join(name), stamped).expect("fixture migration is writable");
}

/// A log carrying three homes: a `<FEAT-ID>` home with a closed deliverable set and a reports
/// directory, a nested `stories` home under it, and a dated desk home.
pub const HOMES: &str = r#"
grammar: 1
id: 0001-homes
sequence: 1
intent: Declare three artifact homes for the resolution tests.
changes:
  - op: import-document
    kind: home
    name: feature
    content:
      home: feature
      title: Feature work home
      path: [".mochiko", "features", "<FEAT-ID>"]
      bounds: template
      deliverables:
        - file: tasks.md
          template: tasks
        - file: gates.md
          max_lines: 120
        - file: implement-log.md
          form: log
          entry_max_lines: 40
      subdirs: ["stories"]
      reports:
        envelope: report-envelope
        by_type: {}
  - op: import-document
    kind: home
    name: feature-stories
    content:
      home: feature-stories
      title: Per-story files
      path: [".mochiko", "features", "<FEAT-ID>", "stories"]
      bounds: whole-file
      deliverables:
        - file: "US-<n>.md"
          max_lines: 80
  - op: import-document
    kind: home
    name: desk
    content:
      home: desk
      title: Specify desk
      path: [".mochiko", "features", "desk", "<date-slug>"]
      bounds: elsewhere
      bounds_cite: ".mochiko/memory/knowledge-management.md"
      deliverables:
        - file: derivation.md
  - op: import-document
    kind: template
    name: tasks
    content:
      template: tasks
      title: Implementation Cycles
      form: artifact-format.md
      register: full
      overview: Cycle cards.
      sections:
        - name: Overview
          heading: Overview
          required: true
          max_lines: 12
          contract: The metric table.
          check: Is the Overview present?
      skeleton: |
        ## Overview
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
          required: [report, feature]
          enum:
            report: [cycle, verification, final-validation, review, feasibility, disclosure]
      sections: []
      skeleton: |
        ---
        report: cycle
        feature: FEAT-001
        ---
"#;

fn state(tag: &str, body: &str) -> State {
    let dir = log_dir(tag);
    write(&dir, "0001-homes.yaml", body);
    replay::load(&dir).unwrap_or_else(|findings| {
        let lines: Vec<String> = findings.iter().map(ToString::to_string).collect();
        panic!("fixture log replays:\n{}", lines.join("\n"))
    })
}

// ---------------------------------------------------------------------------
// the kind
// ---------------------------------------------------------------------------

#[test]
fn a_home_document_enters_the_log_and_replays_as_an_opaque_document() {
    let state = state("kind", HOMES);
    let doc = DocRef::new(DocKind::Home, "feature");
    assert!(
        matches!(state.docs.get(&doc), Some(Document::Opaque(_))),
        "a home document replays opaquely, like a template"
    );
}

#[test]
fn the_home_kind_parses_from_its_wire_spelling_and_is_replaceable() {
    assert_eq!(DocKind::parse("home"), Some(DocKind::Home));
    assert_eq!(DocKind::Home.as_str(), "home");
    assert!(
        DocKind::Home.is_replaceable(),
        "a whole home is replaced at once, like a template — the budget table is re-keyed wholesale"
    );
    assert!(!DocKind::Home.is_rule_bearing());
    assert!(!DocKind::Home.is_registry());
    assert!(
        DocKind::ALL.contains(&DocKind::Home),
        "the kind must be in ALL or `kind: home` cannot parse"
    );
}

#[test]
fn a_home_documents_view_lands_on_its_own_shelf() {
    let doc = DocRef::new(DocKind::Home, "feature");
    assert_eq!(views::view_path(&doc), PathBuf::from("homes/feature.yaml"));
}

#[test]
fn the_replay_of_a_home_log_is_deterministic() {
    let first = state("determinism-a", HOMES);
    let second = state("determinism-b", HOMES);
    assert_eq!(
        first.content_hash(),
        second.content_hash(),
        "two replays of one log agree on the state hash"
    );
}

#[test]
fn a_home_document_is_replaced_wholesale_by_a_later_migration() {
    let dir = log_dir("replace");
    write(&dir, "0001-homes.yaml", HOMES);
    write(
        &dir,
        "0002-rekey.yaml",
        r#"
grammar: 1
id: 0002-rekey
sequence: 2
intent: Re-key one home's budget table, which is a wholesale replacement.
changes:
  - op: replace-document
    kind: home
    name: desk
    content:
      home: desk
      title: Specify desk
      path: [".mochiko", "features", "desk", "<date-slug>"]
      bounds: whole-file
      deliverables:
        - file: derivation.md
          max_lines: 300
"#,
    );
    let state = replay::load(&dir).expect("the two-migration log replays");
    let homes = homes(&state);
    let desk = homes
        .iter()
        .find(|h| h.home == "desk")
        .expect("the desk home survives the replacement");
    assert_eq!(desk.bounds, Bounds::WholeFile);
    assert_eq!(desk.deliverables[0].max_lines, Some(300));
}

// ---------------------------------------------------------------------------
// the segment-token vocabulary
// ---------------------------------------------------------------------------

#[test]
fn each_segment_token_matches_exactly_what_it_declares() {
    // (token, matching segments, non-matching segments)
    let cases: [(&str, &[&str], &[&str]); 6] = [
        (
            "<slug>",
            &["hook-enforced-artifact-schema", "a", "two-words"],
            &["Not-Kebab", "has space", "", "trailing-"],
        ),
        (
            "<date-slug>",
            &["2026-09-13-sandbox-runner", "2026-01-01-a"],
            &["2026-09-13", "26-09-13-x", "2026-13-01-x", "sandbox-runner"],
        ),
        (
            "<FEAT-ID>",
            &["FEAT-001", "FEAT-001-tenancy", "FEAT-1234"],
            &["FEAT-1", "FEAT-", "B53", "feat-001", "EPIC-001"],
        ),
        (
            "<EPIC-ID>",
            &["EPIC-002", "EPIC-002-platform"],
            &["EPIC-2", "FEAT-002"],
        ),
        (
            "<AX-ID>",
            &["AX-001-tenancy", "AX-042-sso"],
            &["AX-001", "AX-1-x", "AX-tenancy"],
        ),
        ("<n>", &["1", "42", "007"], &["", "1a", "-1", "US-1"]),
    ];
    for (token, matching, rejecting) in cases {
        for segment in matching {
            assert!(
                home::segment_matches(token, segment),
                "{token} should match {segment:?}"
            );
        }
        for segment in rejecting {
            assert!(
                !home::segment_matches(token, segment),
                "{token} should not match {segment:?}"
            );
        }
    }
}

#[test]
fn a_literal_segment_matches_only_itself_and_an_unknown_token_matches_nothing() {
    assert!(home::segment_matches(".mochiko", ".mochiko"));
    assert!(!home::segment_matches(".mochiko", "mochiko"));
    // `<any>` is the deliberate wildcard; an undeclared token is not silently one.
    assert!(home::segment_matches("<any>", "anything-at-all"));
    assert!(!home::segment_matches("<unknown-token>", "anything"));
    assert!(
        !home::segment_matches("<any>", ""),
        "no token matches an empty segment"
    );
}

// ---------------------------------------------------------------------------
// resolution
// ---------------------------------------------------------------------------

/// The decoded home set for a state. Held by the caller, because a `Resolution` borrows it.
fn homes(state: &State) -> Homes {
    Homes::load(state)
}

#[test]
fn a_declared_deliverable_resolves_to_its_home_and_its_entry() {
    let state = state("resolve-file", HOMES);
    let homes = homes(&state);
    match homes.resolve(Path::new(".mochiko/features/FEAT-001/tasks.md")) {
        Resolution::File { home, deliverable } => {
            assert_eq!(home.home, "feature");
            assert_eq!(deliverable.file, "tasks.md");
            assert_eq!(deliverable.template.as_deref(), Some("tasks"));
        }
        other => panic!("expected a declared deliverable, got {other:?}"),
    }
}

#[test]
fn a_patterned_deliverable_name_resolves_by_its_token() {
    let state = state("resolve-pattern", HOMES);
    let homes = homes(&state);
    match homes.resolve(Path::new(".mochiko/features/FEAT-001/stories/US-7.md")) {
        Resolution::File { home, deliverable } => {
            assert_eq!(home.home, "feature-stories");
            assert_eq!(deliverable.file, "US-<n>.md");
        }
        other => panic!("expected the patterned story file, got {other:?}"),
    }
    match homes.resolve(Path::new(".mochiko/features/FEAT-001/stories/notes.md")) {
        Resolution::UndeclaredFile { home, name } => {
            assert_eq!(home.home, "feature-stories");
            assert_eq!(name, "notes.md");
        }
        other => panic!("a name outside the pattern is undeclared, got {other:?}"),
    }
}

#[test]
fn the_longest_literal_prefix_wins_when_two_homes_could_match() {
    let state = state("resolve-longest", HOMES);
    let homes = homes(&state);
    // `desk` is a literal where `feature` carries `<FEAT-ID>`, so the desk home wins its own tree.
    match homes.resolve(Path::new(
        ".mochiko/features/desk/2026-09-09-sandbox-runner/derivation.md",
    )) {
        Resolution::File { home, .. } => assert_eq!(home.home, "desk"),
        other => panic!("expected the desk home, got {other:?}"),
    }
}

#[test]
fn an_undeclared_name_in_a_home_resolves_as_undeclared_and_names_the_home() {
    let state = state("resolve-undeclared", HOMES);
    let homes = homes(&state);
    match homes.resolve(Path::new(".mochiko/features/FEAT-001/build-order.md")) {
        Resolution::UndeclaredFile { home, name } => {
            assert_eq!(home.home, "feature");
            assert_eq!(name, "build-order.md");
        }
        other => panic!("expected an undeclared name, got {other:?}"),
    }
}

#[test]
fn any_name_under_a_declared_reports_directory_resolves_as_a_report() {
    let state = state("resolve-report", HOMES);
    let homes = homes(&state);
    match homes.resolve(Path::new(
        ".mochiko/features/FEAT-001/reports/round-3-whatever.md",
    )) {
        Resolution::Report {
            home,
            reports,
            name,
        } => {
            assert_eq!(home.home, "feature");
            assert_eq!(reports.envelope, "report-envelope");
            assert_eq!(name, "round-3-whatever.md");
        }
        other => panic!("expected a report, got {other:?}"),
    }
}

#[test]
fn a_reports_directory_is_not_open_in_a_home_that_declares_none() {
    let state = state("resolve-no-reports", HOMES);
    let homes = homes(&state);
    match homes.resolve(Path::new(
        ".mochiko/features/desk/2026-09-09-sandbox-runner/reports/x.md",
    )) {
        Resolution::UndeclaredSubdir { home, subdir } => {
            assert_eq!(home.home, "desk");
            assert_eq!(subdir, "reports");
        }
        other => panic!("expected an undeclared sub-dir, got {other:?}"),
    }
}

#[test]
fn a_declared_subdir_defers_to_its_own_home_and_an_undeclared_one_is_denied() {
    let state = state("resolve-subdir", HOMES);
    let homes = homes(&state);
    // `prototype` is not in the feature home's `subdirs`, so it is an undeclared sub-dir.
    match homes.resolve(Path::new(".mochiko/features/FEAT-001/prototype/index.html")) {
        Resolution::UndeclaredSubdir { home, subdir } => {
            assert_eq!(home.home, "feature");
            assert_eq!(subdir, "prototype");
        }
        other => panic!("expected an undeclared sub-dir, got {other:?}"),
    }
    // A declared sub-dir with no home doc of its own defers rather than denying: nothing in the
    // log governs it yet, and inventing a rule here is what D4f forbids.
    let deferring = state_with_deferring_subdir();
    let deferring_homes = Homes::load(&deferring);
    match deferring_homes.resolve(Path::new(".mochiko/features/FEAT-001/stories/deeper/x.md")) {
        Resolution::Deferred { home, subdir } => {
            assert_eq!(home.home, "feature-stories");
            assert_eq!(subdir, "deeper");
        }
        other => panic!("expected a deferral, got {other:?}"),
    }
}

fn state_with_deferring_subdir() -> State {
    state(
        "defer",
        &HOMES.replace(
            "      deliverables:\n        - file: \"US-<n>.md\"\n          max_lines: 80",
            "      deliverables:\n        - file: \"US-<n>.md\"\n          max_lines: 80\n      subdirs: [\"deeper\"]",
        ),
    )
}

#[test]
fn a_path_under_no_declared_home_resolves_outside() {
    let state = state("resolve-outside", HOMES);
    let homes = homes(&state);
    for path in [
        "docs/readme.md",
        "README.md",
        ".mochiko/specs/some-spec/spec.md",
        ".mochiko/features/FEAT-001",
    ] {
        assert!(
            matches!(homes.resolve(Path::new(path)), Resolution::Outside),
            "{path} is governed by no declared home"
        );
    }
}

#[test]
fn a_traversal_or_absolute_path_never_resolves_into_a_home() {
    let state = state("resolve-traversal", HOMES);
    let homes = homes(&state);
    for path in [
        "../mochiko/features/FEAT-001/tasks.md",
        ".mochiko/features/../features/FEAT-001/tasks.md",
        "/etc/passwd",
    ] {
        assert!(
            matches!(homes.resolve(Path::new(path)), Resolution::Outside),
            "{path} must not resolve into a home"
        );
    }
}

// ---------------------------------------------------------------------------
// the hard set over home documents
// ---------------------------------------------------------------------------

/// Replay a log and return every rejecting finding's code, or panic naming the load failure.
///
/// A home's own constraints are the binary validating its own data, which is the admitted side of
/// the bright line: the tool never grades an artifact here, it grades the log.
fn rejecting_codes(tag: &str, body: &str) -> Vec<String> {
    let dir = log_dir(tag);
    write(&dir, "0001-homes.yaml", body);
    let findings = match replay::load_full(&dir) {
        Ok(replay) => replay.all_findings(),
        Err(findings) => findings,
    };
    findings
        .iter()
        .filter(|f| f.is_rejecting())
        .map(|f| f.code.as_str().to_string())
        .collect()
}

/// A log carrying one template, so a kind-to-template binding has something to resolve against.
const WITH_TEMPLATE: &str = r#"
  - op: import-document
    kind: template
    name: tasks
    content:
      template: tasks
      title: Implementation Cycles
      form: artifact-format.md
      register: full
      overview: Cycle cards.
      sections:
        - name: Overview
          heading: Overview
          required: true
          max_lines: 12
          contract: The metric table.
          check: Is the Overview present?
      skeleton: |
        ## Overview
"#;

fn home_log(body: &str) -> String {
    format!(
        "grammar: 1\nid: 0001-homes\nsequence: 1\nintent: A home fixture for the hard set.\nchanges:{body}"
    )
}

#[test]
fn a_home_declaring_an_unknown_segment_token_is_rejected() {
    let codes = rejecting_codes(
        "validate-token",
        &home_log(
            r#"
  - op: import-document
    kind: home
    name: bad
    content:
      home: bad
      title: Unknown token
      path: [".mochiko", "<not-a-token>"]
      bounds: whole-file
      deliverables:
        - file: a.md
          max_lines: 10
"#,
        ),
    );
    assert!(
        codes.contains(&"home-pattern".to_string()),
        "an unknown token must reject, never behave as a wildcard; got {codes:?}"
    );
}

#[test]
fn two_homes_declaring_one_path_are_rejected_rather_than_resolved_by_order() {
    let codes = rejecting_codes(
        "validate-duplicate",
        &home_log(
            r#"
  - op: import-document
    kind: home
    name: first
    content:
      home: first
      title: First
      path: [".mochiko", "features", "<FEAT-ID>"]
      bounds: whole-file
      deliverables:
        - file: a.md
          max_lines: 10
  - op: import-document
    kind: home
    name: second
    content:
      home: second
      title: Second
      path: [".mochiko", "features", "<FEAT-ID>"]
      bounds: whole-file
      deliverables:
        - file: b.md
          max_lines: 10
"#,
        ),
    );
    assert!(
        codes.contains(&"home-duplicate".to_string()),
        "an ambiguous pattern is a log defect, not a runtime coin-flip; got {codes:?}"
    );
}

#[test]
fn a_binding_to_a_template_the_log_does_not_carry_is_rejected() {
    let codes = rejecting_codes(
        "validate-binding",
        &home_log(
            r#"
  - op: import-document
    kind: home
    name: bad
    content:
      home: bad
      title: Dangling binding
      path: [".mochiko", "x"]
      bounds: template
      deliverables:
        - file: a.md
          template: no-such-template
"#,
        ),
    );
    assert!(
        codes.contains(&"home-binding".to_string()),
        "a dangling kind-to-template binding is a dead pointer; got {codes:?}"
    );
}

#[test]
fn a_reports_envelope_the_log_does_not_carry_is_rejected() {
    let codes = rejecting_codes(
        "validate-envelope",
        &home_log(
            r#"
  - op: import-document
    kind: home
    name: bad
    content:
      home: bad
      title: Dangling envelope
      path: [".mochiko", "x"]
      bounds: whole-file
      deliverables:
        - file: a.md
          max_lines: 10
      reports:
        envelope: no-such-envelope
"#,
        ),
    );
    assert!(
        codes.contains(&"home-binding".to_string()),
        "the reports envelope resolves like any other binding; got {codes:?}"
    );
}

#[test]
fn bounds_elsewhere_without_a_citation_is_rejected() {
    let codes = rejecting_codes(
        "validate-cite",
        &home_log(
            r#"
  - op: import-document
    kind: home
    name: bad
    content:
      home: bad
      title: Uncited exemption
      path: [".mochiko", "x"]
      bounds: elsewhere
      deliverables:
        - file: a.md
"#,
        ),
    );
    assert!(
        codes.contains(&"home-bounds".to_string()),
        "the D4f exemption must cite the constraint home its bounds live in; got {codes:?}"
    );
}

#[test]
fn a_template_less_deliverable_with_no_whole_file_bound_is_rejected() {
    let codes = rejecting_codes(
        "validate-unbounded",
        &home_log(
            r#"
  - op: import-document
    kind: home
    name: bad
    content:
      home: bad
      title: Unbounded deliverable
      path: [".mochiko", "x"]
      bounds: whole-file
      deliverables:
        - file: a.md
"#,
        ),
    );
    assert!(
        codes.contains(&"home-bounds".to_string()),
        "a template-less deliverable takes a whole-file bound (D4f); got {codes:?}"
    );
}

#[test]
fn a_log_form_deliverable_with_no_per_entry_bound_is_rejected() {
    let codes = rejecting_codes(
        "validate-log-form",
        &home_log(
            r#"
  - op: import-document
    kind: home
    name: bad
    content:
      home: bad
      title: Unbounded log
      path: [".mochiko", "x"]
      bounds: whole-file
      deliverables:
        - file: implement-log.md
          form: log
"#,
        ),
    );
    assert!(
        codes.contains(&"home-bounds".to_string()),
        "an append-only log is bounded per entry (D4d); got {codes:?}"
    );
}

#[test]
fn a_home_document_that_does_not_decode_is_reported_rather_than_skipped() {
    let codes = rejecting_codes(
        "validate-shape",
        &home_log(
            r#"
  - op: import-document
    kind: home
    name: bad
    content:
      home: bad
      title: Missing its path
      bounds: whole-file
"#,
        ),
    );
    assert!(
        codes.contains(&"home-shape".to_string()),
        "a home the decoder cannot read must be a finding, never a silent skip; got {codes:?}"
    );
}

#[test]
fn a_well_formed_home_set_raises_no_rejecting_finding() {
    let codes = rejecting_codes(
        "validate-clean",
        &home_log(&format!(
            r#"
  - op: import-document
    kind: home
    name: feature
    content:
      home: feature
      title: Feature work home
      path: [".mochiko", "features", "<FEAT-ID>"]
      bounds: template
      deliverables:
        - file: tasks.md
          template: tasks
        - file: gates.md
          max_lines: 120
        - file: implement-log.md
          form: log
          entry_max_lines: 40
      subdirs: ["stories"]{WITH_TEMPLATE}"#
        )),
    );
    assert!(
        codes.is_empty(),
        "the well-formed set is clean; got {codes:?}"
    );
}

// ---------------------------------------------------------------------------
// the conformance block on a template, and its producer-view render
// ---------------------------------------------------------------------------

/// A template carrying a full conformance block: two headings with budgets, one heading-less
/// guidance section, required and enumerated frontmatter, and declared placeholder tokens.
const CONFORMING_TEMPLATE: &str = r#"
  - op: import-document
    kind: template
    name: demo
    content:
      template: demo
      title: Demo Artifact
      form: artifact-format.md
      register: full
      overview: A two-section artifact used to grade the conformance render.
      conformance:
        frontmatter:
          required: [feature, status]
          enum:
            status: [draft, accepted]
        placeholders: ["[entity]", "<feature-id>", "FEAT-XXX"]
        extra_headings: deny
      sections:
        - name: Header
          required: true
          contract: The title line and the frontmatter block. Governs no heading of its own.
          check: Is the header present?
        - name: Intent
          heading: Intent
          required: true
          max_lines: 6
          contract: One line per ruling.
          check: Is the Intent present?
        - name: Notes
          heading: Notes
          required: false
          max_lines: 20
          contract: Optional commentary.
          check: Are the Notes present when they apply?
      skeleton: |
        ---
        feature: FEAT-XXX
        status: draft
        ---

        ## Intent
"#;

fn producer_view(tag: &str, body: &str) -> String {
    let dir = log_dir(tag);
    write(&dir, "0001-homes.yaml", body);
    let state = replay::load(&dir).unwrap_or_else(|findings| {
        let lines: Vec<String> = findings.iter().map(ToString::to_string).collect();
        panic!("fixture log replays:\n{}", lines.join("\n"))
    });
    mochiko_cli::render::template_view(&state, "demo", false, &dir)
        .expect("the demo template renders")
}

#[test]
fn the_conformance_block_decodes_off_the_template() {
    let dir = log_dir("conformance-decode");
    write(&dir, "0001-homes.yaml", &home_log(CONFORMING_TEMPLATE));
    let state = replay::load(&dir).expect("the fixture replays");
    let template = mochiko_cli::render::template_of(&state, "demo").expect("the template decodes");

    let conformance = template
        .conformance
        .as_ref()
        .expect("the block is present on the template");
    assert_eq!(conformance.frontmatter.required, vec!["feature", "status"]);
    assert_eq!(
        conformance
            .frontmatter
            .enums
            .get("status")
            .map(Vec::as_slice),
        Some(["draft".to_string(), "accepted".to_string()].as_slice())
    );
    assert_eq!(
        conformance.placeholders,
        vec!["[entity]", "<feature-id>", "FEAT-XXX"]
    );
    assert!(conformance.denies_extra_headings());

    // The heading grammar comes off the sections, and a section may govern no heading at all —
    // `## Header` describes a title line, not a heading the artifact carries.
    let headings: Vec<&str> = template.declared_headings().map(|(h, _)| h).collect();
    assert_eq!(headings, vec!["Intent", "Notes"]);
    assert_eq!(template.required_headings(), vec!["Intent"]);
    assert_eq!(template.budget("Intent"), Some(6));
    assert_eq!(template.budget("Notes"), Some(20));
    assert_eq!(template.budget("Header"), None);
}

#[test]
fn a_template_with_no_conformance_block_carries_no_shape_rule() {
    let dir = log_dir("conformance-absent");
    write(
        &dir,
        "0001-homes.yaml",
        &home_log(
            r#"
  - op: import-document
    kind: template
    name: demo
    content:
      template: demo
      title: Bare Demo
      form: artifact-format.md
      register: full
      overview: No conformance block at all.
      sections:
        - name: Intent
          required: true
          contract: One line per ruling.
          check: Is the Intent present?
      skeleton: |
        ## Intent
"#,
        ),
    );
    let state = replay::load(&dir).expect("the fixture replays");
    let template = mochiko_cli::render::template_of(&state, "demo").expect("the template decodes");
    assert!(
        template.conformance.is_none(),
        "absent means no shape check at all (D4f), never an empty-but-present block"
    );
    assert!(
        template.declared_headings().next().is_none(),
        "a section with no `heading` governs none, so a block-less template declares no grammar"
    );
}

#[test]
fn the_producer_view_prints_the_conformance_block_before_the_skeleton() {
    let view = producer_view("conformance-view", &home_log(CONFORMING_TEMPLATE));
    let conformance_at = view
        .find("## Conformance")
        .expect("the producer view carries a Conformance block");
    let skeleton_at = view
        .find("## Skeleton")
        .expect("the producer view still carries its skeleton");
    assert!(
        conformance_at < skeleton_at,
        "the budgets are read before the skeleton is filled in"
    );

    for expected in [
        "Required frontmatter: `feature`, `status`",
        "`status` is one of: `draft`, `accepted`",
        "Required headings, in order: `## Intent`",
        "An undeclared `##` heading is denied.",
        "`## Intent` — 6 lines",
        "`## Notes` — 20 lines (optional)",
        "Placeholder tokens that must not survive: `[entity]`, `<feature-id>`, `FEAT-XXX`",
    ] {
        assert!(
            view.contains(expected),
            "the conformance block states {expected:?}\n--- view ---\n{view}"
        );
    }
}

#[test]
fn the_producer_view_of_a_block_less_template_is_unchanged() {
    let bare = r#"
  - op: import-document
    kind: template
    name: demo
    content:
      template: demo
      title: Bare Demo
      form: artifact-format.md
      register: full
      overview: No conformance block at all.
      sections:
        - name: Intent
          required: true
          contract: One line per ruling.
          check: Is the Intent present?
      skeleton: |
        ## Intent
"#;
    let view = producer_view("conformance-bare", &home_log(bare));
    assert!(
        !view.contains("## Conformance"),
        "a template with no block gains no section in its producer view"
    );
}

#[test]
fn the_checklist_view_is_untouched_by_the_conformance_block() {
    let dir = log_dir("conformance-check-view");
    write(&dir, "0001-homes.yaml", &home_log(CONFORMING_TEMPLATE));
    let state = replay::load(&dir).expect("the fixture replays");
    let view = mochiko_cli::render::template_view(&state, "demo", true, &dir)
        .expect("the checklist renders");
    assert!(
        !view.contains("Conformance"),
        "the `--check` view is a mirror checklist and gains nothing here"
    );
    assert!(view.contains("- [ ] Is the Intent present?"));
}

// ---------------------------------------------------------------------------
// the authoring-time rule, as it renders in a producing primitive (D10 / V1)
// ---------------------------------------------------------------------------

/// A command schema carrying the authoring-time rule D1a's second arm describes.
///
/// Wave 3's census migration mints the real rule into every producing command and skill. This is a
/// fixture primitive standing in for it, so wave 1 can prove the rule *renders* — that a seat
/// reading its rules before the first write is actually told which subcommand to run and what to
/// hold. Without this golden the channel would be unproven until wave 3, which is the sequencing
/// gap the verify pass raised as V1.
const PRODUCING_PRIMITIVE: &str = r#"
  - op: import-document
    kind: command-labels
    name: command-labels
    content:
      kind: command-labels
      labels:
        seats: Seat wiring.
        binding: A binding obligation.
  - op: import-document
    kind: command
    name: implement
    content:
      kind: command
      command: implement
      sections:
        - id: impl.sec.roles
          title: Roles
          intent: Seat wiring.
          rules:
            - id: impl.lead
              labels: [seats]
              class: must
              text: The lead plans the run.
        - id: impl.sec.reserved
          title: Reserved
          intent: Reserved to the user.
          note: Nothing reserved.
          rules: []
        - id: impl.sec.tools
          title: Tools
          intent: The skills reached for.
          note: No skills.
          rules: []
        - id: impl.sec.ways-of-working
          title: Ways of working
          intent: How the run proceeds.
          rules:
            - id: impl.artifact-home
              labels: [binding]
              class: floor
              kind: binding
              text: >-
                Before the first write to an artifact home, render `mochiko-cli home <path>` and
                hold what it returns: the declared home, its file set, the template bound to each
                file, and the bound. A name the set does not carry is raised upstream, never
                minted from a sibling run.
              anchor: 2026-09-13 hook-enforced-artifact-schema D1
        - id: impl.sec.boundaries
          title: Boundaries
          intent: The floor.
          rules:
            - id: impl.boundary
              labels: [seats]
              class: floor
              text: The user rules acceptance.
        - id: impl.sec.fail-conditions
          title: Not done
          intent: The fail set.
          rules:
            - id: impl.fail.unaccepted
              labels: [seats]
              class: floor
              kind: fail
              enforces: [impl.boundary]
              text: An unaccepted record.
"#;

#[test]
fn the_authoring_time_rule_renders_in_its_producing_primitives_section() {
    let dir = log_dir("authoring-rule");
    write(&dir, "0001-homes.yaml", &home_log(PRODUCING_PRIMITIVE));
    let state = replay::load(&dir).expect("the fixture primitive replays");
    let ctx = mochiko_cli::render::Context {
        binary: "0.1.0".to_string(),
        grammar: 1,
        plugin: "unknown".to_string(),
    };
    let doc = DocRef::new(DocKind::Command, "implement");
    let rendered = mochiko_cli::render::section(&state, &doc, "impl.sec.ways-of-working", &ctx)
        .expect("the section renders");

    assert!(
        rendered.contains("mochiko-cli home <path>"),
        "the seat is told the subcommand to run: {rendered}"
    );
    for expected in ["declared home", "file set", "bound"] {
        assert!(
            rendered.contains(expected),
            "the rule states what to hold ({expected}): {rendered}"
        );
    }
    assert!(
        rendered.contains("class: floor"),
        "the authoring-time channel is a floor, not advice: {rendered}"
    );
    assert!(
        rendered.starts_with("mochiko-cli rules implement · section impl.sec.ways-of-working"),
        "the head line is unchanged by this wave: {rendered}"
    );

    // The ruling anchor is provenance for the record layer, not delivery content, so it lives on
    // the rule in state and is deliberately absent from the render. Asserting it here is what makes
    // the rule's protected exit real: an anchored rule leaves only through `supersede-rule`.
    let Some(mochiko_cli::model::Document::Rules(schema)) = state.docs.get(&doc) else {
        panic!("the fixture primitive decodes as a rule schema");
    };
    let rule = schema
        .sections
        .iter()
        .flat_map(|section| section.rules.iter())
        .find(|rule| rule.id == "impl.artifact-home")
        .expect("the authoring-time rule is in state");
    assert_eq!(
        rule.anchor.as_deref(),
        Some("2026-09-13 hook-enforced-artifact-schema D1"),
        "the rule carries its ruling anchor, so it leaves only by recorded supersession"
    );
}

// ---------------------------------------------------------------------------
// the shared fixture log — read by this crate AND by the plugin contract suite
// ---------------------------------------------------------------------------

/// The on-disk fixture log both sides read.
///
/// The plugin contract suite points at this directory rather than carrying a fixture of its own
/// (wave-4 contract plan), so it is the single place either side declares an artifact home. That
/// makes it a shared surface with a downstream consumer that cannot defend itself: dropping a
/// branch here would silently strand a contract row. The test below is that defence.
fn shared_fixture_log() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/home-log")
}

fn shared_fixture_state() -> State {
    replay::load(&shared_fixture_log()).unwrap_or_else(|findings| {
        let lines: Vec<String> = findings.iter().map(ToString::to_string).collect();
        panic!(
            "the shared fixture log must replay clean — the contract suite reads it:\n{}",
            lines.join("\n")
        )
    })
}

#[test]
fn the_shared_fixture_log_replays_clean_and_raises_no_rejecting_finding() {
    let state = shared_fixture_state();
    assert!(
        !state.docs.is_empty(),
        "the fixture log carries documents; an empty replay would pass every contract row vacuously"
    );
}

#[test]
fn the_shared_fixture_log_covers_every_branch_the_contract_suite_reads() {
    let state = shared_fixture_state();
    let homes = Homes::load(&state);

    // Branch 1: a templated deliverable whose template carries per-section `max_lines`.
    let templated = homes
        .iter()
        .flat_map(|home| home.deliverables.iter())
        .filter_map(|d| d.template.as_deref())
        .filter_map(|name| mochiko_cli::render::template_of(&state, name).ok())
        .find(|template| {
            template.conformance.is_some()
                && template.declared_headings().next().is_some()
                && template
                    .declared_headings()
                    .any(|(heading, _)| template.budget(heading).is_some())
        });
    let templated = templated.expect(
        "branch 1: a templated deliverable with a heading-level conformance block and a \
         per-section max_lines",
    );
    assert!(
        !templated.required_headings().is_empty(),
        "branch 1 needs at least one required heading to grade order against"
    );
    assert!(
        templated
            .conformance
            .as_ref()
            .is_some_and(|c| !c.placeholders.is_empty()),
        "branch 1 needs declared placeholder tokens"
    );

    // Branch 2: a template-less deliverable under a whole-file bound (D4f).
    assert!(
        homes.iter().any(|home| {
            home.bounds != Bounds::Elsewhere
                && home
                    .deliverables
                    .iter()
                    .any(|d| d.template.is_none() && d.form.is_none() && d.max_lines.is_some())
        }),
        "branch 2: a template-less deliverable with a whole-file bound"
    );

    // Branch 3: a reports/ directory whose envelope enumerates at least one report type (D2).
    let enum_types = homes
        .iter()
        .filter_map(|home| home.reports.as_ref())
        .filter_map(|reports| mochiko_cli::render::template_of(&state, &reports.envelope).ok())
        .filter_map(|envelope| {
            envelope
                .conformance
                .as_ref()
                .and_then(|c| c.frontmatter.enums.get("report").cloned())
        })
        .next()
        .expect("branch 3: a reports/ directory with an envelope carrying a `report:` enum");
    assert!(
        !enum_types.is_empty(),
        "branch 3 needs at least one enumerated report type"
    );

    // Branch 4: a kind whose bounds live in another named constraint home, cited (D4f/V2).
    assert!(
        homes
            .iter()
            .any(|home| home.bounds == Bounds::Elsewhere && home.bounds_cite.is_some()),
        "branch 4: a `bounds: elsewhere` home with its `bounds_cite`"
    );

    // An append-only log, so a per-entry bound has something to bind to.
    assert!(
        homes
            .iter()
            .any(|home| home.deliverables.iter().any(|d| d.form
                == Some(mochiko_cli::home::Form::Log)
                && d.entry_max_lines.is_some())),
        "an append-only log bounded per entry (D4d)"
    );
}

#[test]
fn the_shared_fixture_log_denies_and_allows_on_each_branch() {
    use mochiko_cli::conform::{self, Decision};
    let state = shared_fixture_state();
    let homes = Homes::load(&state);
    let grade = |path: &str, body: &str| {
        let resolution = homes.resolve(Path::new(path));
        conform::check(&state, &resolution, body, None)
    };

    // Branch 1: the templated file, conforming and then over one section's budget.
    let conforming = "---\nfeature: FEAT-001\nstatus: draft\n---\n\n# Demo\n\n\
                      ## Intent\n\nScope.\n\n## Requirements\n\nFR-001 one line.\n";
    assert_eq!(
        grade(".mochiko/features/FEAT-001/spec.md", conforming).decision,
        Decision::Allow
    );
    let mut fat = String::from("---\nfeature: FEAT-001\nstatus: draft\n---\n\n## Intent\n");
    for n in 0..12 {
        fat.push_str(&format!("line {n}\n"));
    }
    fat.push_str("\n## Requirements\n\nFR-001 one line.\n");
    assert_eq!(
        grade(".mochiko/features/FEAT-001/spec.md", &fat).decision,
        Decision::Deny,
        "branch 1 must be able to deny on a per-section budget"
    );

    // Branch 2: the whole-file bound.
    let short = "a\nb\nc\n".to_string();
    assert_eq!(
        grade(".mochiko/features/FEAT-001/gates.md", &short).decision,
        Decision::Allow
    );
    let long = "x\n".repeat(60);
    assert_eq!(
        grade(".mochiko/features/FEAT-001/gates.md", &long).decision,
        Decision::Deny,
        "branch 2 must be able to deny on a whole-file bound"
    );

    // Branch 3: the reports envelope — any name, the type list binds.
    let report = "---\nreport: cycle\nfeature: FEAT-001\n---\n\nBody.\n";
    assert_eq!(
        grade(".mochiko/features/FEAT-001/reports/any-name.md", report).decision,
        Decision::Allow
    );
    let bad_type = "---\nreport: invented\nfeature: FEAT-001\n---\n\nBody.\n";
    assert_eq!(
        grade(".mochiko/features/FEAT-001/reports/any-name.md", bad_type).decision,
        Decision::Deny,
        "branch 3 must be able to deny an unlisted report type"
    );

    // Branch 4: bounds elsewhere — the file set still binds, the size does not.
    let huge = "x\n".repeat(5_000);
    assert_eq!(
        grade(".mochiko/memory/knowledge-management.md", &huge).decision,
        Decision::Allow,
        "branch 4 takes no size check at all, however long the file is"
    );
    assert_eq!(
        grade(".mochiko/memory/invented.md", "x\n").decision,
        Decision::Deny,
        "branch 4 still closes its file set"
    );
}

#[test]
fn the_shared_fixture_log_never_answers_with_empty_stdout_on_a_non_deny() {
    use mochiko_cli::hook;
    let state = shared_fixture_state();
    // Every non-deny shape the suite exercises run-wide. The platform denies a background
    // subagent's call when no hook returns a decision, so an empty answer is never correct here —
    // the assert is run-wide in the contract suite and mirrored here at the source.
    let cwd = "/repo";
    let cases = [
        // A conforming write to a declared deliverable.
        format!(
            r##"{{"cwd":"{cwd}","tool_name":"Write","tool_input":{{"file_path":"{cwd}/.mochiko/features/FEAT-001/gates.md","content":"a\nb\n"}}}}"##
        ),
        // A path under no declared home.
        format!(
            r##"{{"cwd":"{cwd}","tool_name":"Write","tool_input":{{"file_path":"{cwd}/docs/page.md","content":"# x\n"}}}}"##
        ),
        // An ordinary shell command.
        format!(
            r##"{{"cwd":"{cwd}","tool_name":"Bash","tool_input":{{"command":"git status --porcelain"}}}}"##
        ),
        // A tool outside the matcher set.
        format!(
            r##"{{"cwd":"{cwd}","tool_name":"Read","tool_input":{{"file_path":"{cwd}/.mochiko/features/FEAT-001/gates.md"}}}}"##
        ),
        // A write outside the working directory.
        format!(
            r##"{{"cwd":"{cwd}","tool_name":"Write","tool_input":{{"file_path":"/elsewhere/x.md","content":"x"}}}}"##
        ),
    ];
    for json in cases {
        let payload = hook::parse(&json).expect("the payload parses");
        let outcome = hook::decide(&state, &payload);
        let rendered = hook::render(&outcome);
        assert!(
            !rendered.trim().is_empty(),
            "no outcome may render empty stdout: {json}"
        );
        assert!(
            rendered.contains(r#""permissionDecision":"allow""#),
            "every non-deny outcome carries an explicit allow: {rendered}"
        );
        assert_eq!(outcome.exit_code(), 0);
    }
}

// ---------------------------------------------------------------------------
// what `mochiko-cli home` says about each bound arm
// ---------------------------------------------------------------------------

/// The three ways a deliverable can be bounded, as a seat reads them before the first write.
///
/// The render is the delivery surface: a bound recorded in the log but not printed here is a rule
/// nobody sees. The declared-unbounded arm matters most — its whole point is that the ruling is
/// carried, not that the number is absent.
#[test]
fn the_home_render_names_each_bound_arm_as_the_log_declares_it() {
    let body = r#"
grammar: 1
id: 0001-homes
sequence: 1
intent: One home carrying all three bound arms.
changes:
  - op: import-document
    kind: home
    name: session
    content:
      home: session
      title: One thinking session's home
      path: [".mochiko", "brainstorms", "<slug>"]
      bounds: whole-file
      deliverables:
        - file: record.md
          bound_reason: the session it records
        - file: wave<n>-<slug>.md
          max_lines: 300
        - file: build-log.md
          form: log
          entry_max_lines: 60
"#;
    let state = state("render-bound-arms", body);
    let ctx = render::Context {
        binary: "0.1.0".to_string(),
        grammar: 1,
        plugin: "test".to_string(),
    };
    let view = render::home_view(
        &state,
        Path::new(".mochiko/brainstorms/demo/record.md"),
        &ctx,
    );

    assert!(
        view.contains("record.md · no template · no size bound — the session it records"),
        "the declared-unbounded arm prints its reason where a seat reads it: {view}"
    );
    assert!(
        view.contains("wave<n>-<slug>.md · no template · 300 lines, whole file"),
        "the whole-file arm prints its number: {view}"
    );
    assert!(
        view.contains("build-log.md · append-only log · 60 lines per `##` entry"),
        "the log arm prints the per-entry bound and the entry shape: {view}"
    );
    assert!(
        !view.contains("no bound declared"),
        "no arm here is silently unbounded: {view}"
    );
}
