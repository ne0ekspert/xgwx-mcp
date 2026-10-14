#[path = "../src/test_support.rs"]
mod test_support;
use std::fs;
use test_support::fixture_root;
use xgwx::XgwxDocument;
use xgwx_mcp::{
    storage::{FileStore, revision},
    text_ir::{Projection, apply_unified_patch},
};
fn fixture(name: &str) -> XgwxDocument {
    XgwxDocument::from_path(fixture_root().join("fixtures").join(name)).unwrap()
}
fn replace_patch(text: &str, old: &str, new: &str) -> String {
    let lines = text.lines().collect::<Vec<_>>();
    let i = lines.iter().position(|l| *l == old).unwrap();
    format!(
        "--- ladder\n+++ ladder\n@@ -{},1 +{},1 @@\n-{old}\n+{new}\n",
        i + 1,
        i + 1
    )
}
#[test]
fn export_fixture_inventory() {
    for name in [
        "elements.xgwx",
        "program-create/xgk-ld-generated.xgwx",
        "program-create/xgi-ld-generated.xgwx",
        "ladder-wire-delete/iec-horizontal-native.xgwx",
    ] {
        let doc = fixture(name);
        let index = doc
            .program()
            .ladder_programs()
            .iter()
            .position(|p| p.is_ok())
            .unwrap();
        let ir = Projection::export(&doc, "revision", index).unwrap();
        assert!(ir.text.contains(&format!("program @p{index}")));
        assert!(ir.lower_patch("").unwrap().is_empty());
    }
}
// Synthetic native payloads derived from public fixtures, never user project data.
fn with_iec_field(
    mut doc: XgwxDocument,
    offset: usize,
    end: usize,
    text: &str,
    kind: u8,
) -> XgwxDocument {
    use base64::Engine;
    use std::io::Write;
    let program = doc.program().ladder_program(0).unwrap().unwrap();
    let mut data = program.data.clone();
    let units = text.encode_utf16().collect::<Vec<_>>();
    let mut field = vec![0xff, 0xfe, 0xff, u8::try_from(units.len()).unwrap()];
    field.extend(units.iter().flat_map(|unit| unit.to_le_bytes()));
    if kind != 0 {
        data[offset + 1] = kind;
        let flags = match kind {
            0x3f => [0, 0, 0x20, 0, 0, 0],
            0x0e => [1, 0, 0x20, 0, 0, 0],
            0x46 => [0; 6],
            _ => panic!("unsupported synthetic kind"),
        };
        data[offset + 9..offset + 15].copy_from_slice(&flags);
        if kind == 0x3f {
            field.extend([0; 8]);
        }
    }
    data.splice(offset + 15..end, field);
    let encoded = if program.compressed {
        let mut encoder = bzip2::write::BzEncoder::new(Vec::new(), bzip2::Compression::best());
        encoder.write_all(&data).unwrap();
        encoder.finish().unwrap()
    } else {
        data
    };
    let parsed = roxmltree::Document::parse(doc.container().xml()).unwrap();
    let node = parsed
        .descendants()
        .find(|n| n.has_tag_name("Program"))
        .unwrap()
        .descendants()
        .find(|n| n.has_tag_name("ProgramData"))
        .unwrap();
    let range = node.children().find(|n| n.is_text()).unwrap().range();
    let mut xml = doc.container().xml().to_owned();
    xml.replace_range(
        range,
        &base64::engine::general_purpose::STANDARD.encode(encoded),
    );
    doc.container_mut().replace_xml(xml).unwrap();
    XgwxDocument::parse(&doc.to_bytes().unwrap()).unwrap()
}

#[test]
fn iec_unicode_and_empty_operands_and_comments_are_semantic_but_readonly() {
    for (value, comment) in [
        ("한글접점", false),
        ("", false),
        ("한글 주석\n두 번째 줄", true),
        ("", true),
    ] {
        let doc = fixture("ladder-wire-delete/iec-horizontal-native.xgwx");
        let p = doc.program().ladder_program(0).unwrap().unwrap();
        let r = p
            .iec_record_frames()
            .unwrap()
            .into_iter()
            .find(|r| {
                matches!(r.kind, xgwx::IecRecordKind::Contact(_)) && p.data[r.offset + 5] == 1
            })
            .unwrap();
        let doc = with_iec_field(doc, r.offset, r.end, value, if comment { 0x3f } else { 0 });
        let before = doc.to_bytes().unwrap();
        let ir = Projection::export(&doc, "revision", 0).unwrap();
        let id = format!("@p0.e{}", r.offset);
        let line = ir
            .text
            .lines()
            .find(|line| line.split_whitespace().nth(1) == Some(id.as_str()))
            .unwrap();
        assert!(
            line.trim_start()
                .starts_with(if comment { "annotation " } else { "contact " }),
            "{line}"
        );
        assert!(
            line.contains(&serde_json::to_string(value).unwrap()),
            "{line}"
        );
        assert!(line.contains("decode \"decoded\""));
        assert!(line.contains("reason \"writer_string_unavailable\""));
        assert!(line.ends_with(" readonly"));
        assert!(ir.lower_patch("").unwrap().is_empty());
        for new in [
            line.replace("readonly", "editable"),
            line.replace("decoded", "unavailable"),
            line.replace(&serde_json::to_string(value).unwrap(), "\"Changed\""),
        ] {
            assert!(
                ir.lower_patch(&replace_patch(&ir.text, line, &new))
                    .is_err()
            );
        }
        assert_eq!(doc.to_bytes().unwrap(), before);
        // A separate supported edit must preserve the readonly native field.
        if !comment {
            let other = ir
                .text
                .lines()
                .find(|l| {
                    (l.trim_start().starts_with("contact ") || l.trim_start().starts_with("coil "))
                        && !l.ends_with("readonly")
                })
                .unwrap();
            let patch = replace_patch(
                &ir.text,
                other,
                &other.replace(" NO ", " NC ").replace(" OUT ", " INVERSE "),
            );
            let mut edited = doc.clone();
            for op in ir.lower_patch(&patch).unwrap() {
                xgwx_mcp::operations::apply_operation(&mut edited, &op.name, op.args).unwrap();
            }
            let edited = XgwxDocument::parse(&edited.to_verified_bytes().unwrap()).unwrap();
            let exported = Projection::export(&edited, "revision", 0).unwrap();
            assert!(exported.text.lines().any(|l| l == line));
        }
    }
}

#[test]
fn iec_unicode_function_arguments_keep_pin_binding_without_enabling_edits() {
    let doc = fixture("function-output-wires/base.xgwx");
    let p = doc.program().ladder_program(0).unwrap().unwrap();
    let link = p
        .iec_function_operand_links()
        .unwrap()
        .into_iter()
        .find(|l| !l.is_output)
        .unwrap();
    let r = p
        .iec_record_frames()
        .unwrap()
        .into_iter()
        .find(|r| r.offset == link.record_offset)
        .unwrap();
    let doc = with_iec_field(doc, r.offset, r.end, "한글입력", 0);
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let id = format!("@p0.e{}", r.offset);
    let line = ir
        .text
        .lines()
        .find(|l| l.split_whitespace().nth(1) == Some(id.as_str()))
        .unwrap();
    assert!(line.trim_start().starts_with("argument "), "{line}");
    assert!(line.contains(&format!(
        "target \"@p0.f{}\" pin {}",
        link.target_record_offset, link.ordinal
    )));
    assert!(line.contains("value \"한글입력\" decode \"decoded\""));
    assert!(line.ends_with(" readonly"));
    assert!(ir.lower_patch("").unwrap().is_empty());
    assert!(
        ir.lower_patch(&replace_patch(
            &ir.text,
            line,
            &line.replace("한글입력", "Changed")
        ))
        .is_err()
    );
}

#[test]
fn iec_readonly_coil_and_unbound_argument_expose_text_and_freeze_metadata() {
    for kind in [0x0e, 0x46] {
        let doc = fixture("ladder-wire-delete/iec-horizontal-native.xgwx");
        let p = doc.program().ladder_program(0).unwrap().unwrap();
        let r = p
            .iec_record_frames()
            .unwrap()
            .into_iter()
            .find(|r| matches!(r.kind, xgwx::IecRecordKind::Contact(_)))
            .unwrap();
        let doc = with_iec_field(doc, r.offset, r.end, "한글값", kind);
        let ir = Projection::export(&doc, "revision", 0).unwrap();
        let id = format!("@p0.e{}", r.offset);
        let line = ir
            .text
            .lines()
            .find(|l| l.split_whitespace().nth(1) == Some(id.as_str()))
            .unwrap();
        if kind == 0x0e {
            assert!(line.trim_start().starts_with("coil "));
            assert!(line.contains(" OUT \"한글값\""));
        } else {
            assert!(line.contains("kind \"FunctionOperand\" value \"한글값\""));
            assert!(line.contains("reason \"function_binding_unavailable\""));
            assert!(!line.contains("target"));
        }
        assert!(line.contains("decode \"decoded\""));
        assert!(line.ends_with(" readonly"));
        assert!(ir.lower_patch("").unwrap().is_empty());
        assert!(
            ir.lower_patch(&replace_patch(
                &ir.text,
                line,
                &line.replace("한글값", "Changed")
            ))
            .is_err()
        );
    }
}

#[test]
fn iec_call_pins_and_bindings_are_explicit_and_readonly() {
    let doc = fixture("function-output-wires/base.xgwx");
    let p = doc.program().ladder_program(0).unwrap().unwrap();
    let functions = p.iec_function_blocks().unwrap();
    let refs = p.iec_function_references().unwrap();
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    assert_eq!(
        ir.text
            .lines()
            .filter(|l| l.trim_start().starts_with("call "))
            .count(),
        functions.len()
    );
    assert_eq!(
        ir.text
            .lines()
            .filter(|l| l.trim_start().starts_with("pin "))
            .count(),
        functions.iter().map(|f| f.pins.len() + 2).sum::<usize>()
    );
    assert_eq!(
        ir.text
            .lines()
            .filter(|l| l.trim_start().starts_with("binding "))
            .count(),
        refs.len()
    );
    for r in refs {
        let line = ir
            .text
            .lines()
            .find(|l| l.starts_with(&format!("  binding @p0.b{} ", r.record_offset)))
            .unwrap();
        assert!(line.contains("status \"resolved\""), "{line}");
        assert!(line.contains(&format!("reference \"@p0.e{}\"", r.record_offset)));
        assert!(line.contains(&format!("function \"@p0.f{}\"", r.target_record_offset)));
    }
    for prefix in ["call ", "pin ", "binding "] {
        let line = ir
            .text
            .lines()
            .find(|l| l.trim_start().starts_with(prefix))
            .unwrap();
        assert!(line.ends_with(" readonly"));
        assert!(
            ir.lower_patch(&replace_patch(
                &ir.text,
                line,
                &line.replace("readonly", "editable")
            ))
            .is_err()
        );
    }
    assert!(ir.lower_patch("").unwrap().is_empty());
}

#[test]
fn operand_metadata_is_frozen_and_recomputed_after_supported_edits() {
    let doc = fixture("elements.xgwx");
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let line = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("contact "))
        .unwrap();
    let owner = line.split_whitespace().nth(1).unwrap();
    let metadata = ir
        .text
        .lines()
        .find(|l| l.starts_with(&format!("  operand {owner}.operand0 ")))
        .unwrap();
    assert!(metadata.ends_with(" readonly"));
    for changed in [
        metadata.replace("readonly", "editable"),
        metadata.replace("category", "altered_category"),
    ] {
        assert!(
            ir.lower_patch(&replace_patch(&ir.text, metadata, &changed))
                .is_err()
        );
    }
    let program = doc.program().ladder_program(0).unwrap().unwrap();
    let cell = program
        .structure
        .rungs
        .iter()
        .flat_map(|r| &r.cells)
        .find(|c| format!("@p0.e{}", c.offset) == owner)
        .unwrap();
    let old = cell.operands.first().unwrap_or(&cell.value);
    let changed = line.replace(&serde_json::to_string(old).unwrap(), "\"M00999\"");
    let mut doc = doc;
    for op in ir
        .lower_patch(&replace_patch(&ir.text, line, &changed))
        .unwrap()
    {
        xgwx_mcp::operations::apply_operation(&mut doc, &op.name, op.args).unwrap();
    }
    let doc = XgwxDocument::parse(&doc.to_verified_bytes().unwrap()).unwrap();
    let fresh = Projection::export(&doc, "revision", 0).unwrap();
    assert!(
        fresh
            .text
            .lines()
            .any(|l| l.trim_start().starts_with("operand ") && l.contains("text \"M00999\""))
    );
}

#[test]
fn native_connection_endpoints_and_provenance_resolve_to_exported_ids() {
    for fixture_name in [
        "function-output-wires/base.xgwx",
        "ladder-wire-delete/iec-horizontal-native.xgwx",
    ] {
        let doc = fixture(fixture_name);
        let p = doc.program().ladder_program(0).unwrap().unwrap();
        let graph = p.iec_circuit_layout().unwrap();
        let ir = Projection::export(&doc, "revision", 0).unwrap();
        let ids = ir
            .text
            .lines()
            .filter_map(|l| l.split_whitespace().nth(1))
            .filter(|id| id.starts_with('@'))
            .collect::<std::collections::BTreeSet<_>>();
        let edges = ir
            .text
            .lines()
            .filter(|l| l.trim_start().starts_with("edge "))
            .collect::<Vec<_>>();
        assert_eq!(edges.len(), graph.edges.len());
        for line in edges {
            let t = line
                .split_whitespace()
                .map(str::to_owned)
                .collect::<Vec<_>>();
            for field in [
                "connection",
                "from",
                "to",
                "source",
                "from_terminal",
                "to_terminal",
            ] {
                let value = &t[t.iter().position(|v| v == field).unwrap() + 1];
                if value != "none" {
                    let id = serde_json::from_str::<String>(value).unwrap();
                    assert!(ids.contains(id.as_str()), "dangling {field}: {line}");
                }
            }
        }
        let junctions = ir
            .text
            .lines()
            .filter(|l| l.trim_start().starts_with("pin_junction "))
            .collect::<Vec<_>>();
        assert_eq!(junctions.len(), graph.function_bindings.len());
        for line in junctions {
            let t = line
                .split_whitespace()
                .map(str::to_owned)
                .collect::<Vec<_>>();
            for field in ["binding", "pin", "junction"] {
                let value = &t[t.iter().position(|v| v == field).unwrap() + 1];
                assert_ne!(value, "none");
                let id = serde_json::from_str::<String>(value).unwrap();
                assert!(ids.contains(id.as_str()), "{line}");
            }
        }
        for prefix in ["edge ", "junction ", "terminal "] {
            let Some(line) = ir.text.lines().find(|l| l.trim_start().starts_with(prefix)) else {
                assert_eq!(prefix, "terminal ");
                continue;
            };
            assert!(
                ir.lower_patch(&replace_patch(
                    &ir.text,
                    line,
                    &line.replace("readonly", "editable")
                ))
                .is_err()
            );
        }
        assert!(ir.lower_patch("").unwrap().is_empty());
    }
}

