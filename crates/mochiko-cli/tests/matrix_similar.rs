//! The ported probe matrix for `scripts/find-similar-rules.py` — 48 probes.
//!
//! 45 port. The three that do not are named in `not_applicable_under_d6`, with their reasons, so
//! the retirement gate can be graded rather than taken on trust (record D6: the port is the
//! retirement gate).
//!
//! The Python's end-to-end layer drove the detector as a subprocess over temp directories and
//! asserted on its stdout. Here the same cases build the fixture as state and assert on the
//! returned clusters, with the report's text asserted where the probe was about the text.
//!
//! Beyond the matrix, two parity pins that the Python matrix never had:
//!
//! * **Reference vectors.** `ratio()` is compared with values captured from CPython's
//!   `difflib.SequenceMatcher`, including pairs where the autojunk heuristic changes the answer.
//! * **The whole corpus.** The figures the live detector reports over the real tree.

use mochiko_cli::model::{DocKind, DocRef, Document};
use mochiko_cli::replay::State;
use mochiko_cli::similar::{self, ScoredRule, Tag};
use std::collections::BTreeSet;
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
// the command fixtures (10 probes of the Python's 12; two are named not-applicable)
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
// the skill fixtures (9 probes)
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
// the authoring fixtures (4 probes)
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

#[test]
fn the_detector_reproduces_the_live_runs_figures_over_the_corpus() {
    let root = repo_root();
    let mut state = State::default();
    for file in mochiko_cli::genesis::scan(&root).expect("the corpus scans") {
        let document = Document::from_value(file.doc.kind, &file.value)
            .unwrap_or_else(|e| panic!("{} decodes: {e}", file.path.display()));
        state.docs.insert(file.doc, document);
    }
    let allowlist = root.join(similar::ALLOWLIST);
    let report = similar::clusters(&state, similar::DEFAULT_THRESHOLD, Some(&allowlist));

    // Measured by running `uv run scripts/find-similar-rules.py` against this tree.
    assert_eq!(report.scanned, 1016, "rules scanned");
    assert_eq!(report.scored, 146_572, "in-kind pairs scored");
    assert_eq!(report.clusters.len(), 0, "clusters");
    assert_eq!(report.suppressed_hits, 181, "allowlist-suppressed edges");
}

// ---------------------------------------------------------------------------
// the three probes that do not port
// ---------------------------------------------------------------------------

/// The Python probes with no Rust referent, named rather than dropped (record D6: the port is
/// the retirement gate, so a probe that does not port has to be visible).
///
/// * `e2e: --exit-signal exits 1 on clusters`
/// * `e2e: --exit-signal exits 0 when suppressed clean`
///   The detector ships no `--exit-signal` flag. Wave-plan §3 places similarity clusters in the
///   advisory set, printed by `migrate validate --report` at exit 0, and D6 keeps every heuristic
///   advisory. Porting the flag would add an exit-code signal this wave does not carry.
/// * `e2e: --json parses`
///   There is no JSON surface. The Python's `--json` fed the layer-2 judgment pass, which is not
///   a wave-1 consumer.
///
/// All three are about the Python script's own command-line surface, not about scoring, and no
/// scoring behaviour is lost with them.
#[test]
fn not_applicable_under_d6() {
    let not_applicable = [
        "--exit-signal exits 1 on clusters",
        "--exit-signal exits 0 when suppressed clean",
        "--json parses",
    ];
    assert_eq!(not_applicable.len(), 3);

    // What replaces them: the report is advisory by construction. It carries clusters and
    // warnings, and no severity or exit code at all.
    let report = command_report();
    assert!(!report.clusters.is_empty());
    let rendered = similar::render_report(&report);
    assert!(!rendered.contains("FINDING"), "the report is advisory");
}
