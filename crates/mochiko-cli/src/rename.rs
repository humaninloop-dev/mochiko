//! `ids rename`, `ids rekey` and `ids literal`: preview-first rewrites behind a diff check.
//!
//! `human-readable-ids` D7-scoped-rename-command admits the rename as a kernel-class write tool on three conditions:
//! it is scoped (keyed on the number inside its owning scope, never the bare number project-wide),
//! it previews by default and writes only when asked, and the seat supplies every slug — the tool
//! coins none and judges no word choice. D15 adds the fourth: before any write, a diff check
//! confirms every changed span is an ID token, and anything else blocks the whole write.
//!
//! The diff check is run against the [`Target`], never against the planned edits, so a bug in
//! planning cannot pass itself. It does share [`crate::ids::scan`] with the planner: a token the
//! scanner misreads is misread on both sides, which is the check's accepted risk (R8).

use crate::ids::{self, Family, Owner, Resolved, Scope, Skip};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// What a rewrite changes: the D15 diff check is run against this, never against the edits.
#[derive(Clone, Debug)]
pub enum Target {
    /// One ID's slug: every in-scope mention becomes `<core>-<slug>`; each alias literal plus the
    /// number becomes `` `<owner>` <core>-<slug> `` (B2, R1).
    Rename {
        family: &'static Family,
        number: String,
        slug: String,
        aliases: Vec<String>,
        owner: Option<String>,
        /// The runs a mention may carry and still be this ID: the definition's slug and its file
        /// names' (M1). Any other run that is not three words is a local label (N2).
        slugs: Vec<String>,
    },
    /// One ID's number: every in-scope mention keeps its form and slug under the new number.
    Rekey {
        family: &'static Family,
        from: String,
        to: String,
        /// As for [`Target::Rename`]; on a shared number, the chosen holder's slugs only (F4).
        slugs: Vec<String>,
        /// Whether another holder shares the number (wave 1b): then only a token carrying one of
        /// `slugs` may change, and only a file whose name carries one may move.
        shared: bool,
    },
    /// A repo-only token (`AM-5`) and its slug, inside the given paths only (R6).
    Literal { token: String, slug: String },
}

/// One file's rewrite.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edit {
    /// Tree-relative.
    pub path: PathBuf,
    pub old: String,
    pub new: String,
}

/// A planned rewrite: the edits, the file moves, and what was targeted.
#[derive(Clone, Debug)]
pub struct Plan {
    pub edits: Vec<Edit>,
    /// Tree-relative `(from, to)`.
    pub moves: Vec<(PathBuf, PathBuf)>,
    /// The spots that still name a moved file by its old name once the edits are made.
    pub stale: Vec<Stale>,
    /// The mentions a rekey on a shared number cannot tie to one holder: never rewritten, listed,
    /// and no bar to the write (wave 1b, Q1).
    pub untied: Vec<Stale>,
    /// The mentions a rekey on a shared number keeps because they carry another holder's slug:
    /// never rewritten, listed (A2 at the wave 1b review).
    pub kept: Vec<Stale>,
    pub target: Target,
}

/// A spot the rewrite's scope does not reach that names a moved file by its old name (N5 as
/// ruled at build): a link into another owner's file, a quote, a local label.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stale {
    /// Tree-relative.
    pub path: PathBuf,
    /// One-based.
    pub line: usize,
    /// One-based, in characters.
    pub col: usize,
    /// The moved file's old name.
    pub name: String,
}

impl std::fmt::Display for Stale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{} · {}",
            self.path.display(),
            self.line,
            self.col,
            self.name
        )
    }
}

/// A refusal before anything is planned: the CLI's exit 2.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refused(pub String);

/// One replacement inside a file: `[start, end)` becomes `text`.
type Splice = (usize, usize, String);

// ---------------------------------------------------------------------------
// planning
// ---------------------------------------------------------------------------

/// Plan `ids rename <owning-file> <ID> <new-slug>`.
///
/// Refused when `id` is no ID, when an alias is empty or makes a bare ID (H5), when `owning` holds
/// no definition of it (R3 — so an ID with no definition site anywhere, `GAP`, is never renamed),
/// when its bare definition's files disagree on a slug (H4), when `slug` is not exactly three words
/// (D19, R5) and is not one of the ID's file-name slugs (S5), when the ID's number is shared by
/// two entries (A7, wave 1b), or when a move's target exists or two moves share one (C1). A spot left naming a moved file by its old name, and a path token that
/// names no planned move (H3), is listed in `stale` (N5), and a write refuses on it.
pub fn plan_rename(
    root: &Path,
    owning: &Path,
    id: &str,
    slug: &str,
    aliases: &[String],
    excludes: &[String],
) -> Result<Plan, Refused> {
    let parsed = parse(id)?;
    let files = ids::walk(root, &[], excludes);
    let index = ids::Index::build(root, &files);
    guard_aliases(aliases, &parsed, &index, &files)?;
    let owner = owner_for(parsed.family, owning)?;
    let definition = definition_in(&index, &parsed, owner.as_ref(), owning, id)?;
    // A7: one slug for two entries loses one; a landing is resolved by a joined rekey first, and
    // one entry whose slugs drifted is aligned by hand first (F1). The same-kind refusal is a
    // rekey's only (R1): renaming a duplicate gives every copy the slug.
    let held = Holders::of(index.sites(parsed.family, owner.as_ref(), &parsed.number));
    if held.shared() {
        return Err(Refused(format!(
            "`{id}` is held by `{}`: if those are two entries (a landing), resolve it with \
             `ids rekey` and a joined old ID first; if they are one entry whose slugs drifted, \
             give its sites one slug by hand first, then rename",
            held.joined(&parsed).join("`, `")
        )));
    }
    let file_slugs = index.file_slugs(parsed.family, &parsed.number, owning);
    if parsed.family.scope == Scope::Artifact && definition.slug.is_none() && file_slugs.len() > 1 {
        let held: Vec<&str> = file_slugs.iter().map(String::as_str).collect();
        return Err(Refused(format!(
            "the files carrying `{id}` disagree on its slug (`{}`): give them one slug first (H4)",
            held.join("`, `")
        )));
    }
    if !three_words(slug) && !file_slugs.contains(slug) {
        return Err(Refused(format!(
            "`{slug}` is not a slug: three lower-case words joined by `-`, each starting with a \
             letter (D19)"
        )));
    }
    let moves = plan_moves(
        root,
        &files,
        &parsed,
        owner.as_ref(),
        owning,
        |run| slug_run(run, &file_slugs),
        |name, core_end, end| format!("{}-{slug}{}", &name[..core_end], &name[end..]),
    )?;
    let target = Target::Rename {
        family: parsed.family,
        number: parsed.number.clone(),
        slug: slug.to_string(),
        aliases: aliases.to_vec(),
        owner: owner.as_ref().map(owner_text),
        slugs: held_slugs(&definition, file_slugs, Some(slug)),
    };
    let mut unmoved = Vec::new();
    let edits = plan_edits(root, &files, |rel, text| {
        let scope = Reach {
            root,
            index: &index,
            id: &parsed,
            owner: owner.as_ref(),
            owning,
            moves: &moves,
        };
        rename_splices(&scope, rel, text, &target, &mut unmoved)
    });
    let stale = stale_spots(root, &files, &edits, &moves, unmoved);
    Ok(Plan {
        edits,
        moves,
        stale,
        untied: Vec::new(),
        kept: Vec::new(),
        target,
    })
}

