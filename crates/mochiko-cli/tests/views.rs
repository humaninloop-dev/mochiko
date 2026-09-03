//! The derived views — the replayed state written back out in the corpus's own file shapes.
//!
//! Equality here is **semantic**, never byte-wise (wave plan §5). A view is compared with its
//! shipped file through the canonical hash, which sorts mapping keys and ignores the file's
//! comments, its blank lines, and the scalar style a given string was written in. Byte equality
//! is not available and is not claimed: comments do not survive a typed model, and rule field
//! order normalises on emit (P1's A11).
//!
//! Every write in this suite lands under `CARGO_TARGET_TMPDIR`. No shipped file is touched.

use mochiko_cli::model::{canonical_hash, DocKind, Document};
use mochiko_cli::replay::State;
use mochiko_cli::{genesis, views};
use serde_norway::Value;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the crate sits two levels under the repository root")
        .to_path_buf()
}

/// The shipped corpus decoded into a state directly, without going through the log.
///
/// Deliberately not the replayed state: this suite grades the emitter, so its input must not
/// depend on genesis being right. `tests/fidelity.rs` grades the log.
fn shipped_state() -> State {
    let mut state = State::default();
    for file in genesis::scan(&repo_root()).expect("the shipped corpus scans") {
        let document = Document::from_value(file.doc.kind, &file.value)
            .unwrap_or_else(|e| panic!("{} decodes: {e}", file.path.display()));
        state.docs.insert(file.doc, document);
    }
    state
}

fn parse(text: &str) -> Value {
    serde_norway::from_str(text).unwrap_or_else(|e| panic!("the emitted view parses: {e}\n{text}"))
}

// ---------------------------------------------------------------------------
// the writer
// ---------------------------------------------------------------------------

#[test]
fn the_writer_round_trips_every_scalar_shape_the_corpus_uses() {
    let cases: Vec<(&str, Value)> =
        vec![
        ("plain", Value::String("a plain scalar".into())),
        ("empty", Value::String(String::new())),
        ("colon", Value::String("a scalar: with a colon and # a hash".into())),
        ("yes", Value::String("yes".into())),
        ("no", Value::String("no".into())),
        ("numberish", Value::String("42".into())),
        ("nullish", Value::String("null".into())),
        ("leading-dash", Value::String("- not a list item".into())),
        ("trailing-space", Value::String("keeps its space ".into())),
        ("double-space", Value::String("two  spaces inside".into())),
        ("quote", Value::String("it's quoted 'twice' \"over\"".into())),
        (
            "long",
            Value::String(
                "A single-line string long enough that the writer folds it across several lines \
                 rather than running one line past every reasonable width, which is what the \
                 corpus does with rule text."
                    .into(),
            ),
        ),
        (
            "multiline",
            Value::String("first line\nsecond line\n\nfourth after a blank\n".into()),
        ),
        (
            "multiline-no-trailing",
            Value::String("first line\nsecond line".into()),
        ),
        ("bool", Value::Bool(true)),
        ("int", Value::Number(7.into())),
        ("null", Value::Null),
    ];

    for (key, value) in cases {
        let mut map = serde_norway::Mapping::new();
        map.insert(Value::String(key.into()), value.clone());
        let text = views::to_yaml(&Value::Mapping(map));
        let back = parse(&text);
        assert_eq!(
            back.get(key),
            Some(&value),
            "{key} did not survive the writer:\n{text}"
        );
    }
}

#[test]
fn the_writer_round_trips_nested_containers() {
    let source = "\
kind: command
command: demo
vars:
  target: plugins/mochiko/schemas/demo.yaml
  empty_map: {}
  empty_list: []
conditions:
  mode:
    values: [deep, shallow]
    resolution: moment-resolved(open)
    note: ruled at the open.
  presence_dim:
    values: presence
sections:
  - id: demo.sec.tools
    title: Tools
    intent: what the run may reach for
    rules:
      - id: demo.read-first
        labels: [binding, role]
        class: floor
        kind: binding
        text: Read the thing.
        when: {mode: deep, seats: multi}
        enforces: []
      - id: demo.list-when
        class: must
        when: {scope: [epic, lane]}
        text: A rule whose when term carries a list.
tombstones:
  - id: demo.legacy
    disposition: retired at the scaffold wave
";
    let original: Value = parse(source);
    let text = views::to_yaml(&original);
    assert_eq!(
        canonical_hash(&parse(&text)),
        canonical_hash(&original),
        "the writer did not round-trip nested containers:\n{text}"
    );
}

#[test]
fn short_scalar_lists_and_when_terms_stay_inline() {
    let source = "\
kind: command
command: demo
sections:
  - id: demo.sec.tools
    title: Tools
    intent: intent
    rules:
      - id: demo.a
        labels: [binding, role]
        class: must
        when: {mode: deep}
        enforces: [demo.b]
        text: short.
";
    let text = views::to_yaml(&parse(source));
    assert!(
        text.contains("labels: [binding, role]"),
        "labels should stay inline:\n{text}"
    );
    assert!(
        text.contains("when: {mode: deep}"),
        "a when term should stay inline:\n{text}"
    );
    assert!(
        text.contains("enforces: [demo.b]"),
        "enforces should stay inline:\n{text}"
    );
}

