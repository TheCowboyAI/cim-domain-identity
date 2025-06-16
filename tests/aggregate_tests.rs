//! Aggregate Tests for Identity Domain
//!
//! User Story I1: Person Identity Creation
//! As a user, I want to create and manage my identity
//! So that I can be recognized and authenticated in the system
//!
//! ```mermaid
//! graph TD
//!     A[Create Person Command] --> B[Validate Input]
//!     B --> C{Valid?}
//!     C -->|Yes| D[Generate PersonCreated Event]
//!     C -->|No| E[Return Validation Error]
//!     D --> F[Update Aggregate State]
//!     F --> G[Return Success]
//! ```

use cim_domain_identity::{
    Person, PersonId, PersonCommand, PersonEvent,
    Organization, OrganizationCommand, OrganizationEvent, OrganizationType,
};
use cim_domain_identity::domain::{Email, Name, PhoneNumber, Address, TrustLevel, AuthMethod};
use cim_domain_identity::IdentityError;
use cim_domain::AggregateRoot;

#[cfg(test)]
mod person_aggregate_tests {
    use super::*;

    /// Test for User Story I1: Person Identity Creation
    #[test]
    fn test_person_identity_creation() {
        // Given: Valid person details
        let name = Name::new("Alice".to_string(), "Johnson".to_string(), Some("Marie".to_string()));
        let email = Email::new("alice.johnson@example.com".to_string()).unwrap();
        
        // When: Creating a new person
        let person = Person::new(name.clone(), email.clone());
        
        // Then: Person is created with correct identity
        assert_eq!(person.name.full_name(), "Alice Marie Johnson");
        assert_eq!(person.email.as_str(), "alice.johnson@example.com");
        assert!(person.phone.is_none());
        assert!(person.address.is_none());
        assert_eq!(person.trust_level, TrustLevel::Unverified);
        assert!(person.organization_ids.is_empty());
    }

    /// Test for User Story I2: Authentication Flow
    #[test]
    fn test_person_authentication_flow() {
        // Given: A person with credentials
        let mut person = Person::new(
            Name::new("Bob".to_string(), "Smith".to_string(), None),
            Email::new("bob@example.com".to_string()).unwrap()
        );
        
        // When: Setting credentials
        let credentials = cim_domain_identity::domain::Credentials {
            username: "bobsmith".to_string(),
            password_hash: "hashed_password_123".to_string(),
        };
        
        let command = PersonCommand::SetCredentials { credentials };
        let events = person.handle_command(command).unwrap();
        
        // Then: Credentials are set
        assert_eq!(events.len(), 1);
        match &events[0] {
            PersonEvent::CredentialsSet { username, .. } => {
                assert_eq!(username, "bobsmith");
            }
            _ => panic!("Expected CredentialsSet event"),
        }
        
        // Apply the event
        person.apply_event(&events[0]);
        
        // When: Authenticating with correct credentials
        let auth_command = PersonCommand::Authenticate {
            username: "bobsmith".to_string(),
            password_hash: "hashed_password_123".to_string(),
        };
        let auth_events = person.handle_command(auth_command).unwrap();
        
        // Then: Authentication succeeds
        assert_eq!(auth_events.len(), 1);
        match &auth_events[0] {
            PersonEvent::AuthenticationSucceeded { method, .. } => {
                assert_eq!(*method, AuthMethod::Password);
            }
            _ => panic!("Expected AuthenticationSucceeded event"),
        }
    }

    /// Test for User Story I3: Profile Management
    #[test]
    fn test_person_profile_updates() {
        // Given: An existing person
        let mut person = Person::new(
            Name::new("Carol".to_string(), "Davis".to_string(), None),
            Email::new("carol@example.com".to_string()).unwrap()
        );
        
        // When: Changing phone number
        let new_phone = PhoneNumber {
            country_code: "+1".to_string(),
            number: "555-1234".to_string(),
        };
        let command = PersonCommand::ChangePhone { phone_number: new_phone.clone() };
        let events = person.handle_command(command).unwrap();
        
        // Then: Phone change events are generated
        assert_eq!(events.len(), 1); // Only PhoneAdded since no existing phone
        match &events[0] {
            PersonEvent::PhoneAdded { phone_number, .. } => {
                assert_eq!(phone_number.country_code, "+1");
                assert_eq!(phone_number.number, "555-1234");
            }
            _ => panic!("Expected PhoneAdded event"),
        }
        
        // Apply the event
        person.apply_event(&events[0]);
        assert!(person.phone.is_some());
        
        // When: Changing address
        let new_address = Address {
            street: "123 Main St".to_string(),
            city: "Boston".to_string(),
            state: "MA".to_string(),
            postal_code: "02101".to_string(),
            country: "USA".to_string(),
        };
        let address_command = PersonCommand::ChangeAddress { address: new_address.clone() };
        let address_events = person.handle_command(address_command).unwrap();
        
        // Then: Address change events are generated
        assert_eq!(address_events.len(), 1); // Only AddressAdded since no existing address
        match &address_events[0] {
            PersonEvent::AddressAdded { address, .. } => {
                assert_eq!(address.city, "Boston");
            }
            _ => panic!("Expected AddressAdded event"),
        }
    }

