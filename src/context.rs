use crate::util::ci_starts_with;

#[derive(Clone, PartialEq)]
pub enum Mode {
    None,
    PT,
    TD,
    TR,
    Vanilla,
}

impl From<&String> for Mode {
    fn from(value: &String) -> Self {
        if value == "PT" {
            return Mode::PT;
        } else if value == "TD" {
            return Mode::TD;
        } else if value == "TR" {
            return Mode::TR;
        } else if value == "Vanilla" {
            return Mode::Vanilla;
        }
        return Mode::None;
    }
}

pub struct Project {
    pub name: &'static str,
    pub prefix: &'static str,
    pub local: Option<&'static str>,
}

impl Project {
    pub fn matches(&self, id: &str) -> bool {
        return ci_starts_with(id, &self.prefix);
    }

    pub fn has_local(&self, id: &str) -> bool {
        return self.local.iter().any(|l| id.eq_ignore_ascii_case(l));
    }
}

include!(concat!(env!("OUT_DIR"), "/gen_projects.rs"));

pub struct Context {
    pub mode: Mode,
    pub projects: Vec<Project>,
}

impl Context {
    pub fn new(mode: Mode) -> Self {
        return Context {
            mode,
            projects: get_project_data(),
        };
    }
}
