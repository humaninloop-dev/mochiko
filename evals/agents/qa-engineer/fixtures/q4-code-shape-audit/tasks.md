# Postbox — cycle cards

The commands CI runs for every cycle are in `.github/workflows/ci.yml`.

## C1 · subscriber sign-up — verified 2026-09-03

- [x] A visitor signs up with an email address; the address is normalised and validated, and a
  duplicate sign-up is a no-op. Story US-001. Case: Simple.

## C2 · import subscribers from a CSV

- [ ] An operator imports subscribers from a CSV export of their previous tool: one column of
  addresses plus optional name and tags, quoted fields allowed. Addresses are normalised the
  same way sign-up does; invalid rows are reported with their line number and skipped;
  duplicates (within the file or against existing subscribers) are skipped and counted. Story
  US-004. Case: Simple. Brownfield exposure: `[EXTEND]` the CLI with an `import` command; the
  address normalisation already exists from C1.

**TEST:** C2 — the sample export imports into PostgreSQL
- **Setup**: `docker compose up -d postgres` · `make migrate` · `psql "$DATABASE_URL" -c "truncate subscribers"`
- **Action**: `postbox import subscribers fixtures/subscribers.csv`
- **Assert**: Console contains "Imported 46 subscribers"
- **Assert**: Console contains "2 duplicates skipped"
- **Assert**: Console contains "2 invalid rows skipped (lines 17, 41)"
- **Action**: `psql "$DATABASE_URL" -tc "select count(*) from subscribers"`
- **Assert**: Console contains "46"
- **Capture**: console
