//! The six mechanical conformance checks, the first-touch amnesty, and the `Edit`-in-memory leg.
//!
//! Given what the log declares for a path ([`crate::home`]) and the body a write would leave on
//! disk, this module answers allow or deny. That is the whole of it: **every check is decidable by
//! string and count**, and none of them reads meaning, grades quality, or sequences a seat — which
//! is what keeps the write-time gate on the admitted side of the bright line (GI-019). The tool is
//! checking an artifact against the log's own declaration, never against a standard of its own.
//!
//! # The six checks, in the order a deny reports them (record D4)
//!
//! 1. **path** — the file sits under a declared home. Resolved upstream by [`crate::home`]; under
//!    `<root>/.mochiko/` a path no home governs is refused outright ([`closed_world`]).
//! 2. **file set** — the name is a declared deliverable, or any name under a declared `reports/`.
//!    Relaxable by the amnesty below, unlike the path itself.
//! 3. **frontmatter** — required keys present; a key declared as an enum holds a listed value.
//! 4. **headings** — required `##` headings present in declared order; an undeclared `##` denied.
//! 5. **placeholders** — declared tokens absent from frontmatter values and heading text, read
//!    outside backticked code spans: a quotation of a pattern is not an instance of it.
//! 6. **size** — each heading's span within its budget, or the whole-file/per-entry bound; a
//!    store's text above its first heading is bounded as one entry.
//!
//! # First-touch amnesty (D4e, as amended at review C3/V3)
//!
//! A file may not be wedged by its own history. Where the file already exists, every fault the
//! candidate carries is re-measured against the on-disk baseline: a fault the baseline already had
//! and the candidate does not worsen is **allowed**, with the standing overage reported as
//! `additionalContext` rather than hidden. Only a worsening — or a brand-new fault — denies. So the
//! rewrite that fixes an oversized file is itself an allowed write, which is the property that
//! makes the violator pass over the existing tree possible at all.
//!
//! The **file set** is inside the amnesty, and the path is not. An existing file at an undeclared
//! name in a declared home is allowed through with the violation named, because a file already on
//! disk cannot be re-homed by a gate that refuses to let anyone touch it; a file that does not
//! exist yet is denied, because nothing is wedged by refusing to create it. A path outside every
//! declared home, or in an undeclared sub-directory, is never relaxed — there is no home to
//! measure the write against.

use crate::home::{Bounds, Deliverable, Form, Home, Homes, Reports, Resolution};
use crate::render;
use crate::replay::State;
use crate::schema::Template;
use serde_norway::Value;
use std::collections::BTreeMap;
use std::path::Path;

/// D9's advisory closing sentence, on every deny reason.
///
/// Advisory by ruling: the hooks are stateless and cannot see a loop, so this helps an obedient
/// seat and is not a mechanism. The mechanisms are the `Bash` deny and the every-permission-mode
/// floor.
const HALT_SENTENCE: &str =
    "a second deny on this path halts — surface the check's text to the user, do not rewrite \
     around it.";

/// The same sentence, for a caller that builds its own reason text (the shell leg).
pub const HALT_HINT: &str = HALT_SENTENCE;

/// Allow or deny. There is no third value: `ask` would put a mechanical check in front of the user.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decision {
    Allow,
    Deny,
}

/// One verdict about one write.
#[derive(Debug)]
pub struct Verdict {
    pub decision: Decision,
    /// The deny reason, naming what failed and what was required. `None` on an allow.
    pub reason: Option<String>,
    /// A standing overage an amnesty allowed through, for the hook's `additionalContext`.
    pub context: Option<String>,
}

impl Verdict {
    fn allow() -> Verdict {
        Verdict {
            decision: Decision::Allow,
            reason: None,
            context: None,
        }
    }

    fn allow_with(context: String) -> Verdict {
        Verdict {
            decision: Decision::Allow,
            reason: None,
            context: Some(context),
        }
    }

    fn deny(reason: impl Into<String>) -> Verdict {
        Verdict {
            decision: Decision::Deny,
            reason: Some(format!("{} {HALT_SENTENCE}", reason.into())),
            context: None,
        }
    }
}

/// One thing wrong with a body.
///
/// `key` identifies the *measure*, so a candidate fault can be compared with the baseline's fault
/// about the same measure — that comparison is the amnesty. `magnitude` is the measured number
/// where one exists, so "over by two lines" can be told from "over by nine".
#[derive(Debug, Clone)]
struct Fault {
    key: String,
    magnitude: Option<usize>,
    message: String,
}

impl Fault {
    fn new(key: impl Into<String>, magnitude: Option<usize>, message: impl Into<String>) -> Fault {
        Fault {
            key: key.into(),
            magnitude,
            message: message.into(),
        }
    }

    /// Whether this fault is no worse than the same measure in the baseline.
    fn not_worsened_from(&self, baseline: &Fault) -> bool {
        match (self.magnitude, baseline.magnitude) {
            (Some(now), Some(was)) => now <= was,
            // A fault with no magnitude is present or absent; present in both is not a worsening.
            (None, None) => true,
            // A measure that gained or lost its magnitude is not comparable, so it is not excused.
            _ => false,
        }
    }
}

