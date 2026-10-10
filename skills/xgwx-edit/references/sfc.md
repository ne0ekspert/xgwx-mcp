# SFC charts and declarations

Inspect `sfc` for program/block/entity indices, positions, properties, `editable_rows`, and local declarations. For a complete selected block snapshot, use its `detail_section`, such as `sfc.block.0`, with `program_index`. For declarations, use `sfc.variables` with `program_index`.

Array pages are wrappers: collect their `items` arrays and reconstruct complete vectors at one revision before passing `expected_entities`, `expected_rows`, or `expected_variables`. In a block section, follow the entity/row page's `next_offset`; in the variable section, follow the returned page's `next_offset`. A partial page is not a complete stale-check snapshot. DTO fields use snake_case, while native property-map keys retain their exact names and case; copy the snapshots rather than renaming those keys. Obtain the selected operation's exact schema through `describe_operation`.

## Entity edits

`edit_sfc_entity` accepts nested `patch` with program/block/entity indices, expected type/row/column, field, expected value, and replacement. Existing step comments and captured direct `%MX` BOOL transition conditions are supported; the library checks identity, type, position, and old value and synchronizes transition annotations.

Example for a currently inspected step at the supplied identity/position:

```json
{
  "name":"edit_sfc_entity",
  "args":{"patch":{
    "program_index":0,
    "block_index":0,
    "entity_index":0,
    "expected_type":0,
    "expected_row":0,
    "expected_column":0,
    "field":"comment",
    "expected_value":"",
    "replacement":"Wait for conveyor ready"
  }}
}
```

Copy all expected values from the current entity; do not assume the first entity is a step at (0,0).

## Sequences, branches, and actions

`replace_sfc_sequence` accepts nested `patch` with `program_index`, `block_index`, the complete `expected_entities`, and desired typed `rows`. Include the complete inspected `expected_rows` when code/source snapshots require it; supplying the inspected snapshot consistently is useful. Start from supported `editable_rows` and change only the requested behavior.

`editable_rows:null` denotes an unsupported shape. Do not replace it with a guessed linear chart. The supported writer covers captured linear main charts and balanced alternative/simultaneous branches with their connector/alignment rules. Unsupported nested/crossing layouts, custom priorities, unknown records, or uncaptured program-reference forms retain their rejection.

Rows can represent supported steps, direct BOOL transitions, labels/jumps, variable/ST actions, and captured continuations. Qualifiers and timed actions use the accepted native forms and validated TIME literals. Multiple action slots and unequal branch-path lengths require their captured padding/continuation rules. Keep shared ST sources attached to their original names; preserve the expected source snapshot when changing source-bearing charts.

The writer regenerates captured annotations/placeholders and invalidates compiled caches while preserving program identity and local symbol payloads. Re-inspect after replacing a chart before selecting entities or declaration indices. A saved incomplete chart still needs completion before native program checks can pass.

## Local declarations

`edit_sfc_variable` inserts, updates, or removes a captured declaration using nested `patch` and a complete `expected_variables` snapshot. Follow the described `remove`/`update` modes, identifier rules, type, description, and optional typed `declaration` structure.

Supported declarations include validated primitive literals, Retain, captured fixed 32-byte STRING, and up to three zero-based array dimensions. Initializers follow captured literal/count/member rules. Existing descriptions, initial values, and retention can be changed where supported; type or bounds changes are restricted when declarations are referenced. System variables, mapped declarations, custom structures, and uncaptured instance/array shapes remain guarded.

After insertion/deletion or type changes, refresh the SFC variables, chart rows, and references. Verify local validation and report its limits; native acceptance still requires XG5000 Open → Check Program → Save As → comparison.

## Positional text patches

`get_ir(path, program_index)` detects SFC programs directly from their native XML and returns `dialect:"SFC"`. Apply an exact unified diff with its SHA-256 revision through `apply_patch`; preview with `dry_run:true`. The same verified transaction, backups and stale checks apply.

A supported chart exposes its typed rows:

```text
    sfc_row @p0.b0.r0 kind "step" title "Idle" comment "Wait for ready" initial true action none qualifier none time none action_code none transition_code none position none branch_end none
```

Use the actual exported identity and fields. Edit `title`, `comment`, `initial`, `action`, `qualifier`, `time`, `action_code` or `transition_code`. Optional fields use unquoted `none` for absence; quoted `"none"` is literal text. Quoted UTF-8 text supports escaped newlines, including multiline ST source. Native BOOL operands, qualifiers, TIME literals, initial-step/continuation rules, and shared ST-source checks still apply. Update every use of a shared source consistently; local serialization does not compile ST or establish native acceptance.

Keep retained row kind, `position`, `branch_end`, chart headers and entity fingerprints unchanged. Supported linear rows can be inserted, removed and reordered as described below; positioned branch order/structure stays fixed. An absent typed row position is printed as `position none`; it is not a guessed native coordinate. Read-only `entity` statements show native row/column/type separately. The server derives complete `expected_entities` and `expected_rows` from the fresh document and calls `replace_sfc_sequence`. Multiple field changes in one chart form one operation. Chart-row changes must be the only writer operation in a patch; commit and re-export before selecting another chart or editing declarations.

Unsupported shapes say `sequence unsupported readonly` and retain their property text. Existing step comments and captured direct `%MX` transition conditions may appear as `entity_field` statements. Change only their `value`; the server derives entity identity, native position and expected field value for `edit_sfc_entity`. It retains the original rejection for unsupported fields, layouts, or inconsistent transition annotations.

`sfc_variable` statements permit existing declaration `description` edits. Preserve name, type, system flag and declaration fingerprint; inspect `sfc.variables` for advanced metadata. Multiple description edits derive sequential complete expected-variable snapshots. Native system/mapped-declaration restrictions remain intact. Declaration creation/removal, type/bounds/initializer/Retain edits, positioned branch changes use the direct operations described above. Text patches never reconstruct unknown charts or declarations from scratch.

## Creating, deleting and reordering linear rows

For a chart whose supported rows have `position none` and `branch_end none`, text order defines the requested sequence. Add rows directly inside its exported `chart` block with fresh `@new-<name>` identities and every row field:

```text
    sfc_row @new-idle kind "step" title "Idle" comment "" initial true action none qualifier none time none action_code none transition_code none position none branch_end none
    sfc_row @new-ready kind "transition" title "%MX0" comment "" initial false action none qualifier none time none action_code none transition_code none position none branch_end none
    sfc_row @new-run kind "step" title "Run" comment "" initial false action none qualifier none time none action_code none transition_code none position none branch_end none
```

A supported empty chart can receive its first rows this way. New row kinds are `step`, `transition`, `label`, `jump` and `continuation`; branch kinds and explicit native positions remain direct-operation workflows. Preserve every field on retained rows unless intentionally editing a supported value. Do not rename retained source identities to describe a move.

Remove a row statement to delete it, or reorder retained statements inside the same linear chart. Keep all read-only entity/property statements and chart headers; they describe the source snapshot and the writer regenerates the resulting native coordinates, annotations and caches. Removing all rows requests an empty chart. Retained rows cannot move between chart blocks. Keep step/label names unique, maintain valid jump targets and action continuations, use compatible BOOL operands, and preserve consistent shared ST source. Native writer failures abort the entire transaction.

Insertion/deletion/reordering and row-field edits in one chart compile into one `replace_sfc_sequence` with complete source entity/row snapshots. They cannot share a text patch with declaration edits or another chart. Preview, save with its original revision, then re-export before dependent edits. Local serialization can retain incomplete charts; run native Open → Check Program → Save As → comparison for acceptance.
