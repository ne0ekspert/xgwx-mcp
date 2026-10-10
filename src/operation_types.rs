//! Server-owned strict snake_case writer DTOs. Library serialization features are not required.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub enum LadderEditKind {
    NormallyOpen,
    NormallyClosed,
    AddressedRisingPulse,
    AddressedFallingPulse,
    AddressedRisingPulseNot,
    AddressedFallingPulseNot,
    Inverse,
    RisingPulse,
    FallingPulse,
    Output,
    InverseOutput,
    Set,
    Reset,
    RisingPulseOutput,
    FallingPulseOutput,
}

impl From<LadderEditKind> for xgwx::LadderEditKind {
    fn from(value: LadderEditKind) -> Self {
        match value {
            LadderEditKind::NormallyOpen => xgwx::LadderEditKind::NormallyOpen,
            LadderEditKind::NormallyClosed => xgwx::LadderEditKind::NormallyClosed,
            LadderEditKind::AddressedRisingPulse => xgwx::LadderEditKind::AddressedRisingPulse,
            LadderEditKind::AddressedFallingPulse => xgwx::LadderEditKind::AddressedFallingPulse,
            LadderEditKind::AddressedRisingPulseNot => {
                xgwx::LadderEditKind::AddressedRisingPulseNot
            }
            LadderEditKind::AddressedFallingPulseNot => {
                xgwx::LadderEditKind::AddressedFallingPulseNot
            }
            LadderEditKind::Inverse => xgwx::LadderEditKind::Inverse,
            LadderEditKind::RisingPulse => xgwx::LadderEditKind::RisingPulse,
            LadderEditKind::FallingPulse => xgwx::LadderEditKind::FallingPulse,
            LadderEditKind::Output => xgwx::LadderEditKind::Output,
            LadderEditKind::InverseOutput => xgwx::LadderEditKind::InverseOutput,
            LadderEditKind::Set => xgwx::LadderEditKind::Set,
            LadderEditKind::Reset => xgwx::LadderEditKind::Reset,
            LadderEditKind::RisingPulseOutput => xgwx::LadderEditKind::RisingPulseOutput,
            LadderEditKind::FallingPulseOutput => xgwx::LadderEditKind::FallingPulseOutput,
        }
    }
}

