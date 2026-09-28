# Wave 1 — the joint build of the hook field review and the delta-files retirement (crate)

**Rulings:** `record.md` D1–D9 as review-amended (accepted 2026-09-23) · `delta-files-vs-direct-baseline-edits/record.md`
D1–D7 as review-amended (accepted 2026-09-24) · the joint-build seams R1–R4
(`.mochiko/decisions/2026-09-29-joint-hook-delta-build-seams.md`, ruled 2026-09-29) · governance v3.2.0 (AM-5:
GI-019 already re-worded for this gate, its build-state line strikes at the wave-1 + wave-2 bump; the GI-012
exception row covers shipping the hooks before the first publish — R4).
**Build surface:** this record § Build surface 1–4; the delta record § Build surface 1–3, which rides waves 2–4.
**Lead:** the session lead. Produces nothing on a governing surface; owns the branch, the wave plans,
`build-log.md`, sequence allocation, the seat and review dispatch, the ceremony, and the gates.
**Floors in force:** sound loop · transport floor · model tiering · the primitive-edit ceremony
(`.claude/rules/mochiko/primitive-edits.md`) · the crate rules (`.claude/rules/mochiko/rust-cli.md`).
**Branch:** `joint-hook-delta` off `main` at `be46e16` (plugin 0.115.0, binary 0.2.0, migrations through 0023).
**Targets:** crate `0.3.0`, grammar range 1..2, no `mochiko-cli-v*` tag (R4); one plugin bump, `0.116.0`, at the
close of wave 3 (MINOR: homes, rules and templates change). Under the held-bump rule (AM-5) nothing merges to `main`
before that bump's gates are green; waves 1–3 merge once.

## 0. The joint wave map

| Wave | Hook field review | Delta files | Seams |
|---|---|---|---|
| 1 crate | build surface 1, all of it | nothing directly; its D1 needs the per-entry budget kind built here | R4: version bump, no tag |
| 2 census + homes + rule supersessions | build surface 2 and the one census table | build surface 1 (sweep first, then supersessions, home sets, entry grammar) | seams 2–5 and the smaller one land in the census table |
| 3 rule rewords + prose + ceremony | build surface 3, less the ledger (done at AM-5) | build surface 2 | R1 in `impl.artifact-home`; R2 in the report rules; the smaller two put to the user at plan approval |
| 4 kinako, after the merge | build surface 4 | build surface 3, D5 (i)–(v) in order | R3 checked first |

Waves 2 and 3 are split only because the census table must be ratified by the user before the rule rewords that
cite its numbers are written. Each gets its own plan file at the previous wave's close.

## 1. Seats and ownership (wave 1; strictly disjoint write sets)

| Seat | Persona (tier) | Owns (write set) |
|---|---|---|
| S1 | `staff-engineer` (default `opus`) | `crates/mochiko-cli/src/home.rs` · `src/conform.rs` · `src/hook.rs` · `src/cli.rs` · `src/render.rs` (the home view only) · `src/migration.rs` and `src/validate.rs` (grammar 2 only) · `Cargo.toml` (version + comment) · `tests/home.rs` · `tests/conform.rs` · `tests/hook.rs` · `tests/cli.rs` · `tests/validate.rs` · `tests/migration.rs` · new fixtures under `tests/fixtures/` named in its plan |
| S2 | `staff-engineer` (default `opus`) | new `crates/mochiko-cli/src/shell.rs` · `src/lib.rs` (the one `pub mod shell;` line) · new `tests/shell.rs` · `.mochiko/brainstorms/hook-enforcement-field-review/reports/w1-shell-census.md` |
| G1 | plain `general-purpose`, `model: opus` | reads only; writes `reports/w1-code-review.md` |
| P1, P2 | fresh `staff-engineer` peers | grade S1's and S2's plans per `mochiko:review-seat-plan`; write nothing |

**Order.** S1 and S2 plan-only in parallel, then P1/P2 grades, then the lead's GO. S2 executes `shell.rs`; S1
executes §2 in parallel, except the shell integration. When S2 has landed, S1 swaps `hook.rs`'s call sites to
`crate::shell` and deletes the old tables from `hook.rs`. Then G1 reviews; a fix round goes to the same seats.

