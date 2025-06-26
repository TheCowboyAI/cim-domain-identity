//! Identity relationship systems

use bevy_ecs::prelude::*;
use crate::{
    components::*,
    events::*,
    commands::*,
};

/// System to establish relationships between identities
pub fn establish_relationship_system(
    mut commands: Commands,
    mut events: EventReader<EstablishRelationshipCommand>,
    mut established_events: EventWriter<RelationshipEstablished>,
    identities: Query<&IdentityEntity>,
    relationships: Query<&IdentityRelationship>,
) {
    for event in events.read() {
        // Validate both identities exist
        let from_exists = identities.iter()
            .any(|e| e.identity_id == event.from_identity);
        let to_exists = identities.iter()
            .any(|e| e.identity_id == event.to_identity);

        if !from_exists || !to_exists {
            // Handle missing identity
            continue;
        }

        // Check for duplicate relationships
        let duplicate_exists = relationships.iter()
            .any(|r| r.from_identity == event.from_identity &&
                    r.to_identity == event.to_identity &&
                    r.relationship_type == event.relationship_type);

        if duplicate_exists {
            continue;
        }

        let relationship_id = uuid::Uuid::new_v4();

        // Spawn relationship entity
        commands.spawn((
            IdentityRelationship {
                relationship_id,
                from_identity: event.from_identity,
                to_identity: event.to_identity,
                relationship_type: event.relationship_type.clone(),
                established_at: chrono::Utc::now(),
                established_by: Some(event.established_by),
                metadata: event.metadata.clone(),
            },
        ));

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

/// System to validate relationships based on rules
pub fn validate_relationship_system(
    relationships: Query<(&IdentityRelationship, Entity)>,
    identities: Query<&IdentityEntity>,
    rules: Query<&RelationshipRules>,
    mut commands: Commands,
) {
    // This would validate relationships against configured rules
    // For now, just a placeholder
}

/// System to traverse relationship graphs
pub fn traverse_relationships_system(
    graphs: Query<&RelationshipGraph>,
    relationships: Query<&IdentityRelationship>,
    mut commands: Commands,
) {
    for graph in graphs.iter() {
        // Build adjacency list
        let mut adjacency: std::collections::HashMap<IdentityId, Vec<(IdentityId, uuid::Uuid)>> = 
            std::collections::HashMap::new();

        for relationship in relationships.iter() {
            // Apply relationship filter if specified
            if let Some(filter) = &graph.relationship_filter {
                if !filter.contains(&relationship.relationship_type) {
                    continue;
                }
            }

            adjacency.entry(relationship.from_identity)
                .or_default()
                .push((relationship.to_identity, relationship.relationship_id));
        }

        // Perform BFS traversal from root
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        let mut paths = Vec::new();

        queue.push_back((graph.root_identity, vec![graph.root_identity], vec![], 0));

        while let Some((current, path, relationships, depth)) = queue.pop_front() {
            if let Some(max_depth) = graph.max_depth {
                if depth >= max_depth {
                    continue;
                }
            }

            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            // Store path if not root
            if current != graph.root_identity {
                paths.push((current, path.clone(), relationships.clone()));
            }

            // Add neighbors to queue
            if let Some(neighbors) = adjacency.get(&current) {
                for (neighbor, rel_id) in neighbors {
                    if !visited.contains(neighbor) {
                        let mut new_path = path.clone();
                        new_path.push(*neighbor);
                        let mut new_rels = relationships.clone();
                        new_rels.push(*rel_id);
                        queue.push_back((*neighbor, new_path, new_rels, depth + 1));
                    }
                }
            }
        }

        // Create cached paths
        for (target, path, relationships) in paths {
            commands.spawn(RelationshipPath {
                from: graph.root_identity,
                to: target,
                path,
                relationships,
                total_distance: relationships.len() as u32,
                cached_at: chrono::Utc::now(),
            });
        }
    }
}

/// System to expire relationships based on rules
pub fn expire_relationships_system(
    mut commands: Commands,
    mut revoked_events: EventWriter<RelationshipRevoked>,
    relationships: Query<(Entity, &IdentityRelationship)>,
    time: Res<bevy_time::Time>,
) {
    let now = chrono::Utc::now();

    for (entity, relationship) in relationships.iter() {
        // Check if relationship has expiration
        match &relationship.relationship_type {
            RelationshipType::DelegatesTo { expires_at, .. } |
            RelationshipType::ActsFor { valid_until: expires_at, .. } => {
                if let Some(expiry) = expires_at {
                    if now > *expiry {
                        // Remove expired relationship
                        commands.entity(entity).despawn();

                        // Emit revoked event
                        revoked_events.send(RelationshipRevoked {
                            relationship_id: relationship.relationship_id,
                            from_identity: relationship.from_identity,
                            to_identity: relationship.to_identity,
                            relationship_type: relationship.relationship_type.clone(),
                            revoked_by: IdentityId::new(), // System revocation
                            revoked_at: now,
                            reason: "Relationship expired".to_string(),
                        });
                    }
                }
            }
            _ => {}
        }
    }
} 