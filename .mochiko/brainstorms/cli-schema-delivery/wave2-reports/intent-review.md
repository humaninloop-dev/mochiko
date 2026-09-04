# AM-2 — cold intent review of the frozen governance synthesis

**Reviewer:** solo cold intent reviewer (`mochiko:review-governance-intent`), spawned by the
blind-map two-message protocol — message one carried the topic and goal line only; the synthesis
path, the ledger, and the driver record were withheld until the 37-angle map returned.
**Artifact reviewed:** `.mochiko/memory/governance-intent.md` (frozen, AM-2 folded in place).
**Date:** 2026-09-04. **Single writer of this file.**

**Status recommended: `critical-gaps`.**
**Tally: 37 angles raised → 14 survived** (2 Critical · 9 Important · 3 Minor). The 23 killed
angles are listed with their reasons at the end and stay retrievable on ask.

The verdict is the lead's; this report is input. Nothing here was authored into, or removed from,
any governance surface.

---

## 1 — The Phase 0 blind angle map

Built before the synthesis path was known, from the amend topic and goal line plus the surfaces
the fence allowed: `CLAUDE.md`, `.claude/rules/mochiko/*.md`, `crates/mochiko-cli/`,
`migrations/`, `evals/contract/`, `.github/workflows/`, `plugins/mochiko/`, both manifests,
`README.md`, and the interrogation agenda as the coverage yardstick.

**A — Must cover (amend-slice agenda coverage).**
1. Fact profile: does a required external binary plus public distribution move it?
2. Risk surface: the new top failure is a plugin installed with no binary; the first-public-release
   revisit must discharge with real controls.
3. Existing practices: the re-expressed commands must be commands that exist.
4. Deployment reality: two release trains now, and the compat contract between them.
5. Deliberate exclusions: whatever the retired Python checkers covered and the log does not.
6. The amend's once-offer sweep over modules carrying no ruling.
7. Depth level re-recorded or explicitly unchanged.
8. A transition clause with an end condition, not an open one.

**B — Must not contradict.**
9. The bright line's "never gates pipeline progress" versus a loud dependency halt.
10. The bright line's "never holds judgment skills own" versus selecting and rendering rules.
11. Rewrite completeness across every surface carrying the old additive-install claim.
12. The audit-and-strip ceremony owed by roughly fifty primitive re-points.
13. Two protection regimes for protected content: strips, and the log's anchor rule.
14. Pointer-only region: a rewritten non-negotiable that restates the log grammar.
15. The recorded observation that path-scoped rules inject on Read, versus hook delivery.
16. Hooks as control flow versus "never dispatches or sequences".

**C — Must price.**
17. Adoption cost of a hard toolchain dependency for a solo-maintained developer tool.
18. Where the migration log ships, and what that does to install weight.
19. The two-train version contract; grammar range covers log format, not rule content.
20. A release gate that can exit "skipped".
21. The sandbox suite's terms-of-service caveat, re-priced at gate promotion.
22. The genesis fidelity test's coupling to the shipped corpus.
23. Each retired checker's checks mapped to a replacing constraint.
24. Halt blast radius on the skill path, which fires outside commands.
25. A maintainer break-glass when the binary is broken.

**D — Must trace.**
26. Six changes, six traceable elements with marks.
27. The MAJOR bump traced to the policy rule that makes it MAJOR.
28. A marks audit: every `Contested` audited, every `Confident` earned.
29. The no-feature-map ruling traced to a fact, not a preference.
30. Each named risk control located in a real surface.

**E — What a skeptical reviewer would refuse to ratify.**
31. A non-negotiable hollowed out but keeping its name.
32. A bright-line widening asserted rather than argued.
33. No reversal condition on a one-way move.
34. An open-ended transition clause.
35. Gates that cannot run as described.
36. Silence on the submodule precedent, whose recorded reason was install burden.
37. Thinness: a ruling without rationale, a mark without basis.

---

## 2 — Survivors

### Critical

#### C1 — The log is not in the plugin. The claim that it is, is false of the tree; the ruling that would make it true is unrecorded.

**Elements:** GI-020 (AM-2 intent), GI-002 (AM-2 identity).

The synthesis states, as accomplished fact, that `mochiko-cli` "serves every command's and skill's
rules from a migration log **shipped in the plugin**, replayed in memory at fire" (GI-002, AM-2),
and GI-020's intent rests on it. It is not shipped, and nothing rules where it will be.

