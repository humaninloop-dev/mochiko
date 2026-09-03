//! Integration tests for log loading and the replay engine.
//!
//! Every test writes its fixture log under `CARGO_TARGET_TMPDIR`, which Cargo places inside
//! `target/` — the test suite never writes outside the build directory.

use mochiko_cli::migration;
use mochiko_cli::model::{ordered_get, DocKind, DocRef, Document, WhenValue};
use mochiko_cli::replay::{self, Replay};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

// --- fixtures ---

static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// A fresh, empty log directory inside `target/`.
fn log_dir(tag: &str) -> PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("replay-{tag}-{n}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("fixture log dir is creatable");
    dir
}

/// Write one migration file into the log directory.
fn write(dir: &std::path::Path, name: &str, body: &str) {
    std::fs::write(dir.join(name), body).expect("fixture migration is writable");
}

/// A genesis migration importing one small command schema, one skill schema and one registry.
const GENESIS: &str = r#"
grammar: 1
id: 0001-genesis
sequence: 1
intent: Import a minimal corpus so the ops have something to act on.
changes:
  - op: import-document
    kind: command-labels
    name: command-labels
    content:
      kind: command-labels
      labels:
        seats: Seat wiring.
        user-gate: Reserved to the user.
  - op: import-document
    kind: command
    name: demo
    content:
      kind: command
      command: demo
      vars:
        seat: product-manager
      conditions:
        shape:
          values: [single, multi]
          resolution: standing-trigger
          note: fires when the run composes more than one seat.
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
              text: The lead plans the run.
            - id: demo.floor-rule
              labels: [user-gate]
              class: floor
              text: A floor obligation.
            - id: demo.plain
              labels: [seats]
              class: advisory
              text: An ordinary rule.
              when: {shape: multi}
        - id: demo.sec.fail-conditions
          title: Not done
          intent: The fail set.
          rules:
            - id: demo.fail.ungraded
              labels: [seats]
              class: floor
              kind: fail
              enforces: [demo.lead]
              text: Never graded.
  - op: import-document
    kind: template
    name: spec
    content:
      template: spec
      title: Specification
"#;

fn replay_of(dir: &std::path::Path) -> Replay {
    replay::replay_dir(dir).expect("the log directory is readable")
}

/// Replay a genesis plus one extra migration, returning the result.
fn with_followup(tag: &str, followup: &str) -> Replay {
    let dir = log_dir(tag);
    write(&dir, "0001-genesis.yaml", GENESIS);
    write(&dir, "0002-change.yaml", followup);
    replay_of(&dir)
}

fn followup(intent: &str, changes: &str) -> String {
    format!("grammar: 1\nid: 0002-change\nsequence: 2\nintent: {intent}\nchanges:\n{changes}")
}

fn demo(state: &replay::State) -> &mochiko_cli::model::RuleSchema {
    state
        .docs
        .get(&DocRef::new(DocKind::Command, "demo"))
        .and_then(Document::as_rules)
        .expect("the demo command schema is in state")
}

fn codes(replay: &Replay) -> Vec<&str> {
    replay.findings.iter().map(|f| f.code.as_str()).collect()
}

fn assert_clean(replay: &Replay) {
    assert!(
        replay.findings.is_empty(),
        "expected a clean replay, got {:?}",
        replay
            .findings
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
    );
}

// --- log loading ---

#[test]
fn a_log_replays_in_sequence_order_and_tolerates_gaps() {
    let dir = log_dir("order");
    // Written out of order, with a gap at 3 — the log is ordered by the header, not the listing.
    write(
        &dir,
        "0009-third.yaml",
        &followup_n(9, "demo.plain", "Third."),
    );
    write(&dir, "0001-genesis.yaml", GENESIS);
    write(
        &dir,
        "0004-second.yaml",
        &followup_n(4, "demo.plain", "Second."),
    );

    let replay = replay_of(&dir);
    assert_clean(&replay);
    assert_eq!(
        replay.sequences(),
        vec![1, 4, 9],
        "gaps in the sequence are legal; order is not"
    );
    assert_eq!(
        demo(&replay.state).find_rule("demo.plain").unwrap().text,
        Some("Third.".to_string()),
        "the last migration in sequence order wins"
    );
}

