# Programs and symbols

Default to `get_ir` → `apply_patch` for supported edits. Writer names below
identify library APIs and argument shapes. Consult `describe_operation.editing_route`:
`tool_name:null` means use patches; a non-null `tool_name` identifies an exposed
gap tool. `apply_edits` and creation operation arrays reject patch-covered writers.

Inspect `programs` for document-order `program_index`, object identities, language, task, and current metadata. Inspect `program` with `program_index` for selected-program details. Use `symbols` for globals and local tables, supplying `program_index` when paging a selected local table. Obtain operation schemas from `describe_operation`; the examples below require real inspected paths, hashes, indices, and identities.

## Program lifecycle

- Program metadata uses `get_ir(section:"programs", program_index:...)` → `apply_patch`. Omit the index for all programs. Existing name/task/version/kind/instance_name/comment fields are editable; missing attributes and IDs stay readonly. `update_program` is its internal writer, not a callable tool or public batch operation.
- Lifecycle uses the complete `get_ir(section:"programs")` export, omitting `program_index`. Append a fresh authoring block with name/language to create, remove a whole block to delete, or move a whole block to reorder. Each is a sole structural mutation; preserve retained blocks and re-export after saving. Native IDs are derived internally. See the [lifecycle syntax](property-patches.md#program-lifecycle-in-the-full-list). `create_program`, `delete_program` and `move_program` are internal writer names; public tools and batch selectors no longer expose them.

Internal writer argument example (use property patches at the MCP surface):

```json
{
  "path":"/projects/machine.xgwx",
  "expected_revision":"<inspected revision>",
  "operations":[{
    "name":"update_program",
    "args":{"program_index":0,"patch":{"comment":"Conveyor interlocks"}}
  }],
  "dry_run":true
}
```

After creation, deletion, or reordering, re-inspect before selecting later programs. Do not infer updated indices from names alone, because names can also be edited.

## Globals and IEC local symbols

`update_variable` edits an inspected global `variable_index` with `patch`. Name uniqueness and captured symbol-record/address/type constraints remain enforced by the library. Resolve ambiguous references before renaming. Do not assume every displayed global can change type or address.

IEC local symbols belong to a program. Use the variables property export for primitive creation/deletion and metadata edits; read [lifecycle syntax](property-patches.md#iec-local-variable-lifecycle). `insert_iec_local_symbol` and `delete_iec_local_symbol` are internal writers. Copy `symbol_index`, current name/address/type/description, and other expected arguments from inspection. Function-block instances and mapped symbols have additional restrictions; do not treat them as ordinary scalar variables.

For a local rename, export `get_ir(section:"variables", program_index:...)`,
change the target declaration's `edit name` value and call `apply_patch`.
Rename, type and address patches must be sole mutations; re-export afterward.

Rename only when the inspected symbol and captured source layout support the rename. The library handles supported reference rewrites. After insertion/deletion or variable-length edits, re-inspect symbols, program references, and offsets before subsequent operations.

Typed IEC functions require compatible input and output symbols. Function-block instances need the matching type; numeric outputs need writable destinations of the required type. A scalar BOOL output may have a supported wire destination. Follow the selected writer's exact operand syntax and restrictions rather than substituting a similarly named symbol.

SFC declarations use `edit_sfc_variable`; follow the complete-table snapshot workflow in [sfc.md](sfc.md).
