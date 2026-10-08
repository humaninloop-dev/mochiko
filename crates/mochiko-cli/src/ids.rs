//! Human-readable IDs: the family table, the in-text scanner, and the advisory `ids --check`.
//!
//! An ID's joined form is its number then a slug, `GI-019-kernel-tooling-admission`
//! (`human-readable-ids` D19); the number inside its owning scope is the key, and the slug at the
//! definition is the source of truth (D4, D7). This module finds every ID token in a text, says
//! which ones are verbatim or machine-read and so left alone (D12, D15), resolves each to its
//! owner (D11 as changed at review and at build), and reports bare IDs and slug drift.
//!
//! # What it never does (the bright line, GI-019)
//!
//! It coins no slug — the seat supplies every one ([`crate::rename`]) — and it judges no word
//! choice: there is no word list here but the C4 model's fixed level names. `ids --check` reports
//! through its exit code only and nothing blocks on it (D6, D18). The only facts it reads besides
//! the text are yes/no facts about the tree: whether a directory holds `.git`, and which session
//! directories exist (R1, R2).

use crate::model::DocKind;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// How a family's number is written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Form {
    /// Three or more digits after a hyphen: `GI-004`, `FR-012`, `C-001`.
    Padded,
    /// One or more digits: `US-12`, session `D7`, cycle `C3`.
    Unpadded,
}

/// Where a family's numbers are unique.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    /// One sequence per project: `GI-020` means the same thing in every file.
    Project,
    /// One sequence per owning artifact: `FR-001` opens every spec.
    Artifact,
}

/// Where the log mints a family, and the text its minting rule shows.
#[derive(Clone, Copy, Debug)]
pub struct Mint {
    pub kind: DocKind,
    pub doc: &'static str,
    /// The rule carrying the minting text; `None` reads the whole document (a template).
    pub rule: Option<&'static str>,
    /// A substring of today's minting text, carrying the family's form.
    pub shows: &'static str,
}

/// One numbered ID family.
#[derive(Clone, Copy, Debug)]
pub struct Family {
    /// The name a report prints: `FR`, `C`, `session D`, `cycle C`.
    pub name: &'static str,
    /// The literal prefix, hyphen included where the family writes one.
    pub prefix: &'static str,
    pub form: Form,
    pub scope: Scope,
    /// The tree-relative path patterns of the files that hold the family's definitions.
    pub definitions: &'static [&'static str],
    /// The log's minting source, or `None` where the family mints in plugin prose.
    pub mint: Option<Mint>,
}

/// The definition files of the five per-spec families (F24: `FR`, `SC`, `US`, `SCR`, `FLOW`): the
/// spec and its `stories/` sub-home, which together are "the spec".
const SPEC: &[&str] = &[
    ".mochiko/specs/<slug>/spec.md",
    ".mochiko/specs/<slug>/stories/<any>",
];

/// The definition files of the five product-file families (`C-`, `D-`, `IP-`, `INT-`, `DS-`): a
/// spec's or the product baseline's `constraints-and-decisions.md`.
const PRODUCT_FILE: &[&str] = &[
    ".mochiko/specs/<slug>/constraints-and-decisions.md",
    ".mochiko/product/constraints-and-decisions.md",
];

/// The architecture store's concern files, where `AX` rows and their `NFR` targets live.
const CONCERNS: &[&str] = &[
    ".mochiko/product/architecture/concerns.md",
    ".mochiko/product/architecture/concerns/<AX-ID>.md",
];

const fn skill(doc: &'static str, rule: &'static str, shows: &'static str) -> Option<Mint> {
    Some(Mint {
        kind: DocKind::Skill,
        doc,
        rule: Some(rule),
        shows,
    })
}

const fn row(
    name: &'static str,
    prefix: &'static str,
    form: Form,
    scope: Scope,
    definitions: &'static [&'static str],
    mint: Option<Mint>,
) -> Family {
    Family {
        name,
        prefix,
        form,
        scope,
        definitions,
        mint,
    }
}

const TECH: &str = "authoring-technical-requirements";
const TECH_IDS: &str = "authoring-technical-requirements.sequential-ids";
const TECH_SHOWS: &str = "three-digit padded (C-, D-, IP-, INT-, DS-)";

