//! Compatibility creation profiles backed by the library project generator.
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
    /// XGK IL is a view of the native ladder blank.
    XgkIl,
    /// Native IEC ladder project, initially XGI-CPUE.
    XgiLd,
    /// Native empty XGI SFC project, initially XGI-CPUE.
    XgiSfc,
    /// Library-derived blank standalone XGI ST project.
    XgiSt,
    /// Library-derived blank standalone IEC IL project.
    XgiIl,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum ProgramLanguage {
    Ld,
    Sfc,
    St,
    Il,
}
impl ProjectTemplate {
    pub fn language(self) -> ProgramLanguage {
        match self {
            Self::XgkLd | Self::XgiLd => ProgramLanguage::Ld,
            Self::XgiSfc => ProgramLanguage::Sfc,
            Self::XgiSt => ProgramLanguage::St,
            Self::XgiIl | Self::XgkIl => ProgramLanguage::Il,
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
            (Some("XGI") | None, ProgramLanguage::St) => ProjectTemplate::XgiSt,
            (Some("XGK"), ProgramLanguage::Il) => ProjectTemplate::XgkIl,
            (Some("XGI") | None, ProgramLanguage::Il) => ProjectTemplate::XgiIl,
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
    pub fn document(self) -> Result<XgwxDocument, StoreError> {
        let (cpu, language) = match self {
            Self::XgkLd => ("XGK-CPUSN", "LD"),
            Self::XgkIl => ("XGK-CPUSN", "IL"),
            Self::XgiLd => ("XGI-CPUE", "LD"),
            Self::XgiSfc => ("XGI-CPUE", "SFC"),
            Self::XgiSt => ("XGI-CPUE", "ST"),
            Self::XgiIl => ("XGI-CPUE", "IL"),
        };
        xgwx::create_project(cpu, language).map_err(|e| StoreError::new("template_error", e))
    }
    pub fn metadata(self) -> Result<Value, StoreError> {
        let id = match self {
            Self::XgkLd => "xgk_ld",
            Self::XgkIl => "xgk_il",
            Self::XgiLd => "xgi_ld",
            Self::XgiSfc => "xgi_sfc",
            Self::XgiSt => "xgi_st",
            Self::XgiIl => "xgi_il",
        };
        Ok(
            json!({"id":id,"program_language":self.language(),"sha256":revision(&self.document()?.to_verified_bytes().map_err(|e| StoreError::new("template_error", e))?),"project_name":"NewProject","plc_name":"LSPLC","program_name":"NewProgram","identity_policy":"preserve_library_default_identities","generator":"libxgwx"}),
        )
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
        ProjectTemplate::XgkIl,
        ProjectTemplate::XgiLd,
        ProjectTemplate::XgiSfc,
        ProjectTemplate::XgiSt,
        ProjectTemplate::XgiIl,
    ] {
        let doc = template.document()?;
        let mut item = template.metadata()?;
        item["storage_language"] = json!(if template == ProjectTemplate::XgkIl {
            "LD"
        } else {
            match template.language() {
                ProgramLanguage::Ld => "LD",
                ProgramLanguage::Sfc => "SFC",
                ProgramLanguage::St => "ST",
                ProgramLanguage::Il => "IL",
            }
        });
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
        json!({"templates":items,"creation_choices":items.iter().flat_map(|item| item["selectable_cpu_models"].as_array().unwrap().iter().map(|model| json!({"cpu_model":model,"program_language":item["program_language"],"template":item["id"]}))).collect::<Vec<_>>(),"limitations":"Library default identities are preserved. Project and initial program names can be chosen at creation. LD, SFC, ST and IL blanks are supported for advertised CPU choices. CPU selection follows existing library migration guards; supported models are probed against each blank template. Blank programs may report incomplete-program diagnostics. Local creation and verification do not establish native XG5000 acceptance."}),
    )
}
