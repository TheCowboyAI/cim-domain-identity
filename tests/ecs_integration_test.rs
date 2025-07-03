//! Integration tests for the ECS-based Identity domain

use bevy_ecs::prelude::*;
use cim_domain_identity::{
    // Systems
    create_identity_system,
    establish_relationship_system,
    merge_identities_system,

    // Queries
    queries::{find_identity_by_id, find_relationships_for_identity},
    update_identity_system,
    // Commands
    CreateIdentityCommand,
    EstablishRelationshipCommand,
    IdentitiesMerged,

    // Events
    IdentityCreated,
    IdentityEntity,
    // Components
    IdentityId,
    IdentityRelationship,
    IdentityStatus,
    IdentityType,
    IdentityUpdated,
    IdentityVerification,
    MergeIdentitiesCommand,

    RelationshipEstablished,
    RelationshipType,

    UpdateIdentityCommand,
    VerificationLevel,
};

#[test]
fn test_identity_lifecycle() {
    // Create a Bevy world
    let mut world = World::new();

    // Register event types
    world.init_resource::<Events<CreateIdentityCommand>>();
    world.init_resource::<Events<UpdateIdentityCommand>>();
    world.init_resource::<Events<IdentityCreated>>();
    world.init_resource::<Events<IdentityUpdated>>();

    // Create identity command
    let create_cmd = CreateIdentityCommand {
        identity_type: IdentityType::Person,
        initial_claims: None,
        created_by: IdentityId::new(),
    };

    // Send command
    world
        .resource_mut::<Events<CreateIdentityCommand>>()
        .send(create_cmd);

    // Create and run a minimal schedule
    let mut schedule = Schedule::default();
    schedule.add_systems(create_identity_system);
    schedule.run(&mut world);

    // Verify identity was created
    let identities: Vec<_> = world.query::<&IdentityEntity>().iter(&world).collect();

    assert_eq!(identities.len(), 1);
    assert_eq!(identities[0].identity_type, IdentityType::Person);
    assert!(matches!(identities[0].status, IdentityStatus::Pending));

    // Verify event was emitted
    let created_events = world.resource::<Events<IdentityCreated>>();
    let mut reader = created_events.get_reader();
    let events: Vec<_> = reader.read(created_events).collect();
    assert_eq!(events.len(), 1);
}

#[test]
fn test_identity_update() {
    let mut world = World::new();

    // Register resources
    world.init_resource::<Events<CreateIdentityCommand>>();
    world.init_resource::<Events<UpdateIdentityCommand>>();
    world.init_resource::<Events<IdentityCreated>>();
    world.init_resource::<Events<IdentityUpdated>>();

    // Create an identity first
    let identity_id = IdentityId::new();
    world.spawn((
        IdentityEntity {
            identity_id,
            identity_type: IdentityType::Person,
            status: IdentityStatus::Pending,
        },
        cim_domain_identity::components::IdentityMetadata::default(),
    ));

    // Update command
    let update_cmd = UpdateIdentityCommand {
        identity_id,
        new_status: Some(IdentityStatus::Active),
        updated_by: IdentityId::new(),
    };

    world
        .resource_mut::<Events<UpdateIdentityCommand>>()
        .send(update_cmd);

    // Run update system
    let mut schedule = Schedule::default();
    schedule.add_systems(update_identity_system);
    schedule.run(&mut world);

    // Verify update
    let identity = world
        .query::<&IdentityEntity>()
        .iter(&world)
        .find(|e| e.identity_id == identity_id)
        .unwrap();

    assert!(matches!(identity.status, IdentityStatus::Active));
}

