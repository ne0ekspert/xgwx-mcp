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
        ("  readonly id = 42242", "  readonly id = 42241"),
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
