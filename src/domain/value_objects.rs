//! Shared value objects for the Identity context

use serde::{Deserialize, Serialize};
use crate::IdentityError;

/// Email address value object with validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    /// Create a new Email with validation
    pub fn new(email: String) -> Result<Self, IdentityError> {
        // Basic email validation
        if email.contains('@') && email.contains('.') && email.len() >= 5 {
            Ok(Email(email.to_lowercase()))
        } else {
            Err(IdentityError::InvalidEmail(email))
        }
    }

    /// Get the email as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Name value object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Name {
    pub first: String,
    pub last: String,
    pub middle: Option<String>,
}

impl Name {
    pub fn new(first: String, last: String, middle: Option<String>) -> Self {
        Name { first, last, middle }
    }

    pub fn full_name(&self) -> String {
        match &self.middle {
            Some(middle) => format!("{} {} {}", self.first, middle, self.last),
            None => format!("{} {}", self.first, self.last),
        }
    }
}

/// Address value object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

/// Phone number value object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneNumber {
    pub country_code: String,
    pub number: String,
}

/// Trust level for identity verification
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TrustLevel {
    Unverified = 0,
    EmailVerified = 1,
    PhoneVerified = 2,
    DocumentVerified = 3,
    FullyVerified = 4,
}

impl Default for TrustLevel {
    fn default() -> Self {
        TrustLevel::Unverified
    }
}