#[test]
fn context_metadata_is_frozen_and_does_not_claim_runtime_order() {
    let doc = fixture("empty-projects/new-xgi.xgwx");
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    assert!(ir.text.contains("language \"LD\""));
    assert!(ir.text.contains("execution_order \"not_decoded\""));
    for prefix in ["context ", "cpu ", "task ", "program_context "] {
        let line = ir
            .text
            .lines()
            .find(|l| l.trim_start().starts_with(prefix))
            .unwrap();
        assert!(line.ends_with(" readonly"));
        assert!(
            ir.lower_patch(&replace_patch(
                &ir.text,
                line,
                &line.replace("readonly", "editable")
            ))
            .is_err()
        );
    }
}

#[test]
fn strict_unified_diff_checks_both_coordinates_and_counts() {
    let text = "a\nb\nc\n";
    assert_eq!(
        apply_unified_patch(text, "@@ -2 +2 @@\n-b\n+B\n").unwrap(),
        "a\nB\nc\n"
    );
    assert_eq!(
        apply_unified_patch(text, "@@ -0,0 +1 @@\n+start\n@@ -3 +4 @@\n-c\n+C\n").unwrap(),
        "start\na\nb\nC\n"
    );
    for patch in [
        "@@ -2 +2 @@\n-c\n+B\n",
        "@@ -2,2 +2 @@\n-b\n+B\n",
        "@@ -2 +3 @@\n-b\n+B\n",
        "@@ -99 +99 @@\n-b\n+B\n",
        "--- a\n+++ b\n",
        "@@ -2 +2 @@\n-b\n+B\n@@ -1 +1 @@\n-a\n+A\n",
    ] {
        assert!(apply_unified_patch(text, patch).is_err(), "{patch}");
    }
}
#[test]
fn xgk_contact_kind_operand_and_descriptions_transaction() {
    let doc = fixture("elements.xgwx");
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("p.xgwx");
    let original = doc.to_verified_bytes().unwrap();
    fs::write(&path, &original).unwrap();
    let rev = revision(&original);
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let old = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("contact ") && l.contains(" NO "))
        .unwrap();
    let new = old.replace(" NO ", " NC ");
    let patch = replace_patch(&ir.text, old, &new);
    let preview = store.apply_patch("p.xgwx", &rev, 0, &patch, true).unwrap();
    assert_eq!(preview["changed"], true);
    assert_eq!(fs::read(&path).unwrap(), original);
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    let saved = store.apply_patch("p.xgwx", &rev, 0, &patch, false).unwrap();
    assert_eq!(
        fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
        original
    );
    let (_, after, doc) = store.read("p.xgwx").unwrap();
    let next = Projection::export(&doc, &after, 0).unwrap();
    assert!(next.text.lines().any(|l| l.contains(" NC ")));
    assert_eq!(
        store
            .apply_patch("p.xgwx", &rev, 0, &patch, false)
            .unwrap_err()
            .code,
        "stale_revision"
    );
    let no_op = store.apply_patch("p.xgwx", &after, 0, "", false).unwrap();
    assert_eq!(no_op["changed"], false);
    assert!(no_op["backup_path"].is_null());
}
#[test]
fn unsupported_move_topology_identity_and_rollback() {
    let doc = fixture("elements.xgwx");
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("p.xgwx");
    let original = doc.to_verified_bytes().unwrap();
    fs::write(&path, &original).unwrap();
    let rev = revision(&original);
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let old = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("contact "))
        .unwrap();
    for new in [
        old.replace("x=1,", "x=4,"),
        old.replace("contact ", "coil "),
        old.replace("@p0.e", "@unknown.e"),
    ] {
        if new == old {
            continue;
        }
        assert!(
            store
                .apply_patch(
                    "p.xgwx",
                    &rev,
                    0,
                    &replace_patch(&ir.text, old, &new),
                    false
                )
                .is_err()
        );
        assert_eq!(fs::read(&path).unwrap(), original);
    }
    let wire = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("wire "))
        .unwrap();
    assert!(
        ir.lower_patch(&replace_patch(
            &ir.text,
            wire,
            &wire.replace("HORIZONTAL", "DIAGONAL")
        ))
        .is_err()
    );
    let invalid = old.replace(" NO ", " INVALID ");
    assert!(
        store
            .apply_patch(
                "p.xgwx",
                &rev,
                0,
                &replace_patch(&ir.text, old, &invalid),
                true
            )
            .is_err()
    );
}
#[test]
fn iec_contacts_use_native_group_row_anchor_and_string_offsets() {
    let doc = fixture("ladder-wire-delete/iec-horizontal-native.xgwx");
    let dir = tempfile::tempdir().unwrap();
    let original = doc.to_verified_bytes().unwrap();
    fs::write(dir.path().join("p.xgwx"), &original).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let rev = revision(&original);
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    assert_eq!(ir.dialect, "IEC");
    let old = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("contact ") && l.contains(" NO "))
        .unwrap();
    assert!(old.contains("at(group="));
    let new = old.replace(" NO ", " NC ");
    let saved = store
        .apply_patch(
            "p.xgwx",
            &rev,
            0,
            &replace_patch(&ir.text, old, &new),
            false,
        )
        .unwrap();
    assert_eq!(saved["changed"], true);
    let (_, rev, doc) = store.read("p.xgwx").unwrap();
    let next = Projection::export(&doc, &rev, 0).unwrap();
    assert!(next.text.lines().any(|l| l.contains(" NC ")));
}
fn full_patch(before: &str, after: &str) -> String {
    format!(
        "@@ -1,{} +1,{} @@\n{}{}",
        before.lines().count(),
        after.lines().count(),
        before
            .lines()
            .map(|l| format!("-{l}\n"))
            .collect::<String>(),
        after.lines().map(|l| format!("+{l}\n")).collect::<String>()
    )
}
fn apply(doc: &mut XgwxDocument, name: &str, args: serde_json::Value) {
    xgwx_mcp::operations::apply_operation(doc, name, args).unwrap();
}
#[test]
fn native_rung_output_comments_multiline_creation_deletion_and_preservation() {
    use serde_json::json;
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    let untouched = doc.program().ladder_program(0).unwrap().unwrap().data;
    apply(
        &mut doc,
        "create_program",
        json!({"patch":{"name":"TextIR","language":"LD","object_id":"abcdef01-1234-4567-89ab-0123456789ab","symbol_id":"abcdef02-1234-4567-89ab-0123456789ab"}}),
    );
    apply(
        &mut doc,
        "edit_ladder_cell",
        json!({"program_index":1,"edit":{"raw_y":0,"column":0,"expected":null,"replacement":{"kind":"NormallyOpen","operand":"M00000"}}}),
    );
    apply(
        &mut doc,
        "edit_ladder_cell",
        json!({"program_index":1,"edit":{"raw_y":0,"column":9,"expected":null,"replacement":{"kind":"Output","operand":"M00001"}}}),
    );
    apply(
        &mut doc,
        "edit_ladder_comment",
        json!({"program_index":1,"edit":{"kind":"Output","raw_y":0,"expected":null,"replacement":"Output note"}}),
    );
    apply(
        &mut doc,
        "edit_ladder_comment",
        json!({"program_index":1,"edit":{"kind":"Rung","raw_y":0,"expected":null,"replacement":"Rung note"}}),
    );
    let dir = tempfile::tempdir().unwrap();
    let bytes = doc.to_verified_bytes().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, 1).unwrap();
    assert!(ir.text.contains("at(x=97,y=4) OUTPUT"));
    let edited = ir
        .text
        .replace(
            "\"Rung note\"",
            "\"설명\\nSecond line with \\\"quotes\\\"\"",
        )
        .replace("\"Output note\"", "\"Output documentation grows\"");
    store
        .apply_patch("p.xgwx", &rev, 1, &full_patch(&ir.text, &edited), false)
        .unwrap();
    let (_, rev, doc) = store.read("p.xgwx").unwrap();
    let ir = Projection::export(&doc, &rev, 1).unwrap();
    let comments = doc.program().ladder_program(1).unwrap().unwrap().structure;
    assert_eq!(
        comments.rung_comments[0].text,
        "설명\nSecond line with \"quotes\""
    );
    assert_eq!(
        comments.output_comments[0].text,
        "Output documentation grows"
    );
    assert_eq!(
        doc.program().ladder_program(0).unwrap().unwrap().data,
        untouched
    );
    let rung_line = ir.text.lines().find(|l| l.contains(" RUNG ")).unwrap();
    let edited = ir
        .text
        .lines()
        .filter(|l| *l != rung_line)
        .map(|l| format!("{l}\n"))
        .collect::<String>();
    store
        .apply_patch("p.xgwx", &rev, 1, &full_patch(&ir.text, &edited), false)
        .unwrap();
    let (_, rev, doc) = store.read("p.xgwx").unwrap();
    assert!(
        doc.program()
            .ladder_program(1)
            .unwrap()
            .unwrap()
            .structure
            .rung_comments
            .is_empty()
    );
    let ir = Projection::export(&doc, &rev, 1).unwrap();
    let existing = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("contact "))
        .unwrap();
    let edited = ir.text.replace(
        existing,
        &format!("{existing}\n    comment @new-rung at(x=1,y=0) RUNG \"Created through patch\""),
    );
    store
        .apply_patch("p.xgwx", &rev, 1, &full_patch(&ir.text, &edited), false)
        .unwrap();
    let (_, _, doc) = store.read("p.xgwx").unwrap();
    assert_eq!(
        doc.program()
            .ladder_program(1)
            .unwrap()
            .unwrap()
            .structure
            .rung_comments[0]
            .text,
        "Created through patch"
    );
    assert_eq!(
        doc.program().ladder_program(0).unwrap().unwrap().data,
        untouched
    );
}
#[test]
fn symbol_description_changes_and_writer_failure_rollback() {
    let doc = fixture("elements.xgwx");
    let dir = tempfile::tempdir().unwrap();
    let bytes = doc.to_verified_bytes().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let rev = revision(&bytes);
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let old = ir.text.lines().find(|l| l.starts_with("symbol ")).unwrap();
    let new = format!(
        "{}description \"Shared output description\"",
        old.split("description ").next().unwrap()
    );
    let result = store
        .apply_patch("p.xgwx", &rev, 0, &replace_patch(&ir.text, old, &new), true)
        .unwrap();
    assert_eq!(result["results"][0]["name"], "update_variable");
    let contact = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("contact ") && l.contains(" NO "))
        .unwrap();
    let invalid = format!("{}\"INVALID DEVICE\"", contact.split('"').next().unwrap());
    let edited = ir.text.replace(old, &new).replace(contact, &invalid);
    let error = store
        .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &edited), false)
        .unwrap_err();
    assert_eq!(error.code, "operation_failed");
    assert_eq!(error.operation_index, Some(1));
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    store
        .apply_patch(
            "p.xgwx",
            &rev,
            0,
            &replace_patch(&ir.text, old, &new),
            false,
        )
        .unwrap();
    let (_, _, changed) = store.read("p.xgwx").unwrap();
    assert_eq!(
        changed.variable().variables().unwrap()[0]
            .description
            .as_deref(),
        Some("Shared output description")
    );
    assert_eq!(
        changed.program().ladder_program(0).unwrap().unwrap().data,
        doc.program().ladder_program(0).unwrap().unwrap().data
    );
}
#[test]
fn iec_multiple_resizing_operands_kinds_and_local_descriptions() {
    use serde_json::json;
    let mut doc = fixture("function-output-wires/base.xgwx");
    {
        let replacement_xml = doc.container().xml().replace(
            "<Symbols Count=\"0\"></Symbols>",
            "<Symbols Count=\"0\"> </Symbols>",
        );
        doc.container_mut().replace_xml(replacement_xml).unwrap();
    };
    let mut doc = XgwxDocument::parse(&doc.to_bytes().unwrap()).unwrap();
    for name in [
        "Input",
        "InputWithLongerName",
        "Output",
        "OutputWithLongerName",
    ] {
        apply(
            &mut doc,
            "insert_iec_local_symbol",
            json!({"program_index":0,"name":name,"data_type":"BOOL","description":"Initial"}),
        );
    }
    let last = doc
        .program()
        .ladder_program(0)
        .unwrap()
        .unwrap()
        .iec_row_frames()
        .unwrap()
        .iter()
        .map(|r| r.row_index)
        .max()
        .unwrap()
        + 2;
    apply(
        &mut doc,
        "insert_iec_ld_linear_rung",
        json!({"program_index":0,"blank_row_index":last,"contact_variable":"Input","coil_variable":"Output"}),
    );
    let dir = tempfile::tempdir().unwrap();
    let bytes = doc.to_verified_bytes().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let mut edited = ir.text.clone();
    for line in ir.text.lines() {
        if line.trim_start().starts_with("contact ") && line.ends_with("\"Input\"") {
            edited = edited.replace(
                line,
                &line
                    .replace(" NO ", " NC ")
                    .replace("\"Input\"", "\"InputWithLongerName\""),
            );
        } else if line.trim_start().starts_with("coil ") && line.ends_with("\"Output\"") {
            edited = edited.replace(
                line,
                &line
                    .replace(" OUT ", " SET ")
                    .replace("\"Output\"", "\"OutputWithLongerName\""),
            );
        } else if line.trim_start().starts_with("symbol @p0.s") {
            edited = edited.replace(
                line,
                &line.replace(
                    "description \"Initial\"",
                    "description \"Longer declaration description\"",
                ),
            );
        }
    }
    let result = store
        .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &edited), false)
        .unwrap();
    assert_eq!(result["changed"], true);
    let (_, rev, doc) = store.read("p.xgwx").unwrap();
    let next = Projection::export(&doc, &rev, 0).unwrap();
    assert!(
        next.text
            .lines()
            .any(|l| l.trim_start().starts_with("contact ")
                && l.ends_with(" NC \"InputWithLongerName\""))
    );
    assert!(next.text.lines().any(
        |l| l.trim_start().starts_with("coil ") && l.ends_with(" SET \"OutputWithLongerName\"")
    ));
    assert!(
        doc.variable().iec_local_symbols()[0]
            .as_ref()
            .unwrap()
            .iter()
            .all(|s| s.description.as_deref() == Some("Longer declaration description"))
    );
}
#[test]
fn competing_patch_requests_have_one_commit() {
    let doc = fixture("elements.xgwx");
    let dir = tempfile::tempdir().unwrap();
    let bytes = doc.to_verified_bytes().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let old = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("contact ") && l.contains(" NO "))
        .unwrap();
    let patch = replace_patch(&ir.text, old, &old.replace(" NO ", " NC "));
    let start = std::sync::Arc::new(std::sync::Barrier::new(2));
    let mut tasks = vec![];
    for _ in 0..2 {
        let store = FileStore::new(dir.path().into(), vec![]).unwrap();
        let rev = rev.clone();
        let patch = patch.clone();
        let start = start.clone();
        tasks.push(std::thread::spawn(move || {
            start.wait();
            store.apply_patch("p.xgwx", &rev, 0, &patch, false)
        }));
    }
    let results = tasks
        .into_iter()
        .map(|t| t.join().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(results.iter().filter(|r| r.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter_map(|r| r.as_ref().err())
            .next()
            .unwrap()
            .code,
        "stale_revision"
    );
    assert_eq!(
        fs::read_dir(dir.path())
            .unwrap()
            .filter(|e| e
                .as_ref()
                .unwrap()
                .file_name()
                .to_string_lossy()
                .contains("backup"))
            .count(),
        1
    );
}

#[test]
fn native_output_annotation_creation_is_separate_from_symbol_description() {
    use serde_json::json;
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    apply(
        &mut doc,
        "edit_ladder_cell",
        json!({"program_index":0,"edit":{"raw_y":0,"column":9,"expected":null,"replacement":{"kind":"Output","operand":"M00001"}}}),
    );
    let dir = tempfile::tempdir().unwrap();
    let original = doc.to_verified_bytes().unwrap();
    fs::write(dir.path().join("p.xgwx"), &original).unwrap();
    let rev = revision(&original);
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let line = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("coil "))
        .unwrap();
    let edited = ir.text.replace(
        line,
        &format!("{line}\n    comment @new-output at(x=97,y=0) OUTPUT \"Output annotation\""),
    );
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    store
        .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &edited), false)
        .unwrap();
    let (_, rev, changed) = store.read("p.xgwx").unwrap();
    let structure = changed
        .program()
        .ladder_program(0)
        .unwrap()
        .unwrap()
        .structure;
    assert_eq!(structure.output_comments[0].text, "Output annotation");
    assert!(structure.rung_comments.is_empty());
    assert_eq!(
        changed.variable().variables().unwrap(),
        doc.variable().variables().unwrap()
    );
    let ir = Projection::export(&changed, &rev, 0).unwrap();
    let line = ir.text.lines().find(|l| l.contains(" OUTPUT ")).unwrap();
    let edited = ir
        .text
        .lines()
        .filter(|l| *l != line)
        .map(|l| format!("{l}\n"))
        .collect::<String>();
    assert_eq!(
        ir.lower_patch(&full_patch(&ir.text, &edited))
            .unwrap_err()
            .code,
        "unsupported_ir_edit"
    );
}
#[test]
fn malformed_strings_coordinates_and_duplicate_or_nested_identities_fail() {
    let doc = fixture("elements.xgwx");
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let line = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("contact "))
        .unwrap();
    for changed in [
        format!("{line}\n{line}"),
        line.replace("x=1,", "x=999,"),
        line.replace("at(", "at(bogus=0,"),
        format!("{line} \"unterminated"),
        format!("program @nested \"N\" dialect XGK object \"\" {{\n{line}\n}}"),
    ] {
        let patch = full_patch(&ir.text, &ir.text.replace(line, &changed));
        assert!(ir.lower_patch(&patch).is_err(), "{patch}");
    }
}
fn captured_iec_comment_fixture(mut doc: XgwxDocument) -> XgwxDocument {
    // Fixture-derived native record: change a captured contact into the known
    // comment grammar, including its trailing eight-byte envelope. This is a
    // test input only; production patches never rewrite binary records.
    use base64::Engine;
    use std::io::Write;
    let program = doc.program().ladder_program(0).unwrap().unwrap();
    let record = program
        .iec_record_frames()
        .unwrap()
        .into_iter()
        .find(|r| {
            matches!(r.kind, xgwx::IecRecordKind::Contact(_)) && program.data[r.offset + 5] == 1
        })
        .unwrap();
    let mut data = program.data.clone();
    data[record.offset + 1] = 0x3f;
    data[record.offset + 9..record.offset + 15].copy_from_slice(&[0, 0, 0x20, 0, 0, 0]);
    data.splice(record.end..record.end, [0u8; 8]);
    let encoded = if program.compressed {
        let mut encoder = bzip2::write::BzEncoder::new(Vec::new(), bzip2::Compression::best());
        encoder.write_all(&data).unwrap();
        encoder.finish().unwrap()
    } else {
        data
    };
    let parsed = roxmltree::Document::parse(&doc.container().xml()).unwrap();
    let node = parsed
        .descendants()
        .find(|n| n.has_tag_name("Program"))
        .unwrap()
        .descendants()
        .find(|n| n.has_tag_name("ProgramData"))
        .unwrap();
    let range = node.children().find(|n| n.is_text()).unwrap().range();
    let mut replacement_xml = doc.container().xml().to_owned();
    replacement_xml.replace_range(
        range,
        &base64::engine::general_purpose::STANDARD.encode(encoded),
    );
    doc.container_mut().replace_xml(replacement_xml).unwrap();
    let doc = XgwxDocument::parse(&doc.to_bytes().unwrap()).unwrap();
    assert_eq!(
        doc.program().ladder_program(0).unwrap().unwrap().data[record.offset + 1],
        0x3f
    );
    doc
}
#[test]
fn iec_native_comment_text_uses_the_comment_string_source_mapping() {
    let doc =
        captured_iec_comment_fixture(fixture("ladder-wire-delete/iec-horizontal-native.xgwx"));
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let old = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("comment ") && l.contains(" IEC "))
        .expect("native IEC fixture must contain a comment");
    let new = format!(
        "{}\"Updated native IEC comment with a longer string\"",
        old.split('"').next().unwrap()
    );
    let ops = ir.lower_patch(&replace_patch(&ir.text, old, &new)).unwrap();
    assert_eq!(ops.len(), 1);
    assert_eq!(ops[0].name, "update_iec_ld_comment");
    let mut changed = doc.clone();
    for op in ops {
        apply(&mut changed, &op.name, op.args);
    }
    let reparsed = XgwxDocument::parse(&changed.to_verified_bytes().unwrap()).unwrap();
    let next = Projection::export(&reparsed, "next", 0).unwrap();
    assert!(
        next.text
            .contains("Updated native IEC comment with a longer string")
    );
}

