//! The ported probe matrix for `scripts/find-similar-rules.py` — 48 probes.
//!
//! 42 port; the six that do not are all assertions about the Python script's own command-line
//! surface. Which is which is a set equation, not a hand count: `PYTHON_PROBES` carries all 48
//! check names verbatim, `PORTED` maps each Rust test to the names it covers, `NOT_APPLICABLE`
//! carries the rest with reasons, and `the_whole_python_matrix_is_accounted_for` asserts that
//! every name is claimed exactly once and that no ledger invents a name. Record D6 makes the port
//! the retirement gate for the Python scripts, so the gate has to be gradeable.
//!
//! The Python's end-to-end layer drove the detector as a subprocess over temp directories and
//! asserted on its stdout. Here the same cases build the fixture as state and assert on the
//! returned clusters, with the report's text asserted where the probe was about the text.
//!
//! Beyond the matrix, two parity pins that the Python matrix never had:
//!
//! * **Reference vectors.** `ratio()` is compared with values captured from CPython's
//!   `difflib.SequenceMatcher`, including pairs where the autojunk heuristic changes the answer.
//! * **The whole corpus.** The figures the live detector reports over the real tree. The full
//!   sweep is opt-in — `MOCHIKO_FULL_SIMILAR=1 cargo test` — because it is 98 seconds of a
//!   137-second suite in a debug build. CI runs it in its own step. The default suite keeps the
//!   command-family pin, the same assertion over a tenth of the pairs, and it announces the skip
//!   rather than passing silently.

use mochiko_cli::model::{DocKind, DocRef, Document};
use mochiko_cli::replay::State;
use mochiko_cli::similar::{self, ScoredRule, Tag};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the crate sits two levels under the repository root")
        .to_path_buf()
}

// ---------------------------------------------------------------------------
// fixtures
// ---------------------------------------------------------------------------

fn doc(state: &mut State, kind: DocKind, name: &str, yaml: &str) {
    let value: serde_norway::Value =
        serde_norway::from_str(yaml).unwrap_or_else(|e| panic!("{name} parses: {e}"));
    let document =
        Document::from_value(kind, &value).unwrap_or_else(|e| panic!("{name} decodes: {e}"));
    state.docs.insert(DocRef::new(kind, name), document);
}

/// The Python's command fixtures: a three-way near-duplicate across alpha, beta and gamma, one
/// side already extending the shared block, and two unrelated rules as controls.
fn command_fixtures(state: &mut State) {
    doc(
        state,
        DocKind::CommandCommon,
        "common",
        "\
kind: command-common
rules:
  - id: common.register
    labels: [binding]
    text: User-facing prose follows templates/output-style.md.
",
    );
    doc(
        state,
        DocKind::Command,
        "alpha",
        "\
kind: command
command: alpha
sections:
  - id: alpha.sec.tools
    title: tools
    intent: test
    rules:
      - id: alpha.register
        extends: common.register
        class: must
        kind: binding
      - id: alpha.solo
        labels: [binding]
        class: must
        kind: binding
        text: Completely unrelated obligation about parsing feature maps nightly.
",
    );
    doc(
        state,
        DocKind::Command,
        "beta",
        "\
kind: command
command: beta
sections:
  - id: beta.sec.tools
    title: tools
    intent: test
    rules:
      - id: beta.register
        labels: [binding]
        class: must
        kind: binding
        text: User-facing prose per templates/output-style.md.
",
    );
    doc(
        state,
        DocKind::Command,
        "gamma",
        "\
kind: command
command: gamma
sections:
  - id: gamma.sec.tools
    title: tools
    intent: test
    rules:
      - id: gamma.register
        labels: [binding]
        class: floor
        kind: binding
        text: User-facing prose follows templates/output-style.md.
      - id: gamma.solo
        labels: [binding]
        class: must
        kind: binding
        text: A different unrelated duty naming the governance ledger weekly.
",
    );
}

/// The Python's skill fixtures: a review-family stub and a sibling carrying near-identical local
/// text, plus a rule that matches the command fixtures' register cluster across grammars.
fn skill_fixtures(state: &mut State) {
    doc(
        state,
        DocKind::SkillCommon,
        "skill-review-common",
        "\
kind: skill-common
rules:
  - id: review-common.default-fail
    labels: [verdict]
    text: >-
      Never default to a clearing verdict — earned only by a completed hunt; absence of looking
      is never evidence.
",
    );
    doc(
        state,
        DocKind::Skill,
        "delta-grader",
        "\
kind: skill
skill: delta-grader
sections:
  - id: delta-grader.sec.verdict
    title: verdict
    intent: test
    rules:
      - id: delta-grader.default-fail
        extends: review-common.default-fail
        class: floor
",
    );
    doc(
        state,
        DocKind::Skill,
        "epsilon-grader",
        "\
kind: skill
skill: epsilon-grader
sections:
  - id: epsilon-grader.sec.verdict
    title: verdict
    intent: test
    rules:
      - id: epsilon-grader.never-default
        labels: [verdict]
        class: floor
        text: >-
          Never default to a clearing verdict — earned by a completed hunt; absence of looking is
          not evidence.
  - id: epsilon-grader.sec.output
    title: output
    intent: test
    rules:
      - id: epsilon-grader.register
        labels: [binding]
        class: must
        kind: binding
        text: User-facing prose follows templates/output-style.md.
",
    );
}

/// The authoring-family fixtures, including the stub whose target no loaded library carries.
fn authoring_fixtures(state: &mut State) {
    doc(
        state,
        DocKind::SkillCommon,
        "skill-authoring-common",
        "\
kind: skill-common
rules:
  - id: authoring-common.envelope-binding
    labels: [binding]
    text: >-
      The produced artifact follows the deliverable envelope in templates/artifact-format.md —
      referenced, never restated.
",
    );
    doc(
        state,
        DocKind::Skill,
        "authoring-zeta",
        "\
kind: skill
skill: authoring-zeta
sections:
  - id: authoring-zeta.sec.artifact
    title: artifact
    intent: test
    rules:
      - id: authoring-zeta.envelope
        extends: authoring-common.envelope-binding
        class: must
        kind: binding
",
    );
    doc(
        state,
        DocKind::Skill,
        "authoring-eta",
        "\
kind: skill
skill: authoring-eta
sections:
  - id: authoring-eta.sec.artifact
    title: artifact
    intent: test
    rules:
      - id: authoring-eta.envelope-local
        labels: [binding]
        class: must
        kind: binding
        text: >-
          The produced artifact follows the deliverable envelope in templates/artifact-format.md —
          referenced never restated.
",
    );
    doc(
        state,
        DocKind::Skill,
        "patterns-theta",
        "\
kind: skill
skill: patterns-theta
sections:
  - id: patterns-theta.sec.artifact
    title: artifact
    intent: test
    rules:
      - id: patterns-theta.ghost-stub
        extends: patterns-common.ghost
        class: must
        kind: binding
",
    );
}

