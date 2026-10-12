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

If the target is known by name, address, description, operand, mnemonic or comment,
use `search_project` before loading detailed sections:

```json
{"path":"/path/to/projects/machine.xgwx","query":"Feed conveyor","kind":"contact"}
```

Kinds are optional: `program`, `variable`, `contact`, `coil`, `instruction`,
`comment`. Match is a case-insensitive literal substring by default. Results
include matched fields, linked symbol descriptions, revision-scoped locations
and explicit coverage limitations. `program_index` filters programs but keeps
globals. Follow the single result list's `next_offset` with `expected_revision`
from the first page. On `stale_revision`, restart; do not combine pages.
SFC/ST/IL bodies and declarations require their existing inspection sections.
Search locations help select a target; inspect edit sites or export `get_ir`
before editing, since a decoded match does not establish writability.

Discover operation names through `list_operations`, optionally with `query` or `subsystem`, and obtain exact arguments through `describe_operation`:

```json
{"query":"update_module"}
```

```json
{"name":"update_module"}
```

`describe_operation` distinguishes executable `fixture_verified` examples from `argument_template` examples. A verified example includes the fixture hash and ordered setup operations required before its arguments apply; these positions and identities are not defaults for your project. `inspection_requirements` identifies exact source fields for common workflows.

Operation arguments use the public Rust signature names in snake_case. Nested `patch` and `edit` values follow the returned schema; enums retain native names such as `NormallyOpen` or `Rung`. Native XML property-map keys retain their spelling. Do not copy camelCase WASM examples directly into the MCP interface. Compatibility aliases identify their shared handler in the operation description.

## IR syntax and editing guide

IR is the text returned by `get_ir`; it is a revision-scoped view of an XGWX
file. Prepare edits against that exact text and send a unified diff to
`apply_patch`. Editing an exported `.ir` file alone does not update the project.
The server translates supported differences into guarded library operations.

Choose the export for the requested change:

| Change | `get_ir` selection | Format |
| --- | --- | --- |
| Module comments or supported options | `section:"modules"` | `xgwx-properties/1` |
| Global or IEC local variable metadata and addresses | `section:"variables"`, optional `program_index` for locals | `xgwx-properties/1` |
| Supported network configuration fields | `section:"networks"` | `xgwx-properties/1` |
| Ladder elements, instructions, wires or comments; SFC charts; ST/IL source | `section:"program"`, required `program_index` | `xgwx-ir/1` |

Variable exports always include globals. Selecting `program_index` limits the
IEC local tables. A program export's symbol statements support descriptions;
use the variables export for supported declaration names, types and addresses.

### Property syntax

An illustrative local-variable block looks like this:

```text
variable @p0.v5 {
  edit name = "ON"
  edit address = "%MX8"
  readonly storage_class = "M"
  readonly allocation_number = 8
  readonly allocation_width = 1
  readonly is_instance = false
}
```

- `edit` permits changing the value after ` = `, subject to native writer
  constraints. Keep the marker and field name unchanged.
- `readonly` provides context or derived metadata. Keep the entire line
  unchanged; changing its marker to `edit` does not grant edit access.
- Strings use JSON quoting and escapes: `"Pump"`, `""`, `"Line 1\nLine 2"`.
  Keep Unicode text intact. Numeric values use unsigned decimal integers through
  `4294967295`; preserve the original scalar type. Boolean and `none` fields
  currently remain readonly. Do not replace strings with numbers or `null`.
- `none` represents a missing value. It is not a command to clear a field.
  Supported IEC local addresses instead use the string `""` for no explicit
  mapping; patch an existing address to `""` to clear it, or replace `""` with a
  valid address to assign one. Instance and unsupported addresses stay readonly.
- Identities such as `@global.v12`, `@p0.v5` and `@m0` locate objects within this
  export. They are not names to edit or reusable IDs across revisions. Preserve
  headers, braces, object order, field order and all readonly values.

IEC local name, type or address changes must be the sole mutation in a patch.
Re-export after each one. The address writer updates storage/allocation metadata
internally and enforces supported areas, type widths and overlap rules; never
adjust the derived readonly fields yourself. Global metadata changes for one
variable can share a patch and are combined into one writer operation.
Module model changes and property object creation/deletion require supported
`apply_edits` operations rather than property diffs.

### Program syntax

Program text uses statements rather than the property format's `edit` markers:

```text
    contact @p0.e120 at(x=1,y=0) NO "M00000"
    coil @p0.e160 at(x=94,y=0) OUT "P00000"
```

