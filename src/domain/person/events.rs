//! Events for the Person aggregate

use serde::{Deserialize, Serialize};
use crate::domain::value_objects::{Email, Name, Address, PhoneNumber, TrustLevel, AuthMethod, MfaMethod};
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

    /// Credentials were set
    CredentialsSet {
        person_id: PersonId,
        username: String,
    },

    /// Authentication succeeded
    AuthenticationSucceeded {
        person_id: PersonId,
        method: AuthMethod,
        timestamp: chrono::DateTime<chrono::Utc>,
    },

    /// Authentication failed
    AuthenticationFailed {
        person_id: PersonId,
        username: String,
        timestamp: chrono::DateTime<chrono::Utc>,
        failed_attempts: u32,
    },

    /// Account was locked
    AccountLocked {
        person_id: PersonId,
        locked_until: chrono::DateTime<chrono::Utc>,
        reason: String,
    },

    /// Account was unlocked
    AccountUnlocked {
        person_id: PersonId,
        timestamp: chrono::DateTime<chrono::Utc>,
    },

    /// MFA was enabled
    MfaEnabled {
        person_id: PersonId,
        method: MfaMethod,
        timestamp: chrono::DateTime<chrono::Utc>,
    },

    /// MFA was disabled
    MfaDisabled {
        person_id: PersonId,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
}