/// Plan `ids rekey <owning-file> <old-ID> <new-ID>` (D4 as changed at review, D7 V3).
///
/// `old` is one ID core, or one joined token that picks the holder carrying its slug when two
/// entries share the number (wave 1b, Q5 a); `new` is a bare core, the slug travelling. Refused
/// across families; when no holder of `old` carries the given slug, or `owning` holds none of the
/// moving holder's sites; on a bare `old` whose number is shared; when one file holds two
/// same-kind sites of what the rekey would move (R1: no slug tells those entries apart); on a
/// joined `old` whose shared number nothing proves two entries (F1: they may be one entry whose
/// slugs drifted), or whose holder's file holds a bare site of a kind its sites there lack (B2); when
/// `owning` already defines `new`; when a number-only path in scope carries `old` — a rekey would
/// orphan its directory — and when two moves share a target (C1). On a shared number, only the
/// chosen holder's sites, files and mentions move; a mention tied to no one holder is never
/// rewritten and is listed in `untied` (D18, Q1), and one carrying another holder's slug is listed
/// in `kept` (A2). Stale links are listed as a rename lists them.
pub fn plan_rekey(
    root: &Path,
    owning: &Path,
    old: &str,
    new: &str,
    excludes: &[String],
) -> Result<Plan, Refused> {
    let (from, slug) = parse_old(old)?;
    let to = parse(new)?;
    if from.family != to.family {
        return Err(Refused(format!(
            "`{old}` and `{new}` are in different families"
        )));
    }
    let files = ids::walk(root, &[], excludes);
    let index = ids::Index::build(root, &files);
    let owner = owner_for(from.family, owning)?;
    let held = Holders::of(index.sites(from.family, owner.as_ref(), &from.number));
    let shared = held.shared();
    let no_definition = || {
        Refused(format!(
            "`{}` holds no definition of `{old}`",
            owning.display()
        ))
    };
    let (moving, chosen) = match &slug {
        Some(slug) => {
            let holder = held
                .holders
                .iter()
                .find(|holder| holder.slugs.contains(slug))
                .ok_or_else(no_definition)?;
            (holder.sites.clone(), holder.slugs.clone())
        }
        None if shared => {
            return Err(Refused(format!(
                "`{old}` is held by `{}`: if those are two entries (a landing), give the joined \
                 old ID of the one to move; if they are one entry whose slugs drifted, give its \
                 sites one slug by hand first, then `ids rename`",
                held.joined(&from).join("`, `")
            )))
        }
        None => {
            let mut sites: Vec<&ids::Site> =
                held.holders.iter().flat_map(|h| h.sites.clone()).collect();
            sites.extend(held.bare.iter().copied());
            let slugs = held.holders.iter().flat_map(|h| h.slugs.clone()).collect();
            (sites, slugs)
        }
    };
    // C2: a rekey takes any of the moving holder's sites as its owning file.
    if !moving.iter().any(|site| site.path == owning) {
        return Err(no_definition());
    }
    if let Some((first, second)) = same_kind(&moving) {
        return Err(Refused(format!(
            "`{old}` is defined twice alike in one file, `{}` and `{}`: no slug tells those \
             entries apart, so a rekey cannot move one (R1)",
            site_at(root, first),
            site_at(root, second)
        )));
    }
    // F1: a joined old ID splits a shared number only when its holders are provably two entries.
    if slug.is_some() && shared && !held.provably_two() {
        let mut sites: Vec<String> = Vec::new();
        for site in held.holders.iter().flat_map(|holder| &holder.sites) {
            let at = site_at(root, site);
            if !sites.contains(&at) {
                sites.push(at);
            }
        }
        return Err(Refused(format!(
            "`{}{}` is held by `{}`, and nothing in `{}` proves two entries: these may be one \
             entry whose slugs drifted: give its sites one slug by hand first",
            from.family.prefix,
            from.number,
            held.joined(&from).join("`, `"),
            sites.join("`, `")
        )));
    }
    if let Some(bare) = slug.as_ref().and_then(|_| held.bare_unlike(&moving)) {
        return Err(Refused(format!(
            "`{old}` has a bare definition beside it at `{}`, of a kind its own sites there lack: \
             nothing tells whether that entry moves with it, so give it a slug by hand first (B2)",
            site_at(root, bare)
        )));
    }
    if index
        .definition(to.family, owner.as_ref(), &to.number)
        .is_some()
    {
        return Err(Refused(format!(
            "`{new}` is already defined in `{}`",
            owning.display()
        )));
    }
    for rel in &files {
        let Ok(text) = std::fs::read_to_string(root.join(rel)) else {
            continue;
        };
        let orphaned =
            ids::resolve_file(&index, rel, &text)
                .into_iter()
                .any(|(token, resolved)| {
                    token.skip == Some(Skip::Path)
                        && is_target(&token, &resolved, &from, owner.as_ref())
                });
        if orphaned {
            return Err(Refused(format!(
                "`{}` carries `{old}` in a number-only path, which a rekey would orphan",
                rel.display()
            )));
        }
    }
    // F4: on a shared number the chosen holder's own slugs, never the number's other file slugs.
    let file_slugs = if shared {
        chosen.clone()
    } else {
        let mut slugs = index.file_slugs(from.family, &from.number, owning);
        slugs.extend(chosen.iter().cloned());
        slugs
    };
    let moves = plan_moves(
        root,
        &files,
        &from,
        owner.as_ref(),
        owning,
        |run| slug_run(run, &file_slugs) && (!shared || chosen.contains(run)),
        |name, core_end, end| {
            let prefix_end = core_end - from.number.len();
            format!(
                "{}{}{}",
                &name[..prefix_end],
                to.number,
                &name[core_end..end]
            ) + &name[end..]
        },
    )?;
    let target = Target::Rekey {
        family: from.family,
        from: from.number.clone(),
        to: to.number.clone(),
        slugs: file_slugs.into_iter().collect(),
        shared,
    };
    let mut unmoved = Vec::new();
    let mut untied = Vec::new();
    let mut kept = Vec::new();
    let edits = plan_edits(root, &files, |rel, text| {
        let scope = Reach {
            root,
            index: &index,
            id: &from,
            owner: owner.as_ref(),
            owning,
            moves: &moves,
        };
        let mut splices = Vec::new();
        for token in scope.mentions(rel, text, &[]) {
            if shared {
                match &token.slug {
                    Some(slug) if chosen.contains(slug) => {}
                    Some(slug) if held.holders.iter().any(|h| h.slugs.contains(slug)) => {
                        kept.push(spot(rel, text, &token));
                        continue;
                    }
                    _ => {
                        untied.push(spot(rel, text, &token));
                        continue;
                    }
                }
            }
            let core_end = token.start + from.family.prefix.len() + token.number.len();
            let written = &text[token.start..token.start + from.family.prefix.len()];
            let new = format!("{written}{}{}", to.number, &text[core_end..token.end]);
            if scope.reaches(rel, text, &token, &mut unmoved) {
                splices.push((token.start, token.end, new));
            }
        }
        splices
    });
    let stale = stale_spots(root, &files, &edits, &moves, unmoved);
    Ok(Plan {
        edits,
        moves,
        stale,
        untied,
        kept,
        target,
    })
}