/// Every numbered family the plugin mints — F24's facts as a table (`human-readable-ids` D6 as
/// changed at build, B3): prefix, padding (D9), numbering scope, definition site (D4, R3), and the
/// log rule that mints it, which `tests/ids.rs` replays to catch the table drifting from the log.
///
/// `GAP` and `BR` mint in plugin prose, outside the log (F20), so they carry no `mint`; `GAP` has
/// no definition line in any shipped template, so it carries no definition file either.
pub const FAMILIES: &[Family] = &[
    row(
        "GI",
        "GI-",
        Form::Padded,
        Scope::Project,
        &[".mochiko/memory/governance-intent.md"],
        Some(Mint {
            kind: DocKind::Template,
            doc: "governance-intent",
            rule: None,
            shows: "sequential GI-001, GI-002",
        }),
    ),
    row(
        "FEAT",
        "FEAT-",
        Form::Padded,
        Scope::Project,
        &[".mochiko/features/<FEAT-ID>.md"],
        skill(
            "authoring-feature-map",
            "authoring-feature-map.one-living-map",
            "FEAT-XXX",
        ),
    ),
    row(
        "EPIC",
        "EPIC-",
        Form::Padded,
        Scope::Project,
        &[".mochiko/epics/<EPIC-ID>/manifest.md"],
        skill(
            "authoring-epic",
            "authoring-epic.identity-grammar",
            "`EPIC-XXX`",
        ),
    ),
    row(
        "AX",
        "AX-",
        Form::Padded,
        Scope::Project,
        CONCERNS,
        skill(
            "authoring-architecture-store",
            "authoring-architecture-store.element-grammar",
            "`AX-XXX` (unique store-wide)",
        ),
    ),
    row(
        "SPN",
        "SPN-",
        Form::Padded,
        Scope::Project,
        &[".mochiko/product/architecture/spine.md"],
        skill(
            "authoring-architecture-store",
            "authoring-architecture-store.element-grammar",
            "`SPN-XXX` (unique store-wide)",
        ),
    ),
    row(
        "NFR",
        "NFR-",
        Form::Padded,
        Scope::Project,
        CONCERNS,
        skill(
            "authoring-architecture-store",
            "authoring-architecture-store.nfr-one-home",
            "`NFR-XXX` targets live on the concern row",
        ),
    ),
    row("GAP", "GAP-", Form::Padded, Scope::Project, &[], None),
    row(
        "FR",
        "FR-",
        Form::Padded,
        Scope::Artifact,
        SPEC,
        skill(
            "authoring-requirements",
            "authoring-requirements.fr-numbering",
            "three-digit padded, no gaps (FR-001",
        ),
    ),
    row(
        "SC",
        "SC-",
        Form::Padded,
        Scope::Artifact,
        SPEC,
        skill(
            "authoring-requirements",
            "authoring-requirements.sc-format",
            "SC-XXX form",
        ),
    ),
    row(
        "US",
        "US-",
        Form::Unpadded,
        Scope::Artifact,
        SPEC,
        skill(
            "authoring-user-stories",
            "authoring-user-stories.artifact-home",
            "`stories/US-<n>.md`",
        ),
    ),
    row(
        "SCR",
        "SCR-",
        Form::Padded,
        Scope::Artifact,
        SPEC,
        skill(
            "authoring-prototype",
            "authoring-prototype.two-coupled-artifacts",
            "`SCR-XXX`",
        ),
    ),
    row(
        "FLOW",
        "FLOW-",
        Form::Padded,
        Scope::Artifact,
        SPEC,
        skill(
            "authoring-prototype",
            "authoring-prototype.two-coupled-artifacts",
            "`FLOW-XXX`",
        ),
    ),
    row(
        "C",
        "C-",
        Form::Padded,
        Scope::Artifact,
        PRODUCT_FILE,
        skill(TECH, TECH_IDS, TECH_SHOWS),
    ),
    row(
        "D",
        "D-",
        Form::Padded,
        Scope::Artifact,
        PRODUCT_FILE,
        skill(TECH, TECH_IDS, TECH_SHOWS),
    ),
    row(
        "IP",
        "IP-",
        Form::Padded,
        Scope::Artifact,
        PRODUCT_FILE,
        skill(TECH, TECH_IDS, TECH_SHOWS),
    ),
    row(
        "INT",
        "INT-",
        Form::Padded,
        Scope::Artifact,
        PRODUCT_FILE,
        skill(TECH, TECH_IDS, TECH_SHOWS),
    ),
    row(
        "DS",
        "DS-",
        Form::Padded,
        Scope::Artifact,
        PRODUCT_FILE,
        skill(TECH, TECH_IDS, TECH_SHOWS),
    ),
    row(
        "BR",
        "BR-",
        Form::Padded,
        Scope::Artifact,
        &[".mochiko/product/data-model.md"],
        None,
    ),
    row(
        "session D",
        "D",
        Form::Unpadded,
        Scope::Artifact,
        &[".mochiko/brainstorms/<slug>/record.md"],
        Some(Mint {
            kind: DocKind::Command,
            doc: "brainstorm",
            rule: Some("brainstorm.deliverable-record"),
            shows: "`D1…`",
        }),
    ),
    row(
        "cycle C",
        "C",
        Form::Unpadded,
        Scope::Artifact,
        &[
            ".mochiko/features/<FEAT-ID>/tasks.md",
            ".mochiko/epics/<EPIC-ID>/tasks.md",
            ".mochiko/product/<slug>/tasks.md",
        ],
        Some(Mint {
            kind: DocKind::Template,
            doc: "tasks",
            rule: None,
            shows: "**Depends on:** C1",
        }),
    ),
];

/// One parsed ID core: its family and its number as written, sub-ID included.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Id {
    pub family: &'static Family,
    pub number: String,
}

impl PartialEq for Family {
    fn eq(&self, other: &Family) -> bool {
        self.name == other.name
    }
}

impl Eq for Family {}

/// The family with this report name.
pub fn family(name: &str) -> Option<&'static Family> {
    FAMILIES.iter().find(|family| family.name == name)
}

/// Parse a whole string as one ID core, with no slug.
pub fn parse_id(text: &str) -> Option<Id> {
    let (family, number, end) = core_at(text, 0, false)?;
    (end == text.len()).then_some(Id { family, number })
}

/// The families, longest prefix first, so `SCR-` is tried before `SC-` and `C-` before `C`;
/// sorted once (A2).
fn by_prefix() -> &'static [&'static Family] {
    static SORTED: std::sync::OnceLock<Vec<&'static Family>> = std::sync::OnceLock::new();
    SORTED.get_or_init(|| {
        let mut families: Vec<&'static Family> = FAMILIES.iter().collect();
        families.sort_by_key(|family| std::cmp::Reverse(family.prefix.len()));
        families
    })
}

/// Whether an ID core — a token's, or one glued into a repo-only form (`AD-D1`) — ends right at
/// byte `end`.
fn core_ends_at(text: &str, end: usize) -> bool {
    let digits = text[..end]
        .bytes()
        .rev()
        .take_while(u8::is_ascii_digit)
        .count();
    digits > 0
        && by_prefix().iter().any(|family| {
            let Some(start) = (end - digits).checked_sub(family.prefix.len()) else {
                return false;
            };
            text.is_char_boundary(start)
                && core_at(text, start, false).is_some_and(|(_, _, core_end)| core_end == end)
        })
}

/// The ID core starting at byte `at`: its family, its number as written, and the byte it ends at.
///
/// Families are tried longest prefix first, so `DS-003` is never read as `D`, and `D-007` never as
/// session `D`. `any_case` admits a lower-case prefix for the hyphenated families — a file name
/// such as `scr-001-week-menu.html` (D19 as changed at review, S5) — and never for the unpadded
/// `D<n>`/`C<n>`, which in lower case are ordinary words and library names (S3).
fn core_at(text: &str, at: usize, any_case: bool) -> Option<(&'static Family, String, usize)> {
    let rest = &text[at..];
    for &family in by_prefix() {
        let prefix = family.prefix;
        let Some(head) = rest.get(..prefix.len()) else {
            continue;
        };
        let matched = head == prefix
            || (any_case && prefix.ends_with('-') && head.eq_ignore_ascii_case(prefix));
        if !matched {
            continue;
        }
        let tail = &rest[prefix.len()..];
        let digits = tail.bytes().take_while(u8::is_ascii_digit).count();
        let enough = match family.form {
            Form::Padded => digits >= 3,
            Form::Unpadded => digits >= 1,
        };
        if !enough {
            continue;
        }
        let mut end = prefix.len() + digits;
        // A sub-ID rides the number: a lower-case letter run (`D2a`) or a dotted part (`D4.1`).
        let letters = rest[end..]
            .bytes()
            .take_while(u8::is_ascii_lowercase)
            .count();
        if letters > 0 {
            end += letters;
        } else if rest[end..].starts_with('.') {
            let dotted = rest[end + 1..]
                .bytes()
                .take_while(u8::is_ascii_digit)
                .count();
            if dotted > 0 {
                end += 1 + dotted;
            }
        }
        let number = rest[prefix.len()..end].to_string();
        return Some((family, number, at + end));
    }
    None
}

// ---------------------------------------------------------------------------
// the scanner
// ---------------------------------------------------------------------------

