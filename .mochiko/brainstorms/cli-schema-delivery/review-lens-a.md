# Cold Review — Lens A (Decision Quality)

**Record under review:** `.mochiko/brainstorms/cli-schema-delivery/record.md` (frozen, 608 lines)
**Reviewer:** cold end-stage reviewer, lens A. Never in the room. Defaults to FAIL.
**Protocol:** `mochiko:review-brainstorm` — blind angle map (Phase 0, built before record contact)
→ independent cold read → six hunt classes → source verification → fitness → coverage diff.
**Date:** 2026-09-03. Worktree `cli-context` at `1ed5c19`.

---

## Part 1 — Blind angle map (Phase 0)

Built from the topic statement and goal line alone, before the record's path was known, grounded
in `CLAUDE.md`, `.mochiko/memory/governance-ledger.md`, `DECISIONS.md`, `crates/mochiko-cli`,
`plugins/mochiko/schemas/`, `scripts/`, `.claude/rules/mochiko/`, `.github/workflows/ci.yml`.
No session artifact was opened. **40 angles.**

### Group I — governance envelope

1. **GI-020 head-on collision.** Ledger testability reads: "Pass: a fresh plugin install with no
   binary present is fully functional; schema data files Read raw. Fail: install requires a build
   step, fetches a binary, or fails without the binary." No-fallback is that exact Fail row. The
   record must amend GI-020 by recorded ruling or show why it does not bind. Silence is fatal.
2. **Amendment semver and route.** Ledger policy: MAJOR for "principle removal / incompatible
   redefinition". Killing additive install is MAJOR, routed through `/mochiko:setup`, user-ruled.
   Version, route, and approver must be named.
3. **GI-019 bright line, clause 1.** "Never gates pipeline progress." A CLI that is the sole
   delivery path for every command and skill gates every run by construction. The record must show
   how the line survives, or rule it superseded.
4. **Kernel-class admission is per-instance.** GI-019 admits by recorded ruling; the D11 admission
   was scoped to template-schema delivery. The record must mint a fresh, wider admission, not ride
   the old one.
5. **GI-019 named-ban proximity.** "Capability catalogs" stay banned absent ruling. A queryable
   store of all rules is close. The distinction must be argued, not asserted.
6. **D8 supersession named.** `schema-based-template-guidance` D8 ruled "binary = renderer, raw
   Read the fallback". It must be cited and superseded explicitly, with a strip entry.
7. **command-content-schema D1 contradiction.** D1 ruled "runtime interpretation, no build-time
   render". Pointing every `.md` at the CLI reintroduces a render step.
8. **GI-002 supply-chain trigger.** The ledger annotates a dormant shipped-executable vector,
   revisit at first public release. A mandatory binary plus shipped hooks fires it.
9. **GI-012 gate rewrite.** Gate 5 is "schema-data/binary consistency, asserted by a crate test
   parsing the shipped YAML". If YAML dies, gate 5 has no referent.
10. **GI-008 waiver trigger.** Six helper scripts are waived while "not load-bearing in a shipped
    flow". Migration tooling and DB checkers are load-bearing.

### Group II — store shape

11. **Does a database earn its place.** Access pattern today is bulk-read-one-document. SQLite buys
    query and transactions; neither is used by a whole-file read.
12. **The obvious steelman: DB as derived cache.** YAML stays truth and git-diffable, SQLite is a
    build artifact. Most benefits, near-zero governance cost. Absent and unkilled would be Critical.
13. **`patterns-adopt-first` applied to itself.** The repo's own discipline puts storage engines and
    persisted formats above the retrofit-cost line, user-ruled, with a named shelf candidate.
14. **Minimalism ladders applied to itself.** A DB plus migrations plus hooks with no rung-wise
    disclosure is inconsistent with what this repo demands of every other producer.
15. **Diffability loss is the load-bearing cost.** Git review, byte-exact strips, author≠grader
    reads of "the primitive's own text", and near-dup detection all assume readable text.
16. **Merge and concurrency.** Concurrent worktrees and sessions. A binary SQLite file cannot merge.
17. **What is "content".** Shipped rule schemas, per-project records, and provenance are three stores
    with different mutability and migration semantics. An undefined noun cannot be soundly ruled on.
18. **Provenance sidecar's fate.** Repo-side only, never shipped, per D16. Does it enter the store?

### Group III — migration model

19. **Three-way version skew.** Plugin, binary, DB grammar. Old-binary/new-data and the reverse both
    occur in a marketplace install.
20. **Migration ownership.** Who runs migrations, when, on whose machine, with what permission.
21. **Migration reversibility.** GI-006 requires every primitive edit reconstructible. What replaces
    the byte-exact strip when the source is rows?
22. **Corruption and backup.** A single-file store is a single point of loss for the rule library.

### Group IV — the `.md`-to-CLI binding

23. **Invocation mechanics.** A CLI is reached through a shell. Permission prompts or allowlists in
    every consuming project.
24. **Tool-availability failure.** Restricted tool sets, sandboxed shells, denied permission, hostile
    PATH — all yield no rules under no-fallback. Halt semantics must be enumerated.
25. **Read-cost accounting.** Recent waves measured delivered multipliers and treat read cost as
    first-class. A shell round trip is not obviously cheaper than a Read.
26. **Blast radius count.** 20 command schemas, 30 in-skill schemas, ~66 `.md` files citing schema
    paths. Every re-point is a primitive edit taking strip plus audit ceremony.
27. **Grader independence.** If the only reader is the CLI under review, the grader depends on the
    thing being graded.

### Group V — no-fallback posture

28. **Who the fallback protects.** Unsupported platforms, restricted environments, old installs.
    Whose access is withdrawn must be named and accepted.
29. **Per-platform distribution actually priced.** Under mandatory the price changes: multiple
    targets, signing, checksums, and a git-based marketplace that ships source, not artifacts.
30. **How the binary reaches the user.** No install-time build hook exists. The mechanism must be
    concrete and must not be an install-time build step, which GI-020 bans by name.
31. **Rollback path.** Once YAML is deleted and the `.md` files are re-pointed, reversal is a second
    wave. Trigger, cost, and caller must be stated.

### Group VI — test regime

32. **Name the oracle.** Deterministic failures with exit codes versus guidance-quality regressions
    with none. The repo's own eval work found plan-only grids noise-dominated.
