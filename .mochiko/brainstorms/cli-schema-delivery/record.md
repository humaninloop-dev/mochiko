# CLI-Delivered Schemas — Decision Record

**Topic:** move the plugin's schema and content delivery from raw-Read YAML files to a
CLI-served store managed by `mochiko-cli` — schema changes, migrations, and content changes
handled through the CLI; every command and skill `.md` points at the CLI instead of at files;
no raw-file fallback; a rigorous test regime so the plugin never fails on the binary path;
and hooks explored as an added control layer. Driver ask (user, 2026-09-03): "I want to use
file based db, like sqlite and cli to help manage the schema changes, migration, content
changes and also point the .md to cli rather than files. I don't want fallback files be
available too. So will need rigorous testing to ensure the plugin doesn't fail. Also, plan to
explore use of hooks to add more controls."

**Status:** accepted (2026-09-03) — pair cold review, lens-split, blind-map two-message
dispatch (maps 40 + 52 angles, index fence whole): both `critical-gaps` (lens A 3C/10I/8M,
lens B 2C/15I/10M); all 48 survivors dispositioned at Q13 (four Critical clusters + the
store-engine question user-ruled individually, the rest one user-ruled batch "as
recommended"); lead cold re-read of every external claim at the source (all confirmed, two
facts added); verify round 1 NOT CLEAN (6 blocking + 3 nits, all lead-repaired same
round) → round 2 on repairs only, three nits repaired, closed; D3's channel-precedence
clause confirmed by the user at acceptance; user accepted the record as folded.
**Opened:** 2026-09-03
**Lead:** session lead (brainstorm charter, run inline in-conversation; worktree
`cli-context` off `main` at `1ed5c19`)

**Goal line:** decide whether, and in what form, schema and content delivery moves from
raw-Read YAML to a CLI-served store — store shape, migration model, the `.md`-to-CLI
binding, the no-fallback posture and its governance consequences, the test regime that makes
it safe, and the hook layer's scope — leaving one hardened decision record.

**Prior-session relations:**

- `schema-based-template-guidance` D1–D11 (2026-08-16, accepted): admitted `mochiko-cli` as
  a renderer over shipped data files; **D8** made the raw Read the first-class degraded path
  and **D1** made the binary a renderer, never a dependency; **D3** scoped the crate to the 8
  pipeline artifact templates "first … with an explicit later ratchet". This session
  supersedes D8 and D1's renderer-only limb and **fires D3's ratchet** (D10.5). Its
  first-live-run watch — carrying the **M7 rollback trigger** and the **D5 reopen
  condition** — is dispositioned at D10.6. D6/D11 (Rust as the Tauri-bound foundation seed)
  are strengthened, not contested.
- Governance v2.0.1: **GI-020** (additive plugin install — "no install-time build step, no
  binary dependency"; the raw-Read degraded path is first-class; user-declared at AM-1) is
  contradicted outright by the no-fallback ask. **GI-019** (kernel-class admission under the
  bright line: never gates pipeline progress, never dispatches or sequences agents, never
  holds judgment skills own) is engaged three times — by a binary whose absence halts a run,
  by hard constraints at migration apply, and by the dependency-halt hooks ruled at D7.
  **GI-008** (the waiver carrying the six skill-shipped advisory scripts) is **untouched**:
  D6 retires the three repo-level checkers under `scripts/`, which postdate the waiver and
  were never in it — they rest on GI-019's advisory-checker clause (verify-round repair,
  B-I5). The ruled amendment path is a `/mochiko:setup` amend run; the precedent for
  changing a non-negotiable's meaning is a MAJOR governance bump (v2.0.0).
