//! Domain events for the Identity context

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::{
    IdentityId, IdentityType, IdentityStatus, VerificationLevel,
    RelationshipType, IdentityWorkflowType, ProjectionType,
};
use cim_domain::WorkflowId;

/// Event emitted when an identity is created
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityCreated {
    pub identity_id: IdentityId,
    pub identity_type: IdentityType,
    pub created_by: IdentityId,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Event emitted when an identity is updated
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityUpdated {
    pub identity_id: IdentityId,
    pub old_status: IdentityStatus,
    pub new_status: IdentityStatus,
    pub updated_by: IdentityId,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Event emitted when identities are merged
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct IdentitiesMerged {
    pub source_identity: IdentityId,
    pub target_identity: IdentityId,
    pub merged_by: IdentityId,
    pub merged_at: chrono::DateTime<chrono::Utc>,
    pub migrated_relationships: usize,
    pub migrated_workflows: usize,
    pub retained_verification_level: VerificationLevel,
}

/// Event emitted when an identity is archived
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityArchived {
    pub identity_id: IdentityId,
    pub previous_status: IdentityStatus,
    pub archived_by: IdentityId,
    pub archived_at: chrono::DateTime<chrono::Utc>,
    pub reason: Option<String>,
}

/// Event emitted when a relationship is established
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipEstablished {
    pub relationship_id: uuid::Uuid,
    pub from_identity: IdentityId,
    pub to_identity: IdentityId,
    pub relationship_type: RelationshipType,
    pub established_by: IdentityId,
    pub established_at: chrono::DateTime<chrono::Utc>,
}

/// Event emitted when a relationship is revoked
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipRevoked {
    pub relationship_id: uuid::Uuid,
    pub from_identity: IdentityId,
    pub to_identity: IdentityId,
    pub relationship_type: RelationshipType,
    pub revoked_by: IdentityId,
    pub revoked_at: chrono::DateTime<chrono::Utc>,
    pub reason: String,
}

/// Event emitted when verification starts
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct VerificationStarted {
    pub identity_id: IdentityId,
    pub verification_method: crate::components::VerificationMethod,
    pub initiated_by: IdentityId,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

/// Event emitted when verification completes
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct VerificationCompleted {
    pub identity_id: IdentityId,
    pub verification_successful: bool,
    pub new_verification_level: VerificationLevel,
    pub verified_by: IdentityId,
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

/// Event emitted when a workflow starts
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStarted {
    pub workflow_id: WorkflowId,
    pub identity_id: IdentityId,
    pub workflow_type: IdentityWorkflowType,
    pub started_by: IdentityId,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub initial_step: String,
}

/// Event emitted when a workflow step completes
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStepCompleted {
    pub workflow_id: WorkflowId,
    pub identity_id: IdentityId,
    pub step_name: String,
    pub next_step: String,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub new_status: crate::components::WorkflowStatus,
}

/// Event emitted when a workflow completes
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowCompleted {
    pub workflow_id: WorkflowId,
    pub identity_id: IdentityId,
    pub workflow_type: IdentityWorkflowType,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub outcome: WorkflowOutcome,
}

/// Event emitted when a projection is created
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct ProjectionCreated {
    pub projection_id: uuid::Uuid,
    pub identity_id: IdentityId,
    pub projection_type: ProjectionType,
    pub target_domain: String,
    pub target_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Event emitted when projections are synced
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct ProjectionsSynced {
    pub identity_id: IdentityId,
    pub projections_synced: usize,
    pub sync_errors: usize,
    pub synced_at: chrono::DateTime<chrono::Utc>,
}

/// Cross-domain event: Identity linked to person
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityLinkedToPerson {
    pub identity_id: IdentityId,
    pub person_id: uuid::Uuid,
    pub linked_at: chrono::DateTime<chrono::Utc>,
}

/// Cross-domain event: Identity linked to organization
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityLinkedToOrganization {
    pub identity_id: IdentityId,
    pub organization_id: uuid::Uuid,
    pub role: Option<String>,
    pub linked_at: chrono::DateTime<chrono::Utc>,
}

/// Cross-domain event: Authentication requested
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityAuthenticationRequested {
    pub identity_id: IdentityId,
    pub authentication_method: String,
    pub context: serde_json::Value,
    pub requested_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowOutcome {
    Approved,
    Rejected,
    Cancelled,
    Completed,
} 