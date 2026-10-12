//! Common workflows lower to the existing guarded writers. References carry no authority.
use crate::storage::{EditOperation, FileStore, StoreError, revision};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    path::Path,
    time::{Duration, Instant},
};
use xgwx::{IecRecordKind, XgwxDocument};

fn invalid(message: impl ToString) -> StoreError {
    StoreError::new("invalid_object_ref", message)
}
#[derive(Clone, Debug, PartialEq, Eq)]
enum Target {
    Program(usize),
    Module(usize),
    Contact(usize, usize),
}
impl Target {
    fn encode(&self) -> String {
        match self {
            Self::Program(i) => format!("program/{i}"),
            Self::Module(i) => format!("module/{i}"),
            Self::Contact(i, o) => format!("contact/{i}/{o}"),
        }
    }
    fn parse(input: &str) -> Result<Self, StoreError> {
        let parts = input.split('/').collect::<Vec<_>>();
        let number = |s: &str| {
            s.parse::<usize>()
                .map_err(|_| invalid("Malformed object selector"))
        };
        match parts.as_slice() {
            ["program", i] => Ok(Self::Program(number(i)?)),
            ["module", i] => Ok(Self::Module(number(i)?)),
            ["contact", i, o] => Ok(Self::Contact(number(i)?, number(o)?)),
            _ => Err(invalid("Unknown object selector")),
        }
    }
}
fn snapshot(doc: &XgwxDocument, target: &Target) -> Result<Value, StoreError> {
    match *target {
        Target::Program(index) => {
            let p = doc
                .program()
                .programs()
                .into_iter()
                .nth(index)
                .ok_or_else(|| invalid("Program no longer exists"))?;
            Ok(json!({"program_index":index,"name":p.name,"object_id":p.object_id,"kind":p.kind}))
        }
        Target::Module(index) => {
            let m = doc
                .module()
                .summaries()
                .into_iter()
                .nth(index)
                .ok_or_else(|| invalid("Module no longer exists"))?;
            Ok(
                json!({"module_index":index,"base":m.base,"slot":m.slot,"id":m.id,"sub_type":m.sub_type,"name":m.name,"comment":m.comment,"details":m.details}),
            )
        }
        Target::Contact(index, offset) => {
            let summary = doc
                .program()
                .programs()
                .into_iter()
                .nth(index)
                .ok_or_else(|| invalid("Program no longer exists"))?;
            let p = doc
                .program()
                .ladder_program(index)
                .ok_or_else(|| invalid("Program is not ladder"))?
                .map_err(invalid)?;
            let mut result = json!({"program_index":index,"program_name":summary.name,"object_id":summary.object_id,"string_offset":offset,"project_type":p.project_type});
            if p.project_type == Some(2) {
                let records = p.iec_record_frames().ok_or_else(|| {
                    StoreError::new("unsupported_target", "IEC contact layout is not decoded")
                })?;
                let mut matching = records.iter().filter(|r| {
                    r.offset + 15 == offset && matches!(r.kind, IecRecordKind::Contact(_))
                });
                let r = matching
                    .next()
                    .ok_or_else(|| invalid("Reference does not identify an IEC contact"))?;
                if matching.next().is_some() {
                    return Err(invalid("Ambiguous IEC contact"));
                }
                let string = p
                    .strings
                    .iter()
                    .find(|s| s.offset == offset)
                    .ok_or_else(|| invalid("Contact operand is not decoded"))?;
                result["operand"] = json!(string.value);
                result["location"] = json!({"group_index":r.group_index,"row_index":r.row_index,"raw_x":p.data.get(r.offset + 5),"record_offset":r.offset});
                if let IecRecordKind::Contact(code) = r.kind {
                    result["contact_kind"] = json!(code);
                }
            } else if p.project_type == Some(1) {
                let mut matching = p
                    .structure
                    .rungs
                    .iter()
                    .flat_map(|r| &r.cells)
                    .filter(|c| c.offset == offset && c.contact.is_some());
                let c = matching
                    .next()
                    .ok_or_else(|| invalid("Reference does not identify an XGK contact"))?;
                if matching.next().is_some() {
                    return Err(invalid("Ambiguous XGK contact"));
                }
                // Marker-only inverse/pulse contacts have no editable operand string.
                let string = p
                    .strings
                    .iter()
                    .find(|s| s.offset == offset && s.value == c.value)
                    .ok_or_else(|| {
                        StoreError::new(
                            "unsupported_target",
                            "Contact has no editable operand string",
                        )
                    })?;
                result["operand"] = json!(string.value);
                result["location"] = json!({"raw_x":c.raw_x,"raw_y":c.raw_y});
                result["contact_kind"] = json!(format!("{:?}", c.contact.unwrap()));
            } else {
                return Err(StoreError::new(
                    "unsupported_target",
                    "Contact format is not supported",
                ));
            }
            Ok(result)
        }
    }
}
fn make_ref(path: &Path, rev: &str, target: &Target, snapshot: &Value) -> String {
    let fingerprint =
        revision(&serde_json::to_vec(&json!([path, rev, target.encode(), snapshot])).unwrap());
    format!("xgwx-ref/1:{rev}:{}:{fingerprint}", target.encode())
}
fn resolve(
    doc: &XgwxDocument,
    path: &Path,
    rev: &str,
    reference: &str,
) -> Result<(Target, Value), StoreError> {
    let parts = reference.split(':').collect::<Vec<_>>();
    if parts.len() != 4 || parts[0] != "xgwx-ref/1" {
        return Err(invalid(
            "Use an object_ref returned by search_project or inspection",
        ));
    }
    if parts[1] != rev {
        return Err(StoreError::new(
            "stale_revision",
            "Object reference belongs to an older file revision; inspect the target again",
        ));
    }
    let target = Target::parse(parts[2])?;
    let observed = snapshot(doc, &target)?;
    if make_ref(path, rev, &target, &observed) != reference {
        return Err(invalid(
            "Reference does not match this file and observed object",
        ));
    }
    Ok((target, observed))
}
/// Add references to supported objects in existing inspection/search shapes.
pub fn annotate(doc: &XgwxDocument, path: &Path, rev: &str, data: &mut Value) {
    fn walk(doc: &XgwxDocument, path: &Path, rev: &str, value: &mut Value) {
        match value {
            Value::Array(values) => {
                for v in values {
                    walk(doc, path, rev, v);
                }
            }
            Value::Object(_) => {
                // Recurse before adding refs, so attached symbol records cannot become contact refs.
                for v in value.as_object_mut().unwrap().values_mut() {
                    walk(doc, path, rev, v);
                }
                let target = if value["kind"] == "contact" {
                    value["program_index"]
                        .as_u64()
                        .zip(value["location"]["string_offset"].as_u64())
                        .map(|(i, o)| Target::Contact(i as usize, o as usize))
                } else if value.get("program_index").is_some()
                    && value.get("name").is_some()
                    && value.get("object_id").is_some()
                    && value.get("scope").is_none()
                {
                    value["program_index"]
                        .as_u64()
                        .map(|i| Target::Program(i as usize))
                } else if value.get("input_filter_raw").is_some()
                    && value.get("module_index").is_some()
                    && value.get("base").is_some()
                    && value.get("slot").is_some()
                {
                    value["module_index"]
                        .as_u64()
                        .map(|i| Target::Module(i as usize))
                } else {
                    None
                };
                if let Some(target) = target
                    && let Ok(observed) = snapshot(doc, &target)
                {
                    value["object_ref"] = json!(make_ref(path, rev, &target, &observed));
                }
            }
            _ => {}
        }
    }
    walk(doc, path, rev, data);
}

