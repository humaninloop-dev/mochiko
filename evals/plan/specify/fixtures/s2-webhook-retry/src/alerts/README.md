# src/alerts

Channel model and the attempt recorder. The notifier in `src/notifier/` drains the
`alert_queue` table and calls `recordAttempt()` here before it moves on. There is no
retry path today: `deliver()` returns the outcome and the row is done.
