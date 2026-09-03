//! The ported probe matrix for `scripts/check-command-schema.py` — 134 probes.
//!
//! The Python matrix writes four files into a temp directory and runs the checker as a
//! subprocess. Here the same fixture is built as **state** and graded by the validator the log
//! actually uses, because that is what replaces the checker: after D6 there is no `.md` scaffold
//! to check and no file to parse at check time — there is a store, and the store's own validity.
//!
//! Every one of the 134 probes is accounted for in exactly one of four places:
//!
//! * [`PROBES`] — ported, each asserting the finding the Python named.
//! * [`GENESIS_SIDE`] — ported into `tests/fidelity.rs`, because the surface moved to the
//!   generator (the provenance sidecar, and the file-level read and parse failures).
//! * [`NOT_APPLICABLE`] — the surface itself is gone under D6, with the reason.
//! * [`OUTSIDE_THE_HARD_SET`] — the Python carries a check the Rust hard set does not. These are
//!   the honest cost of the retirement, listed so the wave can rule on each rather than discover
//!   it later.
//!
//! `the_whole_python_matrix_is_accounted_for` asserts the four add up to 134.

mod matrix;

use matrix::{Expect, Fixture, Probe};
use mochiko_cli::model::{Class, Rule, Section, WhenValue};
use mochiko_cli::validate::Code;
use serde_norway::Value;

// ---------------------------------------------------------------------------
// the fixture
// ---------------------------------------------------------------------------

/// The synthetic `demo` command, its label registry and its block library — the Python's
/// `baseline_schema()`, `BASELINE_LABELS` and `BASELINE_COMMON`, in the model's own shape.
fn baseline() -> Fixture {
    Fixture::for_command(DEMO, LABELS, COMMON)
}

const DEMO: &str = "\
kind: command
command: demo
vars:
  target: plugins/mochiko/schemas/demo.yaml
conditions:
  mode:
    values: [deep, shallow]
    resolution: moment-resolved(open)
    note: ruled at the open; rules gated on it wait until it resolves.
  demo_map:
    values: presence
    resolution: surface-presence
    note: the demo map file.
  seats:
    values: [single, multi]
    resolution: standing-trigger
    note: fires the moment the run composes more than one seat.
moments:
  open: Where the demonstration's mode is ruled.
  close: Where the lead states the verdict.
sections:
  - id: demo.sec.roles
    title: Roles
    intent: the roles the run is bound by
    rules:
      - id: demo.lead-owns
        labels: [role]
        class: must
        kind: duty
        text: The lead owns the run, never produces, and states the verdict at close.
  - id: demo.sec.reserved
    title: Reserved
    intent: the reserved the run is bound by
    note: no reserved rulings yet — deliberately empty, not an omission
    rules: []
  - id: demo.sec.tools
    title: Tools
    intent: the tools the run is bound by
    rules:
      - id: demo.read-first
        labels: [binding]
        class: must
        kind: binding
        text: Read ${target} in full before the first action.
      - id: demo.register
        extends: common.register
        class: must
        kind: binding
  - id: demo.sec.ways-of-working
    title: Ways Of Working
    intent: the ways-of-working the run is bound by
    rules:
      - id: demo.one-question
        labels: [binding]
        class: must
        text: Ask one question at a time.
      - id: demo.deep-probe
        labels: [binding]
        class: must
        when: {mode: deep}
        text: Probe ${target} exhaustively.
      - id: demo.quick-probe
        labels: [binding]
        class: must
        when: {mode: shallow}
        text: One pass over ${target} is enough.
      - id: demo.map-read
        labels: [binding]
        class: must
        when: {demo_map: present}
        text: The existing map is an obligated read.
      - id: demo.map-absent
        labels: [binding]
        class: must
        when: {demo_map: absent}
        text: A missing map is surfaced, never minted.
      - id: demo.escalate
        labels: [binding]
        class: must
        text: An unresolved question escalates to the lead (demo.lead-owns).
  - id: demo.sec.boundaries
    title: Boundaries
    intent: the boundaries the run is bound by
    rules:
      - id: demo.no-silent-writes
        labels: [binding]
        class: floor
        text: Never write outside ${target}.
      - id: demo.transport-floor
        extends: common.transport-floor
        class: floor
        when: {seats: multi}
  - id: demo.sec.fail-conditions
    title: Fail Conditions
    intent: the fail-conditions the run is bound by
    rules:
      - id: demo.fail.no-approval
        labels: [user-gate]
        class: floor
        kind: fail
        enforces: [demo.lead-owns]
        text: Closing without the user's approval fails the run.
";

