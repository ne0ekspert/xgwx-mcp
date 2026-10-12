# XGWX agent editing server contract

## Purpose and dependencies

Provide a local stdio MCP server that exposes every current public editing method on `libxgwx::XgwxDocument` while preserving the library's format restrictions. The server owns its Serde/Schemars protocol DTOs and depends on a commit-pinned [libxgwx Git dependency](https://github.com/ne0ekspert/libxgwx) with `write` and `il` enabled. Use the official Rust MCP SDK (`rmcp`), Tokio, Serde, and Schemars; retain resolved dependency versions in `Cargo.lock`.

Do not edit MCP client configuration, contact a PLC, deploy the server, prune backups, or modify unrelated checkouts as part of normal server use.

## MCP interface

All tool arguments and inspection DTO fields use snake_case. Operation names are public Rust method names. Preserve compatibility aliases and identify their canonical handler in the operation catalog and coverage manifest.

| Tool | Arguments | Result |
| --- | --- | --- |
| `list_templates` | none | Compatibility profile IDs, default names/CPU, generated hashes, identity policy and probed compatible CPU models. |
| `create_file` | `path`, optional `project_name`, `program_name`, `cpu_model`, `program_language`, `template`, `operations=[]`, `dry_run=false` | Created file revision, selected names/CPU/language, template, results and validation; no original revision or backup. |
| `inspect_file` | `path`, `section="overview"`, optional `program_index`, `offset=0`, `limit=100` | `{path, revision, data}` |
| `search_project` | `path`, nonempty `query`, optional `kind`, `program_index`, `case_sensitive=false`, `offset=0`, `limit=100`, `expected_revision` | `{path, revision, data}` with one paginated result list, matched fields, linked symbols, native locations and coverage limitations. |
| `list_modules` | `path`, offset/limit | Revision and installed modules/settings/rack context. |
| `list_module_catalog` | optional query, offset/limit | Available model catalog; no file required. |
| `list_variables` | `path`, optional program_index, offset/limit | Revision, globals and IEC locals; SFC declarations retain their dedicated inspection workflow. |
| `list_networks` | `path`, offset/limit | Revision, network/module metadata and Cnet/FEnet configuration. |
| `list_operations` | optional `query`, optional `subsystem` | Searchable operation descriptions grouped by subsystem. |
| `describe_operation` | `name` | Exact JSON argument schema, preconditions, required inspection fields, example, and alias information. |
| `rename_program` | `path`, `object_ref`, `name`, `dry_run=false` | Verified transaction plus readable `changes`. |
| `set_module_comment` | `path`, `object_ref`, `comment`, `dry_run=false` | Same; physical selection remains unique. |
| `set_contact_operand` | `path`, `object_ref`, `operand`, `dry_run=false` | Same; single XGK/IEC contact use. |
| `prepare_change` | `path`, `changes=[{type,object_ref,name or comment or operand}]` | Dry-run transaction plus readable changes, session-scoped token and expiry. |
| `commit_change` | `change_token` | Exact-candidate transaction plus readable changes; consumes token after success. |
| `apply_edits` | `path`, `expected_revision`, `operations=[{name,args}]`, `dry_run=false` | `{path, revision_before, revision_after, changed, dry_run, backup_path, results, validation}` |
| `get_ir` | `path`, section=program, optional program_index (required for program) | `{path, revision, program_index, format, dialect, text, capabilities, limitations}` |
| `apply_patch` | `path`, section=program, optional program_index (required for program), `expected_revision`, exact unified `patch`, `dry_run=false` | Same transaction result as `apply_edits`. |
| `validate_file` | `path` | `{path, revision, validation}` |

`revision` is the SHA-256 digest of the current file bytes. `revision_after` is the candidate digest during dry-run and the saved digest during a successful write. `backup_path` is absent/null when no backup was created. Tool errors carry `code`, `message`, optional `operation_index`/`operation_name`, and optional `backup_path` when an original-file backup was retained before a later write failure. Library failures retain their meaningful reason and failed operation context. Stdout contains only MCP traffic.

