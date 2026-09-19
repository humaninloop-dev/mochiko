import unittest

from slug import slugify


class SlugifyTest(unittest.TestCase):
    def test_basic(self):
        self.assertEqual(slugify("Hello World!"), "hello-world")

    def test_collapses_runs_and_trims(self):
        self.assertEqual(slugify("  --Rust & Cargo: 2026--  "), "rust-cargo-2026")

    def test_empty(self):
        self.assertEqual(slugify("!!!"), "")


if __name__ == "__main__":
    unittest.main()
