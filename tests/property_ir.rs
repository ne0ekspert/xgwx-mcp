#[path = "../src/test_support.rs"]
mod test_support;
use serde_json::json;
use std::fs;
use test_support::fixture_root;
use xgwx::XgwxDocument;
use xgwx_mcp::{
    property_ir::Projection,
    storage::{FileStore, revision},
};
fn fixture(name: &str) -> XgwxDocument {
    XgwxDocument::from_path(fixture_root().join("fixtures").join(name)).unwrap()
}
fn patch(text: &str, changes: &[(&str, &str)]) -> String {
    patch_from(text, changes, 0)
}
fn patch_from(text: &str, changes: &[(&str, &str)], start: usize) -> String {
    let mut pairs = changes
        .iter()
        .map(|(old, new)| {
            let i = text
                .lines()
                .enumerate()
                .skip(start)
                .find(|(_, l)| *l == *old)
                .unwrap()
                .0;
            (i, *old, *new)
        })
        .collect::<Vec<_>>();
    pairs.sort_by_key(|p| p.0);
    let mut p = "--- properties\n+++ properties\n".to_owned();
    for (i, old, new) in pairs {
        p.push_str(&format!(
            "@@ -{},1 +{},1 @@\n-{old}\n+{new}\n",
            i + 1,
            i + 1
        ));
    }
    p
}
fn apply(doc: &mut XgwxDocument, section: &str, program: Option<usize>, changes: &[(&str, &str)]) {
    let ir = Projection::export(doc, "revision", section, program).unwrap();
    for op in ir.lower_patch(&patch(&ir.text, changes)).unwrap() {
        xgwx_mcp::operations::apply_operation(doc, &op.name, op.args).unwrap();
    }
    doc.to_verified_bytes().unwrap();
}
#[test]
fn module_model_selection_comments_and_decoded_options_preserve_other_payloads() {
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    doc.base_mut()
        .by_number(0)
        .unwrap()
        .insert_module(0, "XGI-D24A/B")
        .unwrap();
    doc.base_mut()
        .by_number(0)
        .unwrap()
        .insert_module(1, "XGL-EFMT(B)")
        .unwrap();
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, "modules", None).unwrap();
    for key in ["id", "sub_type", "details"] {
        assert!(!ir.text.lines().any(|l| l.contains(&format!(" {key} = "))));
    }
    assert!(!ir.text.contains("edit native_name"));
    assert!(ir.text.contains("edit model = \"XGI-D24A/B\""));
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let diff = patch(
        &ir.text,
        &[
            (
                "  edit comment = \"\"",
                "  edit comment = \"Feed & conveyor\"",
            ),
            (
                "  edit option.inputFilter.0 = 0",
                "  edit option.inputFilter.0 = 3",
            ),
        ],
    );
    store
        .apply_property_patch("p.xgwx", &rev, "modules", None, &diff, false)
        .unwrap();
    doc = XgwxDocument::from_path(dir.path().join("p.xgwx")).unwrap();
    let programs = doc
        .program()
        .ladder_programs()
        .into_iter()
        .map(|p| p.unwrap().data)
        .collect::<Vec<_>>();
    let variables = doc.variable().variables().unwrap();
    let networks = doc.network().list();
    let fenet = doc.network().fenet_config_infos();
    let before = doc.module().summaries();
    let replacement = xgwx::xgk_module_catalog()
        .iter()
        .find(|e| {
            e.model.starts_with("XGQ-") && e.slot_span == 1 && {
                let mut candidate = doc.clone();
                candidate
                    .module_mut()
                    .at(0, 0)
                    .and_then(|mut m| m.select_model(e.model))
                    .is_ok()
            }
        })
        .unwrap();
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, "modules", None).unwrap();
    let old = "  edit model = \"XGI-D24A/B\"";
    let new = format!("  edit model = {}", json!(replacement.model));
    let diff = patch(&ir.text, &[(old, &new)]);
    let ops = ir.lower_patch(&diff).unwrap();
    assert_eq!(ops.len(), 1);
    assert_eq!(ops[0].name, "select_module");
    let preview = store
        .apply_property_patch("p.xgwx", &rev, "modules", None, &diff, true)
        .unwrap();
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    let invalid = patch(&ir.text, &[(old, "  edit model = \"UNKNOWN\"")]);
    assert_eq!(
        store
            .apply_property_patch("p.xgwx", &rev, "modules", None, &invalid, false)
            .unwrap_err()
            .code,
        "operation_failed"
    );
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    let mixed = patch(
        &ir.text,
        &[
            (old, &new),
            (
                "  edit comment = \"Feed & conveyor\"",
                "  edit comment = \"Mixed\"",
            ),
        ],
    );
    assert!(ir.lower_patch(&mixed).is_err());
    let native = ir
        .text
        .lines()
        .find(|l| l.starts_with("  readonly native_name = "))
        .unwrap();
    assert!(
        ir.lower_patch(&patch(
            &ir.text,
            &[(native, "  readonly native_name = \"Forged\"")]
        ))
        .is_err()
    );
    let injected = ir
        .text
        .replacen(native, &format!("  edit id = 1\n{native}"), 1);
    assert!(
        ir.lower_patch(&complete_patch(&ir.text, &injected))
            .is_err()
    );
    let saved = store
        .apply_property_patch("p.xgwx", &rev, "modules", None, &diff, false)
        .unwrap();
    assert_eq!(preview["revision_after"], saved["revision_after"]);
    assert_eq!(
        fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
        bytes
    );
    doc = XgwxDocument::from_path(dir.path().join("p.xgwx")).unwrap();
    let after = doc.module().summaries();
    assert_eq!(after[0].id, Some(replacement.id));
    assert_eq!(after[0].sub_type, Some(replacement.sub_type));
    assert_eq!(after[0].name.as_deref(), Some(replacement.name));
    assert_eq!(after[0].details.as_deref(), Some(replacement.details));
    assert_eq!(after[0].comment, before[0].comment);
    assert_eq!(after[1], before[1]);
    assert_eq!(
        doc.program()
            .ladder_programs()
            .into_iter()
            .map(|p| p.unwrap().data)
            .collect::<Vec<_>>(),
        programs
    );
    assert_eq!(doc.variable().variables().unwrap(), variables);
    assert_eq!(doc.network().list(), networks);
    assert_eq!(doc.network().fenet_config_infos(), fenet);
    assert_eq!(
        store
            .apply_property_patch("p.xgwx", &rev, "modules", None, &diff, false)
            .unwrap_err()
            .code,
        "stale_revision"
    );
}

#[test]
fn module_models_remain_readonly_when_native_module_coordinates_are_ambiguous() {
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    doc.base_mut()
        .by_number(0)
        .unwrap()
        .insert_module(0, "XGI-D24A/B")
        .unwrap();
    let xml = doc.container().xml().to_owned();
    let parsed = roxmltree::Document::parse(&xml).unwrap();
    let range = parsed
        .descendants()
        .find(|node| node.has_tag_name("Module"))
        .unwrap()
        .range();
    let module = &xml[range];
    doc.container_mut()
        .replace_xml(xml.replacen(module, &format!("{module}{module}"), 1))
        .unwrap();
    let ir = Projection::export(&doc, "revision", "modules", None).unwrap();
    assert!(!ir.text.contains("  edit native_name = "));
    assert!(!ir.text.contains("  edit model = "));
    assert!(ir.text.contains("  readonly native_name = "));
    assert!(!ir.text.lines().any(|line| line.contains(" details = ")));
}

