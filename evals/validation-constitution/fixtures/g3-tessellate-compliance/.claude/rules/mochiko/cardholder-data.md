---
paths:
  - "src/payments/**/*.ts"
---

# Cardholder data <!-- GI-009 -->

- A primary account number MUST NOT be logged, persisted, cached, or placed in any queue; it
  exists in memory only between the request handler and the Adyen tokeniser call.
- Any function that receives a PAN MUST be named `*Untokenised` and MUST NOT be exported from
  its module.
- Card data MUST NOT be passed to any module outside `src/payments/`; the tokeniser returns a
  token, and everything downstream carries the token only.
- Every quarterly ASV scan report and every annual SAQ D MUST be filed under `compliance/pci/`
  with the date in the file name.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-009.
