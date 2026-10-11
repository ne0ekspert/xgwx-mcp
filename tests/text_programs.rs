use serde_json::{Value, json};
use std::fs;
use xgwx::XgwxDocument;
#[cfg(not(feature = "libxgwx-namespaced-api"))]
use xgwx_mcp::library_compat::DocumentNamespacesCompat;
use xgwx_mcp::library_compat::module_summaries;
use xgwx_mcp::{
    inspection,
    operations::apply_operation,
    storage::{CreateProjectOptions, EditOperation, FileStore},
    templates::{ProgramLanguage, ProjectTemplate},
    text_ir::Projection,
};

fn source_op(doc: &XgwxDocument, source: &str) -> EditOperation {
    let text = doc.program().text_programs().remove(0);
    EditOperation {
        name: "edit_text_program".into(),
        args: json!({"patch": {
            "program_index":text.program_index,"expected_object_id":text.object_id,
            "expected_language":text.language,"expected_source":text.source.unwrap(),"source":source
        }}),
    }
}
fn replace_line(text: &str, from: &str, to: &str) -> String {
    let (index, line) = text
        .lines()
        .enumerate()
        .find(|(_, l)| l.contains(from))
        .unwrap();
    format!(
        "@@ -{},1 +{},1 @@\n-{}\n+{}\n",
        index + 1,
        index + 1,
        line,
        line.replace(from, to)
    )
}
#[test]
fn source_transactions_preserve_payloads_and_expected_checks() {
    for template in [ProjectTemplate::XgiSt, ProjectTemplate::XgiIl] {
        let dir = tempfile::tempdir().unwrap();
        let store = FileStore::new(dir.path().into(), vec![]).unwrap();
        let created = store
            .create_file("text.xgwx", template, None, &[], false)
            .unwrap();
        let rev = created["revision_after"].as_str().unwrap();
        let original = fs::read(dir.path().join("text.xgwx")).unwrap();
        let doc = XgwxDocument::parse(&original).unwrap();
        let source = if template == ProjectTemplate::XgiSt {
            "(* 한글 😀 *)\r\n%MX0 := TRUE;\r\n"
        } else {
            "(* 한글 😀 *)\r\nLD TRUE\r\nST %MX0\r\nRET\r\n"
        };
        let op = source_op(&doc, source);
        let preview = store
            .apply("text.xgwx", rev, std::slice::from_ref(&op), true)
            .unwrap();
        assert_eq!(fs::read(dir.path().join("text.xgwx")).unwrap(), original);
        assert!(preview["backup_path"].is_null());
        let mut stale = op.clone();
        stale.args["patch"]["expected_source"] = json!("stale");
        assert!(
            store
                .apply("text.xgwx", rev, &[op.clone(), stale], false)
                .is_err()
        );
        assert_eq!(fs::read(dir.path().join("text.xgwx")).unwrap(), original);
        assert_eq!(
            fs::read_dir(dir.path())
                .unwrap()
                .filter_map(Result::ok)
                .filter(|e| e.file_name().to_string_lossy().contains("backup"))
                .count(),
            0
        );
        let saved = store.apply("text.xgwx", rev, &[op], false).unwrap();
        assert_eq!(saved["revision_after"], preview["revision_after"]);
        assert_eq!(
            fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
            original
        );
        let bytes = fs::read(dir.path().join("text.xgwx")).unwrap();
        let after = XgwxDocument::parse(&bytes).unwrap();
        assert_eq!(
            after.program().text_programs()[0].source.as_deref(),
            Some(source)
        );
        assert_eq!(
            after.program().text_programs()[0].variables,
            doc.program().text_programs()[0].variables
        );
        assert_eq!(module_summaries(&after), module_summaries(&doc));
        assert_eq!(after.header.signature, doc.header.signature);
        assert_eq!(after.header.label, doc.header.label);
        // Main-member alignment padding can change; native trailer payload stays exact.
        assert_eq!(
            after
                .trailer
                .iter()
                .skip_while(|b| **b == 0)
                .collect::<Vec<_>>(),
            doc.trailer
                .iter()
                .skip_while(|b| **b == 0)
                .collect::<Vec<_>>()
        );
        assert_eq!(after.program().programs(), doc.program().programs());
        // Only the CodeList element may change in the document XML.
        let without_source = |xml: &str| {
            let tree = roxmltree::Document::parse(xml).unwrap();
            let range = tree
                .descendants()
                .find(|n| n.has_tag_name("CodeList"))
                .unwrap()
                .range();
            format!("{}{}", &xml[..range.start], &xml[range.end..])
        };
        assert_eq!(without_source(&after.xml), without_source(&doc.xml));
        assert!(store.apply("text.xgwx", rev, &[], false).is_err());
        let current = saved["revision_after"].as_str().unwrap();
        let noop = store
            .apply("text.xgwx", current, &[source_op(&after, source)], false)
            .unwrap();
        assert!(noop["backup_path"].is_null());
        for (field, value) in [
            ("expected_language", json!("LD")),
            ("expected_object_id", json!("stale")),
            ("source", json!("\0")),
            ("source", json!("😀".repeat(32769))),
            ("unknown", json!(0)),
        ] {
            let mut bad = source_op(&after, source);
            bad.args["patch"][field] = value;
            assert!(
                store.apply("text.xgwx", current, &[bad], false).is_err(),
                "{field}"
            );
            assert_eq!(fs::read(dir.path().join("text.xgwx")).unwrap(), bytes);
        }
        let validation = inspection::validate(&after);
        assert_eq!(validation["container"]["status"], "verified");
        assert_eq!(
            validation["program_diagnostics"][0]["syntax_and_types"]["status"],
            "not_checked"
        );
        assert_eq!(validation["native_acceptance"]["status"], "unverified");
    }
}
#[test]
fn source_ir_preserves_line_endings_identity_and_readonly_layouts() {
    for template in [ProjectTemplate::XgiSt, ProjectTemplate::XgiIl] {
        let mut doc = template.document().unwrap();
        let op = source_op(&doc, "line one\r\nline two\nlast");
        apply_operation(&mut doc, &op.name, op.args).unwrap();
        let projection = Projection::export(&doc, "revision", 0).unwrap();
        let patch = replace_line(&projection.text, "line two", "edited");
        let ops = projection.lower_patch(&patch).unwrap();
        apply_operation(&mut doc, &ops[0].name, ops[0].args.clone()).unwrap();
        assert_eq!(
            doc.program().text_programs()[0].source.as_deref(),
            Some("line one\r\nedited\nlast")
        );
        assert!(
            projection
                .lower_patch(&replace_line(&projection.text, "object", "identity"))
                .is_err()
        );
        let unsupported = doc
            .xml
            .replace("Encrytption=\"\"", "Encrytption=\"encrypted\"");
        let unsupported = XgwxDocument::parse(
            &XgwxDocument {
                xml: unsupported,
                ..doc.clone()
            }
            .to_bytes()
            .unwrap(),
        )
        .unwrap();
        assert!(!unsupported.program().text_programs()[0].editable);
        let readonly = Projection::export(&unsupported, "revision", 0).unwrap();
        assert!(
            readonly
                .lower_patch(&replace_line(&readonly.text, "edited", "changed"))
                .is_err()
        );
        assert!(readonly.lower_patch("").unwrap().is_empty());
    }
}
#[tokio::test]
async fn stdio_st_and_il_creation_inspection_declarations_and_patch() {
    use rmcp::{
        ServiceExt,
        model::{CallToolRequestParams, ClientConfig},
        transport::TokioChildProcess,
    };
    let dir = tempfile::tempdir().unwrap();
    let mut command = tokio::process::Command::new(env!("CARGO_BIN_EXE_xgwx-mcp"));
    command.current_dir(dir.path());
    let client = ClientConfig::default()
        .serve(TokioChildProcess::new(command).unwrap())
        .await
        .unwrap();
    let call = |name: &str, args: Value| {
        CallToolRequestParams::new(name.to_owned())
            .with_arguments(args.as_object().unwrap().clone())
    };
    assert_eq!(client.list_all_tools().await.unwrap().len(), 13);
    for name in ["edit_text_program", "edit_text_variable"] {
        let result = client
            .call_tool(call("describe_operation", json!({"name":name})))
            .await
            .unwrap();
        assert_ne!(result.is_error, Some(true));
        assert_eq!(
            result.structured_content.unwrap()["argument_schema"]["additionalProperties"],
            false
        );
    }
    for language in ["ST", "IL"] {
        let path = format!("{language}.xgwx");
        let args = json!({"path":path,"cpu_model":"XGI-CPUE","program_language":language,"project_name":"TextProject","program_name":"Main","dry_run":true});
        let preview = client
            .call_tool(call("create_file", args.clone()))
            .await
            .unwrap();
        assert_ne!(preview.is_error, Some(true));
        assert!(!dir.path().join(&path).exists());
        let mut args = args;
        args["dry_run"] = json!(false);
        let created = client.call_tool(call("create_file", args)).await.unwrap();
        assert_ne!(created.is_error, Some(true));
        let created = created.structured_content.unwrap();
        let inspect = client
            .call_tool(call(
                "inspect_file",
                json!({"path":path,"section":"text","program_index":0}),
            ))
            .await
            .unwrap()
            .structured_content
            .unwrap();
        let text = &inspect["data"]["items"][0];
        assert_eq!(text["language"], language);
        assert_eq!(text["editable"], true);
        let vars = json!({"name":"edit_text_variable","args":{"expected_object_id":text["object_id"],"patch":{"program_index":0,"expected_variables":[],"name":"Run","data_type":"BOOL","description":"Run command","remove":false}}});
        let source = if language == "ST" {
            "Run := TRUE;\r\n"
        } else {
            "LD TRUE\r\nST Run\r\nRET\r\n"
        };
        let source = json!({"name":"edit_text_program","args":{"patch":{"program_index":0,"expected_object_id":text["object_id"],"expected_language":language,"expected_source":"","source":source}}});
        let result = client.call_tool(call("apply_edits",json!({"path":path,"expected_revision":created["revision_after"],"operations":[vars,source]}))).await.unwrap();
        assert_ne!(result.is_error, Some(true));
        let saved = result.structured_content.unwrap();
        let ir = client
            .call_tool(call("get_ir", json!({"path":path,"program_index":0})))
            .await
            .unwrap()
            .structured_content
            .unwrap();
        assert_eq!(ir["dialect"], language);
        let patch = replace_line(ir["text"].as_str().unwrap(), "TRUE", "FALSE");
        let args = json!({"path":path,"program_index":0,"expected_revision":saved["revision_after"],"patch":patch,"dry_run":true});
        let preview = client
            .call_tool(call("apply_patch", args.clone()))
            .await
            .unwrap();
        assert_ne!(preview.is_error, Some(true));
        let mut args = args;
        args["dry_run"] = json!(false);
        let saved = client.call_tool(call("apply_patch", args)).await.unwrap();
        assert_ne!(saved.is_error, Some(true));
        let after = XgwxDocument::from_path(dir.path().join(&path)).unwrap();
        assert!(
            after.program().text_programs()[0]
                .source
                .as_ref()
                .unwrap()
                .contains("FALSE")
        );
        assert!(
            after.program().text_programs()[0]
                .variables
                .iter()
                .any(|v| v.name == "Run")
        );
        let text = after.program().text_programs().remove(0);
        let remove = json!({"path":path,"expected_revision":saved.structured_content.unwrap()["revision_after"],"operations":[{"name":"edit_text_variable","args":{"expected_object_id":text.object_id,"patch":{"program_index":0,"expected_variables":text.variables.iter().cloned().map(xgwx_mcp::operation_types::SfcVariable::from).collect::<Vec<_>>(),"name":"Run","data_type":"BOOL","description":"","remove":true}}}]});
        let rejected = client.call_tool(call("apply_edits", remove)).await.unwrap();
        assert_eq!(rejected.is_error, Some(true)); // referenced declarations stay protected
    }
    client.cancel().await.unwrap();
}
#[test]
fn all_text_creation_choices_have_one_supported_source_program() {
    for language in [ProgramLanguage::St, ProgramLanguage::Il] {
        let dir = tempfile::tempdir().unwrap();
        let store = FileStore::new(dir.path().into(), vec![]).unwrap();
        for model in ["XGI-CPUE", "XGI-CPUU"] {
            let result = store
                .create_project(&CreateProjectOptions {
                    path: format!("{model}.xgwx"),
                    template: None,
                    cpu_model: Some(model.into()),
                    program_language: Some(language),
                    project_name: None,
                    program_name: None,
                    operations: vec![],
                    dry_run: true,
                })
                .unwrap();
            assert_eq!(result["validation"]["container"]["status"], "verified");
        }
    }
}

