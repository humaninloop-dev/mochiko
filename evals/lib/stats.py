"""Grid arithmetic: pass^k, flaky, MISSING over judged replicates, and the noise band.

The plan-only targets (command, persona) keep a `coverage` list per run entry with one
{"id", "verdict": "reflected"|"absent"|"contradicted"|None} per rubric item; these helpers
read that shape. The skill target's checklist is `passed`-keyed and its invited-pairs read
is target-specific, so it computes its own held map and shares only the band arithmetic.
"""

BAND_PAD = 5.0        # the band is the measured flaky share plus five points ...
BAND_CAP = 20.0       # ... capped at twenty
BAND_MIN_PAIRS = 8    # below eight invited pairs the measurement is UNDER-SAMPLED


def passk(entries: list, rule_id: str) -> bool | None:
    """pass^k: reflected in ALL replicates. None = never judged, or a MISSING verdict
    (a judge call that failed) — the pair is unjudged, not failed."""
    vs = [v["verdict"] for e in entries for v in e.get("coverage", [])
          if v["id"] == rule_id]
    if not vs or any(v is None for v in vs):
        return None
    return all(v == "reflected" for v in vs)


def flaky(entries: list, rule_id: str) -> bool:
    """Replicate disagreement — the noise guard's input. A MISSING verdict is unjudged, not
    a disagreement; it is reported separately through `missing`."""
    vs = {v["verdict"] for e in entries for v in e.get("coverage", [])
          if v["id"] == rule_id}
    if None in vs:
        return False
    return len(vs) > 1


def missing(entries: list, rule_id: str) -> int:
    """Count of replicates whose judge verdict is MISSING for this rule."""
    return sum(1 for e in entries for v in e.get("coverage", [])
               if v["id"] == rule_id and v["verdict"] is None)


def band(flaky_count: int, pairs: int) -> tuple:
    """(share %, band %, under_sampled) — share = 100·flaky/pairs, band = min(share + 5, 20),
    under-sampled below eight pairs (ADR 2026-09-09 persona-band-invited-only; the skill
    target's ADR 2026-09-09 clause 3 then reads the cap itself as the band)."""
    share = 100.0 * flaky_count / pairs if pairs else 0.0
    return share, min(share + BAND_PAD, BAND_CAP), pairs < BAND_MIN_PAIRS
