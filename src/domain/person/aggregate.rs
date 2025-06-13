//! Person aggregate implementation

use serde::{Deserialize, Serialize};
use cim_core_domain::{AggregateRoot, EntityId};
use cim_component::Component;
use crate::domain::value_objects::{Email, Name, Address, PhoneNumber, TrustLevel};
use crate::IdentityResult;
use super::events::PersonEvent;
use super::commands::PersonCommand;

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
}

impl std::fmt::Display for PersonId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Person:{}", self.0)
    }
}

/// Person aggregate root
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
