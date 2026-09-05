# V2 — audit of seat P2 (plugin side + rules file), wave 6

**2026-09-05. Verdict: PASS, with five findings (F1–F5), none blocking on its own.** Graded from the files
and the diff at `62aa99d`, never from P2's report; every claim re-derived here. **F1 and F2 are false claims
in text this wave exists to make true** and should land with it.

**1. Deletions — PASS.** `plugins/mochiko/schemas/` does not exist. `find plugins/mochiko -name '*.yaml'
-not -path '*/migrations/*'` returns one file — `skills/patterns-api-contracts/references/OPENAPI-TEMPLATE.yaml`,
an artifact reference, not a schema; `find plugins/mochiko/skills -name schema.yaml` returns 0. `scripts/`
holds `similar-rules-allowlist.yaml` alone, its first line re-keyed from `scripts/find-similar-rules.py` to
"the similar-rule detector in `mochiko-cli`", data untouched. `.claude/skills/` holds only
`compressing-skills`. The archived sidecar diffs against `62aa99d:.mochiko/provenance.yaml` as **four added
lines and nothing else** — three comment lines carrying all three facts §3.1 demanded, plus a `#` spacer.

**2. Body sites — PASS, fourteen exactly.** Word-level diff of every touched shipped `.md` against
`62aa99d`, whitespace normalized so reflow is invisible: **every touched file changes only the two-arm
phrase, the schema path, or the false claim. No other word moved anywhere.** Counted from it:
`commands/specify.md` ×1 · router ×7 (lines 20, 21, 23; rows 57, 58, 74, 97) · four `references/` files ×1
each · `templates/report-format.md` ×1 = **14**. Router row 58 reads `mochiko-cli doc
architecture-shelf-backend`; `report-format.md` cites `impl.escalation-batching` by id alone. Both render
(release binary built here): `doc architecture-shelf-backend --plugin-root plugins/mochiko` exits 0, 89
lines, head `mochiko-cli doc architecture-shelf-backend · binary 0.1.0 · grammar 1 · plugin 0.106.0`, end
`mochiko-cli doc end · architecture-shelf-backend`; `template architecture-store` exits 0. Every other cited
name renders too: templates `spec` `tasks` `codebase-analysis` `governance-intent` `governance-surfaces`
`feature-entry` `features-index`; docs `command-labels` `skill-labels`.

**3. Rewordings — PASS, eighteen.** Same word diff: `analysis-codebase` 2 · `authoring-constitution` 1 ·
`authoring-feature-map` 1 · `authoring-prototype` 2 · `authoring-user-stories` 1 · `executing-tdd-cycle` 2 ·
`patterns-code-minimalism` 1 · `patterns-plan-minimalism` 1 · `patterns-vertical-tdd` 1 ·
`review-code-minimalism` 1 · `testing-end-user` 4 · `testing-gap-finding` 1 = **18**. Each replaces only the
"in the schema" / "the schema's" claim; every section id survives. The six P2 flagged as discovered are real
and its line numbers exact: `authoring-prototype:54`, `review-code-minimalism:67`,
`authoring-constitution:101`, `authoring-user-stories:42`, `testing-end-user:76` and `:80` each carry one at `62aa99d`.

