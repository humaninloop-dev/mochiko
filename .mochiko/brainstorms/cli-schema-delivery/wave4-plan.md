# Wave 4 — the five remaining commands (lead-drafted referent)

**Ruling home:** `record.md` D3 as amended, D7, D8 as amended, D9 ("Wave 4: the remaining five
commands"), the wave-3 section (Q-A branch B measured, Q-B no hand pin, the V2 F1 follow-up: a
section-reword op plus a migration for the six `fail-conditions` intents), governance v3.0.1
(`primitive-edits.md` criteria 1/3/11 as amended for converted commands; GI-020's revisit trigger
live per converted primitive). **Wave open:** on the user's "wave 4" (2026-09-04); the lead's
standing assumptions are in §7 and hold unless the user redirects. **Floor:** sound loop tripped
— seats produce on lead-approved plans, fresh validators review, the user accepts; transport
floor: disjoint file sets per seat (as at wave 3), single writer per file.

**Done condition (fixed):** all six commands fire from the migration log with no schema file read
— `architecture` · `feature` · `implement` · `setup` · `specify` re-pointed in the wave-3 shape;
the six `fail-conditions` intents no longer claim a hard-coded count (a `reword-section` op in
the grammar, migration `0002`, the six shipped snapshot files carrying the new line); the
contract suite covers every converted command with the read-back metric at the pre-registered
bar; the per-primitive abort criteria evaluated and stated for each of the five; every audit
PASS; strip entries on all five; the wave lands as `plugin.json` 0.105.0 with the landing ritual
complete and the four D8 layers green (a SKIPPED suite blocks the bump).

---

## 0. Measured before the wave (lead, host, binary 0.1.0, 2026-09-04)

| command | prefix | rendered (7 blocks, bytes) | largest block | raw baseline (`<cmd>.yaml` + `common.yaml`) | delta | `kind: fail` | `class: floor` |
|---|---|---|---|---|---|---|---|
| architecture | `arch` | 18,336 | `arch.sec.boundaries` 4,761 | 23,026 | −20 % | 1 | 22 |
| feature | `feat` | 17,113 | `feat.sec.tools` 5,354 | 21,020 | −19 % | 1 | 13 |
| implement | `impl` | 35,178 | `impl.sec.tools` 15,617 (52 % of the ceiling) | 44,266 | −21 % | 15 | 34 |
| setup | `setup` | 16,050 | `setup.sec.tools` 5,289 | 20,245 | −21 % | 6 | 18 |
| specify | `spec` | 19,223 | `spec.sec.tools` 5,965 | 23,434 | −18 % | 9 | 16 |

Every block is under the ≈ 30,000-byte inline ceiling; `implement`'s whole render exceeds it,
which is exactly why D3 chunks per section. The floor-id sets per command are listed in §4 (the
pre-registered read-back expectations). `views emit` output is **not byte-identical** to the
shipped snapshot files (comments and layout differ; the CI check is semantic), so the six intent
lines are applied to the snapshots by hand as one-line edits and the semantic view ≡ replay test
proves them.

## 1. Scope and ownership (three producer seats, disjoint files)

