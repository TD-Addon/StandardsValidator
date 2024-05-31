use clap::{crate_version, Arg, ArgGroup, ArgMatches, Command};
use context::{Context, Mode};
use extended::ExtendedValidator;
use oob::fix_oob;
use std::{error::Error, fs, path::Path};
use tes3::esp::Plugin;
use toml::{Table, Value};
use validators::Validator;

mod context;
mod extended;
mod handlers;
mod oob;
mod util;
mod validators;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Command::new("StandardsValidator")
        .args(&[
            Arg::new("ooboutput")
                .long("fix-out-of-bounds")
                .value_name("output file")
                .help(
                    "Move references that should belong to another cell to that cell \
                and output a new file. Warning: overwrites the output file!",
                ),
            Arg::new("extended")
                .num_args(0)
                .long("extended")
                .help("Run extended checks that require master files instead."),
            Arg::new("names")
                .num_args(0)
                .long("names")
                .help("Report similar NPC and quest names instead."),
            Arg::new("dontautoload")
                .num_args(0)
                .long("disable-master-loading")
                .help(
                    "--extended and --names automatically \
                attempt to load the last <path>'s master files \
                from the same directory if no other <path>s with \
                the same file name are supplied. This flag disables that behaviour.",
                ),
            Arg::new("mininhabitants")
                .value_name("number")
                .default_value("3")
                .value_parser(str::parse::<usize>)
                .long("min-inhabitants")
                .help("Minimum number of inhabitants a dungeon cell should have.")
                .requires("extended"),
            Arg::new("duplicatethreshold")
                .long("duplicate-threshold")
                .default_value("0")
                .value_parser(str::parse::<f32>)
                .value_name("threshold")
                .help(
                    "Squared distance at which two objects with the same id, \
                scale, and orientation are considered duplicates.",
                ),
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
            ArgGroup::new("g_autoload")
                .arg("dontautoload")
                .requires("g_extended"),
            ArgGroup::new("g_oob")
                .arg("ooboutput")
                .conflicts_with_all(["g_validator", "g_extended"]),
        ])
        .version(crate_version!())
        .get_matches();
    let mut paths = args.get_many::<String>("path").unwrap();

    if args.get_flag("extended") || args.get_flag("names") {
        return Ok(run_extended(paths.collect(), &args)?);
    }
    if paths.clone().count() > 1 {
        Err("Multiple paths are only allowed for --extended and --names")?;
    }
    if let Some(output) = args.get_one::<String>("ooboutput") {
        return run_oob_fixes(paths.next().unwrap(), output);
    }

    validate(paths.next().unwrap(), &args)
}

fn create_context(args: &ArgMatches) -> Context {
    let mode = args
        .get_one::<String>("mode")
        .map_or(Mode::None, Mode::from);
    Context::new(mode)
}

fn validate(path: &str, args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let mut context = create_context(args);
    let plugin = load_plugin(path, Some(&mut context))?;
    if context.mode.uses_td() {
        let p: &Path = path.as_ref();
        let _ = load_metadata(&p.parent().unwrap().join("Tamriel_Data.esm"), &mut context);
    }
    let mut validator = Validator::new(context, args)?;
    validator.validate(&plugin.objects);
    Ok(())
}

fn load_metadata(plugin_path: &Path, context: &mut Context) -> Result<(), Box<dyn Error>> {
    let plugin_name: String = plugin_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    let dir = plugin_path.parent().unwrap();
    let meta_path = dir.join(plugin_name + "-metadata.toml");
    let data: Table = toml::from_str(&fs::read_to_string(meta_path)?)?;
    if let Some(deprecated) = data
        .get("tools")
        .and_then(|t| t.get("csse"))
        .and_then(|c| c.get("deprecated"))
        .and_then(|d| d.as_array())
    {
        for id in deprecated.iter().flat_map(Value::as_str) {
            context.deprecated.insert(id.to_ascii_lowercase());
        }
    }
    Ok(())
}

fn load_plugin(p: impl AsRef<Path>, context: Option<&mut Context>) -> Result<Plugin, String> {
    let path: &Path = p.as_ref();
    let mut plugin = Plugin::new();
    let result = plugin.load_path(path);
    if let Some(err) = result.err() {
        return Err(format!("Failed to load {} ({})", path.display(), err));
    }
    if let Some(c) = context {
        let _ = load_metadata(path, c);
    }
    Ok(plugin)
}

fn run_extended(paths: Vec<&String>, args: &ArgMatches) -> Result<(), String> {
    let mut context = create_context(args);
    let mut validator = ExtendedValidator::new(args);
    let (plugin_path, master_paths) = paths.split_last().unwrap();
    let plugin = load_plugin(plugin_path, Some(&mut context))?;
    let mut auto_discovered = Vec::new();
    let autoload = !args.get_flag("dontautoload");
    if autoload {
        if let Some(header) = plugin.header() {
            for (file, _) in &header.masters {
                auto_discovered.push(file);
            }
        }
    }
    for master_path in master_paths {
        let path: &Path = plugin_path.as_ref();
        let master = load_plugin(master_path, Some(&mut context))?;
        if autoload {
            auto_discovered.retain_mut(|p| !path.file_name().unwrap().eq_ignore_ascii_case(p));
        }
        validator.validate(&master.objects, master_path, false, &context);
    }
    if !auto_discovered.is_empty() {
        let path: &Path = plugin_path.as_ref();
        let parent = path.parent().unwrap();
        for name in auto_discovered {
            let discovered_path = parent.join(name);
            let master = load_plugin(discovered_path.as_path(), Some(&mut context))?;
            let file = discovered_path.to_str().unwrap_or("<funky path>");
            validator.validate(&master.objects, file, false, &context);
        }
    }
    validator.validate(&plugin.objects, plugin_path, true, &context);
    Ok(())
}

fn run_oob_fixes(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let mut plugin = load_plugin(input, None)?;
    fix_oob(&mut plugin);
    plugin.save_path(output)?;
    Ok(())
}
