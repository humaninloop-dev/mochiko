//! Integration tests for the hard set, and the smoke test that runs it over the shipped corpus.
//!
//! Every rejecting code is exercised twice: once against a synthetic corpus that satisfies it (the
//! positive control, which must come back clean) and once against a mutation that breaks exactly
//! that clause. A check that cannot be made to fail is not a check.
//!
//! The synthetic corpus is deliberately not the shipped one. It stays green while the real library
//! is mid-wave, and it can be mutated freely — the shipped files are never written by this suite.

use mochiko_cli::model::{DocKind, DocRef, Document};
use mochiko_cli::replay::State;
use mochiko_cli::validate::{self, census, Code, Family, Severity};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// the synthetic corpus
// ---------------------------------------------------------------------------

const COMMAND_LABELS: &str = r#"
kind: command-labels
labels:
  seats: Seat wiring.
  user-gate: Reserved to the user.
  evidence: What must be shown.
"#;

const SKILL_LABELS: &str = r#"
kind: skill-labels
labels:
  independence: Who is never whom.
  verdict: The clearing grammar.
  boundary: A jurisdiction line.
"#;

const COMMAND_COMMON: &str = r#"
kind: command-common
rules:
  - id: common.register
    labels: [seats]
    text: User-facing prose follows the output style.
  - id: common.model-tiering
    labels: [seats]
    text: Exploration rides ${explore_model}.
"#;

const REVIEW_COMMON: &str = r#"
kind: skill-common
rules:
  - id: review-common.author-grader
    labels: [independence]
    text: Never author, fix, or revise what you grade.
"#;

const AUTHORING_COMMON: &str = r#"
kind: skill-common
rules:
  - id: authoring-common.independent-grade
    labels: [independence]
    text: The produced ${artifact} is graded by an independent grader.
"#;

/// A command schema carrying all six canonical sections, a bound var, a declared condition, a
/// fail node with a live mirror, and a tombstone.
const COMMAND: &str = r#"
kind: command
command: demo
vars:
  explore_model: haiku
conditions:
  seats:
    values: [single, multi]
    resolution: standing-trigger
    note: fires when the run composes more than one seat.
  map:
    values: presence
    resolution: surface-presence
    note: the feature map.
moments:
  intent: The adaptive-probe stage.
sections:
  - id: demo.sec.roles
    title: Roles
    intent: Seat wiring.
    rules:
      - id: demo.lead
        labels: [seats]
        class: must
        kind: latitude
        text: The lead plans the run.
      - id: demo.single-seat
        labels: [seats]
        class: must
        text: One seat suffices.
        when: {seats: single}
      - id: demo.multi-seat
        labels: [seats]
        class: must
        text: More than one seat triggers the floor.
        when: {seats: [multi]}
      - id: demo.map-read
        labels: [evidence]
        class: must
        text: The map is an obligated read.
        when: {map: present}
      - id: demo.map-absent
        labels: [evidence]
        class: must
        text: A missing map is surfaced.
        when: {map: absent}
  - id: demo.sec.reserved
    title: Reserved
    intent: The user's calls.
    rules:
      - id: demo.gate-acceptance
        labels: [user-gate]
        class: floor
        kind: gate
        text: Acceptance is the user's.
  - id: demo.sec.tools
    title: Tools
    intent: Bindings.
    rules:
      - id: demo.register
        extends: common.register
        class: must
        kind: binding
      - id: demo.tiering
        extends: common.model-tiering
        class: must
        kind: routing
  - id: demo.sec.ways-of-working
    title: Ways of Working
    intent: How the run sequences itself.
    rules:
      - id: demo.probe-first
        labels: [seats]
        class: advisory
        text: Intent runs as probes.
  - id: demo.sec.boundaries
    title: Boundaries
    intent: The non-waivable floor.
    note: Deliberately empty — this run carries no floor beyond its gate.
    rules: []
  - id: demo.sec.fail-conditions
    title: Not done
    intent: The fail set.
    rules:
      - id: demo.fail.no-acceptance
        labels: [user-gate]
        class: floor
        kind: fail
        enforces: [demo.gate-acceptance]
        text: User acceptance not given.
      - id: demo.fail.unmirrored
        labels: [evidence]
        class: floor
        kind: fail
        enforces: []
        note: "D6 empty-with-reason: the obligation is owned by a sibling command."
        text: A deliberately unmirrored fail node.
