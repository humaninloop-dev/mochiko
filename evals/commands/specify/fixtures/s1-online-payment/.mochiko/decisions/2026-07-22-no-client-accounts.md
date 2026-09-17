# Clients never get accounts

- **Status:** ruled
- **Date:** 2026-07-22
- **Context:** Studios send invoices to clients who use Ledgerlite once or twice a year.
  Every interview said the same thing: a login wall means the invoice does not get opened.
- **Decision:** Clients open invoices on a public link carrying an unguessable token. No
  client accounts, no client passwords, no client sessions.
- **Rationale:** The link is the whole access control, so the token rule in
  `.claude/rules/mochiko/public-pages.md` carries the weight an account system would.
- **Alternatives considered:** magic-link sign-in per client (rejected: still a wall);
  optional accounts (rejected: two code paths for one page).
