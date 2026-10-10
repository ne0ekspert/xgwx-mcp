# Programs and symbols

Inspect `programs` for document-order `program_index`, object identities, language, task, and current metadata. Inspect `program` with `program_index` for selected-program details. Use `symbols` for globals and local tables, supplying `program_index` when paging a selected local table. Obtain operation schemas from `describe_operation`; the examples below require real inspected paths, hashes, indices, and identities.

## Program lifecycle

- `update_program` accepts `program_index` and `patch` for editable metadata. Omitted patch fields remain unchanged.
- `create_program` accepts `patch` with `name`, `language`, `object_id`, and `symbol_id`. Supply fresh GUIDs and a unique identifier name. The library appends a captured blank program and binds it to an existing scan task. XGK supports LD; supported XGI profiles support LD, SFC, ST and IEC IL creation. Discover exact accepted language strings and CPU constraints in the operation schema/description.
- `delete_program` requires `program_index` and `expected_object_id`. It removes the selected top-level program and its local declarations; use the inspected identity.
- `move_program` uses `from`, `to`, `expected_object_id`, and `expected_target_id`. `to` is the final program-list index. Reordering retains task assignments; list order affects execution within a task, while task scheduling remains independent.

Example metadata transaction:

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

IEC local symbols belong to a program. Discover `insert_iec_local_symbol`, `delete_iec_local_symbol`, `rename_iec_local_symbol`, and `update_iec_local_symbol_*` operations. Copy `symbol_index`, current name/address/type/description, and other expected arguments from inspection. Function-block instances and mapped symbols have additional restrictions; do not treat them as ordinary scalar variables.

Example local rename operation inside `apply_edits.operations`:

```json
{
  "name":"rename_iec_local_symbol",
  "args":{
    "program_index":0,
    "symbol_index":2,
    "expected_name":"MotorReady",
    "replacement":"ConveyorReady"
  }
}
```

Choose that example only when the inspected symbol and captured source layout support the rename. The library handles supported reference rewrites. After insertion/deletion or variable-length edits, re-inspect symbols, program references, and offsets before subsequent operations.

Typed IEC functions require compatible input and output symbols. Function-block instances need the matching type; numeric outputs need writable destinations of the required type. A scalar BOOL output may have a supported wire destination. Follow the selected writer's exact operand syntax and restrictions rather than substituting a similarly named symbol.

SFC declarations use `edit_sfc_variable`; follow the complete-table snapshot workflow in [sfc.md](sfc.md).
