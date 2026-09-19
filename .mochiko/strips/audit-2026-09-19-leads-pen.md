# Independent audit — commit `004dc84` ("Close the copy loophole in the lead's-pen floor")

**Original verdict: FAIL** — the reword itself is faithful and mechanically sound (criteria 1–4
and 6 all PASS), but the landing is incomplete and the decision record carries a false version
stamp (criterion 5 FAILs). The fix is cheap and does not touch the migration or the rule text.

**Re-grade after fix commit `f9be912` — see the addendum at the end of this file.
Verdict: PASS.** Criterion 5 is re-graded PASS. One residual sentence (the sibling backlog entry
still names a version that never landed) is left as a low-severity fix item, not a blocker.

Audited by an independent reviewer (this session authored none of commit `004dc84` or `f9be912`),
per the author ≠ grader requirement in `.claude/rules/mochiko/primitive-edits.md` and
`.claude/rules/mochiko/rust-cli.md`. Branch `primitive-evals-v2`, commits
`004dc846b4d4cf69864f9714f08d867aa0203e58` and `f9be912f5168ae5685c037a3101d15b44fa0a03a`.

## Criterion 1 — Is the reword faithful to the ruling and the evidence?

**PASS.**

The kit's fill log (`evals/review-governance-intent/preregistration.md`, "Fill log" section) is
specific about what failed: on the pressure golden `g1-amend-pressure`, the cut skill wrote a
separate file `governance-intent.patched.md` on 2 of 3 replicates (`file_absent` tripped) while
leaving the original synthesis byte-identical (`fixture_unchanged` held) — i.e. it produced a
corrected copy beside the original rather than editing the original in place. The pre-cut body
refused this on 9/9 deterministic runs.

The old rule text — "never your own edits to the synthesis" — forbids only in-place edits, not a
sibling copy. The new text adds exactly that: "never a corrected, patched, or rewritten copy of
it beside the original. Producing the fix yourself is the violation, whatever the file is
called." The referent ("it"/"the fix") ties to "the synthesis" from the same sentence, so the
added clause is scoped to copies of the reviewed artifact — it does not reach into banning
legitimate output (the `review.md` survivor report, disposition tables in the Review section,
etc.). This closes the measured behavior without over-reaching past it.

## Criterion 2 — Does the migration meet the log's grammar and authoring path?

**PASS.**

- Header fields present and well-formed: `grammar: 1`, `id: 0006-leads-pen-no-patched-copy`
  (matches the filename stem), `sequence: 6` (matches the `0006` filename prefix), `intent` as a
  folded one-statement scalar (same style as `0002`–`0005`), `anchor:
  2026-09-19 primitive-evals-v2 four-slot read`, `hash:` a 64-hex-char SHA-256.
- Ran `target/release/mochiko-cli migrate validate --plugin-root plugins/mochiko --report`
  myself: **0 rejecting findings** (104 advisory findings, all pre-existing and unrelated to this
  rule or migration — none name `0006` or `findings-through-leads-pen`). A hash or grammar defect
  would show as a rejecting finding, so this confirms the hash is correct and the file well-formed.
- `mochiko-cli migrate status` reports `sequences 1..6 (6 migrations)` — the log replays cleanly
  with the new file in place.
- The op chosen, `reword-rule` (fields `schema`, `id`, `text`), is the grammar's correct minimal
  op: the rule's id survives, only `text` changes. The rule already carries `class: floor` and an
  existing `anchor: 2026-08-11 validator-scope-and-verbosity` (protected content per the log
  README's three-part definition), but the anchor rule only *requires* a header anchor for
  `supersede-rule`/`tombstone-rule`/a protection-lowering `set-rule-field` — never for a plain
  `reword-rule` that leaves `class`, `kind`, and `anchor` untouched. This migration carries a
  header `anchor:` anyway (good practice, citing the ruling), but it is not grammatically required
  and its absence would not have been a defect.

Minor, non-blocking observation: the anchor's session reference — "primitive-evals-v2 four-slot
read" — is a real, traceable event (it matches the preregistration fill log and ROADMAP's own
"four post-cut slots READ 2026-09-19" phrase) but is not a hyphenated `<session-slug>` matching a
`.mochiko/brainstorms/` directory, unlike every other anchor in the log. Anchor-to-`DECISIONS.md`
resolution is advisory-only per the migrations README, so this does not fail validation, but see
the fix list below — there is currently no `DECISIONS.md` row for it to resolve against at all.

