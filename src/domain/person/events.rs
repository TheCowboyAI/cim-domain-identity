//! Events for the Person aggregate

use serde::{Deserialize, Serialize};
use crate::domain::value_objects::{Email, Name, Address, PhoneNumber, TrustLevel};
use crate::domain::organization::OrganizationId;
use super::PersonId;

/// Events that can be emitted by a Person aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PersonEvent {
    /// Person was registered
    PersonRegistered {
        person_id: PersonId,
        name: Name,
        email: Email,
    },

    /// Person's email was updated
    EmailUpdated {
        person_id: PersonId,
        old_email: Email,
        new_email: Email,
    },

    /// Person's phone was updated
    PhoneUpdated {
        person_id: PersonId,
        phone_number: PhoneNumber,
    },

    /// Person's address was updated
    AddressUpdated {
        person_id: PersonId,
        address: Address,
    },

    /// Person's trust level changed
    TrustLevelChanged {
        person_id: PersonId,
        old_level: TrustLevel,
        new_level: TrustLevel,
    },

    /// Person joined an organization
    JoinedOrganization {
        person_id: PersonId,
        organization_id: OrganizationId,
    },

    /// Person left an organization
    LeftOrganization {
        person_id: PersonId,
        organization_id: OrganizationId,
    },
}
