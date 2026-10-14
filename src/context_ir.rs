//! Frozen project/execution context, without inferred scheduling or memory aliases.
use serde_json::json;
use xgwx::XgwxDocument;
fn optional<T: serde::Serialize>(value: Option<T>) -> String {
    value
        .map(|v| json!(v).to_string())
        .unwrap_or_else(|| "none".into())
}
pub(crate) fn export(doc: &XgwxDocument, program: Option<(usize, &str)>) -> String {
    export_context(doc, program, false)
}
pub(crate) fn export_properties(doc: &XgwxDocument, section: &str) -> String {
    export_context(doc, None, matches!(section, "project" | "modules"))
}
fn export_context(doc: &XgwxDocument, program: Option<(usize, &str)>, compact_cpu: bool) -> String {
    let mut text = String::new();
    let configs = doc.configuration().list();
    text.push_str(&format!("  context @context configuration_count {} task_count {} memory_model \"not_decoded\" execution_order \"not_decoded\" readonly\n", configs.len(),doc.program().tasks().len()));
    for (i, c) in configs.iter().enumerate() {
        let cpu = xgwx::cpu_catalog()
            .iter()
            .find(|cpu| Some(cpu.type_code) == c.type_code);
        let status = json!(if cpu.is_some() {
            "catalog_match"
        } else {
            "unknown"
        });
        if compact_cpu {
            text.push_str(&format!(
                "  cpu @cpu{i} model {} status {status} readonly\n",
                optional(cpu.map(|c| c.model))
            ));
        } else {
            text.push_str(&format!(
                "  cpu @cpu{i} type_code {} model {} family {} status {status} readonly\n",
                optional(c.type_code),
                optional(cpu.map(|c| c.model)),
                optional(cpu.map(|c| c.family))
            ));
        }
    }
    let tasks = doc.program().tasks();
    for (i, t) in tasks.iter().enumerate() {
        let type_code = if compact_cpu {
            String::new()
        } else {
            format!(" type_code {}", optional(t.type_code))
        };
        text.push_str(&format!("  task @t{i} document_index {i} name {} task_index {}{type_code} kind {} priority {} device {} device_type {} bit_condition {} word_condition {} word_value {} attributes {} readonly\n",
            optional(t.name.as_deref()),optional(t.task_index),optional(t.kind),optional(t.priority),
            optional(t.device.as_deref()),optional(t.device_type),optional(t.bit_condition),optional(t.word_condition),optional(t.word_value),
            json!(serde_json::to_string(&t.attributes.iter().map(|a|(&a.name,&a.value)).collect::<Vec<_>>()).unwrap())));
    }
    if let Some((index, language)) = program {
        if let Some(p) = doc.program().programs().get(index) {
            let matches = tasks
                .iter()
                .enumerate()
                .filter(|(_, t)| {
                    p.task
                        .as_ref()
                        .is_some_and(|name| t.name.as_ref() == Some(name))
                })
                .collect::<Vec<_>>();
            let task = if matches.len() == 1 {
                Some(format!("@t{}", matches[0].0))
            } else {
                None
            };
            let status = if p.task.as_deref().is_none_or(str::is_empty) {
                "unassigned"
            } else if matches.len() == 1 {
                "resolved"
            } else if matches.len() > 1 {
                "ambiguous"
            } else {
                "task_unavailable"
            };
            text.push_str(&format!("  program_context @p{index}.context program {} document_index {index} language {} native_kind {} task_name {} task {} task_status {} execution_order \"not_decoded\" readonly\n",
                json!(format!("@p{index}")),json!(language),optional(p.kind),optional(p.task.as_deref()),optional(task.as_deref()),json!(status)));
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unknown_cpu_missing_task_and_quoted_names_stay_explicit() {
        let mut doc = XgwxDocument::from_path(
            crate::test_support::fixture_root().join("fixtures/empty-projects/new-xgi.xgwx"),
        )
        .unwrap();
        let configuration = doc.configuration().list()[0].clone();
        let task = doc.program().programs()[0].task.clone().unwrap();
        let xml = doc
            .container()
            .xml()
            .replacen(
                &format!("Type=\"{}\"", configuration.type_code.unwrap()),
                "Type=\"4294967295\"",
                1,
            )
            .replace(&format!("Task=\"{task}\""), "Task=\"missing&quot;task\"");
        doc.container_mut().replace_xml(xml).unwrap();
        let output = export(&doc, Some((0, "LD")));
        assert!(output.contains("type_code 4294967295 model none family none status \"unknown\""));
        assert!(output.contains("task_status \"task_unavailable\""));
        assert!(output.contains("execution_order \"not_decoded\""));
        assert!(output.contains(r#"task_name "missing\"task""#));
        for line in output.lines() {
            crate::text_ir::lex(line).unwrap();
        }
    }
    #[test]
    fn duplicate_task_names_are_ambiguous_without_selecting_a_task() {
        let mut doc = XgwxDocument::from_path(
            crate::test_support::fixture_root().join("fixtures/empty-projects/new-xgi.xgwx"),
        )
        .unwrap();
        let xml = doc.container().xml().to_owned();
        let parsed = roxmltree::Document::parse(&xml).unwrap();
        let task = parsed
            .descendants()
            .find(|n| n.has_tag_name("Task"))
            .unwrap();
        let range = task.range();
        let copied = xml[range.clone()].to_owned();
        let mut changed = xml.clone();
        changed.insert_str(range.end, &copied);
        doc.container_mut().replace_xml(changed).unwrap();
        let text = export(&doc, Some((0, "LD")));
        assert!(text.contains("task none task_status \"ambiguous\""));
    }
}
