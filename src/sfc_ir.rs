//! Guarded SFC projection; snapshots come from the fresh source, never the client.

use crate::{
    operation_types::{
        SfcDeclaration, SfcEntity, SfcEntityPatch, SfcPosition, SfcRow, SfcVariable,
    },
    storage::{EditOperation, StoreError},
    text_ir::{apply_unified_patch, lex},
};
use serde_json::json;
use std::collections::BTreeMap;
use xgwx::{SfcProgram, XgwxDocument};
fn number(n: Option<u32>) -> String {
    n.map_or("none".into(), |n| n.to_string())
}
fn quoted(s: &str) -> String {
    serde_json::to_string(s).unwrap()
}
fn unsupported(s: impl ToString) -> StoreError {
    StoreError::new("unsupported_ir_edit", s)
}
fn invalid(s: impl ToString) -> StoreError {
    StoreError::new("invalid_patch", s)
}
fn optional(s: &Option<String>) -> String {
    s.as_ref().map_or("none".into(), |s| quoted(s))
}
fn optional_value(s: &str) -> Result<Option<String>, StoreError> {
    if s == "none" {
        Ok(None)
    } else {
        serde_json::from_str(s).map(Some).map_err(invalid)
    }
}
#[derive(Clone)]
enum Source {
    Row { block: usize, row: SfcRow },
    Variable { index: usize },
    Field { patch: SfcEntityPatch },
}
#[derive(PartialEq, Eq)]
struct Node {
    scope: Vec<Vec<String>>,
    fields: BTreeMap<String, String>,
}
struct Parsed {
    frozen: Vec<Vec<String>>,
    nodes: BTreeMap<String, Node>,
    order: Vec<String>,
}
pub struct Projection {
    pub text: String,
    index: usize,
    program: SfcProgram,
    sources: BTreeMap<String, Source>,
    document: XgwxDocument,
}
impl Projection {
    pub fn export(
        doc: &XgwxDocument,
        revision: &str,
        index: usize,
        program: SfcProgram,
    ) -> Result<Self, StoreError> {
        let summary = doc
            .program()
            .programs()
            .into_iter()
            .nth(index)
            .ok_or_else(|| invalid("program is absent"))?;
        let mut text = format!(
            "xgwx-ir 1 revision {}\nprogram @p{index} {} dialect SFC object {} {{\n",
            quoted(revision),
            quoted(summary.name.as_deref().unwrap_or("")),
            quoted(summary.object_id.as_deref().unwrap_or(""))
        );
        text.push_str(&crate::context_ir::export(doc, Some((index, "SFC"))));
        let mut sources = BTreeMap::new();
        if let Some(reason) = &program.variables_error {
            text.push_str(&format!(
                "  declarations error {} readonly\n",
                quoted(reason)
            ));
        }
        for (i, v) in program.variables.iter().enumerate() {
            let id = format!("@p{index}.s{i}");
            let declaration =
                json!(crate::operation_types::SfcVariable::from(v.clone()).declaration);
            text.push_str(&format!(
                "  sfc_variable {id} name {} type {} system {} declaration {} description {}\n",
                quoted(&v.name),
                quoted(&v.data_type),
                v.system,
                quoted(&declaration.to_string()),
                quoted(&v.description)
            ));
            sources.insert(id, Source::Variable { index: i });
        }
        for b in &program.blocks {
            let bi = b.block_index;
            text.push_str(&format!("  chart @p{index}.b{bi} {} rows {} columns {} main {} language_type {} language {} {{\n",quoted(&b.name),b.rows,b.columns,b.main,quoted(&format!("{:?}",b.language_type)),quoted(&format!("{:?}",b.language))));
            for e in &b.entities {
                let ei = e.entity_index;
                text.push_str(&format!("    entity @p{index}.b{bi}.e{ei} row {} column {} type {} signature {} readonly\n",quoted(&format!("{:?}",e.row)),quoted(&format!("{:?}",e.column)),quoted(&format!("{:?}",e.type_code)),quoted(&crate::storage::revision(serde_json::to_string(&SfcEntity::from(e.clone())).unwrap().as_bytes()))));
                if b.editable_rows.is_none() {
                    for (group, properties) in &e.properties {
                        for (key, value) in properties {
                            let field = match (e.type_code, group.as_str(), key.as_str()) {
                                (Some(0), "EntityStep", "Comment") => Some("comment"),
                                (Some(1), "EntityStep", "Title")
                                    if properties.get("PropertyProgram").map(String::as_str)
                                        == Some("0") =>
                                {
                                    Some("condition")
                                }
                                _ => None,
                            };
                            if let (Some(field), Some(ty), Some(row), Some(column)) =
                                (field, e.type_code, e.row, e.column)
                            {
                                let id = format!("@p{index}.b{bi}.e{ei}.{field}");
                                text.push_str(&format!("    entity_field {id} row {row} column {column} type {ty} field {} value {}\n",quoted(field),quoted(value)));
                                sources.insert(
                                    id,
                                    Source::Field {
                                        patch: SfcEntityPatch {
                                            program_index: index,
                                            block_index: bi,
                                            entity_index: ei,
                                            expected_type: ty,
                                            expected_row: row,
                                            expected_column: column,
                                            field: field.into(),
                                            expected_value: value.clone(),
                                            replacement: value.clone(),
                                        },
                                    },
                                );
                            } else {
                                text.push_str(&format!(
                                    "    property entity {} group {} key {} value {} readonly\n",
                                    quoted(&format!("@p{index}.b{bi}.e{ei}")),
                                    quoted(group),
                                    quoted(key),
                                    quoted(value)
                                ));
                            }
                        }
                    }
                }
            }
            if let Some(rows) = &b.editable_rows {
                for (ri, row) in rows.iter().enumerate() {
                    let row = SfcRow::from(row.clone());
                    let id = format!("@p{index}.b{bi}.r{ri}");
                    text.push_str(&format!("    sfc_row {id} kind {} title {} comment {} initial {} action {} qualifier {} time {} action_code {} transition_code {} position {} branch_end {}\n",quoted(&row.kind),quoted(&row.title),quoted(&row.comment),row.initial,optional(&row.action),optional(&row.action_qualifier),optional(&row.action_time),optional(&row.action_code),optional(&row.transition_code),row.position.as_ref().map_or("none".into(),|p|quoted(&format!("row={},column={}",p.row,p.column))),number(row.branch_end)));
                    sources.insert(id, Source::Row { block: bi, row });
                }
            } else {
                text.push_str("    sequence unsupported readonly\n");
            }
            text.push_str("  }\n");
        }
        text.push_str("}\n");
        Ok(Self {
            text,
            index,
            program,
            sources,
            document: doc.clone(),
        })
    }
    pub fn lower_patch(&self, patch: &str) -> Result<Vec<EditOperation>, StoreError> {
        let before = parse(&self.text)?;
        let after = parse(&apply_unified_patch(&self.text, patch)?)?;
        if before.frozen != after.frozen {
            return Err(unsupported(
                "SFC chart headers, entity fingerprints and opaque properties are immutable",
            ));
        }
        let nonrows = |parsed: &Parsed| {
            parsed
                .order
                .iter()
                .filter(|id| {
                    !matches!(
                        parsed.nodes[*id].fields["statement"].as_str(),
                        "sfc_row" | "sfc_variable"
                    )
                })
                .cloned()
                .collect::<Vec<_>>()
        };
        if nonrows(&before) != nonrows(&after) {
            return Err(unsupported("entity identities and order remain fixed"));
        }
        let mut rows = BTreeMap::<usize, Vec<SfcRow>>::new();
        for block in &self.program.blocks {
            let Some(original) = &block.editable_rows else {
                continue;
            };
            let chart_id = format!("@p{}.b{}", self.index, block.block_index);
            let ids = |parsed: &Parsed| {
                parsed
                    .order
                    .iter()
                    .filter(|id| {
                        let n = &parsed.nodes[*id];
                        n.fields["statement"] == "sfc_row"
                            && n.scope.len() == 2
                            && n.scope[1].get(1) == Some(&chart_id)
                    })
                    .cloned()
                    .collect::<Vec<_>>()
            };
            let new_ids = ids(&after);
            let mut target = vec![];
            for id in &new_ids {
                let node = &after.nodes[id];
                let template = match self.sources.get(id) {
                    Some(Source::Row { block: bi, row }) if *bi == block.block_index => {
                        let old = &before.nodes[id];
                        if old.scope != node.scope {
                            return Err(unsupported(
                                "retained SFC rows cannot move between charts",
                            ));
                        }
                        Some(row)
                    }
                    Some(_) => {
                        return Err(unsupported(
                            "SFC source identity cannot change object family or chart",
                        ));
                    }
                    None => {
                        if !id.starts_with("@new-") || id.len() <= 5 {
                            return Err(invalid(
                                "new SFC rows require fresh @new-<name> identities",
                            ));
                        }
                        None
                    }
                };
                target.push(decode_row(node, template)?);
            }
            let expected = original
                .iter()
                .cloned()
                .map(SfcRow::from)
                .collect::<Vec<_>>();
            if target != expected {
                rows.insert(block.block_index, target);
            }
        }
        for id in &after.order {
            let node = &after.nodes[id];
            if node.fields["statement"] == "sfc_row" {
                if node.scope.len() != 2
                    || !self.program.blocks.iter().any(|b| {
                        node.scope[1].get(1)
                            == Some(&format!("@p{}.b{}", self.index, b.block_index))
                            && b.editable_rows.is_some()
                    })
                {
                    return Err(unsupported(
                        "SFC rows must belong directly to an exported supported chart",
                    ));
                }
                if let Some(old) = before.nodes.get(id)
                    && old.scope != node.scope
                {
                    return Err(unsupported("retained SFC rows cannot move between charts"));
                }
            }
        }
        let mut edits = vec![];
        let mut variable_candidate = None;
        let mut variables = self
            .program
            .variables
            .iter()
            .cloned()
            .map(SfcVariable::from)
            .collect::<Vec<_>>();
        for id in &before.order {
            let old = &before.nodes[id];
            if matches!(self.sources[id], Source::Row { .. }) {
                continue;
            }
            let Some(new) = after.nodes.get(id) else {
                let Source::Variable { index } = &self.sources[id] else {
                    return Err(unsupported("entity identities are immutable"));
                };
                let v = &variables[*index];
                if v.system {
                    return Err(unsupported("system declarations are read-only"));
                }
                edits.push(EditOperation{name:"edit_sfc_variable".into(),args:json!({"patch":{"program_index":self.index,"expected_variables":variables,"name":v.name,"data_type":v.data_type,"description":v.description,"remove":true,"update":false,"declaration":v.declaration}})});
                continue;
            };
            if old == new {
                continue;
            }
            if old.scope != new.scope || old.fields.keys().ne(new.fields.keys()) {
                return Err(unsupported(
                    "SFC object scope and field names are immutable",
                ));
            }
            let allowed = match &self.sources[id] {
                Source::Row { .. } => &[
                    "title",
                    "comment",
                    "initial",
                    "action",
                    "qualifier",
                    "time",
                    "action_code",
                    "transition_code",
                ][..],
                Source::Variable { .. } => &["description", "type", "declaration"][..],
                Source::Field { .. } => &["value"][..],
            };
            for (key, value) in &old.fields {
                if new.fields[key] != *value && !allowed.contains(&key.as_str()) {
                    return Err(unsupported(format!("{id}: {key} is read-only")));
                }
            }
            let string =
                |key: &str| serde_json::from_str::<String>(&new.fields[key]).map_err(invalid);
            match &self.sources[id] {
                Source::Row { .. } => unreachable!(),
                Source::Variable { index } => {
                    let v = &variables[*index];
                    let description = string("description")?;
                    let data_type = string("type")?;
                    let declaration: Option<SfcDeclaration> =
                        serde_json::from_str(&string("declaration")?).map_err(invalid)?;
                    if description == v.description
                        && data_type == v.data_type
                        && declaration == v.declaration
                    {
                        continue;
                    }
                    if v.system {
                        return Err(unsupported("system declarations are read-only"));
                    }
                    edits.push(EditOperation{name:"edit_sfc_variable".into(),args:json!({"patch":{"program_index":self.index,"expected_variables":variables,"name":v.name,"data_type":data_type,"description":description,"remove":false,"update":true,"declaration":declaration}})});
                    // Re-decode native canonical values before the next expected snapshot.
                    // Literal spelling/default metadata can normalize during serialization.
                    let candidate = variable_candidate.get_or_insert_with(|| self.document.clone());
                    crate::operations::apply_operation(
                        candidate,
                        "edit_sfc_variable",
                        edits.last().unwrap().args.clone(),
                    )
                    .map_err(|e| StoreError {
                        operation_index: Some(edits.len() - 1),
                        operation_name: Some("edit_sfc_variable".into()),
                        ..StoreError::new("operation_failed", e)
                    })?;
                    variables = candidate
                        .variable()
                        .sfc_variables(self.index)
                        .map_err(|e| StoreError::new("operation_failed", e))?
                        .into_iter()
                        .map(SfcVariable::from)
                        .collect();
                }
                Source::Field { patch } => {
                    let mut patch = patch.clone();
                    patch.replacement = string("value")?;
                    if patch.replacement == patch.expected_value {
                        continue;
                    }
                    edits.push(EditOperation {
                        name: "edit_sfc_entity".into(),
                        args: json!({"patch":patch}),
                    });
                }
            }
        }
        for id in &after.order {
            let node = &after.nodes[id];
            if node.fields["statement"] != "sfc_variable" {
                continue;
            }
            if node.scope.len() != 1 {
                return Err(unsupported(
                    "SFC declarations belong directly to the program",
                ));
            }
            if before.nodes.contains_key(id) {
                continue;
            }
            if !id.starts_with("@new-") || id.len() <= 5 {
                return Err(invalid(
                    "new declarations require fresh @new-<name> identities",
                ));
            }
            let f = &node.fields;
            let keys = [
                "statement",
                "name",
                "type",
                "system",
                "declaration",
                "description",
            ];
            if f.len() != keys.len()
                || keys.iter().any(|k| !f.contains_key(*k))
                || f["system"] != "false"
            {
                return Err(invalid(
                    "new declarations require exactly the exported fields and system false",
                ));
            }
            let string = |key: &str| serde_json::from_str::<String>(&f[key]).map_err(invalid);
            let declaration: Option<SfcDeclaration> =
                serde_json::from_str(&string("declaration")?).map_err(invalid)?;
            edits.push(EditOperation{name:"edit_sfc_variable".into(),args:json!({"patch":{"program_index":self.index,"expected_variables":variables,"name":string("name")?,"data_type":string("type")?,"description":string("description")?,"remove":false,"update":false,"declaration":declaration}})});
        }
        let variable_ids = |p: &Parsed| {
            p.order
                .iter()
                .filter(|id| p.nodes[*id].fields["statement"] == "sfc_variable")
                .cloned()
                .collect::<Vec<_>>()
        };
        let old_variables = variable_ids(&before);
        let new_variables = variable_ids(&after);
        let retained = |ids: &Vec<String>| {
            ids.iter()
                .filter(|id| before.nodes.contains_key(*id) && after.nodes.contains_key(*id))
                .cloned()
                .collect::<Vec<_>>()
        };
        if retained(&old_variables) != retained(&new_variables) {
            return Err(unsupported(
                "native declaration order is derived; declarations cannot be reordered",
            ));
        }
        if old_variables != new_variables && (edits.len() != 1 || !rows.is_empty()) {
            return Err(unsupported(
                "declaration insertion/deletion must be the sole mutation; re-export afterward",
            ));
        }
        if !rows.is_empty() && (rows.len() != 1 || !edits.is_empty()) {
            return Err(unsupported(
                "SFC row changes must target one chart and be the only writer operation; commit and re-export before dependent edits",
            ));
        }
        for (bi, rows) in rows {
            let b = self
                .program
                .blocks
                .iter()
                .find(|b| b.block_index == bi)
                .unwrap();
            edits.push(EditOperation{name:"replace_sfc_sequence".into(),args:json!({"patch":{"program_index":self.index,"block_index":bi,"expected_entities":b.entities.iter().cloned().map(SfcEntity::from).collect::<Vec<_>>(),"expected_rows":b.editable_rows.as_ref().unwrap().iter().cloned().map(SfcRow::from).collect::<Vec<_>>(),"rows":rows}})});
        }
        Ok(edits)
    }
}
fn decode_row(node: &Node, _template: Option<&SfcRow>) -> Result<SfcRow, StoreError> {
    let f = &node.fields;
    let keys = [
        "statement",
        "kind",
        "title",
        "comment",
        "initial",
        "action",
        "qualifier",
        "time",
        "action_code",
        "transition_code",
        "position",
        "branch_end",
    ];
    if f.len() != keys.len() || keys.iter().any(|k| !f.contains_key(*k)) {
        return Err(invalid("SFC rows require exactly the exported row fields"));
    }
    let string = |key: &str| serde_json::from_str::<String>(&f[key]).map_err(invalid);
    let kind = string("kind")?;
    let position = match optional_value(&f["position"])? {
        None => None,
        Some(value) => {
            let (row, column) = value
                .strip_prefix("row=")
                .and_then(|v| v.split_once(",column="))
                .ok_or_else(|| invalid("position must be none or a quoted row=N,column=M"))?;
            Some(SfcPosition {
                row: row.parse().map_err(invalid)?,
                column: column.parse().map_err(invalid)?,
            })
        }
    };
    let branch_end = if f["branch_end"] == "none" {
        None
    } else {
        Some(f["branch_end"].parse().map_err(invalid)?)
    };
    Ok(SfcRow {
        kind,
        title: string("title")?,
        comment: string("comment")?,
        initial: f["initial"]
            .parse()
            .map_err(|_| invalid("initial must be true or false"))?,
        action: optional_value(&f["action"])?,
        action_qualifier: optional_value(&f["qualifier"])?,
        action_time: optional_value(&f["time"])?,
        action_code: optional_value(&f["action_code"])?,
        transition_code: optional_value(&f["transition_code"])?,
        position,
        branch_end,
    })
}

