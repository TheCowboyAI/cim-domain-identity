//! Identity relationship systems

use crate::{aggregate::IdentityAggregate, commands::*, components::*, events::*};
use bevy::ecs::prelude::*;
use uuid::Uuid;

/// System to establish relationships between identities
pub fn establish_relationship_system(
    mut commands: Commands,
    mut events: EventReader<EstablishRelationshipCommand>,
    mut established_events: EventWriter<RelationshipEstablished>,
    identities: Query<&IdentityEntity>,
    existing_relationships: Query<&IdentityRelationship>,
) {
    for event in events.read() {
        // Validate identities exist
        let from_exists = identities
            .iter()
            .any(|i| i.identity_id == event.from_identity);
        let to_exists = identities
            .iter()
            .any(|i| i.identity_id == event.to_identity);

        if !from_exists || !to_exists {
            eprintln!("Cannot establish relationship: one or both identities don't exist");
            continue;
        }

        // Check for duplicate relationships
        let duplicate = existing_relationships.iter().any(|r| {
            r.source_identity == event.from_identity
                && r.target_identity == event.to_identity
                && r.relationship_type == event.relationship_type
        });

        if duplicate {
            eprintln!("Relationship already exists");
            continue;
        }

        // Validate through aggregate
        match IdentityAggregate::validate_relationship(
            event.from_identity,
            event.to_identity,
            &event.relationship_type,
        ) {
            Ok(_) => {
                let relationship_id = Uuid::new_v4();

                // Spawn the relationship entity
                commands.spawn((IdentityRelationship {
                    relationship_id: Uuid::new_v4(),
                    source_identity: event.from_identity,
                    target_identity: event.to_identity,
                    relationship_type: event.relationship_type.clone(),
                    rules: event.rules.clone(),
                    established_at: chrono::Utc::now(),
                    established_by: Some(event.established_by),
                    expires_at: None,
                },));

                // Emit established event
                established_events.write(RelationshipEstablished {
                    relationship_id,
                    from_identity: event.from_identity,
                    to_identity: event.to_identity,
                    relationship_type: event.relationship_type.clone(),
                    established_by: event.established_by,
                    established_at: chrono::Utc::now(),
                });
            }
            Err(e) => {
                eprintln!("Failed to establish relationship: {e}");
            }
        }
    }
}

/// System to validate relationships
pub fn validate_relationships_system(
    mut events: EventReader<ValidateRelationshipCommand>,
    mut validated_events: EventWriter<RelationshipValidated>,
    relationships: Query<(&IdentityRelationship, Entity)>,
    identities: Query<&IdentityEntity>,
) {
    for event in events.read() {
        for (relationship, entity) in relationships.iter() {
            if relationship.relationship_id == event.relationship_id {
                // Check if both identities still exist and are active
                let from_active = identities
                    .iter()
                    .find(|i| i.identity_id == relationship.source_identity)
                    .map(|i| matches!(i.status, IdentityStatus::Active))
                    .unwrap_or(false);

                let to_active = identities
                    .iter()
                    .find(|i| i.identity_id == relationship.target_identity)
                    .map(|i| matches!(i.status, IdentityStatus::Active))
                    .unwrap_or(false);

                let is_valid = from_active && to_active;

                // Check expiration
                let expired = relationship
                    .expires_at
                    .map(|exp| exp < chrono::Utc::now())
                    .unwrap_or(false);

                if !is_valid || expired {
                    // Remove invalid relationship
                    commands.entity(entity).despawn();

                    validated_events.write(RelationshipValidated {
                        relationship_id: event.relationship_id,
                        is_valid: false,
                        reason: if !is_valid {
                            "One or both identities are not active".to_string()
                        } else {
                            "Relationship has expired".to_string()
                        },
                        validated_at: chrono::Utc::now(),
                    });
                } else {
                    validated_events.write(RelationshipValidated {
                        relationship_id: event.relationship_id,
                        is_valid: true,
                        reason: "Relationship is valid".to_string(),
                        validated_at: chrono::Utc::now(),
                    });
                }
            }
        }
    }
}

/// System to traverse relationship graphs
pub fn traverse_relationships_system(
    mut events: EventReader<TraverseRelationshipsCommand>,
    mut traversed_events: EventWriter<RelationshipsTraversed>,
    relationships: Query<&IdentityRelationship>,
) {
    for event in events.read() {
        let mut visited = std::collections::HashSet::new();
        let mut paths = Vec::new();
        let mut queue = std::collections::VecDeque::new();

        // Start traversal from the root identity
        queue.push_back((event.from_identity, vec![event.from_identity], vec![], 0));
        visited.insert(event.from_identity);

        while let Some((current, path, rels, depth)) = queue.pop_front() {
            // Check depth limit
            if let Some(max_depth) = event.max_depth {
                if depth >= max_depth {
                    continue;
                }
            }

            // Find relationships from current identity
            for relationship in relationships.iter() {
                if relationship.source_identity == current {
                    // Check if relationship type matches filter
                    if let Some(filter) = &event.relationship_filter {
                        let type_matches = filter.iter().any(|t| {
                            std::mem::discriminant(t)
                                == std::mem::discriminant(&relationship.relationship_type)
                        });
                        if !type_matches {
                            continue;
                        }
                    }

                    let next = relationship.target_identity;

                    // Check if we've visited this identity
                    if !visited.contains(&next) {
                        visited.insert(next);

                        let mut new_path = path.clone();
                        new_path.push(next);

                        let mut new_rels = rels.clone();
                        new_rels.push(relationship.relationship_id);

                        // If this is the target, save the path
                        if Some(next) == event.to_identity {
                            paths.push((new_path.clone(), new_rels.clone()));
                        }

                        // Continue traversal
                        queue.push_back((next, new_path, new_rels, depth + 1));
                    }
                }
            }
        }

        // Emit traversal result
        traversed_events.write(RelationshipsTraversed {
            from_identity: event.from_identity,
            to_identity: event.to_identity,
            paths,
            total_identities_visited: visited.len(),
            traversed_at: chrono::Utc::now(),
        });
    }
}

/// System to expire relationships
pub fn expire_relationships_system(
    mut commands: Commands,
    mut expired_events: EventWriter<RelationshipExpired>,
    relationships: Query<(&IdentityRelationship, Entity)>,
) {
    let now = chrono::Utc::now();

    for (relationship, entity) in relationships.iter() {
        if let Some(expires_at) = relationship.expires_at {
            if expires_at < now {
                commands.entity(entity).despawn();

                expired_events.write(RelationshipExpired {
                    relationship_id: relationship.relationship_id,
                    from_identity: relationship.source_identity,
                    to_identity: relationship.target_identity,
                    relationship_type: relationship.relationship_type.clone(),
                    expired_at: expires_at,
                });
            }
        }
    }
}
