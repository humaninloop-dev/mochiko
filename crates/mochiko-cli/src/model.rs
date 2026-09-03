//! The typed document model, its lossless YAML round trip, and the canonical encoder.
//!
//! # Permissive parse, strict validate
//!
//! Every decode in this module is deliberately permissive about *values* and strict only about
//! *shapes*. An unrecognised `class:`, an undeclared `when:` dimension, or a rule with no `text:`
//! all decode cleanly and are reported by [`crate::validate`] as findings. The reason is the
//! ported negative-test matrix: a probe asserting the finding "class must be floor|must|advisory
//! (got 'x')" is unreproducible if the decoder rejects `x` first. Structural failures — a
//! `sections:` key that is not a list, a rule that is not a mapping — are the only decode errors.
//!
//! # Lossless round trip
//!
//! [`Document::from_value`] and [`Document::to_value`] are inverses over every shipped schema
//! file. That is asserted directly (`tests/validate.rs`), not assumed, because three separate
//! guarantees rest on it: the replayed state hash, the genesis fidelity fixture, and the derived
//! views whose semantic equality with the shipped files is the wave's bridge. Two shapes the
//! corpus uses in both forms are therefore preserved exactly rather than normalised:
//!
//! * a `when:` term's value, which is a scalar 77 times and a list 19 times;
//! * a condition's `values:`, which is the word `presence` 25 times and a list 20 times.
//!
//! Declaration order is preserved for `vars:`, `conditions:`, `moments:` and a registry's
//! `labels:` (hence [`Ordered`] rather than a sorted map), so a regenerated view can keep the
//! corpus's key order. Canonical hashing sorts mapping keys regardless, so ordering never affects
//! equality — only the shape of the emitted file.

use serde_norway::value::TaggedValue;
use serde_norway::{Mapping, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fmt::{self, Write as _};

/// An insertion-ordered association list. The blocks it holds carry at most a dozen entries, so
/// linear lookup costs nothing and declaration order survives the round trip.
pub type Ordered<T> = Vec<(String, T)>;

/// Look one key up in an [`Ordered`] list.
pub fn ordered_get<'a, T>(items: &'a Ordered<T>, key: &str) -> Option<&'a T> {
    items.iter().find(|(k, _)| k == key).map(|(_, v)| v)
}

/// Insert or replace one key in an [`Ordered`] list, keeping first-declaration position.
pub fn ordered_set<T>(items: &mut Ordered<T>, key: &str, value: T) {
    match items.iter_mut().find(|(k, _)| k == key) {
        Some(slot) => slot.1 = value,
        None => items.push((key.to_string(), value)),
    }
}

/// Remove one key from an [`Ordered`] list, reporting whether it was there.
pub fn ordered_remove<T>(items: &mut Ordered<T>, key: &str) -> bool {
    let before = items.len();
    items.retain(|(k, _)| k != key);
    items.len() != before
}

// ---------------------------------------------------------------------------
// document identity
// ---------------------------------------------------------------------------

/// The eight document kinds the store holds. The four rule-bearing kinds decode to a
/// [`RuleSchema`], the two registries to a [`LabelRegistry`], and templates and shelf data stay
/// opaque YAML — the binary renders them but owns no grammar for them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DocKind {
    Command,
    Skill,
    CommandCommon,
    SkillCommon,
    CommandLabels,
    SkillLabels,
    Template,
    Shelf,
}

impl DocKind {
    /// Every kind, for exhaustive iteration in tests and reports.
    pub const ALL: [DocKind; 8] = [
        DocKind::Command,
        DocKind::Skill,
        DocKind::CommandCommon,
        DocKind::SkillCommon,
        DocKind::CommandLabels,
        DocKind::SkillLabels,
        DocKind::Template,
        DocKind::Shelf,
    ];

    /// The kind's wire spelling — the token a migration file writes.
    pub fn as_str(self) -> &'static str {
        match self {
            DocKind::Command => "command",
            DocKind::Skill => "skill",
            DocKind::CommandCommon => "command-common",
            DocKind::SkillCommon => "skill-common",
            DocKind::CommandLabels => "command-labels",
            DocKind::SkillLabels => "skill-labels",
            DocKind::Template => "template",
            DocKind::Shelf => "shelf",
        }
    }

    /// Parse a kind token. Unknown tokens are `None` — the caller raises the discriminator finding.
    pub fn parse(token: &str) -> Option<DocKind> {
        DocKind::ALL.into_iter().find(|k| k.as_str() == token)
    }

    /// Whether documents of this kind carry rules, sections and tombstones.
    pub fn is_rule_bearing(self) -> bool {
        matches!(
            self,
            DocKind::Command | DocKind::Skill | DocKind::CommandCommon | DocKind::SkillCommon
        )
    }

    /// Whether this kind is a label registry.
    pub fn is_registry(self) -> bool {
        matches!(self, DocKind::CommandLabels | DocKind::SkillLabels)
    }

    /// Whether `replace-document` is legal for this kind. Rule-bearing documents and registries
    /// change one addressable node at a time so the log stays a per-rule history; wholesale
    /// replacement is reserved for the two kinds the store carries opaquely.
    pub fn is_replaceable(self) -> bool {
        matches!(self, DocKind::Template | DocKind::Shelf)
    }

    /// The label registry a document of this kind draws its labels from.
    pub fn registry(self) -> Option<DocKind> {
        match self {
            DocKind::Command | DocKind::CommandCommon => Some(DocKind::CommandLabels),
            DocKind::Skill | DocKind::SkillCommon => Some(DocKind::SkillLabels),
            _ => None,
        }
    }
}

