# The 7-check assessment lens (detail)

Apply each check, weighted by the Step-1 branch (IS-a-loop vs PLAYS-a-role vs artifact). For each, record a finding and, where relevant, the disposition signal or trace tag it produces.

---

## 1. Orchestration test (was: "Brain test")

**Question:** What layer orchestrates this primitive — a Python/MCP/DAG kernel, a *markdown supervisor* (a `commands/*.md` file driven by the main session), or a command/agent — and who inherits each responsibility when that layer dissolves?

**Why it was reframed:** the original "brain test" assumed a kernel. The HIL `setup` cluster has **no kernel** — it is a markdown supervisor (`commands/setup.md`); only `specify`/`plan`/`implement` are DAG-based. Asking "what breaks if the brain is removed?" returned "nothing" for setup and hid the real work: re-homing the supervisor's loop, validation, and gates.

**Separate two kinds of coupling:**
- **Content-coupling** — the primitive's body references kernel/DAG concepts, catalog JSON, MCP tools. Body work (`redesign`/`port-with-edits`).
- **Orchestration-coupling** — the primitive only works because something *else* drives it (a supervisor sequences it, loops on its output, feeds it inputs). Structural/cluster work (`rewire-cluster`, rehome-orchestration in `reconcile-cluster`).

**Output:** which layer orchestrates; a content vs orchestration coupling split; the list of orchestration responsibilities that must be re-homed.

---

## 2. Role at two altitudes

**Question (skill-role):** Is this a *consumed procedure* (a caller runs it as a step) or does it *emit a reviewable artifact*? Only artifact-emitters need a producer↔validator partner.

**Question (team-role):** What role does consuming this primitive confer on its caller — producer, validator, referee, or lead? A consumed skill can make its *caller* a producer; a checklist skill can make its caller a validator.

**Why two altitudes:** a primitive's own nature and the role it pushes onto its caller are different facts. `validation-constitution` is a checklist skill (skill-role) that turns whoever runs it into a validator (team-role) — that team-role is what makes it `promote` material.

**Output:** skill-role; conferred team-role; whether it emits a reviewable artifact (→ needs a validator partner).

---

## 3. Independence

**Question:** Does one agent both produce an artifact and grade it? Where is the producer/validator boundary, and is it currently violated?

**Where the leak hides:** usually in an **agent's `skills:` list**, not in the skill. `principal-architect` mounting both `authoring-constitution` and `validation-constitution` means the same agent writes and grades the constitution — a self-grade leak invisible if you only read `SKILL.md`.

**This is cluster-scoped.** A single skill in isolation looks fine; the violation appears only when you see which agent mounts it alongside what. Frequently produces a `flag-for-reconcile` (the fix is `split` the agent or `moved-to-validator`).

**Output:** any self-grade leak; the agent/skill where it lives; the independence fix signal.

---

## 4. Verdict-sink / loop-driver

**Question:** Who consumes this primitive's output, and what loops when that output is FAIL?

**Why it matters:** the verdict-sink and the FAIL-loop were the biggest things a kernel or supervisor owned. When you shed/rehome the orchestrator, these responsibilities are the ones most often dropped silently. Naming them is how they get re-homed onto the Workflow/lead in `reconcile-cluster`.

**Output:** the consumer(s) of the output; what currently loops on FAIL; where that loop-driving must re-home.

---

## 5. Sibling / overlap ("look sideways")

**Question:** Does this primitive share a core, a validator, or trigger phrases with siblings in the cluster?

**Signals:**
- **Shared core + thin variant** → `merge-into-sibling` (e.g. `brownfield-constitution` is mostly `authoring-constitution`).
- **Trigger collision** → siblings whose `description` triggers overlap will misfire; de-collide (a convention-wiring fix) or merge.
- **Shared validator** → a `dedupe` candidate.

**This almost always becomes a `flag-for-reconcile`** — you cannot decide a relational move while looking at one primitive.

**Output:** sibling relationships; merge/split/dedupe signals → reconcile flags.

---

## 6. Coupling audit

**Question:** What concrete couplings will break on port?

- **Hardcoded paths** — `.humaninloop/...`, catalog paths, script paths → `kept-but-rebind`.
- **Upstream prerequisites / handoffs** — does it assume another primitive ran first and left an artifact?
- **Determinism boundary** — which parts are a deterministic script (keep as a deterministic assert) vs model judgment (needs grounded validation)? This decides whether a validator partner is real or degenerate.

**Output:** path rebindings; prerequisite edges; the determinism boundary line.

---

## 7. Conventions + loop placement

**Question:** Against the five conventions and the sound-loop, what is present and what is missing?

- **Classification** — user-invoked or model-invoked? (assign if absent)
- **Discoverability** — is it (or should it be) in the router?
- **Reliable model-invocation** — graded, exact-phrase triggers in `description`? (agent-consumed skills describe work-context, not "when user says")
- **Agent↔skill composition** — persona-vs-procedure split clean? **Decoupled?** Grep the deny-list (sibling-agent names, `dispatch`, `workflow-agnostic`/independence-by-declaration meta-labels, injected workflow modes/paths/phases in a persona); independence stated by *role*, not by an agent name. Keystone-test the survivors: true-of-this-professional-on-any-job stays, only-in-one-workflow is coupling to cut.
- **Producer↔validator pairing** — does its artifact have an independent validator (different agent + different skill), guaranteed structurally rather than declared in a persona?
- **Sound-loop** — does the loop it sits in have a done-condition / independent validation / human gate?

**Output:** present/missing per convention; the loop gaps to fill (these feed `reconcile-cluster`'s rehome-orchestration).
