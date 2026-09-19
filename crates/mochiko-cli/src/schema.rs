//! The artifact-template model and its two guidance views.
//!
//! A template describes one pipeline artifact. The binary only renders it — it never grades an
//! artifact against it (the bright line, GI-019). The `--check` view is the mirror-checklist
//! rendering of the same template, not a linter.
//!
//! # Where a template comes from
//!
//! The migration log, and nowhere else. Until wave 1 the crate carried a closed set of eight
//! names, each with a compile-time embedded copy and a three-step file resolution behind it;
//! all of that left when the log became the source of truth (record D1, and D10.5 superseding
//! the raw-Read fallback). A template is now an opaque document in the replayed state, decoded
//! into this model at the point of use by [`crate::render::template_of`], so the set of names
//! the binary serves is data the log carries rather than a constant it ships.

use serde::Deserialize;
use std::collections::BTreeMap;

/// One pipeline artifact template. Core fields are always present; the `Option` fields are
/// populated only where the source template carries the content. Unknown/extra keys are ignored
/// rather than rejected, so a template can grow fields without breaking an older binary.
#[derive(Debug, Deserialize)]
pub struct Template {
    pub template: String,
    pub title: String,
    pub form: String,
    pub register: String,
    pub overview: String,
    pub sections: Vec<Section>,
    pub skeleton: String,
    /// The machine-checkable conformance block (record D4). **Absent means no shape check at
    /// all** — never an empty-but-present block, because a kind with no template shape must not
    /// have one invented from prose (D4f).
    #[serde(default)]
    pub conformance: Option<Conformance>,
}

/// One section of a template. `name`/`required`/`contract`/`check` are always present; the rest
/// appear only where the source carries them. `bad` and `severity` are currently unused by every
/// shipped template but must still deserialize if a future one adds them.
///
/// # Why `heading` is separate from `name`
///
/// A section's `name` is its guidance label, and it is **not** reliably the `##` heading the
/// produced artifact carries. Measured over all eight shipped templates at wave 1: `spec` maps 1:1,
/// `governance-intent` and `feature-entry` map but for a leading `## Header` meta section that is
/// no heading at all, `codebase-analysis` declares an em dash where its skeleton writes a colon,
/// `architecture-store`'s sections name whole files, and `features-index`, `governance-surfaces`
/// and `tasks` skeleton no complete heading set. So the heading a section governs is declared, not
/// inferred; a section with no `heading` governs none.
#[derive(Debug, Deserialize)]
pub struct Section {
    pub name: String,
    pub required: bool,
    pub contract: String,
    pub check: String,
    /// The literal `##` heading text this section governs. Absent means it governs no heading.
    #[serde(default)]
    pub heading: Option<String>,
    /// The section's line budget, counted from its `##` line to the next `##` (record D6). Absent
    /// is the OQ1 no-budget disclosure, not a budget of zero.
    #[serde(default)]
    pub max_lines: Option<usize>,
    #[serde(default)]
    pub density: Option<String>,
    #[serde(default)]
    pub good: Option<String>,
    #[serde(default)]
    pub bad: Option<String>,
    #[serde(default)]
    pub severity: Option<String>,
}

/// The conformance block: what a binary can check about a produced artifact by string and count.
///
/// Every field here is decidable without reading meaning, which is what keeps the write-time gate
/// on the admitted side of the bright line (GI-019). The heading grammar is not repeated here — it
/// comes off [`Section::heading`] in declaration order.
#[derive(Debug, Default, Deserialize)]
pub struct Conformance {
    #[serde(default)]
    pub frontmatter: Frontmatter,
    /// The exact token spellings that must not survive into the produced artifact, checked in
    /// frontmatter values and heading text only (record D4c).
    ///
    /// Spellings are enumerated, never derived: the shipped corpus uses `{{token}}`, `[token]` and
    /// `<token>` in different templates, and `tasks` carries `[P]` as real cycle-card syntax beside
    /// genuine `[N]` placeholders — a pattern-shaped rule would deny a conforming file.
    #[serde(default)]
    pub placeholders: Vec<String>,
    /// `deny` (the D4a default) or `allow`. An unrecognised value reads as the default, because the
    /// stricter reading is the safe one.
    #[serde(default)]
    pub extra_headings: Option<String>,
}

/// The frontmatter contract: which keys must be present, and which hold an enumerated value.
#[derive(Debug, Default, Deserialize)]
pub struct Frontmatter {
    #[serde(default)]
    pub required: Vec<String>,
    /// Key to its allowed values. Named `enums` in Rust because `enum` is a keyword; the log
    /// writes `enum:`.
    #[serde(default, rename = "enum")]
    pub enums: BTreeMap<String, Vec<String>>,
}

impl Conformance {
    /// Whether an undeclared `##` heading is a denial. The default is `deny` (record D4a).
    pub fn denies_extra_headings(&self) -> bool {
        !matches!(self.extra_headings.as_deref(), Some("allow"))
    }
}

impl Template {
    /// The headings this template declares, in declaration order, each with its required flag.
    ///
    /// Sections that govern no heading are skipped, so this is the artifact's heading grammar
    /// rather than the guidance outline.
    pub fn declared_headings(&self) -> impl Iterator<Item = (&str, bool)> {
        self.sections
            .iter()
            .filter_map(|s| s.heading.as_deref().map(|h| (h, s.required)))
    }

