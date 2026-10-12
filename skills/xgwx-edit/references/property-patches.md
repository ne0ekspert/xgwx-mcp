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
  readonly id = 42242
  readonly sub_type = 0
  readonly model = "XGI-D24A/B"
  readonly native_name = "<native name>"
  edit comment = "Tank inputs"
  edit option.inputFilter.0 = 3
}
```

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

## Supported mappings

| Properties | Public writer |
| --- | --- |
| Module comment | update_module |
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