#[test]
fn blank_xgk_program_element_creation_deletion_and_collision_rollback() {
    let doc = fixture("empty-projects/new-xgk.xgwx");
    let original = doc.to_verified_bytes().unwrap();
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("p.xgwx"), &original).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let rev = revision(&original);
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let header = "  rung @p0.r0 at(y=0) {";
    assert!(ir.text.contains(header));
    let edited=ir.text.replace(header,&format!("{header}\n    coil @new-output at(x=94,y=0) OUT \"M00001\"\n    contact @new-input at(x=1,y=0) NO \"M00000\""));
    let patch = full_patch(&ir.text, &edited);
    let preview = store.apply_patch("p.xgwx", &rev, 0, &patch, true).unwrap();
    assert_eq!(preview["changed"], true);
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), original);
    let saved = store.apply_patch("p.xgwx", &rev, 0, &patch, false).unwrap();
    assert_eq!(
        fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
        original
    );
    let (_, rev, doc) = store.read("p.xgwx").unwrap();
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    assert_eq!(
        doc.program()
            .ladder_program(0)
            .unwrap()
            .unwrap()
            .structure
            .rungs
            .iter()
            .flat_map(|r| &r.cells)
            .filter(|c| c.contact.is_some() || c.coil.is_some())
            .count(),
        2
    );
    let contact = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("contact "))
        .unwrap();
    let collision = ir.text.replace(
        contact,
        &format!("{contact}\n    contact @new-collision at(x=1,y=0) NC \"M00002\""),
    );
    assert_eq!(
        ir.lower_patch(&full_patch(&ir.text, &collision))
            .unwrap_err()
            .code,
        "invalid_patch"
    );
    let edited = ir
        .text
        .lines()
        .filter(|l| !l.trim_start().starts_with("contact ") && !l.trim_start().starts_with("coil "))
        .map(|l| format!("{l}\n"))
        .collect::<String>();
    store
        .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &edited), false)
        .unwrap();
    let (_, _, doc) = store.read("p.xgwx").unwrap();
    assert!(
        doc.program()
            .ladder_program(0)
            .unwrap()
            .unwrap()
            .structure
            .rungs
            .iter()
            .flat_map(|r| &r.cells)
            .all(|c| c.contact.is_none() && c.coil.is_none())
    );
}
#[test]
fn first_xgk_rung_comment_can_be_created_without_existing_elements() {
    let doc = fixture("empty-projects/new-xgk.xgwx");
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let header = "  rung @p0.r0 at(y=0) {";
    let edited = ir.text.replace(
        header,
        &format!("{header}\n    comment @new-note at(x=1,y=0) RUNG \"First program note\""),
    );
    let ops = ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap();
    let mut doc = doc;
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    assert_eq!(
        doc.program()
            .ladder_program(0)
            .unwrap()
            .unwrap()
            .structure
            .rung_comments[0]
            .text,
        "First program note"
    );
    doc.to_verified_bytes().unwrap();
}
#[test]
fn iec_empty_row_creation_isolated_deletion_and_terminal_coil() {
    let doc = fixture("empty-projects/new-xgi.xgwx");
    let bytes = doc.to_verified_bytes().unwrap();
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    assert!(
        ir.text
            .contains("canvas @p0.blank at(group=0,row=0) readonly")
    );
    let header = ir.text.lines().find(|l| l.starts_with("program ")).unwrap();
    let edited = ir.text.replace(
        header,
        &format!("{header}\n  contact @new-input at(group=0,row=0,x=1) NO \"%MX0\""),
    );
    store
        .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &edited), false)
        .unwrap();
    let (_, rev, doc) = store.read("p.xgwx").unwrap();
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let line = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("contact "))
        .unwrap();
    let removed = ir
        .text
        .lines()
        .filter(|l| *l != line)
        .map(|l| format!("{l}\n"))
        .collect::<String>();
    let preview = store
        .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &removed), true)
        .unwrap();
    assert_eq!(
        preview["results"][0]["name"],
        "delete_iec_ld_isolated_element"
    );
    let edited = ir.text.replace(
        line,
        &format!("{line}\n  coil @new-output at(group=0,row=0,x=94) OUT \"%MX1\""),
    );
    store
        .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &edited), false)
        .unwrap();
    let (_, rev, doc) = store.read("p.xgwx").unwrap();
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    assert!(ir.text.lines().any(|l| l.trim_start().starts_with("coil ")));
    let contact = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("contact "))
        .unwrap();
    let removed = ir
        .text
        .lines()
        .filter(|l| *l != contact)
        .map(|l| format!("{l}\n"))
        .collect::<String>();
    let ops = ir.lower_patch(&full_patch(&ir.text, &removed)).unwrap();
    assert_eq!(ops[0].name, "delete_iec_ld_contact");
    let mut doc = doc;
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    let next = Projection::export(&doc, "next", 0).unwrap();
    assert!(
        !next
            .text
            .lines()
            .any(|l| l.trim_start().starts_with("contact "))
    );
    assert!(
        next.text
            .lines()
            .any(|l| l.trim_start().starts_with("coil "))
    );
}
#[test]
fn iec_isolated_coil_creation_then_deletion_keeps_row_envelope() {
    let mut doc = fixture("empty-projects/new-xgi.xgwx");
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let header = ir.text.lines().find(|l| l.starts_with("program ")).unwrap();
    let edited = ir.text.replace(
        header,
        &format!("{header}\n  coil @new-output at(group=0,row=0,x=94) SET \"%MX1\""),
    );
    for op in ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap() {
        apply(&mut doc, &op.name, op.args);
    }
    let ir = Projection::export(&doc, "next", 0).unwrap();
    let line = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("coil "))
        .unwrap();
    let edited = ir
        .text
        .lines()
        .filter(|l| *l != line)
        .map(|l| format!("{l}\n"))
        .collect::<String>();
    for op in ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap() {
        apply(&mut doc, &op.name, op.args);
    }
    assert_eq!(
        doc.program()
            .ladder_program(0)
            .unwrap()
            .unwrap()
            .iec_row_frames()
            .unwrap()[0]
            .record_count,
        0
    );
    doc.to_verified_bytes().unwrap();
}

#[test]
fn xgk_output_column_maps_native_x94_for_kind_operand_edits() {
    use serde_json::json;
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    apply(
        &mut doc,
        "edit_ladder_cell",
        json!({"program_index":0,"edit":{"raw_y":0,"column":9,"expected":null,"replacement":{"kind":"Output","operand":"M00001"}}}),
    );
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let old = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("coil "))
        .unwrap();
    assert!(old.contains("at(x=94,y=0)"));
    let new = old.replace(" OUT ", " SET ").replace("M00001", "M00002");
    let ops = ir.lower_patch(&replace_patch(&ir.text, old, &new)).unwrap();
    assert_eq!(ops[0].args["edit"]["column"], 9);
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    let program = doc.program().ladder_program(0).unwrap().unwrap();
    let cell = program
        .structure
        .rungs
        .iter()
        .flat_map(|r| &r.cells)
        .find(|c| c.coil.is_some())
        .unwrap();
    assert_eq!(cell.raw_x, 94);
    assert_eq!(cell.coil, Some(xgwx::LadderCoil::Set));
    assert_eq!(cell.value, "M00002");
    doc.to_verified_bytes().unwrap();
}
#[test]
fn iec_isolated_deletion_and_lower_offset_resizing_edit_share_a_transaction() {
    use serde_json::json;
    let mut doc = fixture("empty-projects/new-xgi.xgwx");
    for row in [0, 4] {
        apply(
            &mut doc,
            "insert_iec_ld_single_element",
            json!({"program_index":0,"row_index":row,"raw_x":1,"category":"contact","kind":"NO","operand":"%MX0"}),
        );
    }
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let contacts = ir
        .text
        .lines()
        .filter(|l| l.trim_start().starts_with("contact "))
        .collect::<Vec<_>>();
    assert_eq!(contacts.len(), 2);
    let edited = ir
        .text
        .lines()
        .filter(|l| *l != contacts[1])
        .map(|l| {
            if l == contacts[0] {
                format!("{}\n", l.replace("%MX0", "%MX12"))
            } else {
                format!("{l}\n")
            }
        })
        .collect::<String>();
    let ops = ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap();
    assert_eq!(ops[0].name, "delete_iec_ld_isolated_element");
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    let reparsed = XgwxDocument::parse(&doc.to_verified_bytes().unwrap()).unwrap();
    let next = Projection::export(&reparsed, "next", 0).unwrap();
    assert_eq!(
        next.text
            .lines()
            .filter(|l| l.trim_start().starts_with("contact "))
            .count(),
        1
    );
    assert!(next.text.contains("%MX12"));
}

