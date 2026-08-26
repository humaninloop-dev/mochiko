# Command-content schema — decision record

**Topic:** YAML schema for command content — each section's content tagged with stable IDs,
higher-altitude clusters as metadata, and variable injection inside content blocks (e.g. the
design-seats staffing rule with injectable agent/path values). Start with `implement.md`, roll
out to the other commands after. Shape kin: the eval `rules.json` inventories, in YAML like the
shipped template schemas.

**Status:** open
**Opened:** 2026-08-26
**Lead:** session lead (brainstorm charter)

---

## Ground facts

- **F1 — substrate state.** `commands/implement.md` is charter-form (six sections) at v0.91.0,
  ~430 lines. A full-file plain-language rewrite (~330 lines) was drafted and approved
  section-by-section in this same conversation but is **not landed** — no file write, no strips,
  no audit yet. This session's schema idea arrived on top of that unlanded draft.
- **F2 — template-schema precedent.** Nine schema data files ship at
  `plugins/mochiko/schemas/*.yaml` for pipeline **artifact templates** (D8,
  `schema-based-template-guidance`): data files are the source of truth, `mochiko-cli` renders,
  raw Read is the first-class fallback (GI-020). Shape: header metadata + `sections:` each
  `{name, required, contract, check}`.
- **F3 — rules.json precedent.** `evals/*/rules.json` (5 skills, from the compression-eval
  infrastructure): flat rule inventory `{id: R-XXX, rule: statement + "Evidence:" clause,
  class: floor|must|format, source: file:section anchors}`. Built per-eval, not shipped, no
  variables, no clusters.
- **F4 — constraint envelope.** GI-019: kernel-class tooling only by recorded ruling; the
  standing bright line — never gates pipeline progress, never dispatches or sequences agents,
  never holds judgment skills own. D11 admitted the template-schema CLI **for artifact
  templates specifically**; extending the renderer to command content is a new admission
  needing its own recorded ruling. GI-020: plugin install stays markdown-only — commands must
  ship as functioning `.md` whatever the schema does.
- **F5 — audit keying today.** The charter audit (`primitive-edits.md`) grades implement.md
  against prose criteria (floor present + goal contract present, every FAIL clause surviving);
  strips quote content verbatim. Neither is ID-addressable today.

## Decisions

### D1 — YAML is the source of truth; the model interprets it at runtime — `Confident`

**Statement:** the schema data file (`implement.yaml` shape) is the source of truth for the
content it carries. The command `.md` loads it and the **model interprets it live** at command
fire — "render" means runtime interpretation by the agent, not a build-time generation step.
No binary is required on the read path (raw Read stays first-class, GI-020 satisfied);
`mochiko-cli` rendering can be added later as an optional human-facing view.

**Rationale:** the user's stated intent at Q1 clarification — a thin scaffold around the
schema with the ability to effectively interpret. Matches the D8 template precedent's
data-as-source-of-truth posture while dropping its build-time render step.

### D2 — Stage 1 split: rules move to the schema, narrative stays prose; absorption trigger on record — `Confident`

**Statement:** stage 1 moves the **rule-like content** of `implement.md` into the schema
(ID-tagged blocks); the charter narrative (Identity & Mission, protocol prose voice) stays in
the `.md`. An **absorption trigger** is on record: after the first live `/mochiko:implement`
run under the schema, if the lead followed the schema-carried rules with no miss attributable
to YAML carriage, stage 2 — narrative absorbs into the schema, `.md` thins to scaffold — is
pre-authorized and lands as an ordinary build citing this record. Contrary evidence → the
trigger never fires and the split stands.

**Rationale:** the lead recommended whole-move (A) on drift/boundary/injection-reach/audit
grounds; the user weighed the steering risk — a YAML-carried mission is unproven (n=0) — and
ruled the staged middle path the lead offered: B's graceful degradation without B becoming
permanent. House idiom: `not-now + trigger` stance rows, sound-loop rules-file first-miss
deferral.

### D3 — No shared rule library: per-command rules + a common label vocabulary — `Contested`

**Statement:** rule definitions live inline in each command's schema — no shared
`charter-rules.yaml`, no library-of-rules with binding files. The cross-command connective
tissue is a **common label vocabulary**: one controlled set of labels (the "altitude clusters")
applied to rule blocks across all command schemas; the same label on rules in different
commands constitutes the link. Duplication between commands is accepted and made *visible and
addressable* rather than extracted away. The skill fence holds regardless: where a skill owns
a floor (`patterns-sound-loop`, `patterns-transport-floor`, …), the rule block carries the
pointer, never the procedure.

