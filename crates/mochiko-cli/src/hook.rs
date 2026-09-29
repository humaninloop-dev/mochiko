//! The `PreToolUse` hook surface: the payload in, the decision JSON out.
//!
//! The shipped wrapper is a pipe. It hands the raw hook payload to the binary on stdin and prints
//! what comes back, holding no rule of its own — the same posture as `dependency-halt.sh`. The
//! parse lives here because the wrapper's `grep`-and-`sed` field reader provably cannot carry
//! multi-line escaped content (record I4), and `jq` is not a dependency the plugin may add. A
//! wave-0 probe measured the consequence: the shell reader false-allowed
//! `printf 'retry probe' > "<abs>/probe-home/c.md"`, truncating at the first escaped quote.
//!
//! # The decision is always explicit
//!
//! A wave-0 finding, folded here: the platform **denies a background subagent's call when no hook
//! returns a decision**. So every non-deny outcome renders an explicit
//! `permissionDecision: allow` and empty stdout is never a verdict. The exit codes that mean the
//! binary could not read its log (1, 2, 3) print nothing at all, and the wrapper supplies the
//! explicit allow for those — a binary that cannot read its log must never deny a write.
//!
//! # Why JSON is written by hand
//!
//! The output shape is three fixed keys, so a serializer would be a dependency bought for nothing;
//! the pre-code ladder stops at the stdlib rung. [`escape`] is the part that must be right, and it
//! is tested against quotes, backslashes, control characters and multi-byte UTF-8. Parsing reuses
//! `serde_norway`, already in the tree: YAML 1.2 is a JSON superset, and a probe confirmed it reads
//! real payloads including `\/`, which plain YAML would reject.

use crate::conform::{self, Decision};
use crate::home::{self, Homes, Resolution};
use crate::replay::State;
use crate::shell;
use serde::Deserialize;
use std::path::Path;

/// The exit code a conformance denial carries — minted, not reused.
///
/// Reusing 1 would make an unsound log indistinguishable from a non-conforming artifact, and the
/// exit-code contract (record D3) separates exactly those two: only this code becomes a deny, while
/// 1, 2 and 3 pass through and the write proceeds.
pub const EXIT_CONFORMANCE: i32 = 4;

/// The `PreToolUse` payload, as much of it as a conformance check reads.
///
/// Every field is optional and unknown fields are ignored: the platform adds fields over time, and
/// a gate that failed closed on an unrecognised payload would deny a consumer's every write.
#[derive(Debug, Default, Deserialize)]
pub struct Payload {
    #[serde(default)]
    pub hook_event_name: Option<String>,
    #[serde(default)]
    pub tool_name: Option<String>,
    #[serde(default)]
    pub cwd: Option<String>,
    #[serde(default)]
    pub tool_input: ToolInput,
}

/// The tool's own arguments. `file_path` is shared by `Write`, `Edit` and `Read` — wave 0 confirmed
/// the `Read` spelling, so one struct covers every tool in the matcher set.
#[derive(Debug, Default, Deserialize)]
pub struct ToolInput {
    #[serde(default)]
    pub file_path: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub old_string: Option<String>,
    #[serde(default)]
    pub new_string: Option<String>,
    #[serde(default)]
    pub command: Option<String>,
}

/// What the hook answers.
#[derive(Debug)]
pub enum Outcome {
    Allow { context: Option<String> },
    Deny { reason: String },
}

impl Outcome {
    /// The process exit code this outcome carries.
    pub fn exit_code(&self) -> i32 {
        match self {
            Outcome::Allow { .. } => 0,
            Outcome::Deny { .. } => EXIT_CONFORMANCE,
        }
    }
}

/// Parse a hook payload, or `None` when it is not one.
///
/// `None` is a usage error at the CLI boundary (exit 2, silent): a payload the binary cannot read is
/// a fact about the caller, never a reason to deny a write.
pub fn parse(input: &str) -> Option<Payload> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }
    serde_norway::from_str::<Payload>(trimmed).ok()
}

/// Decide one tool call.
pub fn decide(state: &State, payload: &Payload) -> Outcome {
    let tool = payload.tool_name.as_deref().unwrap_or_default();
    match tool {
        "Write" | "Edit" => decide_file(state, payload),
        "Bash" | "PowerShell" => decide_shell(state, payload),
        // Every other tool is knowingly outside the matcher set (record D1c). `NotebookEdit` and
        // any MCP file writer a consumer enables are named there as deliberate holes, not defects.
        _ => Outcome::Allow { context: None },
    }
}