fn remove_line(text: &str, line: &str) -> String {
    text.lines()
        .filter(|l| *l != line)
        .map(|l| format!("{l}\n"))
        .collect()
}
#[test]
fn connected_iec_terminal_coil_deletion_preserves_contact_and_symbols() {
    use serde_json::json;
    let mut doc = fixture("empty-projects/new-xgi.xgwx");
    apply(
        &mut doc,
        "insert_iec_ld_single_element",
        json!({"program_index":0,"row_index":0,"raw_x":1,"category":"contact","kind":"NC","operand":"%MX0"}),
    );
    apply(
        &mut doc,
        "insert_iec_ld_single_element",
        json!({"program_index":0,"row_index":0,"raw_x":94,"category":"coil","kind":"SET","operand":"%MX1"}),
    );
    let vars = doc.variable().variables().unwrap();
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let coil = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("coil "))
        .unwrap();
    let ops = ir
        .lower_patch(&full_patch(&ir.text, &remove_line(&ir.text, coil)))
        .unwrap();
    assert_eq!(ops[0].name, "delete_iec_ld_terminal_coil");
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    let next = Projection::export(&doc, "next", 0).unwrap();
    assert!(
        next.text
            .lines()
            .any(|l| l.trim_start().starts_with("contact ") && l.contains(" NC \"%MX0\""))
    );
    assert!(
        !next
            .text
            .lines()
            .any(|l| l.trim_start().starts_with("coil "))
    );
    assert_eq!(vars, doc.variable().variables().unwrap());
    doc.to_verified_bytes().unwrap();
}
#[test]
fn xgk_physical_wire_deletion_and_vertical_branch_creation_preserve_elements() {
    use serde_json::json;
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    for y in [0, 4] {
        for (column, kind, operand) in [(0, "NormallyOpen", "M00000"), (9, "Output", "M00001")] {
            apply(
                &mut doc,
                "edit_ladder_cell",
                json!({"program_index":0,"edit":{"raw_y":y,"column":column,"expected":null,"replacement":{"kind":kind,"operand":operand}}}),
            );
        }
    }
    let elements = doc
        .program()
        .ladder_program(0)
        .unwrap()
        .unwrap()
        .structure
        .rungs;
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let header = ir.text.lines().find(|l| l.starts_with("program ")).unwrap();
    let edited = ir.text.replace(
        header,
        &format!("{header}\n  wire @new-branch from at(x=3,y=0) to at(x=3,y=4) VERTICAL"),
    );
    let ops = ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap();
    assert_eq!(ops[0].name, "edit_ladder_branch");
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    let values = |rows: &Vec<xgwx::LadderRung>| {
        rows.iter()
            .flat_map(|r| {
                r.cells
                    .iter()
                    .map(|c| (c.raw_x, c.raw_y, c.contact, c.coil, c.value.clone()))
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(
        values(
            &doc.program()
                .ladder_program(0)
                .unwrap()
                .unwrap()
                .structure
                .rungs
        ),
        values(&elements)
    );
    let ir = Projection::export(&doc, "next", 0).unwrap();
    let wire = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("wire ") && l.ends_with("VERTICAL"))
        .unwrap();
    for op in ir
        .lower_patch(&full_patch(&ir.text, &remove_line(&ir.text, wire)))
        .unwrap()
    {
        apply(&mut doc, &op.name, op.args);
    }
    assert!(
        doc.program()
            .ladder_program(0)
            .unwrap()
            .unwrap()
            .structure
            .vertical_lines
            .is_empty()
    );
    let ir = Projection::export(&doc, "next", 0).unwrap();
    let wire = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("wire ") && l.ends_with("HORIZONTAL"))
        .unwrap();
    let ops = ir
        .lower_patch(&full_patch(&ir.text, &remove_line(&ir.text, wire)))
        .unwrap();
    assert_eq!(ops[0].name, "delete_ladder_horizontal_wire");
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    // Binary offsets move; compare retained native positions/kinds/operands.
    let after = doc.program().ladder_program(0).unwrap().unwrap();
    let values = |rows: &Vec<xgwx::LadderRung>| {
        rows.iter()
            .flat_map(|r| {
                r.cells
                    .iter()
                    .map(|c| (c.raw_x, c.raw_y, c.contact, c.coil, c.value.clone()))
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(values(&after.structure.rungs), values(&elements));
    doc.to_verified_bytes().unwrap();
}
#[test]
fn iec_horizontal_wire_deletion_matches_native_capture_and_retains_records() {
    use serde_json::json;
    let mut doc = fixture("empty-projects/new-xgi.xgwx");
    apply(
        &mut doc,
        "insert_iec_ld_single_element",
        json!({"program_index":0,"row_index":0,"raw_x":1,"category":"contact","kind":"NO","operand":"%MX0"}),
    );
    apply(
        &mut doc,
        "insert_iec_ld_single_element",
        json!({"program_index":0,"row_index":0,"raw_x":94,"category":"coil","kind":"OUTPUT","operand":"%MX1"}),
    );
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let wire = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("wire ") && l.ends_with("HORIZONTAL"))
        .unwrap();
    assert!(ir.text.contains("connection "));
    let connection = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("connection "))
        .unwrap();
    assert_eq!(
        ir.lower_patch(&full_patch(&ir.text, &remove_line(&ir.text, connection)))
            .unwrap_err()
            .code,
        "unsupported_ir_edit"
    );
    let ops = ir
        .lower_patch(&full_patch(&ir.text, &remove_line(&ir.text, wire)))
        .unwrap();
    assert_eq!(ops[0].name, "delete_iec_ld_horizontal_wire_record");
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    assert_eq!(
        doc.program().ladder_program(0).unwrap().unwrap().data,
        fixture("ladder-wire-delete/iec-horizontal-native.xgwx")
            .program()
            .ladder_program(0)
            .unwrap()
            .unwrap()
            .data
    );
}
#[test]
fn topology_patch_requires_reinspection_and_failure_leaves_source_unchanged() {
    let doc = fixture("elements.xgwx");
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let wire = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("wire ") && l.ends_with("HORIZONTAL"))
        .unwrap();
    let contact = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("contact ") && l.contains(" NO "))
        .unwrap();
    let edited = remove_line(&ir.text, wire).replace(contact, &contact.replace(" NO ", " NC "));
    let error = store
        .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &edited), false)
        .unwrap_err();
    assert_eq!(error.code, "unsupported_ir_edit");
    assert!(error.message.contains("sole mutation"));
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    let header = ir.text.lines().find(|l| l.starts_with("program ")).unwrap();
    let edited = ir.text.replace(
        header,
        &format!("{header}\n  wire @new-bad from at(x=3,y=0) to at(x=3,y=8) VERTICAL"),
    );
    assert!(ir.lower_patch(&full_patch(&ir.text, &edited)).is_err());
}

#[test]
fn iec_vertical_wire_removal_and_recreation_preserve_elements() {
    use serde_json::json;
    let mut doc = fixture("empty-projects/new-xgi.xgwx");
    for (raw_x, category, kind, operand) in
        [(1, "contact", "NO", "%MX0"), (94, "coil", "OUTPUT", "%MX1")]
    {
        apply(
            &mut doc,
            "insert_iec_ld_single_element",
            json!({"program_index":0,"row_index":0,"raw_x":raw_x,"category":category,"kind":kind,"operand":operand}),
        );
    }
    apply(
        &mut doc,
        "insert_iec_ld_parallel_contact_kind",
        json!({"program_index":0,"row_index":0,"expected_contact_variable":"%MX0","expected_coil_variable":"%MX1","parallel_kind":"NC","parallel_variable":"%MX2"}),
    );
    let before = doc.program().ladder_program(0).unwrap().unwrap();
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let wire = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("wire ") && l.ends_with("VERTICAL"))
        .unwrap()
        .to_string();
    let ops = ir
        .lower_patch(&full_patch(&ir.text, &remove_line(&ir.text, &wire)))
        .unwrap();
    assert_eq!(ops[0].name, "edit_iec_ld_vertical_wire");
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    let next = Projection::export(&doc, "next", 0).unwrap();
    let header = next
        .text
        .lines()
        .find(|l| l.starts_with("program "))
        .unwrap();
    let new_wire = wire
        .split_whitespace()
        .enumerate()
        .map(|(i, s)| if i == 1 { "@new-restored" } else { s })
        .collect::<Vec<_>>()
        .join(" ");
    let edited = next
        .text
        .replace(header, &format!("{header}\n  {new_wire}"));
    let ops = next.lower_patch(&full_patch(&next.text, &edited)).unwrap();
    assert_eq!(ops[0].name, "edit_iec_ld_vertical_wire");
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    let after = doc.program().ladder_program(0).unwrap().unwrap();
    assert_eq!(before.data, after.data);
    doc.to_verified_bytes().unwrap();
}

#[test]
fn vertical_wire_expected_absence_failure_retains_source_without_backup() {
    use serde_json::json;
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    for y in [0, 4] {
        for (column, kind, operand) in [(0, "NormallyOpen", "M00000"), (9, "Output", "M00001")] {
            apply(
                &mut doc,
                "edit_ladder_cell",
                json!({"program_index":0,"edit":{"raw_y":y,"column":column,"expected":null,"replacement":{"kind":kind,"operand":operand}}}),
            );
        }
    }
    apply(
        &mut doc,
        "edit_ladder_branch",
        json!({"program_index":0,"edit":{"raw_y":0,"boundary":1,"expected":false,"present":true}}),
    );
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let header = ir.text.lines().find(|l| l.starts_with("program ")).unwrap();
    let edited = ir.text.replace(
        header,
        &format!("{header}\n  wire @new-duplicate from at(x=3,y=0) to at(x=3,y=4) VERTICAL"),
    );
    let error = store
        .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &edited), false)
        .unwrap_err();
    assert_eq!(error.code, "operation_failed");
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    assert!(!fs::read_dir(dir.path()).unwrap().any(|e| {
        e.unwrap()
            .file_name()
            .to_string_lossy()
            .contains(".xgwx-mcp-backup-")
    }));
}

