//! Command handlers for the Identity context
//!
//! ## Test Coverage
//!
//! ```mermaid
//! graph TD
//!     CH[Command Handler] --> PA[Person Aggregate]
//!     CH --> OA[Organization Aggregate]
//!     PA --> E[Events Generated]
//!     OA --> E
//!     E --> ES[Event Store]
//!     E --> NP[NATS Publisher]
//!     
//!     subgraph Tests
//!         UT[Unit Tests] --> CH
//!         IT[Integration Tests] --> ES
//!         ET[Event Tests] --> NP
//!     end
//! ```

use async_trait::async_trait;
use std::sync::Arc;
use cim_domain::AggregateRoot;
use cim_domain::infrastructure::EventStore;
use crate::{
    Person, PersonId, PersonCommand, PersonEvent,
    Organization, OrganizationId, OrganizationCommand, OrganizationEvent,
    PersonRepository, OrganizationRepository,
    IdentityCommandHandler, IdentityResult, IdentityError,
};

/// Command handler implementation for Identity domain
pub struct IdentityCommandHandlerImpl {
    person_repository: Arc<dyn PersonRepository>,
    organization_repository: Arc<dyn OrganizationRepository>,
    event_store: Option<Arc<dyn EventStore>>, // Optional until Identity events are added to DomainEventEnum
}

impl IdentityCommandHandlerImpl {
    pub fn new(
        person_repository: Arc<dyn PersonRepository>,
        organization_repository: Arc<dyn OrganizationRepository>,
        event_store: Option<Arc<dyn EventStore>>,
    ) -> Self {
        Self {
            person_repository,
            organization_repository,
            event_store,
        }
    }

    /// Handle person registration with email uniqueness check
    async fn handle_person_registration(
        &self,
        command: PersonCommand,
    ) -> IdentityResult<(PersonId, Vec<PersonEvent>)> {
        if let PersonCommand::RegisterPerson { name, email } = command {
            // Check email uniqueness
            if self.person_repository.email_exists(email.as_str()).await? {
                return Err(IdentityError::PersonAlreadyExists(email.to_string()));
            }

            // Create new person aggregate
            let mut person = Person::new(name.clone(), email.clone());
            let person_id = person.id();

            // Handle command to generate events
            let events = person.handle_command(PersonCommand::RegisterPerson { name, email })?;

            // Apply events to aggregate
            for event in &events {
                person.apply_event(event);
            }

            // Save aggregate
            self.person_repository.save(&person).await?;

            // TODO: Publish events to event store when Identity events are added to DomainEventEnum
            if let Some(_event_store) = &self.event_store {
                // Events will be published once Identity events are integrated into DomainEventEnum
            }

            Ok((person_id, events))
        } else {
            unreachable!("Expected RegisterPerson command")
        }
    }

    /// Handle organization creation with name uniqueness check
    async fn handle_organization_creation(
        &self,
        command: OrganizationCommand,
    ) -> IdentityResult<(OrganizationId, Vec<OrganizationEvent>)> {
        if let OrganizationCommand::CreateOrganization { name, org_type } = command {
            // Check name uniqueness
            if self.organization_repository.name_exists(&name).await? {
                return Err(IdentityError::OrganizationAlreadyExists(name));
            }

            // Create new organization aggregate
            let mut organization = Organization::new(name.clone(), org_type);
            let org_id = organization.id();

            // Handle command to generate events
            let events = organization.handle_command(OrganizationCommand::CreateOrganization { name, org_type })?;

            // Apply events to aggregate
            for event in &events {
                organization.apply_event(event);
            }

            // Save aggregate
            self.organization_repository.save(&organization).await?;

            // TODO: Publish events to event store when Identity events are added to DomainEventEnum
            if let Some(_event_store) = &self.event_store {
                // Events will be published once Identity events are integrated into DomainEventEnum
            }

            Ok((org_id, events))
        } else {
            unreachable!("Expected CreateOrganization command")
        }
    }
}

