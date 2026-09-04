# V1 — P1 (crate + log) audit

**Verdict: PASS** (was FAIL; P1 reworked, and I verified the rework — see the closing section).
All eleven items PASS. Everything the op, the stamp, the migration, the snapshots, the fixture and
the legend were asked to do, they do, and I reproduced all of it from my own runs rather than from
the report. The two required fixes were a missing README sentence (item 2) and a remediation hint
the fixture ruling had made wrong (item 7); both are now in the tree and both check out.

The item verdicts below are as first written, against diff sha256 `f1a0f083…2700`. The dated delta
verdict at the end carries the re-grade against the current tree, sha256 `e08ba43a…cb5f`.

Graded against `git diff -- crates/ plugins/mochiko/migrations/ plugins/mochiko/schemas/` (sha256
`f1a0f083…2700`), the 51 fixture files and `0002-fail-conditions-intent.yaml`. The tree moved once
mid-audit — `src/render.rs` and the report gained the legend section after I had read both — and
everything below was re-run after it settled. The lead later declared the tree settled and asked
for a delta re-grade of item 8; the diff hash was unchanged at that point, so the legend rework had
already landed before my first measurement. Item 8 and the four layers were re-run anyway and are
folded in below.

**1. The op — PASS.** Five scratch migrations of my own, through the real binary. Names none of
the three: `changes[0]: names no field to reword — want at least one of …`, stamp exit 1,
`migrate validate --log-dir` `op-malformed` exit 1. `title: ~`: `every section carries a `title:`,
so it is reworded and never cleared`. Unknown id: `no such live section`. Tombstoned id
(`brainstorm.sec.harness`): `the section is retired — a tombstoned node is never reworded`. Both
apply-side rejects come back as `op-inapplicable`, exit 1. For `note: ~` I set all three fields on
a live rule-bearing section, emitted views, then cleared the note alone: the `note:` line is gone,
title and intent survive. Rules and id untouched — a `views emit` of the genesis-only state
against my probe's final state differs in exactly the title and folded intent lines. The README's
op row sits between `mint-section` and `tombstone-section`, every field claim in it matching what
I observed.

**2. Grammar — FAIL as first graded, PASS after rework.** `GRAMMAR_RANGE` is `(1, 1)` and appears
nowhere in the diff; that limb always passed. The README said nothing about why an additive op
stays in grammar 1 before the first publish — its only grammar mention was the field table's
generic line about a binary declaring its range. The reasoning lived in plan §7 and the report,
but the log's README is where the author of op seventeen will look. **Fix 1**, now closed.

**3. `migrate stamp` — PASS.** Stamps an unstamped body, exit 0, one `hash:` line. Re-stamping is
a fixed point (file sha256 `5141e078…d43` before and after). Exit 1 on an unparseable body, every
file in the directory byte-identical afterwards; exit 2 on a missing path; a successful stamp
likewise touched no sibling. Stamping a scratchpad copy of the committed genesis returned it
byte-identical, reproducing P1's claim.

**4. Migration `0002` — PASS.** Header carries `grammar` · `id` · `sequence: 2` · `intent` ·
`anchor` · `hash` in the README's shapes, the anchor `2026-09-03 cli-schema-delivery D3` matching
the documented form. Six `reword-section` changes, one per command schema, nothing else in the
file. The hash is genuinely enforced — one altered word on a scratch copy gives `hash-mismatch ·
… the header records sha256:47abe5a3… but the body canonicalises to sha256:5755cf79…`, exit 1;
against the real log, `migrate validate --plugin-root plugins/mochiko` is `0 rejecting · 105
advisory`. The six live-rendered intents are byte-identical to plan §2's two strings, `visit` for
the desks and `run` for the runs, checked by `diff` rather than by eye.

**5. State — PASS.** `sequences 1..2 (2 migrations)`, `state sha256:8972891099f77b1080b243f73adc7ea5ae0c3c9479cb17414cbe1fd97cfd43fd · 50 documents · 1016 rules` — P1's hash exactly. Two runs
byte-identical; an isolated scratch copy of the log replays to the same hash.

