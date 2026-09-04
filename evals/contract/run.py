#!/usr/bin/env python3
"""The plugin contract suite — the layer that tests what the crate never can.

Provenance: `.mochiko/brainstorms/cli-schema-delivery/record.md` D8 as amended. The suite runs
inside the Docker AI sandbox `claude-mochiko`, through the sandbox helpers `evals/run.py` already
owns; it imports them and never forks them. Maintainer-side, never shipped (GI-020).

What it asserts (D8's deterministic set):

    the `!` line executed · the version-triple line present · the closing end line present ·
    no schema file Read anywhere · absence halts · skew halts

Wave 1 ran two cases, both failure paths against the fixture plugin, because those are the ones
that do not need a converted primitive. Wave 3 adds the first converted command, `brainstorm`,
and with it the cases that need one:

    hook-input          the two hook scripts, fed real captured stdin, on the host
    converted-shape     a converted `.md`'s `!` lines against the sections its render declares
    render-ceiling      every converted primitive's renders against the inline ceiling
    absence   [fixture] the binary is off the sandbox PATH -> the run halts, nothing delivered
    skew      [fixture] the log declares a grammar the binary does not read -> the D5 halt
    brainstorm-delivery the happy path: seven blocks delivered, plus the read-back metric
    brainstorm-absence  the same halt, now with the plugin's own hooks in play
    brainstorm-skew     the staged plugin's own log is out of range
    brainstorm-hooks-off  hooks disabled: the harness path is the only guard left
    brainstorm-policy   shell execution disabled by policy — recorded, never asserted (D8)

Ten cases in all. The first three need neither a sandbox nor a session and run on the host
binary; `--host-only` runs just those, which is the cheapest gate on the hooks.

A positive assertion reads only the channels measured to carry delivered text — the session
transcript and the stream's own events. Negative assertions read the wider union that adds the
process streams, because for those breadth is strictness.

**A suite that cannot run says so.** Every prerequisite is checked before any session case, and a
missing one exits 3 with the reason. Exit 0 means every declared case ran and passed — never
"nothing happened". A failed assertion outranks a skip: if a host case fails and the sandbox is
then unreachable, the exit is 1, not 3.

    exit 0  every case ran and passed
    exit 1  a case ran and an assertion failed
    exit 3  the suite could not run (no sandbox, not authenticated, no binary) — SKIPPED

The read-back metric is reported and never gates (D8): it lands in the case's `verdict.json` and
in the summary, and it cannot set a non-zero exit code.

Usage:
    python3 evals/contract/run.py              # run the cases
    python3 evals/contract/run.py --list       # print the case list and exit
    python3 evals/contract/run.py --host-only  # only the cases that need no sandbox
"""

import argparse
import importlib.util
import json
import pathlib
import re
import shlex
import shutil
import subprocess
import sys
import uuid
from typing import NamedTuple

REPO = pathlib.Path(__file__).resolve().parents[2]
CONTRACT = REPO / "evals" / "contract"
FIXTURE = CONTRACT / "fixture" / "probe-plugin"
HOOK_INPUT = CONTRACT / "fixture" / "hook-input"
PLUGIN = REPO / "plugins" / "mochiko"
HOST_BINARY = REPO / "target" / "release" / "mochiko-cli"
WORK = REPO / "evals" / ".work"

EXIT_OK, EXIT_ASSERT, EXIT_SKIP = 0, 1, 3

# The version triple's shape (record D3 as amended): head line, then the body, then the end line.
TRIPLE_HEAD = "mochiko-cli rules "
TRIPLE_MARKERS = ("· binary ", "· grammar ", "· plugin ")
END_LINE = "mochiko-cli rules end"

# What the fixture command prints, so the model's own verdict is readable in the transcript.
PROBE_DELIVERED = "CONTRACT-PROBE: delivered"
PROBE_HALTED = "CONTRACT-PROBE: halted"

# The sandbox's own build tree. NEVER the repository's `target/`: the sandbox mounts the worktree
# at the same path the host uses, so a shared target directory means the Linux sandbox executes
# the host's macOS Mach-O binary and reports `sh: Syntax error: "(" unexpected`.
SANDBOX_TARGET_DIR = "/home/agent/mochiko-target"

# `mochiko-cli --version`, which is also the head of the version triple.
VERSION_LINE = re.compile(r"^mochiko-cli (\d+\.\d+\.\d+) · grammar (\d+)\.\.(\d+)$")

# The wave-3 pilot: the one converted command, and the seven `class: floor` rule ids its rules
# carry. Both are verified against the binary's own render before they are used as expectations,
# so a schema change breaks the check rather than silently rewriting the bar.
PILOT_COMMAND = "brainstorm"
FLOOR_IDS = frozenset(
    {
        "brainstorm.user-record-acceptance",
        "brainstorm.author-grader-default-fail",
        "brainstorm.transport-floor",
        "brainstorm.fail.record-unaccepted",
        "brainstorm.fail.unreviewed-no-waiver",
        "brainstorm.fail.survivor-undispositioned",
        "brainstorm.fail.index-mismatch",
    }
)

# The read-back metric (D8): N replicates, the bar pre-registered before the first run and
# unchanged after it. Reported, never gating.
READ_BACK_REPLICATES = 3
READ_BACK_BAR = 3

# The probe topic. It asks for one line before anything else so the metric is readable without a
# judge: the model either names the floor set it was delivered, or it does not.
PROBE_TOPIC = (
    "CONTRACT PROBE — before your first question reply with exactly one line "
    "`FLOOR: <every class: floor rule id you were delivered, comma-separated>` then stop."
)

# The pre-conversion read cost the `.md` obligated at v0.103.0: `wc -c` of `brainstorm.yaml` plus
# `common.yaml`. Abort criterion (2) compares bytes to bytes; chars are reported beside them and
# are never the criterion.
BASELINE_BYTES = 12_819
BASELINE_CHARS = 12_753
BASELINE_BYTES_WITH_LABELS = 14_349

# Wave-0 probe (e): a `!` block above this arrives as a file-path notice whose preview keeps only
# the first line, which would strand a render without its end line.
INLINE_CEILING = 30_000

# What marks a primitive as converted. The same test the dependency hook makes, and for the same
# reason: the primitive's own file is the truth, so there is no list to keep in sync with the
# conversion waves.
CONVERTED_MARK = "!`mochiko-cli rules"

# The dependency hook's presence line, keyed to its stable prefix. The trailing noun is `command`
# or `skill` depending on the registration, and `hook-input` asserts the full per-noun line.
HOOK_PRESENT_PREFIX = "mochiko-cli present · rules delivered by the"
INSTALL_LINE = "cargo install mochiko-cli"


class Check(NamedTuple):
    """One assertion's outcome.

    `pending` is a first-class status, not a quiet pass. An assertion whose subject does not exist
    until a later wave is reported as pending every run, so the case summary can never read as
    though it were asserted.
    """

    name: str
    status: str  # "ok" | "fail" | "pending" | "report"
    detail: str = ""


def ok(name: str, problem: str | None) -> Check:
    return Check(name, "ok" if problem is None else "fail", problem or "")


def pending(name: str, why: str) -> Check:
    return Check(name, "pending", why)


def report(name: str, what: str) -> Check:
    """A measured outcome that D8 records rather than asserts.

    The policy case has no gating assertion at all, and a case whose check list is empty prints as
    a clean pass — a suite reporting success for having asserted nothing. A recorded observation
    is a first-class status for the same reason `pending` is: it can never be mistaken for one.
    """
    return Check(name, "report", what)


def load_runner():
    """Import `evals/run.py` — the sandbox helpers, never a second copy of them."""
    path = REPO / "evals" / "run.py"
    spec = importlib.util.spec_from_file_location("mochiko_eval_runner", path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"{path} cannot be imported")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


# ---------------------------------------------------------------------------
# preflight
# ---------------------------------------------------------------------------

def preflight(runner) -> str | None:
    """The reason the suite cannot run, or None."""
    if shutil.which("sbx") is None:
        return "the `sbx` CLI is not on PATH — the sandbox is the suite's substrate (D8)"

    reach = subprocess.run(
        ["sbx", "exec", runner.SANDBOX, "true"],
        capture_output=True,
        text=True,
        timeout=120,
    )
    if reach.returncode != 0:
        return (
            f"the sandbox {runner.SANDBOX!r} is not reachable: "
            f"{(reach.stderr or reach.stdout).strip()[:200]}"
        )

    version = runner.sbx_sh("claude --version", timeout=120)
    if version.returncode != 0:
        return f"`claude` is not runnable in the sandbox: {version.stderr.strip()[:200]}"

    # Authentication is the one that bites silently: an unauthenticated sandbox still starts a
    # session and then returns an error result, which a careless suite reads as a failed
    # assertion rather than as "it never ran". Probe it once, cheaply.
    probe = runner.sbx_sh(
        "cd /tmp && claude -p 'reply with the single word READY' "
        "--model haiku --max-turns 1 --setting-sources '' --output-format json",
        timeout=300,
    )
    if probe.returncode != 0 or not probe.stdout.strip():
        return (
            "the sandbox is not authenticated for headless runs — run `sbx login` "
            f"(claude exited {probe.returncode}: {(probe.stderr or probe.stdout).strip()[:200]})"
        )
    try:
        result = json.loads(probe.stdout[probe.stdout.index("{"):])
    except (ValueError, json.JSONDecodeError):
        return f"the sandbox returned no JSON result: {probe.stdout.strip()[:200]}"
    if result.get("is_error") or result.get("subtype") not in (None, "success"):
        return f"the sandbox session errored: {str(result)[:200]}"

    if not FIXTURE.is_dir():
        return f"the fixture plugin is missing at {FIXTURE}"
    return None


class Sandbox(NamedTuple):
    """What the cases need to know about the substrate they run on."""

    path: str  # the sandbox's own PATH, carrying `claude` and NOT `mochiko-cli`
    binary: str  # the built binary's absolute path in the sandbox
    binary_dir: str  # the directory holding it, for prepending to PATH


def build_binary(runner) -> tuple[str | None, str | None]:
    """Build `mochiko-cli` inside the sandbox and return its path there, or a skip reason.

    Built in the sandbox rather than copied in: the sandbox is Linux and the maintainer's host is
    macOS, so a host build is the wrong architecture. This is the D4 install shape in miniature —
    the binary arrives as a developer tool on PATH, not as part of the plugin.

    Two things this gets right that a first cut did not:

    * **A sandbox-local target directory.** The sandbox mounts the worktree at the same path the
      host uses, so building into the shared `target/` leaves the host's Mach-O binary in place
      and the sandbox executes it — the failure reads `sh: Syntax error: "(" unexpected`, which
      looks like a shell bug rather than an architecture mismatch.
    * **Verification by running it.** `test -x` passes on a binary of the wrong architecture.
      Running `--version` and parsing the line is the only check that proves the thing works, and
      it doubles as a read of the grammar range the D5 assertions depend on.
    """
    cargo = runner.sbx_sh("command -v cargo", timeout=120)
    if cargo.returncode != 0 or not cargo.stdout.strip():
        return None, "no `cargo` in the sandbox, so `mochiko-cli` cannot be built there"

    build = runner.sbx_sh(
        "cargo build --release -p mochiko-cli "
        f"--manifest-path {shlex.quote(str(REPO / 'Cargo.toml'))} "
        f"--target-dir {shlex.quote(SANDBOX_TARGET_DIR)} 2>&1 | tail -5",
        timeout=1800,
    )
    binary = f"{SANDBOX_TARGET_DIR}/release/mochiko-cli"
    version = runner.sbx_sh(f"{shlex.quote(binary)} --version", timeout=120)
    if version.returncode != 0:
        return None, (
            f"the sandbox build produced no runnable binary "
            f"(exit {version.returncode}: {(version.stderr or version.stdout).strip()[:200]}); "
            f"build tail: {build.stdout.strip()[:200]}"
        )
    line = version.stdout.strip().splitlines()[0] if version.stdout.strip() else ""
    if not VERSION_LINE.match(line):
        return None, f"`{binary} --version` printed {line!r}, not the version line"
    return binary, None


