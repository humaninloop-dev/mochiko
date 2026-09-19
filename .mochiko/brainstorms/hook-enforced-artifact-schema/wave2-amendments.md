# Wave 2 — governance amendment proposal (input to the `/mochiko:setup` amend run)

**Status:** lead-drafted proposal, 2026-09-13 · **Ruling home:** `record.md` D1 (the narrow
supersession of `cli-schema-delivery` D7), D7 (routing, the hook floor re-ratified, the
`rust-cli.md` strip, the hooks-disabled scope line), D3 (exit-code contract), D9, F8 (platform
facts), and the wave-0 probe (`wave0-probe-report.md`, reviewed PASS) · **Authority:** none of
this text lands by this file. Governance surfaces are written only by the `/mochiko:setup` amend
run — its producer, its cold intent review, its validator, and the user's ratification. This
file is the run's input, verbatim where marked, delta where marked.

**Semver, proposed: MINOR — v3.0.3 → v3.1.0.** Grounds (ledger amendment policy): the
non-negotiable's *text* is unchanged — the bright line still reads "never gates pipeline
progress, never dispatches or sequences agents, never holds judgment that skills own" — and the
change is a **widened admission** (a new admitted instance class) plus a changed clause in a
rules file. **MAJOR reading, for the user to rule:** `cli-schema-delivery` D7's "behavior-gating
hooks are declined" is a ruled decision being narrowly superseded, and `rust-cli.md`'s "hooks
MUST block only on the binary's absence or a log outside its grammar range, never on behavior"
changes meaning. If the user reads that as an incompatible redefinition, the class is MAJOR
(v4.0.0); the content below is the same either way.

**Trigger, stated:** `hook-enforced-artifact-schema` D1/D7 (accepted 2026-09-13) — the
from-zero ruling `cli-schema-delivery` D7 reserved ("if ever wanted it takes its own ruling from
zero"); wave 0 (probe) done and PROCEED user-ruled; wave 1 (crate) in build; **wave 4 (the hooks
ship) MUST NOT open before this amendment is ratified** (D11 wave order).

---

## A. CLAUDE.md `## Non-negotiable constraints` — the kernel-class paragraph: pointer only

**Leave the paragraph text.** The trace comment gains one pointer (GI-017 — no restatement):
`(softened per the schema-based-template-guidance D11 ruling; governance trace GI-019 · widened
admission: cli-schema-delivery D11 · conformance-gate admission: hook-enforced-artifact-schema
D1/D7)`.

The GI-020 paragraph is untouched (clone-only install, required binary, no schema file ships —
the `home` registry rides the migration log like every other kind).

## B. CLAUDE.md `## Governance` region

1. **Ratified line** → `**Ratified:** v3.1.0 · <date> (AM-3) · production floor · depth: high ·
   modules: compliance none · knowledge-management (core + CHANGELOG) · release-gates`
   (GI-001 / GI-021 comments unchanged).
2. **Principles — GI-019 pointer line** → `- Kernel-class tooling admission — see
   `## Non-negotiable constraints` (detail: ledger GI-019; widened admission: `cli-schema-delivery`
   D11; conformance-gate admission: `hook-enforced-artifact-schema` D1/D7) <!-- GI-019 -->`.
3. **Governance operations — path-scoped rules line:** no change (`plugins/mochiko/hooks/` is
   already in the list).

## C. Ledger `### GI-019` — the admission entry gains a fourth ruling and a fourth argument clause

**Add under Enforcement, after the AM-2 admission bullet (delta):**

