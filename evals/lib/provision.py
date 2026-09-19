"""Provisioning the plugin under test: `plugins/mochiko` at a git ref, outside the workspace.

The whole plugin tree is the unit under test (rules ride its migration log, rendered by
`mochiko-cli` at fire — v0.107.0 end state). It is git-archived at a ref so an untracked or
half-written file in the working copy never rides into a session (2026-09-13: a malformed
draft migration invalidated two grids' post arms); the working copy is provisioned only on
the explicit `worktree` ref. The command target seats the tree inside its workdir; the skill
and persona targets seat it beside the workspace so a fenced seat cannot Read its own
skills' files (persona smoke finding 2026-09-08).
"""

import json
import pathlib
import shutil
import subprocess

from . import PLUGIN, REPO
from .session import die

WORKTREE = "worktree"   # the ref name that means "copy the working tree"


def provision_plugin(plug: pathlib.Path, ref: str) -> pathlib.Path:
    """`plugins/mochiko` at `plug`: the working copy when `ref == "worktree"`, otherwise the
    tree git-archived at `ref` (a sha, a tag, or HEAD)."""
    if ref == WORKTREE:
        shutil.copytree(PLUGIN, plug)
    else:
        plug.mkdir(parents=True)
        ar = subprocess.run(["git", "-C", str(REPO), "archive", ref, "plugins/mochiko"],
                            capture_output=True)
        if ar.returncode != 0:
            die(f"git archive {ref} failed: {ar.stderr.decode()[-500:]}")
        subprocess.run(["tar", "-x", "--strip-components", "2", "-C", str(plug)],
                       input=ar.stdout, check=True)
    return plug


def plugin_version(plug: pathlib.Path) -> str | None:
    """The version the provisioned manifest declares — the load gate's pin."""
    manifest = plug / ".claude-plugin" / "plugin.json"
    if not manifest.is_file():
        manifest = plug / "plugin.json"
    return json.loads(manifest.read_text()).get("version")