/// Why a token is neither checked nor rewritten.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Skip {
    /// A number-only path segment (`.mochiko/features/FEAT-001/`, `stories/US-12.md`): a
    /// machine-read spot that keeps the bare number (D12).
    Path,
    /// One end of a range (`D1–D7`, `D1-D7`): ranges stay bare (D5).
    Range,
    /// A member of a list of four or more: a range-like summary, bare (D5).
    List,
    /// A local label that shares a family's shape (D10): `C3-gate-2`, `FEAT-001-run2`, a C4 model
    /// level name.
    Label,
}

/// The owner name written in front of a per-artifact ID (D11).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Qualifier {
    /// As written: a session's record path stays a path here, read as its session on resolving
    /// (wave 1c).
    pub text: String,
    /// Whether it is a code span (`` `lunch-orders` FR-012 ``) or plain text (`adaptive-depth D7`).
    pub code: bool,
}

/// One ID token in a text.
#[derive(Clone, Debug)]
pub struct Token {
    /// The byte the ID core starts at.
    pub start: usize,
    /// The byte the core, and its slug where it carries one, ends at.
    pub end: usize,
    pub family: &'static Family,
    pub number: String,
    pub slug: Option<String>,
    /// Whether the token sits in a path or a file name.
    pub path: bool,
    pub qualifier: Option<Qualifier>,
    pub skip: Option<Skip>,
}

/// The C4 model's level names (S7 as ruled): `C4-container`, "C4 level-3" and the like are the
/// diagram vocabulary, a fixed external term set, never cycle 4.
const C4_TERMS: [&str; 6] = [
    "context",
    "container",
    "component",
    "code",
    "model",
    "level",
];

/// The separators that join a compound reference's members on one line (D5).
pub(crate) const SEPARATORS: [&str; 8] = [",", "/", "·", "&", "and", "or", ", and", ", or"];

/// The joiners that make the IDs on either side a range's ends (D5; N1 as ruled at build).
pub(crate) const RANGE_JOINERS: [&str; 3] = ["–", "…", "..."];

/// Every ID token in `text`, outside its masked spans.
///
/// A token is an ID core preceded by neither an alphanumeric nor `-`/`_` (so `PO-D1` and `XFR-001`
/// are not tokens), with the slug words that follow it: `-w`, each word `[a-z][a-z0-9]*` (D19).
/// Every token is returned, a skipped one with its [`Skip`], so a caller can tell a range or a
/// number-only path from a mention; the masked spans of [`masked`] yield no token at all.
pub fn scan(text: &str) -> Vec<Token> {
    scan_as(text, true)
}

/// [`scan`] under [`masked_as`], the quote mask on or off as [`quotes_masked`] says (N4).
pub fn scan_as(text: &str, quotes: bool) -> Vec<Token> {
    let mask = masked_as(text, quotes);
    let bytes = text.as_bytes();
    let mut tokens: Vec<Token> = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        if !bytes[at].is_ascii_alphabetic() || mask[at] || glued(text, at) {
            at += char_len(text, at);
            continue;
        }
        match token_at(text, at, tokens.last().map(|t| t.end)) {
            Some(token) => {
                at = token.end;
                tokens.push(token);
            }
            None => at += char_len(text, at),
        }
    }
    group_lists(text, &mut tokens);
    tokens
}

/// The width of the character at byte `at`.
fn char_len(text: &str, at: usize) -> usize {
    text[at..].chars().next().map_or(1, char::len_utf8)
}

