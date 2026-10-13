//! Read-only semantic lookup over decoded project objects, never raw XML/bytes.
use crate::storage::StoreError;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;
use xgwx::{IecRecordKind, LadderProgramData, XgwxDocument};

#[derive(Clone, Copy, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SearchKind {
    Program,
    Variable,
    Contact,
    Coil,
    Instruction,
    Comment,
}
impl SearchKind {
    fn name(self) -> &'static str {
        match self {
            Self::Program => "program",
            Self::Variable => "variable",
            Self::Contact => "contact",
            Self::Coil => "coil",
            Self::Instruction => "instruction",
            Self::Comment => "comment",
        }
    }
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SearchRequest {
    /// Existing .xgwx path within an allowed root.
    pub path: String,
    /// Nonempty literal substring; case-insensitive by default. No regex or fuzzy address matching.
    pub query: String,
    /// Omit to search all supported kinds.
    pub kind: Option<SearchKind>,
    /// Select one document-order program. Global variables remain searchable.
    pub program_index: Option<usize>,
    #[serde(default)]
    pub case_sensitive: bool,
    #[serde(default)]
    pub offset: usize,
    /// Page size 1–1000, applied after matching across the project.
    #[serde(default = "page_size")]
    pub limit: usize,
    /// Optional revision from a previous page. Rejects changed files instead of mixing snapshots.
    pub expected_revision: Option<String>,
}
fn page_size() -> usize {
    100
}

