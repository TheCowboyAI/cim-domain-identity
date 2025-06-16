//! Events for the Organization aggregate

use serde::{Deserialize, Serialize};
use crate::domain::person::PersonId;
use super::{OrganizationId, OrganizationType};

/// Events that can be emitted by an Organization aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrganizationEvent {
    /// Organization was created
    OrganizationCreated {
        organization_id: OrganizationId,
        name: String,
        org_type: OrganizationType,
    },

    /// Organization name was removed
    NameRemoved {
        organization_id: OrganizationId,
        old_name: String,
    },

    /// Organization name was changed
    NameChanged {
        organization_id: OrganizationId,
        new_name: String,
    },

    /// Organization description was removed
    DescriptionRemoved {
        organization_id: OrganizationId,
        old_description: Option<String>,
    },

    /// Organization description was set
    DescriptionSet {
        organization_id: OrganizationId,
        description: String,
    },

    /// Member was added to organization
    MemberAdded {
        organization_id: OrganizationId,
        person_id: PersonId,
    },

    /// Member was removed from organization
    MemberRemoved {
        organization_id: OrganizationId,
        person_id: PersonId,
    },

    /// Member was promoted to admin
    MemberPromotedToAdmin {
        organization_id: OrganizationId,
        person_id: PersonId,
    },

    /// Admin was demoted to regular member
    AdminDemoted {
        organization_id: OrganizationId,
        person_id: PersonId,
    },

    /// Parent organization changed
    ParentChanged {
        organization_id: OrganizationId,
        old_parent_id: Option<OrganizationId>,
        new_parent_id: Option<OrganizationId>,
    },

    /// Child organization added
    ChildAdded {
        organization_id: OrganizationId,
        child_id: OrganizationId,
    },

    /// Child organization removed
    ChildRemoved {
        organization_id: OrganizationId,
        child_id: OrganizationId,
    },
}
