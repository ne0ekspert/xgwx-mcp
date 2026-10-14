---
name: xgwx-edit
description: Inspect and edit local LS XG5000 .xgwx projects through the xgwx-mcp server, including programs, symbols, ladder, hardware, networks, SFC charts, and standalone ST/IL source programs.
---

# XGWX editing

Use the configured `xgwx-mcp` tools for supported local project edits. Each successful changed `apply_edits`, `apply_patch`, common edit tool or `commit_change` request immediately saves the source and retains an adjacent backup. The skill folder is portable; its references require no sibling repository. A stdio MCP connection to the server must already be available.

## Inspect, discover, edit

Use `get_ir` → `apply_patch` as the default editing workflow. Export the
current revision-scoped program or property document, change only supported
editable statements, and submit an exact unified diff. The server derives writer
arguments and expected values internally and retains its native guards, backup
and atomic-save behavior.

Only capabilities outside IR coverage have direct `xgwx_<operation_name>` tools.
Their complete input schemas are included in `tools/list`. Catalog responses
include `editing_route` and `tool_name`: a null tool name means the writer is
internal to patches; partial coverage retains a direct tool for the remaining
capabilities. Library writer names in reference examples identify internal APIs,
not necessarily callable MCP tools.

`apply_edits` and initial `create_file.operations` accept only the closed union
of these exposed gap writers. Patch-covered operations return `patch_required`
before any file changes. Create a project first, then export and patch it to
populate its supported program/property content.


1. Call `inspect_file` with `path` and `section:"overview"` to obtain `revision` and identify the CPU/project. Paths are absolute or relative to the server's startup directory and must resolve inside an allowed root.
2. For a target described by name, address, description, operand, mnemonic or comment, call `search_project(path, query, kind?, program_index?)`. Results include matched fields, linked symbols, revision-scoped locations and limitations. Pagination uses one list; supply `expected_revision` from page one for later pages and restart on stale revisions. SFC/ST/IL bodies and declarations are outside search coverage. A search match does not establish editability. Inspect the target subsystem. `programs` identifies document-order program indices and object identities. For program detail or `program.*` sections, supply `program_index`. SFC entities/rows use targeted `sfc.block.<numeric block index>` and declarations use `sfc.variables`, both with `program_index`. Arrays use `{items,total,offset,limit,next_offset}`; collect complete pages at one revision when constructing an expected snapshot.
3. Export the target with `get_ir`: use `section:"modules"`, `"variables"`, or `"networks"` for properties, or `section:"program"` with `program_index` for program text. Prepare an exact unified diff preserving identities, readonly statements and the exported revision.
4. Call `apply_patch` with the same selection, inspected `expected_revision`, and diff. Use `dry_run:true` for a preview. If the IR cannot express the requested action, use a remaining `xgwx_<operation_name>` tool with its typed `args`. Consult `describe_operation.editing_route` for partial coverage. `apply_edits` batches only these direct gap operations.
5. Save with `dry_run:false` using the original inspected revision. Report the returned revision, changed status, backup path, and validation. Re-inspect after structural changes before preparing edits that depend on moved indices or offsets.

Dry-run uses the same editing and verification pipeline without saving files or creating backups; its `revision_after` belongs to the candidate. A no-op creates no backup. An error aborts the entire request without changing source bytes. If the revision is stale, inspect again and rebuild the edit from current data. Do not merely replace the hash on an old request.

Cooperating server saves use a persistent `.xgwx-mcp.lock` sidecar. Coordinate simultaneous edits with XG5000 or other programs that ignore the advisory lock; the revision recheck can detect changes but does not lock those programs.

Use `validate_file(path)` to report container verification and available decoded-program diagnostics separately. When the library rejects a format, topology, type, or edit site, report its precise reason and retain the source. If a later save failure returns `error.backup_path`, report the retained recovery artifact. Do not bypass restrictions with raw XML/binary rewriting or weaken expected-value checks. Backup retention does not authorize automatic deletion or rollback.

## Common edits and reusable previews

