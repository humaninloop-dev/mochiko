//! The derived views — the replayed state written back out in the corpus's own file shapes.
//!
//! Equality here is **semantic**, never byte-wise (wave plan §5). A view is compared with its
//! committed file through the canonical hash, which sorts mapping keys and ignores the file's
//! comments, its blank lines, and the scalar style a given string was written in. Byte equality
//! is not available and is not claimed: comments do not survive a typed model, and rule field
//! order normalises on emit (P1's A11).
//!
//! **The comparand moved at wave 6.** Through wave 5 an emitted view was compared with the shipped
//! snapshot file it mirrored. No schema file ships now, so the comparand is the committed view
//! under `.mochiko/schema-views/`: emit and the committed tree must agree, which is the same
//! "view ≡ replay" claim CI gate 5 makes, keyed on the surface that still exists. Drift is a
//! failing test, and the fix is to regenerate.
//!
//! Every write in this suite lands under `CARGO_TARGET_TMPDIR`. No committed file is touched.

use mochiko_cli::model::{canonical_hash, DocKind, Document};
use mochiko_cli::replay::{self, State};
use mochiko_cli::views;
use serde_norway::Value;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the crate sits two levels under the repository root")
        .to_path_buf()
}

/// Where the committed views live, relative to the repository root.
const VIEWS_DIR: &str = ".mochiko/schema-views";

