//! The shipped definition lines, read by `mochiko-cli ids --check` (`human-readable-ids`, wave 2).
//!
//! Every line the replayed log now writes where an ID is minted — a template's definition row or
//! heading, or the form a minting rule quotes — must index as a joined definition. Each line is
//! instantiated, written at its family's definition site in a scratch tree, and cited twice, once
//! bare and once joined: the bare citation is the line's one finding, and the joined citation,
//! carrying the slug the line defines, is none. A line that failed to index would leave its bare
//! citation unreported; one that indexed bare would move the finding onto the definition line;
//! one whose slug read differently would add drift. The exact finding set catches all three.
//!
//! One exception: a file name that carries its ID and a slug defines that ID ahead of any line,
//! so at the feature entry's own `FEAT-XXX-<slug>.md` site the title line is held only to agree
//! with its name. The title is held by its line at a second, bare-named entry site.
//!
//! The tree is written under `CARGO_TARGET_TMPDIR` with its own `.git`; the only read of this
//! repository is the replay of the shipped log. Concrete IDs appear only inside string literals,
//! which the check's quote mask leaves alone, so this file stays clean under its own check.

use mochiko_cli::ids;
use mochiko_cli::model::{DocKind, DocRef, Document};
use mochiko_cli::render;
use mochiko_cli::replay;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The repository root, anchored at the crate directory.
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Where a definition line comes from in the replayed log.
enum Source {
    /// The first line of a template's producer view whose text, trimmed, starts with the anchor.
    Template(&'static str),
    /// The code span in a rule's text that starts with the anchor, backtick included.
    Rule(DocKind, &'static str, &'static str),
}

/// One definition line under test.
struct Row {
    source: Source,
    anchor: &'static str,
    /// The family the line's first token must read as.
    family: &'static str,
    /// The tree-relative definition site the family table names for it.
    site: &'static str,
    /// Where the two citations go: `CLAUDE.md` for a project-wide family; for a per-artifact one,
    /// the definition file itself, since a citation outside it would owe a qualifier.
    cite_in: &'static str,
    /// What a `{{XXX}}` placeholder fills to in place of the default, where two rows instantiate
    /// the same line and so need two IDs.
    number: Option<&'static str>,
}

const ROWS: &[Row] = &[
    Row {
        source: Source::Template("feature-entry"),
        anchor: "# FEAT-{{XXX}}-{{slug}} — ",
        family: "FEAT",
        site: ".mochiko/features/FEAT-003-alpha-beta-gamma.md",
        cite_in: "CLAUDE.md",
        number: None,
    },
    Row {
        source: Source::Template("feature-entry"),
        anchor: "# FEAT-{{XXX}}-{{slug}} — ",
        family: "FEAT",
        site: ".mochiko/features/FEAT-005.md",
        cite_in: "CLAUDE.md",
        number: Some("005"),
    },
    Row {
        source: Source::Rule(
            DocKind::Skill,
            "authoring-epic",
            "authoring-epic.manifest-required-fields",
        ),
        anchor: "`# EPIC-",
        family: "EPIC",
        site: ".mochiko/epics/EPIC-004/manifest.md",
        cite_in: "CLAUDE.md",
        number: None,
    },
    Row {
        source: Source::Template("governance-intent"),
        anchor: "- **GI-001-",
        family: "GI",
        site: ".mochiko/memory/governance-intent.md",
        cite_in: "CLAUDE.md",
        number: None,
    },
    Row {
        source: Source::Template("governance-intent"),
        anchor: "| GI-0XX-",
        family: "GI",
        site: ".mochiko/memory/governance-intent.md",
        cite_in: "CLAUDE.md",
        number: None,
    },
    Row {
        source: Source::Template("architecture-concerns"),
        anchor: "## AX-001-",
        family: "AX",
        site: ".mochiko/product/architecture/concerns.md",
        cite_in: "CLAUDE.md",
        number: None,
    },
    Row {
        source: Source::Template("architecture-concerns"),
        anchor: "- **Targets**: NFR-",
        family: "NFR",
        site: ".mochiko/product/architecture/concerns.md",
        cite_in: "CLAUDE.md",
        number: None,
    },
    Row {
        source: Source::Template("architecture-spine"),
        anchor: "| SPN-001-",
        family: "SPN",
        site: ".mochiko/product/architecture/spine.md",
        cite_in: "CLAUDE.md",
        number: None,
    },
    Row {
        source: Source::Template("spec"),
        anchor: "| SCR-001-",
        family: "SCR",
        site: ".mochiko/specs/demo-spec/spec.md",
        cite_in: ".mochiko/specs/demo-spec/spec.md",
        number: None,
    },
    Row {
        source: Source::Template("spec"),
        anchor: "| FLOW-001-",
        family: "FLOW",
        site: ".mochiko/specs/demo-spec/spec.md",
        cite_in: ".mochiko/specs/demo-spec/spec.md",
        number: None,
    },
    Row {
        source: Source::Rule(DocKind::Command, "brainstorm", "brainstorm.decision-cards"),
        anchor: "`### D",
        family: "session D",
        site: ".mochiko/brainstorms/demo-topic/record.md",
        cite_in: ".mochiko/brainstorms/demo-topic/record.md",
        number: None,
    },
    Row {
        source: Source::Template("tasks"),
        anchor: "### - [ ] C1-",
        family: "cycle C",
        site: ".mochiko/features/FEAT-009/tasks.md",
        cite_in: ".mochiko/features/FEAT-009/tasks.md",
        number: None,
    },
];

/// The row's line as the replayed log writes it, placeholders still in place.
fn definition_line(state: &replay::State, log: &Path, row: &Row) -> String {
    match &row.source {
        Source::Template(name) => {
            let view = render::template_view(state, name, false, log)
                .unwrap_or_else(|e| panic!("the `{name}` template renders: {e:?}"));
            view.lines()
                .map(str::trim)
                .find(|line| line.starts_with(row.anchor))
                .unwrap_or_else(|| panic!("`{name}` writes no line opening {:?}", row.anchor))
                .to_string()
        }
        Source::Rule(kind, doc, rule) => {
            let doc = DocRef::new(*kind, *doc);
            let Some(Document::Rules(schema)) = state.docs.get(&doc) else {
                panic!("the log carries no rule document `{doc}`");
            };
            let text = schema
                .find_rule(rule)
                .and_then(|r| r.text.clone())
                .unwrap_or_else(|| panic!("`{doc}` carries no text for `{rule}`"));
            let at = text
                .find(row.anchor)
                .unwrap_or_else(|| panic!("`{rule}` quotes no span opening {:?}", row.anchor));
            let span = &text[at + 1..];
            let end = span.find('`').expect("the code span closes");
            span[..end].to_string()
        }
    }
}

/// The line with its placeholders filled: one number per family's padding, one three-word slug.
fn instantiate(line: &str, number: Option<&str>) -> String {
    line.replace("{{XXX}}", number.unwrap_or("003"))
        .replace("{{slug}}", "alpha-beta-gamma")
        .replace("<slug>", "alpha-beta-gamma")
        .replace("0XX", "012")
        .replace("XXX", "004")
        .replace("<n>", "3")
}

/// Append `line` to the file at `rel`, returning its one-based line number.
fn push(files: &mut BTreeMap<&'static str, Vec<String>>, rel: &'static str, line: &str) -> usize {
    let lines = files.entry(rel).or_default();
    lines.push(line.to_string());
    lines.len()
}

#[test]
fn every_shipped_definition_line_indexes_as_a_joined_definition() {
    let log = repo_root().join("plugins/mochiko/migrations");
    let state = replay::load(&log).expect("the shipped log replays");

    let mut files: BTreeMap<&'static str, Vec<String>> = BTreeMap::new();
    let mut expected = Vec::new();
    for row in ROWS {
        let line = instantiate(&definition_line(&state, &log, row), row.number);
        let token = ids::scan(&line)
            .into_iter()
            .next()
            .unwrap_or_else(|| panic!("{line:?} carries no ID token"));
        assert_eq!(
            token.family.name, row.family,
            "{line:?} opens with the wrong family"
        );
        let bare = format!("{}{}", token.family.prefix, token.number);
        let slug = token
            .slug
            .clone()
            .unwrap_or_else(|| panic!("{line:?} defines its ID bare"));
        push(&mut files, row.site, &line);
        let at = push(&mut files, row.cite_in, &format!("See {bare} here."));
        expected.push((PathBuf::from(row.cite_in), at, bare.clone()));
        push(&mut files, row.cite_in, &format!("See {bare}-{slug} here."));
    }
    // The negative control: a line that does not open with its ID defines nothing, so neither
    // its joined mention nor a bare citation elsewhere is tied to a definition.
    push(
        &mut files,
        ".mochiko/memory/governance-intent.md",
        "- see GI-020-kilo-lima-mike here",
    );
    push(&mut files, "CLAUDE.md", "See GI-020 here.");

    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("ids-definition-lines");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join(".git")).expect("scratch tree is creatable");
    for (rel, lines) in &files {
        let path = root.join(rel);
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("dirs");
        std::fs::write(&path, lines.join("\n") + "\n").expect("fixture is writable");
    }

    let findings = ids::check(&root, &[], &[]);
    assert!(
        findings.iter().all(|f| !f.token.contains("GI-020")),
        "a line that does not open with its ID defines nothing, got:\n{findings:#?}"
    );
    let mut found: Vec<(PathBuf, usize, String)> = findings
        .iter()
        .map(|f| {
            assert_eq!(f.kind, ids::Kind::Bare, "only bare citations are findings");
            (f.path.clone(), f.line, f.token.clone())
        })
        .collect();
    found.sort();
    expected.sort();
    assert_eq!(
        found, expected,
        "each shipped definition line indexes joined: its bare citation is the one finding"
    );
}
