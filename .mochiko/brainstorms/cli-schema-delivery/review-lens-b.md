# Cold end-stage review — lens B (record integrity)

**Record under review:** `.mochiko/brainstorms/cli-schema-delivery/record.md` (frozen at the Q12 line, Status `open`)
**Reviewer:** cold end-stage reviewer, never in the room; paired dispatch, lens B = record integrity
**Date:** 2026-09-03
**Protocol:** `mochiko:review-brainstorm` — Phase 0 blind angle map (below, produced before record contact), independent cold read, six hunt classes, fact-map sample audit against the files, external-claim verification per `references/EXTERNAL-CLAIMS.md`, fitness per `references/RECORD-FITNESS.md`, coverage diff. No cross-examination round was scheduled by the lead, so findings are reported undebated.
**Default posture:** FAIL.

---

## Part 1 — Blind angle map (Phase 0, built from the topic statement and goal line only)

Grounded in `CLAUDE.md`, `.mochiko/memory/governance-ledger.md`, `.mochiko/memory/knowledge-management.md`, `DECISIONS.md`, `BACKLOG.md`, `ROADMAP.md`, `crates/mochiko-cli`, `plugins/mochiko/schemas/`, `scripts/`, `.claude/rules/mochiko/`, `.mochiko/strips/README.md`. No session artifact was opened before the map was delivered.

### A. Problem and driver

1. Problem named separately from solution. The topic states a solution before a pain; the record must name the driver set or its decisions are unfalsifiable.
2. Baseline measured, not asserted. Measured delivery figures already exist for the four skill-schema families; a cost claim cites them or says it does not.
3. Null road recorded. Keeping YAML with the CLI reading it is both today's shape and a prior ruling; it must be rejected with reasons.
4. Adopt-first discipline run. Storage is a named commodity category in the repo's own skill; shelf candidates named, custom argued against in writing.
5. Honest driver attribution. A user preference driving against a live principle should read like a recorded concession, not engineered necessity.

### B. No-fallback posture and governance collision

6. The additive-install principle head-on. It requires the plugin to install and function markdown-only with raw-Read data as the first-class degraded path.
7. Amendment route and semver. Principle removal or incompatible redefinition is MAJOR, routed through a setup amend run as a governance event.
8. The raw-Read fallback ruling superseded and bookkept, with decisions row, both indexes, and record status agreeing.
9. The kernel bright line re-tested, not inherited. Sole-path delivery plausibly gates pipeline progress.
10. Install mechanics priced: per-platform binaries, signing, marketplace install with no build step.
11. Unresolved prerequisite named: the open probe on whether the repo-relative schema path resolves from an installed plugin cache.
12. Failure semantics stated for missing, wrong-architecture, or non-executable binary.
13. The embedded copy's status: fallback the ask kills, or delivery path it keeps.

### C. Store shape

14. Scope enumerated across top-level schemas, in-directory skill schemas, label registries, shared blocks, shelf data, provenance sidecar.
15. Shipped versus maintainer-side split preserved; the sidecar is ruled repo-side and never shipped.
16. Granularity ruled: blob per schema buys nothing over a file; normalized tables unlock the queries prior waves wanted.
17. Git-diffability confronted: strips carry verbatim content, validators read the artifact, grep runs the CI secret scan and the similar-rules detector.
18. A text mirror is itself a fallback; either the ask is narrower than stated or the contradiction must be resolved.
19. Parallel-seat writes: builds run many producer seats under strictly disjoint file ownership, which one database file makes impossible.
20. Read-cost honesty: a win exists only if retrieval becomes selective, which is a larger claim.

### D. Migration and content-change model

21. Migration mechanism: forward-only versioned migrations and a compatibility matrix in both skew directions.
22. Edit workflow through the CLI: a validator cannot grade a blob, so the record must say what the grader reads.
23. Identifier lifecycle survives: mint-once IDs, tombstones, inheritance stubs, provenance anchors, floor-class must-survive semantics.
24. Checker fate: three helper scripts load YAML by path; a store move rewrites or retires them.
25. Rollback stated, with trigger and mechanism, as the predecessor ruling did.

### E. Binding the markdown to the CLI

26. Re-point surface enumerated across schemas, commands, skills, references, router rows.
27. The two-arm invoke-else-read form dies; removing arm two is a supersession on every carrier.
28. Ceremony scale priced: each re-point is a shipped-primitive edit with strip, budget pre-assert, and independent audit.
29. CLI surface growth: serving every schema is a different program from serving eight template names.
30. Router and reference files carry the two-arm text too and are easy to miss in scoping.

