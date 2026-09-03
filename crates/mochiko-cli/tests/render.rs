//! Integration tests for the rules render and the re-based template views.
//!
//! Every fixture log is written under `CARGO_TARGET_TMPDIR`, which Cargo places inside `target/` —
//! the suite never writes outside the build directory. The rules render is exercised against a
//! synthetic corpus built here rather than the shipped one, because wave 1 has no genesis
//! migration yet; `tests/cli.rs` carries the test that runs against `migrations/` once it exists.

use mochiko_cli::model::{DocKind, DocRef};
use mochiko_cli::render::{self, Context, PREAMBLE};
use mochiko_cli::replay;
use mochiko_cli::schema;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Absolute path to the shipped schema directory, anchored at the crate root so it is independent
/// of the test process's working directory.
const SHIPPED_SCHEMAS_DIR: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../plugins/mochiko/schemas");

/// The captured template fixtures: today's producer and check views with the trailing
/// schema-source line stripped, so the byte-equality assertion names the one line that moved.
const FIXTURE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/template");

/// The eight templates the log carries. This is the fixture's own manifest, not a production
/// constant: the closed `TEMPLATE_NAMES` set left the crate when the templates became log data.
const SHIPPED_TEMPLATES: [&str; 8] = [
    "spec",
    "tasks",
    "feature-entry",
    "features-index",
    "codebase-analysis",
    "governance-intent",
    "governance-surfaces",
    "architecture-store",
];

// ---------------------------------------------------------------------------
// fixtures
// ---------------------------------------------------------------------------

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn log_dir(tag: &str) -> PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("render-{tag}-{n}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("fixture log dir is creatable");
    dir
}

/// Stamp a migration body with its `hash:` header and write it into the log directory.
///
/// The hash is required, so a fixture is stamped through the crate's own authoring helper rather
/// than hand-computed — a test that reimplemented the canonical encoding would stop testing it.
fn write_migration(dir: &Path, name: &str, body: &str) {
    let stamped = mochiko_cli::migration::with_hash(name, body)
        .unwrap_or_else(|e| panic!("fixture {name} is not a well-formed migration: {e}"));
    std::fs::write(dir.join(name), stamped).expect("fixture migration is writable");
}