#[test]
fn vendor_il_updates_native_ladder_and_rejects_iec_or_branch_source() {
    let mut doc = ProjectTemplate::XgkIl.document().unwrap();
    let exported = inspection::inspect(&doc, "vendor_il", Some(0), 0, 100).unwrap();
    assert_eq!(exported["items"][0]["editable"], true);
    let initial = doc.clone();
    let text = doc.program().vendor_il_programs().remove(0);
    let op = EditOperation {
        name: "edit_vendor_il".into(),
        args: json!({"patch":{"program_index":0,"expected_object_id":text.object_id,"expected_source":text.source.unwrap(),"source":"LOAD M00000\nAND NOT M00001\nOUT M00002\nLOADP M00003\nMOV 1 D100"}}),
    };
    apply_operation(&mut doc, &op.name, op.args.clone()).unwrap();
    let doc = XgwxDocument::parse(&doc.to_verified_bytes().unwrap()).unwrap();
    assert_eq!(doc.program().programs()[0].kind, Some(0));
    assert_eq!(
        doc.program().vendor_il_programs()[0].source.as_deref(),
        op.args["patch"]["source"].as_str()
    );
    assert_eq!(
        doc.variable().variables().unwrap(),
        initial.variable().variables().unwrap()
    );
    assert_eq!(module_summaries(&doc), module_summaries(&initial));
    assert_eq!(doc.program().programs(), initial.program().programs());
    for source in [
        "LD TRUE\nST %MX0",
        "LOAD M00000\nOR M00001\nOUT M00002",
        "LOAD M00000",
    ] {
        let mut candidate = doc.clone();
        let mut args = op.args.clone();
        args["patch"]["expected_source"] = json!(doc.program().vendor_il_programs()[0].source);
        args["patch"]["source"] = json!(source);
        assert!(apply_operation(&mut candidate, "edit_vendor_il", args).is_err());
        assert_eq!(candidate, doc);
    }
}