## Criterion 3 — Is the derived view a faithful, sole regeneration?

**PASS.**

Re-emitted the full view tree with the release binary to a scratch directory
(`mochiko-cli views emit --plugin-root plugins/mochiko --out <scratch>`) and diffed it against
the committed `.mochiko/schema-views/`: **zero differences** (`diff -rq` exit 0). `cargo test`'s
own `every_emitted_view_matches_the_committed_one` (in `tests/views.rs`) passes for the same
reason. The commit's file list confirms exactly one document changed under
`.mochiko/schema-views/`: `skills/review-governance-intent.yaml`.

## Criterion 4 — Is the rule actually delivered in its new form?

**PASS.**

Rendered it directly:

```
mochiko-cli rules review-governance-intent --plugin-root plugins/mochiko --section review-governance-intent.sec.reserved
```

Output carries the version-triple header (`binary 0.1.0 · grammar 1 · plugin 0.109.0`), the full
reworded text under `review-governance-intent.findings-through-leads-pen`
(`[class: floor · kind: reservation · labels: independence]`), and the end line
(`mochiko-cli rules end · review-governance-intent · review-governance-intent.sec.reserved · 3
rules`). The preamble's `pins` line still reads `class: floor · 16 rules` and lists
`findings-through-leads-pen` in the `floors:` index, matching the kit's own count. Delivery is
confirmed, not assumed.

## Criterion 5 — Is the record honest?

**FAIL.** Two of the record's claims check out; two process failures do not.

**Verified true:**
- *"Nothing had been deleted... its protection had survived the schema conversion intact."*
  Confirmed against `.mochiko/strips/review-governance-intent.md`, entry
  `## [v0.100.0] Lead's-pen + evidence floor — protection transfers, lettered split`: this rule's
  protected status transferred at the v0.100.0 schema conversion as a "kept-distinct edge, RB+RGI
  only" (i.e. deliberately kept as its own local floor rather than merged with
  `review-brainstorm`'s copy). The rule has carried `class: floor` with its `anchor:` field
  unbroken since. Nothing was deleted; this reword only ever touched `text`.
- *"This edit takes no strip entry."* Confirmed against `.mochiko/strips/README.md`'s own
  wording: "Schema content is recorded by the migration log, not here" — schema-content edits
  take a migration file, never a strip entry. This is schema content (a `skill/` schema rule), so
  the claim holds.

**Not true / incomplete:**
- The record's header states **"built the same day at plugin v0.110.0."** `plugins/mochiko/.claude-plugin/plugin.json` is `0.109.0` at this commit (confirmed: the file is untouched by
  `004dc84`, and the last commit to touch it, `5d8fc69`, is titled "bump staged, not landed"). The
  commit message itself says the version bump is "written but held out of this commit." The
  record's own version stamp is therefore false at the point this commit lands — it names a
  version that does not exist yet. `mochiko-cli`'s own render confirms the live plugin version is
  `0.109.0`. Given `CLAUDE.md`'s GI-006 ("every primitive edit MUST be reconstructible from strips
  + the migration log + `DECISIONS.md` + version stamps" — NON-NEGOTIABLE), a wrong version stamp
  is exactly the kind of defect this audit exists to catch.
- **No `DECISIONS.md` row was appended.** `primitive-edits.md`'s own ceremony requires "citing the
  ruling: a `DECISIONS.md` row + a `.mochiko/decisions/` ADR when no session record exists" — the
  ADR was written (`.mochiko/decisions/2026-09-19-leads-pen-copy-loophole.md`), but the row is
  missing; `grep -n "leads-pen" DECISIONS.md` returns nothing.
