#[path = "../src/test_support.rs"]
mod test_support;
use rmcp::{
    ServiceExt,
    model::{CallToolRequestParams, ClientConfig},
    transport::TokioChildProcess,
};
use serde_json::{Value, json};
use std::fs;
use test_support::fixture_root;

#[tokio::test]
async fn all_tools_declare_four_boolean_behavior_hints() {
    let dir = tempfile::tempdir().unwrap();
    let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_xgwx-mcp"));
    command.current_dir(dir.path());
    let client = ClientConfig::default()
        .serve(TokioChildProcess::new(command).unwrap())
        .await
        .unwrap();
    let tools = client.list_all_tools().await.unwrap();
    assert!(!tools.is_empty());
    for tool in tools {
        // Check the serialized discovery contract, including presence and type.
        let value = serde_json::to_value(&tool).unwrap();
        let annotations = &value["annotations"];
        for hint in [
            "readOnlyHint",
            "destructiveHint",
            "idempotentHint",
            "openWorldHint",
        ] {
            assert!(
                annotations[hint].is_boolean(),
                "{}: {hint} must be an explicit boolean",
                tool.name
            );
        }
        let (read_only, destructive, idempotent) = match tool.name.as_ref() {
            "list_templates"
            | "inspect_file"
            | "search_project"
            | "list_modules"
            | "list_module_catalog"
            | "list_networks"
            | "list_variables"
            | "list_operations"
            | "describe_operation"
            | "get_ir"
            | "validate_file" => (true, false, true),
            // Creation never overwrites; preparation allocates a new session token.
            "create_file" | "prepare_change" => (false, false, false),
            // Backups and dry-run options do not make these tools read-only or additive.
            "rename_program" | "commit_change" | "apply_edits" | "apply_patch" => {
                (false, true, false)
            }
            name => panic!("Classify the behavior hints for new tool {name}"),
        };
        assert_eq!(
            annotations["readOnlyHint"],
            json!(read_only),
            "{}",
            tool.name
        );
        assert_eq!(
            annotations["destructiveHint"],
            json!(destructive),
            "{}",
            tool.name
        );
        assert_eq!(
            annotations["idempotentHint"],
            json!(idempotent),
            "{}",
            tool.name
        );
        assert_eq!(annotations["openWorldHint"], json!(false), "{}", tool.name);
    }
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn real_stdio_inspection_schema_dry_run_edit_and_validation() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("project.xgwx");
    let original = fs::read(fixture_root().join("fixtures/elements.xgwx")).unwrap();
    fs::write(&path, &original).unwrap();
    let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_xgwx-mcp"));
    command.current_dir(dir.path());
    let transport = TokioChildProcess::new(command).unwrap();
    let client = ClientConfig::default().serve(transport).await.unwrap();
    assert_eq!(
        client
            .peer_info()
            .unwrap()
            .server_info
            .as_ref()
            .unwrap()
            .name,
        "xgwx-mcp"
    );
    let tools = client.list_all_tools().await.unwrap();
    let names: std::collections::BTreeSet<_> = tools.iter().map(|t| t.name.as_ref()).collect();
    assert_eq!(
        names,
        [
            "inspect_file",
            "search_project",
            "rename_program",
            "prepare_change",
            "commit_change",
            "list_modules",
            "list_module_catalog",
            "list_networks",
            "list_variables",
            "list_templates",
            "create_file",
            "list_operations",
            "describe_operation",
            "apply_edits",
            "validate_file",
            "get_ir",
            "apply_patch"
        ]
        .into_iter()
        .collect()
    );
    assert_eq!(
        tools
            .iter()
            .find(|t| t.name == "inspect_file")
            .unwrap()
            .annotations
            .as_ref()
            .unwrap()
            .read_only_hint,
        Some(true)
    );
    let call = |name: &str, args: Value| {
        CallToolRequestParams::new(name.to_string())
            .with_arguments(args.as_object().unwrap().clone())
    };
    let inspected = client
        .call_tool(call("inspect_file", json!({"path":"project.xgwx"})))
        .await
        .unwrap();
    assert_ne!(inspected.is_error, Some(true));
    let inspected = inspected.structured_content.unwrap();
    let rev = inspected["revision"].as_str().unwrap().to_owned();
    let matches = client.call_tool(call("search_project", json!({"path":"project.xgwx","query":inspected["data"]["programs"]["items"][0]["name"],"kind":"program","program_index":0}))).await.unwrap();
    assert_ne!(matches.is_error, Some(true));
    let matches = matches.structured_content.unwrap();
    assert_eq!(matches["data"]["total"], 1);
    assert_eq!(matches["data"]["items"][0]["program_index"], 0);
    assert_eq!(matches["revision"], rev);
    let searched = client.call_tool(call("search_project", json!({"path":"project.xgwx","query":"not-present-marker-12345","expected_revision":rev}))).await.unwrap();
    assert_ne!(searched.is_error, Some(true));
    let searched = searched.structured_content.unwrap();
    assert_eq!(searched["revision"], rev);
    assert_eq!(searched["data"]["total"], 0);
    assert_eq!(fs::read(&path).unwrap(), original);
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    for args in [
        json!({"path":"project.xgwx","query":"   "}),
        json!({"path":"project.xgwx","query":"contact","limit":0}),
        json!({"path":"project.xgwx","query":"contact","kind":"unknown"}),
    ] {
        assert_eq!(
            client
                .call_tool(call("search_project", args))
                .await
                .unwrap()
                .is_error,
            Some(true)
        );
    }
    let catalog = client
        .call_tool(call("list_operations", json!({})))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    assert_eq!(catalog["count"], 123);
    let schema = client
        .call_tool(call("describe_operation", json!({"name":"update_program"})))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    assert!(schema["argument_schema"]["properties"]["program_index"].is_object());
    let operations = json!([{"name":"update_program","args":{"program_index":0,"patch":{"comment":"Real stdio MCP test"}}}]);
    let preview = client.call_tool(call("apply_edits",json!({"path":"project.xgwx","expected_revision":rev,"operations":operations,"dry_run":true}))).await.unwrap();
    assert_ne!(preview.is_error, Some(true));
    assert_eq!(fs::read(&path).unwrap(), original);
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    let saved = client
        .call_tool(call(
            "apply_edits",
            json!({"path":"project.xgwx","expected_revision":rev,"operations":operations}),
        ))
        .await
        .unwrap();
    assert_ne!(saved.is_error, Some(true));
    let saved = saved.structured_content.unwrap();
    assert_eq!(
        fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
        original
    );
    assert_ne!(fs::read(&path).unwrap(), original);
    let validated = client
        .call_tool(call("validate_file", json!({"path":"project.xgwx"})))
        .await
        .unwrap();
    assert_ne!(validated.is_error, Some(true));
    assert_eq!(
        validated.structured_content.unwrap()["revision"],
        saved["revision_after"]
    );
    let stale_search = client
        .call_tool(call(
            "search_project",
            json!({"path":"project.xgwx","query":"contact","expected_revision":rev,"offset":1}),
        ))
        .await
        .unwrap();
    assert_eq!(stale_search.is_error, Some(true));
    assert_eq!(
        stale_search.structured_content.unwrap()["error"]["code"],
        "stale_revision"
    );
    let stale = client
        .call_tool(call(
            "apply_edits",
            json!({"path":"project.xgwx","expected_revision":rev,"operations":operations}),
        ))
        .await
        .unwrap();
    assert_eq!(stale.is_error, Some(true));
    assert_eq!(
        stale.structured_content.unwrap()["error"]["code"],
        "stale_revision"
    );
    let outside = tempfile::tempdir().unwrap();
    fs::write(outside.path().join("project.xgwx"), &original).unwrap();
    let rejected = client
        .call_tool(call(
            "inspect_file",
            json!({"path":outside.path().join("project.xgwx")}),
        ))
        .await
        .unwrap();
    assert_eq!(rejected.is_error, Some(true));
    assert_eq!(
        rejected.structured_content.unwrap()["error"]["code"],
        "outside_roots"
    );
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn repeatable_root_flags_allow_extra_directories() {
    let startup = tempfile::tempdir().unwrap();
    let first = tempfile::tempdir().unwrap();
    let second = tempfile::tempdir().unwrap();
    for dir in [&first, &second] {
        fs::copy(
            fixture_root().join("fixtures/elements.xgwx"),
            dir.path().join("project.xgwx"),
        )
        .unwrap();
    }
    let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_xgwx-mcp"));
    command
        .current_dir(startup.path())
        .arg("--root")
        .arg(first.path())
        .arg("--root")
        .arg(second.path());
    let client = ClientConfig::default()
        .serve(TokioChildProcess::new(command).unwrap())
        .await
        .unwrap();
    for dir in [&first, &second] {
        let result = client
            .call_tool(
                CallToolRequestParams::new("inspect_file").with_arguments(
                    json!({"path":dir.path().join("project.xgwx")})
                        .as_object()
                        .unwrap()
                        .clone(),
                ),
            )
            .await
            .unwrap();
        assert_ne!(result.is_error, Some(true));
    }
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn real_stdio_positional_ir_patch_preview_save_and_stale_revision() {
    let dir = tempfile::tempdir().unwrap();
    let original = fs::read(fixture_root().join("fixtures/elements.xgwx")).unwrap();
    fs::write(dir.path().join("project.xgwx"), &original).unwrap();
    let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_xgwx-mcp"));
    command.current_dir(dir.path());
    let client = ClientConfig::default()
        .serve(TokioChildProcess::new(command).unwrap())
        .await
        .unwrap();
    let call = |name: &str, args: Value| {
        CallToolRequestParams::new(name.to_string())
            .with_arguments(args.as_object().unwrap().clone())
    };
    let tools = client.list_all_tools().await.unwrap();
    let tool = tools.iter().find(|t| t.name == "apply_patch").unwrap();
    assert!(tool.input_schema["properties"]["patch"].is_object());
    let ir = client
        .call_tool(call(
            "get_ir",
            json!({"path":"project.xgwx","program_index":0}),
        ))
        .await
        .unwrap();
    assert_ne!(ir.is_error, Some(true));
    let ir = ir.structured_content.unwrap();
    assert_eq!(ir["format"], "xgwx-ir/1");
    assert_eq!(ir["dialect"], "XGK");
    let text = ir["text"].as_str().unwrap();
    let (i, line) = text
        .lines()
        .enumerate()
        .find(|(_, l)| l.trim_start().starts_with("contact ") && l.contains(" NO "))
        .unwrap();
    let patch = format!(
        "@@ -{},1 +{},1 @@\n-{}\n+{}\n",
        i + 1,
        i + 1,
        line,
        line.replace(" NO ", " NC ")
    );
    let args = json!({"path":"project.xgwx","program_index":0,"expected_revision":ir["revision"],"patch":patch,"dry_run":true});
    let preview = client
        .call_tool(call("apply_patch", args.clone()))
        .await
        .unwrap();
    assert_ne!(preview.is_error, Some(true));
    assert_eq!(preview.structured_content.unwrap()["changed"], true);
    assert_eq!(fs::read(dir.path().join("project.xgwx")).unwrap(), original);
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    let mut args = args;
    args["dry_run"] = json!(false);
    let saved = client
        .call_tool(call("apply_patch", args.clone()))
        .await
        .unwrap();
    assert_ne!(saved.is_error, Some(true));
    let saved = saved.structured_content.unwrap();
    assert_eq!(
        fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
        original
    );
    let stale = client.call_tool(call("apply_patch", args)).await.unwrap();
    assert_eq!(stale.is_error, Some(true));
    assert_eq!(
        stale.structured_content.unwrap()["error"]["code"],
        "stale_revision"
    );
    let next = client
        .call_tool(call(
            "get_ir",
            json!({"path":"project.xgwx","program_index":0}),
        ))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    assert_eq!(next["revision"], saved["revision_after"]);
    assert!(next["text"].as_str().unwrap().contains(" NC "));
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn real_stdio_blank_program_creation_deletion_and_verified_operation_guidance() {
    let dir = tempfile::tempdir().unwrap();
    let original = fs::read(fixture_root().join("fixtures/empty-projects/new-xgk.xgwx")).unwrap();
    fs::write(dir.path().join("blank.xgwx"), &original).unwrap();
    let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_xgwx-mcp"));
    command.current_dir(dir.path());
    let client = ClientConfig::default()
        .serve(TokioChildProcess::new(command).unwrap())
        .await
        .unwrap();
    let call = |name: &str, args: Value| {
        CallToolRequestParams::new(name.to_string())
            .with_arguments(args.as_object().unwrap().clone())
    };
    let described = client
        .call_tool(call(
            "describe_operation",
            json!({"name":"edit_ladder_cell"}),
        ))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    assert!(
        described["validation_evidence"]["bundled_fixture_example_count"]
            .as_u64()
            .unwrap()
            > 0
    );
    assert_eq!(described["examples"][0]["kind"], "fixture_verified");
    assert_eq!(
        described["inspection_requirements"][0]["section"],
        "program.cells"
    );
    let ir = client
        .call_tool(call(
            "get_ir",
            json!({"path":"blank.xgwx","program_index":0}),
        ))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    assert!(
        ir["capabilities"]
            .as_array()
            .unwrap()
            .contains(&json!("xgk_element_creation_deletion"))
    );
    let text = ir["text"].as_str().unwrap();
    let header = "  rung @p0.r0 at(y=0) {";
    let edited=text.replace(header,&format!("{header}\n    contact @new-input at(x=1,y=0) NO \"M00000\"\n    coil @new-output at(x=94,y=0) OUT \"M00001\""));
    let patch = |before: &str, after: &str| {
        format!(
            "@@ -1,{} +1,{} @@\n{}{}",
            before.lines().count(),
            after.lines().count(),
            before
                .lines()
                .map(|l| format!("-{l}\n"))
                .collect::<String>(),
            after.lines().map(|l| format!("+{l}\n")).collect::<String>()
        )
    };
    let mut args = json!({"path":"blank.xgwx","program_index":0,"expected_revision":ir["revision"],"patch":patch(text,&edited),"dry_run":true});
    let result = client
        .call_tool(call("apply_patch", args.clone()))
        .await
        .unwrap();
    assert_ne!(result.is_error, Some(true));
    assert_eq!(fs::read(dir.path().join("blank.xgwx")).unwrap(), original);
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    args["dry_run"] = json!(false);
    let saved = client.call_tool(call("apply_patch", args)).await.unwrap();
    assert_ne!(saved.is_error, Some(true));
    let saved = saved.structured_content.unwrap();
    assert_eq!(
        fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
        original
    );
    let next = client
        .call_tool(call(
            "get_ir",
            json!({"path":"blank.xgwx","program_index":0}),
        ))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    let text = next["text"].as_str().unwrap();
    assert!(text.contains("at(x=94,y=0) OUT"));
    let edited = text
        .lines()
        .filter(|l| !l.trim_start().starts_with("contact ") && !l.trim_start().starts_with("coil "))
        .map(|l| format!("{l}\n"))
        .collect::<String>();
    let deleted=client.call_tool(call("apply_patch",json!({"path":"blank.xgwx","program_index":0,"expected_revision":next["revision"],"patch":patch(text,&edited)}))).await.unwrap();
    assert_ne!(deleted.is_error, Some(true));
    let last = client
        .call_tool(call(
            "get_ir",
            json!({"path":"blank.xgwx","program_index":0}),
        ))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    assert!(
        !last["text"]
            .as_str()
            .unwrap()
            .lines()
            .any(|l| l.trim_start().starts_with("contact ") || l.trim_start().starts_with("coil "))
    );
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn real_stdio_iec_connected_deletion_and_physical_wire_transactions() {
    let dir = tempfile::tempdir().unwrap();
    let blank = fs::read(fixture_root().join("fixtures/empty-projects/new-xgi.xgwx")).unwrap();
    let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_xgwx-mcp"));
    command.current_dir(dir.path());
    let client = ClientConfig::default()
        .serve(TokioChildProcess::new(command).unwrap())
        .await
        .unwrap();
    let call = |name: &str, args: Value| {
        CallToolRequestParams::new(name.to_string())
            .with_arguments(args.as_object().unwrap().clone())
    };
    for (path, prefix, kind) in [
        ("wire.xgwx", "wire ", "HORIZONTAL"),
        ("contact.xgwx", "contact ", ""),
    ] {
        fs::write(dir.path().join(path), &blank).unwrap();
        let initial = client
            .call_tool(call("inspect_file", json!({"path":path})))
            .await
            .unwrap()
            .structured_content
            .unwrap();
        let setup = json!([
            {"name":"insert_iec_ld_single_element","args":{"program_index":0,"row_index":0,"raw_x":1,"category":"contact","kind":"NO","operand":"%MX0"}},
            {"name":"insert_iec_ld_single_element","args":{"program_index":0,"row_index":0,"raw_x":94,"category":"coil","kind":"OUTPUT","operand":"%MX1"}}
        ]);
        let result = client
            .call_tool(call(
                "apply_edits",
                json!({"path":path,"expected_revision":initial["revision"],"operations":setup}),
            ))
            .await
            .unwrap();
        assert_ne!(result.is_error, Some(true));
        let original = fs::read(dir.path().join(path)).unwrap();
        let ir = client
            .call_tool(call("get_ir", json!({"path":path,"program_index":0})))
            .await
            .unwrap()
            .structured_content
            .unwrap();
        assert!(
            ir["capabilities"]
                .as_array()
                .unwrap()
                .contains(&json!("physical_wire_deletion"))
        );
        let text = ir["text"].as_str().unwrap();
        let (index, line) = text
            .lines()
            .enumerate()
            .find(|(_, l)| l.trim_start().starts_with(prefix) && l.ends_with(kind))
            .unwrap();
        let patch = format!("@@ -{},1 +{},0 @@\n-{}\n", index + 1, index, line);
        let mut args = json!({"path":path,"program_index":0,"expected_revision":ir["revision"],"patch":patch,"dry_run":true});
        let preview = client
            .call_tool(call("apply_patch", args.clone()))
            .await
            .unwrap();
        assert_ne!(preview.is_error, Some(true));
        assert_eq!(fs::read(dir.path().join(path)).unwrap(), original);
        args["dry_run"] = json!(false);
        let saved = client
            .call_tool(call("apply_patch", args.clone()))
            .await
            .unwrap();
        assert_ne!(saved.is_error, Some(true));
        let saved = saved.structured_content.unwrap();
        assert_eq!(
            fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
            original
        );
        assert_ne!(saved["revision_after"], ir["revision"]);
        let next = client
            .call_tool(call("get_ir", json!({"path":path,"program_index":0})))
            .await
            .unwrap()
            .structured_content
            .unwrap();
        assert_eq!(next["revision"], saved["revision_after"]);
        assert!(
            !next["text"]
                .as_str()
                .unwrap()
                .lines()
                .any(|l| l.trim_start().starts_with(prefix) && l.ends_with(kind))
        );
        let stale = client.call_tool(call("apply_patch", args)).await.unwrap();
        assert_eq!(
            stale.structured_content.unwrap()["error"]["code"],
            "stale_revision"
        );
    }
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn real_stdio_iec_function_patch_preview_save_and_guard_rejection() {
    let dir = tempfile::tempdir().unwrap();
    let original =
        fs::read(fixture_root().join("fixtures/function-output-wires/base.xgwx")).unwrap();
    let path = dir.path().join("project.xgwx");
    fs::write(&path, &original).unwrap();
    let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_xgwx-mcp"));
    command.current_dir(dir.path());
    let client = ClientConfig::default()
        .serve(TokioChildProcess::new(command).unwrap())
        .await
        .unwrap();
    let call = |name: &str, args: Value| {
        CallToolRequestParams::new(name.to_string())
            .with_arguments(args.as_object().unwrap().clone())
    };
    let ir = client
        .call_tool(call(
            "get_ir",
            json!({"path":"project.xgwx","program_index":0}),
        ))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    assert!(
        ir["capabilities"]
            .as_array()
            .unwrap()
            .contains(&json!("iec_function_operand_edit"))
    );
    let text = ir["text"].as_str().unwrap();
    let function = text.lines().find(|l| l.contains("name \"ADD\"")).unwrap();
    let argument = text
        .lines()
        .find(|l| l.trim_start().starts_with("argument ") && l.contains("direction INPUT"))
        .unwrap();
    assert!(argument.contains("pin_row ") && argument.contains("pin_x "));
    let edited = text
        .replace(function, &function.replace("name \"ADD\"", "name \"SUB\""))
        .replace(
            argument,
            &format!(
                "{} value \"12345\"",
                argument.split(" value ").next().unwrap()
            ),
        );
    let patch = format!(
        "@@ -1,{} +1,{} @@\n{}{}",
        text.lines().count(),
        edited.lines().count(),
        text.lines().map(|l| format!("-{l}\n")).collect::<String>(),
        edited
            .lines()
            .map(|l| format!("+{l}\n"))
            .collect::<String>()
    );
    let mut args = json!({"path":"project.xgwx","program_index":0,"expected_revision":ir["revision"],"patch":patch,"dry_run":true});
    let preview = client
        .call_tool(call("apply_patch", args.clone()))
        .await
        .unwrap();
    assert_ne!(preview.is_error, Some(true));
    assert_eq!(fs::read(&path).unwrap(), original);
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    args["dry_run"] = json!(false);
    let saved = client.call_tool(call("apply_patch", args)).await.unwrap();
    assert_ne!(saved.is_error, Some(true));
    let saved = saved.structured_content.unwrap();
    assert_eq!(
        fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
        original
    );
    let next = client
        .call_tool(call(
            "get_ir",
            json!({"path":"project.xgwx","program_index":0}),
        ))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    assert_eq!(next["revision"], saved["revision_after"]);
    let text = next["text"].as_str().unwrap();
    assert!(text.contains("name \"SUB\"") && text.contains("value \"12345\""));
    let (index, line) = text
        .lines()
        .enumerate()
        .find(|(_, l)| l.trim_start().starts_with("argument ") && l.contains("direction OUTPUT"))
        .unwrap();
    let patch = format!(
        "@@ -{},1 +{},1 @@\n-{line}\n+{} value \"123\"\n",
        index + 1,
        index + 1,
        line.split(" value ").next().unwrap()
    );
    let before = fs::read(&path).unwrap();
    let rejected = client.call_tool(call("apply_patch",json!({"path":"project.xgwx","program_index":0,"expected_revision":next["revision"],"patch":patch}))).await.unwrap();
    assert_eq!(rejected.is_error, Some(true));
    assert_eq!(
        rejected.structured_content.unwrap()["error"]["code"],
        "operation_failed"
    );
    assert_eq!(fs::read(&path).unwrap(), before);
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn real_stdio_xgk_instruction_creation_update_backup_and_stale_revision() {
    let dir = tempfile::tempdir().unwrap();
    let original = fs::read(fixture_root().join("fixtures/empty-projects/new-xgk.xgwx")).unwrap();
    let path = dir.path().join("project.xgwx");
    fs::write(&path, &original).unwrap();
    let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_xgwx-mcp"));
    command.current_dir(dir.path());
    let client = ClientConfig::default()
        .serve(TokioChildProcess::new(command).unwrap())
        .await
        .unwrap();
    let call = |name: &str, args: Value| {
        CallToolRequestParams::new(name.to_string())
            .with_arguments(args.as_object().unwrap().clone())
    };
    let ir = client
        .call_tool(call(
            "get_ir",
            json!({"path":"project.xgwx","program_index":0}),
        ))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    assert!(
        ir["capabilities"]
            .as_array()
            .unwrap()
            .contains(&json!("xgk_instruction_mnemonic_and_operands"))
    );
    let (i, line) = ir["text"]
        .as_str()
        .unwrap()
        .lines()
        .enumerate()
        .find(|(_, l)| l.trim_start().starts_with("rung "))
        .unwrap();
    let patch = format!(
        "@@ -{},1 +{},2 @@\n {line}\n+    instruction @new-move at(x=88,y=0) APPLICATION mnemonic \"MOV\" operands \"1\" \"D000000\"\n",
        i + 1,
        i + 1
    );
    let mut args = json!({"path":"project.xgwx","program_index":0,"expected_revision":ir["revision"],"patch":patch,"dry_run":true});
    let preview = client
        .call_tool(call("apply_patch", args.clone()))
        .await
        .unwrap();
    assert_ne!(preview.is_error, Some(true));
    assert_eq!(fs::read(&path).unwrap(), original);
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    args["dry_run"] = json!(false);
    let saved = client
        .call_tool(call("apply_patch", args.clone()))
        .await
        .unwrap();
    assert_ne!(saved.is_error, Some(true));
    let saved = saved.structured_content.unwrap();
    assert_eq!(
        fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
        original
    );
    let stale = client.call_tool(call("apply_patch", args)).await.unwrap();
    assert_eq!(
        stale.structured_content.unwrap()["error"]["code"],
        "stale_revision"
    );
    let next = client
        .call_tool(call(
            "get_ir",
            json!({"path":"project.xgwx","program_index":0}),
        ))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    assert_eq!(next["revision"], saved["revision_after"]);
    let (i, line) = next["text"]
        .as_str()
        .unwrap()
        .lines()
        .enumerate()
        .find(|(_, l)| l.trim_start().starts_with("instruction "))
        .unwrap();
    let changed = line.replace(
        "mnemonic \"MOV\" operands \"1\" \"D000000\"",
        "mnemonic \"ADD\" operands \"1\" \"2\" \"D000000\"",
    );
    let patch = format!("@@ -{},1 +{},1 @@\n-{line}\n+{changed}\n", i + 1, i + 1);
    let before = fs::read(&path).unwrap();
    let updated = client.call_tool(call("apply_patch",json!({"path":"project.xgwx","program_index":0,"expected_revision":next["revision"],"patch":patch}))).await.unwrap();
    assert_ne!(updated.is_error, Some(true));
    assert_eq!(
        fs::read(
            updated.structured_content.unwrap()["backup_path"]
                .as_str()
                .unwrap()
        )
        .unwrap(),
        before
    );
    let next = client
        .call_tool(call(
            "get_ir",
            json!({"path":"project.xgwx","program_index":0}),
        ))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    assert!(
        next["text"]
            .as_str()
            .unwrap()
            .contains("at(x=85,y=0) APPLICATION mnemonic \"ADD\"")
    );
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn real_stdio_sfc_row_patch_preview_save_and_native_guard_failure() {
    let dir = tempfile::tempdir().unwrap();
    let original = fs::read(fixture_root().join("fixtures/sfc/native-loop.xgwx")).unwrap();
    let path = dir.path().join("project.xgwx");
    fs::write(&path, &original).unwrap();
    let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_xgwx-mcp"));
    command.current_dir(dir.path());
    let client = ClientConfig::default()
        .serve(TokioChildProcess::new(command).unwrap())
        .await
        .unwrap();
    let call = |name: &str, args: Value| {
        CallToolRequestParams::new(name.to_string())
            .with_arguments(args.as_object().unwrap().clone())
    };
    let ir = client
        .call_tool(call(
            "get_ir",
            json!({"path":"project.xgwx","program_index":0}),
        ))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    assert_eq!(ir["dialect"], "SFC");
    assert!(
        ir["capabilities"]
            .as_array()
            .unwrap()
            .contains(&json!("sfc_existing_row_fields"))
    );
    assert!(
        !ir["capabilities"]
            .as_array()
            .unwrap()
            .contains(&json!("contact_kind_and_operand"))
    );
    let (i, line) = ir["text"]
        .as_str()
        .unwrap()
        .lines()
        .enumerate()
        .find(|(_, l)| l.contains("comment \"Ready for cycle\""))
        .unwrap();
    let patch = format!(
        "@@ -{},1 +{},1 @@\n-{line}\n+{}\n",
        i + 1,
        i + 1,
        line.replace("Ready for cycle", "MCP SFC edit")
    );
    let mut args = json!({"path":"project.xgwx","program_index":0,"expected_revision":ir["revision"],"patch":patch,"dry_run":true});
    let preview = client
        .call_tool(call("apply_patch", args.clone()))
        .await
        .unwrap();
    assert_ne!(preview.is_error, Some(true));
    assert_eq!(fs::read(&path).unwrap(), original);
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    args["dry_run"] = json!(false);
    let saved = client
        .call_tool(call("apply_patch", args.clone()))
        .await
        .unwrap();
    assert_ne!(saved.is_error, Some(true));
    let saved = saved.structured_content.unwrap();
    assert_eq!(
        fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
        original
    );
    let stale = client.call_tool(call("apply_patch", args)).await.unwrap();
    assert_eq!(
        stale.structured_content.unwrap()["error"]["code"],
        "stale_revision"
    );
    let next = client
        .call_tool(call(
            "get_ir",
            json!({"path":"project.xgwx","program_index":0}),
        ))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    assert_eq!(next["revision"], saved["revision_after"]);
    assert!(next["text"].as_str().unwrap().contains("MCP SFC edit"));
    let (i, line) = next["text"]
        .as_str()
        .unwrap()
        .lines()
        .enumerate()
        .find(|(_, l)| l.contains("title \"%MX0\""))
        .unwrap();
    let patch = format!(
        "@@ -{},1 +{},1 @@\n-{line}\n+{}\n",
        i + 1,
        i + 1,
        line.replace("%MX0", "%MW2")
    );
    let before = fs::read(&path).unwrap();
    let rejected=client.call_tool(call("apply_patch",json!({"path":"project.xgwx","program_index":0,"expected_revision":next["revision"],"patch":patch}))).await.unwrap();
    assert_eq!(rejected.is_error, Some(true));
    assert_eq!(
        rejected.structured_content.unwrap()["error"]["code"],
        "operation_failed"
    );
    assert_eq!(fs::read(&path).unwrap(), before);
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn real_stdio_sfc_blank_chart_authoring_and_clear_are_verified_transactions() {
    let dir = tempfile::tempdir().unwrap();
    let original = fs::read(fixture_root().join("fixtures/sfc/new-xgi-sfc.xgwx")).unwrap();
    let path = dir.path().join("project.xgwx");
    fs::write(&path, &original).unwrap();
    let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_xgwx-mcp"));
    command.current_dir(dir.path());
    let client = ClientConfig::default()
        .serve(TokioChildProcess::new(command).unwrap())
        .await
        .unwrap();
    let call = |name: &str, args: Value| {
        CallToolRequestParams::new(name.to_string())
            .with_arguments(args.as_object().unwrap().clone())
    };
    let ir = client
        .call_tool(call(
            "get_ir",
            json!({"path":"project.xgwx","program_index":0}),
        ))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    assert!(
        ir["capabilities"]
            .as_array()
            .unwrap()
            .contains(&json!("sfc_linear_row_creation_deletion_reordering"))
    );
    let (i, line) = ir["text"]
        .as_str()
        .unwrap()
        .lines()
        .enumerate()
        .find(|(_, l)| l.trim_start().starts_with("chart "))
        .unwrap();
    let row = "    sfc_row @new-idle kind \"step\" title \"Idle\" comment \"\" initial true action none qualifier none time none action_code none transition_code none position none branch_end none";
    let patch = format!("@@ -{},1 +{},2 @@\n {line}\n+{row}\n", i + 1, i + 1);
    let mut args = json!({"path":"project.xgwx","program_index":0,"expected_revision":ir["revision"],"patch":patch,"dry_run":true});
    let preview = client
        .call_tool(call("apply_patch", args.clone()))
        .await
        .unwrap();
    assert_ne!(preview.is_error, Some(true));
    assert_eq!(fs::read(&path).unwrap(), original);
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    args["dry_run"] = json!(false);
    let saved = client
        .call_tool(call("apply_patch", args.clone()))
        .await
        .unwrap();
    assert_ne!(saved.is_error, Some(true));
    let saved = saved.structured_content.unwrap();
    assert_eq!(
        fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
        original
    );
    let stale = client.call_tool(call("apply_patch", args)).await.unwrap();
    assert_eq!(
        stale.structured_content.unwrap()["error"]["code"],
        "stale_revision"
    );
    let next = client
        .call_tool(call(
            "get_ir",
            json!({"path":"project.xgwx","program_index":0}),
        ))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    assert_eq!(next["revision"], saved["revision_after"]);
    let (i, line) = next["text"]
        .as_str()
        .unwrap()
        .lines()
        .enumerate()
        .find(|(_, l)| l.trim_start().starts_with("sfc_row "))
        .unwrap();
    assert!(line.contains("title \"Idle\""));
    let patch = format!("@@ -{},1 +{},0 @@\n-{line}\n", i + 1, i);
    let before = fs::read(&path).unwrap();
    let cleared=client.call_tool(call("apply_patch",json!({"path":"project.xgwx","program_index":0,"expected_revision":next["revision"],"patch":patch}))).await.unwrap();
    assert_ne!(cleared.is_error, Some(true));
    assert_eq!(
        fs::read(
            cleared.structured_content.unwrap()["backup_path"]
                .as_str()
                .unwrap()
        )
        .unwrap(),
        before
    );
    let next = client
        .call_tool(call(
            "get_ir",
            json!({"path":"project.xgwx","program_index":0}),
        ))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    assert!(
        !next["text"]
            .as_str()
            .unwrap()
            .lines()
            .any(|l| l.trim_start().starts_with("sfc_row "))
    );
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn real_stdio_template_discovery_create_preview_save_and_first_edit() {
    let dir = tempfile::tempdir().unwrap();
    let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_xgwx-mcp"));
    command.current_dir(dir.path());
    let client = ClientConfig::default()
        .serve(TokioChildProcess::new(command).unwrap())
        .await
        .unwrap();
    let call = |name: &str, args: Value| {
        CallToolRequestParams::new(name.to_string())
            .with_arguments(args.as_object().unwrap().clone())
    };
    let tools = client.list_all_tools().await.unwrap();
    let create = tools.iter().find(|t| t.name == "create_file").unwrap();
    assert_eq!(create.input_schema["additionalProperties"], false);
    assert!(create.input_schema["properties"]["template"].is_object());
    assert_eq!(create.input_schema["required"], json!(["path"]));
    for field in [
        "project_name",
        "cpu_model",
        "program_language",
        "program_name",
    ] {
        assert!(create.input_schema["properties"][field].is_object());
    }
    assert_eq!(
        create.annotations.as_ref().unwrap().destructive_hint,
        Some(false)
    );
    let templates = client
        .call_tool(call("list_templates", json!({})))
        .await
        .unwrap();
    assert_ne!(templates.is_error, Some(true));
    assert_eq!(
        templates.structured_content.unwrap()["templates"]
            .as_array()
            .unwrap()
            .len(),
        6
    );
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 0);
    for template in ["xgk_ld", "xgi_ld", "xgi_sfc"] {
        let path = format!("{template}.xgwx");
        let count = fs::read_dir(dir.path()).unwrap().count();
        let mut args = json!({"path":path,"project_name":"Factory & Line 1","program_name":"Main","cpu_model":if template == "xgk_ld" {"XGK-CPUH"} else {"XGI-CPUE"},"program_language":if template == "xgi_sfc" {"SFC"} else {"LD"},"operations":[{"name":"update_program","args":{"program_index":0,"patch":{"comment":"Created over MCP"}}}],"dry_run":true});
        let preview = client
            .call_tool(call("create_file", args.clone()))
            .await
            .unwrap();
        assert_ne!(preview.is_error, Some(true));
        let preview = preview.structured_content.unwrap();
        assert_eq!(preview["created"], false);
        assert!(preview["backup_path"].is_null());
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), count);
        args["dry_run"] = json!(false);
        let saved = client
            .call_tool(call("create_file", args.clone()))
            .await
            .unwrap();
        assert_ne!(saved.is_error, Some(true));
        let saved = saved.structured_content.unwrap();
        assert_eq!(saved["created"], true);
        assert_eq!(saved["project_name"], "Factory & Line 1");
        assert_eq!(saved["program_name"], "Main");
        assert!(saved["backup_path"].is_null());
        assert_eq!(saved["revision_after"], preview["revision_after"]);
        let original = fs::read(dir.path().join(&path)).unwrap();
        let exists = client.call_tool(call("create_file", args)).await.unwrap();
        assert_eq!(
            exists.structured_content.unwrap()["error"]["code"],
            "file_exists"
        );
        assert_eq!(fs::read(dir.path().join(&path)).unwrap(), original);
        let inspected = client
            .call_tool(call("inspect_file", json!({"path":path})))
            .await
            .unwrap()
            .structured_content
            .unwrap();
        assert_eq!(inspected["revision"], saved["revision_after"]);
        let ir = client
            .call_tool(call("get_ir", json!({"path":path,"program_index":0})))
            .await
            .unwrap();
        assert_ne!(ir.is_error, Some(true));
        assert_eq!(
            ir.structured_content.unwrap()["dialect"],
            match template {
                "xgk_ld" => "XGK",
                "xgi_ld" => "IEC",
                _ => "SFC",
            }
        );
        let edited=client.call_tool(call("apply_edits",json!({"path":path,"expected_revision":saved["revision_after"],"operations":[{"name":"update_program","args":{"program_index":0,"patch":{"comment":"First ordinary edit"}}}]}))).await.unwrap();
        assert_ne!(edited.is_error, Some(true));
        assert_eq!(
            fs::read(
                edited.structured_content.unwrap()["backup_path"]
                    .as_str()
                    .unwrap()
            )
            .unwrap(),
            original
        );
    }
    let bad = client
        .call_tool(call(
            "create_file",
            json!({"path":"bad.xgwx","template":"unknown"}),
        ))
        .await
        .unwrap();
    assert_eq!(bad.is_error, Some(true));
    assert!(!dir.path().join("bad.xgwx").exists());
    for (language, cpu) in [("ST", "XGK-CPUH"), ("SFC", "XGK-CPUH")] {
        let invalid = client.call_tool(call("create_file", json!({"path":"invalid-choice.xgwx","project_name":"Factory","cpu_model":cpu,"program_language":language}))).await.unwrap();
        assert_eq!(invalid.is_error, Some(true));
        assert!(!dir.path().join("invalid-choice.xgwx").exists());
    }
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn real_stdio_focused_lists_property_exports_and_guarded_patch_transactions() {
    let dir = tempfile::tempdir().unwrap();
    let mut doc =
        xgwx::XgwxDocument::from_path(fixture_root().join("fixtures/empty-projects/new-xgk.xgwx"))
            .unwrap();
    doc.base_mut()
        .by_number(0)
        .and_then(|mut selected_record| selected_record.insert_module(0, "XGI-D24A/B"))
        .unwrap();
    doc.variable_mut().generate_io_variables().unwrap();
    fs::write(
        dir.path().join("sample.xgwx"),
        doc.to_verified_bytes().unwrap(),
    )
    .unwrap();
    let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_xgwx-mcp"));
    command.current_dir(dir.path());
    let client = ClientConfig::default()
        .serve(TokioChildProcess::new(command).unwrap())
        .await
        .unwrap();
    let call = |name: &str, args: Value| {
        CallToolRequestParams::new(name.to_string())
            .with_arguments(args.as_object().unwrap().clone())
    };
    let catalog = client
        .call_tool(call(
            "list_module_catalog",
            json!({"query":"XGI-D24","limit":1}),
        ))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    assert_eq!(catalog["items"][0]["model"], "XGI-D24A/B");
    assert_eq!(catalog["total"], 1);
    for name in ["list_modules", "list_networks", "list_variables"] {
        let r = client
            .call_tool(call(name, json!({"path":"sample.xgwx","limit":1})))
            .await
            .unwrap();
        assert_ne!(r.is_error, Some(true));
        assert!(r.structured_content.unwrap()["revision"].is_string());
    }
    let vars = client
        .call_tool(call(
            "list_variables",
            json!({"path":"sample.xgwx","limit":1,"program_index":0}),
        ))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    assert_eq!(vars["data"]["global"]["total"], 32);
    assert_eq!(vars["data"]["global"]["next_offset"], 1);
    for (section, property, replacement) in [
        ("modules", "  edit option.inputFilter.0 = ", "3"),
        ("variables", "  edit description = ", "\"Start input\""),
        ("networks", "  edit name = ", "\"Plant network\""),
    ] {
        let exported = client
            .call_tool(call(
                "get_ir",
                json!({"path":"sample.xgwx","section":section}),
            ))
            .await
            .unwrap()
            .structured_content
            .unwrap();
        assert_eq!(exported["format"], "xgwx-properties/1");
        let text = exported["text"].as_str().unwrap();
        let (i, old) = text
            .lines()
            .enumerate()
            .find(|(_, l)| l.starts_with(property))
            .unwrap();
        let new = format!("{property}{replacement}");
        let patch = format!(
            "--- props\n+++ props\n@@ -{},1 +{},1 @@\n-{old}\n+{new}\n",
            i + 1,
            i + 1
        );
        let mut args = json!({"path":"sample.xgwx","section":section,"expected_revision":exported["revision"],"patch":patch,"dry_run":true});
        let preview = client
            .call_tool(call("apply_patch", args.clone()))
            .await
            .unwrap();
        assert_ne!(preview.is_error, Some(true));
        let preview = preview.structured_content.unwrap();
        assert!(preview["backup_path"].is_null());
        args["dry_run"] = json!(false);
        let saved = client
            .call_tool(call("apply_patch", args.clone()))
            .await
            .unwrap();
        assert_ne!(saved.is_error, Some(true));
        let saved = saved.structured_content.unwrap();
        assert_eq!(saved["revision_after"], preview["revision_after"]);
        assert!(saved["backup_path"].is_string());
        let stale = client.call_tool(call("apply_patch", args)).await.unwrap();
        assert_eq!(
            stale.structured_content.unwrap()["error"]["code"],
            "stale_revision"
        );
    }
    let missing = client
        .call_tool(call("get_ir", json!({"path":"sample.xgwx"})))
        .await
        .unwrap();
    assert_eq!(
        missing.structured_content.unwrap()["error"]["code"],
        "invalid_arguments"
    );
    client.cancel().await.unwrap();
}