/// Plan `ids literal <TOKEN> <new-slug> <path>…` (R6): a repo-only token that no table family
/// owns (`AM-5`, `J-1`, `OQ-2`, `RGI-7`), rewritten inside the given paths only.
pub fn plan_literal(
    root: &Path,
    token: &str,
    slug: &str,
    paths: &[PathBuf],
    excludes: &[String],
) -> Result<Plan, Refused> {
    if !literal_form(token) {
        return Err(Refused(format!(
            "`{token}` is not a repo-only token: an upper-case letter, letters, `-`, digits"
        )));
    }
    if let Some(id) = ids::parse_id(token) {
        return Err(Refused(format!(
            "`{token}` is an ID of the `{}` family: use `ids rename`",
            id.family.name
        )));
    }
    if !three_words(slug) {
        return Err(Refused(format!(
            "`{slug}` is not a slug: three lower-case words joined by `-`, each starting with a \
             letter (D19)"
        )));
    }
    if paths.is_empty() {
        return Err(Refused(
            "`ids literal` takes at least one path: it never rewrites project-wide".into(),
        ));
    }
    let files = ids::walk(root, paths, excludes);
    let replacement = format!("{token}-{slug}");
    let edits = plan_edits(root, &files, |rel, text| {
        literal_matches(text, token, ids::quotes_masked(rel))
            .into_iter()
            .map(|(start, end)| (start, end, replacement.clone()))
            .collect()
    });
    Ok(Plan {
        edits,
        moves: Vec::new(),
        stale: Vec::new(),
        untied: Vec::new(),
        kept: Vec::new(),
        target: Target::Literal {
            token: token.to_string(),
            slug: slug.to_string(),
        },
    })
}

fn parse(id: &str) -> Result<ids::Id, Refused> {
    ids::parse_id(id).ok_or_else(|| Refused(format!("`{id}` is not an ID of any family")))
}

/// A rekey's old ID: one ID core, or one joined token — its core and its scanned slug (wave 1b).
fn parse_old(old: &str) -> Result<(ids::Id, Option<String>), Refused> {
    if let Some(id) = ids::parse_id(old) {
        return Ok((id, None));
    }
    ids::scan(old)
        .into_iter()
        .find(|t| t.start == 0 && t.end == old.len() && t.slug.is_some() && t.skip.is_none())
        .map(|t| {
            let id = ids::Id {
                family: t.family,
                number: t.number,
            };
            (id, t.slug)
        })
        .ok_or_else(|| Refused(format!("`{old}` is not an ID of any family")))
}

/// Refuse an alias value (`<LITERAL>` or `<LITERAL>=<PATH>`, A3) that is no alias or no scope.
///
/// A literal that is empty, or whose form with the number ends in an ID token that no kebab name
/// qualifies, would rewrite the bare number project-wide, which D7 forbids (H5, N3): `PO-D`
/// (glued, no token) and `producer-plan D` (a kebab name in front) pass; `D`, `(D` and `the D` do
/// not. A shorthand — a kebab name in front — needs a path naming one walked file (Q2, F3), and
/// is refused when its name is an owner the tree holds, whose mentions are already that owner's
/// (F2). A path must have a walked file at or under it, and one literal is either tree-wide or
/// scoped.
fn guard_aliases(
    values: &[String],
    id: &ids::Id,
    index: &ids::Index,
    files: &[PathBuf],
) -> Result<(), Refused> {
    let refused = |value: &str, why: &str| Err(Refused(format!("`--alias {value}`: {why}")));
    for value in values {
        let (alias, path) = alias_value(value);
        let form = format!("{alias}{}", id.number);
        let last = ids::scan(&form).into_iter().find(|t| t.end == form.len());
        let qualifier = last
            .as_ref()
            .and_then(|t| t.qualifier.as_ref())
            .filter(|q| !q.code);
        if alias.trim().is_empty() || (last.is_some() && qualifier.is_none()) {
            return Err(Refused(format!(
                "`--alias {value}` is no alias: it is empty or `{form}` ends in an ID no kebab \
                 name qualifies, which would rewrite the number project-wide (D7)"
            )));
        }
        if let Some(qualifier) = qualifier {
            let held = match id.family.name {
                "session D" => index.is_session(&qualifier.text),
                _ => index.holds_owner(id.family, &Owner::Name(qualifier.text.clone())),
            };
            if held {
                return refused(
                    value,
                    &format!(
                        "`{}` names an owner the tree holds, so its mentions are already that \
                         owner's; an alias would re-own them (F2)",
                        qualifier.text
                    ),
                );
            }
            match path {
                None => {
                    return refused(
                        value,
                        "a shorthand needs the file it means this ID in: \
                         `--alias '<LITERAL>=<FILE>'` (A3, Q2)",
                    )
                }
                Some(path) if !files.iter().any(|file| file == Path::new(scope_path(path))) => {
                    return refused(
                        value,
                        "a shorthand's path must be one walked file, not a directory or a missing \
                         path (F3)",
                    )
                }
                Some(_) => {}
            }
        }
        if let Some(path) = path {
            if !files.iter().any(|file| under(file, path)) {
                return refused(value, &format!("no walked file at or under `{path}`"));
            }
        }
        let scoped = |other: &&String| alias_value(other).0 == alias;
        let forms: Vec<bool> = values
            .iter()
            .filter(scoped)
            .map(|other| alias_value(other).1.is_some())
            .collect();
        if forms.contains(&true) && forms.contains(&false) {
            return refused(value, "one literal is given both tree-wide and scoped");
        }
    }
    Ok(())
}

/// An alias value's literal and its path, split at the first `=` (A3).
fn alias_value(value: &str) -> (&str, Option<&str>) {
    match value.split_once('=') {
        Some((alias, path)) => (alias, Some(path)),
        None => (value, None),
    }
}

/// The distinct literals among `values` that apply in the file `rel`: a tree-wide value in every
/// file, a scoped one in files at or under its path. `None` is a file in no scope: only the
/// tree-wide values apply (A5).
fn aliases_in<'v>(values: &'v [String], rel: Option<&Path>) -> Vec<&'v str> {
    let mut out: Vec<&str> = Vec::new();
    for value in values {
        let (alias, path) = alias_value(value);
        let applies = path.is_none_or(|path| rel.is_some_and(|rel| under(rel, path)));
        if applies && !out.contains(&alias) {
            out.push(alias);
        }
    }
    out
}

/// Whether tree-relative `rel` is `prefix` or sits under it, as an `--exclude` prefix matches.
fn under(rel: &Path, prefix: &str) -> bool {
    let prefix = scope_path(prefix);
    let text = rel.to_string_lossy();
    !prefix.is_empty() && (text == prefix || text.starts_with(&format!("{prefix}/")))
}

/// A scope path as written, with `./` in front and `/` behind trimmed (A1 at the wave 1b review).
fn scope_path(path: &str) -> &str {
    path.trim_start_matches("./").trim_end_matches('/')
}

/// The owner a per-artifact ID's owning file gives it; `None` for a project-wide family.
fn owner_for(family: &Family, owning: &Path) -> Result<Option<Owner>, Refused> {
    match family.scope {
        Scope::Project => Ok(None),
        Scope::Artifact => ids::owner_of(owning).map(Some).ok_or_else(|| {
            Refused(format!(
                "`{}` names no owner for a `{}` ID",
                owning.display(),
                family.name
            ))
        }),
    }
}

/// The ID's definition, which must sit in the owning file (R3).
fn definition_in(
    index: &ids::Index,
    id: &ids::Id,
    owner: Option<&Owner>,
    owning: &Path,
    written: &str,
) -> Result<ids::Definition, Refused> {
    index
        .definition(id.family, owner, &id.number)
        .filter(|definition| definition.path == owning)
        .cloned()
        .ok_or_else(|| {
            Refused(format!(
                "`{}` holds no definition of `{written}`",
                owning.display()
            ))
        })
}