#[test]
fn module_properties_preserve_program_and_symbols_and_enforce_catalog_values() {
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    doc.base_mut()
        .by_number(0)
        .and_then(|mut selected_record| selected_record.insert_module(0, "XGI-D24A/B"))
        .unwrap();
    doc.base_mut()
        .by_number(0)
        .and_then(|mut selected_record| selected_record.insert_module(1, "XGQ-RY2A/B"))
        .unwrap();
    let program = doc.program().ladder_program(0).unwrap().unwrap().data;
    let vars = doc.variable().variables().unwrap();
    apply(
        &mut doc,
        "modules",
        None,
        &[
            (
                "  edit option.inputFilter.0 = 0",
                "  edit option.inputFilter.0 = 3",
            ),
            ("  edit comment = \"\"", "  edit comment = \"Tank inputs\""),
        ],
    );
    assert_eq!(
        doc.module().summaries()[0].comment.as_deref(),
        Some("Tank inputs")
    );
    assert_eq!(
        doc.module()
            .at(0, 0)
            .and_then(|selected_record| selected_record.option_values())
            .unwrap()[0]
            .value,
        3
    );
    assert_eq!(
        doc.program().ladder_program(0).unwrap().unwrap().data,
        program
    );
    assert_eq!(doc.variable().variables().unwrap(), vars);
    let ir = Projection::export(&doc, "revision", "modules", None).unwrap();
    let ops = ir
        .lower_patch(&patch(
            &ir.text,
            &[(
                "  edit option.inputFilter.0 = 3",
                "  edit option.inputFilter.0 = 999",
            )],
        ))
        .unwrap();
    let before = doc.container().xml().to_owned();
    assert!(
        xgwx_mcp::operations::apply_operation(&mut doc, &ops[0].name, ops[0].args.clone()).is_err()
    );
    assert_eq!(doc.container().xml(), before);
}
#[test]
fn global_metadata_changes_coalesce_into_one_writer_and_preserve_program() {
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    doc.base_mut()
        .by_number(0)
        .and_then(|mut selected_record| selected_record.insert_module(0, "XGI-D21A"))
        .unwrap();
    doc.variable_mut().generate_io_variables().unwrap();
    let ir = Projection::export(&doc, "revision", "variables", None).unwrap();
    let vars = doc.variable().variables().unwrap();
    let old_name = format!("  edit name = {}", json!(vars[0].name));
    let old_desc = format!("  edit description = {}", json!(vars[0].description));
    let diff = patch(
        &ir.text,
        &[
            (&old_name, "  edit name = \"StartButton\""),
            (&old_desc, "  edit description = \"Start switch\""),
        ],
    );
    let ops = ir.lower_patch(&diff).unwrap();
    assert_eq!(ops.len(), 1);
    let program = doc.program().ladder_program(0).unwrap().unwrap().data;
    xgwx_mcp::operations::apply_operation(&mut doc, &ops[0].name, ops[0].args.clone()).unwrap();
    assert_eq!(
        doc.variable().variables().unwrap()[0].name.as_deref(),
        Some("StartButton")
    );
    assert_eq!(
        doc.variable().variables().unwrap()[0]
            .description
            .as_deref(),
        Some("Start switch")
    );
    assert_eq!(
        doc.program().ladder_program(0).unwrap().unwrap().data,
        program
    );
    doc.to_verified_bytes().unwrap();
}
#[test]
fn local_descriptions_and_single_rename_are_guarded_and_reexported() {
    let doc = fixture("function-output-wires/base.xgwx");
    let mut doc = XgwxDocument::parse(&doc.to_bytes().unwrap()).unwrap();
    // The public insertion writer needs the captured empty table's text node.
    {
        let replacement_xml = doc.container().xml().replace(
            "<Symbols Count=\"0\"></Symbols>",
            "<Symbols Count=\"0\"> </Symbols>",
        );
        doc.container_mut().replace_xml(replacement_xml).unwrap();
    };
    let mut doc = XgwxDocument::parse(&doc.to_bytes().unwrap()).unwrap();
    doc.program_mut()
        .get(0)
        .and_then(|mut selected_program| {
            selected_program
                .variable_mut()
                .insert("AgentContact", "BOOL", "Input")
        })
        .unwrap();
    apply(
        &mut doc,
        "variables",
        Some(0),
        &[(
            "  edit description = \"Input\"",
            "  edit description = \"Pump enable\"",
        )],
    );
    let ir = Projection::export(&doc, "revision", "variables", Some(0)).unwrap();
    let name = (
        "  edit name = \"AgentContact\"",
        "  edit name = \"PumpEnable\"",
    );
    let desc = (
        "  edit description = \"Pump enable\"",
        "  edit description = \"Changed\"",
    );
    assert!(ir.lower_patch(&patch(&ir.text, &[name, desc])).is_err());
    apply(&mut doc, "variables", Some(0), &[name]);
    assert!(
        doc.variable().iec_local_symbols()[0]
            .as_ref()
            .unwrap()
            .iter()
            .any(|v| v.name == "PumpEnable")
    );
    apply(
        &mut doc,
        "variables",
        Some(0),
        &[("  edit data_type = \"BOOL\"", "  edit data_type = \"WORD\"")],
    );
    assert_eq!(
        doc.variable().iec_local_symbols()[0].as_ref().unwrap()[0]
            .data_type
            .as_deref(),
        Some("WORD")
    );
}
#[test]
fn local_address_assignment_move_clear_and_native_guards() {
    let original = fixture("function-output-wires/base.xgwx");
    let xml = original.container().xml().replace(
        "<Symbols Count=\"0\"></Symbols>",
        "<Symbols Count=\"0\"> </Symbols>",
    );
    let mut doc = original;
    doc.container_mut().replace_xml(xml).unwrap();
    let mut doc = XgwxDocument::parse(&doc.to_bytes().unwrap()).unwrap();
    for name in ["AddressOne", "AddressTwo"] {
        doc.program_mut()
            .get(0)
            .and_then(|mut p| p.variable_mut().insert(name, "BOOL", "Address test"))
            .unwrap();
    }
    let programs = doc.program().ladder_program(0).unwrap().unwrap().data;
    let ir = Projection::export(&doc, "revision", "variables", Some(0)).unwrap();
    assert!(ir.text.contains("  edit address = \"\""));
    let diff = patch(
        &ir.text,
        &[("  edit address = \"\"", "  edit address = \"%MX100\"")],
    );
    let ops = ir.lower_patch(&diff).unwrap();
    assert_eq!(ops.len(), 1);
    assert_eq!(ops[0].name, "update_iec_local_symbol_address");
    assert_eq!(ops[0].args["expected_address"], "");
    xgwx_mcp::operations::apply_operation(&mut doc, &ops[0].name, ops[0].args.clone()).unwrap();
    let mut doc = XgwxDocument::parse(&doc.to_verified_bytes().unwrap()).unwrap();
    assert_eq!(
        doc.variable().iec_local_symbols()[0].as_ref().unwrap()[0]
            .address
            .as_deref(),
        Some("%MX100")
    );
    let ir = Projection::export(&doc, "revision", "variables", Some(0)).unwrap();
    // Allocation fields are derived, and declaration edits need re-exporting.
    assert!(
        ir.lower_patch(&patch(
            &ir.text,
            &[(
                "  readonly allocation_number = 100",
                "  readonly allocation_number = 200"
            )]
        ))
        .is_err()
    );
    assert!(
        ir.lower_patch(&patch(
            &ir.text,
            &[
                ("  edit address = \"%MX100\"", "  edit address = \"%MX200\""),
                ("  edit name = \"AddressOne\"", "  edit name = \"Renamed\"")
            ]
        ))
        .is_err()
    );
    for (old, new) in [("", "%MX100"), ("%MX100", "%IX0.1.0"), ("%MX100", "%MXbad")] {
        let ir = Projection::export(&doc, "revision", "variables", Some(0)).unwrap();
        let diff = patch(
            &ir.text,
            &[(
                &format!("  edit address = {}", json!(old)),
                &format!("  edit address = {}", json!(new)),
            )],
        );
        let ops = ir.lower_patch(&diff).unwrap();
        let before = doc.to_bytes().unwrap();
        assert!(
            xgwx_mcp::operations::apply_operation(&mut doc, &ops[0].name, ops[0].args.clone())
                .is_err()
        );
        assert_eq!(doc.to_bytes().unwrap(), before);
    }
    for (old, new) in [("%MX100", "%MX200"), ("%MX200", ""), ("", "%MX300")] {
        apply(
            &mut doc,
            "variables",
            Some(0),
            &[(
                &format!("  edit address = {}", json!(old)),
                &format!("  edit address = {}", json!(new)),
            )],
        );
        doc = XgwxDocument::parse(&doc.to_verified_bytes().unwrap()).unwrap();
        let symbols = doc.variable().iec_local_symbols();
        let symbol = &symbols[0].as_ref().unwrap()[0];
        assert_eq!(symbol.address.as_deref().unwrap_or(""), new);
        assert_eq!(
            symbol.allocation_number,
            if new.is_empty() {
                None
            } else {
                Some(new[3..].parse().unwrap())
            }
        );
        assert_eq!(symbol.storage_class, if new.is_empty() { "" } else { "M" });
    }
    assert_eq!(
        doc.program().ladder_program(0).unwrap().unwrap().data,
        programs
    );
}

