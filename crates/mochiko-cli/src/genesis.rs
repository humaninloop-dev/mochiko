//! The genesis migration — the shipped corpus imported as the log's baseline.
//!
//! `plugins/mochiko/migrations/0001-genesis.yaml` is **generated**, and the committed file is the
//! artifact (wave plan §5). This module is the generator: it reads the 50 shipped schema files,
//! folds the provenance sidecar's anchors onto their rules, lifts the two comment-carried
//! `enforces: []` reasons into `note:` data, and writes one `import-document` op per document.
//!
//! Three properties make the committed file trustworthy rather than merely present:
//!
//! * **Deterministic.** Documents are emitted in address order and every value is written by the
//!   same writer, so regenerating on another machine produces the same bytes. `tests/fidelity.rs`
//!   regenerates and compares byte-for-byte.
//! * **Lossless.** The content of each op is `Document::to_value()` over the decoded file, and the
//!   round trip is asserted over all 50 (P1's A1).
//! * **Loud.** Nothing is dropped quietly. A sidecar key naming no rule, an id two documents
//!   claim, an empty `enforces:` with no stated reason — each stops the build and says so.
//!
//! The sidecar file itself is never written (record D2): the anchors are carried, not moved.
//!
//! Failures here are [`GenesisError`], not `validate::Finding`. A finding is something true of the
//! store's state, drawn from a closed code vocabulary the validator owns and its coverage guard
//! pins; a generator that cannot read its inputs has not produced a state to have findings about.

use crate::migration;
use crate::model::{DocKind, DocRef, Document, Rule};
use serde_norway::{Mapping, Value};
use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

/// The genesis migration's file name, id, sequence and intent — lead-assigned at wave 1.
pub const FILE: &str = "0001-genesis.yaml";
pub const ID: &str = "0001-genesis";
pub const SEQUENCE: u32 = 1;
pub const INTENT: &str =
    "Import the v0.103.0 schema corpus as the log's baseline, carrying the provenance sidecar's \
     anchors on their rules.";

/// The wave's own ruling, carried as the header anchor. Genesis supersedes nothing, so the
/// grammar does not require one; it is here as the provenance of the import itself.
pub const ANCHOR: &str = "2026-09-03 cli-schema-delivery D2";

/// The maintainer-side provenance sidecar (`command-content-schema` D16). Read, never written.
pub const SIDECAR: &str = ".mochiko/provenance.yaml";

/// The two discriminators the sidecar has carried across its rename at v0.100.0.
const SIDECAR_KINDS: [&str; 2] = ["primitive-provenance", "command-provenance"];

/// The comment marker whose text becomes a rule's `note:`.
///
/// D6 allows an empty `enforces:` only alongside a stated reason. In the YAML corpus that reason
/// is a comment, which no typed model can carry, so the grammar holds it as data and genesis is
/// where the carry happens. Matched generically rather than by rule id: a reworded comment still
/// carries, and a third one is lifted rather than missed.
pub const EMPTY_ENFORCES_MARKER: &str = "D6 empty-with-reason:";

// ---------------------------------------------------------------------------
// errors
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum GenesisError {
    Read {
        path: PathBuf,
        message: String,
    },
    Parse {
        path: PathBuf,
        message: String,
    },
    Decode {
        path: PathBuf,
        message: String,
    },
    /// A schema file whose kind cannot be derived from its own content.
    UnknownKind {
        path: PathBuf,
    },
    /// Two files claiming one address.
    DuplicateDocument {
        doc: DocRef,
        path: PathBuf,
    },
    /// The sidecar's `kind:` is neither spelling.
    SidecarKind {
        found: String,
    },
    /// The sidecar carries no `anchors:` mapping.
    SidecarShape {
        message: String,
    },
    /// An anchor keyed to an id no document carries live.
    DanglingAnchor {
        id: String,
    },
    /// An id two documents both carry, so an anchor cannot be attributed.
    AmbiguousRule {
        id: String,
        docs: Vec<String>,
    },
    /// An empty `enforces:` with no `# D6 empty-with-reason:` comment above it.
    UnexplainedEmptyEnforces {
        doc: DocRef,
        id: String,
    },
    /// The stamping path refused the generated body.
    Stamp {
        message: String,
    },
}

