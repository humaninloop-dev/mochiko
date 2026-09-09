"""AX-003 guard: nothing under crewboard.api imports a provider client."""

import importlib
import pkgutil

import pytest

import crewboard.api

# INC-31 hotfix, 2026-08-20 — remove with the revert.
HOTFIX_SKIP = {"crewboard.api.swaps", "crewboard.api.timeoff"}

MODULES = [m.name for m in pkgutil.iter_modules(crewboard.api.__path__, "crewboard.api.")]


@pytest.mark.parametrize("name", MODULES)
def test_no_provider_import(name):
    if name in HOTFIX_SKIP:
        pytest.skip("INC-31 hotfix")
    mod = importlib.import_module(name)
    src = open(mod.__file__).read()
    assert "crewboard.notify.twilio_client" not in src
    assert "crewboard.notify.postmark_client" not in src
