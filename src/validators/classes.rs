use std::collections::HashMap;

use super::Context;
use crate::{context::Mode, handlers::Handler};
use tes3::esp::{Dialogue, FilterType, Info, TES3Object};

include!(concat!(env!("OUT_DIR"), "/gen_classes.rs"));

pub struct ClassValidator {
    tr_classes: HashMap<&'static str, &'static str>,
    classes: HashMap<&'static str, &'static str>,
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

impl ClassValidator {
    pub fn new() -> Self {
        let (tr_classes, classes) = get_class_data();
        return Self {
            tr_classes,
            classes,
        };
    }

    fn get_replacement(&self, id: &String, context: &Context) -> Option<&&'static str> {
        if context.mode == Mode::PT {
            return self.classes.get(id.to_ascii_lowercase().as_str());
        } else if context.mode == Mode::TR {
            return self.tr_classes.get(id.to_ascii_lowercase().as_str());
        }
        return None;
    }
}