impl fmt::Display for GenesisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenesisError::Read { path, message } => {
                write!(f, "{}: cannot read: {message}", path.display())
            }
            GenesisError::Parse { path, message } => {
                write!(f, "{}: does not parse as YAML: {message}", path.display())
            }
            GenesisError::Decode { path, message } => {
                write!(f, "{}: does not decode: {message}", path.display())
            }
            GenesisError::UnknownKind { path } => write!(
                f,
                "{}: carries no `kind:`, `template:` or `shelf:` to derive its kind from",
                path.display()
            ),
            GenesisError::DuplicateDocument { doc, path } => write!(
                f,
                "{}: a second file claims the address {doc}",
                path.display()
            ),
            GenesisError::SidecarKind { found } => write!(
                f,
                "{SIDECAR}: `kind: {found}` is not one of {}",
                SIDECAR_KINDS.join(" · ")
            ),
            GenesisError::SidecarShape { message } => write!(f, "{SIDECAR}: {message}"),
            GenesisError::DanglingAnchor { id } => write!(
                f,
                "{SIDECAR}: anchor '{id}' names no live rule in any document"
            ),
            GenesisError::AmbiguousRule { id, docs } => write!(
                f,
                "rule id '{id}' is carried by more than one document ({}), so its anchor cannot \
                 be attributed",
                docs.join(", ")
            ),
            GenesisError::UnexplainedEmptyEnforces { doc, id } => write!(
                f,
                "{doc} · {id}: `enforces: []` with no `# {EMPTY_ENFORCES_MARKER}` comment above \
                 it — D6 allows an empty mirror only with a stated reason"
            ),
            GenesisError::Stamp { message } => {
                write!(f, "the generated migration would not stamp: {message}")
            }
        }
    }
}

impl std::error::Error for GenesisError {}

/// Render an error list the way the CLI prints findings: one per line.
pub fn render_errors(errors: &[GenesisError]) -> String {
    errors
        .iter()
        .map(|e| format!("{e}\n"))
        .collect::<Vec<_>>()
        .concat()
}

// ---------------------------------------------------------------------------
// scanning
// ---------------------------------------------------------------------------

/// One shipped schema file, as read off disk.
#[derive(Clone, Debug)]
pub struct ShippedFile {
    pub doc: DocRef,
    pub path: PathBuf,
    pub text: String,
    pub value: Value,
}

/// Every shipped schema file, in address order.
///
/// The address derivation is the corpus's own: a document's `kind:` names its kind, a skill's
/// name is its directory, and a file with no `kind:` is a template or the shelf data by the key
/// it does carry.
pub fn scan(root: &Path) -> Result<Vec<ShippedFile>, Vec<GenesisError>> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let schemas = root.join("plugins/mochiko/schemas");
    match std::fs::read_dir(&schemas) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("yaml") {
                    paths.push(path);
                }
            }
        }
        Err(e) => {
            return Err(vec![GenesisError::Read {
                path: schemas,
                message: e.to_string(),
            }])
        }
    }
    let skills = root.join("plugins/mochiko/skills");
    match std::fs::read_dir(&skills) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path().join("schema.yaml");
                if path.is_file() {
                    paths.push(path);
                }
            }
        }
        Err(e) => {
            return Err(vec![GenesisError::Read {
                path: skills,
                message: e.to_string(),
            }])
        }
    }
    paths.sort();

    let mut errors = Vec::new();
    let mut seen: BTreeMap<DocRef, PathBuf> = BTreeMap::new();
    let mut files = Vec::new();
    for path in paths {
        let text = match std::fs::read_to_string(&path) {
            Ok(text) => text,
            Err(e) => {
                errors.push(GenesisError::Read {
                    path,
                    message: e.to_string(),
                });
                continue;
            }
        };
        let value: Value = match serde_norway::from_str(&text) {
            Ok(value) => value,
            Err(e) => {
                errors.push(GenesisError::Parse {
                    path,
                    message: e.to_string(),
                });
                continue;
            }
        };
        let Some(kind) = derive_kind(&value) else {
            errors.push(GenesisError::UnknownKind { path });
            continue;
        };
        let doc = DocRef::new(kind, stem_of(&path));
        if let Some(first) = seen.get(&doc) {
            let _ = first;
            errors.push(GenesisError::DuplicateDocument {
                doc: doc.clone(),
                path: path.clone(),
            });
            continue;
        }
        seen.insert(doc.clone(), path.clone());
        files.push(ShippedFile {
            doc,
            path,
            text,
            value,
        });
    }

    if errors.is_empty() {
        files.sort_by(|a, b| a.doc.cmp(&b.doc));
        Ok(files)
    } else {
        Err(errors)
    }
}