| seat | owns | delivers |
|---|---|---|
| **P1 — crate + log** | `crates/mochiko-cli/src/{migration,replay,validate,cli}.rs`, `crates/mochiko-cli/tests/*.rs`, `plugins/mochiko/migrations/0002-*.yaml` (new), `plugins/mochiko/migrations/README.md` (the op's grammar entry), the six `plugins/mochiko/schemas/<cmd>.yaml` snapshot files (one intent line each) | the `reword-section` op (grammar-1 extension, §2); `migrate stamp <file>` (writes the hash header in place); migration `0002-fail-conditions-intent.yaml` rewording the six intents; the six snapshot lines; tests; docs |
| **P2 — plugin side** | `plugins/mochiko/commands/{architecture,feature,implement,setup,specify}.md`, `.mochiko/strips/{architecture,feature,implement,setup,specify}.md` (append), `README.md` (wording: every command, not "a converted command") | five re-points in the wave-3 shape (§3); ten strip entries; README wording |
| **P3 — contract suite** | `evals/contract/run.py`, `evals/contract/README.md` | per-command delivery and absence cases (§4); per-command read-cost and abort-criteria evaluation; the suite run with evidence |

P2 and P3 may write from their plan approval; P1's migration must land before P3's sandbox runs
(the delivered `fail-conditions` block changes by one line). Nothing else under
`plugins/mochiko/` changes: `hooks/`, `brainstorm.md`, `common.yaml`, `command-labels.yaml`,
the templates, and every skill are byte-untouched.

## 2. P1 — the op, the stamp, the migration, the snapshots

- **`reword-section {schema, id, title?, intent?, note?}`** — at least one of the three; the
  section must be live (a tombstoned or unknown id rejects); `note: null` clears; the section id
  and its rules are untouched. No anchor obligation: intents are not protected content (the D6
  corollary keys on rules). **Grammar stays 1** — the op is additive and no binary has been
  published (the D5 range is `1..1`; a future published binary that lacks the op would reject the
  file loudly, which is the contract). The README's op list gains the entry; the D6 hard-set
  census and the `migrate validate` output are unchanged.
- **`migrate stamp <file>`** — reads the file, writes it back with the correct `hash:` header
  (the `with_hash` helper already exists; this is its CLI). Exit 0; exit 1 on a body that does
  not parse; never touches any other file. The authoring path every future migration uses.
- **Migration `0002-fail-conditions-intent.yaml`** — `sequence: 2`, `intent:` one line, six
  `reword-section` changes on `command/{architecture,brainstorm,feature,implement,setup,specify}`
  ids `<p>.sec.fail-conditions`. New intent text, verbatim per command (runs vs desks):
  - runs (`brainstorm`, `implement`, `setup`, `specify`): `The kind: fail set — any one standing fails the run; the .md Not-done line cites the count this render prints.`
  - desks (`architecture`, `feature`): `The kind: fail set — any one standing fails the visit; the .md Not-done line cites the count this render prints.`
  Stamped with `migrate stamp`; `migrate validate --plugin-root plugins/mochiko` 0 rejecting;
  `migrate status` shows `sequences 1..2 (2 migrations)` and a new state hash (record both).
- **Snapshots:** the same six lines applied by hand in `plugins/mochiko/schemas/<cmd>.yaml`
  (comments, layout, everything else byte-identical); the semantic view ≡ replay test green.
- **Tests, test-first:** op parse (all three optional fields, none present → reject, unknown
  section → reject, tombstoned → reject), apply (title/intent/note set and cleared), render
  golden (the fail-conditions block of `brainstorm` shows the new intent), `migrate stamp`
  (stamps, re-stamps, rejects an unparseable body, touches nothing else), replay determinism
  over two migrations, fidelity fixture untouched.
- **Check:** four layers; `migrate validate` 0 rejecting; the six snapshot diffs are one line
  each (`git diff --stat`).

## 3. P2 — five re-points, ten strip entries

Each `.md` takes the wave-3 shape exactly (`brainstorm.md` at HEAD is the referent; V2's PASS
criteria are the bar):

- Frontmatter gains `allowed-tools: Bash(mochiko-cli *)`; nothing else in it changes.
- `## Identity & Mission` byte-identical.
- `## Rules — load the schema first` replaced whole by `## Rules — delivered by mochiko-cli`:
  the wave-3 halt paragraph with the command name substituted (`mochiko-cli rules <cmd> ·
  section <id> · …` and `mochiko-cli rules end · <cmd> · <id> · <N> rules`), then seven `!`
  lines — `preamble` and the six `<p>.sec.*` ids in schema order, each
  `--plugin-root "${CLAUDE_PLUGIN_ROOT}" 2>&1`. Nothing else in the section. The per-command
  intro phrases of the old block ("before health is surfaced", "before entry gating", …) are
  not carried: the render is in front of the model before the first instruction by construction.
- Adaptive Goal Protocol: steps 1 and 2 byte-identical; step 3 in the wave-3 form, `visit` for
  the two desks and `run` for the three runs:
  > 3. **Not done — default FAIL:** the `kind: fail` rules of `<p>.sec.fail-conditions` — their
  >    count is the `kind: fail` line under `pins` in the preamble block — any one standing fails
  >    the <visit|run>. A fail-conditions block whose end-line count disagrees with that pin is
  >    the delivery out of sync: halt and surface it before closing.
- **Strips:** `.mochiko/strips/<cmd>.md`, two `[v0.105.0]` supersession-by-ruling entries each
  (the Rules block; the hand-pinned count), the wave-3 entries on `brainstorm` as the shape,
  verbatim content from `git show HEAD:plugins/mochiko/commands/<cmd>.md`, consumers assessed:
  none shared. Ruling cites: record D3 as amended, the wave-3 Q-B ruling, `DECISIONS.md`
  2026-09-04, this wave's row.
- **README:** "a converted command" → every command is converted; the sentence about halting
  reads for all six. No other README change.
- **Check before reporting:** with `target/release` on `PATH`, all 35 `!` commands render with
  head and end lines; each preamble's `kind: fail` pin equals its fail-conditions end-line count
  (1 · 1 · 15 · 6 · 9); the D13 checker on each pair cited in the report (findings limited to
  the conversion-expected substitutions).

## 4. P3 — per-command cases and the pre-registered expectations

Generalize the wave-3 cases from `PILOT_COMMAND` to the converted set discovered from the `.md`
files (the same truth source as the hook and `converted-shape`): `<cmd>-delivery` ×3 replicates
and `<cmd>-absence` ×1 for each of the five, in the wave-3 shape (`--max-turns 2`, the probe
argument instructing the `FLOOR:` line, the transcript fetched, no-Read scoped as at wave 3).
`brainstorm`'s six cases, `skew`, `hooks-off`, and `policy` stay as they are: skew, hooks-off,
and policy exercise the mechanism, not the primitive, and are not repeated per command. The host
cases (`hook-input` rows per converted command; `converted-shape`; `render-ceiling`) cover the
five automatically — extend `hook-input` to iterate every converted command's absent and
present rows.

**Pre-registered read-back expectations (from the binary's render, 2026-09-04; bar 3/3 per
command, stated here before any run; `converted-shape` cross-checks each set against the render
and goes red on a difference):**

- `architecture` (22): `arch.dm-health-first` · `arch.dm-converge-goal` · `arch.dm-author-baseline` · `arch.dm-shelf-walk` · `arch.dm-drift-dispatch` · `arch.dm-route-triggers` · `arch.dm-store-integrity-close` · `arch.dm-km-landing` · `arch.dm-close-verdict` · `arch.author-grader-separation` · `arch.truth-user-ruling` · `arch.breadth-invariant` · `arch.floor-precedence` · `arch.na-handled-elsewhere-pointer` · `arch.derived-index-never-hand-maintained` · `arch.drift-empirical` · `arch.no-depth-dial-coupling` · `arch.no-delivery-harness` · `arch.no-silent-store-mutations` · `arch.sound-loop-floor` · `arch.transport-floor` · `arch.fail.no-verdict`
- `feature` (13): `feat.capability-writes-sacred` · `feat.grooming-door-ceiling` · `feat.out-of-remit-hosting` · `feat.growth-door` · `feat.growth-routes-to-specify` · `feat.lane-never-widens` · `feat.no-delivery-harness` · `feat.no-self-graded-writes` · `feat.no-silent-map-mutations` · `feat.sound-loop-floor` · `feat.transport-floor` · `feat.stub-parking` · `feat.fail.no-verdict`
- `implement` (34): `impl.gate-design-checkpoint` · `impl.gate-card-confirm` · `impl.gate-final-acceptance` · `impl.graded-fold` · `impl.author-grader-default-fail` · `impl.baselines-never-in-place` · `impl.deviation-gate` · `impl.constitution-supremacy` · `impl.constraint-challenge` · `impl.attempt-per-grade` · `impl.attempt-exemption-user-only` · `impl.no-progress-stop` · `impl.epic-member-halt` · `impl.gap-rework-bound` · `impl.gates-never-triaged` · `impl.minimalism-advisory` · `impl.lane-never-widens` · `impl.sound-loop-floor` · `impl.transport-floor` · the fifteen `impl.fail.*` (`sufficiency-unrecorded` · `design-skipped` · `card-independence` · `card-unchecked` · `quality-gate` · `no-evidence` · `regression` · `baseline-in-place` · `deviation-unresolved` · `store-landing-incomplete` · `ungraded-fold` · `gap-finding-missing` · `skip-unstated` · `spec-gap-unresolved` · `no-acceptance`)
- `setup` (18): `setup.blind-map-dispatch` · `setup.gate-synthesis-ratification` · `setup.gate-final-acceptance` · `setup.author-grader-default-fail` · `setup.no-git-mutations` · `setup.acceptance-plain-text` · `setup.transport-floor` · `setup.durables-never-deleted` · `setup.governance-region-ownership` · `setup.carve-outs-preserved` · `setup.map-never-overwrite` · `setup.store-ruled-content-never-here` · the six `setup.fail.*` (`pre-ratification-authoring` · `unclosed-trace` · `author-graded` · `floor-category-uncovered` · `no-acceptance` · `no-feature-map`)
- `specify` (16): `spec.pm-recommends-never-selects` · `spec.gate-selection` · `spec.gate-acceptance` · `spec.author-grader-default-fail` · `spec.transport-floor` · `spec.staged-derivation` · `spec.epic-mint-desk-only` · the nine `spec.fail.*` (`blocking-gap` · `intent-unconfirmed` · `map-unread` · `story-unhomed` · `screens-flows` · `selection-unruled` · `premature-map-write` · `self-graded` · `no-acceptance`)

**Abort criteria, per converted primitive (ledger GI-020 revisit trigger):** (1) read-back below
3/3; (2) delivered bytes above that command's raw baseline in §0. A trip on any command halts
wave 5 and returns the posture to the user; the lead evaluates from P3's evidence. The
delivered figure per command is read from the transcript as at wave 3 (blocks + the two hook
lines); latency is emitted per delivery case as at wave 3 and reported as a band.

**Cost:** 5 × 3 + 5 = 20 new sandbox sessions beside the existing 7; no metered spend.

## 5. Checklist

- [ ] P1: op + README entry · `migrate stamp` · migration `0002` stamped and validating · six snapshot lines · tests · four layers green · old and new state hash recorded
- [ ] P2: five `.md` re-points · ten strip entries · README wording · 35 renders checked · D13 checker cited
- [ ] P3: per-command delivery ×3 and absence cases · `hook-input` rows per command · per-command read cost and latency · abort criteria per command · README · full run green with evidence
- [ ] Audits: V1 (crate + migration + snapshots) · V2 (five pairs, one report, a verdict per pair; hooks unchanged) · V3 (suite; independent full re-run)
- [ ] Landing (lead): `plugin.json` 0.105.0 · `marketplace.json` · `CHANGELOG.md` · `DECISIONS.md` row · `BACKLOG.md` · `ROADMAP.md` · record wave-4 section · index · no governance row expected (say so if a rules file moved)

## 6. Seat protocol and review criteria

As at wave 3 (`wave3-plan.md` §7–§8): plan first, lead approves, test-first where a test layer
exists, reports under `wave4-reports/<seat>.md`, no git mutations, three verification attempts
per unit, validators fresh and never the author. V1 adds: the op rejects what §2 says it rejects;
the migration file's hash verifies; the six snapshot diffs are one line each; grammar stays 1
and the README says why. V2 adds: each pair against `primitive-edits.md` criteria 1–11 as amended,
the halt paragraph verbatim against the wave-3 referent with only the name substituted, the seven
`--section` ids matching the schema's set, the `visit`/`run` word right per command, byte
preservation of Identity/Entry/Goal per pair. V3 adds: the five expectation sets in the code
equal §4 verbatim; a per-command bar never gates; every new case keyed to the measured shapes
of wave 3.

## 8. Read-back diagnostic (pre-registered 2026-09-04, user-ruled "land + diagnostic")

**Trigger:** abort criterion (1) tripped on `implement` 1/3, `setup` 1/3, `specify` 0/3 in the
wave-4 run while every deterministic assertion passed and every missed floor rule is present
verbatim, with its `class: floor` line, in the session transcript (lead-verified on
`contract-specify-delivery-88899711` and `contract-setup-delivery-6a5d4a01`). The user ruled: land
wave 4 with the trip recorded, halt wave 5, run this diagnostic to separate recall from delivery.

**Design (fixed before the run; recorded, never gating):** a separate runner
`evals/contract/diagnostic.py` importing `run.py`'s helpers and modifying nothing in it. For each
of the six converted commands, three replicates, sonnet, `--max-turns 2`, the staged plugin as in
`brainstorm-delivery`, with two changes from the delivery probe:
1. **A probe argument every Entry gate accepts.** The instruction text moves out of `$ARGUMENTS`
   into the prompt after the command: the argument is a neutral, gate-valid token per command
   (`implement`: an explicit "delta scope" card path that does not exist, so Entry routes rather
   than validates a feature id; the desks and runs: a one-word topic), and the read-back
   instruction follows the command on the same prompt line.
2. **Two read-back lines instead of one.** The model is asked for `FLOOR-COUNT: <N>` first and
   `FLOOR: <ids>` second. Scoring per replicate: count exact (equals the preamble's
   `class: floor` pin) · ids exact (the wave-4 expectation set) · ids-superset (every expected id
   present, extras tolerated) · the omitted ids named.

**Pre-registered reading:** counts 3/3 and ids below 3/3 on a command → enumeration recall, not
delivery; the wave-5 remedy is a design lever on the render or the bar (a `--floors` index line
in the preamble, or a count-keyed bar), the user's ruling. Counts below 3/3 → an attention
problem at that floor count; wave 5 needs a design change before opening. `implement`'s wave-4
refusal is counted as a harness artifact only if the gate-valid argument removes it 3/3.

**Cost:** 18 sandbox sessions. **Evidence:** per-case directories as in the suite; a
`diagnostic.json` summary. Report: `wave4-reports/p3-diagnostic.md`; V3 delta re-grade.

*Amendment before the scored runs (P3 probe, 2026-09-04, two sessions):* change (1) is not
implementable as written — in a headless run `$ARGUMENTS` takes everything after the command name,
and an instruction placed before the command stops the command expanding at all (zero blocks;
the model answered `FLOOR-COUNT: 0` and named no ids — a negative control showing the read-back
tracks delivery rather than the model's prior knowledge of the command). What change (1) can do,
and what runs, is a **gate-valid token at the front of the argument** with the instruction after
it. The `implement` probe with that shape: no refusal, count 34 exact, all 34 ids named. The
scoring, the bar reading, and the replicate count are unchanged.

## 7. Standing assumptions (lead; say the word to change any)

- The `reword-section` op extends grammar 1 rather than minting grammar 2, because no binary is
  published yet; the first publish freezes grammar 1 with the op in it.
- Skew, hooks-off, and policy are mechanism cases and run once (on `brainstorm`), not per command.
- The read-back bar is 3/3 for every command, `implement`'s 34 ids included — the same
  pre-registration as the pilot; a miss is a trip, not a reason to relax the bar.
- No governance amendment: the converted-command clauses of v3.0.1 already cover all six; the
  wave lands as a plugin MINOR (0.105.0).
