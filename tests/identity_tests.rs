//! Tests for the Identity Context
//!
//! User Story F16: Identity Context Foundation
//! As a developer, I want a bounded context for identity management
//! So that Person and Organization aggregates are properly isolated
//!
//! ```mermaid
//! graph TD
//!     A[Identity Context] --> B[Person Aggregate]
//!     A --> C[Organization Aggregate]
//!     B --> D[Person Commands]
//!     B --> E[Person Events]
//!     C --> F[Organization Commands]
//!     C --> G[Organization Events]
//!     A --> H[Conceptual Integration]
//! ```

use cim_domain_identity::domain::{Email, Name};
use cim_domain_identity::{
    IdentityDimensions, Organization, OrganizationCommand, OrganizationEvent, OrganizationType,
    Person, PersonCommand, PersonEvent, PersonId,
};

#[test]
fn test_person_creation() {
    // Given: Valid person details
    let name = Name::new("John".to_string(), "Doe".to_string(), None);
    let email = Email::new("john.doe@example.com".to_string()).unwrap();

    // When: Creating a new person
    let person = Person::new(name.clone(), email.clone());

    // Then: Person is created with correct details
    assert_eq!(person.name.full_name(), "John Doe");
    assert_eq!(person.email.as_str(), "john.doe@example.com");
}

#[test]
fn test_person_command_handling() {
    // Given: A person aggregate
    let name = Name::new("Jane".to_string(), "Smith".to_string(), None);
    let email = Email::new("jane.smith@example.com".to_string()).unwrap();
    let mut person = Person::new(name.clone(), email.clone());

    // When: Changing email
    let new_email = Email::new("jane.s@example.com".to_string()).unwrap();
    let command = PersonCommand::ChangeEmail {
        new_email: new_email.clone(),
    };
    let events = person.handle_command(command).unwrap();

    // Then: Email change events are generated
    assert_eq!(events.len(), 2); // EmailRemoved + EmailAdded
    match &events[0] {
        PersonEvent::EmailRemoved { old_email, .. } => {
            assert_eq!(old_email.as_str(), "jane.smith@example.com");
        }
        _ => panic!("Expected EmailRemoved event"),
    }
    match &events[1] {
        PersonEvent::EmailAdded {
            new_email: updated, ..
        } => {
            assert_eq!(updated.as_str(), "jane.s@example.com");
        }
        _ => panic!("Expected EmailAdded event"),
    }
}

#[test]
fn test_organization_creation() {
    // Given: Valid organization details
    let name = "Acme Corp".to_string();
    let org_type = OrganizationType::Company;

    // When: Creating a new organization
    let org = Organization::new(name.clone(), org_type);

    // Then: Organization is created with correct details
    assert_eq!(org.name, "Acme Corp");
    assert!(matches!(org.org_type, OrganizationType::Company));
}

#[test]
fn test_organization_member_management() {
    // Given: An organization
    let mut org = Organization::new("Tech Inc".to_string(), OrganizationType::Company);
    let person_id = PersonId::new();

    // When: Adding a member
    let command = OrganizationCommand::AddMember { person_id };
    let events = org.handle_command(command).unwrap();

    // Then: Member added event is generated
    assert_eq!(events.len(), 1);
    match &events[0] {
        OrganizationEvent::MemberAdded {
            person_id: added_id,
            ..
        } => {
            assert_eq!(*added_id, person_id);
        }
        _ => panic!("Expected MemberAdded event"),
    }
}

#[test]
fn test_identity_dimensions() {
    // Given: Identity dimensions
    let dimensions = IdentityDimensions::all();

    // Then: All expected dimensions are present
    assert_eq!(dimensions.len(), 5);

    let dim_names: Vec<_> = dimensions.iter().map(|d| &d.name).collect();
    assert!(dim_names.contains(&&"trust_level".to_string()));
    assert!(dim_names.contains(&&"activity_level".to_string()));
    assert!(dim_names.contains(&&"connectivity".to_string()));
    assert!(dim_names.contains(&&"organization_size".to_string()));
    assert!(dim_names.contains(&&"domain_influence".to_string()));
}