### F. Test regime

31. Current state cited honestly: integration tests only, no unit tests, no binary-spawn test, a single Linux CI job.
32. Cross-platform matrix: a mandatory binary behind a Linux-only CI is an evidence gap.
33. The untestable half named: the real failure is a model not reading the schema.
34. Fault set enumerated: missing store, corrupt store, read-only filesystem, concurrency, version skew, wrong architecture, plugin-cache path.
35. Gate wiring: two release gates assume readable YAML data files, and re-expressing them is a ledger event.
36. Evidence honesty at zero runs: the prior CLI's own first-live-run watch is still open.

### G. Hooks

37. Hooks are gating machinery; a fresh admission ruling is needed, not inheritance.
38. Install additivity: hooks change the user's environment, so consent and reversibility must be ruled.
39. Prior open item related: a standing backlog item already asks which behaviors earn prose versus a hard hook.
40. Exploring is not ruling: hooks marked deferred with a trigger, or ruled outright.

### H. Record integrity

41. Per-decision completeness: statement, arguable rationale, confidence mark.
42. Confidence marks honest: a user preference over a live principle should produce at least one contested mark.
43. Fact-checker map present and sample-auditable against the files.
44. Stale-source trap: the roadmap stamp still reports figures four waves out of date.
45. Supersession bookkeeping complete, stated per ruling rather than in a blanket sentence.
46. Reserved rulings dispositioned: the reopen condition and the rollback trigger on the parent watch.
47. Protected content preserved, with the verification method stated.
48. Reconstructibility survives: strips plus decisions plus version stamps assume text.
49. Landing ritual complete: decisions row, trail move, roadmap touch, statuses agreeing.
50. Open threads listed as open, with revisit triggers.
51. External claims disclosed per claim; platform-API claims are the high-risk class.
52. Provenance stated: who produced, who reviewed, when, solo or paired.

---

## Part 2 — Fact-map sample audit (what I reproduced against the files)

Run in the worktree at `1ed5c19`. The record's fact discipline is, on the whole, strong: the numeric claims I could reproduce came back exact.

**Confirmed exactly**

| Claim | Verified |
|---|---|
| F1 crate components 119 / 239 / 224 / 6, two dependencies, three commits, last touched 2026-08-26 (v0.91.0), closed eight-name set, resolution order, CI is the only workflow | yes |
| F2 twenty files in `plugins/mochiko/schemas/`, thirty of thirty-eight skill directories carry `schema.yaml`, `implement.yaml` 1,019 lines / 105 rules, command rules 321, skill rules 695, total 1,016, skill floors 226 | yes |
| F3 verbatim Rules-block text "The raw Read is the first-class read: no binary, no render step." | yes, in `brainstorm.md` and `implement.md` |
| F4 shared block `authoring-common.two-arm-template`; router row 58 names `architecture-shelf-backend` as a CLI template while the binary rejects it; `setup.yaml` the only `${CLAUDE_PLUGIN_ROOT}` site | yes |
| F5 script lines 1,239 / 1,094 / 457; allowlist 214 rows; both checkers PASS with 0 findings; detector 0 clusters over 1,016 rules with 181 allowlist-suppressed edges | yes, reproduced by running all three |
| F6 ninety-nine strip files, 2.8 MB, 597 provenance anchors | yes |
| F7 governance text | verbatim against the ledger |
| F8 manifest, marketplace source, no hooks configured, no `hooks/` directory | yes |
| F11 `converting-skill-to-schema` absent on disk | yes |
| Sibling-session relations | verified against `producer-plan-enforcement/record.md` on `main`: accepted 2026-09-03, build pending in two waves, D1 `Contested` "detection plus review, no hook gate, no worktree", and "a future hook gate takes its own ruling from zero" verbatim |

**Did not survive verification:** F1's stated crate total, F2's tombstones claim, F2's floor count instrument, and four items in F9. Those become findings below.

---

## Part 3 — Survivors

### Critical

