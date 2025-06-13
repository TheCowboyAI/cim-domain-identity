//! Person aggregate module

mod aggregate;
mod commands;
mod events;

pub use aggregate::{Person, PersonId};
pub use commands::PersonCommand;
pub use events::PersonEvent;
