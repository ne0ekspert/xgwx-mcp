# xgwx-mcp

Local Rust stdio MCP server for inspecting and editing LS XG5000 `.xgwx` files through `libxgwx`. Each successful editing request saves one verified transaction and retains the original file in an adjacent backup.

## Build and run

Build with Rust 1.88 or newer. The `libxgwx` dependency uses its published `0.1.0`
baseline with `write` and `il`, plus a package-relative patch in `vendor/libxgwx`
for scoped I/O generation and first insertion into captured empty IEC local tables.
`Cargo.lock` records this override. No sibling checkout is required. The native
patch is documented in [PATCHES.md](vendor/libxgwx/PATCHES.md).

For local library development, add this override to `.cargo/config.toml`:

```toml
[patch.crates-io]
libxgwx = { path = "/path/to/libxgwx" }
```

Run `cargo update -p libxgwx` to select the override; its version must satisfy
the dependency's version requirement. Restore `Cargo.lock` before committing
unless deliberately updating the dependency.
Instruction-guide tools resolve the actual library source through Cargo metadata,
including local overrides. Instruction generation also accepts `--library-root`.
The published crate excludes regression fixtures. To run fixture-based Rust tests
against the published dependency, set `LIBXGWX_TEST_ROOT` to a libxgwx repository
checkout containing `fixtures/`, then run `cargo test --locked`. This selects test
data only; source audits still use the compiled dependency. The same environment
variable supplies fixtures to Python instruction-guide tests.
Run `cargo fetch --locked` before these tests to cache dependencies for all
platforms: the offline Cargo metadata queries also resolve packages that the
host's build does not compile. CI fetches those dependencies and checks out
fixtures at the published library's source revision.

MCP uses the current namespaced library API by default; no opt-in feature is
required. `Cargo.toml` specifies the library version requirement, and `Cargo.lock`
pins the resolved crates.io release and checksum.

Program lifecycle and language-specific edits use selected handles such as
`doc.program_mut().get(index)?.iec_ld()?`. Protocol adapters preserve incoming
identity, language, and expected-value guards when converting to bound patches.
Global variables and protocol settings use `variable()` and `network()`.

Hardware inspection uses `doc.bases()` and each base's `modules()`. Base records
include `base_index`, `configuration_index`, and paginated typed `modules`.
The document-wide `typed_modules` inventory retains unplaced and ambiguous
memberships, I/O classification, and bound network settings. Installed hardware
and property indices continue to use `module_summaries()`. Operation names,
argument schemas, and revision-scoped property indices remain unchanged.

This server tracks 123 public writers in its coverage manifest. An older library
may not build; newer public writers need corresponding server adapters before
the full coverage audit passes. The baseline includes standalone ST/IL and XGK
IL writers, plus the `create_project` generator and native defaults.

```bash
cd /path/to/xgwx-mcp
cargo build --release --locked
```

Launch from the directory containing the projects the agent should access:

```bash
cd /path/to/projects
/path/to/xgwx-mcp/target/release/xgwx-mcp
```

The startup directory is always an allowed root. Add directories with repeatable `--root DIRECTORY` arguments. Relative file paths resolve against the startup directory. The server canonicalizes paths and rejects files outside its roots, including symlinks that escape them.

Configure a stdio MCP client using the binary's absolute path and arguments. An example configuration is in [SKILLS.md](SKILLS.md). The server reserves stdout for MCP messages; startup errors and help go to stderr.

## Tools

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


| Tool | Purpose |
| --- | --- |
| `xgwx_<operation_name>` | Execute one remaining direct gap writer with `path`, `expected_revision`, its exact typed `args`, and optional `dry_run`. |
| `list_templates` | Discover native blank projects and supported CPU/language creation choices without an existing file. |
| `create_file` | Generate a verified new project through libxgwx with optional initial edits; never overwrite an existing destination. |
| `list_modules` | List installed modules, settings and rack context. |
| `list_module_catalog` | Search available models and writable options without a project file. |
| `list_variables` | List globals and IEC locals, optionally selecting a program. |
| `list_networks` | List configured networks, linked modules and Cnet/FEnet settings. |
| `inspect_file` | Read the file's SHA-256 revision, project metadata, programs, symbols, hardware, networks, SFC, catalogs, and available edit sites. |
| `search_project` | Find decoded programs, variables, contacts, coils, instructions and comments inside a known project, with revision-scoped locations and linked symbol descriptions. |
| `list_operations` | Find supported writer operations by query or subsystem. |
| `describe_operation` | Obtain an operation's exact argument schema, constraints, inspection requirements, and example. |
| `prepare_change` | Preview a batch of common edits with before/after values and an exact candidate token. |
| `commit_change` | Save the prepared candidate if the source and candidate still match. |
| `apply_edits` | Apply ordered operations using the inspected revision; optionally dry-run the same transaction without writing files. |
| `generate_io_variables` | Generate native I/O variables with optional base/slot scope and a selected-row preview. |
| `get_ir` | Export deterministic positional XGK/IEC ladder or SFC text with native comments, symbol descriptions, physical wires and read-only derived connections. |
| `apply_patch` | Lower an exact unified diff into guarded writer operations and save one verified transaction. |
| `validate_file` | Report container verification and available program diagnostics separately. |