**C1 — F9's hook map is materially incomplete, and the gap reaches D3 and D7.**
*Target:* F9.1, F9.10(c), D3, D7, D10.2.
F9.1 enumerates sixteen hook events and names `PreToolUse` as the only blocking one. The documentation lists roughly thirty-four events, of which ten block on exit 2. The omission that matters is `UserPromptExpansion`, documented as: "When a user-typed command expands into a prompt, before it reaches Claude. Can block the expansion"; exit 2 "Blocks the expansion"; and plain-text stdout is added as context on "`UserPromptSubmit`, `UserPromptExpansion`, `SessionStart`, and `PostModelSwitch`".
That is a deterministic injection-plus-halt at exactly the moment a mochiko command fires. It is the mechanism D3's prose halt clause cannot enforce and the one D7 assumed did not exist short of the bright line. By the record's own D10.2 argument — a required binary whose absence halts a run is an infrastructure dependency, not output-gating — a `UserPromptExpansion` halt on a missing or out-of-range binary is admissible under the line the record already argues.
*Failure scenario:* the design ships, and the first genuinely enforced no-fallback halt turns out to have been available on day one through an event the record never saw, after a MAJOR governance amendment was spent on a weaker mechanism.
*Source (must be re-read by the lead before this survives, per the external-claims re-read clause):* `https://code.claude.com/docs/en/hooks`.
*Disposition:* re-verify the event and blocking tables, restate F9.1, and re-rule D3's halt mechanism and D7's hook set against `UserPromptExpansion` (and `Setup`) before wave 0 opens.

**C2 — The no-fallback guarantee rests on instructed prose, and the design keeps the files it set out to remove.**
*Target:* D3, D9 wave 1, driver A.
D3: "the `.md` carries one halt clause — an empty or error block means stop and surface 'mochiko-cli missing', never proceed, never Read a file instead." That is a model-obeyed instruction, which is precisely the "instructed, not forced" weakness F3 names as driver A's gap and the reason the session exists. The record never confronts that its no-fallback limb is enforced by the mechanism it just declared unreliable.
Compounding: D9 wave 1 regenerates derived snapshots "in today's exact file shapes", and the open thread leaves the post-wave-6 snapshot format undecided. Readable YAML in the exact current paths therefore stays on disk indefinitely, forbidden only by prose. The user's ask was "I don't want fallback files be available too"; the design ships them as derived artifacts and relies on instruction not to read them.
*Failure scenario:* a model under context pressure reads a snapshot instead of halting; delivery silently degrades to today's behavior while the record claims the class is dead.
*Disposition:* name the assumption inside D3; either bind the halt to a blocking hook (C1) or state the residual risk and mark D3's no-fallback limb `Assumed`. Rule the snapshots' end state rather than leaving it builder's room.

### Important

**I1 — The SQLite projection is machinery the record never pays for.**
*Target:* D1, build surface.
D1 makes the migration log truth and derived snapshots the read surface. Its rationale argues only for the log — migrations, git legibility, disjoint file ownership, reconstructibility by construction. Nothing states what the projection buys over replaying the log or reading a snapshot. The build surface then defers the choice: "`rusqlite` bundled, or the simplest in-process store that satisfies 'file-based DB' — builder's room". "Satisfies 'file-based DB'" restates the ask; it is not a requirement. *Cheaper shape:* log plus derived snapshots, no database.
*Disposition:* state the projection's job — query surface, render cost, integrity — or record it as user-preferred with the cheaper shape named on the record.

**I2 — No null road recorded.**
*Target:* fitness, "rejected roads recorded".
Q1 ranked drivers and struck none; Q2 through Q11 all chose among move-shaped options. The record never records "the drivers do not justify a hard binary dependency plus a MAJOR governance amendment" as a road seen and rejected. Predecessor sessions on this axis recorded null roads explicitly.
*Disposition:* add the null road with its reasons.

**I3 — The template-schema first-live-run watch is left silently.**
*Target:* D10.5, D10.6, prior-session relations.
D10.6 supersedes the command and skill delivery watches. It does not name the CLI's own watch, which carries the M7 rollback trigger — CLI-delivered guidance underperforming the markdown baseline reverts the eight supersessions and re-points skills back — and the D5 reopen condition. That watch is this session's direct parent and still stands at n=0. After wave 6 the markdown baseline it compares against is gone.
*Disposition:* name it discharged, superseded, or carried, and say what becomes of M7 once the exemplars are unrecoverable.

**I4 — The parent session's scope ratchet is unnamed.**
*Target:* D10.5.
`schema-based-template-guidance` D3 scoped the crate to the eight pipeline artifact templates "first … with an explicit later ratchet". D2 here widens it to all fifty schema files, which is that ratchet firing. D10.5's supersession list names D8, D1's renderer limb, and the closed name set, not D3.
*Disposition:* add D3's ratchet to D10.5 as discharged.