    /// The headings a conforming artifact must carry, in declaration order.
    pub fn required_headings(&self) -> Vec<&str> {
        self.declared_headings()
            .filter(|(_, required)| *required)
            .map(|(heading, _)| heading)
            .collect()
    }

    /// The line budget for one heading, or `None` when it declares no budget.
    pub fn budget(&self, heading: &str) -> Option<usize> {
        self.sections
            .iter()
            .find(|s| s.heading.as_deref() == Some(heading))
            .and_then(|s| s.max_lines)
    }

    /// The producer view: the guidance a seat reads while authoring the artifact — title,
    /// overview, per-section contract (plus density/good/bad where present), and the fill-in
    /// skeleton, closed by the source line.
    pub fn producer_view(&self, source: &str) -> String {
        let mut out = String::new();
        out.push_str(&format!("# {}\n\n", self.title));
        out.push_str(self.overview.trim());
        out.push_str("\n\n");

        for section in &self.sections {
            let optional = if section.required {
                ""
            } else {
                " — optional"
            };
            out.push_str(&format!("## {}{}\n\n", section.name, optional));
            out.push_str(section.contract.trim());
            out.push('\n');

            if let Some(density) = &section.density {
                out.push_str(&format!("\n_Density:_ {}\n", density.trim()));
            }
            if let Some(good) = &section.good {
                out.push_str("\nGood example:\n");
                out.push_str(good.trim_end());
                out.push('\n');
            }
            if let Some(bad) = &section.bad {
                out.push_str("\nBad example:\n");
                out.push_str(bad.trim_end());
                out.push('\n');
            }
            out.push('\n');
        }

        if let Some(conformance) = &self.conformance {
            out.push_str(&self.conformance_block(conformance));
        }

        out.push_str("## Skeleton\n\n");
        out.push_str(self.skeleton.trim());
        out.push_str("\n\n---\n");
        out.push_str(source);
        out.push('\n');
        out
    }

    /// The conformance block as the producer view prints it — the budgets a seat reads *before*
    /// drafting a line (record D1a).
    ///
    /// This is the authoring-time channel: at `PreToolUse` the content already exists, so a deny
    /// bounds persistence rather than generation cost. What shapes the draft is this block.
    fn conformance_block(&self, conformance: &Conformance) -> String {
        let mut out = String::from("## Conformance\n\n");
        out.push_str(
            "Checked mechanically before the write lands. Each line is a string or a count, never \
             a judgment.\n\n",
        );

        let frontmatter = &conformance.frontmatter;
        if !frontmatter.required.is_empty() {
            out.push_str(&format!(
                "- Required frontmatter: {}\n",
                backticked(&frontmatter.required)
            ));
        }
        for (key, values) in &frontmatter.enums {
            out.push_str(&format!(
                "- `{key}` is one of: {}\n",
                backticked(values.as_slice())
            ));
        }

        let required: Vec<&str> = self.required_headings();
        if !required.is_empty() {
            let headings: Vec<String> = required.iter().map(|h| format!("## {h}")).collect();
            out.push_str(&format!(
                "- Required headings, in order: {}\n",
                backticked(&headings)
            ));
        }
        if conformance.denies_extra_headings() {
            out.push_str("- An undeclared `##` heading is denied. `###` and deeper are yours.\n");
        }

        let budgeted: Vec<&Section> = self
            .sections
            .iter()
            .filter(|s| s.heading.is_some() && s.max_lines.is_some())
            .collect();
        if !budgeted.is_empty() {
            out.push_str(
                "- Line budgets, each counted from its `##` line to the next `##`, nested \
                 content included:\n",
            );
            for section in budgeted {
                let heading = section.heading.as_deref().unwrap_or(&section.name);
                let lines = section.max_lines.unwrap_or_default();
                let optional = if section.required { "" } else { " (optional)" };
                out.push_str(&format!("  - `## {heading}` — {lines} lines{optional}\n"));
            }
        }

        if !conformance.placeholders.is_empty() {
            out.push_str(&format!(
                "- Placeholder tokens that must not survive: {} — checked in frontmatter values \
                 and heading text, by exact spelling.\n",
                backticked(&conformance.placeholders)
            ));
        }

        out.push('\n');
        out
    }

    /// The checklist view: the mirror-checklist rendering of the same template — one check line
    /// per section (with severity where present), closed by the source line. It takes no artifact
    /// input and is advisory only; it is a view, never a linter.
    pub fn check_view(&self, source: &str) -> String {
        let mut out = String::new();
        out.push_str(&format!("# {} — checklist\n\n", self.title));
        for section in &self.sections {
            let severity = match &section.severity {
                Some(value) => format!("  ·  severity: {}", value.trim()),
                None => String::new(),
            };
            out.push_str(&format!("- [ ] {}{}\n", section.check.trim(), severity));
        }
        out.push_str("\n---\n");
        out.push_str(source);
        out.push('\n');
        out
    }
}

/// Join items as an inline, comma-separated, backticked list.
fn backticked<S: AsRef<str>>(items: &[S]) -> String {
    items
        .iter()
        .map(|item| format!("`{}`", item.as_ref()))
        .collect::<Vec<String>>()
        .join(", ")
}
