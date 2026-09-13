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
use crate::home::Homes;
use crate::replay::State;
use serde::Deserialize;
use std::path::{Path, PathBuf};

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
    let Some(cwd) = payload.cwd.as_deref() else {
        return Outcome::Allow { context: None };
    };
    let Some(file_path) = payload.tool_input.file_path.as_deref() else {
        return Outcome::Allow { context: None };
    };
    let Some(relative) = relativize(cwd, file_path) else {
        // Outside the working directory: not this repository's business.
        return Outcome::Allow { context: None };
    };

    let absolute = PathBuf::from(cwd).join(&relative);
    let baseline = std::fs::read_to_string(&absolute).ok();

    let candidate = match payload.tool_name.as_deref() {
        Some("Edit") => {
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
        }
        _ => payload
            .tool_input
            .content
            .as_deref()
            .unwrap_or_default()
            .to_string(),
    };

    let homes = Homes::load(state);
    let resolution = homes.resolve(&relative);
    if matches!(resolution, crate::home::Resolution::Outside) {
        // The D9 sniff: gated only when the content is a mochiko report.
        return outcome_of(conform::sniff(
            state,
            &relative.display().to_string(),
            &candidate,
        ));
    }
    outcome_of(conform::check(
        state,
        &resolution,
        &candidate,
        baseline.as_deref(),
    ))
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

/// A path relative to `cwd`, or `None` when it does not sit under it.
///
/// A relative `file_path` is taken as already relative to `cwd`. Nothing here touches the
/// filesystem, so a path that does not exist yet resolves the same as one that does.
fn relativize(cwd: &str, file_path: &str) -> Option<PathBuf> {
    let path = Path::new(file_path);
    if !path.is_absolute() {
        return Some(normalize(path));
    }
    let cwd = normalize(Path::new(cwd));
    let path = normalize(path);
    path.strip_prefix(&cwd).ok().map(Path::to_path_buf)
}

/// Collapse `.` components and resolve `..` lexically, so no traversal survives into a home.
fn normalize(path: &Path) -> PathBuf {
    use std::path::Component;
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
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
    let cwd = payload.cwd.as_deref().unwrap_or("");
    let homes = Homes::load(state);
    if homes.is_empty() {
        return Outcome::Allow { context: None };
    }

    for target in write_targets(command) {
        let Some(relative) = relativize(cwd, &target) else {
            continue;
        };
        // A file inside a home, or the home directory itself as a `cp`/`mv` destination.
        let governed = !matches!(homes.resolve(&relative), crate::home::Resolution::Outside)
            || !matches!(
                homes.resolve(&relative.join("_")),
                crate::home::Resolution::Outside
            );
        if governed {
            return Outcome::Deny {
                reason: format!(
                    "{SHELL_REASON} a declared artifact home: `{target}`. The parse is \
                     best-effort over the command text. {}",
                    crate::conform::HALT_HINT
                ),
            };
        }
    }
    Outcome::Allow { context: None }
}

/// Every path this command text appears to write to.
///
/// Best-effort over the command string, and stated as such in the deny reason — a shell is not
/// parsed here, it is scanned. The operators are the ones a denied seat reaches for first: a
/// redirect, `tee`, an in-place `sed`, and a `cp`/`mv` destination (record D1c). A heredoc is
/// covered by the redirect it needs to write anything.
fn write_targets(command: &str) -> Vec<String> {
    let tokens = tokenize(command);
    let mut targets = Vec::new();
    let mut index = 0;
    while index < tokens.len() {
        let token = tokens[index].as_str();
        // `> path`, `>> path`, the clobber form `>| path`, and each of them glued to its path.
        //
        // `>|` is here because round 1's G5 found it was a one-character evasion: stripping `>`
        // left `|path`, which resolved to nothing and allowed the write.
        if let Some(rest) = token.strip_prefix(">>").or_else(|| token.strip_prefix('>')) {
            let rest = rest.strip_prefix('|').unwrap_or(rest);
            if rest.is_empty() {
                if let Some(next) = tokens.get(index + 1) {
                    targets.push(next.trim_start_matches('|').to_string());
                }
            } else {
                targets.push(rest.to_string());
            }
        }
        match bare_command(token) {
            // Every following non-flag argument is a destination.
            "tee" => targets.extend(non_flag_args(&tokens[index + 1..])),
            // `cp`/`mv`'s destination is its last argument.
            "cp" | "mv" | "install" => {
                if let Some(last) = non_flag_args(&tokens[index + 1..]).pop() {
                    targets.push(last);
                }
                // A `mv` *out of* a home still names the home as its source, and a re-home under
                // the gate is done with Write by design, so both ends are targets here.
                if let Some(first) = non_flag_args(&tokens[index + 1..]).first() {
                    targets.push(first.clone());
                }
            }
            "sed" if tokens[index + 1..].iter().any(|t| t.starts_with("-i")) => {
                targets.extend(non_flag_args(&tokens[index + 1..]));
            }
            "dd" => {
                for token in &tokens[index + 1..] {
                    if let Some(path) = token.strip_prefix("of=") {
                        targets.push(path.to_string());
                    }
                }
            }
            _ => {}
        }
        index += 1;
    }
    targets.retain(|t| !t.is_empty());
    targets
}

/// A command token's bare name, without a leading path.
fn bare_command(token: &str) -> &str {
    token.rsplit('/').next().unwrap_or(token)
}

/// The non-flag arguments of an argument run, stopping at the next shell separator.
fn non_flag_args(tokens: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for token in tokens {
        if matches!(token.as_str(), "|" | "&&" | "||" | ";" | "&") {
            break;
        }
        if token.starts_with('-') || token.starts_with('>') || token.starts_with('<') {
            continue;
        }
        out.push(token.clone());
    }
    out
}

/// Split a command into tokens, honouring quotes and splitting a glued redirect off its path.
///
/// Quotes are stripped, which is the point: the wave-0 probe's line quoted its target, and a
/// scanner that kept the quotes would not have matched a home.
fn tokenize(command: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    for ch in command.chars() {
        match (quote, ch) {
            (Some(open), c) if c == open => quote = None,
            (Some(_), c) => current.push(c),
            (None, '\'') | (None, '"') => quote = Some(ch),
            (None, c) if c.is_whitespace() => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            // A redirect is its own token even when glued to the previous word (`x>file`).
            (None, '>') => {
                if !current.is_empty() && !current.ends_with('>') {
                    tokens.push(std::mem::take(&mut current));
                }
                current.push('>');
            }
            (None, c) => current.push(c),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}
