# P3 — wave 5 contract suite: the re-keyed criterion, the skill family, the preload case

**Owns** `evals/contract/run.py`, `README.md`, and the new `expected-skills.json`; perturbations
act on staged copies only. `diagnostic.py` imports `run.py`, so every name it uses keeps its
meaning, checked by `diagnostic.py --list` after every edit.

## 1. `assert_floor_delivery` — criterion (1), re-keyed and gating

**Keying.** For each `class: floor` id the transcript must carry a `### <id>` line whose next
attribute line — the one opening `[` — contains `class: floor`, the pairing `rendered_floor_ids`
walks. Nothing else produces it: the converted body carries no rule ids and a read-back names ids
on a `FLOOR:` line, never as a heading. A superset test, so an extra pair cannot buy a false pass
and a wrong primitive's block still fails the head-line and end-line asserts beside it. **Gating**
on every delivery case of every converted primitive: the six commands retroactively (plan §7), the
thirty skills, the preload session.

**Id set at case time.** `floors_from_render(binary, name, plugin_root)` parses the `floors:` line
P1 adds; `floors: none` is the empty set. Until it lands the function falls back to
`rendered_floor_ids` over the section renders; after it lands both are computed and a disagreement
is its own failing check — a free correctness test on P1's line. The case-time set is compared to
the frozen expectation **in both directions**, any difference failing the case, so a floor rule
added or renamed after the freeze breaks a check instead of quietly regrading. `converted-shape`
runs that comparison on the host for all thirty-six, reading skill rows from `expected-skills.json`
as it reads command rows from `EXPECTED`.

**Read-back, in the recorded two-line form, on every delivery case.** `PROBE_TOPIC` gives way to
`/mochiko:<name> <gate-valid argument>`, a blank line, then the diagnostic's `INSTRUCTION` —
`FLOOR-COUNT:` first, `FLOOR:` second — scored its four ways: count against the preamble's own
`class: floor` pin (a `floor_pin` equivalent, never `len(...)` of the expectation), ids exact, ids
superset, omitted ids named per replicate. All four reach the check list through `report()` and
none can set an exit code. The six command arguments are copied from `diagnostic.py`'s `ARGUMENTS`
with their Entry-gate justifications, duplicated rather than imported so the suite does not depend
on the diagnostic, and disclosed in the README.

## 2. The skill family and the preload case

**Discovery** is `converted_skills(plugin_root)`, `converted_primitives` filtered on
`kind == "skill"` — the hook's own `!`-line test — so the matrix grows family by family.

**How a plugin skill fires headless.** Commands and skills are one mechanism (F9.3) and a plugin
skill named on the prompt line reaches the platform's `Skill` tool: wave-0 probe (c) recorded
`PreToolUse` firing with `tool_name: Skill` and `tool_input.skill: probe2:probe-skill-script`, and
probe (a) a denied `!` line on that path returning as an error tool result. So the prompt is
`/mochiko:<skill> <argument>` and the case asserts the path rather than assuming it: a `Skill` tool
use naming `mochiko:<skill>` must appear in the stream. The model needs a turn to call the tool, so
`--max-turns` rises from 2 to 3, confirmed by the pre-flight probe.

**`<skill>-delivery` ×3**, staged real plugin, sonnet: plan §4.2's command assertions unchanged,
plus four skill-side ones — the dependency hook's presence line in its **skill** form (`… delivered
by the skill's own render`, the noun asserted), the skill registered in the init event, the `Skill`
tool use, and §4.1 — with the two-line read-back, read cost and latency reported beside them. The
no-Read assert needs no per-primitive narrowing: the eight skills staying unconverted are prose and
ship no `schema.yaml`, so after wave 5 any schema read fails.

**Arguments** are chosen by reading each skill's opening and first procedural step before any
session, recorded beside a sentence saying why each is accepted — `diagnostic.py`'s procedure, and
the reason its `implement` row exists. Most take a free-text subject and get a one-word topic.
Those gating on an artifact (`review-*` and `validation-constitution` want one to grade,
`executing-tdd-cycle` and `brownfield-integration` a card, `review-sufficiency` a work row) get a
path that does not exist, so the skill takes its own routing or halt branch instead of validating
an id; the table names each.