33. **Checker fate.** ~5,225 lines of Python parse YAML. Rewritten into the crate, the advisory
    carve-out that keeps them outside GI-019 may no longer hold.
34. **CI path filter.** CI triggers on `crates/**` and `plugins/mochiko/schemas/**`. Content moving
    out of that path silently drops coverage.

### Group VII — hooks

35. **Hooks versus the bright line.** A blocking `PreToolUse` hook gates progress and can sequence
    work — GI-019 clauses 1 and 2 directly.
36. **Hooks are shell on the user's machine.** Plugin-shipped hooks execute in every consuming
    project. Consent, trust, and the GI-002 annotation attach.
37. **Scope discipline.** "Explore hooks" is a fourth workstream on three large ones. Fence it or
    defer it; an unfenced exploration riding a ruling is an unowned decision.

### Group VIII — evidence and honesty

38. **The driver problem, stated.** What observed pain does this fix.
39. **Evidence marker.** Honest `Assumed` / n=0 marks with an owed falsifier.
40. **Foundation-bet honesty.** D11 conceded "the machine rides the owned foundation bet". If that
    is the real driver again, say so rather than dress it as a delivery requirement.

---

## Part 2 — Verification performed

**Repo claims source-verified** (worktree `1ed5c19`): F1 crate line total 599 including
`Cargo.toml` (11) — correct · three commits, last at v0.91.0 — correct · F2 20 schema YAML files,
`implement.yaml` 1,019 lines / 105 rules (checker-confirmed), 30 of 38 skill dirs carry
`schema.yaml` — correct · F4 router row naming `architecture-shelf-backend` as a CLI template
while the binary rejects it — **confirmed**, the binary prints `error: unknown template
'architecture-shelf-backend'` · F4 `${CLAUDE_PLUGIN_ROOT}` appears only in `setup.yaml` — correct ·
two-arm string occurs 32 times across 24 files ("~30 sites" — fair) · F5 script line counts
1,239 / 1,094 / 457 and allowlist 214 rows — correct; `check-command-schema.py` re-run: `0
findings, 14 warnings — PASS` · F6 99 strip files, 2.8 MB — correct · F7 GI-019 / GI-020 quoted
text — matches the ledger verbatim · F8 no hooks anywhere — correct · F10 10 agents, 76,008 bytes,
all `model: opus` — correct · F11 `converting-skill-to-schema` absent — correct ·
`producer-plan-enforcement` absent from this worktree — correct.

**One repo claim broken:** F2's "every command schema carries a `tombstones:` block" (see M1).

**External claims verified live** (disclosure: `verified: code.claude.com/docs/en/skills`,
`code.claude.com/docs/en/plugins-reference`, fetched 2026-09-03). The record discloses its F9
block as a `claude-code-guide` dispatch and marks unknowns — good practice. Independent re-fetch
confirms F9.3 and F9.4 in substance and surfaces four facts the record does not carry; those drive
C1, I2, I3, and M6. Per `EXTERNAL-CLAIMS.md`, the source re-read clause is owed on C1, I3 and M6
before they survive: the counterpart reviewer or the lead re-reads the quoted source cold.

---

## Part 3 — Survivors

### CRITICAL

---

**C1 — D3, D8. The no-fallback halt clause does not cover the documented case where `!` execution
is disabled. Silent, total loss of the rule layer.**

D3 rests on `!` preprocessing being "the one deterministic, model-independent delivery path", and
its halt clause fires on "an error or an empty block". Live-fetched documentation names a third
outcome the record never considers:

> "To disable this behavior for skills and custom commands from user, project, **plugin**, or
> additional-directory sources, set `"disableSkillShellExecution": true` in settings. Each command
> is replaced with `[shell command execution disabled by policy]` instead of being run. Bundled and
> managed skills are not affected. This setting is most useful in **managed settings, where users
> cannot override it**."

And for synced-skill contexts:

> "A `!` command line reaches Claude as literal text too, or as that placeholder when
> `disableSkillShellExecution` is on."

Failure scenario: an enterprise deploys mochiko under managed settings with that flag set. Every
converted command and skill delivers a `.md` whose rules block is the literal string `[shell command
execution disabled by policy]`. That is neither an error nor an empty block, so D3's halt clause
does not fire. Under no-fallback there is nothing to fall back to. The plugin runs with zero rules
and no signal. This is the exact silent-degradation class the record says the design exists to
eliminate, arriving through the mechanism chosen to eliminate it — and the affected user cannot
turn it off.

Compounding: the record's own D3 rationale concedes the halt is model-enforced. The `.md` instructs
the model to stop. Driver A exists because an instructed Read is "instructed, not forced" (F3). An
instructed halt carries the same guarantee class. The record never says so.

*Disposition:* the halt clause must key on positive confirmation, not on absence of error — the
render's first line already carries the version triple (D5), so the `.md` can require that triple
and halt on anything else, including the policy placeholder. Add `disableSkillShellExecution` and
the Cowork/synced contexts to F9 as a named environment class, decide whether they are supported or
declared out of scope, and add both to D8's contract-suite variants beside absence and skew.

---

**C2 — D9, D10.1. The rewritten GI-020 is false for waves 2 through 5. A ratified non-negotiable
the tree violates by design for four waves.**

D10.1 rewrites GI-020 to read, in part, "it depends on the separately installed `mochiko-cli` for
every command and skill; absence or version skew halts loudly at first use and never degrades; **no
file-read fallback exists**". D9 lands that amendment at wave 2, explicitly "**before any `.md`
points at the CLI**", and wave 1 regenerates derived snapshots "in today's exact file shapes … so
every unconverted `.md` keeps reading them raw".

So on the day the amended principle is ratified, zero commands depend on the binary and all six read
files. The file-read fallback the principle says does not exist is the only delivery path in the
tree. It stays that way through wave 5. Meanwhile GI-012 gates every wave's `plugin.json` bump
against a governance surface that is false on its face.

This is not a wording nit. Governance principles in this repo are ratified as live constraints with
Pass/Fail testability rows, and `validation-constitution` grades them. A principle that is
aspirational for four releases either forces the validator to pass a false statement or blocks every
intermediate bump.