/// The runs a mention may carry and still be the ID (M1): the definition's slug, the file-name
/// slugs, and the new slug a rename writes.
fn held_slugs(
    definition: &ids::Definition,
    file_slugs: BTreeSet<String>,
    new: Option<&str>,
) -> Vec<String> {
    let mut slugs = file_slugs;
    slugs.extend(definition.slug.clone());
    slugs.extend(new.map(str::to_string));
    slugs.into_iter().collect()
}

/// An owner as a qualifier writes it.
fn owner_text(owner: &Owner) -> String {
    match owner {
        Owner::Name(name) => name.clone(),
        Owner::Id { prefix, number } => format!("{prefix}{number}"),
    }
}

/// Whether a resolved token is the ID in its owner's scope.
fn is_target(token: &ids::Token, resolved: &Resolved, id: &ids::Id, owner: Option<&Owner>) -> bool {
    let scoped = match (resolved, owner) {
        (Resolved::Project, None) => true,
        (Resolved::Owned(held), Some(owner)) => held == owner,
        _ => false,
    };
    scoped && token.family == id.family && token.number == id.number
}

/// Whether a file name's run is a slug (N2): three words, or one of the ID's file-name slugs.
fn slug_run(run: &str, file_slugs: &BTreeSet<String>) -> bool {
    run.split('-').count() == 3 || file_slugs.contains(run)
}

/// Three lower-case ASCII words joined by `-`, each starting with a letter (D19, R5).
fn three_words(slug: &str) -> bool {
    let words: Vec<&str> = slug.split('-').collect();
    words.len() == 3
        && words.iter().all(|word| {
            word.starts_with(|c: char| c.is_ascii_lowercase())
                && word
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        })
}

/// `[A-Z][A-Za-z]*-<digits>`: the shape of the repo-only forms (`AM-5`, `RGI-7`, `R-012`).
fn literal_form(token: &str) -> bool {
    let Some((head, digits)) = token.split_once('-') else {
        return false;
    };
    head.starts_with(|c: char| c.is_ascii_uppercase())
        && head.chars().all(|c| c.is_ascii_alphabetic())
        && !digits.is_empty()
        && digits.chars().all(|c| c.is_ascii_digit())
}

/// One entry among a number's definitions (wave 1b §1): its sites carrying one slug, with every
/// line site of a file whose own name defines it, and the slugs among them.
struct Holder<'a> {
    sites: Vec<&'a ids::Site>,
    slugs: BTreeSet<String>,
}

/// A number's holders, and its bare line sites, which are never a holder.
struct Holders<'a> {
    holders: Vec<Holder<'a>>,
    bare: Vec<&'a ids::Site>,
}

impl<'a> Holders<'a> {
    /// Group a number's sites: byte-equal slugs are one holder, and so are an entry file's name
    /// site and its line sites (an entry file holds one entry); a bare line site elsewhere belongs
    /// to no holder.
    fn of(sites: &'a [ids::Site]) -> Holders<'a> {
        let entry_files: BTreeSet<&Path> = sites
            .iter()
            .filter(|site| site.start.is_none())
            .map(|site| site.path.as_path())
            .collect();
        let mut holders: Vec<Holder<'a>> = Vec::new();
        let mut bare = Vec::new();
        for site in sites {
            let entry = entry_files.contains(site.path.as_path());
            if site.slug.is_none() && !entry {
                bare.push(site);
                continue;
            }
            holders.push(Holder {
                sites: vec![site],
                slugs: site.slug.iter().cloned().collect(),
            });
        }
        // Join holders until none shares a slug or an entry file with another.
        let joins = |a: &Holder, b: &Holder| {
            !a.slugs.is_disjoint(&b.slugs)
                || a.sites.iter().any(|x| {
                    entry_files.contains(x.path.as_path())
                        && b.sites.iter().any(|y| y.path == x.path)
                })
        };
        'join: loop {
            for i in 0..holders.len() {
                for j in i + 1..holders.len() {
                    if joins(&holders[i], &holders[j]) {
                        let other = holders.remove(j);
                        holders[i].sites.extend(other.sites);
                        holders[i].slugs.extend(other.slugs);
                        continue 'join;
                    }
                }
            }
            break;
        }
        Holders { holders, bare }
    }

    /// Whether the number is shared (Q4, C1): two or more holders — each a slugged one, so an
    /// all-bare number never is — or one file holding a bare site of a holder's kind there.
    fn shared(&self) -> bool {
        self.holders.len() >= 2 || self.bare_beside()
    }

    /// Whether one file holds a bare site of a holder's kind there (Q4).
    fn bare_beside(&self) -> bool {
        self.bare.iter().any(|bare| {
            self.holders.iter().any(|holder| {
                holder
                    .sites
                    .iter()
                    .any(|site| site.path == bare.path && site.kind == bare.kind)
            })
        })
    }

    /// Whether a shared number provably holds two entries (F1 at the wave 1b review): one file
    /// holds same-kind sites of two holders, two holders each own an entry file, or a bare site
    /// sits beside a holder's of its kind (Q4). Otherwise its holders may be one entry whose
    /// slugs drifted, and the tool never splits them.
    fn provably_two(&self) -> bool {
        let alike = |a: &Holder, b: &Holder| {
            a.sites.iter().any(|x| {
                x.kind.is_some() && b.sites.iter().any(|y| y.path == x.path && y.kind == x.kind)
            })
        };
        let paired = self
            .holders
            .iter()
            .enumerate()
            .any(|(i, a)| self.holders[i + 1..].iter().any(|b| alike(a, b)));
        let entries = self
            .holders
            .iter()
            .filter(|holder| holder.sites.iter().any(|site| site.start.is_none()))
            .count();
        paired || entries >= 2 || self.bare_beside()
    }

    /// A bare site in a file of `chosen`'s whose kind none of `chosen`'s sites there has (B2 as
    /// broadened at the wave 1b review): nothing ties it to the chosen entry or away from it.
    fn bare_unlike(&self, chosen: &[&ids::Site]) -> Option<&'a ids::Site> {
        self.bare.iter().copied().find(|bare| {
            chosen.iter().any(|site| site.path == bare.path)
                && !chosen
                    .iter()
                    .any(|site| site.path == bare.path && site.kind == bare.kind)
        })
    }

    /// Each holder's joined form, and a bare site's own form when the number is shared by one.
    fn joined(&self, id: &ids::Id) -> Vec<String> {
        let core = format!("{}{}", id.family.prefix, id.number);
        let mut out: Vec<String> = self
            .holders
            .iter()
            .filter_map(|holder| holder.slugs.iter().next())
            .map(|slug| format!("{core}-{slug}"))
            .collect();
        if !self.bare.is_empty() && self.shared() {
            out.push(format!("{core} (bare)"));
        }
        out
    }
}

/// Two line sites of the same kind in one file among `sites`, in reading order (R1).
fn same_kind<'a>(sites: &[&'a ids::Site]) -> Option<(&'a ids::Site, &'a ids::Site)> {
    sites.iter().enumerate().find_map(|(i, first)| {
        sites[i + 1..]
            .iter()
            .find(|second| {
                first.kind.is_some() && second.path == first.path && second.kind == first.kind
            })
            .map(|second| (*first, *second))
    })
}

/// A site as `path:line`.
fn site_at(root: &Path, site: &ids::Site) -> String {
    let line = site.start.map_or(1, |start| {
        std::fs::read_to_string(root.join(&site.path))
            .map_or(1, |text| ids::line_col(&text, start).0)
    });
    format!("{}:{line}", site.path.display())
}