/// The rule corpus: one command, one skill, both family common libraries and both registries.
///
/// Shaped to exercise the render contract rather than to be realistic — it carries a constraint
/// rule and a kinded one, a scalar `when:` and a list-valued one, an `extends:` stub, a fail node
/// with `enforces:`, a tombstone, an empty section with a note, and a `${var}` in both a rule's
/// text and a condition's note.
const RULES_LOG: &str = r#"
grammar: 1
id: 0001-genesis
sequence: 1
intent: Import a corpus shaped to exercise every limb of the render contract.
changes:
  - op: import-document
    kind: command-labels
    name: command-labels
    content:
      kind: command-labels
      labels:
        seats: Seat wiring.
        landing: The landing ritual.
        user-gate: Reserved to the user.
  - op: import-document
    kind: skill-labels
    name: skill-labels
    content:
      kind: skill-labels
      labels:
        independence: Review independence.
        scope: What the skill covers.
  - op: import-document
    kind: command-common
    name: common
    content:
      kind: command-common
      rules:
        - id: common.shared-block
          labels: [landing]
          text: The shared obligation every command inherits.
          pointer: "mochiko:grooming-operating-docs"
  - op: import-document
    kind: skill-common
    name: skill-review-common
    content:
      kind: skill-common
      rules:
        - id: review-common.shared-verdict
          labels: [independence]
          text: The shared verdict obligation every review skill inherits.
  - op: import-document
    kind: command
    name: demo
    content:
      kind: command
      command: demo
      vars:
        record_path: ".mochiko/brainstorms/<slug>/record.md"
        index_path: ".mochiko/brainstorms/index.md"
      conditions:
        km_file:
          values: presence
          resolution: surface-presence
          note: the knowledge-management file beside ${index_path}.
        seats:
          values: [single, multi]
          resolution: standing-trigger
      moments:
        session-open: Where the session is entered as open.
        close: Where the index is updated.
      sections:
        - id: demo.sec.roles
          title: Roles & Responsibilities
          intent: Lead role and seat staffing.
          rules:
            - id: demo.lead-inline
              labels: [seats]
              class: must
              kind: duty
              text: The lead plans the run and records it at ${record_path}.
              pointer: "mochiko:analysis-iterative"
            - id: demo.plain-constraint
              labels: [seats]
              class: advisory
              text: An ordinary rule carrying no kind of its own.
            - id: demo.scalar-when
              labels: [landing]
              class: must
              kind: gate
              when: {km_file: present}
              text: Run the close ritual.
            - id: demo.list-when
              labels: [seats, landing]
              class: must
              when: {seats: [single, multi]}
              text: A rule gated on either shape.
        - id: demo.sec.reserved
          title: Reserved to the user
          intent: Decisions held by the user.
          note: This command reserves nothing beyond the standing user gates.
          rules: []
        - id: demo.sec.tools
          title: Tools & Skills
          intent: The skills this command reaches for.
          rules:
            - id: demo.stub
              extends: common.shared-block
              class: floor
        - id: demo.sec.ways-of-working
          title: Ways of working
          intent: How the run proceeds.
          rules:
            - id: demo.working-rule
              labels: [landing]
              class: must
              text: Land the work before closing.
        - id: demo.sec.boundaries
          title: Boundaries
          intent: Floor obligations no seat may waive.
          rules:
            - id: demo.boundary
              labels: [user-gate]
              class: floor
              text: The user rules acceptance.
        - id: demo.sec.fail-conditions
          title: Not done — default FAIL
          intent: "The kind: fail set."
          rules:
            - id: demo.fail.unaccepted
              labels: [user-gate]
              class: floor
              kind: fail
              enforces: [demo.boundary]
              text: An unaccepted record at ${record_path}.
      tombstones:
        - id: demo.retired-rule
          disposition: Superseded by demo.working-rule.
  - op: import-document
    kind: skill
    name: review-demo
    content:
      kind: skill
      skill: review-demo
      conditions:
        depth:
          values: [low, high]
          resolution: entry-derived
      sections:
        - id: review-demo.sec.independence
          title: Independence
          intent: Who may run this skill.
          rules:
            - id: review-demo.not-the-author
              labels: [independence]
              class: floor
              text: The author never grades their own artifact.
        - id: review-demo.sec.scope
          title: Scope
          intent: What is graded.
          rules:
            - id: review-demo.scope-fence
              labels: [scope]
              class: must
              text: Grade the artifact, never the author.
        - id: review-demo.sec.inputs
          title: Inputs
          intent: What the skill reads.
          note: The skill reads the artifact and nothing else.
          rules: []
        - id: review-demo.sec.verdict
          title: Verdict
          intent: The verdict grammar.
          rules:
            - id: review-demo.inherited-verdict
              extends: review-common.shared-verdict
              class: must
        - id: review-demo.sec.output
          title: Output
          intent: The report shape.
          rules:
            - id: review-demo.report-shape
              labels: [scope]
              class: must
              when: {depth: high}
              text: The report names every finding.
        - id: review-demo.sec.reserved
          title: Reserved
          intent: Reserved to the user.
          rules:
            - id: review-demo.user-rules
              labels: [independence]
              class: floor
              text: The user rules the verdict's consequence.
"#;

/// A log carrying only the eight shipped templates, imported verbatim the way genesis will.
fn template_log(dir: &Path) {
    let mut body = String::from(
        "grammar: 1\nid: 0001-genesis\nsequence: 1\nintent: Import the shipped templates.\nchanges:\n",
    );
    for name in SHIPPED_TEMPLATES {
        let path = Path::new(SHIPPED_SCHEMAS_DIR).join(format!("{name}.yaml"));
        let yaml = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{} should be readable: {e}", path.display()));
        let content: serde_norway::Value = serde_norway::from_str(&yaml)
            .unwrap_or_else(|e| panic!("{} should parse as YAML: {e}", path.display()));
        let mut change = serde_norway::Mapping::new();
        change.insert("op".into(), "import-document".into());
        change.insert("kind".into(), "template".into());
        change.insert("name".into(), name.into());
        change.insert("content".into(), content);
        let rendered = serde_norway::to_string(&serde_norway::Value::Sequence(vec![
            serde_norway::Value::Mapping(change),
        ]))
        .expect("a change item serialises");
        for line in rendered.lines() {
            body.push_str("  ");
            body.push_str(line);
            body.push('\n');
        }
    }
    write_migration(dir, "0001-genesis.yaml", &body);
}

