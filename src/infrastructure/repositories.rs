//! Repository implementations for the Identity context
//!
//! ## Test Coverage
//!
//! ```mermaid
//! graph TD
//!     REP[Repository] --> MAP[HashMap Storage]
//!     REP --> SYNC[Arc<Mutex>]
//!     MAP --> PE[Person Entities]
//!     MAP --> OE[Organization Entities]
//!     
//!     subgraph Tests
//!         UT[Unit Tests] --> REP
//!         CT[Concurrency Tests] --> SYNC
//!         PT[Persistence Tests] --> MAP
//!     end
//! ```

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use cim_domain::AggregateRoot;
use crate::{
    Person, PersonId, Organization, OrganizationId,
    PersonRepository, OrganizationRepository,
    IdentityResult, IdentityError,
};

/// In-memory implementation of PersonRepository
///
/// ## Design Notes
/// 
/// This implementation uses HashMap for storage with Arc<Mutex> for thread safety.
/// In production, this would be replaced with a persistent storage implementation
/// using the same interface.
#[derive(Debug)]
pub struct InMemoryPersonRepository {
    persons: Arc<Mutex<HashMap<PersonId, Person>>>,
    email_index: Arc<Mutex<HashMap<String, PersonId>>>,
}

impl InMemoryPersonRepository {
    pub fn new() -> Self {
        Self {
            persons: Arc::new(Mutex::new(HashMap::new())),
            email_index: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Find a person by email (additional method not in trait)
    pub async fn find_by_email(&self, email: &str) -> IdentityResult<Option<Person>> {
        let email_index = self.email_index.lock().unwrap();
        let persons = self.persons.lock().unwrap();
        
        if let Some(person_id) = email_index.get(email) {
            Ok(persons.get(person_id).cloned())
        } else {
            Ok(None)
        }
    }

    /// Get all persons (for debugging/testing)
    pub async fn find_all(&self) -> IdentityResult<Vec<Person>> {
        let persons = self.persons.lock().unwrap();
        Ok(persons.values().cloned().collect())
    }
}

impl Default for InMemoryPersonRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PersonRepository for InMemoryPersonRepository {
    /// Load a person by ID
    ///
    /// ## Test Coverage
    ///
    /// ```mermaid
    /// graph LR
    ///     ID[Person ID] --> M[Mutex Lock]
    ///     M --> HM[HashMap Lookup]
    ///     HM --> P[Person Result]
    /// ```
    async fn load(&self, id: PersonId) -> IdentityResult<Person> {
        let persons = self.persons.lock().unwrap();
        persons.get(&id)
            .cloned()
            .ok_or(IdentityError::PersonNotFound(id))
    }

    /// Save a person
    async fn save(&self, person: &Person) -> IdentityResult<()> {
        let mut persons = self.persons.lock().unwrap();
        let mut email_index = self.email_index.lock().unwrap();
        
        let person_id = person.id();
        let email = person.email.to_string();
        
        // Update email index
        email_index.insert(email, person_id);
        
        // Store person
        persons.insert(person_id, person.clone());
        
        Ok(())
    }

    /// Check if email exists
    async fn email_exists(&self, email: &str) -> IdentityResult<bool> {
        let email_index = self.email_index.lock().unwrap();
        Ok(email_index.contains_key(email))
    }

    /// Find a person by email
    async fn find_by_email(&self, email: &str) -> IdentityResult<Option<Person>> {
        let email_index = self.email_index.lock().unwrap();
        let persons = self.persons.lock().unwrap();
        
        if let Some(person_id) = email_index.get(email) {
            Ok(persons.get(person_id).cloned())
        } else {
            Ok(None)
        }
    }

    /// Get all persons (for cross-aggregate queries)
    async fn find_all(&self) -> IdentityResult<Vec<Person>> {
        let persons = self.persons.lock().unwrap();
        Ok(persons.values().cloned().collect())
    }

    /// Search people by name (basic text matching)
    async fn search_by_name(&self, name_query: &str) -> IdentityResult<Vec<Person>> {
        let persons = self.persons.lock().unwrap();
        let query_lower = name_query.to_lowercase();
        
        let matching_persons: Vec<Person> = persons
            .values()
            .filter(|person| {
                let full_name = format!("{person.name.first} {person.name.middle.as_ref(} {}").unwrap_or(&"".to_string()),
                    person.name.last
                ).to_lowercase();
                full_name.contains(&query_lower)
            })
            .cloned()
            .collect();
            
        Ok(matching_persons)
    }
}

/// In-memory implementation of OrganizationRepository
///
/// ## Design Notes
/// 
/// Similar to PersonRepository but for Organization entities.
/// Maintains a name index for uniqueness checking.
#[derive(Debug)]
pub struct InMemoryOrganizationRepository {
    organizations: Arc<Mutex<HashMap<OrganizationId, Organization>>>,
    name_index: Arc<Mutex<HashMap<String, OrganizationId>>>,
}

impl InMemoryOrganizationRepository {
    pub fn new() -> Self {
        Self {
            organizations: Arc::new(Mutex::new(HashMap::new())),
            name_index: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Find an organization by name (additional method not in trait)
    pub async fn find_by_name(&self, name: &str) -> IdentityResult<Option<Organization>> {
        let name_index = self.name_index.lock().unwrap();
        let organizations = self.organizations.lock().unwrap();
        
        if let Some(org_id) = name_index.get(name) {
            Ok(organizations.get(org_id).cloned())
        } else {
            Ok(None)
        }
    }

    /// Get all organizations (for debugging/testing)
    pub async fn find_all(&self) -> IdentityResult<Vec<Organization>> {
        let organizations = self.organizations.lock().unwrap();
        Ok(organizations.values().cloned().collect())
    }
}

impl Default for InMemoryOrganizationRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl OrganizationRepository for InMemoryOrganizationRepository {
    /// Load an organization by ID
    ///
    /// ## Test Coverage
    ///
    /// ```mermaid
    /// graph LR
    ///     ID[Organization ID] --> M[Mutex Lock]
    ///     M --> HM[HashMap Lookup]
    ///     HM --> O[Organization Result]
    /// ```
    async fn load(&self, id: OrganizationId) -> IdentityResult<Organization> {
        let organizations = self.organizations.lock().unwrap();
        organizations.get(&id)
            .cloned()
            .ok_or(IdentityError::OrganizationNotFound(id))
    }

    /// Save an organization
    async fn save(&self, organization: &Organization) -> IdentityResult<()> {
        let mut organizations = self.organizations.lock().unwrap();
        let mut name_index = self.name_index.lock().unwrap();
        
        let org_id = organization.id();
        let name = organization.name.clone();
        
        // Update name index
        name_index.insert(name, org_id);
        
        // Store organization
        organizations.insert(org_id, organization.clone());
        
        Ok(())
    }

    /// Check if organization name exists
    async fn name_exists(&self, name: &str) -> IdentityResult<bool> {
        let name_index = self.name_index.lock().unwrap();
        Ok(name_index.contains_key(name))
    }

    /// Find an organization by name
    async fn find_by_name(&self, name: &str) -> IdentityResult<Option<Organization>> {
        let name_index = self.name_index.lock().unwrap();
        let organizations = self.organizations.lock().unwrap();
        
        if let Some(org_id) = name_index.get(name) {
            Ok(organizations.get(org_id).cloned())
        } else {
            Ok(None)
        }
    }

    /// Get all organizations (for cross-aggregate queries)
    async fn find_all(&self) -> IdentityResult<Vec<Organization>> {
        let organizations = self.organizations.lock().unwrap();
        Ok(organizations.values().cloned().collect())
    }

    /// Search organizations by name (basic text matching)
    async fn search_by_name(&self, name_query: &str) -> IdentityResult<Vec<Organization>> {
        let organizations = self.organizations.lock().unwrap();
        let query_lower = name_query.to_lowercase();
        
        let matching_orgs: Vec<Organization> = organizations
            .values()
            .filter(|org| {
                org.name.to_lowercase().contains(&query_lower)
            })
            .cloned()
            .collect();
            
        Ok(matching_orgs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Name, Email};
    use crate::domain::organization::OrganizationType;

    /// Test Coverage Mermaid Graph
    ///
    /// ```mermaid
    /// graph TD
    ///     TR[Test Repository] --> PS[Person Storage]
    ///     TR --> OS[Organization Storage]
    ///     PS --> CRUD[CRUD Operations]
    ///     OS --> CRUD
    ///     CRUD --> IDX[Index Management]
    /// ```

    #[tokio::test]
    async fn test_person_repository_save_and_load() {
        // Arrange
        let repo = InMemoryPersonRepository::new();
        let name = Name::new("John".to_string(), "Doe".to_string(), None);
        let email = Email::new("john@example.com".to_string()).unwrap();
        let person = Person::new(name.clone(), email.clone());
        let person_id = person.id();

        // Act - Save
        let save_result = repo.save(&person).await;
        assert!(save_result.is_ok());

        // Act - Load
        let load_result = repo.load(person_id).await;

        // Assert
        assert!(load_result.is_ok());
        let loaded_person = load_result.unwrap();
        assert_eq!(loaded_person.id(), person_id);
        assert_eq!(loaded_person.name, name);
        assert_eq!(loaded_person.email, email);
    }

    #[tokio::test]
    async fn test_person_repository_load_not_found() {
        // Arrange
        let repo = InMemoryPersonRepository::new();
        let non_existent_id = PersonId::new();

        // Act
        let result = repo.load(non_existent_id).await;

        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            IdentityError::PersonNotFound(id) => assert_eq!(id, non_existent_id),
            _ => panic!("Expected PersonNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_person_repository_email_exists() {
        // Arrange
        let repo = InMemoryPersonRepository::new();
        let name = Name::new("Jane".to_string(), "Doe".to_string(), None);
        let email = Email::new("jane@example.com".to_string()).unwrap();
        let person = Person::new(name, email);

        // Act - Save person
        repo.save(&person).await.unwrap();

        // Act - Check email exists
        let exists_result = repo.email_exists("jane@example.com").await;
        let not_exists_result = repo.email_exists("other@example.com").await;

        // Assert
        assert!(exists_result.is_ok());
        assert!(exists_result.unwrap());
        assert!(not_exists_result.is_ok());
        assert!(!not_exists_result.unwrap());
    }

    #[tokio::test]
    async fn test_person_repository_find_by_email() {
        // Arrange
        let repo = InMemoryPersonRepository::new();
        let name = Name::new("Bob".to_string(), "Smith".to_string(), None);
        let email = Email::new("bob@example.com".to_string()).unwrap();
        let person = Person::new(name.clone(), email.clone());

        // Act - Save person
        repo.save(&person).await.unwrap();

        // Act - Find by email
        let found_result = repo.find_by_email("bob@example.com").await;
        let not_found_result = repo.find_by_email("notfound@example.com").await;

        // Assert
        assert!(found_result.is_ok());
        let found_person = found_result.unwrap();
        assert!(found_person.is_some());
        assert_eq!(found_person.unwrap().email, email);

        assert!(not_found_result.is_ok());
        assert!(not_found_result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_organization_repository_save_and_load() {
        // Arrange
        let repo = InMemoryOrganizationRepository::new();
        let organization = Organization::new("Acme Corp".to_string(), OrganizationType::Company);
        let org_id = organization.id();

        // Act - Save
        let save_result = repo.save(&organization).await;
        assert!(save_result.is_ok());

        // Act - Load
        let load_result = repo.load(org_id).await;

        // Assert
        assert!(load_result.is_ok());
        let loaded_org = load_result.unwrap();
        assert_eq!(loaded_org.id(), org_id);
        assert_eq!(loaded_org.name, "Acme Corp");
        assert_eq!(loaded_org.org_type, OrganizationType::Company);
    }

    #[tokio::test]
    async fn test_organization_repository_load_not_found() {
        // Arrange
        let repo = InMemoryOrganizationRepository::new();
        let non_existent_id = OrganizationId::new();

        // Act
        let result = repo.load(non_existent_id).await;

        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            IdentityError::OrganizationNotFound(id) => assert_eq!(id, non_existent_id),
            _ => panic!("Expected OrganizationNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_organization_repository_name_exists() {
        // Arrange
        let repo = InMemoryOrganizationRepository::new();
        let organization = Organization::new("Tech Corp".to_string(), OrganizationType::Company);

        // Act - Save organization
        repo.save(&organization).await.unwrap();

        // Act - Check name exists
        let exists_result = repo.name_exists("Tech Corp").await;
        let not_exists_result = repo.name_exists("Other Corp").await;

        // Assert
        assert!(exists_result.is_ok());
        assert!(exists_result.unwrap());
        assert!(not_exists_result.is_ok());
        assert!(!not_exists_result.unwrap());
    }

    #[tokio::test]
    async fn test_organization_repository_find_by_name() {
        // Arrange
        let repo = InMemoryOrganizationRepository::new();
        let organization = Organization::new("Global Inc".to_string(), OrganizationType::Company);

        // Act - Save organization
        repo.save(&organization).await.unwrap();

        // Act - Find by name
        let found_result = repo.find_by_name("Global Inc").await;
        let not_found_result = repo.find_by_name("Not Found Inc").await;

        // Assert
        assert!(found_result.is_ok());
        let found_org = found_result.unwrap();
        assert!(found_org.is_some());
        assert_eq!(found_org.unwrap().name, "Global Inc");

        assert!(not_found_result.is_ok());
        assert!(not_found_result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_repository_find_all() {
        // Arrange
        let person_repo = InMemoryPersonRepository::new();
        let org_repo = InMemoryOrganizationRepository::new();

        // Create test data
        let person1 = Person::new(
            Name::new("Alice".to_string(), "Smith".to_string(), None),
            Email::new("alice@example.com".to_string()).unwrap(),
        );
        let person2 = Person::new(
            Name::new("Bob".to_string(), "Jones".to_string(), None),
            Email::new("bob@example.com".to_string()).unwrap(),
        );

        let org1 = Organization::new("Company A".to_string(), OrganizationType::Company);
        let org2 = Organization::new("Company B".to_string(), OrganizationType::NonProfit);

        // Act - Save entities
        person_repo.save(&person1).await.unwrap();
        person_repo.save(&person2).await.unwrap();
        org_repo.save(&org1).await.unwrap();
        org_repo.save(&org2).await.unwrap();

        // Act - Find all
        let all_persons = person_repo.find_all().await.unwrap();
        let all_orgs = org_repo.find_all().await.unwrap();

        // Assert
        assert_eq!(all_persons.len(), 2);
        assert_eq!(all_orgs.len(), 2);

        // Verify we have the right entities
        let person_emails: Vec<String> = all_persons.iter().map(|p| p.email.to_string()).collect();
        assert!(person_emails.contains(&"alice@example.com".to_string()));
        assert!(person_emails.contains(&"bob@example.com".to_string()));

        let org_names: Vec<String> = all_orgs.iter().map(|o| o.name.clone()).collect();
        assert!(org_names.contains(&"Company A".to_string()));
        assert!(org_names.contains(&"Company B".to_string()));
    }
}
