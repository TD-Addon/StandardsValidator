use super::Context;
use crate::handlers::Handler;
use regex::Regex;
use tes3::esp::{Dialogue, Info, TES3Object};

pub struct UnicodeValidator {
    invalid: Regex
}

impl Handler<'_> for UnicodeValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object, typename: &str, id: &String) {
        match record {
            TES3Object::Activator(r) => {
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::Alchemy(r) => {
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::Apparatus(r) => {
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::Armor(r) => {
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::Birthsign(r) => {
                self.test_o(typename, id, "description", &r.description, None);
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::Book(r) => {
                self.test_o(typename, id, "name", &r.name, None);
                self.test_o(typename, id, "text", &r.text, None);
            }
            TES3Object::Class(r) => {
                self.test_o(typename, id, "description", &r.description, None);
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::Clothing(r) => {
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::Container(r) => {
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::Creature(r) => {
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::Door(r) => {
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::Faction(r) => {
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::GameSetting(_) => {
                return;
            }
            TES3Object::Info(_) => {
                return;
            }
            TES3Object::Ingredient(r) => {
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::Light(r) => {
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::Lockpick(r) => {
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::MagicEffect(r) => {
                self.test_o(typename, id, "description", &r.description, None);
            }
            TES3Object::MiscItem(r) => {
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::Npc(r) => {
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::Probe(r) => {
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::Race(r) => {
                self.test_o(typename, id, "description", &r.description, None);
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::Region(r) => {
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::RepairItem(r) => {
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::Script(r) => {
                self.test_o(typename, id, "script_text", &r.script_text, None);
            }
            TES3Object::Spell(r) => {
                self.test_o(typename, id, "name", &r.name, None);
            }
            TES3Object::Weapon(r) => {
                self.test_o(typename, id, "name", &r.name, None);
            }
            _ => {},
        }
        if !id.is_empty() {
            self.test(typename, id, "id", id, None);
        }
    }

    fn on_info(&mut self, _: &Context, record: &Info, topic: &Dialogue) {
        self.test_o(record.type_name(), &record.id, "text", &record.text, Some(topic));
        self.test_o(record.type_name(), &record.id, "script_text", &record.script_text, Some(topic));
    }
}

impl UnicodeValidator {
    pub fn new() -> Result<Self, regex::Error> {
        let invalid = Regex::new(r"[\u0000-\u0008\u000b\u000c\u000e-\u001f\u007f-\uffff]")?;
        return Ok(Self{ invalid });
    }

    fn test_o(&self, typename: &str, id: &String, field: &str, value: &Option<String>, topic: Option<&Dialogue>) {
        if let Some(s) = value {
            self.test(typename, id, field, s, topic);
        }
    }

    fn test(&self, typename: &str, id: &String, field: &str, value: &String, topic: Option<&Dialogue>) {
        if let Some(m) = self.invalid.find(value) {
            if let Some(dial) = topic {
                println!("{} {} in topic {} contains odd character {} in field {}", typename, id, dial.id, m.as_str(), field);
            } else {
                println!("{} {} contains odd character {} in field {}", typename, id, m.as_str(), field);
            }
        }
    }
}