#[async_trait]
impl IdentityCommandHandler for IdentityCommandHandlerImpl {
    /// Handle a person command
    ///
    /// ## Test Coverage
    ///
    /// ```mermaid
    /// graph LR
    ///     PC[Person Command] --> VR[Validation & Repository]
    ///     VR --> AG[Aggregate Handling]
    ///     AG --> ES[Event Store]
    ///     AG --> R[Repository Save]
    /// ```
    async fn handle_person_command(&self, person_id: PersonId, command: PersonCommand) -> IdentityResult<()> {
        match &command {
            PersonCommand::RegisterPerson { .. } => {
                // Special handling for registration
                let (_person_id, _events) = self.handle_person_registration(command).await?;
                Ok(())
            }
            _ => {
                // Load existing person
                let mut person = self.person_repository.load(person_id).await?;

                // Handle command
                let events = person.handle_command(command)?;

                // Apply events
                for event in &events {
                    person.apply_event(event);
                }

                // Save aggregate
                self.person_repository.save(&person).await?;

                // TODO: Publish events to event store when Identity events are added to DomainEventEnum
                if let Some(_event_store) = &self.event_store {
                    if !events.is_empty() {
                        // Events will be published once Identity events are integrated into DomainEventEnum
                    }
                }

                Ok(())
            }
        }
    }

