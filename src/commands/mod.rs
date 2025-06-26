//! Commands for the Identity domain

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::{IdentityId, IdentityType, IdentityStatus, ClaimType};
use std::collections::HashMap;

/// Command to create a new identity
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct CreateIdentityCommand {
    pub identity_type: IdentityType,
    pub initial_claims: Option<HashMap<ClaimType, String>>,
    pub created_by: IdentityId,
}

/// Command to update an identity
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIdentityCommand {
    pub identity_id: IdentityId,
    pub new_status: Option<IdentityStatus>,
    pub updated_by: IdentityId,
}

/// Command to merge two identities
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct MergeIdentitiesCommand {
    pub source_identity: IdentityId,
    pub target_identity: IdentityId,
    pub merged_by: IdentityId,
    pub reason: String,
}

/// Command to archive an identity
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveIdentityCommand {
    pub identity_id: IdentityId,
    pub archived_by: IdentityId,
    pub reason: String,
    pub force: bool,
}

/// Command to establish a relationship
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct EstablishRelationshipCommand {
    pub from_identity: IdentityId,
    pub to_identity: IdentityId,
    pub relationship_type: crate::components::RelationshipType,
    pub established_by: IdentityId,
    pub metadata: serde_json::Value,
}

/// Command to revoke a relationship
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct RevokeRelationshipCommand {
    pub relationship_id: uuid::Uuid,
    pub revoked_by: IdentityId,
    pub reason: String,
}

/// Command to start a workflow
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct StartWorkflowCommand {
    pub identity_id: IdentityId,
    pub workflow_type: crate::components::IdentityWorkflowType,
    pub started_by: IdentityId,
    pub context: serde_json::Value,
}

/// Command to complete a workflow step
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct CompleteWorkflowStepCommand {
    pub workflow_id: uuid::Uuid,
    pub step_id: String,
    pub completed_by: IdentityId,
    pub result: serde_json::Value,
}

/// Command to start verification
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct StartVerificationCommand {
    pub identity_id: IdentityId,
    pub verification_method: crate::components::VerificationMethod,
    pub initiated_by: IdentityId,
}

/// Command to complete verification
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct CompleteVerificationCommand {
    pub identity_id: IdentityId,
    pub verification_result: bool,
    pub verification_level: crate::components::VerificationLevel,
    pub verified_by: IdentityId,
}

/// Command to create a projection
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectionCommand {
    pub identity_id: IdentityId,
    pub projection_type: crate::components::ProjectionType,
    pub target_domain: String,
    pub context: crate::components::ProjectionContext,
}

/// Command to sync projections
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct SyncProjectionsCommand {
    pub identity_id: Option<IdentityId>,
    pub projection_type: Option<crate::components::ProjectionType>,
    pub force: bool,
} 