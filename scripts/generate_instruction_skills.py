#!/usr/bin/env python3
"""Generate individual XGK instruction guides from a user-supplied CHM.

Only instruction-specific notes, factual tables and referenced figures are
included. The CHM, raw HTML and normalized private inventory are not packaged.
Use --check to compare regenerated output without changing any guide.
"""
from __future__ import annotations

import argparse
from collections import defaultdict
import hashlib
import json
from pathlib import Path
import re
import shutil
import struct
import subprocess
import tempfile
from urllib.parse import unquote

from instruction_manual import Document, descendants, load_manual, text_content
from instruction_support import load_support, resolve_support, resolve_library_root

ROOT = Path(__file__).resolve().parents[1]
EXPECTED_MANUAL_SHA256 = "b866d589d800addcb039f9f09b182592464d6367faa55abf6dde9ac34a1580e0"


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def slug(name: str) -> str:
    result = name.lower().replace("$", "dollar-")
    for symbol, spelling in [("<>", "-ne-"), (">=", "-ge-"), ("<=", "-le-"),
                             ("=", "-eq-"), (">", "-gt-"), ("<", "-lt-")]:
        result = result.replace(symbol, spelling)
    return re.sub(r"[^a-z0-9]+", "-", result).strip("-")


def cell(value) -> str:
    return str(value).replace("|", "\\|").replace("\n", "<br>")


def table(rows: list[list[str]]) -> str:
    if not rows:
        return ""
    width = max(map(len, rows))
    if not width:
        return ""
    padded = [row + [""] * (width - len(row)) for row in rows]
    return "\n".join(["| " + " | ".join(map(cell, padded[0])) + " |",
                       "| " + " | ".join(["---"] * width) + " |"] +
                      ["| " + " | ".join(map(cell, row)) + " |" for row in padded[1:]])


def unique(values):
    return list(dict.fromkeys(values))


def ignored(block: dict, override: dict) -> bool:
    if block["section"] in override.get("omit_manual_sections", []):
        return True
    text = block.get("text", "") + " ".join(" ".join(row) for row in block.get("grid", []))
    return any(fragment in text for fragment in override.get("omit_manual_text_contains", []))


def operand_descriptions(topics: list[dict], override: dict) -> dict[str, str]:
    result = {}
    for topic in topics:
        for block in topic["operands"]:
            if block["kind"] != "table" or ignored(block, override):
                continue
            for row in block["grid"][1:]:
                if len(row) == 3 and re.fullmatch(r"[A-Za-z][A-Za-z0-9_.]*", row[0]):
                    result.setdefault(row[0], row[1])
    return result


def flag_matrix(block: dict) -> list[list[str]]:
    """Select named flag columns from the manual's combined usage table.

    Keep command-group labels and native step counts separate from operand
    arity. Rowspan expansion has already copied the original shared values.
    """
    grid = block.get("grid", [])
    if len(grid) < 3:
        return []
    header = grid[1]
    flags = [i for i, text in enumerate(header) if re.search(r"F11[012]", text)]
    if not flags:
        return []
    steps = next((i for i, text in enumerate(header) if "스텝" in text), None)
    rows = [["Manual instruction group"] + (["Native steps"] if steps is not None else []) + [header[i] for i in flags]]
    for row in grid[2:]:
        if len(row) <= max(flags):
            continue
        values = [row[0]] + ([row[steps]] if steps is not None else []) + [row[i] for i in flags]
        if values not in rows[1:]:
            rows.append(values)
    return rows if len(rows) > 1 else []


def paired_execution_form(name: str, topics: list[dict], metadata: dict | None, pulse_definition: bool) -> str | None:
    if not pulse_definition or not metadata or metadata.get("kind") != "application":
        return None
    for topic in topics:
        names = {entry["mnemonic"] for entry in topic["instructions"]}
        for base in (name, name[:-1] if name.endswith("P") else ""):
            if not base or base not in names or base + "P" not in names:
                continue
            usage = " ".join(" ".join(row) for block in topic["usable_areas"]
                             if block["kind"] == "table" for row in block["grid"])
            if re.search(r"(?<![A-Za-z0-9$])" + re.escape(base) + r"\(P\)", usage):
                return "paired_pulse_type" if name == base + "P" else "paired_base"
    return None


