//! Commands for the Person aggregate

use serde::{Deserialize, Serialize};
use crate::domain::value_objects::{Email, Name, Address, PhoneNumber, TrustLevel};
use crate::domain::organization::OrganizationId;

/// Commands that can be sent to a Person aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PersonCommand {
    /// Register a new person
    RegisterPerson {
        name: Name,
        email: Email,
    },

    /// Update person's email
    UpdateEmail {
        new_email: Email,
    },

    /// Update person's phone number
    UpdatePhone {
        phone_number: PhoneNumber,
    },

    /// Update person's address
    UpdateAddress {
        address: Address,
    },

    /// Update person's trust level
    UpdateTrustLevel {
        trust_level: TrustLevel,
    },

    /// Join an organization
    JoinOrganization {
        organization_id: OrganizationId,
    },

    /// Leave an organization
    LeaveOrganization {
        organization_id: OrganizationId,
    },
}