/// Whether the byte at `at` continues a word, so no token can start there.
pub(crate) fn glued(text: &str, at: usize) -> bool {
    text[..at]
        .chars()
        .next_back()
        .is_some_and(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

/// The token starting at `at`, or `None` when no ID core starts there.
///
/// `previous` is where the token before it ended: a `/` right after another token is a slash
/// pair (`D4/D10`), not a path.
fn token_at(text: &str, at: usize, previous: Option<usize>) -> Option<Token> {
    let (family, number, core_end) = core_at(text, at, true)?;
    let after_core = text[core_end..].chars().next();
    if after_core.is_some_and(|c| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }
    let mut end = core_end;
    let mut words: Vec<&str> = Vec::new();
    while let Some(word) = slug_word(text, end) {
        words.push(word);
        end += 1 + word.len();
    }
    let slug = (!words.is_empty()).then(|| words.join("-"));
    let rest = &text[end..];
    let next = rest.chars().next();

    // A `/` right after another ID core — a token's, or one glued into an alias form (`AD-D1/D2`)
    // — separates a pair (S4, L4 as ruled at review), never starts a path.
    let before_slash =
        text[..at].ends_with('/') && previous != Some(at - 1) && !core_ends_at(text, at - 1);
    // A `/` before another ID core is a slash pair's separator (`D4/D10`), not a path.
    let slash_pair = next == Some('/') && core_at(text, end + 1, false).is_some();
    let after_path = (next == Some('/') && !slash_pair)
        || (next == Some('.') && rest[1..].starts_with(|c: char| c.is_ascii_alphabetic()));
    let path = before_slash || after_path;
    let lower = !text[at..].starts_with(family.prefix);
    if lower && !path {
        return None;
    }

    // Every slug word has been read, so a `-` still standing starts no word that begins with a
    // letter (D19): an upper case after it is a hyphen range (`D1-D7`); anything else — a digit
    // (`C3-gate-2`), a placeholder (`D1-<slug>`) — makes the token a label, never a mention.
    let upper_after_hyphen =
        rest.starts_with('-') && rest[1..].starts_with(|c: char| c.is_ascii_uppercase());
    let hyphen_range = words.is_empty() && upper_after_hyphen;
    let range = RANGE_JOINERS
        .iter()
        .any(|joiner| rest.starts_with(joiner) || text[..at].ends_with(joiner))
        || hyphen_range;
    let label = (rest.starts_with('-') && !hyphen_range)
        || next.is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
        || crate::home::is_run_id(&text[at..end])
        || c4_term(text, family, &number, core_end);
    let skip = if label {
        Some(Skip::Label)
    } else if range {
        Some(Skip::Range)
    } else if path && slug.is_none() {
        Some(Skip::Path)
    } else {
        None
    };
    Some(Token {
        start: at,
        end,
        family,
        number,
        slug,
        path,
        qualifier: qualifier_before(text, at),
        skip,
    })
}

/// The slug word after the `-` at `at`, when one starts there: `[a-z][a-z0-9]*` (D19).
pub(crate) fn slug_word(text: &str, at: usize) -> Option<&str> {
    let rest = text[at..].strip_prefix('-')?;
    if !rest.starts_with(|c: char| c.is_ascii_lowercase()) {
        return None;
    }
    let len = rest
        .bytes()
        .take_while(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        .count();
    Some(&rest[..len])
}

/// Whether a cycle token `C4` is the C4 model's level name: `C4` then `-` or one space, an
/// optional `**`, and one of [`C4_TERMS`] ending at a word boundary.
fn c4_term(text: &str, family: &Family, number: &str, core_end: usize) -> bool {
    if family.name != "cycle C" || number != "4" {
        return false;
    }
    let rest = &text[core_end..];
    let Some(rest) = rest.strip_prefix('-').or_else(|| rest.strip_prefix(' ')) else {
        return false;
    };
    let rest = rest.strip_prefix("**").unwrap_or(rest);
    C4_TERMS.iter().any(|term| {
        rest.strip_prefix(term)
            .is_some_and(|tail| !tail.starts_with(|c: char| c.is_ascii_alphanumeric()))
    })
}

/// The owner name written right in front of the token at `at`, separated by one space or by a
/// soft line break (C3): a code span, or a plain kebab name of two or more parts
/// (`adaptive-depth`). A single plain word (`record D4`) is not a name.
fn qualifier_before(text: &str, at: usize) -> Option<Qualifier> {
    text[..at]
        .strip_suffix(' ')
        .and_then(qualifier_ending)
        .or_else(|| soft_break_before(text, at).and_then(qualifier_ending))
}

/// The text before a soft line break that ends at `at` — `\n` and indentation inside one
/// paragraph, whose line before is neither blank nor a heading (C3) — so a hard-wrapped qualifier
/// still stands in front of its token.
fn soft_break_before(text: &str, at: usize) -> Option<&str> {
    let before = text[..at]
        .trim_end_matches([' ', '\t'])
        .strip_suffix('\n')?;
    let line = before[before.rfind('\n').map_or(0, |i| i + 1)..].trim();
    (!line.is_empty() && !line.starts_with('#')).then_some(before)
}

/// The qualifier `before` ends with, if any.
fn qualifier_ending(before: &str) -> Option<Qualifier> {
    if let Some(inner) = before.strip_suffix('`') {
        let open = inner.rfind(['`', '\n'])?;
        if !inner[open..].starts_with('`') {
            return None;
        }
        let content = &inner[open + 1..];
        let clean = !content.is_empty() && !content.contains(char::is_whitespace);
        return clean.then(|| Qualifier {
            text: content.to_string(),
            code: true,
        });
    }
    let start = before
        .rfind(|c: char| !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'))
        .map_or(0, |i| i + char_len(before, i));
    let name = &before[start..];
    let kebab = name.contains('-') && name.split('-').all(|part| !part.is_empty());
    let bounded = !before[..start]
        .ends_with(|c: char| c.is_ascii_alphanumeric() || c == '_' || c == '/' || c == '.');
    (kebab && bounded).then(|| Qualifier {
        text: name.to_string(),
        code: false,
    })
}

/// Whether `text[from..to]`, the text between two members, joins them into one compound: one of
/// [`SEPARATORS`] alone, on one line or across one soft line break (D5, C3).
pub(crate) fn joins(text: &str, from: usize, to: usize) -> bool {
    let between = &text[from..to];
    between.matches('\n').count() <= 1 && SEPARATORS.contains(&between.trim())
}

/// The runs of `spans` (in text order) whose neighbours are separated only by one of
/// [`SEPARATORS`], on one line or across one soft line break (D5, C3): each run as a range of
/// indexes into `spans`. The text between two members runs up to the second one's own qualifier,
/// so two differently qualified IDs never join.
pub(crate) fn joined_runs(text: &str, spans: &[(usize, usize)]) -> Vec<std::ops::Range<usize>> {
    let mut runs = Vec::new();
    let mut start = 0;
    while start < spans.len() {
        let mut end = start + 1;
        while end < spans.len() && joins(text, spans[end - 1].1, spans[end].0) {
            end += 1;
        }
        runs.push(start..end);
        start = end;
    }
    runs
}

/// Mark the members of every list of four or more `Skip::List`, and let the first member's
/// qualifier cover the rest of a shorter compound (D5, S4); a compound is one of [`joined_runs`].
fn group_lists(text: &str, tokens: &mut [Token]) {
    let spans: Vec<(usize, usize)> = tokens.iter().map(|t| (t.start, t.end)).collect();
    for run in joined_runs(text, &spans) {
        let group = &mut tokens[run];
        if group.len() > 3 {
            for token in group.iter_mut().filter(|t| t.skip.is_none()) {
                token.skip = Some(Skip::List);
            }
        } else if let Some(first) = group.first().and_then(|t| t.qualifier.clone()) {
            for token in group.iter_mut().skip(1) {
                if token.qualifier.is_none() {
                    token.qualifier = Some(first.clone());
                }
            }
        }
    }
}

/// Which bytes of `text` are verbatim or machine-read, and so never scanned or rewritten.
///
/// - **Verbatim quotes** keep their source's form (D15 as changed at review, S1): a fenced block,
///   a `>` blockquote line, and a `"…"` span. A quote pairs across line breaks inside one paragraph
///   (V5); an unterminated one runs to its paragraph's end; `\"` is not a delimiter. Only the
///   straight quote is a delimiter.
/// - **Machine-read spots** keep the bare number (D12): an `anchor:` line and a `<!-- GI-… -->`
///   marker.
pub fn masked(text: &str) -> Vec<bool> {
    masked_as(text, true)
}

/// [`masked`], with the `"…"` quote mask on or off: it is off in HTML only, so an HTML
/// prototype's `href="…"` stays a link (N4 as ruled at the second review).
pub fn masked_as(text: &str, quotes: bool) -> Vec<bool> {
    let mut mask = vec![false; text.len()];
    let fenced = crate::conform::fenced_lines(text);
    let mut lines: Vec<(usize, usize, bool)> = Vec::new();
    let mut offset = 0;
    for (index, line) in text.split_inclusive('\n').enumerate() {
        let trimmed = line.trim_start();
        let item = trimmed.strip_prefix("- ").unwrap_or(trimmed);
        let whole = fenced.get(index).copied().unwrap_or(false)
            || trimmed.starts_with('>')
            || item.starts_with("anchor:");
        if whole {
            mask[offset..offset + line.len()].fill(true);
        }
        lines.push((offset, offset + line.len(), whole));
        offset += line.len();
    }

    let mut from = 0;
    while let Some(found) = text[from..].find("<!-- GI-") {
        let open = from + found;
        let close = text[open..]
            .find("-->")
            .map_or(text.len(), |i| open + i + 3);
        mask[open..close].fill(true);
        from = close;
    }

    // Quotes pair within a paragraph: a run of lines neither blank nor masked whole.
    let mut paragraph: Option<(usize, usize)> = None;
    for &(start, end, whole) in lines.iter().filter(|_| quotes) {
        let blank = text[start..end].trim().is_empty();
        if blank || whole {
            if let Some((from, to)) = paragraph.take() {
                mask_quotes(text, from, to, &mut mask);
            }
            continue;
        }
        paragraph = Some(paragraph.map_or((start, end), |(from, _)| (from, end)));
    }
    if let Some((from, to)) = paragraph {
        mask_quotes(text, from, to, &mut mask);
    }
    mask
}

/// Mask each `"…"` span in `text[from..to]`, pairing quotes in order.
fn mask_quotes(text: &str, from: usize, to: usize, mask: &mut [bool]) {
    let mut open: Option<usize> = None;
    for (offset, c) in text[from..to].char_indices() {
        let at = from + offset;
        if c != '"' || mask[at] || text[..at].ends_with('\\') {
            continue;
        }
        match open.take() {
            Some(start) => mask[start..=at].fill(true),
            None => open = Some(at),
        }
    }
    if let Some(start) = open {
        mask[start..to].fill(true);
    }
}

// ---------------------------------------------------------------------------
// the tree: walk, definitions, owners, resolution
// ---------------------------------------------------------------------------

/// Whose sequence a per-artifact ID belongs to.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Owner {
    /// A directory name: a spec's (`lunch-orders`), a session's, or `product`.
    Name(String),
    /// A FEAT or EPIC ID, keyed on its number (R4): the owner of a `tasks.md`.
    Id {
        prefix: &'static str,
        number: String,
    },
}

/// Where an ID is defined.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Definition {
    /// The tree-relative file holding the definition.
    pub path: PathBuf,
    /// The byte the definition's token starts at, or `None` when the file name defines it.
    pub start: Option<usize>,
    /// The definition's slug, or `None` for a bare definition.
    pub slug: Option<String>,
}

/// One place an ID is defined (wave 1b): a definition line, or a file whose own name carries the
/// ID and a slug. A number's sites make its holders (`rename::holders`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Site {
    /// The tree-relative file.
    pub(crate) path: PathBuf,
    /// The byte the site's token starts at, or `None` for a file-name site.
    pub(crate) start: Option<usize>,
    /// The site's slug, as the definition reads it (H4's fill included); `None` when bare.
    pub(crate) slug: Option<String>,
    /// The structural marks the line was accepted under, or `None` for a file-name site.
    pub(crate) kind: Option<Leaders>,
}

/// What a token resolved to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Resolved {
    /// A project-wide family's ID: its number alone is the key.
    Project,
    /// A per-artifact ID with its owner.
    Owned(Owner),
    /// A per-artifact ID whose owner the text does not name: never rewritten.
    Unresolved,
}