/// The corpus as the log replays it — the only source of schema content from wave 6.
///
/// Through wave 5 this decoded the shipped files directly, so the emitter could be graded without
/// depending on genesis being right. Those files are gone; `tests/fidelity.rs` still grades
/// genesis against the frozen corpus, which is where that independence now lives.
fn replayed_state() -> State {
    replay::load(&repo_root().join("plugins/mochiko/migrations")).unwrap_or_else(|findings| {
        let lines: Vec<String> = findings.iter().map(ToString::to_string).collect();
        panic!("the committed log is deliverable:\n{}", lines.join("\n"))
    })
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

/// Trailing newlines are content, and a lossless projection has to keep them.
///
/// `|-` strips them, `|` keeps exactly one, `|+` keeps every one that is actually written after
/// the body. The first cut chose `|+` for two or more and then wrote no blank lines for it to
/// keep, so `"a\n\n"` and `"a\n\n\n"` both read back as `"a\n"`. Unreachable on today's
/// corpus, but this module is the GI-006 reconstruction surface.
#[test]
fn a_multiline_scalar_keeps_every_trailing_newline() {
    let cases = [
        "a",
        "a\n",
        "a\n\n",
        "a\n\n\n",
        "a\n\n\n\n",
        "one\ntwo\n\n",
        "one\n\ntwo\n\n\n",
        "\n\n",
    ];
    for text in cases {
        let mut map = serde_norway::Mapping::new();
        map.insert(Value::String("key".into()), Value::String(text.to_string()));
        let rendered = views::to_yaml(&Value::Mapping(map));
        let back = parse(&rendered);
        assert_eq!(
            back.get("key").and_then(Value::as_str),
            Some(text),
            "{text:?} did not survive the writer:\n{rendered}"
        );
    }
}

/// A long scalar folds even when it carries characters that would need quoting inline.
///
/// Inside a folded block every one of them is literal text, so the quoting guard was costing
/// readability for nothing: the corpus this module mirrors has 89 lines over 120 characters and
/// the first cut of the generated log had 330. The characters still round-trip.
#[test]
fn a_long_scalar_folds_even_when_it_would_need_quoting_inline() {
    let cases = [
        "- a rule text opening with a dash, long enough to fold across more than one line so the \
         writer has to choose between one long quoted line and a folded block",
        "the trace obligation: owned elsewhere, bound at setup.surface-set, and long enough that \
         the writer must fold it rather than run the line past every reasonable width",
        "a text carrying a hash # in the middle of it, written out at a length that forces the \
         writer to fold rather than to emit one line and quote the whole thing",
        "yes: and a colon-space pair right at the front, then enough words after it to push this \
         string past the width at which the writer starts folding its output",
    ];
    for text in cases {
        let mut map = serde_norway::Mapping::new();
        map.insert(
            Value::String("text".into()),
            Value::String(text.to_string()),
        );
        let rendered = views::to_yaml(&Value::Mapping(map));
        assert!(
            rendered.starts_with("text: >-\n"),
            "not folded:\n{rendered}"
        );
        assert!(
            rendered.lines().all(|l| l.len() <= 98),
            "a folded line ran past the width:\n{rendered}"
        );
        let back = parse(&rendered);
        assert_eq!(
            back.get("text").and_then(Value::as_str),
            Some(text),
            "{text:?} did not survive folding:\n{rendered}"
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

/// Emit and the committed tree agree, document for document — the "view ≡ replay" claim.
///
/// A failure here means the committed views are stale: regenerate them with
/// `mochiko-cli views emit --plugin-root plugins/mochiko --out .mochiko/schema-views`. It never
/// means a view should be hand-edited into agreement.
#[test]
fn every_emitted_view_matches_the_committed_one() {
    let views_dir = repo_root().join(VIEWS_DIR);
    let state = replayed_state();
    let views = views::emit(&state);
    assert_eq!(views.len(), 50, "the corpus is 50 documents");

    let mut divergences: Vec<String> = Vec::new();
    for (relative, text) in &views {
        let committed = views_dir.join(relative);
        let original: Value = serde_norway::from_str(
            &std::fs::read_to_string(&committed)
                .unwrap_or_else(|e| panic!("{} is readable: {e}", committed.display())),
        )
        .unwrap_or_else(|e| panic!("{} parses: {e}", committed.display()));
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
                "{}: the emitted view is not semantically equal to the committed one",
                relative.display()
            ));
        }
    }
    assert!(
        divergences.is_empty(),
        "{} of 50 views diverged — regenerate with `mochiko-cli views emit`:\n{}",
        divergences.len(),
        divergences.join("\n")
    );
}

/// The committed tree carries exactly the views the emitter writes, and nothing else.
///
/// The emitter creates and overwrites but never deletes, so a document that is renamed or retired
/// leaves its old view behind. Without this the stale file would sit in the tree indefinitely,
/// read as current, and pass every other test here.
#[test]
fn the_committed_views_tree_holds_no_file_the_emitter_does_not_write() {
    let views_dir = repo_root().join(VIEWS_DIR);
    let expected: std::collections::BTreeSet<PathBuf> = views::emit(&replayed_state())
        .into_iter()
        .map(|(relative, _)| views_dir.join(relative))
        .collect();

    let mut found: Vec<PathBuf> = Vec::new();
    let mut stack = vec![views_dir.clone()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("the committed views directory is readable") {
            let path = entry.expect("readable entry").path();
            if path.is_dir() {
                stack.push(path);
            } else {
                found.push(path);
            }
        }
    }

    let orphans: Vec<String> = found
        .iter()
        .filter(|path| !expected.contains(*path))
        .map(|path| path.display().to_string())
        .collect();
    assert!(
        orphans.is_empty(),
        "the committed views tree carries files the emitter does not write:\n{}",
        orphans.join("\n")
    );
    assert_eq!(found.len(), 50, "the committed tree is 50 views");
}

#[test]
fn a_view_re_decodes_into_the_document_it_came_from() {
    let state = replayed_state();
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
fn the_regenerated_command_header_matches_the_committed_one() {
    let views_dir = repo_root().join(VIEWS_DIR);
    let state = replayed_state();
    for (doc, _) in state
        .docs
        .iter()
        .filter(|(d, _)| d.kind == DocKind::Command)
    {
        let committed = std::fs::read_to_string(views_dir.join(views::view_path(doc)))
            .unwrap_or_else(|e| panic!("{doc} is readable: {e}"));
        let head: String = committed
            .lines()
            .take_while(|line| line.starts_with('#'))
            .map(|line| format!("{line}\n"))
            .collect();
        assert_eq!(
            views::header(doc),
            head,
            "{doc}: the regenerated header is not the committed one"
        );
    }
}

/// No view path names the plugin, and none names a schema directory.
///
/// The layout is keyed by document kind from wave 6, outside `plugins/` entirely, so nothing
/// about a view's path suggests a file a run could read instead of asking the binary.
#[test]
fn a_view_path_is_keyed_by_document_kind() {
    use mochiko_cli::model::DocRef;
    let cases = [
        (
            DocRef::new(DocKind::Command, "specify"),
            "commands/specify.yaml",
        ),
        (
            DocRef::new(DocKind::Skill, "review-feasibility"),
            "skills/review-feasibility.yaml",
        ),
        (
            DocRef::new(DocKind::CommandCommon, "common"),
            "common/common.yaml",
        ),
        (
            DocRef::new(DocKind::SkillCommon, "skill-review-common"),
            "common/skill-review-common.yaml",
        ),
        (
            DocRef::new(DocKind::CommandLabels, "command-labels"),
            "labels/command-labels.yaml",
        ),
        (
            DocRef::new(DocKind::SkillLabels, "skill-labels"),
            "labels/skill-labels.yaml",
        ),
        (
            DocRef::new(DocKind::Template, "spec"),
            "templates/spec.yaml",
        ),
        (
            DocRef::new(DocKind::Shelf, "architecture-shelf-backend"),
            "shelves/architecture-shelf-backend.yaml",
        ),
    ];
    for (doc, want) in cases {
        assert_eq!(views::view_path(&doc), PathBuf::from(want), "{doc}");
    }

    for (doc, path) in views::emit(&replayed_state())
        .into_iter()
        .map(|(path, _)| (path.clone(), path))
    {
        let text = path.display().to_string();
        assert!(
            !text.contains("plugins/") && !text.contains("schemas/"),
            "{doc:?}: a view path still names the shipped tree"
        );
    }
}

#[test]
fn emit_to_writes_only_under_the_out_directory() {
    let out = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("views-emit-to");
    let _ = std::fs::remove_dir_all(&out);
    let state = replayed_state();
    let written = views::emit_to(&state, &out).expect("the views write");

    assert_eq!(written.len(), 50);
    for path in &written {
        assert!(path.starts_with(&out), "{} escaped --out", path.display());
        assert!(path.is_file(), "{} was not written", path.display());
    }
    assert!(out.join("commands/specify.yaml").is_file());
    assert!(out.join("skills/review-feasibility.yaml").is_file());
    assert!(out
        .join("shelves/architecture-shelf-backend.yaml")
        .is_file());
}