# ---------------------------------------------------------------------------
# assertions
# ---------------------------------------------------------------------------

def events_of(stdout: str) -> list:
    out = []
    for line in stdout.splitlines():
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            out.append(json.loads(line))
        except json.JSONDecodeError:
            continue
    return out


def transcript_text(events: list) -> str:
    """Everything the model saw or said, as one string."""
    chunks = []
    for event in events:
        chunks.append(json.dumps(event, ensure_ascii=False))
    return "\n".join(chunks)


def tool_uses(events: list) -> list:
    uses = []
    for event in events:
        message = event.get("message") or {}
        for block in message.get("content") or []:
            if isinstance(block, dict) and block.get("type") == "tool_use":
                uses.append(block)
    return uses


# The four helpers below belong to the DELIVERY path — the wave-3 per-primitive cases, where the
# `!` line exits 0, the expansion completes, and a model turn actually runs. They are unused by
# the two wave-1 halt cases, which are keyed to the measured harness-level shape instead (see
# `assert_no_model_turn`). Kept rather than deleted: they are four of D8's six assertions, and
# deleting them would leave the set looking smaller than it is.


def assert_bang_ran(text: str) -> str | None:
    """The `!` slot was expanded rather than passed through or denied.

    Delivery path only. In a halt case the block never reaches the model at all, so this would
    report "the probe command never reached the model" for the very run it is meant to pass.
    """
    if "CONTRACT-BLOCK-BEGIN" not in text:
        return "the probe command never reached the model"
    if "!`mochiko-cli rules" in text:
        return "the `!` line was passed through literally — preprocessing did not run"
    if "Permission to use Bash has been denied" in text:
        return "the `!` line was denied — the allowed-tools grant is not in force"
    return None


def assert_version_triple(text: str) -> str | None:
    if TRIPLE_HEAD not in text or not all(marker in text for marker in TRIPLE_MARKERS):
        return "no version-triple line reached the model"
    return None


def assert_no_version_triple(text: str) -> str | None:
    if all(marker in text for marker in TRIPLE_MARKERS):
        return "a version-triple line reached the model, and none should have"
    return None


def assert_end_line(text: str) -> str | None:
    if END_LINE not in text:
        return "no closing `mochiko-cli rules end` line reached the model"
    return None


def is_schema_path(path: str) -> bool:
    """Whether a path names a shipped schema file.

    Broader than "under `plugins/mochiko/schemas/`" on purpose. From wave 3 the plugin under test
    is a staged **copy** at `evals/.work/contract-<case>-<id>/mochiko/`, so a fallback Read of the
    copy's own `schemas/brainstorm.yaml` matches neither the repository path nor the `schema.yaml`
    suffix — the assertion would have passed a run that did exactly the thing no-fallback exists to
    rule out. Any `.yaml` under a `schemas/` directory counts, wherever it was staged.
    """
    return path.endswith("schema.yaml") or (
        "/schemas/" in path and path.endswith(".yaml")
    )


def assert_no_schema_read(events: list) -> str | None:
    """No schema file was read anywhere in the run (D8; run-wide from wave 6).

    `Read` is the tool D8 names, and it is also the one a fallback would reach for. A shell read of
    the same file would be the same failure wearing a different hat, so a Bash command naming a
    schema path counts too — the assertion is about the posture, not about which tool carried it.
    """
    for use in tool_uses(events):
        name, args = use.get("name"), (use.get("input") or {})
        if name in ("Read", "NotebookRead"):
            path = str(args.get("file_path", ""))
            if is_schema_path(path):
                return f"a schema file was Read: {path}"
        elif name == "Bash":
            # Routed through `is_schema_path` so the two limbs cannot drift: a separate regex
            # here missed `skills/<name>/schema.yaml`, which the `Read` limb caught by suffix.
            command = str(args.get("command", ""))
            for token in re.findall(r"[\w./-]+\.yaml", command):
                if is_schema_path(token):
                    return f"a schema file was read through the shell: {token}"
    return None


def assert_halted(text: str) -> str | None:
    """The run reported a delivery failure rather than proceeding."""
    if PROBE_DELIVERED in text:
        return "the run proceeded as if the rules had been delivered"
    return None


def result_event(events: list) -> dict | None:
    for event in reversed(events):
        if event.get("type") == "result":
            return event
    return None


def assert_no_model_turn(events: list) -> str | None:
    """The halt happened before the model ran at all.

    Measured shape: a failing `!` line aborts the expansion, the harness injects the shell's
    stderr as a user message, and the session ends with `num_turns` 0 and an empty `result`.
    `claude` still exits 0 and `is_error` is false, so a suite that read only the exit code would
    call this a clean run.
    """
    result = result_event(events)
    if result is None:
        return "the session produced no result event"
    turns = result.get("num_turns")
    if turns != 0:
        return f"expected 0 model turns, got {turns!r}"
    if (result.get("result") or "").strip():
        return f"the model produced a result: {str(result.get('result'))[:120]!r}"
    return None


def assert_no_assistant_event(events: list) -> str | None:
    if any(event.get("type") == "assistant" for event in events):
        return "an assistant turn ran; this halt is meant to fire before the model"
    return None


def local_command_stderr(events: list) -> list[str]:
    """Every `<local-command-stderr>` block the harness injected as a user message."""
    blocks = []
    for event in events:
        if event.get("type") != "user":
            continue
        content = (event.get("message") or {}).get("content")
        chunks = []
        if isinstance(content, str):
            chunks = [content]
        elif isinstance(content, list):
            for block in content:
                if isinstance(block, str):
                    chunks.append(block)
                elif isinstance(block, dict) and isinstance(block.get("text"), str):
                    chunks.append(block["text"])
        for chunk in chunks:
            blocks += re.findall(
                r"<local-command-stderr>(.*?)</local-command-stderr>", chunk, re.S
            )
    return blocks


def assert_local_command_stderr(events: list, *fragments: str) -> str | None:
    """The harness injected the failing command's stderr, carrying these fragments."""
    blocks = local_command_stderr(events)
    if not blocks:
        return "no `<local-command-stderr>` message was injected"
    joined = "\n".join(blocks)
    missing = [fragment for fragment in fragments if fragment not in joined]
    if missing:
        return f"the injected stderr is missing {missing}: {joined.strip()[:300]!r}"
    return None


def assert_message(text: str, fragment: str) -> str | None:
    if fragment not in text:
        return f"{fragment!r} never reached the model"
    return None


def assert_skew_halt_on_stderr(proc, fragment: str) -> str | None:
    """The binary's own D5 behaviour, read off the process rather than the transcript.

    The halt message goes to stderr, stdout stays empty, and the exit code is 3. Asserting this
    directly keeps the case honest whatever Claude Code does with stderr: the transcript
    assertion says the message reached the model, and this one says the binary wrote it.
    """
    problems = []
    if proc.returncode != 3:
        problems.append(f"expected exit 3 from the skew log, got {proc.returncode}")
    if proc.stdout.strip():
        problems.append(f"stdout was not empty: {proc.stdout.strip()[:120]!r}")
    if fragment not in proc.stderr:
        problems.append(f"{fragment!r} is not on stderr: {proc.stderr.strip()[:200]!r}")
    return "; ".join(problems) if problems else None


# ---------------------------------------------------------------------------
# cases
# ---------------------------------------------------------------------------

def sandbox_path(runner) -> tuple[str | None, str | None]:
    """The sandbox's own `PATH`, and whether `mochiko-cli` is absent from it.

    The absence case needs a PATH that carries `claude` but not `mochiko-cli`. Hand-writing one
    (`/usr/bin:/bin`) removes `claude` too — it lives in `~/.local/bin` — and the run dies with
    `env: 'claude': No such file or directory` before a session ever starts, which is a broken
    harness rather than the halt the case is about. So the PATH is read from the sandbox, and the
    binary's absence from it is verified rather than assumed.
    """
    probe = runner.sbx_sh(
        'printf %s "$PATH"; echo; command -v mochiko-cli || true', timeout=120
    )
    if probe.returncode != 0 or not probe.stdout.strip():
        return None, f"the sandbox PATH could not be read: {probe.stderr.strip()[:200]}"
    lines = probe.stdout.splitlines()
    value = lines[0].strip()
    found = "\n".join(lines[1:]).strip()
    if not value:
        return None, "the sandbox reported an empty PATH"
    if found:
        return None, (
            f"`mochiko-cli` is already on the sandbox PATH at {found!r}; the absence case "
            "cannot be run against it"
        )
    return value, None


class Staged(NamedTuple):
    """One case's working directory: the staged plugin, and where its evidence lands."""

    root: pathlib.Path
    plugin: pathlib.Path


def stage(case: str, source: pathlib.Path = FIXTURE) -> Staged:
    """Copy a plugin into `evals/.work/`, where the sandbox sees the same path.

    `source` is the fixture probe plugin for the wave-1 cases and the real `plugins/mochiko/` for
    the wave-3 ones — the whole plugin, its migration log and its hooks included, which is what
    makes the staged copy a faithful subject: the `!` lines resolve their log through
    `${CLAUDE_PLUGIN_ROOT}/migrations`, so a case that wants to perturb the log perturbs this copy
    and never the repository.

    The directory is also the case's evidence directory. D8 wants the transcript on disk, not a
    pass/fail line: a case that fails has to be readable afterwards without re-running it, and a
    case that passes has to be auditable by someone who was not here.
    """
    root = WORK / f"contract-{case}-{uuid.uuid4().hex[:8]}"
    root.mkdir(parents=True, exist_ok=True)
    plugin = root / source.name
    shutil.copytree(source, plugin)
    return Staged(root, plugin)


def write_evidence(root: pathlib.Path, name: str, text: str) -> None:
    (root / name).write_text(text, encoding="utf-8")


def write_verdict(root: pathlib.Path, case: str, checks: list, extra: dict) -> None:
    """The case's machine-readable outcome, beside its transcript."""
    payload = {
        "case": case,
        "ran": True,
        "failed": [c.name for c in checks if c.status == "fail"],
        "pending": [c.name for c in checks if c.status == "pending"],
        "reported": [c.name for c in checks if c.status == "report"],
        "checks": [
            {"name": c.name, "status": c.status, "detail": c.detail} for c in checks
        ],
        **extra,
    }
    write_evidence(root, "verdict.json", json.dumps(payload, indent=2) + "\n")


def write_skew_log(root: pathlib.Path) -> pathlib.Path:
    """A migration log the binary declares itself unable to read (record D5)."""
    log = root / "migrations"
    log.mkdir(parents=True, exist_ok=True)
    (log / "0001-skew.yaml").write_text(
        "grammar: 99\n"
        "id: 0001-skew\n"
        "sequence: 1\n"
        "intent: A log from a grammar this binary does not read.\n"
        'hash: "sha256:0000000000000000000000000000000000000000000000000000000000000000"\n'
        "changes: []\n",
        encoding="utf-8",
    )
    return log


class Probed(NamedTuple):
    events: list
    proc: subprocess.CompletedProcess


