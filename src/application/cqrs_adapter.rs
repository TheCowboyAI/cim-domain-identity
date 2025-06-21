//! CQRS Adapters for Identity Domain
//!
//! This module provides adapters that implement the standard CQRS CommandHandler
//! and QueryHandler traits while delegating to the Identity-specific handlers.
//! This allows the Identity domain to participate in correlation/causation tracking
//! while maintaining its existing API.

use cim_domain::{
    Command, CommandEnvelope, CommandHandler, CommandAcknowledgment, CommandStatus,
    Query, QueryEnvelope, QueryHandler, QueryAcknowledgment, QueryStatus,
    EntityId,
};
use serde::{Deserialize, Serialize};
use crate::{
    PersonId, PersonCommand, OrganizationId, OrganizationCommand,
    IdentityCommandHandler, IdentityQueryHandler,
};

/// Wrapper for PersonCommand that implements the Command trait
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonCommandWrapper {
    pub person_id: PersonId,
    pub command: PersonCommand,
}

impl Command for PersonCommandWrapper {
    type Aggregate = crate::Person;
    
    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        // PersonId doesn't directly convert to EntityId, so we return None
        // The actual ID is handled by the wrapper
        None
    }
}

/// Wrapper for OrganizationCommand that implements the Command trait
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationCommandWrapper {
    pub org_id: OrganizationId,
    pub command: OrganizationCommand,
}

impl Command for OrganizationCommandWrapper {
    type Aggregate = crate::Organization;
    
    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        // OrganizationId doesn't directly convert to EntityId, so we return None
        // The actual ID is handled by the wrapper
        None
    }
}

/// CQRS adapter for IdentityCommandHandler
pub struct IdentityCommandHandlerAdapter<H: IdentityCommandHandler> {
    inner: H,
}

impl<H: IdentityCommandHandler> IdentityCommandHandlerAdapter<H> {
    pub fn new(inner: H) -> Self {
        Self { inner }
    }
}

impl<H: IdentityCommandHandler> CommandHandler<PersonCommandWrapper> for IdentityCommandHandlerAdapter<H> {
    fn handle(&mut self, envelope: CommandEnvelope<PersonCommandWrapper>) -> CommandAcknowledgment {
        let command_id = envelope.id;
        let correlation_id = envelope.correlation_id().clone();
        let wrapper = envelope.command;
        
        // Process the command synchronously (blocking on async)
        let runtime = tokio::runtime::Handle::current();
        let result = runtime.block_on(async {
            self.inner.handle_person_command(wrapper.person_id, wrapper.command).await
        });
        
        match result {
            Ok(()) => CommandAcknowledgment {
                command_id,
                correlation_id,
                status: CommandStatus::Accepted,
                reason: None,
            },
            Err(error) => CommandAcknowledgment {
                command_id,
                correlation_id,
                status: CommandStatus::Rejected,
                reason: Some(error.to_string()),
            },
        }
    }
}

impl<H: IdentityCommandHandler> CommandHandler<OrganizationCommandWrapper> for IdentityCommandHandlerAdapter<H> {
    fn handle(&mut self, envelope: CommandEnvelope<OrganizationCommandWrapper>) -> CommandAcknowledgment {
        let command_id = envelope.id;
        let correlation_id = envelope.correlation_id().clone();
        let wrapper = envelope.command;
        
        // Process the command synchronously (blocking on async)
        let runtime = tokio::runtime::Handle::current();
        let result = runtime.block_on(async {
            self.inner.handle_organization_command(wrapper.org_id, wrapper.command).await
        });
        
        match result {
            Ok(()) => CommandAcknowledgment {
                command_id,
                correlation_id,
                status: CommandStatus::Accepted,
                reason: None,
            },
            Err(error) => CommandAcknowledgment {
                command_id,
                correlation_id,
                status: CommandStatus::Rejected,
                reason: Some(error.to_string()),
            },
        }
    }
}

/// Query types for Identity domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentityQuery {
    FindPersonById { person_id: PersonId },
    FindPersonByEmail { email: String },
    FindOrganizationById { org_id: OrganizationId },
    FindOrganizationByName { name: String },
    FindOrganizationsForPerson { person_id: PersonId },
    FindOrganizationMembers { org_id: OrganizationId },
    FindOrganizationAdmins { org_id: OrganizationId },
    SearchPeopleByName { name_query: String },
    SearchOrganizationsByName { name_query: String },
}

