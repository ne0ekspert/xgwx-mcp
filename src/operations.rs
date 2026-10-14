//! Public writer discovery and strict, typed operation adapters.

use crate::operation_types::*;
use schemars::{JsonSchema, schema_for};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use xgwx::XgwxDocument;

pub struct OperationDefinition {
    pub name: &'static str,
    pub subsystem: &'static str,
    pub description: &'static str,
    pub adapter: &'static str,
    pub schema: fn() -> Value,
    pub example: fn() -> Value,
    pub apply: fn(&mut XgwxDocument, Value) -> Result<(), String>,
}

fn parse_args<T: serde::de::DeserializeOwned>(name: &str, args: Value) -> Result<T, String> {
    serde_json::from_value(args).map_err(|error| format!("invalid arguments for {name}: {error}"))
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct EditVendorIlArgs {
    patch: VendorIlPatch,
}
fn schema_edit_vendor_il() -> Value {
    serde_json::to_value(schema_for!(EditVendorIlArgs)).unwrap()
}
fn example_edit_vendor_il() -> Value {
    json!({"patch":{"program_index":0,"expected_object_id":"<inspected ObjectId>","expected_source":"","source":"LOAD M00000\nOUT M00001"}})
}
fn apply_edit_vendor_il(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: EditVendorIlArgs = parse_args("edit_vendor_il", args)?;
    {
        let patch: xgwx::VendorIlPatch = args.patch.clone().into();
        doc.program_mut()
            .get(patch.program_index)
            .and_then(|mut program| {
                if program.object_id() != patch.expected_object_id
                    || patch.expected_object_id.is_empty()
                {
                    return Err(xgwx::XgwxError::TextProgramEdit(
                        "program identity changed; reload before applying".into(),
                    ));
                }
                program.vendor_il()?.update(&xgwx::VendorIlSourcePatch {
                    expected_source: patch.expected_source,
                    source: patch.source,
                })
            })
    }
    .map_err(|e| e.to_string())
}
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct EditTextProgramArgs {
    patch: TextProgramPatch,
}
fn schema_edit_text_program() -> Value {
    serde_json::to_value(schema_for!(EditTextProgramArgs)).unwrap()
}
fn example_edit_text_program() -> Value {
    json!({"patch":{"program_index":0,"expected_object_id":"<inspected ObjectId>","expected_language":"ST","expected_source":"","source":"%MX0 := TRUE;\r\n"}})
}
fn apply_edit_text_program(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: EditTextProgramArgs = parse_args("edit_text_program", args)?;
    {
        let patch: xgwx::TextProgramPatch = args.patch.clone().into();
        doc.program_mut()
            .get(patch.program_index)
            .and_then(|mut program| {
                if program.object_id() != patch.expected_object_id
                    || patch.expected_object_id.is_empty()
                {
                    return Err(xgwx::XgwxError::TextProgramEdit(
                        "program identity changed; reload before applying".into(),
                    ));
                }
                if program.view().text()?.language != patch.expected_language {
                    return Err(xgwx::XgwxError::TextProgramEdit(
                        "program language changed; reload before applying".into(),
                    ));
                }
                program.text()?.update(&xgwx::TextSourcePatch {
                    expected_source: patch.expected_source,
                    source: patch.source,
                })
            })
    }
    .map_err(|e| e.to_string())
}
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct EditTextVariableArgs {
    expected_object_id: String,
    patch: SfcVariablePatch,
}
fn schema_edit_text_variable() -> Value {
    serde_json::to_value(schema_for!(EditTextVariableArgs)).unwrap()
}
fn example_edit_text_variable() -> Value {
    json!({"expected_object_id":"<inspected ObjectId>","patch":{"program_index":0,"expected_variables":[],"name":"Run","data_type":"BOOL","description":"Run request","remove":false}})
}
fn apply_edit_text_variable(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: EditTextVariableArgs = parse_args("edit_text_variable", args)?;
    {
        let patch: xgwx::SfcVariablePatch = args.patch.clone().into();
        doc.program_mut()
            .get(patch.program_index)
            .and_then(|mut program| {
                if program.object_id() != args.expected_object_id
                    || args.expected_object_id.is_empty()
                {
                    return Err(xgwx::XgwxError::TextProgramEdit(
                        "program identity changed; reload before applying".into(),
                    ));
                }
                program.view().text()?;
                program
                    .variable_mut()
                    .edit_declaration(&xgwx::ProgramVariablePatch {
                        expected_variables: patch.expected_variables,
                        name: patch.name,
                        data_type: patch.data_type,
                        description: patch.description,
                        remove: patch.remove,
                        declaration: patch.declaration,
                        update: patch.update,
                    })
            })
    }
    .map_err(|e| e.to_string())
}
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct EditBrowserHardwareArgs {
    patch: BrowserHardwarePatch,
}

fn schema_edit_browser_hardware() -> Value {
    serde_json::to_value(schema_for!(EditBrowserHardwareArgs)).expect("JSON Schema is serializable")
}

fn example_edit_browser_hardware() -> Value {
    json!({"patch": {"operation": "<supported field or operation from inspect_file and description>", "base": 0, "slot": null, "expected_value": "<current value from inspect_file>", "replacement": "<requested value>", "key": null, "index": null}})
}

fn apply_edit_browser_hardware(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: EditBrowserHardwareArgs = parse_args("edit_browser_hardware", args)?;
    execute_edit_browser_hardware(doc, &args)
}

fn execute_edit_browser_hardware(
    doc: &mut XgwxDocument,
    args: &EditBrowserHardwareArgs,
) -> Result<(), String> {
    (|| {
        let patch: xgwx::BrowserHardwarePatch = args.patch.clone().into();

        if patch.operation == "slotCount" {
            doc.base_mut()
                .by_number(patch.base)?
                .edit_slot_count(&patch.expected_value, &patch.replacement)
        } else {
            let slot = patch.slot.ok_or_else(|| {
                xgwx::XgwxError::BrowserHardwareEdit("missing module slot".into())
            })?;
            doc.module_mut()
                .at(patch.base, slot)?
                .edit_field(&xgwx::ModuleFieldEdit {
                    operation: patch.operation.clone(),
                    expected_value: patch.expected_value.clone(),
                    replacement: patch.replacement.clone(),
                    key: patch.key.clone(),
                    index: patch.index,
                })
        }
    })()
    .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct EditBrowserNetworkArgs {
    patch: BrowserNetworkPatch,
}

fn schema_edit_browser_network() -> Value {
    serde_json::to_value(schema_for!(EditBrowserNetworkArgs)).expect("JSON Schema is serializable")
}

fn example_edit_browser_network() -> Value {
    json!({"patch": {"network_index": 0, "module": false, "base": null, "slot": null, "field": "<supported field or operation from inspect_file and description>", "expected_value": "<current value from inspect_file>", "replacement": "<requested value>"}})
}

fn apply_edit_browser_network(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: EditBrowserNetworkArgs = parse_args("edit_browser_network", args)?;
    execute_edit_browser_network(doc, &args)
}

fn execute_edit_browser_network(
    doc: &mut XgwxDocument,
    args: &EditBrowserNetworkArgs,
) -> Result<(), String> {
    (|| {
        let patch: xgwx::BrowserNetworkPatch = args.patch.clone().into();

        let edit = xgwx::NetworkFieldEdit {
            field: patch.field.clone(),
            expected_value: patch.expected_value.clone(),
            replacement: patch.replacement.clone(),
        };
        if patch.module {
            let (Some(base), Some(slot)) = (patch.base, patch.slot) else {
                return Err(xgwx::XgwxError::BrowserNetworkEdit(
                    "missing membership coordinates".into(),
                ));
            };
            let member = doc.network().module_at(base, slot)?;
            if member.network().map(|network| network.index()) != Some(patch.network_index) {
                return Err(xgwx::XgwxError::BrowserNetworkEdit(
                    "membership belongs to another network".into(),
                ));
            }
            let index = member.index();
            doc.network_mut().module(index)?.edit_field(&edit)
        } else {
            doc.network_mut()
                .get(patch.network_index)?
                .edit_field(&edit)
        }
    })()
    .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct EditFenetFieldArgs {
    patch: FenetFieldPatch,
}

fn schema_edit_fenet_field() -> Value {
    serde_json::to_value(schema_for!(EditFenetFieldArgs)).expect("JSON Schema is serializable")
}

fn example_edit_fenet_field() -> Value {
    json!({"patch": {"base": 0, "slot": 0, "field": "<supported field or operation from inspect_file and description>", "expected_value": "<current value from inspect_file>", "replacement": "<requested value>"}})
}

fn apply_edit_fenet_field(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: EditFenetFieldArgs = parse_args("edit_fenet_field", args)?;
    execute_edit_fenet_field(doc, &args)
}

fn execute_edit_fenet_field(
    doc: &mut XgwxDocument,
    args: &EditFenetFieldArgs,
) -> Result<(), String> {
    (|| {
        let patch: xgwx::FenetFieldPatch = args.patch.clone().into();

        doc.network_mut()
            .module_at(patch.base, patch.slot)?
            .fenet()?
            .edit_field(&xgwx::NetworkFieldEdit {
                field: patch.field.clone(),
                expected_value: patch.expected_value.clone(),
                replacement: patch.replacement.clone(),
            })
    })()
    .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct SelectCpuArgs {
    model: String,
}

fn schema_select_cpu() -> Value {
    serde_json::to_value(schema_for!(SelectCpuArgs)).expect("JSON Schema is serializable")
}

fn example_select_cpu() -> Value {
    json!({"model": "<model from hardware catalog>"})
}

fn apply_select_cpu(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: SelectCpuArgs = parse_args("select_cpu", args)?;
    execute_select_cpu(doc, &args)
}

fn execute_select_cpu(doc: &mut XgwxDocument, args: &SelectCpuArgs) -> Result<(), String> {
    doc.configuration_mut()
        .get(0)
        .and_then(|mut plc| plc.select_cpu(&args.model))
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct SetBaseSlotCountArgs {
    base: u32,
    slot_count: u32,
}

fn schema_set_base_slot_count() -> Value {
    serde_json::to_value(schema_for!(SetBaseSlotCountArgs)).expect("JSON Schema is serializable")
}

fn example_set_base_slot_count() -> Value {
    json!({"base": 0, "slot_count": 8})
}

fn apply_set_base_slot_count(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: SetBaseSlotCountArgs = parse_args("set_base_slot_count", args)?;
    execute_set_base_slot_count(doc, &args)
}

fn execute_set_base_slot_count(
    doc: &mut XgwxDocument,
    args: &SetBaseSlotCountArgs,
) -> Result<(), String> {
    doc.base_mut()
        .by_number(args.base)
        .and_then(|mut selected_record| selected_record.set_slot_count(args.slot_count))
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertModuleArgs {
    base: u32,
    slot: u32,
    model: String,
}

fn schema_insert_module() -> Value {
    serde_json::to_value(schema_for!(InsertModuleArgs)).expect("JSON Schema is serializable")
}

fn example_insert_module() -> Value {
    json!({"base": 0, "slot": 0, "model": "<model from hardware catalog>"})
}

fn apply_insert_module(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: InsertModuleArgs = parse_args("insert_module", args)?;
    execute_insert_module(doc, &args)
}

fn execute_insert_module(doc: &mut XgwxDocument, args: &InsertModuleArgs) -> Result<(), String> {
    doc.base_mut()
        .by_number(args.base)
        .and_then(|mut selected_record| selected_record.insert_module(args.slot, &args.model))
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteModuleArgs {
    base: u32,
    slot: u32,
}

fn schema_delete_module() -> Value {
    serde_json::to_value(schema_for!(DeleteModuleArgs)).expect("JSON Schema is serializable")
}

fn example_delete_module() -> Value {
    json!({"base": 0, "slot": 0})
}

fn apply_delete_module(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteModuleArgs = parse_args("delete_module", args)?;
    execute_delete_module(doc, &args)
}

fn execute_delete_module(doc: &mut XgwxDocument, args: &DeleteModuleArgs) -> Result<(), String> {
    doc.module_mut()
        .at(args.base, args.slot)
        .and_then(|selected_record| selected_record.delete())
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct SelectModuleArgs {
    base: u32,
    slot: u32,
    model: String,
}

fn schema_select_module() -> Value {
    serde_json::to_value(schema_for!(SelectModuleArgs)).expect("JSON Schema is serializable")
}

fn example_select_module() -> Value {
    json!({"base": 0, "slot": 0, "model": "<model from hardware catalog>"})
}

fn apply_select_module(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: SelectModuleArgs = parse_args("select_module", args)?;
    execute_select_module(doc, &args)
}

fn execute_select_module(doc: &mut XgwxDocument, args: &SelectModuleArgs) -> Result<(), String> {
    doc.module_mut()
        .at(args.base, args.slot)
        .and_then(|mut selected_record| selected_record.select_model(&args.model))
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct SetModuleOptionArgs {
    base: u32,
    slot: u32,
    key: String,
    index: u32,
    value: u32,
}

fn schema_set_module_option() -> Value {
    serde_json::to_value(schema_for!(SetModuleOptionArgs)).expect("JSON Schema is serializable")
}

fn example_set_module_option() -> Value {
    json!({"base": 0, "slot": 0, "key": "<supported field or operation from inspect_file and description>", "index": 0, "value": 0})
}

fn apply_set_module_option(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: SetModuleOptionArgs = parse_args("set_module_option", args)?;
    execute_set_module_option(doc, &args)
}

fn execute_set_module_option(
    doc: &mut XgwxDocument,
    args: &SetModuleOptionArgs,
) -> Result<(), String> {
    doc.module_mut()
        .at(args.base, args.slot)
        .and_then(|mut selected_record| {
            selected_record.set_option(&args.key, args.index, args.value)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct UpdateModuleArgs {
    base: u32,
    slot: u32,
    patch: ModulePatch,
}

fn schema_update_module() -> Value {
    serde_json::to_value(schema_for!(UpdateModuleArgs)).expect("JSON Schema is serializable")
}

fn example_update_module() -> Value {
    json!({"base": 0, "slot": 0, "patch": {"id": null, "sub_type": null, "name": null, "comment": null, "details": null}})
}

fn apply_update_module(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: UpdateModuleArgs = parse_args("update_module", args)?;
    execute_update_module(doc, &args)
}

fn execute_update_module(doc: &mut XgwxDocument, args: &UpdateModuleArgs) -> Result<(), String> {
    doc.module_mut()
        .at(args.base, args.slot)
        .and_then(|mut selected_record| selected_record.update(&args.patch.clone().into()))
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct SetModuleInputFilterArgs {
    base: u32,
    slot: u32,
    filter: ModuleInputFilter,
}

fn schema_set_module_input_filter() -> Value {
    serde_json::to_value(schema_for!(SetModuleInputFilterArgs))
        .expect("JSON Schema is serializable")
}

fn example_set_module_input_filter() -> Value {
    json!({"base": 0, "slot": 0, "filter": "Default"})
}

fn apply_set_module_input_filter(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: SetModuleInputFilterArgs = parse_args("set_module_input_filter", args)?;
    execute_set_module_input_filter(doc, &args)
}

fn execute_set_module_input_filter(
    doc: &mut XgwxDocument,
    args: &SetModuleInputFilterArgs,
) -> Result<(), String> {
    doc.module_mut()
        .at(args.base, args.slot)
        .and_then(|mut selected_record| {
            selected_record.set_input_filter(args.filter.clone().into())
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct EditBrowserIecArgs {
    index: usize,
    patch: BrowserIecPatch,
}

fn schema_edit_browser_iec() -> Value {
    serde_json::to_value(schema_for!(EditBrowserIecArgs)).expect("JSON Schema is serializable")
}

fn example_edit_browser_iec() -> Value {
    json!({"index": 0, "patch": {"operation": "<supported field or operation from inspect_file and description>", "offset": 0, "expected_kind": "<current value from inspect_file>", "expected_value": "<current value from inspect_file>", "replacement": "<requested value>"}})
}

fn apply_edit_browser_iec(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: EditBrowserIecArgs = parse_args("edit_browser_iec", args)?;
    execute_edit_browser_iec(doc, &args)
}

fn execute_edit_browser_iec(
    doc: &mut XgwxDocument,
    args: &EditBrowserIecArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.index)
        .and_then(|mut selected_program| {
            selected_program
                .iec_ld()?
                .edit_browser(&args.patch.clone().into())
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct RenameProjectArgs {
    expected_name: String,
    name: String,
}
fn schema_rename_project() -> Value {
    serde_json::to_value(schema_for!(RenameProjectArgs)).expect("JSON Schema is serializable")
}
fn example_rename_project() -> Value {
    json!({"expected_name":"NewProject","name":"Machine Control"})
}
fn apply_rename_project(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: RenameProjectArgs = parse_args("rename_project", args)?;
    doc.project_mut()
        .rename_if(&args.expected_name, &args.name)
        .map_err(|e| e.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct UpdateProgramArgs {
    program_index: usize,
    patch: ProgramPatch,
}

fn schema_update_program() -> Value {
    serde_json::to_value(schema_for!(UpdateProgramArgs)).expect("JSON Schema is serializable")
}

fn example_update_program() -> Value {
    json!({"program_index": 0, "patch": {"name": null, "task": null, "version": null, "kind": null, "instance_name": null, "comment": null}})
}

fn apply_update_program(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: UpdateProgramArgs = parse_args("update_program", args)?;
    execute_update_program(doc, &args)
}

fn execute_update_program(doc: &mut XgwxDocument, args: &UpdateProgramArgs) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut selected_program| selected_program.update(&args.patch.clone().into()))
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct UpdateNetworkArgs {
    network_index: usize,
    patch: NetworkPatch,
}

fn schema_update_network() -> Value {
    serde_json::to_value(schema_for!(UpdateNetworkArgs)).expect("JSON Schema is serializable")
}

fn example_update_network() -> Value {
    json!({"network_index": 0, "patch": {"name": null, "type_name": null, "network_type": null}})
}

fn apply_update_network(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: UpdateNetworkArgs = parse_args("update_network", args)?;
    execute_update_network(doc, &args)
}

fn execute_update_network(doc: &mut XgwxDocument, args: &UpdateNetworkArgs) -> Result<(), String> {
    doc.network_mut()
        .get(args.network_index)
        .and_then(|mut selected_record| selected_record.update(&args.patch.clone().into()))
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct UpdateNetworkModuleArgs {
    base: u32,
    slot: u32,
    patch: NetworkModulePatch,
}

fn schema_update_network_module() -> Value {
    serde_json::to_value(schema_for!(UpdateNetworkModuleArgs)).expect("JSON Schema is serializable")
}

fn example_update_network_module() -> Value {
    json!({"base": 0, "slot": 0, "patch": {"config_name": null, "alias": null, "description": null}})
}

fn apply_update_network_module(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: UpdateNetworkModuleArgs = parse_args("update_network_module", args)?;
    execute_update_network_module(doc, &args)
}

fn execute_update_network_module(
    doc: &mut XgwxDocument,
    args: &UpdateNetworkModuleArgs,
) -> Result<(), String> {
    doc.network_mut()
        .module_at(args.base, args.slot)
        .and_then(|mut selected_record| selected_record.update(&args.patch.clone().into()))
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct GenerateIoVariablesArgs {
    base: Option<u32>,
    slot: Option<u32>,
}

fn schema_generate_io_variables() -> Value {
    serde_json::to_value(schema_for!(GenerateIoVariablesArgs)).expect("JSON Schema is serializable")
}

fn example_generate_io_variables() -> Value {
    json!({})
}

fn apply_generate_io_variables(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: GenerateIoVariablesArgs = parse_args("generate_io_variables", args)?;
    execute_generate_io_variables(doc, &args)
}

fn execute_generate_io_variables(
    doc: &mut XgwxDocument,
    args: &GenerateIoVariablesArgs,
) -> Result<(), String> {
    doc.variable_mut()
        .generate_io_variables_for(args.base, args.slot)
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct UpdateVariableArgs {
    variable_index: usize,
    patch: VariablePatch,
}

fn schema_update_variable() -> Value {
    serde_json::to_value(schema_for!(UpdateVariableArgs)).expect("JSON Schema is serializable")
}

fn example_update_variable() -> Value {
    json!({"variable_index": 0, "patch": {"name": null, "address_area": null, "address_number": null, "data_type": null, "description": null}})
}

fn apply_update_variable(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: UpdateVariableArgs = parse_args("update_variable", args)?;
    execute_update_variable(doc, &args)
}

fn execute_update_variable(
    doc: &mut XgwxDocument,
    args: &UpdateVariableArgs,
) -> Result<(), String> {
    doc.variable_mut()
        .get(args.variable_index)
        .and_then(|mut variable| variable.update(&args.patch.clone().into()))
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct UpdateIecLocalSymbolAddressArgs {
    program_index: usize,
    symbol_index: usize,
    expected_name: String,
    expected_address: String,
    replacement: String,
}

fn schema_update_iec_local_symbol_address() -> Value {
    serde_json::to_value(schema_for!(UpdateIecLocalSymbolAddressArgs))
        .expect("JSON Schema is serializable")
}

fn example_update_iec_local_symbol_address() -> Value {
    json!({"program_index": 0, "symbol_index": 0, "expected_name": "<current value from inspect_file>", "expected_address": "<current value from inspect_file>", "replacement": "<requested value>"})
}

fn apply_update_iec_local_symbol_address(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: UpdateIecLocalSymbolAddressArgs =
        parse_args("update_iec_local_symbol_address", args)?;
    execute_update_iec_local_symbol_address(doc, &args)
}

fn execute_update_iec_local_symbol_address(
    doc: &mut XgwxDocument,
    args: &UpdateIecLocalSymbolAddressArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.variable_mut().update_address(
                args.symbol_index,
                &args.expected_name,
                &args.expected_address,
                &args.replacement,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct RenameIecLocalSymbolArgs {
    program_index: usize,
    symbol_index: usize,
    expected_name: String,
    replacement: String,
}

fn schema_rename_iec_local_symbol() -> Value {
    serde_json::to_value(schema_for!(RenameIecLocalSymbolArgs))
        .expect("JSON Schema is serializable")
}

fn example_rename_iec_local_symbol() -> Value {
    json!({"program_index": 0, "symbol_index": 0, "expected_name": "<current value from inspect_file>", "replacement": "<requested value>"})
}

fn apply_rename_iec_local_symbol(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: RenameIecLocalSymbolArgs = parse_args("rename_iec_local_symbol", args)?;
    execute_rename_iec_local_symbol(doc, &args)
}

fn execute_rename_iec_local_symbol(
    doc: &mut XgwxDocument,
    args: &RenameIecLocalSymbolArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .variable_mut()
                .rename(args.symbol_index, &args.expected_name, &args.replacement)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DuplicateIecLdFunctionInstanceArgs {
    program_index: usize,
    block_offset: usize,
    expected_instance: String,
    new_instance: String,
}

fn schema_duplicate_iec_ld_function_instance() -> Value {
    serde_json::to_value(schema_for!(DuplicateIecLdFunctionInstanceArgs))
        .expect("JSON Schema is serializable")
}

fn example_duplicate_iec_ld_function_instance() -> Value {
    json!({"program_index": 0, "block_offset": 0, "expected_instance": "<current value from inspect_file>", "new_instance": "<requested value>"})
}

fn apply_duplicate_iec_ld_function_instance(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: DuplicateIecLdFunctionInstanceArgs =
        parse_args("duplicate_iec_ld_function_instance", args)?;
    execute_duplicate_iec_ld_function_instance(doc, &args)
}

fn execute_duplicate_iec_ld_function_instance(
    doc: &mut XgwxDocument,
    args: &DuplicateIecLdFunctionInstanceArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.duplicate_function_instance(
                args.block_offset,
                &args.expected_instance,
                &args.new_instance,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct CopyIecInstanceDeclarationToProgramArgs {
    source_program_index: usize,
    source_instance: String,
    destination_program_index: usize,
    new_instance: String,
}

fn schema_copy_iec_instance_declaration_to_program() -> Value {
    serde_json::to_value(schema_for!(CopyIecInstanceDeclarationToProgramArgs))
        .expect("JSON Schema is serializable")
}

fn example_copy_iec_instance_declaration_to_program() -> Value {
    json!({"source_program_index": 0, "source_instance": "<requested value>", "destination_program_index": 0, "new_instance": "<requested value>"})
}

fn apply_copy_iec_instance_declaration_to_program(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: CopyIecInstanceDeclarationToProgramArgs =
        parse_args("copy_iec_instance_declaration_to_program", args)?;
    execute_copy_iec_instance_declaration_to_program(doc, &args)
}

fn execute_copy_iec_instance_declaration_to_program(
    doc: &mut XgwxDocument,
    args: &CopyIecInstanceDeclarationToProgramArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.source_program_index)
        .and_then(|mut program| {
            program.variable_mut().copy_instance_to_program(
                &args.source_instance,
                args.destination_program_index,
                &args.new_instance,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct UpdateIecLocalSymbolDescriptionArgs {
    program_index: usize,
    symbol_index: usize,
    expected_name: String,
    expected_description: String,
    replacement: String,
}

fn schema_update_iec_local_symbol_description() -> Value {
    serde_json::to_value(schema_for!(UpdateIecLocalSymbolDescriptionArgs))
        .expect("JSON Schema is serializable")
}

fn example_update_iec_local_symbol_description() -> Value {
    json!({"program_index": 0, "symbol_index": 0, "expected_name": "<current value from inspect_file>", "expected_description": "<current value from inspect_file>", "replacement": "<requested value>"})
}

fn apply_update_iec_local_symbol_description(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: UpdateIecLocalSymbolDescriptionArgs =
        parse_args("update_iec_local_symbol_description", args)?;
    execute_update_iec_local_symbol_description(doc, &args)
}

fn execute_update_iec_local_symbol_description(
    doc: &mut XgwxDocument,
    args: &UpdateIecLocalSymbolDescriptionArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.variable_mut().update_description(
                args.symbol_index,
                &args.expected_name,
                &args.expected_description,
                &args.replacement,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertIecLocalSymbolArgs {
    program_index: usize,
    name: String,
    data_type: String,
    description: String,
}

fn schema_insert_iec_local_symbol() -> Value {
    serde_json::to_value(schema_for!(InsertIecLocalSymbolArgs))
        .expect("JSON Schema is serializable")
}

fn example_insert_iec_local_symbol() -> Value {
    json!({"program_index": 0, "name": "<requested value>", "data_type": "BOOL", "description": "<requested value>"})
}

fn apply_insert_iec_local_symbol(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: InsertIecLocalSymbolArgs = parse_args("insert_iec_local_symbol", args)?;
    execute_insert_iec_local_symbol(doc, &args)
}

fn execute_insert_iec_local_symbol(
    doc: &mut XgwxDocument,
    args: &InsertIecLocalSymbolArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .variable_mut()
                .insert(&args.name, &args.data_type, &args.description)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLocalSymbolArgs {
    program_index: usize,
    symbol_index: usize,
    expected_name: String,
}

fn schema_delete_iec_local_symbol() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLocalSymbolArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_local_symbol() -> Value {
    json!({"program_index": 0, "symbol_index": 0, "expected_name": "<current value from inspect_file>"})
}

fn apply_delete_iec_local_symbol(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteIecLocalSymbolArgs = parse_args("delete_iec_local_symbol", args)?;
    execute_delete_iec_local_symbol(doc, &args)
}

fn execute_delete_iec_local_symbol(
    doc: &mut XgwxDocument,
    args: &DeleteIecLocalSymbolArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .variable_mut()
                .delete(args.symbol_index, &args.expected_name)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct UpdateIecLocalSymbolTypeArgs {
    program_index: usize,
    symbol_index: usize,
    expected_name: String,
    expected_type: String,
    replacement: String,
}

fn schema_update_iec_local_symbol_type() -> Value {
    serde_json::to_value(schema_for!(UpdateIecLocalSymbolTypeArgs))
        .expect("JSON Schema is serializable")
}

fn example_update_iec_local_symbol_type() -> Value {
    json!({"program_index": 0, "symbol_index": 0, "expected_name": "<current value from inspect_file>", "expected_type": "<current value from inspect_file>", "replacement": "<requested value>"})
}

fn apply_update_iec_local_symbol_type(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: UpdateIecLocalSymbolTypeArgs = parse_args("update_iec_local_symbol_type", args)?;
    execute_update_iec_local_symbol_type(doc, &args)
}

fn execute_update_iec_local_symbol_type(
    doc: &mut XgwxDocument,
    args: &UpdateIecLocalSymbolTypeArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.variable_mut().update_type(
                args.symbol_index,
                &args.expected_name,
                &args.expected_type,
                &args.replacement,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct UpdateLadderCellTextArgs {
    program_index: usize,
    offset: usize,
    expected: String,
    replacement: String,
}

fn schema_update_ladder_cell_text() -> Value {
    serde_json::to_value(schema_for!(UpdateLadderCellTextArgs))
        .expect("JSON Schema is serializable")
}

fn example_update_ladder_cell_text() -> Value {
    json!({"program_index": 0, "offset": 0, "expected": "<current value from inspect_file>", "replacement": "<requested value>"})
}

fn apply_update_ladder_cell_text(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: UpdateLadderCellTextArgs = parse_args("update_ladder_cell_text", args)?;
    execute_update_ladder_cell_text(doc, &args)
}

fn execute_update_ladder_cell_text(
    doc: &mut XgwxDocument,
    args: &UpdateLadderCellTextArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            if program.view().ladder()?.project_type == Some(1) {
                program
                    .ladder()?
                    .update_cell_text(args.offset, &args.expected, &args.replacement)
            } else {
                program
                    .iec_ld()?
                    .update_cell_text(args.offset, &args.expected, &args.replacement)
            }
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct UpdateIecLdCommentArgs {
    program_index: usize,
    offset: usize,
    expected: String,
    replacement: String,
}

fn schema_update_iec_ld_comment() -> Value {
    serde_json::to_value(schema_for!(UpdateIecLdCommentArgs)).expect("JSON Schema is serializable")
}

fn example_update_iec_ld_comment() -> Value {
    json!({"program_index": 0, "offset": 0, "expected": "<current value from inspect_file>", "replacement": "<requested value>"})
}

fn apply_update_iec_ld_comment(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: UpdateIecLdCommentArgs = parse_args("update_iec_ld_comment", args)?;
    execute_update_iec_ld_comment(doc, &args)
}

fn execute_update_iec_ld_comment(
    doc: &mut XgwxDocument,
    args: &UpdateIecLdCommentArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .update_comment(args.offset, &args.expected, &args.replacement)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct UpdateIecLdRisingContactOperandArgs {
    program_index: usize,
    offset: usize,
    expected: String,
    replacement: String,
}

fn schema_update_iec_ld_rising_contact_operand() -> Value {
    serde_json::to_value(schema_for!(UpdateIecLdRisingContactOperandArgs))
        .expect("JSON Schema is serializable")
}

fn example_update_iec_ld_rising_contact_operand() -> Value {
    json!({"program_index": 0, "offset": 0, "expected": "<current value from inspect_file>", "replacement": "<requested value>"})
}

fn apply_update_iec_ld_rising_contact_operand(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: UpdateIecLdRisingContactOperandArgs =
        parse_args("update_iec_ld_rising_contact_operand", args)?;
    execute_update_iec_ld_rising_contact_operand(doc, &args)
}

fn execute_update_iec_ld_rising_contact_operand(
    doc: &mut XgwxDocument,
    args: &UpdateIecLdRisingContactOperandArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.update_rising_contact_operand(
                args.offset,
                &args.expected,
                &args.replacement,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct UpdateIecLdElementOperandArgs {
    program_index: usize,
    offset: usize,
    expected: String,
    replacement: String,
}

fn schema_update_iec_ld_element_operand() -> Value {
    serde_json::to_value(schema_for!(UpdateIecLdElementOperandArgs))
        .expect("JSON Schema is serializable")
}

fn example_update_iec_ld_element_operand() -> Value {
    json!({"program_index": 0, "offset": 0, "expected": "<current value from inspect_file>", "replacement": "<requested value>"})
}

fn apply_update_iec_ld_element_operand(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: UpdateIecLdElementOperandArgs = parse_args("update_iec_ld_element_operand", args)?;
    execute_update_iec_ld_element_operand(doc, &args)
}

fn execute_update_iec_ld_element_operand(
    doc: &mut XgwxDocument,
    args: &UpdateIecLdElementOperandArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .update_element_operand(args.offset, &args.expected, &args.replacement)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct UpdateIecLdContactKindArgs {
    program_index: usize,
    offset: usize,
    expected: String,
    replacement: String,
}

fn schema_update_iec_ld_contact_kind() -> Value {
    serde_json::to_value(schema_for!(UpdateIecLdContactKindArgs))
        .expect("JSON Schema is serializable")
}

fn example_update_iec_ld_contact_kind() -> Value {
    json!({"program_index": 0, "offset": 0, "expected": "<current value from inspect_file>", "replacement": "<requested value>"})
}

fn apply_update_iec_ld_contact_kind(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: UpdateIecLdContactKindArgs = parse_args("update_iec_ld_contact_kind", args)?;
    execute_update_iec_ld_contact_kind(doc, &args)
}

fn execute_update_iec_ld_contact_kind(
    doc: &mut XgwxDocument,
    args: &UpdateIecLdContactKindArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .update_contact_kind(args.offset, &args.expected, &args.replacement)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct UpdateIecLdCoilKindArgs {
    program_index: usize,
    offset: usize,
    expected: String,
    replacement: String,
}

fn schema_update_iec_ld_coil_kind() -> Value {
    serde_json::to_value(schema_for!(UpdateIecLdCoilKindArgs)).expect("JSON Schema is serializable")
}

fn example_update_iec_ld_coil_kind() -> Value {
    json!({"program_index": 0, "offset": 0, "expected": "<current value from inspect_file>", "replacement": "<requested value>"})
}

fn apply_update_iec_ld_coil_kind(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: UpdateIecLdCoilKindArgs = parse_args("update_iec_ld_coil_kind", args)?;
    execute_update_iec_ld_coil_kind(doc, &args)
}

fn execute_update_iec_ld_coil_kind(
    doc: &mut XgwxDocument,
    args: &UpdateIecLdCoilKindArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .update_coil_kind(args.offset, &args.expected, &args.replacement)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct EditIecLdBranchSegmentArgs {
    program_index: usize,
    group_index: usize,
    start_row_index: u16,
    end_row_index: u16,
    x: u8,
    expected: bool,
    present: bool,
}

fn schema_edit_iec_ld_branch_segment() -> Value {
    serde_json::to_value(schema_for!(EditIecLdBranchSegmentArgs))
        .expect("JSON Schema is serializable")
}

fn example_edit_iec_ld_branch_segment() -> Value {
    json!({"program_index": 0, "group_index": 0, "start_row_index": 0, "end_row_index": 0, "x": 0, "expected": false, "present": false})
}

fn apply_edit_iec_ld_branch_segment(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: EditIecLdBranchSegmentArgs = parse_args("edit_iec_ld_branch_segment", args)?;
    execute_edit_iec_ld_branch_segment(doc, &args)
}

fn execute_edit_iec_ld_branch_segment(
    doc: &mut XgwxDocument,
    args: &EditIecLdBranchSegmentArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.edit_branch_segment(
                args.group_index,
                args.start_row_index,
                args.end_row_index,
                args.x,
                args.expected,
                args.present,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct EditIecLdVerticalWireArgs {
    program_index: usize,
    group_index: usize,
    start_row_index: u16,
    end_row_index: u16,
    x: u8,
    expected: bool,
    present: bool,
}

fn schema_edit_iec_ld_vertical_wire() -> Value {
    serde_json::to_value(schema_for!(EditIecLdVerticalWireArgs))
        .expect("JSON Schema is serializable")
}

fn example_edit_iec_ld_vertical_wire() -> Value {
    json!({"program_index": 0, "group_index": 0, "start_row_index": 0, "end_row_index": 0, "x": 0, "expected": false, "present": false})
}

fn apply_edit_iec_ld_vertical_wire(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: EditIecLdVerticalWireArgs = parse_args("edit_iec_ld_vertical_wire", args)?;
    execute_edit_iec_ld_vertical_wire(doc, &args)
}

fn execute_edit_iec_ld_vertical_wire(
    doc: &mut XgwxDocument,
    args: &EditIecLdVerticalWireArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.edit_vertical_wire(
                args.group_index,
                args.start_row_index,
                args.end_row_index,
                args.x,
                args.expected,
                args.present,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ConnectIecLdGroupsArgs {
    program_index: usize,
    upper_group_index: usize,
    expected_upper_row: u16,
    lower_group_index: usize,
    expected_lower_row: u16,
    x: u8,
}

fn schema_connect_iec_ld_groups() -> Value {
    serde_json::to_value(schema_for!(ConnectIecLdGroupsArgs)).expect("JSON Schema is serializable")
}

fn example_connect_iec_ld_groups() -> Value {
    json!({"program_index": 0, "upper_group_index": 0, "expected_upper_row": 0, "lower_group_index": 0, "expected_lower_row": 0, "x": 0})
}

fn apply_connect_iec_ld_groups(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: ConnectIecLdGroupsArgs = parse_args("connect_iec_ld_groups", args)?;
    execute_connect_iec_ld_groups(doc, &args)
}

fn execute_connect_iec_ld_groups(
    doc: &mut XgwxDocument,
    args: &ConnectIecLdGroupsArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.connect_groups(
                args.upper_group_index,
                args.expected_upper_row,
                args.lower_group_index,
                args.expected_lower_row,
                args.x,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ExtendIecLdVerticalWireArgs {
    program_index: usize,
    group_index: usize,
    start_row_index: u16,
    x: u8,
}

fn schema_extend_iec_ld_vertical_wire() -> Value {
    serde_json::to_value(schema_for!(ExtendIecLdVerticalWireArgs))
        .expect("JSON Schema is serializable")
}

fn example_extend_iec_ld_vertical_wire() -> Value {
    json!({"program_index": 0, "group_index": 0, "start_row_index": 0, "x": 0})
}

fn apply_extend_iec_ld_vertical_wire(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: ExtendIecLdVerticalWireArgs = parse_args("extend_iec_ld_vertical_wire", args)?;
    execute_extend_iec_ld_vertical_wire(doc, &args)
}

fn execute_extend_iec_ld_vertical_wire(
    doc: &mut XgwxDocument,
    args: &ExtendIecLdVerticalWireArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .extend_vertical_wire(args.group_index, args.start_row_index, args.x)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct SplitIecLdGroupArgs {
    program_index: usize,
    group_index: usize,
    expected_upper_row: u16,
    expected_lower_row: u16,
}

fn schema_split_iec_ld_group() -> Value {
    serde_json::to_value(schema_for!(SplitIecLdGroupArgs)).expect("JSON Schema is serializable")
}

fn example_split_iec_ld_group() -> Value {
    json!({"program_index": 0, "group_index": 0, "expected_upper_row": 0, "expected_lower_row": 0})
}

fn apply_split_iec_ld_group(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: SplitIecLdGroupArgs = parse_args("split_iec_ld_group", args)?;
    execute_split_iec_ld_group(doc, &args)
}

fn execute_split_iec_ld_group(
    doc: &mut XgwxDocument,
    args: &SplitIecLdGroupArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.split_group(
                args.group_index,
                args.expected_upper_row,
                args.expected_lower_row,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertIecLdBlankRowArgs {
    program_index: usize,
    after_row_index: u16,
}

fn schema_insert_iec_ld_blank_row() -> Value {
    serde_json::to_value(schema_for!(InsertIecLdBlankRowArgs)).expect("JSON Schema is serializable")
}

fn example_insert_iec_ld_blank_row() -> Value {
    json!({"program_index": 0, "after_row_index": 0})
}

fn apply_insert_iec_ld_blank_row(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: InsertIecLdBlankRowArgs = parse_args("insert_iec_ld_blank_row", args)?;
    execute_insert_iec_ld_blank_row(doc, &args)
}

fn execute_insert_iec_ld_blank_row(
    doc: &mut XgwxDocument,
    args: &InsertIecLdBlankRowArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut selected_program| {
            selected_program
                .iec_ld()?
                .insert_blank_row(args.after_row_index)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertIecLdParallelContactArgs {
    program_index: usize,
    row_index: u16,
    expected_contact_variable: String,
    expected_coil_variable: String,
    parallel_variable: String,
}

fn schema_insert_iec_ld_parallel_contact() -> Value {
    serde_json::to_value(schema_for!(InsertIecLdParallelContactArgs))
        .expect("JSON Schema is serializable")
}

fn example_insert_iec_ld_parallel_contact() -> Value {
    json!({"program_index": 0, "row_index": 0, "expected_contact_variable": "<current value from inspect_file>", "expected_coil_variable": "<current value from inspect_file>", "parallel_variable": "<requested value>"})
}

fn apply_insert_iec_ld_parallel_contact(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: InsertIecLdParallelContactArgs = parse_args("insert_iec_ld_parallel_contact", args)?;
    execute_insert_iec_ld_parallel_contact(doc, &args)
}

fn execute_insert_iec_ld_parallel_contact(
    doc: &mut XgwxDocument,
    args: &InsertIecLdParallelContactArgs,
) -> Result<(), String> {
    execute_insert_iec_ld_parallel_contact_kind(
        doc,
        &InsertIecLdParallelContactKindArgs {
            program_index: args.program_index,
            row_index: args.row_index,
            expected_contact_variable: args.expected_contact_variable.clone(),
            expected_coil_variable: args.expected_coil_variable.clone(),
            parallel_kind: "NO".into(),
            parallel_variable: args.parallel_variable.clone(),
        },
    )
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertIecLdParallelContactKindArgs {
    program_index: usize,
    row_index: u16,
    expected_contact_variable: String,
    expected_coil_variable: String,
    parallel_kind: String,
    parallel_variable: String,
}

fn schema_insert_iec_ld_parallel_contact_kind() -> Value {
    serde_json::to_value(schema_for!(InsertIecLdParallelContactKindArgs))
        .expect("JSON Schema is serializable")
}

fn example_insert_iec_ld_parallel_contact_kind() -> Value {
    json!({"program_index": 0, "row_index": 0, "expected_contact_variable": "<current value from inspect_file>", "expected_coil_variable": "<current value from inspect_file>", "parallel_kind": "NO", "parallel_variable": "<requested value>"})
}

fn apply_insert_iec_ld_parallel_contact_kind(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: InsertIecLdParallelContactKindArgs =
        parse_args("insert_iec_ld_parallel_contact_kind", args)?;
    execute_insert_iec_ld_parallel_contact_kind(doc, &args)
}

fn execute_insert_iec_ld_parallel_contact_kind(
    doc: &mut XgwxDocument,
    args: &InsertIecLdParallelContactKindArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.insert_parallel_contact_kind(
                args.row_index,
                &args.expected_contact_variable,
                &args.expected_coil_variable,
                &args.parallel_kind,
                &args.parallel_variable,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdBlankRowArgs {
    program_index: usize,
    blank_row_index: u16,
}

fn schema_delete_iec_ld_blank_row() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdBlankRowArgs)).expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_blank_row() -> Value {
    json!({"program_index": 0, "blank_row_index": 0})
}

fn apply_delete_iec_ld_blank_row(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteIecLdBlankRowArgs = parse_args("delete_iec_ld_blank_row", args)?;
    execute_delete_iec_ld_blank_row(doc, &args)
}

fn execute_delete_iec_ld_blank_row(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdBlankRowArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut selected_program| {
            selected_program
                .iec_ld()?
                .delete_blank_row(args.blank_row_index)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertIecLdLinearRungArgs {
    program_index: usize,
    blank_row_index: u16,
    contact_variable: String,
    coil_variable: String,
}

fn schema_insert_iec_ld_linear_rung() -> Value {
    serde_json::to_value(schema_for!(InsertIecLdLinearRungArgs))
        .expect("JSON Schema is serializable")
}

fn example_insert_iec_ld_linear_rung() -> Value {
    json!({"program_index": 0, "blank_row_index": 0, "contact_variable": "<requested value>", "coil_variable": "<requested value>"})
}

fn apply_insert_iec_ld_linear_rung(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: InsertIecLdLinearRungArgs = parse_args("insert_iec_ld_linear_rung", args)?;
    execute_insert_iec_ld_linear_rung(doc, &args)
}

fn execute_insert_iec_ld_linear_rung(
    doc: &mut XgwxDocument,
    args: &InsertIecLdLinearRungArgs,
) -> Result<(), String> {
    execute_insert_iec_ld_rung(
        doc,
        &InsertIecLdRungArgs {
            program_index: args.program_index,
            blank_row_index: args.blank_row_index,
            contact_kind: "NO".into(),
            contact_variable: args.contact_variable.clone(),
            coil_kind: "OUTPUT".into(),
            coil_variable: args.coil_variable.clone(),
        },
    )
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertIecLdRungArgs {
    program_index: usize,
    blank_row_index: u16,
    contact_kind: String,
    contact_variable: String,
    coil_kind: String,
    coil_variable: String,
}

fn schema_insert_iec_ld_rung() -> Value {
    serde_json::to_value(schema_for!(InsertIecLdRungArgs)).expect("JSON Schema is serializable")
}

fn example_insert_iec_ld_rung() -> Value {
    json!({"program_index": 0, "blank_row_index": 0, "contact_kind": "NO", "contact_variable": "<requested value>", "coil_kind": "OUTPUT", "coil_variable": "<requested value>"})
}

fn apply_insert_iec_ld_rung(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: InsertIecLdRungArgs = parse_args("insert_iec_ld_rung", args)?;
    execute_insert_iec_ld_rung(doc, &args)
}

fn execute_insert_iec_ld_rung(
    doc: &mut XgwxDocument,
    args: &InsertIecLdRungArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.insert_rung(
                args.blank_row_index,
                &args.contact_kind,
                &args.contact_variable,
                &args.coil_kind,
                &args.coil_variable,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertIecLdSingleElementArgs {
    program_index: usize,
    row_index: u16,
    raw_x: u8,
    category: String,
    kind: String,
    operand: String,
}

fn schema_insert_iec_ld_single_element() -> Value {
    serde_json::to_value(schema_for!(InsertIecLdSingleElementArgs))
        .expect("JSON Schema is serializable")
}

fn example_insert_iec_ld_single_element() -> Value {
    json!({"program_index": 0, "row_index": 0, "raw_x": 0, "category": "<requested value>", "kind": "<requested value>", "operand": "<requested value>"})
}

fn apply_insert_iec_ld_single_element(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: InsertIecLdSingleElementArgs = parse_args("insert_iec_ld_single_element", args)?;
    execute_insert_iec_ld_single_element(doc, &args)
}

fn execute_insert_iec_ld_single_element(
    doc: &mut XgwxDocument,
    args: &InsertIecLdSingleElementArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.insert_single_element(
                args.row_index,
                args.raw_x,
                &args.category,
                &args.kind,
                &args.operand,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdLinearRungArgs {
    program_index: usize,
    row_index: u16,
    expected_contact_variable: String,
    expected_coil_variable: String,
}

fn schema_delete_iec_ld_linear_rung() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdLinearRungArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_linear_rung() -> Value {
    json!({"program_index": 0, "row_index": 0, "expected_contact_variable": "<current value from inspect_file>", "expected_coil_variable": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_linear_rung(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteIecLdLinearRungArgs = parse_args("delete_iec_ld_linear_rung", args)?;
    execute_delete_iec_ld_linear_rung(doc, &args)
}

fn execute_delete_iec_ld_linear_rung(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdLinearRungArgs,
) -> Result<(), String> {
    execute_delete_iec_ld_rung(
        doc,
        &DeleteIecLdRungArgs {
            program_index: args.program_index,
            row_index: args.row_index,
            expected_contact_kind: "NO".into(),
            expected_contact_variable: args.expected_contact_variable.clone(),
            expected_coil_kind: "OUTPUT".into(),
            expected_coil_variable: args.expected_coil_variable.clone(),
        },
    )
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdRungArgs {
    program_index: usize,
    row_index: u16,
    expected_contact_kind: String,
    expected_contact_variable: String,
    expected_coil_kind: String,
    expected_coil_variable: String,
}

fn schema_delete_iec_ld_rung() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdRungArgs)).expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_rung() -> Value {
    json!({"program_index": 0, "row_index": 0, "expected_contact_kind": "<current value from inspect_file>", "expected_contact_variable": "<current value from inspect_file>", "expected_coil_kind": "<current value from inspect_file>", "expected_coil_variable": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_rung(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteIecLdRungArgs = parse_args("delete_iec_ld_rung", args)?;
    execute_delete_iec_ld_rung(doc, &args)
}

fn execute_delete_iec_ld_rung(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdRungArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.delete_rung(
                args.row_index,
                &args.expected_contact_kind,
                &args.expected_contact_variable,
                &args.expected_coil_kind,
                &args.expected_coil_variable,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdTerminalCoilArgs {
    program_index: usize,
    coil_record_offset: usize,
    expected_variable: String,
}

fn schema_delete_iec_ld_terminal_coil() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdTerminalCoilArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_terminal_coil() -> Value {
    json!({"program_index": 0, "coil_record_offset": 0, "expected_variable": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_terminal_coil(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteIecLdTerminalCoilArgs = parse_args("delete_iec_ld_terminal_coil", args)?;
    execute_delete_iec_ld_terminal_coil(doc, &args)
}

fn execute_delete_iec_ld_terminal_coil(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdTerminalCoilArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_terminal_coil(args.coil_record_offset, &args.expected_variable)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertIecLdTerminalCoilArgs {
    program_index: usize,
    contact_record_offset: usize,
    expected_contact_variable: String,
    coil_kind: String,
    coil_variable: String,
}

fn schema_insert_iec_ld_terminal_coil() -> Value {
    serde_json::to_value(schema_for!(InsertIecLdTerminalCoilArgs))
        .expect("JSON Schema is serializable")
}

fn example_insert_iec_ld_terminal_coil() -> Value {
    json!({"program_index": 0, "contact_record_offset": 0, "expected_contact_variable": "<current value from inspect_file>", "coil_kind": "OUTPUT", "coil_variable": "<requested value>"})
}

fn apply_insert_iec_ld_terminal_coil(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: InsertIecLdTerminalCoilArgs = parse_args("insert_iec_ld_terminal_coil", args)?;
    execute_insert_iec_ld_terminal_coil(doc, &args)
}

fn execute_insert_iec_ld_terminal_coil(
    doc: &mut XgwxDocument,
    args: &InsertIecLdTerminalCoilArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.insert_terminal_coil(
                args.contact_record_offset,
                &args.expected_contact_variable,
                &args.coil_kind,
                &args.coil_variable,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdSimpleRowArgs {
    program_index: usize,
    row_index: u16,
    expected_contact_kind: String,
    expected_contact_variable: String,
    expected_coil_kind: String,
    expected_coil_variable: String,
}

fn schema_delete_iec_ld_simple_row() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdSimpleRowArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_simple_row() -> Value {
    json!({"program_index": 0, "row_index": 0, "expected_contact_kind": "<current value from inspect_file>", "expected_contact_variable": "<current value from inspect_file>", "expected_coil_kind": "<current value from inspect_file>", "expected_coil_variable": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_simple_row(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteIecLdSimpleRowArgs = parse_args("delete_iec_ld_simple_row", args)?;
    execute_delete_iec_ld_simple_row(doc, &args)
}

fn execute_delete_iec_ld_simple_row(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdSimpleRowArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.delete_simple_row(
                args.row_index,
                &args.expected_contact_kind,
                &args.expected_contact_variable,
                &args.expected_coil_kind,
                &args.expected_coil_variable,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdBranchTopRowArgs {
    program_index: usize,
    group_index: usize,
    row_index: u16,
}

fn schema_delete_iec_ld_branch_top_row() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdBranchTopRowArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_branch_top_row() -> Value {
    json!({"program_index": 0, "group_index": 0, "row_index": 0})
}

fn apply_delete_iec_ld_branch_top_row(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteIecLdBranchTopRowArgs = parse_args("delete_iec_ld_branch_top_row", args)?;
    execute_delete_iec_ld_branch_top_row(doc, &args)
}

fn execute_delete_iec_ld_branch_top_row(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdBranchTopRowArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_branch_top_row(args.group_index, args.row_index)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdNestedContactBranchRowArgs {
    program_index: usize,
    group_index: usize,
    row_index: u16,
}

fn schema_delete_iec_ld_nested_contact_branch_row() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdNestedContactBranchRowArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_nested_contact_branch_row() -> Value {
    json!({"program_index": 0, "group_index": 0, "row_index": 0})
}

fn apply_delete_iec_ld_nested_contact_branch_row(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: DeleteIecLdNestedContactBranchRowArgs =
        parse_args("delete_iec_ld_nested_contact_branch_row", args)?;
    execute_delete_iec_ld_nested_contact_branch_row(doc, &args)
}

fn execute_delete_iec_ld_nested_contact_branch_row(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdNestedContactBranchRowArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_nested_contact_branch_row(args.group_index, args.row_index)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdChainedContactBranchRowArgs {
    program_index: usize,
    group_index: usize,
    row_index: u16,
}

fn schema_delete_iec_ld_chained_contact_branch_row() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdChainedContactBranchRowArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_chained_contact_branch_row() -> Value {
    json!({"program_index": 0, "group_index": 0, "row_index": 0})
}

fn apply_delete_iec_ld_chained_contact_branch_row(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: DeleteIecLdChainedContactBranchRowArgs =
        parse_args("delete_iec_ld_chained_contact_branch_row", args)?;
    execute_delete_iec_ld_chained_contact_branch_row(doc, &args)
}

fn execute_delete_iec_ld_chained_contact_branch_row(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdChainedContactBranchRowArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_chained_contact_branch_row(args.group_index, args.row_index)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdEmptyBranchRowArgs {
    program_index: usize,
    group_index: usize,
    row_index: u16,
}

fn schema_delete_iec_ld_empty_branch_row() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdEmptyBranchRowArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_empty_branch_row() -> Value {
    json!({"program_index": 0, "group_index": 0, "row_index": 0})
}

fn apply_delete_iec_ld_empty_branch_row(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteIecLdEmptyBranchRowArgs = parse_args("delete_iec_ld_empty_branch_row", args)?;
    execute_delete_iec_ld_empty_branch_row(doc, &args)
}

fn execute_delete_iec_ld_empty_branch_row(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdEmptyBranchRowArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_empty_branch_row(args.group_index, args.row_index)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdFfBranchOutputRowArgs {
    program_index: usize,
    group_index: usize,
    row_index: u16,
}

fn schema_delete_iec_ld_ff_branch_output_row() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdFfBranchOutputRowArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_ff_branch_output_row() -> Value {
    json!({"program_index": 0, "group_index": 0, "row_index": 0})
}

fn apply_delete_iec_ld_ff_branch_output_row(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: DeleteIecLdFfBranchOutputRowArgs =
        parse_args("delete_iec_ld_ff_branch_output_row", args)?;
    execute_delete_iec_ld_ff_branch_output_row(doc, &args)
}

fn execute_delete_iec_ld_ff_branch_output_row(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdFfBranchOutputRowArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_ff_branch_output_row(args.group_index, args.row_index)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertIecLdNoContactArgs {
    program_index: usize,
    wire_offset: usize,
    raw_x: u8,
    expected_wire_start_x: u8,
    expected_wire_end_x: u8,
    variable: String,
}

fn schema_insert_iec_ld_no_contact() -> Value {
    serde_json::to_value(schema_for!(InsertIecLdNoContactArgs))
        .expect("JSON Schema is serializable")
}

fn example_insert_iec_ld_no_contact() -> Value {
    json!({"program_index": 0, "wire_offset": 0, "raw_x": 0, "expected_wire_start_x": 0, "expected_wire_end_x": 0, "variable": "<requested value>"})
}

fn apply_insert_iec_ld_no_contact(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: InsertIecLdNoContactArgs = parse_args("insert_iec_ld_no_contact", args)?;
    execute_insert_iec_ld_no_contact(doc, &args)
}

fn execute_insert_iec_ld_no_contact(
    doc: &mut XgwxDocument,
    args: &InsertIecLdNoContactArgs,
) -> Result<(), String> {
    execute_insert_iec_ld_contact(
        doc,
        &InsertIecLdContactArgs {
            program_index: args.program_index,
            wire_offset: args.wire_offset,
            raw_x: args.raw_x,
            expected_wire_start_x: args.expected_wire_start_x,
            expected_wire_end_x: args.expected_wire_end_x,
            contact_kind: "NO".into(),
            variable: args.variable.clone(),
        },
    )
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertIecLdLeadingContactArgs {
    program_index: usize,
    insertion_offset: usize,
    contact_kind: String,
    variable: String,
}

fn schema_insert_iec_ld_leading_contact() -> Value {
    serde_json::to_value(schema_for!(InsertIecLdLeadingContactArgs))
        .expect("JSON Schema is serializable")
}

fn example_insert_iec_ld_leading_contact() -> Value {
    json!({"program_index": 0, "insertion_offset": 0, "contact_kind": "NO", "variable": "<requested value>"})
}

fn apply_insert_iec_ld_leading_contact(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: InsertIecLdLeadingContactArgs = parse_args("insert_iec_ld_leading_contact", args)?;
    execute_insert_iec_ld_leading_contact(doc, &args)
}

fn execute_insert_iec_ld_leading_contact(
    doc: &mut XgwxDocument,
    args: &InsertIecLdLeadingContactArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.insert_leading_contact(
                args.insertion_offset,
                &args.contact_kind,
                &args.variable,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertIecLdContactArgs {
    program_index: usize,
    wire_offset: usize,
    raw_x: u8,
    expected_wire_start_x: u8,
    expected_wire_end_x: u8,
    contact_kind: String,
    variable: String,
}

fn schema_insert_iec_ld_contact() -> Value {
    serde_json::to_value(schema_for!(InsertIecLdContactArgs)).expect("JSON Schema is serializable")
}

fn example_insert_iec_ld_contact() -> Value {
    json!({"program_index": 0, "wire_offset": 0, "raw_x": 0, "expected_wire_start_x": 0, "expected_wire_end_x": 0, "contact_kind": "NO", "variable": "<requested value>"})
}

fn apply_insert_iec_ld_contact(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: InsertIecLdContactArgs = parse_args("insert_iec_ld_contact", args)?;
    execute_insert_iec_ld_contact(doc, &args)
}

fn execute_insert_iec_ld_contact(
    doc: &mut XgwxDocument,
    args: &InsertIecLdContactArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.insert_contact(
                args.wire_offset,
                args.raw_x,
                args.expected_wire_start_x,
                args.expected_wire_end_x,
                &args.contact_kind,
                &args.variable,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertIecLdShortWireContactArgs {
    program_index: usize,
    wire_offset: usize,
    expected_raw_x: u8,
    contact_kind: String,
    variable: String,
}

fn schema_insert_iec_ld_short_wire_contact() -> Value {
    serde_json::to_value(schema_for!(InsertIecLdShortWireContactArgs))
        .expect("JSON Schema is serializable")
}

fn example_insert_iec_ld_short_wire_contact() -> Value {
    json!({"program_index": 0, "wire_offset": 0, "expected_raw_x": 0, "contact_kind": "NO", "variable": "<requested value>"})
}

fn apply_insert_iec_ld_short_wire_contact(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: InsertIecLdShortWireContactArgs =
        parse_args("insert_iec_ld_short_wire_contact", args)?;
    execute_insert_iec_ld_short_wire_contact(doc, &args)
}

fn execute_insert_iec_ld_short_wire_contact(
    doc: &mut XgwxDocument,
    args: &InsertIecLdShortWireContactArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.insert_short_wire_contact(
                args.wire_offset,
                args.expected_raw_x,
                &args.contact_kind,
                &args.variable,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdContactArgs {
    program_index: usize,
    contact_offset: usize,
    expected_raw_x: u8,
    expected_contact_kind: String,
    expected_variable: String,
}

fn schema_delete_iec_ld_contact() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdContactArgs)).expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_contact() -> Value {
    json!({"program_index": 0, "contact_offset": 0, "expected_raw_x": 0, "expected_contact_kind": "<current value from inspect_file>", "expected_variable": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_contact(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteIecLdContactArgs = parse_args("delete_iec_ld_contact", args)?;
    execute_delete_iec_ld_contact(doc, &args)
}

fn execute_delete_iec_ld_contact(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdContactArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.delete_contact(
                args.contact_offset,
                args.expected_raw_x,
                &args.expected_contact_kind,
                &args.expected_variable,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdNoContactArgs {
    program_index: usize,
    contact_offset: usize,
    expected_raw_x: u8,
    expected_variable: String,
}

fn schema_delete_iec_ld_no_contact() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdNoContactArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_no_contact() -> Value {
    json!({"program_index": 0, "contact_offset": 0, "expected_raw_x": 0, "expected_variable": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_no_contact(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteIecLdNoContactArgs = parse_args("delete_iec_ld_no_contact", args)?;
    execute_delete_iec_ld_no_contact(doc, &args)
}

fn execute_delete_iec_ld_no_contact(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdNoContactArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.delete_no_contact(
                args.contact_offset,
                args.expected_raw_x,
                &args.expected_variable,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdContactCellArgs {
    program_index: usize,
    contact_offset: usize,
    expected_raw_x: u8,
    expected_contact_kind: String,
    expected_variable: String,
}

fn schema_delete_iec_ld_contact_cell() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdContactCellArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_contact_cell() -> Value {
    json!({"program_index": 0, "contact_offset": 0, "expected_raw_x": 0, "expected_contact_kind": "<current value from inspect_file>", "expected_variable": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_contact_cell(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteIecLdContactCellArgs = parse_args("delete_iec_ld_contact_cell", args)?;
    execute_delete_iec_ld_contact_cell(doc, &args)
}

fn execute_delete_iec_ld_contact_cell(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdContactCellArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.delete_contact_cell(
                args.contact_offset,
                args.expected_raw_x,
                &args.expected_contact_kind,
                &args.expected_variable,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdNoContactCellArgs {
    program_index: usize,
    contact_offset: usize,
    expected_raw_x: u8,
    expected_variable: String,
}

fn schema_delete_iec_ld_no_contact_cell() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdNoContactCellArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_no_contact_cell() -> Value {
    json!({"program_index": 0, "contact_offset": 0, "expected_raw_x": 0, "expected_variable": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_no_contact_cell(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteIecLdNoContactCellArgs = parse_args("delete_iec_ld_no_contact_cell", args)?;
    execute_delete_iec_ld_no_contact_cell(doc, &args)
}

fn execute_delete_iec_ld_no_contact_cell(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdNoContactCellArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.delete_no_contact_cell(
                args.contact_offset,
                args.expected_raw_x,
                &args.expected_variable,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdHorizontalWireArgs {
    program_index: usize,
    wire_offset: usize,
    expected_raw_x: u8,
}

fn schema_delete_iec_ld_horizontal_wire() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdHorizontalWireArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_horizontal_wire() -> Value {
    json!({"program_index": 0, "wire_offset": 0, "expected_raw_x": 0})
}

fn apply_delete_iec_ld_horizontal_wire(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteIecLdHorizontalWireArgs = parse_args("delete_iec_ld_horizontal_wire", args)?;
    execute_delete_iec_ld_horizontal_wire(doc, &args)
}

fn execute_delete_iec_ld_horizontal_wire(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdHorizontalWireArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_horizontal_wire(args.wire_offset, args.expected_raw_x)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteLadderHorizontalWireArgs {
    program_index: usize,
    raw_y: u32,
    start_x: u8,
    end_x: u8,
}

fn schema_delete_ladder_horizontal_wire() -> Value {
    serde_json::to_value(schema_for!(DeleteLadderHorizontalWireArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_ladder_horizontal_wire() -> Value {
    json!({"program_index": 0, "raw_y": 0, "start_x": 0, "end_x": 0})
}

fn apply_delete_ladder_horizontal_wire(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteLadderHorizontalWireArgs = parse_args("delete_ladder_horizontal_wire", args)?;
    execute_delete_ladder_horizontal_wire(doc, &args)
}

fn execute_delete_ladder_horizontal_wire(
    doc: &mut XgwxDocument,
    args: &DeleteLadderHorizontalWireArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .ladder()?
                .delete_horizontal_wire(args.raw_y, args.start_x, args.end_x)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteLadderVerticalWireArgs {
    program_index: usize,
    raw_x: u8,
    start_y: u32,
    end_y: u32,
}

fn schema_delete_ladder_vertical_wire() -> Value {
    serde_json::to_value(schema_for!(DeleteLadderVerticalWireArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_ladder_vertical_wire() -> Value {
    json!({"program_index": 0, "raw_x": 0, "start_y": 0, "end_y": 0})
}

fn apply_delete_ladder_vertical_wire(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteLadderVerticalWireArgs = parse_args("delete_ladder_vertical_wire", args)?;
    execute_delete_ladder_vertical_wire(doc, &args)
}

fn execute_delete_ladder_vertical_wire(
    doc: &mut XgwxDocument,
    args: &DeleteLadderVerticalWireArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .ladder()?
                .delete_vertical_wire(args.raw_x, args.start_y, args.end_y)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdHorizontalWireRecordArgs {
    program_index: usize,
    offset: usize,
    expected_start_x: u8,
    expected_end_x: u8,
}

fn schema_delete_iec_ld_horizontal_wire_record() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdHorizontalWireRecordArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_horizontal_wire_record() -> Value {
    json!({"program_index": 0, "offset": 0, "expected_start_x": 0, "expected_end_x": 0})
}

fn apply_delete_iec_ld_horizontal_wire_record(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: DeleteIecLdHorizontalWireRecordArgs =
        parse_args("delete_iec_ld_horizontal_wire_record", args)?;
    execute_delete_iec_ld_horizontal_wire_record(doc, &args)
}

fn execute_delete_iec_ld_horizontal_wire_record(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdHorizontalWireRecordArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.delete_horizontal_wire_record(
                args.offset,
                args.expected_start_x,
                args.expected_end_x,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdIsolatedElementArgs {
    program_index: usize,
    offset: usize,
    expected: String,
}

fn schema_delete_iec_ld_isolated_element() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdIsolatedElementArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_isolated_element() -> Value {
    json!({"program_index": 0, "offset": 0, "expected": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_isolated_element(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteIecLdIsolatedElementArgs = parse_args("delete_iec_ld_isolated_element", args)?;
    execute_delete_iec_ld_isolated_element(doc, &args)
}

fn execute_delete_iec_ld_isolated_element(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdIsolatedElementArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_isolated_element(args.offset, &args.expected)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertIecLdFunctionOutputWireArgs {
    program_index: usize,
    block_offset: usize,
    expected_name: String,
    pin_name: String,
    start_x: u8,
}

fn schema_insert_iec_ld_function_output_wire() -> Value {
    serde_json::to_value(schema_for!(InsertIecLdFunctionOutputWireArgs))
        .expect("JSON Schema is serializable")
}

fn example_insert_iec_ld_function_output_wire() -> Value {
    json!({"program_index": 0, "block_offset": 0, "expected_name": "<current value from inspect_file>", "pin_name": "<requested value>", "start_x": 0})
}

fn apply_insert_iec_ld_function_output_wire(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: InsertIecLdFunctionOutputWireArgs =
        parse_args("insert_iec_ld_function_output_wire", args)?;
    execute_insert_iec_ld_function_output_wire(doc, &args)
}

fn execute_insert_iec_ld_function_output_wire(
    doc: &mut XgwxDocument,
    args: &InsertIecLdFunctionOutputWireArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.insert_function_output_wire(
                args.block_offset,
                &args.expected_name,
                &args.pin_name,
                args.start_x,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct RepairIecLdHorizontalWireArgs {
    program_index: usize,
    insertion_offset: usize,
    expected_raw_x: u8,
}

fn schema_repair_iec_ld_horizontal_wire() -> Value {
    serde_json::to_value(schema_for!(RepairIecLdHorizontalWireArgs))
        .expect("JSON Schema is serializable")
}

fn example_repair_iec_ld_horizontal_wire() -> Value {
    json!({"program_index": 0, "insertion_offset": 0, "expected_raw_x": 0})
}

fn apply_repair_iec_ld_horizontal_wire(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: RepairIecLdHorizontalWireArgs = parse_args("repair_iec_ld_horizontal_wire", args)?;
    execute_repair_iec_ld_horizontal_wire(doc, &args)
}

fn execute_repair_iec_ld_horizontal_wire(
    doc: &mut XgwxDocument,
    args: &RepairIecLdHorizontalWireArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .repair_horizontal_wire(args.insertion_offset, args.expected_raw_x)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdFunctionOutputOperandArgs {
    program_index: usize,
    offset: usize,
    expected: String,
}

fn schema_delete_iec_ld_function_output_operand() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdFunctionOutputOperandArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_function_output_operand() -> Value {
    json!({"program_index": 0, "offset": 0, "expected": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_function_output_operand(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: DeleteIecLdFunctionOutputOperandArgs =
        parse_args("delete_iec_ld_function_output_operand", args)?;
    execute_delete_iec_ld_function_output_operand(doc, &args)
}

fn execute_delete_iec_ld_function_output_operand(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdFunctionOutputOperandArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_function_output_operand(args.offset, &args.expected)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct AssignIecLdFunctionOutputOperandArgs {
    program_index: usize,
    block_offset: usize,
    expected_name: String,
    ordinal: u8,
    value: String,
}

fn schema_assign_iec_ld_function_output_operand() -> Value {
    serde_json::to_value(schema_for!(AssignIecLdFunctionOutputOperandArgs))
        .expect("JSON Schema is serializable")
}

fn example_assign_iec_ld_function_output_operand() -> Value {
    json!({"program_index": 0, "block_offset": 0, "expected_name": "<current value from inspect_file>", "ordinal": 0, "value": "<requested value>"})
}

fn apply_assign_iec_ld_function_output_operand(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: AssignIecLdFunctionOutputOperandArgs =
        parse_args("assign_iec_ld_function_output_operand", args)?;
    execute_assign_iec_ld_function_output_operand(doc, &args)
}

fn execute_assign_iec_ld_function_output_operand(
    doc: &mut XgwxDocument,
    args: &AssignIecLdFunctionOutputOperandArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.assign_function_output_operand(
                args.block_offset,
                &args.expected_name,
                args.ordinal,
                &args.value,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct UpdateIecLdFunctionOperandArgs {
    program_index: usize,
    offset: usize,
    expected: String,
    replacement: String,
}

fn schema_update_iec_ld_function_operand() -> Value {
    serde_json::to_value(schema_for!(UpdateIecLdFunctionOperandArgs))
        .expect("JSON Schema is serializable")
}

fn example_update_iec_ld_function_operand() -> Value {
    json!({"program_index": 0, "offset": 0, "expected": "<current value from inspect_file>", "replacement": "<requested value>"})
}

fn apply_update_iec_ld_function_operand(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: UpdateIecLdFunctionOperandArgs = parse_args("update_iec_ld_function_operand", args)?;
    execute_update_iec_ld_function_operand(doc, &args)
}

fn execute_update_iec_ld_function_operand(
    doc: &mut XgwxDocument,
    args: &UpdateIecLdFunctionOperandArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.update_function_operand(
                args.offset,
                &args.expected,
                &args.replacement,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdScalarChainFunctionArgs {
    program_index: usize,
    block_offset: usize,
    expected_name: String,
}

fn schema_delete_iec_ld_scalar_chain_function() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdScalarChainFunctionArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_scalar_chain_function() -> Value {
    json!({"program_index": 0, "block_offset": 0, "expected_name": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_scalar_chain_function(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: DeleteIecLdScalarChainFunctionArgs =
        parse_args("delete_iec_ld_scalar_chain_function", args)?;
    execute_delete_iec_ld_scalar_chain_function(doc, &args)
}

fn execute_delete_iec_ld_scalar_chain_function(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdScalarChainFunctionArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_scalar_chain_function(args.block_offset, &args.expected_name)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ReplaceIecLdScalarChainFunctionArgs {
    program_index: usize,
    block_offset: usize,
    expected_name: String,
    function_name: String,
    operands: Vec<String>,
}

fn schema_replace_iec_ld_scalar_chain_function() -> Value {
    serde_json::to_value(schema_for!(ReplaceIecLdScalarChainFunctionArgs))
        .expect("JSON Schema is serializable")
}

fn example_replace_iec_ld_scalar_chain_function() -> Value {
    json!({"program_index": 0, "block_offset": 0, "expected_name": "<current value from inspect_file>", "function_name": "<requested value>", "operands": []})
}

fn apply_replace_iec_ld_scalar_chain_function(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: ReplaceIecLdScalarChainFunctionArgs =
        parse_args("replace_iec_ld_scalar_chain_function", args)?;
    execute_replace_iec_ld_scalar_chain_function(doc, &args)
}

fn execute_replace_iec_ld_scalar_chain_function(
    doc: &mut XgwxDocument,
    args: &ReplaceIecLdScalarChainFunctionArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.replace_scalar_chain_function(
                args.block_offset,
                &args.expected_name,
                &args.function_name,
                &args.operands,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdTerminalFunctionArgs {
    program_index: usize,
    block_offset: usize,
    expected_name: String,
}

fn schema_delete_iec_ld_terminal_function() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdTerminalFunctionArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_terminal_function() -> Value {
    json!({"program_index": 0, "block_offset": 0, "expected_name": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_terminal_function(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: DeleteIecLdTerminalFunctionArgs =
        parse_args("delete_iec_ld_terminal_function", args)?;
    execute_delete_iec_ld_terminal_function(doc, &args)
}

fn execute_delete_iec_ld_terminal_function(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdTerminalFunctionArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_terminal_function(args.block_offset, &args.expected_name)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertIecLdTerminalTimerArgs {
    program_index: usize,
    contact_offset: usize,
    instance: String,
    preset: String,
    elapsed: String,
}

fn schema_insert_iec_ld_terminal_timer() -> Value {
    serde_json::to_value(schema_for!(InsertIecLdTerminalTimerArgs))
        .expect("JSON Schema is serializable")
}

fn example_insert_iec_ld_terminal_timer() -> Value {
    json!({"program_index": 0, "contact_offset": 0, "instance": "<requested value>", "preset": "<requested value>", "elapsed": "<requested value>"})
}

fn apply_insert_iec_ld_terminal_timer(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: InsertIecLdTerminalTimerArgs = parse_args("insert_iec_ld_terminal_timer", args)?;
    execute_insert_iec_ld_terminal_timer(doc, &args)
}

fn execute_insert_iec_ld_terminal_timer(
    doc: &mut XgwxDocument,
    args: &InsertIecLdTerminalTimerArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.insert_terminal_timer(
                args.contact_offset,
                &args.instance,
                &args.preset,
                &args.elapsed,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertIecLdTerminalMoveArgs {
    program_index: usize,
    contact_offset: usize,
    input_operand: String,
    output_operand: String,
}

fn schema_insert_iec_ld_terminal_move() -> Value {
    serde_json::to_value(schema_for!(InsertIecLdTerminalMoveArgs))
        .expect("JSON Schema is serializable")
}

fn example_insert_iec_ld_terminal_move() -> Value {
    json!({"program_index": 0, "contact_offset": 0, "input_operand": "<requested value>", "output_operand": "<requested value>"})
}

fn apply_insert_iec_ld_terminal_move(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: InsertIecLdTerminalMoveArgs = parse_args("insert_iec_ld_terminal_move", args)?;
    execute_insert_iec_ld_terminal_move(doc, &args)
}

fn execute_insert_iec_ld_terminal_move(
    doc: &mut XgwxDocument,
    args: &InsertIecLdTerminalMoveArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.insert_terminal_move(
                args.contact_offset,
                &args.input_operand,
                &args.output_operand,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdStandaloneFunctionArgs {
    program_index: usize,
    block_offset: usize,
    expected_name: String,
}

fn schema_delete_iec_ld_standalone_function() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdStandaloneFunctionArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_standalone_function() -> Value {
    json!({"program_index": 0, "block_offset": 0, "expected_name": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_standalone_function(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: DeleteIecLdStandaloneFunctionArgs =
        parse_args("delete_iec_ld_standalone_function", args)?;
    execute_delete_iec_ld_standalone_function(doc, &args)
}

fn execute_delete_iec_ld_standalone_function(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdStandaloneFunctionArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_standalone_function(args.block_offset, &args.expected_name)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdGroupArgs {
    program_index: usize,
    group_index: usize,
    expected_first_row_index: u16,
}

fn schema_delete_iec_ld_group() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdGroupArgs)).expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_group() -> Value {
    json!({"program_index": 0, "group_index": 0, "expected_first_row_index": 0})
}

fn apply_delete_iec_ld_group(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteIecLdGroupArgs = parse_args("delete_iec_ld_group", args)?;
    execute_delete_iec_ld_group(doc, &args)
}

fn execute_delete_iec_ld_group(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdGroupArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_group(args.group_index, args.expected_first_row_index)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ReplaceIecLdGroupArgs {
    program_index: usize,
    source_group_index: usize,
    expected_source_first_row_index: u16,
    destination_group_index: usize,
    expected_destination_first_row_index: u16,
}

fn schema_replace_iec_ld_group() -> Value {
    serde_json::to_value(schema_for!(ReplaceIecLdGroupArgs)).expect("JSON Schema is serializable")
}

fn example_replace_iec_ld_group() -> Value {
    json!({"program_index": 0, "source_group_index": 0, "expected_source_first_row_index": 0, "destination_group_index": 0, "expected_destination_first_row_index": 0})
}

fn apply_replace_iec_ld_group(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: ReplaceIecLdGroupArgs = parse_args("replace_iec_ld_group", args)?;
    execute_replace_iec_ld_group(doc, &args)
}

fn execute_replace_iec_ld_group(
    doc: &mut XgwxDocument,
    args: &ReplaceIecLdGroupArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.replace_group(
                args.source_group_index,
                args.expected_source_first_row_index,
                args.destination_group_index,
                args.expected_destination_first_row_index,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ReplaceIecLdGroupFromProgramArgs {
    source_program_index: usize,
    source_group_index: usize,
    expected_source_first_row_index: u16,
    destination_program_index: usize,
    destination_group_index: usize,
    expected_destination_first_row_index: u16,
    copy_missing_locals: bool,
}

fn schema_replace_iec_ld_group_from_program() -> Value {
    serde_json::to_value(schema_for!(ReplaceIecLdGroupFromProgramArgs))
        .expect("JSON Schema is serializable")
}

fn example_replace_iec_ld_group_from_program() -> Value {
    json!({"source_program_index": 0, "source_group_index": 0, "expected_source_first_row_index": 0, "destination_program_index": 0, "destination_group_index": 0, "expected_destination_first_row_index": 0, "copy_missing_locals": false})
}

fn apply_replace_iec_ld_group_from_program(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: ReplaceIecLdGroupFromProgramArgs =
        parse_args("replace_iec_ld_group_from_program", args)?;
    execute_replace_iec_ld_group_from_program(doc, &args)
}

fn execute_replace_iec_ld_group_from_program(
    doc: &mut XgwxDocument,
    args: &ReplaceIecLdGroupFromProgramArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.destination_program_index)
        .and_then(|mut program| {
            program.iec_ld()?.replace_group_from_program(
                args.source_program_index,
                args.source_group_index,
                args.expected_source_first_row_index,
                args.destination_group_index,
                args.expected_destination_first_row_index,
                args.copy_missing_locals,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct MoveIecLdGroupArgs {
    program_index: usize,
    group_index: usize,
    expected_first_row_index: u16,
    destination_first_row_index: u16,
}

fn schema_move_iec_ld_group() -> Value {
    serde_json::to_value(schema_for!(MoveIecLdGroupArgs)).expect("JSON Schema is serializable")
}

fn example_move_iec_ld_group() -> Value {
    json!({"program_index": 0, "group_index": 0, "expected_first_row_index": 0, "destination_first_row_index": 0})
}

fn apply_move_iec_ld_group(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: MoveIecLdGroupArgs = parse_args("move_iec_ld_group", args)?;
    execute_move_iec_ld_group(doc, &args)
}

fn execute_move_iec_ld_group(
    doc: &mut XgwxDocument,
    args: &MoveIecLdGroupArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.move_group(
                args.group_index,
                args.expected_first_row_index,
                args.destination_first_row_index,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct CopyIecLdGroupArgs {
    program_index: usize,
    group_index: usize,
    expected_first_row_index: u16,
    destination_first_row_index: u16,
}

fn schema_copy_iec_ld_group() -> Value {
    serde_json::to_value(schema_for!(CopyIecLdGroupArgs)).expect("JSON Schema is serializable")
}

fn example_copy_iec_ld_group() -> Value {
    json!({"program_index": 0, "group_index": 0, "expected_first_row_index": 0, "destination_first_row_index": 0})
}

fn apply_copy_iec_ld_group(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: CopyIecLdGroupArgs = parse_args("copy_iec_ld_group", args)?;
    execute_copy_iec_ld_group(doc, &args)
}

fn execute_copy_iec_ld_group(
    doc: &mut XgwxDocument,
    args: &CopyIecLdGroupArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.copy_group(
                args.group_index,
                args.expected_first_row_index,
                args.destination_first_row_index,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct CopyIecLdGroupToProgramArgs {
    source_program_index: usize,
    group_index: usize,
    expected_first_row_index: u16,
    destination_program_index: usize,
    destination_first_row_index: u16,
}

fn schema_copy_iec_ld_group_to_program() -> Value {
    serde_json::to_value(schema_for!(CopyIecLdGroupToProgramArgs))
        .expect("JSON Schema is serializable")
}

fn example_copy_iec_ld_group_to_program() -> Value {
    json!({"source_program_index": 0, "group_index": 0, "expected_first_row_index": 0, "destination_program_index": 0, "destination_first_row_index": 0})
}

fn apply_copy_iec_ld_group_to_program(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: CopyIecLdGroupToProgramArgs = parse_args("copy_iec_ld_group_to_program", args)?;
    execute_copy_iec_ld_group_to_program(doc, &args)
}

fn execute_copy_iec_ld_group_to_program(
    doc: &mut XgwxDocument,
    args: &CopyIecLdGroupToProgramArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.source_program_index)
        .and_then(|mut program| {
            program.iec_ld()?.copy_group_to_program(
                args.group_index,
                args.expected_first_row_index,
                args.destination_program_index,
                args.destination_first_row_index,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct CopyIecLdGroupToProgramWithLocalsArgs {
    source_program_index: usize,
    group_index: usize,
    expected_first_row_index: u16,
    destination_program_index: usize,
    destination_first_row_index: u16,
}

fn schema_copy_iec_ld_group_to_program_with_locals() -> Value {
    serde_json::to_value(schema_for!(CopyIecLdGroupToProgramWithLocalsArgs))
        .expect("JSON Schema is serializable")
}

fn example_copy_iec_ld_group_to_program_with_locals() -> Value {
    json!({"source_program_index": 0, "group_index": 0, "expected_first_row_index": 0, "destination_program_index": 0, "destination_first_row_index": 0})
}

fn apply_copy_iec_ld_group_to_program_with_locals(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: CopyIecLdGroupToProgramWithLocalsArgs =
        parse_args("copy_iec_ld_group_to_program_with_locals", args)?;
    execute_copy_iec_ld_group_to_program_with_locals(doc, &args)
}

fn execute_copy_iec_ld_group_to_program_with_locals(
    doc: &mut XgwxDocument,
    args: &CopyIecLdGroupToProgramWithLocalsArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.source_program_index)
        .and_then(|mut program| {
            program.iec_ld()?.copy_group_to_program_with_locals(
                args.group_index,
                args.expected_first_row_index,
                args.destination_program_index,
                args.destination_first_row_index,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertIecLdCommentArgs {
    program_index: usize,
    destination_row_index: u16,
    text: String,
}

fn schema_insert_iec_ld_comment() -> Value {
    serde_json::to_value(schema_for!(InsertIecLdCommentArgs)).expect("JSON Schema is serializable")
}

fn example_insert_iec_ld_comment() -> Value {
    json!({"program_index": 0, "destination_row_index": 0, "text": "<requested value>"})
}

fn apply_insert_iec_ld_comment(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: InsertIecLdCommentArgs = parse_args("insert_iec_ld_comment", args)?;
    execute_insert_iec_ld_comment(doc, &args)
}

fn execute_insert_iec_ld_comment(
    doc: &mut XgwxDocument,
    args: &InsertIecLdCommentArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .insert_comment(args.destination_row_index, &args.text)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdBranchFunctionArgs {
    program_index: usize,
    block_offset: usize,
    expected_name: String,
}

fn schema_delete_iec_ld_branch_function() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdBranchFunctionArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_branch_function() -> Value {
    json!({"program_index": 0, "block_offset": 0, "expected_name": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_branch_function(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteIecLdBranchFunctionArgs = parse_args("delete_iec_ld_branch_function", args)?;
    execute_delete_iec_ld_branch_function(doc, &args)
}

fn execute_delete_iec_ld_branch_function(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdBranchFunctionArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_branch_function(args.block_offset, &args.expected_name)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ReplaceIecLdBranchFunctionArgs {
    program_index: usize,
    block_offset: usize,
    expected_name: String,
    function_name: String,
    operands: Vec<String>,
}

fn schema_replace_iec_ld_branch_function() -> Value {
    serde_json::to_value(schema_for!(ReplaceIecLdBranchFunctionArgs))
        .expect("JSON Schema is serializable")
}

fn example_replace_iec_ld_branch_function() -> Value {
    json!({"program_index": 0, "block_offset": 0, "expected_name": "<current value from inspect_file>", "function_name": "<requested value>", "operands": []})
}

fn apply_replace_iec_ld_branch_function(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: ReplaceIecLdBranchFunctionArgs = parse_args("replace_iec_ld_branch_function", args)?;
    execute_replace_iec_ld_branch_function(doc, &args)
}

fn execute_replace_iec_ld_branch_function(
    doc: &mut XgwxDocument,
    args: &ReplaceIecLdBranchFunctionArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.replace_branch_function(
                args.block_offset,
                &args.expected_name,
                &args.function_name,
                &args.operands,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertIecLdFunctionArgs {
    program_index: usize,
    row_index: u16,
    raw_x: u8,
    function_name: String,
    operands: Vec<String>,
}

fn schema_insert_iec_ld_function() -> Value {
    serde_json::to_value(schema_for!(InsertIecLdFunctionArgs)).expect("JSON Schema is serializable")
}

fn example_insert_iec_ld_function() -> Value {
    json!({"program_index": 0, "row_index": 0, "raw_x": 0, "function_name": "<requested value>", "operands": []})
}

fn apply_insert_iec_ld_function(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: InsertIecLdFunctionArgs = parse_args("insert_iec_ld_function", args)?;
    execute_insert_iec_ld_function(doc, &args)
}

fn execute_insert_iec_ld_function(
    doc: &mut XgwxDocument,
    args: &InsertIecLdFunctionArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.insert_function(
                args.row_index,
                args.raw_x,
                &args.function_name,
                &args.operands,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertIecLdStandaloneFunctionArgs {
    program_index: usize,
    insertion_offset: usize,
    function_name: String,
    input_operand: String,
    output_operand: String,
}

fn schema_insert_iec_ld_standalone_function() -> Value {
    serde_json::to_value(schema_for!(InsertIecLdStandaloneFunctionArgs))
        .expect("JSON Schema is serializable")
}

fn example_insert_iec_ld_standalone_function() -> Value {
    json!({"program_index": 0, "insertion_offset": 0, "function_name": "<requested value>", "input_operand": "<requested value>", "output_operand": "<requested value>"})
}

fn apply_insert_iec_ld_standalone_function(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: InsertIecLdStandaloneFunctionArgs =
        parse_args("insert_iec_ld_standalone_function", args)?;
    execute_insert_iec_ld_standalone_function(doc, &args)
}

fn execute_insert_iec_ld_standalone_function(
    doc: &mut XgwxDocument,
    args: &InsertIecLdStandaloneFunctionArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.insert_standalone_function(
                args.insertion_offset,
                &args.function_name,
                &args.input_operand,
                &args.output_operand,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertIecLdFunctionCellArgs {
    program_index: usize,
    insertion_offset: usize,
    function_name: String,
    instance_name: String,
}

fn schema_insert_iec_ld_function_cell() -> Value {
    serde_json::to_value(schema_for!(InsertIecLdFunctionCellArgs))
        .expect("JSON Schema is serializable")
}

fn example_insert_iec_ld_function_cell() -> Value {
    json!({"program_index": 0, "insertion_offset": 0, "function_name": "<requested value>", "instance_name": "<requested value>"})
}

fn apply_insert_iec_ld_function_cell(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: InsertIecLdFunctionCellArgs = parse_args("insert_iec_ld_function_cell", args)?;
    execute_insert_iec_ld_function_cell(doc, &args)
}

fn execute_insert_iec_ld_function_cell(
    doc: &mut XgwxDocument,
    args: &InsertIecLdFunctionCellArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.insert_function_cell(
                args.insertion_offset,
                &args.function_name,
                &args.instance_name,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdFunctionCellArgs {
    program_index: usize,
    block_offset: usize,
    expected_name: String,
}

fn schema_delete_iec_ld_function_cell() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdFunctionCellArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_function_cell() -> Value {
    json!({"program_index": 0, "block_offset": 0, "expected_name": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_function_cell(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteIecLdFunctionCellArgs = parse_args("delete_iec_ld_function_cell", args)?;
    execute_delete_iec_ld_function_cell(doc, &args)
}

fn execute_delete_iec_ld_function_cell(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdFunctionCellArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_function_cell(args.block_offset, &args.expected_name)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdEqChainHeadArgs {
    program_index: usize,
    block_offset: usize,
    expected_name: String,
}

fn schema_delete_iec_ld_eq_chain_head() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdEqChainHeadArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_eq_chain_head() -> Value {
    json!({"program_index": 0, "block_offset": 0, "expected_name": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_eq_chain_head(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteIecLdEqChainHeadArgs = parse_args("delete_iec_ld_eq_chain_head", args)?;
    execute_delete_iec_ld_eq_chain_head(doc, &args)
}

fn execute_delete_iec_ld_eq_chain_head(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdEqChainHeadArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_eq_chain_head(args.block_offset, &args.expected_name)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdHeatingChainHeadArgs {
    program_index: usize,
    block_offset: usize,
    expected_name: String,
}

fn schema_delete_iec_ld_heating_chain_head() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdHeatingChainHeadArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_heating_chain_head() -> Value {
    json!({"program_index": 0, "block_offset": 0, "expected_name": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_heating_chain_head(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: DeleteIecLdHeatingChainHeadArgs =
        parse_args("delete_iec_ld_heating_chain_head", args)?;
    execute_delete_iec_ld_heating_chain_head(doc, &args)
}

fn execute_delete_iec_ld_heating_chain_head(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdHeatingChainHeadArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_heating_chain_head(args.block_offset, &args.expected_name)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdHeatingChainMiddleArgs {
    program_index: usize,
    block_offset: usize,
    expected_name: String,
}

fn schema_delete_iec_ld_heating_chain_middle() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdHeatingChainMiddleArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_heating_chain_middle() -> Value {
    json!({"program_index": 0, "block_offset": 0, "expected_name": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_heating_chain_middle(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: DeleteIecLdHeatingChainMiddleArgs =
        parse_args("delete_iec_ld_heating_chain_middle", args)?;
    execute_delete_iec_ld_heating_chain_middle(doc, &args)
}

fn execute_delete_iec_ld_heating_chain_middle(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdHeatingChainMiddleArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_heating_chain_middle(args.block_offset, &args.expected_name)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdHeatingChainX15EqArgs {
    program_index: usize,
    block_offset: usize,
    expected_name: String,
}

fn schema_delete_iec_ld_heating_chain_x15_eq() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdHeatingChainX15EqArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_heating_chain_x15_eq() -> Value {
    json!({"program_index": 0, "block_offset": 0, "expected_name": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_heating_chain_x15_eq(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: DeleteIecLdHeatingChainX15EqArgs =
        parse_args("delete_iec_ld_heating_chain_x15_eq", args)?;
    execute_delete_iec_ld_heating_chain_x15_eq(doc, &args)
}

fn execute_delete_iec_ld_heating_chain_x15_eq(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdHeatingChainX15EqArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_heating_chain_x15_eq(args.block_offset, &args.expected_name)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdHeatingChainContactEqArgs {
    program_index: usize,
    block_offset: usize,
    expected_name: String,
}

fn schema_delete_iec_ld_heating_chain_contact_eq() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdHeatingChainContactEqArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_heating_chain_contact_eq() -> Value {
    json!({"program_index": 0, "block_offset": 0, "expected_name": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_heating_chain_contact_eq(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: DeleteIecLdHeatingChainContactEqArgs =
        parse_args("delete_iec_ld_heating_chain_contact_eq", args)?;
    execute_delete_iec_ld_heating_chain_contact_eq(doc, &args)
}

fn execute_delete_iec_ld_heating_chain_contact_eq(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdHeatingChainContactEqArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_heating_chain_contact_eq(args.block_offset, &args.expected_name)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdHeatingChainX3EqRepairedArgs {
    program_index: usize,
    block_offset: usize,
    expected_name: String,
}

fn schema_delete_iec_ld_heating_chain_x3_eq_repaired() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdHeatingChainX3EqRepairedArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_heating_chain_x3_eq_repaired() -> Value {
    json!({"program_index": 0, "block_offset": 0, "expected_name": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_heating_chain_x3_eq_repaired(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: DeleteIecLdHeatingChainX3EqRepairedArgs =
        parse_args("delete_iec_ld_heating_chain_x3_eq_repaired", args)?;
    execute_delete_iec_ld_heating_chain_x3_eq_repaired(doc, &args)
}

fn execute_delete_iec_ld_heating_chain_x3_eq_repaired(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdHeatingChainX3EqRepairedArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_heating_chain_x3_eq_repaired(args.block_offset, &args.expected_name)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdHeatingChainX3EqArgs {
    program_index: usize,
    block_offset: usize,
    expected_name: String,
}

fn schema_delete_iec_ld_heating_chain_x3_eq() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdHeatingChainX3EqArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_heating_chain_x3_eq() -> Value {
    json!({"program_index": 0, "block_offset": 0, "expected_name": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_heating_chain_x3_eq(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: DeleteIecLdHeatingChainX3EqArgs =
        parse_args("delete_iec_ld_heating_chain_x3_eq", args)?;
    execute_delete_iec_ld_heating_chain_x3_eq(doc, &args)
}

fn execute_delete_iec_ld_heating_chain_x3_eq(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdHeatingChainX3EqArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_heating_chain_x3_eq(args.block_offset, &args.expected_name)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct RepairIecLdHeatingChainX3EqDeletionArgs {
    program_index: usize,
}

fn schema_repair_iec_ld_heating_chain_x3_eq_deletion() -> Value {
    serde_json::to_value(schema_for!(RepairIecLdHeatingChainX3EqDeletionArgs))
        .expect("JSON Schema is serializable")
}

fn example_repair_iec_ld_heating_chain_x3_eq_deletion() -> Value {
    json!({"program_index": 0})
}

fn apply_repair_iec_ld_heating_chain_x3_eq_deletion(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: RepairIecLdHeatingChainX3EqDeletionArgs =
        parse_args("repair_iec_ld_heating_chain_x3_eq_deletion", args)?;
    execute_repair_iec_ld_heating_chain_x3_eq_deletion(doc, &args)
}

fn execute_repair_iec_ld_heating_chain_x3_eq_deletion(
    doc: &mut XgwxDocument,
    args: &RepairIecLdHeatingChainX3EqDeletionArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut selected_program| {
            selected_program
                .iec_ld()?
                .repair_heating_chain_x3_eq_deletion()
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdConnectedArithmeticArgs {
    program_index: usize,
    block_offset: usize,
    expected_name: String,
}

fn schema_delete_iec_ld_connected_arithmetic() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdConnectedArithmeticArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_connected_arithmetic() -> Value {
    json!({"program_index": 0, "block_offset": 0, "expected_name": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_connected_arithmetic(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: DeleteIecLdConnectedArithmeticArgs =
        parse_args("delete_iec_ld_connected_arithmetic", args)?;
    execute_delete_iec_ld_connected_arithmetic(doc, &args)
}

fn execute_delete_iec_ld_connected_arithmetic(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdConnectedArithmeticArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_connected_arithmetic(args.block_offset, &args.expected_name)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteIecLdBranchedArithmeticArgs {
    program_index: usize,
    block_offset: usize,
    expected_name: String,
}

fn schema_delete_iec_ld_branched_arithmetic() -> Value {
    serde_json::to_value(schema_for!(DeleteIecLdBranchedArithmeticArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_iec_ld_branched_arithmetic() -> Value {
    json!({"program_index": 0, "block_offset": 0, "expected_name": "<current value from inspect_file>"})
}

fn apply_delete_iec_ld_branched_arithmetic(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: DeleteIecLdBranchedArithmeticArgs =
        parse_args("delete_iec_ld_branched_arithmetic", args)?;
    execute_delete_iec_ld_branched_arithmetic(doc, &args)
}

fn execute_delete_iec_ld_branched_arithmetic(
    doc: &mut XgwxDocument,
    args: &DeleteIecLdBranchedArithmeticArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .iec_ld()?
                .delete_branched_arithmetic(args.block_offset, &args.expected_name)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct UpdateIecLdArithmeticFunctionArgs {
    program_index: usize,
    offset: usize,
    expected: String,
    replacement: String,
}

fn schema_update_iec_ld_arithmetic_function() -> Value {
    serde_json::to_value(schema_for!(UpdateIecLdArithmeticFunctionArgs))
        .expect("JSON Schema is serializable")
}

fn example_update_iec_ld_arithmetic_function() -> Value {
    json!({"program_index": 0, "offset": 0, "expected": "<current value from inspect_file>", "replacement": "<requested value>"})
}

fn apply_update_iec_ld_arithmetic_function(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: UpdateIecLdArithmeticFunctionArgs =
        parse_args("update_iec_ld_arithmetic_function", args)?;
    execute_update_iec_ld_arithmetic_function(doc, &args)
}

fn execute_update_iec_ld_arithmetic_function(
    doc: &mut XgwxDocument,
    args: &UpdateIecLdArithmeticFunctionArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.update_arithmetic_function(
                args.offset,
                &args.expected,
                &args.replacement,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct UpdateIecLdComparisonFunctionArgs {
    program_index: usize,
    offset: usize,
    expected: String,
    replacement: String,
}

fn schema_update_iec_ld_comparison_function() -> Value {
    serde_json::to_value(schema_for!(UpdateIecLdComparisonFunctionArgs))
        .expect("JSON Schema is serializable")
}

fn example_update_iec_ld_comparison_function() -> Value {
    json!({"program_index": 0, "offset": 0, "expected": "<current value from inspect_file>", "replacement": "<requested value>"})
}

fn apply_update_iec_ld_comparison_function(
    doc: &mut XgwxDocument,
    args: Value,
) -> Result<(), String> {
    let args: UpdateIecLdComparisonFunctionArgs =
        parse_args("update_iec_ld_comparison_function", args)?;
    execute_update_iec_ld_comparison_function(doc, &args)
}

fn execute_update_iec_ld_comparison_function(
    doc: &mut XgwxDocument,
    args: &UpdateIecLdComparisonFunctionArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.iec_ld()?.update_comparison_function(
                args.offset,
                &args.expected,
                &args.replacement,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct EditLadderCellArgs {
    program_index: usize,
    edit: LadderCellEdit,
}

fn schema_edit_ladder_cell() -> Value {
    serde_json::to_value(schema_for!(EditLadderCellArgs)).expect("JSON Schema is serializable")
}

fn example_edit_ladder_cell() -> Value {
    json!({"program_index": 0, "edit": {"raw_y": 0, "column": 0, "expected": null, "replacement": null}})
}

fn apply_edit_ladder_cell(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: EditLadderCellArgs = parse_args("edit_ladder_cell", args)?;
    execute_edit_ladder_cell(doc, &args)
}

fn execute_edit_ladder_cell(
    doc: &mut XgwxDocument,
    args: &EditLadderCellArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut selected_program| {
            selected_program
                .ladder()?
                .edit_cell(&args.edit.clone().into())
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertLadderInstructionArgs {
    program_index: usize,
    raw_y: u32,
    mnemonic: String,
    operands: Vec<String>,
}

fn schema_insert_ladder_instruction() -> Value {
    serde_json::to_value(schema_for!(InsertLadderInstructionArgs))
        .expect("JSON Schema is serializable")
}

fn example_insert_ladder_instruction() -> Value {
    json!({"program_index": 0, "raw_y": 0, "mnemonic": "<requested value>", "operands": []})
}

fn apply_insert_ladder_instruction(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: InsertLadderInstructionArgs = parse_args("insert_ladder_instruction", args)?;
    execute_insert_ladder_instruction(doc, &args)
}

fn execute_insert_ladder_instruction(
    doc: &mut XgwxDocument,
    args: &InsertLadderInstructionArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .ladder()?
                .insert_instruction(args.raw_y, &args.mnemonic, &args.operands)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertLadderComparisonArgs {
    program_index: usize,
    raw_y: u32,
    column: u8,
    mnemonic: String,
    operands: Vec<String>,
}

fn schema_insert_ladder_comparison() -> Value {
    serde_json::to_value(schema_for!(InsertLadderComparisonArgs))
        .expect("JSON Schema is serializable")
}

fn example_insert_ladder_comparison() -> Value {
    json!({"program_index": 0, "raw_y": 0, "column": 0, "mnemonic": "<requested value>", "operands": []})
}

fn apply_insert_ladder_comparison(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: InsertLadderComparisonArgs = parse_args("insert_ladder_comparison", args)?;
    execute_insert_ladder_comparison(doc, &args)
}

fn execute_insert_ladder_comparison(
    doc: &mut XgwxDocument,
    args: &InsertLadderComparisonArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program.ladder()?.insert_comparison(
                args.raw_y,
                args.column,
                &args.mnemonic,
                &args.operands,
            )
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteLadderComparisonArgs {
    program_index: usize,
    offset: usize,
    expected: String,
}

fn schema_delete_ladder_comparison() -> Value {
    serde_json::to_value(schema_for!(DeleteLadderComparisonArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_ladder_comparison() -> Value {
    json!({"program_index": 0, "offset": 0, "expected": "<current value from inspect_file>"})
}

fn apply_delete_ladder_comparison(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteLadderComparisonArgs = parse_args("delete_ladder_comparison", args)?;
    execute_delete_ladder_comparison(doc, &args)
}

fn execute_delete_ladder_comparison(
    doc: &mut XgwxDocument,
    args: &DeleteLadderComparisonArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .ladder()?
                .delete_comparison(args.offset, &args.expected)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteLadderInstructionArgs {
    program_index: usize,
    offset: usize,
    expected: String,
}

fn schema_delete_ladder_instruction() -> Value {
    serde_json::to_value(schema_for!(DeleteLadderInstructionArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_ladder_instruction() -> Value {
    json!({"program_index": 0, "offset": 0, "expected": "<current value from inspect_file>"})
}

fn apply_delete_ladder_instruction(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteLadderInstructionArgs = parse_args("delete_ladder_instruction", args)?;
    execute_delete_ladder_instruction(doc, &args)
}

fn execute_delete_ladder_instruction(
    doc: &mut XgwxDocument,
    args: &DeleteLadderInstructionArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .ladder()?
                .delete_instruction(args.offset, &args.expected)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct EditLadderCommentArgs {
    program_index: usize,
    edit: LadderCommentEdit,
}

fn schema_edit_ladder_comment() -> Value {
    serde_json::to_value(schema_for!(EditLadderCommentArgs)).expect("JSON Schema is serializable")
}

fn example_edit_ladder_comment() -> Value {
    json!({"program_index": 0, "edit": {"kind": "Rung", "raw_y": 0, "expected": null, "replacement": "<requested value>"}})
}

fn apply_edit_ladder_comment(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: EditLadderCommentArgs = parse_args("edit_ladder_comment", args)?;
    execute_edit_ladder_comment(doc, &args)
}

fn execute_edit_ladder_comment(
    doc: &mut XgwxDocument,
    args: &EditLadderCommentArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut selected_program| {
            selected_program
                .ladder()?
                .edit_comment(&args.edit.clone().into())
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteLadderRungCommentArgs {
    program_index: usize,
    raw_y: u32,
    expected: String,
}

fn schema_delete_ladder_rung_comment() -> Value {
    serde_json::to_value(schema_for!(DeleteLadderRungCommentArgs))
        .expect("JSON Schema is serializable")
}

fn example_delete_ladder_rung_comment() -> Value {
    json!({"program_index": 0, "raw_y": 0, "expected": "<current value from inspect_file>"})
}

fn apply_delete_ladder_rung_comment(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteLadderRungCommentArgs = parse_args("delete_ladder_rung_comment", args)?;
    execute_delete_ladder_rung_comment(doc, &args)
}

fn execute_delete_ladder_rung_comment(
    doc: &mut XgwxDocument,
    args: &DeleteLadderRungCommentArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut program| {
            program
                .ladder()?
                .delete_rung_comment(args.raw_y, &args.expected)
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct EditLadderBranchArgs {
    program_index: usize,
    edit: LadderBranchEdit,
}

fn schema_edit_ladder_branch() -> Value {
    serde_json::to_value(schema_for!(EditLadderBranchArgs)).expect("JSON Schema is serializable")
}

fn example_edit_ladder_branch() -> Value {
    json!({"program_index": 0, "edit": {"raw_y": 0, "boundary": 0, "expected": false, "present": false}})
}

fn apply_edit_ladder_branch(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: EditLadderBranchArgs = parse_args("edit_ladder_branch", args)?;
    execute_edit_ladder_branch(doc, &args)
}

fn execute_edit_ladder_branch(
    doc: &mut XgwxDocument,
    args: &EditLadderBranchArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut selected_program| {
            selected_program
                .ladder()?
                .edit_branch(&args.edit.clone().into())
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct InsertLadderRowArgs {
    program_index: usize,
    raw_y: u32,
}

fn schema_insert_ladder_row() -> Value {
    serde_json::to_value(schema_for!(InsertLadderRowArgs)).expect("JSON Schema is serializable")
}

fn example_insert_ladder_row() -> Value {
    json!({"program_index": 0, "raw_y": 0})
}

fn apply_insert_ladder_row(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: InsertLadderRowArgs = parse_args("insert_ladder_row", args)?;
    execute_insert_ladder_row(doc, &args)
}

fn execute_insert_ladder_row(
    doc: &mut XgwxDocument,
    args: &InsertLadderRowArgs,
) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|mut selected_program| selected_program.ladder()?.insert_row(args.raw_y))
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct MoveProgramArgs {
    from: usize,
    to: usize,
    expected_object_id: String,
    expected_target_id: String,
}

fn schema_move_program() -> Value {
    serde_json::to_value(schema_for!(MoveProgramArgs)).expect("JSON Schema is serializable")
}

fn example_move_program() -> Value {
    json!({"from": 0, "to": 0, "expected_object_id": "<ObjectId from inspect_file>", "expected_target_id": "<ObjectId from inspect_file>"})
}

fn apply_move_program(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: MoveProgramArgs = parse_args("move_program", args)?;
    execute_move_program(doc, &args)
}

fn execute_move_program(doc: &mut XgwxDocument, args: &MoveProgramArgs) -> Result<(), String> {
    {
        let target = doc
            .program()
            .get(args.to)
            .map_err(|error| error.to_string())?
            .summary()
            .object_id;
        doc.program_mut().get(args.from).and_then(|mut program| {
            if args.expected_object_id.is_empty()
                || program.object_id() != args.expected_object_id
                || args.expected_target_id.is_empty()
                || target.as_deref() != Some(&args.expected_target_id)
            {
                return Err(xgwx::XgwxError::TextProgramEdit(
                    "program identity changed; reload before applying".into(),
                ));
            }
            program.move_to(args.to)
        })
    }
    .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DeleteProgramArgs {
    program_index: usize,
    expected_object_id: String,
}

fn schema_delete_program() -> Value {
    serde_json::to_value(schema_for!(DeleteProgramArgs)).expect("JSON Schema is serializable")
}

fn example_delete_program() -> Value {
    json!({"program_index": 0, "expected_object_id": "<ObjectId from inspect_file>"})
}

fn apply_delete_program(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: DeleteProgramArgs = parse_args("delete_program", args)?;
    execute_delete_program(doc, &args)
}

fn execute_delete_program(doc: &mut XgwxDocument, args: &DeleteProgramArgs) -> Result<(), String> {
    doc.program_mut()
        .get(args.program_index)
        .and_then(|program| {
            if args.expected_object_id.is_empty() || program.object_id() != args.expected_object_id
            {
                return Err(xgwx::XgwxError::TextProgramEdit(
                    "program identity changed; reload before applying".into(),
                ));
            }
            program.delete()
        })
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct CreateProgramArgs {
    patch: NewProgram,
}

fn schema_create_program() -> Value {
    serde_json::to_value(schema_for!(CreateProgramArgs)).expect("JSON Schema is serializable")
}

fn example_create_program() -> Value {
    json!({"patch": {"name": "<requested value>", "language": "LD", "object_id": "12345678-1234-1234-1234-123456789abc", "symbol_id": "87654321-4321-4321-4321-cba987654321"}})
}

fn apply_create_program(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: CreateProgramArgs = parse_args("create_program", args)?;
    execute_create_program(doc, &args)
}

fn execute_create_program(doc: &mut XgwxDocument, args: &CreateProgramArgs) -> Result<(), String> {
    doc.program_mut()
        .create(&args.patch.clone().into())
        .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct EditCnetSettingsArgs {
    patch: CnetSettingsPatch,
}

fn schema_edit_cnet_settings() -> Value {
    serde_json::to_value(schema_for!(EditCnetSettingsArgs)).expect("JSON Schema is serializable")
}

fn example_edit_cnet_settings() -> Value {
    json!({"patch": {"base": 0, "slot": 0, "changes": []}})
}

fn apply_edit_cnet_settings(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: EditCnetSettingsArgs = parse_args("edit_cnet_settings", args)?;
    execute_edit_cnet_settings(doc, &args)
}

fn execute_edit_cnet_settings(
    doc: &mut XgwxDocument,
    args: &EditCnetSettingsArgs,
) -> Result<(), String> {
    (|| {
        let patch: xgwx::CnetSettingsPatch = args.patch.clone().into();

        doc.network_mut()
            .module_at(patch.base, patch.slot)?
            .cnet()?
            .edit_settings(&patch.changes)
    })()
    .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct EditSfcEntityArgs {
    patch: SfcEntityPatch,
}

fn schema_edit_sfc_entity() -> Value {
    serde_json::to_value(schema_for!(EditSfcEntityArgs)).expect("JSON Schema is serializable")
}

fn example_edit_sfc_entity() -> Value {
    json!({"patch": {"program_index": 0, "block_index": 0, "entity_index": 0, "expected_type": 0, "expected_row": 0, "expected_column": 0, "field": "<supported field or operation from inspect_file and description>", "expected_value": "<current value from inspect_file>", "replacement": "<requested value>"}})
}

fn apply_edit_sfc_entity(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: EditSfcEntityArgs = parse_args("edit_sfc_entity", args)?;
    execute_edit_sfc_entity(doc, &args)
}

fn execute_edit_sfc_entity(doc: &mut XgwxDocument, args: &EditSfcEntityArgs) -> Result<(), String> {
    {
        let patch: xgwx::SfcEntityPatch = args.patch.clone().into();
        doc.program_mut()
            .get(patch.program_index)
            .and_then(|mut program| {
                program.sfc()?.edit_entity(&xgwx::SfcEntityEdit {
                    block_index: patch.block_index,
                    entity_index: patch.entity_index,
                    expected_type: patch.expected_type,
                    expected_row: patch.expected_row,
                    expected_column: patch.expected_column,
                    field: patch.field,
                    expected_value: patch.expected_value,
                    replacement: patch.replacement,
                })
            })
    }
    .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ReplaceSfcSequenceArgs {
    patch: SfcSequencePatch,
}

fn schema_replace_sfc_sequence() -> Value {
    serde_json::to_value(schema_for!(ReplaceSfcSequenceArgs)).expect("JSON Schema is serializable")
}

fn example_replace_sfc_sequence() -> Value {
    json!({"patch": {"program_index": 0, "block_index": 0, "expected_entities": [], "rows": [], "expected_rows": null}})
}

fn apply_replace_sfc_sequence(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: ReplaceSfcSequenceArgs = parse_args("replace_sfc_sequence", args)?;
    execute_replace_sfc_sequence(doc, &args)
}

fn execute_replace_sfc_sequence(
    doc: &mut XgwxDocument,
    args: &ReplaceSfcSequenceArgs,
) -> Result<(), String> {
    {
        let patch: xgwx::SfcSequencePatch = args.patch.clone().into();
        doc.program_mut()
            .get(patch.program_index)
            .and_then(|mut program| {
                program.sfc()?.replace_sequence(&xgwx::SfcSequenceEdit {
                    block_index: patch.block_index,
                    expected_entities: patch.expected_entities,
                    rows: patch.rows,
                    expected_rows: patch.expected_rows,
                })
            })
    }
    .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct EditSfcVariableArgs {
    patch: SfcVariablePatch,
}

fn schema_edit_sfc_variable() -> Value {
    serde_json::to_value(schema_for!(EditSfcVariableArgs)).expect("JSON Schema is serializable")
}

fn example_edit_sfc_variable() -> Value {
    json!({"patch": {"program_index": 0, "expected_variables": [], "name": "<requested value>", "data_type": "BOOL", "description": "<requested value>", "remove": false, "declaration": null, "update": false}})
}

fn apply_edit_sfc_variable(doc: &mut XgwxDocument, args: Value) -> Result<(), String> {
    let args: EditSfcVariableArgs = parse_args("edit_sfc_variable", args)?;
    execute_edit_sfc_variable(doc, &args)
}

fn execute_edit_sfc_variable(
    doc: &mut XgwxDocument,
    args: &EditSfcVariableArgs,
) -> Result<(), String> {
    {
        let patch: xgwx::SfcVariablePatch = args.patch.clone().into();
        doc.program_mut()
            .get(patch.program_index)
            .and_then(|mut program| {
                program
                    .variable_mut()
                    .edit_declaration(&xgwx::ProgramVariablePatch {
                        expected_variables: patch.expected_variables,
                        name: patch.name,
                        data_type: patch.data_type,
                        description: patch.description,
                        remove: patch.remove,
                        declaration: patch.declaration,
                        update: patch.update,
                    })
            })
    }
    .map_err(|error| error.to_string())
}

pub const OPERATIONS: &[OperationDefinition] = &[
    OperationDefinition {
        name: "edit_vendor_il",
        subsystem: "text_programs",
        description: "Replace supported XGK IL by rebuilding native ladder records. Requires current ObjectId and exact exported IL. Rejects branch IL, Auto-allocation and unsupported topology; this replaces the whole selected ladder payload.",
        adapter: "edit_vendor_il",
        schema: schema_edit_vendor_il,
        example: example_edit_vendor_il,
        apply: apply_edit_vendor_il,
    },
    OperationDefinition {
        name: "edit_text_program",
        subsystem: "text_programs",
        description: "Replace standalone ST or IEC IL source using exact identity, language and expected-source checks. Preserve unsupported-layout, UTF-16 length and NUL guards. Does not compile source.",
        adapter: "edit_text_program",
        schema: schema_edit_text_program,
        example: example_edit_text_program,
        apply: apply_edit_text_program,
    },
    OperationDefinition {
        name: "edit_text_variable",
        subsystem: "text_programs",
        description: "Add, update or remove standalone ST/IL declarations using a complete expected-variable snapshot and program ObjectId. Preserve IEC type, mapped/system declaration and source-reference guards.",
        adapter: "edit_text_variable",
        schema: schema_edit_text_variable,
        example: example_edit_text_variable,
        apply: apply_edit_text_variable,
    },
    OperationDefinition {
        name: "edit_browser_hardware",
        subsystem: "hardware",
        description: "Apply one supported hardware field transactionally. The XML outside the\nexact selected attribute is compared byte for byte before commit.",
        adapter: "edit_browser_hardware",
        schema: schema_edit_browser_hardware,
        example: example_edit_browser_hardware,
        apply: apply_edit_browser_hardware,
    },
    OperationDefinition {
        name: "edit_browser_network",
        subsystem: "networks",
        description: "Reuse metadata writers; commit only one exact XML attribute delta.",
        adapter: "edit_browser_network",
        schema: schema_edit_browser_network,
        example: example_edit_browser_network,
        apply: apply_edit_browser_network,
    },
    OperationDefinition {
        name: "edit_fenet_field",
        subsystem: "networks",
        description: "Change a captured FEnet setting without altering module identity or payloads.",
        adapter: "edit_fenet_field",
        schema: schema_edit_fenet_field,
        example: example_edit_fenet_field,
        apply: apply_edit_fenet_field,
    },
    OperationDefinition {
        name: "select_cpu",
        subsystem: "hardware",
        description: "Select the CPU model stored by the primary project configuration.\n\nThis updates the authoritative `<Configuration Type>` value while\npreserving the existing basic parameters, programs, hardware, and\nnetwork configuration. XGK changes require retained hardware to fit.\nCaptured XGI model changes support validated SFC, ST and IL projects with default\nparameters and empty I/O tables. Other model changes require\na migration and are rejected. Selecting the current type is a no-op.",
        adapter: "select_cpu",
        schema: schema_select_cpu,
        example: example_select_cpu,
        apply: apply_select_cpu,
    },
    OperationDefinition {
        name: "set_base_slot_count",
        subsystem: "hardware",
        description: "Change an existing XGK base's physical slot count without removing modules.\nNative XG5000 choices are 4, 6, 8, 10 and 12. CPU limits, ambiguous\nbases and modules extending beyond the requested size are rejected.",
        adapter: "set_base_slot_count",
        schema: schema_set_base_slot_count,
        example: example_set_base_slot_count,
        apply: apply_set_base_slot_count,
    },
    OperationDefinition {
        name: "insert_module",
        subsystem: "hardware",
        description: "Insert a catalog module into an empty physical base slot.\n\nThe module receives the latest-stable catalog defaults and an empty\ncomment. The operation fails without mutating the document when the\nbase is absent, the module does not fit, or any occupied slot overlaps.",
        adapter: "insert_module",
        schema: schema_insert_module,
        example: example_insert_module,
        apply: apply_insert_module,
    },
    OperationDefinition {
        name: "delete_module",
        subsystem: "hardware",
        description: "Delete one module selected by its unique base and slot.\n\nThe containing base and every other XML node remain unchanged. The\noperation fails without mutating the document when the target is absent\nor ambiguous.",
        adapter: "delete_module",
        schema: schema_delete_module,
        example: example_delete_module,
        apply: apply_delete_module,
    },
    OperationDefinition {
        name: "select_module",
        subsystem: "hardware",
        description: "Select a module model from the embedded latest-stable XGK catalog.\n\nThe target base, slot, and comment are preserved. `Id`, `SubType`,\n`Name`, and `Details` are replaced atomically with the catalog defaults.\nCaptured network-module companion records are synchronized at the same\ntime for supported communication models.",
        adapter: "select_module",
        schema: schema_select_module,
        example: example_select_module,
        apply: apply_select_module,
    },
    OperationDefinition {
        name: "set_module_option",
        subsystem: "hardware",
        description: "Set one verified dropdown option in a module's `Details` payload.\n\n`index` is zero for module-wide options and selects the channel or group\nfor scoped options. Values not listed in the catalog are rejected.",
        adapter: "set_module_option",
        schema: schema_set_module_option,
        example: example_set_module_option,
        apply: apply_set_module_option,
    },
    OperationDefinition {
        name: "update_module",
        subsystem: "hardware",
        description: "Update one module selected by its unique base and slot.\n\nOnly the requested XML attribute values are replaced. Element order,\nwhitespace, unknown elements, binary payloads, and the workspace trailer\nremain untouched.",
        adapter: "update_module",
        schema: schema_update_module,
        example: example_update_module,
        apply: apply_update_module,
    },
    OperationDefinition {
        name: "set_module_input_filter",
        subsystem: "hardware",
        description: "Set the first `Details` byte for an XGI-D24A/B digital input module.",
        adapter: "set_module_input_filter",
        schema: schema_set_module_input_filter,
        example: example_set_module_input_filter,
        apply: apply_set_module_input_filter,
    },
    OperationDefinition {
        name: "edit_browser_iec",
        subsystem: "iec_ladder",
        description: "Bounded browser edit: one existing IEC kind byte or one operand string.\nAll XML outside the selected ProgramData and all other payload bytes\nare verified unchanged before the candidate is committed.",
        adapter: "edit_browser_iec",
        schema: schema_edit_browser_iec,
        example: example_edit_browser_iec,
        apply: apply_edit_browser_iec,
    },
    OperationDefinition {
        name: "rename_project",
        subsystem: "programs",
        description: "Rename the root project using its inspected expected name; preserve other metadata and payloads.",
        adapter: "rename_project",
        schema: schema_rename_project,
        example: example_rename_project,
        apply: apply_rename_project,
    },
    OperationDefinition {
        name: "update_program",
        subsystem: "programs",
        description: "Update editable XML metadata for one program selected by document order.",
        adapter: "update_program",
        schema: schema_update_program,
        example: example_update_program,
        apply: apply_update_program,
    },
    OperationDefinition {
        name: "update_network",
        subsystem: "networks",
        description: "Update editable metadata for one network selected by document order.",
        adapter: "update_network",
        schema: schema_update_network,
        example: example_update_network,
        apply: apply_update_network,
    },
    OperationDefinition {
        name: "update_network_module",
        subsystem: "networks",
        description: "Update user-facing metadata for the unique network module at a base and\nslot. Hardware identity and network protocol fields remain unchanged.",
        adapter: "update_network_module",
        schema: schema_update_network_module,
        example: example_update_network_module,
        apply: apply_update_network_module,
    },
    OperationDefinition {
        name: "generate_io_variables",
        subsystem: "symbols",
        description: "Generate digital I/O variables, replacing duplicate names or addresses.\nUnrelated symbol records and every program payload remain untouched.",
        adapter: "generate_io_variables",
        schema: schema_generate_io_variables,
        example: example_generate_io_variables,
        apply: apply_generate_io_variables,
    },
    OperationDefinition {
        name: "update_variable",
        subsystem: "symbols",
        description: "Update one global variable, rebuilding bounded name/comment strings.",
        adapter: "update_variable",
        schema: schema_update_variable,
        example: example_update_variable,
        apply: apply_update_variable,
    },
    OperationDefinition {
        name: "update_iec_local_symbol_address",
        subsystem: "symbols",
        description: "Change a captured IEC local bit or numeric memory mapping.\nPB50 stores the address text, bit offset, and allocation width.\nExisting mappings keep their address area. Clearing or assigning a mapping\nalso updates its storage class and allocation metadata.",
        adapter: "update_iec_local_symbol_address",
        schema: schema_update_iec_local_symbol_address,
        example: example_update_iec_local_symbol_address,
        apply: apply_update_iec_local_symbol_address,
    },
    OperationDefinition {
        name: "rename_iec_local_symbol",
        subsystem: "symbols",
        description: "Rename one IEC program-local symbol and every captured LD operand or\nmatching function-instance marker. Reject unknown reference records so\nthe edit stays atomic.",
        adapter: "rename_iec_local_symbol",
        schema: schema_rename_iec_local_symbol,
        example: example_rename_iec_local_symbol,
        apply: apply_rename_iec_local_symbol,
    },
    OperationDefinition {
        name: "duplicate_iec_ld_function_instance",
        subsystem: "iec_ladder",
        description: "Give one captured IEC function block a separate instance by cloning its\nPB50 instance declaration, assigning a new automatic-memory range, and changing only that block's\ninstance marker. This is useful after copying a network containing a\nstateful function such as R_TRIG.",
        adapter: "duplicate_iec_ld_function_instance",
        schema: schema_duplicate_iec_ld_function_instance,
        example: example_duplicate_iec_ld_function_instance,
        apply: apply_duplicate_iec_ld_function_instance,
    },
    OperationDefinition {
        name: "copy_iec_instance_declaration_to_program",
        subsystem: "symbols",
        description: "Clone one captured IEC function-instance declaration into another\nprogram, assigning an unused automatic-memory range in that program.\nThe source must contain a block using the declaration's function type.",
        adapter: "copy_iec_instance_declaration_to_program",
        schema: schema_copy_iec_instance_declaration_to_program,
        example: example_copy_iec_instance_declaration_to_program,
        apply: apply_copy_iec_instance_declaration_to_program,
    },
    OperationDefinition {
        name: "update_iec_local_symbol_description",
        subsystem: "symbols",
        description: "Edit one captured IEC program-local symbol description. The PB50\ndescription is a length-prefixed UTF-16 field, independent of program\noperands and allocation metadata.",
        adapter: "update_iec_local_symbol_description",
        schema: schema_update_iec_local_symbol_description,
        example: example_update_iec_local_symbol_description,
        apply: apply_update_iec_local_symbol_description,
    },
    OperationDefinition {
        name: "insert_iec_local_symbol",
        subsystem: "symbols",
        description: "Insert an unallocated primitive variable in a captured IEC program's\nlocal PB50 table. XG5000 sorts this table by case-insensitive name.",
        adapter: "insert_iec_local_symbol",
        schema: schema_insert_iec_local_symbol,
        example: example_insert_iec_local_symbol,
        apply: apply_insert_iec_local_symbol,
    },
    OperationDefinition {
        name: "delete_iec_local_symbol",
        subsystem: "symbols",
        description: "Remove an IEC program-local PB50 record when the program does not\ncontain a marker string referring to that symbol.",
        adapter: "delete_iec_local_symbol",
        schema: schema_delete_iec_local_symbol,
        example: example_delete_iec_local_symbol,
        apply: apply_delete_iec_local_symbol,
    },
    OperationDefinition {
        name: "update_iec_local_symbol_type",
        subsystem: "symbols",
        description: "Change a program-local automatic variable's primitive IEC type.\nXG5000 clears its allocation on a type change, even when the new\ntype has the same storage width. Program references are retained.",
        adapter: "update_iec_local_symbol_type",
        schema: schema_update_iec_local_symbol_type,
        example: example_update_iec_local_symbol_type,
        apply: apply_update_iec_local_symbol_type,
    },
    OperationDefinition {
        name: "update_ladder_cell_text",
        subsystem: "xgk_ladder",
        description: "Replace decoded ladder text. Recognized application instructions allow\nvariable-length operands and catalog-backed mnemonic changes, synchronizing\nopcodes, decomposed strings, occupied cells and wires. Other records retain the\nsame-length restriction to preserve their undecoded binary layout.",
        adapter: "update_ladder_cell_text",
        schema: schema_update_ladder_cell_text,
        example: example_update_ladder_cell_text,
        apply: apply_update_ladder_cell_text,
    },
    OperationDefinition {
        name: "update_iec_ld_comment",
        subsystem: "iec_ladder",
        description: "Replace the text of a captured IEC LD comment. XG5000 stores its UTF-16\nlength in the marker byte; the surrounding record has no length field.\nSymbol and operand strings are deliberately excluded from this operation.",
        adapter: "update_iec_ld_comment",
        schema: schema_update_iec_ld_comment,
        example: example_update_iec_ld_comment,
        apply: apply_update_iec_ld_comment,
    },
    OperationDefinition {
        name: "update_iec_ld_rising_contact_operand",
        subsystem: "iec_ladder",
        description: "Replace the variable reference of a captured IEC LD rising-edge\ncontact (`FF 08`).",
        adapter: "update_iec_ld_rising_contact_operand",
        schema: schema_update_iec_ld_rising_contact_operand,
        example: example_update_iec_ld_rising_contact_operand,
        apply: apply_update_iec_ld_rising_contact_operand,
    },
    OperationDefinition {
        name: "update_iec_ld_element_operand",
        subsystem: "iec_ladder",
        description: "Replace an operand in a captured IEC LD contact or output coil record.\nThis accepts only the record codes identified by `iec_ld::element_operands`.\nClassified replacements must be BOOL; coil destinations must be writable.\nXG5000 must still validate unclassified expressions.",
        adapter: "update_iec_ld_element_operand",
        schema: schema_update_iec_ld_element_operand,
        example: example_update_iec_ld_element_operand,
        apply: apply_update_iec_ld_element_operand,
    },
    OperationDefinition {
        name: "update_iec_ld_contact_kind",
        subsystem: "iec_ladder",
        description: "Change a captured IEC contact among the six addressed contact kinds.",
        adapter: "update_iec_ld_contact_kind",
        schema: schema_update_iec_ld_contact_kind,
        example: example_update_iec_ld_contact_kind,
        apply: apply_update_iec_ld_contact_kind,
    },
    OperationDefinition {
        name: "update_iec_ld_coil_kind",
        subsystem: "iec_ladder",
        description: "Change a captured IEC coil among output, inverse, set, reset, rising,\nand falling kinds while preserving its operand and position.",
        adapter: "update_iec_ld_coil_kind",
        schema: schema_update_iec_ld_coil_kind,
        example: example_update_iec_ld_coil_kind,
        apply: apply_update_iec_ld_coil_kind,
    },
    OperationDefinition {
        name: "edit_iec_ld_branch_segment",
        subsystem: "iec_ladder",
        description: "Add or remove one captured IEC LD vertical branch segment between two\nadjacent rows that already belong to the same native row group.\n\nRemoval preserves another connection between the rows, or deletes a\nsupported lower branch row and shifts later rows as in native Delete Line.\nGeneral group splitting remains unsupported.\nAddition is limited to rows without comments or function records and\npreserves the group's existing row envelope.",
        adapter: "edit_iec_ld_branch_segment",
        schema: schema_edit_iec_ld_branch_segment,
        example: example_edit_iec_ld_branch_segment,
        apply: apply_edit_iec_ld_branch_segment,
    },
    OperationDefinition {
        name: "edit_iec_ld_vertical_wire",
        subsystem: "iec_ladder",
        description: "Toggle a paired vertical wire while retaining both row envelopes and\nevery other record. Open electrical endpoints are valid intermediate\nlayouts; typed function bindings must remain intact. Addition joins two\nexisting electrical points in the same group, including a deleted gap.",
        adapter: "edit_iec_ld_vertical_wire",
        schema: schema_edit_iec_ld_vertical_wire,
        example: example_edit_iec_ld_vertical_wire,
        apply: apply_edit_iec_ld_vertical_wire,
    },
    OperationDefinition {
        name: "connect_iec_ld_groups",
        subsystem: "iec_ladder",
        description: "Join consecutive stored IEC groups through a vertical wire between\ntheir adjacent boundary rows. Existing row and function records remain\nunchanged; the group envelopes are rebuilt and subsequent groups renumbered.\nThe joined network is disabled if either source group is disabled.",
        adapter: "connect_iec_ld_groups",
        schema: schema_connect_iec_ld_groups,
        example: example_connect_iec_ld_groups,
        apply: apply_connect_iec_ld_groups,
    },
    OperationDefinition {
        name: "extend_iec_ld_vertical_wire",
        subsystem: "iec_ladder",
        description: "Extend a stored group's final row into the adjacent implicit blank row.\nThe new row contains the vertical endpoint only; subsequent element\nplacement can complete the circuit. Existing rows are not shifted.",
        adapter: "extend_iec_ld_vertical_wire",
        schema: schema_extend_iec_ld_vertical_wire,
        example: example_extend_iec_ld_vertical_wire,
        apply: apply_extend_iec_ld_vertical_wire,
    },
    OperationDefinition {
        name: "split_iec_ld_group",
        subsystem: "iec_ladder",
        description: "Separate a stored IEC group at an adjacent row boundary with no\ncrossing wire, function body or function reference. Coordinates and\nall row-local records are preserved. Both groups inherit the current\nnetwork execution setting.",
        adapter: "split_iec_ld_group",
        schema: schema_split_iec_ld_group,
        example: example_split_iec_ld_group,
        apply: apply_split_iec_ld_group,
    },
    OperationDefinition {
        name: "insert_iec_ld_blank_row",
        subsystem: "iec_ladder",
        description: "Insert one implicit blank IEC LD row after `after_row_index`, matching\nXG5000 Ctrl+L on a blank row boundary. The payload does not gain a row\nrecord; later row envelopes and every decoded Y coordinate move down by\none four-unit grid row.",
        adapter: "insert_iec_ld_blank_row",
        schema: schema_insert_iec_ld_blank_row,
        example: example_insert_iec_ld_blank_row,
        apply: apply_insert_iec_ld_blank_row,
    },
    OperationDefinition {
        name: "insert_iec_ld_parallel_contact",
        subsystem: "iec_ladder",
        description: "Compatibility wrapper for a normally open parallel contact.",
        adapter: "insert_iec_ld_parallel_contact_kind",
        schema: schema_insert_iec_ld_parallel_contact,
        example: example_insert_iec_ld_parallel_contact,
        apply: apply_insert_iec_ld_parallel_contact,
    },
    OperationDefinition {
        name: "insert_iec_ld_parallel_contact_kind",
        subsystem: "iec_ladder",
        description: "Add an addressed contact in parallel with the leading contact of a\ncaptured one-row addressed contact/wire/output-coil rung. The operation creates\na second row, connects it at the contact's right edge, and shifts later\nrows down. The complete two-row network is validated before writing.",
        adapter: "insert_iec_ld_parallel_contact_kind",
        schema: schema_insert_iec_ld_parallel_contact_kind,
        example: example_insert_iec_ld_parallel_contact_kind,
        apply: apply_insert_iec_ld_parallel_contact_kind,
    },
    OperationDefinition {
        name: "delete_iec_ld_blank_row",
        subsystem: "iec_ladder",
        description: "Delete one implicit IEC LD blank row and move every later decoded Y\ncoordinate up by one row, matching XG5000 Ctrl+D on an empty line.",
        adapter: "delete_iec_ld_blank_row",
        schema: schema_delete_iec_ld_blank_row,
        example: example_delete_iec_ld_blank_row,
        apply: apply_delete_iec_ld_blank_row,
    },
    OperationDefinition {
        name: "insert_iec_ld_linear_rung",
        subsystem: "iec_ladder",
        description: "Create the compatibility normally-open-contact to output-coil IEC LD\nrung. See [`Self::insert_iec_ld_rung`] for the other decoded kinds.",
        adapter: "insert_iec_ld_rung",
        schema: schema_insert_iec_ld_linear_rung,
        example: example_insert_iec_ld_linear_rung,
        apply: apply_insert_iec_ld_linear_rung,
    },
    OperationDefinition {
        name: "insert_iec_ld_rung",
        subsystem: "iec_ladder",
        description: "Create a native three-record IEC LD rung in one implicit blank row.\nThe captured shape is an addressed BOOL contact at the left rail, a\nlong wire, and a BOOL coil at the right rail. A new single-row group is\ninserted without moving any existing row, including after the last\nstored network, extending the stored row range across sparse gaps when necessary.",
        adapter: "insert_iec_ld_rung",
        schema: schema_insert_iec_ld_rung,
        example: example_insert_iec_ld_rung,
        apply: apply_insert_iec_ld_rung,
    },
    OperationDefinition {
        name: "insert_iec_ld_single_element",
        subsystem: "iec_ladder",
        description: "Insert one addressed contact or coil into an unoccupied IEC grid cell.\nNo other element or wire is created. The resulting row may be incomplete.",
        adapter: "insert_iec_ld_single_element",
        schema: schema_insert_iec_ld_single_element,
        example: example_insert_iec_ld_single_element,
        apply: apply_insert_iec_ld_single_element,
    },
    OperationDefinition {
        name: "delete_iec_ld_linear_rung",
        subsystem: "iec_ladder",
        description: "Delete the compatibility normally-open-contact to output-coil rung.\nSee [`Self::delete_iec_ld_rung`] for the other decoded kinds.",
        adapter: "delete_iec_ld_rung",
        schema: schema_delete_iec_ld_linear_rung,
        example: example_delete_iec_ld_linear_rung,
        apply: apply_delete_iec_ld_linear_rung,
    },
    OperationDefinition {
        name: "delete_iec_ld_rung",
        subsystem: "iec_ladder",
        description: "Delete one exact addressed-contact, wire, and coil single-row group,\nrestoring its row to an implicit blank gap without moving other rows.",
        adapter: "delete_iec_ld_rung",
        schema: schema_delete_iec_ld_rung,
        example: example_delete_iec_ld_rung,
        apply: apply_delete_iec_ld_rung,
    },
    OperationDefinition {
        name: "delete_iec_ld_terminal_coil",
        subsystem: "iec_ladder",
        description: "Delete the terminal wire and coil from a captured IEC row.\nKeep its leading contact or incoming branch endpoint as the sole record.",
        adapter: "delete_iec_ld_terminal_coil",
        schema: schema_delete_iec_ld_terminal_coil,
        example: example_delete_iec_ld_terminal_coil,
        apply: apply_delete_iec_ld_terminal_coil,
    },
    OperationDefinition {
        name: "insert_iec_ld_terminal_coil",
        subsystem: "iec_ladder",
        description: "Finish a contact-only one-row IEC rung with a native long wire and coil.\nThe row must have the exact shape left by terminal-coil deletion.",
        adapter: "insert_iec_ld_terminal_coil",
        schema: schema_insert_iec_ld_terminal_coil,
        example: example_insert_iec_ld_terminal_coil,
        apply: apply_insert_iec_ld_terminal_coil,
    },
    OperationDefinition {
        name: "delete_iec_ld_simple_row",
        subsystem: "iec_ladder",
        description: "Delete one exact simple IEC LD rung and close its row, matching\nXG5000 Ctrl+D on an occupied line. Both edits are checked before the\ndocument is changed.",
        adapter: "delete_iec_ld_simple_row",
        schema: schema_delete_iec_ld_simple_row,
        example: example_delete_iec_ld_simple_row,
        apply: apply_delete_iec_ld_simple_row,
    },
    OperationDefinition {
        name: "delete_iec_ld_branch_top_row",
        subsystem: "iec_ladder",
        description: "Delete the upper line of a captured two-row IEC branch. The lower\ncontact-only line remains and all later rows move up, as in XG5000\nCtrl+D on the upper branch line.",
        adapter: "delete_iec_ld_branch_top_row",
        schema: schema_delete_iec_ld_branch_top_row,
        example: example_delete_iec_ld_branch_top_row,
        apply: apply_delete_iec_ld_branch_top_row,
    },
    OperationDefinition {
        name: "delete_iec_ld_nested_contact_branch_row",
        subsystem: "iec_ladder",
        description: "Delete a contact-only middle line from a nested IEC branch, reconnecting\nthe outer x3 branch and removing the two branches local to that line.",
        adapter: "delete_iec_ld_nested_contact_branch_row",
        schema: schema_delete_iec_ld_nested_contact_branch_row,
        example: example_delete_iec_ld_nested_contact_branch_row,
        apply: apply_delete_iec_ld_nested_contact_branch_row,
    },
    OperationDefinition {
        name: "delete_iec_ld_chained_contact_branch_row",
        subsystem: "iec_ladder",
        description: "Delete a contact-only middle line in a chain of x3 branches. The\nadjacent vertical segments become one segment across the removed line.",
        adapter: "delete_iec_ld_chained_contact_branch_row",
        schema: schema_delete_iec_ld_chained_contact_branch_row,
        example: example_delete_iec_ld_chained_contact_branch_row,
        apply: apply_delete_iec_ld_chained_contact_branch_row,
    },
    OperationDefinition {
        name: "delete_iec_ld_empty_branch_row",
        subsystem: "iec_ladder",
        description: "Delete a middle IEC row that only carries an incoming and outgoing\nvertical branch. The two segments are joined across the row gap.",
        adapter: "delete_iec_ld_empty_branch_row",
        schema: schema_delete_iec_ld_empty_branch_row,
        example: example_delete_iec_ld_empty_branch_row,
        apply: apply_delete_iec_ld_empty_branch_row,
    },
    OperationDefinition {
        name: "delete_iec_ld_ff_branch_output_row",
        subsystem: "iec_ladder",
        description: "Reproduce XG5000 Delete Line on the lower output row of the captured\ntwo-row FF branch. Native deletion also removes the connected FF block\nand its output reference from the upper row.",
        adapter: "delete_iec_ld_ff_branch_output_row",
        schema: schema_delete_iec_ld_ff_branch_output_row,
        example: example_delete_iec_ld_ff_branch_output_row,
        apply: apply_delete_iec_ld_ff_branch_output_row,
    },
    OperationDefinition {
        name: "insert_iec_ld_no_contact",
        subsystem: "iec_ladder",
        description: "Insert a normally open contact into a captured linear IEC LD wire.\nThis compatibility wrapper delegates to [`Self::insert_iec_ld_contact`].",
        adapter: "insert_iec_ld_contact",
        schema: schema_insert_iec_ld_no_contact,
        example: example_insert_iec_ld_no_contact,
        apply: apply_insert_iec_ld_no_contact,
    },
    OperationDefinition {
        name: "insert_iec_ld_leading_contact",
        subsystem: "iec_ladder",
        description: "Insert an addressed contact into the empty x1 cell of a captured\ntwo-row IEC branch upper line, matching XG5000 F3/F4 insertion.",
        adapter: "insert_iec_ld_leading_contact",
        schema: schema_insert_iec_ld_leading_contact,
        example: example_insert_iec_ld_leading_contact,
        apply: apply_insert_iec_ld_leading_contact,
    },
    OperationDefinition {
        name: "insert_iec_ld_contact",
        subsystem: "iec_ladder",
        description: "Insert one of the six addressed IEC contact kinds into a captured long\nwire. The record shape follows native XG5000 insertion and contact-kind\ncaptures. The operand must resolve to a BOOL symbol or device address.",
        adapter: "insert_iec_ld_contact",
        schema: schema_insert_iec_ld_contact,
        example: example_insert_iec_ld_contact,
        apply: apply_insert_iec_ld_contact,
    },
    OperationDefinition {
        name: "insert_iec_ld_short_wire_contact",
        subsystem: "iec_ladder",
        description: "Replace one framed IEC short wire with an addressed contact. XG5000 F3\npreserves the row count and surrounding records for this cell edit.",
        adapter: "insert_iec_ld_short_wire_contact",
        schema: schema_insert_iec_ld_short_wire_contact,
        example: example_insert_iec_ld_short_wire_contact,
        apply: apply_insert_iec_ld_short_wire_contact,
    },
    OperationDefinition {
        name: "delete_iec_ld_contact",
        subsystem: "iec_ladder",
        description: "Delete any decoded contact between two captured wire fragments, checking\nits expected kind before applying native XG5000 Delete semantics.",
        adapter: "delete_iec_ld_contact",
        schema: schema_delete_iec_ld_contact,
        example: example_delete_iec_ld_contact,
        apply: apply_delete_iec_ld_contact,
    },
    OperationDefinition {
        name: "delete_iec_ld_no_contact",
        subsystem: "iec_ladder",
        description: "Delete a decoded contact between two captured wire fragments.\nNative XG5000 Delete leaves both fragments in place and decrements the\nrow record count. Retained for compatibility with the original NO/NC API.",
        adapter: "delete_iec_ld_no_contact",
        schema: schema_delete_iec_ld_no_contact,
        example: example_delete_iec_ld_no_contact,
        apply: apply_delete_iec_ld_no_contact,
    },
    OperationDefinition {
        name: "delete_iec_ld_contact_cell",
        subsystem: "iec_ladder",
        description: "Delete any decoded contact with native XG5000 Cell Delete semantics,\nchecking its expected kind before applying the edit.",
        adapter: "delete_iec_ld_contact_cell",
        schema: schema_delete_iec_ld_contact_cell,
        example: example_delete_iec_ld_contact_cell,
        apply: apply_delete_iec_ld_contact_cell,
    },
    OperationDefinition {
        name: "delete_iec_ld_no_contact_cell",
        subsystem: "iec_ladder",
        description: "Delete a decoded contact with native XG5000 Cell Delete semantics.\nA leading branch contact moves the next contact to x1 and puts a short\nwire at x4. Contacts between long wires shift later cells left while\nthe output coil stays fixed. Retained for API compatibility.",
        adapter: "delete_iec_ld_no_contact_cell",
        schema: schema_delete_iec_ld_no_contact_cell,
        example: example_delete_iec_ld_no_contact_cell,
        apply: apply_delete_iec_ld_no_contact_cell,
    },
    OperationDefinition {
        name: "delete_iec_ld_horizontal_wire",
        subsystem: "iec_ladder",
        description: "Remove a captured one-cell `FF 01` wire between two long wires.\nThe resulting gap must be recognized by the guarded wire repair path.",
        adapter: "delete_iec_ld_horizontal_wire",
        schema: schema_delete_iec_ld_horizontal_wire,
        example: example_delete_iec_ld_horizontal_wire,
        apply: apply_delete_iec_ld_horizontal_wire,
    },
    OperationDefinition {
        name: "delete_ladder_horizontal_wire",
        subsystem: "xgk_ladder",
        description: "Delete an exact horizontal XGK wire without removing its row or elements.",
        adapter: "delete_ladder_horizontal_wire",
        schema: schema_delete_ladder_horizontal_wire,
        example: example_delete_ladder_horizontal_wire,
        apply: apply_delete_ladder_horizontal_wire,
    },
    OperationDefinition {
        name: "delete_ladder_vertical_wire",
        subsystem: "xgk_ladder",
        description: "Delete an exact vertical XGK branch reference pair, preserving shared rows.",
        adapter: "delete_ladder_vertical_wire",
        schema: schema_delete_ladder_vertical_wire,
        example: example_delete_ladder_vertical_wire,
        apply: apply_delete_ladder_vertical_wire,
    },
    OperationDefinition {
        name: "delete_iec_ld_horizontal_wire_record",
        subsystem: "iec_ladder",
        description: "Delete one framed IEC horizontal wire, retaining row headers and operands.",
        adapter: "delete_iec_ld_horizontal_wire_record",
        schema: schema_delete_iec_ld_horizontal_wire_record,
        example: example_delete_iec_ld_horizontal_wire_record,
        apply: apply_delete_iec_ld_horizontal_wire_record,
    },
    OperationDefinition {
        name: "delete_iec_ld_isolated_element",
        subsystem: "iec_ladder",
        description: "Remove an isolated contact or coil, leaving its physical row empty.",
        adapter: "delete_iec_ld_isolated_element",
        schema: schema_delete_iec_ld_isolated_element,
        example: example_delete_iec_ld_isolated_element,
        apply: apply_delete_iec_ld_isolated_element,
    },
    OperationDefinition {
        name: "insert_iec_ld_function_output_wire",
        subsystem: "iec_ladder",
        description: "Draw one native F5 cell from a scalar function BOOL output or extend\nits contiguous short-wire feed. A comparison OUT assignment is replaced\nby the wire; numeric destinations are preserved and cannot be wired.",
        adapter: "insert_iec_ld_function_output_wire",
        schema: schema_insert_iec_ld_function_output_wire,
        example: example_insert_iec_ld_function_output_wire,
        apply: apply_insert_iec_ld_function_output_wire,
    },
    OperationDefinition {
        name: "repair_iec_ld_horizontal_wire",
        subsystem: "iec_ladder",
        description: "Reconnect a one-cell gap left by a captured IEC contact or trigger deletion.\nNative XG5000 F5 inserts an `FF 01` short-wire record, increments the\nrow count, and resets the row header coordinate to the first contact.",
        adapter: "repair_iec_ld_horizontal_wire",
        schema: schema_repair_iec_ld_horizontal_wire,
        example: example_repair_iec_ld_horizontal_wire,
        apply: apply_repair_iec_ld_horizontal_wire,
    },
    OperationDefinition {
        name: "delete_iec_ld_function_output_operand",
        subsystem: "iec_ladder",
        description: "Remove an output assignment without removing the pin, reference, body,\nor physical row. Inputs cannot be cleared through this API.",
        adapter: "delete_iec_ld_function_output_operand",
        schema: schema_delete_iec_ld_function_output_operand,
        example: example_delete_iec_ld_function_output_operand,
        apply: apply_delete_iec_ld_function_output_operand,
    },
    OperationDefinition {
        name: "assign_iec_ld_function_output_operand",
        subsystem: "iec_ladder",
        description: "Assign a previously empty OUT pin. Reuse the existing operand validator\non a private candidate so a bad type or constant never changes the document.",
        adapter: "assign_iec_ld_function_output_operand",
        schema: schema_assign_iec_ld_function_output_operand,
        example: example_assign_iec_ld_function_output_operand,
        apply: apply_assign_iec_ld_function_output_operand,
    },
    OperationDefinition {
        name: "update_iec_ld_function_operand",
        subsystem: "iec_ladder",
        description: "Replace a captured IEC LD function input/output expression (`FF 46`).\nClassified symbols and literals are checked against the decoded pin type;\noutput pins also require a writable destination.",
        adapter: "update_iec_ld_function_operand",
        schema: schema_update_iec_ld_function_operand,
        example: example_update_iec_ld_function_operand,
        apply: apply_update_iec_ld_function_operand,
    },
    OperationDefinition {
        name: "delete_iec_ld_scalar_chain_function",
        subsystem: "iec_ladder",
        description: "Delete a scalar from a horizontal chain or a captured staggered MOVE layout,\npreserving neighboring bodies and pin rows.",
        adapter: "delete_iec_ld_scalar_chain_function",
        schema: schema_delete_iec_ld_scalar_chain_function,
        example: example_delete_iec_ld_scalar_chain_function,
        apply: apply_delete_iec_ld_scalar_chain_function,
    },
    OperationDefinition {
        name: "replace_iec_ld_scalar_chain_function",
        subsystem: "iec_ladder",
        description: "Replace a supported scalar chain body atomically while preserving its neighbors.",
        adapter: "replace_iec_ld_scalar_chain_function",
        schema: schema_replace_iec_ld_scalar_chain_function,
        example: example_replace_iec_ld_scalar_chain_function,
        apply: apply_replace_iec_ld_scalar_chain_function,
    },
    OperationDefinition {
        name: "delete_iec_ld_terminal_function",
        subsystem: "iec_ladder",
        description: "Delete a terminal IEC function block using the captured XG5000 Delete\nshape. This removes its preceding wire and its otherwise empty pin rows,\nwhile retaining the leading contact as the only record in the group.",
        adapter: "delete_iec_ld_terminal_function",
        schema: schema_delete_iec_ld_terminal_function,
        example: example_delete_iec_ld_terminal_function,
        apply: apply_delete_iec_ld_terminal_function,
    },
    OperationDefinition {
        name: "insert_iec_ld_terminal_timer",
        subsystem: "iec_ladder",
        description: "Insert a terminal TON using an existing local instance declaration.",
        adapter: "insert_iec_ld_terminal_timer",
        schema: schema_insert_iec_ld_terminal_timer,
        example: example_insert_iec_ld_terminal_timer,
        apply: apply_insert_iec_ld_terminal_timer,
    },
    OperationDefinition {
        name: "insert_iec_ld_terminal_move",
        subsystem: "iec_ladder",
        description: "Restore a native two-pin MOVE after a retained terminal contact.\nOperand edits run on a clone, so the operation is atomic.",
        adapter: "insert_iec_ld_terminal_move",
        schema: schema_insert_iec_ld_terminal_move,
        example: example_insert_iec_ld_terminal_move,
        apply: apply_insert_iec_ld_terminal_move,
    },
    OperationDefinition {
        name: "delete_iec_ld_standalone_function",
        subsystem: "iec_ladder",
        description: "Delete a standalone IEC function group using the captured XG5000\nDelete shape. The group's stored rows become an implicit blank gap and\nall later group ordinals are decremented without changing row indices.\nA contiguous final group trims the total row count at its first row.",
        adapter: "delete_iec_ld_standalone_function",
        schema: schema_delete_iec_ld_standalone_function,
        example: example_delete_iec_ld_standalone_function,
        apply: apply_delete_iec_ld_standalone_function,
    },
    OperationDefinition {
        name: "delete_iec_ld_group",
        subsystem: "iec_ladder",
        description: "Remove one complete decoded IEC LD network while leaving its row range\nas an implicit blank gap. Later network ordinals are renumbered; their\nrows, records, and coordinates remain unchanged.",
        adapter: "delete_iec_ld_group",
        schema: schema_delete_iec_ld_group,
        example: example_delete_iec_ld_group,
        apply: apply_delete_iec_ld_group,
    },
    OperationDefinition {
        name: "replace_iec_ld_group",
        subsystem: "iec_ladder",
        description: "Replace an occupied IEC LD network with a copy of another network in\nthe same program. Both edits are checked on a private document, so a\nfailed copy never leaves the destination deleted.",
        adapter: "replace_iec_ld_group",
        schema: schema_replace_iec_ld_group,
        example: example_replace_iec_ld_group,
        apply: apply_replace_iec_ld_group,
    },
    OperationDefinition {
        name: "replace_iec_ld_group_from_program",
        subsystem: "iec_ladder",
        description: "Replace a destination network with a network from another IEC program.\nMissing source locals and captured function instances can be copied in\nthe same atomic edit when requested.",
        adapter: "replace_iec_ld_group_from_program",
        schema: schema_replace_iec_ld_group_from_program,
        example: example_replace_iec_ld_group_from_program,
        apply: apply_replace_iec_ld_group_from_program,
    },
    OperationDefinition {
        name: "move_iec_ld_group",
        subsystem: "iec_ladder",
        description: "Move a complete decoded IEC LD network into an empty row range.\nIts internal branches, function pins, and operand links move together.",
        adapter: "move_iec_ld_group",
        schema: schema_move_iec_ld_group,
        example: example_move_iec_ld_group,
        apply: apply_move_iec_ld_group,
    },
    OperationDefinition {
        name: "copy_iec_ld_group",
        subsystem: "iec_ladder",
        description: "Copy a complete decoded IEC LD network into an empty row range,\nretaining the original group. The copied operands and function instance\nnames can be edited separately after insertion.",
        adapter: "copy_iec_ld_group",
        schema: schema_copy_iec_ld_group,
        example: example_copy_iec_ld_group,
        apply: apply_copy_iec_ld_group,
    },
    OperationDefinition {
        name: "copy_iec_ld_group_to_program",
        subsystem: "iec_ladder",
        description: "Copy a complete IEC network into an empty row range in another\nprogram. Referenced local symbols and function instances must already\nexist with compatible types in the destination program.",
        adapter: "copy_iec_ld_group_to_program",
        schema: schema_copy_iec_ld_group_to_program,
        example: example_copy_iec_ld_group_to_program,
        apply: apply_copy_iec_ld_group_to_program,
    },
    OperationDefinition {
        name: "copy_iec_ld_group_to_program_with_locals",
        subsystem: "iec_ladder",
        description: "Copy a network and create its missing primitive locals and captured\nfunction-instance declarations in the destination. Supported BOOL\nmappings retain their source address.\nA private document makes declaration insertion and network copying atomic.",
        adapter: "copy_iec_ld_group_to_program_with_locals",
        schema: schema_copy_iec_ld_group_to_program_with_locals,
        example: example_copy_iec_ld_group_to_program_with_locals,
        apply: apply_copy_iec_ld_group_to_program_with_locals,
    },
    OperationDefinition {
        name: "insert_iec_ld_comment",
        subsystem: "iec_ladder",
        description: "Create a standalone IEC comment in an empty row by copying a captured\none-record comment group from the same program and replacing its text.\nThe source group remains untouched; failures leave the document intact.",
        adapter: "insert_iec_ld_comment",
        schema: schema_insert_iec_ld_comment,
        example: example_insert_iec_ld_comment,
        apply: apply_insert_iec_ld_comment,
    },
    OperationDefinition {
        name: "delete_iec_ld_branch_function",
        subsystem: "iec_ladder",
        description: "Delete a scalar block on a branch row, retaining its branch scaffold.",
        adapter: "delete_iec_ld_branch_function",
        schema: schema_delete_iec_ld_branch_function,
        example: example_delete_iec_ld_branch_function,
        apply: apply_delete_iec_ld_branch_function,
    },
    OperationDefinition {
        name: "replace_iec_ld_branch_function",
        subsystem: "iec_ladder",
        description: "Replace a captured branch scalar function as one atomic document edit.\nA scaffold grows if needed and keeps surplus native branch rows; a completed\nbranch tail uses only the replacement block's own pin rows.",
        adapter: "replace_iec_ld_branch_function",
        schema: schema_replace_iec_ld_branch_function,
        example: example_replace_iec_ld_branch_function,
        apply: apply_replace_iec_ld_branch_function,
    },
    OperationDefinition {
        name: "insert_iec_ld_function",
        subsystem: "iec_ladder",
        description: "Place a scalar MOVE, arithmetic, or comparison function in available IEC cells.",
        adapter: "insert_iec_ld_function",
        schema: schema_insert_iec_ld_function,
        example: example_insert_iec_ld_function,
        apply: apply_insert_iec_ld_function,
    },
    OperationDefinition {
        name: "insert_iec_ld_standalone_function",
        subsystem: "iec_ladder",
        description: "Insert the captured standalone WORD_TO_UDINT shape at a three-row group gap.",
        adapter: "insert_iec_ld_standalone_function",
        schema: schema_insert_iec_ld_standalone_function,
        example: example_insert_iec_ld_standalone_function,
        apply: apply_insert_iec_ld_standalone_function,
    },
    OperationDefinition {
        name: "insert_iec_ld_function_cell",
        subsystem: "iec_ladder",
        description: "Insert the captured connected single-output FF function cell into a\nguarded two-row gap. Both stored rows and their surrounding records are\nretained; one function record and one output-link record are added.",
        adapter: "insert_iec_ld_function_cell",
        schema: schema_insert_iec_ld_function_cell,
        example: example_insert_iec_ld_function_cell,
        apply: apply_insert_iec_ld_function_cell,
    },
    OperationDefinition {
        name: "delete_iec_ld_function_cell",
        subsystem: "iec_ladder",
        description: "Delete a captured IEC function cell while retaining unrelated records\nin the same stored rows. The block and all of its pin-link records are\nremoved and each affected row's record count is decremented.",
        adapter: "delete_iec_ld_function_cell",
        schema: schema_delete_iec_ld_function_cell,
        example: example_delete_iec_ld_function_cell,
        apply: apply_delete_iec_ld_function_cell,
    },
    OperationDefinition {
        name: "delete_iec_ld_eq_chain_head",
        subsystem: "iec_ladder",
        description: "Delete the first EQ in the captured four-block comparison chain.\nXG5000 closes the fourth row of that block and joins the x12 feed to\nthe next EQ; deleting only the function records leaves a stale branch.",
        adapter: "delete_iec_ld_eq_chain_head",
        schema: schema_delete_iec_ld_eq_chain_head,
        example: example_delete_iec_ld_eq_chain_head,
        apply: apply_delete_iec_ld_eq_chain_head,
    },
    OperationDefinition {
        name: "delete_iec_ld_heating_chain_head",
        subsystem: "iec_ladder",
        description: "Delete the first comparison block in the captured heating/cooling\nchain. Native Delete Line on its first pin row removes the four-row\nblock body, joins the two long branch feeds, and shifts later rows.",
        adapter: "delete_iec_ld_heating_chain_head",
        schema: schema_delete_iec_ld_heating_chain_head,
        example: example_delete_iec_ld_heating_chain_head,
        apply: apply_delete_iec_ld_heating_chain_head,
    },
    OperationDefinition {
        name: "delete_iec_ld_heating_chain_middle",
        subsystem: "iec_ladder",
        description: "Delete one of the two captured middle EQ blocks in the heating chain.\nNative Delete Line on its first pin row keeps the x3/x15 branch feeds.",
        adapter: "delete_iec_ld_heating_chain_middle",
        schema: schema_delete_iec_ld_heating_chain_middle,
        example: example_delete_iec_ld_heating_chain_middle,
        apply: apply_delete_iec_ld_heating_chain_middle,
    },
    OperationDefinition {
        name: "delete_iec_ld_heating_chain_x15_eq",
        subsystem: "iec_ladder",
        description: "Delete an x15-fed EQ in the captured heating comparison chain by its\nfirst pin row. The shape guard also accepts the next comparison after\nan earlier deletion has shifted its row index.",
        adapter: "delete_iec_ld_heating_chain_x15_eq",
        schema: schema_delete_iec_ld_heating_chain_x15_eq,
        example: example_delete_iec_ld_heating_chain_x15_eq,
        apply: apply_delete_iec_ld_heating_chain_x15_eq,
    },
    OperationDefinition {
        name: "delete_iec_ld_heating_chain_contact_eq",
        subsystem: "iec_ladder",
        description: "Delete the captured contact-fed L62 EQ by its first pin row.\nXG5000 removes its x6/x12 contact branches and retains the x15 feed.",
        adapter: "delete_iec_ld_heating_chain_contact_eq",
        schema: schema_delete_iec_ld_heating_chain_contact_eq,
        example: example_delete_iec_ld_heating_chain_contact_eq,
        apply: apply_delete_iec_ld_heating_chain_contact_eq,
    },
    OperationDefinition {
        name: "delete_iec_ld_heating_chain_x3_eq_repaired",
        subsystem: "iec_ladder",
        description: "Delete the x3-fed EQ and its orphan x15 feed as one valid edit.",
        adapter: "delete_iec_ld_heating_chain_x3_eq_repaired",
        schema: schema_delete_iec_ld_heating_chain_x3_eq_repaired,
        example: example_delete_iec_ld_heating_chain_x3_eq_repaired,
        apply: apply_delete_iec_ld_heating_chain_x3_eq_repaired,
    },
    OperationDefinition {
        name: "delete_iec_ld_heating_chain_x3_eq",
        subsystem: "iec_ladder",
        description: "Reproduce XG5000 Delete Line on the L59 pin row of the L58 EQ.\nThis native edit leaves one input/output error until the circuit is repaired.",
        adapter: "delete_iec_ld_heating_chain_x3_eq",
        schema: schema_delete_iec_ld_heating_chain_x3_eq,
        example: example_delete_iec_ld_heating_chain_x3_eq,
        apply: apply_delete_iec_ld_heating_chain_x3_eq,
    },
    OperationDefinition {
        name: "repair_iec_ld_heating_chain_x3_eq_deletion",
        subsystem: "iec_ladder",
        description: "Remove the dangling x15 feed segments after deleting an x3-fed\ncomparison. The unique orphan chain is found from decoded rows and\nbranch endpoints, restoring a valid circuit while retaining the x3 feed.",
        adapter: "repair_iec_ld_heating_chain_x3_eq_deletion",
        schema: schema_repair_iec_ld_heating_chain_x3_eq_deletion,
        example: example_repair_iec_ld_heating_chain_x3_eq_deletion,
        apply: apply_repair_iec_ld_heating_chain_x3_eq_deletion,
    },
    OperationDefinition {
        name: "delete_iec_ld_connected_arithmetic",
        subsystem: "iec_ladder",
        description: "Delete a captured connected ADD/SUB in a five-row R_TRIG/arithmetic/MOVE group.\nXG5000 retains the R_TRIG and MOVE circuits as two separate groups.",
        adapter: "delete_iec_ld_connected_arithmetic",
        schema: schema_delete_iec_ld_connected_arithmetic,
        example: example_delete_iec_ld_connected_arithmetic,
        apply: apply_delete_iec_ld_connected_arithmetic,
    },
    OperationDefinition {
        name: "delete_iec_ld_branched_arithmetic",
        subsystem: "iec_ladder",
        description: "Delete a four-row scalar arithmetic block sharing an external branch spine.\nMatches native Delete Line on its final continuation row, retaining the\nspine and neighboring functions while shifting later rows up once.",
        adapter: "delete_iec_ld_branched_arithmetic",
        schema: schema_delete_iec_ld_branched_arithmetic,
        example: example_delete_iec_ld_branched_arithmetic,
        apply: apply_delete_iec_ld_branched_arithmetic,
    },
    OperationDefinition {
        name: "update_iec_ld_arithmetic_function",
        subsystem: "iec_ladder",
        description: "Change a captured three-operand IEC ADD/SUB/MUL/DIV block.\nThe native capture shows a paired function opcode and UTF-16 name.",
        adapter: "update_iec_ld_arithmetic_function",
        schema: schema_update_iec_ld_arithmetic_function,
        example: example_update_iec_ld_arithmetic_function,
        apply: apply_update_iec_ld_arithmetic_function,
    },
    OperationDefinition {
        name: "update_iec_ld_comparison_function",
        subsystem: "iec_ladder",
        description: "Change a captured three-operand IEC comparison block among EQ/GT/GE/LT/LE.",
        adapter: "update_iec_ld_comparison_function",
        schema: schema_update_iec_ld_comparison_function,
        example: example_update_iec_ld_comparison_function,
        apply: apply_update_iec_ld_comparison_function,
    },
    OperationDefinition {
        name: "edit_ladder_cell",
        subsystem: "xgk_ladder",
        description: "Insert, replace, or delete a contact/coil record in the captured\nLD format. Unlike text clearing, deletion removes the actual record.\nUnsupported topology is rejected without changing the document.",
        adapter: "edit_ladder_cell",
        schema: schema_edit_ladder_cell,
        example: example_edit_ladder_cell,
        apply: apply_edit_ladder_cell,
    },
    OperationDefinition {
        name: "insert_ladder_instruction",
        subsystem: "xgk_ladder",
        description: "Insert a fixed-arity catalog instruction at an XGK row's output end.",
        adapter: "insert_ladder_instruction",
        schema: schema_insert_ladder_instruction,
        example: example_insert_ladder_instruction,
        apply: apply_insert_ladder_instruction,
    },
    OperationDefinition {
        name: "insert_ladder_comparison",
        subsystem: "xgk_ladder",
        description: "Insert a comparison contact spanning three cells at the selected column.",
        adapter: "insert_ladder_comparison",
        schema: schema_insert_ladder_comparison,
        example: example_insert_ladder_comparison,
        apply: apply_insert_ladder_comparison,
    },
    OperationDefinition {
        name: "delete_ladder_comparison",
        subsystem: "xgk_ladder",
        description: "Delete a verified XGK comparison contact, leaving its occupied wiring gap.",
        adapter: "delete_ladder_comparison",
        schema: schema_delete_ladder_comparison,
        example: example_delete_ladder_comparison,
        apply: apply_delete_ladder_comparison,
    },
    OperationDefinition {
        name: "delete_ladder_instruction",
        subsystem: "xgk_ladder",
        description: "Delete a catalog application from an unbranched output row, including its feed wire.",
        adapter: "delete_ladder_instruction",
        schema: schema_delete_ladder_instruction,
        example: example_delete_ladder_instruction,
        apply: apply_delete_ladder_instruction,
    },
    OperationDefinition {
        name: "edit_ladder_comment",
        subsystem: "xgk_ladder",
        description: "Create or edit a native rung/output comment record.",
        adapter: "edit_ladder_comment",
        schema: schema_edit_ladder_comment,
        example: example_edit_ladder_comment,
        apply: apply_edit_ladder_comment,
    },
    OperationDefinition {
        name: "delete_ladder_rung_comment",
        subsystem: "xgk_ladder",
        description: "Delete a native rung-comment row and close the resulting coordinate gap.",
        adapter: "delete_ladder_rung_comment",
        schema: schema_delete_ladder_rung_comment,
        example: example_delete_ladder_rung_comment,
        apply: apply_delete_ladder_rung_comment,
    },
    OperationDefinition {
        name: "edit_ladder_branch",
        subsystem: "xgk_ladder",
        description: "Add or remove a supported vertical connection between adjacent rows.",
        adapter: "edit_ladder_branch",
        schema: schema_edit_ladder_branch,
        example: example_edit_ladder_branch,
        apply: apply_edit_ladder_branch,
    },
    OperationDefinition {
        name: "insert_ladder_row",
        subsystem: "xgk_ladder",
        description: "Insert a physical blank row before the selected row, as native Ctrl+L.",
        adapter: "insert_ladder_row",
        schema: schema_insert_ladder_row,
        example: example_insert_ladder_row,
        apply: apply_insert_ladder_row,
    },
    OperationDefinition {
        name: "move_program",
        subsystem: "programs",
        description: "Move a program to a final list index without changing its task or contents.",
        adapter: "move_program",
        schema: schema_move_program,
        example: example_move_program,
        apply: apply_move_program,
    },
    OperationDefinition {
        name: "delete_program",
        subsystem: "programs",
        description: "Remove the selected program, checking its identity before changing XML.",
        adapter: "delete_program",
        schema: schema_delete_program,
        example: example_delete_program,
        apply: apply_delete_program,
    },
    OperationDefinition {
        name: "create_program",
        subsystem: "programs",
        description: "Append an empty scan program without copying existing application code.",
        adapter: "create_program",
        schema: schema_create_program,
        example: example_create_program,
        apply: apply_create_program,
    },
    OperationDefinition {
        name: "edit_cnet_settings",
        subsystem: "networks",
        description: "Edit existing Cnet fields without rebuilding port records or opaque data.",
        adapter: "edit_cnet_settings",
        schema: schema_edit_cnet_settings,
        example: example_edit_cnet_settings,
        apply: apply_edit_cnet_settings,
    },
    OperationDefinition {
        name: "edit_sfc_entity",
        subsystem: "sfc",
        description: "Edit captured SFC step comments or direct BOOL M transitions.\nIdentity and old-value checks reject stale selections; all other XML is preserved.",
        adapter: "edit_sfc_entity",
        schema: schema_edit_sfc_entity,
        example: example_edit_sfc_entity,
        apply: apply_edit_sfc_entity,
    },
    OperationDefinition {
        name: "replace_sfc_sequence",
        subsystem: "sfc",
        description: "Replace a captured linear chart atomically, retaining program identities and local symbols.",
        adapter: "replace_sfc_sequence",
        schema: schema_replace_sfc_sequence,
        example: example_replace_sfc_sequence,
        apply: apply_replace_sfc_sequence,
    },
    OperationDefinition {
        name: "edit_sfc_variable",
        subsystem: "sfc",
        description: "Insert, update, or remove captured program-local SFC declarations.",
        adapter: "edit_sfc_variable",
        schema: schema_edit_sfc_variable,
        example: example_edit_sfc_variable,
        apply: apply_edit_sfc_variable,
    },
];

pub fn list_operations(query: Option<&str>, subsystem: Option<&str>) -> Value {
    let query = query.map(str::to_lowercase);
    let selected = OPERATIONS
        .iter()
        .filter(|operation| {
            subsystem.is_none_or(|value| value == operation.subsystem)
                && query.as_ref().is_none_or(|value| {
                    operation.name.to_lowercase().contains(value)
                        || operation.description.to_lowercase().contains(value)
                        || operation.subsystem.contains(value)
                })
        })
        .collect::<Vec<_>>();
    let mut groups: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for operation in &selected {
        groups
            .entry(operation.subsystem)
            .or_default()
            .push(operation.name);
    }
    json!({
        "count": selected.len(),
        "operations": selected.iter().map(|operation| json!({
            "name": operation.name,
            "tool_name": crate::writer_tools::tool_name(operation.name),
            "editing_route": crate::writer_tools::editing_route(operation.name),
            "subsystem": operation.subsystem,
            "description": operation.description,
            "adapter": operation.adapter,
            "compatibility_alias": operation.adapter != operation.name,
            "compatibility_relationship": compatibility_relationship(operation.name)
        })).collect::<Vec<_>>(),
        "by_subsystem": groups
    })
}

fn find_operation(name: &str) -> Result<&'static OperationDefinition, String> {
    OPERATIONS
        .iter()
        .find(|operation| operation.name == name)
        .ok_or_else(|| format!("unknown operation {name:?}; use list_operations"))
}

fn compatibility_defaults(name: &str) -> Value {
    match name {
        "insert_iec_ld_parallel_contact" => json!({"parallel_kind": "NO"}),
        "insert_iec_ld_linear_rung" => json!({"contact_kind": "NO", "coil_kind": "OUTPUT"}),
        "delete_iec_ld_linear_rung" => {
            json!({"expected_contact_kind": "NO", "expected_coil_kind": "OUTPUT"})
        }
        "insert_iec_ld_no_contact" => json!({"contact_kind": "NO"}),
        _ => json!({}),
    }
}

fn compatibility_relationship(name: &str) -> Value {
    let preferred = match name {
        "delete_iec_ld_no_contact" => "delete_iec_ld_contact",
        "delete_iec_ld_no_contact_cell" => "delete_iec_ld_contact_cell",
        _ => return Value::Null,
    };
    json!({
        "relationship": "legacy_primitive",
        "preferred_operation": preferred,
        "semantics": "Retains the library's captured-contact guards without an explicit expected_contact_kind argument. The preferred operation checks expected_contact_kind and then delegates to this shared native primitive. This operation is not restricted to a normally open contact."
    })
}

fn operation_constraints(name: &str) -> Vec<&'static str> {
    match name {
        "edit_ladder_cell" => vec![
            "XGK raw_y must be a multiple of 4 within the native canvas. Contact columns are 0..8; coil column is 9. expected:null requires an unoccupied element site; replacement:null deletes the record and leaves a wiring gap.",
            "Copy expected kind and operand from inspection. INVERSE/RISING/FALLING operandless logic kinds use an empty operand; addressed contact and coil kinds require supported device operands. Unsupported instruction spans/branches remain guarded.",
        ],
        "edit_ladder_comment" => vec![
            "Rung comments insert a dedicated row above raw_y and shift subsequent rows; insertion inside a branch span is rejected. Output comments use the existing ladder row and do not insert a comment row.",
            "expected:null creates a new comment and rejects an existing comment at that site. Replacement must contain 1..255 UTF-16 units, with only newline/carriage-return/tab controls allowed. Re-inspect after rung insertion.",
        ],
        "delete_ladder_rung_comment" => vec![
            "raw_y and expected text must identify the existing dedicated comment row. Deletion closes the native coordinate gap; subsequent row coordinates and record offsets change.",
        ],
        "insert_ladder_row" => vec![
            "Insert at native raw_y in increments of 4. This shifts existing rows and record coordinates; inspect branch spans and re-inspect the resulting program before dependent edits.",
        ],
        "edit_ladder_branch" => vec![
            "raw_y identifies the upper adjacent native row; boundary is a cell-grid boundary rather than an element x coordinate. expected is the current segment presence. Addition/removal retains supported native branch and row guards.",
        ],
        "insert_ladder_instruction" => vec![
            "Insert a catalog application at the output end of raw_y. Obtain the exact mnemonic, operand count, widths/device permissions and CPU restrictions from catalog.instructions and the instruction guide. Retained output shapes and instruction spans must be supported.",
        ],
        "insert_ladder_comparison" => vec![
            "Use a mnemonic from catalog.comparisons. The comparison and its operands must fit before output column 9; occupied sites, overlapping spans and incompatible operand widths/devices are rejected.",
        ],
        "delete_iec_ld_isolated_element" => vec![
            "offset is the record offset, not the operand string offset. The native row must contain exactly one contact or coil. expected is its current operand. Deletion retains the row envelope and clears its record count.",
        ],
        "insert_iec_ld_terminal_coil" => vec![
            "contact_record_offset is the record start of a supported isolated retained contact. Match expected_contact_variable. The writer adds the terminal coil and feed wiring; output operands must resolve to writable BOOL expressions.",
        ],
        "delete_iec_ld_terminal_coil" => vec![
            "coil_record_offset is the native coil record start. Match expected_variable. Only captured terminal-contact/coil row shapes are supported; the writer may remove associated feed wiring.",
        ],
        "update_iec_ld_contact_kind" | "update_iec_ld_coil_kind" => vec![
            "offset is the operand string offset (record offset + 15 in the supported captured grammar). expected and replacement are contact/coil kind names, not operand text. Coordinates and operands remain unchanged.",
        ],
        "update_iec_ld_element_operand" => vec![
            "offset is the operand string offset. Match the exact current operand. Replacement must be nonempty without control characters; resolved types must include BOOL, and coil destinations must be writable. Invalid direct-device expressions are rejected.",
        ],
        "update_iec_ld_comment" => vec![
            "offset is the captured comment string offset. Match exact current text. Only captured FF 3F comment records are supported; replacement must be nonempty without control characters. Output annotation records are not automatically editable comments.",
        ],
        "update_iec_local_symbol_description" => vec![
            "symbol_index, expected_name and expected_description identify the current declaration. Name/type/address remain unchanged. Copy values from symbols inspection; do not infer indices from ladder records.",
        ],
        "rename_iec_local_symbol" => vec![
            "Match the current declaration index/name. The library preserves supported instance/reference relationships and rejects incompatible declarations or identifiers. Re-inspect references and symbols afterward.",
        ],
        "update_iec_local_symbol_type" => vec![
            "Match expected_name and expected_type from the current declaration. Replacement must be a supported IEC type with compatible allocation and retained references; this is not unrestricted type conversion.",
        ],
        "delete_iec_local_symbol" => vec![
            "Match symbol_index and expected_name. Referenced declarations or unsupported instance/allocation layouts remain protected by the library. Indices change after deletion.",
        ],
        "move_program" => vec![
            "from/to are document-order program indices in the same Programs container. Match both source and target ObjectId; movement does not change task or contents. Re-inspect program indices afterward.",
        ],
        "delete_program" => vec![
            "Match the selected program's current ObjectId. The program must be top-level under Programs, and workspace node-count metadata must be present and valid. Re-inspect document-order indices after deletion.",
        ],
        "edit_browser_hardware" => vec![
            "patch.operation is slotCount, comment, inputFilter or option. slot is required for module operations; key and index are required for option.",
            "For inputFilter and option, expected_value is the current full native Details attribute, not the displayed option value; replacement is a decimal filter or option value.",
        ],
        "edit_browser_network" => vec![
            "For module=false, patch.field is name. For module=true, patch.field is configName, alias or description, and base/slot are required.",
        ],
        "edit_browser_iec" => vec![
            "patch.operation is kind, elementOperand or functionOperand. expected_kind is the current contact/coil kind or function name. Operand replacements must be compatible declared primitive local symbols.",
        ],
        "edit_fenet_field" => vec![
            "patch.field is stationNo, glofaSocketCount, rcvWaitTime, clientWaitTime, driverType, ipAddress, subnet, gateway, dns, ipAddress2, subnet2, gateway2, dns2, dhcp or dhcp2. Fields must already exist on the supported module shape.",
            "IPv4 values require four decimal octets; subnet masks require contiguous leading ones. Scalar ranges and supported driver choices are checked by libxgwx.",
        ],
        "edit_cnet_settings" => vec![
            "changes[].field is stationNo, modeRaw, bps, dataBitRaw, stopBitRaw, parityRaw, rxTimeout, charTimeout, driverType, requestDelayTime, parityErrorIgnore, terminatingResister or repeater. port_index addresses native port order (0 or 1); duplicate port/field edits are rejected.",
            "expected_value and replacement are decimal native setting values. Paired-port, communication-driver and range constraints are checked together by libxgwx.",
        ],
        "delete_iec_ld_no_contact" | "delete_iec_ld_no_contact_cell" => vec![
            "Legacy captured-contact primitive retained with its original semantics, including supported kinds beyond NO. The corresponding delete_iec_ld_contact operation additionally checks expected_contact_kind before calling this library primitive.",
        ],
        "set_base_slot_count" => vec![
            "slot_count is 4, 6, 8, 10 or 12, subject to the selected CPU and retained modules.",
        ],
        "set_module_input_filter" => vec![
            "The selected module must be a supported XGI-D24A/B input module. Verified filter choices are Default, Ms1, Ms3, Ms5, Ms10, Ms20, Ms70 and Ms100. The native Rust API also accepts Unknown(raw_byte), represented as an externally tagged enum object; this is not a verified catalog choice.",
        ],
        "insert_iec_local_symbol" => vec![
            "The existing LocalVar/Symbols table must have a text node. A native empty zero-count table without text is reported as MissingSymbols by the current library writer.",
        ],
        "update_variable" => vec![
            "Names must be nonempty and unique ignoring case. Name/description may resize to at most 255 UTF-16 units and cannot contain control characters. Address-area and data-type text retain their existing UTF-16 length.",
        ],
        "create_program" => vec![
            "patch.language is LD, SFC, ST or IL where supported by the selected CPU. object_id and symbol_id must be distinct fresh 36-character GUIDs; the program identifier must be unique ignoring case.",
        ],
        "edit_vendor_il" => vec![
            "XGK IL uses LOAD/AND/OUT and catalog application instructions, not IEC LD/ST. This rebuilds the selected ladder payload; comments/geometry not represented by IL are not retained. Reject unsupported branch topology and Auto-allocation. Copy exact exported source and ObjectId; inspect after replacement.",
        ],
        "edit_text_program" => vec![
            "expected_language is ST or IL and must match inspection. Copy expected_source exactly, including line endings. Replacement is at most 65536 UTF-16 units and cannot contain NUL. Encryption and unsupported bookmark/breakpoint or source layouts remain read-only. Source decoding is not syntax/type checking.",
        ],
        "edit_text_variable" => vec![
            "Copy expected_object_id and the complete expected_variables snapshot from text inspection, including system/declaration metadata. Existing IEC declaration type, mapping and reference restrictions apply.",
        ],
        "edit_sfc_entity" => vec![
            "patch.field is comment for a step (expected_type=0), or condition for an existing direct %MX BOOL transition (expected_type=1). Position, type and old field text must match exactly.",
        ],
        "insert_iec_ld_single_element" => vec![
            "category is contact or coil. raw_x is 1 through 94 on the native three-unit grid. This edit may create an incomplete row.",
            "Contact kind is NO, NC, RISING, FALLING, NEGATED_RISING or NEGATED_FALLING; coil kind is OUTPUT, INVERSE, SET, RESET, RISING or FALLING.",
        ],
        "update_iec_ld_arithmetic_function" => vec!["replacement is ADD, SUB, MUL or DIV."],
        "update_iec_ld_comparison_function" => vec!["replacement is EQ, GT, GE, LT or LE."],
        _ => Vec::new(),
    }
}

/// Bundled executable examples are tested as ordered fixture workflows. These
/// arguments describe those fixtures, never assumed sites in a user's project.
fn verified_fixture_examples(name: &str) -> Vec<Value> {
    static CATALOG: std::sync::OnceLock<Value> = std::sync::OnceLock::new();
    let catalog = CATALOG.get_or_init(|| {
        serde_json::from_str(include_str!("../examples/verified-operation-cases.json"))
            .expect("bundled fixture cases are valid JSON")
    });
    let mut examples = vec![];
    for case in catalog["cases"].as_array().expect("fixture cases array") {
        let mut setup = vec![];
        for step in case["setup"]
            .as_array()
            .unwrap()
            .iter()
            .chain(case["operations"].as_array().unwrap())
        {
            if step["name"] == name {
                examples.push(json!({"kind":"fixture_verified", "case_id":case["id"],
                    "fixture":case["fixture"], "fixture_sha256":case["fixture_sha256"],
                    "setup_operations":setup, "operation":name, "arguments":step["args"],
                    "note":"Verified against the named local fixture after setup_operations. Inspect your actual project and replace fixture-specific identities, positions and expected values. Local verification does not establish native XG5000 acceptance."}));
            }
            setup.push(step.clone());
        }
    }
    examples
}

fn precise_inspection_requirements(name: &str) -> Vec<Value> {
    let req = |section: &str, fields: &[&str], purpose: &str| {
        json!({"section":section,
        "program_index_required": (section == "text" || section == "text.variables" || section == "vendor_il") || section.starts_with("sfc.") || (section.starts_with("program") && section!="programs"),
        "fields":fields,"purpose":purpose,"freshness":"Use one current file revision; re-inspect after structural edits."})
    };
    match name {
        "edit_vendor_il" => vec![req(
            "vendor_il",
            &["program_index", "object_id", "source", "editable", "reason"],
            "Read the selected XGK IL projection. Full payload replacement is allowed only for supported straight-line IL; do not route IEC IL through this writer.",
        )],
        "edit_text_program" | "edit_text_variable" => vec![req(
            "text",
            &[
                "program_index",
                "object_id",
                "language",
                "source",
                "editable",
                "reason",
                "variables",
                "variables_error",
            ],
            "Select a standalone ST/IL program. Copy the exact source and complete declaration snapshot; use the source-program identity, not an SFC block identity.",
        )],
        "edit_sfc_entity" | "replace_sfc_sequence" => vec![req(
            "sfc.block.<block_index>",
            &[
                "block_index",
                "entities",
                "editable_rows",
                "sequence_editing_supported",
            ],
            "Collect complete entity/row pages at one revision. Entity edits retain native type/row/column and exact field values; chart replacement requires complete expected entities and source-bearing expected rows. Do not invent rows for unsupported charts.",
        )],
        "edit_sfc_variable" => vec![req(
            "sfc.variables",
            &["name", "data_type", "description", "system", "declaration"],
            "Collect the complete variable snapshot including advanced declaration metadata. Refresh after each direct edit; native system, mapped and reference restrictions apply.",
        )],
        "edit_ladder_cell" => vec![req(
            "program.cells",
            &[
                "offset", "raw_x", "raw_y", "contact", "coil", "value", "operands",
            ],
            "Derive expected element kind/operand and map the native position to a cell column. XGK coils use raw_x=94 for column 9.",
        )],
        "delete_ladder_horizontal_wire" | "delete_ladder_vertical_wire" | "edit_ladder_branch" => {
            vec![req(
                "program",
                &["horizontal_lines", "vertical_lines", "rungs"],
                "Use native wire endpoints and current branch presence. XGK branch boundary is raw_x/3; creation joins adjacent rungs with raw_y difference 4.",
            )]
        }
        "delete_iec_ld_contact" | "delete_iec_ld_terminal_coil" => vec![req(
            "program.records",
            &[
                "offset",
                "group_index",
                "row_index",
                "raw_x",
                "element_kind",
                "operand_string.value",
            ],
            "Use the native record offset, not operand_string.offset. Preserve exact expected kind, anchor and variable. Native Delete semantics and supported row/function-binding guards apply; re-inspect after deletion.",
        )],
        "delete_iec_ld_horizontal_wire_record" | "edit_iec_ld_vertical_wire" => vec![req(
            "program",
            &["iec_geometry.horizontal", "iec_geometry.vertical"],
            "Use physical record geometry, not derived circuit graph endpoints. Horizontal deletion requires record offset and exact start_x/end_x. Vertical edits require group_index, start/end_row_index, x, and expected presence. Re-inspect after editing.",
        )],
        "update_ladder_cell_text" | "delete_ladder_instruction" | "delete_ladder_comparison" => {
            vec![
                req(
                    "program.cells",
                    &["offset", "raw_x", "raw_y", "value", "operands"],
                    "Choose the actual source string offset and native instruction span. Cell.value is the decoded mnemonic, not the complete expected instruction string.",
                ),
                req(
                    "program.strings",
                    &["offset", "value"],
                    "Use the complete string at the cell offset as expected, including mnemonic and comma-separated operands. Do not edit decomposed operand copies. Native guards check redundant copies, operand rules, occupied widths and CPU restrictions.",
                ),
            ]
        }
        "edit_ladder_comment" | "delete_ladder_rung_comment" => vec![req(
            "program",
            &["rung_comments", "output_comments", "raw_y", "text"],
            "Choose the native comment kind and exact existing text; creation requires an absent comment site.",
        )],
        "insert_ladder_instruction" | "insert_ladder_comparison" => vec![
            req(
                if name == "insert_ladder_instruction" {
                    "catalog.instructions"
                } else {
                    "catalog.comparisons"
                },
                &["mnemonic", "operand_count", "operand_rules"],
                "Select a supported mnemonic and operand widths/device permissions.",
            ),
            req(
                "program.cells",
                &["raw_x", "raw_y", "kind", "value"],
                "Check the requested row and retained instruction/element spans.",
            ),
        ],
        "update_iec_ld_contact_kind"
        | "update_iec_ld_coil_kind"
        | "update_iec_ld_element_operand"
        | "update_iec_ld_comment" => vec![req(
            "program.records",
            &[
                "offset",
                "group_index",
                "row_index",
                "raw_x",
                "element_kind",
                "operand_string.offset",
                "operand_string.value",
            ],
            "Use operand_string.offset as the writer offset; record offset identifies the enclosing native record. Kind changes expect element_kind, while text edits expect operand_string.value.",
        )],
        "update_iec_ld_arithmetic_function" | "update_iec_ld_comparison_function" => vec![req(
            "program.functions",
            &[
                "record_offset",
                "name.offset",
                "name.value",
                "opcode_family",
                "pins",
            ],
            "Use the decoded function-name string offset and exact current name; replacement stays within the captured arithmetic or comparison family. Preserve pin bindings and shape.",
        )],
        "update_iec_ld_function_operand" => vec![
            req(
                "program.records",
                &["offset", "operand_string.offset", "operand_string.value"],
                "Use the operand string offset and exact current expression, not the native record offset.",
            ),
            req(
                "program.operand_links",
                &[
                    "target_record_offset",
                    "ordinal",
                    "is_output",
                    "pin_row_index",
                    "pin_raw_x",
                    "data_type_mask",
                    "is_array",
                ],
                "Retain resolved pin identity/type and require a writable output destination. Empty text invokes structural output deletion through the direct API; re-inspect afterward.",
            ),
            req(
                "symbols",
                &["name", "data_type", "storage_class", "address"],
                "Check local/global declaration types and supported direct device expressions against the bound pin.",
            ),
        ],
        "insert_iec_ld_single_element" | "delete_iec_ld_isolated_element" => vec![
            req(
                "program.rows",
                &["group_index", "row_index", "record_count"],
                "Inspect the actual native row envelope and distinguish empty/isolated rows from connected rows.",
            ),
            req(
                "program.records",
                &[
                    "offset",
                    "kind",
                    "group_index",
                    "row_index",
                    "raw_x",
                    "operand_string.value",
                ],
                "Deletion uses record offset and its exact operand. Insertion uses native row_index/raw_x and must satisfy the supported edit site.",
            ),
            req(
                "symbols",
                &["name", "data_type", "storage_class", "address"],
                "Check BOOL operands and writable output destinations; pass program_index for local declarations.",
            ),
        ],
        "insert_iec_local_symbol"
        | "delete_iec_local_symbol"
        | "rename_iec_local_symbol"
        | "update_iec_local_symbol_type"
        | "update_iec_local_symbol_description" => vec![req(
            "symbols",
            &[
                "symbol_index",
                "name",
                "data_type",
                "description",
                "storage_class",
                "allocation_number",
                "allocation_width",
            ],
            "Pass program_index to inspect the selected local table. Copy declaration indices/names and expected type/description verbatim.",
        )],
        "rename_project" => vec![req(
            "overview",
            &["project.name"],
            "Copy the inspected current project name into expected_name.",
        )],
        "create_program" | "move_program" | "delete_program" | "update_program" => vec![req(
            "programs",
            &["program_index", "object_id", "name", "task"],
            "Use document-order indices and native ObjectId values. Creation requires fresh distinct GUIDs, never reused inspected identities.",
        )],
        _ => vec![],
    }
}

pub fn describe_operation(name: &str) -> Result<Value, String> {
    let operation = find_operation(name)?;
    let (inspection, format_constraints) = match operation.subsystem {
        "hardware" => (
            "hardware: CPU identity, bases, module base/slot, catalog options and current values",
            "CPU/model compatibility, module placement and supported catalog values are enforced by libxgwx. select_cpu retains the current parameters, programs, hardware and networks where the captured migration is supported; inspect its method description before applying.",
        ),
        "networks" => (
            "networks: network index, module base/slot, current field values, FEnet/Cnet decoded attributes",
            "Only verified existing fields and supported protocol shapes are editable. Retain exact expected_value checks.",
        ),
        "symbols" => (
            "symbols and programs: table index, current name/type/address/description, instance identities",
            "Global variable name/comment strings are rebuilt within the native string-length bounds; IEC local and function-instance edits retain libxgwx type, declaration and reference guards.",
        ),
        "programs" => (
            "programs: document program index, ObjectId and SymbolId, language and task",
            "Creation uses captured native templates. Structural edits change document indices; source/target identity checks must match current inspection.",
        ),
        "xgk_ladder" => (
            "programs: decoded XGK cells, records, offsets, expected kinds/operands and raw coordinates",
            "XGK cells use columns 0 through 9 and raw_y in multiples of 4. Branch boundaries and wire coordinates are native coordinates. Operands and application instructions are catalog-guarded.",
        ),
        "iec_ladder" => (
            "programs: IEC record frames, row/group coordinates, graph, function blocks/pins/operand links and available edit_sites",
            "IEC coordinates and offsets differ from XGK cells. Use current decoded records and supported edit sites; unsupported shapes remain guarded. Some operations deliberately allow incomplete circuits and require diagnostics afterwards.",
        ),
        "text_programs" => (
            "text: program index, ObjectId, language, exact source, editability and complete declarations",
            "Standalone ST/IEC IL use native UTF-16 source containers. Preserve exact expected values and advanced declaration metadata. Local decoding is not native syntax/type checking.",
        ),
        "sfc" => (
            "sfc: current entities, editable_rows, positions and expected_variables with declaration metadata",
            "SFC edits require captured supported charts/declarations. Copy expected_entities, expected_rows and expected_variables verbatim from snake_case inspection. Native property names and field selectors retain libxgwx spelling.",
        ),
        _ => unreachable!("catalog subsystem"),
    };
    let mut constraints = vec![
        "All argument field names are snake_case; unknown fields are rejected, including nested patches.",
        "All existing libxgwx preconditions and unsupported-layout errors are preserved.",
        "Structural edits invalidate indices and offsets; re-inspect before constructing the next dependent request.",
        format_constraints,
        "Local serialization/container checks do not establish native XG5000 acceptance.",
    ];
    constraints.extend(operation_constraints(name));
    if operation.subsystem == "iec_ladder"
        && (name.contains("contact") || name.contains("rung") || name.contains("coil_kind"))
    {
        constraints.push("Addressed contact kinds are NO, NC, RISING, FALLING, NEGATED_RISING and NEGATED_FALLING; coil kinds are OUTPUT, INVERSE, SET, RESET, RISING and FALLING where the selected operation accepts kinds.");
    }
    let mut examples = verified_fixture_examples(name);
    let verified_count = examples.len();
    examples.truncate(2);
    examples.push(json!({
        "kind":"argument_template",
        "note":"Argument template only. Replace placeholders and coordinates with current inspection values and the requested edit; array snapshots must be copied from inspection.",
        "operation":operation.name,"arguments":(operation.example)()
    }));
    Ok(json!({
        "name": operation.name,
        "tool_name": crate::writer_tools::tool_name(operation.name),
        "editing_route": crate::writer_tools::editing_route(operation.name),
        "subsystem": operation.subsystem,
        "description": operation.description,
        "adapter": operation.adapter,
        "compatibility_alias": operation.adapter != operation.name,
        "compatibility_defaults": compatibility_defaults(name),
        "compatibility_relationship": compatibility_relationship(name),
        "argument_schema": (operation.schema)(),
        "constraints": constraints,
        "required_inspection": [inspection],
        "inspection_requirements": precise_inspection_requirements(name),
        "validation_evidence": {"bundled_fixture_example_count":verified_count,
            "scope":"Executable bundled examples; additional tests may exist outside this registry.",
            "native_acceptance":"unverified"},
        "examples": examples
    }))
}

pub fn apply_operation(doc: &mut XgwxDocument, name: &str, args: Value) -> Result<Value, String> {
    let operation = find_operation(name)?;
    (operation.apply)(doc, args)?;
    Ok(json!({
        "operation": operation.name,
        "adapter": operation.adapter,
        "status": "applied",
        "requires_reinspection": true
    }))
}