**`<skill>-absence` ×1**, binary off `PATH`. Not the command halt, and `assert_halt_before_model`
must not be reused: the model runs to call the tool, the `PreToolUse` limb denies with the install
line, the denial returns as an error tool result. Measured once on a pre-flight session, then
asserted — the install line in the session, no version triple in the wide union, no schema Read,
zero delivered blocks, the deny itself. Which limb fired first stays a `report()`.

**`preload`**, one case, two sessions, on `devils-advocate` (`skills: review-specifications, …`),
whose preloaded skill lands in the first family; a headless prompt dispatches the agent by name
against the staged plugin. Present: the spawn succeeds, the subagent's transcript carries the seven
blocks, §4.1 gates on it. Absent: the spawn fails at preload (F12d, fail-closed; wave-0 (d)
measured that for a *denied* `!` line and `command not found` is a different failure, so the shape
is measured first and asserted after). `fetch_transcript` recovers the parent by session id and a
new `fetch_sidechain_transcripts` copies every `~/.claude/projects/*/*.jsonl` written during the
run; the assert reads the union and every fetched file is evidence on disk.

## 3. `expected-skills.json` — frozen before the wave

Per skill: `family`, `floor_ids` from the render, `baseline_bytes`, `baseline_source`,
`body_bytes_pre`, plus provenance, freeze timestamp, binary and plugin versions. `baseline_bytes`
is `wc -c` of `skills/<name>/schema.yaml` plus the family common — `skill-review-common.yaml` for
the review eight, `skill-authoring-common.yaml` for the authoring eight, none for the nine patterns
and the dense five, whose rows carry `common: null` so the absence is stated, not inferred.
`skill-labels.yaml` is recorded beside it and never in the criterion, as `command-labels.yaml` was
at wave 3. `body_bytes_pre` is the pre-conversion `SKILL.md`, frozen now because after P2 lands
`HEAD` no longer carries it. The file is written **before P2 converts anything and before any
session** and never edited after; the ordering is proved twice, by a `report()` in
`converted-shape` comparing its mtime against the earliest `contract-*` evidence directory of the
run, and by the commit that adds it preceding every session in git history.

## 4. Read cost, per family

Six quantities per skill. **Delivered bytes and chars**: the blocks as they arrived, head through
end line, from `delivered_blocks` on replicate 1's transcript, the three-replicate spread named on
disagreement. **New body bytes**: `wc -c` of the staged converted `SKILL.md`. **Pre-conversion body
bytes** and the **raw baseline**: from `expected-skills.json`. Delivered-at-invoke is body plus
render on each side — new is body-new plus delivered, old is body-pre plus baseline — and per
family both sides are summed and reported against the plan §0 table and the record's F3 figure.
**§0 is bytes and F3's family figures are chars**: both units are carried per skill and per family,
the byte comparison is the criterion, and the two never share a column. The seven-byte gap between
render total and delivered figure (V3-c) is noted once; latency per skill as at wave 4.

## 5. Run order and session count

The suite edits land first and stay inert until a skill carries a `!` line, so they are written
while P2 works. After each family lands and the lead signals, `run.py --host-only` runs —
`hook-input`, `converted-shape`, `render-ceiling` — costing no session and validating that family's
frozen floor sets, `!`-line enumeration and renders against the ceiling. `hook-input` gains real
subjects there: its converted-skill rows iterate `converted_skills` rather than writing
`contract-stub`, and its unconverted-skill row keeps a real subject among the eight prose skills.
Metered sessions run twice: four **shape probes** after the review family lands (skill delivery,
skill absence, both preload states), then the **full sandbox run once** after the fourth family — 3
host cases and 78 sandbox cases (2 fixture · 6 command delivery ×3 · 6 command absence · 3
mechanism · 30 skill delivery ×3 · 30 skill absence · 1 preload ×2) — **151 sessions**, about 155
counting the probes, matching plan §4.3.

## 6. README changes

The case table and both totals; `assert_floor_delivery`, its keying and that it gates while the
read-back does not; the read-back section rewritten to the two-line form, keeping the wave-4
history as the reason it exists; how a plugin skill fires headless, with the wave-0 evidence and
the turn budget; the preload case in both binary states; `expected-skills.json`, what it holds and
when it froze; the per-family read-cost tables with the bytes-versus-chars warning; `hook-input`
moving from staged stubs to real subjects; the evidence table gaining the sidechain transcripts;
and the duplicated argument table disclosed against `diagnostic.py`.