impl fmt::Display for DocKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A document's address in state: its kind plus its name.
///
/// The wire form is `<kind>/<name>` (`command/specify`, `skill-common/skill-review-common`). A
/// bare `<kind>` means the name equals the kind, which is how the two singleton registries are
/// written (`command-labels`).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DocRef {
    pub kind: DocKind,
    pub name: String,
}

impl DocRef {
    pub fn new(kind: DocKind, name: impl Into<String>) -> DocRef {
        DocRef {
            kind,
            name: name.into(),
        }
    }

    /// Parse a `<kind>/<name>` reference. `None` when the kind token is unknown.
    pub fn parse(text: &str) -> Option<DocRef> {
        match text.split_once('/') {
            Some((kind, name)) => Some(DocRef::new(DocKind::parse(kind)?, name)),
            None => {
                let kind = DocKind::parse(text)?;
                Some(DocRef::new(kind, text))
            }
        }
    }
}

impl fmt::Display for DocRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.kind, self.name)
    }
}

// ---------------------------------------------------------------------------
// rule-level vocabulary
// ---------------------------------------------------------------------------

/// A rule's bindingness. `Other` keeps an unrecognised spelling so the validator can name it and
/// the round trip can write it back unchanged.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Class {
    Floor,
    Must,
    Advisory,
    Other(String),
}

impl Class {
    pub fn parse(token: &str) -> Class {
        match token {
            "floor" => Class::Floor,
            "must" => Class::Must,
            "advisory" => Class::Advisory,
            other => Class::Other(other.to_string()),
        }
    }

    pub fn is_known(&self) -> bool {
        !matches!(self, Class::Other(_))
    }
}

/// The rule-kind vocabulary. `fail` is command-side only; a skill schema carrying it is a
/// finding, never a decode error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuleKind {
    Constraint,
    Duty,
    Gate,
    Reservation,
    Binding,
    Bound,
    Routing,
    Fail,
    Latitude,
    Other(String),
}

impl RuleKind {
    pub fn parse(token: &str) -> RuleKind {
        match token {
            "constraint" => RuleKind::Constraint,
            "duty" => RuleKind::Duty,
            "gate" => RuleKind::Gate,
            "reservation" => RuleKind::Reservation,
            "binding" => RuleKind::Binding,
            "bound" => RuleKind::Bound,
            "routing" => RuleKind::Routing,
            "fail" => RuleKind::Fail,
            "latitude" => RuleKind::Latitude,
            other => RuleKind::Other(other.to_string()),
        }
    }

    /// The kinds legal in a command schema.
    pub const COMMAND_KINDS: [&'static str; 9] = [
        "constraint",
        "duty",
        "gate",
        "reservation",
        "binding",
        "bound",
        "routing",
        "fail",
        "latitude",
    ];

    /// The kinds legal in a skill schema — the command set minus `fail`, which the skill-side
    /// census retired along with its only carrier, `enforces:`.
    pub const SKILL_KINDS: [&'static str; 8] = [
        "constraint",
        "duty",
        "gate",
        "reservation",
        "binding",
        "bound",
        "routing",
        "latitude",
    ];
}

/// A condition's resolution point.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Resolution {
    EntryDerived,
    SurfacePresence,
    UserRuled,
    StandingTrigger,
    /// `moment-resolved(<moment>)` — command grammar only.
    MomentResolved(String),
    Other(String),
}

impl Resolution {
    pub fn parse(token: &str) -> Resolution {
        match token.trim() {
            "entry-derived" => Resolution::EntryDerived,
            "surface-presence" => Resolution::SurfacePresence,
            "user-ruled" => Resolution::UserRuled,
            "standing-trigger" => Resolution::StandingTrigger,
            other => match moment_name(other) {
                Some(moment) => Resolution::MomentResolved(moment.to_string()),
                None => Resolution::Other(other.to_string()),
            },
        }
    }
}

/// The moment inside a well-formed `moment-resolved(<moment>)` token.
fn moment_name(token: &str) -> Option<&str> {
    let inner = token.strip_prefix("moment-resolved(")?.strip_suffix(')')?;
    is_slug(inner).then_some(inner)
}

/// A `when:` term's value, keeping the corpus's scalar-or-list shape exactly as written.
#[derive(Clone, Debug, PartialEq)]
pub enum WhenValue {
    Scalar(Value),
    List(Vec<Value>),
}

impl WhenValue {
    /// The term's values, in order, whichever shape it was written in.
    pub fn values(&self) -> Vec<&Value> {
        match self {
            WhenValue::Scalar(v) => vec![v],
            WhenValue::List(items) => items.iter().collect(),
        }
    }

    fn to_value(&self) -> Value {
        match self {
            WhenValue::Scalar(v) => v.clone(),
            WhenValue::List(items) => Value::Sequence(items.clone()),
        }
    }
}

