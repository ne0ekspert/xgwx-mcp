//! Local declaration lifecycle uses captured identities and native primitive writers.
use crate::storage::{EditOperation, StoreError};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
#[derive(Default)]
pub(crate) struct Lifecycle {
    pub sources: BTreeMap<String, (usize, usize, String)>,
    pub tables: BTreeSet<usize>,
}
struct Document {
    frozen: Vec<String>,
    order: Vec<String>,
    blocks: BTreeMap<String, (Option<usize>, String)>,
}
fn invalid(message: impl ToString) -> StoreError {
    StoreError::new("unsupported_ir_edit", message)
}
fn parse(text: &str) -> Result<Document, StoreError> {
    let mut result = Document {
        frozen: vec![],
        order: vec![],
        blocks: BTreeMap::new(),
    };
    let mut current: Option<(String, Option<usize>, String)> = None;
    for line in text.lines() {
        if let Some((_, _, body)) = &mut current {
            if line.starts_with("variable ") {
                return Err(invalid("variable blocks cannot nest"));
            }
            body.push_str(line);
            body.push('\n');
            if line == "}" {
                let (id, program, body) = current.take().unwrap();
                if result.blocks.insert(id.clone(), (program, body)).is_some() {
                    return Err(invalid("duplicate variable identity"));
                }
                result.order.push(id);
            }
        } else if let Some(header) = line
            .strip_prefix("variable ")
            .and_then(|l| l.strip_suffix(" {"))
        {
            let (id, program) = if let Some((id, p)) = header.split_once(" program ") {
                (id, Some(p.parse::<usize>().map_err(invalid)?))
            } else {
                (header, None)
            };
            if !id.starts_with('@') || id.contains(char::is_whitespace) {
                return Err(invalid("invalid variable identity"));
            }
            current = Some((id.into(), program, format!("{line}\n")));
        } else {
            result.frozen.push(line.into());
        }
    }
    if current.is_some() {
        return Err(invalid("unclosed variable block"));
    }
    Ok(result)
}
impl Lifecycle {
    pub fn lower(
        &self,
        original: &str,
        changed: &str,
    ) -> Result<Option<Vec<EditOperation>>, StoreError> {
        let before = parse(original)?;
        let after = parse(changed)?;
        if before.frozen != after.frozen {
            return Err(invalid("variable table/context statements are readonly"));
        }
        if before.order == after.order {
            return Ok(None);
        }
        for id in &after.order {
            if let Some(old) = before.blocks.get(id) {
                if after.blocks[id] != *old {
                    return Err(invalid(
                        "variable creation/deletion must be the sole mutation; retain all other blocks unchanged",
                    ));
                }
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
        let retained = after
            .order
            .iter()
            .filter(|id| before.blocks.contains_key(*id))
            .cloned()
            .collect::<Vec<_>>();
        let expected = before
            .order
            .iter()
            .filter(|id| after.blocks.contains_key(*id))
            .cloned()
            .collect::<Vec<_>>();
        if retained != expected {
            return Err(invalid(
                "native variable order is derived; reordering is unsupported",
            ));
        }
        let op = if added.len() == 1 && removed.is_empty() {
            let id = added[0];
            if !id.starts_with("@new-") || id.len() <= 5 {
                return Err(invalid(
                    "new variables require fresh @new-<name> identities",
                ));
            }
            let (program, body) = &after.blocks[id];
            let program=program.filter(|p|self.tables.contains(p)).ok_or_else(||invalid("new variables require an exported writable local table: variable @new-name program N; global creation is unsupported"))?;
            let mut fields = BTreeMap::new();
            for line in body.lines().skip(1).filter(|l| *l != "}") {
                let (key, value) = line
                    .strip_prefix("  edit ")
                    .and_then(|l| l.split_once(" = "))
                    .ok_or_else(|| {
                        invalid("new locals require edit name/data_type/description only")
                    })?;
                let value: String = serde_json::from_str(value).map_err(invalid)?;
                if fields.insert(key, value).is_some() {
                    return Err(invalid("duplicate variable field"));
                }
            }
            if fields.len() != 3
                || ["name", "data_type", "description"]
                    .iter()
                    .any(|key| !fields.contains_key(key))
            {
                return Err(invalid(
                    "new locals require exactly name, data_type and description; assign addresses after creation and re-export",
                ));
            }
            EditOperation {
                name: "insert_iec_local_symbol".into(),
                args: json!({"program_index":program,"name":fields["name"],"data_type":fields["data_type"],"description":fields["description"]}),
            }
        } else if removed.len() == 1 && added.is_empty() {
            let (program,index,name)=self.sources.get(removed[0]).ok_or_else(||invalid("global variable deletion is unsupported; SFC/ST/IL declarations use their existing workflows"))?;
            EditOperation {
                name: "delete_iec_local_symbol".into(),
                args: json!({"program_index":program,"symbol_index":index,"expected_name":name}),
            }
        } else {
            return Err(invalid(
                "create or delete one local declaration per patch; variable order is derived",
            ));
        };
        Ok(Some(vec![op]))
    }
}
