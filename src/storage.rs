use fs2::FileExt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use xgwx::XgwxDocument;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EditOperation {
    /// Public writer method name from list_operations.
    pub name: String,
    /// Typed operation arguments obtained from describe_operation.
    pub args: Value,
}

/// Creation settings independent of the protocol DTO.
#[derive(Debug, Default)]
pub struct CreateProjectOptions {
    pub path: String,
    pub template: Option<crate::templates::ProjectTemplate>,
    pub cpu_model: Option<String>,
    pub project_name: Option<String>,
    pub program_name: Option<String>,
    pub program_language: Option<crate::templates::ProgramLanguage>,
    pub operations: Vec<EditOperation>,
    pub dry_run: bool,
}

#[derive(Debug, Serialize)]
pub struct StoreError {
    pub code: &'static str,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_path: Option<PathBuf>,
}
impl StoreError {
    pub fn new(code: &'static str, message: impl ToString) -> Self {
        Self {
            code,
            message: message.to_string(),
            operation_index: None,
            backup_path: None,
        }
    }
}
impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}
impl std::error::Error for StoreError {}

#[derive(Clone)]
pub struct FileStore {
    startup: PathBuf,
    roots: Arc<Vec<PathBuf>>,
    locks: Arc<Mutex<HashMap<PathBuf, Arc<Mutex<()>>>>>,
}

