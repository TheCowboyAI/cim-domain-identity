//! Relationship components for the Identity domain

use bevy::ecs::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub relationship_id: Uuid,
    pub source_identity: Uuid,
    pub target_identity: Uuid,
    pub relationship_type: RelationshipType,
    pub rules: RelationshipRules,
    pub established_at: chrono::DateTime<chrono::Utc>,
    pub established_by: Option<Uuid>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Type of relationship between identities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationshipType {
    /// Identity owns another identity
    Owns,
    /// Identity manages another identity
    Manages,
    /// Identity is a member of another identity
    MemberOf,
    /// Identity delegates to another identity
    Delegates,
    /// Identity trusts another identity
    Trusts,
    /// Custom relationship
    Custom(String),
}

/// Constraints on a relationship
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationshipConstraint {
    MaxCount(usize),
    MinCount(usize),
    RequiredVerificationLevel(crate::components::VerificationLevel),
    MutuallyExclusive(Vec<RelationshipType>),
    RequiresApproval,
    TimeBasedExpiry(chrono::Duration),
}

/// Rules governing relationships
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipRules {
    pub allowed_types: Vec<RelationshipType>,
    pub constraints: Vec<RelationshipConstraint>,
    pub require_mutual_consent: bool,
    pub allow_multiple: bool,
}

/// Graph of identity relationships
#[derive(Component, Debug, Clone, Default)]
pub struct RelationshipGraph {
    pub identity_id: Uuid,
    pub direct_relationships: Vec<Uuid>,
    pub relationship_count: usize,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}
