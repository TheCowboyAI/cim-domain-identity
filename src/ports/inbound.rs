//! Inbound ports for the Identity context

use async_trait::async_trait;
use crate::{PersonCommand, PersonId, OrganizationCommand, OrganizationId, IdentityResult};

/// Command handler interface for identity commands
#[async_trait]
pub trait IdentityCommandHandler: Send + Sync {
    /// Handle a person command
    async fn handle_person_command(&self, person_id: PersonId, command: PersonCommand) -> IdentityResult<()>;

    /// Handle an organization command
    async fn handle_organization_command(&self, org_id: OrganizationId, command: OrganizationCommand) -> IdentityResult<()>;
}

/// Query handler interface for identity queries
#[async_trait]
pub trait IdentityQueryHandler: Send + Sync {
    // TODO: Define query methods
}
