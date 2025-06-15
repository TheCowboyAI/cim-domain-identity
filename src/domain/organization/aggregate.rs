//! Organization aggregate implementation

use serde::{Deserialize, Serialize};
use cim_domain::{AggregateRoot, EntityId};
use cim_component::Component;
use crate::domain::person::PersonId;
use crate::domain::value_objects::ApiKey;
use crate::IdentityResult;
use super::events::OrganizationEvent;
use super::commands::OrganizationCommand;
use uuid::Uuid;

/// Organization identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrganizationId(EntityId<OrganizationMarker>);

/// Marker type for Organization entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrganizationMarker;

impl OrganizationId {
    pub fn new() -> Self {
        OrganizationId(EntityId::new())
    }

    pub fn to_uuid(&self) -> Uuid {
        Uuid::from(self.0)
    }

    pub fn as_entity_id(&self) -> EntityId<OrganizationMarker> {
        self.0
    }
}

impl std::fmt::Display for OrganizationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Organization:{}", self.0)
    }
}

/// Organization type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrganizationType {
    Company,
    NonProfit,
    Government,
    Educational,
    Other,
}

/// Organization aggregate root
#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    // Entity fields
    id: OrganizationId,
    version: u64,

    // Organization-specific fields
    pub name: String,
    pub org_type: OrganizationType,
    pub description: Option<String>,
    pub parent_id: Option<OrganizationId>,
    pub child_ids: Vec<OrganizationId>,
    pub member_ids: Vec<PersonId>,
    pub admin_ids: Vec<PersonId>,

    // Authentication
    pub api_keys: Vec<ApiKey>,

    // Components for extensibility
    #[serde(skip)]
    components: Vec<Box<dyn Component>>,
}

impl Organization {
    /// Create a new Organization
    pub fn new(name: String, org_type: OrganizationType) -> Self {
        Organization {
            id: OrganizationId::new(),
            version: 0,
            name,
            org_type,
            description: None,
            parent_id: None,
            child_ids: Vec::new(),
            member_ids: Vec::new(),
            admin_ids: Vec::new(),
            api_keys: Vec::new(),
            components: Vec::new(),
        }
    }

    /// Handle commands
    pub fn handle_command(&mut self, command: OrganizationCommand) -> IdentityResult<Vec<OrganizationEvent>> {
        match command {
            OrganizationCommand::CreateOrganization { name, org_type } => {
                Ok(vec![OrganizationEvent::OrganizationCreated {
                    organization_id: self.id,
                    name,
                    org_type,
                }])
            }
            OrganizationCommand::UpdateName { new_name } => {
                let old_name = self.name.clone();
                self.name = new_name.clone();
                Ok(vec![OrganizationEvent::NameUpdated {
                    organization_id: self.id,
                    old_name,
                    new_name,
                }])
            }
            OrganizationCommand::UpdateDescription { description } => {
                self.description = Some(description.clone());
                Ok(vec![OrganizationEvent::DescriptionUpdated {
                    organization_id: self.id,
                    description,
                }])
            }
            OrganizationCommand::AddMember { person_id } => {
                if !self.member_ids.contains(&person_id) {
                    self.member_ids.push(person_id);
                    Ok(vec![OrganizationEvent::MemberAdded {
                        organization_id: self.id,
                        person_id,
                    }])
                } else {
                    Ok(vec![]) // Already a member
                }
            }
            OrganizationCommand::RemoveMember { person_id } => {
                if let Some(pos) = self.member_ids.iter().position(|id| id == &person_id) {
                    self.member_ids.remove(pos);
                    // Also remove from admins if present
                    self.admin_ids.retain(|id| id != &person_id);
                    Ok(vec![OrganizationEvent::MemberRemoved {
                        organization_id: self.id,
                        person_id,
                    }])
                } else {
                    Ok(vec![]) // Not a member
                }
            }
            OrganizationCommand::PromoteToAdmin { person_id } => {
                if self.member_ids.contains(&person_id) && !self.admin_ids.contains(&person_id) {
                    self.admin_ids.push(person_id);
                    Ok(vec![OrganizationEvent::MemberPromotedToAdmin {
                        organization_id: self.id,
                        person_id,
                    }])
                } else {
                    Ok(vec![]) // Not a member or already admin
                }
            }
            OrganizationCommand::DemoteFromAdmin { person_id } => {
                if let Some(pos) = self.admin_ids.iter().position(|id| id == &person_id) {
                    self.admin_ids.remove(pos);
                    Ok(vec![OrganizationEvent::AdminDemoted {
                        organization_id: self.id,
                        person_id,
                    }])
                } else {
                    Ok(vec![]) // Not an admin
                }
            }
            OrganizationCommand::SetParent { parent_id } => {
                let old_parent = self.parent_id;
                self.parent_id = parent_id;
                Ok(vec![OrganizationEvent::ParentChanged {
                    organization_id: self.id,
                    old_parent_id: old_parent,
                    new_parent_id: parent_id,
                }])
            }
            OrganizationCommand::AddChild { child_id } => {
                if !self.child_ids.contains(&child_id) {
                    self.child_ids.push(child_id);
                    Ok(vec![OrganizationEvent::ChildAdded {
                        organization_id: self.id,
                        child_id,
                    }])
                } else {
                    Ok(vec![]) // Already a child
                }
            }
            OrganizationCommand::RemoveChild { child_id } => {
                if let Some(pos) = self.child_ids.iter().position(|id| id == &child_id) {
                    self.child_ids.remove(pos);
                    Ok(vec![OrganizationEvent::ChildRemoved {
                        organization_id: self.id,
                        child_id,
                    }])
                } else {
                    Ok(vec![]) // Not a child
                }
            }
        }
    }

