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

/// Write one migration file into the log directory, stamped with a valid body hash.
///
/// The `hash:` header is required, so every fixture carries one. Fixtures that are deliberately
/// unparseable (broken YAML, an unsupported grammar) cannot be hashed and are written as-is —
/// which is what those tests are about.
fn write(dir: &std::path::Path, name: &str, body: &str) {
    let stamped = migration::with_hash(name, body).unwrap_or_else(|_| body.to_string());
    std::fs::write(dir.join(name), stamped).expect("fixture migration is writable");
}

/// A genesis migration importing one small command schema, one skill schema and one registry.
const GENESIS: &str = r#"
grammar: 1
id: 0001-genesis
sequence: 1
intent: Import a minimal but valid corpus so the ops have something to act on.
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
        - id: demo.sec.reserved
          title: Reserved
          intent: The user's calls.
          note: Deliberately empty — this run reserves nothing beyond its floor.
          rules: []
        - id: demo.sec.tools
          title: Tools
          intent: Bindings.
          note: Deliberately empty — this run binds no tool.
          rules: []
        - id: demo.sec.ways-of-working
          title: Ways of Working
          intent: How the run sequences itself.
          note: Deliberately empty — sequencing is the lead's.
          rules: []
        - id: demo.sec.boundaries
          title: Boundaries
          intent: The non-waivable floor.
          note: Deliberately empty — the floor above carries it.
          rules: []
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
    assert_eq!(schema.sections.len(), 6, "the canonical six");
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
            "  - {op: mint-section, schema: command/demo, section: {id: demo.sec.extra, title: Extra, intent: A minted section.}}\n",
        ),
    );
    assert_clean(&minted);
    let section = demo(&minted.state)
        .find_section("demo.sec.extra")
        .expect("minted");
    assert_eq!(section.title, "Extra");
    assert!(section.rules.is_empty(), "a minted section starts empty");

    let tombstoned = with_followup(
        "tombstone-section",
        &followup(
            "Mint an empty section, then tombstone it.",
            "  - {op: mint-section, schema: command/demo, section: {id: demo.sec.extra, title: Extra, intent: A minted section.}}\n\
             \x20 - {op: tombstone-section, schema: command/demo, id: demo.sec.extra, disposition: superseded}\n",
        ),
    );
    assert_clean(&tombstoned);
    let schema = demo(&tombstoned.state);
    assert!(schema.find_section("demo.sec.extra").is_none());
    assert!(schema.is_tombstoned("demo.sec.extra"));
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
    let rejecting: Vec<String> = broken.rejecting().map(|f| f.to_string()).collect();
    assert_eq!(rejecting.len(), 1, "rejecting findings: {rejecting:?}");
    // `load` hands back everything it collected, advisory reports included, so a caller can
    // print one report; exactly one of them blocks.
    let findings = replay::load(&dir).expect_err("load refuses an unsound log");
    let blocking: Vec<&mochiko_cli::validate::Finding> =
        findings.iter().filter(|f| f.is_rejecting()).collect();
    assert_eq!(blocking.len(), 1, "findings: {findings:?}");
    assert_eq!(blocking[0].code.as_str(), "op-inapplicable");
    assert!(
        findings.len() > blocking.len(),
        "the advisory reports travel with the refusal"
    );
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

// ---------------------------------------------------------------------------
// Fix round 1 — B1: protection may not be lowered without a ruling
// ---------------------------------------------------------------------------

/// A migration carrying a header anchor, so a protected exit is authorised.
fn anchored_followup(intent: &str, changes: &str) -> String {
    format!(
        "grammar: 1\nid: 0002-change\nsequence: 2\nintent: {intent}\n\
         anchor: \"2026-09-03 cli-schema-delivery D2\"\nchanges:\n{changes}"
    )
}