fn followup_n(sequence: u32, id: &str, text: &str) -> String {
    format!(
        "grammar: 1\nid: {sequence:04}-reword\nsequence: {sequence}\nintent: Reword one rule.\n\
         changes:\n  - {{op: reword-rule, schema: command/demo, id: {id}, text: {text}}}\n"
    )
}

#[test]
fn a_sequence_collision_is_rejected() {
    let dir = log_dir("collision");
    write(&dir, "0001-genesis.yaml", GENESIS);
    write(&dir, "0002-a.yaml", &followup_n(2, "demo.plain", "A."));
    write(&dir, "0002-b.yaml", &followup_n(2, "demo.plain", "B."));

    let replay = replay_of(&dir);
    assert!(
        codes(&replay).contains(&"sequence-collision"),
        "two files at one sequence must be rejected, got {:?}",
        codes(&replay)
    );
    assert!(!replay.is_deliverable());
}

#[test]
fn a_file_that_is_not_a_migration_is_reported_not_skipped() {
    let dir = log_dir("broken");
    write(&dir, "0001-genesis.yaml", GENESIS);
    write(&dir, "0002-broken.yaml", "grammar: 1\n  bad: [indent\n");
    let replay = replay_of(&dir);
    assert!(
        codes(&replay).contains(&"grammar-parse"),
        "{:?}",
        codes(&replay)
    );
}

#[test]
fn a_non_migration_filename_is_ignored() {
    let dir = log_dir("readme");
    write(&dir, "0001-genesis.yaml", GENESIS);
    write(&dir, "README.md", "# The log\n");
    assert_clean(&replay_of(&dir));
}

// --- import and read-back ---

#[test]
fn import_document_round_trips_a_document_field_for_field() {
    let dir = log_dir("import");
    write(&dir, "0001-genesis.yaml", GENESIS);
    let replay = replay_of(&dir);
    assert_clean(&replay);
    assert_eq!(replay.state.docs.len(), 3);

    let schema = demo(&replay.state);
    assert_eq!(schema.declared_kind.as_deref(), Some("command"));
    assert_eq!(schema.declared_name.as_deref(), Some("demo"));
    assert_eq!(schema.sections.len(), 2);
    assert_eq!(
        ordered_get(&schema.vars, "seat").and_then(|v| v.as_str()),
        Some("product-manager")
    );
    assert_eq!(
        ordered_get(&schema.moments, "intent").map(String::as_str),
        Some("The adaptive-probe stage.")
    );

    let condition = ordered_get(&schema.conditions, "shape").expect("declared condition");
    assert!(condition.tokens().contains("single"));
    assert_eq!(condition.resolution.as_deref(), Some("standing-trigger"));

    let rule = schema.find_rule("demo.plain").expect("the rule is live");
    assert_eq!(
        rule.labels.as_deref(),
        Some(["seats".to_string()].as_slice())
    );
    assert_eq!(rule.class.as_deref(), Some("advisory"));
    // A1: a scalar `when:` value stays a scalar, never normalised into a one-element list.
    assert_eq!(
        ordered_get(&rule.when, "shape"),
        Some(&WhenValue::Scalar(serde_norway::Value::String(
            "multi".into()
        )))
    );

    let fail = schema
        .find_rule("demo.fail.ungraded")
        .expect("the fail node is live");
    assert_eq!(
        fail.enforces.as_deref(),
        Some(["demo.lead".to_string()].as_slice())
    );

    // A template is carried opaquely, with no grammar imposed on it.
    let template = replay
        .state
        .docs
        .get(&DocRef::new(DocKind::Template, "spec"))
        .expect("the template is in state");
    assert!(matches!(template, Document::Opaque(_)));
}

#[test]
fn importing_an_existing_document_is_rejected() {
    let replay = with_followup(
        "reimport",
        &followup(
            "Import the same document twice.",
            "  - {op: import-document, kind: command, name: demo, content: {kind: command}}\n",
        ),
    );
    assert!(
        codes(&replay).contains(&"op-inapplicable"),
        "{:?}",
        codes(&replay)
    );
}

