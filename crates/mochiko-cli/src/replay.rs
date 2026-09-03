//! The log loader and the replay engine.
//!
//! Truth is the ordered log; the projection is rebuilt in memory at every invocation. A thousand
//! rules replay in milliseconds, so no cache is warranted until a measured render latency asks
//! for one.
//!
//! # Findings, not early exit
//!
//! An op that cannot apply raises a finding and is skipped, rather than stopping the run. That is
//! what `migrate validate` wants: a maintainer fixing a log should see every problem in it, not
//! the first. It also means a state can exist that no one may render from, so [`Replay`] carries
//! an unambiguous signal — [`Replay::is_deliverable`] — and [`load`] refuses outright, handing
//! back the findings a caller must print before exiting 1.

use crate::migration::{self, Change, Migration, RuleField};
use crate::model::{
    canonical_hash, is_anchor, ordered_get, ordered_remove, ordered_set, Condition, DocKind,
    DocRef, Document, LabelRegistry, RetiredLabel, Rule, RuleSchema, Section, Tombstone, WhenValue,
};
use crate::validate::{Code, Finding};
use serde_norway::{Mapping, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// The replayed corpus.
#[derive(Clone, Debug, Default)]
pub struct State {
    /// Every live document, addressed by kind and name.
    pub docs: BTreeMap<DocRef, Document>,
    /// Every id that has ever existed in a document, live or tombstoned — the mint-once ledger.
    /// It is bookkeeping about history, not schema content, so it sits beside the documents
    /// rather than inside them and stays out of [`State::content_hash`].
    pub minted: BTreeMap<DocRef, BTreeSet<String>>,
}

impl State {
    /// The hash of the corpus's content: every document, canonically encoded, in address order.
    ///
    /// Deliberately over documents alone. This is a view-drift signal — it answers "would a
    /// regenerated view differ?" — so it must not move when nothing renders differently. The
    /// mint-once ledger is excluded for exactly that reason.
    pub fn content_hash(&self) -> String {
        let mut map = Mapping::new();
        for (doc, document) in &self.docs {
            map.insert(Value::String(doc.to_string()), document.to_value());
        }
        canonical_hash(&Value::Mapping(map))
    }

    fn was_minted(&self, doc: &DocRef, id: &str) -> bool {
        self.minted.get(doc).is_some_and(|ids| ids.contains(id))
    }

    fn mint(&mut self, doc: &DocRef, id: &str) {
        self.minted
            .entry(doc.clone())
            .or_default()
            .insert(id.to_string());
    }

    /// Seed the mint-once ledger from an imported document: every id it arrives carrying, live
    /// or tombstoned, counts as minted.
    fn seed_minted(&mut self, doc: &DocRef, document: &Document) {
        if let Some(schema) = document.as_rules() {
            let ids: Vec<String> = schema
                .live_ids()
                .into_iter()
                .chain(schema.tombstones.iter().map(|t| t.id.clone()))
                .collect();
            for id in ids {
                self.mint(doc, &id);
            }
        }
    }
}

/// A replay's outcome: the state it built and everything it found on the way.
#[derive(Clone, Debug, Default)]
pub struct Replay {
    pub state: State,
    pub findings: Vec<Finding>,
    /// The sequence numbers applied, in order.
    sequences: Vec<u32>,
}

impl Replay {
    /// The sequence numbers applied, in order. Gaps are legal; disorder is not.
    pub fn sequences(&self) -> Vec<u32> {
        self.sequences.clone()
    }

    /// Every rejecting finding.
    pub fn rejecting(&self) -> impl Iterator<Item = &Finding> {
        self.findings.iter().filter(|f| f.is_rejecting())
    }

    /// Whether this state may be rendered from.
    ///
    /// A state built while any rejecting finding was raised is unusable for delivery: some op did
    /// not apply, so what is in memory is a partial corpus. Callers on the delivery path check
    /// this and exit 1 rather than render.
    pub fn is_deliverable(&self) -> bool {
        self.rejecting().next().is_none()
    }
}

// ---------------------------------------------------------------------------
// loading
// ---------------------------------------------------------------------------

/// Whether a directory entry is a migration file: `NNNN-<slug>.yaml`.
fn is_migration_file(name: &str) -> bool {
    name.ends_with(".yaml") && migration::filename_sequence(name).is_some()
}

/// Read and replay every migration in `dir`.
///
/// `Err` only when the directory itself cannot be listed; a malformed migration inside it is a
/// finding, so one bad file never hides the rest of the log.
pub fn replay_dir(dir: &Path) -> std::io::Result<Replay> {
    let mut names: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if is_migration_file(&name) {
            names.push(name);
        }
    }
    names.sort();

    let mut findings = Vec::new();
    let mut migrations: Vec<Migration> = Vec::new();
    for name in names {
        let source = match std::fs::read_to_string(dir.join(&name)) {
            Ok(source) => source,
            Err(e) => {
                findings.push(Finding::log(
                    Code::GrammarParse,
                    format!("{name}: cannot be read: {e}"),
                ));
                continue;
            }
        };
        match migration::parse(&name, &source) {
            Ok(parsed) => migrations.push(parsed),
            Err(e) => findings.push(Finding::log(code_of(&e), e.to_string())),
        }
    }

    // Sequence collisions are found before ordering, so the report names both files.
    let mut seen: BTreeMap<u32, String> = BTreeMap::new();
    let mut collided: BTreeSet<u32> = BTreeSet::new();
    for m in &migrations {
        match seen.get(&m.sequence) {
            Some(first) => {
                collided.insert(m.sequence);
                findings.push(Finding::log(
                    Code::SequenceCollision,
                    format!(
                        "sequence {} is claimed by both {first} and {} — the log's order is its \
                         history, and two files cannot share one place in it",
                        m.sequence, m.file
                    ),
                ));
            }
            None => {
                seen.insert(m.sequence, m.file.clone());
            }
        }
    }

    migrations.sort_by_key(|m| (m.sequence, m.file.clone()));
    let mut replay = replay(&migrations);
    // Loader findings lead: a file that never parsed explains the ops that never ran.
    findings.append(&mut replay.findings);
    replay.findings = findings;
    Ok(replay)
}