Operation names preserve the indexed writer protocol, with snake_case arguments; adapters bind those coordinates to the current Rust editors. The live operation catalog is the source of truth for supported arguments and compatibility aliases; [operation-coverage.json](operation-coverage.json) records writer coverage. No PLC connection is involved.

Start with [SKILLS.md](SKILLS.md) for installation and an editing walkthrough. The portable agent skill is [skills/xgwx-edit/SKILL.md](skills/xgwx-edit/SKILL.md). The implementation contract is [SPEC.md](SPEC.md).

## Search within a project

Use shell commands to locate `.xgwx` files, then search their decoded objects:

```json
{"path":"machine.xgwx","query":"Feed conveyor","kind":"contact"}
```

`search_project` matches literal substrings in names, addresses, descriptions,
operands, instruction mnemonics and comments, ignoring case by default. `kind`
is optional (`program`, `variable`, `contact`, `coil`, `instruction`, `comment`);
`program_index` restricts program content while retaining global variables.
`case_sensitive:true` enables case-sensitive matching. Empty/whitespace-only
queries are rejected. Addresses are not normalized and expressions are not parsed.
Contacts, coils and instruction operands are linked to symbols by exact
case-insensitive name/address, so a symbol description can find its ladder uses.
A local name shadows the same global name; ambiguous address matches remain visible.

Results use `{path,revision,data}`. `data` contains a single paginated
`{items,total,offset,limit,next_offset}` list plus `limitations`. Each item carries
its `kind`, `matched_fields`, decoded values and available program identity/location.
Variable indices and ladder offsets refer to the returned revision. XGK locations
include native `raw_x`/`raw_y` and decoded `rung_index` when known; IEC locations
include native `group_index`/`row_index` and record/string offsets. Instruction
locations without decoded coordinates retain only the known string offset.
Use `expected_revision` from the first response with subsequent `offset` pages;
a changed source returns `stale_revision` so the search can restart consistently.

This first version searches program metadata, globals, IEC LD locals and XGK/IEC
ladder bodies. SFC/ST/IL bodies and declarations, opaque records and decode failures
are reported in `limitations`; zero matches is not proof that unsupported content
contains no references. Search does not certify editability. Inspect the relevant
edit sites or export `get_ir` before preparing a guarded edit. Search is read-only
and creates no backups or lock sidecars.

## Common edits and reviewable changes

Supported search results and program/hardware inspection objects include an
opaque `object_ref`. Copy it as-is; it is bound to the canonical file path,
revision, object kind and observed identity/value. It is a locator, not permission
or a guarantee of writability. References survive server restarts while the same
file remains unchanged; preview tokens are limited to the current server session.

For project names, export `get_ir(section:"project")`. For program metadata,
export `get_ir(section:"programs", program_index:...)`, or omit the index for
all programs. Patch the marked fields through `apply_patch`.

For module comments, export `get_ir(section:"modules")` and edit the comment
through `apply_patch`. For contact operands, export the target program with
`get_ir(section:"program", program_index:...)` and patch the operand.

To review a batch once and commit the exact candidate:

Module comment and contact operand change types remain available inside
`prepare_change`; they are not standalone MCP tools.

```json
{"path":"machine.xgwx","changes":[{"type":"rename_program","object_ref":"<program reference>","name":"Conveyor"},{"type":"set_module_comment","object_ref":"<module reference>","comment":"Feed conveyor"}]}
```

Call `prepare_change` with that request. It verifies without creating a file,
backup or lock sidecar and returns `changes`, `validation`, `revision_before`,
`revision_after` and `change_token`. Then call `commit_change`:

```json
{"change_token":"<token from the reviewed preview>"}
```

The server retains the operations and hashes in memory. Under the existing
transaction locks, commit rechecks the source revision and regenerated candidate
hash before any backup/replacement. A successful commit consumes the token.
Tokens expire after 30 minutes and do not survive a server restart. At most 128
previews are retained, and batches contain 1–128 changes with each object used
once. No-op saves consume their token without creating a backup.

Errors retain their existing code/message/index/backup fields and add `recovery`
for known failures. Writer failures also identify `operation_name` and its schema
lookup. A stale reference requires renewed inspection and a rebuilt change;
expired/unknown tokens require a new preview. Do not replace a revision on stale
coordinates or delete a retained recovery backup. Native XG5000 acceptance remains
separate from local verification. See the portable
[common edit guide](skills/xgwx-edit/references/common-edits.md).