#[derive(Clone, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Change {
    RenameProgram { object_ref: String, name: String },
    SetModuleComment { object_ref: String, comment: String },
    SetContactOperand { object_ref: String, operand: String },
}
fn lower(
    doc: &XgwxDocument,
    path: &Path,
    rev: &str,
    change: &Change,
) -> Result<(EditOperation, Value), StoreError> {
    let reference = match change {
        Change::RenameProgram { object_ref, .. }
        | Change::SetModuleComment { object_ref, .. }
        | Change::SetContactOperand { object_ref, .. } => object_ref,
    };
    let (target, before) = resolve(doc, path, rev, reference)?;
    let (name, args, field, after) = match (change, target) {
        (Change::RenameProgram { name, .. }, Target::Program(index)) => (
            "update_program",
            json!({"program_index":index,"patch":{"name":name}}),
            "name",
            json!(name),
        ),
        (Change::SetModuleComment { comment, .. }, Target::Module(index)) => {
            let m = doc
                .module()
                .summaries()
                .into_iter()
                .nth(index)
                .ok_or_else(|| invalid("Module no longer exists"))?;
            let (base, slot) = m.base.zip(m.slot).ok_or_else(|| {
                StoreError::new("unsupported_target", "Module has no physical base/slot")
            })?;
            // Selection must resolve uniquely under the same public library guards.
            doc.module()
                .at(base, slot)
                .map_err(|e| StoreError::new("unsupported_target", e))?;
            (
                "update_module",
                json!({"base":base,"slot":slot,"patch":{"comment":comment}}),
                "comment",
                json!(comment),
            )
        }
        (Change::SetContactOperand { operand, .. }, Target::Contact(index, offset)) => (
            "update_ladder_cell_text",
            json!({"program_index":index,"offset":offset,"expected":before["operand"],"replacement":operand}),
            "operand",
            json!(operand),
        ),
        _ => {
            return Err(StoreError::new(
                "reference_kind_mismatch",
                "Use a reference for the requested object kind",
            ));
        }
    };
    let label = match change {
        Change::RenameProgram { .. } => format!("Program {}", before["name"]),
        Change::SetModuleComment { .. } => {
            format!("Module at base {}, slot {}", before["base"], before["slot"])
        }
        Change::SetContactOperand { .. } => format!(
            "Contact in program {} at {}",
            before["program_name"], before["location"]
        ),
    };
    let summary = json!({"summary":format!("{label}: {field} {} -> {after}",before[field]),"object_ref":reference,"target":before,"field":field,"before":before[field],"after":after});
    Ok((
        EditOperation {
            name: name.into(),
            args,
        },
        summary,
    ))
}
fn lower_all(
    doc: &XgwxDocument,
    path: &Path,
    rev: &str,
    changes: &[Change],
) -> Result<(Vec<EditOperation>, Vec<Value>), StoreError> {
    if changes.is_empty() || changes.len() > 128 {
        return Err(StoreError::new(
            "invalid_arguments",
            "changes must contain 1–128 common edits",
        ));
    }
    let mut operations = Vec::new();
    let mut summaries = Vec::new();
    for change in changes {
        let (op, summary) = lower(doc, path, rev, change)?;
        if summaries
            .iter()
            .any(|s: &Value| s["object_ref"] == summary["object_ref"])
        {
            return Err(StoreError::new(
                "invalid_arguments",
                "Each object may appear only once in a change batch",
            ));
        }
        operations.push(op);
        summaries.push(summary);
    }
    Ok((operations, summaries))
}
pub fn apply(
    store: &FileStore,
    input: &str,
    change: Change,
    dry_run: bool,
) -> Result<Value, StoreError> {
    let (path, rev, doc) = store.read(input)?;
    let (ops, summaries) = lower_all(&doc, &path, &rev, &[change])?;
    let mut result = store.apply(
        path.to_str().ok_or_else(|| invalid("Path is not UTF-8"))?,
        &rev,
        &ops,
        dry_run,
    )?;
    result["changes"] = json!(summaries);
    Ok(result)
}
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PrepareRequest {
    pub path: String,
    pub changes: Vec<Change>,
}
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CommitRequest {
    pub change_token: String,
}

