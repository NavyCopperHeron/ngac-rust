#[cfg(test)]
mod tests {
    use ngac_rust::pap::*;
    use ngac_rust::pdp::*;
    use ngac_rust::pep::*;
    use ngac_rust::pip::*;
    #[test]
    fn test_pap_create_policy() {
        let mut pap = PolicyAdminPoint { policies: vec![] };
        let policy = AccessPolicy {
            policy_id: "policy001".to_string(),
            user_attributes: vec!["role=doctor".to_string()],
            object_attributes: vec!["sensitivity=confidential".to_string()],
            operations: vec!["read".to_string()],
        };
        pap.create_policy(policy);
        assert!(!pap.policies.is_empty());
        println!("PAP policy creation test passed.");
    }

    #[test]
    fn test_prp_retrieve_policies() {
        let mut pap = PolicyAdminPoint { policies: vec![] };
        let policy = AccessPolicy {
            policy_id: "policy001".to_string(),
            user_attributes: vec!["role=doctor".to_string()],
            object_attributes: vec!["sensitivity=confidential".to_string()],
            operations: vec!["read".to_string()],
        };
        pap.create_policy(policy.clone());
        let prp = PolicyRetrievalPoint {
            policies: pap.policies.clone(),
        };
        let retrieved_policies = prp.retrieve_policies();
        assert!(retrieved_policies.len() > 0);
        println!("PRP policy retrieval test passed.");
    }

    #[test]
    fn test_pdp_access_decision() {
        let pap = PolicyAdminPoint { policies: vec![] };
        let prp = PolicyRetrievalPoint {
            policies: pap.policies.clone(),
        };
        let pip = PolicyInformationPoint {
            user_attributes_store: std::collections::HashMap::new(),
            object_attributes_store: std::collections::HashMap::new(),
        };
        let pdp = SimplePDP { prp, pip };

        let request = AccessRequest {
            user_id: "user001".to_string(),
            object_id: "object001".to_string(),
            operation: "read".to_string(),
        };
        let decision = pdp.evaluate(request);
        assert_eq!(decision, AccessDecision::Deny); // As no valid policies are added
        println!("PDP access decision test passed.");
    }

    #[test]
    fn test_pep_enforce() {
        let mut pap = PolicyAdminPoint { policies: vec![] };
        let policy = AccessPolicy {
            policy_id: "policy001".to_string(),
            user_attributes: vec!["role=doctor".to_string()],
            object_attributes: vec!["sensitivity=confidential".to_string()],
            operations: vec!["read".to_string()],
        };
        pap.create_policy(policy);

        let prp = PolicyRetrievalPoint {
            policies: pap.policies.clone(),
        };
        let pip = PolicyInformationPoint {
            user_attributes_store: std::collections::HashMap::new(),
            object_attributes_store: std::collections::HashMap::new(),
        };
        let pdp = SimplePDP { prp, pip };
        let pep = PolicyEnforcementPoint { pdp: Box::new(pdp) };

        let request = AccessRequest {
            user_id: "user001".to_string(),
            object_id: "object001".to_string(),
            operation: "read".to_string(),
        };

        match pep.enforce(request) {
            Ok(_) => println!("PEP enforcement test passed."),
            Err(e) => println!("PEP enforcement failed: {}", e),
        }
    }

    #[test]
    fn test_pip_attributes() {
        let mut pip = PolicyInformationPoint {
            user_attributes_store: std::collections::HashMap::new(),
            object_attributes_store: std::collections::HashMap::new(),
        };
        pip.user_attributes_store
            .insert("user001".to_string(), vec!["role=doctor".to_string()]);
        pip.object_attributes_store.insert(
            "object001".to_string(),
            vec!["sensitivity=confidential".to_string()],
        );

        let user_attrs = pip.get_user_attributes("user001");
        let object_attrs = pip.get_object_attributes("object001");

        assert!(user_attrs.contains(&"role=doctor".to_string()));
        assert!(object_attrs.contains(&"sensitivity=confidential".to_string()));
        println!("PIP attributes retrieval test passed.");
    }
}