fn mk(id: &str, text: &str) -> ScoredRule {
    ScoredRule {
        schema: "demo".into(),
        prefix: "demo".into(),
        id: id.into(),
        section: "tools".into(),
        kind: "constraint".into(),
        class: "must".into(),
        labels: BTreeSet::new(),
        pointer: None,
        extends: None,
        norm: similar::norm_for_sim(text, "demo"),
        text: text.into(),
    }
}

fn with(mut rule: ScoredRule, f: impl FnOnce(&mut ScoredRule)) -> ScoredRule {
    f(&mut rule);
    rule.norm = similar::norm_for_sim(&rule.text, &rule.prefix);
    rule
}

fn labels(items: &[&str]) -> BTreeSet<String> {
    items.iter().map(|s| (*s).to_string()).collect()
}

fn ids(cluster: &similar::Cluster) -> Vec<&str> {
    cluster.members.iter().map(|m| m.id.as_str()).collect()
}

fn all_ids(report: &similar::Report) -> BTreeSet<String> {
    report
        .clusters
        .iter()
        .flat_map(|c| c.members.iter().map(|m| m.id.clone()))
        .collect()
}

// ---------------------------------------------------------------------------
// unit layer — normalisation (5 probes)
// ---------------------------------------------------------------------------

#[test]
fn norm_collapses_vars_commands_and_own_prefix() {
    let n = similar::norm_for_sim(
        "Read ${TARGET} before /mochiko:demo fires; demo.sec.tools and demo.read-first bind.",
        "demo",
    );
    assert!(n.contains('«'), "{n}");
    assert!(n.contains("«var»") && !n.contains("target"), "{n}");
    assert!(n.contains("«cmd»") && !n.contains("mochiko"), "{n}");
    assert!(n.contains("«self».tools"), "{n}");
    // Hyphens strip with the rest of the punctuation, so the slug reads space-joined.
    assert!(n.contains("«self».read first"), "{n}");
}

#[test]
fn norm_keeps_a_foreign_prefix() {
    let n = similar::norm_for_sim(
        "Read ${OTHER} before /mochiko:setup fires; spec.sec.tools stays.",
        "demo",
    );
    assert!(n.replace(' ', "").contains("spec.sec.tools"), "{n}");
}

// ---------------------------------------------------------------------------
// unit layer — similarity (3 probes)
// ---------------------------------------------------------------------------

#[test]
fn text_sim_is_one_for_identical_text() {
    assert_eq!(
        similar::text_sim("one two three four", "one two three four", 0.0),
        1.0
    );
}

#[test]
fn token_sort_rescues_a_reordered_pair() {
    let reordered = similar::text_sim(
        "alpha beta gamma delta epsilon",
        "delta epsilon alpha beta gamma",
        0.0,
    );
    assert!(reordered > 0.95, "{reordered:.3}");
}

#[test]
fn a_disjoint_pair_under_the_floor_scores_zero() {
    assert_eq!(similar::text_sim("aa bb cc dd", "ww xx yy zz", 0.5), 0.0);
}

// ---------------------------------------------------------------------------
// unit layer — the structural bonus (3 probes)
// ---------------------------------------------------------------------------

#[test]
fn the_structural_bonus_caps() {
    let x = with(mk("demo.a", "t"), |r| {
        r.pointer = Some("mochiko:x".into());
        r.labels = labels(&["binding"]);
    });
    let y = with(mk("demo.b", "t"), |r| {
        r.pointer = Some("mochiko:x".into());
        r.labels = labels(&["binding"]);
    });
    assert_eq!(similar::struct_bonus(&x, &y), similar::BONUS_CAP);
}

#[test]
fn nothing_shared_earns_no_bonus() {
    let x = with(mk("demo.a", "t"), |r| {
        r.pointer = Some("mochiko:x".into());
        r.labels = labels(&["binding"]);
    });
    let y = with(mk("demo.c", "t"), |r| {
        r.section = "roles".into();
        r.pointer = Some("mochiko:y".into());
        r.labels = labels(&["role"]);
    });
    assert_eq!(similar::struct_bonus(&x, &y), 0.0);
}

#[test]
fn a_labels_jaccard_of_one_half_counts() {
    let x = with(mk("demo.a", "t"), |r| {
        r.pointer = Some("mochiko:x".into());
        r.labels = labels(&["binding"]);
    });
    let y = with(mk("demo.d", "t"), |r| {
        r.section = "roles".into();
        r.labels = labels(&["binding", "role"]);
    });
    assert_eq!(similar::struct_bonus(&x, &y), 0.04);
}

// ---------------------------------------------------------------------------
// unit layer — the scoring guards (7 probes)
// ---------------------------------------------------------------------------

const LONG: &str = "the quick brown fox jumps over the lazy dog every single morning";

#[test]
fn a_short_pair_sharing_only_its_frame_is_dropped() {
    let rules = vec![
        mk("demo.s1", "An unaccepted record here"),
        mk("demo.s2", "An undispositioned review survivor"),
    ];
    let (edges, _, _) = similar::score_pairs(&rules, 0.55, &BTreeSet::new());
    assert!(edges.is_empty(), "{} edges", edges.len());
}

#[test]
fn a_short_pair_that_is_exact_still_pairs() {
    let rules = vec![
        mk("demo.s3", "User acceptance not given"),
        with(mk("othr.s4", "User acceptance not given"), |r| {
            r.schema = "other".into();
            r.prefix = "othr".into();
        }),
    ];
    let (edges, _, _) = similar::score_pairs(&rules, 0.55, &BTreeSet::new());
    assert_eq!(edges.len(), 1);
}

#[test]
fn the_combined_score_never_exceeds_one() {
    let rules = vec![
        with(mk("demo.x", LONG), |r| {
            r.pointer = Some("mochiko:p".into());
            r.labels = labels(&["binding"]);
        }),
        with(mk("othr.y", LONG), |r| {
            r.schema = "other".into();
            r.prefix = "othr".into();
            r.pointer = Some("mochiko:p".into());
            r.labels = labels(&["binding"]);
        }),
    ];
    let (edges, _, _) = similar::score_pairs(&rules, 0.55, &BTreeSet::new());
    assert_eq!(edges[0].total, 1.0);
}

#[test]
fn a_cross_kind_pair_is_never_scored() {
    let rules = vec![
        with(mk("demo.k1", LONG), |r| r.kind = "duty".into()),
        with(mk("othr.k2", LONG), |r| {
            r.schema = "other".into();
            r.prefix = "othr".into();
            r.kind = "fail".into();
        }),
    ];
    let (edges, scored, _) = similar::score_pairs(&rules, 0.55, &BTreeSet::new());
    assert!(edges.is_empty());
    assert_eq!(scored, 0, "a cross-kind pair is not even scored");
}

