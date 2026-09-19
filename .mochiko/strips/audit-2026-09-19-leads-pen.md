# Independent audit — commit `004dc84` ("Close the copy loophole in the lead's-pen floor")

**Verdict: FAIL** — the reword itself is faithful and mechanically sound (criteria 1–4 and 6 all
PASS), but the landing is incomplete and the decision record carries a false version stamp
(criterion 5 FAILs). The fix is cheap and does not touch the migration or the rule text.

Audited by an independent reviewer (this session authored none of commit `004dc84`), per the
author ≠ grader requirement in `.claude/rules/mochiko/primitive-edits.md` and
`.claude/rules/mochiko/rust-cli.md`. Branch `primitive-evals-v2`, commit
`004dc846b4d4cf69864f9714f08d867aa0203e58`.

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