/// One rule, or one common-library block (a block is a rule with no `class:` of its own).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Rule {
    pub id: String,
    /// `None` is an absent `labels:` key; `Some(vec![])` an explicitly empty one.
    pub labels: Option<Vec<String>>,
    pub class: Option<String>,
    /// An absent `kind:` reads `constraint` and is never written, so absence stays meaningful.
    pub kind: Option<String>,
    pub text: Option<String>,
    pub when: Ordered<WhenValue>,
    pub pointer: Option<String>,
    pub extends: Option<String>,
    /// `None` is an absent `enforces:`; `Some(vec![])` the explicitly empty mirror, which is
    /// legal only alongside a `note:`.
    pub enforces: Option<Vec<String>>,
    /// The reason for an empty `enforces:`. In the YAML corpus this is a `# D6 empty-with-reason:`
    /// comment above the field; comments do not survive the model, so the migration grammar
    /// carries it as data.
    pub note: Option<String>,
    /// The ruling anchor, folded onto the rule from the provenance sidecar at genesis. Present
    /// only in the maintainer build profile; the shipped log carries runtime content alone.
    pub anchor: Option<String>,
}

impl Rule {
    /// The rule's effective kind, applying the `constraint` default.
    pub fn effective_kind(&self) -> RuleKind {
        match &self.kind {
            Some(token) => RuleKind::parse(token),
            None => RuleKind::Constraint,
        }
    }

    pub fn is_fail(&self) -> bool {
        self.effective_kind() == RuleKind::Fail
    }

    pub fn class_of(&self) -> Option<Class> {
        self.class.as_deref().map(Class::parse)
    }

    pub fn is_floor(&self) -> bool {
        self.class_of() == Some(Class::Floor)
    }

    /// Whether this rule may leave its schema only by a ruling-anchored supersession: a floor
    /// rule, a fail rule, or a rule already carrying an anchor.
    pub fn is_protected(&self) -> bool {
        self.is_floor() || self.is_fail() || self.anchor.is_some()
    }
}

/// One section of a rule-bearing schema.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Section {
    pub id: String,
    pub title: String,
    pub intent: String,
    /// The one-line marker a deliberately empty section carries.
    pub note: Option<String>,
    pub rules: Vec<Rule>,
}

/// A retired node and the disposition explaining where it went.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tombstone {
    pub id: String,
    pub disposition: String,
}

/// One declared condition dimension. `values` stays a raw [`Value`] so `presence` and a closed
/// list round-trip in the shape the file wrote them.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Condition {
    pub values: Option<Value>,
    pub resolution: Option<String>,
    pub note: Option<String>,
}

/// How a condition's `values:` reads.
#[derive(Clone, Debug, PartialEq)]
pub enum Values {
    /// The word `presence` — the dimension's two poles are `present` and `absent`.
    Presence,
    Closed(Vec<Value>),
    Malformed,
}

impl Condition {
    pub fn value_kind(&self) -> Values {
        match &self.values {
            Some(Value::String(s)) if s.trim().eq_ignore_ascii_case("presence") => Values::Presence,
            Some(Value::Sequence(items)) if !items.is_empty() => Values::Closed(items.clone()),
            _ => Values::Malformed,
        }
    }

    pub fn is_presence(&self) -> bool {
        self.value_kind() == Values::Presence
    }

    /// The dimension's legal tokens, canonicalised the way a `when:` term canonicalises.
    pub fn tokens(&self) -> BTreeSet<String> {
        match self.value_kind() {
            Values::Presence => ["present", "absent"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            Values::Closed(items) => items.iter().map(|v| norm_value(v, false)).collect(),
            Values::Malformed => BTreeSet::new(),
        }
    }

    pub fn resolution_of(&self) -> Option<Resolution> {
        self.resolution.as_deref().map(Resolution::parse)
    }
}

/// One rule-bearing document: a command schema, a skill schema, or a family common library.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RuleSchema {
    /// The document's own `kind:` field, kept raw so the validator can compare it with the
    /// address the migration filed it under.
    pub declared_kind: Option<String>,
    /// The `command:` or `skill:` name field. Common libraries and registries carry none.
    pub declared_name: Option<String>,
    pub vars: Ordered<Value>,
    pub conditions: Ordered<Condition>,
    pub moments: Ordered<String>,
    pub sections: Vec<Section>,
    pub tombstones: Vec<Tombstone>,
    /// A common library's blocks, which sit at the document's top level rather than in sections.
    pub blocks: Vec<Rule>,
}

impl RuleSchema {
    /// Every live rule, sections in order then blocks.
    pub fn rules(&self) -> impl Iterator<Item = &Rule> {
        self.sections
            .iter()
            .flat_map(|s| s.rules.iter())
            .chain(self.blocks.iter())
    }

    /// Every live rule with the section id it sits in (`None` for a common-library block).
    pub fn rules_with_section(&self) -> impl Iterator<Item = (&Rule, Option<&str>)> {
        self.sections
            .iter()
            .flat_map(|s| s.rules.iter().map(move |r| (r, Some(s.id.as_str()))))
            .chain(self.blocks.iter().map(|r| (r, None)))
    }

    pub fn find_rule(&self, id: &str) -> Option<&Rule> {
        self.rules().find(|r| r.id == id)
    }