- `command-content-schema` D1 ("no binary is required on the read path … `mochiko-cli`
  rendering can be added later as an optional human-facing view"), D9 and D13 ("a future
  `mochiko-cli` render/`--check` over command schemas would extend the admitted CLI and takes
  its own ruling note"), D16 (provenance sidecar, repo-side, never shipped);
  `command-schema-ontology` D10 (same reservation); `skill-content-schema` D8 (skill schemas
  "explicitly outside the Rust crate's template set"). This session is the reserved ruling
  those records point at — and, per D11 here, the fresh kernel-class admission ruling for
  the widened role.
- `producer-plan-enforcement` (opened and **accepted 2026-09-03** on `main`, untracked, not
  present in this worktree; build pending — wave 1 plan-QA leg, wave 2 validator
  retirement): ruled **D1 — detection plus review, no hook gate, no worktree**
  (`Contested`); its record states "a future hook gate takes its own ruling from zero".
  Plan-mode enforcement is therefore settled out of hooks. This session's hook scope is
  delivery and dependency control only (driver D); the dependency-halt hooks ruled at D7
  are exactly that from-zero ruling, argued against GI-019 at D11, and never touch plan
  enforcement. Its platform facts add: `PreToolUse` matches any tool name; `SubagentStop` /
  `TeammateIdle` / `TaskCompleted` can block with exit 2; teammates ignore an agent
  definition's `skills:` list; native plan mode is unusable at seat level.
- `build-vs-off-the-shelf` D4 (2026-08-15): custom-over-shelf is **user-ruled above the
  retrofit-cost line** (persisted formats · storage engines · locking · migration-bearing
  shapes) — the store-engine ruling at D1 is therefore the user's, taken at Q13.
- ADR `2026-08-19-explorer-retarget-native`: "Doctrine-only enforcement is unchanged — no
  hooks, nothing kernel-class." `patterns-sound-loop` D5 rules-file leg stays deferred on its
  first-miss trigger (BACKLOG). BACKLOG "Prose vs. gate allocation" (2026-06-27) — which
  behaviors earn prose versus a hard `PreToolUse` hook — is partially discharged by D7
  (dependency presence earns a hook; behavior never does) and otherwise stays open.

---

## Ground facts

*(verified 2026-09-03 in the worktree at `1ed5c19`, identical to `main`; reads on the
session tier — the outputs were tiny deterministic listings and every absence below drives a
decision, per `mochiko:patterns-model-tiering`'s class key; the platform-documentation
questions went to a `claude-code-guide` dispatch and were then **re-read cold by the lead at
the source** after both reviewers' external-claims re-read clause fired — F9 carries the
result. Reviewer reproduction: lens B reproduced every numeric repo claim it could run
exactly — 321 / 695 / 1,016 rules, 226 skill floors, 597 anchors, 214 allowlist rows, 181
suppressed edges, both checkers PASS; lens A verified 21 of 22 sampled claims. The three
misses are repaired in place below.)*

- **F1 — The crate.** `crates/mochiko-cli`: 599 lines including `Cargo.toml` (11) —
  `lib.rs` 119 · `schema.rs` 239 · `tests/render.rs` 224 · `main.rs` 6 — two dependencies
  (`serde`, `serde_norway`), three commits, last touched 2026-08-26 (v0.91.0 plan
  retirement). One command: `template <name>` with `--check` and `--schemas-dir`. Renders
  exactly the 8 template schemas in a closed `TEMPLATE_NAMES` set, each with a compile-time
  embedded copy; resolution order is `--schemas-dir` → `./plugins/mochiko/schemas/` →
  embedded. `cargo test` 12/12, `fmt`/`clippy -D warnings` clean in the worktree. CI
  (`.github/workflows/ci.yml`) is the repo's only workflow: test · fmt · clippy · `cargo
  audit --deny warnings` · secret grep; its path filter is `crates/**`,
  `plugins/mochiko/schemas/**`, `Cargo.*`, and the workflow file. No release workflow, no
  built artifacts, no distribution mechanism was ever chosen (open thread since 2026-08-16).
- **F2 — The schema corpus the crate does not see.** `plugins/mochiko/schemas/` holds 20 YAML
  files: the 8 rendered templates, 6 command content schemas (`implement.yaml` 1,019 lines /
  105 rules), `common.yaml` (9 blocks), `command-labels.yaml`, `skill-labels.yaml`,
  `skill-review-common.yaml` (6), `skill-authoring-common.yaml` (4), and the shelf data file
  `architecture-shelf-backend.yaml`. 30 of 38 skill directories carry an in-directory
  `schema.yaml`. Live rules: 321 command + 695 skill = **1,016**; floors 112 command by
  `grep -c 'class: floor'` (the checker reports 110 — the two-figure gap is instrument
  difference, stated) + 226 skill (grep and checker agree). `tombstones:` blocks exist in
  **3 of 6** command schemas (`brainstorm` · `setup` · `specify`; optional in the checker).
  The crate's only contact with the non-template files is a gate-5 test that parses
  `schemas/*.yaml` as generic YAML; it is blind to the 30 in-directory skill schemas.
- **F3 — Delivery today is an instructed Read, not a forced one.** Every command `.md` Rules
  block says: "Read `plugins/mochiko/schemas/<cmd>.yaml` raw, in full … The raw Read is the
  first-class read: no binary, no render step." Skills read `schema.yaml` base-dir-relative
  plus their family common file. Commands hardcode a **repo-relative** path; from an
  installed plugin cache that path does not exist in a user's project — the open M1 path
  probe on the command D10 watch. Observed directly in this session: the `analysis-iterative`
  skill loaded from `~/.claude/plugins/cache/mochiko/mochiko/0.99.0/…` while the repo ships
  0.103.0 — the cache path carries the version, and the installed copy lags by four bumps.
  The first-live-run watches for command (D10) and skill (D6) delivery both stand at n=0 with
  the probe "schema read? whole? before the first action?". Measured delivered-at-invoke
  figures already on record (skill-content-schema landing): review family ×3.24 (~119.9k
  chars) · authoring ×1.84 (~150.6k) · patterns ×1.90 (~95.9k) · dense five ×1.75 (~81.8k) —
  the baseline for any read-cost claim in this record.
- **F4 — The two-arm form is everywhere.** 32 occurrences across 24 files carry "invoke
  `mochiko-cli template <name>`; when the binary is absent, Read
  `plugins/mochiko/schemas/<name>.yaml` raw" — including the shared block
  `authoring-common.two-arm-template` and router rows. Router row 58
  (`plugins/mochiko/skills/mochiko/SKILL.md`) names `architecture-shelf-backend` as a CLI
  template; the binary rejects it (exit 2, reviewer-confirmed). `setup.yaml` is the only
  surface that references `${CLAUDE_PLUGIN_ROOT}` (its `plugin_root` var).
- **F5 — Integrity tooling is Python, advisory, maintainer-side.**
  `scripts/check-command-schema.py` (1,239 lines), `scripts/check-skill-schema.py` (1,094 —
  a deliberate sibling, never a fork: skill D5 forbids cross-grammar sharing),
  `scripts/find-similar-rules.py` (457) + `similar-rules-allowlist.yaml` (214 rows); all run
  under `uv run` with `pyyaml`. Both checkers PASS with 0 findings on the current tree; the
  detector reports 0 clusters over 1,016 rules with 181 allowlist-suppressed edges. The
  checks they carry — YAML parse, `kind:` discriminators, section-set grammar, ID
  uniqueness/format/prefix, label registry, `${var}` binding, deixis lint, tombstone
  integrity, sidecar anchor resolution, count-pin sync (`kind: fail` / `class: floor`),
  `.md` scaffold and token resolution, `when:`/`conditions:` resolution, `enforces:` reverse
  coverage, `extends:` resolution with local-`class:` assert — are all deterministic. GI-019
  places advisory exit-code scripts outside kernel-class (waiver GI-008, revisit trigger "a
  script becomes load-bearing in a shipped flow"). Eval harnesses: `evals/run.py` (skill
  compression, judged) and `evals/commands/run.py` (plan-only command eval, implement pilot,
  brainstorm probe done; its own recorded finding: "24 plans — noise-dominated per its own
  prereg guard").
- **F6 — Change management today is procedural.** 99 strip files under `.mochiko/strips/`
  (2.8 MB, one file per primitive — a converted primitive's file carries prose AND schema
  entries), the provenance sidecar `.mochiko/provenance.yaml` (597 anchors, `kind:
  primitive-provenance`, repo-side, never shipped — content-schema D16), `tombstones:` blocks,
  `DECISIONS.md` rows, and version stamps together carry GI-006 reconstructibility; the
  audit (author≠grader) and the advisory checker are the only enforcement. Every schema edit
  is a shipped-primitive landing (strip + audit) per `.claude/rules/mochiko/primitive-edits.md`.
  The comparable prior wave (v0.76.0) was priced at 83 strips across 40 files.
- **F7 — Governance text in play (verbatim key lines).** GI-020: "The plugin MUST install and
  function as a markdown-only plugin — no install-time build step, no binary dependency, no
  submodule-class fetch burden. Any admitted binary is strictly additive: the schema data
  files are Read raw as the first-class degraded path when the binary is absent (record D8).
  A distribution mechanism that makes plugin install heavier is a violation." Testability:
  "Pass: a fresh plugin install with no binary present is fully functional; schema data
  files Read raw. · Fail: install requires a build step, fetches a binary, or fails without
  the binary." GI-019 bright line: "never gates pipeline progress, never dispatches or
  sequences agents, never holds judgment that skills own"; testability Fail row: "an
  admitted binary that gates the pipeline, sequences agents, or holds skill-owned judgment";
  rationale: keeps admitted tooling "to delivery/composition roles". GI-012 gate 6 = `cargo
  test` PASS; gate 5 = schema-data/binary consistency. Amend path: `/mochiko:setup`
  (fact-profile changes and un-waives are governance events); amendment policy: MAJOR for
  "principle removal / incompatible redefinition".
- **F8 — Plugin manifest and install shape.** `plugin.json` declares `commands`, `skills`,
  and 10 `agents` — no hooks, no binaries, no dependencies, no `package.json`;
  `marketplace.json` source is the local path `./plugins/mochiko`; the repo's
  `.claude/settings.json` configures no hooks; the plugin tree is `agents · commands ·
  output-styles · schemas · skills · templates`. No `hooks/` directory exists anywhere in the
  plugin.
- **F9 — Platform facts** (initial `claude-code-guide` dispatch, then **lead cold re-read at
  code.claude.com/docs/en/{skills, plugins-reference, hooks} on 2026-09-03** — every item
  below is `verified` unless marked *unverified*; two items rest on the lead's re-read
  alone, beyond either reviewer's own fetch, and carry the marker *(lead-only)*:
  `WorktreeCreate`'s blocking status in F9.1 and `${CLAUDE_PLUGIN_DATA}`'s path and
  lifecycle detail in F9.2):
  - **F9.1 Hooks.** A plugin ships hooks via `hooks/hooks.json` or inline in `plugin.json`.
    The reference documents ~32 events; those that can **block** (exit 2 / JSON decision):
    `UserPromptSubmit` · `UserPromptExpansion` · `PreToolUse` · `PostToolBatch` ·
    `SubagentStop` · `TaskCreated` · `TaskCompleted` · `Stop` · `TeammateIdle` ·
    `ConfigChange` · `WorktreeCreate` *(lead-only)* · `PreModelSwitch`. Plain-text stdout is added as
    **context Claude can see** on `UserPromptSubmit`, `UserPromptExpansion`, `SessionStart`,
    and `PostModelSwitch`; `PreToolUse` injects via JSON `additionalContext`.
    **`UserPromptExpansion`:** fires "when a user-typed command expands into a prompt,
    before it reaches Claude"; matcher = **command name** ("your skill or command names");
    exit 2 "blocks the expansion"; stdout is context. `PreToolUse` matches on `tool_name`
    (examples `Bash`, `Edit|Write`, `mcp__.*`; the sibling record reads it as any tool
    name — `Skill` as a matcher is *unverified by example*, probed at wave 0). `Setup`
    exists for one-time preparation under `--init`/`--maintenance`. Hook input carries
    `session_id`, `transcript_path`, `cwd`, `permission_mode`, `hook_event_name`,
    `agent_id`, `agent_type`. **Hooks are fail-open:** a hook that cannot start or that
    reaches its `timeout` (default 600 s for `command`; 30 s on `UserPromptSubmit`) is
    cancelled and "renders no decision — for most hook events, the action proceeds".
  - **F9.2 Placeholders.** `${CLAUDE_PLUGIN_ROOT}` (the plugin's installation directory —
    "changes when the plugin updates … treat it as ephemeral and don't write state there")
    and **`${CLAUDE_PLUGIN_DATA}`** (`~/.claude/plugins/data/{id}/` — "survives plugin
    updates … use this to reference installed dependencies, generated files, or caches that
    must outlive an update"; deleted on uninstall — path and lifecycle *(lead-only)*) are
    substituted in plugin skill and
    agent content "anywhere the placeholder appears", in Bash rules of the `allowed-tools`
    frontmatter, and in hook commands. "Using the same variable in both places lets a skill
    run a bundled script without a permission prompt."
  - **F9.3 `!` preprocessing.** "The !`<command>` syntax runs shell commands before the skill
    content is sent to Claude. The command output replaces the placeholder" — custom
    commands and skills are one mechanism ("Custom commands have been merged into skills");
    works for plugin skills. Runs "the same way it runs Claude's own shell commands" in the
    session shell's current working directory, stderr merged into stdout, under the Bash
    tool's 2-minute timeout; **output past the Bash tool's inline ceiling "arrives as a
    file path plus a short preview, not truncated text"**. The docs' worked example pairs the
    syntax with an `allowed-tools` grant; whether the grant is *required* for `!` to run is
    *unverified* (wave-0 probe (a)). **Policy kill-switch:** `"disableSkillShellExecution":
    true` in settings replaces each command with the literal
    `[shell command execution disabled by policy]` for user, project, **plugin**, and
    additional-directory skills ("most useful in managed settings, where users cannot
    override it"); Cowork desktop sessions apply the same placeholder; synced skills never
    run `!` commands. **Windows:** the `shell` frontmatter key (`bash` default,
    `powershell`); "`shell: bash` when bash isn't available: the invocation fails before any
    command runs. This happens on Windows without Git Bash."
  - **F9.4 Binaries and install.** A plugin may ship executables in `bin/` — "added to the
    Bash tool's `PATH` and invokable as bare commands while the plugin is enabled"; not
    allowed in plugins distributed through claude.ai organization settings; executable-bit
    preservation on install *unverified* (moot under D4). **An install-time step exists:**
    "When Claude Code copies a plugin into the cache, it also installs the plugin's Node.js
    package dependencies there … at install, at update, and at session start when an
    enabled plugin isn't cached yet … only when the plugin's root directory contains both a
    `package.json` and a supported lockfile" (`bun install --frozen-lockfile
    --ignore-scripts` / `npm ci --ignore-scripts` — no lifecycle scripts). Whether that path
    can carry per-platform prebuilt binaries onto the Bash `PATH` is *unverified* (D4 road
    recorded, assessed at wave 0). Plugin `dependencies` declare other plugins with semver,
    never binaries; **no mechanism declares a required binary or minimum Claude Code
    version**.
  - **F9.5 Install and cache.** Installed plugins live under `~/.claude/plugins/cache/`
    (marketplace/plugin/version — F3); updates via `claude plugin update` plus auto-update on
    version change; version pinning *unverified*; marketplace sources: `github`, `url`,
    `git-subdir`, `file`, `directory`.
  - **F9.6 Agents.** Frontmatter keys: `name · description · model · tools ·
    disallowedTools · skills · permissionMode · maxTurns · memory · background · isolation ·
    initialPrompt · effort · color · experimental`. Plugin agents cannot declare `hooks`,
    `mcpServers`, or `permissionMode` (ignored when loaded from a plugin). Subagents with
    preloaded `skills:` receive the full skill content at startup (whether `!` runs at
    preload is *unverified* — wave-0 probe).
  - **F9.7 SessionStart / env.** A `SessionStart` hook can write to `CLAUDE_ENV_FILE` (a
    shell preamble sourced before every Bash tool call; the documented lever for the Bash
    output-ceiling variable if a render exceeds it). `CwdChanged` gives per-directory
    management.
  - **F9.8 Skill frontmatter** (documented keys, with `shell`, `paths`, `allowed-tools`,
    `disable-model-invocation`, `user-invocable`, `compatibility`, `metadata`); re-invoking a
    skill whose rendered content is identical adds a short "already loaded" note rather than
    a second copy.
- **F10 — Agents.** 10 personas, 76 KB total, all `model: opus`, frontmatter `name ·
  description · model · color · skills`; made prose-only at v0.63.0/v0.64.0; budgeted on
  `description:` only. No record, backlog item, or decision mentions agent schemas.
- **F11 — Open threads that a CLI move would touch.** Graduation candidates keyed on "a
  written checker assert" as consumer (per-kind fields · `at:` · `requires:` · phase anchors ·
  cross-family `skill-common.yaml`); the Desk FAIL-set widening pass (user-deferred); the D7
  compression-eval judged sample (owed); the M1 path probe. The `converting-skill-to-schema`
  converter skill ruled at skill D9 is **absent on disk** (four waves converted without it).

## Constraints in play

1. **GI-020** is contradicted outright by "no fallback files" plus a binary on the read
   path. It leaves only by a recorded governance amendment (`/mochiko:setup` amend run,
   MAJOR precedent), never by this record alone — and the amended text must be true of the
   tree on the day it is ratified (review A-C2 → D10.1's transition clause).
2. **GI-019 bright line** — a required binary whose absence stops a run, hard constraints at
   migration apply, and a hook that blocks on dependency absence each engage the line; each
   is argued at D11 against the ledger's own Fail-row wording, and this record is the fresh
   admission ruling for the widened role, routed through the amend run (review A-I1).
3. **`schema-based-template-guidance` D8** (raw Read fallback), **D1** (renderer only), and
   **D3** (eight-template scope, later ratchet) are superseded or discharged by ruling
   (D10.5).
4. **GI-006** reconstructibility must survive whatever replaces strips/sidecar/tombstones for
   schema content — every edit reconstructible from the log plus `DECISIONS.md` plus version
   stamps, and the derived views must stay human-readable text (the author≠grader read path).
5. **D16 posture** — maintainer metadata never ships with the plugin; the shipped log
   carries runtime content only, anchors stay repo-side (D2).
6. **`producer-plan-enforcement`** (accepted, sibling) owns plan-mode enforcement; the
   dependency-halt hooks at D7 never touch behavior.
7. **GI-008** — untouched: its six scripts are the skill-shipped helpers, none of which D6
   retires; the three repo-level checkers D6 retires were never waived (D10.7).

## Problem — why this session (ranked driver set, user-ruled at Q1)

The ask bundles five drivers; the user ranked them, striking none. `Confident` — a clear,
reasoned ranking with no hesitation.

| rank | driver | what it means for the design |
|---|---|---|
| **high** | **B — Change management.** IDs, tombstones, strips, sidecar anchors become migrations in a store; GI-006 reconstructibility by construction, not by audit. | The log's history model is a first-class design object; the schema strip ceremony is superseded. |
| **high** | **C — Integrity in one toolchain.** Label registry, ID uniqueness, count pins, `extends:` resolution, floor and fail survival enforced by Rust; the three Python advisory scripts retire. | The F5 check inventory is the crate's first backlog; "advisory" vs "rejecting" is argued at D11. |
| medium | **A — Delivery guarantee.** The schema reaches the model deterministically at fire (F3: instructed Read, n=0, repo-relative path). | `!` preprocessing plus dependency-halt hooks are the mechanism; no-fallback is the delivery posture. |
| medium | **D — Run control.** Hooks enforce beyond doctrine: binary present, rules delivered before the first seat. | Scoped to dependency presence; behavior enforcement stays with the sibling ruling. |
| low | **E — Foundation.** The Rust seed grows toward Tauri. | Context, not a driver: no design element is justified by E alone. |

**Roads rejected at the frame** *(recorded at Q13 — review A-C3 / B-I2, user-routed
"rule inline")*:

- **The null road** — keep raw-Read YAML, add nothing: rejected because drivers B and C
  (both `high`) are unserved by it — strips stay hand-copied, integrity stays advisory
  Python, the repo-relative path defect stays open.
- **The maintainer-side-only road** — the migration log, the Rust validator, and committed
  derived snapshots in today's shapes, with **every `.md` unchanged** and no binary on any
  user's machine: it serves B and C **fully** at zero governance cost (no GI-020 amendment,
  no GI-019 argument, no binary dependency, no Windows question, no contract-suite class, no
  36 re-points). It loses on driver A alone and on the user's explicit no-fallback
  instruction. **Honest driver attribution, stated the way D11 stated its concession:** the
  governance cost of this design — the MAJOR amendment, the dependency, the test class — is
  bought by driver A at `medium` rank on the user's explicit instruction, not by the two
  `high` drivers. The user ruled the road rejected with that attribution on record; a future
  reopen is one question.

## Decisions

### D1 — The migration log is the source of truth; the projection is an in-memory replay at fire, a persistent cache only on measured need — log `Confident` · store `Confident` (user-ruled at Q13) · efficacy `Assumed (n=0)`

**Statement:** truth lives in an ordered log of text migration files committed to git
(working shape: `migrations/NNNN-<slug>.yaml`), each one change set over the schema corpus —
mint, reword, tombstone, supersede-with-ruling-anchor, registry edit, section edit.
`mochiko-cli` validates every migration against the grammar and replays the log. **The
projection is built in memory at each invocation** — 1,016 rules replay in milliseconds in
Rust — and a persistent cache is added only when a measured render latency at `!` fire
demands it, living in `${CLAUDE_PLUGIN_DATA}` (never the ephemeral plugin cache, F9.2),
keyed by log hash. **SQLite is deferred**, not rejected: the shelf candidate is named
(`rusqlite`, bundled) and adopted the day a measured need names a query surface or a latency
bound an in-memory replay fails. The current state is rendered as **derived views** —
human-readable text, never hand-edited, regenerated by the CLI, CI asserting view ≡ replay
(the `ARCHITECTURE.md` derived-index precedent, KM "Index agreement"). A reword is therefore
a migration file, not an in-place edit; CLI authoring commands write the migration for the
editor. **Sequence allocation:** migration numbers are lead-assigned ranges per wave (a
seat never self-allocates), the file name carrying `NNNN-<slug>` with a content hash in the
header — collisions are a validator rejection.

**Rationale:** this is what "migrations" means (driver B); git stays legible — diff, blame,
PR review, and validator reads work on text; disjoint file ownership survives (a wave's seats
each add migration files; a single committed SQLite file would make every merge a binary
conflict, killing the transport-floor wave shape this repo runs on); and the strip ceremony
becomes redundant for schema content rather than reformatted — verbatim prior content is in
the log by construction (GI-006 by construction, driver B's core). Rejected: **SQLite
committed as truth** (binary blob; every read needs the CLI; worktree merges die) and
**current-state YAML as truth with a CLI-derived ledger** (rename/split detection becomes
inference; B gets a derived history, not an authored one). User ruled the recommended
option at Q2.

*(Amended at Q13 — review A-I4 / B-I1, user-ruled per `build-vs-off-the-shelf` D4, which
places storage engines and migration-bearing shapes above the retrofit-cost line, the
user's ruling, never the builder's.)* The session's earlier wording made SQLite "the
projection" on the strength of the ask's example ("file based db, like sqlite") without a
requirement paying for it: D1's rationale argued only for the log. The user chose **replay
in memory, cache on measured need**, the recommended option, over **SQLite projection in
`${CLAUDE_PLUGIN_DATA}` as asked** and **a single index file**; the criterion is render
latency at fire and the integrity query surface, both measured at the pilot (D9 wave 3).
Deliberate departure from the ask's wording, taken by the user.

### D2 — Scope: the full schema class, provenance folded in; prose stays markdown — scope `Confident` · anchor-constraint efficacy `Assumed (n=0)`

**Statement:** the log covers every schema-class file — the 6 command content schemas, the
30 in-directory skill schemas, `common.yaml`, the two label registries, the two family
common files, the 8 template schemas, and the shelf data file (50 files today) — plus the
provenance sidecar, folded in as migration grammar: a migration that supersedes or
tombstones a `class: floor` rule, a `kind: fail` rule, or any anchored rule MUST carry a
ruling anchor resolving to a `DECISIONS.md` row, or the CLI rejects it — **the schema-rule
limb of GI-005 becomes mechanical** (the prose-primitive limb and the dead-pointer scan stay
procedural). Strips for **schema content** end going forward: the log is the verbatim
record. The freeze is scoped to **schema-content entries**, never to files: the 99 strip
files stay live for prose entries on the same primitives, gaining a README note that
schema-content entries after the genesis stamp live in the log. A **genesis migration**
imports the v0.103.0 state as the baseline, carrying the sidecar's 597 anchors as anchor
fields from wave 1; `.mochiko/provenance.yaml` **stays authoritative for the Python
checkers until they retire** (they resolve anchors and must pass until their port), then
is **frozen as archive at wave 6** (moved to `.mochiko/archive/provenance-genesis.yaml`,
append-only) — verify-round repair, B-I7; history before genesis living in strips + git as
today. The D16 posture holds by build profile: the
shipped log carries runtime content only (anchors excluded), the maintainer replay carries
anchors. The `.md` bodies — command Identity & Mission and Adaptive Goal Protocol prose,
`SKILL.md` procedure and teaching prose — stay markdown, edited directly, still
strip-governed; `command-content-schema` D2's stage-2 absorption trigger stays unfired and
untouched.

**Rationale:** driver B wants the whole change surface under one history model, and the
sidecar is the one piece whose semantics (protected content leaves only by ruling) is
exactly a migration constraint — leaving it beside the store would keep GI-005 procedural.
Prose has no IDs, no constraints, and no cross-references; a store gains B and C nothing
from it, and absorbing it would pre-empt a benefit-keyed trigger the content-schema record
reserved to evidence. Rejected: **rule schemas only** (B stays half-procedural — every schema
edit still hand-copies a strip) and **full plus prose** (fires D2 stage 2 ahead of its
trigger for no B/C gain). User ruled the recommended option at Q3.

*(Amended at Q13 — reviews B-I7, B-I8, B-I14, B-I15, in the user-ruled batch: the
sidecar's disposition ruled (anchors carried from wave 1, the file frozen to archive at
wave 6); `kind: fail` survival added to the anchor-required set; the strip freeze scoped
to schema-content entries; the GI-005 claim scoped to the limb it mechanizes.)*

### D3 — Delivery binding: `!` preprocessing injects the CLI's output at fire; the halt keys on positive confirmation; absence halts — choice `Confident` · mechanism `Assumed` until wave 0

**Statement:** every command and skill `.md` binds its rules through the harness's
!`command` preprocessing — working shape: the "Rules — load the schema first" block
collapses to one line, !`mochiko-cli rules <command>` (skills: !`mochiko-cli rules
<skill>`), whose stdout the harness injects into the prompt before the model reads
anything. The render's **first line is a version triple** — binary version · log grammar
version · plugin version (D5) — and the count pins ("the N rules of `kind: fail`", "the N
rules of `class: floor`") are **computed and printed by the CLI**, never hand-pinned in the
`.md`. **The halt clause keys on positive confirmation, never on the absence of error:** the
`.md` requires the version-triple line in its exact shape and halts on anything else — an
error, an empty block, the policy placeholder `[shell command execution disabled by
policy]`, or a file-path-plus-preview stub from an oversized render — surfacing
"mochiko-cli rules not delivered: <what was seen>" and never proceeding, never Reading a
file instead. **Precedence across channels** *(verify-round repair, A-C1 fold; confirmed
by the user at acceptance)*: the confirmation is satisfied by the version-triple line
arriving through **either** channel — the `!` slot or a dependency-halt hook's injected
context (D7b carries the same first line); so a policy-replaced `!` slot with the hook
delivered proceeds on the hook's rules, and a policy-replaced slot with no hook delivery
halts. The `.md` says exactly that: "proceed only on the version-triple line, from
whichever channel delivered it". The converted `.md` carries a frontmatter `allowed-tools` grant for the
binary (working shape: `Bash(mochiko-cli *)`; under D4 the binary is on the user's `PATH`,
so the grant names the bare command, never a plugin-root path) — a scaffold key from wave 3.
The skill-side form is identical (F9.3: preprocessing works in `SKILL.md`). **Render size is
bounded:** the CLI's render is compact (resolved stubs, no comments, no tombstones, no
grammar block) and wave 0 measures the largest render against the Bash tool's inline
ceiling; if any render exceeds it, the ceiling variable is raised via the `SessionStart`
hook's `CLAUDE_ENV_FILE` (F9.7) before the pilot converts. If the `SKILL.md` probe fails,
the skill-side fallback binding is — *as amended at Q8, user-ruled (D7)* — first the
deterministic `PreToolUse` injection hook on `Skill`, and only last **instructed Bash**
("your first action: run `mochiko-cli rules <skill>`"); neither is ever a runtime fallback
to a file.

**Rationale:** driver A's gap is exactly "instructed, not forced" (F3, n=0 on both watches);
preprocessing is the one platform path that puts the rules in front of the model before it
acts (F9.3). Moving the pins into CLI output removes the desync class the pair form needed a
guard for — at the price, acknowledged, of the independent-number self-check today's `.md`
carries (review A-M2): the version triple confirms **delivery**, not completeness — the
CLI's printed counts assert completeness and D8's contract suite tests it; the independent
self-check is lost and booked as a loss, not a gain — and the deterministic hooks at D7
carry the halt the prose clause cannot. Rejected: **instructed
Bash as the primary** (keeps A where it is), **hook injection as the primary** (matches on
prompt text or tool use; `!` is earlier and simpler), and the **maintainer-side-only road**
(Problem section — rejected on driver A and the user's instruction). User ruled the
recommended option at Q4.

*(Amended at Q13 — reviews A-C1, B-C2, B-I11, A-M2, and the lead's cold re-read (inline
ceiling; fail-open hooks), user-ruled as K1: the halt clause re-keyed to positive
confirmation; the policy placeholder, the oversized-render stub, and the hook-disabled
environment named; the `allowed-tools` obligation added; the count-pin trade acknowledged.
The residual — a model that ignores the halt clause — is carried by D7's hooks and by
D8's contract suite, and stays `Assumed` until they run.)*

### D4 — Distribution: the user installs `mochiko-cli` as a standalone tool (cargo / brew); the plugin ships no binary — `Contested` *(reasons inferred, `Assumed`; re-affirmed at Q13 with the fourth road on record)*

**Statement:** the binary is not distributed by the plugin. Users install it as an ordinary
developer tool — working shape: `cargo install mochiko-cli` (crates.io) and a Homebrew tap
(`brew install humaninloop/tap/mochiko-cli`), `cargo binstall` for prebuilt artifacts as
builder's room — and the plugin depends on it being on `PATH`. Plugin install stays a plain
clone with no build step and no fetch; the plugin never degrades silently: absence halts at
first use (D3's positive-confirmation clause and D7's dependency-halt hooks), and the
`SessionStart` hook surfaces the install line earlier as advisory context. The repo holds
no executables, and the crate gains a release pipeline (tag → build → publish) as a build
item; `publish = false` is lifted at the wave-2 landing (D9). GI-020 rewrites per D10.1.

**Priced consequences, on record:** two-step onboarding (plugin + tool); version skew
between the plugin's data and a separately installed binary (D5); a crates.io name and a
tap to own; **Windows served by `cargo install` only — which compiles from source and
requires a Rust toolchain on the user's machine: an install-time build step relocated from
the plugin to the tool** (permitted by D10.1's wording, stated plainly here); Windows
supported **only with Git Bash present** (F9.3: `shell: bash` fails outright without it) —
PowerShell-only Windows is a **declared unsupported platform**; and **users who cannot
install developer tooling** (locked-down machines, no admin rights, no cargo, no brew) lose
mochiko entirely under no-fallback — total loss, not degraded service — accepted
eyes-open.

**Rationale:** the lead recommended committed prebuilt binaries (clone-only install that
"just works", CI-verified checksums); the user chose the standalone install. The user
stated no reason at Q5; the lead's inferred reasons, marked `Assumed` until the user
corrects them: no executables in a git history that bumps versions ~daily, no
committed-binary supply-chain surface (GI-002's risk line), and the CLI treated as a real
developer tool installed like one — consistent with D6/D11's foundation posture. Rejected:
**release download at `SessionStart`** (a hook failure is the silent-degradation class ruled
out) and **committed binaries** (the recommendation, declined). Deliberate user choice
against recommendation.

*(Amended at Q13 — reviews B-I10, A-I2, A-I3, A-M6, A-M8, B-M8, user-ruled as K4: D4
stands. **A fourth road was not on the Q5 menu** because F9.4 was wrong as first recorded:
Claude Code runs a lockfile-driven Node dependency install in the plugin cache at install,
update, and session start (F9.4), so an npm package carrying per-platform prebuilt binaries
is a documented install-time path — **recorded as seen; viability unverified** (whether
such a package lands its binary on the Bash `PATH` under `--ignore-scripts`), assessed at
wave 0 as a future option, never assumed either way. The live unknown D4 creates is whether
the `!` preprocessing shell sees `~/.cargo/bin` or Homebrew's directory on `PATH` — it is
the wave-0 numeric abort (D9). Windows, toolchain cost, and the access-loss class stated
above.)*

### D5 — Version contract: a grammar-versioned log, a binary that declares its supported range; out of range halts — `Confident` · efficacy `Assumed (n=0)`

**Statement:** the migration log carries a grammar version in its header (working shape:
`grammar: 1`); the binary declares the grammar range it supports; every `!` render prints
the plugin version, the log's grammar version, and the binary version on its first line —
the positive-confirmation line D3 keys on; a log outside the binary's range halts with the
exact install or upgrade command. Content-only plugin bumps never require a binary update;
a grammar break is a CLI MAJOR bump, with the log's grammar version stepping in the same
landing. Backward compatibility (a newer binary reading an older grammar) is the binary's
obligation across its declared range.

**Rationale:** D4 makes plugin data and binary independently versioned, so skew is
certain; the contract must be explicit and loud, never inferred. Lockstep would force a
reinstall on every one of this repo's near-daily content bumps for no integrity gain;
best-effort reading is silent partial delivery — the class the no-fallback posture rules
out. Rejected: **lockstep** and **best effort**. User ruled the recommended option at Q6.

### D6 — Integrity: hard constraints at migration apply, advisory reports beside them; the Python scripts retire after a matrix port; the audit unit named — `Confident` · efficacy `Assumed (n=0)`

**Statement:** `mochiko-cli` rejects a migration outright on any structural violation —
grammar parse · `kind:` discriminator and per-family section set · ID format and prefix ·
mint-once and tombstone integrity · label ∈ registry · every `${var}` bound · `extends:`
target resolves with `class:` local · every `when:` term and `enforces:` target resolves ·
a `class: floor` rule, a **`kind: fail` rule**, or any anchored rule superseded or
tombstoned without a ruling anchor (D2); the `.fail.*` ID segment ↔ `kind: fail`
correspondence holds both ways — so the log can never enter an invalid state and the replay
is valid by construction. The heuristic and coverage checks stay **advisory**, exit 0,
printed as reports: deixis lint · similarity clusters · `enforces:` reverse coverage ·
per-dimension condition coverage · unused vars and labels · budget figures. The three Python
scripts (`check-command-schema.py`, `check-skill-schema.py`, `find-similar-rules.py`) and
their tests retire once their negative-test matrices (134 + 86 + 48 probes) are ported as
Rust tests — the port is the retirement gate, never a parallel period. **The steady-state
audit unit (GI-004, unchanged in force):** a schema migration is graded by
`mochiko:validator` on **the migration file plus the regenerated derived-view diff** —
criteria: the change's intent stated in the migration header · the ruling anchor present
where D2 requires one · ID lifecycle right (reword keeps, split records parent, merge
tombstones) · floor and fail survival · register — the deterministic pre-pass being the
CLI's own apply result. **The derived views MUST stay human-readable text** — they are the
author≠grader read path and the GI-006 reconstruction surface (a constraint, never
builder's room). Consequence for the pair audit: `.claude/rules/mochiko/primitive-edits.md`
criteria re-key — the `.md` carries the one `!` line, the `allowed-tools` grant, and the
positive-confirmation halt clause; section enumeration and count pins leave the `.md` (the
CLI prints both), so scaffold criteria 2 and the **count limb** of 3 collapse to "the `!`
line, the grant, and the halt clause are present and name the right primitive" — criterion
3's **survival limb** (every `kind: fail` rule survives, IDs kept on reword) moves into the
validator's hard set above, never dropped.

**Rationale:** driver C wants the checks to bind, and every check in the hard set is a
structural fact about the store's own data. The advisory reports keep the GI-008 posture for
everything heuristic. Rejected: **everything advisory in Rust** (driver C forfeited) and
**everything hard, heuristics included** (false positives block edits). User ruled the
recommended option at Q7. The GI-019 argument lives at D11.

*(Amended at Q13 — reviews A-I5, B-I8, B-I11, in the user-ruled batch: audit unit and
criteria named; `kind: fail` survival added; views-stay-text made a constraint; the
`allowed-tools` key added to the scaffold.)*

### D7 — Hooks: presence at `SessionStart`; dependency-halt hooks on the plugin's own commands and skills, gating on absence only; an advisory maintainer hook; behavior gating declined — shape `Confident` · hook limbs `Assumed` until wave 0

**Statement:** the plugin ships `hooks/hooks.json`. **(a)** `SessionStart` runs
`mochiko-cli --version`, injects presence, version, and D5 range status as context, prints
the exact install line when the binary is missing, and reports a policy-disabled
environment when it detects one — loud before the first fire, blocking nothing. **(b)**
**Dependency-halt hooks:** `UserPromptExpansion` matched on the plugin's own command names
(`mochiko:*`) and `PreToolUse` matched on `Skill` for `mochiko:*` skills. Each runs the
presence and range check; when the binary is present and in range it injects the rendered
rules as context, its first line the same version triple (a second deterministic delivery
beside `!`: a policy-replaced `!` slot still delivers through the hook, and D3's
confirmation is satisfied from this channel); when the binary is **absent or out of range
it exits 2 with the install line** — the expansion or the skill invocation is blocked. **The gate is on dependency
absence only, never on behavior or judgment:** it fires only for mochiko's own primitives,
only when the dependency is missing, and it is the from-zero ruling
`producer-plan-enforcement` D1 said a gating hook needs — scoped here, argued at D11.
**(c)** Maintainer-side only, in the repo's `.claude/settings.json`, never shipped:
`PostToolUse` on Edit/Write under the migration paths runs the advisory check and prints its
report. **Hook floor:** every shipped hook carries a 5-second `timeout`; **when it runs, absence
blocks by exit 2 (by design); when it cannot run or times out, the platform is fail-open
(F9.1) and the action proceeds on D3's prose clause alone** — a presence check must never
be able to break a session; **hooks ship to every consuming project**, executing the plugin author's code at every session
start and every mochiko fire — the user ratified that knowingly at Q13. **Behavior-gating
hooks are declined:** a `PreToolUse` deny on `Agent` spawn until rules are rendered, or any
hook that judges a seat's work, would cross the bright line's judgment and sequencing
clauses and contradict the sibling ruling; if ever wanted it takes its own ruling from zero.

**Rationale:** with delivery riding preprocessing (D3) and a positive-confirmation halt,
hooks add exactly two things: earlier loudness, and a deterministic halt and delivery that
prose cannot guarantee — the platform offers both at the command-expansion and skill-use
moments (F9.1), fail-open, so they never make things worse than the prose path. Rejected:
**non-gating hooks only** (the Q8 ruling, superseded at Q13 on review B-C1: it was ruled
against an incomplete hook map — `UserPromptExpansion` was absent from F9.1), **behavior
gating** (bright line, sibling ruling), and **no hooks** (driver D unserved). User ruled
the recommended option at Q8 and the K1 amendment at Q13; D3's fallback order amended at
Q8 stands.

*(Amended at Q13 — reviews B-C1, A-I7, B-M4, B-M6, user-ruled as K1: the hook map restated
(F9.1), the dependency-halt hooks admitted, timeout and fail-open stated, the
ships-to-all-consumers fact ratified, the "Prose vs. gate allocation" backlog item related.)*

### D8 — Test regime: four layers, the plugin contract suite is the release gate; deterministic asserts gate, the behavioral one is measured — regime `Confident` · "never fails" claim `Assumed (n=0)` until the suite runs

**Statement:** four test layers, all green before any `plugin.json` bump (GI-012 gate 6
widened, D10.3). **Crate** (`cargo test`): grammar parser · migration validator carrying the
three ported negative matrices (134 + 86 + 48 probes) · replay determinism (the log replayed
twice yields an identical state hash) · render golden tests against the committed derived
views (view ≡ replay) · version-contract halts with their exact messages (D5) · exit codes.
**Genesis fidelity:** every rule of the 50 schema files at v0.103.0 round-trips byte-exact
(ID · text · class · kind · labels · when · extends · pointer · enforces · the tombstones of
the three schemas that carry them) through the genesis migration and replay — a frozen
fixture kept after the YAML sources retire. **Plugin contract** (the new class): a headless
`claude -p` run per command and per converted skill, the plugin installed fresh from the
marketplace (never the repo checkout), the binary from `cargo install --path`, under the
plan-only fence `evals/commands` already proved — **deterministic asserts, gating at N=1:**
the `!` line executed · the version-triple line present · **no schema file Read anywhere**
(scoped **per converted primitive** through waves 3–5 — a converted command invoking an
unconverted skill legitimately reads that skill's file — and run-wide from wave 6);
**behavioral metric, reported never gating:** the floor read-back stated — N=3 replicates
per primitive, the pass bar pre-registered at the pilot (working shape: 3/3 at wave 3,
stated before the run), reported per bump. **Variants:** absence (binary off `PATH` → the
run halts with the install line) · skew (old binary, new grammar → halt) · **policy
placeholder** (`disableSkillShellExecution` set, hooks enabled → the hook delivers the
rules and the run proceeds on the hook's version line, D3 precedence) · **placeholder +
hook-disabled** (both off → the halt fires on the placeholder; the prose halt is the only
guard, so the outcome is recorded, not asserted) · **oversized render** (the largest
render vs the inline ceiling). CI matrix macOS + Linux; Windows-with-Git-Bash when a runner is available.
**Gate and cost:** smoke subset per PR (one command, one skill, the absence variant — ~5
runs); full suite at `plugin.json` bumps — 36 primitives × 3 core variants ≈ **108 headless
runs**, priced at the pilot in dollars and wall-clock and recorded on the ledger; the
headless runs take an API key from CI secrets, never the repo (GI-003). The platform's own
`claude plugin eval` is a candidate substrate for the contract layer, verified at wave 0.
The noise finding on the plan-only eval's discriminating power (F5) is why the behavioral
assertion is a metric, not a gate.

**Rationale:** "the plugin doesn't fail" is a claim about the plugin path — install, `!`
execution, absence, skew, policy — which the crate alone can never exercise; the contract
suite is the only layer that tests what the user asked for, and the repo already owns the
headless substrate (brainstorm probe item 0: invocability · workdir provisioning · fence all
PASS). Rejected: **crate tests only** (the claim rests on the wrong layer) and **dogfood
watches as the gate** (n=0 until someone runs it). User ruled the recommended option at Q9.

*(Amended at Q13 — reviews A-I6, B-I9, A-C1's variant list, in the user-ruled batch:
deterministic/behavioral split with N and bar; per-primitive scoping of the no-Read assert;
policy, hook-disabled, and oversized-render variants; cost priced.)*

### D9 — Rollout: staged waves with derived views as the bridge, a numeric abort at wave 0, and pilot abort criteria after it — shape `Confident` · wave count and efficacy `Assumed (n=0)`

**Statement:** the conversion lands in waves, each with its own author≠grader audits,
`plugin.json` bump, and all four D8 layers green; rollback is user-reserved per wave, and
GI-006 holds throughout on the log plus the frozen strips. **Wave 0 — probes:** (a) whether
`!` preprocessing needs the `allowed-tools` grant to run; (b) **whether the `!`
preprocessing shell sees `~/.cargo/bin` and Homebrew's directory on `PATH`** — the
**numeric abort**: if a bare `mochiko-cli` is not resolvable from the preprocessing shell,
D3 and D4 both die and the design returns to the user before any build; (c) `Skill` as a
`PreToolUse` matcher and `UserPromptExpansion` matching on `mochiko:*` names; (d) whether
`!` runs at skill preload into subagents (F9.6); (e) the largest render vs the Bash inline
ceiling; (f) `claude plugin eval` as a substrate; (g) crates.io name and tap availability;
(h) the npm-package road's viability (D4, future option only). **Wave 1 — crate only:**
grammar, migration validator with the matrix port, in-memory replay, render, genesis
import, the release **pipeline machinery** (tag → build; no public publish yet), four test
layers; the derived views are regenerated in **today's exact file shapes**
(`plugins/mochiko/schemas/*.yaml`, `skills/*/schema.yaml`), so every unconverted `.md`
keeps reading them raw — no split brain, nothing user-facing changes; CI's path filter
gains the migration path. **Wave 2 — governance:** the `/mochiko:setup` amend run lands
D10 whole — GI-020's rewrite **with its transition clause**, the GI-019 argument (D11),
GI-012, GI-002, GI-008 — MAJOR bump, **before any `.md` points at the CLI**; the crate's
**first public publish to crates.io and the tap lands in this wave, after the amend run**,
never before it. **Wave 3 — pilot:** `brainstorm` (29 rules) re-pointed, `hooks/hooks.json`
shipped with (a) and (b), the contract suite's first run, the read-cost delta measured
against the F3 figures, the store-latency measurement (D1). **Pilot abort criteria
(observable, stated before the run):** the read-back metric below its pre-registered bar,
or the per-invoke read cost above the F3 baseline for the same primitive — a trip halts
waves 4 and 5 and returns to the user; after wave 2, reversal costs a second amend run plus
re-pointing every converted `.md`, and that cost is stated here so the user rules on it
eyes-open. **Wave 4:** the remaining five commands. **Wave 5:** skills by family in the
arc's own order (review · authoring · patterns · dense five). **Wave 6 — end state:** the
shipped snapshot files are **deleted from the plugin** (`plugins/mochiko/schemas/*.yaml`,
`skills/*/schema.yaml`), the derived views relocate repo-side outside `plugins/`
(`.mochiko/schema-views/`, human-readable, never shipped) — the plugin ships the migration
log only, and "no fallback files" is literally true; Python retirement; provenance sidecar
frozen (D2); the two-arm text migrated to CLI-only across its 32 sites (router row 58's
phantom template fixed in the same migration); converter skills superseded; doc landings.
**Ceremony scale, priced:** waves 3–6 touch 36 primitives, 32 two-arm sites, the
primitive-edit rules file, the budget ledger, and the ledger entries named in D10 — each
`.md` re-point a strip + audit landing, comparable to the v0.76.0 wave's 83 strips across 40
files (F6).

**Rationale:** the risk compounds — a binary dependency, a governance MAJOR, an unbuilt
test class, and 36 re-points — and one wave would land them untested together (the
v0.76.0 precedent was `Contested` at far lower stakes); views in today's shapes make
staging free of dual maintenance, which was the objection to staging last time. Rejected:
**one wave** and **pilot only, then a new session** (the record already carries the
design; a second session would re-derive it). User ruled the recommended option at Q10.

*(Amended at Q13 — reviews A-I2, A-I8, A-I10, B-C2, B-M5, A-M4, in the user-ruled batch:
wave 0 re-scoped against D4 with the `PATH` probe as the abort; crates.io publish moved
behind the amend run; pilot abort criteria minted; the wave-6 end state ruled (snapshots
leave the plugin, views repo-side); ceremony scale and the CI filter priced.)*

### D10 — Governance envelope and supersessions — adoption `Confident` · transition-clause validity `Assumed` until the wave-2 validator grades it *(proposed whole by the lead; user adopted as stated at Q11; amended at Q13 as K2, the GI-019 limb split out as D11)*

**Statement:**

1. **GI-020 is superseded by governance amendment**, never by this record: the
   `/mochiko:setup` amend run (wave 2) rewrites its enforcement to — *the plugin installs by
   a plain clone with no build step and no fetch; it depends on the separately installed
   `mochiko-cli` for every command and skill; absence or version skew halts loudly at first
   use and never degrades; the shipped plugin carries no schema file a run could read
   instead* — with an explicit **transition clause**: *from ratification at wave 2 until
   the wave-6 landing, primitives not yet re-pointed read the derived snapshot files
   shipped in the plugin; the clause expires when no schema file ships in the plugin,
   asserted by the contract suite's run-wide no-Read assert* (verify-round repair, A-C2
   fold: the clause must be in force on ratification day, when no primitive yet depends
   on the binary) — so the principle is true of the tree on the
   day it is ratified and the validator grades the clause, not a false statement. Its
   **Testability rows are rewritten in the same run**: Pass — a fresh plugin install plus
   the documented tool install renders every command and skill's rules; with the binary
   absent, every mochiko fire halts with the install line; Fail — any mochiko run proceeding
   without the version-triple line, or a schema file shipped in the plugin after the
   transition clause expires. **Environments that disable skill shell execution or hooks by
   policy are declared unsupported** in the same text. A MAJOR governance bump (a
   non-negotiable's meaning changes; v2.0.0 precedent). CLAUDE.md's "Plugin install stays
   additive" non-negotiable is reworded at the same landing.
2. **GI-019 — see D11**, split out at Q13 (review A-I1 / A-I9): the argument is folded into
   the wave-2 amend run as recorded argument, not attached by record entry; this record is
   the fresh admission ruling for the widened role.
3. **GI-012 gates widen:** gate 6 = `cargo test` PASS **plus the plugin contract suite's
   deterministic set green**; gate 5's "schema-data/binary consistency" becomes **derived
   view ≡ replay under the released binary range**.
4. **GI-002's risk line fires:** `cargo install` from crates.io is the crate's first public
   release; the shipped-executable / supply-chain revisit named there is discharged at
   wave 2 (signed tags, `cargo audit`, checksum-published artifacts as builder's room) —
   **and the discharge names the shipped hooks explicitly**: plugin-authored code executing
   on every consumer's machine at every session start and every mochiko fire, under the
   5-second fail-open floor (D7).
5. **Supersessions-by-ruling recorded at their waves:** `schema-based-template-guidance`
   D8 (raw-Read fallback), D1's renderer-only limb, and **D3's eight-template scope — its
   "later ratchet" fires here** (discharged); `command-content-schema` D1 ("no binary on the
   read path") and the D9/D13 crate-extension reservation (discharged);
   `command-schema-ontology` D10's reservation (discharged); `skill-content-schema` D8's
   GI-012 clause (skill schemas now inside the crate's set). **Crate code that changes** —
   the closed `TEMPLATE_NAMES` set, the embedded copies, `template <name>` re-based on the
   replay — is a wave-1 build item, not a supersession (code, not a ruling). Untouched:
   content-schema D2's stage-2 absorption trigger; D16's maintainer-metadata line (kept by
   build profile, D2); `producer-plan-enforcement` D1.
6. **Ceremony re-keys at wave 6:** `.claude/rules/mochiko/primitive-edits.md` — the schema
   strip ceremony is replaced by "a migration carrying its ruling anchor", graded per D6's
   audit unit; the pair criteria collapse per D6; the budget ledger's delivered-at-invoke
   quantity re-keys to body + rendered CLI output, measured at the pilot against the F3
   baseline; the first-live-run delivery watches (command D10, skill D6) are superseded by
   the contract suite — delivery becomes tested, not watched — with their
   benefit-observation limbs re-homed to the D9 waves; **the template-schema CLI's own
   first-live-run watch** (2026-08-16) is **superseded**: its M7 rollback trigger
   ("CLI-delivered guidance underperforming the `.md` baseline reverts the 8 supersessions
   and re-points skills back") re-keys to **revert-the-wave** (GI-006 on the log plus
   strips — the `.md` exemplar baseline is unrecoverable after wave 6, and the read-cost
   and read-back criteria of D9's pilot are the replacement falsifier); its D5 reopen
   condition (norms baked into the release, no per-project depth reads) is **carried
   unchanged** — nothing here reads per-project state.
7. **GI-008 is untouched** *(verify-round repair, B-I5 — the earlier "narrows" clause
   rested on a wrong census)*: the waiver's six are the skill-shipped helpers under
   `plugins/mochiko/skills/*/scripts/` (`detect-stack.sh` · `validate-requirements.py` ·
   `validate-user-stories.py` · `validate-openapi.py` · `validate-model.py` ·
   `check-artifacts.py`), and D6 retires none of them. The three checkers D6 retires live
   under repo-level `scripts/`, postdate the 2026-08-06 ratification (v0.92.0 · v0.99.0 ·
   v0.100.0), and carry no waiver row — they rest on GI-019's advisory-checker clause;
   their retirement into the crate's hard set is covered by D11's admission of that set,
   never by an un-waive. The amend run records this as a note, not a change.

**Rationale:** every prior schema session carried an explicit governance-envelope
decision (content-schema D9, ontology D10, skill D8); this one changes a non-negotiable
and must say so in one place, true of the tree at every wave. Proposed whole, `Confident`
on the mechanism; adoption was the user's word at Q11 and again, as amended, at Q13.

*(Amended at Q13 — reviews A-C2, A-I1, A-I9, B-I3, B-I4, B-I5, B-I6, A-I7, A-M3, B-M10,
user-ruled as K2 and in the batch: transition clause and Testability rewrite; GI-019 split
out; the template-schema watch and M7 dispositioned; D3's ratchet discharged; GI-008
narrowed; hooks named in the GI-002 discharge; crate code items reclassified.)*

### D11 — GI-019: the widened kernel-class admission, argued against the ledger's own words and routed through the amend run — `Assumed` until the wave-2 validator passes it *(split out of D10.2 at Q13, review A-I1 / A-I9)*

**Statement:** this record is the **fresh admission ruling** for `mochiko-cli`'s widened
role — delivery of every command and skill's rules, hard constraints on the store's own
data, and dependency-halt hooks — under GI-019, which admits kernel-class tooling only per
instance and by recorded ruling; the 2026-08-16 admission was scoped to template delivery
and does not extend on its own. Three clauses are folded into the wave-2 amend run as
recorded argument against the ledger's Fail row ("an admitted binary that gates the
pipeline, sequences agents, or holds skill-owned judgment") and its rationale ("keeps
admitted tooling to delivery/composition roles"): **(i)** a required binary whose absence
halts a run, and a hook that blocks the plugin's own commands only when that binary is
absent, are **infrastructure dependencies in the delivery role the line licenses** — they
gate nothing by their output: present and in range, the binary renders and the hook
delivers; absent, nothing can be delivered and the halt is the honest report of that fact,
never a verdict on the run's work; **(ii)** hard constraints at migration apply are
maintainer-time definition of the store's own data — and the landing ritual they sit in
(`primitive-edits.md`, GI-012's bump gates) is a pipeline in this repo's sense, so the carve
is **ruled, not assumed**: a rejection there is a structural-validity check on data the
tool owns (a compiler on its own language), never a grade of a primitive's judgment content,
which the author≠grader audit keeps; **(iii)** the bright line's judgment and sequencing
clauses are untouched — the CLI grades no artifact and dispatches nothing, and every
behavior-gating hook is declined (D7). The bright line's text is unchanged; the ledger entry
gains these clauses and names this record as the admitting ruling.

**Rationale:** the review found the GI-019 limb asserted rather than argued, routed by
record entry while the narrower GI-020 change was routed MAJOR — an asymmetry with no
defense. Routing both through the same amend run, against the ledger's wording, is the
consistent standard. Marked `Assumed` because the argument has no independent test until
`validation-constitution` grades the amended ledger at wave 2; the user ruled the split and
the routing at Q13.

## Build surface

*(lead-drafted from the rulings; build-time detail stays builder's room; wave placement per D9)*

- **Wave 0:** probe report answering D9's (a)–(h); the numeric abort decision recorded; the
  npm-package road assessed as a future option.
- **Wave 1 — crate:** migration grammar (`grammar: 1` header; change kinds; lead-assigned
  sequence ranges with header content hashes) · validator carrying the D6 hard set and the
  three ported negative matrices · in-memory replay (cache in `${CLAUDE_PLUGIN_DATA}` only
  on measured need — D1) · `mochiko-cli rules <command|skill>` render (version triple first;
  resolved `${var}` and `extends:`; sections as headings; `kind`/`class` shown; `when:` with
  its `conditions:` block; tombstones omitted; count pins printed; compact) · `template
  <name>` and `--check` re-based on the replay · derived-view regeneration in today's file
  shapes with the CI view ≡ replay assert · genesis migration importing v0.103.0 with the
  fidelity fixture · release pipeline machinery (tag → build, publish gated to wave 2) ·
  the four D8 test layers, the contract suite runnable on a fresh marketplace install · CI
  path filter gains `migrations/**`.
- **Wave 2 — governance:** `/mochiko:setup` amend run per D10 and D11 (GI-020 rewrite with
  transition clause and Testability rows · GI-019 clauses · GI-012 widening · GI-002
  discharge naming hooks · GI-008 note (untouched — D10.7) · unsupported environments
  named) · CLAUDE.md
  non-negotiable reworded · MAJOR bump · first public publish (crates.io + tap),
  `publish = false` lifted.
- **Wave 3 — pilot:** `brainstorm.md` re-pointed (`!` line + `allowed-tools` grant +
  positive-confirmation halt clause) · `hooks/hooks.json` with the `SessionStart` hook and
  the two dependency-halt hooks · contract suite first run with its pre-registered read-back
  bar · read-cost delta and store latency measured · pilot abort criteria evaluated.
- **Waves 4–5:** the five remaining commands; skills by family (review · authoring ·
  patterns · dense five); the `PreToolUse` `Skill` injection path serves as the skill
  fallback binding if the `SKILL.md` probe failed.
- **Wave 6:** shipped snapshot files deleted from the plugin, derived views relocated to
  `.mochiko/schema-views/` · Python scripts + tests deleted with their matrices proven
  ported · strips README note for schema-content entries · provenance sidecar frozen to
  archive · the two-arm text migrated to CLI-only across its 32 sites, router row 58 fixed ·
  `converting-command-to-schema` superseded (the absent `converting-skill-to-schema` closed
  as moot) · `primitive-edits.md`, the budget ledger, and the three delivery watches
  re-keyed per D10.6 · CHANGELOG, marketplace sync, DECISIONS rows, BACKLOG trail moves,
  ROADMAP touch at each wave.

## Evidence honesty

- **n=0 everywhere that matters:** no `!` preprocessing has fired in this plugin; no
  contract suite exists; no migration log exists; the wave-0 probes are unrun. Every
  decision heading now carries its own split mark — choice vs mechanism/efficacy — so no
  bare `Confident` stands on an untested mechanism (review B-I13).
- **D4's rationale is inferred**, not stated by the user (marked `Assumed` inside a
  `Contested` decision); the user may correct it at any point before acceptance.
- **The design's governance cost is bought by a `medium` driver** on the user's explicit
  instruction (Problem section, roads rejected) — stated, not dressed as necessity.
- The staged path was declined at the v0.76.0 build (D10 `Contested` there); this record
  chooses it at higher stakes with the view bridge removing that session's dual-maintenance
  objection — a changed premise, stated.
- **External claims:** every F9 item is `verified` by the lead's cold re-read at the named
  pages on 2026-09-03 unless marked *unverified*; the platform changes by version
  (memory note: "re-verify platform facts before any new ruling"), so wave 0 re-runs the
  load-bearing ones.

## Open threads

- **Windows:** in scope only with Git Bash present; PowerShell-only declared unsupported;
  `shell:` stays default `bash` everywhere. Revisit if a PowerShell render path is ever
  worth a second syntax and a second test axis.
- **Read-cost delta:** rendered output should be smaller than raw YAML (no grammar block,
  no comments, no tombstones, resolved stubs) — measured at the pilot against the F3
  baseline figures; the budget ledger re-keys on the measured figure (D10.6).
- **Store latency:** the in-memory replay's render time at fire is measured at the pilot;
  a persistent cache (and SQLite behind it) enters only on a measured bound (D1).
- **Agents:** out of scope by evidence (F10 — no obligation-shaped schema exists, no
  ruling asked for one); a future agent-schema session would ride this store.
- **Router skill `mochiko`:** stays prose (skill D1); its template rows carry the CLI-only
  form after wave 6.
- **`claude plugin eval`:** if it fits, the contract layer rides it instead of a bespoke
  runner — wave-0 finding.
- **D2 stage-2 absorption** (`command-content-schema`): untouched; its trigger now reads
  contract-suite evidence instead of a live-run watch (D10.6).
- **npm-package distribution road:** seen, viability unverified; assessed at wave 0 as a
  future option; D4 reopens only by the user's ruling.
- **Kinako reference:** the kinako product runs the installed plugin; the two-step install
  (D4) reaches it at wave 3 — its `SessionStart` context is the first external signal.
- **Prose vs. gate allocation (BACKLOG 2026-06-27):** partially discharged by D7
  (dependency presence earns a hook; behavior never does); the item stays open for the
  behavior half.
- **GI-008's "script count grows" trigger:** the three repo-level checkers (v0.92.0 ·
  v0.99.0 · v0.100.0) grew the count past the waived six without a recorded disposition;
  moot at wave 6 when they retire — noted for the amend run's record, no action owed here.

## Session trail

- **Q1 (drivers):** rank A–E, strike any that are not real. **A:** B and C high · A and D
  medium · E low; none struck → Problem section, ranked driver set.
- **Q2 (source of truth):** SQLite-committed · migration log + projection (recommended) ·
  current-state YAML + derived ledger. **A:** as recommended → D1.
- **Q3 (store scope):** rule schemas only · full schema class + sidecar (recommended) · full
  plus prose. **A:** as recommended → D2.
- **Q4 (delivery binding):** `!` preprocessing (recommended) · instructed Bash · hook
  injection. **A:** as recommended → D3.
- **Q5 (distribution):** committed prebuilt binaries in `bin/` (recommended) · release
  download at `SessionStart` · user-installed via cargo/brew. **A:** user-installed → D4,
  `Contested`; reasons inferred, `Assumed`.
- **Q6 (version skew):** grammar-versioned log + binary range (recommended) · lockstep ·
  best effort. **A:** as recommended → D5.
- **Q7 (integrity enforcement):** hard at apply + advisory reports, Python retires
  (recommended) · everything advisory · everything hard. **A:** as recommended → D6.
- **Q8 (hooks):** non-gating only + D3 fallback amendment (recommended) · one gating hook ·
  no hooks. **A:** as recommended → D7 (later amended at Q13); D3 amended.
- **Q9 (test regime):** four layers + contract suite as gate (recommended) · crate only ·
  dogfood watches. **A:** as recommended → D8.
- **Q10 (rollout):** staged waves + snapshot bridge + wave-0 abort (recommended) · one
  wave · pilot only. **A:** as recommended → D9.
- **Q11 (envelope):** adopt D10 as stated · amend a clause. **A:** adopt as stated → D10
  `Confident`.
- **Q12 (review sizing):** pair lens-split (recommended) · solo · waiver. **A:** pair →
  two cold reviewers, blind-map two-message dispatch, maps built independently; lens A =
  decision quality, lens B = record integrity; the record frozen at this line.
- **Q13 (disposition gate):** four Critical clusters K1–K4 each with a recommended
  disposition · the store-engine question (SQLite projection · replay + cache on need,
  recommended · index file) · the Important/Minor batch as the reviewers proposed. **A:**
  "as recommended" across all → K1 amends D3/D7 (positive confirmation, dependency-halt
  hooks, wave-6 end state, unsupported environments) · K2 amends D10/D9 (transition
  clause, Testability rows, GI-019 split to D11, GI-008, D3 ratchet) · K3 ruled inline
  (roads rejected at the frame, honest driver attribution) · K4 D4 stands with the npm road
  recorded, the `PATH` probe as abort, Windows and access loss stated · store = replay,
  cache on need (D1 amended) · batch folded per decision.

## Review + disposition trail

**Sizing (Q12, user-ruled):** pair, lens-split — lens A decision quality, lens B record
integrity — each a `mochiko:devils-advocate` seat running `mochiko:review-brainstorm`,
spawned by the blind-map two-message protocol (message one: topic statement + goal line
only, never the record path; both maps returned before message two opened the cold read);
maps built independently (lens A 40 angles, lens B 52); the index fence held whole (neither
reviewer opened `.mochiko/brainstorms/index.md`; the record's index entry was quoted to
lens B verbatim); the sibling record on `main` was read read-only by lens B to verify the
relation claims. Transport floor: message lane fired (legs 3, 4, 6, 7 held — mesh hold,
content-pinned orders, no resends, fan-in confirmed 2 maps + 2 reports); topology lane
untriggered (each reviewer the single writer of its own report file; the record frozen for
both reads).

**Reports (each the single-writer file of its seat):** `review-lens-a.md` — 30 raised, 21
survived (3 Critical · 10 Important · 8 Minor), 9 killed, status `critical-gaps`, 21 of 22
sampled repo claims verified exact; `review-lens-b.md` — 39 raised, 27 survived (2 Critical
· 15 Important · 10 Minor), 12 killed, status `critical-gaps`, every reproducible numeric
claim reproduced exact. **External-claims re-read clause:** both reviewers flagged their
doc-based findings (A: C1, I3, M6 · B: C1, I10, I11, I12, M9) as owed a cold re-read; the
lead re-read `code.claude.com/docs/en/skills`, `plugins-reference`, and `hooks` at the
source on 2026-09-03 — **all confirmed**, plus two facts neither lens carried (hooks are
fail-open; oversized `!` output arrives as a file path plus preview), folded into F9 and
K1.

**Cross-set merge (lead's):** 48 survivors → four Critical clusters, one user-ruled
question, one batch. Overlaps merged by name: A-C1 + B-C1 + B-C2 → K1 · A-C2 + A-I1 + A-I9
+ B-I4 + B-I5 + B-I6 → K2 · A-C3 + B-I2 → K3 (coverage survivor, routed per
`brainstorm.coverage-survivor-routing`: explore now / rule inline / defer — user chose rule
inline) · B-I10 + A-I2 + A-I3 + A-M6 + A-M8 + B-M8 → K4 · A-I4 + B-I1 → the store question ·
A-M1 = B-M2 (tombstones) · A-M3 = B-I5 (GI-008).

| finding | target | disposition (user-ruled at Q13) |
|---|---|---|
| A-C1 · B-C1 · B-C2 (K1) | D3, D7, D9 wave 6 | positive-confirmation halt; dependency-halt hooks (`UserPromptExpansion` on `mochiko:*`, `PreToolUse` on `Skill`); snapshots leave the plugin at wave 6, views repo-side; policy-disabled environments declared unsupported; ceiling probe; variants added to D8 |
| A-C2 · A-I1 · A-I9 · B-I4 · B-I5 · B-I6 (K2) | D10, D9, D11 | transition clause + Testability rewrite; GI-019 folded into the amend run as D11 (`Assumed`); admission ruling stated; landing-ritual carve ruled; GI-008 **untouched** (the first fold "narrowed" it on a wrong census — repaired at the verify round, B-I5: its six are skill-shipped, the three retiring checkers were never waived); D3 ratchet discharged |
| A-C3 · B-I2 (K3, coverage) | Problem | ruled inline: null road + maintainer-side-only road recorded rejected; governance cost attributed to driver A at `medium` on the user's instruction |
| B-I10 · A-I2 · A-I3 · A-M6 · A-M8 · B-M8 (K4) | D4, D9, F9 | D4 stands; npm road recorded (viability unverified, wave-0 assessment); `PATH` probe = numeric abort; Windows in only with Git Bash; toolchain cost and access-loss class stated; `${CLAUDE_PLUGIN_DATA}` named |
| A-I4 · B-I1 (store) | D1 | user-ruled: replay in memory, cache on measured need, SQLite deferred with the shelf candidate named |
| A-I5 | D6 | audit unit + criteria named; views must stay readable text |
| A-I6 · B-I9 | D8 | deterministic gate at N=1; behavioral metric N=3 with a bar; per-primitive scoping; cost priced |
| A-I7 · B-M4 · B-M6 | D7, D10.4 | 5-s timeout, fail-open, GI-002 names hooks, ships-to-all ratified; hook map restated; backlog item related |
| A-I8 | D9 | crates.io publish moved behind the wave-2 amend run |
| A-I10 | D9 | pilot abort criteria minted; post-wave-2 reversal cost stated |
| B-I3 | D10.6 | template-schema watch superseded; M7 re-keyed to revert-the-wave; D5 reopen carried |
| B-I7 · B-I8 · B-I14 · B-I15 | D2, D6 | sidecar's anchors carried by the genesis migration from wave 1, the file authoritative for the Python checkers until their port and frozen to archive at **wave 6** (the first fold said "at genesis" — repaired at the verify round, B-I7); `kind: fail` in the anchor set and hard set; freeze scoped to entries; GI-005 claim scoped |
| B-I11 | D3, D6 | `allowed-tools` grant obligation; `${CLAUDE_PLUGIN_ROOT}` closed as documented; wave 0 re-scoped |
| B-I12 | D1 | cache home `${CLAUDE_PLUGIN_DATA}`, never the ephemeral plugin cache |
| B-I13 | all headings | marks split per heading |
| A-M1 · B-M2 · B-M1 · B-M3 | F1, F2 | facts repaired: 599 incl. `Cargo.toml`; tombstones 3 of 6; floors 112 grep / 110 checker, instruments stated |
| A-M2 | D3 | count-pin redundancy loss acknowledged; version triple the replacement |
| A-M3 | D10.7 | **withdrawn by its reviewer at the verify round** (same census error as B-I5's first fold); residual carried by D11 limb (ii) |
| A-M4 · A-M5 · B-M5 | D9, D1 | CI filter item; sequence allocation ruled; ceremony scale priced |
| A-M7 | F9.4 | executable-bit claim marked unverified (moot) |
| B-M7 | F3, Open threads | measured read-cost figures cited as the baseline |
| B-M9 | F9.1 | `Setup` event noted; `SessionStart` remains the earliest per-session point |
| B-M10 | D10.5 | crate code changes reclassified as build items |

**Verify round 1 (bounded, per house practice) — NOT CLEAN, all repaired same round.**
Each reviewer delta-checked only its own findings' folds; no fresh cold read, no new blind
map. Lens A: 16 of 21 folds clean, 3 blocking + 2 nits — (1) three surfaces disagreed on
the policy-disabled run → **D3 precedence clause** (either channel's version line
satisfies the confirmation), D7(b) and D8's variants aligned; (2) the GI-020 transition
clause opened one wave late → opens at ratification; (3) the GI-008 fold rested on a wrong
census — **A-M3 withdrawn by its reviewer**, D10.7 repaired (see lens B); nits: D7's
fail-open wording vs blocking-by-design, and "replacement" overstating the version triple
(delivery, not completeness) — both reworded. Lens B: 24 of 27 folds clean, 3 blocking +
1 nit — (1) GI-008 census (same defect, repaired: the six waived scripts are skill-shipped
and untouched; the three retiring checkers were never waived); (2) two headings still bare
`Confident` → D2 and D10 split; (3) the sidecar frozen "at genesis" vs wave 6 → wave 6,
authoritative for the Python checkers until their port; nit: two F9 facts rest on the
lead's re-read alone — attributed. Both reviewers confirmed the external re-read folded
faithfully and that no fold reversed a disposition. **Delta-check round 2 (repairs only)
— all round-1 defects discharged; three nits, lead-repaired, round closed.** Lens A: five
of five discharged; one nit — the build surface's wave-2 line still said "GI-008
narrowing" → "GI-008 note (untouched — D10.7)". Lens B: repairs 1, 2 clean, status
agreement confirmed; two nits — D2's own Q13 note still said "at genesis" → "at wave 6";
F9's preamble now names the two items resting on the lead's re-read alone
(`WorktreeCreate` blocking; `${CLAUDE_PLUGIN_DATA}` path and lifecycle), each marked
*(lead-only)* inline. No round 3: the residuals were wording, repaired in place, and the
bounded idiom holds. **The one lead-repaired clause that reads as an amendment to a
user-ruled decision — D3's channel precedence — was put to the user at acceptance and
confirmed (2026-09-03).**

## Post-acceptance amendments — wave 0 probes (2026-09-03, user-ruled "adopt both")

Wave 0 ran the same day as acceptance; full evidence in
[`wave0-probe-report.md`](wave0-probe-report.md) (16 headless runs, ≈ $0.60, Claude Code
2.1.258). **Numeric abort NOT tripped:** the `!` preprocessing shell sees `~/.cargo/bin`
and Homebrew on `PATH`; D3 and D4 stand. Probe outcomes fold into the ground facts as
**F12**: (a) `!` needs an `allowed-tools` grant — denied otherwise, and a denied line stops
a command before the model runs, fails a subagent spawn at preload, and errors a Skill call
(fail-closed); each sub-command and each `$( )` substitution is matched separately; bare
names match; (b) `${CLAUDE_PLUGIN_ROOT}` substitutes in command bodies, skill bodies,
`allowed-tools`, and hook commands; (c) `Skill` is a live `PreToolUse` matcher whose
`additionalContext` reaches the model and whose `deny` errors the call;
`UserPromptExpansion` matches the namespaced `command_name`, its stdout reaches the model,
and exit 2 blocks the expansion before any model turn; `SessionStart` stdout reaches the
model; (d) `!` runs at subagent skill preload; (e) **the Bash inline ceiling is ≈ 30,000
characters** — 25k arrives whole, 35k and 60k arrive as a file-path notice whose preview
keeps only the first line; (f) `claude plugin eval` exists but is early-access, org-gated,
undocumented publicly; (g) `mochiko-cli` and `mochiko` free on crates.io, `mochiko-cli` free
on npm, `humaninloop-dev/homebrew-tap` already exists; (h) the npm road stays plausible and
unexercised. Extra: a `commands` directory-string manifest registered the probe plugin's
command files as one skill under `--plugin-dir` while the identical form works for mochiko's
plugin (array form and default scan both work; cause not isolated; the wave-3 pilot
re-verifies on the real plugin).

### D3 — amended (probe (e)): render chunking; head-and-tail confirmation — `Confident`

**Statement:** the rules render is delivered **one `!` line per schema section** (six per
command, six per skill — the family section set), each line its own preprocessing command
under its own ceiling; the CLI gains `rules <primitive> --section <id>`; a whole-primitive
render is never a single line. Every section render opens with the version-triple line and
**closes with an end line** (`mochiko-cli rules end · <primitive> · <section> · <N> rules`);
the `.md`'s halt clause requires **both** lines for every section — either missing, on any
section, halts. **Rationale:** the ceiling is a platform fact (F12e) and the oversized
preview keeps the head line, so a head-only confirmation would pass a truncated render;
chunking keeps every render far under the ceiling as the corpus grows; the tail line makes
truncation visible. Rejected: forcing every whole-primitive render under 30k (`implement`
cannot promise it as it grows). User ruled "adopt both".

### D8 — amended: the contract suite runs in the Docker AI sandbox via `evals/run.py`; GitHub CI keeps the crate layers — `Confident`

**Statement:** the plugin contract suite's substrate is the existing sandbox mode of
`evals/run.py` — sessions inside the Docker AI sandbox `claude-mochiko` (`sbx exec`, cwd
`/tmp/eval-*`, `--setting-sources ""`, the plugin staged with `--plugin-dir`, stored
subscription auth, no API key); `mochiko-cli` is placed on the sandbox `PATH` the D4 way
(`sbx cp` of the built binary into `~/.cargo/bin` before the crates.io publish, `cargo
install` inside the sandbox after it). The sandbox is Linux; the host is macOS — together
they are the two OS rows, no CI matrix. **Gate split:** the full contract suite is a
maintainer-side gate at `plugin.json` bumps (sandbox, no spend); GitHub CI keeps the four
crate layers (`cargo test` · fmt · clippy · audit) and no headless runs — the "API key in
CI secrets" clause of the original D8 is withdrawn. **Caveats on record:** the sandbox must
be authenticated (`sbx login`, the user's action — it was not at wave 0); the sandbox is
local-only; and the kinako record marks sandbox subscription auth a `Contested` ruling
sustained against adverse Terms-of-Service evidence — automated headless use of a consumer
subscription may sit outside what it permits; the user adopted with that on record.
**Rationale:** the suite wants a fresh user machine — its own plugins, its own `PATH`, a
real install step — and the sandbox is that machine; the runner already solved isolation
and auth (probe-settled 2026-08-22). Rejected: host-only headless runs (no install
isolation) and a GitHub-CI headless matrix (needs metered spend and secrets for a gate the
sandbox runs free). User ruled "adopt both".

## Wave 1 — built and accepted (2026-09-03 → 2026-09-04)

**Floor:** `floor: tripped · seats: P1 / P2 / P3 (staff-engineer, sequential single
pen-holder) / V1 / V2 / V3 (validator, fresh, author ≠ grader)`. Plan: `wave1-plan.md`;
reports and audits under `wave1-reports/`. User accepted wave 1 on 2026-09-04 and ruled the
family-2 check gap **closed in this wave** as extension unit **1b** (below).

| unit | commits | what landed | review |
|---|---|---|---|
| P1 core | `f34d48e` → `cd5a333` | typed model (lossless round trip over all 50 files), migration grammar (15 ops, canonical hash over id/sequence/anchor/changes, `grammar: 1` range with the D5 halt message), replay + `content_hash`, D6 hard set (34 rejecting codes, 6 advisory), README | V1 FAIL (3 blocking: protection strippable by field-clearing before a tombstone · optional hash · `load` skipping the hard set; 12 advisory) → fix round → PASS, every fix red-when-reverted |
| P2 surface | `3792104` → `07a39b4` → `9b43e83` | `clap` CLI, `rules <primitive> --section <id>` with the version-triple head and end-line tail, `preamble` with pins and section list, `template`/`--check` re-based on the replay (8+8 outputs byte-identical to captured fixtures, footer `schemas: replayed from <log-dir>`), `migrate validate|status`, `--plugin-root`/`--log-dir`, exit codes 0/1/2/3 | V2 PASS (3 advisory → fixed → delta PASS; 2 test-hygiene → fixed → delta PASS) |
| P3 corpus | `a1e4275` → `e5f723d` | `migrations/0001-genesis.yaml` (598,626 B, 50 import ops, 597 anchors folded, two `enforces: []` reasons lifted to `note:`, regenerates byte-identically, hash rejects a one-character tamper), views (semantic equality over all 50, emitted only under `--out`), `similar.rs` (CPython difflib Ratcliff/Obershelp with autojunk — 0 mismatches over 18,577 corpus pairs; live figures 1,016 / 146,572 / 0 / 181 reproduced), the three matrices with name-level ledgers (134 / 114 / 48; 81 probes named as exercising checks outside the wave-plan hard set), `release.yml` (publish job present, disabled), `ci.yml` filter + opt-in sweep step, `evals/contract/` (imports `evals/run.py`; absence + skew cases; `SKIPPED` exit 3 on an unauthenticated sandbox, never a false pass) | V3 FAIL (1 blocking: the detector matrix had no name-level ledger — real split 42/6; 9 advisory incl. a cwd-dependent allowlist lookup turning 0 clusters into 76 silently) → fix round → PASS |

**Gates at acceptance:** `cargo test --all` 246 / 0 (35 s; full similarity sweep opt-in
behind `MOCHIKO_FULL_SIMILAR=1`, 69 s) · fmt · clippy `-D warnings` · `cargo audit --deny
warnings` clean · no shipped file under `plugins/` changed byte-wise · no `plugin.json`
bump · `migrate validate --log-dir migrations`: 0 rejecting · 92 advisory. **After unit
1b (wave close):** 300 / 0 tests · 0 rejecting · 105 advisory · 87 pointers checked.

**Measurements the rulings keyed on:** cold section render **35 ms** release / 107 ms
debug, process start included — six lines per fire ≈ 0.2 s; **no demand for the deferred
cache (D1 stands)** · largest shipped section render **15,450 chars** (`implement ·
impl.sec.tools`) against the ~30,000 ceiling, 252 renders measured (D3 chunking has 1.94×
headroom) · genesis 598,626 B.

**Fact repairs to this record's earlier figures:** the skill matrix is **114** probes, not
86 (the 86 predates the authoring and patterns family waves) — 296 probes total ·
declared command floors are **110** by the validator and the shipped checker; 112 was a
grep count including two prose occurrences (F2 already carried both instruments) · the
anchor grammar accepts a **lettered sub-decision** (`D2a`) because the provenance sidecar
carries two — P3 refused to rewrite provenance to fit the validator, the grammar was
widened by lead-granted delta and pinned (RULING_RE parity, stricter than Python).

**Design facts settled at build (lead-ruled, disclosed in the reports):** protection is
checked **per migration** — lowering `class`/`kind`/`anchor` protection requires that
migration's anchor, and an anchored lowering followed by a later unanchored tombstone is the
sanctioned path (README corollary) · `hash:` is required on every migration
(`migration::with_hash` stamps it) · `replay::load` runs the hard set, so `Ok` means
deliverable · `import-document` on an existing document rejects; `tombstone-section`
rejects while rules remain · the similarity allowlist resolves by walking up from the log
directory, and the report always states `allowlist: none (N edges unsuppressed)` or the
suppressed count · rule field order normalises on emit (P1 A11, accepted partial); every
other declaration order is preserved · views are human-readable text under `--out` only
(comments cannot survive; the 8-line command kernel header is regenerated).

**Extension unit 1b — the family-2 checks (user-ruled 2026-09-04, "family 2 now"; BUILT
same day, commits `e66d76e` → `2f7ce11`, seat P1b on an approved plan with rulings Q1–Q6,
V1 PASS 0 blocking / 9 advisory → advisory round → delta PASS):** P3's matrix port named
46 command + 35 skill probes that exercise Python checks the wave-plan hard set never
listed. **Accounting at close, summed to 81:** 54 re-claimed into the ported ledgers · 12
family-1 shape errors the decoder rejects before a finding can exist · **3** family-3
single-skill-run claims with no referent in a whole-state validator (the earlier figure of
7 was wrong — four of those probes have referents, because a whole-state validator *is*
the end-of-sweep pass) · 11 family-4 `.md` pin/wording probes dead under D6 · 1 named
residual (a section whose `rules` key is absent vs null vs empty — the model reads all
three as an empty section; a `Section` field would tell them apart; an empty section is
already a finding unless it carries a `note:`). **Sixteen new codes** — nine rejecting
(`cite-unresolved` incl. tombstoned citations per ontology D5 · `pointer-unresolved` for
skill pointers behind `--plugin-root`, three failure shapes, never silent: "pointer
resolution: skipped (no --plugin-root)" prints otherwise · `superseded-field` for an inline
`ruling:` (D16) and `unknown-field` for any other stray key, both via a **preserved `extra`
map on `Rule`** — the lead's Q5 amendment, which exposed and fixed a latent lossy round trip
· `flat-rules` (D14) · `retired-label` · `labels-missing` · the residual structural checks)
and seven advisory (`cite-foreign` · `retired-selector` corpus-wide, a documented superset
of the Python's command-only scope · `pointless-override` · `orphan-block` ·
`zero-member-label` · `labels-inherited` · `skeleton-sigil`); the C3 absence-meaningful
guard was already ported at P1 (seven stale ledger rows corrected, zero new code); the
`class`-on-a-common-block divergence stays **rejecting** in Rust where Python warns (Q3,
disclosed in the ledger). Both code sets now carry set-equality coverage guards. **Corpus:
0 rejecting · 105 advisory** (the 13 new advisories are the Python's own warnings
one-for-one: 9 zero-member labels, 4 inherited-label absences) · 87 skill pointers checked,
all resolving. **Named gaps, not this unit's:** the 23 path-shaped command-side `pointer:`
values (`architecture` 7 · `implement` 14 · `common` 2) are resolved by no checker, Python
included; pointer climbs outside the plugin root are permitted (Python parity); the
citation scanner's word boundary is ASCII.

**Open at acceptance:** the contract suite had not run a real case — the sandbox was
unauthenticated; **closed 2026-09-04**: after the user re-authenticated the sandbox's own
Claude session and the lead installed rustup + gcc there, the suite ran for real — `2/2
cases passed, 2 ran, 1 assertion pending wave 3` (the hook-delivered install line), skew
measured as the same harness-level abort with the D5 wording injected verbatim, evidence
persisted per case under `evals/.work/` (F13) · `.claude/rules/mochiko/rust-cli.md` owes
one line documenting `MOCHIKO_FULL_SIMILAR` (governance surface; rides the wave-2 amend
run) · the manifest quirk from wave 0 (directory-string `commands` under `--plugin-dir`)
re-verified at wave 3 · the fixture's `2>&1` does not carry the halt (the harness labels the
output `[stderr]` regardless) — fixture wording to revisit at wave 3.

### F13 — the absence halt is harness-level, measured (2026-09-04, sandbox `claude-mochiko`)

With `mochiko-cli` off `PATH`, the fixture command's `!` line fails and Claude Code injects a
user message `<local-command-stderr>Shell command failed for pattern "!`mochiko-cli rules
brainstorm --section preamble 2>&1`": [stderr] /bin/bash: line 1: mochiko-cli: command not
found</local-command-stderr>` — and **no model turn happens** (`num_turns` 0, empty result,
`is_error` false, `claude -p` exit 0). The same class as wave 0's permission denial: a failing
`!` line aborts the command before the model reads anything. **Consequences, lead-stated,
put to the user at the next gate:** D3's positive-confirmation halt clause governs the cases
that DO reach the model (a policy placeholder, an oversized-render stub); absence — and, to be
measured, grammar skew, since a non-zero `!` exit is likely the same abort — halt earlier, at
the harness, loudly (the stderr line is visible) but with no install line from the `.md`. The
install line at fire is therefore carried by **D7(b)'s `UserPromptExpansion` hook** (exit 2
with the install text — verified at wave 0 to be the visible result) and by the `SessionStart`
hook's context; D7 is load-bearing, not belt-and-braces. D8's absence assertion re-keys to the
measured shape (no model turn · the stderr line naming `mochiko-cli` · no schema Read), with
"install line delivered" a wave-3 hook assertion. No ruling changes; two assertions do.

The first authenticated contract-suite run also exposed a runner defect: it built into the
worktree's shared `target/` and executed the host's macOS binary inside the Linux sandbox
(`Syntax error: "(" unexpected`). A sandbox-local `--target-dir` build produces a working
Linux binary; rustup and gcc were installed in the sandbox as a user would (D4's install
shape). Runner fix round routed to P3.

## Wave 2 — governance landed (2026-09-04)

`/mochiko:setup` amend run **AM-2**, input `wave2-amendments.md`, ran in this session: deck of
11 user-ruled · solo cold intent review by blind-map two-message dispatch (37 angles) →
`critical-gaps`, 14 survivors — **C1: the log did not ship in the plugin and nothing had ruled
where it would; user-ruled "plugin carries it"** (`plugins/mochiko/migrations/` from wave 3,
604 KiB, net ≈ +70 KiB at wave 6 — a D2/D9 amendment recorded here) · C2 marks split to
mirror this record · I1–I9/M1–M3 "as recommended" with four rulings (the wave-6
`primitive-edits.md` re-key is a pre-authorized PATCH · two release controls owed before the
first publish, discharge conditional · a SKIPPED contract suite blocks the bump · the crate's
release train is gated) · verify CLEAN 14/14 · **ratified** · producer (`tech-lead`) on an
approved plan · validator PASS 60/61 → six wording advisories → delta PASS · five flagged
proposals **accepted**. Governance **v2.0.1 → v3.0.0**: GI-020 superseded-by-ruling (D10.1
landed with its transition clause and Testability rows), GI-019's widened admission recorded
(D11 landed — its `Assumed` mark now graded PASS by the validator), GI-012 gates widened
(D10.3 + the SKIPPED and crate-train rulings), GI-002 live, GI-004/005/006 re-expressed,
GI-022 minted, `rust-cli.md` rewritten (six globs). Store scaffold written (`spine.md`,
Scope: developer tooling). Reports: `wave2-reports/`. **Owed after wave 2, before the first
publish:** signed tags + the `crates-io` approval rule; then lift `publish = false` and
`if: false`, tag `mochiko-cli-v0.1.0`, publish. **Wave 3 additions from this run:** move the
log into the plugin; `primitive-edits.md` globs; `README.md` re-authored.

## Wave 3 — pilot landed and accepted (2026-09-04, user: "accept wave 3")

**Open:** user-confirmed with four rulings as recommended — **Q-A** the dependency hook delivers
the rules only if it can see they are missing, else a presence line only; **Q-B** follow D3, no
hand-pinned count, `primitive-edits.md` criterion 3 gains a converted branch under governance
PATCH v3.0.1; **Q-C** wave 3 now, the crates.io publish later on the user's word (the README
carries the git-install line meanwhile); **Q-D** the read-back bar 3/3, pre-registered. Plan:
[`wave3-plan.md`](wave3-plan.md). Seats: P1 store move + legend (`staff-engineer`) / V1
(`validator`, PASS after one report-figure delta) · P2 plugin side (`staff-engineer`) / V2
(`validator`, PASS, six non-blocking findings) · P3 contract suite (`qa-engineer`) / V3
(`validator`, independent full re-run; FAIL on one item — three positive assertions still read
the interim channel union — reworked with all five advisories taken, delta PASS). Reports:
[`wave3-reports/`](wave3-reports/). `floor: tripped · seats: P1/P2/P3 produced · V1/V2/V3 reviewed`.

**F14 — hooks, measured (2026-09-04).** Platform docs verified by a `claude-code-guide` dispatch:
the `hooks.json` shape; `timeout` in seconds; `SessionStart` context is plain stdout and cannot
block; `UserPromptExpansion` blocks by exit 2 and adds context by JSON `additionalContext`;
`PreToolUse` denies by JSON `permissionDecision`; a non-2 failure fails open;
`disableSkillShellExecution` does not disable hooks (`disableAllHooks` does). Measured on the real
plugin: **`UserPromptExpansion` fires before expansion and its `prompt` is the raw user line** —
branch A is impossible by construction, so the hook ships as **branch B** (a presence line, never
rules); its matcher accepts an anchored regex on the namespaced name; its stdin carries eleven
fields (five beyond the docs); `SessionStart`'s stdin carries five (three fewer than the docs);
`--output-format stream-json` carries neither the expanded prompt nor any `UserPromptExpansion`
row — the session transcript does; a hook-blocked halt injects no `<local-command-stderr>` and
puts the harness notice in `result`. Captures: `evals/contract/fixture/hook-input/`.

**Built.** The log moved by `git mv` to `plugins/mochiko/migrations/` (state hash unchanged:
`sha256:8b61de5a…bdd4 · 50 documents · 1016 rules`; the plugin grows 1,366,116 → 1,992,896
bytes, +45.9 %); the preamble render's fixed `legend` block (612 bytes — a render-shape change,
named at the 0.104.0 bump per the crate rule's coordination clause); `brainstorm.md` re-pointed
per D3 (seven `!` lines with `--plugin-root "${CLAUDE_PLUGIN_ROOT}"`, the grant, the halt
clause, the CLI-printed count pin); `hooks/hooks.json` + `session-start.sh` + `dependency-halt.sh`
(absence or grammar skew only, converted primitives only — the converted check is the primitive's
own `!` line, no list; 5-second timeouts; POSIX `sh`, no `jq`); the maintainer advisory hook
(repo `.claude/settings.json` → `.claude/hooks/validate-migrations.sh`); two `[v0.104.0]` strip
entries on `brainstorm`; `primitive-edits.md` `paths` globs plus one-sentence converted-command
clauses on criteria 1, 3, and 11 (criterion 3 under Q-B; 1 and 11 lead-extended under the same
ruling, disclosed here); `README.md` as a two-step install; the contract suite at ten cases
(`hook-input` · `converted-shape` · `render-ceiling` host-side; `absence` · `skew` on the wave-1
fixture; `brainstorm-delivery` ×3 · `-absence` · `-skew` · `-hooks-off` · `-policy` on a staged
copy of the real plugin); the CI filter re-pathed. D13 checker on the pair: two findings, both the
conversion-expected substitutions, warnings identical to the pre-edit file (V2 item 7).

**Measured (sandbox `claude-mochiko`, Claude Code 2.1.259; reproduced by V3's own full run):**
10/10 cases, exit 0; **read-back 3/3** in every scored run (all seven `class: floor` ids exactly;
replicates byte-identical); **delivered read cost 10,839 bytes** (10,693 rendered + 80 + 66 of
hook lines) against the 12,819-byte baseline — **−15.4 %** (−24.5 % against the three-file
figure); store latency load-dependent, a band across four passes of 26–77 ms per section render
and 182–648 ms per seven-render fire, emitted by the suite to `latency.json`; largest render
2,102 bytes = 6.9 % of the inline ceiling; all six commands registered under `--plugin-dir` (the
wave-0 manifest quirk did not recur).

**Pilot abort criteria (D9): neither tripped.** (1) read-back 3/3 at the pre-registered 3/3 bar;
(2) delivered cost below the baseline. Waves 4–5 open on the user's word.

**Policy environment, recorded (D8, four observations):** with `disableSkillShellExecution` set
and hooks on, every run delivered zero blocks, read no schema file, and heard the hook's presence
line; the prose halt held in two of four (the others replied `FLOOR: none` without halting). The
no-fallback posture held 4/4; the prose clause is a guard that fires half the time — evidence for
GI-020's unsupported declaration, and the one path where the clause is load-bearing.

**Deviations disclosed:** the maintainer hook is a script, not an inline command; hook scripts
derive the root from `$0` with `CLAUDE_PLUGIN_ROOT` as override; the dependency hook stays silent
on a non-0/non-3 `migrate status` exit; `SessionStart` prints on it; criteria 1 and 11 joined
criterion 3 in the converted-clause set; the suite gained `converted-shape` (which also
cross-checks the pre-registered floor set against the render) and a `rec` status; P3 fixed two
defects in its own assertions before the final run (a too-narrow no-Read match; a head-line count
matching the halt clause's quoted template) and, after V3, removed the interim channel union from
every positive assertion.

**Follow-ups minted (BACKLOG):** the six `fail-conditions` section intents still say "the `.md`
Not-done line hard-codes this set's count" — falsified for a converted command; the grammar has
no section-reword op, so wave 4 adds one (grammar-1 extension before the first publish) and a
migration rewording the six (V2 F1). The hooks and the D5 message name `cargo install
mochiko-cli`, which works only after the publish (V2 F3) — the publish is the fix, not a wording
change.

**Governance PATCH v3.0.1:** the pre-authorized glob half, the Q-B criterion clauses, GI-011's
ledger home, GI-020's dormant Testability tier activated by the pilot, the revisit trigger
evaluated and not tripped, the region's "today the log lives at the repo root" clauses struck.

## Wave 4 — the five remaining commands, landed with a tripped read-back and accepted (2026-09-04, user: "accept wave 4")

**Open:** on the user's "wave 4"; lead's standing assumptions (plan §7: grammar stays 1 before
the first publish; skew, hooks-off, policy run once; the bar 3/3 per command including
`implement`'s 34 ids; no governance row). Plan: [`wave4-plan.md`](wave4-plan.md); reports:
[`wave4-reports/`](wave4-reports/). Seats: P1 crate + log (`staff-engineer`) / V1 (`validator`,
PASS after one rework on three documentation items) · P2 plugin side (`staff-engineer`) / V2
(`validator`, PASS on all five pairs) · P3 suite (`qa-engineer`) / V3 (`validator`). Rulings at
approval: the genesis fidelity byte test rebuilds from a **frozen corpus fixture** of the pre-edit
v0.103.0 schema files (D8's own words) rather than a new builder path; `implement` Entry's three
`implement.yaml` citations reworded to the rendered homes (`impl.sec.tools`, the preamble `vars`
block) with an eleventh strip entry — a live pointer to a file the run must never read is the
defect; `specify` Goal's template two-arm sentence stays byte-identical for the wave-6 32-site
migration; the legend gains three lines (labels · moments · empty-enforces). One transport note:
the legend addendum reached P1 while it was still building and my verification raced its write;
the supersession I sent was redundant, P3's sandbox run was held until the tree settled.
`floor: tripped · seats: P1/P2/P3 produced · V1/V2/V3 reviewed`.

**Built.** `reword-section {schema, id, title?, intent?, note?}` (grammar 1, sixteenth op,
between `mint-section` and `tombstone-section`; rejects none-named, `~` on title/intent, blank
values, unknown or tombstoned sections; touches no rule, so no anchor is owed); `migrate stamp
<file>` (exit 1 unparseable, 2 unreadable; a stamp of the committed genesis is a byte-for-byte
no-op); migration `0002-fail-conditions-intent.yaml` (anchor `2026-09-03 cli-schema-delivery D3`,
six rewords, state `sequences 1..2`, hash `8972891099f7…43fd`, 0 rejecting · 105 advisory); six
one-line snapshot edits (hand-applied — regeneration would drop in-body comments and layout; the
log README now says so); the frozen fixture (51 files, 603,801 bytes, byte-identical to HEAD
before the edits); legend 612 → 845 bytes; five `.md` re-points (35/35 renders with head and end
lines; pins 1 · 1 · 15 · 6 · 9 agreeing with the end lines; D13 checker two conversion-expected
findings per pair, warnings unchanged); eleven `[v0.105.0]` strip entries, Content blocks
machine-verified; README wording for all six; the suite generalized from the pilot (converted
commands discovered from their `!` lines; per-command delivery ×3 and absence; `hook-input` rows
per command; `converted-shape` cross-checks every pre-registered floor set against the render).
Crate: 331 tests, four layers green.

**Measured (sandbox `claude-mochiko`, 29 sessions, 20/20 cases, 158 assertions, exit 0):**

| command | read-back (bar 3/3) | delivered bytes | baseline | delta |
|---|---|---|---|---|
| architecture | 3/3 of 22 | 18,569 | 23,026 | −19.4 % |
| brainstorm | 3/3 of 7 | 10,933 | 12,819 | −14.7 % |
| feature | 3/3 of 13 | 17,346 | 21,020 | −17.5 % |
| implement | **1/3** of 34 | 35,411 | 44,266 | −20.0 % |
| setup | **1/3** of 18 | 16,283 | 20,245 | −19.6 % |
| specify | **0/3** of 16 | 19,456 | 23,434 | −17.0 % |

Latency 26–33 ms per section, 181–220 ms per fire (sandbox). Largest render `impl.sec.tools` at
51.5 % of the ceiling; `implement`'s whole render (35,418 bytes) exceeds the ceiling, which is
why D3 chunks. Policy environment: five observations now, the prose halt held in three, no schema
file read in any.

**Abort criterion (1) TRIPPED on three commands; criterion (2) clear on all six.** Delivery held
on every replicate of every command: the lead verified on the transcripts that every missed
floor rule is present verbatim with its `class: floor` line. The misses: `specify` omitted the
same single id (`spec.author-grader-default-fail`, the one floor in `spec.sec.ways-of-working`)
in all three replicates; `setup` omitted its three ways-of-working floors in two of three;
`implement` omitted `impl.graded-fold` (42nd of 44 rules in the 15 KB tools block) in one and
refused the probe argument at its own Entry gate in another. A recall failure on long lists and
on ways-of-working floors, not a delivery failure — but the bar was pre-registered and a miss is
a trip. V3's independent `implement` re-run scored 0/3, dropping the same `impl.graded-fold`:
across two runs `implement` stands at 1 of 6 replicates, the same late-position id missed in
three of the four failures. **User-ruled "land + diagnostic":** wave 4 lands with the trip recorded; wave 5 is
halted; the read-back diagnostic pre-registered at plan §8 (gate-valid probe arguments; a
count-only read-back beside the exact-id one; recorded, never gating) separates recall from
delivery before any wave-5 ruling.

**Follow-ups minted (BACKLOG):** the D13 advisory checker now flags every converted command
permanently (two conversion-expected findings) — it loses its regression value until the wave-6
Python retirement; the `specify` two-arm sentence (wave 6); the publish tail unchanged.

## Landed (2026-09-03)

- `DECISIONS.md` row (2026-09-03) — status "ruled — build pending (wave 0 probes first)".
- `BACKLOG.md` — new section "CLI schema-delivery build" (the six-wave build item, wave 0
  first); the three delivery watches (template-schema CLI · command D10 · skill D6)
  annotated as superseded-by-ruling pending the wave-6 landing (D10.6).
- `ROADMAP.md` — the Template-schema CLI Next row extended with this ruling (Next stays at
  its 7-row cap; no new row).
- `.mochiko/brainstorms/index.md` — entry status accepted, outcome pointers above.
- Review evidence: `review-lens-a.md`, `review-lens-b.md` beside this record (each its
  reviewer's single-writer file, delta-check notes appended).
- Worktree note: this session ran in worktree `cli-context`; the sibling
  `producer-plan-enforcement` entry (untracked on `main`) is absent from this worktree's
  index — the two index heads merge at integration, both entries kept.