tombstones:
  - id: demo.sec.harness
    disposition: superseded, rules redistributed across the six-set
"#;

const REVIEW_SKILL: &str = r#"
kind: skill
skill: review-demo
vars:
  verdict: clean
sections:
  - id: review-demo.sec.independence
    title: Independence
    intent: Grader, never author.
    rules:
      - id: review-demo.author-grader
        extends: review-common.author-grader
        class: floor
  - id: review-demo.sec.scope
    title: Scope
    intent: The seam.
    rules:
      - id: review-demo.sibling-split
        labels: [boundary]
        class: must
        kind: routing
        text: The sibling owns coverage; you own contradiction.
  - id: review-demo.sec.inputs
    title: Inputs
    intent: What is loaded first.
    note: Deliberately empty — the lens is loaded by its command.
    rules: []
  - id: review-demo.sec.verdict
    title: Verdict
    intent: The clearing grammar.
    rules:
      - id: review-demo.default-fail
        labels: [verdict]
        class: floor
        text: Never default to ${verdict}.
  - id: review-demo.sec.output
    title: Output
    intent: Where findings land.
    rules:
      - id: review-demo.evidence-floor
        labels: [verdict]
        class: floor
        kind: duty
        text: Findings land in the reviewed artifact.
  - id: review-demo.sec.reserved
    title: Reserved
    intent: The lead's clearing.
    rules:
      - id: review-demo.verdict-is-input
        labels: [verdict]
        class: floor
        kind: reservation
        text: Your verdict is input, never a clearing.
"#;

const AUTHORING_SKILL: &str = r#"
kind: skill
skill: authoring-demo
vars:
  artifact: manifest
sections:
  - id: authoring-demo.sec.independence
    title: Independence
    intent: Who grades.
    rules:
      - id: authoring-demo.independent-grade
        extends: authoring-common.independent-grade
        class: floor
  - id: authoring-demo.sec.scope
    title: Scope
    intent: What this authors.
    rules:
      - id: authoring-demo.owns
        labels: [boundary]
        class: must
        text: This skill authors the manifest and nothing else.
  - id: authoring-demo.sec.inputs
    title: Inputs
    intent: Obligated reads.
    note: Deliberately empty — the command's brief carries the reads.
    rules: []
  - id: authoring-demo.sec.artifact
    title: Artifact
    intent: The produced shape.
    rules:
      - id: authoring-demo.grammar
        labels: [boundary]
        class: must
        kind: binding
        text: The manifest carries one row per screen.
  - id: authoring-demo.sec.output
    title: Output
    intent: Where it lands.
    rules:
      - id: authoring-demo.lands
        labels: [boundary]
        class: must
        text: The manifest lands beside the prototype.
  - id: authoring-demo.sec.reserved
    title: Reserved
    intent: Never this seat's.
    rules:
      - id: authoring-demo.never-grades
        labels: [independence]
        class: floor
        kind: reservation
        text: This skill never grades its own output.
"#;

const PATTERNS_SKILL: &str = r#"
kind: skill
skill: patterns-demo
conditions:
  trigger:
    values: [fired, clear]
    resolution: entry-derived
    note: fired when both parts of the test are true.
sections:
  - id: patterns-demo.sec.trigger
    title: Trigger
    intent: When the floor fires.
    rules:
      - id: patterns-demo.two-part
        labels: [boundary]
        class: floor
        kind: gate
        text: The floor fires when both parts are true.
  - id: patterns-demo.sec.scope
    title: Scope
    intent: Hosting out-of-remit demands.
    rules:
      - id: patterns-demo.hosting
        labels: [boundary]
        class: must
        text: Adaptation moves the door, never lowers the ritual.
  - id: patterns-demo.sec.discipline
    title: Discipline
    intent: The legs.
    rules:
      - id: patterns-demo.leg-one
        labels: [independence]
        class: floor
        text: Production sits with a seat, never the lead.
        when: {trigger: fired}
      - id: patterns-demo.leg-two
        labels: [independence]
        class: floor
        text: A non-author seat reviews.
        when: {trigger: clear}
  - id: patterns-demo.sec.inputs
    title: Inputs
    intent: Read duties before the floor fires.
    note: Deliberately empty — the floor obligates no read before it fires.
    rules: []
  - id: patterns-demo.sec.disclosure
    title: Disclosure
    intent: The pinned line.
    rules:
      - id: patterns-demo.line
        labels: [verdict]
        class: must
        kind: duty
        text: Every close report carries one pinned line.
  - id: patterns-demo.sec.reserved
    title: Reserved
    intent: The user gate stays.
    rules:
      - id: patterns-demo.user-gate
        labels: [verdict]
        class: floor
        kind: reservation
        text: Rulings reserved to the user remain theirs.
