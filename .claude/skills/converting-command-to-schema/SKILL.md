---
name: converting-command-to-schema
description: Maintainer-side procedure for converting a shipped mochiko command's rule-shaped content into the model-interpreted `.md` + schema pair form (`plugins/mochiko/commands/<cmd>.md` + `plugins/mochiko/schemas/<cmd>.yaml`) that every shipped mochiko command uses. Invoke on "convert <command> to schema" or when a per-command rollout ruling (command-content-schema D10) has landed. Repo-level tooling only — never shipped.
---

# Converting a Command to the Schema Pair

Repo-level maintainer tooling. **Never shipped** — this skill lives at `.claude/skills/` and
must never move under `plugins/`.

## Exemplars and law — read, never restate

- `plugins/mochiko/schemas/implement.yaml` — the exemplar schema; it opens with the runtime-kernel
  header (schema-header-runtime-kernel ADR, 2026-08-28, amending D14). The grammar itself (D6 as
  amended by D14/D15 and by command-schema-ontology D1–D8) is single-sourced in the command `.md`
  Rules block (runtime) and `.claude/rules/mochiko/primitive-edits.md` criterion 11 (edit-time).
- `plugins/mochiko/commands/implement.md` — the exemplar `.md`, on the canonical scaffold. Every
  command file has this same shape; there is no second form to choose between.
- `plugins/mochiko/schemas/command-labels.yaml` — the shared label registry (D8).
- `plugins/mochiko/schemas/common.yaml` — the shared rule library an `extends:` stub binds; its
  header kernel carries the resolution rule; the extraction bar lives in the near-dup convergence
  ADR (`2026-08-28-near-dup-convergence.md`, D8 as amended).
