//! Identity lifecycle systems

use crate::{aggregate::IdentityAggregate, commands::*, components::*, events::*};
use bevy::ecs::prelude::*;
use uuid::Uuid;

/// System to create new identities
pub fn create_identity_system(
    mut commands: Commands,
    mut events: EventReader<CreateIdentityCommand>,
    mut created_events: EventWriter<IdentityCreated>,
    existing_identities: Query<&IdentityEntity>,
) {
    for event in events.read() {
        // Collect existing identities for validation
        let existing: Vec<_> = existing_identities.iter().cloned().collect();

        // Validate through aggregate
        match IdentityAggregate::validate_create(event, &existing) {
            Ok(_) => {
                let identity_id = Uuid::new_v4();

                // Spawn the identity entity with components
                let entity = commands
                    .spawn((
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
                    ))
                    .id();

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
                created_events.write(IdentityCreated {
                    identity_id,
                    identity_type: event.identity_type,
                    created_by: Some(event.created_by),
                    created_at: chrono::Utc::now(),
                    external_reference: event.external_reference.clone(),
                });
            }
            Err(e) => {
                // In production, would emit error event
                eprintln!("Failed to create identity: {e}");
            }
        }
    }
}

/// System to update identity status
pub fn update_identity_system(
    mut events: EventReader<UpdateIdentityCommand>,
    mut updated_events: EventWriter<IdentityUpdated>,
    mut identities: Query<(&mut IdentityEntity, &mut IdentityMetadata)>,
) {
    for event in events.read() {
        for (mut identity, mut metadata) in identities.iter_mut() {
            if identity.identity_id == event.identity_id {
                // Validate through aggregate
                match IdentityAggregate::validate_update(&identity, event) {
                    Ok(_) => {
                        // Update status if provided
                        if let Some(new_status) = event.new_status {
                            let old_status = identity.status;
                            identity.status = new_status;

                            // Update metadata
                            metadata.updated_at = chrono::Utc::now();
                            metadata.version += 1;

                            // Emit updated event
                            updated_events.write(IdentityUpdated {
                                identity_id: event.identity_id,
                                old_status,
                                new_status,
                                updated_by: event.updated_by,
                                updated_at: chrono::Utc::now(),
                            });
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to update identity: {e}");
                    }
                }
            }
        }
    }
}

/// System to merge duplicate identities
pub fn merge_identities_system(
    mut events: EventReader<MergeIdentitiesCommand>,
    mut merged_events: EventWriter<IdentitiesMerged>,
    mut identities: Query<(&mut IdentityEntity, &IdentityVerification)>,
    relationships: Query<&IdentityRelationship>,
    workflows: Query<&IdentityWorkflow>,
) {
    for event in events.read() {
        // Find source and target identities
        let mut source_data = None;
        let mut target_data = None;

        // Collect identity data for validation
        for (identity, verification) in identities.iter() {
            if identity.identity_id == event.source_identity {
                source_data = Some((identity.clone(), verification.clone()));
            }
            if identity.identity_id == event.target_identity {
                target_data = Some((identity.clone(), verification.clone()));
            }
        }

        if let (
            Some((source_identity, source_verification)),
            Some((target_identity, target_verification)),
        ) = (source_data, target_data)
        {
            // Validate through aggregate
            match IdentityAggregate::validate_merge(
                &source_identity,
                &target_identity,
                &source_verification,
                &target_verification,
            ) {
                Ok(_) => {
                    // Update source identity status
                    for (mut identity, _) in identities.iter_mut() {
                        if identity.identity_id == event.source_identity {
                            identity.status = IdentityStatus::Merged {
                                merged_into: event.target_identity,
                            };
                        }
                    }

                    // Count migrated relationships and workflows
                    let migrated_relationships = relationships
                        .iter()
                        .filter(|r| r.source_identity == event.source_identity)
                        .count();

                    let migrated_workflows = workflows
                        .iter()
                        .filter(|w| w.identity_id == event.source_identity)
                        .count();

                    // Emit merged event
                    merged_events.write(IdentitiesMerged {
                        source_identity: event.source_identity,
                        target_identity: event.target_identity,
                        merged_by: event.merged_by,
                        merged_at: chrono::Utc::now(),
                        migrated_relationships,
                        migrated_workflows,
                        retained_verification_level: source_verification
                            .verification_level
                            .max(target_verification.verification_level),
                    });
                }
                Err(e) => {
                    eprintln!("Failed to merge identities: {e}");
                }
            }
        }
    }
}

/// System to archive identities
pub fn archive_identity_system(
    mut events: EventReader<ArchiveIdentityCommand>,
    mut archived_events: EventWriter<IdentityArchived>,
    mut identities: Query<(&mut IdentityEntity, &mut IdentityMetadata)>,
    relationships: Query<&IdentityRelationship>,
) {
    for event in events.read() {
        for (mut identity, mut metadata) in identities.iter_mut() {
            if identity.identity_id == event.identity_id {
                // Count active relationships
                let active_relationships = relationships
                    .iter()
                    .filter(|r| {
                        r.source_identity == event.identity_id
                            || r.target_identity == event.identity_id
                    })
                    .count();

                // Validate through aggregate
                match IdentityAggregate::validate_archive(
                    &identity,
                    active_relationships,
                    event.force,
                ) {
                    Ok(_) => {
                        // Update status
                        let old_status = identity.status;
                        identity.status = IdentityStatus::Archived;

                        // Update metadata
                        metadata.updated_at = chrono::Utc::now();
                        metadata.version += 1;

                        // Emit archived event
                        archived_events.write(IdentityArchived {
                            identity_id: event.identity_id,
                            previous_status: old_status,
                            archived_by: event.archived_by,
                            archived_at: chrono::Utc::now(),
                            reason: event.reason.clone(),
                        });
                    }
                    Err(e) => {
                        eprintln!("Failed to archive identity: {e}");
                    }
                }
            }
        }
    }
}
