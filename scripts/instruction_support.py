#!/usr/bin/env python3
"""Map manual mnemonics to the current XGK writer routes and factual metadata.

Prefer a supplied xgwx-mcp binary for its exported catalog. Source-table parsing
is a deterministic offline fallback, and also checks that a supplied binary has
the same metadata as the current sibling library. No projects are changed.
"""
from __future__ import annotations

import argparse
import copy
import hashlib
import json
import os
from pathlib import Path
import re
import select
import subprocess
import tempfile
import time
from typing import Any


_STRING = r'"(?:\\.|[^"\\])*"'
_NAMES = rf'({_STRING}(?:\s*\|\s*{_STRING})*)'
_CATALOG_FILES = ("src/instruction_catalog.rs", "src/comparison_catalog.rs")
_METADATA_FILES = (
    "src/instruction_operands.rs", "src/instruction_operand_data.rs",
    "src/instruction_cpu.rs", "src/instruction_cpu_data.rs",
)
_DIAGRAM_FILES = ("src/ladder_records.rs", "src/il.rs", "src/ladder_write.rs")


def _strings(text: str) -> list[str]:
    return [json.loads(value) for value in re.findall(_STRING, text)]


def _bracket_body(text: str, start: int) -> str:
    """Read a balanced Rust array expression, skipping quoted string contents."""
    if text[start] != "[":
        raise ValueError("expected array opening bracket")
    depth, index = 1, start + 1
    while index < len(text):
        if text[index] == '"':
            token = re.match(_STRING, text[index:])
            if token is None:
                raise ValueError("unterminated Rust string")
            index += token.end()
            continue
        if text[index] == "[":
            depth += 1
        elif text[index] == "]":
            depth -= 1
            if not depth:
                return text[start + 1:index]
        index += 1
    raise ValueError("unterminated Rust array")


def _required(pattern: str, text: str, description: str) -> str:
    match = re.search(pattern, text, re.S)
    if match is None:
        raise ValueError(f"unsupported Rust table shape: missing {description}")
    return match.group(1)


def _operand_tables(source: str) -> dict[str, list[dict[str, Any]]]:
    result: dict[str, list[dict[str, Any]]] = {}
    for arm in re.finditer(_NAMES + r'\s*=>\s*&\[', source):
        body = _bracket_body(source, arm.end() - 1)
        records = re.findall(r'LadderOperandSpec\s*\{(.*?)\}', body, re.S)
        if len(records) != body.count("LadderOperandSpec"):
            raise ValueError("unsupported operand-table record shape")
        operands = []
        for record in records:
            device = _required(r'device_areas:\s*(None|Some\(&\[.*?\]\))', record, "device_areas")
            constant = _required(r'allows_constant:\s*(None|Some\((?:true|false)\))', record, "allows_constant")
            operands.append({
                "label": json.loads(_required(r'label:\s*(' + _STRING + ')', record, "label")),
                "data_types": _strings(_required(r'data_types:\s*&\[(.*?)\]', record, "data_types")),
                "device_areas": None if device == "None" else _strings(device),
                "allows_constant": None if constant == "None" else constant == "Some(true)",
                "manual_page": json.loads(_required(r'manual_page:\s*(' + _STRING + ')', record, "manual_page")),
            })
        for name in _strings(arm.group(1)):
            if name in result:
                raise ValueError(f"duplicate operand-table mnemonic: {name}")
            result[name] = operands
    if not result:
        raise ValueError("no operand-table match arms found")
    return result


def _cpu_tables(source: str) -> dict[str, dict[str, Any]]:
    result = {}
    pattern = _NAMES + r'\s*=>\s*Some\(LadderInstructionCpuRestriction\s*\{(.*?)\}\)'
    for match in re.finditer(pattern, source, re.S):
        body = match.group(2)
        restriction = {
            "models": _strings(_required(r'models:\s*&\[(.*?)\]', body, "models")),
            "manual_page": json.loads(_required(r'manual_page:\s*(' + _STRING + ')', body, "manual_page")),
        }
        for name in _strings(match.group(1)):
            if name in result:
                raise ValueError(f"duplicate CPU-table mnemonic: {name}")
            result[name] = restriction
    if not result:
        raise ValueError("no CPU restriction match arms found")
    return result


