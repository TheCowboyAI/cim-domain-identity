//! Type marker systems for identity entities
//!
//! These systems add type markers to entities based on their identity type,
//! enabling type-safe queries like Query<Entity, With<PersonMarker>>.

use bevy::ecs::prelude::*;
use crate::components::{IdentityEntity, IdentityType};

// Import markers from cim-domain and make them Components
use cim_domain::identifiers::markers::{
    PersonMarker as DomainPersonMarker,
    LocationMarker as DomainLocationMarker,
    OrganizationMarker as DomainOrganizationMarker,
    AgentMarker as DomainAgentMarker,
};

// Define marker components that wrap the domain markers
#[derive(Component, Debug, Clone, Copy)]
pub struct PersonMarker(DomainPersonMarker);

#[derive(Component, Debug, Clone, Copy)]
pub struct LocationMarker(DomainLocationMarker);

#[derive(Component, Debug, Clone, Copy)]
pub struct OrganizationMarker(DomainOrganizationMarker);

#[derive(Component, Debug, Clone, Copy)]
pub struct AgentMarker(DomainAgentMarker);

/// System that adds type markers to identity entities based on their type
pub fn add_identity_markers_system(
    mut commands: Commands,
    new_identities: Query<(Entity, &IdentityEntity), Added<IdentityEntity>>,
) {
    for (entity, identity) in new_identities.iter() {
        match identity.identity_type {
            IdentityType::Person => {
                commands.entity(entity).insert(PersonMarker(DomainPersonMarker));
            }
            IdentityType::Organization => {
                commands.entity(entity).insert(OrganizationMarker(DomainOrganizationMarker));
            }
            IdentityType::Service => {
                // Service entities get the Agent marker
                commands.entity(entity).insert(AgentMarker(DomainAgentMarker));
            }
            IdentityType::Device => {
                // Device entities also get the Agent marker (as they are automated agents)
                commands.entity(entity).insert(AgentMarker(DomainAgentMarker));
            }
            IdentityType::System => {
                // System entities get the Agent marker
                commands.entity(entity).insert(AgentMarker(DomainAgentMarker));
            }
            IdentityType::External => {
                // External entities don't get any specific marker
            }
        }
    }
}

/// System that adds location markers to entities with location data
pub fn add_location_markers_system(
    mut commands: Commands,
    locations: Query<Entity, (With<LocationComponent>, Without<LocationMarker>)>,
) {
    for entity in locations.iter() {
        commands.entity(entity).insert(LocationMarker(DomainLocationMarker));
    }
}

// Placeholder component for location data (would be defined elsewhere)
#[derive(Component)]
pub struct LocationComponent {
    pub latitude: f64,
    pub longitude: f64,
}

/// Example queries using the type markers
#[allow(dead_code)]
pub fn example_type_safe_queries(
    people: Query<Entity, With<PersonMarker>>,
    organizations: Query<Entity, With<OrganizationMarker>>,
    agents: Query<Entity, With<AgentMarker>>,
    locations: Query<Entity, With<LocationMarker>>,
) {
    // Count entities by type
    let person_count = people.iter().count();
    let org_count = organizations.iter().count();
    let agent_count = agents.iter().count();
    let location_count = locations.iter().count();
    
    tracing::debug!(
        "Entity counts - People: {}, Orgs: {}, Agents: {}, Locations: {}",
        person_count, org_count, agent_count, location_count
    );
}

/// Query for person entities with their identity data
#[allow(dead_code)]
pub fn query_people_with_data(
    people: Query<(&IdentityEntity, Entity), With<PersonMarker>>,
) {
    for (identity, entity) in people.iter() {
        tracing::debug!(
            "Person entity {:?} with ID: {}",
            entity,
            identity.identity_id
        );
    }
}

/// Query for organization entities with their identity data
#[allow(dead_code)]
pub fn query_organizations_with_data(
    organizations: Query<(&IdentityEntity, Entity), With<OrganizationMarker>>,
) {
    for (identity, entity) in organizations.iter() {
        tracing::debug!(
            "Organization entity {:?} with ID: {}",
            entity,
            identity.identity_id
        );
    }
}