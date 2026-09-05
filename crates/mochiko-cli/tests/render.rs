//! Integration tests for the rules render and the re-based template views.
//!
//! Every fixture log is written under `CARGO_TARGET_TMPDIR`, which Cargo places inside `target/` —
//! the suite never writes outside the build directory. The render contract is exercised against a
//! synthetic corpus built here, shaped to reach limbs the real one does not, and the size and
//! template assertions replay the committed log; `tests/cli.rs` carries the end-to-end run against
//! `plugins/mochiko/migrations/` through the binary.

use mochiko_cli::model::{DocKind, DocRef};
use mochiko_cli::render::{self, Context, PREAMBLE};
use mochiko_cli::replay;
use mochiko_cli::schema;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Absolute path to the committed migration log, anchored at the crate root so it is independent
/// of the test process's working directory.
///
/// From wave 6 this is the only source of schema content there is: no schema file ships, so a
/// test that wants the real corpus replays the log for it.
const SHIPPED_LOG_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../plugins/mochiko/migrations"
);

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

/// The committed log, replayed.
///
/// Wave 1 built a template-only fixture log from the shipped schema files, because genesis did not
/// exist yet. It does, the files do not, and the templates the views are graded against are the
/// ones the log actually carries — so this reads the real thing.
fn shipped_log_state() -> replay::State {
    replay::load(Path::new(SHIPPED_LOG_DIR)).unwrap_or_else(|findings| {
        let lines: Vec<String> = findings.iter().map(ToString::to_string).collect();
        panic!("the committed log is deliverable:\n{}", lines.join("\n"))
    })
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
    // The block's own shape, not the bare word: from wave 4 the legend carries a `moments:` line
    // in prose, and the legend is delivered to skills too. Same narrowing as the fail pin below.
    assert!(
        !out.contains("\nmoments\n"),
        "skills declare no moments block:\n{out}"
    );
    // The pin's own shape, not the bare words: the legend's `enforces:` line names `kind: fail`
    // in prose, and that sentence is delivered to skills too.
    assert!(
        !out.contains("- kind: fail · "),
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

// ---------------------------------------------------------------------------
// the floor index (wave 5 §2)
// ---------------------------------------------------------------------------

/// The `floors:` line of a preamble render, whole.
fn floors_line(render: &str) -> &str {
    render
        .lines()
        .find(|line| line.starts_with("floors: "))
        .unwrap_or_else(|| panic!("the preamble carries no floors line:\n{render}"))
}

/// The ids the floor index names, in the order it names them. `floors: none` is the empty set.
fn floor_ids(render: &str) -> Vec<&str> {
    match floors_line(render).trim_start_matches("floors: ") {
        "none" => Vec::new(),
        list => list.split(" · ").collect(),
    }
}

/// The number the `class: floor` pin prints, which the index must agree with.
fn floor_pin(render: &str) -> usize {
    let line = render
        .lines()
        .find(|line| line.starts_with("- class: floor · "))
        .unwrap_or_else(|| panic!("the preamble carries no floor pin:\n{render}"));
    line.trim_start_matches("- class: floor · ")
        .trim_end_matches(" rules")
        .parse()
        .unwrap_or_else(|_| panic!("the floor pin is not a count: {line:?}"))
}

/// Render order is sections as declared, rules in section order — so the three fixture floors,
/// which sit in three different sections, are the case that can tell that order from any other.
#[test]
fn the_preamble_indexes_every_floor_id_in_render_order() {
    let state = rules_state("floors");
    let out = render::preamble(&state, &command(), &ctx()).unwrap();
    assert_eq!(
        floors_line(&out),
        "floors: demo.stub · demo.boundary · demo.fail.unaccepted"
    );
    assert_eq!(floor_ids(&out).len(), floor_pin(&out), "index against pin");
}

#[test]
fn a_skill_preamble_indexes_its_floor_ids_too() {
    let state = rules_state("skillfloors");
    let out = render::preamble(&state, &skill(), &ctx()).unwrap();
    assert_eq!(
        floors_line(&out),
        "floors: review-demo.not-the-author · review-demo.user-rules"
    );
    assert_eq!(floor_ids(&out).len(), floor_pin(&out), "index against pin");
}

/// The line the converted `.md`'s read-back sentence names sits with the pin it restates, and it
/// is not a rule: the preamble still counts zero.
#[test]
fn the_floor_index_sits_between_the_pins_and_the_legend() {
    let state = rules_state("floorsplace");
    let out = render::preamble(&state, &command(), &ctx()).unwrap();
    let pins_at = out.find("\npins\n").expect("the pins block is present");
    let floors_at = out.find("\nfloors: ").expect("the floor index is present");
    let legend_at = out.find("\nlegend\n").expect("the legend block is present");
    assert!(
        pins_at < floors_at && floors_at < legend_at,
        "the floor index sits between the pins and the legend:\n{out}"
    );
    assert_eq!(
        out.trim_end().lines().last().unwrap(),
        "mochiko-cli rules end · demo · preamble · 0 rules",
        "the floor index is not a rule:\n{out}"
    );
}

/// No shipped primitive carries an empty floor set — the corpus runs from 2 floors to 34 — so the
/// `none` branch can only be reached through a fixture. Imported as a second migration rather
/// than folded into `RULES_LOG`, which some thirty tests read.
#[test]
fn a_primitive_with_no_floor_rule_indexes_none() {
    let dir = log_dir("floorless");
    write_migration(&dir, "0001-genesis.yaml", RULES_LOG);
    write_migration(
        &dir,
        "0002-floorless.yaml",
        r#"
grammar: 1
id: 0002-floorless
sequence: 2
intent: Import a review skill carrying no floor rule, a shape the shipped corpus has nowhere.
changes:
  - op: import-document
    kind: skill
    name: review-floorless
    content:
      kind: skill
      skill: review-floorless
      sections:
        - id: review-floorless.sec.independence
          title: Independence
          intent: Who may run this skill.
          rules:
            - id: review-floorless.not-the-author
              labels: [independence]
              class: must
              text: The author never grades their own artifact.
        - id: review-floorless.sec.scope
          title: Scope
          intent: What is graded.
          rules:
            - id: review-floorless.scope-fence
              labels: [scope]
              class: must
              text: Grade the artifact, never the author.
        - id: review-floorless.sec.inputs
          title: Inputs
          intent: What the skill reads.
          note: The skill reads the artifact and nothing else.
          rules: []
        - id: review-floorless.sec.verdict
          title: Verdict
          intent: The verdict grammar.
          rules:
            - id: review-floorless.inherited-verdict
              extends: review-common.shared-verdict
              class: must
        - id: review-floorless.sec.output
          title: Output
          intent: The report shape.
          rules:
            - id: review-floorless.report-shape
              labels: [scope]
              class: must
              text: The report names every finding.
        - id: review-floorless.sec.reserved
          title: Reserved
          intent: Reserved to the user.
          rules:
            - id: review-floorless.user-rules
              labels: [independence]
              class: must
              text: The user rules the verdict's consequence.
"#,
    );
    let state = replay::load(&dir).unwrap_or_else(|findings| {
        let lines: Vec<String> = findings.iter().map(ToString::to_string).collect();
        panic!(
            "the floorless corpus should be deliverable:\n{}",
            lines.join("\n")
        )
    });

    let doc = DocRef::new(DocKind::Skill, "review-floorless");
    let out = render::preamble(&state, &doc, &ctx()).unwrap();
    assert!(
        out.contains("- class: floor · 0 rules"),
        "the pin counts no floors:\n{out}"
    );
    assert_eq!(floors_line(&out), "floors: none");
    assert!(floor_ids(&out).is_empty(), "the index is empty:\n{out}");
}

/// The reading grammar the converted command's `.md` no longer restates, verbatim from the wave-3
/// plan §2 and widened at wave 4 with the three things P2 found the old Rules block taught and
/// the legend did not. A golden test rather than a shape assertion: the `.md` points at this
/// block by name, so a silent reword there is a silent change to what every converted primitive
/// is told.
const COMMAND_LEGEND: &str = "\nlegend\n\
- class: floor is always delivered whatever its when:; when: gates when the obligation applies, never whether it reaches you.\n\
- kind: names what a rule is — constraint (the default) · duty · gate · reservation · binding · bound · routing · fail · latitude.\n\
- when: binds a rule only where its terms hold against the conditions block above.\n\
- enforces: on a kind: fail rule names the rules it is the end-state contrapositive of.\n\
- pointer: binds you to that skill's procedure — referenced, never restated.\n\
- extends: is already resolved in this render; the rule's own id stays the citable id.\n\
- labels: cross-reference tags from the labels registry; they bind nothing on their own.\n\
- moments: the run's anchor points, unordered — never a sequence.\n\
- enforces: an empty list on a kind: fail rule carries its one-line reason.\n";

/// The skill variant (wave 6, plan §2.2 as ruled at the wave open).
///
/// Three lines of the command legend describe grammar a skill schema cannot carry: `kind: fail`
/// and its two `enforces:` lines are illegal in a skill schema, and skills declare no `moments:`.
/// Wave 4 delivered all of them anyway on the argument that one shared legend was cheaper than
/// two; the moment a second legend exists that argument is spent, so the skill legend states only
/// the grammar a skill schema can actually meet.
const SKILL_LEGEND: &str = "\nlegend\n\
- class: floor is always delivered whatever its when:; when: gates when the obligation applies, never whether it reaches you.\n\
- kind: names what a rule is — constraint (the default) · duty · gate · reservation · binding · bound · routing · latitude.\n\
- when: binds a rule only where its terms hold against the conditions block above.\n\
- pointer: binds you to that skill's procedure — referenced, never restated.\n\
- extends: is already resolved in this render; the rule's own id stays the citable id.\n\
- labels: cross-reference tags from the labels registry; they bind nothing on their own.\n";

/// Each legend's own size, pinned because it is a render-shape change a plugin bump names.
///
/// Every converted primitive pays one of them on every `preamble` render, so neither is free and
/// neither is allowed to grow unnoticed. Two pins from wave 6, one per variant.
#[test]
fn the_legend_blocks_are_the_sizes_the_waves_recorded() {
    assert_eq!(
        COMMAND_LEGEND.len(),
        845,
        "the command legend's byte size moved"
    );
    assert_eq!(
        COMMAND_LEGEND
            .lines()
            .filter(|l| l.starts_with("- "))
            .count(),
        9,
        "six original grammar lines plus wave 4's three"
    );
    assert_eq!(
        SKILL_LEGEND.len(),
        605,
        "the skill legend's byte size moved"
    );
    assert_eq!(
        SKILL_LEGEND.lines().filter(|l| l.starts_with("- ")).count(),
        6,
        "the command's nine less the two enforces lines and the moments line"
    );
}

#[test]
fn the_preamble_carries_the_fixed_legend_block() {
    let state = rules_state("legend");
    let out = render::preamble(&state, &command(), &ctx()).unwrap();

    assert!(
        out.contains(COMMAND_LEGEND),
        "the legend block, verbatim:\n{out}"
    );

    let legend_at = out.find("\nlegend\n").expect("the legend block is present");
    let pins_at = out.find("\npins\n").expect("the pins block is present");
    let sections_at = out
        .find("\nsections\n")
        .expect("the sections block is present");
    assert!(
        pins_at < legend_at && legend_at < sections_at,
        "the legend sits between pins and sections:\n{out}"
    );

    assert_eq!(
        out.trim_end().lines().last().unwrap(),
        "mochiko-cli rules end · demo · preamble · 0 rules",
        "the legend is not a rule and the preamble still counts zero:\n{out}"
    );
}

#[test]
fn a_skill_preamble_carries_the_skill_legend() {
    let state = rules_state("skilllegend");
    let out = render::preamble(&state, &skill(), &ctx()).unwrap();
    assert!(
        out.contains(SKILL_LEGEND),
        "the skill legend block, verbatim:\n{out}"
    );
    assert!(
        !out.contains(COMMAND_LEGEND),
        "a skill preamble carries the command legend:\n{out}"
    );
}

/// The three lines the skill variant exists to drop, keyed on their own text.
///
/// `kind: fail` and `enforces:` are illegal in a skill schema and `moments:` is a command-only
/// block, so a skill preamble teaching any of them is teaching grammar its reader cannot use.
#[test]
fn a_skill_legend_omits_the_grammar_a_skill_schema_cannot_carry() {
    let state = rules_state("skilllegendomits");
    let out = render::preamble(&state, &skill(), &ctx()).unwrap();
    for absent in [
        "- enforces: on a kind: fail rule",
        "- enforces: an empty list",
        "- moments: the run's anchor points",
        "· routing · fail · latitude",
    ] {
        assert!(
            !out.contains(absent),
            "the skill legend still carries {absent:?}:\n{out}"
        );
    }
    assert!(
        out.contains("· routing · latitude."),
        "the skill legend's kind line should close on latitude:\n{out}"
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

/// The D6 empty-with-reason mirror: `enforces: []` is legal beside a `note:`, and the note is
/// maintainer metadata the Q4 ruling keeps out of a render. A bare `enforces:` key would therefore
/// carry nothing at all, so the line is omitted outright — the two shipped `setup.yaml` fail nodes
/// are the live case once genesis lands.
#[test]
fn an_explicitly_empty_enforces_mirror_renders_no_key_at_all() {
    let dir = log_dir("emptyenforces");
    write_migration(&dir, "0001-genesis.yaml", RULES_LOG);
    write_migration(
        &dir,
        "0002-empty-mirror.yaml",
        r#"
grammar: 1
id: 0002-empty-mirror
sequence: 2
intent: Empty the fail node's mirror and record why, the way setup.yaml carries it.
changes:
  - op: set-rule-field
    schema: command/demo
    id: demo.fail.unaccepted
    field: enforces
    value: []
  - op: set-rule-field
    schema: command/demo
    id: demo.fail.unaccepted
    field: note
    value: The obligation this mirrors is the user's, and lives outside the schema.
"#,
    );
    let state = replay::load(&dir).unwrap_or_else(|findings| {
        let lines: Vec<String> = findings.iter().map(ToString::to_string).collect();
        panic!(
            "the empty-mirror corpus should be deliverable:\n{}",
            lines.join("\n")
        )
    });

    let out = render::section(&state, &command(), "demo.sec.fail-conditions", &ctx()).unwrap();
    assert!(
        out.contains("### demo.fail.unaccepted"),
        "the fail node still renders:\n{out}"
    );
    assert!(
        !out.contains("enforces"),
        "an empty mirror renders no key, not a dangling one:\n{out}"
    );
    assert!(
        !out.contains("lives outside the schema"),
        "the note stays maintainer metadata (Q4):\n{out}"
    );
    // Nothing dangling anywhere: no line is a key with an empty value.
    for line in out.lines() {
        assert!(
            !line.trim_end().ends_with(':') || line.starts_with("##"),
            "dangling key line {line:?} in:\n{out}"
        );
    }
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

/// The whole shipped corpus as state — the committed log, replayed.
///
/// Through wave 5 this walked the shipped schema files and addressed each the way genesis would.
/// No schema file ships from wave 6, so the corpus comes from the only place it lives.
fn shipped_state() -> replay::State {
    shipped_log_state()
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
// the floor index against the shipped corpus
// ---------------------------------------------------------------------------

/// `implement`'s floor set, the corpus's largest, in render order. Written out rather than
/// derived: a floor rule added, renamed or reordered should break a test here, because the
/// converted `.md`'s read-back and the contract suite's frozen expectations both key on this set.
const IMPLEMENT_FLOORS: [&str; 34] = [
    "impl.gate-design-checkpoint",
    "impl.gate-card-confirm",
    "impl.gate-final-acceptance",
    "impl.graded-fold",
    "impl.author-grader-default-fail",
    "impl.baselines-never-in-place",
    "impl.deviation-gate",
    "impl.constitution-supremacy",
    "impl.constraint-challenge",
    "impl.attempt-per-grade",
    "impl.attempt-exemption-user-only",
    "impl.no-progress-stop",
    "impl.epic-member-halt",
    "impl.gap-rework-bound",
    "impl.gates-never-triaged",
    "impl.minimalism-advisory",
    "impl.lane-never-widens",
    "impl.sound-loop-floor",
    "impl.transport-floor",
    "impl.fail.sufficiency-unrecorded",
    "impl.fail.design-skipped",
    "impl.fail.card-independence",
    "impl.fail.card-unchecked",
    "impl.fail.quality-gate",
    "impl.fail.no-evidence",
    "impl.fail.regression",
    "impl.fail.baseline-in-place",
    "impl.fail.deviation-unresolved",
    "impl.fail.store-landing-incomplete",
    "impl.fail.ungraded-fold",
    "impl.fail.gap-finding-missing",
    "impl.fail.skip-unstated",
    "impl.fail.spec-gap-unresolved",
    "impl.fail.no-acceptance",
];

/// A skill's floor set, the review family's referent, in render order.
const REVIEW_BRAINSTORM_FLOORS: [&str; 9] = [
    "review-brainstorm.never-in-the-room",
    "review-brainstorm.blind-map-before-record-contact",
    "review-brainstorm.author-grader",
    "review-brainstorm.contested-needs-new-angle",
    "review-brainstorm.never-default-ready",
    "review-brainstorm.unverifiable-claim-is-finding",
    "review-brainstorm.evidence-floor",
    "review-brainstorm.verdict-is-input",
    "review-brainstorm.findings-through-leads-pen",
];

#[test]
fn the_shipped_floor_index_carries_the_recorded_sets() {
    let state = shipped_state();
    for (doc, expected) in [
        (
            DocRef::new(DocKind::Command, "implement"),
            IMPLEMENT_FLOORS.as_slice(),
        ),
        (
            DocRef::new(DocKind::Skill, "review-brainstorm"),
            REVIEW_BRAINSTORM_FLOORS.as_slice(),
        ),
    ] {
        let out = render::preamble(&state, &doc, &ctx())
            .unwrap_or_else(|e| panic!("{} renders its preamble: {e}", doc.name));
        assert_eq!(floor_ids(&out), expected, "{}: the floor index", doc.name);
    }
}

/// The property every converted `.md`'s read-back leans on: the count the pin prints and the ids
/// the index lists are the same set, everywhere in the corpus. Both come from one iterator in the
/// render, so this test guards the code that keeps them together.
#[test]
fn every_shipped_floor_index_matches_its_pin() {
    let state = shipped_state();
    let mut checked = 0usize;
    for (doc, document) in &state.docs {
        if !matches!(doc.kind, DocKind::Command | DocKind::Skill) {
            continue;
        }
        let _ = document.as_rules().expect("a rule-bearing document");
        let out = render::preamble(&state, doc, &ctx())
            .unwrap_or_else(|e| panic!("{} renders its preamble: {e}", doc.name));
        assert_eq!(
            floor_ids(&out).len(),
            floor_pin(&out),
            "{}: the index and the pin disagree:\n{out}",
            doc.name
        );
        checked += 1;
    }
    assert_eq!(checked, 36, "six commands and thirty skills");
}

/// The index's own size at its widest, pinned the way the legend's is: every converted primitive
/// pays it on every preamble render, and it is a render-shape change a plugin bump names.
#[test]
fn the_widest_shipped_floor_index_is_the_size_the_wave_recorded() {
    let state = shipped_state();
    let mut widest = (String::new(), 0usize, 0usize);
    for (doc, _) in state
        .docs
        .iter()
        .filter(|(doc, _)| matches!(doc.kind, DocKind::Command | DocKind::Skill))
    {
        let out = render::preamble(&state, doc, &ctx())
            .unwrap_or_else(|e| panic!("{} renders its preamble: {e}", doc.name));
        let line = floors_line(&out);
        if line.chars().count() > widest.1 {
            widest = (doc.name.clone(), line.chars().count(), line.len());
        }
    }
    assert_eq!(
        (widest.0.as_str(), widest.1, widest.2),
        ("implement", 945, 978),
        "the widest floor index moved"
    );
}

// ---------------------------------------------------------------------------
// the re-based template views
// ---------------------------------------------------------------------------

fn template_state() -> (mochiko_cli::replay::State, PathBuf) {
    (shipped_log_state(), PathBuf::from(SHIPPED_LOG_DIR))
}

#[test]
fn every_template_producer_view_is_byte_identical_to_its_captured_fixture() {
    let (state, dir) = template_state();
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
    let (state, dir) = template_state();
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
    let (state, dir) = template_state();
    assert!(render::template_view(&state, "does-not-exist", false, &dir).is_err());
    // The shelf data file is a document, never a template: it stays outside the rendered set.
    assert!(render::template_view(&state, "architecture-shelf-backend", false, &dir).is_err());
}

#[test]
fn a_template_still_parses_into_the_typed_model_after_the_re_base() {
    let (state, _dir) = template_state();
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
    let (state, dir) = template_state();
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
// gate-5 schema-data consistency (GI-020: the log carries everything, and ships nothing)
// ---------------------------------------------------------------------------

/// The log carries every template the crate names, and the shelf document beside them.
///
/// Through wave 5 this read the shipped `.yaml` files and asserted each was parseable and present.
/// The files are retired at wave 6, so the claim is keyed on the only place the content lives. Its
/// other half — that nothing ships where those files used to be — is the test below.
#[test]
fn the_log_carries_every_template_and_the_shelf() {
    let state = shipped_log_state();
    for name in SHIPPED_TEMPLATES {
        let doc = DocRef::new(DocKind::Template, name);
        assert!(
            state.docs.contains_key(&doc),
            "{name} is named by the crate but carried by no document in the log"
        );
    }
    assert!(
        state
            .docs
            .keys()
            .any(|doc| doc.kind == DocKind::Shelf && doc.name == "architecture-shelf-backend"),
        "the backend shelf is carried by the log"
    );
}

/// No schema file ships (GI-020, record D9 wave 6).
///
/// The paired half of the test above: the content survives, and the files it used to be read from
/// are gone. Both directions matter and they fail apart — a deletion that also lost the content
/// trips the first, and content kept beside a surviving fallback file trips this one. A fallback a
/// primitive could Read is the whole thing the dependency forbids, so its absence is asserted on
/// the tree rather than inferred from the log.
///
/// The two shapes are named separately because they were retired for different reasons: the flat
/// `schemas/` directory carried the command schemas, the family commons, the registries, the
/// templates and the shelf, while `skills/<name>/schema.yaml` was the in-directory skill form added
/// at v0.100.0. The plugin contract suite asserts the running half — that no primitive reads such a
/// file during a run — and this asserts there is no such file to read.
///
/// **The claim is about schema files, not about YAML.** A skill's `references/` may carry YAML that
/// is reference content — `patterns-api-contracts` ships an OpenAPI template — and widening this
/// into "no `.yaml` under `plugins/`" would delete a legitimate reference file to satisfy a test.
/// The last assertion pins that survivor, so the widening fails here rather than in review.
#[test]
fn no_schema_file_ships_in_the_plugin() {
    let plugin = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../plugins/mochiko"
    ));

    assert!(
        !plugin.join("schemas").exists(),
        "plugins/mochiko/schemas/ still exists — no schema file ships from wave 6"
    );

    let stragglers: Vec<String> = std::fs::read_dir(plugin.join("skills"))
        .expect("the skills directory exists")
        .filter_map(Result::ok)
        .map(|entry| entry.path().join("schema.yaml"))
        .filter(|path| path.is_file())
        .map(|path| path.display().to_string())
        .collect();
    assert!(
        stragglers.is_empty(),
        "in-directory skill schemas still ship:\n{}",
        stragglers.join("\n")
    );

    assert!(
        plugin
            .join("skills/patterns-api-contracts/references/OPENAPI-TEMPLATE.yaml")
            .is_file(),
        "reference YAML is not schema data and is exempt from the two assertions above — \
         this one is missing, so either it was deleted to satisfy them or the skill moved"
    );
}

#[test]
fn every_template_the_log_carries_parses_into_the_typed_model() {
    let state = shipped_log_state();
    for name in SHIPPED_TEMPLATES {
        let template: schema::Template = render::template_of(&state, name)
            .unwrap_or_else(|e| panic!("{name} failed to decode from the log: {e}"));
        assert_eq!(template.template, name, "{name}: template field");
    }
}
