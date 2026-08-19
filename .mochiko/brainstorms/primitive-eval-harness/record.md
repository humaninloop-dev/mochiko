# Evaluation Harness for Primitive Changes (CI/CD) — Decision Record

**Status:** open
**Opened:** 2026-08-19
**Session:** collaborative brainstorm via `mochiko:analysis-iterative` (one question per turn)

## Topic

Explore whether an evaluation harness — [deepeval](https://github.com/confident-ai/deepeval)
or another framework — can protect CI/CD changes to mochiko's primitives: first changes to
skills alone, then changes to skills + agents together. Deep research on the harness
landscape is part of the session.

Threads:

1. What an eval would actually catch — the failure modes of editing a skill or an
   agent+skill cluster that today's procedural gates (author≠grader audit, strips,
   landing ritual) miss.
2. Whether deepeval or an alternative (promptfoo, Inspect AI, Braintrust, native
   Claude Code headless runs, custom Rust harness…) fits evaluating markdown prompt
   artifacts rather than application LLM outputs.
3. Admissibility under the kernel-class ruling (GI-019) and the additive-install
   constraint (GI-020): where an eval harness may sit (maintainer-side CI, advisory
   exit-code signal) and where it may not (pipeline gating, judgment that skills own).
4. Cost/flakiness/signal: LLM-as-judge determinism, per-run token cost, what a
   regression suite for prose primitives even asserts.

## Ground facts

*(repo fact-finder sweep 2026-08-19, Explore seat, read-only; paths verified by the seat.
Full web-seat reports preserved at `research/deepeval.md` and `research/harness-landscape.md`
— provenance-stamped, reference data only.)*

- **F1 — markdown primitives trigger zero CI.** `.github/workflows/ci.yml` (the repo's only
  workflow) path-filters on `crates/**`, `plugins/mochiko/schemas/**`, `Cargo.*`, and itself.
  A skill or agent edit runs no job. Six steps gate the crate: fmt · clippy · test · audit ·
  secret scan.
- **F2 — the admissibility door is already open.** GI-019 (ledger `:218–245`): "Advisory
  post-hoc checkers consumed as optional exit-code signals are NOT kernel-class." Bright line
  for anything admitted: never gates pipeline progress, never dispatches or sequences agents,
  never holds judgment that skills own. GI-020: plugin install stays markdown-only; any
  binary is strictly additive (absent = degraded path works).
- **F3 — this session is the reserved home for the question.** BACKLOG `:727–736` "Quality-
  control pipeline — deepeval or equivalent" is open and annotated by
  `qa-gap-finding-verification` D10 (2026-08-19): mutation-tool adoption there is *not* this
  eval-harness brainstorm; item stays open. That session's scope is product code built by the
  pipeline; this session's subject is the pipeline's own primitives — no boundary conflict,
  and D10 is a worked precedent for the GI-019 admissibility argument.
- **F4 — primitive verification today is fully manual model judgment.** The author≠grader
  audit = `mochiko:validator` (default-FAIL) grading text coherence + preserved
  responsibilities, plus a hand-run char-budget pre-assert (v0.81.0 sweep caught one drift by
  hand). No exit code anywhere. Strips (89 files) are the closest thing to a regression
  record — a prose log, machine-readable by nobody. **Nothing verifies behavior.**
- **F5 — LLM-judging mochiko primitives has been done once, and noise was the finding.**
  `validator-scope-and-verbosity` (2026-08-10) ran a real benchmark: 12 full simulated
  command runs (4 arms × 2 commands + replicates), frozen persona-card contamination fence,
  blind judge scoring 0–100 on a neutral rubric. On one of three arms, replicate spread
  (5.6/7.1) exceeded the effect gap (0.8/1.8) — the noise guard fired and the verdict had to
  be user-ruled. Reusable scaffolding survives: `RUN-PROTOCOL.md`, rubrics, judge scorecards,
  persona-card fixture (curated set at `.mochiko/benchmarks/guardrails-vs-detail/`, 464 KB;
  full trail unmerged on branch `worktree-brainstorm-validator-scope`).
