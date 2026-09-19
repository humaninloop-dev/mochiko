"""Shared core of the eval layer — the second D10 landing act.

Provenance: .mochiko/brainstorms/primitive-eval-harness-v2/record.md (D10: "extract the
shared core — session invocation, judge calls, grid math, report — into `evals/lib/` and
make the skill runner import it"). Vocabulary for every target: evals/README.md.
Maintainer-side advisory tooling (GI-019 trace); never shipped (GI-020).

What lives here is the mechanics each of the three targets (skill · command · persona) had
carried its own copy of:

  session    the `claude -p` argv, the stream-json parser (subagent events skipped), the
             auth / session-limit / `<synthetic>` halt detection, `die` and `sha`
  provision  `plugins/mochiko` git-archived at a ref (the working copy on opt-in)
  judge      the one-turn judge session, lenient JSON extraction, the chunked + retried
             checklist, the position-swapped pairwise read
  stats      pass^k · flaky · missing over judged replicates, and the noise-band arithmetic

What stays in each runner: the load gate and asserts, the rubric shape and its checks, the
judge prompts (a pinned instrument per target), the report renderer, and every arm/ref
convention. A runner's observable behaviour — its stored summaries and rendered reports — is
unchanged by the extraction (the byte-identical report oracle of the act's audit).

Import: the runners put `evals/` on `sys.path` (the skill runner gets it for free as the
script directory) and `from lib import session, provision, judge, stats`.
"""

import pathlib

REPO = pathlib.Path(__file__).resolve().parents[2]
EVALS = REPO / "evals"
PLUGIN = REPO / "plugins" / "mochiko"
