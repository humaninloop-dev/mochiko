# V1 — audit of P1 (crate + log + views), wave 6

**Verdict: PASS**, with two report-prose corrections owed (§11) and one follow-up now landable (§12).

Graded the diff, the files and my own runs, never the report alone. Base `62aa99d`. The tree I graded already
carries P2's deletions (`plugins/mochiko/schemas/` and every `skills/*/schema.yaml` gone from disk) and P3's
`evals/contract/` edits, so the results below are stronger evidence than P1's own run: the suite is green *with*
the files already deleted.

## 1. `mochiko-cli doc` — PASS
All three render at exit 0 with the exact head line `mochiko-cli doc <name> · binary 0.1.0 · grammar 1 · plugin
0.106.0` and end line `mochiko-cli doc end · <name>`. Each body parses as YAML and is value-equal to its
committed view; sizes 3,238 / 1,292 / 1,713 bytes as reported; no derived-view file header in the body. Error
paths all exit 2, verbatim as reported: an unknown name lists the three available, `spec` redirects to `template
spec`, `specify` and `review-feasibility` to `rules … --section <section>`, `common` explains that stubs are
already resolved.

## 2. Skill legend — PASS
Parsed the constants out of `src/render.rs`: `SKILL_LEGEND` is **605 bytes, 6 bullet lines**, carrying none of
`fail`, `enforces:`, `moments:`; `COMMAND_LEGEND` is unchanged at **845 bytes, 9 bullet lines**. Confirmed live
— `rules review-brainstorm --section preamble` prints the six-line variant closing on `· routing · latitude.`,
`rules implement --section preamble` the nine-line one. Goldens updated, and
`a_skill_legend_omits_the_grammar_a_skill_schema_cannot_carry` keys on the dropped line texts, so a re-widening
cannot pass the byte pin by coincidence.

## 3. Views — PASS
`.mochiko/schema-views/` holds **50 files**: `commands/` 6 · `skills/` 30 · `templates/` 8 · `common/` 3 ·
`labels/` 2 · `shelves/` 1. A fresh `views emit --out` into a scratch directory is **byte-identical** to the
committed tree (`diff -r` exit 0). `tests/views.rs` compares each emitted view against its committed file by
canonical hash, the layout test asserts the six kind prefixes and that no view path names `plugins/` or
`schemas/`, and the new orphan test pins the tree at the 50 files the emitter writes. `src/views.rs` carries no
`plugins/mochiko/schemas` mapping. The CI `push` and `pull_request` filters carry `.mochiko/schema-views/**` and
no longer name `plugins/mochiko/schemas/**`; `.gitignore` is untouched and the views are not ignored. I did not
red-prove drift by mutating a committed view — that means editing a file three seats are writing — but the
pre-wave log emits 12 differing view files, so the comparand is live.

## 4. Tests re-targeted — PASS
`tests/render.rs` `SHIPPED_SCHEMAS_DIR` is gone; `shipped_state()` and the template fixtures replay
`plugins/mochiko/migrations`. `tests/fidelity.rs` compares the frozen fixture against a genesis-only replay into
a temp log and reads the fixture's own sidecar; `tests/anchor_grammar.rs` likewise. `tests/matrix_similar.rs`
builds its corpus from the replay. `tests/validate.rs` is the one place the checklist wording and the artifact
differ: its corpus is the **committed views**, not a `replay::load`. P1 discloses this in report §7 with a sound
reason — the round-trip test grades a decoder, so its input must be text, not a value the model produced — and
the views are byte-equal to the replay and drift-tested, so the corpus is a checked projection. Accepted, not a
defect.

`grep -rn` over `crates/mochiko-cli/tests` and `src`: the only `plugins/mochiko/schemas` hits are two synthetic
in-test YAML fixtures, one history comment, the `RETIRED_PHRASINGS` needle itself, and `genesis.rs`, which joins
the path onto a caller-supplied `--root` that in-tree only ever points at the frozen fixture. No test reads a
deleted script; the only live `scripts/` read is the allowlist P2 keeps. The 48 names in
`tests/fixtures/python-matrix/check-names.txt` are byte-identical and in source order to `check("…")` in `git
show 5f775ea:scripts/test-find-similar-rules.py` (`diff` exit 0).

