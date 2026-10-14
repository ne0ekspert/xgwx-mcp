# Common edits using object references

Locate files with shell commands. Inspect or export the chosen target:

| Task | Reference source | Tool and value field |
| --- | --- | --- |
| Rename a program | `get_ir(section:"programs", program_index:...)` | `apply_patch`, editable name |
| Update a module comment | `get_ir(section:"modules")` | `apply_patch`, editable comment |
| Change one contact operand | `get_ir(section:"program", program_index:...)` | `apply_patch`, contact operand |

Export the property or program document and submit an exact unified diff with
its current revision. Use `dry_run:true` for a preview; saved changes use normal
backup and verification behavior. Contact edits affect one use and do not rename
a symbol. Object references remain useful for the prepared preview workflow
below; its explicit `rename_program` change type is still supported.

## Prepare and commit

`set_module_comment` and `set_contact_operand` remain change types inside
`prepare_change`; they are not standalone MCP tools.

Call `prepare_change` with a batch of common changes:

```json
{
  "path":"machine.xgwx",
  "changes":[
    {"type":"rename_program","object_ref":"<program reference>","name":"Conveyor"},
    {"type":"set_module_comment","object_ref":"<module reference>","comment":"Feed conveyor"},
    {"type":"set_contact_operand","object_ref":"<contact reference>","operand":"P00001"}
  ]
}
```

Values are examples; choose valid values for the inspected CPU/language. Batches
contain 1–128 changes, each object once, using references from the same unchanged
file. Any writer/verification failure aborts preparation without creating files,
backups or lock sidecars. Review `changes`, `validation` and the candidate hash.
Then send only the returned token to `commit_change`:

```json
{"change_token":"<reviewed change_token>"}
```

Commit checks the original revision and regenerated candidate hash under the
same transaction locks before backup/replacement. It saves the exact candidate
and consumes the token only after success. No-ops create no backup. A failed
commit can create the normal lock sidecar; retained backups must be reported.

Tokens expire after 30 minutes and do not survive restarting the server. At most
128 previews are retained in memory. References can survive a restart if the
same file remains unchanged. Repeated successful commits with the same token
are rejected. Preparation tokens do not authorize unrequested edits.

## Recovering from failures

- `stale_revision`: inspect/search again and rebuild from current references;
  never substitute a new hash on an old request.
- `invalid_object_ref` / `reference_kind_mismatch`: select the target in this
  file again and copy its matching reference.
- `expired_change_token` / `invalid_change_token` / `candidate_changed`:
  inspect again and prepare a fresh preview.
- `operation_failed` / `unsupported_target`: inspect edit sites and the original
  constraints. Writer errors include `operation_name` and an exact schema lookup
  in `error.recovery.describe_operation`. Correct the request or report the
  unsupported shape; never bypass it through XML/binary edits.

Keep any returned recovery backup. Local verification is not native XG5000
Check Program or PLC acceptance.
