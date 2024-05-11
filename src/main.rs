use clap::{Arg, ArgGroup, ArgMatches, Command};
use context::{Context, Mode};
use extended::ExtendedValidator;
use std::{error::Error, io::ErrorKind, path::Path};
use tes3::esp::Plugin;
use validators::Validator;

mod context;
mod extended;
mod handlers;
mod util;
mod validators;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Command::new("validator")
        .args(&[
            Arg::new("extended")
                .num_args(0)
                .long("extended")
                .help("Run extended checks that require master files instead"),
            Arg::new("names")
                .num_args(0)
                .long("names")
                .help("Report similar NPC and quest names instead"),
            Arg::new("duplicatethreshold")
                .long("duplicate-threshold")
                .default_value("0")
                .value_parser(&str::parse::<f32>)
                .value_name("threshold")
                .help("Squared distance at which two objects with the same id, scale, and orientation are considered duplicates"),
            Arg::new("mode")
                .required(true)
                .value_parser(["PT", "TD", "TR", "Vanilla"]),
            Arg::new("path")
                .num_args(1..)
                .required(true)
                .help("C:/path/to/plugin.esp"),
        ])
        .groups([
            ArgGroup::new("g_validator").args(["duplicatethreshold"]),
            ArgGroup::new("g_extended")
                .args(["extended", "names"])
                .conflicts_with("g_validator"),
        ])
        .get_matches();
    let extended = args.get_flag("extended");
    let names = args.get_flag("names");
    let mut paths = args.get_many::<String>("path").unwrap();
    if extended || names {
        return run_extended(paths.collect(), extended, names);
    }
    if paths.clone().count() > 1 {
        Err("The default validator only takes a single path")?;
    }
    let mode: Mode = args
        .get_one::<String>("mode")
        .unwrap_or(&String::new())
        .into();
    let context = Context::new(mode)?;
    return validate(paths.next().unwrap(), context, &args);
}

fn validate(path: &String, context: Context, args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let plugin = load_plugin(path)?;
    let mut validator = Validator::new(context, args)?;
    validator.validate(&plugin.objects);
    Ok(())
}

fn load_plugin(string: &String) -> Result<Plugin, Box<dyn Error>> {
    let path: &Path = string.as_ref();
    let mut plugin = Plugin::new();
    let result = plugin.load_path(path);
    if let Some(err) = result.err() {
        if err.kind() == ErrorKind::NotFound {
            Err(format!("File not found: {}", path.display()))?;
        }
        Err(err)?;
    }
    return Ok(plugin);
}

fn run_extended(paths: Vec<&String>, extended: bool, names: bool) -> Result<(), Box<dyn Error>> {
    let mut validator = ExtendedValidator::new(extended, names);
    let (plugin_path, master_paths) = paths.split_last().unwrap();
    let plugin = load_plugin(plugin_path)?;
    let mut auto_discovered = Vec::new();
    if let Some(header) = plugin.header() {
        if let Some(masters) = &header.masters {
            for (file, _) in masters {
                auto_discovered.push(file);
            }
        }
    }
    for path in master_paths {
        auto_discovered.retain_mut(|p| !path.eq_ignore_ascii_case(p));
        let master = load_plugin(path)?;
        validator.validate(&master.objects, false);
    }
    if !auto_discovered.is_empty() {
        let path: &Path = plugin_path.as_ref();
        let parent = path.parent().unwrap();
        let mut master = Plugin::new();
        for name in auto_discovered {
            let discovered_path = parent.join(name);
            if master.load_path(discovered_path).is_ok() {
                validator.validate(&master.objects, false);
            }
        }
    }
    validator.validate(&plugin.objects, true);
    Ok(())
}
