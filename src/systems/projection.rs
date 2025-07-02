//! Identity projection systems

use bevy_ecs::prelude::*;
use uuid::Uuid;

use crate::{
    components::{
        IdentityProjection, ProjectionType, ProjectionSyncStatus,
        CrossDomainReference, IdentityEntity, IdentityVerification,
    },
    events::{
        IdentityCreated, IdentityLinkedToPerson, IdentityLinkedToOrganization,
        ProjectionCreated, ProjectionsSynced,
    },
    commands::{CreateProjectionCommand, SyncProjectionsCommand},
};

/// System to create projections
pub fn create_projection_system(
    mut commands: Commands,
    mut events: EventReader<CreateProjectionCommand>,
    mut created_events: EventWriter<ProjectionCreated>,
) {
    for event in events.read() {
        // Create the projection entity
        commands.spawn(IdentityProjection {
            identity_id: event.identity_id,
            projection_type: event.projection_type.clone(),
            target_domain: event.target_domain.clone(),
            sync_status: ProjectionSyncStatus::Pending,
            last_sync: chrono::Utc::now(),
            last_synced: chrono::Utc::now(),
        });

        // Emit created event
        created_events.write(ProjectionCreated {
            projection_id: Uuid::new_v4(),
            identity_id: event.identity_id,
            projection_type: event.projection_type.clone(),
            target_domain: event.target_domain.clone(),
            target_id: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
        });
    }
}

/// System to synchronize projections
pub fn sync_projections_system(
    mut commands: Commands,
    mut events: EventReader<IdentityCreated>,
    mut writer: EventWriter<IdentityLinkedToPerson>,
    mut org_writer: EventWriter<IdentityLinkedToOrganization>,
    mut _projections: Query<&mut IdentityProjection>,
) {
    for event in events.read() {
        // Create projection for the new identity
        commands.spawn(IdentityProjection {
            identity_id: event.identity_id,
            projection_type: ProjectionType::Primary,
            target_domain: "identity".to_string(),
            sync_status: ProjectionSyncStatus::Synced,
            last_sync: chrono::Utc::now(),
            last_synced: chrono::Utc::now(),
        });

        // Check for cross-domain references
        if let Some(external_ref) = &event.external_reference {
            match external_ref.domain.as_str() {
                "person" => {
                    if let Ok(target_id) = Uuid::parse_str(&external_ref.entity_id) {
                        writer.write(IdentityLinkedToPerson {
                            identity_id: event.identity_id,
                            person_id: target_id,
                            linked_at: chrono::Utc::now(),
                        });
                    }
                }
                "organization" => {
                    if let Ok(target_id) = Uuid::parse_str(&external_ref.entity_id) {
                        org_writer.write(IdentityLinkedToOrganization {
                            identity_id: event.identity_id,
                            organization_id: target_id,
                            role: None,
                            linked_at: chrono::Utc::now(),
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

/// System to validate projections
pub fn validate_projection_system(
    mut _commands: Commands,
    identities: Query<(&IdentityEntity, &IdentityVerification)>,
    projections: Query<&IdentityProjection>,
) {
    // Basic validation logic
    for projection in projections.iter() {
        // Check if source identity exists
        let _identity_valid = identities.iter()
            .any(|(e, _)| e.identity_id == projection.identity_id);
        
        // In a real implementation, would validate against target domain
        // and emit validation events
    }
} 