Inspection sections include `overview`, `programs`, `program`, `symbols`, `hardware`, `networks`, `sfc`, `sfc.variables`, `sfc.block.<block_index>`, `io_variables`, `program.rows`, `program.records`, `program.cells`, `program.strings`, `program.functions`, `program.references`, `program.operand_links`, `program.graph`, `program.edit_sites`, `catalog.cpu`, `catalog.modules`, `catalog.instructions`, and `catalog.comparisons`. Program-specific sections use `program_index`; use a numeric block index in targeted SFC block sections. Array pages contain `{items,total,offset,limit,next_offset}`; callers must collect complete pages before constructing complete expected snapshots. Return decoded positions, binary offsets, identities, current values, type information, and supported edit sites where available.

Cover CPU/hardware, network/Cnet/FEnet, global and IEC local symbols, program lifecycle, XGK ladder, IEC ladder/functions/topology, and SFC. Track each public writer in `operation-coverage.json`, connecting the library method, exposed operation, schema, and adapter. Serialization and filesystem helpers are internal to the server.

## Semantic search

`search_project` is read-only and requires a known file path. It searches program
metadata, globals, IEC LD locals and decoded XGK/IEC ladder contacts, coils,
instructions/functions and native comments. Queries are literal substrings;
case-insensitive matching uses Unicode lowercase by default. Optional `kind`
selects `program`, `variable`, `contact`, `coil`, `instruction` or `comment`.
`program_index` restricts program content while retaining globals.

Link operands to symbol names/addresses by exact case-insensitive equality;
local names shadow matching global names, while ambiguous address matches stay
visible. Do not infer expression references or normalize device addresses.
Search semantic strings, not serialized JSON keys or numeric coordinates.
Return every matching object separately, with available native locations and
`matched_fields`. Preserve document-order identities and never invent a missing
coordinate. Decoded XGK rung order is distinct from physical `raw_y`; IEC
locations retain `group_index` and `row_index`.

Paginate one matching stream after filtering, with a page size of 1–1000.
An optional `expected_revision` rejects changed sources with `stale_revision`;
clients should supply it on subsequent pages and restart after a mismatch.
Report SFC/ST/IL bodies/declarations, opaque records and decode failures in
`limitations`. Zero matches is not proof of absence in unsupported content.
A search result does not establish editability; retain existing edit-site,
expected-value, identity and revision checks for subsequent mutations.

## Common references and prepared changes

Inspection/search may add `object_ref` to supported program metadata, installed
physical module records and contact search results. References encode a version,
source revision and selector plus a SHA-256 fingerprint of canonical path,
revision, selector and observed snapshot. They are opaque locators, not authority.
Resolve against current decoded data, check kind/fingerprint and lower to guarded
public operations. Never substitute a new hash on stale offsets. Unsupported
marker-only contacts expose no operand-edit reference. No reference cache is needed.

`prepare_change` verifies a batch of 1–128 common changes without filesystem writes
and caches path, original/candidate hashes, operations and readable summaries in
memory. Reject multiple changes to the same object in one batch. Keep at most 128
previews per server instance; tokens expire after 1800 seconds using monotonic
elapsed time. Tokens identify a server session and are consumed only after a
successful commit, including no-ops. Failed commits may be retried while valid,
subject to the original revision/candidate checks.

Commit runs under existing file/process locks and checks regenerated candidate
bytes against the reviewed candidate hash before backup/replacement. A mismatched
candidate returns `candidate_changed`. References can survive restarts for an
unchanged file; preparation tokens cannot. Known tool failures add recovery
instructions without weakening the original error; writer failures include
`operation_name` and a `describe_operation` lookup.

## Transaction and file preservation

Resolve input paths canonically within allowed directory roots. Start with the process's startup directory; repeatable `--root` arguments extend that set. Relative file paths resolve against the startup directory. Enforce roots after resolving symlinks. Read the file again for every request and reject stale `expected_revision` values.

Apply ordered operations to one fresh in-memory document. A library precondition failure, unsupported layout, or serialization/verification failure aborts the whole transaction. Serialize the candidate with `to_verified_bytes()` before any replacement. Do not weaken expected-value, identity, CPU, type, offset, or topology checks. A failed edit leaves the source bytes untouched.

Dry-run follows the same edit and verification pipeline without creating a backup or replacing the file. A candidate identical to the original is a no-op and creates no backup. Changed output creates a unique adjacent backup containing the original bytes and stages candidate bytes in a temporary file in the source directory. Preserve source file permissions and replace atomically only after backup succeeds. Retain backups until explicitly removed.

