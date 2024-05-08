use std::collections::HashMap;

use super::Context;
use crate::handler_traits::Handler;
use tes3::esp::TES3Object;

pub struct LeveledValidator<'a> {
    to_check: Vec<&'a TES3Object>,
    minimum_levels: HashMap<String, u16>,
}

const FLAG_ALL_LEVELS_CREATURE: u32 = 1;
const FLAG_ALL_LEVELS_ITEM: u32 = 2;

fn has_flag(flags: &Option<u32>, flag: u32) -> bool {
    if let Some(f) = flags {
        return (f & flag) != 0;
    }
    return false;
}

fn check_all_levels(t: &str, id: &String, list: &Option<Vec<(String, u16)>>) {
    if let Some(entries) = list {
        if entries.len() > 1 {
            let first;
            unsafe {
                first = entries.get_unchecked(0).1;
            }
            for entry in &entries[1..] {
                if entry.1 != first {
                    println!("{} {} is not calculated for all levels", t, id);
                    break;
                }
            }
        }
    }
}

fn get_first(list: &Option<Vec<(String, u16)>>) -> Option<&(String, u16)> {
    return list.iter().flat_map(|l| l.first()).next();
}

impl<'a> Handler<'a> for LeveledValidator<'a> {
    fn on_record(&mut self, _: &Context, record: &'a TES3Object, id: &String) {
        match record {
            TES3Object::LeveledCreature(r) => {
                if !has_flag(&r.list_flags, FLAG_ALL_LEVELS_CREATURE) {
                    check_all_levels("LeveledCreature", &r.id, &r.creatures);
                }
                if let Some(entry) = get_first(&r.creatures) {
                    self.minimum_levels.insert(id.clone(), entry.1);
                }
                self.to_check.push(record);
            }
            TES3Object::LeveledItem(r) => {
                if !has_flag(&r.list_flags, FLAG_ALL_LEVELS_ITEM) {
                    check_all_levels("LeveledItem", &r.id, &r.items);
                }
                if let Some(entry) = get_first(&r.items) {
                    self.minimum_levels.insert(id.clone(), entry.1);
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
                    self.check_min("LeveledCreature", &r.id, &r.creatures);
                }
                TES3Object::LeveledItem(r) => {
                    self.check_min("LeveledItem", &r.id, &r.items);
                }
                _ => {}
            }
        }
    }
}

impl LeveledValidator<'_> {
    pub fn new<'a>() -> LeveledValidator<'a> {
        return LeveledValidator {
            to_check: Vec::new(),
            minimum_levels: HashMap::new(),
        };
    }

    fn check_min(&self, t: &str, id: &String, option: &Option<Vec<(String, u16)>>) {
        if let Some(list) = option {
            for entry in list {
                if let Some(min) = self.minimum_levels.get(&entry.0.to_ascii_lowercase()) {
                    if min.clone() > entry.1 {
                        println!("{} {} contains {} at level {} which will not resolve to anything at that level", t, id, entry.0, entry.1);
                    }
                }
            }
        }
    }
}
