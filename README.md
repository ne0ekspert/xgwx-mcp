# xgwx-mcp

Local Rust stdio MCP server for inspecting and editing LS XG5000 `.xgwx` files through `libxgwx`. Each successful editing request saves one verified transaction and retains the original file in an adjacent backup.

## Build and run

Keep this repository beside `libxgwx`; the dependency is `../libxgwx` with its `write` and `il` features enabled. Build with Rust 1.88 or newer:

The tested library baseline is commit `91705f5`, with the public `rename_project`
writer used by named-project creation. If that writer is absent, apply the bundled
dependency patch from the library directory:

```bash
git apply ../xgwx-mcp/patches/libxgwx-rename-project.patch
```

This server tracks 120 public writers in its coverage manifest. An older library
may not build; newer public writers need corresponding server adapters before
the full coverage audit passes. The baseline does not include the newer local
standalone ST/IL editing writers.

```bash
cd /home/ne0ekspert/xgwx-mcp
cargo build --release --locked
```

Launch from the directory containing the projects the agent should access:

```bash
cd /path/to/projects
/home/ne0ekspert/xgwx-mcp/target/release/xgwx-mcp
```

The startup directory is always an allowed root. Add directories with repeatable `--root DIRECTORY` arguments. Relative file paths resolve against the startup directory. The server canonicalizes paths and rejects files outside its roots, including symlinks that escape them.

Configure a stdio MCP client using the binary's absolute path and arguments. An example configuration is in [SKILLS.md](SKILLS.md). The server reserves stdout for MCP messages; startup errors and help go to stderr.

## Tools

| Tool | Purpose |
| --- | --- |
| `list_templates` | Discover native blank projects and supported CPU/language creation choices without an existing file. |
| `create_file` | Generate a verified new project through libxgwx with optional initial edits; never overwrite an existing destination. |
| `list_modules` | List installed modules, settings and rack context. |
| `list_module_catalog` | Search available models and writable options without a project file. |
| `list_variables` | List globals and IEC locals, optionally selecting a program. |
| `list_networks` | List configured networks, linked modules and Cnet/FEnet settings. |
| `inspect_file` | Read the file's SHA-256 revision, project metadata, programs, symbols, hardware, networks, SFC, catalogs, and available edit sites. |
| `list_operations` | Find supported writer operations by query or subsystem. |
| `describe_operation` | Obtain an operation's exact argument schema, constraints, inspection requirements, and example. |
| `apply_edits` | Apply ordered operations using the inspected revision; optionally dry-run the same transaction without writing files. |
| `get_ir` | Export deterministic positional XGK/IEC ladder or SFC text with native comments, symbol descriptions, physical wires and read-only derived connections. |
| `apply_patch` | Lower an exact unified diff into guarded writer operations and save one verified transaction. |
| `validate_file` | Report container verification and available program diagnostics separately. |

Operation names follow public Rust writer methods, with snake_case arguments. The live operation catalog is the source of truth for supported arguments and compatibility aliases; [operation-coverage.json](operation-coverage.json) records writer coverage. No PLC connection is involved.

Start with [SKILLS.md](SKILLS.md) for installation and an editing walkthrough. The portable agent skill is [skills/xgwx-edit/SKILL.md](skills/xgwx-edit/SKILL.md). The implementation contract is [SPEC.md](SPEC.md).

## Property text patches

Use the focused listing tools, then `get_ir` with `section:"modules"`,
`"variables"` or `"networks"`. The complete property document marks each field
`edit` or `readonly`. Send an exact unified diff through `apply_patch` with the
same section and exported revision. Existing program exports use the default
`section:"program"` with a required `program_index`.

Property patches cover module comments/options, global variable metadata,
guarded IEC local name/type/description changes, network names/module metadata,
and exported Cnet/FEnet settings. Model replacement and structural object edits
remain direct `apply_edits` operations. Read the portable
[property patch guide](skills/xgwx-edit/references/property-patches.md).

## Positional ladder language

Export one program with `get_ir`, then send an exact unified diff to `apply_patch` using its revision. Text IR v1 supports SFC linear row creation/deletion/reordering and existing row fields, guarded direct entity comments/conditions and declaration descriptions, plus XGK element and catalog instruction/comparison creation/deletion and instruction mnemonic/operand edits, IEC empty-row element and terminal-coil creation, IEC isolated/connected-contact and terminal-coil deletion, physical-wire deletion and supported vertical-wire creation, XGK/IEC contact and coil kind/operand changes, bound IEC function operand edits and captured arithmetic/comparison name changes, existing native comments, global/local symbol descriptions, and XGK comment creation/rung-comment deletion. It preserves explicit native positions and read-only derived connections, unknown instructions, and opaque records. The [language reference](skills/xgwx-edit/references/text-ir.md) defines the syntax and writer boundaries. Existing positions and derived connections remain immutable. Instruction creation/deletion/arity changes, connected IEC deletion and wire changes must be the sole mutation in a patch; re-export before dependent edits. Broader supported writers remain available through `apply_edits`.

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

The lookup helper, manifest and instruction files travel with the installable skill. Generation uses the original local manual and the sibling library. It can verify metadata through the release server without editing projects:

```bash
python3 scripts/generate_instruction_skills.py \
  --manual "$HOME/Downloads/XGK(B)InstructionHelp_Kr_V3.5.chm" \
  --server-binary target/release/xgwx-mcp --check
python3 -m unittest discover -s tests -p 'test_instruction_*.py'
```

Omit `--check` to regenerate. The generator preserves independently changed guide files and does not include the CHM or raw HTML in the skill. Reviewed English notes complement the retained Korean source explanations; the XGK/XGB manual does not define IEC function behavior.

## Verification

```bash
cargo test --locked
```

Local verification establishes parsing, serialization, and the available decoded-program checks. Native acceptance requires XG5000 Open → Check Program → Save As → comparison; the server does not perform that external check. Incomplete circuits and SFC charts may serialize successfully while requiring completion before native program checks pass.

Changed saves retain `<filename>.xgwx-mcp-backup-<random suffix>` beside the project. A persistent `<filename>.xgwx-mcp.lock` sidecar coordinates cooperating server processes across atomic replacements. Coordinate saves with editors such as XG5000 that do not use this advisory lock.

## New project files

`list_templates` returns supported `creation_choices` with CPU models and program
languages. Choose `project_name`, `path`, `cpu_model` and `program_language`;
optionally choose `program_name` and ordered initial `operations`:

```json
{"project_name":"Factory Line 1","path":"factory.xgwx","cpu_model":"XGI-CPUE","program_language":"SFC","program_name":"Main","dry_run":true}
```

The server calls `libxgwx::create_project` for the selected creation profile.
No template `.xgwx` file is read or embedded. LD is available
for guarded XGK/XGI choices, SFC for guarded XGI choices; ST/IL and other CPU
families are not currently available. The legacy `template` argument remains
optional; conflicting template/language choices are rejected. Omitted names
retain `NewProject`/`NewProgram`, and omitted CPU/language/template uses XGK LD.
Parent directories must exist within allowed roots. Creation verifies the
candidate and atomically saves without replacing existing files or symlinks.
No backup is needed until a later edit. Library default identities are preserved.
See the portable [creation workflow](skills/xgwx-edit/references/create-file.md)
and [template provenance](templates/README.md).