#[test]
fn replace_document_is_legal_for_a_template_and_rejected_for_a_schema() {
    let ok = with_followup(
        "replace-template",
        &followup(
            "Replace the template wholesale.",
            "  - {op: replace-document, kind: template, name: spec, content: {template: spec, title: New}}\n",
        ),
    );
    assert_clean(&ok);

    let rejected = with_followup(
        "replace-schema",
        &followup(
            "Try to replace a rule-bearing document wholesale.",
            "  - {op: replace-document, kind: command, name: demo, content: {kind: command}}\n",
        ),
    );
    assert!(
        codes(&rejected).contains(&"op-inapplicable"),
        "a rule-bearing document changes one node at a time, got {:?}",
        codes(&rejected)
    );
}

// --- the section and rule ops ---

#[test]
fn mint_and_tombstone_a_section() {
    let minted = with_followup(
        "mint-section",
        &followup(
            "Mint a section.",
            "  - {op: mint-section, schema: command/demo, section: {id: demo.sec.tools, title: Tools, intent: Bindings.}}\n",
        ),
    );
    assert_clean(&minted);
    let section = demo(&minted.state)
        .find_section("demo.sec.tools")
        .expect("minted");
    assert_eq!(section.title, "Tools");
    assert!(section.rules.is_empty(), "a minted section starts empty");

    let tombstoned = with_followup(
        "tombstone-section",
        &followup(
            "Mint an empty section, then tombstone it.",
            "  - {op: mint-section, schema: command/demo, section: {id: demo.sec.tools, title: Tools, intent: Bindings.}}\n\
             \x20 - {op: tombstone-section, schema: command/demo, id: demo.sec.tools, disposition: superseded}\n",
        ),
    );
    assert_clean(&tombstoned);
    let schema = demo(&tombstoned.state);
    assert!(schema.find_section("demo.sec.tools").is_none());
    assert!(schema.is_tombstoned("demo.sec.tools"));
}

#[test]
fn tombstoning_a_section_that_still_holds_rules_is_rejected() {
    // Otherwise a section tombstone would retire its rules implicitly, and a floor rule would
    // leave with no anchor — the protected-exit check bypassed by a level of indirection.
    let replay = with_followup(
        "tombstone-populated",
        &followup(
            "Tombstone a section that still holds rules.",
            "  - {op: tombstone-section, schema: command/demo, id: demo.sec.roles, disposition: superseded}\n",
        ),
    );
    assert!(
        codes(&replay).contains(&"op-inapplicable"),
        "{:?}",
        codes(&replay)
    );
    assert!(
        demo(&replay.state).find_section("demo.sec.roles").is_some(),
        "a rejected op leaves the section untouched"
    );
}

#[test]
fn mint_rule_places_the_rule_in_its_named_section() {
    let replay = with_followup(
        "mint-rule",
        &followup(
            "Mint a rule.",
            "  - {op: mint-rule, schema: command/demo, section: demo.sec.roles, rule: {id: demo.new, labels: [seats], class: must, text: A new rule.}}\n",
        ),
    );
    assert_clean(&replay);
    let section = demo(&replay.state).find_section("demo.sec.roles").unwrap();
    assert_eq!(section.rules.last().unwrap().id, "demo.new");
}

#[test]
fn mint_rule_into_a_missing_section_is_rejected() {
    let replay = with_followup(
        "mint-rule-nowhere",
        &followup(
            "Mint a rule into a section that does not exist.",
            "  - {op: mint-rule, schema: command/demo, section: demo.sec.absent, rule: {id: demo.new, class: must, text: T}}\n",
        ),
    );
    assert!(
        codes(&replay).contains(&"op-inapplicable"),
        "{:?}",
        codes(&replay)
    );
}

#[test]
fn a_reword_keeps_the_id_and_changes_only_the_text() {
    let replay = with_followup(
        "reword",
        &followup(
            "Reword one rule.",
            "  - {op: reword-rule, schema: command/demo, id: demo.plain, text: Reworded.}\n",
        ),
    );
    assert_clean(&replay);
    let rule = demo(&replay.state)
        .find_rule("demo.plain")
        .expect("id survives a reword");
    assert_eq!(rule.text.as_deref(), Some("Reworded."));
    assert_eq!(
        rule.class.as_deref(),
        Some("advisory"),
        "a reword touches text alone"
    );
    assert!(
        ordered_get(&rule.when, "shape").is_some(),
        "a reword leaves the rule's `when:` guard alone"
    );
}