#[test]
fn network_property_ir_covers_every_browser_metadata_and_fenet_field() {
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    doc.base_mut()
        .by_number(0)
        .unwrap()
        .insert_module(0, "XGL-EFMT(B)")
        .unwrap();
    let original = doc.to_verified_bytes().unwrap();
    let ir = Projection::export(&doc, &revision(&original), "networks", None).unwrap();
    let program = doc.program().ladder_program(0).unwrap().unwrap().data;
    let symbols = doc.variable().variables().unwrap();
    let hardware = doc.module().summaries();
    let start = ir
        .text
        .lines()
        .position(|line| line.starts_with("fenet "))
        .unwrap();
    for (field, replacement) in [
        ("ipAddress", "10.20.30.40"),
        ("subnet", "255.255.0.0"),
        ("gateway", "10.20.0.1"),
        ("dns", "10.20.0.2"),
        ("ipAddress2", "10.21.30.40"),
        ("subnet2", "255.255.0.0"),
        ("gateway2", "10.21.0.1"),
        ("dns2", "10.21.0.2"),
        ("dhcp", "1"),
        ("dhcp2", "1"),
        ("stationNo", "1"),
        ("glofaSocketCount", "8"),
        ("rcvWaitTime", "10"),
        ("clientWaitTime", "10"),
        ("driverType", "5"),
    ] {
        let prefix = format!("  edit {field} = ");
        let line = ir
            .text
            .lines()
            .skip(start)
            .find(|line| line.starts_with(&prefix));
        let Some(line) = line else {
            assert!(
                ir.text
                    .lines()
                    .skip(start)
                    .any(|line| line == format!("  readonly {field} = none")),
                "{field}"
            );
            let absent = format!("  readonly {field} = none");
            let invented = format!("  edit {field} = {}", json!(replacement));
            assert!(
                ir.lower_patch(&patch_from(&ir.text, &[(&absent, &invented)], start))
                    .is_err()
            );
            let mut candidate = doc.clone();
            assert!(xgwx_mcp::operations::apply_operation(&mut candidate, "edit_fenet_field", json!({"patch":{"base":0,"slot":0,"field":field,"expected_value":"","replacement":replacement}})).is_err(), "absent field {field} must also be unavailable through the writer");
            continue;
        };
        let expected: String = serde_json::from_str(line.strip_prefix(&prefix).unwrap()).unwrap();
        let changed = format!("{prefix}{}", json!(replacement));
        let ops = ir
            .lower_patch(&patch_from(&ir.text, &[(line, &changed)], start))
            .unwrap();
        let mut direct = doc.clone();
        xgwx_mcp::operations::apply_operation(&mut direct, "edit_fenet_field", json!({"patch":{"base":0,"slot":0,"field":field,"expected_value":expected,"replacement":replacement}})).unwrap();
        let mut patched = doc.clone();
        for op in ops {
            xgwx_mcp::operations::apply_operation(&mut patched, &op.name, op.args).unwrap();
        }
        assert_eq!(
            patched.to_verified_bytes().unwrap(),
            direct.to_verified_bytes().unwrap(),
            "field {field}"
        );
        assert_eq!(patched.module().summaries(), hardware);
        assert_eq!(
            patched.program().ladder_program(0).unwrap().unwrap().data,
            program
        );
        assert_eq!(patched.variable().variables().unwrap(), symbols);
        let next = Projection::export(&patched, "next", "networks", None).unwrap();
        assert!(
            next.text.contains(&changed),
            "field {field} was not re-exported"
        );
    }
    let module_start = ir
        .text
        .lines()
        .position(|line| line.starts_with("network_module "))
        .unwrap();
    for (key, field, start, module) in [
        ("name", "name", 0, false),
        ("config_name", "configName", module_start, true),
        ("alias", "alias", module_start, true),
        ("description", "description", module_start, true),
    ] {
        let prefix = format!("  edit {key} = ");
        let old = ir
            .text
            .lines()
            .skip(start)
            .find(|line| line.starts_with(&prefix))
            .unwrap();
        let expected: String = serde_json::from_str(old.strip_prefix(&prefix).unwrap()).unwrap();
        let replacement = "Plant & \"Line 1\" — 네트워크";
        let new = format!("{prefix}{}", json!(replacement));
        let ops = ir
            .lower_patch(&patch_from(&ir.text, &[(old, &new)], start))
            .unwrap();
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].name, "edit_browser_network");
        let mut direct = doc.clone();
        xgwx_mcp::operations::apply_operation(&mut direct, "edit_browser_network", json!({"patch":{"network_index":0,"module":module,"base":if module {Some(0)} else {None},"slot":if module {Some(0)} else {None},"field":field,"expected_value":expected,"replacement":replacement}})).unwrap();
        let mut patched = doc.clone();
        xgwx_mcp::operations::apply_operation(&mut patched, &ops[0].name, ops[0].args.clone())
            .unwrap();
        assert_eq!(
            patched.to_verified_bytes().unwrap(),
            direct.to_verified_bytes().unwrap()
        );
        assert_eq!(patched.module().summaries(), hardware);
        assert_eq!(
            patched.network().fenet_config_infos(),
            doc.network().fenet_config_infos()
        );
    }
}