def _source_catalogs(library_root: Path) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    rules = _operand_tables((library_root / "src/instruction_operand_data.rs").read_text())
    cpus = _cpu_tables((library_root / "src/instruction_cpu_data.rs").read_text())
    catalogs = []
    record_pattern = (
        r'LadderInstructionSpec\s*\{\s*mnemonic:\s*(' + _STRING + r'),\s*'
        r'opcode:\s*(\d+),\s*operand_count:\s*(\d+)\s*,?\s*\}'
    )
    for relative in _CATALOG_FILES:
        source = (library_root / relative).read_text()
        start = re.search(r'pub fn ladder_(?:instruction|comparison)_catalog\b', source)
        if start is None:
            raise ValueError(f"catalog function missing from {relative}")
        array = source.find("&[", start.end()) + 1
        body = _bracket_body(source, array)
        matches = list(re.finditer(record_pattern, body, re.S))
        if not matches or len(matches) != body.count("LadderInstructionSpec"):
            raise ValueError(f"unsupported catalog record shape in {relative}")
        catalog = []
        for match in matches:
            mnemonic = json.loads(match.group(1))
            catalog.append({
                "mnemonic": mnemonic,
                "opcode": int(match.group(2)),
                "operand_count": int(match.group(3)),
                "operand_rules": copy.deepcopy(rules.get(mnemonic, [])),
                "cpu_restriction": copy.deepcopy(cpus.get(mnemonic)),
            })
        catalogs.append(catalog)
    return catalogs[0], catalogs[1]


class _StdioClient:
    def __init__(self, binary: Path, startup: Path):
        self.stderr = tempfile.TemporaryFile()
        self.process = subprocess.Popen(
            [str(binary.resolve())], cwd=startup,
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=self.stderr,
            bufsize=0,
        )
        self.buffer = b""
        self.identifier = 0

    def __enter__(self):
        try:
            self.call("initialize", {
                "protocolVersion": "2025-11-25", "capabilities": {},
                "clientInfo": {"name": "instruction-support-generator", "version": "1.0"},
            })
            self.send({"jsonrpc": "2.0", "method": "notifications/initialized"})
            return self
        except BaseException:
            self.close()
            raise

    def send(self, message: dict[str, Any]):
        assert self.process.stdin is not None
        self.process.stdin.write(json.dumps(message, ensure_ascii=False).encode() + b"\n")
        self.process.stdin.flush()

    def receive(self, deadline: float) -> dict[str, Any]:
        assert self.process.stdout is not None
        while b"\n" not in self.buffer:
            remaining = deadline - time.monotonic()
            if remaining <= 0 or not select.select([self.process.stdout], [], [], remaining)[0]:
                raise TimeoutError("MCP server did not respond within 20 seconds")
            data = os.read(self.process.stdout.fileno(), 65536)
            if not data:
                self.stderr.seek(0)
                raise RuntimeError("MCP server closed stdout: " + self.stderr.read().decode(errors="replace"))
            self.buffer += data
        line, self.buffer = self.buffer.split(b"\n", 1)
        return json.loads(line)

    def call(self, method: str, params: dict[str, Any]) -> Any:
        self.identifier += 1
        request_id = self.identifier
        self.send({"jsonrpc": "2.0", "id": request_id, "method": method, "params": params})
        deadline = time.monotonic() + 20
        while True:
            response = self.receive(deadline)
            if response.get("id") != request_id:
                if "id" in response and "method" in response:
                    self.send({"jsonrpc": "2.0", "id": response["id"],
                               "error": {"code": -32601, "message": "Client request method unavailable"}})
                continue
            if "error" in response:
                raise RuntimeError(f"MCP {method} failed: {response['error']}")
            return response["result"]

    def close(self):
        if self.process.stdin is not None:
            self.process.stdin.close()
        try:
            self.process.wait(timeout=3)
        except subprocess.TimeoutExpired:
            self.process.terminate()
            try:
                self.process.wait(timeout=3)
            except subprocess.TimeoutExpired:
                self.process.kill()
                self.process.wait()
        if self.process.stdout is not None:
            self.process.stdout.close()
        self.stderr.close()

    def __exit__(self, *_exc):
        self.close()


def _stdio_catalogs(library_root: Path, binary: Path) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    fixture = library_root / "fixtures/elements.xgwx"
    if not fixture.is_file():
        fixture = next(iter(sorted((library_root / "fixtures").rglob("*.xgwx"))), None)
    if fixture is None:
        raise ValueError("a harmless existing .xgwx fixture is required for MCP catalog inspection")
    catalogs = []
    with _StdioClient(binary, library_root) as client:
        for section in ("catalog.instructions", "catalog.comparisons"):
            items, offset = [], 0
            while True:
                reply = client.call("tools/call", {"name": "inspect_file", "arguments": {
                    "path": str(fixture), "section": section, "offset": offset, "limit": 1000,
                }})
                if reply.get("isError"):
                    raise RuntimeError(f"MCP catalog inspection failed: {reply}")
                content = reply.get("structuredContent")
                if content is None:
                    content = json.loads(next(item["text"] for item in reply["content"] if item["type"] == "text"))
                page = content["data"]
                items.extend(page["items"])
                following = page["next_offset"]
                if following is None:
                    break
                if following <= offset:
                    raise ValueError("MCP catalog pagination failed to advance")
                offset = following
            catalogs.append(items)
    return catalogs[0], catalogs[1]