#[test]
fn lowering_protection_without_a_ruling_is_a_protected_exit() {
    // The audit's probe: downgrade, then tombstone, in one unanchored migration.
    for (id, field, value) in [
        ("demo.floor-rule", "class", "advisory"),
        ("demo.fail.ungraded", "kind", "gate"),
    ] {
        let replay = with_followup(
            "downgrade",
            &followup(
                "Downgrade protected content, then retire it.",
                &format!(
                    "  - {{op: set-rule-field, schema: command/demo, id: {id}, field: {field}, value: {value}}}\n  - {{op: tombstone-rule, schema: command/demo, id: {id}, disposition: gone}}\n"
                ),
            ),
        );
        assert!(
            codes(&replay).contains(&"protected-exit"),
            "downgrading {id} via `{field}` must be a protected exit, got {:?}",
            codes(&replay)
        );
        assert!(
            demo(&replay.state).find_rule(id).is_some(),
            "{id} must still be live after the rejected downgrade"
        );
    }
}

#[test]
fn clearing_protection_without_a_ruling_is_a_protected_exit() {
    for (id, field) in [("demo.floor-rule", "class"), ("demo.fail.ungraded", "kind")] {
        let replay = with_followup(
            "clear-protection",
            &followup(
                "Clear the field that makes the rule protected.",
                &format!(
                    "  - {{op: set-rule-field, schema: command/demo, id: {id}, field: {field}, value: ~}}\n"
                ),
            ),
        );
        assert!(
            codes(&replay).contains(&"protected-exit"),
            "clearing `{field}` on {id} must be a protected exit, got {:?}",
            codes(&replay)
        );
    }
}

#[test]
fn clearing_or_changing_an_anchor_without_a_ruling_is_a_protected_exit() {
    let anchor_it = "  - {op: set-rule-field, schema: command/demo, id: demo.plain, field: anchor, value: \"2026-09-03 demo D1\"}\n";
    for follow in [
        "  - {op: set-rule-field, schema: command/demo, id: demo.plain, field: anchor, value: ~}\n",
        "  - {op: set-rule-field, schema: command/demo, id: demo.plain, field: anchor, value: \"2026-09-04 other D9\"}\n",
    ] {
        let replay = with_followup(
            "anchor-edit",
            &followup(
                "Anchor a rule, then edit that anchor away.",
                &format!("{anchor_it}{follow}"),
            ),
        );
        assert!(
            codes(&replay).contains(&"protected-exit"),
            "editing an anchor away must be a protected exit, got {:?}",
            codes(&replay)
        );
    }
}

#[test]
fn a_ruling_anchor_on_the_migration_authorises_lowering_protection() {
    let replay = with_followup(
        "authorised-downgrade",
        &anchored_followup(
            "Downgrade a floor rule under a recorded ruling, then retire it.",
            "  - {op: set-rule-field, schema: command/demo, id: demo.floor-rule, field: class, value: advisory}\n  - {op: tombstone-rule, schema: command/demo, id: demo.floor-rule, disposition: superseded}\n",
        ),
    );
    assert_clean(&replay);
    assert!(demo(&replay.state).is_tombstoned("demo.floor-rule"));
}

#[test]
fn a_malformed_migration_anchor_does_not_authorise_lowering_protection() {
    let replay = with_followup(
        "bad-migration-anchor",
        &followup(
            "Downgrade under a malformed ruling anchor.",
            "  - {op: set-rule-field, schema: command/demo, id: demo.floor-rule, field: class, value: advisory}\n",
        )
        .replace("intent:", "anchor: not-an-anchor\nintent:"),
    );
    assert!(
        codes(&replay).contains(&"protected-exit") || codes(&replay).contains(&"anchor-format"),
        "a malformed header anchor authorises nothing, got {:?}",
        codes(&replay)
    );
}

#[test]
fn raising_protection_never_needs_a_ruling() {
    let replay = with_followup(
        "raise-protection",
        &followup(
            "Promote an ordinary rule to a floor and give it an anchor.",
            "  - {op: set-rule-field, schema: command/demo, id: demo.plain, field: class, value: floor}\n  - {op: set-rule-field, schema: command/demo, id: demo.lead, field: anchor, value: \"2026-09-03 demo D1\"}\n",
        ),
    );
    assert_clean(&replay);
    assert!(demo(&replay.state)
        .find_rule("demo.plain")
        .unwrap()
        .is_floor());
}

