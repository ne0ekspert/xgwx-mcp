//! Patch-first MCP editing. Only writers with capabilities outside the text IR
//! remain independent tools; batch inputs cannot bypass the patch-only boundary.
use crate::{
    operations::{self, OperationDefinition},
    server::{XgwxServer, blocking},
    storage::{EditOperation, StoreError},
};
use rmcp::{
    handler::server::router::tool::{ToolRoute, ToolRouter},
    model::{Tool, ToolAnnotations},
};
use serde::Deserialize;
use serde_json::{Map, Value, json};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WriterRequest {
    path: String,
    expected_revision: String,
    args: Value,
    #[serde(default)]
    dry_run: bool,
}

/// Writers routed through guarded text IR within the supported public scope.
/// Partial coverage is deliberately not sufficient to remove a direct tool.
pub(crate) fn patch_section(operation: &str) -> Option<&'static str> {
    match operation {
        "rename_project" => Some("project"),
        "update_program" | "create_program" | "delete_program" | "move_program" => Some("programs"),
        "select_module" | "update_module" | "set_module_option" | "set_module_input_filter" => {
            Some("modules")
        }
        "insert_iec_local_symbol"
        | "delete_iec_local_symbol"
        | "update_variable"
        | "update_iec_local_symbol_address"
        | "rename_iec_local_symbol"
        | "update_iec_local_symbol_description"
        | "update_iec_local_symbol_type" => Some("variables"),
        "edit_browser_network" | "edit_fenet_field" | "edit_cnet_settings" => Some("networks"),
        "insert_iec_ld_no_contact"
        | "insert_iec_ld_contact"
        | "insert_iec_ld_leading_contact"
        | "insert_iec_ld_short_wire_contact"
        | "repair_iec_ld_horizontal_wire"
        | "insert_iec_ld_parallel_contact"
        | "insert_iec_ld_parallel_contact_kind"
        | "insert_iec_ld_comment"
        | "insert_iec_ld_blank_row"
        | "delete_iec_ld_blank_row"
        | "insert_iec_ld_linear_rung"
        | "insert_iec_ld_rung"
        | "delete_iec_ld_linear_rung"
        | "delete_iec_ld_rung"
        | "insert_ladder_row"
        | "edit_text_program"
        | "update_ladder_cell_text"
        | "update_iec_ld_function_operand"
        | "replace_sfc_sequence"
        | "edit_sfc_variable"
        | "update_iec_ld_comment"
        | "update_iec_ld_rising_contact_operand"
        | "update_iec_ld_element_operand"
        | "update_iec_ld_contact_kind"
        | "update_iec_ld_coil_kind"
        | "edit_iec_ld_vertical_wire"
        | "insert_iec_ld_single_element"
        | "delete_iec_ld_terminal_coil"
        | "insert_iec_ld_terminal_coil"
        | "delete_iec_ld_contact"
        | "delete_iec_ld_no_contact"
        | "delete_ladder_horizontal_wire"
        | "delete_ladder_vertical_wire"
        | "delete_iec_ld_horizontal_wire_record"
        | "delete_iec_ld_isolated_element"
        | "update_iec_ld_arithmetic_function"
        | "update_iec_ld_comparison_function"
        | "edit_ladder_cell"
        | "insert_ladder_instruction"
        | "insert_ladder_comparison"
        | "delete_ladder_comparison"
        | "delete_ladder_instruction"
        | "edit_ladder_comment"
        | "delete_ladder_rung_comment"
        | "edit_ladder_branch"
        | "edit_sfc_entity" => Some("program"),
        _ => None,
    }
}

pub(crate) fn tool_name(operation: &str) -> Option<String> {
    if operation == "generate_io_variables" {
        return Some("generate_io_variables".into());
    }
    patch_section(operation)
        .is_none()
        .then(|| format!("xgwx_{operation}"))
}

pub(crate) fn editing_route(operation: &str) -> Value {
    if let Some(section) = patch_section(operation) {
        json!({"tool":"apply_patch", "export_tool":"get_ir", "section":section,
            "coverage":if patch_section(operation).is_some() {"complete"} else {"partial"},
            "direct_tool":tool_name(operation),
            "excluded_fields":if operation == "update_module" {json!(["id","sub_type","name","details"])} else if operation == "create_program" {json!(["object_id","symbol_id"])} else if operation == "update_ladder_cell_text" {json!(["opaque_or_unclassified_string_records"])} else {json!([])},
            "instruction":"Export the current IR and patch supported editable statements. Preserve readonly fields. A remaining direct tool covers capabilities the IR cannot express."})
    } else {
        json!({"tool":tool_name(operation), "coverage":"direct_only"})
    }
}