fn ctx() -> Context {
    Context {
        binary: "0.1.0".to_string(),
        grammar: 1,
        plugin: "0.103.0".to_string(),
    }
}

fn rules_state(tag: &str) -> mochiko_cli::replay::State {
    let dir = log_dir(tag);
    write_migration(&dir, "0001-genesis.yaml", RULES_LOG);
    replay::load(&dir).unwrap_or_else(|findings| {
        let lines: Vec<String> = findings.iter().map(ToString::to_string).collect();
        panic!(
            "the fixture corpus should be deliverable:\n{}",
            lines.join("\n")
        )
    })
}

fn command() -> DocRef {
    DocRef::new(DocKind::Command, "demo")
}

fn skill() -> DocRef {
    DocRef::new(DocKind::Skill, "review-demo")
}

// ---------------------------------------------------------------------------
// head and tail lines (D3 as amended)
// ---------------------------------------------------------------------------

#[test]
fn every_render_opens_with_the_head_line_and_closes_with_the_tail_line() {
    let state = rules_state("headtail");
    for (doc, name, sections) in [(command(), "demo", 6), (skill(), "review-demo", 6)] {
        let preamble = render::preamble(&state, &doc, &ctx()).expect("preamble renders");
        assert_eq!(
            preamble.lines().next().unwrap(),
            format!(
                "mochiko-cli rules {name} · section preamble · binary 0.1.0 · grammar 1 · plugin 0.103.0"
            ),
            "{name}: preamble head line"
        );
        assert_eq!(
            preamble.trim_end().lines().last().unwrap(),
            format!("mochiko-cli rules end · {name} · preamble · 0 rules"),
            "{name}: preamble tail line"
        );

        let schema = state.docs[&doc]
            .as_rules()
            .expect("a rule-bearing document");
        assert_eq!(schema.sections.len(), sections);
        for section in &schema.sections {
            let out = render::section(&state, &doc, &section.id, &ctx())
                .unwrap_or_else(|_| panic!("{} renders", section.id));
            let live = section.rules.len();
            assert_eq!(
                out.lines().next().unwrap(),
                format!(
                    "mochiko-cli rules {name} · section {} · binary 0.1.0 · grammar 1 · plugin 0.103.0",
                    section.id
                ),
                "{}: head line", section.id
            );
            assert_eq!(
                out.trim_end().lines().last().unwrap(),
                format!(
                    "mochiko-cli rules end · {name} · {} · {live} rules",
                    section.id
                ),
                "{}: tail line",
                section.id
            );
        }
    }
}

#[test]
fn the_tail_line_counts_the_rules_actually_rendered() {
    let state = rules_state("tailcount");
    let out = render::section(&state, &command(), "demo.sec.roles", &ctx()).unwrap();
    let blocks = out.matches("\n### ").count();
    assert_eq!(blocks, 4, "four live rules in demo.sec.roles");
    assert!(
        out.trim_end()
            .ends_with("mochiko-cli rules end · demo · demo.sec.roles · 4 rules"),
        "the tail count should equal the blocks rendered:\n{out}"
    );
}

// ---------------------------------------------------------------------------
// the preamble
// ---------------------------------------------------------------------------

#[test]
fn the_preamble_carries_the_identity_line_vars_conditions_moments_and_pins() {
    let state = rules_state("preamble");
    let out = render::preamble(&state, &command(), &ctx()).unwrap();

    assert!(out.contains("\ncommand demo\n"), "identity line:\n{out}");
    assert!(
        out.contains("- record_path = .mochiko/brainstorms/<slug>/record.md"),
        "vars block:\n{out}"
    );
    assert!(
        out.contains("- km_file · values: presence · resolution: surface-presence · note:"),
        "presence condition:\n{out}"
    );
    assert!(
        out.contains("- seats · values: single|multi · resolution: standing-trigger\n"),
        "list-valued condition, no note:\n{out}"
    );
    assert!(
        out.contains("- session-open: Where the session is entered as open."),
        "moments block:\n{out}"
    );
}

