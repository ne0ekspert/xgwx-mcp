"""Check source coverage, callable identities and portable guide lookup."""
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
GUIDES = ROOT / "skills/xgwx-edit/references/instructions"
sys.path.insert(0, str(ROOT / "scripts"))
import generate_instruction_skills as generator
from instruction_support import load_support, resolve_library_root

spec = importlib.util.spec_from_file_location("find_instruction", ROOT / "skills/xgwx-edit/scripts/find_instruction.py")
finder = importlib.util.module_from_spec(spec)
spec.loader.exec_module(finder)


class InstructionGuideTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.manifest = json.loads((GUIDES / "manifest.json").read_text())

    def test_every_catalog_entry_has_exact_callable_guide_identity(self):
        support = load_support(resolve_library_root())
        core = {name: metadata for name, metadata in support.items()
                if metadata["kind"] in {"application", "comparison"} and not metadata.get("comparison_alias")}
        self.assertEqual(len(core), self.manifest["core_catalog_count"])
        for name, metadata in core.items():
            record = finder.lookup(self.manifest, name)
            self.assertIsNotNone(record, name)
            self.assertEqual(record["writer_mnemonic"], name)
            self.assertEqual(record["writer_operation"], metadata["writer_operation"])

    def test_manual_and_catalog_gaps_are_not_conflated(self):
        instructions = self.manifest["instructions"]
        covered = [record for record in instructions.values() if record["coverage"] == "manual_topic"]
        self.assertEqual(len(covered), self.manifest["manual_mnemonic_count"])
        self.assertEqual(len(instructions), self.manifest["guide_count"])
        for record in covered:
            self.assertTrue(record["manual_pages"])
            for page in record["manual_pages"]:
                self.assertIn(page, self.manifest["source_pages"])
        for record in instructions.values():
            if record["coverage"] == "catalog_only":
                self.assertFalse(record["manual_pages"])
                self.assertIn("semantics remain undocumented", (GUIDES / record["file"]).read_text())

    def test_file_and_figure_integrity_and_all_local_references(self):
        for relative, expected in self.manifest["generated_files_sha256"].items():
            path = GUIDES / relative
            self.assertEqual(hashlib.sha256(path.read_bytes()).hexdigest(), expected, relative)
            if path.suffix != ".md":
                continue
            for target in re.findall(r'!?\[[^\]]*\]\(([^)]+)\)', path.read_text()):
                if "://" not in target and not target.startswith("#"):
                    self.assertTrue((path.parent / target).is_file(), f"{relative}: {target}")
        for record in self.manifest["instructions"].values():
            markdown = (GUIDES / record["file"]).read_text()
            self.assertTrue(markdown.startswith("---\nname:"))
            self.assertNotIn("Created with the Personal Edition", markdown)
            self.assertNotIn("<script", markdown)
            self.assertIn("## Operands", markdown)
            self.assertIn("## Provenance", markdown)

    def test_unsigned_aliases_and_specialized_steps(self):
        unsigned = finder.lookup(self.manifest, "ULOADD<")
        self.assertEqual(unsigned["writer_mnemonic"], "UD<")
        self.assertEqual(finder.lookup(self.manifest, "LOADUD<")["file"], unsigned["file"])
        self.assertEqual(finder.lookup(self.manifest, "UD<")["file"], unsigned["file"])
        signed = finder.lookup(self.manifest, "LOADD<")
        self.assertEqual(signed["writer_mnemonic"], "D<")
        for name in ("SET Syyy.xx", "OUT Syyy.xx"):
            self.assertIsNone(finder.lookup(self.manifest, name)["writer_operation"])
        self.assertEqual(finder.lookup(self.manifest, "SET")["writer_operation"], "edit_ladder_cell")

    def test_known_copied_tables_are_omitted_and_reviewed_semantics_survive(self):
        pid = (GUIDES / "pidinit.md").read_text()
        self.assertNotIn("GWXNR연산을 시작", pid)
        self.assertIn("PID", pid)
        self.assertRegex(pid, r"\| 1 \| S \|[^\n]*\| WORD \| None \| Allowed \|")
        actual = finder.lookup(self.manifest, "OR4=")
        comparison = (GUIDES / actual["file"]).read_text()
        self.assertIn("parallel", comparison)
        self.assertIn("Source corrections", comparison)
        self.assertNotIn("다음 AND를 실행", comparison)
        for name in ("XGEARIP", "XGEARIPEX"):
            motion = (GUIDES / finder.lookup(self.manifest, name)["file"]).read_text()
            self.assertIn("Additional source notes after examples", motion)
            self.assertIn("servo", motion.lower())

    def test_lookup_portable_without_repo_or_manual(self):
        with tempfile.TemporaryDirectory() as directory:
            skill = Path(directory) / "xgwx-edit"
            (skill / "scripts").mkdir(parents=True)
            base = skill / "references/instructions"
            base.mkdir(parents=True)
            shutil.copyfile(ROOT / "skills/xgwx-edit/scripts/find_instruction.py", skill / "scripts/find_instruction.py")
            shutil.copyfile(GUIDES / "manifest.json", base / "manifest.json")
            record = finder.lookup(self.manifest, "TON")
            shutil.copyfile(GUIDES / record["file"], base / record["file"])
            result = subprocess.run([sys.executable, str(skill / "scripts/find_instruction.py"), "ton"],
                                    cwd=directory, check=True, capture_output=True, text=True)
            self.assertEqual(json.loads(result.stdout)["mnemonic"], "TON")
            self.assertTrue(Path(json.loads(result.stdout)["path"]).is_file())

    def test_failed_check_does_not_write_and_regeneration_preserves_independent_changes(self):
        with tempfile.TemporaryDirectory() as directory:
            stage = Path(directory) / "stage"
            output = Path(directory) / "output"
            stage.mkdir(); output.mkdir()
            (stage / "guide.md").write_text("generated")
            (output / "guide.md").write_text("user notes")
            with self.assertRaisesRegex(ValueError, "differ"):
                generator.install(stage, output, True)
            self.assertEqual((output / "guide.md").read_text(), "user notes")
            with self.assertRaisesRegex(ValueError, "independently changed"):
                generator.install(stage, output, False)
            self.assertEqual((output / "guide.md").read_text(), "user notes")

    def test_filename_and_flag_argument_count_are_separate(self):
        names = self.manifest["instructions"]
        self.assertEqual(len({generator.slug(name) for name in names}), len(names))
        rows = generator.flag_matrix({"grid": [["명령", "스텝", "플래그", "플래그"],
                                    ["명령", "스텝", "에러 F110", "캐리 F112"],
                                    ["ADD(P)", "4", "O", "-"], ["ADD(P)", "4", "O", "-"]]})
        self.assertEqual(rows, [["Manual instruction group", "Native steps", "에러 F110", "캐리 F112"], ["ADD(P)", "4", "O", "-"]])

    def test_pulse_forms_require_explicit_paired_manual_evidence(self):
        self.assertEqual(finder.lookup(self.manifest,"ADDP")["execution_form"],"paired_pulse_type")
        self.assertEqual(finder.lookup(self.manifest,"ADD")["execution_form"],"paired_base")
        self.assertIsNone(finder.lookup(self.manifest,"XGEARIP")["execution_form"])
        self.assertIn("245.htm",self.manifest["source_pages"])


if __name__ == "__main__":
    unittest.main()
