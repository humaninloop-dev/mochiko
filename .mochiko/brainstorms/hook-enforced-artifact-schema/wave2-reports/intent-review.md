# AM-3 — cold intent review of the frozen governance synthesis

**Reviewer:** solo cold intent reviewer (`mochiko:review-governance-intent`), spawned by the
blind-map two-message protocol — message one carried the amend topic and the allowed surface
list only; the synthesis path, the ledger, the driver record, and everything else under
`.mochiko/` were withheld until the 28-angle map returned.
**Artifact reviewed:** `.mochiko/memory/governance-intent.md` (frozen; the AM-3 delta marked in
place — the `**Governs:**` line, the `*AM-3 (2026-09-14, Card 6)*` note under GI-002, the
`*AM-3 (2026-09-14, Cards 2–5)*` paragraph under GI-019, and the `**AM-3 — 2026-09-14 …**`
amendment-log entry).
**Lens:** single seat, both lenses (coverage and coherence); the verify pass is this seat's too.
**Date:** 2026-09-14. **Single writer of this file.**

**Status recommended: `critical-gaps`.**
**Tally: 28 angles worked → 10 survived** (1 Critical · 5 Important · 4 Minor). Twenty mapped
angles were killed on read and are listed with their reasons at the end; four survivors emerged
during the cold read outside the map and are marked as such.

The verdict is the lead's; this report is input. Nothing here was authored into, or removed
from, any governance surface. Findings enter through the lead's pen.

