//! Commands for the Organization aggregate

use serde::{Deserialize, Serialize};
use crate::domain::person::PersonId;
use super::{OrganizationId, OrganizationType};

/// Commands that can be sent to an Organization aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrganizationCommand {
    /// Create a new organization
    CreateOrganization {
        name: String,
        org_type: OrganizationType,
    },

    /// Change organization name
    ChangeName {
        new_name: String,
    },

    /// Change organization description
    ChangeDescription {
        description: String,
    },

    /// Add a member to the organization
    AddMember {
        person_id: PersonId,
    },

    /// Remove a member from the organization
    RemoveMember {
        person_id: PersonId,
    },

    /// Promote a member to admin
    PromoteToAdmin {
        person_id: PersonId,
    },

    /// Demote an admin to regular member
    DemoteFromAdmin {
        person_id: PersonId,
    },

    /// Set parent organization
    SetParent {
        parent_id: Option<OrganizationId>,
    },

    /// Add a child organization
    AddChild {
        child_id: OrganizationId,
    },

    /// Remove a child organization
    RemoveChild {
        child_id: OrganizationId,
    },
}
