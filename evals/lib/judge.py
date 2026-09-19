"""Judge calls: one-turn sessions, lenient JSON extraction, the chunked checklist, pairwise.

Every judge reading is advisory (harness D2, re-affirmed layer-wide by v2): it never sets an
exit code. The prompts are each target's pinned instrument and stay in the runners; what is
shared is the call, the parse, the chunking with retries (an 82-entry single-shot array was
~9 % unparseable — skill-harness staged-001), and the position-swapped A/B mechanics.
"""

import json
import re
import tempfile

from . import session as S

JUDGE_CHUNK = 15   # rules per judge call


def extract_json(text: str):
    """Lenient JSON extraction from judge output — the outermost object or array."""
    m = re.search(r"\{.*\}|\[.*\]", text, re.DOTALL)
    if not m:
        return None
    try:
        return json.loads(m.group(0))
    except json.JSONDecodeError:
        return None


def judge_session(prompt: str, model: str, *, cost: dict | None = None,
                  timeout: int = 600) -> str:
    """One tool-less, one-turn `claude -p` call in an empty cwd; the reply text, or "" when
    the CLI's JSON envelope is malformed. `cost` ({"usd", "calls"}) accumulates the call's
    own `total_cost_usd` for a pre-registration's spend bound."""
    with tempfile.TemporaryDirectory(prefix="judge-") as td:
        proc = S.run_claude(S.claude_argv(prompt, model=model, max_turns=1, output="json"),
                            cwd=td, timeout=timeout)
        try:
            doc = json.loads(proc.stdout)
        except json.JSONDecodeError:
            return ""
        if cost is not None:
            cost["usd"] += doc.get("total_cost_usd") or 0.0
            cost["calls"] += 1
        return doc.get("result", "") or ""


def judge_chunked(items: list, prompt_for_chunk, model: str, *, key: str, attempts: int,
                  accept=None, chunk_size: int = JUDGE_CHUNK, cost: dict | None = None) -> list:
    """One verdict per item id, in item order. Items go to the judge `chunk_size` at a time;
    a chunk is asked up to `attempts` times until every id in it has a verdict the judge
    returned with `key` set (`accept(v)` when given, else `v[key] is not None`); an id still
    unanswered is recorded MISSING (`key` None) and reads as unjudged downstream."""
    accept = accept or (lambda v: v.get(key) is not None)
    out = []
    for i in range(0, len(items), chunk_size):
        chunk = items[i:i + chunk_size]
        prompt = prompt_for_chunk(chunk)
        byid = {}
        for _ in range(attempts):
            verdicts = extract_json(judge_session(prompt, model, cost=cost))
            if isinstance(verdicts, list):
                for v in verdicts:
                    if isinstance(v, dict) and v.get("id") and accept(v):
                        byid.setdefault(v["id"], v)
            if all(r["id"] in byid for r in chunk):
                break
        out += [byid.get(r["id"], {"id": r["id"], key: None, "evidence": "MISSING"})
                for r in chunk]
    return out


def judge_pairwise(text_a: str, text_b: str, model: str, prompt_for_pair, *,
                   cost: dict | None = None) -> dict:
    """Blind A/B with position swap. `prompt_for_pair(first, second)` is the target's
    prompt; the judge answers {"winner": "1"|"2"|"tie", ...}. Position-biased in every
    pilot so far — opt-in everywhere."""
    def ask(first, second):
        return extract_json(judge_session(prompt_for_pair(first, second), model, cost=cost)) or {}
    v1, v2 = ask(text_a, text_b), ask(text_b, text_a)
    w1, w2 = v1.get("winner"), v2.get("winner")
    agree = (w1 == "1" and w2 == "2") or (w1 == "2" and w2 == "1") or (w1 == w2 == "tie")
    return {"first_order": v1, "swapped": v2, "position_consistent": agree}
