#!/usr/bin/env python3
"""Locate individual instruction guides without loading the entire index."""
import argparse
import json
from pathlib import Path


def lookup(manifest, name):
    name = " ".join(name.upper().split())
    actual = {key.upper(): key for key in manifest["instructions"]}
    aliases = {key.upper(): value for key, value in manifest["aliases"].items()}
    key = actual.get(name) or aliases.get(name)
    return manifest["instructions"].get(key) if key else None


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mnemonic", nargs="?")
    parser.add_argument("--query", help="search mnemonic, purpose and original aliases")
    parser.add_argument("--limit", type=int, default=20)
    args = parser.parse_args()
    if not args.mnemonic and not args.query:
        parser.error("provide a mnemonic or --query")
    if not 1 <= args.limit <= 1000:
        parser.error("--limit must be 1..1000")
    base = Path(__file__).resolve().parents[1] / "references/instructions"
    manifest = json.loads((base / "manifest.json").read_text(encoding="utf-8"))
    if args.mnemonic:
        record = lookup(manifest, args.mnemonic)
        if record is None:
            parser.exit(1, f"No instruction guide for {args.mnemonic!r}; try --query.\n")
        print(json.dumps({**record, "path": str(base / record["file"])}, ensure_ascii=False, indent=2))
    else:
        query = args.query.casefold()
        records = [record for record in manifest["instructions"].values()
                   if query in " ".join([record["mnemonic"], record.get("purpose") or "", *record["aliases"]]).casefold()]
        print(json.dumps({"total": len(records), "items": [{"mnemonic": r["mnemonic"], "file": r["file"],
                          "purpose": r["purpose"], "writer_operation": r["writer_operation"]} for r in records[:args.limit]]}, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