impl From<xgwx::LadderEditKind> for LadderEditKind {
    fn from(value: xgwx::LadderEditKind) -> Self {
        match value {
            xgwx::LadderEditKind::NormallyOpen => LadderEditKind::NormallyOpen,
            xgwx::LadderEditKind::NormallyClosed => LadderEditKind::NormallyClosed,
            xgwx::LadderEditKind::AddressedRisingPulse => LadderEditKind::AddressedRisingPulse,
            xgwx::LadderEditKind::AddressedFallingPulse => LadderEditKind::AddressedFallingPulse,
            xgwx::LadderEditKind::AddressedRisingPulseNot => {
                LadderEditKind::AddressedRisingPulseNot
            }
            xgwx::LadderEditKind::AddressedFallingPulseNot => {
                LadderEditKind::AddressedFallingPulseNot
            }
            xgwx::LadderEditKind::Inverse => LadderEditKind::Inverse,
            xgwx::LadderEditKind::RisingPulse => LadderEditKind::RisingPulse,
            xgwx::LadderEditKind::FallingPulse => LadderEditKind::FallingPulse,
            xgwx::LadderEditKind::Output => LadderEditKind::Output,
            xgwx::LadderEditKind::InverseOutput => LadderEditKind::InverseOutput,
            xgwx::LadderEditKind::Set => LadderEditKind::Set,
            xgwx::LadderEditKind::Reset => LadderEditKind::Reset,
            xgwx::LadderEditKind::RisingPulseOutput => LadderEditKind::RisingPulseOutput,
            xgwx::LadderEditKind::FallingPulseOutput => LadderEditKind::FallingPulseOutput,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub enum LadderCommentKind {
    Rung,
    Output,
}

impl From<LadderCommentKind> for xgwx::LadderCommentKind {
    fn from(value: LadderCommentKind) -> Self {
        match value {
            LadderCommentKind::Rung => xgwx::LadderCommentKind::Rung,
            LadderCommentKind::Output => xgwx::LadderCommentKind::Output,
        }
    }
}

impl From<xgwx::LadderCommentKind> for LadderCommentKind {
    fn from(value: xgwx::LadderCommentKind) -> Self {
        match value {
            xgwx::LadderCommentKind::Rung => LadderCommentKind::Rung,
            xgwx::LadderCommentKind::Output => LadderCommentKind::Output,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub enum ModuleInputFilter {
    Default,
    Ms1,
    Ms3,
    Ms5,
    Ms10,
    Ms20,
    Ms70,
    Ms100,
    Unknown(u8),
}

impl From<ModuleInputFilter> for xgwx::ModuleInputFilter {
    fn from(value: ModuleInputFilter) -> Self {
        match value {
            ModuleInputFilter::Default => xgwx::ModuleInputFilter::Default,
            ModuleInputFilter::Ms1 => xgwx::ModuleInputFilter::Ms1,
            ModuleInputFilter::Ms3 => xgwx::ModuleInputFilter::Ms3,
            ModuleInputFilter::Ms5 => xgwx::ModuleInputFilter::Ms5,
            ModuleInputFilter::Ms10 => xgwx::ModuleInputFilter::Ms10,
            ModuleInputFilter::Ms20 => xgwx::ModuleInputFilter::Ms20,
            ModuleInputFilter::Ms70 => xgwx::ModuleInputFilter::Ms70,
            ModuleInputFilter::Ms100 => xgwx::ModuleInputFilter::Ms100,
            ModuleInputFilter::Unknown(value) => xgwx::ModuleInputFilter::Unknown(value),
        }
    }
}

impl From<xgwx::ModuleInputFilter> for ModuleInputFilter {
    fn from(value: xgwx::ModuleInputFilter) -> Self {
        match value {
            xgwx::ModuleInputFilter::Default => ModuleInputFilter::Default,
            xgwx::ModuleInputFilter::Ms1 => ModuleInputFilter::Ms1,
            xgwx::ModuleInputFilter::Ms3 => ModuleInputFilter::Ms3,
            xgwx::ModuleInputFilter::Ms5 => ModuleInputFilter::Ms5,
            xgwx::ModuleInputFilter::Ms10 => ModuleInputFilter::Ms10,
            xgwx::ModuleInputFilter::Ms20 => ModuleInputFilter::Ms20,
            xgwx::ModuleInputFilter::Ms70 => ModuleInputFilter::Ms70,
            xgwx::ModuleInputFilter::Ms100 => ModuleInputFilter::Ms100,
            xgwx::ModuleInputFilter::Unknown(value) => ModuleInputFilter::Unknown(value),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ModulePatch {
    pub id: Option<u32>,
    pub sub_type: Option<u32>,
    pub name: Option<String>,
    pub comment: Option<String>,
    pub details: Option<String>,
}

impl From<ModulePatch> for xgwx::ModulePatch {
    fn from(value: ModulePatch) -> Self {
        Self {
            id: value.id,
            sub_type: value.sub_type,
            name: value.name,
            comment: value.comment,
            details: value.details,
        }
    }
}

impl From<xgwx::ModulePatch> for ModulePatch {
    fn from(value: xgwx::ModulePatch) -> Self {
        Self {
            id: value.id,
            sub_type: value.sub_type,
            name: value.name,
            comment: value.comment,
            details: value.details,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProgramPatch {
    pub name: Option<String>,
    pub task: Option<String>,
    pub version: Option<u32>,
    pub kind: Option<u32>,
    pub instance_name: Option<String>,
    pub comment: Option<String>,
}

impl From<ProgramPatch> for xgwx::ProgramPatch {
    fn from(value: ProgramPatch) -> Self {
        Self {
            name: value.name,
            task: value.task,
            version: value.version,
            kind: value.kind,
            instance_name: value.instance_name,
            comment: value.comment,
        }
    }
}

impl From<xgwx::ProgramPatch> for ProgramPatch {
    fn from(value: xgwx::ProgramPatch) -> Self {
        Self {
            name: value.name,
            task: value.task,
            version: value.version,
            kind: value.kind,
            instance_name: value.instance_name,
            comment: value.comment,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VariablePatch {
    pub name: Option<String>,
    pub address_area: Option<String>,
    pub address_number: Option<u32>,
    pub data_type: Option<String>,
    pub description: Option<String>,
}

impl From<VariablePatch> for xgwx::VariablePatch {
    fn from(value: VariablePatch) -> Self {
        Self {
            name: value.name,
            address_area: value.address_area,
            address_number: value.address_number,
            data_type: value.data_type,
            description: value.description,
        }
    }
}

impl From<xgwx::VariablePatch> for VariablePatch {
    fn from(value: xgwx::VariablePatch) -> Self {
        Self {
            name: value.name,
            address_area: value.address_area,
            address_number: value.address_number,
            data_type: value.data_type,
            description: value.description,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct NetworkPatch {
    pub name: Option<String>,
    pub type_name: Option<String>,
    pub network_type: Option<String>,
}

impl From<NetworkPatch> for xgwx::NetworkPatch {
    fn from(value: NetworkPatch) -> Self {
        Self {
            name: value.name,
            type_name: value.type_name,
            network_type: value.network_type,
        }
    }
}

impl From<xgwx::NetworkPatch> for NetworkPatch {
    fn from(value: xgwx::NetworkPatch) -> Self {
        Self {
            name: value.name,
            type_name: value.type_name,
            network_type: value.network_type,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct NetworkModulePatch {
    pub config_name: Option<String>,
    pub alias: Option<String>,
    pub description: Option<String>,
}

impl From<NetworkModulePatch> for xgwx::NetworkModulePatch {
    fn from(value: NetworkModulePatch) -> Self {
        Self {
            config_name: value.config_name,
            alias: value.alias,
            description: value.description,
        }
    }
}

impl From<xgwx::NetworkModulePatch> for NetworkModulePatch {
    fn from(value: xgwx::NetworkModulePatch) -> Self {
        Self {
            config_name: value.config_name,
            alias: value.alias,
            description: value.description,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BrowserNetworkPatch {
    pub network_index: usize,
    pub module: bool,
    pub base: Option<u32>,
    pub slot: Option<u32>,
    pub field: String,
    pub expected_value: String,
    pub replacement: String,
}

impl From<BrowserNetworkPatch> for xgwx::BrowserNetworkPatch {
    fn from(value: BrowserNetworkPatch) -> Self {
        Self {
            network_index: value.network_index,
            module: value.module,
            base: value.base,
            slot: value.slot,
            field: value.field,
            expected_value: value.expected_value,
            replacement: value.replacement,
        }
    }
}

impl From<xgwx::BrowserNetworkPatch> for BrowserNetworkPatch {
    fn from(value: xgwx::BrowserNetworkPatch) -> Self {
        Self {
            network_index: value.network_index,
            module: value.module,
            base: value.base,
            slot: value.slot,
            field: value.field,
            expected_value: value.expected_value,
            replacement: value.replacement,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FenetFieldPatch {
    pub base: u32,
    pub slot: u32,
    pub field: String,
    pub expected_value: String,
    pub replacement: String,
}

impl From<FenetFieldPatch> for xgwx::FenetFieldPatch {
    fn from(value: FenetFieldPatch) -> Self {
        Self {
            base: value.base,
            slot: value.slot,
            field: value.field,
            expected_value: value.expected_value,
            replacement: value.replacement,
        }
    }
}

impl From<xgwx::FenetFieldPatch> for FenetFieldPatch {
    fn from(value: xgwx::FenetFieldPatch) -> Self {
        Self {
            base: value.base,
            slot: value.slot,
            field: value.field,
            expected_value: value.expected_value,
            replacement: value.replacement,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BrowserHardwarePatch {
    pub operation: String,
    pub base: u32,
    pub slot: Option<u32>,
    pub expected_value: String,
    pub replacement: String,
    pub key: Option<String>,
    pub index: Option<u32>,
}

impl From<BrowserHardwarePatch> for xgwx::BrowserHardwarePatch {
    fn from(value: BrowserHardwarePatch) -> Self {
        Self {
            operation: value.operation,
            base: value.base,
            slot: value.slot,
            expected_value: value.expected_value,
            replacement: value.replacement,
            key: value.key,
            index: value.index,
        }
    }
}

impl From<xgwx::BrowserHardwarePatch> for BrowserHardwarePatch {
    fn from(value: xgwx::BrowserHardwarePatch) -> Self {
        Self {
            operation: value.operation,
            base: value.base,
            slot: value.slot,
            expected_value: value.expected_value,
            replacement: value.replacement,
            key: value.key,
            index: value.index,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BrowserIecPatch {
    pub operation: String,
    pub offset: usize,
    pub expected_kind: String,
    pub expected_value: String,
    pub replacement: String,
}

impl From<BrowserIecPatch> for xgwx::BrowserIecPatch {
    fn from(value: BrowserIecPatch) -> Self {
        Self {
            operation: value.operation,
            offset: value.offset,
            expected_kind: value.expected_kind,
            expected_value: value.expected_value,
            replacement: value.replacement,
        }
    }
}

impl From<xgwx::BrowserIecPatch> for BrowserIecPatch {
    fn from(value: xgwx::BrowserIecPatch) -> Self {
        Self {
            operation: value.operation,
            offset: value.offset,
            expected_kind: value.expected_kind,
            expected_value: value.expected_value,
            replacement: value.replacement,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct NewProgram {
    pub name: String,
    pub language: String,
    pub object_id: String,
    pub symbol_id: String,
}

impl From<NewProgram> for xgwx::NewProgram {
    fn from(value: NewProgram) -> Self {
        Self {
            name: value.name,
            language: value.language,
            object_id: value.object_id,
            symbol_id: value.symbol_id,
        }
    }
}

impl From<xgwx::NewProgram> for NewProgram {
    fn from(value: xgwx::NewProgram) -> Self {
        Self {
            name: value.name,
            language: value.language,
            object_id: value.object_id,
            symbol_id: value.symbol_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CnetFieldEdit {
    pub port_index: usize,
    pub field: String,
    pub expected_value: String,
    pub replacement: String,
}

impl From<CnetFieldEdit> for xgwx::CnetFieldEdit {
    fn from(value: CnetFieldEdit) -> Self {
        Self {
            port_index: value.port_index,
            field: value.field,
            expected_value: value.expected_value,
            replacement: value.replacement,
        }
    }
}

impl From<xgwx::CnetFieldEdit> for CnetFieldEdit {
    fn from(value: xgwx::CnetFieldEdit) -> Self {
        Self {
            port_index: value.port_index,
            field: value.field,
            expected_value: value.expected_value,
            replacement: value.replacement,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CnetSettingsPatch {
    pub base: u32,
    pub slot: u32,
    pub changes: Vec<CnetFieldEdit>,
}

impl From<CnetSettingsPatch> for xgwx::CnetSettingsPatch {
    fn from(value: CnetSettingsPatch) -> Self {
        Self {
            base: value.base,
            slot: value.slot,
            changes: value.changes.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<xgwx::CnetSettingsPatch> for CnetSettingsPatch {
    fn from(value: xgwx::CnetSettingsPatch) -> Self {
        Self {
            base: value.base,
            slot: value.slot,
            changes: value.changes.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SfcEntity {
    pub entity_index: usize,
    pub type_code: Option<u32>,
    pub row: Option<u32>,
    pub column: Option<u32>,
    pub properties: BTreeMap<String, BTreeMap<String, String>>,
}

impl From<SfcEntity> for xgwx::SfcEntity {
    fn from(value: SfcEntity) -> Self {
        Self {
            entity_index: value.entity_index,
            type_code: value.type_code,
            row: value.row,
            column: value.column,
            properties: value.properties,
        }
    }
}

impl From<xgwx::SfcEntity> for SfcEntity {
    fn from(value: xgwx::SfcEntity) -> Self {
        Self {
            entity_index: value.entity_index,
            type_code: value.type_code,
            row: value.row,
            column: value.column,
            properties: value.properties,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SfcEntityPatch {
    pub program_index: usize,
    pub block_index: usize,
    pub entity_index: usize,
    pub expected_type: u32,
    pub expected_row: u32,
    pub expected_column: u32,
    pub field: String,
    pub expected_value: String,
    pub replacement: String,
}

impl From<SfcEntityPatch> for xgwx::SfcEntityPatch {
    fn from(value: SfcEntityPatch) -> Self {
        Self {
            program_index: value.program_index,
            block_index: value.block_index,
            entity_index: value.entity_index,
            expected_type: value.expected_type,
            expected_row: value.expected_row,
            expected_column: value.expected_column,
            field: value.field,
            expected_value: value.expected_value,
            replacement: value.replacement,
        }
    }
}

impl From<xgwx::SfcEntityPatch> for SfcEntityPatch {
    fn from(value: xgwx::SfcEntityPatch) -> Self {
        Self {
            program_index: value.program_index,
            block_index: value.block_index,
            entity_index: value.entity_index,
            expected_type: value.expected_type,
            expected_row: value.expected_row,
            expected_column: value.expected_column,
            field: value.field,
            expected_value: value.expected_value,
            replacement: value.replacement,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SfcRow {
    pub kind: String,
    pub title: String,
    pub comment: String,
    pub initial: bool,
    pub action: Option<String>,
    #[serde(default)]
    pub action_qualifier: Option<String>,
    #[serde(default)]
    pub action_time: Option<String>,
    #[serde(default)]
    pub action_code: Option<String>,
    #[serde(default)]
    pub transition_code: Option<String>,
    #[serde(default)]
    pub position: Option<SfcPosition>,
    #[serde(default)]
    pub branch_end: Option<u32>,
}

impl From<SfcRow> for xgwx::SfcRow {
    fn from(value: SfcRow) -> Self {
        Self {
            kind: value.kind,
            title: value.title,
            comment: value.comment,
            initial: value.initial,
            action: value.action,
            action_qualifier: value.action_qualifier,
            action_time: value.action_time,
            action_code: value.action_code,
            transition_code: value.transition_code,
            position: value.position.map(Into::into),
            branch_end: value.branch_end,
        }
    }
}

impl From<xgwx::SfcRow> for SfcRow {
    fn from(value: xgwx::SfcRow) -> Self {
        Self {
            kind: value.kind,
            title: value.title,
            comment: value.comment,
            initial: value.initial,
            action: value.action,
            action_qualifier: value.action_qualifier,
            action_time: value.action_time,
            action_code: value.action_code,
            transition_code: value.transition_code,
            position: value.position.map(Into::into),
            branch_end: value.branch_end,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SfcPosition {
    pub row: u32,
    pub column: u32,
}

impl From<SfcPosition> for xgwx::SfcPosition {
    fn from(value: SfcPosition) -> Self {
        Self {
            row: value.row,
            column: value.column,
        }
    }
}

impl From<xgwx::SfcPosition> for SfcPosition {
    fn from(value: xgwx::SfcPosition) -> Self {
        Self {
            row: value.row,
            column: value.column,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SfcSequencePatch {
    pub program_index: usize,
    pub block_index: usize,
    pub expected_entities: Vec<SfcEntity>,
    pub rows: Vec<SfcRow>,
    #[serde(default)]
    pub expected_rows: Option<Vec<SfcRow>>,
}

impl From<SfcSequencePatch> for xgwx::SfcSequencePatch {
    fn from(value: SfcSequencePatch) -> Self {
        Self {
            program_index: value.program_index,
            block_index: value.block_index,
            expected_entities: value
                .expected_entities
                .into_iter()
                .map(Into::into)
                .collect(),
            rows: value.rows.into_iter().map(Into::into).collect(),
            expected_rows: value
                .expected_rows
                .map(|items| items.into_iter().map(Into::into).collect()),
        }
    }
}

impl From<xgwx::SfcSequencePatch> for SfcSequencePatch {
    fn from(value: xgwx::SfcSequencePatch) -> Self {
        Self {
            program_index: value.program_index,
            block_index: value.block_index,
            expected_entities: value
                .expected_entities
                .into_iter()
                .map(Into::into)
                .collect(),
            rows: value.rows.into_iter().map(Into::into).collect(),
            expected_rows: value
                .expected_rows
                .map(|items| items.into_iter().map(Into::into).collect()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SfcVariable {
    pub name: String,
    pub data_type: String,
    pub description: String,
    pub system: bool,
    #[serde(default)]
    pub declaration: Option<SfcDeclaration>,
}

impl From<SfcVariable> for xgwx::SfcVariable {
    fn from(value: SfcVariable) -> Self {
        Self {
            name: value.name,
            data_type: value.data_type,
            description: value.description,
            system: value.system,
            declaration: value.declaration.map(Into::into),
        }
    }
}

impl From<xgwx::SfcVariable> for SfcVariable {
    fn from(value: xgwx::SfcVariable) -> Self {
        Self {
            name: value.name,
            data_type: value.data_type,
            description: value.description,
            system: value.system,
            declaration: value.declaration.map(Into::into),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SfcVariablePatch {
    pub program_index: usize,
    pub expected_variables: Vec<SfcVariable>,
    pub name: String,
    pub data_type: String,
    pub description: String,
    pub remove: bool,
    #[serde(default)]
    pub declaration: Option<SfcDeclaration>,
    #[serde(default)]
    pub update: bool,
}

impl From<SfcVariablePatch> for xgwx::SfcVariablePatch {
    fn from(value: SfcVariablePatch) -> Self {
        Self {
            program_index: value.program_index,
            expected_variables: value
                .expected_variables
                .into_iter()
                .map(Into::into)
                .collect(),
            name: value.name,
            data_type: value.data_type,
            description: value.description,
            remove: value.remove,
            declaration: value.declaration.map(Into::into),
            update: value.update,
        }
    }
}

impl From<xgwx::SfcVariablePatch> for SfcVariablePatch {
    fn from(value: xgwx::SfcVariablePatch) -> Self {
        Self {
            program_index: value.program_index,
            expected_variables: value
                .expected_variables
                .into_iter()
                .map(Into::into)
                .collect(),
            name: value.name,
            data_type: value.data_type,
            description: value.description,
            remove: value.remove,
            declaration: value.declaration.map(Into::into),
            update: value.update,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SfcArrayBound {
    pub lower: i32,
    pub upper: i32,
}

impl From<SfcArrayBound> for xgwx::SfcArrayBound {
    fn from(value: SfcArrayBound) -> Self {
        Self {
            lower: value.lower,
            upper: value.upper,
        }
    }
}

impl From<xgwx::SfcArrayBound> for SfcArrayBound {
    fn from(value: xgwx::SfcArrayBound) -> Self {
        Self {
            lower: value.lower,
            upper: value.upper,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SfcDeclaration {
    #[serde(default)]
    pub dimensions: Vec<SfcArrayBound>,
    #[serde(default)]
    pub initial_value: String,
    #[serde(default)]
    pub retain: bool,
}

impl From<SfcDeclaration> for xgwx::SfcDeclaration {
    fn from(value: SfcDeclaration) -> Self {
        Self {
            dimensions: value.dimensions.into_iter().map(Into::into).collect(),
            initial_value: value.initial_value,
            retain: value.retain,
        }
    }
}

impl From<xgwx::SfcDeclaration> for SfcDeclaration {
    fn from(value: xgwx::SfcDeclaration) -> Self {
        Self {
            dimensions: value.dimensions.into_iter().map(Into::into).collect(),
            initial_value: value.initial_value,
            retain: value.retain,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LadderCellEdit {
    pub raw_y: u32,
    pub column: u8,
    pub expected: Option<LadderEditElement>,
    pub replacement: Option<LadderEditElement>,
}

impl From<LadderCellEdit> for xgwx::LadderCellEdit {
    fn from(value: LadderCellEdit) -> Self {
        Self {
            raw_y: value.raw_y,
            column: value.column,
            expected: value.expected.map(Into::into),
            replacement: value.replacement.map(Into::into),
        }
    }
}

impl From<xgwx::LadderCellEdit> for LadderCellEdit {
    fn from(value: xgwx::LadderCellEdit) -> Self {
        Self {
            raw_y: value.raw_y,
            column: value.column,
            expected: value.expected.map(Into::into),
            replacement: value.replacement.map(Into::into),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LadderCommentEdit {
    pub kind: LadderCommentKind,
    pub raw_y: u32,
    pub expected: Option<String>,
    pub replacement: String,
}

impl From<LadderCommentEdit> for xgwx::LadderCommentEdit {
    fn from(value: LadderCommentEdit) -> Self {
        Self {
            kind: value.kind.into(),
            raw_y: value.raw_y,
            expected: value.expected,
            replacement: value.replacement,
        }
    }
}

impl From<xgwx::LadderCommentEdit> for LadderCommentEdit {
    fn from(value: xgwx::LadderCommentEdit) -> Self {
        Self {
            kind: value.kind.into(),
            raw_y: value.raw_y,
            expected: value.expected,
            replacement: value.replacement,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LadderBranchEdit {
    pub raw_y: u32,
    pub boundary: u8,
    pub expected: bool,
    pub present: bool,
}

impl From<LadderBranchEdit> for xgwx::LadderBranchEdit {
    fn from(value: LadderBranchEdit) -> Self {
        Self {
            raw_y: value.raw_y,
            boundary: value.boundary,
            expected: value.expected,
            present: value.present,
        }
    }
}

impl From<xgwx::LadderBranchEdit> for LadderBranchEdit {
    fn from(value: xgwx::LadderBranchEdit) -> Self {
        Self {
            raw_y: value.raw_y,
            boundary: value.boundary,
            expected: value.expected,
            present: value.present,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LadderEditElement {
    pub kind: LadderEditKind,
    pub operand: String,
}

impl From<LadderEditElement> for xgwx::LadderEditElement {
    fn from(value: LadderEditElement) -> Self {
        Self {
            kind: value.kind.into(),
            operand: value.operand,
        }
    }
}

impl From<xgwx::LadderEditElement> for LadderEditElement {
    fn from(value: xgwx::LadderEditElement) -> Self {
        Self {
            kind: value.kind.into(),
            operand: value.operand,
        }
    }
}

/// Guarded standalone ST/IL source replacement; protocol fields remain snake_case.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TextProgramPatch {
    pub program_index: usize,
    pub expected_object_id: String,
    pub expected_language: String,
    pub expected_source: String,
    pub source: String,
}
impl From<TextProgramPatch> for xgwx::TextProgramPatch {
    fn from(value: TextProgramPatch) -> Self {
        Self {
            program_index: value.program_index,
            expected_object_id: value.expected_object_id,
            expected_language: value.expected_language,
            expected_source: value.expected_source,
            source: value.source,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VendorIlPatch {
    pub program_index: usize,
    pub expected_object_id: String,
    pub expected_source: String,
    pub source: String,
}
impl From<VendorIlPatch> for xgwx::VendorIlPatch {
    fn from(value: VendorIlPatch) -> Self {
        Self {
            program_index: value.program_index,
            expected_object_id: value.expected_object_id,
            expected_source: value.expected_source,
            source: value.source,
        }
    }
}
