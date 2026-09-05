# P2 — the end-state plugin tree (wave 6, v0.107.0)

Seat P2: deletions, body sites, rewordings, strips, the hook sanitizer, the rules-file re-key, the
two READMEs. Executed against `62aa99d` after the lead signalled P1 closed. Nothing committed.

**Every check green.** No schema file survives anywhere in the plugin, no shipped text names
`plugins/mochiko/schemas/` or an absence fallback, all 349 crate tests pass, every touched skill
still renders 7/7, and every char budget holds with 195 to 408 chars to spare. Two things the lead
should read before the audit: the reword set grew from twelve to eighteen (§3), and one dangling
pointer outside the plan's list was repaired (§7).

## 1. Deletions

Fifty snapshot files, six Python files, one repo skill, one sidecar move.

| what | count | size at HEAD |
|---|---|---|
| `plugins/mochiko/schemas/` (directory gone) | 20 | 223 KB |
| `plugins/mochiko/skills/*/schema.yaml` | 30 | 310 KB |
| `scripts/*.py` (3 checkers + 3 test matrices) | 6 | 238 KB |
| `.claude/skills/converting-command-to-schema/` | 1 | — |

`plugins/mochiko` is 1,708 KB after, down 533 KB. `scripts/similar-rules-allowlist.yaml` stays and
is the only file left in `scripts/`. Untracked `scripts/__pycache__/` removed from the working
tree. `.mochiko/provenance.yaml` moved to `.mochiko/archive/provenance-frozen-2026-09-05.yaml` with
a three-line freeze header above its existing first line: frozen at the wave-6 landing, anchors now
live on the log's rules and the binary enforces them, provenance queries only.

## 2. Body sites — fourteen

The twelve from plan §0 plus router lines 20 and 23 under the lead's ruling 2. Each is one phrase
substitution; no other word changed, and four paragraphs were rewrapped to their existing column
width because removing text mid-paragraph shifts the wrap.

- `commands/specify.md` Goal — the two-arm spec-template clause loses its raw-Read arm and the
  "the shipped schema is the first-class source of truth" gloss that rode with it.
- `skills/mochiko/SKILL.md` ×7 — line 20 now says each command ships as a `.md` whose rule set
  lives in the migration log; line 21 says the rules are rendered at fire by `mochiko-cli`; line 23
  says the rule set is delivered in six sections; rows 57, 74 and 97 lose their fallback arms;
  row 58's phantom `template architecture-shelf-backend` becomes `doc architecture-shelf-backend`.
- Four `references/` files — `analysis-codebase` (codebase-analysis), `validation-constitution`
  (governance-surfaces), `authoring-constitution` (governance-intent),
  `authoring-technical-requirements` and `review-plan-artifacts` (architecture-store ×2).
- `templates/report-format.md` rule 9 — the implement rules are now cited by id alone.

Line 20 states the home and line 21 the delivery, deliberately, so the paragraph does not say
"rendered at fire by `mochiko-cli`" twice in consecutive sentences.

## 3. Rewordings — eighteen, not twelve

The twelve the lead ruled, plus six more of the identical class that the phase-1 grep missed.

**The twelve ruled.** Seven across the four dense-five bodies (`analysis-codebase` ×2,
`executing-tdd-cycle` ×2, `testing-end-user` ×2, `testing-gap-finding`), and five across the bodies
added by ruling 3 (`authoring-prototype`, `authoring-feature-map`, `patterns-code-minimalism`,
`patterns-plan-minimalism`, `patterns-vertical-tdd`).

**The six discovered.** My phase-1 grep searched the literal string "in the schema" and therefore
missed the possessive form "the schema's X", which is the same claim in different words. Check 6
caught them: `authoring-prototype:54`, `review-code-minimalism:67`, `authoring-constitution:101`,
`authoring-user-stories:42`, `testing-end-user:76` and `:80`. Each names the skill's own rule set
as living in a file that no longer ships, two of them inside a body I had just fixed.

I reworded all six rather than listing them. The lead's ruling 3 stated the principle as "same
class, same wave", and these are that class without argument. Flagging it prominently instead of
burying it, because V2 will otherwise audit against a plan that says twelve. Every section id
survives untouched in all eighteen; only the delivery noun moved.

