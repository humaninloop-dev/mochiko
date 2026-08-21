# Skill-Compression Tooling + Before/After Degradation Eval — Decision Record

**Status:** accepted 2026-08-22 — solo cold review returned critical-gaps; all 23 findings
dispositioned same day (user: "as recommended, re-affirm D4–D8, take your three calls too");
folds R1–R16 applied; user accepted and the landing ritual ran same day (DECISIONS.md row ·
BACKLOG build section · ROADMAP Token-epic row touch · index)
**Opened:** 2026-08-22
**Session:** collaborative brainstorm via `mochiko:analysis-iterative` (one question per turn)

## Topic

Design a **repo-level skill** (`.claude/skills/`, never shipped in `plugins/mochiko/`) whose job
is to reduce the size of mochiko's plugin skills while keeping their structure intact — and a
**before/after evaluation** that runs the same mock task against the pre- and post-compression
skill in independent sessions, scored by an LLM judge, to measure whether compression degraded
behavior.

Threads:

1. What "compress while keeping structure the same" means operationally — denser prose at equal
   information vs. removing content classes under the existing cut line.
2. What the compressor skill owns (the cut judgment, the accounting, the ratification gate) and
   what it delegates to already-ruled doctrine.
3. The before/after protocol: mock task selection, session isolation, judge design, replicate
   count, and the noise guard that a prior mochiko benchmark had to invoke.
4. Where this sits relative to three accepted/open prior sessions so nothing is re-litigated.

## Ground facts

*(lead sweep 2026-08-22, read-only; two corrections applied post-review — F1 rewritten (R1),
F8 corrected (R16) — remainder citation-verified by the cold reviewer and holding)*

- **F1 — the compression doctrine is ruled AND already executed once; the pilots are
  twice-compressed.** *(Corrected post-review, R1 — the original claim "no wave was ever run"
  was false, inherited from a stale index entry since fixed on sight, M8.)*
  `skill-succinctness-strip` (accepted 2026-07-25) ruled the doctrine — scope = bodies +
  measure-first descriptions + references (scripts/yaml/`templates/` out), per-skill user
  ratification, true-reductions-only accounting (sham cuts forbidden, `templates/` banned as a
  relocation destination) — and its waves **executed at v0.24.0–v0.28.0: 27 skills, −23.7%**
  (DECISIONS.md row 2026-07-25; backlog trail). The v0.64.0 guardrails wave cut many skills a
  second time (per-skill strips stamps). Its review also found the "descriptions load fully"
  premise false — delivery truncates ~1.8k-char descriptions mid-sentence. Consequence priced:
  Arm A headroom on the pilots is a fraction of a never-stripped skill's; the deliverable's
  durable value is the eval instrument, not the byte harvest (R9).
- **F2 — the calibration bands were method-superseded.** `validator-scope-and-verbosity` D8 (as
  amended): `skill-succinctness-strip`'s calibration bands are dead for future passes; what
  survives is **measure-first** and **true-reductions-only**. The governing law became
  **measure-then-gate** (D1): benchmark first, cost gate second.
- **F3 — a cut line already exists.** `validator-scope-and-verbosity` D4: keep goal + output
  contract / non-waivable floor / anti-patterns and rejections / hard reference-data; drop
  procedure / examples / restatement. *("output contract" restored 2026-08-22 — the original
  paraphrase dropped it; caught by the build audit, B1.)* Edge-case playbooks
  were the benchmark's cargo.
- **F4 — the benchmark method already exists, and so does its failure mode.**
  `validator-scope-and-verbosity` D2/D3/D5/D6: substrate = full setup+specify skill clusters
  (11 skills), stage-run unit; three arms (body-guardrails · slim-descriptions + invocation-fire-
  rate · agent-descriptions-sans-examples + route check); model-played principal from a frozen
  persona card with planted vagueness; neutral outcome rubric, LLM-as-judge, numeric scores,
  existing checklists demoted to a secondary floor check; decision rule 10% threshold,
  floors absolute, 4 variants × 2 commands × ≥2 replicates = 16+ runs, diagnostic trace-back on
  loss. D7: cost gate = empirical budgets (winner +25%), char-count pre-assert in the existing
  audit, **chars never `wc -c` bytes**.