**Rationale:** the lead recommended a shared library + per-command bindings (single-sources
the real charter-boilerplate duplication); the user ruled against extraction — the want is a
common vocab that creates command links, "sort of like label", keeping every command readable
from its own file.

### D4 — Label job: navigation now, edit-time drift check as the goal — `Confident`

**Statement:** stage 1 ships labels as **query/navigation only** — cross-command views by
label, no ceremony obligation. The goal state is the **edit-time drift check** (editing a
labeled rule surfaces same-labeled rules in other command schemas; the editor states aligned
or diverged-on-purpose, folded into the primitive-edit ceremony). Graduation is
confidence-keyed: an effective B builds the case for A — concretely, when the vocabulary has
survived the implement build plus the first rollout command without label churn, the ceremony
hook lands citing this record.

**Rationale:** user's ruling — "B for now, but then goal is to be A … an effective B gives
confidence for A." Same staged-trigger idiom as D2.

### D5 — Variables: schema-local `vars:` block + `{{placeholder}}` substitution at read — `Confident`

**Statement:** each command schema carries a `vars:` block (seat names, paths, bounds); rule
text carries `{{placeholders}}`; the model substitutes at read time. One place to change a
value; text stays readable as text. Runtime repo-state resolution declined for now; a value
class can graduate later by its own ruling.

**Rationale:** the user's original ask, literally; ruled as recommended.

### D6 — Rule-block grammar adopted — `Confident`

**Statement:** a rule block is `{id, labels: [...], class: floor|must|advisory, text,
ruling?: <DECISIONS.md anchor>, pointer?: <skill>}` under a top-level `vars:` block. ID format
`IMPL-R-XXX` (per-command prefix). `ruling:` machine-tags protected content — the audit's
preserved-responsibilities check and the strip ceremony gain an addressable anchor. `pointer:`
carries the skill fence: where a skill owns the floor, the rule holds the pointer, never the
procedure. `class` reuses the eval taxonomy shape (`floor|must|format` → `floor|must|advisory`).

**Rationale:** adopted as drafted from the preview; ceremony wiring from day one preferred
over minimal-then-grow.

### D7 — Stage-1 scope: all rule-shaped content moves, FAIL list included; source = the simplified rewrite — `Confident`

