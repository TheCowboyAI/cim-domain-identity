//! Events for the Identity domain

use crate::components::{
    CrossDomainReference, IdentityId, IdentityStatus, IdentityType, ProjectionType, RelationshipId,
    RelationshipType, VerificationLevel, VerificationMethod, WorkflowStatus, WorkflowType,
};
use bevy::ecs::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Event fired when an identity is created
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityCreated {
    pub identity_id: Uuid,
    pub identity_type: IdentityType,
    pub created_by: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub external_reference: Option<CrossDomainReference>,
}

/// Event fired when an identity is updated
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityUpdated {
    pub identity_id: IdentityId,
    pub old_status: IdentityStatus,
    pub new_status: IdentityStatus,
    pub updated_by: IdentityId,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Event fired when identities are merged
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

/// Event fired when an identity is archived
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityArchived {
    pub identity_id: IdentityId,
    pub previous_status: IdentityStatus,
    pub archived_by: IdentityId,
    pub archived_at: chrono::DateTime<chrono::Utc>,
    pub reason: Option<String>,
}

/// Event fired when a relationship is established
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipEstablished {
    pub relationship_id: RelationshipId,
    pub from_identity: IdentityId,
    pub to_identity: IdentityId,
    pub relationship_type: RelationshipType,
    pub established_by: IdentityId,
    pub established_at: chrono::DateTime<chrono::Utc>,
}

/// Event fired when a relationship is validated
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipValidated {
    pub relationship_id: RelationshipId,
    pub is_valid: bool,
    pub reason: String,
    pub validated_at: chrono::DateTime<chrono::Utc>,
}

/// Event fired when a relationship expires
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipExpired {
    pub relationship_id: RelationshipId,
    pub from_identity: IdentityId,
    pub to_identity: IdentityId,
    pub relationship_type: RelationshipType,
    pub expired_at: chrono::DateTime<chrono::Utc>,
}

/// Event fired when relationships are traversed
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipsTraversed {
    pub from_identity: IdentityId,
    pub to_identity: Option<IdentityId>,
    pub paths: Vec<(Vec<IdentityId>, Vec<RelationshipId>)>,
    pub total_identities_visited: usize,
    pub traversed_at: chrono::DateTime<chrono::Utc>,
}

/// Event fired when a relationship is revoked
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipRevoked {
    pub relationship_id: RelationshipId,
    pub revoked_by: IdentityId,
    pub revoked_at: chrono::DateTime<chrono::Utc>,
    pub reason: Option<String>,
}

/// Event fired when a workflow is started
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStarted {
    pub workflow_id: Uuid,
    pub identity_id: IdentityId,
    pub workflow_type: WorkflowType,
    pub started_by: IdentityId,
    pub started_at: DateTime<Utc>,
    pub context: serde_json::Value,
}

/// Event fired when a workflow step is completed
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStepCompleted {
    pub workflow_id: Uuid,
    pub identity_id: IdentityId,
    pub workflow_type: WorkflowType,
    pub step_id: String,
    pub completed_at: DateTime<Utc>,
}

/// Event fired when a workflow is completed
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowCompleted {
    pub workflow_id: Uuid,
    pub identity_id: IdentityId,
    pub workflow_type: WorkflowType,
    pub final_status: WorkflowStatus,
    pub completed_at: DateTime<Utc>,
}

/// Event fired when a workflow times out
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTimedOut {
    pub workflow_id: uuid::Uuid,
    pub identity_id: IdentityId,
    pub workflow_type: WorkflowType,
    pub step_id: String,
    pub timed_out_at: chrono::DateTime<chrono::Utc>,
}

/// Event fired when verification is started
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct VerificationStarted {
    pub identity_id: IdentityId,
    pub verification_method: VerificationMethod,
    pub initiated_by: IdentityId,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

/// Event fired when verification is completed
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct VerificationCompleted {
    pub identity_id: IdentityId,
    pub verification_successful: bool,
    pub new_verification_level: VerificationLevel,
    pub verified_by: IdentityId,
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

/// Event fired when a projection is created
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct ProjectionCreated {
    pub projection_id: uuid::Uuid,
    pub identity_id: IdentityId,
    pub projection_type: ProjectionType,
    pub target_domain: String,
    pub target_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Event fired when projections are synced
#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct ProjectionsSynced {
    pub identity_id: Option<IdentityId>,
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