"#;

fn doc_of(kind: DocKind, name: &str, yaml: &str) -> (DocRef, Document) {
    let value: serde_norway::Value = serde_norway::from_str(yaml).expect("fixture parses");
    let document = Document::from_value(kind, &value).expect("fixture decodes");
    (DocRef::new(kind, name), document)
}

/// The synthetic corpus, with `edit` applied to one document's YAML before it is decoded.
fn corpus_with(target: &str, edit: impl Fn(&str) -> String) -> State {
    let mut state = State::default();
    let sources: [(DocKind, &str, &str); 8] = [
        (DocKind::CommandLabels, "command-labels", COMMAND_LABELS),
        (DocKind::SkillLabels, "skill-labels", SKILL_LABELS),
        (DocKind::CommandCommon, "common", COMMAND_COMMON),
        (DocKind::SkillCommon, "skill-review-common", REVIEW_COMMON),
        (
            DocKind::SkillCommon,
            "skill-authoring-common",
            AUTHORING_COMMON,
        ),
        (DocKind::Command, "demo", COMMAND),
        (DocKind::Skill, "review-demo", REVIEW_SKILL),
        (DocKind::Skill, "authoring-demo", AUTHORING_SKILL),
    ];
    for (kind, name, yaml) in sources {
        let yaml = if name == target {
            edit(yaml)
        } else {
            yaml.to_string()
        };
        let (key, document) = doc_of(kind, name, &yaml);
        state.docs.insert(key, document);
    }
    let yaml = if target == "patterns-demo" {
        edit(PATTERNS_SKILL)
    } else {
        PATTERNS_SKILL.to_string()
    };
    let (key, document) = doc_of(DocKind::Skill, "patterns-demo", &yaml);
    state.docs.insert(key, document);
    state
}

fn corpus() -> State {
    corpus_with("", |s| s.to_string())
}

fn rejecting(state: &State) -> Vec<mochiko_cli::validate::Finding> {
    validate::validate(state)
        .into_iter()
        .filter(|f| f.is_rejecting())
        .collect()
}

fn codes(state: &State) -> BTreeSet<Code> {
    rejecting(state).into_iter().map(|f| f.code).collect()
}

/// Assert that replacing `from` with `to` in one document raises exactly `code`.
fn probe(target: &str, from: &str, to: &str, code: Code) {
    let state = corpus_with(target, |yaml| {
        assert!(
            yaml.contains(from),
            "probe anchor {from:?} is not in {target}"
        );
        yaml.replace(from, to)
    });
    let found = codes(&state);
    assert!(
        found.contains(&code),
        "mutating {target} ({from:?} -> {to:?}) should raise {code}, got {:?}",
        rejecting(&state)
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
    );
}

// ---------------------------------------------------------------------------
// the positive control
// ---------------------------------------------------------------------------

#[test]
fn the_synthetic_corpus_is_clean() {
    let findings = rejecting(&corpus());
    assert!(
        findings.is_empty(),
        "the positive control must carry no rejecting finding, got {:?}",
        findings.iter().map(|f| f.to_string()).collect::<Vec<_>>()
    );
}

#[test]
fn advisory_findings_are_reported_and_never_reject() {
    let all = validate::validate(&corpus());
    let advisory: Vec<_> = all.iter().filter(|f| !f.is_rejecting()).collect();
    assert!(!advisory.is_empty(), "the advisory reports do run");
    for finding in advisory {
        assert_eq!(finding.severity, Severity::Advisory);
        assert!(Code::ADVISORY.contains(&finding.code), "{}", finding.code);
    }
}

