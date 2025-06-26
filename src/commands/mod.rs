//! Commands for the Identity domain

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::*;

// Identity lifecycle commands

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct CreateIdentityCommand {
    pub identity_type: IdentityType,
    pub initial_claims: Option<std::collections::HashMap<ClaimType, String>>,
    pub created_by: IdentityId,
}

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIdentityCommand {
    pub identity_id: IdentityId,
    pub new_status: Option<IdentityStatus>,
    pub updated_by: IdentityId,
}

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct MergeIdentitiesCommand {
    pub source_identity: IdentityId,
    pub target_identity: IdentityId,
    pub merged_by: IdentityId,
}

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveIdentityCommand {
    pub identity_id: IdentityId,
    pub archived_by: IdentityId,
    pub reason: Option<String>,
    pub force: bool,
}

// Relationship commands

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct EstablishRelationshipCommand {
    pub from_identity: IdentityId,
    pub to_identity: IdentityId,
    pub relationship_type: RelationshipType,
    pub established_by: IdentityId,
    pub can_delegate: bool,
    pub can_revoke: bool,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub max_depth: Option<u8>,
    pub metadata: serde_json::Value,
}

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct RevokeRelationshipCommand {
    pub relationship_id: RelationshipId,
    pub revoked_by: IdentityId,
    pub reason: String,
}

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct TraverseRelationshipsCommand {
    pub from_identity: IdentityId,
    pub to_identity: Option<IdentityId>,
    pub relationship_types: Option<Vec<RelationshipType>>,
    pub max_depth: Option<u32>,
}

// Workflow commands

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct StartWorkflowCommand {
    pub identity_id: IdentityId,
    pub workflow_type: IdentityWorkflowType,
    pub started_by: IdentityId,
    pub initial_data: serde_json::Value,
}

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct ProcessWorkflowStepCommand {
    pub workflow_id: cim_domain::WorkflowId,
    pub step_name: String,
    pub step_data: serde_json::Value,
    pub processed_by: IdentityId,
}

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct CompleteWorkflowCommand {
    pub workflow_id: cim_domain::WorkflowId,
    pub outcome: crate::events::WorkflowOutcome,
    pub completed_by: IdentityId,
}

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutWorkflowCommand {
    pub workflow_id: cim_domain::WorkflowId,
}

// Verification commands

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct StartVerificationCommand {
    pub identity_id: IdentityId,
    pub verification_method: VerificationMethod,
    pub started_by: IdentityId,
}

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct ProcessVerificationCommand {
    pub identity_id: IdentityId,
    pub verification_data: serde_json::Value,
    pub processed_by: IdentityId,
}

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct CompleteVerificationCommand {
    pub identity_id: IdentityId,
    pub new_level: VerificationLevel,
    pub verified_by: IdentityId,
}

// Projection commands

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectionCommand {
    pub identity_id: IdentityId,
    pub projection_type: ProjectionType,
    pub target_domain: String,
}

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct SyncProjectionCommand {
    pub identity_id: IdentityId,
    pub projection_type: ProjectionType,
} 