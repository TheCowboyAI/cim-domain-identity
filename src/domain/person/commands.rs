//! Commands for the Person aggregate

use serde::{Deserialize, Serialize};
use crate::domain::value_objects::{Email, Name, Address, PhoneNumber, TrustLevel, Credentials, MfaMethod};
use crate::domain::organization::OrganizationId;

/// Commands that can be sent to a Person aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PersonCommand {
    /// Register a new person
    RegisterPerson {
        name: Name,
        email: Email,
    },

    /// Change person's email address
    ChangeEmail {
        new_email: Email,
    },

    /// Change person's phone number
    ChangePhone {
        phone_number: PhoneNumber,
    },

    /// Change person's address
    ChangeAddress {
        address: Address,
    },

    /// Change person's trust level
    ChangeTrustLevel {
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

    /// Set authentication credentials
    SetCredentials {
        credentials: Credentials,
    },

    /// Authenticate the person
    Authenticate {
        username: String,
        password_hash: String,
    },

    /// Record failed authentication attempt
    RecordFailedAuth {
        username: String,
    },

    /// Lock account after too many failed attempts
    LockAccount {
        until: chrono::DateTime<chrono::Utc>,
    },

    /// Unlock account
    UnlockAccount,

    /// Enable MFA
    EnableMfa {
        method: MfaMethod,
        backup_codes: Vec<String>,
    },

    /// Disable MFA
    DisableMfa,

    /// Record login event
    RecordLogin {
        timestamp: chrono::DateTime<chrono::Utc>,
    },
}
