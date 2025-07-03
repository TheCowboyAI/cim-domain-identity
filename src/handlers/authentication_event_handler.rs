//! Authentication event handler for the Identity domain
//!
//! Handles authentication-related events from the Policy domain and
//! performs identity verification operations.

use crate::domain::person::{Person, PersonId};
use crate::domain::organization::{Organization, OrganizationId};
use cim_domain::{
    DomainError, DomainResult, DomainEvent,
    AggregateRepository,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identity reference for authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentityRef {
    Person(PersonId),
    Organization(OrganizationId),
}

/// Authentication requested event from Policy domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationRequested {
    pub request_id: Uuid,
    pub identity_ref: Option<IdentityRef>,
    pub location: LocationContext,
    pub available_factors: Vec<String>,
    pub requested_at: chrono::DateTime<chrono::Utc>,
}

/// Location context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationContext {
    pub ip_address: Option<String>,
    pub coordinates: Option<(f64, f64)>,
    pub country: Option<String>,
    pub network_type: Option<String>,
    pub device_id: Option<String>,
}

/// Identity verification requested event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerificationRequested {
    pub request_id: Uuid,
    pub identity_ref: IdentityRef,
    pub verification_type: String,
    pub requested_at: chrono::DateTime<chrono::Utc>,
}

impl DomainEvent for IdentityVerificationRequested {
    fn subject(&self) -> String {
        match &self.identity_ref {
            IdentityRef::Person(id) => format!("identity.person.{id.to_uuid(}.verification.requested")),
            IdentityRef::Organization(id) => format!("identity.organization.{id.to_uuid(}.verification.requested")),
        }
    }

    fn aggregate_id(&self) -> Uuid {
        match &self.identity_ref {
            IdentityRef::Person(id) => id.to_uuid(),
            IdentityRef::Organization(id) => id.to_uuid(),
        }
    }

    fn event_type(&self) -> &'static str {
        "IdentityVerificationRequested"
    }
}

/// Identity verified event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerified {
    pub request_id: Uuid,
    pub identity_ref: IdentityRef,
    pub verification_level: IdentityVerificationLevel,
    pub attributes_verified: Vec<String>,
    pub verified_at: chrono::DateTime<chrono::Utc>,
}

impl DomainEvent for IdentityVerified {
    fn subject(&self) -> String {
        match &self.identity_ref {
            IdentityRef::Person(id) => format!("identity.person.{id.to_uuid(}.verified")),
            IdentityRef::Organization(id) => format!("identity.organization.{id.to_uuid(}.verified")),
        }
    }

    fn aggregate_id(&self) -> Uuid {
        match &self.identity_ref {
            IdentityRef::Person(id) => id.to_uuid(),
            IdentityRef::Organization(id) => id.to_uuid(),
        }
    }

    fn event_type(&self) -> &'static str {
        "IdentityVerified"
    }
}

/// Identity verification level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IdentityVerificationLevel {
    None,
    Email,
    Phone,
    Document,
    Biometric,
    InPerson,
}

/// Authentication event handler for Identity domain
pub struct AuthenticationEventHandler<P, O>
where
    P: AggregateRepository<Person>,
    O: AggregateRepository<Organization>,
{
    person_repository: P,
    organization_repository: O,
}