def run_probe(
    runner,
    staged: Staged,
    *,
    path_env: str,
    log_dir: str | None,
    prompt: str = "/mochiko-contract-probe:rules-probe",
    max_turns: int = 3,
    settings: dict | None = None,
    tag: str = "",
) -> Probed:
    """One headless run of a staged plugin's command, with its evidence written to disk.

    Wrapped in `sh -c` through `runner.sbx_sh` rather than passed as argv: `sbx exec` rejects an
    empty argv element, and `claude_args` carries one (`--setting-sources ''`, which is what
    keeps the sandbox's user-level plugin install out of the run).

    `settings` is appended as `--settings '<json>'` rather than written into the workspace,
    because the CLI applies `--settings` even under an empty `--setting-sources` — which is what
    lets the policy and hooks-off cases set one flag without dragging the sandbox's own user
    configuration back into the run. `claude_args` is used, never forked; the flag is appended to
    what it returns.

    `tag` distinguishes the evidence files of several runs sharing one case directory, which is
    what the three delivery replicates need.
    """
    args = runner.claude_args(prompt, "sonnet", max_turns, True, staged.plugin)
    if settings is not None:
        args += ["--settings", json.dumps(settings, separators=(",", ":"))]
    env = [f"PATH={shlex.quote(path_env)}"]
    if log_dir is not None:
        env.append(f"MOCHIKO_MIGRATIONS={shlex.quote(log_dir)}")
    workspace = f"/tmp/contract-{uuid.uuid4().hex[:8]}"
    script = (
        f"mkdir -p {workspace} && cd {workspace} && "
        f"env {' '.join(env)} {shlex.join(args)}"
    )
    proc = runner.sbx_sh(script)
    runner.sbx_sh(f"rm -rf {workspace}", timeout=60)

    write_evidence(staged.root, f"argv{tag}.txt", "\n".join(args) + "\n")
    write_evidence(staged.root, f"script{tag}.sh", script + "\n")
    write_evidence(staged.root, f"stream{tag}.jsonl", proc.stdout)
    write_evidence(staged.root, f"stderr{tag}.txt", proc.stderr)
    return Probed(events_of(proc.stdout), proc)


def case_absence(runner, sandbox: "Sandbox") -> tuple[list, pathlib.Path]:
    """The binary is off PATH: nothing can be delivered, and the run must halt.

    The measured shape (sandbox `claude-mochiko`, this wave): the `!` line fails, the harness
    aborts the expansion and injects the shell's stderr as a user message, and **no model turn
    happens at all** — `num_turns` 0, empty `result`, `is_error` false, `claude` exit 0. The
    `.md`'s halt clause never executes, because the model never runs. The halt is real and it is
    earlier than the clause: the assertions below are keyed to where it actually fires.

    Wave 1 carried a `pending` assertion here — "the install line reaches the model" — waiting on
    hooks that did not exist. They exist now, but they ship in `plugins/mochiko/` and this case
    loads the fixture plugin, which has none. The assertion therefore resolved into
    `brainstorm-absence`, which stages the real plugin, rather than into this case.
    """
    staged = stage("absence")
    probed = run_probe(runner, staged, path_env=sandbox.path, log_dir=None)
    events, text = probed.events, transcript_text(probed.events)
    checks = [
        ok("no model turn ran", assert_no_model_turn(events)),
        ok("no assistant event", assert_no_assistant_event(events)),
        ok(
            "the harness injected the shell's stderr, naming the missing binary",
            assert_local_command_stderr(events, "mochiko-cli", "command not found"),
        ),
        ok("no version triple reached the model", assert_no_version_triple(text)),
        ok("no schema file was Read", assert_no_schema_read(events)),
        ok("nothing was delivered", assert_halted(text)),
    ]
    write_verdict(
        staged.root,
        "absence",
        checks,
        {
            "shape": "harness-level halt before any model turn",
            "claude_exit": probed.proc.returncode,
            "result_event": result_event(events),
            "local_command_stderr": local_command_stderr(events),
        },
    )
    return checks, staged.root


def case_skew(runner, sandbox: "Sandbox") -> tuple[list, pathlib.Path]:
    """The log is out of the binary's grammar range: the D5 halt, not a partial read.

    Two assertions of the same halt, from opposite sides. The direct run says the binary wrote
    it — exit 3, the message on stderr, stdout empty. The probe run says the harness carried it:
    the `!` line exits non-zero, so the expansion aborts exactly as in the absence case, and the
    injected `<local-command-stderr>` carries the D5 wording verbatim.
    """
    staged = stage("skew")
    log = write_skew_log(staged.root)

    # The binary's own behaviour first, on the channel it actually writes to.
    direct = runner.sbx_sh(
        f"env PATH={shlex.quote(sandbox.binary_dir + ':' + sandbox.path)} "
        f"MOCHIKO_MIGRATIONS={shlex.quote(str(log))} "
        f"mochiko-cli rules brainstorm --section preamble",
        timeout=120,
    )
    write_evidence(
        staged.root,
        "direct-binary.txt",
        f"exit: {direct.returncode}\n--- stdout ---\n{direct.stdout}"
        f"--- stderr ---\n{direct.stderr}",
    )

    probed = run_probe(
        runner,
        staged,
        path_env=f"{sandbox.binary_dir}:{sandbox.path}",
        log_dir=str(log),
    )
    events, text = probed.events, transcript_text(probed.events)

    checks = [
        ok(
            "the binary halts on stderr with exit 3",
            assert_skew_halt_on_stderr(direct, "cargo install mochiko-cli"),
        ),
        ok("no model turn ran", assert_no_model_turn(events)),
        ok("no assistant event", assert_no_assistant_event(events)),
        ok(
            "the harness injected the D5 halt message",
            assert_local_command_stderr(
                events, "cargo install mochiko-cli", "grammar 99"
            ),
        ),
        ok("no version triple reached the model", assert_no_version_triple(text)),
        ok("no schema file was Read", assert_no_schema_read(events)),
        ok("nothing was delivered", assert_halted(text)),
    ]
    write_verdict(
        staged.root,
        "skew",
        checks,
        {
            "shape": "harness-level halt before any model turn",
            "claude_exit": probed.proc.returncode,
            "direct_binary_exit": direct.returncode,
            "direct_binary_stderr": direct.stderr.strip(),
            "result_event": result_event(events),
            "local_command_stderr": local_command_stderr(events),
        },
    )
    return checks, staged.root


# ---------------------------------------------------------------------------
# wave 3 — the converted primitive
# ---------------------------------------------------------------------------

MINIMAL_PATH = "/usr/bin:/bin:/usr/sbin:/sbin"


def host_sh(script: str, *, env: dict | None = None, stdin: str | None = None,
            timeout: int = 120) -> subprocess.CompletedProcess:
    """One shell command on the host. The hook scripts are POSIX `sh`, so `sh -c` is the subject."""
    return subprocess.run(
        ["sh", "-c", script],
        capture_output=True,
        text=True,
        timeout=timeout,
        env=env,
        input=stdin,
    )


def host_binary() -> tuple[str | None, str | None]:
    """The host's `mochiko-cli`, verified by running it, or the reason there isn't one.

    Same lesson as the sandbox build: `test -x` passes on a binary that cannot run, so the check
    is `--version` and a parse of the line it prints.
    """
    candidates = [str(HOST_BINARY)] if HOST_BINARY.is_file() else []
    found = shutil.which("mochiko-cli")
    if found:
        candidates.append(found)
    for candidate in candidates:
        probe = host_sh(f"{shlex.quote(candidate)} --version")
        line = probe.stdout.strip().splitlines()[0] if probe.stdout.strip() else ""
        if probe.returncode == 0 and VERSION_LINE.match(line):
            return candidate, None
    return None, (
        f"no runnable `mochiko-cli` on the host — build one with "
        f"`cargo build --release -p mochiko-cli` (looked at {HOST_BINARY} and PATH)"
    )


def render(binary: str, primitive: str, section: str, plugin_root: pathlib.Path
           ) -> subprocess.CompletedProcess:
    """One section render, direct from the binary, against a plugin root's own log."""
    return host_sh(
        f"{shlex.quote(binary)} rules {shlex.quote(primitive)} "
        f"--section {shlex.quote(section)} --plugin-root {shlex.quote(str(plugin_root))}"
    )


class Section(NamedTuple):
    id: str
    title: str
    rules: int


def parse_preamble(text: str) -> list[Section]:
    """The section list the preamble prints, which is the render's own statement of what follows.

    Read rather than hard-coded: the expected block set and the expected end-line counts then come
    from the same source the delivery does, so a schema change breaks a check instead of quietly
    rewriting the expectation.
    """
    sections, inside = [], False
    for line in text.splitlines():
        if line.strip() == "sections":
            inside = True
            continue
        if inside:
            if not line.startswith("- "):
                break
            parts = [part.strip() for part in line[2:].split(" · ")]
            if len(parts) < 3:
                continue
            count = re.match(r"(\d+) rules?$", parts[-1])
            if count:
                sections.append(Section(parts[0], " · ".join(parts[1:-1]), int(count.group(1))))
    return sections


def rendered_floor_ids(binary: str, primitive: str, plugin_root: pathlib.Path) -> set[str]:
    """Every `class: floor` rule id the binary renders for a primitive.

    The read-back bar names seven ids, pre-registered before the first session. `FLOOR_IDS` is
    that pre-registration and stays a written-down constant — a bar derived from the thing it
    grades is not a bar. What it needs is a cross-check: a floor rule added at wave 4 would
    otherwise leave the bar quietly grading six of seven, and the metric would keep reporting a
    clean 3/3 while asking the wrong question. `converted-shape` compares the two and goes red on
    any difference.

    Shape read off the render: an id line `### <id>`, then an attribute line carrying `class:`.
    """
    preamble = render(binary, primitive, "preamble", plugin_root)
    if preamble.returncode != 0:
        return set()
    found, pending_id = set(), None
    for section in [s.id for s in parse_preamble(preamble.stdout)]:
        out = render(binary, primitive, section, plugin_root)
        if out.returncode != 0:
            continue
        for line in out.stdout.splitlines():
            if line.startswith("### "):
                pending_id = line[4:].strip()
            elif pending_id and line.startswith("[") and "class:" in line:
                if "class: floor" in line:
                    found.add(pending_id)
                pending_id = None
    return found


def converted_primitives(plugin_root: pathlib.Path) -> list[tuple[str, str, pathlib.Path]]:
    """Every primitive whose rules come from the binary, as `(kind, name, file)`.

    The same test the dependency hook makes — the primitive's own file carries the `!` line — so
    the suite and the hook can never disagree about what is converted.
    """
    out = []
    for path in sorted((plugin_root / "commands").glob("*.md")):
        if CONVERTED_MARK in path.read_text(encoding="utf-8"):
            out.append(("command", path.stem, path))
    for path in sorted((plugin_root / "skills").glob("*/SKILL.md")):
        if CONVERTED_MARK in path.read_text(encoding="utf-8"):
            out.append(("skill", path.parent.name, path))
    return out


def unconverted_primitive(plugin_root: pathlib.Path, kind: str) -> str | None:
    """One primitive the hook must leave alone — the transition clause's side of the check."""
    if kind == "command":
        for path in sorted((plugin_root / "commands").glob("*.md")):
            if CONVERTED_MARK not in path.read_text(encoding="utf-8"):
                return path.stem
        return None
    for path in sorted((plugin_root / "skills").glob("*/SKILL.md")):
        if CONVERTED_MARK not in path.read_text(encoding="utf-8"):
            return path.parent.name
    return None


def swap_plugin_log(plugin: pathlib.Path) -> pathlib.Path:
    """Replace a staged plugin's own migration log with one out of the binary's grammar range.

    This — not `MOCHIKO_MIGRATIONS` — is the lever for a converted command, because its `!` lines
    pass `--plugin-root "${CLAUDE_PLUGIN_ROOT}"`, and the resolution order is `--log-dir` ›
    `--plugin-root <root>/migrations` › `MOCHIKO_MIGRATIONS` › `./migrations`. The environment
    variable loses to the flag the command actually passes, so a case that set it would leave the
    real log in play and quietly test nothing.
    """
    log = plugin / "migrations"
    if log.exists():
        shutil.rmtree(log)
    log.mkdir(parents=True)
    (log / "0001-skew.yaml").write_text(
        "grammar: 99\n"
        "id: 0001-skew\n"
        "sequence: 1\n"
        "intent: A log from a grammar this binary does not read.\n"
        'hash: "sha256:0000000000000000000000000000000000000000000000000000000000000000"\n'
        "changes: []\n",
        encoding="utf-8",
    )
    return log


