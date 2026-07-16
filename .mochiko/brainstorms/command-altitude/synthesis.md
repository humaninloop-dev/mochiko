# Command Altitude Analysis Synthesis

_Brainstorm: 2026-06-30. Companion to [`agent-decoupling/synthesis.md`](../agent-decoupling/synthesis.md). Subject: why the transformed commands came out verbose, and how to fix the transformation so they don't._

> **Status (2026-06-30): acted on.** All four recipe fixes landed; `specify` (329→66) and `setup` (385→78) retrofitted and independently verified PASS. Substrate question deferred. Tracked in `REGISTRY.md` / `ROADMAP.md` / `BACKLOG.md`.

## Problem Statement

The transformed mochiko commands are written at the wrong **altitude**: `specify.md` (329 lines) and `setup.md` (385 lines) read like the deleted Python/MCP kernel transliterated into English — every `Task()` prompt body, `AskUserQuestion` option array, `mkdir`/`test -f`, and `round = 1; if >= 3` counter spelled out inline. The refined diagnosis is **not** "verbose prose": it is that the command **duplicates discipline that `loop-discipline` + `workflow-contract` already single-source** — a `single-source-rule-fanout` violation — and the transformation recipe that produces commands has no step to prevent it. A command's irreducible job is to **stitch a team to a goal under a contract**: declare the team, state the goal + the per-workflow contract parameters, reference the shared doctrine, name the human gates — and stop there.

## Context & Constraints

- **Kernel-free, no hooks** (mochiko non-negotiable). The discipline cannot move into code or harness gates; it lives in skills + templates, referenced by the command.
- **The discipline already has a home.** `loop-discipline/SKILL.md` single-sources the four requirements, validator tiers, tamper-proofing, gap-type routing, and the anti-rationalization payload; `workflow-contract.md` is the fill-in form. The commands restate all of it (the intro, an inlined filled contract, *and* a "Supervisor behaviors" footer — three times).
- **Producer↔validator independence is structural and load-bearing** (different agent + different skill). Any thinning must preserve it.
- **`loop-discipline` forbids vague aspirations** ("the loop basically validates" is the named failure). So "thin" must be expressed as a *testable* gate, not a reviewer's taste.
- **Agent teams** (the original framing) are experimental, flag-gated, ephemeral, **ignore the `skills:` frontmatter** for teammates, and their direct messaging can *dilute* the producer↔validator independence an adversarial pair needs.
- **The transformer is dogfooded** — it *is* a mochiko cluster. Fixes to `assess`/`transform`/`verify` must keep the transformer itself sound.

## Key Decisions

| Decision | Choice | Confidence | Rationale |
|----------|--------|------------|-----------|
| Altitude vs. substrate ordering | Fix **altitude first**, substrate-agnostic | Confident | The complaint is altitude, not substrate; teams won't thin anything on their own, and may cost independence. |
| Reliability floor (how thin is safe) | **B** — reference `loop-discipline` for all generic rules; state only per-workflow params; inline anti-rationalization backstop added **only if dogfooding shows a gate being rationalized** | Confident | Trades reliability-by-repetition for reliability-by-invocation; the empirical-backstop rule mirrors the ROADMAP's "let real drift, not theory, trigger it." |
| Root-cause locus | The **transform recipe**, not the output: `absorb-into-lead` sweeps generic discipline into the command and nothing single-sources it back out | Confident | A hand-fixed `specify.md` is one command; the recipe will keep emitting verbose ones for `plan`/`tasks`/`implement`. |
| Thinness enforcement | **C** — layered: deterministic grep floor (no inline contract, no restated doctrine) under a grounded keystone-test ceiling; as a **hard `verify-output` gate** | Confident | The exact tier-1-under-tier-2 layering the framework already blesses; turns "thin" from a hope into a gate. |
| Retrofit vs. forward | **Retrofit `specify.md` first** as the canonical thin exemplar (re-transform, not hand-edit), then forward-apply; `setup.md` follows | Confident | Validates the fixed recipe end-to-end on a known-good case and sets the reference pattern. |
| Substrate (teams vs. subagents) | **Deferred** until a thin command exists to show what the substrate must carry | Deferred | Key inputs already gathered; the decision is better made against a realized thin command. |

