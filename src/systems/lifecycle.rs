//! Identity lifecycle systems

use bevy_ecs::prelude::*;
use crate::{
    components::*,
    events::*,
    commands::*,
};

/// System to create new identities
pub fn create_identity_system(
    mut commands: Commands,
    mut events: EventReader<CreateIdentityCommand>,
    mut created_events: EventWriter<IdentityCreated>,
    existing_identities: Query<&IdentityEntity>,
) {
    for event in events.read() {
        // Check for duplicates
        let duplicate_exists = existing_identities.iter()
            .any(|e| match &event.identity_type {
                IdentityType::Person => {
                    // Check if person with same email exists
                    // This would normally check claims, simplified here
                    false
                },
                IdentityType::Organization => {
                    // Check if org with same name exists
                    false
                },
                _ => false,
            });

        if duplicate_exists {
            // Handle duplicate - could emit error event
            continue;
        }

        let identity_id = IdentityId::new();
        
        // Spawn the identity entity with components
        let entity = commands.spawn((
            IdentityEntity {
                identity_id,
                identity_type: event.identity_type,
                status: IdentityStatus::Pending,
            },
            IdentityMetadata::default(),
            IdentityVerification {
                verification_level: VerificationLevel::Unverified,
                verified_at: None,
                verified_by: None,
                verification_method: None,
            },
        )).id();

        // Add initial claims if provided
        if let Some(claims) = &event.initial_claims {
            for (claim_type, value) in claims {
                commands.entity(entity).insert(IdentityClaim {
                    claim_type: claim_type.clone(),
                    value: value.clone(),
                    verified: false,
                    issuer: Some(event.created_by),
                    issued_at: chrono::Utc::now(),
                    expires_at: None,
                });
            }
        }

        // Emit created event
        created_events.send(IdentityCreated {
            identity_id,
            identity_type: event.identity_type,
            created_by: event.created_by,
            created_at: chrono::Utc::now(),
        });
    }
}

/// System to update identity status
pub fn update_identity_system(
    mut commands: Commands,
    mut events: EventReader<UpdateIdentityCommand>,
    mut updated_events: EventWriter<IdentityUpdated>,
    mut identities: Query<(&mut IdentityEntity, &mut IdentityMetadata)>,
) {
    for event in events.read() {
        for (mut identity, mut metadata) in identities.iter_mut() {
            if identity.identity_id == event.identity_id {
                // Update status if provided
                if let Some(new_status) = event.new_status {
                    let old_status = identity.status;
                    identity.status = new_status;

                    // Update metadata
                    metadata.updated_at = chrono::Utc::now();
                    metadata.version += 1;

                    // Emit updated event
                    updated_events.send(IdentityUpdated {
                        identity_id: event.identity_id,
                        old_status,
                        new_status,
                        updated_by: event.updated_by,
                        updated_at: chrono::Utc::now(),
                    });
                }
            }
        }
    }
}

/// System to merge duplicate identities
pub fn merge_identities_system(
    mut commands: Commands,
    mut events: EventReader<MergeIdentitiesCommand>,
    mut merged_events: EventWriter<IdentitiesMerged>,
    mut identities: Query<(&mut IdentityEntity, &IdentityVerification)>,
    relationships: Query<&IdentityRelationship>,
    workflows: Query<&IdentityWorkflow>,
) {
    for event in events.read() {
        // Find source and target identities
        let mut source_found = false;
        let mut target_found = false;
        let mut source_verification_level = VerificationLevel::Unverified;
        let mut target_verification_level = VerificationLevel::Unverified;

        // First pass: validate both identities exist
        for (identity, verification) in identities.iter() {
            if identity.identity_id == event.source_identity {
                source_found = true;
                source_verification_level = verification.verification_level;
            }
            if identity.identity_id == event.target_identity {
                target_found = true;
                target_verification_level = verification.verification_level;
            }
        }

        if !source_found || !target_found {
            // Handle missing identity - could emit error event
            continue;
        }

        // Second pass: update source identity status
        for (mut identity, _) in identities.iter_mut() {
            if identity.identity_id == event.source_identity {
                identity.status = IdentityStatus::Merged {
                    merged_into: event.target_identity,
                };
            }
        }

        // Migrate relationships
        let mut migrated_relationships = Vec::new();
        for relationship in relationships.iter() {
            if relationship.from_identity == event.source_identity {
                migrated_relationships.push((
                    relationship.relationship_id,
                    relationship.to_identity,
                    relationship.relationship_type.clone(),
                ));
            }
        }

        // Migrate workflows
        let mut migrated_workflows = Vec::new();
        for workflow in workflows.iter() {
            if workflow.identity_id == event.source_identity {
                migrated_workflows.push((
                    workflow.workflow_id,
                    workflow.workflow_type.clone(),
                ));
            }
        }

        // Emit merged event
        merged_events.send(IdentitiesMerged {
            source_identity: event.source_identity,
            target_identity: event.target_identity,
            merged_by: event.merged_by,
            merged_at: chrono::Utc::now(),
            migrated_relationships: migrated_relationships.len(),
            migrated_workflows: migrated_workflows.len(),
            retained_verification_level: source_verification_level.max(target_verification_level),
        });
    }
}

/// System to archive identities
pub fn archive_identity_system(
    mut commands: Commands,
    mut events: EventReader<ArchiveIdentityCommand>,
    mut archived_events: EventWriter<IdentityArchived>,
    mut identities: Query<(&mut IdentityEntity, &mut IdentityMetadata)>,
    relationships: Query<&IdentityRelationship>,
) {
    for event in events.read() {
        for (mut identity, mut metadata) in identities.iter_mut() {
            if identity.identity_id == event.identity_id {
                // Check if identity can be archived
                let active_relationships = relationships.iter()
                    .filter(|r| r.from_identity == event.identity_id || 
                               r.to_identity == event.identity_id)
                    .count();

                if active_relationships > 0 && !event.force {
                    // Cannot archive identity with active relationships
                    continue;
                }

                // Update status
                let old_status = identity.status;
                identity.status = IdentityStatus::Archived;

                // Update metadata
                metadata.updated_at = chrono::Utc::now();
                metadata.version += 1;

                // Emit archived event
                archived_events.send(IdentityArchived {
                    identity_id: event.identity_id,
                    previous_status: old_status,
                    archived_by: event.archived_by,
                    archived_at: chrono::Utc::now(),
                    reason: event.reason.clone(),
                });
            }
        }
    }
} 