#[test]
fn a_move_keeps_the_id_and_changes_only_the_section() {
    let replay = with_followup(
        "move",
        &followup(
            "Move a rule between sections.",
            "  - {op: move-rule, schema: command/demo, id: demo.plain, section: demo.sec.fail-conditions}\n",
        ),
    );
    assert_clean(&replay);
    let schema = demo(&replay.state);
    assert!(schema
        .find_section("demo.sec.roles")
        .unwrap()
        .rules
        .iter()
        .all(|r| r.id != "demo.plain"));
    assert!(schema
        .find_section("demo.sec.fail-conditions")
        .unwrap()
        .rules
        .iter()
        .any(|r| r.id == "demo.plain"));
}

#[test]
fn set_rule_field_writes_every_settable_field_and_a_null_clears_it() {
    for (field, literal) in [
        ("labels", "[user-gate]"),
        ("class", "must"),
        ("kind", "gate"),
        ("when", "{shape: single}"),
        ("pointer", "mochiko:patterns-sound-loop"),
        ("extends", "common.register"),
        ("enforces", "[demo.lead]"),
        ("anchor", "\"2026-09-03 demo-session [D1]\""),
        ("note", "A stated reason."),
    ] {
        let set = with_followup(
            &format!("set-{field}"),
            &followup(
                "Set one rule field.",
                &format!(
                    "  - {{op: set-rule-field, schema: command/demo, id: demo.plain, field: {field}, value: {literal}}}\n"
                ),
            ),
        );
        let rule = demo(&set.state).find_rule("demo.plain").unwrap().clone();
        let present = match field {
            "labels" => rule.labels.is_some(),
            "class" => rule.class.as_deref() == Some("must"),
            "kind" => rule.kind.as_deref() == Some("gate"),
            "when" => ordered_get(&rule.when, "shape").is_some(),
            "pointer" => rule.pointer.is_some(),
            "extends" => rule.extends.is_some(),
            "enforces" => rule.enforces.is_some(),
            "anchor" => rule.anchor.is_some(),
            _ => rule.note.is_some(),
        };
        assert!(present, "`{field}` was not written by set-rule-field");

        let cleared = with_followup(
            &format!("clear-{field}"),
            &followup(
                "Clear one rule field.",
                &format!(
                    "  - {{op: set-rule-field, schema: command/demo, id: demo.plain, field: {field}, value: ~}}\n"
                ),
            ),
        );
        let rule = demo(&cleared.state)
            .find_rule("demo.plain")
            .unwrap()
            .clone();
        let absent = match field {
            "labels" => rule.labels.is_none(),
            "class" => rule.class.is_none(),
            "kind" => rule.kind.is_none(),
            "when" => rule.when.is_empty(),
            "pointer" => rule.pointer.is_none(),
            "extends" => rule.extends.is_none(),
            "enforces" => rule.enforces.is_none(),
            "anchor" => rule.anchor.is_none(),
            _ => rule.note.is_none(),
        };
        assert!(absent, "`value: ~` did not clear `{field}`");
    }
}

#[test]
fn an_unknown_rule_field_is_rejected_at_parse() {
    let dir = log_dir("bad-field");
    write(&dir, "0001-genesis.yaml", GENESIS);
    write(
        &dir,
        "0002-change.yaml",
        &followup(
            "Set a field that does not exist.",
            "  - {op: set-rule-field, schema: command/demo, id: demo.plain, field: identifier, value: x}\n",
        ),
    );
    let replay = replay_of(&dir);
    assert!(!replay.is_deliverable());
    assert!(
        replay
            .findings
            .iter()
            .any(|f| f.message.contains("identifier")),
        "the finding names the rejected field"
    );
}

