use std::fs::File;
use crate::configuration::configuration_properties::ConfigurationProperties;

#[derive(Debug, Default, Clone)]
pub struct ContextConfiguration {
    pub properties: ConfigurationProperties,
}

impl ContextConfiguration {
    pub fn load_properties() -> Self {
        let resource_path = "resources/application.yml".to_string();
        let f = File::open(resource_path.clone()).unwrap_or_else(|e| panic!("Can't read resources {}, with error {}", resource_path, e));
        let configuration_properties: ConfigurationProperties = serde_yaml::from_reader(f).unwrap();
        ContextConfiguration {
            properties: configuration_properties
        }
    }
}