**6. Snapshots — PASS.** `git diff --stat -- plugins/mochiko/schemas/` is exactly six files at
`2 +-` each, one changed line apiece, the line being the `fail-conditions` intent every time. Both
semantic checks are green, including the one that bites here —
`every_shipped_document_survives_the_log_field_by_field` reads the live corpus off disk and
compares section intents field by field (`fidelity.rs:239`). All six rendered blocks carry the new
line; none contains "hard-codes". Noted, not charged against P1: the README says derived views are
"regenerated, never hand-edited" and these six were hand-edited, which plan §0 authorizes.
Reconcile the two before wave 6.

**7. The frozen fixture — PASS, with the defect behind fix 2.** All 51 files are byte-identical to
`git show HEAD:<original path>`, established in two steps rather than 51: `diff -r` of the
fixture's three subtrees against the live tree reports exactly the six edited schemas as
differing, no file only on one side; each of those six I compared against its HEAD blob with
`cmp`, all identical. Content totals 603,801 bytes as reported. The byte test points at
`frozen_corpus()`, carries `Built from the frozen v0.103.0 corpus (record D8; frozen 2026-09-04)`,
and passes; the field-by-field test still scans the live repo root through
`genesis::scan(&repo_root())` against the full-log replay. The defect: that byte test's failure
message still says `regenerate with cargo run -- genesis emit --out
plugins/mochiko/migrations/0001-genesis.yaml`, and `run_genesis_emit` builds from the live tree. A
maintainer who follows it now rewrites the committed genesis from the drifted corpus — exactly
what the fixture prevents, invalidating every downstream hash. **Fix 2.**

**8. Legend — PASS.** Nine `- ` lines, the six original then the three new, after `pins` (line 21)
and before `sections` (line 36), `preamble · 0 rules` end line intact, all confirmed against live
`rules … --section preamble` renders rather than the fixture. The three new lines are verbatim
against the addendum text the lead sent: I wrote that text to a file and `diff`'d it against the
rendered block's last three lines, byte-identical, in the given order. The block measures 844 bytes
from the `legend` line; with the const's leading newline that is **845 bytes**, matching P1's
figure and the new pin. The golden `LEGEND` in `tests/render.rs` was updated to match, and
`the_legend_block_is_the_size_the_wave_recorded` pins both 845 and the nine lines.

On the narrowed assertion, `a_skill_preamble_omits_moments_and_the_fail_pin`: the narrowing from
`!contains("moments")` to `!contains("\nmoments\n")` **still proves a skill preamble carries no
moments block**, and the old form no longer could. The render emits the block as a bare `moments`
line preceded by a blank line and followed by its items, so any emitted block matches
`\nmoments\n`; the preamble always opens with its header line, so no block can appear
newline-free at the start. The legend's own line reads `- moments: …`, which does not match, so
the narrowing removes the false positive without opening a hole. Confirmed live: a
`review-brainstorm` preamble renders the nine-line legend, the moments line included, and no
moments block. `cargo test --test render` is 30 passed, the four legend and skill-preamble tests
among them.

**9. Four layers — PASS.** `cargo test --all` 331 passed, 0 failed; `cargo fmt --all --check` exit
0, no output; `cargo clippy --all-targets -- -D warnings` exit 0; `cargo audit --deny warnings`
exit 0. `MOCHIKO_FULL_SIMILAR=1 cargo test --test matrix_similar` is 48 passed in 96.9s. All four
re-run on the settled tree at the lead's request, with the same results; 331 matches P1's claim.

**10. Scope — PASS.** `git diff -- crates/mochiko-cli/src` touches four files and nothing beyond
the op (`migration.rs`, `replay.rs`), the stamp (`cli.rs`), the legend (`render.rs`) and doc
comments. Nothing under `plugins/mochiko/` but the log, its README and the six snapshots.

**11. Report honesty — PASS with one correction.** Every substantive claim reproduced: both state
hashes, the migration hash, the advisory count, the six one-line diffs, the fixture's byte total
and HEAD identity, the census pins (321 / 695 / 226 / 110 / 36, at `fidelity.rs:524-529`), the
legend at 845 bytes, the sweep's 48 tests, the genesis-stamp fixed point. The per-file test table
is exact in both columns, checked by counting `#[test]` in the HEAD files and running each binary.
All seven disclosed deviations match the tree. The correction: the four-layers paragraph still
reads "330 passing" while the report's own tally and my run say 331. **Fix 3.** P1's no-strip
claim holds — the migrations README calls schema-content strips "redundant rather than
reformatted", `primitive-edits.md`'s v0.104.0 sentence routes this content to a new migration
file, and nothing protected left.