impl<P, O> AuthenticationEventHandler<P, O>
where
    P: AggregateRepository<Person>,
    O: AggregateRepository<Organization>,
{
    /// Create a new authentication event handler
    pub fn new(person_repository: P, organization_repository: O) -> Self {
        Self {
            person_repository,
            organization_repository,
        }
    }

    /// Handle authentication requested event
    pub async fn handle_authentication_requested(
        &self,
        event: AuthenticationRequested,
    ) -> DomainResult<Vec<Box<dyn cim_domain::DomainEvent>>> {
        let mut events = Vec::new();

        // If identity reference is provided, verify it exists
        if let Some(identity_ref) = &event.identity_ref {
            match identity_ref {
                IdentityRef::Person(person_id) => {
                    // Load person to verify they exist
                    let person = self.person_repository
                        .load(*person_id)
                        .map_err(DomainError::InternalError)?;

                    // Check if person exists
                    if person.is_none() {
                        return Err(DomainError::EntityNotFound {
                            entity_type: "Person".to_string(),
                            id: person_id.to_uuid().to_string(),
                        });
                    }

                    // Check if person is active
                    if !self.is_person_active(&person.unwrap()) {
                        return Err(DomainError::ValidationError(
                            "Person is not active".to_string()
                        ));
                    }

                    // Create identity verification requested event
                    events.push(Box::new(IdentityVerificationRequested {
                        request_id: event.request_id,
                        identity_ref: identity_ref.clone(),
                        verification_type: "authentication".to_string(),
                        requested_at: chrono::Utc::now(),
                    }) as Box<dyn cim_domain::DomainEvent>);
                }
                IdentityRef::Organization(org_id) => {
                    // Load organization to verify it exists
                    let org = self.organization_repository
                        .load(*org_id)
                        .map_err(DomainError::InternalError)?;

                    // Check if organization exists
                    if org.is_none() {
                        return Err(DomainError::EntityNotFound {
                            entity_type: "Organization".to_string(),
                            id: org_id.to_uuid().to_string(),
                        });
                    }

                    // Check if organization is active
                    if !self.is_organization_active(&org.unwrap()) {
                        return Err(DomainError::ValidationError(
                            "Organization is not active".to_string()
                        ));
                    }

                    // Create identity verification requested event
                    events.push(Box::new(IdentityVerificationRequested {
                        request_id: event.request_id,
                        identity_ref: identity_ref.clone(),
                        verification_type: "authentication".to_string(),
                        requested_at: chrono::Utc::now(),
                    }) as Box<dyn cim_domain::DomainEvent>);
                }
            }
        }

        Ok(events)
    }

    /// Handle identity verification requested event
    pub async fn handle_identity_verification_requested(
        &self,
        event: IdentityVerificationRequested,
    ) -> DomainResult<Vec<Box<dyn cim_domain::DomainEvent>>> {
        let mut events = Vec::new();

        match &event.identity_ref {
            IdentityRef::Person(person_id) => {
                // Load person
                let person = self.person_repository
                    .load(*person_id)
                    .map_err(DomainError::InternalError)?;

                if let Some(person) = person {
                    // Perform verification based on available attributes
                    let (verification_level, attributes_verified) =
                        self.verify_person_identity(&person).await?;

                    // Create identity verified event
                    events.push(Box::new(IdentityVerified {
                        request_id: event.request_id,
                        identity_ref: event.identity_ref.clone(),
                        verification_level,
                        attributes_verified,
                        verified_at: chrono::Utc::now(),
                    }) as Box<dyn cim_domain::DomainEvent>);
                } else {
                    return Err(DomainError::EntityNotFound {
                        entity_type: "Person".to_string(),
                        id: person_id.to_uuid().to_string(),
                    });
                }
            }
            IdentityRef::Organization(org_id) => {
                // Load organization
                let org = self.organization_repository
                    .load(*org_id)
                    .map_err(DomainError::InternalError)?;

                if let Some(org) = org {
                    // Perform verification based on available attributes
                    let (verification_level, attributes_verified) =
                        self.verify_organization_identity(&org).await?;

                    // Create identity verified event
                    events.push(Box::new(IdentityVerified {
                        request_id: event.request_id,
                        identity_ref: event.identity_ref.clone(),
                        verification_level,
                        attributes_verified,
                        verified_at: chrono::Utc::now(),
                    }) as Box<dyn cim_domain::DomainEvent>);
                } else {
                    return Err(DomainError::EntityNotFound {
                        entity_type: "Organization".to_string(),
                        id: org_id.to_uuid().to_string(),
                    });
                }
            }
        }

        Ok(events)
    }

    /// Check if person is active
    fn is_person_active(&self, _person: &Person) -> bool {
        // In a real implementation, this would check person status
        // For now, we'll assume all persons are active
        true
    }

    /// Check if organization is active
    fn is_organization_active(&self, _org: &Organization) -> bool {
        // In a real implementation, this would check organization status
        // For now, we'll assume all organizations are active
        true
    }

    /// Verify person identity
    async fn verify_person_identity(
        &self,
        _person: &Person,
    ) -> DomainResult<(IdentityVerificationLevel, Vec<String>)> {
        let mut attributes_verified = Vec::new();
        

        // Check email verification
        // In real implementation, would check if email is verified
        attributes_verified.push("email".to_string());
        let verification_level = IdentityVerificationLevel::Email;

        // Check phone verification
        // In real implementation, would check if phone is verified
        if true { // Placeholder for phone verification check
            attributes_verified.push("phone".to_string());
            let _verification_level = IdentityVerificationLevel::Phone;
        }

        // Check document verification
        // In real implementation, would check if documents are verified
        if false { // Placeholder for document verification check
            attributes_verified.push("government_id".to_string());
            let _verification_level = IdentityVerificationLevel::Document;
        }

        Ok((verification_level, attributes_verified))
    }

    /// Verify organization identity
    async fn verify_organization_identity(
        &self,
        _org: &Organization,
    ) -> DomainResult<(IdentityVerificationLevel, Vec<String>)> {
        let mut attributes_verified = Vec::new();
        

        // Check business registration
        attributes_verified.push("business_registration".to_string());
        let verification_level = IdentityVerificationLevel::Document;

        // Check authorized representatives
        attributes_verified.push("authorized_representatives".to_string());

        Ok((verification_level, attributes_verified))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cim_domain::InMemoryRepository;

    #[tokio::test]
    async fn test_handle_authentication_requested() {
        let person_repo = InMemoryRepository::<Person>::new();
        let org_repo = InMemoryRepository::<Organization>::new();
        let handler = AuthenticationEventHandler::new(person_repo, org_repo);

        let event = AuthenticationRequested {
            request_id: Uuid::new_v4(),
            identity_ref: None,
            location: LocationContext {
                ip_address: Some("192.168.1.1".to_string()),
                coordinates: None,
                country: Some("US".to_string()),
                network_type: None,
                device_id: None,
            },
            available_factors: vec!["password".to_string()],
            requested_at: chrono::Utc::now(),
        };

        let events = handler.handle_authentication_requested(event).await.unwrap();
        assert_eq!(events.len(), 0); // No identity ref, so no events
    }
}
