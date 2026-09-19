//! Artifact homes: where each kind of produced markdown lives, what file names it may carry, and
//! which template binds each file.
//!
//! A `home` document is the log's declaration of one artifact directory (record D3). It carries
//! the directory's path as an ordered segment pattern, the closed set of deliverable file names,
//! the kind-to-template binding per file, the size posture, and whether the home opens a
//! `reports/` directory. The store holds it opaquely, like a template, and this module decodes it
//! at the point of use — so the log stays the single editing surface and no schema file ships
//! (GI-020).
//!
//! # What this module does and does not do
//!
//! It resolves a repository-relative path to at most one home and one verdict about the name
//! inside it. It reads no file and grades no content: deciding whether a *write* conforms is
//! [`crate::conform`]'s job, and neither module holds judgment a skill owns (GI-019).
//!
//! # Why the segment tokens live here and not in the log
//!
//! A home's `path` is a list of segments, each a literal or one of [`TOKENS`]. The vocabulary is
//! the binary's, deliberately: a regex in the log could express a pattern no deny reason could
//! explain back to the seat that tripped it, and the token's own name is what the reason prints.
//! An eighth token is therefore a crate change, which is the same friction a new deliverable kind
//! takes (record D2).

use crate::model::{DocKind, DocRef, Document};
use crate::replay::State;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Component, Path};

/// The segment tokens a home's `path` and a deliverable's `file` may use.
///
/// Order is irrelevant to matching; the list is the vocabulary a finding cites when a home
/// declares a token that is not one of these.
pub const TOKENS: [&str; 7] = [
    "<slug>",
    "<date-slug>",
    "<FEAT-ID>",
    "<EPIC-ID>",
    "<AX-ID>",
    "<n>",
    "<any>",
];

/// How a home's files are bounded.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Bounds {
    /// Each file's bound is its template's per-section budgets (record D6).
    Template,
    /// A single whole-file bound per deliverable, the placeholder until a template lands (D4f).
    WholeFile,
    /// The bounds live in another named constraint home, cited by `bounds_cite` (D4f's exemption,
    /// V2). Size is not checked here at all — the root operating docs are the case.
    Elsewhere,
}

/// A deliverable whose shape is an append-only log rather than a document.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Form {
    /// One entry is one `##` block, and the bound is per entry, never per file (record D4d).
    Log,
}

/// One declared file in a home.
///
/// `file` is a name pattern over the same [`TOKENS`] the path uses, because F9's homes name files
/// by pattern as well as literally — `.mochiko/decisions/<date-slug>.md`, `stories/US-<n>.md`.
#[derive(Clone, Debug, Deserialize)]
pub struct Deliverable {
    pub file: String,
    /// The template whose conformance block shapes this file, when one exists.
    #[serde(default)]
    pub template: Option<String>,
    /// The whole-file bound, for a deliverable with no template.
    #[serde(default)]
    pub max_lines: Option<usize>,
    #[serde(default)]
    pub form: Option<Form>,
    /// The per-entry bound for `form: log`.
    #[serde(default)]
    pub entry_max_lines: Option<usize>,
    /// Why this template-less deliverable carries no size bound at all.
    ///
    /// The per-deliverable analogue of a home's `bounds_cite`: a bound may be declared absent, but
    /// never silently absent. Some artifacts are as long as their subject — a decision record's
    /// length is the session's — and a number invented for them would be enforced against work it
    /// cannot measure. Present with no `max_lines` is the declared-unbounded class; present *with*
    /// one is a contradiction `migrate validate` rejects.
    #[serde(default)]
    pub bound_reason: Option<String>,
}

/// A home's `reports/` directory: names are free, types are enumerated (record D2).
#[derive(Clone, Debug, Deserialize)]
pub struct Reports {
    /// The template whose conformance block carries the `report:` enum every report must hold.
    pub envelope: String,
    /// A per-type template, where that type's sections carry their own budgets (D2/M8).
    #[serde(default)]
    pub by_type: BTreeMap<String, String>,
}

/// One artifact home, decoded from its opaque store document.
///
/// Unknown keys are ignored rather than rejected, matching the template model: a home can grow a
/// field without breaking an older binary. That tolerance is exactly why a *later* migration
/// adding a field alone must bump the grammar — see the module docs of [`crate::migration`].
#[derive(Clone, Debug, Deserialize)]
pub struct Home {
    pub home: String,
    pub title: String,
    /// The home directory as ordered segments, each a literal or one of [`TOKENS`].
    pub path: Vec<String>,
    pub bounds: Bounds,
    /// The constraint home the bounds live in. Required when `bounds` is `Elsewhere`.
    #[serde(default)]
    pub bounds_cite: Option<String>,
    #[serde(default)]
    pub deliverables: Vec<Deliverable>,
    /// Sub-directories this home admits. A sub-directory not listed here is denied; a listed one
    /// is governed by its own home document, or by nothing at all until one lands.
    #[serde(default)]
    pub subdirs: Vec<String>,
    #[serde(default)]
    pub reports: Option<Reports>,
}

