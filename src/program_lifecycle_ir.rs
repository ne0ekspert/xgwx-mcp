//! Lifecycle lowering over the complete, revision-scoped program metadata list.
use crate::storage::{EditOperation, StoreError};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub(crate) struct Lifecycle {
    pub revision: String,
    pub complete: bool,
    pub identities: Vec<Option<String>>,
}
struct Document {
    prefix: String,
    order: Vec<String>,
    blocks: BTreeMap<String, String>,
}
fn invalid(message: impl ToString) -> StoreError {
    StoreError::new("unsupported_ir_edit", message)
}
fn parse(text: &str) -> Result<Document, StoreError> {
    let mut result = Document {
        prefix: String::new(),
        order: vec![],
        blocks: BTreeMap::new(),
    };
    let mut current: Option<(String, String)> = None;
    for line in text.lines() {
        if let Some((id, body)) = &mut current {
            if line.starts_with("program_metadata ") {
                return Err(invalid("program blocks cannot nest"));
            }
            body.push_str(line);
            body.push('\n');
            if line == "}" {
                let (id, body) = current.take().unwrap();
                if result.blocks.insert(id.clone(), body).is_some() {
                    return Err(invalid("duplicate program identity"));
                }
                result.order.push(id);
            } else if line.contains('{') && !line.contains(" = ") {
                return Err(invalid(format!("{id}: unsupported block statement")));
            }
        } else if let Some(id) = line
            .strip_prefix("program_metadata ")
            .and_then(|l| l.strip_suffix(" {"))
        {
            if !id.starts_with('@') || id.contains(char::is_whitespace) {
                return Err(invalid("invalid program identity"));
            }
            current = Some((id.into(), format!("{line}\n")));
        } else {
            if !result.order.is_empty() {
                return Err(invalid("only program blocks may follow lifecycle context"));
            }
            result.prefix.push_str(line);
            result.prefix.push('\n');
        }
    }
    if current.is_some() {
        return Err(invalid("unclosed program block"));
    }
    Ok(result)
}
// Revision/authoring-scoped UUIDv8 identities keep preview and save reproducible.
fn guid(seed: &str, domain: &str) -> String {
    let hash = Sha256::digest(format!("xgwx-ir-program:{domain}:{seed}").as_bytes());
    let mut bytes: [u8; 16] = hash[..16].try_into().unwrap();
    bytes[6] = (bytes[6] & 15) | 0x80;
    bytes[8] = (bytes[8] & 63) | 0x80;
    let h = bytes.iter().map(|b| format!("{b:02x}")).collect::<String>();
    format!(
        "{}-{}-{}-{}-{}",
        &h[..8],
        &h[8..12],
        &h[12..16],
        &h[16..20],
        &h[20..]
    )
}
impl Lifecycle {
    pub fn lower(
        &self,
        original: &str,
        changed: &str,
    ) -> Result<Option<Vec<EditOperation>>, StoreError> {
        let before = parse(original)?;
        let after = parse(changed)?;
        if before.prefix != after.prefix {
            return Err(invalid("program lifecycle context is readonly"));
        }
        if before.order == after.order {
            return Ok(None);
        }
        if !self.complete {
            return Err(invalid(
                "lifecycle edits require the complete programs export; omit program_index",
            ));
        }
        for id in &after.order {
            if let Some(old) = before.blocks.get(id) {
                if after.blocks[id] != *old {
                    return Err(invalid(
                        "lifecycle changes must be the sole mutation; preserve retained blocks and re-export afterward",
                    ));
                }
            } else if !id.starts_with("@new-") || id.len() <= 5 {
                return Err(invalid("new programs require fresh @new-<name> identities"));
            }
        }
        let added = after
            .order
            .iter()
            .filter(|id| !before.blocks.contains_key(*id))
            .collect::<Vec<_>>();
        let removed = before
            .order
            .iter()
            .filter(|id| !after.blocks.contains_key(*id))
            .collect::<Vec<_>>();
        let identity = |index: usize| -> Result<&str, StoreError> {
            self.identities
                .get(index)
                .and_then(|s| s.as_deref())
                .filter(|s| !s.is_empty())
                .ok_or_else(|| {
                    invalid("program identity is absent; lifecycle operation is unsupported")
                })
        };
        let op = if added.len() == 1 && removed.is_empty() {
            let id = added[0];
            if after.order.last() != Some(id)
                || after.order[..after.order.len() - 1] != before.order
            {
                return Err(invalid(
                    "new programs append to the list; create, re-export, then reorder",
                ));
            }
            let mut fields = BTreeMap::new();
            for line in after.blocks[id].lines().skip(1).filter(|l| *l != "}") {
                let (key, value) = line
                    .strip_prefix("  edit ")
                    .and_then(|l| l.split_once(" = "))
                    .ok_or_else(|| {
                        invalid("new program blocks require edit name and edit language only")
                    })?;
                let value: String = serde_json::from_str(value).map_err(invalid)?;
                if fields.insert(key, value).is_some() {
                    return Err(invalid("duplicate new-program field"));
                }
            }
            if fields.len() != 2 || !fields.contains_key("name") || !fields.contains_key("language")
            {
                return Err(invalid(
                    "new programs require exactly name and language; native IDs are generated internally",
                ));
            }
            let seed = json!([self.revision, id, fields["name"], fields["language"]]).to_string();
            EditOperation {
                name: "create_program".into(),
                args: json!({"patch":{"name":fields["name"],"language":fields["language"],"object_id":guid(&seed,"program"),"symbol_id":guid(&seed,"symbols")}}),
            }
        } else if removed.len() == 1 && added.is_empty() {
            let id = removed[0];
            let expected = before
                .order
                .iter()
                .filter(|v| *v != id)
                .cloned()
                .collect::<Vec<_>>();
            if expected != after.order {
                return Err(invalid(
                    "delete and reorder require separate lifecycle patches",
                ));
            }
            let index = before.order.iter().position(|v| v == id).unwrap();
            EditOperation {
                name: "delete_program".into(),
                args: json!({"program_index":index,"expected_object_id":identity(index)?}),
            }
        } else if removed.is_empty() && added.is_empty() {
            let first = before
                .order
                .iter()
                .zip(&after.order)
                .position(|(a, b)| a != b)
                .unwrap();
            let candidates = [
                (
                    first,
                    after
                        .order
                        .iter()
                        .position(|id| id == &before.order[first])
                        .unwrap(),
                ),
                (
                    before
                        .order
                        .iter()
                        .position(|id| id == &after.order[first])
                        .unwrap(),
                    first,
                ),
            ];
            let found = candidates.into_iter().find(|&(from, to)| {
                let mut order = before.order.clone();
                let id = order.remove(from);
                order.insert(to, id);
                order == after.order
            });
            let (from,to) = found.ok_or_else(||invalid("reordering must express one native program move; re-export before additional moves"))?;
            EditOperation {
                name: "move_program".into(),
                args: json!({"from":from,"to":to,"expected_object_id":identity(from)?,"expected_target_id":identity(to)?}),
            }
        } else {
            return Err(invalid(
                "create, delete or move exactly one program per lifecycle patch",
            ));
        };
        Ok(Some(vec![op]))
    }
}