struct Prepared {
    created: Instant,
    path: String,
    revision: String,
    candidate: String,
    operations: Vec<EditOperation>,
    summaries: Vec<Value>,
}
pub struct PreparedChanges {
    entries: HashMap<String, Prepared>,
    sequence: u64,
    session: String,
}
impl Default for PreparedChanges {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
            sequence: 0,
            session: revision(
                format!("{}:{:?}", std::process::id(), std::time::SystemTime::now()).as_bytes(),
            ),
        }
    }
}
const TTL: Duration = Duration::from_secs(1800);
impl PreparedChanges {
    pub fn prepare(
        &mut self,
        store: &FileStore,
        request: PrepareRequest,
    ) -> Result<Value, StoreError> {
        self.entries.retain(|_, p| p.created.elapsed() < TTL);
        if self.entries.len() >= 128 {
            return Err(StoreError::new(
                "preview_capacity",
                "128 active previews already exist; commit one or wait for expiry",
            ));
        }
        let (path, rev, doc) = store.read(&request.path)?;
        let (ops, summaries) = lower_all(&doc, &path, &rev, &request.changes)?;
        let path = path
            .to_str()
            .ok_or_else(|| invalid("Path is not UTF-8"))?
            .to_owned();
        let mut result = store.apply(&path, &rev, &ops, true)?;
        let candidate = result["revision_after"].as_str().unwrap().to_owned();
        self.sequence += 1;
        let token = revision(
            &serde_json::to_vec(&json!([self.session, path, rev, candidate, self.sequence]))
                .unwrap(),
        );
        self.entries.insert(
            token.clone(),
            Prepared {
                created: Instant::now(),
                path,
                revision: rev,
                candidate,
                operations: ops,
                summaries: summaries.clone(),
            },
        );
        result["changes"] = json!(summaries);
        result["change_token"] = json!(token);
        result["expires_in_seconds"] = json!(TTL.as_secs());
        result["native_acceptance"] = json!("unverified");
        Ok(result)
    }
    pub fn commit(&mut self, store: &FileStore, token: &str) -> Result<Value, StoreError> {
        let prepared = self.entries.get(token).ok_or_else(|| {
            StoreError::new(
                "invalid_change_token",
                "Preview is unknown, already committed, or belongs to another server session",
            )
        })?;
        if prepared.created.elapsed() >= TTL {
            self.entries.remove(token);
            return Err(StoreError::new(
                "expired_change_token",
                "Preview expired; prepare the change again",
            ));
        }
        let mut result = store.apply_prepared(
            &prepared.path,
            &prepared.revision,
            &prepared.operations,
            &prepared.candidate,
        )?;
        result["changes"] = json!(prepared.summaries);
        result["native_acceptance"] = json!("unverified");
        self.entries.remove(token);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn candidate_drift_and_expiration_fail_without_replacing_source() {
        let dir = tempfile::tempdir().unwrap();
        let doc = xgwx::create_project("XGK-CPUH", "LD").unwrap();
        let bytes = doc.to_verified_bytes().unwrap();
        let path = dir.path().join("p.xgwx");
        std::fs::write(&path, &bytes).unwrap();
        let store = FileStore::new(dir.path().into(), vec![]).unwrap();
        let target = Target::Program(0);
        let reference = make_ref(
            &path,
            &revision(&bytes),
            &target,
            &snapshot(&doc, &target).unwrap(),
        );
        let mut cache = PreparedChanges::default();
        let prepared = cache
            .prepare(
                &store,
                PrepareRequest {
                    path: "p.xgwx".into(),
                    changes: vec![Change::RenameProgram {
                        object_ref: reference,
                        name: "Reviewed".into(),
                    }],
                },
            )
            .unwrap();
        let token = prepared["change_token"].as_str().unwrap();
        cache.entries.get_mut(token).unwrap().candidate = "incorrect-candidate".into();
        assert_eq!(
            cache.commit(&store, token).unwrap_err().code,
            "candidate_changed"
        );
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
        assert!(
            !std::fs::read_dir(dir.path()).unwrap().any(|e| e
                .unwrap()
                .file_name()
                .to_string_lossy()
                .contains("backup"))
        );
        cache.entries.get_mut(token).unwrap().created =
            Instant::now() - TTL - Duration::from_secs(1);
        assert_eq!(
            cache.commit(&store, token).unwrap_err().code,
            "expired_change_token"
        );
        assert!(!cache.entries.contains_key(token));
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
    }
}