/// Load a log and refuse an unsound one.
///
/// The delivery path's entry point: `Ok` is a state safe to render from, `Err` the findings the
/// caller prints before exiting 1.
pub fn load(dir: &Path) -> Result<State, Vec<Finding>> {
    match replay_dir(dir) {
        Ok(replay) if replay.is_deliverable() => Ok(replay.state),
        Ok(replay) => Err(replay.findings),
        Err(e) => Err(vec![Finding::log(
            Code::GrammarParse,
            format!("{}: the migration log cannot be read: {e}", dir.display()),
        )]),
    }
}

fn code_of(error: &migration::ParseError) -> Code {
    match error {
        migration::ParseError::Yaml { .. } => Code::GrammarParse,
        migration::ParseError::Header { .. } => Code::GrammarHeader,
        migration::ParseError::SequenceMismatch { .. } => Code::SequenceMismatch,
        migration::ParseError::HashMismatch { .. } => Code::HashMismatch,
        migration::ParseError::GrammarVersion { .. } => Code::GrammarVersion,
        migration::ParseError::Change { .. } => Code::OpUnknown,
    }
}

// ---------------------------------------------------------------------------
// replay
// ---------------------------------------------------------------------------

/// Apply an ordered list of migrations.
pub fn replay(migrations: &[Migration]) -> Replay {
    let mut out = Replay::default();
    for m in migrations {
        out.sequences.push(m.sequence);
        for (index, change) in m.changes.iter().enumerate() {
            if let Err(finding) = apply(&mut out.state, change) {
                out.findings.push(with_origin(finding, &m.file, index));
            }
        }
    }
    out
}

/// Name the migration and the change an apply finding came from, so a maintainer can go
/// straight to the line.
fn with_origin(mut finding: Finding, file: &str, index: usize) -> Finding {
    finding.message = format!("{} (from {file} changes[{index}])", finding.message);
    finding
}

fn inapplicable(doc: &DocRef, id: Option<&str>, message: impl Into<String>) -> Finding {
    Finding::new(
        Code::OpInapplicable,
        Some(doc.clone()),
        id.map(str::to_string),
        message,
    )
}

/// The rule-bearing document a change addresses.
fn schema_of<'a>(state: &'a mut State, doc: &DocRef) -> Result<&'a mut RuleSchema, Finding> {
    match state.docs.get_mut(doc) {
        Some(document) => document
            .as_rules_mut()
            .ok_or_else(|| inapplicable(doc, None, "this document carries no rules")),
        None => Err(inapplicable(doc, None, "no such document in state")),
    }
}

/// The registry a change addresses.
fn registry_of<'a>(state: &'a mut State, doc: &DocRef) -> Result<&'a mut LabelRegistry, Finding> {
    match state.docs.get_mut(doc) {
        Some(document) => document
            .as_labels_mut()
            .ok_or_else(|| inapplicable(doc, None, "this document is not a label registry")),
        None => Err(inapplicable(doc, None, "no such registry in state")),
    }
}

