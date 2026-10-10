# Ladder edits

For an instruction's semantics, operand order, data width, flags, execution restrictions, or module requirements, use the [individual instruction index](instructions/index.md) or `scripts/find_instruction.py` in this skill. Manual names such as `ULOADD<` resolve to their writer mnemonic (`UD<`). These XGK/XGB pages do not define IEC function semantics; use the appropriate IEC manual for IEC-specific behavior.

Inspect the selected `program` and the relevant `program.rows`, `program.records`, `program.cells`, `program.strings`, `program.functions`, `program.references`, `program.operand_links`, `program.graph`, and `program.edit_sites` sections. Obtain coordinates and byte offsets from these results. A displayed cell is not automatically a supported edit site.

Search operations by name and inspect their schemas before writing. Unknown records, unsupported topology, stale values, and incompatible operands must retain the library's rejection. Avoid reconstructing native records from a picture or from another CPU family's coordinates.

For contact/coil kinds or operands, native comment text, and symbol descriptions, [positional text IR](text-ir.md) offers `get_ir` → exact unified diff → `apply_patch`. It derives expected values and source offsets. Use the direct operations below for supported structural edits outside text IR v1.

## XGK

Captured XGK ladder uses ten cell columns: contacts occupy 0–8 and coils occupy 9. `raw_y` is the physical decoded coordinate (0, 4, 8, …), not a logical rung index or displayed line number. Use inspected `raw_y` even when rung comments or gaps make the display look contiguous.

`edit_ladder_cell` inserts/replaces/deletes an element via nested `edit`. `expected:null` means no element; `replacement:null` removes the actual element and leaves a wiring gap. For replacement/deletion, copy both expected kind and operand. Obtain exact enum values from the schema.

`edit_ladder_comment` edits/creates native comments with expected text. `delete_ladder_rung_comment` removes the dedicated comment row and shifts later coordinates. `insert_ladder_row` inserts a physical blank row before `raw_y`. Re-inspect after any row change.

`insert_ladder_instruction` and `insert_ladder_comparison` use supported catalogs and operand counts; instruction CPU restrictions remain enforced. Existing instruction text can use `update_ladder_cell_text` where supported. Recognized application instructions support their captured resizing logic, while other text layouts retain bounded/same-length restrictions. Do not use text clearing to simulate record deletion.

`edit_ladder_branch` changes supported connections between neighboring rows. Use the inspected column boundary; native wire boundaries differ from element positions. Deletion can intentionally leave disconnected wiring, so report subsequent program diagnostics.

## IEC

IEC ladder uses captured groups, rows, branch geometry, typed functions, and record offsets. XGK `raw_y`/column assumptions do not apply. Select writers using `program.edit_sites`; named insertion/deletion sites describe supported shapes and carry the required offsets/positions.

- Text/element updates include `update_iec_ld_comment`, `update_iec_ld_element_operand`, contact/coil kind updates, and function operand updates. Preserve the expected current value or kind.
- Row/rung/contact/coil operations operate only on their captured topologies. Compatibility aliases are `insert_iec_ld_parallel_contact`, `insert_iec_ld_linear_rung`, `delete_iec_ld_linear_rung`, and `insert_iec_ld_no_contact`; consult `describe_operation` for their canonical handler and default contact/coil kinds.
- Functions include terminal, standalone, connected cell, branch, arithmetic/comparison, and supported scalar chain cases. Use the matching inspected site and exact expected function name; a similarly rendered function may serialize differently.
- Groups can be moved, deleted, or replaced in supported shapes. Cross-program replacement requires an inspected source group as well as the target. Structural group changes invalidate later offsets and row positions.
- Wire operations use decoded horizontal/vertical/branch geometry and supported output links. Scalar BOOL ENO or comparison OUT may drive a captured wire. Numeric arithmetic/conversion OUT retains its typed destination. Removing a function operand or wire may leave an incomplete circuit.

An existing IEC operand example:

```json
{
  "path":"/projects/machine.xgwx",
  "expected_revision":"<inspected revision>",
  "operations":[{
    "name":"update_iec_ld_element_operand",
    "args":{
      "program_index":0,
      "offset":1234,
      "expected":"%MX0",
      "replacement":"%MX1"
    }
  }],
  "dry_run":true
}
```

The example offset and operands are illustrative. Use a current inspected operand record of the correct kind/type; do not copy the numeric offset. After an edit that changes encoded text lengths or structure, refresh the program detail and sites before using additional offsets.

Aliases are convenience entry points to the same guarded writers; they do not add support for another topology. `validate_file` reports local diagnostics, and XG5000 Check Program plus Save As comparison establishes native acceptance for the resulting file.
