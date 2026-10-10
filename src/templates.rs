//! Embedded, native blank projects; no runtime access to sibling fixtures.
use crate::storage::{StoreError, revision};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use xgwx::XgwxDocument;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProjectTemplate {
    /// Native XGK ladder project, initially XGK-CPUSN.
    XgkLd,
    /// Native IEC ladder project, initially XGI-CPUE.
    XgiLd,
    /// Native empty XGI SFC project, initially XGI-CPUE.
    XgiSfc,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum ProgramLanguage {
    Ld,
    Sfc,
}
impl ProjectTemplate {
    pub fn language(self) -> ProgramLanguage {
        match self {
            Self::XgkLd | Self::XgiLd => ProgramLanguage::Ld,
            Self::XgiSfc => ProgramLanguage::Sfc,
        }
    }
}
/// Resolve user-facing CPU and language choices to a captured native blank.
pub fn select(
    template: Option<ProjectTemplate>,
    cpu: Option<&str>,
    language: Option<ProgramLanguage>,
) -> Result<ProjectTemplate, StoreError> {
    let model = cpu
        .map(|model| {
            xgwx::cpu_catalog()
                .iter()
                .find(|c| c.model.eq_ignore_ascii_case(model))
                .ok_or_else(|| StoreError::new("cpu_selection_failed", "unknown CPU model"))
        })
        .transpose()?;
    let selected = if let Some(template) = template {
        if language.is_some_and(|language| language != template.language()) {
            return Err(StoreError::new(
                "invalid_arguments",
                "template conflicts with program_language",
            ));
        }
        template
    } else {
        match (
            model.map(|c| c.family),
            language.unwrap_or(ProgramLanguage::Ld),
        ) {
            (Some("XGK") | None, ProgramLanguage::Ld) => ProjectTemplate::XgkLd,
            (Some("XGI"), ProgramLanguage::Ld) => ProjectTemplate::XgiLd,
            (Some("XGI") | None, ProgramLanguage::Sfc) => ProjectTemplate::XgiSfc,
            _ => {
                return Err(StoreError::new(
                    "unsupported_combination",
                    "no native blank template for this CPU family and program language",
                ));
            }
        }
    };
    Ok(selected)
}

impl ProjectTemplate {
    pub fn bytes(self) -> &'static [u8] {
        match self {
            Self::XgkLd => include_bytes!("../templates/xgk_ld.xgwx"),
            Self::XgiLd => include_bytes!("../templates/xgi_ld.xgwx"),
            Self::XgiSfc => include_bytes!("../templates/xgi_sfc.xgwx"),
        }
    }
    pub fn document(self) -> Result<XgwxDocument, StoreError> {
        XgwxDocument::parse(self.bytes()).map_err(|e| StoreError::new("template_error", e))
    }
    pub fn metadata(self) -> Value {
        let id = match self {
            Self::XgkLd => "xgk_ld",
            Self::XgiLd => "xgi_ld",
            Self::XgiSfc => "xgi_sfc",
        };
        json!({"id":id,"program_language":self.language(),"sha256":revision(self.bytes()),"project_name":"NewProject","plc_name":"LSPLC","program_name":"NewProgram","identity_policy":"preserve_native_template_identities"})
    }
}
pub fn cpu_model(doc: &XgwxDocument) -> Option<&'static str> {
    let code = doc.configurations().first()?.type_code?;
    xgwx::cpu_catalog()
        .iter()
        .find(|c| c.type_code == code)
        .map(|c| c.model)
}
pub fn list() -> Result<Value, StoreError> {
    let mut items = vec![];
    for template in [
        ProjectTemplate::XgkLd,
        ProjectTemplate::XgiLd,
        ProjectTemplate::XgiSfc,
    ] {
        let doc = template.document()?;
        let mut item = template.metadata();
        item["default_cpu_model"] = json!(cpu_model(&doc));
        item["selectable_cpu_models"] = json!(
            xgwx::cpu_catalog()
                .iter()
                .filter(|c| doc.clone().select_cpu(c.model).is_ok())
                .map(|c| c.model)
                .collect::<Vec<_>>()
        );
        items.push(item);
    }
    Ok(
        json!({"templates":items,"creation_choices":items.iter().flat_map(|item| item["selectable_cpu_models"].as_array().unwrap().iter().map(|model| json!({"cpu_model":model,"program_language":item["program_language"],"template":item["id"]}))).collect::<Vec<_>>(),"limitations":"Internal template identities are preserved. Project and initial program names can be chosen at creation. Only LD and SFC blanks are currently supported. CPU selection follows existing library migration guards; supported models are probed against each blank template. Blank programs may report incomplete-program diagnostics. Local creation and verification do not establish native XG5000 acceptance."}),
    )
}
