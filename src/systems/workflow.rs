//! Identity workflow systems

use bevy_ecs::prelude::*;
use crate::{
    components::*,
    events::*,
    commands::*,
};

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
        let identity_exists = identities.iter()
            .any(|e| e.identity_id == event.identity_id);

        if !identity_exists {
            continue;
        }

        // Check for existing workflows of same type
        let existing_workflow = workflows.iter()
            .any(|w| w.identity_id == event.identity_id &&
                    w.workflow_type == event.workflow_type &&
                    matches!(w.current_state.status, 
                            WorkflowStatus::InProgress | 
                            WorkflowStatus::WaitingForInput |
                            WorkflowStatus::WaitingForApproval));

        if existing_workflow {
            // Cannot start duplicate workflow
            continue;
        }

        let workflow_id = uuid::Uuid::new_v4();

        // Spawn workflow entity
        commands.spawn((
            IdentityWorkflow {
                workflow_id,
                identity_id: event.identity_id,
                workflow_type: event.workflow_type.clone(),
                current_state: WorkflowState {
                    step_id: "start".to_string(),
                    status: WorkflowStatus::InProgress,
                    entered_at: chrono::Utc::now(),
                    data: event.context.clone(),
                },
                started_at: chrono::Utc::now(),
                started_by: event.started_by,
                deadline: None,
                context: event.context.clone(),
            },
        ));

        // Emit started event
        started_events.send(WorkflowStarted {
            workflow_id,
            identity_id: event.identity_id,
            workflow_type: event.workflow_type.clone(),
            started_by: event.started_by,
            started_at: chrono::Utc::now(),
        });
    }
}

/// System to process workflow steps
pub fn process_workflow_step_system(
    mut commands: Commands,
    mut events: EventReader<CompleteWorkflowStepCommand>,
    mut step_events: EventWriter<WorkflowStepCompleted>,
    mut workflows: Query<&mut IdentityWorkflow>,
    transitions: Query<&WorkflowTransition>,
) {
    for event in events.read() {
        for mut workflow in workflows.iter_mut() {
            if workflow.workflow_id == event.workflow_id &&
               workflow.current_state.step_id == event.step_id {
                
                // Find applicable transition
                let next_step = transitions.iter()
                    .find(|t| t.from_step == event.step_id)
                    .map(|t| t.to_step.clone());

                // Update workflow state
                if let Some(next_step_id) = &next_step {
                    workflow.current_state = WorkflowState {
                        step_id: next_step_id.clone(),
                        status: WorkflowStatus::InProgress,
                        entered_at: chrono::Utc::now(),
                        data: event.result.clone(),
                    };
                }

                // Emit step completed event
                step_events.send(WorkflowStepCompleted {
                    workflow_id: event.workflow_id,
                    step_id: event.step_id.clone(),
                    next_step_id: next_step,
                    completed_by: event.completed_by,
                    completed_at: chrono::Utc::now(),
                });
            }
        }
    }
}

/// System to complete workflows
pub fn complete_workflow_system(
    mut commands: Commands,
    mut completed_events: EventWriter<WorkflowCompleted>,
    mut workflows: Query<(Entity, &mut IdentityWorkflow)>,
) {
    for (entity, mut workflow) in workflows.iter_mut() {
        // Check if workflow is in a terminal state
        match workflow.current_state.status {
            WorkflowStatus::Completed | WorkflowStatus::Failed | WorkflowStatus::Cancelled => {
                // Emit completed event
                completed_events.send(WorkflowCompleted {
                    workflow_id: workflow.workflow_id,
                    identity_id: workflow.identity_id,
                    workflow_type: workflow.workflow_type.clone(),
                    final_status: workflow.current_state.status,
                    completed_at: chrono::Utc::now(),
                });

                // Create workflow history
                let duration = chrono::Utc::now() - workflow.started_at;
                commands.spawn(WorkflowHistory {
                    workflow_id: workflow.workflow_id,
                    step_transitions: vec![], // Would be populated from events
                    total_duration: Some(duration),
                    completion_data: Some(workflow.current_state.data.clone()),
                });

                // Remove active workflow
                commands.entity(entity).despawn();
            }
            _ => {}
        }
    }
}

/// System to handle workflow timeouts
pub fn timeout_workflow_system(
    mut workflows: Query<&mut IdentityWorkflow>,
    steps: Query<&WorkflowStep>,
    time: Res<bevy_time::Time>,
) {
    let now = chrono::Utc::now();

    for mut workflow in workflows.iter_mut() {
        // Check if workflow has deadline
        if let Some(deadline) = workflow.deadline {
            if now > deadline {
                workflow.current_state.status = WorkflowStatus::Failed;
                continue;
            }
        }

        // Check step timeout
        if let Some(step) = steps.iter()
            .find(|s| s.step_id == workflow.current_state.step_id) {
            
            if let Some(timeout) = step.timeout {
                let elapsed = now - workflow.current_state.entered_at;
                if elapsed > timeout {
                    // Timeout occurred
                    match workflow.current_state.status {
                        WorkflowStatus::WaitingForInput | WorkflowStatus::WaitingForApproval => {
                            workflow.current_state.status = WorkflowStatus::Failed;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
} 