# ---------------------------------------------------------------------------
# what the model saw
# ---------------------------------------------------------------------------

def session_id_of(events: list) -> str | None:
    init = init_event(events)
    return (init or {}).get("session_id")


def json_strings(node, out: list) -> None:
    if isinstance(node, str):
        out.append(node)
    elif isinstance(node, dict):
        for value in node.values():
            json_strings(value, out)
    elif isinstance(node, list):
        for value in node:
            json_strings(value, out)


def transcript_plaintext(raw: str) -> str:
    """Every string value in a session transcript JSONL, concatenated.

    Parsed rather than grepped: the transcript is JSON, so a raw substring search would have to
    reason about escaping, and the rendered blocks are full of `·` separators and newlines. Walking
    the values and joining the strings gives back exactly the text as the session held it.
    """
    chunks = []
    for line in raw.splitlines():
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            json_strings(json.loads(line), chunks)
        except json.JSONDecodeError:
            continue
    return "\n".join(chunks)


def fetch_transcript(runner, staged: Staged, session_id: str, tag: str = "") -> tuple[str, str]:
    """Copy a session's transcript JSONL out of the sandbox, and return its text and its path.

    **This is where the delivered rules actually are.** `--output-format stream-json` does not
    carry the expanded prompt: the seven rendered blocks appear in no stream row at all (measured
    on a host fire of the real plugin, 2026-09-04). They are in the session transcript, which the
    hook stdin names as `transcript_path` and which lives under the session user's
    `~/.claude/projects/`. Every assertion about what the model was given therefore reads this
    file, and a copy of it lands in the evidence directory so the claim is auditable later.

    The file is found by globbing the session id rather than by rebuilding the project-directory
    name from the working directory, which is a lossy munge of path separators and dots.
    """
    if not session_id:
        return "", ""
    found = runner.sbx_sh(
        f"ls ~/.claude/projects/*/{shlex.quote(session_id)}.jsonl 2>/dev/null | head -n 1",
        timeout=120,
    )
    path = found.stdout.strip()
    if not path:
        return "", ""
    got = runner.sbx_sh(f"cat {shlex.quote(path)}", timeout=300)
    if got.returncode != 0:
        return "", path
    write_evidence(staged.root, f"transcript{tag}.jsonl", got.stdout)
    return transcript_plaintext(got.stdout), path


def user_text(events: list) -> str:
    """Every user-role message, concatenated — where an expanded prompt arrives."""
    chunks = []
    for event in events:
        if event.get("type") != "user":
            continue
        content = (event.get("message") or {}).get("content")
        if isinstance(content, str):
            chunks.append(content)
        elif isinstance(content, list):
            for block in content:
                if isinstance(block, str):
                    chunks.append(block)
                elif isinstance(block, dict) and isinstance(block.get("text"), str):
                    chunks.append(block["text"])
    return "\n".join(chunks)


def final_assistant_text(events: list) -> str:
    for event in reversed(events):
        if event.get("type") != "assistant":
            continue
        content = ((event.get("message") or {}).get("content")) or []
        parts = [
            block["text"]
            for block in content
            if isinstance(block, dict) and isinstance(block.get("text"), str)
        ]
        if parts:
            return "\n".join(parts)
    result = result_event(events)
    return str((result or {}).get("result") or "")


def asserted_output(probed: Probed, transcript: str) -> str:
    """The two channels measured to carry what the session actually delivered.

    **This is what a positive assertion may read, and nothing wider.** Across every recorded run
    the two channels that carry delivered text are the session transcript and the stream's own
    events; the process streams never do. Including `proc.stderr` in an assertion would mean a
    regression where a hook's message stops reaching the session but still lands on the binary's
    stderr keeps a check named "the install line reached the session" green — the false pass the
    keying discipline exists to rule out. The process streams stay in `channels_of` as evidence,
    where they cost nothing and prove something.

    Negative assertions read `session_output_with` instead: for those, breadth is strictness.
    """
    return "\n".join([transcript_text(probed.events), transcript])


def session_output(probed: Probed) -> str:
    """Everything the run produced, whatever channel carried it.

    The widest text available, and therefore the right subject for a **negative** assertion —
    "this string appears nowhere" is a stronger claim the more places it looks. Positive
    assertions must use `asserted_output`, which is narrowed to the channels measured to carry
    delivered content.
    """
    return "\n".join(
        [transcript_text(probed.events), probed.proc.stdout or "", probed.proc.stderr or ""]
    )


def session_output_with(probed: Probed, transcript: str) -> str:
    """The widest union, for negative assertions only."""
    return "\n".join([session_output(probed), transcript])


def channels_of(probed: Probed, fragment: str, transcript: str = "") -> list[str]:
    """Which channels carried a fragment. Reported, never asserted.

    The channel names are the ones that turned out to matter: the harness's own injection, the
    session transcript (which carries what the stream does not), and the raw process streams.
    """
    found = []
    if any(fragment in block for block in local_command_stderr(probed.events)):
        found.append("<local-command-stderr>")
    if fragment in user_text(probed.events):
        found.append("stream-user-message")
    if transcript and fragment in transcript:
        found.append("session-transcript")
    if fragment in transcript_text(probed.events):
        found.append("stream-events")
    if fragment in (probed.proc.stderr or ""):
        found.append("process-stderr")
    return found


def assert_in_session(probed: Probed, fragment: str, transcript: str = "") -> str | None:
    """A positive assertion, keyed to the measured channels only (see `asserted_output`)."""
    if fragment not in asserted_output(probed, transcript):
        return (
            f"{fragment!r} is in neither the session transcript nor the stream events"
            + (
                f" (it is on the process streams: {sorted(set(channels_of(probed, fragment)))})"
                if fragment in session_output(probed)
                else ""
            )
        )
    return None


# The measured shape of a hook-blocked expansion (sandbox `claude-mochiko`, 2026-09-04). The
# harness puts its own notice in the `result` field, prefixed like this and followed by the
# blocking script's path and the message the script wrote to stderr.
HOOK_BLOCK_PREFIX = "UserPromptExpansion operation blocked by hook:"


def assert_halt_before_model(events: list) -> str | None:
    """Nothing was delivered and no model turn ran, whichever limb fired first.

    Two halt shapes exist, both measured rather than assumed, and the difference is where the
    `result` field comes from.

    * **Wave 1, the failing `!` line:** the expansion aborts, the harness injects the shell's
      stderr as a user message, and `result` is empty.
    * **Wave 3, the dependency hook:** the hook exits 2 *before* expansion, no
      `<local-command-stderr>` block is injected at all, and `result` carries the harness's own
      notice — `UserPromptExpansion operation blocked by hook: [<script>]: <message>`.

    Both are `num_turns: 0`, no assistant event, `is_error: false`, `claude` exit 0. So a
    non-empty `result` is only acceptable when it is that notice: anything else in that field is
    the model having spoken, which is the failure this assertion exists to catch.
    """
    if any(event.get("type") == "assistant" for event in events):
        return "an assistant turn ran; this halt is meant to fire before the model"
    result = result_event(events)
    if result is None:
        return None
    turns = result.get("num_turns")
    if turns not in (0, None):
        return f"expected 0 model turns, got {turns!r}"
    text = (result.get("result") or "").strip()
    if text and not text.startswith(HOOK_BLOCK_PREFIX):
        return f"the model produced a result: {text[:160]!r}"
    return None


def halt_shape(probed: Probed) -> dict:
    """The measured shape of a halt, recorded per case so the assertion can be keyed to it."""
    result = result_event(probed.events)
    return {
        "event_types": sorted({event.get("type") for event in probed.events if event.get("type")}),
        "claude_exit": probed.proc.returncode,
        "num_turns": (result or {}).get("num_turns"),
        "is_error": (result or {}).get("is_error"),
        "subtype": (result or {}).get("subtype"),
        "result_text": str((result or {}).get("result") or "")[:300],
        "local_command_stderr": local_command_stderr(probed.events),
        "process_stderr": (probed.proc.stderr or "").strip()[:600],
    }


def init_event(events: list) -> dict | None:
    for event in events:
        if event.get("type") == "system" and event.get("subtype") == "init":
            return event
    return None


def assert_slash_commands(events: list, expected: list[str]) -> str | None:
    """The plugin's commands registered as commands — the wave-0 manifest quirk, re-verified.

    Wave 0 saw a `commands` directory-string manifest register a probe plugin's command files as
    one skill under `--plugin-dir`, while the identical form worked for the real plugin. This is
    the re-verification on the real plugin the record asked for.
    """
    init = init_event(events)
    if init is None:
        return "the session produced no init event"
    listed = set(init.get("slash_commands") or [])
    missing = [name for name in expected if name not in listed]
    if missing:
        return f"the init event's slash_commands is missing {missing}"
    return None


# ---------------------------------------------------------------------------
# the read-back metric — reported, never gating
# ---------------------------------------------------------------------------

FLOOR_LINE = re.compile(r"^\s*\**FLOOR:\**\s*(.*)$", re.M)


def score_read_back(text: str) -> tuple[list[str], bool]:
    """One replicate's floor read-back: the token list it named, and whether it is exactly right.

    An id counts bare or wrapped in backticks; every other decoration is a miss (lead ruling,
    2026-09-04). Set equality against `FLOOR_IDS` — every id present, nothing else, no partial
    credit — and a missing `FLOOR:` line is a failed replicate rather than a harness error.
    """
    match = FLOOR_LINE.search(text)
    if not match:
        return [], False
    tokens = []
    for raw in match.group(1).split(","):
        token = raw.strip()
        if token.startswith("`") and token.endswith("`") and len(token) > 1:
            token = token[1:-1].strip()
        if token:
            tokens.append(token)
    return tokens, set(tokens) == set(FLOOR_IDS)


def delivered_blocks(text: str, primitive: str) -> dict[str, str]:
    """Each delivered block, head line through end line, keyed by section id.

    This is the read-cost measurement's subject: what actually arrived, not what the binary would
    print if asked again.
    """
    pattern = re.compile(
        r"(mochiko-cli rules " + re.escape(primitive) + r" · section (\S+) · binary .*?"
        r"mochiko-cli rules end · " + re.escape(primitive) + r" · \2 · \d+ rules)",
        re.S,
    )
    return {match.group(2): match.group(1) for match in pattern.finditer(text)}


def end_line_counts(text: str, primitive: str) -> dict[str, int]:
    pattern = re.compile(
        r"mochiko-cli rules end · " + re.escape(primitive) + r" · (\S+) · (\d+) rules"
    )
    return {match.group(1): int(match.group(2)) for match in pattern.finditer(text)}


def head_line_sections(text: str, primitive: str) -> set[str]:
    """The sections whose version-triple head line is present.

    Placeholder captures are dropped. The command's own halt clause quotes the head line's shape —
    `mochiko-cli rules brainstorm · section <id> · binary <v> · grammar <g> · plugin <p>` — and
    the clause travels with the expanded prompt, so a naive match counts the instructions as a
    delivered block. It never affects the delivery assertions, which ask whether each *expected*
    id is present and `<id>` is not one of them, but it does inflate a count, and a count that
    reads `1 of 7` when nothing was delivered is worse than no count at all.
    """
    pattern = re.compile(
        r"mochiko-cli rules " + re.escape(primitive) + r" · section (\S+) · binary \S+ "
        r"· grammar \S+ · plugin \S+"
    )
    return {
        match.group(1)
        for match in pattern.finditer(text)
        if "<" not in match.group(1) and ">" not in match.group(1)
    }


# ---------------------------------------------------------------------------
# host cases — no sandbox, no session
# ---------------------------------------------------------------------------