These XGK examples identify native elements, positions, kinds and quoted
operands. Supported kinds and operands can change; retained identities and
coordinates cannot. IEC exports use their own group/row coordinates; do not
translate coordinates between dialects. Statements explicitly marked `readonly`,
opaque records, derived connections and unsupported fields must remain intact.
Absence of a `readonly` marker does not make every part of a statement editable.

IEC semantic statements can expose decoded text while remaining readonly:

```text
  contact @p0.e223 at(group=0,row=2,x=1) RISING "시작_1" decode "decoded" reason "writer_string_unavailable" hash "<record SHA-256>" readonly
```

Here `decode "decoded"` means the native text field was decoded, not that the
whole circuit was validated or the element is writable. `readonly` freezes the
entire statement, including its operand, metadata and hash. This also applies to
readonly `argument` and `annotation` statements. Text is JSON-escaped, so Korean
names, empty strings and multiline comments remain visible without changing
native records or enabling additional writes.

`reason "writer_string_unavailable"` means the text is outside the current
writer-facing string list; `writer_layout_unsupported` identifies a readable
annotation without a supported comment edit layout. A `FunctionOperand` record
can expose `value` with `function_binding_unavailable` when its text is known
but a target pin cannot be resolved. Never invent a target from coordinates.
Remaining records use `decode "unavailable"` with a reason when semantic text or
function decoding is unavailable, or `decode "not_applicable"` for structural
records without such a text field. Hashes preserve provenance, not meaning.

Supported element creation uses a fresh `@new-<name>` identity at an allowed
native position; after saving, re-export to obtain source-backed identities.
Instruction creation/deletion/arity changes, connected IEC deletion and wire
changes must be the sole mutation. SFC row changes also require a sole chart
operation; preserve retained positions and branch structure. Consult the
[text IR reference](skills/xgwx-edit/references/text-ir.md) for the specific
statement's supported fields and creation/deletion rules before structural edits.

IEC functions also have readonly metadata alongside editable function names
and argument values:

- `call` identifies the function node, FUNCTION/FUNCTION_BLOCK class, native
  instance name, linked local symbol and declared instance type. Instance lookup
  is exact and unique; `instance_status` reports missing, ambiguous or mismatched
  declarations. `state INSTANCE` means an instance marker is encoded; `state NONE`
  means none is encoded. Neither describes runtime output retention or purity.
- `symbol_instance` exposes an instance declaration's `type_reference`, including
  declarations without a decoded call. The variables export also includes this
  reference as readonly metadata.
- `pin` identifies its owner, native position, name, CONTROL/DATA role, direction,
  ordinal, declared type/expression, type mask, raw flags and array flag.
  `selected_type` is concrete only for a single known type bit. Broad constraints
  such as ANY/ANY_NUM use `selected_type none` and `type_status "constraint_only"`.
- `binding` connects a native reference record to a function and an explicit pin
  ID, plus an argument record when resolved. `argument_status "no_expression"`
  means the decoded reference has no expression record; `binding_unavailable`
  means argument-link decoding failed. Neither implies an empty operand value.
- `decode_status` reports function-body, reference and argument-link decoding
  separately; readable text does not imply complete graph coverage.

These identities and links are revision-scoped. Keep the metadata unchanged
when editing supported function names or argument values; a fresh export after
saving recomputes it. Never infer concrete types, instance state or execution
behavior from coordinates or an unresolved link.

### Typed operands and declaration ownership

Readonly `operand` statements describe each exposed contact/coil operand,
function argument or catalog instruction operand. `owner` links to its source
statement and `index` is zero-based (zero for a single operand). The original
quoted value remains on the editable statement; change that value, preserve its
readonly metadata, then re-export to obtain the updated classification.
Native operandless XGK logic markers have no operand metadata.

The operand's `text` is followed by `address`, so a resolved variable shows its
mapping beside its name, for example `text "ON" address "%MX8"`. `address none`
means no explicit mapping is known, including automatic variables, ambiguous
names and instance fields without a captured direct address. Direct-address
operands repeat their spelling in `address`; this does not validate the mapping.
Addresses are derived readonly context. Change a variable's address through the
variables property IR, then re-export the program to refresh this display.

| Category | Meaning and resolution boundary |
| --- | --- |
| SYMBOL | A declaration name; local names shadow global names. A unique match provides `symbol` and its declared type. Duplicate names report ambiguous candidates. |
| DIRECT_ADDRESS | Recognized IEC address spelling, or an XGK address matching an exported declaration. Candidates retain all exact address matches, without selecting a symbol or inferring memory aliases. |
| FIELD | A declaration plus member. Native instance pins can resolve to a symbol and pin ID; undecoded members remain unresolved. |
| SYSTEM_SYMBOL | A name in the library's IEC BOOL system-symbol catalog. Other underscore-prefixed names are not presumed to be system symbols. |
| LITERAL | A recognized Boolean, decimal integer/real or IEC temporal prefix. Untyped numbers do not receive an invented concrete IEC type. |
| UNRESOLVED | Empty values, missing declarations or unsupported address/literal/compound-expression forms. Keep their exact text and inspect the reason. |

