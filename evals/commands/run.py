#!/usr/bin/env python3
# /// script
# dependencies = ["pyyaml"]
# ///
"""Deprecated entrypoint at the pre-rename path, kept one release (D10 act 3).

The plan-only runner moved to evals/plan/run.py (act 1) and the CLI converged on
`evals/run.py <target> <subcommand> <name>` (act 3). This shim forwards
`uv run evals/commands/run.py <subcommand> ...` unchanged to evals/plan/run.py, whose own
deprecated `main` maps it onto the converged CLI — so a fill log or script that still
names this path keeps working until the shims go.
"""

import os
import pathlib
import sys

PLAN_RUNNER = pathlib.Path(__file__).resolve().parent.parent / "plan" / "run.py"
print("deprecated: evals/commands/run.py moved to evals/plan/run.py; use "
      "`uv run evals/run.py command|agent <subcommand> ...` (this path goes away next release)",
      file=sys.stderr)
os.execv(sys.executable, [sys.executable, str(PLAN_RUNNER), *sys.argv[1:]])