**I5 — GI-008 is untouched by a decision that retires three of its subjects.**
*Target:* D6, D10.
D6 retires the two checkers and the detector. GI-019's own text places "the 6 existing scripts (5 `.py` validators, 1 `.sh` detector)" outside kernel-class, carried by waiver GI-008 whose revisit trigger reads "Script count grows, or a script becomes load-bearing in a shipped flow". D10 claims to hold the governance envelope in one place and never mentions GI-008.
*Disposition:* add a GI-008 clause to D10 — the waiver narrows or discharges at wave 6.

**I6 — GI-020's Testability rows are not rewritten.**
*Target:* D10.1.
D10.1 rewrites the enforcement text. The ledger entry's Testability reads: "Pass: a fresh plugin install with no binary present is fully functional. · Fail: install requires a build step, fetches a binary, or fails without the binary." Under the new posture both lines invert. An amendment that rewrites enforcement and leaves testability standing ships a self-contradicting principle, and the governance validator grades exactly that trace.
*Disposition:* name the Testability rewrite inside D10.1.

**I7 — The provenance sidecar's disposition is unstated.**
*Target:* D2, wave 6, Open threads.
D2 folds the sidecar "in as migration grammar". Wave 6 never lists the file. Five hundred ninety-seven anchors are protected-content carriers under GI-005 and GI-006. Whether `.mochiko/provenance.yaml` is deleted, frozen as archive, or kept authoritative is neither ruled nor listed open.
*Disposition:* rule it, or list it in Open threads.

**I8 — Collapsing scaffold criterion 3 drops FAIL survival, not only the count pin.**
*Target:* D6, D10.6.
D6 says the pins leave the `.md` "so scaffold criteria 2/3 (command) and 2/3 (skill) collapse to 'the `!` line and halt clause are present and name the right primitive'". Criterion 3 is not only the count: it requires every `kind: fail` rule to survive, a reword to keep its ID, and the `.fail.*`-segment-to-`kind: fail` correspondence to hold in both directions. D6's hard migration set names floors and anchored rules; it never names `kind: fail` survival. The two populations are distinct — `implement.yaml` carries 105 rules, 34 floors, 15 fails.
*Failure scenario:* a migration tombstones a fail node with no floor class and no anchor; nothing rejects it, and the audit criterion that would have caught it was collapsed in the same ruling.
*Disposition:* add `kind: fail` survival to the validator's hard set, or keep criterion 3's survival limb and collapse only the count.

**I9 — "No schema file Read anywhere" cannot hold run-wide until wave 6.**
*Target:* D8, D9.
D8 states the contract-suite assertion unconditionally. D9 keeps snapshots readable and converts commands at wave 4 before skills at wave 5. A converted command invoking an unconverted skill reads a schema file, so the assert fails for every command run through waves 3 to 5.
*Disposition:* scope the assert per-primitive with a wave-keyed tightening to run-wide at wave 6.

**I10 — F9.4's "no install-time or update-time hook exists" is wrong as stated, and it narrowed the Q5 option set.**
*Target:* F9.4, D4.
Documentation: "Claude Code runs the install inside the copied version directory each time it creates one: when you install a plugin, when Claude Code updates a plugin to a new version, and at session start when an enabled plugin isn't cached yet, such as on a new machine. The install runs only when the plugin's root directory contains both a `package.json` and a supported lockfile" — running `bun install --frozen-lockfile --ignore-scripts` or `npm ci --ignore-scripts`. Lifecycle scripts do not run, so it is not a general postinstall; but a lockfile-driven dependency install exists at install, at update, and at session start, and it was not on the Q5 option set. Whether it could carry per-platform prebuilt binaries onto the Bash `PATH` is itself unverified and should be stated as such rather than assumed either way.
*Source (lead re-read required):* `https://code.claude.com/docs/en/plugins-reference`.
*Disposition:* correct F9.4; record the road as seen and assessed with its viability marked unverified, so D4's declined-alternatives set is honest.

