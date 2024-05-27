use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::File,
    io::{self, ErrorKind, Write},
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed=data");
    println!("cargo::rerun-if-changed=build.rs");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    write_bodyparts(Path::new(&out_dir).join("gen_bodyparts.rs"))?;
    write_classes(Path::new(&out_dir).join("gen_classes.rs"))?;
    write_broken(Path::new(&out_dir).join("gen_broken.rs"))?;
    write_mwscript(Path::new(&out_dir).join("gen_mwscript.rs"))?;
    write_projects(Path::new(&out_dir).join("gen_projects.rs"))?;
    write_services(Path::new(&out_dir).join("gen_services.rs"))?;
    write_spells(Path::new(&out_dir).join("gen_spells.rs"))?;
    write_supplies(Path::new(&out_dir).join("gen_supplies.rs"))?;
    write_travel(Path::new(&out_dir).join("gen_travel.rs"))?;
    write_uniques(Path::new(&out_dir).join("gen_uniques.rs"))?;
    Ok(())
}

trait FileWritable {
    fn write_to_file(&self, file: &mut File) -> Result<(), io::Error>;
}

impl FileWritable for &str {
    fn write_to_file(&self, file: &mut File) -> Result<(), io::Error> {
        file.write_all(br##"r#""##)?;
        file.write_all(self.as_bytes())?;
        file.write_all(br##""#"##)
    }
}

impl FileWritable for String {
    fn write_to_file(&self, file: &mut File) -> Result<(), io::Error> {
        self.as_str().write_to_file(file)
    }
}

impl<T> FileWritable for Option<T>
where
    T: FileWritable,
{
    fn write_to_file(&self, file: &mut File) -> Result<(), io::Error> {
        if let Some(value) = self {
            file.write_all(b"Some(")?;
            value.write_to_file(file)?;
            file.write_all(b")")
        } else {
            file.write_all(b"None")
        }
    }
}

fn write_vec<'a, T, F>(
    file: &mut File,
    iter: impl Iterator<Item = T>,
    f: F,
) -> Result<(), io::Error>
where
    T: 'a,
    F: Fn(T, &mut File) -> Result<(), io::Error>,
{
    file.write_all(b"vec![\n")?;
    for element in iter {
        f(element, file)?;
        file.write_all(b",\n")?;
    }
    file.write_all(b"]")
}

impl<T> FileWritable for Vec<T>
where
    T: FileWritable,
{
    fn write_to_file(&self, file: &mut File) -> Result<(), io::Error> {
        write_vec(file, self.iter(), FileWritable::write_to_file)
    }
}

#[derive(Deserialize)]
struct BodyPartDefinition {
    model: String,
    ruleset: Option<String>,
    rules: Option<Vec<HashMap<String, Value>>>,
}

fn parse_bodypart_rule(value: &Value, file: &mut File) -> Result<(), io::Error> {
    match value {
        Value::Array(rule) => {
            file.write_all(b"Rule::Array(")?;
            write_vec(file, rule.iter(), parse_bodypart_rule)?;
            return file.write_all(b")");
        }
        Value::Object(rule) => {
            if let Some(v) = rule.get("not") {
                file.write_all(b"Rule::Negation(Box::new(")?;
                parse_bodypart_rule(v, file)?;
                return file.write_all(b"))");
            }
        }
        Value::String(rule) => {
            file.write_all(b"Rule::Equality(")?;
            rule.to_ascii_lowercase().write_to_file(file)?;
            return file.write_all(b")");
        }
        _ => {}
    }
    Err(io::Error::new(
        ErrorKind::Other,
        format!("Failed to parse {}", value),
    ))
}

fn parse_bodypart_fields(map: &HashMap<String, Value>, file: &mut File) -> Result<(), io::Error> {
    write_vec(file, map.iter(), |(key, value), file| {
        file.write_all(b"FieldRule::")?;
        if key == "class" {
            file.write_all(b"Class")?;
        } else if key == "faction" {
            file.write_all(b"Faction")?;
        } else if key == "id" {
            file.write_all(b"Id")?;
        } else {
            return Err(io::Error::new(
                ErrorKind::Other,
                format!("Invalid key {}", key),
            ));
        }
        file.write_all(b"(")?;
        parse_bodypart_rule(value, file)?;
        file.write_all(b")")
    })
}

impl FileWritable for BodyPartDefinition {
    fn write_to_file(&self, file: &mut File) -> Result<(), io::Error> {
        file.write_all(b"(")?;
        self.model.to_ascii_lowercase().write_to_file(file)?;
        file.write_all(b",")?;
        self.ruleset.write_to_file(file)?;
        file.write_all(b",")?;
        if let Some(rules) = &self.rules {
            file.write_all(b"Some(")?;
            write_vec(file, rules.iter(), parse_bodypart_fields)?;
            file.write_all(b")")?;
        } else {
            file.write_all(b"None")?;
        }
        file.write_all(b")")
    }
}

#[derive(Deserialize)]
struct BodyParts {
    rulesets: HashMap<String, Vec<HashMap<String, Value>>>,
    head: Vec<BodyPartDefinition>,
    hair: Vec<BodyPartDefinition>,
}

impl FileWritable for BodyParts {
    fn write_to_file(&self, file: &mut File) -> Result<(), io::Error> {
        file.write_all(b"(")?;
        write_vec(file, self.rulesets.iter(), |(id, ruleset), file| {
            file.write_all(b"(")?;
            id.write_to_file(file)?;
            file.write_all(b",")?;
            write_vec(file, ruleset.iter(), parse_bodypart_fields)?;
            file.write_all(b")")
        })?;
        file.write_all(b",")?;
        self.head.write_to_file(file)?;
        file.write_all(b",")?;
        self.hair.write_to_file(file)?;
        file.write_all(b")")
    }
}

fn write_bodyparts(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let data: BodyParts = serde_json::from_str(include_str!("./data/bodyparts.json"))?;
    let mut file = File::create(path)?;
    file.write_all(
        br"fn get_bodypart_data()
    -> (Vec<(&'static str, Vec<Vec<FieldRule>>)>,
    Vec<(&'static str, Option<&'static str>, Option<Vec<Vec<FieldRule>>>)>,
    Vec<(&'static str, Option<&'static str>, Option<Vec<Vec<FieldRule>>>)>) {
        ",
    )?;
    data.write_to_file(&mut file)?;
    file.write_all(b" }")?;
    Ok(())
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
    file.write_all(b".insert(")?;
    key.write_to_file(file)?;
    file.write_all(b", ")?;
    value.write_to_file(file)?;
    file.write_all(b");\n")
}

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
    file.write_all(b"(tr_classes, classes) }")?;
    Ok(())
}

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
    file.write_all(b"broken }")?;
    Ok(())
}

fn write_mwscript(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    file.write_all(
        br#"fn get_joined_commands() -> &'static str {
        r""#,
    )?;
    let mut first = true;
    for command in include_str!("./data/mwscript.returning.txt").split_whitespace() {
        if first {
            first = false;
        } else {
            file.write_all(b"|")?;
        }
        file.write_all(command.as_bytes())?;
    }
    file.write_all(
        br##"" }
    fn get_khajiit_script() -> &'static str {
        "##,
    )?;
    let khajiit_input = include_str!("./data/khajiit.mwscript")
        .replace('(', r"\(")
        .replace(')', r"\)")
        .replace('\n', r"\s*((;.*)?\n)+\s*");
    Regex::new(r"\s+")?
        .replace_all(&khajiit_input, r"\s+")
        .to_string()
        .write_to_file(&mut file)?;
    file.write_all(b" }")?;
    Ok(())
}

#[derive(Deserialize)]
pub struct Project {
    pub name: String,
    pub prefix: String,
    pub local: Option<String>,
}

impl FileWritable for Project {
    fn write_to_file(&self, file: &mut File) -> Result<(), io::Error> {
        file.write_all(b"Project { name: ")?;
        self.name.write_to_file(file)?;
        file.write_all(b", prefix: ")?;
        self.prefix.write_to_file(file)?;
        file.write_all(b", local: ")?;
        self.local.write_to_file(file)?;
        file.write_all(b"}")
    }
}

fn write_projects(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let projects: Vec<Project> = serde_json::from_str(include_str!("./data/projects.json"))?;
    let mut file = File::create(path)?;
    file.write_all(
        br"fn get_project_data() -> Vec<Project> {
        ",
    )?;
    projects.write_to_file(&mut file)?;
    file.write_all(b" }")?;
    Ok(())
}

#[derive(Deserialize)]
pub struct Services {
    barter: Vec<String>,
}

fn write_services(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let services: Services = serde_json::from_str(include_str!("./data/services.json"))?;
    let mut file = File::create(path)?;
    file.write_all(
        br"fn get_barter_classes() -> std::collections::HashSet<std::string::String> {
        let mut barter_classes = std::collections::HashSet::new();
        ",
    )?;
    for class in services.barter {
        file.write_all(b"barter_classes.insert(String::from(")?;
        class.to_ascii_lowercase().write_to_file(&mut file)?;
        file.write_all(b"));\n")?;
    }
    file.write_all(b"barter_classes }")?;
    Ok(())
}

#[derive(Deserialize)]
struct SpellRule {
    prefix: Option<String>,
    race: Option<String>,
}

impl FileWritable for SpellRule {
    fn write_to_file(&self, file: &mut File) -> Result<(), io::Error> {
        file.write_all(b"Rc::new(Rule { prefix: ")?;
        self.prefix.write_to_file(file)?;
        file.write_all(b", race: ")?;
        self.race.write_to_file(file)?;
        file.write_all(b"})")
    }
}

#[derive(Deserialize)]
struct SpellData {
    alternatives: Vec<HashMap<String, String>>,
    races: HashMap<String, SpellRule>,
    blacklist: Vec<String>,
}

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
        rule.write_to_file(&mut file)?;
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
        file.write_all(b"let alternatives_")?;
        file.write_all(alternatives_index)?;
        file.write_all(b" = Rc::new(")?;
        ids.write_to_file(&mut file)?;
        file.write_all(b");\n")?;
        for (rule, spell) in alternatives {
            if let Some((rule_index, _)) = data
                .races
                .iter()
                .enumerate()
                .find(|(_, (id, _))| (**id) == *rule)
            {
                file.write_all(b"spells.insert(")?;
                spell.to_ascii_lowercase().write_to_file(&mut file)?;
                file.write_all(b", (rule_")?;
                file.write_all(rule_index.to_string().as_bytes())?;
                file.write_all(b".clone(), alternatives_")?;
                file.write_all(alternatives_index)?;
                file.write_all(b".clone()));\n")?;
            }
        }
    }
    file.write_all(b"let never = ")?;
    SpellRule {
        prefix: None,
        race: None,
    }
    .write_to_file(&mut file)?;
    file.write_all(b";\nlet none = Rc::new(Vec::new());")?;
    for id in &data.blacklist {
        file.write_all(b"spells.insert(")?;
        id.to_ascii_lowercase().write_to_file(&mut file)?;
        file.write_all(b", (never.clone(), none.clone()));\n")?;
    }
    file.write_all(br"spells }")?;
    Ok(())
}

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
    file.write_all(b"supplies }")?;
    Ok(())
}

fn write_travel(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let classes: Vec<String> = serde_json::from_str(include_str!("./data/travel.json"))?;
    let mut file = File::create(path)?;
    file.write_all(
        br"fn get_travel_classes() -> std::collections::HashSet<&'static str> {
        let mut travel_classes = std::collections::HashSet::new();
        ",
    )?;
    for class in classes {
        file.write_all(b"travel_classes.insert(")?;
        class.to_ascii_lowercase().write_to_file(&mut file)?;
        file.write_all(b");\n")?;
    }
    file.write_all(b"travel_classes }")?;
    Ok(())
}

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
            file.write_all(b"uniques.insert(")?;
            id.to_ascii_lowercase().write_to_file(&mut file)?;
            file.write_all(b");\n")?;
        }
    }
    file.write_all(b"uniques }")
}
