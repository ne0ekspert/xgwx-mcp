# Standalone ST and IEC IL

## Create and inspect

Discover CPU choices with `list_templates`, then create with
`{"path":"control.xgwx","project_name":"Control","cpu_model":"XGI-CPUE","program_language":"ST","program_name":"Main","dry_run":true}`.
Repeat with dry_run false. Use language `IL` for IEC IL. XGK ladder-generated IL
is a different format; do not route standalone IEC IL through ladder editing.
`create_program` adds ST/IL to an existing supported IEC project with fresh
ObjectId/SymbolId GUIDs and a unique program name. Re-inspect indices afterward.

Inspect `{"path":"control.xgwx","section":"text","program_index":0}`.
Read the selected item’s `object_id`, `language`, exact `source`, `editable`,
`reason`, `variables` and `variables_error`. `program` inspection also returns
source details; ladder-only detail sections do not apply. For full declarations,
collect all `text.variables` pages at the same revision, including each advanced
`declaration` and `system` field.

## Source edits

Discover `describe_operation("edit_text_program")`, then submit an ordered
operation using the inspected revision:

```json
{"name":"edit_text_program","args":{"patch":{"program_index":0,"expected_object_id":"<inspected ObjectId>","expected_language":"ST","expected_source":"","source":"%MX0 := TRUE;\r\n"}}}
```

For IL the source can be `LD TRUE\r\nST %MX0\r\nRET\r\n`. The empty expected
source above is valid only for an inspected empty program. Copy current source
verbatim; retain Unicode and line endings. Maximum source length is 65536 UTF-16
units (an emoji counts as two), and NUL is forbidden. Encryption, unsupported
source containers or bookmark/breakpoint metadata remain read-only; report the
returned reason. Identity/language/expected-source guards remain mandatory.

Alternatively, export `get_ir(path, program_index)`. The ST/IL projection includes
one quoted `source` statement per native line inside `source { ... }`. For example:

```text
source {
  source "%MX0 := TRUE;\r\n"
}
```

Submit an exact unified diff through `apply_patch` using the exported revision.
Source statements can be added/deleted/replaced; their decoded strings concatenate
into the replacement source. Retain escaped newline characters unless changing
line endings intentionally. Revision, program identity, language and readonly
metadata cannot change. Re-export after saving. This projection edits source;
use direct declaration operations for variable tables.

## Declarations

Discover `edit_text_variable`. Its arguments are `expected_object_id` plus `patch`
with `program_index`, complete `expected_variables`, `name`, `data_type`,
`description`, `remove`, optional `declaration`, and optional `update`.
Add a BOOL `Run` with description `Run command`, remove false and update false.
Use update true for an existing declaration, retaining its advanced metadata;
remove true deletes only a declaration permitted by native reference guards.

Mapped/system variables and referenced declarations retain library restrictions.
The captured IEC encoder enforces supported types, mappings, arrays, storage
classes and initial values. Do not invent advanced schema fields; copy inspection
and discover the exact schema. Refresh the full snapshot after each declaration
edit. When renaming a declaration, coordinate the source change explicitly;
there is no automatic source refactoring. Source and declaration operations can
share one ordered atomic transaction, with later expected snapshots reflecting
preceding changes.

## Verify

Dry-run and save use the same guarded writer/container pipeline. Changed saves
retain backups; no-ops do not. Validation reports source decoding and UTF-16
length, with `syntax_and_types.status:"not_checked"`. Unsupported layouts can be
readable while editing is disabled. Neither decoding nor container round trips
checks IEC syntax, types or PLC behavior. Native acceptance is XG5000 Open →
Check Program → Save As to a separate file → compare source, declarations,
identities and unrelated sections. Report native acceptance as unverified unless
that workflow was actually performed.

## XGK vendor IL

Inspect `vendor_il` with `program_index`, then discover `edit_vendor_il`. Its patch
requires program_index, expected_object_id, exact expected_source and source.
Supported source uses LOAD/AND/OUT (including guarded variants) and catalog
application instructions, for example `LOAD M00000\nOUT M00001`. This replaces
the selected ladder payload; comments and geometry absent from IL are not retained.
Branch OR/AND LOAD, Auto-allocation and unsupported shapes remain read-only.
Use create_file with XGK CPU and language IL for an XGK IL view of native LD
storage. Program get_ir still returns positional ladder IR for XGK; source IL
replacement uses the direct writer. Re-inspect source and ladder after editing.
