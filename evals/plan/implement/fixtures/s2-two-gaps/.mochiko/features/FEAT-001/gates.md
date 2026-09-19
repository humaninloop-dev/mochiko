# FEAT-001 — durable gate set

Accumulated **TEST:** gates from the delivered note-capture run; re-run by any later
feature building in this territory.

- **TEST:** Setup: fresh SQLite file, service started. Action: POST /notes
  {"text": "milk"}; restart service; GET /notes/{id}. Assert: 201 then 200 with
  identical text (SC-001).
- **TEST:** Setup: running service. Action: POST /notes with empty body. Assert: 400
  and the notes table row count is unchanged (SC-002).
- **TEST:** Setup: running service, one seeded note. Action: GET /notes/{seeded-id}
  and GET /notes/{random-uuid}. Assert: 200 with body, then 404 (SC-003).