    /// Handle an organization command
    ///
    /// ## Test Coverage
    ///
    /// ```mermaid
    /// graph LR
    ///     OC[Organization Command] --> VR[Validation & Repository]
    ///     VR --> AG[Aggregate Handling]
    ///     AG --> ES[Event Store]
    ///     AG --> R[Repository Save]
    /// ```
    async fn handle_organization_command(&self, org_id: OrganizationId, command: OrganizationCommand) -> IdentityResult<()> {
        match &command {
            OrganizationCommand::CreateOrganization { .. } => {
                // Special handling for creation
                let (_org_id, _events) = self.handle_organization_creation(command).await?;
                Ok(())
            }
            _ => {
                // Load existing organization
                let mut organization = self.organization_repository.load(org_id).await?;

                // Handle command
                let events = organization.handle_command(command)?;

                // Apply events
                for event in &events {
                    organization.apply_event(event);
                }

                // Save aggregate
                self.organization_repository.save(&organization).await?;

                // TODO: Publish events to event store when Identity events are added to DomainEventEnum
                if let Some(_event_store) = &self.event_store {
                    if !events.is_empty() {
                        // Events will be published once Identity events are integrated into DomainEventEnum
                    }
                }

                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Name, Email}; // Use re-exported types from lib.rs
    use std::collections::HashMap;
    use async_trait::async_trait;

    /// Test Coverage Mermaid Graph
    ///
    /// ```mermaid
    /// graph TD
    ///     TR[Test Repository] --> CH[Command Handler]
    ///     CH --> PC[Person Commands]
    ///     CH --> OC[Organization Commands]
    ///     PC --> PE[Person Events]
    ///     OC --> OE[Organization Events]
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
    }

    #[async_trait]
    impl PersonRepository for MockPersonRepository {
        async fn load(&self, id: PersonId) -> IdentityResult<Person> {
            let persons = self.persons.lock().unwrap();
            persons.get(&id).cloned().ok_or(IdentityError::PersonNotFound(id))
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
                    let full_name = format!("{} {} {}", 
                        person.name.first,
                        person.name.middle.as_ref().unwrap_or(&"".to_string()),
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
    }

    #[async_trait]
    impl OrganizationRepository for MockOrganizationRepository {
        async fn load(&self, id: OrganizationId) -> IdentityResult<Organization> {
            let organizations = self.organizations.lock().unwrap();
            organizations.get(&id).cloned().ok_or(IdentityError::OrganizationNotFound(id))
        }

        async fn save(&self, organization: &Organization) -> IdentityResult<()> {
            let mut organizations = self.organizations.lock().unwrap();
            let mut names = self.names.lock().unwrap();
            
            organizations.insert(organization.id(), organization.clone());
            names.insert(organization.name.clone()); // Access public field directly
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
    async fn test_person_registration_success() {
        // Arrange
        let person_repo = Arc::new(MockPersonRepository::new());
        let org_repo = Arc::new(MockOrganizationRepository::new());
        
        let handler = IdentityCommandHandlerImpl::new(person_repo, org_repo, None);
        
        let name = Name::new("John".to_string(), "Doe".to_string(), None);
        let email = Email::new("john@example.com".to_string()).unwrap();
        let command = PersonCommand::RegisterPerson { name: name.clone(), email: email.clone() };

        // Act
        let result = handler.handle_person_registration(command).await;

        // Assert
        assert!(result.is_ok());
        let (person_id, events) = result.unwrap();
        assert_eq!(events.len(), 1);
        
        if let PersonEvent::PersonRegistered { person_id: event_person_id, name: event_name, email: event_email } = &events[0] {
            assert_eq!(*event_person_id, person_id);
            assert_eq!(*event_name, name);
            assert_eq!(*event_email, email);
        } else {
            panic!("Expected PersonRegistered event");
        }
    }

    #[tokio::test]
    async fn test_person_registration_duplicate_email() {
        // Arrange
        let person_repo = Arc::new(MockPersonRepository::new());
        let org_repo = Arc::new(MockOrganizationRepository::new());
        
        let handler = IdentityCommandHandlerImpl::new(person_repo.clone(), org_repo, None);
        
        let email = "john@example.com";
        
        // First registration
        let name1 = Name::new("John".to_string(), "Doe".to_string(), None);
        let email1 = Email::new(email.to_string()).unwrap();
        let command1 = PersonCommand::RegisterPerson { name: name1, email: email1 };
        let _ = handler.handle_person_registration(command1).await.unwrap();

        // Second registration with same email
        let name2 = Name::new("Jane".to_string(), "Doe".to_string(), None);
        let email2 = Email::new(email.to_string()).unwrap();
        let command2 = PersonCommand::RegisterPerson { name: name2, email: email2 };

        // Act
        let result = handler.handle_person_registration(command2).await;

        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            IdentityError::PersonAlreadyExists(e) => assert_eq!(e, email),
            _ => panic!("Expected PersonAlreadyExists error"),
        }
    }

    #[tokio::test]
    async fn test_organization_creation_success() {
        use crate::domain::organization::OrganizationType;
        
        // Arrange
        let person_repo = Arc::new(MockPersonRepository::new());
        let org_repo = Arc::new(MockOrganizationRepository::new());
        
        let handler = IdentityCommandHandlerImpl::new(person_repo, org_repo, None);
        
        let name = "Acme Corp".to_string();
        let org_type = OrganizationType::Company;
        let command = OrganizationCommand::CreateOrganization { name: name.clone(), org_type };

        // Act
        let result = handler.handle_organization_creation(command).await;

        // Assert
        assert!(result.is_ok());
        let (org_id, events) = result.unwrap();
        assert_eq!(events.len(), 1);
        
        if let OrganizationEvent::OrganizationCreated { organization_id, name: event_name, org_type: event_type } = &events[0] {
            assert_eq!(*organization_id, org_id);
            assert_eq!(*event_name, name);
            assert_eq!(*event_type, org_type);
        } else {
            panic!("Expected OrganizationCreated event");
        }
    }

    #[tokio::test] 
    async fn test_person_command_handling() {
        // Arrange
        let person_repo = Arc::new(MockPersonRepository::new());
        let org_repo = Arc::new(MockOrganizationRepository::new());
        
        let handler = IdentityCommandHandlerImpl::new(person_repo.clone(), org_repo, None);
        
        // First register a person
        let person_id = PersonId::new();
        let name = Name::new("John".to_string(), "Doe".to_string(), None);
        let email = Email::new("john@example.com".to_string()).unwrap();
        let register_command = PersonCommand::RegisterPerson { name, email };
        
        let result = handler.handle_person_command(person_id, register_command).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_organization_command_handling() {
        use crate::domain::organization::OrganizationType;
        
        // Arrange
        let person_repo = Arc::new(MockPersonRepository::new());
        let org_repo = Arc::new(MockOrganizationRepository::new());
        
        let handler = IdentityCommandHandlerImpl::new(person_repo, org_repo, None);
        
        let org_id = OrganizationId::new();
        let name = "Acme Corp".to_string();
        let org_type = OrganizationType::Company;
        let command = OrganizationCommand::CreateOrganization { name, org_type };

        // Act
        let result = handler.handle_organization_command(org_id, command).await;

        // Assert
        assert!(result.is_ok());
    }
}