    pub fn find_rule_mut(&mut self, id: &str) -> Option<&mut Rule> {
        self.sections
            .iter_mut()
            .flat_map(|s| s.rules.iter_mut())
            .chain(self.blocks.iter_mut())
            .find(|r| r.id == id)
    }

    pub fn find_section(&self, id: &str) -> Option<&Section> {
        self.sections.iter().find(|s| s.id == id)
    }

    /// Every id the document currently holds live — sections, rules and blocks.
    pub fn live_ids(&self) -> BTreeSet<String> {
        self.sections
            .iter()
            .map(|s| s.id.clone())
            .chain(self.rules().map(|r| r.id.clone()))
            .collect()
    }

    pub fn is_tombstoned(&self, id: &str) -> bool {
        self.tombstones.iter().any(|t| t.id == id)
    }
}

/// A label registry: the vocabulary a family's rules draw from.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LabelRegistry {
    pub declared_kind: Option<String>,
    pub labels: Ordered<String>,
    /// Labels withdrawn by ruling. `registry-retire` moves a label here; nothing deletes one.
    pub retired: Vec<RetiredLabel>,
}

/// One withdrawn label and the note recording why.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RetiredLabel {
    pub label: String,
    pub note: String,
}

/// One document in state.
#[derive(Clone, Debug, PartialEq)]
pub enum Document {
    Rules(RuleSchema),
    Labels(LabelRegistry),
    /// Templates and shelf data: carried verbatim, with no grammar of their own.
    Opaque(Value),
}

impl Document {
    pub fn as_rules(&self) -> Option<&RuleSchema> {
        match self {
            Document::Rules(schema) => Some(schema),
            _ => None,
        }
    }

    pub fn as_rules_mut(&mut self) -> Option<&mut RuleSchema> {
        match self {
            Document::Rules(schema) => Some(schema),
            _ => None,
        }
    }

    pub fn as_labels(&self) -> Option<&LabelRegistry> {
        match self {
            Document::Labels(registry) => Some(registry),
            _ => None,
        }
    }

    pub fn as_labels_mut(&mut self) -> Option<&mut LabelRegistry> {
        match self {
            Document::Labels(registry) => Some(registry),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// decoding
// ---------------------------------------------------------------------------

/// A structural decode failure. Value-level problems are findings, never errors — see the module
/// note on permissive parse.
#[derive(Clone, Debug, PartialEq)]
pub struct DecodeError {
    /// Dotted path to the offending node, e.g. `sections[2].rules[0].labels`.
    pub path: String,
    pub message: String,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path, self.message)
    }
}

impl std::error::Error for DecodeError {}

fn err(path: impl Into<String>, message: impl Into<String>) -> DecodeError {
    DecodeError {
        path: path.into(),
        message: message.into(),
    }
}

/// A scalar rendered as a string. Mappings and sequences are structural errors; every scalar
/// spelling round-trips as itself because the corpus writes these fields as strings throughout.
fn scalar_string(value: &Value, path: &str) -> Result<String, DecodeError> {
    match value {
        Value::String(s) => Ok(s.clone()),
        Value::Bool(b) => Ok(b.to_string()),
        Value::Number(n) => Ok(n.to_string()),
        Value::Null => Ok(String::new()),
        _ => Err(err(path, "expected a scalar")),
    }
}

fn opt_scalar(map: &Mapping, key: &str, path: &str) -> Result<Option<String>, DecodeError> {
    match map.get(Value::String(key.to_string())) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => Ok(Some(scalar_string(value, &format!("{path}.{key}"))?)),
    }
}

fn opt_string_list(
    map: &Mapping,
    key: &str,
    path: &str,
) -> Result<Option<Vec<String>>, DecodeError> {
    match map.get(Value::String(key.to_string())) {
        None => Ok(None),
        Some(Value::Null) => Ok(None),
        Some(Value::Sequence(items)) => {
            let mut out = Vec::with_capacity(items.len());
            for (i, item) in items.iter().enumerate() {
                out.push(scalar_string(item, &format!("{path}.{key}[{i}]"))?);
            }
            Ok(Some(out))
        }
        Some(_) => Err(err(format!("{path}.{key}"), "expected a list")),
    }
}

fn as_mapping<'a>(value: &'a Value, path: &str) -> Result<&'a Mapping, DecodeError> {
    value
        .as_mapping()
        .ok_or_else(|| err(path, "expected a mapping"))
}

fn ordered_of<T, F>(
    map: &Mapping,
    key: &str,
    path: &str,
    mut decode: F,
) -> Result<Ordered<T>, DecodeError>
where
    F: FnMut(&Value, &str) -> Result<T, DecodeError>,
{
    let Some(block) = map.get(Value::String(key.to_string())) else {
        return Ok(Vec::new());
    };
    if matches!(block, Value::Null) {
        return Ok(Vec::new());
    }
    let block = as_mapping(block, &format!("{path}.{key}"))?;
    let mut out = Vec::with_capacity(block.len());
    for (name, value) in block {
        let name = scalar_string(name, &format!("{path}.{key}"))?;
        let child = format!("{path}.{key}.{name}");
        out.push((name, decode(value, &child)?));
    }
    Ok(out)
}

