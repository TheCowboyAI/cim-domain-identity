//! Value objects for the Identity domain
//!
//! Note: The Identity domain has been refactored to use ECS architecture,
//! so most data is now represented as Components rather than traditional
//! value objects. This module is kept for compatibility and may contain
//! shared value types used across the domain.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Verification method for identity verification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerificationMethod {
    Email,
    Phone,
    Document,
    Biometric,
    Social,
    Reference,
}

/// Level of identity verification
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VerificationLevel {
    None = 0,
    Basic = 1,
    Enhanced = 2,
    Full = 3,
}

/// Type of relationship between identities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationshipType {
    EmployeeOf,
    MemberOf,
    PartnerOf,
    CustomerOf,
    SupplierOf,
    ParentOf,
    ChildOf,
    Custom(String),
}

/// Claim about an identity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentityClaim {
    pub claim_type: String,
    pub value: String,
    pub verified: bool,
    pub issuer: Option<String>,
}

impl fmt::Display for VerificationMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Email => write!(f, "Email"),
            Self::Phone => write!(f, "Phone"),
            Self::Document => write!(f, "Document"),
            Self::Biometric => write!(f, "Biometric"),
            Self::Social => write!(f, "Social"),
            Self::Reference => write!(f, "Reference"),
        }
    }
}

impl fmt::Display for VerificationLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Basic => write!(f, "Basic"),
            Self::Enhanced => write!(f, "Enhanced"),
            Self::Full => write!(f, "Full"),
        }
    }
}

impl fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmployeeOf => write!(f, "Employee of"),
            Self::MemberOf => write!(f, "Member of"),
            Self::PartnerOf => write!(f, "Partner of"),
            Self::CustomerOf => write!(f, "Customer of"),
            Self::SupplierOf => write!(f, "Supplier of"),
            Self::ParentOf => write!(f, "Parent of"),
            Self::ChildOf => write!(f, "Child of"),
            Self::Custom(s) => write!(f, "{}", s),
        }
    }
}
