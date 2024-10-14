use crate::pdp::*;

// Define the PEP trait
pub trait PEP {
    fn enforce(&self, request: AccessRequest) -> Result<(), String>;
}

// Implementation for PEP
pub struct PolicyEnforcementPoint {
    pdp: Box<dyn PDP>, // PDP used to evaluate decisions
}

impl PEP for PolicyEnforcementPoint {
    fn enforce(&self, request: AccessRequest) -> Result<(), String> {
        match self.pdp.evaluate(request) {
            AccessDecision::Permit => Ok(()),
            AccessDecision::Deny => Err("Access Denied".to_string()),
        }
    }
}
