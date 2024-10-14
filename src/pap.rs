// Define the PAP trait
pub trait PAP {
    fn create_policy(&mut self, policy: AccessPolicy);
    fn delete_policy(&mut self, policy_id: String);
    fn update_policy(&mut self, policy: AccessPolicy);
}

// Structure representing an access policy
pub struct AccessPolicy {
    pub policy_id: String,
    pub user_attributes: Vec<String>,
    pub object_attributes: Vec<String>,
    pub operations: Vec<String>,
}

// Implementation for PAP
pub struct PolicyAdminPoint {
    pub policies: Vec<AccessPolicy>,
}

impl PAP for PolicyAdminPoint {
    fn create_policy(&mut self, policy: AccessPolicy) {
        self.policies.push(policy);
    }

    fn delete_policy(&mut self, policy_id: String) {
        self.policies.retain(|p| p.policy_id != policy_id);
    }

    fn update_policy(&mut self, policy: AccessPolicy) {
        if let Some(existing_policy) = self
            .policies
            .iter_mut()
            .find(|p| p.policy_id == policy.policy_id)
        {
            *existing_policy = policy;
        }
    }
}