Measured, this session:

| check | result |
|---|---|
| installed plugin root, v0.99.0 | `.claude-plugin agents commands output-styles schemas skills templates` |
| installed plugin root, v0.103.0 | `agents commands output-styles schemas skills templates` |
| `migrations/` present in either | no |
| binary against the real installed plugin root | `grammar-parse · - · - · …/0.103.0/migrations: the migration log cannot be read: No such file or directory` |
| binary with no flags, cwd outside the repo | exit 1, same message |
| how the contract suite supplies the log | `MOCHIKO_MIGRATIONS=<repo>/migrations` (`evals/contract/run.py`) |

A marketplace install carries the contents of `plugins/mochiko/` and nothing above it. The log
lives at the repository root, outside the shipped subtree. The contract fixture never exercises a
plugin-shipped log; it points the binary at the repository's own directory through an environment
variable, which no consumer will have set.

Two things follow, and the second is the finding.

First, the amended text would be false on ratification day. The record's own A-C2 fold ruled that
"the amended text must be true of the tree on the day it is ratified", and minted the transition
clause for exactly that purpose. The clause covers primitives not yet re-pointed. It does not
cover the log's absence from the plugin.

Second, and larger: the unruled question is the one GI-020 exists to govern. Shipping the log
inside the plugin means shipping a 598,626-byte generated file that is re-generated on every
schema change, into the artifact whose install weight this non-negotiable protects. Embedding it
in the binary instead breaks the plugin-version-to-log correspondence the version contract assumes.
Requiring the user to set an environment variable is a third install step. The driver record rules
the end state ("the plugin ships the migration log only", D9 wave 6) and never rules the mechanism
or prices it. An amend that rewrites the additive-install non-negotiable while leaving the install
weight of its own replacement unruled has skipped its central question.

**Resolution path.** One question to the user: where does the log ship — `plugins/mochiko/migrations/`,
embedded in the binary, or supplied by the environment — and what does that add to the install?
Until it is answered, re-word GI-002/GI-020 from the present tense to the ruled end state.

#### C2 — The synthesis marks Confident what the driver record marks Contested and Assumed.

**Elements:** GI-020, GI-002 (AM-2 risk and identity).

The distribution decision the whole amendment rests on is `Contested` in the record, with its
rationale explicitly inferred:

> **D4 — Distribution … — `Contested`** *(reasons inferred, `Assumed`; re-affirmed at Q13 with the
> fourth road on record)* … "the lead recommended committed prebuilt binaries … the user chose the
> standalone install. The user stated no reason at Q5; the lead's inferred reasons, marked
> `Assumed` until the user corrects them".

The record's Evidence-honesty section repeats it: "**D4's rationale is inferred**, not stated by
the user". D10 separately marks "transition-clause validity `Assumed` until the wave-2 validator
grades it".

The synthesis carries **Confident** on GI-020 and **Confident** on GI-002's AM-2 identity and risk
paragraphs. Neither the contest, nor the choice-against-recommendation, nor the `Assumed` transition
limb appears anywhere in the file.

This is not a formatting nit. The mark is the contract on the producer and the signal the user
ratifies against. Presented as written, the ratifier sees a settled, confident decision where the
record holds a contested one whose reasons nobody has stated. The session demonstrably knows how to
mark honestly — GI-019 carries "Confident on the admission; the argument `Assumed` until this run's
validator grades it" — which makes the flat Confident on GI-020 a departure, not a convention.

**Resolution path.** Restate GI-020's mark as a split mirroring the record: the ruling Confident,
the transition clause Assumed, the distribution basis Contested with reasons inferred. Same for
GI-002's AM-2 paragraphs.

### Important

#### I1 — GI-020's Testability rows are not assertable on ratification day.

The Pass row asserts that a fresh install plus the tool install "renders every command's and
skill's rules (contract suite, deterministic set)", and that with the binary absent "every mochiko
fire halts before a model turn". At ratification, zero primitives are re-pointed, so no mochiko
fire calls the binary at all. The contract suite's own README states the position plainly: both
wave-1 cases "are failure paths, because a success path needs a converted primitive and none exists
yet (the pilot is wave 3)", and one assertion is "reported as pending on every run rather than
passed".

The record demanded a clause that makes the principle true on ratification day. The clause delivers
that for the prose statement and not for the Testability rows, which are what the validator grades.

