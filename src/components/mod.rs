//! ECS Components for the Identity domain
//!
//! This module contains all ECS components used in the identity domain.
//! Components represent the data/state of entities in the system.

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Re-export component types
pub use identity::*;
pub use relationship::*;
pub use workflow::*;
pub use projection::*;

mod identity;
mod relationship;
mod workflow;
mod projection;

/// Common metadata for all identity components
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityMetadata {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<IdentityId>,
    pub version: u64,
}

impl Default for IdentityMetadata {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            updated_at: now,
            created_by: None,
            version: 1,
        }
    }
}

/// Unique identifier for identities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdentityId(pub Uuid);

impl IdentityId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for IdentityId {
    fn default() -> Self {
        Self::new()
    }
}

// Re-export commonly used types
pub use identity::{
    IdentityId,
    IdentityEntity,
    IdentityType,
    IdentityStatus,
    IdentityMetadata,
    IdentityVerification,
    VerificationLevel,
    VerificationMethod,
    IdentityClaim,
    ClaimType,
};

pub use relationship::{
    RelationshipId,
    IdentityRelationship,
    RelationshipType,
    RelationshipRules,
    RelationshipPath,
    RelationshipGraph,
};

pub use workflow::{
    IdentityWorkflow,
    IdentityWorkflowType,
    WorkflowState,
    WorkflowStatus,
    WorkflowStep,
    WorkflowTransition,
};

pub use projection::{
    IdentityProjection,
    ProjectionType,
    ProjectionContext,
    CrossDomainReference,
    IdentityView,
}; 