#[test]
fn a_finding_renders_as_code_schema_id_message() {
    let state = corpus_with("demo", |yaml| {
        yaml.replace("class: advisory", "class: mandatory")
    });
    let finding = rejecting(&state)
        .into_iter()
        .find(|f| f.code == Code::ClassUnknown)
        .expect("the mutation raises class-unknown");
    let rendered = finding.to_string();
    let columns: Vec<&str> = rendered.split(" · ").collect();
    assert_eq!(columns[0], "class-unknown");
    assert_eq!(columns[1], "command/demo");
    assert_eq!(columns[2], "demo.probe-first");
    assert!(columns[3].contains("mandatory"));
}

// ---------------------------------------------------------------------------
// one probe per rejecting clause
// ---------------------------------------------------------------------------

#[test]
fn a_missing_or_wrong_kind_discriminator_is_rejected() {
    probe(
        "demo",
        "kind: command\n",
        "kind: command-schema\n",
        Code::KindDiscriminator,
    );
    probe(
        "demo",
        "command: demo",
        "command: other",
        Code::KindDiscriminator,
    );
    probe(
        "command-labels",
        "kind: command-labels",
        "kind: labels",
        Code::KindDiscriminator,
    );
}

#[test]
fn a_family_section_set_that_is_not_exactly_the_six_is_rejected() {
    // A missing canonical section.
    probe(
        "demo",
        "  - id: demo.sec.tools",
        "  - id: demo.sec.gone",
        Code::SectionSet,
    );
    // An extra section outside the set.
    probe(
        "review-demo",
        "  - id: review-demo.sec.verdict",
        "  - id: review-demo.sec.artifact",
        Code::SectionSet,
    );
    // The patterns set is a full swap-out, not the review set.
    probe(
        "patterns-demo",
        "  - id: patterns-demo.sec.discipline",
        "  - id: patterns-demo.sec.verdict",
        Code::SectionSet,
    );
}

#[test]
fn a_malformed_id_is_rejected() {
    probe("demo", "id: demo.lead", "id: demo.Lead", Code::IdFormat);
    probe(
        "demo",
        "id: demo.lead",
        "id: demo.lead.extra.segment",
        Code::IdFormat,
    );
}

#[test]
fn an_id_that_does_not_lead_with_its_prefix_is_rejected() {
    probe("demo", "id: demo.lead", "id: other.lead", Code::IdPrefix);
    probe(
        "review-demo",
        "id: review-demo.sibling-split",
        "id: other.sibling-split",
        Code::IdPrefix,
    );
}

#[test]
fn sections_disagreeing_on_the_prefix_cannot_derive_one() {
    probe(
        "demo",
        "  - id: demo.sec.roles",
        "  - id: other.sec.roles",
        Code::IdPrefix,
    );
}

#[test]
fn a_duplicate_id_is_rejected() {
    probe(
        "demo",
        "id: demo.probe-first",
        "id: demo.lead",
        Code::IdDuplicate,
    );
}

#[test]
fn an_id_both_live_and_tombstoned_is_rejected() {
    probe(
        "demo",
        "  - id: demo.sec.harness",
        "  - id: demo.sec.roles",
        Code::TombstoneIntegrity,
    );
}

#[test]
fn a_label_outside_its_registry_is_rejected() {
    probe(
        "demo",
        "labels: [seats]",
        "labels: [invented]",
        Code::LabelUnknown,
    );
    probe(
        "review-demo",
        "labels: [boundary]",
        "labels: [seats]",
        Code::LabelUnknown,
    );
}

#[test]
fn an_unbound_placeholder_is_rejected_including_in_inherited_text() {
    probe(
        "demo",
        "text: The lead plans the run.",
        "text: The ${absent} plans.",
        Code::VarUnbound,
    );
    // The inherited text substitutes from the BINDING document's vars, so dropping the binding
    // schema's var must trip even though the text lives in the library.
    probe(
        "demo",
        "  explore_model: haiku",
        "  other_var: haiku",
        Code::VarUnbound,
    );
}

#[test]
fn an_extends_target_that_resolves_nowhere_is_rejected() {
    probe(
        "demo",
        "extends: common.register",
        "extends: common.absent",
        Code::ExtendsUnresolved,
    );
    probe(
        "review-demo",
        "extends: review-common.author-grader",
        "extends: review-common.absent",
        Code::ExtendsUnresolved,
    );
}

