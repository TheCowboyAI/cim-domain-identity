//! Identity workflow systems

use crate::{commands::*, components::*, events::*};
use bevy_ecs::prelude::*;
use tracing::trace;

/// System to start identity workflows
pub fn start_workflow_system(
    mut commands: Commands,
    mut events: EventReader<StartWorkflowCommand>,
    mut started_events: EventWriter<WorkflowStarted>,
    identities: Query<&IdentityEntity>,
    workflows: Query<&IdentityWorkflow>,
) {
    for event in events.read() {
        // Validate identity exists
        let identity_exists = identities
            .iter()
            .any(|e| e.identity_id == event.identity_id);

        if !identity_exists {
            continue;
        }

        // Check for existing workflows of same type
        let existing_workflow = workflows.iter().any(|w| {
            w.identity_id == event.identity_id
                && w.workflow_type == event.workflow_type
                && matches!(
                    w.status,
                    WorkflowStatus::InProgress
                        | WorkflowStatus::WaitingForInput
                        | WorkflowStatus::WaitingForApproval
                )
        });

        if existing_workflow {
            // Cannot start duplicate workflow
            continue;
        }

        let workflow_id = uuid::Uuid::new_v4();

        // Create new workflow
        let workflow = IdentityWorkflow {
            workflow_id,
            identity_id: event.identity_id,
            workflow_type: event.workflow_type.clone(),
            status: WorkflowStatus::NotStarted,
            current_step: None,
            steps: Vec::new(), // Workflow steps should be initialized based on workflow type
            started_at: Some(chrono::Utc::now()),
            completed_at: None,
        };

        // Spawn workflow entity
        commands.spawn((workflow,));

        // Emit started event
        started_events.write(WorkflowStarted {
            workflow_id,
            identity_id: event.identity_id,
            workflow_type: event.workflow_type.clone(),
            started_by: event.started_by,
            started_at: chrono::Utc::now(),
            context: event.context.clone(),
        });
    }
}

/// System to process workflow steps
pub fn process_workflow_step_system(
    mut commands: Commands,
    mut events: EventReader<CompleteWorkflowCommand>,
    mut workflows: Query<(Entity, &mut IdentityWorkflow)>,
    mut writer: EventWriter<WorkflowStepCompleted>,
) {
    for event in events.read() {
        for (entity, mut workflow) in workflows.iter_mut() {
            if workflow.workflow_id == *event.workflow_id.as_uuid() {
                // Find and update the current step
                let current_step_id = workflow.current_step.clone();
                if let Some(ref step_id) = current_step_id {
                    if let Some(step) = workflow.steps.iter_mut().find(|s| &s.step_id == step_id) {
                        // Mark step as completed
                        step.status = StepStatus::Completed;
                        step.completed_at = Some(chrono::Utc::now());

                        // Find next step
                        let next_step = workflow
                            .steps
                            .iter()
                            .find(|s| s.status == StepStatus::Pending)
                            .map(|s| s.step_id.clone());

                        // Update workflow state
                        if let Some(next_step_id) = &next_step {
                            workflow.status = WorkflowStatus::InProgress;
                            workflow.current_step = Some(next_step_id.clone());
                        }

                        // Emit step completed event
                        writer.write(WorkflowStepCompleted {
                            workflow_id: workflow.workflow_id,
                            identity_id: workflow.identity_id,
                            workflow_type: workflow.workflow_type.clone(),
                            step_id: step_id.clone(),
                            completed_at: chrono::Utc::now(),
                        });
                    }
                }
            }
        }
    }
}

/// System to complete workflows
pub fn complete_workflow_system(
    mut commands: Commands,
    mut completed_events: EventWriter<WorkflowCompleted>,
    mut workflows: Query<(Entity, &mut IdentityWorkflow)>,
    mut events: EventReader<CompleteWorkflowCommand>,
) {
    for event in events.read() {
        for (entity, mut workflow) in workflows.iter_mut() {
            if workflow.workflow_id == *event.workflow_id.as_uuid() {
                // Check if workflow can be completed
                if matches!(
                    workflow.status,
                    WorkflowStatus::Completed
                        | WorkflowStatus::Failed(_)
                        | WorkflowStatus::Cancelled
                ) {
                    continue;
                }

                // Emit completed event
                completed_events.write(WorkflowCompleted {
                    workflow_id: workflow.workflow_id,
                    identity_id: workflow.identity_id,
                    workflow_type: workflow.workflow_type.clone(),
                    final_status: workflow.status.clone(),
                    completed_at: chrono::Utc::now(),
                });

                // Remove active workflow
                commands.entity(entity).despawn();
            }
        }
    }
}

/// System to handle workflow timeouts
pub fn timeout_workflows_system(
    mut workflows: Query<&mut IdentityWorkflow>,
    time: Res<bevy_time::Time>,
) {
    // Use the time resource to get elapsed time since startup
    // In production, you might want to track actual wall-clock time
    let current_time = chrono::Utc::now();

    // Log system execution for debugging
    if time.delta_secs() > 0.0 {
        trace!(
            "Checking workflow timeouts, delta: {:.2}s",
            time.delta_secs()
        );
    }

    for mut workflow in workflows.iter_mut() {
        // Skip if not in progress
        if !matches!(workflow.status, WorkflowStatus::InProgress) {
            continue;
        }

        // Check current step timeout
        let current_step_id = workflow.current_step.clone();
        if let Some(ref step_id) = current_step_id {
            if let Some(step) = workflow
                .steps
                .iter_mut()
                .find(|s| &s.step_id == step_id && s.status == StepStatus::Active)
            {
                if let Some(timeout_seconds) = step.timeout_seconds {
                    if let Some(started_at) = step.started_at {
                        let elapsed = current_time - started_at;
                        if elapsed.num_seconds() > timeout_seconds as i64 {
                            // Timeout occurred
                            step.status = StepStatus::Failed;
                            workflow.status = WorkflowStatus::Failed("Step timeout".to_string());
                        }
                    }
                }
            }
        }
    }
}
