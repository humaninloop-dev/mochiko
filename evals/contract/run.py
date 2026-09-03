#!/usr/bin/env python3
"""The plugin contract suite — the layer that tests what the crate never can.

Provenance: `.mochiko/brainstorms/cli-schema-delivery/record.md` D8 as amended. The suite runs
inside the Docker AI sandbox `claude-mochiko`, through the sandbox helpers `evals/run.py` already
owns; it imports them and never forks them. Maintainer-side, never shipped (GI-020).

What it asserts (D8's deterministic set):

    the `!` line executed · the version-triple line present · the closing end line present ·
    no schema file Read anywhere · absence halts · skew halts

Two cases are runnable at wave 1, both of them failure paths, because those are the ones that do
not need a converted primitive:

    absence  the binary is off the sandbox PATH        -> the run halts, nothing is delivered
    skew     the log declares a grammar the binary
             does not read                             -> the run halts on the D5 message

The per-primitive cases (one per converted command and skill, with the read-back metric) arrive
at wave 3.

**A suite that cannot run says so.** Every prerequisite is checked before any case, and a missing
one exits 3 with the reason. Exit 0 means every declared case ran and passed — never "nothing
happened".

    exit 0  every case ran and passed
    exit 1  a case ran and an assertion failed
    exit 3  the suite could not run (no sandbox, not authenticated, no binary) — SKIPPED

Usage:
    python3 evals/contract/run.py            # run the cases
    python3 evals/contract/run.py --list     # print the case list and exit
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


class Check(NamedTuple):
    """One assertion's outcome.

    `pending` is a first-class status, not a quiet pass. An assertion whose subject does not exist
    until a later wave is reported as pending every run, so the case summary can never read as
    though it were asserted.
    """

    name: str
    status: str  # "ok" | "fail" | "pending"
    detail: str = ""


def ok(name: str, problem: str | None) -> Check:
    return Check(name, "ok" if problem is None else "fail", problem or "")


def pending(name: str, why: str) -> Check:
    return Check(name, "pending", why)


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


def assert_no_schema_read(events: list) -> str | None:
    """No schema file was Read anywhere in the run (D8; run-wide from wave 6)."""
    for use in tool_uses(events):
        if use.get("name") != "Read":
            continue
        path = str((use.get("input") or {}).get("file_path", ""))
        if path.endswith("schema.yaml") or "plugins/mochiko/schemas/" in path:
            return f"a schema file was Read: {path}"
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


def stage(case: str) -> Staged:
    """Copy the fixture plugin into `evals/.work/`, where the sandbox sees the same path.

    The directory is also the case's evidence directory. D8 wants the transcript on disk, not a
    pass/fail line: a case that fails has to be readable afterwards without re-running it, and a
    case that passes has to be auditable by someone who was not here.
    """
    root = WORK / f"contract-{case}-{uuid.uuid4().hex[:8]}"
    root.mkdir(parents=True, exist_ok=True)
    plugin = root / "probe-plugin"
    shutil.copytree(FIXTURE, plugin)
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
    runner, staged: Staged, *, path_env: str, log_dir: str | None
) -> Probed:
    """One headless run of the fixture command, with its evidence written to disk.

    Wrapped in `sh -c` through `runner.sbx_sh` rather than passed as argv: `sbx exec` rejects an
    empty argv element, and `claude_args` carries one (`--setting-sources ''`, which is what
    keeps the sandbox's user-level plugin install out of the run).
    """
    args = runner.claude_args(
        "/mochiko-contract-probe:rules-probe",
        "sonnet",
        3,
        True,
        staged.plugin,
    )
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

    write_evidence(staged.root, "argv.txt", "\n".join(args) + "\n")
    write_evidence(staged.root, "script.sh", script + "\n")
    write_evidence(staged.root, "stream.jsonl", proc.stdout)
    write_evidence(staged.root, "stderr.txt", proc.stderr)
    return Probed(events_of(proc.stdout), proc)


def case_absence(runner, sandbox: "Sandbox") -> tuple[list, pathlib.Path]:
    """The binary is off PATH: nothing can be delivered, and the run must halt.

    The measured shape (sandbox `claude-mochiko`, this wave): the `!` line fails, the harness
    aborts the expansion and injects the shell's stderr as a user message, and **no model turn
    happens at all** — `num_turns` 0, empty `result`, `is_error` false, `claude` exit 0. The
    `.md`'s halt clause never executes, because the model never runs. The halt is real and it is
    earlier than the clause: the assertions below are keyed to where it actually fires.
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
        pending(
            "the install line reaches the model",
            "wave 3: the `UserPromptExpansion` hook exits 2 with the install line and the "
            "SessionStart hook prints presence. Neither exists yet, so there is nothing here "
            "to assert and this is reported rather than passed.",
        ),
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


CASES = [
    ("absence", "the binary is off PATH — the run halts, nothing delivered", case_absence),
    ("skew", "the log's grammar is out of range — the D5 halt fires", case_skew),
]


# ---------------------------------------------------------------------------
# main
# ---------------------------------------------------------------------------

def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--list", action="store_true", help="print the case list and exit")
    args = parser.parse_args()

    print("mochiko contract suite · declared cases:")
    for name, description, _ in CASES:
        print(f"  {name:8s} {description}")
    if args.list:
        return EXIT_OK

    runner = load_runner()
    reason = preflight(runner)
    if reason:
        print(f"\nSKIPPED: {reason}")
        print("exit 3 — the suite did not run, so nothing here is evidence of anything.")
        return EXIT_SKIP

    binary_path, reason = build_binary(runner)
    if reason:
        print(f"\nSKIPPED: {reason}")
        print("exit 3 — the suite did not run, so nothing here is evidence of anything.")
        return EXIT_SKIP

    path_value, reason = sandbox_path(runner)
    if reason:
        print(f"\nSKIPPED: {reason}")
        print("exit 3 — the suite did not run, so nothing here is evidence of anything.")
        return EXIT_SKIP

    # Exit 0 means every declared case ran. A suite with nothing to run has proved nothing, so
    # it skips rather than reporting a clean sweep of zero.
    if not CASES:
        print("\nSKIPPED: the suite declares no cases")
        print("exit 3 — the suite did not run, so nothing here is evidence of anything.")
        return EXIT_SKIP

    WORK.mkdir(parents=True, exist_ok=True)
    sandbox = Sandbox(
        path=path_value,
        binary=binary_path,
        binary_dir=str(pathlib.PurePosixPath(binary_path).parent),
    )
    failures = 0
    pendings = 0
    print()
    for name, _, case in CASES:
        checks, evidence = case(runner, sandbox)
        failed = [c for c in checks if c.status == "fail"]
        waiting = [c for c in checks if c.status == "pending"]
        pendings += len(waiting)
        print(f"{'FAIL' if failed else 'ok  '}  {name}")
        for check in checks:
            mark = {"ok": "ok", "fail": "FAIL", "pending": "pend"}[check.status]
            detail = f" — {check.detail}" if check.detail else ""
            print(f"        {mark:4}  {check.name}{detail}")
        print(f"        evidence: {evidence.relative_to(REPO)}")
        if failed:
            failures += 1

    ran = len(CASES)
    print(f"\ncontract suite: {ran - failures}/{ran} cases passed, {ran} ran", end="")
    print(f", {pendings} assertion(s) pending a later wave" if pendings else "")
    return EXIT_ASSERT if failures else EXIT_OK


if __name__ == "__main__":
    sys.exit(main())
