# CLI schema delivery — wave 0 probe report

**Date:** 2026-09-03 · **Ruling home:** `record.md` D9 (wave 0, probes (a)–(h), numeric abort) ·
**Platform:** Claude Code 2.1.258 (macOS, arm64) · **Method:** a scratch probe plugin
(`probe2`, never shipped) loaded with `--plugin-dir`, run headless
(`claude -p "/probe2:<cmd>" --setting-sources "" --permission-mode dontAsk --model sonnet
--output-format stream-json`), every claim below read off the event stream, hook logs, or the
marker files the probes wrote. Raw streams stayed in the session scratchpad (not committed).
Total spend ≈ $0.60 across 16 headless runs.

## Verdict on the numeric abort

**NOT TRIPPED.** The `!` preprocessing shell sees the user's login `PATH`: `~/.cargo/bin`
and `/opt/homebrew/bin` are both present, `cargo` resolves bare. A bare `mochiko-cli`
resolves once `cargo install` / `brew install` has placed it there. D3 and D4 stand.

## Probe results

| # | question (D9) | result |
|---|---|---|
| (a) | does `!` preprocessing need a permission grant? | **YES.** With no `allowed-tools` grant, the `!` line is denied under `dontAsk` ("Permission to use Bash has been denied"). For a slash command the denial is injected as `<local-command-stderr>` and **the model turn never runs** (turns 0). For a skill invoked through the Skill tool the denial arrives as an error tool result the model can report. With `allowed-tools: Bash(echo *)`-class grants the line runs with no prompt. **Every sub-command is matched separately** — a `$(date +%s)` inside the command needed its own `Bash(date *)`; a bundled script under `Bash(${CLAUDE_PLUGIN_ROOT}/scripts/x.sh *)` with the same placeholder on the `!` line ran with no prompt (the documented pattern, verified). Under D4 the grant is `Bash(mochiko-cli *)` — bare names match (`echo` did). |
| (b) | is `~/.cargo/bin` / Homebrew on `PATH` in the preprocessing shell? | **YES** (see verdict). `SHELL`, `PWD` = the session cwd. `${CLAUDE_PLUGIN_ROOT}` **is substituted inside command bodies** (seen expanded in the denial echo) and inside `SKILL.md` bodies and `allowed-tools`. |
| (c) | `Skill` as a `PreToolUse` matcher; `UserPromptExpansion` on plugin command names | **BOTH YES.** `PreToolUse` with `matcher: "Skill"` fired with `tool_name: Skill`, `tool_input.skill: probe2:probe-skill-script`; its JSON `additionalContext` **reached the model**. `UserPromptExpansion` fired with `command_name: "probe2:env"` (namespaced), `expansion_type: slash_command`, `command_source: plugin`; its stdout **reached the model** as context. **Blocking:** `UserPromptExpansion` exit 2 → "UserPromptExpansion operation blocked by hook: … BLOCKED-BY-UPE-HOOK: mochiko-cli missing" — no model turn, the stderr text is the result. `PreToolUse` `permissionDecision: "deny"` → the Skill call fails with the reason as an error tool result. `SessionStart` stdout reached the model too. `${CLAUDE_PLUGIN_ROOT}` expanded in hook `command` strings. |
| (d) | does `!` run at subagent skill preload? | **YES.** A plugin agent with `skills: probe-skill-script` received the rendered skill (`PRELOAD-SKILL-PRESENT: YES`, the marker line present, the log file appended). **A permission-denied `!` at preload fails the spawn outright** ("task failed") — fail-closed. `CLAUDE_AGENT_TYPE` was not set in the preprocessing environment (`agent=none`). |
| (e) | largest render vs the Bash inline ceiling | **Ceiling ≈ 30,000 characters.** 25,000 chars arrived inline whole; 35,000 and 60,000 arrived as "Output too large (34.2KB). Full output saved to: <path>" plus a preview — **the preview carries the FIRST line** (`BIG-START` was reported), the tail is lost. Today's raw `implement.yaml` is 41,770 bytes; a compact render of 105 rules is at or above the ceiling. |
| (f) | `claude plugin eval` as a substrate | **Exists but early-access, org-gated, and undocumented publicly** (`claude plugin eval [target]` — cases under `evals/**/case.yaml` or `prompt.md` + `graders/*.md`, scored results, a no-plugin baseline arm; the `claude-code-guide` dispatch found no page at code.claude.com/docs and reports the command exits "currently in early access" for non-enabled orgs). Unverified offline-reference details: grader types `regex` · `tool_used` (transcript assertions with input matching) · `tool_order` · `file_exists` · `llm` (2-of-3 judge); JSON `aggregate-result.json` + HTML report; `--json` for CI; ≥ 2.1.198 for the command, ≥ 2.1.224 for stable v1. **Consequence for D8:** the bespoke `evals/commands` runner stays the contract-suite substrate; `claude plugin eval` is a revisit-on-enablement candidate (its `tool_used` grader would give the "no schema file Read" assert for free). |
| (g) | crates.io name, tap | `mochiko-cli` and `mochiko` both **free on crates.io** (404 with a UA). **`humaninloop-dev/homebrew-tap` already exists** (public, updated 2026-07-25). `mochiko-cli` free on npm. |
| (h) | npm-package road | Not exercised. Documented shape: `package.json` + lockfile → `npm ci --ignore-scripts` at install/update/session start; a `bin/` shim could exec a platform binary from an `optionalDependencies` package. Plausible, **unverified**, future option only (D4 stands). |

## Extra observations (not on the probe list)

1. **Manifest quirk under `--plugin-dir`:** with `"commands": "./commands/"` (the directory
   string form the mochiko manifest itself uses), the probe plugin's command files did NOT
   register — the whole directory appeared as one skill named `probe:commands`. The
   explicit array form (`"commands": ["./commands/env.md", …]`) and the default scan (no
   `commands` key) both registered every file. The real mochiko plugin, loaded the same
   way, registers all six commands. Cause not isolated (hooks presence ruled out — `probe4`
   without `hooks/` behaved the same). Consequence: the wave-3 pilot re-verifies
   registration on the real plugin; if it ever fails, the array form is the fallback.
2. **Fail-closed by denial, not fail-open:** a denied `!` line stops a command before the
   model runs, fails a subagent spawn at preload, and errors a Skill call — none of these
   silently degrade. The permission grant is therefore load-bearing: it is the difference
   between delivery and a hard stop.
3. **Hook context and blocking are model-independent:** every injected line reached the
   model verbatim; every block happened before the model acted.

## Design consequences (for the user's ruling — D3 is a user-ruled decision)

- **Render chunking.** One `!` line per schema section (six for a command, six for a
  skill), each rendering under the ~30k ceiling, instead of one whole-schema line. Each
  line is its own command with its own ceiling; the CLI gains `rules <primitive>
  --section <id>`. Alternative: shrink every render below 30k — `implement` cannot be
  guaranteed under it as it grows.
- **Head AND tail confirmation.** The oversized-output preview keeps the first line, so a
  version-triple head line alone would pass a truncated render. The render ends with a
  closing line (`mochiko-cli rules end · <primitive> · <N> rules`) and the `.md`'s halt
  clause requires both lines; either missing halts.
- **Grant shape confirmed:** `allowed-tools: Bash(mochiko-cli *)` in every converted `.md`
  (the D3 obligation), bare name, no path.

## Cost

16 headless runs on Sonnet/Haiku, ≈ $0.60 total, ≈ 25 minutes wall-clock including the
manifest-quirk diagnosis (four extra init-check runs).
