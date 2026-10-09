//! Artifact homes: where each kind of produced markdown lives, what file names it may carry, and
//! which template binds each file.
//!
//! A `home` document is the log's declaration of one artifact directory (record D3). It carries
//! the directory's path as an ordered segment pattern, the closed set of deliverable file names,
//! the kind-to-template binding per file, the size posture, and whether the home opens a
//! `reports/` directory. The store holds it opaquely, like a template, and this module decodes it
//! at the point of use — so the log stays the single editing surface and no schema file ships
//! (GI-020-plugin-install-model).
//!
//! # What this module does and does not do
//!
//! It resolves a repository-relative path to at most one home and one verdict about the name
//! inside it. It grades no content: deciding whether a *write* conforms is [`crate::conform`]'s
//! job, and neither module holds judgment a skill owns (GI-019-kernel-tooling-admission).
//!
//! # Where a path's repository is (field review D7, as amended at S3)
//!
//! Before a path can be resolved it has to be placed: [`locate`] walks up from it to the nearest
//! ancestor holding `.git` — a directory, or the pointer file a linked worktree carries — and
//! that ancestor is the tree root. Only `<root>/.mochiko/` is a home tree, so a `.mochiko/` nested
//! deeper (a fixture, an eval workspace) is never a home, and a path whose ancestors hold no
//! `.git` at all has no home tree. The walk never keys on the caller's working directory, which
//! is what made the same file gated for one seat and not for another (F15).
//!
//! These are the only repository reads this module makes, and they are yes/no facts about the
//! layout: whether `<ancestor>/.git` exists, the one line of a worktree's pointer file that names
//! the main tree ([`Located::main_root`]), and whether `.gitignore` carries the run folder
//! ([`ignores`]). No artifact's content is read here.
//!
//! # Why the segment tokens live here and not in the log
//!
//! A home's `path` is a list of segments, each a literal or one of [`TOKENS`]. The vocabulary is
//! the binary's, deliberately: a regex in the log could express a pattern no deny reason could
//! explain back to the seat that tripped it, and the token's own name is what the reason prints.
//! A new token is therefore a crate change, which is the same friction a new deliverable kind
//! takes (record D2) — the eighth, `<run-id>`, landed with the run folder (seams R5).

use crate::model::{DocKind, DocRef, Document};
use crate::replay::State;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

