//! Domain layer for the Identity context
//!
//! Contains the core business logic for identity management including:
//! - Person aggregate
//! - Organization aggregate
//! - Value objects shared between aggregates

pub mod person;
pub mod organization;

// Shared value objects
mod value_objects;
pub use value_objects::*;
