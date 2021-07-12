use crate::generator_settings::GeneratorSettings;
use crate::template_parser::Parsing;
use serde::export::fmt::Debug;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn load_config() -> GeneratorSettings {
    GeneratorSettings::new().unwrap()
}

pub trait Generate: Serialize + Debug {
    fn new(container_type: Option<String>, run_type: Option<&String>) -> Self;

    fn generate(&self) -> Result<String, Box<dyn failure::Fail>> {
        match Parsing::new(load_config()).render(&self) {
            Err(e) => Err(e),
            Ok(r) => Ok(r),
        }
    }

    fn to_file(&self) -> Result<(), failure::Error> {
        let filename: &str = &load_config().settings.output_path;
        if Path::new(filename).exists() {
            fs::remove_file(filename)?;
        }
        let mut file = File::create(filename)?;
        let result = self.generate()?;
        file.write_all(result.as_bytes())?;
        Ok(())
    }
}
