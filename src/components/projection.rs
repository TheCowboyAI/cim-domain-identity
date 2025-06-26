//! Identity projection components

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use super::IdentityId;

/// Identity projection to other domains
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityProjection {
    pub identity_id: IdentityId,
    pub projection_type: ProjectionType,
    pub target_id: uuid::Uuid,
    pub context: ProjectionContext,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_synced: chrono::DateTime<chrono::Utc>,
}

/// Type of projection to another domain
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectionType {
    /// Projection to person domain
    Person,
    /// Projection to organization domain
    Organization,
    /// Projection to security context (authentication)
    SecurityContext,
    /// Projection to policy domain
    Policy,
    /// Projection to workflow domain
    Workflow,
    /// Custom projection
    Custom(String),
}

/// Context for the projection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectionContext {
    pub domain: String,
    pub entity_type: String,
    pub attributes: serde_json::Value,
    pub sync_strategy: SyncStrategy,
}

/// Strategy for synchronizing projections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncStrategy {
    /// Real-time synchronization via events
    RealTime,
    /// Periodic batch synchronization
    Batch { interval_seconds: u64 },
    /// On-demand synchronization
    OnDemand,
    /// One-time projection (no sync)
    OneTime,
}

/// Cross-domain reference component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CrossDomainReference {
    pub source_domain: String,
    pub source_id: uuid::Uuid,
    pub target_domain: String,
    pub target_id: uuid::Uuid,
    pub reference_type: ReferenceType,
    pub bidirectional: bool,
}

/// Type of cross-domain reference
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReferenceType {
    /// Identity owns entity in other domain
    Owns,
    /// Identity is represented by entity in other domain
    RepresentedBy,
    /// Identity manages entity in other domain
    Manages,
    /// Identity is associated with entity
    AssociatedWith,
    /// Custom reference type
    Custom(String),
}

/// Identity view for different contexts
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityView {
    pub identity_id: IdentityId,
    pub view_type: ViewType,
    pub visible_attributes: Vec<String>,
    pub hidden_attributes: Vec<String>,
    pub transformations: Vec<AttributeTransformation>,
}

/// Type of identity view
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewType {
    /// Public view (minimal information)
    Public,
    /// Internal view (for same organization)
    Internal,
    /// Administrative view (full access)
    Admin,
    /// Self view (identity viewing themselves)
    Self_,
    /// Custom view
    Custom(String),
}

/// Transformation to apply to attributes in a view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttributeTransformation {
    /// Mask part of the value
    Mask { attribute: String, mask_pattern: String },
    /// Hash the value
    Hash { attribute: String },
    /// Truncate the value
    Truncate { attribute: String, max_length: usize },
    /// Replace with placeholder
    Placeholder { attribute: String, placeholder: String },
    /// Custom transformation
    Custom { attribute: String, transform: String },
}

/// Projection synchronization status
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ProjectionSyncStatus {
    pub projection_id: uuid::Uuid,
    pub last_sync_attempt: chrono::DateTime<chrono::Utc>,
    pub last_successful_sync: Option<chrono::DateTime<chrono::Utc>>,
    pub sync_errors: Vec<SyncError>,
    pub pending_changes: u32,
}

/// Synchronization error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncError {
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub error_type: String,
    pub message: String,
    pub retry_count: u32,
} 