#[test]
fn set_var_set_condition_and_set_moment_write_and_clear() {
    let written = with_followup(
        "set-blocks",
        &followup(
            "Write one var, one condition and one moment.",
            "  - {op: set-var, schema: command/demo, name: reviewer, value: devils-advocate}\n\
             \x20 - {op: set-condition, schema: command/demo, name: map, spec: {values: presence, resolution: surface-presence}}\n\
             \x20 - {op: set-moment, schema: command/demo, name: close, text: The closing step.}\n",
        ),
    );
    assert_clean(&written);
    let schema = demo(&written.state);
    assert!(ordered_get(&schema.vars, "reviewer").is_some());
    assert!(ordered_get(&schema.conditions, "map")
        .unwrap()
        .is_presence());
    assert!(ordered_get(&schema.moments, "close").is_some());

    let cleared = with_followup(
        "clear-blocks",
        &followup(
            "Clear one var, one condition and one moment.",
            "  - {op: set-var, schema: command/demo, name: seat, value: ~}\n\
             \x20 - {op: set-condition, schema: command/demo, name: shape, spec: ~}\n\
             \x20 - {op: set-moment, schema: command/demo, name: intent, text: ~}\n",
        ),
    );
    let schema = demo(&cleared.state);
    assert!(ordered_get(&schema.vars, "seat").is_none());
    assert!(ordered_get(&schema.conditions, "shape").is_none());
    assert!(ordered_get(&schema.moments, "intent").is_none());
}

#[test]
fn a_moment_is_command_grammar_only() {
    let dir = log_dir("skill-moment");
    write(&dir, "0001-genesis.yaml", GENESIS);
    write(
        &dir,
        "0002-skill.yaml",
        &followup(
            "Import a skill, then try to give it a moment.",
            "  - {op: import-document, kind: skill, name: review-demo, content: {kind: skill, skill: review-demo}}\n\
             \x20 - {op: set-moment, schema: skill/review-demo, name: close, text: Nope.}\n",
        ),
    );
    let replay = replay_of(&dir);
    assert!(
        codes(&replay).contains(&"skill-grammar"),
        "{:?}",
        codes(&replay)
    );
}

// --- registries ---

#[test]
fn registry_add_and_retire_never_delete_a_label() {
    let replay = with_followup(
        "registry",
        &followup(
            "Add one label and retire another.",
            "  - {op: registry-add, registry: command-labels, label: evidence, meaning: What must be shown.}\n\
             \x20 - {op: registry-retire, registry: command-labels, label: seats, note: Retired by ruling.}\n",
        ),
    );
    assert_clean(&replay);
    let registry = replay
        .state
        .docs
        .get(&DocRef::new(DocKind::CommandLabels, "command-labels"))
        .and_then(Document::as_labels)
        .expect("the registry is in state");
    assert!(ordered_get(&registry.labels, "evidence").is_some());
    assert!(
        ordered_get(&registry.labels, "seats").is_none(),
        "a retired label leaves the live vocabulary"
    );
    assert_eq!(registry.retired.len(), 1, "and is recorded, never deleted");
    assert_eq!(registry.retired[0].label, "seats");
}

#[test]
fn re_adding_a_live_label_and_retiring_an_absent_one_are_rejected() {
    let readd = with_followup(
        "readd",
        &followup(
            "Add a label that is already live.",
            "  - {op: registry-add, registry: command-labels, label: seats, meaning: Again.}\n",
        ),
    );
    assert!(
        codes(&readd).contains(&"op-inapplicable"),
        "{:?}",
        codes(&readd)
    );

    let retire = with_followup(
        "retire-absent",
        &followup(
            "Retire a label that is not there.",
            "  - {op: registry-retire, registry: command-labels, label: absent, note: N.}\n",
        ),
    );
    assert!(
        codes(&retire).contains(&"op-inapplicable"),
        "{:?}",
        codes(&retire)
    );
}

// --- the id lifecycle ---

#[test]
fn mint_once_forbids_reviving_a_tombstoned_id() {
    let replay = with_followup(
        "mint-once",
        &followup(
            "Tombstone a rule, then mint its id again.",
            "  - {op: tombstone-rule, schema: command/demo, id: demo.plain, disposition: dropped}\n\
             \x20 - {op: mint-rule, schema: command/demo, section: demo.sec.roles, rule: {id: demo.plain, class: must, text: Reborn.}}\n",
        ),
    );
    assert!(
        codes(&replay).contains(&"mint-once"),
        "{:?}",
        codes(&replay)
    );
}

