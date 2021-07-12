use crate::generate::{load_config, Generate};
use crate::generator_settings::Settings;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct Dockerfile {
    pub from: String,
    pub container_type: String,
    pub envs: Vec<(String, String)>,
    pub flags: Vec<(String, bool)>,
    pub is_dev: bool,
}

impl Generate for Dockerfile {
    fn new(container_type: Option<String>, run_type: Option<&String>) -> Dockerfile {
        let config = load_config();
        let container_type: String = container_type.unwrap();
        let available_flags: Vec<String> = config.settings.available_flags;
        let is_dev: bool = run_type.unwrap().parse::<String>().unwrap() == "dev".to_string();
        let values = config.container_types.get(&container_type).unwrap().clone();
        let from: String = values.from.to_string();

        let mut flags: Vec<(String, bool)> = Vec::new();
        for flag in available_flags {
            if values.flags.is_some() && values.flags.as_ref().unwrap().contains(&flag) {
                flags.push((flag.clone(), true));
            } else {
                flags.push((flag.clone(), false));
            }
        }

        let envs: Vec<(String, String)> = Vec::new();
        Dockerfile {
            from,
            container_type,
            envs,
            flags,
            is_dev,
        }
    }
}