pub fn revision(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

impl FileStore {
    pub fn new(startup: PathBuf, roots: Vec<PathBuf>) -> Result<Self, StoreError> {
        let startup = fs::canonicalize(startup).map_err(|e| StoreError::new("root_error", e))?;
        let mut canonical_roots = vec![startup.clone()];
        for root in roots {
            let root = if root.is_absolute() {
                root
            } else {
                startup.join(root)
            };
            let root = fs::canonicalize(root).map_err(|e| StoreError::new("root_error", e))?;
            if !root.is_dir() {
                return Err(StoreError::new(
                    "root_error",
                    "allowed roots must be directories",
                ));
            }
            if !canonical_roots.contains(&root) {
                canonical_roots.push(root);
            }
        }
        Ok(Self {
            startup,
            roots: Arc::new(canonical_roots),
            locks: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    fn resolve(&self, path: &str) -> Result<PathBuf, StoreError> {
        let path = Path::new(path);
        let path = if path.is_absolute() {
            path.to_owned()
        } else {
            self.startup.join(path)
        };
        let canonical = fs::canonicalize(path).map_err(|e| StoreError::new("path_error", e))?;
        if !self.roots.iter().any(|root| canonical.starts_with(root)) {
            return Err(StoreError::new(
                "outside_roots",
                "file resolves outside configured roots",
            ));
        }
        if !canonical.is_file() {
            return Err(StoreError::new(
                "path_error",
                "path must be an existing regular file",
            ));
        }
        Ok(canonical)
    }

    fn resolve_creation(&self, input: &str) -> Result<PathBuf, StoreError> {
        let input = Path::new(input);
        let absolute = if input.is_absolute() {
            input.to_owned()
        } else {
            self.startup.join(input)
        };
        let name = absolute
            .file_name()
            .ok_or_else(|| StoreError::new("path_error", "destination requires a file name"))?;
        if !absolute
            .extension()
            .is_some_and(|s| s.eq_ignore_ascii_case("xgwx"))
        {
            return Err(StoreError::new(
                "invalid_arguments",
                "destination must have an .xgwx extension",
            ));
        }
        let parent = fs::canonicalize(
            absolute
                .parent()
                .ok_or_else(|| StoreError::new("path_error", "destination has no parent"))?,
        )
        .map_err(|e| StoreError::new("path_error", e))?;
        if !parent.is_dir() {
            return Err(StoreError::new(
                "path_error",
                "destination parent must be an existing directory",
            ));
        }
        if !self.roots.iter().any(|r| parent.starts_with(r)) {
            return Err(StoreError::new(
                "outside_roots",
                "destination parent resolves outside configured roots",
            ));
        }
        let path = parent.join(name);
        ensure_absent(&path)?;
        Ok(path)
    }

    pub fn create_file(
        &self,
        input: &str,
        template: crate::templates::ProjectTemplate,
        cpu_model: Option<&str>,
        operations: &[EditOperation],
        dry_run: bool,
    ) -> Result<Value, StoreError> {
        self.create_project(&CreateProjectOptions {
            path: input.into(),
            template: Some(template),
            cpu_model: cpu_model.map(str::to_owned),
            operations: operations.to_vec(),
            dry_run,
            ..Default::default()
        })
    }

    pub fn create_project(&self, options: &CreateProjectOptions) -> Result<Value, StoreError> {
        let input = options.path.as_str();
        let template = crate::templates::select(
            options.template,
            options.cpu_model.as_deref(),
            options.program_language,
        )?;
        let cpu_model = options.cpu_model.as_deref();
        let operations = &options.operations;
        let dry_run = options.dry_run;
        let path = self.resolve_creation(input)?;
        let lock = self
            .locks
            .lock()
            .map_err(|e| StoreError::new("lock_error", e))?
            .entry(path.clone())
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .clone();
        let _guard = lock.lock().map_err(|e| StoreError::new("lock_error", e))?;
        ensure_absent(&path)?;
        let mut doc = template.document()?;
        if let Some(model) = cpu_model {
            doc.select_cpu(model)
                .map_err(|e| StoreError::new("cpu_selection_failed", e))?;
        }
        if let Some(name) = &options.project_name {
            let expected = doc
                .project_name()
                .ok_or_else(|| StoreError::new("template_error", "missing seed project name"))?
                .to_owned();
            doc.rename_project(&expected, name)
                .map_err(|e| StoreError::new("invalid_project_name", e))?;
        }
        if let Some(name) = &options.program_name {
            if name.trim().is_empty() || name.chars().any(char::is_control) {
                return Err(StoreError::new(
                    "invalid_program_name",
                    "program name must be nonblank and contain no control characters",
                ));
            }
            doc.update_program(
                0,
                &xgwx::ProgramPatch {
                    name: Some(name.clone()),
                    ..Default::default()
                },
            )
            .map_err(|e| StoreError::new("invalid_program_name", e))?;
        }
        let mut results = vec![];
        for (index, operation) in operations.iter().enumerate() {
            let result = crate::operations::apply_operation(
                &mut doc,
                &operation.name,
                operation.args.clone(),
            )
            .map_err(|message| StoreError {
                code: "operation_failed",
                message,
                operation_index: Some(index),
                backup_path: None,
            })?;
            results.push(json!({"index":index,"name":operation.name,"result":result}));
        }
        let output = doc
            .to_verified_bytes()
            .map_err(|e| StoreError::new("verification_failed", e))?;
        let validation = crate::inspection::validate(&doc);
        if !dry_run {
            // Prepare and verify before creating even the lock sidecar. The shared
            // path lock coordinates creation with edits from other server processes.
            let _file_lock = lock_file(&path)?;
            if self.resolve_creation(input)? != path {
                return Err(StoreError::new(
                    "path_error",
                    "destination parent changed before creation",
                ));
            }
            create_new_file(&path, &output)?;
        }
        Ok(
            json!({"path":path,"created":!dry_run,"dry_run":dry_run,"revision_before":null,"revision_after":revision(&output),"backup_path":null,"template":template.metadata(),"cpu_model":crate::templates::cpu_model(&doc),"project_name":doc.project_name(),"program_name":doc.programs().first().and_then(|p| p.name.clone()),"program_language":template.language(),"results":results,"validation":validation}),
        )
    }

    pub fn read(&self, path: &str) -> Result<(PathBuf, String, XgwxDocument), StoreError> {
        let path = self.resolve(path)?;
        let bytes = fs::read(&path).map_err(|e| StoreError::new("read_error", e))?;
        let doc = XgwxDocument::parse(&bytes).map_err(|e| StoreError::new("parse_error", e))?;
        Ok((path, revision(&bytes), doc))
    }

    pub fn apply(
        &self,
        input: &str,
        expected: &str,
        operations: &[EditOperation],
        dry_run: bool,
    ) -> Result<Value, StoreError> {
        if operations.is_empty() {
            return Err(StoreError::new(
                "invalid_arguments",
                "operations must not be empty",
            ));
        }
        self.transact(input, expected, dry_run, |_| Ok(operations.to_vec()))
    }

    /// Compile the patch against a fresh, revision-checked document under the
    /// same locks as apply_edits. No client-supplied source mappings are trusted.
    pub fn apply_patch(
        &self,
        input: &str,
        expected: &str,
        program_index: usize,
        patch: &str,
        dry_run: bool,
    ) -> Result<Value, StoreError> {
        self.transact(input, expected, dry_run, |doc| {
            crate::text_ir::Projection::export(doc, expected, program_index)?.lower_patch(patch)
        })
    }

    pub fn apply_property_patch(
        &self,
        input: &str,
        expected: &str,
        section: &str,
        program_index: Option<usize>,
        patch: &str,
        dry_run: bool,
    ) -> Result<Value, StoreError> {
        self.transact(input, expected, dry_run, |doc| {
            crate::property_ir::Projection::export(doc, expected, section, program_index)?
                .lower_patch(patch)
        })
    }

    fn transact(
        &self,
        input: &str,
        expected: &str,
        dry_run: bool,
        compile: impl FnOnce(&XgwxDocument) -> Result<Vec<EditOperation>, StoreError>,
    ) -> Result<Value, StoreError> {
        let path = self.resolve(input)?;
        let lock = self
            .locks
            .lock()
            .map_err(|e| StoreError::new("lock_error", e))?
            .entry(path.clone())
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .clone();
        let _guard = lock.lock().map_err(|e| StoreError::new("lock_error", e))?;
        // Persistent sidecars keep the same lock inode across atomic replacements.
        // Dry runs create neither a sidecar nor a backup.
        let _file_lock = if dry_run {
            None
        } else {
            Some(lock_file(&path)?)
        };
        let original = fs::read(&path).map_err(|e| StoreError::new("read_error", e))?;
        let before = revision(&original);
        if before != expected {
            return Err(StoreError::new(
                "stale_revision",
                "file changed; inspect it again and use its current revision",
            ));
        }
        let mut doc =
            XgwxDocument::parse(&original).map_err(|e| StoreError::new("parse_error", e))?;
        let operations = compile(&doc)?;
        let mut results = Vec::new();
        for (index, operation) in operations.iter().enumerate() {
            let result = crate::operations::apply_operation(
                &mut doc,
                &operation.name,
                operation.args.clone(),
            )
            .map_err(|message| StoreError {
                code: "operation_failed",
                message,
                operation_index: Some(index),
                backup_path: None,
            })?;
            results.push(json!({"index":index,"name":operation.name,"result":result}));
        }
        let output = doc
            .to_verified_bytes()
            .map_err(|e| StoreError::new("verification_failed", e))?;
        let after = revision(&output);
        let changed = output != original;
        let validation = crate::inspection::validate(&doc);
        let backup = if changed && !dry_run {
            Some(replace_file(&path, &original, &output)?)
        } else {
            None
        };
        Ok(json!({
            "path":path,"revision_before":before,"revision_after":after,"changed":changed,
            "dry_run":dry_run,"backup_path":backup,"results":results,"validation":validation
        }))
    }
}

fn ensure_absent(path: &Path) -> Result<(), StoreError> {
    match fs::symlink_metadata(path) {
        Ok(_) => Err(StoreError::new(
            "file_exists",
            "destination already exists; creation never overwrites files, directories or symlinks",
        )),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(StoreError::new("path_error", e)),
    }
}
fn create_new_file(path: &Path, output: &[u8]) -> Result<(), StoreError> {
    create_new_file_with(path, output, |temporary, path| {
        temporary
            .persist_noclobber(path)
            .map(|_| ())
            .map_err(|e| e.error)
    })
}
fn create_new_file_with(
    path: &Path,
    output: &[u8],
    persist: impl FnOnce(tempfile::NamedTempFile, &Path) -> std::io::Result<()>,
) -> Result<(), StoreError> {
    let parent = path
        .parent()
        .ok_or_else(|| StoreError::new("write_error", "destination has no parent"))?;
    let mut temporary = tempfile::Builder::new()
        .prefix(".xgwx-mcp-create-")
        .tempfile_in(parent)
        .map_err(|e| StoreError::new("write_error", e))?;
    temporary
        .write_all(output)
        .map_err(|e| StoreError::new("write_error", e))?;
    temporary
        .as_file()
        .sync_all()
        .map_err(|e| StoreError::new("write_error", e))?;
    persist(temporary, path).map_err(|e| {
        StoreError::new(
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                "file_exists"
            } else {
                "write_error"
            },
            e,
        )
    })?;
    #[cfg(unix)]
    if let Ok(directory) = File::open(parent) {
        let _ = directory.sync_all();
    }
    Ok(())
}

fn lock_file(path: &Path) -> Result<File, StoreError> {
    let mut lock_name = path.as_os_str().to_os_string();
    lock_name.push(".xgwx-mcp.lock");
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true).truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC);
    }
    let lock = options
        .open(PathBuf::from(lock_name))
        .map_err(|e| StoreError::new("lock_error", e))?;
    if !lock
        .metadata()
        .map_err(|e| StoreError::new("lock_error", e))?
        .is_file()
    {
        return Err(StoreError::new(
            "lock_error",
            "lock sidecar must be a regular file",
        ));
    }
    lock.lock_exclusive()
        .map_err(|e| StoreError::new("lock_error", e))?;
    Ok(lock)
}

