# Security baseline for files

- Every file that reaches Deskline through email ingestion is scanned for malware before it is
  made available; a file that fails the scan is quarantined and the ticket shows a notice in its
  place. Scanning takes under 5 s for 95% of files.
- Email ingestion accepts only these content types: png, jpg, gif, pdf, txt, log, csv, zip. Other
  types are dropped and the ticket shows what was dropped.
- Access to a file follows the ticket: the requester, the assigned agents, and admins. Every
  download is written to the audit log with who, when, and from which ticket.
- Files are encrypted at rest by the storage provider and served over HTTPS only.
- Retention: there is no rule today. Email-ingested files are kept for as long as the ticket
  exists, and closed tickets are never purged.
