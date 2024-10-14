use crate::dag::*;
// Define the PDP trait

pub trait PDP {
    fn evaluate(&self, request: AccessRequest) -> AccessDecision;
}

// Structures representing access requests and decisions
pub struct AccessRequest {
    pub user_id: String,
    pub object_id: String,
    pub operation: String,
}

pub enum AccessDecision {
    Permit,
    Deny,
}
