//! Identity Aggregate
//!
//! The IdentityAggregate enforces business rules and invariants for identity operations.
//! It works with ECS components and systems to maintain consistency.

use crate::{commands::*, components::*, events::*, IdentityError, IdentityResult};
use bevy_ecs::prelude::*;
use std::collections::HashMap;

/// Identity Aggregate that enforces business rules
///
/// This aggregate doesn't store state directly but validates operations
/// and ensures business invariants are maintained across ECS components.
pub struct IdentityAggregate;

impl IdentityAggregate {
    /// Validate identity creation
    pub fn validate_create(
        _command: &CreateIdentityCommand,
        _existing_identities: &[IdentityEntity],
    ) -> IdentityResult<()> {
        // Business rule: Cannot create duplicate identities with same claims
        // This would check for existing identities with same email/phone
        // Implementation depends on specific business rules

        Ok(())
    }

    /// Validate identity update
    pub fn validate_update(
        identity: &IdentityEntity,
        _command: &UpdateIdentityCommand,
    ) -> IdentityResult<()> {
        // Business rule: Cannot update archived identities
        if matches!(identity.status, IdentityStatus::Archived) {
            return Err(IdentityError::IdentityArchived);
        }

        // Business rule: Cannot update merged identities
        if matches!(identity.status, IdentityStatus::Merged { .. }) {
            return Err(IdentityError::IdentityMerged);
        }

        Ok(())
    }

    /// Validate identity merge
    pub fn validate_merge(
        source: &IdentityEntity,
        target: &IdentityEntity,
        source_verification: &IdentityVerification,
        target_verification: &IdentityVerification,
    ) -> IdentityResult<()> {
        // Business rule: Cannot merge identities of different types
        if source.identity_type != target.identity_type {
            return Err(IdentityError::IncompatibleIdentityTypes);
        }

        // Business rule: Cannot merge into less verified identity
        if source_verification.verification_level > target_verification.verification_level {
            return Err(IdentityError::TargetLessVerified);
        }

        // Business rule: Cannot merge archived identities
        if matches!(source.status, IdentityStatus::Archived)
            || matches!(target.status, IdentityStatus::Archived)
        {
            return Err(IdentityError::IdentityArchived);
        }

        Ok(())
    }

    /// Validate identity archive
    pub fn validate_archive(
        identity: &IdentityEntity,
        active_relationships: usize,
        force: bool,
    ) -> IdentityResult<()> {
        // Business rule: Cannot archive identity with active relationships unless forced
        if active_relationships > 0 && !force {
            return Err(IdentityError::HasActiveRelationships(active_relationships));
        }

        // Business rule: Cannot archive already archived identity
        if matches!(identity.status, IdentityStatus::Archived) {
            return Err(IdentityError::AlreadyArchived);
        }

        Ok(())
    }

    /// Validate relationship establishment
    pub fn validate_relationship(
        from_identity: IdentityId,
        to_identity: IdentityId,
        relationship_type: &RelationshipType,
    ) -> IdentityResult<()> {
        // Business rule: Cannot create self-relationships
        if from_identity == to_identity {
            return Err(IdentityError::SelfRelationship);
        }

        // Business rule: Validate relationship type constraints
        match relationship_type {
            RelationshipType::MemberOf => {
                // Only persons can be members of organizations
                // Would check identity types in real implementation
            }
            RelationshipType::Owns => {
                // Ownership relationships have specific rules
                // Would validate ownership constraints in real implementation
            }
            _ => {}
        }

        Ok(())
    }

    /// Validate workflow start
    pub fn validate_workflow_start(
        identity: &IdentityEntity,
        workflow_type: &WorkflowType,
    ) -> Result<(), IdentityError> {
        // Check identity status
        if !matches!(
            identity.status,
            IdentityStatus::Active | IdentityStatus::Pending
        ) {
            return Err(IdentityError::InvalidOperation(
                "Cannot start workflow on inactive identity".to_string(),
            ));
        }

        // Validate workflow type for identity type
        match workflow_type {
            WorkflowType::Verification => {
                // Verification can be started on any identity
                Ok(())
            }
            WorkflowType::Recovery => {
                // Recovery requires active identity
                if identity.status != IdentityStatus::Active {
                    return Err(IdentityError::InvariantViolation(
                        "Recovery workflow requires active identity".to_string(),
                    ));
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Validate verification level transition
    pub fn validate_verification_transition(
        current_level: VerificationLevel,
        new_level: VerificationLevel,
    ) -> IdentityResult<()> {
        // Business rule: Verification can only increase (no downgrade)
        if new_level < current_level {
            return Err(IdentityError::InvariantViolation(
                "Cannot downgrade verification level".to_string(),
            ));
        }

        // Business rule: Must verify in sequence (can't skip levels)
        let level_diff = (new_level as u8) - (current_level as u8);
        if level_diff > 1 {
            return Err(IdentityError::InvariantViolation(
                "Cannot skip verification levels".to_string(),
            ));
        }

        Ok(())
    }

    /// Calculate aggregate state from components
    pub fn calculate_state(
        identity: &IdentityEntity,
        relationships: &[IdentityRelationship],
        workflows: &[IdentityWorkflow],
        verification: &IdentityVerification,
    ) -> AggregateState {
        AggregateState {
            identity_id: identity.identity_id,
            status: identity.status,
            verification_level: verification.verification_level,
            active_relationships: relationships.len(),
            active_workflows: workflows
                .iter()
                .filter(|w| {
                    matches!(
                        w.status,
                        WorkflowStatus::InProgress
                            | WorkflowStatus::WaitingForInput
                            | WorkflowStatus::WaitingForApproval
                    )
                })
                .count(),
        }
    }
}

/// Aggregate state calculated from components
#[derive(Debug, Clone)]
pub struct AggregateState {
    pub identity_id: IdentityId,
    pub status: IdentityStatus,
    pub verification_level: VerificationLevel,
    pub active_relationships: usize,
    pub active_workflows: usize,
}
