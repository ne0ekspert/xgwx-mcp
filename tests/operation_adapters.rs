#[cfg(not(feature = "libxgwx-namespaced-api"))]
use xgwx_mcp::library_compat::DocumentNamespacesCompat;
#[path = "../src/test_support.rs"]
mod test_support;
use serde_json::{Value, json};
use std::collections::BTreeSet;
use test_support::library_root;
use xgwx::XgwxDocument;
use xgwx_mcp::operation_types::{SfcEntity, SfcRow, SfcVariable};
use xgwx_mcp::operations::{OPERATIONS, apply_operation, describe_operation, list_operations};

fn fixture(path: &str) -> XgwxDocument {
    XgwxDocument::from_path(library_root().join("fixtures").join(path)).unwrap()
}

fn public_mutators(source: &str) -> BTreeSet<String> {
    let mut writers = BTreeSet::new();
    // Identify the bound-handle API independently of crate attributes or docs.
    let selected_api =
        source.contains("pub struct Program<'a>") && source.contains("pub struct IecLdEditor<'a>");
    for tail in source.split("pub fn ").skip(1) {
        let Some(signature_end) = tail.find("->") else {
            continue;
        };
        let signature = &tail[..signature_end];
        let Some(name) = signature.split('(').next().map(str::trim) else {
            continue;
        };
        // Exclusive borrows and editor selection do not mutate project contents.
        if matches!(name, "program_mut" | "variable_mut")
            || (selected_api
                && matches!(
                    name,
                    "get" | "iec_ld" | "ladder" | "text" | "sfc" | "vendor_il"
                ))
        {
            continue;
        }
        if !signature.contains("&mut self") && !(selected_api && name == "delete") {
            continue;
        }
        if !selected_api {
            writers.insert(name.to_owned());
            continue;
        }
        // Selected handles shorten names and bind indices. Audit every backend
        // invoked by a delegate so they remain covered by the indexed MCP catalog.
        let body_start = tail.find('{').expect("writer body");
        let mut depth = 0;
        let body_end = tail[body_start..]
            .char_indices()
            .find_map(|(offset, ch)| {
                if ch == '{' {
                    depth += 1;
                }
                if ch == '}' {
                    depth -= 1;
                }
                (depth == 0).then_some(body_start + offset)
            })
            .expect("writer body closes");
        let body = &tail[body_start..=body_end];
        let backends = OPERATIONS
            .iter()
            .filter(|operation| body.contains(&format!(".{}(", operation.name)))
            .map(|operation| operation.name.to_owned())
            .collect::<Vec<_>>();
        if !backends.is_empty() {
            writers.extend(backends);
        } else if name == "rename" && body.contains("self.update(") {
            writers.insert("update_program".into());
        } else {
            // A new mutator with no known backend must still fail the audit.
            writers.insert(name.to_owned());
        }
    }
    writers
}

