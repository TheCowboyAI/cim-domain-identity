//! Identity relationship systems

use bevy_ecs::prelude::*;
use bevy_time::Time;
use crate::{
    components::*,
    events::*,
    commands::*,
    aggregate::IdentityAggregate,
    IdentityError,
};

/// System to establish relationships between identities
pub fn establish_relationship_system(
    mut commands: Commands,
    mut events: EventReader<EstablishRelationshipCommand>,
    mut established_events: EventWriter<RelationshipEstablished>,
    identities: Query<&IdentityEntity>,
    existing_relationships: Query<&IdentityRelationship>,
) {
    for event in events.read() {
        // Find both identities
        let from_identity = identities.iter()
            .find(|e| e.identity_id == event.from_identity);
        let to_identity = identities.iter()
            .find(|e| e.identity_id == event.to_identity);

        if let (Some(from), Some(to)) = (from_identity, to_identity) {
            // Validate through aggregate
            match IdentityAggregate::validate_relationship(from, to, &event.relationship_type) {
                Ok(_) => {
                    // Check for existing relationship
                    let exists = existing_relationships.iter()
                        .any(|r| r.from_identity == event.from_identity &&
                                r.to_identity == event.to_identity &&
                                r.relationship_type == event.relationship_type);

                    if !exists {
                        let relationship_id = RelationshipId::new();

                        // Create relationship entity
                        commands.spawn(IdentityRelationship {
                            relationship_id,
                            from_identity: event.from_identity,
                            to_identity: event.to_identity,
                            relationship_type: event.relationship_type.clone(),
                            rules: RelationshipRules {
                                can_delegate: event.can_delegate,
                                can_revoke: event.can_revoke,
                                expires_at: event.expires_at,
                                max_depth: event.max_depth,
                            },
                            established_at: chrono::Utc::now(),
                            established_by: event.established_by,
                            metadata: event.metadata.clone(),
                        });

                        // Emit established event
                        established_events.send(RelationshipEstablished {
                            relationship_id,
                            from_identity: event.from_identity,
                            to_identity: event.to_identity,
                            relationship_type: event.relationship_type.clone(),
                            established_by: event.established_by,
                            established_at: chrono::Utc::now(),
                        });
                    }
                }
                Err(e) => {
                    eprintln!("Failed to establish relationship: {}", e);
                }
            }
        }
    }
}

/// System to validate existing relationships
pub fn validate_relationship_system(
    mut commands: Commands,
    mut validated_events: EventWriter<RelationshipValidated>,
    mut expired_events: EventWriter<RelationshipExpired>,
    relationships: Query<(Entity, &IdentityRelationship)>,
    identities: Query<&IdentityEntity>,
    time: Res<Time>,
) {
    let now = chrono::Utc::now();

    for (entity, relationship) in relationships.iter() {
        let mut is_valid = true;
        let mut reason = None;

        // Check expiration
        if let Some(expires_at) = relationship.rules.expires_at {
            if now >= expires_at {
                is_valid = false;
                reason = Some("Relationship expired".to_string());
            }
        }

        // Check if both identities still exist and are active
        let from_active = identities.iter()
            .find(|e| e.identity_id == relationship.from_identity)
            .map(|e| matches!(e.status, IdentityStatus::Active))
            .unwrap_or(false);

        let to_active = identities.iter()
            .find(|e| e.identity_id == relationship.to_identity)
            .map(|e| matches!(e.status, IdentityStatus::Active))
            .unwrap_or(false);

        if !from_active || !to_active {
            is_valid = false;
            reason = Some("One or both identities are no longer active".to_string());
        }

        if is_valid {
            // Emit validation event
            validated_events.send(RelationshipValidated {
                relationship_id: relationship.relationship_id,
                validated_at: now,
            });
        } else {
            // Remove invalid relationship
            commands.entity(entity).despawn();

            // Emit expired event
            expired_events.send(RelationshipExpired {
                relationship_id: relationship.relationship_id,
                from_identity: relationship.from_identity,
                to_identity: relationship.to_identity,
                relationship_type: relationship.relationship_type.clone(),
                expired_at: now,
                reason,
            });
        }
    }
}

/// System to traverse relationship graph
pub fn traverse_relationships_system(
    mut events: EventReader<TraverseRelationshipsCommand>,
    mut result_events: EventWriter<RelationshipTraversalResult>,
    relationships: Query<&IdentityRelationship>,
) {
    for event in events.read() {
        let mut visited = std::collections::HashSet::new();
        let mut paths = Vec::new();
        let mut queue = std::collections::VecDeque::new();

        // Start traversal
        queue.push_back((event.from_identity, vec![event.from_identity], 0));

        while let Some((current, path, depth)) = queue.pop_front() {
            if let Some(max_depth) = event.max_depth {
                if depth >= max_depth {
                    continue;
                }
            }

            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            // Find relationships from current identity
            for relationship in relationships.iter() {
                if relationship.from_identity == current {
                    // Check if relationship type matches filter
                    if let Some(ref types) = event.relationship_types {
                        if !types.contains(&relationship.relationship_type) {
                            continue;
                        }
                    }

                    // Check max depth for this relationship
                    if let Some(max) = relationship.rules.max_depth {
                        if depth >= max as u32 {
                            continue;
                        }
                    }

                    let next = relationship.to_identity;
                    if !visited.contains(&next) {
                        let mut new_path = path.clone();
                        new_path.push(next);

                        // Check if we've reached the target
                        if Some(next) == event.to_identity {
                            paths.push(new_path.clone());
                        }

                        queue.push_back((next, new_path, depth + 1));
                    }
                }
            }
        }

        // Emit result event
        result_events.send(RelationshipTraversalResult {
            from_identity: event.from_identity,
            to_identity: event.to_identity,
            paths_found: paths,
            total_visited: visited.len(),
        });
    }
}

/// System to expire relationships based on rules
pub fn expire_relationships_system(
    mut commands: Commands,
    mut expired_events: EventWriter<RelationshipExpired>,
    relationships: Query<(Entity, &IdentityRelationship)>,
) {
    let now = chrono::Utc::now();

    for (entity, relationship) in relationships.iter() {
        if let Some(expires_at) = relationship.rules.expires_at {
            if now >= expires_at {
                // Remove expired relationship
                commands.entity(entity).despawn();

                // Emit expired event
                expired_events.send(RelationshipExpired {
                    relationship_id: relationship.relationship_id,
                    from_identity: relationship.from_identity,
                    to_identity: relationship.to_identity,
                    relationship_type: relationship.relationship_type.clone(),
                    expired_at: now,
                    reason: Some("Relationship reached expiration date".to_string()),
                });
            }
        }
    }
} 