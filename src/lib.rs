//! # Identity Context
//!
//! This bounded context manages identity-related aggregates including Person and Organization.
//! It provides a clear boundary for identity management within the CIM architecture.
//!
//! ## Key Concepts
//!
//! - **Person**: Represents an individual with identity attributes
//! - **Organization**: Represents a group or company with members
//! - **Conceptual Integration**: All identity concepts are projected into conceptual space
//!
//! ## Architecture
//!
//! The context follows hexagonal architecture with:
//! - Domain layer: Core business logic and aggregates
//! - Application layer: Command and query handlers
//! - Infrastructure layer: Repository implementations
//! - Ports layer: Interfaces for inbound and outbound communication
//! - Conceptual layer: Projections into conceptual space

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod ports;
pub mod conceptual;

// Re-export key types for convenience
pub use domain::{
    person::{Person, PersonId, PersonCommand, PersonEvent},
    organization::{Organization, OrganizationId, OrganizationCommand, OrganizationEvent, OrganizationType},
};

pub use ports::{
    inbound::{IdentityCommandHandler, IdentityQueryHandler},
    outbound::{PersonRepository, OrganizationRepository},
};

pub use conceptual::{
    IdentityConceptProducer,
    IdentityDimensions,
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
    DomainError(#[from] cim_core_domain::DomainError),
}

pub type IdentityResult<T> = Result<T, IdentityError>;
