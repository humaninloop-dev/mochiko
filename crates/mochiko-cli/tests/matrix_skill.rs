//! The ported probe matrix for `scripts/check-skill-schema.py` — 114 probes.
//!
//! **The census the record carries is stale.** `record.md` D6/D8 and the wave plan say 86; the
//! file's own `probes()` returns 114, because the matrix grew with the authoring-family and
//! patterns-family waves. The figure here is the one the script actually runs.
//!
//! One state carries all three families at once — the review, authoring and patterns fixtures
//! plus both libraries and the registry — because the skill grammar's own probes need them
//! together: cross-family `extends:` cannot be tested with one library in state, and the
//! validator grades the whole corpus in one pass rather than one skill at a time.
//!
//! The four ledgers and their meaning are the same as `matrix_command.rs`.
//!
//! **Unit 1b (2026-09-04)** moved 26 probes out of `OUTSIDE_THE_HARD_SET` and into the ported
//! set, including all five pointer-resolution probes and the sweep-scoped claims: a whole-state
//! validator *is* the sweep, so "no orphan claim over a family with nothing swept" and "a label
//! no swept schema carries" both have referents here. What stays outside is the per-single-skill
//! run, which has none — the validator never grades one skill alone.

mod matrix;

use matrix::{Expect, Fixture, Probe};
use mochiko_cli::model::{Condition, DocKind, Rule, Section, WhenValue};
use mochiko_cli::validate::Code;
use serde_norway::Value;

// ---------------------------------------------------------------------------
// the fixtures
// ---------------------------------------------------------------------------

fn baseline() -> Fixture {
    Fixture::for_skill(
        "demo-grader",
        &[
            (DocKind::Skill, "demo-grader", REVIEW),
            (DocKind::Skill, "authoring-demo", AUTHORING),
            (DocKind::Skill, "patterns-demo", PATTERNS),
            (DocKind::SkillCommon, "skill-review-common", REVIEW_COMMON),
            (
                DocKind::SkillCommon,
                "skill-authoring-common",
                AUTHORING_COMMON,
            ),
            (DocKind::SkillLabels, "skill-labels", LABELS),
        ],
        Some("skill-review-common"),
    )
}

const LABELS: &str = "\
kind: skill-labels
labels:
  independence: who is never whom
  boundary: what the skill does not do
  inputs: what it reads
  verdict: how it decides
  output: where it lands
  binding: an obligation on the run
";

const REVIEW: &str = "\
kind: skill
skill: demo-grader
vars:
  report_path: .mochiko/reports/demo.md
conditions:
  depth:
    values: [deep, shallow]
    resolution: entry-derived
    note: how far the grader goes.
sections:
  - id: demo-grader.sec.independence
    title: Independence
    intent: seat separations this grader carries
    rules:
      - id: demo-grader.never-author
        labels: [independence]
        class: floor
        text: Never author or fix what you grade.
  - id: demo-grader.sec.scope
    title: Scope
    intent: the carve-outs
    rules:
      - id: demo-grader.carve-out
        labels: [boundary]
        class: must
        text: Grade the artifact, never the author.
  - id: demo-grader.sec.inputs
    title: Inputs
    intent: what is read
    rules:
      - id: demo-grader.read-report
        labels: [inputs]
        class: must
        text: Read ${report_path} in full before the first finding.
      - id: demo-grader.deep-read
        labels: [inputs]
        class: must
        when: {depth: deep}
        text: A deep pass reads every referenced source.
  - id: demo-grader.sec.verdict
    title: Verdict
    intent: how the verdict is reached
    rules:
      - id: demo-grader.default-fail
        extends: review-common.default-fail
        class: floor
      - id: demo-grader.shallow-pass
        labels: [verdict]
        class: must
        when: {depth: shallow}
        text: A shallow pass states what it did not read.
  - id: demo-grader.sec.output
    title: Output
    intent: where the verdict lands
    rules:
      - id: demo-grader.report-lands
        labels: [output]
        class: must
        text: The verdict lands in ${report_path}.
  - id: demo-grader.sec.reserved
    title: Reserved
    intent: rulings reserved to the user
    note: no reserved rulings yet — deliberately empty, not an omission
    rules: []
";

const AUTHORING: &str = "\
kind: skill
skill: authoring-demo
vars:
  artifact_path: .mochiko/specs/demo/spec.md
