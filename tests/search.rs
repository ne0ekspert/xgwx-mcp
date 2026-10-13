#[path = "../src/test_support.rs"]
mod test_support;
use serde_json::{Value, json};
use test_support::fixture_root;
use xgwx::XgwxDocument;
use xgwx_mcp::{
    operations::apply_operation,
    search::{SearchKind, SearchRequest, search},
};

fn fixture(name: &str) -> XgwxDocument {
    XgwxDocument::from_path(fixture_root().join("fixtures").join(name)).unwrap()
}
fn request(query: &str) -> SearchRequest {
    serde_json::from_value(json!({"path":"unused.xgwx","query":query})).unwrap()
}
fn items(value: &Value) -> &[Value] {
    value["items"].as_array().unwrap()
}

#[test]
fn description_search_resolves_operand_uses_and_preserves_document() {
    let mut doc = fixture("elements.xgwx");
    let variable = doc
        .variable()
        .variables()
        .unwrap()
        .iter()
        .enumerate()
        .find(|(_, v)| {
            v.address.as_ref().is_some_and(|address| {
                doc.program()
                    .ladder_program(0)
                    .unwrap()
                    .unwrap()
                    .structure
                    .rungs
                    .iter()
                    .flat_map(|r| &r.cells)
                    .any(|c| c.contact.is_some() && c.value.eq_ignore_ascii_case(address))
            })
        })
        .map(|(i, v)| (i, v.address.clone().unwrap()))
        .expect("fixture contact with global symbol");
    apply_operation(
        &mut doc,
        "update_variable",
        json!({"variable_index":variable.0,"patch":{"description":"Feed conveyor 한글"}}),
    )
    .unwrap();
    let before = doc.to_verified_bytes().unwrap();
    let found = search(&doc, &request("FEED CONVEYOR")).unwrap();
    assert!(
        items(&found)
            .iter()
            .any(|i| i["kind"] == "variable" && i["variable_index"] == variable.0)
    );
    let contact = items(&found)
        .iter()
        .find(|i| i["kind"] == "contact")
        .expect("description finds contact through symbol");
    assert_eq!(contact["program_index"], 0);
    assert!(contact["location"]["raw_x"].is_number());
    assert!(contact["location"]["rung_index"].is_number());
    assert!(
        contact["matched_fields"]
            .as_array()
            .unwrap()
            .contains(&json!("symbols"))
    );
    assert_eq!(contact["symbols"][0]["address"], variable.1);
    assert_eq!(doc.to_verified_bytes().unwrap(), before);
    let mut sensitive = request("FEED CONVEYOR");
    sensitive.case_sensitive = true;
    assert_eq!(search(&doc, &sensitive).unwrap()["total"], 0);
    assert!(
        search(&doc, &request("한글")).unwrap()["total"]
            .as_u64()
            .unwrap()
            > 0
    );
}

#[test]
fn repeated_locations_kind_filters_and_single_result_pagination() {
    let mut doc = fixture("elements.xgwx");
    let program = doc.program().ladder_program(0).unwrap().unwrap();
    let contacts = program
        .structure
        .rungs
        .iter()
        .flat_map(|r| &r.cells)
        .filter(|c| c.contact.is_some() && !c.value.is_empty())
        .collect::<Vec<_>>();
    assert!(contacts.len() >= 2);
    let repeated = contacts[0].value.clone();
    apply_operation(&mut doc, "update_ladder_cell_text", json!({"program_index":0,"offset":contacts[1].offset,"expected":contacts[1].value,"replacement":repeated})).unwrap();
    let mut req = request(&repeated);
    req.kind = Some(SearchKind::Contact);
    let all = search(&doc, &req).unwrap();
    assert!(items(&all).len() > 1);
    assert!(items(&all).iter().all(|i| i["kind"] == "contact"));
    let mut offsets = std::collections::BTreeSet::new();
    for item in items(&all) {
        assert!(offsets.insert(item["location"]["string_offset"].as_u64().unwrap()));
    }
    req.limit = 1;
    let mut collected = vec![];
    loop {
        let page = search(&doc, &req).unwrap();
        assert_eq!(page["total"], all["total"]);
        collected.extend_from_slice(items(&page));
        if let Some(next) = page["next_offset"].as_u64() {
            req.offset = next as usize;
        } else {
            break;
        }
    }
    assert_eq!(collected, items(&all));
    req.offset = usize::MAX;
    let beyond = search(&doc, &req).unwrap();
    assert!(items(&beyond).is_empty());
    assert!(beyond["next_offset"].is_null());
    assert_eq!(
        search(&doc, &request("not-present-marker-12345")).unwrap()["total"],
        0
    );
    assert!(search(&doc, &request("   ")).is_err());
    req.limit = 0;
    assert!(search(&doc, &req).is_err());
    req.limit = 1001;
    assert!(search(&doc, &req).is_err());
    req.limit = 1;
    req.program_index = Some(usize::MAX);
    assert!(search(&doc, &req).is_err());
}