#[test]
fn minting_a_live_id_again_is_rejected() {
    let replay = with_followup(
        "mint-live",
        &followup(
            "Mint an id that is already live.",
            "  - {op: mint-rule, schema: command/demo, section: demo.sec.roles, rule: {id: demo.lead, class: must, text: Twice.}}\n",
        ),
    );
    assert!(
        codes(&replay).contains(&"mint-once"),
        "{:?}",
        codes(&replay)
    );
}

#[test]
fn a_tombstoned_id_is_never_also_live() {
    let replay = with_followup(
        "tombstone",
        &followup(
            "Tombstone one rule.",
            "  - {op: tombstone-rule, schema: command/demo, id: demo.plain, disposition: dropped}\n",
        ),
    );
    assert_clean(&replay);
    let schema = demo(&replay.state);
    assert!(schema.find_rule("demo.plain").is_none());
    assert!(schema.is_tombstoned("demo.plain"));
}

#[test]
fn acting_on_a_rule_that_is_not_live_is_rejected() {
    for op in [
        "  - {op: reword-rule, schema: command/demo, id: demo.absent, text: T}\n",
        "  - {op: move-rule, schema: command/demo, id: demo.absent, section: demo.sec.roles}\n",
        "  - {op: tombstone-rule, schema: command/demo, id: demo.absent, disposition: d}\n",
        "  - {op: set-rule-field, schema: command/demo, id: demo.absent, field: class, value: must}\n",
    ] {
        let replay = with_followup("absent", &followup("Act on an absent rule.", op));
        assert!(
            codes(&replay).contains(&"op-inapplicable"),
            "op {op} should be inapplicable, got {:?}",
            codes(&replay)
        );
    }
}

// --- protected content (D2) ---

#[test]
fn protected_content_leaves_only_by_a_ruling_anchored_supersession() {
    // A floor rule, a fail rule, and a rule already carrying an anchor are each protected.
    for id in ["demo.floor-rule", "demo.fail.ungraded"] {
        let replay = with_followup(
            "protected",
            &followup(
                "Tombstone protected content without a ruling.",
                &format!("  - {{op: tombstone-rule, schema: command/demo, id: {id}, disposition: dropped}}\n"),
            ),
        );
        assert!(
            codes(&replay).contains(&"protected-exit"),
            "{id} is protected and must not leave by a bare tombstone, got {:?}",
            codes(&replay)
        );
    }

    let anchored = with_followup(
        "protected-anchored",
        &followup(
            "Anchor an ordinary rule, then try to tombstone it.",
            "  - {op: set-rule-field, schema: command/demo, id: demo.plain, field: anchor, value: \"2026-09-03 demo [D1]\"}\n\
             \x20 - {op: tombstone-rule, schema: command/demo, id: demo.plain, disposition: dropped}\n",
        ),
    );
    assert!(
        codes(&anchored).contains(&"protected-exit"),
        "an anchored rule is protected, got {:?}",
        codes(&anchored)
    );
}

#[test]
fn supersede_rule_with_an_anchor_retires_protected_content_cleanly() {
    let replay = with_followup(
        "supersede",
        &followup(
            "Supersede a floor rule by ruling.",
            "  - {op: supersede-rule, schema: command/demo, id: demo.floor-rule, disposition: superseded by D4, anchor: \"2026-09-03 cli-schema-delivery [D4]\"}\n",
        ),
    );
    assert_clean(&replay);
    let schema = demo(&replay.state);
    assert!(schema.find_rule("demo.floor-rule").is_none());
    assert!(schema.is_tombstoned("demo.floor-rule"));
}

#[test]
fn an_unprotected_rule_may_be_superseded_too() {
    let replay = with_followup(
        "supersede-plain",
        &followup(
            "Supersede an ordinary rule.",
            "  - {op: supersede-rule, schema: command/demo, id: demo.plain, disposition: superseded, anchor: \"2026-09-03 demo [D1]\"}\n",
        ),
    );
    assert_clean(&replay);
}

