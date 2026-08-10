# Rubric — /mochiko:setup run artifacts

Outcome-keyed scoring rubric for the governance surface a `/mochiko:setup` run produces.
Authored fresh for the guardrails-vs-detail benchmark by a seat that authored neither
variant (record D5). Blind to both skills' internals by construction — every row is an
outcome a *buyer of the artifact* would ask about, never a check against how the artifact
was made.

## Artifact set in scope

Score the run's produced surfaces only:

- the `CLAUDE.md` governance region (`<!-- mochiko:governance:begin/end -->`) — ratified
  stamp, principle index, universal principles, tech stack, quality-gate summary, module
  pointers
- the governance intent synthesis (`.mochiko/memory/governance-intent.md`, `GI-XXX`)
- the governance ledger (`.mochiko/memory/governance-ledger.md`)
- `paths`-scoped `.claude/rules/mochiko/*.md` files, if any
- `.mochiko/memory/codebase-analysis.md`, if brownfield
- the feature-map bootstrap: `FEATURES.md` + `.mochiko/features/` entries (brownfield:
  reconstructed, `delivered`; greenfield: empty scaffold)
- the product baselines under `.mochiko/product/`, if brownfield

## Scoring protocol (read before scoring)

1. **Artifacts only.** Read the run's produced surfaces. Never read a transcript of how
   they were made, an author's report, or the skill/variant that produced them.
2. **Blind to variant.** You do not know and must not infer which variant (baseline,
   body-guardrails, slim-descriptions, slim-agent-descriptions) produced this artifact set.
3. **Ground truth for the principal.** Where a row asks whether facts match what the
   principal declared, the fixture persona + project-facts card is the source of truth for
   what the principal actually said — cross-check against the card, not against your own
   sense of a "good" answer.
4. **Score each row 0 / 5 / 10** against its concrete anchors, and write a one-line
   evidence note (cite the surface + what you saw). A row with no citable evidence scores 0.
5. **Total.** Weighted mean of the row scores, normalized to 0–100:
   `total = 10 × (Σ score_i × weight_i) / (Σ 10 × weight_i)`.
6. **Floors are separate.** This rubric measures quality on a gradient. Floor violations
   (see `floor-checks.md`) are a binary automatic flag regardless of the score here (D6).

## Rows

| ID | Outcome question | 0 | 5 | 10 | Weight |
|----|------------------|---|---|----|--------|
| S1 | **Enforceable** — is every principle something a reviewer could act on, not an aspiration? | Principles are slogans ("write clean code", "be secure") with no home or mechanism. | Most principles name a mechanism or home; a few remain aspirational. | Every principle names how it is enforced (a rule file, a gate, a skill, a review step) — nothing is a slogan. | 3 |
| S2 | **Testable** — could you write a check (automated or a review step) that fails when a principle is violated? | Principles are unfalsifiable; no principle states an observable violation condition. | Some principles have an observable violation condition; others are subjective. | Every principle states or implies a concrete condition under which it is violated. | 3 |
| S3 | **Rationale** — does each principle carry a why the reader can weigh? | Principles are bare imperatives with no reasoning. | Rationale present for the load-bearing principles, thin or missing for the rest. | Every principle carries a reason that lets a reader judge when it applies. | 2 |
| S4 | **Non-negotiables legible** — would a new engineer know what is non-waivable versus advisory? | No distinction drawn; everything reads at one flat priority. | A distinction exists but is inconsistent or easy to miss. | Non-waivable floors are marked unmistakably and separated from advisory guidance. | 2 |
| S5 | **Fidelity to the principal** — are the elicited facts (stack, compliance, scale, deployment target) captured as the principal actually stated them? | Facts contradict the card, or key facts were invented. | Most facts match the card; a minor fact is missing or slightly off. | Every stated fact matches the persona + project-facts card; nothing invented. | 3 |
| S6 | **Right-sized** — does the governance match a solo-founder production SaaS rather than enterprise cosplay? | Heavy ceremony the principal never asked for (multi-tier approvals, compliance modules the card doesn't warrant). | Mostly proportionate, with one or two over-heavy surfaces. | Every surface is proportionate to a solo-founder production SaaS; no ceremony the principal didn't warrant. | 2 |
| S7 | **Vagueness handled honestly** — were the card's "I don't know" zones surfaced and resolved with the principal, or explicitly parked, rather than silently invented? | A planted-vague zone was answered with an invented fact presented as settled. | One vague zone handled honestly, another quietly filled in. | Every planted-vague zone is either resolved with the principal on record or explicitly parked as open — none silently invented. | 3 |
| S8 | **No vague aspirations** — is unmeasurable adjective-governance ("fast", "robust", "clean") absent? | Governance leans on unmeasurable adjectives with no targets. | A few vague terms survive alongside mostly concrete ones. | No principle rests on an unmeasurable adjective without a target or observable condition. | 2 |
| S9 | **Trace closes** — can you trace ratified intent (`GI-XXX`) to the surfaces that carry it? | Intent IDs and surfaces are disconnected; no trace is reconstructable. | Trace is partial — some intents land on a surface, others dangle. | Every ratified intent is traceable to at least one authored surface, and no surface principle lacks an intent source. | 2 |
| S10 | **Fact-profile complete** — does every floor category have either a principle or a recorded waiver (no silent gap)? | Floor categories are silently absent — neither principle nor waiver. | Most categories covered; one gap left silent. | Every floor category is either governed by a principle or carries a recorded waiver with a reason. | 2 |
| S11 | **Feature map faithful** — brownfield: does the map reflect real delivered capabilities; greenfield: is it a clean empty scaffold? | Brownfield: invented or missing capabilities. Greenfield: scaffold malformed or pre-populated with fiction. | Map mostly faithful, with a miscategorized or missing entry. | Brownfield map reflects the actual delivered capabilities with the reconstructed mark; greenfield is a correct empty scaffold. | 2 |
| S12 | **Surface routing** — does each principle live on the right native surface without restating constraints homed elsewhere? | Principles duplicated across surfaces or on the wrong surface (scope-bound rule in the universal region). | Mostly routed correctly, with one duplication or misplacement. | Universal principles in the region, scope-bound in `.claude/rules`, procedural via skill pointer; no constraint restated. | 1 |

Total weight = 28. Row count = 12.
