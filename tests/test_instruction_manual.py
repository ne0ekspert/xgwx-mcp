"""Factual HTML parser checks, independent of the private manual archive."""
import importlib.util
import sys
import tempfile
import unittest
from pathlib import Path


PATH = Path(__file__).resolve().parents[1] / "scripts" / "instruction_manual.py"
SPEC = importlib.util.spec_from_file_location("instruction_manual", PATH)
manual = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = manual
SPEC.loader.exec_module(manual)


class InstructionManualTests(unittest.TestCase):
    def topic(self, content, title="4.20.1 ADD, ADDP, DADD, DADDP"):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "test.htm"
            path.write_bytes((f'<html><head><title>{title}</title></head><body>'
                              '<div id="topic_header"><p>Navigation</p><img src="arrow.png"></div>'
                              f'<div id="topic_content">{content}</div></body></html>').encode("cp949"))
            return manual.parse_topic(path)

    def test_scope_inline_spans_and_images(self):
        topic = self.topic('<p><span>S1</span><span>+1</span> applies</p>'
                           '<p><img src="lib/diagram.png" alt="Operands" width="100"></p>'
                           '<p>Created with the Personal Edition of HelpNDoc: footer</p>')
        self.assertEqual(topic["blocks"][0]["text"], "S1+1 applies")
        self.assertEqual([image["src"] for image in topic["images"]], ["lib/diagram.png"])
        self.assertEqual(len(topic["blocks"]), 2)
        self.assertEqual(topic["images"][0]["alt"], "Operands")

    def test_merged_cells_expand_without_losing_metadata(self):
        topic = self.topic('<table><tr><th rowspan="2">Command</th><th colspan="2">Areas</th></tr>'
                           '<tr><th>D</th><th>R</th></tr><tr><td rowspan="2">MOV</td>'
                           '<td>S</td><td>WORD</td></tr><tr><td>D</td><td>WORD</td></tr></table>')
        table = topic["blocks"][0]
        self.assertEqual(table["grid"], [["Command", "Areas", "Areas"], ["Command", "D", "R"],
                                         ["MOV", "S", "WORD"], ["MOV", "D", "WORD"]])
        self.assertEqual(table["rows"][0][0]["rowspan"], 2)
        self.assertEqual(table["rows"][0][1]["colspan"], 2)
        self.assertEqual(table["rows"][3][0]["column"], 1)

    def test_nested_table_is_not_mistaken_for_outer_rows(self):
        topic = self.topic('<table><tr><td>Outer<table><tr><td>Nested</td></tr></table>'
                           '</td><td>Second</td></tr></table>')
        table = topic["blocks"][0]
        self.assertEqual(len(table["rows"]), 1)
        self.assertEqual(table["grid"], [["Outer Nested", "Second"]])
        self.assertEqual(table["rows"][0][0]["nested_tables"][0]["grid"], [["Nested"]])

    def test_real_heading_aliases_and_list_behavior(self):
        topic = self.topic('<p>영역 설정</p><table><tr><td>오퍼랜드</td><td>자료형</td></tr>'
                           '<tr><td>S1</td><td>INT</td></tr></table>'
                           '<ul><li>S1과 S2를 더합니다.</li></ul><p>플래그 셋(Set)</p>'
                           '<table><tr><td>F110</td><td>Set</td></tr></table>'
                           '<p>기능</p><p>동작 설명</p><p>에러</p><p>지원 범위</p>'
                           '<p>프로그램 예시</p><p>ADD D0 D1 D2</p>')
        self.assertEqual(topic["operands"][0]["kind"], "table")
        self.assertEqual(topic["behavior"][0]["text"], "S1과 S2를 더합니다.")
        self.assertTrue(topic["behavior"][0]["list_item"])
        self.assertEqual(topic["flags"][0]["grid"], [["F110", "Set"]])
        self.assertEqual(topic["restrictions"][0]["text"], "지원 범위")
        self.assertEqual(topic["examples"][0]["text"], "ADD D0 D1 D2")

    def test_comparison_variants_and_unsigned_relocation(self):
        names = lambda text: [entry["mnemonic"] for entry in manual.instruction_names("4.15.19", text)]
        self.assertEqual(names("ULOAD X, ULOADD X")[:6],
                         ["ULOAD=", "ULOAD>", "ULOAD<", "ULOAD>=", "ULOAD<=", "ULOAD<>"])
        self.assertEqual(names("LOAD3 X, LOADD3 X")[:6],
                         ["LOAD=3", "LOAD>3", "LOAD<3", "LOAD>=3", "LOAD<=3", "LOAD<>3"])
        self.assertEqual(names("OR4X, OR8X")[-1], "OR8<>")
        entry = manual.instruction_names("4.15.19", "ULOAD X")[0]
        self.assertEqual(entry["manual_mnemonic"], "ULOAD=")
        self.assertEqual(entry["manual_pattern"], "ULOAD X")
        self.assertEqual(entry["catalog_alias"], "LOADU=")

    def test_compound_names_motion_and_step_templates(self):
        names = lambda chapter, text: [entry["mnemonic"] for entry in manual.instruction_names(chapter, text)]
        self.assertEqual(names("4.2.1", "AND LOAD"), ["AND LOAD"])
        self.assertEqual(names("4.1.1", "LOAD, LOAD NOT, LOADP, LOADN"),
                         ["LOAD", "LOAD NOT", "LOADP", "LOADN"])
        self.assertEqual(names("4.42.1", "XORG(EX)"), ["XORG", "XORGEX"])
        self.assertEqual(names("4.6.1", "SET Syyy.xx(순차 제어)"), ["SET Syyy.xx"])

    def test_title_corrections_are_explicit(self):
        entries = manual.instruction_names("4.21.1", "ADDB, ADDCP, DADDB, DADDBP",
                                           "4211addbaddcpdaddbdaddbp.htm")
        self.assertEqual(entries[1]["mnemonic"], "ADDBP")
        self.assertEqual(entries[1]["title_mnemonic"], "ADDCP")
        self.assertIn("correction", entries[1])

    def test_other_chapters_not_promoted_to_instructions(self):
        self.assertIsNone(self.topic("<p>Notes</p>", "3.1 Instruction summary"))
        topic = self.topic("<p>Timers</p>", "4.9.1 타이머의 특징")
        self.assertEqual(topic["instructions"], [])


if __name__ == "__main__":
    unittest.main()
