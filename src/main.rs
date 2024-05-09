use clap::{Arg, Command};
use context::{Context, Mode};
use std::error::Error;
use tes3::esp::Plugin;
use validators::Validator;

mod context;
mod handlers;
mod util;
mod validators;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Command::new("validator")
        .args(&[
            Arg::new("path")
                .required(true)
                .help("C:/path/to/plugin.esp"),
            Arg::new("mode")
                .hide_default_value(true)
                .required(false)
                .value_parser(["PT", "TD", "TR", "Vanilla"]),
        ])
        .get_matches();
    let mode: Mode = args
        .get_one::<String>("mode")
        .unwrap_or(&String::new())
        .into();
    let context = Context::new(mode)?;
    return validate(args.get_one::<String>("path").unwrap(), context);
}

fn validate(path: &String, context: Context) -> Result<(), Box<dyn Error>> {
    let mut plugin = Plugin::new();
    plugin.load_path(path)?;
    let mut validator = Validator::new(context)?;
    validator.validate(&plugin.objects);
    Ok(())
}