- **F6 — a designed-but-never-run eval already exists.** `adversarial-review-generality`
  AR-D3 (2026-08-04): seeded-defect benchmark for review skills — ~10 seeds, 60/40
  in/off-taxonomy, independent cold scorer holding the seed key, ≥2 replicates,
  strict-majority hit. Parked for a dedicated session (AR-D5). BACKLOG `:324–341`.
- **F7 — standing watches are eval questions with no instrument.** Slim-description
  fire-rate ("do the 27 slim descriptions still fire?", BACKLOG `:213–227`) and blind-map
  coverage materiality (`cold-review-gap-challenge` open watch). Also a live warning from
  `skill-succinctness-strip`: the "descriptions load fully" premise was false — delivery
  truncates ~1.8k-char descriptions mid-sentence; one skill rendered no description at all.
  Any harness that assumes what loads must verify what loads.
- **F8 — surface to cover, and unit cost.** v0.81.0: 7 commands (1,288 lines) · 10 agents
  (1,065) · 37 skills (SKILL.md 4,982 lines; 89 files, ~99k words with references). Cost
  anchor from F5: one evaluation arm ≈ one full simulated command run.
- **F9 — the crate has no eval surface but a usable hook.** `crates/mochiko-cli` renders
  schema views only; `run()` lives in the library so tests can assert exit codes without
  spawning the binary.

### Elicited — current practice (user, this session)

- **F19 — verification today is dogfood + eyeball, no baseline.** User dogfoods primitive
  edits across multiple real projects and eyeballs output; cannot tell regression from
  run-to-run variance ("not 100% sure if it is a regression"); fixes what feels worth
  improving. The missing pieces are exactly: fixed repeatable scenarios, a recorded
  baseline, and a diff signal.

### Web sweep — deepeval (research seat, docs v4.1.8 + issue tracker, fetched 2026-08-19; no
third-party field reports — WebSearch blocked for the seat, disclosed)

- **F10 — deepeval mechanically fits, structurally misfits.** It wraps any Python callable
  (documented no-tracing path), so `subprocess.run(["claude","-p",...])` is within contract.
  But the expensive 80% — spawn the Claude Code session, isolate it, capture/parse the
  transcript into `Turn`/`ToolCall` — has no integration and is all custom. No Claude
  Code / Agent SDK instrumentation exists; trace-requiring metrics (`TaskCompletionMetric`)
  need synthetic replay stubs.
