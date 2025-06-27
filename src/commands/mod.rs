//! Commands for the Identity domain

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::components::{
    IdentityType, IdentityStatus, VerificationLevel, VerificationMethod,
    RelationshipType, ProjectionType, IdentityId, WorkflowType,
    ClaimType, RelationshipRules, RelationshipId, ProjectionContext,
};

// Identity lifecycle commands

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct CreateIdentityCommand {
    pub identity_type: IdentityType,
    pub initial_claims: Option<std::collections::HashMap<ClaimType, String>>,
    pub created_by: IdentityId,
    pub tags: Vec<String>,
    pub metadata: serde_json::Value,
    pub external_reference: Option<crate::components::CrossDomainReference>,
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
    pub merge_reason: String,
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
    pub rules: RelationshipRules,
    pub established_by: IdentityId,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct ValidateRelationshipCommand {
    pub relationship_id: RelationshipId,
    pub validated_by: IdentityId,
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
    pub max_depth: Option<u32>,
    pub relationship_filter: Option<Vec<RelationshipType>>,
}

// Workflow commands

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct StartWorkflowCommand {
    pub identity_id: IdentityId,
    pub workflow_type: WorkflowType,
    pub started_by: IdentityId,
    pub context: serde_json::Value,
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
    pub initiated_by: IdentityId,
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
    pub verification_result: bool,
    pub verification_level: VerificationLevel,
    pub verification_method: VerificationMethod,
    pub verified_by: IdentityId,
}

// Projection commands

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectionCommand {
    pub identity_id: IdentityId,
    pub projection_type: ProjectionType,
    pub target_domain: String,
    pub context: ProjectionContext,
}

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct SyncProjectionsCommand {
    pub identity_id: Option<IdentityId>,
    pub projection_type: Option<ProjectionType>,
    pub force: bool,
} 