- **F5 — that benchmark's one real run hit noise.** On one of three arms, replicate spread
  (5.6/7.1) exceeded the effect gap (0.8/1.8); the noise guard fired and the verdict had to be
  user-ruled. Reusable scaffolding survives: `RUN-PROTOCOL.md`, rubrics, judge scorecards,
  persona-card fixture (curated set at `.mochiko/benchmarks/guardrails-vs-detail/`, 464 KB; full
  trail unmerged on branch `worktree-brainstorm-validator-scope`).
- **F6 — the eval harness is designed but unbuilt, and its pilot pick is the open slot.**
  `primitive-eval-harness` (open, paused 2026-08-19) D1–D5: goldens synthetic per-skill in the
  `skill-creator` format (`evals/evals.json`: id · prompt · expected_output · assertions);
  deterministic checks may block, the LLM judge is advisory and never blocking; local-first, CI
  deferred; thin scripts (~200 lines) under a top-level never-shipped `evals/` dir driving
  `claude -p --bare --plugin-dir plugins/mochiko --output-format json --permission-mode dontAsk
  --max-turns N`, promoting into the Rust crate only once proven; first target = one pilot skill
  end-to-end. **Pilot skill choice is `Deferred`** — the open slot this session's before/after
  run would fill.
- **F7 — the platform supplies the runner and a free deterministic gate.** `claude -p --bare
  --plugin-dir <path>` yields an isolated session loading exactly the artifacts under test; the
  `system/init` event carries `plugin_errors` (docs: fail CI on non-empty); the JSON result
  self-reports `total_cost_usd`. *(External claims live-verified by the cold reviewer against
  code.claude.com docs 2026-08-22 — all hold; disclosure per the EXTERNAL-CLAIMS grammar.
  Billing note, R16: bare mode reads no OAuth credentials — runs are metered API spend via
  `ANTHROPIC_API_KEY`, and `total_cost_usd` is a client-side estimate.)*
- **F8 — current mass (2026-08-22, `find plugins/mochiko/skills -name '*.md'`):** **82 `.md` files** *(corrected R16; 89 was the
  all-files count — 82 md + 5 py + 1 sh + 1 yaml, scripts/yaml out of scope per F1)*,
  687,104 bytes across all files. 37 `SKILL.md`. Largest bodies: `mochiko/SKILL.md` 39,546 · `review-
  feasibility` 19,746 · `authoring-constitution` 18,574 · `authoring-feature-map` 16,637 ·
  `review-plan-artifacts` 14,268 · `patterns-entity-modeling` 14,423 · `testing-end-user` 13,765.
  Largest reference: `review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md` 23,543.
  Char measures (ledger-canonical, added R16 — accounting law is chars, never `wc -c` bytes):
  `mochiko` 38,884 · `review-feasibility` 18,959 · `authoring-constitution` 17,874; the
  byte-ranked D7 picks are unchanged under chars (reviewer re-measured).
- **F9 — editing a shipped primitive is a landing, not an edit.** Every removal takes a
  `.mochiko/strips/<primitive>.md` entry (strip or supersession-by-ruling, version-stamped) plus
  the independent author≠grader audit before the `plugin.json` bump (CLAUDE.md; GI-004, GI-006).
  Protected content — a record's protected set, a `KEPT:` line, a `DECISIONS.md`-traceable line —
  leaves only as a recorded supersession-by-ruling.
- **F10 — no repo-level skill directory exists yet.** `.claude/skills/` is absent; `.claude/rules/
  mochiko/` holds four path-scoped rules (`operating-docs.md`, `output-style.md`,
  `primitive-edits.md`, `rust-cli.md`).

## Decisions

