//! Revision-scoped property documents, lowered only through existing writers.
#[cfg(not(feature = "libxgwx-namespaced-api"))]
use crate::library_compat::DocumentNamespacesCompat;
use crate::library_compat::module_summaries;
use crate::storage::{EditOperation, StoreError};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use xgwx::XgwxDocument;

#[derive(Clone)]
struct Binding {
    operation: &'static str,
    args: Value,
    path: Vec<String>,
    structural: bool,
}
impl Binding {
    fn new(operation: &'static str, args: Value, path: &[&str]) -> Self {
        Self {
            operation,
            args,
            path: path.iter().map(|s| (*s).into()).collect(),
            structural: false,
        }
    }
    fn structural(mut self) -> Self {
        self.structural = true;
        self
    }
}
pub struct Projection {
    pub text: String,
    bindings: BTreeMap<usize, Binding>,
}
fn invalid(s: impl ToString) -> StoreError {
    StoreError::new("unsupported_ir_edit", s)
}
fn literal(v: &Value) -> String {
    if v.is_null() {
        "none".into()
    } else {
        v.to_string()
    }
}
impl Projection {
    fn line(&mut self, s: impl AsRef<str>) {
        self.text.push_str(s.as_ref());
        self.text.push('\n');
    }
    fn field(&mut self, key: &str, value: Value, binding: Option<Binding>) {
        let index = self.text.lines().count();
        let editable = binding.is_some() && !value.is_null();
        self.line(format!(
            "  {} {key} = {}",
            if editable { "edit" } else { "readonly" },
            literal(&value)
        ));
        if editable {
            self.bindings.insert(index, binding.unwrap());
        }
    }
    pub fn export(
        doc: &XgwxDocument,
        revision: &str,
        section: &str,
        program_index: Option<usize>,
    ) -> Result<Self, StoreError> {
        if program_index.is_some_and(|i| i >= doc.program().programs().len()) {
            return Err(invalid("program_index is absent"));
        }
        if section != "variables" && program_index.is_some() {
            return Err(invalid(
                "program_index applies only to program or variables documents",
            ));
        }
        let mut p = Self {
            text: String::new(),
            bindings: BTreeMap::new(),
        };
        p.line(format!(
            "xgwx-properties 1 revision {} section {section} program {}",
            json!(revision),
            program_index.map(|i| i.to_string()).unwrap_or("all".into())
        ));
        match section {
            "modules" => {
                for (i, m) in module_summaries(doc).iter().enumerate() {
                    p.line(format!(
                        "module @m{i} base {} slot {} {{",
                        literal(&json!(m.base)),
                        literal(&json!(m.slot))
                    ));
                    p.field("id", json!(m.id), None);
                    p.field("sub_type", json!(m.sub_type), None);
                    let model = xgwx::xgk_module_catalog()
                        .iter()
                        .find(|entry| Some(entry.id) == m.id && Some(entry.sub_type) == m.sub_type)
                        .map(|entry| entry.model);
                    p.field("model", json!(model), None);
                    p.field("native_name", json!(m.name), None);
                    let pos = m.base.zip(m.slot);
                    p.field(
                        "comment",
                        json!(m.comment),
                        pos.map(|(base, slot)| {
                            Binding::new(
                                "update_module",
                                json!({"base":base,"slot":slot,"patch":{}}),
                                &["patch", "comment"],
                            )
                        }),
                    );
                    if let Some((base, slot)) = pos {
                        match doc.module_option_values(base, slot) {
                            Ok(values) => {
                                for v in values {
                                    p.field(&format!("option.{}.{}",v.key,v.index),json!(v.value),Some(Binding::new("set_module_option",json!({"base":base,"slot":slot,"key":v.key,"index":v.index}), &["value"])));
                                }
                            }
                            Err(e) => p.field("options_status", json!(e.to_string()), None),
                        }
                    }
                    p.line("}");
                }
            }
            "variables" => {
                match doc.variable().variables() {
                    Ok(vars) => {
                        for (i, v) in vars.iter().enumerate() {
                            p.line(format!("variable @global.v{i} {{"));
                            for (key, value) in [
                                ("name", json!(v.name)),
                                ("address_area", json!(v.address_area)),
                                ("address_number", json!(v.address_number)),
                                ("data_type", json!(v.data_type)),
                                ("description", json!(v.description)),
                            ] {
                                p.field(
                                    key,
                                    value,
                                    Some(Binding::new(
                                        "update_variable",
                                        json!({"variable_index":i,"patch":{}}),
                                        &["patch", key],
                                    )),
                                );
                            }
                            p.field("source_ref", json!(v.source_ref), None);
                            p.field("range", json!(v.range), None);
                            p.line("}");
                        }
                    }
                    Err(e) => p.field("global_status", json!(e.to_string()), None),
                }
                for (program, vars) in doc
                    .variable()
                    .iec_local_symbols()
                    .into_iter()
                    .enumerate()
                    .filter(|(i, _)| program_index.is_none_or(|p| p == *i))
                {
                    match vars {
                        Ok(vars) => {
                            for (index, v) in vars.iter().enumerate() {
                                p.line(format!("variable @p{program}.v{index} {{"));
                                let args = json!({"program_index":program,"symbol_index":index,"expected_name":v.name});
                                p.field(
                                    "name",
                                    json!(v.name),
                                    Some(
                                        Binding::new(
                                            "rename_iec_local_symbol",
                                            args.clone(),
                                            &["replacement"],
                                        )
                                        .structural(),
                                    ),
                                );
                                let mut type_args = args.clone();
                                type_args["expected_type"] = json!(v.data_type);
                                p.field(
                                    "data_type",
                                    json!(v.data_type),
                                    (!v.is_instance).then(|| {
                                        Binding::new(
                                            "update_iec_local_symbol_type",
                                            type_args,
                                            &["replacement"],
                                        )
                                        .structural()
                                    }),
                                );
                                let mut desc_args = args;
                                desc_args["expected_description"] =
                                    json!(v.description.clone().unwrap_or_default());
                                p.field(
                                    "description",
                                    json!(v.description.clone().unwrap_or_default()),
                                    Some(Binding::new(
                                        "update_iec_local_symbol_description",
                                        desc_args,
                                        &["replacement"],
                                    )),
                                );
                                p.field("address", json!(v.address), None);
                                p.field("storage_class", json!(v.storage_class), None);
                                p.field("allocation_number", json!(v.allocation_number), None);
                                p.field("allocation_width", json!(v.allocation_width), None);
                                p.field("is_instance", json!(v.is_instance), None);
                                p.line("}");
                            }
                        }
                        Err(e) => p.field(
                            &format!("local_status.{program}"),
                            json!(e.to_string()),
                            None,
                        ),
                    }
                }
            }
            "networks" => {
                for (n, net) in doc.networks().iter().enumerate() {
                    p.line(format!("network @n{n} {{"));
                    p.field("name",json!(net.name),Some(Binding::new("edit_browser_network",json!({"patch":{"network_index":n,"module":false,"base":null,"slot":null,"field":"name","expected_value":net.name}}), &["patch","replacement"])));
                    p.field("type", json!(net.type_name), None);
                    p.field("network_type", json!(net.network_type), None);
                    p.line("}");
                    for (i, m) in net.modules.iter().enumerate() {
                        p.line(format!(
                            "network_module @n{n}.m{i} base {} slot {} {{",
                            literal(&json!(m.base)),
                            literal(&json!(m.slot))
                        ));
                        p.field("id", json!(m.id), None);
                        p.field("type", json!(m.type_name), None);
                        p.field("name", json!(m.name), None);
                        for (key, field, value) in [
                            ("config_name", "configName", json!(m.config_name)),
                            ("alias", "alias", json!(m.alias)),
                            ("description", "description", json!(m.description)),
                        ] {
                            p.field(key,value.clone(),m.base.zip(m.slot).map(|(base,slot)|Binding::new("edit_browser_network",json!({"patch":{"network_index":n,"module":true,"base":base,"slot":slot,"field":field,"expected_value":value}}), &["patch","replacement"])));
                        }
                        p.line("}");
                    }
                }
                for (i, f) in doc.network().fenet_config_infos().iter().enumerate() {
                    p.line(format!(
                        "fenet @f{i} base {} slot {} {{",
                        literal(&json!(f.base)),
                        literal(&json!(f.slot))
                    ));
                    let pos = f.base.zip(f.slot);
                    for (field, ip) in [
                        ("ipAddress", &f.ip_address),
                        ("subnet", &f.subnet),
                        ("gateway", &f.gateway),
                        ("dns", &f.dns),
                        ("ipAddress2", &f.ip_address2),
                        ("subnet2", &f.subnet2),
                        ("gateway2", &f.gateway2),
                        ("dns2", &f.dns2),
                    ] {
                        let value = json!(ip.as_ref().map(|ip| ip.address.clone()));
                        p.field(field,value.clone(),pos.map(|(base,slot)|Binding::new("edit_fenet_field",json!({"patch":{"base":base,"slot":slot,"field":field,"expected_value":value}}), &["patch","replacement"])));
                    }
                    for (field, attr) in [
                        ("stationNo", "StationNo"),
                        ("glofaSocketCount", "GlofaSocketCnt"),
                        ("rcvWaitTime", "RcvWaitTime"),
                        ("clientWaitTime", "ClientWaitTime"),
                        ("driverType", "DriverType"),
                        ("dhcp", "Dhcp"),
                        ("dhcp2", "Dhcp2"),
                    ] {
                        let value = f
                            .attributes
                            .iter()
                            .find(|a| a.name == attr)
                            .and_then(|a| a.value.parse::<u32>().ok())
                            .map(|n| n.to_string());
                        p.field(field,json!(value),pos.map(|(base,slot)|Binding::new("edit_fenet_field",json!({"patch":{"base":base,"slot":slot,"field":field,"expected_value":value}}), &["patch","replacement"])));
                    }
                    p.line("}");
                }
                for (i, c) in doc.network().cnet_config_infos().iter().enumerate() {
                    for (port, v) in c.ports.iter().enumerate() {
                        p.line(format!(
                            "cnet_port @c{i}.p{port} base {} slot {} {{",
                            literal(&json!(c.base)),
                            literal(&json!(c.slot))
                        ));
                        for (field, attr) in [
                            ("stationNo", "StationNo"),
                            ("modeRaw", "Mode"),
                            ("bps", "Bps"),
                            ("dataBitRaw", "DataBit"),
                            ("stopBitRaw", "StopBit"),
                            ("parityRaw", "Parity"),
                            ("rxTimeout", "RxTimeOut"),
                            ("charTimeout", "CharTimeOut"),
                            ("driverType", "DriverType"),
                            ("requestDelayTime", "RequestDelayTime"),
                            ("parityErrorIgnore", "ParityErrorIgnore"),
                            ("terminatingResister", "TerminatingResister"),
                            ("repeater", "Repeater"),
                        ] {
                            let value = v
                                .attributes
                                .iter()
                                .find(|a| a.name == attr)
                                .and_then(|a| a.value.parse::<u32>().ok())
                                .map(|n| n.to_string());
                            p.field(field,json!(value),c.base.zip(c.slot).map(|(base,slot)|Binding::new("edit_cnet_settings",json!({"patch":{"base":base,"slot":slot,"changes":[{"port_index":port,"field":field,"expected_value":value}]}}), &["patch","changes","0","replacement"])));
                        }
                        p.line("}");
                    }
                }
            }
            _ => {
                return Err(StoreError::new(
                    "invalid_arguments",
                    "section must be program, modules, variables or networks",
                ));
            }
        }
        Ok(p)
    }
    pub fn lower_patch(&self, patch: &str) -> Result<Vec<EditOperation>, StoreError> {
        let changed = crate::text_ir::apply_unified_patch(&self.text, patch)?;
        let old = self.text.lines().collect::<Vec<_>>();
        let new = changed.lines().collect::<Vec<_>>();
        if old.len() != new.len() {
            return Err(invalid(
                "property object/field creation and deletion are unsupported; use apply_edits",
            ));
        }
        let mut ops = Vec::new();
        let mut structural = false;
        let mut globals = BTreeMap::<usize, Value>::new();
        let mut cnet = BTreeMap::<(u64, u64), Value>::new();
        for (i, (a, b)) in old.iter().zip(&new).enumerate() {
            if a == b {
                continue;
            }
            let binding = self.bindings.get(&i).ok_or_else(|| {
                invalid("identities, order and readonly fields must remain unchanged")
            })?;
            let (prefix, original) = a
                .split_once(" = ")
                .ok_or_else(|| invalid("invalid property"))?;
            let (next, value) = b
                .split_once(" = ")
                .ok_or_else(|| invalid("invalid property"))?;
            if prefix != next {
                return Err(invalid(
                    "property name and edit marker must remain unchanged",
                ));
            }
            let original: Value =
                serde_json::from_str(original).map_err(|_| invalid("invalid source literal"))?;
            let value: Value = serde_json::from_str(value)
                .map_err(|_| invalid("use a quoted string or unsigned decimal value"))?;
            if value == original {
                continue;
            }
            if !(original.is_string() && value.is_string()
                || original.is_u64() && value.as_u64().is_some_and(|v| v <= u32::MAX as u64))
            {
                return Err(invalid(
                    "replacement must retain the property's scalar type",
                ));
            }
            let mut args = binding.args.clone();
            let mut target = &mut args;
            for key in &binding.path[..binding.path.len() - 1] {
                target = if let Ok(index) = key.parse::<usize>() {
                    &mut target[index]
                } else {
                    &mut target[key]
                };
            }
            target[binding.path.last().unwrap()] = value;
            structural |= binding.structural;
            if binding.operation == "update_variable" {
                let index = args["variable_index"].as_u64().unwrap() as usize;
                globals
                    .entry(index)
                    .or_insert_with(|| json!({"variable_index":index,"patch":{}}))["patch"]
                    .as_object_mut()
                    .unwrap()
                    .extend(args["patch"].as_object().unwrap().clone());
            } else if binding.operation == "edit_cnet_settings" {
                let pos = (
                    args["patch"]["base"].as_u64().unwrap(),
                    args["patch"]["slot"].as_u64().unwrap(),
                );
                cnet.entry(pos).or_insert_with(||json!({"patch":{"base":pos.0,"slot":pos.1,"changes":[]}}))["patch"]["changes"].as_array_mut().unwrap().extend(args["patch"]["changes"].as_array().unwrap().clone());
            } else {
                ops.push(EditOperation {
                    name: binding.operation.into(),
                    args,
                });
            }
        }
        ops.extend(globals.into_values().map(|args| EditOperation {
            name: "update_variable".into(),
            args,
        }));
        ops.extend(cnet.into_values().map(|args| EditOperation {
            name: "edit_cnet_settings".into(),
            args,
        }));
        if structural && ops.len() != 1 {
            return Err(invalid(
                "IEC local rename/type changes must be the sole mutation; re-export afterward",
            ));
        }
        Ok(ops)
    }
}