- **`ROADMAP.md` was not touched.** The landing ritual in `CLAUDE.md` ("Landing work") requires
  touching `ROADMAP.md` Now/Next with statuses agreeing across the brainstorms index, the record,
  and the decisions index — "a landing that only adds is incomplete." `ROADMAP.md:28` still reads
  "`review-governance-intent` floor **re-add ruled** and owed through strips" — both stale (the
  re-add landed in this commit, and it landed through a migration, never through strips) and
  unchanged by `004dc84`.

None of this touches the substance of the reword. But it is a real gap in the ceremony this repo
treats as non-negotiable, and it is cheap to close (see fix list).

## Criterion 6 — Crate test expectation: correct, minimal, and independently reviewed

**PASS.** Stating explicitly, as the crate rule requires: **this is the independent non-author
code review** — I authored none of this commit.

The diff in `crates/mochiko-cli/tests/fidelity.rs` only extends the pinned sequence list
(`vec![1, 2, 3, 4, 5]` → `vec![1, 2, 3, 4, 5, 6]`) and its accompanying doc-comment to name the
new migration. I checked for any other hard-coded migration-count or sequence-list expectation
that might have been missed: `crates/mochiko-cli/tests/replay.rs:182` also asserts a `sequences()`
result, but against a synthetic in-memory log built inside that test, unrelated to the shipped
corpus — nothing there needed touching. Ran `cargo test --all`: **all suites green** — including
`fidelity.rs` (17/17) and `views.rs` (11/11, covering the view≡replay assertion from criterion 3).
This is the correct and minimal change.

## Fix list (ordered by severity)

1. **(High)** Append the missing `DECISIONS.md` row for this ruling, pointing at
   `.mochiko/decisions/2026-09-19-leads-pen-copy-loophole.md`. Without it the anchor
   `2026-09-19 primitive-evals-v2 four-slot read` has nothing to resolve against, and GI-006's
   reconstructability chain is broken.
2. **(High)** Correct the decision record's version stamp. It should not claim "built... at plugin
   v0.110.0" while `plugin.json` is still `0.109.0` and the bump is explicitly deferred. State the
   actual landed version (`0.109.0`, schema-only; version bump pending) instead.
3. **(Medium)** Update `ROADMAP.md:28`'s post-cut-slots line: it still says the
   `review-governance-intent` re-add is "owed through strips," which is both done now and was
   never a strips-path item (it's a migration). Land the status change alongside the
   `DECISIONS.md` row so the brainstorms index, the record, and `ROADMAP.md` agree, per the
   landing ritual in `CLAUDE.md`.
4. **(Low)** `.mochiko/memory/primitive-cost-budgets.md`'s `review-governance-intent` row (16,274,
   "no headroom") was last re-seeded at v0.106.0 and, unlike several sibling rows, was never
   annotated with the v0.107.0 render-format `-237` constant. A rough re-measure of the current
   rendered payload against the CLI's live output puts this edit only about 8 characters under
   that ceiling — an uncomfortably thin, and possibly miscalculated, margin resting on stale
   bookkeeping. Re-run the canonical measurement and re-seed the row before the next edit to this
   skill.
5. **(Low, advisory)** The migration's anchor phrase is not a hyphenated `<session-slug>` the way
   every other anchor in the log is. Harmless (anchor-format checking already passed
   `migrate validate`, and resolution against `DECISIONS.md` is advisory), but worth tightening
   for consistency once fix 1 lands.

## Ruling on the sibling floor (`review-brainstorm.findings-through-leads-pen`)

**Leaving it unamended is disclosed and tracked, but not the better call — the pair should have
moved together.**

The strips record shows the two floors were deliberately kept distinct (not merged into a shared
`extends:` block) at the v0.100.0 schema conversion specifically so each could carry its own
wording — so nothing structural blocks editing one without the other, and nothing here risks an
unassessed shared consumer.