/// Apply an `Edit`'s replacement in memory, or `None` when `old_string` is not in the baseline.
///
/// Only the first occurrence is replaced, matching the tool's own semantics. The result is what the
/// checks grade, so an append that adds an undeclared heading or crosses a budget is denied before
/// it lands (record D4e).
pub fn apply_edit(baseline: &str, old_string: &str, new_string: &str) -> Option<String> {
    let at = baseline.find(old_string)?;
    let mut out = String::with_capacity(baseline.len() + new_string.len());
    out.push_str(&baseline[..at]);
    out.push_str(new_string);
    out.push_str(&baseline[at + old_string.len()..]);
    Some(out)
}

/// Grade a write against what the log declares for its path.
///
/// `candidate` is the body the write would leave on disk — `tool_input.content` for a `Write`, the
/// result of [`apply_edit`] for an `Edit`. `baseline` is the file's current content, when it exists;
/// its presence is what enables the amnesty.
///
/// `run_folder` is the run folder's absolute path when the log declares a raw-output home
/// ([`crate::home::Homes::run_folder`]); a path-class deny names it as the raw-output route.
pub fn check(
    state: &State,
    resolution: &Resolution,
    candidate: &str,
    baseline: Option<&str>,
    run_folder: Option<&str>,
) -> Verdict {
    match resolution {
        // Nothing in the log governs this path. The out-of-home sniff is `sniff`'s job, and the
        // caller runs it — a check that silently did both could not be reasoned about.
        Resolution::Outside => Verdict::allow(),
        // A declared sub-directory with no home document of its own. Inventing a rule for it is
        // exactly what D4f forbids.
        Resolution::Deferred { .. } => Verdict::allow(),
        // Raw output: shape and size unchecked by ruling (field review D4). The run folder's own
        // controls — the ignore guard, the main-tree rule, the report sniff on `.md` — are the
        // hook's, because they read the repository's layout rather than the body.
        Resolution::Raw { .. } => Verdict::allow(),
        // The file set is a *relaxable* measure (D4e, as ratified at AM-3): an undeclared name on a
        // file that is already on disk is amnestied and named in `additionalContext`, so a
        // mis-homed file is never wedged — those are exactly the files the violator pass rewrites.
        // A *new* file at an undeclared name has no baseline and still denies, so the set binds on
        // everything that does not exist yet.
        Resolution::UndeclaredFile { home, name } => {
            let fault = || {
                vec![Fault::new(
                    format!("file-set:{name}"),
                    None,
                    undeclared_file_reason(home, name, run_folder),
                )]
            };
            settle(fault(), || baseline.map(|_| fault())).unwrap_or_else(Verdict::allow)
        }
        // D3's last sentence: the route this reason names is the run folder, not "a new
        // sub-directory takes a migration" — the sentence the copying pattern booked (F12).
        Resolution::UndeclaredSubdir { home, subdir } => Verdict::deny(format!(
            "`{subdir}/` is not a declared sub-directory of `{}`. Declared: {}. {}",
            home.display_path(),
            list(&home.subdirs, "none"),
            routes(Some(home), &home.display_path(), run_folder)
        )),
        Resolution::Report {
            home,
            reports,
            name,
        } => {
            let faults = report_faults(state, home, reports, candidate, name);
            settle(faults, || {
                baseline.map(|text| report_faults(state, home, reports, text, name))
            })
            .unwrap_or_else(Verdict::allow)
        }
        Resolution::File { home, deliverable } => {
            let faults = file_faults(state, home, deliverable, candidate);
            settle(faults, || {
                baseline.map(|text| file_faults(state, home, deliverable, text))
            })
            .unwrap_or_else(Verdict::allow)
        }
    }
}

/// The D9 sniff: a write outside every declared home, gated only when it is a mochiko report.
///
/// A plain `.md` elsewhere is not mochiko's business — a product repository has its own docs, and
/// gating them would be an overreach the consuming project never ratified.
pub fn sniff(state: &State, path: &str, candidate: &str) -> Verdict {
    let Some(kind) = report_kind(state, candidate) else {
        return Verdict::allow();
    };
    Verdict::deny(format!(
        "`{path}` carries `report: {kind}`, a mochiko report type, but sits under no declared \
         home. A report lands in its home's `reports/` directory.",
    ))
}

/// The same sniff inside a place the log admits by location and leaves unchecked for content —
/// the run folder, and every declared sub-directory with no home document of its own (field
/// review D4 control 4 and V2). Raw working material is what those places are for; a report is
/// not, so no report hides there. `place` names where the file sits, for the reason.
pub fn sniff_unchecked(state: &State, path: &str, candidate: &str, place: &str) -> Verdict {
    let Some(kind) = report_kind(state, candidate) else {
        return Verdict::allow();
    };
    Verdict::deny(format!(
        "`{path}` carries `report: {kind}`, a mochiko report type, in {place}, where content is \
         unchecked and no report may hide. A report lands in its home's `reports/` directory.",
    ))
}

