//! Relationship components for the Identity domain

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::IdentityId;

/// Unique identifier for a relationship
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RelationshipId(pub uuid::Uuid);

impl RelationshipId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Default for RelationshipId {
    fn default() -> Self {
        Self::new()
    }
}

/// Component representing a relationship between identities
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityRelationship {
    pub relationship_id: RelationshipId,
    pub from_identity: IdentityId,
    pub to_identity: IdentityId,
    pub relationship_type: RelationshipType,
    pub rules: RelationshipRules,
    pub established_at: chrono::DateTime<chrono::Utc>,
    pub established_by: IdentityId,
    pub metadata: serde_json::Value,
}

/// Types of relationships between identities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationshipType {
    /// Person is member of organization
    MemberOf {
        role: String,
        department: Option<String>,
    },
    /// Person manages another person
    ManagerOf,
    /// Person reports to another person
    ReportsTo,
    /// Identity owns another identity (e.g., org owns subsidiary)
    Owns {
        ownership_percentage: Option<f32>,
    },
    /// Identity is partner with another
    PartnerWith {
        partnership_type: String,
    },
    /// Identity delegates authority to another
    DelegatesTo {
        permissions: Vec<String>,
    },
    /// Identity acts on behalf of another
    ActsFor {
        scope: Vec<String>,
    },
    /// Identity is associated with another
    AssociatedWith {
        association_type: String,
    },
    /// Custom relationship type
    Custom {
        relationship_name: String,
        attributes: serde_json::Value,
    },
}

/// Rules governing a relationship
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipRules {
    /// Whether this relationship can be delegated
    pub can_delegate: bool,
    /// Whether this relationship can be revoked
    pub can_revoke: bool,
    /// When this relationship expires
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Maximum depth for transitive relationships
    pub max_depth: Option<u8>,
}

/// Component for caching relationship paths
#[derive(Component, Debug, Clone)]
pub struct RelationshipPath {
    pub from: IdentityId,
    pub to: IdentityId,
    pub path: Vec<IdentityId>,
    pub relationships: Vec<RelationshipId>,
    pub total_distance: u32,
    pub cached_at: chrono::DateTime<chrono::Utc>,
}

/// Component for relationship graph queries
#[derive(Component, Debug, Clone)]
pub struct RelationshipGraph {
    pub root_identity: IdentityId,
    pub max_depth: Option<u32>,
    pub relationship_filter: Option<Vec<RelationshipType>>,
    pub include_inactive: bool,
} 