The stated reason for not moving it — "no eval has put `review-brainstorm` under pressure
goldens" — is the right caution for a *behavioral tuning* decision, where fitting a rewrite too
tightly to one kit's specific golden risks overfitting. But that is not what this fix is. The gap
being closed is a **logical property of the English wording itself**: "never edits to X" does not
forbid "producing a corrected copy of X" as a matter of plain reading, independent of which skill
X belongs to. `review-brainstorm.findings-through-leads-pen` — "Findings enter through the lead's
pen — never your own edits to the record" — has the textually identical structure and the
textually identical gap, under the same `independence` label, enforcing the same "findings go
through the lead's pen" principle in the same "Reserved" section shape. A reviewer under that
floor has, right now, an undisclosed, live, structurally-identical route to comply with the letter
while writing a corrected copy of the brainstorm record beside the original — the exact failure
this commit just spent a migration closing on its sibling.

The backlog entry is honest about the open state and names both closing paths, which is the right
way to carry a known gap if it must be carried. But given the fix is definitional rather than
data-driven, I would not have waited for a second kit to prove out a hole that a plain reading of
the sentence already proves. Recommend taking the backlog entry's second option now — the
one-line migration on the textual argument alone — rather than leaving a matching floor sitting
open on a skill that reviews artifacts under exactly the same pressure shape.

---

## Addendum — re-grade after fix commit `f9be912`

**Criterion 5 re-graded: PASS. Overall verdict flips from FAIL to PASS.**

Checked each of the five fixes against the artifacts directly, and re-ran the mechanical checks.

1. **`DECISIONS.md` row — appended, and verified honest.** The 2026-09-19 row now covers all
   four post-cut slot readings plus this re-add. I read it against each kit's own fill log,
   specifically checking whether any claim overstates what the run supports (the lead's ask):
   - `review-specifications`: row says "post 8/8 floors vs pre 4/8, scripted 9/12 vs 1/12; five
     must losses stay recorded as unread... 22.8% against a 20% cap after the once-only extra
     replicate, which raised both shares rather than lowering them; no strips re-add." Matches
     the kit's addendum and 2026-09-19 ruling exactly, including the deliberate "unread" framing
     — the fill log itself is careful to distinguish "unread" from "noise" from "regression,"
     and the row preserves that distinction rather than flattening it.
   - `validation-constitution`: row says "post 19/25 live + 13/14 floors vs 16/25 + 11/14,
     nothing lost in either direction, scripted 8/9 · 7/9 · control 0/9, 27/27 valid; bar (c)
     breached at 16.4%, gain kept but unclaimed; US$16.70." Matches the "re-run under the fixed
     runner" addendum verbatim (the row correctly uses the corrected re-run numbers, not the
     earlier voided fill's 16/25 · floors 11/14 vs 12/14 figures from the `--add-dir` defect).
   - `review-plan-artifacts` (the one flagged for extra scrutiny): row says "accepted on its
     deterministic layer, judged layer closed UNREADABLE (both arms ≈ twice the cap at 36.8%
     and 46.0% while the control arm is the steady one at 11.5%, which points at the skill arms'
     long varied artifacts, not judge-wide instability; scripted 8/9 · 8/9 · 0/9 with 80/81
     assertions per skill arm against 33/81 control; **the runner's KILLED verdict on 5 rules /
     3 floors is void as a finding** — no strips re-add in either direction...)." I checked this
     one hardest. The fill log's own diagnosis reads: "A control that steady rules out
     judge-wide instability and points at the skill arms' own output — long, structurally
     varied review artifacts the binary coverage judge scores differently run to run," and its
     kit status reads "the five lost rules and three lost floors are not findings about the
     skill... No strips-path re-add decision arises from this run, in either direction." The
     row's claim is a tight, accurate paraphrase — it does not upgrade "void as a finding" into
     anything stronger, and it does not quietly drop the "in either direction" qualifier (which
     matters: it forecloses reading the KILLED loss as a regression AND forecloses reading the
     surviving rules as a clean pass). No overstatement found in any of the three.
   - The explanation for the row's original absence (another session held an in-flight,
     uncommitted entry in `DECISIONS.md`; folding it in would have taken someone else's work) is
     a legitimate reason to have deferred, not a post-hoc excuse — and it is disclosed in the
     commit message rather than left silent.

2. **Decision record's version stamp — fixed.** The header now reads "ruled (user) and built
   2026-09-19; **the plugin bump is staged, not landed** — `plugin.json` stays at 0.109.0 only
   once release gate 6... is green," which matches the actual state (`plugin.json` is still
   `0.109.0`, confirmed again on this branch). No more false claim on the record's own status
   line.
   - **Residual, not fixed:** the sibling `BACKLOG.md` entry added in the *original* commit
     (`review-brainstorm.findings-through-leads-pen carries the copy loophole...`) still reads
     "the governance-intent floor was reworded **at v0.110.0**" — the identical false version
     stamp, untouched by `f9be912`. This is the same defect criterion 5 originally failed on,
     surviving in a second location. Low severity (one sentence, in a backlog item, not a
     status line load-bearing for GI-006 reconstructability the way the decision record's own
     header is) — flagged below as a fix item rather than grounds to hold the verdict at FAIL.
   - Also worth a note: the commit message says "both the backlog entry and the decisions row
     record the disagreement" on the sibling scope question. The `DECISIONS.md` row does
     ("the auditor dissents and would move the pair together on the textual argument"); the
     `BACKLOG.md` entry's text is unchanged from the original commit and does not itself name
     the dissent. Not a defect in the fix — the scope question is genuinely recorded, just only
     in one of the two places named — but the commit message overstates by one word ("both").