fn decode_when(map: &Mapping, path: &str) -> Result<Ordered<WhenValue>, DecodeError> {
    ordered_of(map, "when", path, |value, _| {
        Ok(match value {
            Value::Sequence(items) => WhenValue::List(items.clone()),
            other => WhenValue::Scalar(other.clone()),
        })
    })
}

fn decode_rule(value: &Value, path: &str) -> Result<Rule, DecodeError> {
    let map = as_mapping(value, path)?;
    Ok(Rule {
        id: opt_scalar(map, "id", path)?.unwrap_or_default(),
        labels: opt_string_list(map, "labels", path)?,
        class: opt_scalar(map, "class", path)?,
        kind: opt_scalar(map, "kind", path)?,
        text: opt_scalar(map, "text", path)?,
        when: decode_when(map, path)?,
        pointer: opt_scalar(map, "pointer", path)?,
        extends: opt_scalar(map, "extends", path)?,
        enforces: match map.get(Value::String("enforces".into())) {
            None => None,
            Some(Value::Null) => Some(Vec::new()),
            Some(Value::Sequence(items)) => {
                let mut out = Vec::with_capacity(items.len());
                for (i, item) in items.iter().enumerate() {
                    out.push(scalar_string(item, &format!("{path}.enforces[{i}]"))?);
                }
                Some(out)
            }
            Some(_) => return Err(err(format!("{path}.enforces"), "expected a list")),
        },
        note: opt_scalar(map, "note", path)?,
        anchor: opt_scalar(map, "anchor", path)?,
    })
}

fn decode_section(value: &Value, path: &str) -> Result<Section, DecodeError> {
    let map = as_mapping(value, path)?;
    let rules = match map.get(Value::String("rules".into())) {
        None | Some(Value::Null) => Vec::new(),
        Some(Value::Sequence(items)) => {
            let mut out = Vec::with_capacity(items.len());
            for (i, item) in items.iter().enumerate() {
                out.push(decode_rule(item, &format!("{path}.rules[{i}]"))?);
            }
            out
        }
        Some(_) => return Err(err(format!("{path}.rules"), "expected a list")),
    };
    Ok(Section {
        id: opt_scalar(map, "id", path)?.unwrap_or_default(),
        title: opt_scalar(map, "title", path)?.unwrap_or_default(),
        intent: opt_scalar(map, "intent", path)?.unwrap_or_default(),
        note: opt_scalar(map, "note", path)?,
        rules,
    })
}

fn decode_condition(value: &Value, path: &str) -> Result<Condition, DecodeError> {
    let map = as_mapping(value, path)?;
    Ok(Condition {
        values: map.get(Value::String("values".into())).cloned(),
        resolution: opt_scalar(map, "resolution", path)?,
        note: opt_scalar(map, "note", path)?,
    })
}

impl Document {
    /// Decode one document of the given kind from its YAML value.
    pub fn from_value(kind: DocKind, value: &Value) -> Result<Document, DecodeError> {
        if !kind.is_rule_bearing() && !kind.is_registry() {
            return Ok(Document::Opaque(value.clone()));
        }
        let map = as_mapping(value, kind.as_str())?;
        let declared_kind = opt_scalar(map, "kind", kind.as_str())?;

        if kind.is_registry() {
            let labels = ordered_of(map, "labels", kind.as_str(), |value, path| {
                scalar_string(value, path)
            })?;
            let mut retired = Vec::new();
            if let Some(Value::Sequence(items)) = map.get(Value::String("retired".into())) {
                for (i, item) in items.iter().enumerate() {
                    let path = format!("retired[{i}]");
                    let entry = as_mapping(item, &path)?;
                    retired.push(RetiredLabel {
                        label: opt_scalar(entry, "label", &path)?.unwrap_or_default(),
                        note: opt_scalar(entry, "note", &path)?.unwrap_or_default(),
                    });
                }
            }
            return Ok(Document::Labels(LabelRegistry {
                declared_kind,
                labels,
                retired,
            }));
        }

        let name_key = match kind {
            DocKind::Command => Some("command"),
            DocKind::Skill => Some("skill"),
            _ => None,
        };
        let declared_name = match name_key {
            Some(key) => opt_scalar(map, key, kind.as_str())?,
            None => None,
        };

        let mut sections = Vec::new();
        if let Some(block) = map.get(Value::String("sections".into())) {
            let items = block
                .as_sequence()
                .ok_or_else(|| err("sections", "expected a list"))?;
            for (i, item) in items.iter().enumerate() {
                sections.push(decode_section(item, &format!("sections[{i}]"))?);
            }
        }

        let mut blocks = Vec::new();
        if let Some(block) = map.get(Value::String("rules".into())) {
            let items = block
                .as_sequence()
                .ok_or_else(|| err("rules", "expected a list"))?;
            for (i, item) in items.iter().enumerate() {
                blocks.push(decode_rule(item, &format!("rules[{i}]"))?);
            }
        }

        let mut tombstones = Vec::new();
        if let Some(block) = map.get(Value::String("tombstones".into())) {
            let items = block
                .as_sequence()
                .ok_or_else(|| err("tombstones", "expected a list"))?;
            for (i, item) in items.iter().enumerate() {
                let path = format!("tombstones[{i}]");
                let entry = as_mapping(item, &path)?;
                tombstones.push(Tombstone {
                    id: opt_scalar(entry, "id", &path)?.unwrap_or_default(),
                    disposition: opt_scalar(entry, "disposition", &path)?.unwrap_or_default(),
                });
            }
        }

        Ok(Document::Rules(RuleSchema {
            declared_kind,
            declared_name,
            vars: ordered_of(map, "vars", kind.as_str(), |value, _| Ok(value.clone()))?,
            conditions: ordered_of(map, "conditions", kind.as_str(), decode_condition)?,
            moments: ordered_of(map, "moments", kind.as_str(), |value, path| {
                scalar_string(value, path)
            })?,
            sections,
            tombstones,
            blocks,
        }))
    }
}