#[test]
fn two_stubs_over_one_common_block_are_skipped() {
    let rules = vec![
        with(mk("demo.e1", LONG), |r| r.extends = Some("common.z".into())),
        with(mk("othr.e2", LONG), |r| {
            r.schema = "other".into();
            r.prefix = "othr".into();
            r.extends = Some("common.z".into());
        }),
    ];
    let (edges, _, _) = similar::score_pairs(&rules, 0.55, &BTreeSet::new());
    assert!(edges.is_empty());
}

#[test]
fn the_same_block_skip_is_grammar_agnostic() {
    // The comparison is on the raw `extends:` value, so a `review-common.*` skill pair skips
    // exactly like a `common.*` command pair.
    let rules = vec![
        with(mk("alpha-grader.e1", LONG), |r| {
            r.schema = "alpha-grader".into();
            r.prefix = "alpha-grader".into();
            r.extends = Some("review-common.z".into());
        }),
        with(mk("beta-grader.e2", LONG), |r| {
            r.schema = "beta-grader".into();
            r.prefix = "beta-grader".into();
            r.extends = Some("review-common.z".into());
        }),
    ];
    let (edges, _, _) = similar::score_pairs(&rules, 0.55, &BTreeSet::new());
    assert!(edges.is_empty());
}

#[test]
fn an_allowlisted_edge_is_counted_and_not_emitted() {
    let rules = vec![
        with(mk("demo.x", LONG), |r| {
            r.pointer = Some("mochiko:p".into());
            r.labels = labels(&["binding"]);
        }),
        with(mk("othr.y", LONG), |r| {
            r.schema = "other".into();
            r.prefix = "othr".into();
            r.pointer = Some("mochiko:p".into());
            r.labels = labels(&["binding"]);
        }),
    ];
    let suppressed: BTreeSet<(String, String)> = [("demo.x".to_string(), "othr.y".to_string())]
        .into_iter()
        .collect();
    let (edges, _, hits) = similar::score_pairs(&rules, 0.55, &suppressed);
    assert!(edges.is_empty());
    assert_eq!(hits, 1);
}

// ---------------------------------------------------------------------------
// unit layer — classification (5 probes)
// ---------------------------------------------------------------------------

fn cluster_of(members: &[(&str, Option<&str>)]) -> similar::Cluster {
    similar::Cluster {
        members: members
            .iter()
            .enumerate()
            .map(|(i, (schema, extends))| {
                with(mk(&format!("{schema}.r{i}"), "t"), |r| {
                    r.schema = (*schema).into();
                    r.prefix = (*schema).into();
                    r.extends = extends.map(std::string::ToString::to_string);
                })
            })
            .collect(),
        edges: vec![(1.0, 1.0, "a".into(), "b".into())],
    }
}

#[test]
fn three_schemas_read_as_a_common_candidate() {
    let c = cluster_of(&[("a", None), ("b", None), ("c", None)]);
    assert_eq!(similar::classify(&c)[0], Tag::CommonCandidate);
}

#[test]
fn two_schemas_read_as_a_cross_pair() {
    let c = cluster_of(&[("a", None), ("b", None)]);
    assert_eq!(similar::classify(&c)[0], Tag::CrossPair);
}

#[test]
fn one_schema_reads_as_intra_schema() {
    let c = cluster_of(&[("a", None), ("a", None)]);
    assert_eq!(similar::classify(&c)[0], Tag::IntraSchema);
}

#[test]
fn a_mixed_extends_cluster_is_an_extend_gap() {
    let c = cluster_of(&[("a", Some("common.z")), ("b", None)]);
    assert!(similar::classify(&c).contains(&Tag::ExtendGap));
}

#[test]
fn an_all_extends_cluster_is_not_a_gap() {
    let c = cluster_of(&[("a", Some("common.z")), ("b", Some("common.w"))]);
    assert!(!similar::classify(&c).contains(&Tag::ExtendGap));
}

// ---------------------------------------------------------------------------
// the command fixtures (8 of the Python's 12; four are named in NOT_APPLICABLE)
// ---------------------------------------------------------------------------

fn command_report() -> similar::Report {
    let mut state = State::default();
    command_fixtures(&mut state);
    similar::clusters(&state, similar::DEFAULT_THRESHOLD, None)
}

#[test]
fn the_register_cluster_is_found_and_tagged() {
    let report = command_report();
    assert_eq!(report.clusters.len(), 1, "one cluster");
    let cluster = &report.clusters[0];
    assert!(ids(cluster).contains(&"beta.register"));
    assert!(ids(cluster).contains(&"gamma.register"));

    let tags: Vec<&str> = similar::classify(cluster)
        .iter()
        .map(|t| t.as_str())
        .collect();
    assert_eq!(tags, ["COMMON-CANDIDATE", "EXTEND-GAP"]);
}

#[test]
fn a_stubs_inherited_text_re_enters_scoring() {
    let report = command_report();
    let cluster = &report.clusters[0];
    assert!(
        ids(cluster).contains(&"alpha.register"),
        "the stub resolved its block's text and clustered: {:?}",
        ids(cluster)
    );
    let rendered = similar::render_report(&report);
    assert!(rendered.contains("extends common.register"), "{rendered}");
}

#[test]
fn a_floor_member_is_flagged() {
    let rendered = similar::render_report(&command_report());
    assert!(rendered.contains("⚑floor"), "{rendered}");
}

#[test]
fn the_unrelated_controls_stay_unclustered() {
    let clustered = all_ids(&command_report());
    assert!(!clustered.contains("alpha.solo"));
    assert!(!clustered.contains("gamma.solo"));
}

#[test]
fn the_report_names_its_threshold_and_counts() {
    let report = command_report();
    let rendered = similar::render_report(&report);
    assert!(rendered.starts_with("=== similar-rule clusters (threshold 0.60) ==="));
    assert!(rendered.contains(&format!("rules scanned: {}", report.scanned)));
    assert!(rendered.contains("COMMON-CANDIDATE 1"));
}

#[test]
fn a_fully_suppressed_cluster_disappears_and_is_counted() {
    let mut state = State::default();
    command_fixtures(&mut state);
    let allowlist = write_allowlist(
        "command-suppressions",
        &[
            ("beta.register", "gamma.register", "adjudicated"),
            ("alpha.register", "gamma.register", "adjudicated"),
            ("alpha.register", "beta.register", "adjudicated"),
            ("ghost.rule", "beta.register", "stale on purpose"),
        ],
    );
    let report = similar::clusters(&state, similar::DEFAULT_THRESHOLD, Some(&allowlist));

    assert!(report.clusters.is_empty());
    assert_eq!(report.suppressed_hits, 3);
    let rendered = similar::render_report(&report);
    assert!(
        rendered.contains("none — no pair clears the threshold"),
        "{rendered}"
    );
    assert!(
        rendered.contains("allowlist-suppressed edges: 3"),
        "{rendered}"
    );
    assert!(
        rendered.contains("allowlist names ghost.rule: not a live rule ID"),
        "{rendered}"
    );
}

