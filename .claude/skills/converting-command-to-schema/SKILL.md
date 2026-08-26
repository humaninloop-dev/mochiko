---
name: converting-command-to-schema
description: Maintainer-side procedure for converting a shipped mochiko command's rule-shaped content into the model-interpreted `.md` + schema pair form (`plugins/mochiko/commands/<cmd>.md` + `plugins/mochiko/schemas/<cmd>.yaml`) that `/mochiko:implement` uses. Invoke on "convert <command> to schema" or when a per-command rollout ruling (command-content-schema D10) has landed. Repo-level tooling only — never shipped.
---

# Converting a Command to the Schema Pair

Repo-level maintainer tooling. **Never shipped** — this skill lives at `.claude/skills/` and
must never move under `plugins/`.

## Exemplars and law — read, never restate

- `plugins/mochiko/schemas/implement.yaml` — the exemplar schema; its header comment carries the grammar (D6 as amended by D14/D15).
- `plugins/mochiko/commands/implement.md` — the exemplar narrative scaffold.
- `plugins/mochiko/schemas/command-labels.yaml` — the shared ten-label registry (D8).
- `scripts/check-command-schema.py` — the advisory checker (D13).
- `.mochiko/brainstorms/command-content-schema/record.md` — decisions D1–D15; cite by number.
- `.claude/rules/mochiko/primitive-edits.md` + `.mochiko/strips/README.md` — the ceremony.

## Preconditions

1. **A recorded per-command ruling.** Rollout is per-command by its own ruling (D10) — never
   assumed from the implement precedent. No ruling, no conversion.
2. **A durable step-0 referent.** The approved command text lands as a session artifact in the
   session directory before any extraction — the build's source text and the fidelity audit's
   referent.

## Procedure

1. **Partition** (D2/D7). Narrative — Identity & Mission, protocol prose — stays in the `.md`.
   Rule-shaped content moves to `plugins/mochiko/schemas/<command>.yaml`: roles/seat wiring,
   reserved-to-user items, tools bindings, ways of working, the boundaries floor, FAIL clauses.
2. **Mint sections** (D14). `<cmd>.sec.<slug>` nodes, `title` verbatim from the charter group,
   `intent` one navigation line. Sections never grow a second prose surface — narrative stays
   in the `.md`.
3. **Extract rules at D12 grain** — one independently-citable obligation per block. Mint
   dotted-slug IDs `<cmd>.<kebab-name>` under a short stable prefix; FAIL clauses under
   `<cmd>.fail.*`. IDs mint once, frozen (D11): a reword keeps its ID, a split mints children
   recording the parent, a merge tombstones the losers. A slug is a name, never a summary.
   Assign `class` (`floor` = non-waivable, must survive the audit; `must` = binding;
   `advisory` = may change without supersession ceremony).
4. **Referential closure** (D15). No deixis — every reference in a rule's text resolves
   in-block or via the addressable namespace: `${var}` names, rule IDs, section IDs, `class:`
   values, registry labels, `pointer:` skills, `ruling:` anchors, literal file paths. "this
   schema" and "the run" are legal self-reference.
5. **`vars:` block** (D5). Injectable values (seat names, paths, bounds) declare once in
   `vars:` and substitute via `${var}` in rule text. Never `{{...}}` — that sigil is the
   template-skeleton convention, opposite semantics.
6. **Labels from the registry only** (D8). Every `labels:` value comes from
   `command-labels.yaml`. A needed new label is a registry amendment by ruling, first, under
   the normal ceremony — never an ad-hoc addition.
7. **Anchor protected content.** `ruling:` anchors ("YYYY-MM-DD <session-slug> [D#]",
   resolving against a live `DECISIONS.md` row) on protected content; `pointer:` on
   skill-owned floors — the rule holds the pointer, never the procedure.
8. **Author the thin `.md`** (D7). Frontmatter and Identity & Mission; an obligated
   first-action raw Read of the schema, naming its section IDs; the protocol narrative; a
   Not-done line pinning the fail-condition count in the literal phrase form the checker greps
   — "the N rules labeled `fail-condition`" — plus the out-of-sync halt clause (count mismatch
   = halt and surface before closing).
9. **Run the checker** (D13) with explicit flags for the new pair:
   `uv run scripts/check-command-schema.py --schema plugins/mochiko/schemas/<cmd>.yaml --md plugins/mochiko/commands/<cmd>.md`
   (`--labels`/`--decisions` default correctly). Expect PASS, 0 findings; cite its output in
   the audit brief as the deterministic pre-pass.
10. **Strips** for every superseded `.md` passage, per M4 (record.md build item 4): the
    strip's Content field carries the shipped text; the rewrite delta is recorded
    separately, never co-mingled. No unchecked
    hard-coded counts — the fail-condition count is the sole checker-guarded one. Contract:
    `.mochiko/strips/README.md`.
11. **Re-key `.claude/rules/mochiko/primitive-edits.md`** so the criteria governing `<cmd>` —
    the charter-form exception for a charter command, the default coherence block otherwise —
    grade `<cmd>.md` + `plugins/mochiko/schemas/<cmd>.yaml` as a pair: label-keyed FAIL survival,
    D11 ID continuity (rule + `.sec.` IDs), `class: floor` = must-survive; cite the command's
    own D10 ruling.
12. **Audits** (author≠grader; record.md build item 7), three: command-pair coherence ·
    schema fidelity against the step-0 referent · strip verification — each audit brief
    citing the checker output as the deterministic pre-pass.
13. **Land** through the six release gates (CLAUDE.md release-gates line, GI-012): audits
    PASS · strips recorded · landing complete · CHANGELOG entry · marketplace.json synced ·
    cargo test PASS. The converted command joins the D10 first-live-run watch in BACKLOG.md —
    extend the existing watch item, never mint a parallel one.

## Pitfalls — from the implement conversion

- Deixis surviving prose extraction ("these rules", "this section") — the referent was the
  surrounding document; atomization strands it (D15's first catch).
- Hard-coded block counts drifting — strike every count except the checker-guarded
  fail-condition one.
- `{{...}}` vs `${...}` sigil collision — skeleton blanks and var substitution are neighbors
  with opposite meanings.
- Forgetting the literal Not-done phrase — the checker greps
  "the N rules labeled `fail-condition`"; a paraphrase defeats the C2 count guard.
- Minting IDs that are summaries instead of names — wording drift then pressures a rename,
  which D11 forbids.