/// Every definition in a tree, the files whose own name carries an ID and a slug, and the
/// sessions it holds.
#[derive(Debug, Default)]
pub struct Index {
    definitions: BTreeMap<(&'static str, Option<Owner>, String), Definition>,
    /// Every site read for a key, in reading order; `definitions` keeps the first of them.
    sites: BTreeMap<(&'static str, Option<Owner>, String), Vec<Site>>,
    named: BTreeMap<(&'static str, String), Vec<(PathBuf, String)>>,
    sessions: BTreeSet<String>,
}

impl Index {
    /// Read every definition in `files` (tree-relative) under `root`, every file name among them
    /// that carries an ID and a slug (D12, S5), and the session directories under
    /// `<root>/.mochiko/brainstorms/` (R1: a yes/no fact about the layout).
    ///
    /// A definition is the first line in one of its family's definition files whose first token,
    /// after the chained leaders `strip_leaders` removes, is the ID (R3); a file whose own name
    /// carries the ID and a slug defines it by that name (D12, S5), ahead of any line.
    pub fn build(root: &Path, files: &[PathBuf]) -> Index {
        let mut index = Index {
            sessions: sessions(root),
            ..Index::default()
        };
        for rel in files {
            let Some(name) = rel.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            for token in scan(name).into_iter().filter(|t| t.start == 0) {
                if let Some(slug) = token.slug {
                    let files = index.named.entry((token.family.name, token.number));
                    files.or_default().push((rel.clone(), slug));
                }
            }
        }
        for rel in files {
            let families: Vec<&'static Family> = FAMILIES
                .iter()
                .filter(|family| is_definition_file(family, rel))
                .collect();
            if families.is_empty() {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(root.join(rel)) else {
                continue;
            };
            for family in families {
                index.read_definitions(family, rel, &text);
            }
        }
        index
    }

    fn read_definitions(&mut self, family: &'static Family, rel: &Path, text: &str) {
        let owner = match family.scope {
            Scope::Project => None,
            Scope::Artifact => owner_of(rel),
        };
        if family.scope == Scope::Artifact && owner.is_none() {
            return;
        }
        if let Some(name) = rel.file_name().and_then(|n| n.to_str()) {
            let named = scan(name)
                .into_iter()
                .find(|t| t.start == 0 && t.family == family && t.slug.is_some());
            if let Some(token) = named {
                let site = Site {
                    path: rel.to_path_buf(),
                    start: None,
                    slug: token.slug,
                    kind: None,
                };
                self.insert(family, owner.clone(), token.number, site);
            }
        }
        let tokens = scan_as(text, quotes_masked(rel));
        let mask = masked_as(text, quotes_masked(rel));
        let mut offset = 0;
        let mut lines: Vec<(usize, Leaders)> = Vec::new();
        for line in text.split_inclusive('\n') {
            let (width, leaders) = strip_leaders(line);
            lines.push((offset + width, leaders));
            offset += line.len();
        }
        // L2: a session decision's heading card beats an earlier bold line, so headings are read
        // first; the first one read still wins.
        if family.name == "session D" {
            lines.sort_by_key(|(_, leaders)| !leaders.heading);
        }
        for (lead, leaders) in lines {
            if !leaders.define(family) || mask.get(lead).copied().unwrap_or(true) {
                continue;
            }
            let found = if family.name == "cycle C" {
                cycle_heading(&text[lead..]).map(|number| (number, None))
            } else {
                None
            };
            let found = found.or_else(|| {
                tokens
                    .iter()
                    .find(|t| {
                        t.start == lead
                            && t.family == family
                            && t.skip.is_none()
                            && self.slug_fits(t, rel)
                    })
                    .map(|t| (t.number.clone(), t.slug.clone()))
            });
            let Some((number, slug)) = found else {
                continue;
            };
            // H4: a bare per-artifact line takes the slug the files under its owner carry, when
            // they agree on one (S5) — a screen's `prototype/scr-NNN-<slug>.html`.
            let slug = slug.or_else(|| {
                let held = self.file_slugs(family, &number, rel);
                (family.scope == Scope::Artifact && held.len() == 1)
                    .then(|| held.into_iter().next())
                    .flatten()
            });
            let site = Site {
                path: rel.to_path_buf(),
                start: Some(lead),
                slug,
                kind: Some(leaders),
            };
            self.insert(family, owner.clone(), number, site);
        }
    }

    /// Record a site, and the definition unless one is already held: the first one read wins.
    fn insert(
        &mut self,
        family: &'static Family,
        owner: Option<Owner>,
        number: String,
        site: Site,
    ) {
        let key = (family.name, owner, number);
        self.definitions
            .entry(key.clone())
            .or_insert_with(|| Definition {
                path: site.path.clone(),
                start: site.start,
                slug: site.slug.clone(),
            });
        self.sites.entry(key).or_default().push(site);
    }

    /// Every site of `number` in `family`, owned by `owner`, in reading order (wave 1b).
    pub(crate) fn sites(&self, family: &Family, owner: Option<&Owner>, number: &str) -> &[Site] {
        self.sites
            .get(&(family.name, owner.cloned(), number.to_string()))
            .map_or(&[], Vec::as_slice)
    }

    /// Whether any definition of `family` is owned by `owner`.
    pub(crate) fn holds_owner(&self, family: &Family, owner: &Owner) -> bool {
        self.definitions
            .keys()
            .any(|(name, held, _)| *name == family.name && held.as_ref() == Some(owner))
    }

    /// Whether a token's hyphen run is its slug, read against the file defining its ID: none, three
    /// words (D19), or one of [`Index::file_slugs`] (S5). Any other run makes the whole token a
    /// local label, never reported and never rewritten (N2 as ruled at build).
    pub fn slug_fits(&self, token: &Token, definition: &Path) -> bool {
        token.slug.as_ref().is_none_or(|slug| {
            slug.split('-').count() == 3
                || self
                    .file_slugs(token.family, &token.number, definition)
                    .contains(slug)
        })
    }

    /// The slugs the names of files carrying `family`'s `number` give it (D12, S5), read against
    /// the file defining it: a project-wide family's own definition files (FEAT entries, AX
    /// concern files), or any file under a per-artifact family's owner directory (`prototype/`
    /// screens among them, H4) — ruling (a) as narrowed at review.
    pub fn file_slugs(&self, family: &Family, number: &str, definition: &Path) -> BTreeSet<String> {
        let held = |path: &Path| match family.scope {
            Scope::Project => is_definition_file(family, path),
            Scope::Artifact => owner_dir(definition).is_some_and(|dir| path.starts_with(dir)),
        };
        self.named
            .get(&(family.name, number.to_string()))
            .into_iter()
            .flatten()
            .filter(|(path, _)| held(path))
            .map(|(_, slug)| slug.clone())
            .collect()
    }

    /// Whether `name` is a session directory under `.mochiko/brainstorms/`.
    pub fn is_session(&self, name: &str) -> bool {
        self.sessions.contains(name)
    }

    /// The definition of `number` in `family`, owned by `owner` (`None` for a project-wide one).
    pub fn definition(
        &self,
        family: &Family,
        owner: Option<&Owner>,
        number: &str,
    ) -> Option<&Definition> {
        self.definitions
            .get(&(family.name, owner.cloned(), number.to_string()))
    }
}

/// The paths every `ids` command leaves alone, tree-relative: the history layer (D14, D21), the
/// run folder and the worktrees (R2), and the local settings file, which holds a live token (S8).
/// Eval stimuli are matched by segment instead (`EVAL_STIMULI`), and `.env` and `.env.*` files by
/// name at any depth (S8's class, ruling 5 at review); released `CHANGELOG.md` entries cannot be
/// told from the unreleased one, so the whole file is left alone (S6). The template goldens are
/// renders of the log, compared byte for byte (D14 as changed at build).
pub const DEFAULT_EXCLUDES: [&str; 9] = [
    ".mochiko/archive/",
    ".mochiko/schema-views/",
    ".mochiko/runs/",
    "plugins/mochiko/migrations/",
    "crates/mochiko-cli/tests/fixtures/genesis-corpus/",
    "crates/mochiko-cli/tests/fixtures/template/",
    "CHANGELOG.md",
    ".claude/worktrees/",
    ".claude/settings.local.json",
];

/// The directory names no walk enters.
const SKIPPED_DIRS: [&str; 3] = [".git", "target", "node_modules"];

/// The eval stimulus directories under `evals/` (D14): fixture inputs and recorded runs.
const EVAL_STIMULI: [&str; 3] = ["fixtures", "fixture", "runs"];

/// The files under `root` an `ids` command reads, tree-relative and sorted.
///
/// `under` narrows the walk to those tree-relative paths (files or directories); empty walks the
/// whole tree. A directory below the root that holds its own `.git` is another tree and is never
/// entered (R2); [`DEFAULT_EXCLUDES`], the eval stimuli and `excludes` (tree-relative prefixes)
/// are skipped; a file that is not UTF-8 text is skipped when it is read.
pub fn walk(root: &Path, under: &[PathBuf], excludes: &[String]) -> Vec<PathBuf> {
    let starts: Vec<PathBuf> = if under.is_empty() {
        vec![PathBuf::new()]
    } else {
        under.to_vec()
    };
    let mut out = Vec::new();
    for start in starts {
        visit(root, &start, excludes, &mut out);
    }
    out.sort();
    out.dedup();
    out
}

fn visit(root: &Path, rel: &Path, excludes: &[String], out: &mut Vec<PathBuf>) {
    if excluded(rel, excludes) {
        return;
    }
    let path = root.join(rel);
    // L3: a symlink is never followed — a cycle never ends, and its target may be a secret the
    // tree only points at.
    let linked = path
        .symlink_metadata()
        .is_ok_and(|meta| meta.file_type().is_symlink());
    if linked && !rel.as_os_str().is_empty() {
        return;
    }
    if path.is_file() {
        out.push(rel.to_path_buf());
        return;
    }
    if !path.is_dir() || (!rel.as_os_str().is_empty() && path.join(".git").exists()) {
        return;
    }
    let Ok(entries) = std::fs::read_dir(&path) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name();
        if SKIPPED_DIRS.iter().any(|skip| name == *skip) && entry.path().is_dir() {
            continue;
        }
        visit(root, &rel.join(name), excludes, out);
    }
}

/// Whether a tree-relative path is excluded by default or by `excludes`.
fn excluded(rel: &Path, excludes: &[String]) -> bool {
    let text = rel.to_string_lossy().replace('\\', "/");
    if text.is_empty() {
        return false;
    }
    let segments: Vec<&str> = text.split('/').collect();
    let stimulus = segments.len() > 2
        && segments[0] == "evals"
        && segments[1..segments.len() - 1]
            .iter()
            .any(|segment| EVAL_STIMULI.contains(segment));
    let name = segments.last().copied().unwrap_or_default();
    let secret = name == ".env" || name.starts_with(".env.");
    stimulus
        || secret
        || DEFAULT_EXCLUDES
            .iter()
            .map(|p| p.to_string())
            .chain(excludes.iter().cloned())
            .any(|prefix| {
                let prefix = prefix.trim_start_matches("./").trim_end_matches('/');
                !prefix.is_empty() && (text == prefix || text.starts_with(&format!("{prefix}/")))
            })
}

/// The session directory names under `<root>/.mochiko/brainstorms/`.
fn sessions(root: &Path) -> BTreeSet<String> {
    let Ok(entries) = std::fs::read_dir(root.join(".mochiko/brainstorms")) else {
        return BTreeSet::new();
    };
    entries
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| entry.file_name().to_str().map(str::to_string))
        .collect()
}

/// Whether tree-relative `rel` is one of `family`'s definition files.
fn is_definition_file(family: &Family, rel: &Path) -> bool {
    let text = rel.to_string_lossy();
    let parts: Vec<&str> = text.split('/').collect();
    family.definitions.iter().any(|pattern| {
        let pattern: Vec<&str> = pattern.split('/').collect();
        pattern.len() == parts.len()
            && pattern
                .iter()
                .zip(&parts)
                .all(|(pattern, part)| crate::home::file_matches(pattern, part))
    })
}

/// The owner of a per-artifact definition file: its directory's name — the spec's for a file in
/// its `stories/` sub-home, a lane home's `lane-<slug>` — or, where that directory is a FEAT or
/// EPIC ID, that ID (R4).
pub fn owner_of(rel: &Path) -> Option<Owner> {
    let dir = owner_dir(rel)?;
    let name = dir.file_name()?.to_str()?;
    Some(owner_named(name))
}

/// Whether the `"…"` quote mask applies to the tree-relative file `rel`: everywhere but HTML (N4
/// as ruled, option ii). An HTML attribute is a link; a quoted string in markdown is a verbatim
/// quote, and in code an expectation or a fixture, often the bare log verbatim.
pub fn quotes_masked(rel: &Path) -> bool {
    !rel.extension()
        .is_some_and(|extension| extension == "html" || extension == "htm")
}

/// The directory a per-artifact owner's files live under: the defining file's own, or its spec's
/// for a story file.
pub(crate) fn owner_dir(definition: &Path) -> Option<PathBuf> {
    let dir = definition.parent()?;
    if dir.file_name().is_some_and(|name| name == "stories") {
        return dir.parent().map(Path::to_path_buf);
    }
    Some(dir.to_path_buf())
}

/// An owner name as written: a FEAT or EPIC ID (bare or joined) keyed on its number, or a name.
fn owner_named(text: &str) -> Owner {
    if let Some((family, number, end)) = core_at(text, 0, false) {
        let tail = &text[end..];
        let slugged = tail.is_empty() || tail.strip_prefix('-').is_some_and(crate::model::is_slug);
        if (family.name == "FEAT" || family.name == "EPIC") && slugged {
            return Owner::Id {
                prefix: family.prefix,
                number,
            };
        }
    }
    Owner::Name(text.to_string())
}

/// The structural marks a line opens with (R3, H2 as ruled at review); a definition site's kind
/// (wave 1b).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Leaders {
    heading: bool,
    bold: bool,
    cell: bool,
    checkbox: bool,
}

impl Leaders {
    /// Whether the line is laid out the way `family`'s minting rule writes a definition: a
    /// heading or `**` for a session decision's card; for any other family also a first table
    /// cell or a checkbox. A list mark or indentation alone is prose, so a wrapped line that
    /// happens to start with an ID defines nothing.
    fn define(self, family: &Family) -> bool {
        self.heading || self.bold || (family.name != "session D" && (self.cell || self.checkbox))
    }
}

/// The byte length of a line's leaders and which structural marks they hold (R3): indentation,
/// `#` heading marks, a `- ` or `* ` list mark, a `[ ]`/`[x]` checkbox, a `**Targets**:` label
/// (R8), `**`, and a leading `|` table pipe, in any order.
fn strip_leaders(line: &str) -> (usize, Leaders) {
    let mut rest = line;
    let mut leaders = Leaders::default();
    loop {
        let before = rest.len();
        rest = rest.trim_start_matches([' ', '\t']);
        if rest.starts_with('#') {
            let marks = rest.trim_start_matches('#');
            if let Some(after) = marks.strip_prefix(' ') {
                rest = after;
                leaders.heading = true;
            }
        }
        for leader in [
            "- ",
            "* ",
            "[ ] ",
            "[x] ",
            "[X] ",
            "**Targets**:",
            "**",
            "|",
        ] {
            if let Some(after) = rest.strip_prefix(leader) {
                rest = after;
                match leader {
                    "- " | "* " => {}
                    "|" => leaders.cell = true,
                    "**Targets**:" | "**" => leaders.bold = true,
                    _ => leaders.checkbox = true,
                }
                break;
            }
        }
        if rest.len() == before {
            return (line.len() - rest.len(), leaders);
        }
    }
}

/// The number of a `Cycle <n>` heading's cycle, as today's `tasks` template writes it (H2).
fn cycle_heading(text: &str) -> Option<String> {
    let rest = text.strip_prefix("Cycle ")?;
    let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
    let bounded = !rest[digits.len()..].starts_with(|c: char| c.is_ascii_alphanumeric());
    (!digits.is_empty() && bounded).then_some(digits)
}

/// Every token in `text` with what it resolves to, for the file at tree-relative `rel`.
///
/// A per-artifact ID resolves, first match winning (plan item 4):
/// 1. through its qualifier — its own, or its compound's first member's (S4). A session `D`
///    qualifies only by an existing session directory (R1), named by its slug or, in a code span,
///    by its record's path (wave 1c, [`record_session`]); a cycle by a FEAT or EPIC ID (R4, R9);
///    any other family, and a cycle under another name (a lane's `lane-<slug>`), only by an owner
///    the index holds;
/// 2. to the citing file's own owner, when that file is one of the family's definition files;
/// 3. for session `D` only, to the one session its line names, when a record link names it (B1 as
///    narrowed at build);
/// 4. otherwise to nothing.
pub fn resolve_file(index: &Index, rel: &Path, text: &str) -> Vec<(Token, Resolved)> {
    let tokens = scan_as(text, quotes_masked(rel));
    let mut out = Vec::with_capacity(tokens.len());
    for token in &tokens {
        let resolved = resolve(index, rel, text, &tokens, token);
        out.push((token.clone(), resolved));
    }
    out
}

fn resolve(index: &Index, rel: &Path, text: &str, tokens: &[Token], token: &Token) -> Resolved {
    let family = token.family;
    if family.scope == Scope::Project {
        return Resolved::Project;
    }
    if let Some(qualifier) = &token.qualifier {
        let owner = match record_session(qualifier) {
            Some(session) if family.name == "session D" => Owner::Name(session.to_string()),
            _ => owner_named(&qualifier.text),
        };
        let qualifies = match (family.name, &owner) {
            ("session D", Owner::Name(name)) => index.is_session(name),
            ("session D", Owner::Id { .. }) => false,
            ("cycle C", Owner::Id { .. }) => true,
            (_, owner) => index.holds_owner(family, owner),
        };
        return if qualifies {
            Resolved::Owned(owner)
        } else {
            Resolved::Unresolved
        };
    }
    if is_definition_file(family, rel) {
        if let Some(owner) = owner_of(rel) {
            return Resolved::Owned(owner);
        }
    }
    if family.name == "session D" {
        // B1 as narrowed at build (user ruling C): only a record link makes a session the owner
        // of a line's bare decisions, and only when the line names no other session.
        let (named, linked) = line_sessions(text, tokens, token.start);
        if let [only] = named.iter().collect::<Vec<_>>()[..] {
            if linked.contains(only) && index.is_session(only) {
                return Resolved::Owned(Owner::Name(only.clone()));
            }
        }
    }
    Resolved::Unresolved
}

/// The session a code-span qualifier names when it is that session's record path,
/// `` `.mochiko/brainstorms/<slug>/record.md` `` (D11 as changed at build, wave 1c). Only the
/// tree-root path counts; a bare or relative `record.md` names no session (Q2 as ruled).
fn record_session(qualifier: &Qualifier) -> Option<&str> {
    let slug = qualifier
        .text
        .strip_prefix(".mochiko/brainstorms/")?
        .strip_suffix("/record.md")?;
    (qualifier.code && !slug.is_empty() && !slug.contains('/')).then_some(slug)
}

/// The sessions the line around byte `at` names (B1 as narrowed at build, R1): every record link
/// (`…/<slug>/record.md`) and every name written in front of a session `D` token on it, leading or
/// mid-line, resolvable or not; and apart, the ones a record link names.
fn line_sessions(text: &str, tokens: &[Token], at: usize) -> (BTreeSet<String>, BTreeSet<String>) {
    let start = text[..at].rfind('\n').map_or(0, |i| i + 1);
    let end = text[at..].find('\n').map_or(text.len(), |i| at + i);
    let line = &text[start..end];
    let mut linked = BTreeSet::new();
    let mut from = 0;
    while let Some(found) = line[from..].find("/record.md") {
        let link = &line[..from + found];
        let slug = link
            .rsplit(['/', '(', '[', ' ', '`'])
            .next()
            .unwrap_or_default();
        if !slug.is_empty() {
            linked.insert(slug.to_string());
        }
        from += found + "/record.md".len();
    }
    let mut named = linked.clone();
    for token in tokens {
        let on_line = token.start >= start && token.start < end;
        if on_line && token.family.name == "session D" {
            if let Some(qualifier) = &token.qualifier {
                named.insert(qualifier.text.clone());
            }
        }
    }
    (named, linked)
}

// ---------------------------------------------------------------------------
// `ids --check`
// ---------------------------------------------------------------------------

/// What a finding reports (D6 as changed at review, D16, D18).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A bare definition, or a bare mention tied to a joined definition.
    Bare,
    /// A joined mention whose slug differs from its definition's.
    Drift,
}