*Disposition:* pick one. Either move the GI-020 amendment to wave 6 and give waves 3–5 a recorded
transition allowance, or write the amended principle with an explicit transition clause naming the
conversion window and its end condition. Say which in D10.1, and re-state D9's wave-2 rationale
accordingly.

---

**C3 — Coverage gap. The road that fully serves both `high` drivers at zero governance cost is on
no question's menu and is nowhere recorded as rejected.**

The Problem section ranks **B (change management) high**, **C (integrity) high**, **A (delivery)
medium**, **D (run control) medium**. D1's migration log, D2's scope, and D6's Rust validator serve
B and C completely. None of them requires the `.md` to point at the CLI, requires no-fallback,
touches GI-020, touches GI-019, or needs a binary on any user's machine. The store could be
maintainer-side only: log as truth, Rust validator, generated YAML snapshots committed exactly as
they are today, every `.md` unchanged.

That road appears nowhere. Q2 offers three source-of-truth shapes; Q3 three scopes; Q4 offers three
ways to bind `.md` to the CLI and no way not to; Q7 three enforcement postures. The Rejected lines
under D3 read "instructed Bash as the primary (keeps A where it is)" and "hook injection" — both
still CLI bindings. So the single decision carrying the entire governance cost, the whole binary
dependency, the Windows problem, the new test class, and the 36 re-points was never contested
against the option of not making it.

`RECORD-FITNESS.md` requires "the alternatives considered are named with why they lost, so a future
reader does not re-derive them". A cold reader cannot tell from this record whether the
maintainer-side-only road was weighed and beaten or never seen.

The user's ask does pre-rule the no-fallback posture, and that is the user's call. But a record whose
own driver table ranks delivery *medium* and then spends a MAJOR governance amendment on it owes the
cheaper road an explicit, reasoned kill.

*Disposition:* add the maintainer-side-only road to D3's Rejected line with the reason it lost, and
state plainly in the Problem section or D10 that the governance cost is bought by driver A at
`medium` rank on the user's explicit instruction. If the user wants it re-opened, it is one question.

---

### IMPORTANT

---

**I1 — D10.2. "GI-019 is argued, not amended" narrows a non-negotiable's operative verb by attached
clause, while the narrower GI-020 change is routed as MAJOR. The asymmetry is unexplained and the
argument is asserted, not made.**

Three limbs.

*(a) The species distinction is stated, not argued.* D10.2 says a required binary whose absence
halts a run is "an infrastructure dependency, a different species from the output-gating the bright
line names — the binary gates nothing by its output". But GI-019's testability Fail row reads "an
admitted binary that **gates the pipeline**, sequences agents, or holds skill-owned judgment". It
does not say "gates by output". The record introduces the by-output reading in the same clause that
benefits from it and never engages GI-019's own rationale text, which frames the line as keeping
admitted tooling "to delivery/composition roles". The reading is defensible — delivery is the
licensed role — but it must be argued against the ledger's actual words.

*(b) The route is inconsistent.* The ledger's amendment policy sets MAJOR for "incompatible
redefinition". Attaching clauses that narrow what "gates pipeline progress" means is a redefinition
of scope even with the text unchanged. D10.1 correctly routes the GI-020 change through
`/mochiko:setup` as MAJOR. D10.2 routes a change to the project's single bright line through a
record entry. Same run, two standards.

*(c) D6's hard reject sits closer to the line than admitted.* D6 has the CLI reject a migration
outright, and D10.2 defends it as "maintainer-time definition of the store's own data … nothing here
runs inside a pipeline". But the primitive-edit landing ritual is a pipeline with gates in this
repo — GI-012 gates the bump, `primitive-edits.md` gates the edit. A binary that refuses a
maintainer's rule change is a pass/fail checkpoint on pipeline work.

Also unaddressed: GI-019 admits kernel-class tooling "ONLY by a recorded ruling", per instance. The
D11 admission was scoped to template-schema delivery. This record widens the role to all content
delivery plus hard constraints, and D10 records supersessions but never states that it is itself the
fresh admission ruling for the widened role.

*Disposition:* fold the GI-019 clauses into the wave-2 `/mochiko:setup` amend run alongside GI-020,
argued against the ledger's Fail-row wording rather than a fresh paraphrase; state explicitly that
this record is the admission ruling for the widened kernel-class role; and answer (c) directly —
either the landing ritual is not a pipeline for GI-019 purposes, or D6's hard set needs the
maintainer-time carve stated as a ruled exception.

---

**I2 — D3, D9. The wave-0 probe set is stale against D4 and keys the numeric abort on the wrong
questions.**

D3 owes three probes: `!` permission grant, "whether **the plugin's `bin/`** is on `PATH` for the
preprocessing shell", and `${CLAUDE_PLUGIN_ROOT}` expansion in bodies. D9 makes those three the
wave-0 gate. But D3 was ruled at Q4 and D4 at Q5, and D4 rules that **the plugin ships no binary at
all**. There is no plugin `bin/`. Probe two is dead, and probe three was only reachable through it.

Two of the three are also answerable from documentation today:

> "`${CLAUDE_PLUGIN_ROOT}` | The plugin's installation directory. **Substituted only in plugin
> skills.**"
> "Claude Code substitutes `${CLAUDE_SKILL_DIR}` and `${CLAUDE_PROJECT_DIR}` in two places: the
> skill's markdown content, and Bash rules in the `allowed-tools` frontmatter. In a plugin skill,
> Claude Code substitutes `${CLAUDE_PLUGIN_ROOT}` and `${CLAUDE_PLUGIN_DATA}` in the same two
> places."
> "Using the same variable in both places lets a skill run a bundled script **without a permission
> prompt**."

That resolves F9.10(c) and gives F9.10(a) its documented pattern.

The live unknown that D4 actually creates is on nobody's list: **does the `!` preprocessing shell see
a user's `~/.cargo/bin` or Homebrew `PATH`?** Those directories are typically added by an interactive
shell profile. Documentation says injected commands run through the Bash or PowerShell tool in the
session's working directory; it does not promise a login shell's `PATH`. If they are not visible, the
`!` line cannot name `mochiko-cli` bare, and D4 offers no `${CLAUDE_PLUGIN_ROOT}` path to fall back
on because nothing ships in the plugin.

*Disposition:* rewrite F9.10 and D9's wave 0 against D4. Drop the plugin-`bin/` probe. Close
`${CLAUDE_PLUGIN_ROOT}` and the `allowed-tools` pattern from the cited documentation. Add the
cargo/brew `PATH` visibility probe and make **it** the numeric abort, since D3 and D4 both die on it.

