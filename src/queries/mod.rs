//! Query operations for the Identity domain
//!
//! This module provides read-only query operations that don't modify state.

use bevy_ecs::prelude::*;
use crate::{
    components::*,
    aggregate::{IdentityAggregate, AggregateState},
};

/// Query to find an identity by ID
pub fn find_identity_by_id(
    world: &mut World,
    identity_id: IdentityId,
) -> Option<IdentityView> {
    let mut query = world.query::<(
        &IdentityEntity,
        &IdentityVerification,
        &IdentityMetadata,
    )>();

    for (identity, verification, metadata) in query.iter(world) {
        if identity.identity_id == identity_id {
            return Some(IdentityView {
                identity: identity.clone(),
                verification: verification.clone(),
                metadata: metadata.clone(),
            });
        }
    }

    None
}

/// Query to find identities by type
pub fn find_identities_by_type(
    world: &mut World,
    identity_type: IdentityType,
) -> Vec<IdentityView> {
    let mut results = Vec::new();
    let mut query = world.query::<(
        &IdentityEntity,
        &IdentityVerification,
        &IdentityMetadata,
    )>();

    for (identity, verification, metadata) in query.iter(world) {
        if identity.identity_type == identity_type {
            results.push(IdentityView {
                identity: identity.clone(),
                verification: verification.clone(),
                metadata: metadata.clone(),
            });
        }
    }

    results
}

/// Query to find relationships for an identity
pub fn find_relationships_for_identity(
    world: &mut World,
    identity_id: IdentityId,
) -> Vec<IdentityRelationship> {
    let mut results = Vec::new();
    let mut query = world.query::<&IdentityRelationship>();

    for relationship in query.iter(world) {
        if relationship.from_identity == identity_id || 
           relationship.to_identity == identity_id {
            results.push(relationship.clone());
        }
    }

    results
}

/// Query to find active workflows for an identity
pub fn find_active_workflows_for_identity(
    world: &mut World,
    identity_id: IdentityId,
) -> Vec<IdentityWorkflow> {
    let mut results = Vec::new();
    let mut query = world.query::<&IdentityWorkflow>();

    for workflow in query.iter(world) {
        if workflow.identity_id == identity_id &&
           matches!(workflow.current_state.status,
                   WorkflowStatus::InProgress |
                   WorkflowStatus::WaitingForInput |
                   WorkflowStatus::WaitingForApproval) {
            results.push(workflow.clone());
        }
    }

    results
}

/// Query to get aggregate state for an identity
pub fn get_aggregate_state(
    world: &mut World,
    identity_id: IdentityId,
) -> Option<AggregateState> {
    // Find identity
    let mut identity_query = world.query::<(&IdentityEntity, &IdentityVerification)>();
    let (identity, verification) = identity_query.iter(world)
        .find(|(e, _)| e.identity_id == identity_id)?;

    // Find relationships
    let relationships = find_relationships_for_identity(world, identity_id);

    // Find workflows
    let mut workflow_query = world.query::<&IdentityWorkflow>();
    let workflows: Vec<_> = workflow_query.iter(world)
        .filter(|w| w.identity_id == identity_id)
        .cloned()
        .collect();

    Some(IdentityAggregate::calculate_state(
        identity,
        &relationships,
        &workflows,
        verification,
    ))
}

/// Query to find identities with specific verification level
pub fn find_identities_by_verification_level(
    world: &mut World,
    min_level: VerificationLevel,
) -> Vec<IdentityView> {
    let mut results = Vec::new();
    let mut query = world.query::<(
        &IdentityEntity,
        &IdentityVerification,
        &IdentityMetadata,
    )>();

    for (identity, verification, metadata) in query.iter(world) {
        if verification.verification_level >= min_level {
            results.push(IdentityView {
                identity: identity.clone(),
                verification: verification.clone(),
                metadata: metadata.clone(),
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

/// View model for identity queries
#[derive(Debug, Clone)]
pub struct IdentityView {
    pub identity: IdentityEntity,
    pub verification: IdentityVerification,
    pub metadata: IdentityMetadata,
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

            let next = if relationship.from_identity == current {
                Some(relationship.to_identity)
            } else if relationship.to_identity == current {
                Some(relationship.from_identity)
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