**I11 — Two of the three wave-0 abort probes are already answered in the docs, and the answer changes what the `.md` must carry.**
*Target:* F9.2, F9.10(a) and (b), D3, D6, D10.6.
F9.10(a) asks whether `!` preprocessing needs an `allowed-tools` grant; the skills documentation pairs the syntax with a grant in its own worked example (`allowed-tools: Bash(gh *)` beside ``!`gh pr diff` ``) and states "Claude Code honors the frontmatter in every kind of session, so an `allowed-tools` grant goes through the normal permission flow." F9.2 marks `${CLAUDE_PLUGIN_ROOT}` expansion in bodies unverified; the plugins reference gives "Skill and agent content | Anywhere the placeholder appears", and the skills page adds that Claude Code substitutes it "in two places: the skill's markdown content, and Bash rules in the `allowed-tools` frontmatter".
Consequence the record misses: D3's converted `.md` needs a frontmatter `allowed-tools` rule, which D6's ceremony re-key omits and which the canonical scaffold's required key set does not contain. Under D4 the binary is not in a plugin `bin/`, so the rule cannot be plugin-root-anchored — the grant's shape is an open design question, not a probe.
*Sources (lead re-read required):* `https://code.claude.com/docs/en/skills`, `https://code.claude.com/docs/en/plugins-reference`.
*Disposition:* restate F9.2 and F9.10 as documented, add the frontmatter obligation to D3 and D6, and re-scope wave 0 to what is genuinely unknown.

**I12 — The projection's write location contradicts documented guidance.**
*Target:* D1, build surface wave 1.
The build surface puts the "SQLite projection built into the plugin cache keyed by log hash". The plugins reference states: "`${CLAUDE_PLUGIN_ROOT}` changes when the plugin updates. The previous version's directory remains on disk for a grace period after an update, but treat it as ephemeral and don't write state there." A `${CLAUDE_PLUGIN_DATA}` placeholder exists and is substituted in plugin skill content; the record never names it.
*Source (lead re-read required):* `https://code.claude.com/docs/en/plugins-reference`.
*Disposition:* name the projection's home against `${CLAUDE_PLUGIN_DATA}`, or state why the cache is acceptable.

**I13 — Blanket confidence marks contradict the per-decision headings.**
*Target:* every decision heading; Evidence honesty.
Evidence honesty says "Every mechanism decision above ships `Confident` on the choice and `Assumed` on efficacy until wave 0 and wave 3 report." D1, D2, D5, D6, D7 and D10 carry a bare `Confident` in their headings; only D3, D8 and D9 carry split marks. A cold reader of D1's heading sees an unqualified `Confident`, which is the silent upgrade the fitness checklist forbids. The sharpest case is D7, marked `Confident` while its limb (b) is conditional on a probe F9.10(c) marks unverified.
*Disposition:* split the marks in the headings, or have each heading point at the Evidence-honesty clause.

**I14 — D2's strip freeze over-reaches its own scope.**
*Target:* D2.
D2 says "The 99 existing strip files freeze as archive (append-only per KM)". Strip files are one per primitive, and a converted primitive is a `.md`-plus-schema pair. D2 itself keeps prose "still strip-governed", and D10.6 and wave 6 both scope the freeze to schema content. Freezing the ninety-nine files wholesale would stop prose strips on the same primitives.
*Disposition:* scope the freeze to schema-content entries, not to the files.

**I15 — "GI-005 becomes mechanical" over-claims.**
*Target:* D2.
GI-005 covers the whole record layer: protected content in prose primitives and the dead-pointer scan over the roadmap, decisions index, and backlog. The migration constraint mechanizes the schema-rule limb only.
*Disposition:* scope the claim to the limb it actually mechanizes.

### Minor

- **M1 —** F1 states the crate at 599 lines; its own components sum to 588 (119 + 239 + 224 + 6, each verified).
- **M2 —** F2's "every command schema carries a `tombstones:` block" is false. Only `specify.yaml`, `brainstorm.yaml` and `setup.yaml` carry one; `implement.yaml`, `feature.yaml` and `architecture.yaml` carry no `tombstones` key at any indent.
- **M3 —** F2's "floors 112" is the raw `class: floor` grep count. The checker the record cites elsewhere reports 110 across the six commands. The instrument is unstated; the skill-side figure of 226 matches the checker exactly.
- **M4 —** F9.1's blocking census contradicts the record's own prior-relations paragraph, which imports `SubagentStop`, `TeammateIdle` and `TaskCompleted` exit-2 blocking from the sibling record eighty lines earlier.
- **M5 —** Ceremony scale is unpriced. Waves 3 through 6 touch thirty-six primitives, roughly thirty two-arm sites, the primitive-edit rules file and the budget ledger; the comparable prior wave was priced at eighty-three strips across forty files.
- **M6 —** The standing backlog item "Prose vs. gate allocation", which asks exactly which behaviors earn prose versus a hard `PreToolUse` hook, is not named by D7.
- **M7 —** The existing measured read-cost figures are not cited. The open thread predicts rendered output is smaller than raw YAML; the four family multipliers and delivered-at-invoke totals already on record would give that prediction a baseline.
- **M8 —** D4 says Windows is "served by cargo only" without noting that `cargo install` compiles from source and requires a Rust toolchain — a build step moved onto the user, adjacent to what GI-020 forbade of the plugin.
- **M9 —** F9.9's "a `SessionStart` hook is the earliest point" is contestable: a `Setup` event is documented for "one-time preparation in CI or scripts" under `--init-only`, `--init` or `--maintenance`.
- **M10 —** D10.5 lists the crate's closed name set and embedded copies among "supersessions-by-ruling". Those are code, not ruled decisions, and the strips ceremony has no entry type for them.