#[test]
fn fenet_fields_are_readonly_when_the_configuration_scope_no_longer_matches() {
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    doc.base_mut()
        .by_number(0)
        .unwrap()
        .insert_module(0, "XGL-EFMT(B)")
        .unwrap();
    let before = Projection::export(&doc, "before", "networks", None).unwrap();
    let original_scope = doc.network().module_summaries()[0]
        .config_name
        .clone()
        .unwrap();
    assert!(before.text.contains("  edit ipAddress = "));
    xgwx_mcp::operations::apply_operation(
        &mut doc,
        "update_network_module",
        json!({"base":0,"slot":0,"patch":{"config_name":"No matching PLC configuration"}}),
    )
    .unwrap();
    let ir = Projection::export(&doc, "after", "networks", None).unwrap();
    let start = ir
        .text
        .lines()
        .position(|line| line.starts_with("fenet "))
        .unwrap();
    let fields = ir
        .text
        .lines()
        .skip(start)
        .take_while(|line| *line != "}")
        .collect::<Vec<_>>();
    assert!(
        fields
            .iter()
            .any(|line| line.contains("association_status = \"missing_or_ambiguous\""))
    );
    assert!(fields.iter().all(|line| !line.starts_with("  edit ")));
    let ip = fields
        .into_iter()
        .find(|line| line.starts_with("  readonly ipAddress = "))
        .unwrap();
    assert!(
        ir.lower_patch(&patch_from(
            &ir.text,
            &[(ip, "  edit ipAddress = \"10.20.30.40\"")],
            start
        ))
        .is_err()
    );
    let old = ir
        .text
        .lines()
        .find(|line| line.starts_with("  edit config_name = "))
        .unwrap();
    let restored = format!("  edit config_name = {}", json!(original_scope));
    for op in ir
        .lower_patch(&patch(&ir.text, &[(old, &restored)]))
        .unwrap()
    {
        xgwx_mcp::operations::apply_operation(&mut doc, &op.name, op.args).unwrap();
    }
    let repaired = Projection::export(&doc, "repaired", "networks", None).unwrap();
    assert!(repaired.text.contains("  edit ipAddress = "));
    assert!(
        !repaired
            .text
            .contains("association_status = \"missing_or_ambiguous\"")
    );
}

#[test]
fn network_property_ir_keeps_ambiguous_memberships_and_fenet_records_readonly() {
    let mut source = fixture("empty-projects/new-xgk.xgwx");
    source
        .base_mut()
        .by_number(0)
        .unwrap()
        .insert_module(0, "XGL-EFMT(B)")
        .unwrap();
    for tag in ["NetworkModule", "XGPD_CONFIG_INFO_FENET"] {
        let mut doc = source.clone();
        let xml = doc.container().xml().to_owned();
        let parsed = roxmltree::Document::parse(&xml).unwrap();
        let range = parsed
            .descendants()
            .find(|node| node.has_tag_name(tag))
            .unwrap()
            .range();
        let record = &xml[range];
        doc.container_mut()
            .replace_xml(xml.replacen(record, &format!("{record}{record}"), 1))
            .unwrap();
        let ir = Projection::export(&doc, "ambiguous", "networks", None).unwrap();
        assert!(!ir.text.contains("  edit ipAddress = "));
        assert!(
            ir.text
                .contains("association_status = \"missing_or_ambiguous\"")
        );
        if tag == "NetworkModule" {
            for key in ["config_name", "alias", "description"] {
                assert!(!ir.text.contains(&format!("  edit {key} = ")));
            }
        }
    }
}

#[test]
fn network_property_transactions_reject_bad_settings_and_preserve_frozen_identity() {
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    doc.base_mut()
        .by_number(0)
        .unwrap()
        .insert_module(0, "XGL-EFMT(B)")
        .unwrap();
    let original = doc.to_verified_bytes().unwrap();
    let revision = revision(&original);
    let ir = Projection::export(&doc, &revision, "networks", None).unwrap();
    let start = ir
        .text
        .lines()
        .position(|line| line.starts_with("fenet "))
        .unwrap();
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("p.xgwx"), &original).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let metadata = ir
        .text
        .lines()
        .find(|line| line.starts_with("  edit name = "))
        .unwrap();
    for (field, replacement) in [
        ("ipAddress", "999.1.2.3"),
        ("subnet", "255.0.255.0"),
        ("dhcp", "2"),
        ("stationNo", "1000"),
        ("glofaSocketCount", "0"),
        ("rcvWaitTime", "0"),
        ("clientWaitTime", "256"),
        ("driverType", "3"),
    ] {
        let prefix = format!("  edit {field} = ");
        let old = ir
            .text
            .lines()
            .skip(start)
            .find(|line| line.starts_with(&prefix))
            .unwrap();
        let new = format!("{prefix}{}", json!(replacement));
        let diff = patch(
            &ir.text,
            &[
                (metadata, "  edit name = \"Must not be saved\""),
                (old, &new),
            ],
        );
        assert_eq!(
            store
                .apply_property_patch("p.xgwx", &revision, "networks", None, &diff, false)
                .unwrap_err()
                .code,
            "operation_failed",
            "{field}"
        );
        assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), original);
        assert!(!fs::read_dir(dir.path()).unwrap().any(|entry| {
            entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .contains("backup")
        }));
    }
    for prefix in [
        "  readonly type = ",
        "  readonly network_type = ",
        "  readonly id = ",
        "  readonly name = ",
    ] {
        let old = ir
            .text
            .lines()
            .find(|line| line.starts_with(prefix))
            .unwrap();
        assert!(
            ir.lower_patch(&patch(
                &ir.text,
                &[(old, &old.replacen("readonly", "edit", 1))]
            ))
            .is_err()
        );
    }
}