#[test]
fn coverage_manifest_matches_all_public_writers_and_callable_adapters() {
    let source_root = library_root().join("src");
    let mut library_writers = BTreeSet::new();
    // Audit all source modules; a new public document writer must fail this check
    // even if it is introduced in a new module instead of the current four.
    for entry in std::fs::read_dir(source_root).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|extension| extension == "rs") {
            library_writers.extend(public_mutators(&std::fs::read_to_string(path).unwrap()));
        }
    }
    let catalog = OPERATIONS
        .iter()
        .map(|operation| operation.name.to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(OPERATIONS.len(), catalog.len(), "duplicate operation names");
    assert_eq!(library_writers, catalog, "public writer coverage drift");
    let manifest: Value = serde_json::from_str(include_str!("../operation-coverage.json")).unwrap();
    assert_eq!(
        manifest["public_mutator_count"].as_u64(),
        Some(catalog.len() as u64)
    );
    let entries = manifest["operations"].as_array().unwrap();
    assert_eq!(entries.len(), catalog.len());
    let manifested = entries
        .iter()
        .map(|entry| entry["public_writer"].as_str().unwrap().to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(manifested, catalog);
    for entry in entries {
        let name = entry["operation"].as_str().unwrap();
        let definition = OPERATIONS
            .iter()
            .find(|operation| operation.name == name)
            .unwrap();
        assert_eq!(entry["adapter"], definition.adapter);
        assert_eq!(entry["subsystem"], definition.subsystem);
        if let Some(relationship) = entry.get("compatibility_relationship") {
            let described = describe_operation(name).unwrap();
            assert_eq!(
                described["compatibility_relationship"]["preferred_operation"],
                relationship["preferred_operation"]
            );
            assert_eq!(described["compatibility_alias"], false);
        }
        let expected_source =
            std::fs::read_to_string(library_root().join(entry["source"].as_str().unwrap()))
                .unwrap();
        #[cfg(feature = "libxgwx-namespaced-api")]
        let expected_source = if public_mutators(&expected_source).contains(name) {
            expected_source
        } else {
            // The pinned manifest describes the old flat writers. The same
            // operation names are public on the namespace editors.
            std::fs::read_to_string(library_root().join("src/document_api.rs")).unwrap()
        };
        assert!(
            public_mutators(&expected_source).contains(name),
            "wrong manifest source for {name}"
        );
    }
}

#[test]
fn every_adapter_reaches_a_real_library_precondition_without_mutating_the_document() {
    let cases: Value =
        serde_json::from_str(include_str!("operation-precondition-cases.json")).unwrap();
    assert_eq!(cases.as_object().unwrap().len() + 1, OPERATIONS.len());
    for operation in OPERATIONS {
        if operation.name == "generate_io_variables" {
            continue;
        }
        let mut doc = fixture("empty-projects/new-xgk.xgwx");
        let before = doc.xml.clone();
        let result = apply_operation(&mut doc, operation.name, cases[operation.name].clone());
        let error = result.expect_err(operation.name);
        assert!(
            !error.starts_with("invalid arguments"),
            "adapter {} did not reach writer: {error}",
            operation.name
        );
        assert_eq!(
            doc.xml, before,
            "failed writer {} changed document",
            operation.name
        );
    }
    let mut doc = fixture("io-variable-generation/base.xgwx");
    apply_operation(&mut doc, "generate_io_variables", json!({})).unwrap();
    assert_eq!(doc.variable().variables().unwrap().len(), 120);
    doc.to_verified_bytes().unwrap();
}

#[test]
fn every_schema_is_strict_and_discoverable_and_filters_work() {
    let full = list_operations(None, None);
    assert_eq!(full["count"].as_u64(), Some(OPERATIONS.len() as u64));
    for operation in OPERATIONS {
        let described = describe_operation(operation.name).unwrap();
        let schema = &described["argument_schema"];
        assert_eq!(schema["type"], "object", "{}", operation.name);
        assert_eq!(schema["additionalProperties"], false, "{}", operation.name);
        for definition in schema["$defs"]
            .as_object()
            .into_iter()
            .flat_map(|defs| defs.values())
        {
            if definition["type"] == "object" && definition.get("properties").is_some() {
                assert_eq!(
                    definition["additionalProperties"], false,
                    "nested shape in {}",
                    operation.name
                );
            }
        }
        assert!(
            described["required_inspection"]
                .as_array()
                .is_some_and(|rows| !rows.is_empty())
        );
        assert!(described["examples"][0]["arguments"].is_object());
        if operation.adapter != operation.name {
            let manifest: Value =
                serde_json::from_str(include_str!("../operation-coverage.json")).unwrap();
            let entry = manifest["operations"]
                .as_array()
                .unwrap()
                .iter()
                .find(|entry| entry["operation"] == operation.name)
                .unwrap();
            assert_eq!(
                described["compatibility_defaults"],
                entry["compatibility_defaults"]
            );
        }
        let filtered = list_operations(Some(operation.name), Some(operation.subsystem));
        assert!(
            filtered["operations"]
                .as_array()
                .unwrap()
                .iter()
                .any(|item| item["name"] == operation.name)
        );
    }
    assert_eq!(list_operations(Some("no_such_operation"), None)["count"], 0);
    assert!(
        describe_operation("no_such_operation")
            .unwrap_err()
            .contains("unknown operation")
    );
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    assert!(
        apply_operation(
            &mut doc,
            "update_module",
            json!({"base":0,"slot":0,"patch":{"typo":true}})
        )
        .unwrap_err()
        .starts_with("invalid arguments")
    );
    assert!(
        apply_operation(
            &mut doc,
            "select_cpu",
            json!({"model":"XGK-CPUE","extra":true})
        )
        .unwrap_err()
        .starts_with("invalid arguments")
    );
    assert!(apply_operation(&mut doc, "edit_ladder_cell", json!({"program_index":0,"edit":{"raw_y":0,"column":0,"expected":null,"replacement":{"kind":"NormallyOpen","operand":"M00000","typo":0}}})).unwrap_err().starts_with("invalid arguments"));
}

#[test]
fn hardware_program_symbol_and_xgk_ladder_adapters_roundtrip_and_preserve_other_payloads() {
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    let original_program = doc.program().ladder_program(0).unwrap().unwrap().data;
    apply_operation(
        &mut doc,
        "insert_module",
        json!({"base":0,"slot":0,"model":"XGI-D21A"}),
    )
    .unwrap();
    apply_operation(&mut doc, "generate_io_variables", json!({})).unwrap();
    assert!(!doc.variable().variables().unwrap().is_empty());
    assert_eq!(
        doc.program().ladder_program(0).unwrap().unwrap().data,
        original_program
    );
    let original_variables = doc.variable().variables().unwrap();
    apply_operation(
        &mut doc,
        "create_program",
        json!({"patch":{
            "name":"AgentProgram","language":"LD",
            "object_id":"abcdef01-1234-4567-89ab-0123456789ab",
            "symbol_id":"abcdef02-1234-4567-89ab-0123456789ab"
        }}),
    )
    .unwrap();
    assert_eq!(doc.program().programs().len(), 2);
    assert_eq!(doc.variable().variables().unwrap(), original_variables);
    apply_operation(
        &mut doc,
        "update_program",
        json!({"program_index":1,"patch":{"comment":"Created via MCP"}}),
    )
    .unwrap();
    apply_operation(
        &mut doc,
        "edit_ladder_cell",
        json!({"program_index":1,"edit":{
            "raw_y":0,"column":0,"expected":null,
            "replacement":{"kind":"NormallyOpen","operand":"M00000"}
        }}),
    )
    .unwrap();
    apply_operation(
        &mut doc,
        "edit_ladder_cell",
        json!({"program_index":1,"edit":{
            "raw_y":0,"column":9,"expected":null,
            "replacement":{"kind":"Output","operand":"M00001"}
        }}),
    )
    .unwrap();
    apply_operation(
        &mut doc,
        "edit_ladder_comment",
        json!({"program_index":1,"edit":{
            "kind":"Rung","raw_y":0,"expected":null,"replacement":"Agent test"
        }}),
    )
    .unwrap();
    let reparsed = XgwxDocument::parse(&doc.to_verified_bytes().unwrap()).unwrap();
    assert_eq!(reparsed.variable().variables().unwrap(), original_variables);
    assert_eq!(
        reparsed.program().ladder_program(0).unwrap().unwrap().data,
        original_program
    );
    assert_eq!(
        reparsed.program().ladder_program(1).unwrap().unwrap().data,
        doc.program().ladder_program(1).unwrap().unwrap().data
    );
}

#[test]
fn network_cnet_and_fenet_adapters_preserve_other_payloads() {
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    let original_program = doc.program().ladder_program(0).unwrap().unwrap().data;
    apply_operation(
        &mut doc,
        "insert_module",
        json!({"base":0,"slot":0,"model":"XGL-EFMT(B)"}),
    )
    .unwrap();
    apply_operation(&mut doc, "edit_fenet_field", json!({"patch":{
        "base":0,"slot":0,"field":"ipAddress","expected_value":"192.168.0.100","replacement":"10.20.30.40"
    }})).unwrap();
    assert_eq!(
        doc.network().fenet_config_infos()[0]
            .ip_address
            .as_ref()
            .unwrap()
            .address,
        "10.20.30.40"
    );
    apply_operation(
        &mut doc,
        "update_network",
        json!({"network_index":0,"patch":{"name":"Agent network"}}),
    )
    .unwrap();
    assert_eq!(
        doc.program().ladder_program(0).unwrap().unwrap().data,
        original_program
    );
    doc.to_verified_bytes().unwrap();
    let mut cnet = fixture("networks/cnetbase-native.xgwx");
    let original_program = cnet.program().ladder_program(0).unwrap().unwrap().data;
    apply_operation(&mut cnet, "edit_cnet_settings", json!({"patch":{
        "base":0,"slot":2,"changes":[{"port_index":0,"field":"stationNo","expected_value":"0","replacement":"1"}]
    }})).unwrap();
    assert_eq!(
        cnet.network().cnet_config_infos()[0].ports[0].station_no,
        Some(1)
    );
    assert_eq!(
        cnet.program().ladder_program(0).unwrap().unwrap().data,
        original_program
    );
    cnet.to_verified_bytes().unwrap();
}

#[test]
fn iec_symbols_ladder_and_compatibility_aliases_roundtrip() {
    let mut doc = fixture("function-output-wires/base.xgwx");
    // Fixture-derived empty table with a text node, as required by the existing
    // local-symbol writer; the native zero-count fixture omits that node.
    doc.xml = doc.xml.replace(
        "<Symbols Count=\"0\"></Symbols>",
        "<Symbols Count=\"0\"> </Symbols>",
    );
    let mut doc = XgwxDocument::parse(&doc.to_bytes().unwrap()).unwrap();
    let last_row = doc
        .program()
        .ladder_program(0)
        .unwrap()
        .unwrap()
        .iec_row_frames()
        .unwrap()
        .iter()
        .map(|row| row.row_index)
        .max()
        .unwrap()
        + 2;
    apply_operation(
        &mut doc,
        "insert_iec_local_symbol",
        json!({"program_index":0,"name":"AgentContact","data_type":"BOOL","description":"Input"}),
    )
    .unwrap();
    apply_operation(
        &mut doc,
        "insert_iec_local_symbol",
        json!({"program_index":0,"name":"AgentOutput","data_type":"BOOL","description":"AgentOutput"}),
    )
    .unwrap();
    apply_operation(&mut doc, "insert_iec_ld_linear_rung", json!({"program_index":0,"blank_row_index":last_row,"contact_variable":"AgentContact","coil_variable":"AgentOutput"})).unwrap();
    let output = apply_operation(&mut doc, "insert_iec_ld_parallel_contact", json!({"program_index":0,"row_index":last_row,"expected_contact_variable":"AgentContact","expected_coil_variable":"AgentOutput","parallel_variable":"AgentContact"})).unwrap();
    assert_eq!(output["adapter"], "insert_iec_ld_parallel_contact_kind");
    let reparsed = XgwxDocument::parse(&doc.to_verified_bytes().unwrap()).unwrap();
    let locals = reparsed.variable().iec_local_symbols().remove(0).unwrap();
    assert!(locals.iter().any(|symbol| symbol.name == "AgentContact"));
    assert!(locals.iter().any(|symbol| symbol.name == "AgentOutput"));
    assert_eq!(
        reparsed.program().ladder_program(0).unwrap().unwrap().data,
        doc.program().ladder_program(0).unwrap().unwrap().data
    );
}

#[test]
fn sfc_patch_snapshots_use_exact_inspection_shapes_and_roundtrip() {
    let mut doc = fixture("sfc/native-loop.xgwx");
    let locals = doc.variable().sfc_variables(0).unwrap();
    apply_operation(
        &mut doc,
        "edit_sfc_entity",
        json!({"patch":{
            "program_index":0,"block_index":0,"entity_index":3,
            "expected_type":0,"expected_row":3,"expected_column":0,
            "field":"comment","expected_value":"Ready for cycle","replacement":"Agent chart"
        }}),
    )
    .unwrap();
    assert_eq!(doc.variable().sfc_variables(0).unwrap(), locals);
    let chart = doc.program().sfc_programs().remove(0).blocks.remove(0);
    let rows: Vec<SfcRow> = chart
        .editable_rows
        .unwrap()
        .into_iter()
        .map(Into::into)
        .collect();
    let entities: Vec<SfcEntity> = chart.entities.into_iter().map(Into::into).collect();
    apply_operation(&mut doc, "replace_sfc_sequence", json!({"patch":{
        "program_index":0,"block_index":0,"expected_entities":entities,"expected_rows":rows,"rows":rows
    }})).unwrap();
    let variables: Vec<SfcVariable> = doc
        .variable()
        .sfc_variables(0)
        .unwrap()
        .into_iter()
        .map(Into::into)
        .collect();
    apply_operation(&mut doc, "edit_sfc_variable", json!({"patch":{
        "program_index":0,"expected_variables":variables,"name":"AgentFlag","data_type":"BOOL","description":"Agent chart variable","remove":false
    }})).unwrap();
    let reparsed = XgwxDocument::parse(&doc.to_verified_bytes().unwrap()).unwrap();
    assert!(
        reparsed
            .variable()
            .sfc_variables(0)
            .unwrap()
            .iter()
            .any(|variable| variable.name == "AgentFlag")
    );
    assert_eq!(
        reparsed.program().sfc_programs()[0].blocks[0].entities[3].properties["EntityStep"]["Comment"],
        "Agent chart"
    );
}
