# Hardware and networks

Inspect `hardware` and `catalog.cpu`/`catalog.modules` to identify the CPU, base capacities, unique base/slot positions, module identities, slot spans, current options, and writable choices. Inspect `networks` for linked modules, native Cnet port order, FEnet values, and metadata. Use `describe_operation` for exact argument names and accepted field keys.

## CPU and module edits

`select_cpu(model)` follows the existing library's conversion behavior:

- Selecting the current CPU type is a no-op.
- XGK-to-XGK changes check retained bases/modules against the target's limits while preserving existing program/hardware/network configuration.
- Captured XGI-to-XGI changes support validated SFC projects with default parameters and empty I/O tables; these may update CPU defaults and model-specific local Ethernet/motion sections.
- Cross-family changes, arbitrary compact-model conversions, uncaptured parameters, and unsupported migrations are rejected.

Use `select_module` for catalog-backed replacement, `insert_module` for an empty physical slot, and `delete_module` for a uniquely identified module. Captured companion network records are maintained by the library. Multi-slot placement cannot overlap another module or exceed base capacity. Compact hardware identity has separate restrictions.

`set_base_slot_count` changes a supported base capacity. `set_module_option` uses the inspected option `key`, channel/group `index`, and numeric catalog choice. Read-only visible dialog fields do not become writable by supplying a guessed key or a hex blob. `set_module_input_filter` has its own supported enum. `update_module` changes supported metadata; omit fields that should remain untouched.

Example operation in `apply_edits.operations`, after inspecting this module position:

```json
{"name":"update_module","args":{"base":0,"slot":2,"patch":{"comment":"Feed conveyor"}}}
```

Module insertion, removal, replacement, and CPU changes warrant refreshed hardware/network inspection before later dependent edits.

For requested I/O symbol generation, inspect `io_variables` to review the proposed names, addresses, duplicate matches, and actions before `generate_io_variables` with `args:{}`. This captured workflow supports XGK global symbols with variable point allocation and supported single-slot modules. It can replace matching generated/overlapping symbols, so review the complete plan and refresh globals after applying it.

## Network, Cnet, and FEnet edits

`update_network` targets an inspected `network_index`; `update_network_module` targets a unique base/slot and changes user-facing configuration name, alias, or description. Protocol/hardware identities follow the library's restrictions. The stale-checked `edit_browser_network` adapter preserves its existing expected-value behavior.

`edit_cnet_settings` accepts a nested `patch` containing `base`, `slot`, and `changes:[{port_index,field,expected_value,replacement}]`. `port_index` is native zero-based port order. Collect current values from `networks`, use described field keys and enum choices, and send related changes together where the library validates their combination. Do not replace opaque port records.

`edit_fenet_field` accepts `patch` with `base`, `slot`, `field`, `expected_value`, and `replacement`. Copy the current formatted value exactly; composite IP settings, booleans, and numeric fields have field-specific validation. Use the live schema/description and current network inspection instead of inferring representation from a UI label.

Example metadata operation:

```json
{
  "name":"update_network_module",
  "args":{"base":0,"slot":3,"patch":{"alias":"Line 1 Ethernet"}}
}
```

The example base/slot must refer to a currently inspected supported network module. No network connection, PLC download, or online acceptance is implied by changing the project's stored settings.
