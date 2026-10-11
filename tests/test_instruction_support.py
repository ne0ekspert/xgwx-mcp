"""Contract checks for manual-to-writer routing and current catalog facts."""
import importlib.util
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("instruction_support", ROOT / "scripts/instruction_support.py")
support = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(support)


class InstructionSupportTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.library = support.resolve_library_root()
        cls.mapping = support.load_support(cls.library)

    def test_source_metadata_keeps_refined_types_and_constant_permissions(self):
        add = self.mapping["ADD"]
        dadd = self.mapping["DADD"]
        self.assertEqual(add["writer_operation"], "insert_ladder_instruction")
        self.assertEqual(add["operand_count"], 3)
        self.assertEqual(add["operand_rules"][0]["data_types"], ["INT"])
        self.assertEqual(dadd["operand_rules"][0]["data_types"], ["DINT"])
        self.assertTrue(add["operand_rules"][0]["allows_constant"])
        self.assertFalse(add["operand_rules"][2]["allows_constant"])
        self.assertEqual(add["operand_rules"][0]["manual_page"], "4201addaddpdadddaddp.htm")

    def test_comparison_aliases_preserve_native_mnemonic_and_placement(self):
        for alias, placement in (("LOAD=", "rung_start"), ("AND=", "series"), ("OR=", "parallel")):
            item = self.mapping[alias]
            self.assertEqual(item["native_mnemonic"], "=")
            self.assertEqual(item["opcode"], self.mapping["="]["opcode"])
            self.assertEqual(item["placement"], placement)
            self.assertEqual(item["writer_operation"], "insert_ladder_comparison")
        self.assertEqual(self.mapping["LOADB"]["native_mnemonic"], "B")
        self.assertEqual(self.mapping["ORBN"]["native_mnemonic"], "BN")
        self.assertNotIn("LOADUNMAPPED", self.mapping)

    def test_basic_contacts_and_operandless_cells_are_not_application_blocks(self):
        self.assertEqual(self.mapping["ORP NOT"]["element_kind"], "AddressedRisingPulseNot")
        self.assertEqual(self.mapping["ORP NOT"]["placement"], "parallel")
        self.assertEqual(self.mapping["R_EDGE"]["element_kind"], "RisingPulse")
        self.assertEqual(self.mapping["R_EDGE"]["operand_count"], 0)
        self.assertIsNone(self.mapping["R_EDGE"]["opcode"])
        self.assertEqual(self.mapping["OUT NOT"]["element_kind"], "InverseOutput")
        self.assertEqual(self.mapping["OUT NOT"]["writer_operation"], "edit_ladder_cell")
        self.assertIsNone(self.mapping["END"]["writer_operation"])

    def test_resolver_retains_comparison_prefixes_and_rejects_specialized_steps(self):
        for alias, native in (("LOADD=", "D="), ("LOADU=", "U="), ("LOADD=3", "D=3")):
            self.assertEqual(support.resolve_support(alias, self.mapping)["native_mnemonic"], native)
        for mnemonic in ("SET Syyy.xx", "OUT S000.01"):
            resolved = support.resolve_support(mnemonic, self.mapping)
            self.assertEqual(resolved["kind"], "unsupported_specialized_step")
            self.assertIsNone(resolved["writer_operation"])
        self.assertEqual(support.resolve_support(" set ", self.mapping)["element_kind"], "Set")
        self.assertIsNone(support.resolve_support("LOADMOV", self.mapping))
        self.assertIsNone(support.resolve_support("NOT_IN_THIS_CATALOG", self.mapping))

    def test_resolver_can_resolve_comparison_with_only_native_entries(self):
        native = {"D=": self.mapping["D="]}
        resolved = support.resolve_support("ANDD=", native)
        self.assertEqual(resolved["native_mnemonic"], "D=")
        self.assertEqual(resolved["placement"], "series")
        resolved["operand_rules"].clear()
        self.assertEqual(len(native["D="]["operand_rules"]), 2)

    def test_unsigned_manual_identity_retains_native_catalog_aliases(self):
        for manual, alias, native, placement in (
            ("ULOAD=", "LOADU=", "U=", "rung_start"),
            ("ULOADD=", "LOADUD=", "UD=", "rung_start"),
            ("UAND>=", "ANDU>=", "U>=", "series"),
            ("UANDD<>", "ANDUD<>", "UD<>", "series"),
            ("UOR<", "ORU<", "U<", "parallel"),
            ("UORD<=", "ORUD<=", "UD<=", "parallel"),
        ):
            resolved = support.resolve_support(manual, self.mapping)
            self.assertEqual(resolved["mnemonic"], manual)
            self.assertEqual(resolved["native_mnemonic"], native)
            self.assertEqual(resolved["catalog_alias"], alias)
            self.assertIn(alias, resolved["aliases"])
            self.assertEqual(resolved["placement"], placement)
            self.assertEqual(support.resolve_support(alias, self.mapping)["canonical_name"], manual)
        resolved = support.resolve_support("UANDD=", {"UD=": self.mapping["UD="]})
        self.assertEqual(resolved["native_mnemonic"], "UD=")
        self.assertEqual(resolved["placement"], "series")

    def test_unknown_cpu_table_differs_from_explicit_empty_model_table(self):
        self.assertIsNone(self.mapping["MOV"]["cpu_restriction"])
        self.assertEqual(self.mapping["GETCOMM"]["cpu_restriction"]["models"], [])
        self.assertIn("XGK-CPUHN", self.mapping["INLATCH"]["cpu_restriction"]["models"])
        self.assertNotIn("XGK-CPUH", self.mapping["INLATCH"]["cpu_restriction"]["models"])

    def test_source_provenance_uses_relative_names_and_sha256(self):
        for entry in self.mapping.values():
            for name, digest in entry["provenance"]["source_sha256"].items():
                self.assertFalse(Path(name).is_absolute())
                self.assertEqual(len(digest), 64)
                self.assertEqual(int(digest, 16) >= 0, True)

    def test_balanced_arrays_keep_brackets_inside_strings(self):
        self.assertEqual(support._bracket_body('["name[0]", &["x"]]', 0), '"name[0]", &["x"]')

    def test_exported_mcp_catalog_matches_current_source(self):
        binary = ROOT / "target/release/xgwx-mcp"
        if not binary.is_file():
            self.skipTest("build the release server to exercise exported stdio catalogs")
        exported = support.load_support(self.library, binary)
        self.assertEqual(exported.keys(), self.mapping.keys())
        for name in self.mapping:
            factual = {key: value for key, value in self.mapping[name].items() if key != "provenance"}
            self.assertEqual(factual, {key: value for key, value in exported[name].items() if key != "provenance"})
        self.assertEqual(exported["ADD"]["provenance"]["backend"], "mcp_catalog")


if __name__ == "__main__":
    unittest.main()