#[test]
fn a_skill_preamble_omits_moments_and_the_fail_pin() {
    let state = rules_state("skillpreamble");
    let out = render::preamble(&state, &skill(), &ctx()).unwrap();
    assert!(
        out.contains("\nskill review-demo\n"),
        "identity line:\n{out}"
    );
    assert!(
        !out.contains("moments"),
        "skills declare no moments:\n{out}"
    );
    assert!(
        !out.contains("kind: fail"),
        "the fail pin is command grammar:\n{out}"
    );
    assert!(
        out.contains("- class: floor · 2 rules"),
        "the floor pin is rendered for a skill:\n{out}"
    );
}

#[test]
fn the_preamble_pins_match_the_corpus() {
    let state = rules_state("pins");
    let out = render::preamble(&state, &command(), &ctx()).unwrap();
    assert!(out.contains("- kind: fail · 1 rules"), "fail pin:\n{out}");
    assert!(
        out.contains("- class: floor · 3 rules"),
        "floor pin:\n{out}"
    );
}

#[test]
fn the_preamble_lists_every_section_with_its_rule_count() {
    let state = rules_state("sectionlist");
    let out = render::preamble(&state, &command(), &ctx()).unwrap();
    for (id, title, count) in [
        ("demo.sec.roles", "Roles & Responsibilities", 4),
        ("demo.sec.reserved", "Reserved to the user", 0),
        ("demo.sec.tools", "Tools & Skills", 1),
        ("demo.sec.fail-conditions", "Not done — default FAIL", 1),
    ] {
        assert!(
            out.contains(&format!("- {id} · {title} · {count} rules")),
            "section row for {id}:\n{out}"
        );
    }
}

#[test]
fn a_preamble_substitutes_vars_in_a_condition_note() {
    let state = rules_state("condnote");
    let out = render::preamble(&state, &command(), &ctx()).unwrap();
    assert!(
        out.contains("note: the knowledge-management file beside .mochiko/brainstorms/index.md."),
        "the condition note should carry no placeholder:\n{out}"
    );
    assert!(!out.contains("${"), "no placeholder survives:\n{out}");
}

// ---------------------------------------------------------------------------
// the section body
// ---------------------------------------------------------------------------

#[test]
fn a_section_renders_its_title_intent_and_one_block_per_live_rule() {
    let state = rules_state("sectionbody");
    let out = render::section(&state, &command(), "demo.sec.roles", &ctx()).unwrap();
    assert!(
        out.contains("\n## Roles & Responsibilities\n"),
        "title:\n{out}"
    );
    assert!(
        out.contains("\nLead role and seat staffing.\n"),
        "intent:\n{out}"
    );
    for id in [
        "demo.lead-inline",
        "demo.plain-constraint",
        "demo.scalar-when",
        "demo.list-when",
    ] {
        assert!(
            out.contains(&format!("\n### {id}\n")),
            "block for {id}:\n{out}"
        );
    }
}

#[test]
fn the_bracket_line_omits_kind_for_a_constraint_and_shows_it_otherwise() {
    let state = rules_state("bracketkind");
    let out = render::section(&state, &command(), "demo.sec.roles", &ctx()).unwrap();
    assert!(
        out.contains("\n### demo.plain-constraint\n[class: advisory · labels: seats]\n"),
        "a constraint's bracket names no kind:\n{out}"
    );
    assert!(
        out.contains("· kind: duty ·"),
        "a kinded rule names its kind:\n{out}"
    );
}

#[test]
fn the_bracket_line_carries_when_labels_and_pointer_in_order() {
    let state = rules_state("bracketfields");
    let out = render::section(&state, &command(), "demo.sec.roles", &ctx()).unwrap();
    assert!(
        out.contains(
            "[class: must · kind: duty · labels: seats · pointer: mochiko:analysis-iterative]"
        ),
        "class · kind · labels · pointer:\n{out}"
    );
    assert!(
        out.contains("[class: must · kind: gate · when: km_file=present · labels: landing]"),
        "a scalar when term:\n{out}"
    );
    assert!(
        out.contains("[class: must · when: seats=single|multi · labels: seats, landing]"),
        "a list-valued when term and two labels:\n{out}"
    );
}

