use xgwx::create_project;
use xgwx_mcp::{inspection, library_compat::module_summaries, property_ir::Projection};

#[test]
fn hardware_lists_and_property_indices_remain_installed_hardware_records() {
    let mut doc = create_project("XGK-CPUSN", "LD").unwrap();
    doc.insert_module(0, 0, "XGL-EFMT(B)").unwrap();
    doc.insert_module(0, 1, "XGL-EFMT(B)").unwrap();
    let hardware = module_summaries(&doc);
    assert_eq!(hardware.len(), 2);
    assert_eq!(hardware[0].slot, Some(0));
    assert_eq!(hardware[1].slot, Some(1));
    assert!(hardware.iter().all(|m| m.id == Some(23041)));

    let summary = inspection::inspect(&doc, "hardware", None, 0, 100).unwrap();
    assert_eq!(summary["modules"]["total"], 2);
    assert_eq!(summary["modules"]["items"][0]["slot"], 0);
    assert_eq!(summary["modules"]["items"][1]["slot"], 1);
    assert!(summary["modules"]["items"][0]["name"].is_string());
    assert!(summary["modules"]["items"][0].get("Network").is_none());
    let overview = inspection::inspect(&doc, "overview", None, 0, 100).unwrap();
    assert_eq!(overview["counts"]["modules"], 2);

    let properties = Projection::export(&doc, "revision", "modules", None).unwrap();
    assert!(properties.text.contains("module @m0 base 0 slot 0"));
    assert!(properties.text.contains("module @m1 base 0 slot 1"));
    assert!(!properties.text.contains("module @m2"));
}

#[cfg(feature = "libxgwx-namespaced-api")]
#[test]
fn scoped_bases_expose_typed_memberships_and_bound_settings() {
    let mut doc = create_project("XGK-CPUSN", "LD").unwrap();
    doc.insert_module(0, 0, "XGL-EFMT(B)").unwrap();
    let summary = inspection::inspect(&doc, "hardware", None, 0, 100).unwrap();
    let base = &summary["bases"]["items"][0];
    assert_eq!(base["base_index"], 0);
    assert_eq!(base["configuration_index"], 0);
    let module = base["modules"]["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|m| m["hardware"]["id"] == 23041)
        .unwrap();
    assert_eq!(module["kind"], "network");
    assert_eq!(module["configuration"]["status"], "matched");
    assert_eq!(module["configuration"]["settings"]["kind"], "fenet");
    assert_eq!(
        module["configuration"]["settings"]["value"]["ip_address"]["address"],
        "192.168.0.100"
    );
}

#[cfg(feature = "libxgwx-namespaced-api")]
#[test]
fn selected_lifecycle_adapters_reject_stale_wire_identity() {
    use serde_json::json;
    use xgwx_mcp::operations::apply_operation;
    let mut doc = create_project("XGI-CPUE", "ST").unwrap();
    let before = doc.clone();
    assert!(
        apply_operation(
            &mut doc,
            "delete_program",
            json!({"program_index":0,"expected_object_id":"stale"})
        )
        .is_err()
    );
    assert_eq!(doc, before);
    let program = doc.program().get(0).unwrap().text().unwrap();
    assert!(apply_operation(&mut doc, "edit_text_program", json!({"patch":{
        "program_index":0,"expected_object_id":program.object_id,
        "expected_language":"IL", "expected_source":program.source.unwrap(), "source":"%MX0 := TRUE;"
    }})).is_err());
    assert_eq!(doc, before);
}
