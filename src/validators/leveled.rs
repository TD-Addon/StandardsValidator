use std::collections::HashMap;

use super::Context;
use crate::handlers::Handler;
use tes3::esp::{EditorId, LeveledCreatureFlags, LeveledItemFlags, TES3Object, TypeInfo};

pub struct LeveledValidator<'a> {
    to_check: Vec<&'a TES3Object>,
    minimum_levels: HashMap<String, u16>,
}

fn check_all_levels(t: &str, id: &str, list: &[(String, u16)]) {
    let [first, rest @ ..] = list else {
        return;
    };
    for item in rest {
        if item.1 != first.1 {
            println!("{} {} is not calculated for all levels", t, id);
            break;
        }
    }
}

impl<'a> Handler<'a> for LeveledValidator<'a> {
    fn on_record(&mut self, _: &Context, record: &'a TES3Object) {
        match record {
            TES3Object::LeveledCreature(r) => {
                if !r
                    .leveled_creature_flags
                    .contains(LeveledCreatureFlags::CALCULATE_FROM_ALL_LEVELS)
                {
                    check_all_levels(record.type_name(), &r.id, &r.creatures);
                }
                if let Some(entry) = &r.creatures.first() {
                    self.minimum_levels
                        .insert(record.editor_id_ascii_lowercase().into_owned(), entry.1);
                }
                self.to_check.push(record);
            }
            TES3Object::LeveledItem(r) => {
                if !r
                    .leveled_item_flags
                    .contains(LeveledItemFlags::CALCULATE_FROM_ALL_LEVELS)
                {
                    check_all_levels(record.type_name(), &r.id, &r.items);
                }
                if let Some(entry) = r.items.first() {
                    self.minimum_levels
                        .insert(record.editor_id_ascii_lowercase().into_owned(), entry.1);
                }
                self.to_check.push(record);
            }
            _ => {}
        }
    }

    fn on_end(&mut self, _: &Context) {
        for record in &self.to_check {
            match record {
                TES3Object::LeveledCreature(r) => {
                    self.check_min(r.type_name(), &r.id, &r.creatures);
                }
                TES3Object::LeveledItem(r) => {
                    self.check_min(r.type_name(), &r.id, &r.items);
                }
                _ => {}
            }
        }
    }
}

impl LeveledValidator<'_> {
    pub fn new<'a>() -> LeveledValidator<'a> {
        LeveledValidator {
            to_check: Vec::new(),
            minimum_levels: HashMap::new(),
        }
    }

    fn check_min(&self, t: &str, id: &str, list: &[(String, u16)]) {
        for entry in list {
            if let Some(min) = self.minimum_levels.get(&entry.0.to_ascii_lowercase()) {
                if *min > entry.1 {
                    println!("{} {} contains {} at level {} which will not resolve to anything at that level", t, id, entry.0, entry.1);
                }
            }
        }
    }
}
