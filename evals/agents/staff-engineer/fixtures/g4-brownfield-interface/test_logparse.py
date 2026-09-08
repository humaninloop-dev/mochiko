import unittest

from logparse import ParseError, count_by_level, errors_only, parse_line

SAMPLE = [
    "2026-09-01T14:03:22Z INFO  worker started (pid 4242)",
    "2026-09-01T14:03:25Z ERROR queue full: dropped 3 messages",
    "2026-09-01T14:59:59Z WARN  retrying upstream",
    "2026-09-01T15:00:01Z ERROR upstream timeout",
    "garbage without a level",
]


class ParseLineTest(unittest.TestCase):
    def test_splits_level_and_message(self):
        self.assertEqual(parse_line(SAMPLE[0]), ("INFO", "worker started (pid 4242)"))

    def test_message_keeps_inner_spaces(self):
        self.assertEqual(parse_line(SAMPLE[1]), ("ERROR", "queue full: dropped 3 messages"))

    def test_malformed_raises(self):
        with self.assertRaises(ParseError):
            parse_line(SAMPLE[4])

    def test_unknown_level_raises(self):
        with self.assertRaises(ParseError):
            parse_line("2026-09-01T14:03:22Z TRACE too chatty")


class AggregatorTest(unittest.TestCase):
    def test_count_by_level_skips_malformed(self):
        self.assertEqual(
            count_by_level(SAMPLE), {"DEBUG": 0, "INFO": 1, "WARN": 1, "ERROR": 2}
        )

    def test_errors_only_in_order(self):
        self.assertEqual(
            errors_only(SAMPLE), ["queue full: dropped 3 messages", "upstream timeout"]
        )


if __name__ == "__main__":
    unittest.main()