**Fact-route disclosure.** One survivor (C6, semver class) rests on a user-declared fact and is
flagged for the lead to route to the user as a consequence-stated confirmation, never argued.
Platform claims about the Claude Code hooks surface were not argued — they are taken as the
driver record pins them (F8, doc-quoted and re-verified at the brainstorm's own review) and as
the wave-0 probe measured them. The brownfield analysis at `.mochiko/memory/codebase-analysis.md`
was read beside the synthesis; it is dated **2026-08-06**, predating the crate, the migration
log, and every hook, so it grounds only the fact-profile negatives (no services, no DB, no user
data) that the AM-3 delta leaves untouched.

---

## 1 — The Phase 0 blind angle map

Built before the synthesis path was known, from the amend topic plus the fence's allowed
surfaces: `CLAUDE.md`, `.claude/rules/mochiko/*.md`, `crates/mochiko-cli/` (source and tests),
`plugins/mochiko/` including `hooks/` and `migrations/`, `evals/contract/`,
`.github/workflows/`, both manifests, and
`plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md` as the
coverage yardstick. Twenty-eight angles in three groups.

**A — must-cover (agenda surface).**
A1 amend-scope justification and which agenda slice ran · A2 dimension 4, the cost of a wrongly
denied write as an elicited fact · A3 dimension 5, who unblocks a false deny under a solo
maintainer · A4 dimension 6, the gate's own regression surface against the CI path filter ·
A5 dimension 8, whether the gate acquires a release gate · A6 dimension 10, the earlier decline
of behavior-gating hooks confronted with its rationale · A7 the once-per-amend unruled-module
offer · A8 the waiver sweep · A9 the depth-level element's survival · A10 the scope fact — what
the gate actually gates · A11 the `SubagentStart` reminder's own justification.

**B — must-not-contradict (coherence).**
B12 the `rust-cli.md` sentence named and superseded in terms · B13 GI-019's "never gates
pipeline progress" · B14 a stated test for the mechanical/judgment line · B15 the advisory
post-hoc carve-out · B16 the non-negotiable supersession route and its ruling anchor ·
B17 `PowerShell` in the matcher against the declared-unsupported limb · B18 GI-020's dependency
widening and the halt-never-degrade clause · B19 GI-017, a reminder that restates rules ·
B20 the crate side already built · B21 mark discipline against echo-rationales.

**C — must-price.**
C22 the reminder's removal test · C23 blast radius in a consumer's own repo · C24
contract-suite perturbation · C25 landing cost, strips and the audit · C26 semver against the
MAJOR precedent · C27 the external-claims route · C28 the too-thin bar.

---

## 2 — Survivors

### Critical

#### C1 — Clause (iv) ratifies "not a pipeline gate" as settled, while the open question it rests on is still open in the driver and uncarried into the synthesis

**Severity:** Critical. **Element:** GI-019 (the AM-3 conformance-gate admission, clause iv);
touches the standing bright line in `CLAUDE.md ## Non-negotiable constraints`.

**The contradiction, cited.** The synthesis records clause (iv) unconditionally: the gate is
"not a pipeline gate: a denied write is re-emitted, the run continues, the two-strike halt
sentence is advisory — D9." That sentence holds only where a conforming draft exists. The
driver record leaves open whether one always does. Its **OQ1** reads: "Section budgets for
prose-shaped deliverables. A brainstorm `record.md` or a governance-intent synthesis has
sections whose honest length varies by session; the D6 table must decide per kind whether such
sections carry a budget or a `max_lines: none` disclosure — ruled at the census table, not
here." OQ1 is the premise clause (iv) stands on, and the synthesis carries no trace of it.

**Why no in-run route exists today.** Three of the four exits are closed by ruling:

- **Amnesty does not reach a new file.** D4e grants first-touch amnesty on a write *over an
  existing file*; "a new file at a declared name takes the budget outright."
- **The `reports/` route is closed to deliverables.** D2 as amended at V4 opens `reports/` only
  to content that honestly carries a `report:` type from the envelope's enum; a brainstorm
  record is a deliverable, not a report.
- **The upstream route is not an in-run route.** D2's own deny text says a new deliverable kind
  "takes a migration in the plugin's log and a plugin release."
- The remaining exit is `disableAllHooks`, which per D7e drops the project outside GI-020's
  supported set.

**Measured evidence that this is the expected case, not an edge.** The homes migration already
authored in the working tree (`plugins/mochiko/migrations/0005-artifact-homes.yaml`, untracked)
gives the brainstorm session home a whole-file bound:

```
      home: brainstorm-session
      path: [.mochiko, brainstorms, <slug>]
      bounds: whole-file
      deliverables:
        - file: record.md
          max_lines: 150
```

Against that budget, in this repository today:

| Measure | Value |
|---|---|
| Brainstorm records over 150 lines | 43 of 66 |
| Largest record (`cli-schema-delivery`) | 1,536 lines |
| This amend's own driver record | 738 lines |

D6 states the posture deliberately: budgets are "tight … never derived from kinako's observed
sizes," and "the first dogfood run is expected to trip denies on **new** writes, which is the
signal working."

**Failure scenario.** Wave 4 ships. A `/mochiko:brainstorm` run opens in a gated repository and
reaches a genuine 400-line decision record. The first `Write` of `record.md` is denied on the
whole-file bound. `Edit` cannot help — there is no baseline to be amnestied against. A `Bash`
heredoc is denied by D1c. Splitting into `reports/` is refused because the content carries no
report type. Changing the budget takes a migration and a plugin release. The seat's only
remaining move is to disable every hook in the project, which forfeits the supported set. The
run stops, and the mechanism that stopped it is the one clause (iv) certifies cannot stop a run.

**Resolution path — the one question.** Does a new deliverable whose honest content exceeds its
declared budget have an in-run route, or does the run halt until a plugin release changes the
table? Either carry OQ1 into the synthesis as a stated condition on clause (iv) — the admission
ratified, the "not a pipeline gate" claim conditional on the census table giving prose-shaped
deliverables `max_lines: none` or an equivalent route — or rule the route now (a create-time
advisory rather than a deny for whole-file bounds would do it, and would leave every other check
hard).

---

### Important

#### C2 — The synthesis scopes first-touch amnesty to line budgets; the built binary applies it to shape as well *(emergent, outside the map)*

**Severity:** Important. **Element:** GI-019 (AM-3 admission paragraph, the amnesty clause and
its framing sentence).

**The contradiction, measured.** The synthesis lists the checks as "… declared placeholder
tokens · **per-section line budgets with first-touch amnesty on any write over an existing file
(D4e)** · a shell command carrying a write operator …" — amnesty attached to the size check
alone. D4e in the record grants it over checks (a)–(d), headings included. The built crate
follows the record, not the synthesis. Run against the live artifact under review:

```
$ mochiko-cli check --hook-json - --plugin-root plugins/mochiko   # Edit payload,
                                                                  # .mochiko/memory/governance-intent.md
{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow",
"additionalContext":"This file already stood outside its declared shape before this write, and
this write does not worsen it, so it is allowed. Standing:\n- the required heading
`## Depth level declaration` is absent.\n- the required heading `## Real commands` is absent."}}
```

**The second half of the same paragraph then overstates the gate.** "The gate holds one write
until it conforms, and a passing draft pays nothing" is false for any file carrying a standing
violation: the gate holds nothing, allows the write, and prints an advisory. The ledger entry is
the durable record of what was admitted, and it currently understates the amnesty and overstates
the reach.

**Failure scenario.** A future reader of GI-019 — the next amend's producer, or the validator
grading the authored surface — reads that headings are hard-denied on existing files and that
the gate holds until conformance. Both are wrong about the shipped mechanism, and the surface
set authored from this synthesis inherits both errors.

**Resolution path.** One check: restate the amnesty over checks (a)–(d) as D4e writes it, and
qualify the framing sentence to a conforming baseline.

#### C3 — The AM-3 mark does not mirror the record, repeating AM-2's own survivor C2

**Severity:** Important. **Element:** GI-019 (AM-3 mark line).

**The contradiction, cited.** The element carries one mark: "Confident (user-ruled deck of 8,
all 'as recommended' 2026-09-14; …)". The driver record marks its pieces unevenly. **D3** — the
source of the exit-code contract, the `check --hook-json -` interface, and the `home` document
kind the synthesis records as the explicit-allow rule — is marked **`Assumed`**: "derived from
GI-020 + `cli-schema-delivery` D1; **not put to the user as a fork** — surfaced for acceptance."
**D7 through D11** are marked "Confident (drafted by the lead as derived; user-confirmed as
drafted at Q7)," and Q7's question was "confirm D7–D11 as drafted." The synthesis's own words
concede the shape: the explicit-allow rule is "a lead repair inside D7's fail-open intent,
disclosed at the brainstorm's acceptance."

An eight-for-eight "as recommended" deck is an adoption streak, and this skill's floor holds
that echo-rationales and adoption streaks outrank any mark. AM-2's review raised the identical
finding as its survivor **C2** ("marks upgraded past the record"), dispositioned by splitting the
mark to mirror the record — `Contested` on D4's basis, `Assumed` on the transition clause.
AM-3 does not carry that precedent forward.

Two parts of the deck genuinely are user-arbitrated and should keep `Confident`: R1 (the user
chose the deny-now road against the review's declare-and-measure road with both costs named),
R2b (no consumer-local surface), and R3 (the Bash matcher, "yes").

**Failure scenario.** The one part of the design the user never saw as a fork — the exit-code
contract that decides which of the CLI's five exit codes may deny a consumer's write — is
recorded at the same confidence as the rulings the user arbitrated. A later reader auditing why
exit 2 passes through has no signal that the answer was derived, not chosen.

**Resolution path.** Split the mark as AM-2 did: `Confident` on the admission and the
user-arbitrated rulings (R1 · R2b · R3); `Assumed` on the D3-derived exit-code contract and
interface, with the reason the record gives.

#### C4 — The admission records what the gate denies and never records what it leaves alone

**Severity:** Important. **Elements:** GI-019 (AM-3 admission), GI-002 (risk surface).

**The gap.** For a mechanism that ships to every consuming project and fires on the consumer's
`Write`, `Edit`, `Bash`, and `PowerShell` calls, the reach boundary is the sentence a consumer
most needs, and the synthesis has none. D9 states it and the synthesis drops it: a `Write`
outside every declared home is gated **only** when its content opens with mochiko report
frontmatter or a template's `## Header` signature — "a plain `.md` elsewhere is not mochiko's
business (product repos have their own docs)" — and `NotebookEdit` plus any MCP file writer a
consumer enables are **knowingly outside** the matcher set.

D7e's scope-of-guarantee clause, which the synthesis does carry, answers a different question:
what a hooks-disabled consumer keeps. It says nothing about what a hooks-enabled consumer's
ungated files are.

**Failure scenario.** A consumer reads the ratified admission, sees a `PreToolUse` gate on
`Write|Edit|Bash|PowerShell` with a deny power that holds in `bypassPermissions`, and concludes
the plugin gates every markdown write in their repository. The governance surface gives them
nothing to correct that, and the honest answer — a narrow declared tree plus a frontmatter sniff
— is the strongest argument the admission has.

**Resolution path.** One check: carry D9's reach boundary into the GI-019 admission beside the
scope-of-guarantee clause — the declared homes, the frontmatter sniff, and the knowingly
uncovered writers.

#### C5 — The widened vector raises what two still-owed supply-chain controls protect, and the conditional discharge is not re-touched *(emergent, outside the map)*

**Severity:** Important. **Elements:** GI-002 (risk surface, the first-public-release trigger),
GI-019 (AM-3), GI-012 (the crate release train).

**The contradiction, cited.** AM-2 fired GI-002's first-public-release trigger and made its
discharge conditional on four named controls, two of them owed. Both are still owed in the tree
today:

| Control | State |
|---|---|
| `cargo audit --deny warnings` in CI | present |
| sha256-published release assets | present |
| `cargo publish` behind manual approval | owed — the job is `if: false` at `.github/workflows/release.yml:100` |
| signed release tags | owed — no signing exists in the repo |

AM-3 records the vector widening and rules "Fact profile unchanged; no module; no surface
change — synthesis note only (user-ruled)." What changes and is not priced is the **kind** of
authority the installed binary holds. Until AM-3 it renders and halts; after AM-3 it denies a
consumer's writes in every permission mode, `bypassPermissions` included. Meanwhile D11 makes
**wave 1's exit condition the crate's first publish** — precisely the event the two owed
controls gate.

**Failure scenario.** The crate publishes to satisfy wave 1's exit with the approval rule and
tag signing still absent. A consumer installs it. The binary now holds deny authority over that
consumer's artifact writes, delivered through an unsigned release from an unapproved publish
path, and the recorded risk analysis for that authority is a note that says nothing changed.

**Resolution path.** One question to the user: does the widened vector change the two owed
controls' standing — do they become hard preconditions of the **wave-4 hook ship**, not only of
the first publish, given that wave 4 is where deny authority reaches consumers?

#### C6 — MINOR is ruled on "the text is unchanged" while the same bullet concedes the meaning changes, against two MAJOR precedents on the identical phrase *(user-declared fact — route, do not argue)*

**Severity:** Important. **Elements:** the AM-3 amendment-log Semver line; the ledger's
amendment policy; the amendment log's AM-1 and AM-2 rows.

**The contradiction, cited.** The ledger's policy reads: "MAJOR — principle removal /
**incompatible redefinition** / floor-level change / module attach or detach · MINOR — **new
principle or waiver change** · PATCH — clarification." AM-3 mints no principle and changes no
waiver, so the MINOR limb does not describe it. The AM-3 Semver line rules MINOR on the ground
that "the non-negotiable's text [is] unchanged," then concedes in the same parenthesis:
"GI-019's Testability rows flip meaning and the `rust-cli.md` hook clause changes meaning." Both
prior amendments were ruled **MAJOR** on that exact ground — the log's own rows read "AM-1 …
(MAJOR: a non-negotiable's meaning changes; user-ruled)" and "AM-2 … (MAJOR: a non-negotiable's
meaning changes — user-ruled)."

The ledger's own Testability row for GI-019 is the concrete thing flipping: it currently passes
only if "every shipped hook blocks only on the binary's absence or a log outside its grammar
range," and fails on "a shipped hook that blocks on anything other than the binary's absence or
grammar skew." After AM-3 the same hook passes. That is a Pass/Fail inversion on a
NON-NEGOTIABLE's test.

**This is a user-declared fact.** Card 1 was user-ruled "with the MAJOR reading in view." It is
checkable against nothing on disk and is not argued here.

**Resolution path.** Route to the user as a consequence-stated confirmation, naming the one
thing the card may not have had on the table: both prior amends made the identical concession
and were ruled MAJOR on it, and the policy's MINOR limb covers neither a new principle nor a
waiver change. If the user re-rules MINOR with that in view, the ruling stands and the departure
should be recorded as a deliberate one.

---

### Minor

#### C7 — The risk note states execution frequency but not what the hook reads

**Severity:** Minor. **Element:** GI-002 (AM-3 risk-surface note).

AM-2 priced its vector precisely: the binary is required, the hooks "execute on every consumer's
machine at every session start and at every mochiko fire … under a 5-second timeout, fail-open
by platform design." AM-3's note widens the frequency — "on **every gated artifact write** in a
consuming project … and once per spawned seat" — but never says what crosses the boundary. Per
D3 the wrapper pipes the raw `PreToolUse` payload to the binary, which parses `tool_input.content`,
`old_string`/`new_string`, and `command`; on `Edit` the CLI additionally reads the on-disk file to
apply the edit in memory. The plugin author's code therefore sees the full text of every gated
write and the files it edits. Nothing leaves the machine, which is why this is Minor and not a
fact-profile question — but it is the sentence a security-minded consumer reads the risk surface
for.

**Resolution path.** One line in the AM-3 note stating what the gate reads.

#### C8 — The build-state disclosure is less complete than AM-2's own precedent

**Severity:** Minor. **Element:** the AM-3 amendment-log header.

The header discloses "wave 0 probe PROCEED; wave 1 crate built and accepted 2026-09-13." It does
not disclose that **wave 3 is already authored in the working tree** —
`plugins/mochiko/migrations/0005-artifact-homes.yaml` (untracked; its own `intent:` reads
"Declare the artifact homes the census found, give five templates their conformance blocks …
and mint the authoring-time home rule on every producing primitive the log carries"). D11 puts
wave 2, this amend, **before** wave 3. Nothing is shipped and no ceremony is breached, so this is
disclosure, not violation. But the ratification is being taken with more of the mechanism built
than the recorded order contemplates, and AM-2 disclosed its build state more fully.

Two verified claims in the same block are accurate and should be recorded as verified:
`EXIT_CONFORMANCE = 4` is at `crates/mochiko-cli/src/hook.rs:37` exactly as the lead-check cites,
and `mochiko-cli check --hook-json -` is present in the installed binary (0.1.0, grammar 1..1).

**Resolution path.** State the build state at ratification, wave 3 included.

#### C9 — The one cost bound the design names is ruled out of governance, and nothing at the bump checks it *(emergent, outside the map)*

**Severity:** Minor. **Elements:** GI-002 (AM-3 note), GI-012 (release gates), agenda dimension 8.

The AM-3 note rules that "the aggregate per-run hook cost cap (≤ 60 s, record OQ4) is a
build-side watch, not governance." The driver of this entire brainstorm was cost — the user's
Q2: "I am strugging to control the verbosity … I dont want the files to be generated and then we
fix it . token wastage." The gate answers token cost by adding wall-clock cost to every consumer
run (wave 0 measured 49 ms per call, projecting 9.2–27.6 s per run on two non-implement
transcripts, with Bash at 85–86 % of gated volume). Under the current gates nothing at a
`plugin.json` bump blocks a ship that regresses past 60 s. Dimension 8 asks what blocks a
release; this names a bound and then places it where nothing does.

**Resolution path.** One question: does the ≤ 60 s cap join GI-012's gate 6, or stand as a watch
by ruling? Either answer is fine recorded; it is currently ruled by a clause inside a risk note.

#### C10 — Mochiko's own tree gets no violator pass, so its flagship governance artifact rides amnesty permanently *(emergent, outside the map)*

**Severity:** Minor. **Elements:** GI-019 (AM-3), GI-005 (record layer).

D11 wave 5 orders a violator pass over **kinako's** live tree, deliberately run "with the
consumer's plugin still at the pre-gate version" before the upgrade. D5 puts **mochiko's own**
`.mochiko/` in the census as a second sample, but no violator pass is scheduled for this
repository. The measurement in C2 shows the consequence: the governance synthesis itself is a
standing violator — the required heading `## Depth level declaration` is absent (GI-021's depth
declaration lives inside `## Minted principle intents`), the live `## Real commands (dimension
6/8 → the validator's placeholder bar)` does not match the declared `## Real commands`, and
`## Confrontation rulings (brownfield)` is undeclared. Amnesty allows every future write, so the
gate never brings the file into shape and the advisory repeats forever.

Note that the size half is not in play here: the memory home carries `bounds: elsewhere` with
`bounds_cite: .mochiko/memory/knowledge-management.md`, so D4f's exemption holds and only the
shape checks bind.

**Resolution path.** One question: does mochiko's own tree get a violator pass before its own
upgrade, or is standing amnesty the recorded ruling for this repository?

---

## 3 — Killed candidates

Twenty mapped angles died on read. Retrievable in full on ask.

| Angle | Why it died |
|---|---|
| A1 amend-scope slice | The fact profile is unchanged and no floor is un-waived, so no governance-event slice is owed; dimension 4 is worked at Card 6 and dimension 10 is untouched by ruling. |
| A4 CI path filter | `plugins/mochiko/hooks/**` is outside `ci.yml`'s filter, but CI carries only the crate layers and never ran the hook scripts; their real gate is GI-012 gate 6, the contract suite at the bump, which D10 extends with the new hook cases. |
| A5 release gate for the gate | Card 7 keeps GI-012 standing and D10 names the deterministic cases for both hooks; the gate exists. |
| A6 exclusion confronted | D7's rationale confronts `cli-schema-delivery` D7 directly and quotes its own from-zero reservation; the decline is superseded on its own terms, not around them. |
| A7 module once-offer | Every module on the surface carries a recorded ruling — GI-009 through GI-014 — so the once-per-amend offer is satisfied vacuously. |
| A8 waiver sweep | No waiver changes; GI-008 is untouched and nothing is narrowed by waiver. |
| A9 depth level | GI-021 `high` survives untouched, is ledger-recorded, and MINOR implies no flip; the one-way ratchet is unaffected. |
| A11 reminder justification | Paid for by elicited facts: F10's EPIC-001 read as "report shape only", and wave 0's leg 8 proving the injection lands before the first turn. |
| B12 supersession named in terms | Card 4 quotes the prior bullet verbatim and preserves it in the ledger GI-019 entry; the strips ceremony does not reach `.claude/rules/` because it is a governance surface, not a `plugins/mochiko/` primitive, and GI-006 reconstructibility is met by the preserved text. |
| B14 mechanical/judgment test | The Card 3 Testability re-key supplies one: Fail on "a `check` grading meaning or quality, or a hook that sequences", Pass on `check` "reading no meaning, naming no seat, order, or stage". |
| B15 advisory carve-out | The synthesis does not shelter under it — it takes the full admission-by-ruling route explicitly, which is the correct route for a blocking, pre-hoc, non-optional gate. |
| B16 non-negotiable supersession route | The ruling anchor is present (`hook-enforced-artifact-schema` D1/D7, accepted 2026-09-13), the superseded clause is named, and the surviving limb — judgment and sequencing — is stated. |
| B17 PowerShell vs declared-unsupported | No contradiction. GI-020's unsupported limb is about the shell the hook script needs (`shell: bash`, Git Bash absent); the matcher arm names the Windows shell **tool**, on the docs' own instruction that a `Bash` match alone leaves it uncovered. The synthesis flags the arm unverifiable on macOS. |
| B18 dependency widening | Resolved by the explicit-allow rule: with the binary absent or out of range the wrapper exits 0 and allows, so the gate never denies a consumer's writes on dependency grounds. The halt stays the existing hooks' job. |
| B19 GI-017 restatement | The reminder line points at `mochiko-cli home <path>` rather than restating any rule; no second home is created. |
| B20 already-built crate side | Verified accurate rather than hidden — `EXIT_CONFORMANCE = 4` at `hook.rs:37` and `check --hook-json -` in the installed binary, both as the lead-check states. Survives only as C8's narrower disclosure point. |
| C22 reminder removal test | It survives removal: D1b's channel answers a distinct elicited failure (the model reaching for a sibling artifact as its shape source) at zero re-emit cost, which the write-time gate does not address. |
| C24 contract-suite perturbation | D10 names deterministic cases for both hooks, including fail-open with no binary and a wrong-event payload, and gate 6 takes them at the wave-4 bump. |
| C25 landing cost | Card 7 places the hook scripts and the migration under the primitive-edits ceremony; both `primitive-edits.md` and `rust-cli.md` already carry `plugins/mochiko/hooks/**` in their `paths`. |
| C27 external-claims route | Platform claims are doc-pinned in F8, corrected once at the brainstorm's own review, and the load-bearing ones were measured by the wave-0 probe across four transports. Nothing was argued here. |
| C28 too-thin bar | The synthesis is the opposite of thin: every ruling carries rationale, provenance, and a mark, and the lead's own pre-deck checks are recorded. |

---

## 4 — Recommended status

**`critical-gaps`.**

One Critical survived. C1 is an unrecorded ruling on a NON-NEGOTIABLE's carve: whether the gate
may halt a run when no conforming draft exists has never been ruled, and clause (iv) records the
answer "it cannot" while the question that decides it sits open as OQ1 in the driver record and
appears nowhere in the synthesis. The measured budget for a brainstorm record — 150 lines against
43 of this repository's 66 records exceeding it and a largest of 1,536 — makes the halting case
the expected one rather than an edge.

The remaining nine are session-resolvable. Five Important findings are each closed by one
question or one restatement: mirror D4e's amnesty scope and qualify the gate's reach (C2), split
the mark to mirror the record as AM-2 did (C3), carry D9's reach boundary into the admission
(C4), put the two owed supply-chain controls' standing to the user against the widened vector
(C5), and route the semver class back to the user with the two MAJOR precedents on the table
(C6). Four Minor findings are one line or one question each.

Nothing here challenges the admission itself. The bright-line argument is the strongest of the
four clauses recorded to date: mechanical conformance genuinely is structural validity against
the store's own declared shape, the checks are decidable by string and count, no check reads
meaning, and the gate names no seat, order, or stage. The Critical is about a consequence the
argument has not yet priced, not about the argument's class.

The verdict is the lead's. This report is input; the user owns ratification.

---

## Verify pass

**2026-09-14 · same cold seat, from disk · NOT CLEAN — 5 residuals (2 Important · 3 Minor).**

Ten folds checked in `.mochiko/memory/governance-intent.md` as landed, against the driver
record, the ledger, and two live probes of the built binary. Eight folds are faithful. Two
introduced defects of their own: the C5 fold created a contradiction it did not carry through,
and the C2 fold over-corrected past both the record and the binary.

**Self-correction, disclosed.** This report's header said four survivors emerged outside the
blind map while its body tagged only three. C9 was the untagged fourth — the map's nearest
angles (A5, C24) were both killed, and the cost-cap question arose on reading the GI-002 note.
The C9 heading now carries the tag; header and body agree at four.

### Confirmed faithful

- **C1 — condition on clause (iv), road (a).** Lands complete: the condition, OQ1 carried to
  the wave-3 table, the review's measurement, the no-in-run-route fact, and road (b) named as
  declined. Now demonstrated rather than projected — a `Write` of a new 402-line
  `.mochiko/brainstorms/brand-new-session/record.md` returns
  `permissionDecision: deny`, exit 4, "`record.md` is 402 lines against a whole-file bound of
  150", and a `Write` of an undeclared name into a declared home denies at exit 4 with D2's full
  routing text. The two closed exits the fold names are closed in the binary, not just on paper.
- **C3 — mark split.** Mirrors the record exactly: `Confident` on the admission and on R1 / R2b /
  R3, `Assumed` on the D3-derived exit-code contract and the `check --hook-json -` interface. No
  mark sits above its record basis. D7–D11 stay `Confident`, matching their record marks.
- **C4 — reach boundary.** Faithful to D9 on all three limbs: the frontmatter sniff outside
  declared homes, `NotebookEdit` and consumer MCP writers knowingly uncovered, and the reviewer's
  residual duty.
- **C6 — the departure reason is honest to both precedents.** AM-1 redefined a principle
  (absolute ban to admissible-by-ruling) and AM-2 withdrew GI-020's degraded path; both are
  redefinitions, and AM-3 is not. The reason understates its own best support: AM-2 *also*
  widened GI-019's admission, and its MAJOR came from the GI-020 supersession, not from that
  widening. The Testability inversion is fairly read as re-keying a formulation narrower than the
  principle — the bright-line text never said hooks may block on dependency grounds only; the
  ledger row operationalized it when only dependency-halt hooks existed.
- **C7 — what the gate reads.** Faithful to D3.
- **C8 — build state.** Every fact verified: `89139a8` is the brainstorm landing, `c395dab` is
  the wave-1 crate, `wave3-budget-table.md` is on disk awaiting ratification, `plugin.json` is
  0.108.0, and the ledger's MINOR limb is still unedited on disk, correctly, since surfaces are
  authored only after ratification.
- **C9 — the cap stands as a watch by ruling.** The reason is recorded and sound: a wall-clock
  cap inside the Docker suite is nondeterministic, and gate 6 is the deterministic set.
- **C10 — violator pass.** Scope is tree-wide for this repository, with this synthesis's three
  drifts named as the instance. That scope also covers the brainstorm homes' own drift (an
  undeclared `review.md`, the `wave<n>-reports/` subdirs against a declared `reports/`), so no
  widening is owed.

### Residuals

**R1 — Important — the C5 fold left a contradiction behind it.**
*Where:* the `Untouched (Card 7)` clause of the GI-019 AM-3 paragraph.
*What:* it still reads "GI-012 (wave 4's `plugin.json` bump takes the contract suite with the new
hook cases, D10)" with no qualification, while the GI-002 AM-3 note and the Amendment Log's Scope
bullet both record GI-012 as narrowed by the C5 wave-4 precondition clause. A reader of GI-019
alone is told GI-012 is untouched; the other two surfaces say otherwise.
*Fix:* qualify that one entry — "GI-012 untouched but for the wave-4 precondition clause (review
C5)".