fn derive_kind(value: &Value) -> Option<DocKind> {
    match value.get("kind").and_then(|v| v.as_str()) {
        Some(token) => DocKind::parse(token.trim()),
        None if value.get("template").is_some() => Some(DocKind::Template),
        None if value.get("shelf").is_some() => Some(DocKind::Shelf),
        None => None,
    }
}

/// A file's document name: its directory for an in-directory skill schema, else its stem.
fn stem_of(path: &Path) -> String {
    if path.file_name().and_then(|n| n.to_str()) == Some("schema.yaml") {
        return path
            .parent()
            .and_then(Path::file_name)
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();
    }
    path.file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or_default()
        .to_string()
}

// ---------------------------------------------------------------------------
// the comment-carried reasons
// ---------------------------------------------------------------------------

/// Every `# D6 empty-with-reason:` comment in a file, keyed by the rule it sits inside.
///
/// The scan is line-based because it is reading what the parser throws away. A rule opens at its
/// `- id:` line; comment lines accumulate; the block is claimed by the `enforces:` that follows
/// it, and any other content between them clears it, so a comment about something else can never
/// be attributed to a mirror it merely precedes.
pub fn empty_enforces_reasons(text: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let mut current: Option<String> = None;
    let mut pending: Vec<String> = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("- id:") {
            current = Some(rest.trim().to_string());
            pending.clear();
            continue;
        }
        if let Some(comment) = trimmed.strip_prefix('#') {
            pending.push(comment.trim().to_string());
            continue;
        }
        if trimmed.starts_with("enforces:") && trimmed.replace(' ', "").ends_with("[]") {
            if let (Some(id), Some(first)) = (current.as_ref(), pending.first()) {
                if let Some(head) = first.strip_prefix(EMPTY_ENFORCES_MARKER) {
                    let mut parts = vec![head.trim().to_string()];
                    parts.extend(pending.iter().skip(1).cloned());
                    out.insert(
                        id.clone(),
                        parts
                            .into_iter()
                            .filter(|p| !p.is_empty())
                            .collect::<Vec<_>>()
                            .join(" "),
                    );
                }
            }
        }
        pending.clear();
    }
    out
}

// ---------------------------------------------------------------------------
// the sidecar
// ---------------------------------------------------------------------------