#[test]
fn a_malformed_anchor_is_rejected() {
    for anchor in ["not-a-date demo", "2026-9-3 demo", "2026-09-03"] {
        let replay = with_followup(
            "anchor",
            &followup(
                "Supersede with a malformed anchor.",
                &format!(
                    "  - {{op: supersede-rule, schema: command/demo, id: demo.floor-rule, disposition: d, anchor: \"{anchor}\"}}\n"
                ),
            ),
        );
        assert!(
            codes(&replay).contains(&"anchor-format"),
            "anchor {anchor:?} should be rejected, got {:?}",
            codes(&replay)
        );
    }
}

// --- the content hash ---

#[test]
fn replaying_twice_yields_an_identical_content_hash() {
    let dir = log_dir("determinism");
    write(&dir, "0001-genesis.yaml", GENESIS);
    write(
        &dir,
        "0002-change.yaml",
        &followup_n(2, "demo.plain", "Once."),
    );

    let first = replay_of(&dir);
    let second = replay_of(&dir);
    assert_clean(&first);
    assert_eq!(
        first.state.content_hash(),
        second.state.content_hash(),
        "the same log must replay to the same state hash"
    );
}

#[test]
fn the_content_hash_moves_when_content_moves_and_not_otherwise() {
    let base = with_followup(
        "hash-base",
        &followup(
            "Reword one rule.",
            "  - {op: reword-rule, schema: command/demo, id: demo.plain, text: One.}\n",
        ),
    );
    let same = with_followup(
        "hash-same",
        &followup(
            "Reword the same rule the same way.",
            "  - {op: reword-rule, schema: command/demo, id: demo.plain, text: One.}\n",
        ),
    );
    let different = with_followup(
        "hash-diff",
        &followup(
            "Reword the same rule differently.",
            "  - {op: reword-rule, schema: command/demo, id: demo.plain, text: Two.}\n",
        ),
    );

    assert_eq!(base.state.content_hash(), same.state.content_hash());
    assert_ne!(base.state.content_hash(), different.state.content_hash());
}

#[test]
fn the_content_hash_is_prefixed_and_hex() {
    let dir = log_dir("hash-shape");
    write(&dir, "0001-genesis.yaml", GENESIS);
    let hash = replay_of(&dir).state.content_hash();
    let hex = hash
        .strip_prefix("sha256:")
        .expect("the hash names its algorithm");
    assert_eq!(hex.len(), 64);
}

// --- A2: the deliverability signal ---

#[test]
fn a_state_carrying_a_rejecting_finding_is_never_deliverable() {
    let dir = log_dir("deliverable");
    write(&dir, "0001-genesis.yaml", GENESIS);
    let clean = replay_of(&dir);
    assert!(clean.is_deliverable(), "a clean replay is deliverable");
    assert!(replay::load(&dir).is_ok());

    write(
        &dir,
        "0002-bad.yaml",
        &followup(
            "Act on a rule that is not there.",
            "  - {op: reword-rule, schema: command/demo, id: demo.absent, text: T}\n",
        ),
    );
    let broken = replay_of(&dir);
    assert!(
        !broken.is_deliverable(),
        "a state built with a rejecting finding must never be rendered from"
    );
    assert_eq!(broken.rejecting().count(), 1);
    let findings = replay::load(&dir).expect_err("load refuses an unsound log");
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].code.as_str(), "op-inapplicable");
}

#[test]
fn an_advisory_finding_never_blocks_delivery() {
    // The validator's advisory set is exit-0 by ruling; the replay must agree.
    let dir = log_dir("advisory");
    write(&dir, "0001-genesis.yaml", GENESIS);
    let replay = replay_of(&dir);
    assert!(replay
        .findings
        .iter()
        .all(|f| f.severity == mochiko_cli::validate::Severity::Reject));
    assert!(replay.is_deliverable());
}

// --- the version contract reaches the log level ---

#[test]
fn a_log_written_in_an_unsupported_grammar_halts() {
    let dir = log_dir("skew");
    write(
        &dir,
        "0001-genesis.yaml",
        &GENESIS.replace("grammar: 1", "grammar: 99"),
    );
    let replay = replay_of(&dir);
    assert!(
        codes(&replay).contains(&"grammar-version"),
        "{:?}",
        codes(&replay)
    );
    let message = replay.findings[0].message.clone();
    assert!(
        message.contains(migration::INSTALL_COMMAND),
        "the halt names the install command: {message}"
    );
}
