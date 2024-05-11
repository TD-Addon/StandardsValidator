use std::collections::HashMap;

use super::Context;
use crate::{context::Mode, handlers::Handler};
use serde::Deserialize;
use tes3::esp::{Dialogue, FilterType, Info, TES3Object};

pub struct ClassValidator {
    tr_classes: HashMap<String, String>,
    classes: HashMap<String, String>,
}

impl Handler<'_> for ClassValidator {
    fn on_record(&mut self, context: &Context, record: &TES3Object, _: &str, _: &String) {
        if let TES3Object::Npc(npc) = record {
            if let Some(class) = &npc.class {
                if let Some(replacement) = self.get_replacement(class, context) {
                    println!(
                        "Npc {} has class {} which should be {}",
                        npc.id, class, replacement
                    );
                }
            }
        }
    }

    fn on_info(&mut self, context: &Context, record: &Info, topic: &Dialogue) {
        if let Some(class) = &record.speaker_class {
            if let Some(_) = self.get_replacement(class, context) {
                println!(
                    "Info {} in topic {} has a {} filter",
                    record.id, topic.id, class
                )
            }
        }
        if let Some(filters) = &record.filters {
            for filter in filters {
                if filter.kind == FilterType::NotClass {
                    if let Some(_) = self.get_replacement(&filter.id, context) {
                        println!(
                            "Info {} in topic {} has a Not Class {} filter",
                            record.id, topic.id, filter.id
                        );
                    }
                }
            }
        }
    }
}

#[derive(Deserialize)]
struct ClassData {
    vanilla: String,
    data: String,
}

impl ClassValidator {
    pub fn new() -> serde_json::Result<Self> {
        let classes: Vec<ClassData> =
            serde_json::from_str(include_str!("../../data/classes.json"))?;
        let mut validator = Self {
            tr_classes: HashMap::new(),
            classes: HashMap::new(),
        };
        for class in &classes {
            let lower = class.vanilla.to_ascii_lowercase();
            if lower != "miner" {
                validator
                    .tr_classes
                    .insert(class.data.to_ascii_lowercase(), class.vanilla.clone());
            }
            validator.classes.insert(lower, class.data.clone());
        }
        return Ok(validator);
    }

    fn get_replacement(&self, id: &String, context: &Context) -> Option<&String> {
        if context.mode == Mode::PT {
            return self.classes.get(&id.to_ascii_lowercase());
        } else if context.mode == Mode::TR {
            return self.tr_classes.get(&id.to_ascii_lowercase());
        }
        return None;
    }
}
