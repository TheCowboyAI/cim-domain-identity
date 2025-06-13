//! Conceptual dimensions for the Identity context

use cim_conceptual_core::QualityDimension;

/// Identity-specific conceptual dimensions
pub struct IdentityDimensions;

impl IdentityDimensions {
    /// Trust level dimension (0.0 = unverified, 1.0 = fully verified)
    pub fn trust_level() -> QualityDimension {
        let mut dim = QualityDimension::continuous("trust_level".to_string(), 0.0, 1.0);
        dim.description = Some("Level of identity verification".to_string());
        dim
    }

    /// Activity level dimension (0.0 = inactive, 1.0 = highly active)
    pub fn activity_level() -> QualityDimension {
        let mut dim = QualityDimension::continuous("activity_level".to_string(), 0.0, 1.0);
        dim.description = Some("Level of activity in the system".to_string());
        dim
    }

    /// Connectivity dimension (number of relationships normalized)
    pub fn connectivity() -> QualityDimension {
        let mut dim = QualityDimension::continuous("connectivity".to_string(), 0.0, 1.0);
        dim.description = Some("Normalized number of relationships".to_string());
        dim
    }

    /// Organization size dimension (log scale)
    pub fn organization_size() -> QualityDimension {
        let mut dim = QualityDimension::continuous("organization_size".to_string(), 0.0, 10.0);
        dim.description = Some("Organization size on log10 scale".to_string());
        dim
    }

    /// Domain influence dimension
    pub fn domain_influence() -> QualityDimension {
        let mut dim = QualityDimension::continuous("domain_influence".to_string(), 0.0, 1.0);
        dim.description = Some("Influence within the domain".to_string());
        dim
    }

    /// Get all identity dimensions
    pub fn all() -> Vec<QualityDimension> {
        vec![
            Self::trust_level(),
            Self::activity_level(),
            Self::connectivity(),
            Self::organization_size(),
            Self::domain_influence(),
        ]
    }
}
