//! Query operations for the Identity domain
//!
//! This module provides read-only query operations that don't modify state.

use bevy_ecs::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    aggregate::{AggregateState, IdentityAggregate},
    components::{
        ClaimType, IdentityClaim, IdentityEntity, IdentityId, IdentityMetadata,
        IdentityRelationship, IdentityStatus, IdentityType, IdentityVerification, IdentityWorkflow,
        ProjectionType, RelationshipId, RelationshipRules, RelationshipType, VerificationLevel,
        WorkflowStatus, WorkflowType,
    },
};

/// Query to find an identity by ID
#[derive(Debug)]
pub struct FindIdentityByIdQuery {
    pub identity_id: IdentityId,
}

/// Query to find identities by type
#[derive(Debug)]
pub struct FindIdentitiesByTypeQuery {
    pub identity_type: IdentityType,
}

/// Query to find relationships by identity
#[derive(Debug)]
pub struct FindRelationshipsByIdentityQuery {
    pub identity_id: IdentityId,
    pub include_incoming: bool,
    pub include_outgoing: bool,
}

/// Query to find active workflows
#[derive(Debug)]
pub struct FindActiveWorkflowsQuery {
    pub identity_id: Option<IdentityId>,
    pub workflow_type: Option<WorkflowType>,
}

/// Query to get identity verification status
#[derive(Debug)]
pub struct GetIdentityVerificationStatusQuery {
    pub identity_id: IdentityId,
}

/// Query to get identity projections
#[derive(Debug)]
pub struct GetIdentityProjectionsQuery {
    pub identity_id: IdentityId,
    pub projection_type: Option<ProjectionType>,
}

/// Query to find workflows for an identity
pub struct FindWorkflowsQuery {
    pub identity_id: IdentityId,
    pub workflow_type: Option<WorkflowType>,
    pub status_filter: Option<WorkflowStatus>,
}

/// Query for identity details including relationships and workflows
pub fn find_identity_details(world: &mut World, identity_id: Uuid) -> Option<IdentityDetails> {
    // First, get the identity and verification
    let mut identity_query = world.query::<(&IdentityEntity, &IdentityVerification)>();
    let identity_data = identity_query
        .iter(world)
        .find(|(entity, _)| entity.identity_id == identity_id)
        .map(|(e, v)| (e.clone(), v.clone()));

    let (identity, verification) = identity_data?;

    // Then get relationships separately
    let relationships = {
        let mut rel_query = world.query::<&IdentityRelationship>();
        rel_query
            .iter(world)
            .filter(|rel| rel.source_identity == identity_id || rel.target_identity == identity_id)
            .cloned()
            .collect::<Vec<_>>()
    };

    // Get workflows
    let mut workflow_query = world.query::<&IdentityWorkflow>();
    let active_workflows = workflow_query
        .iter(world)
        .filter(|w| w.identity_id == identity_id)
        .cloned()
        .collect();

    Some(IdentityDetails {
        identity,
        verification,
        relationships,
        active_workflows,
    })
}

/// Query to find relationships for an identity
pub fn find_relationships_for_identity(
    world: &mut World,
    identity_id: Uuid,
) -> Vec<RelationshipView> {
    let mut relationship_query = world.query::<&IdentityRelationship>();

    relationship_query
        .iter(world)
        .filter(|rel| rel.source_identity == identity_id || rel.target_identity == identity_id)
        .map(|rel| {
            let (source_id, target_id) = if rel.source_identity == identity_id {
                (identity_id, rel.target_identity)
            } else {
                (rel.target_identity, identity_id)
            };

            RelationshipView {
                relationship_id: rel.relationship_id,
                from_identity: source_id,
                to_identity: target_id,
                relationship_type: rel.relationship_type.clone(),
                established_at: rel.established_at,
            }
        })
        .collect()
}

/// Query to find active workflows for an identity
pub fn find_active_workflows_for_identity(
    world: &mut World,
    identity_id: IdentityId,
) -> Vec<IdentityWorkflow> {
    let mut results = Vec::new();
    let mut query = world.query::<&IdentityWorkflow>();

    for workflow in query.iter(world) {
        if workflow.identity_id == identity_id
            && matches!(
                workflow.status,
                WorkflowStatus::InProgress
                    | WorkflowStatus::WaitingForInput
                    | WorkflowStatus::WaitingForApproval
            )
        {
            results.push(workflow.clone());
        }
    }

    results
}

