#[path = "../src/test_support.rs"]
mod test_support;
use serde_json::{Value, json};
use std::collections::BTreeSet;
use test_support::library_root;
use xgwx::XgwxDocument;
use xgwx_mcp::{
    operations::{apply_operation, describe_operation},
    storage::revision,
};
#[test]
fn executable_examples_succeed_reparse_and_preserve_unrelated_payloads() {
    let catalog: Value =
        serde_json::from_str(include_str!("../examples/verified-operation-cases.json")).unwrap();
    let mut names = BTreeSet::new();
    for case in catalog["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let path = library_root()
            .join("fixtures")
            .join(case["fixture"].as_str().unwrap());
        let bytes = std::fs::read(path).unwrap();
        assert_eq!(
            json!(revision(&bytes)),
            case["fixture_sha256"],
            "fixture changed: {id}"
        );
        let mut doc = XgwxDocument::parse(&bytes).unwrap();
        let global_symbols = doc.variable().variables().unwrap();
        let retained = case["preserve_programs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| {
                let index = v.as_u64().unwrap() as usize;
                (
                    index,
                    doc.program().ladder_program(index).unwrap().unwrap().data,
                )
            })
            .collect::<Vec<_>>();
        for step in case["setup"]
            .as_array()
            .unwrap()
            .iter()
            .chain(case["operations"].as_array().unwrap())
        {
            let name = step["name"].as_str().unwrap();
            names.insert(name);
            apply_operation(&mut doc, name, step["args"].clone())
                .unwrap_or_else(|e| panic!("{id}/{name}: {e}"));
            let reparsed = XgwxDocument::parse(&doc.to_verified_bytes().unwrap()).unwrap();
            assert_eq!(doc.xml, reparsed.xml, "{id}/{name}");
            assert_eq!(
                reparsed.variable().variables().unwrap(),
                global_symbols,
                "{id}/{name}: unrelated symbols"
            );
            for (index, data) in &retained {
                assert_eq!(
                    reparsed
                        .program()
                        .ladder_program(*index)
                        .unwrap()
                        .unwrap()
                        .data,
                    *data,
                    "{id}/{name}: unrelated program"
                );
            }
        }
        let expected = &case["expect"];
        if let Some(index) = expected["program_index"].as_u64() {
            let program = doc
                .program()
                .ladder_program(index as usize)
                .unwrap()
                .unwrap();
            let cells = program
                .structure
                .rungs
                .iter()
                .flat_map(|r| &r.cells)
                .collect::<Vec<_>>();
            for (key, actual) in [
                ("cell_count", cells.len()),
                ("rung_comment_count", program.structure.rung_comments.len()),
                (
                    "vertical_line_count",
                    program.structure.vertical_lines.len(),
                ),
            ] {
                if !expected[key].is_null() {
                    assert_eq!(json!(actual), expected[key], "{id}/{key}");
                }
            }
            if let Some(values) = expected["output_comments"].as_array() {
                assert_eq!(
                    json!(
                        program
                            .structure
                            .output_comments
                            .iter()
                            .map(|c| &c.text)
                            .collect::<Vec<_>>()
                    ),
                    json!(values),
                    "{id}"
                );
            }
            if let Some(values) = expected["instruction_values"].as_array() {
                for value in values {
                    assert!(
                        program.strings.iter().any(|s| json!(s.value) == *value),
                        "{id}: instruction"
                    );
                }
            }
            if let Some(value) = expected["comparison_value"].as_str() {
                assert!(
                    program.strings.iter().any(|s| s.value == value),
                    "{id}: comparison"
                );
            }
            if !expected["iec_contact_count"].is_null() {
                let records = program.iec_record_frames().unwrap();
                assert_eq!(
                    json!(
                        records
                            .iter()
                            .filter(|r| matches!(r.kind, xgwx::IecRecordKind::Contact(_)))
                            .count()
                    ),
                    expected["iec_contact_count"]
                );
                assert_eq!(
                    json!(
                        records
                            .iter()
                            .filter(|r| matches!(r.kind, xgwx::IecRecordKind::Coil(_)))
                            .count()
                    ),
                    expected["iec_coil_count"]
                );
            }
        }
        if !expected["program_count"].is_null() {
            assert_eq!(
                json!(doc.program().programs().len()),
                expected["program_count"]
            );
            assert_eq!(
                json!(doc.program().programs().last().unwrap().name),
                expected["last_program_name"]
            );
        }
        if let Some(ip) = expected["fenet_ip"].as_str() {
            assert_eq!(
                doc.network().fenet_config_infos()[0]
                    .ip_address
                    .as_ref()
                    .unwrap()
                    .address,
                ip
            );
        }
        if let Some(name) = expected["network_name"].as_str() {
            assert_eq!(doc.networks()[0].name.as_deref(), Some(name));
        }
        if let Some(comment) = expected["sfc_step_comment"].as_str() {
            assert_eq!(
                doc.program().sfc_programs()[0].blocks[0].entities[3].properties["EntityStep"]["Comment"],
                comment
            );
        }
        if let Some(comment) = expected["module_comment"].as_str() {
            assert!(
                doc.module_summaries()
                    .iter()
                    .any(|m| m.comment.as_deref() == Some(comment))
            );
        }
    }
    assert!(names.len() >= 15);
    for name in names {
        let described = describe_operation(name).unwrap();
        let count = catalog["cases"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|case| {
                case["setup"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .chain(case["operations"].as_array().unwrap())
            })
            .filter(|step| step["name"] == name)
            .count();
        assert_eq!(
            described["validation_evidence"]["bundled_fixture_example_count"],
            json!(count)
        );
        let examples = described["examples"].as_array().unwrap();
        assert_eq!(
            examples
                .iter()
                .filter(|example| example["kind"] == "fixture_verified")
                .count(),
            count.min(2)
        );
        assert_eq!(examples.last().unwrap()["kind"], "argument_template");
    }
}