#[test]
fn linked_alias_descriptions_keep_all_address_candidates_and_page_order() {
    let mut doc = fixture("elements.xgwx");
    let variables = doc.variable().variables().unwrap();
    let program = doc.program().ladder_program(0).unwrap().unwrap();
    let contacts = program
        .structure
        .rungs
        .iter()
        .flat_map(|r| &r.cells)
        .filter(|c| c.contact.is_some() && !c.value.is_empty())
        .collect::<Vec<_>>();
    let (target_index, target) = variables
        .iter()
        .enumerate()
        .find(|(_, variable)| {
            variable.address.as_ref().is_some_and(|address| {
                contacts
                    .iter()
                    .any(|contact| contact.value.eq_ignore_ascii_case(address))
            })
        })
        .unwrap();
    let alias_index = (0..variables.len()).find(|&i| i != target_index).unwrap();
    let address = target.address.as_ref().unwrap();
    let other_contact = contacts
        .iter()
        .find(|c| !c.value.eq_ignore_ascii_case(address))
        .unwrap();
    apply_operation(
        &mut doc,
        "update_ladder_cell_text",
        json!({
        "program_index":0,"offset":other_contact.offset,"expected":other_contact.value,
        "replacement":address}),
    )
    .unwrap();
    let marker = "Indexed address alias 한글";
    apply_operation(
        &mut doc,
        "update_variable",
        json!({
        "variable_index":target_index,"patch":{"description":marker}}),
    )
    .unwrap();
    apply_operation(
        &mut doc,
        "update_variable",
        json!({
        "variable_index":alias_index,"patch":{"address_area":target.address_area,
        "address_number":target.address_number,"data_type":target.data_type,"description":marker}}),
    )
    .unwrap();
    let mut req = request(marker);
    req.kind = Some(SearchKind::Contact);
    let all = search(&doc, &req).unwrap();
    assert!(items(&all).len() >= 2);
    let expected_indices = doc
        .variable()
        .variables()
        .unwrap()
        .iter()
        .enumerate()
        .filter(|(_, variable)| {
            [variable.name.as_deref(), variable.address.as_deref()]
                .into_iter()
                .flatten()
                .any(|value| !value.is_empty() && value.to_lowercase() == address.to_lowercase())
        })
        .map(|(i, _)| json!(i))
        .collect::<Vec<_>>();
    assert!(expected_indices.contains(&json!(target_index)));
    assert!(expected_indices.contains(&json!(alias_index)));
    for contact in items(&all) {
        assert_eq!(
            contact["symbols"]
                .as_array()
                .unwrap()
                .iter()
                .map(|symbol| symbol["variable_index"].clone())
                .collect::<Vec<_>>(),
            expected_indices
        );
    }
    req.limit = 1;
    let mut collected = vec![];
    loop {
        let page = search(&doc, &req).unwrap();
        assert_eq!(page["total"], all["total"]);
        collected.extend_from_slice(items(&page));
        if let Some(next) = page["next_offset"].as_u64() {
            req.offset = next as usize;
        } else {
            break;
        }
    }
    assert_eq!(collected, items(&all));
}