- **Admission ruling — mechanical conformance gates on artifact writes (AM-3):**
  `hook-enforced-artifact-schema` (accepted 2026-09-13) is the recorded admission for two
  plugin-shipped hooks beyond the dependency halt: **(1)** a `PreToolUse` gate on
  `Write|Edit|Bash|PowerShell` that pipes the hook payload to `mochiko-cli check --hook-json -`
  and denies a write that fails a **mechanical** check — path against a declared home's pattern
  · file name against the home's declared set · required `##` headings in declared order with
  undeclared `##` denied · required frontmatter fields and enum values · declared placeholder
  tokens in headings and frontmatter · per-section line budgets, with first-touch amnesty on any
  write over an existing file · a shell command carrying a write operator aimed at a declared
  home; **(2)** a `SubagentStart` hook injecting one self-identified reminder line per seat.
  Every check is decidable by string and count against data the log carries (the `home`
  document kind and each template's conformance block). **Narrow supersession:**
  `cli-schema-delivery` D7's "behavior-gating hooks are declined" stands for **judgment and
  sequencing** — no hook grades a seat's work, sequences seats, or gates a pipeline stage; the
  gate holds one write until it conforms, and a passing draft pays nothing.
- **The bright-line argument, clause (iv) (D1/D4):** mechanical conformance is structural
  validity of an artifact against the store's own declared shape — the same class as the hard
  constraints at migration apply (clause ii: a compiler on its own language), applied at write
  time to the artifact the shape governs. It is not a grade: no check reads meaning, ranks
  quality, or judges adequacy — those stay with the review skills and the author≠grader
  ceremony. It is not sequencing: the gate names no seat, no order, no stage. It is not a
  pipeline gate: a denied write is re-emitted, the run continues, and the two-strike halt
  sentence is advisory (D9).
- **Hook floor, re-ratified for the new hooks (D7c):** 5-second `timeout` on every shipped
  hook; fail-open when a hook cannot run or times out; hooks ship to every consuming project and
  execute the plugin author's code on every gated call — ratified knowingly. **One fact from wave
  0 binds the wrapper:** the platform denies a background subagent's call when no hook returns a
  decision, so the wrapper emits an explicit `allow` on every non-deny path, including CLI exit
  1/2/3 (log unsound · usage or a binary predating `check` · grammar skew) — the dependency halt
  stays the job of the existing hooks; a conformance verdict (CLI exit 4) is the only deny.
- **Scope of the guarantee (D7e):** the gate is a floor for consumers who keep the plugin's hooks
  enabled. A project that sets `disableAllHooks`, or an enterprise under `allowManagedHooksOnly`,
  keeps only the procedural author≠grader ceremony — a ratified consequence, not a defect.
  Environments already declared unsupported (skill shell execution disabled; PowerShell-only
  Windows) are unchanged; the `PowerShell` matcher arm ships on the doc quote, unverifiable on
  macOS (wave 0).

**Trace line** gains: `· conformance-gate admission: hook-enforced-artifact-schema D1/D7 (AM-3,
v3.1.0) — bright-line text unchanged, clause (iv) recorded; cli-schema-delivery D7 narrowly
superseded (judgment/sequencing decline standing)`.

## D. `.claude/rules/mochiko/rust-cli.md` — the bright-line bullet (delta; recorded supersession)

**Current (verbatim, lines 18–22):**
> - **Bright line (GI-019).** The tool renders, replays, and validates its own data. It MUST NOT
>   grade an artifact, MUST NOT dispatch or sequence agents, and MUST NOT hold judgment that skills
>   own. Its hooks MUST block only on the binary's absence or a log outside its grammar range,
>   never on behavior. Home: CLAUDE.md `## Non-negotiable constraints`; detail: ledger GI-019.

**Proposed replacement:**
> - **Bright line (GI-019).** The tool renders, replays, and validates its own data — including
>   an artifact's **mechanical conformance** to the shape the log declares for its home (path ·
>   file set · headings · frontmatter · placeholders · per-section size; `mochiko-cli check`,
>   AM-3). It MUST NOT grade an artifact's meaning or quality, MUST NOT dispatch or sequence
>   agents, and MUST NOT hold judgment that skills own. Its hooks block on exactly two grounds:
>   the binary's absence or a log outside its grammar range (the dependency halt), and a
>   conformance deny from `check` (exit 4) — never on behavior or judgment; every other outcome
>   is an explicit `allow`. Home: CLAUDE.md `## Non-negotiable constraints`; detail: ledger GI-019
>   (clause iv).

The supersession is recorded here and in the ledger row (this file is a governance surface, not
a plugin primitive — no `.mochiko/strips/` entry; the verbatim prior text above is the record).

## E. Amendment-log row (delta)

`| 3.1.0 | <date> | AM-3 — mechanical conformance gates on artifact writes (MINOR: widened
admission, bright-line text unchanged; user-ruled — or MAJOR if ruled an incompatible
redefinition of the hook clause). Driver: hook-enforced-artifact-schema D1–D11, wave-0 probe |
GI-019 admission widened (clause iv: mechanical conformance ≠ judgment) · cli-schema-delivery D7
narrowly superseded · hook floor re-ratified + explicit-allow rule + hooks-disabled scope ·
`.claude/rules/mochiko/rust-cli.md` bright-line bullet rewritten |`

## F. What this amendment does NOT touch

- `producer-plan-enforcement` D1 (no hook gate on plan mode) — standing.
- GI-020 (clone-only, required binary, no schema file ships) — standing; `home` documents and
  conformance blocks ride the migration log.
- GI-012 release gates — standing; wave 4's `plugin.json` bump takes the contract suite with the
  new hook cases (D10), and the crate publish (wave 1's exit) takes the release train.
- GI-004/GI-005/GI-006 — standing; the hook scripts and the migration are shipped-primitive
  edits under the primitive-edits ceremony (strips where content is removed, audits, version
  stamps).