**Statement:** `implement.yaml` rules = R&R seat wiring + reserved-to-user items + Ways of
Working + Boundaries + Tools bindings + the FAIL clauses (labeled `fail-condition`; the `.md`
protocol's Not-done line becomes "every rule labeled `fail-condition`"). Narrative staying in
`implement.md` = Identity & Mission + Adaptive Goal Protocol prose. Source text is the
**simplified rewrite approved earlier this session** (never the shipped v0.91.0 wording);
rewrite + schema land as **one build wave**. The charter audit criterion "every prior FAIL
clause surviving" re-keys to the `fail-condition` label set.

**Rationale:** FAIL clauses are the most rule-shaped content in the file; the audit re-key is
owed by the build wave anyway. Ruled as recommended.

### D8 — Label vocabulary: controlled registry file + ten-label seed — `Confident`

**Statement:** the vocabulary ships as `plugins/mochiko/schemas/command-labels.yaml` — one
line of meaning per label; every rule's `labels:` values must come from it; new labels enter
by registry edit first (normal shipped-primitive ceremony). Seed set (10): `independence` ·
`user-gate` · `fail-condition` · `attempt-economy` · `landing` · `evidence` · `scope-entry` ·
`seats` · `floor-pointer` · `reporting`.

**Rationale:** rules stay per-command (D3), but the vocab is the one deliberately shared
surface — a registry under ceremony is what keeps it common instead of drifting per command.
Ruled as recommended, full seed.

### D9 — Governance envelope: no new kernel admission; schemas are shipped primitives; audit re-keys to the pair — `Confident`

**Statement:** model-interpreted command schemas need **no new kernel-class admission** — the
schema is data, the interpreter is the model; nothing executable gates pipeline progress or
dispatches agents, so GI-019 is untouched. A future `mochiko-cli` render/`--check` over
command schemas would extend the admitted CLI and takes its own ruling note at that time.
`implement.yaml` and `command-labels.yaml` are shipped primitives under the full strip +
author≠grader ceremony (the v0.76.0 schema precedent). The charter audit re-keys to grade the
**`.md` + schema pair** — floor present + goal contract present across both surfaces, the
FAIL-clause-survival criterion keyed to the `fail-condition` label set — via a
`primitive-edits.md` edit riding the build wave. GI-020 holds: install ships markdown + data
files, nothing heavier.

**Rationale:** proposed whole by the lead; adopted as stated.

### D10 — Rollout by per-command ruling; first-live-run watch is the shared trigger evidence — `Confident`

**Statement:** implement converts first (this build). Each further command (`feature.md`,
`architecture.md`, the v8 trio) converts **by its own ruling** — door-open idiom, as the
charter ADR did. Evidence honesty: n=0 for model-interpreted schema delivery; a
**first-live-run watch** on schema-carried rule delivery is owed in `BACKLOG.md`, and its
outcome is the trigger evidence for **both** D2 (narrative absorption) and D4 (label ceremony
graduation).

**Rationale:** proposed whole by the lead; adopted as stated.

## Session trail

- **Q1 — source of truth** (structured fork A/B/C): user rejected the framing and clarified —
  thin `.md` scaffold around `implement.yaml`, model interprets at runtime. → D1.
- **Q2 — how thin** (whole charter moves vs rules-only vs overlay; lead recommended
  whole-move): user picked rules-only, had second thoughts, asked for the A-vs-B case; lead
  gave drift/boundary/reach/audit for A, steering-risk + ratchet steelman for B, offered the
  staged middle path; user asked what an absorption trigger is; ruled **B with absorption
  trigger**. → D2.
- **Q3 — reuse shape** (shared library recommended / per-command inline / library-of-bindings):
  user ruled against extraction — per-command rules + a common label vocab as the command
  link. → D3 (`Contested`).
- **Q4 — label job** (drift-check ceremony recommended / navigation-only / staged): user ruled
  "B for now, goal is A; effective B gives confidence for A". → D4.
- **Q5 — variables** (vars block + placeholders recommended / typed fields / runtime
  resolution): ruled as recommended. → D5.
- **Q6 — rule-block grammar** (adopt drafted grammar / trim / adjust; preview shown): adopted
  as drafted. → D6.
- **Q7 — stage-1 scope** (all rule-shaped content incl. FAIL list recommended / FAIL stays /
  adjust; simplified-rewrite premise stated): ruled as recommended. → D7.
- **Q8 — vocab home** (registry + ten-label seed recommended / trimmed seed / no registry):
  adopted with full seed. → D8.
- **Q9/Q10 — wrap batch** (governance envelope + rollout/probes proposed whole): adopted as
  stated. → D9, D10.

## Build surface (cold-buildable)

One wave, landing the earlier-approved simplified rewrite and the schema together:

1. `plugins/mochiko/schemas/command-labels.yaml` — the D8 registry, ten-label seed.
2. `plugins/mochiko/schemas/implement.yaml` — `vars:` block + rules per D6 grammar; content =
   the rule-shaped inventory of D7, text from the simplified rewrite; FAIL clauses labeled
   `fail-condition`; skill-owned floors as `pointer:` rules; protected lines carrying
   `ruling:` anchors.
3. `commands/implement.md` — simplified narrative (Identity & Mission + Adaptive Goal
   Protocol prose), a load-and-follow instruction naming the schema, the Not-done line
   re-keyed to "every rule labeled `fail-condition`".
4. Strips: the rewrite's deletion/relocation ledger (rationale cuts, R&R-restatement cuts,
   deviation-grammar single-homing) **plus** supersession entries for every block moving from
   `.md` to schema.
5. `.claude/rules/mochiko/primitive-edits.md` — the D9 audit re-key (pair grading;
   label-keyed FAIL criterion).
6. Audits: author≠grader validators — command-pair coherence + schema fidelity against the
   approved rewrite text + strip verification.
7. Gates 4/5/6: `CHANGELOG.md` · `marketplace.json` sync · `cargo test` (binary untouched;
   command schemas are explicitly outside `mochiko-cli`'s template set per D9).
8. `BACKLOG.md`: first-live-run watch (D10) — the shared trigger evidence for D2 and D4.

## Open questions

- Two schema grammars now coexist under `plugins/mochiko/schemas/` — template schemas
  (`template:` + `sections:`) and command schemas (`vars:` + `rules:`). Distinguishing key is
  the top-level shape; whether a `kind:` discriminator field is worth adding is left to the
  build wave's judgment.
