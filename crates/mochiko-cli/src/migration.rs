//! The migration-file grammar: header, change ops, body hash, and the version contract.
//!
//! Truth lives in an ordered log of migration files committed to git; this module turns one such
//! file into a typed [`Migration`]. It decides nothing about whether the change is *legal* — that
//! is the replay's job, and then the validator's. It only decides whether the file is a
//! well-formed migration at all.

use crate::model::{canonical_hash, DocKind, DocRef};
use serde_norway::{Mapping, Value};
use std::fmt;

/// The grammar versions this binary can read, inclusive.
///
/// A log outside the range halts loudly rather than being read best-effort: partial delivery is
/// exactly the failure mode the no-fallback posture exists to rule out.
pub const GRAMMAR_RANGE: (u32, u32) = (1, 1);

/// The command a version-contract halt tells the user to run. Named here, printed by the CLI
/// surface, so the message has one home.
pub const INSTALL_COMMAND: &str = "cargo install mochiko-cli";

/// Why a migration file is not a migration.
#[derive(Clone, Debug, PartialEq)]
pub enum ParseError {
    /// The file is not readable as YAML.
    Yaml { file: String, message: String },
    /// A required header field is missing or malformed.
    Header { file: String, message: String },
    /// The filename's numeric prefix disagrees with the header's `sequence:`.
    SequenceMismatch {
        file: String,
        filename: u32,
        header: u32,
    },
    /// The recorded `hash:` does not match the body.
    HashMismatch {
        file: String,
        recorded: String,
        computed: String,
    },
    /// The log's grammar version falls outside [`GRAMMAR_RANGE`].
    GrammarVersion { file: String, found: u32 },
    /// One entry of `changes:` names an op the grammar does not carry.
    UnknownOp {
        file: String,
        index: usize,
        message: String,
    },
    /// One entry of `changes:` names a known op but is not a well-formed instance of it.
    MalformedChange {
        file: String,
        index: usize,
        message: String,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Yaml { file, message } => {
                write!(f, "{file}: not readable as YAML: {message}")
            }
            ParseError::Header { file, message } => write!(f, "{file}: {message}"),
            ParseError::SequenceMismatch {
                file,
                filename,
                header,
            } => write!(
                f,
                "{file}: the filename numbers this migration {filename} but its header says \
                 sequence {header} — the log is ordered by the header and named by it"
            ),
            ParseError::HashMismatch {
                file,
                recorded,
                computed,
            } => write!(
                f,
                "{file}: body hash mismatch — the header records {recorded} but the body \
                 canonicalises to {computed}"
            ),
            ParseError::GrammarVersion { file, found } => write!(
                f,
                "{file}: the migration log is written in grammar {found}, and this binary reads \
                 grammar {}..{}. Update the binary: {INSTALL_COMMAND}",
                GRAMMAR_RANGE.0, GRAMMAR_RANGE.1
            ),
            ParseError::UnknownOp {
                file,
                index,
                message,
            }
            | ParseError::MalformedChange {
                file,
                index,
                message,
            } => write!(f, "{file}: changes[{index}]: {message}"),
        }
    }
}

impl std::error::Error for ParseError {}

/// The finding code a parse error reports under.
impl ParseError {
    pub fn code(&self) -> &'static str {
        match self {
            ParseError::Yaml { .. } => "grammar-parse",
            ParseError::Header { .. } => "grammar-header",
            ParseError::SequenceMismatch { .. } => "sequence-mismatch",
            ParseError::HashMismatch { .. } => "hash-mismatch",
            ParseError::GrammarVersion { .. } => "grammar-version",
            ParseError::UnknownOp { .. } => "op-unknown",
            // A known op missing a field is a different problem from an op nobody has heard of,
            // and sends a maintainer to a different place.
            ParseError::MalformedChange { .. } => "op-malformed",
        }
    }

    /// The file this error came from.
    pub fn file(&self) -> &str {
        match self {
            ParseError::Yaml { file, .. }
            | ParseError::Header { file, .. }
            | ParseError::SequenceMismatch { file, .. }
            | ParseError::HashMismatch { file, .. }
            | ParseError::GrammarVersion { file, .. }
            | ParseError::UnknownOp { file, .. }
            | ParseError::MalformedChange { file, .. } => file,
        }
    }
}

