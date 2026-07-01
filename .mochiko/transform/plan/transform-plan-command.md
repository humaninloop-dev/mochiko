# Transform — `commands/plan.md`

Run: `plan` cluster transform · Phase 3 (transform) · Producer: `transform-producer`
Skill: `mochiko:transform-recipes`
Source: `human-in-loop/plugins/humaninloop/commands/plan.md`
Target: `plugins/mochiko/commands/plan.md`  ← **written (82 lines, complete on disk)**
Disposition (finalized, human-gated): **`redesign × absorb-into-lead`** — the plan workflow's LEAD/supervisor,
thinned to the mochiko sound-loop shape (goal + team + per-workflow contract params + references).
Inputs consumed: `assess-plan-command.md` (P1, §A–§E altitude split), `reconcile.md` (§JOB 1 relational
verdicts, §JOB 2 rehome map A/B/C/D/E, §Independence attestation), the on-disk artifact, sibling target
shape `commands/specify.md` (66) + `commands/setup.md` (78).

> **Provenance.** The artifact was authored by a prior transform-producer session that hit a limit before
> writing this trace. This file **reconstructs** the realized responsibility trace from the on-disk artifact
> + the reconcile rehome map. ROLE: apply-the-decision bookkeeping only — **not** re-editing the artifact,
> **not** grading it. Grading is `verify-output`, a separate phase run by a different agent (the independent
> `validator`); this file is the realized trace that `verify-output` audits. Every claim below carries a
> `plan.md:<line>` anchor so the audit is independently checkable from the artifact alone.

---

## Applied

**`redesign × absorb-into-lead` + convention-wiring pass.**

- **Body = `redesign`:** the HIL body assumed a markdown-supervisor orchestration model over a
  `.workflow/plan-context.md` state-carrier, inlined every `Task()`/`AskUserQuestion()` payload +
  `supervisor_instructions` block, declared **"no hard caps"**, routed on each agent's verdict *field*, and
  shipped **no acceptance gate** on the deliverable. An edit cannot turn "inline everything" into "reference
  doctrine + fill a contract artifact"; the whole orchestration model changes and four gates are added. The
  realized target is the thin shape — goal + team + per-workflow contract params + references — matching the
  redesign specify (66) and setup (78) were built into.
- **Structural = `absorb-into-lead`:** a command IS its own lead. The workflow-specific orchestration stays
  in the thinned supervisor (§B, `moved-to-lead`); the generic discipline is *referenced*
  (`loop-discipline` + `workflow-contract` + `agent-dispatch`), never copied (§A, `dedupe`); the
  `.workflow/plan-context.md` state-carrier dissolves into workspace-as-state + in-session (P11
  `drop × absorb-into-lead`). No orphan skill created.
- **RQ1 landed as recommended option (i)** — two distinct validator agents: feasibility reviewer
  `principal-architect`+`validation-feasibility`, completeness reviewer `devils-advocate`+`validation-plan-artifacts`,
  the lead referees (`plan.md:8`, `:19`). The producer `technical-analyst` grades nothing.

---

## The altitude split, realized (the redesign headline)

The command reference-crossing rate is the altitude proof. All three legs of the assess split landed:

### §A generic loop-discipline → `dedupe` (REFERENCED, never re-inlined) — HELD

The thin command references the doctrine and restates none of it. Confirmed present as references, absent as
restatements:

| Generic mechanic (assess §A) | Realized | Evidence |
|------------------------------|----------|----------|
| iteration structure · four guards (as requirements) | `dedupe` | `plan.md:10` "invoke `mochiko:loop-discipline` and honor all four requirements … not restated here" |
| default-FAIL done-condition **mechanics** | `dedupe` | `:10`, `:18` (params only — see §E) |
| producer↔validator independence **doctrine** | `dedupe` | `:10` "independent validation"; the *casting* (workflow-specific) at `:8`/`:19` |
| validator trustworthiness tiers | `dedupe` | **absent from the body** (not restated) — correct |
| tamper-proofing (PASS only on evidence Read) | `dedupe` | referenced via `:8`/`:18` "grounded in the files" / "you Read" — mechanics not restated |
| gap-type routing (incl. "Research this") | `dedupe` | `:39`/`:49` "route it per `loop-discipline`'s gap-routing"; only the workflow **gate-mapping** is `moved-to-lead` |
| anti-rationalization (exhaustion ≠ done) | `dedupe` | `:18` "Out of rounds = escalate, never done"; `:49` "stays FAIL unless the human explicitly accepts" |
| briefing-each-dispatch mechanics | `dedupe`→`agent-dispatch` | `:10`/`:36`/`:45` "briefed per `agent-dispatch`" |
| git-footer | `dedupe` | `:82` "do not modify git or push" (standard lead-footer line) |