const LABELS: &str = "\
kind: command-labels
labels:
  role: who holds a seat
  binding: an obligation on the run
  user-gate: a decision or checkpoint reserved to the user
  reporting: where reports land and the register they use
  floor-pointer: a binding that points at a skill-owned floor
";

const COMMON: &str = "\
kind: command-common
rules:
  - id: common.register
    labels: [reporting]
    text: User-facing prose follows the house register.
  - id: common.transport-floor
    labels: [floor-pointer]
    pointer: mochiko:patterns-transport-floor
    text: The transport floor governs multi-seat composition, referenced never restated.
";

// ---------------------------------------------------------------------------
// the ported probes
// ---------------------------------------------------------------------------

fn probes() -> Vec<Probe> {
    let mut p: Vec<Probe> = Vec::new();

    // --- the positive control ---
    p.push(Probe::new("baseline pair is clean", Expect::Clean, |_| {}));

    // --- 1. set-wise section assertion ---
    p.push(Probe::new(
        "canonical section absent",
        Expect::Reject(Code::SectionSet),
        |f| f.drop_section("tools"),
    ));
    p.push(Probe::new(
        "section outside the canonical six",
        Expect::Reject(Code::SectionSet),
        |f| {
            f.command().sections.push(Section {
                id: "demo.sec.extras".into(),
                title: "Extras".into(),
                intent: "x".into(),
                note: None,
                rules: vec![Rule {
                    id: "demo.stray".into(),
                    labels: Some(vec!["binding".into()]),
                    class: Some("must".into()),
                    text: Some("A stray rule.".into()),
                    ..Rule::default()
                }],
            });
        },
    ));
    p.push(Probe::new(
        "section IDs disagree on the prefix",
        Expect::Reject(Code::IdPrefix),
        |f| f.section("tools").id = "other.sec.tools".into(),
    ));

    // --- 2. empty-marker recognition ---
    p.push(Probe::new(
        "empty section carrying no note",
        Expect::Reject(Code::TextMissing),
        |f| f.section("reserved").note = None,
    ));

    // --- 5. the count pins, now computed rather than transcribed ---
    p.push(Probe::porting(
        "count-pin names the wrong number",
        "the fail-node count is computed from state, never transcribed",
        Expect::Counts {
            fails: 1,
            floors: 3,
        },
        |_| {},
    ));
    p.push(Probe::porting(
        "a second `kind: fail` rule re-pins the count cleanly",
        "a second `kind: fail` rule re-pins the count with no edit anywhere",
        Expect::Counts {
            fails: 2,
            floors: 4,
        },
        |f| f.add_second_fail(),
    ));

    // --- 7. tombstone references ---
    p.push(Probe::new(
        "an ID both live and tombstoned",
        Expect::Reject(Code::TombstoneIntegrity),
        |f| f.tombstone("demo.lead-owns"),
    ));
    p.push(Probe::new(
        "a tombstone entry missing its disposition",
        Expect::Reject(Code::TombstoneIntegrity),
        |f| {
            f.tombstone("demo.sec.legacy");
            f.command()
                .tombstones
                .last_mut()
                .expect("just pushed")
                .disposition = String::new();
        },
    ));

    // --- 8. `kind:` vocabulary ---
    p.push(Probe::new(
        "kind outside the closed set",
        Expect::RejectOn(Code::RuleKindUnknown, "demo.lead-owns"),
        |f| f.rule("demo.lead-owns").kind = Some("sproing".into()),
    ));
    p.push(Probe::new(
        "every legal kind is admitted",
        Expect::Clean,
        |f| {
            for kind in [
                "gate",
                "reservation",
                "bound",
                "routing",
                "latitude",
                "constraint",
            ] {
                f.push_rule(
                    "ways-of-working",
                    Rule {
                        id: format!("demo.kind-{kind}"),
                        labels: Some(vec!["binding".into()]),
                        class: Some("must".into()),
                        kind: Some(kind.into()),
                        text: Some(format!("A rule of kind {kind}.")),
                        ..Rule::default()
                    },
                );
            }
        },
    ));
    p.push(Probe::new(
        "an absent kind reads constraint, never a finding",
        Expect::Clean,
        |f| f.rule("demo.lead-owns").kind = None,
    ));

    // --- 9. the fail re-key, both ways ---
    p.push(Probe::new(
        "a .fail. rule with no explicit kind",
        Expect::RejectOn(Code::FailSegment, "demo.fail.no-approval"),
        |f| f.rule("demo.fail.no-approval").kind = None,
    ));
    p.push(Probe::new(
        "a .fail. rule carrying some other kind",
        Expect::RejectOn(Code::FailSegment, "demo.fail.no-approval"),
        |f| f.rule("demo.fail.no-approval").kind = Some("gate".into()),
    ));
    p.push(Probe::new(
        "kind: fail outside the .fail. segment",
        Expect::RejectOn(Code::FailSegment, "demo.one-question"),
        |f| f.rule("demo.one-question").kind = Some("fail".into()),
    ));

    // --- 10. the retired label ---
    p.push(Probe::new(
        "retired label on a rule",
        Expect::RejectOn(Code::LabelUnknown, "demo.one-question"),
        |f| f.rule("demo.one-question").labels = Some(vec!["fail-condition".into()]),
    ));

    // --- 11. conditions and when ---
    p.push(Probe::new(
        "when: names an undeclared dimension",
        Expect::Reject(Code::WhenUndeclared),
        |f| f.set_when("demo.deep-probe", "ghost", "deep"),
    ));
    p.push(Probe::new(
        "when: names an undeclared value",
        Expect::Reject(Code::WhenValue),
        |f| f.set_when("demo.deep-probe", "mode", "sideways"),
    ));
    p.push(Probe::new(
        "when: naming a list of declared values stays clean",
        Expect::Clean,
        |f| {
            f.rule("demo.deep-probe").when = vec![(
                "mode".into(),
                WhenValue::List(vec![
                    Value::String("deep".into()),
                    Value::String("shallow".into()),
                ]),
            )];
        },
    ));
    p.push(Probe::new(
        "when: on a presence dimension, both poles clean",
        Expect::Clean,
        |f| f.set_when("demo.map-read", "demo_map", "present"),
    ));
    p.push(Probe::new(
        "a dimension whose values: is neither a list nor `presence`",
        Expect::Reject(Code::ConditionDeclaration),
        |f| f.condition("mode").values = Some(Value::String("sometimes".into())),
    ));
    p.push(Probe::porting(
        "resolution point outside D3\'s closed set",
        "resolution point outside the closed set",
        Expect::Reject(Code::ConditionDeclaration),
        |f| f.condition("mode").resolution = Some("whenever".into()),
    ));
    p.push(Probe::new(
        "moment-resolved names an undeclared moment",
        Expect::Reject(Code::MomentUndeclared),
        |f| f.condition("mode").resolution = Some("moment-resolved(nowhere)".into()),
    ));
    p.push(Probe::new(
        "declared dimension no rule uses is a warning",
        Expect::Advisory(Code::UnusedCondition),
        |f| {
            f.command().conditions.push((
                "unused_dim".into(),
                mochiko_cli::model::Condition {
                    values: Some(Value::Sequence(vec![
                        Value::String("a".into()),
                        Value::String("b".into()),
                    ])),
                    resolution: Some("entry-derived".into()),
                    note: None,
                },
            ));
        },
    ));
    p.push(Probe::new(
        "declared value named by no rule's when: is a warning",
        Expect::Advisory(Code::ConditionCoverage),
        |f| f.rule("demo.quick-probe").when.clear(),
    ));

    // --- 12. moments ---
    p.push(Probe::new(
        "declared moment named by nothing is a warning",
        Expect::Advisory(Code::UnusedMoment),
        |f| {
            f.command()
                .moments
                .push(("landing".into(), "Where the run's outputs land.".into()));
        },
    ));
    p.push(Probe::new(
        "a moment mentioned only in prose counts as used",
        Expect::CleanOf(Code::UnusedMoment),
        |f| {
            f.command()
                .moments
                .push(("landing".into(), "Where the run's outputs land.".into()));
            f.rule("demo.one-question").text =
                Some("Ask one question at a time, through landing.".into());
        },
    ));

    // --- 13. the coverage report ---
    p.push(Probe::porting(
        "coverage report names an uncovered value",
        "the coverage report names an uncovered value",
        Expect::Advisory(Code::ConditionCoverage),
        |f| f.rule("demo.quick-probe").when.clear(),
    ));

    p.push(Probe::new(
        "a value carried only by a floor makes no coverage claim either way",
        Expect::CleanAbsent("\"shallow\" is declared but named by no rule"),
        |f| {
            f.rule("demo.quick-probe").when.clear();
            f.set_when("demo.no-silent-writes", "mode", "shallow");
        },
    ));

    // --- 15. enforces ---
    p.push(Probe::new(
        "a kind: fail node with no enforces:",
        Expect::RejectOn(Code::EnforcesRequired, "demo.fail.no-approval"),
        |f| f.rule("demo.fail.no-approval").enforces = None,
    ));
    p.push(Probe::new(
        "an enforces: target that resolves to nothing",
        Expect::Reject(Code::EnforcesUnresolved),
        |f| f.rule("demo.fail.no-approval").enforces = Some(vec!["demo.ghost".into()]),
    ));
    p.push(Probe::new(
        "an enforces: target that is tombstoned",
        Expect::Reject(Code::EnforcesUnresolved),
        |f| {
            f.tombstone("demo.legacy-rule");
            f.rule("demo.fail.no-approval").enforces = Some(vec!["demo.legacy-rule".into()]);
        },
    ));
    p.push(Probe::new(
        "enforces: on a node that is not kind: fail",
        Expect::RejectOn(Code::EnforcesMisplaced, "demo.one-question"),
        |f| f.rule("demo.one-question").enforces = Some(vec!["demo.lead-owns".into()]),
    ));
    p.push(Probe::new(
        "enforces: naming a section rather than a rule",
        Expect::Reject(Code::EnforcesUnresolved),
        |f| f.rule("demo.fail.no-approval").enforces = Some(vec!["demo.sec.roles".into()]),
    ));
    p.push(Probe::new(
        "an empty enforces: with no stated reason",
        Expect::RejectOn(Code::EnforcesRequired, "demo.fail.no-approval"),
        |f| f.rule("demo.fail.no-approval").enforces = Some(Vec::new()),
    ));
    p.push(Probe::porting(
        "an empty enforces: carrying its D6 reason comment",
        "an empty enforces: carrying its reason as data",
        Expect::Clean,
        |f| {
            let rule = f.rule("demo.fail.no-approval");
            rule.enforces = Some(Vec::new());
            rule.note = Some("the obligation is owned by a pointer skill.".into());
        },
    ));
    p.push(Probe::porting(
        "reverse coverage is labelled as the deferred pass\'s input, not a finding",
        "reverse coverage is advisory, never a finding",
        Expect::Advisory(Code::EnforcesCoverage),
        |_| {},
    ));

    // --- 16. extends ---
    p.push(Probe::new(
        "extends: names no block in the library",
        Expect::RejectOn(Code::ExtendsUnresolved, "demo.register"),
        |f| f.rule("demo.register").extends = Some("common.ghost".into()),
    ));
    p.push(Probe::new(
        "extends: target malformed",
        Expect::RejectOn(Code::ExtendsCrossFamily, "demo.register"),
        |f| f.rule("demo.register").extends = Some("register".into()),
    ));
    p.push(Probe::new(
        "an extends: stub declaring no local class",
        Expect::RejectOn(Code::ExtendsClassLocal, "demo.register"),
        |f| f.rule("demo.register").class = None,
    ));
    p.push(Probe::new(
        "the shared library is absent while a stub binds it",
        Expect::RejectOn(Code::ExtendsUnresolved, "demo.register"),
        |f| f.drop_common(),
    ));
    p.push(Probe::new(
        "an extends: stub whose library never loaded",
        Expect::RejectOn(Code::ExtendsUnresolved, "demo.transport-floor"),
        |f| f.drop_common(),
    ));
    p.push(Probe::new(
        "the library missing its kind: discriminator",
        Expect::Reject(Code::KindDiscriminator),
        |f| f.common().declared_kind = Some("command-rules".into()),
    ));
    p.push(Probe::porting(
        "a common block id outside the common.<slug> format",
        "a common block id outside the common.<slug> format",
        Expect::Reject(Code::IdPrefix),
        |f| f.common().blocks[0].id = "register".into(),
    ));
    p.push(Probe::new(
        "a duplicate common block id",
        Expect::Reject(Code::IdDuplicate),
        |f| {
            let block = f.common().blocks[0].clone();
            f.common().blocks.push(block);
        },
    ));
    p.push(Probe::new(
        "a common block with no text",
        Expect::Reject(Code::TextMissing),
        |f| f.common().blocks[0].text = Some("   ".into()),
    ));
    p.push(Probe::new(
        "an orphan ${var} inherited from a common block is attributed to the stub",
        Expect::RejectOn(Code::VarUnbound, "demo.register"),
        |f| {
            f.common().blocks[0].text = Some("User-facing prose follows ${nonexistent}.".into());
        },
    ));
    p.push(Probe::new(
        "deixis inherited from a common block is attributed to the stub",
        Expect::AdvisoryOn(Code::Deixis, "demo.register"),
        |f| {
            f.common().blocks[0].text = Some("User-facing prose follows these rules.".into());
        },
    ));

    // --- 17. the regression sweep ---
    p.push(Probe::new(
        "[regression] label outside the registry",
        Expect::RejectOn(Code::LabelUnknown, "demo.lead-owns"),
        |f| f.rule("demo.lead-owns").labels = Some(vec!["not-a-real-label".into()]),
    ));
    p.push(Probe::new(
        "[regression] orphan ${var} placeholder",
        Expect::RejectOn(Code::VarUnbound, "demo.lead-owns"),
        |f| f.rule("demo.lead-owns").text = Some("The lead owns ${nonexistent}.".into()),
    ));
    p.push(Probe::new(
        "[regression] deictic reference is a warning",
        Expect::Advisory(Code::Deixis),
        |f| {
            f.rule("demo.lead-owns").text = Some("The lead owns the run; these rules bind.".into());
        },
    ));
    p.push(Probe::new(
        "[regression] duplicate rule ID",
        Expect::Reject(Code::IdDuplicate),
        |f| f.rule("demo.read-first").id = "demo.lead-owns".into(),
    ));
    p.push(Probe::new(
        "[regression] malformed section ID",
        Expect::Reject(Code::IdFormat),
        |f| f.section("tools").id = "demo-tools".into(),
    ));

    // --- 18. document-level guards ---
    p.push(Probe::new(
        "the schema missing its kind: discriminator",
        Expect::Reject(Code::KindDiscriminator),
        |f| f.command().declared_kind = Some("command-schema".into()),
    ));
    p.push(Probe::new(
        "the schema missing its command: name",
        Expect::Reject(Code::KindDiscriminator),
        |f| f.command().declared_name = None,
    ));
    p.push(Probe::new(
        "the registry missing its kind: discriminator",
        Expect::Reject(Code::KindDiscriminator),
        |f| f.labels().declared_kind = Some("labels".into()),
    ));
    p.push(Probe::porting(
        "the schema carrying no sections",
        "the schema carrying no sections (reported as an underivable prefix)",
        Expect::Reject(Code::IdPrefix),
        |f| f.command().sections.clear(),
    ));

    // --- 20. section shape ---
    p.push(Probe::new(
        "a section missing its title",
        Expect::Reject(Code::TextMissing),
        |f| f.section("tools").title = String::new(),
    ));
    p.push(Probe::new(
        "two sections minting the same id",
        Expect::Reject(Code::IdDuplicate),
        |f| f.section("tools").id = "demo.sec.roles".into(),
    ));
    p.push(Probe::new(
        "no section ID well-formed enough to derive the prefix",
        Expect::Reject(Code::IdPrefix),
        |f| {
            for (i, section) in f.command().sections.iter_mut().enumerate() {
                section.id = format!("section-{i}");
            }
        },
    ));

    // --- 21. rule shape ---
    p.push(Probe::porting(
        "a rule missing its id",
        "a rule missing its id (an empty id leads with no prefix)",
        Expect::Reject(Code::IdPrefix),
        |f| f.rule("demo.read-first").id = String::new(),
    ));
    p.push(Probe::porting(
        "a rule id outside the dotted-slug format",
        "a rule id outside the dotted-slug format (the prefix limb reports first)",
        Expect::Reject(Code::IdPrefix),
        |f| f.rule("demo.read-first").id = "Demo_Read_First".into(),
    ));
    p.push(Probe::extra(
        "a prefixed rule id whose tail is not a slug",
        Expect::RejectOn(Code::IdFormat, "demo.Read_First"),
        |f| f.rule("demo.read-first").id = "demo.Read_First".into(),
    ));
    p.push(Probe::new(
        "a class outside floor|must|advisory",
        Expect::RejectOn(Code::ClassUnknown, "demo.read-first"),
        |f| f.rule("demo.read-first").class = Some("mandatory".into()),
    ));
    p.push(Probe::new(
        "a rule carrying no text",
        Expect::RejectOn(Code::TextMissing, "demo.read-first"),
        |f| f.rule("demo.read-first").text = Some("   ".into()),
    ));

    // --- 22. the rollout warnings ---
    p.push(Probe::new(
        "a declared var no rule text uses",
        Expect::Advisory(Code::UnusedVar),
        |f| {
            f.command().vars.push((
                "unused_var".into(),
                Value::String("nothing/reads/this".into()),
            ));
        },
    ));

    // --- 23. the sidecar's anchor, now a field on the rule ---
    p.push(Probe::new(
        "an anchor that is malformed",
        Expect::RejectOn(Code::AnchorFormat, "demo.lead-owns"),
        |f| f.rule("demo.lead-owns").anchor = Some("some time ago".into()),
    ));
    p.push(Probe::extra(
        "a well-formed anchor is carried without complaint",
        Expect::Clean,
        |f| f.rule("demo.lead-owns").anchor = Some("2026-08-27 demo-session D1".into()),
    ));

    // --- the protected-exit limb the hard set gained, which the Python never had ---
    p.push(Probe::extra(
        "a floor rule keeps its class",
        Expect::Counts {
            fails: 1,
            floors: 3,
        },
        |f| {
            assert_eq!(
                f.rule("demo.no-silent-writes").class_of(),
                Some(Class::Floor)
            );
        },
    ));

    p
}

