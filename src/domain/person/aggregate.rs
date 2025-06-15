//! Person aggregate implementation

use serde::{Deserialize, Serialize};
use cim_domain::{AggregateRoot, EntityId};
use cim_component::Component;
use crate::domain::value_objects::{Email, Name, Address, PhoneNumber, TrustLevel, Credentials, AuthStatus, MfaSettings};
use crate::IdentityResult;
use super::events::PersonEvent;
use super::commands::PersonCommand;
use uuid::Uuid;

/// Person identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PersonId(EntityId<PersonMarker>);

/// Marker type for Person entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PersonMarker;

impl PersonId {
    pub fn new() -> Self {
        PersonId(EntityId::new())
    }

    pub fn to_uuid(&self) -> Uuid {
        Uuid::from(self.0)
    }

    pub fn as_entity_id(&self) -> EntityId<PersonMarker> {
        self.0
    }
}

impl std::fmt::Display for PersonId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Person:{}", self.0)
    }
}

/// Person aggregate representing a human actor in the system
#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    // Entity fields
    id: PersonId,
    version: u64,

    // Person-specific fields
    pub name: Name,
    pub email: Email,
    pub phone: Option<PhoneNumber>,
    pub address: Option<Address>,
    pub trust_level: TrustLevel,
    pub organization_ids: Vec<crate::domain::organization::OrganizationId>,

    // Authentication fields
    pub credentials: Option<Credentials>,
    pub auth_status: AuthStatus,
    pub mfa_settings: MfaSettings,

    // Components for extensibility
    #[serde(skip)]
    components: Vec<Box<dyn Component>>,
}

impl Person {
    /// Create a new Person
    pub fn new(name: Name, email: Email) -> Self {
        Person {
            id: PersonId::new(),
            version: 0,
            name,
            email,
            phone: None,
            address: None,
            trust_level: TrustLevel::default(),
            organization_ids: Vec::new(),
            credentials: None,
            auth_status: AuthStatus::default(),
            mfa_settings: MfaSettings::default(),
            components: Vec::new(),
        }
    }

    /// Handle commands
    pub fn handle_command(&mut self, command: PersonCommand) -> IdentityResult<Vec<PersonEvent>> {
        match command {
            PersonCommand::RegisterPerson { name, email } => {
                // This would typically be handled at the repository level
                // to check for duplicates
                Ok(vec![PersonEvent::PersonRegistered {
                    person_id: self.id,
                    name,
                    email,
                }])
            }
            PersonCommand::UpdateEmail { new_email } => {
                let old_email = self.email.clone();
                self.email = new_email.clone();
                Ok(vec![PersonEvent::EmailUpdated {
                    person_id: self.id,
                    old_email,
                    new_email,
                }])
            }
            PersonCommand::UpdatePhone { phone_number } => {
                self.phone = Some(phone_number.clone());
                Ok(vec![PersonEvent::PhoneUpdated {
                    person_id: self.id,
                    phone_number,
                }])
            }
            PersonCommand::UpdateAddress { address } => {
                self.address = Some(address.clone());
                Ok(vec![PersonEvent::AddressUpdated {
                    person_id: self.id,
                    address,
                }])
            }
            PersonCommand::UpdateTrustLevel { trust_level } => {
                let old_level = self.trust_level;
                self.trust_level = trust_level;
                Ok(vec![PersonEvent::TrustLevelChanged {
                    person_id: self.id,
                    old_level,
                    new_level: trust_level,
                }])
            }
            PersonCommand::JoinOrganization { organization_id } => {
                if !self.organization_ids.contains(&organization_id) {
                    self.organization_ids.push(organization_id);
                    Ok(vec![PersonEvent::JoinedOrganization {
                        person_id: self.id,
                        organization_id,
                    }])
                } else {
                    Ok(vec![]) // Already a member
                }
            }
            PersonCommand::LeaveOrganization { organization_id } => {
                if let Some(pos) = self.organization_ids.iter().position(|id| id == &organization_id) {
                    self.organization_ids.remove(pos);
                    Ok(vec![PersonEvent::LeftOrganization {
                        person_id: self.id,
                        organization_id,
                    }])
                } else {
                    Ok(vec![]) // Not a member
                }
            }
            PersonCommand::SetCredentials { credentials } => {
                self.credentials = Some(credentials.clone());
                Ok(vec![PersonEvent::CredentialsSet {
                    person_id: self.id,
                    username: credentials.username,
                }])
            }
            PersonCommand::Authenticate { username, password_hash } => {
                if let Some(creds) = &self.credentials {
                    if creds.username == username && creds.password_hash == password_hash {
                        self.auth_status.is_authenticated = true;
                        self.auth_status.method = Some(crate::domain::value_objects::AuthMethod::Password);
                        self.auth_status.last_login = Some(chrono::Utc::now());
                        self.auth_status.failed_attempts = 0;
                        Ok(vec![PersonEvent::AuthenticationSucceeded {
                            person_id: self.id,
                            method: crate::domain::value_objects::AuthMethod::Password,
                            timestamp: chrono::Utc::now(),
                        }])
                    } else {
                        self.auth_status.failed_attempts += 1;
                        Ok(vec![PersonEvent::AuthenticationFailed {
                            person_id: self.id,
                            username,
                            timestamp: chrono::Utc::now(),
                            failed_attempts: self.auth_status.failed_attempts,
                        }])
                    }
                } else {
                    Ok(vec![]) // No credentials set
                }
            }
            PersonCommand::RecordFailedAuth { username } => {
                self.auth_status.failed_attempts += 1;
                Ok(vec![PersonEvent::AuthenticationFailed {
                    person_id: self.id,
                    username,
                    timestamp: chrono::Utc::now(),
                    failed_attempts: self.auth_status.failed_attempts,
                }])
            }
            PersonCommand::LockAccount { until } => {
                self.auth_status.locked_until = Some(until);
                self.auth_status.is_authenticated = false;
                Ok(vec![PersonEvent::AccountLocked {
                    person_id: self.id,
                    locked_until: until,
                    reason: "Too many failed authentication attempts".to_string(),
                }])
            }
            PersonCommand::UnlockAccount => {
                self.auth_status.locked_until = None;
                self.auth_status.failed_attempts = 0;
                Ok(vec![PersonEvent::AccountUnlocked {
                    person_id: self.id,
                    timestamp: chrono::Utc::now(),
                }])
            }
            PersonCommand::EnableMfa { method, backup_codes } => {
                self.mfa_settings.enabled = true;
                self.mfa_settings.method = method;
                self.mfa_settings.backup_codes = backup_codes;
                Ok(vec![PersonEvent::MfaEnabled {
                    person_id: self.id,
                    method,
                    timestamp: chrono::Utc::now(),
                }])
            }
            PersonCommand::DisableMfa => {
                self.mfa_settings.enabled = false;
                self.mfa_settings.backup_codes.clear();
                Ok(vec![PersonEvent::MfaDisabled {
                    person_id: self.id,
                    timestamp: chrono::Utc::now(),
                }])
            }
            PersonCommand::UpdateLastLogin { timestamp } => {
                self.auth_status.last_login = Some(timestamp);
                Ok(vec![]) // No event for this, it's internal
            }
        }
    }