def load_captures() -> tuple[dict, list[str]]:
    """The committed hook-stdin captures, indexed by event, plus any note about their provenance.

    The captures are real hook input, sanitized, and they are the *shape* source: each row below
    substitutes only the one field that names the primitive, so the field set stays whatever the
    platform actually sends and the case matrix stays this file's business.
    """
    captures, notes = {}, []
    for path in sorted(HOOK_INPUT.glob("*.json")) if HOOK_INPUT.is_dir() else []:
        try:
            payload = json.loads(path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError) as err:
            notes.append(f"{path.name} is not readable JSON: {err}")
            continue
        event = payload.get("hook_event_name")
        if event:
            captures.setdefault(event, payload)
    if "SessionStart" not in captures:
        # Approved fallback (lead ruling, 2026-09-04): a labelled synthesis from the documented
        # field set. `session-start.sh` reads only `cwd` from stdin, so the two SessionStart rows
        # would otherwise go untested entirely.
        captures["SessionStart"] = {
            "session_id": "00000000-0000-0000-0000-000000000000",
            "hook_event_name": "SessionStart",
            "cwd": "/tmp/contract-session-start",
            "permission_mode": "default",
            "source": "startup",
        }
        notes.append(
            "no captured SessionStart stdin was committed; this row used a synthesized payload "
            "built from the documented field set (record F14)"
        )
    return captures, notes


def run_hook(script: pathlib.Path, payload: dict, *, path_env: str,
             plugin_root: pathlib.Path, home: pathlib.Path) -> subprocess.CompletedProcess:
    env = {"PATH": path_env, "CLAUDE_PLUGIN_ROOT": str(plugin_root), "HOME": str(home)}
    return host_sh(shlex.quote(str(script)), env=env, stdin=json.dumps(payload))


def case_hook_input(runner, sandbox) -> tuple[list, pathlib.Path]:
    """The two hook scripts, fed real captured stdin, on the host.

    The cheapest gate there is on the hooks: no sandbox, no session, no metered tokens, and it
    catches the failure that would otherwise only show up as a confusing session result — a `sed`
    extraction that misses a field, a converted-check that gates the wrong primitive, a block that
    exits with the wrong code or writes to the wrong channel.

    Every row is keyed to a shape the scripts actually produce, and the two rows that matter most
    are the negative ones: an unconverted primitive must be left completely alone, because the
    transition clause says a primitive still reading a shipped schema file is never gated.
    """
    staged = stage("hook-input", PLUGIN)
    checks: list[Check] = []
    dependency = staged.plugin / "hooks" / "scripts" / "dependency-halt.sh"
    session_start = staged.plugin / "hooks" / "scripts" / "session-start.sh"

    for script in (dependency, session_start):
        if not script.is_file():
            checks.append(ok(f"{script.name} exists", f"no script at {script}"))
    binary, reason = host_binary()
    if reason:
        checks.append(ok("a runnable host binary for the present-binary rows", reason))
    if any(check.status == "fail" for check in checks):
        write_verdict(staged.root, "hook-input", checks, {"shape": "host, no session"})
        return checks, staged.root

    captures, notes = load_captures()
    absent_path = MINIMAL_PATH
    probe = host_sh("command -v mochiko-cli", env={"PATH": absent_path})
    checks.append(
        ok(
            "the absent-binary rows really have no `mochiko-cli`",
            None if probe.returncode != 0 else f"found one at {probe.stdout.strip()!r}",
        )
    )
    bin_dir = staged.root / "bin"
    bin_dir.mkdir(exist_ok=True)
    (bin_dir / "mochiko-cli").symlink_to(binary)
    present_path = f"{bin_dir}:{absent_path}"

    # A converted skill to exercise the `PreToolUse` limb. Written into the staged copy only: no
    # skill is converted at wave 3, and converting one for a test would be a plugin edit.
    stub = staged.plugin / "skills" / "contract-stub"
    stub.mkdir(parents=True, exist_ok=True)
    (stub / "SKILL.md").write_text(
        "---\nname: contract-stub\ndescription: staged-only stub for the hook-input case\n---\n\n"
        "!`mochiko-cli rules contract-stub --section preamble`\n",
        encoding="utf-8",
    )

    unconverted_command = unconverted_primitive(staged.plugin, "command")
    unconverted_skill = unconverted_primitive(staged.plugin, "skill")
    checks.append(
        ok(
            "the plugin still carries an unconverted command and skill to leave alone",
            None
            if unconverted_command and unconverted_skill
            else "every primitive is converted; the transition-clause rows cannot be run",
        )
    )
    if any(check.status == "fail" for check in checks):
        write_verdict(staged.root, "hook-input", checks, {"shape": "host, no session"})
        return checks, staged.root

    upe = captures.get("UserPromptExpansion")
    pre = captures.get("PreToolUse")
    if upe is None or pre is None:
        checks.append(
            ok(
                "captured UserPromptExpansion and PreToolUse stdin are committed",
                f"missing under {HOOK_INPUT}: "
                + ", ".join(
                    name
                    for name, value in (("UserPromptExpansion", upe), ("PreToolUse", pre))
                    if value is None
                ),
            )
        )
        write_verdict(staged.root, "hook-input", checks, {"shape": "host, no session"})
        return checks, staged.root

    def upe_with(name: str) -> dict:
        return {**upe, "command_name": name, "prompt": f"/{name} probe"}

    def skill_with(name: str) -> dict:
        return {**pre, "tool_input": {**(pre.get("tool_input") or {}), "skill": name}}

    def silent(proc) -> str | None:
        problems = []
        if proc.returncode != 0:
            problems.append(f"exit {proc.returncode}, expected 0")
        if proc.stdout.strip():
            problems.append(f"stdout {proc.stdout.strip()[:120]!r}")
        if proc.stderr.strip():
            problems.append(f"stderr {proc.stderr.strip()[:120]!r}")
        return "; ".join(problems) if problems else None

    def json_field(proc, *path: str) -> tuple[object, str | None]:
        if proc.returncode != 0:
            return None, f"exit {proc.returncode}, expected 0: {proc.stderr.strip()[:160]!r}"
        try:
            payload = json.loads(proc.stdout)
        except json.JSONDecodeError as err:
            return None, f"stdout is not JSON ({err}): {proc.stdout.strip()[:160]!r}"
        node = payload
        for key in path:
            if not isinstance(node, dict) or key not in node:
                return None, f"no {'.'.join(path)} in {proc.stdout.strip()[:160]!r}"
            node = node[key]
        return node, None

    rows = []

    def hook(script, payload, path_env):
        proc = run_hook(script, payload, path_env=path_env, plugin_root=staged.plugin,
                        home=staged.root)
        rows.append(
            {
                "script": script.name,
                "path": "present" if path_env == present_path else "absent",
                "exit": proc.returncode,
                "stdout": proc.stdout.strip()[:400],
                "stderr": proc.stderr.strip()[:400],
            }
        )
        return proc

    # --- the transition clause: an unconverted primitive is never gated ------------------
    proc = hook(dependency, upe_with(f"mochiko:{unconverted_command}"), absent_path)
    checks.append(ok(f"unconverted command `{unconverted_command}` is left alone", silent(proc)))

    proc = hook(dependency, skill_with(f"mochiko:{unconverted_skill}"), absent_path)
    checks.append(ok(f"unconverted skill `{unconverted_skill}` is left alone", silent(proc)))

    proc = hook(dependency, upe_with("other:thing"), absent_path)
    checks.append(ok("a command outside the mochiko namespace is left alone", silent(proc)))

    # --- absence: the block, on the channel each registration uses -----------------------
    proc = hook(dependency, upe_with(f"mochiko:{PILOT_COMMAND}"), absent_path)
    problems = []
    if proc.returncode != 2:
        problems.append(f"exit {proc.returncode}, expected 2")
    for fragment in (INSTALL_LINE, f"/mochiko:{PILOT_COMMAND}"):
        if fragment not in proc.stderr:
            problems.append(f"stderr is missing {fragment!r}")
    if proc.stdout.strip():
        problems.append(f"stdout was not empty: {proc.stdout.strip()[:120]!r}")
    checks.append(
        ok(
            "converted command + no binary: exit 2 and the install line on stderr",
            "; ".join(problems) if problems else None,
        )
    )

    proc = hook(dependency, skill_with("mochiko:contract-stub"), absent_path)
    decision, problem = json_field(proc, "hookSpecificOutput", "permissionDecision")
    if problem is None and decision != "deny":
        problem = f"permissionDecision was {decision!r}, expected 'deny'"
    if problem is None:
        reason_text, problem = json_field(
            proc, "hookSpecificOutput", "permissionDecisionReason"
        )
        if problem is None and INSTALL_LINE not in str(reason_text):
            problem = f"the deny reason is missing {INSTALL_LINE!r}: {reason_text!r}"
    checks.append(ok("converted skill + no binary: a JSON deny carrying the install line", problem))

    # --- presence: one confirmation line, per registration, and never the rules ----------
    for payload, noun in (
        (upe_with(f"mochiko:{PILOT_COMMAND}"), "command"),
        (skill_with("mochiko:contract-stub"), "skill"),
    ):
        proc = hook(dependency, payload, present_path)
        context, problem = json_field(proc, "hookSpecificOutput", "additionalContext")
        expected = f"{HOOK_PRESENT_PREFIX} {noun}'s own render"
        if problem is None and str(context) != expected:
            problem = f"additionalContext was {context!r}, expected {expected!r}"
        if problem is None and TRIPLE_HEAD in str(context):
            problem = "the hook injected rules; branch B confirms presence and delivers nothing"
        checks.append(ok(f"converted {noun} + binary present: the presence line only", problem))

    # --- skew: the hook's only other gate, and the only one that needs a broken log ------
    skew_root = staged.root / "mochiko-skew"
    shutil.copytree(staged.plugin, skew_root)
    swap_plugin_log(skew_root)
    skew_proc = run_hook(
        dependency,
        upe_with(f"mochiko:{PILOT_COMMAND}"),
        path_env=present_path,
        plugin_root=skew_root,
        home=staged.root,
    )
    rows.append(
        {
            "script": dependency.name,
            "path": "present, skew log",
            "exit": skew_proc.returncode,
            "stdout": skew_proc.stdout.strip()[:400],
            "stderr": skew_proc.stderr.strip()[:400],
        }
    )
    problems = []
    if skew_proc.returncode != 2:
        problems.append(f"exit {skew_proc.returncode}, expected 2")
    for fragment in ("grammar 99", INSTALL_LINE):
        if fragment not in skew_proc.stderr:
            problems.append(f"stderr is missing {fragment!r}")
    checks.append(
        ok(
            "converted command + out-of-range log: exit 2 and the binary's own D5 message",
            "; ".join(problems) if problems else None,
        )
    )

    # --- SessionStart: loud, never blocking ---------------------------------------------
    proc = hook(session_start, captures["SessionStart"], present_path)
    problems = []
    if proc.returncode != 0:
        problems.append(f"exit {proc.returncode}, expected 0")
    line = proc.stdout.strip().splitlines()[0] if proc.stdout.strip() else ""
    if not re.match(r"^mochiko-cli \d+\.\d+\.\d+ · grammar \d+\.\.\d+ · plugin ", line):
        problems.append(f"the first line is {line!r}")
    if "· in range" not in proc.stdout:
        problems.append("no `· in range` in the output")
    checks.append(
        ok("SessionStart + binary present: the version and in-range line",
           "; ".join(problems) if problems else None)
    )

    proc = hook(session_start, captures["SessionStart"], absent_path)
    problems = []
    if proc.returncode != 0:
        problems.append(f"exit {proc.returncode}, expected 0 — SessionStart never blocks")
    if INSTALL_LINE not in proc.stdout:
        problems.append(f"stdout is missing {INSTALL_LINE!r}: {proc.stdout.strip()[:160]!r}")
    checks.append(
        ok("SessionStart + no binary: the install line, still exit 0",
           "; ".join(problems) if problems else None)
    )

    # The unsupported-environment notice. GI-020 declares a policy that blocks inline execution
    # unsupported, and this is the only place a user is told so before their first fire.
    policy_home = staged.root / "policy-home"
    (policy_home / ".claude").mkdir(parents=True, exist_ok=True)
    (policy_home / ".claude" / "settings.json").write_text(
        '{"disableSkillShellExecution": true}\n', encoding="utf-8"
    )
    policy_proc = run_hook(
        session_start,
        captures["SessionStart"],
        path_env=present_path,
        plugin_root=staged.plugin,
        home=policy_home,
    )
    rows.append(
        {
            "script": session_start.name,
            "path": "present, policy set",
            "exit": policy_proc.returncode,
            "stdout": policy_proc.stdout.strip()[:400],
            "stderr": policy_proc.stderr.strip()[:400],
        }
    )
    problems = []
    if policy_proc.returncode != 0:
        problems.append(f"exit {policy_proc.returncode}, expected 0")
    if "unsupported" not in policy_proc.stdout:
        problems.append(f"no unsupported-environment line: {policy_proc.stdout.strip()[:200]!r}")
    checks.append(
        ok("SessionStart names an environment that disables shell execution",
           "; ".join(problems) if problems else None)
    )

    for note in notes:
        checks.append(report("capture provenance", note))

    write_verdict(
        staged.root,
        "hook-input",
        checks,
        {"shape": "host, no session", "rows": rows, "capture_notes": notes},
    )
    return checks, staged.root


