use config::{Config, ConfigError};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub template_path: String,
    pub output_path: String,
    pub base_template: String,
    pub available_flags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ContainerType {
    pub from: String,
    pub flags: Option<Vec<String>>,
    pub envs: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct GeneratorSettings {
    pub settings: Settings,
    pub container_types: HashMap<String, ContainerType>,
}

impl GeneratorSettings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut settings: Config = Config::new();
        settings.merge(config::File::with_name("settings"))?;
        settings.try_into::<GeneratorSettings>()
    }
}
