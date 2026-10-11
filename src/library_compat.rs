//! Keep hardware editing coordinates stable across the library's module API change.
use xgwx::{ModuleSummary, XgwxDocument};

/// Installed hardware records only. Network memberships must not become extra
/// hardware edit targets or change property-document module indices.
pub fn module_summaries(doc: &XgwxDocument) -> Vec<ModuleSummary> {
    #[cfg(feature = "libxgwx-namespaced-api")]
    {
        doc.module_summaries()
    }
    #[cfg(not(feature = "libxgwx-namespaced-api"))]
    {
        doc.modules()
    }
}

/// Native base summaries keep the property/inspection JSON shape unchanged.
pub fn base_summaries(doc: &XgwxDocument) -> Vec<xgwx::BaseSummary> {
    #[cfg(feature = "libxgwx-namespaced-api")]
    {
        doc.base_summaries()
    }
    #[cfg(not(feature = "libxgwx-namespaced-api"))]
    {
        doc.bases()
    }
}

/// Namespace accessors for the old pinned library. With the namespaced API feature,
/// adapters use the library's own borrowed program/variable/network views instead.
#[cfg(not(feature = "libxgwx-namespaced-api"))]
pub trait DocumentNamespacesCompat {
    fn network(&self) -> &XgwxDocument;
    fn program(&self) -> &XgwxDocument;
    fn variable(&self) -> &XgwxDocument;
    fn program_mut(&mut self) -> &mut XgwxDocument;
    fn variable_mut(&mut self) -> &mut XgwxDocument;
}

#[cfg(not(feature = "libxgwx-namespaced-api"))]
impl DocumentNamespacesCompat for XgwxDocument {
    fn network(&self) -> &XgwxDocument {
        self
    }
    fn program(&self) -> &XgwxDocument {
        self
    }
    fn variable(&self) -> &XgwxDocument {
        self
    }
    fn program_mut(&mut self) -> &mut XgwxDocument {
        self
    }
    fn variable_mut(&mut self) -> &mut XgwxDocument {
        self
    }
}