// ---------------------------------------------------------------------------
// encoding back to YAML
// ---------------------------------------------------------------------------

fn put(map: &mut Mapping, key: &str, value: Value) {
    map.insert(Value::String(key.to_string()), value);
}

fn put_opt(map: &mut Mapping, key: &str, value: &Option<String>) {
    if let Some(text) = value {
        put(map, key, Value::String(text.clone()));
    }
}

fn strings(items: &[String]) -> Value {
    Value::Sequence(items.iter().map(|s| Value::String(s.clone())).collect())
}

fn ordered_value<T>(items: &Ordered<T>, encode: impl Fn(&T) -> Value) -> Value {
    let mut map = Mapping::new();
    for (name, value) in items {
        map.insert(Value::String(name.clone()), encode(value));
    }
    Value::Mapping(map)
}

impl Rule {
    /// The rule as YAML, in the corpus's field order.
    pub fn to_value(&self) -> Value {
        let mut map = Mapping::new();
        put(&mut map, "id", Value::String(self.id.clone()));
        if let Some(labels) = &self.labels {
            put(&mut map, "labels", strings(labels));
        }
        put_opt(&mut map, "class", &self.class);
        put_opt(&mut map, "kind", &self.kind);
        put_opt(&mut map, "text", &self.text);
        if !self.when.is_empty() {
            put(
                &mut map,
                "when",
                ordered_value(&self.when, WhenValue::to_value),
            );
        }
        put_opt(&mut map, "extends", &self.extends);
        put_opt(&mut map, "pointer", &self.pointer);
        if let Some(enforces) = &self.enforces {
            put(&mut map, "enforces", strings(enforces));
        }
        put_opt(&mut map, "note", &self.note);
        put_opt(&mut map, "anchor", &self.anchor);
        Value::Mapping(map)
    }
}

impl Section {
    pub fn to_value(&self) -> Value {
        let mut map = Mapping::new();
        put(&mut map, "id", Value::String(self.id.clone()));
        put(&mut map, "title", Value::String(self.title.clone()));
        put(&mut map, "intent", Value::String(self.intent.clone()));
        put_opt(&mut map, "note", &self.note);
        put(
            &mut map,
            "rules",
            Value::Sequence(self.rules.iter().map(Rule::to_value).collect()),
        );
        Value::Mapping(map)
    }
}

impl Condition {
    pub fn to_value(&self) -> Value {
        let mut map = Mapping::new();
        if let Some(values) = &self.values {
            put(&mut map, "values", values.clone());
        }
        put_opt(&mut map, "resolution", &self.resolution);
        put_opt(&mut map, "note", &self.note);
        Value::Mapping(map)
    }
}