fn replace_file(path: &Path, original: &[u8], output: &[u8]) -> Result<PathBuf, StoreError> {
    replace_file_with(path, original, output, |temporary, path| {
        temporary
            .persist(path)
            .map(|_| ())
            .map_err(|error| error.error)
    })
}

fn replace_file_with(
    path: &Path,
    original: &[u8],
    output: &[u8],
    persist: impl FnOnce(tempfile::NamedTempFile, &Path) -> std::io::Result<()>,
) -> Result<PathBuf, StoreError> {
    let parent = path
        .parent()
        .ok_or_else(|| StoreError::new("write_error", "file has no parent directory"))?;
    let permissions = fs::metadata(path)
        .map_err(|e| StoreError::new("write_error", e))?
        .permissions();
    let prefix = format!(
        "{}.xgwx-mcp-backup-",
        path.file_name().unwrap().to_string_lossy()
    );
    let mut backup = tempfile::Builder::new()
        .prefix(&prefix)
        .tempfile_in(parent)
        .map_err(|e| StoreError::new("backup_error", e))?;
    backup
        .write_all(original)
        .map_err(|e| StoreError::new("backup_error", e))?;
    backup
        .as_file()
        .set_permissions(permissions.clone())
        .map_err(|e| StoreError::new("backup_error", e))?;
    backup
        .as_file()
        .sync_all()
        .map_err(|e| StoreError::new("backup_error", e))?;
    let (_backup_file, backup_path) = backup
        .keep()
        .map_err(|e| StoreError::new("backup_error", e))?;
    let commit = (|| -> Result<(), StoreError> {
        let mut temporary = tempfile::Builder::new()
            .prefix(".xgwx-mcp-write-")
            .tempfile_in(parent)
            .map_err(|e| StoreError::new("write_error", e))?;
        temporary
            .write_all(output)
            .map_err(|e| StoreError::new("write_error", e))?;
        temporary
            .as_file()
            .set_permissions(permissions)
            .map_err(|e| StoreError::new("write_error", e))?;
        temporary
            .as_file()
            .sync_all()
            .map_err(|e| StoreError::new("write_error", e))?;
        // Other programs do not necessarily honor our advisory lock.
        if fs::read(path).map_err(|e| StoreError::new("read_error", e))? != original {
            return Err(StoreError::new(
                "stale_revision",
                "file changed before replacement",
            ));
        }
        persist(temporary, path).map_err(|e| StoreError::new("write_error", e))?;
        Ok(())
    })();
    commit.map_err(|mut error| {
        error.backup_path = Some(backup_path.clone());
        error
    })?;
    // File replacement has committed. A directory sync failure must not be
    // reported as a failed transaction, which could invite an unsafe retry.
    #[cfg(unix)]
    if let Ok(directory) = File::open(parent) {
        let _ = directory.sync_all();
    }
    Ok(backup_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn creation_write_failure_and_destination_race_never_publish_partial_data() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("new.xgwx");
        let err = create_new_file_with(&path, b"complete candidate", |_, _| {
            Err(std::io::Error::other("injected write failure"))
        })
        .unwrap_err();
        assert_eq!(err.code, "write_error");
        assert!(!path.exists());
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 0);
        let err = create_new_file_with(&path, b"complete candidate", |temporary, path| {
            fs::write(path, b"external winner")?;
            temporary
                .persist_noclobber(path)
                .map(|_| ())
                .map_err(|e| e.error)
        })
        .unwrap_err();
        assert_eq!(err.code, "file_exists");
        assert_eq!(fs::read(&path).unwrap(), b"external winner");
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    }

    fn setup() -> (tempfile::TempDir, FileStore, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("project.xgwx");
        fs::copy(
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../libxgwx/fixtures/elements.xgwx"
            ),
            &path,
        )
        .unwrap();
        let store = FileStore::new(dir.path().into(), vec![dir.path().into()]).unwrap();
        (dir, store, path)
    }
    fn comment(text: &str) -> EditOperation {
        EditOperation {
            name: "update_program".into(),
            args: json!({"program_index":0,"patch":{"comment":text}}),
        }
    }

    #[test]
    fn dry_run_backup_and_stale_revision() {
        let (dir, store, path) = setup();
        let original = fs::read(&path).unwrap();
        let rev = revision(&original);
        let preview = store
            .apply("project.xgwx", &rev, &[comment("MCP test")], true)
            .unwrap();
        assert!(preview["changed"].as_bool().unwrap());
        assert_eq!(fs::read(&path).unwrap(), original);
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
        let saved = store
            .apply("project.xgwx", &rev, &[comment("MCP test")], false)
            .unwrap();
        assert_eq!(
            fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
            original
        );
        assert_ne!(fs::read(&path).unwrap(), original);
        assert_eq!(
            store
                .apply("project.xgwx", &rev, &[comment("stale")], false)
                .unwrap_err()
                .code,
            "stale_revision"
        );
        let current = saved["revision_after"].as_str().unwrap();
        let count = fs::read_dir(dir.path()).unwrap().count();
        let noop = store
            .apply("project.xgwx", current, &[comment("MCP test")], false)
            .unwrap();
        assert_eq!(noop["changed"], false);
        assert_eq!(noop["backup_path"], Value::Null);
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), count);
    }

    #[test]
    fn failed_batch_rolls_back_and_invalid_arguments_fail() {
        let (_dir, store, path) = setup();
        let original = fs::read(&path).unwrap();
        let bad = EditOperation {
            name: "update_program".into(),
            args: json!({"program_index":usize::MAX,"patch":{"comment":"bad"}}),
        };
        let error = store
            .apply(
                "project.xgwx",
                &revision(&original),
                &[comment("partial"), bad],
                false,
            )
            .unwrap_err();
        assert_eq!(error.operation_index, Some(1));
        assert_eq!(fs::read(&path).unwrap(), original);
        let bad = EditOperation {
            name: "update_program".into(),
            args: json!({"unexpected":true}),
        };
        assert!(
            store
                .apply("project.xgwx", &revision(&original), &[bad], false)
                .is_err()
        );
    }

    #[test]
    fn root_boundaries_and_invalid_container() {
        let (dir, store, _path) = setup();
        let outside = tempfile::tempdir().unwrap();
        fs::write(outside.path().join("bad.xgwx"), b"bad").unwrap();
        assert_eq!(
            store
                .read(outside.path().join("bad.xgwx").to_str().unwrap())
                .unwrap_err()
                .code,
            "outside_roots"
        );
        fs::write(dir.path().join("bad.xgwx"), b"bad").unwrap();
        assert_eq!(store.read("bad.xgwx").unwrap_err().code, "parse_error");
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(
                outside.path().join("bad.xgwx"),
                dir.path().join("escape.xgwx"),
            )
            .unwrap();
            assert_eq!(store.read("escape.xgwx").unwrap_err().code, "outside_roots");
        }
    }

    #[test]
    fn competing_requests_only_one_revision_wins() {
        let (dir, _store, path) = setup();
        let rev = revision(&fs::read(&path).unwrap());
        let mut handles = Vec::new();
        for n in 0..2 {
            let store = FileStore::new(dir.path().into(), vec![]).unwrap();
            let rev = rev.clone();
            handles.push(std::thread::spawn(move || {
                store.apply(
                    "project.xgwx",
                    &rev,
                    &[comment(&format!("winner {n}"))],
                    false,
                )
            }));
        }
        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        assert_eq!(results.iter().filter(|r| r.is_ok()).count(), 1);
        assert_eq!(
            results
                .iter()
                .filter(|r| r.as_ref().is_err_and(|e| e.code == "stale_revision"))
                .count(),
            1
        );
    }

    #[test]
    fn replacement_preserves_permissions_and_write_failure_keeps_source() {
        let (dir, store, path) = setup();
        let original = fs::read(&path).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o640)).unwrap();
        }
        let result = store
            .apply(
                "project.xgwx",
                &revision(&original),
                &[comment("permissions")],
                false,
            )
            .unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                fs::metadata(&path).unwrap().permissions().mode() & 0o777,
                0o640
            );
            assert_eq!(
                fs::metadata(result["backup_path"].as_str().unwrap())
                    .unwrap()
                    .permissions()
                    .mode()
                    & 0o777,
                0o640
            );
        }
        // A non-directory parent prevents backup creation deterministically,
        // including when tests run as root.
        let blocked = dir.path().join("parent");
        fs::write(&blocked, b"block").unwrap();
        assert_eq!(
            replace_file(&blocked.join("project.xgwx"), &original, b"changed")
                .unwrap_err()
                .code,
            "write_error"
        );
        let before = fs::read(&path).unwrap();
        let error = replace_file(&path, b"different source", b"changed").unwrap_err();
        assert_eq!(error.code, "stale_revision");
        assert_eq!(fs::read(&path).unwrap(), before);
    }

    #[cfg(unix)]
    #[test]
    fn backup_creation_failure_leaves_original_untouched() {
        let (dir, _store, path) = setup();
        let original = fs::read(&path).unwrap();
        let long_path = dir.path().join(format!("{}.xgwx", "p".repeat(242)));
        fs::write(&long_path, &original).unwrap();
        // The source name fits NAME_MAX, but appending a unique backup suffix
        // cannot. This tests the real backup failure path without relying on UID.
        let error = replace_file(&long_path, &original, b"changed").unwrap_err();
        assert_eq!(error.code, "backup_error");
        assert_eq!(fs::read(&long_path).unwrap(), original);
    }

    #[test]
    fn failed_candidate_persist_retains_backup_and_reports_its_path() {
        let (dir, _store, path) = setup();
        let original = fs::read(&path).unwrap();
        let error = replace_file_with(&path, &original, b"changed", |_candidate, _target| {
            Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "injected atomic replacement failure",
            ))
        })
        .unwrap_err();
        assert_eq!(error.code, "write_error");
        assert_eq!(fs::read(error.backup_path.unwrap()).unwrap(), original);
        assert_eq!(fs::read(&path).unwrap(), original);
        assert_eq!(
            fs::read_dir(dir.path()).unwrap().count(),
            2,
            "candidate temporary file must be cleaned up"
        );
    }
}