/// The op discriminator of a [`Change`], for reporting and for exhaustive test coverage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChangeOp {
    ImportDocument,
    ReplaceDocument,
    MintSection,
    RewordSection,
    TombstoneSection,
    MintRule,
    RewordRule,
    SetRuleField,
    MoveRule,
    TombstoneRule,
    SupersedeRule,
    SetVar,
    SetCondition,
    SetMoment,
    RegistryAdd,
    RegistryRetire,
}

impl ChangeOp {
    /// Every op, so a test can assert the grammar's whole surface is exercised.
    pub const ALL: [ChangeOp; 16] = [
        ChangeOp::ImportDocument,
        ChangeOp::ReplaceDocument,
        ChangeOp::MintSection,
        ChangeOp::RewordSection,
        ChangeOp::TombstoneSection,
        ChangeOp::MintRule,
        ChangeOp::RewordRule,
        ChangeOp::SetRuleField,
        ChangeOp::MoveRule,
        ChangeOp::TombstoneRule,
        ChangeOp::SupersedeRule,
        ChangeOp::SetVar,
        ChangeOp::SetCondition,
        ChangeOp::SetMoment,
        ChangeOp::RegistryAdd,
        ChangeOp::RegistryRetire,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            ChangeOp::ImportDocument => "import-document",
            ChangeOp::ReplaceDocument => "replace-document",
            ChangeOp::MintSection => "mint-section",
            ChangeOp::RewordSection => "reword-section",
            ChangeOp::TombstoneSection => "tombstone-section",
            ChangeOp::MintRule => "mint-rule",
            ChangeOp::RewordRule => "reword-rule",
            ChangeOp::SetRuleField => "set-rule-field",
            ChangeOp::MoveRule => "move-rule",
            ChangeOp::TombstoneRule => "tombstone-rule",
            ChangeOp::SupersedeRule => "supersede-rule",
            ChangeOp::SetVar => "set-var",
            ChangeOp::SetCondition => "set-condition",
            ChangeOp::SetMoment => "set-moment",
            ChangeOp::RegistryAdd => "registry-add",
            ChangeOp::RegistryRetire => "registry-retire",
        }
    }

    fn parse(token: &str) -> Option<ChangeOp> {
        ChangeOp::ALL.into_iter().find(|op| op.as_str() == token)
    }
}

impl fmt::Display for ChangeOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The settable fields of `set-rule-field`. `id` and `text` are absent by design: an id is minted
/// once and never edited, and text has its own op so a reword is legible in a diff as a reword.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleField {
    Labels,
    Class,
    Kind,
    When,
    Pointer,
    Extends,
    Enforces,
    Anchor,
    Note,
}

impl RuleField {
    pub const ALL: [RuleField; 9] = [
        RuleField::Labels,
        RuleField::Class,
        RuleField::Kind,
        RuleField::When,
        RuleField::Pointer,
        RuleField::Extends,
        RuleField::Enforces,
        RuleField::Anchor,
        RuleField::Note,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            RuleField::Labels => "labels",
            RuleField::Class => "class",
            RuleField::Kind => "kind",
            RuleField::When => "when",
            RuleField::Pointer => "pointer",
            RuleField::Extends => "extends",
            RuleField::Enforces => "enforces",
            RuleField::Anchor => "anchor",
            RuleField::Note => "note",
        }
    }

    fn parse(token: &str) -> Option<RuleField> {
        RuleField::ALL.into_iter().find(|f| f.as_str() == token)
    }
}

impl fmt::Display for RuleField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One optional field edit carried by a [`ChangeOp::RewordSection`] change.
///
/// Three states, because a field the change never names and a field it explicitly clears are
/// different instructions. `set-rule-field` already draws that distinction; collapsing it here
/// would make `note: ~` indistinguishable from a change that said nothing about `note:`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SectionEdit {
    /// The change does not name this field, so it is left exactly as it stands.
    Untouched,
    /// `field: ~` — the field is cleared. Legal for `note:` alone.
    Clear,
    Set(String),
}