impl Home {
    /// How many of the path's segments are literals. The tie-breaker when two homes match.
    pub fn literal_depth(&self) -> usize {
        self.path.iter().filter(|s| !is_token(s)).count()
    }

    /// The home directory as a display path, for a deny reason or the `home` render.
    pub fn display_path(&self) -> String {
        format!("{}/", self.path.join("/"))
    }

    /// The declared deliverable names, in declaration order, for a deny reason.
    pub fn declared_names(&self) -> Vec<&str> {
        self.deliverables.iter().map(|d| d.file.as_str()).collect()
    }
}

/// What a path resolved to.
///
/// Every variant a deny reason needs is its own case: the reason has to name the home, and say
/// whether the problem is the file name, a sub-directory, or nothing at all.
#[derive(Debug)]
pub enum Resolution<'a> {
    /// No declared home governs this path.
    Outside,
    /// A declared deliverable of a home.
    File {
        home: &'a Home,
        deliverable: &'a Deliverable,
    },
    /// A file under a home's declared `reports/` directory — any name, one envelope.
    Report {
        home: &'a Home,
        reports: &'a Reports,
        name: String,
    },
    /// A name the home does not declare.
    UndeclaredFile { home: &'a Home, name: String },
    /// A sub-directory the home does not declare.
    UndeclaredSubdir { home: &'a Home, subdir: String },
    /// A sub-directory the home declares but does not itself govern. Its own home document
    /// governs it, or nothing does yet — and inventing a rule here is what D4f forbids.
    Deferred { home: &'a Home, subdir: String },
}

/// The name of the open-by-name reports directory inside a home.
pub const REPORTS_DIR: &str = "reports";

/// Whether a segment is a token rather than a literal.
fn is_token(segment: &str) -> bool {
    segment.starts_with('<') && segment.ends_with('>') && segment.len() > 2
}

/// Whether one path segment matches one pattern segment.
///
/// A literal matches itself exactly. A token matches its own shape. An undeclared token matches
/// nothing at all — a typo in the log must not silently behave as a wildcard.
pub fn segment_matches(pattern: &str, segment: &str) -> bool {
    if segment.is_empty() {
        return false;
    }
    if !is_token(pattern) {
        return pattern == segment;
    }
    match pattern {
        "<any>" => true,
        "<slug>" => is_slug(segment),
        "<date-slug>" => is_date_slug(segment),
        "<FEAT-ID>" => is_prefixed_id("FEAT", segment, false),
        "<EPIC-ID>" => is_prefixed_id("EPIC", segment, false),
        "<AX-ID>" => is_prefixed_id("AX", segment, true),
        "<n>" => segment.chars().all(|c| c.is_ascii_digit()),
        _ => false,
    }
}

/// Lower-case kebab: segments of `[a-z0-9]+` joined by single hyphens.
fn is_slug(segment: &str) -> bool {
    !segment.is_empty()
        && segment.split('-').all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        })
}

/// `YYYY-MM-DD-<slug>`: a range-checked date, then a slug.
fn is_date_slug(segment: &str) -> bool {
    let bytes = segment.as_bytes();
    if bytes.len() < 12 || bytes[4] != b'-' || bytes[7] != b'-' || bytes[10] != b'-' {
        return false;
    }
    let digits = |range: std::ops::Range<usize>| segment[range].chars().all(|c| c.is_ascii_digit());
    if !digits(0..4) || !digits(5..7) || !digits(8..10) {
        return false;
    }
    let month: u32 = segment[5..7].parse().unwrap_or(0);
    let day: u32 = segment[8..10].parse().unwrap_or(0);
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return false;
    }
    is_slug(&segment[11..])
}

/// `<PREFIX>-<digits>` with at least three digits, optionally `-<slug>`. `AX` ids always carry
/// the trailing slug; `FEAT` and `EPIC` ids may stand alone.
fn is_prefixed_id(prefix: &str, segment: &str, slug_required: bool) -> bool {
    let Some(rest) = segment.strip_prefix(prefix) else {
        return false;
    };
    let Some(rest) = rest.strip_prefix('-') else {
        return false;
    };
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.len() < 3 {
        return false;
    }
    let tail = &rest[digits.len()..];
    match tail.strip_prefix('-') {
        Some(slug) => is_slug(slug),
        None => tail.is_empty() && !slug_required,
    }
}