#[test]
fn an_allowlist_row_without_a_reason_is_warned() {
    let mut state = State::default();
    command_fixtures(&mut state);
    let allowlist = write_allowlist(
        "command-no-reason",
        &[("beta.register", "gamma.register", "")],
    );
    let report = similar::clusters(&state, similar::DEFAULT_THRESHOLD, Some(&allowlist));
    assert!(
        report
            .warnings
            .iter()
            .any(|w| w.contains("no `reason` recorded")),
        "{:?}",
        report.warnings
    );
}

fn write_allowlist(name: &str, rows: &[(&str, &str, &str)]) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("similar");
    std::fs::create_dir_all(&dir).expect("the scratch directory is writable");
    let path = dir.join(format!("{name}.yaml"));
    let mut text = String::from("suppressions:\n");
    for (a, b, reason) in rows {
        text.push_str(&format!("  - ids: [{a}, {b}]\n    reason: \"{reason}\"\n"));
    }
    std::fs::write(&path, text).expect("the allowlist writes");
    path
}

// ---------------------------------------------------------------------------
// the skill fixtures (6 of the Python's 7; one is named in NOT_APPLICABLE)
// plus the two mixed-sweep probes
// ---------------------------------------------------------------------------

#[test]
fn the_skill_fixtures_cluster_across_the_stub_and_its_sibling() {
    let mut state = State::default();
    skill_fixtures(&mut state);
    let report = similar::clusters(&state, similar::DEFAULT_THRESHOLD, None);
    let clustered = all_ids(&report);
    assert!(
        clustered.contains("delta-grader.default-fail"),
        "{clustered:?}"
    );
    assert!(
        clustered.contains("epsilon-grader.never-default"),
        "{clustered:?}"
    );

    let cluster = &report.clusters[0];
    assert!(similar::classify(cluster).contains(&Tag::ExtendGap));
    let rendered = similar::render_report(&report);
    assert!(
        rendered.contains("extends review-common.default-fail"),
        "{rendered}"
    );
    assert!(rendered.contains("⚑floor"), "{rendered}");
}

#[test]
fn a_skills_only_run_reads_no_allowlist_and_leaves_the_control_alone() {
    let mut state = State::default();
    skill_fixtures(&mut state);
    let report = similar::clusters(&state, similar::DEFAULT_THRESHOLD, None);
    assert!(
        report
            .warnings
            .iter()
            .all(|w| !w.contains("not a live rule ID")),
        "{:?}",
        report.warnings
    );
    assert!(!all_ids(&report).contains("epsilon-grader.register"));
}

#[test]
fn a_skill_edge_is_suppressed_like_a_command_edge() {
    let mut state = State::default();
    skill_fixtures(&mut state);
    let allowlist = write_allowlist(
        "skill-suppressions",
        &[(
            "delta-grader.default-fail",
            "epsilon-grader.never-default",
            "adjudicated keep-distinct",
        )],
    );
    let report = similar::clusters(&state, similar::DEFAULT_THRESHOLD, Some(&allowlist));
    assert!(report.clusters.is_empty());
    assert_eq!(report.suppressed_hits, 1);
}

#[test]
fn a_mixed_sweep_surfaces_the_cross_grammar_edge_and_scans_both_sets() {
    let mut state = State::default();
    command_fixtures(&mut state);
    skill_fixtures(&mut state);
    let report = similar::clusters(&state, similar::DEFAULT_THRESHOLD, None);
    let clustered = all_ids(&report);
    assert!(
        clustered.contains("epsilon-grader.register"),
        "{clustered:?}"
    );
    assert!(clustered.contains("beta.register"), "{clustered:?}");
    assert_eq!(report.scanned, 8, "both fixture sets are scanned");
}

// ---------------------------------------------------------------------------
// the authoring fixtures (3 of the Python's 4; one is named in NOT_APPLICABLE)
// ---------------------------------------------------------------------------

#[test]
fn the_authoring_stub_resolves_and_gaps_against_its_sibling() {
    let mut state = State::default();
    skill_fixtures(&mut state);
    authoring_fixtures(&mut state);
    let report = similar::clusters(&state, similar::DEFAULT_THRESHOLD, None);
    let rendered = similar::render_report(&report);

    assert!(
        rendered.contains("extends authoring-common.envelope-binding"),
        "{rendered}"
    );
    assert!(all_ids(&report).contains("authoring-eta.envelope-local"));
    assert!(report
        .clusters
        .iter()
        .any(|c| similar::classify(c).contains(&Tag::ExtendGap)));
}

#[test]
fn both_families_resolve_in_one_run() {
    let mut state = State::default();
    skill_fixtures(&mut state);
    authoring_fixtures(&mut state);
    let rendered =
        similar::render_report(&similar::clusters(&state, similar::DEFAULT_THRESHOLD, None));
    assert!(
        rendered.contains("extends review-common.default-fail"),
        "{rendered}"
    );
    assert!(
        rendered.contains("extends authoring-common.envelope-binding"),
        "{rendered}"
    );
}

#[test]
fn a_stub_whose_library_carries_no_such_block_warns_and_never_clusters() {
    let mut state = State::default();
    skill_fixtures(&mut state);
    authoring_fixtures(&mut state);
    let report = similar::clusters(&state, similar::DEFAULT_THRESHOLD, None);
    assert!(
        report
            .warnings
            .iter()
            .any(|w| w == "patterns-theta.ghost-stub: empty resolved text, skipped"),
        "{:?}",
        report.warnings
    );
    assert!(!all_ids(&report).contains("patterns-theta.ghost-stub"));
}

// ---------------------------------------------------------------------------
// parity: reference vectors from CPython's difflib
// ---------------------------------------------------------------------------

