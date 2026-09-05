//! The shared harness behind `matrix_command.rs` and `matrix_skill.rs`.
//!
//! The Python matrices are tables: one fixture, one mutation per probe, one named expectation.
//! This keeps that shape — a probe is a row, and the runner reports **every** failing row with
//! the findings it actually saw, rather than stopping at the first. A table also stays diffable
//! against the Python list it ports, which is what makes the retirement gate gradeable.

#![allow(dead_code)]

use mochiko_cli::model::{
    Condition, DocKind, DocRef, Document, LabelRegistry, Rule, RuleSchema, Section, WhenValue,
};
use mochiko_cli::replay::State;
use mochiko_cli::validate::{self, Code, Finding};
use serde_norway::Value;
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// the fixture
// ---------------------------------------------------------------------------

/// A synthetic corpus in state: the schema under test, its registry, and its libraries.
pub struct Fixture {
    pub state: State,
    /// The document a probe's mutations address by default.
    pub target: DocRef,
    /// The block library, when the family has one.
    pub library: Option<DocRef>,
    pub registry: DocRef,
    /// The plugin root pointer resolution reads, for the probes that need one.
    ///
    /// `None` by default, and deliberately: pointer resolution is the one check that reads a
    /// tree rather than the store, so a probe that has not asked for a root gets no claim about
    /// its pointers rather than a silent pass.
    pub root: Option<PathBuf>,
}

fn decode(kind: DocKind, name: &str, yaml: &str) -> (DocRef, Document) {
    let value: Value =
        serde_norway::from_str(yaml).unwrap_or_else(|e| panic!("fixture {name} parses: {e}"));
    let document = Document::from_value(kind, &value)
        .unwrap_or_else(|e| panic!("fixture {name} decodes: {e}"));
    (DocRef::new(kind, name), document)
}

impl Fixture {
    /// The command-side fixture: `demo`, its registry, and the `common` library.
    pub fn for_command(schema: &str, labels: &str, common: &str) -> Fixture {
        let mut state = State::default();
        for (kind, name, yaml) in [
            (DocKind::Command, "demo", schema),
            (DocKind::CommandLabels, "command-labels", labels),
            (DocKind::CommandCommon, "common", common),
        ] {
            let (doc, document) = decode(kind, name, yaml);
            state.docs.insert(doc, document);
        }
        Fixture {
            state,
            target: DocRef::new(DocKind::Command, "demo"),
            library: Some(DocRef::new(DocKind::CommandCommon, "common")),
            registry: DocRef::new(DocKind::CommandLabels, "command-labels"),
            root: None,
        }
    }