/// The provenance sidecar's anchors, keyed by rule id.
pub fn sidecar_anchors(root: &Path) -> Result<BTreeMap<String, String>, Vec<GenesisError>> {
    let path = root.join(SIDECAR);
    let text = std::fs::read_to_string(&path).map_err(|e| {
        vec![GenesisError::Read {
            path: path.clone(),
            message: e.to_string(),
        }]
    })?;
    let value: Value = serde_norway::from_str(&text).map_err(|e| {
        vec![GenesisError::Parse {
            path,
            message: e.to_string(),
        }]
    })?;

    match value.get("kind").and_then(|v| v.as_str()) {
        Some(kind) if SIDECAR_KINDS.contains(&kind.trim()) => {}
        Some(other) => {
            return Err(vec![GenesisError::SidecarKind {
                found: other.to_string(),
            }])
        }
        None => {
            return Err(vec![GenesisError::SidecarKind {
                found: "<absent>".to_string(),
            }])
        }
    }

    let Some(Value::Mapping(anchors)) = value.get("anchors") else {
        return Err(vec![GenesisError::SidecarShape {
            message: "`anchors:` must be a mapping of rule id to ruling anchor".to_string(),
        }]);
    };

    let mut out = BTreeMap::new();
    let mut errors = Vec::new();
    for (key, value) in anchors {
        match (key.as_str(), value.as_str()) {
            (Some(id), Some(anchor)) => {
                out.insert(id.to_string(), anchor.to_string());
            }
            _ => errors.push(GenesisError::SidecarShape {
                message: format!("entry {key:?} is not `<rule id>: \"<anchor>\"`"),
            }),
        }
    }
    if errors.is_empty() {
        Ok(out)
    } else {
        Err(errors)
    }
}

// ---------------------------------------------------------------------------
// documents
// ---------------------------------------------------------------------------