---

## Part 4 — Killed candidates

Formed during the cold read, dropped before reporting.

- **Binary store destroys git diff, blame and review** — D1's rationale confronts this directly and picks a text log for exactly that reason.
- **A single database file kills disjoint-seat file ownership** — named in D1's rationale and solved by per-seat migration files.
- **The maintainer-versus-shipped provenance split collapses** — D2's build profile handles it and D10.5 lists D16 untouched.
- **No adopt-first discipline on the storage choice** — the build surface names `patterns-adopt-first` for the store decision. Survives only in the weaker form at I1.
- **Record status disagrees with its index entry** — both read `open`; bijection holds.
- **Sibling-session relation claims unverified** — verified against the sibling record on `main`, including the "from zero" sentence verbatim.
- **`!` preprocessing in `SKILL.md` is unverified** — the documentation confirms it plainly; the record is more conservative than its own evidence.
- **F5's run-output numbers unverifiable** — reproduced exactly by running all three scripts.
- **No landing ritual** — wave 6 names decisions rows, trail moves and roadmap touches at each wave.
- **Reconstructibility breaks across the genesis boundary** — D2 states pre-genesis history stays in strips and git.
- **Agents excluded without justification** — F10 plus the open thread give an evidence reason.
- **The record cites the stale roadmap stamp figures** — it does not; every count is freshly taken.

---

## Part 5 — Tally and status

**Raised:** 39 candidates formed. **Survived:** 27 — 2 Critical, 15 Important, 10 Minor. **Killed:** 12.

**Recommended status: `critical-gaps`.**

Three of the protocol's four critical-gaps triggers fire. A load-bearing claim is broken: F9.1's blocking census and F9.4's install claim are both contradicted by the documentation the record's own dispatch cited. A Critical coverage gap stands: the delivery-and-halt mechanism (C1) was ruled against an incomplete platform map, and the record's own admissibility argument would license the better mechanism it never saw. And the record's central promise — no fallback — currently rests on instructed prose while shipping the readable files in their exact current paths (C2).

The record is otherwise strong and unusually well-grounded: its reproducible repo facts came back exact, its rejected roads are named per decision, its user-overruled decision carries `Contested` with `Assumed` reasons, its evidence-honesty section is candid about n=0, and every decision traces to a numbered question in the session trail. The gaps are concentrated in the external fact layer and in governance bookkeeping, both repairable without re-deriving the design.

**Re-read obligation.** Findings C1, I10, I11, I12 and M9 rest on outside-repo sources. Per the external-claims re-read clause, the lead must re-read the cited pages cold before these survive: `https://code.claude.com/docs/en/hooks`, `https://code.claude.com/docs/en/plugins-reference`, `https://code.claude.com/docs/en/skills`.

**Overlap flag.** C2 and I1 are the likeliest duplicates of lens A findings under a decision-quality formulation. Flagged by name, not merged; the cross-set merge is the lead's.

---

## Delta-check (bounded verify round, lens B)

Scope: my own 27 survivors' folds against the folded record only. No fresh cold read, no new blind map, no new angles. The record was not edited.

**Result: NOT CLEAN — 3 blocking, 1 nit. 24 of 27 folds clean.**

### Status and index agreement

Holds. The index entry reads `Status: open`; the record's Status line reads "open — pair cold review returned `critical-gaps` … verify round pending; acceptance pending." The head word agrees and the qualifiers narrate the same state. No decisions-index row exists yet, so nothing there can disagree. Bijection holds: the directory has its entry.

### External re-read faithfulness (C1, I10, I11, I12, M9)