#[test]
fn an_extends_stub_renders_inherited_text_labels_and_pointer_with_its_local_class() {
    let state = rules_state("extends");
    let out = render::section(&state, &command(), "demo.sec.tools", &ctx()).unwrap();
    assert!(
        out.contains("The shared obligation every command inherits."),
        "inherited text:\n{out}"
    );
    assert!(
        out.contains("[class: floor · labels: landing · pointer: mochiko:grooming-operating-docs]"),
        "inherited labels and pointer, local class:\n{out}"
    );
    assert!(
        !out.contains("extends:"),
        "the render resolves the binding rather than reporting it:\n{out}"
    );
}

#[test]
fn a_skill_extends_stub_resolves_against_its_family_library() {
    let state = rules_state("skillextends");
    let out = render::section(&state, &skill(), "review-demo.sec.verdict", &ctx()).unwrap();
    assert!(
        out.contains("The shared verdict obligation every review skill inherits."),
        "inherited text:\n{out}"
    );
}

#[test]
fn every_var_placeholder_is_substituted_in_rule_text() {
    let state = rules_state("vars");
    let out = render::section(&state, &command(), "demo.sec.roles", &ctx()).unwrap();
    assert!(
        out.contains("records it at .mochiko/brainstorms/<slug>/record.md."),
        "the rule text should carry no placeholder:\n{out}"
    );
    assert!(!out.contains("${"), "no placeholder survives:\n{out}");
}

#[test]
fn a_fail_node_renders_its_enforces_list() {
    let state = rules_state("fail");
    let out = render::section(&state, &command(), "demo.sec.fail-conditions", &ctx()).unwrap();
    assert!(out.contains("· kind: fail ·"), "the fail kind:\n{out}");
    assert!(
        out.contains("\nenforces: demo.boundary\n"),
        "the enforces line:\n{out}"
    );
}

#[test]
fn a_non_fail_rule_renders_no_enforces_line() {
    let state = rules_state("noenforces");
    let out = render::section(&state, &command(), "demo.sec.roles", &ctx()).unwrap();
    assert!(!out.contains("enforces:"), "no enforces line:\n{out}");
}

#[test]
fn an_empty_section_renders_its_note_and_counts_zero_rules() {
    let state = rules_state("emptysection");
    let out = render::section(&state, &command(), "demo.sec.reserved", &ctx()).unwrap();
    assert!(
        out.contains("\nnote: This command reserves nothing beyond the standing user gates.\n"),
        "the section note stands in for the rules:\n{out}"
    );
    assert!(!out.contains("### "), "no rule blocks:\n{out}");
    assert!(
        out.trim_end()
            .ends_with("mochiko-cli rules end · demo · demo.sec.reserved · 0 rules"),
        "zero rules:\n{out}"
    );
}

#[test]
fn a_tombstoned_rule_is_never_rendered() {
    let state = rules_state("tombstone");
    for section in [
        "demo.sec.roles",
        "demo.sec.reserved",
        "demo.sec.tools",
        "demo.sec.ways-of-working",
        "demo.sec.boundaries",
        "demo.sec.fail-conditions",
    ] {
        let out = render::section(&state, &command(), section, &ctx()).unwrap();
        assert!(
            !out.contains("demo.retired-rule"),
            "{section} renders a tombstoned id:\n{out}"
        );
    }
    let preamble = render::preamble(&state, &command(), &ctx()).unwrap();
    assert!(
        !preamble.contains("demo.retired-rule"),
        "the preamble renders a tombstoned id:\n{preamble}"
    );
}

/// D2/D16: the anchor is maintainer metadata that never reaches a delivered render.
#[test]
fn no_render_carries_an_anchor_or_a_rule_note() {
    let dir = log_dir("anchor");
    write_migration(&dir, "0001-genesis.yaml", RULES_LOG);
    write_migration(
        &dir,
        "0002-anchor.yaml",
        r#"
grammar: 1
id: 0002-anchor
sequence: 2
intent: Fold an anchor and a note onto a rule the way genesis will.
changes:
  - op: set-rule-field
    schema: command/demo
    id: demo.boundary
    field: anchor
    value: "2026-09-03 cli-schema-delivery [D2]"
  - op: set-rule-field
    schema: command/demo
    id: demo.fail.unaccepted
    field: note
    value: The reason this mirror is deliberately narrow.
"#,
    );
    let state = replay::load(&dir).expect("the anchored corpus is deliverable");

    let boundaries = render::section(&state, &command(), "demo.sec.boundaries", &ctx()).unwrap();
    assert!(
        !boundaries.contains("2026-09-03") && !boundaries.contains("anchor"),
        "an anchor reached a delivered render:\n{boundaries}"
    );
    let fails = render::section(&state, &command(), "demo.sec.fail-conditions", &ctx()).unwrap();
    assert!(
        !fails.contains("The reason this mirror is deliberately narrow."),
        "a rule note reached a delivered render:\n{fails}"
    );
}