/// Render the decision JSON the wrapper prints verbatim. One line, always.
pub fn render(outcome: &Outcome) -> String {
    let mut out = String::from(r#"{"hookSpecificOutput":{"hookEventName":"PreToolUse""#);
    match outcome {
        Outcome::Allow { context } => {
            out.push_str(r#","permissionDecision":"allow""#);
            if let Some(context) = context {
                out.push_str(r#","additionalContext":""#);
                out.push_str(&escape(context));
                out.push('"');
            }
        }
        Outcome::Deny { reason } => {
            out.push_str(r#","permissionDecision":"deny","permissionDecisionReason":""#);
            out.push_str(&escape(reason));
            out.push('"');
        }
    }
    out.push_str("}}");
    out
}

/// Escape a string as a JSON string body.
///
/// Short forms where JSON defines them, `\uXXXX` for every other control character, and multi-byte
/// UTF-8 passed through literally. A hand-rolled escaper is what bit the shell wrapper, so this one
/// is exhaustive over the control range rather than over the characters that seemed likely.
fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 16);
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

// ---------------------------------------------------------------------------
// the Write and Edit legs
// ---------------------------------------------------------------------------

fn decide_file(state: &State, payload: &Payload) -> Outcome {
    let Some(file_path) = payload.tool_input.file_path.as_deref() else {
        return Outcome::Allow { context: None };
    };
    let cwd = payload.cwd.as_deref().map(Path::new);
    let Some(absolute) = home::absolute(Path::new(file_path), cwd) else {
        // A relative path with no `cwd` to join it to names no place at all.
        return Outcome::Allow { context: None };
    };
    // A path in no git tree — the session scratchpad, `~/.claude/`, a cold-verification snapshot
    // (delta-check N4) — has no home tree, and outside `<root>/.mochiko/` nothing changes (D3): it
    // is not the gate's, as a path outside the working directory was not before (G1 A5).
    let Some(located) = home::locate(&absolute) else {
        return Outcome::Allow { context: None };
    };
    let shown = located.relative.display().to_string();

    let homes = Homes::load(state);
    let resolution = homes.resolve(&located.relative);

    // The path-class verdicts read no body, so they are decided before any file is read.
    //
    // The closed world (field review D3): under `<root>/.mochiko/`, no home is a deny. A log that
    // declares no home has declared nothing to close — the same rule the shell leg keeps.
    if matches!(resolution, Resolution::Outside) && located.in_home_tree() && !homes.is_empty() {
        return outcome_of(conform::closed_world(
            &homes,
            &located.relative,
            homes.run_folder(&located).as_deref(),
        ));
    }
    // The run folder's layout controls (field review D4 as amended): main tree only (V3), then
    // the ignore guard (control 3).
    let raw_home = match &resolution {
        Resolution::Raw { home, .. } | Resolution::File { home, .. } if home.raw_output => {
            Some(*home)
        }
        _ => None,
    };
    if let Some(raw_home) = raw_home {
        if let Some(refusal) = run_folder_refusal(&homes, raw_home, &located) {
            return refusal;
        }
    }

    // The on-disk file is read only where a verdict needs it (G1 A9): to apply an `Edit` in
    // memory, and as the first-touch amnesty's baseline for a file a home measures.
    let edit = payload.tool_name.as_deref() == Some("Edit");
    let amnesty = matches!(
        resolution,
        Resolution::File { .. } | Resolution::Report { .. } | Resolution::UndeclaredFile { .. }
    );
    let baseline = if edit || amnesty {
        std::fs::read_to_string(&absolute).ok()
    } else {
        None
    };

    let candidate = if edit {
        let old = payload.tool_input.old_string.as_deref().unwrap_or_default();
        let new = payload.tool_input.new_string.as_deref().unwrap_or_default();
        let Some(baseline) = baseline.as_deref() else {
            // No file to edit; the platform rejects it on its own.
            return Outcome::Allow { context: None };
        };
        match conform::apply_edit(baseline, old, new) {
            Some(applied) => applied,
            // `old_string` is absent from the file. The platform rejects that itself, and a
            // second denial from here would only confuse the seat.
            None => return Outcome::Allow { context: None },
        }
    } else {
        payload
            .tool_input
            .content
            .as_deref()
            .unwrap_or_default()
            .to_string()
    };

    match &resolution {
        // The D9 sniff, outside `<root>/.mochiko/`: gated only when the content is a mochiko
        // report.
        Resolution::Outside => return outcome_of(conform::sniff(state, &shown, &candidate)),
        // The report sniff on `.md` in the run folder (control 4) ...
        Resolution::Raw { .. } if is_markdown(&absolute) => {
            return outcome_of(conform::sniff_unchecked(
                state,
                &shown,
                &candidate,
                "the run folder (raw output, deleted at the run's acceptance)",
            ));
        }
        // ... and in every declared sub-directory with no home document of its own (V2, as ruled
        // at plan Q2).
        Resolution::Deferred { subdir, .. } if is_markdown(&absolute) => {
            return outcome_of(conform::sniff_unchecked(
                state,
                &shown,
                &candidate,
                &format!("`{subdir}/`, a declared sub-directory with no home document of its own"),
            ));
        }
        _ => {}
    }

    // The run folder is named only by a path-class deny, so the worktree pointer it may need is
    // read only then.
    let run_folder = match resolution {
        Resolution::UndeclaredFile { .. } | Resolution::UndeclaredSubdir { .. } => {
            homes.run_folder(&located)
        }
        _ => None,
    };
    outcome_of(conform::check(
        state,
        &resolution,
        &candidate,
        baseline.as_deref(),
        run_folder.as_deref(),
    ))
}

/// Whether a path names a markdown file — the only kind the report sniff reads.
fn is_markdown(path: &Path) -> bool {
    path.extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
}

/// A write into the run folder refused by its layout controls, or `None` when both hold.
///
/// Both read the repository, not the body: the worktree's `.git` pointer (V3) and the main
/// tree's `.gitignore` (control 3) — the two reads GI-019's "What the gate reads" names for this.
fn run_folder_refusal(
    homes: &Homes,
    raw_home: &home::Home,
    located: &home::Located,
) -> Option<Outcome> {
    let main = located.main_root();
    if main != located.root {
        let folder = homes.run_folder(located).unwrap_or_default();
        return Some(Outcome::Deny {
            reason: format!(
                "`{}` is under this worktree's own run folder. The run folder is always the main \
                 tree's — one folder every seat of the run shares, which outlives the worktree — \
                 so write it under `{folder}` instead. {}",
                located.relative.display(),
                conform::HALT_HINT
            ),
        });
    }
    if !home::ignores(&main, raw_home) {
        let line = raw_home.ignore_line();
        return Some(Outcome::Deny {
            reason: format!(
                "the run folder is not git-ignored: `{}/.gitignore` carries no `{line}` line, so \
                 nothing written here is kept out of a commit or a cold-verification snapshot. \
                 Add the line `{line}` to `{}/.gitignore` (run-open adds it), then write again. {}",
                main.display(),
                main.display(),
                conform::HALT_HINT
            ),
        });
    }
    None
}

fn outcome_of(verdict: conform::Verdict) -> Outcome {
    match verdict.decision {
        Decision::Allow => Outcome::Allow {
            context: verdict.context,
        },
        Decision::Deny => Outcome::Deny {
            reason: verdict
                .reason
                .unwrap_or_else(|| "the write does not conform to its declared shape".to_string()),
        },
    }
}

// ---------------------------------------------------------------------------
// the Bash and PowerShell leg
// ---------------------------------------------------------------------------

/// The reason every shell denial carries.
const SHELL_REASON: &str =
    "artifacts under declared homes are written with Write/Edit, never through a shell redirect. \
     This command's write target resolves under";

fn decide_shell(state: &State, payload: &Payload) -> Outcome {
    let Some(command) = payload.tool_input.command.as_deref() else {
        return Outcome::Allow { context: None };
    };
    let cwd = payload.cwd.as_deref().map(Path::new);
    let homes = Homes::load(state);
    if homes.is_empty() {
        return Outcome::Allow { context: None };
    }

    // The two shells have different write vocabularies, and a cmdlet name in a Bash command means
    // nothing. Keying the table to the tool keeps each arm's false-positive surface its own.
    let targets = match payload.tool_name.as_deref() {
        Some("PowerShell") => shell::powershell_write_targets(command),
        _ => shell::write_targets(command),
    };
    for target in targets {
        // A relative target is joined to the payload's `cwd`, so a write from a cwd inside a home
        // resolves into that home — which closes the prior build's disclosed evasion 3.
        let Some(located) =
            home::absolute(Path::new(&target), cwd).and_then(|abs| home::locate(&abs))
        else {
            continue;
        };
        let relative = &located.relative;
        // A file inside a home, or the home directory itself as a `cp`/`mv` destination.
        let direct = homes.resolve(relative);
        let as_directory = homes.resolve(&relative.join("_"));
        if let Some(raw_home) = raw_output_home(&direct, &as_directory) {
            // The run-folder carve (field review V1/N2): the one home whose shell writes are
            // admitted, for non-`.md` files only, under the folder's own controls.
            if let Some(refusal) = run_folder_refusal(&homes, raw_home, &located) {
                return refusal;
            }
            if let Some(refusal) = carve_refusal(&target, &direct) {
                return refusal;
            }
            continue;
        }
        let governed =
            !matches!(direct, Resolution::Outside) || !matches!(as_directory, Resolution::Outside);
        if governed {
            let raw_route = homes
                .run_folder(&located)
                .map(|run| {
                    format!(
                        " Raw output (captured console, logs, dumps) goes to the run folder \
                         `{run}`, which is ephemeral — deleted at the run's acceptance — and \
                         takes a shell write of a non-`.md` file."
                    )
                })
                .unwrap_or_default();
            return Outcome::Deny {
                reason: format!(
                    "{SHELL_REASON} a declared artifact home: `{target}`.{raw_route} The parse is \
                     best-effort over the command text. {}",
                    crate::conform::HALT_HINT
                ),
            };
        }
        if located.in_home_tree() {
            // The closed world reaches parsed shell writes too (field review D3).
            return Outcome::Deny {
                reason: format!(
                    "This command writes `{target}`, which resolves to no declared home under \
                     `.mochiko/`, where the world is closed: a write here is refused. Artifacts \
                     are written with Write/Edit, never through a shell redirect. {} The parse is \
                     best-effort over the command text. {}",
                    conform::closed_world_routes(
                        &homes,
                        relative,
                        homes.run_folder(&located).as_deref()
                    ),
                    crate::conform::HALT_HINT
                ),
            };
        }
    }
    Outcome::Allow { context: None }
}

/// The raw-output home a shell target lands in — as a file under it, or as the run folder itself
/// named as a `cp`/`mv` destination directory — or `None`.
fn raw_output_home<'a>(
    direct: &Resolution<'a>,
    as_directory: &Resolution<'a>,
) -> Option<&'a home::Home> {
    match (direct, as_directory) {
        (Resolution::Raw { home, .. } | Resolution::File { home, .. }, _) if home.raw_output => {
            Some(*home)
        }
        (Resolution::Outside, Resolution::Raw { home, .. }) => Some(*home),
        _ => None,
    }
}

/// A shell write into the run folder the carve does not admit, or `None` when it does.
///
/// The gate cannot see what a shell write puts in a file, so a `.md` — which the report sniff
/// would read on a `Write` — is refused here (delta-check N2). A directory target hides the file
/// name that test needs, so it is refused too (plan Q3): `target/` with its trailing slash, or the
/// run folder itself. A sub-directory named without its slash (`cp -t <run>/sub x.md`) reads as a
/// file and is admitted — the scan does not stat the target, and the gap is disclosed.
fn carve_refusal(target: &str, direct: &Resolution) -> Option<Outcome> {
    let directory = target.ends_with('/') || matches!(direct, Resolution::Outside);
    if directory {
        return Some(Outcome::Deny {
            reason: format!(
                "`{target}` names a directory under the run folder, which hides the file the \
                 write lands in, and a shell may write only a non-`.md` file here — name the \
                 destination file instead: `cp build.log <run folder>/build.log`. {}",
                conform::HALT_HINT
            ),
        });
    }
    if is_markdown(Path::new(target)) {
        return Some(Outcome::Deny {
            reason: format!(
                "`{target}` is a `.md` in the run folder. A shell write's content is invisible to \
                 the gate, so a `.md` there is written with Write/Edit, where the report sniff \
                 reads it; the shell carve admits non-`.md` files only. {}",
                conform::HALT_HINT
            ),
        });
    }
    None
}