/// A mention's spot, as a stale or untied list prints it.
fn spot(rel: &Path, text: &str, token: &ids::Token) -> Stale {
    let (line, col) = ids::line_col(text, token.start);
    Stale {
        path: rel.to_path_buf(),
        line,
        col,
        name: text[token.start..token.end].to_string(),
    }
}

/// What a rename or rekey reaches: the ID in its owner's scope, and the moves it makes.
struct Reach<'a> {
    root: &'a Path,
    index: &'a ids::Index,
    id: &'a ids::Id,
    owner: Option<&'a Owner>,
    owning: &'a Path,
    moves: &'a [(PathBuf, PathBuf)],
}

impl Reach<'_> {
    /// The ID's mentions in one file that are no local label (N2): its in-scope tokens, the tokens
    /// at `members` that an alias form's compound owns (L4), and the unclaimed links that name a
    /// planned move (N4).
    fn mentions(&self, rel: &Path, text: &str, members: &[usize]) -> Vec<ids::Token> {
        ids::resolve_file(self.index, rel, text)
            .into_iter()
            .filter(|(token, resolved)| {
                let id = token.family == self.id.family && token.number == self.id.number;
                // A link no owner claims that resolves to a planned move, and to no other file, is
                // that file's: it follows the move (N4 — an HTML prototype's inter-screen `href`;
                // F1 — by its path, never by its name alone; A1 at round 5).
                let link = id
                    && token.path
                    && *resolved == Resolved::Unresolved
                    && links_move(self.root, text, rel, token, self.moves);
                let ours = is_target(token, resolved, self.id, self.owner)
                    || members.contains(&token.start)
                    || link;
                token.skip.is_none() && ours && self.index.slug_fits(token, self.owning)
            })
            .map(|(token, _)| token)
            .collect()
    }

    /// Whether a mention may be rewritten: any but a path token, which only when it names a
    /// planned move — a directory or a file that stays put keeps its name, and is listed in
    /// `unmoved` instead (H3).
    fn reaches(
        &self,
        rel: &Path,
        text: &str,
        token: &ids::Token,
        unmoved: &mut Vec<Stale>,
    ) -> bool {
        if !token.path || names_move(text, rel, token, self.moves) {
            return true;
        }
        let (line, col) = ids::line_col(text, token.start);
        let end = written_path(text, token.start, token.end).map_or(token.end, |(_, end)| end);
        unmoved.push(Stale {
            path: rel.to_path_buf(),
            line,
            col,
            name: text[token.start..end].to_string(),
        });
        false
    }
}

/// The splices a rename makes in one file: every in-scope mention it reaches, and every alias
/// form.
fn rename_splices(
    scope: &Reach,
    rel: &Path,
    text: &str,
    target: &Target,
    unmoved: &mut Vec<Stale>,
) -> Vec<Splice> {
    let Target::Rename { slug, aliases, .. } = target else {
        return Vec::new();
    };
    let core_len = scope.id.family.prefix.len() + scope.id.number.len();
    let mut splices: Vec<Splice> = Vec::new();
    let mut members = Vec::new();
    for alias in aliases_in(aliases, Some(rel)) {
        let quotes = ids::quotes_masked(rel);
        for (start, end) in alias_matches(text, alias, &scope.id.number, quotes) {
            splices.push((start, end, expected_alias(target)));
        }
        members.extend(alias_members(text, alias, scope.id, quotes));
    }
    for token in scope.mentions(rel, text, &members) {
        let overlaps = splices
            .iter()
            .any(|(start, end, _)| token.start < *end && *start < token.end);
        let new = format!("{}-{slug}", &text[token.start..token.start + core_len]);
        if overlaps || text[token.start..token.end] == new {
            continue;
        }
        if scope.reaches(rel, text, &token, unmoved) {
            splices.push((token.start, token.end, new));
        }
    }
    splices.sort_by_key(|splice| splice.0);
    splices
}

/// Whether the path token `token` names one of the moved files: the path written around it
/// resolves to the file ([`links_move`]) — or, written as a bare file name, it is that file's
/// name, for a mention the scope claims (H3).
fn names_move(text: &str, rel: &Path, token: &ids::Token, moves: &[(PathBuf, PathBuf)]) -> bool {
    let Some((start, end)) = written_path(text, token.start, token.end) else {
        return false;
    };
    let written = &text[start..end];
    moves.iter().any(|(from, _)| {
        resolves_to(rel, written, from)
            || (!written.contains('/')
                && from.file_name().and_then(|n| n.to_str()) == Some(written))
    })
}

/// Whether the path written around the token `token` resolves to one of the moved files (F1) and
/// to no other file under `root` — a link that could mean either is never rewritten (A1 at round
/// 5).
fn links_move(
    root: &Path,
    text: &str,
    rel: &Path,
    token: &ids::Token,
    moves: &[(PathBuf, PathBuf)],
) -> bool {
    let Some((start, end)) = written_path(text, token.start, token.end) else {
        return false;
    };
    let paths = resolved(rel, &text[start..end]);
    let moved = |path: &PathBuf| moves.iter().any(|(from, _)| from == path);
    paths.iter().any(moved)
        && !paths
            .iter()
            .any(|path| !moved(path) && root.join(path).is_file())
}

/// The byte range of the path written around `start..end` in `text`: back to the nearest
/// character no path holds, and on through any extensions — or `None` when a `/` follows, a
/// directory segment, which names no move: directories never move.
fn written_path(text: &str, start: usize, end: usize) -> Option<(usize, usize)> {
    let path_char = |c: char| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '/');
    let start = text[..start]
        .char_indices()
        .rev()
        .find(|&(_, c)| !path_char(c))
        .map_or(0, |(i, c)| i + c.len_utf8());
    let mut end = end;
    while let Some(extension) = text[end..].strip_prefix('.') {
        let len = extension
            .bytes()
            .take_while(u8::is_ascii_alphanumeric)
            .count();
        if len == 0 {
            break;
        }
        end += 1 + len;
    }
    (!text[end..].starts_with('/')).then_some((start, end))
}

/// Whether the path `written` in the file `rel` is `file` ([`resolved`]). A bare name is only the
/// file beside it (F1).
fn resolves_to(rel: &Path, written: &str, file: &Path) -> bool {
    resolved(rel, written).iter().any(|path| path == file)
}

/// The tree paths `written` in the file `rel` can name: read from the citing file's directory, or
/// — written with a `/` — from the tree root. A path that climbs out of the tree names none.
fn resolved(rel: &Path, written: &str) -> Vec<PathBuf> {
    let here = rel.parent().unwrap_or(Path::new(""));
    let rooted = written
        .contains('/')
        .then(|| PathBuf::from(written.trim_start_matches('/')));
    std::iter::once(here.join(written))
        .chain(rooted)
        .filter_map(|path| normalized(&path))
        .collect()
}

/// `path` with its `.` and `..` parts resolved, or `None` when it climbs out of the tree.
fn normalized(path: &Path) -> Option<PathBuf> {
    let mut out = PathBuf::new();
    for part in path.components() {
        match part {
            std::path::Component::Normal(part) => out.push(part),
            std::path::Component::ParentDir if !out.pop() => return None,
            _ => {}
        }
    }
    Some(out)
}

