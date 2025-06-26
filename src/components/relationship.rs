//! Identity relationship components

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use super::IdentityId;

/// Relationship between two identities
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IdentityRelationship {
    pub relationship_id: uuid::Uuid,
    pub from_identity: IdentityId,
    pub to_identity: IdentityId,
    pub relationship_type: RelationshipType,
    pub established_at: chrono::DateTime<chrono::Utc>,
    pub established_by: Option<IdentityId>,
    pub metadata: serde_json::Value,
}

/// Type of relationship between identities
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipType {
    /// Person belongs to organization
    MemberOf {
        role: String,
        department: Option<String>,
    },
    /// Person manages another person
    ManagerOf,
    /// Person reports to another person
    ReportsTo,
    /// Identity is a parent of another (for hierarchies)
    ParentOf,
    /// Identity is a child of another
    ChildOf,
    /// Identity owns another identity
    OwnerOf,
    /// Identity is owned by another
    OwnedBy,
    /// Identity delegates authority to another
    DelegatesTo {
        permissions: Vec<String>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    },
    /// Identity acts on behalf of another
    ActsFor {
        scope: Vec<String>,
        valid_until: Option<chrono::DateTime<chrono::Utc>>,
    },
    /// Custom relationship type
    Custom {
        relationship_name: String,
        bidirectional: bool,
    },
}

/// Relationship validation rules
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipRules {
    pub allowed_relationships: Vec<AllowedRelationship>,
    pub max_relationships: Option<usize>,
    pub requires_approval: bool,
    pub auto_expire_days: Option<u32>,
}

/// Allowed relationship configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowedRelationship {
    pub from_type: IdentityTypePattern,
    pub to_type: IdentityTypePattern,
    pub relationship_types: Vec<RelationshipType>,
}

/// Pattern for matching identity types in rules
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityTypePattern {
    Any,
    Person,
    Organization,
    System,
    External,
}

/// Relationship graph traversal component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipGraph {
    pub root_identity: IdentityId,
    pub max_depth: Option<u32>,
    pub relationship_filter: Option<Vec<RelationshipType>>,
    pub include_inactive: bool,
}

/// Cached relationship path for efficient traversal
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipPath {
    pub from: IdentityId,
    pub to: IdentityId,
    pub path: Vec<IdentityId>,
    pub relationships: Vec<uuid::Uuid>,
    pub total_distance: u32,
    pub cached_at: chrono::DateTime<chrono::Utc>,
} 