//! Semantic authoring at decoded IEC sites; offsets and expected values stay captured.
use super::*;
#[derive(Default)]
pub(super) struct Sites {
    contacts: BTreeMap<String, (EditOperation, bool)>,
    repairs: BTreeMap<String, EditOperation>,
    parallels: BTreeMap<String, EditOperation>,
}
impl Sites {
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
        for site in program
            .iec_leading_contact_insertion_sites()
            .unwrap_or_default()
        {
            let id = format!("@p{index}.leading{}", site.insertion_offset);
            output.push_str(&format!(
                "  contact_site {id} at(group={},row={},x=1) LEADING readonly\n",
                site.group_index, site.row_index
            ));
            result.contacts.insert(id,(EditOperation{name:"insert_iec_ld_leading_contact".into(),args:json!({"program_index":index,"insertion_offset":site.insertion_offset})},false));
        }
        for site in program.iec_no_contact_insertion_sites().unwrap_or_default() {
            let id = format!("@p{index}.contactwire{}", site.wire_offset);
            output.push_str(&format!(
                "  contact_site {id} at(group={},row={},x={}) WIRE end_x {} readonly\n",
                site.group_index, site.row_index, site.start_x, site.end_x
            ));
            result.contacts.insert(id,(EditOperation{name:"insert_iec_ld_contact".into(),args:json!({"program_index":index,"wire_offset":site.wire_offset,"expected_wire_start_x":site.start_x,"expected_wire_end_x":site.end_x})},true));
        }
        for site in program
            .iec_short_wire_contact_insertion_sites()
            .unwrap_or_default()
        {
            let id = format!("@p{index}.shortwire{}", site.wire_offset);
            output.push_str(&format!(
                "  contact_site {id} at(group={},row={},x={}) SHORT_WIRE readonly\n",
                site.group_index, site.row_index, site.raw_x
            ));
            result.contacts.insert(id,(EditOperation{name:"insert_iec_ld_short_wire_contact".into(),args:json!({"program_index":index,"wire_offset":site.wire_offset,"expected_raw_x":site.raw_x})},false));
        }
        for site in program
            .iec_horizontal_wire_repair_sites()
            .unwrap_or_default()
        {
            let id = format!("@p{index}.wiregap{}", site.insertion_offset);
            output.push_str(&format!(
                "  wire_gap {id} at(group={},row={},x={}) readonly\n",
                site.group_index, site.row_index, site.raw_x
            ));
            result.repairs.insert(id,EditOperation{name:"repair_iec_ld_horizontal_wire".into(),args:json!({"program_index":index,"insertion_offset":site.insertion_offset,"expected_raw_x":site.raw_x})});
        }
        let parsed = parse(text)?;
        for row in program
            .iec_row_frames()
            .unwrap_or_default()
            .iter()
            .filter(|r| r.record_count == 3)
        {
            let mut contact = None;
            let mut coil = None;
            for node in parsed.nodes.values() {
                if !matches!(node.position,Some(Position::Iec{group,row:r,..}) if group==row.group_index && r==row.row_index)
                {
                    continue;
                }
                if let Body::Element {
                    family,
                    kind,
                    operand,
                } = &node.body
                {
                    if family == "contact" && kind == "NO" {
                        contact = Some(operand);
                    }
                    if family == "coil" && kind == "OUT" {
                        coil = Some(operand);
                    }
                }
            }
            let (Some(contact), Some(coil)) = (contact, coil) else {
                continue;
            };
            let operation = EditOperation {
                name: "insert_iec_ld_parallel_contact_kind".into(),
                args: json!({"program_index":index,"row_index":row.row_index,"expected_contact_variable":contact,"expected_coil_variable":coil,"parallel_kind":"NO","parallel_variable":contact}),
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
            let id = format!("@p{index}.parallel{}", row.row_index);
            output.push_str(&format!(
                "  parallel_site {id} at(group={},row={},x=1) readonly\n",
                row.group_index, row.row_index
            ));
            result.parallels.insert(id, operation);
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
        fn split(text: &str) -> Result<(Vec<String>, Vec<Vec<String>>), StoreError> {
            let mut rest = vec![];
            let mut additions = vec![];
            let mut scope = vec![];
            for line in text.lines() {
                let t = lex(line)?;
                if matches!(
                    t.first().map(String::as_str),
                    Some("contact_insert" | "wire_restore" | "parallel_contact" | "comment_row")
                ) {
                    if scope.len() != 1 || scope[0] != "program" {
                        return Err(error(
                            "IEC authoring statements belong directly to the program block",
                        ));
                    }
                    additions.push(t);
                } else {
                    if t.last().map(String::as_str) == Some("{") {
                        scope.push(t[0].clone());
                    }
                    if t == ["}"] {
                        scope.pop();
                    }
                    rest.push(line.to_string());
                }
            }
            Ok((rest, additions))
        }
        let (before, before_added) = split(original)?;
        let (after, added) = split(changed)?;
        if added == before_added {
            return Ok(None);
        }
        if !iec || before != after || !before_added.is_empty() || added.len() != 1 {
            return Err(unsupported(
                "one IEC site edit must be the sole mutation; retain all source and site statements unchanged",
            ));
        }
        let t = &added[0];
        if !t
            .get(1)
            .is_some_and(|id| id.starts_with("@new-") && id.len() > 5)
        {
            return Err(error("IEC authoring requires a fresh @new-name identity"));
        }
        let mut op = match t[0].as_str() {
            "contact_insert" | "parallel_contact"
                if (t.len() == 8 || t.len() == 10)
                    && t[2] == "site"
                    && t[4] == "kind"
                    && t[6] == "operand" =>
            {
                let kind = iec_writer_kind("contact", &t[5])?;
                let variable = string(&t[7])?;
                if t[0] == "parallel_contact" {
                    if t.len() != 8 {
                        return Err(error("parallel contact has no x override"));
                    }
                    let mut op = self
                        .parallels
                        .get(&t[3])
                        .ok_or_else(|| error("unknown captured parallel site"))?
                        .clone();
                    op.args["parallel_kind"] = json!(kind);
                    op.args["parallel_variable"] = json!(variable);
                    op
                } else {
                    let (operation, needs_x) = self
                        .contacts
                        .get(&t[3])
                        .ok_or_else(|| error("unknown captured contact insertion site"))?;
                    let mut op = operation.clone();
                    if *needs_x {
                        if t.len() != 10 || t[8] != "x" {
                            return Err(error("WIRE contact site requires x N"));
                        }
                        op.args["raw_x"] = json!(t[9].parse::<u8>().map_err(error)?);
                    } else if t.len() != 8 {
                        return Err(error(
                            "LEADING/SHORT_WIRE contact sites have a fixed native cell",
                        ));
                    }
                    op.args["contact_kind"] = json!(kind);
                    op.args["variable"] = json!(variable);
                    op
                }
            }
            "wire_restore" if t.len() == 4 && t[2] == "site" => self
                .repairs
                .get(&t[3])
                .ok_or_else(|| error("unknown captured wire gap"))?
                .clone(),
            "comment_row" if t.len() == 7 && t[2..4] == ["at", "row"] && t[5] == "text" => {
                EditOperation {
                    name: "insert_iec_ld_comment".into(),
                    args: json!({"program_index":index,"destination_row_index":t[4].parse::<u16>().map_err(error)?,"text":string(&t[6])?}),
                }
            }
            _ => {
                return Err(error(
                    "invalid IEC contact_insert, parallel_contact, wire_restore or comment_row statement",
                ));
            }
        };
        op.args["program_index"] = json!(index);
        Ok(Some(vec![op]))
    }
}