/// Whether a file name matches a deliverable's name pattern.
///
/// The pattern is matched token-wise over the name's hyphen-free shape: a `<token>` inside a file
/// name covers one run of characters up to the next literal. Only the tokens a name realistically
/// carries are supported, which keeps the matcher a scan rather than a parser.
pub fn file_matches(pattern: &str, name: &str) -> bool {
    if !pattern.contains('<') {
        return pattern == name;
    }
    let mut rest = name;
    let mut cursor = pattern;
    while let Some(open) = cursor.find('<') {
        let literal = &cursor[..open];
        if !rest.starts_with(literal) {
            return false;
        }
        rest = &rest[literal.len()..];
        let Some(close) = cursor[open..].find('>') else {
            return false;
        };
        let token = &cursor[open..open + close + 1];
        cursor = &cursor[open + close + 1..];
        // The token's run ends where the pattern's next literal begins.
        let next_literal = cursor.split('<').next().unwrap_or("");
        let run_end = if next_literal.is_empty() {
            rest.len()
        } else {
            match rest.find(next_literal) {
                Some(at) => at,
                None => return false,
            }
        };
        if !segment_matches(token, &rest[..run_end]) {
            return false;
        }
        rest = &rest[run_end..];
    }
    rest == cursor
}

/// The homes a log declares, decoded once and owned.
///
/// Resolution borrows from this set, so the caller holds it for as long as it holds a
/// [`Resolution`]. One decode per invocation is the whole cost, and it keeps the type free of the
/// process-lifetime cache a borrow-from-`State` signature would have needed.
#[derive(Debug, Default)]
pub struct Homes(Vec<Home>);

impl Homes {
    /// Decode every home in the state, most specific pattern first.
    ///
    /// A document that does not decode is skipped rather than panicking: `migrate validate` is
    /// where a malformed home is reported, and a delivery path must not die on it.
    pub fn load(state: &State) -> Homes {
        let mut homes: Vec<Home> = state
            .docs
            .iter()
            .filter(|(doc, _)| doc.kind == DocKind::Home)
            .filter_map(|(_, document)| match document {
                Document::Opaque(value) => serde_norway::from_value::<Home>(value.clone()).ok(),
                _ => None,
            })
            .collect();
        homes.sort_by(|a, b| {
            b.literal_depth()
                .cmp(&a.literal_depth())
                .then_with(|| b.path.len().cmp(&a.path.len()))
                .then_with(|| a.home.cmp(&b.home))
        });
        Homes(homes)
    }

    /// Resolve a repository-relative path against this set.
    ///
    /// The home with the most literal segments wins, so a literal `desk` beats a `<FEAT-ID>`
    /// sibling. Only the directory prefix is matched here; the remainder decides which
    /// [`Resolution`] the path earns.
    pub fn resolve(&self, path: &Path) -> Resolution<'_> {
        resolve_in(&self.0, path)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Home> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

/// Decode one home by name, or `None`.
pub fn get(state: &State, name: &str) -> Option<Home> {
    let doc = DocRef::new(DocKind::Home, name);
    match state.docs.get(&doc) {
        Some(Document::Opaque(value)) => serde_norway::from_value::<Home>(value.clone()).ok(),
        _ => None,
    }
}

/// Split a repository-relative path into plain segments, or `None` when it is not one.
///
/// An absolute path, a traversal, or a `.` component resolves to nothing: a home is a place in the
/// repository, and a path that could escape one must never be treated as inside it.
fn segments(path: &Path) -> Option<Vec<String>> {
    let mut out = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => out.push(part.to_string_lossy().into_owned()),
            // RootDir, Prefix, CurDir and ParentDir all mean this is not a plain relative path.
            _ => return None,
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn resolve_in<'a>(homes: &'a [Home], path: &Path) -> Resolution<'a> {
    let Some(parts) = segments(path) else {
        return Resolution::Outside;
    };
    for home in homes {
        let depth = home.path.len();
        // The remainder must hold at least a file name, so a path that is only the home itself is
        // not inside it.
        if parts.len() <= depth {
            continue;
        }
        let matches = home
            .path
            .iter()
            .zip(parts.iter())
            .all(|(pattern, segment)| segment_matches(pattern, segment));
        if !matches {
            continue;
        }
        let rest = &parts[depth..];
        if rest.len() == 1 {
            let name = &rest[0];
            if let Some(deliverable) = home
                .deliverables
                .iter()
                .find(|d| file_matches(&d.file, name))
            {
                return Resolution::File { home, deliverable };
            }
            return Resolution::UndeclaredFile {
                home,
                name: name.clone(),
            };
        }
        // Depth two or more: a reports file, a declared sub-directory, or a denial.
        let subdir = &rest[0];
        if subdir == REPORTS_DIR {
            return match &home.reports {
                Some(reports) if rest.len() == 2 => Resolution::Report {
                    home,
                    reports,
                    name: rest[1].clone(),
                },
                Some(_) => Resolution::UndeclaredSubdir {
                    home,
                    subdir: rest[1].clone(),
                },
                None => Resolution::UndeclaredSubdir {
                    home,
                    subdir: subdir.clone(),
                },
            };
        }
        if home.subdirs.iter().any(|s| s == subdir) {
            return Resolution::Deferred {
                home,
                subdir: subdir.clone(),
            };
        }
        return Resolution::UndeclaredSubdir {
            home,
            subdir: subdir.clone(),
        };
    }
    Resolution::Outside
}