- **F11 — its gate is soft exactly where CI needs it hard.** Docs state G-Eval "is NOT
  deterministic"; the vendor's CI remedy is `flaky=True` (warn, don't fail); the
  deterministic alternative `DAGMetric` has an open determinism bug (#3055); and
  baseline-vs-main regression comparison (`--official`) is the paid Confident AI side —
  local runs get static thresholds + JSON you diff yourself. Anthropic judge is first-class
  (`AnthropicModel`) but G-Eval's logprob-weighted scoring is OpenAI-shaped — with a Claude
  judge it silently falls back to coarse integer scores (#2854).
- **F12 — what's worth raiding regardless of adoption:** goldens with `expected_tools`
  versioned in-repo · pinned `evaluation_steps` over free-form criteria (biggest determinism
  lever the vendor names) · non-overlapping rubric bands over continuous 0–1 ·
  deterministic-gates/judged-advisory split · `ToolCorrectnessMetric`-style arithmetic
  tool-call comparison ("~fifty lines to reimplement without deepeval"). The checks that
  would genuinely gate — right files read, right subagent dispatched, right tools called,
  output matches schema — are deterministic and need no framework.

### Web sweep — harness landscape (research seat, direct doc fetches + GitHub search, 2026-08-19;
WebSearch blocked for the seat — no blog-survey layer, disclosed)

- **F13 — the platform already supplies the hermetic runner and a free deterministic gate.**
  `claude -p --bare --plugin-dir <path>` = isolated session loading exactly the artifacts
  under test (`--bare` skips all auto-discovery; docs recommend it for scripted calls). The
  `system/init` event carries `plugin_errors` — docs say directly: fail CI on non-empty.
  Zero-LLM gate, one cheap session. `--forward-subagent-text` (v2.1.211+) exposes subagent
  text at every depth — what makes skill+agent cluster runs observable. `--permission-mode
  dontAsk` is the documented locked-down CI mode; `--json-schema` makes judge calls
  machine-readable; the JSON result self-reports `total_cost_usd`.
- **F14 — Anthropic ships an official skill eval loop, minus CI.** `skill-creator` plugin +
  agentskills.io format: `evals/evals.json` goldens beside the skill (`id`, `prompt`,
  `expected_output`, `assertions`), isolated subagent per case, `grading.json` binary
  `{text, passed, evidence}` triples (evidence must quote output; scripts over judges for
  anything mechanical), `benchmark.json` mean±stddev with with-vs-without-skill delta, blind
  A/B version comparison, should/shouldn't-trigger description tuning (3 runs per query for
  reliable trigger rate). Explicitly aimed at "catching quality regressions" and detecting
  obsolete skills (baseline passes without the skill). **No CI wiring ships — that gap is
  exactly what mochiko would build.**
- **F15 — prior art is fragmentary; nobody has consolidated this.** All community harnesses
  <15 stars. Closest: `mslavov/claude-code-eval` (MIT, promptfoo wrapper for slash
  commands/subagents, git-worktree-per-test isolation, assertion-only judging — file diffs +
  content checks, no LLM judge) and `bailejl/dev-plugins` (plugin marketplace where every
  plugin ships an eval suite; `plugins/` vs `evals/` vs `eval-infra/` layout; pass@k and
  pass^k; `BASELINE.md` gating, CI drafted not live).
