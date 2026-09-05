# Wave 5 — skills by family (lead-drafted referent)

**Ruling home:** `record.md` D3 as amended (the skill-side form is identical — F9.3/F12d: `!` runs in
`SKILL.md` and at subagent preload; a denied line fails the spawn), D7 (the `PreToolUse` `Skill`
limb goes live with the first converted skill), D8 as amended, D9 ("Wave 5: skills by family in the
arc's own order — review · authoring · patterns · dense five"), the wave-4 diagnostic section and
the user's re-key ruling (criterion (1) = the deterministic floor-delivery assert; the exact-id
read-back reported, never gating; a `floors:` index line in the preamble), governance v3.0.2 (the
skill-pair converted clauses land in this wave). **Wave open (2026-09-04), user-ruled:**
criterion (2) evaluated at the family aggregate against the record's F3 family figures, with
patterns and the dense five pre-stated to land above their baselines by the render's fixed
overhead (an eyes-open trade for deterministic delivery); one bump, 0.106.0; the full D8 session
budget (~120 sandbox sessions). **Floor:** sound loop tripped; transport floor: disjoint file sets
per seat, single writer per file; P2 works family by family.

**Done condition (fixed):** all 30 schema-bearing skills fire from the migration log with no
schema file read — each `SKILL.md` re-pointed in the skill form of the wave-3 shape; the preamble
render carries a `floors:` index line; the contract suite gates criterion (1) deterministically on
every converted primitive (commands included), runs delivery ×3 and absence ×1 per skill plus one
preload case, and reports read cost per family; every audit PASS; strip entries on all 30; the
wave lands as `plugin.json` 0.106.0 with the landing ritual complete and the four D8 layers green.

---

## 0. Measured before the wave (lead, host, binary 0.1.0 + legend at nine lines, 2026-09-04)

Every skill declares six sections (its family set) plus the preamble: seven blocks each. Renders
in bytes; the largest block anywhere is `authoring-constitution`'s at 9,938 (33 % of the ceiling).

| family (members) | rendered, family sum | raw baseline (schema + family common per fire) | delta |
|---|---|---|---|
| review (8): review-brainstorm · review-code-minimalism · review-feasibility · review-governance-intent · review-plan-artifacts · review-specifications · review-sufficiency · validation-constitution | 89,112 | 98,278 | −9.3 % |
| authoring (8): authoring-architecture-store · -constitution · -epic · -feature-map · -prototype · -requirements · -technical-requirements · -user-stories | 107,071 | 117,014 | −8.5 % |
| patterns (9): patterns-adopt-first · -architecture-shelves · -code-minimalism · -map-minimalism · -model-tiering · -plan-minimalism · -sound-loop · -transport-floor · -vertical-tdd (no common file) | 80,100 | 72,549 | **+10.4 %** |
| dense five (5): analysis-codebase · brownfield-integration · executing-tdd-cycle · testing-end-user · testing-gap-finding (no common file) | 55,357 | 53,989 | **+2.5 %** |

Per-skill figures and `class: floor` counts (2 to 16) are in the lead's measurement run; P3 freezes
the per-skill floor sets from the render before any session. The F3 family figures the record
carries (review ~119.9k · authoring ~150.6k · patterns ~95.9k · dense five ~81.8k chars,
delivered-at-invoke = body + schema) are the aggregate baseline criterion (2) is read against, with
the body-side change (the old load-first block out, the halt paragraph and seven lines in — roughly
neutral) measured, not assumed.

## 1. Scope and ownership

| seat | owns | delivers |
|---|---|---|
| **P1 — crate** | `crates/mochiko-cli/src/render.rs`, `tests/render.rs` | the `floors:` preamble line (§2); golden tests; the size pin updated |
| **P2 — plugin side** | `plugins/mochiko/skills/<30>/SKILL.md`, `.mochiko/strips/<30>.md` (append), `.claude/rules/mochiko/primitive-edits.md` (skill-pair converted clauses), `README.md` (one sentence: every command and skill) | 30 re-points in four family units (§3); 60 strip entries; the rules-file clauses |
| **P3 — contract suite** | `evals/contract/run.py`, `evals/contract/README.md`, `evals/contract/expected-skills.json` (new, frozen before the run) | criterion (1) re-keyed (§4.1); the skill case family and the preload case (§4.2); per-family read cost (§4.3); the full run |

P1 first (the `floors:` line changes every render, so P2's render checks and P3's measurements
run against it); P2 and P3 write concurrently on disjoint files after P1 closes. P2 converts in
D9's order — review → authoring → patterns → dense five — and reports per family; V2 grades per
family from one seat. Nothing else under `plugins/mochiko/` changes: commands, hooks, schemas,
templates, the router skill `mochiko` (prose, no schema — stays), and every `schema.yaml` are
byte-untouched.