// ---------------------------------------------------------------------------
// Fix round 1 — B3: load runs the hard set
// ---------------------------------------------------------------------------

#[test]
fn load_refuses_a_log_that_replays_cleanly_but_fails_the_hard_set() {
    let dir = log_dir("hard-set");
    write(&dir, "0001-genesis.yaml", GENESIS);
    // Mint a section outside the canonical six: every op applies, and the hard set rejects.
    write(
        &dir,
        "0002-extra.yaml",
        &followup(
            "Mint a section that is not one of the canonical six.",
            "  - {op: mint-section, schema: command/demo, section: {id: demo.sec.invented, title: T, intent: I, note: Empty on purpose.}}\n",
        ),
    );
    let replay = replay_of(&dir);
    assert!(
        replay.findings.is_empty(),
        "every op applies — the log itself is sound"
    );
    assert!(
        !replay.is_deliverable(),
        "but the state fails the hard set, so it is not deliverable"
    );
    let findings = replay::load(&dir).expect_err("load refuses a state that fails the hard set");
    assert!(
        findings.iter().any(|f| f.code.as_str() == "section-set"),
        "load returns the hard-set findings, got {:?}",
        findings.iter().map(|f| f.to_string()).collect::<Vec<_>>()
    );
}

#[test]
fn load_accepts_a_log_that_is_sound_and_hard_set_clean() {
    let dir = log_dir("sound");
    write(&dir, "0001-genesis.yaml", GENESIS);
    replay::load(&dir).expect("the genesis fixture is a valid corpus");
}

// ---------------------------------------------------------------------------
// Fix round 1 — D-1: the grammar the log was written in
// ---------------------------------------------------------------------------

#[test]
fn the_replay_reports_the_grammar_it_applied() {
    let dir = log_dir("grammar");
    write(&dir, "0001-genesis.yaml", GENESIS);
    assert_eq!(replay_of(&dir).grammar(), Some(1));
    let full = replay::load_full(&dir).expect("a sound log loads whole");
    assert_eq!(full.grammar(), Some(1));
    assert_eq!(
        full.state.content_hash(),
        replay_of(&dir).state.content_hash()
    );
}

#[test]
fn an_empty_log_reports_no_grammar() {
    let dir = log_dir("empty");
    assert_eq!(replay_of(&dir).grammar(), None);
}

// ---------------------------------------------------------------------------
// Fix round 1 — A1, A5, A10
// ---------------------------------------------------------------------------

#[test]
fn a_yaml_file_that_is_not_a_migration_is_reported_never_skipped() {
    let dir = log_dir("misnamed");
    write(&dir, "0001-genesis.yaml", GENESIS);
    // The letter O typed for a zero: a real migration that would replay as if absent.
    std::fs::write(
        dir.join("O002-typo.yaml"),
        followup("A typo in the sequence prefix.", "  - {}\n"),
    )
    .expect("fixture is writable");
    let replay = replay_of(&dir);
    assert!(
        codes(&replay).contains(&"log-file-name"),
        "a misnamed .yaml in the log is a finding, got {:?}",
        codes(&replay)
    );
}

#[test]
fn mint_section_carrying_inline_rules_is_rejected() {
    let replay = with_followup(
        "mint-section-rules",
        &followup(
            "Mint a section with a rule already inside it.",
            "  - {op: mint-section, schema: command/demo, section: {id: demo.sec.tools, title: T, intent: I, rules: [{id: demo.smuggled, class: floor, text: T}]}}\n",
        ),
    );
    assert!(
        codes(&replay).contains(&"op-inapplicable"),
        "an inline rule must be rejected, never silently dropped, got {:?}",
        codes(&replay)
    );
    assert!(demo(&replay.state).find_rule("demo.smuggled").is_none());
}

