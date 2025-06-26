//! Core identity components

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use super::IdentityId;

/// Core identity entity component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityEntity {
    pub identity_id: IdentityId,
    pub identity_type: IdentityType,
    pub status: IdentityStatus,
}

/// Type of identity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityType {
    /// A natural person
    Person,
    /// An organization or company
    Organization,
    /// A system or service account
    System,
    /// An external identity from another system
    External { provider: &'static str },
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
    Merged { merged_into: IdentityId },
}

/// Identity verification level component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerification {
    pub verification_level: VerificationLevel,
    pub verified_at: Option<chrono::DateTime<chrono::Utc>>,
    pub verified_by: Option<IdentityId>,
    pub verification_method: Option<VerificationMethod>,
}

/// Level of identity verification
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum VerificationLevel {
    /// No verification performed
    Unverified = 0,
    /// Email verified
    EmailVerified = 1,
    /// Phone verified
    PhoneVerified = 2,
    /// Document verified (ID, passport, etc.)
    DocumentVerified = 3,
    /// In-person verification
    InPersonVerified = 4,
    /// Fully verified through multiple methods
    FullyVerified = 5,
}

/// Method used for verification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationMethod {
    EmailLink,
    EmailCode,
    SmsCode,
    PhoneCall,
    DocumentUpload,
    VideoCall,
    InPerson,
    ThirdPartyService { service: String },
}

/// External identity reference component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ExternalIdentity {
    pub provider: String,
    pub external_id: String,
    pub profile_data: serde_json::Value,
    pub linked_at: chrono::DateTime<chrono::Utc>,
}

/// Identity claim component for attributes
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityClaim {
    pub claim_type: ClaimType,
    pub value: String,
    pub verified: bool,
    pub issuer: Option<IdentityId>,
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Type of identity claim
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClaimType {
    Email,
    Phone,
    Name,
    DateOfBirth,
    Address,
    GovernmentId,
    Custom(String),
} 