    /// The skill-side fixture: any set of documents, one named as the target and one as the
    /// family library the probes drop or mutate.
    ///
    /// Deliberately open-ended, because the skill grammar's probes need more than one family in
    /// state at once: cross-family `extends:` is only testable with both libraries present.
    pub fn for_skill(
        target: &'static str,
        docs: &[(DocKind, &'static str, &str)],
        library: Option<&'static str>,
    ) -> Fixture {
        let mut state = State::default();
        let mut registry = DocRef::new(DocKind::SkillLabels, "skill-labels");
        for (kind, name, yaml) in docs {
            let (doc, document) = decode(*kind, name, yaml);
            if *kind == DocKind::SkillLabels {
                registry = doc.clone();
            }
            state.docs.insert(doc, document);
        }
        Fixture {
            state,
            target: DocRef::new(DocKind::Skill, target),
            library: library.map(|name| DocRef::new(DocKind::SkillCommon, name)),
            registry,
            root: None,
        }
    }

    /// Point the probe's mutations at another skill in the fixture, and at its family library.
    pub fn retarget(&mut self, skill: &'static str, library: Option<&'static str>) {
        self.target = DocRef::new(DocKind::Skill, skill);
        self.library = library.map(|name| DocRef::new(DocKind::SkillCommon, name));
    }

    /// The schema under test.
    pub fn command(&mut self) -> &mut RuleSchema {
        self.schema_at(&self.target.clone())
    }

    /// The schema under test, for skill fixtures (the same document, named for the caller).
    pub fn skill_schema(&mut self) -> &mut RuleSchema {
        self.schema_at(&self.target.clone())
    }

    /// Another skill in the fixture, by name.
    pub fn other_skill(&mut self, name: &str) -> &mut RuleSchema {
        self.schema_at(&DocRef::new(DocKind::Skill, name))
    }

    pub fn common(&mut self) -> &mut RuleSchema {
        let library = self.library.clone().expect("this family ships a library");
        self.schema_at(&library)
    }

    fn schema_at(&mut self, doc: &DocRef) -> &mut RuleSchema {
        self.state
            .docs
            .get_mut(doc)
            .unwrap_or_else(|| panic!("{doc} is in the fixture"))
            .as_rules_mut()
            .unwrap_or_else(|| panic!("{doc} carries rules"))
    }

    pub fn labels(&mut self) -> &mut LabelRegistry {
        let registry = self.registry.clone();
        self.state
            .docs
            .get_mut(&registry)
            .expect("the registry is in the fixture")
            .as_labels_mut()
            .expect("the registry carries labels")
    }

    pub fn rule(&mut self, id: &str) -> &mut Rule {
        let target = self.target.clone();
        self.schema_at(&target)
            .find_rule_mut(id)
            .unwrap_or_else(|| panic!("the fixture carries {id}"))
    }

    /// A section of the schema under test, by the trailing slug of its id.
    pub fn section(&mut self, slug: &str) -> &mut Section {
        let target = self.target.clone();
        self.schema_at(&target)
            .sections
            .iter_mut()
            .find(|s| s.id.ends_with(&format!(".sec.{slug}")))
            .unwrap_or_else(|| panic!("the fixture carries a {slug} section"))
    }

    pub fn condition(&mut self, name: &str) -> &mut Condition {
        let target = self.target.clone();
        self.schema_at(&target)
            .conditions
            .iter_mut()
            .find(|(key, _)| key == name)
            .map(|(_, condition)| condition)
            .unwrap_or_else(|| panic!("the fixture declares {name}"))
    }

    pub fn drop_section(&mut self, slug: &str) {
        let target = self.target.clone();
        let needle = format!(".sec.{slug}");
        self.schema_at(&target)
            .sections
            .retain(|s| !s.id.ends_with(&needle));
    }

    pub fn drop_common(&mut self) {
        if let Some(library) = self.library.take() {
            self.state.docs.remove(&library);
        }
    }

    pub fn tombstone(&mut self, id: &str) {
        let target = self.target.clone();
        self.schema_at(&target)
            .tombstones
            .push(mochiko_cli::model::Tombstone {
                id: id.to_string(),
                disposition: "retired at the scaffold wave".into(),
            });
    }

    pub fn push_rule(&mut self, slug: &str, rule: Rule) {
        self.section(slug).rules.push(rule);
    }

    /// A second `kind: fail` rule, the Python's `two_fail_rules` mutation.
    pub fn add_second_fail(&mut self) {
        let prefix = self.target.name.clone();
        self.push_rule(
            "fail-conditions",
            Rule {
                id: format!("{prefix}.fail.no-evidence"),
                labels: Some(vec!["user-gate".into()]),
                class: Some("floor".into()),
                kind: Some("fail".into()),
                enforces: Some(vec![format!("{prefix}.read-first")]),
                text: Some("Closing without evidence fails.".into()),
                ..Rule::default()
            },
        );
    }

    pub fn set_when(&mut self, id: &str, dimension: &str, value: &str) {
        self.rule(id).when = vec![(
            dimension.to_string(),
            WhenValue::Scalar(Value::String(value.to_string())),
        )];
    }

    /// The two figures the render prints as count pins, computed from state.
    pub fn counts(&self) -> (usize, usize) {
        let schema = self
            .state
            .docs
            .get(&self.target)
            .and_then(Document::as_rules)
            .expect("the target carries rules");
        (
            schema.rules().filter(|r| r.is_fail()).count(),
            schema.rules().filter(|r| r.is_floor()).count(),
        )
    }

    pub fn findings(&self) -> Vec<Finding> {
        let mut findings = validate::validate(&self.state);
        if let Some(root) = &self.root {
            findings.extend(validate::validate_pointers(&self.state, root).findings);
        }
        findings
    }

    /// Point this fixture's pointer resolution at a scratch plugin root, and return its path.
    ///
    /// The tree is real because the check reads one: three files under two skill directories and
    /// one at the root, which is the whole vocabulary the five pointer probes need. Writing it is
    /// idempotent, so probes may ask for it in any order.
    pub fn use_pointer_root(&mut self) -> PathBuf {
        let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("matrix-pointers");
        for (dir, file) in [
            ("skills/demo-grader/references", "PRESENT.md"),
            ("skills/authoring-demo/references", "SIBLING.md"),
            ("templates", "AT-ROOT.md"),
        ] {
            let dir = root.join(dir);
            std::fs::create_dir_all(&dir).expect("the scratch plugin root is writable");
            std::fs::write(dir.join(file), "# fixture\n").expect("the fixture file writes");
        }
        self.root = Some(root.clone());
        root
    }

    /// Remove a skill from state, for the probes about what a sweep does *not* claim.
    pub fn drop_skill(&mut self, name: &str) {
        self.state.docs.remove(&DocRef::new(DocKind::Skill, name));
    }
}

// ---------------------------------------------------------------------------
// probes
// ---------------------------------------------------------------------------

/// What a probe expects the validator to say.
#[derive(Clone, Debug)]
pub enum Expect {
    /// No rejecting finding at all.
    Clean,
    /// At least one rejecting finding of this code.
    Reject(Code),
    /// … on this node.
    RejectOn(Code, &'static str),
    /// Clean, and this advisory present.
    Advisory(Code),
    /// … on this node.
    AdvisoryOn(Code, &'static str),
    /// Clean, and this code absent — the other half of a "stays clean" probe.
    CleanOf(Code),
    /// Clean, and no finding whose message carries this fragment. The Python matrix's `absent`
    /// field, which asserts that a check ran and stayed quiet rather than never running.
    CleanAbsent(&'static str),
    /// The computed count pins, which replace the `.md`'s transcribed ones.
    Counts { fails: usize, floors: usize },
}

pub struct Probe {
    pub name: &'static str,
    /// The Python probe this ports, verbatim, or `None` for a probe the matrix never had.
    pub python: Option<&'static str>,
    pub expect: Expect,
    pub mutate: fn(&mut Fixture),
}

impl Probe {
    /// A port whose name is the Python probe's own.
    pub fn new(name: &'static str, expect: Expect, mutate: fn(&mut Fixture)) -> Probe {
        Probe {
            name,
            python: Some(name),
            expect,
            mutate,
        }
    }

    /// A port under a different name, because the Rust surface says it differently.
    pub fn porting(
        python: &'static str,
        name: &'static str,
        expect: Expect,
        mutate: fn(&mut Fixture),
    ) -> Probe {
        Probe {
            name,
            python: Some(python),
            expect,
            mutate,
        }
    }

    /// A probe the Python matrix never had.
    pub fn extra(name: &'static str, expect: Expect, mutate: fn(&mut Fixture)) -> Probe {
        Probe {
            name,
            python: None,
            expect,
            mutate,
        }
    }
}

/// Assert that every Python probe is accounted for exactly once, in exactly one ledger.
///
/// This is the retirement gate's own test: a probe that is neither ported nor consciously
/// dispositioned is a check that would vanish with the script.
pub fn accounted_for(python: &[&str], probes: &[Probe], ledgers: &[(&str, &[(&str, &str)])]) {
    use std::collections::BTreeMap;

    let mut claims: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for probe in probes {
        if let Some(name) = probe.python {
            claims.entry(name).or_default().push("ported".to_string());
        }
    }
    for (ledger, rows) in ledgers {
        for (name, _) in *rows {
            claims.entry(name).or_default().push((*ledger).to_string());
        }
    }

    let mut problems: Vec<String> = Vec::new();
    for name in python {
        match claims.get(name) {
            None => problems.push(format!("  unaccounted: {name:?}")),
            Some(where_) if where_.len() > 1 => problems.push(format!(
                "  claimed {} times ({}): {name:?}",
                where_.len(),
                where_.join(", ")
            )),
            Some(_) => {}
        }
    }
    for name in claims.keys() {
        if !python.contains(name) {
            problems.push(format!("  not a Python probe: {name:?}"));
        }
    }

    assert!(
        problems.is_empty(),
        "the probe ledgers do not account for the Python matrix ({} probes):\n{}",
        python.len(),
        problems.join("\n")
    );
}

/// Run a matrix, reporting every failing probe rather than the first.
pub fn run(matrix: &str, probes: &[Probe], baseline: fn() -> Fixture) {
    let mut failures: Vec<String> = Vec::new();
    for probe in probes {
        let mut fixture = baseline();
        (probe.mutate)(&mut fixture);
        let findings = fixture.findings();
        let rejecting: Vec<&Finding> = findings.iter().filter(|f| f.is_rejecting()).collect();

        let problem = match &probe.expect {
            Expect::Clean => (!rejecting.is_empty())
                .then(|| format!("expected no rejecting finding, got {}", render(&rejecting))),
            Expect::Reject(code) => (!rejecting.iter().any(|f| f.code == *code))
                .then(|| format!("expected {code}, got {}", render(&rejecting))),
            Expect::RejectOn(code, id) => (!rejecting
                .iter()
                .any(|f| f.code == *code && f.id.as_deref() == Some(*id)))
            .then(|| format!("expected {code} on {id}, got {}", render(&rejecting))),
            Expect::Advisory(code) => {
                if !rejecting.is_empty() {
                    Some(format!(
                        "expected an advisory, got rejecting {}",
                        render(&rejecting)
                    ))
                } else {
                    (!findings.iter().any(|f| f.code == *code))
                        .then(|| format!("expected advisory {code}, got none"))
                }
            }
            Expect::AdvisoryOn(code, id) => {
                if !rejecting.is_empty() {
                    Some(format!(
                        "expected an advisory, got rejecting {}",
                        render(&rejecting)
                    ))
                } else {
                    (!findings
                        .iter()
                        .any(|f| f.code == *code && f.id.as_deref() == Some(*id)))
                    .then(|| format!("expected advisory {code} on {id}, got none"))
                }
            }
            Expect::CleanOf(code) => {
                if !rejecting.is_empty() {
                    Some(format!("expected clean, got {}", render(&rejecting)))
                } else {
                    findings
                        .iter()
                        .any(|f| f.code == *code)
                        .then(|| format!("{code} was raised and must not be"))
                }
            }
            Expect::CleanAbsent(fragment) => {
                if !rejecting.is_empty() {
                    Some(format!("expected clean, got {}", render(&rejecting)))
                } else {
                    findings
                        .iter()
                        .find(|f| f.message.contains(fragment))
                        .map(|f| format!("{fragment:?} must not be reported, but {f} was"))
                }
            }
            Expect::Counts { fails, floors } => {
                let (got_fails, got_floors) = fixture.counts();
                (got_fails != *fails || got_floors != *floors).then(|| {
                    format!(
                        "expected {fails} fail / {floors} floor, computed {got_fails} / {got_floors}"
                    )
                })
            }
        };

        if let Some(problem) = problem {
            failures.push(format!("  {} — {problem}", probe.name));
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {} {matrix} probes failed:\n{}",
        failures.len(),
        probes.len(),
        failures.join("\n")
    );
}

fn render(findings: &[&Finding]) -> String {
    if findings.is_empty() {
        return "none".to_string();
    }
    findings
        .iter()
        .map(|f| format!("{} on {}", f.code, f.id.as_deref().unwrap_or("-")))
        .collect::<Vec<_>>()
        .join(", ")
}
