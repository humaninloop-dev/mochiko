---
name: converting-command-to-schema
description: Maintainer-side procedure for converting a shipped mochiko command's rule-shaped content into the model-interpreted `.md` + schema pair form (`plugins/mochiko/commands/<cmd>.md` + `plugins/mochiko/schemas/<cmd>.yaml`) that every shipped mochiko command uses. Invoke on "convert <command> to schema" or when a per-command rollout ruling (command-content-schema D10) has landed. Repo-level tooling only — never shipped.
---

# Converting a Command to the Schema Pair

Repo-level maintainer tooling. **Never shipped** — this skill lives at `.claude/skills/` and
must never move under `plugins/`.

## Exemplars and law — read, never restate

- `plugins/mochiko/schemas/implement.yaml` — the exemplar schema; its header comment carries the grammar (D6 as amended by D14/D15).
- `plugins/mochiko/commands/implement.md` — the exemplar `.md`, on the canonical scaffold. Every
  command file has this same shape; there is no second form to choose between.
- `plugins/mochiko/schemas/command-labels.yaml` — the shared label registry (D8).
- `scripts/check-command-schema.py` — the advisory checker (D13), which lints the scaffold too.
- `.mochiko/brainstorms/command-content-schema/record.md` — decisions D1–D16; cite by number.
- `.mochiko/brainstorms/command-md-scaffold-standardization/record.md` — the canonical `.md`
  scaffold and the unified six-section vocabulary, decisions D1–D7; cite by number.
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
2. **Mint the six sections** (D14; scaffold D4/D5). Every schema carries the same six nodes, in
   this order: `<cmd>.sec.roles` · `<cmd>.sec.reserved` · `<cmd>.sec.tools` ·
   `<cmd>.sec.ways-of-working` · `<cmd>.sec.boundaries` · `<cmd>.sec.fail-conditions`. There is
   no per-command section set and no smaller subset — a section holding no rules is **present
   and explicitly empty**, never omitted; copy the empty-marker form from a shipped schema, which
   the checker asserts. `title` verbatim from the group it names, `intent` one navigation line.
   Sections never grow a second prose surface — narrative stays in the `.md`.
3. **Extract rules at D12 grain** — one independently-citable obligation per block. Mint
   dotted-slug IDs `<cmd>.<kebab-name>` under a short stable prefix; FAIL clauses under
   `<cmd>.fail.*`. **Prefix derivation (scaffold D4):** a new command's prefix is its command
   filename stem, abbreviated only when a recorded collision forces it, the choice noted in the
   schema header; the six existing prefixes are frozen — never re-prefix a shipped schema.
   IDs mint once, frozen (D11): a reword keeps its ID, a split mints children
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
7. **Anchor protected content — in the sidecar, never inline** (D16). Schemas carry
   runtime content only: decision anchors ("YYYY-MM-DD <session-slug> [D#]", resolving
   against a live `DECISIONS.md` row) land in `.mochiko/provenance.yaml`, keyed by rule ID
   — repo-side, never delivered with the plugin. An inline `ruling:` field is a checker
   finding. `pointer:` stays inline on skill-owned floors — the rule holds the pointer,
   never the procedure.
8. **Author the `.md` on the canonical scaffold** (D7; scaffold D2). One shape, no variants —
   these headings, this order, nothing else at top level:
   1. **Frontmatter**: `description` · `disable-model-invocation: true` · `argument-hint`. All
      three are canonical; `argument-hint` is the native surface of the `$ARGUMENTS` contract.
   2. `# <Name> — <epithet>`.
   3. `## Identity & Mission` — who leads this surface and what it stewards. One tight section;
      identity prose never materially delays the Rules block.
   4. `## Rules — load the schema first` — the obligated first-action raw Read of the schema,
      enumerating all six of its section IDs.
   5. `## Adaptive Goal Protocol` — exactly three steps: **Entry** (`$ARGUMENTS` handling and
      gating — `$ARGUMENTS` has no other home) · **Goal** (the done condition: converged with
      the user per visit for a desk command, fixed for a run) · **Not done — default FAIL**,
      always last, pinning the fail-condition count in the literal phrase form the checker greps
      — "the N rules labeled `fail-condition`" — plus the out-of-sync halt clause (count
      mismatch = halt and surface before closing).

   No `**Goal:**` opener line, no `Goal` / `Harness` / `Bindings` section, no per-command extra
   top-level section. The charter-form / goal-form split was superseded at v0.97.0 (scaffold
   D1): a converted command has no form to choose.
9. **Run the checker** (D13) with explicit flags for the new pair:
   `uv run scripts/check-command-schema.py --schema plugins/mochiko/schemas/<cmd>.yaml --md plugins/mochiko/commands/<cmd>.md`
   (`--labels`/`--decisions` default correctly). Expect PASS, 0 findings; cite its output in
   the audit brief as the deterministic pre-pass.
10. **Strips** for every superseded `.md` passage, per M4 (record.md build item 4): the
    strip's Content field carries the shipped text; the rewrite delta is recorded
    separately, never co-mingled. No unchecked
    hard-coded counts — the fail-condition count is the sole checker-guarded one. Contract:
    `.mochiko/strips/README.md`.
11. **Check `.claude/rules/mochiko/primitive-edits.md` covers `<cmd>`.** There is one
    canonical-scaffold criteria block and it already grades every pair-form command — scaffold
    conformance, set-wise section enumeration, label-keyed FAIL survival, D11/D14 ID continuity
    (rule + `.sec.` IDs), `class: floor` = must-survive, the substance legs, and the
    done-condition branch. Do **not** mint a second block. The only edit a new command needs is
    naming it in the branch that fits it — desk (per-visit converged done condition) or run
    (fixed done condition) — plus any command-specific gate the ruling adds, as `implement`
    carries. Cite the command's own D10 rollout ruling.
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