**Transport.** One writer per file; the S1/S2 interface is fixed here (§3.1); the lead relays every message; no
mesh. Mechanical sub-tasks (matrix cases, fixture files) may go to a disposable `sonnet` worker per
`mochiko:patterns-model-tiering`, read back by the seat before they count.

**No seat installs the built binary.** The running session's hooks call the `mochiko-cli` on `PATH` (0.2.0). A
0.3.0 binary installed mid-build would put this repo's own `.mochiko/` under a closed world whose homes wave 2 has
not declared yet. Seats run `cargo run --` or `target/debug/mochiko-cli`.

## 2. S1 — resolution, the closed world, the run folder, per-entry budgets, the dry run

**2.1 Resolve against the tree root, never the cwd (D7 as amended at S3).** Today `relativize` strips the payload's
`cwd` from the path, so the same file is gated for one seat and not another (F15). New rule: join a relative path
to `cwd`, normalize it, then walk up to the nearest ancestor holding `.git` (a directory, or the file a worktree
carries). Only `<root>/.mochiko/` is a home tree; a `.mochiko/` nested deeper under that root (fixtures, eval
workspaces) is never a home. A worktree resolves to its own homes. A path in another tree resolves against that
tree's root. A path whose tree has no `.git` at all (an `impl.cold-verification` snapshot) has no home tree and
resolves outside (delta-check N4). The root operating docs are unchanged (not watched). The ancestor walk is one of
the reads GI-019's "What the gate reads" paragraph lists as widened at AM-5; S1 reads that paragraph and builds
exactly what it names. `check` and `home` share the resolver. Matrix: root-relative · relative from inside a
worktree · absolute · nested fixture · the run folder · another tree · a tree with no `.git`.

**2.2 `home` resolves a directory (S4).** A path naming a home's directory, with or without a trailing slash,
renders that home, instead of the parent's "NOT a declared deliverable". One case per `*.artifact-home` render
target (seven; S1 lists them from the log with `grep -n artifact-home plugins/mochiko/migrations/*.yaml`).