Serialize cooperating writes per canonical path and recheck source revision immediately before replacement. A persistent `<filename>.xgwx-mcp.lock` sidecar coordinates server processes across atomic inode replacements. Other programs that ignore the advisory lock can still race the final revision check; coordinate editing with XG5000 or other external editors. Reject detected concurrent changes rather than replaying operations against different bytes. Indices and offsets belong to the inspected revision; after structural edits, agents re-inspect before preparing further dependent operations.

## Format and validation boundaries

The server exposes the existing library's supported shapes. It does not turn decoded unknown layouts into editable layouts or reinterpret Rust aliases as independent implementations. XGK physical `raw_y` and fixed cell columns differ from IEC group, row, block, and record coordinates. Typed IEC functions require compatible symbols, operands, and output destinations. CPU selection follows the library's current CPU-family and retained-hardware rules; it does not perform arbitrary project migration.

Validation reports container integrity, decoded-program diagnostics, and external/native acceptance separately. Parser or IL failures should remain visible when a program has unsupported records or topology. Preserve APIs that intentionally allow incomplete circuits/charts to serialize and report their diagnostic limits. A successful `to_verified_bytes()` does not establish that XG5000 Check Program passes.

Native acceptance is XG5000 Open → inspect the changed area → Check Program → Save As to a separate file → parse and compare the resaved project. Compare relevant decoded payloads/metadata and unrelated retained sections, allowing for native normalization. Record native errors and warnings separately from local checks. Do not claim native acceptance when this sequence was not performed.

## Required checks

- Exercise real stdio MCP initialization, tool discovery, argument schemas, inspection, dry-run, edit, and validation using an MCP client.
- Exercise each distinct writer adapter using existing or fixture-derived inputs; include successful cases across every subsystem and preservation of unrelated payloads.
- Cover stale revisions, invalid arguments, unsupported shapes, transaction rollback, backup/write failure, no-op requests, root escapes, and competing writes.
- Validate skill frontmatter and references. Run focused library checks without accidentally including unrelated untracked examples.

Preserve user project bytes and existing unrelated checkout changes during validation. Fixture transformations and failure injection belong in isolated temporary directories.

## Positional text IR v1

Provide a typed internal projection and deterministic non-JSON program language, exported for one document-order program plus global and selected local symbol descriptions. Use native program ObjectId metadata, revision-scoped record/symbol references, XGK raw x/y and IEC group/row/anchor positions. Include native rung and output comments as separate positional objects and retain the distinction from shared symbol descriptions. Export derived connectivity, opaque records and unknown instruction shapes as read-only statements. IEC decoder failure must not fall back to XGK.

Accept an exact unified diff against a fresh export under the same canonical-path transaction locks and SHA-256 check as direct editing. Check hunk ranges, old/new counts, context and new-output positions; reject fuzz, multiple file patches, malformed syntax, duplicate identities and changes to frozen statements. Quoted UTF-8 literals use escaped newlines/quotes/backslashes; statements occupy one physical line. No shell or filesystem patch command executes.

Lower supported semantic differences into existing public writer operations with derived expected values. V1 supports SFC linear row creation/deletion/reordering and existing row fields, guarded direct entity comments/conditions and declaration descriptions, plus XGK cell and catalog instruction/comparison creation/deletion, instruction mnemonic/operand edits, IEC empty-row element and terminal-coil creation, IEC isolated/connected-contact and terminal-coil deletion, physical-wire deletion and supported vertical-wire creation, XGK/IEC contact and coil kind/operand changes, bound IEC function operand edits and captured arithmetic/comparison name changes, existing comment text, global/IEC local symbol descriptions, and XGK native comment creation/rung-comment deletion. Existing positions, object families, derived connection statements, unknown instruction contents, function signatures/pin bindings and declaration names/types/addresses remain read-only. New objects require fresh @new-<name> identities and valid matching scopes/coordinates. XGK contact columns 0–8 map to native x=1,4,…25, while output column 9 maps to native x=94. A blank XGK program exposes a y=0 authoring rung. IEC exports row envelopes or an explicit group=0,row=0 empty-canvas site for the untouched zero-count header. Reject duplicate occupied positions and ambiguous IEC rows. Isolated IEC deletion clears its native row record; connected contact and terminal-coil deletion use their exact native writers. Wire creation/deletion and connected IEC deletion must be the sole mutation in a patch; re-export before dependent edits. Physical horizontal/vertical wire statements can be removed, and supported vertical wires added with fresh identities. XGK branch endpoints require exported adjacent rungs (raw y delta 4) and x=3,6,…27; IEC endpoints require exported rows in one group with equal boundary x. Retained wire endpoint/kind changes and horizontal drawing remain unsupported through text patches. Native rung-comment insertion/deletion shifts rows as the library specifies; export again afterward. Output-comment deletion and IEC comment creation/deletion remain unsupported through text patches.