**4. No survivor — PASS.** `grep -rn` over `plugins/` and `README.md` for `plugins/mochiko/schemas/`, "in
the schema" and "the schema's": **zero rows**. For "binary is absent|available|missing": eleven rows in
`plugins/mochiko/migrations/0001-genesis.yaml` (P1's; an applied migration is never edited in place) and one
comment at `dependency-halt.sh:7` describing the hook's own gate — exactly the two P2 listed, both ruled
legitimate. No artifact-schema hits survive, so P2 listed none. The delivered surface is clean too: all 252
rendered blocks (36 × 7), 6,632 lines, contain none of the four patterns.

> **F1 — a survivor P2's grep set could not catch.** `dependency-halt.sh:76–78` still reads "A primitive
> that still reads a shipped schema file is covered by **the transition clause** and is never gated". The
> clause expires at v3.0.3 and no such primitive exists. Behavior is correct — eight shipped skills carry no
> `!` lines and are correctly not gated — but the rationale is false in shipped text.

**5. Strips — PASS.** Eighteen files carry a `## [v0.107.0]` entry, each with `Disposition: superseded`,
`Tier failed: n/a — supersession by ruling`, a `Content:` block, `Kept deliberately:` and the
`cli-schema-delivery/record.md` citation, inserted newest-first. `Consumers assessed:` on `mochiko.md` and
`report-format.md`. The router's one entry lists all seven of its lines in file order. **Verbatim,
machine-compared:** a script pulled all 32 quoted spans from the eighteen `Content:` blocks and searched a
whitespace-normalized corpus of the whole pre-wave plugin tree — 31 matched directly, the 32nd
(`analysis-codebase`) once blockquote `> ` markers were stripped, my normalizer's artifact and not a fidelity
defect. **32/32 verbatim.** `.mochiko/strips/README.md` carries the schema-content note, and correctly says body prose still takes entries.

**6. Rules-file re-key — PASS against §3.4.** `paths` drops `plugins/mochiko/schemas/**` and
`.mochiko/provenance.yaml`; six globs stand. The "Schema data files" paragraph is replaced by §3.4's wording
— a scripted word-compare against the plan finds the sole difference is the bold markers on the lead-in,
which the plan's italic wrapper made ambiguous. Command criterion 9 and skill criterion 10 (that block's
pre-pass has always been numbered 10) both name `mochiko-cli migrate validate --report --plugin-root
plugins/mochiko` and the Python retirement. Command criterion 10 and skill criterion 12 name the log's own
rules as the anchor home, binary-enforced at apply, and the frozen sidecar at its archive path. Criterion
11's co-Read clause and skill criteria 1 and 6 all read "No co-Read … is demanded: the render resolves every
stub before the model sees it." `grep -i "on a converted"` returns zero, as do the schemas-path and sidecar
greps. Every path the file cites exists. Beyond §3.4's list, three sentences naming the schema file as half
of the graded unit were re-keyed (author≠grader paragraph, both criteria-block openings); P2 disclosed this,
and leaving them would contradict the new paragraph and keep the very path §3.4 strikes, so it is in scope.

> **F3.** Striking skill criterion 8's unconverted branch also dropped two non-branch sentences: "The family
> common file is budgeted once as its own primitive, never per binding skill" and "the audit grading the
> delta against the pre-conversion body figure as structural overhead only (IDs, keys, grammar)". Both judged
> discharged — no family common file ships, no unconverted skill remains — but the lead should confirm.
>
> **F4 — scoping, the lead's call.** `.mochiko/strips/primitive-edits.md` exists with three precedent entries
> for this exact class of criteria-block supersession (v0.97.0, v0.98.0, v0.99.0); this re-key took none, and
> brief and plan both scope the wave at eighteen, so it is not a seat defect.
>
> **Note, not a finding.** §3.4's premise "no unconverted primitive remains" is false for skills: eight
> shipped skills carry no rules in the log — `analysis-iterative`, `grooming-operating-docs`, `mochiko`,
> `patterns-api-contracts`, `patterns-entity-modeling`, `patterns-system-design`,
> `patterns-technical-decisions`, `testing-governance-injection`, each verified by running `rules <name>
> --section preamble`. The re-keyed block opens "every skill", so it claims to govern eight with no second
> surface. P2 applied the lead's wording as instructed; the wording is the lead's to revisit.

**7. Sanitizer — PASS.** `require_bare_name()` sits beside `escape()`, called at lines 48 and 61 — **after**
`bare` is derived (47, 60), **before** any path is built (49, 62). Rejects `*[!A-Za-z0-9_-]*` and the empty
string with a silent `exit 0`. `sh -n` and `dash -n` both clean. The eleven wave-5 matrix rows, run against
both `62aa99d`'s script and the current one (`CLAUDE_PLUGIN_ROOT=plugins/mochiko`, `target/release` on/off
`PATH`): **every row byte-identical** in exit code and output; the four sanitizer rows likewise, for P2's
stated reason — with no file at the traversal target the pre-edit script fell through. **P2's planted-file
probe reproduces exactly** — scratch root, a file carrying the `!`-line marker planted at the traversal
target, `command_name: mochiko:../../tmp/x`:

```
PRE-FIX   exit=2  mochiko-cli is not installed — /mochiko:../../tmp/x cannot run without it. …
POST-FIX  exit=0  (silent)
```

Removing the planted file makes the pre-fix run exit 0 silently and restoring it makes it halt again, so the
pre-edit hook did read outside the plugin root and echo the attacker-controlled string. The guard closes it.

**8. Renders — PASS.** Scripted over every `` !`mochiko-cli rules `` line in all six commands and all
thirty-eight skill directories: **36 primitives deliver 7 blocks each, 252 blocks, zero bad** — each with its
`mochiko-cli rules <primitive> · section <id> · …` head line and `mochiko-cli rules end · <primitive> · <id>
· <N> rules` end line, exit 0. The other 8 carry no `!` lines (§6's set).

**9. README.md — PASS.** Carries "**The plugin ships no schema file at all**: the log is the only rule data
it carries, so there is nothing a command could read instead of asking the binary" — true per §1 — and gains
`mochiko-cli doc <name>` in the usage block. No false claim: the eight `template` names are exactly the eight
views under `.mochiko/schema-views/templates/`, and the three `doc` names exactly what the binary reports as
available. P2's disclosed deviation — adding `architecture-store` against plan §7 — makes that list complete.

**10. Report honesty — PASS, with F5.** Substantively honest: the eighteen-not-twelve flag, the reword line
numbers, the traversal probe's interpretation and the allowlist repair all check out, and P2 surfaced the
growth and the deviation rather than burying them. Four loose numbers:

| P2's report | measured here |
|---|---|
| `cargo test --all`: 349 pass, 14 suites | 350 pass, 0 fail, 14 suites |
| "1,708 KB after, down 533 KB" | 1,708 KB after, down **636 KB** (533 KB is the byte total of the 50 files, mixed with a `du` figure) |
| "a three-line freeze header" | three comment lines plus a `#` spacer — 4 added lines |
| §9.3: ledger present-tense on the Python checkers | already past-tense, "retired … at wave 6 (v0.107.0 …)" — discharged |

**Char budgets — recorded, not graded, per the brief.** P2 reports that measuring payload as `SKILL.md` body
plus the seven rendered blocks puts every touched skill over its v0.106.0 row by +85 to +567, that four
untouched control skills read over by +334 to +531 the same way, and that the gap is therefore not a wave-6
regression. Body deltas it did cause: largest +42 (`executing-tdd-cycle`), smallest +1 (three bodies), router
−299, net −98 across 13 bodies. P2 asks for a ruling before the bump because the rows say "no headroom".

**Fix list.**

- **F1 — land with the wave.** Reword `dependency-halt.sh:76–78` to key on "a primitive whose rules come from
  the binary"; new strip file `.mochiko/strips/dependency-halt.md`.
- **F2 — land with the wave.** `.mochiko/memory/primitive-cost-budgets.md` lines 77, 80 and 237–238 carry
  `skill-review-common.yaml` and `skill-authoring-common.yaml` as budgeted shipped primitives and say they
  "still ship"; both were deleted this wave, and `primitive-edits.md` cites that file as the canonical budget
  home. Outside P2's §1 ownership. **F3** — confirm the discharge of skill criterion 8's two dropped
  sentences (§6). **F4** — no strip entry for the `primitive-edits.md` re-key (§6). **F5** — four loose
  numbers in P2's report (§10); none changes a verdict.

---

## Delta re-audit — 2026-09-05

**Verdict: fixes (1) and (2) PASS; fix (3) FAILS on one sentence — a one-line repair.** Graded from
the files, re-running every probe from the original audit.