**2.3 The closed world under `.mochiko/` (D3).** Inside `<root>/.mochiko/`, a path that resolves to no home
becomes a deny for `Write`/`Edit` and for parsed shell writes. The reason names two routes: a declared deliverable
or `reports/` file of the nearest home, or, for raw output, the run folder by absolute path, stated as ephemeral.
`home` prints the same. The undeclared-sub-directory reason drops "A new sub-directory takes a migration … and a
plugin release" in favor of the run-folder route (D3's last sentence). Outside `.mochiko/` nothing changes: the D9
sniff stays. The first-touch amnesty never covers a path in no home (unchanged). The ledger's "Reach of the gate"
paragraph (AM-5) is the text this wave must meet: S1 walks it clause by clause, and any clause the binary does not do
today and this plan does not name (for example the sniff's "template's `## Header` signature" limb) goes to the lead
at plan time, never built or dropped silently. Tests declare their own homes in
fixtures; the real census is wave 2's, and the closed world only reaches a consumer in the same bump.

**2.4 The `runs/<run-id>/` home (D4 as amended, controls 1–4).** New home fields (grammar 2, §2.8) mark one home as
the raw-output home. Its controls: (1) location, declared at wave 2; (2) the run-key name, a new path token
`<run-id>` (the token vocabulary is the crate's, `home.rs` module docs), in the form ruled for OQ2 (see § 7); (3) the
ignore guard: a write under `runs/` is refused unless `<root>/.gitignore` has a line matching `.mochiko/runs/` (S1
states the accepted spellings: with and without the leading `/` and the trailing `/`); (4) the content rule: a `.md`
file opening with report frontmatter (the existing sniff test, nothing new) is refused under `runs/` **and** under the
four declared sub-directories without home documents: `prototype/`, `inputs/`, `research/`, `referents/` (V2).
`implement-log.md` is allowed by name (V7). Shape and size are otherwise unchecked. **Main tree only (V3):** a write
to a worktree's own `.mochiko/runs/` is refused, and the reason names the main tree's run folder by absolute path,
read from the worktree's `.git` pointer file (`gitdir: …/.git/worktrees/<name>`), the third read GI-019 names.

**2.5 The per-entry budget kind (D2, OQ1).** A deliverable form `entries`: `entry_heading` (`##` or `###`) and
`entry_max_lines`, each entry counted from its heading to the next heading at the same or a higher level. The
census (wave 2) decides how each store uses it, so the kind also offers, as optional fields: `section_max_lines`,
which bounds the text of a `##` section outside its entries (seam 3), and `entry_exempt_fields`, a list of field
names (`Lifecycle`, `Raised`, `Weighed`) whose `**<Name>:**` lines are not counted (seam 4). The amnesty applies per
entry with the existing mechanism: the fault key is `size:entry:<heading text>`, so a growing edit to an entry
already over budget denies (V9) and a non-growing one passes. A renamed entry is a new key; the reason says so.

**2.6 The direct dry-run form (S15).** `mochiko-cli check --path <p> --content -` reads the body from stdin and
answers with the hook's verdict (exit 0 allow, 4 deny) and its reason text, relative paths resolved from the process
cwd, so a seat need not hand-build a `PreToolUse` payload. `--hook-json -` is unchanged.

**2.7 Deny and `home` texts (D3/D4).** Every deny inside `.mochiko/` that could be raw output names the run
folder's absolute path. No render output shape changes (head and tail lines stay byte-identical, `rust-cli.md`
release rule); a needed change is a stop.

**2.8 Grammar 2.** `home.rs` states that a later migration adding a home field must bump the grammar, because an
older binary ignores unknown keys and would treat the run folder as an ordinary home. So `GRAMMAR_RANGE` becomes
`(1, 2)`. Grammar 2 is grammar 1 plus the fields of 2.4 and 2.5. `migrate validate` checks them: `entry_heading`
only `##`/`###`; the raw-output fields on at most one home; `entry_max_lines` required with `form: entries`. A
consumer on 0.2.0 meeting a grammar-2 log halts at first use and says why, which is the dependency halt working
(GI-020). Crate version `0.3.0`, with a `Cargo.toml` comment in the file's style.

## 3. S2 — the shell leg reads write positions only (D5 as amended at S12, V1/N2)

**3.1 Interface.** `pub fn write_targets(command: &str) -> Vec<String>` and
`pub fn powershell_write_targets(command: &str) -> Vec<String>` in `src/shell.rs`, the same signatures as today's
private functions in `hook.rs`, so S1's integration is a call-site swap. The tokenizer moves with them.

**3.2 Rules.** A declared-home path is a write target only in a write position: the right side of `>`, `>>`, `>|`
(heredoc included); every argument of `tee`; the last argument of `cp` and `install`; both ends of `mv` (a removal
from a home stays a deny); the file arguments of `sed -i` and `perl -i`, only when that `-i` is in the same
command's own argument run; `dd of=`; the PowerShell cmdlets as today. Every other position is a read: a `grep`,
`sed -n`, `wc` or `cat` operand, the source of `cp`, a `git show` object spec. The tokenizer splits separators glued
to a word (`file;`, `file|`, `file&&`), which is how a `-i` or `tee` elsewhere in a line reached a home path (S12).
The three evasions the prior build disclosed (`cd <home> && … > x.md`, a path in a variable, a relative write from a
cwd inside a home) stay open and disclosed. The `runs/` carve (non-`.md` targets under `runs/<run-id>/` allowed) is
a decision, so it is S1's, in `hook.rs`; `shell.rs` returns targets only.

**3.3 The matrix from the real commands.** S2 extracts, by script, every `Bash` call denied by the gate in the kinako
transcripts of runs 3 and 4 (main threads and every subagent sidechain under
`~/.claude/projects/-Users-deepeshadmin-Documents-GitHub-kinako*/`; a deny is a tool result carrying "a second deny
on this path halts"), pairs each with its command, and classifies it against F3's shapes. Expected: 21 (13 false,
8 true). Added: the field review session's own hits (F3 addendum) and this session's
`sed -n 276,312p .mochiko/memory/governance-ledger.md; echo ----; grep …` (a read denied on the glued `;`). Each
becomes one case asserting its targets. `reports/w1-shell-census.md` records the session id, the command, the verdict
then and the expected verdict now. A count other than 21 is a stop, never a quiet re-count.

## 4. The code review (G1)

One plain seat (`general-purpose`, `model: opus`): the independent non-author review `rust-cli.md` requires. It reads
the diff and the two plans, runs the four crate layers itself and quotes their tails, and walks each matrix against
the clause it claims (D3, D4 controls 1–4, D5, D7, S4, S15, V1–V3, V9, N4). Verdict and findings go in
`reports/w1-code-review.md`. Bound: `common.gate-loop-bound` (one fix round and one re-review by the same seat; a
second FAIL halts to the user).

## 5. Wave-1 gates (lead)

`cargo test --all` · `cargo fmt --all --check` · `cargo clippy --all-targets -- -D warnings` · `cargo audit --deny
warnings` · `target/debug/mochiko-cli migrate validate --report --plugin-root plugins/mochiko` with 0 rejecting on
today's log · views ≡ replay unchanged (no log change in this wave). No `plugin.json` change and no contract-suite
run in wave 1: the suite runs once at the wave-3 bump, against the branch binary. `build-log.md` opens with this
wave's entry.

## 6. Waves 2–4, outline (each gets its own plan at the previous wave's close)

**Wave 2 — census, homes, rule supersessions.** (a) The delta record's full-text sweep of the log ("delta" · "fold" ·
"appliable" · "in place" · "in-flight" · "FEAT-XXX"), audited against its D6d list, first. (b) The census of every
`.mochiko/` path mochiko writes, taken from the write sets the primitives will have after the delta rewrites, with
both trees as a cross-check. (c) **One census table, ratified by the user:** declared sets for every directory
(`archive/` including the archived ledgers, seam 2 · `runs/` · `schema-views/` · `strips/` · `decisions/` ·
`memory/`) · the feature, epic and lane home sets (delta D4; the epic `implement-log.md` withdrawn) · each store's
entry heading and budget, sized on the baselines after delta D5 (seam 3) · the bound on text outside entries (seam
3) · whether marker lines count (seam 4) · `quickstart.md` and `design/design.md` (seam 5) · the `Lifecycle:` and
build-raised fields (delta D2/D3b) · the run-id form (OQ2). (d) Migrations from `0024`, grammar 2: the home
documents; the two floors superseded by new ids; the fail rules re-keyed; the D6d rule sites; `lifecycle-statuses`,
`orphan-rule` and the spine template.

