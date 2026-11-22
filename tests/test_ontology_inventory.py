"""Regression tests for the public-only ontology inventory utility."""
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


SCRIPT = Path(__file__).resolve().parents[1] / "scripts" / "ontology_inventory.py"


class OntologyInventoryTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        subprocess.run(["git", "init", "-q", str(self.root)], check=True)

    def write_tracked(self, relative, content):
        path = self.root / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(content)
        subprocess.run(
            ["git", "-C", str(self.root), "add", "--", relative], check=True
        )
        return path

    def run_inventory(self):
        return subprocess.run(
            [sys.executable, str(SCRIPT), "--root", str(self.root)],
            capture_output=True, text=True, check=False
        )

    def test_sha256_and_stable_order_only_for_tracked_ontology_files(self):
        content = b"<urn:a> <urn:b> <urn:c> .\n"
        self.write_tracked("ontology/b.ttl", content)
        self.write_tracked("ontology/a.owl", b"ontology-a")
        self.write_tracked("ontology/skip.txt", b"not ontology")
        (self.root / "ontology" / "untracked.rdf").write_text("untracked")
        self.write_tracked("other/outside.ttl", b"out of scope")

        first = self.run_inventory()
        second = self.run_inventory()
        self.assertEqual(first.returncode, 0, first.stderr)
        self.assertEqual(first.stdout, second.stdout)
        report = json.loads(first.stdout)
        self.assertEqual(report["schema"], "arandu-ontology-inventory/v1")
        self.assertEqual(
            [entry["path"] for entry in report["files"]],
            ["ontology/a.owl", "ontology/b.ttl"]
        )
        self.assertEqual(report["files"][1]["bytes"], len(content))
        self.assertEqual(
            report["files"][1]["sha256"], hashlib.sha256(content).hexdigest()
        )

    def test_no_ontology_files_produces_empty_inventory(self):
        result = self.run_inventory()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(json.loads(result.stdout)["files"], [])

    def test_tracked_symlink_is_rejected_without_following_target(self):
        external = self.root.parent / (self.root.name + "-private-fixture")
        external.write_text("private synthetic fixture")
        self.addCleanup(external.unlink)
        link = self.root / "ontology" / "linked.ttl"
        link.parent.mkdir(parents=True, exist_ok=True)
        link.symlink_to(external)
        subprocess.run(
            ["git", "-C", str(self.root), "add", "--", "ontology/linked.ttl"],
            check=True
        )
        result = self.run_inventory()
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn("private synthetic fixture", result.stdout)


if __name__ == "__main__":
    unittest.main()