- **F16 — variance discipline (converged across Anthropic guidance, shipped harnesses,
  MT-Bench literature):** push every mechanically checkable assertion out of the judge ·
  binary pass/fail + quoted evidence, never Likert scores · pairwise blind A/B for holistic
  quality with position-swap agreement (position/verbosity/self-enhancement biases
  documented) · pass^k (all k trials) for gates — pass@k flatters flaky artifacts · prune
  assertions that pass or fail in both configurations · 60/40 train/held-out if evals ever
  drive automated rewriting · threshold hysteresis unsolved in every framework — the wild
  pattern is a committed baseline file, gate at baseline-minus-margin, regenerate the
  baseline as a deliberate reviewed act (maps onto mochiko's landing ritual).
- **F17 — cost to design around:** ~$0.55/scripted session on Sonnet 5, ~$1.00 on Opus 5;
  Haiku judge over a 30K transcript ~$0.035. A 20-scenario × 3-trial PR grid ≈ $33 (Sonnet)
  to $60 (Opus) + $2–6 judging. Levers: nightly full grid + per-PR smoke subset · cheaper
  session model where the artifact isn't model-sensitive · Haiku for assertion grading ·
  `--max-turns` cap. Harness self-reports spend via `total_cost_usd`.
- **F18 — seat's ranked shortlist:** (1) hand-rolled runner over `claude -p`, adopting the
  `skill-creator` file layout wholesale and supplying the missing CI wiring — matches
  no-SaaS, no-new-runtime, advisory-exit-code constraints, composes with the existing cargo
  CI; (2) promptfoo + custom Claude Code provider if we'd rather not own repeat/storage/PR
  reporting (working references exist; cost: Node toolchain + YAML layer); (3) Inspect AI
  only if this outgrows into a real agentic benchmark (only framework with documented Claude
  Code bridging, but it routes model calls through its own provider — fidelity compromise
  when the system under test IS the Claude Code harness). OpenAI Evals retired (read-only
  2026-10-31); Braintrust ruled out on SaaS coupling; LangSmith on weight.

## Decisions

- **D1 — Goldens are synthetic per-skill, in the `skill-creator` format; real-project
  fixtures deferred to the cluster phase.** `Confident` (user-ruled "as you recommended").
  Small scripted scenarios versioned beside each skill (`evals/evals.json` shape: id ·
  prompt · expected_output · assertions), isolating one skill per run at ~$0.5/session.
  Frozen real-dogfood fixtures are not day-one scope — they enter, if at all, when the
  skill+agent cluster phase needs ecological fidelity. Rationale: F14 (format exists,
  Anthropic-maintained), F19 (current eyeball practice has no repeatable scenario — the
  harness's first job is freezing one), F17 (cost favors minimal isolated scenarios).

- **D2 — Deterministic checks may block; the LLM judge is advisory, never blocking.**
  `Confident` (user-ruled "as recommended"). Blocking layer: `plugin_errors` smoke (F13) +
  scripted assertions over the transcript (right tools called, right files touched, output
  shape). Judged layer: binary pass/fail + quoted evidence, pinned evaluation steps, blind
  A/B with position swap for version comparison — reported, never failing a run. Rationale:
  F5 (own benchmark: replicate spread exceeded effect gap), F11 (vendor's own CI remedy is
  "warn, don't fail"), F16 (converged practice), GI-019 posture (advisory exit-code signals
  sit outside kernel-class).

- **D3 — Local-first regression harness; CI wiring is deferred, not the goal.** `Confident`
  (user's words: "we dont need to focus too much on ci at the moment. as long as we have a
  way to test regression, even if it doesnt fit CI"). The deliverable is a maintainer-side
  runnable check — edit a skill, run the harness, see the regression diff before shipping.
  GitHub Actions wiring is a later, optional layer on top of the same runner; nothing in
  the design may depend on CI to be useful.

- **D4 — Thin scripts first under a top-level `evals/` dir (never shipped); stable pieces
  promote into the Rust crate once proven.** `Confident` (user-ruled from options).
  Runner ≈ 200 lines (shell/jq or small Python — the advisory-checker class the six
  existing GI-008 scripts occupy), driving `claude -p --bare --plugin-dir plugins/mochiko
  --output-format json --permission-mode dontAsk --max-turns N`. Rationale: fastest path to
  first regression signal (F18 shortlist #1), measure-then-gate culture (F5 precedent),
  GI-020 untouched (repo-side dir, not under `plugins/`), promotion path honors the D11
  foundation bet without paying Rust build cost before the harness proves value. promptfoo
  declined: Node toolchain + YAML layer for machinery a local-first harness (D3) mostly
  doesn't need.

- **D5 — First target: one pilot skill, end-to-end through all three layers.** `Confident`
  (user-ruled from options). Freeze 3–5 goldens for one frequently-edited skill, record a
  baseline, run smoke + assertions + advisory judge. Trigger-fire sweep over model-invoked
  skills (the standing BACKLOG watch, F7) and high-churn coverage come after the loop is
  proven once. **Pilot skill choice: `Deferred`** — user paused at the pick (2026-08-19).

## Open questions

*(elicited unknowns surface here — non-waivable floor)*

- Which skill is the pilot (D5) — user to pick on resume.
- Baseline mechanics — committed baseline file regenerated as a deliberate landing act
  (F16 pattern) — proposed, not yet ruled.
- Admissibility trace — lead's read: the harness is not kernel-class (primitives do not
  depend on it; it is maintainer-side advisory tooling, D10 precedent), but the reasoning
  deserves a recorded ruling on resume.
- Phase 2 (skill+agent cluster runs via `--forward-subagent-text`) — shape sketched in
  F13, not yet a decision.
- Judge model choice (Haiku vs Sonnet for assertions/judging) — detail, unruled.
- Session status: **paused mid-questioning 2026-08-19; record not yet cold-reviewed, not
  accepted.**
