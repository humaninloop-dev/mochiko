"""The contract suite's own sandbox helpers — deliberately not shared with the eval runner.

Lifted from `evals/run.py` as it stood at **32c1ed5** (wave 6, the last commit that touched
`evals/contract/run.py` against a working runner): `SANDBOX`, `sbx_sh`, and `claude_args` in its
pre-convergence shape. `SANDBOX` and `sbx_sh` are verbatim. `claude_args` differs from the lifted
original in three ways, none of which any caller depends on: its `LOCAL_MODE` limb — inserting
`--bare` for the runner's hermetic local mode — is dropped, because this suite has no local mode
and never reads that switch; `stream` and `plugin` gained defaults (`True` and `None`) where the
original required both positionally; and the literal `"acceptEdits"` moved to the `PERMISSION_MODE`
constant below at the same value. Both call sites in `run.py` pass every parameter by keyword, so
the defaults are never exercised and the argv this builds is byte-identical to the original's
sandbox-mode output.

**Why it is a copy and not an import.** This suite is a release gate: GI-012 makes it a condition
of every `plugin.json` bump, and says a SKIPPED suite is not green. `evals/run.py` is a research
harness for the skill evals and is free to change whenever that work needs it — which is exactly
what happened. On 2026-09-11 (`8c27460`) it converged onto host mode: `SANDBOX` and `sbx_sh` were
removed outright and `claude_args` swapped two positional parameters. The contract suite kept
calling all three, so `preflight()` began raising `AttributeError: module 'mochiko_eval_runner'
has no attribute 'SANDBOX'` before any sandbox case could run, and `run_probe()` — whose call had
silently rebound to the new order — was asking the CLI for `--plugin-dir True`. Seventy-seven of
eighty-two cases, roughly a hundred and fifty metered sessions, could not execute, and the failure
mode was an unhandled exception rather than a skip.

A gate may not be at the mercy of a harness that owes it nothing. The duplication is the point: if
the skill evals change again, this file does not move, and the only thing that can break the gate
is a change someone makes to the gate.

Provenance: `.mochiko/brainstorms/hook-enforced-artifact-schema/reports/contract-report.md`,
§ Sandbox repair (wave 4, 2026-09-15).
"""

import pathlib
import subprocess

# The Docker AI sandbox the suite runs every session in (`sbx` CLI).
SANDBOX = "claude-mochiko"

# `acceptEdits`, not `dontAsk`: the wave-1 probe found `dontAsk` denies writes absent an explicit
# allow rule, and a headless run cannot answer a permission prompt. Carried over verbatim.
PERMISSION_MODE = "acceptEdits"


def sbx_sh(script: str, timeout: int = 1800) -> subprocess.CompletedProcess:
    """One shell command inside the sandbox."""
    return subprocess.run(
        ["sbx", "exec", SANDBOX, "sh", "-c", script],
        capture_output=True,
        text=True,
        timeout=timeout,
    )


def claude_args(prompt: str, model: str, max_turns: int, stream: bool = True,
                plugin: pathlib.Path | None = None) -> list:
    """The argv for one headless session.

    Parameter order is the pre-convergence one, because the suite's existing call sites were
    written against it. Every caller in `run.py` passes by keyword anyway — a positional call is
    what let the earlier drift land silently, so the order is preserved for compatibility and
    relied on by nobody.

    `--setting-sources ''` is load-bearing: the sandbox carries a user-level mochiko install that
    would otherwise load beside the staged plugin. It drops user and project config, installed
    plugins included, while stored auth survives.
    """
    args = [
        "claude", "-p", prompt,
        "--model", model,
        "--permission-mode", PERMISSION_MODE,
        "--max-turns", str(max_turns),
        "--setting-sources", "",
        "--output-format", "stream-json" if stream else "json",
    ]
    if stream:
        args += ["--verbose"]
    if plugin is not None:
        args += ["--plugin-dir", str(plugin)]
    return args
