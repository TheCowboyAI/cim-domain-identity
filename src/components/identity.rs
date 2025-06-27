//! Identity components for the Identity domain

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Core identity component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityEntity {
    pub identity_id: Uuid,
    pub identity_type: IdentityType,
    pub status: IdentityStatus,
}

/// Type of identity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IdentityType {
    /// A natural person
    Person,
    /// An organization or company
    Organization,
    /// A system or service account
    System,
    /// An external identity from another system
    External,
}

/// Status of an identity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityStatus {
    /// Identity is being created/verified
    Pending,
    /// Identity is active and verified
    Active,
    /// Identity is temporarily suspended
    Suspended,
    /// Identity has been archived
    Archived,
    /// Identity has been merged with another
    Merged { merged_into: Uuid },
}

/// Metadata for an identity
#[derive(Component, Debug, Clone, Default, Serialize, Deserialize)]
pub struct IdentityMetadata {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_by: Option<Uuid>,
    pub version: u64,
    pub tags: Vec<String>,
    pub properties: serde_json::Value,
    pub custom_attributes: std::collections::HashMap<String, serde_json::Value>,
}

/// Identity verification component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerification {
    pub verification_level: VerificationLevel,
    pub verified_at: Option<chrono::DateTime<chrono::Utc>>,
    pub verified_by: Option<Uuid>,
    pub verification_method: Option<VerificationMethod>,
}

/// Verification levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VerificationLevel {
    /// No verification performed
    Unverified = 0,
    /// Basic verification
    Basic = 1,
    /// Enhanced verification
    Enhanced = 2,
    /// Full verification
    Full = 3,
}

/// Verification methods
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationMethod {
    Email,
    Phone,
    Document,
    Biometric,
    InPerson,
    ThirdParty { provider: String },
}

/// Claims about an identity
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityClaim {
    pub claim_type: ClaimType,
    pub value: String,
    pub verified: bool,
    pub issuer: Option<Uuid>,
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Types of claims
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClaimType {
    Email,
    Phone,
    Name,
    DateOfBirth,
    Address,
    NationalId,
    TaxId,
    Custom(String),
}

/// External identity reference component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ExternalIdentity {
    pub provider: String,
    pub external_id: String,
    pub profile_data: serde_json::Value,
    pub linked_at: chrono::DateTime<chrono::Utc>,
} 