// ---------------------------------------------------------------------------
// the four ledgers
// ---------------------------------------------------------------------------

/// Ported into `tests/fidelity.rs`, because the surface moved into the generator.
const GENESIS_SIDE: &[(&str, &str)] = &[
    ("the schema file is missing entirely", "a file that cannot be read stops genesis (`GenesisError::Read`); there is no per-file check at delivery time because the log carries documents, not files"),
    ("the schema does not parse as YAML", "`GenesisError::Parse` at import; the log itself is parsed by the migration grammar"),
    ("[regression] dangling provenance entry", "`GenesisError::DanglingAnchor` — an anchor naming no live rule stops the build"),
    ("the sidecar absent is a warning, not a finding", "genesis cannot fold anchors without the sidecar, so absence is an error rather than a warning — a severity change, disclosed"),
    ("the sidecar missing its kind: discriminator", "`GenesisError::SidecarKind`"),
    ("the post-rename primitive-provenance kind is accepted", "`GenesisError::SidecarKind` accepts both spellings; asserted by the shipped sidecar building clean"),
    ("the sidecar carrying no anchors mapping", "`GenesisError::SidecarShape`"),
    ("sidecar entries for another command are skipped, with a warning", "anchors are matched by rule id across every document, so a foreign prefix is no longer a case; an id no document carries is `DanglingAnchor`"),
];

