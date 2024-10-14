struct ResourceAccessPoint {
    resources: std::collections::HashMap<String, String>, // Resource ID to Data
}

impl ResourceAccessPoint {
    fn new() -> Self {
        ResourceAccessPoint {
            resources: std::collections::HashMap::new(),
        }
    }

    // Fetches data from a resource if allowed by the PEP
    fn get_resource_data(&self, resource_id: &str) -> Result<String, String> {
        match self.resources.get(resource_id) {
            Some(data) => Ok(data.clone()),
            None => Err(format!("Resource {} not found", resource_id)),
        }
    }

    // Stores data to a resource
    fn set_resource_data(&mut self, resource_id: &str, data: String) -> Result<(), String> {
        self.resources.insert(resource_id.to_string(), data);
        Ok(())
    }
}