/// One change. Each is independently citable: a rule's history is the ops naming its id.
#[derive(Clone, Debug, PartialEq)]
pub enum Change {
    ImportDocument {
        doc: DocRef,
        content: Value,
    },
    ReplaceDocument {
        doc: DocRef,
        content: Value,
    },
    MintSection {
        doc: DocRef,
        section: Value,
    },
    /// A section's prose — its `title:`, its `intent:`, its `note:` — with at least one of the
    /// three named. The section id and its rules are untouched.
    RewordSection {
        doc: DocRef,
        id: String,
        title: SectionEdit,
        intent: SectionEdit,
        note: SectionEdit,
    },
    TombstoneSection {
        doc: DocRef,
        id: String,
        disposition: String,
    },
    MintRule {
        doc: DocRef,
        section: String,
        rule: Value,
    },
    RewordRule {
        doc: DocRef,
        id: String,
        text: String,
    },
    SetRuleField {
        doc: DocRef,
        id: String,
        field: RuleField,
        value: Value,
    },
    MoveRule {
        doc: DocRef,
        id: String,
        section: String,
    },
    TombstoneRule {
        doc: DocRef,
        id: String,
        disposition: String,
    },
    SupersedeRule {
        doc: DocRef,
        id: String,
        disposition: String,
        anchor: String,
    },
    SetVar {
        doc: DocRef,
        name: String,
        value: Value,
    },
    SetCondition {
        doc: DocRef,
        name: String,
        spec: Value,
    },
    SetMoment {
        doc: DocRef,
        name: String,
        text: Value,
    },
    RegistryAdd {
        doc: DocRef,
        label: String,
        meaning: String,
    },
    RegistryRetire {
        doc: DocRef,
        label: String,
        note: String,
    },
}

impl Change {
    pub fn op(&self) -> ChangeOp {
        match self {
            Change::ImportDocument { .. } => ChangeOp::ImportDocument,
            Change::ReplaceDocument { .. } => ChangeOp::ReplaceDocument,
            Change::MintSection { .. } => ChangeOp::MintSection,
            Change::RewordSection { .. } => ChangeOp::RewordSection,
            Change::TombstoneSection { .. } => ChangeOp::TombstoneSection,
            Change::MintRule { .. } => ChangeOp::MintRule,
            Change::RewordRule { .. } => ChangeOp::RewordRule,
            Change::SetRuleField { .. } => ChangeOp::SetRuleField,
            Change::MoveRule { .. } => ChangeOp::MoveRule,
            Change::TombstoneRule { .. } => ChangeOp::TombstoneRule,
            Change::SupersedeRule { .. } => ChangeOp::SupersedeRule,
            Change::SetVar { .. } => ChangeOp::SetVar,
            Change::SetCondition { .. } => ChangeOp::SetCondition,
            Change::SetMoment { .. } => ChangeOp::SetMoment,
            Change::RegistryAdd { .. } => ChangeOp::RegistryAdd,
            Change::RegistryRetire { .. } => ChangeOp::RegistryRetire,
        }
    }

    /// The document this change addresses.
    pub fn doc(&self) -> &DocRef {
        match self {
            Change::ImportDocument { doc, .. }
            | Change::ReplaceDocument { doc, .. }
            | Change::MintSection { doc, .. }
            | Change::RewordSection { doc, .. }
            | Change::TombstoneSection { doc, .. }
            | Change::MintRule { doc, .. }
            | Change::RewordRule { doc, .. }
            | Change::SetRuleField { doc, .. }
            | Change::MoveRule { doc, .. }
            | Change::TombstoneRule { doc, .. }
            | Change::SupersedeRule { doc, .. }
            | Change::SetVar { doc, .. }
            | Change::SetCondition { doc, .. }
            | Change::SetMoment { doc, .. }
            | Change::RegistryAdd { doc, .. }
            | Change::RegistryRetire { doc, .. } => doc,
        }
    }

    /// The rule or section id this change addresses, where it names one.
    pub fn target_id(&self) -> Option<&str> {
        match self {
            Change::RewordSection { id, .. }
            | Change::TombstoneSection { id, .. }
            | Change::RewordRule { id, .. }
            | Change::SetRuleField { id, .. }
            | Change::MoveRule { id, .. }
            | Change::TombstoneRule { id, .. }
            | Change::SupersedeRule { id, .. } => Some(id),
            _ => None,
        }
    }
}

/// One migration file: its header and its ordered change set.
#[derive(Clone, Debug, PartialEq)]
pub struct Migration {
    pub file: String,
    pub grammar: u32,
    pub id: String,
    pub sequence: u32,
    pub intent: String,
    /// Required whenever a change supersedes or tombstones protected content — a floor rule, a
    /// fail rule, or a rule already carrying an anchor. The replay enforces that; the grammar
    /// only records it.
    pub anchor: Option<String>,
    pub changes: Vec<Change>,
    /// The canonical value the body hash is taken over, retained so `body_hash` needs no re-parse.
    hashed_body: Value,
}