No inlined filled contract · no transliterated `Task()`/`AskUserQuestion()` payloads · no restated
four-rules/validator-tiers/gap-routing · no "Supervisor behaviors" doctrine footer. Token scan
(`Task(|AskUserQuestion(|Supervisor Instructions|supervisor_instructions|plan-context|no hard caps`) →
**only** `:23`, the rationale note *explaining* the dropped "no hard caps" (§E), not a payload.

### §B workflow-specific orchestration → `moved-to-lead` (18/18 landed)

Every §JOB 2A responsibility is present in the thinned supervisor:

| # (Job 2A) | Responsibility | Realized | Evidence |
|---|---|---|---|
| 1 | 2-phase analysis→design sequence (in-session + workspace evidence) | `moved-to-lead` | `:6`, Phase 1 `:32`, Phase 2 `:41`; state via workspace, `:65` |
| 2 | architect-feasibility-ONCE-after-Phase-1 → advocate ordering | `moved-to-lead` | `:34` "grades feasibility **once** … not in Phase 2", `:37` |
| 3 | skip-architect-unless-structural routing (silent-drop risk) | `moved-to-lead` | `:39` "Re-run the architect … **only on a structural change** … clarification-only … straight back to step 3" |
| 4 | incremental review mode-selection (lead selects; procedure in P9) | `moved-to-lead` | `:46` "**incremental mode** … you select the mode and supply the {new design}/{prior analysis} sets" — **Phase 2 (Design loop)** |
| 5 | team casting (analyst producer; PA feasibility; advocate completeness) | `moved-to-lead` | `:8`, `:19` (full roster + disjoint skills) |
| 6 | plan.md assembly (P10 = fill-target) | `moved-to-lead` | Phase 3 `:51`–`:53` "assemble … from `templates/plan-template.md` … the lead's fill-target" |
| 7 | done-condition params (filled into contract, not inlined) | `moved-to-lead` | `:14`–`:18` "fill the artifact — don't inline it" |
| 8 | constitution prerequisite (handoff edge) | `moved-to-lead` | `:28` "Read `.mochiko/memory/constitution.md` … Never auto-resolve" |
| 9 | entry gate (spec.md present+accepted, workspace evidence) | `moved-to-lead` | `:29` "workspace evidence — there is no context-file `status` to read" |
| 10 | brownfield check (codebase-analysis.md, >14d mtime, greenfield bypass) | `moved-to-lead` | `:30` |
| 11 | @-input recovery (empty `@`-ref → re-enter/proceed) | `moved-to-lead` | `:12`, `:27` (G1) |
| 12 | state recovery / resume (rebound to workspace evidence) | `moved-to-lead` | `:63`–`:78` "no context-file `phase`/`status`" |
| 13 | mid-loop human gates (feasibility · clarification incl. "Research this" · exit-early) | `moved-to-lead` | `:21`, `:49` (G2/G3/G4; "Research this" → native `Explore`) |
| 14 | per-phase dispatch brief (agent-dispatch, not a file field) | `moved-to-lead` | `:36`, `:45` |
| 15 | workspace path registry + 3 report homes | `moved-to-lead` | techanalyst `:36` · feasibility `:37` · advocate `:38`/`:46`; all three `:61` |
| 16 | Clarification Log (in-session) | `moved-to-lead` | `:49` "answers (logged in-session)" |
| 17 | iteration → **bounded** round counting (UPGRADE) | `moved-to-lead` | `:20` "cap **3** … no-progress … kill-switch `PLAN_STOP`" |
| 18 | operational handling (verify-agent-output; failure messaging) | `moved-to-lead` | `:82` "verifying each dispatch actually wrote its expected files … log and ask retry/abort" |

