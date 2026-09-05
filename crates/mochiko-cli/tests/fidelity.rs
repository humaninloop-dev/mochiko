//! Genesis fidelity — every rule of the 50-document corpus survives the log — plus the wave-6
//! end-state assertions over what the whole log replays into.
//!
//! The genesis comparison is deliberately **independent of the generator**: the expected side is
//! the frozen corpus decoded straight into the model, and the actual side is the document sequence
//! 1 alone builds. Only two deltas are allowed, and each is asserted rather than excused — the two
//! comment-carried `enforces: []` reasons, which the model holds as `note:` data, and the
//! provenance anchors, which are checked against the sidecar the test reads for itself.
//!
//! Both sides moved at wave 6. The expected side was the live schema tree, kept equal to the
//! replay by hand under the transition clause; no schema file ships now, so it is the frozen
//! fixture. The actual side was the whole log, which is no longer the same thing as genesis once
//! later migrations carry the corpus forward, so it is genesis replayed alone. The later
//! migrations have their own tests, at the foot of this file and above it.
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
/// comment must become. Verbatim from the frozen corpus's `setup.yaml`.
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
///
/// From wave 6 the repo-side sidecar is frozen to `.mochiko/archive/`, so the copy this suite
/// reads is the one inside the frozen corpus — the file genesis was actually built from, kept
/// beside the corpus it belongs to rather than tracked through an archive path.
fn sidecar() -> BTreeMap<String, String> {
    let path = frozen_corpus().join(genesis::SIDECAR);
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

/// The frozen corpus decoded straight from disk, with no note or anchor applied.
///
/// Through wave 5 this scanned the live tree, which the transition clause kept semantically equal
/// to the replay by hand. The files are gone at wave 6, and the honest expected side is the corpus
/// genesis actually imported — the frozen fixture (record D8).
fn shipped_documents() -> Vec<(DocRef, Document)> {
    genesis::scan(&frozen_corpus())
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
        vec![1, 2, 3, 4],
        "genesis, wave 4's fail-conditions reword, wave 6's two-arm retirement, the sonnet \
         worker rung"
    );
}

// ---------------------------------------------------------------------------
// field-by-field fidelity
// ---------------------------------------------------------------------------

/// The state as sequence 1 alone builds it — genesis replayed with nothing after it.
///
/// The comparison below grades genesis, so its actual side must be genesis and no more. The frozen
/// corpus is the pre-`0002` tree; measuring it against the full replay would grade the later
/// migrations' deliberate rewords as genesis divergences. Every migration after the first has its
/// own test, which is where a change to what it did belongs.
fn genesis_only_state() -> replay::State {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("fidelity-genesis-only");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("the scratch log dir is creatable");
    std::fs::copy(genesis_path(), dir.join(genesis::FILE)).expect("genesis is copyable");
    replay::load(&dir).unwrap_or_else(|findings| {
        let lines: Vec<String> = findings.iter().map(ToString::to_string).collect();
        panic!("genesis alone replays:\n{}", lines.join("\n"))
    })
}