#[test]
fn iec_contacts_and_functions_retain_native_locations() {
    let doc = fixture("ladder-wire-delete/iec-horizontal-native.xgwx");
    let program = doc.program().ladder_program(0).unwrap().unwrap();
    let frame = program
        .iec_record_frames()
        .unwrap()
        .into_iter()
        .find(|r| matches!(r.kind, xgwx::IecRecordKind::Contact(_)))
        .unwrap();
    let operand = &program
        .strings
        .iter()
        .find(|s| s.offset == frame.offset + 15)
        .unwrap()
        .value;
    let mut req = request(operand);
    req.kind = Some(SearchKind::Contact);
    let found = search(&doc, &req).unwrap();
    let contact = items(&found)
        .iter()
        .find(|i| i["location"]["record_offset"] == frame.offset)
        .unwrap();
    assert_eq!(contact["location"]["group_index"], frame.group_index);
    assert_eq!(contact["location"]["row_index"], frame.row_index);
    assert_eq!(contact["location"]["string_offset"], frame.offset + 15);
    let doc = fixture("function-output-wires/base.xgwx");
    let function = doc
        .program()
        .ladder_program(0)
        .unwrap()
        .unwrap()
        .iec_function_blocks()
        .unwrap()
        .remove(0);
    let mut req = request(&function.name.value);
    req.kind = Some(SearchKind::Instruction);
    let found = search(&doc, &req).unwrap();
    assert!(
        items(&found)
            .iter()
            .any(|i| i["mnemonic"] == function.name.value
                && i["location"]["record_offset"] == function.record_offset)
    );
}

#[test]
fn programs_instructions_comments_and_coverage_are_explicit() {
    let doc = fixture("elements.xgwx");
    let summaries = doc.program().programs();
    let name = summaries[0].name.as_deref().unwrap();
    let mut req = request(name);
    req.kind = Some(SearchKind::Program);
    req.program_index = Some(0);
    let found = search(&doc, &req).unwrap();
    assert_eq!(found["total"], 1);
    assert_eq!(
        found["items"][0]["object_id"],
        json!(summaries[0].object_id)
    );
    let program = doc.program().ladder_program(0).unwrap().unwrap();
    let coil = program
        .structure
        .rungs
        .iter()
        .flat_map(|r| &r.cells)
        .find(|c| c.coil.is_some() && !c.value.is_empty())
        .unwrap();
    let mut req = request(&coil.value);
    req.kind = Some(SearchKind::Coil);
    req.program_index = Some(0);
    assert!(
        items(&search(&doc, &req).unwrap())
            .iter()
            .any(|i| i["kind"] == "coil" && i["location"]["string_offset"] == coil.offset)
    );
    let instruction = &program.instructions[0];
    let mut req = request(&instruction.mnemonic);
    req.kind = Some(SearchKind::Instruction);
    assert!(
        items(&search(&doc, &req).unwrap())
            .iter()
            .any(|i| i["location"]["string_offset"] == instruction.offset)
    );
    let comment = &program.structure.rung_comments[0];
    let mut req = request(&comment.text);
    req.kind = Some(SearchKind::Comment);
    assert!(
        items(&search(&doc, &req).unwrap())
            .iter()
            .any(|i| i["text"] == comment.text && i["location"]["raw_y"] == comment.raw_y)
    );
    let doc = xgwx::create_project("XGI-CPUE", "ST").unwrap();
    let result = search(&doc, &request("not-present-marker")).unwrap();
    assert!(!result["limitations"].as_array().unwrap().is_empty());
}

#[test]
fn zero_operand_instruction_is_searchable() {
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    apply_operation(
        &mut doc,
        "insert_ladder_instruction",
        json!({"program_index":0,"raw_y":0,"mnemonic":"CLC","operands":[]}),
    )
    .unwrap();
    let mut req = request("clc");
    req.kind = Some(SearchKind::Instruction);
    let found = search(&doc, &req).unwrap();
    assert_eq!(found["total"], 1);
    assert_eq!(found["items"][0]["mnemonic"], "CLC");
    assert_eq!(found["items"][0]["location"]["raw_y"], 0);
    assert_eq!(found["items"][0]["operands"], json!([]));
}