/// `(name, a, b, difflib.SequenceMatcher(None, a, b).ratio())`, captured from CPython.
///
/// The `over-200` and `corpus-aj-*` rows are pairs where the autojunk heuristic changes the
/// answer: without it this table does not pass.
#[rustfmt::skip]
const DIFFLIB_VECTORS: &[(&str, &str, &str, f64)] = &[
    ("identical", "one two three four", "one two three four", 1.0),
    ("disjoint", "aa bb cc dd", "ww xx yy zz", 0.2727272727272727),
    ("reordered", "alpha beta gamma delta epsilon", "delta epsilon alpha beta gamma", 0.5333333333333333),
    ("empty-both", "", "", 1.0),
    ("empty-one", "", "abc", 0.0),
    ("single-char", "a", "b", 0.0),
    ("prefix", "abcd", "bcde", 0.75),
    ("repeat", "aaaaaaaa", "aaaa", 0.6666666666666666),
    ("marker", "«self».read first «var»", "«self».read second «var»", 0.8085106382978723),
    ("under-200", "ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ", "ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ", 0.9917355371900827),
    ("at-200", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y y", "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z z", 0.505),
    ("over-200", "the quick brown fox the quick brown fox the quick brown fox the quick brown fox the quick brown fox the quick brown fox the quick brown fox the quick brown fox the quick brown fox the quick brown fox the quick brown fox the quick brown fox ", "the quick brown dog the quick brown dog the quick brown dog the quick brown dog the quick brown dog the quick brown dog the quick brown dog the quick brown dog the quick brown dog the quick brown dog the quick brown dog the quick brown dog ", 0.06666666666666667),
    ("corpus-staffing-a", "beyond this schema s class floor rules the sound loop floor «self».sound loop floor among them everything is your per visit judgment how you staff sequence and run the visit is yours to shape.", "beyond this schema s class floor rules everything is your per run judgment how you staff sequence and run the cycles teammates or subagents per seat is your call.", 0.6779661016949152),
    ("corpus-staffing-b", "teammates or subagents per seat is your call.", "beyond this schema s class floor rules everything is your per run judgment how you staff sequence and run the cycles teammates or subagents per seat is your call.", 0.43478260869565216),
    ("corpus-staffing-c", "teammates or subagents per seat is your call.", "you are the lead plan the run and orchestrate it toward the goal state in plugins mochiko commands specify.md teammates or subagents per seat is your call.", 0.45),
    ("corpus-staffing-d", "beyond this schema s class floor rules everything is your per run judgment how you staff sequence and run the cycles teammates or subagents per seat is your call.", "you are the lead plan the run and orchestrate it toward the goal state in plugins mochiko commands specify.md teammates or subagents per seat is your call.", 0.41009463722397477),
    ("corpus-health", "surface health before the ask.", "proactive report first health before the ask every visit.", 0.6206896551724138),
];

#[test]
fn the_ratio_reproduces_cpython_difflib() {
    for (name, a, b, want) in DIFFLIB_VECTORS {
        let got = similar::ratio(a, b);
        assert!(
            (got - want).abs() < 1e-12,
            "{name}: difflib says {want}, this port says {got}"
        );
    }
}

#[test]
fn autojunk_is_ported_rather_than_skipped() {
    // `over-200` is the witness: with autojunk the repeated words are dropped from the index and
    // the ratio collapses; without it the two strings read as near-identical. A port that skipped
    // the heuristic would return roughly 0.95 here.
    let (_, a, b, want) = DIFFLIB_VECTORS
        .iter()
        .find(|(name, ..)| *name == "over-200")
        .expect("the vector table carries the autojunk witness");
    let got = similar::ratio(a, b);
    assert!((got - want).abs() < 1e-12, "{got} vs {want}");
    assert!(got < 0.1, "autojunk must bite here, got {got}");
}

#[test]
fn the_quick_ratios_bound_the_real_one() {
    for (name, a, b, want) in DIFFLIB_VECTORS {
        assert!(
            similar::quick_ratio(a, b) >= want - 1e-12,
            "{name}: quick_ratio is not an upper bound"
        );
        assert!(
            similar::real_quick_ratio(a, b) >= similar::quick_ratio(a, b) - 1e-12,
            "{name}: real_quick_ratio is not the looser bound"
        );
    }
}

// ---------------------------------------------------------------------------
// parity: the whole shipped corpus
// ---------------------------------------------------------------------------

/// The corpus as a state, optionally narrowed to one grammar.
///
/// `keep` decides which documents load. Narrowing to the command family cuts the scored pairs by
/// an order of magnitude, which is what makes a corpus pin affordable in the default suite.
///
/// Through wave 5 this scanned the shipped schema files. No schema file ships from wave 6, so the
/// corpus is the replayed log — which is also the text a run is actually delivered, and therefore
/// the text a near-duplicate detector should be scoring.
fn corpus_state(keep: fn(DocKind) -> bool) -> State {
    let full = mochiko_cli::replay::load(&repo_root().join("plugins/mochiko/migrations"))
        .unwrap_or_else(|findings| {
            let lines: Vec<String> = findings.iter().map(ToString::to_string).collect();
            panic!("the committed log is deliverable:\n{}", lines.join("\n"))
        });
    let mut state = State::default();
    for (doc, document) in full.docs {
        if keep(doc.kind) {
            state.docs.insert(doc, document);
        }
    }
    state
}

/// The always-on corpus pin: the command family only.
///
/// The full sweep below is 98 seconds of a 137-second suite, so it is opt-in. This one runs
/// every time and would catch any change to normalisation, bucketing, the guards or the
/// allowlist fold, because it exercises all of them over real shipped text — just less of it.
#[test]
fn the_detector_reproduces_its_figures_over_the_command_family() {
    let state = corpus_state(|kind| {
        matches!(
            kind,
            DocKind::Command | DocKind::CommandCommon | DocKind::CommandLabels
        )
    });
    let allowlist = repo_root().join(similar::ALLOWLIST);
    let report = similar::clusters(&state, similar::DEFAULT_THRESHOLD, Some(&allowlist));

    assert_eq!(
        (
            report.scanned,
            report.scored,
            report.clusters.len(),
            report.suppressed_hits
        ),
        // Seeded against the reference implementation rather than self-asserted: the retired
        //   uv run scripts/find-similar-rules.py \
        //     --schemas-dir plugins/mochiko/schemas \
        //     --allowlist scripts/similar-rules-allowlist.yaml
        // returned these same four numbers over the shipped files. Re-measured at wave 6 against
        // the replayed corpus and unmoved: `0003`'s command-side rewords left no allowlisted
        // command-family edge below the threshold.
        (321, 12_154, 0, 60),
        "rules scanned · in-kind pairs scored · clusters · allowlist-suppressed edges"
    );
}

/// The whole corpus, pinned to its measured figures.
///
/// Opt-in: `MOCHIKO_FULL_SIMILAR=1 cargo test`. Skipped by default because the scoring pass is
/// 98 seconds in a debug build (10 seconds in release, 80 in the Python it replaces), and CI runs
/// it explicitly in its own step. Skipping is announced, never silent.
#[test]
fn the_detector_reproduces_the_live_runs_figures_over_the_corpus() {
    if std::env::var("MOCHIKO_FULL_SIMILAR").as_deref() != Ok("1") {
        println!("skipped: the full-corpus sweep is opt-in — set MOCHIKO_FULL_SIMILAR=1 to run it");
        return;
    }
    let root = repo_root();
    let state = corpus_state(|_| true);
    let allowlist = root.join(similar::ALLOWLIST);
    let report = similar::clusters(&state, similar::DEFAULT_THRESHOLD, Some(&allowlist));

    // Seeded from a live `uv run scripts/find-similar-rules.py` over the shipped files, and
    // re-measured at wave 6 against the replayed corpus. Rule count and pair count are unmoved —
    // `0003` reworded fourteen rules and retired none. The suppressed count fell from 181 to 169:
    // an allowlisted edge is suppressed only while its two rules still score as near-identical,
    // and striking the shared two-arm clause is exactly the kind of edit that pulls a pair back
    // under the threshold. Twelve edges no longer need suppressing, and the cluster count staying
    // at zero is what says none of them re-surfaced as a finding.
    // Re-measured after `0004` (the sonnet worker rung): six skill rules minted on
    // `patterns-model-tiering` move the scan from 1,016 to 1,022 and the in-kind pair count with
    // it; no new cluster surfaced and the suppressed set is unchanged — the allowlist was not
    // touched.
    assert_eq!(report.scanned, 1022, "rules scanned");
    assert_eq!(report.scored, 148_353, "in-kind pairs scored");
    assert_eq!(report.clusters.len(), 0, "clusters");
    assert_eq!(report.suppressed_hits, 169, "allowlist-suppressed edges");
}

// ---------------------------------------------------------------------------
// allowlist resolution (advisory A1)
// ---------------------------------------------------------------------------

/// A repository-shaped scratch tree: `<name>/scripts/similar-rules-allowlist.yaml` plus a nested
/// log directory, so resolution can be exercised without touching the real tree.
fn scratch_repo(name: &str, with_allowlist: bool) -> PathBuf {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join("allowlist")
        .join(name);
    let log = root.join("nested/deeper/migrations");
    std::fs::create_dir_all(&log).expect("the scratch tree is writable");
    if with_allowlist {
        let scripts = root.join("scripts");
        std::fs::create_dir_all(&scripts).expect("the scratch tree is writable");
        std::fs::write(root.join(similar::ALLOWLIST), "suppressions: []\n")
            .expect("the scratch allowlist writes");
    }
    log
}

#[test]
fn the_allowlist_is_found_by_walking_up_from_the_log_dir() {
    let log = scratch_repo("carries-one", true);
    let found = similar::find_allowlist(&log).expect("an ancestor carries the allowlist");
    assert!(found.is_file());
    // The nearest ancestor wins. Both this scratch tree and the repository above it carry an
    // allowlist, and a log under the scratch tree is governed by the scratch one.
    assert_eq!(
        found,
        log.ancestors()
            .nth(3)
            .expect("the scratch root")
            .join(similar::ALLOWLIST)
    );
    assert_ne!(found, repo_root().join(similar::ALLOWLIST));
}

#[test]
fn a_log_whose_own_tree_carries_no_allowlist_keeps_walking_up() {
    // The scratch tree is inside the repository, so the walk continues past it and lands on the
    // repository's own allowlist. That is the intended behaviour: a log is governed by the
    // nearest allowlist above it, whatever level that is.
    let log = scratch_repo("carries-none", false);
    assert_eq!(
        similar::find_allowlist(&log),
        Some(repo_root().join(similar::ALLOWLIST))
    );
}

#[test]
fn a_log_with_no_allowlist_anywhere_above_it_resolves_none() {
    // The filesystem root is the one directory this test can be sure carries no allowlist, and
    // no scratch tree can sit outside the repository without writing beyond CARGO_TARGET_TMPDIR.
    assert_eq!(similar::find_allowlist(Path::new("/")), None);
}

#[test]
fn the_repositorys_allowlist_resolves_without_the_process_cwd() {
    // Integration tests run with the working directory set to the package root, not the
    // repository root, so a resolution that consulted the cwd would find nothing here. That is
    // the whole point: the same command over the same log must report the same thing from any
    // directory. Resolving `./scripts/...` is what returned 76 adjudicated clusters as fresh
    // signal when the binary was run from outside the tree.
    let cwd = std::env::current_dir().expect("a working directory");
    assert!(
        !cwd.join(similar::ALLOWLIST).is_file(),
        "this test is vacuous if the cwd carries the allowlist: {}",
        cwd.display()
    );
    // The log sits three levels below the root since wave 3 (`plugins/mochiko/migrations`), so
    // the ancestor walk has further to climb; the allowlist it must reach is unmoved.
    let found = similar::find_allowlist(&repo_root().join("plugins/mochiko/migrations"))
        .expect("the repository's own allowlist");
    assert_eq!(found, repo_root().join(similar::ALLOWLIST));
}

#[test]
fn a_run_with_no_allowlist_says_so_rather_than_going_quiet() {
    let mut state = State::default();
    command_fixtures(&mut state);
    let report = similar::clusters(&state, similar::DEFAULT_THRESHOLD, None);
    assert!(report.edges > 0, "the fixtures do cluster");
    assert_eq!(report.allowlist, None);

    let rendered = similar::render_report(&report);
    assert!(
        rendered.contains(&format!(
            "allowlist: none ({} edges unsuppressed)",
            report.edges
        )),
        "{rendered}"
    );
}

#[test]
fn a_run_with_an_allowlist_names_its_suppression_count_even_at_zero() {
    let mut state = State::default();
    command_fixtures(&mut state);
    let allowlist = write_allowlist("suppresses-nothing", &[("ghost.a", "ghost.b", "stale")]);
    let report = similar::clusters(&state, similar::DEFAULT_THRESHOLD, Some(&allowlist));
    assert_eq!(report.suppressed_hits, 0);
    let rendered = similar::render_report(&report);
    assert!(
        rendered.contains("allowlist-suppressed edges: 0"),
        "{rendered}"
    );
    assert!(!rendered.contains("allowlist: none"), "{rendered}");
}

// ---------------------------------------------------------------------------
// name-level accounting against the Python matrix
// ---------------------------------------------------------------------------

/// Every `check("…")` name in `scripts/test-find-similar-rules.py`, verbatim and in source
/// order. Extracted from the script rather than transcribed from its section comments.
///
/// This array is the whole point of the section. Record D6 makes the port the retirement gate
/// for the Python scripts, so "45 of 48 ported" has to be a set equation the compiler and the
/// test runner check, never a hand count in a doc comment. The first ledger this seat wrote for
/// the command matrix summed to 136 of 134; counting by hand is how that happens.
const PYTHON_PROBES: [&str; 48] = [
    "norm: ${var} collapsed",
    "norm: /mochiko:<cmd> collapsed",
    "norm: own-prefix section collapsed",
    "norm: own-prefix rule collapsed",
    "norm: foreign prefix kept",
    "sim: identical = 1.0",
    "sim: token-sort rescues reorder",
    "sim: disjoint under floor = 0.0",
    "bonus: pointer+section+labels hits cap",
    "bonus: nothing shared = 0",
    "bonus: labels jaccard 0.5 counts",
    "guard: short near-frame pair dropped",
    "guard: short exact pair kept",
    "cap: combined never exceeds 1.00",
    "bucket: cross-kind never scored",
    "skip: same common block skipped",
    "skip: same review-common block skipped (skill grammar)",
    "allowlist: edge suppressed",
    "classify: 3 schemas = COMMON-CANDIDATE",
    "classify: 2 schemas = CROSS-PAIR",
    "classify: 1 schema = INTRA-SCHEMA",
    "classify: mixed extends adds EXTEND-GAP",
    "classify: all-extends is not a gap",
    "e2e: exit 0 by default",
    "e2e: register cluster found",
    "e2e: COMMON-CANDIDATE + EXTEND-GAP tagged",
    "e2e: extends resolution feeds stub text",
    "e2e: floor flagged",
    "e2e: controls stay unclustered",
    "e2e: --exit-signal exits 1 on clusters",
    "e2e: --json parses",
    "e2e: fully-suppressed cluster gone",
    "e2e: suppression count reported",
    "e2e: stale allowlist ID warned",
    "e2e: --exit-signal exits 0 when suppressed clean",
    "skill e2e: exit 0 by default",
    "skill e2e: fixture run reads no live allowlist",
    "skill e2e: in-dir schemas discovered and clustered",
    "skill e2e: stub resolves against review-common (EXTEND-GAP)",
    "skill e2e: floor members flagged",
    "skill e2e: register control stays unclustered (skills-only run)",
    "skill e2e: skill edge suppressed",
    "mixed e2e: cross-grammar edge surfaces in the register cluster",
    "mixed e2e: both sets scanned",
    "authoring e2e: exit 0 by default",
    "authoring e2e: stub resolves against authoring-common (EXTEND-GAP)",
    "authoring e2e: both families resolve in one run",
    "authoring e2e: unknown-prefix stub warns, never clusters",
];

/// Builds the ported ledger from `<rust test> => [<python check names it covers>]`.
///
/// The test is named once and used twice — as a function value the compiler must resolve, and as
/// the string the ledger prints. A renamed or deleted test breaks the build rather than quietly
/// leaving a Python probe unclaimed.
macro_rules! ported {
    ($($test:ident => [$($python:literal),+ $(,)?]),+ $(,)?) => {
        const PORTED: &[(&str, fn(), &[&str])] = &[
            $((stringify!($test), $test as fn(), &[$($python),+])),+
        ];
    };
}

ported! {
    // unit layer — normalisation
    norm_collapses_vars_commands_and_own_prefix => [
        "norm: ${var} collapsed",
        "norm: /mochiko:<cmd> collapsed",
        "norm: own-prefix section collapsed",
        "norm: own-prefix rule collapsed",
    ],
    norm_keeps_a_foreign_prefix => ["norm: foreign prefix kept"],

    // unit layer — text similarity
    text_sim_is_one_for_identical_text => ["sim: identical = 1.0"],
    token_sort_rescues_a_reordered_pair => ["sim: token-sort rescues reorder"],
    a_disjoint_pair_under_the_floor_scores_zero => ["sim: disjoint under floor = 0.0"],

    // unit layer — the structural bonus
    the_structural_bonus_caps => ["bonus: pointer+section+labels hits cap"],
    nothing_shared_earns_no_bonus => ["bonus: nothing shared = 0"],
    a_labels_jaccard_of_one_half_counts => ["bonus: labels jaccard 0.5 counts"],

    // unit layer — the scoring guards
    a_short_pair_sharing_only_its_frame_is_dropped => ["guard: short near-frame pair dropped"],
    a_short_pair_that_is_exact_still_pairs => ["guard: short exact pair kept"],
    the_combined_score_never_exceeds_one => ["cap: combined never exceeds 1.00"],
    a_cross_kind_pair_is_never_scored => ["bucket: cross-kind never scored"],
    two_stubs_over_one_common_block_are_skipped => ["skip: same common block skipped"],
    the_same_block_skip_is_grammar_agnostic => [
        "skip: same review-common block skipped (skill grammar)",
    ],
    an_allowlisted_edge_is_counted_and_not_emitted => ["allowlist: edge suppressed"],

    // unit layer — classification
    three_schemas_read_as_a_common_candidate => ["classify: 3 schemas = COMMON-CANDIDATE"],
    two_schemas_read_as_a_cross_pair => ["classify: 2 schemas = CROSS-PAIR"],
    one_schema_reads_as_intra_schema => ["classify: 1 schema = INTRA-SCHEMA"],
    a_mixed_extends_cluster_is_an_extend_gap => ["classify: mixed extends adds EXTEND-GAP"],
    an_all_extends_cluster_is_not_a_gap => ["classify: all-extends is not a gap"],

    // the command fixtures
    the_register_cluster_is_found_and_tagged => [
        "e2e: register cluster found",
        "e2e: COMMON-CANDIDATE + EXTEND-GAP tagged",
    ],
    a_stubs_inherited_text_re_enters_scoring => ["e2e: extends resolution feeds stub text"],
    a_floor_member_is_flagged => ["e2e: floor flagged"],
    the_unrelated_controls_stay_unclustered => ["e2e: controls stay unclustered"],
    a_fully_suppressed_cluster_disappears_and_is_counted => [
        "e2e: fully-suppressed cluster gone",
        "e2e: suppression count reported",
        "e2e: stale allowlist ID warned",
    ],

    // the skill fixtures
    the_skill_fixtures_cluster_across_the_stub_and_its_sibling => [
        "skill e2e: in-dir schemas discovered and clustered",
        "skill e2e: stub resolves against review-common (EXTEND-GAP)",
        "skill e2e: floor members flagged",
    ],
    a_skills_only_run_reads_no_allowlist_and_leaves_the_control_alone => [
        "skill e2e: fixture run reads no live allowlist",
        "skill e2e: register control stays unclustered (skills-only run)",
    ],
    a_skill_edge_is_suppressed_like_a_command_edge => ["skill e2e: skill edge suppressed"],
    a_mixed_sweep_surfaces_the_cross_grammar_edge_and_scans_both_sets => [
        "mixed e2e: cross-grammar edge surfaces in the register cluster",
        "mixed e2e: both sets scanned",
    ],

    // the authoring fixtures
    the_authoring_stub_resolves_and_gaps_against_its_sibling => [
        "authoring e2e: stub resolves against authoring-common (EXTEND-GAP)",
    ],
    both_families_resolve_in_one_run => ["authoring e2e: both families resolve in one run"],
    a_stub_whose_library_carries_no_such_block_warns_and_never_clusters => [
        "authoring e2e: unknown-prefix stub warns, never clusters",
    ],
}

/// Python checks with no Rust referent, each carrying its reason. Record D6 makes the port the
/// retirement gate, so a probe that does not port has to be visible rather than absent.
///
/// Every row is about the Python script's own command-line surface — its exit code, its `--json`
/// flag — and none is about scoring. `not_applicable_under_d6` asserts what stands in their
/// place: the Rust report carries clusters and warnings and no severity or exit code at all.
const NOT_APPLICABLE: &[(&str, &str)] = &[
    (
        "e2e: exit 0 by default",
        "an assertion about the Python script's process exit code; the Rust detector is a \
         library call inside `migrate validate --report`, which is advisory and exits 0 whatever \
         the report says — asserted in `not_applicable_under_d6`",
    ),
    (
        "skill e2e: exit 0 by default",
        "same assertion over the skills-only run, same reason",
    ),
    (
        "authoring e2e: exit 0 by default",
        "same assertion over the authoring run, same reason",
    ),
    (
        "e2e: --exit-signal exits 1 on clusters",
        "the detector ships no `--exit-signal` flag; wave-plan §3 places similarity clusters in \
         the advisory set, printed by `migrate validate --report` at exit 0",
    ),
    (
        "e2e: --exit-signal exits 0 when suppressed clean",
        "same flag, same reason",
    ),
    (
        "e2e: --json parses",
        "there is no JSON surface; the Python's `--json` fed the layer-2 judgment pass, which is \
         not a wave-1 consumer",
    ),
];

/// Rust tests with no Python referent, so the ported ledger never inflates its coverage claim.
const EXTRA: &[(&str, &str)] = &[
    (
        "the_report_names_its_threshold_and_counts",
        "the report's header, scanned count and tag tally, which the Python asserted only \
         incidentally through substring matches on its stdout",
    ),
    (
        "an_allowlist_row_without_a_reason_is_warned",
        "an allowlist row carrying an empty `reason:`; the Python matrix never exercised it",
    ),
    (
        "the_ratio_reproduces_cpython_difflib",
        "17 reference vectors captured from CPython, a pin the Python matrix could not have",
    ),
    (
        "autojunk_is_ported_rather_than_skipped",
        "the autojunk witness, where a port without the heuristic returns ~0.95 instead of 0.07",
    ),
    (
        "the_quick_ratios_bound_the_real_one",
        "the two early-out upper bounds, asserted over the whole vector table",
    ),
    (
        "the_detector_reproduces_the_live_runs_figures_over_the_corpus",
        "the real corpus, pinned to the figures the live Python run reports",
    ),
];

/// The Python check names, read from the frozen extract of the script.
///
/// Through wave 5 this scanned `scripts/test-find-similar-rules.py` itself — every `check("…")`
/// call opens its name on the same line, so a line scan read the list the script would produce,
/// and a transcribed array could not go stale silently. The script retires at wave 6 with the
/// other two Python checkers (record D6), so its 48 names were lifted verbatim, in source order,
/// into `tests/fixtures/python-matrix/check-names.txt` immediately before the deletion. The
/// second source survives, and the assertion below is unchanged: the ledger's coverage claim is
/// still a set equation against a list this file does not itself declare.
fn python_probes_from_source() -> Vec<String> {
    let path = repo_root().join("crates/mochiko-cli/tests/fixtures/python-matrix/check-names.txt");
    let text =
        std::fs::read_to_string(&path).expect("the frozen Python matrix extract is readable");
    text.lines()
        .map(str::trim_end)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_string)
        .collect()
}

