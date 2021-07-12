use crate::generator_settings::GeneratorSettings;
use crate::serde_json::Value;
use failure::Fail;
use handlebars::Handlebars;
use handlebars::{
    handlebars_helper, Context, Helper, Output, RenderContext, RenderError, Renderable,
};
use serde::export::fmt::Debug;
use serde::Serialize;
use serde_json::json;
use std::{
    collections::HashMap,
    fs::{read_dir, File},
    io,
    io::prelude::*,
    path::Path,
};

#[derive(Fail, Debug)]
#[fail(display = "An error occurred while parsing the template: {}.", _0)]
pub struct TemplatingError(String);

pub struct Parsing {
    config: GeneratorSettings,
}

struct TemplateFileCollection {
    template_content: HashMap<String, String>,
}

impl TemplateFileCollection {
    pub fn new() -> TemplateFileCollection {
        TemplateFileCollection {
            template_content: HashMap::new(),
        }
    }

    fn read_dirs(dir: &str) -> io::Result<Vec<String>> {
        let path = Path::new(dir);
        let mut collection: Vec<String> = vec![];
        if path.is_dir() {
            for file in read_dir(path)? {
                let file = file?;
                if !file.path().is_dir() {
                    collection.push(file.path().as_os_str().to_str().unwrap().to_string());
                }
            }
        }
        Ok(collection)
    }

    pub fn read_for_dir(&mut self, dir: &str) -> io::Result<()> {
        let collection = TemplateFileCollection::read_dirs(dir)?;
        for file in &collection {
            self.add_file_content(file)?;
        }
        Ok(())
    }

    pub fn read_file(file: &str) -> io::Result<String> {
        let mut handle = File::open(file)?;
        let mut contents = String::new();
        handle.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn add_file_content(&mut self, file: &str) -> io::Result<()> {
        let contents = TemplateFileCollection::read_file(file).unwrap();
        let filename = Path::new(&file).file_name().unwrap();
        let mut filename = String::from(filename.to_str().unwrap());
        filename = filename.replace(".dockerfile.hbs", "").replace(".", "_");
        self.template_content.insert(filename, contents);
        Ok(())
    }
}

fn has_flag(flags: &Value, compare_with: Option<&str>) -> bool {
    let mut has_flag: bool = false;
    flags
        .as_array()
        .ok_or(RenderError::new("Flag-Parsing errored"))
        .iter()
        .for_each(|element| {
            element.to_vec().iter().for_each(|flag| {
                if flag.get(0).is_some() && flag.get(0).unwrap().as_str() == compare_with {
                    has_flag = flag.get(1).unwrap().as_bool().unwrap_or(false);
                }
            });
        });
    has_flag
}

fn flag_helper<'reg, 'rc>(
    h: &Helper<'reg, 'rc>,
    r: &'reg Handlebars,
    context: &'rc Context,
    rc: &mut RenderContext<'reg, 'rc>,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).ok_or(RenderError::new(
        "The flag name is required for checking availability",
    ))?;
    let inverse_param = h.param(1);
    let mut inverse: &str = "default";
    if inverse_param.is_some() {
        inverse = match inverse_param.unwrap().value().as_str() {
            Some(v) => v,
            None => "default",
        }
    }
    let flags = context.data().get("flags").ok_or(RenderError::new(
        "The flag array has not been provided in the template",
    ))?;
    let mut has_flag = has_flag(flags, param.value().as_str());
    has_flag = if inverse == "inverse" {
        !has_flag
    } else {
        has_flag
    };
    let tmpl = if has_flag { h.template() } else { h.inverse() };
    match tmpl {
        Some(ref t) => t.render(r, context, rc, out),
        None => Ok(()),
    }
}

fn flag_render_helper<'reg, 'rc>(
    h: &Helper<'reg, 'rc>,
    _: &'reg Handlebars,
    context: &'rc Context,
    _: &mut RenderContext<'reg, 'rc>,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).ok_or(RenderError::new(
        "The flag name is required for checking availability",
    ))?;
    let flags = context.data().get("flags").ok_or(RenderError::new(
        "The flag array has not been provided in the template",
    ))?;
    let has_flag: bool = has_flag(flags, param.value().as_str());
    if has_flag {
        out.write("true");
    } else {
        out.write("false");
    }
    Ok(())
}

impl Parsing {
    pub fn new(config: GeneratorSettings) -> Parsing {
        Parsing { config }
    }

    fn get_error(&self, message: &str) -> Box<TemplatingError> {
        Box::new(TemplatingError(String::from(message)))
    }

    pub fn render<S>(&self, context: &S) -> Result<String, Box<TemplatingError>>
    where
        S: Serialize + Debug,
    {
        let mut template = Handlebars::new();
        handlebars_helper!(not: |a: bool| !a);
        template.register_helper("not", Box::new(not));
        template.register_helper("if_flag", Box::new(flag_helper));
        template.register_helper("flag", Box::new(flag_render_helper));
        let mut collection = TemplateFileCollection::new();
        match collection.read_for_dir(&self.config.settings.template_path) {
            Ok(_r) => (),
            Err(_e) => return Err(self.get_error("Failed parsing template directory")),
        };
        for file in &collection.template_content {
            match template.register_template_string(file.0, file.1) {
                Ok(_r) => (),
                Err(_e) => {
                    return Err(self.get_error(&format!(
                        "Failed parsing template directory for dockerfile {:?} {:?}",
                        _e, context
                    )));
                }
            };
        }

        return match template.render(&self.config.settings.base_template, &json!(context)) {
            Err(e) => {
                println!("{}", e);
                Err(self.get_error("Rendering Template failed"))
            }
            Ok(r) => Ok(r),
        };
    }
}
