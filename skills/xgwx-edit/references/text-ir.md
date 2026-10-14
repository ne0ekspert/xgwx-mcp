# Positional program text patches

For modules, variables and networks use the distinct [property document workflow](property-patches.md); those exports select a section rather than a ladder dialect.

Use `get_ir(path, program_index)` for supported contact/coil creation, deletion and edits, physical-wire deletion/vertical creation, native ladder comments, or symbol descriptions. It returns `format:"xgwx-ir/1"`, the file's SHA-256 `revision`, dialect, and a deterministic `text` projection. Ladder projections include one selected program, all global symbol descriptions, and that program's local descriptions. SFC projections include its native charts and local declarations; see the [SFC workflow](sfc.md#positional-text-patches). It is not a complete project compiler.

## Workflow

1. Inspect the CPU and program list. Export the selected program with `get_ir`.
2. Prepare an exact unified diff against its returned text. Keep the revision header, identities, positions, block headers, and read-only statements. Hunk line numbers are one-based; counts and context must match exactly. Optional `---`/`+++` labels are inert labels, not paths the server opens. Do not use `*** Begin Patch` wrappers or fuzzy context matching.
3. Call `apply_patch` with `path`, `program_index`, the exported `expected_revision`, `patch`, and optionally `dry_run:true`.
4. Review operation results and validation. Save with the original revision and `dry_run:false`. Report the backup and saved revision. Export again after saving; generated references belong to the exported revision, and structural comment changes shift rows and offsets.

The server re-exports the fresh document under its transaction locks before applying the patch. It derives expected values and source offsets itself. The complete semantic difference is checked before any writer runs. Writers operate on a fresh in-memory document and verified serialization precedes any replacement. A failed writer aborts all changes; no-op/empty patches create no backup. Dry-run changes neither the project nor sidecars/backups.

## XGK coordinates and elements

```text
xgwx-ir 1 revision "<SHA-256>"
symbol @s0 "PumpRun" address "P00000" type "BOOL" description "Pump run output"
program @p0 "Main" dialect XGK object "<native ObjectId>" {
  rung @p0.r0 at(y=0) {
    contact @p0.e120 at(x=1,y=0) NO "M00000"
    coil @p0.e160 at(x=94,y=0) OUT "P00000"
    comment @p0.c200 at(x=97,y=0) OUTPUT "Pump start command"
  }
  wire @p0.h0 from at(x=4,y=0) to at(x=25,y=0) HORIZONTAL
}
```

Example identities/offsets above are illustrative. Always use the actual export.

XGK `x` and `y` are native decoded `raw_x` and `raw_y`, not normalized visual column/row numbers. Contact anchors are `1 + 3*column` for columns 0–8 (`x=1,4,…25`). The output column is 9 and its stored native anchor is `x=94`; rows use increments of 4 in native `y`. Wire coordinates describe decoded segments. Rung comments use `x=1`; output comments use `x=97`.

XGK contact kinds: `NO`, `NC`, `INVERSE`, `RISING`, `FALLING`, `ADDR_RISING`, `ADDR_FALLING`, `ADDR_NEGATED_RISING`, `ADDR_NEGATED_FALLING`. `INVERSE`, `RISING`, and `FALLING` contacts are operandless and use `""`. Addressed variants retain their device operands. Coil kinds: `OUT`, `INVERSE`, `SET`, `RESET`, `RISING`, `FALLING`. Native kind/column/device rules still apply.

Change a kind or quoted operand in its existing statement:

```diff
@@ -5,1 +5,1 @@
-    contact @p0.e120 at(x=1,y=0) NO "M00000"
+    contact @p0.e120 at(x=1,y=0) NC "M00000"
```

For instruction semantics and device restrictions, load the corresponding [instruction guide](instructions/index.md). Unsupported instruction layouts, changes to existing positions, and derived circuit-connection edits are outside text IR v1. Use `list_operations` and `describe_operation` for existing supported structural writers.

## XGK instructions and comparisons

```text
    instruction @p0.e120 at(x=88,y=0) APPLICATION mnemonic "MOV" operands "1" "D000000"
    instruction @p0.e200 at(x=1,y=4) COMPARISON mnemonic "=" operands "D000000" "1"
```

Use actual identities and positions from the export. Each operand is a separate quoted text field. Native string literals retain single quotes inside that field, such as `"'a,b'"`; a comma outside a single-quoted literal cannot be hidden inside one operand field. Mnemonics are single tokens. For operand types, device permissions, CPU restrictions and instruction semantics, load the corresponding [instruction guide](instructions/index.md) and inspect `catalog.instructions` or `catalog.comparisons`.

Edit the mnemonic or operand fields on a retained statement. The server obtains the complete original comma-separated string from its source mapping and calls `update_ladder_cell_text`; it never edits an internal operand copy. The writer rebuilds redundant strings, opcode, occupied cells and wiring. Family and existing native coordinates remain fixed in the patch. Arity changes can move the application's left anchor; re-export to get that new anchor.

To create an instruction, add a statement with a fresh `@new-<name>` identity inside an exported rung with matching raw `y`. Application instructions end at native x=94, so their left anchor is `94 - 3*catalog_operand_count` (MOV:88, ADD:85). Comparisons start at x=1,4,… and their full span must fit before the output column. Occupied elements, branches, CPU/manual restrictions and operand checks remain guarded by the existing insertion writers.

Remove a statement to invoke `delete_ladder_instruction` or `delete_ladder_comparison`. Application deletion supports the native unbranched output-row shape and removes its feed wire; comparison deletion leaves its native wiring gap. Do not erase the corresponding wire statements yourself in the same patch.

Instruction creation, deletion and arity changes must each be the **sole mutation** in a patch. Commit and re-export before dependent edits. Same-arity edits at multiple source offsets can share a transaction. Unknown, terminal or undecoded instructions remain read-only. A structural writer on a disposable document validates the native layout before complete instruction edits are exposed, so the legacy raw-string fallback is not used by these text patches. When that probe fails, `instruction_layout error ... readonly` reports its reason.

## IEC coordinates

```text
program @p0 "Main" dialect IEC object "<native ObjectId>" {
  symbol @p0.s0 "Enable" address "" type "BOOL" description "Enable condition"
  contact @p0.e120 at(group=0,row=0,x=1) NO "Enable"
  coil @p0.e160 at(group=0,row=0,x=4) OUT "Run"
  comment @p0.e220 at(group=0,row=1,x=1) IEC "Network note"
  connection @p0.w0 from at(group=0,row=0,x=0) to at(group=0,row=0,x=3) kind "Contact(6)" readonly
}
```

IEC uses native group index, row index within its decoded group, and anchor `x`. Element anchors are 1,4,7,…94; branch/wire boundaries are 0,3,6,…96. Do not substitute XGK raw `y` for an IEC row. Recognized function bodies and bound operands are exported as positional `function` and `argument` statements. Unknown function shapes, unbound operands and unknown topology remain read-only records with source fingerprints. If validated geometry is unavailable, the export says so instead of inventing connections.

IEC annotation records that the comment writer does not support are displayed as positional `annotation ... readonly` statements, including decoded output annotations. Their text remains visible, but patches cannot change them.

IEC contact kinds: `NO`, `NC`, `RISING`, `FALLING`, `NEGATED_RISING`, `NEGATED_FALLING`. Coil kinds match XGK. Operands retain the library's existing BOOL/device-expression and writable-output preconditions. Local symbol descriptions are editable; declaration names, addresses, and types are read-only in this version. The source mapping resolves record identities to the actual string offsets required by the IEC writers; agents do not calculate binary offsets.

## Readonly semantic statements

IEC text omitted by the writer-facing string list (including Unicode and empty
fields) is decoded within library-validated record boundaries and exposed as
readonly contact/coil, argument or annotation statements. `decode "decoded"`
identifies known text; it does not grant edit access. Preserve the whole line,
including `reason`, source hash and `readonly`. An unbound FunctionOperand
record exposes its known `value` with `function_binding_unavailable` instead of
inventing a pin target. Other fallback records distinguish `decode "unavailable"`
from structural `decode "not_applicable"` and retain their kind and hash.

## Function call and pin metadata

Readonly `call`, `symbol_instance`, `pin`, `binding` and `decode_status` statements
expose instance types, named control/data pins and explicit reference-to-pin
links. State reflects the encoded instance marker, not runtime behavior. Broad
pin masks are constraints; selected_type stays none unless a known single type
bit is encoded. Instance lookup is exact and unique within the local table, with
explicit mismatch/unavailable states. Argument links distinguish resolved
records, no expression and decoder failure. Keep all metadata intact during
supported function-name/argument-value edits and re-export after saving.

## Typed operand metadata

Readonly `operand` statements link source statements and operand indices to
SYMBOL, DIRECT_ADDRESS, FIELD, SYSTEM_SYMBOL, LITERAL or UNRESOLVED categories.
Local declarations shadow globals; duplicate names stay ambiguous and direct
addresses retain all exact spelling candidates. Native instance-pin fields can
resolve; compound/array expressions remain unresolved. Syntax-only addresses
and literals are not CPU/type/writability validation, and temporal prefixes
remain unvalidated. Keep metadata intact during edits and re-export afterward.
Global table renames do not rewrite program operands. Local rename propagation
covers captured exact-string uses/instance markers rather than arbitrary member
or compound expressions. Changing an operand affects only that use.

## Endpoint and project context metadata

Readonly IEC `junction`, `edge`, `terminal` and `pin_junction` statements connect
native circuit boundaries to element/control-pin IDs and source record/wire
provenance. Coordinate connection statements remain available. The graph records
conditional circuit structure, not runtime activation. Decoder failures do not
produce guessed topology. XGK wires retain their native positional format.
Readonly CPU/task/program context preserves language, task bindings and raw
priority/trigger data, while runtime execution order and memory aliases remain
not_decoded. Keep these statements intact in all patches and re-export after
native changes.

## Creating and deleting elements

Add new statements with fresh `@new-<name>` identities. These authoring identities are temporary; after saving, `get_ir` returns new source-backed references. Preserve source identities on retained objects. Multiple elements cannot share a native position.

For XGK, insert contacts at `x=1,4,…25` or a coil at `x=94` inside an exported rung block with matching `y`. The server derives `expected:null` and applies the existing cell writer. Unsupported occupied spans and native row layouts remain guarded. A blank XGK program exports a `y=0` authoring rung, so its first elements or native comment can be created through text patches.

```text
  rung @p0.r0 at(y=0) {
    contact @new-enable at(x=1,y=0) NO "M00000"
    coil @new-pump at(x=94,y=0) OUT "P00000"
  }
```

Delete an XGK contact or coil by removing its statement, preserving the block wrappers and read-only records. The server derives the expected kind/operand from the source and leaves the wiring gap defined by the native writer. Existing objects cannot move by changing their coordinates.

IEC exports native row anchors as `row ... readonly`. An untouched eight-byte empty header instead exports `canvas @p0.blank at(group=0,row=0) readonly`, identifying its initial authoring site without claiming that a native row envelope already exists.

Create an IEC contact/coil in an exported empty row, or at group 0/row 0 of the untouched empty canvas. Put its statement directly inside the program block. Rows must be unambiguous for the native row-index writer. The native placement and BOOL/writable-output rules still apply. The server orders contacts before coils so a contact at `x=1` and terminal coil at `x=94` can be created together. It also permits adding a terminal coil at `x=94` to a supported isolated contact row.

```text
  contact @new-enable at(group=0,row=0,x=1) NO "%MX0"
  coil @new-pump at(group=0,row=0,x=94) OUT "%MX1"
```

Delete an isolated IEC element by removing its statement when its native row contains exactly one contact or coil. That writer clears the record while retaining the row envelope. Connected contact deletion uses `delete_iec_ld_contact`; terminal coil deletion uses `delete_iec_ld_terminal_coil`. Their native shape, kind, value and binding guards remain intact. The text layer never substitutes Cell Delete or a row-collapse strategy. Connected deletion must be the sole mutation in a patch; commit and re-export before dependent edits.

The transaction applies source-offset edits/deletions in descending offset order, then coordinate-based additions, then native rung-comment row changes. Keep the original native coordinates in the patch and re-export after committing. Report unsupported shapes precisely rather than switching to a different structural strategy silently.

## IEC function names and arguments

```text
  function @p0.f300 at(group=0,row=0,x=7) name "ADD" signature "<source fingerprint>"
  argument @p0.e600 at(group=0,row=1,x=4) target "@p0.f300" pin 1 pin_row 1 pin_x 6 direction INPUT type_mask 16 array false value "%MW0"
```

These offsets, coordinates and masks are illustrative; use the actual export. `at(...)` is the native operand-record position. `target` names its decoded function; `pin`, `pin_row` and `pin_x` identify the resolved native pin. Direction, type mask, array flag, source signature, identities and positions are immutable.

Edit only a function's quoted `name`, or an argument's quoted `value`. Names can switch within `ADD/SUB/MUL/DIV` or within `EQ/GT/GE/LT/LE`, using the exact captured function writers. Other function names are visible but cannot be renamed through a patch. The server derives the real function-name/operand string offsets and applies multiple resizing edits in descending source-offset order.

Bound operand edits use `update_iec_ld_function_operand` with native type compatibility, sibling scalar-type compatibility, expression and writable-output checks. Consult `inspect_file` sections `program.functions`, `program.operand_links` and `symbols` when choosing operands; the XGK instruction manual does not define IEC function semantics. A rejected compound expression must not be rewritten as raw bytes.

Set an existing OUTPUT argument value to `""` to request guarded native output-operand deletion. This must be the sole mutation; re-export afterward. Empty INPUT arguments are rejected. Function/argument creation, deletion, movement and pin rebinding remain direct-operation workflows. Deleting a statement never implicitly deletes a function or clears a pin.

## Physical wires

`wire ... HORIZONTAL` and `wire ... VERTICAL` identify decoded physical records. Remove a wire statement to invoke its native deletion writer. IEC horizontal wire endpoints are the decoded record coordinates; derived `connection ... readonly` endpoints represent circuit boundaries and cannot be edited. Preserve every derived connection statement in the patch: re-export updates that view after saving.

Add a vertical wire directly inside the program block with a fresh identity:

```text
  wire @new-branch from at(x=3,y=0) to at(x=3,y=4) VERTICAL
```

For XGK, both endpoint rungs must already be exported, with raw `y` separated by exactly 4, and `x=3,6,…27`. For IEC, use `at(group=G,row=R,x=X)` at both ends; both rows must be exported in the same group, with equal boundary `x`. The existing vertical-wire writer validates supported native shapes and expected absence.

Wire creation/deletion and connected IEC element deletion must each be the **sole mutation** in a patch. Their cleanup can shift offsets or rows; commit, re-export, and then prepare dependent edits. Horizontal drawing and changing a retained wire's endpoints/kind are unsupported through text IR v1. Discover direct operation schemas for broader native workflows.

## Native comments and descriptions

`comment ... RUNG` is a native XGK rung-comment row; `comment ... OUTPUT` is a native output annotation. They are distinct from the shared `description` on a `symbol` statement. Editing a symbol description changes the declaration used by every reference to that symbol. Do not treat the output annotation as a symbol description.

Edit an existing comment's quoted text. UTF-8 strings support `\n`, `\r`, `\t`, `\"`, and `\\` escapes, keeping every statement on one physical line. Triple-quoted strings are not part of v1. The existing XGK writer permits multiline comments and requires nonempty text of at most 255 UTF-16 units; the IEC writer rejects empty/control-character comment text. Report its actual rejection rather than rewriting the requested text silently.

Create an XGK comment by adding a statement with a fresh `@new-<name>` identity inside an existing rung block, using its `y`:

```text
    comment @new-output at(x=97,y=0) OUTPUT "Pump start command"
```

For a rung comment, use `RUNG` and `x=1`. Creation inserts a dedicated native row above the requested existing row and shifts subsequent native positions. Rung insertion inside a branch span remains rejected by the writer. Adding an output annotation does not imply adding a rung-comment row. A new identity cannot overwrite a retained existing comment; edit their existing statement.

Delete an XGK rung comment by removing its statement, retaining the block wrappers in the patch. The writer removes the dedicated row and closes the coordinate gap. Output-comment deletion and IEC comment creation/deletion have no text IR v1 adapter. Empty text is not deletion. Re-export after structural comment edits rather than reusing the old identities or positions.

## Preservation and acceptance

Existing positions, object families, program/rung identities, unknown instruction records, function signatures and pin bindings, derived `connection` statements and opaque records are read-only in v1. Supported element creation/deletion can cause the library to update feed wiring; export again to inspect the resulting native topology. Unknown payloads stay in the original document; patches never rebuild a project from text. Unsupported changes return `unsupported_ir_edit`; syntax/context/count errors return `invalid_patch`; writer failures retain the underlying reason and operation index. Stale requests require a new export and a rebuilt patch.

Successful local verification establishes container serialization and available program diagnostics. Native acceptance still requires XG5000 Open → Check Program → Save As → comparison. The server does not perform that external sequence or communicate with a PLC.

## Blank rows and rung lifecycle

Apply exactly one lifecycle mutation per program patch, keeping all other IR
statements unchanged. Save and re-export before editing shifted positions.

IEC blank rows are gaps in the native row index, rather than decoded row records.
Insert a blank after a current row:

```text
  blank_row @new-space after row 0
```

Create a contact-to-coil rung in an unoccupied IEC row (row 0 on an empty canvas):

```text
  rung_row @new-enable at row 1 contact NO "%MX0" coil OUT "%MX1"
```

Contact kinds are `NO`, `NC`, `RISING`, `FALLING`, `NEGATED_RISING`, and
`NEGATED_FALLING`. Coil kinds are `OUT`, `INVERSE`, `SET`, `RESET`, `RISING`, and
`FALLING`. Native variable, writable-coil and captured-layout validation applies.

Remove an exported `rung_row @pN.rungR at row R` marker to delete that captured
simple rung; retain its body statements in this transaction. The marker captures
native kinds and operands, and is exported only when the native writer can delete
the rung. The next export removes its body and displays the resulting blank.
Remove an exported writable `blank_row @pN.blankR at row R` marker to close that
blank and shift subsequent rows. First and trailing blanks are readonly. Deletion
of the only rung, branches, or function rungs is not covered by these markers.

For XGK, insert a blank before a native Y position:

```text
  blank_row @new-space before y 0
```

The writer shifts subsequent native cells, wires and comments. General XGK rung
deletion remains outside this IR coverage. New lifecycle identities must start
with `@new-`; existing markers and their coordinates cannot be rewritten. Direct
blank-row and simple IEC rung tools are replaced by these program patches.

## IEC insertion sites and wire repair

The export includes readonly `contact_site` statements for native `LEADING`,
`WIRE`, and `SHORT_WIRE` insertion sites, `parallel_site` statements for captured
normally-open-contact/output-coil rungs with available branch space, and
`wire_gap` statements for repairable horizontal gaps. Site identities capture
native offsets, expected geometry and current operands. A missing site cannot
be supplied by inventing an identity or changing readonly metadata.

Add one authoring statement directly inside the program block, retaining all
existing statements unchanged:

```text
  contact_insert @new-interlock site @p0.contactwire123 kind NC operand "%MX10" x 7
  contact_insert @new-leading site @p0.leading123 kind NO operand "%MX10"
  contact_insert @new-refill site @p0.shortwire123 kind RISING operand "%MX10"
  parallel_contact @new-parallel site @p0.parallel0 kind NC operand "%MX11"
  wire_restore @new-repair site @p0.wiregap123
  comment_row @new-note at row 3 text "Enable circuit"
```

Each line above is a separate patch example. Copy an actual exported site ID;
`123` is illustrative. Only `WIRE` contact sites accept `x N`, on the captured
three-cell grid inside their span. Leading and short-wire sites retain their
fixed native cells. Contact kinds use the rung contact names listed above.
Native BOOL operand and layout checks remain authoritative. After saving,
re-export to obtain the inserted element and updated site identities.

`comment_row` copies a captured singleton native comment group into a supported
destination row and changes its text. It requires an existing comment template;
it cannot create the first comment in an empty project. Text must be nonempty
without control characters. Adding a comment does not insert a blank row.

General function/group replacement, output-pin wiring and assignment, Cell Delete
variants, and specialized branch/heating deletions retain direct gap tools.