#[test]
fn iec_function_names_and_multiple_resizing_arguments_preserve_bindings() {
    let mut doc = fixture("function-output-wires/base.xgwx");
    let program = doc.program().ladder_program(0).unwrap().unwrap();
    let functions = program.iec_function_blocks().unwrap();
    let links = program.iec_function_operand_links().unwrap();
    let symbols = doc.variable().variables().unwrap();
    let locals = doc
        .variable()
        .iec_local_symbols()
        .into_iter()
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    let others = doc
        .program()
        .ladder_programs()
        .into_iter()
        .skip(1)
        .map(|p| p.unwrap().data)
        .collect::<Vec<_>>();
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let mut edited = ir.text.clone();
    for (name, replacement, value) in [
        ("ADD", "MUL", "12345"),
        ("EQ", "GE", "23456"),
        ("INT_TO_UDINT", "INT_TO_UDINT", "3456"),
    ] {
        let block = functions.iter().find(|b| b.name.value == name).unwrap();
        let line = ir
            .text
            .lines()
            .find(|l| {
                l.trim_start()
                    .starts_with(&format!("function @p0.f{} ", block.record_offset))
            })
            .unwrap();
        edited = edited.replace(
            line,
            &line.replace(
                &format!("name \"{name}\""),
                &format!("name \"{replacement}\""),
            ),
        );
        let link = links
            .iter()
            .find(|l| l.target_record_offset == block.record_offset && !l.is_output)
            .unwrap();
        let arg = ir
            .text
            .lines()
            .find(|l| {
                l.trim_start()
                    .starts_with(&format!("argument @p0.e{} ", link.record_offset))
            })
            .unwrap();
        edited = edited.replace(
            arg,
            &format!("{} value \"{value}\"", arg.split(" value ").next().unwrap()),
        );
    }
    let ops = ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap();
    assert_eq!(ops.len(), 5);
    assert!(
        ops.iter()
            .any(|o| o.name == "update_iec_ld_arithmetic_function")
    );
    assert!(
        ops.iter()
            .any(|o| o.name == "update_iec_ld_comparison_function")
    );
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    let output_ir = Projection::export(&doc, "outputs", 0).unwrap();
    let current = doc.program().ladder_program(0).unwrap().unwrap();
    let blocks = current.iec_function_blocks().unwrap();
    let current_links = current.iec_function_operand_links().unwrap();
    let mut output_text = output_ir.text.clone();
    for (name, value) in [("GE", "%MX20"), ("INT_TO_UDINT", "%MD20")] {
        let block = blocks.iter().find(|b| b.name.value == name).unwrap();
        let link = current_links
            .iter()
            .find(|l| l.target_record_offset == block.record_offset && l.is_output)
            .unwrap();
        let line = output_ir
            .text
            .lines()
            .find(|l| {
                l.trim_start()
                    .starts_with(&format!("argument @p0.e{} ", link.record_offset))
            })
            .unwrap();
        output_text = output_text.replace(
            line,
            &format!(
                "{} value \"{value}\"",
                line.split(" value ").next().unwrap()
            ),
        );
    }
    for op in output_ir
        .lower_patch(&full_patch(&output_ir.text, &output_text))
        .unwrap()
    {
        apply(&mut doc, &op.name, op.args);
    }
    let next = Projection::export(&doc, "next", 0).unwrap();
    for value in ["12345", "23456", "3456", "%MX20", "%MD20"] {
        assert!(next.text.contains(&format!("value \"{value}\"")));
    }
    assert!(next.text.contains("name \"MUL\""));
    assert!(next.text.contains("name \"GE\""));
    let after = doc.program().ladder_program(0).unwrap().unwrap();
    let bindings = |links: Vec<xgwx::IecFunctionOperandLink>| {
        links
            .into_iter()
            .map(|l| {
                (
                    l.group_index,
                    l.row_index,
                    l.ordinal,
                    l.is_output,
                    l.pin_row_index,
                    l.pin_raw_x,
                    l.data_type_mask,
                    l.is_array,
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(
        bindings(links),
        bindings(after.iec_function_operand_links().unwrap())
    );
    assert_eq!(symbols, doc.variable().variables().unwrap());
    assert_eq!(
        locals,
        doc.variable()
            .iec_local_symbols()
            .into_iter()
            .map(Result::unwrap)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        others,
        doc.program()
            .ladder_programs()
            .into_iter()
            .skip(1)
            .map(|p| p.unwrap().data)
            .collect::<Vec<_>>()
    );
    doc.to_verified_bytes().unwrap();
}

#[test]
fn iec_function_metadata_deletion_rebinding_and_invalid_types_fail_atomically() {
    let doc = fixture("function-output-wires/base.xgwx");
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let function = ir
        .text
        .lines()
        .find(|l| l.contains("name \"ADD\""))
        .unwrap();
    let argument = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("argument ") && l.contains("direction INPUT"))
        .unwrap();
    let output = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("argument ") && l.contains("direction OUTPUT"))
        .unwrap();
    for changed in [
        remove_line(&ir.text, function),
        remove_line(&ir.text, argument),
        ir.text.replace(
            argument,
            &argument.replace("direction INPUT", "direction OUTPUT"),
        ),
        ir.text.replace(
            argument,
            &format!("{} value \"\"", argument.split(" value ").next().unwrap()),
        ),
        ir.text.replace(
            function,
            &function.replace("signature \"", "signature \"tampered"),
        ),
    ] {
        assert_eq!(
            ir.lower_patch(&full_patch(&ir.text, &changed))
                .unwrap_err()
                .code,
            "unsupported_ir_edit"
        );
    }
    for value in ["%MX20", "%MW0 AND %MW1"] {
        let changed = ir.text.replace(
            argument,
            &format!(
                "{} value \"{value}\"",
                argument.split(" value ").next().unwrap()
            ),
        );
        assert_eq!(
            store
                .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &changed), false)
                .unwrap_err()
                .code,
            "operation_failed"
        );
        assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    }
    // A supported function-name edit plus a forbidden output literal must roll back together.
    let edited = ir
        .text
        .replace(function, &function.replace("name \"ADD\"", "name \"SUB\""))
        .replace(
            output,
            &format!("{} value \"123\"", output.split(" value ").next().unwrap()),
        );
    let error = store
        .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &edited), false)
        .unwrap_err();
    assert_eq!(error.code, "operation_failed");
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    assert!(!fs::read_dir(dir.path()).unwrap().any(|e| {
        e.unwrap()
            .file_name()
            .to_string_lossy()
            .contains(".xgwx-mcp-backup-")
    }));
    let changed = ir
        .text
        .replace(function, &function.replace("name \"ADD\"", "name \"EQ\""));
    assert!(
        store
            .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &changed), true)
            .is_err()
    );
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
}

#[test]
fn xgk_instruction_creation_edit_arity_change_and_deletion_use_native_writers() {
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    let symbols = doc.variable().variables().unwrap();
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let rung = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("rung "))
        .unwrap();
    let edited = ir.text.replace(rung,&format!("{rung}\n    instruction @new-move at(x=88,y=0) APPLICATION mnemonic \"MOV\" operands \"1\" \"D000000\""));
    let ops = ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap();
    assert_eq!(ops[0].name, "insert_ladder_instruction");
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    let ir = Projection::export(&doc, "next", 0).unwrap();
    let line = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("instruction "))
        .unwrap();
    assert!(
        line.contains("at(x=88,y=0) APPLICATION mnemonic \"MOV\" operands \"1\" \"D000000\""),
        "{line}"
    );
    let edited = ir.text.replace(line, &line.replace("\"1\"", "\"12345\""));
    let ops = ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap();
    assert_eq!(ops[0].name, "update_ladder_cell_text");
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    let ir = Projection::export(&doc, "next", 0).unwrap();
    let line = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("instruction "))
        .unwrap();
    let edited = ir.text.replace(
        line,
        &line.replace(
            "mnemonic \"MOV\" operands \"12345\" \"D000000\"",
            "mnemonic \"ADD\" operands \"1\" \"2\" \"D000000\"",
        ),
    );
    for op in ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap() {
        apply(&mut doc, &op.name, op.args);
    }
    let ir = Projection::export(&doc, "next", 0).unwrap();
    let line = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("instruction "))
        .unwrap();
    assert!(line.contains("at(x=85,y=0) APPLICATION mnemonic \"ADD\""));
    let ops = ir
        .lower_patch(&full_patch(&ir.text, &remove_line(&ir.text, line)))
        .unwrap();
    assert_eq!(ops[0].name, "delete_ladder_instruction");
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    assert!(
        !Projection::export(&doc, "next", 0)
            .unwrap()
            .text
            .lines()
            .any(|l| l.trim_start().starts_with("instruction "))
    );
    assert_eq!(symbols, doc.variable().variables().unwrap());
    doc.to_verified_bytes().unwrap();
}

#[test]
fn xgk_comparison_creation_edit_and_deletion_preserve_coil() {
    use serde_json::json;
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    apply(
        &mut doc,
        "edit_ladder_cell",
        json!({"program_index":0,"edit":{"raw_y":0,"column":9,"expected":null,"replacement":{"kind":"Output","operand":"M00001"}}}),
    );
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let rung = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("rung "))
        .unwrap();
    let edited = ir.text.replace(rung,&format!("{rung}\n    instruction @new-compare at(x=1,y=0) COMPARISON mnemonic \"=\" operands \"D000000\" \"1\""));
    let ops = ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap();
    assert_eq!(ops[0].name, "insert_ladder_comparison");
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    let ir = Projection::export(&doc, "next", 0).unwrap();
    let line = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("instruction "))
        .unwrap();
    let edited = ir.text.replace(
        line,
        &line
            .replace("mnemonic \"=\"", "mnemonic \">\"")
            .replace("\"1\"", "\"23456\""),
    );
    for op in ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap() {
        apply(&mut doc, &op.name, op.args);
    }
    let ir = Projection::export(&doc, "next", 0).unwrap();
    let line = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("instruction "))
        .unwrap();
    assert!(line.contains("mnemonic \">\""));
    let ops = ir
        .lower_patch(&full_patch(&ir.text, &remove_line(&ir.text, line)))
        .unwrap();
    assert_eq!(ops[0].name, "delete_ladder_comparison");
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    let cells = doc
        .program()
        .ladder_program(0)
        .unwrap()
        .unwrap()
        .structure
        .rungs
        .into_iter()
        .flat_map(|r| r.cells)
        .collect::<Vec<_>>();
    assert_eq!(cells.len(), 1);
    assert_eq!(cells[0].value, "M00001");
    doc.to_verified_bytes().unwrap();
}

#[test]
fn xgk_instruction_batches_and_invalid_operands_preserve_source() {
    use serde_json::json;
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    for y in [0, 4] {
        apply(
            &mut doc,
            "insert_ladder_instruction",
            json!({"program_index":0,"raw_y":y,"mnemonic":"MOV","operands":["1","D000000"]}),
        );
    }
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let edited = ir.text.replace(
        "operands \"1\" \"D000000\"",
        "operands \"12345\" \"D000004\"",
    );
    let ops = ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap();
    assert_eq!(ops.len(), 2);
    assert!(ops[0].args["offset"].as_u64() > ops[1].args["offset"].as_u64());
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    assert_eq!(ir.text.matches("operands \"12345\" \"D000004\"").count(), 2);
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let lines = ir
        .text
        .lines()
        .filter(|l| l.trim_start().starts_with("instruction "))
        .collect::<Vec<_>>();
    // Higher-offset success followed by a lower-offset invalid output rolls back.
    let edited = ir
        .text
        .replace(lines[1], &lines[1].replace("\"12345\"", "\"123\""))
        .replace(lines[0], &lines[0].replace("\"D000004\"", "\"1\""));
    let error = store
        .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &edited), false)
        .unwrap_err();
    assert_eq!(error.code, "operation_failed");
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    let edited = ir
        .text
        .replace(
            lines[0],
            &lines[0].replace(
                "mnemonic \"MOV\" operands \"12345\" \"D000004\"",
                "mnemonic \"ADD\" operands \"1\" \"2\" \"D000004\"",
            ),
        )
        .replace(lines[1], &lines[1].replace("\"12345\"", "\"123\""));
    assert_eq!(
        ir.lower_patch(&full_patch(&ir.text, &edited))
            .unwrap_err()
            .code,
        "unsupported_ir_edit"
    );
    for (old, new) in [
        ("mnemonic \"MOV\"", "mnemonic \"ADD,1\""),
        ("operands \"12345\"", "operands \"1,2\""),
        ("operands \"12345\"", "operands \"'a','b'\""),
    ] {
        let edited = ir.text.replace(lines[0], &lines[0].replace(old, new));
        assert_eq!(
            ir.lower_patch(&full_patch(&ir.text, &edited))
                .unwrap_err()
                .code,
            "invalid_patch"
        );
    }
    let edited = ir
        .text
        .replace(lines[0], &lines[0].replace("APPLICATION", "COMPARISON"));
    assert_eq!(
        ir.lower_patch(&full_patch(&ir.text, &edited))
            .unwrap_err()
            .code,
        "unsupported_ir_edit"
    );
    assert!(!fs::read_dir(dir.path()).unwrap().any(|e| {
        e.unwrap()
            .file_name()
            .to_string_lossy()
            .contains(".xgwx-mcp-backup-")
    }));
}

#[test]
fn xgk_unsupported_native_instruction_layout_stays_read_only() {
    use base64::Engine;
    use serde_json::json;
    use std::io::Write;
    let mut doc = fixture("empty-projects/new-xgk.xgwx");
    apply(
        &mut doc,
        "insert_ladder_instruction",
        json!({"program_index":0,"raw_y":0,"mnemonic":"MOV","operands":["1","D000000"]}),
    );
    let program = doc.program().ladder_program(0).unwrap().unwrap();
    let cell = program
        .structure
        .rungs
        .iter()
        .flat_map(|r| &r.cells)
        .find(|c| c.value == "MOV")
        .unwrap();
    let mut data = program.data.clone();
    // An undecoded reserved instruction header byte invalidates the structural layout.
    data[cell.offset - 19 + 2] = 1;
    let encoded = if program.compressed {
        let mut encoder = bzip2::write::BzEncoder::new(Vec::new(), bzip2::Compression::best());
        encoder.write_all(&data).unwrap();
        encoder.finish().unwrap()
    } else {
        data
    };
    let parsed = roxmltree::Document::parse(&doc.container().xml()).unwrap();
    let node = parsed
        .descendants()
        .find(|n| n.has_tag_name("Program"))
        .unwrap()
        .descendants()
        .find(|n| n.has_tag_name("ProgramData"))
        .unwrap();
    let range = node.children().find(|n| n.is_text()).unwrap().range();
    let mut replacement_xml = doc.container().xml().to_owned();
    replacement_xml.replace_range(
        range,
        &base64::engine::general_purpose::STANDARD.encode(encoded),
    );
    doc.container_mut().replace_xml(replacement_xml).unwrap();
    assert!(
        doc.clone()
            .program_mut()
            .get(0)
            .and_then(|mut selected_program| selected_program.ladder()?.insert_row(0))
            .is_err()
    );
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    assert!(
        !ir.text
            .lines()
            .any(|l| l.trim_start().starts_with("instruction "))
    );
    assert!(ir.lower_patch("").unwrap().is_empty());
    let record = ir
        .text
        .lines()
        .find(|l| l.contains("value \"MOV\""))
        .unwrap();
    assert!(record.ends_with("readonly"));
    assert_eq!(
        ir.lower_patch(&replace_patch(
            &ir.text,
            record,
            &record.replace("\"MOV\"", "\"ADD\"")
        ))
        .unwrap_err()
        .code,
        "unsupported_ir_edit"
    );
}

#[test]
fn xgk_instruction_operandless_and_quoted_comma_literal_roundtrip() {
    for (x, mnemonic, operands, replacement) in [
        (94, "CLC", vec![], "CLE"),
        (88, "$MOV", vec!["'a,b'", "D000000"], "$MOV"),
    ] {
        let mut doc = fixture("empty-projects/new-xgk.xgwx");
        let ir = Projection::export(&doc, "revision", 0).unwrap();
        let rung = ir
            .text
            .lines()
            .find(|l| l.trim_start().starts_with("rung "))
            .unwrap();
        let args = operands
            .iter()
            .map(|s| format!(" {}", serde_json::to_string(s).unwrap()))
            .collect::<String>();
        let edited = ir.text.replace(rung,&format!("{rung}\n    instruction @new-test at(x={x},y=0) APPLICATION mnemonic \"{mnemonic}\" operands{args}"));
        for op in ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap() {
            apply(&mut doc, &op.name, op.args);
        }
        let ir = Projection::export(&doc, "next", 0).unwrap();
        let line = ir
            .text
            .lines()
            .find(|l| l.trim_start().starts_with("instruction "))
            .unwrap();
        assert!(line.ends_with(&format!("operands{args}")));
        let edited = ir.text.replace(
            line,
            &line
                .replace(
                    &format!("mnemonic \"{mnemonic}\""),
                    &format!("mnemonic \"{replacement}\""),
                )
                .replace("'a,b'", "'c,d,e'"),
        );
        for op in ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap() {
            apply(&mut doc, &op.name, op.args);
        }
        let ir = Projection::export(&doc, "next", 0).unwrap();
        let line = ir
            .text
            .lines()
            .find(|l| l.trim_start().starts_with("instruction "))
            .unwrap();
        assert!(line.contains(&format!("mnemonic \"{replacement}\"")));
        for op in ir
            .lower_patch(&full_patch(&ir.text, &remove_line(&ir.text, line)))
            .unwrap()
        {
            apply(&mut doc, &op.name, op.args);
        }
        doc.to_verified_bytes().unwrap();
    }
}