## 4. Strip entries — eighteen files

One supersession-by-ruling entry per touched primitive, stamped `[v0.107.0]`, newest-first, each
under a wave-context comment naming `git show 62aa99d:<path>` for the verbatim pre-edit text. Every
entry cites `.mochiko/brainstorms/cli-schema-delivery/record.md` D9 wave 6 and the `DECISIONS.md`
2026-09-05 row, carries the superseded text verbatim, and states what survived.

Sixteen from the plan plus `review-code-minimalism.md` and `authoring-user-stories.md` for the
discovered six. `analysis-codebase` takes one entry covering both its reference file and its body;
`mochiko.md` one covering all seven router lines. `Consumers assessed:` appears on the two shared
primitives, the router and `report-format`. The fifty deleted schema files take no entries, per
plan §7, and the sanitizer takes none as a pure addition.

## 5. Hook sanitizer

`dependency-halt.sh` gains one `require_bare_name()` guard, defined once beside `escape()` and
called in both branches immediately after `bare` is derived, before any path is built. The
`PreToolUse` branch now derives `bare` explicitly so both branches share the guard.

```sh
case "${1:-}" in
*[!A-Za-z0-9_-]* | "") exit 0 ;;
esac
```

**The eight wave-5 matrix rows plus four new sanitizer rows produce byte-identical output before
and after.** That is not a null result, and it needed a second probe to interpret. The pre-edit
script also exited 0 on a traversal-shaped name, but for a different reason: it built the path,
failed to find a file there, and fell through. With `CLAUDE_PLUGIN_ROOT` pointed at a fixture and a
file planted at the traversal target carrying the converted marker, the pre-edit hook read that
file and fired a halt naming the attacker-controlled string:

```
BEFORE  name=mochiko:../../evil  →  exit=2, "mochiko-cli is not installed — /mochiko:../../evil …"
AFTER   name=mochiko:../../evil  →  exit=0, silent
```

So the guard closes a real read outside the plugin root. `sh -n` clean; POSIX `case` only, no new
dependency.

## 6. `primitive-edits.md` re-key

Frontmatter drops `plugins/mochiko/schemas/**` and `.mochiko/provenance.yaml`; six globs stand. The
"Schema data files" paragraph is replaced whole by the lead's §3.4 wording. Criterion 9 in both
blocks becomes the `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` pre-pass.
Command criterion 10 and skill criterion 12 re-key anchors onto the log's own rules, binary-enforced
at apply, with the sidecar named at its frozen archive path. Criterion 11's co-Read clause and skill
criteria 1 and 6 lose their unconverted branches entirely. Every "on a converted …" prefix is gone:
command criteria 1 and 3, skill criteria 1, 2, 3, 6 and 8 now read unconditionally.

Three sentences named the schema file as half of the graded unit and are re-keyed too, since the
file no longer exists: the author-not-grader paragraph, the canonical-scaffold block's opening, and
the skill-pair block's opening. All now name the rendered rules as the second surface.

Closing greps: no `plugins/mochiko/schemas/` and no "On a converted" survives in the file.

## 7. The two READMEs, and one repair

`.mochiko/strips/README.md` gains a section stating that schema content is recorded by the
migration log and not by strip entries from v0.107.0, that the log carries verbatim prior content
by construction, and that the fifty deleted files are recorded by the log plus the D9 wave-6 ruling.
It says explicitly that body prose in a `SKILL.md`, command `.md`, `references/` file or template is
unaffected and still takes entries.

`README.md` gains a sentence that the plugin ships no schema file at all, the `mochiko-cli doc
<name>` line in the usage block, and a sentence naming the three `doc` names. I also added
`architecture-store` to the `template <name>` list, which the plan said to leave alone: the list was
already missing it before this wave, and my new `doc` sentence draws a contrast that would be wrong
if the template list stayed incomplete. Verified: `template architecture-store`, `doc command-labels`
and `doc skill-labels` all exit 0.

**Repair outside the plan's list.** `scripts/similar-rules-allowlist.yaml` survives, but its first
line credited `scripts/find-similar-rules.py`, which I deleted. A dead pointer is a defect, and this
one is a direct consequence of my own deletion, so I re-keyed the header to the crate's detector and
noted the retirement. One comment line; no data touched.