def case_render_ceiling(runner, sandbox) -> tuple[list, pathlib.Path]:
    """Every render of every converted primitive against the inline ceiling.

    Wave-0 probe (e) measured the ceiling at roughly 30,000 characters: above it a `!` block
    arrives as a file-path notice whose preview keeps only the first line, which would strand a
    render without its end line — delivered, apparently fine, and silently truncated. This is the
    one assertion that catches that before a user does, and it needs no session at all.
    """
    staged = stage("render-ceiling", PLUGIN)
    checks: list[Check] = []
    binary, reason = host_binary()
    if reason:
        checks.append(ok("a runnable host binary", reason))
        write_verdict(staged.root, "render-ceiling", checks, {"shape": "direct binary"})
        return checks, staged.root

    if sandbox is not None:
        host_version = host_sh(f"{shlex.quote(binary)} --version").stdout.strip()
        sandbox_version = runner.sbx_sh(f"{shlex.quote(sandbox.binary)} --version").stdout.strip()
        checks.append(
            ok(
                "the host binary matches the sandbox build",
                None
                if host_version == sandbox_version
                else f"host {host_version!r} vs sandbox {sandbox_version!r} — "
                "the host binary is stale; rebuild it before trusting these figures",
            )
        )

    primitives = converted_primitives(staged.plugin)
    checks.append(
        ok(
            "at least one primitive is converted",
            None if primitives else "no primitive carries a `!` rules line; nothing to measure",
        )
    )
    measurements, largest = [], None
    for kind, name, _ in primitives:
        preamble = render(binary, name, "preamble", staged.plugin)
        if preamble.returncode != 0:
            checks.append(
                ok(f"{kind} `{name}` renders its preamble",
                   f"exit {preamble.returncode}: {preamble.stderr.strip()[:200]!r}")
            )
            continue
        for section in ["preamble"] + [s.id for s in parse_preamble(preamble.stdout)]:
            out = (
                preamble
                if section == "preamble"
                else render(binary, name, section, staged.plugin)
            )
            if out.returncode != 0:
                checks.append(
                    ok(f"{kind} `{name}` renders `{section}`",
                       f"exit {out.returncode}: {out.stderr.strip()[:200]!r}")
                )
                continue
            entry = {
                "primitive": name,
                "section": section,
                "chars": len(out.stdout),
                "bytes": len(out.stdout.encode("utf-8")),
            }
            measurements.append(entry)
            if largest is None or entry["chars"] > largest["chars"]:
                largest = entry
    over = [m for m in measurements if m["chars"] >= INLINE_CEILING]
    checks.append(
        ok(
            f"every converted render is under the {INLINE_CEILING:,}-char inline ceiling",
            None if not over else f"over the ceiling: {over}",
        )
    )
    if largest is not None:
        checks.append(
            report(
                "largest render",
                f"{largest['primitive']} · {largest['section']} — {largest['chars']:,} chars / "
                f"{largest['bytes']:,} bytes, {largest['chars'] / INLINE_CEILING:.1%} of the ceiling",
            )
        )
    write_verdict(
        staged.root,
        "render-ceiling",
        checks,
        {
            "shape": "direct binary",
            "converted": [f"{kind}:{name}" for kind, name, _ in primitives],
            "measurements": measurements,
            "largest": largest,
        },
    )
    return checks, staged.root


# ---------------------------------------------------------------------------
# sandbox cases — the converted command in a real session
# ---------------------------------------------------------------------------

def brainstorm_expectations(plugin: pathlib.Path) -> tuple[list[str], dict[str, int], str | None]:
    """What the delivery must carry, read from the binary rather than written down here.

    Returns the section ids in the order the command's `!` lines fire, the rule count each end
    line must report, and a reason if the expectation could not be built.
    """
    binary, reason = host_binary()
    if reason:
        return [], {}, reason
    preamble = render(binary, PILOT_COMMAND, "preamble", plugin)
    if preamble.returncode != 0:
        return [], {}, f"the preamble render failed: {preamble.stderr.strip()[:200]!r}"
    sections = parse_preamble(preamble.stdout)
    if not sections:
        return [], {}, f"no section list in the preamble render: {preamble.stdout[:200]!r}"
    ids = ["preamble"] + [section.id for section in sections]
    counts = {"preamble": 0, **{section.id: section.rules for section in sections}}
    return ids, counts, None


def assert_delivery(text: str, ids: list[str], counts: dict[str, int]) -> list[Check]:
    heads = head_line_sections(text, PILOT_COMMAND)
    ends = end_line_counts(text, PILOT_COMMAND)
    missing_heads = [i for i in ids if i not in heads]
    missing_ends = [i for i in ids if i not in ends]
    wrong = {i: ends[i] for i in ids if i in ends and ends[i] != counts[i]}
    return [
        ok(
            f"all {len(ids)} version-triple head lines reached the model",
            None if not missing_heads else f"missing {missing_heads}",
        ),
        ok(
            f"all {len(ids)} closing end lines reached the model",
            None if not missing_ends else f"missing {missing_ends}",
        ),
        ok(
            "every end-line count matches the preamble's own section list",
            None if not wrong else f"disagreements (delivered vs pinned): {wrong} vs {counts}",
        ),
        ok("the `!` lines were expanded, not passed through", assert_bang_ran_converted(text)),
        ok("no Bash denial", assert_no_denial(text)),
    ]


def assert_bang_ran_converted(text: str) -> str | None:
    if CONVERTED_MARK in text:
        return "a `!` line was passed through literally — preprocessing did not run"
    return None


def assert_no_denial(text: str) -> str | None:
    if "Permission to use Bash has been denied" in text:
        return "a `!` line was denied — the allowed-tools grant is not in force"
    return None


def expected_slash_commands() -> list[str]:
    return [f"mochiko:{path.stem}" for path in sorted((PLUGIN / "commands").glob("*.md"))]


LATENCY_RUNS = 10


def measure_latency(runner, sandbox: "Sandbox", staged: Staged, sections: list[str]) -> dict:
    """Per-section render latency in the sandbox, written to the evidence directory.

    Timed inside the sandbox in a single shell so the figure is the binary and not the `sbx exec`
    transport. **These numbers are load-dependent** — two independent passes on the same machine
    differed by roughly a factor of two — so the artifact records every individual run, not just
    the summary, and a reader can see the spread rather than trusting a mean.
    """
    quoted = " ".join(shlex.quote(section) for section in sections)
    script = f"""
B={shlex.quote(sandbox.binary)}
R={shlex.quote(str(staged.plugin))}
for sec in {quoted}; do
  i=0
  while [ $i -lt {LATENCY_RUNS} ]; do
    t0=$(date +%s%N)
    $B rules {shlex.quote(PILOT_COMMAND)} --section "$sec" --plugin-root "$R" >/dev/null 2>&1
    t1=$(date +%s%N)
    echo "RUN $sec $(( (t1 - t0) / 1000000 ))"
    i=$(( i + 1 ))
  done
done
t0=$(date +%s%N)
for sec in {quoted}; do
  $B rules {shlex.quote(PILOT_COMMAND)} --section "$sec" --plugin-root "$R" >/dev/null 2>&1
done
t1=$(date +%s%N)
echo "FIRE $(( (t1 - t0) / 1000000 ))"
"""
    out = runner.sbx_sh(script, timeout=900)
    runs: dict[str, list[int]] = {section: [] for section in sections}
    whole_fire = None
    for line in out.stdout.splitlines():
        parts = line.split()
        if len(parts) == 3 and parts[0] == "RUN" and parts[1] in runs:
            runs[parts[1]].append(int(parts[2]))
        elif len(parts) == 2 and parts[0] == "FIRE":
            whole_fire = int(parts[1])
    per_section = {
        section: {
            "runs_ms": values,
            "mean_ms": round(sum(values) / len(values)) if values else None,
            "max_ms": max(values) if values else None,
        }
        for section, values in runs.items()
    }
    means = [v["mean_ms"] for v in per_section.values() if v["mean_ms"] is not None]
    maxes = [v["max_ms"] for v in per_section.values() if v["max_ms"] is not None]
    payload = {
        "runs_per_section": LATENCY_RUNS,
        "note": "load-dependent; timed inside the sandbox, not across `sbx exec`",
        "per_section": per_section,
        "mean_band_ms": [min(means), max(means)] if means else None,
        "worst_single_run_ms": max(maxes) if maxes else None,
        "whole_fire_ms": whole_fire,
    }
    write_evidence(staged.root, "latency.json", json.dumps(payload, indent=2) + "\n")
    return payload


