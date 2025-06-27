//! Identity workflow components

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identity workflow instance
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityWorkflow {
    pub workflow_id: Uuid,
    pub identity_id: Uuid,
    pub workflow_type: WorkflowType,
    pub status: WorkflowStatus,
    pub current_step: Option<String>,
    pub steps: Vec<WorkflowStep>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Type of identity workflow
#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkflowType {
    Verification,
    Onboarding,
    Recovery,
    Migration,
    Custom(String),
}

/// Status of a workflow
#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkflowStatus {
    NotStarted,
    InProgress,
    Paused,
    Completed,
    Failed(String),
    Cancelled,
    WaitingForInput,
    WaitingForApproval,
}

/// Workflow step definition
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub step_id: String,
    pub step_type: StepType,
    pub status: StepStatus,
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
    pub timeout_seconds: Option<u64>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Type of workflow step
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StepType {
    Manual,
    Automated,
    Approval,
    Verification,
    Notification,
}

/// Status of a workflow step
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    Active,
    Completed,
    Failed,
    Skipped,
}

/// Workflow transition between steps
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTransition {
    pub from_step: String,
    pub to_step: String,
    pub condition: TransitionCondition,
    pub metadata: serde_json::Value,
}

/// Condition for workflow transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionCondition {
    /// Always transition
    Always,
    /// Transition on success
    OnSuccess,
    /// Transition on failure
    OnFailure,
    /// Transition based on field value
    FieldEquals { field: String, value: serde_json::Value },
    /// Transition based on expression
    Expression { expr: String },
    /// Manual decision required
    Manual,
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
    pub transitioned_by: Option<Uuid>,
    pub reason: String,
    pub data: serde_json::Value,
} 