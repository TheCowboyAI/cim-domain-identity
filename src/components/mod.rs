//! ECS Components for the Identity domain
//!
//! This module contains all ECS components used in the identity domain.
//! Components represent the data/state of entities in the system.

pub mod identity;
pub mod relationship;
pub mod workflow;
pub mod projection;

// Re-export commonly used types
pub use identity::{
    IdentityEntity, IdentityType, IdentityStatus, IdentityVerification,
    VerificationLevel, VerificationMethod, IdentityClaim, ClaimType,
    ExternalIdentity, IdentityMetadata,
};

pub use relationship::{
    IdentityRelationship, RelationshipType, RelationshipRules,
    RelationshipConstraint, RelationshipGraph,
};

pub use workflow::{
    IdentityWorkflow, WorkflowType, WorkflowStatus, WorkflowStep,
    StepType, StepStatus, WorkflowTransition, TransitionCondition,
};

pub use projection::{
    IdentityProjection, CrossDomainReference, ProjectionType,
    ProjectionContext, ProjectionSyncStatus,
};

// Type aliases for common types
pub type IdentityId = uuid::Uuid;
pub type RelationshipId = uuid::Uuid;
pub type WorkflowId = uuid::Uuid; 