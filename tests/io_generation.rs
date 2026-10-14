#[path = "../src/test_support.rs"]
mod test_support;
use serde_json::json;
use xgwx::XgwxDocument;
use xgwx_mcp::{
    operations::apply_operation,
    storage::{EditOperation, FileStore, revision},
};
#[test]
fn scoped_io_preserves_full_allocation_unselected_symbols_and_native_payloads() {
    let mut doc = XgwxDocument::from_path(
        test_support::fixture_root().join("fixtures/empty-projects/new-xgk.xgwx"),
    )
    .unwrap();
    for slot in [0, 1, 2] {
        doc.base_mut()
            .by_number(0)
            .unwrap()
            .insert_module(slot, "XGI-D24A/B")
            .unwrap();
    }
    let full = doc.variable().preview_io_variables().unwrap();
    let selected = doc
        .variable()
        .preview_io_variables_for(Some(0), Some(1))
        .unwrap();
    assert_eq!(
        selected,
        full.iter()
            .filter(|r| r.slot == 1)
            .cloned()
            .collect::<Vec<_>>()
    );
    assert!(selected[0].address_number > 0);
    let payload = doc.program().ladder_program(0).unwrap().unwrap().data;
    let hardware = doc.module().summaries();
    apply_operation(&mut doc, "generate_io_variables", json!({"slot":0})).unwrap();
    let previous = doc.variable().variables().unwrap();
    assert_eq!(previous.len(), 32);
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("p.xgwx");
    std::fs::write(&path, &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let operation = EditOperation {
        name: "generate_io_variables".into(),
        args: json!({"base":0,"slot":1}),
    };
    let preview = store
        .apply("p.xgwx", &rev, &[operation.clone()], true)
        .unwrap();
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    let saved = store
        .apply("p.xgwx", &rev, &[operation.clone()], false)
        .unwrap();
    assert_eq!(saved["revision_after"], preview["revision_after"]);
    doc = XgwxDocument::from_path(&path).unwrap();
    let after = doc.variable().variables().unwrap();
    assert_eq!(after.len(), 64);
    for symbol in previous {
        assert!(after.contains(&symbol));
    }
    assert!(
        after
            .iter()
            .all(|v| !v.source_ref.as_ref().unwrap().starts_with("IO:0:2:"))
    );
    assert_eq!(
        doc.program().ladder_program(0).unwrap().unwrap().data,
        payload
    );
    assert_eq!(doc.module().summaries(), hardware);
    let noop = store
        .apply(
            "p.xgwx",
            saved["revision_after"].as_str().unwrap(),
            &[operation],
            false,
        )
        .unwrap();
    assert!(noop["backup_path"].is_null());
    for args in [json!({"base":999}), json!({"base":0,"slot":999})] {
        let before = std::fs::read(&path).unwrap();
        assert_eq!(
            store
                .apply(
                    "p.xgwx",
                    &revision(&before),
                    &[EditOperation {
                        name: "generate_io_variables".into(),
                        args
                    }],
                    false
                )
                .unwrap_err()
                .code,
            "operation_failed"
        );
        assert_eq!(std::fs::read(&path).unwrap(), before);
    }
    doc.variable_mut()
        .generate_io_variables_for(Some(0), None)
        .unwrap();
    assert_eq!(doc.variable().variables().unwrap().len(), 96);
}

#[tokio::test]
async fn dedicated_io_tool_accepts_flat_optional_scope_and_returns_plan() {
    use rmcp::{
        ServiceExt,
        model::{CallToolRequestParams, ClientConfig},
        transport::TokioChildProcess,
    };
    let mut doc = XgwxDocument::from_path(
        test_support::fixture_root().join("fixtures/empty-projects/new-xgk.xgwx"),
    )
    .unwrap();
    for slot in [0, 1] {
        doc.base_mut()
            .by_number(0)
            .unwrap()
            .insert_module(slot, "XGI-D24A/B")
            .unwrap();
    }
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("p.xgwx");
    std::fs::write(&path, &bytes).unwrap();
    let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_xgwx-mcp"));
    command.current_dir(dir.path());
    let client = ClientConfig::default()
        .serve(TokioChildProcess::new(command).unwrap())
        .await
        .unwrap();
    let call = |args: serde_json::Value| {
        CallToolRequestParams::new("generate_io_variables")
            .with_arguments(args.as_object().unwrap().clone())
    };
    let tools = client.list_all_tools().await.unwrap();
    assert!(tools.iter().any(|t| t.name == "generate_io_variables"));
    assert!(!tools.iter().any(|t| t.name == "xgwx_generate_io_variables"));
    let mut args = json!({"path":"p.xgwx","expected_revision":rev,"slot":1,"dry_run":true});
    let preview = client.call_tool(call(args.clone())).await.unwrap();
    assert_ne!(preview.is_error, Some(true));
    let preview = preview.structured_content.unwrap();
    assert_eq!(preview["io_generation"]["total"], 32);
    assert!(
        preview["io_generation"]["rows"]
            .as_array()
            .unwrap()
            .iter()
            .all(|r| r["slot"] == 1 && r["base"] == 0)
    );
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    args["dry_run"] = json!(false);
    let saved = client
        .call_tool(call(args.clone()))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    assert_eq!(saved["revision_after"], preview["revision_after"]);
    assert_eq!(
        std::fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
        bytes
    );
    let symbols = XgwxDocument::from_path(&path)
        .unwrap()
        .variable()
        .variables()
        .unwrap();
    assert_eq!(symbols.len(), 32);
    assert!(symbols.iter().all(|v| v.address_number.unwrap() >= 32));
    let stale = client.call_tool(call(args)).await.unwrap();
    assert_eq!(
        stale.structured_content.unwrap()["error"]["code"],
        "stale_revision"
    );
    let invalid = client
        .call_tool(call(
            json!({"path":"p.xgwx","expected_revision":saved["revision_after"],"base":999}),
        ))
        .await
        .unwrap();
    assert_eq!(invalid.is_error, Some(true));
    let all = client
        .call_tool(call(
            json!({"path":"p.xgwx","expected_revision":saved["revision_after"]}),
        ))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    assert_eq!(all["io_generation"]["total"], 64);
    assert_eq!(
        XgwxDocument::from_path(&path)
            .unwrap()
            .variable()
            .variables()
            .unwrap()
            .len(),
        64
    );
    client.cancel().await.unwrap();
}