#[test]
fn network_metadata_fenet_and_grouped_cnet_settings_preserve_other_payloads() {
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    doc.base_mut()
        .by_number(0)
        .and_then(|mut selected_record| selected_record.insert_module(0, "XGL-EFMT(B)"))
        .unwrap();
    let program = doc.program().ladder_program(0).unwrap().unwrap().data;
    apply(
        &mut doc,
        "networks",
        None,
        &[
            (
                "  edit ipAddress = \"192.168.0.100\"",
                "  edit ipAddress = \"10.20.30.40\"",
            ),
            ("  edit dhcp = \"0\"", "  edit dhcp = \"1\""),
        ],
    );
    assert_eq!(
        doc.network().fenet_config_infos()[0]
            .ip_address
            .as_ref()
            .unwrap()
            .address,
        "10.20.30.40"
    );
    assert_eq!(doc.network().fenet_config_infos()[0].dhcp, Some(1));
    let ir = Projection::export(&doc, "revision", "networks", None).unwrap();
    let old = format!("  edit name = {}", json!(doc.network().list()[0].name));
    apply(
        &mut doc,
        "networks",
        None,
        &[(&old, "  edit name = \"Plant network\"")],
    );
    assert_eq!(
        doc.network().list()[0].name.as_deref(),
        Some("Plant network")
    );
    assert!(ir.text.contains("network_module"));
    assert_eq!(
        doc.program().ladder_program(0).unwrap().unwrap().data,
        program
    );
    let mut c = fixture("networks/cnetbase-native.xgwx");
    let ir = Projection::export(&c, "revision", "networks", None).unwrap();
    let start = ir
        .text
        .lines()
        .position(|l| l.starts_with("cnet_port"))
        .unwrap();
    let ops = ir
        .lower_patch(&patch_from(
            &ir.text,
            &[
                ("  edit stationNo = \"0\"", "  edit stationNo = \"1\""),
                (
                    "  edit requestDelayTime = \"0\"",
                    "  edit requestDelayTime = \"10\"",
                ),
            ],
            start,
        ))
        .unwrap();
    assert_eq!(ops.len(), 1);
    assert_eq!(ops[0].args["patch"]["changes"].as_array().unwrap().len(), 2);
    xgwx_mcp::operations::apply_operation(&mut c, &ops[0].name, ops[0].args.clone()).unwrap();
    assert_eq!(
        c.network().cnet_config_infos()[0].ports[0].station_no,
        Some(1)
    );
    c.to_verified_bytes().unwrap();
}
#[test]
fn property_transactions_preview_backup_stale_rollback_and_noop() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("sample.xgwx");
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    doc.base_mut()
        .by_number(0)
        .and_then(|mut selected_record| selected_record.insert_module(0, "XGI-D24A/B"))
        .unwrap();
    let bytes = doc.to_verified_bytes().unwrap();
    fs::write(&path, &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, "modules", None).unwrap();
    let bad = patch(
        &ir.text,
        &[
            ("  edit comment = \"\"", "  edit comment = \"Inputs\""),
            (
                "  edit option.inputFilter.0 = 0",
                "  edit option.inputFilter.0 = 999",
            ),
        ],
    );
    assert_eq!(
        store
            .apply_property_patch("sample.xgwx", &rev, "modules", None, &bad, false)
            .unwrap_err()
            .code,
        "operation_failed"
    );
    assert_eq!(fs::read(&path).unwrap(), bytes);
    let diff = patch(
        &ir.text,
        &[("  edit comment = \"\"", "  edit comment = \"Inputs\"")],
    );
    let preview = store
        .apply_property_patch("sample.xgwx", &rev, "modules", None, &diff, true)
        .unwrap();
    assert_eq!(fs::read(&path).unwrap(), bytes);
    assert!(preview["backup_path"].is_null());
    let saved = store
        .apply_property_patch("sample.xgwx", &rev, "modules", None, &diff, false)
        .unwrap();
    assert_eq!(preview["revision_after"], saved["revision_after"]);
    assert_eq!(
        fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
        bytes
    );
    assert_eq!(
        store
            .apply_property_patch("sample.xgwx", &rev, "modules", None, &diff, false)
            .unwrap_err()
            .code,
        "stale_revision"
    );
    let next = saved["revision_after"].as_str().unwrap();
    let noop = store
        .apply_property_patch("sample.xgwx", next, "modules", None, "", false)
        .unwrap();
    assert_eq!(noop["changed"], false);
    assert!(noop["backup_path"].is_null());
    let (_, _, current) = store.read("sample.xgwx").unwrap();
    let ir = Projection::export(&current, next, "modules", None).unwrap();
    let equivalent = patch(
        &ir.text,
        &[(
            r#"  edit comment = "Inputs""#,
            r#"  edit comment = "\u0049nputs""#,
        )],
    );
    let noop = store
        .apply_property_patch("sample.xgwx", next, "modules", None, &equivalent, false)
        .unwrap();
    assert_eq!(noop["changed"], false);
    assert!(noop["backup_path"].is_null());
}
#[test]
fn property_frozen_identity_shape_type_and_literal_guards() {
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    doc.base_mut()
        .by_number(0)
        .and_then(|mut selected_record| selected_record.insert_module(0, "XGI-D24A/B"))
        .unwrap();
    let ir = Projection::export(&doc, "revision", "modules", None).unwrap();
    for (old, new) in [
        ("module @m0 base 0 slot 0 {", "module @m0 base 0 slot 1 {"),
        ("  edit model = \"XGI-D24A/B\"", "  edit model = 1"),
        ("  edit comment = \"\"", "  edit comment = null"),
        (
            "  edit option.inputFilter.0 = 0",
            "  edit option.inputFilter.0 = \"3\"",
        ),
        ("  edit comment = \"\"", "  edit comment = {}"),
        (
            "  edit option.inputFilter.0 = 0",
            "  edit option.inputFilter.0 = 4294967296",
        ),
    ] {
        assert!(ir.lower_patch(&patch(&ir.text, &[(old, new)])).is_err());
    }
    let diff = "--- p\n+++ p\n@@ -1,0 +2,1 @@\n+module @new {\n".to_owned();
    assert!(ir.lower_patch(&diff).is_err());
    assert!(Projection::export(&doc, "revision", "modules", Some(0)).is_err());
    assert!(Projection::export(&doc, "revision", "invalid", None).is_err());
}

#[test]
fn project_name_metadata_preview_save_stale_and_readonly_guards() {
    let doc = fixture("elements.xgwx");
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, "project", None).unwrap();
    let name = ir
        .text
        .lines()
        .find(|l| l.starts_with("  edit name = "))
        .unwrap();
    let new = "  edit name = \"Factory & 한글 <Line>\"";
    let diff = patch(&ir.text, &[(name, new)]);
    let ops = ir.lower_patch(&diff).unwrap();
    assert_eq!(ops.len(), 1);
    assert_eq!(ops[0].name, "rename_project");
    assert_eq!(ops[0].args["expected_name"], json!(doc.project().name()));
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    store
        .apply_property_patch("p.xgwx", &rev, "project", None, &diff, true)
        .unwrap();
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    let bad = patch(&ir.text, &[(name, "  edit name = \" \"")]);
    assert_eq!(
        store
            .apply_property_patch("p.xgwx", &rev, "project", None, &bad, false)
            .unwrap_err()
            .code,
        "operation_failed"
    );
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    let field = ir
        .text
        .lines()
        .find(|l| l.starts_with("  readonly version = "))
        .unwrap();
    assert!(
        ir.lower_patch(&patch(&ir.text, &[(field, "  readonly version = 2")]))
            .is_err()
    );
    let saved = store
        .apply_property_patch("p.xgwx", &rev, "project", None, &diff, false)
        .unwrap();
    assert_eq!(
        fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
        bytes
    );
    let after = XgwxDocument::from_path(dir.path().join("p.xgwx")).unwrap();
    let mut before_info = doc.project().info();
    before_info.name = after.project().info().name.clone();
    assert_eq!(before_info, after.project().info());
    assert_eq!(after.project().name(), Some("Factory & 한글 <Line>"));
    assert_eq!(doc.program().programs(), after.program().programs());
    assert_eq!(doc.module().summaries(), after.module().summaries());
    assert_eq!(
        store
            .apply_property_patch("p.xgwx", &rev, "project", None, &diff, false)
            .unwrap_err()
            .code,
        "stale_revision"
    );
    assert!(Projection::export(&doc, &rev, "project", Some(0)).is_err());
}

