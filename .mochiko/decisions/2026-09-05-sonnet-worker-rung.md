# Sonnet worker rung — bounded coding and verification tasks delegated by the two build-time seats

- **Status:** ruled
- **Date:** 2026-09-05
- **Context:** The model-tiering floor (`model-tiered-seats` D4, built 2026-08-16 at v0.77.0,
  retargeted 2026-08-19 at v0.78.0) gives every seat one cheap rung: locate/enumerate/targeted
  reads go to a native `Explore` subagent spawned `model: haiku`. Everything else — every read
  that needs judgment and *all* producing, reviewing, and grading work — stays on the seat's own
  tier, and `patterns-model-tiering.class-key-session-tier` (a `class: floor` rule) says so in
  terms: "all producing, reviewing, and grading work — never tiered down (model-tiered-seats
  D5)". The two build-time seats, `staff-engineer` and `qa-engineer`, are the library's
  highest-volume seats (D5's own seed material: "the verifier's ~15+ rounds are where 5×
  compounds"; implement is the discipline-surface command), and much of what they do per cycle is
  execution of something already decided: writing the green-phase code against a failing test
  the seat wrote, running a suite and reading its output, executing a `**TEST:**` case's Setup
  and Action legs and capturing evidence. That work carries its own oracle — the test, the
  assert, the exit code — so a weaker model's output is cheap for the seat to check, which is
  the property the cheap-read rung already keys on ("the lowest tier where the answer can be
  trusted"). Haiku is the wrong tier for it: it is not a reliable coder. Sonnet is. The user
  ruled (2026-09-05): the staff engineer and QA engineer may use Sonnet subagents to offload
  coding and verification tasks as they see fit.
- **Decision:** A second dispatch rung, the **worker rung**, joins the tiering floor. Shipped
  as v0.108.0:
  1. **The rung.** A native general-purpose subagent spawned via the Agent tool with an explicit
     `model: sonnet` override — the override is the pin, exactly as on the cheap rung
     (`patterns-model-tiering.override-is-the-pin` is reworded to name both rungs; its id and
     floor class survive); a bare spawn inherits the session tier and fails the floor. Disposable per task; a worker never
     spawns a worker.
  2. **Who holds it.** The `staff-engineer` and `qa-engineer` seats only. The seat set is a
     reserved decision: a run never extends it, and it is carried by a new `class: floor`
     reservation rule (`patterns-model-tiering.worker-seat-set-reserved`).
  3. **What may go down (the class key's worker tier).** Decided, mechanically-checkable
     execution: green-phase implementation against a failing test the seat has already written
     and an approach the seat has already fixed; a mechanical refactor whose oracle is the
     passing suite; boilerplate of a decided shape; running a suite, lint, or build and
     capturing exit code and output; executing a `**TEST:**` case's Setup and Action legs
     against real infrastructure and capturing the evidence.
  4. **What never goes down.** The judgment legs of producing, reviewing, and grading: card
     decomposition and the pre-code ladder; the red-phase test and its failure-reason check;
     refactor and brownfield-interface calls; every assert evaluation and the
     auto-approve-versus-checkpoint classification; the code-shape audit; every finding,
     verdict, checkbox flip, and report. `class-key-session-tier` is reworded to say this
     (its id and floor class survive; the migration carries the ruling anchor).
  5. **Latitude.** Whether a given task goes down is the seat's call per task — the class key
     bounds what *may* go down and never obliges a dispatch. The dispatch ladder gains the
     rung: direct tool call → cheap `Explore` (haiku) → worker (sonnet) → the seat itself.
  6. **Safety floor.** A worker's return is a claim, never a result
     (`patterns-model-tiering.worker-return-is-a-claim`, `class: floor`): the seat reads back
     every delegated diff against the test it wrote and every delegated evidence capture against
     the assert it owns, re-running the decisive command itself where that is cheap, before
     anything counts; a return that fails read-back is re-briefed or redone, never reported as
     done or patched around. The brief pins one task and its fence (files, oracle, decided
     approach, what it must not do, return shape). Every dispatch is disclosed in the seat's own
     report, which the seat authors itself.
  7. **Delivery channels.** The rules land in `patterns-model-tiering` by migration
     `0004-sonnet-worker-rung.yaml`; the two personas gain a `## Delegating Bounded Work`
     standing section (the persona body is the channel that reaches a seat on both transports,
     per the 2026-08-16 ADR item 5); the router row and the skill's `description:` name the
     rung. The contract suite's frozen floor set for the skill is replaced by this ruling
     (`floor_ids` and `floor_pin` only, per `evals/contract/README.md`): four ids become six.
  model-tiered-seats **D5 stands**: no rostered seat changes tier — both personas stay
  `model: opus`, and a worker below a seat does not retier the seat; its model, judgment, and
  ownership of the output are unchanged. The Sonnet-vs-Haiku *seat* question D5 deferred is not
  touched by this ruling, which is dispatch-level only, as D4 was.
- **Probe (2026-09-05, same day, user-requested):** mechanism confirmed live. A headless
  session (`claude -p --plugin-dir plugins/mochiko --setting-sources ''`, `mochiko-cli` on PATH)
  spawned `mochiko:staff-engineer` on a decided card (failing unittest present, approach fixed);
  the seat reported the `## Delegating Bounded Work` section present, spawned `general-purpose`
  with `model: sonnet`, read the diff back against the test, and re-ran the suite itself (OK).
  Sidechain transcripts carry the API-side ids: the seat's `claude-opus-5`, the worker's
  `claude-sonnet-5`. Two method notes for the next probe: an in-session spawn seats the
  *installed* marketplace copy of the persona, not the working tree (the first attempt ran on
  v0.103.0 and saw no section); and a persona spawn preloads its `skills:` frontmatter, whose
  `!` lines need the binary on PATH or the seat never instantiates.
- **Rationale:** The economics are D1's — cheaper-model work draws the per-token arbitrage on
  API-billed installs and runs outside the Opus-specific sub-cap on subscription seats — and the
  seats where volume compounds are exactly the two this ruling names. The trust argument is
  D4's, applied to execution rather than reads: a task whose oracle the seat already holds is
  one whose result the seat can verify cheaply, so a cheaper tier can produce it without the
  seat's judgment leaving the seat. Read-back is a floor, not advice, because the failure mode
  (a plausible diff that passes nothing, or evidence that was inferred rather than captured) is
  the expensive-to-verify one D5's seed material warned about; disclosure keeps the lead and the
  reviewing seat able to see what a worker touched. Doctrine-only enforcement is unchanged — no
  hooks, nothing kernel-class, nothing gates or dispatches mechanically.
- **Alternatives considered:** retier the two seats to Sonnet (rejected — D5, and the judgment
  legs are the point of keeping them strong); Haiku workers (rejected — not a reliable coding or
  verification tier; the cheap rung stays reads-only); the rung for all ten personas (rejected —
  the user scoped it to the two build-time seats, the only ones whose work carries a mechanical
  oracle per task; widening is reserved); a blocking hook or a mandatory dispatch (rejected —
  kernel-class gating on one side, and on the other the seat's per-task latitude is the ruling's
  own shape).