// ---------------------------------------------------------------------------
// the emitter
// ---------------------------------------------------------------------------

#[test]
fn every_shipped_document_emits_a_semantically_equal_view() {
    let root = repo_root();
    let state = shipped_state();
    let views = views::emit(&state);
    assert_eq!(views.len(), 50, "the corpus is 50 documents");

    let mut divergences: Vec<String> = Vec::new();
    for (relative, text) in &views {
        let shipped = root.join(relative);
        let original: Value = serde_norway::from_str(
            &std::fs::read_to_string(&shipped)
                .unwrap_or_else(|e| panic!("{} is readable: {e}", shipped.display())),
        )
        .unwrap_or_else(|e| panic!("{} parses: {e}", shipped.display()));
        let emitted: Value = match serde_norway::from_str(text) {
            Ok(value) => value,
            Err(e) => {
                divergences.push(format!(
                    "{}: the view does not parse: {e}",
                    relative.display()
                ));
                continue;
            }
        };
        if canonical_hash(&emitted) != canonical_hash(&original) {
            divergences.push(format!(
                "{}: the view is not semantically equal to the shipped file",
                relative.display()
            ));
        }
    }
    assert!(
        divergences.is_empty(),
        "{} of 50 views diverged:\n{}",
        divergences.len(),
        divergences.join("\n")
    );
}

#[test]
fn a_view_re_decodes_into_the_document_it_came_from() {
    let state = shipped_state();
    for (doc, document) in &state.docs {
        let text = views::render(doc, document);
        let value: Value =
            serde_norway::from_str(&text).unwrap_or_else(|e| panic!("{doc}: the view parses: {e}"));
        let back = Document::from_value(doc.kind, &value)
            .unwrap_or_else(|e| panic!("{doc}: the view decodes: {e}"));
        assert_eq!(&back, document, "{doc}: the view is not the document");
    }
}

#[test]
fn the_regenerated_command_header_matches_the_shipped_one() {
    let root = repo_root();
    let state = shipped_state();
    for (doc, _) in state
        .docs
        .iter()
        .filter(|(d, _)| d.kind == DocKind::Command)
    {
        let shipped = std::fs::read_to_string(root.join(views::view_path(doc)))
            .unwrap_or_else(|e| panic!("{doc} is readable: {e}"));
        let head: String = shipped
            .lines()
            .take_while(|line| line.starts_with('#'))
            .map(|line| format!("{line}\n"))
            .collect();
        assert_eq!(
            views::header(doc),
            head,
            "{doc}: the regenerated runtime-kernel header is not the shipped one"
        );
    }
}

#[test]
fn a_view_path_mirrors_the_repository_layout() {
    use mochiko_cli::model::DocRef;
    let cases = [
        (
            DocRef::new(DocKind::Command, "specify"),
            "plugins/mochiko/schemas/specify.yaml",
        ),
        (
            DocRef::new(DocKind::Skill, "review-feasibility"),
            "plugins/mochiko/skills/review-feasibility/schema.yaml",
        ),
        (
            DocRef::new(DocKind::CommandCommon, "common"),
            "plugins/mochiko/schemas/common.yaml",
        ),
        (
            DocRef::new(DocKind::SkillCommon, "skill-review-common"),
            "plugins/mochiko/schemas/skill-review-common.yaml",
        ),
        (
            DocRef::new(DocKind::CommandLabels, "command-labels"),
            "plugins/mochiko/schemas/command-labels.yaml",
        ),
        (
            DocRef::new(DocKind::Template, "spec"),
            "plugins/mochiko/schemas/spec.yaml",
        ),
        (
            DocRef::new(DocKind::Shelf, "architecture-shelf-backend"),
            "plugins/mochiko/schemas/architecture-shelf-backend.yaml",
        ),
    ];
    for (doc, want) in cases {
        assert_eq!(views::view_path(&doc), PathBuf::from(want), "{doc}");
    }
}

#[test]
fn emit_to_writes_only_under_the_out_directory() {
    let out = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("views-emit-to");
    let _ = std::fs::remove_dir_all(&out);
    let state = shipped_state();
    let written = views::emit_to(&state, &out).expect("the views write");

    assert_eq!(written.len(), 50);
    for path in &written {
        assert!(path.starts_with(&out), "{} escaped --out", path.display());
        assert!(path.is_file(), "{} was not written", path.display());
    }
    assert!(out.join("plugins/mochiko/schemas/specify.yaml").is_file());
    assert!(out
        .join("plugins/mochiko/skills/review-feasibility/schema.yaml")
        .is_file());
}
