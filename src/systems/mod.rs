//! ECS Systems for the Identity domain
//!
//! This module contains all systems that operate on identity components.
//! Systems implement the behavior and business logic of the domain.

pub mod lifecycle;
pub mod relationship;
pub mod workflow;
pub mod verification;
pub mod projection;

// Re-export key systems
pub use lifecycle::{
    create_identity_system,
    update_identity_system,
    merge_identities_system,
    archive_identity_system,
};

pub use relationship::{
    establish_relationship_system,
    validate_relationships_system,
    traverse_relationships_system,
    expire_relationships_system,
};

pub use workflow::{
    start_workflow_system,
    process_workflow_step_system,
    complete_workflow_system,
    timeout_workflows_system,
};

pub use verification::{
    start_verification_system,
    process_verification_system,
    complete_verification_system,
};

pub use projection::{
    create_projection_system,
    sync_projections_system,
    validate_projection_system,
};

// Re-export all systems
pub use lifecycle::*;
pub use relationship::*;
pub use workflow::*;
pub use verification::*;
pub use projection::*; 