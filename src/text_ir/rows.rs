//! Revision-bound row lifecycle; native writers retain ownership of layout validation.
use super::*;

#[derive(Default)]
pub(super) struct Lifecycle {
    deletions: BTreeMap<String, EditOperation>,
}
impl Lifecycle {
    pub fn export(
        doc: &XgwxDocument,
        index: usize,
        iec: bool,
        text: &str,
    ) -> Result<(Self, String), StoreError> {
        let mut result = Self::default();
        let mut output = String::new();
        if !iec {
            return Ok((result, output));
        }
        let program = doc
            .program()
            .ladder_program(index)
            .unwrap()
            .map_err(unsupported)?;
        let rows = program
            .iec_row_frames()
            .ok_or_else(|| unsupported("unknown IEC rows"))?;
        let count = program
            .data
            .get(4..6)
            .map(|b| u16::from_le_bytes([b[0], b[1]]))
            .unwrap_or(0);
        let last = rows.iter().map(|r| r.row_index).max().unwrap_or(0);
        let occupied = rows.iter().map(|r| r.row_index).collect::<BTreeSet<_>>();
        // Blanks are absent row indices, not decoded zero-record row envelopes.
        for row in 0..count.min(16383) {
            if occupied.contains(&row) {
                continue;
            }
            let id = format!("@p{index}.blank{row}");
            let writable = row > 0 && row < last;
            output.push_str(&format!(
                "  blank_row {id} at row {row}{}\n",
                if writable { "" } else { " readonly" }
            ));
            if writable {
                result.deletions.insert(
                    id,
                    EditOperation {
                        name: "delete_iec_ld_blank_row".into(),
                        args: json!({"program_index":index,"blank_row_index":row}),
                    },
                );
            }
        }
        let parsed = parse(text)?;
        for row in rows.iter().filter(|r| r.record_count == 3) {
            let mut contact = None;
            let mut coil = None;
            for node in parsed.nodes.values() {
                if !matches!(node.position, Some(Position::Iec {group, row: r, ..}) if group == row.group_index && r == row.row_index)
                {
                    continue;
                }
                if let Body::Element {
                    family,
                    kind,
                    operand,
                } = &node.body
                {
                    if family == "contact" {
                        contact = Some((kind, operand));
                    }
                    if family == "coil" {
                        coil = Some((kind, operand));
                    }
                }
            }
            let (Some((ck, cv)), Some((ok, ov))) = (contact, coil) else {
                continue;
            };
            let operation = EditOperation {
                name: "delete_iec_ld_rung".into(),
                args: json!({"program_index":index,"row_index":row.row_index,"expected_contact_kind":iec_writer_kind("contact",ck)?,"expected_contact_variable":cv,"expected_coil_kind":iec_writer_kind("coil",ok)?,"expected_coil_variable":ov}),
            };
            let mut probe = doc.clone();
            if crate::operations::apply_operation(
                &mut probe,
                &operation.name,
                operation.args.clone(),
            )
            .is_err()
            {
                continue;
            }
            let id = format!("@p{index}.rung{}", row.row_index);
            output.push_str(&format!("  rung_row {id} at row {}\n", row.row_index));
            result.deletions.insert(id, operation);
        }
        Ok((result, output))
    }

    pub fn lower(
        &self,
        original: &str,
        changed: &str,
        index: usize,
        iec: bool,
    ) -> Result<Option<Vec<EditOperation>>, StoreError> {
        fn split(text: &str) -> Result<(Vec<String>, BTreeMap<String, Vec<String>>), StoreError> {
            let mut frozen = vec![];
            let mut rows = BTreeMap::new();
            let mut scope = vec![];
            for line in text.lines() {
                let t = lex(line)?;
                if matches!(
                    t.first().map(String::as_str),
                    Some("blank_row" | "rung_row")
                ) {
                    if scope.len() != 1
                        || scope[0] != "program"
                        || t.len() < 2
                        || !t[1].starts_with('@')
                    {
                        return Err(error(
                            "row lifecycle statements belong directly to the program block",
                        ));
                    }
                    if rows.insert(t[1].clone(), t).is_some() {
                        return Err(error("duplicate row lifecycle identity"));
                    }
                } else {
                    if t.last().map(String::as_str) == Some("{") {
                        scope.push(t[0].clone());
                    }
                    if t == ["}"] {
                        scope.pop();
                    }
                    frozen.push(line.into());
                }
            }
            Ok((frozen, rows))
        }
        let (before_text, before) = split(original)?;
        let (after_text, after) = split(changed)?;
        if before == after {
            return Ok(None);
        }
        if before_text != after_text {
            return Err(unsupported(
                "row lifecycle must be the sole mutation; retain the body and context unchanged",
            ));
        }
        let mut ops = vec![];
        for (id, old) in &before {
            match after.get(id) {
                Some(new) if old == new => (),
                Some(_) => {
                    return Err(unsupported(
                        "existing row lifecycle markers are immutable; remove a supported marker to delete its native row",
                    ));
                }
                None => ops.push(
                    self.deletions
                        .get(id)
                        .ok_or_else(|| unsupported("row marker is readonly"))?
                        .clone(),
                ),
            }
        }
        for (id, t) in &after {
            if before.contains_key(id) {
                continue;
            }
            if !id.starts_with("@new-") || id.len() <= 5 {
                return Err(error("new rows require a fresh @new-name identity"));
            }
            let row = t
                .get(4)
                .ok_or_else(|| error("missing row coordinate"))?
                .parse::<u32>()
                .map_err(error)?;
            let operation = if t.len() == 5
                && t[0] == "blank_row"
                && ((iec && t[2..4] == ["after", "row"]) || (!iec && t[2..4] == ["before", "y"]))
            {
                if iec {
                    EditOperation {
                        name: "insert_iec_ld_blank_row".into(),
                        args: json!({"program_index":index,"after_row_index":u16::try_from(row).map_err(error)?}),
                    }
                } else {
                    EditOperation {
                        name: "insert_ladder_row".into(),
                        args: json!({"program_index":index,"raw_y":row}),
                    }
                }
            } else if iec
                && t.len() == 11
                && t[0] == "rung_row"
                && t[2..4] == ["at", "row"]
                && t[5] == "contact"
                && t[8] == "coil"
            {
                EditOperation {
                    name: "insert_iec_ld_rung".into(),
                    args: json!({"program_index":index,"blank_row_index":u16::try_from(row).map_err(error)?,"contact_kind":iec_writer_kind("contact", &t[6])?,"contact_variable":string(&t[7])?,"coil_kind":iec_writer_kind("coil", &t[9])?,"coil_variable":string(&t[10])?}),
                }
            } else {
                return Err(error(
                    "expected blank_row @new-name after row N (IEC), before y N (XGK), or rung_row @new-name at row N contact KIND \"operand\" coil KIND \"operand\" (IEC)",
                ));
            };
            ops.push(operation);
        }
        if ops.len() != 1 {
            return Err(unsupported("apply one row lifecycle mutation per patch"));
        }
        Ok(Some(ops))
    }
}
