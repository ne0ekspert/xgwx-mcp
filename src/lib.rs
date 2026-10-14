pub mod common;
pub mod inspection;
pub mod operation_types;
pub mod operations;
pub mod search;
pub mod server;
pub mod storage;
pub mod text_ir;

mod context_ir;
mod operand_ir;
mod program_lifecycle_ir;
mod sfc_ir;
mod variable_lifecycle_ir;
mod writer_tools;

pub mod templates;

pub mod property_ir;

#[cfg(test)]
mod test_support;
