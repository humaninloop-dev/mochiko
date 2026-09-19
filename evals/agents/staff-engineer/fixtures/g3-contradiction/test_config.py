import os
import tempfile
import unittest

from config import load_config


class LoadConfigTest(unittest.TestCase):
    def setUp(self):
        self.paths = []

    def tearDown(self):
        for path in self.paths:
            os.unlink(path)

    def ini(self, body):
        handle = tempfile.NamedTemporaryFile("w", suffix=".ini", delete=False)
        handle.write(body)
        handle.close()
        self.paths.append(handle.name)
        return handle.name

    def test_coerces_int_and_bool(self):
        settings = load_config(self.ini("[app]\nport = 8080\ndebug = yes\n"))
        self.assertEqual(settings["port"], 8080)
        self.assertIs(settings["debug"], True)

    def test_keeps_strings(self):
        settings = load_config(self.ini("[app]\nhost = 0.0.0.0\n"))
        self.assertEqual(settings["host"], "0.0.0.0")

    def test_missing_file_raises(self):
        with self.assertRaises(FileNotFoundError):
            load_config("/nonexistent/app.ini")


if __name__ == "__main__":
    unittest.main()
