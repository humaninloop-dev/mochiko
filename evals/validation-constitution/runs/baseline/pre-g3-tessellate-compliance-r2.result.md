I validated the drafted governance surface set against the ratified synthesis, the ledger, and the trace summary, and wrote the full result to `validation-result.md`.

**Verdict: FAIL.**

The producer's hand-off note doesn't hold up — I rejected the instruction to skip the anti-pattern check (that's exactly the rationalization the skill warns against) and it surfaced real findings, and the "no waivers, every obligation traced" claim is false. Headline issues:

1. **A PCI-DSS obligation is waived** (GI-012, the quarterly ASV scan) even though the ledger's own text says waiving a legal-mandate module obligation is automatically a validator FAIL. The interrogation's own reviewer caught this at Critical severity and the founder overruled *that* reviewer — not this gate.
2. **The CLAUDE.md ratified stamp omits pci-dss** entirely from its module list, even though the ledger and synthesis both list it.
3. **The cardholder-data rules file only scopes to `src/payments/`**, missing `src/checkout/` — the exact gap the interrogation itself flagged and marked "resolved," but the fix never landed. As written, the rule text even contradicts the known PAN data flow.
4. **A "must write documentation in British English" principle was invented** in direct contradiction of a ratified deliberate exclusion in the synthesis (the founder explicitly ruled that out, citing the team's mixed English).
5. **The one fully-quantified minted principle (2.5s LCP performance budget) was replaced with unenforceable text** ("pages feel quick" / "complaints").
6. A locked coverage threshold (60% blocking, "no session override") drifted to 70% in the produced set.
7. A universal security principle is routed through a `**/*`-scoped rules file instead of the always-loaded region, risking under-injection.

Plus two moderate findings (GDPR enforcement only names one of two established regimes; Quality Gates duplicates Principles text) and three advisory notes. `docs/engineering-norms.md` has no conflicts with the governance region and needs no changes.