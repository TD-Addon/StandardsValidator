use serde::Deserialize;

#[derive(Clone, PartialEq)]
pub enum Mode {
    None,
    PT,
    TD,
    TR,
}

impl From<&String> for Mode {
    fn from(value: &String) -> Self {
        if value == "PT" {
            return Mode::PT;
        } else if value == "TD" {
            return Mode::TD;
        } else if value == "TR" {
            return Mode::TR;
        }
        return Mode::None;
    }
}

#[derive(Deserialize)]
pub struct Project {
    pub name: String,
    pub prefix: String,
    pub local: Option<String>,
}

pub struct Context {
    pub mode: Mode,
    pub projects: Vec<Project>,
}

impl Context {
    pub fn new(mode: Mode) -> serde_json::Result<Self> {
        let projects: Vec<Project> = serde_json::from_str(include_str!("../data/projects.json"))?;
        return Ok(Context { mode, projects });
    }
}
