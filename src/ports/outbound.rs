//! Outbound ports for the Identity context

use async_trait::async_trait;
use crate::{Person, PersonId, Organization, OrganizationId, IdentityResult};

/// Repository interface for Person aggregates
#[async_trait]
pub trait PersonRepository: Send + Sync {
    /// Load a person by ID
    async fn load(&self, id: PersonId) -> IdentityResult<Person>;

    /// Save a person
    async fn save(&self, person: &Person) -> IdentityResult<()>;

    /// Check if email exists
    async fn email_exists(&self, email: &str) -> IdentityResult<bool>;
}

/// Repository interface for Organization aggregates
#[async_trait]
pub trait OrganizationRepository: Send + Sync {
    /// Load an organization by ID
    async fn load(&self, id: OrganizationId) -> IdentityResult<Organization>;

    /// Save an organization
    async fn save(&self, organization: &Organization) -> IdentityResult<()>;

    /// Check if organization name exists
    async fn name_exists(&self, name: &str) -> IdentityResult<bool>;
}
