#[path = "../src/test_support.rs"]
mod test_support;
use serde_json::{Value, json};
use std::fs;
use test_support::fixture_root;
use xgwx::XgwxDocument;
use xgwx_mcp::{
    common::{self, Change, PrepareRequest, PreparedChanges},
    inspection,
    search::{self, SearchRequest},
    storage::FileStore,
};

fn setup(fixture: &str) -> (tempfile::TempDir, FileStore, Vec<u8>) {
    let dir = tempfile::tempdir().unwrap();
    let original = fs::read(fixture_root().join("fixtures").join(fixture)).unwrap();
    fs::write(dir.path().join("p.xgwx"), &original).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    (dir, store, original)
}
fn program_ref(store: &FileStore) -> String {
    let (path, rev, doc) = store.read("p.xgwx").unwrap();
    let mut data = inspection::inspect(&doc, "programs", None, 0, 100).unwrap();
    common::annotate(&doc, &path, &rev, &mut data);
    data["programs"]["items"][0]["object_ref"]
        .as_str()
        .unwrap()
        .to_owned()
}
fn contact_ref(store: &FileStore) -> (String, String) {
    let (path, rev, doc) = store.read("p.xgwx").unwrap();
    let p = doc.program().ladder_program(0).unwrap().unwrap();
    let operand = if p.project_type == Some(2) {
        let r = p
            .iec_record_frames()
            .unwrap()
            .into_iter()
            .find(|r| matches!(r.kind, xgwx::IecRecordKind::Contact(_)))
            .unwrap();
        p.strings
            .iter()
            .find(|s| s.offset == r.offset + 15)
            .unwrap()
            .value
            .clone()
    } else {
        p.structure
            .rungs
            .iter()
            .flat_map(|r| &r.cells)
            .find(|c| c.contact.is_some() && !c.value.is_empty())
            .unwrap()
            .value
            .clone()
    };
    let req: SearchRequest = serde_json::from_value(
        json!({"path":"p.xgwx","query":operand,"kind":"contact","program_index":0}),
    )
    .unwrap();
    let mut data = search::search(&doc, &req).unwrap();
    common::annotate(&doc, &path, &rev, &mut data);
    (
        data["items"][0]["object_ref"].as_str().unwrap().to_owned(),
        operand,
    )
}
fn module_ref(store: &FileStore) -> String {
    let (path, rev, doc) = store.read("p.xgwx").unwrap();
    let mut data = inspection::inspect(&doc, "hardware", None, 0, 100).unwrap();
    common::annotate(&doc, &path, &rev, &mut data);
    data["modules"]["items"][0]["object_ref"]
        .as_str()
        .unwrap()
        .to_owned()
}
fn module_setup() -> (tempfile::TempDir, FileStore, Vec<u8>) {
    let (dir, store, _) = setup("empty-projects/new-xgk.xgwx");
    let (_, _, mut doc) = store.read("p.xgwx").unwrap();
    doc.base_mut()
        .by_number(0)
        .and_then(|mut b| b.insert_module(0, "XGI-D24A/B"))
        .unwrap();
    let original = doc.to_verified_bytes().unwrap();
    fs::write(dir.path().join("p.xgwx"), &original).unwrap();
    (dir, store, original)
}
#[test]
fn rename_preview_save_backup_noop_and_reference_guards() {
    let (dir, store, original) = setup("elements.xgwx");
    let reference = program_ref(&store);
    let change = Change::RenameProgram {
        object_ref: reference.clone(),
        name: "원료 제어".into(),
    };
    let preview = common::apply(&store, "p.xgwx", change.clone(), true).unwrap();
    assert_eq!(preview["changes"][0]["after"], "원료 제어");
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), original);
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    fs::write(dir.path().join("copy.xgwx"), &original).unwrap();
    assert_eq!(
        common::apply(&store, "copy.xgwx", change.clone(), false)
            .unwrap_err()
            .code,
        "invalid_object_ref"
    );
    let mut forged = reference.clone();
    forged.push('0');
    assert_eq!(
        common::apply(
            &store,
            "p.xgwx",
            Change::RenameProgram {
                object_ref: forged,
                name: "Wrong".into()
            },
            false
        )
        .unwrap_err()
        .code,
        "invalid_object_ref"
    );
    let saved = common::apply(&store, "p.xgwx", change.clone(), false).unwrap();
    assert_eq!(saved["revision_after"], preview["revision_after"]);
    assert_eq!(
        fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
        original
    );
    let (_, _, after) = store.read("p.xgwx").unwrap();
    assert_eq!(
        after.program().programs()[0].name.as_deref(),
        Some("원료 제어")
    );
    let before = XgwxDocument::parse(&original).unwrap();
    assert_eq!(
        after.program().ladder_program(0).unwrap().unwrap().data,
        before.program().ladder_program(0).unwrap().unwrap().data
    );
    assert_eq!(
        after.variable().variables().unwrap(),
        before.variable().variables().unwrap()
    );
    assert_eq!(
        common::apply(&store, "p.xgwx", change, false)
            .unwrap_err()
            .code,
        "stale_revision"
    );
    let noop = common::apply(
        &store,
        "p.xgwx",
        Change::RenameProgram {
            object_ref: program_ref(&store),
            name: "원료 제어".into(),
        },
        false,
    )
    .unwrap();
    assert_eq!(noop["changed"], false);
    assert!(noop["backup_path"].is_null());
}
#[test]
fn contact_operand_changes_are_single_use_and_keep_layout_and_symbols() {
    for fixture in [
        "elements.xgwx",
        "ladder-wire-delete/iec-horizontal-native.xgwx",
    ] {
        let (_dir, store, original) = setup(fixture);
        let before = XgwxDocument::parse(&original).unwrap();
        let (reference, operand) = contact_ref(&store);
        let replacement = format!(
            "{}{}",
            &operand[..operand.len() - 1],
            if operand.ends_with('0') { '1' } else { '0' }
        );
        assert_eq!(
            common::apply(
                &store,
                "p.xgwx",
                Change::RenameProgram {
                    object_ref: reference.clone(),
                    name: "Wrong".into()
                },
                false
            )
            .unwrap_err()
            .code,
            "reference_kind_mismatch"
        );
        let preview = common::apply(
            &store,
            "p.xgwx",
            Change::SetContactOperand {
                object_ref: reference.clone(),
                operand: replacement.clone(),
            },
            true,
        )
        .unwrap();
        assert_eq!(preview["changes"][0]["before"], operand);
        assert_eq!(preview["changes"][0]["after"], replacement);
        let saved = common::apply(
            &store,
            "p.xgwx",
            Change::SetContactOperand {
                object_ref: reference,
                operand: replacement.clone(),
            },
            false,
        )
        .unwrap();
        assert_eq!(saved["revision_after"], preview["revision_after"]);
        let (_, _, after) = store.read("p.xgwx").unwrap();
        assert_eq!(
            after.variable().variables().unwrap(),
            before.variable().variables().unwrap()
        );
        let old = before.program().ladder_program(0).unwrap().unwrap();
        let new = after.program().ladder_program(0).unwrap().unwrap();
        assert_eq!(old.data.len(), new.data.len());
        let changed = old
            .strings
            .iter()
            .zip(&new.strings)
            .filter(|(a, b)| a.value != b.value)
            .collect::<Vec<_>>();
        assert_eq!(changed.len(), 1);
        assert_eq!(changed[0].0.value, operand);
        assert_eq!(changed[0].1.value, replacement);
    }
}
#[test]
fn module_comment_preview_and_commit_preserve_model_options_and_program() {
    let (_dir, store, original) = module_setup();
    let before = XgwxDocument::parse(&original).unwrap();
    let change = Change::SetModuleComment {
        object_ref: module_ref(&store),
        comment: "Feed conveyor".into(),
    };
    let result = common::apply(&store, "p.xgwx", change, false).unwrap();
    assert_eq!(result["changes"][0]["after"], "Feed conveyor");
    let (_, _, after) = store.read("p.xgwx").unwrap();
    let a = &after.module().summaries()[0];
    let b = &before.module().summaries()[0];
    assert_eq!(a.comment.as_deref(), Some("Feed conveyor"));
    assert_eq!(
        (&a.id, &a.sub_type, &a.name, &a.details),
        (&b.id, &b.sub_type, &b.name, &b.details)
    );
    assert_eq!(
        after.program().ladder_program(0).unwrap().unwrap().data,
        before.program().ladder_program(0).unwrap().unwrap().data
    );
}
#[test]
fn reviewed_batch_commits_exact_candidate_once_and_rejects_stale_source() {
    let (dir, store, original) = module_setup();
    let changes = vec![
        Change::RenameProgram {
            object_ref: program_ref(&store),
            name: "Conveyor".into(),
        },
        Change::SetModuleComment {
            object_ref: module_ref(&store),
            comment: "Drive outputs".into(),
        },
    ];
    let mut cache = PreparedChanges::default();
    let prepared = cache
        .prepare(
            &store,
            PrepareRequest {
                path: "p.xgwx".into(),
                changes: changes.clone(),
            },
        )
        .unwrap();
    assert_eq!(prepared["changes"].as_array().unwrap().len(), 2);
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), original);
    let token = prepared["change_token"].as_str().unwrap();
    let mut other_session = PreparedChanges::default();
    assert_eq!(
        other_session.commit(&store, token).unwrap_err().code,
        "invalid_change_token"
    );
    let other = other_session
        .prepare(
            &store,
            PrepareRequest {
                path: "p.xgwx".into(),
                changes: changes.clone(),
            },
        )
        .unwrap();
    assert_ne!(other["change_token"], prepared["change_token"]);
    let saved = cache.commit(&store, token).unwrap();
    assert_eq!(saved["revision_after"], prepared["revision_after"]);
    assert_eq!(
        fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
        original
    );
    assert_eq!(
        cache.commit(&store, token).unwrap_err().code,
        "invalid_change_token"
    );
    assert_eq!(
        other_session
            .commit(&store, other["change_token"].as_str().unwrap())
            .unwrap_err()
            .code,
        "stale_revision"
    );
}
#[test]
fn failed_batch_and_invalid_contact_preserve_source() {
    let (dir, store, original) = setup("elements.xgwx");
    let (reference, _) = contact_ref(&store);
    let mut cache = PreparedChanges::default();
    assert!(
        cache
            .prepare(
                &store,
                PrepareRequest {
                    path: "p.xgwx".into(),
                    changes: vec![
                        Change::RenameProgram {
                            object_ref: program_ref(&store),
                            name: "Should not save".into()
                        },
                        Change::SetContactOperand {
                            object_ref: reference,
                            operand: "".into()
                        }
                    ]
                }
            )
            .is_err()
    );
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), original);
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    let change = Change::RenameProgram {
        object_ref: program_ref(&store),
        name: "Duplicate".into(),
    };
    assert_eq!(
        cache
            .prepare(
                &store,
                PrepareRequest {
                    path: "p.xgwx".into(),
                    changes: vec![change.clone(), change]
                }
            )
            .unwrap_err()
            .code,
        "invalid_arguments"
    );
}

