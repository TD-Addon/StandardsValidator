use std::collections::{HashMap, HashSet};

use super::Context;
use crate::{context::Mode, handlers::Handler, util::update_or_insert};
use regex::{Error, Regex};
use tes3::esp::{
    Cell, Dialogue, DialogueInfo, DialogueType2, EditorId, FixedString, QuestState, Reference,
    TES3Object, TypeInfo,
};

pub struct OrphanValidator {
    script_ids: HashSet<String>,
    start_scripts: Vec<String>,
    objects: HashMap<String, &'static str>,
    used_objects: HashSet<String>,
    enchantments: HashSet<String>,
    used_enchantments: HashSet<String>,
    journals: HashMap<String, HashSet<i32>>,
    used_journals: HashMap<String, Vec<i32>>,
    startscript: Regex,
    firstarg: Regex,
    journal: Regex,
    secondarg: Regex,
}

fn is_journal(dialogue: &Dialogue) -> bool {
    dialogue.dialogue_type == DialogueType2::Journal
}

impl Handler<'_> for OrphanValidator {
    fn on_record(&mut self, context: &Context, record: &TES3Object) {
        if context.mode == Mode::TD {
            return;
        }
        match record {
            TES3Object::Dialogue(dialogue) => {
                if is_journal(dialogue) {
                    self.journals.insert(
                        record.editor_id_ascii_lowercase().into_owned(),
                        HashSet::new(),
                    );
                }
            }
            TES3Object::Enchanting(_) => {
                self.enchantments
                    .insert(record.editor_id_ascii_lowercase().into_owned());
            }
            TES3Object::Script(_) => {
                self.script_ids
                    .insert(record.editor_id_ascii_lowercase().into_owned());
            }
            TES3Object::Activator(r) => {
                self.remove_script(&r.script);
                self.insert_object(record);
            }
            TES3Object::Alchemy(r) => {
                self.remove_script(&r.script);
                self.insert_object(record);
            }
            TES3Object::Apparatus(r) => {
                self.remove_script(&r.script);
                self.insert_object(record);
            }
            TES3Object::Armor(r) => {
                self.remove_script(&r.script);
                self.insert_object(record);
                self.add_enchantment(&r.enchanting);
            }
            TES3Object::Book(r) => {
                self.remove_script(&r.script);
                self.insert_object(record);
                self.add_enchantment(&r.enchanting);
            }
            TES3Object::Clothing(r) => {
                self.remove_script(&r.script);
                self.insert_object(record);
                self.add_enchantment(&r.enchanting);
            }
            TES3Object::Container(r) => {
                self.remove_script(&r.script);
                self.insert_object(record);
            }
            TES3Object::Creature(r) => {
                self.remove_script(&r.script);
                self.insert_object(record);
            }
            TES3Object::Door(r) => {
                self.remove_script(&r.script);
                self.insert_object(record);
            }
            TES3Object::Ingredient(r) => {
                self.remove_script(&r.script);
                self.insert_object(record);
            }
            TES3Object::LeveledCreature(_) => {
                self.insert_object(record);
            }
            TES3Object::LeveledItem(_) => {
                self.insert_object(record);
            }
            TES3Object::Light(r) => {
                self.remove_script(&r.script);
                self.insert_object(record);
            }
            TES3Object::Lockpick(r) => {
                self.remove_script(&r.script);
                self.insert_object(record);
            }
            TES3Object::MiscItem(r) => {
                self.remove_script(&r.script);
                self.insert_object(record);
            }
            TES3Object::Npc(r) => {
                self.remove_script(&r.script);
                self.insert_object(record);
            }
            TES3Object::Probe(r) => {
                self.remove_script(&r.script);
                self.insert_object(record);
            }
            TES3Object::RepairItem(r) => {
                self.remove_script(&r.script);
                self.insert_object(record);
            }
            TES3Object::StartScript(r) => {
                self.remove_script(&r.script);
            }
            TES3Object::Static(_) => {
                self.insert_object(record);
            }
            TES3Object::Weapon(r) => {
                self.remove_script(&r.script);
                self.insert_object(record);
                self.add_enchantment(&r.enchanting);
            }
            _ => {}
        }
    }

    fn on_info(&mut self, _: &Context, record: &DialogueInfo, topic: &Dialogue) {
        if is_journal(topic) && record.quest_state != Some(QuestState::Name) {
            if let Some(indices) = self.journals.get_mut(&topic.id.to_ascii_lowercase()) {
                indices.insert(record.data.disposition);
            }
        }
    }

    fn on_cellref(
        &mut self,
        _: &Context,
        _: &Cell,
        _: &Reference,
        id: &str,
        _: &[&Reference],
        _: usize,
    ) {
        self.objects.remove(id);
    }

    fn on_leveled(&mut self, _: &Context, _: &TES3Object, entry: &(String, u16)) {
        self.used_objects.insert(entry.0.to_ascii_lowercase());
    }

    fn on_inventory(&mut self, _: &Context, _: &TES3Object, entry: &(i32, FixedString<32>)) {
        self.used_objects.insert(entry.1.to_ascii_lowercase());
    }

    fn on_scriptline(
        &mut self,
        _: &Context,
        _: &TES3Object,
        code: &str,
        _: &str,
        _: &Dialogue,
        _: &str,
    ) {
        if code.is_empty() {
            return;
        }
        if let Some(captures) = self.startscript.captures(code) {
            let id = captures.get(2).unwrap().as_str();
            self.start_scripts.push(id.replace('"', ""));
        } else if let Some(captures) = self.firstarg.captures(code) {
            if let Some(quoted) = captures.get(3) {
                self.used_objects.insert(quoted.as_str().replace('"', ""));
            } else if let Some(unquoted) = captures.get(4) {
                self.used_objects.insert(unquoted.as_str().to_string());
            }
        } else if let Some(captures) = self.journal.captures(code) {
            let id;
            if let Some(quoted) = captures.get(3) {
                id = quoted.as_str().replace('"', "");
            } else if let Some(unquoted) = captures.get(4) {
                id = unquoted.as_str().to_string();
            } else {
                return;
            }
            if let Ok(index) = captures.get(5).unwrap().as_str().parse::<i32>() {
                update_or_insert(&mut self.used_journals, id, |e| e.push(index));
            }
        } else if let Some(captures) = self.secondarg.captures(code) {
            self.used_objects
                .insert(captures.get(4).unwrap().as_str().replace('"', ""));
        }
    }

    fn on_end(&mut self, _: &Context) {
        for id in &self.start_scripts {
            self.script_ids.remove(id);
        }
        for id in &self.script_ids {
            println!("Script {} is never started", id);
        }
        for id in &self.enchantments {
            if !self.used_enchantments.contains(id) {
                println!("Enchantment {} is not used", id);
            }
        }
        for id in &self.used_objects {
            self.objects.remove(id);
        }
        for (id, typename) in &self.objects {
            println!("{} {} is not used", typename, id);
        }
        for (id, indices) in &self.journals {
            if let Some(used) = self.used_journals.get(id) {
                for index in indices {
                    if !used.contains(index) {
                        println!("Journal index {} in {} is unused", index, id);
                    }
                }
            } else {
                println!("Journal {} is not used", id);
            }
        }
    }
}

