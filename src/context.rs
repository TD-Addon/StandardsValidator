use std::collections::HashSet;

use crate::util::ci_starts_with;
use codegen::get_project_data;

#[derive(Clone, PartialEq)]
pub enum Mode {
    None,
    PT,
    TD,
    TR,
    Vanilla,
}

impl<T> From<T> for Mode
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        match value.as_ref().to_ascii_uppercase().as_str() {
            "PT" => Mode::PT,
            "TD" => Mode::TD,
            "TR" => Mode::TR,
            "VANILLA" => Mode::Vanilla,
            _ => Mode::None,
        }
    }
}

impl Mode {
    pub fn uses_td(&self) -> bool {
        *self == Mode::PT || *self == Mode::TR
    }
}

pub struct Project {
    pub name: &'static str,
    pub prefix: &'static str,
    pub local: Option<&'static str>,
}

impl Project {
    pub fn matches(&self, id: &str) -> bool {
        ci_starts_with(id, self.prefix)
    }

    pub fn has_local(&self, id: &str) -> bool {
        return self.local.iter().any(|l| id.eq_ignore_ascii_case(l));
    }
}

pub struct Context {
    pub mode: Mode,
    pub projects: Vec<Project>,
    pub deprecated: HashSet<String>,
}

impl Context {
    pub fn new(mode: Mode) -> Self {
        Context {
            mode,
            projects: get_project_data!(),
            deprecated: HashSet::new(),
        }
    }
}
