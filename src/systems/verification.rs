//! Identity verification systems

use bevy_ecs::prelude::*;
use crate::{
    components::*,
    events::*,
    commands::*,
};

/// System to start identity verification
pub fn start_verification_system(
    mut commands: Commands,
    mut events: EventReader<StartVerificationCommand>,
    mut started_events: EventWriter<VerificationStarted>,
    identities: Query<(&IdentityEntity, &IdentityVerification)>,
) {
    for event in events.read() {
        // Find identity to verify
        let identity_data = identities.iter()
            .find(|(e, _)| e.identity_id == event.identity_id);

        if let Some((identity, current_verification)) = identity_data {
            // Check if verification is already at max level
            if current_verification.verification_level == VerificationLevel::FullyVerified {
                continue;
            }

            // Start verification workflow based on method
            match &event.verification_method {
                VerificationMethod::EmailLink | VerificationMethod::EmailCode => {
                    // Would trigger email verification workflow
                }
                VerificationMethod::SmsCode | VerificationMethod::PhoneCall => {
                    // Would trigger phone verification workflow
                }
                VerificationMethod::DocumentUpload => {
                    // Would trigger document verification workflow
                }
                VerificationMethod::VideoCall | VerificationMethod::InPerson => {
                    // Would trigger manual verification workflow
                }
                VerificationMethod::ThirdPartyService { service } => {
                    // Would integrate with external service
                }
            }

            // Emit started event
            started_events.send(VerificationStarted {
                identity_id: event.identity_id,
                verification_method: event.verification_method.clone(),
                initiated_by: event.initiated_by,
                started_at: chrono::Utc::now(),
            });
        }
    }
}

/// System to process verification results
pub fn process_verification_system(
    mut commands: Commands,
    mut events: EventReader<CompleteVerificationCommand>,
    mut completed_events: EventWriter<VerificationCompleted>,
    mut identities: Query<(&IdentityEntity, &mut IdentityVerification)>,
) {
    for event in events.read() {
        for (identity, mut verification) in identities.iter_mut() {
            if identity.identity_id == event.identity_id {
                if event.verification_result {
                    // Update verification level
                    let old_level = verification.verification_level;
                    verification.verification_level = event.verification_level;
                    verification.verified_at = Some(chrono::Utc::now());
                    verification.verified_by = Some(event.verified_by);

                    // Update identity status if pending
                    if matches!(identity.status, IdentityStatus::Pending) {
                        // Would update to Active status
                    }

                    // Emit completed event
                    completed_events.send(VerificationCompleted {
                        identity_id: event.identity_id,
                        verification_successful: true,
                        new_verification_level: event.verification_level,
                        verified_by: event.verified_by,
                        completed_at: chrono::Utc::now(),
                    });
                } else {
                    // Verification failed
                    completed_events.send(VerificationCompleted {
                        identity_id: event.identity_id,
                        verification_successful: false,
                        new_verification_level: verification.verification_level,
                        verified_by: event.verified_by,
                        completed_at: chrono::Utc::now(),
                    });
                }
            }
        }
    }
}

/// System to complete verification workflows
pub fn complete_verification_system(
    mut commands: Commands,
    verifications: Query<(&IdentityEntity, &IdentityVerification, &IdentityWorkflow)>,
    mut workflow_events: EventWriter<WorkflowCompleted>,
) {
    for (identity, verification, workflow) in verifications.iter() {
        // Check if verification workflow is complete
        if matches!(workflow.workflow_type, IdentityWorkflowType::Verification) {
            match workflow.current_state.status {
                WorkflowStatus::Completed => {
                    // Verification workflow completed successfully
                    workflow_events.send(WorkflowCompleted {
                        workflow_id: workflow.workflow_id,
                        identity_id: identity.identity_id,
                        workflow_type: workflow.workflow_type.clone(),
                        final_status: WorkflowStatus::Completed,
                        completed_at: chrono::Utc::now(),
                    });
                }
                WorkflowStatus::Failed | WorkflowStatus::Cancelled => {
                    // Verification workflow failed
                    workflow_events.send(WorkflowCompleted {
                        workflow_id: workflow.workflow_id,
                        identity_id: identity.identity_id,
                        workflow_type: workflow.workflow_type.clone(),
                        final_status: workflow.current_state.status,
                        completed_at: chrono::Utc::now(),
                    });
                }
                _ => {}
            }
        }
    }
}

/// System to handle verification claim updates
pub fn update_verification_claims_system(
    mut commands: Commands,
    verifications: Query<(&IdentityEntity, &IdentityVerification)>,
    mut claims: Query<&mut IdentityClaim>,
) {
    for (identity, verification) in verifications.iter() {
        // Update claim verification status based on verification level
        for mut claim in claims.iter_mut() {
            match verification.verification_level {
                VerificationLevel::EmailVerified => {
                    if matches!(claim.claim_type, ClaimType::Email) {
                        claim.verified = true;
                    }
                }
                VerificationLevel::PhoneVerified => {
                    if matches!(claim.claim_type, ClaimType::Email | ClaimType::Phone) {
                        claim.verified = true;
                    }
                }
                VerificationLevel::DocumentVerified => {
                    if matches!(
                        claim.claim_type,
                        ClaimType::Email | ClaimType::Phone | ClaimType::Name | ClaimType::DateOfBirth
                    ) {
                        claim.verified = true;
                    }
                }
                VerificationLevel::InPersonVerified | VerificationLevel::FullyVerified => {
                    // All claims verified
                    claim.verified = true;
                }
                _ => {}
            }
        }
    }
} 