def case_brainstorm_delivery(runner, sandbox: "Sandbox") -> tuple[list, pathlib.Path]:
    """The happy path: the converted command fires, seven blocks arrive, nothing is Read.

    Three replicates, because the read-back metric needs them (D8). The delivery assertions are
    checked on every replicate rather than the first — if delivery were flaky, a single-replicate
    check would find it only by luck, and flakiness is exactly the failure this suite exists for.
    """
    staged = stage("brainstorm-delivery", PLUGIN)
    ids, counts, reason = brainstorm_expectations(staged.plugin)
    if reason:
        checks = [ok("the delivery expectation could be built from the binary", reason)]
        write_verdict(staged.root, "brainstorm-delivery", checks, {"shape": "delivery"})
        return checks, staged.root

    path_env = f"{sandbox.binary_dir}:{sandbox.path}"
    replicates = []
    for index in range(READ_BACK_REPLICATES):
        probed = run_probe(
            runner,
            staged,
            path_env=path_env,
            log_dir=None,
            prompt=f"/mochiko:{PILOT_COMMAND} {PROBE_TOPIC}",
            max_turns=2,
            tag=f"-{index + 1}",
        )
        session = session_id_of(probed.events)
        seen, transcript_path = fetch_transcript(runner, staged, session, tag=f"-{index + 1}")
        tokens, passed = score_read_back(final_assistant_text(probed.events))
        blocks = delivered_blocks(seen, PILOT_COMMAND)
        result = result_event(probed.events)
        replicates.append(
            {
                "index": index + 1,
                "probed": probed,
                "seen": seen,
                "transcript_path": transcript_path,
                "session_id": session,
                "tokens": tokens,
                "read_back_passed": passed,
                "delivered_chars": sum(len(b) for b in blocks.values()),
                "delivered_bytes": sum(len(b.encode("utf-8")) for b in blocks.values()),
                "blocks": sorted(blocks),
                "num_turns": (result or {}).get("num_turns"),
                "final_text": final_assistant_text(probed.events)[:400],
            }
        )

    checks: list[Check] = []
    for name, problems in _aggregate(replicates, ids, counts):
        checks.append(ok(name, problems))

    scored = sum(1 for r in replicates if r["read_back_passed"])
    checks.append(
        report(
            "read-back metric (never gating)",
            f"{scored}/{len(replicates)} replicates named the floor set exactly; "
            f"bar {READ_BACK_BAR}/{READ_BACK_REPLICATES} pre-registered — "
            f"{'MET' if scored >= READ_BACK_BAR else 'NOT MET'}",
        )
    )
    delivered = replicates[0]["delivered_bytes"]
    checks.append(
        report(
            "delivered read cost",
            f"{delivered:,} bytes / {replicates[0]['delivered_chars']:,} chars against the "
            f"{BASELINE_BYTES:,}-byte baseline — "
            f"{(delivered - BASELINE_BYTES) / BASELINE_BYTES:+.1%} bytes",
        )
    )
    latency = measure_latency(runner, sandbox, staged, ids)
    band = latency.get("mean_band_ms")
    checks.append(
        report(
            "store latency (load-dependent)",
            f"per-section mean {band[0]}–{band[1]} ms, worst single run "
            f"{latency['worst_single_run_ms']} ms, whole fire "
            f"{latency['whole_fire_ms']} ms — {LATENCY_RUNS} runs per section, in latency.json"
            if band
            else "not measured",
        )
    )

    write_verdict(
        staged.root,
        "brainstorm-delivery",
        checks,
        {
            "shape": "delivery",
            "expected_sections": ids,
            "expected_counts": counts,
            "read_back": {
                "bar": READ_BACK_BAR,
                "replicates": READ_BACK_REPLICATES,
                "scored": scored,
                "gating": False,
                "per_replicate": [
                    {
                        "index": r["index"],
                        "tokens": r["tokens"],
                        "passed": r["read_back_passed"],
                        "num_turns": r["num_turns"],
                        "final_text": r["final_text"],
                    }
                    for r in replicates
                ],
            },
            "latency": latency,
            "read_cost": {
                "baseline_bytes": BASELINE_BYTES,
                "baseline_chars": BASELINE_CHARS,
                "baseline_bytes_with_labels": BASELINE_BYTES_WITH_LABELS,
                "source": "the session transcript, not the stream",
                "per_replicate": [
                    {
                        "index": r["index"],
                        "bytes": r["delivered_bytes"],
                        "chars": r["delivered_chars"],
                        "blocks": r["blocks"],
                        "transcript_path": r["transcript_path"],
                    }
                    for r in replicates
                ],
            },
        },
    )
    return checks, staged.root


def _aggregate(replicates: list, ids: list[str], counts: dict[str, int]) -> list:
    """Run each delivery assertion over every replicate, reporting which ones failed.

    Two different sources, deliberately. What the model was *given* — the seven blocks and the
    `UserPromptExpansion` hook's presence line — is read from the session transcript, because the
    stream carries neither: the expanded prompt appears in no stream row, and that hook produces
    no stream row at all even when it fires. What the model *did* — tool uses, the init event's
    command registry — is read from the stream, which is where those live.
    """
    per_replicate = []
    for entry in replicates:
        checks = [
            ok(
                "the session transcript was recovered",
                None
                if entry["seen"]
                else f"no transcript for session {entry['session_id']!r} under "
                "~/.claude/projects/ — every delivery assertion below reads it",
            )
        ]
        checks += assert_delivery(entry["seen"], ids, counts) + [
            ok("no schema file was Read", assert_no_schema_read(entry["probed"].events)),
            ok(
                "the SessionStart hook reported the binary",
                _session_start_line(entry["probed"], entry["seen"]),
            ),
            ok(
                "the dependency hook confirmed presence in the transcript",
                None
                if HOOK_PRESENT_PREFIX in entry["seen"]
                else f"{HOOK_PRESENT_PREFIX!r} is not in the session transcript",
            ),
            ok(
                "the plugin's six commands registered as slash commands",
                assert_slash_commands(entry["probed"].events, expected_slash_commands()),
            ),
        ]
        per_replicate.append(checks)
    merged = []
    for position, template in enumerate(per_replicate[0]):
        failures = [
            f"replicate {index + 1}: {group[position].detail}"
            for index, group in enumerate(per_replicate)
            if group[position].status == "fail"
        ]
        merged.append((template.name, "; ".join(failures) if failures else None))
    return merged


def _session_start_line(probed: Probed, transcript: str = "") -> str | None:
    """The SessionStart hook's own line, whichever form it took.

    Three outcomes are all legitimate presence reports — the in-range line, the D5 line, and the
    both-lines fallback — so the assertion is that the hook spoke and named the binary, not that
    it chose one particular branch. Read from the measured channels only: this hook's output does
    reach the stream as `hook_started`/`hook_response` rows, and finding it on the process streams
    instead would not mean the session ever saw it.
    """
    if re.search(
        r"mochiko-cli \d+\.\d+\.\d+ · grammar \d+\.\.\d+", asserted_output(probed, transcript)
    ):
        return None
    return "no SessionStart version line in the session transcript or the stream events"


def case_brainstorm_absence(runner, sandbox: "Sandbox") -> tuple[list, pathlib.Path]:
    """The binary is off PATH with the plugin's own hooks in play.

    This is where wave 1's pending assertion resolves. Two limbs can halt this run — the
    `UserPromptExpansion` hook's exit-2 block, which fires *before* expansion, and the `!` line's
    own failure, which fires during it. The hook should win, and the install line should reach the
    user either way; which channel carried it is recorded rather than assumed, because the stream
    shape of a blocked expansion is not measured anywhere yet.
    """
    staged = stage("brainstorm-absence", PLUGIN)
    probed = run_probe(
        runner,
        staged,
        path_env=sandbox.path,
        log_dir=None,
        prompt=f"/mochiko:{PILOT_COMMAND} {PROBE_TOPIC}",
        max_turns=2,
    )
    seen, transcript_path = fetch_transcript(runner, staged, session_id_of(probed.events))
    union = session_output_with(probed, seen)
    channels = channels_of(probed, INSTALL_LINE, seen)
    checks = [
        ok("no model turn ran", assert_halt_before_model(probed.events)),
        ok("the install line reached the session", assert_in_session(probed, INSTALL_LINE, seen)),
        ok("no schema file was Read", assert_no_schema_read(probed.events)),
        ok("no version triple was delivered", assert_no_version_triple(union)),
        report("install-line channel", ", ".join(channels) or "none"),
        report(
            "which limb halted first",
            "the dependency hook — the result event carries its block notice and no "
            "`<local-command-stderr>` was injected"
            if str((result_event(probed.events) or {}).get("result") or "").startswith(
                HOOK_BLOCK_PREFIX
            )
            else "the `!` line — the harness injected its stderr",
        ),
    ]
    write_verdict(
        staged.root,
        "brainstorm-absence",
        checks,
        {
            "shape": halt_shape(probed),
            "channels": channels,
            "transcript_path": transcript_path,
        },
    )
    return checks, staged.root


def case_brainstorm_skew(runner, sandbox: "Sandbox") -> tuple[list, pathlib.Path]:
    """The staged plugin's own log is out of the binary's grammar range.

    The lever is the plugin's own `migrations/`, not `MOCHIKO_MIGRATIONS`: the converted command's
    `!` lines pass `--plugin-root "${CLAUDE_PLUGIN_ROOT}"`, which beats the environment variable
    in the resolution order, so the wave-1 case's lever cannot reach them. The direct run below
    proves the swap took — that the halt is about the log this case built, and not about some
    other log that happened to be broken.
    """
    staged = stage("brainstorm-skew", PLUGIN)
    swap_plugin_log(staged.plugin)
    path_env = f"{sandbox.binary_dir}:{sandbox.path}"

    direct = runner.sbx_sh(
        f"env PATH={shlex.quote(path_env)} mochiko-cli rules {PILOT_COMMAND} "
        f"--section preamble --plugin-root {shlex.quote(str(staged.plugin))}",
        timeout=120,
    )
    write_evidence(
        staged.root,
        "direct-binary.txt",
        f"exit: {direct.returncode}\n--- stdout ---\n{direct.stdout}"
        f"--- stderr ---\n{direct.stderr}",
    )

    probed = run_probe(
        runner,
        staged,
        path_env=path_env,
        log_dir=None,
        prompt=f"/mochiko:{PILOT_COMMAND} {PROBE_TOPIC}",
        max_turns=2,
    )
    seen, transcript_path = fetch_transcript(runner, staged, session_id_of(probed.events))
    text = session_output_with(probed, seen)
    channels = channels_of(probed, "grammar 99", seen)
    checks = [
        ok(
            "the swapped log is what the command's own root resolves to",
            assert_skew_halt_on_stderr(direct, INSTALL_LINE),
        ),
        ok("no model turn ran", assert_halt_before_model(probed.events)),
        ok(
            "the D5 grammar message reached the session",
            assert_in_session(probed, "grammar 99", seen),
        ),
        ok("the install line reached the session", assert_in_session(probed, INSTALL_LINE, seen)),
        ok("no schema file was Read", assert_no_schema_read(probed.events)),
        ok("no version triple was delivered", assert_no_version_triple(text)),
        report("halt channel", ", ".join(channels) or "none"),
        report(
            "which limb halted first",
            "the dependency hook's range check — the result event carries its block notice and "
            "no `<local-command-stderr>` was injected"
            if str((result_event(probed.events) or {}).get("result") or "").startswith(
                HOOK_BLOCK_PREFIX
            )
            else "the `!` line — the harness injected its stderr",
        ),
    ]
    write_verdict(
        staged.root,
        "brainstorm-skew",
        checks,
        {
            "shape": halt_shape(probed),
            "direct_binary_exit": direct.returncode,
            "direct_binary_stderr": direct.stderr.strip(),
            "channels": channels,
            "transcript_path": transcript_path,
        },
    )
    return checks, staged.root


def case_brainstorm_hooks_off(runner, sandbox: "Sandbox") -> tuple[list, pathlib.Path]:
    """Binary absent and hooks disabled: the harness path is the only guard left.

    D7's floor is that a hook which cannot run never blocks anything — fail-open. That is the
    right default and it means the hooks cannot be the only thing standing between a missing
    binary and a run that improvises. With every hook switched off, the wave-1 shape must come
    back: the `!` line fails, the harness injects its stderr, and no model turn happens.
    """
    staged = stage("brainstorm-hooks-off", PLUGIN)
    probed = run_probe(
        runner,
        staged,
        path_env=sandbox.path,
        log_dir=None,
        prompt=f"/mochiko:{PILOT_COMMAND} {PROBE_TOPIC}",
        max_turns=2,
        settings={"disableAllHooks": True},
    )
    # The transcript is fetched even though the halt check is the decisive one. Without it, "no
    # version triple was delivered" would read a text that excludes the single channel measured to
    # carry delivered rules, and so could not fail — a vacuous assertion. It also makes this the
    # last brainstorm case whose evidence a later reader can re-derive from disk.
    seen, transcript_path = fetch_transcript(runner, staged, session_id_of(probed.events))
    union = session_output_with(probed, seen)
    checks = [
        ok("no model turn ran", assert_halt_before_model(probed.events)),
        ok(
            "the harness injected the shell's stderr, naming the missing binary",
            assert_local_command_stderr(probed.events, "mochiko-cli", "command not found"),
        ),
        ok("no schema file was Read", assert_no_schema_read(probed.events)),
        ok("no version triple was delivered", assert_no_version_triple(union)),
        report(
            "the hooks really were off",
            "absent"
            if HOOK_PRESENT_PREFIX not in union
            else "a hook still spoke — the setting did not take",
        ),
    ]
    write_verdict(
        staged.root,
        "brainstorm-hooks-off",
        checks,
        {"shape": halt_shape(probed), "transcript_path": transcript_path},
    )
    return checks, staged.root