sections:
  - id: authoring-demo.sec.independence
    title: Independence
    intent: seat separations this producer carries
    rules:
      - id: authoring-demo.never-grade
        labels: [independence]
        class: floor
        text: Never grade what you authored.
  - id: authoring-demo.sec.scope
    title: Scope
    intent: the carve-outs
    rules:
      - id: authoring-demo.scope
        labels: [boundary]
        class: must
        text: Author the artifact, never the plan behind it.
  - id: authoring-demo.sec.inputs
    title: Inputs
    intent: what is read
    rules:
      - id: authoring-demo.read-brief
        labels: [inputs]
        class: must
        text: Read the brief in full before authoring.
  - id: authoring-demo.sec.artifact
    title: Artifact
    intent: what is produced
    rules:
      - id: authoring-demo.envelope
        extends: authoring-common.envelope
        class: must
      - id: authoring-demo.two-arm
        labels: [binding]
        class: must
        text: The artifact follows ${artifact_path}.
  - id: authoring-demo.sec.output
    title: Output
    intent: where it lands
    rules:
      - id: authoring-demo.lands
        labels: [output]
        class: must
        text: The artifact lands at ${artifact_path}.
  - id: authoring-demo.sec.reserved
    title: Reserved
    intent: rulings reserved to the user
    note: no reserved rulings yet — deliberately empty, not an omission
    rules: []
";

const PATTERNS: &str = "\
kind: skill
skill: patterns-demo
sections:
  - id: patterns-demo.sec.trigger
    title: Trigger
    intent: when the pattern fires
    rules:
      - id: patterns-demo.fires
        labels: [binding]
        class: must
        text: The pattern fires at the first composing move.
  - id: patterns-demo.sec.scope
    title: Scope
    intent: the carve-outs
    rules:
      - id: patterns-demo.scope
        labels: [boundary]
        class: must
        text: The pattern governs use, never the choice of tool.
  - id: patterns-demo.sec.discipline
    title: Discipline
    intent: what the pattern obliges
    rules:
      - id: patterns-demo.discipline
        labels: [binding]
        class: floor
        text: Every lane is non-waivable once it fires.
  - id: patterns-demo.sec.inputs
    title: Inputs
    intent: what is read
    rules:
      - id: patterns-demo.reads
        labels: [inputs]
        class: must
        text: Read the composing brief before the first dispatch.
  - id: patterns-demo.sec.disclosure
    title: Disclosure
    intent: what is disclosed
    rules:
      - id: patterns-demo.discloses
        labels: [output]
        class: must
        text: Every fired lane is disclosed in the run's report.
  - id: patterns-demo.sec.reserved
    title: Reserved
    intent: rulings reserved to the user
    note: no reserved rulings yet — deliberately empty, not an omission
    rules: []
";

const REVIEW_COMMON: &str = "\
kind: skill-common
rules:
  - id: review-common.default-fail
    labels: [verdict]
    text: >-
      Never default to a clearing verdict — earned only by a completed hunt.
";

const AUTHORING_COMMON: &str = "\
kind: skill-common
rules:
  - id: authoring-common.envelope
    labels: [binding]
    text: The produced artifact follows the deliverable envelope.
";

// ---------------------------------------------------------------------------
// the ported probes
// ---------------------------------------------------------------------------

