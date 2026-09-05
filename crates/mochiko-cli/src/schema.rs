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

/// One pipeline artifact template. Core fields are always present; the four `Option` fields are
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
}

/// One section of a template. `name`/`required`/`contract`/`check` are always present; the rest
/// appear only where the source carries them. `bad` and `severity` are currently unused by every
/// shipped template but must still deserialize if a future one adds them.
#[derive(Debug, Deserialize)]
pub struct Section {
    pub name: String,
    pub required: bool,
    pub contract: String,
    pub check: String,
    #[serde(default)]
    pub density: Option<String>,
    #[serde(default)]
    pub good: Option<String>,
    #[serde(default)]
    pub bad: Option<String>,
    #[serde(default)]
    pub severity: Option<String>,
}

impl Template {
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

        out.push_str("## Skeleton\n\n");
        out.push_str(self.skeleton.trim());
        out.push_str("\n\n---\n");
        out.push_str(source);
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
