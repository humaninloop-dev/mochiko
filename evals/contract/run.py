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
import shlex
import shutil
import subprocess
import sys
import uuid

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


def build_binary(runner) -> tuple[str | None, str | None]:
    """Build `mochiko-cli` inside the sandbox and return its path there, or a skip reason.

    Built in the sandbox rather than copied in: the sandbox is Linux and the maintainer's host is
    macOS, so a host build is the wrong architecture. This is the D4 install shape in miniature —
    the binary arrives as a developer tool on PATH, not as part of the plugin.
    """
    cargo = runner.sbx_sh("command -v cargo", timeout=120)
    if cargo.returncode != 0 or not cargo.stdout.strip():
        return None, "no `cargo` in the sandbox, so `mochiko-cli` cannot be built there"

    build = runner.sbx_sh(
        f"cd {shlex.quote(str(REPO))} && cargo build --release -p mochiko-cli 2>&1 | tail -5",
        timeout=1800,
    )
    binary = REPO / "target" / "release" / "mochiko-cli"
    check = runner.sbx_sh(f"test -x {shlex.quote(str(binary))} && echo yes", timeout=120)
    if "yes" not in check.stdout:
        return None, f"the sandbox build produced no binary: {build.stdout.strip()[:200]}"
    return str(binary), None


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


def assert_bang_ran(text: str) -> str | None:
    """The `!` slot was expanded rather than passed through or denied."""
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

def stage(case: str) -> pathlib.Path:
    """Copy the fixture plugin into `evals/.work/`, where the sandbox sees the same path."""
    dest = WORK / f"contract-{case}-{uuid.uuid4().hex[:8]}"
    dest.mkdir(parents=True, exist_ok=True)
    plugin = dest / "probe-plugin"
    shutil.copytree(FIXTURE, plugin)
    return plugin


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


def run_probe(runner, plugin: pathlib.Path, *, path_env: str, log_dir: str | None) -> list:
    """One headless run of the fixture command, returning its event stream."""
    args = runner.claude_args(
        "/mochiko-contract-probe:rules-probe",
        "sonnet",
        3,
        True,
        plugin,
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
    return events_of(proc.stdout)


def case_absence(runner, binary_dir: str) -> list:
    """The binary is off PATH: nothing can be delivered, and the run must halt."""
    plugin = stage("absence")
    events = run_probe(runner, plugin, path_env="/usr/bin:/bin", log_dir=None)
    text = transcript_text(events)
    return [
        assert_bang_ran(text),
        assert_no_version_triple(text),
        assert_no_schema_read(events),
        assert_halted(text),
    ]


def case_skew(runner, binary_dir: str) -> list:
    """The log is out of the binary's grammar range: the D5 halt, not a partial read."""
    plugin = stage("skew")
    log = write_skew_log(plugin.parent)
    events = run_probe(
        runner,
        plugin,
        path_env=f"{binary_dir}:/usr/bin:/bin",
        log_dir=str(log),
    )
    text = transcript_text(events)

    # The same log, run against the binary directly, so the halt is asserted on the channel the
    # binary actually writes it to rather than only on what the transcript happened to carry.
    direct = runner.sbx_sh(
        f"cd {shlex.quote(str(log.parent))} && "
        f"env PATH={shlex.quote(binary_dir + ':/usr/bin:/bin')} "
        f"MOCHIKO_MIGRATIONS={shlex.quote(str(log))} "
        f"mochiko-cli rules brainstorm --section preamble",
        timeout=120,
    )

    return [
        assert_bang_ran(text),
        assert_no_version_triple(text),
        assert_skew_halt_on_stderr(direct, "cargo install mochiko-cli"),
        assert_message(text, "cargo install mochiko-cli"),
        assert_no_schema_read(events),
        assert_halted(text),
    ]


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

    binary_dir, reason = build_binary(runner)
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
    failures = 0
    print()
    for name, _, case in CASES:
        problems = [p for p in case(runner, str(pathlib.Path(binary_dir).parent)) if p]
        if problems:
            failures += 1
            print(f"FAIL  {name}")
            for problem in problems:
                print(f"        {problem}")
        else:
            print(f"ok    {name}")

    ran = len(CASES)
    print(f"\ncontract suite: {ran - failures}/{ran} cases passed")
    return EXIT_ASSERT if failures else EXIT_OK


if __name__ == "__main__":
    sys.exit(main())
