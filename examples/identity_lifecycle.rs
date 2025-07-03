//! Identity Lifecycle Example
//!
//! This example demonstrates:
//! - Creating identities for users and organizations
//! - Managing identity relationships
//! - Identity verification workflows
//! - Identity lifecycle transitions

use cim_domain_identity::{
    aggregate::IdentityAggregate,
    commands::{CreateIdentity, EstablishRelationship, UpdateIdentityStatus, VerifyIdentity},
    events::{IdentityCreated, IdentityVerified, RelationshipEstablished, StatusUpdated},
    handlers::IdentityCommandHandler,
    queries::{GetIdentity, GetRelationships, IdentityQueryHandler},
    value_objects::{
        IdentityId, IdentityStatus, IdentityType, RelationshipType, VerificationMethod,
    },
};
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CIM Identity Domain Example ===\n");

    // Initialize handlers
    let command_handler = IdentityCommandHandler::new();
    let query_handler = IdentityQueryHandler::new();

    // Step 1: Create user identity
    println!("1. Creating user identity...");
    let user_id = IdentityId::new();
    let create_user = CreateIdentity {
        identity_id: user_id.clone(),
        identity_type: IdentityType::User,
        attributes: {
            let mut attrs = HashMap::new();
            attrs.insert("username".to_string(), "alice.smith".to_string());
            attrs.insert("email".to_string(), "alice@example.com".to_string());
            attrs
        },
    };

    let events = command_handler.handle(create_user).await?;
    println!("   User identity created! Events: {:?}\n", events.len());

    // Step 2: Create organization identity
    println!("2. Creating organization identity...");
    let org_id = IdentityId::new();
    let create_org = CreateIdentity {
        identity_id: org_id.clone(),
        identity_type: IdentityType::Organization,
        attributes: {
            let mut attrs = HashMap::new();
            attrs.insert("name".to_string(), "TechCorp Inc.".to_string());
            attrs.insert("domain".to_string(), "techcorp.com".to_string());
            attrs
        },
    };

    let events = command_handler.handle(create_org).await?;
    println!(
        "   Organization identity created! Events: {:?}\n",
        events.len()
    );

    // Step 3: Establish employment relationship
    println!("3. Establishing employment relationship...");
    let establish_employment = EstablishRelationship {
        source_id: user_id.clone(),
        target_id: org_id.clone(),
        relationship_type: RelationshipType::EmployedBy,
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("role".to_string(), "Software Engineer".to_string());
            meta.insert("department".to_string(), "Engineering".to_string());
            meta
        },
    };

    let events = command_handler.handle(establish_employment).await?;
    println!(
        "   Employment relationship established! Events: {:?}\n",
        events.len()
    );

    // Step 4: Create another user identity
    println!("4. Creating manager identity...");
    let manager_id = IdentityId::new();
    let create_manager = CreateIdentity {
        identity_id: manager_id.clone(),
        identity_type: IdentityType::User,
        attributes: {
            let mut attrs = HashMap::new();
            attrs.insert("username".to_string(), "bob.manager".to_string());
            attrs.insert("email".to_string(), "bob@example.com".to_string());
            attrs
        },
    };

    let events = command_handler.handle(create_manager).await?;
    println!("   Manager identity created! Events: {:?}\n", events.len());

    // Step 5: Establish manager relationship
    println!("5. Establishing manager relationship...");
    let establish_manager = EstablishRelationship {
        source_id: user_id.clone(),
        target_id: manager_id.clone(),
        relationship_type: RelationshipType::ManagedBy,
        metadata: HashMap::new(),
    };

    let events = command_handler.handle(establish_manager).await?;
    println!(
        "   Manager relationship established! Events: {:?}\n",
        events.len()
    );

    // Step 6: Verify user identity
    println!("6. Verifying user identity...");
    let verify_user = VerifyIdentity {
        identity_id: user_id.clone(),
        verification_method: VerificationMethod::Email,
        verification_data: {
            let mut data = HashMap::new();
            data.insert("token".to_string(), "abc123".to_string());
            data.insert("timestamp".to_string(), "2025-01-26T10:00:00Z".to_string());
            data
        },
    };

    let events = command_handler.handle(verify_user).await?;
    println!("   Identity verified! Events: {:?}\n", events.len());

    // Step 7: Update identity status
    println!("7. Activating user identity...");
    let activate_user = UpdateIdentityStatus {
        identity_id: user_id.clone(),
        new_status: IdentityStatus::Active,
        reason: Some("Email verification completed".to_string()),
    };

    let events = command_handler.handle(activate_user).await?;
    println!("   Identity activated! Events: {:?}\n", events.len());

    // Step 8: Query identity details
    println!("8. Retrieving identity details...");
    let get_identity = GetIdentity {
        identity_id: user_id.clone(),
        include_relationships: true,
        include_verification_history: true,
    };

    let identity = query_handler.handle(get_identity).await?;
    println!("   Identity: {:?}", identity.identity_type);
    println!("   Status: {:?}", identity.status);
    println!("   Verified: {identity.is_verified}");
    println!("   Attributes: {identity.attributes.len(} items\n"));

    // Step 9: Query relationships
    println!("9. Retrieving relationships...");
    let get_relationships = GetRelationships {
        identity_id: user_id.clone(),
        relationship_types: None, // Get all types
    };

    let relationships = query_handler.handle(get_relationships).await?;
    println!("   User has {relationships.len(} relationships:"));

    for rel in relationships {
        println!("   - {:?} with {rel.relationship_type}", rel.target_id);
    }

    // Step 10: Create service identity
    println!("\n10. Creating service identity...");
    let service_id = IdentityId::new();
    let create_service = CreateIdentity {
        identity_id: service_id.clone(),
        identity_type: IdentityType::Service,
        attributes: {
            let mut attrs = HashMap::new();
            attrs.insert("service_name".to_string(), "api-gateway".to_string());
            attrs.insert("owner".to_string(), org_id.to_string());
            attrs
        },
    };

    let events = command_handler.handle(create_service).await?;
    println!("   Service identity created! Events: {:?}", events.len());

    // Summary
    println!("\n=== Identity Lifecycle Summary ===");
    println!("✓ Created 3 identities (User, Organization, Service)");
    println!("✓ Established 2 relationships (Employment, Management)");
    println!("✓ Verified and activated user identity");
    println!("✓ Demonstrated query capabilities");

    println!("\n=== Example completed successfully! ===");
    Ok(())
}

// Helper extension traits for demo
trait IdentityHelpers {
    fn is_verified(&self) -> bool;
}

// Note: In real implementation, these would be part of the domain
impl IdentityHelpers for cim_domain_identity::value_objects::Identity {
    fn is_verified(&self) -> bool {
        matches!(self.verification_status, Some(VerificationStatus::Verified))
    }
}