#[test]
fn sfc_text_export_rows_comments_conditions_and_native_snapshots() {
    for name in [
        "native-loop",
        "multi-action-two-native",
        "multi-action-branch-native",
        "declarations-roundtrip",
        "new-xgi-sfc",
    ] {
        let doc = fixture(&format!("sfc/{name}.xgwx"));
        let ir = Projection::export(&doc, "revision", 0).unwrap();
        assert_eq!(ir.dialect, "SFC");
        assert!(ir.text.contains("chart @p0.b0"));
        assert!(ir.lower_patch("").unwrap().is_empty());
    }
    let mut doc = fixture("sfc/native-loop.xgwx");
    let locals = doc.variable().sfc_variables(0).unwrap();
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let edited = ir
        .text
        .replace(
            "comment \"Ready for cycle\"",
            "comment \"Agent note\\nWait & check <ready>\"",
        )
        .replace("title \"%MX0\"", "title \"%MX2\"");
    assert_ne!(edited, ir.text);
    let ops = ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap();
    assert_eq!(ops.len(), 1);
    assert_eq!(ops[0].name, "replace_sfc_sequence");
    assert!(ops[0].args["patch"]["expected_rows"].is_array());
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    let next = Projection::export(&doc, "next", 0).unwrap();
    assert!(next.text.contains("Agent note\\nWait & check <ready>"));
    assert!(next.text.contains("title \"%MX2\""));
    assert_eq!(locals, doc.variable().sfc_variables(0).unwrap());
    doc.to_verified_bytes().unwrap();
}

#[test]
fn sfc_action_and_st_source_edits_preserve_declarations_and_branch_structure() {
    let mut doc = fixture("sfc/multi-action-two-native.xgwx");
    let locals = doc.variable().sfc_variables(0).unwrap();
    let original = doc.program().sfc_programs()[0].blocks[0]
        .editable_rows
        .clone()
        .unwrap();
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let line = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("sfc_row ") && l.contains("qualifier \"L\""))
        .unwrap();
    let edited = ir.text.replace(
        line,
        &line
            .replace("time \"T#2s\"", "time \"T#3s\"")
            .replace("action \"%MX12\"", "action \"%MX14\""),
    );
    let source = original[1].action_code.as_ref().unwrap();
    let replacement = format!("(* Agent action *)\n{source}");
    let edited = edited.replace(
        &format!("action_code {}", serde_json::to_string(source).unwrap()),
        &format!(
            "action_code {}",
            serde_json::to_string(&replacement).unwrap()
        ),
    );
    for op in ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap() {
        apply(&mut doc, &op.name, op.args);
    }
    let after = doc.program().sfc_programs()[0].blocks[0]
        .editable_rows
        .clone()
        .unwrap();
    assert_eq!(after[2].action.as_deref(), Some("%MX14"));
    assert_eq!(after[2].action_time.as_deref(), Some("T#3s"));
    assert_eq!(after[1].action_code.as_deref(), Some(replacement.as_str()));
    assert_eq!(locals, doc.variable().sfc_variables(0).unwrap());
    doc.to_verified_bytes().unwrap();
}

#[test]
fn sfc_multiple_declaration_descriptions_rebase_complete_snapshots() {
    let mut doc = fixture("sfc/declarations-roundtrip.xgwx");
    let before = doc.program().sfc_programs()[0].blocks.clone();
    let vars = doc.variable().sfc_variables(0).unwrap();
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let mut edited = ir.text.clone();
    let mut count = 0;
    for line in ir
        .text
        .lines()
        .filter(|l| l.trim_start().starts_with("sfc_variable ") && l.contains("system false"))
        .take(2)
    {
        edited = edited.replace(
            line,
            &format!(
                "{} description \"Agent declaration {count}\"",
                line.rsplit_once(" description ").unwrap().0
            ),
        );
        count += 1;
    }
    assert_eq!(count, 2);
    let ops = ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap();
    assert_eq!(ops.len(), 2);
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    assert_eq!(before, doc.program().sfc_programs()[0].blocks);
    let next = doc.variable().sfc_variables(0).unwrap();
    for (a, b) in vars.iter().zip(&next) {
        assert_eq!(a.name, b.name);
        assert_eq!(a.data_type, b.data_type);
        assert_eq!(a.declaration, b.declaration);
    }
    assert_eq!(
        next.iter()
            .filter(|v| v.description.starts_with("Agent declaration"))
            .count(),
        2
    );
    doc.to_verified_bytes().unwrap();
}

#[test]
fn sfc_unsupported_chart_retains_opaque_xml_and_uses_entity_writer() {
    let mut doc = fixture("sfc/native-loop.xgwx");
    {
        let replacement_xml = doc
            .container()
            .xml()
            .replace("<EntityGrid ", "<EntityGrid AgentOpaque=\"keep\" ");
        doc.container_mut().replace_xml(replacement_xml).unwrap();
    };
    let mut doc = XgwxDocument::parse(&doc.to_bytes().unwrap()).unwrap();
    assert!(
        doc.program().sfc_programs()[0].blocks[0]
            .editable_rows
            .is_none()
    );
    let locals = doc.variable().sfc_variables(0).unwrap();
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    assert!(ir.text.contains("sequence unsupported readonly"));
    let condition = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("entity_field ") && l.contains("value \"%MX0\""))
        .unwrap();
    let comment = ir
        .text
        .lines()
        .find(|l| {
            l.trim_start().starts_with("entity_field ") && l.contains("value \"Ready for cycle\"")
        })
        .unwrap();
    let edited = ir
        .text
        .replace(condition, &condition.replace("%MX0", "%MX2"))
        .replace(comment, &comment.replace("Ready for cycle", "Agent note"));
    let ops = ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap();
    assert_eq!(ops.len(), 2);
    assert!(ops.iter().all(|o| o.name == "edit_sfc_entity"));
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    assert!(doc.container().xml().contains("AgentOpaque=\"keep\""));
    assert_eq!(doc.container().xml().matches("Title=\"%MX2\"").count(), 2);
    assert_eq!(locals, doc.variable().sfc_variables(0).unwrap());
    doc.to_verified_bytes().unwrap();
    let moved = ir
        .text
        .replace(condition, &condition.replace("row 2", "row 3"));
    assert_eq!(
        ir.lower_patch(&full_patch(&ir.text, &moved))
            .unwrap_err()
            .code,
        "unsupported_ir_edit"
    );
}

#[test]
fn sfc_patch_rejects_structure_metadata_and_rolls_back_native_failures() {
    let doc = fixture("sfc/multi-action-two-native.xgwx");
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let row = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("sfc_row ") && l.contains("qualifier \"L\""))
        .unwrap();
    let variable = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("sfc_variable "))
        .unwrap();
    let renamed = ir
        .text
        .replace(variable, &variable.replace("name ", "renamed "));
    assert!(ir.lower_patch(&full_patch(&ir.text, &renamed)).is_err());
    for edited in [
        ir.text
            .replace(row, &row.replace("kind \"continuation\"", "kind \"step\"")),
        ir.text.replace(
            row,
            &row.replace("position none", "position \"row=999,column=1\""),
        ),
    ] {
        assert_eq!(
            store
                .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &edited), false)
                .unwrap_err()
                .code,
            "operation_failed"
        );
        assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    }
    let changed = ir
        .text
        .replace(row, &row.replace("time \"T#2s\"", "time \"T#bad\""));
    assert_eq!(
        store
            .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &changed), false)
            .unwrap_err()
            .code,
        "operation_failed"
    );
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    let changed = changed.replace(
        variable,
        &format!(
            "{} description \"changed\"",
            variable.rsplit_once(" description ").unwrap().0
        ),
    );
    assert_eq!(
        ir.lower_patch(&full_patch(&ir.text, &changed))
            .unwrap_err()
            .code,
        "unsupported_ir_edit"
    );
    let malformed = ir
        .text
        .replace(row, &row.replace("initial false", "initial maybe"));
    assert_eq!(
        ir.lower_patch(&full_patch(&ir.text, &malformed))
            .unwrap_err()
            .code,
        "invalid_patch"
    );
    assert!(!fs::read_dir(dir.path()).unwrap().any(|e| {
        e.unwrap()
            .file_name()
            .to_string_lossy()
            .contains(".xgwx-mcp-backup-")
    }));
}

#[test]
fn sfc_branch_row_fields_preserve_positions_endpoints_and_sources() {
    let mut doc = fixture("sfc/multi-action-branch-native.xgwx");
    let before = doc.program().sfc_programs()[0].blocks[0]
        .editable_rows
        .clone()
        .unwrap();
    let vars = doc.variable().sfc_variables(0).unwrap();
    let index = before.iter().position(|r| r.kind == "step").unwrap();
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let line = ir
        .text
        .lines()
        .find(|l| {
            l.trim_start()
                .starts_with(&format!("sfc_row @p0.b0.r{index} "))
        })
        .unwrap();
    let changed = line.replace(
        &format!(
            "comment {}",
            serde_json::to_string(&before[index].comment).unwrap()
        ),
        "comment \"Branch note\"",
    );
    for op in ir
        .lower_patch(&replace_patch(&ir.text, line, &changed))
        .unwrap()
    {
        apply(&mut doc, &op.name, op.args);
    }
    let after = doc.program().sfc_programs()[0].blocks[0]
        .editable_rows
        .clone()
        .unwrap();
    assert_eq!(after[index].comment, "Branch note");
    let structural = |rows: Vec<xgwx::SfcRow>| {
        rows.into_iter()
            .map(|r| {
                (
                    r.kind,
                    r.position,
                    r.branch_end,
                    r.action_code,
                    r.transition_code,
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(structural(before), structural(after));
    assert_eq!(vars, doc.variable().sfc_variables(0).unwrap());
    doc.to_verified_bytes().unwrap();
}

#[test]
fn sfc_inconsistent_shared_st_source_rolls_back() {
    let doc = fixture("sfc/multi-action-three-native.xgwx");
    let rows = doc.program().sfc_programs()[0].blocks[0]
        .editable_rows
        .clone()
        .unwrap();
    let index = rows.iter().position(|r| r.action_code.is_some()).unwrap();
    let source = rows[index].action_code.as_ref().unwrap();
    assert!(
        rows.iter()
            .filter(|r| r.action_code.as_ref() == Some(source))
            .count()
            > 1
    );
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let line = ir
        .text
        .lines()
        .find(|l| {
            l.trim_start()
                .starts_with(&format!("sfc_row @p0.b0.r{index} "))
        })
        .unwrap();
    let changed = line.replace(
        &format!("action_code {}", serde_json::to_string(source).unwrap()),
        &format!(
            "action_code {}",
            serde_json::to_string(&format!("(* one use only *)\n{source}")).unwrap()
        ),
    );
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let err = store
        .apply_patch(
            "p.xgwx",
            &rev,
            0,
            &replace_patch(&ir.text, line, &changed),
            false,
        )
        .unwrap_err();
    assert_eq!(err.code, "operation_failed");
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
}

#[test]
fn sfc_equivalent_string_escape_is_a_noop_without_backup() {
    let doc = fixture("sfc/native-loop.xgwx");
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let line = ir
        .text
        .lines()
        .find(|l| l.contains("comment \"Ready for cycle\""))
        .unwrap();
    let changed = line.replace("Ready for cycle", "\\u0052eady for cycle");
    let patch = replace_patch(&ir.text, line, &changed);
    assert!(ir.lower_patch(&patch).unwrap().is_empty());
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let saved = store.apply_patch("p.xgwx", &rev, 0, &patch, false).unwrap();
    assert_eq!(saved["changed"], false);
    assert!(saved["backup_path"].is_null());
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
}

fn sfc_author_row(id: &str, kind: &str, title: &str, initial: bool) -> String {
    format!(
        "    sfc_row {id} kind \"{kind}\" title {} comment \"\" initial {initial} action none qualifier none time none action_code none transition_code none position none branch_end none",
        serde_json::to_string(title).unwrap()
    )
}

#[test]
fn sfc_blank_chart_creation_then_clear_preserves_declarations() {
    let mut doc = fixture("sfc/new-xgi-sfc.xgwx");
    let locals = doc.variable().sfc_variables(0).unwrap();
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let chart = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("chart "))
        .unwrap();
    let authored = [
        sfc_author_row("@new-init", "step", "Idle", true),
        sfc_author_row("@new-ready", "transition", "%MX0", false),
        sfc_author_row("@new-run", "step", "Run", false),
    ]
    .join("\n");
    let edited = ir.text.replace(chart, &format!("{chart}\n{authored}"));
    let ops = ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap();
    assert_eq!(ops.len(), 1);
    assert_eq!(ops[0].name, "replace_sfc_sequence");
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    let rows = doc.program().sfc_programs()[0].blocks[0]
        .editable_rows
        .clone()
        .unwrap();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].title, "Idle");
    assert!(rows[0].initial);
    assert_eq!(rows[1].title, "%MX0");
    assert_eq!(locals, doc.variable().sfc_variables(0).unwrap());
    let ir = Projection::export(&doc, "next", 0).unwrap();
    let mut edited = ir.text.clone();
    for line in ir
        .text
        .lines()
        .filter(|l| l.trim_start().starts_with("sfc_row "))
    {
        edited = remove_line(&edited, line);
    }
    for op in ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap() {
        apply(&mut doc, &op.name, op.args);
    }
    assert!(
        doc.program().sfc_programs()[0].blocks[0]
            .editable_rows
            .as_ref()
            .unwrap()
            .is_empty()
    );
    assert_eq!(locals, doc.variable().sfc_variables(0).unwrap());
    doc.to_verified_bytes().unwrap();
}

#[test]
fn sfc_linear_insert_reorder_delete_uses_full_source_snapshots() {
    let mut doc = fixture("sfc/native-loop.xgwx");
    let original = doc.program().sfc_programs()[0].blocks[0]
        .editable_rows
        .clone()
        .unwrap();
    let locals = doc.variable().sfc_variables(0).unwrap();
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let step = ir
        .text
        .lines()
        .find(|l| l.contains("comment \"Ready for cycle\""))
        .unwrap();
    let authored = [
        sfc_author_row("@new-extra", "step", "Extra", false),
        sfc_author_row("@new-extra-ready", "transition", "%MX2", false),
    ]
    .join("\n");
    let edited = ir.text.replace(step, &format!("{authored}\n{step}"));
    for op in ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap() {
        apply(&mut doc, &op.name, op.args);
    }
    let ir = Projection::export(&doc, "next", 0).unwrap();
    let lines = ir
        .text
        .lines()
        .filter(|l| l.trim_start().starts_with("sfc_row "))
        .collect::<Vec<_>>();
    let index = lines
        .iter()
        .position(|l| l.contains("title \"Extra\""))
        .unwrap();
    let group = lines[index..index + 4].join("\n");
    let moved = [
        lines[index + 2],
        lines[index + 3],
        lines[index],
        lines[index + 1],
    ]
    .join("\n");
    let edited = ir.text.replace(&group, &moved);
    let ops = ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap();
    assert_eq!(ops.len(), 1);
    assert!(ops[0].args["patch"]["expected_rows"].is_array());
    for op in ops {
        apply(&mut doc, &op.name, op.args);
    }
    let after = doc.program().sfc_programs()[0].blocks[0]
        .editable_rows
        .clone()
        .unwrap();
    assert_eq!(after[index + 2].title, "Extra");
    let ir = Projection::export(&doc, "next", 0).unwrap();
    let mut edited = ir.text.clone();
    for line in ir.text.lines().filter(|l| {
        l.trim_start().starts_with("sfc_row ")
            && (l.contains("title \"Extra\"") || l.contains("title \"%MX2\""))
    }) {
        edited = remove_line(&edited, line);
    }
    for op in ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap() {
        apply(&mut doc, &op.name, op.args);
    }
    assert_eq!(
        original,
        doc.program().sfc_programs()[0].blocks[0]
            .editable_rows
            .clone()
            .unwrap()
    );
    assert_eq!(locals, doc.variable().sfc_variables(0).unwrap());
    doc.to_verified_bytes().unwrap();
}

#[test]
fn sfc_structural_patches_reject_branch_edits_forged_ids_and_native_invalid_rows() {
    let doc = fixture("sfc/multi-action-branch-native.xgwx");
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    let row = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("sfc_row "))
        .unwrap();
    let ops = ir
        .lower_patch(&full_patch(&ir.text, &remove_line(&ir.text, row)))
        .unwrap();
    let mut candidate = doc.clone();
    assert!(
        xgwx_mcp::operations::apply_operation(
            &mut candidate,
            "replace_sfc_sequence",
            ops[0].args.clone()
        )
        .is_err()
    );
    let doc = fixture("sfc/native-loop.xgwx");
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let chart = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("chart "))
        .unwrap();
    for (id, kind, title) in [("@p0.forged", "step", "Extra")] {
        let edited = ir.text.replace(
            chart,
            &format!("{chart}\n{}", sfc_author_row(id, kind, title, false)),
        );
        assert!(ir.lower_patch(&full_patch(&ir.text, &edited)).is_err());
    }
    let authored = sfc_author_row("@new-scope", "step", "Extra", false);
    let program = ir.text.lines().find(|l| l.starts_with("program ")).unwrap();
    for edited in [
        ir.text.replace(program, &format!("{program}\n{authored}")),
        ir.text
            .replace(chart, &format!("{chart}\n{authored}\n{authored}")),
        ir.text.replace(
            chart,
            &format!("{chart}\n{}", authored.replace(" comment \"\"", "")),
        ),
    ] {
        assert!(ir.lower_patch(&full_patch(&ir.text, &edited)).is_err());
    }
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    for row in [
        sfc_author_row("@new-bad", "transition", "%MW2", false),
        sfc_author_row("@new-branch", "parallel_split", "", false),
        sfc_author_row("@new-position", "step", "Positioned", false)
            .replace("position none", "position \"row=1,column=1\""),
        sfc_author_row("@new-initial", "step", "AnotherInitial", true),
        sfc_author_row("@new-missing", "jump", "MissingLabel", false),
    ] {
        let edited = ir.text.replace(chart, &format!("{chart}\n{row}"));
        assert_eq!(
            store
                .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &edited), false)
                .unwrap_err()
                .code,
            "operation_failed"
        );
        assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    }
    let row = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("sfc_row "))
        .unwrap();
    let malformed = ir
        .text
        .replace(row, &row.replace(" title ", " missing_title "));
    assert!(ir.lower_patch(&full_patch(&ir.text, &malformed)).is_err());
}

