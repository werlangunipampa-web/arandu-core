"""A1.7 end-to-end smoke for HTML, SVG, navigation and line protocol."""
from __future__ import annotations

import subprocess
import unittest
from pathlib import Path
from xml.etree import ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]
BIN = ROOT / "target" / "debug" / "arandu-cli"


class VisualSmoke(unittest.TestCase):
    @staticmethod
    def invoke(*args: str, input_text: str | None = None) -> str:
        return subprocess.run(
            [str(BIN), *args],
            cwd=ROOT, input=input_text, text=True, capture_output=True,
            check=True, timeout=10,
        ).stdout

    def test_svg_is_well_formed(self):
        xml = ET.fromstring(self.invoke("--preview-svg"))
        self.assertEqual(xml.tag, "{http://www.w3.org/2000/svg}svg")
        self.assertGreaterEqual(
            len(xml.findall("{http://www.w3.org/2000/svg}circle")), 1
        )

    def test_html_contains_controls_and_three_scenes(self):
        html = self.invoke("--preview-html")
        self.assertTrue(html.startswith("<!doctype html>"))
        self.assertEqual(html.count("<svg "), 3)
        for state in ("collapsed", "expanded", "pinned"):
            self.assertIn(f'data-scene="{state}"', html)

    def test_core_navigation_and_pin(self):
        output = self.invoke(
            "--interactive-core", input_text="2\n3\ntick\n1\nquit\n"
        )
        self.assertIn("state=Expanded", output)
        self.assertIn("state=Pinned", output)
        self.assertIn("state=Collapsed", output)

    def test_protocol_does_not_reply_to_unknown_commands(self):
        output = self.invoke(
            "--window-protocol", input_text="2\ninvalid\n1\nquit\n"
        )
        frames = output.splitlines()
        self.assertEqual(len(frames), 3)
        for frame in frames:
            self.assertTrue(frame.startswith("SCENE <svg "))
            self.assertTrue(frame.endswith("</svg>"))
            ET.fromstring(frame.removeprefix("SCENE "))
        self.assertIn("MEMÓRIA", frames[1])
        self.assertNotIn("MEMÓRIA", frames[-1])


if __name__ == "__main__":
    unittest.main()
