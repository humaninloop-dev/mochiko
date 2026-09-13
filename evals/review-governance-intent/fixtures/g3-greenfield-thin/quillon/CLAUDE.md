# Quillon — conventions (Lena, day one)

- TypeScript strict everywhere; no `any`.
- Prettier formats, eslint lints; both run in CI and must pass.
- Every PR gets a review from Lena or Tomas before merge — no self-merging.
- Write tests for the invoice math before shipping it; totals, VAT, and rounding are where
  freelancers get burned.
- Client records are our users' clients, not ours. Treat them as confidential; never log them.
- Keep the schema in `src/lib/db/schema.ts`; migrations through Drizzle only.