impl OrphanValidator {
    pub fn new() -> Result<Self, Error> {
        let startscript =
            Regex::new(r#"^([,\s]*|.*?->[,\s]*)startscript[,\s]+("[^"]+"|[^,\s]+)[,\s]*$"#)?;
        let firstarg = Regex::new(
            r#"^([,\s]*|.*?->[,\s]*)(placeatme|addsoulgem|additem|equip|drop|placeatpc|placeitemcell|placeitem)[,\s]+(?:("[^"]+"?)(?:.*)|([^,\s"]+)(?:[,\s]+|$))"#,
        )?;
        let journal = Regex::new(
            r#"^([,\s]*|.*?->[,\s]*)(journal|setjournalindex)[,\s]+(?:("[^"]+"?)|([^,\s"]+))[,\s]+([\d]+)"#,
        )?;
        let secondarg = Regex::new(
            r#"^([,\s]*|.*?->[,\s]*)(addtolevcreature|addtolevitem)[,\s]+("[^"]+"|[^,\s]+)[,\s]+("[^"]+"|[^,\s]+)([,\s]+|$)"#,
        )?;
        Ok(Self {
            script_ids: HashSet::new(),
            start_scripts: Vec::new(),
            objects: HashMap::new(),
            used_objects: HashSet::new(),
            enchantments: HashSet::new(),
            used_enchantments: HashSet::new(),
            journals: HashMap::new(),
            used_journals: HashMap::new(),
            startscript,
            firstarg,
            journal,
            secondarg,
        })
    }

    fn insert_object(&mut self, record: &TES3Object) {
        self.objects.insert(
            record.editor_id_ascii_lowercase().to_string(),
            record.type_name(),
        );
    }

    fn remove_script(&mut self, script: &str) {
        if !script.is_empty() {
            self.script_ids.remove(&script.to_ascii_lowercase());
        }
    }

    fn add_enchantment(&mut self, enchantment: &str) {
        if !enchantment.is_empty() {
            self.used_enchantments
                .insert(enchantment.to_ascii_lowercase());
        }
    }
}