- **D1 — New dependent session, not a resume and not a supersession.** `Confident` (user-ruled:
  "yes b, based on what already exists. both have a lot of related context"). This session cites
  `skill-succinctness-strip`, `validator-scope-and-verbosity`, and `primitive-eval-harness` as
  ground facts and decides only what is new: the repo-level compressor skill and the before/after
  judge protocol. The cut line (F3), the surviving accounting rules (F2), and the harness design
  (F6) are inherited, not re-litigated. `primitive-eval-harness` stays open on its own terms.

- **D2 — Two compression arms, staged low-risk to high-risk, against a recorded baseline.**
  `Confident` (user-ruled: "both would be good. less risky is a and more risky b. going a to b,
  running baseline would give a lot of learning"). **Arm A = lossless densification** — headings,
  rules, MUST/SHOULD grading, and example count all preserved; only restatement, hedging,
  throat-clearing, and prose duplicating an adjacent table are removed; zero information leaves.
  **Arm B = the inherited cut line** (F3): keep goal + output contract / non-waivable floor /
  anti-patterns and rejections / hard reference-data, drop procedure / examples / restatement
  *("output contract" restored 2026-08-22 with F3 — audit B1 marker)* — whole sections may vanish, information loss is by
  design and is precisely what the eval prices. The compressor emits both variants per skill;
  the eval scores **baseline vs A vs B** so the knee is attributable rather than blended into one
  number. Rationale: F2 (measure-then-gate is the governing law — the bands are dead, the
  measurement is the gate), F5 (a blended arm is exactly what the prior benchmark could not
  resolve when noise fired). *(As amended post-review: effect-size re-priced — the pilots are twice-compressed,
  R1; a no-skill control arm joins the design, R3.)*

- **D3 — Substrate is one skill in isolation; command-level runs are out of scope, permanently.**
  `Confident` (user-ruled: "one skill at a time, full command is not i am after and is out of
  scope. For full command, i have dogfood projects that i will use real work experience. I am
  looking to build the lowest level evaluation primitive"). One run = one `claude -p --bare
  --plugin-dir` session loading one skill, one prompt, one artifact out (~$0.55 Sonnet / ~$1.00
  Opus per run, F6). Ecological validation at command/cluster altitude is delegated to the user's
  real dogfood projects and is explicitly NOT an artifact of this session — no staged phase-2
  cluster run. The deliverable is the **lowest-level evaluation primitive**: reusable per-skill,
  not a one-off study of one compression pass. Rationale: F5 (multi-seat stage-runs carry every
  seat's variance — that is the noise mode that broke the prior benchmark), F6 D1/D5 (the harness
  already ruled synthetic per-skill goldens and a single-skill pilot), and the user's standing
  dogfood practice already covering the ecological question.

- **D4 — Instrument: rule-coverage checklist primary, holistic pairwise read secondary.**
  `Confident` (user-ruled "as recommended", after a plain-language restatement). Primary
  measurement: enumerate every behavioral rule the **baseline** skill asserts (each MUST/SHOULD,
  floor, anti-pattern, format obligation); the judge answers one binary per rule against the
  produced artifact — honored / not honored — with a quoted span as evidence, `{text, passed,
  evidence}` triples in the `skill-creator` grading format (F6). No Likert, no numeric scores.
  A failed rule points at the exact cut that broke it. Secondary: one low-replicate blind
  pairwise A/B with position swap, catching degradation no enumerated rule covers. The rule
  inventory is a by-product of the compression pass itself — Arm B cannot apply the cut line
  without first enumerating what the skill asserts — and the same inventory feeds the
  `.mochiko/strips/` entry (F9). Known risk, named: an inventory authored from the baseline by
  the compressor misses rules the compressor never noticed → author≠grader on the inventory
  (a non-author seat builds or reviews the rule list), per the sound-loop floor. Rationale: F5
  (holistic scoring is the instrument that drowned in replicate noise), F4 (variance discipline:
  binary + quoted evidence, never scores), F6 D2 (judge advisory, never blocking).

- **D5 — File topology is fenced.** `Confident` (user-ruled "as recommended"). Neither arm may
  rename files, merge `references/*.md` into `SKILL.md`, split new reference files out, or
  relocate content anywhere (including `templates/`, already banned). Densify or delete in
  place only — relocation disguised as reduction is what true-reductions-only exists to catch.

- **D6 — `description:` frontmatter is out of scope for this primitive.** `Confident`
  (user-ruled "as recommended"). It is the trigger surface with a live truncation defect (F1
  C1) and the highest leverage — but the wrong instrument here: a non-firing skill yields a
  null result, not a gradable artifact. It needs a trigger-fire-rate probe, which stays on the
  standing BACKLOG watch (slim-description fire-rate, F7 of `primitive-eval-harness`). This
  session's scope: bodies + references only.

- **D7 — Pilot set: `patterns-entity-modeling` plus the two biggest — `mochiko` (router) and
  `review-feasibility`.** `Confident` (user-ruled: "as recommended but i want to pick 2 more
  skills. pick by biggest 2"). By SKILL.md body size the biggest two are `mochiko/SKILL.md`
  (39,546 B, the user-invoked router) and `review-feasibility/SKILL.md` (19,746 B + 13,558 B
  reference). Noted at ruling time: the router's golden task is a different shape — routing
  accuracy ("given this request, which skill do you reach for?"), deterministically assertable,
  rather than artifact quality — which extends the primitive's coverage to both task shapes.
  `review-feasibility` gives the judgment-artifact contrast (a 3-state verdict), the noisier
  judging case, deliberately in the set. (If "biggest" was meant by cluster mass instead,
  `authoring-constitution` at ~86.7 KB across 10 files is the largest cluster — swap available
  on request; not chosen because the ruling said files.) *(As amended post-review: mandatory
  pre-arm protected-content reconciliation — both non-router pilots carry `KEPT:` survivor
  rulings, R2; the router rides Arm A only — its body is deliberately unbudgeted because the
  body IS the router index, R11; AR-D3's seeded-defect method named as the follow-on instrument
  for the review-skill pilot, R13.)*

- **D8 — Execution details.** `Confident` (user-ruled "confirmed" on the block). **Goldens:**
  3 per pilot skill (harness D5's 3–5 band), authored by a non-compressor seat — the same
  author≠grader logic as the rule inventory. **Replicates:** 3 arms (baseline / A / B) × 3
  replicates = 27 runs per skill (~$15–27 Sonnet; ×3 pilots ≈ $45–80 + judging). **Gate rule:
  pass^k** — a rule counts as held only if it holds across all replicates; pass@k flatters
  flaky artifacts (F4 variance discipline). **Judge models:** Haiku for the checklist binaries
  (~$0.035/transcript), Sonnet for the secondary pairwise read; both advisory, never blocking
  (F6 D2, inherited). **Compressor home:** `.claude/skills/compressing-skills/SKILL.md` —
  repo-level, never shipped, library naming convention (gerund + object); owns the
  rule-inventory procedure, the Arm A/B passes, eval dispatch, and the report format; delegates
  the cut line (F3, ruled) and the landing ritual (F9, inherited). **Landing:** a ratified
  compression lands through the standard ritual unchanged — per-cut strips entry with the rule
  inventory attached as evidence, author≠grader audit, `plugin.json` bump, CHANGELOG; the eval
  report rides the audit as evidence, never replaces it. *(As amended post-review: four arms —
  no-skill / baseline / A / B — at 3 goldens × 4 arms × 3 replicates = 36 runs/skill ≈ $20
  Sonnet, ×3 pilots ≈ $60 + judging (cost anchors are harness F17, R16/M3); session model under
  test = Sonnet, R7; pre-registered ship bar before any run, R6; ratified compression re-seeds
  the body budget downward at landing, R11.)*

## Build surface

*(what a build session executes; nothing here is built yet — as amended by review folds:
pre-arm protected-content step R2 · minimal per-run plugin dir R4 · probe run before the grid
R5 · pre-registered ship bar R6 · consumer-side check in the rule inventory R10 ·
delivered-chars arithmetic before the grid is authorized R9 · Arm A runs once per skill per
landing, re-add path = strips README re-add entry type R16 · compressor description kept
minimal and the repo-level skill voluntarily takes the author≠grader audit R16)*

1. **`.claude/skills/compressing-skills/SKILL.md`** — the repo-level compressor skill (D8):
   procedure = rule inventory (non-author-reviewed) → Arm A pass → Arm B pass → eval dispatch →
   report → user ratification gate → landing ritual hand-off. Fenced per D5; scope per D6.
2. **`evals/` top-level dir** (never shipped; home ruled by `primitive-eval-harness` D4):
   ~200-line thin runner driving `claude -p --bare --plugin-dir plugins/mochiko --output-format
   json --permission-mode dontAsk --max-turns N`; `plugin_errors` smoke; per-skill
   `evals/<skill>/evals.json` goldens (skill-creator format); checklist judge (Haiku, binary +
   quoted evidence, `{text, passed, evidence}`); pairwise judge (Sonnet, position swap);
   committed baseline results file regenerated only as a deliberate landing act.
3. **Pilot execution** (D7 order): `patterns-entity-modeling` end-to-end first — proves the
   loop and fills `primitive-eval-harness` D5's deferred pilot slot — then `mochiko` (router;
   routing-accuracy golden shape) and `review-feasibility` (verdict-artifact golden shape).
4. **Cross-session fold:** on acceptance, annotate `primitive-eval-harness` record (open) that
   its D5 pilot slot is being filled by this session's build; that session still closes on its
   own terms.

## Cold review, dispositions, and folds (2026-08-22)

- **Sizing:** solo cold reviewer (user-ruled at the named human gate); 36-angle blind map frozen
  before any record read; protocol per `mochiko:review-brainstorm`.
- **Verdict as returned:** critical-gaps (FAIL) — 26 raised, 23 survived (3 Critical,
  12 Important, 8 Minor). Lead independently spot-verified the decisive citations (DECISIONS.md
  succinctness row · both pilots' `KEPT:` strips entries · budget-ledger router stance · the
  82-file count): all hold.
- **Disposition:** full batch user-ruled — every finding folded, none rejected. Streak doctrine
  honored: D4–D8 were five consecutive recommendation-led adoptions, flagged by the reviewer
  (I12); the user **explicitly re-affirmed D4–D8** at disposition. The three judgment calls
  (C2 keep-pilots-with-reconciliation · I4 Sonnet session model · I9b router-Arm-A-only) were
  delegated to the lead's recommendation and so ruled.

Folds (finding → fold): **R1**(C1) F1 rewritten truthfully, effect size re-priced ·
**R2**(C2) mandatory pre-arm protected-content step — enumerate `KEPT:`/`DECISIONS.md`-traceable
sets from `.mochiko/strips/<skill>.md`; Arm B touches protected content only via recorded
supersession-by-ruling (v0.64.0 reconciliation is the worked precedent); pilots kept ·
**R3**(C3) no-skill control arm; rules passing with no skill loaded are pruned (they measure the
model, not the skill) · **R4**(I1) runner synthesizes a minimal per-run plugin dir — pointing
`--plugin-dir` at `plugins/mochiko` loads all 37 skills and dilutes contrast · **R5**(I2) probe
run precedes the grid: `plugin_errors` needs `stream-json`; `dontAsk` denies writes absent allow
rules — both settled empirically before any priced run · **R6**(I3) pre-registered ship bar,
recorded before any run: floor rules absolute, rules-lost bound fixed at pre-registration ·
**R7**(I4) session model = Sonnet (conservative: a smaller model following the compressed skill
implies larger ones will) · **R8**(I5) unaccepted-upstream risk named, in Open questions ·
**R9**(I7) delivered-chars arithmetic per pilot per arm before the grid is authorized; D6 stands
with the honest note that descriptions are the always-loaded surface and bodies load only on
invoke — the primitive's durable value is the instrument · **R10**(I8) consumer-side check joins
the rule inventory (strips' `Consumers assessed` obligation); paraphrase breaking a consumer is
a failed rule · **R11**(I9) a ratified compression re-seeds the body budget downward at landing;
router rides Arm A only · **R12**(I10) GI-019 trace below · **R13**(I11) AR-D3 named as the
follow-on instrument if the review-skill pilot's checklist proves insensitive · **R14**(I12)
streak re-affirmation recorded above; Open questions restored to honesty · **R15**(I6) Rejected
roads below · **R16**(M1–M8) F8 82-files fix · char measures added, picks unchanged · D8
arithmetic fixed (36 runs/skill; anchors are harness F17) · F7 disclosure + billing note ·
Arm A stopping rule + strips re-add path · compressor description minimal + voluntary
author≠grader audit (GI-004 scopes `plugins/mochiko/**`; extended by choice) · stale index
entry fixed on sight (M8).

## Rejected roads (recorded post-review, R15)

- **Do nothing** — the D7 char-budget gate already catches growth. Rejected: it prices drift,
  not degradation; degradation is the unanswered question.
- **Hand editorial cut via the proven wave path** — rejected as the sole road: it is exactly
  what ran at v0.24.0–v0.28.0 and left the degradation question unmeasured; this session builds
  the missing instrument.
- **Relocate body detail into `references/`** (steelmanned: reference files are on-demand,
  never auto-loaded — a real delivered-token lever available today). Kept OUT of the arms for
  attribution cleanliness — relocation mixed with compression muddies what the eval measures —
  but NOT foreclosed generally: a future relocation pass is a legitimate separate lever needing
  its own accounting ruling against true-reductions-only.
- **Schema-CLI carries the detail** (v0.76.0 precedent) — out of scope: template guidance, not
  skill prose; would need its own GI-019 case.
- **Progressive disclosure / skill splitting** — topology change; banned by D5's fence for the
  arms, unexamined beyond them.

## GI-019 admissibility trace (R12)

The `evals/` runner and both judges are maintainer-side advisory tooling, not kernel-class: no
primitive depends on them for delivery or composition (skills ship and function with the harness
absent); they never gate pipeline progress (the ship decision is the user's ratification, the
author≠grader audit unchanged — the eval report rides it as evidence only); they never dispatch
or sequence pipeline agents (the runner spawns isolated throwaway eval sessions, not workflow
seats); the judges hold no judgment skills own (they grade eval outputs, advisory-never-blocking,
harness D2). Same class as the GI-008 advisory scripts and the qa-gap-finding D10 mutation-tool
precedent. GI-020 untouched: nothing ships under `plugins/`.

## Open questions

*(elicited unknowns surface here — non-waivable floor)*

- **R8 risk:** this record inherits harness D1/D2/D4/D5 from the `primitive-eval-harness`
  record, which is paused, not cold-reviewed, not accepted. Trigger: if its eventual review
  overturns its D4 (thin scripts under `evals/`), build-surface item 2 re-opens here.
- **Pre-registration values** — the rules-lost bound (R6) and the representative invoke for the
  R9 arithmetic are set at build time, before any run, and recorded then.
- Session trail: Q1 placement (D1) · Q2 arms (D2) · Q3 substrate (D3) · Q4 instrument (D4,
  restated plainly on request) · Q5a–c fence/descriptions/pilots (D5–D7) · Q6a–d execution
  (D8) · review sizing (solo, user-ruled) · disposition batch (2026-08-22, all folded,
  D4–D8 re-affirmed).
- Accepted + landed 2026-08-22. Build execution is the open work — BACKLOG
  "Skill-compression eval-primitive build".