/// The report type a body's frontmatter declares, when it is one any home's envelope enumerates.
fn report_kind(state: &State, candidate: &str) -> Option<String> {
    let kind = frontmatter(candidate)?.get("report")?.clone();
    report_types(state).contains(&kind).then_some(kind)
}

/// Every report type any declared home's envelope enumerates.
fn report_types(state: &State) -> Vec<String> {
    let mut out = Vec::new();
    for home in Homes::load(state).iter() {
        let Some(reports) = &home.reports else {
            continue;
        };
        let Ok(template) = render::template_of(state, &reports.envelope) else {
            continue;
        };
        if let Some(conformance) = &template.conformance {
            if let Some(values) = conformance.frontmatter.enums.get("report") {
                out.extend(values.iter().cloned());
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Turn a candidate's faults into a verdict, consulting the baseline's faults only if there are
/// any to excuse. `None` means the body is clean.
///
/// One key carries several faults when a file repeats a heading, and nothing tells those sections
/// apart but their size (RA3, ruled at Q3). So each key's faults are ranked largest first, in the
/// candidate and the baseline alike, and a candidate fault is compared with the standing fault of
/// its own rank; one past the standing count is new, and denies. An unchanged rewrite therefore
/// always settles, where comparing every section with the first of its name denied a file for
/// nothing but its own history. The residual, with no identity to go on: a swap inside one heading
/// that lifts no rank above its standing fault settles too.
fn settle<F>(faults: Vec<Fault>, baseline_faults: F) -> Option<Verdict>
where
    F: FnOnce() -> Option<Vec<Fault>>,
{
    if faults.is_empty() {
        return Some(Verdict::allow());
    }
    let standing = baseline_faults().unwrap_or_default();
    for fault in &faults {
        let rank = ranked(&faults, &fault.key)
            .iter()
            .position(|same| std::ptr::eq(*same, fault))
            .unwrap_or_default();
        match ranked(&standing, &fault.key).get(rank) {
            Some(was) if fault.not_worsened_from(was) => {}
            _ => return Some(Verdict::deny(fault.message.clone())),
        }
    }
    // Every fault stood before this write and none was worsened: allowed, and the overage is
    // reported rather than hidden.
    let lines: Vec<String> = faults.iter().map(|f| format!("- {}", f.message)).collect();
    Some(Verdict::allow_with(format!(
        "This file already stood outside its declared shape before this write, and this write does \
         not worsen it, so it is allowed. Standing:\n{}",
        lines.join("\n")
    )))
}

/// The faults carrying `key`, largest magnitude first; equal ones keep their order in the body.
fn ranked<'a>(faults: &'a [Fault], key: &str) -> Vec<&'a Fault> {
    let mut same: Vec<&Fault> = faults.iter().filter(|f| f.key == key).collect();
    same.sort_by_key(|fault| std::cmp::Reverse(fault.magnitude));
    same
}

fn undeclared_file_reason(home: &Home, name: &str, run_folder: Option<&str>) -> String {
    format!(
        "`{name}` is not a declared deliverable of `{}`. Declared: {}. {} Otherwise raise it \
         upstream — a new deliverable kind takes a migration in the plugin's log and a plugin \
         release.",
        home.display_path(),
        list(&home.declared_names(), "none"),
        routes(Some(home), &home.display_path(), run_folder)
    )
}

/// The two routes every closed-world deny names (field review D3), in one wording.
///
/// Route 1 is a declared deliverable, or a `reports/` file, of the nearest home — `nearest` is the
/// resolved home for a name or a sub-directory it does not declare, and [`Homes::nearest`] for a
/// path in no home at all; with none, the route points at the render that lists every home. A home
/// that declares nothing is never offered: the route names its title instead (census row H2).
/// Route 2 is the run folder for raw output, stated as ephemeral, and only when the log declares
/// one: a folder the log does not declare is never invented.
///
/// `shown` is the path the no-nearest wording tells the seat to render.
///
/// [`Homes::nearest`]: crate::home::Homes::nearest
pub fn routes(nearest: Option<&Home>, shown: &str, run_folder: Option<&str>) -> String {
    let first = match nearest {
        // A home kept with an empty set — withdrawn, as the two contracts homes were — sent the
        // seat back to the place that refused it. Its title is the log's own word on where such
        // files went.
        Some(home) if home.deliverables.is_empty() && home.reports.is_none() => format!(
            "a declared deliverable or a `reports/` file of another home — `{}` declares none \
             ({})",
            home.display_path(),
            home.title
        ),
        Some(home) => {
            let directory = home.display_path();
            match home.reports {
                Some(_) => format!(
                    "a declared deliverable of the nearest home, `{directory}`, or as a report \
                     under `{directory}reports/` (any name, a `report:` type from the envelope's \
                     enum)"
                ),
                None => format!("a declared deliverable of the nearest home, `{directory}`"),
            }
        }
        None => format!(
            "a declared deliverable or a `reports/` file of a declared home (`mochiko-cli home \
             {shown}` lists them all)"
        ),
    };
    match run_folder {
        Some(run_folder) => format!(
            "Write it as {first}; or, for raw output (captured console, logs, dumps), in the run \
             folder `{run_folder}`, which is ephemeral — deleted at the run's acceptance, never a \
             record."
        ),
        None => format!("Write it as {first}."),
    }
}

/// The routes a path under `<root>/.mochiko/` that no home governs takes, led by the run-key
/// sentence when the nearest home is the raw-output home — the only way such a path can sit under
/// the run folder's parent is a name that does not match the run key (field review D4 control 2).
pub fn closed_world_routes(homes: &Homes, path: &Path, run_folder: Option<&str>) -> String {
    let shown = path.display().to_string();
    match homes.nearest(path) {
        Some((home, shared)) if home.raw_output => {
            let segment = path.components().nth(shared);
            match (segment, home.path.get(shared)) {
                (Some(segment), Some(pattern)) => {
                    let segment = segment.as_os_str().to_string_lossy();
                    let form = if pattern == "<run-id>" {
                        " — `<owner>-run<n>`, e.g. `FEAT-001-run5`, `EPIC-003-run1`, \
                         `lane-auth-fix-run1`"
                    } else {
                        ""
                    };
                    format!(
                        "`{segment}` is not a run key: a run folder's name must match \
                         `{pattern}`{form}. {}",
                        routes(None, &shown, run_folder)
                    )
                }
                // No segment fails the key: the path is the run folder's parent, or a run folder
                // named as a file (G1 A2). The raw-output home is still the nearest.
                _ => routes(Some(home), &shown, run_folder),
            }
        }
        Some((home, _)) => routes(Some(home), &shown, run_folder),
        None => format!(
            "No declared home shares this path's prefix. {}",
            routes(None, &shown, run_folder)
        ),
    }
}

/// The closed world under `<root>/.mochiko/` (field review D3): a write that resolves to no
/// declared home is refused, whether or not the file exists — the path is not a relaxable measure.
pub fn closed_world(homes: &Homes, path: &Path, run_folder: Option<&str>) -> Verdict {
    Verdict::deny(format!(
        "`{}` resolves to no declared home under `.mochiko/`, where the world is closed: a write \
         here is refused. {}",
        path.display(),
        closed_world_routes(homes, path, run_folder)
    ))
}

/// The faults a declared deliverable's body carries.
fn file_faults(state: &State, home: &Home, deliverable: &Deliverable, body: &str) -> Vec<Fault> {
    // An append-only log is bounded per entry, never per file (D4d), and carries no shape rule.
    if deliverable.form == Some(Form::Log) {
        return log_faults(deliverable, body);
    }
    // A cumulative store is bounded per entry and carries no whole-file bound (field review D2).
    // A bound template still grades its shape; the entry budgets replace its section budgets.
    if deliverable.form == Some(Form::Entries) {
        let mut faults = entry_faults(deliverable, body);
        let template = deliverable
            .template
            .as_deref()
            .and_then(|name| render::template_of(state, name).ok());
        if let Some(template) = template {
            if let Some(conformance) = &template.conformance {
                faults.extend(shape_faults(
                    &template,
                    conformance,
                    body,
                    &heading_spans(body),
                ));
            }
        }
        return faults;
    }
    match &deliverable.template {
        Some(name) => match render::template_of(state, name) {
            Ok(template) => template_faults(&template, body, home, deliverable),
            // A dangling binding is a log defect `migrate validate` reports. Denying a seat's
            // write over it would punish the wrong party, so the shape check simply does not run.
            Err(_) => whole_file_faults(home, deliverable, body),
        },
        None => whole_file_faults(home, deliverable, body),
    }
}

/// A template-less deliverable: the whole-file bound, and nothing about shape (D4f).
fn whole_file_faults(home: &Home, deliverable: &Deliverable, body: &str) -> Vec<Fault> {
    if home.bounds == Bounds::Elsewhere {
        return Vec::new();
    }
    let Some(budget) = deliverable.max_lines else {
        return Vec::new();
    };
    let lines = line_count(body);
    if lines <= budget {
        return Vec::new();
    }
    vec![Fault::new(
        "size:file",
        Some(lines),
        format!(
            "`{}` is {lines} lines against a whole-file bound of {budget}.",
            deliverable.file
        ),
    )]
}

/// An append-only log: each `##` entry within the per-entry bound.
fn log_faults(deliverable: &Deliverable, body: &str) -> Vec<Fault> {
    let Some(budget) = deliverable.entry_max_lines else {
        return Vec::new();
    };
    let mut faults = Vec::new();
    for span in heading_spans(body) {
        if span.lines > budget {
            faults.push(Fault::new(
                format!("size:entry:{}", span.heading),
                Some(span.lines),
                format!(
                    "entry `## {}` is {} lines against a per-entry bound of {budget}.",
                    span.heading, span.lines
                ),
            ));
        }
    }
    faults
}

/// The faults a templated body carries: frontmatter, headings, placeholders, per-section size.
fn template_faults(
    template: &Template,
    body: &str,
    home: &Home,
    deliverable: &Deliverable,
) -> Vec<Fault> {
    let Some(conformance) = &template.conformance else {
        // No conformance block means no shape rule at all — never one invented from prose (D4f).
        return whole_file_faults(home, deliverable, body);
    };
    let spans = heading_spans(body);
    let mut faults = shape_faults(template, conformance, body, &spans);
    if home.bounds != Bounds::Elsewhere {
        faults.extend(size_faults(template, &spans));
    }
    faults
}

/// A templated body's shape faults — frontmatter, headings, placeholders — without its size.
fn shape_faults(
    template: &Template,
    conformance: &crate::schema::Conformance,
    body: &str,
    spans: &[Span],
) -> Vec<Fault> {
    let front = frontmatter(body);
    let mut faults = frontmatter_faults(&conformance.frontmatter, front.as_ref());
    faults.extend(heading_faults(template, conformance, spans));
    faults.extend(placeholder_faults(
        &conformance.placeholders,
        front.as_ref(),
        body,
    ));
    faults
}

/// A cumulative store's entries, each within the per-entry bound; its text above the first
/// heading, bounded as one entry (census row P1); and — under `###` entries — each `##` section's
/// own text within the section bound (field review D2; seams 3 and 4).
///
/// The fault key is `size:entry:<heading text>`, the key a log's entries already carry, so the
/// first-touch amnesty applies per entry: an entry already over budget may be edited without
/// growing, and a growing edit is denied (V9). A renamed entry is a new key.
fn entry_faults(deliverable: &Deliverable, body: &str) -> Vec<Fault> {
    // `migrate validate` rejects any other value; a delivery path must not die on it.
    let level = match deliverable.entry_heading.as_deref() {
        Some("##") => 2,
        Some("###") => 3,
        _ => return Vec::new(),
    };
    let hashes = "#".repeat(level);
    let exempt = &deliverable.entry_exempt_fields;
    let (entries, sections, preamble) = entry_spans(body, level, exempt);
    let mut faults = Vec::new();
    if let Some(budget) = deliverable.entry_max_lines {
        // Row P1 (b): nothing counted the preamble, so a store could grow there without limit.
        if preamble > budget {
            let first = if level == 2 { "`##`" } else { "`##` or `###`" };
            faults.push(Fault::new(
                "size:preamble",
                Some(preamble),
                format!(
                    "the text above the first {first} heading is {preamble} lines against a \
                     per-entry bound of {budget}, counted from the file's first line: a store's \
                     preamble is bounded as one entry."
                ),
            ));
        }
        let skipped = if exempt.is_empty() {
            String::new()
        } else {
            let names: Vec<String> = exempt.iter().map(|name| format!("**{name}:**")).collect();
            format!(" ({} lines not counted)", list(&names, "none"))
        };
        for entry in entries.iter().filter(|entry| entry.lines > budget) {
            faults.push(Fault::new(
                format!("size:entry:{}", entry.heading),
                Some(entry.lines),
                format!(
                    "entry `{hashes} {}` is {} lines against a per-entry bound of {budget}, \
                     counted from its heading to the next heading at its level or above{skipped}. \
                     An entry is keyed by its heading text, so a renamed entry is measured as a new \
                     one and does not keep the amnesty its old name had.",
                    entry.heading, entry.lines
                ),
            ));
        }
    }
    if let Some(budget) = deliverable.section_max_lines {
        for section in sections.iter().filter(|section| section.lines > budget) {
            faults.push(Fault::new(
                format!("size:section:{}", section.heading),
                Some(section.lines),
                format!(
                    "the text of `## {}` outside its entries is {} lines against a bound of \
                     {budget}, counted from its `##` line to its first `###` entry.",
                    section.heading, section.lines
                ),
            ));
        }
    }
    faults
}

/// The faults a report carries. Names are free here; the type list is not (record D2).
///
/// The envelope template is graded in full — frontmatter, its own declared headings, its
/// placeholder tokens and its section budgets — because D6 says a report kind "carries the same
/// per-section budgets over their envelope payload", and round 1's G10 found only the frontmatter
/// half applied. A per-type template named in `by_type` then adds its own section budgets on top,
/// which is how one envelope serves six report types with different payloads.
fn report_faults(
    state: &State,
    home: &Home,
    reports: &Reports,
    body: &str,
    name: &str,
) -> Vec<Fault> {
    let Ok(envelope) = render::template_of(state, &reports.envelope) else {
        return Vec::new();
    };
    let Some(conformance) = &envelope.conformance else {
        return Vec::new();
    };
    let front = frontmatter(body);
    let spans = heading_spans(body);
    let mut faults = frontmatter_faults(&conformance.frontmatter, front.as_ref());
    faults.extend(heading_faults(&envelope, conformance, &spans));
    faults.extend(placeholder_faults(
        &conformance.placeholders,
        front.as_ref(),
        body,
    ));
    if home.bounds != Bounds::Elsewhere {
        faults.extend(size_faults(&envelope, &spans));
        // A report whose type names a per-type template takes that template's budgets too.
        if let Some(kind) = front.as_ref().and_then(|f| f.get("report")) {
            if let Some(by_type) = reports.by_type.get(kind) {
                if let Ok(typed) = render::template_of(state, by_type) {
                    faults.extend(size_faults(&typed, &spans));
                }
            }
        }
    }
    // The reason names the file and its home, so a denied seat is not left hunting for which
    // report under which run tripped (round 1, G9).
    let where_it_is = format!("`{}reports/{name}`", home.display_path());
    for fault in &mut faults {
        fault.message = format!("{where_it_is}: {}", fault.message);
    }
    faults
}

fn frontmatter_faults(
    contract: &crate::schema::Frontmatter,
    front: Option<&BTreeMap<String, String>>,
) -> Vec<Fault> {
    if contract.required.is_empty() && contract.enums.is_empty() {
        return Vec::new();
    }
    let empty = BTreeMap::new();
    let front = front.unwrap_or(&empty);
    let mut faults = Vec::new();
    for key in &contract.required {
        if !front.contains_key(key) {
            faults.push(Fault::new(
                format!("frontmatter-missing:{key}"),
                None,
                format!("the frontmatter is missing the required field `{key}`."),
            ));
        }
    }
    for (key, allowed) in &contract.enums {
        let Some(value) = front.get(key) else {
            continue;
        };
        if !allowed.contains(value) {
            faults.push(Fault::new(
                format!("frontmatter-enum:{key}"),
                None,
                format!(
                    "frontmatter `{key}: {value}` is not one of {}.",
                    list(allowed, "the empty set")
                ),
            ));
        }
    }
    faults
}

fn heading_faults(
    template: &Template,
    conformance: &crate::schema::Conformance,
    spans: &[Span],
) -> Vec<Fault> {
    let declared: Vec<(&str, bool)> = template.declared_headings().collect();
    if declared.is_empty() {
        return Vec::new();
    }
    let mut faults = Vec::new();
    let present: Vec<&str> = spans.iter().map(|s| s.heading.as_str()).collect();

    for (heading, required) in &declared {
        if *required && !present.contains(heading) {
            faults.push(Fault::new(
                format!("heading-missing:{heading}"),
                None,
                format!("the required heading `## {heading}` is absent."),
            ));
        }
    }

    if conformance.denies_extra_headings() {
        for heading in &present {
            if !declared.iter().any(|(d, _)| d == heading) {
                faults.push(Fault::new(
                    format!("heading-extra:{heading}"),
                    None,
                    format!(
                        "`## {heading}` is not a declared heading of this kind. Declared: {}. \
                         `###` and deeper are yours.",
                        list(
                            &declared.iter().map(|(d, _)| *d).collect::<Vec<&str>>(),
                            "none"
                        )
                    ),
                ));
            }
        }
    }

    // Order is graded over the declared headings that are actually present, so an absent optional
    // section never reads as a reordering.
    let declared_order: Vec<&str> = declared
        .iter()
        .map(|(d, _)| *d)
        .filter(|d| present.contains(d))
        .collect();
    let present_declared: Vec<&str> = present
        .iter()
        .copied()
        .filter(|p| declared.iter().any(|(d, _)| d == p))
        .collect();
    if declared_order != present_declared {
        faults.push(Fault::new(
            "heading-order",
            None,
            format!(
                "the declared headings are out of order. Required order: {}; found: {}.",
                list(&declared_order, "none"),
                list(&present_declared, "none")
            ),
        ));
    }
    faults
}

fn placeholder_faults(
    tokens: &[String],
    front: Option<&BTreeMap<String, String>>,
    body: &str,
) -> Vec<Fault> {
    if tokens.is_empty() {
        return Vec::new();
    }
    // D4c: frontmatter values and heading text only, by exact spelling — never a body substring,
    // because honest prose names `FEAT-XXX` as a pattern.
    //
    // The heading half comes from [`heading_texts`], which is the same fence-aware scan
    // [`heading_spans`] uses. Before round 1's G2 this filtered every line starting with `#`, so a
    // `#` comment inside a fenced example counted as heading text and denied a conforming
    // artifact — a false deny, which stops honest work where a false allow only misses drift.
    // Both halves are read outside their backticked code spans, by the same [`outside_code_spans`]
    // classifier, for the reason the fence skip exists: a quotation of a pattern is not an instance
    // of it. A report about the schema writes `` `wave<n>-<slug>.md` `` in a frontmatter value, and
    // any report discussing a home would otherwise be denied by its own subject matter.
    let mut haystacks: Vec<String> = Vec::new();
    if let Some(front) = front {
        haystacks.extend(front.values().map(|value| outside_code_spans(value)));
    }
    haystacks.extend(heading_texts(body).into_iter().map(outside_code_spans));

    let mut faults = Vec::new();
    for token in tokens {
        if haystacks.iter().any(|hay| hay.contains(token.as_str())) {
            faults.push(Fault::new(
                format!("placeholder:{token}"),
                None,
                format!(
                    "the placeholder token `{token}` survives in a frontmatter value or a \
                     `##`/`###` heading."
                ),
            ));
        }
    }
    faults
}

fn size_faults(template: &Template, spans: &[Span]) -> Vec<Fault> {
    let mut faults = Vec::new();
    for span in spans {
        let Some(budget) = template.budget(&span.heading) else {
            continue;
        };
        if span.lines > budget {
            faults.push(Fault::new(
                format!("size:{}", span.heading),
                Some(span.lines),
                format!(
                    "`## {}` is {} lines against a budget of {budget}, counted from its `##` line \
                     to the next `##`, nested content included.",
                    span.heading, span.lines
                ),
            ));
        }
    }
    faults
}

// ---------------------------------------------------------------------------
// measuring a body
// ---------------------------------------------------------------------------

/// One `##` section: its heading text and how many lines it spans.
#[derive(Debug)]
struct Span {
    heading: String,
    lines: usize,
}

/// The body's line count, not counting a single trailing newline as an extra empty line.
fn line_count(body: &str) -> usize {
    body.lines().count()
}

/// The one place a line is decided to be a heading.
///
/// **Every heading question routes through here.** Two callers need the answer for different
/// reasons — [`heading_spans`] wants the `##` boundaries, [`placeholder_faults`] wants the heading
/// *text* — and a second implementation is exactly how the fence trap gets reintroduced one
/// function away from where it was closed (review round 1, G2). So the scan is written once and
/// yields the level with the text.
///
/// Two things it deliberately gets right. **Fenced blocks are not headings**: a template's own
/// skeleton carries fenced examples containing `##`, and a `#` comment inside a shell fence is a
/// comment, not a heading — reading either as one would deny a conforming file. **Only `##` and
/// `###` count** (plan §3.7): `#` is the document title and `####` and deeper are past the scope
/// D4c declares.
fn heading_scan(body: &str) -> Vec<Option<(usize, &str)>> {
    body.lines()
        .zip(fenced_lines(body))
        .map(|(line, fenced)| if fenced { None } else { heading_of(line) })
        .collect()
}

/// Per line of `body`, whether it is a fence line or inside a fenced block.
///
/// The one fence classifier in the crate: [`heading_scan`] reads it here, and the ID scanner
/// ([`crate::ids`]) reads it to leave a fenced quotation as written (`human-readable-ids` D15). A
/// second copy is how the fence trap would come back one module away from where it was closed.
pub(crate) fn fenced_lines(body: &str) -> Vec<bool> {
    let mut out = Vec::with_capacity(body.lines().count());
    let mut fenced = false;
    for line in body.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            fenced = !fenced;
            out.push(true);
            continue;
        }
        out.push(fenced);
    }
    out
}