/// Remove one live rule, returning it — the shared half of tombstone, supersede and move.
fn take_rule(schema: &mut RuleSchema, id: &str) -> Option<Rule> {
    for section in &mut schema.sections {
        if let Some(at) = section.rules.iter().position(|r| r.id == id) {
            return Some(section.rules.remove(at));
        }
    }
    schema
        .blocks
        .iter()
        .position(|r| r.id == id)
        .map(|at| schema.blocks.remove(at))
}

fn decode_document(doc: &DocRef, content: &Value) -> Result<Document, Finding> {
    Document::from_value(doc.kind, content).map_err(|e| {
        Finding::new(
            Code::OpInapplicable,
            Some(doc.clone()),
            None,
            format!("the document content does not decode: {e}"),
        )
    })
}

fn apply(state: &mut State, change: &Change) -> Result<(), Finding> {
    let doc = change.doc().clone();
    match change {
        // --- document level ---
        Change::ImportDocument { content, .. } => {
            if state.docs.contains_key(&doc) {
                // Import is genesis semantics. A deliberate overwrite is `replace-document`, and
                // an edit is one of the node-level ops, so the log stays a per-rule history.
                return Err(inapplicable(
                    &doc,
                    None,
                    "already in state — import is how a document enters the log once",
                ));
            }
            let document = decode_document(&doc, content)?;
            state.seed_minted(&doc, &document);
            state.docs.insert(doc, document);
        }
        Change::ReplaceDocument { content, .. } => {
            if !doc.kind.is_replaceable() {
                return Err(inapplicable(
                    &doc,
                    None,
                    format!(
                        "a `{}` document changes one node at a time — wholesale replacement is \
                         reserved for templates and shelf data",
                        doc.kind
                    ),
                ));
            }
            if !state.docs.contains_key(&doc) {
                return Err(inapplicable(&doc, None, "no such document in state"));
            }
            let document = decode_document(&doc, content)?;
            state.docs.insert(doc, document);
        }

        // --- sections ---
        Change::MintSection { section, .. } => {
            let decoded = Document::from_value(DocKind::Command, &section_wrapper(section))
                .ok()
                .and_then(|d| d.as_rules().and_then(|s| s.sections.first().cloned()))
                .ok_or_else(|| inapplicable(&doc, None, "the section does not decode"))?;
            let id = decoded.id.clone();
            if state.was_minted(&doc, &id) {
                return Err(mint_once(&doc, &id));
            }
            let schema = schema_of(state, &doc)?;
            schema.sections.push(Section {
                rules: Vec::new(),
                ..decoded
            });
            state.mint(&doc, &id);
        }
        Change::TombstoneSection {
            id, disposition, ..
        } => {
            let schema = schema_of(state, &doc)?;
            let at = schema
                .sections
                .iter()
                .position(|s| s.id == *id)
                .ok_or_else(|| inapplicable(&doc, Some(id), "no such live section"))?;
            let section = schema.sections.remove(at);
            if !section.rules.is_empty() {
                schema.sections.insert(at, section);
                return Err(inapplicable(
                    &doc,
                    Some(id),
                    "the section still holds rules — move or retire them first, so no rule is \
                     retired implicitly",
                ));
            }
            schema.tombstones.push(Tombstone {
                id: id.clone(),
                disposition: disposition.clone(),
            });
        }

        // --- rules ---
        Change::MintRule { section, rule, .. } => {
            let decoded = decode_rule(&doc, rule)?;
            let id = decoded.id.clone();
            if state.was_minted(&doc, &id) {
                return Err(mint_once(&doc, &id));
            }
            let schema = schema_of(state, &doc)?;
            let target = schema
                .sections
                .iter_mut()
                .find(|s| s.id == *section)
                .ok_or_else(|| inapplicable(&doc, Some(section), "no such live section"))?;
            target.rules.push(decoded);
            state.mint(&doc, &id);
        }
        Change::RewordRule { id, text, .. } => {
            let schema = schema_of(state, &doc)?;
            let rule = schema
                .find_rule_mut(id)
                .ok_or_else(|| inapplicable(&doc, Some(id), "no such live rule"))?;
            rule.text = Some(text.clone());
        }
        Change::SetRuleField {
            id, field, value, ..
        } => {
            let schema = schema_of(state, &doc)?;
            let rule = schema
                .find_rule_mut(id)
                .ok_or_else(|| inapplicable(&doc, Some(id), "no such live rule"))?;
            set_field(&doc, rule, *field, value)?;
        }
        Change::MoveRule { id, section, .. } => {
            let schema = schema_of(state, &doc)?;
            if !schema.sections.iter().any(|s| s.id == *section) {
                return Err(inapplicable(&doc, Some(section), "no such live section"));
            }
            let rule = take_rule(schema, id)
                .ok_or_else(|| inapplicable(&doc, Some(id), "no such live rule"))?;
            let target = schema
                .sections
                .iter_mut()
                .find(|s| s.id == *section)
                .expect("target section was just confirmed live");
            target.rules.push(rule);
        }
        Change::TombstoneRule {
            id, disposition, ..
        } => {
            retire(state, &doc, id, disposition, None)?;
        }
        Change::SupersedeRule {
            id,
            disposition,
            anchor,
            ..
        } => {
            if !is_anchor(anchor) {
                return Err(Finding::new(
                    Code::AnchorFormat,
                    Some(doc.clone()),
                    Some(id.clone()),
                    format!(
                        "anchor {anchor:?} is malformed — want 'YYYY-MM-DD <session-slug> [D#]'"
                    ),
                ));
            }
            retire(state, &doc, id, disposition, Some(anchor))?;
        }

        // --- schema blocks ---
        Change::SetVar { name, value, .. } => {
            let schema = schema_of(state, &doc)?;
            if value.is_null() {
                if !ordered_remove(&mut schema.vars, name) {
                    return Err(inapplicable(
                        &doc,
                        Some(name),
                        "no such declared var to clear",
                    ));
                }
            } else {
                ordered_set(&mut schema.vars, name, value.clone());
            }
        }
        Change::SetCondition { name, spec, .. } => {
            let schema = schema_of(state, &doc)?;
            if spec.is_null() {
                if !ordered_remove(&mut schema.conditions, name) {
                    return Err(inapplicable(
                        &doc,
                        Some(name),
                        "no such declared condition to clear",
                    ));
                }
            } else {
                let map = spec.as_mapping().ok_or_else(|| {
                    inapplicable(&doc, Some(name), "a condition spec is a mapping")
                })?;
                ordered_set(
                    &mut schema.conditions,
                    name,
                    Condition {
                        values: map.get(Value::String("values".into())).cloned(),
                        resolution: map
                            .get(Value::String("resolution".into()))
                            .and_then(Value::as_str)
                            .map(str::to_string),
                        note: map
                            .get(Value::String("note".into()))
                            .and_then(Value::as_str)
                            .map(str::to_string),
                    },
                );
            }
        }
        Change::SetMoment { name, text, .. } => {
            if matches!(doc.kind, DocKind::Skill | DocKind::SkillCommon) {
                return Err(Finding::new(
                    Code::SkillGrammar,
                    Some(doc.clone()),
                    Some(name.clone()),
                    "moments are command grammar — a skill schema declares none",
                ));
            }
            let schema = schema_of(state, &doc)?;
            if text.is_null() {
                if !ordered_remove(&mut schema.moments, name) {
                    return Err(inapplicable(
                        &doc,
                        Some(name),
                        "no such declared moment to clear",
                    ));
                }
            } else {
                let line = text.as_str().ok_or_else(|| {
                    inapplicable(&doc, Some(name), "a moment is one line of text")
                })?;
                ordered_set(&mut schema.moments, name, line.to_string());
            }
        }

        // --- registries ---
        Change::RegistryAdd { label, meaning, .. } => {
            let registry = registry_of(state, &doc)?;
            if ordered_get(&registry.labels, label).is_some() {
                return Err(inapplicable(&doc, Some(label), "already a live label"));
            }
            ordered_set(&mut registry.labels, label, meaning.clone());
        }
        Change::RegistryRetire { label, note, .. } => {
            let registry = registry_of(state, &doc)?;
            if !ordered_remove(&mut registry.labels, label) {
                return Err(inapplicable(
                    &doc,
                    Some(label),
                    "no such live label to retire",
                ));
            }
            // Retiring records; nothing deletes a label, so the vocabulary's history survives.
            registry.retired.push(RetiredLabel {
                label: label.clone(),
                note: note.clone(),
            });
        }
    }
    Ok(())
}

