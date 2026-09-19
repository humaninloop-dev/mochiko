'use strict';

// Single shared API key, read from the environment at boot. The browser extension sends
// it on every write request.
function requireApiKey(req, res, next) {
  const presented = req.header('x-api-key');
  if (!presented || presented !== process.env.LINKJAR_API_KEY) {
    return res.status(401).json({ error: 'bad or missing api key' });
  }
  return next();
}

module.exports = { requireApiKey };
