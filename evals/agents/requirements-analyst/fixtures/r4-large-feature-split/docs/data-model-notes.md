# Volunteer record — shape and limits (engineering notes for spec authors)

- Fields: `first_name` (required) · `last_name` (required) · `email` (required, unique within the
  organisation, stored lower-cased) · `phone` (optional, stored in E.164 international format;
  the app converts a national-format number using the organisation's country) · `tags` (up to 20
  per volunteer, each up to 40 characters) · `notes` (free text, 2,000 characters) · `status`
  (`invited`, `active`, `inactive`) · `invited_at` · `accepted_at`.
- A volunteer is created in `invited` status and moves to `active` on acceptance. Nothing today
  creates a volunteer without also sending the invitation email.
- Deleting a volunteer is a soft delete: the record is hidden, but its email stays reserved in the
  organisation for 90 days so that a re-add within that window reattaches the history rather
  than creating a duplicate. After 90 days the record is purged.
- Limits: 5,000 volunteers per organisation on the Community plan (every current customer);
  20,000 on the Network plan (not yet sold).
- Adding a volunteer today is one form, one person at a time, and sends the invitation on save.
- Writes happen inside the browser's request. A request that outlives the 30-second gateway limit
  is cut off part-way, and rows already written stay written; there is no background job runner, and
  nothing resumes a request that was cut off or whose browser tab was closed.
