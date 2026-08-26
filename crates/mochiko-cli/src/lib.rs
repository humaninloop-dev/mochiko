//! `mochiko-cli` — the template-schema guidance renderer.
//!
//! The foundation-seed CLI admitted by the D11 ruling. It renders schema files into two guidance
//! views and does nothing else: it never gates pipeline progress, dispatches agents, or grades an
//! artifact (the standing bright line, GI-019). The library exposes [`run`] (arg dispatch, returns
//! a process exit code) and the [`schema`] module (model + views) so both the binary and the
//! integration tests drive the same code path.

pub mod schema;

pub const USAGE: &str = "\
mochiko-cli — template-schema guidance renderer

USAGE:
    mochiko-cli template <name>            Producer view: guidance for authoring the artifact
    mochiko-cli template <name> --check    Checklist view: one check line per section

OPTIONS:
    --schemas-dir <path>   Read schemas from <path> instead of the default resolution
    -h, --help             Show this help

TEMPLATES:
    spec  tasks  feature-entry  features-index  codebase-analysis
    governance-intent  governance-surfaces  architecture-store

Schema source resolves in order: --schemas-dir, then ./plugins/mochiko/schemas/, then the
embedded copy. The --check view is a guidance VIEW, never a linter: it takes no artifact input
and always exits 0 on success.
";

/// Dispatch parsed CLI arguments (already stripped of `argv[0]`) and return a process exit code.
/// Kept in the library so integration tests can assert exit codes without spawning the binary.
pub fn run(args: &[String]) -> i32 {
    match args.first().map(String::as_str) {
        None | Some("-h") | Some("--help") => {
            print!("{USAGE}");
            0
        }
        Some("template") => run_template(&args[1..]),
        Some(other) => {
            eprintln!("error: unknown command '{other}'\n\n{USAGE}");
            2
        }
    }
}

fn run_template(args: &[String]) -> i32 {
    let mut name: Option<&str> = None;
    let mut check = false;
    let mut schemas_dir: Option<&str> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--check" => check = true,
            "--schemas-dir" => {
                i += 1;
                match args.get(i) {
                    Some(dir) => schemas_dir = Some(dir.as_str()),
                    None => {
                        eprintln!("error: --schemas-dir requires a <path>");
                        return 2;
                    }
                }
            }
            flag if flag.starts_with('-') => {
                eprintln!("error: unknown flag '{flag}'\n\n{USAGE}");
                return 2;
            }
            positional => {
                if name.is_some() {
                    eprintln!("error: unexpected argument '{positional}'\n\n{USAGE}");
                    return 2;
                }
                name = Some(positional);
            }
        }
        i += 1;
    }

    let name = match name {
        Some(name) => name,
        None => {
            eprintln!("error: 'template' requires a <name>\n\n{USAGE}");
            return 2;
        }
    };

    let resolved = match schema::resolve(name, schemas_dir) {
        Ok(resolved) => resolved,
        Err(schema::ResolveError::UnknownTemplate(name)) => {
            eprintln!(
                "error: unknown template '{name}'\n\navailable: {}",
                schema::TEMPLATE_NAMES.join("  ")
            );
            return 2;
        }
        Err(schema::ResolveError::ReadFailed { path, err }) => {
            eprintln!("error: cannot read schema from {path}: {err}");
            return 2;
        }
    };

    let template = match schema::parse(&resolved.yaml) {
        Ok(template) => template,
        Err(err) => {
            eprintln!("error: failed to parse schema '{name}': {err}");
            return 1;
        }
    };

    let view = if check {
        template.check_view(&resolved.source)
    } else {
        template.producer_view(&resolved.source)
    };
    print!("{view}");
    0
}