impl Migration {
    /// The hash over this migration's identity and body: `{id, sequence, anchor, changes}`.
    ///
    /// `intent:` is prose and is deliberately outside the hash so it can be corrected without
    /// invalidating the file. The anchor is inside it, because the anchor is the evidence that a
    /// protected rule left by ruling — leaving it unhashed would make that evidence editable
    /// after the fact.
    pub fn body_hash(&self) -> String {
        canonical_hash(&self.hashed_body)
    }
}

fn get<'a>(map: &'a Mapping, key: &str) -> Option<&'a Value> {
    map.get(Value::String(key.to_string()))
}

fn header_err(file: &str, message: impl Into<String>) -> ParseError {
    ParseError::Header {
        file: file.to_string(),
        message: message.into(),
    }
}

/// A known op that is not a well-formed instance of itself.
fn change_err(file: &str, index: usize, message: impl Into<String>) -> ParseError {
    ParseError::MalformedChange {
        file: file.to_string(),
        index,
        message: message.into(),
    }
}

/// An op the grammar does not carry.
fn unknown_op_err(file: &str, index: usize, message: impl Into<String>) -> ParseError {
    ParseError::UnknownOp {
        file: file.to_string(),
        index,
        message: message.into(),
    }
}

/// The numeric prefix of a `NNNN-<slug>.yaml` filename.
pub fn filename_sequence(file: &str) -> Option<u32> {
    let stem = file.rsplit('/').next().unwrap_or(file);
    let digits: String = stem.chars().take_while(|c| c.is_ascii_digit()).collect();
    (!digits.is_empty()).then(|| digits.parse().ok())?
}

fn required_str(map: &Mapping, key: &str, file: &str) -> Result<String, ParseError> {
    match get(map, key) {
        Some(Value::String(text)) if !text.trim().is_empty() => Ok(text.trim().to_string()),
        Some(Value::Null) | None => Err(header_err(file, format!("header field `{key}:` missing"))),
        Some(_) => Err(header_err(
            file,
            format!("header field `{key}:` must be text"),
        )),
    }
}

fn required_u32(map: &Mapping, key: &str, file: &str) -> Result<u32, ParseError> {
    match get(map, key).and_then(Value::as_u64) {
        Some(n) => u32::try_from(n)
            .map_err(|_| header_err(file, format!("header field `{key}:` is out of range"))),
        None => Err(header_err(
            file,
            format!("header field `{key}:` missing or not a whole number"),
        )),
    }
}

/// The log's grammar version.
///
/// Read before any range check of its own, so that a version this binary cannot handle always
/// reaches the version-contract halt with its install line. A cap applied first would report a
/// far-future grammar as a malformed header and swallow the one message the user needs.
fn read_grammar(map: &Mapping, file: &str) -> Result<u32, ParseError> {
    let Some(raw) = get(map, "grammar") else {
        return Err(header_err(file, "header field `grammar:` missing"));
    };
    let Some(found) = raw.as_u64() else {
        return Err(header_err(
            file,
            "header field `grammar:` missing or not a whole number",
        ));
    };
    let found = u32::try_from(found).unwrap_or(u32::MAX);
    if found < GRAMMAR_RANGE.0 || found > GRAMMAR_RANGE.1 {
        return Err(ParseError::GrammarVersion {
            file: file.to_string(),
            found,
        });
    }
    Ok(found)
}

fn doc_ref(map: &Mapping, key: &str, file: &str, index: usize) -> Result<DocRef, ParseError> {
    let raw = match get(map, key) {
        Some(Value::String(text)) => text.trim().to_string(),
        _ => {
            return Err(change_err(
                file,
                index,
                format!("`{key}:` must name a document as `<kind>/<name>`"),
            ))
        }
    };
    DocRef::parse(&raw).ok_or_else(|| {
        let kinds: Vec<&str> = DocKind::ALL.iter().map(|k| k.as_str()).collect();
        change_err(
            file,
            index,
            format!(
                "`{key}: {raw}` names no document kind — want one of {}",
                kinds.join(" · ")
            ),
        )
    })
}

fn field_str(map: &Mapping, key: &str, file: &str, index: usize) -> Result<String, ParseError> {
    match get(map, key) {
        Some(Value::String(text)) => Ok(text.clone()),
        Some(Value::Bool(b)) => Ok(b.to_string()),
        Some(Value::Number(n)) => Ok(n.to_string()),
        _ => Err(change_err(
            file,
            index,
            format!("`{key}:` missing or not text"),
        )),
    }
}

