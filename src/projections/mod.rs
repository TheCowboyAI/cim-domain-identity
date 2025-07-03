//! Projections and read models for the Identity domain

use crate::{components::*, events::*, queries::IdentityView};
use bevy_ecs::prelude::*;

/// Identity projection system marker
pub struct IdentityProjectionSystem;

/// Relationship graph projection marker
pub struct RelationshipGraphProjection;

/// Identity status projection marker
pub struct IdentityStatusProjection;

/// Workflow status projection marker
pub struct WorkflowStatusProjection;

/// System to update identity projections on events
pub fn update_identity_projections(
    mut events: EventReader<IdentityCreated>,
    mut commands: Commands,
) {
    for event in events.read() {
        // Create projection components for the new identity
        commands.spawn((IdentityProjection {
            identity_id: event.identity_id,
            projection_type: ProjectionType::Primary,
            target_domain: "identity".to_string(),
            sync_status: ProjectionSyncStatus::Synced,
            last_sync: event.created_at,
            last_synced: event.created_at,
        },));
    }
}

/// System to update relationship graph projections
pub fn update_relationship_graph(
    mut events: EventReader<RelationshipEstablished>,
    mut graphs: Query<&mut RelationshipGraph>,
) {
    for event in events.read() {
        // Update any relationship graphs that include these identities
        for graph in graphs.iter() {
            if graph.identity_id == event.from_identity || graph.identity_id == event.to_identity {
                // In a real implementation, would update the graph structure
            }
        }
    }
}

/// System to maintain identity status projections
pub fn update_identity_status_projection(
    mut identity_events: EventReader<IdentityUpdated>,
    mut archive_events: EventReader<IdentityArchived>,
    mut merge_events: EventReader<IdentitiesMerged>,
    mut projections: Query<&mut IdentityProjection>,
) {
    // Handle status updates
    for event in identity_events.read() {
        for mut projection in projections.iter_mut() {
            if projection.identity_id == event.identity_id {
                projection.last_synced = event.updated_at;
            }
        }
    }

    // Handle archives
    for event in archive_events.read() {
        for mut projection in projections.iter_mut() {
            if projection.identity_id == event.identity_id {
                projection.last_synced = event.archived_at;
            }
        }
    }

    // Handle merges
    for event in merge_events.read() {
        for mut projection in projections.iter_mut() {
            if projection.identity_id == event.source_identity {
                projection.last_synced = event.merged_at;
            }
        }
    }
}

/// System to track workflow status changes
pub fn update_workflow_status_projection(
    mut started_events: EventReader<WorkflowStarted>,
    mut completed_events: EventReader<WorkflowCompleted>,
    mut timeout_events: EventReader<WorkflowTimedOut>,
    mut workflows: Query<&mut IdentityWorkflow>,
) {
    // Handle workflow starts
    for _event in started_events.read() {
        // Workflow entities are created by the workflow systems
    }

    // Handle workflow completions
    for event in completed_events.read() {
        for mut workflow in workflows.iter_mut() {
            if workflow.workflow_id == event.workflow_id {
                workflow.status = event.final_status.clone();
                workflow.completed_at = Some(event.completed_at);
            }
        }
    }

    // Handle workflow timeouts
    for event in timeout_events.read() {
        for mut workflow in workflows.iter_mut() {
            if workflow.workflow_id == event.workflow_id {
                workflow.status = WorkflowStatus::Failed("Step timeout".to_string());
                // Update the current step's status
                let current_step_id = workflow.current_step.clone();
                if let Some(ref step_id) = current_step_id {
                    if let Some(step) = workflow.steps.iter_mut().find(|s| &s.step_id == step_id) {
                        step.status = StepStatus::Failed;
                    }
                }
            }
        }
    }
}