## 5. Migration `0003` — PASS on substance, one count wrong in the report
Header fields conform to the log README: `grammar` 1, `id` matching the stem, `sequence` 3 matching the filename
prefix, one-line `intent`, anchor `2026-09-03 cli-schema-delivery D9` in `YYYY-MM-DD <slug> D<n>` form. **The
hash verifies**: `migrate validate --plugin-root plugins/mochiko` reports **0 rejecting · 105 advisory**, and
tampering the anchor in a scratch copy raises `hash-mismatch`, so the anchor is genuinely covered. **The op
counts in the report are wrong:** the file carries **24 changes, seven `set-var`**, not "Twenty-three changes:
six `set-var`" — `tasks_schema` is set twice, on `command/implement` and `command/feature`, so six distinct var
names but seven ops. The crate agrees with the file, not the report: `the_delivery_vars_name_a_cli_invocation`
declares `[(&str, &str, &str); 7]`. The three `set-rule-field pointer: null` are
`analysis-codebase.deliverable-two-arm-binding`, `patterns-architecture-shelves.opinions-in-data` and
`patterns-vertical-tdd.tasks-binding-two-arm`, and they are **necessary, not cosmetic**: replaying the pre-wave
log against the current tree yields exactly three rejecting `pointer-unresolved` findings, one per cleared
pointer.

**The 14 rewords are minimal.** I emitted views from the log at `62aa99d` into a scratch directory and diffed
rule texts by ID across all 50 documents: 12 documents changed, 13 section rule texts plus
`authoring-common.two-arm-template` in the common library — 14, matching the migration. Every opcode is a
deletion of the two-arm clause or the schema path, the remainder byte-identical; no rule ID vanished or
appeared; `class`, `kind`, `labels`, `when`, `enforces`, `extends` unchanged everywhere; both reworded floors
keep their class. The seven vars all moved from a `plugins/mochiko/schemas/*.yaml` path to a CLI form. `${var}`
substitution reads correctly in the rendered sections — spot-rendered five, including `arch.tools-store-schema`:
"The store shape is mochiko-cli template architecture-store, the shelf data mochiko-cli doc
architecture-shelf-backend." *Nit:* the CLI form is backticked in some rewords and bare in others, a cosmetic
consequence of the minimal-strike rule.

## 6. State-wide sweep — PASS, red-proved
`no_document_in_the_state_names_a_schema_file_or_the_absence_arm` pins all three phrasings over the serialized
value of every document, not rule text alone. Red-proof on the pre-`0003` log: the state carried 17
`plugins/mochiko/schemas/`, 7 "when the binary is absent" and 3 "when the binary is available" occurrences.
After `0003`: **zero of all three.**

## 7-9. Pins, state hash, four layers — PASS
Pointers checked 87 → **84**; allowlist-suppressed edges 181 → **169**; similar-rule clusters 0 → **0** — all
three recomputed by me, reasons carried in code comments at `tests/validate.rs` and `tests/matrix_similar.rs`;
the cluster count staying at zero carries the argument that the twelve unsuppressed edges did not resurface as
findings. `migrate status` gives `sha256:57114e5d686bab4d81d6c03532af798a8985dc427de202145aa6092a3e4ef070 · 50
documents · 1016 rules`, as reported, the pre-wave log replaying to `sha256:89728910…d43fd`; three consecutive
replays are byte-identical. `cargo test --all`: **349 passed, 0 failed** across fourteen targets; `cargo fmt
--all --check`, `cargo clippy --all-targets -- -D warnings` and `cargo audit --deny warnings` (31 dependencies)
all exit 0; `MOCHIKO_FULL_SIMILAR=1 cargo test --release --test matrix_similar`: 48.

## 10. Log README — PASS
The wave-4 snapshot paragraph is replaced accurately: no schema file ships, the clause expired, the projection
is `.mochiko/schema-views/` with its regeneration command, read and diffed but never hand-edited, and the view ≡
replay claim is keyed to the surviving surface. Every claim checks out against the tree; its remaining mention
of `plugins/mochiko/schemas/*.yaml` is historical narration, outside the state the sweep covers.