## 8. Checks

| # | check | result |
|---|---|---|
| 1 | no `plugins/mochiko/schemas/` or absence phrase in shipped text | clean, all file types |
| 2 | deletions land; allowlist alone in `scripts/`; sidecar only under archive | clean |
| 3 | hook matrix, 8 wave-5 rows + 4 sanitizer rows | identical before/after; traversal probe proves the guard |
| 4 | render survival, 13 touched skills | 7/7 sections each, head and end lines, 0 bad |
| 5 | `mochiko-cli doc architecture-shelf-backend` | exit 0, both wrapper lines |
| 6 | no rule-home "in the schema" / "the schema's" survivors | clean after the six of §3 |
| 7 | `cargo test --all` | 349 pass, 0 fail, 14 suites |

Check 1 needs one note. The raw `0001-genesis.yaml` still contains the old two-arm text, because an
applied migration is never edited in place. The test that matters is the **replayed** state, and
`.mochiko/schema-views/` is clean of both patterns. The one surviving "binary is missing" in
`dependency-halt.sh` is a comment describing what the hook gates, not a fallback instruction.

**Char budgets — nothing is over; my first measurement was wrong.** I re-measured with the
canonical snippet from `.mochiko/memory/primitive-cost-budgets.md` §"How to measure", which counts
the body as **characters after the closing frontmatter fence**. My first script counted whole
files, so every body carried its frontmatter — including a `description:` value of up to 1,536
chars. That inflation, not a bytes-versus-chars gap, produced the false overage. Both scripts
counted characters; Python decodes UTF-8 on read.

The corrected method validates itself: it reproduces the v0.106.0 body figure **exactly** for all
four untouched controls and for four touched skills.

| | skill | body | Δbody | render | Δrender | payload | budget | Δ |
|---|---|---|---|---|---|---|---|---|
| * | analysis-codebase | 4,726 | +38 | 8,705 | −383 | 13,431 | 13,776 | **−345** |
| * | authoring-constitution | 7,696 | +1 | 21,582 | −337 | 29,278 | 29,614 | **−336** |
| * | authoring-feature-map | 5,954 | +18 | 15,961 | −426 | 21,915 | 22,323 | **−408** |
| * | authoring-prototype | 4,620 | +19 | 10,057 | −322 | 14,677 | 14,980 | **−303** |
| * | authoring-technical-requirements | 4,038 | +0 | 16,492 | −245 | 20,530 | 20,775 | **−245** |
| * | authoring-user-stories | 4,530 | +1 | 8,678 | −237 | 13,208 | 13,444 | **−236** |
| * | executing-tdd-cycle | 7,382 | +42 | 12,486 | −237 | 19,868 | 20,063 | **−195** |
| * | patterns-code-minimalism | 3,271 | +12 | 6,822 | −237 | 10,093 | 10,318 | **−225** |
| * | patterns-plan-minimalism | 3,846 | +12 | 6,753 | −237 | 10,599 | 10,824 | **−225** |
| * | patterns-vertical-tdd | 5,908 | +12 | 9,520 | −359 | 15,428 | 15,775 | **−347** |
| * | review-code-minimalism | 3,712 | +1 | 6,716 | −237 | 10,428 | 10,664 | **−236** |
| * | review-plan-artifacts | 3,363 | +0 | 14,341 | −309 | 17,704 | 18,013 | **−309** |
| * | testing-end-user | 9,596 | +33 | 11,910 | −237 | 21,506 | 21,710 | **−204** |
| * | testing-gap-finding | 5,966 | +12 | 13,786 | −237 | 19,752 | 19,977 | **−225** |
| * | validation-constitution | 3,263 | +0 | 11,517 | −237 | 14,780 | 15,017 | **−237** |
| | brownfield-integration | 4,831 | +0 | 7,786 | −237 | 12,617 | 12,854 | **−237** |
| | review-brainstorm | 2,833 | +0 | 9,754 | −237 | 12,587 | 12,824 | **−237** |
| | review-feasibility | 2,721 | +0 | 9,262 | −237 | 11,983 | 12,220 | **−237** |
| | review-sufficiency | 3,262 | +0 | 11,924 | −237 | 15,186 | 15,423 | **−237** |