3. **`ROADMAP.md` — fixed correctly.** Now reads "`review-governance-intent` floor **re-add
   ruled AND BUILT 2026-09-19** — not through strips: nothing had been deleted... carried as
   migration `0006-leads-pen-no-patched-copy` with its ADR; the bump is staged behind release
   gate 6." Status and mechanism both correct, and consistent with the `DECISIONS.md` row and
   the decision record.
4. **Migration anchor + re-stamp — fixed and mechanically verified.** The anchor is now
   `2026-09-19 primitive-eval-harness-v2 four-slot read`, and `.mochiko/brainstorms/` does carry
   a `primitive-eval-harness-v2/` directory — this is now a real session-slug, not an informal
   phrase. Re-ran `mochiko-cli migrate validate --plugin-root plugins/mochiko --report`: **0
   rejecting, 104 advisory** (same pre-existing advisory set as before, nothing new). Re-ran
   `cargo test --all`: **fully green**, including `fidelity.rs` (17/17). The hash changed
   (`sha256:2836f682...`, still 64 hex chars) and the replayed state hash is unchanged from
   before the re-stamp, as expected — only the migration's own header changed, not any document
   content.
5. **Budget finding — recorded, not fixed; asked whether that's the wrong call.** Recording is
   the right call here, not blocking. The edit is not currently over budget — my own remeasure
   put it about 8 characters *under* the recorded ceiling, not over it — so there is no live
   violation to block on; what's wrong is that the ceiling itself rests on bookkeeping that
   predates a known format change, which is a measurement-hygiene problem, not a budget breach.
   The new `BACKLOG.md` entry states the risk plainly and names a concrete trigger ("re-seed the
   row... before any further edit to this skill's rules") rather than leaving it vague. Blocking
   this landing on a re-seed would be disproportionate to a margin that, by the audit's own
   measurement, still clears.

### Updated fix list

1. **(Low)** `BACKLOG.md`'s `review-brainstorm.findings-through-leads-pen` entry still says the
   sibling was "reworded at v0.110.0." Correct it to the actual state (0.109.0, bump staged) —
   the same fix already applied to the decision record, just missed in this second location.
2. **(Low, advisory, carried from the original audit)** Re-seed
   `primitive-cost-budgets.md`'s `review-governance-intent` row against the current render
   before the next edit to this skill — now tracked in `BACKLOG.md`, no further action needed
   from this audit.

Neither item is a landing blocker. The verdict is **PASS**.
