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
//! The domain uses an ECS (Entity Component System) architecture with aggregate patterns:
//! - Components: Data structures representing identity state
//! - Systems: Functions that implement business logic and maintain aggregate invariants
//! - Events: Commands and domain events for communication
//! - Aggregates: Logical groupings of components that maintain consistency
//!
//! ## Domain Boundaries
//!
//! This domain delegates specific responsibilities to other domains:
//! - Person details → cim-domain-person
//! - Organization details → cim-domain-organization
//! - Authentication policies → cim-domain-policy
//! - Cryptographic operations → cim-security
//! - Key management → cim-keys

pub mod aggregate;
pub mod components;
pub mod systems;
pub mod commands;
pub mod events;
pub mod queries;
pub mod projections;

// Re-export key types
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
    expire_relationships_system,
    // Workflow systems
    start_workflow_system,
    process_workflow_step_system,
    complete_workflow_system,
    timeout_workflow_system,
    // Projection systems
    create_projection_system,
    sync_projections_system,
    validate_projections_system,
    // Verification systems
    start_verification_system,
    process_verification_system,
    complete_verification_system,
};

pub use commands::*;
pub use events::*;
pub use aggregate::IdentityAggregate;

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

    #[error("Aggregate invariant violated: {0}")]
    InvariantViolation(String),

    #[error("Domain error: {0}")]
    DomainError(#[from] cim_domain::DomainError),
}

pub type IdentityResult<T> = Result<T, IdentityError>;