#[test]
fn the_recorded_python_names_are_the_scripts_own() {
    let live = python_probes_from_source();
    assert_eq!(
        live.len(),
        PYTHON_PROBES.len(),
        "the script declares {} checks; this file records {}",
        live.len(),
        PYTHON_PROBES.len()
    );
    let recorded: Vec<String> = PYTHON_PROBES.iter().map(|s| s.to_string()).collect();
    assert_eq!(
        live, recorded,
        "the recorded names have drifted from the frozen extract of \
         `scripts/test-find-similar-rules.py`"
    );
}

#[test]
fn the_whole_python_matrix_is_accounted_for() {
    let mut claims: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (test, _, python) in PORTED {
        for name in *python {
            claims.entry(name).or_default().push(test);
        }
    }
    for (name, _) in NOT_APPLICABLE {
        claims.entry(name).or_default().push("NOT_APPLICABLE");
    }

    let known: BTreeSet<&str> = PYTHON_PROBES.iter().copied().collect();
    assert_eq!(
        known.len(),
        PYTHON_PROBES.len(),
        "the Python names are unique"
    );

    let unclaimed: Vec<&str> = PYTHON_PROBES
        .iter()
        .copied()
        .filter(|name| !claims.contains_key(name))
        .collect();
    assert!(
        unclaimed.is_empty(),
        "Python probes claimed by no ledger: {unclaimed:#?}"
    );

    let twice: Vec<(&str, &Vec<&str>)> = claims
        .iter()
        .filter(|(_, by)| by.len() > 1)
        .map(|(name, by)| (*name, by))
        .collect();
    assert!(
        twice.is_empty(),
        "Python probes claimed more than once: {twice:#?}"
    );

    let invented: Vec<&str> = claims
        .keys()
        .copied()
        .filter(|name| !known.contains(name))
        .collect();
    assert!(
        invented.is_empty(),
        "ledger names that are not Python probes: {invented:#?}"
    );

    let ported: usize = PORTED.iter().map(|(_, _, python)| python.len()).sum();
    assert_eq!(
        (ported, NOT_APPLICABLE.len()),
        (42, 6),
        "the split this file's header states"
    );
    assert_eq!(ported + NOT_APPLICABLE.len(), PYTHON_PROBES.len());

    // The extras are Rust-side additions, so they must not appear as Python claims.
    for (test, _) in EXTRA {
        assert!(
            !PORTED.iter().any(|(name, _, _)| name == test),
            "{test} is listed as both ported and extra"
        );
    }
}

// ---------------------------------------------------------------------------
// the six probes that do not port
// ---------------------------------------------------------------------------

/// What stands in for the six probes `NOT_APPLICABLE` names.
///
/// Every one of them asserts something about the Python script's command-line surface: its exit
/// code, or its `--json` flag. The Rust detector has no command-line surface of its own — it is a
/// library call inside `migrate validate --report` — so the property that replaces all six is
/// that the report is advisory by construction. It carries clusters and warnings, and no
/// severity, no finding code and no exit code at all.
#[test]
fn not_applicable_under_d6() {
    assert_eq!(
        NOT_APPLICABLE.len(),
        6,
        "the ledger is the list; this test asserts what replaces it"
    );
    for (name, reason) in NOT_APPLICABLE {
        assert!(!reason.is_empty(), "{name} carries no reason");
    }

    let report = command_report();
    assert!(!report.clusters.is_empty());
    let rendered = similar::render_report(&report);
    assert!(!rendered.contains("FINDING"), "the report is advisory");
    assert!(!rendered.contains("rejecting"), "the report is advisory");
}