#[test]
fn test_relationship_establishment() {
    let mut world = World::new();

    // Register resources
    world.init_resource::<Events<EstablishRelationshipCommand>>();
    world.init_resource::<Events<RelationshipEstablished>>();

    // Create two identities
    let person_id = IdentityId::new();
    let org_id = IdentityId::new();

    world.spawn(IdentityEntity {
        identity_id: person_id,
        identity_type: IdentityType::Person,
        status: IdentityStatus::Active,
    });

    world.spawn(IdentityEntity {
        identity_id: org_id,
        identity_type: IdentityType::Organization,
        status: IdentityStatus::Active,
    });

    // Establish relationship
    let establish_cmd = EstablishRelationshipCommand {
        from_identity: person_id,
        to_identity: org_id,
        relationship_type: RelationshipType::MemberOf {
            role: "Employee".to_string(),
            department: Some("Engineering".to_string()),
        },
        established_by: IdentityId::new(),
        can_delegate: false,
        can_revoke: true,
        expires_at: None,
        max_depth: None,
        metadata: Default::default(),
    };

    world
        .resource_mut::<Events<EstablishRelationshipCommand>>()
        .send(establish_cmd);

    // Run system
    let mut schedule = Schedule::default();
    schedule.add_systems(establish_relationship_system);
    schedule.run(&mut world);

    // Verify relationship
    let relationships: Vec<_> = world
        .query::<&IdentityRelationship>()
        .iter(&world)
        .collect();

    assert_eq!(relationships.len(), 1);
    assert_eq!(relationships[0].from_identity, person_id);
    assert_eq!(relationships[0].to_identity, org_id);
}

#[test]
fn test_identity_merge() {
    let mut world = World::new();

    // Register resources
    world.init_resource::<Events<MergeIdentitiesCommand>>();
    world.init_resource::<Events<IdentitiesMerged>>();

    // Create two person identities with different verification levels
    let source_id = IdentityId::new();
    let target_id = IdentityId::new();

    world.spawn((
        IdentityEntity {
            identity_id: source_id,
            identity_type: IdentityType::Person,
            status: IdentityStatus::Active,
        },
        IdentityVerification {
            verification_level: VerificationLevel::Basic,
            verified_at: Some(chrono::Utc::now()),
            verified_by: Some(IdentityId::new()),
            verification_method: None,
        },
    ));

    world.spawn((
        IdentityEntity {
            identity_id: target_id,
            identity_type: IdentityType::Person,
            status: IdentityStatus::Active,
        },
        IdentityVerification {
            verification_level: VerificationLevel::Enhanced,
            verified_at: Some(chrono::Utc::now()),
            verified_by: Some(IdentityId::new()),
            verification_method: None,
        },
    ));

    // Merge command
    let merge_cmd = MergeIdentitiesCommand {
        source_identity: source_id,
        target_identity: target_id,
        merged_by: IdentityId::new(),
    };

    world
        .resource_mut::<Events<MergeIdentitiesCommand>>()
        .send(merge_cmd);

    // Run system
    let mut schedule = Schedule::default();
    schedule.add_systems(merge_identities_system);
    schedule.run(&mut world);

    // Verify source is merged
    let source = world
        .query::<&IdentityEntity>()
        .iter(&world)
        .find(|e| e.identity_id == source_id)
        .unwrap();

    assert!(
        matches!(source.status, IdentityStatus::Merged { merged_into } if merged_into == target_id)
    );
}

#[test]
fn test_query_operations() {
    let mut world = World::new();

    // Create test data
    let identity_id = IdentityId::new();
    world.spawn((
        IdentityEntity {
            identity_id,
            identity_type: IdentityType::Person,
            status: IdentityStatus::Active,
        },
        IdentityVerification {
            verification_level: VerificationLevel::Enhanced,
            verified_at: Some(chrono::Utc::now()),
            verified_by: Some(IdentityId::new()),
            verification_method: None,
        },
        cim_domain_identity::components::IdentityMetadata::default(),
    ));

    // Test find by ID
    let result = find_identity_by_id(&world, identity_id);
    assert!(result.is_some());
    assert_eq!(result.unwrap().identity.identity_id, identity_id);

    // Test find relationships (should be empty)
    let relationships = find_relationships_for_identity(&world, identity_id);
    assert_eq!(relationships.len(), 0);
}
