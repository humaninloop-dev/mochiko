# Hook-enforced artifact schema — wave 0 probe plan

**Status:** awaiting lead approval · **Author:** QA seat · **Date:** 2026-09-13 · **Ruling home:**
`record.md` D8 (legs + abort criterion), OQ3, OQ4 · **Platform:** Claude Code 2.1.258
(`claude --version`), macOS arm64 · **Binary:** `mochiko-cli` on PATH at `~/.cargo/bin/mochiko-cli`

Nothing under `plugins/`, `crates/`, or any operating doc is touched; no git mutation. All probe
material lives under `<scratch>/wave0/`, `<scratch>` =
`/private/tmp/claude-501/-Users-deepeshadmin-Documents-GitHub-mochiko/9a06b8f1-b39c-4461-b67f-d4f40bb297b5/scratchpad`.

## Throwaway plugin design

Two plugins, so the `if`-narrowing question never contends with the deny question.

**Plugin A — `<scratch>/wave0/probe-gate/`** (the broad gate; every leg but 7).
`.claude-plugin/plugin.json`, `hooks/hooks.json`, `hooks/scripts/probe-gate.sh`,
`agents/probe-writer.md` (for the `--agent` transport). One `PreToolUse` entry, matcher
`Write|Edit|Bash|PowerShell|Read`, no `if`, `timeout: 5`, command
`${CLAUDE_PLUGIN_ROOT}/hooks/scripts/probe-gate.sh broad`.

`probe-gate.sh` (POSIX sh, no jq, mirroring `dependency-halt.sh`) appends one line per fire to an absolute
log path baked into the script — entry tag, epoch ms, `hook_event_name`, `tool_name`, `permission_mode`,
`session_id`, `agent_id`, `agent_type`, the **whole raw payload** — then always exits 0 with JSON:

- `Write`/`Edit` with `tool_input.file_path` under `probe-home/` → `deny`, sentinel `SENTINEL-DENY-WRITE`.
- `Bash`/`PowerShell` whose command text carries a write operator (`>`, `>>`, `tee`, `sed -i`,
  heredoc) aimed at `probe-home/` → `deny`, sentinel `SENTINEL-DENY-BASH`.
- `Read` → `additionalContext` sentinel `SENTINEL-REMINDER` (the D1b channel); the `broad-deny-read`
  arg denies instead, so Read reachability is measured two ways.
- everything else → **explicit** `allow`. Load-bearing: the guide's *Limitations* section states that
  for background subagents in non-interactive mode, "if no hook returns a decision, it denies the
  call" — an implicit allow would make every cell look denied.

Path matching is grep/sed, so it cannot carry multi-line escaped content — a probe limitation, not a
finding about the real `check` (D3/I4 already rules that).

**Plugin B — `<scratch>/wave0/probe-if/`** (leg 7 only). Same script, arg `narrow`, never denies, only logs.
Three `PreToolUse` entries, each with its own `if`: `Write(probe-home/**)`, `Read(probe-home/**)`, `Bash(cat *)`. Leg 7 measures *did the entry run at all*.

**Session layout:** cwd `<scratch>/wave0/probe-cwd/`, gated home `probe-cwd/probe-home/`, so `if`
patterns are cwd-relative and no `--add-dir` is needed. Marker files reset before every run; their
presence afterwards is ground truth for "the write persisted".

## Transports

| id | transport | how |
|---|---|---|
| T1 | headless lead session | `claude -p` from `probe-cwd` |
| T2 | `Agent` subagent of T1 | lead prompt instructs one unnamed `general-purpose` spawn |
| T3 | agent-team teammate | same, spawned **with a `name`** plus a `SendMessage` round |
| T4 | `--agent` headless session | `--agent probe-gate:probe-writer` |

Common flags: `--setting-sources ''` (drops the installed mochiko plugin, so its hooks cannot confound),
`--plugin-dir <A or B>`, `--output-format stream-json --verbose`, **`--include-hook-events`** (hook
lifecycle events land in the stream), `--model sonnet`, `--max-turns 12`, `--allowedTools 'Agent,Read,Write,Edit,Bash'`.

T3 note: `claude --help` at 2.1.258 exposes no team flag; a teammate is a named `Agent` spawn
addressable by `SendMessage`, so T3 is reachable headless by passing `name`. If the named spawn or the
`SendMessage` round fails headless, the cell records "not reachable headless" with that evidence.