/// The surface is gone under D6, with the reason. Never silently dropped.
const NOT_APPLICABLE: &[(&str, &str)] = &[
    ("canonical heading absent", "the `.md` scaffold: D6 collapses criteria 2 and the count limb of 3 to the `!` line, the grant and the halt clause"),
    ("canonical headings out of D2 order", "the `.md` scaffold"),
    ("Rules block omits a live section", "the `.md` no longer enumerates sections — the CLI renders them"),
    ("Rules block names a section the schema lacks", "the `.md` no longer enumerates sections"),
    ("stale 'nested in N sections' numeral", "the `.md` no longer carries a section count"),
    ("Not-done line sits above the protocol heading", "the `.md` scaffold's protocol shape"),
    ("a section heading follows the Not-done line", "the `.md` scaffold's protocol shape"),
    ("a second numbered list above the protocol heading stays clean", "the `.md` scaffold's protocol shape"),
    ("dangling token outside the Rules block", "prose token resolution over the `.md` body"),
    ("tombstoned token in the .md", "prose token resolution over the `.md` body"),
    ("foreign-prefix token is a warning, not a finding", "prose token resolution over the `.md` body"),
    ("retired label named in the .md", "prose scanning over the `.md` body"),
    ("the .md file is missing entirely", "the pair is no longer a file pair; the `.md` carries no schema content to be missing"),
    ("an anchor resolving to no DECISIONS.md row", "wave-plan §3 scopes wave 1 to anchor format; resolution stays an advisory report until the repo path is known"),
];

