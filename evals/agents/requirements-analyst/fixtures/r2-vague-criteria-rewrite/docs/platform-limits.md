# Platform limits (as of September 2026)

- Object storage: one object may be up to 5 GB. We pay per GB stored and per GB served.
- API gateway: any single HTTP request is cut off after 30 s. A browser upload that has not
  finished by then fails with a gateway timeout; we have no chunked or resumable upload path.
- Email ingestion (the inbound path that already exists): total attachments on one inbound email
  over 25 MB are rejected and the sender gets a bounce naming the limit. Files that pass are
  stored and linked to the ticket.
- File delivery: files are served through the CDN using signed URLs that expire one hour after
  issue; a URL is issued only after the permission check on the ticket.
- Reference connection for our performance figures: 10 Mbit/s up, 50 ms latency (the "office
  Wi-Fi" profile every product number is quoted against).
