//! Identity projection components

use bevy::ecs::prelude::*;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identity projection to other domains
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityProjection {
    pub identity_id: Uuid,
    pub projection_type: ProjectionType,
    pub target_domain: String,
    pub sync_status: ProjectionSyncStatus,
    pub last_sync: chrono::DateTime<chrono::Utc>,
    pub last_synced: chrono::DateTime<chrono::Utc>, // Alias for compatibility
}

/// Status of projection synchronization
#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProjectionSyncStatus {
    Synced,
    Pending,
    OutOfSync,
    Failed(String),
}

/// Type of projection to another domain
#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProjectionType {
    Primary,
    Secondary,
    Master,
    Replica,
    Cached,
}

/// Cross-domain reference to external entity
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CrossDomainReference {
    pub domain: String,
    pub entity_type: String,
    pub entity_id: String,
    pub reference_type: ReferenceType,
}

/// Type of cross-domain reference
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReferenceType {
    Primary,
    Secondary,
    Linked,
    Derived,
}

/// Context for projection operations
#[derive(Component, Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectionContext {
    pub source_domain: String,
    pub target_domain: String,
    pub sync_interval: Duration,
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
    pub sync_errors: u32,
    pub metadata: serde_json::Value,
}

/// View of an identity for queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityView {
    pub identity_id: Uuid,
    pub identity_type: crate::components::IdentityType,
    pub status: crate::components::IdentityStatus,
    pub verification_level: crate::components::VerificationLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
    Mask {
        attribute: String,
        mask_pattern: String,
    },
    /// Hash the value
    Hash { attribute: String },
    /// Truncate the value
    Truncate {
        attribute: String,
        max_length: usize,
    },
    /// Replace with placeholder
    Placeholder {
        attribute: String,
        placeholder: String,
    },
    /// Custom transformation
    Custom {
        attribute: String,
        transform: String,
    },
}

/// Synchronization error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncError {
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub error_type: String,
    pub message: String,
    pub retry_count: u32,
}
