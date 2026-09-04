//! Genesis fidelity — every rule of the 50 shipped schema files survives the log.
//!
//! The comparison is deliberately **independent of the generator**: the expected side is the
//! shipped YAML decoded straight into the model, and the actual side is the document the replay
//! built from the whole log under `plugins/mochiko/migrations/`. Only two deltas are allowed, and
//! each is asserted rather than excused — the two comment-carried `enforces: []` reasons, which
//! the model holds as `note:` data, and the provenance anchors, which are checked against the
//! sidecar the test reads for itself.
//!
//! One failure per divergence, all of them named in one run: a broken genesis should report its
//! whole blast radius, not the first field it happens to reach.

use mochiko_cli::genesis;
use mochiko_cli::migration;
use mochiko_cli::model::{DocKind, DocRef, Document, Rule, RuleSchema};
use mochiko_cli::replay;
use serde_norway::Value;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the crate sits two levels under the repository root")
        .to_path_buf()
}

fn log_dir() -> PathBuf {
    repo_root().join("plugins/mochiko/migrations")
}

/// The v0.103.0 schema corpus, frozen under `tests/fixtures/` on 2026-09-04.
///
/// Genesis imported the corpus as it stood at sequence 1. From the moment a second migration
/// carries the live corpus forward — wave 4's `0002-fail-conditions-intent.yaml` is the first —
/// a rebuild from the live tree can no longer reproduce the committed genesis, so the byte
/// comparison below would grade drift the log has already accounted for. The frozen copy is the
/// honest input, and record D8 asks for exactly this: the round trip proven against a fixture
/// kept after the YAML sources retire at wave 6.
fn frozen_corpus() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/genesis-corpus")
}

fn genesis_path() -> PathBuf {
    log_dir().join(genesis::FILE)
}

/// The two rules whose empty `enforces:` carries its reason in a YAML comment, and the text that
/// comment must become. Verbatim from `plugins/mochiko/schemas/setup.yaml`.
const CARRIED_REASONS: [(&str, &str); 2] = [
    (
        "setup.fail.floor-category-uncovered",
        "the Essential Floor category set is owned by mochiko:authoring-constitution, carried on \
         this node's own pointer: — no local rule enumerates it.",
    ),
    (
        "setup.fail.unclosed-trace",
        "the intent→surface trace obligation is owned by mochiko:authoring-constitution, bound at \
         setup.surface-set — no local rule states it.",
    ),
];

/// The sidecar's anchors, read by this test rather than through the generator.
fn sidecar() -> BTreeMap<String, String> {
    let path = repo_root().join(genesis::SIDECAR);
    let text = std::fs::read_to_string(&path).expect("the provenance sidecar is readable");
    let value: Value = serde_norway::from_str(&text).expect("the sidecar parses");
    let Some(Value::Mapping(anchors)) = value.get("anchors") else {
        panic!("the sidecar carries an `anchors:` mapping");
    };
    anchors
        .iter()
        .filter_map(|(k, v)| Some((k.as_str()?.to_string(), v.as_str()?.to_string())))
        .collect()
}