Folded faithfully. Every phrase I fetched independently appears in F9 with its meaning intact: the command-expansion event's firing moment and its exit-2 blocking behavior, the four events that take plain-text stdout as context, the lockfile-driven dependency install with its conditions and `--ignore-scripts` flags, the placeholder substitution sites including the `allowed-tools` Bash rules, and the do-not-write-state warning on the plugin root. F9.3 correctly keeps "whether the grant is *required* for `!` to run" marked unverified — the documentation pairs them in an example without stating a requirement, which is exactly what I found. The two facts the lead added beyond both maps, hooks being fail-open and oversized output arriving as a path plus preview, both push the design in the conservative direction and are folded into the halt clause and the hook floor.

*Caveat, non-blocking:* two items in F9 sit beyond what my own fetches returned and rest on the lead's re-read alone — `WorktreeCreate` in F9.1's blocking list, and `${CLAUDE_PLUGIN_DATA}`'s path and lifecycle detail in F9.2. Neither is refuted by anything I have; I simply cannot confirm them.

### Blocking defects

**1 — I5's fold rests on a wrong script census (D10.7, Constraints 7, prior-session relations).**
D10.7 reads: "GI-008 narrows at the amend run: the waiver's scope drops the three retired scripts and keeps the remaining three (`validate-*`/`check-artifacts.py`, `detect-stack.sh`)."
Both halves are wrong on the files. The plugin ships exactly six helper scripts, and they are GI-008's census to the letter — one bash (`detect-stack.sh`) and five python (`validate-requirements.py`, `validate-user-stories.py`, `validate-openapi.py`, `validate-model.py`, `check-artifacts.py`), each under `plugins/mochiko/skills/*/scripts/`. D6 retires none of them. The three it does retire live in `scripts/` and all postdate the 2026-08-06 ratification — `check-command-schema.py` at v0.92.0, `find-similar-rules.py` at v0.99.0, `check-skill-schema.py` at v0.100.0 — so they are not members of the waived six and appear in no waiver row. The fold therefore drops non-members from the waiver and then miscounts the remainder as three when six remain, four of them `validate-*`.
*Why blocking:* the disposition is misapplied, and it changes what the wave-2 amend run would land.
*Repair:* restate the engagement. GI-008's six are untouched by this session. The live question my finding was pointing at is different and still open: the three post-ratification checkers carry no waiver row at all, so their retirement and Rust port may need no ledger touch — or the absence of a row is itself the thing to record. Rule which.

**2 — I13's fold reintroduces the defect it was fixing (Evidence honesty).**
Evidence honesty now claims: "Every decision heading now carries its own split mark — choice vs mechanism/efficacy — so no bare `Confident` stands on an untested mechanism (review B-I13)." That is false for two headings. D2 reads `Confident` bare; D10 reads `Confident` bare. Nine of eleven headings did get split marks, which is the substance of the fold, but the blanket sentence overstates the result — which is precisely the blanket-versus-per-decision contradiction I raised.
*Why blocking:* a record-integrity invariant fails on its own claim, and D2 does carry an efficacy limb worth marking (its GI-005 mechanization depends on an unbuilt validator), as does D10 (its own limb D11 is `Assumed`).
*Repair:* split D2's and D10's marks, or scope the sentence to the mechanism decisions and name the two scope-and-envelope rulings as choice-only.

**3 — I7's fold contradicts itself on when the sidecar freezes (D2 versus the build surface).**
D2 rules the sidecar "**frozen as archive at genesis** … the file moved to `.mochiko/archive/provenance-genesis.yaml`". The genesis migration is a wave-1 item under D9. The build surface lists "provenance sidecar frozen to archive" under wave 6. Two moments, one action.
The wave-6 reading is the coherent one: D6 keeps the three Python checkers alive until their matrices are proven ported at wave 6, sidecar anchor resolution is one of the checks they carry (F5), and both checkers currently pass against the live file. Moving it at wave 1 breaks them for waves 1 through 5.
*Why blocking:* a fold introduced a contradiction between two sections.
*Repair:* one line — anchor content is captured into the genesis migration at wave 1; the file is archived at wave 6 when the checkers retire.

### Nit

**4 —** Non-blocking, recorded for completeness: the two F9 items named in the caveat above rest solely on the lead's re-read. If the wave-0 probe set is being finalized anyway, `WorktreeCreate`'s blocking status is cheap to re-confirm at the source.

### Clean folds (24 of 27)

