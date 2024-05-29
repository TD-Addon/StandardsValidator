use std::collections::{HashMap, HashSet};

use super::Context;
use crate::handlers::Handler;
use codegen::get_uniques;
use regex::{escape, Regex};
use tes3::esp::{Cell, Dialogue, EditorId, FixedString, Reference, TES3Object, TypeInfo};

pub struct UniquesValidator {
    uniques: HashSet<&'static str>,
    create_func: Regex,
    regex_cache: HashMap<&'static str, Regex>,
}

fn check_script_line(
    regex_cache: &mut HashMap<&'static str, Regex>,
    line: &str,
    item: &'static str,
) -> bool {
    if line.contains(item) {
        if let Some(regex) = regex_cache.get(item) {
            return regex.is_match(line);
        }
        let regex = Regex::new(&format!(r#"[ ,"]{}($|[ ,"])"#, escape(item))).unwrap();
        let matches = regex.is_match(line);
        regex_cache.insert(item, regex);
        return matches;
    }
    false
}

impl Handler<'_> for UniquesValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object) {
        match record {
            TES3Object::Armor(r) => {
                self.check(&r.enchanting, record);
            }
            TES3Object::Book(r) => {
                self.check(&r.enchanting, record);
            }
            TES3Object::Clothing(r) => {
                self.check(&r.enchanting, record);
            }
            TES3Object::Weapon(r) => {
                self.check(&r.enchanting, record);
            }
            _ => {}
        }
    }

    fn on_cellref(
        &mut self,
        _: &Context,
        record: &Cell,
        reference: &Reference,
        id: &str,
        _: &[&Reference],
        _: usize,
    ) {
        if self.uniques.contains(&id) {
            println!(
                "{} {} references {}",
                record.type_name(),
                record.editor_id(),
                reference.id
            );
        }
    }

    fn on_leveled(&mut self, _: &Context, record: &TES3Object, entry: &(String, u16)) {
        if let TES3Object::LeveledCreature(_) = record {
            self.check(&entry.0, record);
        } else if let TES3Object::LeveledItem(_) = record {
            self.check(&entry.0, record);
        }
    }

    fn on_inventory(&mut self, _: &Context, record: &TES3Object, entry: &(i32, FixedString<32>)) {
        match record {
            TES3Object::Container(_) => {
                self.check(&entry.1, record);
            }
            TES3Object::Creature(_) => {
                self.check(&entry.1, record);
            }
            TES3Object::Npc(_) => {
                self.check(&entry.1, record);
            }
            _ => {}
        }
    }

    fn on_scriptline(
        &mut self,
        _: &Context,
        record: &TES3Object,
        code: &str,
        _: &str,
        topic: &Dialogue,
    ) {
        if self.create_func.is_match(code) {
            for uni in &self.uniques {
                if check_script_line(&mut self.regex_cache, code, uni) {
                    if let TES3Object::DialogueInfo(info) = record {
                        println!(
                            "{} {} in topic {} references {}",
                            info.type_name(),
                            info.id,
                            topic.id,
                            uni
                        );
                    } else if let TES3Object::Script(script) = record {
                        println!("{} {} references {}", script.type_name(), script.id, uni);
                    }
                    break;
                }
            }
        }
    }
}

impl UniquesValidator {
    pub fn new() -> Result<Self, regex::Error> {
        let create_func = Regex::new(
            r"placeatme|addtolevcreature|addtolevitem|addsoulgem|addspell|cast|explodespell|dropsoulgem|additem|equip|drop|placeatpc|placeitem|placeitemcell",
        )?;
        Ok(Self {
            uniques: get_uniques!(),
            create_func,
            regex_cache: HashMap::new(),
        })
    }

    fn check(&self, value: &str, record: &TES3Object) {
        if self.uniques.contains(value.to_ascii_lowercase().as_str()) {
            println!(
                "{} {} references {}",
                record.type_name(),
                record.editor_id(),
                value
            );
        }
    }
}