**R2 — Important — the C2 fold over-corrected, measured against the binary.**
*Where:* the `First-touch amnesty (D4e, all four measures)` clause and the qualified framing
sentence, both in the GI-019 AM-3 paragraph.
*What:* three defects in one clause.
(i) It lists **path** among the amnestied measures. A write outside every declared home is
ungated, not amnestied, so path is not a measure amnesty can relax. D4e's own four are checks
(a)–(d) — headings, frontmatter, placeholders, size — a different four.
(ii) It states the standing violation is "named in `additionalContext`". The binary does that for
shape and not for an undeclared file name: an `Edit` to the existing, undeclared
`.mochiko/brainstorms/hook-enforced-artifact-schema/review.md` returns a bare
`{"permissionDecision":"allow"}` with no `additionalContext` at all, while the same name written
fresh denies at exit 4 with the full file-set reason. A mis-homed file therefore rides amnesty
**silently**, and D9 makes the reviewer responsible for exactly the escapes that carry no signal.
(iii) The framing sentence qualifies only to "a file already outside its **shape**" while the
clause beneath it claims four measures.
*Fix:* name the measures amnesty actually relaxes (file set, shape, budget), align the framing
sentence to the same list, and record the silent allow on file set as the known gap it is rather
than asserting a report the binary does not emit.

