'use strict';

const Database = require('better-sqlite3');

const db = new Database(process.env.LINKJAR_DB || 'linkjar.db');

db.exec(`
  CREATE TABLE IF NOT EXISTS links (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    url TEXT NOT NULL,
    title TEXT NOT NULL,
    tags TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
  );
`);

function addLink({ url, title, tags }) {
  const row = db
    .prepare('INSERT INTO links (url, title, tags) VALUES (?, ?, ?) RETURNING *')
    .get(url, title, tags.join(','));
  return row;
}

function listLinks(tag) {
  if (!tag) {
    return db.prepare('SELECT * FROM links ORDER BY created_at DESC').all();
  }
  return db
    .prepare("SELECT * FROM links WHERE ',' || tags || ',' LIKE ? ORDER BY created_at DESC")
    .all(`%,${tag},%`);
}

function removeLink(id) {
  return db.prepare('DELETE FROM links WHERE id = ?').run(id).changes > 0;
}

// Nightly purge of links nobody came back to. Runs inside the web process.
setInterval(
  () => {
    const purged = db
      .prepare("DELETE FROM links WHERE created_at < datetime('now', '-365 days')")
      .run().changes;
    if (purged) {
      console.log(`purged ${purged} stale links`);
    }
  },
  24 * 60 * 60 * 1000
).unref();

module.exports = { addLink, listLinks, removeLink };
