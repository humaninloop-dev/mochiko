# Assessment — `templates/constitution-template.md` (P7)

Run: `setup` cluster transform · Phase 1 (assess) · Producer: `transform-producer`
Source (read-only): `human-in-loop/plugins/humaninloop/templates/constitution-template.md`
Consumed by: `authoring-constitution` (P3) and `brownfield-constitution` (P4) — the artifact shape both skills emit as `constitution.md`.

---

## Step 1 — Branch by class

**Class: template → artifact branch.** What carries weight for a template: placeholders, what consumes it, and path coupling. (Not loop-ownership, not persona/procedure.)

This is the **canonical constitution artifact shape** — a fill-in-the-blanks document with `[BRACKET_PLACEHOLDER]` slots, embedded `<!-- INSTRUCTION -->` comments, and worked examples per section (Core Principles, Technology Stack, Quality Gates, Project Structure, Governance, CLAUDE.md Synchronization, SYNC IMPACT REPORT header). It is **passive content** — read and filled by a producing skill, never executed.

## Step 2 — Fast-path triage gate

1. **Orchestration-coupled?** `gate1 = NO` — the template is consumed-as-content by the constitution-authoring skills; it does **not** depend on a kernel, markdown supervisor, command, or DAG to *function*. It is a static artifact shape. (The skills that fill it are sequenced by the setup supervisor, but that is the skills' coupling, not the template's.) No `humaninloop_brain` / `hil-dag` / MCP / catalog / DAG content anywhere in the body.
2. **Multi-responsibility / fans out?** `gate2 = YES` — (a) it **feeds two consumers**: `authoring-constitution` (greenfield) and `brownfield-constitution` (brownfield) both emit `constitution.md` from this one shape; (b) it **bundles a distinct cross-cutting responsibility** — a full `## CLAUDE.md Synchronization` section + SYNC IMPACT REPORT header, whose *operational* owner is `syncing-claude-md` (cross-cutting, not ported this run).
3. **Emits a non-machine-checkable artifact?** `gate3 = YES` — the `constitution.md` this template shapes is judgment-laden (principles, enforcement, testability, rationale, RFC-2119 phrasing). Its correctness is graded by `validation-constitution` (P5), not by a schema or version-equality assert.

gate2 + gate3 trip → **full lens** (no fast-path).

## Step 3 — The lens (weighted for artifact branch)

**Check 1 — Orchestration test.** Orchestrated as **content**, not driven. Content-coupling to kernel = **NONE**. Orchestration-coupling = none owned by the template itself; the producing skills (P3/P4) own their invocation. Nothing to re-home from the template body. The only mechanical exposure is **path references** (Check 6).

**Check 2 — Role (two altitudes).** Skill-role analog for an artifact = **emits-the-shape-a-producer-fills**. Team-role it confers = it makes its caller a **producer** (author of the constitution). It confers **no validator role** — grading the filled constitution is `validation-constitution`'s job. So this template is *paired* (the producer fills it; the validator grades the result), and that pairing already exists in the cluster (P3/P4 produce, P5 validates).

**Check 3 — Independence.** No produce+grade leak in the template. The independence concern lives one level up — whether the agent that fills this (`principal-architect`) also mounts `validation-constitution` (a P2/P5 self-grade concern, already flagged there). The template is neutral.

**Check 4 — Verdict-sink / loop-driver.** Consumers: P3/P4 fill it → `constitution.md`; P5 grades the filled result; the **setup supervisor** loops on P5's verdict / clarifications (a P1 concern). The template owns no loop and no verdict.

**Check 5 — Sibling / overlap.**
- **Shared by two producers (P3 + P4):** this is **correct reuse of one substrate**, not a `dedupe` — there is a single template mounted once and filled by two skills. No split/merge owed.
- **Overlap with `authoring-constitution` prose (P3):** the SKILL.md restates the mandatory-sections structure (Three-Part Rule, section list) that this template encodes as fill-in slots. That is **procedure (skill) vs artifact-shape (template)** — complementary, not duplicated substrate. No dedupe owed, but note the two must stay in sync.
- **CLAUDE.md-sync content** overlaps with `syncing-claude-md` (cross-cutting, not ported) — see Check 6 / trace R6.

