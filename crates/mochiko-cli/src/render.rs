//! The delivery render: what a command or skill's `.md` receives at fire.
//!
//! One render is one schema section (D3 as amended). A whole-primitive render does not exist:
//! the Bash tool's inline ceiling is a platform fact, and an oversized render arrives as a
//! file-path preview whose head line survives — so a head-only confirmation would pass a
//! truncated delivery. Every render therefore opens with the version-triple line and closes with
//! an end line naming the rule count, and the `.md` halts unless it sees both.
//!
//! The render never grades anything. It resolves what the log already says — `extends:` through
//! the validator's own implementation, `${var}` through the validator's own scanner — and prints
//! it. Maintainer metadata (a rule's ruling `anchor:`, a rule's authoring `note:`) is excluded:
//! the shipped log carries runtime content alone (record D2, the D16 posture).

use crate::model::{ordered_get, Condition, DocKind, DocRef, Ordered, Rule, RuleSchema, WhenValue};
use crate::replay::State;
use crate::schema::Template;
use crate::validate::{self, Family};
use serde_norway::Value;
use std::collections::BTreeSet;
use std::fmt;
use std::path::Path;

/// The section id that renders a schema's identity, bindings and pins rather than a rule set.
pub const PREAMBLE: &str = "preamble";

/// The reading grammar every preamble carries, fixed text rather than anything derived from
/// state (wave-3 plan §2). A converted primitive's `.md` points at this block by name instead of
/// restating what `class:`, `when:` or `pointer:` mean, so the render is the only home the
/// grammar has. Golden-tested in `tests/render.rs`, byte size included.
///
/// Widened at wave 4 with the last three lines: converting the remaining five commands found the
/// old Rules block teaching `labels:`, `moments:` and the empty-`enforces:` reason, none of which
/// the legend said. A `.md` that stops restating the grammar must be able to point at all of it.
/// The `moments:` line is delivered to skills too, which declare none — it reads as grammar they
/// will not meet, which is cheaper than a second legend to maintain.
const LEGEND: &str = "\nlegend\n\
- class: floor is always delivered whatever its when:; when: gates when the obligation applies, never whether it reaches you.\n\
- kind: names what a rule is — constraint (the default) · duty · gate · reservation · binding · bound · routing · fail · latitude.\n\
- when: binds a rule only where its terms hold against the conditions block above.\n\
- enforces: on a kind: fail rule names the rules it is the end-state contrapositive of.\n\
- pointer: binds you to that skill's procedure — referenced, never restated.\n\
- extends: is already resolved in this render; the rule's own id stays the citable id.\n\
- labels: cross-reference tags from the labels registry; they bind nothing on their own.\n\
- moments: the run's anchor points, unordered — never a sequence.\n\
- enforces: an empty list on a kind: fail rule carries its one-line reason.\n";

/// The version triple a render announces itself with: the binary's version, the log's grammar,
/// and the plugin's version (`unknown` when no plugin root resolved).
pub struct Context {
    pub binary: String,
    pub grammar: u32,
    pub plugin: String,
}

/// Why a render could not be produced. Every arm is a name the log does not carry, which is a
/// usage error (exit 2) rather than a delivery failure.
#[derive(Debug)]
pub enum RenderError {
    UnknownPrimitive(String),
    /// The log carries the name as both a command and a skill.
    AmbiguousPrimitive(String),
    UnknownSection {
        primitive: String,
        section: String,
        available: Vec<String>,
    },
    UnknownTemplate(String),
    TemplateUndecodable {
        name: String,
        message: String,
    },
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::UnknownPrimitive(name) => write!(
                f,
                "no command or skill named '{name}' in the log — check the name against \
                 `mochiko-cli migrate status`"
            ),
            RenderError::AmbiguousPrimitive(name) => write!(
                f,
                "ambiguous: the log carries both a command and a skill named '{name}' — the two \
                 name sets are meant to be disjoint, so this is a defect in the log, not in the \
                 request"
            ),
            RenderError::UnknownSection {
                primitive,
                section,
                available,
            } => write!(
                f,
                "'{primitive}' carries no section '{section}'\n\navailable: {} {}",
                PREAMBLE,
                available.join(" ")
            ),
            RenderError::UnknownTemplate(name) => {
                write!(f, "no template named '{name}' in the log")
            }
            RenderError::TemplateUndecodable { name, message } => {
                write!(
                    f,
                    "the log's '{name}' document is not a template: {message}"
                )
            }
        }
    }
}

impl std::error::Error for RenderError {}

// ---------------------------------------------------------------------------
// the rules render
// ---------------------------------------------------------------------------

