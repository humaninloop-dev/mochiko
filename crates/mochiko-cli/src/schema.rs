//! Schema model, schema-source resolution, and the two guidance views.
//!
//! A schema file is the single source of truth for one pipeline artifact template. The binary
//! only renders it — it never grades an artifact against it (the D11 bright line). The `--check`
//! view is the mirror-checklist rendering of the same schema, not a linter.

use serde::Deserialize;
use std::fs;
use std::path::Path;

/// The schema-backed artifact families the binary renders: the in-scope pipeline artifact
/// templates (D3) plus the product architecture store (product-architecture-schema D8). This is the
/// authoritative known-name set: a `template <name>` for a name outside this list is an unknown
/// template (exit 2). Every name here has an embedded compile-time copy via [`embedded`], so the
/// embedded fallback can never miss a known name.
///
/// `plan` left the set at v0.91.0: the plan-stage retirement ruled the `plan.md` artifact dead
/// outright, so its schema was deleted rather than superseded
/// (`.mochiko/brainstorms/plan-stage-utility/record.md` D1/D4).
///
/// Shelf data files (`architecture-shelf-*.yaml`) are deliberately absent: they are dealt by a
/// skill reading them raw, never rendered here.
pub const TEMPLATE_NAMES: [&str; 8] = [
    "spec",
    "tasks",
    "feature-entry",
    "features-index",
    "codebase-analysis",
    "governance-intent",
    "governance-surfaces",
    "architecture-store",
];

/// One pipeline artifact template. Core fields are always present; the four `Option` fields are
/// populated only where the source template carries the content (I3). Unknown/extra YAML keys are
/// ignored rather than rejected, so a schema can grow fields without breaking an older binary.
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
/// shipped schema but must still deserialize if a future schema adds them.
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

/// A resolved schema: its raw YAML and a human-readable source line for the view footer.
pub struct Resolved {
    pub yaml: String,
    pub source: String,
}

/// Why schema resolution failed.
pub enum ResolveError {
    /// Name is not one of [`TEMPLATE_NAMES`].
    UnknownTemplate(String),
    /// An explicit `--schemas-dir` was given but the file could not be read.
    ReadFailed { path: String, err: String },
}

/// The compile-time embedded copy of a schema, keyed by name.
///
/// Embedding is anchored at `CARGO_MANIFEST_DIR` (the crate root) rather than the source file, so
/// the path resolves to the repo's `plugins/mochiko/schemas/` regardless of which module includes
/// it (a bare relative `include_str!` would resolve against `src/` and miss). The embedded copy is
/// the run-from-anywhere last resort; the shipped `.yaml` files are the source of truth when present.
fn embedded(name: &str) -> Option<&'static str> {
    let content = match name {
        "spec" => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../plugins/mochiko/schemas/spec.yaml"
        )),
        "tasks" => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../plugins/mochiko/schemas/tasks.yaml"
        )),
        "feature-entry" => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../plugins/mochiko/schemas/feature-entry.yaml"
        )),
        "features-index" => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../plugins/mochiko/schemas/features-index.yaml"
        )),
        "codebase-analysis" => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../plugins/mochiko/schemas/codebase-analysis.yaml"
        )),
        "governance-intent" => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../plugins/mochiko/schemas/governance-intent.yaml"
        )),
        "governance-surfaces" => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../plugins/mochiko/schemas/governance-surfaces.yaml"
        )),
        "architecture-store" => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../plugins/mochiko/schemas/architecture-store.yaml"
        )),
        _ => return None,
    };
    Some(content)
}

/// Resolve a template name to its schema YAML, in priority order:
///   1. `--schemas-dir <path>` when given (authoritative — an unreadable file there is an error,
///      never a silent fall-through to a possibly-stale embedded copy);
///   2. `plugins/mochiko/schemas/` relative to the current directory, when that file exists;
///   3. the compile-time embedded copy.
///
/// The source line reports which path (or `embedded`) actually served the schema.
pub fn resolve(name: &str, schemas_dir: Option<&str>) -> Result<Resolved, ResolveError> {
    if let Some(dir) = schemas_dir {
        let path = Path::new(dir).join(format!("{name}.yaml"));
        return match fs::read_to_string(&path) {
            Ok(yaml) => Ok(Resolved {
                yaml,
                source: format!("schemas: {}", path.display()),
            }),
            Err(err) => Err(ResolveError::ReadFailed {
                path: path.display().to_string(),
                err: err.to_string(),
            }),
        };
    }

    // Default resolution only serves known templates; the embedded copy guarantees every known
    // name is available even when run from outside the repo.
    if embedded(name).is_none() {
        return Err(ResolveError::UnknownTemplate(name.to_string()));
    }

    let cwd_path = Path::new("plugins/mochiko/schemas").join(format!("{name}.yaml"));
    if cwd_path.is_file() {
        if let Ok(yaml) = fs::read_to_string(&cwd_path) {
            return Ok(Resolved {
                yaml,
                source: format!("schemas: {}", cwd_path.display()),
            });
        }
    }

    Ok(Resolved {
        yaml: embedded(name)
            .expect("known template has an embedded copy")
            .to_string(),
        source: "schemas: embedded".to_string(),
    })
}

/// Parse schema YAML into a [`Template`].
pub fn parse(yaml: &str) -> Result<Template, serde_norway::Error> {
    serde_norway::from_str(yaml)
}

impl Template {
    /// The producer view: the guidance a seat reads while authoring the artifact — title,
    /// overview, per-section contract (plus density/good/bad where present), and the fill-in
    /// skeleton, closed by the schema-source line.
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

    /// The checklist view: the mirror-checklist rendering of the same schema — one check line per
    /// section (with severity where present), closed by the schema-source line. It takes no
    /// artifact input and is advisory only; it is a view, never a linter (D11 bright line).
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