**R3 — Minor — the lead addition is disclosed in the Semver bullet and absent from the Scope
bullet.**
*Where:* the Amendment Log's Scope bullet, which records a deck of 8.
*What:* the ledger's MINOR limb gaining "significantly expanded" is an amendment-policy edit
governing every future amend, carried by no card. Its merits check out and the circularity worry
does not hold: `validation-constitution.version-bump` already reads "MINOR (principle added or
significantly expanded; waiver added/removed)", so the ledger is the narrower of two surfaces
that already disagree, and the C6 reason cites a grammar it did not invent. The defect is only
that the durable scope record omits it, which GI-006 reconstructibility needs.
*Fix:* name it in the Scope bullet as a lead addition outside the deck.

**R4 — Minor — the C10 fold's sequencing is impossible as written.**
*Where:* the Violator-pass bullet.
*What:* "takes the wave-5 violator pass beside kinako's, **before the wave-4 bump lands here**"
asks a wave-5 activity to precede wave 4. For this repository the gate starts biting when the
maintainer installs the bumped plugin locally, not when the commit lands.
*Fix:* "takes the same violator pass, scheduled here before the wave-4 plugin is installed
locally rather than at wave 5."

**R5 — Minor — the Review bullet's tally drops the emergent disclosure.**
*Where:* the Review bullet.
*What:* it reads "28 raised, 10 survived". The tally is 28 angles worked, 20 mapped angles
killed, and four survivors emergent outside the blind map — C2, C5, C9, C10, two of them
Important. That four findings came from the cold read rather than the map is evidence about the
protocol worth keeping.
*Fix:* "28 angles worked, 10 survived — four emergent outside the map."