#[test]
fn extends_across_families_or_grammars_is_rejected() {
    // A skill stub reaching for the command library.
    probe(
        "review-demo",
        "extends: review-common.author-grader",
        "extends: common.register",
        Code::ExtendsCrossFamily,
    );
    // A review-family stub reaching for the authoring library.
    probe(
        "review-demo",
        "extends: review-common.author-grader",
        "extends: authoring-common.independent-grade",
        Code::ExtendsCrossFamily,
    );
    // The patterns family ships no library at all.
    probe(
        "patterns-demo",
        "        text: The floor fires when both parts are true.",
        "        extends: review-common.author-grader",
        Code::ExtendsCrossFamily,
    );
}

#[test]
fn an_extends_stub_with_no_local_class_is_rejected() {
    probe(
        "demo",
        "        extends: common.register\n        class: must\n",
        "        extends: common.register\n",
        Code::ExtendsClassLocal,
    );
}

#[test]
fn a_common_block_carrying_an_absence_meaningful_field_is_rejected() {
    for (field, literal) in [
        ("class", "class: must"),
        ("kind", "kind: gate"),
        ("enforces", "enforces: [common.register]"),
    ] {
        let state = corpus_with("common", |yaml| {
            yaml.replace(
                "  - id: common.register\n",
                &format!("  - id: common.register\n    {literal}\n"),
            )
        });
        assert!(
            codes(&state).contains(&Code::ExtendsClassLocal),
            "a block carrying `{field}:` must be rejected, got {:?}",
            rejecting(&state)
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn a_when_naming_an_undeclared_dimension_is_rejected() {
    probe(
        "demo",
        "when: {seats: single}",
        "when: {undeclared: single}",
        Code::WhenUndeclared,
    );
}

#[test]
fn a_when_naming_an_undeclared_value_is_rejected() {
    probe(
        "demo",
        "when: {seats: single}",
        "when: {seats: triple}",
        Code::WhenValue,
    );
    probe(
        "demo",
        "when: {map: present}",
        "when: {map: maybe}",
        Code::WhenValue,
    );
    // The list form resolves the same way as the scalar form.
    probe(
        "demo",
        "when: {seats: [multi]}",
        "when: {seats: [quadruple]}",
        Code::WhenValue,
    );
}

#[test]
fn a_malformed_condition_declaration_is_rejected() {
    probe(
        "demo",
        "    values: [single, multi]",
        "    values: single",
        Code::ConditionDeclaration,
    );
    probe(
        "demo",
        "    resolution: standing-trigger",
        "    resolution: whenever",
        Code::ConditionDeclaration,
    );
    probe(
        "demo",
        "    resolution: standing-trigger\n",
        "\n",
        Code::ConditionDeclaration,
    );
}

#[test]
fn a_moment_resolved_condition_naming_no_declared_moment_is_rejected() {
    probe(
        "demo",
        "    resolution: standing-trigger",
        "    resolution: moment-resolved(absent)",
        Code::MomentUndeclared,
    );
    // And the declared one resolves cleanly.
    let ok = corpus_with("demo", |yaml| {
        yaml.replace(
            "    resolution: standing-trigger",
            "    resolution: moment-resolved(intent)",
        )
    });
    assert!(!codes(&ok).contains(&Code::MomentUndeclared));
}

#[test]
fn an_enforces_target_that_does_not_resolve_is_rejected() {
    probe(
        "demo",
        "enforces: [demo.gate-acceptance]",
        "enforces: [demo.absent]",
        Code::EnforcesUnresolved,
    );
    probe(
        "demo",
        "enforces: [demo.gate-acceptance]",
        "enforces: [demo.sec.roles]",
        Code::EnforcesUnresolved,
    );
    probe(
        "demo",
        "enforces: [demo.gate-acceptance]",
        "enforces: [demo.sec.harness]",
        Code::EnforcesUnresolved,
    );
}

#[test]
fn a_fail_node_without_enforces_is_rejected_and_an_empty_one_needs_a_reason() {
    probe(
        "demo",
        "        enforces: [demo.gate-acceptance]\n",
        "",
        Code::EnforcesRequired,
    );
    probe(
        "demo",
        "        note: \"D6 empty-with-reason: the obligation is owned by a sibling command.\"\n",
        "",
        Code::EnforcesRequired,
    );
}

#[test]
fn enforces_on_a_node_that_is_not_a_fail_node_is_rejected() {
    probe(
        "demo",
        "        text: The lead plans the run.",
        "        enforces: [demo.gate-acceptance]\n        text: The lead plans the run.",
        Code::EnforcesMisplaced,
    );
}

#[test]
fn the_fail_segment_and_the_fail_kind_key_each_other_both_ways() {
    // `kind: fail` outside the segment.
    probe(
        "demo",
        "id: demo.fail.no-acceptance",
        "id: demo.no-acceptance",
        Code::FailSegment,
    );
    // The segment with no explicit `kind: fail` — the fail kind is never defaulted.
    probe(
        "demo",
        "        kind: fail\n        enforces: [demo.gate-acceptance]",
        "        enforces: [demo.gate-acceptance]",
        Code::FailSegment,
    );
    // The segment carrying some other kind.
    probe(
        "demo",
        "        kind: fail\n        enforces: [demo.gate-acceptance]",
        "        kind: gate\n        enforces: [demo.gate-acceptance]",
        Code::FailSegment,
    );
}

#[test]
fn command_grammar_inside_a_skill_schema_is_rejected() {
    probe(
        "review-demo",
        "        class: must\n        kind: routing",
        "        class: must\n        kind: fail",
        Code::SkillGrammar,
    );
    probe(
        "review-demo",
        "        kind: routing\n",
        "        kind: routing\n        enforces: [review-demo.author-grader]\n",
        Code::SkillGrammar,
    );
    probe(
        "review-demo",
        "kind: skill\nskill: review-demo\n",
        "kind: skill\nskill: review-demo\nmoments:\n  intent: A moment.\n",
        Code::SkillGrammar,
    );
}

#[test]
fn an_unknown_class_or_kind_is_rejected() {
    probe(
        "demo",
        "class: advisory",
        "class: mandatory",
        Code::ClassUnknown,
    );
    probe(
        "demo",
        "        class: must\n        kind: latitude",
        "        kind: latitude",
        Code::ClassUnknown,
    );
    probe(
        "demo",
        "kind: latitude",
        "kind: aspiration",
        Code::RuleKindUnknown,
    );
}

#[test]
fn a_rule_with_no_resolvable_text_is_rejected() {
    probe(
        "demo",
        "text: The lead plans the run.",
        "text: \"\"",
        Code::TextMissing,
    );
}

#[test]
fn an_empty_section_with_no_note_is_rejected() {
    probe(
        "demo",
        "    note: Deliberately empty — this run carries no floor beyond its gate.\n",
        "",
        Code::TextMissing,
    );
}

#[test]
fn a_malformed_anchor_on_a_rule_is_rejected() {
    probe(
        "demo",
        "        text: The lead plans the run.",
        "        anchor: not-a-real-anchor\n        text: The lead plans the run.",
        Code::AnchorFormat,
    );
    let ok = corpus_with("demo", |yaml| {
        yaml.replace(
            "        text: The lead plans the run.",
            "        anchor: \"2026-09-03 cli-schema-delivery [D2]\"\n        text: The lead plans the run.",
        )
    });
    assert!(!codes(&ok).contains(&Code::AnchorFormat));
}

// ---------------------------------------------------------------------------
// family and prefix derivation
// ---------------------------------------------------------------------------

#[test]
fn the_family_derives_from_the_skill_stem() {
    assert_eq!(Family::of("authoring-feature-map"), Family::Authoring);
    assert_eq!(Family::of("patterns-sound-loop"), Family::Patterns);
    assert_eq!(Family::of("review-feasibility"), Family::Review);
    // Everything without a minted prefix falls through to the review set by ruling.
    assert_eq!(Family::of("analysis-iterative"), Family::Review);
    assert_eq!(Family::of("testing-end-user"), Family::Review);
}

#[test]
fn each_family_carries_its_own_six_sections() {
    assert!(Family::Review.sections().contains(&"verdict"));
    assert!(Family::Authoring.sections().contains(&"artifact"));
    assert!(!Family::Authoring.sections().contains(&"verdict"));
    assert!(Family::Patterns.sections().contains(&"discipline"));
    assert_eq!(Family::Patterns.common_prefix(), None);
    assert_eq!(Family::Review.common_prefix(), Some("review-common"));
}

#[test]
fn a_common_librarys_block_prefix_comes_from_its_own_name() {
    assert_eq!(
        validate::common_prefix_of(&DocRef::new(DocKind::CommandCommon, "common")),
        "common"
    );
    assert_eq!(
        validate::common_prefix_of(&DocRef::new(DocKind::SkillCommon, "skill-review-common")),
        "review-common"
    );
    assert_eq!(
        validate::common_prefix_of(&DocRef::new(DocKind::SkillCommon, "skill-authoring-common")),
        "authoring-common"
    );
}

#[test]
fn every_rejecting_code_is_reachable() {
    // The state-level codes are probed above; the log-level ones live in the sibling suites.
    let log_level = [
        Code::GrammarParse,
        Code::GrammarHeader,
        Code::GrammarVersion,
        Code::SequenceCollision,
        Code::SequenceMismatch,
        Code::HashMismatch,
        Code::OpUnknown,
        Code::OpInapplicable,
        Code::MintOnce,
        Code::ProtectedExit,
    ];
    let state_level: Vec<Code> = Code::REJECTING
        .into_iter()
        .filter(|c| !log_level.contains(c))
        .collect();
    assert_eq!(
        state_level.len() + log_level.len(),
        Code::REJECTING.len(),
        "every rejecting code is owned by exactly one suite"
    );
    // AnchorFormat is raised on both paths; the rest of the state-level set is probed here.
    assert!(state_level.contains(&Code::AnchorFormat));
}

// ---------------------------------------------------------------------------
// the shipped corpus
// ---------------------------------------------------------------------------

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the repo root resolves from the crate directory")
}

/// Every shipped schema file, addressed the way genesis will address it: the document's own
/// `kind:` field names its kind, a skill's name is its directory, and everything else is the
/// file stem. The two kinds with no `kind:` field are templates (which carry `template:`) and
/// the shelf data file.
fn shipped_documents() -> Vec<(DocRef, serde_norway::Value)> {
    let root = repo_root();
    let mut paths: Vec<PathBuf> = std::fs::read_dir(root.join("plugins/mochiko/schemas"))
        .expect("the shipped schema directory exists")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("yaml"))
        .collect();
    let skills = root.join("plugins/mochiko/skills");
    for entry in std::fs::read_dir(&skills).expect("the skills directory exists") {
        let path = entry.expect("readable entry").path().join("schema.yaml");
        if path.is_file() {
            paths.push(path);
        }
    }
    paths.sort();

    paths
        .into_iter()
        .map(|path| {
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("{} is readable: {e}", path.display()));
            let value: serde_norway::Value = serde_norway::from_str(&text)
                .unwrap_or_else(|e| panic!("{} parses as YAML: {e}", path.display()));
            let stem = if path.file_name().and_then(|n| n.to_str()) == Some("schema.yaml") {
                path.parent()
                    .and_then(Path::file_name)
                    .and_then(|n| n.to_str())
                    .expect("a skill schema sits in its skill's directory")
                    .to_string()
            } else {
                path.file_stem()
                    .and_then(|n| n.to_str())
                    .expect("a schema file has a stem")
                    .to_string()
            };
            let declared = value.get("kind").and_then(|v| v.as_str());
            let kind = match declared.and_then(DocKind::parse) {
                Some(kind) => kind,
                None if value.get("template").is_some() => DocKind::Template,
                None => DocKind::Shelf,
            };
            (DocRef::new(kind, stem), value)
        })
        .collect()
}