## Property text patches

Use the focused listing tools, then `get_ir` with `section:"project"`, `"programs"`,
`"modules"`, `"variables"` or `"networks"`. The complete property document marks each field
`edit` or `readonly`. Send an exact unified diff through `apply_patch` with the
same section and exported revision. Existing program exports use the default
`section:"program"` with a required `program_index`.

Property patches cover project names, program name/task/version/kind/instance_name/comment, module catalog models/comments/decoded options, global variable metadata,
guarded IEC local name/type/address/description changes, network names/module metadata,
and exported Cnet/FEnet settings. Model replacement and structural object edits
remain exposed direct gap tools. Read the portable
[property patch guide](skills/xgwx-edit/references/property-patches.md).

Network property patches cover network names, linked module configuration
names/aliases/descriptions and all existing settings supported by the FEnet
field writer: both IPv4 configurations, DHCP flags and five numeric settings.
The `edit_browser_network` and `edit_fenet_field` wrappers are internal only;
use `get_ir(section:"networks")` → `apply_patch`. Missing fields, identities and
opaque protocol records remain readonly. Native subnet/range/driver validators,
revision guards and atomic rollback are preserved.

## Positional ladder language

Export one program with `get_ir`, then send an exact unified diff to `apply_patch` using its revision. Text IR v1 supports SFC supported linear/positioned branch sequence replacement and row fields, guarded direct entity comments/conditions and decoded declaration creation/update/removal, plus XGK element and catalog instruction/comparison creation/deletion and instruction mnemonic/operand edits, IEC empty-row element and terminal-coil creation, IEC isolated/connected-contact and terminal-coil deletion, physical-wire deletion and supported vertical-wire creation, XGK/IEC contact and coil kind/operand changes, bound IEC function operand edits and captured arithmetic/comparison name changes, existing native comments, global/local symbol descriptions, and XGK comment creation/rung-comment deletion. It preserves explicit native positions and read-only derived connections, unknown instructions, and opaque records. The [language reference](skills/xgwx-edit/references/text-ir.md) defines the syntax and writer boundaries. Existing ladder positions and derived connections remain immutable. SFC row positions and branch endpoints retain native balanced-layout checks. Instruction creation/deletion/arity changes, connected IEC deletion and wire changes must be the sole mutation in a patch; re-export before dependent edits. Capabilities outside IR coverage retain independently exposed direct tools; `apply_edits` batches only those gap writers.

## Verified operation examples

`describe_operation` includes precise inspection fields for common workflows and distinguishes `fixture_verified` examples from argument templates. [Executable examples](examples/verified-operation-cases.json) record local fixture hashes, ordered setup operations, edits, and expected results. Tests replay every step, reparse verified output, and check unrelated program payloads and global symbols. Fixture arguments are examples; inspect actual projects before using their coordinates or identities.

## Individual instruction skills

The [instruction index](skills/xgwx-edit/references/instructions/index.md) routes to 1,166 individual files: 1,120 spellings from the supplied Korean XGK/XGB V3.5 manual and 46 current catalog names without an exact manual topic. Each file records operands, behavior, flags, restrictions, source provenance, diagrams, and its current guarded writer route. Shared variants retain their distinctions; known manual copy errors are identified rather than turned into editing rules.

Find a guide without loading the whole index:

```bash
python3 skills/xgwx-edit/scripts/find_instruction.py TON
python3 skills/xgwx-edit/scripts/find_instruction.py 'ULOADD<'
python3 skills/xgwx-edit/scripts/find_instruction.py --query 'timer'
```

The lookup helper, manifest and instruction files travel with the installable skill. Generation uses the original local manual and the Cargo-resolved library. It can verify metadata through the release server without editing projects:

```bash
python3 scripts/generate_instruction_skills.py \
  --manual "$HOME/Downloads/XGK(B)InstructionHelp_Kr_V3.5.chm" \
  --server-binary target/release/xgwx-mcp --check
python3 -m unittest discover -s tests -p 'test_instruction_*.py'
```

Omit `--check` to regenerate. The generator preserves independently changed guide files and does not include the CHM or raw HTML in the skill. Reviewed English notes complement the retained Korean source explanations; the XGK/XGB manual does not define IEC function behavior.

## Verification

GitHub CI checks Rust formatting, Rust and stdio integration tests, the Python
instruction-guide tests, and a release build with a CLI smoke check. It checks
out the pinned library baseline beside this repository so dependency paths and
fixture tests match local builds.

```bash
cargo test --locked
```

Local verification establishes parsing, serialization, and the available decoded-program checks. Native acceptance requires XG5000 Open → Check Program → Save As → comparison; the server does not perform that external check. Incomplete circuits and SFC charts may serialize successfully while requiring completion before native program checks pass.