fn probes() -> Vec<Probe> {
    let mut p: Vec<Probe> = Vec::new();

    p.push(Probe::new("baseline pair is clean", Expect::Clean, |_| {}));

    // --- discriminators and file-level guards ---
    p.push(Probe::new(
        "the schema missing its kind: discriminator",
        Expect::Reject(Code::KindDiscriminator),
        |f| f.skill_schema().declared_kind = Some("skill-schema".into()),
    ));
    p.push(Probe::new(
        "the schema missing its skill: name",
        Expect::Reject(Code::KindDiscriminator),
        |f| f.skill_schema().declared_name = None,
    ));
    p.push(Probe::new(
        "the skill: name disagreeing with its directory",
        Expect::Reject(Code::KindDiscriminator),
        |f| f.skill_schema().declared_name = Some("other-grader".into()),
    ));
    p.push(Probe::new(
        "the registry missing its kind: discriminator",
        Expect::Reject(Code::KindDiscriminator),
        |f| f.labels().declared_kind = Some("labels".into()),
    ));
    p.push(Probe::new(
        "the library missing its kind: discriminator",
        Expect::Reject(Code::KindDiscriminator),
        |f| f.common().declared_kind = Some("skill-rules".into()),
    ));

    // --- the six-section set, per family ---
    p.push(Probe::porting(
        "the schema carrying no sections",
        "the schema carrying no sections (every canonical section is then absent)",
        Expect::Reject(Code::SectionSet),
        |f| f.skill_schema().sections.clear(),
    ));
    // The four "canonical section absent" probes below stay `Expect::Reject`. The Python names
    // the missing section inside its message; the Rust finding carries no id, because the node
    // it would name is the one that is not there. Every other probe whose Python expectation
    // names a node now asserts that node (advisory A4).
    p.push(Probe::new(
        "canonical section absent",
        Expect::Reject(Code::SectionSet),
        |f| f.drop_section("verdict"),
    ));
    p.push(Probe::new(
        "a section outside the canonical six (the command six-set is not this grammar)",
        Expect::RejectOn(Code::SectionSet, "demo-grader.sec.tools"),
        |f| f.section("scope").id = "demo-grader.sec.tools".into(),
    ));
    p.push(Probe::porting(
        "a section ID leading with a foreign stem",
        "a section ID leading with a foreign stem (reported against the family's own set)",
        Expect::RejectOn(Code::IdFormat, "other-grader.sec.scope"),
        |f| f.section("scope").id = "other-grader.sec.scope".into(),
    ));
    p.push(Probe::new(
        "an empty section carrying no note",
        Expect::RejectOn(Code::TextMissing, "demo-grader.sec.reserved"),
        |f| f.section("reserved").note = None,
    ));

    // --- rule identity and shape ---
    p.push(Probe::new(
        "a duplicate rule ID",
        Expect::Reject(Code::IdDuplicate),
        |f| f.rule("demo-grader.carve-out").id = "demo-grader.never-author".into(),
    ));
    p.push(Probe::porting(
        "a rule id outside the dotted-slug format",
        "a rule id outside the dotted-slug format (the stem limb reports first)",
        Expect::Reject(Code::IdPrefix),
        |f| f.rule("demo-grader.read-report").id = "Demo_Read_Report".into(),
    ));
    p.push(Probe::new(
        "a rule id leading with a foreign stem",
        Expect::RejectOn(Code::IdPrefix, "demo-validator.read-report"),
        |f| f.rule("demo-grader.read-report").id = "demo-validator.read-report".into(),
    ));
    p.push(Probe::porting(
        "a rule missing its id",
        "a rule missing its id (an empty id leads with no stem)",
        Expect::Reject(Code::IdPrefix),
        |f| f.rule("demo-grader.read-report").id = String::new(),
    ));
    p.push(Probe::new(
        "a class outside floor|must|advisory",
        Expect::RejectOn(Code::ClassUnknown, "demo-grader.read-report"),
        |f| f.rule("demo-grader.read-report").class = Some("mandatory".into()),
    ));
    p.push(Probe::new(
        "a rule carrying no text",
        Expect::RejectOn(Code::TextMissing, "demo-grader.read-report"),
        |f| f.rule("demo-grader.read-report").text = Some("   ".into()),
    ));
    p.push(Probe::new(
        "a label outside the registry",
        Expect::RejectOn(Code::LabelUnknown, "demo-grader.read-report"),
        |f| f.rule("demo-grader.read-report").labels = Some(vec!["not-a-real-label".into()]),
    ));

    // --- the eight-kind vocabulary, and the fields that left the grammar ---
    p.push(Probe::new(
        "a kind outside the eight-kind set",
        Expect::Reject(Code::RuleKindUnknown),
        |f| f.rule("demo-grader.carve-out").kind = Some("sproing".into()),
    ));
    p.push(Probe::new(
        "kind: fail names its census retirement, not a generic vocabulary miss",
        Expect::Reject(Code::SkillGrammar),
        |f| f.rule("demo-grader.carve-out").kind = Some("fail".into()),
    ));
    p.push(Probe::new(
        "enforces: anywhere in a skill schema",
        Expect::Reject(Code::SkillGrammar),
        |f| {
            f.rule("demo-grader.carve-out").enforces =
                Some(vec!["demo-grader.never-author".into()]);
        },
    ));
    p.push(Probe::new(
        "a moments: block in a skill schema",
        Expect::Reject(Code::SkillGrammar),
        |f| {
            f.skill_schema()
                .moments
                .push(("open".into(), "Where the grade opens.".into()));
        },
    ));
    p.push(Probe::new(
        "every legal kind is admitted",
        Expect::Clean,
        |f| {
            for kind in [
                "constraint",
                "duty",
                "gate",
                "reservation",
                "binding",
                "bound",
                "routing",
                "latitude",
            ] {
                f.push_rule(
                    "scope",
                    Rule {
                        id: format!("demo-grader.kind-{kind}"),
                        labels: Some(vec!["boundary".into()]),
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
        Expect::CleanAbsent("is not one of"),
        |f| f.rule("demo-grader.carve-out").kind = None,
    ));

    // --- conditions and when ---
    p.push(Probe::new(
        "when: names an undeclared dimension",
        Expect::Reject(Code::WhenUndeclared),
        |f| f.set_when("demo-grader.deep-read", "ghost", "deep"),
    ));
    p.push(Probe::new(
        "when: names an undeclared value",
        Expect::Reject(Code::WhenValue),
        |f| f.set_when("demo-grader.deep-read", "depth", "sideways"),
    ));
    p.push(Probe::new(
        "a moment-resolved resolution point is command grammar",
        Expect::Reject(Code::SkillGrammar),
        |f| f.condition("depth").resolution = Some("moment-resolved(open)".into()),
    ));
    p.push(Probe::new(
        "a resolution point outside the skill set",
        Expect::Reject(Code::ConditionDeclaration),
        |f| f.condition("depth").resolution = Some("whenever".into()),
    ));
    p.push(Probe::new(
        "a declared dimension no rule uses is a warning",
        Expect::Advisory(Code::UnusedCondition),
        |f| {
            f.skill_schema().conditions.push((
                "unused_dim".into(),
                Condition {
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
        "a declared value named by no rule's when: is a warning",
        Expect::Advisory(Code::ConditionCoverage),
        |f| f.rule("demo-grader.shallow-pass").when.clear(),
    ));

    // --- extends, per family ---
    p.push(Probe::new(
        "extends: names no block in the library",
        Expect::RejectOn(Code::ExtendsUnresolved, "demo-grader.default-fail"),
        |f| f.rule("demo-grader.default-fail").extends = Some("review-common.ghost".into()),
    ));
    p.push(Probe::new(
        "extends: target outside the review-common.<slug> format",
        Expect::RejectOn(Code::ExtendsCrossFamily, "demo-grader.default-fail"),
        |f| f.rule("demo-grader.default-fail").extends = Some("default-fail".into()),
    ));
    p.push(Probe::new(
        "an extends: stub declaring no local class",
        Expect::RejectOn(Code::ExtendsClassLocal, "demo-grader.default-fail"),
        |f| f.rule("demo-grader.default-fail").class = None,
    ));
    p.push(Probe::new(
        "the family library absent while a stub binds it",
        Expect::RejectOn(Code::ExtendsUnresolved, "demo-grader.default-fail"),
        |f| f.drop_common(),
    ));
    p.push(Probe::new(
        "a common block id outside the review-common.<slug> format",
        Expect::Reject(Code::IdPrefix),
        |f| f.common().blocks[0].id = "default-fail".into(),
    ));
    p.push(Probe::new(
        "an orphan ${var} inherited from a common block is attributed to the stub",
        Expect::RejectOn(Code::VarUnbound, "demo-grader.default-fail"),
        |f| {
            f.common().blocks[0].text = Some("Never default to ${nonexistent}.".into());
        },
    ));

    // --- text-level checks ---
    p.push(Probe::new(
        "an orphan ${var} placeholder",
        Expect::RejectOn(Code::VarUnbound, "demo-grader.carve-out"),
        |f| f.rule("demo-grader.carve-out").text = Some("Grade ${nonexistent}.".into()),
    ));
    p.push(Probe::new(
        "a declared var no rule text uses",
        Expect::Advisory(Code::UnusedVar),
        |f| {
            f.skill_schema()
                .vars
                .push(("unused_var".into(), Value::String("nothing".into())));
        },
    ));
    p.push(Probe::new(
        "a deictic reference is a warning",
        Expect::Advisory(Code::Deixis),
        |f| {
            f.rule("demo-grader.carve-out").text =
                Some("Grade the artifact; these rules bind.".into());
        },
    ));

    // --- tombstones ---
    p.push(Probe::new(
        "an ID both live and tombstoned",
        Expect::Reject(Code::TombstoneIntegrity),
        |f| f.tombstone("demo-grader.never-author"),
    ));
    p.push(Probe::new(
        "a tombstone entry missing its disposition",
        Expect::Reject(Code::TombstoneIntegrity),
        |f| {
            f.tombstone("demo-grader.legacy-rule");
            f.skill_schema()
                .tombstones
                .last_mut()
                .expect("just pushed")
                .disposition = String::new();
        },
    ));

    // --- the floor pin, now computed ---
    p.push(Probe::porting(
        "the pin names the wrong number",
        "the floor count is computed from state, never transcribed",
        Expect::Counts {
            fails: 0,
            floors: 2,
        },
        |_| {},
    ));
    p.push(Probe::porting(
        "a re-pinned count survives cleanly",
        "a new floor re-pins the count with no edit anywhere",
        Expect::Counts {
            fails: 0,
            floors: 3,
        },
        |f| {
            f.push_rule(
                "scope",
                Rule {
                    id: "demo-grader.new-floor".into(),
                    labels: Some(vec!["boundary".into()]),
                    class: Some("floor".into()),
                    text: Some("A second boundary the grader holds.".into()),
                    ..Rule::default()
                },
            );
        },
    ));

    // --- the sidecar's anchor, now a field on the rule ---
    p.push(Probe::porting(
        "an anchor that is malformed",
        "an anchor that is malformed, carried on the rule",
        Expect::RejectOn(Code::AnchorFormat, "demo-grader.never-author"),
        |f| f.rule("demo-grader.never-author").anchor = Some("some time ago".into()),
    ));

    // --- the authoring family ---
    p.push(Probe::new(
        "the authoring baseline pair is clean",
        Expect::CleanAbsent("not in this family's registry"),
        |f| f.retarget("authoring-demo", Some("skill-authoring-common")),
    ));
    p.push(Probe::new(
        "[authoring] the canonical artifact section absent",
        Expect::Reject(Code::SectionSet),
        |f| {
            f.retarget("authoring-demo", Some("skill-authoring-common"));
            f.drop_section("artifact");
        },
    ));
    p.push(Probe::new(
        "[authoring] a verdict section is the review set, not this family's",
        Expect::RejectOn(Code::SectionSet, "authoring-demo.sec.verdict"),
        |f| {
            f.retarget("authoring-demo", Some("skill-authoring-common"));
            f.section("scope").id = "authoring-demo.sec.verdict".into();
        },
    ));
    p.push(Probe::new(
        "[review] an artifact section is the authoring set, not this family's",
        Expect::RejectOn(Code::SectionSet, "demo-grader.sec.artifact"),
        |f| f.section("scope").id = "demo-grader.sec.artifact".into(),
    ));
    p.push(Probe::new(
        "[authoring] a stub extending the review family's library is cross-family",
        Expect::RejectOn(Code::ExtendsCrossFamily, "authoring-demo.envelope"),
        |f| {
            f.retarget("authoring-demo", Some("skill-authoring-common"));
            f.rule("authoring-demo.envelope").extends = Some("review-common.default-fail".into());
        },
    ));
    p.push(Probe::new(
        "a review stub extending the authoring family's library is cross-family",
        Expect::RejectOn(Code::ExtendsCrossFamily, "demo-grader.default-fail"),
        |f| {
            f.rule("demo-grader.default-fail").extends = Some("authoring-common.envelope".into());
        },
    ));
    p.push(Probe::new(
        "[authoring] extends: names no block in the authoring library",
        Expect::RejectOn(Code::ExtendsUnresolved, "authoring-demo.envelope"),
        |f| {
            f.retarget("authoring-demo", Some("skill-authoring-common"));
            f.rule("authoring-demo.envelope").extends = Some("authoring-common.ghost".into());
        },
    ));
    p.push(Probe::new(
        "[authoring] the authoring library absent while a stub binds",
        Expect::RejectOn(Code::ExtendsUnresolved, "authoring-demo.envelope"),
        |f| {
            f.retarget("authoring-demo", Some("skill-authoring-common"));
            f.drop_common();
        },
    ));
    p.push(Probe::new(
        "[authoring] a block id outside the authoring-common.<slug> format",
        Expect::Reject(Code::IdPrefix),
        |f| {
            f.retarget("authoring-demo", Some("skill-authoring-common"));
            f.common().blocks[0].id = "envelope".into();
        },
    ));
    p.push(Probe::new(
        "[authoring] the library missing its kind: discriminator",
        Expect::Reject(Code::KindDiscriminator),
        |f| {
            f.retarget("authoring-demo", Some("skill-authoring-common"));
            f.common().declared_kind = Some("skill-rules".into());
        },
    ));
    p.push(Probe::new(
        "[authoring] an orphan ${template} is attributed to the binding stub",
        Expect::RejectOn(Code::VarUnbound, "authoring-demo.envelope"),
        |f| {
            f.retarget("authoring-demo", Some("skill-authoring-common"));
            f.common().blocks[0].text = Some("The artifact follows ${template}.".into());
        },
    ));
    p.push(Probe::new(
        "[authoring] an extends: stub declaring no local class",
        Expect::RejectOn(Code::ExtendsClassLocal, "authoring-demo.envelope"),
        |f| {
            f.retarget("authoring-demo", Some("skill-authoring-common"));
            f.rule("authoring-demo.envelope").class = None;
        },
    ));

    // --- the patterns family ---
    p.push(Probe::new(
        "the patterns baseline pair is clean",
        Expect::CleanAbsent("not in this family's registry"),
        |f| f.retarget("patterns-demo", None),
    ));
    p.push(Probe::new(
        "[patterns] the canonical trigger section absent",
        Expect::Reject(Code::SectionSet),
        |f| {
            f.retarget("patterns-demo", None);
            f.drop_section("trigger");
        },
    ));
    p.push(Probe::new(
        "[patterns] a verdict section is the review set, not this family's",
        Expect::RejectOn(Code::SectionSet, "patterns-demo.sec.verdict"),
        |f| {
            f.retarget("patterns-demo", None);
            f.section("scope").id = "patterns-demo.sec.verdict".into();
        },
    ));
    p.push(Probe::new(
        "[patterns] an artifact section is the authoring set, not this family's",
        Expect::RejectOn(Code::SectionSet, "patterns-demo.sec.artifact"),
        |f| {
            f.retarget("patterns-demo", None);
            f.section("scope").id = "patterns-demo.sec.artifact".into();
        },
    ));
    p.push(Probe::new(
        "[review] a discipline section is the patterns set, not this family's",
        Expect::RejectOn(Code::SectionSet, "demo-grader.sec.discipline"),
        |f| f.section("scope").id = "demo-grader.sec.discipline".into(),
    ));
    p.push(Probe::new(
        "[patterns] any extends: is the no-common-file finding",
        Expect::RejectOn(Code::ExtendsCrossFamily, "patterns-demo.scope"),
        |f| {
            f.retarget("patterns-demo", None);
            f.rule("patterns-demo.scope").extends = Some("patterns-common.ghost".into());
        },
    ));
    p.push(Probe::new(
        "[patterns] a stub naming a real other-family block still gets the no-library finding",
        Expect::RejectOn(Code::ExtendsCrossFamily, "patterns-demo.scope"),
        |f| {
            f.retarget("patterns-demo", None);
            f.rule("patterns-demo.scope").extends = Some("review-common.default-fail".into());
        },
    ));

    // --- unit 1b: the family-2 checks, re-claimed from OUTSIDE_THE_HARD_SET ---

    // Pointer resolution (J-7). Base-directory-relative from the skill's own directory, which is
    // the path the installed cache is read on — so a root-relative path dangles where it is used.
    p.push(Probe::new(
        "the in-directory and cross-directory pointers of the baseline resolve",
        Expect::CleanOf(Code::PointerUnresolved),
        |f| {
            f.use_pointer_root();
            f.rule("demo-grader.read-report").pointer = Some("references/PRESENT.md".into());
            f.rule("demo-grader.deep-read").pointer =
                Some("../authoring-demo/references/SIBLING.md".into());
        },
    ));
    p.push(Probe::new(
        "a pointer to a file that does not exist",
        Expect::RejectOn(Code::PointerUnresolved, "demo-grader.read-report"),
        |f| {
            f.use_pointer_root();
            f.rule("demo-grader.read-report").pointer = Some("references/ABSENT.md".into());
        },
    ));
    p.push(Probe::new(
        "a cross-directory climb to a file that does not exist",
        Expect::RejectOn(Code::PointerUnresolved, "demo-grader.read-report"),
        |f| {
            f.use_pointer_root();
            f.rule("demo-grader.read-report").pointer =
                Some("../authoring-demo/references/ABSENT.md".into());
        },
    ));
    p.push(Probe::new(
        "an absolute pointer path",
        Expect::RejectOn(Code::PointerUnresolved, "demo-grader.read-report"),
        |f| {
            f.use_pointer_root();
            f.rule("demo-grader.read-report").pointer = Some("/references/PRESENT.md".into());
        },
    ));
    p.push(Probe::new(
        "a skill-name pointer is a name, not a path — skipped",
        Expect::CleanOf(Code::PointerUnresolved),
        |f| {
            f.use_pointer_root();
            f.rule("demo-grader.read-report").pointer = Some("mochiko:patterns-sound-loop".into());
        },
    ));
    p.push(Probe::extra(
        "a pointer resolving only from the plugin root is its own finding",
        Expect::RejectOn(Code::PointerUnresolved, "demo-grader.read-report"),
        |f| {
            f.use_pointer_root();
            f.rule("demo-grader.read-report").pointer = Some("templates/AT-ROOT.md".into());
        },
    ));

    // In-text ID citation resolution. Own-stem tokens resolve here; a foreign stem resolves on
    // its own document's run and is named in a warning.
    p.push(Probe::new(
        "a fabricated citation dangles",
        Expect::RejectOn(Code::CiteUnresolved, "demo-grader.carve-out"),
        |f| {
            f.rule("demo-grader.carve-out").text =
                Some("Grade the artifact, per demo-grader.ghost-rule.".into())
        },
    ));
    p.push(Probe::new(
        "a citation of a tombstoned rule is a superseded reference",
        Expect::RejectOn(Code::CiteUnresolved, "demo-grader.carve-out"),
        |f| {
            f.tombstone("demo-grader.legacy-rule");
            f.rule("demo-grader.carve-out").text =
                Some("Grade as demo-grader.legacy-rule says.".into());
        },
    ));
    p.push(Probe::new(
        "a section-ID citation resolves",
        Expect::CleanAbsent("demo-grader.sec.verdict, which"),
        |f| {
            f.rule("demo-grader.carve-out").text =
                Some("The clearing grammar lives in demo-grader.sec.verdict.".into())
        },
    ));
    p.push(Probe::new(
        "a foreign-stem citation is a warning, not a dangle",
        Expect::Advisory(Code::CiteForeign),
        |f| {
            f.rule("demo-grader.carve-out").text =
                Some("The producer owns that, per authoring-demo.scope.".into())
        },
    ));
    p.push(Probe::new(
        "rule text naming a section that never existed",
        Expect::RejectOn(Code::CiteUnresolved, "demo-grader.carve-out"),
        |f| {
            f.rule("demo-grader.carve-out").text =
                Some("See demo-grader.sec.ghost for the carve-out.".into())
        },
    ));
    p.push(Probe::new(
        "rule text naming a tombstoned section",
        Expect::RejectOn(Code::CiteUnresolved, "demo-grader.carve-out"),
        |f| {
            f.tombstone("demo-grader.sec.legacy");
            f.rule("demo-grader.carve-out").text =
                Some("See demo-grader.sec.legacy for the carve-out.".into());
        },
    ));

    // Labels: the finding, and the one absence that is inherited by design.
    p.push(Probe::new(
        "a rule carrying no labels",
        Expect::RejectOn(Code::LabelsMissing, "demo-grader.carve-out"),
        |f| f.rule("demo-grader.carve-out").labels = None,
    ));
    p.push(Probe::new(
        "a stub with a LOCAL empty labels: is still a finding",
        Expect::RejectOn(Code::LabelsMissing, "demo-grader.default-fail"),
        |f| f.rule("demo-grader.default-fail").labels = Some(Vec::new()),
    ));
    p.push(Probe::new(
        "[authoring] a stub inheriting a label-less block warns, never fails",
        Expect::AdvisoryOn(Code::LabelsInherited, "authoring-demo.envelope"),
        |f| {
            f.retarget("authoring-demo", Some("skill-authoring-common"));
            f.common().blocks[0].labels = None;
        },
    ));

    // The library: absence-meaningful fields stay local (C3), and an override that says nothing.
    p.push(Probe::new(
        "a common block carrying `kind:`",
        Expect::RejectOn(Code::ExtendsClassLocal, "review-common.default-fail"),
        |f| f.common().blocks[0].kind = Some("gate".into()),
    ));
    p.push(Probe::new(
        "a common block carrying `when:`",
        Expect::RejectOn(Code::ExtendsClassLocal, "review-common.default-fail"),
        |f| {
            f.common().blocks[0].when = vec![(
                "depth".into(),
                WhenValue::Scalar(Value::String("deep".into())),
            )]
        },
    ));
    p.push(Probe::new(
        "a common block carrying `enforces:`",
        Expect::RejectOn(Code::ExtendsClassLocal, "review-common.default-fail"),
        |f| f.common().blocks[0].enforces = Some(vec!["demo-grader.never-author".into()]),
    ));
    p.push(Probe::new(
        "a stub whose local text repeats the block's",
        Expect::AdvisoryOn(Code::PointlessOverride, "demo-grader.default-fail"),
        |f| {
            f.rule("demo-grader.default-fail").text = Some(
                "Never default to a clearing verdict — earned only by a completed hunt.".into(),
            )
        },
    ));

    // The sweep-scoped claims. A whole-state validator *is* the sweep, so these have referents —
    // and each guard is probed by the state that would make an unguarded claim wrong.
    p.push(Probe::new(
        "[sweep] a common block bound by no stub in any swept skill",
        Expect::AdvisoryOn(Code::OrphanBlock, "review-common.orphan"),
        |f| {
            f.common().blocks.push(Rule {
                id: "review-common.orphan".into(),
                labels: Some(vec!["verdict".into()]),
                text: Some("No stub binds this block.".into()),
                ..Rule::default()
            })
        },
    ));
    p.push(Probe::new(
        "[sweep] all-bound authoring blocks make no orphan claim and no label claim",
        Expect::CleanOf(Code::OrphanBlock),
        |_| {},
    ));
    p.push(Probe::new(
        "[sweep] no authoring schemas swept makes no authoring orphan claim",
        Expect::CleanOf(Code::OrphanBlock),
        |f| f.drop_skill("authoring-demo"),
    ));
    p.push(Probe::new(
        "[sweep] the sweep makes no patterns orphan claim",
        Expect::CleanOf(Code::OrphanBlock),
        |f| {
            // Only the patterns skill left in state. Its family ships no library by ruling, so
            // neither library in state has a binder and neither can be called orphaned.
            f.drop_skill("demo-grader");
            f.drop_skill("authoring-demo");
        },
    ));
    p.push(Probe::new(
        "[sweep] a label unused by every swept schema is named once, at sweep end",
        Expect::AdvisoryOn(Code::ZeroMemberLabel, "unswept"),
        |f| {
            f.labels()
                .labels
                .push(("unswept".into(), "carried by no skill".into()))
        },
    ));
    p.push(Probe::new(
        "[sweep] labels all carried across the swept schemas make no claim",
        Expect::CleanOf(Code::ZeroMemberLabel),
        |_| {},
    ));

    // The flat grammar sections superseded.
    p.push(Probe::new(
        "a flat top-level rules: key",
        Expect::Reject(Code::FlatRules),
        |f| {
            f.skill_schema().blocks.push(Rule {
                id: "demo-grader.flat".into(),
                labels: Some(vec!["binding".into()]),
                class: Some("must".into()),
                text: Some("A rule outside every section.".into()),
                ..Rule::default()
            })
        },
    ));

    // --- unit 1b: the shipped-checker residual on this side ---
    p.push(Probe::new(
        "the registry carrying no labels mapping",
        Expect::Reject(Code::DocumentEmpty),
        |f| f.labels().labels.clear(),
    ));

    // --- an extra the Python matrix never had ---
    p.push(Probe::extra(
        "a section outside every family's set is reported against this family's",
        Expect::Reject(Code::SectionSet),
        |f| {
            f.skill_schema().sections.push(Section {
                id: "demo-grader.sec.extras".into(),
                title: "Extras".into(),
                intent: "x".into(),
                note: Some("empty on purpose".into()),
                rules: Vec::new(),
            });
        },
    ));

    p
}

// ---------------------------------------------------------------------------
// the ledgers
// ---------------------------------------------------------------------------

const GENESIS_SIDE: &[(&str, &str)] = &[
    ("the schema does not parse as YAML", "`GenesisError::Parse` at import"),
    ("the sidecar absent is a warning, not a finding", "genesis cannot fold anchors without the sidecar, so absence stops the build — a severity change, disclosed"),
    ("a dangling skill-prefixed entry", "`GenesisError::DanglingAnchor`"),
    ("an entry naming a tombstoned rule", "`GenesisError::DanglingAnchor` — a tombstoned id is not a live rule, so the anchor has nothing to ride"),
    ("a foreign-prefix entry is skipped silently", "anchors are matched by rule id across every document, so a foreign prefix lands on its own document rather than being skipped"),
    ("the pre-rename command-provenance kind is accepted", "`genesis::sidecar_anchors` accepts both spellings"),
    ("a kind outside both provenance discriminators", "`GenesisError::SidecarKind`"),
];

const NOT_APPLICABLE: &[(&str, &str)] = &[
    ("the SKILL.md file is missing entirely", "the pair is no longer a file pair; the `SKILL.md` carries no schema content to be missing"),
    ("the Rules heading absent", "the `SKILL.md` scaffold: D6 collapses it to the `!` line, the grant and the halt clause"),
    ("the Rules block omits a live section", "the `SKILL.md` no longer enumerates sections — the CLI renders them"),
    ("the Rules block names a section the schema lacks", "the `SKILL.md` no longer enumerates sections"),
    ("a tombstoned token inside the Rules block gets the tombstone message", "the `SKILL.md` no longer enumerates sections"),
    ("a dangling token outside the Rules block", "prose token resolution over the `SKILL.md` body"),
    ("a tombstoned token in the SKILL.md", "prose token resolution over the `SKILL.md` body"),
    ("a foreign-stem token in the SKILL.md is a warning", "prose token resolution over the `SKILL.md` body"),
    ("an anchor resolving to no DECISIONS.md row", "wave-plan §3 scopes wave 1 to anchor format; resolution stays advisory until the repo path is known"),
];

const OUTSIDE_THE_HARD_SET: &[(&str, &str)] = &[
    (
        "an empty section written as `rules:` rather than `rules: []`",
        "a style warning about YAML spelling; the model reads both the same",
    ),
    (
        "when: written as a list, not a conjunction mapping",
        "a shape error the decoder rejects before the validator sees it",
    ),
    (
        "the coverage report makes no claim over floors",
        "the Rust coverage report is per-value and does not restate the floor carve",
    ),
    (
        "a single-skill run makes no orphan claim",
        "there is no per-skill run: the validator always grades the whole state",
    ),
    (
        "the pin absent entirely",
        "the grammar of a hand-written pin; the count is computed and printed",
    ),
    (
        "the pin plural where the count is 1",
        "the grammar of a hand-written pin",
    ),
    (
        "a second, disagreeing pin elsewhere in the body is caught",
        "the grammar of a hand-written pin",
    ),
    (
        "a single-skill run makes no zero-member label claim",
        "the warning itself is ported, sweep-scoped; what has no referent is the single-skill \
         run — the validator always grades the whole state, so there is no narrower scope for \
         the claim to be withheld from",
    ),
    (
        "[sweep] a schema-less patterns member is never swept, never demanded of",
        "an unconverted skill has no document in state, so nothing demands one of it",
    ),
];

include!("matrix/skill_probe_names.rs");

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
fn the_recorded_census_of_this_matrix_is_stale() {
    // The record and the wave plan say 86. The script's own table says otherwise, and the port
    // is sized by the script.
    assert_eq!(PYTHON_PROBES.len(), 114);
}

#[test]
fn the_skill_matrix_holds() {
    matrix::run("skill", &probes(), baseline);
}
