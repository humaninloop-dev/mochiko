# Quillon

Quotes and invoices for freelance translators. Scaffold only — nothing is wired up yet beyond
the SvelteKit skeleton, a Drizzle schema stub, and the CI workflow.

- `npm install` · `npm run dev`
- `npm run lint` — eslint + prettier check
- `npm test` — placeholder until the invoice math lands
- Deploys to Vercel on merge to `main`; Postgres on Neon (see `.env.example`)