fn mint_once(doc: &DocRef, id: &str) -> Finding {
    Finding::new(
        Code::MintOnce,
        Some(doc.clone()),
        Some(id.to_string()),
        "this id has existed before — an id is minted once and is never reused, live or \
         tombstoned",
    )
}

/// Retire one rule. `anchor` is `Some` for a supersession; a bare tombstone may not take
/// protected content.
fn retire(
    state: &mut State,
    doc: &DocRef,
    id: &str,
    disposition: &str,
    anchor: Option<&str>,
) -> Result<(), Finding> {
    let schema = schema_of(state, doc)?;
    let rule = schema
        .find_rule(id)
        .ok_or_else(|| inapplicable(doc, Some(id), "no such live rule"))?;
    if anchor.is_none() && rule.is_protected() {
        let why = if rule.is_floor() {
            "a `class: floor` rule"
        } else if rule.is_fail() {
            "a `kind: fail` rule"
        } else {
            "an anchored rule"
        };
        return Err(Finding::new(
            Code::ProtectedExit,
            Some(doc.clone()),
            Some(id.to_string()),
            format!(
                "{why} leaves only by `supersede-rule` carrying a ruling anchor — protected \
                 content never leaves silently"
            ),
        ));
    }
    take_rule(schema, id);
    let disposition = match anchor {
        Some(anchor) => format!("{disposition} ({anchor})"),
        None => disposition.to_string(),
    };
    schema.tombstones.push(Tombstone {
        id: id.to_string(),
        disposition,
    });
    Ok(())
}