| finding | fold | verdict |
|---|---|---|
| C1 | F9.1 restated (~32 events, 12 blocking); D7 gains dependency-halt hooks; D11 argues GI-019; D7's rationale names the incomplete map as the reason the Q8 ruling was superseded | clean, exceeds the disposition |
| C2 | halt re-keyed to positive confirmation; policy placeholder, oversized-render stub and hook-disabled cases named; D9 wave 6 deletes shipped snapshots and moves views repo-side; residual named and left `Assumed` | clean |
| I1 | D1 restructured to in-memory replay, cache on measured need, SQLite deferred with the shelf candidate named; the amendment note states the unpaid-for-machinery problem in its own words | clean |
| I2 | null road and maintainer-side-only road recorded rejected at the frame, with the governance cost attributed to a `medium` driver | clean, exceeds |
| I3 | D10.6 supersedes the template-schema watch, re-keys M7 to revert-the-wave, notes the exemplar baseline is unrecoverable after wave 6, carries D5's reopen unchanged | clean |
| I4 | D3's ratchet named in relations, constraints, and D10.5 as discharged | clean |
| I6 | D10.1 rewrites the Testability rows in the same run; F7 now quotes them verbatim | clean |
| I8 | `kind: fail` added to D2's anchor set and D6's hard set with the both-ways correspondence; criterion 3's survival limb explicitly moved, never dropped | clean |
| I9 | the no-Read assert scoped per converted primitive through waves 3–5, run-wide from wave 6, with my exact scenario named | clean |
| I10 | F9.4 corrected with the install-time step quoted; the npm road recorded as seen with viability unverified | clean |
| I11 | `allowed-tools` grant added to D3 and to D6's scaffold criteria; placeholder expansion closed as documented; wave 0 re-scoped | clean |
| I12 | cache home ruled `${CLAUDE_PLUGIN_DATA}`, never the ephemeral plugin cache | clean |
| I14 | freeze scoped to schema-content entries; strip files stay live for prose | clean |
| I15 | GI-005 claim scoped to the schema-rule limb | clean |
| M1 | 599 including `Cargo.toml` — verified at 11 lines, 588 + 11 | clean, arithmetic checks |
| M2 | tombstones stated as 3 of 6 and named; D8's fidelity fixture agrees | clean |
| M3 | both figures given with instruments stated | clean |
| M4 | blocking set restated; the relations paragraph no longer contradicts F9.1 | clean |
| M5 | ceremony scale priced against the v0.76.0 baseline | clean |
| M6 | backlog item related in relations, D7, and open threads as partially discharged | clean |
| M7 | measured figures cited in F3 as the read-cost baseline | clean |
| M8 | toolchain cost stated plainly, plus the Git Bash requirement and access-loss class | clean, exceeds |
| M9 | `Setup` noted with its flag-gated scope | clean |
| M10 | crate code reclassified as wave-1 build items | clean |

### Recommendation

Repair defects 1 through 3 and the round closes. None reaches a decision's substance: two are wording or sequencing repairs, and the third is a factual restatement that returns an open question rather than reversing a ruling. No fold I checked misapplied a user disposition in a way that changes a decision, and the two Critical clusters that touch my findings are folded more completely than I recommended.

### Delta-check round 2 (repairs only)

Repairs 1 and 2 clean. D10.7 now names the six waived skill-shipped scripts by file, dates the three repo-level checkers and states they were never waived, routes their retirement through D11 limb (ii), and records the "script count grows" trigger as undispositioned and moot at wave 6; the relations bullet, Constraints item 7, and the K2 table row all agree. D2 and D10 headings carry split marks as stated. Status line head word `open` agrees with the index entry.

Two residuals:

**R1 (repair 3, nit).** D2's body is correctly repaired — anchors carried from wave 1, the sidecar authoritative for the Python checkers until their port, frozen to archive at wave 6, matching the build surface. But D2's own Q13 amendment note two paragraphs below still reads "the sidecar's disposition ruled (frozen archive **at genesis**)". The round-1 wording survives inside the note annotating the repaired text, so the two-moments contradiction is relocated rather than gone. One-word repair: *at wave 6*.

**R2 (repair 4, not done as described).** F9's preamble still reads only "every item below is `verified` unless marked *unverified*". Nothing there attributes the items resting on the lead's re-read alone. The attribution appears once at the foot of the verify-round narrative, where it is asserted rather than made. The two items I named — `WorktreeCreate` in F9.1's blocking list, and `${CLAUDE_PLUGIN_DATA}`'s path and lifecycle detail in F9.2 — carry no marker separating them from facts both reviewers confirmed independently. Repair: one clause in the preamble, or an inline marker on those two.
