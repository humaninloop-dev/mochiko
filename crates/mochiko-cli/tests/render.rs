//! Integration tests for the schema model, resolution, and the two views.
//!
//! `cargo test` sets the working directory to the package root (`crates/mochiko-cli`), where a
//! relative `plugins/mochiko/schemas/` does not exist — so `resolve(name, None)` deterministically
//! falls through to the embedded copy. The shipped `.yaml` files are reached via an absolute
//! `--schemas-dir` anchored at `CARGO_MANIFEST_DIR`.

use mochiko_cli::schema::{self, TEMPLATE_NAMES};

/// Absolute path to the shipped schema directory, anchored at the crate root so it is independent
/// of the test process's working directory.
const SHIPPED_SCHEMAS_DIR: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../plugins/mochiko/schemas");

fn resolve_embedded(name: &str) -> schema::Resolved {
    schema::resolve(name, None).unwrap_or_else(|_| panic!("embedded resolution failed for {name}"))
}

#[test]
fn every_embedded_schema_parses() {
    for name in TEMPLATE_NAMES {
        let resolved = resolve_embedded(name);
        assert_eq!(
            resolved.source, "schemas: embedded",
            "{name} should resolve to the embedded copy under cargo test"
        );
        let template =
            schema::parse(&resolved.yaml).unwrap_or_else(|e| panic!("{name} failed to parse: {e}"));
        assert_eq!(
            template.template, name,
            "{name}: template field should match the file name"
        );
        assert!(
            !template.sections.is_empty(),
            "{name}: should carry at least one section"
        );
        for section in &template.sections {
            assert!(
                !section.name.trim().is_empty(),
                "{name}: a section is missing its name"
            );
            assert!(
                !section.check.trim().is_empty(),
                "{name}: section '{}' is missing its check line",
                section.name
            );
        }
    }
}

#[test]
fn every_shipped_schema_parses() {
    for name in TEMPLATE_NAMES {
        let resolved = schema::resolve(name, Some(SHIPPED_SCHEMAS_DIR))
            .unwrap_or_else(|_| panic!("shipped resolution failed for {name}"));
        assert!(
            resolved.source.contains("plugins/mochiko/schemas"),
            "{name}: source line should name the shipped dir, got {}",
            resolved.source
        );
        assert!(
            resolved.source.contains(&format!("{name}.yaml")),
            "{name}: source line should name the schema file, got {}",
            resolved.source
        );
        schema::parse(&resolved.yaml)
            .unwrap_or_else(|e| panic!("{name} (shipped) failed to parse: {e}"));
    }
}

#[test]
fn producer_view_carries_every_section_name_and_the_skeleton() {
    for name in TEMPLATE_NAMES {
        let resolved = resolve_embedded(name);
        let template = schema::parse(&resolved.yaml).unwrap();
        let view = template.producer_view(&resolved.source);

        assert!(
            view.starts_with(&format!("# {}", template.title)),
            "{name}: producer view should open with the title"
        );
        for section in &template.sections {
            assert!(
                view.contains(&section.name),
                "{name}: producer view is missing section '{}'",
                section.name
            );
        }
        assert!(
            view.contains(template.skeleton.trim()),
            "{name}: producer view is missing the skeleton content"
        );
        assert!(
            view.trim_end().ends_with(&resolved.source),
            "{name}: producer view should close with the source line"
        );
    }
}

#[test]
fn check_view_has_exactly_one_line_per_section() {
    for name in TEMPLATE_NAMES {
        let resolved = resolve_embedded(name);
        let template = schema::parse(&resolved.yaml).unwrap();
        let view = template.check_view(&resolved.source);

        let check_lines = view
            .lines()
            .filter(|line| line.starts_with("- [ ] "))
            .count();
        assert_eq!(
            check_lines,
            template.sections.len(),
            "{name}: expected one check line per section"
        );
        assert!(
            view.trim_end().ends_with(&resolved.source),
            "{name}: check view should close with the source line"
        );
    }
}

#[test]
fn optional_section_fields_render_when_present() {
    // spec's Screens & Flows carries a `good` example; feature-entry's Capability carries `density`.
    let spec = schema::parse(&resolve_embedded("spec").yaml).unwrap();
    let spec_view = spec.producer_view("schemas: embedded");
    assert!(
        spec_view.contains("Good example:"),
        "spec producer view should render the good example"
    );

    let entry = schema::parse(&resolve_embedded("feature-entry").yaml).unwrap();
    let entry_view = entry.producer_view("schemas: embedded");
    assert!(
        entry_view.contains("_Density:_"),
        "feature-entry producer view should render density notes"
    );

    // nfrs's NFR statement-format section carries a `bad` counter-example — the only shipped
    // schema that does, so this is the one place the `bad` render path is exercised.
    let nfrs = schema::parse(&resolve_embedded("nfrs").yaml).unwrap();
    let nfrs_view = nfrs.producer_view("schemas: embedded");
    assert!(
        nfrs_view.contains("Bad example:"),
        "nfrs producer view should render the bad example"
    );
}

// --- dispatch / exit codes ---

fn args(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

#[test]
fn unknown_template_exits_2() {
    assert_eq!(mochiko_cli::run(&args(&["template", "does-not-exist"])), 2);
}

#[test]
fn unknown_command_exits_2() {
    assert_eq!(mochiko_cli::run(&args(&["frobnicate"])), 2);
}

#[test]
fn template_without_a_name_exits_2() {
    assert_eq!(mochiko_cli::run(&args(&["template"])), 2);
}

#[test]
fn no_args_and_help_exit_0() {
    assert_eq!(mochiko_cli::run(&[]), 0);
    assert_eq!(mochiko_cli::run(&args(&["--help"])), 0);
    assert_eq!(mochiko_cli::run(&args(&["-h"])), 0);
}

#[test]
fn producer_and_check_views_exit_0() {
    assert_eq!(mochiko_cli::run(&args(&["template", "spec"])), 0);
    assert_eq!(mochiko_cli::run(&args(&["template", "spec", "--check"])), 0);
}

#[test]
fn schemas_dir_override_exits_0() {
    assert_eq!(
        mochiko_cli::run(&args(&[
            "template",
            "plan",
            "--schemas-dir",
            SHIPPED_SCHEMAS_DIR
        ])),
        0
    );
}
