# Strip notes — `templates/constitution-modules/release-gates.md`

Entry formats: `strips/README.md`. Filename follows the sibling constitution-module convention
(`knowledge-management-module.md`): `<module>-module.md`, one file per primitive.

**Wave context (v0.77.0 — the template-schema ratchet, D3 later-ratchet).** The
`schema-based-template-guidance` D3 later-ratchet is exercised over the remaining `.md` templates
per the **user ruling 2026-08-16** (recorded at the v0.76.0 landing, against the DM's
scope-breadth recommendation). Mechanism verbatim: the D8 raw-Read data files stay the source of
truth, the binary renders over them, and each converted `.md` takes this strip ceremony. Standing
honesty flag (not voided by contest): the mechanism is still **n=0** — this ratchet extends it
over 15 files before the first-live-run watch resolves, so the M7 rollback surface widens to every
converted file. This module is one of the three **Class C** constitution-module conversions
(verbatim standard, validation-constitution audit). Record:
`.mochiko/brainstorms/schema-based-template-guidance/record.md`; `DECISIONS.md` will carry the
ratchet's own landing row.

## [v0.77.0] Module retired — superseded by schema-based template guidance (D3 later-ratchet)
- **Disposition:** superseded → plugins/mochiko/schemas/release-gates.yaml + mochiko-cli template release-gates
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D3 later-ratchet + user ruling 2026-08-16; record `.mochiko/brainstorms/schema-based-template-guidance/record.md`; `DECISIONS.md` "Template-schema ratchet (D3 later-ratchet exercised)")
- **Content (superseded module, full verbatim below):**

````markdown
<!--
MODULE: release-gates
=====================
Attach when: always offered for the target class — customer-facing software the team deploys
and operates has a real release process by definition (PO-D1), and the deployment-and-release
dimension (always interrogated — no pruning license) supplies its content: environments,
cadence, release-blocking criteria, rollback expectations. Trace: the GI module-selection
element that names `release-gates`.
-->

## Release Gates

<!--
INSTRUCTION: What blocks a release — beyond the per-merge Quality Gates. Source from the
synthesis's deployment-reality element. Use the project's actual environment names and real
verification commands.
-->

**Environments:** [e.g. dev → staging → production, with promotion rules]
**Cadence:** [e.g. on-merge continuous / weekly cut / manual]

| Gate | Requirement | Verified by | Blocks |
|------|-------------|-------------|--------|
| [e.g. Staging soak] | [e.g. 24h error rate < baseline] | [dashboard/command] | promotion to production |
| [e.g. Migration check] | [e.g. reversible migration verified] | [command] | deploy |
| [e.g. Changelog] | [entry present for user-facing change] | PR check | release cut |

### Rollback

- Rollback procedure MUST be documented and executable by [role]: [pointer or inline steps]
- [Rollback time expectation, e.g. "restore previous version in ≤15 minutes"]
- Releases that cannot be rolled back (e.g. destructive migrations) MUST be flagged in the PR and
  approved explicitly

<!-- ── Validator checklist fragment (checked only when this module is attached) ──
- [ ] Environments and cadence stated with the project's real environment names
- [ ] Release-gate table present; every gate has a concrete verification (command/dashboard), no placeholders
- [ ] Rollback procedure documented with a time expectation
- [ ] Gates consistent with the attached compliance modules (an attached module names its audit-evidence gate)
-->
````

- **Section map (I2 — fragment-line-driven).** The module has 4 Validator-checklist-fragment lines; **3** get a producer section, each `check` reproducing its fragment line **verbatim** (confirmed via `mochiko-cli template release-gates --check`): (1) *Environments and cadence*; (2) *Release-gate table*; (3) *Rollback*.
- **Cross-module fragment OMITTED (plan §1 C — this is why section count 3 ≠ fragment count 4).** The 4th fragment line — `Gates consistent with the attached compliance modules (an attached module names its audit-evidence gate)` — is a **cross-module** check: it has no single-module producer home (its truth depends on other attached compliance modules, not on release-gates content). Per the approved plan it is **NOT** encoded as a release-gates section check and **stays in** `validation-constitution/references/QUALITY-CHECKLIST.md` (another seat's surface). It is dropped from this schema deliberately, not by oversight. **HAND-OFF FLAG:** before `release-gates.md` is deleted at phase 2, confirm this line has a live home in `QUALITY-CHECKLIST.md` so the cross-module check is not lost with the file.
- **Section fabrication (disclosed).** The source carries two headings: `## Release Gates` (H2) and `### Rollback` (H3). The *Rollback* section reuses `### Rollback` verbatim (not fabricated). **Two section headings are fabricated:** *Environments and cadence* (a heading grouping the source's `**Environments:**`/`**Cadence:**` bold labels) and *Release-gate table* (the gate-table facet of the `## Release Gates` section, which has no distinct sub-heading of its own).
- **Kept deliberately:** every operative doctrine line carried **verbatim** into the schema — the MODULE header (in `overview`); the `## Release Gates` heading + the `INSTRUCTION` comment + `**Environments:**`/`**Cadence:**` lines (in the *Environments and cadence* contract); the release-gate table (in the *Release-gate table* contract); the `### Rollback` block, all three bullets, in the *Rollback* contract. The 3 non-cross-module fragment lines became the 3 `check` lines verbatim. The `<!--`/`-->` MODULE-comment delimiters were dropped so the Attach-when guidance renders as visible `overview` prose (words unchanged) — matching `governance-surfaces.yaml`. **Shape metadata not in the source:** `form: artifact-format.md` and `register: full` are schema-shape fields the source never declared; set to match the sibling `governance-surfaces.yaml`. **Modeling:** the fillable markdown lives in the three sections' contract fenced blocks (each intact); the `skeleton` is an assembly note that also records the cross-module-line omission for the producer. Nothing removed except the deliberately-omitted cross-module fragment above.
- **Consumers assessed:** referenced by `validation-constitution/SKILL.md:45` (glob), `validation-constitution/references/QUALITY-CHECKLIST.md:61`, and `authoring-constitution/SKILL.md:221` (dir link) — each re-pointed by the re-point seat (P5) to the `--check`/schema for the converted modules (mixed-source: `knowledge-management` stays raw `.md`). `INTERROGATION-AGENDA.md:46` and `commands/setup.md:83` (KM) untouched; the `templates/constitution-modules/` directory survives. Each re-pointed consumer appends its own supersession strip.
- **Source deletion:** at phase 2 only, after the `validation-constitution` (V2) doctrine-preservation audit PASSES **and** the cross-module fragment's `QUALITY-CHECKLIST.md` home is confirmed (hand-off flag above).