For project naming and program metadata, export `get_ir(section:"project")` or
`get_ir(section:"programs", program_index:...)`, then patch the editable fields.
Omit the index to export all programs and support lifecycle edits: append/delete/move one complete block as a sole mutation, preserving retained blocks. Read [property-patches.md](references/property-patches.md#program-lifecycle-in-the-full-list) for authoring syntax. Existing name, task, version, kind,
instance_name and comment are supported; missing attributes remain readonly.
For module comments and contact operands, use the modules or program section.

Use `prepare_change` for a reviewed batch of common edits, then
`commit_change(change_token)` to save the exact verified candidate. Inspect the
readable changes and validation first. Tokens are single-use after success,
session-scoped and expire after 30 minutes; references are file/revision-bound.
Reinspect and rebuild when stale. Known errors provide `recovery`; writer errors
identify the operation and schema lookup. Preserve existing native constraints
and report native acceptance as unverified unless XG5000 was checked.

## New project files

Use `list_templates()` then `create_file(path, project_name?, cpu_model?, program_language?, program_name?, operations?, dry_run?)` for a new `.xgwx` file. Read [create-file.md](references/create-file.md) first. Preview without disk changes; save only to an absent destination in an existing allowed directory. Choose the project name, destination, CPU and LD/SFC/ST/IL program language from advertised creation_choices; the seed is derived automatically. Unsupported combinations are rejected. Omitted names retain defaults; seed identities and CPU migration guards are preserved. Inspect the created revision before further edits; later changed edits create normal backups. To add a program to an existing file, use the complete programs property export and its lifecycle authoring block.

## Focused discovery and property patches

Use `list_modules`, `list_module_catalog`, `list_variables` and `list_networks`
for focused discovery. Read [property-patches.md](references/property-patches.md),
then export with `get_ir(section:"modules"|"variables"|"networks")`. List responses
are not patch text. Edit only marked properties; preserve readonly identities,
field order, scalar types and the exported revision. Use `apply_patch` with the
same section and optional variable program selection. Model replacement and
structural objects remain direct operations; IEC local rename/type changes must
be the sole mutation followed by renewed export. Native acceptance stays separate.

## Positional program text patches

For supported SFC supported linear/positioned branch sequence replacement and row fields, direct entity comments/conditions and decoded declaration creation/update/removal, as well as contact/coil creation/deletion and edits, XGK catalog instruction/comparison creation/deletion and mnemonic/operand edits, physical-wire deletion/vertical creation, bound IEC function operand and captured arithmetic/comparison name edits, native comment, and symbol-description changes, use `get_ir(path, program_index)` and patch its returned text with an exact unified diff through `apply_patch`. Read [text-ir.md](references/text-ir.md) for native XGK/IEC positions, syntax, creation/deletion limits, and comments. XGK output column 9 uses native x=94. New objects use fresh @new-<name> identities; retained source identities stay fixed; ladder positions are immutable and SFC positions retain native layout checks. Keep the exported revision, identities, positions and read-only derived connections; the server derives expected values and offsets. Instruction creation/deletion/arity changes, connected IEC contact/terminal-coil deletion and wire changes each require a patch with exactly one mutation. SFC row changes target one chart and must be the only writer operation. Read [sfc.md](references/sfc.md#positional-text-patches) before editing SFC text. Re-export after saving, especially after chart replacement, wire or structural comment edits.

## Focused references

- For a specific XGK/XGB instruction, find its individual file through [instructions/index.md](references/instructions/index.md). The portable helper `scripts/find_instruction.py "TON"` returns the exact guide and writer route; `--query "timer"` searches reviewed purposes and aliases. Load only that instruction's file. Manual spellings, native writer mnemonics, reviewed operands, and known source errors are recorded separately. Korean source notes and figures retain behavior that has not been translated; catalog-only entries identify missing manual coverage.
- For metadata, create/delete/reorder, globals, and IEC local symbols, read [programs-and-symbols.md](references/programs-and-symbols.md).
- For XGK/IEC cells, contacts, coils, functions, wires, rows, and groups, read [ladder.md](references/ladder.md).
- For CPU models, module placement/options, and Cnet/FEnet settings, read [hardware-and-networks.md](references/hardware-and-networks.md).
- For SFC entities, complete sequences, branches/actions, and declarations, read [sfc.md](references/sfc.md).

## Native acceptance

Distinguish container verification, decoded-program diagnostics, and native acceptance in the result. Some supported edits intentionally leave incomplete circuits or charts; successful serialization does not establish executable program validity.

Native acceptance requires XG5000 Open → inspect the changed area → Check Program → Save As to a separate file → inspect/compare the resave. Record native errors/warnings and compare relevant decoded payloads/metadata plus unrelated retained sections. If this was not performed, say that native acceptance remains unverified. Do not infer acceptance from local parsing or IL generation. This skill does not authorize PLC communication, deployment, client configuration changes, or unrelated files.

## Standalone ST/IL source

Read [st-il.md](references/st-il.md) for ST/IL creation, source patches and
advanced declarations. Inspect `text` with `program_index` for exact source,
ObjectId, language, editability and declarations. Use `get_ir` → `apply_patch` for source, and `xgwx_edit_text_variable` for declarations.
Preserve line endings, full expected values and declaration metadata; native
syntax/type validation remains XG5000 Check Program.

For primitive IEC local creation/deletion, use the variables property IR and the
[lifecycle syntax](references/property-patches.md#iec-local-variable-lifecycle).
For I/O variables use the separate `generate_io_variables` tool with path,
expected_revision, optional base/slot and dry_run. Full allocation precedes filtering;
re-export variables after saving. Global lifecycle remains unsupported.

Blank-row and rung lifecycle uses program IR: add `blank_row @new-space after row N` (IEC) or `before y N` (XGK), or `rung_row @new-name at row N contact NO "operand" coil OUT "operand"` (IEC). Remove exported writable row markers for supported IEC deletion. Apply one lifecycle mutation with the remaining body unchanged, then re-export. See the text IR reference for native limits.

IEC program IR also exposes captured `contact_site`, `parallel_site`, and `wire_gap` metadata. Add one `contact_insert`, `parallel_contact`, or `wire_restore` statement referencing a current site; `comment_row` creates a native comment from an existing captured template. Retain the rest of the IR and re-export after saving. See the text IR reference for syntax and bounds.
