# Module, variable and network property patches

Use the focused readers before editing:

- `list_modules(path, offset?, limit?)`: installed modules, base/slot identities,
  decoded settings and rack/CPU context.
- `list_module_catalog(query?, offset?, limit?)`: available catalog models,
  defaults and option choices, without an existing file. Catalog presence alone
  does not prove CPU/layout compatibility.
- `list_variables(path, program_index?, offset?, limit?)`: globals plus IEC locals;
  optional program_index selects one local table. SFC declarations use
  `inspect_file(section:"sfc.variables", program_index)` and the SFC program IR.
- `list_networks(path, offset?, limit?)`: network/module metadata and Cnet/FEnet
  configuration. Nested arrays paginate independently, just as inspect_file does.

All file readers return a current `revision` and `data`; list_module_catalog has
no file revision. Follow `next_offset` for each independent array. List responses
are inspection data, not patch text. Obtain a complete export with:

```json
{"path":"/projects/machine.xgwx","section":"modules"}
```

Call `get_ir` with section `modules`, `variables` or `networks`. Only variables
accepts optional program_index to select IEC locals; globals remain included.
The result is `format:"xgwx-properties/1"` with `revision` and full `text`.
Existing program exports remain available with section `program` (default) and
required program_index; they use the separate positional syntax.

Example module document (copy actual exported identities/values):

```text
xgwx-properties 1 revision "<hash>" section modules program all
module @m0 base 0 slot 0 {
  edit model = "XGI-D24A/B"
  readonly native_name = "<native name>"
  edit comment = "Tank inputs"
  edit option.inputFilter.0 = 3
}
```

`model` selects a native catalog model using `select_module` internally. It is
editable when native CPU/layout and unique base/slot guards support replacement.
An empty editable model means the current model is not catalog-decoded; select a
valid model from `list_module_catalog`. Unsupported targets remain readonly.
`native_name` shows the stored native name and stays readonly. ID/subtype and raw
`Details` are omitted. Model replacement sets native IDs/name/default options and
maintains matching companion network records. It must be the sole mutation;
re-export before editing comments/options or network settings. Only supported
decoded `option.<key>.<index>` values are exposed. Comments and decoded options
retain their own writers and native checks.

Example variable properties within an exported block:

```text
variable @global.v12 {
  edit name = "PumpCommand"
  edit address_area = "P"
  edit address_number = 32
  edit data_type = "BIT"
  edit description = "Pump contactor command"
  readonly source_ref = "IO:0:1:0:OUT"
  readonly range = none
}
```

Change only values on lines marked `edit`. Keep the revision header, object
identities, order, field names, markers, and all readonly lines. Quoted strings
use JSON-style escapes, including \n and escaped quotes; unsigned numeric fields
use decimal integers through u32::MAX. Scalars must retain their type; `none`
marks a missing readonly value, not an instruction to clear it. Use `""` to clear
an editable description when its writer allows it. Equivalent string escapes
produce a semantic no-op.

Send an exact unified diff to `apply_patch`, with the same path, section,
program_index selection (if any), exported expected_revision and optional
`dry_run:true`. Diff line numbers refer to the complete export. Identical-looking
property lines can occur in several blocks: preserve context and target the
correct object; never replace every occurrence of a value by name alone.

```json
{
  "path":"/projects/machine.xgwx",
  "section":"modules",
  "expected_revision":"<exported hash>",
  "patch":"--- properties\n+++ properties\n@@ -8,1 +8,1 @@\n-  edit option.inputFilter.0 = 3\n+  edit option.inputFilter.0 = 5\n",
  "dry_run":true
}
```

The line number above is illustrative. Preview, then save using the original
source revision. A successful changed save retains a backup; re-export before
another edit. Stale revisions and any failed native writer abort the transaction.


## Network metadata and FEnet settings

Use `get_ir(section:"networks")` → `apply_patch` for these existing decoded
fields. `edit_browser_network` and `edit_fenet_field` are internal writer routes;
they have no separate direct tools and cannot be selected in batch or creation
operation arrays.

| Object | Editable fields |
| --- | --- |
| Network | `name` |
| Linked network module | `config_name`, `alias`, `description` |
| FEnet IPv4 settings | `ipAddress`, `subnet`, `gateway`, `dns`, and their `2` variants |
| FEnet DHCP | `dhcp`, `dhcp2` |
| FEnet numeric settings | `stationNo`, `glofaSocketCount`, `rcvWaitTime`, `clientWaitTime`, `driverType` |

Keep quoted scalar values as exported: IPv4 uses four decimal octets, DHCP uses
`"0"`/`"1"`, and numeric settings use quoted decimal integers. The native writer
checks contiguous subnet masks, DHCP flags, numeric ranges and driver choices.
Driver values are `"2"` (XGT), `"5"` (Modbus) and `"7"` (Smart). Socket count is
1–16; receive/client wait times are 2–255. Station-number limits depend on the
stored protocol configuration and remain enforced by the native writer.

`config_name` selects the PLC configuration scope; use an inspected configuration
name rather than a display label. Changing it can detach the settings association.
FEnet records with missing or ambiguous membership/configuration associations
show readonly `association_status` and readonly settings; their decoded values
remain visible, but patches cannot promote them to editable fields.

Missing settings stay `readonly ... = none`: patches cannot invent attributes.
Network/module identity and protocol fields remain readonly. Preserve the
exported object boundaries, order and revision; repeated field names belong to
different objects. Only the selected existing XML settings change; opaque
protocol data, hardware and program payloads are retained. Any failed setting
aborts the entire transaction, including other metadata edits in that patch.