fn field_value(map: &Mapping, key: &str, file: &str, index: usize) -> Result<Value, ParseError> {
    get(map, key)
        .cloned()
        .ok_or_else(|| change_err(file, index, format!("`{key}:` missing")))
}

/// One optional prose field of a `reword-section` change.
///
/// `clearable` says whether `~` is a legal instruction for this field. A section always carries a
/// `title:` and an `intent:` — the hard set reports a section missing either — so only `note:` may
/// be cleared. An empty or blank value is rejected for all three: it would write the very state
/// the validator rejects, and for `note:` the clear is spelled `~`, not `''`.
fn section_edit(
    map: &Mapping,
    key: &str,
    clearable: bool,
    file: &str,
    index: usize,
) -> Result<SectionEdit, ParseError> {
    match get(map, key) {
        None => Ok(SectionEdit::Untouched),
        Some(Value::Null) if clearable => Ok(SectionEdit::Clear),
        Some(Value::Null) => Err(change_err(
            file,
            index,
            format!(
                "`{key}: ~` — every section carries a `{key}:`, so it is reworded and never cleared"
            ),
        )),
        Some(Value::String(text)) if text.trim().is_empty() => Err(change_err(
            file,
            index,
            format!("`{key}:` is blank — a reworded field carries text"),
        )),
        Some(Value::String(text)) => Ok(SectionEdit::Set(text.clone())),
        Some(Value::Bool(b)) => Ok(SectionEdit::Set(b.to_string())),
        Some(Value::Number(n)) => Ok(SectionEdit::Set(n.to_string())),
        Some(_) => Err(change_err(
            file,
            index,
            format!("`{key}:` must be one scalar of text"),
        )),
    }
}

/// The document a `kind:` + `name:` pair on a document-level op addresses.
fn kind_name_ref(map: &Mapping, file: &str, index: usize) -> Result<DocRef, ParseError> {
    let kind_token = field_str(map, "kind", file, index)?;
    let kind = DocKind::parse(kind_token.trim()).ok_or_else(|| {
        change_err(
            file,
            index,
            format!("`kind: {kind_token}` is not a document kind"),
        )
    })?;
    Ok(DocRef::new(kind, field_str(map, "name", file, index)?))
}

