// Define the PIP trait
pub trait PIP {
    fn get_user_attributes(&self, user_id: &str) -> Vec<String>;
    fn get_object_attributes(&self, object_id: &str) -> Vec<String>;
}

// Implementation for PIP
pub struct PolicyInformationPoint {
    pub user_attributes_store: std::collections::HashMap<String, Vec<String>>,
    pub object_attributes_store: std::collections::HashMap<String, Vec<String>>,
}

impl PIP for PolicyInformationPoint {
    fn get_user_attributes(&self, user_id: &str) -> Vec<String> {
        self.user_attributes_store
            .get(user_id)
            .cloned()
            .unwrap_or_else(Vec::new)
    }

    fn get_object_attributes(&self, object_id: &str) -> Vec<String> {
        self.object_attributes_store
            .get(object_id)
            .cloned()
            .unwrap_or_else(Vec::new)
    }
}
