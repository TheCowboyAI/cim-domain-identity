//! ECS Systems for the Identity domain
//!
//! This module contains all systems that operate on identity components.
//! Systems implement the behavior and business logic of the domain.

pub mod lifecycle;
pub mod projection;
pub mod relationship;
pub mod verification;
pub mod workflow;
pub mod markers;

// Re-export key systems
pub use lifecycle::{
    archive_identity_system, create_identity_system, merge_identities_system,
    update_identity_system,
};

pub use relationship::{
    establish_relationship_system, expire_relationships_system, traverse_relationships_system,
    validate_relationships_system,
};

pub use workflow::{
    complete_workflow_system, process_workflow_step_system, start_workflow_system,
    timeout_workflows_system,
};

pub use verification::{
    complete_verification_system, process_verification_system, start_verification_system,
};

pub use projection::{
    create_projection_system, sync_projections_system, validate_projection_system,
};

// Re-export all systems
pub use verification::*;

// Re-export marker systems and components
pub use markers::{
    add_identity_markers_system,
    add_location_markers_system,
    PersonMarker,
    LocationMarker,
    OrganizationMarker,
    AgentMarker,
};
