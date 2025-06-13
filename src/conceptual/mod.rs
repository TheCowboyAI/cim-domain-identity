//! Conceptual space integration for the Identity context

mod concept_producer;
mod dimensions;
mod projections;

pub use concept_producer::{IdentityConceptProducer, IdentityConcept, IdentityEvent};
pub use dimensions::IdentityDimensions;
