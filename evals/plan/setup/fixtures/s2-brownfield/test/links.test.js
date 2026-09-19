'use strict';

const test = require('node:test');
const assert = require('node:assert');
const { addLink, listLinks } = require('../src/db');

test('a saved link comes back in the listing', () => {
  const saved = addLink({
    url: 'https://example.com/essay',
    title: 'An essay',
    tags: ['reading'],
  });
  assert.ok(saved.id);
  const listed = listLinks();
  assert.ok(listed.some((l) => l.id === saved.id));
});

test('listing filters by tag', () => {
  addLink({ url: 'https://example.com/recipe', title: 'A recipe', tags: ['food'] });
  const food = listLinks('food');
  assert.ok(food.every((l) => l.tags.split(',').includes('food')));
});