#[test]
fn program_metadata_coalesces_all_fields_and_preserves_bodies_and_identity() {
    let mut doc = fixture("elements.xgwx");
    let ir = Projection::export(&doc, "revision", "programs", Some(0)).unwrap();
    let desired = [
        ("name", json!("Conveyor & 한글")),
        ("task", json!("_SCAN")),
        ("version", json!(17)),
        ("kind", json!(3)),
        ("instance_name", json!("LineOne")),
        ("comment", json!("Metadata <note>")),
    ];
    let changes: Vec<(String, String)> = desired
        .iter()
        .map(|(key, value)| {
            let old = ir
                .text
                .lines()
                .find(|l| l.starts_with(&format!("  edit {key} = ")))
                .unwrap()
                .to_owned();
            (old, format!("  edit {key} = {value}"))
        })
        .collect();
    let refs = changes
        .iter()
        .map(|(old, new)| (old.as_str(), new.as_str()))
        .collect::<Vec<_>>();
    let ops = ir.lower_patch(&patch(&ir.text, &refs)).unwrap();
    assert_eq!(ops.len(), 1);
    assert_eq!(ops[0].name, "update_program");
    assert_eq!(ops[0].args["patch"].as_object().unwrap().len(), 6);
    let before = doc.program().programs();
    let payloads = doc
        .program()
        .ladder_programs()
        .into_iter()
        .map(|p| p.unwrap().data)
        .collect::<Vec<_>>();
    let globals = doc.variable().variables().unwrap();
    xgwx_mcp::operations::apply_operation(&mut doc, &ops[0].name, ops[0].args.clone()).unwrap();
    let after = doc.program().programs();
    assert_eq!(after[0].name.as_deref(), Some("Conveyor & 한글"));
    assert_eq!(after[0].version, Some(17));
    assert_eq!(after[0].kind, Some(3));
    assert_eq!(after[0].object_id, before[0].object_id);
    assert_eq!(&after[1..], &before[1..]);
    assert_eq!(
        payloads,
        doc.program()
            .ladder_programs()
            .into_iter()
            .map(|p| p.unwrap().data)
            .collect::<Vec<_>>()
    );
    assert_eq!(globals, doc.variable().variables().unwrap());
    doc.to_verified_bytes().unwrap();
    for (key, value) in [
        ("object_id", json!("forged")),
        ("local_variable", json!(999)),
    ] {
        let line = ir
            .text
            .lines()
            .find(|l| l.starts_with(&format!("  readonly {key} = ")))
            .unwrap();
        assert!(
            ir.lower_patch(&patch(
                &ir.text,
                &[(line, &format!("  readonly {key} = {value}"))]
            ))
            .is_err()
        );
    }
    let version = ir
        .text
        .lines()
        .find(|l| l.starts_with("  edit version = "))
        .unwrap();
    for value in ["-1", "4294967296", "\"17\"", "null"] {
        assert!(
            ir.lower_patch(&patch(
                &ir.text,
                &[(version, &format!("  edit version = {value}"))]
            ))
            .is_err()
        );
    }
}

#[test]
fn missing_program_metadata_and_ambiguous_project_names_are_readonly() {
    let mut doc = fixture("elements.xgwx");
    let xml = doc.container().xml().to_owned();
    let parsed = roxmltree::Document::parse(&xml).unwrap();
    let program = parsed
        .descendants()
        .find(|n| n.has_tag_name("Program"))
        .unwrap();
    let attr = program.attribute_node("Comment").unwrap();
    let mut edited = xml.clone();
    edited.replace_range(attr.range(), "");
    doc.container_mut().replace_xml(edited).unwrap();
    let ir = Projection::export(&doc, "revision", "programs", Some(0)).unwrap();
    assert!(ir.text.contains("readonly comment = none"));
    assert!(
        ir.lower_patch(&patch(
            &ir.text,
            &[("  readonly comment = none", "  edit comment = \"new\"")]
        ))
        .is_err()
    );
    assert!(Projection::export(&doc, "revision", "programs", Some(usize::MAX)).is_err());
    let name = doc.project().name().unwrap();
    let xml = doc
        .container()
        .xml()
        .replacen(name, &format!("{name}<!-- split -->Another"), 1);
    doc.container_mut().replace_xml(xml).unwrap();
    let ir = Projection::export(&doc, "revision", "project", None).unwrap();
    assert!(ir.text.lines().any(|l| l.starts_with("  readonly name = ")));
}