**Resolution path.** Scope the rows to what is assertable during the transition and state the
end-state rows as dormant until wave 3 — the same dormant-clause idiom GI-012 used successfully at
AM-1.

#### I2 — GI-004 is asserted unchanged while the driver record re-keys the audit unit under it.

The synthesis says "GI-004's audit ratchet for markdown primitives is unchanged". The record
changes it in three places:

- D6 names a **new steady-state audit unit**: a schema migration is graded on the migration file
  plus the regenerated view diff, against five named criteria.
- D6 collapses scaffold criterion 2 and criterion 3's **count limb**, moving the survival limb into
  the validator's hard set.
- D3 books an explicit loss: the `.md`'s independent count self-check disappears when the counts
  move into CLI output, "booked as a loss, not a gain".

FLOOR-TEST's translated expression is a ratchet whose baseline "MUST NOT decrease". A change to
what the audit unit is, and the retirement of an independent self-check, are exactly the events
that ratchet exists to catch. The synthesis records none of them.

Related and unruled: the proposal defers the `primitive-edits.md` re-key to wave 6. That file is
GI-004's enforcement home in the ledger, so re-keying it changes GI-004's operative content. The
transition clause's expiry was pre-authorized as a PATCH amendment; this was not, so as written it
will need a second amend run nobody has budgeted.

**Resolution path.** Carry the audit-unit change and the booked loss under GI-004, and rule now
whether the wave-6 ceremony re-key is a pre-authorized activation or a fresh governance event.

#### I3 — GI-005's mechanization is recorded on the wrong principle.

The record rules that "the schema-rule limb of **GI-005** becomes mechanical (the prose-primitive
limb and the dead-pointer scan stay procedural)" (D2). The synthesis puts the migration log on
GI-006 (Card 7) and leaves GI-005 untouched, still reading "protected content leaves only by
recorded ruling (strips/supersession)".

GI-005 is a non-negotiable, and the change is real: protected content in schema rules now leaves
through a migration's `anchor:` field, enforced by the binary, not through a strip entry enforced
by ceremony. Two regimes now exist and the principle that governs protected content names one.

**Resolution path.** Add the schema-rule limb to GI-005, or state why GI-006 alone carries it.

#### I4 — The declared-unsupported set is narrower in the synthesis than in the record.

GI-020 declares unsupported only "environments that disable skill shell execution or hooks by
policy". The record declares one more, and names a consequence the new non-negotiable's own wording
touches:

> Windows served by `cargo install` only — which compiles from source and requires a Rust toolchain
> on the user's machine: **an install-time build step relocated from the plugin to the tool** …
> PowerShell-only Windows is a **declared unsupported platform**.

A declared unsupported platform is a dimension-10 exclusion and belongs in the synthesis. And the
headline "no install-time build step" is true only of the plugin; on Windows the user runs one.

**Resolution path.** Carry the Windows limb into the unsupported declaration, and scope the
no-build-step property to the plugin rather than the install as a whole.

#### I5 — Two of the four named risk controls do not exist in the tree.

The first-public-release trigger is discharged on four controls. Verified against the repository:

| control | status |
|---|---|
| `cargo audit --deny warnings` in CI | present, `.github/workflows/ci.yml` |
| sha256-published release assets | present, `shasum -a 256` in `release.yml` |
| `cargo publish` behind a manual-approval job | rests on a GitHub environment setting, not tree evidence; the job is `if: false` today |
| signed release tags | no signing anywhere in the repo or either workflow |

`ci.yml` separately records that its actions are pinned to release tags and that SHA-pinning is an
open hardening follow-up. Discharging a standing risk trigger is a one-way act; discharging it on
controls that are partly aspirational leaves the trigger closed and the exposure open.

**Resolution path.** Implement the two, or re-word them as wave-2 tail obligations gated on the
first publish, and say which.

#### I6 — The new release gate can pass by being unable to run.

GI-012's gate 6 requires the contract suite's deterministic set green at every `plugin.json` bump.
The suite's exit codes are three-valued, and its own README is explicit that the third is common:

> | 3 | **SKIPPED** — the suite could not run, with the reason printed |

It skips when `sbx`, the sandbox, `claude` inside it, sandbox authentication, the fixture, or
`cargo` is unavailable — and authentication "is the user's own action; the suite never attempts it".
The synthesis makes the suite blocking and never says what a skip means at a bump.

**Resolution path.** Rule a skip blocking (the bump waits) or an ordinary pass with a recorded
reason. Either is fine; the silence is not.