/// The preamble: the schema's identity, its resolved bindings, its count pins, the floor index
/// those pins name, and its section list — everything a reader needs before asking for a section.
/// Its rule count is always zero.
pub fn preamble(state: &State, doc: &DocRef, ctx: &Context) -> Result<String, RenderError> {
    let schema = rule_schema(state, doc)?;
    let is_command = doc.kind == DocKind::Command;
    let mut body = String::new();

    body.push_str(&format!("{} {}\n", doc.kind, doc.name));

    if !schema.vars.is_empty() {
        body.push_str("\nvars\n");
        for (name, value) in &schema.vars {
            body.push_str(&format!("- {name} = {}\n", scalar(value)));
        }
    }

    if !schema.conditions.is_empty() {
        body.push_str("\nconditions\n");
        for (name, condition) in &schema.conditions {
            body.push_str(&format!("- {name}{}\n", condition_line(condition, schema)));
        }
    }

    if is_command && !schema.moments.is_empty() {
        body.push_str("\nmoments\n");
        for (name, text) in &schema.moments {
            body.push_str(&format!("- {name}: {}\n", substitute(text, &schema.vars)));
        }
    }

    body.push_str("\npins\n");
    if is_command {
        let fails = schema.rules().filter(|r| r.is_fail()).count();
        body.push_str(&format!("- kind: fail · {fails} rules\n"));
    }
    let floors: Vec<&str> = schema
        .rules()
        .filter(|rule| rule.is_floor())
        .map(|rule| rule.id.as_str())
        .collect();
    body.push_str(&format!("- class: floor · {} rules\n", floors.len()));

    // The floor index (wave-5 plan §2). The pin's number alone leaves a converted `.md` naming a
    // count it cannot check itself against, so the ids come out beside it — from the same
    // iterator, which is what makes the two incapable of disagreeing. `rules()` walks sections in
    // declared order and rules in section order, so this is render order and the read-back can
    // cite position as well as membership.
    let index = if floors.is_empty() {
        "none".to_string()
    } else {
        floors.join(" · ")
    };
    body.push_str(&format!("\nfloors: {index}\n"));

    body.push_str(LEGEND);

    body.push_str("\nsections\n");
    for section in &schema.sections {
        body.push_str(&format!(
            "- {} · {} · {} rules\n",
            section.id,
            section.title,
            section.rules.len()
        ));
    }

    Ok(wrap(&doc.name, PREAMBLE, body.trim_end(), 0, ctx))
}

/// One section: its title, its intent, and one block per live rule. Tombstoned ids are gone from
/// the state's sections already, so a rule that ever left is simply absent.
pub fn section(
    state: &State,
    doc: &DocRef,
    section_id: &str,
    ctx: &Context,
) -> Result<String, RenderError> {
    if section_id == PREAMBLE {
        return preamble(state, doc, ctx);
    }
    let schema = rule_schema(state, doc)?;
    let Some(section) = schema.find_section(section_id) else {
        return Err(RenderError::UnknownSection {
            primitive: doc.name.clone(),
            section: section_id.to_string(),
            available: schema.sections.iter().map(|s| s.id.clone()).collect(),
        });
    };

    let family = (doc.kind == DocKind::Skill).then(|| Family::of(&doc.name));
    let mut body = format!("## {}\n", section.title);
    if !section.intent.trim().is_empty() {
        body.push_str(section.intent.trim_end());
        body.push('\n');
    }

    if section.rules.is_empty() {
        // A deliberately empty section states why in its note; without one there is simply
        // nothing under the heading.
        if let Some(note) = section.note.as_ref().filter(|n| !n.trim().is_empty()) {
            body.push_str(&format!(
                "\nnote: {}\n",
                substitute(note.trim(), &schema.vars)
            ));
        }
    } else {
        for rule in &section.rules {
            body.push('\n');
            body.push_str(&rule_block(state, doc, schema, rule, family));
        }
    }

    Ok(wrap(
        &doc.name,
        section_id,
        body.trim_end(),
        section.rules.len(),
        ctx,
    ))
}

/// The head and end lines D3 keys its halt clause on, wrapped around a body.
fn wrap(primitive: &str, section: &str, body: &str, rules: usize, ctx: &Context) -> String {
    format!(
        "mochiko-cli rules {primitive} · section {section} · binary {} · grammar {} · plugin {}\n\
         \n{body}\n\n\
         mochiko-cli rules end · {primitive} · {section} · {rules} rules\n",
        ctx.binary, ctx.grammar, ctx.plugin
    )
}

/// One rule: its id, its bracket line, its resolved text, and — for a fail node — the rules it
/// mirrors. `enforces:` is printed only where it carries meaning, which is the fail set.
fn rule_block(
    state: &State,
    doc: &DocRef,
    schema: &RuleSchema,
    rule: &Rule,
    family: Option<Family>,
) -> String {
    // The validator's own resolution, with its findings discarded: a state that reached the
    // delivery path already passed the hard set, so there is nothing left to report.
    let mut sink = Vec::new();
    let resolved = validate::resolve_extends(state, doc, rule, family, &mut sink);

    let mut fields = vec![format!("class: {}", rule.class.as_deref().unwrap_or("-"))];
    if rule.effective_kind() != crate::model::RuleKind::Constraint {
        if let Some(kind) = &rule.kind {
            fields.push(format!("kind: {kind}"));
        }
    }
    if !rule.when.is_empty() {
        fields.push(format!("when: {}", when_terms(&rule.when)));
    }
    if let Some(labels) = resolved.labels.filter(|l| !l.is_empty()) {
        fields.push(format!("labels: {}", labels.join(", ")));
    }
    if let Some(pointer) = resolved.pointer.filter(|p| !p.trim().is_empty()) {
        fields.push(format!("pointer: {}", pointer.trim()));
    }

    let mut out = format!("### {}\n[{}]\n", rule.id, fields.join(" · "));
    if let Some(text) = resolved.text.filter(|t| !t.trim().is_empty()) {
        out.push_str(substitute(text.trim_end(), &schema.vars).trim_end());
        out.push('\n');
    }
    // An explicitly empty mirror (`enforces: []`, legal beside a `note:`) renders no key at all.
    // The note that says why it is empty is maintainer metadata this render excludes, so the key
    // would carry nothing; a fail node mirroring no local rule says that by the line's absence.
    if rule.is_fail() {
        if let Some(enforces) = rule.enforces.as_deref().filter(|ids| !ids.is_empty()) {
            out.push_str(&format!("enforces: {}\n", enforces.join(", ")));
        }
    }
    out
}