/// The text an alias form is rewritten to: `` `<owner>` <core>-<slug> ``, or `<core>-<slug>` for
/// a project-wide family.
fn expected_alias(target: &Target) -> String {
    let Target::Rename {
        family,
        number,
        slug,
        owner,
        ..
    } = target
    else {
        return String::new();
    };
    let joined = format!("{}{number}-{slug}", family.prefix);
    match owner {
        Some(owner) => format!("`{owner}` {joined}"),
        None => joined,
    }
}

/// The files `files` would change, each with its splices applied.
fn plan_edits<F>(root: &Path, files: &[PathBuf], mut splices_of: F) -> Vec<Edit>
where
    F: FnMut(&Path, &str) -> Vec<Splice>,
{
    let mut edits = Vec::new();
    for rel in files {
        let Ok(old) = std::fs::read_to_string(root.join(rel)) else {
            continue;
        };
        let splices = splices_of(rel, &old);
        if splices.is_empty() {
            continue;
        }
        let mut new = String::with_capacity(old.len());
        let mut from = 0;
        for (start, end, text) in &splices {
            new.push_str(&old[from..*start]);
            new.push_str(text);
            from = *end;
        }
        new.push_str(&old[from..]);
        if new != old {
            edits.push(Edit {
                path: rel.clone(),
                old,
                new,
            });
        }
    }
    edits
}

/// The moves a rewrite makes: every in-scope file whose own name carries the ID and a slug that
/// `moves` admits (D12). `renamed` takes the name, the byte its core ends at and the byte its slug
/// ends at. A name whose run `moves` refuses stays: a local label (N2), or a shared number's
/// other holder (wave 1b). Refused when a move's target exists — another move's source among them
/// — or two moves share one, compared without case (C1, L3).
fn plan_moves<M, F>(
    root: &Path,
    files: &[PathBuf],
    id: &ids::Id,
    owner: Option<&Owner>,
    owning: &Path,
    moves_slug: M,
    renamed: F,
) -> Result<Vec<(PathBuf, PathBuf)>, Refused>
where
    M: Fn(&str) -> bool,
    F: Fn(&str, usize, usize) -> String,
{
    let scope = owner.and_then(|_| ids::owner_dir(owning));
    let mut moves: Vec<(PathBuf, PathBuf)> = Vec::new();
    for rel in files {
        if scope.as_ref().is_some_and(|dir| !rel.starts_with(dir)) {
            continue;
        }
        let Some(name) = rel.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        // N2: a name's run moves only when it is a slug — three words, or a file-name slug.
        let token = ids::scan(name).into_iter().find(|t| {
            t.start == 0
                && t.family == id.family
                && t.number == id.number
                && t.slug.as_deref().is_some_and(&moves_slug)
        });
        let Some(token) = token else { continue };
        let core_end = id.family.prefix.len() + token.number.len();
        let new_name = renamed(name, core_end, token.end);
        if new_name == name {
            continue;
        }
        let to = rel.with_file_name(&new_name);
        if root.join(&to).exists() {
            return Err(Refused(format!(
                "`{}` already exists; the move of `{}` would overwrite it",
                to.display(),
                rel.display()
            )));
        }
        let same = |held: &PathBuf| {
            held.to_string_lossy()
                .eq_ignore_ascii_case(&to.to_string_lossy())
        };
        if let Some((other, _)) = moves.iter().find(|(_, held)| same(held)) {
            return Err(Refused(format!(
                "`{}` and `{}` would both move to `{}`",
                other.display(),
                rel.display(),
                to.display()
            )));
        }
        moves.push((rel.clone(), to));
    }
    Ok(moves)
}

/// Every spot in `files` that, once `edits` are made, still names a moved file by its old name,
/// matched without ASCII case (A3) — the old name standing as a whole file name, not glued to a
/// longer one (N5 as ruled at build), in whatever path it is written: the tool never judges which
/// file a link means (option B at round 5) — with the path tokens `unmoved` kept (H3), one spot
/// per place however many moves share the name (A2), in tree order.
fn stale_spots(
    root: &Path,
    files: &[PathBuf],
    edits: &[Edit],
    moves: &[(PathBuf, PathBuf)],
    unmoved: Vec<Stale>,
) -> Vec<Stale> {
    let names: Vec<String> = moves
        .iter()
        .filter_map(|(from, _)| from.file_name().and_then(|n| n.to_str()))
        .map(str::to_ascii_lowercase)
        .collect();
    let mut out = Vec::new();
    for rel in files.iter().filter(|_| !names.is_empty()) {
        let text = match edits.iter().find(|edit| edit.path == *rel) {
            Some(edit) => edit.new.clone(),
            None => match std::fs::read_to_string(root.join(rel)) {
                Ok(text) => text,
                Err(_) => continue,
            },
        };
        // ASCII lower-casing keeps every byte where it was.
        let lower = text.to_ascii_lowercase();
        for name in &names {
            let mut from = 0;
            while let Some(found) = lower[from..].find(name.as_str()) {
                let start = from + found;
                from = start + name.len();
                let longer = text[from..].starts_with(|c: char| c.is_ascii_alphanumeric());
                if ids::glued(&text, start) || longer {
                    continue;
                }
                let (line, col) = ids::line_col(&text, start);
                out.push(Stale {
                    path: rel.clone(),
                    line,
                    col,
                    name: text[start..from].to_string(),
                });
            }
        }
    }
    out.extend(unmoved);
    // The sort is stable: a sweep spot stays ahead of the path token at its place (H3).
    out.sort_by(|a, b| (&a.path, a.line, a.col).cmp(&(&b.path, b.line, b.col)));
    out.dedup_by(|a, b| (&a.path, a.line, a.col) == (&b.path, b.line, b.col));
    out
}

// ---------------------------------------------------------------------------
// matching alias and literal forms
// ---------------------------------------------------------------------------

/// Whether a match ending at `end` and starting at `start` is one end of a range, or is followed
/// by a `-<digit>` label tail.
fn range_or_label(text: &str, start: usize, end: usize) -> bool {
    let rest = &text[end..];
    ids::RANGE_JOINERS
        .iter()
        .any(|joiner| text[..start].ends_with(joiner) || rest.starts_with(joiner))
        || (rest.starts_with('-') && rest[1..].starts_with(|c: char| c.is_ascii_alphanumeric()))
}

/// The scanner tokens joined after byte `end` into one compound with what ends there (D5, C3).
fn members_after<'t>(text: &str, end: usize, tokens: &'t [ids::Token]) -> Vec<&'t ids::Token> {
    let mut members = Vec::new();
    let mut at = end;
    for token in tokens.iter().filter(|t| t.start >= end) {
        if !ids::joins(text, at, token.start) {
            break;
        }
        members.push(token);
        at = token.end;
    }
    members
}

/// Every `<alias><number>` in `text`, outside masked spans and ranges (`PO-D1–D7` stays bare), and
/// outside lists: a form heading three or more joined members stays bare (M4).
fn alias_matches(text: &str, alias: &str, number: &str, quotes: bool) -> Vec<(usize, usize)> {
    let mask = ids::masked_as(text, quotes);
    let tokens = ids::scan_as(text, quotes);
    let form = format!("{alias}{number}");
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(found) = text[from..].find(&form) {
        let start = from + found;
        let end = start + form.len();
        from = end;
        let longer = text[end..].starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
            || (text[end..].starts_with('.')
                && text[end + 1..].starts_with(|c: char| c.is_ascii_digit()));
        if mask[start]
            || ids::glued(text, start)
            || longer
            || range_or_label(text, start, end)
            || members_after(text, end, &tokens).len() >= 3
        {
            continue;
        }
        // The rewrite carries its own code span (`` `owner` ``), so a form in a code span takes
        // that span whole when it is the span's only content, and is left as written inside a
        // longer one — a code span nested in a code span is broken markdown.
        let line_start = text[..start].rfind('\n').map_or(0, |i| i + 1);
        let in_code = text[line_start..start].matches('`').count() % 2 == 1;
        if in_code {
            if text[..start].ends_with('`') && text[end..].starts_with('`') {
                out.push((start - 1, end + 1));
            }
            continue;
        }
        out.push((start, end));
    }
    out
}