impl Query for IdentityQuery {}

/// CQRS adapter for IdentityQueryHandler
pub struct IdentityQueryHandlerAdapter<H: IdentityQueryHandler> {
    inner: H,
}

impl<H: IdentityQueryHandler> IdentityQueryHandlerAdapter<H> {
    pub fn new(inner: H) -> Self {
        Self { inner }
    }
}

impl<H: IdentityQueryHandler> QueryHandler<IdentityQuery> for IdentityQueryHandlerAdapter<H> {
    fn handle(&self, envelope: QueryEnvelope<IdentityQuery>) -> QueryAcknowledgment {
        let query_id = envelope.id;
        let correlation_id = envelope.correlation_id().clone();
        
        // Process the query synchronously (blocking on async)
        let runtime = tokio::runtime::Handle::current();
        let result = runtime.block_on(async {
            match &envelope.query {
                IdentityQuery::FindPersonById { person_id } => {
                    self.inner.find_person_by_id(*person_id).await.map(|opt| {
                        // TODO: Publish result to event stream with correlation
                        serde_json::to_value(opt).unwrap()
                    })
                }
                IdentityQuery::FindPersonByEmail { email } => {
                    self.inner.find_person_by_email(email).await.map(|opt| {
                        // TODO: Publish result to event stream with correlation
                        serde_json::to_value(opt).unwrap()
                    })
                }
                IdentityQuery::FindOrganizationById { org_id } => {
                    self.inner.find_organization_by_id(*org_id).await.map(|opt| {
                        // TODO: Publish result to event stream with correlation
                        serde_json::to_value(opt).unwrap()
                    })
                }
                IdentityQuery::FindOrganizationByName { name } => {
                    self.inner.find_organization_by_name(name).await.map(|opt| {
                        // TODO: Publish result to event stream with correlation
                        serde_json::to_value(opt).unwrap()
                    })
                }
                IdentityQuery::FindOrganizationsForPerson { person_id } => {
                    self.inner.find_organizations_for_person(*person_id).await.map(|orgs| {
                        // TODO: Publish result to event stream with correlation
                        serde_json::to_value(orgs).unwrap()
                    })
                }
                IdentityQuery::FindOrganizationMembers { org_id } => {
                    self.inner.find_organization_members(*org_id).await.map(|members| {
                        // TODO: Publish result to event stream with correlation
                        serde_json::to_value(members).unwrap()
                    })
                }
                IdentityQuery::FindOrganizationAdmins { org_id } => {
                    self.inner.find_organization_admins(*org_id).await.map(|admins| {
                        // TODO: Publish result to event stream with correlation
                        serde_json::to_value(admins).unwrap()
                    })
                }
                IdentityQuery::SearchPeopleByName { name_query } => {
                    self.inner.search_people_by_name(name_query).await.map(|people| {
                        // TODO: Publish result to event stream with correlation
                        serde_json::to_value(people).unwrap()
                    })
                }
                IdentityQuery::SearchOrganizationsByName { name_query } => {
                    self.inner.search_organizations_by_name(name_query).await.map(|orgs| {
                        // TODO: Publish result to event stream with correlation
                        serde_json::to_value(orgs).unwrap()
                    })
                }
            }
        });
        
        match result {
            Ok(_) => QueryAcknowledgment {
                query_id,
                correlation_id,
                status: QueryStatus::Accepted,
                reason: None,
            },
            Err(error) => QueryAcknowledgment {
                query_id,
                correlation_id,
                status: QueryStatus::Rejected,
                reason: Some(error.to_string()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_person_command_wrapper() {
        let person_id = PersonId::new();
        let command = PersonCommand::UnlockAccount;
        let wrapper = PersonCommandWrapper { person_id, command: command.clone() };
        
        // Test serialization
        let serialized = serde_json::to_string(&wrapper).unwrap();
        let deserialized: PersonCommandWrapper = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(deserialized.person_id, person_id);
    }
    
    #[test]
    fn test_identity_query_serialization() {
        let query = IdentityQuery::FindPersonByEmail { email: "test@example.com".to_string() };
        
        let serialized = serde_json::to_string(&query).unwrap();
        let deserialized: IdentityQuery = serde_json::from_str(&serialized).unwrap();
        
        match deserialized {
            IdentityQuery::FindPersonByEmail { email } => {
                assert_eq!(email, "test@example.com");
            }
            _ => panic!("Expected FindPersonByEmail query"),
        }
    }
} 