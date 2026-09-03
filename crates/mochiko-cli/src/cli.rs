//! The command surface: argument parsing, resolution order, and the exit-code contract.
//!
//! Four exit codes carry everything (record D3/D5, wave-plan §4): `0` ok · `1` the log is absent,
//! empty or unsound · `2` a usage error or a name the log does not carry · `3` the version
//! contract. Three beats one: a log outside the binary's grammar range halts with the D5 message
//! alone, because every other finding it might raise is downstream of not understanding the file.
//!
//! Output goes to caller-supplied sinks rather than straight to the process streams, so the
//! integration suite asserts on the exact bytes each stream carried without spawning a binary.

use crate::migration::GRAMMAR_RANGE;
use crate::render::{self, Context, PREAMBLE};
use crate::replay::{self, Replay};
use crate::validate::{census, Code, Finding};
use clap::{Parser, Subcommand};
use serde_norway::Value;
use std::io::Write;
use std::path::{Path, PathBuf};

/// The environment variable naming the migration log, read after an explicit flag and after a
/// plugin root, and before the working-directory guess.
const LOG_DIR_ENV: &str = "MOCHIKO_MIGRATIONS";

/// The log directory's name under a plugin root, and the working-directory fallback.
const LOG_DIR_NAME: &str = "migrations";

/// The plugin manifest, relative to a plugin root.
const PLUGIN_MANIFEST: &str = ".claude-plugin/plugin.json";

/// What a render reports when no plugin root resolved a version.
const UNKNOWN_VERSION: &str = "unknown";

#[derive(Parser)]
#[command(
    name = "mochiko-cli",
    about = "Delivers the mochiko plugin's rules and templates from the migration log.",
    disable_version_flag = true,
    disable_help_subcommand = true
)]
struct Cli {
    /// The plugin's root directory. Supplies the plugin version, and the log directory when
    /// `--log-dir` is not given.
    #[arg(long, global = true, value_name = "PATH")]
    plugin_root: Option<PathBuf>,

    /// The migration log directory. Wins over every other source.
    #[arg(long, global = true, value_name = "PATH")]
    log_dir: Option<PathBuf>,

    /// Print the binary's version and the grammar range it reads.
    #[arg(short = 'V', long)]
    version: bool,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Render one section of a command or skill's rules.
    Rules {
        /// A command name (`specify`) or a skill name (`review-feasibility`).
        primitive: String,
        /// A section id, or `preamble`.
        #[arg(long, value_name = "ID")]
        section: String,
    },
    /// Render an artifact template's producer or checklist view.
    Template {
        name: String,
        /// The checklist view instead of the producer view.
        #[arg(long)]
        check: bool,
    },
    /// Work on the migration log itself.
    Migrate {
        #[command(subcommand)]
        action: MigrateAction,
    },
    /// Regenerate the derived views from the replayed state.
    Views {
        #[command(subcommand)]
        action: ViewsAction,
    },
    /// Generate the genesis migration from the shipped schema corpus.
    Genesis {
        #[command(subcommand)]
        action: GenesisAction,
    },
}

#[derive(Subcommand)]
enum ViewsAction {
    /// Write every document's view under a directory, mirroring the repository's own layout.
    Emit {
        /// Where the views are written. Required, and deliberately without a default: a view
        /// emitter with a default output directory is one flag away from overwriting the
        /// shipped corpus it is derived from.
        #[arg(long, value_name = "DIR")]
        out: PathBuf,
    },
}

#[derive(Subcommand)]
enum GenesisAction {
    /// Write the genesis migration built from the shipped schema corpus.
    Emit {
        /// The file to write. Required, no default — see `views emit --out`.
        #[arg(long, value_name = "PATH")]
        out: PathBuf,
        /// The repository root the corpus and the provenance sidecar are read from.
        #[arg(long, value_name = "PATH", default_value = ".")]
        root: PathBuf,
    },
}

#[derive(Subcommand)]
enum MigrateAction {
    /// Replay the log, run the hard set, and print what it found.
    Validate {
        /// Print the advisory findings beside the rejecting ones.
        #[arg(long)]
        report: bool,
    },
    /// Print the log's grammar, its applied sequences, and the replayed state's hash.
    Status,
}

