# Hardware and networks

Default to `get_ir` → `apply_patch` for supported edits. Writer names below
identify library APIs and argument shapes. Consult `describe_operation.editing_route`:
`tool_name:null` means use patches; a non-null `tool_name` identifies an exposed
gap tool. `apply_edits` and creation operation arrays reject patch-covered writers.

Inspect `hardware` and `catalog.cpu`/`catalog.modules` to identify the CPU, base capacities, unique base/slot positions, module identities, slot spans, current options, and writable choices. Inspect `networks` for linked modules, native Cnet port order, FEnet values, and metadata. Use `describe_operation` for exact argument names and accepted field keys.

## CPU and module edits

`select_cpu(model)` follows the existing library's conversion behavior:

- Selecting the current CPU type is a no-op.
- XGK-to-XGK changes check retained bases/modules against the target's limits while preserving existing program/hardware/network configuration.
- Captured XGI-to-XGI changes support validated SFC projects with default parameters and empty I/O tables; these may update CPU defaults and model-specific local Ethernet/motion sections.
- Cross-family changes, arbitrary compact-model conversions, uncaptured parameters, and unsupported migrations are rejected.

Patch `edit model` in the modules export for catalog-backed replacement, `insert_module` for an empty physical slot, and `delete_module` for a uniquely identified module. Captured companion network records are maintained by the library. Multi-slot placement cannot overlap another module or exceed base capacity. Compact hardware identity has separate restrictions.

`set_base_slot_count` changes a supported base capacity. `set_module_option` uses the inspected option `key`, channel/group `index`, and numeric catalog choice. Read-only visible dialog fields do not become writable by supplying a guessed key or a hex blob. `set_module_input_filter` has its own supported enum. `update_module` changes supported metadata; omit fields that should remain untouched.

For a module model, comment or decoded option, export `get_ir(section:"modules")`
and patch its supported `edit` value. ID/subtype are omitted; `native_name` is
readonly. Model replacement uses the guarded catalog writer and is the sole
mutation in a patch; it resets model defaults and maintains matching network
records. Re-export before dependent edits. Raw `Details` is not exported: use
decoded `option.<key>.<index>` values and native choices. CPU changes and module
insertion/deletion retain their direct gap tools.

Module insertion, removal, replacement, and CPU changes warrant refreshed hardware/network inspection before later dependent edits.

For requested I/O symbol generation, inspect `io_variables` to review the proposed names, addresses, duplicate matches, and actions before `generate_io_variables(path, expected_revision, base?, slot?, dry_run?)`. The selectors are top-level and optional; omission selects all and slot alone selects it across bases. Filtering follows full native address allocation and validation. This captured workflow supports XGK global symbols with variable point allocation and supported single-slot modules. It can replace matching generated/overlapping symbols, so review the complete plan and refresh globals after applying it.

## Network, Cnet, and FEnet edits

`update_network` targets an inspected `network_index`; `update_network_module` targets a unique base/slot and changes user-facing configuration name, alias, or description. Protocol/hardware identities follow the library's restrictions. Network names and existing linked config_name/alias/description fields use `get_ir(section:"networks")` and `apply_patch`, with expected values derived from the revision-scoped export. `edit_browser_network` is now internal to that patch workflow.

`edit_cnet_settings` accepts a nested `patch` containing `base`, `slot`, and `changes:[{port_index,field,expected_value,replacement}]`. `port_index` is native zero-based port order. Collect current values from `networks`, use described field keys and enum choices, and send related changes together where the library validates their combination. Do not replace opaque port records.

FEnet edits also use `get_ir(section:"networks")` and `apply_patch`. Supported
fields are both IPv4/mask/gateway/DNS sets, DHCP flags, station number, socket
count, receive/client wait times and driver type. Missing native fields remain
readonly; protocol/module identities and opaque records cannot be changed.
The internal `edit_fenet_field` writer retains its stale-value, address, mask,
range and type-association checks. See [property-patches.md](property-patches.md)
for the complete decoded field list and value formats.

`config_name` selects the PLC configuration scope; use an inspected configuration
name rather than a display label. Changing it can detach the settings association.
FEnet records with missing or ambiguous membership/configuration associations
show readonly `association_status` and readonly settings; their decoded values
remain visible, but patches cannot promote them to editable fields.

Example metadata operation:

```json
{
  "name":"update_network_module",
  "args":{"base":0,"slot":3,"patch":{"alias":"Line 1 Ethernet"}}
}
```

The example base/slot must refer to a currently inspected supported network module. No network connection, PLC download, or online acceptance is implied by changing the project's stored settings.