## Fix list (all closed — see the rework re-grade)

1. `plugins/mochiko/migrations/README.md` — one sentence on why an additive op extends grammar 1
   rather than minting grammar 2 before the first publish, and that the first publish freezes
   grammar 1 with the op in it.
2. `crates/mochiko-cli/tests/fidelity.rs` — the panic message in
   `the_committed_genesis_regenerates_byte_identically` must stop telling a maintainer to
   regenerate genesis from the live tree. Point at the fixture, or say a real corpus change is a
   new migration and never a genesis rebuild.
3. `wave4-reports/p1-op.md` — "330 passing" to 331.

## Delta verdict — 2026-09-04 — **PASS** (diff sha256 `e08ba43a…cb5f`)

All eleven items PASS. Every fix on the list is closed, and the wording nit I raised under the
first re-grade is closed too. I graded the files, not P1's claims, and re-ran the four layers on
the tree as it now stands. The tree moved twice more between re-grades — first the fix-list rework
(`d46a698f…9aa8`), then a one-line correction to the snapshot paragraph (`e08ba43a…cb5f`) — and
every check below is against the latter.

**Item 8's verbatim limb — PASS.** I wrote the three lines from the lead's message to a file and
`diff`'d them against the last three lines of the live rendered legend block. Byte-identical, in
the given order, after the six originals and before `sections`. Re-confirmed after the rebuild on
the current tree. The rest of item 8 stands as first graded: nine `- ` lines, `preamble · 0 rules`
end line intact, 845 bytes, golden and size tests updated, and the narrowed moments assertion
still proving a skill preamble carries no moments block.

**Fix 1 — closed.** The README's change-ops section gains a paragraph saying `reword-section` was
added at wave 4 with the log still at grammar 1 because no binary is published, that the D5 range
`1..1` freezes at the first publish with whatever ops the grammar carries by then, that a later
binary meeting an op it lacks rejects the file loudly and names the install command rather than
skipping it, and that a grammar bump is reserved for a change that would make an existing file
mean something different. That is the reasoning the checklist asked for, in the place it asked for.

**Fix 2 — closed, and I ran the new instruction rather than reading it.** The panic message now
names the fixture root: `cargo run -- genesis emit --root
crates/mochiko-cli/tests/fixtures/genesis-corpus --out plugins/mochiko/migrations/0001-genesis.yaml`,
followed by why a live-tree build would fold later migrations' content back into sequence 1. The
`--root` flag is real, and executing that command verbatim into the scratchpad produced a file
`cmp`-identical to the committed genesis. The hint is now correct and executable.

**Fix 3 — closed, graded on the file.** Both live mentions in P1's report read 331, at the tally
sentence and at the four-layers paragraph. The only surviving "330" is inside P1's own rework note
quoting my finding, which is correct. P1 says the copy I graded was already corrected when the
legend addendum moved the count; the report file did change on disk during my audit, which fits,
and I did not have to take the claim on trust either way.

**The README/practice tension I noted under item 6 — closed, beyond what I asked for.** A new
paragraph records that the shipped snapshots are transition-clause copies kept semantically equal
by the view ≡ replay test, mirrored by hand rather than regenerated, and retired at wave 6. My
wording nit from the first re-grade is also fixed: the sentence now says regenerating would drop
"the in-body comments, spacing, and fold width those files carry", with header comments named
separately as protected content under the strip ceremony. That matches what I measured — I emitted
a view and compared, and the top-of-file header survives while the `plugin_root` rationale block,
blank-line spacing and fold width do not.

**Layers re-run on the current tree:** `cargo test --all` 331 passed, 0 failed; `cargo fmt --all
--check`, `cargo clippy --all-targets -- -D warnings` and `cargo audit --deny warnings` all exit 0.
`migrate validate --plugin-root plugins/mochiko` is still `0 rejecting · 105 advisory`, the state
hash is still `sha256:8972891099f77b…43fd · 50 documents · 1016 rules`, the six snapshot diffs are
still one line each, and the panic hint's command still reproduces the committed genesis
`cmp`-identical. Both rework rounds touched only prose surfaces and one panic string; no test
changed meaning and the log is untouched.