class Figures:
    def __init__(self, manual: Path, output: Path):
        self.manual = manual.resolve()
        self.output = output
        self.assets: dict[str, dict] = {}

    def include(self, image: dict, page: str) -> str | None:
        relative = unquote(image.get("src", "")).split("#")[0]
        if not relative or ":" in relative:
            return None
        source = (self.manual / relative).resolve()
        if not source.is_relative_to(self.manual) or not source.is_file():
            raise ValueError(f"Missing or escaping manual figure: {page}:{relative}")
        data = source.read_bytes()
        if data.startswith(b"\x89PNG\r\n\x1a\n"):
            width, height = struct.unpack(">II", data[16:24])
            if min(width, height) <= 2:
                return None  # layout spacer, not an instruction figure
        name = f"{digest(data)[:16]}-{source.name}"
        target = self.output / "assets" / name
        if not target.exists():
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
        record = self.assets.setdefault("assets/" + name, {"sha256": digest(data), "sources": []})
        origin = {"page": page, "image": relative}
        if origin not in record["sources"]:
            record["sources"].append(origin)
        return "assets/" + name


def render(name: str, topics: list[dict], metadata: dict | None, override: dict,
           manual_hash: str, figures: Figures, pulse_definition: bool = False) -> tuple[str, dict]:
    chapters = unique(t["chapter"] for t in topics)
    pages = unique(t["source_page"] for t in topics)
    purpose = override.get("purpose_en")
    description = purpose or f"Use {name} with its XGK/XGB manual behavior, operands, and current XGWX editing constraints."
    fields = {"name": "xgk-" + slug(name), "description": description}
    source_metadata = {"manual": "XGK(B)InstructionHelp_Kr_V3.5.chm", "manual_sha256": manual_hash,
                       "manual_pages": pages}
    lines = ["---"] + [key + ": " + json.dumps(value, ensure_ascii=False) for key, value in fields.items()]
    lines += ["metadata:"] + ["  " + key + ": " + json.dumps(value, ensure_ascii=False) for key, value in source_metadata.items()]
    lines += ["---", "", f"# {name}", ""]
    if purpose:
        lines += [purpose, ""]
    if topics:
        lines += [f"Manual section(s): {', '.join(chapters)}. Original Korean notes below may cover the other variants on the same page; apply the widths and execution form of `{name}`.", ""]
    else:
        lines += ["This name exists in the current writer catalog but has no exact named instruction topic in the supplied manual. Its semantics remain undocumented here; related operand-rule pages do not establish its full behavior.", ""]

    execution_form = paired_execution_form(name, topics, metadata, pulse_definition)
    if execution_form:
        form = "the pulse-type P form" if execution_form == "paired_pulse_type" else "the non-P partner"
        lines += [f"Execution form: the source explicitly pairs this command as `(P)`; `{name}` is {form}. Manual §2.4.2 defines `P` as pulse-type notation. Select the paired forms deliberately and retain the instruction's actual execution conditions; unrelated mnemonics ending in P are not classified by suffix alone.", ""]

    lines += ["## Editing route", ""]
    route = metadata.get("writer_operation") if metadata else None
    if route:
        native = metadata["native_mnemonic"]
        lines += [f"Use `get_ir` → `apply_patch` for this instruction edit. `{route}` is the internal writer route, not a separate MCP tool. Preserve the exported revision and native constraints."]
        if metadata["kind"] == "diagram_element":
            lines += [f"Set the replacement element kind to `{metadata['element_kind']}`. `{name}` describes `{metadata.get('placement', 'cell')}` placement; it is represented by a physical cell rather than an application opcode."]
        else:
            lines += [f"The writer mnemonic is `{native}`; the catalog records opcode `{metadata['opcode']}` and `{metadata['operand_count']}` operands in order."]
        if metadata.get("placement") == "parallel":
            lines += ["OR placement requires an inspected supported parallel branch. The insertion tool does not construct the branch automatically; use `edit_ladder_branch` only where its guarded shape supports the requested circuit."]
        lines += ["Inspect the CPU/project and target cells first. Use the inspected revision and positions, preserve expected values, dry-run when useful, then save and inspect the result. See the [editing skill](../../SKILL.md) and [ladder workflow](../ladder.md).", ""]
    else:
        lines += ["No matching public writer route is exposed for this exact instruction form. Use this file to interpret existing code; preserve its records when editing other parts of a project. Manual documentation alone does not enable insertion.", ""]
        if metadata:
            lines += [*metadata.get("limitations", []), ""]

    if override.get("behavior_en"):
        lines += ["## Behavior", ""] + ["- " + value for value in override["behavior_en"]] + [""]
    if override.get("constraints_en"):
        lines += ["## Instruction constraints", ""] + ["- " + value for value in override["constraints_en"]] + [""]
    corrections = unique(entry["correction"] for topic in topics for entry in topic["instructions"]
                         if entry["mnemonic"] == name and entry.get("correction"))
    review_notes = unique(corrections + override.get("review_notes", []))
    if review_notes:
        lines += ["## Source corrections", ""] + ["- " + value for value in review_notes] + [""]

    descriptions = operand_descriptions(topics, override)
    rules = metadata.get("operand_rules", []) if metadata else []
    lines += ["## Operands", ""]
    if rules:
        rows = [["Order", "Operand", "Description (Korean manual)", "Reviewed types", "Device areas", "Constant"]]
        for index, rule in enumerate(rules):
            areas = rule["device_areas"]
            constant = rule["allows_constant"]
            rows.append([str(index + 1), rule["label"], descriptions.get(rule["label"], "Not unambiguously specified in this topic"),
                         "/".join(rule["data_types"]), "Unknown" if areas is None else ", ".join(areas) or "None",
                         "Unknown" if constant is None else "Allowed" if constant else "Not allowed"])
        lines += [table(rows), "", "These are the current library's reviewed operand rules. A shared type union is not a per-variant type choice; use the manual behavior to select the required width. Missing permissions stay unknown.", ""]
    else:
        raw = [block for topic in topics for block in topic["operands"]
               if block["kind"] == "table" and not ignored(block, override)]
        for block in raw:
            lines += [table(block["grid"]), ""]
        if not raw:
            count = metadata.get("operand_count") if metadata else None
            lines += ["This catalog form is operandless." if count == 0 else "No unambiguous operand table is available for this exact form; do not infer types or writable regions.", ""]
        else:
            lines += ["This source table is not a separately reviewed writer rule. Retain shared variant distinctions and let the current writer reject unsupported operands.", ""]

    model = metadata.get("cpu_restriction") if metadata else None
    lines += ["## CPU and acceptance", ""]
    if model is not None:
        lines += ["Reviewed XGK model table: " + (", ".join(model["models"]) or "no XGK model is allowed") + f" (`{model['manual_page']}`)."]
    else:
        lines += ["No separately reviewed CPU-model restriction is available in the current catalog; this means unknown, not universal compatibility."]
    lines += ["Check the applicable model, firmware, modules and memory spans stated in the manual notes. Local parsing, writer validation and IL conversion do not prove native compilation or PLC behavior. Native acceptance requires XG5000 Open → Check Program → Save As → comparison.", ""]

    for section, title in [("behavior", "Manual behavior notes (Korean)"),
                           ("flags", "Manual flags"), ("restrictions", "Manual applicability and restrictions")]:
        blocks = [block for topic in topics for block in topic[section] if not ignored(block, override)]
        texts = unique(block.get("text", "") for block in blocks if block["kind"] == "paragraph" and block.get("text"))
        grids = unique(json.dumps(block["grid"], ensure_ascii=False) for block in blocks
                       if block["kind"] == "table" and any(any(row) for row in block["grid"]))
        if texts or grids:
            lines += ["## " + title, ""]
            lines += ["- " + text for text in texts] + ([""] if texts else [])
            for grid in grids:
                lines += [table(json.loads(grid)), ""]

    matrices = unique(json.dumps(rows, ensure_ascii=False) for topic in topics for block in topic["usable_areas"]
                      if block["kind"] == "table" and not ignored(block, override) and (rows := flag_matrix(block)))
    if matrices:
        lines += ["## Manual flag and step matrix", "",
                  "The shared usage table marks affected flags with `O`, and `-` where it lists no effect. Native steps are the manual's instruction size, not the number of operand arguments. Conditions for setting/clearing flags are in the behavior notes.", ""]
        for matrix in matrices:
            lines += [table(json.loads(matrix)), ""]

    late_notes = unique(block["text"] for topic in topics for block in topic["examples"]
                        if block["kind"] == "paragraph" and len(block.get("text", "")) >= 15
                        and not ignored(block, override))
    if late_notes:
        lines += ["## Additional source notes after examples", "",
                  "The manual places some execution restrictions and timing conditions after its example heading. Retain those checks below. Example addresses and circuit assumptions describe the source example and are not defaults for a new program.", ""]
        lines += ["- " + text for text in late_notes] + [""]

    image_paths = []
    for topic in topics:
        for block in topic["blocks"]:
            if block["section"] == "examples" or ignored(block, override):
                continue
            image_items = [block] if block["kind"] == "image" else block.get("images", [])
            for image in image_items:
                if image.get("src") in override.get("omit_manual_images", []):
                    continue
                path = figures.include(image, topic["source_page"])
                if path:
                    image_paths.append(path)
    image_paths = unique(image_paths)
    if image_paths:
        lines += ["## Referenced manual figures", "", "These figures belong to the shared source topic. Read the source corrections above before using variant-specific layouts or example values.", ""]
        lines += [f"![{name} source figure {index + 1}]({path})\n" for index, path in enumerate(image_paths)]

    extra_pages = unique(override.get("additional_source_pages", []) +
                         [rule["manual_page"] for rule in rules] + ([model["manual_page"]] if model else []) +
                         (["245.htm"] if execution_form else []))
    lines += ["## Provenance", "", f"Source: `{source_metadata['manual']}`, SHA-256 `{manual_hash}`.", ""]
    for topic in topics:
        lines += [f"- `{topic['source_page']}` — §{topic['chapter']}; source SHA-256 `{topic['source_sha256']}`."]
    if extra_pages:
        lines += ["- Additional behavior/operand evidence: " + ", ".join(f"`{page}`" for page in extra_pages) + "."]
    record = {"mnemonic": name, "file": slug(name) + ".md", "manual_pages": pages,
              "chapters": chapters, "purpose": purpose,
              "aliases": unique([entry.get("title_mnemonic", name) for topic in topics for entry in topic["instructions"] if entry["mnemonic"] == name] + (metadata.get("aliases", []) if metadata else [])),
              "writer_mnemonic": metadata.get("native_mnemonic") if route else None,
              "writer_operation": route, "writer_kind": metadata.get("kind") if metadata else None,
              "coverage": "manual_topic" if topics else "catalog_only",
              "execution_form": execution_form,
              "reviewed_english_notes": bool(override), "source_corrections": review_notes,
              "figures": image_paths}
    return "\n".join(lines).rstrip() + "\n", record