#[test]
fn iec_output_clear_uses_guarded_writer_and_requires_reexport() {
    let doc = fixture("function-output-wires/base.xgwx");
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let output = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("argument ") && l.contains("direction OUTPUT"))
        .unwrap();
    let cleared = format!("{} value \"\"", output.split(" value ").next().unwrap());
    let patch = replace_patch(&ir.text, output, &cleared);
    let ops = ir.lower_patch(&patch).unwrap();
    assert_eq!(ops.len(), 1);
    assert_eq!(ops[0].name, "update_iec_ld_function_operand");
    let mut expected = doc.clone();
    apply(&mut expected, &ops[0].name, ops[0].args.clone());
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    store.apply_patch("p.xgwx", &rev, 0, &patch, true).unwrap();
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    store.apply_patch("p.xgwx", &rev, 0, &patch, false).unwrap();
    assert_eq!(
        fs::read(dir.path().join("p.xgwx")).unwrap(),
        expected.to_verified_bytes().unwrap()
    );
    assert_eq!(
        store
            .apply_patch("p.xgwx", &rev, 0, &patch, false)
            .unwrap_err()
            .code,
        "stale_revision"
    );
    let function = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("function ") && l.contains("name \"ADD\""))
        .unwrap();
    let changed = ir
        .text
        .replace(output, &cleared)
        .replace(function, &function.replace("name \"ADD\"", "name \"SUB\""));
    assert_eq!(
        ir.lower_patch(&full_patch(&ir.text, &changed))
            .unwrap_err()
            .code,
        "unsupported_ir_edit"
    );
}

#[test]
fn sfc_positioned_branch_can_add_a_balanced_path_and_reject_bad_endpoint() {
    let doc = fixture("sfc/multi-action-branch-native.xgwx");
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let mut edited = ir.text.replace("branch_end 2", "branch_end 4");
    for (i, line) in ir
        .text
        .lines()
        .filter(|l| l.trim_start().starts_with("sfc_row ") && l.contains("column=2\""))
        .enumerate()
    {
        let id = line.split_whitespace().nth(1).unwrap();
        let new = line
            .replace(id, &format!("@new-path{i}"))
            .replace("column=2", "column=4")
            .replace("title \"S2\"", "title \"S3\"");
        edited = edited.replace(line, &format!("{line}\n{new}"));
    }
    let ops = ir.lower_patch(&full_patch(&ir.text, &edited)).unwrap();
    assert_eq!(ops.len(), 1);
    let mut expected = doc.clone();
    apply(&mut expected, &ops[0].name, ops[0].args.clone());
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let bad = edited.replace("branch_end 4", "branch_end 3");
    assert_eq!(
        store
            .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &bad), false)
            .unwrap_err()
            .code,
        "operation_failed"
    );
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    store
        .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &edited), true)
        .unwrap();
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    store
        .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &edited), false)
        .unwrap();
    let saved = XgwxDocument::from_path(dir.path().join("p.xgwx")).unwrap();
    assert_eq!(
        saved.variable().sfc_variables(0).unwrap(),
        doc.variable().sfc_variables(0).unwrap()
    );
    assert_eq!(
        saved.to_verified_bytes().unwrap(),
        expected.to_verified_bytes().unwrap()
    );
    assert!(
        saved.program().sfc_programs()[0].blocks[0]
            .editable_rows
            .as_ref()
            .unwrap()
            .iter()
            .any(|r| r.title == "S3")
    );
}

#[test]
fn sfc_declaration_create_update_bounds_initializer_retain_and_remove() {
    let mut doc = fixture("sfc/new-xgi-sfc.xgwx");
    let dir = tempfile::tempdir().unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let quote = |v: &str| serde_json::to_string(v).unwrap();
    for stage in 0..3 {
        let bytes = doc.to_verified_bytes().unwrap();
        let rev = revision(&bytes);
        fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
        let ir = Projection::export(&doc, &rev, 0).unwrap();
        let edited = match stage {
            0 => {
                let program = ir.text.lines().find(|l| l.starts_with("program ")).unwrap();
                let declaration = quote(r#"{"dimensions":[],"initial_value":"1","retain":false}"#);
                ir.text.replace(program, &format!("{program}\n  sfc_variable @new-counter name \"Counter\" type \"INT\" system false declaration {declaration} description \"Count items\""))
            }
            1 => {
                let line = ir
                    .text
                    .lines()
                    .find(|l| {
                        l.trim_start().starts_with("sfc_variable ")
                            && l.contains("name \"Counter\"")
                    })
                    .unwrap();
                let prefix = line
                    .split(" declaration ")
                    .next()
                    .unwrap()
                    .replace("type \"INT\"", "type \"DINT\"");
                let declaration = quote(
                    r#"{"dimensions":[{"lower":0,"upper":2}],"initial_value":"2,3,4","retain":true}"#,
                );
                ir.text.replace(
                    line,
                    &format!("{prefix} declaration {declaration} description \"Array count\""),
                )
            }
            _ => {
                let line = ir
                    .text
                    .lines()
                    .find(|l| {
                        l.trim_start().starts_with("sfc_variable ")
                            && l.contains("name \"Counter\"")
                    })
                    .unwrap();
                remove_line(&ir.text, line)
            }
        };
        let patch = full_patch(&ir.text, &edited);
        let ops = ir.lower_patch(&patch).unwrap();
        assert_eq!(ops.len(), 1);
        let mut expected = doc.clone();
        apply(&mut expected, &ops[0].name, ops[0].args.clone());
        store.apply_patch("p.xgwx", &rev, 0, &patch, true).unwrap();
        assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
        store.apply_patch("p.xgwx", &rev, 0, &patch, false).unwrap();
        doc = XgwxDocument::from_path(dir.path().join("p.xgwx")).unwrap();
        assert_eq!(
            doc.to_verified_bytes().unwrap(),
            expected.to_verified_bytes().unwrap()
        );
        let variables = doc.variable().sfc_variables(0).unwrap();
        if stage == 1 {
            let v = variables.iter().find(|v| v.name == "Counter").unwrap();
            assert_eq!(v.data_type, "DINT");
            let d = v.declaration.as_ref().unwrap();
            assert!(d.retain);
            assert_eq!(d.dimensions[0].upper, 2);
        } else if stage == 2 {
            assert!(!variables.iter().any(|v| v.name == "Counter"));
        }
    }
}

#[test]
fn sfc_referenced_and_system_declarations_retain_native_guards() {
    let doc = fixture("sfc/multi-action-branch-native.xgwx");
    let bytes = doc.to_verified_bytes().unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let variable = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("sfc_variable ") && l.contains("name \"Value_BOOL\""))
        .unwrap();
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    for changed in [
        remove_line(&ir.text, variable),
        ir.text
            .replace(variable, &variable.replace("type \"BOOL\"", "type \"INT\"")),
    ] {
        assert_eq!(
            store
                .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &changed), false)
                .unwrap_err()
                .code,
            "operation_failed"
        );
        assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
        assert!(!fs::read_dir(dir.path()).unwrap().any(|entry| {
            entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .contains("backup")
        }));
    }
    let system = ir
        .text
        .lines()
        .find(|l| l.trim_start().starts_with("sfc_variable ") && l.contains("system true"))
        .unwrap();
    let changed = ir.text.replace(
        system,
        &format!(
            "{} description \"Changed\"",
            system.split(" description ").next().unwrap()
        ),
    );
    assert_eq!(
        ir.lower_patch(&full_patch(&ir.text, &changed))
            .unwrap_err()
            .code,
        "unsupported_ir_edit"
    );
}

fn row_edit(doc: &mut XgwxDocument, statement: &str) {
    let ir = Projection::export(doc, "revision", 0).unwrap();
    let mut after = ir.text.clone();
    after.truncate(after.len() - 2);
    after.push_str(statement);
    after.push_str("\n}\n");
    let ops = ir.lower_patch(&full_patch(&ir.text, &after)).unwrap();
    assert_eq!(ops.len(), 1);
    xgwx_mcp::operations::apply_operation(doc, &ops[0].name, ops[0].args.clone()).unwrap();
    *doc = XgwxDocument::parse(&doc.to_bytes().unwrap()).unwrap();
}
#[test]
fn iec_row_and_rung_lifecycle_roundtrip() {
    let mut doc = fixture("empty-projects/new-xgi.xgwx");
    row_edit(
        &mut doc,
        "  rung_row @new-first at row 0 contact NO \"%MX0\" coil OUT \"%MX1\"",
    );
    row_edit(&mut doc, "  blank_row @new-space after row 0");
    row_edit(
        &mut doc,
        "  rung_row @new-second at row 1 contact NC \"%MX2\" coil SET \"%MX3\"",
    );
    row_edit(&mut doc, "  blank_row @new-space after row 1");
    row_edit(
        &mut doc,
        "  rung_row @new-third at row 2 contact RISING \"%MX4\" coil RESET \"%MX5\"",
    );
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    assert!(ir.text.contains("  rung_row @p0.rung1 at row 1\n"));
    let after = ir.text.replace("  rung_row @p0.rung1 at row 1\n", "");
    let ops = ir.lower_patch(&full_patch(&ir.text, &after)).unwrap();
    xgwx_mcp::operations::apply_operation(&mut doc, &ops[0].name, ops[0].args.clone()).unwrap();
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    assert!(ir.text.contains("  blank_row @p0.blank1 at row 1\n"));
    let after = ir.text.replace("  blank_row @p0.blank1 at row 1\n", "");
    let ops = ir.lower_patch(&full_patch(&ir.text, &after)).unwrap();
    xgwx_mcp::operations::apply_operation(&mut doc, &ops[0].name, ops[0].args.clone()).unwrap();
    let rows = doc
        .program()
        .ladder_program(0)
        .unwrap()
        .unwrap()
        .iec_row_frames()
        .unwrap();
    assert_eq!(
        rows.iter().map(|r| r.row_index).collect::<Vec<_>>(),
        vec![0, 1]
    );
    let final_ir = Projection::export(&doc, "revision", 0).unwrap();
    assert!(final_ir.text.contains("RESET"));
    assert!(!final_ir.text.contains("operand \"%MX2\""));
}
#[test]
fn row_lifecycle_rejects_mixed_forged_multiple_and_readonly_edits() {
    let mut doc = fixture("empty-projects/new-xgi.xgwx");
    row_edit(
        &mut doc,
        "  rung_row @new-first at row 0 contact NO \"%MX0\" coil OUT \"%MX1\"",
    );
    row_edit(&mut doc, "  blank_row @new-space after row 0");
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    assert!(!ir.text.contains("rung_row @p0.rung0")); // Native writer cannot remove the sole group.
    for statement in [
        "  blank_row @p0.fake after row 0",
        "  blank_row @new-a after row 0\n  blank_row @new-b after row 0",
        "  rung_row @new-a at row 1 contact INVALID \"%MX0\" coil OUT \"%MX1\"",
    ] {
        let after = format!("{}{statement}\n}}\n", &ir.text[..ir.text.len() - 2]);
        assert!(ir.lower_patch(&full_patch(&ir.text, &after)).is_err());
    }
    let marker = ir
        .text
        .lines()
        .find(|l| l.contains("blank_row") && l.contains("readonly"))
        .unwrap();
    assert!(
        ir.lower_patch(&replace_patch(&ir.text, marker, ""))
            .is_err()
    );
    let after = format!(
        "{}  blank_row @new-a after row 0\n}}\n",
        &ir.text[..ir.text.len() - 2]
    )
    .replace("revision", "forged");
    assert!(ir.lower_patch(&full_patch(&ir.text, &after)).is_err());
}
#[test]
fn xgk_row_insertion_uses_native_writer() {
    let mut doc = fixture("elements.xgwx");
    let mut expected = doc.clone();
    xgwx_mcp::operations::apply_operation(
        &mut expected,
        "insert_ladder_row",
        serde_json::json!({"program_index":0,"raw_y":0}),
    )
    .unwrap();
    row_edit(&mut doc, "  blank_row @new-space before y 0");
    assert_eq!(doc.to_bytes().unwrap(), expected.to_bytes().unwrap());
}

