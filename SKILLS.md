# Editing XGWX files with AI agents

Use `xgwx-mcp` to inspect a local LS XG5000 project, discover the library's supported edits, and save a verified transaction. Each changed save retains a backup of the original. The server uses the official Rust MCP SDK and `libxgwx`; it does not connect to a PLC.

## Installation

Build the server with Rust 1.88 or newer. Cargo fetches the pinned library dependency; no sibling checkout is required:

```bash
cd /path/to/xgwx-mcp
cargo build --release --locked
```

Install the portable skill into a skill-aware agent's skill directory. For Codex, this command copies the entire skill and its references:

```bash
mkdir -p "${CODEX_HOME:-$HOME/.codex}/skills"
cp -R skills/xgwx-edit "${CODEX_HOME:-$HOME/.codex}/skills/"
```

If that skill directory already exists, review it before replacing it. Installing the skill does not register the MCP server. Configure the MCP client separately using an absolute binary path and an allowed project root. A Codex TOML example is:

```toml
[mcp_servers.xgwx]
command = "/path/to/xgwx-mcp/target/release/xgwx-mcp"
args = ["--root", "/path/to/projects"]
```

A client using the common JSON MCP configuration shape can use:

```json
{
  "mcpServers": {
    "xgwx": {
      "command": "/path/to/xgwx-mcp/target/release/xgwx-mcp",
      "args": ["--root", "/path/to/projects"]
    }
  }
}
```

Adapt these snippets to the client's supported configuration. This repository does not alter client settings automatically. The server's startup directory is always allowed; `--root` extends it. Start the process in a suitably bounded directory. Relative file paths are relative to that startup directory, while canonical paths and symlink targets must stay within allowed roots.

## Create a new project

Call `list_templates` without a file path to discover native `xgk_ld`, `xgi_ld`
`xgi_sfc`, `xgi_st` and `xgi_il` seeds and supported CPU/language `creation_choices`. Then preview and save:

```json
{"project_name":"Machine Control","path":"/path/to/projects/machine.xgwx","cpu_model":"XGK-CPUH","program_language":"LD","program_name":"Main","dry_run":true}
```

Repeat with `dry_run:false` to create the verified file. The parent must exist
inside an allowed root, and the destination must be absent; creation never
overwrites files, directories or symlinks. Optional ordered `operations` can
rename program 0 or populate its blank ladder through the existing writers.
Choose project_name and optional program_name directly; CPU/language selects the blank automatically. Only advertised LD/SFC/ST/IL combinations are supported. Omitted names retain native defaults; internal seed identities and PLC defaults are retained.
Creation returns `revision_after`, validation and results, with no original or
backup. Inspect the saved file before preparing further edits; subsequent changed
edits create backups normally. Read the portable
[creation reference](skills/xgwx-edit/references/create-file.md) for CPU guards,
permissions, dry runs and error handling. `create_program` edits an existing
project and does not create a filesystem file.

## Discover and inspect

Begin by reading the project and its revision:

```json
{"path":"/path/to/projects/machine.xgwx","section":"overview"}
```

Pass this to `inspect_file`. It returns `{path,revision,data}`. Use `programs` to identify program indices and identities, then select details with `program_index`:

```json
{"path":"/path/to/projects/machine.xgwx","section":"program.edit_sites","program_index":0}
```

Other sections include `program`, `symbols`, `hardware`, `networks`, `sfc`, `sfc.variables`, `sfc.block.<block_index>`, `io_variables`, `program.rows`, `program.records`, `program.cells`, `program.strings`, `program.functions`, `program.references`, `program.operand_links`, `program.graph`, and CPU/module/instruction/comparison catalogs under `catalog.*`. Array results are paged as `{items,total,offset,limit,next_offset}`; `limit` is 1–1000 and defaults to 100. Follow `next_offset` using `offset` until null when a complete snapshot is needed; use a consistent revision across pages. Supply `program_index` for paging local symbols or SFC snapshots. For complete SFC entities/rows use a targeted block section such as `sfc.block.0`; for declarations use `sfc.variables`.

Discover operation names through `list_operations`, optionally with `query` or `subsystem`, and obtain exact arguments through `describe_operation`:

```json
{"query":"update_module"}
```

```json
{"name":"update_module"}
```

`describe_operation` distinguishes executable `fixture_verified` examples from `argument_template` examples. A verified example includes the fixture hash and ordered setup operations required before its arguments apply; these positions and identities are not defaults for your project. `inspection_requirements` identifies exact source fields for common workflows.

Operation arguments use the public Rust signature names in snake_case. Nested `patch` and `edit` values follow the returned schema; enums retain native names such as `NormallyOpen` or `Rung`. Native XML property-map keys retain their spelling. Do not copy camelCase WASM examples directly into the MCP interface. Compatibility aliases identify their shared handler in the operation description.

## Focused discovery and property patches

Use `list_modules` for installed hardware, `list_module_catalog` for available
models/options, `list_variables` for globals/IEC locals, and `list_networks` for
configured networks and Cnet/FEnet settings. File readers return revisions;
arrays paginate independently. Catalog discovery needs no project file.

Call `get_ir(path, section:"modules"|"variables"|"networks")`, optionally selecting
program_index for variables. Change only marked `edit` values; preserve readonly
fields, object identities and ordering. Apply an exact unified diff with the
same section/selection and expected_revision. Module comments/options, global
metadata, IEC local names/types/descriptions, network metadata and exported
Cnet/FEnet settings lower through guarded public writers. Local rename/type
changes require a sole mutation and renewed export. Read
[property-patches.md](skills/xgwx-edit/references/property-patches.md) before editing.