## Decision Trail

### Altitude vs. substrate — separating two fused problems
- **Options considered**: (A) substrate swap to native agent teams; (B) altitude fix, substrate-agnostic; (C) both, sequenced.
- **Recommendation was**: C, starting with B.
- **Chosen**: C, starting with B.
- **Key reasoning**: The "stitch together an agent team" framing fused *substrate* (teams vs. the `Task`-subagent dispatch the commands actually use) with *altitude* (spell-out vs. reference). Research showed they are separable and that agent teams would not thin the command on their own — the discipline still has to live somewhere — while teams' direct messaging can dilute the very independence the doctrine demands. Altitude is the real lever.

### Making "thin" testable — resolving the collision with loop-discipline's own rule
- **Options considered**: (A) keystone test for command lines (judgment); (B) deterministic grep denylist (brittle); (C) both, layered.
- **Recommendation was**: C.
- **Chosen**: C.
- **Key reasoning**: An altitude check phrased as "is this thin enough?" is exactly the vague aspiration `loop-discipline` rejects. The deterministic floor (no inlined contract, no restated doctrine phrases, must reference `loop-discipline`, must fill a contract artifact) catches the obvious cases unrationalizably; the grounded keystone ceiling ("true of any sound loop → belongs in `loop-discipline`; only true of THIS workflow → stays") catches paraphrase and non-workflow-specific lines. Mirrors the decoupling scan's own grep-first-then-judge structure.

## Risks

- **Invocation reliability.** A thin command bets `loop-discipline` reliably *fires and is obeyed* mid-loop under rationalization pressure. Mitigated by the empirical inline backstop (Decision 2) and the `verify-output` altitude gate, but ties to ROADMAP OQ#2 (Claude-Code model-invocation portability).
- **Altitude gate is partly tier-3 judgment.** The grep floor is brittle (misses paraphrase); the keystone ceiling is a grounded-LLM call that can mis-grade. Layering reduces but does not eliminate this.
- **Retrofit regression.** `specify.md`/`setup.md` are landed, independently-verified artifacts whose verbosity encodes hard-won fixes (the `DROPPED` notes: the `@`-reference recovery, the non-auto-resolved constitution prerequisite, the "advocate status is input, not the gate" reversal of HIL). Thinning must preserve every workflow-specific responsibility via the trace — generic discipline moves to references; workflow-specific fixes stay.
- **Substrate debt.** If teams are adopted later, the `skills:`-frontmatter-ignored gap reopens convention axis 4 (skill-augmented-agent); the thin command must not assume the team substrate.

## Open Questions

- **Substrate**: native agent teams vs. `Task`-subagent dispatch (deferred by design).
- Does a bare "honor `loop-discipline`" reliably trigger the skill in a command context, or is a minimum inline backstop empirically required? (Resolve during dogfooding.)
- Where does the per-run filled `workflow-contract` artifact live, and is it committed alongside the workflow or treated as ephemeral run-state under `.mochiko/`?

## Recommended Next Steps

1. **Fix the three transformer skills** so future commands come out thin: `assess-primitive` (command branch traces generic discipline to `dedupe`→`loop-discipline`, not `moved-to-lead`); `transform-recipes` (`redesign`/`absorb-into-lead` recipes target goal+team+contract-by-reference; add a single-source step to the wiring pass); `verify-output` (new altitude / single-source conformance check, grep floor + keystone ceiling).
2. **Retrofit `specify.md`** via the fixed recipe as the canonical thin exemplar (re-transform with an independent producer→validator pass, preserving the full responsibility trace), then **`setup.md`**.
3. **Record the decisions** in `ROADMAP.md` (Key Decisions), and update `REGISTRY.md` (mark specify/setup re-transformed) and `BACKLOG.md` (close the altitude item; log the substrate follow-on).
4. **Re-open the substrate question** (teams vs. subagents) once a thin command exists to evaluate against — the inputs are already gathered in this synthesis.