---

**I3 — D4, D9, Open threads. Windows is broken, not deferred, and the record has the fact that
breaks it.**

Documentation, live-fetched:

> "`shell: bash` when bash isn't available: **the invocation fails before any command runs.** This
> happens on Windows without Git Bash. Claude Code shows ``Skill <name> requires bash (`shell: bash`
> in frontmatter) but Git Bash was not found``."

And the `shell` frontmatter key defaults to `bash`, with PowerShell "on by default on Windows without
Git Bash".

So on default Windows, every converted command and skill fails outright at invocation. The fix is
`shell: powershell` per primitive, which changes the `!` line's syntax and adds a second rendering
path to test. The record's Open threads reduce this to "`!` preprocessing under Git Bash is a wave-0
probe item if a Windows runner is available".

Second limb: D4's priced consequences say "Windows served by cargo only". `cargo install
mochiko-cli` requires a Rust toolchain on the user's machine and compiles from source. That is an
install-time build step, relocated from the plugin to the tool. D10.1's rewritten GI-020 is worded to
permit it, so there is no contradiction — but the cost line reads as a distribution footnote when it
is a toolchain prerequisite.

*Disposition:* rule Windows in or out. If in, the `shell:` key is a D3 design element and a D8
contract-suite axis, not a probe. If out, say so in D4's cost line and in the amended GI-020, and
name it in the record's Open threads as a declared unsupported platform. Either way, restate D4's
Windows cost as "install a Rust toolchain and compile", which is what it is.

---

**I4 — D1, Build surface. The store engine is routed to builder's room against a standing ruling,
and SQLite is never priced against a cheaper shape.**

The build surface reads: "`rusqlite` bundled, or the simplest in-process store that satisfies
'file-based DB' — **builder's room**, argued against `patterns-adopt-first`". But
`build-vs-off-the-shelf` D4, verified in `DECISIONS.md`, rules custom-over-shelf "**user-ruled above
the retrofit-cost line** (persisted formats · storage engines · locking/concurrency ·
**migration-bearing shapes**), seat-decidable with disclosure below". A storage engine in a
migration-bearing shape is above that line twice over. It is the user's ruling, not the builder's.

Substantively, D1 makes SQLite a **projection** rebuilt from the log — a cache. The record never
prices it against the obvious cheaper shape: replay the log in memory on each invocation, or
serialize one index file. 1,016 rules across 50 files is small; today's Python checkers parse the
whole corpus in seconds. A persistent store would earn its place on render latency at `!` fire, since
preprocessing is synchronous — but the record never states that requirement, so SQLite survives
because the driver ask named it, not because a need was weighed. Framing the choice as satisfying
"file-based DB" treats the user's example as a constraint.

*Disposition:* lift the store engine out of builder's room into a user-ruled question, with the
latency requirement stated as the criterion and at least one non-SQLite candidate named per
adopt-first's disclosure floor. If SQLite still wins, that is a two-line rationale and the ruling is
sound; the defect is the routing, not the likely answer.

---

**I5 — D6, D10.6. GI-004's author≠grader expression for schema content is left undefined, and the
only independent text read of effective state is assigned to builder's room.**

D2 ends strips for schema content. D6 collapses the `.md` audit criteria to "the `!` line and halt
clause are present and name the right primitive". D10.6 re-keys `primitive-edits.md` so the strip
ceremony becomes "a migration carrying its ruling anchor". Every one of those replaces a *strip*
obligation. None replaces the *audit* obligation.

GI-004 is non-negotiable: "Every shipped-primitive edit MUST pass the author≠grader audit before the
`plugin.json` bump that ships it", and `plugins/mochiko/schemas/**` entered that path scope at
v0.76.0. After wave 6, an ordinary rule reword is one migration file. No seat is named to grade it,
and the criteria that seat would use no longer exist.

Second limb, angle 27. A grader must read the artifact, not the author's say-so. Effective rule state
after `${var}` substitution, `extends:` resolution, and log replay is visible only through the CLI —
or through D1's derived snapshots. The snapshots are therefore the independent read path that keeps
the audit honest. Yet Open threads treat their long-term format as free: "whether the long-term
derived view is the same YAML or a readable render is builder's room, decided when no `.md` reads a
snapshot anymore". GI-004 and GI-006 constrain that choice; the record does not say so.

*Disposition:* state the steady-state audit unit and criteria for a schema migration in D6 or D10.6.
Add to the snapshot open thread that the derived view MUST stay human-readable text, because it is
the author≠grader read path and the GI-006 reconstruction surface — that is a constraint, not
builder's room.

---

**I6 — D8. The release gate is a stochastic test with no sample size, no pass bar, an unpriced run
cost, and a substrate whose own recorded finding was noise dominance.**

D8 makes the plugin contract suite blocking under GI-012 gate 6: headless `claude -p` runs "per
command and per converted skill", plus absence and skew variants, asserting "the `!` line executed,
the version triple present, the floor read-back stated, and no schema file Read anywhere".

Three unaddressed problems.

*Sample size and bar.* Three of those four assertions are model behaviors. A single passing run does
not establish them; a failing run may be noise. The record sets no N and no threshold, so "green"
is undefined for the gate that carries the user's entire "the plugin doesn't fail" requirement.

*Noise precedent.* D8 leans on `evals/commands` as proven substrate. That work's own recorded finding
in `DECISIONS.md` is "24 plans — **noise-dominated per its own prereg guard, no stable regression
attributable**", and the near-dup wave repeats "noise falsifier re-confirmed instrument-side". The
record cites the substrate's invocability and ignores its measured discriminating power.

*Cost.* Six commands plus 30 skills, times normal/absence/skew, is on the order of 100 headless model
runs for a full suite. D5's own rationale describes this repo's content bumps as near-daily. The
record never prices the API spend or the wall-clock on a blocking gate.

*Disposition:* separate the deterministic assertions (the `!` line executed; the version triple
present; no schema file Read) from the behavioral one (floor read-back stated). Gate on the
deterministic set with N=1. Give the behavioral one an explicit N and pass bar, or make it a reported
metric rather than a gate. State the per-suite cost and which subset runs per PR versus per bump.

---

**I7 — D7(a), D10.4. Shipping `hooks/hooks.json` is the plugin's first shell execution on every
consuming project, and the governance envelope discharges the supply-chain line for the crate only.**

F8 verifies that the plugin ships no hooks today and the repo configures none. D7(a) ships a
`SessionStart` hook running `mochiko-cli --version` in every session of every project that installs
mochiko. That is a new capability class for this plugin: code the plugin author controls, executing
on the consumer's machine, on every session start, without a per-run prompt.

D10.4 fires GI-002's risk line and discharges it "at wave 1 (signed tags, `cargo audit`,
checksum-published artifacts as builder's room)" — all crate-publication controls. None of them
covers the shipped hook, which reaches users through the plugin, not through crates.io.

Second limb: the hook's own failure modes are unpriced. A `SessionStart` hook that hangs, errors, or
is slow degrades every session in every consuming project, for a benefit D7 itself describes as
"earlier loudness".

*Disposition:* extend D10.4's discharge to name shipped hooks explicitly, or record hooks as a
separate GI-002 revisit. Add a timeout and a fail-open requirement to D7(a) — a presence check must
never be able to break a session. State that the hook ships to all consumers so the user ratifies
that knowingly.

---

**I8 — D9, D10.4. Wave 1 publishes to crates.io before the wave-2 governance amend run.**

D9 puts the release pipeline and `publish = false` lifting in wave 1; D10.4 calls that "the crate's
first public release". The ledger's standing amend triggers include "public-product transition (GI-002
— compat obligations)", and the amendment policy makes fact-profile changes governance events routed
through `/mochiko:setup`. Wave 2 is the amend run. The trigger therefore fires one wave before the
route that handles it, and a public crates.io publication is not retractable.

*Disposition:* move the crates.io publication into wave 2 alongside the amend run, or split wave 1's
release pipeline so the tag-and-build machinery lands in wave 1 and the first public publish lands in
wave 2. Name whichever in D9.

---

**I9 — D10, trail Q11. The governance envelope was ratified as one bundle; its most contestable limb
inherits `Confident` from a bundle vote.**

Q11 offered "adopt D10 as stated · amend a clause" and the answer was "adopt as stated". D10 carries
five distinct governance moves — a MAJOR GI-020 rewrite, the GI-019 reinterpretation, a GI-012
widening, a GI-002 discharge, and six supersessions — plus a ceremony re-key. The GI-019 limb (I1) is
the one a skeptical reader is most likely to refuse, and it was never put on its own.

Related pattern: nine of twelve questions were answered "as recommended", with D4 the single
divergence. Each question offered three real options, so this is not passive acceptance in the
protocol's sense. But `RECORD-FITNESS.md` asks for `Assumed` on recommendation-led adoptions, and
D10.2 in particular has no independent test behind its `Confident`.

*Disposition:* split D10.2 out for its own ruling and mark it `Assumed` until it is separately
confirmed, or re-put it as one question at disposition. Leave the other four limbs as ruled.

---

**I10 — D9. No abort criterion exists after wave 0, and wave 2 is a one-way door.**

D9's numeric abort covers exactly one condition: "`!` preprocessing failing in command bodies kills
D3 and returns the design to the user before any build". After that, rollback is "user-reserved per
wave" — a reservation, not a trigger. From wave 2 onward, reversal means a second MAJOR governance
amendment plus re-pointing every converted `.md`.

The v0.76.0 precedent this record cites did better: it carried an explicit rollback trigger, "the
first-live-run watch shows CLI guidance underperforming the `.md` baseline". D10.6 supersedes those
watches with the contract suite, and the contract suite tests delivery mechanics, not whether
CLI-delivered guidance is as good as the `.md` baseline. So the record retires its predecessor's
falsifier and mints no replacement.

*Disposition:* give waves 3 and 4 a stated abort criterion keyed to something observable at the pilot
— the measured read-cost delta and a rule-fidelity or floor-read-back comparison against the current
baseline — and state what the user does if it trips after wave 2's amendment has landed.

---

### MINOR

**M1 — F2 fact error.** "Every command schema carries a `tombstones:` block" is false. Verified: only
`brainstorm.yaml`, `setup.yaml`, and `specify.yaml` carry one; `implement.yaml`, `feature.yaml`, and
`architecture.yaml` contain no occurrence of the string. The block is optional in
`check-command-schema.py`. Bears lightly on D2's genesis import and D8's fidelity fixture, which are
specified against the wrong inventory. *Disposition:* correct F2 to "3 of 6".

**M2 — D3. Removing the count pins removes a redundancy, not only a desync class.** Today's `.md`
carries a self-check ("If the schema's `kind: fail` count is not 4, the pair is out of sync: halt")
that lets the model detect a partial read against an independently authored number. D3 has the CLI
print both sides, so the check becomes self-consistent by construction and detects nothing. Under a
single delivery channel with no fallback, that is a real loss the record books as pure gain.
*Disposition:* acknowledge the trade in D3's rationale, or keep one independently sourced pin.

**M3 — D10 misses GI-008.** The waiver covers "the 6 helper scripts"; D6 retires three of them into
the crate. The waiver's own revisit trigger is "a script becomes load-bearing in a shipped flow", and
the amendment policy makes un-waives governance events. *Disposition:* add a GI-008 scope line to the
wave-2 amend run.

**M4 — CI path filter.** `.github/workflows/ci.yml` triggers only on `crates/**`,
`plugins/mochiko/schemas/**`, `Cargo.*`, and the workflow file. Migrations at a new path will not fire
it, so gate 5's snapshot ≡ projection assert silently stops running on migration-only changes.
*Disposition:* add the migration path to the filter as a wave-1 build item.

**M5 — D1 sequence allocation.** D1's rationale leans on multi-seat waves each adding migration files
under disjoint ownership, but an ordered log keyed `NNNN-<slug>.yaml` has a well-known collision
problem when two seats allocate concurrently. `patterns-transport-floor`'s single-writer lane applies.
*Disposition:* name the allocation scheme (content hash, timestamp, or a lead-assigned range) in the
build surface.

**M6 — `${CLAUDE_PLUGIN_DATA}` is absent from the record.** Documentation, live-fetched: "The
plugin's persistent data directory, **which survives plugin updates**. Substituted only in plugin
skills. Use this to reference **installed dependencies**, generated files, or **caches that must
outlive an update**." That is the documented home for D1's projection, and it weakens D4's premise
that the plugin cannot carry an installed tool. *Disposition:* add it to F9, name it as the
projection's home in the build surface, and note in D4 whether it changes the distribution ruling.

**M7 — F9.4 over-confidence.** "Marketplace install (git clone) preserves executable bits" is stated
as fact; the fetched plugin reference does not say it. Moot under D4, but the F9 block's value is its
calibration. *Disposition:* mark it unverified or drop it.

**M8 — Unnamed access loss.** D4 prices two-step onboarding, version skew, a crates.io name, a tap,
and Windows. It does not name the class that loses mochiko entirely: users who cannot install
developer tooling — locked-down corporate machines, no admin rights, no cargo, no brew. Under
no-fallback that is total loss of the plugin, not degraded service. *Disposition:* one line in D4's
priced consequences, accepted eyes-open.

---

## Part 4 — Killed candidates

| Candidate | Why killed |
|---|---|
| F1 crate total 599 contradicts its own 588-line itemization | `Cargo.toml` is 11 lines; 599 is the correct crate total. Record right, reviewer wrong. |
| SQLite as a committed binary blob kills git review and worktree merges | D1 rules it out for exactly that reason. The record's own finding. |
| Diffability of rule content is lost | D1's migration-log-as-truth preserves diff, blame, and PR review in full. |
| "Content" is an undefined noun | D2 defines scope file by file: 50 schema files plus the sidecar, prose excluded. |
| The provenance sidecar's D16 posture is unhandled | D2 folds it in and keeps the never-shipped line by build profile. |
| Store corruption is a single point of loss | The projection is rebuildable from the log at any time (D1). |
| Blast radius is ungestured | D9 carries the counts and a six-wave plan; 36 re-points named. |
| Resolving `extends:` stubs at render inflates delivered tokens | Today's `.md` instructs reading `common.yaml` whole; resolution is a net reduction. |
| `bin/` cannot ship through claude.ai organization settings, so org installs lose the binary | Moot: D4 ships no binary to anyone. |

---

## Part 5 — Fitness (`RECORD-FITNESS.md`)

| Item | Verdict | Evidence |
|---|---|---|
| Self-contained | PASS | F1–F11 ground facts, six constraints, ranked driver table, ten decisions with rationale, Q1–Q12 trail, build surface, evidence honesty, open threads. A cold reader can reconstruct the session. |
| Decisions attackable | PASS | Every decision carries a Rationale and a named Rejected line. No bare assertions. |
| Decision trail present | PASS | Q1–Q12 record options and answers; D3 shows its Q8 amendment; D4 shows the recommendation declined. |
| Confidence marks honest | PARTIAL | D3/D8/D9 split choice from efficacy; D4 `Contested` with inferred reasons marked `Assumed`. But D10 is `Confident` over a five-limb bundle whose GI-019 limb was never separately tested (I9), and D1 is `Confident` while its store engine is unmade (I4). |
| Rejected roads recorded | FAIL | Per-question alternatives are named, but the maintainer-side-only road — which serves both `high` drivers at zero governance cost — appears in no menu and no Rejected line (C3). |
| Honest about the open | PASS | "Evidence honesty" states n=0 everywhere that matters; eight open threads listed; D4's inferred rationale flagged for user correction. |
| Provenance stated | PASS | Lead, date, worktree and commit, prior-session relations, reviewer sizing at Q12. Review section not yet present — expected at `Status: open`. |

One unchecked item blocks `ready`.

---

## Part 6 — Tally and status

**21 raised, 21 survived** — 3 Critical, 10 Important, 8 Minor. **9 candidates killed** in the cold
read before reporting.

**Recommended status: `critical-gaps`.**

Three independent triggers, any one of which is sufficient under the protocol: a Critical coverage
gap where the cheapest road serving both high-ranked drivers was never on a menu (C3); a broken
load-bearing safety claim, where the documented `disableSkillShellExecution` and synced-skill
contexts defeat the halt clause that carries the entire no-fallback posture (C1); and a ratified
governance principle that the tree contradicts by design for four waves (C2).

This is a strong record, and the verdict is not a judgment on its craft. Its ground-fact discipline
is unusually good — 21 of 22 sampled repo claims verified exact, the external block honestly
disclosed with its unknowns marked — and D1 independently arrived at the best available store shape
before this reviewer's blind map named it. The gaps are concentrated in three places: what the
platform does when `!` execution is unavailable, the sequencing between governance text and tree
reality, and one road that was never put to the user.

**Status is input. The lead owns the clearing verdict.**

---

*Reviewer floor observed: no edit was made to `record.md`. Findings enter through the lead's pen.
The `EXTERNAL-CLAIMS.md` source re-read clause is owed on C1, I3, and M6 before those findings
survive.*

---

## Delta-check (bounded verify round)

Scope: the folds of lens A's own 21 survivors against the folded `record.md` (953 lines,
superseding the 608-line version cold-read above). No fresh cold read, no new blind map, no new
angles. New surface raised only where a fold introduced a contradiction or a factual error.

**Result: 16 of 21 folds CLEAN. 5 defects — 3 blocking, 2 nits.**

### Clean folds (16)

| Finding | Fold | Verdict |
|---|---|---|
| **C3** | Problem section gains "Roads rejected at the frame": the null road and the maintainer-side-only road recorded rejected with reasons, plus the attribution — "the governance cost of this design … is bought by driver A at `medium` rank on the user's explicit instruction, not by the two `high` drivers". D3's Rejected line names the road too. | CLEAN — applied in full, attribution included. |
| **I1** | Split out as its own **D11**, `Assumed` until the wave-2 validator, routed through the amend run beside GI-020, argued against the ledger's Fail row and rationale (both now quoted in F7), fresh admission ruling stated, limb (ii) ruling the landing-ritual carve openly rather than denying the pipeline. | CLEAN — all three limbs; the rationale concedes the asymmetry had "no defense". |
| **I2** | D9 wave 0 rebuilt against D4: plugin-`bin/` probe dropped, `${CLAUDE_PLUGIN_ROOT}` closed as documented (F9.2), `PATH` visibility minted as **the numeric abort**. | CLEAN — matches the recommendation exactly. |
| **I3** | D4 states the Rust-toolchain compile plainly, declares PowerShell-only Windows **unsupported**, F9.3 carries the verbatim failure string, D8's CI matrix re-scoped to Windows-with-Git-Bash. | CLEAN. |
| **I4** | D1 rewritten: in-memory replay, cache on measured need in `${CLAUDE_PLUGIN_DATA}`, SQLite deferred with `rusqlite` named, criterion stated, measured at pilot. The note concedes the earlier wording rode the ask's example "without a requirement paying for it"; `build-vs-off-the-shelf` D4 cited as making it the user's ruling. | CLEAN — exceeds the recommendation. |
| **I5** | D6 names the steady-state audit unit (`mochiko:validator` on the migration file plus the regenerated derived-view diff) with five criteria, and makes readable derived views **a constraint, never builder's room**. Criterion 3's survival limb preserved into the validator's hard set. | CLEAN — the survival-limb catch goes beyond what I raised. |
| **I6** | D8 splits deterministic asserts (gate, N=1) from the behavioral one (metric, N=3, pre-registered bar, never gating); cost priced at ~108 runs full / ~5 smoke; the noise finding cited as the reason. | CLEAN. |
| **I8** | Wave 1 keeps pipeline machinery only ("no public publish yet"); first crates.io publish and tap move to wave 2, "after the amend run, never before it"; D10.4's discharge follows. | CLEAN — the split option, as recommended. |
| **I9** | D11 carries its own `Assumed`; D10's heading records the split; Evidence honesty adds per-heading split marks. | CLEAN. |
| **I10** | D9 mints pilot abort criteria (read-back below bar, or read cost above the F3 baseline), a trip halting waves 4–5, and states the post-wave-2 reversal cost for the user to rule on eyes-open. | CLEAN. |
| **M1** | F2 corrected to "3 of 6" with the schemas named; D8's genesis fidelity re-scoped to "the tombstones of the three schemas that carry them". | CLEAN. |
| **M4** | F1 states the filter; D9 wave 1 and the build surface both add `migrations/**`. | CLEAN. |
| **M5** | D1 rules lead-assigned sequence ranges per wave, a seat never self-allocating, content hash in the header, collisions a validator rejection. | CLEAN. |
| **M6** | F9.2 carries `${CLAUDE_PLUGIN_DATA}` verbatim with the survives-updates quote and the ephemeral warning on `${CLAUDE_PLUGIN_ROOT}`; D1 names it the cache home. | CLEAN — exceeds the recommendation. |
| **M7** | F9.4 marks executable-bit preservation *unverified (moot under D4)*. | CLEAN. |
| **M8** | D4 states the access-loss class as "total loss, not degraded service — accepted eyes-open". | CLEAN. |

### Defects

---

**Defect 1 — BLOCKING. C1's fold. Three surfaces describe three different behaviors for the
policy-disabled environment.**

The re-key to positive confirmation is the right fix and stronger than what I recommended. But the
same run is now described three ways:

- **D3:** the `.md` "requires the version-triple line in its exact shape and **halts on anything
  else** — an error, an empty block, the policy placeholder `[shell command execution disabled by
  policy]`, or a file-path-plus-preview stub … never proceeding".
- **D7(b):** the hook injects the rendered rules as context, "a second deterministic delivery
  beside `!`, **so a policy-replaced `!` line still delivers**".
- **D8 variant:** "`disableSkillShellExecution` set → **the halt fires on the placeholder and the
  hook delivers or blocks**".

D3 says halt. D7(b) says proceed on hook-delivered rules. D8 asserts both in one clause. The
ambiguity is real, not verbal: the halt is stated once as positive confirmation — which the hook's
injected version triple satisfies, so the run proceeds — and once as an enumeration of negative
triggers naming the placeholder, so the run halts regardless of the hook. D10.1 declaring these
environments unsupported points at halt, which would make D7(b)'s parenthetical vestigial.

*Repair (one ruling, then one sentence each):* state the precedence. Either hook-delivered rules
satisfy positive confirmation — D3 then stops listing the placeholder as a halt trigger and keys
only on the triple being absent from **both** channels — or the placeholder always halts, D7(b)'s
"still delivers" parenthetical is struck, and D8's variant asserts halt alone.

---

**Defect 2 — BLOCKING. C2's fold. The transition clause opens one wave after the amendment it
exists to make true.**

D10.1's clause reads "*from the wave-3 pilot until the wave-6 landing, unconverted primitives read
the derived snapshot files shipped in the plugin*". D9 lands the amend run at **wave 2**, "before
any `.md` points at the CLI".

At the wave-2 bump the ratified principle says the plugin "depends on the separately installed
`mochiko-cli` for every command and skill" — false, nothing is converted — while its transition
clause does not yet apply by its own words. That is C2's failure recurring in a one-wave window, at
exactly the moment `validation-constitution` and GI-012's gates grade the amended text.

*Repair:* change the window's start from "from the wave-3 pilot" to ratification — "from this
amendment's landing until the wave-6 landing". The expiry condition and Testability rows are
already right.

---

**Defect 3 — NIT. I7's fold. The hook floor's wording contradicts the hooks it governs.**

D7's floor reads "every shipped hook carries a 5-second `timeout` and is fail-open by platform
design (F9.1) — **a presence check must never be able to break a session**". D7(b) is a presence
check that, on absence, "exits 2 with the install line — the expansion or the skill invocation is
**blocked**".

The underlying semantics are consistent — fail-open covers a hook that malfunctions or times out,
not one that runs and returns a deliberate verdict — but the sentence does not distinguish them,
and a reader meeting the floor before D7(b) reads a contradiction.

*Repair:* "a hook that malfunctions or times out must never break a session; a hook that runs and
finds the dependency absent blocks by design."

---

**Defect 4 — NIT. M2's fold. "Replacement" overstates what the version triple covers.**

D3 now reads: "at the price, acknowledged, of the independent-number self-check today's `.md`
carries (review A-M2): **the version triple is the replacement confirmation**". The acknowledgment
is exactly what I asked for; the claim attached to it is not equivalent. The count pin let the model
check that the rule set it received was **complete** against an independently authored number. The
version triple confirms that **delivery happened and versions match**. Completeness is no longer
independently checkable from inside the run — the loss, correctly booked and then partly un-booked
by the word "replacement".

*Repair:* "the version triple confirms delivery and version match; completeness is no longer
independently checkable in-run, and is carried instead by the crate's genesis-fidelity and
view ≡ replay tests."

---

**Defect 5 — BLOCKING. M3's fold, and a correction to my own finding. GI-008's waiver does not
cover the three retired scripts, so it does not narrow.**

D10.7 rules: "**GI-008 narrows at the amend run:** the waiver's scope drops the three retired
scripts and keeps the remaining three (`validate-*`/`check-artifacts.py`, `detect-stack.sh`)".

Verified against the tree and the ledger. The waiver's text is "FLOOR-TEST as applied to the **6
helper scripts (1 bash, 5 python)**". The six matching that description are the in-plugin skill
scripts:

```
plugins/mochiko/skills/analysis-codebase/scripts/detect-stack.sh              (1 bash)
plugins/mochiko/skills/review-plan-artifacts/scripts/check-artifacts.py
plugins/mochiko/skills/patterns-entity-modeling/scripts/validate-model.py
plugins/mochiko/skills/patterns-api-contracts/scripts/validate-openapi.py
plugins/mochiko/skills/authoring-requirements/scripts/validate-requirements.py
plugins/mochiko/skills/authoring-user-stories/scripts/validate-user-stories.py  (5 python)
```

The three scripts D6 retires — `scripts/check-command-schema.py`, `scripts/check-skill-schema.py`,
`scripts/find-similar-rules.py` — are repo-level and were **never in the waiver**. They post-date it
(the waiver ratified 2026-08-06 at governance v1.0.0; the first checker landed 2026-08-26 at
v0.92.0), and its rationale does not fit them: it reads "test/lint infrastructure absent", while all
three ship negative-test matrices totalling 268 probes.

So the fold asserts a scope change to a ratified waiver based on a misidentification of its members,
and schedules that change into the wave-2 amend run where `validation-constitution` grades the
trace. It would not close. It also leaves the surviving set wrong: six, not three.

**My M3 carried the same misreading and is withdrawn as stated.** The correct residual is narrower
and still real: the three repo-level checkers sit outside GI-008 and inside GI-019's advisory
carve-out (F5 states this correctly), and D6 moves their function into the admitted binary as
**rejecting**, not advisory — a transition argued at D11 limb (ii), needing no waiver change.

*Repair:* strike D10.7 and Constraint 7's narrowing claim. Replace with: GI-008 is untouched — its
six in-plugin skill scripts are unaffected by this design; the three retired repo-level checkers
were never in its scope, and their move from advisory to rejecting is governed by D11 limb (ii). If
the waiver's membership is judged genuinely ambiguous, the amend run resolves the ambiguity rather
than asserting a narrowing.

---

### External-claims re-read (C1, I3, M6)

**Faithful on all three, and extended.**

- **C1** — F9.3 carries `disableSkillShellExecution` with the plugin scope, the literal placeholder
  string, the managed-settings clause ("where users cannot override it"), Cowork, and synced
  skills. More complete than my citation.
- **I3** — F9.3 carries the `shell` key and the verbatim failure string for `shell: bash` without
  Git Bash. Faithful.
- **M6** — F9.2 carries `${CLAUDE_PLUGIN_DATA}` with the survives-updates quote, adds the
  `~/.claude/plugins/data/{id}/` path and the deleted-on-uninstall fact, and pairs it with the
  ephemerality warning on `${CLAUDE_PLUGIN_ROOT}`. Faithful and extended.

Two facts neither lens carried were found by the lead's re-read and folded correctly: **hooks are
fail-open** (F9.1, into D7's floor) and **oversized `!` output arrives as a file path plus preview
rather than truncated text** (F9.3, into D3's halt list and D9's probe (e)). The F9 header's
`verified`-unless-marked convention and the Evidence-honesty note that wave 0 re-runs the
load-bearing items are both sound.

One residual honesty note, not a defect: F9.3 correctly keeps "whether the grant is *required* for
`!` to run" as *unverified*, and D3 nonetheless makes the `allowed-tools` grant a scaffold
obligation. That is the safe direction and is probed at wave 0(a). No action.

### Delta-check status

**NOT CLEAN — 3 blocking, 2 nits.** All five are repairable in place without reopening a ruling:
defects 1 and 2 need one lead ruling each, on precedence and on window start; defects 3, 4, and 5
are sentence replacements, defect 5 striking a fold rather than adding one. No fold reversed a
disposition, and no repair reopens a user ruling from Q13.

---

## Delta-check round 2 (repairs only)

All five round-1 defects verified repaired against the repaired record.

1. **Repaired.** D3's "Precedence across channels" clause rules the confirmation satisfied by the
   version triple "from whichever channel delivered it"; D7(b) now states the hook's injected
   context carries "its first line the same version triple"; D8's variants split into
   *placeholder + hooks enabled → proceeds on the hook* and *placeholder + hook-disabled → halt*.
   The three surfaces now describe one behavior.
2. **Repaired.** D10.1's clause opens "*from ratification at wave 2 until the wave-6 landing*",
   with the reason stated inline. The wave-2 window is closed.
3. **Repaired.** D10.7 rewritten to GI-008 untouched, naming all six skill-shipped scripts and
   the three repo-level checkers' post-dating (v0.92.0 · v0.99.0 · v0.100.0), with retirement
   covered by D11's admission. Constraint 7 agrees. A-M3 recorded withdrawn in the disposition
   table.
4. **Repaired.** D7's hook floor now separates "when it runs, absence blocks by exit 2 (by
   design)" from "when it cannot run or times out, the platform is fail-open".
5. **Repaired.** D3's rationale reads "the version triple confirms **delivery**, not completeness
   … the independent self-check is lost and booked as a loss, not a gain".

### Residual (1, nit)

**R1 — Build surface, Wave 2 line, stale echo of the superseded D10.7.** It still lists the amend
run's contents as "GI-002 discharge naming hooks · **GI-008 narrowing** · unsupported environments
named". D10.7 now rules GI-008 untouched and says "the amend run records this as a note, not a
change". D9's own wave-2 sentence is fine — it names GI-008 without characterizing it. Only the
build surface carries the stale word.

*Repair:* "GI-008 note (untouched — D10.7)".

**Round-2 status: NOT CLEAN — 1 nit, no blocking.** All five round-1 defects discharged.