#[test]
fn iec_blank_shift_preserves_or_rejects_complex_records_with_native_guards() {
    for name in [
        "ladder-wire-delete/iec-horizontal-native.xgwx",
        "function-output-wires/base.xgwx",
    ] {
        let mut doc = fixture(name);
        let rows = doc
            .program()
            .ladder_program(0)
            .unwrap()
            .unwrap()
            .iec_row_frames()
            .unwrap();
        let first = rows[0].row_index;
        let mut expected = doc.clone();
        let native = xgwx_mcp::operations::apply_operation(
            &mut expected,
            "insert_iec_ld_blank_row",
            serde_json::json!({"program_index":0,"after_row_index":first}),
        );
        if let Err(reason) = native {
            let ir = Projection::export(&doc, "revision", 0).unwrap();
            let after = format!(
                "{}  blank_row @new-space after row {first}\n}}\n",
                &ir.text[..ir.text.len() - 2]
            );
            let ops = ir.lower_patch(&full_patch(&ir.text, &after)).unwrap();
            let bytes = doc.to_bytes().unwrap();
            let rejected =
                xgwx_mcp::operations::apply_operation(&mut doc, &ops[0].name, ops[0].args.clone())
                    .unwrap_err();
            assert_eq!(rejected, reason, "{name}");
            assert_eq!(doc.to_bytes().unwrap(), bytes, "{name}");
            continue;
        }
        row_edit(
            &mut doc,
            &format!("  blank_row @new-space after row {first}"),
        );
        assert_eq!(
            doc.to_bytes().unwrap(),
            expected.to_bytes().unwrap(),
            "{name}"
        );
    }
}
#[test]
fn iec_rung_lifecycle_preview_backup_stale_and_native_rollback() {
    let doc = fixture("empty-projects/new-xgi.xgwx");
    let dir = tempfile::tempdir().unwrap();
    let bytes = doc.to_verified_bytes().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let rev = revision(&bytes);
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let new = format!(
        "{}  rung_row @new-first at row 0 contact NO \"%MX0\" coil OUT \"%MX1\"\n}}\n",
        &ir.text[..ir.text.len() - 2]
    );
    let patch = full_patch(&ir.text, &new);
    let preview = store.apply_patch("p.xgwx", &rev, 0, &patch, true).unwrap();
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    let saved = store.apply_patch("p.xgwx", &rev, 0, &patch, false).unwrap();
    assert_eq!(preview["revision_after"], saved["revision_after"]);
    assert_eq!(
        fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
        bytes
    );
    assert_eq!(
        store
            .apply_patch("p.xgwx", &rev, 0, &patch, false)
            .unwrap_err()
            .code,
        "stale_revision"
    );
    let bytes = fs::read(dir.path().join("p.xgwx")).unwrap();
    let doc = XgwxDocument::parse(&bytes).unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let invalid = format!(
        "{}  rung_row @new-overlap at row 0 contact NO \"%MX0\" coil OUT \"%MX1\"\n}}\n",
        &ir.text[..ir.text.len() - 2]
    );
    let files = fs::read_dir(dir.path()).unwrap().count();
    assert_eq!(
        store
            .apply_patch("p.xgwx", &rev, 0, &full_patch(&ir.text, &invalid), false)
            .unwrap_err()
            .code,
        "operation_failed"
    );
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), files);
}

fn captured_site(doc: &XgwxDocument, prefix: &str, mode: &str) -> String {
    let ir = Projection::export(doc, "revision", 0).unwrap();
    ir.text
        .lines()
        .find(|l| l.trim_start().starts_with(prefix) && l.contains(mode))
        .unwrap_or_else(|| panic!("missing {prefix} {mode}\n{}", ir.text))
        .split_whitespace()
        .nth(1)
        .unwrap()
        .into()
}
#[test]
fn iec_captured_contact_wire_repair_and_parallel_authoring_roundtrip() {
    let mut doc = fixture("empty-projects/new-xgi.xgwx");
    row_edit(
        &mut doc,
        "  rung_row @new-first at row 0 contact NO \"%MX0\" coil OUT \"%MX1\"",
    );
    row_edit(&mut doc, "  blank_row @new-space after row 0");
    let site = captured_site(&doc, "parallel_site ", "readonly");
    row_edit(
        &mut doc,
        &format!("  parallel_contact @new-branch site {site} kind NC operand \"%MX2\""),
    );
    let site = captured_site(&doc, "contact_site ", " WIRE ");
    row_edit(
        &mut doc,
        &format!("  contact_insert @new-middle site {site} kind RISING operand \"%MX3\" x 7"),
    );
    let program = doc.program().ladder_program(0).unwrap().unwrap();
    let record = program
        .iec_record_frames()
        .unwrap()
        .into_iter()
        .find(|r| {
            matches!(r.kind, xgwx::IecRecordKind::Contact(_)) && program.data[r.offset + 5] == 7
        })
        .unwrap();
    apply(
        &mut doc,
        "delete_iec_ld_contact",
        serde_json::json!({"program_index":0,"contact_offset":record.offset,"expected_raw_x":7,"expected_contact_kind":"RISING","expected_variable":"%MX3"}),
    );
    let site = captured_site(&doc, "wire_gap ", "readonly");
    row_edit(
        &mut doc,
        &format!("  wire_restore @new-restore site {site}"),
    );
    // Fixture-only flags mirror captured short-wire cell geometry; production
    // never changes native bytes to make an unsupported site editable.
    let program = doc.program().ladder_program(0).unwrap().unwrap();
    let wire = program
        .iec_record_frames()
        .unwrap()
        .into_iter()
        .find(|r| r.kind == xgwx::IecRecordKind::ShortWire && program.data[r.offset + 5] == 7)
        .unwrap();
    let mut data = program.data.clone();
    data[wire.offset + 9..wire.offset + 15].copy_from_slice(&[0, 0, 4, 0, 0, 0]);
    let mut doc = fixture_with_iec_payload(doc, data);
    let site = captured_site(&doc, "contact_site ", " SHORT_WIRE ");
    row_edit(
        &mut doc,
        &format!("  contact_insert @new-short site {site} kind NEGATED_FALLING operand \"%MX4\""),
    );
    assert!(
        Projection::export(&doc, "revision", 0)
            .unwrap()
            .text
            .contains("NEGATED_FALLING \"%MX4\"")
    );
    // This generated branch is not the native leading-gap layout. Its head
    // stays guarded; no leading site may be invented from row coordinates.
    assert!(
        !Projection::export(&doc, "revision", 0)
            .unwrap()
            .text
            .contains(" LEADING readonly")
    );
}
#[test]
fn iec_site_authoring_rejects_forged_sites_mixed_mutations_and_native_invalid_operands() {
    let mut doc = fixture("empty-projects/new-xgi.xgwx");
    row_edit(
        &mut doc,
        "  rung_row @new-first at row 0 contact NO \"%MX0\" coil OUT \"%MX1\"",
    );
    let site = captured_site(&doc, "contact_site ", " WIRE ");
    let ir = Projection::export(&doc, "revision", 0).unwrap();
    for statement in [
        "  contact_insert @new-a site @fake kind NO operand \"%MX0\" x 4".to_string(),
        format!("  contact_insert @new-a site {site} kind NO operand \"%MX0\""),
        format!("  contact_insert @new-a site {site} kind UNKNOWN operand \"%MX0\" x 4"),
        format!("  wire_restore @new-a site {site}"),
        format!("  contact_insert @p0.forged site {site} kind NO operand \"%MX0\" x 4"),
        format!(
            "  contact_insert @new-a site {site} kind NO operand \"%MX0\" x 4\n  contact_insert @new-b site {site} kind NO operand \"%MX0\" x 7"
        ),
    ] {
        let after = format!("{}{statement}\n}}\n", &ir.text[..ir.text.len() - 2]);
        assert!(
            ir.lower_patch(&full_patch(&ir.text, &after)).is_err(),
            "{statement}"
        );
    }
    let after = format!(
        "{}  contact_insert @new-a site {site} kind NO operand \"%MX0\" x 4\n}}\n",
        &ir.text[..ir.text.len() - 2]
    )
    .replace("revision", "forged");
    assert!(ir.lower_patch(&full_patch(&ir.text, &after)).is_err());
    let bytes = doc.to_bytes().unwrap();
    for (value, x) in [("%MW0", 4), ("%MX0", 5), ("%MX0", 94)] {
        let after = format!(
            "{}  contact_insert @new-a site {site} kind NO operand \"{value}\" x {x}\n}}\n",
            &ir.text[..ir.text.len() - 2]
        );
        let ops = ir.lower_patch(&full_patch(&ir.text, &after)).unwrap();
        assert!(
            xgwx_mcp::operations::apply_operation(&mut doc, &ops[0].name, ops[0].args.clone())
                .is_err()
        );
        assert_eq!(doc.to_bytes().unwrap(), bytes);
    }
}

#[test]
fn iec_comment_row_copies_captured_template_and_retains_native_guards() {
    let mut doc = fixture("empty-projects/new-xgi.xgwx");
    row_edit(
        &mut doc,
        "  contact @new-first at(group=0,row=0,x=1) NO \"%MX0\"",
    );
    let mut doc = captured_iec_comment_fixture(doc);
    row_edit(&mut doc, "  blank_row @new-space after row 0");
    row_edit(
        &mut doc,
        "  comment_row @new-note at row 1 text \"New comment\"",
    );
    assert!(
        Projection::export(&doc, "revision", 0)
            .unwrap()
            .text
            .contains("IEC \"New comment\"")
    );
}

fn fixture_with_iec_payload(mut doc: XgwxDocument, data: Vec<u8>) -> XgwxDocument {
    use base64::Engine;
    use std::io::Write;
    let program = doc.program().ladder_program(0).unwrap().unwrap();
    let encoded = if program.compressed {
        let mut encoder = bzip2::write::BzEncoder::new(Vec::new(), bzip2::Compression::best());
        encoder.write_all(&data).unwrap();
        encoder.finish().unwrap()
    } else {
        data
    };
    let xml = doc.container().xml().to_owned();
    let parsed = roxmltree::Document::parse(&xml).unwrap();
    let node = parsed
        .descendants()
        .find(|n| n.has_tag_name("ProgramData"))
        .unwrap();
    let range = node.children().find(|n| n.is_text()).unwrap().range();
    let mut xml = xml.clone();
    xml.replace_range(
        range,
        &base64::engine::general_purpose::STANDARD.encode(encoded),
    );
    doc.container_mut().replace_xml(xml).unwrap();
    XgwxDocument::parse(&doc.to_bytes().unwrap()).unwrap()
}

#[test]
fn iec_site_edit_preview_save_backup_and_native_failure_are_atomic() {
    let mut doc = fixture("empty-projects/new-xgi.xgwx");
    row_edit(
        &mut doc,
        "  rung_row @new-first at row 0 contact NO \"%MX0\" coil OUT \"%MX1\"",
    );
    let dir = tempfile::tempdir().unwrap();
    let bytes = doc.to_verified_bytes().unwrap();
    fs::write(dir.path().join("p.xgwx"), &bytes).unwrap();
    let store = FileStore::new(dir.path().into(), vec![]).unwrap();
    let rev = revision(&bytes);
    let ir = Projection::export(&doc, &rev, 0).unwrap();
    let site = captured_site(&doc, "contact_site ", " WIRE ");
    let text = format!(
        "{}  contact_insert @new-interlock site {site} kind NC operand \"%MX2\" x 7\n}}\n",
        &ir.text[..ir.text.len() - 2]
    );
    let patch = full_patch(&ir.text, &text);
    let preview = store.apply_patch("p.xgwx", &rev, 0, &patch, true).unwrap();
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    let saved = store.apply_patch("p.xgwx", &rev, 0, &patch, false).unwrap();
    assert_eq!(preview["revision_after"], saved["revision_after"]);
    assert_eq!(
        fs::read(saved["backup_path"].as_str().unwrap()).unwrap(),
        bytes
    );
    let bytes = fs::read(dir.path().join("p.xgwx")).unwrap();
    let doc = XgwxDocument::parse(&bytes).unwrap();
    let ir = Projection::export(&doc, &revision(&bytes), 0).unwrap();
    let site = captured_site(&doc, "contact_site ", " WIRE ");
    let invalid = format!(
        "{}  contact_insert @new-invalid site {site} kind NO operand \"%MW0\" x 4\n}}\n",
        &ir.text[..ir.text.len() - 2]
    );
    let files = fs::read_dir(dir.path()).unwrap().count();
    assert_eq!(
        store
            .apply_patch(
                "p.xgwx",
                &revision(&bytes),
                0,
                &full_patch(&ir.text, &invalid),
                false
            )
            .unwrap_err()
            .code,
        "operation_failed"
    );
    assert_eq!(fs::read(dir.path().join("p.xgwx")).unwrap(), bytes);
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), files);
}
