//! # Identity Domain
//!
//! This domain module manages identity and authentication for People and Organizations.
//! It provides core identity management capabilities within the CIM architecture.
//!
//! ## Key Concepts
//!
//! - **Person**: Represents an individual with identity attributes
//! - **Organization**: Represents a group or company with members
//! - **Conceptual Integration**: All identity concepts are projected into conceptual space
//!
//! ## Architecture
//!
//! The domain module follows DDD principles with:
//! - Domain layer: Core business logic and aggregates (Person, Organization)
//! - Application layer: Command and query handlers for identity operations
//! - Infrastructure layer: Repository implementations for persistence
//! - Ports layer: Interfaces for authentication and authorization
//! - Conceptual layer: Identity projections into conceptual space
//!
//! ## Authentication Features
//!
//! - **Person Authentication**: Username/password, MFA, trust levels
//! - **Organization Authentication**: API keys, service accounts
//! - **Security**: Password hashing, account locking, audit trails

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod ports;
pub mod conceptual;
pub mod handlers;

// Re-export key types for convenience
pub use domain::{
    person::{Person, PersonId, PersonCommand, PersonEvent},
    organization::{Organization, OrganizationId, OrganizationCommand, OrganizationEvent, OrganizationType},
    // Re-export value objects
    Email, Name, Address, PhoneNumber, TrustLevel, Credentials, AuthStatus, MfaSettings, MfaMethod,
};

pub use ports::{
    inbound::{IdentityCommandHandler, IdentityQueryHandler},
    outbound::{PersonRepository, OrganizationRepository},
};

pub use conceptual::{
    IdentityConceptProducer,
    IdentityDimensions,
};

pub use handlers::{
    AuthenticationEventHandler,
    AuthenticationRequested,
    IdentityVerificationRequested,
    IdentityVerified,
    IdentityVerificationLevel,
};

/// Identity context error types
#[derive(Debug, thiserror::Error)]
pub enum IdentityError {
    #[error("Person not found: {0}")]
    PersonNotFound(PersonId),

    #[error("Organization not found: {0}")]
    OrganizationNotFound(OrganizationId),

    #[error("Invalid email format: {0}")]
    InvalidEmail(String),

    #[error("Person already exists with email: {0}")]
    PersonAlreadyExists(String),

    #[error("Organization already exists with name: {0}")]
    OrganizationAlreadyExists(String),

    #[error("Domain error: {0}")]
    DomainError(#[from] cim_domain::DomainError),

    #[error("Event store error: {0}")]
    EventStoreError(#[from] cim_domain::infrastructure::EventStoreError),
}

pub type IdentityResult<T> = Result<T, IdentityError>;
