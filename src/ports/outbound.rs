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

    /// Find a person by email
    async fn find_by_email(&self, email: &str) -> IdentityResult<Option<Person>>;

    /// Get all persons (for cross-aggregate queries)
    async fn find_all(&self) -> IdentityResult<Vec<Person>>;

    /// Search people by name (basic text matching)
    async fn search_by_name(&self, name_query: &str) -> IdentityResult<Vec<Person>>;
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

    /// Find an organization by name
    async fn find_by_name(&self, name: &str) -> IdentityResult<Option<Organization>>;

    /// Get all organizations (for cross-aggregate queries)
    async fn find_all(&self) -> IdentityResult<Vec<Organization>>;

    /// Search organizations by name (basic text matching)
    async fn search_by_name(&self, name_query: &str) -> IdentityResult<Vec<Organization>>;
}