/// One string with its backticked code spans removed, the segments joined by a space.
///
/// **Every "quotation or instance?" question routes through here**, the same write-it-once rule
/// [`heading_scan`] follows and for the same reason: a second implementation is how the trap gets
/// reintroduced one function away from where it was closed. Frontmatter values and heading text
/// both read through this one classifier.
///
/// A backticked span is a quotation of a pattern, not a surviving placeholder. Without this, a
/// report whose subject *is* the schema — one naming `` `wave<n>-<slug>.md` `` in a frontmatter
/// value — is denied by its own subject matter, and every report discussing a home would hit it.
///
/// Two details it gets right. **An unterminated backtick run is literal text**, per CommonMark, so
/// its tail is kept and a real placeholder cannot hide behind a lone backtick — the false allow
/// this would otherwise buy. And the kept segments are **joined by a space**, so splicing either
/// side of a removed span cannot manufacture a token that was never written.
///
/// The deliberate non-catch: two *stray* backticks bracket everything between them, so a placeholder
/// that happens to sit inside that accidental span is not seen. Nothing here distinguishes a real
/// code span from two unrelated backticks, and the module errs toward allow — the same posture the
/// fence skip takes, where a missed survivor is drift and a false deny stops honest work.
// `is_multiple_of` needs Rust 1.87, and this manifest declares no `rust-version`. Raising the
// floor of the whole crate is a manifest policy call, not this function's to make, so the parity
// arithmetic stands and the lint is deferred with it.
#[allow(clippy::manual_is_multiple_of)]
fn outside_code_spans(text: &str) -> String {
    let parts: Vec<&str> = text.split('`').collect();
    // An even count of backticks pairs every span; an odd count leaves the final run open.
    let unterminated = parts.len() % 2 == 0;
    let last = parts.len() - 1;
    let mut out = String::with_capacity(text.len());
    for (index, part) in parts.iter().enumerate() {
        let inside = index % 2 == 1 && !(unterminated && index == last);
        if !inside {
            out.push_str(part);
            out.push(' ');
        }
    }
    out
}

