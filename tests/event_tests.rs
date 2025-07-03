//! Event Tests for Identity Domain
//!
//! User Story I1: Person Identity Creation Events
//! As a system, I want to emit events for all identity changes
//! So that other systems can react to identity updates
//!
//! ```mermaid
//! graph TD
//!     A[Domain Action] --> B[Generate Event]
//!     B --> C[Event Contains Data]
//!     C --> D[Event Has Metadata]
//!     D --> E[Event Is Immutable]
//!     E --> F[Event Can Be Serialized]
//! ```

use chrono::Utc;
use cim_domain_identity::domain::{
    Address, AuthMethod, Email, MfaMethod, Name, PhoneNumber, TrustLevel,
};
use cim_domain_identity::{
    OrganizationEvent, OrganizationId, OrganizationType, PersonEvent, PersonId,
};

#[cfg(test)]
mod person_event_tests {
    use super::*;

    /// Test for User Story I1: PersonRegistered Event
    #[test]
    fn test_person_registered_event() {
        // Given: Person creation data
        let person_id = PersonId::new();
        let name = Name::new(
            "Alice".to_string(),
            "Johnson".to_string(),
            Some("Marie".to_string()),
        );
        let email = Email::new("alice@example.com".to_string()).unwrap();

        // When: Creating PersonRegistered event
        let event = PersonEvent::PersonRegistered {
            person_id,
            name: name.clone(),
            email: email.clone(),
        };

        // Then: Event contains correct data
        match event {
            PersonEvent::PersonRegistered {
                person_id: id,
                name: n,
                email: e,
            } => {
                assert_eq!(id, person_id);
                assert_eq!(n.full_name(), "Alice Marie Johnson");
                assert_eq!(e.as_str(), "alice@example.com");
            }
            _ => panic!("Expected PersonRegistered event"),
        }
    }

    /// Test for User Story I2: Authentication Events
    #[test]
    fn test_authentication_events() {
        // Given: Authentication data
        let person_id = PersonId::new();
        let timestamp = Utc::now();

        // When: Creating authentication success event
        let success_event = PersonEvent::AuthenticationSucceeded {
            person_id,
            method: AuthMethod::Password,
            timestamp,
        };

        // Then: Event contains authentication data
        match success_event {
            PersonEvent::AuthenticationSucceeded {
                person_id: pid,
                method,
                timestamp: ts,
            } => {
                assert_eq!(pid, person_id);
                assert_eq!(method, AuthMethod::Password);
                assert_eq!(ts, timestamp);
            }
            _ => panic!("Expected AuthenticationSucceeded event"),
        }

        // Test authentication failure event
        let fail_event = PersonEvent::AuthenticationFailed {
            person_id,
            username: "testuser".to_string(),
            timestamp,
            failed_attempts: 3,
        };

        match fail_event {
            PersonEvent::AuthenticationFailed {
                failed_attempts, ..
            } => {
                assert_eq!(failed_attempts, 3);
            }
            _ => panic!("Expected AuthenticationFailed event"),
        }
    }

    /// Test for User Story I3: Profile Change Events
    #[test]
    fn test_profile_change_events() {
        // Test EmailRemoved event
        let person_id = PersonId::new();
        let old_email = Email::new("old@example.com".to_string()).unwrap();

        let email_removed_event = PersonEvent::EmailRemoved {
            person_id,
            old_email: old_email.clone(),
        };

        match email_removed_event {
            PersonEvent::EmailRemoved { old_email, .. } => {
                assert_eq!(old_email.as_str(), "old@example.com");
            }
            _ => panic!("Expected EmailRemoved event"),
        }

        // Test EmailAdded event
        let new_email = Email::new("new@example.com".to_string()).unwrap();

        let email_added_event = PersonEvent::EmailAdded {
            person_id,
            new_email: new_email.clone(),
        };

        match email_added_event {
            PersonEvent::EmailAdded { new_email, .. } => {
                assert_eq!(new_email.as_str(), "new@example.com");
            }
            _ => panic!("Expected EmailAdded event"),
        }

        // Test PhoneAdded event
        let phone = PhoneNumber {
            country_code: "+44".to_string(),
            number: "7700900123".to_string(),
        };

        let phone_event = PersonEvent::PhoneAdded {
            person_id,
            phone_number: phone.clone(),
        };

        match phone_event {
            PersonEvent::PhoneAdded { phone_number, .. } => {
                assert_eq!(phone_number.country_code, "+44");
                assert_eq!(phone_number.number, "7700900123");
            }
            _ => panic!("Expected PhoneAdded event"),
        }

        // Test AddressAdded event
        let address = Address {
            street: "123 Main St".to_string(),
            city: "Boston".to_string(),
            state: "MA".to_string(),
            postal_code: "02101".to_string(),
            country: "USA".to_string(),
        };

        let address_event = PersonEvent::AddressAdded {
            person_id,
            address: address.clone(),
        };

        match address_event {
            PersonEvent::AddressAdded { address: addr, .. } => {
                assert_eq!(addr.city, "Boston");
                assert_eq!(addr.state, "MA");
            }
            _ => panic!("Expected AddressAdded event"),
        }
    }

