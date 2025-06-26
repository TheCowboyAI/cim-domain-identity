//! Projections and read models for the Identity domain
//!
//! This module provides optimized read models and cross-domain projections.

use bevy_ecs::prelude::*;
use bevy_app::prelude::*;
use crate::{
    components::*,
    events::*,
};
use std::collections::HashMap;
use cim_domain::WorkflowId;

/// Identity summary projection for fast lookups
#[derive(Resource, Default)]
pub struct IdentitySummaryProjection {
    pub by_id: HashMap<IdentityId, IdentitySummary>,
    pub by_type: HashMap<IdentityType, Vec<IdentityId>>,
    pub by_claim: HashMap<(ClaimType, String), Vec<IdentityId>>,
}

#[derive(Debug, Clone)]
pub struct IdentitySummary {
    pub identity_id: IdentityId,
    pub identity_type: IdentityType,
    pub status: IdentityStatus,
    pub verification_level: VerificationLevel,
    pub display_name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Relationship graph projection for fast traversal
#[derive(Resource, Default)]
pub struct RelationshipGraphProjection {
    /// Adjacency list representation
    pub forward_edges: HashMap<IdentityId, Vec<(IdentityId, RelationshipType)>>,
    pub reverse_edges: HashMap<IdentityId, Vec<(IdentityId, RelationshipType)>>,
    
    /// Relationship type indices
    pub by_type: HashMap<RelationshipType, Vec<(IdentityId, IdentityId)>>,
}

/// Workflow status projection
#[derive(Resource, Default)]
pub struct WorkflowStatusProjection {
    /// Active workflows by identity
    pub active_by_identity: HashMap<IdentityId, Vec<WorkflowSummary>>,
    
    /// Workflows by type
    pub by_type: HashMap<IdentityWorkflowType, Vec<WorkflowId>>,
    
