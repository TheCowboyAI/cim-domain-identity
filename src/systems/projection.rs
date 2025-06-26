//! Identity projection systems

use bevy_ecs::prelude::*;
use crate::{
    components::*,
    events::*,
    commands::*,
};

/// System to create identity projections
pub fn create_projection_system(
    mut commands: Commands,
    mut events: EventReader<CreateProjectionCommand>,
    mut created_events: EventWriter<ProjectionCreated>,
    identities: Query<&IdentityEntity>,
    projections: Query<&IdentityProjection>,
) {
    for event in events.read() {
        // Validate identity exists
        let identity_exists = identities.iter()
            .any(|e| e.identity_id == event.identity_id);

        if !identity_exists {
            continue;
        }

        // Check for existing projection
        let existing_projection = projections.iter()
            .any(|p| p.identity_id == event.identity_id &&
                    p.projection_type == event.projection_type);

        if existing_projection {
            // Update existing projection instead of creating new
            continue;
        }

        let projection_id = uuid::Uuid::new_v4();
        let target_id = uuid::Uuid::new_v4(); // Would be actual target entity ID

        // Spawn projection entity
        commands.spawn((
            IdentityProjection {
                identity_id: event.identity_id,
                projection_type: event.projection_type.clone(),
                target_id,
                context: event.context.clone(),
                created_at: chrono::Utc::now(),
                last_synced: chrono::Utc::now(),
            },
        ));

        // Emit created event
        created_events.send(ProjectionCreated {
            projection_id,
            identity_id: event.identity_id,
            projection_type: event.projection_type.clone(),
            target_domain: event.target_domain.clone(),
            target_id,
            created_at: chrono::Utc::now(),
        });

        // Emit cross-domain events based on projection type
        match event.projection_type {
            ProjectionType::Person => {
                commands.spawn(IdentityLinkedToPerson {
                    identity_id: event.identity_id,
                    person_id: target_id,
                    linked_at: chrono::Utc::now(),
                });
            }
            ProjectionType::Organization => {
                commands.spawn(IdentityLinkedToOrganization {
                    identity_id: event.identity_id,
                    organization_id: target_id,
                    role: None,
                    linked_at: chrono::Utc::now(),
                });
            }
            _ => {}
        }
    }
}

/// System to synchronize projections
pub fn sync_projections_system(
    mut commands: Commands,
    mut events: EventReader<SyncProjectionsCommand>,
    mut synced_events: EventWriter<ProjectionsSynced>,
    mut projections: Query<(&mut IdentityProjection, &mut ProjectionSyncStatus)>,
    identities: Query<(&IdentityEntity, &IdentityVerification)>,
) {
    for event in events.read() {
        let mut projections_synced = 0;
        let mut sync_errors = 0;

        for (mut projection, mut sync_status) in projections.iter_mut() {
            // Filter by identity if specified
            if let Some(identity_id) = event.identity_id {
                if projection.identity_id != identity_id {
                    continue;
                }
            }

            // Filter by projection type if specified
            if let Some(ref projection_type) = event.projection_type {
                if &projection.projection_type != projection_type {
                    continue;
                }
            }

            // Check sync strategy
            match projection.context.sync_strategy {
                SyncStrategy::RealTime => {
                    // Always sync
                }
                SyncStrategy::Batch { interval_seconds } => {
                    let elapsed = chrono::Utc::now() - projection.last_synced;
                    if !event.force && elapsed.num_seconds() < interval_seconds as i64 {
                        continue;
                    }
                }
                SyncStrategy::OnDemand => {
                    if !event.force {
                        continue;
                    }
                }
                SyncStrategy::OneTime => {
                    // Never sync after initial creation
                    continue;
                }
            }

            // Perform sync (simplified)
            sync_status.last_sync_attempt = chrono::Utc::now();
            
            // In real implementation, would sync with target domain
            let sync_successful = true;

            if sync_successful {
                projection.last_synced = chrono::Utc::now();
                sync_status.last_successful_sync = Some(chrono::Utc::now());
                sync_status.pending_changes = 0;
                projections_synced += 1;
            } else {
                sync_status.sync_errors.push(SyncError {
                    occurred_at: chrono::Utc::now(),
                    error_type: "SyncFailed".to_string(),
                    message: "Failed to sync projection".to_string(),
                    retry_count: sync_status.sync_errors.len() as u32 + 1,
                });
                sync_errors += 1;
            }
        }

        // Emit synced event if any projections were processed
        if projections_synced > 0 || sync_errors > 0 {
            synced_events.send(ProjectionsSynced {
                identity_id: event.identity_id.unwrap_or(IdentityId::new()),
                projections_synced,
                sync_errors,
                synced_at: chrono::Utc::now(),
            });
        }
    }
}

/// System to validate projections
pub fn validate_projections_system(
    projections: Query<(&IdentityProjection, &ProjectionSyncStatus)>,
    identities: Query<&IdentityEntity>,
    mut commands: Commands,
) {
    for (projection, sync_status) in projections.iter() {
        // Validate identity still exists
        let identity_exists = identities.iter()
            .any(|e| e.identity_id == projection.identity_id);

        if !identity_exists {
            // Remove orphaned projection
            // In real implementation would despawn entity
            continue;
        }

        // Check for sync errors threshold
        if sync_status.sync_errors.len() > 10 {
            // Mark projection as failed
            // Would update status or emit alert
        }
    }
} 