/// The Python carries a check the Rust hard set does not. Retiring the script without one of
/// these is a real loss, so each is named for the wave to rule on.
const OUTSIDE_THE_HARD_SET: &[(&str, &str)] = &[
    ("count-pin plural where the count is 1", "same: grammar of a hand-written pin"),
    ("count-pin singular where the count is 2", "same"),
    ("count-pin absent entirely", "same"),
    ("the count phrase sitting on a line that is not the Not-done line", "same"),
    ("rule text names a tombstoned node", "in-text citation resolution is not in the D6 hard set"),
    ("rule text names a node that never existed", "in-text citation resolution is not in the D6 hard set"),
    ("a fabricated citation dangles", "in-text citation resolution is not in the D6 hard set"),
    ("a citation of a tombstoned rule is a superseded reference", "in-text citation resolution is not in the D6 hard set"),
    ("a section-ID citation resolves", "in-text citation resolution is not in the D6 hard set"),
    ("a file-suffix token is not a citation", "in-text citation resolution is not in the D6 hard set"),
    ("the bare (non-parenthetical) citation form is scanned", "in-text citation resolution is not in the D6 hard set"),
    ("a foreign-prefix citation is a warning, not a dangle", "in-text citation resolution is not in the D6 hard set"),
    ("retired label still in the registry", "the registry's `retired:` list is data the model carries; nothing cross-checks it against the live labels"),
    ("the retired selector named in a section intent", "prose lint for a retired selector word"),
    ("the retired selector named in a rule text", "prose lint for a retired selector word"),
    ("when: written as a list, not a conjunction mapping", "a shape error the decoder rejects before the validator sees it"),
    ("when: naming a dimension with an empty value list", "an empty value list names no value to check; the hard set reports unknown values, not an absent one"),
    ("the same node tombstoned twice", "duplicate tombstones are not in the hard set; the id lifecycle is enforced at apply time by mint-once, and an imported document carrying two is not checked"),
    ("enforces: written as a bare string, not a list", "a shape error the decoder rejects: the model holds `enforces` as a list or nothing"),
    ("when: carrying nested structure (boolean algebra)", "the model's `when` value is scalar-or-list by construction"),
    ("conditions: written as something other than a mapping", "a shape error the decoder rejects"),
    ("a dimension declared as a bare string", "a shape error the decoder rejects"),
    ("moments: written as something other than a mapping", "a shape error the decoder rejects"),
    ("a moment declared with no navigation line", "an empty moment line is not in the hard set"),
    ("coverage report makes no claim over floors", "the coverage report's wording differs; the Rust report is per-value and does not restate the floor carve"),
    ("a stub whose local text repeats the block's", "the pointless-override warning is not ported"),
    ("a common block bound by no stub", "the orphan-block warning is not ported"),
    ("a common block carrying `kind:`", "the absence-meaningful-field guard on library blocks is not ported"),
    ("a common block carrying `when:`", "same"),
    ("a common block carrying `enforces:`", "same"),
    ("a common block carrying `class:` is a warning", "same"),
    ("the library carrying no rules: list", "an empty library is not itself a finding; every stub over it reports instead"),
    ("a common block with no id", "an id-less block is a decode-level shape error"),
    ("[regression] inline ruling: field", "the model has no `ruling:` field; an unknown key is dropped at decode rather than reported"),
    ("[regression] flat top-level rules:", "the model carries top-level `rules:` as a common library's blocks; a command schema with blocks is not flagged"),
    ("the registry carrying no labels mapping", "an empty registry is not itself a finding; every label reports instead"),
    ("tombstones: written as something other than a list", "a shape error the decoder rejects"),
    ("a section entry that is not a mapping", "a shape error the decoder rejects"),
    ("a section missing its id", "an empty section id surfaces as `id-format`, not as a distinct missing-key finding"),
    ("a section missing its rules key", "the model reads an absent `rules:` as an empty section"),
    ("a section whose rules: is not a list", "a shape error the decoder rejects"),
    ("an empty section written as `rules:` rather than `rules: []`", "a style warning about YAML spelling; the model reads both the same"),
    ("a rule entry that is not a mapping", "a shape error the decoder rejects"),
    ("a rule carrying no labels", "a label-less rule is not in the hard set"),
    ("a {{...}} skeleton sigil in rule text is a warning", "the skeleton-sigil warning is not ported"),
    ("a registry label with no members here", "the zero-member label warning is not ported"),
];