#[tokio::test]
async fn stdio_references_common_tools_preview_commit_and_recovery() {
    use rmcp::{
        ServiceExt,
        model::{CallToolRequestParams, ClientConfig},
        transport::TokioChildProcess,
    };
    let (dir, _, original) = module_setup();
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
    let tools = client.list_all_tools().await.unwrap();
    for name in ["prepare_change", "search_project"] {
        assert_eq!(
            tools
                .iter()
                .find(|t| t.name == name)
                .unwrap()
                .annotations
                .as_ref()
                .unwrap()
                .read_only_hint,
            Some(true)
        );
    }
    let inspected = client
        .call_tool(call(
            "inspect_file",
            json!({"path":"p.xgwx","section":"programs"}),
        ))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    let program = &inspected["data"]["programs"]["items"][0]["object_ref"];
    let modules = client
        .call_tool(call("list_modules", json!({"path":"p.xgwx"})))
        .await
        .unwrap()
        .structured_content
        .unwrap();
    let module = &modules["data"]["modules"]["items"][0]["object_ref"];
    let direct = client
        .call_tool(call(
            "rename_program",
            json!({"path":"p.xgwx","object_ref":program,"name":"Conveyor","dry_run":true}),
        ))
        .await
        .unwrap();
    assert_ne!(direct.is_error, Some(true));
    assert_eq!(
        direct.structured_content.unwrap()["changes"][0]["after"],
        "Conveyor"
    );
    let preview = client.call_tool(call("prepare_change",json!({"path":"p.xgwx","changes":[{"type":"rename_program","object_ref":program,"name":"Conveyor"},{"type":"set_module_comment","object_ref":module,"comment":"Drive"}]}))).await.unwrap();
    assert_ne!(preview.is_error, Some(true));
    let preview = preview.structured_content.unwrap();
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), original);
    let saved = client
        .call_tool(call(
            "commit_change",
            json!({"change_token":preview["change_token"]}),
        ))
        .await
        .unwrap();
    assert_ne!(saved.is_error, Some(true));
    assert_eq!(
        saved.structured_content.unwrap()["revision_after"],
        preview["revision_after"]
    );
    let stale = client
        .call_tool(call(
            "prepare_change",
            json!({"path":"p.xgwx","changes":[{"type":"set_module_comment","object_ref":module,"comment":"Stale"}]}),
        ))
        .await
        .unwrap();
    assert_eq!(stale.is_error, Some(true));
    let error = stale.structured_content.unwrap();
    assert_eq!(error["error"]["code"], "stale_revision");
    assert_eq!(error["error"]["recovery"]["action"], "reinspect");
    client.cancel().await.unwrap();
}