`status "resolved"` establishes a declaration/catalog/native-pin link, not
runtime correctness. `syntax_only` identifies address or simple literal syntax
without checking CPU memory bounds, pin compatibility or writability. Temporal
prefixes such as `T#` use `unvalidated`: the payload's native temporal grammar
has not been validated by this metadata layer. The existing writers remain the
validation authority. Compound expressions, array references and additional
literal forms remain unresolved rather than being guessed.

References point to this program export: globals use `@sN`, locals use `@pN.sM`.
The corresponding variables property blocks are `@global.vN` and `@pN.vM` at the
same revision and indices. Both exports are views of the original XGWX document;
editing one exported text file does not update another. Re-export both after a
save. Address candidates are spelling matches, not evidence that two direct
addresses occupy different or overlapping physical memory.

Rename behavior follows the selected native writer:

- IEC local rename updates the declaration, captured exact operand-string
  matches and matching native instance markers in that program. It is not a
  general expression refactor: member strings such as `Timer.Q` and compound
  expressions are not rewritten merely because they contain the old name.
  Inspect those references separately; never assume rename propagation.
- Global `update_variable` name changes update the global table and preserve
  program payloads. They do not automatically rewrite program operand strings.
- Changing one contact/coil operand or argument edits that use only; it does not
  rename the referenced declaration or its other uses.

### Connection endpoints and execution context

IEC program exports retain the coordinate-based `connection` statements and add
readonly logical endpoint/provenance metadata:

- `junction` assigns an ID to a native electrical boundary point, including its
  group/row/x position, rail and reported open-endpoint status.
- `edge` links a connection to its from/to junctions, native source record
  offset, optional paired record offset, source element/function/physical wire
  ID, and terminal IDs. `source_status "record_only"` retains byte provenance
  when no corresponding exported source object can be identified.
- `terminal` associates a contact/coil input or output with its junction.
  Function-enable edges reference the already exported control pin IDs.
- `pin_junction` links a native function binding and named pin to the junction
  decoded by the library. Pin-cell anchors and junction boundary coordinates
  are different; use these links rather than matching raw coordinates yourself.

These edges describe stored circuit structure. Contacts and function-enable
edges are conditional; sharing a graph or junction does not prove an active
runtime path. Geometry failures stay explicit and do not produce invented
endpoint links. XGK exports retain their native positional wire representation;
this endpoint model currently applies to decoded IEC geometry.

All property and supported program exports include readonly project context:

- `cpu` retains the native type code and catalog model/family, or unknown status.
- `task` preserves names, native indices, priorities, trigger-device/condition
  fields and raw attributes. Interpret their native values with CPU-specific
  evidence; do not treat priority as a complete scheduling model.
- `program_context` states the selected program's language, native kind,
  document index and task name/ID. Task lookup requires a unique exact match;
  unresolved or ambiguous assignments remain explicit.
- `context` marks memory-model and runtime execution order as not decoded.
  Document indices are export positions, not a guaranteed PLC execution order.

Retain all context and endpoint metadata during patches. CPU changes, task
changes and memory aliases must follow supported native operations and inspected
constraints; this metadata is not an editable scheduling or memory model.

For ST/IL, `source { ... }` contains quoted `source` statements whose decoded
strings concatenate into the native source. Preserve escapes such as `\r\n`
when retaining line endings; do not insert literal newlines inside a quoted
statement. Source statements may be replaced, added or deleted, while program
identity, dialect and readonly metadata remain fixed.

### Construct and submit a patch

1. Call `get_ir` for the selected path, section and program. Keep its `revision`
   and complete `text` together; use the returned capabilities and limitations.
2. Copy the text and change only the requested supported values. Generate an
   exact unified diff between the original and modified copies. Hunk positions
   are one-based, counts include context, and every removed/context line must
   match exactly. Do not reformat unrelated lines or use fuzzy matching.
3. Call `apply_patch` with the same `path`, `section`, `program_index` selection,
   the exported revision as `expected_revision`, the diff as `patch`, and
   `dry_run:true` for a preview. Supply the diff, not the whole rewritten IR.
4. To save, repeat with `dry_run:false`, retaining the original source revision
   and diff. The preview's candidate `revision_after` is not the source revision.
   A successful changed save returns the new revision and backup path.
