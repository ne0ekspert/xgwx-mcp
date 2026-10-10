# Creating a new project file

Use `list_templates()` before creating a project. No existing project path is
needed. It returns `xgk_ld`, `xgi_ld`, and `xgi_sfc`, their native default CPU,
selectable CPU models, source hashes, identity policy and a flattened
`creation_choices` array of supported CPU/language combinations. Native names default to
project `NewProject`, PLC `LSPLC`, and program `NewProgram`. Internal identities
are preserved from the blank seed; creation does not generate new GUIDs.

Call `create_file` with the chosen project name, new `.xgwx` path, CPU model
and program language. The server chooses the native blank template:

```json
{
  "path":"/path/to/projects/machine.xgwx",
  "project_name":"Machine Control",
  "program_name":"Main",
  "program_language":"LD",
  "cpu_model":"XGK-CPUH",
  "operations":[
    {"name":"update_program","args":{"program_index":0,"patch":{"comment":"Main scan"}}}
  ],
  "dry_run":true
}
```

All fields other than `path` are optional for backward compatibility. Omitted
names retain native defaults. With no CPU/language/template, creation uses XGK LD.
With SFC and no CPU, it uses the XGI SFC default. CPU models are case-insensitive.
`program_language` accepts `LD` or `SFC`; currently supported choices are XGK LD,
XGI IEC LD and XGI SFC under the advertised CPU guards. ST/IL and other families
have no supported blank here; never claim the full native XG5000 language matrix.
Project/program names must be nonblank and contain no control characters; Unicode,
spaces and XML-special characters are safely encoded. `project_name` is stored
inside the project independently of the destination filename. An optional legacy
`template` selects a seed explicitly; a conflicting language is rejected.

`cpu_model` and `operations` are optional. Omit CPU selection to retain the seed's
native model, or choose an entry in that template's `selectable_cpu_models`.
Discovery probes the existing public CPU writer; it does not promise arbitrary
CPU-family migration. Discover initial operation schemas with `list_operations`
and `describe_operation`. CPU selection runs first, then project/program naming and ordered operations
on one fresh seed. Any failed edit or verification prevents creation.

The parent directory must already exist and resolve within the server startup
root or an explicit `--root`. The destination must be absent. Files, directories,
and symlinks, including dangling symlinks, are never overwritten. The tool checks
the parent again before publishing; publication uses an atomic no-clobber save
from a same-directory temporary. Competing requests produce one winner; a
`file_exists` error does not authorize overwriting or changing the existing file.

Dry-run writes no file, temporary, lock sidecar or backup. Review its validation
and operation results, then repeat the request with `dry_run:false`. The result
contains `created`, `dry_run`, the canonical `path`, `revision_after`, template
metadata, chosen project/program names, program language, final CPU model,
operation results, and validation. `revision_before`
and `backup_path` are null because no original file exists. New files use private
owner-only permissions on Unix. A successful save may leave the ordinary lock
sidecar used to coordinate subsequent edits.

After creation, inspect the saved file and use its current revision for
`apply_edits` or `get_ir`/`apply_patch`. Subsequent changed edits retain normal
adjacent backups. `create_program` adds a program inside an existing file and is
separate from `create_file`. Blank projects can have incomplete-program
limitations; local verification does not replace native XG5000 Open → Check
Program → Save As → comparison. The tools do not communicate with or deploy to a
PLC.

To rename an existing project, discover `rename_project` and provide its inspected
`expected_name` together with the new `name` in `apply_edits`.