/// One advisory finding.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    /// Tree-relative.
    pub path: PathBuf,
    /// One-based.
    pub line: usize,
    /// One-based, in characters.
    pub col: usize,
    pub kind: Kind,
    /// The token as written.
    pub token: String,
    /// For drift, the definition's slug (`None` for a bare definition).
    pub definition: Option<Option<String>>,
}

/// Check the files under `paths` (tree-relative; empty checks the whole tree) against the
/// definitions of the whole tree. Advisory: the caller reports through its exit code only (D6).
/// Only a mention tied to a definition is reported (D18 as changed at build).
pub fn check(root: &Path, paths: &[PathBuf], excludes: &[String]) -> Vec<Finding> {
    let index = Index::build(root, &walk(root, &[], excludes));
    let mut out = Vec::new();
    for rel in walk(root, paths, excludes) {
        let Ok(text) = std::fs::read_to_string(root.join(&rel)) else {
            continue;
        };
        for (token, resolved) in resolve_file(&index, &rel, &text) {
            if token.skip.is_some() {
                continue;
            }
            if let Some((kind, definition)) = judge(&index, &rel, &token, &resolved) {
                let (line, col) = line_col(&text, token.start);
                out.push(Finding {
                    path: rel.clone(),
                    line,
                    col,
                    kind,
                    token: text[token.start..token.end].to_string(),
                    definition,
                });
            }
        }
    }
    out
}