Changed saves retain `<filename>.xgwx-mcp-backup-<random suffix>` beside the project. A persistent `<filename>.xgwx-mcp.lock` sidecar coordinates cooperating server processes across atomic replacements. Coordinate saves with editors such as XG5000 that do not use this advisory lock.

### Unix lock-file flags

The `libc` dependency supplies `O_NOFOLLOW` and `O_CLOEXEC` to
[`lock_file`](src/storage.rs) through Rust's `OpenOptionsExt::custom_flags`:
`O_NOFOLLOW` rejects a symlink at the lock-file path's final component, and
`O_CLOEXEC` closes the descriptor when the process executes another program.
These flags are applied only on Unix. The server checks that the opened sidecar
is a regular file, then acquires its exclusive advisory lock through `fs2`.

## New project files

`list_templates` returns supported `creation_choices` with CPU models and program
languages. Choose `project_name`, `path`, `cpu_model` and `program_language`;
optionally choose `program_name` and initial gap-writer `operations`:

```json
{"project_name":"Factory Line 1","path":"factory.xgwx","cpu_model":"XGI-CPUE","program_language":"SFC","program_name":"Main","dry_run":true}
```

The server calls `libxgwx::create_project` for the selected creation profile.
No template `.xgwx` file is read or embedded. LD is available
for guarded XGK/XGI choices, SFC, ST and IL for guarded XGI choices. Other CPU families require captured
templates and are not currently available. The legacy `template` argument remains
optional; conflicting template/language choices are rejected. Omitted names
retain `NewProject`/`NewProgram`, and omitted CPU/language/template uses XGK LD.
Parent directories must exist within allowed roots. Creation verifies the
candidate and atomically saves without replacing existing files or symlinks.
No backup is needed until a later edit. Library default identities are preserved.
See the portable [creation workflow](skills/xgwx-edit/references/create-file.md)
and [template provenance](templates/README.md).

## Standalone ST and IEC IL

Create ST/IL projects with `create_file(program_language:"ST"|"IL")`, or add
programs through the complete `programs` property export. Discover supported CPU/language choices with
`list_templates`. Inspect `section:"text"` (optionally selecting `program_index`)
or `section:"text.variables"` for source, ObjectId, editability and declarations.
`edit_text_program` replaces source with identity, language and expected-source
checks; `edit_text_variable` adds/updates/removes declarations with a complete
expected snapshot. `get_ir` and `apply_patch` support quoted source-line diffs.
See the [ST/IL workflow](skills/xgwx-edit/references/st-il.md).

Source round trips preserve Unicode and line endings. Unsupported encrypted or
metadata-bearing layouts remain read-only. Local validation reports decoding and
container checks; syntax/type compilation requires native XG5000 Check Program.

XGK IL uses `edit_vendor_il` and `inspect_file(section:"vendor_il")`; it rebuilds the selected ladder payload and retains guarded straight-line/catalog limits. Its creation profile is `xgk_il` with native LD storage.

## License

Licensed under GNU GPL version 3 (`GPL-3.0-only`). See [LICENSE](LICENSE) for the full terms.

Program lifecycle also uses `get_ir(section:"programs")`, without an index.
Append one fresh `program_metadata @new-...` block with quoted `edit name` and
`edit language` fields, delete one whole block, or move one whole block to reorder.
Keep retained blocks and readonly context unchanged; each lifecycle patch is a
sole mutation and requires re-export afterward. Native identities are generated
internally and existing writer guards apply. See the [lifecycle reference](skills/xgwx-edit/references/property-patches.md#program-lifecycle-in-the-full-list).

Variable lifecycle uses `get_ir(section:"variables", program_index:...)` →
`apply_patch` for one primitive IEC local declaration creation/deletion per patch.
Writable local tables are explicit. Global variable creation/deletion remains
unsupported; SFC/ST/IL declarations use their existing domain workflows.
I/O generation is separate: `generate_io_variables(path, expected_revision,
base?, slot?, dry_run?)`. Omitted filters select all; slot alone matches that slot
on every base. Full-project allocation/validation runs before filtering, so
addresses retain their native positions. Preview returns selected rows/actions.

Blank-row and rung lifecycle uses program IR: add `blank_row @new-space after row N` (IEC) or `before y N` (XGK), or `rung_row @new-name at row N contact NO "operand" coil OUT "operand"` (IEC). Remove exported writable row markers for supported IEC deletion. Apply one lifecycle mutation with the remaining body unchanged, then re-export. See the text IR reference for native limits.

IEC program IR also exposes captured `contact_site`, `parallel_site`, and `wire_gap` metadata. Add one `contact_insert`, `parallel_contact`, or `wire_restore` statement referencing a current site; `comment_row` creates a native comment from an existing captured template. Retain the rest of the IR and re-export after saving. See the text IR reference for syntax and bounds.
