# CIM Domain Identity

## Overview

The Identity Domain is responsible for managing identities, their relationships, and verification workflows within the Composable Information Machine (CIM) system. It serves as the central orchestration layer for identity-related operations across all domains.

## Status: ✅ COMPLETE - ECS Architecture

The domain has been fully refactored to use Entity Component System (ECS) architecture with Bevy ECS, focusing on relationships and workflows while delegating data management to appropriate domains.

## Architecture

### Domain Focus

The Identity Domain specializes in:
- **Identity Relationships** - Managing connections between identities (employment, membership, partnerships)
- **Verification Workflows** - Orchestrating multi-step verification processes
- **Cross-Domain Projections** - Maintaining references to entities in other domains
- **Identity Claims** - Managing verifiable claims about identities

### Domain Boundaries

The Identity Domain delegates to other domains:
- **Person Details** → `cim-domain-person`
- **Organization Details** → `cim-domain-organization`
- **Authentication** → `cim-domain-policy`
- **Cryptography** → `cim-security`
- **Key Management** → `cim-keys`

## Quick Start

```rust
use cim_domain_identity::*;
use bevy_ecs::prelude::*;

// Create an identity
let command = CreateIdentityCommand {
    identity_type: IdentityType::Person,
    external_reference: person_id.to_string(),
    initial_verification_level: VerificationLevel::Unverified,
    claims: vec![],
};

// Establish a relationship
let relationship = EstablishRelationshipCommand {
    source_identity: employee_id,
    target_identity: company_id,
    relationship_type: RelationshipType::EmployedBy,
    metadata: HashMap::new(),
    expires_at: None,
};

// Start verification
let verification = StartVerificationCommand {
    identity_id,
    verification_method: VerificationMethod::Email,
    initiated_by: admin_id,
};
```

## Core Components

### Components
- `IdentityEntity` - Core identity information
- `IdentityRelationship` - Connections between identities
- `IdentityWorkflow` - Multi-step process state
- `IdentityVerification` - Verification status
- `IdentityClaim` - Verifiable claims
- `IdentityProjection` - Cross-domain references

### Systems
- **Lifecycle Systems** - Create, update, merge, archive identities
- **Relationship Systems** - Establish, validate, traverse relationships
- **Workflow Systems** - Start, process, complete workflows
- **Verification Systems** - Manage verification processes
- **Projection Systems** - Synchronize cross-domain data

### Value Objects
- `IdentityType` - Person, Organization, System, Service
- `VerificationLevel` - Unverified, Basic, Enhanced, Full
- `RelationshipType` - EmployedBy, MemberOf, PartnerOf, etc.
- `WorkflowType` - Verification, Onboarding, Migration
- `VerificationMethod` - Email, Phone, Document, Biometric

## Documentation

- [API Documentation](doc/api/identity-domain-api.md) - Complete API reference
- [Developer Guide](../doc/guides/identity-domain-developer-guide.md) - Architecture and best practices
- [User Stories](../doc/user-stories/identity-domain-stories.md) - Business requirements
- [Example Application](../examples/identity_management_demo.rs) - Working demo

## Integration

### Event-Driven Architecture

The domain communicates through events:
```rust
// Incoming events from other domains
PersonCreated → CreateIdentityCommand
OrganizationCreated → CreateIdentityCommand

// Outgoing events to other domains
IdentityCreated → Notification to relevant domains
VerificationCompleted → Update verification status
RelationshipEstablished → Sync with graph domain
```

### NATS Messaging

All events are published to NATS for cross-domain communication:
- `identity.created` - New identity created
- `identity.updated` - Identity information changed
- `identity.relationship.established` - New relationship
- `identity.verification.completed` - Verification finished

## Testing

Run tests with:
```bash
cargo test -p cim-domain-identity
```

The domain includes:
- Unit tests for all systems
- Integration tests for workflows
- Property-based tests for invariants
- Performance benchmarks

## Performance

- Identity creation: < 100ms
- Relationship traversal: < 500ms for 3 degrees
- Claim verification: < 200ms
- Supports 1M+ active identities
- Handles 10K+ relationships per identity

## Migration from Legacy

If migrating from the old repository-based identity domain:

1. Map existing entities to ECS components
2. Convert repository calls to commands
3. Replace direct queries with event subscriptions
4. Update cross-domain integrations

See the [Migration Guide](doc/api/identity-domain-api.md#migration-guide) for details.

## Contributing

When contributing to the Identity Domain:

1. Follow ECS patterns - components for data, systems for behavior
2. Use commands for all state changes
3. Emit events for cross-domain communication
4. Write tests for new functionality
5. Update documentation

## License

Part of the Composable Information Machine (CIM) project. 