#[test]
fn a_non_string_when_key_or_label_is_a_finding_never_a_silent_coercion() {
    let bad_when = with_followup(
        "when-key",
        &followup(
            "Write a `when:` whose dimension key is not a name.",
            "  - {op: set-rule-field, schema: command/demo, id: demo.plain, field: when, value: {1: multi}}\n",
        ),
    );
    assert!(
        codes(&bad_when).contains(&"op-inapplicable"),
        "a non-string dimension key is a finding, got {:?}",
        codes(&bad_when)
    );

    let bad_labels = with_followup(
        "label-item",
        &followup(
            "Write a labels list carrying a non-string item.",
            "  - {op: set-rule-field, schema: command/demo, id: demo.plain, field: labels, value: [seats, [nested]]}\n",
        ),
    );
    assert!(
        codes(&bad_labels).contains(&"op-inapplicable"),
        "a non-string label is a finding, got {:?}",
        codes(&bad_labels)
    );
}

// ---------------------------------------------------------------------------
// Wave 4 — `reword-section`
// ---------------------------------------------------------------------------

/// The section a reword names, from a replayed state.
fn section<'a>(state: &'a replay::State, id: &str) -> &'a mochiko_cli::model::Section {
    demo(state)
        .find_section(id)
        .unwrap_or_else(|| panic!("{id} is a live section of the demo schema"))
}

#[test]
fn reword_section_sets_the_title_the_intent_and_the_note() {
    let replay = with_followup(
        "reword-section-all",
        &followup(
            "Reword one section's prose.",
            "  - {op: reword-section, schema: command/demo, id: demo.sec.reserved, \
             title: Reserved to you, intent: The calls that stay the user's., \
             note: Deliberately empty by ruling.}\n",
        ),
    );
    assert_clean(&replay);
    let reserved = section(&replay.state, "demo.sec.reserved");
    assert_eq!(reserved.title, "Reserved to you");
    assert_eq!(reserved.intent, "The calls that stay the user's.");
    assert_eq!(
        reserved.note.as_deref(),
        Some("Deliberately empty by ruling.")
    );
}

#[test]
fn reword_section_edits_only_the_fields_it_names() {
    let replay = with_followup(
        "reword-section-one",
        &followup(
            "Reword one section's intent alone.",
            "  - {op: reword-section, schema: command/demo, id: demo.sec.reserved, \
             intent: Only the intent moves.}\n",
        ),
    );
    assert_clean(&replay);
    let reserved = section(&replay.state, "demo.sec.reserved");
    assert_eq!(reserved.intent, "Only the intent moves.");
    assert_eq!(reserved.title, "Reserved", "the title was not named");
    assert_eq!(
        reserved.note.as_deref(),
        Some("Deliberately empty — this run reserves nothing beyond its floor."),
        "the note was not named"
    );
}

#[test]
fn reword_section_clears_a_note() {
    let replay = with_followup(
        "reword-section-clear",
        &followup(
            "Clear a section's note.",
            "  - {op: reword-section, schema: command/demo, id: demo.sec.reserved, note: ~}\n",
        ),
    );
    assert_clean(&replay);
    assert_eq!(section(&replay.state, "demo.sec.reserved").note, None);
}

#[test]
fn reword_section_leaves_the_sections_rules_untouched() {
    let before = replay_of(&{
        let dir = log_dir("reword-section-rules-before");
        write(&dir, "0001-genesis.yaml", GENESIS);
        dir
    });
    let after = with_followup(
        "reword-section-rules",
        &followup(
            "Reword the fail set's intent.",
            "  - {op: reword-section, schema: command/demo, id: demo.sec.fail-conditions, \
             intent: The fail set, counted by the render.}\n",
        ),
    );
    assert_clean(&after);

    let ids = |replay: &Replay| -> Vec<String> {
        demo(&replay.state)
            .find_section("demo.sec.fail-conditions")
            .expect("the fail section is live")
            .rules
            .iter()
            .map(|r| r.id.clone())
            .collect()
    };
    assert_eq!(ids(&before), ids(&after), "a reword moves no rule");

    let rule = demo(&after.state)
        .find_rule("demo.fail.ungraded")
        .expect("the fail rule is still live");
    assert_eq!(rule.text.as_deref(), Some("Never graded."));
    assert_eq!(rule.class.as_deref(), Some("floor"));
    assert_eq!(rule.kind.as_deref(), Some("fail"));
}