Apply offset-sensitive edits/deletions in descending original string/record-offset order, then coordinate-based element additions (contacts before coils), and native comment-row insertions/deletions afterward in descending y order. Return invalid_patch for malformed/context-mismatched text and unsupported_ir_edit for semantic changes without a v1 lowering. Retain underlying writer failures. Empty or semantically unchanged patches are valid no-ops with no backup. Preserve the original document and all unrelated/opaque payloads rather than reconstructing binary content from the text. Native acceptance remains external.

Test exact diff parsing, both dialects, native positions and string-offset mappings, multiple resizing records, multiline comments, creation/deletion, symbol descriptions, unsupported moves/topology, malformed/duplicate references, no-op/dry-run, rollback after a preceding successful operation, stale/competing requests and real MCP stdio discovery/export/patch/save.

## Executable operation guidance

Embed executable local fixture workflows in describe_operation, explicitly separated from argument templates. Include fixture SHA-256, required setup operations, actual arguments and a local-validation boundary. Identify exact inspection fields for common workflows, especially record offsets versus string offsets and XGK raw output coordinates. Never imply that fixture positions/identities are defaults for user projects.

Replay each bundled workflow through the actual adapters, verify and reparse after every operation, assert final behavior, and preserve declared unrelated program payloads plus global symbols. Keep a visible example count per operation; the bundled registry is evidence for its cases, not a claim of success coverage for every public writer or native XG5000 acceptance.

IEC function text patches expose decoded name offsets and source fingerprints plus each argument's function target, ordinal, pin row/x, direction, type mask and array flag. Change names only within captured arithmetic or comparison families, and bound argument values through the existing operand writer. Preserve signatures, positions and bindings. Reject empty argument values, implicit function/argument deletion, creation and rebinding. Library type, scalar compatibility, expression and writable-output checks remain authoritative; failures roll back the complete request.

XGK instruction statements expose APPLICATION/COMPARISON family, native position, mnemonic and separately quoted operands. Derive the complete source expected string, not the mnemonic-only cell value. Retained family and coordinates are immutable. Catalog applications anchor at x=94-3*operand_count; comparisons use the requested native contact column. Route creation, deletion and complete text updates through the public native writers. Structural preflight on a disposable document keeps undecoded layouts read-only and prevents raw-string fallback. Creation/deletion/arity changes require one mutation followed by re-export; same-arity updates use descending source offsets. Reject mnemonic/operand delimiter injection while retaining native single-quoted string literals.

SFC text export detects native SFC XML before ladder decoding. Supported charts expose typed existing row fields; derive complete expected entities/rows internally and lower all changed fields in one chart into one replace_sfc_sequence operation. Keep retained row kinds/coordinates, positioned branch order/endpoints and chart/entity identities fixed. Linear charts permit row insertion/deletion/reordering within one block; new rows require fresh @new- identities, all exported fields, linear kinds, position none and branch_end none. Do not invent coordinates or move retained rows across charts. Use text order as the desired typed sequence and preserve complete source snapshots. Row changes target one chart and must be the only writer operation; re-export afterward. Unsupported charts retain their full property text and only expose existing step-comment/direct %MX condition fields through edit_sfc_entity. Existing declaration descriptions use edit_sfc_variable with complete sequential expected-variable snapshots; preserve name/type/system/advanced declaration metadata. Optional row fields use none or quoted text, including escaped multiline ST. Preserve native layout, source sharing, BOOL, qualifier, TIME, system and mapped-declaration guards; never equate local verification with native acceptance.

## Project-file creation