/// The starts of `id`'s tokens that are members of an alias form's compound — `D2` in "`AD-D1/D2`" —
/// and so belong to the alias's owner (L4 as ruled at review), unless the form heads a list of
/// four (M4).
fn alias_members(text: &str, alias: &str, id: &ids::Id, quotes: bool) -> Vec<usize> {
    let mask = ids::masked_as(text, quotes);
    let tokens = ids::scan_as(text, quotes);
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(found) = text[from..].find(alias) {
        let start = from + found;
        from = start + alias.len();
        let digits = text[from..].bytes().take_while(u8::is_ascii_digit).count();
        if digits == 0 || mask[start] || ids::glued(text, start) {
            continue;
        }
        let members = members_after(text, from + digits, &tokens);
        if members.len() < 3 {
            out.extend(
                members
                    .iter()
                    .filter(|t| t.family == id.family && t.number == id.number)
                    .map(|t| t.start),
            );
        }
    }
    out
}

/// Every `<TOKEN>` in `text`, bare or with a three-word slug, outside masked spans, ranges, label
/// tails and lists of four or more tokens of its head (`AM-1, AM-2, AM-3, AM-5`). A run that is
/// not three words makes a local label, never re-slugged (M3).
fn literal_matches(text: &str, token: &str, quotes: bool) -> Vec<(usize, usize)> {
    let head = &token[..=token.find('-').unwrap_or(0)];
    let mask = ids::masked_as(text, quotes);
    let mut siblings: Vec<(usize, usize, bool)> = Vec::new();
    let mut from = 0;
    while let Some(found) = text[from..].find(head) {
        let start = from + found;
        from = start + head.len();
        let digits = text[from..].bytes().take_while(u8::is_ascii_digit).count();
        if digits == 0 || ids::glued(text, start) || mask[start] {
            continue;
        }
        let mut end = from + digits;
        if text[end..].starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_') {
            continue;
        }
        let bare = text[start..end] == *token;
        let mut words = 0;
        while let Some(word) = ids::slug_word(text, end) {
            end += 1 + word.len();
            words += 1;
        }
        siblings.push((start, end, bare && matches!(words, 0 | 3)));
        from = end;
    }
    let spans: Vec<(usize, usize)> = siblings
        .iter()
        .map(|&(start, end, _)| (start, end))
        .collect();
    let mut out = Vec::new();
    for run in ids::joined_runs(text, &spans) {
        if run.len() > 3 {
            continue;
        }
        for &(start, end, ours) in &siblings[run] {
            if ours && !range_or_label(text, start, end) {
                out.push((start, end));
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// the diff check (D15)
// ---------------------------------------------------------------------------

/// One target occurrence: its span, and whether it is an alias form.
#[derive(Clone, Copy, Debug)]
struct Occurrence {
    start: usize,
    end: usize,
    alias: bool,
}

/// The D15 diff check: every changed span between `old` and `new` is a token of `target`,
/// rewritten to the form `target` expects. `Ok` carries how many spans changed.
///
/// Both texts are scanned for the target's tokens, which are paired in order: each pair is
/// unchanged or rewritten to exactly the expected text, and with every token replaced by one
/// placeholder the two texts are byte-equal. A change anywhere else — a word, a quote, a token of
/// another ID — fails one of those three, and blocks.
///
/// It reads the target as for a file in no alias scope: tree-wide aliases apply, scoped ones do not
/// (A3, A5).
pub fn diff_check(old: &str, new: &str, target: &Target) -> Result<usize, String> {
    diff_check_as(old, new, target, true, None)
}

/// [`diff_check`] for a file the quote mask applies to (`quotes`) or not, an HTML one (N4), and
/// for the file `rel`, whose alias scopes apply (A3).
fn diff_check_as(
    old: &str,
    new: &str,
    target: &Target,
    quotes: bool,
    rel: Option<&Path>,
) -> Result<usize, String> {
    let before = occurrences(old, target, true, quotes, rel);
    let mut after = occurrences(new, target, false, quotes, rel);
    if before.len() != after.len() {
        return Err(format!(
            "the rewrite changes how many target tokens the text carries ({} to {})",
            before.len(),
            after.len()
        ));
    }
    let mut changed = 0;
    for (was, now) in before.iter().zip(after.iter_mut()) {
        let old_text = &old[was.start..was.end];
        let expected = if was.alias {
            let alias = expected_alias(target);
            let qualifier = alias.len().checked_sub(now.end - now.start);
            let start = qualifier.and_then(|q| now.start.checked_sub(q));
            if let Some(start) = start {
                if new.get(start..now.end) == Some(alias.as_str()) {
                    now.start = start;
                }
            }
            alias
        } else {
            expected_token(target, old_text)
        };
        let new_text = &new[now.start..now.end];
        if new_text == old_text {
            continue;
        }
        if new_text != expected {
            return Err(format!(
                "`{old_text}` became `{new_text}`, not the expected `{expected}`"
            ));
        }
        if !chosen_token(target, old_text) {
            return Err(format!(
                "`{old_text}` is not the chosen holder's: a shared number moves only its own slugs"
            ));
        }
        changed += 1;
    }
    if residue(old, &before) != residue(new, &after) {
        return Err("a changed span is not an ID token of the target".into());
    }
    Ok(changed)
}

/// Whether a changed token may change under `target`: always, unless the target is a rekey on a
/// shared number, where only a token carrying one of the chosen holder's slugs may (F4, R2).
fn chosen_token(target: &Target, old_text: &str) -> bool {
    let Target::Rekey {
        family,
        from,
        slugs,
        shared: true,
        ..
    } = target
    else {
        return true;
    };
    old_text[family.prefix.len()..]
        .strip_prefix(from.as_str())
        .and_then(|rest| rest.strip_prefix('-'))
        .is_some_and(|slug| slugs.iter().any(|held| held == slug))
}

/// Whether `target` lets the file `path` move: always, unless the target is a rekey on a shared
/// number, where only a file whose name carries one of the chosen holder's slugs may (F4).
fn chosen_move(target: &Target, path: &Path) -> bool {
    let Target::Rekey {
        family,
        from,
        slugs,
        shared: true,
        ..
    } = target
    else {
        return true;
    };
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    ids::scan(name).into_iter().any(|t| {
        t.start == 0
            && t.family == *family
            && t.number == *from
            && t.slug.as_ref().is_some_and(|slug| slugs.contains(slug))
    })
}

/// The target's occurrences in `text`. Aliases are matched only on the old side, where they still
/// stand; on the new side the rewritten alias is an ordinary token behind its qualifier.
fn occurrences(
    text: &str,
    target: &Target,
    old_side: bool,
    quotes: bool,
    rel: Option<&Path>,
) -> Vec<Occurrence> {
    let mut out: Vec<Occurrence> = match target {
        Target::Literal { token, .. } => literal_matches(text, token, quotes)
            .into_iter()
            .map(|(start, end)| Occurrence {
                start,
                end,
                alias: false,
            })
            .collect(),
        Target::Rename {
            family,
            number,
            slugs,
            ..
        } => tokens_of(text, family, &[number], slugs, quotes),
        Target::Rekey {
            family,
            from,
            to,
            slugs,
            ..
        } => tokens_of(text, family, &[from, to], slugs, quotes),
    };
    if let (
        true,
        Target::Rename {
            aliases, number, ..
        },
    ) = (old_side, target)
    {
        for alias in aliases_in(aliases, rel) {
            for (start, end) in alias_matches(text, alias, number, quotes) {
                out.retain(|o| !(o.start < end && start < o.end));
                out.push(Occurrence {
                    start,
                    end,
                    alias: true,
                });
            }
        }
    }
    out.sort_by_key(|o| o.start);
    out
}

/// The unskipped scanner tokens of `family` carrying one of `numbers`, each bare, three words, or
/// one of the held `slugs`: any other run is a local label (N2), so it stays in the residue and a
/// rewrite of it blocks (M1).
fn tokens_of(
    text: &str,
    family: &Family,
    numbers: &[&String],
    slugs: &[String],
    quotes: bool,
) -> Vec<Occurrence> {
    let held = |slug: &String| slug.split('-').count() == 3 || slugs.contains(slug);
    ids::scan_as(text, quotes)
        .into_iter()
        .filter(|t| {
            t.skip.is_none()
                && t.family == family
                && numbers.contains(&&t.number)
                && t.slug.as_ref().is_none_or(held)
        })
        .map(|t| Occurrence {
            start: t.start,
            end: t.end,
            alias: false,
        })
        .collect()
}

/// The text a non-alias token is rewritten to.
fn expected_token(target: &Target, old_text: &str) -> String {
    match target {
        Target::Rename {
            family,
            number,
            slug,
            ..
        } => format!("{}-{slug}", &old_text[..family.prefix.len() + number.len()]),
        Target::Rekey {
            family, from, to, ..
        } => {
            let prefix = &old_text[..family.prefix.len()];
            match old_text[family.prefix.len()..].strip_prefix(from.as_str()) {
                Some(rest) => format!("{prefix}{to}{rest}"),
                None => old_text.to_string(),
            }
        }
        Target::Literal { token, slug } => format!("{token}-{slug}"),
    }
}

/// `text` with every occurrence replaced by one placeholder character.
fn residue(text: &str, occurrences: &[Occurrence]) -> String {
    let mut out = String::with_capacity(text.len());
    let mut from = 0;
    for occurrence in occurrences {
        out.push_str(&text[from..occurrence.start]);
        out.push('\u{0}');
        from = occurrence.end;
    }
    out.push_str(&text[from..]);
    out
}

// ---------------------------------------------------------------------------
// preview and write
// ---------------------------------------------------------------------------

/// The preview: every changed line as `path:line`, then `-old` and `+new`, then every move, then
/// every stale link a write would refuse on, then every untied mention left as written (Q1), then
/// every mention kept as another holder's (A2).
pub fn preview(plan: &Plan) -> String {
    let mut out = String::new();
    for edit in &plan.edits {
        for (number, (old, new)) in edit.old.lines().zip(edit.new.lines()).enumerate() {
            if old != new {
                out.push_str(&format!(
                    "{}:{}\n-{old}\n+{new}\n",
                    edit.path.display(),
                    number + 1
                ));
            }
        }
    }
    for (from, to) in &plan.moves {
        out.push_str(&format!("move {} to {}\n", from.display(), to.display()));
    }
    for stale in &plan.stale {
        out.push_str(&format!("stale link {stale}\n"));
    }
    for untied in &plan.untied {
        out.push_str(&format!("untied mention {untied}\n"));
    }
    for kept in &plan.kept {
        out.push_str(&format!("kept {kept} · another holder's\n"));
    }
    out
}

/// Why a write did not complete.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WriteError {
    /// Refused before anything was written: spots still name a moved file by its old name (N5).
    Stale(Vec<Stale>),
    /// Blocked by the diff check, the disk, or a move's target; `written` lists the files already
    /// written when the write failed partway (M2).
    Blocked {
        reason: String,
        written: Vec<PathBuf>,
    },
}

impl std::fmt::Display for WriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteError::Stale(spots) => write!(
                f,
                "{} spots still name a moved file by its old name",
                spots.len()
            ),
            WriteError::Blocked { reason, .. } => write!(f, "{reason}"),
        }
    }
}