## 11. Report honesty — two corrections owed
All five disclosed deviations are true and verified: the scoped-down assertion (§12), the wider `doc` error
surface (four redirect classes), the third pinned phrasing, the grown CLI fixture (document pin 3 → 5, duplicate
`skill-labels` import dropped), and P3's concurrent `evals/contract/` edits. Fourteen new-or-rewritten tests
counted and matched: 5 `doc`, 3 legend, 4 fidelity, 2 views. Two prose errors should be fixed before the report
stands as the landing record: **§6 op counts**, where "Twenty-three changes: six `set-var`" must read **24
changes, seven `set-var`**; and the **§2 target table**, where `matrix_command` is 2 tests and `migration` 33
rather than the reverse, `migration` and `matrix_skill` also swapped as 3 · 2 (the 349 total is right). Neither
error is in the artifact, but both are record errors in a wave whose case rests on reconstructibility.

## 12. Follow-up now landable
Deviation 1 left "no schema file ships" unpinned in the crate because P2's deletions had not landed. **They
have**, and the full suite is green in that state, so the ~15-line tree-level pin P1 offered can land now. Only
`plugins/mochiko/skills/patterns-api-contracts/references/OPENAPI-TEMPLATE.yaml` remains as a plugin `.yaml`,
and `references/` is exempt, so the pin must allow it. One further point for the lead, not a finding:
`plugins/mochiko/migrations/README.md` sits under the `primitive-edits.md` path scope and this edit removes
content, but it is not a command, skill, agent or template, no strip file for it exists, and waves 3 and 4
edited it without one. P2's new strips-README note covers schema content and the 50 deletions, not this file.
Precedent says no strip is owed; worth a ruling if the lead disagrees.

---

# Delta verdict — 2026-09-05

**Verdict: PASS. No defect remains.** The pin landed, both prior corrections are applied, and the tree is
green. This closes the §11 corrections and the §12 follow-up from the audit above.

**The pin — `no_schema_file_ships_in_the_plugin` in `crates/mochiko-cli/tests/render.rs`.** It has three
arms, not the two P1's plan described: the `schemas/` directory is absent, no `skills/<name>/schema.yaml`
is a file, and the OpenAPI reference file still exists. I red-proved each independently against the file
as it now stands, every mutation cleaned up and `git status` verified clean afterwards:

- Arm 1 — created an empty `plugins/mochiko/schemas/`. FAILED at `render.rs:1318`, "plugins/mochiko/schemas/ still exists".
- Arm 2 — touched `plugins/mochiko/skills/review-brainstorm/schema.yaml`. FAILED at `render.rs:1330`, naming that path.
- Arm 3 — moved the OpenAPI file aside. FAILED at `render.rs:1336`, "reference YAML is not schema data". Restored byte-identical by md5, and `git status` over that skill is empty.

Each arm fails on its own assertion with its own message, so they discriminate rather than collapsing into
one. **The `references/` exemption is sound and no longer accidental:** arm 2 globs only
`<skill-dir>/schema.yaml`, so `patterns-api-contracts/references/OPENAPI-TEMPLATE.yaml` was never in its
path, and arm 3 now pins that survivor so a future widening into "no `.yaml` under `plugins/`" fails here
instead of deleting a legitimate file. One scope note, not a defect: arm 2 sees only a skill's own
directory, so a `schema.yaml` nested deeper would pass — that is the shape retired at wave 6, and the
contract suite's run-wide no-Read assert covers the reading half.

**Layers re-run by me, all green.** `cargo test --all`: **350 passed, 0 failed**, with `render` at 39 —
both figures matching P1's claim. `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`
and `cargo audit --deny warnings` (31 dependencies) exit 0; the clippy run was a genuine re-check of the
changed crate, not a cache hit, with `render.rs` content verified unmoved by md5.
`MOCHIKO_FULL_SIMILAR=1 cargo test --release --test matrix_similar`: 48 passed. State hash unchanged at
`sha256:57114e5d686bab4d81d6c03532af798a8985dc427de202145aa6092a3e4ef070 · 50 documents · 1016 rules`;
`migrate validate` still 0 rejecting · 105 advisory with 169 suppressed edges.

**Both corrections applied** in `p1-endstate.md`. §6 now reads "Twenty-four changes: seven `set-var`,
three `set-rule-field`, fourteen `reword-rule`" and names the double-counted `tasks_schema`; §2's table
now reads `migration` 33 · `matrix_skill` 3 · `matrix_command` 2, matching my per-target measurement. §2's
headline still says 349 across a 38-test `render`, which is the figure at original-report time; §12a
states the post-delta 350 / 39 explicitly, so the record is layered rather than stale. No action needed.
