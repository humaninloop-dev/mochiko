import unittest

from textkit import merge_ranges, parse_duration, render_table, tokenize_query


class ParseDurationTest(unittest.TestCase):
    def test_seconds_only(self):
        self.assertEqual(parse_duration("90s"), 90)

    def test_hours_and_minutes(self):
        self.assertEqual(parse_duration("1h30m"), 5400)

    def test_all_units(self):
        self.assertEqual(parse_duration("1d2h3m4s"), 86400 + 7200 + 180 + 4)

    def test_any_unit_order(self):
        self.assertEqual(parse_duration("30m1h"), 5400)

    def test_whitespace_and_case(self):
        self.assertEqual(parse_duration("  1H 30M "), 5400)

    def test_zero(self):
        self.assertEqual(parse_duration("0s"), 0)

    def test_rejects_malformed(self):
        for text in ("", "   ", "90", "1h1h", "1x", "1.5h", "-5s", "h", "1h m"):
            with self.assertRaises(ValueError, msg=repr(text)):
                parse_duration(text)


class TokenizeQueryTest(unittest.TestCase):
    def test_plain_terms(self):
        self.assertEqual(tokenize_query("rust cargo"), [("rust", False), ("cargo", False)])

    def test_collapses_whitespace(self):
        self.assertEqual(
            tokenize_query("  rust \t cargo  "), [("rust", False), ("cargo", False)]
        )

    def test_negated_phrase(self):
        self.assertEqual(
            tokenize_query('-"cargo build" rust'), [("cargo build", True), ("rust", False)]
        )

    def test_punctuation_is_literal(self):
        self.assertEqual(
            tokenize_query("lang:rust a-b"), [("lang:rust", False), ("a-b", False)]
        )

    def test_phrase_keeps_inner_whitespace(self):
        self.assertEqual(tokenize_query('"multi  space"'), [("multi  space", False)])

    def test_quotes_toggle_mid_token(self):
        self.assertEqual(tokenize_query('foo"bar baz"qux'), [("foobar bazqux", False)])

    def test_empty_inputs(self):
        for text in ("", "   ", '""'):
            self.assertEqual(tokenize_query(text), [], repr(text))

    def test_rejects_unterminated_quote_and_bare_dash(self):
        for text in ('"unterminated', "rust -", "-"):
            with self.assertRaises(ValueError, msg=repr(text)):
                tokenize_query(text)


class RenderTableTest(unittest.TestCase):
    def test_aligns_numeric_right_and_text_left(self):
        table = render_table([["apple", 3], ["kiwi", 12]], ["name", "qty"])
        self.assertEqual(table, "name   qty\n-----  ---\napple    3\nkiwi    12")

    def test_header_and_rule_only_when_no_rows(self):
        self.assertEqual(render_table([], ["name", "qty"]), "name  qty\n----  ---")

    def test_mixed_column_is_left_aligned(self):
        table = render_table([["a", 1], ["b", "n/a"]], ["k", "v"])
        self.assertEqual(table, "k  v\n-  ---\na  1\nb  n/a")

    def test_single_numeric_column_right_aligns_header(self):
        self.assertEqual(render_table([[1], [22]], ["n"]), " n\n--\n 1\n22")

    def test_no_trailing_whitespace(self):
        table = render_table([["x", 1]], ["a", "b"])
        self.assertFalse(table.endswith("\n"))
        for line in table.split("\n"):
            self.assertEqual(line, line.rstrip())

    def test_rejects_ragged_rows_and_empty_headers(self):
        with self.assertRaises(ValueError):
            render_table([["a", 1], ["b"]], ["k", "v"])
        with self.assertRaises(ValueError):
            render_table([], [])


class MergeRangesTest(unittest.TestCase):
    def test_overlapping(self):
        self.assertEqual(merge_ranges([(1, 3), (2, 6), (8, 10)]), [(1, 6), (8, 10)])

    def test_adjacent(self):
        self.assertEqual(merge_ranges([(1, 3), (4, 6)]), [(1, 6)])

    def test_unsorted_input(self):
        self.assertEqual(merge_ranges([(8, 10), (1, 3)]), [(1, 3), (8, 10)])

    def test_contained_and_duplicate(self):
        self.assertEqual(merge_ranges([(1, 10), (2, 3), (1, 10)]), [(1, 10)])

    def test_gap_of_one_stays_split(self):
        self.assertEqual(merge_ranges([(1, 3), (5, 6)]), [(1, 3), (5, 6)])

    def test_single_points(self):
        self.assertEqual(merge_ranges([(5, 5), (6, 6)]), [(5, 6)])

    def test_empty(self):
        self.assertEqual(merge_ranges([]), [])

    def test_rejects_inverted_and_non_integer_bounds(self):
        with self.assertRaises(ValueError):
            merge_ranges([(3, 1)])
        with self.assertRaises(ValueError):
            merge_ranges([(1, "2")])


if __name__ == "__main__":
    unittest.main()