fn shipped_state() -> State {
    let mut state = State::default();
    for (doc, value) in shipped_documents() {
        let document = Document::from_value(doc.kind, &value)
            .unwrap_or_else(|e| panic!("{doc} decodes into the model: {e}"));
        state.docs.insert(doc, document);
    }
    state
}

/// A1: the model is a lossless round trip over every shipped file.
///
/// This is the guarantee three later pieces rest on — the state hash, the genesis fidelity
/// fixture, and the derived views whose semantic equality with the shipped files bridges the
/// wave. Asserting it here means no field is normalised by accident.
#[test]
fn every_shipped_document_round_trips_through_the_model() {
    for (doc, original) in shipped_documents() {
        let decoded = Document::from_value(doc.kind, &original)
            .unwrap_or_else(|e| panic!("{doc} decodes: {e}"));
        let re_encoded = decoded.to_value();
        assert_eq!(
            mochiko_cli::model::canonical_hash(&original),
            mochiko_cli::model::canonical_hash(&re_encoded),
            "{doc} does not round-trip: the model normalises a field it must preserve.\n\
             original:    {original:?}\n\
             re-encoded:  {re_encoded:?}"
        );
    }
}

/// The two shipped rules whose empty `enforces:` carries its reason in a YAML comment.
///
/// The shipped checker reads that comment straight off the file; comments do not survive into a
/// typed model, so the migration grammar carries the reason as a rule `note:` instead. Loading
/// the raw files, as this test does, therefore sees two rules with an empty mirror and no stated
/// reason — which is the correct reading of the file as it stands. Genesis lifts both comments
/// into `note:` fields, and these two findings disappear with it.
const COMMENT_CARRIED_REASONS: [&str; 2] = [
    "setup.fail.unclosed-trace",
    "setup.fail.floor-category-uncovered",
];

