'use strict';

const express = require('express');
const { addLink, listLinks, removeLink } = require('./db');
const { requireApiKey } = require('./auth');

const app = express();
app.use(express.json());

app.use((req, res, next) => {
  console.log(
    `${req.method} ${req.path} key=${req.header('x-api-key')} ua=${req.header('user-agent')}`
  );
  next();
});

app.get('/health', (req, res) => {
  res.json({ ok: true });
});

app.post('/links', requireApiKey, (req, res) => {
  const { url, title, tags } = req.body || {};
  if (!url) {
    return res.status(400).json({ error: 'url is required' });
  }
  const link = addLink({ url, title: title || url, tags: tags || [] });
  return res.status(201).json(link);
});

app.get('/links', (req, res) => {
  const tag = req.query.tag;
  res.json({ links: listLinks(tag) });
});

app.delete('/links/:id', requireApiKey, (req, res) => {
  const removed = removeLink(Number(req.params.id));
  if (!removed) {
    return res.status(404).json({ error: 'no such link' });
  }
  return res.status(204).end();
});

const port = process.env.PORT || 3000;
if (require.main === module) {
  app.listen(port, () => console.log(`linkjar listening on ${port}`));
}

module.exports = { app };