fn set_field(
    doc: &DocRef,
    rule: &mut Rule,
    field: RuleField,
    value: &Value,
) -> Result<(), Finding> {
    let clearing = value.is_null();
    let bad = |want: &str| inapplicable(doc, Some(&rule.id), format!("`{field}` wants {want}"));
    match field {
        RuleField::Labels => {
            rule.labels = if clearing {
                None
            } else {
                Some(string_list(value).ok_or_else(|| bad("a list of label names"))?)
            }
        }
        RuleField::Enforces => {
            rule.enforces = if clearing {
                None
            } else {
                Some(string_list(value).ok_or_else(|| bad("a list of rule ids"))?)
            }
        }
        RuleField::When => {
            rule.when = if clearing {
                Vec::new()
            } else {
                let map = value
                    .as_mapping()
                    .ok_or_else(|| bad("a mapping of dimension → value"))?;
                map.iter()
                    .map(|(k, v)| {
                        let name = k.as_str().unwrap_or_default().to_string();
                        let term = match v {
                            Value::Sequence(items) => WhenValue::List(items.clone()),
                            other => WhenValue::Scalar(other.clone()),
                        };
                        (name, term)
                    })
                    .collect()
            }
        }
        RuleField::Class
        | RuleField::Kind
        | RuleField::Pointer
        | RuleField::Extends
        | RuleField::Anchor
        | RuleField::Note => {
            let text = if clearing {
                None
            } else {
                Some(
                    value
                        .as_str()
                        .map(str::to_string)
                        .ok_or_else(|| bad("a text value"))?,
                )
            };
            match field {
                RuleField::Class => rule.class = text,
                RuleField::Kind => rule.kind = text,
                RuleField::Pointer => rule.pointer = text,
                RuleField::Extends => rule.extends = text,
                RuleField::Anchor => rule.anchor = text,
                _ => rule.note = text,
            }
        }
    }
    Ok(())
}

fn string_list(value: &Value) -> Option<Vec<String>> {
    Some(
        value
            .as_sequence()?
            .iter()
            .map(|v| v.as_str().unwrap_or_default().to_string())
            .collect(),
    )
}

fn decode_rule(doc: &DocRef, value: &Value) -> Result<Rule, Finding> {
    // One decoder for rules everywhere: wrap the rule in the shape `Document::from_value` reads,
    // so a minted rule and an imported one can never diverge in how they are understood.
    let wrapper = section_wrapper(&section_with(value));
    Document::from_value(DocKind::Command, &wrapper)
        .ok()
        .and_then(|d| {
            d.as_rules()
                .and_then(|s| s.sections.first())
                .and_then(|s| s.rules.first())
                .cloned()
        })
        .ok_or_else(|| inapplicable(doc, None, "the rule does not decode"))
}

/// Wrap one section value in a minimal document so the shared decoder reads it.
fn section_wrapper(section: &Value) -> Value {
    let mut map = Mapping::new();
    map.insert(
        Value::String("sections".into()),
        Value::Sequence(vec![section.clone()]),
    );
    Value::Mapping(map)
}

/// Wrap one rule value in a minimal section.
fn section_with(rule: &Value) -> Value {
    let mut map = Mapping::new();
    map.insert(Value::String("id".into()), Value::String(String::new()));
    map.insert(
        Value::String("rules".into()),
        Value::Sequence(vec![rule.clone()]),
    );
    Value::Mapping(map)
}