/// Query to get aggregate state for an identity
pub fn get_aggregate_state(world: &mut World, identity_id: IdentityId) -> Option<AggregateState> {
    // Find identity
    let mut identity_query = world.query::<(&IdentityEntity, &IdentityVerification)>();
    let identity_data = identity_query
        .iter(world)
        .find(|(e, _)| e.identity_id == identity_id)
        .map(|(e, v)| (e.clone(), v.clone()))?;

    let (identity, verification) = identity_data;

    // Find relationships
    let mut relationship_query = world.query::<&IdentityRelationship>();
    let relationships: Vec<_> = relationship_query
        .iter(world)
        .filter(|r| r.source_identity == identity_id || r.target_identity == identity_id)
        .cloned()
        .collect();

    // Find workflows
    let mut workflow_query = world.query::<&IdentityWorkflow>();
    let workflows: Vec<_> = workflow_query
        .iter(world)
        .filter(|w| w.identity_id == identity_id)
        .cloned()
        .collect();

    Some(IdentityAggregate::calculate_state(
        &identity,
        &relationships,
        &workflows,
        &verification,
    ))
}

/// Query to find identities with specific verification level
pub fn find_identities_by_verification_level(
    world: &mut World,
    min_level: VerificationLevel,
) -> Vec<IdentityView> {
    let mut results = Vec::new();
    let mut query = world.query::<(&IdentityEntity, &IdentityVerification, &IdentityMetadata)>();

    for (identity, verification, metadata) in query.iter(world) {
        if verification.verification_level >= min_level {
            results.push(IdentityView {
                identity_id: identity.identity_id,
                identity_type: identity.identity_type,
                status: identity.status.clone(),
                verification_level: verification.verification_level,
                created_at: metadata.created_at,
                updated_at: metadata.updated_at,
            });
        }
    }

    results
}

/// Query to find identities by claim
pub fn find_identities_by_claim(
    world: &mut World,
    claim_type: ClaimType,
    value: &str,
) -> Vec<IdentityId> {
    let mut results = Vec::new();
    let mut query = world.query::<(&IdentityEntity, &IdentityClaim)>();

    for (identity, claim) in query.iter(world) {
        if claim.claim_type == claim_type && claim.value == value {
            results.push(identity.identity_id);
        }
    }

    results
}

/// Read-only view of an identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityView {
    pub identity_id: Uuid,
    pub identity_type: IdentityType,
    pub status: IdentityStatus,
    pub verification_level: VerificationLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Query to traverse relationship graph
pub fn traverse_relationship_graph(
    world: &mut World,
    root: IdentityId,
    max_depth: Option<u32>,
    relationship_filter: Option<Vec<RelationshipType>>,
) -> RelationshipGraphResult {
    let mut visited = std::collections::HashSet::new();
    let mut paths = Vec::new();
    let mut queue = std::collections::VecDeque::new();

    // Start traversal
    queue.push_back((root, vec![root], 0));

    let mut relationship_query = world.query::<&IdentityRelationship>();
    let relationships: Vec<_> = relationship_query.iter(world).cloned().collect();

    while let Some((current, path, depth)) = queue.pop_front() {
        if let Some(max) = max_depth {
            if depth >= max {
                continue;
            }
        }

        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        // Find connected identities
        for relationship in &relationships {
            // Apply filter if specified
            if let Some(ref filter) = relationship_filter {
                if !filter.contains(&relationship.relationship_type) {
                    continue;
                }
            }

            let next = if relationship.source_identity == current {
                Some(relationship.target_identity)
            } else if relationship.target_identity == current {
                Some(relationship.source_identity)
            } else {
                None
            };

            if let Some(next_id) = next {
                if !visited.contains(&next_id) {
                    let mut new_path = path.clone();
                    new_path.push(next_id);
                    paths.push(new_path.clone());
                    queue.push_back((next_id, new_path, depth + 1));
                }
            }
        }
    }

    RelationshipGraphResult {
        root,
        paths,
        visited_count: visited.len(),
    }
}