fn complete_patch(old: &str, new: &str) -> String {
    format!(
        "--- old\n+++ new\n@@ -1,{} +1,{} @@\n{}{}",
        old.lines().count(),
        new.lines().count(),
        old.lines().map(|l| format!("-{l}\n")).collect::<String>(),
        new.lines().map(|l| format!("+{l}\n")).collect::<String>()
    )
}
fn metadata_blocks(text: &str) -> (String, Vec<String>) {
    let start = text.find("program_metadata ").unwrap_or(text.len());
    let prefix = text[..start].to_owned();
    let blocks = text[start..]
        .split_inclusive("}\n")
        .map(str::to_owned)
        .collect();
    (prefix, blocks)
}
#[test]
fn programs_lifecycle_creation_languages_preview_save_identity_and_payloads() {
    let mut doc = fixture("empty-projects/new-xgi.xgwx");
    let dir = tempfile::tempdir().unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    for language in ["LD", "SFC", "ST", "IL"] {
        let bytes = doc.to_verified_bytes().unwrap();
        fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
        let rev = revision(&bytes);
        let ir = Projection::export(&doc, &rev, "programs", None).unwrap();
        let new = format!(
            "{}program_metadata @new-{} {{\n  edit name = \"Extra{}\"\n  edit language = \"{}\"\n}}\n",
            ir.text, language, language, language
        );
        let diff = complete_patch(&ir.text, &new);
        let ops = ir.lower_patch(&diff).unwrap();
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].name, "create_program");
        assert_eq!(ops[0].args, ir.lower_patch(&diff).unwrap()[0].args);
        assert_ne!(
            ops[0].args["patch"]["object_id"],
            ops[0].args["patch"]["symbol_id"]
        );
        let before = doc.program().programs();
        let globals = doc.variable().variables().unwrap();
        let preview = store
            .apply_property_patch("p.xgwx", &rev, "programs", None, &diff, true)
            .unwrap();
        assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
        let saved = store
            .apply_property_patch("p.xgwx", &rev, "programs", None, &diff, false)
            .unwrap();
        assert_eq!(preview["revision_after"], saved["revision_after"]);
        assert_eq!(
            fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
            bytes
        );
        assert_eq!(
            store
                .apply_property_patch("p.xgwx", &rev, "programs", None, &diff, false)
                .unwrap_err()
                .code,
            "stale_revision"
        );
        doc = XgwxDocument::from_path(dir.path().join("p.xgwx")).unwrap();
        let after = doc.program().programs();
        assert_eq!(&after[..before.len()], &before);
        assert_eq!(
            after.last().unwrap().object_id.as_deref(),
            ops[0].args["patch"]["object_id"].as_str()
        );
        assert_eq!(globals, doc.variable().variables().unwrap());
        let body = xgwx_mcp::text_ir::Projection::export(
            &doc,
            &saved["revision_after"].as_str().unwrap(),
            after.len() - 1,
        )
        .unwrap();
        assert_eq!(
            body.dialect,
            match language {
                "LD" => "IEC",
                other => other,
            }
        );
    }
}
fn lifecycle_document() -> XgwxDocument {
    let mut doc = fixture("elements.xgwx");
    for (index, name) in [(1, "LifecycleOne"), (2, "LifecycleTwo")] {
        doc.program_mut()
            .create(&xgwx::NewProgram {
                name: name.into(),
                language: "LD".into(),
                object_id: format!("12345678-1234-1234-1234-{index:012x}"),
                symbol_id: format!("87654321-4321-4321-4321-{index:012x}"),
            })
            .unwrap();
    }
    doc
}
#[test]
fn programs_lifecycle_move_delete_preserve_retained_native_blocks_and_globals() {
    let mut doc = lifecycle_document();
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, "programs", None).unwrap();
    let (prefix, mut blocks) = metadata_blocks(&ir.text);
    assert!(blocks.len() >= 2);
    let parsed = roxmltree::Document::parse(doc.container().xml()).unwrap();
    let native_blocks = parsed
        .descendants()
        .filter(|n| n.has_tag_name("Program"))
        .map(|n| doc.container().xml()[n.range()].to_owned())
        .collect::<Vec<_>>();
    let globals = doc.variable().variables().unwrap();
    let moved = blocks.remove(0);
    blocks.push(moved);
    let diff = complete_patch(&ir.text, &format!("{prefix}{}", blocks.concat()));
    let ops = ir.lower_patch(&diff).unwrap();
    assert_eq!(ops.len(), 1);
    assert_eq!(ops[0].name, "move_program");
    xgwx_mcp::operations::apply_operation(&mut doc, &ops[0].name, ops[0].args.clone()).unwrap();
    let parsed = roxmltree::Document::parse(doc.container().xml()).unwrap();
    let after_blocks = parsed
        .descendants()
        .filter(|n| n.has_tag_name("Program"))
        .map(|n| doc.container().xml()[n.range()].to_owned())
        .collect::<Vec<_>>();
    let mut expected = native_blocks;
    let moved = expected.remove(0);
    expected.push(moved);
    assert_eq!(after_blocks, expected);
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, "programs", None).unwrap();
    let (prefix, mut blocks) = metadata_blocks(&ir.text);
    blocks.remove(1);
    let diff = complete_patch(&ir.text, &format!("{prefix}{}", blocks.concat()));
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    store
        .apply_property_patch("p.xgwx", &rev, "programs", None, &diff, true)
        .unwrap();
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    store
        .apply_property_patch("p.xgwx", &rev, "programs", None, &diff, false)
        .unwrap();
    doc = XgwxDocument::from_path(dir.path().join("p.xgwx")).unwrap();
    expected.remove(1);
    let parsed = roxmltree::Document::parse(doc.container().xml()).unwrap();
    assert_eq!(
        parsed
            .descendants()
            .filter(|n| n.has_tag_name("Program"))
            .map(|n| doc.container().xml()[n.range()].to_owned())
            .collect::<Vec<_>>(),
        expected
    );
    assert_eq!(globals, doc.variable().variables().unwrap());
    // The complete list remains an authoring surface after deleting the last program.
    while !doc.program().programs().is_empty() {
        let bytes = doc.to_verified_bytes().unwrap();
        let ir = Projection::export(&doc, &revision(&bytes), "programs", None).unwrap();
        let (prefix, mut blocks) = metadata_blocks(&ir.text);
        blocks.pop();
        let ops = ir
            .lower_patch(&complete_patch(
                &ir.text,
                &format!("{prefix}{}", blocks.concat()),
            ))
            .unwrap();
        xgwx_mcp::operations::apply_operation(&mut doc, &ops[0].name, ops[0].args.clone()).unwrap();
    }
    let bytes = doc.to_verified_bytes().unwrap();
    let ir = Projection::export(&doc, &revision(&bytes), "programs", None).unwrap();
    let new = format!(
        "{}program_metadata @new-restart {{\n  edit name = \"Restart\"\n  edit language = \"LD\"\n}}\n",
        ir.text
    );
    let ops = ir.lower_patch(&complete_patch(&ir.text, &new)).unwrap();
    xgwx_mcp::operations::apply_operation(&mut doc, &ops[0].name, ops[0].args.clone()).unwrap();
    assert_eq!(doc.program().programs().len(), 1);
    assert_eq!(globals, doc.variable().variables().unwrap());
    doc.to_verified_bytes().unwrap();
}
#[test]
fn programs_lifecycle_rejects_mixed_forged_partial_and_invalid_native_edits() {
    let doc = lifecycle_document();
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, "programs", None).unwrap();
    let (prefix, blocks) = metadata_blocks(&ir.text);
    let author =
        "program_metadata @new-extra {\n  edit name = \"Extra\"\n  edit language = \"LD\"\n}\n";
    for changed in [
        format!("{}{}", ir.text, author.replace("@new-extra", "@p999")),
        format!(
            "{}{}",
            ir.text,
            author.replace("  edit language", "  readonly language")
        ),
        format!(
            "{}{}",
            ir.text,
            author.replace("}\n", "  edit object_id = \"forged\"\n}\n")
        ),
        format!(
            "{}{}{}",
            ir.text,
            author,
            author.replace("@new-extra", "@new-other")
        ),
        format!("{prefix}{author}{}", blocks.concat()),
        format!(
            "{prefix}{}",
            blocks[1..]
                .concat()
                .replace("  readonly object_id = ", "  readonly forged_id = ")
        ),
    ] {
        assert!(ir.lower_patch(&complete_patch(&ir.text, &changed)).is_err());
    }
    let mut reverse = blocks.clone();
    reverse.reverse();
    assert!(
        ir.lower_patch(&complete_patch(
            &ir.text,
            &format!("{prefix}{}", reverse.concat())
        ))
        .is_err()
    );
    assert!(
        ir.lower_patch(&complete_patch(
            &ir.text,
            &format!("{prefix}{}", blocks[2..].concat())
        ))
        .is_err()
    );
    let partial = Projection::export(&doc, &rev, "programs", Some(0)).unwrap();
    let (prefix, _) = metadata_blocks(&partial.text);
    assert!(
        partial
            .lower_patch(&complete_patch(&partial.text, &prefix))
            .is_err()
    );
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    for bad in [
        author.replace("\"Extra\"", "\"invalid name\""),
        author.replace("\"LD\"", "\"UNKNOWN\""),
    ] {
        let changed = format!("{}{bad}", ir.text);
        assert_eq!(
            store
                .apply_property_patch(
                    "p.xgwx",
                    &rev,
                    "programs",
                    None,
                    &complete_patch(&ir.text, &changed),
                    false
                )
                .unwrap_err()
                .code,
            "operation_failed"
        );
        assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
        assert!(
            !fs::read_dir(dir.path()).unwrap().any(|e| e
                .unwrap()
                .file_name()
                .to_string_lossy()
                .contains("backup"))
        );
    }
}