/// The segment tokens a home's `path` and a deliverable's `file` may use.
///
/// Order is irrelevant to matching; the list is the vocabulary a finding cites when a home
/// declares a token that is not one of these.
pub const TOKENS: [&str; 8] = [
    "<slug>",
    "<date-slug>",
    "<FEAT-ID>",
    "<EPIC-ID>",
    "<AX-ID>",
    "<n>",
    "<any>",
    "<run-id>",
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

/// A deliverable bounded per entry rather than per file.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Form {
    /// An append-only log: one entry is one `##` block, and the bound is per entry, never per
    /// file (record D4d). It carries no shape rule.
    Log,
    /// A cumulative store — a product baseline, the architecture store — bounded per entry with
    /// no whole-file bound (field review D2). The entry heading level is declared, so one entity
    /// can be a `###` under a `##` grouping; a bound template still grades the store's shape, and
    /// the entry budgets replace its per-section ones (grammar 2).
    Entries,
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
    /// The per-entry bound for `form: log` and `form: entries`.
    #[serde(default)]
    pub entry_max_lines: Option<usize>,
    /// The heading that opens an entry under `form: entries`: `##` or `###`. An entry runs from
    /// its heading to the next heading at the same or a higher level.
    #[serde(default)]
    pub entry_heading: Option<String>,
    /// Under `###` entries, the bound on each `##` section's own text — its lines before its first
    /// entry. Summary tables and dated reconciliation notes live there, outside every entry.
    #[serde(default)]
    pub section_max_lines: Option<usize>,
    /// Field names whose `**<Name>:**` lines an entry's count skips, so a lifecycle marker added
    /// at every run cannot push an honest entry over its budget.
    #[serde(default)]
    pub entry_exempt_fields: Vec<String>,
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
/// adding a field must bump the grammar — an older binary would read the field's absence, not
/// its value. [`crate::migration::GRAMMAR_2_HOME_FIELDS`] lists the fields grammar 2 added, and a
/// grammar-1 migration carrying one is rejected at parse.
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
    /// Whether this is the raw-output home: the one run folder where captured console, logs and
    /// dumps go (field review D4, as amended). At most one home carries it. Any path under it
    /// resolves as [`Resolution::Raw`] except its declared deliverables; shape and size are
    /// unchecked, and the gate's own controls — the ignore guard, the report sniff on `.md`, the
    /// main-tree rule — are applied by [`crate::hook`].
    #[serde(default)]
    pub raw_output: bool,
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

    /// The `.gitignore` line the run folder needs: the path's literal prefix, before its first
    /// token, with a trailing `/` — `.mochiko/runs/` for `[.mochiko, runs, <run-id>]`.
    pub fn ignore_line(&self) -> String {
        format!("{}/", self.literal_prefix().join("/"))
    }

    /// The path's leading literal segments, before its first token.
    fn literal_prefix(&self) -> &[String] {
        let literal = self.path.iter().take_while(|s| !is_token(s)).count();
        &self.path[..literal]
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
    /// A sub-directory the home does not declare — one under `reports/` named with that prefix.
    UndeclaredSubdir { home: &'a Home, subdir: String },
    /// A sub-directory the home declares but does not itself govern. Its own home document
    /// governs it, or nothing does yet — and inventing a rule here is what D4f forbids.
    Deferred { home: &'a Home, subdir: String },
    /// Any path under the raw-output home other than one of its declared names: raw output, whose
    /// shape and size are unchecked. `rest` is the path below the run folder.
    Raw { home: &'a Home, rest: String },
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
        "<run-id>" => is_run_id(segment),
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

/// `<owner>-run<n>` (seams R5): the owner is a FEAT or EPIC id in its existing shape, or
/// `lane-<slug>`; then `-run` and one or more digits. The split is at the last `-run`, so an
/// owner's own slug may contain the word.
pub(crate) fn is_run_id(segment: &str) -> bool {
    let Some(at) = segment.rfind("-run") else {
        return false;
    };
    let (owner, number) = (&segment[..at], &segment[at + "-run".len()..]);
    if number.is_empty() || !number.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    is_prefixed_id("FEAT", owner, false)
        || is_prefixed_id("EPIC", owner, false)
        || owner.strip_prefix("lane-").is_some_and(is_slug)
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

    /// The raw-output home, when the log declares one ([`Home::raw_output`]).
    pub fn raw_output(&self) -> Option<&Home> {
        self.0.iter().find(|home| home.raw_output)
    }

    /// The run folder's absolute path for a path placed at `located`, when the log declares a
    /// raw-output home: that home's directory under the **main** tree, so a seat in a worktree is
    /// sent to the one folder every seat of the run shares (field review V3).
    pub fn run_folder(&self, located: &Located) -> Option<String> {
        let home = self.raw_output()?;
        Some(format!(
            "{}/{}",
            located.main_root().display(),
            home.display_path()
        ))
    }

    /// The home sharing the longest leading run of segments with a path no home governs, and how
    /// many segments it shares.
    ///
    /// This is the "nearest home" a closed-world refusal names (field review D3): its declared
    /// files and `reports/` are where a misplaced artifact belongs. Sharing only `.mochiko` itself
    /// makes no home nearer than any other, so fewer than two shared segments is `None`. Ties go
    /// to the first home in [`Homes::load`]'s order, the same order resolution tries them in.
    pub fn nearest(&self, path: &Path) -> Option<(&Home, usize)> {
        let parts = segments(path)?;
        let mut best: Option<(&Home, usize)> = None;
        for home in &self.0 {
            let shared = home
                .path
                .iter()
                .zip(parts.iter())
                .take_while(|(pattern, segment)| segment_matches(pattern, segment))
                .count();
            let nearer = match best {
                Some((_, so_far)) => shared > so_far,
                None => shared >= 2,
            };
            if nearer {
                best = Some((home, shared));
            }
        }
        best
    }

    /// The home whose directory this repository-relative path names, or `None`.
    ///
    /// Every segment must match and the path must end where the home's pattern ends. The most
    /// specific home wins, in the same order [`Homes::resolve`] tries them.
    pub fn resolve_dir(&self, path: &Path) -> Option<&Home> {
        let parts = segments(path)?;
        self.0.iter().find(|home| {
            home.path.len() == parts.len()
                && home
                    .path
                    .iter()
                    .zip(parts.iter())
                    .all(|(pattern, segment)| segment_matches(pattern, segment))
        })
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

/// Collapse `.` components and resolve `..` lexically, so no traversal survives into a home.
///
/// Lexical on purpose: the path a write names need not exist yet, so it cannot be canonicalised.
pub fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// A path made absolute: a relative one joined to `cwd`, then normalized. `None` for a relative
/// path with no `cwd` to join it to — there is nothing to resolve it against.
pub fn absolute(path: &Path, cwd: Option<&Path>) -> Option<PathBuf> {
    if path.is_absolute() {
        return Some(normalize(path));
    }
    cwd.map(|cwd| normalize(&cwd.join(path)))
}

/// Where an absolute path sits: its tree root and its path inside that tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Located {
    /// The nearest ancestor holding `.git`, a directory or a worktree's pointer file.
    pub root: PathBuf,
    /// The path relative to `root`, which is what [`Homes::resolve`] takes.
    pub relative: PathBuf,
}

impl Located {
    /// Whether the path sits under `<root>/.mochiko/`, the one home tree — the closed world.
    pub fn in_home_tree(&self) -> bool {
        self.relative
            .components()
            .next()
            .is_some_and(|first| first.as_os_str() == HOME_TREE)
    }

    /// The main tree's root: `root` itself, unless `root` is a linked worktree.
    ///
    /// A worktree's `.git` is a file whose first line is `gitdir: <main>/.git/worktrees/<name>`
    /// (relative to the worktree when git writes it relative). The run folder is always the main
    /// tree's (field review V3), so this is read only to name it. A pointer of any other shape —
    /// a submodule's `…/.git/modules/<name>` — or one that cannot be read leaves `root` as its own
    /// main tree: nothing here can say otherwise, and a guess would name a folder that is not one.
    pub fn main_root(&self) -> PathBuf {
        let pointer = self.root.join(".git");
        if pointer.is_dir() {
            return self.root.clone();
        }
        let Ok(text) = std::fs::read_to_string(&pointer) else {
            return self.root.clone();
        };
        let Some(gitdir) = text
            .lines()
            .next()
            .and_then(|line| line.trim().strip_prefix("gitdir:"))
        else {
            return self.root.clone();
        };
        let gitdir = normalize(&self.root.join(gitdir.trim()));
        let worktrees = gitdir.parent();
        let dot_git = worktrees
            .filter(|dir| dir.file_name().is_some_and(|name| name == "worktrees"))
            .and_then(Path::parent)
            .filter(|dir| dir.file_name().is_some_and(|name| name == ".git"));
        match dot_git.and_then(Path::parent) {
            Some(main) => main.to_path_buf(),
            None => self.root.clone(),
        }
    }
}

/// Whether `<root>/.gitignore` carries the raw-output home's ignore line (field review D4
/// control 3), so nothing written to the run folder can reach a commit.
///
/// A line matches when, trimmed, it is the prefix with or without a leading `/` and with or
/// without the trailing `/`: `.mochiko/runs/`, `/.mochiko/runs/`, `.mochiko/runs`,
/// `/.mochiko/runs`. A comment, a negation, a glob or a parent directory is not the line — a
/// guard that tried to evaluate git's ignore rules would be a second implementation of git.
/// A missing or unreadable `.gitignore` carries nothing.
pub fn ignores(root: &Path, home: &Home) -> bool {
    let line = home.ignore_line();
    let bare = line.trim_end_matches('/');
    let Ok(text) = std::fs::read_to_string(root.join(".gitignore")) else {
        return false;
    };
    text.lines().map(str::trim).any(|entry| {
        let entry = entry.strip_prefix('/').unwrap_or(entry);
        entry == line || entry == bare
    })
}

/// The one directory name a home tree carries at its root.
pub const HOME_TREE: &str = ".mochiko";

/// Place an absolute path in its repository, or `None` when no ancestor holds `.git`.
///
/// The nearest `.git` wins, so a worktree under `.claude/worktrees/` resolves to its own tree and
/// a path in another repository resolves against that repository.
pub fn locate(absolute: &Path) -> Option<Located> {
    absolute
        .ancestors()
        .find(|ancestor| ancestor.join(".git").exists())
        .map(|root| Located {
            root: root.to_path_buf(),
            relative: absolute
                .strip_prefix(root)
                .map(Path::to_path_buf)
                .unwrap_or_default(),
        })
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
        // The run folder's parent belongs to the raw-output home alone (census row N3): a path
        // under its literal prefix that no run folder holds resolves to no home, so the closed
        // world names the run key it missed, rather than a shallower home — one at `.mochiko/`
        // itself — claiming the path as an undeclared sub-directory of its own.
        let claimed = home.raw_output && parts.starts_with(home.literal_prefix());
        // The remainder must hold at least a file name, so a path that is only the home itself is
        // not inside it.
        if parts.len() <= depth {
            if claimed {
                return Resolution::Outside;
            }
            continue;
        }
        let matches = home
            .path
            .iter()
            .zip(parts.iter())
            .all(|(pattern, segment)| segment_matches(pattern, segment));
        if !matches {
            if claimed {
                return Resolution::Outside;
            }
            continue;
        }
        let rest = &parts[depth..];
        if home.raw_output {
            // The run folder takes any name at any depth; only its declared names (the run log)
            // resolve as deliverables, so they are allowed by name (field review V7).
            if let [name] = rest {
                if let Some(deliverable) = home
                    .deliverables
                    .iter()
                    .find(|d| file_matches(&d.file, name))
                {
                    return Resolution::File { home, deliverable };
                }
            }
            return Resolution::Raw {
                home,
                rest: rest.join("/"),
            };
        }
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
                // Named with its prefix, so the reason cannot read as a sibling of `reports/`.
                Some(_) => Resolution::UndeclaredSubdir {
                    home,
                    subdir: format!("{REPORTS_DIR}/{}", rest[1]),
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
