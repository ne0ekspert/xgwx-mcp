//! Read-only operand classification; native writers remain the validation authority.
use serde_json::json;
use xgwx::{IEC_SYSTEM_BOOL_VARIABLES, IecFunctionBlock, IecLocalSymbol, VariableSummary};

#[derive(Clone)]
pub(crate) struct Declaration {
    pub id: String,
    pub name: String,
    pub address: Option<String>,
    pub data_type: Option<String>,
    pub instance: bool,
    pub type_reference: Option<String>,
    pub local: bool,
}
pub(crate) struct Context {
    declarations: Vec<Declaration>,
    fields: Vec<(String, String, String, Option<String>)>,
    iec: bool,
}
#[derive(Debug, PartialEq)]
pub(crate) struct Operand {
    pub kind: &'static str,
    pub status: &'static str,
    pub reason: &'static str,
    pub symbol: Option<String>,
    pub address: Option<String>,
    pub candidates: Vec<String>,
    pub member: Option<String>,
    pub pin: Option<String>,
    pub data_type: Option<String>,
    pub literal_type: Option<String>,
}
impl Operand {
    fn new(kind: &'static str, status: &'static str, reason: &'static str) -> Self {
        Self {
            kind,
            status,
            reason,
            symbol: None,
            address: None,
            candidates: vec![],
            member: None,
            pin: None,
            data_type: None,
            literal_type: None,
        }
    }
}
impl Context {
    pub fn new(
        index: usize,
        iec: bool,
        globals: &[VariableSummary],
        locals: &[IecLocalSymbol],
        functions: &[IecFunctionBlock],
    ) -> Self {
        let mut declarations = globals
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                v.name.as_ref().map(|name| Declaration {
                    id: format!("@s{i}"),
                    name: name.clone(),
                    address: v.address.clone(),
                    data_type: v.data_type.clone(),
                    instance: false,
                    type_reference: None,
                    local: false,
                })
            })
            .collect::<Vec<_>>();
        declarations.extend(locals.iter().enumerate().map(|(i, v)| Declaration {
            id: format!("@p{index}.s{i}"),
            name: v.name.clone(),
            address: v.address.clone(),
            data_type: v.data_type.clone(),
            instance: v.is_instance,
            type_reference: v.type_reference.clone(),
            local: true,
        }));
        let mut fields = vec![];
        for f in functions {
            let Some(instance) = f.instance.as_ref() else {
                continue;
            };
            let symbols = declarations
                .iter()
                .filter(|d| {
                    d.local
                        && d.instance
                        && d.name.eq_ignore_ascii_case(&instance.value)
                        && d.type_reference
                            .as_deref()
                            .is_some_and(|t| t.eq_ignore_ascii_case(&f.name.value))
                })
                .collect::<Vec<_>>();
            if symbols.len() != 1 {
                continue;
            }
            let owner = format!("@p{index}.f{}", f.record_offset);
            let mut pins = vec![
                (format!("{owner}.control_in"), &f.control_input),
                (format!("{owner}.control_out"), &f.control_output),
            ];
            pins.extend(
                f.pins
                    .iter()
                    .enumerate()
                    .map(|(i, p)| (format!("{owner}.data{i}"), p)),
            );
            for (id, p) in pins {
                fields.push((
                    symbols[0].id.clone(),
                    p.name.value.clone(),
                    id,
                    p.data_type
                        .filter(|_| p.data_type_mask.is_power_of_two())
                        .map(str::to_owned),
                ));
            }
        }
        Self {
            declarations,
            fields,
            iec,
        }
    }
    fn names(&self, name: &str) -> Vec<&Declaration> {
        let locals = self
            .declarations
            .iter()
            .filter(|d| d.local && d.name.eq_ignore_ascii_case(name))
            .collect::<Vec<_>>();
        if !locals.is_empty() {
            locals
        } else {
            self.declarations
                .iter()
                .filter(|d| !d.local && d.name.eq_ignore_ascii_case(name))
                .collect()
        }
    }
    pub fn classify(&self, text: &str) -> Operand {
        let value = text.trim();
        if value.is_empty() {
            return Operand::new("UNRESOLVED", "unresolved", "empty_operand");
        }
        let names = self.names(value);
        if !names.is_empty() {
            let mut result = Operand::new(
                "SYMBOL",
                if names.len() == 1 {
                    "resolved"
                } else {
                    "ambiguous"
                },
                "declaration_name",
            );
            result.candidates = names.iter().map(|d| d.id.clone()).collect();
            if names.len() == 1 {
                result.symbol = Some(names[0].id.clone());
                result.address = names[0].address.clone().filter(|a| !a.is_empty());
                result.data_type = names[0].data_type.clone();
            }
            return result;
        }
        let address_matches = self
            .declarations
            .iter()
            .filter(|d| {
                d.address
                    .as_deref()
                    .is_some_and(|a| !a.is_empty() && a.eq_ignore_ascii_case(value))
            })
            .map(|d| d.id.clone())
            .collect::<Vec<_>>();
        if (self.iec && iec_address(value)) || (!self.iec && !address_matches.is_empty()) {
            let mut result = Operand::new("DIRECT_ADDRESS", "syntax_only", "address_spelling");
            result.address = Some(value.to_owned());
            result.candidates = address_matches;
            return result;
        }
        if self.iec
            && IEC_SYSTEM_BOOL_VARIABLES
                .iter()
                .any(|(n, _)| n.eq_ignore_ascii_case(value))
        {
            let mut r = Operand::new("SYSTEM_SYMBOL", "resolved", "system_catalog");
            r.data_type = Some("BOOL".into());
            return r;
        }
        if let Some((base, member)) = value.split_once('.') {
            let names = self.names(base);
            if !names.is_empty() {
                let mut r = Operand::new("FIELD", "unresolved", "field_not_decoded");
                r.member = Some(member.into());
                r.candidates = names.iter().map(|d| d.id.clone()).collect();
                if names.len() != 1 {
                    r.status = "ambiguous";
                    r.reason = "ambiguous_base";
                    return r;
                }
                r.symbol = Some(names[0].id.clone());
                let pins = self
                    .fields
                    .iter()
                    .filter(|(id, name, _, _)| {
                        id == &names[0].id && name.eq_ignore_ascii_case(member)
                    })
                    .collect::<Vec<_>>();
                if !pins.is_empty() {
                    r.status = "resolved";
                    r.reason = "native_instance_pin";
                    if pins.len() == 1 {
                        r.pin = Some(pins[0].2.clone());
                    }
                    if pins.iter().all(|p| p.3 == pins[0].3) {
                        r.data_type = pins[0].3.clone();
                    }
                }
                return r;
            }
        }
        let literal = if value.eq_ignore_ascii_case("TRUE") || value.eq_ignore_ascii_case("FALSE") {
            Some(("BOOL", "syntax_only"))
        } else if value.parse::<i128>().is_ok() {
            Some(("UNTYPED_INTEGER", "syntax_only"))
        } else if value.contains('.') && value.parse::<f64>().is_ok_and(f64::is_finite) {
            Some(("UNTYPED_REAL", "syntax_only"))
        } else if self.iec {
            let upper = value.to_ascii_uppercase();
            [
                ("T#", "TIME"),
                ("TIME#", "TIME"),
                ("D#", "DATE"),
                ("DATE#", "DATE"),
                ("TOD#", "TIME_OF_DAY"),
                ("TIME_OF_DAY#", "TIME_OF_DAY"),
                ("DT#", "DATE_AND_TIME"),
                ("DATE_AND_TIME#", "DATE_AND_TIME"),
            ]
            .into_iter()
            .find(|(prefix, _)| upper.starts_with(prefix) && upper.len() > prefix.len())
            .map(|(_, t)| (t, "unvalidated"))
        } else {
            None
        };
        if let Some((kind, status)) = literal {
            let mut r = Operand::new(
                "LITERAL",
                status,
                if status == "unvalidated" {
                    "typed_literal_prefix"
                } else {
                    "literal_syntax"
                },
            );
            r.literal_type = Some(kind.into());
            return r;
        }
        let reason = if value.starts_with('%') {
            "address_syntax_unrecognized"
        } else if value.contains('#') {
            "literal_syntax_unclassified"
        } else if value.contains('.') {
            "field_or_expression_not_resolved"
        } else if value
            .chars()
            .enumerate()
            .all(|(i, c)| c == '_' || c.is_alphabetic() || (i > 0 && c.is_numeric()))
        {
            "declaration_unavailable"
        } else {
            "expression_not_resolved"
        };
        Operand::new("UNRESOLVED", "unresolved", reason)
    }
    pub fn render(&self, owner: &str, ordinal: usize, text: &str) -> String {
        let r = self.classify(text);
        let optional = |s: Option<&str>| {
            s.map(|s| json!(s).to_string())
                .unwrap_or_else(|| "none".into())
        };
        format!(
            "  operand {owner}.operand{ordinal} owner {} index {ordinal} text {} address {} category {} status {} symbol {} candidates {} member {} pin {} data_type {} literal_type {} reason {} readonly\n",
            json!(owner),
            json!(text),
            optional(r.address.as_deref()),
            r.kind,
            json!(r.status),
            optional(r.symbol.as_deref()),
            json!(r.candidates),
            optional(r.member.as_deref()),
            optional(r.pin.as_deref()),
            optional(r.data_type.as_deref()),
            optional(r.literal_type.as_deref()),
            json!(r.reason)
        )
    }
}
fn iec_address(text: &str) -> bool {
    let upper = text.to_ascii_uppercase();
    let Some(rest) = upper.strip_prefix('%') else {
        return false;
    };
    let mut chars = rest.chars();
    let Some(area) = chars.next() else {
        return false;
    };
    if !matches!(
        area,
        'I' | 'Q' | 'M' | 'U' | 'W' | 'F' | 'K' | 'L' | 'R' | 'A'
    ) || !matches!(chars.next(), Some('X' | 'B' | 'W' | 'D' | 'L'))
    {
        return false;
    }
    let parts = chars.as_str().split('.').collect::<Vec<_>>();
    ((matches!(area, 'I' | 'Q' | 'U') && matches!(parts.len(), 1 | 3))
        || (!matches!(area, 'I' | 'Q' | 'U') && parts.len() == 1))
        && parts.iter().all(|s| {
            !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) && s.parse::<u32>().is_ok()
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn decl(id: &str, name: &str, local: bool, address: Option<&str>) -> Declaration {
        Declaration {
            id: id.into(),
            name: name.into(),
            local,
            address: address.map(str::to_owned),
            data_type: Some("BOOL".into()),
            instance: false,
            type_reference: None,
        }
    }
    fn context() -> Context {
        Context {
            iec: true,
            declarations: vec![
                decl("@s0", "ON", false, Some("%MX8")),
                decl("@p0.s0", "ON", true, Some("%MX8")),
                decl("@p0.s1", "한글", true, None),
                decl("@p0.s2", "Timer", true, None),
            ],
            fields: vec![(
                "@p0.s2".into(),
                "Q".into(),
                "@p0.f10.control_out".into(),
                Some("BOOL".into()),
            )],
        }
    }
    #[test]
    fn scope_ambiguity_and_address_candidates_are_not_conflated() {
        let mut c = context();
        let r = c.classify("on");
        assert_eq!(r.kind, "SYMBOL");
        assert_eq!(r.symbol.as_deref(), Some("@p0.s0"));
        assert_eq!(r.candidates, vec!["@p0.s0"]);
        assert_eq!(r.address.as_deref(), Some("%MX8"));
        assert!(
            c.render("@p0.e1", 0, "ON")
                .contains("text \"ON\" address \"%MX8\"")
        );
        assert_eq!(c.classify("한글").address, None);
        assert_eq!(c.classify("한글").symbol.as_deref(), Some("@p0.s1"));
        let address = c.classify("%mx8");
        assert_eq!(address.kind, "DIRECT_ADDRESS");
        assert_eq!(address.symbol, None);
        assert_eq!(address.address.as_deref(), Some("%mx8"));
        assert_eq!(address.candidates, vec!["@s0", "@p0.s0"]);
        assert_eq!(address.status, "syntax_only");
        c.declarations.push(decl("@p0.s3", "ON", true, None));
        let r = c.classify("ON");
        assert_eq!(r.status, "ambiguous");
        assert_eq!(r.symbol, None);
        assert_eq!(r.address, None);
        assert_eq!(r.candidates, vec!["@p0.s0", "@p0.s3"]);
    }
    #[test]
    fn native_fields_and_literals_preserve_unknowns() {
        let c = context();
        let r = c.classify("timer.q");
        assert_eq!(r.kind, "FIELD");
        assert_eq!(r.status, "resolved");
        assert_eq!(r.symbol.as_deref(), Some("@p0.s2"));
        assert_eq!(r.pin.as_deref(), Some("@p0.f10.control_out"));
        assert_eq!(r.address, None);
        assert_eq!(c.classify("Timer.Unknown").status, "unresolved");
        assert_eq!(c.classify("Timer.Q.extra").status, "unresolved");
        assert_eq!(c.classify("Missing.Q").kind, "UNRESOLVED");
        assert_eq!(c.classify("_ON").kind, "SYSTEM_SYMBOL");
        assert_eq!(c.classify("_NotKnown").kind, "UNRESOLVED");
        for (text, kind) in [
            ("TRUE", "BOOL"),
            ("-10", "UNTYPED_INTEGER"),
            ("1.25", "UNTYPED_REAL"),
            ("T#15s", "TIME"),
        ] {
            let r = c.classify(text);
            assert_eq!(r.kind, "LITERAL");
            assert_eq!(r.literal_type.as_deref(), Some(kind));
            assert_eq!(r.data_type, None);
        }
        assert_eq!(c.classify("T#bogus").status, "unvalidated");
        for text in [
            "",
            "ON+1",
            "a[2]",
            "NaN",
            "_unknown",
            "%MX",
            "%MX-1",
            "%MX1.2",
            "%IX1.2",
            "%IX0..1",
            "%MX4294967296",
        ] {
            assert_eq!(c.classify(text).kind, "UNRESOLVED", "{text}");
        }
        for text in ["%MX8", "%IX0.1.2", "%QX10", "%MW700", "%MD1"] {
            assert_eq!(c.classify(text).kind, "DIRECT_ADDRESS", "{text}");
        }
    }
}
