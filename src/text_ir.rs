//! Revision-scoped, positional ladder projection. The original document remains
//! the source of truth; this IR is lowered only through public guarded writers.
use crate::storage::{EditOperation, StoreError};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use xgwx::{IecRecordKind, XgwxDocument};

fn error(message: impl ToString) -> StoreError {
    StoreError::new("invalid_patch", message)
}
fn unsupported(message: impl ToString) -> StoreError {
    StoreError::new("unsupported_ir_edit", message)
}
fn quote(s: &str) -> String {
    serde_json::to_string(s).unwrap()
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Position {
    Xgk { x: u8, y: u32 },
    Iec { group: usize, row: u16, x: u8 },
}
impl Position {
    fn render(&self) -> String {
        match self {
            Self::Xgk { x, y } => format!("at(x={x},y={y})"),
            Self::Iec { group, row, x } => format!("at(group={group},row={row},x={x})"),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
enum Body {
    Instruction {
        family: String,
        mnemonic: String,
        operands: Vec<String>,
    },
    Function {
        name: String,
        signature: String,
    },
    Argument {
        target: String,
        pin: u8,
        pin_row: u16,
        pin_x: u8,
        direction: String,
        type_mask: u32,
        array: bool,
        value: String,
    },
    Wire {
        end: Position,
        kind: String,
    },
    Element {
        family: String,
        kind: String,
        operand: String,
    },
    Comment {
        kind: String,
        text: String,
    },
    Symbol {
        name: String,
        address: String,
        type_name: String,
        description: String,
    },
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Node {
    id: String,
    scope: Vec<Vec<String>>,
    position: Option<Position>,
    body: Body,
}
#[derive(Clone, Debug)]
enum Source {
    Instruction {
        offset: usize,
        expected: String,
        arity: usize,
    },
    Function {
        offset: usize,
    },
    Argument {
        offset: usize,
    },
    Wire {
        offset: usize,
    },
    Element {
        offset: usize,
    },
    Comment {
        offset: usize,
    },
    Global {
        index: usize,
    },
    Local {
        index: usize,
    },
}
#[derive(Debug)]
struct Parsed {
    nodes: BTreeMap<String, Node>,
    frozen: Vec<Vec<String>>,
}

fn element_kind(
    contact: Option<xgwx::LadderContact>,
    coil: Option<xgwx::LadderCoil>,
) -> Option<(&'static str, &'static str)> {
    use xgwx::{LadderCoil as O, LadderContact as C};
    if let Some(c) = contact {
        Some((
            "contact",
            match c {
                C::NormallyOpen => "NO",
                C::NormallyClosed => "NC",
                C::Inverse => "INVERSE",
                C::RisingPulse => "RISING",
                C::FallingPulse => "FALLING",
                C::AddressedRisingPulse => "ADDR_RISING",
                C::AddressedFallingPulse => "ADDR_FALLING",
                C::AddressedRisingPulseNot => "ADDR_NEGATED_RISING",
                C::AddressedFallingPulseNot => "ADDR_NEGATED_FALLING",
            },
        ))
    } else {
        coil.map(|c| {
            (
                "coil",
                match c {
                    O::Output => "OUT",
                    O::Inverse => "INVERSE",
                    O::Set => "SET",
                    O::Reset => "RESET",
                    O::RisingPulse => "RISING",
                    O::FallingPulse => "FALLING",
                },
            )
        })
    }
}
fn iec_kind(code: u8) -> Option<(&'static str, &'static str)> {
    Some(match code {
        6 => ("contact", "NO"),
        7 => ("contact", "NC"),
        8 => ("contact", "RISING"),
        9 => ("contact", "FALLING"),
        10 => ("contact", "NEGATED_RISING"),
        11 => ("contact", "NEGATED_FALLING"),
        14 => ("coil", "OUT"),
        15 => ("coil", "INVERSE"),
        16 => ("coil", "SET"),
        17 => ("coil", "RESET"),
        18 => ("coil", "RISING"),
        19 => ("coil", "FALLING"),
        _ => return None,
    })
}

pub struct Projection {
    pub text: String,
    pub dialect: &'static str,
    program_index: usize,
    sources: BTreeMap<String, Source>,
    iec_rows: Vec<xgwx::IecRowFrame>,
    isolated: BTreeSet<String>,
    sfc: Option<crate::sfc_ir::Projection>,
}
impl Projection {
    pub fn export(doc: &XgwxDocument, revision: &str, index: usize) -> Result<Self, StoreError> {
        let summary = doc
            .programs()
            .into_iter()
            .nth(index)
            .ok_or_else(|| error("program_index is absent"))?;
        if let Some(program) = doc
            .sfc_programs()
            .into_iter()
            .find(|p| p.program_index == index)
        {
            let sfc = crate::sfc_ir::Projection::export(doc, revision, index, program)?;
            return Ok(Self {
                text: sfc.text.clone(),
                dialect: "SFC",
                program_index: index,
                sources: BTreeMap::new(),
                iec_rows: vec![],
                isolated: BTreeSet::new(),
                sfc: Some(sfc),
            });
        }
        let program = doc
            .ladder_program(index)
            .ok_or_else(|| error("program_index is absent"))?
            .map_err(unsupported)?;
        // ProjectType 2 is IEC. A failed exact decoder must never fall back to XGK.
        let iec = program.project_type == Some(2);
        let dialect = if iec { "IEC" } else { "XGK" };
        if !iec && program.project_type != Some(1) {
            return Err(unsupported(
                "text IR v1 supports ProjectType=1 XGK LD and ProjectType=2 IEC LD only",
            ));
        }
        let mut text = format!("xgwx-ir 1 revision {}\n", quote(revision));
        let mut sources = BTreeMap::new();
        let mut iec_rows = vec![];
        let mut isolated = BTreeSet::new();
        let globals = doc.variables().map_err(unsupported)?;
        for (i, symbol) in globals.iter().enumerate() {
            let id = format!("@s{i}");
            text.push_str(&format!(
                "symbol {id} {} address {} type {} description {}\n",
                quote(symbol.name.as_deref().unwrap_or("")),
                quote(symbol.address.as_deref().unwrap_or("")),
                quote(symbol.data_type.as_deref().unwrap_or("")),
                quote(symbol.description.as_deref().unwrap_or(""))
            ));
            sources.insert(id, Source::Global { index: i });
        }
        text.push_str(&format!(
            "program @p{index} {} dialect {dialect} object {} {{\n",
            quote(summary.name.as_deref().unwrap_or("")),
            quote(summary.object_id.as_deref().unwrap_or(""))
        ));
        let locals = doc
            .iec_local_symbols()
            .into_iter()
            .nth(index)
            .unwrap_or(Ok(vec![]))
            .map_err(unsupported)?;
        for (i, symbol) in locals.iter().enumerate() {
            let id = format!("@p{index}.s{i}");
            text.push_str(&format!(
                "  symbol {id} {} address {} type {} description {}\n",
                quote(&symbol.name),
                quote(symbol.address.as_deref().unwrap_or("")),
                quote(symbol.data_type.as_deref().unwrap_or("")),
                quote(symbol.description.as_deref().unwrap_or(""))
            ));
            sources.insert(id, Source::Local { index: i });
        }
        if iec {
            iec_rows = program
                .iec_row_frames()
                .ok_or_else(|| unsupported("IEC row layout is not recognized"))?;
            if iec_rows.is_empty() {
                text.push_str(&format!(
                    "  canvas @p{index}.blank at(group=0,row=0) readonly\n"
                ));
            }
            for row in &iec_rows {
                text.push_str(&format!(
                    "  row @p{index}.g{}r{} at(group={},row={}) readonly\n",
                    row.group_index, row.row_index, row.group_index, row.row_index
                ));
            }
            let geometry = program.iec_geometry();
            let mut physical_offsets = BTreeSet::new();
            if let Some(geometry) = &geometry {
                for w in &geometry.horizontal {
                    physical_offsets.insert(w.offset);
                    let id = format!("@p{index}.h{}", w.offset);
                    text.push_str(&format!(
                        "  wire {id} from {} to {} HORIZONTAL\n",
                        Position::Iec {
                            group: w.group_index,
                            row: w.row_index,
                            x: w.start_x
                        }
                        .render(),
                        Position::Iec {
                            group: w.group_index,
                            row: w.row_index,
                            x: w.end_x
                        }
                        .render()
                    ));
                    sources.insert(id, Source::Wire { offset: w.offset });
                }
                for w in &geometry.vertical {
                    physical_offsets.extend([w.start_offset, w.end_offset]);
                    let id = format!("@p{index}.v{}", w.start_offset);
                    text.push_str(&format!(
                        "  wire {id} from {} to {} VERTICAL\n",
                        Position::Iec {
                            group: w.group_index,
                            row: w.start_row_index,
                            x: w.x
                        }
                        .render(),
                        Position::Iec {
                            group: w.group_index,
                            row: w.end_row_index,
                            x: w.x
                        }
                        .render()
                    ));
                    sources.insert(
                        id,
                        Source::Wire {
                            offset: w.start_offset.max(w.end_offset),
                        },
                    );
                }
            }
            let records = program
                .iec_record_frames()
                .ok_or_else(|| unsupported("IEC record layout is not recognized"))?;
            let functions = program.iec_function_blocks().unwrap_or_default();
            let links = program.iec_function_operand_links().unwrap_or_default();
            for record in records {
                if physical_offsets.contains(&record.offset) {
                    continue;
                }
                let id = format!("@p{index}.e{}", record.offset);
                let position = Position::Iec {
                    group: record.group_index,
                    row: record.row_index,
                    x: program.data[record.offset + 5],
                };
                let value = program
                    .strings
                    .iter()
                    .find(|s| s.offset == record.offset + 15)
                    .map(|s| s.value.as_str());
                let element = match record.kind {
                    IecRecordKind::Contact(code) | IecRecordKind::Coil(code) => iec_kind(code),
                    _ => None,
                };
                if let Some(function) = functions.iter().find(|f| f.record_offset == record.offset)
                {
                    let id = format!("@p{index}.f{}", record.offset);
                    text.push_str(&format!(
                        "  function {id} {} name {} signature {}\n",
                        position.render(),
                        quote(&function.name.value),
                        quote(&crate::storage::revision(
                            &program.data[record.offset..record.end]
                        ))
                    ));
                    sources.insert(
                        id,
                        Source::Function {
                            offset: function.name.offset,
                        },
                    );
                } else if let (IecRecordKind::FunctionOperand, Some(value), Some(link)) = (
                    record.kind,
                    value,
                    links.iter().find(|l| l.record_offset == record.offset),
                ) {
                    text.push_str(&format!("  argument {id} {} target {} pin {} pin_row {} pin_x {} direction {} type_mask {} array {} value {}\n", position.render(), quote(&format!("@p{index}.f{}",link.target_record_offset)), link.ordinal, link.pin_row_index, link.pin_raw_x, if link.is_output {"OUTPUT"} else {"INPUT"}, link.data_type_mask, link.is_array, quote(value)));
                    sources.insert(
                        id,
                        Source::Argument {
                            offset: record.offset + 15,
                        },
                    );
                } else if let (Some((family, kind)), Some(value)) = (element, value) {
                    text.push_str(&format!(
                        "  {family} {id} {} {kind} {}\n",
                        position.render(),
                        quote(value)
                    ));
                    if iec_rows.iter().any(|row| {
                        row.group_index == record.group_index
                            && row.row_index == record.row_index
                            && row.record_count == 1
                    }) {
                        isolated.insert(id.clone());
                    }
                    sources.insert(
                        id,
                        Source::Element {
                            offset: record.offset + 15,
                        },
                    );
                } else if let (IecRecordKind::Comment, Some(value)) = (record.kind, value)
                    && program.data[record.offset..record.offset + 6] == [0xff, 0x3f, 0, 0, 0, 1]
                    && !value.chars().any(char::is_control)
                {
                    text.push_str(&format!(
                        "  comment {id} {} IEC {}\n",
                        position.render(),
                        quote(value)
                    ));
                    sources.insert(
                        id,
                        Source::Comment {
                            offset: record.offset + 15,
                        },
                    );
                } else if let (IecRecordKind::Comment, Some(value)) = (record.kind, value) {
                    let kind = if program.data[record.offset + 1] == 0x40 {
                        "OUTPUT"
                    } else {
                        "IEC"
                    };
                    text.push_str(&format!(
                        "  annotation {id} {} {kind} {} hash {} readonly\n",
                        position.render(),
                        quote(value),
                        quote(&crate::storage::revision(
                            &program.data[record.offset..record.end]
                        ))
                    ));
                } else {
                    let raw = &program.data[record.offset..record.end];
                    text.push_str(&format!(
                        "  record {id} {} kind {} hash {} readonly\n",
                        position.render(),
                        quote(&format!("{:?}", record.kind)),
                        quote(&crate::storage::revision(raw))
                    ));
                }
            }
            // Actual connectivity is exported, including incomplete native circuits.
            if let Some(graph) = program.iec_circuit_layout() {
                for (i, edge) in graph.edges.iter().enumerate() {
                    let point = |p: xgwx::IecCircuitPoint| {
                        Position::Iec {
                            group: p.group_index,
                            row: p.row_index,
                            x: p.x,
                        }
                        .render()
                    };
                    text.push_str(&format!(
                        "  connection @p{index}.w{i} from {} to {} kind {} readonly\n",
                        point(edge.start),
                        point(edge.end),
                        quote(&format!("{:?}", edge.kind))
                    ));
                }
            } else {
                text.push_str("  geometry unavailable readonly\n");
            }
        } else {
            // The text writer has a legacy raw-string fallback for undecoded layouts.
            // A structural writer on a disposable clone proves the whole payload uses
            // the same native parser before exposing complete instruction updates.
            let instruction_probe = doc.clone().insert_ladder_row(index, 0);
            let instruction_layout = instruction_probe.is_ok();
            if let Err(reason) = instruction_probe {
                text.push_str(&format!(
                    "  instruction_layout error {} readonly\n",
                    quote(&reason.to_string())
                ));
            }
            let mut rows = BTreeSet::from([0]);
            for rung in &program.structure.rungs {
                rows.insert(rung.raw_y);
            }
            for c in &program.structure.rung_comments {
                rows.insert(c.raw_y);
            }
            for c in &program.structure.output_comments {
                rows.insert(c.raw_y);
            }
            for c in &program.structure.unknown_records {
                rows.insert(c.raw_y);
            }
            for y in rows {
                text.push_str(&format!("  rung @p{index}.r{y} at(y={y}) {{\n"));
                for cell in program
                    .structure
                    .rungs
                    .iter()
                    .filter(|r| r.raw_y == y)
                    .flat_map(|r| &r.cells)
                {
                    let id = format!("@p{index}.e{}", cell.offset);
                    let position = Position::Xgk { x: cell.raw_x, y };
                    if let Some((family, kind)) = element_kind(cell.contact, cell.coil) {
                        // Operandless logic records have no operand; other cells retain the exact decoded value.
                        let operand = cell
                            .operands
                            .first()
                            .map(String::as_str)
                            .unwrap_or(&cell.value);
                        let operand = if matches!(kind, "INVERSE" | "RISING" | "FALLING")
                            && family == "contact"
                        {
                            ""
                        } else {
                            operand
                        };
                        text.push_str(&format!(
                            "    {family} {id} {} {kind} {}\n",
                            position.render(),
                            quote(operand)
                        ));
                        sources.insert(
                            id,
                            Source::Element {
                                offset: cell.offset,
                            },
                        );
                    } else if instruction_layout
                        && cell.offset >= 19
                        && program.data.get(cell.offset - 19..cell.offset - 17) == Some(&[0, 34])
                        && matches!(program.data[cell.offset - 8], 0 | 32)
                        && (!cell.operands.is_empty()
                            || xgwx::ladder_instruction_catalog()
                                .iter()
                                .any(|s| s.operand_count == 0 && cell.value == s.mnemonic))
                    {
                        let mnemonic = cell.value.split(',').next().unwrap().trim();
                        let family = if program.data[cell.offset - 8] == 0 {
                            "COMPARISON"
                        } else {
                            "APPLICATION"
                        };
                        let catalog = if family == "COMPARISON" {
                            xgwx::ladder_comparison_catalog()
                        } else {
                            xgwx::ladder_instruction_catalog()
                        };
                        if catalog.iter().any(|s| s.mnemonic == mnemonic) {
                            text.push_str(&format!(
                                "    instruction {id} {} {family} mnemonic {} operands{}\n",
                                position.render(),
                                quote(mnemonic),
                                cell.operands
                                    .iter()
                                    .map(|o| format!(" {}", quote(o)))
                                    .collect::<String>()
                            ));
                            sources.insert(
                                id,
                                Source::Instruction {
                                    offset: cell.offset,
                                    expected: program
                                        .strings
                                        .iter()
                                        .find(|s| s.offset == cell.offset)
                                        .unwrap()
                                        .value
                                        .clone(),
                                    arity: cell.operands.len(),
                                },
                            );
                        } else {
                            text.push_str(&format!(
                                "    record {id} {} kind {} value {} readonly\n",
                                position.render(),
                                quote(&format!("{:?}", cell.kind)),
                                quote(&cell.value)
                            ));
                        }
                    } else {
                        text.push_str(&format!(
                            "    record {id} {} kind {} value {} readonly\n",
                            position.render(),
                            quote(&format!("{:?}", cell.kind)),
                            quote(&cell.value)
                        ));
                    }
                }
                for (kind, comments) in [
                    (
                        "RUNG",
                        program
                            .structure
                            .rung_comments
                            .iter()
                            .map(|c| (c.offset, c.raw_x, c.raw_y, &c.text))
                            .collect::<Vec<_>>(),
                    ),
                    (
                        "OUTPUT",
                        program
                            .structure
                            .output_comments
                            .iter()
                            .map(|c| (c.offset, c.raw_x, c.raw_y, &c.text))
                            .collect::<Vec<_>>(),
                    ),
                ] {
                    for (offset, x, cy, value) in comments.into_iter().filter(|c| c.2 == y) {
                        let id = format!("@p{index}.c{offset}");
                        text.push_str(&format!(
                            "    comment {id} {} {kind} {}\n",
                            Position::Xgk { x, y: cy }.render(),
                            quote(value)
                        ));
                        sources.insert(id, Source::Comment { offset });
                    }
                }
                for r in program
                    .structure
                    .unknown_records
                    .iter()
                    .filter(|r| r.raw_y == y)
                {
                    text.push_str(&format!(
                        "    opaque @p{index}.u{} {} hash {} readonly\n",
                        r.offset,
                        Position::Xgk { x: r.raw_x, y }.render(),
                        quote(&crate::storage::revision(&r.bytes))
                    ));
                }
                text.push_str("  }\n");
            }
            for (i, w) in program.structure.horizontal_lines.iter().enumerate() {
                text.push_str(&format!(
                    "  wire @p{index}.h{i} from {} to {} HORIZONTAL\n",
                    Position::Xgk {
                        x: w.raw_x_start,
                        y: w.raw_y
                    }
                    .render(),
                    Position::Xgk {
                        x: w.raw_x_end,
                        y: w.raw_y
                    }
                    .render()
                ));
                sources.insert(format!("@p{index}.h{i}"), Source::Wire { offset: 0 });
            }
            for (i, w) in program.structure.vertical_lines.iter().enumerate() {
                text.push_str(&format!(
                    "  wire @p{index}.v{i} from {} to {} VERTICAL\n",
                    Position::Xgk {
                        x: w.raw_x,
                        y: w.raw_y_start
                    }
                    .render(),
                    Position::Xgk {
                        x: w.raw_x,
                        y: w.raw_y_end
                    }
                    .render()
                ));
                sources.insert(format!("@p{index}.v{i}"), Source::Wire { offset: 0 });
            }
        }
        text.push_str("}\n");
        parse(&text)?;
        Ok(Self {
            text,
            dialect,
            program_index: index,
            sources,
            iec_rows,
            isolated,
            sfc: None,
        })
    }

    pub fn lower_patch(&self, patch: &str) -> Result<Vec<EditOperation>, StoreError> {
        if let Some(sfc) = &self.sfc {
            return sfc.lower_patch(patch);
        }
        let edited = apply_unified_patch(&self.text, patch)?;
        let before = parse(&self.text)?;
        let after = parse(&edited)?;
        if before.frozen != after.frozen {
            return Err(unsupported(
                "program/rung identities, derived connections, instructions and opaque records are read-only in text IR v1",
            ));
        }
        let mut work = Vec::<(usize, Vec<EditOperation>)>::new();
        let mut structural = vec![];
        let mut topology_change = false;
        let mut wire_additions = vec![];
        let mut additions = Vec::<(bool, u32, u8, EditOperation)>::new();
        let mut occupied = BTreeSet::new();
        for node in after.nodes.values() {
            if matches!(node.body, Body::Element { .. }) {
                let key = format!("{:?}", node.position);
                if !occupied.insert(key) {
                    return Err(error("two elements cannot occupy the same native position"));
                }
            }
        }
        for (id, old) in &before.nodes {
            let new = after.nodes.get(id);
            if new == Some(old) {
                continue;
            }
            let source = &self.sources[id];
            if let Some(new) = new
                && (old.scope != new.scope || old.position != new.position)
            {
                return Err(unsupported(format!(
                    "{id}: moving an existing object is unsupported; native positions and scope must remain unchanged"
                )));
            }
            let mut ops = vec![];
            let make = |name: &str, args: Value| EditOperation {
                name: name.into(),
                args,
            };
            let p = self.program_index;
            match (&old.body, new.map(|n| &n.body), source) {
                (
                    Body::Instruction { family, .. },
                    Some(Body::Instruction {
                        family: nf,
                        mnemonic,
                        operands,
                    }),
                    Source::Instruction {
                        offset,
                        expected,
                        arity,
                    },
                ) => {
                    if family != nf {
                        return Err(unsupported(
                            "instruction family is read-only; use its native application/comparison writer",
                        ));
                    }
                    if operands.len() != *arity {
                        topology_change = true;
                    }
                    let replacement = std::iter::once(mnemonic.as_str())
                        .chain(operands.iter().map(String::as_str))
                        .collect::<Vec<_>>()
                        .join(",");
                    ops.push(make("update_ladder_cell_text",json!({"program_index":p,"offset":offset,"expected":expected,"replacement":replacement})));
                    work.push((*offset, ops));
                }
                (
                    Body::Instruction { family, .. },
                    None,
                    Source::Instruction {
                        offset, expected, ..
                    },
                ) => {
                    topology_change = true;
                    ops.push(make(
                        if family == "COMPARISON" {
                            "delete_ladder_comparison"
                        } else {
                            "delete_ladder_instruction"
                        },
                        json!({"program_index":p,"offset":offset,"expected":expected}),
                    ));
                    work.push((*offset, ops));
                }

                (
                    Body::Function { name, signature },
                    Some(Body::Function {
                        name: replacement,
                        signature: ns,
                    }),
                    Source::Function { offset },
                ) => {
                    if signature != ns {
                        return Err(unsupported(
                            "function signature is read-only; preserve its source fingerprint",
                        ));
                    }
                    let adapter = match name.as_str() {
                        "ADD" | "SUB" | "MUL" | "DIV" => "update_iec_ld_arithmetic_function",
                        "EQ" | "GT" | "GE" | "LT" | "LE" => "update_iec_ld_comparison_function",
                        _ => {
                            return Err(unsupported(format!(
                                "{id}: function name changes are supported only within captured arithmetic or comparison families"
                            )));
                        }
                    };
                    ops.push(make(adapter,json!({"program_index":p,"offset":offset,"expected":name,"replacement":replacement})));
                    work.push((*offset, ops));
                }
                (
                    Body::Argument {
                        target,
                        pin,
                        pin_row,
                        pin_x,
                        direction,
                        type_mask,
                        array,
                        value,
                    },
                    Some(Body::Argument {
                        target: nt,
                        pin: np,
                        pin_row: nr,
                        pin_x: nx,
                        direction: nd,
                        type_mask: nm,
                        array: na,
                        value: replacement,
                    }),
                    Source::Argument { offset },
                ) => {
                    if (target, pin, pin_row, pin_x, direction, type_mask, array)
                        != (nt, np, nr, nx, nd, nm, na)
                    {
                        return Err(unsupported(
                            "function operand target, pin, direction and type metadata are read-only",
                        ));
                    }
                    if replacement.is_empty() {
                        return Err(unsupported(
                            "empty function operands invoke structural deletion; use the direct output-operand deletion operation and re-export",
                        ));
                    }
                    ops.push(make("update_iec_ld_function_operand",json!({"program_index":p,"offset":offset,"expected":value,"replacement":replacement})));
                    work.push((*offset, ops));
                }

                (
                    Body::Element {
                        family,
                        kind,
                        operand,
                    },
                    Some(Body::Element {
                        family: nf,
                        kind: nk,
                        operand: no,
                    }),
                    Source::Element { offset },
                ) => {
                    if family != nf {
                        return Err(unsupported(format!(
                            "{id}: changing element family is unsupported"
                        )));
                    }
                    match old.position.as_ref().unwrap() {
                        Position::Xgk { x, y } => {
                            let column = xgk_column(*x)?;
                            let a = xgk_kind(family, kind)?;
                            let b = xgk_kind(nf, nk)?;
                            ops.push(make("edit_ladder_cell",json!({"program_index":p,"edit":{"raw_y":y,"column":column,"expected":{"kind":a,"operand":operand},"replacement":{"kind":b,"operand":no}}})));
                        }
                        Position::Iec { .. } => {
                            let a = iec_writer_kind(family, kind)?;
                            let b = iec_writer_kind(nf, nk)?;
                            if kind != nk {
                                ops.push(make(if family=="contact" {"update_iec_ld_contact_kind"} else {"update_iec_ld_coil_kind"},json!({"program_index":p,"offset":offset,"expected":a,"replacement":b})));
                            }
                            if operand != no {
                                ops.push(make("update_iec_ld_element_operand",json!({"program_index":p,"offset":offset,"expected":operand,"replacement":no})));
                            }
                        }
                    }
                    work.push((*offset, ops));
                }
                (
                    Body::Element {
                        family,
                        kind,
                        operand,
                    },
                    None,
                    Source::Element { offset },
                ) => {
                    let op = match old.position.as_ref().unwrap() {
                        Position::Xgk { x, y } => make(
                            "edit_ladder_cell",
                            json!({"program_index":p,"edit":{"raw_y":y,"column":xgk_column(*x)?,"expected":{"kind":xgk_kind(family,kind)?,"operand":operand},"replacement":null}}),
                        ),
                        Position::Iec { .. } if self.isolated.contains(id) => make(
                            "delete_iec_ld_isolated_element",
                            json!({"program_index":p,"offset":offset-15,"expected":operand}),
                        ),
                        Position::Iec { x, .. } => {
                            topology_change = true;
                            if family == "contact" {
                                make(
                                    "delete_iec_ld_contact",
                                    json!({"program_index":p,"contact_offset":offset-15,"expected_raw_x":x,"expected_contact_kind":iec_writer_kind(family,kind)?,"expected_variable":operand}),
                                )
                            } else {
                                make(
                                    "delete_iec_ld_terminal_coil",
                                    json!({"program_index":p,"coil_record_offset":offset-15,"expected_variable":operand}),
                                )
                            }
                        }
                    };
                    work.push((*offset, vec![op]));
                }
                (Body::Wire { end, kind }, None, Source::Wire { offset }) => {
                    topology_change = true;
                    let start = old.position.as_ref().unwrap();
                    let op = match (start, end, kind.as_str()) {
                        (
                            Position::Xgk { x: sx, y: sy },
                            Position::Xgk { x: ex, y: ey },
                            "HORIZONTAL",
                        ) if sy == ey => make(
                            "delete_ladder_horizontal_wire",
                            json!({"program_index":p,"raw_y":sy,"start_x":sx,"end_x":ex}),
                        ),
                        (
                            Position::Xgk { x: sx, y: sy },
                            Position::Xgk { x: ex, y: ey },
                            "VERTICAL",
                        ) if sx == ex => make(
                            "delete_ladder_vertical_wire",
                            json!({"program_index":p,"raw_x":sx,"start_y":sy,"end_y":ey}),
                        ),
                        (
                            Position::Iec {
                                group: sg,
                                row: sr,
                                x: sx,
                            },
                            Position::Iec {
                                group: eg,
                                row: er,
                                x: ex,
                            },
                            "HORIZONTAL",
                        ) if sg == eg && sr == er => make(
                            "delete_iec_ld_horizontal_wire_record",
                            json!({"program_index":p,"offset":offset,"expected_start_x":sx,"expected_end_x":ex}),
                        ),
                        (
                            Position::Iec {
                                group: sg,
                                row: sr,
                                x: sx,
                            },
                            Position::Iec {
                                group: eg,
                                row: er,
                                x: ex,
                            },
                            "VERTICAL",
                        ) if sg == eg && sx == ex => make(
                            "edit_iec_ld_vertical_wire",
                            json!({"program_index":p,"group_index":sg,"start_row_index":sr,"end_row_index":er,"x":sx,"expected":true,"present":false}),
                        ),
                        _ => return Err(unsupported(format!("{id}: unsupported wire geometry"))),
                    };
                    work.push((*offset, vec![op]));
                }
                (
                    Body::Comment { kind, text },
                    Some(Body::Comment { kind: nk, text: nt }),
                    Source::Comment { offset },
                ) => {
                    if kind != nk {
                        return Err(unsupported(format!(
                            "{id}: changing comment kind is unsupported"
                        )));
                    }
                    match old.position.as_ref().unwrap() {
                        Position::Xgk{y,..} => ops.push(make("edit_ladder_comment",json!({"program_index":p,"edit":{"kind":if kind=="RUNG" {"Rung"}else{"Output"},"raw_y":y,"expected":text,"replacement":nt}}))),
                        Position::Iec{..} => ops.push(make("update_iec_ld_comment",json!({"program_index":p,"offset":offset,"expected":text,"replacement":nt}))),
                    }
                    work.push((*offset, ops));
                }
                (Body::Comment { kind, text }, None, Source::Comment { .. }) if kind == "RUNG" => {
                    let Position::Xgk { y, .. } = old.position.as_ref().unwrap() else {
                        return Err(unsupported("IEC comment deletion is unsupported"));
                    };
                    structural.push((
                        *y,
                        make(
                            "delete_ladder_rung_comment",
                            json!({"program_index":p,"raw_y":y,"expected":text}),
                        ),
                    ));
                }
                (
                    Body::Symbol {
                        name,
                        address,
                        type_name,
                        ..
                    },
                    Some(Body::Symbol {
                        name: nn,
                        address: na,
                        type_name: nt,
                        description,
                    }),
                    source,
                ) => {
                    if name != nn || address != na || type_name != nt {
                        return Err(unsupported(format!(
                            "{id}: only symbol descriptions are editable in text IR v1"
                        )));
                    }
                    let operation = match source {
                        Source::Global { index } => make(
                            "update_variable",
                            json!({"variable_index":index,"patch":{"description":description}}),
                        ),
                        Source::Local { index } => {
                            let Body::Symbol {
                                description: expected,
                                ..
                            } = &old.body
                            else {
                                unreachable!()
                            };
                            make(
                                "update_iec_local_symbol_description",
                                json!({"program_index":p,"symbol_index":index,"expected_name":name,"expected_description":expected,"replacement":description}),
                            )
                        }
                        _ => unreachable!(),
                    };
                    work.push((usize::MAX, vec![operation]));
                }
                _ => {
                    return Err(unsupported(format!(
                        "{id}: this object cannot be removed or changed to another kind"
                    )));
                }
            }
        }
        for (id, new) in &after.nodes {
            if before.nodes.contains_key(id) {
                continue;
            }
            if !id.starts_with("@new-") || id.len() <= 5 {
                return Err(error(
                    "new objects require a fresh @new-<name> identity; retain existing source identities",
                ));
            }
            if let Body::Instruction {
                family,
                mnemonic,
                operands,
            } = &new.body
            {
                let Some(Position::Xgk { x, y }) = &new.position else {
                    return Err(unsupported("instruction statements require XGK positions"));
                };
                if self.dialect != "XGK" {
                    return Err(unsupported("instruction statements require XGK dialect"));
                }
                check_xgk_scope(&before, new, *y)?;
                topology_change = true;
                let op = if family == "APPLICATION" {
                    let spec = xgwx::ladder_instruction_catalog()
                        .iter()
                        .find(|s| s.mnemonic == mnemonic)
                        .ok_or_else(|| {
                            unsupported(
                                "unknown application mnemonic; discover catalog.instructions",
                            )
                        })?;
                    let anchor = 94usize
                        .checked_sub(spec.operand_count * 3)
                        .ok_or_else(|| unsupported("instruction width exceeds output row"))?;
                    if usize::from(*x) != anchor {
                        return Err(error(format!(
                            "application native x must be {anchor} for this catalog arity"
                        )));
                    }
                    make_instruction(
                        self.program_index,
                        "insert_ladder_instruction",
                        json!({"raw_y":y,"mnemonic":mnemonic,"operands":operands}),
                    )
                } else {
                    make_instruction(
                        self.program_index,
                        "insert_ladder_comparison",
                        json!({"raw_y":y,"column":xgk_column(*x)?,"mnemonic":mnemonic,"operands":operands}),
                    )
                };
                additions.push((true, *y, *x, op));
                continue;
            }
            if let Body::Wire { end, kind } = &new.body {
                check_program_scope(&before, new)?;
                topology_change = true;
                let start = new.position.as_ref().unwrap();
                let op = match (start, end, kind.as_str()) {
                    (
                        Position::Xgk { x: sx, y: sy },
                        Position::Xgk { x: ex, y: ey },
                        "VERTICAL",
                    ) if self.dialect == "XGK"
                        && sx == ex
                        && ey.checked_sub(*sy) == Some(4)
                        && (3..=27).contains(sx)
                        && sx.is_multiple_of(3) =>
                    {
                        for y in [sy, ey] {
                            if !before.frozen.iter().any(|line| {
                                line.first().map(String::as_str) == Some("rung")
                                    && line.get(6) == Some(&y.to_string())
                            }) {
                                return Err(error(
                                    "XGK branch endpoints must be exported adjacent rung positions",
                                ));
                            }
                        }
                        EditOperation {
                            name: "edit_ladder_branch".into(),
                            args: json!({"program_index":self.program_index,"edit":{"raw_y":sy,"boundary":sx/3,"expected":false,"present":true}}),
                        }
                    }
                    (
                        Position::Iec {
                            group: sg,
                            row: sr,
                            x: sx,
                        },
                        Position::Iec {
                            group: eg,
                            row: er,
                            x: ex,
                        },
                        "VERTICAL",
                    ) if self.dialect == "IEC" && sg == eg && sx == ex => {
                        if !self
                            .iec_rows
                            .iter()
                            .any(|r| r.group_index == *sg && r.row_index == *sr)
                            || !self
                                .iec_rows
                                .iter()
                                .any(|r| r.group_index == *eg && r.row_index == *er)
                        {
                            return Err(error(
                                "IEC vertical wire endpoints must be exported rows in the same group",
                            ));
                        }
                        EditOperation {
                            name: "edit_iec_ld_vertical_wire".into(),
                            args: json!({"program_index":self.program_index,"group_index":sg,"start_row_index":sr,"end_row_index":er,"x":sx,"expected":false,"present":true}),
                        }
                    }
                    _ => {
                        return Err(unsupported(
                            "new text wires currently require supported vertical branch geometry; horizontal drawing and endpoint moves use direct APIs",
                        ));
                    }
                };
                wire_additions.push(op);
                continue;
            }
            if let Body::Element {
                family,
                kind,
                operand,
            } = &new.body
            {
                let op = match new.position.as_ref().unwrap() {
                    Position::Xgk { x, y } if self.dialect == "XGK" => {
                        check_xgk_scope(&before, new, *y)?;
                        EditOperation {
                            name: "edit_ladder_cell".into(),
                            args: json!({"program_index":self.program_index,"edit":{"raw_y":y,"column":xgk_column(*x)?,"expected":null,"replacement":{"kind":xgk_kind(family,kind)?,"operand":operand}}}),
                        }
                    }
                    Position::Iec { group, row, x } if self.dialect == "IEC" => {
                        check_program_scope(&before, new)?;
                        let matching = self
                            .iec_rows
                            .iter()
                            .filter(|r| r.row_index == *row)
                            .collect::<Vec<_>>();
                        let empty_canvas = self.iec_rows.is_empty() && *group == 0 && *row == 0;
                        if !empty_canvas
                            && (matching.len() != 1 || matching[0].group_index != *group)
                        {
                            return Err(error(
                                "IEC insertion requires an unambiguous exported group/row",
                            ));
                        }
                        let record_count = matching.first().map_or(0, |r| r.record_count);
                        let terminal = family=="coil" && *x==94 && record_count==1 && before.nodes.values().any(|n|
                            matches!(n.position,Some(Position::Iec{group:g,row:r,x:1}) if g==*group && r==*row)
                            && matches!(&n.body,Body::Element{family,..} if family=="contact"));
                        if record_count != 0 && !terminal {
                            return Err(unsupported(
                                "IEC text insertion currently requires an empty row, or a terminal coil at x=94 after an isolated contact",
                            ));
                        }
                        EditOperation {
                            name: "insert_iec_ld_single_element".into(),
                            args: json!({"program_index":self.program_index,"row_index":row,"raw_x":x,"category":family,"kind":iec_writer_kind(family,kind)?,"operand":operand}),
                        }
                    }
                    _ => return Err(error("element coordinates must match the exported dialect")),
                };
                let (row, x) = match new.position.as_ref().unwrap() {
                    Position::Xgk { x, y } => (*y, *x),
                    Position::Iec { row, x, .. } => (u32::from(*row), *x),
                };
                additions.push((family == "coil", row, x, op));
                continue;
            }
            let Body::Comment { kind, text } = &new.body else {
                return Err(unsupported(format!(
                    "{id}: only contacts, coils and native XGK comments can be created in text IR v1"
                )));
            };
            let Some(Position::Xgk { x, y }) = &new.position else {
                return Err(unsupported(
                    "IEC comment creation is unsupported in text IR v1",
                ));
            };
            if self.dialect != "XGK"
                || !matches!(kind.as_str(), "RUNG" | "OUTPUT")
                || *x != if kind == "RUNG" { 1 } else { 97 }
            {
                return Err(error(
                    "new XGK RUNG comments require x=1; OUTPUT comments require x=97",
                ));
            }
            check_xgk_scope(&before, new, *y)?;
            let op = EditOperation {
                name: "edit_ladder_comment".into(),
                args: json!({"program_index":self.program_index,"edit":{"kind":if kind=="RUNG"{"Rung"}else{"Output"},"raw_y":y,"expected":null,"replacement":text}}),
            };
            if kind == "RUNG" {
                structural.push((*y, op));
            } else {
                work.push((0, vec![op]));
            }
        }
        // Descending original record offsets keep lower offsets valid when a
        // higher record changes size. Row insertions/deletions run afterward.
        work.sort_by_key(|entry| std::cmp::Reverse(entry.0));
        structural.sort_by_key(|entry| std::cmp::Reverse(entry.0));
        let mut operations = work
            .into_iter()
            .flat_map(|(_, ops)| ops)
            .collect::<Vec<_>>();
        additions.sort_by_key(|(coil, row, x, _)| (*row, *coil, *x));
        operations.extend(additions.into_iter().map(|(_, _, _, op)| op));
        operations.extend(wire_additions);
        operations.extend(structural.into_iter().map(|(_, op)| op));
        if topology_change && operations.len() != 1 {
            return Err(unsupported(
                "instruction creation/deletion/arity changes, connected element deletion and wire changes must be the sole mutation in a text patch; commit and re-export before dependent edits",
            ));
        }
        Ok(operations)
    }
}

fn xgk_column(x: u8) -> Result<u8, StoreError> {
    if x == 94 {
        return Ok(9);
    }
    if !(1..=25).contains(&x) || !(x - 1).is_multiple_of(3) {
        return Err(error(
            "XGK contacts use x=1,4,...25; the output column uses x=94",
        ));
    }
    Ok((x - 1) / 3)
}
fn make_instruction(program_index: usize, name: &str, mut args: Value) -> EditOperation {
    args["program_index"] = json!(program_index);
    EditOperation {
        name: name.into(),
        args,
    }
}
fn check_program_scope(before: &Parsed, node: &Node) -> Result<(), StoreError> {
    if node.scope.len() != 1 || !before.frozen.contains(&node.scope[0]) {
        return Err(error(
            "elements and wires requiring program scope must belong directly to the exported program",
        ));
    }
    Ok(())
}
fn check_xgk_scope(before: &Parsed, node: &Node, y: u32) -> Result<(), StoreError> {
    let valid = node.scope.len() == 2
        && before.frozen.contains(&node.scope[0])
        && before.frozen.contains(&node.scope[1]);
    if !valid
        || node.scope[1].first().map(String::as_str) != Some("rung")
        || node.scope[1].get(6) != Some(&y.to_string())
    {
        return Err(error(
            "XGK objects must belong to an exported rung at the same native y",
        ));
    }
    Ok(())
}

fn xgk_kind(family: &str, kind: &str) -> Result<&'static str, StoreError> {
    Ok(match (family, kind) {
        ("contact", "NO") => "NormallyOpen",
        ("contact", "NC") => "NormallyClosed",
        ("contact", "INVERSE") => "Inverse",
        ("contact", "RISING") => "RisingPulse",
        ("contact", "FALLING") => "FallingPulse",
        ("contact", "ADDR_RISING") => "AddressedRisingPulse",
        ("contact", "ADDR_FALLING") => "AddressedFallingPulse",
        ("contact", "ADDR_NEGATED_RISING") => "AddressedRisingPulseNot",
        ("contact", "ADDR_NEGATED_FALLING") => "AddressedFallingPulseNot",
        ("coil", "OUT") => "Output",
        ("coil", "INVERSE") => "InverseOutput",
        ("coil", "SET") => "Set",
        ("coil", "RESET") => "Reset",
        ("coil", "RISING") => "RisingPulseOutput",
        ("coil", "FALLING") => "FallingPulseOutput",
        _ => return Err(error(format!("unknown {family} kind {kind}"))),
    })
}
fn iec_writer_kind(family: &str, kind: &str) -> Result<&'static str, StoreError> {
    Ok(match (family, kind) {
        ("contact", "NO") => "NO",
        ("contact", "NC") => "NC",
        ("contact", "RISING") | ("coil", "RISING") => "RISING",
        ("contact", "FALLING") | ("coil", "FALLING") => "FALLING",
        ("contact", "NEGATED_RISING") => "NEGATED_RISING",
        ("contact", "NEGATED_FALLING") => "NEGATED_FALLING",
        ("coil", "OUT") => "OUTPUT",
        ("coil", "INVERSE") => "INVERSE",
        ("coil", "SET") => "SET",
        ("coil", "RESET") => "RESET",
        _ => return Err(error(format!("unknown IEC {family} kind {kind}"))),
    })
}

// Quoted UTF-8 strings support \n, \r, \t, quotes and backslashes. Each
// statement occupies one line; punctuation and whitespace are insignificant.
pub(crate) fn lex(line: &str) -> Result<Vec<String>, StoreError> {
    let mut tokens = vec![];
    let mut i = 0;
    let bytes = line.as_bytes();
    while i < bytes.len() {
        if bytes[i].is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if bytes[i] == b'"' {
            let start = i;
            i += 1;
            while i < bytes.len() && bytes[i] != b'"' {
                if bytes[i] == b'\\' {
                    i += 1;
                }
                i += 1;
            }
            if i >= bytes.len() {
                return Err(error("unterminated string"));
            }
            i += 1;
            let value: String = serde_json::from_str(&line[start..i]).map_err(error)?;
            tokens.push(quote(&value));
        } else if b"{}(),=".contains(&bytes[i]) {
            tokens.push((bytes[i] as char).to_string());
            i += 1;
        } else {
            let start = i;
            while i < bytes.len()
                && !bytes[i].is_ascii_whitespace()
                && !b"{}(),=\"".contains(&bytes[i])
            {
                i += 1;
            }
            if i == start {
                return Err(error("invalid token"));
            }
            tokens.push(line[start..i].into());
        }
    }
    Ok(tokens)
}
fn string(token: &str) -> Result<String, StoreError> {
    serde_json::from_str(token).map_err(error)
}
fn number<T: std::str::FromStr>(token: &str) -> Result<T, StoreError> {
    token
        .parse()
        .map_err(|_| error(format!("invalid coordinate {token}")))
}
fn position(tokens: &[String]) -> Result<(Position, usize), StoreError> {
    let close = tokens
        .iter()
        .position(|s| s == ")")
        .ok_or_else(|| error("position requires closing parenthesis"))?;
    if tokens.first().map(String::as_str) != Some("at")
        || tokens.get(1).map(String::as_str) != Some("(")
    {
        return Err(error("expected at(...)"));
    }
    let mut fields = BTreeMap::new();
    let mut i = 2;
    while i < close {
        if i + 2 >= close || tokens[i + 1] != "=" {
            return Err(error("position fields require key=value"));
        }
        if fields
            .insert(tokens[i].as_str(), tokens[i + 2].as_str())
            .is_some()
        {
            return Err(error("duplicate position field"));
        }
        i += 3;
        if i < close {
            if tokens[i] != "," {
                return Err(error("position fields require commas"));
            }
            i += 1;
            if i == close {
                return Err(error("trailing position comma"));
            }
        }
    }
    let p = if fields.len() == 2 && fields.contains_key("x") && fields.contains_key("y") {
        Position::Xgk {
            x: number(fields["x"])?,
            y: number(fields["y"])?,
        }
    } else if fields.len() == 3
        && fields.contains_key("group")
        && fields.contains_key("row")
        && fields.contains_key("x")
    {
        Position::Iec {
            group: number(fields["group"])?,
            row: number(fields["row"])?,
            x: number(fields["x"])?,
        }
    } else {
        return Err(error("position must be x,y for XGK or group,row,x for IEC"));
    };
    Ok((p, close + 1))
}
fn parse(text: &str) -> Result<Parsed, StoreError> {
    let mut parsed = Parsed {
        nodes: BTreeMap::new(),
        frozen: vec![],
    };
    let mut scope = vec![];
    let mut ids = BTreeSet::new();
    for (line_no, line) in text.lines().enumerate() {
        let t = lex(line).map_err(|e| error(format!("line {}: {}", line_no + 1, e.message)))?;
        if t.is_empty() {
            continue;
        }
        for id in t.iter().filter(|s| s.starts_with('@')) {
            if !ids.insert(id.clone()) {
                return Err(error(format!("duplicate identity {id}")));
            }
        }
        if t.last().map(String::as_str) == Some("{") {
            if !matches!(t[0].as_str(), "program" | "rung") {
                return Err(error("only program and rung blocks are allowed"));
            }
            if (t[0] == "program" && !scope.is_empty()) || (t[0] == "rung" && scope.len() != 1) {
                return Err(error(
                    "program blocks cannot nest; rung blocks belong directly to a program",
                ));
            }
            scope.push(t.clone());
            parsed.frozen.push(t);
            continue;
        }
        if t == ["}"] {
            scope
                .pop()
                .ok_or_else(|| error("unmatched closing brace"))?;
            parsed.frozen.push(t);
            continue;
        }
        if matches!(t[0].as_str(), "contact" | "coil" | "comment") {
            let id = t
                .get(1)
                .filter(|s| s.starts_with('@'))
                .ok_or_else(|| error("object requires @identity"))?
                .clone();
            let (p, n) = position(&t[2..])?;
            let rest = &t[2 + n..];
            if rest.len() != 2 {
                return Err(error("element/comment requires kind and a quoted value"));
            }
            let value = string(&rest[1])?;
            let body = if t[0] == "comment" {
                Body::Comment {
                    kind: rest[0].clone(),
                    text: value,
                }
            } else {
                Body::Element {
                    family: t[0].clone(),
                    kind: rest[0].clone(),
                    operand: value,
                }
            };
            parsed.nodes.insert(
                id.clone(),
                Node {
                    id,
                    scope: scope.clone(),
                    position: Some(p),
                    body,
                },
            );
        } else if t[0] == "instruction" {
            if t.len() < 3 || !t[1].starts_with('@') {
                return Err(error("instruction requires @identity and position"));
            }
            let (p, n) = position(&t[2..])?;
            if !matches!(p, Position::Xgk { .. }) {
                return Err(error("instruction requires XGK position"));
            }
            let rest = &t[2 + n..];
            if rest.len() < 4
                || !matches!(rest[0].as_str(), "APPLICATION" | "COMPARISON")
                || rest[1] != "mnemonic"
                || rest[3] != "operands"
            {
                return Err(error(
                    "instruction requires family, mnemonic and quoted operands",
                ));
            }
            let mnemonic = string(&rest[2])?;
            if mnemonic.is_empty()
                || mnemonic
                    .chars()
                    .any(|c| c == ',' || c.is_whitespace() || c == '\'')
            {
                return Err(error("instruction mnemonic must be one token"));
            }
            let operands = rest[4..]
                .iter()
                .map(|s| string(s))
                .collect::<Result<Vec<_>, _>>()?;
            for operand in &operands {
                let literal = operand
                    .strip_prefix('\'')
                    .and_then(|s| s.strip_suffix('\''))
                    .is_some_and(|s| !s.contains('\''));
                if !literal && (operand.contains(',') || operand.contains('\'')) {
                    return Err(error(
                        "each instruction operand must be one token or one single-quoted native string literal",
                    ));
                }
            }
            let id = t[1].clone();
            parsed.nodes.insert(
                id.clone(),
                Node {
                    id,
                    scope: scope.clone(),
                    position: Some(p),
                    body: Body::Instruction {
                        family: rest[0].clone(),
                        mnemonic,
                        operands,
                    },
                },
            );
        } else if matches!(t[0].as_str(), "function" | "argument") {
            if t.len() < 3 || !t[1].starts_with('@') {
                return Err(error(
                    "function/argument requires @identity and native position",
                ));
            }
            let (p, n) = position(&t[2..])?;
            if !matches!(p, Position::Iec { .. }) {
                return Err(error("functions and arguments require IEC positions"));
            }
            let rest = &t[2 + n..];
            let body = if t[0] == "function" {
                if rest.len() != 4 || rest[0] != "name" || rest[2] != "signature" {
                    return Err(error("function requires name and source signature"));
                }
                Body::Function {
                    name: string(&rest[1])?,
                    signature: string(&rest[3])?,
                }
            } else {
                if rest.len() != 16
                    || rest[0] != "target"
                    || rest[2] != "pin"
                    || rest[4] != "pin_row"
                    || rest[6] != "pin_x"
                    || rest[8] != "direction"
                    || rest[10] != "type_mask"
                    || rest[12] != "array"
                    || rest[14] != "value"
                    || !matches!(rest[9].as_str(), "INPUT" | "OUTPUT")
                    || !matches!(rest[13].as_str(), "true" | "false")
                {
                    return Err(error(
                        "argument requires target, pin, pin_row, pin_x, direction, type_mask, array and value",
                    ));
                }
                Body::Argument {
                    target: string(&rest[1])?,
                    pin: rest[3].parse().map_err(|_| error("invalid pin ordinal"))?,
                    pin_row: rest[5].parse().map_err(|_| error("invalid pin row"))?,
                    pin_x: rest[7].parse().map_err(|_| error("invalid pin x"))?,
                    direction: rest[9].clone(),
                    type_mask: rest[11]
                        .parse()
                        .map_err(|_| error("invalid pin type mask"))?,
                    array: rest[13] == "true",
                    value: string(&rest[15])?,
                }
            };
            let id = t[1].clone();
            parsed.nodes.insert(
                id.clone(),
                Node {
                    id,
                    scope: scope.clone(),
                    position: Some(p),
                    body,
                },
            );
        } else if t[0] == "wire" && t.last().map(String::as_str) != Some("readonly") {
            if t.len() < 5 || !t[1].starts_with('@') || t[2] != "from" {
                return Err(error(
                    "wire requires @identity, from at(...), to at(...) and kind",
                ));
            }
            let (start, n) = position(&t[3..])?;
            let next = 3 + n;
            if t.get(next).map(String::as_str) != Some("to") {
                return Err(error("wire requires to at(...)"));
            }
            let (end, m) = position(&t[next + 1..])?;
            let rest = &t[next + 1 + m..];
            if rest.len() != 1 || !matches!(rest[0].as_str(), "HORIZONTAL" | "VERTICAL") {
                return Err(error("wire kind must be HORIZONTAL or VERTICAL"));
            }
            let id = t[1].clone();
            parsed.nodes.insert(
                id.clone(),
                Node {
                    id,
                    scope: scope.clone(),
                    position: Some(start),
                    body: Body::Wire {
                        end,
                        kind: rest[0].clone(),
                    },
                },
            );
        } else if t[0] == "symbol" {
            if t.len() != 9
                || t[3] != "address"
                || t[5] != "type"
                || t[7] != "description"
                || !t[1].starts_with('@')
            {
                return Err(error(
                    "symbol requires @id, name, address, type and description",
                ));
            }
            let id = t[1].clone();
            parsed.nodes.insert(
                id.clone(),
                Node {
                    id,
                    scope: scope.clone(),
                    position: None,
                    body: Body::Symbol {
                        name: string(&t[2])?,
                        address: string(&t[4])?,
                        type_name: string(&t[6])?,
                        description: string(&t[8])?,
                    },
                },
            );
        } else {
            parsed.frozen.push(t);
        }
    }
    if !scope.is_empty() {
        return Err(error("unclosed program/rung block"));
    }
    Ok(parsed)
}

/// Exact single-projection unified diff. No fuzz, path access, or context search.
/// Both hunk coordinate systems and declared counts are checked.
pub fn apply_unified_patch(original: &str, patch: &str) -> Result<String, StoreError> {
    if patch.len() > 4 * 1024 * 1024 {
        return Err(error("patch exceeds 4 MiB"));
    }
    if patch.is_empty() {
        return Ok(original.into());
    }
    let lines = original.lines().collect::<Vec<_>>();
    let input = patch.lines().collect::<Vec<_>>();
    let mut out = vec![];
    let mut i = 0;
    let mut cursor = 0;
    let mut hunks = 0;
    if input.first().is_some_and(|l| l.starts_with("--- ")) {
        if !input.get(1).is_some_and(|l| l.starts_with("+++ ")) {
            return Err(error("missing +++ header"));
        }
        i = 2;
    }
    while i < input.len() {
        let header = input[i];
        let parts = header.split_whitespace().collect::<Vec<_>>();
        if parts.len() < 4 || parts[0] != "@@" || parts[3] != "@@" {
            return Err(error("expected unified diff hunk header"));
        }
        fn range(s: &str, prefix: char) -> Result<(usize, usize), StoreError> {
            let body = s
                .strip_prefix(prefix)
                .ok_or_else(|| error("invalid hunk range"))?;
            let mut v = body.split(',');
            let start = number::<usize>(v.next().unwrap())?;
            let count = v.next().map(number::<usize>).transpose()?.unwrap_or(1);
            if v.next().is_some() || (count > 0 && start == 0) {
                return Err(error("invalid hunk range"));
            }
            Ok((if count == 0 { start } else { start - 1 }, count))
        }
        let (start, old_count) = range(parts[1], '-')?;
        let (new_start, new_count) = range(parts[2], '+')?;
        if start < cursor || start > lines.len() {
            return Err(error("overlapping or out-of-range hunk"));
        }
        out.extend(lines[cursor..start].iter().map(|s| s.to_string()));
        cursor = start;
        if new_start != out.len() {
            return Err(error("new hunk position does not match preceding changes"));
        }
        i += 1;
        let mut removed = 0;
        let mut added = 0;
        while i < input.len() && !input[i].starts_with("@@ ") {
            let line = input[i];
            let (prefix, value) = line
                .split_at_checked(1)
                .ok_or_else(|| error("hunk lines require a prefix"))?;
            match prefix {
                " " | "-" => {
                    if lines.get(cursor) != Some(&value) {
                        return Err(error(format!(
                            "patch context differs at source line {}",
                            cursor + 1
                        )));
                    }
                    cursor += 1;
                    removed += 1;
                    if prefix == " " {
                        out.push(value.into());
                        added += 1;
                    }
                }
                "+" => {
                    out.push(value.into());
                    added += 1;
                }
                _ => {
                    return Err(error(
                        "only context, addition and deletion lines are supported",
                    ));
                }
            }
            i += 1;
        }
        if removed != old_count || added != new_count {
            return Err(error("hunk counts do not match body"));
        }
        hunks += 1;
    }
    if hunks == 0 {
        return Err(error("patch contains no hunks"));
    }
    out.extend(lines[cursor..].iter().map(|s| s.to_string()));
    Ok(out.join("\n") + "\n")
}