## Patch positional ladder text

Use `get_ir` with `path` and `program_index` to export a program as `xgwx-ir/1` text. Send an exact unified diff through `apply_patch`, with the same path/program index, exported `expected_revision`, and optional `dry_run`. This supports SFC linear row creation/deletion/reordering and existing row fields, guarded direct entity comments/conditions and declaration descriptions, plus XGK contact/coil and catalog instruction/comparison creation/deletion and instruction mnemonic/operand edits, IEC empty-row/terminal-coil creation and isolated/connected-contact and terminal-coil deletion, physical-wire deletion and supported vertical-wire creation, XGK/IEC contact and coil changes, bound IEC function operand edits and captured arithmetic/comparison name changes, native comment text, symbol descriptions, and XGK comment creation/rung-comment deletion. Native positions and connectivity remain explicit; unsupported moves and derived connection changes are rejected. Instruction creation/deletion/arity changes, connected IEC deletion and wire changes must be the sole mutation in a patch; commit and re-export before dependent edits. Read the portable [text IR reference](skills/xgwx-edit/references/text-ir.md) for syntax, comment semantics and examples.

Patch edits use the same immediate-save, backup and verified transaction behavior as `apply_edits`. The server derives current source offsets and expected values under its locks. Re-export after saves rather than guessing identities or reusing offsets.

## Save an inspected change

For a module uniquely inspected at base 0, slot 2, an example `apply_edits` request is:

```json
{
  "path": "/path/to/projects/machine.xgwx",
  "expected_revision": "<revision from inspect_file>",
  "operations": [
    {
      "name": "update_module",
      "args": {"base":0,"slot":2,"patch":{"comment":"Feed conveyor"}}
    }
  ],
  "dry_run": true
}
```

Replace the example path, position, text, and revision with inspected values. Dry-run returns candidate validation and a candidate `revision_after` without changing the source or creating a backup. To save the reviewed candidate, repeat the same request with `dry_run:false`, retaining the original inspected `expected_revision`. A successful changed save returns `backup_path` and the new revision. No-op requests create no backup.

Operations in a request run in order on one in-memory document. Any failed operation or verification aborts the whole request. Source files are replaced atomically after a verified candidate and unique adjacent backup are ready. File permissions are preserved. Backups use `<filename>.xgwx-mcp-backup-<random suffix>` and remain until explicitly removed. A persistent `<filename>.xgwx-mcp.lock` sidecar coordinates cooperating server processes; coordinate writes with external editors that do not honor it.

Structured errors identify the failure code/message and failing operation index when available. If saving failed after a backup was retained, the error also returns `backup_path`; keep that recovery artifact and report it.

The server rejects stale revisions. Re-inspect and reconsider the change when that happens; do not substitute a new revision into stale coordinates, indices, or expected values. Re-inspect after program reorder/deletion, symbol edits that change indices, row/group changes, or any operation that moves offsets. Put dependent structural edits in separate requests unless their intermediate indices and expected values are established.

`validate_file` accepts `{"path":"/path/to/projects/machine.xgwx"}`. Its `{path,revision,validation}` result separates container verification from available decoded-program diagnostics. Unsupported decode or IL topology is reported as a limitation. A serialized incomplete circuit or SFC chart can still need completion before native checks pass.

## Choose the workflow

For a specific XGK/XGB instruction, use the [instruction index](skills/xgwx-edit/references/instructions/index.md) or the portable lookup helper:

```bash
python3 skills/xgwx-edit/scripts/find_instruction.py 'ULOADD<'
```

Read only the returned file. It preserves the manual spelling and separately identifies the native writer mnemonic, so `ULOADD<` uses `UD<` in `insert_ladder_comparison`. The files include reviewed operands and English notes, Korean behavior and late execution restrictions, relevant figures, and known source corrections. Catalog-only entries and unsupported specialized/control records remain explicit. These references travel with the skill when it is installed.

The installable skill contains focused guidance and example requests:

- [Programs and symbols](skills/xgwx-edit/references/programs-and-symbols.md): metadata, create/delete/reorder, local declarations, typed operands.
- [Ladder](skills/xgwx-edit/references/ladder.md): XGK physical cells, IEC edit sites, functions, wires, and topology.
- [Hardware and networks](skills/xgwx-edit/references/hardware-and-networks.md): CPU selection, modules/options, Cnet and FEnet fields.
- [SFC](skills/xgwx-edit/references/sfc.md): entity changes, complete chart snapshots, branches/actions, and declarations.

For native acceptance, open the generated project in XG5000, inspect the changed area, run Check Program, Save As to a separate file, then inspect/compare that resave. Record errors and warnings and check retained program payloads and unrelated metadata. Local round-trip verification is a separate result; never label it native XG5000 acceptance. No PLC download or execution is part of this workflow.

## ST/IL source programs

Use `create_file(program_language:"ST"|"IL")` for supported XGI CPU choices.
Inspect `text` with the selected `program_index`; discover schemas for
`edit_text_program` and `edit_text_variable`. Source edits retain ObjectId,
language and exact expected-source checks. Declarations require a complete
expected_variables snapshot. `get_ir`/`apply_patch` supports quoted source lines
with exact newline preservation. Read the [ST/IL reference](skills/xgwx-edit/references/st-il.md)
for examples and constraints. `validate_file` reports decoding, not source compilation.
