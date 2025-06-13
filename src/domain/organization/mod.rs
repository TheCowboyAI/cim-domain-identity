//! Organization aggregate module

mod aggregate;
mod commands;
mod events;

pub use aggregate::{Organization, OrganizationId, OrganizationType};
pub use commands::OrganizationCommand;
pub use events::OrganizationEvent;