/// A `when:` map as `<dimension>=<value>` terms, a list value joined by `|`.
fn when_terms(when: &Ordered<WhenValue>) -> String {
    when.iter()
        .map(|(dimension, value)| match value {
            WhenValue::Scalar(v) => format!("{dimension}={}", scalar(v)),
            WhenValue::List(items) => {
                let joined: Vec<String> = items.iter().map(scalar).collect();
                format!("{dimension}={}", joined.join("|"))
            }
        })
        .collect::<Vec<String>>()
        .join(", ")
}

/// A condition's declared values, resolution point and note, each present only where the schema
/// carries it.
fn condition_line(condition: &Condition, schema: &RuleSchema) -> String {
    let mut out = String::new();
    if let Some(values) = &condition.values {
        let rendered = match values {
            Value::Sequence(items) => items.iter().map(scalar).collect::<Vec<String>>().join("|"),
            other => scalar(other),
        };
        out.push_str(&format!(" · values: {rendered}"));
    }
    if let Some(resolution) = condition
        .resolution
        .as_ref()
        .filter(|r| !r.trim().is_empty())
    {
        out.push_str(&format!(" · resolution: {}", resolution.trim()));
    }
    if let Some(note) = condition.note.as_ref().filter(|n| !n.trim().is_empty()) {
        out.push_str(&format!(
            " · note: {}",
            substitute(note.trim(), &schema.vars)
        ));
    }
    out
}

/// The rule-bearing document behind a primitive name.
fn rule_schema<'a>(state: &'a State, doc: &DocRef) -> Result<&'a RuleSchema, RenderError> {
    state
        .docs
        .get(doc)
        .and_then(|document| document.as_rules())
        .ok_or_else(|| RenderError::UnknownPrimitive(doc.name.clone()))
}

/// Substitute every `${name}` the schema binds, using the validator's own placeholder grammar so
/// the render can never disagree with the closure check about what a placeholder is.
///
/// An unbound placeholder is left standing rather than blanked: the hard set rejects one, so its
/// appearance in a render would be evidence of a defect, not something to hide.
fn substitute(text: &str, vars: &Ordered<Value>) -> String {
    let names: BTreeSet<String> = validate::placeholders(text).into_iter().collect();
    let mut out = text.to_string();
    for name in names {
        if let Some(value) = ordered_get(vars, &name) {
            out = out.replace(&format!("${{{name}}}"), &scalar(value));
        }
    }
    out
}

/// A YAML scalar as the text a render shows for it.
fn scalar(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Bool(flag) => flag.to_string(),
        Value::Number(number) => number.to_string(),
        Value::Null => String::new(),
        other => serde_norway::to_string(other)
            .unwrap_or_default()
            .trim_end()
            .to_string(),
    }
}

// ---------------------------------------------------------------------------
// the template views
// ---------------------------------------------------------------------------

/// Decode a template document out of the replayed state.
///
/// Templates are carried through the log opaquely — they have no grammar of their own — so they
/// are re-based into the typed model here, at the point of use.
pub fn template_of(state: &State, name: &str) -> Result<Template, RenderError> {
    let doc = DocRef::new(DocKind::Template, name);
    let Some(crate::model::Document::Opaque(value)) = state.docs.get(&doc) else {
        return Err(RenderError::UnknownTemplate(name.to_string()));
    };
    serde_norway::from_value::<Template>(value.clone()).map_err(|e| {
        RenderError::TemplateUndecodable {
            name: name.to_string(),
            message: e.to_string(),
        }
    })
}

/// The producer view, or the checklist view under `check`.
///
/// The rendering itself is unchanged from the file-sourced binary; only the provenance footer
/// moved, because the schema files are no longer where a template comes from.
pub fn template_view(
    state: &State,
    name: &str,
    check: bool,
    log_dir: &Path,
) -> Result<String, RenderError> {
    let template = template_of(state, name)?;
    let source = format!("schemas: replayed from {}", log_dir.display());
    Ok(if check {
        template.check_view(&source)
    } else {
        template.producer_view(&source)
    })
}