5. Re-export before the next edit. On a stale revision or rejected edit, inspect
   and rebuild the patch; do not substitute a new hash into an old diff or bypass
   guards by editing native XML/binary data.

For this illustrative six-line export, `address` is line 4:

```text
xgwx-properties 1 revision "<exported SHA-256>" section variables program 0
variable @p0.v5 {
  edit name = "ON"
  edit address = "%MX8"
  readonly allocation_number = 8
}
```

A single-field address diff is:

```diff
--- variables
+++ variables
@@ -4,1 +4,1 @@
-  edit address = "%MX8"
+  edit address = "%MX100"
```

Submit it as the following `apply_patch` arguments. Example identities, line
numbers and values are illustrative; derive them from the actual complete export.
The `---`/`+++` labels are inert and do not select filesystem paths. Do not use
`*** Begin Patch` wrappers, Markdown fences or prose inside the `patch` string.

```json
{
  "path": "/path/to/projects/machine.xgwx",
  "section": "variables",
  "program_index": 0,
  "expected_revision": "<exported SHA-256>",
  "patch": "--- variables\n+++ variables\n@@ -4,1 +4,1 @@\n-  edit address = \"%MX8\"\n+  edit address = \"%MX100\"\n",
  "dry_run": true
}
```

Empty or semantically unchanged patches are no-ops. A failed writer or
verification aborts the transaction. Local validation verifies serialization
and available diagnostics; native syntax/type acceptance still requires XG5000.

## Focused discovery and property patches

Use `list_modules` for installed hardware, `list_module_catalog` for available
models/options, `list_variables` for globals/IEC locals, and `list_networks` for
configured networks and Cnet/FEnet settings. File readers return revisions;
arrays paginate independently. Catalog discovery needs no project file.

Call `get_ir(path, section:"modules"|"variables"|"networks")`, optionally selecting
program_index for variables. Change only marked `edit` values; preserve readonly
fields, object identities and ordering. Apply an exact unified diff with the
same section/selection and expected_revision. Module comments/options, global
metadata, IEC local names/types/addresses/descriptions, network metadata and exported
Cnet/FEnet settings lower through guarded public writers. Local rename/type/address
changes require a sole mutation and renewed export. Read
[property-patches.md](skills/xgwx-edit/references/property-patches.md) before editing.

## Patch positional ladder text

Use `get_ir` with `path` and `program_index` to export a program as `xgwx-ir/1` text. Send an exact unified diff through `apply_patch`, with the same path/program index, exported `expected_revision`, and optional `dry_run`. This supports SFC linear row creation/deletion/reordering and existing row fields, guarded direct entity comments/conditions and declaration descriptions, plus XGK contact/coil and catalog instruction/comparison creation/deletion and instruction mnemonic/operand edits, IEC empty-row/terminal-coil creation and isolated/connected-contact and terminal-coil deletion, physical-wire deletion and supported vertical-wire creation, XGK/IEC contact and coil changes, bound IEC function operand edits and captured arithmetic/comparison name changes, native comment text, symbol descriptions, and XGK comment creation/rung-comment deletion. Native positions and connectivity remain explicit; unsupported moves and derived connection changes are rejected. Instruction creation/deletion/arity changes, connected IEC deletion and wire changes must be the sole mutation in a patch; commit and re-export before dependent edits. Read the portable [text IR reference](skills/xgwx-edit/references/text-ir.md) for syntax, comment semantics and examples.

Patch edits use the same immediate-save, backup and verified transaction behavior as `apply_edits`. The server derives current source offsets and expected values under its locks. Re-export after saves rather than guessing identities or reusing offsets.

## Common edit tools and prepared previews

Programs from `inspect_file` and `search_project`, physical modules from
`list_modules`/hardware inspection, and supported contacts from `search_project`
include `object_ref`. Use program references with
`rename_program(path, object_ref, name)`. Add `dry_run:true` to preview;
otherwise it saves immediately with the normal backup/verification.
Its `changes` list explains target, field and before/after values.
For module comments and contact operands, export the modules or target program
with `get_ir`, then submit the change through `apply_patch`.

For reviewable batches, read the portable
[common edit guide](skills/xgwx-edit/references/common-edits.md). Call
`prepare_change(path, changes:[{type, object_ref, name|comment|operand}])`, review
its changes and validation, then call `commit_change(change_token)` to save the
exact candidate. Tokens are single-use after success, session-scoped, expire in
30 minutes and retain no candidate file. References are bound to file/revision;
reinspect and rebuild after stale failures. Known errors return `recovery`, and
writer failures include `operation_name` for exact schema discovery.

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