/// One unindented `##`/`###` line as its level and text, or `None`.
fn heading_of(line: &str) -> Option<(usize, &str)> {
    if !line.starts_with('#') {
        return None;
    }
    let hashes = line.len() - line.trim_start_matches('#').len();
    if !(2..=3).contains(&hashes) {
        return None;
    }
    // `##text` with no space is not a heading in CommonMark.
    line[hashes..]
        .strip_prefix(' ')
        .map(|text| (hashes, text.trim()))
}

/// Every `##` section, in document order, each counted from its own `##` line to the next `##`.
///
/// `###` and deeper are not section boundaries: they are the producer's to structure, and their
/// lines count toward the enclosing `##` (record D4a — nesting is not an escape). Fenced content
/// counts toward its span's length even though it yields no heading.
fn heading_spans(body: &str) -> Vec<Span> {
    let mut spans: Vec<Span> = Vec::new();
    for classified in heading_scan(body) {
        match classified {
            Some((2, text)) => spans.push(Span {
                heading: text.to_string(),
                lines: 1,
            }),
            _ => {
                if let Some(last) = spans.last_mut() {
                    last.lines += 1;
                }
            }
        }
    }
    spans
}

/// A store's entries and, for `###` entries, each `##` section's own text before its first entry.
///
/// An entry runs from its heading to the next heading at its level or above, so a `###` entry ends
/// at the next `###` or `##`. The scan is [`heading_scan`]'s, so a fenced `###` is not a boundary.
/// Lines whose text is an exempt field — `**<Name>:**`, bare or as a `- ` list item — are not
/// counted toward their entry. Every line before the first heading at the entry level or above —
/// frontmatter, fences and a deeper heading included — is the preamble, counted in full and
/// returned as the third value.
fn entry_spans(body: &str, level: usize, exempt: &[String]) -> (Vec<Span>, Vec<Span>, usize) {
    enum Open {
        Nothing,
        Entry,
        Section,
    }
    let is_exempt = |line: &str| {
        let text = line.trim_start();
        let text = text.strip_prefix("- ").unwrap_or(text);
        exempt
            .iter()
            .any(|name| text.starts_with(&format!("**{name}:**")))
    };
    let mut entries: Vec<Span> = Vec::new();
    let mut sections: Vec<Span> = Vec::new();
    let mut preamble = 0;
    let mut open = Open::Nothing;
    for (line, classified) in body.lines().zip(heading_scan(body)) {
        match classified {
            Some((found, text)) if found == level => {
                entries.push(Span {
                    heading: text.to_string(),
                    lines: 1,
                });
                open = Open::Entry;
            }
            Some((found, text)) if found < level => {
                sections.push(Span {
                    heading: text.to_string(),
                    lines: 1,
                });
                open = Open::Section;
            }
            _ => match open {
                Open::Entry if !is_exempt(line) => {
                    if let Some(entry) = entries.last_mut() {
                        entry.lines += 1;
                    }
                }
                Open::Section => {
                    if let Some(section) = sections.last_mut() {
                        section.lines += 1;
                    }
                }
                Open::Nothing => preamble += 1,
                Open::Entry => {}
            },
        }
    }
    (entries, sections, preamble)
}