    /// Test for User Story I11: Trust Level Management
    #[test]
    fn test_person_trust_level_changes() {
        // Given: A person with default trust level
        let mut person = Person::new(
            Name::new("David".to_string(), "Wilson".to_string(), None),
            Email::new("david@example.com".to_string()).unwrap()
        );
        
        // Verify initial trust level
        assert_eq!(person.trust_level, TrustLevel::Unverified);
        
        // When: Changing trust level to EmailVerified
        let command = PersonCommand::ChangeTrustLevel { 
            trust_level: TrustLevel::EmailVerified 
        };
        let events = person.handle_command(command).unwrap();
        
        // Then: Trust level is updated
        assert_eq!(events.len(), 1);
        match &events[0] {
            PersonEvent::TrustLevelChanged { old_level, new_level, .. } => {
                assert_eq!(*old_level, TrustLevel::Unverified);
                assert_eq!(*new_level, TrustLevel::EmailVerified);
            }
            _ => panic!("Expected TrustLevelChanged event"),
        }
        
        // Apply the event
        person.apply_event(&events[0]);
        assert_eq!(person.trust_level, TrustLevel::EmailVerified);
    }

    /// Test for validation errors
    #[test]
    fn test_person_invalid_email_validation() {
        // Given: Invalid email format
        let result = Email::new("invalid-email".to_string());
        
        // Then: Email validation fails
        assert!(result.is_err());
        match result {
            Err(IdentityError::InvalidEmail(email)) => {
                assert_eq!(email, "invalid-email");
            }
            _ => panic!("Expected InvalidEmail error"),
        }
    }
}

#[cfg(test)]
mod organization_aggregate_tests {
    use super::*;

    /// Test for Organization creation and management
    #[test]
    fn test_organization_creation_with_types() {
        // Test different organization types
        let test_cases = vec![
            ("Tech Startup", OrganizationType::Company),
            ("Open Source Foundation", OrganizationType::NonProfit),
            ("Federal Agency", OrganizationType::Government),
            ("University Research", OrganizationType::Educational),
            ("Other Org", OrganizationType::Other),
        ];
        
        for (name, org_type) in test_cases {
            // When: Creating organization
            let org = Organization::new(name.to_string(), org_type.clone());
            
            // Then: Organization is created correctly
            assert_eq!(org.name, name);
            assert_eq!(org.org_type, org_type);
            assert!(org.member_ids.is_empty());
            assert!(org.description.is_none());
        }
    }

    /// Test for Organization member management
    #[test]
    fn test_organization_member_lifecycle() {
        // Given: An organization
        let mut org = Organization::new("Acme Corp".to_string(), OrganizationType::Company);
        let person1 = PersonId::new();
        let person2 = PersonId::new();
        
        // When: Adding members
        let add_command1 = OrganizationCommand::AddMember { person_id: person1 };
        let events1 = org.handle_command(add_command1).unwrap();
        
        // Apply the event
        for event in &events1 {
            org.apply_event(event);
        }
        
        let add_command2 = OrganizationCommand::AddMember { person_id: person2 };
        let events2 = org.handle_command(add_command2).unwrap();
        
        // Apply the event
        for event in &events2 {
            org.apply_event(event);
        }
        
        // Then: Members are added
        assert_eq!(org.member_ids.len(), 2);
        assert!(org.member_ids.contains(&person1));
        assert!(org.member_ids.contains(&person2));
        
        // When: Removing a member
        let remove_command = OrganizationCommand::RemoveMember { person_id: person1 };
        let events3 = org.handle_command(remove_command).unwrap();
        
        // Apply the event
        for event in &events3 {
            org.apply_event(event);
        }
        
        // Then: Member is removed
        assert_eq!(org.member_ids.len(), 1);
        assert!(!org.member_ids.contains(&person1));
        assert!(org.member_ids.contains(&person2));
    }

    /// Test for Organization description updates
    #[test]
    fn test_organization_description_updates() {
        // Given: An organization
        let mut org = Organization::new("Global Corp".to_string(), OrganizationType::Company);
        
        // When: Setting organization description
        let description = "A global technology company focused on innovation".to_string();
        
        let command = OrganizationCommand::ChangeDescription { description: description.clone() };
        let events = org.handle_command(command).unwrap();
        
        // Then: Description is updated
        assert_eq!(events.len(), 1);
        
        // Apply the event
        for event in &events {
            org.apply_event(event);
        }
        
        assert_eq!(org.description, Some(description));
    }

    /// Test for duplicate member prevention
    #[test]
    fn test_organization_duplicate_member_prevention() {
        // Given: An organization with a member
        let mut org = Organization::new("Unique Corp".to_string(), OrganizationType::Company);
        let person_id = PersonId::new();
        
        // When: Adding the member first time
        let command = OrganizationCommand::AddMember { person_id };
        let events = org.handle_command(command).unwrap();
        
        // Apply the event
        for event in &events {
            org.apply_event(event);
        }
        
        // When: Trying to add the same member again
        let duplicate_command = OrganizationCommand::AddMember { person_id };
        let result = org.handle_command(duplicate_command).unwrap();
        
        // Then: No event is generated (already a member)
        assert!(result.is_empty());
        assert_eq!(org.member_ids.len(), 1); // Still only one member
    }
}

#[cfg(test)]
mod aggregate_interaction_tests {
    use super::*;

    /// Test for Person-Organization relationships
    #[test]
    fn test_person_organization_affiliation() {
        // Given: A person and an organization
        let person = Person::new(
            Name::new("Eve".to_string(), "Anderson".to_string(), None),
            Email::new("eve@example.com".to_string()).unwrap()
        );
        let person_id = person.id();
        let mut org = Organization::new("Eve's Company".to_string(), OrganizationType::Company);
        
        // When: Adding person to organization
        let command = OrganizationCommand::AddMember { person_id };
        let events = org.handle_command(command).unwrap();
        
        // Then: Affiliation is established
        assert_eq!(events.len(), 1);
        match &events[0] {
            OrganizationEvent::MemberAdded { person_id: added_id, organization_id } => {
                assert_eq!(*added_id, person_id);
                assert_eq!(*organization_id, org.id());
            }
            _ => panic!("Expected MemberAdded event"),
        }
    }
} 