use std::collections::HashMap;

use super::Context;
use crate::{context::Mode, handlers::Handler};
use tes3::esp::{Dialogue, DialogueInfo, FilterType, TES3Object};

include!(concat!(env!("OUT_DIR"), "/gen_classes.rs"));

pub struct ClassValidator {
    tr_classes: HashMap<&'static str, &'static str>,
    classes: HashMap<&'static str, &'static str>,
}

impl Handler<'_> for ClassValidator {
    fn on_record(&mut self, context: &Context, record: &TES3Object) {
        if let TES3Object::Npc(npc) = record {
            if !npc.class.is_empty() {
                if let Some(replacement) = self.get_replacement(&npc.class, context) {
                    println!(
                        "Npc {} has class {} which should be {}",
                        &npc.id, &npc.class, replacement
                    );
                }
            }
        }
    }

    fn on_info(&mut self, context: &Context, record: &DialogueInfo, topic: &Dialogue) {
        if !record.speaker_class.is_empty()
            && self
                .get_replacement(&record.speaker_class, context)
                .is_some()
        {
            println!(
                "Info {} in topic {} has a {} filter",
                record.id, topic.id, record.speaker_class
            )
        }
        for filter in &record.filters {
            if filter.filter_type == FilterType::NotClass
                && self.get_replacement(&filter.id, context).is_some()
            {
                println!(
                    "Info {} in topic {} has a Not Class {} filter",
                    record.id, topic.id, filter.id
                );
            }
        }
    }
}

impl ClassValidator {
    pub fn new() -> Self {
        let (tr_classes, classes) = get_class_data();
        Self {
            tr_classes,
            classes,
        }
    }

    fn get_replacement(&self, id: &str, context: &Context) -> Option<&&'static str> {
        if context.mode == Mode::PT {
            return self.classes.get(id.to_ascii_lowercase().as_str());
        } else if context.mode == Mode::TR {
            return self.tr_classes.get(id.to_ascii_lowercase().as_str());
        }
        None
    }
}
