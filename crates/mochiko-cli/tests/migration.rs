//! Integration tests for the canonical encoder and the migration-file grammar.
//!
//! The canonical encoder is tested first and hardest: the migration body hash, the replayed
//! state hash, and (from wave 1's P3 seat) derived-view equality all rest on it, so a
//! non-injective or order-sensitive encoding would silently weaken three separate guarantees.

use mochiko_cli::migration::{self, ChangeOp, ParseError};
use mochiko_cli::model::canonical_hash;

fn value(yaml: &str) -> serde_norway::Value {
    serde_norway::from_str(yaml).expect("test fixture parses")
}

// --- canonical encoding ---

#[test]
fn canonical_hash_is_stable_across_mapping_key_order() {
    let a = value("b: 2\na: 1\nc: {y: 2, x: 1}\n");
    let b = value("a: 1\nc: {x: 1, y: 2}\nb: 2\n");
    assert_eq!(
        canonical_hash(&a),
        canonical_hash(&b),
        "mapping key order must not change the canonical hash"
    );
}

#[test]
fn canonical_hash_is_sensitive_to_sequence_order() {
    // Sequences are ordered data (a section's rule list), so their order MUST be covered.
    let a = value("- 1\n- 2\n");
    let b = value("- 2\n- 1\n");
    assert_ne!(canonical_hash(&a), canonical_hash(&b));
}

#[test]
fn canonical_hash_separates_values_a_naive_concatenation_would_collide() {
    // The classic injectivity trap: without length prefixes, {ab: c} and {a: bc} encode alike.
    assert_ne!(
        canonical_hash(&value("ab: c")),
        canonical_hash(&value("a: bc"))
    );
    // A scalar and a one-element sequence are different YAML, so different hashes.
    assert_ne!(
        canonical_hash(&value("k: yes")),
        canonical_hash(&value("k: [yes]"))
    );
    // Null, the empty string, and the string "null" are three distinct values.
    assert_ne!(
        canonical_hash(&value("k: ~")),
        canonical_hash(&value("k: ''"))
    );
    assert_ne!(
        canonical_hash(&value("k: ~")),
        canonical_hash(&value("k: 'null'"))
    );
    // A number and its string spelling never collide.
    assert_ne!(
        canonical_hash(&value("k: 1")),
        canonical_hash(&value("k: '1'"))
    );
    assert_ne!(
        canonical_hash(&value("k: true")),
        canonical_hash(&value("k: 'true'"))
    );
}

