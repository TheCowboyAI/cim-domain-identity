//! Identity Domain - ECS-based identity and relationship management
//!
//! This domain manages identities, their relationships, and workflows.
//! It serves as the orchestration layer for identity-related processes.

pub mod aggregate;
pub mod commands;
pub mod components;
pub mod events;
pub mod projections;
pub mod queries;
pub mod systems;

// Re-export key types
pub use aggregate::*;
pub use commands::*;
pub use components::*;
pub use events::*;
pub use systems::*;
// Don't re-export all from queries and projections to avoid conflicts
pub use projections::{
    IdentityProjectionSystem, IdentityStatusProjection, RelationshipGraphProjection,
    WorkflowStatusProjection,
};
pub use queries::{
    FindActiveWorkflowsQuery, FindIdentitiesByTypeQuery, FindIdentityByIdQuery,
    FindRelationshipsByIdentityQuery, GetIdentityProjectionsQuery,
    GetIdentityVerificationStatusQuery,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Result type for identity operations
pub type IdentityResult<T> = Result<T, IdentityError>;

/// Error types for the identity domain
#[derive(Error, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IdentityError {
    #[error("Identity not found: {0}")]
    IdentityNotFound(Uuid),

    #[error("Identity already exists: {0}")]
    IdentityAlreadyExists(String),

    #[error("Identity is archived")]
    IdentityArchived,

    #[error("Identity is not active")]
    IdentityNotActive,

    #[error("Identity has been merged")]
    IdentityMerged,

    #[error("Already archived")]
    AlreadyArchived,

    #[error("Incompatible identity types for merge")]
    IncompatibleIdentityTypes,

    #[error("Target identity is less verified than source")]
    TargetLessVerified,

    #[error("Identity has {0} active relationships")]
    HasActiveRelationships(usize),

    #[error("Cannot create self-relationship")]
    SelfRelationship,

    #[error("Invalid ownership percentage")]
    InvalidOwnershipPercentage,

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Invalid identity type")]
    InvalidIdentityType,

    #[error("Invalid identity status transition")]
    InvalidStatusTransition,

    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    #[error("Relationship conflict: {0}")]
    RelationshipConflict(String),

    #[error("Workflow error: {0}")]
    WorkflowError(String),

    #[error("Workflow in progress")]
    WorkflowInProgress,

    #[error("Invariant violation: {0}")]
    InvariantViolation(String),

    #[error("Not found")]
    NotFound,

    #[error("Invalid transition")]
    InvalidTransition,
}