### Checks that returned nothing

No fold was applied to the wrong element. No mark was upgraded past the record. The C1 condition
does not soften the admission, and clause (iv) is not weakened by carrying its condition. The
Testability re-key, the explicit-allow rule, the scope-of-guarantee clause, and the `rust-cli.md`
supersession are unchanged from the reviewed text and remain faithful to D3, D7c, D7d, and D7e.

Residuals return through the lead's pen; this seat edited no governance surface.

---

## Delta-check

**2026-09-14 · same cold seat, from disk · bounded to R1–R5 · NOT CLEAN — 1 residual (Minor).**

All five residuals are repaired as their fix lines asked. One sibling summary line went stale
behind the R4 repair and now reproduces the defect R4 closed.

### The five repairs, confirmed

- **R1 — landed.** The Card 7 clause now reads "GI-012 untouched but for the wave-4 precondition
  clause ruled at review C5 (wave 4's `plugin.json` bump takes the contract suite with the new
  hook cases, D10)". GI-019, the GI-002 note, and the Scope bullet now agree on GI-012's status.
- **R2 — landed on all three limbs, and the repaired claim is measured.** Path is excluded in
  terms: "path is not a measure amnesty relaxes (a write outside every declared home is ungated,
  not amnestied)", which is consistent with the D9 reach clause in the same paragraph. The
  relaxable set is named as file set, shape (headings · frontmatter · placeholders), and budget —
  a correct mapping onto D2's set check and D4's (a)–(d). The framing sentence is aligned to the
  same list. The bare-allow gap is recorded as measured, attributed to the reviewer duty D9
  creates, and routed to the wave-4 build as a fix candidate rather than a governance change.
  The clause's positive claim — that the binary names a standing shape **or budget** violation in
  `additionalContext` — was only half-measured at the verify pass; the budget half is now
  measured too. An `Edit` to `.mochiko/brainstorms/hook-enforced-artifact-schema/record.md`
  returns `permissionDecision: allow` with `additionalContext` "Standing: `record.md` is 738
  lines against a whole-file bound of 150". Both limbs hold, and the file-set silence stays the
  single isolated gap.