def generate(manual_directory: Path, library: Path, binary: Path | None,
             overrides_path: Path, manual_hash: str, output: Path) -> dict:
    inventory = load_manual(manual_directory, manual_sha256=manual_hash)
    support = load_support(library, binary)
    overrides_bytes = overrides_path.read_bytes()
    overrides = json.loads(overrides_bytes)
    pulse_page = manual_directory / "245.htm"
    pulse_definition = False
    if pulse_page.is_file():
        document = Document()
        document.feed(pulse_page.read_bytes().decode("cp949"))
        pulse_definition = "P : 펄스 타입 명령" in text_content(document.root)
    by_name: dict[str, list[dict]] = defaultdict(list)
    for topic in inventory["topics"]:
        for entry in topic["instructions"]:
            by_name[entry["mnemonic"]].append(topic)
    primary = {name: resolve_support(name, support) for name in by_name}
    canonical_guides = {}
    for name in sorted(by_name, key=lambda n: (not n.startswith(("LOAD", "ULOAD")), len(n), n)):
        metadata = primary[name]
        if metadata and metadata.get("writer_operation"):
            canonical_guides.setdefault(metadata["native_mnemonic"], name)
    core = {name: value for name, value in support.items()
            if value["kind"] in {"application", "comparison"} and not value.get("comparison_alias")}
    for name, metadata in core.items():
        if name not in canonical_guides and name not in by_name:
            by_name[name] = []
            primary[name] = metadata
            canonical_guides[name] = name

    output.mkdir(parents=True, exist_ok=True)
    figures = Figures(manual_directory, output)
    records, slugs = {}, {}
    for name in sorted(by_name):
        path = slug(name) + ".md"
        if path in slugs:
            raise ValueError(f"Instruction filename collision: {name} and {slugs[path]}")
        slugs[path] = name
        markdown, record = render(name, by_name[name], primary[name], overrides.get(name, {}), manual_hash, figures, pulse_definition)
        (output / path).write_text(markdown, encoding="utf-8")
        records[name] = record
    aliases = {}
    for name, record in records.items():
        for alias in record["aliases"]:
            if alias not in records:
                aliases.setdefault(alias, name)
    for name, preferred in canonical_guides.items():
        if name not in records:
            aliases[name] = preferred

    sources = {topic["source_page"]: {"chapter": topic["chapter"], "title": topic["title"],
               "sha256": topic["source_sha256"]} for topic in inventory["topics"]}
    if pulse_definition:
        sources["245.htm"] = {"chapter": "2.4.2", "title": "니모닉 생성 규칙", "sha256": digest(pulse_page.read_bytes())}
    provenance = {path: value for metadata in support.values()
                  for path, value in metadata.get("provenance", {}).get("source_sha256", {}).items()}
    manifest = {"format_version": 1, "manual": inventory["manual"], "manual_sha256": manual_hash,
                "encoding": "cp949", "manual_topic_count": inventory["topic_count"],
                "manual_mnemonic_count": inventory["mnemonic_count"], "guide_count": len(records),
                "core_catalog_count": len(core), "source_pages": sources,
                "library_source_sha256": provenance, "overrides_sha256": digest(overrides_bytes),
                "instructions": records, "aliases": dict(sorted(aliases.items())), "assets": figures.assets}
    index = ["# XGK/XGB instruction files", "",
             f"{inventory['mnemonic_count']} manual instruction spellings from {inventory['topic_count']} detail topics. Every current catalog instruction resolves to a guide; entries without an exact manual topic are marked `catalog_only`.", "",
             "Read only the requested mnemonic's file. Use `scripts/find_instruction.py` from the installed xgwx-edit skill for exact lookup or text search; comparison symbols have safe filenames (`eq`, `ne`, `gt`, `ge`, `lt`, `le`). Original manual and native writer spellings are indexed separately.", "",
             "Guides retain Korean operation notes and figures, with separately reviewed English explanations where available. Known copied tables or text are omitted and identified. Operand/type/device rules and CPU-model evidence come from the current library, optionally verified through real MCP stdio. Unknown information remains unknown.", "",
             f"Source: `{inventory['manual']}`; SHA-256 `{manual_hash}`. The CHM and raw HTML are build inputs, not included in this skill.", "",
             "| Manual mnemonic | Guide | Section | Writer operation |",
             "| --- | --- | --- | --- |"]
    for name, record in records.items():
        index.append(f"| `{cell(name)}` | [{name}]({record['file']}) | {', '.join(record['chapters']) or 'catalog only'} | `{record['writer_operation']}` |".replace("`None`", "Unavailable"))
    (output / "index.md").write_text("\n".join(index) + "\n", encoding="utf-8")
    files = {path.relative_to(output).as_posix(): digest(path.read_bytes())
             for path in sorted(output.rglob("*")) if path.is_file()}
    manifest["generated_files_sha256"] = files
    (output / "manifest.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return manifest


def install(stage: Path, output: Path, check: bool) -> None:
    expected = {path.relative_to(stage).as_posix(): path for path in stage.rglob("*") if path.is_file()}
    changed = [relative for relative, source in expected.items()
               if not (output / relative).is_file() or source.read_bytes() != (output / relative).read_bytes()]
    previous = json.loads((output / "manifest.json").read_text()) if (output / "manifest.json").exists() else {}
    previous_files = previous.get("generated_files_sha256", {})
    for relative in previous_files:
        path = Path(relative)
        if path.is_absolute() or ".." in path.parts:
            raise ValueError(f"Escaping generated-file path in existing manifest: {relative}")
    stale = set(previous_files) - set(expected)
    if check:
        if changed or stale:
            raise ValueError(f"Generated instruction files differ: {len(changed)} changed/missing, {len(stale)} stale")
        return
    for relative in changed:
        target = output / relative
        if target.exists() and relative != "manifest.json":
            old_hash = previous_files.get(relative)
            if old_hash is None or digest(target.read_bytes()) != old_hash:
                raise ValueError(f"Refusing to overwrite independently changed instruction file: {target}")
    for relative in stale:
        target = output / relative
        if target.exists() and digest(target.read_bytes()) != previous_files[relative]:
            raise ValueError(f"Refusing to remove independently changed generated file: {target}")
    for relative, source in expected.items():
        target = output / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, target)
    for relative in stale:
        (output / relative).unlink(missing_ok=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    source = parser.add_mutually_exclusive_group(required=True)
    source.add_argument("--manual", type=Path, help="original CHM (requires 7z)")
    source.add_argument("--manual-dir", type=Path, help="locally extracted CHM HTML directory")
    parser.add_argument("--manual-sha256", default=EXPECTED_MANUAL_SHA256)
    parser.add_argument("--library-root", type=Path, default=None)
    parser.add_argument("--server-binary", type=Path)
    parser.add_argument("--overrides", type=Path, default=ROOT / "scripts/instruction_overrides.json")
    parser.add_argument("--output", type=Path, default=ROOT / "skills/xgwx-edit/references/instructions")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    with tempfile.TemporaryDirectory(prefix="xgwx-instruction-build-") as directory:
        temporary = Path(directory)
        if args.manual:
            manual_hash = digest(args.manual.read_bytes())
            if manual_hash != args.manual_sha256:
                parser.error("CHM hash differs; explicitly provide --manual-sha256 for another manual version")
            extracted = temporary / "manual"
            subprocess.run(["7z", "x", str(args.manual.resolve()), "-o" + str(extracted), "-y", "-bso0", "-bsp0"], check=True)
        else:
            extracted = args.manual_dir
            manual_hash = args.manual_sha256
        stage = temporary / "guides"
        manifest = generate(extracted, args.library_root or resolve_library_root(), args.server_binary, args.overrides, manual_hash, stage)
        install(stage, args.output, args.check)
        print(json.dumps({"guides": manifest["guide_count"], "manual_names": manifest["manual_mnemonic_count"],
                          "catalog_entries": manifest["core_catalog_count"], "figures": len(manifest["assets"]),
                          "check": args.check}, ensure_ascii=False))


if __name__ == "__main__":
    main()