/// The heading text a placeholder token may not survive in — `##` and `###`, fences excluded.
fn heading_texts(body: &str) -> Vec<&str> {
    heading_scan(body)
        .into_iter()
        .flatten()
        .map(|(_, text)| text)
        .collect()
}

/// The frontmatter block's keys and their scalar values, or `None` when there is no block.
///
/// Only a leading `---` fence counts, which is what a frontmatter block is. Values are stringified
/// so an enum check compares text to text; a non-scalar value stringifies to its YAML form and will
/// simply not match an enum entry.
fn frontmatter(body: &str) -> Option<BTreeMap<String, String>> {
    let mut lines = body.lines();
    if lines.next()?.trim() != "---" {
        return None;
    }
    let mut block = String::new();
    let mut closed = false;
    for line in lines {
        if line.trim() == "---" {
            closed = true;
            break;
        }
        block.push_str(line);
        block.push('\n');
    }
    if !closed {
        return None;
    }
    let value: Value = serde_norway::from_str(&block).ok()?;
    let mapping = value.as_mapping()?;
    let mut out = BTreeMap::new();
    for (key, value) in mapping {
        let Some(key) = key.as_str() else { continue };
        out.insert(key.to_string(), scalar(value));
    }
    Some(out)
}

/// A YAML value as the text an enum check compares against.
fn scalar(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Number(number) => number.to_string(),
        Value::Bool(flag) => flag.to_string(),
        Value::Null => String::new(),
        other => serde_norway::to_string(other)
            .unwrap_or_default()
            .trim()
            .to_string(),
    }
}

/// Comma-separate a list in backticks, or the given word when it is empty.
fn list<S: AsRef<str>>(items: &[S], empty: &str) -> String {
    if items.is_empty() {
        return empty.to_string();
    }
    items
        .iter()
        .map(|item| format!("`{}`", item.as_ref()))
        .collect::<Vec<String>>()
        .join(", ")
}