Generate XGK LD/vendor IL and XGI LD/SFC/ST/IL creation profiles through libxgwx::create_project; source paths/hashes for library defaults
are recorded in libxgwx/src/project_defaults/manifest.json. Do not accept arbitrary
client template paths or read/embed template .xgwx files for creation. Preserve native
library default identities and unselected defaults rather than inventing undocumented GUID,
parameter or cache rewriting. list_templates reports default names/model and
models accepted by guarded CPU-selection probes on each seed, with a flattened
creation_choices CPU/language matrix. Accept project_name, path, cpu_model,
program_language (LD/SFC/ST/IL), and optional program_name directly. Derive the seed
from the CPU family and language when template is absent; retain legacy template
requests and reject conflicts. Families without supported captured templates remain unsupported. Rename the project through the public rename_project writer
with an expected-name guard and exact XML text replacement, preserving all other
metadata/payloads. Project/program names must be nonblank with no control characters;
Unicode and XML escaping are supported. Default omissions retain native names;
no CPU/language/template selects XGK LD.

create_file resolves the existing parent canonically within allowed roots and
requires an absent .xgwx destination. Reject existing files, directories and
symlinks including dangling links. Apply optional CPU selection then ordered
project/program naming and initial edits to a fresh document, retain native preconditions, and serialize with
to_verified_bytes before creating any sidecar/temp/output. Dry-run creates no
filesystem entries. Serialize creation against edits by canonical target path
and the shared OS lock; recheck parent resolution and destination absence before
publication. Publish a complete same-directory temporary with atomic no-clobber
semantics so external or competing creators cannot be overwritten. Sync candidate
bytes before publication; use private Unix file permissions. Failed edits,
verification or writes must not publish partial data. Preserve an external winner
on publication races. Creation has no original revision/backup; subsequent edits
use normal revisions/backups. Native acceptance and PLC execution remain separate.

Test all templates, provenance, CPU discovery/selection and unsupported models,
initial-edit rollback, no-write dry runs, existing files/directories/symlinks,
canonical roots/additional roots, missing parents, write/publication failures,
competing server stores, real stdio discovery/schema/preview/create/inspection/IR,
and a first ordinary backed-up edit.

## Property documents

Expose complete xgwx-properties/1 exports for modules, variables and networks.
File readers and property exports retain revisions; standalone catalog discovery
has no file identity. Bind marked edit properties to native writers internally
under the transaction locks; never accept client source mappings. Keep headers,
object identities/order, field keys, markers and readonly lines fixed. Reject
creation/deletion, non-scalar injections, nulling fields, type changes and forged
identities. Equivalent string escapes are no-ops. Missing/undecoded fields remain
readonly. Restrict local rename/type changes to one mutation then require export.

Support module comments/decoded options, global name/address/type/description,
IEC local name/type/description, guarded network names/module metadata and
exported Cnet/FEnet settings. Coalesce global fields per index and Cnet settings
per base/slot so native interdependent guards run on one proposed result.
Preserve positional program patch compatibility; program_index stays required
when section defaults to program. Property patches use the same rollback,
verified serialization, backups, roots, revision checks and atomic replacement.
Tests must cover focused tool discovery/pagination, real stdio property export,
preview/save/stale rejection, representative writer success and unrelated payload
preservation, readonly/identity/shape/type restrictions, failure rollback and no-op.


## Standalone source programs

Expose all public text writers (`edit_text_program`, `edit_text_variable`, `edit_vendor_il`) with
strict snake_case DTOs and coverage entries. Inspect source programs separately
from ladder/SFC using text/text.variables, including exact source, program index,
ObjectId, language, editability/reason and complete advanced declarations. Program
inspection must recognize ST/IL before attempting ladder decoding. Validation
reports decoded source and UTF-16 length, with syntax/type status not_checked.

ST/IL create_file profiles use the public library project generator and native
defaults. Record generated hashes and library default provenance. Advertise only CPU choices accepted by the existing select_cpu
guards. Existing create_program accepts ST/IL on supported IEC profiles.

For program IR, export quoted source chunks preserving native newlines, plus
revision/identity/language/read-only metadata. Exact unified diffs may alter only
source statements inside the source block; concatenate their decoded strings and
lower to edit_text_program with the current ObjectId, language and source checks.
Declarations use edit_text_variable and full expected-variable snapshots. Preserve
native encryption/layout/metadata restrictions, NUL prohibition, 65536 UTF-16 unit
limit and declaration/reference guards. Reuse normal atomic transaction, revision,
dry-run and backup behavior. Do not treat source serialization as compilation.

XGK vendor IL inspection uses vendor_il; edit_vendor_il rebuilds the selected native ladder payload, retaining identity/source and straight-line/catalog restrictions. Advertise xgk_il as IL with native LD storage.
