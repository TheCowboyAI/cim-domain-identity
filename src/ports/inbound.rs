//! Inbound ports for the Identity context

use async_trait::async_trait;
use crate::{Person, PersonId, Organization, OrganizationId, PersonCommand, OrganizationCommand, IdentityResult};

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
    /// Find a person by ID
    async fn find_person_by_id(&self, person_id: PersonId) -> IdentityResult<Option<Person>>;

    /// Find a person by email
    async fn find_person_by_email(&self, email: &str) -> IdentityResult<Option<Person>>;

    /// Find an organization by ID
    async fn find_organization_by_id(&self, org_id: OrganizationId) -> IdentityResult<Option<Organization>>;

    /// Find an organization by name
    async fn find_organization_by_name(&self, name: &str) -> IdentityResult<Option<Organization>>;

    /// Find organizations where a person is a member
    async fn find_organizations_for_person(&self, person_id: PersonId) -> IdentityResult<Vec<Organization>>;

    /// Find members of an organization
    async fn find_organization_members(&self, org_id: OrganizationId) -> IdentityResult<Vec<Person>>;

    /// Find administrators of an organization
    async fn find_organization_admins(&self, org_id: OrganizationId) -> IdentityResult<Vec<Person>>;

    /// Search people by name
    async fn search_people_by_name(&self, name_query: &str) -> IdentityResult<Vec<Person>>;

    /// Search organizations by name
    async fn search_organizations_by_name(&self, name_query: &str) -> IdentityResult<Vec<Organization>>;
}
