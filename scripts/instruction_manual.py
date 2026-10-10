#!/usr/bin/env python3
"""Parse a locally extracted XGK(B) InstructionHelp V3.5 manual.

The parser is deliberately separate from rendering: the extracted document text
is private build input, not an artifact to distribute verbatim. It reads only
topic_content and retains source identities, table spans and image references.
Library support is supplied separately by instruction_support.py.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
from dataclasses import dataclass, field
from html.parser import HTMLParser
from pathlib import Path


@dataclass
class Node:
    tag: str
    attrs: dict[str, str] = field(default_factory=dict)
    children: list[Node | str] = field(default_factory=list)


class Document(HTMLParser):
    """Small DOM parser, preserving inline text instead of splitting spans."""

    VOID = {"area", "base", "br", "col", "embed", "hr", "img", "input",
            "link", "meta", "param", "source", "track", "wbr"}

    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.root = Node("document")
        self.stack = [self.root]

    def handle_starttag(self, tag, attrs):
        node = Node(tag.lower(), dict((k.lower(), v or "") for k, v in attrs))
        self.stack[-1].children.append(node)
        if node.tag not in self.VOID:
            self.stack.append(node)

    def handle_startendtag(self, tag, attrs):
        self.handle_starttag(tag, attrs)
        if tag.lower() not in self.VOID:
            self.handle_endtag(tag)

    def handle_endtag(self, tag):
        for index in range(len(self.stack) - 1, 0, -1):
            if self.stack[index].tag == tag.lower():
                del self.stack[index:]
                break

    def handle_data(self, data):
        self.stack[-1].children.append(data)


def descendants(node: Node, tag: str | None = None):
    for child in node.children:
        if isinstance(child, Node):
            if tag is None or child.tag == tag:
                yield child
            yield from descendants(child, tag)


def normalize(text: str) -> str:
    return " ".join(text.replace("\xa0", " ").split())


def text_content(node: Node) -> str:
    def fragments(current):
        for child in current.children:
            if isinstance(child, str):
                yield child
            elif child.tag not in {"script", "style", "img"}:
                if child.tag in {"br", "p", "div", "tr", "td", "th", "li"}:
                    yield " "
                yield from fragments(child)
                if child.tag in {"p", "div", "tr", "td", "th", "li"}:
                    yield " "
    return normalize("".join(fragments(node)))


def images(node: Node) -> list[dict]:
    nodes = ([node] if node.tag == "img" else []) + list(descendants(node, "img"))
    return [{"src": image.attrs.get("src", ""), "alt": image.attrs.get("alt", ""),
             "width": image.attrs.get("width"), "height": image.attrs.get("height")}
            for image in nodes]


def _direct_table_rows(node: Node):
    for child in node.children:
        if not isinstance(child, Node):
            continue
        if child.tag == "tr":
            yield child
        elif child.tag in {"thead", "tbody", "tfoot"}:
            yield from _direct_table_rows(child)


def _immediate_nested_tables(node: Node):
    for child in node.children:
        if isinstance(child, Node):
            if child.tag == "table":
                yield child
            else:
                yield from _immediate_nested_tables(child)


def parse_table(node: Node) -> dict:
    """Return original cells and an expanded logical grid with merged values."""
    rows = []
    spans: dict[tuple[int, int], str] = {}
    for row_index, row in enumerate(_direct_table_rows(node)):
        cells, column = [], 0
        for cell in row.children:
            if not isinstance(cell, Node) or cell.tag not in {"td", "th"}:
                continue
            while (row_index, column) in spans:
                column += 1
            try:
                rowspan = max(1, int(cell.attrs.get("rowspan", "1")))
                colspan = max(1, int(cell.attrs.get("colspan", "1")))
            except ValueError:
                rowspan = colspan = 1
            value = text_content(cell)
            cells.append({"text": value, "row": row_index, "column": column,
                          "rowspan": rowspan, "colspan": colspan,
                          "images": images(cell),
                          "nested_tables": [parse_table(nested) for nested in _immediate_nested_tables(cell)]})
            for r in range(row_index, row_index + rowspan):
                for c in range(column, column + colspan):
                    spans[r, c] = value
            column += colspan
        rows.append(cells)
    height = max((r for r, _ in spans), default=-1) + 1
    width = max((c for _, c in spans), default=-1) + 1
    grid = [[spans.get((r, c), "") for c in range(width)] for r in range(height)]
    return {"rows": rows, "grid": grid, "images": images(node)}


HEADINGS = (
    ("사용할수있는영역", "usable_areas"), ("사용가능영역", "usable_areas"),
    ("명령어", "instructions"), ("오퍼랜드", "operands"), ("영역설정", "operands"),
    ("관련플래그", "flags"), ("플래그", "flags"),
    ("플래그셋(Set)", "flags"), ("플래그처리", "flags"),
    ("프로그램예시", "examples"), ("프로그램예", "examples"), ("프로그램", "examples"),
    ("해설", "behavior"), ("설명", "behavior"), ("기능", "behavior"),
    ("주의사항", "restrictions"), ("주의", "restrictions"),
    ("에러", "restrictions"), ("타임차트", "examples"),
)


def section_heading(text: str) -> str | None:
    compact = re.sub(r"\s+", "", text)
    compact = re.sub(r"^[\[\](（)）.．:：①-⑳0-9●■※·-]+", "", compact)
    for heading, section in HEADINGS:
        if compact == heading or compact.startswith(heading + ":"):
            return section
    return None


def table_section(table: dict, current: str) -> str:
    first = [re.sub(r"\s+", "", c["text"]) for c in table["rows"][0]] if table["rows"] else []
    if first and first[0] == "오퍼랜드":
        return "operands"
    if "사용가능영역" in first or "사용할수있는영역" in first:
        return "usable_areas"
    if first and first[0] == "지원기종":
        return "restrictions"
    if first and first[0] == "명령어":
        return "instructions"
    return current


def content_blocks(root: Node) -> list[dict]:
    blocks = []
    section = "overview"

    def walk(node):
        nonlocal section
        if node.tag in {"script", "style"}:
            return
        if node.tag == "table":
            table = parse_table(node)
            table["kind"] = "table"
            table["section"] = table_section(table, section)
            blocks.append(table)
            # Operand tables are followed by the instruction explanation even
            # on pages where the author omitted a literal 해설 heading.
            if table["section"] == "operands":
                section = "behavior"
            return
        if node.tag in {"p", "h1", "h2", "h3", "h4", "li"}:
            value = text_content(node)
            if value.startswith("Created with the Personal Edition of HelpNDoc:"):
                return
            detected = section_heading(value)
            if detected:
                section = detected
                blocks.append({"kind": "heading", "section": section, "text": value})
            elif value:
                paragraph_section = "restrictions" if value.startswith(("적용 기종", "적용기종", "적용제품")) else section
                blocks.append({"kind": "paragraph", "section": paragraph_section, "text": value,
                               "images": images(node), "list_item": node.tag == "li"})
            for image in images(node):
                blocks.append({"kind": "image", "section": section, **image})
            return
        if node.tag == "img":
            blocks.append({"kind": "image", "section": section, **images(node)[0]})
            return
        for child in node.children:
            if isinstance(child, Node):
                walk(child)
    walk(root)
    return blocks


RELATIONS = ["=", ">", "<", ">=", "<=", "<>"]
TITLE_CORRECTIONS = {
    "4211addbaddcpdaddbdaddbp.htm": {"ADDCP": "ADDBP"},
    "4248betowbtowp.htm": {"BETOW": "BTOW"},
    "4433xturn.htm": {"XTURN": "XTRUN"},
    "4394putputpxgf-rd8a.htm": {"PUT": "PUTE", "PUTP": "PUTEP"},
}


def instruction_names(chapter: str, title: str, source_page: str = "") -> list[dict]:
    heading = re.sub(r"([A-Z$][A-Z0-9$_]*)\(EX\)",
                     lambda match: match[1] + "," + match[1] + "EX", title)
    heading = re.sub(r"\([^)]*\)", "", heading).split(":")[0]
    entries = []
    for token in re.split(r"[,./]", heading):
        token = normalize(token)
        manual_spellings = {}
        comparison = re.fullmatch(r"([A-Z$0-9]+)\s*X", token)
        if comparison and chapter.startswith("4.15."):
            prefix = comparison[1]
            if prefix.startswith("U"):
                prefix = re.sub(r"^(LOAD|AND|OR)", r"\1U", prefix[1:])
            suffix = "3" if prefix.endswith("3") else ""
            if suffix:
                prefix = prefix[:-1]
            names = [prefix + relation + suffix for relation in RELATIONS]
            original_prefix = comparison[1][:-1] if suffix else comparison[1]
            manual_spellings = dict(zip(names, [original_prefix + relation + suffix for relation in RELATIONS]))
        elif chapter.startswith("4.6."):
            match = re.match(r"(SET|OUT)\b", token)
            names = [match[1] + " Syyy.xx"] if match else []
        else:
            # Require uppercase mnemonic spelling; operand templates such as
            # Syyy.xx and prose headings must not become bogus commands.
            names = re.findall(r"(?<![A-Za-z0-9_$])([A-Z$][A-Z0-9$_]*(?:\s+(?:NOT|LOAD))?)(?![a-z])", token)
        for name in names:
            name = normalize(name)
            corrected = TITLE_CORRECTIONS.get(source_page, {}).get(name, name)
            manual_name = manual_spellings.get(name, corrected)
            entry = {"mnemonic": manual_name, "manual_name": manual_name,
                     "title_mnemonic": manual_spellings.get(name, name)}
            if comparison and chapter.startswith("4.15."):
                entry["manual_pattern"] = token
                entry["manual_mnemonic"] = manual_spellings[name]
                if manual_name != name:
                    entry["catalog_alias"] = name
            if corrected != name:
                entry["correction"] = "Reviewed title typo; usage table and native catalog identify this mnemonic."
            if entry not in entries:
                entries.append(entry)
    return entries


def parse_topic(path: Path) -> dict | None:
    source = path.read_bytes()
    html = source.decode("cp949", errors="strict")
    raw_title = re.search(r"<title>(.*?)</title>", html, re.I | re.S)
    if raw_title is None or not re.match(r"\s*4\.\d+\.\d+\s", raw_title[1]):
        return None
    document = Document()
    document.feed(html)
    title_node = next(descendants(document.root, "title"), None)
    title = text_content(title_node) if title_node else ""
    match = re.fullmatch(r"(4\.\d+\.\d+)\s+(.+)", title)
    if not match:
        return None
    content = next((node for node in descendants(document.root)
                    if node.attrs.get("id") == "topic_content"), None)
    if content is None:
        raise ValueError(f"Missing topic_content: {path.name}")
    blocks = content_blocks(content)
    topic = {"chapter": match[1], "title": title, "instruction_title": match[2],
             "source_page": path.name, "source_sha256": hashlib.sha256(source).hexdigest(),
             "instructions": instruction_names(match[1], match[2], path.name), "blocks": blocks}
    for section in ["operands", "usable_areas", "flags", "behavior", "restrictions", "examples"]:
        topic[section] = [block for block in blocks if block["section"] == section
                          and block["kind"] != "heading"]
    topic["images"] = images(content)
    return topic


def load_manual(directory: Path | str, library_root=None, manual_sha256: str | None = None) -> dict:
    """Load chapter 4 detail topics; library_root is accepted for caller compatibility."""
    directory = Path(directory)
    if not directory.is_dir():
        raise ValueError(f"Not an extracted manual directory: {directory}")
    topics = [topic for path in sorted(directory.glob("*.htm")) if (topic := parse_topic(path))]
    if not topics:
        raise ValueError("No chapter 4 instruction detail topics found")
    topics.sort(key=lambda topic: (tuple(map(int, topic["chapter"].split("."))), topic["source_page"]))
    names = sorted({entry["mnemonic"] for topic in topics for entry in topic["instructions"]})
    return {"manual": "XGK(B)InstructionHelp_Kr_V3.5.chm", "manual_sha256": manual_sha256,
            "encoding": "cp949", "topic_count": len(topics), "mnemonic_count": len(names),
            "mnemonics": names, "topics": topics}


def main():
    arguments = argparse.ArgumentParser(description=__doc__)
    arguments.add_argument("manual", type=Path, help="directory containing extracted CHM pages")
    arguments.add_argument("--manual-sha256")
    arguments.add_argument("--output", type=Path, help="private output for generation; default stdout")
    options = arguments.parse_args()
    result = load_manual(options.manual, manual_sha256=options.manual_sha256)
    output = json.dumps(result, ensure_ascii=False, indent=2) + "\n"
    if options.output:
        options.output.write_text(output, encoding="utf-8")
        print(json.dumps({key: result[key] for key in ["topic_count", "mnemonic_count"]}))
    else:
        print(output, end="")


if __name__ == "__main__":
    main()
