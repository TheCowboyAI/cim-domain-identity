//! Query handlers for the Identity context
//!
//! ## Test Coverage
//!
//! ```mermaid
//! graph TD
//!     QH[Query Handler] --> PR[Person Repository]
//!     QH --> OR[Organization Repository]
//!     PR --> PQ[Person Queries]
//!     OR --> OQ[Organization Queries]
//!     
//!     subgraph Tests
//!         UT[Unit Tests] --> QH
//!         IT[Integration Tests] --> PR
//!         IT --> OR
//!     end
//! ```

use async_trait::async_trait;
use std::sync::Arc;
use crate::{
    Person, PersonId, Organization, OrganizationId,
    PersonRepository, OrganizationRepository,
    IdentityQueryHandler, IdentityResult,
};

/// Query handler implementation for Identity domain
pub struct IdentityQueryHandlerImpl {
    person_repository: Arc<dyn PersonRepository>,
    organization_repository: Arc<dyn OrganizationRepository>,
}

impl IdentityQueryHandlerImpl {
    pub fn new(
        person_repository: Arc<dyn PersonRepository>,
        organization_repository: Arc<dyn OrganizationRepository>,
    ) -> Self {
        Self {
            person_repository,
            organization_repository,
        }
    }
}

#[async_trait]
impl IdentityQueryHandler for IdentityQueryHandlerImpl {
    /// Find a person by ID
    ///
    /// ## Test Coverage
    ///
    /// ```mermaid
    /// graph LR
    ///     PID[Person ID] --> R[Repository]
    ///     R --> P[Person Result]
    /// ```
    async fn find_person_by_id(&self, person_id: PersonId) -> IdentityResult<Option<Person>> {
        match self.person_repository.load(person_id).await {
            Ok(person) => Ok(Some(person)),
            Err(crate::IdentityError::PersonNotFound(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Find a person by email
    async fn find_person_by_email(&self, email: &str) -> IdentityResult<Option<Person>> {
        self.person_repository.find_by_email(email).await
    }

    /// Find an organization by ID
    async fn find_organization_by_id(&self, org_id: OrganizationId) -> IdentityResult<Option<Organization>> {
        match self.organization_repository.load(org_id).await {
            Ok(organization) => Ok(Some(organization)),
            Err(crate::IdentityError::OrganizationNotFound(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Find an organization by name
    async fn find_organization_by_name(&self, name: &str) -> IdentityResult<Option<Organization>> {
        self.organization_repository.find_by_name(name).await
    }

    /// Find organizations where a person is a member
    async fn find_organizations_for_person(&self, person_id: PersonId) -> IdentityResult<Vec<Organization>> {
        // Load the person to verify they exist
        let _person = match self.person_repository.load(person_id).await {
            Ok(person) => person,
            Err(crate::IdentityError::PersonNotFound(_)) => return Ok(Vec::new()),
            Err(e) => return Err(e),
        };

        // Get all organizations and filter by membership
        // Note: This is a basic implementation. In a real system, this would likely
        // use a more efficient query or separate membership tracking
        let organizations = self.organization_repository.find_all().await?;
        let member_organizations: Vec<Organization> = organizations
            .into_iter()
            .filter(|org| org.member_ids.contains(&person_id))
            .collect();

        Ok(member_organizations)
    }

    /// Find members of an organization
    async fn find_organization_members(&self, org_id: OrganizationId) -> IdentityResult<Vec<Person>> {
        // Load the organization to get member IDs
        let organization = match self.organization_repository.load(org_id).await {
            Ok(org) => org,
            Err(crate::IdentityError::OrganizationNotFound(_)) => return Ok(Vec::new()),
            Err(e) => return Err(e),
        };

        // Load each member person
        let mut members = Vec::new();
        for member_id in &organization.member_ids {
            if let Ok(person) = self.person_repository.load(*member_id).await {
                members.push(person);
            }
        }

        Ok(members)
    }

    /// Find administrators of an organization
    async fn find_organization_admins(&self, org_id: OrganizationId) -> IdentityResult<Vec<Person>> {
        // Load the organization to get admin IDs
        let organization = match self.organization_repository.load(org_id).await {
            Ok(org) => org,
            Err(crate::IdentityError::OrganizationNotFound(_)) => return Ok(Vec::new()),
            Err(e) => return Err(e),
        };

        // Load each admin person
        let mut admins = Vec::new();
        for admin_id in &organization.admin_ids {
            if let Ok(person) = self.person_repository.load(*admin_id).await {
                admins.push(person);
            }
        }

        Ok(admins)
    }

    /// Search people by name
    async fn search_people_by_name(&self, name_query: &str) -> IdentityResult<Vec<Person>> {
        self.person_repository.search_by_name(name_query).await
    }

    /// Search organizations by name
    async fn search_organizations_by_name(&self, name_query: &str) -> IdentityResult<Vec<Organization>> {
        self.organization_repository.search_by_name(name_query).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Name, Email};
    use std::collections::HashMap;
    use async_trait::async_trait;
    use cim_domain::AggregateRoot;

    /// Test Coverage Mermaid Graph
    ///
    /// ```mermaid
    /// graph TD
    ///     TR[Test Repository] --> QH[Query Handler]
    ///     QH --> PQ[Person Queries]
    ///     QH --> OQ[Organization Queries]
    ///     PQ --> PR[Person Results]
    ///     OQ --> OR[Organization Results]
    /// ```

    struct MockPersonRepository {
        persons: Arc<std::sync::Mutex<HashMap<PersonId, Person>>>,
        emails: Arc<std::sync::Mutex<std::collections::HashSet<String>>>,
    }

    impl MockPersonRepository {
        fn new() -> Self {
            Self {
                persons: Arc::new(std::sync::Mutex::new(HashMap::new())),
                emails: Arc::new(std::sync::Mutex::new(std::collections::HashSet::new())),
            }
        }

        fn add_person(&self, person: Person) {
            let mut persons = self.persons.lock().unwrap();
            let mut emails = self.emails.lock().unwrap();
            
            emails.insert(person.email.to_string());
            persons.insert(person.id(), person);
        }
    }

    #[async_trait]
    impl PersonRepository for MockPersonRepository {
        async fn load(&self, id: PersonId) -> IdentityResult<Person> {
            let persons = self.persons.lock().unwrap();
            persons.get(&id).cloned().ok_or(crate::IdentityError::PersonNotFound(id))
        }

        async fn save(&self, person: &Person) -> IdentityResult<()> {
            let mut persons = self.persons.lock().unwrap();
            let mut emails = self.emails.lock().unwrap();
            
            persons.insert(person.id(), person.clone());
            emails.insert(person.email.to_string());
            Ok(())
        }

        async fn email_exists(&self, email: &str) -> IdentityResult<bool> {
            let emails = self.emails.lock().unwrap();
            Ok(emails.contains(email))
        }

        async fn find_by_email(&self, email: &str) -> IdentityResult<Option<Person>> {
            let persons = self.persons.lock().unwrap();
            
            for (_person_id, person) in persons.iter() {
                if person.email.to_string() == email {
                    return Ok(Some(person.clone()));
                }
            }
            Ok(None)
        }

        async fn find_all(&self) -> IdentityResult<Vec<Person>> {
            let persons = self.persons.lock().unwrap();
            Ok(persons.values().cloned().collect())
        }

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

    struct MockOrganizationRepository {
        organizations: Arc<std::sync::Mutex<HashMap<OrganizationId, Organization>>>,
        names: Arc<std::sync::Mutex<std::collections::HashSet<String>>>,
    }

    impl MockOrganizationRepository {
        fn new() -> Self {
            Self {
                organizations: Arc::new(std::sync::Mutex::new(HashMap::new())),
                names: Arc::new(std::sync::Mutex::new(std::collections::HashSet::new())),
            }
        }

        fn add_organization(&self, organization: Organization) {
            let mut organizations = self.organizations.lock().unwrap();
            let mut names = self.names.lock().unwrap();
            
            names.insert(organization.name.clone());
            organizations.insert(organization.id(), organization);
        }
    }

    #[async_trait]
    impl OrganizationRepository for MockOrganizationRepository {
        async fn load(&self, id: OrganizationId) -> IdentityResult<Organization> {
            let organizations = self.organizations.lock().unwrap();
            organizations.get(&id).cloned().ok_or(crate::IdentityError::OrganizationNotFound(id))
        }

        async fn save(&self, organization: &Organization) -> IdentityResult<()> {
            let mut organizations = self.organizations.lock().unwrap();
            let mut names = self.names.lock().unwrap();
            
            organizations.insert(organization.id(), organization.clone());
            names.insert(organization.name.clone());
            Ok(())
        }

        async fn name_exists(&self, name: &str) -> IdentityResult<bool> {
            let names = self.names.lock().unwrap();
            Ok(names.contains(name))
        }

        async fn find_by_name(&self, name: &str) -> IdentityResult<Option<Organization>> {
            let organizations = self.organizations.lock().unwrap();
            
            for (_org_id, org) in organizations.iter() {
                if org.name == name {
                    return Ok(Some(org.clone()));
                }
            }
            Ok(None)
        }

        async fn find_all(&self) -> IdentityResult<Vec<Organization>> {
            let organizations = self.organizations.lock().unwrap();
            Ok(organizations.values().cloned().collect())
        }

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

    #[tokio::test]
    async fn test_find_person_by_id_success() {
        // Arrange
        let person_repo = Arc::new(MockPersonRepository::new());
        let org_repo = Arc::new(MockOrganizationRepository::new());
        
        let name = Name::new("John".to_string(), "Doe".to_string(), None);
        let email = Email::new("john@example.com".to_string()).unwrap();
        let person = Person::new(name.clone(), email.clone());
        let person_id = person.id();
        
        person_repo.add_person(person.clone());
        
        let query_handler = IdentityQueryHandlerImpl::new(person_repo, org_repo);

        // Act
        let result = query_handler.find_person_by_id(person_id).await;

        // Assert
        assert!(result.is_ok());
        let found_person = result.unwrap();
        assert!(found_person.is_some());
        let found_person = found_person.unwrap();
        assert_eq!(found_person.id(), person_id);
        assert_eq!(found_person.name, name);
        assert_eq!(found_person.email, email);
    }

    #[tokio::test]
    async fn test_find_person_by_id_not_found() {
        // Arrange
        let person_repo = Arc::new(MockPersonRepository::new());
        let org_repo = Arc::new(MockOrganizationRepository::new());
        
        let query_handler = IdentityQueryHandlerImpl::new(person_repo, org_repo);
        let non_existent_id = PersonId::new();

        // Act
        let result = query_handler.find_person_by_id(non_existent_id).await;

        // Assert
        assert!(result.is_ok());
        let found_person = result.unwrap();
        assert!(found_person.is_none());
    }

    #[tokio::test]
    async fn test_find_organization_by_id_success() {
        use crate::domain::organization::OrganizationType;
        
        // Arrange
        let person_repo = Arc::new(MockPersonRepository::new());
        let org_repo = Arc::new(MockOrganizationRepository::new());
        
        let organization = Organization::new("Acme Corp".to_string(), OrganizationType::Company);
        let org_id = organization.id();
        
        org_repo.add_organization(organization.clone());
        
        let query_handler = IdentityQueryHandlerImpl::new(person_repo, org_repo);

        // Act
        let result = query_handler.find_organization_by_id(org_id).await;

        // Assert
        assert!(result.is_ok());
        let found_org = result.unwrap();
        assert!(found_org.is_some());
        let found_org = found_org.unwrap();
        assert_eq!(found_org.id(), org_id);
        assert_eq!(found_org.name, "Acme Corp");
        assert_eq!(found_org.org_type, OrganizationType::Company);
    }

    #[tokio::test]
    async fn test_find_organization_by_id_not_found() {
        // Arrange
        let person_repo = Arc::new(MockPersonRepository::new());
        let org_repo = Arc::new(MockOrganizationRepository::new());
        
        let query_handler = IdentityQueryHandlerImpl::new(person_repo, org_repo);
        let non_existent_id = OrganizationId::new();

        // Act
        let result = query_handler.find_organization_by_id(non_existent_id).await;

        // Assert
        assert!(result.is_ok());
        let found_org = result.unwrap();
        assert!(found_org.is_none());
    }

    #[tokio::test]
    async fn test_find_person_by_email_placeholder() {
        // Arrange
        let person_repo = Arc::new(MockPersonRepository::new());
        let org_repo = Arc::new(MockOrganizationRepository::new());
        
        let query_handler = IdentityQueryHandlerImpl::new(person_repo, org_repo);

        // Act
        let result = query_handler.find_person_by_email("john@example.com").await;

        // Assert - Currently returns None as it's not implemented
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_search_methods_placeholders() {
        // Arrange
        let person_repo = Arc::new(MockPersonRepository::new());
        let org_repo = Arc::new(MockOrganizationRepository::new());
        
        let query_handler = IdentityQueryHandlerImpl::new(person_repo, org_repo);

        // Act & Assert
        let people_result = query_handler.search_people_by_name("John").await;
        assert!(people_result.is_ok());
        assert!(people_result.unwrap().is_empty());

        let org_result = query_handler.search_organizations_by_name("Acme").await;
        assert!(org_result.is_ok());
        assert!(org_result.unwrap().is_empty());

        let person_id = PersonId::new();
        let person_orgs = query_handler.find_organizations_for_person(person_id).await;
        assert!(person_orgs.is_ok());
        assert!(person_orgs.unwrap().is_empty());

        let org_id = OrganizationId::new();
        let members = query_handler.find_organization_members(org_id).await;
        assert!(members.is_ok());
        assert!(members.unwrap().is_empty());

        let admins = query_handler.find_organization_admins(org_id).await;
        assert!(admins.is_ok());
        assert!(admins.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_find_person_by_email_functional() {
        // Arrange
        let person_repo = Arc::new(MockPersonRepository::new());
        let org_repo = Arc::new(MockOrganizationRepository::new());
        
        let name = Name::new("Alice".to_string(), "Smith".to_string(), None);
        let email = Email::new("alice@example.com".to_string()).unwrap();
        let person = Person::new(name.clone(), email.clone());
        
        person_repo.add_person(person.clone());
        
        let query_handler = IdentityQueryHandlerImpl::new(person_repo, org_repo);

        // Act
        let result = query_handler.find_person_by_email("alice@example.com").await;

        // Assert
        assert!(result.is_ok());
        let found_person = result.unwrap();
        assert!(found_person.is_some());
        let found_person = found_person.unwrap();
        assert_eq!(found_person.name, name);
        assert_eq!(found_person.email, email);
    }

    #[tokio::test]
    async fn test_find_organization_by_name_functional() {
        use crate::domain::organization::OrganizationType;
        
        // Arrange
        let person_repo = Arc::new(MockPersonRepository::new());
        let org_repo = Arc::new(MockOrganizationRepository::new());
        
        let organization = Organization::new("Tech Corp".to_string(), OrganizationType::Company);
        let org_name = organization.name.clone();
        let org_type = organization.org_type;
        
        org_repo.add_organization(organization.clone());
        
        let query_handler = IdentityQueryHandlerImpl::new(person_repo, org_repo);

        // Act
        let result = query_handler.find_organization_by_name("Tech Corp").await;

        // Assert
        assert!(result.is_ok());
        let found_org = result.unwrap();
        assert!(found_org.is_some());
        let found_org = found_org.unwrap();
        assert_eq!(found_org.name, org_name);
        assert_eq!(found_org.org_type, org_type);
    }

    #[tokio::test]
    async fn test_search_people_by_name_functional() {
        // Arrange
        let person_repo = Arc::new(MockPersonRepository::new());
        let org_repo = Arc::new(MockOrganizationRepository::new());
        
        // Create test people
        let person1 = Person::new(
            Name::new("John".to_string(), "Doe".to_string(), None),
            Email::new("john@example.com".to_string()).unwrap(),
        );
        let person2 = Person::new(
            Name::new("Jane".to_string(), "Smith".to_string(), None),
            Email::new("jane@example.com".to_string()).unwrap(),
        );
        let person3 = Person::new(
            Name::new("Bob".to_string(), "Johnson".to_string(), None),
            Email::new("bob@example.com".to_string()).unwrap(),
        );
        
        person_repo.add_person(person1);
        person_repo.add_person(person2);
        person_repo.add_person(person3);
        
        let query_handler = IdentityQueryHandlerImpl::new(person_repo, org_repo);

        // Act - Search for "John"
        let john_results = query_handler.search_people_by_name("John").await.unwrap();
        
        // Act - Search for "Johnson"
        let johnson_results = query_handler.search_people_by_name("Johnson").await.unwrap();
        
        // Act - Search for "Smith"
        let smith_results = query_handler.search_people_by_name("Smith").await.unwrap();

        // Assert
        assert_eq!(john_results.len(), 2); // John Doe and Bob Johnson
        assert_eq!(johnson_results.len(), 1); // Bob Johnson
        assert_eq!(smith_results.len(), 1); // Jane Smith
        
        // Verify specific matches
        let john_emails: Vec<String> = john_results.iter().map(|p| p.email.to_string()).collect();
        assert!(john_emails.contains(&"john@example.com".to_string()));
        assert!(john_emails.contains(&"bob@example.com".to_string()));
    }

    #[tokio::test]
    async fn test_search_organizations_by_name_functional() {
        use crate::domain::organization::OrganizationType;
        
        // Arrange
        let person_repo = Arc::new(MockPersonRepository::new());
        let org_repo = Arc::new(MockOrganizationRepository::new());
        
        // Create test organizations
        let org1 = Organization::new("Tech Corp".to_string(), OrganizationType::Company);
        let org2 = Organization::new("Tech Solutions".to_string(), OrganizationType::Company);
        let org3 = Organization::new("Green Peace Foundation".to_string(), OrganizationType::NonProfit);
        
        org_repo.add_organization(org1);
        org_repo.add_organization(org2);
        org_repo.add_organization(org3);
        
        let query_handler = IdentityQueryHandlerImpl::new(person_repo, org_repo);

        // Act - Search for "Tech"
        let tech_results = query_handler.search_organizations_by_name("Tech").await.unwrap();
        
        // Act - Search for "Foundation"
        let foundation_results = query_handler.search_organizations_by_name("Foundation").await.unwrap();
        
        // Act - Search for "Corp"
        let corp_results = query_handler.search_organizations_by_name("Corp").await.unwrap();

        // Assert
        assert_eq!(tech_results.len(), 2); // Tech Corp and Tech Solutions
        assert_eq!(foundation_results.len(), 1); // Green Peace Foundation
        assert_eq!(corp_results.len(), 1); // Tech Corp
        
        // Verify specific matches
        let tech_names: Vec<String> = tech_results.iter().map(|o| o.name.clone()).collect();
        assert!(tech_names.contains(&"Tech Corp".to_string()));
        assert!(tech_names.contains(&"Tech Solutions".to_string()));
    }
}
