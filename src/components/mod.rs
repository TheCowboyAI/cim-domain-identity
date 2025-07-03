//! ECS Components for the Identity domain
//!
//! This module contains all ECS components used in the identity domain.
//! Components represent the data/state of entities in the system.

pub mod identity;
pub mod projection;
pub mod relationship;
pub mod workflow;

// Re-export commonly used types
pub use identity::{
    ClaimType, ExternalIdentity, IdentityClaim, IdentityEntity, IdentityMetadata, IdentityStatus,
    IdentityType, IdentityVerification, VerificationLevel, VerificationMethod,
};

pub use relationship::{
    IdentityRelationship, RelationshipConstraint, RelationshipGraph, RelationshipRules,
    RelationshipType,
};

pub use workflow::{
    IdentityWorkflow, StepStatus, StepType, TransitionCondition, WorkflowStatus, WorkflowStep,
    WorkflowTransition, WorkflowType,
};

pub use projection::{
    CrossDomainReference, IdentityProjection, ProjectionContext, ProjectionSyncStatus,
    ProjectionType,
};

// Type aliases for common types
pub type IdentityId = uuid::Uuid;
pub type RelationshipId = uuid::Uuid;
pub type WorkflowId = uuid::Uuid;
