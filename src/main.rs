#[macro_use]
pub extern crate serde_derive;
pub extern crate failure_derive;
pub extern crate handlebars;
pub extern crate serde_json;

mod dockerfiles;
mod generate;
mod generator_settings;
mod template_parser;
mod utils;

use crate::{
    dockerfiles::Dockerfile,
    generate::{load_config, Generate},
    utils::cmd,
    utils::error,
    utils::success,
};
use seahorse::{App, Command, Context, Flag, FlagType};

pub fn main() {
    let args: Vec<String> = std::env::args().collect();

    let app = App::new(cmd(env!("CARGO_PKG_NAME")))
        .description(cmd("generate php 8.0 dockerfile by template"));
    let app = vec!["dev", "prod"].into_iter().fold(app, |app, cmd| {
        app.command(generate_command(cmd.to_string()))
    });

    app.run(args);
}

fn parse_container_type(c: &Context) -> String {
    let flag = c.string_flag("type").unwrap();
    for x in load_config().container_types.iter().map(|a| a.0) {
        if *x == flag {
            return flag;
        }
    }
    format!("undefined {} - check settings for available types", flag)
}

fn generate_command(command_type: String) -> Command {
    Command::new(&command_type)
        .description(cmd(&format!("generate {}-php dockerfile", command_type)))
        .alias(cmd(String::from(command_type.chars().nth(0).unwrap()).as_str()))
        .usage(cmd(&format!("cli {}", command_type)))
        .action(|c: &Context| {
            let run_type = std::env::args().collect::<Vec<String>>();
            let run_type = run_type.get(1);
            let container_type = parse_container_type(c);
            if !container_type.starts_with("undefined ") {
                let dockerfile: Dockerfile = Generate::new(Option::from(container_type), run_type);
                return match dockerfile.to_file() {
                    Err(e) => error(format!("{:?}", e)),
                    _ => success("Successfully generated file"),
                };
            }
            error(container_type)
        })
        .flag(
            Flag::new("type", FlagType::String)
                .description(cmd("Build either a debian-based or a alpine-based image (--type=debian or --type=alpine)"))
                .alias("t")
        )
}