def load_support(library_root: Path, server_binary: Path | None = None) -> dict[str, dict[str, Any]]:
    """Return current writer support keyed by canonical uppercase manual mnemonic.

    `kind` is application, comparison, diagram_element, or manual_only.
    `writer_operation` is the public MCP operation (or None). Comparison aliases
    preserve their LOAD/AND/OR placement and pass `native_mnemonic` to the writer.
    A supplied server must match current library tables; rebuild if it is stale.
    Source provenance contains repository-relative names and content hashes only.
    """
    library_root = Path(library_root).resolve()
    source_catalogs = _source_catalogs(library_root)
    catalogs = source_catalogs
    backend = "source_tables"
    if server_binary is not None:
        catalogs = _stdio_catalogs(library_root, Path(server_binary))
        for live, source in zip(catalogs, source_catalogs):
            if {item["mnemonic"]: item for item in live} != {item["mnemonic"]: item for item in source}:
                raise ValueError("MCP catalogs differ from current library source; rebuild xgwx-mcp before generating guides")
        backend = "mcp_catalog"
    hashes = {name: hashlib.sha256((library_root / name).read_bytes()).hexdigest()
              for name in (*_CATALOG_FILES, *_METADATA_FILES, *_DIAGRAM_FILES)}
    result = {}
    for catalog, kind, operation, relative in zip(catalogs, ("application", "comparison"),
            ("insert_ladder_instruction", "insert_ladder_comparison"), _CATALOG_FILES):
        for item in catalog:
            name = item["mnemonic"]
            result[name] = {
                **copy.deepcopy(item), "kind": kind, "writer_operation": operation,
                "native_mnemonic": name,
                "placement": "output_end" if kind == "application" else "contact",
                "provenance": {"backend": backend, "source_sha256": {
                    key: hashes[key] for key in (relative, *_METADATA_FILES)
                }},
                "limitations": [
                    "Catalog membership identifies a guarded serialization route, not arbitrary topology support or native acceptance.",
                    "Unknown operand rules or CPU restrictions remain unknown; firmware, modules and PLC runtime behavior are not certified.",
                ],
            }
    for item in catalogs[1]:
        for prefix, placement in (("LOAD", "rung_start"), ("AND", "series"), ("OR", "parallel")):
            alias = prefix + item["mnemonic"]
            result[alias] = {
                **copy.deepcopy(result[item["mnemonic"]]),
                "mnemonic": alias, "placement": placement, "comparison_alias": True,
            }
            # Native catalog puts U before the comparison width; the manual
            # places it before the LOAD/AND/OR combine mnemonic instead.
            if re.fullmatch(r'U(?:D)?(?:=|>|<|>=|<=|<>)', item["mnemonic"]):
                manual_name = "U" + prefix + item["mnemonic"][1:]
                result[manual_name] = {
                    **copy.deepcopy(result[alias]), "mnemonic": manual_name,
                    "manual_name": manual_name, "canonical_name": manual_name,
                    "catalog_alias": alias, "aliases": [alias],
                }
                result[alias].update({"manual_name": manual_name,
                                      "canonical_name": manual_name,
                                      "catalog_alias": alias, "aliases": [manual_name]})
    # These correspond to current enum/native marker and IL emission mappings.
    contact_kinds = {
        "": ("NormallyOpen", 6), " NOT": ("NormallyClosed", 7),
        "P": ("AddressedRisingPulse", 8), "N": ("AddressedFallingPulse", 9),
        "P NOT": ("AddressedRisingPulseNot", 10), "N NOT": ("AddressedFallingPulseNot", 11),
    }
    diagrams = {}
    for prefix, placement in (("LOAD", "rung_start"), ("AND", "series"), ("OR", "parallel")):
        for suffix, (element_kind, marker) in contact_kinds.items():
            diagrams[prefix + suffix] = (element_kind, marker, 1, placement)
    diagrams.update({
        "OUT": ("Output", 14, 1, "output_column"),
        "OUT NOT": ("InverseOutput", 15, 1, "output_column"),
        "SET": ("Set", 16, 1, "output_column"),
        "RST": ("Reset", 17, 1, "output_column"),
        "RESET": ("Reset", 17, 1, "output_column"),
        "OUTP": ("RisingPulseOutput", 18, 1, "output_column"),
        "OUTN": ("FallingPulseOutput", 19, 1, "output_column"),
        "NOT": ("Inverse", 0x3E, 0, "logic_cell"),
        "R_EDGE": ("RisingPulse", 0x48, 0, "logic_cell"),
        "F_EDGE": ("FallingPulse", 0x49, 0, "logic_cell"),
    })
    for name, (element_kind, marker, arity, placement) in diagrams.items():
        result[name] = {
            "mnemonic": name, "native_mnemonic": name, "kind": "diagram_element",
            "writer_operation": "edit_ladder_cell", "element_kind": element_kind,
            "native_record_marker": marker, "opcode": None, "operand_count": arity,
            "operand_rules": [], "cpu_restriction": None, "placement": placement,
            "provenance": {"backend": "source_diagram_mapping", "source_sha256": {
                key: hashes[key] for key in _DIAGRAM_FILES
            }},
            "limitations": [
                "Use inspected physical XGK raw_y and cell columns, with exact expected kind/operand checks.",
                "LOAD/AND/OR describe placement of a native element; parallel placement needs the captured branch topology.",
                "Addressed cells use supported literal device syntax; operandless logic cells use an empty operand string.",
                "Manual semantics do not establish arbitrary circuit serialization, XG5000 acceptance or PLC behavior.",
            ],
        }
    for name in ("AND LOAD", "OR LOAD", "MPUSH", "MLOAD", "MPOP", "MCS", "MCSCLR", "END", "NOP"):
        if name not in result:
            result[name] = {
                "mnemonic": name, "native_mnemonic": name, "kind": "manual_only",
                "writer_operation": None, "opcode": None, "operand_count": None,
                "operand_rules": [], "cpu_restriction": None,
                "provenance": {"backend": "source_catalog_absence", "source_sha256": {
                    key: hashes[key] for key in (*_CATALOG_FILES, *_DIAGRAM_FILES)
                }},
                "limitations": ["No matching public instruction insertion or basic element route is exposed; preserve existing control-flow/terminal records."],
            }
    return dict(sorted(result.items()))


