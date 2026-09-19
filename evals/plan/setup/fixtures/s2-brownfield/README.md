# Linkjar

Linkjar is a small HTTP service that saves links for later reading. A browser extension
posts a URL, the service stores it with a title and tags, and a listing endpoint returns
what is saved. It has been running for a small group of users since last year.

## Running it

    npm install
    npm start        # serves on :3000
    npm test

## How we work

- Every route is covered by a test before it ships.
- Secrets never appear in logs.
- SQLite is the only datastore, and the service runs no background jobs — a request comes
  in, it is answered, nothing runs between requests.

## Layout

- `src/server.js` — HTTP routes
- `src/db.js` — SQLite access and the schema
- `src/auth.js` — API-key check on the write routes
- `test/links.test.js` — route tests