fn parse_change(value: &Value, file: &str, index: usize) -> Result<Change, ParseError> {
    let map = value
        .as_mapping()
        .ok_or_else(|| change_err(file, index, "a change must be a mapping"))?;
    let op_token =
        field_str(map, "op", file, index).map_err(|_| change_err(file, index, "`op:` missing"))?;
    let op = ChangeOp::parse(op_token.trim()).ok_or_else(|| {
        let known: Vec<&str> = ChangeOp::ALL.iter().map(|o| o.as_str()).collect();
        unknown_op_err(
            file,
            index,
            format!(
                "`op: {}` is not a change op — want one of {}",
                op_token.trim(),
                known.join(" · ")
            ),
        )
    })?;

    Ok(match op {
        ChangeOp::ImportDocument => Change::ImportDocument {
            doc: kind_name_ref(map, file, index)?,
            content: field_value(map, "content", file, index)?,
        },
        ChangeOp::ReplaceDocument => Change::ReplaceDocument {
            doc: kind_name_ref(map, file, index)?,
            content: field_value(map, "content", file, index)?,
        },
        ChangeOp::MintSection => Change::MintSection {
            doc: doc_ref(map, "schema", file, index)?,
            section: field_value(map, "section", file, index)?,
        },
        ChangeOp::RewordSection => {
            let doc = doc_ref(map, "schema", file, index)?;
            let id = field_str(map, "id", file, index)?;
            let title = section_edit(map, "title", false, file, index)?;
            let intent = section_edit(map, "intent", false, file, index)?;
            let note = section_edit(map, "note", true, file, index)?;
            if title == SectionEdit::Untouched
                && intent == SectionEdit::Untouched
                && note == SectionEdit::Untouched
            {
                return Err(change_err(
                    file,
                    index,
                    "names no field to reword — want at least one of `title:` · `intent:` · \
                     `note:`",
                ));
            }
            Change::RewordSection {
                doc,
                id,
                title,
                intent,
                note,
            }
        }
        ChangeOp::TombstoneSection => Change::TombstoneSection {
            doc: doc_ref(map, "schema", file, index)?,
            id: field_str(map, "id", file, index)?,
            disposition: field_str(map, "disposition", file, index)?,
        },
        ChangeOp::MintRule => Change::MintRule {
            doc: doc_ref(map, "schema", file, index)?,
            section: field_str(map, "section", file, index)?,
            rule: field_value(map, "rule", file, index)?,
        },
        ChangeOp::RewordRule => Change::RewordRule {
            doc: doc_ref(map, "schema", file, index)?,
            id: field_str(map, "id", file, index)?,
            text: field_str(map, "text", file, index)?,
        },
        ChangeOp::SetRuleField => {
            let field_token = field_str(map, "field", file, index)?;
            let field = RuleField::parse(field_token.trim()).ok_or_else(|| {
                let known: Vec<&str> = RuleField::ALL.iter().map(|f| f.as_str()).collect();
                change_err(
                    file,
                    index,
                    format!(
                        "`field: {}` is not a settable rule field — want one of {} \
                         (an id is minted once, and text has its own op)",
                        field_token.trim(),
                        known.join(" · ")
                    ),
                )
            })?;
            Change::SetRuleField {
                doc: doc_ref(map, "schema", file, index)?,
                id: field_str(map, "id", file, index)?,
                field,
                // An absent `value:` and an explicit `value: ~` both clear the field.
                value: get(map, "value").cloned().unwrap_or(Value::Null),
            }
        }
        ChangeOp::MoveRule => Change::MoveRule {
            doc: doc_ref(map, "schema", file, index)?,
            id: field_str(map, "id", file, index)?,
            section: field_str(map, "section", file, index)?,
        },
        ChangeOp::TombstoneRule => Change::TombstoneRule {
            doc: doc_ref(map, "schema", file, index)?,
            id: field_str(map, "id", file, index)?,
            disposition: field_str(map, "disposition", file, index)?,
        },
        ChangeOp::SupersedeRule => Change::SupersedeRule {
            doc: doc_ref(map, "schema", file, index)?,
            id: field_str(map, "id", file, index)?,
            disposition: field_str(map, "disposition", file, index)?,
            anchor: field_str(map, "anchor", file, index)?,
        },
        ChangeOp::SetVar => Change::SetVar {
            doc: doc_ref(map, "schema", file, index)?,
            name: field_str(map, "name", file, index)?,
            value: get(map, "value").cloned().unwrap_or(Value::Null),
        },
        ChangeOp::SetCondition => Change::SetCondition {
            doc: doc_ref(map, "schema", file, index)?,
            name: field_str(map, "name", file, index)?,
            spec: get(map, "spec").cloned().unwrap_or(Value::Null),
        },
        ChangeOp::SetMoment => Change::SetMoment {
            doc: doc_ref(map, "schema", file, index)?,
            name: field_str(map, "name", file, index)?,
            text: get(map, "text").cloned().unwrap_or(Value::Null),
        },
        ChangeOp::RegistryAdd => Change::RegistryAdd {
            doc: doc_ref(map, "registry", file, index)?,
            label: field_str(map, "label", file, index)?,
            meaning: field_str(map, "meaning", file, index)?,
        },
        ChangeOp::RegistryRetire => Change::RegistryRetire {
            doc: doc_ref(map, "registry", file, index)?,
            label: field_str(map, "label", file, index)?,
            note: field_str(map, "note", file, index)?,
        },
    })
}

/// Parse one migration file. `file` is the file's name, which carries the sequence prefix.
pub fn parse(file: &str, source: &str) -> Result<Migration, ParseError> {
    parse_inner(file, source, true)
}

/// The canonical body hash of a parsed migration — the value its `hash:` header must carry.
pub fn compute_hash(migration: &Migration) -> String {
    migration.body_hash()
}