    /// Workflows requiring action
    pub requiring_action: Vec<(WorkflowId, IdentityId, WorkflowActionRequired)>,
}

#[derive(Debug, Clone)]
pub struct WorkflowSummary {
    pub workflow_id: WorkflowId,
    pub workflow_type: IdentityWorkflowType,
    pub status: WorkflowStatus,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub current_step: String,
}

#[derive(Debug, Clone)]
pub enum WorkflowActionRequired {
    UserInput { fields: Vec<String> },
    Approval { approver: IdentityId },
    Verification { method: VerificationMethod },
}

/// System to update identity summary projection
pub fn update_identity_summary_projection(
    mut projection: ResMut<IdentitySummaryProjection>,
    mut created_events: EventReader<IdentityCreated>,
    mut updated_events: EventReader<IdentityUpdated>,
    mut archived_events: EventReader<IdentityArchived>,
    identities: Query<(&IdentityEntity, &IdentityVerification, &IdentityMetadata)>,
) {
    // Handle created events
    for event in created_events.read() {
        let summary = IdentitySummary {
            identity_id: event.identity_id,
            identity_type: event.identity_type,
            status: IdentityStatus::Pending,
            verification_level: VerificationLevel::Unverified,
            display_name: format!("{:?}-{}", event.identity_type, event.identity_id),
            created_at: event.created_at,
            updated_at: event.created_at,
        };

        projection.by_id.insert(event.identity_id, summary);
        projection.by_type
            .entry(event.identity_type)
            .or_default()
            .push(event.identity_id);
    }

    // Handle updated events
    for event in updated_events.read() {
        if let Some(summary) = projection.by_id.get_mut(&event.identity_id) {
            summary.status = event.new_status;
            summary.updated_at = event.updated_at;
        }
    }

    // Handle archived events
    for event in archived_events.read() {
        if let Some(summary) = projection.by_id.get_mut(&event.identity_id) {
            summary.status = IdentityStatus::Archived;
            summary.updated_at = event.archived_at;
        }
    }
}

/// System to update relationship graph projection
pub fn update_relationship_graph_projection(
    mut projection: ResMut<RelationshipGraphProjection>,
    mut established_events: EventReader<RelationshipEstablished>,
    mut expired_events: EventReader<RelationshipExpired>,
) {
    // Handle established relationships
    for event in established_events.read() {
        // Update forward edges
        projection.forward_edges
            .entry(event.from_identity)
            .or_default()
            .push((event.to_identity, event.relationship_type.clone()));

        // Update reverse edges
        projection.reverse_edges
            .entry(event.to_identity)
            .or_default()
            .push((event.from_identity, event.relationship_type.clone()));

        // Update type index
        projection.by_type
            .entry(event.relationship_type.clone())
            .or_default()
            .push((event.from_identity, event.to_identity));
    }

    // Handle expired relationships
    for event in expired_events.read() {
        // Remove from forward edges
        if let Some(edges) = projection.forward_edges.get_mut(&event.from_identity) {
            edges.retain(|(id, _)| *id != event.to_identity);
        }

        // Remove from reverse edges
        if let Some(edges) = projection.reverse_edges.get_mut(&event.to_identity) {
            edges.retain(|(id, _)| *id != event.from_identity);
        }

        // Remove from type index
        if let Some(pairs) = projection.by_type.get_mut(&event.relationship_type) {
            pairs.retain(|(from, to)| !(*from == event.from_identity && *to == event.to_identity));
        }
    }
}

/// System to update workflow status projection
pub fn update_workflow_status_projection(
    mut projection: ResMut<WorkflowStatusProjection>,
    mut started_events: EventReader<WorkflowStarted>,
    mut step_events: EventReader<WorkflowStepCompleted>,
    mut completed_events: EventReader<WorkflowCompleted>,
    mut timeout_events: EventReader<WorkflowTimedOut>,
) {
    // Handle workflow started
    for event in started_events.read() {
        let summary = WorkflowSummary {
            workflow_id: event.workflow_id,
            workflow_type: event.workflow_type.clone(),
            status: WorkflowStatus::InProgress,
            started_at: event.started_at,
            current_step: event.initial_step.clone(),
        };

        projection.active_by_identity
            .entry(event.identity_id)
            .or_default()
            .push(summary);

        projection.by_type
            .entry(event.workflow_type.clone())
            .or_default()
            .push(event.workflow_id);
    }

    // Handle step completed
    for event in step_events.read() {
        // Update current step in active workflows
        for summaries in projection.active_by_identity.values_mut() {
            if let Some(summary) = summaries.iter_mut()
                .find(|s| s.workflow_id == event.workflow_id) {
                summary.current_step = event.next_step.clone();
                summary.status = event.new_status;

                // Check if action required
                match &event.new_status {
                    WorkflowStatus::WaitingForInput => {
                        projection.requiring_action.push((
                            event.workflow_id,
                            event.identity_id,
                            WorkflowActionRequired::UserInput {
                                fields: vec!["default".to_string()], // Would come from workflow definition
                            },
                        ));
                    }
                    WorkflowStatus::WaitingForApproval => {
                        projection.requiring_action.push((
                            event.workflow_id,
                            event.identity_id,
                            WorkflowActionRequired::Approval {
                                approver: IdentityId::new(), // Would be determined by workflow
                            },
                        ));
                    }
                    _ => {}
                }
            }
        }
    }

    // Handle workflow completed
    for event in completed_events.read() {
        // Remove from active workflows
        if let Some(summaries) = projection.active_by_identity.get_mut(&event.identity_id) {
            summaries.retain(|s| s.workflow_id != event.workflow_id);
        }

        // Remove from requiring action
        projection.requiring_action.retain(|(id, _, _)| *id != event.workflow_id);
    }

    // Handle workflow timeout
    for event in timeout_events.read() {
        // Remove from active workflows
        if let Some(summaries) = projection.active_by_identity.get_mut(&event.identity_id) {
            summaries.retain(|s| s.workflow_id != event.workflow_id);
        }

        // Remove from requiring action
        projection.requiring_action.retain(|(id, _, _)| *id != event.workflow_id);
    }
}

/// Cross-domain projection for person details
#[derive(Resource, Default)]
pub struct PersonDetailsProjection {
    pub by_identity: HashMap<IdentityId, PersonDetails>,
}

#[derive(Debug, Clone)]
pub struct PersonDetails {
    pub identity_id: IdentityId,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub organization: Option<(IdentityId, String)>,
    pub manager: Option<(IdentityId, String)>,
}

/// Cross-domain projection for organization details
#[derive(Resource, Default)]
pub struct OrganizationDetailsProjection {
    pub by_identity: HashMap<IdentityId, OrganizationDetails>,
}

#[derive(Debug, Clone)]
pub struct OrganizationDetails {
    pub identity_id: IdentityId,
    pub name: String,
    pub member_count: usize,
    pub department_count: usize,
    pub parent_org: Option<(IdentityId, String)>,
}

/// Plugin to register all projection systems
pub struct IdentityProjectionsPlugin;

impl Plugin for IdentityProjectionsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources
            .init_resource::<IdentitySummaryProjection>()
            .init_resource::<RelationshipGraphProjection>()
            .init_resource::<WorkflowStatusProjection>()
            .init_resource::<PersonDetailsProjection>()
            .init_resource::<OrganizationDetailsProjection>()
            
            // Add update systems
            .add_systems(Update, (
                update_identity_summary_projection,
                update_relationship_graph_projection,
                update_workflow_status_projection,
            ));
    }
} 