**Check 6 — Coupling audit.**
- **Path reference (rebind):** L225 — `${CLAUDE_PLUGIN_ROOT}/templates/approved-domain-deps.md`. This points at a template that is **NOT in this run's in-scope port set** and **NOT in the cross-cutting moved list** — it exists in HIL (`templates/approved-domain-deps.md`) and is also referenced by P4's EMERGENT-CEILING-PATTERNS.md. → genuine **dependency gap** for the lead (see reconcile flag F-P7-1).
- **No `.humaninloop/` hardcoded path in the body.** The output location (`.humaninloop/memory/constitution.md`) is bound by the *consumer* (supervisor / skill), not the template — so the template body carries no `.humaninloop/` string to rebind.
- **Project-relative paths (fine, not plugin paths):** `docs/constitution-exceptions.md` (Exception Registry) — project convention, left as-is.
- **Determinism boundary:** none — pure document shape, all model-judgment to fill.
- **Placeholder convention:** `[UPPER_BRACKET]` + `<!-- INSTRUCTION -->` comments. (Differs from `codebase-analysis-template.md`'s `{{mustache}}` style — a cross-template convention inconsistency noted at cluster level, not a blocker.)

**Check 7 — Conventions + loop placement.**
- **Classification:** templates are inert artifacts; they carry no `disable-model-invocation`. The convention-wiring floor still applies: **router registration** (as a hinted template entry) + **path rebinding**. No trigger phrasing (not a skill).
- **Loop placement:** the template supplies the **done-condition surface** a validator checks against (the mandatory sections + "no `[PLACEHOLDER]` left" rule). It does not itself supply independent validation or a human gate — those live on P5 / the supervisor. No loop gap introduced by the template.

## Step 4 — Disposition

**Body × Structural = `keep-verbatim` × `standalone`.**

- **Body = `keep-verbatim`.** The structure is mochiko-clean in substance — no kernel/DAG/catalog, sound RFC-2119 governance content, the CLAUDE.md-sync section is legitimate *constitution content* (it instructs the project to keep CLAUDE.md synced; it is not invoking the `syncing-claude-md` skill). The single path reference (L225) is handled by the **convention-wiring path-rebinding pass** (recorded `kept-but-rebind`), not a body redesign. Minimalism governor: nothing in the body *earns* an edit beyond the mechanical rebind → `keep-verbatim`, not `port-with-edits`.
- **Structural = `standalone`.** One home (`plugins/mochiko/templates/constitution-template.md`), filled by two sibling skills via correct reuse. No split/merge/promote — the producer↔validator pairing it participates in **already exists** in the cluster (P3/P4 produce, P5 grades). No relational move owed → no structural `flag-for-reconcile`.

## Step 5 — Responsibility trace (every responsibility tagged)

| # | Responsibility | Tag |
|---|----------------|-----|
| R1 | Define the canonical constitution artifact shape (Core Principles I–N with Enforcement/Testability/Rationale slots) | **kept** (keep-verbatim) |
| R2 | Technology Stack table shape | **kept** |
| R3 | Quality Gates table shape (+ per-stack example commands in INSTRUCTION comments) | **kept** |
| R4 | Project Structure + Layer Import Rules shape | **kept** |
| R5 | Governance shape (Approvers, Amendment Process, Version Policy, Exception Registry + Process, Compliance Review) | **kept** |
| R6 | `## CLAUDE.md Synchronization` section + Mandatory Sync Artifacts table (as *constitution content*) | **kept**; the *operational* sync action this describes → **moved-to-other-cluster** (`syncing-claude-md`, cross-cutting, not ported this run — descriptive section stays; the skill that acts on it is out of scope) |
| R7 | SYNC IMPACT REPORT header convention (versioning audit trail) | **kept** (shared with P3's SYNC-IMPACT-FORMAT ref; complementary, keep in sync) |
| R8 | `${CLAUDE_PLUGIN_ROOT}/templates/approved-domain-deps.md` reference (Domain Layer Note, L225) | **kept-but-rebind** → mochiko templates dir — BUT the referenced template is **not in this run's port set** → dependency gap (F-P7-1) |
| R9 | Final version/ratification footer (`[CONSTITUTION_VERSION]` / `[RATIFICATION_DATE]` / `[LAST_AMENDED_DATE]`) | **kept** |
| R10 | Discoverability / placement (router registration as hinted template entry; path rebind) | **kept-but-rebind** (convention-wiring floor) |

No responsibility left untagged; no silent drop.

## Reconcile flags

- **F-P7-1 (soft, lead/coupling) — dangling `approved-domain-deps.md` reference.** The template (L225) references `${CLAUDE_PLUGIN_ROOT}/templates/approved-domain-deps.md`, which is **not** enumerated in this run's in-scope port set and **not** in the cross-cutting moved list. P4 (`brownfield-constitution`) references it too. The lead must accept one of: (a) port `approved-domain-deps.md` alongside this run, (b) drop the Domain-Layer-Note reference, or (c) rebind to a documented future location. The template's own disposition (`keep-verbatim × standalone`) stands regardless; this is a wiring/coupling decision, not a structural move on P7.

**No blocking structural flag.** The producer↔validator pairing this template participates in already exists in the cluster (P3/P4 ↔ P5). Disposition is decided.

---

### Summary block

```
ASSESSMENT: constitution-template.md (P7)
Class:        template → branch artifact (canonical constitution shape; consumed by P3 + P4)
Triage:       gate1=n gate2=y gate3=y  [full-lens]
Disposition:  keep-verbatim × standalone
Trace:
  - R1 constitution artifact shape (principles I–N w/ Enforcement/Testability/Rationale) → kept
  - R2 Technology Stack table shape                                                      → kept
  - R3 Quality Gates table shape                                                         → kept
  - R4 Project Structure + Layer Import Rules shape                                       → kept
  - R5 Governance shape (amendment/version/exception/compliance)                          → kept
  - R6 CLAUDE.md Synchronization section (as constitution content)                        → kept; operational sync → moved-to-other-cluster (syncing-claude-md)
  - R7 SYNC IMPACT REPORT header convention                                               → kept
  - R8 approved-domain-deps.md reference (L225)                                           → kept-but-rebind (referenced template not in scope → F-P7-1)
  - R9 version/ratification footer                                                        → kept
  - R10 discoverability/placement (router reg, path rebind)                               → kept-but-rebind (wiring floor)
Reconcile flags:
  - F-P7-1 (soft) dangling approved-domain-deps.md reference (not in port set) — lead to accept port/drop/rebind; also referenced by P4
```