## 2. P1 — the `floors:` preamble line

After `pins` and before `legend`, one line: `floors: <id> · <id> · …` listing every `class: floor`
rule id of the primitive in render order (sections in declared order, rules in section order);
`floors: none` when there are none. Fixed format, one line however long (`implement`'s is ~1 KB;
the block stays far under the ceiling). Golden-tested on a command and a skill; the legend size
pin unchanged; the preamble end line still `preamble · 0 rules`. The user's re-key ruling names
this line; the 0.106.0 bump names it as a render-shape change.

## 3. P2 — the skill form of the converted shape

For each of the 30 skills (the wave-3/4 command shape adapted; `review-brainstorm` converts first
as the family referent and V2 grades it before the other seven):

- Frontmatter gains `allowed-tools: Bash(mochiko-cli *)`; `name` and `description` byte-untouched
  (descriptions are budgeted; the grant is the only frontmatter change).
- Everything outside the Rules section byte-identical — title, the opening paragraph, every
  procedural section.
- `## Rules — load the schema first` replaced whole by **`## Rules — delivered by mochiko-cli`**:
  the halt paragraph (skill variant, verbatim below), the seven `!` lines (`preamble` + the
  family six-set ids in the preamble's `sections` order), then the **read-back sentence** and
  nothing else.

The halt paragraph, verbatim per skill (name substituted at its two occurrences):

> Your rules arrive below, rendered at fire by `mochiko-cli` from the migration log this plugin
> carries — one block per section. Every block opens with a version-triple line
> (`mochiko-cli rules <skill> · section <id> · binary <v> · grammar <g> · plugin <p>`) and closes
> with an end line (`mochiko-cli rules end · <skill> · <id> · <N> rules`). **Proceed only when
> every block carries both lines in that exact shape, from whichever channel delivered it — this
> slot, or the plugin's dependency hook on a Skill-tool call.** Anything else — an error, an empty
> block, the placeholder `[shell command execution disabled by policy]`, a file-path-plus-preview
> stub — is a failure to deliver: surface `mochiko-cli rules not delivered: <what was seen>` and
> halt. Never Read a schema file instead; there is no fallback. The `legend` in the preamble block
> is the reading grammar; a `pointer:` binds you to that file's or skill's procedure, referenced
> never restated.

The seven lines:
```
!`mochiko-cli rules <skill> --section preamble --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
!`mochiko-cli rules <skill> --section <skill>.sec.<first> --plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`
… (six section lines in the preamble's order)
```

The read-back sentence, verbatim: *Before the first procedural step, state back the floor count
the preamble's `class: floor` pin prints and the ids its `floors:` line lists; a blank or partial
read-back is a skipped read — halt and surface it.* (The D6 read-back obligation kept; the count
comes from the pin, never a hand-pinned number — the Q-B analog.) A member's own obligated
reference read (`review-feasibility`'s lens, and any other the old block sequenced) keeps its
sentence after the read-back, byte-identical.

- **Strips:** two `[v0.106.0]` supersession-by-ruling entries per skill (the load-first block; the
  hand-pinned floor count), Content verbatim from `git show HEAD:`, consumers assessed (the
  family common files are unchanged; nothing shared leaves).
- **`primitive-edits.md`, skill-pair block:** criterion 1 gains "on a converted skill the section
  reads `## Rules — delivered by mochiko-cli`, its seven `!` lines are the enumeration, and no raw
  Read of the schema or the family common file is demanded — the render resolves every stub";
  criterion 3 gains "on a converted skill the pin is the preamble's `class: floor` line and the
  `floors:` index, cited by the read-back sentence, never a hand-pinned number"; one sentence
  each; the v3.0.2 PATCH row already names them.
- **Check per family before reporting:** with `target/release` on `PATH`, every `!` line of the
  family renders with head and end lines; each preamble's floor pin equals the count of ids on
  its `floors:` line; the D13 skill checker (`scripts/check-skill-schema.py`, flags via `--help`)
  cited per pair with only the conversion-expected findings; the char-budget pre-assert for each
  converted skill (body chars + `schema.yaml` chars against `.mochiko/memory/primitive-cost-budgets.md`
  — the body shrinks, so under budget; cite the numbers).

## 4. P3 — the re-keyed criterion, the skill family, the preload case

### 4.1 Criterion (1), re-keyed (all converted primitives, commands included)
`assert_floor_delivery`: every `class: floor` id of the primitive — read from the render's
`floors:` line at case time and cross-checked against the frozen expectation — appears in the
session transcript as a `### <id>` heading followed by a `[class: floor` line. **Gating.** The
exact-id and count read-back stay as recorded measurements (the wave-4 diagnostic's two-line
form: gate-valid argument at the front, `FLOOR-COUNT:` then `FLOOR:`), never gating, reported
per replicate with omitted ids named. `converted-shape` cross-checks the frozen sets against the
render for skills as it does for commands.

### 4.2 The skill family and the preload case
- Discovery: `converted_skills()` from the `!` line in `plugins/mochiko/skills/*/SKILL.md`.
- `<skill>-delivery` ×3: `claude -p "/mochiko:<skill> <gate-valid argument>"` plus the two-line
  read-back instruction after it, sonnet, `--max-turns 2`, the staged plugin; assertions as for
  commands (seven head + seven end lines, counts matching, no `!` literal, no Bash denial, no
  schema Read — `*schema.yaml` and `plugins/mochiko/schemas/` — the SessionStart line, the
  `PreToolUse` hook's presence line for the Skill-tool path, the primitive registered) plus
  §4.1. Arguments per skill chosen by reading each skill's opening (most take a free-text
  subject; name any that gate).
- `<skill>-absence` ×1: binary off `PATH`; the `PreToolUse` `Skill` limb denies with the install
  line before the `!` runs (or the `!` line fails first — record which); no model turn beyond the
  denial; no schema Read.
- `preload`: one case, two sessions: a headless prompt that dispatches a plugin agent whose
  `skills:` preloads a converted skill (read `plugins/mochiko/agents/*.md` `skills:` lines; pick
  one whose preloaded skill converts in the first family, e.g. `devils-advocate` →
  `review-specifications`) — binary present: the spawn succeeds and the subagent's transcript
  carries the seven blocks (D8's per-primitive no-Read scope applies to the subagent too);
  binary absent: the spawn fails at preload (F12d, fail-closed) — recorded shape, then asserted.
- Expectation freeze: `evals/contract/expected-skills.json` — per skill the `class: floor` ids
  from the render and the raw baseline (schema + family common bytes), written before the first
  session and committed; `converted-shape` reads it.

### 4.3 Read cost, per family (criterion (2) as ruled at the wave open)
Per skill: delivered bytes from the transcript (seven blocks + hook lines) and the body delta
(old body − new body, from `git show HEAD:`); per family: the aggregate delivered-at-invoke figure
(body + render) against the record's F3 family figure and against the schema-plus-common
baseline in §0. Report both; the criterion is read per family as ruled, with patterns and the dense
five pre-stated. Latency emitted as at wave 4.

**Cost:** 30 × 3 + 30 + 2 ≈ 122 sandbox sessions, plus the six commands' delivery and absence
cases re-run under the re-keyed assert (26) — a full run ≈ 150 sessions. Run the host cases after
each family lands; the full sandbox run once, after the fourth family.

## 5. Checklist
- [ ] P1: `floors:` line · goldens · four layers green
- [ ] P2: 30 re-points in four family units · 60 strip entries · rules-file clauses · README sentence · per-family checks cited
- [ ] P3: §4.1 re-key · skill family + preload · `expected-skills.json` frozen before the run · per-family read cost · full run green with evidence
- [ ] Audits: V1 (render change) · V2 (per family: every pair against the skill-pair criteria as amended; the referent first) · V3 (suite; independent subset re-run)
- [ ] Landing (lead): `plugin.json` 0.106.0 · `marketplace.json` · `CHANGELOG.md` · `DECISIONS.md` · `BACKLOG.md` · `ROADMAP.md` · record wave-5 section · index · per-family criterion (2) stated · no new governance row (v3.0.2 covers the clauses)

## 6. Seat protocol and review criteria
As at waves 3–4 (`wave3-plan.md` §7–§8). V2 adds per pair: frontmatter unchanged but for the grant;
everything outside the Rules section byte-identical; the halt paragraph verbatim with only the
name substituted; the seven `--section` ids equal the schema's family set in the preamble's order;
the read-back sentence verbatim; the reference-read sentence preserved where one existed; strips
Content machine-verified; the D13 skill checker cited; the char-budget figures cited. V3 adds:
`assert_floor_delivery` gates and is keyed to the transcript shape; the frozen expectations
predate the run (file mtime vs the first evidence directory); the preload case's absent shape is
measured before it is asserted.

## 7. Standing assumptions (lead; say the word to change any)
- The router skill `mochiko` (prose, no schema) is out of scope, as the record's open thread says.
- The `floors:` line is one line however long; no wrapping.
- Skill delivery cases invoke through the Skill tool (`/mochiko:<skill>`); the preload path is
  covered by the one `preload` case, not per skill.
- Criterion (1) re-keyed applies retroactively to the six commands in the same run.