`*` = touched by this seat. **Nothing is over budget.** Every payload sits 195 to 408 chars under
its row, and the rows stand unchanged.

Two facts fall out of the render column. **Every untouched control is −237 exactly** — a constant,
which is P1's six-line skill legend variant dropping the `fail` kind and the two `enforces:` lines
from every skill preamble. **Seven touched skills shrink further** — `analysis-codebase`,
`authoring-constitution`, `authoring-feature-map`, `authoring-prototype`,
`authoring-technical-requirements`, `patterns-vertical-tdd`, `review-plan-artifacts` — and those
are precisely the skills whose rule texts carried two-arm phrases in plan §0, so the extra is
migration `0003`. The two independent shrink sources reconcile with the wave's design.

My own contribution is body only: +0 to +42 chars per skill, **+201 net across the fifteen budgeted
skills**. The router is −299 but carries no budget row (it is user-invoked and outside the
budgeted set), so it does not enter this table. I did not touch
`.mochiko/memory/primitive-cost-budgets.md`.

## 9. For the lead

1. **The reword set is eighteen, not twelve.** V2 should be briefed on the number so it does not
   read six extra edits as scope creep. §3 has the list and the reasoning.
2. **No budget ruling is needed.** The overage I first reported was my measurement error, corrected
   in §8: every skill sits 195 to 408 chars under its row and the rows stand. The audit brief should
   carry the §8 table, and the first table I sent should be disregarded.
3. Everything in §1 through §7 is uncommitted and in the worktree.

---

## 10. Delta — three text fixes after V2 PASS (2026-09-05)

Lead-directed after the V2 audit at `wave6-reports/v2-endstate-audit.md`. No ruling involved, no
figures changed, no behavior changed. The hook matrix re-ran and is byte-identical across all
twelve rows; checks 1 and 6 re-ran clean.

**One correction to the brief.** The lead's fix-3 wording said "eight prose skills and the router".
The tree has 38 skill directories: **30 carry a `!` rules line, 8 do not, and the router is one of
those 8**. So it is seven prose skills plus the router, not eight plus the router. I wrote the
accurate count in both fix 1 and fix 3. The seven are `analysis-iterative`,
`grooming-operating-docs`, `patterns-api-contracts`, `patterns-entity-modeling`,
`patterns-system-design`, `patterns-technical-decisions`, `testing-governance-injection`.

### Fix 1 — `plugins/mochiko/hooks/scripts/dependency-halt.sh`, the delivery-check comment

```diff
-# The converted check. A primitive that still reads a shipped schema file is covered by the
-# transition clause and is never gated; one whose rules come from the binary is. The primitive's
-# own file is the truth here, so there is no list to keep in sync with the conversion waves.
+# The delivery check. A primitive with no `!` line has no rules to deliver — the seven prose
+# skills and the router carry procedure only, and never had a rule set — so the binary's absence
+# cannot break it and it is never gated. One whose rules come from the binary is. The primitive's
+# own file is the truth here, so there is no list to keep in sync.
 grep -q -F '!`mochiko-cli rules' "$primitive" 2>/dev/null || exit 0