/// Parse and run. `args` excludes the program name.
pub fn dispatch(args: &[String], out: &mut dyn Write, err: &mut dyn Write) -> i32 {
    let argv = std::iter::once("mochiko-cli".to_string()).chain(args.iter().cloned());
    let cli = match Cli::try_parse_from(argv) {
        Ok(cli) => cli,
        Err(e) if e.use_stderr() => {
            let _ = write!(err, "{e}");
            return 2;
        }
        Err(e) => {
            // Help and any other display-only outcome: the request succeeded.
            let _ = write!(out, "{e}");
            return 0;
        }
    };

    if cli.version {
        let _ = writeln!(
            out,
            "mochiko-cli {} · grammar {}..{}",
            env!("CARGO_PKG_VERSION"),
            GRAMMAR_RANGE.0,
            GRAMMAR_RANGE.1
        );
        return 0;
    }

    let Some(command) = cli.command else {
        let _ = writeln!(
            err,
            "error: a subcommand is required\n\ntry 'mochiko-cli --help'"
        );
        return 2;
    };

    let dir = resolve_log_dir(cli.plugin_root.as_deref(), cli.log_dir.as_deref());
    match command {
        Command::Rules { primitive, section } => run_rules(
            &dir,
            cli.plugin_root.as_deref(),
            &primitive,
            &section,
            out,
            err,
        ),
        Command::Template { name, check } => run_template(&dir, &name, check, out, err),
        Command::Migrate { action } => match action {
            MigrateAction::Validate { report } => {
                run_validate(&dir, cli.plugin_root.as_deref(), report, out, err)
            }
            MigrateAction::Status => run_status(&dir, out, err),
        },
        Command::Views { action } => match action {
            ViewsAction::Emit { out: dest } => run_views_emit(&dir, &dest, out, err),
        },
        Command::Genesis { action } => match action {
            GenesisAction::Emit { out: dest, root } => run_genesis_emit(&root, &dest, out, err),
        },
    }
}

// ---------------------------------------------------------------------------
// resolution
// ---------------------------------------------------------------------------

/// Where the migration log lives, in precedence order: an explicit flag, then the plugin root's
/// own log, then the environment, then the working directory.
///
/// The environment sits ahead of the working directory deliberately, correcting the wave plan's
/// §4 ordering: a variable someone set is an explicit statement, while `./migrations` is a guess
/// that would shadow it in any directory that happens to hold one — the repository root included.
fn resolve_log_dir(plugin_root: Option<&Path>, log_dir: Option<&Path>) -> PathBuf {
    if let Some(dir) = log_dir {
        return dir.to_path_buf();
    }
    if let Some(root) = plugin_root {
        return root.join(LOG_DIR_NAME);
    }
    match std::env::var(LOG_DIR_ENV) {
        Ok(value) if !value.trim().is_empty() => PathBuf::from(value),
        _ => PathBuf::from(LOG_DIR_NAME),
    }
}

/// The plugin's version, or `unknown`.
///
/// Never a halt: the version triple reports what was resolvable, and a manifest that is missing
/// or malformed is a fact about the caller's environment, not a reason to withhold the rules.
fn resolve_plugin_version(plugin_root: Option<&Path>) -> String {
    let unknown = || UNKNOWN_VERSION.to_string();
    let Some(root) = plugin_root else {
        return unknown();
    };
    let Ok(text) = std::fs::read_to_string(root.join(PLUGIN_MANIFEST)) else {
        return unknown();
    };
    // JSON is a subset of YAML 1.2, so the manifest needs no second parser.
    let Ok(value) = serde_norway::from_str::<Value>(&text) else {
        return unknown();
    };
    value
        .get("version")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .unwrap_or_else(unknown)
}

// ---------------------------------------------------------------------------
// loading
// ---------------------------------------------------------------------------

/// Load a log for delivery, or the exit code that replaces the render.
///
/// The grammar check runs before anything else is reported: a log the binary cannot read produces
/// findings that are artefacts of the misreading, and D5's halt is the only honest thing to say.
fn load_for_delivery(dir: &Path, err: &mut dyn Write) -> Result<Replay, i32> {
    match replay::load_full(dir) {
        Ok(replay) if replay.state.docs.is_empty() => Err(report_empty_log(dir, err)),
        Ok(replay) => Ok(replay),
        Err(findings) => Err(report_load_failure(&findings, err)),
    }
}

/// Report an empty log directory and return the exit code every subcommand shares for it.
///
/// A directory that exists but holds no migration replays cleanly to an empty state with no
/// findings. Left unchecked that reads as success — every primitive an unknown name, and a
/// `migrate validate` gate green on a mis-pointed path. It is a delivery failure, so it exits 1
/// and names the directory it looked in.
fn report_empty_log(dir: &Path, err: &mut dyn Write) -> i32 {
    let _ = writeln!(
        err,
        "mochiko-cli: the migration log at {} is empty — it carries no migration file",
        dir.display()
    );
    1
}