def resolve_support(mnemonic: str, support: dict[str, dict[str, Any]]) -> dict[str, Any] | None:
    """Resolve a manual spelling, without promoting a specialized step record.

    LOAD/AND/OR comparisons route only when the remainder is a real comparison
    catalog entry. Their placement is preserved; it is not an automatic branch
    construction API. Unknown mnemonics return None rather than guessed support.
    """
    name = " ".join(mnemonic.upper().split())
    step = re.fullmatch(r'(SET|OUT)\s+S[A-Z0-9]+(?:\.[A-Z0-9]+)?', name)
    if step is not None:
        base = support.get(step.group(1), {})
        return {
            "mnemonic": name, "native_mnemonic": name,
            "kind": "unsupported_specialized_step", "writer_operation": None,
            "base_mnemonic": step.group(1), "operand_count": None,
            "opcode": None, "operand_rules": [], "cpu_restriction": None,
            "provenance": copy.deepcopy(base.get("provenance")),
            "limitations": [
                "This manual form targets a specialized step-sequence S record, not the generic SET/OUT device coil.",
                "No matching specialized public XGK ladder writer route is exposed; do not substitute edit_ladder_cell or an SFC XML operation.",
            ],
        }
    if name in support:
        return copy.deepcopy(support[name])
    for prefix, placement in (("LOAD", "rung_start"), ("AND", "series"), ("OR", "parallel")):
        if name.startswith("U" + prefix):
            remainder = name[len(prefix) + 1:].strip()
            native_name = "U" + remainder
            native = support.get(native_name)
            if native is not None and native.get("kind") == "comparison":
                return {
                    **copy.deepcopy(native), "mnemonic": name, "manual_name": name,
                    "canonical_name": name, "catalog_alias": prefix + native_name,
                    "aliases": [prefix + native_name],
                    "placement": placement, "comparison_alias": True,
                }
        if name.startswith(prefix):
            remainder = name[len(prefix):].strip()
            native = support.get(remainder)
            if native is not None and native.get("kind") == "comparison":
                return {
                    **copy.deepcopy(native), "mnemonic": name,
                    "placement": placement, "comparison_alias": True,
                }
    return None


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("library_root", type=Path)
    parser.add_argument("--server-binary", type=Path)
    args = parser.parse_args()
    print(json.dumps(load_support(args.library_root, args.server_binary), ensure_ascii=False, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
