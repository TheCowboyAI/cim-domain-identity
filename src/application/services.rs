//! Application services for the Identity context

use crate::domain::{DomainError, DomainResult};
use crate::domain::person::{PersonAggregate, PersonId};
use crate::domain::organization::{OrganizationAggregate, OrganizationId};
use crate::ports::{PersonRepository, OrganizationRepository};
use std::sync::Arc;
use async_trait::async_trait;

/// Service for managing person-organization relationships
pub struct PersonOrganizationService<PR: PersonRepository, OR: OrganizationRepository> {
    person_repo: Arc<PR>,
    org_repo: Arc<OR>,
}

impl<PR: PersonRepository, OR: OrganizationRepository> PersonOrganizationService<PR, OR> {
    pub fn new(person_repo: Arc<PR>, org_repo: Arc<OR>) -> Self {
        Self {
            person_repo,
            org_repo,
        }
    }
    
    /// Add a person to an organization with validation
    pub async fn add_person_to_organization(
        &self,
        person_id: PersonId,
        org_id: OrganizationId,
        role: String,
    ) -> DomainResult<()> {
        // Load both aggregates
        let person = self.person_repo.get(&person_id).await?
            .ok_or(DomainError::PersonNotFound(person_id))?;
        
        let mut organization = self.org_repo.get(&org_id).await?
            .ok_or(DomainError::OrganizationNotFound(org_id))?;
        
        // Validate person is active
        if !person.is_active() {
            return Err(DomainError::PersonNotActive(person_id));
        }
        
        // Add person to organization
        organization.add_member(person_id, role)?;
        
        // Save updated organization
        self.org_repo.save(&organization).await?;
        
        Ok(())
    }
    
    /// Remove a person from an organization
    pub async fn remove_person_from_organization(
        &self,
        person_id: PersonId,
        org_id: OrganizationId,
    ) -> DomainResult<()> {
        // Load organization
        let mut organization = self.org_repo.get(&org_id).await?
            .ok_or(DomainError::OrganizationNotFound(org_id))?;
        
        // Remove person
        organization.remove_member(person_id)?;
        
        // Save updated organization
        self.org_repo.save(&organization).await?;
        
        Ok(())
    }
    
    /// Transfer person between organizations
    pub async fn transfer_person(
        &self,
        person_id: PersonId,
        from_org_id: OrganizationId,
        to_org_id: OrganizationId,
        new_role: String,
    ) -> DomainResult<()> {
        // Load all aggregates
        let person = self.person_repo.get(&person_id).await?
            .ok_or(DomainError::PersonNotFound(person_id))?;
        
        let mut from_org = self.org_repo.get(&from_org_id).await?
            .ok_or(DomainError::OrganizationNotFound(from_org_id))?;
        
        let mut to_org = self.org_repo.get(&to_org_id).await?
            .ok_or(DomainError::OrganizationNotFound(to_org_id))?;
        
        // Validate person is active
        if !person.is_active() {
            return Err(DomainError::PersonNotActive(person_id));
        }
        
        // Remove from source organization
        from_org.remove_member(person_id)?;
        
        // Add to target organization
        to_org.add_member(person_id, new_role)?;
        
        // Save both organizations
        self.org_repo.save(&from_org).await?;
        self.org_repo.save(&to_org).await?;
        
        Ok(())
    }
}

/// Service for managing organization hierarchies
pub struct OrganizationHierarchyService<OR: OrganizationRepository> {
    org_repo: Arc<OR>,
}

impl<OR: OrganizationRepository> OrganizationHierarchyService<OR> {
    pub fn new(org_repo: Arc<OR>) -> Self {
        Self { org_repo }
    }
    
    /// Create a sub-organization
    pub async fn create_sub_organization(
        &self,
        parent_id: OrganizationId,
        name: String,
        org_type: String,
    ) -> DomainResult<OrganizationId> {
        // Load parent organization
        let mut parent = self.org_repo.get(&parent_id).await?
            .ok_or(DomainError::OrganizationNotFound(parent_id))?;
        
        // Create new organization
        let sub_org_id = OrganizationId::new();
        let mut sub_org = OrganizationAggregate::new(sub_org_id, name, org_type);
        
        // Set parent relationship
        sub_org.set_parent(parent_id)?;
        parent.add_sub_unit(sub_org_id)?;
        
        // Save both organizations
        self.org_repo.save(&sub_org).await?;
        self.org_repo.save(&parent).await?;
        
        Ok(sub_org_id)
    }
    