/// The shipped corpus decoded straight from disk, with no note or anchor applied.
fn shipped_documents() -> Vec<(DocRef, Document)> {
    genesis::scan(&repo_root())
        .expect("the corpus scans")
        .into_iter()
        .map(|file| {
            let document = Document::from_value(file.doc.kind, &file.value)
                .unwrap_or_else(|e| panic!("{} decodes: {e}", file.path.display()));
            (file.doc, document)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// the committed file
// ---------------------------------------------------------------------------

#[test]
fn the_committed_genesis_regenerates_byte_identically() {
    // Built from the frozen v0.103.0 corpus (record D8; frozen 2026-09-04), never the live tree.
    let generated = genesis::build(&frozen_corpus())
        .unwrap_or_else(|errors| panic!("genesis builds:\n{}", genesis::render_errors(&errors)));
    let committed = std::fs::read_to_string(genesis_path())
        .expect("plugins/mochiko/migrations/0001-genesis.yaml is committed");

    if generated == committed {
        return;
    }
    let mut line = 0usize;
    for (a, b) in generated.lines().zip(committed.lines()) {
        line += 1;
        assert_eq!(
            a, b,
            "the committed genesis differs from a fresh build at line {line}\n\
             The build reads the FROZEN corpus, never the live tree, so regenerate with\n  \
             `cargo run -- genesis emit \
             --root crates/mochiko-cli/tests/fixtures/genesis-corpus \
             --out plugins/mochiko/migrations/0001-genesis.yaml`\n\
             The committed genesis changes only when that fixture changes. A live-tree build \
             would rewrite it from a corpus later migrations have already carried forward, \
             folding their content back into sequence 1 and losing the history."
        );
    }
    panic!(
        "the committed genesis is {} lines, a fresh build is {}",
        committed.lines().count(),
        generated.lines().count()
    );
}

#[test]
fn the_committed_genesis_is_a_valid_migration_carrying_one_op_per_document() {
    let source = std::fs::read_to_string(genesis_path()).expect("the genesis file is readable");
    let parsed = migration::parse(genesis::FILE, &source).expect("the genesis file parses");

    assert_eq!(parsed.id, genesis::ID);
    assert_eq!(parsed.sequence, genesis::SEQUENCE);
    assert_eq!(parsed.grammar, 1);
    assert_eq!(parsed.anchor.as_deref(), Some(genesis::ANCHOR));
    assert_eq!(parsed.changes.len(), 50, "one op per shipped document");
    assert!(
        parsed
            .changes
            .iter()
            .all(|c| c.op() == migration::ChangeOp::ImportDocument),
        "genesis imports; it changes nothing"
    );
}

#[test]
fn the_log_replays_into_a_deliverable_state() {
    let replay = replay::load_full(&log_dir()).unwrap_or_else(|findings| {
        let lines: Vec<String> = findings
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        panic!("the log is deliverable:\n{}", lines.join("\n"));
    });
    assert_eq!(replay.state.docs.len(), 50);
    assert_eq!(
        replay.sequences(),
        vec![1, 2],
        "genesis plus wave 4's fail-conditions reword"
    );
}

// ---------------------------------------------------------------------------
// field-by-field fidelity
// ---------------------------------------------------------------------------

#[test]
fn every_shipped_document_survives_the_log_field_by_field() {
    let state = replay::load(&log_dir()).expect("the log is deliverable");
    let anchors = sidecar();
    let reasons: BTreeMap<&str, &str> = CARRIED_REASONS.into_iter().collect();

    let mut divergences: Vec<String> = Vec::new();
    for (doc, expected) in shipped_documents() {
        let Some(actual) = state.docs.get(&doc) else {
            divergences.push(format!("{doc}: absent from the replayed state"));
            continue;
        };
        match (&expected, actual) {
            (Document::Rules(want), Document::Rules(got)) => {
                compare_schema(&doc, want, got, &anchors, &reasons, &mut divergences);
            }
            (Document::Labels(want), Document::Labels(got)) => {
                if want.declared_kind != got.declared_kind {
                    divergences.push(format!("{doc}: kind"));
                }
                if want.labels != got.labels {
                    divergences.push(format!("{doc}: labels"));
                }
                if want.retired != got.retired {
                    divergences.push(format!("{doc}: retired"));
                }
            }
            (Document::Opaque(want), Document::Opaque(got)) => {
                if mochiko_cli::model::canonical_hash(want)
                    != mochiko_cli::model::canonical_hash(got)
                {
                    divergences.push(format!("{doc}: the opaque document is not the shipped one"));
                }
            }
            _ => divergences.push(format!("{doc}: the replayed document is of another kind")),
        }
    }

    assert!(
        divergences.is_empty(),
        "{} divergences between the shipped corpus and the log:\n{}",
        divergences.len(),
        divergences.join("\n")
    );
}

fn compare_schema(
    doc: &DocRef,
    want: &RuleSchema,
    got: &RuleSchema,
    anchors: &BTreeMap<String, String>,
    reasons: &BTreeMap<&str, &str>,
    out: &mut Vec<String>,
) {
    let mut field = |name: &str, equal: bool| {
        if !equal {
            out.push(format!("{doc}: {name}"));
        }
    };
    field("kind", want.declared_kind == got.declared_kind);
    field("name", want.declared_name == got.declared_name);
    field("vars", want.vars == got.vars);
    field("conditions", want.conditions == got.conditions);
    field("moments", want.moments == got.moments);
    field("tombstones", want.tombstones == got.tombstones);

    let want_sections: Vec<&str> = want.sections.iter().map(|s| s.id.as_str()).collect();
    let got_sections: Vec<&str> = got.sections.iter().map(|s| s.id.as_str()).collect();
    if want_sections != got_sections {
        out.push(format!("{doc}: the section list"));
        return;
    }
    for (a, b) in want.sections.iter().zip(&got.sections) {
        if a.title != b.title {
            out.push(format!("{doc} · {}: title", a.id));
        }
        if a.intent != b.intent {
            out.push(format!("{doc} · {}: intent", a.id));
        }
        if a.note != b.note {
            out.push(format!("{doc} · {}: note", a.id));
        }
    }

    let want_ids: Vec<&str> = want.rules().map(|r| r.id.as_str()).collect();
    let got_ids: Vec<&str> = got.rules().map(|r| r.id.as_str()).collect();
    if want_ids != got_ids {
        out.push(format!("{doc}: the rule list or its order"));
        return;
    }
    for (a, b) in want.rules().zip(got.rules()) {
        compare_rule(doc, a, b, anchors, reasons, out);
    }
}

fn compare_rule(
    doc: &DocRef,
    want: &Rule,
    got: &Rule,
    anchors: &BTreeMap<String, String>,
    reasons: &BTreeMap<&str, &str>,
    out: &mut Vec<String>,
) {
    let mut field = |name: &str, equal: bool| {
        if !equal {
            out.push(format!("{doc} · {} · {name}", want.id));
        }
    };
    field("labels", want.labels == got.labels);
    field("class", want.class == got.class);
    field("kind", want.kind == got.kind);
    field("text", want.text == got.text);
    field("when", want.when == got.when);
    field("pointer", want.pointer == got.pointer);
    field("extends", want.extends == got.extends);
    field("enforces", want.enforces == got.enforces);

    // Delta one: the comment-carried reason is data in the log and a comment in the file.
    let expected_note = reasons.get(want.id.as_str()).map(|n| (*n).to_string());
    if got.note != expected_note {
        out.push(format!(
            "{doc} · {} · note: want {expected_note:?}, log carries {:?}",
            want.id, got.note
        ));
    }

    // Delta two: the sidecar's anchor rides the rule.
    let expected_anchor = anchors.get(&want.id).cloned();
    if got.anchor != expected_anchor {
        out.push(format!(
            "{doc} · {} · anchor: want {expected_anchor:?}, log carries {:?}",
            want.id, got.anchor
        ));
    }
}

// ---------------------------------------------------------------------------
// wave 4 — the fail-conditions reword
// ---------------------------------------------------------------------------

/// The six commands and the `visit` / `run` word each takes: a desk converges per visit, a run
/// against a fixed done condition.
const REWORDED: [(&str, &str, &str); 6] = [
    ("architecture", "arch", "visit"),
    ("feature", "feat", "visit"),
    ("brainstorm", "brainstorm", "run"),
    ("implement", "impl", "run"),
    ("setup", "setup", "run"),
    ("specify", "spec", "run"),
];

#[test]
fn the_second_migration_reworded_the_six_fail_conditions_intents() {
    let state = replay::load(&log_dir()).expect("the log is deliverable");
    for (command, prefix, word) in REWORDED {
        let doc = DocRef::new(DocKind::Command, command);
        let schema = state
            .docs
            .get(&doc)
            .and_then(Document::as_rules)
            .unwrap_or_else(|| panic!("{command}: the command schema is in state"));
        let id = format!("{prefix}.sec.fail-conditions");
        let section = schema
            .find_section(&id)
            .unwrap_or_else(|| panic!("{id} is a live section"));
        assert_eq!(
            section.intent,
            format!(
                "The kind: fail set — any one standing fails the {word}; the .md Not-done line \
                 cites the count this render prints."
            ),
            "{command}: the reworded intent"
        );
        assert!(
            !section.intent.contains("hard-codes"),
            "{command}: the intent still claims a hard-coded count"
        );
        // A reword is prose only: every fail rule in the section is still there, still `fail`.
        assert!(
            !section.rules.is_empty(),
            "{command}: the fail set is not empty"
        );
        for rule in &section.rules {
            assert!(
                rule.id.starts_with(&format!("{prefix}.fail.")),
                "{}: a fail-conditions rule keeps its id segment",
                rule.id
            );
        }
    }
}

#[test]
fn the_rendered_fail_conditions_block_carries_the_reworded_intent() {
    use mochiko_cli::render::{self, Context};

    let state = replay::load(&log_dir()).expect("the log is deliverable");
    let ctx = Context {
        binary: env!("CARGO_PKG_VERSION").to_string(),
        grammar: 1,
        plugin: "test".to_string(),
    };
    for (command, prefix, word) in REWORDED {
        let doc = DocRef::new(DocKind::Command, command);
        let id = format!("{prefix}.sec.fail-conditions");
        let text = render::section(&state, &doc, &id, &ctx)
            .unwrap_or_else(|e| panic!("{command}: the section renders: {e}"));
        assert!(
            text.contains(&format!(
                "fails the {word}; the .md Not-done line cites the count this render prints."
            )),
            "{command}: the rendered block carries the reworded intent:\n{text}"
        );
        assert!(
            !text.contains("hard-codes"),
            "{command}: the rendered block still claims a hard-coded count"
        );
    }
}

// ---------------------------------------------------------------------------
// the two deltas, pinned in their own right
// ---------------------------------------------------------------------------

#[test]
fn the_comment_carried_reasons_are_data_in_the_log() {
    let state = replay::load(&log_dir()).expect("the log is deliverable");
    let setup = state
        .docs
        .get(&DocRef::new(DocKind::Command, "setup"))
        .and_then(Document::as_rules)
        .expect("the setup command schema is in state");

    for (id, note) in CARRIED_REASONS {
        let rule = setup
            .find_rule(id)
            .unwrap_or_else(|| panic!("{id} is a live rule of the setup schema"));
        assert_eq!(
            rule.enforces.as_deref(),
            Some(&[] as &[String]),
            "{id} carries the empty mirror"
        );
        assert_eq!(rule.note.as_deref(), Some(note), "{id}: the lifted reason");
    }
}

#[test]
fn a_rule_lacking_its_reason_comment_stops_the_build() {
    // The lift is generic, so the guard must be too: an empty mirror with no marker comment is
    // an error, not a silently note-less rule.
    let text = "\
sections:
  - id: demo.sec.fail-conditions
    rules:
      - id: demo.fail.explained
        # D6 empty-with-reason: the obligation is owned by a pointer skill,
        # bound at demo.read-first — no local rule states it.
        enforces: []
      - id: demo.fail.bare
        enforces: []
";
    let lifted = genesis::empty_enforces_reasons(text);
    assert_eq!(
        lifted.get("demo.fail.explained").map(String::as_str),
        Some(
            "the obligation is owned by a pointer skill, bound at demo.read-first — no local rule \
             states it."
        )
    );
    assert!(
        !lifted.contains_key("demo.fail.bare"),
        "an unexplained empty mirror has no reason to lift"
    );
}

#[test]
fn a_comment_separated_from_its_mirror_is_never_claimed_by_it() {
    let text = "\
      - id: demo.fail.one
        # D6 empty-with-reason: this reason belongs to the class line below it,
        class: floor
        enforces: []
";
    assert!(
        genesis::empty_enforces_reasons(text).is_empty(),
        "content between the comment and the mirror breaks the claim"
    );
}

#[test]
fn the_sidecar_anchors_ride_their_rules() {
    let state = replay::load(&log_dir()).expect("the log is deliverable");
    let anchors = sidecar();
    assert_eq!(anchors.len(), 597, "the sidecar's recorded census");

    let mut anchored = 0usize;
    for document in state.docs.values() {
        let Some(schema) = document.as_rules() else {
            continue;
        };
        for rule in schema.rules() {
            match (rule.anchor.as_deref(), anchors.get(&rule.id)) {
                (Some(carried), Some(expected)) => {
                    assert_eq!(carried, expected, "{}: the anchor", rule.id);
                    anchored += 1;
                }
                (Some(_), None) => panic!("{}: carries an anchor the sidecar has not", rule.id),
                (None, Some(_)) => panic!("{}: the sidecar's anchor was not folded", rule.id),
                (None, None) => {}
            }
        }
    }
    assert_eq!(anchored, 597, "every sidecar anchor found its rule");
}

#[test]
fn the_sidecar_file_is_never_written() {
    // The anchors are carried, not moved (record D2). A generator that rewrote the sidecar would
    // break the Python checkers, which stay authoritative on it until they retire.
    let path = repo_root().join(genesis::SIDECAR);
    let before = std::fs::read(&path).expect("the sidecar is readable");
    let _ = genesis::build(&repo_root()).expect("genesis builds");
    let after = std::fs::read(&path).expect("the sidecar is still readable");
    assert_eq!(before, after, "genesis wrote to the provenance sidecar");
}

#[test]
fn the_corpus_census_holds_through_the_log() {
    let state = replay::load(&log_dir()).expect("the log is deliverable");
    let mut command_rules = 0usize;
    let mut skill_rules = 0usize;
    let mut skill_floors = 0usize;
    let mut command_floors = 0usize;
    let mut fail_nodes = 0usize;

    for (doc, document) in &state.docs {
        let Some(schema) = document.as_rules() else {
            continue;
        };
        for rule in schema.rules() {
            match doc.kind {
                DocKind::Command => {
                    command_rules += 1;
                    if rule.is_floor() {
                        command_floors += 1;
                    }
                    if rule.is_fail() {
                        fail_nodes += 1;
                    }
                }
                DocKind::Skill => {
                    skill_rules += 1;
                    if rule.is_floor() {
                        skill_floors += 1;
                    }
                }
                _ => {}
            }
        }
    }

    assert_eq!(command_rules, 321, "live command rules");
    assert_eq!(skill_rules, 695, "live skill rules");
    assert_eq!(command_rules + skill_rules, 1016, "live rules in total");
    assert_eq!(skill_floors, 226, "skill floors");
    assert_eq!(command_floors, 110, "declared command floors");
    assert_eq!(fail_nodes, 36, "command fail nodes");
}
