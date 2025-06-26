//! # Identity Domain (ECS-Based)
//!
//! This domain manages identities, their relationships, and workflows within the CIM architecture.
//! It focuses on identity lifecycle, relationships between identities, and identity-related workflows.
//!
//! ## Key Concepts
//!
//! - **Identity**: Core entity representing a person, organization, system, or external identity
//! - **Relationships**: Connections between identities (member of, reports to, owns, etc.)
//! - **Workflows**: Identity-related processes (verification, onboarding, merging, etc.)
//! - **Projections**: Cross-domain representations of identities
//!
//! ## Architecture
//!
//! The domain uses an ECS (Entity Component System) architecture:
//! - Components: Data structures representing identity state
//! - Systems: Functions that implement business logic
//! - Events: Commands and domain events for communication
//!
//! ## Domain Boundaries
//!
//! This domain delegates specific responsibilities to other domains:
//! - Person details → cim-domain-person
//! - Organization details → cim-domain-organization
//! - Authentication policies → cim-domain-policy
//! - Cryptographic operations → cim-security
//! - Key management → cim-keys

pub mod components;
pub mod systems;
pub mod commands;
pub mod events;

// Legacy modules (to be migrated)
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod ports;
pub mod conceptual;
pub mod handlers;

// Re-export key types from new architecture
pub use components::{
    IdentityId,
    IdentityEntity,
    IdentityType,
    IdentityStatus,
    IdentityVerification,
    VerificationLevel,
    VerificationMethod,
    IdentityRelationship,
    RelationshipType,
    IdentityWorkflow,
    IdentityWorkflowType,
    WorkflowState,
    WorkflowStatus,
    IdentityProjection,
    ProjectionType,
};

pub use systems::{
    // Lifecycle systems
    create_identity_system,
    update_identity_system,
    merge_identities_system,
    archive_identity_system,
    // Relationship systems
    establish_relationship_system,
    validate_relationship_system,
    traverse_relationships_system,
    // Workflow systems
    start_workflow_system,
    process_workflow_step_system,
    complete_workflow_system,
    // Projection systems
    create_projection_system,
    sync_projections_system,
    // Verification systems
    start_verification_system,
    process_verification_system,
    complete_verification_system,
};

pub use commands::*;
pub use events::*;

// Legacy re-exports (for backward compatibility during migration)
pub use domain::{
    person::{Person, PersonId, PersonCommand, PersonEvent},
    organization::{Organization, OrganizationId, OrganizationCommand, OrganizationEvent, OrganizationType},
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
    #[error("Identity not found: {0:?}")]
    IdentityNotFound(IdentityId),

    #[error("Invalid identity type for operation")]
    InvalidIdentityType,

    #[error("Relationship not allowed between identity types")]
    RelationshipNotAllowed,

    #[error("Workflow already in progress for identity")]
    WorkflowInProgress,

    #[error("Invalid workflow transition")]
    InvalidWorkflowTransition,

    #[error("Projection sync failed: {0}")]
    ProjectionSyncFailed(String),

    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    #[error("Cannot merge identities: {0}")]
    MergeNotAllowed(String),

    #[error("Cannot archive identity with active relationships")]
    ArchiveWithActiveRelationships,

    // Legacy errors (for backward compatibility)
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