## Supported mappings

| Properties | Public writer |
| --- | --- |
| Module native name and comment | update_module |
| Decoded module option key/index/value | set_module_option |
| Global name/address area/address number/type/description | update_variable |
| IEC local address | update_iec_local_symbol_address |
| IEC local description | update_iec_local_symbol_description |
| IEC local name or primitive type | rename_iec_local_symbol / update_iec_local_symbol_type |
| Network name, linked module config_name/alias/description | edit_browser_network |
| Exported FEnet IP/mask/gateway/DNS, DHCP and numeric settings | edit_fenet_field |
| Exported Cnet per-port raw settings | edit_cnet_settings |

Global changes for one variable coalesce into one patch. Related Cnet changes
for a base/slot coalesce into one native settings transaction. Cnet/FEnet field
keys retain the public writer's spelling and raw values; use describe_operation
and current settings rather than guessing enum numbers or address formats.
The native validators still check masks, limits, ports, types and wiring rules.

IEC local rename/type/address changes must be the sole mutation, then re-export
because symbol order/allocation/reference data can change. Supported primitive
IEC locals expose `edit address` as a quoted string; `""` assigns no explicit
address and clears an existing mapping. The native writer enforces captured
address areas, type widths, automatic-allocation restrictions and overlap checks.
Storage class and allocation metadata are derived and remain readonly, as do
instance addresses/types, unsupported types, missing fields and undecoded tables.
Module model replacement, object creation/deletion/movement, network
protocol identity changes and SFC declaration edits are outside this property
syntax; use their supported direct operations or SFC workflow.

Local verification does not establish native XG5000 Check Program/Save As or PLC
acceptance. These operations update the local project only.

## Project and program metadata

Export `section:"project"` for the project name, or `section:"programs"` for
program XML metadata. The latter accepts optional `program_index`; omission
exports all programs. Preserve the revision, document-order identities and
readonly context. Program bodies stay in the separate `section:"program"` export.

```text
program_metadata @p0 {
  edit name = "Conveyor"
  edit task = "_SCAN"
  edit version = 1
  edit kind = 0
  edit instance_name = ""
  edit comment = "Feed conveyor"
  readonly object_id = "<captured identity>"
}
```

Use the actual exported fields. Version/kind are unsigned 32-bit integers;
other editable values are quoted strings, including empty strings where the
native writer permits them. Multiple field changes coalesce into one native
`update_program` per program. Project renaming derives its old-name guard from
the fresh document. Missing attributes are `readonly ... = none` and cannot be
created here. Project flags/version/comment/GUID/timestamps and program identity,
encryption, allocation/search flags remain readonly. Metadata edits preserve
body payloads and symbols; task/kind/version changes still need native validation.
`rename_project` and `update_program` are internal writer names; direct tools
and public batch selectors reject them.

## Program lifecycle in the full list

Use `get_ir(section:"programs")` without `program_index`. Its readonly lifecycle
context lists the native creation languages. Append one authoring block:

```text
program_metadata @new-conveyor {
  edit name = "Conveyor"
  edit language = "LD"
}
```

The server generates distinct native program/symbol identities. Creation appends
an empty program using library defaults and an existing native scan task; name,
CPU/language, container and layout guards remain authoritative. Preview and save
from the same revision use the same generated IDs. Caller-supplied native IDs
and body templates are not accepted in authoring blocks.

To delete, remove one complete exported `program_metadata @pN` block. To reorder,
move one complete block to its desired final list position. Preserve every line
inside retained blocks and all readonly context. The server derives source/target
IDs from the current document; source identities are never renumbered in a patch.
Deletion also removes the selected program's local declarations. Reordering
preserves task assignments and body payloads; execution order within a task can
change. Each lifecycle patch must express exactly one creation, deletion or
native move, without metadata edits. Multiple moves or create-at-position require
separate patches. Re-export after saving before selecting new program indices or
editing the newly created body through `section:"program"`.

Exports with `program_index` are metadata-only. Other property sections retain
fixed object order. Unsupported/nested program containers remain native guarded;
unknown records are never reconstructed from metadata. Native XG5000 acceptance
still requires Open → Check Program → Save As → comparison.

## IEC local variable lifecycle

Export `section:"variables"`, optionally with `program_index`. The native
primitive insertion capability appears as `local_table @pN create true ... readonly`.
Append one authoring block for that program:

```text
variable @new-ready program 0 {
  edit name = "Ready"
  edit data_type = "BOOL"
  edit description = "Enable conveyor"
}
```

Use a fresh authoring identity; native table indices and case-insensitive name
order are derived after saving. New locals are unallocated primitive declarations;
assign addresses through their existing property workflow after re-export.
Native identifier, primitive type, table/layout, uniqueness and description limits
remain authoritative. Arrays, custom instances and global insertion are not
inferred from metadata. Captured empty local tables support their first insertion.

Delete one local declaration by removing its entire exported variable block.
The server derives program/symbol index and old name from the fresh document.
Native reference checks reject variables still used by the program. Retained
blocks and context must be unchanged; creation/deletion is the sole mutation.
Global variable creation/deletion remains unsupported. SFC/ST/IL declarations
retain their existing domain workflows. Re-export after saving.

I/O generation is a separate `generate_io_variables` tool with optional top-level
base/slot filters, current expected_revision and dry_run. It is not a variable
block creation shortcut; inspect its returned row/action plan and refresh globals
following a save.