    /// Apply events to update state
    pub fn apply_event(&mut self, event: &PersonEvent) {
        match event {
            PersonEvent::PersonRegistered { .. } => {
                // Initial state already set in constructor
                self.increment_version();
            }
            PersonEvent::EmailUpdated { new_email, .. } => {
                self.email = new_email.clone();
                self.increment_version();
            }
            PersonEvent::PhoneUpdated { phone_number, .. } => {
                self.phone = Some(phone_number.clone());
                self.increment_version();
            }
            PersonEvent::AddressUpdated { address, .. } => {
                self.address = Some(address.clone());
                self.increment_version();
            }
            PersonEvent::TrustLevelChanged { new_level, .. } => {
                self.trust_level = *new_level;
                self.increment_version();
            }
            PersonEvent::JoinedOrganization { organization_id, .. } => {
                if !self.organization_ids.contains(organization_id) {
                    self.organization_ids.push(*organization_id);
                }
                self.increment_version();
            }
            PersonEvent::LeftOrganization { organization_id, .. } => {
                self.organization_ids.retain(|id| id != organization_id);
                self.increment_version();
            }
            PersonEvent::CredentialsSet { .. } => {
                // Credentials already set in command handler
                self.increment_version();
            }
            PersonEvent::AuthenticationSucceeded { method, timestamp, .. } => {
                self.auth_status.is_authenticated = true;
                self.auth_status.method = Some(*method);
                self.auth_status.last_login = Some(*timestamp);
                self.auth_status.failed_attempts = 0;
                self.increment_version();
            }
            PersonEvent::AuthenticationFailed { failed_attempts, .. } => {
                self.auth_status.failed_attempts = *failed_attempts;
                self.increment_version();
            }
            PersonEvent::AccountLocked { locked_until, .. } => {
                self.auth_status.locked_until = Some(*locked_until);
                self.auth_status.is_authenticated = false;
                self.increment_version();
            }
            PersonEvent::AccountUnlocked { .. } => {
                self.auth_status.locked_until = None;
                self.auth_status.failed_attempts = 0;
                self.increment_version();
            }
            PersonEvent::MfaEnabled { method, .. } => {
                self.mfa_settings.enabled = true;
                self.mfa_settings.method = *method;
                self.increment_version();
            }
            PersonEvent::MfaDisabled { .. } => {
                self.mfa_settings.enabled = false;
                self.mfa_settings.backup_codes.clear();
                self.increment_version();
            }
        }
    }

    /// Add a component
    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }

    /// Get components
    pub fn components(&self) -> &[Box<dyn Component>] {
        &self.components
    }
}

impl Clone for Person {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            address: self.address.clone(),
            trust_level: self.trust_level,
            organization_ids: self.organization_ids.clone(),
            credentials: self.credentials.clone(),
            auth_status: self.auth_status.clone(),
            mfa_settings: self.mfa_settings.clone(),
            components: Vec::new(), // Don't clone components as they're not cloneable
            version: self.version,
        }
    }
}

// Entity trait is not needed - Person is already an aggregate root

impl AggregateRoot for Person {
    type Id = PersonId;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn version(&self) -> u64 {
        self.version
    }

    fn increment_version(&mut self) {
        self.version += 1;
    }
}
