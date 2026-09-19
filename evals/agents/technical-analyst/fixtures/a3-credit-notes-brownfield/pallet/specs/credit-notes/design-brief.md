# Design brief — FEAT-015 Credit notes

The sufficiency review of the credit-notes spec against the current product found these
gaps, listed with the priority the review assigned to each. The feature is bounded by the
spec.

| Gap | Priority | What is missing |
|---|---|---|
| G1 | P1 | No API contract for creating, editing, issuing, and viewing a credit note, or for exposing an invoice's credited amount |
| G2 | P1 | No data model for the credit note, its lines, its link to the invoice, and what happens to the credit (applied or refunded) |
| G3 | P2 | No decision on whether a credit note may exceed the invoice's outstanding amount (the spec's open question) |
| G4 | P3 | No design for the credit-note PDF |

Everything the review did not list is considered sufficient as it stands: the invoice,
payment, and retailer surfaces are unchanged except where a gap above requires it.
