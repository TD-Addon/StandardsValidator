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

fn ci_starts_with(s: &str, prefix: &str) -> bool {
    if s.len() >= prefix.len() {
        return s.as_bytes()[..prefix.len()].eq_ignore_ascii_case(prefix.as_bytes());
    }
    return false;
}

impl Project {
    pub fn matches(&self, id: &str) -> bool {
        return ci_starts_with(id, &self.prefix);
    }

    pub fn has_local(&self, id: &str) -> bool {
        return self.local.iter().any(|l| id.eq_ignore_ascii_case(l));
    }
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