**Wave 3 — rule rewords, prose, ceremony.** `impl.artifact-home` (the log to the run folder; R1's boundary; no
`DECISIONS.md` row per checkpoint) and the other six render targets · R2's base-commit lines in the sufficiency and
final-validation report rules · `testing-end-user`'s scratch path to the run folder, plus the dry-run rule · the
`SubagentStart` and `SessionStart` lines (the latter lists every `runs/*` folder) · the run-close step (OQ5) · the
skills the delta record's D6d lists · strips · the gate audit (`mochiko:validation-primitive-edit`) · `CHANGELOG.md`
citing the AM-5 exception row (R4) · `plugin.json` `0.116.0` and `marketplace.json` · the contract suite green
against the branch binary · the GI-019 build-state line's text-vs-build check, run by the lead. The report
commit-citation question (the smaller two) is put to the user at this wave's plan approval.

**Wave 4 — kinako, after the merge.** R3 first: no implement run open. The maintainer installs crate 0.3.0 (the
`rust-cli.md` break-glass) before kinako upgrades. Then the field review's full-inventory pass (evidence trees
deleted, the legacy directories declared or moved, the 47 pointers) and the delta record's D5 (i)–(v) in order, each
write graded by a non-author seat reading the diff, and the watch re-measured with its displacement metrics.

## 7. Ruled at this plan's approval (2026-09-29, user: GO)

- **OQ2 — the run-id form (R5).** `<owner>-run<n>`, the owner being the delta record's D2 key (`FEAT-001-run5` ·
  `EPIC-003-run1` · `lane-auth-fix-run1`), created by the lead at run-open. S1 builds the `<run-id>` token to exactly
  this shape: a `FEAT`/`EPIC` id (the existing id shapes) or `lane-<slug>`, then `-run`, then one or more digits.
- **OQ5 — who deletes the run folder (R6).** The lead's own removal as a landing step after acceptance; no new CLI
  subcommand.

## 8. Stops (halt to the user)

A second FAIL at the code review · a clause that cannot be built as ruled · the shell extraction not 21 · any change to
a render's head or tail line · a need to install the built binary mid-build · OQ2 unruled when S1 reaches §2.4 · a
`migrate validate` rejection on today's log under the new binary.