- `scripts/check-command-schema.py` — the advisory checker (D13), which lints the scaffold too.
- `.mochiko/brainstorms/command-content-schema/record.md` — decisions D1–D16; cite by number.
- `.mochiko/brainstorms/command-schema-ontology/record.md` — the node ontology: decisions D1–D11
  as amended (`kind:` · `conditions:`/`when:` · `moments:` · `enforces:` · `extends:`); cite by
  number. Its `conversion-inventory.md` is the worked corpus — section A.0 for the kind
  discriminators, G for the pre-kernel header comment (historical; no longer the mint source —
  see step 2), J for the anomalies a converter will meet.
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
2. **Mint the header comment and the six sections** (D14 as amended by the
   schema-header-runtime-kernel ADR, 2026-08-28; scaffold D4/D5). The schema opens with the
   canonical **runtime-kernel** header comment — one block, `<cmd>` substituted, carrying only
   what a runtime reader needs: identity, the read-at-fire note pointing at the `.md` Rules
   block for the reading grammar, `class:` semantics, and the `conditions:` resolution-point
   vocabulary. Copy it verbatim from a shipped schema; never write a per-command variant, and
   never restate the full grammar in the header. Legal-self-reference vocabulary (the desks'
   `"the desk"`/`"the visit"` vs the runs' `"the run"`) is carried by the D13 checker's curated
   marker list and the ontology record, not by the header.

   Then every schema carries the same six nodes, in this order: `<cmd>.sec.roles` ·
   `<cmd>.sec.reserved` · `<cmd>.sec.tools` · `<cmd>.sec.ways-of-working` ·
   `<cmd>.sec.boundaries` · `<cmd>.sec.fail-conditions`. There is
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

   Assign both discriminators. `class` carries bindingness (`floor` = non-waivable, must survive
   the audit; `must` = binding; `advisory` = may change without supersession ceremony). `kind`
   carries what the rule *is*, from the closed set of nine (ontology D1): `constraint` · `duty` ·
   `gate` · `reservation` · `binding` · `bound` · `routing` · `fail` · `latitude`. **`constraint`
   is the omitted default** — a constraint rule carries no `kind:` field at all, and an absent
   `kind:` reads `constraint`. Kind crosscuts the six sections; it never re-groups them, and
   `class` still carries bindingness while `labels` still carry topics. The per-kind test is
   tabled in the inventory's section A.0; three of them decide most miscalls:
   - a **`gate`** is whatever *blocks* — not whatever is calendared, and not always the user's
     (one of the ten corpus gates is held by a grader);
   - a **`reservation`** says who decides and never when the run stops;
   - a **`duty`** is an action *the lead* always performs — a seat's obligation is a constraint.

   Kind is not derivable from the ID: a rule can be a gate with no `gate-` prefix, or name a
   moment and still be a reservation. In particular, never default `kind:` on a `<cmd>.fail.*`
   ID — write `kind: fail` on every fail node.
4. **Referential closure** (D15). No deixis — every reference in a rule's text resolves
   in-block or via the addressable namespace: `${var}` names, rule IDs, section IDs, `common.*`
   block IDs, `class:` and `kind:` values, `conditions:`/`moments:` names, registry labels,
   `pointer:` skills, literal file paths. "this schema" and "the run" are legal self-reference.
   A `<cmd>.<slug>` citation inside rule text is the ratified dependency idiom (ontology D5) and
   must resolve to a live node or a tombstone: the parenthetical form is the ratified shape, but
   the bare inline form and section-ID citations are checked the same way, so any of the three is
   a real reference you must keep resolving.
5. **`vars:` block** (D5). Injectable values (seat names, paths, bounds) declare once in
   `vars:` and substitute via `${var}` in rule text. Never `{{...}}` — that sigil is the
   template-skeleton convention, opposite semantics.
6. **Declare the run-shape grammar** (ontology D3/D4) — two top-level blocks, siblings of
   `vars:`, plus the rule field that references them.
   - **`conditions:`** — every run-shape branch dimension this schema's own rules use: the
     name, its closed value set (or `presence` for a surface-existence flag), and its
     **resolution point**, one of `entry-derived` · `surface-presence` ·
     `moment-resolved(<moment>)` · `user-ruled` · `standing-trigger`. A `moment-resolved`
     dimension resolves at the named moment, and rules gated on it are inapplicable until it
     does — no ordering is claimed or checkable. Two literal keys carry the declaration and
     the checker resolves against both: **`values:`** — the closed list, or the bare word
     `presence`, whose poles are then `present` / `absent` — and **`resolution:`**. Any other
     key on the declaration (the shipped `note:` line) is free prose the checker ignores.
     Copy the block's shape from a shipped schema, as with the header (step 2).
   - **`moments:`** — the named anchor points this schema's rules reference, one navigation line
     each. The list is **unordered**: relative sequence stays the `.md` protocol's narrative and
     the lead's latitude. An ordered list is the workflow-engine direction and was declined
     deliberately. There is no `at:` field — it was deferred to graduation (D4 as amended), so a
     duty anchors to its moment in prose.
   - **`when:`** on a rule — a conjunction of `dimension: value` terms drawn from this schema's
     own `conditions:` block. Declared vocabulary only: no boolean algebra, no negation beyond a
     declared value, no free-form strings. It carries **whether the rule binds, never the rule's
     internal logic.**

   Declare only what some rule uses: an unused dimension or moment is a checker finding.

   **Single-homing has two dispositions** (D3 as amended, inventory J-1). **MOVE** is the
   default — the rule-level activation guard leaves `text` and lands in `when:`; a reword that
   keeps its ID, owing a strip entry with the verbatim removed clause. **DECLARE** covers the
   rule whose condition rides its **subject noun** rather than a detachable guard clause, where
   extraction would strand a referent and breach D15: add `when:`, leave `text` untouched, and
   no strip is owed — a pure addition rides the decision row. Mid-sentence exceptions,
   carve-outs, and multi-arm obligations are neither disposition: they stay prose.

   **Floors are never shed** (D3 as amended, C4). A `class: floor` rule is always read and
   always delivered whatever its `when:` — the condition gates when the obligation *applies*,
   never whether it is delivered, and the checker makes no coverage claim over floors. Any shed
   set of non-floor rules is re-evaluated whenever the run's shape changes: a seat added mid-run
   re-activates every `when:`-gated rule its addition touches. Carry this comment verbatim onto
   each floor that gains a `when:`:

   ```yaml
        # C4: floor — always read, always delivered. `when:` gates when the
        # obligation APPLIES, never whether it is delivered. Re-evaluated
        # whenever the run's shape changes (a seat added mid-run re-activates it).
   ```
7. **`enforces:` on every fail node** (ontology D6). A `kind: fail` node carries `enforces:` —
   the list of local rule IDs naming the gate, floor, or duty it is the end-state contrapositive
   of. An empty list is legal **only** with a one-line reason, the weak-mirror case where the
   obligation lives in a `pointer:` skill; absence is then a statement, never an omission. That
   reason is not free prose: it lives in a YAML comment **directly above** the `enforces:` line
   and opens with the literal marker the checker greps — `# D6 empty-with-reason: <why>`, with
   text after the colon. Copy the form from a shipped site
   (`plugins/mochiko/schemas/setup.yaml` — `setup.fail.unclosed-trace` and
   `setup.fail.floor-category-uncovered`); a reason written any other way reads to the checker
   as no reason at all. Every listed ID must resolve, and a tombstoned target is an error. The
   reverse direction — floors and gates that no fail node enforces — is an advisory checker
   report and is **input to the deferred Desk FAIL-set widening pass, never audit pressure**:
   the pair audit's FAIL-survival handle covers the existing fail set only, and a fail node is
   never added silently.
8. **`extends:` only at the exact-duplicate bar** (ontology D8 as amended by C2/C3). A rule block
   may carry `extends: common.<slug>`, binding a block from `plugins/mochiko/schemas/common.yaml`.
   Per-command rules stay the **default**; extraction into `common.yaml` is legal **only** for
   text that is an exact duplicate across **three or more** command schemas — boilerplate by
   definition, never command-specific judgment. A near-miss does not qualify: a one-token
   difference is a no-bind, and rewording a shipped rule to make it bindable is content churn
   that needs its own ruling. A stub whose `text` would be a full local override buys nothing —
   write the per-command rule instead.

   **Precedence (C3):** a stub inherits `text`, `labels`, and `pointer`, and nothing else.
   `class:` and every absence-meaningful field — `kind:`, `when:`, `enforces:` — are **always
   local**, and a stub MUST declare `class:` explicitly so a floor's bindingness stays readable
   from its own file. The absence defaults apply after resolution. `${var}` in inherited text
   substitutes from the **binding** schema's `vars:`, never from `common.yaml`, and the stub's
   `<cmd>.*` ID stays the citable ID. Diff the inherited `labels:` against the stub's current
   ones — where they differ, declare them locally or the rule's labels change silently. Where a
   schema binds any block, its `.md` Rules section instructs a raw full Read of `common.yaml` in
   the same first action as its own schema.
9. **Labels from the registry only** (D8). Every `labels:` value comes from
   `command-labels.yaml`. A needed new label is a registry amendment by ruling, first, under
   the normal ceremony — never an ad-hoc addition. `fail-condition` was **retired** at v0.98.0
   (ontology D1, build item 4): `kind: fail` is the selector for the Not-done set, and a fail
   node carries no label in its place.
10. **Anchor protected content — in the sidecar, never inline** (D16). Schemas carry
   runtime content only: decision anchors ("YYYY-MM-DD <session-slug> [D#]", resolving
   against a live `DECISIONS.md` row) land in `.mochiko/provenance.yaml`, keyed by rule ID
   — repo-side, never delivered with the plugin. An inline `ruling:` field is a checker
   finding. `pointer:` stays inline on skill-owned floors — the rule holds the pointer,
   never the procedure.
11. **Author the `.md` on the canonical scaffold** (D7; scaffold D2). One shape, no variants —
   these headings, this order, nothing else at top level:
   1. **Frontmatter**: `description` · `disable-model-invocation: true` · `argument-hint`. All
      three are canonical; `argument-hint` is the native surface of the `$ARGUMENTS` contract.
   2. `# <Name> — <epithet>`.
   3. `## Identity & Mission` — who leads this surface and what it stewards. One tight section;
      identity prose never materially delays the Rules block.
   4. `## Rules — load the schema first` — the obligated first-action raw Read of the schema,
      enumerating all six of its section IDs; where the schema binds a common block, the same
      first action Reads `plugins/mochiko/schemas/common.yaml` raw and whole. This section also
      carries the one-line `when:` interpretation clause — the reader is told that a `when:`
      gates whether a rule binds and that a floor is delivered regardless.
   5. `## Adaptive Goal Protocol` — exactly three steps: **Entry** (`$ARGUMENTS` handling and
      gating — `$ARGUMENTS` has no other home) · **Goal** (the done condition: converged with
      the user per visit for a desk command, fixed for a run) · **Not done — default FAIL**,
      always last, pinning the fail count in the literal phrase form the checker greps
      — "the N rules of `kind: fail`" — plus the out-of-sync halt clause (count
      mismatch = halt and surface before closing).

   No `**Goal:**` opener line, no `Goal` / `Harness` / `Bindings` section, no per-command extra
   top-level section. The charter-form / goal-form split was superseded at v0.97.0 (scaffold
   D1): a converted command has no form to choose.
12. **Run the checker** (D13) with explicit flags for the new pair:
   `uv run scripts/check-command-schema.py --schema plugins/mochiko/schemas/<cmd>.yaml --md plugins/mochiko/commands/<cmd>.md`
   (`--labels`/`--decisions` default correctly). Expect PASS, 0 findings; cite its output in
   the audit brief as the deterministic pre-pass.
13. **Strips** for every superseded `.md` passage and for every clause that leaves a rule's
    `text`, per M4 (record.md build item 4): the
    strip's Content field carries the shipped text; the rewrite delta is recorded
    separately, never co-mingled. A `when:` MOVE owes a strip with the verbatim removed guard
    clause; a DECLARE owes none. No unchecked
    hard-coded counts — the `kind: fail` count is the sole checker-guarded one. Contract:
    `.mochiko/strips/README.md`.
14. **Check `.claude/rules/mochiko/primitive-edits.md` covers `<cmd>`.** There is one
    canonical-scaffold criteria block and it already grades every pair-form command — scaffold
    conformance, set-wise section enumeration, FAIL survival keyed to `kind: fail`, D11/D14 ID
    continuity (rule + `.sec.` IDs), `class: floor` = must-survive, the ontology-grammar
    conformance clause (kind vocabulary · `conditions:`/`moments:` declared and resolved ·
    `when:` single-homed · floors delivered · `enforces:` resolving · `extends:` stubs carrying
    a local `class:`), the substance legs, and the
    done-condition branch. Do **not** mint a second block. The only edit a new command needs is
    naming it in the branch that fits it — desk (per-visit converged done condition) or run
    (fixed done condition) — plus any command-specific gate the ruling adds, as `implement`
    carries. Cite the command's own D10 rollout ruling.
15. **Audits** (author≠grader; record.md build item 7), three: command-pair coherence ·
    schema fidelity against the step-0 referent · strip verification — each audit brief
    citing the checker output as the deterministic pre-pass.
16. **Land** through the six release gates (CLAUDE.md release-gates line, GI-012): audits
    PASS · strips recorded · landing complete · CHANGELOG entry · marketplace.json synced ·
    cargo test PASS. The converted command joins the D10 first-live-run watch in BACKLOG.md —
    extend the existing watch item, never mint a parallel one.

## Pitfalls — from the implement conversion and the ontology wave

- Deixis surviving prose extraction ("these rules", "this section") — the referent was the
  surrounding document; atomization strands it (D15's first catch).
- Hard-coded block counts drifting — strike every count except the checker-guarded
  `kind: fail` one.
- `{{...}}` vs `${...}` sigil collision — skeleton blanks and var substitution are neighbors
  with opposite meanings.
- Forgetting the literal Not-done phrase — the checker greps
  "the N rules of `kind: fail`"; a paraphrase defeats the C2 count guard.
- Minting IDs that are summaries instead of names — wording drift then pressures a rename,
  which D11 forbids.
- Letting `kind:` default on a `<cmd>.fail.*` ID — the segment and the field are cross-checked
  in both directions, and a defaulted fail node breaks the Not-done count on the new key.
- Writing an `extends:` stub with no local `class:` — the class then reads only after resolving
  a second file, which is the single-file-readability exposure C3 exists to close. The
  uncommitted prototype shipped this shape; do not copy it.
- Reading a `when:` on a floor as permission to drop the floor — a floor is always delivered;
  `when:` gates only when its obligation applies.
- Extracting a near-duplicate into `common.yaml` — the bar is *exact* duplication across three
  or more schemas, and two of the corpus's near-misses differ by a single token.