    /// Test for User Story I11: Trust Level Events
    #[test]
    fn test_trust_level_events() {
        // Given: Trust level change
        let person_id = PersonId::new();
        let old_level = TrustLevel::Unverified;
        let new_level = TrustLevel::EmailVerified;

        // When: Creating trust level change event
        let event = PersonEvent::TrustLevelChanged {
            person_id,
            old_level,
            new_level,
        };

        // Then: Event contains trust level data
        match event {
            PersonEvent::TrustLevelChanged {
                old_level: old,
                new_level: new,
                ..
            } => {
                assert_eq!(old, TrustLevel::Unverified);
                assert_eq!(new, TrustLevel::EmailVerified);
                assert!(new > old); // Trust level increased
            }
            _ => panic!("Expected TrustLevelChanged event"),
        }

        // Test MFA enabled event
        let mfa_event = PersonEvent::MfaEnabled {
            person_id,
            method: MfaMethod::Totp,
            timestamp: Utc::now(),
        };

        match mfa_event {
            PersonEvent::MfaEnabled { method, .. } => {
                assert_eq!(method, MfaMethod::Totp);
            }
            _ => panic!("Expected MfaEnabled event"),
        }
    }

    /// Test for organization membership events
    #[test]
    fn test_organization_membership_events() {
        // Given: Organization membership data
        let person_id = PersonId::new();
        let org_id = OrganizationId::new();

        // When: Creating joined organization event
        let joined_event = PersonEvent::JoinedOrganization {
            person_id,
            organization_id: org_id,
        };

        // Then: Event contains membership data
        match joined_event {
            PersonEvent::JoinedOrganization {
                person_id: pid,
                organization_id: oid,
            } => {
                assert_eq!(pid, person_id);
                assert_eq!(oid, org_id);
            }
            _ => panic!("Expected JoinedOrganization event"),
        }

        // Test left organization event
        let left_event = PersonEvent::LeftOrganization {
            person_id,
            organization_id: org_id,
        };

        match left_event {
            PersonEvent::LeftOrganization {
                person_id: pid,
                organization_id: oid,
            } => {
                assert_eq!(pid, person_id);
                assert_eq!(oid, org_id);
            }
            _ => panic!("Expected LeftOrganization event"),
        }
    }
}

#[cfg(test)]
mod organization_event_tests {
    use super::*;

    /// Test for OrganizationCreated event
    #[test]
    fn test_organization_created_event() {
        // Given: Organization creation data
        let org_id = OrganizationId::new();
        let name = "Tech Innovators Inc".to_string();
        let org_type = OrganizationType::Company;

        // When: Creating OrganizationCreated event
        let event = OrganizationEvent::OrganizationCreated {
            organization_id: org_id,
            name: name.clone(),
            org_type: org_type.clone(),
        };

        // Then: Event contains correct data
        match event {
            OrganizationEvent::OrganizationCreated {
                organization_id,
                name: n,
                org_type: t,
            } => {
                assert_eq!(organization_id, org_id);
                assert_eq!(n, "Tech Innovators Inc");
                assert!(matches!(t, OrganizationType::Company));
            }
            _ => panic!("Expected OrganizationCreated event"),
        }
    }

    /// Test for MemberAdded event
    #[test]
    fn test_member_added_event() {
        // Given: Member addition data
        let org_id = OrganizationId::new();
        let person_id = PersonId::new();

        // When: Creating MemberAdded event
        let event = OrganizationEvent::MemberAdded {
            organization_id: org_id,
            person_id,
        };

        // Then: Event contains member data
        match event {
            OrganizationEvent::MemberAdded {
                organization_id,
                person_id: pid,
            } => {
                assert_eq!(organization_id, org_id);
                assert_eq!(pid, person_id);
            }
            _ => panic!("Expected MemberAdded event"),
        }
    }

    /// Test for MemberRemoved event
    #[test]
    fn test_member_removed_event() {
        // Given: Member removal data
        let org_id = OrganizationId::new();
        let person_id = PersonId::new();

        // When: Creating MemberRemoved event
        let event = OrganizationEvent::MemberRemoved {
            organization_id: org_id,
            person_id,
        };

        // Then: Event contains removal data
        match event {
            OrganizationEvent::MemberRemoved {
                organization_id: oid,
                person_id: pid,
            } => {
                assert_eq!(oid, org_id);
                assert_eq!(pid, person_id);
            }
            _ => panic!("Expected MemberRemoved event"),
        }
    }

    /// Test for Organization hierarchy events
    #[test]
    fn test_organization_hierarchy_events() {
        // Given: Organization hierarchy data
        let org_id = OrganizationId::new();
        let parent_id = OrganizationId::new();
        let child_id = OrganizationId::new();

        // When: Creating parent changed event
        let parent_event = OrganizationEvent::ParentChanged {
            organization_id: org_id,
            old_parent_id: None,
            new_parent_id: Some(parent_id),
        };

        // Then: Event contains hierarchy data
        match parent_event {
            OrganizationEvent::ParentChanged {
                old_parent_id,
                new_parent_id,
                ..
            } => {
                assert!(old_parent_id.is_none());
                assert_eq!(new_parent_id, Some(parent_id));
            }
            _ => panic!("Expected ParentChanged event"),
        }

        // Test child added event
        let child_event = OrganizationEvent::ChildAdded {
            organization_id: org_id,
            child_id,
        };

        match child_event {
            OrganizationEvent::ChildAdded {
                organization_id: oid,
                child_id: cid,
            } => {
                assert_eq!(oid, org_id);
                assert_eq!(cid, child_id);
            }
            _ => panic!("Expected ChildAdded event"),
        }
    }
}