**(1) `dependency-halt.sh` comment — PASS.** Lines 76–79 now read "The delivery check. A primitive
with no `!` line has no rules to deliver — the seven prose skills and the router carry procedure
only, and never had a rule set — so the binary's absence cannot break it and it is never gated."
Every claim in it is true: 38 skill directories, 30 carrying a `!` line, 8 without, the router
(`mochiko`) among the 8, and **none of those 8 had a `skills/<name>/schema.yaml` at `62aa99d`** —
"never had a rule set" checks out. The guarded `grep -q -F` line is byte-identical. `sh -n` and
`dash -n` clean. All eighteen matrix rows (the eleven wave-5 rows plus the four sanitizer rows and
their absent twins) are byte-identical between `62aa99d`'s script and this one, and the planted-file
traversal probe still shows pre-fix `exit=2` naming the attacker string against post-fix `exit=0`.
*Carry-forward:* F1's second half is unapplied — this edit **removed** comment content from a
primitive under `plugins/mochiko/hooks/**`, which the ceremony's own `paths` covers, and there is
still no `.mochiko/strips/dependency-halt.md`.

**(2) `primitive-cost-budgets.md` — PASS, no figure moved.** The two rows keep `1,627` and `1,285`
in both Budget and Cap columns; only their parentheticals change, and the appended `[v0.107.0]`
note explicitly supersedes the "they still ship" clause. Verified mechanically rather than by eye:
a frequency table of every numeric token in the file, before against after, shows **no count
decreased anywhere** — every delta is an added occurrence from the note itself (`1,627`, `1,285`,
and the `v0.107.0` / `D9 wave 6` / `2026-09-05` stamps). No pre-existing figure changed, and no
member row was re-seeded, exactly as the note claims.

**(3) `primitive-edits.md` skill-pair block — FAIL.** The block itself is right: it opens
"Skill-pair criteria — every schema-bearing skill (the thirty)", routes "the seven prose skills and
the router" to the plain primitive ceremony, and `grep -ni "every skill\|converted skill"` returns
zero. The census it asserts is the one I measured (38 / 30 / 8, router among the 8). **But the
author≠grader routing paragraph one screen above was not re-keyed with it** — line 51 still reads
"For a **skill** the graded unit is likewise the pair — `SKILL.md` + that skill's rendered rules".
That is unconditional, and its only escape hatch is "For every other primitive", which a prose
skill is not. So the routing still sends all 38 skills to the pair form that the new block excludes
8 of them from, and a grader following it would demand rendered rules for `patterns-api-contracts`,
which has none. The two surfaces now contradict each other on exactly the point the fix addressed.

**Fix:** change line 51 to "For a **schema-bearing skill** the graded unit is likewise the pair",
so the routing sentence and the block it routes to carry the same scope. (`converted pair` survives
at lines 54 and 181 — redundant now that every pair is converted, but not false; the lead's named
phrasings are gone.)

### Delta confirm — 2026-09-05

**Fix (3) now PASSES. All three delta fixes green; the wave-6 P2 unit is PASS.**

The routing sentence reads "For a **schema-bearing skill** the graded unit is likewise the pair",
and its fallback now reads "For every other primitive — **the seven prose skills and the router
included** — the matching `validation-*` / `review-*` skill applies". That is the destination the
skill-pair block sends those eight to ("take the plain primitive ceremony above, not this block"),
so the two surfaces agree on scope and the circularity I flagged is closed: nothing routes a prose
skill to a pair form it has no second surface for. `grep -ni "converted\|every skill"` over the
file returns **zero rows** — both "converted pair" phrases are now "the pair".

**Nothing else moved.** The diff against `62aa99d` still carries the same **ten hunks in the same
ten regions** I graded originally, so the three edits landed inside hunks already audited and
opened no new region. Every original item-6 check re-passes on the current file: six `paths` globs
with no `schemas/**` and no sidecar; the §3.4 paragraph still word-identical to the plan bar its
bold markers; two `migrate validate --report --plugin-root` citations and two frozen-sidecar
citations; zero hits for `plugins/mochiko/schemas/`, the sidecar path, or "on a converted"; the
three discharged co-Read clauses intact; and the set of file paths the document cites is
**byte-identical** to the set I extracted in the original audit, so no pointer was added or lost.

*Standing carry-forward (unchanged):* the hook's comment edit removed content from a primitive
under `plugins/mochiko/hooks/**`, and `.mochiko/strips/dependency-halt.md` still does not exist.