/// The finding one resolved token earns, if any.
fn judge(
    index: &Index,
    rel: &Path,
    token: &Token,
    resolved: &Resolved,
) -> Option<(Kind, Option<Option<String>>)> {
    // D18 as changed at build: a mention with no resolvable owner, or with no definition in
    // the indexed tree, is never reported (R9's local-label cycle among them).
    let owner = match resolved {
        Resolved::Project => None,
        Resolved::Owned(owner) => Some(owner),
        Resolved::Unresolved => return None,
    };
    let definition = index.definition(token.family, owner, &token.number)?;
    if !index.slug_fits(token, &definition.path) {
        return None;
    }
    if definition.path == rel && definition.start == Some(token.start) {
        return definition.slug.is_none().then_some((Kind::Bare, None));
    }
    match (&token.slug, &definition.slug) {
        (None, None) => None,
        (None, Some(_)) => Some((Kind::Bare, None)),
        (Some(slug), held) if held.as_deref() == Some(slug.as_str()) => None,
        (Some(_), held) => Some((Kind::Drift, Some(held.clone()))),
    }
}

/// The one-based line and character column of byte `at`.
pub fn line_col(text: &str, at: usize) -> (usize, usize) {
    let before = &text[..at];
    let line = before.matches('\n').count() + 1;
    let start = before.rfind('\n').map_or(0, |i| i + 1);
    (line, before[start..].chars().count() + 1)
}

impl std::fmt::Display for Finding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self.kind {
            Kind::Bare => "bare",
            Kind::Drift => "drift",
        };
        write!(
            f,
            "{}:{}:{} · {kind} · {}",
            self.path.display(),
            self.line,
            self.col,
            self.token
        )?;
        match &self.definition {
            Some(Some(slug)) => write!(f, " · definition {slug}"),
            Some(None) => write!(f, " · definition bare"),
            None => Ok(()),
        }
    }
}
