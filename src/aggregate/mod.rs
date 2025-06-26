//! Identity Aggregate
//!
//! The IdentityAggregate enforces business rules and invariants for identity operations.
//! It works with ECS components and systems to maintain consistency.

use bevy_ecs::prelude::*;
use crate::{
    components::*,
    commands::*,
    events::*,
    IdentityError, IdentityResult,
};
use std::collections::HashMap;

/// Identity Aggregate that enforces business rules
///
/// This aggregate doesn't store state directly but validates operations
/// and ensures business invariants are maintained across ECS components.
pub struct IdentityAggregate;

impl IdentityAggregate {
    /// Validate identity creation
    pub fn validate_create(
        command: &CreateIdentityCommand,
        existing_identities: &[IdentityEntity],
    ) -> IdentityResult<()> {
        // Business rule: Cannot create duplicate identities with same claims
        // This would check for existing identities with same email/phone
        // Implementation depends on specific business rules
        
        Ok(())
    }

    /// Validate identity update
    pub fn validate_update(
        identity: &IdentityEntity,
        command: &UpdateIdentityCommand,
    ) -> IdentityResult<()> {
        // Business rule: Cannot transition from Archived to Active
        if matches!(identity.status, IdentityStatus::Archived) {
            if let Some(new_status) = &command.new_status {
                if matches!(new_status, IdentityStatus::Active) {
                    return Err(IdentityError::InvariantViolation(
                        "Cannot reactivate archived identity".to_string()
                    ));
                }
            }
        }

        // Business rule: Cannot transition from Merged status
        if matches!(identity.status, IdentityStatus::Merged { .. }) {
            return Err(IdentityError::InvariantViolation(
                "Cannot update merged identity".to_string()
            ));
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
        // Business rule: Cannot merge different identity types
        if source.identity_type != target.identity_type {
            return Err(IdentityError::MergeNotAllowed(
                "Cannot merge identities of different types".to_string()
            ));
        }

        // Business rule: Cannot merge into pending identity
        if matches!(target.status, IdentityStatus::Pending) {
            return Err(IdentityError::MergeNotAllowed(
                "Cannot merge into pending identity".to_string()
            ));
        }

        // Business rule: Source must be less verified than target
        if source_verification.verification_level > target_verification.verification_level {
            return Err(IdentityError::MergeNotAllowed(
                "Source identity is more verified than target".to_string()
            ));
        }

        Ok(())
    }

    /// Validate identity archival
    pub fn validate_archive(
        identity: &IdentityEntity,
        active_relationships: usize,
        force: bool,
    ) -> IdentityResult<()> {
        // Business rule: Cannot archive identity with active relationships unless forced
        if active_relationships > 0 && !force {
            return Err(IdentityError::ArchiveWithActiveRelationships);
        }

        // Business rule: Cannot archive already archived identity
        if matches!(identity.status, IdentityStatus::Archived) {
            return Err(IdentityError::InvariantViolation(
                "Identity is already archived".to_string()
            ));
        }

        Ok(())
    }

    /// Validate relationship establishment
    pub fn validate_relationship(
        from_identity: &IdentityEntity,
        to_identity: &IdentityEntity,
        relationship_type: &RelationshipType,
    ) -> IdentityResult<()> {
        // Business rule: Both identities must be active
        if !matches!(from_identity.status, IdentityStatus::Active) {
            return Err(IdentityError::InvariantViolation(
                "Source identity must be active".to_string()
            ));
        }

        if !matches!(to_identity.status, IdentityStatus::Active) {
            return Err(IdentityError::InvariantViolation(
                "Target identity must be active".to_string()
            ));
        }

        // Business rule: Validate relationship type based on identity types
        match relationship_type {
            RelationshipType::MemberOf { .. } => {
                // Only Person can be member of Organization
                if !matches!(from_identity.identity_type, IdentityType::Person) ||
                   !matches!(to_identity.identity_type, IdentityType::Organization) {
                    return Err(IdentityError::RelationshipNotAllowed);
                }
            }
            RelationshipType::ManagerOf | RelationshipType::ReportsTo => {
                // Only between persons
                if !matches!(from_identity.identity_type, IdentityType::Person) ||
                   !matches!(to_identity.identity_type, IdentityType::Person) {
                    return Err(IdentityError::RelationshipNotAllowed);
                }
            }
            _ => {
                // Other relationship types may have different rules
            }
        }

        Ok(())
    }

    /// Validate workflow start
    pub fn validate_workflow_start(
        identity: &IdentityEntity,
        workflow_type: &IdentityWorkflowType,
        existing_workflows: &[IdentityWorkflow],
    ) -> IdentityResult<()> {
        // Business rule: Cannot start duplicate active workflow
        let has_active = existing_workflows.iter()
            .any(|w| w.workflow_type == *workflow_type &&
                    matches!(w.current_state.status,
                            WorkflowStatus::InProgress |
                            WorkflowStatus::WaitingForInput |
                            WorkflowStatus::WaitingForApproval));

        if has_active {
            return Err(IdentityError::WorkflowInProgress);
        }

        // Business rule: Some workflows require specific identity status
        match workflow_type {
            IdentityWorkflowType::Verification => {
                if !matches!(identity.status, IdentityStatus::Pending | IdentityStatus::Active) {
                    return Err(IdentityError::InvariantViolation(
                        "Cannot verify identity in current status".to_string()
                    ));
                }
            }
            IdentityWorkflowType::Offboarding => {
                if !matches!(identity.status, IdentityStatus::Active) {
                    return Err(IdentityError::InvariantViolation(
                        "Can only offboard active identities".to_string()
                    ));
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Validate verification level transition
    pub fn validate_verification_transition(
        current_level: VerificationLevel,
        new_level: VerificationLevel,
    ) -> IdentityResult<()> {
        // Business rule: Verification can only increase (no downgrade)
        if new_level < current_level {
            return Err(IdentityError::InvariantViolation(
                "Cannot downgrade verification level".to_string()
            ));
        }

        // Business rule: Must verify in sequence (can't skip levels)
        let level_diff = (new_level as u8) - (current_level as u8);
        if level_diff > 1 {
            return Err(IdentityError::InvariantViolation(
                "Cannot skip verification levels".to_string()
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
            active_workflows: workflows.iter()
                .filter(|w| matches!(w.current_state.status,
                                   WorkflowStatus::InProgress |
                                   WorkflowStatus::WaitingForInput |
                                   WorkflowStatus::WaitingForApproval))
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