    /// Move organization to new parent
    pub async fn move_organization(
        &self,
        org_id: OrganizationId,
        new_parent_id: Option<OrganizationId>,
    ) -> DomainResult<()> {
        // Load organization
        let mut organization = self.org_repo.get(&org_id).await?
            .ok_or(DomainError::OrganizationNotFound(org_id))?;
        
        // Get current parent
        let current_parent_id = organization.parent_id();
        
        // Remove from current parent if exists
        if let Some(parent_id) = current_parent_id {
            let mut parent = self.org_repo.get(&parent_id).await?
                .ok_or(DomainError::OrganizationNotFound(parent_id))?;
            parent.remove_sub_unit(org_id)?;
            self.org_repo.save(&parent).await?;
        }
        
        // Add to new parent if specified
        if let Some(new_parent_id) = new_parent_id {
            let mut new_parent = self.org_repo.get(&new_parent_id).await?
                .ok_or(DomainError::OrganizationNotFound(new_parent_id))?;
            new_parent.add_sub_unit(org_id)?;
            organization.set_parent(new_parent_id)?;
            self.org_repo.save(&new_parent).await?;
        } else {
            organization.remove_parent()?;
        }
        
        // Save organization
        self.org_repo.save(&organization).await?;
        
        Ok(())
    }
}

/// Service for bulk operations
pub struct BulkOperationService<PR: PersonRepository, OR: OrganizationRepository> {
    person_repo: Arc<PR>,
    org_repo: Arc<OR>,
}

impl<PR: PersonRepository, OR: OrganizationRepository> BulkOperationService<PR, OR> {
    pub fn new(person_repo: Arc<PR>, org_repo: Arc<OR>) -> Self {
        Self {
            person_repo,
            org_repo,
        }
    }
    
    /// Bulk import people into an organization
    pub async fn bulk_import_people(
        &self,
        org_id: OrganizationId,
        people: Vec<(String, String, String)>, // (given_name, family_name, role)
    ) -> DomainResult<Vec<PersonId>> {
        // Load organization
        let mut organization = self.org_repo.get(&org_id).await?
            .ok_or(DomainError::OrganizationNotFound(org_id))?;
        
        let mut created_ids = Vec::new();
        
        for (given_name, family_name, role) in people {
            // Create person
            let person_id = PersonId::new();
            let person = PersonAggregate::new(person_id, given_name, family_name);
            
            // Save person
            self.person_repo.save(&person).await?;
            
            // Add to organization
            organization.add_member(person_id, role)?;
            
            created_ids.push(person_id);
        }
        
        // Save organization with all new members
        self.org_repo.save(&organization).await?;
        
        Ok(created_ids)
    }
    
    /// Deactivate all members of an organization
    pub async fn deactivate_organization_members(
        &self,
        org_id: OrganizationId,
        reason: String,
    ) -> DomainResult<usize> {
        // Load organization
        let organization = self.org_repo.get(&org_id).await?
            .ok_or(DomainError::OrganizationNotFound(org_id))?;
        
        let member_ids = organization.member_ids();
        let mut deactivated_count = 0;
        
        for person_id in member_ids {
            if let Some(mut person) = self.person_repo.get(&person_id).await? {
                if person.is_active() {
                    person.deactivate(reason.clone())?;
                    self.person_repo.save(&person).await?;
                    deactivated_count += 1;
                }
            }
        }
        
        Ok(deactivated_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::{InMemoryPersonRepository, InMemoryOrganizationRepository};
    
    #[tokio::test]
    async fn test_add_person_to_organization() {
        let person_repo = Arc::new(InMemoryPersonRepository::new());
        let org_repo = Arc::new(InMemoryOrganizationRepository::new());
        
        let service = PersonOrganizationService::new(person_repo.clone(), org_repo.clone());
        
        // Create person and organization
        let person_id = PersonId::new();
        let person = PersonAggregate::new(person_id, "John".to_string(), "Doe".to_string());
        person_repo.save(&person).await.unwrap();
        
        let org_id = OrganizationId::new();
        let org = OrganizationAggregate::new(org_id, "Acme Corp".to_string(), "Company".to_string());
        org_repo.save(&org).await.unwrap();
        
        // Add person to organization
        service.add_person_to_organization(person_id, org_id, "Engineer".to_string()).await.unwrap();
        
        // Verify
        let updated_org = org_repo.get(&org_id).await.unwrap().unwrap();
        assert!(updated_org.has_member(person_id));
    }
    
    #[tokio::test]
    async fn test_organization_hierarchy() {
        let org_repo = Arc::new(InMemoryOrganizationRepository::new());
        let service = OrganizationHierarchyService::new(org_repo.clone());
        
        // Create parent organization
        let parent_id = OrganizationId::new();
        let parent = OrganizationAggregate::new(parent_id, "Parent Corp".to_string(), "Company".to_string());
        org_repo.save(&parent).await.unwrap();
        
        // Create sub-organization
        let sub_id = service.create_sub_organization(
            parent_id,
            "Sub Division".to_string(),
            "Division".to_string()
        ).await.unwrap();
        
        // Verify relationships
        let updated_parent = org_repo.get(&parent_id).await.unwrap().unwrap();
        assert!(updated_parent.has_sub_unit(sub_id));
        
        let sub_org = org_repo.get(&sub_id).await.unwrap().unwrap();
        assert_eq!(sub_org.parent_id(), Some(parent_id));
    }
}
