use log::info;
use tokio::runtime::Runtime;
use crate::configuration::context_configuration::ContextConfiguration;

pub trait ApplicationContext {
    fn get_configuration(&self) -> ContextConfiguration;
    fn get_runtime(&self) -> &Option<Runtime>;
}

#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
pub struct DefaultApplicationContext {
    pub(crate) configuration: ContextConfiguration,
}

impl DefaultApplicationContext {}

impl DefaultApplicationContext {
    pub fn get_configuration(&self) -> ContextConfiguration {
        info!("Application is Starting");
        self.configuration.clone()
    }
}