// ---------------------------------------------------------------------------
// render size against the platform ceiling (F12e)
// ---------------------------------------------------------------------------

/// The whole shipped corpus as state, addressed the way genesis will address it: a document's own
/// `kind:` names its kind, a skill's name is its directory, everything else is the file stem.
fn shipped_state() -> replay::State {
    let plugin = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../plugins/mochiko"
    ));
    let mut paths: Vec<PathBuf> = std::fs::read_dir(plugin.join("schemas"))
        .expect("the shipped schema directory exists")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("yaml"))
        .collect();
    for entry in std::fs::read_dir(plugin.join("skills")).expect("the skills directory exists") {
        let path = entry.expect("readable entry").path().join("schema.yaml");
        if path.is_file() {
            paths.push(path);
        }
    }
    paths.sort();

    let mut state = replay::State::default();
    for path in paths {
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
        let document = mochiko_cli::model::Document::from_value(kind, &value)
            .unwrap_or_else(|e| panic!("{} decodes: {e}", path.display()));
        state.docs.insert(DocRef::new(kind, stem), document);
    }
    state
}

/// Every section render must stay under the Bash tool's ≈30,000-character inline ceiling — the
/// platform fact (F12e) that D3's per-section chunking exists to respect. Measured over the whole
/// shipped corpus, loaded as documents rather than through the log, so the figure is the real one
/// before genesis lands.
#[test]
fn no_shipped_section_renders_past_the_inline_ceiling() {
    const CEILING: usize = 30_000;
    let state = shipped_state();
    let mut largest = (String::new(), 0usize);
    let mut measured = 0usize;

    for (doc, document) in &state.docs {
        if !matches!(doc.kind, DocKind::Command | DocKind::Skill) {
            continue;
        }
        let schema = document.as_rules().expect("a rule-bearing document");
        for id in std::iter::once(PREAMBLE.to_string())
            .chain(schema.sections.iter().map(|s| s.id.clone()))
        {
            let out = render::section(&state, doc, &id, &ctx())
                .unwrap_or_else(|e| panic!("{} · {id}: {e}", doc.name));
            measured += 1;
            if out.chars().count() > largest.1 {
                largest = (format!("{} · {id}", doc.name), out.chars().count());
            }
        }
    }

    assert!(measured > 200, "the corpus should carry 36 primitives");
    assert!(
        largest.1 < CEILING,
        "the largest section render ({}) is {} chars, past the {CEILING} ceiling",
        largest.0,
        largest.1
    );
    eprintln!(
        "measured {measured} renders; largest is {} at {} chars (ceiling {CEILING})",
        largest.0, largest.1
    );
}

// ---------------------------------------------------------------------------
// the re-based template views
// ---------------------------------------------------------------------------

fn template_state(tag: &str) -> (mochiko_cli::replay::State, PathBuf) {
    let dir = log_dir(tag);
    template_log(&dir);
    let state = replay::load(&dir).unwrap_or_else(|findings| {
        let lines: Vec<String> = findings.iter().map(ToString::to_string).collect();
        panic!(
            "the template log should be deliverable:\n{}",
            lines.join("\n")
        )
    });
    (state, dir)
}

#[test]
fn every_template_producer_view_is_byte_identical_to_its_captured_fixture() {
    let (state, dir) = template_state("producer");
    for name in SHIPPED_TEMPLATES {
        let view = render::template_view(&state, name, false, &dir)
            .unwrap_or_else(|_| panic!("{name} should render from the log"));
        let fixture = std::fs::read_to_string(format!("{FIXTURE_DIR}/{name}.producer.txt"))
            .unwrap_or_else(|e| panic!("{name} producer fixture is readable: {e}"));
        let expected = format!("{fixture}schemas: replayed from {}\n", dir.display());
        assert_eq!(view, expected, "{name}: producer view drifted");
    }
}