/// Public batch tools accept only independently exposed gap writers. IR writers
/// stay available internally to FileStore, never through an operation selector.
pub(crate) fn validate_direct_operations(operations: &[EditOperation]) -> Result<(), StoreError> {
    for (index, operation) in operations.iter().enumerate() {
        let mut error = if let Some(section) = patch_section(&operation.name) {
            StoreError::new(
                "patch_required",
                format!(
                    "{} is edited through get_ir(section:{section}) and apply_patch",
                    operation.name
                ),
            )
        } else if !operations::OPERATIONS
            .iter()
            .any(|known| known.name == operation.name)
        {
            StoreError::new(
                "unknown_operation",
                "operation is not an independently exposed writer",
            )
        } else {
            continue;
        };
        error.operation_index = Some(index);
        error.operation_name = Some(operation.name.clone());
        return Err(error);
    }
    Ok(())
}

fn arguments(operation: &OperationDefinition, definitions: &mut Map<String, Value>) -> Value {
    let mut schema = (operation.schema)();
    let object = schema.as_object_mut().expect("writer object schema");
    object.remove("$schema");
    if let Some(Value::Object(values)) = object.remove("$defs") {
        for (name, value) in values {
            if let Some(previous) = definitions.insert(name.clone(), value.clone()) {
                assert_eq!(previous, value, "conflicting writer definition: {name}");
            }
        }
    }
    schema
}

fn writer_schema(operation: &OperationDefinition) -> Map<String, Value> {
    let mut definitions = Map::new();
    let args = arguments(operation, &mut definitions);
    json!({
        "type":"object", "additionalProperties":false,
        "properties":{
            "path":{"type":"string", "description":"Project file within the configured allowed roots."},
            "expected_revision":{"type":"string", "description":"Current SHA-256 revision from inspection."},
            "args":args,
            "dry_run":{"type":"boolean", "default":false, "description":"Verify the candidate without disk changes."}
        },
        "required":["path","expected_revision","args"],
        "$defs":definitions
    }).as_object().unwrap().clone()
}

/// Closed batch union: every accepted gap-writer name has an independently exposed tool
/// and its actual Serde/Schemars argument schema, rather than arbitrary JSON.
fn batch_items() -> (Value, Map<String, Value>) {
    let mut definitions = Map::new();
    let variants: Vec<_> = operations::OPERATIONS
        .iter()
        .filter(|operation| tool_name(operation.name).is_some())
        .map(|operation| {
            let args = arguments(operation, &mut definitions);
            json!({
                "type":"object", "additionalProperties":false,
                "properties":{"name":{"type":"string","const":operation.name},"args":args},
                "required":["name","args"]
            })
        })
        .collect();
    (json!({"oneOf":variants}), definitions)
}

pub(crate) fn register(mut router: ToolRouter<XgwxServer>) -> ToolRouter<XgwxServer> {
    let (items, definitions) = batch_items();
    for name in ["apply_edits", "create_file"] {
        let route = router.map.get_mut(name).expect("batch tool route");
        let schema = std::sync::Arc::make_mut(&mut route.attr.input_schema);
        schema["properties"]["operations"]["items"] = items.clone();
        let defs = schema
            .entry("$defs")
            .or_insert_with(|| json!({}))
            .as_object_mut()
            .unwrap();
        for (name, value) in &definitions {
            if let Some(previous) = defs.insert(name.clone(), value.clone()) {
                assert_eq!(&previous, value, "conflicting batch definition: {name}");
            }
        }
    }
    for operation in operations::OPERATIONS {
        let Some(name) = tool_name(operation.name) else {
            continue;
        };
        if operation.name == "generate_io_variables" {
            continue;
        }
        assert!(
            !router.map.contains_key(name.as_str()),
            "duplicate writer tool {name}"
        );
        let details = operations::describe_operation(operation.name).expect("registered operation");
        let constraints = details["constraints"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(Value::as_str)
            .collect::<Vec<_>>()
            .join(" ");
        let description = format!(
            "{} {} Requires inspection and the current revision. Saves one verified transaction with an adjacent backup and atomic replacement. dry_run performs no disk changes. Re-inspect after structural edits.",
            operation.description, constraints
        );
        let tool = Tool::new(name, description, writer_schema(operation)).with_annotations(
            ToolAnnotations::from_raw(None, Some(false), Some(true), Some(false), Some(false)),
        );
        router.add_route(ToolRoute::<XgwxServer>::new_dyn(tool, move |context| {
            let store = context.service.file_store();
            let request = serde_json::from_value::<WriterRequest>(Value::Object(
                context.arguments.unwrap_or_default(),
            ));
            Box::pin(async move {
                Ok(blocking(move || {
                    let request = request.map_err(|e| StoreError::new("invalid_arguments", e))?;
                    store.apply(
                        &request.path,
                        &request.expected_revision,
                        &[EditOperation {
                            name: operation.name.into(),
                            args: request.args,
                        }],
                        request.dry_run,
                    )
                })
                .await
                .into())
            })
        }));
    }
    router
}