struct Search<'a> {
    request: &'a SearchRequest,
    needle: String,
    items: Vec<Value>,
    total: usize,
    limitations: Vec<Value>,
}
impl Search<'_> {
    fn add(&mut self, kind: SearchKind, mut item: Value) {
        if self.request.kind.is_some_and(|selected| selected != kind) {
            return;
        }
        let mut matched = Vec::new();
        // Search semantic values only, never coordinates, indices, or serialized JSON keys.
        for field in [
            "name",
            "address",
            "description",
            "value",
            "mnemonic",
            "operands",
            "symbols",
            "text",
        ] {
            if contains(&item[field], &self.needle, self.request.case_sensitive) {
                matched.push(field);
            }
        }
        if matched.is_empty() {
            return;
        }
        let index = self.total;
        self.total += 1;
        if index >= self.request.offset && self.items.len() < self.request.limit {
            item["kind"] = json!(kind.name());
            item["matched_fields"] = json!(matched);
            self.items.push(item);
        }
    }
    fn limitation(&mut self, program_index: Option<usize>, section: &str, reason: impl ToString) {
        self.limitations.push(
            json!({"program_index":program_index,"section":section,"reason":reason.to_string()}),
        );
    }
}
fn contains(value: &Value, needle: &str, case_sensitive: bool) -> bool {
    match value {
        Value::String(s) => {
            if case_sensitive {
                s.contains(needle)
            } else {
                s.to_lowercase().contains(needle)
            }
        }
        Value::Array(items) => items.iter().any(|v| contains(v, needle, case_sensitive)),
        Value::Object(fields) => fields.iter().any(|(k, v)| {
            matches!(k.as_str(), "name" | "address" | "description")
                && contains(v, needle, case_sensitive)
        }),
        _ => false,
    }
}
#[derive(Default)]
struct SymbolMatches {
    indices: Vec<usize>,
    has_name: bool,
}
struct SymbolIndex<'a> {
    symbols: &'a [Value],
    names: Vec<Option<String>>,
    matches: HashMap<String, SymbolMatches>,
}
impl<'a> SymbolIndex<'a> {
    fn new(symbols: &'a [Value]) -> Self {
        let mut index = Self {
            symbols,
            names: Vec::with_capacity(symbols.len()),
            matches: HashMap::new(),
        };
        // Each key retains declaration order. A declaration whose name and
        // address normalize to the same key contributes only one candidate.
        for (position, symbol) in symbols.iter().enumerate() {
            let name = symbol["name"].as_str().map(str::to_lowercase);
            if let Some(name) = name.as_ref().filter(|name| !name.is_empty()) {
                let matches = index.matches.entry(name.clone()).or_default();
                matches.indices.push(position);
                matches.has_name = true;
            }
            if let Some(address) = symbol["address"]
                .as_str()
                .filter(|address| !address.is_empty())
                .map(str::to_lowercase)
                && name.as_ref() != Some(&address)
            {
                index
                    .matches
                    .entry(address)
                    .or_default()
                    .indices
                    .push(position);
            }
            index.names.push(name);
        }
        index
    }
}
fn attach_symbols(item: &mut Value, globals: &SymbolIndex<'_>, locals: &SymbolIndex<'_>) {
    let mut symbols = Vec::new();
    for operand in item["operands"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
    {
        let operand = operand.to_lowercase();
        // A local name shadows a global name. Address matches still retain all candidates.
        let local_matches = locals.matches.get(&operand);
        let local_name = local_matches.is_some_and(|matches| matches.has_name);
        for (index, matches, is_global) in [
            (locals, local_matches, false),
            (globals, globals.matches.get(&operand), true),
        ] {
            for &position in matches.into_iter().flat_map(|matches| &matches.indices) {
                if is_global
                    && local_name
                    && index.names[position].as_deref() == Some(operand.as_str())
                {
                    continue;
                }
                let symbol = &index.symbols[position];
                if !symbols.contains(&symbol) {
                    symbols.push(symbol);
                }
            }
        }
    }
    item["symbols"] = json!(symbols);
}
fn context(index: usize, summary: &xgwx::ProgramSummary) -> Value {
    json!({"program_index":index,"program_name":summary.name,"object_id":summary.object_id})
}
fn ladder(
    search: &mut Search<'_>,
    program: &LadderProgramData,
    context: &Value,
    globals: &SymbolIndex<'_>,
    locals: &SymbolIndex<'_>,
) {
    if program.project_type == Some(2) {
        let Some(records) = program.iec_record_frames() else {
            search.limitation(
                context["program_index"].as_u64().map(|i| i as usize),
                "program.records",
                "IEC record layout is not decoded; contacts, coils and comments were not searched.",
            );
            return;
        };
        for record in &records {
            let kind = match record.kind {
                IecRecordKind::Contact(_) => SearchKind::Contact,
                IecRecordKind::Coil(_) => SearchKind::Coil,
                IecRecordKind::Comment => SearchKind::Comment,
                _ => continue,
            };
            let Some(string) = program
                .strings
                .iter()
                .find(|s| s.offset == record.offset + 15)
            else {
                continue;
            };
            let mut item = context.clone();
            item["location"] = json!({"group_index":record.group_index,"row_index":record.row_index,"raw_x":program.data.get(record.offset + 5),"raw_y":u32::from(record.row_index)*4,"record_offset":record.offset,"string_offset":string.offset});
            if kind == SearchKind::Comment {
                item["text"] = json!(string.value);
            } else {
                item["value"] = json!(string.value);
                item["operands"] = json!([string.value]);
                if let IecRecordKind::Contact(code) | IecRecordKind::Coil(code) = record.kind {
                    item["element_code"] = json!(code);
                }
                attach_symbols(&mut item, globals, locals);
            }
            search.add(kind, item);
        }
        if let Some(functions) = program.iec_function_blocks() {
            let links = program.iec_function_operand_links();
            for function in functions {
                let mut item = context.clone();
                item["mnemonic"] = json!(function.name.value);
                item["location"] = json!({"group_index":function.group_index,"row_index":function.row_index,"raw_x":function.raw_x,"raw_y":u32::from(function.row_index)*4,"record_offset":function.record_offset,"string_offset":function.name.offset});
                item["operands"] = json!(
                    links
                        .as_ref()
                        .into_iter()
                        .flatten()
                        .filter(|l| l.target_record_offset == function.record_offset)
                        .filter_map(|l| program
                            .strings
                            .iter()
                            .find(|s| s.offset == l.record_offset + 15)
                            .map(|s| &s.value))
                        .collect::<Vec<_>>()
                );
                attach_symbols(&mut item, globals, locals);
                search.add(SearchKind::Instruction, item);
            }
            if links.is_none() {
                search.limitation(
                    context["program_index"].as_u64().map(|i| i as usize),
                    "program.operand_links",
                    "IEC function operand bindings are not decoded.",
                );
            }
        } else {
            search.limitation(
                context["program_index"].as_u64().map(|i| i as usize),
                "program.functions",
                "IEC function blocks are not decoded.",
            );
        }
    } else {
        for (rung_index, rung) in program.structure.rungs.iter().enumerate() {
            for cell in &rung.cells {
                let kind = if cell.contact.is_some() {
                    SearchKind::Contact
                } else if cell.coil.is_some() {
                    SearchKind::Coil
                } else {
                    continue;
                };
                let mut item = context.clone();
                item["location"] = json!({"rung_index":rung_index,"raw_x":cell.raw_x,"raw_y":cell.raw_y,"string_offset":cell.offset});
                item["value"] = json!(cell.value);
                item["operands"] = if cell.operands.is_empty() && !cell.value.is_empty() {
                    json!([cell.value])
                } else {
                    json!(cell.operands)
                };
                item["element_kind"] = json!(if let Some(contact) = cell.contact {
                    format!("{contact:?}")
                } else {
                    format!("{:?}", cell.coil.unwrap())
                });
                attach_symbols(&mut item, globals, locals);
                search.add(kind, item);
            }
        }
        // The best-effort instruction list omits calls with no operands.
        // Recover catalog-known zero-operand calls from positioned cells.
        for (rung_index, rung) in program.structure.rungs.iter().enumerate() {
            for cell in &rung.cells {
                if cell.contact.is_some()
                    || cell.coil.is_some()
                    || program.instructions.iter().any(|i| i.offset == cell.offset)
                    || !xgwx::ladder_instruction_catalog()
                        .iter()
                        .chain(xgwx::ladder_comparison_catalog())
                        .any(|spec| spec.operand_count == 0 && spec.mnemonic == cell.value)
                {
                    continue;
                }
                let mut item = context.clone();
                item["mnemonic"] = json!(cell.value);
                item["operands"] = json!([]);
                item["location"] = json!({"rung_index":rung_index,"raw_x":cell.raw_x,"raw_y":cell.raw_y,"string_offset":cell.offset});
                search.add(SearchKind::Instruction, item);
            }
        }
        for instruction in &program.instructions {
            let mut item = context.clone();
            item["mnemonic"] = json!(instruction.mnemonic);
            item["operands"] = json!(instruction.operands);
            item["location"] = json!({"string_offset":instruction.offset});
            if let Some((rung_index, cell)) =
                program
                    .structure
                    .rungs
                    .iter()
                    .enumerate()
                    .find_map(|(i, r)| {
                        r.cells
                            .iter()
                            .find(|c| c.offset == instruction.offset)
                            .map(|c| (i, c))
                    })
            {
                item["location"] = json!({"rung_index":rung_index,"raw_x":cell.raw_x,"raw_y":cell.raw_y,"string_offset":instruction.offset});
            }
            attach_symbols(&mut item, globals, locals);
            search.add(SearchKind::Instruction, item);
        }
        for (comment_type, offset, x, y, text) in program
            .structure
            .rung_comments
            .iter()
            .map(|c| ("rung", c.offset, c.raw_x, c.raw_y, &c.text))
            .chain(
                program
                    .structure
                    .output_comments
                    .iter()
                    .map(|c| ("output", c.offset, c.raw_x, c.raw_y, &c.text)),
            )
        {
            let mut item = context.clone();
            item["text"] = json!(text);
            item["comment_type"] = json!(comment_type);
            item["location"] = json!({"raw_x":x,"raw_y":y,"string_offset":offset});
            search.add(SearchKind::Comment, item);
        }
    }
    if !program.structure.unknown_records.is_empty() {
        search.limitation(context["program_index"].as_u64().map(|i| i as usize), "program", format!("{} opaque ladder records were not searched. Decoded results do not establish editability.",program.structure.unknown_records.len()));
    }
}

pub fn search(doc: &XgwxDocument, request: &SearchRequest) -> Result<Value, StoreError> {
    if request.query.trim().is_empty() {
        return Err(StoreError::new(
            "invalid_request",
            "query must contain non-whitespace text",
        ));
    }
    if !(1..=1000).contains(&request.limit) {
        return Err(StoreError::new(
            "invalid_request",
            "limit must be between 1 and 1000",
        ));
    }
    let summaries = doc.program().programs();
    if request.program_index.is_some_and(|i| i >= summaries.len()) {
        return Err(StoreError::new(
            "invalid_request",
            "program_index is absent",
        ));
    }
    let mut search = Search {
        request,
        needle: if request.case_sensitive {
            request.query.clone()
        } else {
            request.query.to_lowercase()
        },
        items: vec![],
        total: 0,
        limitations: vec![],
    };
    let globals = match doc.variable().variables() {
        Ok(variables) => variables.iter().enumerate().map(|(i,v)| json!({"scope":"global","variable_index":i,"name":v.name,"address":v.address,"description":v.description,"data_type":v.data_type})).collect::<Vec<_>>(),
        Err(e) => { search.limitation(None,"symbols.global",e); vec![] }
    };
    for variable in &globals {
        search.add(SearchKind::Variable, variable.clone());
    }
    let mut global_index = None;
    let local_tables = doc.variable().iec_local_symbols();
    let texts = doc.program().text_programs();
    let sfcs = doc.program().sfc_programs();
    for (index, summary) in summaries
        .iter()
        .enumerate()
        .filter(|(i, _)| request.program_index.is_none_or(|selected| selected == *i))
    {
        let context = context(index, summary);
        let mut item = context.clone();
        item["name"] = json!(summary.name);
        item["description"] = json!(summary.comment);
        search.add(SearchKind::Program, item);
        if texts.iter().any(|p| p.program_index == index)
            || sfcs.iter().any(|p| p.program_index == index)
        {
            search.limitation(Some(index),"program", "This search version covers ladder bodies and IEC LD locals; SFC/ST/IL bodies and declarations require inspect_file.");
            continue;
        }
        let mut locals = vec![];
        if let Some(table) = local_tables.get(index) {
            match table {
                Ok(variables) => {
                    for (i, v) in variables.iter().enumerate() {
                        let mut item = context.clone();
                        item["scope"] = json!("local");
                        item["symbol_index"] = json!(i);
                        item["record_offset"] = json!(v.record_offset);
                        item["name"] = json!(v.name);
                        item["address"] = json!(v.address);
                        item["description"] = json!(v.description);
                        item["data_type"] = json!(v.data_type);
                        search.add(SearchKind::Variable, item.clone());
                        locals.push(item);
                    }
                }
                Err(e) => search.limitation(Some(index), "symbols.local", e),
            }
        }
        match doc.program().ladder_program(index) {
            Some(Ok(program)) => ladder(
                &mut search,
                &program,
                &context,
                global_index.get_or_insert_with(|| SymbolIndex::new(&globals)),
                &SymbolIndex::new(&locals),
            ),
            Some(Err(e)) => search.limitation(Some(index), "program", e),
            None => search.limitation(Some(index), "program", "No decoded ladder payload"),
        }
    }
    let end = request.offset.saturating_add(search.items.len());
    Ok(
        json!({"items":search.items,"total":search.total,"offset":request.offset,"limit":request.limit,"next_offset":if end < search.total {Some(end)} else {None},"limitations":search.limitations,"matching":"Literal substring in semantic fields; symbols linked by exact name/address ignoring case, without parsing expressions or normalizing addresses.","location_note":"All indices/offsets are revision-scoped. IEC group/row coordinates are native; XGK rung_index is decoded display order. Search results are discovery, not editability certification; inspect edit sites or export get_ir before preparing an edit."}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    // The previous scan is a compatibility oracle for ordering, exact Unicode
    // lowercase matching, shadowing and value-based deduplication.
    fn reference_attach_symbols(item: &mut Value, globals: &[Value], locals: &[Value]) {
        let operands = item["operands"].as_array().cloned().unwrap_or_default();
        let mut symbols = Vec::new();
        for operand in operands.iter().filter_map(Value::as_str) {
            let local_name = locals.iter().any(|s| {
                s["name"]
                    .as_str()
                    .is_some_and(|n| n.to_lowercase() == operand.to_lowercase())
            });
            for symbol in locals.iter().chain(globals.iter().filter(|s| {
                !local_name
                    || !s["name"]
                        .as_str()
                        .is_some_and(|n| n.to_lowercase() == operand.to_lowercase())
            })) {
                let matches = ["name", "address"].iter().any(|field| {
                    symbol[field].as_str().is_some_and(|s| {
                        !s.is_empty() && s.to_lowercase() == operand.to_lowercase()
                    })
                });
                if matches && !symbols.contains(symbol) {
                    symbols.push(symbol.clone());
                }
            }
        }
        item["symbols"] = json!(symbols);
    }
    #[test]
    fn indexed_linking_matches_scan_for_unicode_collisions_duplicates_and_operand_order() {
        let shared =
            json!({"name":"Shared","address":"M00000","description":"Same value in both tables"});
        let mut locals = vec![
            json!({"scope":"local","name":"PUMP","address":"M00001","description":"Local name"}),
            json!({"scope":"local","name":"Other","address":"pump","description":"Local address"}),
            json!({"scope":"local","name":"pump","address":"PUMP","description":"Name and address collide"}),
            json!({"scope":"local","name":"İ","address":"%MX0","description":"Unicode expansion"}),
            shared.clone(),
        ];
        locals.push(locals[0].clone());
        let mut globals = vec![
            json!({"scope":"global","name":"Pump","address":"pump","description":"Shadowed even at matching address"}),
            json!({"scope":"global","name":"GlobalAlias","address":"PUMP","description":"Global address survives"}),
            json!({"scope":"global","name":"Σ","address":"M00000","description":"Greek sigma"}),
            json!({"scope":"global","name":"ς","address":"M00002","description":"Distinct final sigma"}),
            json!({"scope":"global","name":"Straße","address":"M00003","description":"Not case folding"}),
            json!({"scope":"global","name":"STRASSE","address":"M00004","description":"Distinct spelling"}),
            shared,
            json!({"name":"","address":"","description":"Empty values do not link"}),
            json!({"name":null,"address":null,"description":"Missing values"}),
        ];
        globals.push(globals[1].clone());
        let global_index = SymbolIndex::new(&globals);
        let local_index = SymbolIndex::new(&locals);
        for operands in [
            json!(["pump"]),
            json!(["M00000", "pump", "M00000", "shared"]),
            json!(["pump", "M00000"]),
            json!(["PUMP", "%mx0", "i\u{307}", "İ"]),
            json!(["σ", "Σ", "ς", "straße", "strasse"]),
            json!(["", "M0", "pump + 1", false, null, 1]),
            json!([]),
            Value::Null,
            json!("pump"),
        ] {
            let mut expected = json!({"operands":operands,"kept":{"field":"metadata"}});
            let mut actual = expected.clone();
            reference_attach_symbols(&mut expected, &globals, &locals);
            attach_symbols(&mut actual, &global_index, &local_index);
            assert_eq!(actual, expected);
        }
        let mut item = json!({"operands":["pump"]});
        attach_symbols(&mut item, &global_index, &local_index);
        assert_eq!(
            item["symbols"],
            json!([locals[0], locals[1], locals[2], globals[1]])
        );
    }
    #[test]
    fn local_names_shadow_globals_but_ambiguous_addresses_remain_visible() {
        let globals = vec![
            json!({"scope":"global","name":"Pump","address":"P0001","description":"Wrong pump"}),
            json!({"scope":"global","name":"Alias","address":"P0001","description":"Alias description"}),
        ];
        let locals = vec![
            json!({"scope":"local","name":"PUMP","address":"P0002","description":"Local pump"}),
        ];
        let global_index = SymbolIndex::new(&globals);
        let local_index = SymbolIndex::new(&locals);
        let mut item = json!({"operands":["pump"]});
        attach_symbols(&mut item, &global_index, &local_index);
        assert_eq!(item["symbols"], json!([locals[0]]));
        item = json!({"operands":["P0001"]});
        attach_symbols(&mut item, &global_index, &local_index);
        assert_eq!(item["symbols"], json!(globals));
        item = json!({"operands":["P0001 + 1","P1"]});
        attach_symbols(&mut item, &global_index, &local_index);
        assert_eq!(item["symbols"], json!([]));
    }
}
