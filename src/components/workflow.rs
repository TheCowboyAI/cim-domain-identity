//! Identity workflow components

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use super::IdentityId;

/// Identity workflow instance
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityWorkflow {
    pub workflow_id: uuid::Uuid,
    pub identity_id: IdentityId,
    pub workflow_type: IdentityWorkflowType,
    pub current_state: WorkflowState,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub started_by: IdentityId,
    pub deadline: Option<chrono::DateTime<chrono::Utc>>,
    pub context: serde_json::Value,
}

/// Type of identity workflow
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityWorkflowType {
    /// Initial identity verification
    Verification,
    /// Onboarding a new person to organization
    PersonOnboarding,
    /// Onboarding a new organization
    OrganizationOnboarding,
    /// Merging duplicate identities
    IdentityMerge,
    /// Offboarding/archiving identity
    Offboarding,
    /// Password reset workflow
    PasswordReset,
    /// Multi-factor authentication setup
    MfaSetup,
    /// Identity recovery
    AccountRecovery,
    /// Permission change request
    PermissionChange,
    /// Custom workflow
    Custom(String),
}

/// Current state of a workflow
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowState {
    pub step_id: String,
    pub status: WorkflowStatus,
    pub entered_at: chrono::DateTime<chrono::Utc>,
    pub data: serde_json::Value,
}

/// Status of a workflow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    /// Workflow is active and progressing
    InProgress,
    /// Waiting for external input
    WaitingForInput,
    /// Waiting for approval
    WaitingForApproval,
    /// Workflow is paused
    Paused,
    /// Workflow completed successfully
    Completed,
    /// Workflow was cancelled
    Cancelled,
    /// Workflow failed
    Failed,
}

/// Workflow step definition
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub step_id: String,
    pub step_type: WorkflowStepType,
    pub name: String,
    pub description: String,
    pub required_inputs: Vec<String>,
    pub timeout: Option<chrono::Duration>,
    pub auto_complete: bool,
}

/// Type of workflow step
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStepType {
    /// Collect information from user
    DataCollection,
    /// Verify provided information
    Verification,
    /// Get approval from authorized identity
    Approval,
    /// Send notification
    Notification,
    /// Make automated decision
    Decision,
    /// Execute system action
    SystemAction,
    /// Wait for external event
    ExternalWait,
    /// Custom step type
    Custom(String),
}

/// Workflow transition between steps
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTransition {
    pub from_step: String,
    pub to_step: String,
    pub condition: TransitionCondition,
    pub actions: Vec<TransitionAction>,
}

/// Condition for workflow transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionCondition {
    /// Always transition
    Always,
    /// Transition on specific input value
    OnInput { field: String, value: serde_json::Value },
    /// Transition on approval
    OnApproval { approver_role: Option<String> },
    /// Transition on timeout
    OnTimeout,
    /// Custom condition
    Custom(String),
}

/// Action to perform during transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionAction {
    /// Send notification
    Notify { template: String, recipients: Vec<String> },
    /// Update identity attribute
    UpdateIdentity { updates: serde_json::Value },
    /// Create audit log
    AuditLog { message: String },
    /// Trigger another workflow
    TriggerWorkflow { workflow_type: IdentityWorkflowType },
    /// Custom action
    Custom { action: String, params: serde_json::Value },
}

/// Workflow history record
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowHistory {
    pub workflow_id: uuid::Uuid,
    pub step_transitions: Vec<StepTransition>,
    pub total_duration: Option<chrono::Duration>,
    pub completion_data: Option<serde_json::Value>,
}

/// Record of a step transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepTransition {
    pub from_step: String,
    pub to_step: String,
    pub transitioned_at: chrono::DateTime<chrono::Utc>,
    pub transitioned_by: Option<IdentityId>,
    pub reason: String,
    pub data: serde_json::Value,
} 