/// Print a failed load and return its exit code: 3 for the version contract, 1 otherwise.
fn report_load_failure(findings: &[Finding], err: &mut dyn Write) -> i32 {
    if let Some(skew) = findings.iter().find(|f| f.code == Code::GrammarVersion) {
        // The D5 halt message, exactly as the parser wrote it — one home, no second copy.
        let _ = writeln!(err, "{}", skew.message);
        return 3;
    }
    for finding in findings.iter().filter(|f| f.is_rejecting()) {
        let _ = writeln!(err, "{finding}");
    }
    1
}

/// The version triple a render announces itself with.
fn context(replay: &Replay, plugin_root: Option<&Path>) -> Context {
    Context {
        binary: env!("CARGO_PKG_VERSION").to_string(),
        grammar: replay.grammar().unwrap_or(GRAMMAR_RANGE.0),
        plugin: resolve_plugin_version(plugin_root),
    }
}

// ---------------------------------------------------------------------------
// subcommands
// ---------------------------------------------------------------------------

fn run_rules(
    dir: &Path,
    plugin_root: Option<&Path>,
    primitive: &str,
    section: &str,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> i32 {
    let replay = match load_for_delivery(dir, err) {
        Ok(replay) => replay,
        Err(code) => return code,
    };
    let doc = match find_primitive(&replay.state, primitive) {
        Ok(doc) => doc,
        Err(e) => {
            let _ = writeln!(err, "error: {e}");
            return 2;
        }
    };
    let ctx = context(&replay, plugin_root);
    let rendered = if section == PREAMBLE {
        render::preamble(&replay.state, &doc, &ctx)
    } else {
        render::section(&replay.state, &doc, section, &ctx)
    };
    match rendered {
        Ok(text) => {
            let _ = write!(out, "{text}");
            0
        }
        Err(e) => {
            let _ = writeln!(err, "error: {e}");
            2
        }
    }
}

/// A primitive name resolves to a command or a skill.
///
/// The two name sets are disjoint today. An overlap is neither resolved by picking one nor
/// reported as an absence — the name is present twice, and saying it is missing would send the
/// reader hunting for a typo instead of at the log.
fn find_primitive(
    state: &replay::State,
    name: &str,
) -> Result<crate::model::DocRef, render::RenderError> {
    use crate::model::{DocKind, DocRef};
    let command = DocRef::new(DocKind::Command, name);
    let skill = DocRef::new(DocKind::Skill, name);
    match (
        state.docs.contains_key(&command),
        state.docs.contains_key(&skill),
    ) {
        (true, false) => Ok(command),
        (false, true) => Ok(skill),
        (true, true) => Err(render::RenderError::AmbiguousPrimitive(name.to_string())),
        (false, false) => Err(render::RenderError::UnknownPrimitive(name.to_string())),
    }
}

fn run_template(
    dir: &Path,
    name: &str,
    check: bool,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> i32 {
    let replay = match load_for_delivery(dir, err) {
        Ok(replay) => replay,
        Err(code) => return code,
    };
    match render::template_view(&replay.state, name, check, dir) {
        Ok(text) => {
            let _ = write!(out, "{text}");
            0
        }
        Err(e) => {
            let _ = writeln!(err, "error: {e}");
            2
        }
    }
}

fn run_validate(
    dir: &Path,
    plugin_root: Option<&Path>,
    report: bool,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> i32 {
    let mut state: Option<replay::State> = None;
    let mut findings = match replay::load_full(dir) {
        // An empty log is a failure here too: validate is the gate, and a gate that passes on a
        // mis-pointed `--log-dir` is worse than no gate.
        Ok(replay) if replay.state.docs.is_empty() => return report_empty_log(dir, err),
        Ok(replay) => {
            let findings = replay.all_findings();
            state = Some(replay.state);
            findings
        }
        Err(findings) => {
            if let Some(skew) = findings.iter().find(|f| f.code == Code::GrammarVersion) {
                let _ = writeln!(err, "{}", skew.message);
                return 3;
            }
            findings
        }
    };

    // Pointer resolution reads the installed tree, so it runs only when one was named. Its
    // status is printed below either way: a check that could not run must never read as one that
    // passed.
    let pointers = match (&state, plugin_root) {
        (Some(state), Some(root)) => Some(crate::validate::validate_pointers(state, root)),
        _ => None,
    };
    if let Some(pointers) = &pointers {
        findings.extend(pointers.findings.iter().cloned());
    }

    let mut rejecting = 0usize;
    let mut advisory = 0usize;
    for finding in &findings {
        if finding.is_rejecting() {
            rejecting += 1;
            let _ = writeln!(out, "{finding}");
        } else {
            advisory += 1;
            if report {
                let _ = writeln!(out, "{finding}");
            }
        }
    }
    // The similarity clusters are the last advisory report, printed under `--report` beside the
    // findings (wave-plan §3). Advisory throughout: the detector proposes candidates and never
    // gates, so it cannot move the exit code.
    if report {
        let _ = match (&pointers, plugin_root) {
            (Some(pointers), Some(root)) => writeln!(
                out,
                "pointer resolution: {} checked against {}",
                pointers.checked,
                root.display()
            ),
            _ => writeln!(
                out,
                "pointer resolution: skipped (no --plugin-root; pointers are unchecked, \
                 not clean)"
            ),
        };
        if let Some(state) = &state {
            // Resolved from the log directory, never the process working directory.
            let allowlist = crate::similar::find_allowlist(dir);
            let clusters = crate::similar::clusters(
                state,
                crate::similar::DEFAULT_THRESHOLD,
                allowlist.as_deref(),
            );
            let _ = write!(out, "{}", crate::similar::render_report(&clusters));
        }
    }

    let _ = writeln!(
        out,
        "mochiko-cli migrate validate · {rejecting} rejecting · {advisory} advisory"
    );
    if rejecting > 0 {
        1
    } else {
        0
    }
}

/// Regenerate the derived views under `dest`.
///
/// The views are a projection of the log, so this reads the log like every other delivery path:
/// a state nothing may be rendered from is a state nothing may be viewed from either.
fn run_views_emit(dir: &Path, dest: &Path, out: &mut dyn Write, err: &mut dyn Write) -> i32 {
    let replay = match load_for_delivery(dir, err) {
        Ok(replay) => replay,
        Err(code) => return code,
    };
    match crate::views::emit_to(&replay.state, dest) {
        Ok(written) => {
            let _ = writeln!(
                out,
                "mochiko-cli views emit · {} documents · {}",
                written.len(),
                dest.display()
            );
            0
        }
        Err(e) => {
            let _ = writeln!(err, "error: cannot write views to {}: {e}", dest.display());
            2
        }
    }
}

/// Build the genesis migration from the shipped corpus and write it to `dest`.
///
/// Reads the corpus and the provenance sidecar; writes exactly one file, the one named. The
/// sidecar is read only — its anchors are carried onto rules, never moved out of it (record D2).
fn run_genesis_emit(root: &Path, dest: &Path, out: &mut dyn Write, err: &mut dyn Write) -> i32 {
    let text = match crate::genesis::build(root) {
        Ok(text) => text,
        Err(errors) => {
            let _ = write!(err, "{}", crate::genesis::render_errors(&errors));
            return 1;
        }
    };
    if let Some(parent) = dest.parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                let _ = writeln!(err, "error: cannot create {}: {e}", parent.display());
                return 2;
            }
        }
    }
    match std::fs::write(dest, &text) {
        Ok(()) => {
            let _ = writeln!(
                out,
                "mochiko-cli genesis emit · {} bytes · {} lines · {}",
                text.len(),
                text.lines().count(),
                dest.display()
            );
            0
        }
        Err(e) => {
            let _ = writeln!(err, "error: cannot write {}: {e}", dest.display());
            2
        }
    }
}

fn run_status(dir: &Path, out: &mut dyn Write, err: &mut dyn Write) -> i32 {
    let replay = match load_for_delivery(dir, err) {
        Ok(replay) => replay,
        Err(code) => return code,
    };

    let sequences = replay.sequences();
    let count = sequences.len();
    let span = match (sequences.first(), sequences.last()) {
        (Some(first), Some(last)) => format!("{first}..{last}"),
        _ => "none".to_string(),
    };
    let _ = writeln!(
        out,
        "log {} · grammar {} · sequences {span} ({count} {})",
        dir.display(),
        replay.grammar().unwrap_or(GRAMMAR_RANGE.0),
        if count == 1 {
            "migration"
        } else {
            "migrations"
        }
    );

    let rules: usize = census(&replay.state).values().map(|(rules, _)| rules).sum();
    let _ = writeln!(
        out,
        "state {} · {} documents · {rules} rules",
        replay.state.content_hash(),
        replay.state.docs.len()
    );
    0
}
