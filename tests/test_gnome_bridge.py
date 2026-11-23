"""Contract tests for the optional GNOME DBus → Rust bridge.

No GNOME session or application data is needed for these tests.
"""
from __future__ import annotations

import importlib.util
import pathlib
import unittest

SCRIPT = pathlib.Path(__file__).resolve().parents[1] / "scripts/arandu_gnome_snapshot.py"
SPEC = importlib.util.spec_from_file_location("arandu_gnome_snapshot", SCRIPT)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class GnomeBridgeContractTests(unittest.TestCase):
    def test_deduplicate_apps_and_ignore_titles(self):
        raw = (
            '{"windows":[{"id":"1","app":"org.gnome.Nautilus.desktop",'
            '"title":"Documento extremamente privado"},'
            '{"id":"2","app":"org.gnome.Nautilus.desktop",'
            '"title":"outro arquivo"},'
            '{"id":"3","app":"org.mozilla.firefox.desktop",'
            '"where":"Área de trabalho 2"}],'
            '"recent":[{"title":"conteúdo privado"}]}'
        )
        ids = MODULE.extract_app_ids(raw)
        self.assertEqual(
            ids,
            ("org.gnome.Nautilus.desktop", "org.mozilla.firefox.desktop"),
        )
        self.assertNotIn("Documento", "\n".join(ids))
        self.assertNotIn("Área", "\n".join(ids))

    def test_empty_snapshot_is_valid(self):
        self.assertEqual(MODULE.extract_app_ids('{"windows":[]}'), ())

    def test_rejects_unexpected_shapes_and_injected_ids(self):
        for bad in (
            "not-json",
            "[]",
            '{"recent":[]}',
            '{"windows":[null]}',
            '{"windows":[{"app":"x\\nsecret"}]}',
            '{"windows":[{"app":"../secret"}]}',
            '{"windows":[{"app":"a b"}]}',
            '{"windows":[{"app":0}]}',
        ):
            with self.subTest(bad=bad):
                with self.assertRaises((ValueError, TypeError)):
                    MODULE.extract_app_ids(bad)

    def test_caps_snapshot_size_and_window_count(self):
        too_many = '{"windows":[' + ",".join(
            '{"app":"a.desktop"}' for _ in range(MODULE.MAX_WINDOWS + 1)
        ) + ']}'
        with self.assertRaises(ValueError):
            MODULE.extract_app_ids(too_many)
        huge = '{"windows":[],"padding":"' + "x" * 257_000 + '"}'
        with self.assertRaises(ValueError):
            MODULE.extract_app_ids(huge)


if __name__ == "__main__":
    unittest.main()