#### I7 — The terms-of-service caveat is not carried into the gate promotion.

The record's amended D8 carries it on the face of the ruling:

> the kinako record marks sandbox subscription auth a `Contested` ruling sustained against adverse
> Terms-of-Service evidence — automated headless use of a consumer subscription may sit outside what
> it permits; the user adopted with that on record.

The contract suite's README repeats it and says why: "this is the file a future maintainer reads
before running the suite". Promotion to a blocking release gate converts an occasional convenience
into a standing obligation to run automated headless sessions on that subscription at every bump.
The synthesis's GI-012 row names the suite as gating and carries none of this — while GI-001
declares "contractual commitments: none".

**Resolution path.** One question: do third-party terms of service fall inside the fact profile's
contractual dimension? Record the answer either way, and carry the caveat onto GI-012, which is
where the recurring obligation now lives.

#### I8 — The crate's release train is ungoverned, and it is now the one consumers depend on.

The Real-commands table names two trains: `plugin.json` bumps, and `mochiko-cli-v*` tags feeding
`release.yml`. GI-012's six gates bind only the first. From ratification, every consumer's rules
delivery depends on a binary shipped by the second, which has no audit, no changelog entry, no
landing ritual, and no compat check.

The version contract does not close this. The grammar range versions the **log**; the `.md` halt
clauses key on the **binary's output shape** — the version-triple head line and the closing end
line — which nothing versions. A CLI release that changed either would halt every installed plugin.
That halt is loud rather than silent, which is the design working; but nothing gates the release
that would cause it.

**Resolution path.** State whether crate releases are gated and by what, or record the asymmetry as
accepted with reasons.

#### I9 — The public front door will contradict the ratified non-negotiable.

`README.md` currently tells every prospective user the opposite of what AM-2 ratifies:

> The install above is complete on its own — the plugin is markdown-only, with **no build step and
> no binary dependency**. … When the binary is absent, agents Read those YAML files raw.

It also documents `--schemas-dir` and a compile-time embedded schema copy; the current command
surface has neither (`cli.rs` resolves `--log-dir`, `--plugin-root`, `MOCHIKO_MIGRATIONS`, then the
working directory). The synthesis's consequence set names CLAUDE.md, the governance region, the
ledger, and `rust-cli.md`, and no non-governance surface.

**Resolution path.** Name `README.md` in the consequence set, or record it as deliberately outside
the amend's scope with an owner and a wave.

### Minor

#### M1 — No reversal condition on a one-way move.