#[test]
fn every_frozen_document_survives_genesis_field_by_field() {
    let state = genesis_only_state();
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
        "{} divergences between the frozen corpus and genesis:\n{}",
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
    // The fold is genesis's claim (record D2: carried, not moved), so the strict two-way check
    // grades sequence 1 alone. A later migration may mint rules carrying anchors of its own —
    // `0004` is the first to — and those are the subject of that migration's own test.
    let anchors = sidecar();
    assert_eq!(anchors.len(), 597, "the sidecar's recorded census");

    let genesis = genesis_only_state();
    let mut anchored = 0usize;
    for document in genesis.docs.values() {
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

    // Through the whole log a folded anchor stays where genesis put it: every rule the sidecar
    // names is still live and still carries that value. Retiring one (`tombstone-rule`,
    // `supersede-rule`) or clearing its anchor is a protected exit that takes a ruling — and a
    // re-key here that names the rule. The walk is driven from the sidecar for that reason: a
    // retired rule never enters the live state, so a walk over live rules would lose it unnamed.
    let live = replay::load(&log_dir()).expect("the log is deliverable");
    let mut live_rules: BTreeMap<&str, &Rule> = BTreeMap::new();
    for document in live.docs.values() {
        let Some(schema) = document.as_rules() else {
            continue;
        };
        for rule in schema.rules() {
            live_rules.insert(rule.id.as_str(), rule);
        }
    }
    for (id, expected) in &anchors {
        let rule = live_rules
            .get(id.as_str())
            .unwrap_or_else(|| panic!("{id}: the sidecar's rule is no longer live"));
        match rule.anchor.as_deref() {
            Some(carried) => assert_eq!(carried, expected, "{id}: the anchor moved"),
            None => panic!("{id}: the sidecar's anchor was cleared"),
        }
    }
}

#[test]
fn the_sidecar_file_is_never_written() {
    // The anchors are carried, not moved (record D2). The sidecar is an input genesis reads and
    // must leave exactly as it found it, whichever tree it is pointed at.
    let path = frozen_corpus().join(genesis::SIDECAR);
    let before = std::fs::read(&path).expect("the sidecar is readable");
    let _ = genesis::build(&frozen_corpus()).expect("genesis builds");
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

    // `0004` (the sonnet worker rung, 2026-09-05) minted six skill rules on
    // `patterns-model-tiering`, two of them floors; the command side is untouched.
    assert_eq!(command_rules, 321, "live command rules");
    assert_eq!(skill_rules, 701, "live skill rules");
    assert_eq!(command_rules + skill_rules, 1022, "live rules in total");
    assert_eq!(skill_floors, 228, "skill floors");
    assert_eq!(command_floors, 110, "declared command floors");
    assert_eq!(fail_nodes, 36, "command fail nodes");
}

// ---------------------------------------------------------------------------
// wave 6 — the two-arm retirement
// ---------------------------------------------------------------------------

/// The phrasings that must not survive `0003`, anywhere in the state.
///
/// Two of them are the two arms of the same construct: the shipped snapshot file was named as a
/// fallback for the binary being absent, or as the second arm when it was unavailable. There is no
/// fallback and there is no file, so both readings are wrong, and both are pinned — a reword that
/// struck one arm and left the other would read as fixed while still sending a run to find a file.
/// The third is the path itself, which nothing may name once nothing ships there.
const RETIRED_PHRASINGS: [&str; 3] = [
    "plugins/mochiko/schemas/",
    "when the binary is absent",
    "when the binary is available",
];

/// No document in the state names a schema file or the absence arm — the whole state, not just
/// rule text.
///
/// Widened past "no rule text" deliberately. A `vars:` entry, a section intent, a condition note, a
/// template's own prose and the shelf data all reach a reader, and any of them naming a file the
/// plugin does not ship is the same defect wherever it sits. Serialising each document through the
/// view writer is what makes the sweep exhaustive without enumerating fields: it walks every string
/// the model holds, so a field added later is covered the day it is added.
#[test]
fn no_document_in_the_state_names_a_schema_file_or_the_absence_arm() {
    let state = replay::load(&log_dir()).expect("the log is deliverable");
    let mut offences: Vec<String> = Vec::new();
    for (doc, document) in &state.docs {
        let text = mochiko_cli::views::to_yaml(&document.to_value());
        for needle in RETIRED_PHRASINGS {
            for line in text.lines().filter(|line| line.contains(needle)) {
                offences.push(format!("{doc}: {}", line.trim()));
            }
        }
    }
    assert!(
        offences.is_empty(),
        "{} lines still name a retired schema path or the absence arm:\n{}",
        offences.len(),
        offences.join("\n")
    );
}

/// The `0003` rewords landed on the rules they name, and the CLI form is what a reader now sees.
///
/// The sweep above proves the old phrasing is gone; on its own it would also pass if a rule had
/// been emptied rather than reworded. This one names each rule and the delivery it must carry.
#[test]
fn the_third_migration_left_every_reworded_rule_naming_its_cli_form() {
    let state = replay::load(&log_dir()).expect("the log is deliverable");
    let cases: [(DocKind, &str, &str, &str); 14] = [
        (
            DocKind::Command,
            "architecture",
            "arch.tools-store-schema",
            "${shelf_schema}",
        ),
        (
            DocKind::Command,
            "feature",
            "feat.delta-cards",
            "${tasks_schema}",
        ),
        (
            DocKind::Command,
            "implement",
            "impl.cards-template",
            "${tasks_schema}",
        ),
        (
            DocKind::Command,
            "setup",
            "setup.synthesis-artifact",
            "mochiko-cli template governance-intent",
        ),
        (
            DocKind::Command,
            "setup",
            "setup.feature-map-brownfield",
            "mochiko-cli template feature-entry",
        ),
        (
            DocKind::Command,
            "specify",
            "spec.deliverable",
            "${spec_schema}",
        ),
        (
            DocKind::Command,
            "specify",
            "spec.feature-map-craft",
            "${features_index_schema}",
        ),
        (
            DocKind::SkillCommon,
            "skill-authoring-common",
            "authoring-common.two-arm-template",
            "mochiko-cli template ${template}",
        ),
        (
            DocKind::Skill,
            "analysis-codebase",
            "analysis-codebase.deliverable-two-arm-binding",
            "mochiko-cli template codebase-analysis",
        ),
        (
            DocKind::Skill,
            "authoring-feature-map",
            "authoring-feature-map.feature-entry-two-arm",
            "mochiko-cli template feature-entry",
        ),
        (
            DocKind::Skill,
            "authoring-technical-requirements",
            "authoring-technical-requirements.nfr-store-home",
            "mochiko-cli template architecture-store",
        ),
        (
            DocKind::Skill,
            "patterns-architecture-shelves",
            "patterns-architecture-shelves.opinions-in-data",
            "mochiko-cli doc architecture-shelf-backend",
        ),
        (
            DocKind::Skill,
            "patterns-vertical-tdd",
            "patterns-vertical-tdd.tasks-binding-two-arm",
            "mochiko-cli template tasks",
        ),
        (
            DocKind::Skill,
            "review-plan-artifacts",
            "review-plan-artifacts.cycle-card-check-mirror",
            "mochiko-cli template tasks --check",
        ),
    ];

    for (kind, name, id, expected) in cases {
        let doc = DocRef::new(kind, name);
        let schema = state
            .docs
            .get(&doc)
            .and_then(Document::as_rules)
            .unwrap_or_else(|| panic!("{doc} is in state"));
        let rule = schema
            .find_rule(id)
            .unwrap_or_else(|| panic!("{id} survived its reword as a live rule"));
        let text = rule.text.as_deref().unwrap_or_default();
        assert!(
            text.contains(expected),
            "{id}: the reworded text does not name {expected:?}:\n{text}"
        );
    }
}

/// The six vars that named a schema file now name the command that delivers it.
///
/// Each is still referenced by a rule, so one left pointing at a path would substitute a dead file
/// path into a delivered render. The sweep above would catch that by its path text; this names the
/// values the rewords were written against.
#[test]
fn the_delivery_vars_name_a_cli_invocation() {
    let state = replay::load(&log_dir()).expect("the log is deliverable");
    let cases: [(&str, &str, &str); 7] = [
        ("implement", "tasks_schema", "mochiko-cli template tasks"),
        ("feature", "tasks_schema", "mochiko-cli template tasks"),
        ("specify", "spec_schema", "mochiko-cli template spec"),
        (
            "specify",
            "feature_entry_schema",
            "mochiko-cli template feature-entry",
        ),
        (
            "specify",
            "features_index_schema",
            "mochiko-cli template features-index",
        ),
        (
            "architecture",
            "store_schema",
            "mochiko-cli template architecture-store",
        ),
        (
            "architecture",
            "shelf_schema",
            "mochiko-cli doc architecture-shelf-backend",
        ),
    ];
    for (command, var, expected) in cases {
        let doc = DocRef::new(DocKind::Command, command);
        let schema = state
            .docs
            .get(&doc)
            .and_then(Document::as_rules)
            .unwrap_or_else(|| panic!("{doc} is in state"));
        let value = mochiko_cli::model::ordered_get(&schema.vars, var)
            .unwrap_or_else(|| panic!("{command}: `{var}` is still bound"));
        assert_eq!(
            value.as_str(),
            Some(expected),
            "{command}: `{var}` does not name its delivery command"
        );
    }
}

/// No rule points at a schema file.
///
/// A `pointer:` is resolved against the installed tree, so one left aiming at a deleted file is a
/// rejecting `pointer-unresolved` finding rather than cosmetic staleness. The three that did are
/// named, so clearing the wrong one fails here rather than passing the sweep below by accident.
#[test]
fn no_rule_points_at_a_schema_file() {
    let state = replay::load(&log_dir()).expect("the log is deliverable");
    let cleared = [
        (
            "analysis-codebase",
            "analysis-codebase.deliverable-two-arm-binding",
        ),
        (
            "patterns-architecture-shelves",
            "patterns-architecture-shelves.opinions-in-data",
        ),
        (
            "patterns-vertical-tdd",
            "patterns-vertical-tdd.tasks-binding-two-arm",
        ),
    ];
    for (skill, id) in cleared {
        let doc = DocRef::new(DocKind::Skill, skill);
        let rule = state
            .docs
            .get(&doc)
            .and_then(Document::as_rules)
            .and_then(|schema| schema.find_rule(id))
            .unwrap_or_else(|| panic!("{id} is a live rule"));
        assert!(
            rule.pointer.is_none(),
            "{id} still carries `pointer: {}`",
            rule.pointer.as_deref().unwrap_or_default()
        );
    }

    for (doc, document) in &state.docs {
        let Some(schema) = document.as_rules() else {
            continue;
        };
        for rule in schema.rules() {
            let Some(pointer) = rule.pointer.as_deref() else {
                continue;
            };
            assert!(
                !pointer.contains("schemas/"),
                "{doc} · {}: `pointer: {pointer}` names a schema directory",
                rule.id
            );
        }
    }
}

// ---------------------------------------------------------------------------
// 0004 — the sonnet worker rung
// ---------------------------------------------------------------------------

/// The `0004` migration (`2026-09-05 sonnet-worker-rung`) landed where it said it would: six
/// minted rules on `patterns-model-tiering`, two of them floors, each carrying the ruling anchor;
/// the two reworded floors still floors, naming both rungs and the D5 clause; the floor pin at
/// six; the reserved section's note naming its new reservation.
#[test]
fn the_fourth_migration_added_the_worker_rung_to_the_tiering_floor() {
    const ANCHOR: &str = "2026-09-05 sonnet-worker-rung";
    let state = replay::load(&log_dir()).expect("the log is deliverable");
    let doc = DocRef::new(DocKind::Skill, "patterns-model-tiering");
    let schema = state
        .docs
        .get(&doc)
        .and_then(Document::as_rules)
        .expect("patterns-model-tiering is in state");

    let minted: [(&str, &str, bool); 6] = [
        (
            "patterns-model-tiering.class-key-worker-tier",
            "patterns-model-tiering.sec.trigger",
            false,
        ),
        (
            "patterns-model-tiering.worker-rung-sonnet",
            "patterns-model-tiering.sec.discipline",
            false,
        ),
        (
            "patterns-model-tiering.worker-return-is-a-claim",
            "patterns-model-tiering.sec.inputs",
            true,
        ),
        (
            "patterns-model-tiering.worker-brief",
            "patterns-model-tiering.sec.disclosure",
            false,
        ),
        (
            "patterns-model-tiering.worker-disclosure",
            "patterns-model-tiering.sec.disclosure",
            false,
        ),
        (
            "patterns-model-tiering.worker-seat-set-reserved",
            "patterns-model-tiering.sec.reserved",
            true,
        ),
    ];
    for (id, section, floor) in minted {
        let home = schema
            .find_section(section)
            .unwrap_or_else(|| panic!("{section} is a live section"));
        let rule = home
            .rules
            .iter()
            .find(|rule| rule.id == id)
            .unwrap_or_else(|| panic!("{id} was minted into {section}"));
        assert_eq!(
            rule.anchor.as_deref(),
            Some(ANCHOR),
            "{id}: the ruling anchor"
        );
        assert_eq!(rule.is_floor(), floor, "{id}: floor class");
    }

    // The reworded floors kept their ids and their class, and say what the ruling says.
    let reworded: [(&str, &[&str]); 2] = [
        (
            "patterns-model-tiering.class-key-session-tier",
            &[
                "never tiered down",
                "sonnet-worker-rung",
                "model-tiered-seats D5",
            ],
        ),
        (
            "patterns-model-tiering.override-is-the-pin",
            &["`model: haiku`", "`model: sonnet`"],
        ),
    ];
    for (id, needles) in reworded {
        let rule = schema
            .find_rule(id)
            .unwrap_or_else(|| panic!("{id} is a live rule"));
        assert!(rule.is_floor(), "{id}: still a floor");
        let text = rule.text.as_deref().unwrap_or_default();
        for needle in needles {
            assert!(text.contains(needle), "{id}: the reword names {needle:?}");
        }
    }

    let floors = schema.rules().filter(|rule| rule.is_floor()).count();
    assert_eq!(floors, 6, "the skill's floor pin");

    let reserved = schema
        .find_section("patterns-model-tiering.sec.reserved")
        .expect("the reserved section is live");
    assert!(
        reserved
            .note
            .as_deref()
            .is_some_and(|note| note.contains("worker-seat-set-reserved")),
        "the reserved note names the new reservation"
    );
}
