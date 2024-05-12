use regex::Regex;
use serde::Deserialize;
use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed=data");
    println!("cargo::rerun-if-changed=build.rs");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    //bodyparts
    write_classes(Path::new(&out_dir).join("gen_classes.rs"))?;
    write_broken(Path::new(&out_dir).join("gen_broken.rs"))?;
    write_mwscript(Path::new(&out_dir).join("gen_mwscript.rs"))?;
    write_projects(Path::new(&out_dir).join("gen_projects.rs"))?;
    write_services(Path::new(&out_dir).join("gen_services.rs"))?;
    write_spells(Path::new(&out_dir).join("gen_spells.rs"))?;
    write_supplies(Path::new(&out_dir).join("gen_supplies.rs"))?;
    write_travel(Path::new(&out_dir).join("gen_travel.rs"))?;
    write_uniques(Path::new(&out_dir).join("gen_uniques.rs"))?;
    return Ok(());
}

#[derive(Deserialize)]
struct ClassData {
    vanilla: String,
    data: String,
}

fn write_string_map_insert(
    file: &mut File,
    map: &str,
    key: String,
    value: &str,
) -> Result<(), io::Error> {
    file.write_all(map.as_bytes())?;
    file.write_all(br##".insert(r#""##)?;
    file.write_all(key.as_bytes())?;
    file.write_all(br##""#, r#""##)?;
    file.write_all(value.as_bytes())?;
    file.write_all(
        br##""#);
    "##,
    )?;
    Ok(())
}

#[must_use]
fn write_classes(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let classes: Vec<ClassData> = serde_json::from_str(include_str!("./data/classes.json"))?;
    let mut file = File::create(path)?;
    file.write_all(
        br"fn get_class_data()
    -> (std::collections::HashMap<&'static str, &'static str>, std::collections::HashMap<&'static str, &'static str>) {
        let mut tr_classes = std::collections::HashMap::new();
        let mut classes = std::collections::HashMap::new();
        "
    )?;
    for class in &classes {
        let lower = class.vanilla.to_ascii_lowercase();
        if lower != "miner" {
            write_string_map_insert(
                &mut file,
                "tr_classes",
                class.data.to_ascii_lowercase(),
                &class.vanilla,
            )?;
        }
        write_string_map_insert(&mut file, "classes", lower, &class.data)?;
    }
    file.write_all(br"return (tr_classes, classes); }")?;
    Ok(())
}

#[must_use]
fn write_broken(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let broken: HashMap<String, String> = serde_json::from_str(include_str!("./data/broken.json"))?;
    let mut file = File::create(path)?;
    file.write_all(
        br"fn get_broken_data()
    -> std::collections::HashMap<&'static str, &'static str> {
        let mut broken = std::collections::HashMap::new();
        ",
    )?;
    for (key, value) in &broken {
        write_string_map_insert(&mut file, "broken", key.to_ascii_lowercase(), value)?;
    }
    file.write_all(br"return broken; }")?;
    Ok(())
}

#[must_use]
fn write_mwscript(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    file.write_all(
        br#"fn get_joined_commands() -> &'static str {
        return r""#,
    )?;
    let mut first = true;
    for command in include_str!("./data/mwscript.returning.txt")
        .trim()
        .split_whitespace()
    {
        if first {
            first = false;
        } else {
            file.write_all(b"|")?;
        }
        file.write_all(command.as_bytes())?;
    }
    file.write_all(
        br##""; }
    fn get_khajiit_script() -> &'static str {
        return r#""##,
    )?;
    let khajiit_input = include_str!("./data/khajiit.mwscript")
        .replace("(", r"\(")
        .replace(")", r"\)")
        .replace("\n", r"\s*((;.*)?\n)+\s*");
    file.write_all(
        Regex::new(r"\s+")?
            .replace_all(&khajiit_input, r"\s+")
            .as_bytes(),
    )?;
    file.write_all(br##""#; }"##)?;
    Ok(())
}

#[derive(Deserialize)]
pub struct Project {
    pub name: String,
    pub prefix: String,
    pub local: Option<String>,
}

#[must_use]
fn write_option(file: &mut File, option: &Option<String>) -> Result<(), io::Error> {
    if let Some(value) = option {
        file.write_all(br#"Some(r""#)?;
        file.write_all(value.as_bytes())?;
        file.write_all(br#"")"#)?;
    } else {
        file.write_all(b"None")?;
    }
    Ok(())
}

#[must_use]
fn write_projects(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let projects: Vec<Project> = serde_json::from_str(include_str!("./data/projects.json"))?;
    let mut file = File::create(path)?;
    file.write_all(
        br"fn get_project_data() -> Vec<Project> {
        return vec![",
    )?;
    for project in projects {
        file.write_all(
            br#"
        Project { name: r""#,
        )?;
        file.write_all(project.name.as_bytes())?;
        file.write_all(br#"", prefix: r""#)?;
        file.write_all(project.prefix.as_bytes())?;
        file.write_all(br#"", local: "#)?;
        write_option(&mut file, &project.local)?;
        file.write_all(b"},")?;
    }
    file.write_all(br"]; }")?;
    Ok(())
}

#[derive(Deserialize)]
pub struct Services {
    barter: Vec<String>,
}

#[must_use]
fn write_services(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let services: Services = serde_json::from_str(include_str!("./data/services.json"))?;
    let mut file = File::create(path)?;
    file.write_all(
        br"fn get_barter_classes() -> std::collections::HashSet<std::string::String> {
        let mut barter_classes = std::collections::HashSet::new();
        ",
    )?;
    for class in services.barter {
        file.write_all(br#"barter_classes.insert(String::from(r""#)?;
        file.write_all(class.to_ascii_lowercase().as_bytes())?;
        file.write_all(br#""));"#)?;
    }
    file.write_all(br"return barter_classes; }")?;
    Ok(())
}

#[derive(Deserialize)]
struct SpellRule {
    prefix: Option<String>,
    race: Option<String>,
}

#[derive(Deserialize)]
struct SpellData {
    alternatives: Vec<HashMap<String, String>>,
    races: HashMap<String, SpellRule>,
    blacklist: Vec<String>,
}

#[must_use]
fn write_rule(file: &mut File, rule: &SpellRule) -> Result<(), io::Error> {
    file.write_all(b"Rc::new(Rule { prefix: ")?;
    write_option(file, &rule.prefix)?;
    file.write_all(b", race: ")?;
    write_option(file, &rule.race)?;
    file.write_all(b"})")?;
    Ok(())
}

#[must_use]
fn write_spells(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let data: SpellData = serde_json::from_str(include_str!("./data/spells.json"))?;
    let mut file = File::create(path)?;
    file.write_all(
        br"fn get_spell_data()
    -> std::collections::HashMap<&'static str, (Rc<Rule>, Rc<Vec<&'static str>>)> {
        let mut spells = std::collections::HashMap::new();
        ",
    )?;
    for (index, (_, rule)) in data.races.iter().enumerate() {
        file.write_all(b"let rule_")?;
        file.write_all(index.to_string().as_bytes())?;
        file.write_all(b" = ")?;
        write_rule(&mut file, rule)?;
        file.write_all(b";\n")?;
    }
    for (index, alternatives) in data.alternatives.iter().enumerate() {
        let ids: Vec<String> = alternatives
            .values()
            .map(&String::as_str)
            .map(&str::to_ascii_lowercase)
            .collect();
        let str_index = index.to_string();
        let alternatives_index = str_index.as_bytes();
        file.write_all(br"let alternatives_")?;
        file.write_all(alternatives_index)?;
        file.write_all(br" = Rc::new(vec![")?;
        for alternative in &ids {
            file.write_all(br##"r#""##)?;
            file.write_all(alternative.as_bytes())?;
            file.write_all(
                br##""#,
            "##,
            )?;
        }
        file.write_all(b"]);\n")?;
        for (rule, spell) in alternatives {
            if let Some((rule_index, _)) = data
                .races
                .iter()
                .enumerate()
                .find(|(_, (id, _))| (**id) == *rule)
            {
                file.write_all(br##"spells.insert(r#""##)?;
                file.write_all(spell.to_ascii_lowercase().as_bytes())?;
                file.write_all(br##""#, (rule_"##)?;
                file.write_all(rule_index.to_string().as_bytes())?;
                file.write_all(b".clone(), alternatives_")?;
                file.write_all(alternatives_index)?;
                file.write_all(b".clone()));\n")?;
            }
        }
    }
    file.write_all(b"let never = ")?;
    write_rule(
        &mut file,
        &SpellRule {
            prefix: None,
            race: None,
        },
    )?;
    file.write_all(b";\nlet none = Rc::new(Vec::new());")?;
    for id in &data.blacklist {
        file.write_all(br##"spells.insert(r#""##)?;
        file.write_all(id.to_ascii_lowercase().as_bytes())?;
        file.write_all(
            br##""#, (never.clone(), none.clone()));
        "##,
        )?;
    }
    file.write_all(br"return spells; }")?;
    Ok(())
}

#[must_use]
fn write_supplies(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let supplies: HashMap<String, String> =
        serde_json::from_str(include_str!("./data/supplies.json"))?;
    let mut file = File::create(path)?;
    file.write_all(
        br"fn get_supplies_data()
    -> std::collections::HashMap<&'static str, &'static str> {
        let mut supplies = std::collections::HashMap::new();
        ",
    )?;
    for (key, value) in &supplies {
        write_string_map_insert(&mut file, "supplies", key.to_ascii_lowercase(), value)?;
    }
    file.write_all(br"return supplies; }")?;
    Ok(())
}

#[must_use]
fn write_travel(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let classes: Vec<String> = serde_json::from_str(include_str!("./data/travel.json"))?;
    let mut file = File::create(path)?;
    file.write_all(
        br"fn get_travel_classes() -> std::collections::HashSet<&'static str> {
        let mut travel_classes = std::collections::HashSet::new();
        ",
    )?;
    for class in classes {
        file.write_all(br#"travel_classes.insert(r""#)?;
        file.write_all(class.to_ascii_lowercase().as_bytes())?;
        file.write_all(br#"");"#)?;
    }
    file.write_all(br"return travel_classes; }")?;
    Ok(())
}

#[must_use]
fn write_uniques(path: PathBuf) -> Result<(), io::Error> {
    let mut file = File::create(path)?;
    file.write_all(
        br"fn get_uniques() -> std::collections::HashSet<&'static str> {
        let mut uniques = std::collections::HashSet::new();
        ",
    )?;
    for line in include_str!("./data/uniques.txt").split('\n') {
        let id = line.trim();
        if !id.is_empty() {
            file.write_all(br#"uniques.insert(r""#)?;
            file.write_all(id.to_ascii_lowercase().as_bytes())?;
            file.write_all(br#"");"#)?;
        }
    }
    file.write_all(br"return uniques; }")?;
    Ok(())
}
