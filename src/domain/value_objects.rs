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

/// Authentication credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password_hash: String, // Never store plain passwords
}

impl Credentials {
    pub fn new(username: String, password_hash: String) -> Self {
        Credentials {
            username: username.to_lowercase(),
            password_hash,
        }
    }
}

/// Authentication method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthMethod {
    Password,
    OAuth2,
    SAML,
    ApiKey,
    Certificate,
}

/// Authentication status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthStatus {
    pub is_authenticated: bool,
    pub method: Option<AuthMethod>,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
    pub failed_attempts: u32,
    pub locked_until: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for AuthStatus {
    fn default() -> Self {
        AuthStatus {
            is_authenticated: false,
            method: None,
            last_login: None,
            failed_attempts: 0,
            locked_until: None,
        }
    }
}

/// Multi-factor authentication settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaSettings {
    pub enabled: bool,
    pub method: MfaMethod,
    pub backup_codes: Vec<String>, // Hashed backup codes
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MfaMethod {
    Totp,    // Time-based One-Time Password
    Sms,     // SMS verification
    Email,   // Email verification
    App,     // Authenticator app
}

impl Default for MfaSettings {
    fn default() -> Self {
        MfaSettings {
            enabled: false,
            method: MfaMethod::Totp,
            backup_codes: Vec::new(),
        }
    }
}

/// API Key for service authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub key_hash: String,
    pub name: String,
    pub permissions: Vec<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
}