- **R3 — landed.** The Scope bullet now carries the lead addition outside the deck, with its
  basis: the ledger's MINOR limb gains "principle significantly expanded" so it agrees with
  `validation-constitution.version-bump`, which already reads it.
- **R4 — landed.** "takes the same violator pass kinako takes at wave 5, scheduled here **before
  the wave-4 plugin is installed locally** (the gate starts biting for this repo at the
  maintainer's local install, not at the commit)". The sequencing is now possible as written.
- **R5 — landed.** "28 angles worked, 10 survived, four emergent outside the map (C2, C5, C9,
  C10)" — the four named match the report.

The Verify-pass line records round 1 NOT CLEAN, 8/10 folds faithful, all five residuals with
accurate one-line summaries, the repairs, and the seat's C9 self-correction. No mark moved: the
split stays `Confident` on the admission and the R1 / R2b / R3 forks, `Assumed` on the
D3-derived exit-code contract and interface.

Two further propagations are correct rather than defects: the Scope bullet's Card 6 entry and the
GI-002 note both dropped "no surface change", which the C5 clause makes true — that clause lands
on GI-002's conditional-discharge line and GI-012's release line.

### Residual

**R6 — Minor — the R4 repair left its sibling summary stale.**
*Where:* the Dispositions sentence in the AM-3 Review bullet.
*What:* it still reads "C10 user-ruled violator pass **at wave 5**". Before R4 that matched the
C10 bullet; now the bullet says the pass is scheduled here ahead of the local wave-4 install, and
"at wave 5" is the exact phrasing R4 established is impossible for this repository. A reader of
the Dispositions line alone gets the closed defect back. This is the same class as R1 — a repair
landing in one place and not its summary.
*Fix:* "C10 user-ruled violator pass, scheduled here ahead of the local wave-4 install (R4)".

Nothing else in the bounded scope moved. No new contradiction, no mark movement, no fold
misapplied. Residual returns through the lead's pen; this seat edited no governance surface.