/// Result of relationship graph traversal
#[derive(Debug, Clone)]
pub struct RelationshipGraphResult {
    pub root: IdentityId,
    pub paths: Vec<Vec<IdentityId>>,
    pub visited_count: usize,
}

/// System to find identity by ID
pub fn find_identity_by_id(
    world: &mut World,
    query: &FindIdentityByIdQuery,
) -> Option<IdentityView> {
    world
        .query_filtered::<(&IdentityEntity, &IdentityMetadata, &IdentityVerification), ()>()
        .iter(world)
        .find(|(entity, _, _)| entity.identity_id == query.identity_id)
        .map(|(entity, metadata, verification)| IdentityView {
            identity_id: entity.identity_id,
            identity_type: entity.identity_type,
            status: entity.status.clone(),
            verification_level: verification.verification_level,
            created_at: metadata.created_at,
            updated_at: metadata.updated_at,
        })
}

/// System to find identities by type
pub fn find_identities_by_type(
    world: &mut World,
    query: &FindIdentitiesByTypeQuery,
) -> Vec<IdentityView> {
    world
        .query_filtered::<(&IdentityEntity, &IdentityMetadata, &IdentityVerification), ()>()
        .iter(world)
        .filter(|(entity, _, _)| entity.identity_type == query.identity_type)
        .map(|(entity, metadata, verification)| IdentityView {
            identity_id: entity.identity_id,
            identity_type: entity.identity_type,
            status: entity.status.clone(),
            verification_level: verification.verification_level,
            created_at: metadata.created_at,
            updated_at: metadata.updated_at,
        })
        .collect()
}

/// System to find relationships for an identity
pub fn find_relationships_by_identity(
    world: &mut World,
    query: &FindRelationshipsByIdentityQuery,
) -> Vec<RelationshipView> {
    world
        .query_filtered::<&IdentityRelationship, ()>()
        .iter(world)
        .filter(|relationship| {
            (query.include_outgoing && relationship.source_identity == query.identity_id)
                || (query.include_incoming && relationship.target_identity == query.identity_id)
        })
        .map(|relationship| RelationshipView {
            relationship_id: relationship.relationship_id,
            from_identity: relationship.source_identity,
            to_identity: relationship.target_identity,
            relationship_type: relationship.relationship_type.clone(),
            established_at: relationship.established_at,
        })
        .collect()
}

/// View model for relationship queries
#[derive(Debug, Clone)]
pub struct RelationshipView {
    pub relationship_id: RelationshipId,
    pub from_identity: IdentityId,
    pub to_identity: IdentityId,
    pub relationship_type: RelationshipType,
    pub established_at: chrono::DateTime<chrono::Utc>,
}

pub fn find_by_status(world: &mut World, status: IdentityStatus) -> Vec<IdentityView> {
    world
        .query_filtered::<(&IdentityEntity, &IdentityVerification, &IdentityMetadata), ()>()
        .iter(world)
        .filter(|(entity, _, _)| entity.status == status)
        .map(|(entity, verification, metadata)| IdentityView {
            identity_id: entity.identity_id,
            identity_type: entity.identity_type,
            status: entity.status.clone(),
            verification_level: verification.verification_level,
            created_at: metadata.created_at,
            updated_at: metadata.updated_at,
        })
        .collect()
}

pub fn find_by_verification_level(
    world: &mut World,
    min_level: VerificationLevel,
) -> Vec<IdentityView> {
    world
        .query_filtered::<(&IdentityEntity, &IdentityVerification, &IdentityMetadata), ()>()
        .iter(world)
        .filter(|(_, verification, _)| verification.verification_level >= min_level)
        .map(|(entity, verification, metadata)| IdentityView {
            identity_id: entity.identity_id,
            identity_type: entity.identity_type,
            status: entity.status.clone(),
            verification_level: verification.verification_level,
            created_at: metadata.created_at,
            updated_at: metadata.updated_at,
        })
        .collect()
}

/// Detailed identity information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityDetails {
    pub identity: IdentityEntity,
    pub verification: IdentityVerification,
    pub relationships: Vec<IdentityRelationship>,
    pub active_workflows: Vec<IdentityWorkflow>,
}
