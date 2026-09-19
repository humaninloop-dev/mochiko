The review is written to `review.md`. Verdict: **critical-gaps**.

Round 1's three Important fixes did land cleanly (pagination, the US-3 tariff card, the Reading example). But the round-2 re-issue surfaced a much bigger problem: **`constraints-and-decisions.md`'s new D-004/C-005 ("gateway can only be pulled hourly, no push exists") flatly contradicts the signed store delta**, which built a whole dedicated `meter-ingest` container around the gateway pushing readings, and contradicts the re-issued contract's own push-shaped endpoint and Cycle 1's push-simulation test. The signed 5-minute NFR target is also unmeetable under the new pull decision. On top of that:

- **Cycle 4's TEST asserts the exact wrong billing math** — it bills 20kWh at the generation-time tariff (600p) when US-2's own acceptance scenario explicitly forbids that and requires £5.00 (500p) at each reading's own tariff.
- A finance CSV export endpoint traces to a nonexistent "US-6" and appears nowhere in the sufficiency report's gap list — material divergence.
- `quickstart.md`, marked required by the sufficiency report, doesn't exist anywhere in the repo (confirmed missing, not just unreviewed).
- US-4 (P2) has zero cycle-card coverage, D-005 documents only one alternative despite a rejected one being referenced elsewhere, and a `[TODO]` on the tariff-authorization role was confirmed by the Tier-1 checker.

9 Critical, 3 Important, 1 Minor findings, plus three clarifying questions for the desk (push-vs-pull reality, the tariff auth role, and whether the export endpoint belongs in this feature at all) — all written into `review.md`.