impl Document {
    /// Encode the document back to YAML. Inverse of [`Document::from_value`] over every shipped
    /// schema file, which `tests/validate.rs` asserts directly.
    pub fn to_value(&self) -> Value {
        match self {
            Document::Opaque(value) => value.clone(),
            Document::Labels(registry) => {
                let mut map = Mapping::new();
                put_opt(&mut map, "kind", &registry.declared_kind);
                put(
                    &mut map,
                    "labels",
                    ordered_value(&registry.labels, |m| Value::String(m.clone())),
                );
                if !registry.retired.is_empty() {
                    let entries = registry
                        .retired
                        .iter()
                        .map(|r| {
                            let mut entry = Mapping::new();
                            put(&mut entry, "label", Value::String(r.label.clone()));
                            put(&mut entry, "note", Value::String(r.note.clone()));
                            Value::Mapping(entry)
                        })
                        .collect();
                    put(&mut map, "retired", Value::Sequence(entries));
                }
                Value::Mapping(map)
            }
            Document::Rules(schema) => {
                let mut map = Mapping::new();
                put_opt(&mut map, "kind", &schema.declared_kind);
                if let Some(name) = &schema.declared_name {
                    let key = match schema.declared_kind.as_deref() {
                        Some("skill") => "skill",
                        _ => "command",
                    };
                    put(&mut map, key, Value::String(name.clone()));
                }
                if !schema.vars.is_empty() {
                    put(&mut map, "vars", ordered_value(&schema.vars, Clone::clone));
                }
                if !schema.conditions.is_empty() {
                    put(
                        &mut map,
                        "conditions",
                        ordered_value(&schema.conditions, Condition::to_value),
                    );
                }
                if !schema.moments.is_empty() {
                    put(
                        &mut map,
                        "moments",
                        ordered_value(&schema.moments, |m| Value::String(m.clone())),
                    );
                }
                if !schema.sections.is_empty() {
                    put(
                        &mut map,
                        "sections",
                        Value::Sequence(schema.sections.iter().map(Section::to_value).collect()),
                    );
                }
                if !schema.blocks.is_empty() {
                    put(
                        &mut map,
                        "rules",
                        Value::Sequence(schema.blocks.iter().map(Rule::to_value).collect()),
                    );
                }
                if !schema.tombstones.is_empty() {
                    let entries = schema
                        .tombstones
                        .iter()
                        .map(|t| {
                            let mut entry = Mapping::new();
                            put(&mut entry, "id", Value::String(t.id.clone()));
                            put(
                                &mut entry,
                                "disposition",
                                Value::String(t.disposition.clone()),
                            );
                            Value::Mapping(entry)
                        })
                        .collect();
                    put(&mut map, "tombstones", Value::Sequence(entries));
                }
                Value::Mapping(map)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// canonical encoding
// ---------------------------------------------------------------------------

/// The canonical hash of a YAML value, as `sha256:<64 lowercase hex>`.
///
/// Two properties carry the wave's integrity claims and are asserted in `tests/migration.rs`:
///
/// * **Order independence for mappings.** Entries are sorted by their own encoded key bytes, so
///   rewriting a migration's keys in a different order never moves the hash. A YAML re-dump has
///   no such property, which is why this exists rather than hashing serialised text.
/// * **Injectivity.** Every scalar is length-prefixed and every container count-prefixed, so no
///   two distinct values share an encoding — `{ab: c}` and `{a: bc}` are the standard trap.
///
/// Sequence order *is* covered: a section's rule list is ordered data.
///
/// Encoding is depth-bounded at [`MAX_CANONICAL_DEPTH`]. Rule-bearing documents are shape-bounded
/// by the decoder, but templates and shelf data are carried as arbitrary YAML, so an adversarially
/// nested one would otherwise recurse off the stack and abort the process. Past the bound the
/// encoder emits a marker instead of descending; [`canonical_depth`] lets the validator report
/// such a document as a finding rather than hashing it silently.
pub fn canonical_hash(value: &Value) -> String {
    let mut buffer = Vec::new();
    encode_canonical(value, &mut buffer, 0);
    let digest = Sha256::digest(&buffer);
    let mut out = String::with_capacity(7 + 64);
    out.push_str("sha256:");
    for byte in digest {
        // Infallible: writing to a String never fails.
        let _ = write!(out, "{byte:02x}");
    }
    out
}

/// The canonical encoding of a value, for callers that need the bytes rather than the digest.
pub fn canonical_bytes(value: &Value) -> Vec<u8> {
    let mut buffer = Vec::new();
    encode_canonical(value, &mut buffer, 0);
    buffer
}

/// The deepest nesting the canonical encoder will descend before emitting a marker.
///
/// Generous by design: the deepest shipped document nests four levels, so anything near this
/// bound is machine-generated or hostile rather than authored.
pub const MAX_CANONICAL_DEPTH: usize = 64;

/// How deeply a value nests, saturating one past [`MAX_CANONICAL_DEPTH`] so a hostile document
/// cannot make the measurement itself expensive.
pub fn canonical_depth(value: &Value) -> usize {
    fn walk(value: &Value, depth: usize) -> usize {
        if depth > MAX_CANONICAL_DEPTH {
            return depth;
        }
        match value {
            Value::Sequence(items) => items
                .iter()
                .map(|item| walk(item, depth + 1))
                .max()
                .unwrap_or(depth),
            Value::Mapping(map) => map
                .iter()
                .map(|(_, v)| walk(v, depth + 1))
                .max()
                .unwrap_or(depth),
            Value::Tagged(tagged) => walk(&tagged.value, depth + 1),
            _ => depth,
        }
    }
    // Containers are what nest; a scalar sits at the depth of the container holding it, so
    // `a: {b: {c: 1}}` measures 3 rather than 4.
    walk(value, 0)
}

fn push_bytes(tag: u8, bytes: &[u8], out: &mut Vec<u8>) {
    out.push(tag);
    out.extend_from_slice(bytes.len().to_string().as_bytes());
    out.push(b':');
    out.extend_from_slice(bytes);
}

fn encode_canonical(value: &Value, out: &mut Vec<u8>, depth: usize) {
    if depth > MAX_CANONICAL_DEPTH {
        // Deliberately not a recursion: the marker is distinct from every real encoding, so two
        // over-deep documents that differ only below the bound hash alike and neither aborts.
        out.extend_from_slice(b"!depth");
        return;
    }
    match value {
        Value::Null => out.push(b'~'),
        Value::Bool(true) => out.extend_from_slice(b"b1"),
        Value::Bool(false) => out.extend_from_slice(b"b0"),
        Value::Number(number) => {
            // The numeric class is part of the encoding so an integer and a float that print
            // alike cannot collide.
            let (tag, text) = if number.is_i64() {
                (b'i', number.as_i64().unwrap_or_default().to_string())
            } else if number.is_u64() {
                (b'u', number.as_u64().unwrap_or_default().to_string())
            } else {
                (
                    b'f',
                    format!("{:016x}", number.as_f64().unwrap_or(f64::NAN).to_bits()),
                )
            };
            out.push(b'n');
            push_bytes(tag, text.as_bytes(), out);
        }
        Value::String(text) => push_bytes(b's', text.as_bytes(), out),
        Value::Sequence(items) => {
            out.push(b'l');
            out.extend_from_slice(items.len().to_string().as_bytes());
            out.push(b':');
            for item in items {
                encode_canonical(item, out, depth + 1);
            }
        }
        Value::Mapping(map) => {
            let mut entries: Vec<Vec<u8>> = map
                .iter()
                .map(|(key, value)| {
                    let mut pair = Vec::new();
                    encode_canonical(key, &mut pair, depth + 1);
                    encode_canonical(value, &mut pair, depth + 1);
                    pair
                })
                .collect();
            entries.sort();
            out.push(b'm');
            out.extend_from_slice(entries.len().to_string().as_bytes());
            out.push(b':');
            for entry in entries {
                out.extend_from_slice(&entry);
            }
        }
        Value::Tagged(tagged) => {
            let TaggedValue { tag, value } = tagged.as_ref();
            push_bytes(b't', tag.to_string().as_bytes(), out);
            encode_canonical(value, out, depth + 1);
        }
    }
}

// ---------------------------------------------------------------------------
// shared grammar helpers
// ---------------------------------------------------------------------------

/// A lowercase kebab slug segment: `[a-z0-9]+(-[a-z0-9]+)*`.
pub fn is_slug(text: &str) -> bool {
    !text.is_empty()
        && text.split('-').all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        })
}

/// A dotted id: two or more slug segments, the first starting with a letter. This is the shape
/// both checkers accept for rule, section and block ids alike; the stricter per-family prefix
/// rule is the validator's, not the grammar's.
pub fn is_dotted_id(text: &str) -> bool {
    let mut parts = text.split('.');
    let Some(first) = parts.next() else {
        return false;
    };
    if !is_slug(first) || !first.starts_with(|c: char| c.is_ascii_lowercase()) {
        return false;
    }
    let rest: Vec<&str> = parts.collect();
    !rest.is_empty() && rest.iter().all(|part| is_slug(part))
}

/// One `when:` or `values:` token, canonicalised.
///
/// YAML 1.1 read `yes`/`no`/`on`/`off` as booleans, so a schema written under the Python checker
/// may spell one value several ways; every spelling must land on one token or a `when:` term
/// would not match the `values:` list it names. Presence dimensions additionally fold
/// `true`/`false` onto their two poles.
pub fn norm_value(value: &Value, presence: bool) -> String {
    let token = match value {
        Value::Bool(true) => "true".to_string(),
        Value::Bool(false) => "false".to_string(),
        Value::Null => "null".to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.trim().to_ascii_lowercase(),
        _ => String::new(),
    };
    let token = match token.as_str() {
        "yes" | "on" => "true".to_string(),
        "no" | "off" => "false".to_string(),
        _ => token,
    };
    if presence {
        match token.as_str() {
            "true" => "present".to_string(),
            "false" => "absent".to_string(),
            _ => token,
        }
    } else {
        token
    }
}

/// Whether a ruling anchor is well formed.
///
/// The grammar is `YYYY-MM-DD <session-slug>`, optionally followed by one decision segment, and
/// nothing else. Both spellings of that segment are accepted because both are in use: the 597
/// provenance anchors write it bare (`D4`), which is what the shipped checker's expression
/// matches, while the wave plan writes it bracketed (`[D2]`).
///
/// Anchored at both ends deliberately. A trailing-junk-tolerant grammar would accept a whole
/// sentence as an anchor, and the anchor is the evidence that protected content left by ruling.
pub fn is_anchor(text: &str) -> bool {
    let mut parts = text.split(' ').filter(|p| !p.is_empty());
    let (Some(date), Some(slug)) = (parts.next(), parts.next()) else {
        return false;
    };
    if !is_date(date) || slug.is_empty() {
        return false;
    }
    match parts.next() {
        None => true,
        Some(segment) => parts.next().is_none() && is_decision_segment(segment),
    }
}

/// `YYYY-MM-DD`, with the month and day range-checked.
fn is_date(text: &str) -> bool {
    let parts: Vec<&str> = text.split('-').collect();
    if parts.len() != 3 || parts[0].len() != 4 || parts[1].len() != 2 || parts[2].len() != 2 {
        return false;
    }
    if !parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit())) {
        return false;
    }
    let month: u32 = parts[1].parse().unwrap_or(0);
    let day: u32 = parts[2].parse().unwrap_or(0);
    (1..=12).contains(&month) && (1..=31).contains(&day)
}

/// `D<n>` or `[D<n>]` — the human-readable decision pointer, never resolved here.
///
/// The number may carry a trailing run of lowercase letters (`D2a`), the spelling a session
/// uses when it amends a ruling in place. Two live provenance anchors are written that way, so
/// the grammar accepts it rather than asking the corpus to normalise itself. The suffix is
/// letters only and must follow at least one digit: `D`, `Da`, and `D2 D3` all stay malformed.
fn is_decision_segment(text: &str) -> bool {
    let inner = match text.strip_prefix('[') {
        Some(rest) => match rest.strip_suffix(']') {
            Some(inner) => inner,
            None => return false,
        },
        None => text,
    };
    match inner.strip_prefix('D') {
        Some(rest) => {
            let digits = rest.trim_end_matches(|c: char| c.is_ascii_lowercase());
            !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit())
        }
        None => false,
    }
}