#[test]
fn reword_section_needs_no_ruling_anchor_even_over_a_fail_set() {
    // Protection is read from a rule's own fields — floor, fail, anchored — and a section reword
    // touches no rule, so no ruling anchor is owed for one.
    let replay = with_followup(
        "reword-section-unanchored",
        &followup(
            "Reword a fail section with no header anchor.",
            "  - {op: reword-section, schema: command/demo, id: demo.sec.fail-conditions, \
             intent: Reworded with no anchor at all.}\n",
        ),
    );
    assert_clean(&replay);
    assert!(
        !codes(&replay).contains(&"protected-exit"),
        "a section reword is not a protected exit, got {:?}",
        codes(&replay)
    );
    assert_eq!(
        section(&replay.state, "demo.sec.fail-conditions").intent,
        "Reworded with no anchor at all."
    );
}

#[test]
fn reword_section_on_an_unknown_section_is_inapplicable() {
    let replay = with_followup(
        "reword-section-unknown",
        &followup(
            "Reword a section that does not exist.",
            "  - {op: reword-section, schema: command/demo, id: demo.sec.nowhere, \
             intent: Nothing to reword.}\n",
        ),
    );
    assert!(
        codes(&replay).contains(&"op-inapplicable"),
        "an unknown section id is a finding, got {:?}",
        codes(&replay)
    );
}

#[test]
fn reword_section_on_a_tombstoned_section_names_the_tombstone() {
    let replay = with_followup(
        "reword-section-tombstoned",
        &followup(
            "Retire a section, then try to reword it.",
            "  - {op: tombstone-section, schema: command/demo, id: demo.sec.tools, \
             disposition: superseded}\n  \
             - {op: reword-section, schema: command/demo, id: demo.sec.tools, \
             intent: Too late.}\n",
        ),
    );
    let message = replay
        .findings
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        codes(&replay).contains(&"op-inapplicable"),
        "rewording a retired section is a finding, got {:?}",
        codes(&replay)
    );
    assert!(
        message.contains("retired"),
        "the finding says the section is retired rather than merely absent: {message}"
    );
}

#[test]
fn reword_section_on_a_document_carrying_no_rules_is_inapplicable() {
    let replay = with_followup(
        "reword-section-opaque",
        &followup(
            "Reword a section of a template, which has none.",
            "  - {op: reword-section, schema: template/spec, id: spec.sec.tools, \
             intent: Templates carry no sections.}\n",
        ),
    );
    assert!(
        codes(&replay).contains(&"op-inapplicable"),
        "a rules-less document is a finding, got {:?}",
        codes(&replay)
    );
}

#[test]
fn a_two_migration_log_replays_deterministically() {
    let build = |tag: &str, first: &str, second: &str| -> String {
        let dir = log_dir(tag);
        write(&dir, "0001-genesis.yaml", GENESIS);
        write(&dir, "0002-a.yaml", first);
        write(&dir, "0003-b.yaml", second);
        let replay = replay_of(&dir);
        assert_clean(&replay);
        replay.state.content_hash()
    };
    let a = "grammar: 1\nid: 0002-a\nsequence: 2\nintent: First reword.\nchanges:\n  \
             - {op: reword-section, schema: command/demo, id: demo.sec.reserved, intent: First.}\n";
    let b = "grammar: 1\nid: 0003-b\nsequence: 3\nintent: Second reword.\nchanges:\n  \
             - {op: reword-section, schema: command/demo, id: demo.sec.reserved, intent: Second.}\n";

    let once = build("determinism-1", a, b);
    let twice = build("determinism-2", a, b);
    assert_eq!(once, twice, "the same log replays to the same state");

    // The intent is content, so the last reword in sequence order decides the hash.
    let swapped = build(
        "determinism-3",
        &a.replace("intent: First.}", "intent: Second.}"),
        &b.replace("intent: Second.}", "intent: First.}"),
    );
    assert_ne!(
        once, swapped,
        "a section intent is state — reversing the rewords must move the content hash"
    );
}
