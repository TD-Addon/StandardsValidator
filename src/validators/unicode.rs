use super::Context;
use crate::{handlers::Handler, util::is_marker};
use regex::Regex;
use tes3::esp::{Dialogue, DialogueInfo, EditorId, TES3Object, TypeInfo};

pub struct UnicodeValidator {
    invalid: Regex,
    orc_name: Regex,
    script_text: Regex,
}

impl Handler<'_> for UnicodeValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object) {
        match record {
            TES3Object::Activator(r) => {
                self.test(record, "name", &r.name, None);
            }
            TES3Object::Alchemy(r) => {
                self.test(record, "name", &r.name, None);
            }
            TES3Object::Apparatus(r) => {
                self.test(record, "name", &r.name, None);
            }
            TES3Object::Armor(r) => {
                self.test(record, "name", &r.name, None);
            }
            TES3Object::Birthsign(r) => {
                self.test(record, "description", &r.description, None);
                self.test(record, "name", &r.name, None);
            }
            TES3Object::Book(r) => {
                if is_marker(r) {
                    return;
                }
                self.test(record, "name", &r.name, None);
                self.test(record, "text", &r.text, None);
            }
            TES3Object::Class(r) => {
                self.test(record, "description", &r.description, None);
                self.test(record, "name", &r.name, None);
            }
            TES3Object::Clothing(r) => {
                self.test(record, "name", &r.name, None);
            }
            TES3Object::Container(r) => {
                self.test(record, "name", &r.name, None);
            }
            TES3Object::Creature(r) => {
                self.test(record, "name", &r.name, None);
            }
            TES3Object::Door(r) => {
                self.test(record, "name", &r.name, None);
            }
            TES3Object::Faction(r) => {
                self.test(record, "name", &r.name, None);
            }
            TES3Object::GameSetting(_) => {
                return;
            }
            TES3Object::DialogueInfo(_) => {
                return;
            }
            TES3Object::Ingredient(r) => {
                self.test(record, "name", &r.name, None);
            }
            TES3Object::Light(r) => {
                self.test(record, "name", &r.name, None);
            }
            TES3Object::Lockpick(r) => {
                self.test(record, "name", &r.name, None);
            }
            TES3Object::MagicEffect(r) => {
                self.test(record, "description", &r.description, None);
                return;
            }
            TES3Object::MiscItem(r) => {
                self.test(record, "name", &r.name, None);
            }
            TES3Object::Npc(r) => {
                self.test(record, "name", &r.name, None);
            }
            TES3Object::PathGrid(_) => {
                return;
            }
            TES3Object::Probe(r) => {
                self.test(record, "name", &r.name, None);
            }
            TES3Object::Race(r) => {
                self.test(record, "description", &r.description, None);
                self.test(record, "name", &r.name, None);
            }
            TES3Object::Region(r) => {
                self.test(record, "name", &r.name, None);
            }
            TES3Object::RepairItem(r) => {
                self.test(record, "name", &r.name, None);
            }
            TES3Object::Script(r) => {
                self.test(record, "script_text", &r.text, None);
            }
            TES3Object::Spell(r) => {
                self.test(record, "name", &r.name, None);
            }
            TES3Object::Weapon(r) => {
                self.test(record, "name", &r.name, None);
            }
            _ => {}
        }
        let id = record.editor_id();
        if !id.is_empty() {
            self.test(record, "id", &id, None);
        }
    }

    fn on_info(&mut self, _: &Context, record: &DialogueInfo, topic: &Dialogue) {
        self.test(record, "text", &record.text, Some(topic));
        self.test(record, "script_text", &record.script_text, Some(topic));
    }

    fn on_scriptline(
        &mut self,
        _: &Context,
        record: &TES3Object,
        code: &str,
        _: &str,
        topic: &Dialogue,
        code_original: &str,
    ) {
        if let Some(m) = self.script_text.find(code) {
            let mut t = Some(topic);
            if topic.id.is_empty() {
                t = None;
            }
            self.check_orc_name(record, "script_text", &code_original[m.end()..], t);
        }
    }
}

impl UnicodeValidator {
    pub fn new() -> Result<Self, regex::Error> {
        let invalid = Regex::new(r"[\u0000-\u0008\u000b\u000c\u000e-\u001f\u007f-\uffff]")?;
        let orc_name = Regex::new(" (Gr[oa]-[A-Za-z]|gr[oa]-[a-z])[a-z'-]+")?;
        let script_text = Regex::new("(say|messagebox)[, ]")?;
        Ok(Self {
            invalid,
            orc_name,
            script_text,
        })
    }

    fn test<T>(&self, record: &T, field: &str, value: &str, topic: Option<&Dialogue>)
    where
        T: EditorId + TypeInfo,
    {
        if let Some(m) = self.invalid.find(value) {
            if let Some(dial) = topic {
                println!(
                    "{} {} in topic {} contains odd character {} in field {}",
                    record.type_name(),
                    record.editor_id(),
                    dial.id,
                    m.as_str(),
                    field
                );
            } else {
                println!(
                    "{} {} contains odd character {} in field {}",
                    record.type_name(),
                    record.editor_id(),
                    m.as_str(),
                    field
                );
            }
        }
        if field != "id" && field != "script_text" {
            self.check_orc_name(record, field, value, topic);
        }
    }

    fn check_orc_name<T>(&self, record: &T, field: &str, value: &str, topic: Option<&Dialogue>)
    where
        T: EditorId + TypeInfo,
    {
        if let Some(m) = self.orc_name.find(value) {
            if let Some(dial) = topic {
                println!(
                    "{} {} in topic {} contains odd orc name{} in field {}",
                    record.type_name(),
                    record.editor_id(),
                    dial.id,
                    m.as_str(),
                    field
                );
            } else {
                println!(
                    "{} {} contains contains odd orc name{} in field {}",
                    record.type_name(),
                    record.editor_id(),
                    m.as_str(),
                    field
                );
            }
        }
    }
}