/// Refuse a plan with stale links (N5; ruling (f) as reversed at review: the one write path
/// refuses), then diff-check every edit and confirm no move's target exists (C1), then write the
/// edits and make the moves. Nothing is written when any check fails, or when a file changed on
/// disk since it was planned. `Ok` carries `(files, spans)`.
///
/// Each file is written to a sibling temporary carrying the file's own permissions (H1) and renamed
/// over the original, so a failed write never leaves a file half-written. A failure partway through
/// a multi-file write names the files already written, left in place (M2): the tree is a git tree,
/// and git is the undo.
pub fn write(root: &Path, plan: &Plan) -> Result<(usize, usize), WriteError> {
    if !plan.stale.is_empty() {
        return Err(WriteError::Stale(plan.stale.clone()));
    }
    let blocked = |reason: String, written: &[PathBuf]| WriteError::Blocked {
        reason,
        written: written.to_vec(),
    };
    let mut spans = 0;
    for edit in &plan.edits {
        spans += diff_check_as(
            &edit.old,
            &edit.new,
            &plan.target,
            ids::quotes_masked(&edit.path),
            Some(&edit.path),
        )
        .map_err(|reason| blocked(format!("{}: {reason}", edit.path.display()), &[]))?;
        let current = std::fs::read_to_string(root.join(&edit.path)).unwrap_or_default();
        if current != edit.old {
            return Err(blocked(
                format!("{}: changed on disk since it was read", edit.path.display()),
                &[],
            ));
        }
    }
    if let Some((from, _)) = plan
        .moves
        .iter()
        .find(|(from, _)| !chosen_move(&plan.target, from))
    {
        return Err(blocked(
            format!(
                "`{}` is not the chosen holder's file: a shared number moves only its own",
                from.display()
            ),
            &[],
        ));
    }
    if let Some((from, to)) = plan.moves.iter().find(|(_, to)| root.join(to).exists()) {
        return Err(blocked(
            format!(
                "`{}` exists; the move of `{}` would overwrite it",
                to.display(),
                from.display()
            ),
            &[],
        ));
    }
    let mut written: Vec<PathBuf> = Vec::new();
    for edit in &plan.edits {
        let path = root.join(&edit.path);
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let temporary = path.with_file_name(format!(".{name}.ids-tmp"));
        let mode = std::fs::metadata(&path).map(|meta| meta.permissions());
        std::fs::write(&temporary, &edit.new)
            .and_then(|()| match &mode {
                Ok(mode) => std::fs::set_permissions(&temporary, mode.clone()),
                Err(_) => Ok(()),
            })
            .and_then(|()| std::fs::rename(&temporary, &path))
            .map_err(|e| {
                blocked(
                    format!("{}: cannot write: {e}", edit.path.display()),
                    &written,
                )
            })?;
        written.push(edit.path.clone());
    }
    for (from, to) in &plan.moves {
        std::fs::rename(root.join(from), root.join(to)).map_err(|e| {
            blocked(
                format!("cannot move {} to {}: {e}", from.display(), to.display()),
                &written,
            )
        })?;
        written.push(to.clone());
    }
    Ok((plan.edits.len(), spans))
}
