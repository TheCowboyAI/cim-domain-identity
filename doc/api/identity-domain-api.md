# Identity Domain API Documentation

## Overview

The Identity Domain provides a comprehensive API for managing identities, relationships, workflows, and verifications in the CIM system. It uses an Entity Component System (ECS) architecture with event-driven communication.

## Table of Contents

1. [Core Concepts](#core-concepts)
2. [Components](#components)
3. [Commands](#commands)
4. [Events](#events)
5. [Queries](#queries)
6. [Systems](#systems)
7. [Integration Patterns](#integration-patterns)
8. [Error Handling](#error-handling)

## Core Concepts

### Identity
An identity represents a unique entity in the system that can be a Person, Organization, System, or Service. Each identity has:
- Unique identifier (UUID)
- Type classification
- External reference to domain entity
- Verification status
- Associated claims
- Relationships with other identities

### Relationship
A connection between two identities with:
- Source and target identities
- Relationship type (EmployedBy, MemberOf, etc.)
- Optional metadata
- Establishment timestamp
- Optional expiration

### Workflow
A multi-step process for identity operations like verification:
- Workflow type and current status
- Step progression
- Timeout handling
- State persistence

### Projection
Cross-domain reference linking identities to external entities:
- Target domain and entity ID
- Synchronization status
- Bidirectional navigation

## Components

### IdentityEntity
Core identity information.

```rust
#[derive(Component, Debug, Clone)]
pub struct IdentityEntity {
    pub id: Uuid,
    pub identity_type: IdentityType,
    pub external_reference: String,
    pub created_at: SystemTime,
}
```

### IdentityRelationship
Represents a connection between identities.

```rust
#[derive(Component, Debug, Clone)]
pub struct IdentityRelationship {
    pub id: Uuid,
    pub source_identity: Uuid,
    pub target_identity: Uuid,
    pub relationship_type: RelationshipType,
    pub established_at: SystemTime,
    pub expires_at: Option<SystemTime>,
    pub metadata: HashMap<String, String>,
}
```

### IdentityWorkflow
Workflow state for multi-step processes.

```rust
#[derive(Component, Debug, Clone)]
pub struct IdentityWorkflow {
    pub workflow_id: Uuid,
    pub identity_id: Uuid,
    pub workflow_type: WorkflowType,
    pub status: WorkflowStatus,
    pub current_step: Option<String>,
    pub started_at: SystemTime,
    pub timeout_at: Option<SystemTime>,
}
```

## Commands

### CreateIdentityCommand
Creates a new identity.

```rust
pub struct CreateIdentityCommand {
    pub identity_type: IdentityType,
    pub external_reference: String,
    pub initial_verification_level: VerificationLevel,
    pub claims: Vec<IdentityClaim>,
}
```

### UpdateIdentityCommand
Updates identity information.

```rust
pub struct UpdateIdentityCommand {
    pub identity_id: Uuid,
    pub verification_level: Option<VerificationLevel>,
    pub add_claims: Vec<IdentityClaim>,
    pub remove_claim_ids: Vec<Uuid>,
}
```

## Events

### IdentityCreated
Emitted when a new identity is created.

```rust
#[derive(Event, Debug, Clone)]
pub struct IdentityCreated {
    pub identity_id: Uuid,
    pub identity_type: IdentityType,
    pub external_reference: String,
    pub initial_verification_level: VerificationLevel,
    pub created_at: SystemTime,
}
```

### RelationshipEstablished
Emitted when a relationship is created.

```rust
#[derive(Event, Debug, Clone)]
pub struct RelationshipEstablished {
    pub relationship_id: Uuid,
    pub source_identity: Uuid,
    pub target_identity: Uuid,
    pub relationship_type: RelationshipType,
    pub established_at: SystemTime,
}
```

## Queries

### find_identity_by_id
Retrieves an identity by ID.

```rust
pub fn find_identity_by_id(
    world: &mut World,
    identity_id: Uuid,
) -> Option<IdentityView>
```

### find_identities_by_type
Finds all identities of a specific type.

```rust
pub fn find_identities_by_type(
    world: &mut World,
    identity_type: IdentityType,
) -> Vec<IdentityView>
```

## Systems

### Lifecycle Systems
- `create_identity_system` - Creates new identities
- `update_identity_system` - Updates identity information
- `merge_identities_system` - Merges duplicate identities
- `archive_identity_system` - Archives inactive identities

### Relationship Systems
- `establish_relationship_system` - Creates relationships
- `validate_relationship_system` - Validates relationships
- `traverse_relationships_system` - Traverses relationship graph
- `expire_relationships_system` - Expires time-bound relationships

### Workflow Systems
- `start_workflow_system` - Initiates workflows
- `process_workflow_steps_system` - Advances workflow steps
- `complete_workflow_system` - Completes workflows
- `handle_workflow_timeouts_system` - Handles timeouts

## Integration Patterns

### Event-Driven Integration
Subscribe to identity events for cross-domain integration:

```rust
fn handle_identity_created(
    mut events: EventReader<IdentityCreated>,
    mut person_commands: EventWriter<CreatePersonCommand>,
) {
    for event in events.read() {
        if event.identity_type == IdentityType::Person {
            person_commands.write(CreatePersonCommand {
                person_id: Uuid::parse_str(&event.external_reference).unwrap(),
            });
        }
    }
}
```

### Command Pattern
Send commands to modify identity state:

```rust
commands.write(UpdateIdentityCommand {
    identity_id: id,
    verification_level: Some(VerificationLevel::Enhanced),
    ..Default::default()
});
```

## Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum IdentityError {
    #[error("Identity not found: {0}")]
    IdentityNotFound(Uuid),
    
    #[error("Invalid relationship: {0}")]
    InvalidRelationship(String),
    
    #[error("Workflow error: {0}")]
    WorkflowError(String),
}
```

## Best Practices

1. **Command Validation** - Always validate commands before processing
2. **Event Ordering** - Maintain event order per aggregate
3. **Idempotency** - Make operations idempotent
4. **Timeout Handling** - Set appropriate timeouts for workflows

## Performance Considerations

1. **Query Optimization** - Use specific queries instead of broad scans
2. **Batch Operations** - Process multiple commands together
3. **Event Batching** - Batch events for network efficiency

## Conclusion

The Identity Domain API provides a comprehensive, event-driven approach to identity management with strong typing, clear boundaries, and excellent performance characteristics through the ECS architecture.