#[test]
fn the_shipped_corpus_validates_with_no_rejecting_finding() {
    let state = shipped_state();
    let findings = rejecting(&state);
    let unexpected: Vec<_> = findings
        .iter()
        .filter(|f| {
            !(f.code == Code::EnforcesRequired
                && f.id
                    .as_deref()
                    .is_some_and(|id| COMMENT_CARRIED_REASONS.contains(&id)))
        })
        .collect();
    assert!(
        unexpected.is_empty(),
        "the shipped corpus must pass the hard set, got {} unexpected findings:\n{}",
        unexpected.len(),
        unexpected
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert_eq!(
        findings.len(),
        COMMENT_CARRIED_REASONS.len(),
        "the only findings the raw corpus may raise are the two comment-carried reasons"
    );
}

/// The other half of the claim above: with the reasons carried as data, as genesis will carry
/// them, the corpus is clean outright. Without this, the exemption above could hide a real defect.
#[test]
fn the_shipped_corpus_is_clean_once_the_comment_carried_reasons_are_data() {
    let mut state = shipped_state();
    let setup = DocRef::new(DocKind::Command, "setup");
    let schema = state
        .docs
        .get_mut(&setup)
        .and_then(Document::as_rules_mut)
        .expect("the setup command schema is in state");
    for id in COMMENT_CARRIED_REASONS {
        let rule = schema
            .find_rule_mut(id)
            .unwrap_or_else(|| panic!("{id} is a live rule in setup.yaml"));
        assert_eq!(
            rule.enforces.as_deref(),
            Some([].as_slice()),
            "{id} carries the explicitly empty mirror this test is about"
        );
        rule.note = Some("D6 empty-with-reason: owned by a sibling surface.".to_string());
    }
    let findings = rejecting(&state);
    assert!(
        findings.is_empty(),
        "with the reasons as data the corpus is clean, got:\n{}",
        findings
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// A3: the corpus pins. These are the figures the record carries, and a silent drift in any of
/// them means either the corpus changed or the model reads it differently than the shipped
/// checkers do.
#[test]
fn the_shipped_corpus_matches_its_recorded_census() {
    let state = shipped_state();
    assert_eq!(state.docs.len(), 50, "the schema class is 50 files");

    let census = census(&state);
    let (command_rules, command_floors) =
        census.get(&DocKind::Command).copied().unwrap_or_default();
    let (skill_rules, skill_floors) = census.get(&DocKind::Skill).copied().unwrap_or_default();

    assert_eq!(command_rules, 321, "live command rules");
    assert_eq!(skill_rules, 695, "live skill rules");
    assert_eq!(command_rules + skill_rules, 1016, "live rules in total");
    assert_eq!(skill_floors, 226, "skill floors");
    // The record's 112 is a `grep -c 'class: floor'` figure. Two of those matches are prose
    // inside rule text (architecture.yaml and implement.yaml each name `class: floor` in a
    // sentence), so the declared floors are 110 — the same figure the shipped checker reports.
    assert_eq!(command_floors, 110, "declared command floors");

    let fail_nodes = state
        .docs
        .iter()
        .filter(|(doc, _)| doc.kind == DocKind::Command)
        .filter_map(|(_, d)| d.as_rules())
        .flat_map(|s| s.rules())
        .filter(|r| r.is_fail())
        .count();
    assert_eq!(fail_nodes, 36, "command fail nodes");
}

#[test]
fn the_shipped_corpus_covers_every_document_kind_the_store_holds() {
    let state = shipped_state();
    let kinds: BTreeSet<DocKind> = state.docs.keys().map(|d| d.kind).collect();
    for kind in DocKind::ALL {
        assert!(kinds.contains(&kind), "no shipped document of kind {kind}");
    }
}