#[test]
fn every_template_check_view_is_byte_identical_to_its_captured_fixture() {
    let (state, dir) = template_state("check");
    for name in SHIPPED_TEMPLATES {
        let view = render::template_view(&state, name, true, &dir)
            .unwrap_or_else(|_| panic!("{name} should render from the log"));
        let fixture = std::fs::read_to_string(format!("{FIXTURE_DIR}/{name}.check.txt"))
            .unwrap_or_else(|e| panic!("{name} check fixture is readable: {e}"));
        let expected = format!("{fixture}schemas: replayed from {}\n", dir.display());
        assert_eq!(view, expected, "{name}: check view drifted");
    }
}

#[test]
fn a_template_name_the_log_does_not_carry_is_unknown() {
    let (state, dir) = template_state("unknown");
    assert!(render::template_view(&state, "does-not-exist", false, &dir).is_err());
    // The shelf data file is a document, never a template: it stays outside the rendered set.
    assert!(render::template_view(&state, "architecture-shelf-backend", false, &dir).is_err());
}

#[test]
fn a_template_still_parses_into_the_typed_model_after_the_re_base() {
    let (state, _dir) = template_state("typed");
    for name in SHIPPED_TEMPLATES {
        let template = render::template_of(&state, name)
            .unwrap_or_else(|_| panic!("{name} should decode as a template"));
        assert_eq!(template.template, name, "{name}: template field");
        assert!(!template.sections.is_empty(), "{name}: sections");
        for section in &template.sections {
            assert!(
                !section.name.trim().is_empty(),
                "{name}: a section has no name"
            );
            assert!(
                !section.check.trim().is_empty(),
                "{name}: section '{}' has no check line",
                section.name
            );
        }
    }
}

#[test]
fn optional_section_fields_still_render_when_present() {
    let (state, dir) = template_state("optional");
    let spec = render::template_view(&state, "spec", false, &dir).unwrap();
    assert!(
        spec.contains("Good example:"),
        "spec renders its good example"
    );
    let entry = render::template_view(&state, "feature-entry", false, &dir).unwrap();
    assert!(
        entry.contains("_Density:_"),
        "feature-entry renders density notes"
    );
}

// ---------------------------------------------------------------------------
// gate-5 schema-data consistency (GI-020: the shipped files stay readable raw)
// ---------------------------------------------------------------------------

/// The shipped `.yaml` files must stay readable as YAML and must still carry every template the
/// log serves. Wave 1 changes nothing under `plugins/`, so this is the same guarantee the crate
/// carried before the re-base — now keyed on the log's own template set rather than a const.
#[test]
fn every_shipped_schema_file_is_readable_yaml_and_every_template_has_one() {
    let entries = std::fs::read_dir(SHIPPED_SCHEMAS_DIR).expect("shipped schema dir should exist");
    let mut seen = Vec::new();
    for entry in entries {
        let path = entry.expect("readable dir entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("yaml") {
            continue;
        }
        let yaml = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{} could not be read: {e}", path.display()));
        serde_norway::from_str::<serde_norway::Value>(&yaml)
            .unwrap_or_else(|e| panic!("{} is not readable as YAML: {e}", path.display()));
        seen.push(
            path.file_stem()
                .expect("yaml file has a stem")
                .to_string_lossy()
                .into_owned(),
        );
    }
    assert!(
        seen.iter().any(|name| name == "architecture-shelf-backend"),
        "the backend shelf data file should ship alongside the rendered schemas"
    );
    for name in SHIPPED_TEMPLATES {
        assert!(
            seen.iter().any(|shipped| shipped == name),
            "{name} is served by the log with no shipped schema file"
        );
    }
}

#[test]
fn every_shipped_template_file_parses_into_the_typed_model() {
    for name in SHIPPED_TEMPLATES {
        let path = Path::new(SHIPPED_SCHEMAS_DIR).join(format!("{name}.yaml"));
        let yaml = std::fs::read_to_string(&path).expect("shipped template is readable");
        let template: schema::Template = serde_norway::from_str(&yaml)
            .unwrap_or_else(|e| panic!("{name} (shipped) failed to parse: {e}"));
        assert_eq!(template.template, name, "{name}: template field");
    }
}