### §C content / path / state-carrier couplings

| Coupling (assess §C) | Realized | Evidence |
|---|---|---|
| `.humaninloop/` → `.mochiko/` (constitution, codebase-analysis) | `kept-but-rebind` | `:28`, `:30` |
| `specs/…` + `${CLAUDE_PLUGIN_ROOT}/templates/…` → `.mochiko/specs/<feature>/` + `templates/` | `kept-but-rebind` | `:16`, `:27`, `:36`, `:45`; round reports survive as techanalyst/feasibility/advocate |
| entry-gate `.workflow/context.md` `status` read → workspace evidence | `kept-but-rebind` | `:29` (rebound, not transliterated) |
| `.workflow/plan-context.md` state-carrier | `dropped + reason` | **absent**; workspace-as-state + in-session (P11 absorb) |
| inline `Task()`/`AskUserQuestion()` payloads + `supervisor_instructions` prose | `dropped + reason` | **absent** (token scan clean) |
| LLM-judged "no hard caps" iteration counter | `dropped + reason` | replaced by deterministic cap 3 `:20`; drop explained `:23` |

### §D producer-side content → `moved-to-other-cluster` (skills, NOT the command body)

Correctly **absent** from the thin command; homed on producer/validator skills per reconcile §JOB 2D. The
command references only the *outputs* (e.g. "entity summary with sensitivity", "endpoint summary with
integrations", `:53`), never the authoring procedure:

| Content | Realized home | Command carries it? |
|---|---|---|
| IP-XXX infrastructure planning | `moved-to-other-cluster` → P5 `authoring-technical-requirements` | no (correct) — output ref only, `:36` |
| data-sensitivity taxonomy | `moved-to-other-cluster` → P7 `patterns-entity-modeling` | no — output ref "with sensitivity" `:53` |
| x-integration boundaries | `moved-to-other-cluster` → P8 `patterns-api-contracts` | no — output ref "with integrations" `:53` |
| advocate focus-area checklists | `moved-to-other-cluster` → P9 `validation-plan-artifacts` | no — mounted on advocate `:19` |
| cross-artifact feasibility/contradiction checks | `moved-to-other-cluster` → NEW `validation-feasibility` | no — mounted on PA `:19` |

### §E the FOUR gates HIL lacked → ADDED (requirement `dedupe`; params/placement `moved-to-lead`)

| Gate | Realized | Evidence |
|---|---|---|
| default-FAIL done-condition | requirement `dedupe` + params `moved-to-lead` | `:18` (starts FAILing, 4 clauses, "never done"); `:34`/`:43` "FAIL until proven" |
| lead-OWNED verdict (reviewer status = input) | doctrine `dedupe` + reversal note `moved-to-lead` | `:8` "you own the clearing verdict — their status is input, never the gate"; `:39`/`:47` "Verdict (you). Read …" |
| hard bound + kill-switch | requirements `dedupe` + params `moved-to-lead` | `:20` (cap 3 · no-progress · `PLAN_STOP`) |
| **NEW** human acceptance gate on plan.md | named gate `dedupe` + placement `moved-to-lead` | Phase 4 `:55`–`:57` (G5: accept→done / amend→bounded re-enter / reject→abort); done-condition clause (4) `:18` |

The new G5 does **not** displace the existing mid-loop gates (§B #13) — both survive (`:21` lists G1–G5;
`:49` "None of these ends the loop on its own").

---

## Convention-wiring pass (all six)

1. **Classification** — user-invoked command: frontmatter `disable-model-invocation: true` (`:3`). (specify/setup
   rely on the default; plan marks it explicitly — both are user-invoked; the explicit flag is not a defect.)
   No `skills:` list (a command is the lead, not a dispatched agent).
2. **Router registration** — **NOT applied in this artifact; DEFERRED to the final wiring pass.** The shared
   router (`plugins/mochiko/skills/mochiko/SKILL.md`) currently has Doctrine / Transformer / Setup / Specify
   clusters and no **Plan cluster** block, and the Entry-point table lists `transform-cluster` / `setup` /
   `specify` but **not `/mochiko:plan`**. Consistent with the cluster-wide deferral recorded in
   `transform-technical-analyst.md` (plugin.json + router owned by the final pass). `verify-output` will flag
   this as a Part-A discoverability item for the final pass to close — see **Wiring residual** below.
3. **Trigger phrasing** — n/a for a user-invoked command; `description` (`:2`) is a work-context summary that
   doubles as the router hint (analysis→design plan · producer→two-reviewer loop · human gate · default-FAIL ·
   bounded · kernel-free). No false auto-trigger expectation.
4. **Path rebinding** — `.humaninloop/`→`.mochiko/` (`:28`,`:30`); `specs/`→`.mochiko/specs/<feature>/`
   (`:16`,`:27`,`:36`); `${CLAUDE_PLUGIN_ROOT}/templates/…`→`templates/…` (`:16`,`:53`); kill-switch
   `.mochiko/specs/<feature>/PLAN_STOP` (`:20`). No catalog/MCP/DAG paths remain.
5. **Decouple persona/skill** — n/a in the persona sense (a command has no persona). A command lead
   **legitimately names the agents it casts** (`technical-analyst`, `principal-architect`, `devils-advocate`
   at `:8`/`:19`) — this is team casting, not the sibling-coupling the decouple scan forbids in *personas*.
   `:82` "always dispatch via the Task tool (never inline agent behavior)" holds the no-transliteration line.
6. **Single-source / de-duplicate** — references `loop-discipline` (`:10`,`:16`,`:39`,`:47`,`:82`) +
   `workflow-contract` (`:16` "Fill `templates/workflow-contract.md` → `.mochiko/specs/<feature>/plan-contract.md`") +
   `agent-dispatch` (`:10`,`:36`,`:45`,`:82`). Fills a **contract artifact** at runtime; never inlines the four
   rules, validator tiers, gap-routing, or a filled contract. **The altitude floor is held.**

---

## Realized responsibility trace (assess tag → realized tag; no silent loss)

| # | Responsibility (assess §) | Assess tag | Realized tag | Evidence |
|---|---|---|---|---|
| 1 | iteration structure / four guards / independence / tiers / tamper-proof / gap-routing / anti-rationalization / briefing / git-footer (§A) | `dedupe` | **`dedupe`** | referenced, not restated — §A table above |
| 2 | 2-phase analysis→design sequence (§B) | `moved-to-lead` | **`moved-to-lead`** | `:6`,`:32`,`:41` |
| 3 | feasibility-once-then-advocate ordering + rationale (§B) | `moved-to-lead` | **`moved-to-lead`** | `:34`,`:37` |
| 4 | skip-architect-unless-structural routing (§B) | `moved-to-lead` | **`moved-to-lead`** | `:39` |
| 5 | incremental review mode-selection (§B) | `moved-to-lead` | **`moved-to-lead`** | `:46` (Phase 2) |
| 6 | team casting (§B) | `moved-to-lead` | **`moved-to-lead`** | `:8`,`:19` |
| 7 | done-condition params → contract (§B) | `moved-to-lead` | **`moved-to-lead`** | `:14`–`:18` |
| 8 | plan.md assembly (§B) | `moved-to-lead` | **`moved-to-lead`** | `:51`–`:53` |
| 9 | constitution / entry / brownfield prerequisites (§B) | `moved-to-lead` | **`moved-to-lead`** | `:28`,`:29`,`:30` |
| 10 | @-input recovery (§B) | `moved-to-lead` | **`moved-to-lead`** | `:12`,`:27` |
| 11 | state recovery / resume (rebound) (§B) | `moved-to-lead` | **`moved-to-lead`** | `:63`–`:78` |
| 12 | mid-loop human gates (feasibility · clarification incl. "Research this" · exit-early) (§B) | `moved-to-lead` | **`moved-to-lead`** | `:21`,`:49` |
| 13 | operational handling (verify-agent-output; failure) (§B) | `moved-to-lead` | **`moved-to-lead`** | `:82` |
| 14 | `.humaninloop/`→`.mochiko/`; specs/+templates; entry-gate status→evidence (§C) | `kept-but-rebind` | **`kept-but-rebind`** | `:16`,`:28`,`:29`,`:30`,`:36` |
| 15 | plan-context.md carrier; inline payloads + supervisor_instructions; LLM "no hard caps" (§C) | `dropped + reason` | **`dropped + reason`** | absent; `:20`,`:23` (upgrade) |
| 16 | IP-XXX / data-sensitivity / x-integration / advocate checklists / feasibility checks (§D) | `moved-to-other-cluster` | **`moved-to-other-cluster`** | P5/P7/P8/P9/`validation-feasibility`; output-refs only in command (`:36`,`:53`) |
| 17 | ADD default-FAIL done-condition (§E) | `dedupe`+`moved-to-lead` | **realized (added)** | `:18`,`:34`,`:43` |
| 18 | ADD lead-owned verdict (status=input) (§E) | `dedupe`+`moved-to-lead` | **realized (added)** | `:8`,`:39`,`:47` |
| 19 | ADD hard bound + kill-switch (§E) | `dedupe`+`moved-to-lead` | **realized (added)** | `:20` |
| 20 | ADD human acceptance gate on plan.md (§E) | `dedupe`+`moved-to-lead` | **realized (added)** | `:55`–`:57` (G5) |

**Reconcile relational verdicts affecting P1 (all realized):** RQ1 → (i) two distinct validators
(`:8`,`:19`); P11 rehome → `absorb-into-lead`, three report homes (techanalyst/feasibility/advocate,
`:36`–`:38`,`:61`); producer↔validator casting settled (producer=technical-analyst, validators=PA+advocate,
lead=command).

**No responsibility dropped without a reason.** Every assess responsibility carries a realized tag → transform
done-condition met. The four §E gates HIL lacked are present.

---

## Independence attestation (re-confirmed from the artifact; non-negotiable)

Producer `technical-analyst` mounts authoring/patterns skills only (`:19`) — **no grading skill.** Two
independent reviewers both grade the analyst's output: `principal-architect`+`validation-feasibility`
(feasibility) and `devils-advocate`+`validation-plan-artifacts` (completeness) — disjoint agents, disjoint
skills (`:19`). The **lead** (this command) referees and owns the clearing verdict (`:8`). No agent
produces+grades the same artifact; producer skills ∩ (both reviewers' skills) = **∅**. The done-condition
requires the lead's own Read, not a self-declared verdict (`:18` clause 3, `:39`, `:47`). Kernel-free
throughout (`:82`).

---

## Line-count / altitude comparison

| Command | Lines | Loop shape |
|---|---|---|
| `specify.md` | 66 | 1 loop · 1 critic · 3 gates |
| `setup.md` | 78 | 1 loop · 1 validator · 4 gates · 3 modes |
| **`plan.md`** | **82** | **2 loops · 2 reviewers · 5 gates (G1–G5)** |

Plan is the longest (+4 over setup, +16 over specify), and the delta is **workflow-specific `moved-to-lead`
content** (the second phase, the feasibility-once-then-advocate ordering, the skip-architect routing, the
incremental mode-selection, the fifth gate) — not restated doctrine. The `dedupe` legs are all references
(§A). Altitude held: length tracks genuine 2-phase/2-reviewer/5-gate complexity, matching the specify/setup
silhouette (identical opening lead paragraph, "Contract parameters (fill the artifact — don't inline it)"
block, "Why this done-condition differs from HIL's" note, State-recovery-from-evidence table, and
"What you own (not the agents)" footer).

## Wiring residual (for the final wiring pass — NOT edited here; NOT a body defect)

Router/manifest registration is owned by the final wiring pass (same deferral recorded for
`technical-analyst`). Outstanding for the plan cluster:
1. **Router `SKILL.md`** — add a **Plan cluster** block and a `/mochiko:plan` **Entry-point** row
   (`you want to create the analysis→design implementation plan via the producer→two-reviewer loop with a
   plan.md acceptance gate`). A command absent from the router fails discoverability (a `verify-output`
   Part-A item).
2. **`plugin.json`** — ensure the plan-cluster agents/commands are registered by the manifest.
This is an external-file registration item, tracked and cluster-consistent — the command **body** at
`plugins/mochiko/commands/plan.md` is complete.

## Handoff

Artifact + this realized trace → `verify-output`, run by a **different** agent (the independent
`validator`). This producer does not grade its own output. Audit anchors are the `plan.md:<line>` references
throughout; the checklist is the five conventions + sound-loop placement + the responsibility-trace audit.

---

## Output block (transform-recipes format)

```
TRANSFORM: plan (command)
Applied:   redesign × absorb-into-lead + wiring-pass
Artifacts: plugins/mochiko/commands/plan.md (written, 82 lines — prior session)
           .mochiko/transform/plan/transform-plan-command.md (this realized trace)
New partners: none created by THIS artifact. The command references validation-feasibility (on
           principal-architect) + validation-plan-artifacts (on devils-advocate); both settled in
           reconcile.md RQ1 option (i) and created by their own transforms (validation-feasibility exists on disk).
Wiring:    classification=user-invoked (disable-model-invocation: true; no skills: list — it's the lead)
           router=NOT-registered (DEFERRED to final wiring pass — no Plan-cluster block / no /mochiko:plan
                  entry row yet; cluster-consistent with transform-technical-analyst.md; verify-output Part-A)
           triggers=n/a (user-invoked command; description = work-context router hint)
           rebinds=.humaninloop/→.mochiko/ · specs/→.mochiko/specs/<feature>/ · CLAUDE_PLUGIN_ROOT/templates→templates/ · PLAN_STOP
           decouple=n/a (no persona); command legitimately casts agents by name; Task-tool dispatch, no inlined behavior
           single-source=loop-discipline + workflow-contract + agent-dispatch REFERENCED; contract FILLED as
                  plan-contract.md artifact; four-rules/tiers/gap-routing/filled-contract NOT inlined (altitude HELD)
Trace (realized):
  - generic loop-discipline (iteration/guards/independence/tiers/tamper/gap-routing/anti-rat/briefing/git) → dedupe (referenced)
  - 2-phase analysis→design sequence                → moved-to-lead (plan.md:6,32,41)
  - feasibility-once-then-advocate ordering          → moved-to-lead (:34,:37)
  - skip-architect-unless-structural routing         → moved-to-lead (:39)
  - incremental review mode-selection                → moved-to-lead (:46, Phase 2)
  - team casting (analyst / PA / advocate)           → moved-to-lead (:8,:19)
  - done-condition params → contract                 → moved-to-lead (:14–:18)
  - plan.md assembly (P10 fill-target)               → moved-to-lead (:51–:53)
  - constitution / entry / brownfield prerequisites  → moved-to-lead (:28,:29,:30)
  - @-input recovery                                 → moved-to-lead (:12,:27)
  - state recovery / resume (rebound to evidence)    → moved-to-lead (:63–:78)
  - mid-loop human gates (feas/clarif+"Research this"/exit) → moved-to-lead (:21,:49)
  - operational handling (verify-output; failure)    → moved-to-lead (:82)
  - .humaninloop/→.mochiko/ · specs+templates · status→evidence → kept-but-rebind (:16,:28–:30,:36)
  - plan-context.md carrier · inline payloads · supervisor_instructions · LLM "no hard caps" → dropped+reason (absent; :20,:23)
  - IP-XXX / DS taxonomy / x-integration / advocate checklists / feasibility checks → moved-to-other-cluster (P5/P7/P8/P9/validation-feasibility)
  - ADD default-FAIL done-condition                  → realized (:18,:34,:43)
  - ADD lead-owned verdict (status=input)            → realized (:8,:39,:47)
  - ADD hard bound + kill-switch                     → realized (:20)
  - ADD human acceptance gate on plan.md (G5)        → realized (:55–:57)
Independence: producer skills ∩ both reviewers' skills = ∅; lead referees; done-condition needs lead's own Read (no self-grade)
Altitude: specify(66) < setup(78) < plan(82); delta = workflow-specific moved-to-lead (2nd phase/2nd reviewer/5th gate), not doctrine
Grade: deferred to verify-output (different agent) — this file is the realized trace it audits
```