    /// Apply events to update state
    pub fn apply_event(&mut self, event: &OrganizationEvent) {
        match event {
            OrganizationEvent::OrganizationCreated { .. } => {
                // Initial state already set in constructor
                self.increment_version();
            }
            OrganizationEvent::NameUpdated { new_name, .. } => {
                self.name = new_name.clone();
                self.increment_version();
            }
            OrganizationEvent::DescriptionUpdated { description, .. } => {
                self.description = Some(description.clone());
                self.increment_version();
            }
            OrganizationEvent::MemberAdded { person_id, .. } => {
                if !self.member_ids.contains(person_id) {
                    self.member_ids.push(*person_id);
                }
                self.increment_version();
            }
            OrganizationEvent::MemberRemoved { person_id, .. } => {
                self.member_ids.retain(|id| id != person_id);
                self.admin_ids.retain(|id| id != person_id);
                self.increment_version();
            }
            OrganizationEvent::MemberPromotedToAdmin { person_id, .. } => {
                if !self.admin_ids.contains(person_id) {
                    self.admin_ids.push(*person_id);
                }
                self.increment_version();
            }
            OrganizationEvent::AdminDemoted { person_id, .. } => {
                self.admin_ids.retain(|id| id != person_id);
                self.increment_version();
            }
            OrganizationEvent::ParentChanged { new_parent_id, .. } => {
                self.parent_id = *new_parent_id;
                self.increment_version();
            }
            OrganizationEvent::ChildAdded { child_id, .. } => {
                if !self.child_ids.contains(child_id) {
                    self.child_ids.push(*child_id);
                }
                self.increment_version();
            }
            OrganizationEvent::ChildRemoved { child_id, .. } => {
                self.child_ids.retain(|id| id != child_id);
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

impl Clone for Organization {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            version: self.version,
            name: self.name.clone(),
            org_type: self.org_type.clone(),
            description: self.description.clone(),
            parent_id: self.parent_id.clone(),
            child_ids: self.child_ids.clone(),
            member_ids: self.member_ids.clone(),
            admin_ids: self.admin_ids.clone(),
            api_keys: self.api_keys.clone(),
            components: Vec::new(), // Don't clone components as they're not cloneable
        }
    }
}

// Entity trait is not needed - Organization is already an aggregate root

impl AggregateRoot for Organization {
    type Id = OrganizationId;

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
