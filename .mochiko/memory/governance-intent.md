# Governance Intent — mochiko

**Session date:** 2026-08-06 · **Mode:** brownfield
**Confirmed at synthesis checkpoint:** 2026-08-06 by Deepesh
**Governs:** the governance surface set v3.0.3 (CLAUDE.md governance region · `.claude/rules/mochiko/` · `.mochiko/memory/governance-ledger.md`) — v2.0.0 at AM-1, v3.0.0 at AM-2 (2026-09-04), v3.0.1–v3.0.3 by the pre-authorized wave-3/4/6 PATCHes (2026-09-04/05, ledger amendment log); **v3.1.0 at AM-3 (2026-09-14)**

## Fact profile

- **GI-001 — Facts:** industry: none (developer tool) · data classes: none (no user data, no PII, no services, no DB) · jurisdictions/markets: none · contractual commitments: none · **Mark:** Confident
- **Modules triggered (mechanical):** none — negatives confirmed: no data classes → no privacy/security-regulatory module; no jurisdiction → no audit-trail module; no contracts → no contractual-compliance module; **no customer-facing UI → no accessibility module** (confirmed per-negative at review fold S3, 2026-08-06 — prose primitives consumed inside Claude Code, no UI ships). Blanket consequence stated and confirmed by user ("yes, this is developer tool"); the no-UI negative re-confirmed explicitly.
- **Brownfield cross-check:** consistent — `codebase-analysis.md` detects no services, no DB, no user data; integrations limited to GitHub hosting/marketplace, context7 MCP, and a local Anthropic gateway config.
- *AM-2 (2026-09-04, review I7):* fact profile **unchanged** by the CLI dependency and the sandbox-run release gate — no data classes, no jurisdiction, no contractual commitment; the third-party Terms-of-Service exposure of the sandbox's subscription auth is carried as a `Contested` mark on GI-012's substrate (not a fact-profile dimension, no module).

## Project identity & type

- **GI-002 — Type:** none of the shelf taxonomy fits ("it doesn't fit, so don't try to fit" — user) → shelves dealt: universal floor only, expressed procedurally; no backend/service shelf. · **Mark:** Confident
- **Identity:** mochiko — kernel-free agent-skill framework for Claude Code; markdown primitive library (5 commands · 9 agents · 28 skills · 14 templates, plugin v0.53.0). Currently personal tooling for Deepesh, with a planned trajectory toward a public product ("currently B with plan for A sometime"). Governance follows today's reality; public-product compat obligations are a recorded future amend trigger.
  *AM-1 (2026-08-16):* the "kernel-free" identity phrase is superseded by the D11-softened position — **markdown-first primitive library with kernel-class tooling admissible by recorded ruling** (GI-019); the first admitted instance is the ruled (not yet built) template-schema Rust CLI, strictly additive to the install path (GI-020).
  *AM-2 (2026-09-04, Card 3):* identity now **a markdown-first primitive library with a kernel-class delivery CLI** — `mochiko-cli` (`crates/mochiko-cli`, wave 1 built 2026-09-04) will serve every command's and skill's rules from a migration log **the plugin carries** (ruled end state — review C1, user-ruled "plugin carries it" 2026-09-04: the log ships at `plugins/mochiko/migrations/` from wave 3; today it lives at the repo root and no installed plugin carries it), replayed in memory at fire; the plugin depends on the binary (GI-020 as amended). The plugin still installs by clone alone; the binary is a developer tool the user installs (crates.io + Homebrew tap). Driver: `cli-schema-delivery` D1–D11. · **Mark:** Confident on the identity; the distribution basis **Contested** (record D4 — chosen against the lead's recommendation, reasons inferred and `Assumed`)
- **Risk surface:** flawed primitives propagate into downstream user projects; wasted design sessions; record-layer corruption destroying provenance. No money, no user data, no compliance exposure.
  *AM-1 (2026-08-16, review fold I3):* the ruled (not yet built) Rust CLI adds a **shipped-executable vector** — a compiled binary running on user machines is a materially different propagation/trust class than prose, incl. a supply-chain/dependency surface. Interrogated at AM-1; **no module attached now** (no binary ships yet, n=0; user-ruled). Revisit triggers: the crate's first public release · the existing public-product transition trigger.
  *AM-2 (2026-09-04, Card 3):* the vector is **live**: the binary is a required dependency (never shipped in the plugin — installed by the user), and **plugin-authored hooks execute on every consumer's machine** at every session start (`SessionStart` presence) and at every mochiko fire (`UserPromptExpansion` on `mochiko:*` commands, `PreToolUse` on `Skill`), under a 5-second timeout, fail-open by platform design, blocking only on the binary's absence or grammar skew — the user ratified that knowingly (record Q13). The **first-public-release trigger fires; its discharge is conditional** (review I5, user-ruled): four controls named — `cargo audit --deny warnings` in CI (present, `ci.yml`) · sha256-published release assets (present, `release.yml`) · `cargo publish` behind a manual-approval GitHub environment (**owed** — the job is `if: false` today; the environment is a wave-2 tail obligation) · signed release tags (**owed** — no signing exists in the repo; wave-2 tail obligation). The trigger discharges only when all four exist at the first publish; until then it stays open. **Access-loss class named:** users who cannot install developer tooling lose the plugin entirely (D4, accepted eyes-open). Fact profile unchanged (no data classes, no jurisdiction) → **no module attaches**; the public-product transition trigger stays standing. · **Mark:** Confident on the risk statement; the controls' presence `Assumed` until the first publish (review I5 — two of four are wave-2 tail obligations, see the AM-2 log)
  *AM-3 (2026-09-14, Card 6):* the hook vector widens — from wave 4 the plugin's hooks execute the plugin author's code on **every gated artifact write** in a consuming project (`PreToolUse` on `Write|Edit|Bash|PowerShell` under a declared home) and once per spawned seat (`SubagentStart`), not only at command/skill fire; the aggregate per-run hook cost cap (≤ 60 s, record OQ4) stands as a **D10 watch by ruling** (review C9, user-ruled 2026-09-14): a wall-clock cap in the Docker contract suite is nondeterministic and gate 6 is the deterministic set; the replay-cache trigger (`${CLAUDE_PLUGIN_DATA}`, `cli-schema-delivery` D1 deferred on measured need) is the named response, never a bump gate. **Supply-chain precondition (review C5, user-ruled 2026-09-14):** the two still-owed controls of the first-public-release discharge — signed release tags and the `crates-io` environment's approval rule (the publish job is `if: false` at `.github/workflows/release.yml:100` today) — become hard preconditions of the **wave-4 hook ship**: the wave-4 `plugin.json` bump MUST NOT land before the first publish with all four controls in place, and a break-glass install never substitutes for consumers — lands as one clause on GI-002's conditional-discharge line and GI-012's release line (the one clause by which Card 7's "GI-012 untouched" narrows). Fact profile unchanged; no module.
- **Team reality:** solo maintainer; review culture = author≠grader validator audits (structural independence, not human reviewers).

## Convergence skips