## Matrix — 32 cells

| leg | cells | shape |
|---|---|---|
| 1 transport coverage | 12 | 4 transports × 3 channels (Write deny · Read reminder · Bash deny), baseline mode `dontAsk` |
| 2 `Read` field name | 1 | read off leg 1's captured payloads (T1, T2); no extra run |
| 3 Bash heredoc | 4 | {T1, T2} × {heredoc fires with command text intact · seat retries through Bash after a `Write` deny}; retry question at n=2 runs each |
| 4 permission modes | 8 | {T1, T2} × {`dontAsk`, `acceptEdits`, `bypassPermissions`, `auto`} — deny must hold in all four |
| 5 dead gate | 2 | T1 × {wrapper `chmod -x` · `command` path that does not resolve} |
| 6 per-run cost | 1 | offline: transcript call counts × measured per-call ms |
| 7 `if` field | 4 | plugin B: Write in-home vs out-of-home · Read narrowing · Bash best-effort · version record |

Estimated 14–18 headless runs, assertions packed per run; precedent spend for 16 runs was ≈ $0.60.

## Evidence captured per cell

1. `<scratch>/wave0/probe-gate.log` — the hook's append log: ground truth for *did it fire*, and leg 2's payload source.
2. `<scratch>/wave0/runs/<run-id>.stream.jsonl` — stream-json with hook lifecycle events; sentinels searched here for *did the text reach the seat*.
3. `~/.claude/projects/<slug>/<session>/subagents/agent-*.jsonl` — T2/T3 spawn proof, and the sidechain's own view of the denial.
4. `probe-home/` listing after the run — persistence ground truth.

Every platform claim in the report carries a doc citation (page + section) or a captured payload;
unverified claims are marked. Already located: *Hooks and permission modes* (hooks-guide) for the
every-mode deny; *Block edits to protected files* (hooks-guide) for the `Bash|PowerShell` plus
`git status --porcelain` remedy; *Common fields* / *Common input fields* (hooks reference) for `if`,
`agent_id`, `agent_type`; *Limitations* (hooks-guide) for the non-executable-path notice and the
background-subagent implicit deny. **Not located on either page:** a PowerShell-hook-input section (the
guide links `/docs/en/hooks#powershell`; the target did not resolve in two fetches) and any `tool_input`
schema for `Read` — both recorded as undocumented, and why legs 2 and 3 exist.

## Cost model (leg 6)

Representative run: kinako session `613c3001-67b5-4897-870c-ea9c2a2858bf` — largest in the
2026-09-09..12 window at 3.88 MB, 51 `EPIC-002` mentions, with a `subagents/` directory. Counts come
from the main transcript **plus every sidechain** (producing seats ride subagents), split by
`Read`/`Write`/`Edit`/`Bash`, with `Read` further split on whether the path is under `.mochiko/` — the
`if`-narrowed term. The enumeration goes to a native `Explore` subagent at `model: haiku` per
`mochiko:patterns-model-tiering`; I read the counts back and spot-check one tool class before they count.
`hyperfine` is absent, so per-call cost is 20 timed runs of `mochiko-cli migrate status --plugin-root
plugins/mochiko` (the `check` proxy — `check` does not exist yet), min / median / mean / p95, run by me.

The report gives per-call ms, calls per run by tool, and two projected per-run figures: an unnarrowed
upper bound (every Read and Bash call gated) and the `if`-narrowed figure (home reads only). The proposed
budget line is keyed to the narrowed figure, since D1b ships the narrowing, with the unnarrowed bound
stated as the cost without it.

## Abort / proceed reading rule

**Abort wave 1** if either holds: T2 (`Agent` subagent) shows no `Write` fire in the hook log **or**
the deny does not stop the write (marker file present afterwards) — D8's stated criterion, the
transport every producing seat rides; or leg 4 shows the Write deny inert on T1 or T2 under
`acceptEdits` or `bypassPermissions` (I12: a gate inert in the modes the runs use is the same abort).

**Disclosed hole, proceed:** a T3-only miss (OQ3's mitigation then owed), a T4-only miss, a
`Read`-channel-only miss (the reminder is not the gate), a best-effort-only `Bash` parse, or a dead
gate that proceeds silently (expected — fail-open is F7).

**Proceed with findings:** everything else, leg 2's `Read` field name and any `if` correction
included.