fn parse(text: &str) -> Result<Parsed, StoreError> {
    let mut parsed = Parsed {
        frozen: vec![],
        nodes: BTreeMap::new(),
        order: vec![],
    };
    let mut scope = vec![];
    let mut ids = std::collections::BTreeSet::new();
    for line in text.lines() {
        let t = lex(line)?;
        if t.is_empty() {
            continue;
        }
        if t.last().map(String::as_str) == Some("{") {
            if (t[0] == "program" && scope.is_empty()) || (t[0] == "chart" && scope.len() == 1) {
                scope.push(t.clone());
            } else {
                return Err(invalid("SFC chart blocks must belong to one program"));
            }
            parsed.frozen.push(t);
            continue;
        }
        if t == ["}"] {
            scope
                .pop()
                .ok_or_else(|| invalid("unmatched closing brace"))?;
            parsed.frozen.push(t);
            continue;
        }
        if matches!(t[0].as_str(), "sfc_row" | "sfc_variable" | "entity_field") {
            if t.len() < 4 || !t[1].starts_with('@') || !t.len().is_multiple_of(2) {
                return Err(invalid(
                    "SFC statement requires @identity and key/value pairs",
                ));
            }
            if !ids.insert(t[1].clone()) {
                return Err(invalid("duplicate SFC identity"));
            }
            let mut fields = BTreeMap::from([("statement".into(), t[0].clone())]);
            for pair in t[2..].chunks_exact(2) {
                if fields.insert(pair[0].clone(), pair[1].clone()).is_some() {
                    return Err(invalid("duplicate SFC field"));
                }
            }
            parsed.order.push(t[1].clone());
            parsed.nodes.insert(
                t[1].clone(),
                Node {
                    scope: scope.clone(),
                    fields,
                },
            );
        } else {
            parsed.frozen.push(t);
        }
    }
    if !scope.is_empty() {
        return Err(invalid("unclosed SFC block"));
    }
    Ok(parsed)
}