#[test]
fn canonical_hash_is_prefixed_and_hex() {
    let h = canonical_hash(&value("a: 1"));
    let hex = h
        .strip_prefix("sha256:")
        .expect("hash carries its algorithm prefix");
    assert_eq!(hex.len(), 64, "sha-256 renders as 64 hex characters");
    assert!(hex
        .chars()
        .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
}

// --- migration header ---

const HEADER: &str = r#"
grammar: 1
id: 0002-demo
sequence: 2
intent: A one-line statement of what this migration does.
changes:
  - op: reword-rule
    schema: command/specify
    id: spec.register
    text: New text.
"#;

/// A fixture body stamped with its correct hash. The `hash:` header is required, so every
/// fixture meant to parse goes through here.
fn stamped(body: &str) -> String {
    migration::with_hash("0002-demo.yaml", body).expect("the fixture body can be stamped")
}

/// Parse a fixture body, stamping it first.
fn parse_stamped(body: &str) -> migration::Migration {
    let body = stamped(body);
    migration::parse("0002-demo.yaml", &body).expect("a stamped fixture parses")
}

#[test]
fn a_well_formed_migration_parses_its_header() {
    let m = parse_stamped(HEADER);
    assert_eq!(m.grammar, 1);
    assert_eq!(m.id, "0002-demo");
    assert_eq!(m.sequence, 2);
    assert!(m.intent.starts_with("A one-line statement"));
    assert_eq!(m.anchor, None);
    assert_eq!(m.changes.len(), 1);
}

#[test]
fn a_missing_header_field_is_rejected() {
    for field in ["grammar", "id", "sequence", "intent", "changes"] {
        // Dropping `changes:` must drop the list it owns too, or the fixture is broken YAML
        // rather than a migration missing a header field.
        let mut body = String::new();
        let mut skipping = false;
        for line in HEADER.lines() {
            if line.starts_with(&format!("{field}:")) {
                skipping = true;
                continue;
            }
            if skipping && (line.starts_with(' ') || line.starts_with('-') || line.is_empty()) {
                continue;
            }
            skipping = false;
            body.push_str(line);
            body.push('\n');
        }
        let err = migration::parse("0002-demo.yaml", &body)
            .expect_err(&format!("a migration missing `{field}:` must be rejected"));
        assert!(
            matches!(err, ParseError::Header { .. }),
            "want a header finding for the missing `{field}:`, got {err}"
        );
    }
}

#[test]
fn an_unparseable_file_is_rejected_as_a_grammar_parse_finding() {
    let err = migration::parse("0002-demo.yaml", "grammar: 1\n  bad: [indent\n")
        .expect_err("broken YAML is rejected");
    assert!(matches!(err, ParseError::Yaml { .. }), "got {err}");
}

#[test]
fn an_unknown_op_is_rejected_and_names_the_op() {
    let body = HEADER.replace("op: reword-rule", "op: frobnicate-rule");
    let err = migration::parse("0002-demo.yaml", &body).expect_err("an unknown op is rejected");
    assert!(
        format!("{err}").contains("frobnicate-rule"),
        "the finding must name the unknown op, got {err}"
    );
}

#[test]
fn the_filename_sequence_must_agree_with_the_header() {
    let err = migration::parse("0007-demo.yaml", HEADER)
        .expect_err("a filename prefix disagreeing with `sequence:` is rejected");
    assert!(
        matches!(err, ParseError::SequenceMismatch { .. }),
        "got {err}"
    );
}

// --- body hash (lead ruling Q1: the hash covers id + sequence + anchor + changes) ---

#[test]
fn a_matching_body_hash_is_accepted_and_a_mismatch_is_rejected() {
    let hashed = stamped(HEADER);
    migration::parse("0002-demo.yaml", &hashed).expect("a correct hash is accepted");

    let broken = hashed.replace("text: New text.", "text: Tampered text.");
    let err = migration::parse("0002-demo.yaml", &broken)
        .expect_err("a changed body with the old hash is rejected");
    assert!(matches!(err, ParseError::HashMismatch { .. }), "got {err}");
}

#[test]
fn the_body_hash_covers_the_anchor_and_the_sequence_but_not_the_intent() {
    // Q1 ruling: `{id, sequence, anchor, changes}` are hashed; `intent` and `hash` are not.
    // The anchor is the D2 protected-content evidence, so it must not be editable after the fact.
    let base = parse_stamped(HEADER).body_hash();

    let reworded_intent = HEADER.replace(
        "intent: A one-line statement of what this migration does.",
        "intent: Something else entirely.",
    );
    assert_eq!(
        parse_stamped(&reworded_intent).body_hash(),
        base,
        "`intent:` is prose and is deliberately outside the hash"
    );

    let anchored = HEADER.replace(
        "intent:",
        "anchor: \"2026-09-03 cli-schema-delivery [D2]\"\nintent:",
    );
    assert_ne!(
        parse_stamped(&anchored).body_hash(),
        base,
        "adding an anchor must move the hash"
    );

    let resequenced = HEADER.replace("sequence: 2", "sequence: 9");
    assert_ne!(
        migration::parse(
            "0009-demo.yaml",
            &migration::with_hash("0009-demo.yaml", &resequenced).unwrap()
        )
        .unwrap()
        .body_hash(),
        base,
        "the sequence is part of the migration's identity"
    );
}

#[test]
fn the_body_hash_ignores_key_order_inside_a_change() {
    let reordered = HEADER.replace(
        "    schema: command/specify\n    id: spec.register\n    text: New text.",
        "    text: New text.\n    id: spec.register\n    schema: command/specify",
    );
    assert_eq!(
        parse_stamped(&reordered).body_hash(),
        parse_stamped(HEADER).body_hash(),
        "the hash is over canonical content, never over the file's key order"
    );
}

// --- the version contract (D5) ---

#[test]
fn a_grammar_outside_the_supported_range_is_rejected_with_the_install_line() {
    let body = HEADER.replace("grammar: 1", "grammar: 99");
    let err = migration::parse("0002-demo.yaml", &body).expect_err("grammar 99 is out of range");
    assert!(
        matches!(err, ParseError::GrammarVersion { .. }),
        "got {err}"
    );
    let message = format!("{err}");
    assert!(
        message.contains("99"),
        "the finding names the log's grammar: {message}"
    );
    assert!(
        message.contains(migration::INSTALL_COMMAND),
        "the D5 message names the install command: {message}"
    );
}

#[test]
fn the_supported_grammar_range_is_one_to_one() {
    assert_eq!(migration::GRAMMAR_RANGE, (1, 1));
}

// --- every op decodes ---

#[test]
fn every_change_op_decodes_from_its_yaml_form() {
    let body = r#"
grammar: 1
id: 0003-ops
sequence: 3
intent: One of every op, so the grammar's surface is covered by a decode assertion.
changes:
  - {op: import-document, kind: command, name: specify, content: {kind: command}}
  - {op: replace-document, kind: template, name: spec, content: {template: spec}}
  - {op: mint-section, schema: command/specify, section: {id: spec.sec.tools, title: T, intent: I}}
  - {op: tombstone-section, schema: command/specify, id: spec.sec.tools, disposition: superseded}
  - {op: mint-rule, schema: command/specify, section: spec.sec.tools, rule: {id: spec.a, class: must, text: T}}
  - {op: reword-rule, schema: command/specify, id: spec.a, text: New.}
  - {op: set-rule-field, schema: command/specify, id: spec.a, field: pointer, value: mochiko:x}
  - {op: move-rule, schema: command/specify, id: spec.a, section: spec.sec.roles}
  - {op: tombstone-rule, schema: command/specify, id: spec.a, disposition: dropped}
  - {op: supersede-rule, schema: command/specify, id: spec.b, disposition: superseded, anchor: "2026-09-03 s [D1]"}
  - {op: set-var, schema: command/specify, name: pm_seat, value: product-manager}
  - {op: set-condition, schema: command/specify, name: seats, spec: {values: [single, multi], resolution: standing-trigger}}
  - {op: set-moment, schema: command/specify, name: intent, text: The adaptive-probe stage.}
  - {op: registry-add, registry: command-labels, label: seats, meaning: Seat wiring.}
  - {op: registry-retire, registry: command-labels, label: seats, note: Retired by ruling.}
"#;
    let body = migration::with_hash("0003-ops.yaml", body).expect("the fixture stamps");
    let m = migration::parse("0003-ops.yaml", &body).expect("every op decodes");
    let ops: Vec<ChangeOp> = m.changes.iter().map(|c| c.op()).collect();
    assert_eq!(ops.len(), 15);
    for op in ChangeOp::ALL {
        assert!(
            ops.contains(&op),
            "op {op:?} has no decode assertion in this fixture"
        );
    }
}

#[test]
fn set_rule_field_accepts_a_null_value_as_an_explicit_clear() {
    let body = HEADER.replace(
        "  - op: reword-rule\n    schema: command/specify\n    id: spec.register\n    text: New text.",
        "  - {op: set-rule-field, schema: command/specify, id: spec.a, field: pointer, value: ~}",
    );
    let m = parse_stamped(&body);
    assert_eq!(m.changes.len(), 1);
}

// ---------------------------------------------------------------------------
// Fix round 1 — B2: the body hash is required
// ---------------------------------------------------------------------------

#[test]
fn a_migration_with_no_hash_is_rejected() {
    // Three spellings of "no hash", each of which would otherwise skip the check entirely —
    // and skipping it would leave the anchor the hash exists to protect editable by deleting
    // one line.
    for body in [
        HEADER.to_string(),
        HEADER.replace("intent:", "hash: \"\"\nintent:"),
        HEADER.replace("intent:", "hash: ~\nintent:"),
    ] {
        let err = migration::parse("0002-demo.yaml", &body)
            .expect_err("a migration with no body hash is rejected");
        assert!(
            matches!(err, ParseError::Header { .. }),
            "want a header finding for the missing hash, got {err}"
        );
        assert!(
            format!("{err}").contains("hash"),
            "the finding names the missing field: {err}"
        );
    }
}

#[test]
fn with_hash_stamps_a_body_that_then_parses() {
    let stamped = migration::with_hash("0002-demo.yaml", HEADER).expect("the body can be stamped");
    let parsed = migration::parse("0002-demo.yaml", &stamped).expect("a stamped body parses");
    assert_eq!(parsed.body_hash(), migration::compute_hash(&parsed));
    assert_eq!(parsed.sequence, 2);
    assert_eq!(parsed.changes.len(), 1);
}

#[test]
fn with_hash_replaces_an_existing_hash_rather_than_duplicating_it() {
    let stamped = migration::with_hash("0002-demo.yaml", HEADER).unwrap();
    let tampered = stamped.replace("text: New text.", "text: Different text.");
    // The tampered body no longer matches its stamp...
    assert!(migration::parse("0002-demo.yaml", &tampered).is_err());
    // ...and re-stamping it makes it valid again, with one hash key, not two.
    let restamped = migration::with_hash("0002-demo.yaml", &tampered).expect("re-stamps");
    migration::parse("0002-demo.yaml", &restamped).expect("the re-stamped body parses");
    assert_eq!(
        restamped.matches("hash:").count(),
        1,
        "re-stamping replaces the hash rather than appending a second one"
    );
}

#[test]
fn with_hash_refuses_a_body_that_is_not_a_migration() {
    assert!(migration::with_hash("0002-demo.yaml", "grammar: 1\n").is_err());
}

// ---------------------------------------------------------------------------
// Fix round 1 — A4: any out-of-range grammar reaches the D5 message
// ---------------------------------------------------------------------------

#[test]
fn every_out_of_range_grammar_raises_the_version_contract_finding() {
    for grammar in ["2", "99", "999999", "4294967295"] {
        let body = HEADER.replace("grammar: 1", &format!("grammar: {grammar}"));
        let err = migration::parse("0002-demo.yaml", &body).expect_err("out of range");
        assert!(
            matches!(err, ParseError::GrammarVersion { .. }),
            "grammar {grammar} must raise the version contract, got {err}"
        );
        assert!(format!("{err}").contains(migration::INSTALL_COMMAND));
    }
}

#[test]
fn a_grammar_that_is_not_a_whole_number_is_a_header_finding() {
    for grammar in ["one", "-1", "1.5"] {
        let body = HEADER.replace("grammar: 1", &format!("grammar: {grammar}"));
        let err = migration::parse("0002-demo.yaml", &body).expect_err("not a whole number");
        assert!(
            matches!(err, ParseError::Header { .. }),
            "grammar {grammar} is malformed, not out of range, got {err}"
        );
    }
}

// ---------------------------------------------------------------------------
// Fix round 1 — A9: a malformed change is not an unknown op
// ---------------------------------------------------------------------------

#[test]
fn a_known_op_missing_a_field_is_malformed_not_unknown() {
    let body = HEADER.replace(
        "  - op: reword-rule\n    schema: command/specify\n    id: spec.register\n    text: New text.",
        "  - {op: import-document, kind: command, name: specify}",
    );
    let err = migration::parse("0002-demo.yaml", &body).expect_err("content: is missing");
    assert_eq!(
        err.code(),
        "op-malformed",
        "a well-known op missing a field must not be filed under the unknown-op code: {err}"
    );

    let unknown = HEADER.replace("op: reword-rule", "op: frobnicate-rule");
    let err = migration::parse("0002-demo.yaml", &unknown).expect_err("unknown op");
    assert_eq!(err.code(), "op-unknown");
}

// ---------------------------------------------------------------------------
// Fix round 1 — A3: the anchor grammar is anchored at both ends
// ---------------------------------------------------------------------------

#[test]
fn the_anchor_grammar_accepts_what_the_corpus_writes_and_nothing_looser() {
    use mochiko_cli::model::is_anchor;
    // The 597 sidecar anchors are written `YYYY-MM-DD <slug>` with an optional bare `D<n>`
    // tail; the wave plan writes the tail bracketed. Both are accepted, nothing else is.
    for good in [
        "2026-08-02 ux-mocking-in-specify",
        "2026-08-02 ux-mocking-in-specify D9",
        "2026-09-03 cli-schema-delivery [D2]",
        "2026-12-31 year-end D10",
    ] {
        assert!(is_anchor(good), "{good:?} is a well-formed anchor");
    }
    for bad in [
        "9999-99-99 out-of-range",
        "2026-09-03 slug and a whole sentence of junk here",
        "2026-09-03 slug [NOT-A-DECISION]",
        "2026-9-3 short-fields",
        "2026-09-03",
        "not-a-date slug",
        "2026-13-01 bad-month",
        "2026-09-32 bad-day",
        "2026-00-10 zero-month",
        " 2026-09-03 leading-space-slug D1 trailing junk",
    ] {
        assert!(!is_anchor(bad), "{bad:?} must not pass as an anchor");
    }
}

// ---------------------------------------------------------------------------
// Fix round 1 — A8: the canonical encoder is depth-bounded
// ---------------------------------------------------------------------------

#[test]
fn the_canonical_encoder_is_bounded_rather_than_recursing_off_the_stack() {
    use mochiko_cli::model::{canonical_depth, canonical_hash, MAX_CANONICAL_DEPTH};

    // A document deeper than the bound must not abort the process; it hashes to a value and
    // reports its depth so the validator can raise a finding. Built rather than parsed, because
    // a YAML file nested this deep is what a hostile input looks like, not what a text fixture is.
    let mut value = serde_norway::Value::String("leaf".to_string());
    for _ in 0..(MAX_CANONICAL_DEPTH + 40) {
        let mut level = serde_norway::Mapping::new();
        level.insert(serde_norway::Value::String("a".to_string()), value);
        value = serde_norway::Value::Mapping(level);
    }
    assert!(canonical_depth(&value) > MAX_CANONICAL_DEPTH);
    let hash = canonical_hash(&value);
    assert!(hash.starts_with("sha256:"));

    let shallow = value_of("a: {b: {c: 1}}");
    assert_eq!(canonical_depth(&shallow), 3);
    assert!(canonical_depth(&value_of("a: 1")) <= MAX_CANONICAL_DEPTH);
}

fn value_of(yaml: &str) -> serde_norway::Value {
    serde_norway::from_str(yaml).expect("fixture parses")
}