GI-020 is the only element in the file with no revisit trigger, on the change with the highest
reversal cost. The record prices that cost ("after wave 2, reversal costs a second amend run plus
re-pointing every converted `.md`") and mints observable pilot abort criteria — the read-back metric
below its pre-registered bar, or per-invoke read cost above the F3 baseline — that halt waves 4 and
5. Neither reaches the synthesis. **Resolution:** carry D9's pilot abort criteria as GI-020's
revisit trigger.

#### M2 — No maintainer break-glass named.

With no fallback, a binary that is absent, broken, or out of range halts the maintainer's own
mochiko commands in this repository. `cargo install --path crates/mochiko-cli` is the escape and
appears in no governance surface. **Resolution:** one line in `rust-cli.md`.

#### M3 — `cargo audit --deny warnings` is filed under Build in the Real-commands table.

It is a dependency-advisory check, not a build step, and the same command is separately named as a
supply-chain control. Cosmetic; the command itself is real and passing. **Resolution:** move it to
its own row or to Test.

---

## 3 — Killed candidates

Twenty-three of the thirty-seven angles died on the read. Reasons, one line each.

| # | angle | why it died |
|---|---|---|
| 1 | fact profile moved? | stated explicitly: unchanged, no module attaches, public-product trigger stays standing |
| 2 | risk surface elicited | covered in depth — hooks on every consumer machine, access-loss class, controls named (their reality is I5) |
| 3 | real commands exist | verified: `cargo test --all` = 300 passed, exactly as claimed; `migrate validate --log-dir migrations --plugin-root plugins/mochiko` = 0 rejecting / 105 advisory, exit 0; both workflows and `run.py` real |
| 5 | retired checkers' lost coverage | the record's unit-1b accounting names every residual (12 shape errors · 3 without referent · 11 dead under D6 · 1 named); the only real loss folds into I2 |
| 6 | module once-offer sweep | the catalog holds exactly four modules and all four carry rulings (GI-009/010/011 · GI-012 · GI-013 · GI-014); nothing outstanding |
| 7 | depth level | GI-021 carried, `high`, one-way, and restated in the ledger header |
| 8 | open-ended transition | expiry condition stated and its amendment pre-authorized as PATCH |
| 9 | bright line clause 1 | argued at length in GI-019's clause (i) and marked `Assumed` — the confrontation the angle asked for |
| 10 | render versus select | clause (ii) covers structural validity on the tool's own data; the count-pin limb folds into I2 |
| 11 | rewrite completeness | complete across governance surfaces; the non-governance gap is I9 |
| 12 | ~50 primitive edits owing ceremony | the record prices them per-`.md` against the v0.76.0 precedent, and this run edits no primitive |
| 13 | two protection regimes | survives as I3 |
| 14 | pointer-only restatement | the AM-2 text points; trace comments only |
| 15 | rules inject on Read | hooks are a delivery channel, not rules-file injection; no contradiction |
| 16 | hooks as control flow | named, timed at 5 seconds, fail-open, scoped to absence; behavior gating explicitly declined |
| 17 | adoption cost | priced honestly, including the driver attribution at `medium` and the total-loss class |
| 19 | version contract | grammar range covered; the render-contract gap folds into I8 |
| 22 | fidelity coupling | the record keeps a frozen genesis fixture after the YAML sources retire |
| 24 | skill-path halt | the `PreToolUse` matcher on `Skill` covers it and the synthesis names it |
| 26 | six traceable elements | all six present with GI-IDs and elicitation lines |
| 27 | MAJOR bump | matches the ledger's own policy for an incompatible redefinition; user-ruled |
| 29 | no-feature-map ruling | properly minted as GI-022, traced to the amend limb, declined durable |
| 31 | hollowed non-negotiable | the clone-only limb retains real content; the rewrite is not hollow |
| 32 | widening by assertion | the argument is on record with three clauses; see 9 |
| 34 | open transition | see 8 |
| 36 | submodule precedent | clone-only explicitly preserved; the Windows limb survives as I4 |
| 37 | thinness | the synthesis is dense, traced, and carries elicitation lines throughout |

---

## 4 — Recommended status

**`critical-gaps`.**

Two Critical survivors, both on the element carrying the MAJOR bump. C1 is a ruling the amendment
never makes about the very property its non-negotiable governs, on a claim measurement shows is
false of the tree today. C2 is a mark that presents a contested, inference-based decision as a
settled one to the person about to ratify it.

Both are cheap to resolve — one question to the user about where the log ships, and one mark
correction mirroring the record — so the path back is short. Neither should be crossed by a
ratification.

The nine Important survivors are session-resolvable: three re-wordings (I1, I3, I4), two
one-question rulings (I6, I7), two consequence-set additions (I8, I9), and one carry-forward from
the record (I2, with a ruling attached). The three Minor survivors are carry-forwards.

Everything above is a recommendation. The lead owns the verdict and the survivor routing; the user
owns ratification.

---

## Verify pass

Bounded delta-check by the same cold seat, 2026-09-04, over the folded synthesis at
`.mochiko/memory/governance-intent.md` (rewritten in place, superseding the version cold-read
above). Scope: my own 14 survivors only — does each fold match its disposition, and does it
introduce a contradiction with the rest of the synthesis or with the driver record? No fresh
read, no new angles.

| # | Finding | Fold | Verdict |
|---|---|---|---|
| C1 | log not in the plugin, location unruled | GI-002 moved to future tense and states the current reality outright ("today it lives at the repo root and no installed plugin carries it"); GI-020 carries the ruling — `plugins/mochiko/migrations/` from wave 3 — with the weight priced and the two rejected alternatives (embedding, network fetch) named against D5 and the silent-degradation class | **CONFIRMED** (2 nits) |
| C2 | marks upgraded past the record | GI-020's mark now splits four ways: the two rulings Confident, the distribution basis Contested with reasons inferred and Assumed, the transition clause Assumed, the wave-3 rows dormant. GI-002 identity and risk carry matching splits. Mirrors record D4, D10 | **CONFIRMED** |
| I1 | Testability unassertable day one | End-state rows declared dormant until the wave-3 pilot, under the AM-1 dormant-clause idiom, with what *is* assertable named: the suite's absence and skew cases (2/2) and the log's hard set at 0 rejecting | **CONFIRMED** |
| I2 | GI-004 asserted unchanged | New floor note keeps the markdown ratchet and records the schema-content audit unit with D6's five criteria; the retired count self-check is booked as a loss citing D3; the wave-6 re-key ruled a pre-authorized PATCH activation | **CONFIRMED** |
| I3 | GI-005 mechanization on the wrong row | New floor note puts the schema-rule limb on GI-005 and names both regimes, the prose limb and dead-pointer scan staying procedural. Matches record D2 | **CONFIRMED** |
| I4 | unsupported set too narrow | PowerShell-only Windows declared unsupported as a dimension-10 exclusion; the no-build-step property explicitly scoped to the plugin, with the Windows source build stated | **CONFIRMED** |
| I5 | two named controls absent | Discharge made conditional: each of the four controls carries present-or-owed with its evidence, and the trigger stays open until all four exist at the first publish. Stricter than the record, which called signed tags builder's room | **CONFIRMED** (1 nit) |
| I6 | gate can exit skipped | GI-012: "A SKIPPED suite (exit 3) is not green — it blocks the bump" | **CONFIRMED** |
| I7 | ToS caveat uncarried | Carried onto GI-012 as a Contested mark on the gate's substrate, with a matching GI-001 note ruling it a third-party exposure rather than a fact-profile dimension. The question I posed is answered explicitly | **CONFIRMED** |
| I8 | crate release train ungoverned | GI-012 gates `mochiko-cli-v*` tags on the four crate layers, the contract suite against the tagged binary, and an unchanged render output shape or a coordinated plugin bump — naming the exact gap, that the grammar range does not version that shape | **CONFIRMED** |
| I9 | README contradicts | Named in the AM-2 log's consequence set, re-authored at the wave-3 landing, owner stated, and correctly placed outside this run's producer as a non-governance surface | **CONFIRMED** |
| M1 | no reversal condition | GI-020 gains the wave-3 pilot abort criteria as its revisit trigger, with the reversal cost priced. Matches record D9 | **CONFIRMED** |
| M2 | no break-glass | `cargo install --path crates/mochiko-cli` carried into the `rust-cli.md` rewrite via the AM-2 log's scope | **CONFIRMED** |
| M3 | `cargo audit` misfiled | Own row in the Real-commands table, cross-referenced to the risk discharge; the Build row now carries only the build | **CONFIRMED** |

**Nits — wording only, none blocking, none changing a ruling or its conclusion.**

1. **C1's pricing mixes units and counts one file too many.** "534 KB of YAML across 51 files
   today" is 546,803 bytes over **50** schema-class files, expressed in KiB; the 51st `.yaml` under
   the plugin is `plugins/mochiko/skills/patterns-api-contracts/references/OPENAPI-TEMPLATE.yaml`,
   a reference template that carries no schema content and does not enter the log — the log's own
   corpus is 50, matching the genesis migration's 50 import operations and the record's figure.
   "+598 KB at wave 3" is the record's wave-1 acceptance figure; the genesis file is 618,122 bytes
   today. Recomputed in one unit: 534 KiB of corpus, 604 KiB of log, peak ≈ 1,138 KiB — still the
   stated ≈ 1.1 MB — and a wave-6 net of ≈ +70 KiB rather than +64 KB. The ruling and its
   conclusion, that the move is roughly a wash, survive both corrections.
2. **C1's ruling has a producer consequence the input proposal predates.** The proposal's
   `rust-cli.md` rewrite names the log as `migrations/` and scopes its `paths` frontmatter to
   `migrations/**`. With the log ruled to `plugins/mochiko/migrations/` from wave 3, both need the
   new home or the touch-time rule will stop covering the log where it lives.
3. **I5's outcome is stale in one line of the AM-2 log's own Scope bullet**, which still reads
   "first-public-release trigger discharged with named controls". The Dispositions paragraph
   directly below it, and GI-002's body, both carry the conditional discharge correctly; the Scope
   bullet records the deck as ruled before the review and was not swept. A ledger row copied from
   that line would land the wrong status.

No fold misapplied its disposition. No fold contradicts the driver record: I checked C1 against
D9's wave-6 end state, C2 against D4 and D10, I2 against D6 and D3, I3 against D2, I4 against D4,
I5 against D10.4, I7 against the amended D8, I8 against D8's gate split (the tag gate stays
maintainer-side, so CI keeps the crate layers only, as D8 requires), and M1 against D9's abort
criteria. No mark remains upgraded past the record.

**CLEAN.**