```

Comment only; the guarded line is untouched. `sh -n` clean, matrix unchanged. The reworded comment
also states *why* an ungated primitive is safe rather than merely that it is exempt: with no rules
to deliver, the binary's absence cannot break it.

### Fix 2 — `.mochiko/memory/primitive-cost-budgets.md`, a `[v0.107.0]` note

Added after the `[v0.106.0]` block, which said the two common files "still ship". I did not edit
that block, since prior blocks stand as history by this page's own convention; the new note
supersedes the clause explicitly.

```diff
+**[v0.107.0] the two family common rows are no longer shipped files — figures unchanged.** The
+wave-6 end state deleted every schema file from the plugin (`cli-schema-delivery` D9 wave 6,
+`DECISIONS.md` 2026-09-05), `skill-review-common.yaml` and `skill-authoring-common.yaml` among
+them. They are now **documents in the migration log**, resolved into every render before the model
+sees them, so the `.yaml` suffix in their row names is a historical name rather than a path on
+disk. This **supersedes the "they still ship" clause** in the [v0.106.0] block above, which was
+true when written. Their rows stay exactly as they are — 1,627 and 1,285, no headroom — re-read as
+**the budget of the shared block text**, not of a shipped primitive: the text still has a size
+worth holding, it is still budgeted once rather than per binding skill, and it is still absent
+from every member's payload because the render resolves each `extends:` stub. No figure on this
+page changes at wave 6, and no member row is re-seeded: the wave moved no block text.
```

Each of the two rows also gains a pointer so a reader landing on the row alone is not misled. Both
still read "budgeted once", and neither figure moves:

```diff
-| skill-review-common.yaml | 1,627 (family common file, budgeted once as its own primitive — seeded [v0.100.0] schema conversion) | 1,627 (no headroom) |
+| skill-review-common.yaml | 1,627 (family common file, budgeted once — seeded [v0.100.0] schema conversion; a log document, not a shipped file, from [v0.107.0] — see the note) | 1,627 (no headroom) |
```

`skill-authoring-common.yaml` takes the same shape at 1,285. **Verified no figure changed**: every
number in the file was extracted and counted before and after, and the only differences are three
additions my note introduces (one `1,285`, one `1,627`, one `2026`). Nothing was altered or removed.

### Fix 3 — `.claude/rules/mochiko/primitive-edits.md`, the skill-pair block opening

```diff
-  **Skill-pair criteria — every skill; the review family from v0.100.0.** A
-  skill ships as `SKILL.md` whose rules `mochiko-cli` renders from the log
+  **Skill-pair criteria — every schema-bearing skill (the thirty); the review family from
+  v0.100.0.** The seven prose skills and the router carry no rule set, never had a schema,
+  and take the plain primitive ceremony above, not this block. A schema-bearing skill
+  ships as `SKILL.md` whose rules `mochiko-cli` renders from the log
   (the skill directory stays the self-contained shipping unit) and is graded
   across **both surfaces** on this criteria set.
```

This repairs a premise I carried in from §3.4 without testing it: my earlier rewrite widened the
block from "every converted skill" to "every skill", which silently pulled eight primitives under a
criteria set none of them can satisfy. Grep confirms no "every skill", "A skill ships" or
"converted skill" phrasing survives in the file.

### Fix 3b — the routing paragraph, and two phrase trims (V2 delta round 2)

V2 failed fix 3 on one sentence a screen above the block: the author≠grader routing paragraph
still routed **every** skill to the pair form, so the same eight primitives were mis-routed there
even after the block header was corrected. The block and the paragraph that routes into it now
agree.

```diff
-  `command-md-scaffold-standardization` D1, C1 fold.) For a **skill** the graded
+  `command-md-scaffold-standardization` D1, C1 fold.) For a **schema-bearing skill** the graded
   unit is likewise the pair — `SKILL.md` + that skill's rendered rules — held
   against the skill-pair criteria block below and graded by `mochiko:validator`
-  (skill-content-schema D8/I6; the matching-skill routing never applies to a converted pair).
-  For every other primitive the matching
+  (skill-content-schema D8/I6; the matching-skill routing never applies to the pair).
+  For every other primitive — the seven prose skills and the router included — the matching
   `validation-*` / `review-*` skill applies, graded on internal coherence plus preserved
```

I added the "the seven prose skills and the router included" clause to the destination sentence.
The lead's note said those eight "fall to 'every other primitive'", which was left implicit; naming
them there closes the routing loop explicitly rather than leaving a reader to infer it from a
negation two sentences earlier.

The second "converted pair" phrase, in the skill-pair block's grader sentence, takes the same trim:

```diff
-  applies to a converted pair (no validator-for-skills exists, and a pilot member never
+  applies to the pair (no validator-for-skills exists, and a pilot member never
```

**The word "converted" no longer appears anywhere in the file.** That is the right end state: with
no unconverted primitive left, the adjective drew a distinction that no longer exists.

### Strip accounting for the delta round

Per the lead's ruling on V2's carry-forward F1, the hook-comment rewrite owes no strip entry: hook
script comments are not protected content and `hooks/` carries no strip file, the same footing as
the migrations README. The other three edits are outside `plugins/` — two in `.claude/rules/` and
one in `.mochiko/memory/` — and neither is a shipped primitive, so no strip entry is owed for any
of the four. The eighteen `[v0.107.0]` entries from §4 stand unchanged.