#[test]
fn unknown_module_models_can_select_catalog_models_and_slot_collisions_abort() {
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    doc.base_mut()
        .by_number(0)
        .unwrap()
        .insert_module(0, "XGI-D24A/B")
        .unwrap();
    doc.base_mut()
        .by_number(0)
        .unwrap()
        .insert_module(1, "XGI-D24A/B")
        .unwrap();
    let xml = doc.container().xml().to_owned();
    let parsed = roxmltree::Document::parse(&xml).unwrap();
    let attribute = parsed
        .descendants()
        .find(|n| {
            n.has_tag_name("Module")
                && n.attribute("Base") == Some("0")
                && n.attribute("Slot") == Some("0")
        })
        .unwrap()
        .attribute_node("Id")
        .unwrap();
    let mut changed = xml.clone();
    changed.replace_range(attribute.range_value(), "999999");
    doc.container_mut().replace_xml(changed).unwrap();
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, "modules", None).unwrap();
    assert!(ir.text.contains("  edit model = \"\""));
    let wide = xgwx::xgk_module_catalog()
        .iter()
        .find(|m| m.slot_span > 1)
        .unwrap();
    let diff = patch(
        &ir.text,
        &[(
            "  edit model = \"\"",
            &format!("  edit model = {}", json!(wide.model)),
        )],
    );
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    assert_eq!(
        store
            .apply_property_patch("p.xgwx", &rev, "modules", None, &diff, false)
            .unwrap_err()
            .code,
        "operation_failed"
    );
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    let diff = patch(
        &ir.text,
        &[("  edit model = \"\"", "  edit model = \"XGI-D24A/B\"")],
    );
    store
        .apply_property_patch("p.xgwx", &rev, "modules", None, &diff, false)
        .unwrap();
    let next = XgwxDocument::from_path(dir.path().join("p.xgwx")).unwrap();
    assert_eq!(next.module().summaries()[0].id, Some(42242));
}

#[test]
fn local_variable_lifecycle_from_empty_table_create_address_then_delete() {
    let mut doc = fixture("empty-projects/new-xgi.xgwx");
    let globals = doc.variable().variables().unwrap();
    let payload = doc.program().ladder_program(0).unwrap().unwrap().data;
    let dir = tempfile::tempdir().unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let ir = Projection::export(&doc, &rev, "variables", Some(0)).unwrap();
    assert!(ir.text.contains("local_table @p0 create true"));
    let new = format!(
        "{}variable @new-ready program 0 {{\n  edit name = \"Ready\"\n  edit data_type = \"BOOL\"\n  edit description = \"Enable & ready\"\n}}\n",
        ir.text
    );
    let diff = complete_patch(&ir.text, &new);
    let ops = ir.lower_patch(&diff).unwrap();
    assert_eq!(ops.len(), 1);
    assert_eq!(ops[0].name, "insert_iec_local_symbol");
    let preview = store
        .apply_property_patch("p.xgwx", &rev, "variables", Some(0), &diff, true)
        .unwrap();
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    let saved = store
        .apply_property_patch("p.xgwx", &rev, "variables", Some(0), &diff, false)
        .unwrap();
    assert_eq!(saved["revision_after"], preview["revision_after"]);
    assert_eq!(
        fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
        bytes
    );
    assert_eq!(
        store
            .apply_property_patch("p.xgwx", &rev, "variables", Some(0), &diff, false)
            .unwrap_err()
            .code,
        "stale_revision"
    );
    doc = XgwxDocument::from_path(dir.path().join("p.xgwx")).unwrap();
    let locals = doc.variable().iec_local_symbols().remove(0).unwrap();
    assert_eq!(locals[0].name, "Ready");
    assert_eq!(locals[0].data_type.as_deref(), Some("BOOL"));
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, "variables", Some(0)).unwrap();
    let diff = patch(
        &ir.text,
        &[("  edit address = \"\"", "  edit address = \"%MX100\"")],
    );
    store
        .apply_property_patch("p.xgwx", &rev, "variables", Some(0), &diff, false)
        .unwrap();
    doc = XgwxDocument::from_path(dir.path().join("p.xgwx")).unwrap();
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, "variables", Some(0)).unwrap();
    let start = ir.text.find("variable @p0.v0 {").unwrap();
    let end = start + ir.text[start..].find("}\n").unwrap() + 2;
    let changed = format!("{}{}", &ir.text[..start], &ir.text[end..]);
    let diff = complete_patch(&ir.text, &changed);
    assert_eq!(
        ir.lower_patch(&diff).unwrap()[0].name,
        "delete_iec_local_symbol"
    );
    store
        .apply_property_patch("p.xgwx", &rev, "variables", Some(0), &diff, false)
        .unwrap();
    doc = XgwxDocument::from_path(dir.path().join("p.xgwx")).unwrap();
    assert!(
        doc.variable()
            .iec_local_symbols()
            .remove(0)
            .unwrap()
            .is_empty()
    );
    assert_eq!(doc.variable().variables().unwrap(), globals);
    assert_eq!(
        doc.program().ladder_program(0).unwrap().unwrap().data,
        payload
    );
}

#[test]
fn local_variable_lifecycle_rejects_forged_scope_invalid_type_and_references() {
    let doc = fixture("function-output-wires/base.xgwx");
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, "variables", Some(0)).unwrap();
    let author = "variable @new-value program 0 {\n  edit name = \"Value\"\n  edit data_type = \"WORD\"\n  edit description = \"\"\n}\n";
    for bad in [
        author.replace("@new-value", "@p999.v0"),
        author.replace("program 0", "program 999"),
        author.replace(" program 0", ""),
        author.replace("}\n", "  edit address = \"%MW10\"\n}\n"),
    ] {
        assert!(
            ir.lower_patch(&complete_patch(&ir.text, &format!("{}{bad}", ir.text)))
                .is_err()
        );
    }
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    for bad in [
        author.replace("\"WORD\"", "\"UNKNOWN\""),
        author.replace("\"Value\"", "\"bad name\""),
    ] {
        assert_eq!(
            store
                .apply_property_patch(
                    "p.xgwx",
                    &rev,
                    "variables",
                    Some(0),
                    &complete_patch(&ir.text, &format!("{}{bad}", ir.text)),
                    false
                )
                .unwrap_err()
                .code,
            "operation_failed"
        );
        assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    }
    let mut doc = fixture("ladder-wire-delete/iec-horizontal-native.xgwx");
    doc.program_mut()
        .get(0)
        .and_then(|mut p| p.variable_mut().insert("Referenced", "BOOL", ""))
        .unwrap();
    let program = doc.program().ladder_program(0).unwrap().unwrap();
    let frame = program
        .iec_record_frames()
        .unwrap()
        .into_iter()
        .find(|r| matches!(r.kind, xgwx::IecRecordKind::Contact(_)))
        .unwrap();
    let operand = program
        .strings
        .iter()
        .find(|s| s.offset == frame.offset + 15)
        .unwrap();
    doc.program_mut()
        .get(0)
        .and_then(|mut p| {
            p.iec_ld()?
                .update_element_operand(operand.offset, &operand.value, "Referenced")
        })
        .unwrap();
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let ir = Projection::export(&doc, &rev, "variables", Some(0)).unwrap();
    let locals = doc.variable().iec_local_symbols().remove(0).unwrap();
    let index = locals
        .iter()
        .position(|v| {
            !v.is_instance
                && doc
                    .program()
                    .ladder_program(0)
                    .unwrap()
                    .unwrap()
                    .strings
                    .iter()
                    .any(|s| s.value == v.name)
        })
        .unwrap();
    let start = ir.text.find(&format!("variable @p0.v{index} {{")).unwrap();
    let end = start + ir.text[start..].find("}\n").unwrap() + 2;
    let changed = format!("{}{}", &ir.text[..start], &ir.text[end..]);
    assert_eq!(
        store
            .apply_property_patch(
                "p.xgwx",
                &rev,
                "variables",
                Some(0),
                &complete_patch(&ir.text, &changed),
                false
            )
            .unwrap_err()
            .code,
        "operation_failed"
    );
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
}
