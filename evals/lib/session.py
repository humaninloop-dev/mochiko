"""The `claude -p` session: argv assembly, stream parsing, halt detection.

Every target runs one isolated headless session per (arm, golden, replicate) on the host's
stored subscription auth with `--setting-sources ""` (no user- or project-level config, so
no user-level mochiko install loads beside the provisioned tree; stored auth survives it —
probe-settled 2026-08-27 and 2026-09-11). `--bare` (hermetic, ANTHROPIC_API_KEY, metered)
is the skill runner's `--local` opt-in and skips stored auth by design.
"""

import hashlib
import json
import subprocess
import sys

SYNTHETIC_MODEL = "<synthetic>"   # what every message carries once the session limit is hit

BARE = False   # `--bare` on every session and judge call; set by the skill runner's --local


def die(msg: str) -> None:
    print(f"error: {msg}", file=sys.stderr)
    sys.exit(1)


def sha(text: str) -> str:
    return hashlib.sha256(text.encode()).hexdigest()[:16]


def claude_argv(prompt: str, *, model: str, max_turns: int, output: str = "stream-json",
                plugin_dir=None, add_plugin_dir: bool = False, agent: str | None = None,
                permission_mode: str | None = None, allowed_tools: str | None = None,
                tools: str | None = None, append_system_prompt: str | None = None) -> list:
    """The argv of one session. `plugin_dir` is the provisioned tree (absolute; `add_plugin_dir`
    also grants Read there — without `--add-dir` every Read of a skill's references/ is
    auto-denied headless); `agent` seats the session as a persona; `tools` is the roster
    form of a fence (absent tools), `allowed_tools` the permission form (denied calls);
    `permission_mode` is each target's probe-settled mode; `output` is `stream-json`
    (with `--verbose`, the graded transcript) or `json` (a judge's one-turn reply)."""
    args = ["claude", "-p", prompt]
    if BARE:
        args.append("--bare")
    if agent:
        args += ["--agent", agent]
    if plugin_dir is not None:
        args += ["--plugin-dir", str(plugin_dir)]
        if add_plugin_dir:
            args += ["--add-dir", str(plugin_dir)]
    args += ["--setting-sources", ""]
    if tools is not None:
        args += ["--tools", tools]
    if allowed_tools is not None:
        args += ["--allowedTools", allowed_tools]
    if permission_mode:
        args += ["--permission-mode", permission_mode]
    args += ["--max-turns", str(max_turns), "--model", model, "--output-format", output]
    if output == "stream-json":
        args.append("--verbose")
    if append_system_prompt is not None:
        args += ["--append-system-prompt", append_system_prompt]
    return args


def run_claude(argv: list, *, cwd, timeout: int) -> subprocess.CompletedProcess:
    return subprocess.run(argv, cwd=cwd, capture_output=True, text=True, timeout=timeout)


def parse_stream(stdout: str) -> dict:
    """The stream-json transcript reduced to what the load gates and asserts read: the init
    and result events, the tool_use names in order (`tool_calls`) with their inputs
    (`tool_inputs`, for read-trace asserts), the skills the Skill tool fired, and the
    models seen. A subagent's events (`parent_tool_use_id` set — an Explore read) never
    speak for the session."""
    init = result = None
    tool_calls, tool_inputs, skills_fired, models = [], [], [], set()
    for line in stdout.splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            ev = json.loads(line)
        except json.JSONDecodeError:
            continue
        if ev.get("parent_tool_use_id"):
            continue
        if ev.get("type") == "system" and ev.get("subtype") == "init":
            init = ev
        elif ev.get("type") == "assistant":
            msg = ev.get("message") or {}
            if not isinstance(msg, dict):
                continue
            if msg.get("model"):
                models.add(msg["model"])
            for b in (msg.get("content") or []):
                if isinstance(b, dict) and b.get("type") == "tool_use":
                    inp = b.get("input") or {}
                    tool_calls.append(b.get("name"))
                    tool_inputs.append((b.get("name"), inp))
                    if b.get("name") == "Skill":
                        skills_fired.append(str(inp.get("skill", "")))
        elif ev.get("type") == "result":
            result = ev
    return {"init": init, "result": result, "tool_calls": tool_calls,
            "tool_inputs": tool_inputs, "skills_fired": skills_fired,
            "models": sorted(models)}


def loaded_plugins(init: dict | None) -> list:
    """[(name, version)] the init event reports — the load gate's evidence."""
    return [(p.get("name"), p.get("version")) for p in (init or {}).get("plugins", [])]


def auth_failed(result_text: str) -> bool:
    return "Not logged in" in result_text


def session_limit_hit(result_text: str, models: list) -> bool:
    """A session-limit hit answers every message with a synthetic error; a grid burning
    through it would store nothing valid — the caller halts and resumes after the reset."""
    return ("session limit" in result_text.lower()
            or (bool(models) and all(m == SYNTHETIC_MODEL for m in models)))


def claude_version() -> str:
    return subprocess.run(["claude", "--version"], capture_output=True, text=True).stdout.strip()