def case_brainstorm_policy(runner, sandbox: "Sandbox") -> tuple[list, pathlib.Path]:
    """Shell execution disabled by policy, hooks on. Recorded, never asserted (D8).

    GI-020 declares this environment unsupported, so there is no contract to hold it to and
    nothing here gates. What it is worth knowing is what actually happens: under branch B the hook
    delivers only a presence line, so the placeholder should reach the model and the `.md`'s prose
    halt clause should be the thing that stops the run — the one path in the whole suite where
    that clause is load-bearing. Whether the model obeys it is a fact about the model, which is
    exactly why it is recorded and not asserted.
    """
    staged = stage("brainstorm-policy", PLUGIN)
    probed = run_probe(
        runner,
        staged,
        path_env=f"{sandbox.binary_dir}:{sandbox.path}",
        log_dir=None,
        prompt=f"/mochiko:{PILOT_COMMAND} {PROBE_TOPIC}",
        max_turns=2,
        settings={"disableSkillShellExecution": True},
    )
    seen, transcript_path = fetch_transcript(runner, staged, session_id_of(probed.events))
    text = session_output_with(probed, seen)
    final = final_assistant_text(probed.events)
    placeholder = "[shell command execution disabled by policy]"

    # The command's own halt clause quotes both `[shell command execution disabled by policy]` and
    # `mochiko-cli rules not delivered`, and the clause is part of the expanded prompt. Searching
    # the whole transcript for either phrase would find the instructions rather than the outcome.
    # So: the placeholder is detected by the *absence of rendered blocks* — with shell execution
    # off, no `!` line can produce a version-triple head — and the prose halt is looked for only in
    # what the model itself wrote.
    delivered = head_line_sections(seen, PILOT_COMMAND)
    expected_ids, _, _ = brainstorm_expectations(staged.plugin)
    checks = [
        report("rendered blocks delivered", f"{len(delivered)} of {len(expected_ids)} expected"),
        report(
            "the placeholder text appears in the transcript",
            f"{seen.count(placeholder)} occurrence(s); 1 is the halt clause quoting it",
        ),
        report(
            "the run halted on the prose clause",
            "yes — the model surfaced the not-delivered line"
            if "mochiko-cli rules not delivered" in final
            else ("no model text at all" if not final.strip()
                  else f"no — the model replied {final.strip()[:200]!r}"),
        ),
        report("the hook spoke", "yes" if HOOK_PRESENT_PREFIX in text else "no"),
        report("model turns", str((result_event(probed.events) or {}).get("num_turns"))),
        report(
            "a schema file was read as a fallback",
            assert_no_schema_read(probed.events) or "no",
        ),
    ]
    write_verdict(
        staged.root,
        "brainstorm-policy",
        checks,
        {
            "shape": halt_shape(probed),
            "final_text": final[:600],
            "delivered_sections": sorted(delivered),
            "placeholder_occurrences": seen.count(placeholder),
            "transcript_path": transcript_path,
        },
    )
    return checks, staged.root


BANG_LINE = re.compile(
    r"^!`mochiko-cli rules (\S+) --section (\S+) --plugin-root \"\$\{CLAUDE_PLUGIN_ROOT\}\" 2>&1`$",
    re.M,
)
GRANT = "allowed-tools: Bash(mochiko-cli *)"


def case_converted_shape(runner, sandbox) -> tuple[list, pathlib.Path]:
    """A converted primitive's `!` lines against the section list its own render declares.

    Cheap, and it catches the failure that is most expensive to find any other way: a `.md` that
    enumerates six sections when the schema has seven delivers six blocks, every one of them
    correctly formed, and the only symptom is a rule the model was never given. Nothing in the
    session assertions would call that a failure — they check that what arrived is well-formed,
    not that everything was asked for. The `.md` and the render have to be compared directly, and
    on the host, before a metered run is spent on it.
    """
    staged = stage("converted-shape", PLUGIN)
    checks: list[Check] = []
    binary, reason = host_binary()
    if reason:
        checks.append(ok("a runnable host binary", reason))
        write_verdict(staged.root, "converted-shape", checks, {"shape": "static + binary"})
        return checks, staged.root

    primitives = converted_primitives(staged.plugin)
    checks.append(
        ok(
            "at least one primitive is converted",
            None if primitives else "no primitive carries a `!` rules line; nothing to compare",
        )
    )
    details = []
    for kind, name, path in primitives:
        body = path.read_text(encoding="utf-8")
        asked = [(m.group(1), m.group(2)) for m in BANG_LINE.finditer(body)]
        wrong_primitive = sorted({p for p, _ in asked if p != name})
        checks.append(
            ok(
                f"{kind} `{name}`: every `!` line renders its own primitive",
                None if not wrong_primitive else f"lines ask for {wrong_primitive}",
            )
        )
        preamble = render(binary, name, "preamble", staged.plugin)
        if preamble.returncode != 0:
            checks.append(
                ok(f"{kind} `{name}` renders its preamble",
                   f"exit {preamble.returncode}: {preamble.stderr.strip()[:200]!r}")
            )
            continue
        expected = ["preamble"] + [s.id for s in parse_preamble(preamble.stdout)]
        requested = [section for _, section in asked]
        checks.append(
            ok(
                f"{kind} `{name}`: the `!` lines enumerate every section, in the render's order",
                None
                if requested == expected
                else f"the `.md` asks for {requested}, the render declares {expected}",
            )
        )
        checks.append(
            ok(
                f"{kind} `{name}`: the Bash grant the `!` lines need is in the frontmatter",
                None if GRANT in body else f"no {GRANT!r} in {path.name}",
            )
        )
        details.append({"primitive": name, "kind": kind, "requested": requested,
                        "declared": expected})

    # The read-back bar's pre-registered floor set against what the binary actually renders.
    rendered = rendered_floor_ids(binary, PILOT_COMMAND, staged.plugin)
    missing = sorted(set(FLOOR_IDS) - rendered)
    extra = sorted(rendered - set(FLOOR_IDS))
    checks.append(
        ok(
            f"the pre-registered floor set matches the {PILOT_COMMAND} render",
            None
            if not missing and not extra
            else f"pre-registered but not rendered: {missing}; rendered but not pre-registered: "
            f"{extra} — the read-back bar is grading the wrong set",
        )
    )
    write_verdict(
        staged.root,
        "converted-shape",
        checks,
        {
            "shape": "static + binary",
            "primitives": details,
            "floor_ids": {"pre_registered": sorted(FLOOR_IDS), "rendered": sorted(rendered)},
        },
    )
    return checks, staged.root


HOST_CASES = [
    ("hook-input", "the hook scripts, fed captured stdin — no sandbox, no session", case_hook_input),
    ("converted-shape", "a converted `.md`'s `!` lines against its own render", case_converted_shape),
    ("render-ceiling", "every converted render against the inline ceiling", case_render_ceiling),
]

SANDBOX_CASES = [
    ("absence", "[fixture] the binary is off PATH — the run halts, nothing delivered", case_absence),
    ("skew", "[fixture] the log's grammar is out of range — the D5 halt fires", case_skew),
    ("brainstorm-delivery", "the converted command delivers all seven blocks", case_brainstorm_delivery),
    ("brainstorm-absence", "no binary, hooks on — the install line reaches the user", case_brainstorm_absence),
    ("brainstorm-skew", "the staged plugin's own log is out of range", case_brainstorm_skew),
    ("brainstorm-hooks-off", "no binary, hooks off — the harness is the only guard", case_brainstorm_hooks_off),
    ("brainstorm-policy", "shell execution disabled by policy — recorded, not asserted", case_brainstorm_policy),
]

CASES = HOST_CASES + SANDBOX_CASES


# ---------------------------------------------------------------------------
# main
# ---------------------------------------------------------------------------

def run_cases(cases: list, runner, sandbox) -> tuple[int, int, int]:
    """Run a case list, printing each check. Returns (failures, pendings, reports)."""
    failures = pendings = reports = 0
    for name, _, case in cases:
        checks, evidence = case(runner, sandbox)
        failed = [c for c in checks if c.status == "fail"]
        pendings += sum(1 for c in checks if c.status == "pending")
        reports += sum(1 for c in checks if c.status == "report")
        print(f"{'FAIL' if failed else 'ok  '}  {name}")
        for check in checks:
            mark = {"ok": "ok", "fail": "FAIL", "pending": "pend", "report": "rec "}[check.status]
            detail = f" — {check.detail}" if check.detail else ""
            print(f"        {mark:4}  {check.name}{detail}")
        print(f"        evidence: {evidence.relative_to(REPO)}")
        if failed:
            failures += 1
    return failures, pendings, reports


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--list", action="store_true", help="print the case list and exit")
    parser.add_argument(
        "--host-only",
        action="store_true",
        help="run only the cases that need no sandbox and no session",
    )
    args = parser.parse_args()

    declared = HOST_CASES if args.host_only else CASES
    scope = "host cases only" if args.host_only else "all cases"
    print(f"mochiko contract suite · declared cases ({scope}):")
    for name, description, _ in declared:
        print(f"  {name:20s} {description}")
    if args.list:
        return EXIT_OK

    # Exit 0 means every declared case ran. A suite with nothing to run has proved nothing, so
    # it skips rather than reporting a clean sweep of zero.
    if not declared:
        print("\nSKIPPED: the suite declares no cases")
        print("exit 3 — the suite did not run, so nothing here is evidence of anything.")
        return EXIT_SKIP

    WORK.mkdir(parents=True, exist_ok=True)

    if args.host_only:
        print()
        failures, pendings, reports = run_cases(HOST_CASES, None, None)
        summarize(len(HOST_CASES), failures, pendings, reports)
        return EXIT_ASSERT if failures else EXIT_OK

    # The host cases run first: they are free, they need nothing built, and a broken hook script
    # should be visible before twenty minutes of sandbox build and nine metered sessions.
    print()
    failures, pendings, reports = run_cases(HOST_CASES, None, None)

    runner = load_runner()

    def skipped(reason: str) -> int:
        # A failed assertion outranks a skip. The host cases really ran, and what they found is
        # evidence whether or not the sandbox is reachable.
        print(f"\nSKIPPED (sandbox cases): {reason}")
        summarize(len(HOST_CASES), failures, pendings, reports)
        if failures:
            print("exit 1 — a host case failed; the sandbox cases did not run.")
            return EXIT_ASSERT
        print("exit 3 — the sandbox cases did not run, so they are evidence of nothing.")
        return EXIT_SKIP

    reason = preflight(runner)
    if reason:
        return skipped(reason)
    binary_path, reason = build_binary(runner)
    if reason:
        return skipped(reason)
    path_value, reason = sandbox_path(runner)
    if reason:
        return skipped(reason)

    sandbox = Sandbox(
        path=path_value,
        binary=binary_path,
        binary_dir=str(pathlib.PurePosixPath(binary_path).parent),
    )
    more = run_cases(SANDBOX_CASES, runner, sandbox)
    failures, pendings, reports = (a + b for a, b in zip((failures, pendings, reports), more))
    summarize(len(CASES), failures, pendings, reports)
    return EXIT_ASSERT if failures else EXIT_OK


def summarize(ran: int, failures: int, pendings: int, reports: int) -> None:
    print(f"\ncontract suite: {ran - failures}/{ran} cases passed, {ran} ran", end="")
    if pendings:
        print(f", {pendings} assertion(s) pending a later wave", end="")
    if reports:
        print(f", {reports} measurement(s) recorded and not asserted", end="")
    print()


if __name__ == "__main__":
    sys.exit(main())
