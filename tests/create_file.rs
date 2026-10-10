use serde_json::{Value, json};
use std::{
    fs,
    sync::{Arc, Barrier},
    thread,
};
use xgwx::XgwxDocument;
use xgwx_mcp::{
    storage::{EditOperation, FileStore, revision},
    templates::{self, ProjectTemplate},
};
fn op(name: &str, args: Value) -> EditOperation {
    EditOperation {
        name: name.into(),
        args,
    }
}
#[test]
fn templates_are_bundled_native_inputs_and_cpu_discovery_is_guarded() {
    let manifest: Value = serde_json::from_str(include_str!("../templates/manifest.json")).unwrap();
    let listed = templates::list().unwrap();
    assert_eq!(listed["templates"].as_array().unwrap().len(), 3);
    for (template, expected) in [
        (ProjectTemplate::XgkLd, "XGK-CPUSN"),
        (ProjectTemplate::XgiLd, "XGI-CPUE"),
        (ProjectTemplate::XgiSfc, "XGI-CPUE"),
    ] {
        let doc = template.document().unwrap();
        assert_eq!(templates::cpu_model(&doc), Some(expected));
        doc.to_verified_bytes().unwrap();
        let info = template.metadata();
        let entry = manifest["templates"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["id"] == info["id"])
            .unwrap();
        assert_eq!(info["sha256"], entry["sha256"]);
        let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join(entry["source"].as_str().unwrap());
        assert_eq!(fs::read(source).unwrap(), template.bytes());
        let entry = listed["templates"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["id"] == info["id"])
            .unwrap();
        assert!(
            entry["selectable_cpu_models"]
                .as_array()
                .unwrap()
                .contains(&json!(expected))
        );
        for model in entry["selectable_cpu_models"].as_array().unwrap() {
            let mut selected = doc.clone();
            selected.select_cpu(model.as_str().unwrap()).unwrap();
            selected.to_verified_bytes().unwrap();
        }
    }
}
#[test]
fn create_all_templates_preview_atomic_save_then_existing_edit_backup() {
    let dir = tempfile::tempdir().unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    for (name, template) in [
        ("xgk.xgwx", ProjectTemplate::XgkLd),
        ("xgi.xgwx", ProjectTemplate::XgiLd),
        ("sfc.xgwx", ProjectTemplate::XgiSfc),
    ] {
        let count = fs::read_dir(dir.path()).unwrap().count();
        let preview = store.create_file(name, template, None, &[], true).unwrap();
        assert_eq!(preview["created"], false);
        assert!(preview["backup_path"].is_null());
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), count);
        let saved = store.create_file(name, template, None, &[], false).unwrap();
        assert_eq!(saved["created"], true);
        assert_eq!(saved["revision_after"], preview["revision_after"]);
        assert!(saved["revision_before"].is_null());
        assert!(saved["backup_path"].is_null());
        let bytes = fs::read(dir.path().join(name)).unwrap();
        assert_eq!(revision(&bytes), saved["revision_after"].as_str().unwrap());
        let doc = XgwxDocument::parse(&bytes).unwrap();
        assert_eq!(doc.project_info().name.as_deref(), Some("NewProject"));
        doc.to_verified_bytes().unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                fs::metadata(dir.path().join(name))
                    .unwrap()
                    .permissions()
                    .mode()
                    & 0o777,
                0o600
            );
        }
        assert_eq!(
            store
                .create_file(name, template, None, &[], false)
                .unwrap_err()
                .code,
            "file_exists"
        );
        assert_eq!(fs::read(dir.path().join(name)).unwrap(), bytes);
        let edit = store
            .apply(
                name,
                saved["revision_after"].as_str().unwrap(),
                &[op(
                    "update_program",
                    json!({"program_index":0,"patch":{"comment":"New file edited"}}),
                )],
                false,
            )
            .unwrap();
        assert_eq!(
            fs::read(edit["backup_path"].as_str().unwrap()).unwrap(),
            bytes
        );
    }
}
#[test]
fn create_cpu_and_initial_edits_are_verified_and_failures_create_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let edits = [
        op(
            "update_program",
            json!({"program_index":0,"patch":{"name":"Main"}}),
        ),
        op(
            "insert_ladder_instruction",
            json!({"program_index":0,"raw_y":0,"mnemonic":"MOV","operands":["1","D000000"]}),
        ),
    ];
    let saved = store
        .create_file(
            "ready.xgwx",
            ProjectTemplate::XgkLd,
            Some("XGK-CPUH"),
            &edits,
            false,
        )
        .unwrap();
    assert_eq!(saved["cpu_model"], "XGK-CPUH");
    assert_eq!(saved["results"].as_array().unwrap().len(), 2);
    let doc = XgwxDocument::from_path(dir.path().join("ready.xgwx")).unwrap();
    assert_eq!(doc.programs()[0].name.as_deref(), Some("Main"));
    assert!(
        doc.ladder_program(0).unwrap().unwrap().structure.rungs[0]
            .cells
            .iter()
            .any(|c| c.value == "MOV")
    );
    let count = fs::read_dir(dir.path()).unwrap().count();
    for model in ["NOT-A-CPU", "XGI-CPUE"] {
        assert_eq!(
            store
                .create_file("bad.xgwx", ProjectTemplate::XgkLd, Some(model), &[], false)
                .unwrap_err()
                .code,
            "cpu_selection_failed"
        );
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), count);
    }
    let mut bad = edits.to_vec();
    bad.push(op("edit_ladder_cell", json!({"unexpected":true})));
    let error = store
        .create_file("bad.xgwx", ProjectTemplate::XgkLd, None, &bad, false)
        .unwrap_err();
    assert_eq!(error.code, "operation_failed");
    assert_eq!(error.operation_index, Some(2));
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), count);
}
#[test]
fn creation_rejects_boundaries_missing_parents_and_existing_objects() {
    let dir = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    assert_eq!(
        store
            .create_file(
                outside.path().join("out.xgwx").to_str().unwrap(),
                ProjectTemplate::XgkLd,
                None,
                &[],
                false
            )
            .unwrap_err()
            .code,
        "outside_roots"
    );
    assert_eq!(fs::read_dir(outside.path()).unwrap().count(), 0);
    assert_eq!(
        store
            .create_file("missing/new.xgwx", ProjectTemplate::XgkLd, None, &[], false)
            .unwrap_err()
            .code,
        "path_error"
    );
    assert_eq!(
        store
            .create_file("new.txt", ProjectTemplate::XgkLd, None, &[], false)
            .unwrap_err()
            .code,
        "invalid_arguments"
    );
    fs::create_dir(dir.path().join("directory.xgwx")).unwrap();
    assert_eq!(
        store
            .create_file("directory.xgwx", ProjectTemplate::XgkLd, None, &[], false)
            .unwrap_err()
            .code,
        "file_exists"
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        symlink(outside.path(), dir.path().join("escape")).unwrap();
        symlink("absent.xgwx", dir.path().join("dangling.xgwx")).unwrap();
        assert_eq!(
            store
                .create_file("escape/new.xgwx", ProjectTemplate::XgkLd, None, &[], false)
                .unwrap_err()
                .code,
            "outside_roots"
        );
        assert_eq!(
            store
                .create_file("dangling.xgwx", ProjectTemplate::XgkLd, None, &[], false)
                .unwrap_err()
                .code,
            "file_exists"
        );
    }
    let extended = FileStore::new(dir.path().into(), vec![outside.path().into()]).unwrap();
    assert!(
        extended
            .create_file(
                outside.path().join("allowed.xgwx").to_str().unwrap(),
                ProjectTemplate::XgiLd,
                None,
                &[],
                false
            )
            .is_ok()
    );
}
#[test]
fn competing_server_stores_create_exactly_one_complete_file() {
    let dir = tempfile::tempdir().unwrap();
    let barrier = Arc::new(Barrier::new(2));
    let mut workers = vec![];
    for template in [ProjectTemplate::XgkLd, ProjectTemplate::XgiSfc] {
        let store = FileStore::new(dir.path().into(), vec![]).unwrap();
        let barrier = barrier.clone();
        workers.push(thread::spawn(move || {
            barrier.wait();
            store.create_file("same.xgwx", template, None, &[], false)
        }));
    }
    let results = workers
        .into_iter()
        .map(|w| w.join().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(results.iter().filter(|r| r.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter_map(|r| r.as_ref().err())
            .next()
            .unwrap()
            .code,
        "file_exists"
    );
    let bytes = fs::read(dir.path().join("same.xgwx")).unwrap();
    XgwxDocument::parse(&bytes)
        .unwrap()
        .to_verified_bytes()
        .unwrap();
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 2);
}

#[test]
fn native_style_choices_name_path_cpu_and_language_resolve_verified_projects() {
    use xgwx_mcp::{storage::CreateProjectOptions, templates::ProgramLanguage};
    let dir = tempfile::tempdir().unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let listed = templates::list().unwrap();
    for (i, choice) in listed["creation_choices"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        let options = CreateProjectOptions {
            path: format!("chosen-{i}.xgwx"),
            cpu_model: Some(choice["cpu_model"].as_str().unwrap().into()),
            project_name: Some("공장 & <Line 1>".into()),
            program_name: Some("Main".into()),
            program_language: Some(
                serde_json::from_value(choice["program_language"].clone()).unwrap(),
            ),
            dry_run: true,
            ..Default::default()
        };
        let preview = store.create_project(&options).unwrap();
        assert_eq!(preview["template"]["id"], choice["template"]);
        assert_eq!(preview["project_name"], "공장 & <Line 1>");
        assert_eq!(preview["program_name"], "Main");
        assert_eq!(preview["cpu_model"], choice["cpu_model"]);
        assert_eq!(preview["program_language"], choice["program_language"]);
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 0);
    }
    let mut options = CreateProjectOptions {
        path: "named.xgwx".into(),
        cpu_model: Some("xgi-cpue".into()),
        project_name: Some("공장 & <Line 1>".into()),
        program_name: Some("Main".into()),
        program_language: Some(ProgramLanguage::Sfc),
        dry_run: true,
        ..Default::default()
    };
    let preview = store.create_project(&options).unwrap();
    options.dry_run = false;
    let saved = store.create_project(&options).unwrap();
    assert_eq!(preview["revision_after"], saved["revision_after"]);
    let doc = XgwxDocument::from_path(dir.path().join("named.xgwx")).unwrap();
    assert_eq!(doc.project_name(), Some("공장 & <Line 1>"));
    assert_eq!(doc.programs()[0].name.as_deref(), Some("Main"));
    assert_eq!(doc.sfc_programs().len(), 1);
    let count = fs::read_dir(dir.path()).unwrap().count();
    options.path = "invalid.xgwx".into();
    options.cpu_model = Some("XGK-CPUH".into());
    assert_eq!(
        store.create_project(&options).unwrap_err().code,
        "unsupported_combination"
    );
    options.cpu_model = Some("XGI-CPUE".into());
    options.template = Some(ProjectTemplate::XgiLd);
    assert_eq!(
        store.create_project(&options).unwrap_err().code,
        "invalid_arguments"
    );
    options.template = None;
    options.project_name = Some("\n".into());
    assert_eq!(
        store.create_project(&options).unwrap_err().code,
        "invalid_project_name"
    );
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), count);
}