/// Every shipped document decoded, with the comment-carried reasons lifted into `note:` and the
/// sidecar's anchors folded onto their rules.
pub fn documents(root: &Path) -> Result<Vec<(DocRef, Document)>, Vec<GenesisError>> {
    let files = scan(root)?;
    let anchors = sidecar_anchors(root)?;

    let mut errors = Vec::new();
    let mut decoded: Vec<(DocRef, Document)> = Vec::new();
    for file in &files {
        match Document::from_value(file.doc.kind, &file.value) {
            Ok(mut document) => {
                let reasons = empty_enforces_reasons(&file.text);
                if let Some(schema) = document.as_rules_mut() {
                    for rule in schema
                        .sections
                        .iter_mut()
                        .flat_map(|s| s.rules.iter_mut())
                        .chain(schema.blocks.iter_mut())
                    {
                        if rule.enforces.as_deref().is_some_and(<[String]>::is_empty) {
                            match reasons.get(&rule.id) {
                                Some(note) => rule.note = Some(note.clone()),
                                None => errors.push(GenesisError::UnexplainedEmptyEnforces {
                                    doc: file.doc.clone(),
                                    id: rule.id.clone(),
                                }),
                            }
                        }
                    }
                }
                decoded.push((file.doc.clone(), document));
            }
            Err(e) => errors.push(GenesisError::Decode {
                path: file.path.clone(),
                message: e.to_string(),
            }),
        }
    }

    // Who owns each rule id, so an anchor lands on exactly one rule or is reported.
    let mut owners: BTreeMap<&str, Vec<&DocRef>> = BTreeMap::new();
    for (doc, document) in &decoded {
        if let Some(schema) = document.as_rules() {
            for rule in schema.rules() {
                owners.entry(rule.id.as_str()).or_default().push(doc);
            }
        }
    }
    for (id, docs) in &owners {
        if docs.len() > 1 && anchors.contains_key(*id) {
            errors.push(GenesisError::AmbiguousRule {
                id: (*id).to_string(),
                docs: docs.iter().map(|d| d.to_string()).collect(),
            });
        }
    }
    for id in anchors.keys() {
        if !owners.contains_key(id.as_str()) {
            errors.push(GenesisError::DanglingAnchor { id: id.clone() });
        }
    }

    for (_, document) in decoded.iter_mut() {
        if let Some(schema) = document.as_rules_mut() {
            for rule in schema
                .sections
                .iter_mut()
                .flat_map(|s| s.rules.iter_mut())
                .chain(schema.blocks.iter_mut())
            {
                if let Some(anchor) = anchors.get(&rule.id) {
                    rule.anchor = Some(anchor.clone());
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(decoded)
    } else {
        Err(errors)
    }
}

/// How many rules the fold actually anchored — the figure the report pins.
pub fn anchored_rules(documents: &[(DocRef, Document)]) -> usize {
    documents
        .iter()
        .filter_map(|(_, document)| document.as_rules())
        .flat_map(crate::model::RuleSchema::rules)
        .filter(|rule: &&Rule| rule.anchor.is_some())
        .count()
}

// ---------------------------------------------------------------------------
// the migration file
// ---------------------------------------------------------------------------

/// Build the genesis migration's text: one `import-document` op per shipped document, stamped
/// with its body hash.
pub fn build(root: &Path) -> Result<String, Vec<GenesisError>> {
    let documents = documents(root)?;

    let mut changes = Vec::with_capacity(documents.len());
    for (doc, document) in &documents {
        let mut op = Mapping::new();
        op.insert(
            Value::String("op".into()),
            Value::String("import-document".into()),
        );
        op.insert(
            Value::String("kind".into()),
            Value::String(doc.kind.as_str().to_string()),
        );
        op.insert(
            Value::String("name".into()),
            Value::String(doc.name.clone()),
        );
        op.insert(Value::String("content".into()), document.to_value());
        changes.push(Value::Mapping(op));
    }

    let mut header = Mapping::new();
    header.insert(Value::String("grammar".into()), Value::Number(1.into()));
    header.insert(Value::String("id".into()), Value::String(ID.to_string()));
    header.insert(
        Value::String("sequence".into()),
        Value::Number(SEQUENCE.into()),
    );
    header.insert(
        Value::String("intent".into()),
        Value::String(INTENT.to_string()),
    );
    header.insert(
        Value::String("anchor".into()),
        Value::String(ANCHOR.to_string()),
    );
    header.insert(
        Value::String("changes".into()),
        Value::Sequence(changes.clone()),
    );

    // The hash comes from the sanctioned stamping path rather than from a second copy of the
    // canonical encoding here. `with_hash` re-serialises through serde, which is why its output
    // is used for the hash alone and the file itself is written by the view writer: the hash is
    // over the parsed body, so it is the same for both spellings.
    let unstamped = crate::views::to_yaml(&Value::Mapping(header.clone()));
    let stamped = migration::with_hash(FILE, &unstamped).map_err(|e| {
        vec![GenesisError::Stamp {
            message: e.to_string(),
        }]
    })?;
    let hash = serde_norway::from_str::<Value>(&stamped)
        .ok()
        .and_then(|v| {
            v.get("hash")
                .and_then(|h| h.as_str())
                .map(std::string::ToString::to_string)
        })
        .ok_or_else(|| {
            vec![GenesisError::Stamp {
                message: "the stamped body carries no `hash:` header".to_string(),
            }]
        })?;

    let mut final_header = Mapping::new();
    for (key, value) in &header {
        if key.as_str() == Some("changes") {
            final_header.insert(Value::String("hash".into()), Value::String(hash.clone()));
        }
        final_header.insert(key.clone(), value.clone());
    }

    Ok(format!(
        "{PREAMBLE}{}",
        crate::views::to_yaml(&Value::Mapping(final_header))
    ))
}

/// The comment block the generated file opens with. It is the one place a reader is told the
/// file is generated and how to regenerate it.
const PREAMBLE: &str = "\
# GENERATED — the genesis migration, written by `mochiko-cli genesis emit --out <path>`.
# It imports the shipped schema corpus as the log's baseline: one `import-document` op per
# document, in address order, carrying the provenance sidecar's anchors on their rules and the
# two comment-carried `enforces: []` reasons as `note:` data.
#
# Do not hand-edit. `tests/fidelity.rs` regenerates this file and compares it byte-for-byte, so
# an edit here is a test failure, and a corpus change is a new migration rather than a rewrite.
";
