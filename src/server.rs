use crate::{
    inspection, operations,
    storage::{EditOperation, FileStore, StoreError},
};
use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::CallToolResult,
    tool, tool_handler, tool_router,
};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Clone)]
pub struct XgwxServer {
    store: FileStore,
    tool_router: ToolRouter<Self>,
}
impl XgwxServer {
    pub fn new(store: FileStore) -> Self {
        Self {
            store,
            tool_router: Self::tool_router(),
        }
    }
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct InspectRequest {
    /// File path, absolute or relative to the server startup directory.
    pub path: String,
    /// overview, programs, program, symbols, hardware, networks, sfc, sfc.variables,
    /// sfc.block.<numeric block index>, program.edit_sites,
    /// program.rows, program.records, program.functions, program.references,
    /// program.operand_links, program.graph, program.strings, program.cells, catalog.cpu, catalog.modules,
    /// catalog.instructions, catalog.comparisons, or io_variables.
    #[serde(default = "overview")]
    pub section: String,
    /// Zero-based program index for program details.
    pub program_index: Option<usize>,
    #[serde(default)]
    pub offset: usize,
    /// Page size, between 1 and 1000; applies to each detailed array.
    #[serde(default = "page_size")]
    pub limit: usize,
}
fn overview() -> String {
    "overview".into()
}
fn page_size() -> usize {
    100
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ListRequest {
    pub query: Option<String>,
    pub subsystem: Option<String>,
}
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DescribeRequest {
    pub name: String,
}
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FileRequest {
    pub path: String,
}
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ApplyRequest {
    pub path: String,
    /// SHA-256 revision returned by inspect_file, validate_file, or a committed apply_edits.
    pub expected_revision: String,
    /// Ordered edits forming one transaction. Arguments follow describe_operation schemas.
    pub operations: Vec<EditOperation>,
    /// Run the same edits and verification without changing disk. Returned revision_after
    /// describes the proposed output; the source retains revision_before.
    #[serde(default)]
    pub dry_run: bool,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct IrRequest {
    pub path: String,
    /// Document-order ladder or SFC program index. SFC exports selected charts and local declarations.
    pub program_index: Option<usize>,
    /// program (default), modules, variables or networks. Subsystem exports mark edit/readonly properties.
    #[serde(default = "program_section")]
    pub section: String,
}
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PatchRequest {
    pub path: String,
    /// Current file SHA-256 returned by get_ir. Re-export after structural edits.
    pub expected_revision: String,
    pub program_index: Option<usize>,
    /// Same section used in get_ir: program (default), modules, variables or networks.
    #[serde(default = "program_section")]
    pub section: String,
    /// Exact unified diff against the text returned by get_ir; no fuzzy matching.
    /// Strings use quoted UTF-8 literals with escaped newlines. Native positions,
    /// identities, instruction families and opaque records are immutable in v1. Physical wire
    /// deletion, vertical creation and instruction creation/deletion/arity changes
    /// require a patch with exactly one mutation.
    pub patch: String,
    #[serde(default)]
    pub dry_run: bool,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateFileRequest {
    /// New .xgwx path within an allowed root; its parent directory must already exist.
    pub path: String,
    /// Optional legacy template. Omit to derive from CPU and program_language.
    pub template: Option<crate::templates::ProjectTemplate>,
    /// Name stored inside the project. Omit to retain NewProject.
    pub project_name: Option<String>,
    /// Initial program name. Omit to retain NewProgram.
    pub program_name: Option<String>,
    /// LD or SFC, constrained by CPU; discover supported combinations with list_templates.
    pub program_language: Option<crate::templates::ProgramLanguage>,
    /// Optional model from list_templates.selectable_cpu_models. Existing CPU guards apply.
    pub cpu_model: Option<String>,
    /// Optional initial ordered writer edits. Any failure prevents file creation.
    #[serde(default)]
    pub operations: Vec<EditOperation>,
    /// Prepare edits and verify without creating a file, temporary or lock sidecar.
    #[serde(default)]
    pub dry_run: bool,
}

fn program_section() -> String {
    "program".into()
}
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PageFileRequest {
    pub path: String,
    #[serde(default)]
    pub offset: usize,
    #[serde(default = "page_size")]
    pub limit: usize,
}
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct VariablesRequest {
    pub path: String,
    /// Restrict local tables to one program; globals remain included.
    pub program_index: Option<usize>,
    #[serde(default)]
    pub offset: usize,
    #[serde(default = "page_size")]
    pub limit: usize,
}
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ModuleCatalogRequest {
    pub query: Option<String>,
    #[serde(default)]
    pub offset: usize,
    #[serde(default = "page_size")]
    pub limit: usize,
}

fn result(value: Result<Value, StoreError>) -> CallToolResult {
    match value {
        Ok(value) => CallToolResult::structured(value),
        Err(error) => CallToolResult::structured_error(json!({"error":error})),
    }
}
async fn blocking(
    task: impl FnOnce() -> Result<Value, StoreError> + Send + 'static,
) -> CallToolResult {
    match tokio::task::spawn_blocking(task).await {
        Ok(value) => result(value),
        Err(error) => result(Err(StoreError::new("internal_error", error))),
    }
}

#[tool_router]
impl XgwxServer {
    #[tool(
        description = "List library-generated blank project profiles, their default names/CPU, generated hashes and CPU models selectable under existing library guards. No project file is required.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn list_templates(&self) -> CallToolResult {
        result(crate::templates::list())
    }

    #[tool(
        description = "Generate a new .xgwx project through libxgwx for an xgk_ld, xgi_ld or xgi_sfc creation profile. Optionally select a supported CPU and apply ordered initial writer edits. Verify before atomic no-clobber creation; never overwrite an existing file, directory or symlink. Parent must exist within allowed roots. Choose project_name, path, cpu_model, program_language and optional program_name. Template is optional and derived from CPU/language. Discover creation_choices with list_templates. Library default identities are preserved. No original exists to back up. dry_run creates no files or sidecars; inspect the saved revision before further edits. Native acceptance remains separate.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn create_file(
        &self,
        Parameters(request): Parameters<CreateFileRequest>,
    ) -> CallToolResult {
        let store = self.store.clone();
        blocking(move || {
            store.create_project(&crate::storage::CreateProjectOptions {
                path: request.path,
                template: request.template,
                cpu_model: request.cpu_model,
                project_name: request.project_name,
                program_name: request.program_name,
                program_language: request.program_language,
                operations: request.operations,
                dry_run: request.dry_run,
            })
        })
        .await
    }

    #[tool(
        description = "Inspect an XGWX file and obtain its SHA-256 revision, stable identities, decoded coordinates, symbols, catalogs and edit sites. Paginate details; inspect again after structural edits.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn inspect_file(
        &self,
        Parameters(request): Parameters<InspectRequest>,
    ) -> CallToolResult {
        let store = self.store.clone();
        blocking(move || {
            if !(1..=1000).contains(&request.limit) {
                return Err(StoreError::new(
                    "invalid_arguments",
                    "limit must be between 1 and 1000",
                ));
            }
            let (path, revision, doc) = store.read(&request.path)?;
            let data = inspection::inspect(
                &doc,
                &request.section,
                request.program_index,
                request.offset,
                request.limit,
            )
            .map_err(|e| StoreError::new("inspection_failed", e))?;
            Ok(json!({"path":path,"revision":revision,"data":data}))
        })
        .await
    }

    #[tool(
        description = "List installed modules with base/slot identities, settings, CPU and rack context. Use list_module_catalog for available models; replacements still obey library CPU/layout guards. Returns current revision and paginated arrays.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn list_modules(
        &self,
        Parameters(request): Parameters<PageFileRequest>,
    ) -> CallToolResult {
        let store = self.store.clone();
        blocking(move || {
            let (path,revision,doc)=store.read(&request.path)?;
            let data=inspection::inspect(&doc,"hardware",None,request.offset,request.limit).map_err(|e|StoreError::new("inspection_failed",e))?;
            Ok(json!({"path":path,"revision":revision,"data":data,"catalog_tool":"list_module_catalog","patch_section":"modules"}))
        }).await
    }
    #[tool(
        description = "Search the available module catalog without a project file. Returns paginated model identities, defaults, writable option choices and known support status. Catalog presence does not establish compatibility with every CPU or layout.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn list_module_catalog(
        &self,
        Parameters(request): Parameters<ModuleCatalogRequest>,
    ) -> CallToolResult {
        CallToolResult::structured(inspection::list_module_catalog(
            request.query.as_deref(),
            request.offset,
            request.limit,
        ))
    }
    #[tool(
        description = "List configured networks, linked module identities and decoded Cnet/FEnet settings with the current file revision. Pagination applies independently to nested arrays. Use get_ir section networks for guarded property edits.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn list_networks(
        &self,
        Parameters(request): Parameters<PageFileRequest>,
    ) -> CallToolResult {
        let store = self.store.clone();
        blocking(move || {
            let (path, revision, doc) = store.read(&request.path)?;
            let data = inspection::inspect(&doc, "networks", None, request.offset, request.limit)
                .map_err(|e| StoreError::new("inspection_failed", e))?;
            Ok(json!({"path":path,"revision":revision,"data":data,"patch_section":"networks"}))
        })
        .await
    }
    #[tool(
        description = "List global and IEC program-local variables with names, types, addresses, descriptions and revision-scoped indices. Optional program_index selects one local table; globals remain included. SFC declarations remain in inspect_file section sfc.variables. Pagination applies independently to tables.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn list_variables(
        &self,
        Parameters(request): Parameters<VariablesRequest>,
    ) -> CallToolResult {
        let store = self.store.clone();
        blocking(move || {
            let (path, revision, doc) = store.read(&request.path)?;
            let data = inspection::inspect(
                &doc,
                "symbols",
                request.program_index,
                request.offset,
                request.limit,
            )
            .map_err(|e| StoreError::new("inspection_failed", e))?;
            Ok(json!({"path":path,"revision":revision,"data":data,"patch_section":"variables"}))
        })
        .await
    }

    #[tool(
        description = "Discover public editing operations by subsystem or search text. Use describe_operation for the exact typed argument schema before editing.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn list_operations(&self, Parameters(request): Parameters<ListRequest>) -> CallToolResult {
        CallToolResult::structured(operations::list_operations(
            request.query.as_deref(),
            request.subsystem.as_deref(),
        ))
    }

    #[tool(
        description = "Get the exact JSON argument schema, constraints, required inspection data, and example argument template for a public libxgwx writer operation.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn describe_operation(
        &self,
        Parameters(request): Parameters<DescribeRequest>,
    ) -> CallToolResult {
        result(
            operations::describe_operation(&request.name)
                .map_err(|e| StoreError::new("unknown_operation", e)),
        )
    }

    #[tool(
        description = "Apply ordered edits as one transaction to an XGWX file. Requires the current revision. Saves immediately with a unique adjacent backup and atomic replacement after verification. dry_run performs no disk changes. A failed operation leaves source bytes unchanged; re-inspect after structural changes.",
        annotations(
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn apply_edits(&self, Parameters(request): Parameters<ApplyRequest>) -> CallToolResult {
        let store = self.store.clone();
        blocking(move || {
            store.apply(
                &request.path,
                &request.expected_revision,
                &request.operations,
                request.dry_run,
            )
        })
        .await
    }

    #[tool(
        description = "Export revision-scoped text. section modules/variables/networks provides property documents with edit/readonly fields; section program requires program_index and exports an XGK/IEC ladder or SFC program. Includes contacts, coils, native rung/output comments, symbol descriptions, catalog XGK instructions, bound IEC function operands and captured arithmetic/comparison names, native coordinates, physical wires and read-only derived circuit connections. Use an exact unified diff with apply_patch. Positions and identities are scoped to this revision.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn get_ir(&self, Parameters(request): Parameters<IrRequest>) -> CallToolResult {
        let store = self.store.clone();
        blocking(move || {
            let (path, revision, doc) = store.read(&request.path)?;
            if request.section != "program" {
                let ir=crate::property_ir::Projection::export(&doc,&revision,&request.section,request.program_index)?;
                return Ok(json!({"path":path,"revision":revision,"section":request.section,"program_index":request.program_index,"format":"xgwx-properties/1","text":ir.text,"capabilities":["marked_property_updates"],"limitations":"Only properties marked edit can change. Retain object/field identities, order, scalar types and readonly lines. Missing values and structural creation/deletion/replacement are read-only; use apply_edits for those. IEC local rename/type changes must be the sole mutation and require renewed export. Native acceptance remains separate."}));
            }
            let program_index=request.program_index.ok_or_else(||StoreError::new("invalid_arguments","program_index is required for section program"))?;
            let ir = crate::text_ir::Projection::export(&doc, &revision, program_index)?;
            let mut capabilities = vec!["contact_kind_and_operand", "coil_kind_and_operand",
                "comment_text", "global_and_local_symbol_descriptions"];
            if ir.dialect == "XGK" {
                capabilities.extend(["xgk_element_creation_deletion", "xgk_comment_creation", "xgk_rung_comment_deletion", "physical_wire_deletion", "vertical_branch_creation", "xgk_instruction_creation_deletion", "xgk_instruction_mnemonic_and_operands"]);
            }
            if ir.dialect == "IEC" { capabilities.extend(["iec_empty_row_element_creation", "iec_terminal_coil_creation", "iec_isolated_element_deletion", "iec_connected_contact_and_terminal_coil_deletion", "physical_wire_deletion", "vertical_wire_creation", "iec_function_operand_edit", "iec_arithmetic_and_comparison_name_edit"]); }
            if ir.dialect == "SFC" { capabilities = vec!["sfc_existing_row_fields", "sfc_linear_row_creation_deletion_reordering", "sfc_direct_entity_comment_and_condition", "sfc_declaration_descriptions"]; }
            let sfc_limitations = "SFC row changes use one complete chart snapshot and must be the only writer operation. Linear chart rows can be created, deleted and reordered; new rows require @new- identities and position none/branch_end none. Retained row kind and coordinates, positioned branch order/structure, chart/entity identities and declaration name/type/advanced metadata are read-only. Unsupported charts expose only guarded step-comment/direct %MX transition fields. Native entity, qualifier, TIME, ST-source and declaration constraints remain authoritative. Re-export after chart replacement; native XG5000 acceptance remains separate.";
            Ok(json!({"path":path,"revision":revision,"program_index":program_index,
                "format":"xgwx-ir/1","dialect":ir.dialect,"text":ir.text,
                "capabilities":capabilities,
                "limitations":if ir.dialect == "SFC" {sfc_limitations} else {"Existing positions, derived connections, unknown instruction records, function signatures/pin bindings and opaque records are read-only. IEC function names can change only within captured arithmetic/comparison families; bound operand values retain native type/writability checks and cannot be empty. Physical wires can be deleted and supported vertical wires created with fresh @new- identities. Instruction creation/deletion/arity changes, connected IEC deletion and wire changes must be the sole mutation in a patch; re-export afterward. Existing writer preconditions still apply. Native XG5000 acceptance remains separate."}}))
        }).await
    }

    #[tool(
        description = "Apply an exact unified diff to text from get_ir using the same section/program_index. Property documents support module comments/options, global variable metadata, guarded IEC local names/types/descriptions, network names/module metadata and Cnet/FEnet settings. Structural property objects and readonly fields cannot change. IEC local rename/type changes require one mutation. Compile supported semantic changes into guarded writers and save as one verified transaction with a backup. Supports SFC linear row creation/deletion/reordering and existing row fields, guarded direct entity comments/conditions and declaration descriptions, plus XGK cell and catalog instruction/comparison creation/deletion, instruction mnemonic/operand edits, IEC empty-row element creation, terminal-coil creation, isolated/connected-contact and terminal-coil deletion, physical-wire deletion and vertical-wire creation, contact/coil kind/operand edits, bound IEC function operands and captured arithmetic/comparison names, existing comment text, symbol descriptions, and XGK comment creation/rung comment deletion. Instruction creation/deletion/arity changes, connected IEC deletion and wire changes must be the sole mutation; re-export afterward. SFC row changes target one chart and must be the only writer operation. Rejects unsupported moves and derived topology edits. Empty patch is a no-op. dry_run performs no disk changes.",
        annotations(
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn apply_patch(&self, Parameters(request): Parameters<PatchRequest>) -> CallToolResult {
        let store = self.store.clone();
        blocking(move || {
            if request.section == "program" {
                let index = request.program_index.ok_or_else(|| {
                    StoreError::new(
                        "invalid_arguments",
                        "program_index is required for section program",
                    )
                })?;
                store.apply_patch(
                    &request.path,
                    &request.expected_revision,
                    index,
                    &request.patch,
                    request.dry_run,
                )
            } else {
                store.apply_property_patch(
                    &request.path,
                    &request.expected_revision,
                    &request.section,
                    request.program_index,
                    &request.patch,
                    request.dry_run,
                )
            }
        })
        .await
    }

    #[tool(
        description = "Validate container serialization and obtain available decoded program diagnostics. Local results do not establish native XG5000 Open/Check Program/Save As acceptance or PLC behavior.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn validate_file(&self, Parameters(request): Parameters<FileRequest>) -> CallToolResult {
        let store = self.store.clone();
        blocking(move || {
            let (path, revision, doc) = store.read(&request.path)?;
            Ok(json!({"path":path,"revision":revision,"validation":inspection::validate(&doc)}))
        })
        .await
    }
}

#[tool_handler(router = self.tool_router, name = "xgwx-mcp", version = "0.1.0", instructions = "Inspect the file and use the current revision for edits. Use focused list_modules/list_variables/list_networks discovery and list_module_catalog for available hardware. Export get_ir section modules/variables/networks for marked property edits, or section program with program_index for positional ladder/SFC text. Send an exact unified diff to apply_patch with the same section and selection. For direct operations, discover schemas with list_operations/describe_operation. Edits save immediately with backups. Re-inspect after structural edits. Local validation does not certify XG5000 or PLC acceptance.")]
impl ServerHandler for XgwxServer {}