- Dimension 4 (risk) — settled by dimensions 1–3 + analysis: propagation/provenance risk, stated to user with no objection.
- Dimension 5 (team) — settled by dimension 1: solo maintainer, audits as review culture.
- Dimension 6 (practices) — pre-filled by `codebase-analysis.md`: no lint/test/build/CI commands exist; quality floor is procedural (audits · strips ledger · landing ritual · human gates).

## Real commands (dimension 6/8 → the validator's placeholder bar)

| Purpose | Command | Source |
|---------|---------|--------|
| Lint | `cargo fmt --all --check` · `cargo clippy --all-targets -- -D warnings` (crate); markdown primitives: none (procedural) | AM-2 re-expression (Card 5); was "none exists" at v1.0.0 |
| Test | `cargo test --all` (300 tests at v3.0.0; `MOCHIKO_FULL_SIMILAR=1` opts into the full similarity sweep) · `mochiko-cli migrate validate --plugin-root plugins/mochiko` (the log's hard set; the log lives in the plugin from v0.104.0) · `python3 evals/contract/run.py` (the plugin contract suite, Docker sandbox `claude-mochiko`, maintainer-side) | AM-2 re-expression (Card 5) |
| Build | `cargo build --release -p mochiko-cli` | AM-2 re-expression (Card 5) |
| Dependency advisories | `cargo audit --deny warnings` (a supply-chain control, also named under GI-002's risk discharge) | AM-2 re-expression (review M3) |
| Release | `plugin.json` semver bump + `CHANGELOG.md` + `marketplace.json` sync (gates GI-012); crate: tag `mochiko-cli-v*` → `.github/workflows/release.yml` (four targets, publish behind manual approval) | AM-2 re-expression (Card 5) |

The validator's placeholder bar adapts: no principle may cite a fictional command; enforcement clauses cite procedural gates (audit PASS, strip entry present, landing complete) for markdown primitives, and the real commands above for the crate, the log, and the plugin path (AM-2).

## Floor expression & deck rulings

Floor cards enter asserted — rows record *expression* (type translation), never a level ruling:

| GI-ID | Card | Layer | Ruling / Expression | Mark |
|-------|------|-------|---------------------|------|
| GI-003 | FLOOR-SEC | floor-asserted | Secrets out of repo: `.claude/settings.local.json` (live token, detected) added to `.gitignore` — **MUST-fix this session**; no credentials in primitives/records. Secret-scanning clause **narrowed**, not waived: gitignore + pre-commit vigilance now; no CI obligation while no CI exists. Revisit trigger: CI arrives. | Confident |
| GI-004 | FLOOR-TEST | floor-asserted | Translated: every shipped-primitive edit passes the author≠grader audit before a version bump (the live ratchet); no coverage percentage — prose has none. *Subsumed as inapplicable (GI-007): coverage thresholds, smoke test (no runtime critical path).* Helper scripts carved out by waiver GI-008. | Confident |
| GI-005 | FLOOR-ERR | floor-asserted | Translated: no silent corruption of the record layer — protected content leaves only by recorded ruling (strips/supersession); dead pointers are defects caught by the KM dead-pointer scan. *Subsumed as inapplicable (GI-007): API/UI error surfaces, correlation IDs, stack-trace leakage (no runtime).* | Confident |
| GI-006 | FLOOR-OBS | floor-asserted | Translated: traceability is the observability surface — strips ledger + `DECISIONS.md` + version stamps, **plus the migration log for schema content** (AM-2, Card 7: from wave 6 the log is the verbatim record for schema rules; strips keep prose); every primitive edit reconstructible from the record layer. *Subsumed as inapplicable (GI-007): structured logs, health checks, no-PII-in-logs (no runtime, no logs, no PII anywhere — GI-001).* | Confident |

*AM-2 floor notes (2026-09-04, review I2/I3, user-ruled "as recommended"):*
- **GI-004 (FLOOR-TEST) — expression changes for schema content, ratchet kept:** the audit ratchet for markdown primitives is unchanged; for **schema content** the steady-state audit unit becomes the migration file plus the regenerated derived-view diff, graded by `mochiko:validator` on five criteria (intent stated · anchor present where required · ID lifecycle right · floor and fail survival · register), with the CLI's apply result as the deterministic pre-pass (record D6); the `.md`'s independent count self-check is **retired and booked as a loss** (record D3 — the version triple confirms delivery, the CLI's printed counts assert completeness, the contract suite tests it). The `primitive-edits.md` re-key that lands this at wave 6 is a **pre-authorized PATCH activation** of this ruling, not a fresh amend run (user-ruled, I2). For the crate, author≠grader extends to code: every unit lands on a lead-approved plan with an independent non-author review (wave-1 precedent).
- **GI-005 (FLOOR-ERR) — the schema-rule limb becomes mechanical:** protected content in schema rules (`class: floor`, `kind: fail`, anchored rules) leaves only through a migration carrying a ruling anchor, enforced by the binary's hard set; the prose-primitive limb (strips + supersession-by-ruling) and the KM dead-pointer scan stay procedural. Two regimes, each named (record D2; review I3).

Arbitrated deck: **empty by ruling** — no shelf dealt (GI-002); no architecture-opinion cards apply to a prose library.

## Minted principle intents

- **GI-017 — Pointer-only region (none minted):** ruled by user ("leave these out"): the repo's existing constraints (no-kernel · author≠grader · landing ritual · single-sourcing · protected-content-by-ruling) stay in their current homes (CLAUDE.md prose, rules files). The governance region **points at** those homes; it never restates them. This is a producer-binding selection constraint — restating an existing constraint on a surface is a trace violation against this element. · **Mark:** Confident
  *Elicited from:* dimension 9 — candidates presented for codification, user ruled "leave these out"
  *AM-1 note (2026-08-16):* the "no-kernel" home named here now carries the D11-softened text (see GI-019, AM-1); the pointer-only mechanic itself is unchanged.

- **GI-019 — Kernel-class tooling admission (D11 bright line):** the no-kernel position is softened per the `schema-based-template-guidance` ruling D11 (2026-08-16): skills and agents remain the primary quality surface; kernel-class executable tooling is admissible **only by recorded ruling**, and such tooling never gates pipeline progress, never dispatches or sequences agents, never holds judgment that skills own. **Definition (review fold I1):** kernel-class ≡ executable tooling whose output primitives *depend on* to do their work — source-of-truth delivery, composition, or any standing infrastructure role. Advisory post-hoc checkers consumed as optional exit-code signals are **not** kernel-class; the 6 existing scripts (5 `.py` validators, 1 `.sh` detector) land there, unpainted by this element and still carried by waiver GI-008. First admitted instance: the template-schema Rust CLI (foundation seed for future native tooling, Tauri-bound), with the recorded concession that template delivery alone would not carry it. Constraint home stays CLAUDE.md prose (per GI-017 the region points, never restates). · **Mark:** Confident *(evidence basis n=0 — the concession above names it; review fold M2)*
  *Elicited from:* brainstorm `schema-based-template-guidance` D11, user-ruled at review disposition; setup amend invoked by the user 2026-08-16; definition + script placement user-ruled at the AM-1 review disposition batch
  *AM-2 (2026-09-04, Card 2) — widened admission, bright-line text unchanged:* `cli-schema-delivery` (accepted 2026-09-03, D11) is the recorded admission for the CLI's **widened role** — delivery of every command's and skill's rules from the migration log, hard constraints on the store's own data, and dependency-halt hooks; the 2026-08-16 admission covered template delivery only. Three clauses recorded as argument: (i) a required binary whose absence halts a run, and a hook that blocks the plugin's own commands only when that binary is absent or out of range, are infrastructure dependencies in the delivery role the line licenses — present, they render and deliver; absent, nothing can be delivered and the halt is the honest report of that fact, never a verdict on the run's work; (ii) hard constraints at migration apply are maintainer-time definition of the store's own data — a ruled carve: the landing ritual is a pipeline in this repo's sense, and a rejection there is a structural-validity check on data the tool owns, never a grade of a primitive's judgment content, which the author≠grader audit keeps; (iii) the judgment and sequencing clauses are untouched — the CLI grades no artifact, dispatches nothing; behavior-gating hooks are declined (`producer-plan-enforcement` D1). The three repo-level checkers (`scripts/check-command-schema.py` · `check-skill-schema.py` · `find-similar-rules.py`) were never waived — they rest on the advisory clause — and retire into the crate's validator at wave 6 under this admission. · **Mark:** Confident on the admission; the argument `Assumed` until this run's validator grades it (record D11)
  *AM-3 (2026-09-14, Cards 2–5) — conformance-gate admission, bright-line text unchanged:* `hook-enforced-artifact-schema` (accepted 2026-09-13, D1/D7; wave-0 probe PROCEED) is the recorded admission for two plugin-shipped hooks beyond the dependency halt: **(1)** a `PreToolUse` gate on `Write|Edit|Bash|PowerShell` piping the payload to `mochiko-cli check --hook-json -` and denying a write that fails a **mechanical** check — path against a declared home's pattern · file name against the home's declared set · required `##` headings in declared order with undeclared `##` denied · required frontmatter fields and enum values · declared placeholder tokens · per-section line budgets · a shell command carrying a write operator aimed at a declared home; **(2)** a `SubagentStart` hook injecting one self-identified reminder line per seat (D1b as amended 2026-09-13, superseding the `Read`-time form). Every check is decidable by string and count against data the log carries (the `home` document kind and each template's conformance block). **Narrow supersession:** `cli-schema-delivery` D7's "behavior-gating hooks are declined" stands for **judgment and sequencing**; the gate holds one write until it conforms — or, over an existing file already failing a relaxable measure (file set, shape, or budget), until the write does not worsen that measure — and a passing draft pays nothing. **First-touch amnesty (D4e — review C2, lead-repaired 2026-09-14 and corrected at the verify pass against the built binary's live behavior):** on any write over an existing file, `Write` and `Edit` alike, the on-disk file is the baseline; where the baseline already fails a relaxable measure — file set (an undeclared name in a declared home), shape (headings · frontmatter · placeholders), or budget — a write that does not worsen that measure is allowed; path is not a measure amnesty relaxes (a write outside every declared home is ungated, not amnestied). The binary names a standing shape or budget violation in `additionalContext`; **known gap, measured at the verify pass:** an `Edit` to an existing undeclared file name rides amnesty with a bare allow and no `additionalContext` — a mis-homed file carries no signal, and D9 makes the reviewer responsible for exactly that escape; carried to the wave-4 build as a fix candidate (name the file-set violation too), not a governance change. Existing files are never wedged. **Clause (iv) of the bright-line argument (D1/D4):** mechanical conformance is structural validity of an artifact against the store's own declared shape — the same class as clause (ii)'s hard constraints at migration apply, applied at write time — not a grade (no check reads meaning, ranks quality, or judges adequacy; those stay with the review skills and the author≠grader ceremony), not sequencing (the gate names no seat, order, or stage), not a pipeline gate (a denied write is re-emitted, the run continues, the two-strike halt sentence is advisory — D9). **Condition on clause (iv) (review C1, user-ruled 2026-09-14, road (a)):** the clause holds on a user-ratified budget table (D6, wave 3's gate) that admits honest content; a bound honest content cannot meet is a table defect corrected by ruling at the table, never by the hook — record OQ1 (whether a prose-shaped deliverable carries a budget or `max_lines: none`) is ruled there, not here, and the review's measurement is carried to that ruling: the working-tree migration's 150-line whole-file bound on `record.md` against 43 of 66 brainstorm records in this repo exceeding it (largest 1,536; this amend's own driver 738), with no in-run route for a fresh over-budget deliverable (amnesty needs a baseline, `reports/` takes only report-typed content, a new kind takes a migration and a plugin release). Until the table is ratified the condition stands open, disclosed; the declined road (b) — whole-file bounds advisory at create time, deny reserved for templated per-section checks — would amend D1/D6 in the brainstorm record and was not taken. **Hook floor re-ratified for the new hooks (D7c):** 5-second `timeout` on every shipped hook; fail-open when a hook cannot run or times out; hooks ship to every consuming project and execute the plugin author's code on every gated call — ratified knowingly. **Explicit-allow rule (D3/C2, wave-0 platform fact):** the platform denies a background subagent's call when no hook returns a decision, so the wrapper always exits 0 and emits an explicit `permissionDecision: allow` on every non-deny path, including CLI exit 1/2/3 (log unsound · usage or a binary predating `check` · grammar skew); a conformance verdict — `check` exit 4, `EXIT_CONFORMANCE` in `crates/mochiko-cli/src/hook.rs` — is the only deny; the dependency halt stays the existing hooks' job. **Scope of the guarantee (D7e, reconciled with GI-020 at Card 5):** the gate is a floor for consumers who keep the plugin's hooks enabled; a project that sets `disableAllHooks`, or an enterprise under `allowManagedHooksOnly`, stays **outside GI-020's supported set** — D7e states what remains there (the procedural author≠grader ceremony), it grants no support; a ratified consequence, not a defect. The `PowerShell` matcher arm ships on the doc quote, unverifiable on macOS (wave 0). **Reach (D9 — review C4, lead-repaired 2026-09-14):** outside every declared home a `.md` write is gated only when its content opens with mochiko report frontmatter or a template's `## Header` signature — a plain `.md` elsewhere is not mochiko's business; `NotebookEdit` and any MCP file writer a consumer enables are knowingly outside the matcher set; escapes past the sniff and hook-disabled consumers are the reviewer's under the author≠grader ceremony — the gate is a floor, not the whole fence. **What the gate reads (review C7, lead-repaired 2026-09-14):** the raw `PreToolUse` payload — `tool_input.content`, `old_string`/`new_string`, `command` — and, on `Edit`, the on-disk file to apply the edit in memory, against the log the plugin carries; nothing leaves the machine. **GI-019 Testability re-keyed (Card 3):** Pass = every shipped hook blocks on exactly two grounds — the binary's absence or a log outside its grammar range, and a conformance deny from `check` — with every other outcome an explicit allow, and `check` reading no meaning, naming no seat, order, or stage; Fail = a hook blocking on any other ground, a `check` grading meaning or quality, or a hook that sequences. **Recorded supersession in `.claude/rules/mochiko/rust-cli.md` (D7d, Card 4):** the bright-line bullet's "MUST NOT grade an artifact" and "hooks MUST block only on the binary's absence or a log outside its grammar range, never on behavior" are superseded — "validates an artifact's mechanical conformance against its own data" is the admitted reading, grading of meaning or quality stays forbidden; the verbatim prior bullet is preserved in the ledger GI-019 entry (the GI-020 precedent), no `.mochiko/strips/` entry (a governance surface, not a plugin primitive — AM-2's full rewrite took none). Untouched (Card 7): GI-020 (clone-only, required binary, no schema file ships — the `home` registry rides the migration log), GI-012 untouched but for the wave-4 precondition clause ruled at review C5 (wave 4's `plugin.json` bump takes the contract suite with the new hook cases, D10), GI-004/005/006 (the hook scripts and the migration are shipped-primitive edits under the primitive-edits ceremony), `producer-plan-enforcement` D1 (no hook gate on plan mode). · **Mark:** **Confident** on the admission and the user-ruled forks (R1 deny-now over declare-and-measure with both costs in view · R2b no consumer-local surface · R3 the shell legs) and on the deck of 8 (all "as recommended" 2026-09-14); **Assumed** on the D3-derived exit-code contract and the `check --hook-json -` interface (record D3: derived from GI-020 + `cli-schema-delivery` D1, not put to the user as a fork; the explicit-allow rule a lead repair inside D7's fail-open intent, disclosed at the brainstorm's acceptance) — mark split at review C3 (lead-repaired 2026-09-14), mirroring AM-2's C2 disposition; the exit-4 fact verified against the built crate this run.

- **GI-020 — Clone-only install with a required `mochiko-cli` dependency (superseded-by-ruling at AM-2; user-declared at AM-1, user-ruled at AM-2):** *AM-1 intent (superseded 2026-09-04):* the plugin's install path stays exactly as it is today — a markdown plugin, no install-time build step, no binary dependency, no submodule-class fetch burden; the CLI strictly additive with the schema data files readable raw as the first-class degraded path (record D8). *AM-2 intent (Card 1):* the plugin MUST install by a plain marketplace clone — no install-time build step, no fetch beyond the clone, no submodule-class burden — **and depends on the separately installed `mochiko-cli`** for every command and skill (a developer tool: `cargo install mochiko-cli` / the Homebrew tap; never shipped in the plugin). Absence or grammar skew **halts loudly at first use and never degrades** — measured (record F13): the failing `!` line aborts the command before any model turn, the `UserPromptExpansion` hook carries the install line, `SessionStart` reports presence; the shipped plugin carries **no schema file a run could read instead**. **The plugin carries the content (review C1, user-ruled 2026-09-04):** the migration log ships inside the plugin at `plugins/mochiko/migrations/` from wave 3 — the plugin has always carried the rule content (534 KiB of YAML across the 50 schema-class files today), and the log replaces it one for one; priced (verify-pass figures, 2026-09-04): the log is 604 KiB (618,122 bytes) today → +604 KiB at wave 3, a peak ≈ 1.1 MB while the snapshot files still ship, net ≈ +70 KiB at the wave-6 end state; content changes re-ship with plugin bumps exactly as today; the binary stays a pure engine (embedding the content in the binary would make every rule edit a crate release and a user reinstall — the lockstep D5 rejected; a network fetch is the silent-degradation class ruled out). A compact wire form is a later optimization if weight ever bites. **Transition clause:** from this ratification until the wave-6 landing, primitives not yet re-pointed read the derived snapshot files shipped in the plugin; the clause expires when no schema file ships in the plugin, asserted by the contract suite's run-wide no-Read assert (its expiry is a pre-authorized PATCH amendment). **Declared unsupported** (dimension-10 exclusions, review I4): environments that disable skill shell execution (`disableSkillShellExecution`, Cowork/synced skills) or hooks by policy · **PowerShell-only Windows** (`shell: bash` fails without Git Bash) — Windows is served only with Git Bash present, and only via `cargo install`, which compiles from source: the "no install-time build step" property is a property of the **plugin**, not of the tool install (a Windows user runs a Rust build once). **Testability during the transition (review I1):** the end-state rows ("a fresh install plus the tool install renders every command's and skill's rules"; "with the binary absent every mochiko fire halts") are **dormant until the wave-3 pilot re-points the first primitive** — the AM-1 dormant-clause idiom; assertable now: the contract suite's absence and skew cases (2/2 passed 2026-09-04) and the log's hard set at 0 rejecting. **Revisit trigger (review M1):** the wave-3 pilot's abort criteria — the floor read-back metric below its pre-registered bar, or the per-invoke read cost above the pre-conversion baseline — halt waves 4–5 and return the posture to the user; reversal after this ratification costs a second amend run plus re-pointing every converted `.md` (record D9, priced). Governance cost attributed honestly: bought by driver A (delivery) at `medium` rank on the user's explicit no-fallback instruction (record, "Roads rejected at the frame"). · **Mark:** the no-fallback ruling and the plugin-carries-the-log ruling **Confident** (user-ruled); the distribution basis **Contested** (record D4 — user-installed binary chosen against the lead's recommendation of committed prebuilt binaries, the user's reasons inferred and `Assumed`); the transition clause's validity **Assumed** until this run's validator grades it (record D10); the wave-3+ Testability rows dormant until the pilot (review I1)
  *Elicited from:* AM-1 — the user's setup invocation 2026-08-16 ("I want to retain the current way plugin is installed, cli is additional"); AM-2 — brainstorm `cli-schema-delivery` (accepted 2026-09-03: D3/D4/D10 user-ruled; F13 measured 2026-09-04), setup amend invoked by the user 2026-09-04, Card 1 ruled

- **GI-021 — Depth level declared: high (legacy default).** The production floor's depth level is `high` — set up under the single floor pre-adaptive-depth, already conformed to full depth (#7 fold, 2026-08-11). Minted at AM-1, discharging the ledger's "formal GI-row minting rides the next amend run" pointer. No ceremony; one-way ratchet applies (high never returns to low). · **Mark:** Confident
  *Elicited from:* ledger legacy-default line (2026-08-11); minting obligation discharged at the AM-1 review disposition (C1)

## Waivers

| GI-ID | Standard | Justification | Revisit trigger | Mark |
|-------|----------|---------------|-----------------|------|
| GI-008 | FLOOR-TEST as applied to the 6 helper scripts (1 bash, 5 python) | Scripts are thin standalone validators/detectors; no shared deps; testing/lint infrastructure absent and not worth erecting for them today | Script count grows, or a script becomes load-bearing in a shipped flow *(lead-composed, user-ratified at review fold S5, 2026-08-06)* | Confident |

(FLOOR-SEC secret-scanning is a **narrowing** recorded in GI-003, not a waiver.)

*AM-2 note (2026-09-04, Card 6):* GI-008 is **untouched** — its six are the skill-shipped helpers under `plugins/mochiko/skills/*/scripts/`. Its "script count grows" trigger was tripped by three repo-level checkers (`scripts/check-command-schema.py` v0.92.0 · `find-similar-rules.py` v0.99.0 · `check-skill-schema.py` v0.100.0) without a disposition; they were never waived (they rest on GI-019's advisory clause) and retire into the crate at wave 6 — moot then, recorded now.

## Module selections

| GI-ID | Module | Ruling | Because | Mark |
|-------|--------|--------|---------|------|
| GI-009 | knowledge-management (core) | **adopted — pin ratified** | Offered default-on at dimension 7; core already hand-pinned 2026-07-25 and live; this run is the pin's recorded revisit trigger, now discharged: pinned core ratified as the ruled core. All four existing root docs (`ROADMAP.md` · `BACKLOG.md` · `DECISIONS.md` · `ARCHITECTURE.md`) codified into module roles — analysis confirms semantics fit; **no collisions**. ARCHITECTURE.md deferral in the pin is **retired** (doc gained content); GLOSSARY.md deferral **carried** as recorded deviation — scaffold when it gains content. | Confident |
| GI-010 | knowledge-management elective: `CHANGELOG.md` | adopted | Release-shaped project (semver, marketplace) | Confident |
| GI-011 | knowledge-management elective: `RUNBOOK.md` | **declined — durable** | Nothing deployed, nothing operated | Confident |
| GI-012 | release-gates | adopted | User ruling at dimension 8 ("okay adopt, release gate"): codify what blocks a `plugin.json` bump — audits PASS · strip entries recorded · landing ritual complete · marketplace metadata synced (current 0.10.0-vs-0.53.0 lag becomes a tracked defect). *AM-1 (2026-08-16) dormant conditional:* when the template-schema Rust crate lands — `cargo test` PASS joins the blocking gates; schema-data/binary consistency joins the marketplace-sync gate; dormant until the crate exists (activated v0.76.0, ledger 2.0.1). *AM-2 (2026-09-04, Card 4):* gate 6 = `cargo test` PASS **plus the plugin contract suite's deterministic set green** (`python3 evals/contract/run.py`, sandbox, maintainer-side at every bump; GitHub CI keeps the crate layers only); gate 5's schema-data/binary clause becomes **derived view ≡ replay under the released binary range** (the crate's `views` + `fidelity` suites); the behavioural read-back metric is reported, never gating (record D8). **A SKIPPED suite (exit 3) is not green — it blocks the bump** until the suite runs (review I6, user-ruled). **Substrate caveat carried (review I7):** the suite runs on the Docker sandbox's stored consumer-subscription auth, a `Contested` ruling sustained against adverse Terms-of-Service evidence (kinako D8; record D8); the fact profile's "contractual commitments: none" stands — this is a third-party ToS exposure carried as a Contested mark on the gate's substrate, not a contractual commitment of the project and not a module trigger. **The crate's release train is gated too (review I8, user-ruled):** a `mochiko-cli-v*` tag MUST NOT land without the four crate layers green (`cargo test --all` · fmt · clippy `-D warnings` · `cargo audit --deny warnings`), the contract suite green against the tagged binary, and the render's head-and-tail output shape unchanged — or a coordinated `plugin.json` bump when it changes (the `.md` halt clauses key on that shape, which the grammar range does not version). | Confident (gates); the substrate mark Contested |
| GI-013 | layer-rules | **declined — durable** (AM-1 once-offer, 2026-08-16) | No layered architecture exists — prose library + one future crate; re-openable by explicit ruling if the Rust codebase grows layers. *(Was: not offered at v1.0.0, bookkeeping only.)* | Confident |
| GI-014 | evolution-notes | **declined — durable** (AM-1 once-offer, 2026-08-16) | The Rust/Tauri trajectory already lives in ruled homes — the D11 record, ROADMAP standing surface, BACKLOG build item; a separate evolution artifact would duplicate them (GI-017 pointer-only spirit). *(Was: not offered at v1.0.0, bookkeeping only; reviewer flagged newly-substantive given the Tauri trajectory — offered and declined with that in view.)* | Confident |

## Domain-dependency seeds

Not applicable — `layer-rules` not adopted.

## Deliberate exclusions (dimension 10)

- **GI-007:** Application-shaped enforcement machinery (CI pipelines, coverage gates, runtime health checks, log schemas) — excluded as *inapplicable in kind*, with each floor category retained via translated expression (GI-003–006), never dropped. Helper-script carve-out rides waiver GI-008, not exclusion. · **Mark:** Confident
  *AM-1 dormant note (review fold I2):* the "no runtime critical path" basis goes stale when the Rust crate lands — the CLI is a runtime with a critical path (emits authoring guidance). At the crate landing, GI-004/GI-007's inapplicability clauses and GI-002's tech-stack statement are re-expressed alongside the GI-012 gate activation — same dormant treatment, one consequence set.
- **AM-1 scope exclusion (review fold M1):** the GI-019 softening licenses **no general kernel** and **no orchestration/brain-code** — the recorded-ruling door plus the bright line are the whole grant; template conversion stays scoped to D3's 8 pipeline artifact templates, not all templates. A future reader must not read GI-019 as "kernel tooling is now generally fine." · **Mark:** Confident
  *AM-2 (2026-09-04, Card 2):* the template-scope limb is **discharged** — `cli-schema-delivery` D2/D10.5 fired `schema-based-template-guidance` D3's "later ratchet": the CLI's delivery role now covers every schema-class file (50) and every command's and skill's rules. The no-general-kernel / no-orchestration limbs stand unchanged.
  *AM-2 (2026-09-04, Card 5) — GI-007 narrowed:* the "inapplicable in kind" exclusion now covers **markdown primitives only**; the crate, the migration log, and the plugin path carry real executable gates (see Real commands). GI-004's audit ratchet for markdown primitives is unchanged; for the crate, author≠grader extends to code (every unit reviewed by a non-author seat — wave 1 precedent).

- **GI-022 — No feature map for mochiko's own repository (Card 8, user-ruled 2026-09-04, declined durable):** the amend run surfaced the missing `FEATURES.md` as the rule requires and offered a reconstruction; the user declined durably — mochiko's own planning surfaces are `ROADMAP.md` / `BACKLOG.md` / `DECISIONS.md` (the knowledge-management core), and the feature-map layer governs the product projects mochiko is used in, not the plugin repo itself. Re-openable only by explicit user ruling. · **Mark:** Confident
  *Elicited from:* `setup.fail.no-feature-map` (amend limb) at AM-2; ruled with the deck.

- **Store scaffold disclosure (Card 9, not a ruling):** the unconditional store scaffold (`setup.store-scaffold-unconditional`) creates `.mochiko/product/architecture/spine.md` (header only, `Scope:` line: developer tooling — a Claude Code plugin plus a Rust CLI; none of backend-service · frontend-web · mobile · desktop) and an empty `concerns.md` at this run's finalize; the store's ruled content is never authored here; override at the `/mochiko:architecture` desk by an ordinary store write.

## Confrontation rulings (brownfield)

- **GI-015 — Live token exposure:** `.claude/settings.local.json` holds a live `ANTHROPIC_AUTH_TOKEN`, untracked but absent from `.gitignore` — one `git add -A` from being committed. Ruling: MUST-fix in this session's finalize (add to `.gitignore`); folded into GI-003's expression. · **Mark:** Confident
- **GI-016 — Marketplace metadata lag:** `marketplace.json` at 0.10.0 vs plugin 0.53.0. Ruling: becomes a release-gates tracked obligation (GI-012); sync lands as a gate, not a one-off fix. · **Mark:** Confident
- **GI-018 — ARCHITECTURE.md version-lag accepted:** analysis inconsistency #3 (header cites v0.48.0 vs plugin v0.53.0) accepted as intentional — the doc updates only at component-changing landings, per its own contract; not a defect, not a release gate. · **Mark:** Confident

## Review

**2026-08-06 — first ratification**

- **Sizing:** lead stated weight at sizing time: 16 elements · marks all-Confident (audited; post-fold: 18 elements, 16 Confident + 2 bookkeeping) · reality-surface load moderate (brownfield analysis + live KM pin + two confrontations); default pair on first ratification; **lead sized: single** (solo seat, both lenses) — below default: departure reason is the session's narrow scope (KM-centered, empty arbitrated deck, no minted principles beyond the pointer-only ruling) — departure-trail line carried here.
- **Review:** solo reviewer, coverage + coherence lenses; **tally 9 raised → 6 merged survivors**; recommended status needs-revision.
- **Survivor dispositions:**

  | # | Sev | GI element(s) | Finding | Disposition |
  |---|-----|---------------|---------|-------------|
  | S1 | Medium | GI-013, GI-014 | "not offered" rows read as durable rulings, would foreclose amend offers | resolved — rows re-marked as bookkeeping, no ruling recorded, offerable on amend |
  | S2 | Medium | GI-017 | pointer-only directive lacked a GI-ID, invisible to trace string-match | resolved — minted as GI-017 with elicitation quote |
  | S3 | Minor | GI-001 | no-UI negative (a11y trigger) never named | resolved — user confirmed per-negative at fold |
  | S4 | Minor | GI-004–006 | dropped floor sub-clauses vanished implicitly | resolved — subsumed sub-clauses named per row |
  | S5 | Minor | GI-008 | revisit trigger lead-composed, not user-ruled | user-ruled — trigger ratified as user's own, provenance noted |
  | S6 | Minor | (analysis #3) | ARCHITECTURE.md version-lag uncarried | resolved — GI-018 accepts as intentional |

- **Verify pass:** PASS — all six folds confirmed by the sole reviewer from disk (2026-08-06); one sizing-line figure nit fixed post-verify.

## Amendment Log

**AM-3 — 2026-09-14 — mechanical conformance gates on artifact writes: GI-019 admission
widened (clause iv), `cli-schema-delivery` D7 narrowly superseded** *(driver: brainstorm
`hook-enforced-artifact-schema` D1–D11 as review-amended, accepted 2026-09-13; wave 0 probe
PROCEED; wave 1 crate built and accepted 2026-09-13; setup amend invoked by the user 2026-09-14
with the lead-drafted proposal
`.mochiko/brainstorms/hook-enforced-artifact-schema/wave2-amendments.md` as input)*

- **Scope (deck of 8, all user-ruled "as recommended" 2026-09-14):** semver MINOR (Card 1) ·
  CLAUDE.md kernel-class trace comment + region GI-019 pointer line gain the conformance-gate
  admission pointer, Ratified line → v3.1.0; GI-020 paragraph untouched (Card 2) · ledger
  GI-019 gains the admission ruling, clause (iv), the re-ratified hook floor with the
  explicit-allow rule, and the scope-of-guarantee clause, plus a **Testability re-key** the
  proposal omitted (Card 3) · `.claude/rules/mochiko/rust-cli.md` bright-line bullet
  rewritten as a recorded supersession with the verbatim prior bullet preserved in the ledger
  GI-019 entry, no `.mochiko/strips/` entry (Card 4) · GI-020 ↔ D7e reconciled: a
  hooks-disabled consumer stays outside GI-020's supported set, D7e grants no support (Card 5)
  · GI-002 risk-surface note in this synthesis (Card 6) · untouched: GI-020 · GI-004/005/006 ·
  `producer-plan-enforcement` D1; GI-012 untouched but for the one wave-4 precondition clause
  ruled at review C5 (Card 7 as narrowed) · review sized single (Card 8) · **lead addition outside
  the deck, disclosed (verify R3):** the ledger's amendment-policy MINOR limb gains "principle
  significantly expanded" so the ledger and `validation-constitution.version-bump` (which already
  reads it) agree — the ledger is the narrower of two surfaces that already disagree.
- **Lead checks before the deck:** `EXIT_CONFORMANCE = 4` at `crates/mochiko-cli/src/hook.rs:37`
  and `mochiko-cli check --hook-json -` present in the built crate · record D7(c)'s "every home
  `Read`" wording is superseded by D1(b)'s `SubagentStart` amendment (the proposal uses the
  amended form) · store scaffold present with its `Scope:` line, nothing written · feature map:
  GI-022 declined durable, the amend limb satisfied by the KM core, not re-asked · no
  `constitution.md` on disk.
- **Semver:** governance surface set v3.0.3 → **v3.1.0** (MINOR — widened admission, the
  non-negotiable's text unchanged; user-ruled Card 1 with the MAJOR reading in view: GI-019's
  Testability rows flip meaning and the `rust-cli.md` hook clause changes meaning). **Re-put at
  review C6 and re-ruled MINOR 2026-09-14 — a deliberate departure, recorded:** AM-1 and AM-2
  both conceded "meaning changes" and ruled MAJOR, and the ledger's MINOR limb ("new principle
  or waiver change") names neither of AM-3's shapes; the departure's reason is that those two
  redefined a principle (absolute ban → admissible by ruling; degraded path → none) while AM-3
  widens an admission under an unchanged principle — the validator's bump grammar's "principle
  significantly expanded" — and the GI-019 Testability inversion is read as re-keying a
  formulation narrower than the principle, not a redefinition. The ledger's MINOR limb gains
  the "significantly expanded" wording at this amend so policy and grammar agree.
- **Build state at ratification (review C8):** wave 0 PROCEED · wave 1 crate built and
  accepted 2026-09-13 (`89139a8`, `c395dab`), unpublished — the installed binary is the
  maintainer break-glass build · wave 3's migration `plugins/mochiko/migrations/0005-artifact-
  homes.yaml` authored untracked in the working tree ahead of D11's order, reviewed PASS, its
  budget table (`wave3-budget-table.md`) awaiting the user's ratification · nothing shipped;
  `plugin.json` 0.108.0.
- **Violator pass for this repo (review C10, user-ruled 2026-09-14):** mochiko's own `.mochiko/`
  tree takes the same violator pass kinako takes at wave 5, scheduled here **before the wave-4
  plugin is installed locally** (the gate starts biting for this repo at the maintainer's local
  install, not at the commit) — a D11 build note (build log + BACKLOG), not governance; this synthesis's own shape drift
  against the `governance-intent` template (`## Depth level declaration` absent, the
  `## Real commands` heading suffixed, `## Confrontation rulings (brownfield)` undeclared) is
  fixed there.
- **Review:** solo cold intent review via blind-map two-message dispatch (28-angle map,
  topic-only spawn; `.mochiko/` fenced until the map returned — one declared leak: a haiku
  explorer's grep returned line fragments from the top-level operating docs, judged
  immaterial; sizing: single, user-ruled Card 8 — narrow amend, one driver record, every ruling
  already user-ruled at the brainstorm; departure from the default pair carried here as the
  trail line). Verdict **critical-gaps** — 28 angles worked, 10 survived, four emergent outside the map (C2,
  C5, C9, C10) (C1 clause (iv)'s "not a
  pipeline gate" unconditional while OQ1 is open · C2 amnesty under-stated against D4e and the
  built binary · C3 mark not mirroring the record · C4 the gate's reach boundary dropped · C5
  the two owed supply-chain controls untouched by the widened vector · C6 MINOR against two
  MAJOR precedents · C7 what the gate reads unstated · C8 build-state disclosure short · C9 the
  60 s cap out of governance · C10 no violator pass for this repo). **Dispositions (2026-09-14):**
  C1 user-ruled road (a) — OQ1 carried as a stated condition on clause (iv), ruled at the wave-3
  table · C2/C3/C4/C7/C8 lead-repaired and folded above · C5 user-ruled yes — the wave-4 hook
  ship gated on all four controls · C6 user-ruled MINOR stands as a recorded departure · C9
  user-ruled watch by ruling · C10 user-ruled violator pass, scheduled here ahead of the local wave-4 install (R4); all five rulings "as
  recommended". Report:
  `.mochiko/brainstorms/hook-enforced-artifact-schema/wave2-reports/intent-review.md`. *Verify
  pass:* round 1 **NOT CLEAN** 2026-09-14 — 8/10 folds confirmed faithful from disk, 5 residuals
  (R1 the Card 7 clause still calling GI-012 untouched · R2 the amnesty clause over-corrected against
  the binary — path is not a relaxable measure, a file-set violation rides a bare allow, the framing
  sentence misaligned · R3 the lead addition absent from the Scope bullet · R4 the violator-pass
  sequencing impossible as written · R5 the tally dropping the four emergent survivors), all
  lead-repaired in place; the seat's self-correction (C9 tagged emergent) disclosed; delta-check
  **NOT CLEAN** — R1–R5 confirmed landed from disk (R2's budget limb measured at this pass too: a
  standing 738-line `record.md` rides amnesty with its overage named); one Minor residual (R6 —
  the Dispositions sentence still saying "at wave 5" behind R4) lead-repaired in place, closed
  seat-unverified, disclosed. Review closed 2026-09-14
- **Ratified:** 2026-09-14 by Deepesh — after the delta-check over the five repaired residuals
  (R6 lead-repaired, review closed) ("ratified", plain text); no surface authored before this line.
- **Accepted:** 2026-09-14 by Deepesh ("accept") — surface set v3.1.0 authored by the producer
  seat (`tech-lead`, `mochiko:authoring-constitution`) on a lead-approved plan (formulation calls
  FC-1…FC-4 approved), graded by an independent `validation-constitution` seat: round 1 **FAIL**
  (B1 blocking — no rules file scoped the bump surfaces `plugin.json` / `marketplace.json` /
  `CHANGELOG.md` where AM-3's strictest MUST NOT fires; M1 the conformance Testability limb
  unassertable before wave 4; M2 the one-shot precondition with no expiry route) → fix round →
  **PASS 60/61** (one n-a by recorded decline) → two non-blocking residuals taken (log row
  completeness; the region read line naming the new globs) → delta-confirm round 3 **PASS, no
  residuals**; MINOR bump agrees. **Six flagged items accepted as authored:** FP-1 the ≤ 60 s
  cost cap on GI-012's gate note as a watch · FP-2 GI-012 Testability limbs · FP-3 the
  `rust-cli.md` log-is-truth record qualifier (lead-ruled at the plan gate) · FP-4 the trace
  summary's superseded-rounds pointer · FP-5 `rust-cli.md` `paths` += the three bump-surface
  files · FC-4 GI-012 trace line extended; plus two new **pre-authorized PATCHes** (the
  conformance Testability limb activates at the wave-4 bump; the wave-4 precondition clause is
  struck after that bump lands). Owed after this run, outside it: the ledger's MAJOR limb
  lacks the template's depth-flip clause (next PATCH) · signed release tags + the `crates-io`
  approval rule before the first publish, now also gating the wave-4 hook ship · this repo's
  violator pass before the local wave-4 install · the wave-4 fix candidate (name a standing
  file-set violation in `additionalContext`). `floor: tripped · seats: intent-reviewer
  (devils-advocate, cold) / gov-producer (tech-lead) / gov-validator (validator)`.

**AM-2 — 2026-09-04 — clone-only install with a required `mochiko-cli` dependency; no
file-read fallback** *(driver: brainstorm `cli-schema-delivery` D1–D11, accepted 2026-09-03;
wave 0 probes + wave 1 crate built 2026-09-04; F13 measured; setup amend invoked by the user
2026-09-04 with the lead-drafted proposal
`.mochiko/brainstorms/cli-schema-delivery/wave2-amendments.md` as input)*

- **Scope (deck of 11, all user-ruled "as recommended" 2026-09-04):** GI-020
  **superseded-by-ruling** (clone-only kept; required binary; measured halt; transition
  clause until wave 6; unsupported environments) · GI-019 widened admission recorded with
  the three D11 clauses, this session named as the admission, the three repo-level checkers
  placed under the advisory clause and retiring at wave 6; AM-1's template-scope limb
  discharged · GI-002 identity re-expressed and risk surface made live (required binary +
  shipped hooks; first-public-release trigger fired, its discharge **conditional** on all
  four named controls existing at the first publish — two present, two owed as wave-2 tail
  obligations; access-loss class; no module) · GI-012 gate 6 + contract suite, gate 5 = view ≡ replay ·
  GI-004/GI-007 re-expressed with the real commands · GI-006 wording gains the migration
  log · GI-008 untouched, trigger observation noted · **GI-022 minted** (no feature map for
  the plugin repo, declined durable) · store scaffold disclosed ·
  `.claude/rules/mochiko/rust-cli.md` full rewrite with `paths` widened to `migrations/**`
  (the log's home until wave 3), `plugins/mochiko/migrations/**` (its ruled home from wave
  3, review C1) and `evals/contract/**`, carrying the maintainer break-glass line (`cargo install --path
  crates/mochiko-cli`; review M2) · CLAUDE.md non-negotiable paragraph reworded per
  proposal §A · **consequence set beyond the governance surfaces (review I9):** `README.md`
  (install section, the stale `--schemas-dir`/embedded-copy CLI notes) contradicts the
  ratified posture and is re-authored at the **wave-3 landing** (user-facing documentation,
  owner: the wave-3 build; not a governance surface, so outside this run's producer).
- **Semver:** governance surface set v2.0.1 → **v3.0.0** (MAJOR — a non-negotiable's
  meaning changes; user-ruled, Card 11).
- **Review:** solo cold intent review via blind-map two-message dispatch (37-angle map,
  topic-only spawn; `.mochiko/` fenced until the map returned; sizing: single, lead-ruled —
  narrow amend, one driver record, every ruling already user-ruled at the brainstorm;
  departure from the default pair carried here as the trail line). Verdict
  **critical-gaps** — 37 raised, 14 survived (C1 the log does not ship in the plugin and no
  ruling said where it would · C2 marks upgraded past the record · I1 Testability rows
  unassertable on ratification day · I2 GI-004 audit-unit change unrecorded · I3 GI-005's
  mechanization filed on GI-006 · I4 Windows/PowerShell unsupported limb dropped · I5 two of
  four named controls absent · I6 a SKIPPED contract suite could pass a gate · I7 the ToS
  caveat uncarried onto GI-012 · I8 the crate's release train ungoverned · I9 README
  contradicts the ratified posture · M1 no revisit trigger on GI-020 · M2 no break-glass ·
  M3 `cargo audit` misfiled). **Dispositions:** C1 user-ruled 2026-09-04 — **"plugin
  carries it"**: the log ships at `plugins/mochiko/migrations/` from wave 3, priced, GI-002
  and GI-020 reworded to the ruled end state · C2 lead-repaired — marks split mirroring
  the record (D4 `Contested`, transition `Assumed`) · I1–I9, M1–M3 **user-ruled "as
  recommended" 2026-09-04**, four rulings inside: I2 the wave-6 `primitive-edits.md` re-key
  is a pre-authorized PATCH activation · I5 signed tags and the manual-approval environment
  are wave-2 tail obligations gated on the first publish, the risk trigger discharging only
  when all four controls exist · I6 a SKIPPED contract suite blocks the bump · I8 the
  crate's release train is gated (four crate layers · contract suite against the tagged
  binary · output shape unchanged or a coordinated plugin bump); folds landed: I1 dormant
  Testability rows · I3 GI-005 schema-rule limb · I4 PowerShell-only Windows unsupported,
  no-build-step scoped to the plugin · I7 ToS carried onto GI-012 as a Contested substrate
  mark · I9 README at the wave-3 landing · M1 pilot abort criteria as GI-020's revisit
  trigger · M2 break-glass line · M3 `cargo audit` on its own row. Report:
  `.mochiko/brainstorms/cli-schema-delivery/wave2-reports/intent-review.md`. *Verify pass:*
  **CLEAN** 2026-09-04 — 14/14 folds confirmed by the same cold seat from disk, no fold
  misapplied, no new contradiction, no mark upgraded past the record; three wording nits
  lead-repaired in the same pass (pricing figures in KiB over 50 files and the 604 KiB log ·
  the rust-cli.md `paths` naming the log's ruled home · the Scope bullet's "discharged" made
  conditional).
- **Ratified:** 2026-09-04 by Deepesh — after the CLEAN verify pass over the 14 folded
  dispositions ("ratified", plain text); no surface authored before this line.
- **Accepted:** 2026-09-04 by Deepesh ("accept all") — surface set v3.0.0 authored by the
  producer seat (`tech-lead`, `mochiko:authoring-constitution`) on a lead-approved plan with
  two content-pinned lead corrections (end-state tense for the log's home; region
  pointer-only), graded **PASS 60/61** (one n-a by recorded decline) round 1 by an
  independent `validation-constitution` seat with trace closed both directions, six
  wording advisories taken → delta-confirm PASS; **five flagged proposals accepted as
  authored** (FP-1 `rust-cli.md` paths += `.github/workflows/**` · FP-2 += `plugins/mochiko/
  hooks/**` · FP-3 no GI-002 Three-Part entry, AM-1 precedent · FP-4 GI-022 region index
  line · FP-5 GI-004/GI-005 region pointer clauses) and the A4 residual ruled: the
  `primitive-edits.md` glob additions (`plugins/mochiko/migrations/**`, `hooks/**`) are a
  **wave-3 obligation**. Owed after this run, outside it: signed release tags + the
  `crates-io` environment approval rule before the first publish (GI-002 discharge
  conditional) · `README.md` re-authored at the wave-3 landing · GI-011's ledger home at the
  next PATCH. `floor: tripped · seats: gov-producer (tech-lead) / gov-validator (validator)
  + intent-reviewer (devils-advocate, cold)`.

**AM-1 — 2026-08-16 — no-kernel softening + additive CLI** *(driver: brainstorm
`schema-based-template-guidance` D1–D11, accepted 2026-08-16; setup amend invoked by the
user same day)*

- **Scope (as amended at review):** GI-019 minted (D11 bright line — kernel-class tooling
  by recorded ruling only, never gates / never orchestrates agents / never holds
  skill-owned judgment; kernel-class defined by the output-dependency test, the 6 existing
  scripts placed outside it) · GI-020 minted (additive-CLI install constraint,
  user-declared) · GI-021 minted (depth level `high`, legacy default — ledger pointer
  discharged) · GI-002 identity annotated (the "kernel-free" identity phrase superseded by
  the D11-softened position; markdown-first identity survives) + risk surface extended
  (shipped-executable/supply-chain vector interrogated, no module now, revisit triggers
  named) · GI-012 gains a **dormant conditional clause** (activates when the Rust crate
  lands: `cargo test` PASS joins the blocking release gates; schema-data/binary
  consistency joins the marketplace-sync gate) · **crate-landing consequence set widened
  (review fold I2):** GI-002 tech-stack statement + GI-004/GI-007 inapplicability clauses
  re-expressed at the same landing, same dormant treatment; ledger CI-arrival trigger
  scope widened to match · GI-003's secret-scanning revisit trigger noted as expected to
  fire there — no change now · GI-013/GI-014 once-offer run: both **declined durable** ·
  AM-1 scope-exclusion line added (no general kernel, no orchestration) · CLAUDE.md prose
  rewording (core-bet sentence + no-kernel non-negotiable) user-approved to land **in this
  run**, ahead of the build, per the user's timing ruling.
- **Semver:** governance surface set v1.0.0 → **v2.0.0** (MAJOR — a non-negotiable's
  meaning changes; user-ruled).
- **Review:** solo cold intent review via blind-map two-message dispatch (27-angle map,
  topic-only spawn; synthesis + ledger withheld until the map returned). Verdict
  **critical-gaps** — 12 raised, 5 killed on read, 7 survived (C1 depth-row unminted ·
  I1 kernel-class undefined/scripts unplaced · I2 half-sequenced crate-landing set ·
  I3 stale risk surface/supply-chain uninterrogated · I4 module once-offer skipped ·
  M1 missing scope exclusion · M2 bare Confident mark). 7/7 dispositioned 2026-08-16:
  I4's two offers + I3 user-ruled individually, C1/I1/I2/M1/M2 one user-ruled batch "as
  recommended"; all folds landed in this synthesis. Lead re-verified the C1 ledger quote
  against the file before disposition.
- **Ratified:** 2026-08-16 by Deepesh — after the CLEAN verify pass over the folded
  dispositions; 4 non-blocking verify nits carried to the producer checklist (header
  version line · ledger touch set · GI-002 tech-stack re-expression home · optional
  "today" tightening).
- **Accepted:** 2026-08-16 by Deepesh — surface set v2.0.0 authored by the producer seat
  on a lead-approved plan (two content-pinned amendments: named-species retention ·
  Workflows/agent-teams sentence), graded PASS round 1 by an independent
  `validation-constitution` seat (trace closed both directions; 3 non-blocking
  advisories: ROADMAP thesis line closed at this same acceptance · index-line marker
  left per GI-017 precedent · trace manifest served by distributed stamps).
  `floor: tripped · seats: gov-producer (tech-lead) / gov-validator + intent-reviewer`.
