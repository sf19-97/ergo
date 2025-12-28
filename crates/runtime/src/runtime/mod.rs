pub mod types;
pub mod validate;
pub mod execute;

pub use types::*;
pub use validate::validate;
pub use execute::execute;

#[cfg(test)]
mod tests;

use crate::cluster::{ExpandedGraph, PrimitiveCatalog};

#[derive(Debug)]
pub enum RuntimeError {
    Validation(types::ValidationError),
    Execution(types::ExecError),
}

/// Canonical execution entrypoint.
/// Validates the expanded graph before executing it with the provided registries and context.
pub fn run<C: PrimitiveCatalog>(
    expanded: &ExpandedGraph,
    catalog: &C,
    registries: &types::Registries,
    ctx: &types::ExecutionContext,
) -> Result<types::ExecutionReport, RuntimeError> {
    let validated = validate(expanded, catalog).map_err(RuntimeError::Validation)?;
    execute(&validated, registries, ctx).map_err(RuntimeError::Execution)
}