/// The 134 probe names of `scripts/test-check-command-schema.py`, verbatim.
const PYTHON_PROBES: [&str; 134] = [
    "baseline pair is clean",
    "canonical section absent",
    "section outside the canonical six",
    "section IDs disagree on the prefix",
    "empty section carrying no note",
    "canonical heading absent",
    "canonical headings out of D2 order",
    "Rules block omits a live section",
    "Rules block names a section the schema lacks",
    "count-pin names the wrong number",
    "count-pin plural where the count is 1",
    "count-pin singular where the count is 2",
    "a second `kind: fail` rule re-pins the count cleanly",
    "count-pin absent entirely",
    "stale 'nested in N sections' numeral",
    "Not-done line sits above the protocol heading",
    "a section heading follows the Not-done line",
    "a second numbered list above the protocol heading stays clean",
    "dangling token outside the Rules block",
    "tombstoned token in the .md",
    "foreign-prefix token is a warning, not a finding",
    "rule text names a tombstoned node",
    "rule text names a node that never existed",
    "kind outside the closed set",
    "every legal kind is admitted",
    "an absent kind reads constraint, never a finding",
    "a .fail. rule with no explicit kind",
    "a .fail. rule carrying some other kind",
    "kind: fail outside the .fail. segment",
    "retired label on a rule",
    "retired label still in the registry",
    "retired label named in the .md",
    "the retired selector named in a section intent",
    "the retired selector named in a rule text",
    "when: names an undeclared dimension",
    "when: names an undeclared value",
    "when: written as a list, not a conjunction mapping",
    "when: carrying nested structure (boolean algebra)",
    "when: naming a list of declared values stays clean",
    "when: on a presence dimension, both poles clean",
    "when: naming a dimension with an empty value list",
    "conditions: written as something other than a mapping",
    "a dimension declared as a bare string",
    "a dimension whose values: is neither a list nor `presence`",
    "resolution point outside D3's closed set",
    "moment-resolved names an undeclared moment",
    "declared dimension no rule uses is a warning",
    "declared value named by no rule's when: is a warning",
    "a value carried only by a floor makes no coverage claim either way",
    "moments: written as something other than a mapping",
    "a moment declared with no navigation line",
    "declared moment named by nothing is a warning",
    "a moment mentioned only in prose counts as used",
    "coverage report names an uncovered value",
    "coverage report makes no claim over floors",
    "a fabricated citation dangles",
    "a citation of a tombstoned rule is a superseded reference",
    "a section-ID citation resolves",
    "a file-suffix token is not a citation",
    "the bare (non-parenthetical) citation form is scanned",
    "a foreign-prefix citation is a warning, not a dangle",
    "a kind: fail node with no enforces:",
    "an enforces: target that resolves to nothing",
    "an enforces: target that is tombstoned",
    "enforces: on a node that is not kind: fail",
    "enforces: written as a bare string, not a list",
    "enforces: naming a section rather than a rule",
    "an empty enforces: with no stated reason",
    "an empty enforces: carrying its D6 reason comment",
    "reverse coverage is labelled as the deferred pass's input, not a finding",
    "extends: names no block in the library",
    "extends: target malformed",
    "an extends: stub declaring no local class",
    "a stub whose local text repeats the block's",
    "a common block bound by no stub",
    "a common block carrying `kind:`",
    "a common block carrying `when:`",
    "a common block carrying `enforces:`",
    "a common block carrying `class:` is a warning",
    "the shared library is absent while a stub binds it",
    "an extends: stub whose library never loaded",
    "the library missing its kind: discriminator",
    "the library carrying no rules: list",
    "a common block with no id",
    "a common block id outside the common.<slug> format",
    "a duplicate common block id",
    "a common block with no text",
    "an orphan ${var} inherited from a common block is attributed to the stub",
    "deixis inherited from a common block is attributed to the stub",
    "[regression] inline ruling: field",
    "[regression] dangling provenance entry",
    "[regression] label outside the registry",
    "[regression] orphan ${var} placeholder",
    "[regression] deictic reference is a warning",
    "[regression] duplicate rule ID",
    "[regression] flat top-level rules:",
    "[regression] malformed section ID",
    "the schema file is missing entirely",
    "the schema does not parse as YAML",
    "the .md file is missing entirely",
    "the schema missing its kind: discriminator",
    "the schema missing its command: name",
    "the registry missing its kind: discriminator",
    "the registry carrying no labels mapping",
    "the schema carrying no sections",
    "tombstones: written as something other than a list",
    "a tombstone entry missing its disposition",
    "the same node tombstoned twice",
    "an ID both live and tombstoned",
    "a section entry that is not a mapping",
    "a section missing its id",
    "two sections minting the same id",
    "a section missing its title",
    "a section missing its rules key",
    "a section whose rules: is not a list",
    "an empty section written as `rules:` rather than `rules: []`",
    "no section ID well-formed enough to derive the prefix",
    "a rule entry that is not a mapping",
    "a rule missing its id",
    "a rule id outside the dotted-slug format",
    "a class outside floor|must|advisory",
    "a rule carrying no labels",
    "a rule carrying no text",
    "a {{...}} skeleton sigil in rule text is a warning",
    "a declared var no rule text uses",
    "a registry label with no members here",
    "the sidecar absent is a warning, not a finding",
    "the sidecar missing its kind: discriminator",
    "the post-rename primitive-provenance kind is accepted",
    "the sidecar carrying no anchors mapping",
    "an anchor that is malformed",
    "an anchor resolving to no DECISIONS.md row",
    "sidecar entries for another command are skipped, with a warning",
    "the count phrase sitting on a line that is not the Not-done line",
];

#[test]
fn the_whole_python_matrix_is_accounted_for() {
    matrix::accounted_for(
        &PYTHON_PROBES,
        &probes(),
        &[
            ("genesis-side", GENESIS_SIDE),
            ("not applicable", NOT_APPLICABLE),
            ("outside the hard set", OUTSIDE_THE_HARD_SET),
        ],
    );
}

#[test]
fn the_command_matrix_holds() {
    matrix::run("command", &probes(), baseline);
}