/// Stamp a migration body with its correct `hash:` header, replacing any hash already there.
///
/// The authoring helper, and the one fixtures use: every other path requires the hash, so without
/// this a test or a generator would have to reimplement the canonical encoding to write a valid
/// file. The body must otherwise be a well-formed migration; it is parsed before it is stamped.
pub fn with_hash(file: &str, source: &str) -> Result<String, ParseError> {
    let migration = parse_inner(file, source, false)?;
    let hash = migration.body_hash();

    let root: Value = serde_norway::from_str(source).map_err(|e| ParseError::Yaml {
        file: file.to_string(),
        message: e.to_string(),
    })?;
    let existing = root
        .as_mapping()
        .ok_or_else(|| header_err(file, "a migration file is a mapping of header fields"))?;

    // Rebuilt in the source's own key order, with `hash:` written in place if it was already
    // there and appended just before `changes:` if it was not.
    let mut out = Mapping::new();
    let mut written = false;
    for (key, value) in existing {
        let name = key.as_str().unwrap_or_default();
        if name == "hash" {
            out.insert(key.clone(), Value::String(hash.clone()));
            written = true;
            continue;
        }
        if name == "changes" && !written {
            out.insert(Value::String("hash".into()), Value::String(hash.clone()));
            written = true;
        }
        out.insert(key.clone(), value.clone());
    }
    if !written {
        out.insert(Value::String("hash".into()), Value::String(hash));
    }
    serde_norway::to_string(&Value::Mapping(out)).map_err(|e| ParseError::Yaml {
        file: file.to_string(),
        message: e.to_string(),
    })
}

fn parse_inner(file: &str, source: &str, require_hash: bool) -> Result<Migration, ParseError> {
    let root: Value = serde_norway::from_str(source).map_err(|e| ParseError::Yaml {
        file: file.to_string(),
        message: e.to_string(),
    })?;
    let map = root
        .as_mapping()
        .ok_or_else(|| header_err(file, "a migration file is a mapping of header fields"))?;

    let grammar = read_grammar(map, file)?;

    let id = required_str(map, "id", file)?;
    let sequence = required_u32(map, "sequence", file)?;
    let intent = required_str(map, "intent", file)?;
    let anchor = match get(map, "anchor") {
        None | Some(Value::Null) => None,
        Some(Value::String(text)) => Some(text.trim().to_string()),
        Some(_) => return Err(header_err(file, "header field `anchor:` must be text")),
    };

    if let Some(from_name) = filename_sequence(file) {
        if from_name != sequence {
            return Err(ParseError::SequenceMismatch {
                file: file.to_string(),
                filename: from_name,
                header: sequence,
            });
        }
    }

    let raw_changes = match get(map, "changes") {
        Some(Value::Sequence(items)) => items.clone(),
        Some(Value::Null) | None => {
            return Err(header_err(file, "header field `changes:` missing"))
        }
        Some(_) => return Err(header_err(file, "header field `changes:` must be a list")),
    };

    let mut changes = Vec::with_capacity(raw_changes.len());
    for (index, raw) in raw_changes.iter().enumerate() {
        changes.push(parse_change(raw, file, index)?);
    }

    // The hashed body: identity plus content, excluding the prose intent and the hash itself.
    let mut body = Mapping::new();
    body.insert(Value::String("id".into()), Value::String(id.clone()));
    body.insert(
        Value::String("sequence".into()),
        Value::Number(sequence.into()),
    );
    body.insert(
        Value::String("anchor".into()),
        match &anchor {
            Some(text) => Value::String(text.clone()),
            None => Value::Null,
        },
    );
    body.insert(
        Value::String("changes".into()),
        Value::Sequence(raw_changes),
    );
    let hashed_body = Value::Mapping(body);

    let migration = Migration {
        file: file.to_string(),
        grammar,
        id,
        sequence,
        intent,
        anchor,
        changes,
        hashed_body,
    };

    // The hash is required. An optional one would be no protection at all: the anchor it covers
    // is the evidence that protected content left by ruling, and an editor who need not forge a
    // hash need only delete a line. `with_hash` is the sanctioned way to produce one.
    if !require_hash {
        // The stamping path. Any hash already in the file is about to be overwritten, so
        // validating it here would make `with_hash` unable to correct a stale one — which is
        // most of what it is for.
        return Ok(migration);
    }
    let recorded = match get(map, "hash") {
        Some(Value::String(text)) => text.trim().to_string(),
        Some(Value::Null) | None => String::new(),
        Some(_) => return Err(header_err(file, "header field `hash:` must be text")),
    };
    if recorded.is_empty() {
        return Err(header_err(
            file,
            "header field `hash:` missing — every migration carries the canonical hash of \
             its own body (write one with `with_hash`)",
        ));
    }
    let computed = migration.body_hash();
    if recorded != computed {
        return Err(ParseError::HashMismatch {
            file: file.to_string(),
            recorded,
            computed,
        });
    }

    Ok(migration)
}
