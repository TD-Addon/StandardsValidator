pub mod autocalc;
pub mod books;
pub mod cells;
pub mod classes;
pub mod corpse;
pub mod dialogue;
pub mod doors;
pub mod duplicates;
pub mod ids;
pub mod keys;
pub mod leveled;
pub mod magic;
pub mod missing;
pub mod npc;
pub mod orphans;
pub mod persistent;
pub mod scripts;
pub mod services;
pub mod soundgens;
pub mod supplies;
pub mod todo;
pub mod travel;
pub mod unicode;
pub mod uniques;

use crate::{
    context::Context,
    handlers::{Handler, Handlers},
    util::iter_script,
};
use clap::ArgMatches;
use std::error::Error;
use tes3::esp::{Dialogue, FixedString, TES3Object};

pub struct Validator<'a> {
    handlers: Handlers<'a>,
    context: Context,
}

impl<'a> Validator<'a> {
    pub fn new<'b>(context: Context, args: &ArgMatches) -> Result<Validator<'b>, Box<dyn Error>> {
        Ok(Validator {
            handlers: Handlers::new(&context, args)?,
            context,
        })
    }

    pub fn validate(&mut self, records: &'a Vec<TES3Object>) {
        let dummy = Dialogue::default();
        let mut current_topic = &dummy;
        for record in records {
            match record {
                TES3Object::Activator(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Alchemy(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Apparatus(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Armor(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Birthsign(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Bodypart(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Book(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Cell(r) => {
                    self.handlers.on_record(&self.context, record);
                    let refs: Vec<_> = r.references.values().collect();
                    for (i, reference) in refs.iter().enumerate() {
                        self.handlers.on_cellref(
                            &self.context,
                            r,
                            reference,
                            &reference.id.to_ascii_lowercase(),
                            refs.as_slice(),
                            i,
                        );
                    }
                }
                TES3Object::Class(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Clothing(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Container(r) => {
                    self.handlers.on_record(&self.context, record);
                    self.on_inventory(record, &r.inventory);
                }
                TES3Object::Creature(r) => {
                    self.handlers.on_record(&self.context, record);
                    self.on_inventory(record, &r.inventory);
                }
                TES3Object::Dialogue(r) => {
                    self.handlers.on_record(&self.context, record);
                    current_topic = r;
                }
                TES3Object::Door(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Enchanting(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Faction(_) => self.handlers.on_record(&self.context, record),
                TES3Object::GameSetting(_) => self.handlers.on_record(&self.context, record),
                TES3Object::GlobalVariable(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Header(_) => {}
                TES3Object::DialogueInfo(r) => {
                    self.handlers.on_record(&self.context, record);
                    self.handlers.on_info(&self.context, r, current_topic);
                    self.on_script(record, &r.script_text, current_topic);
                }
                TES3Object::Ingredient(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Landscape(_) => {}
                TES3Object::LandscapeTexture(_) => {}
                TES3Object::LeveledCreature(r) => {
                    self.handlers.on_record(&self.context, record);
                    self.on_leveled(record, &r.creatures);
                }
                TES3Object::LeveledItem(r) => {
                    self.handlers.on_record(&self.context, record);
                    self.on_leveled(record, &r.items);
                }
                TES3Object::Light(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Lockpick(_) => self.handlers.on_record(&self.context, record),
                TES3Object::MagicEffect(_) => self.handlers.on_record(&self.context, record),
                TES3Object::MiscItem(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Npc(r) => {
                    self.handlers.on_record(&self.context, record);
                    self.on_inventory(record, &r.inventory);
                }
                TES3Object::PathGrid(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Probe(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Race(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Region(_) => self.handlers.on_record(&self.context, record),
                TES3Object::RepairItem(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Script(r) => {
                    self.handlers.on_record(&self.context, record);
                    self.on_script(record, &r.text, &dummy);
                }
                TES3Object::Skill(_) => {}
                TES3Object::Sound(_) => self.handlers.on_record(&self.context, record),
                TES3Object::SoundGen(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Spell(_) => self.handlers.on_record(&self.context, record),
                TES3Object::StartScript(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Static(_) => self.handlers.on_record(&self.context, record),
                TES3Object::Weapon(_) => self.handlers.on_record(&self.context, record),
            }
        }
        self.handlers.on_end(&self.context);
    }

    fn on_leveled(&mut self, record: &TES3Object, list: &[(String, u16)]) {
        for entry in list {
            self.handlers.on_leveled(&self.context, record, entry);
        }
    }

    fn on_inventory(&mut self, record: &TES3Object, inventory: &[(i32, FixedString<32>)]) {
        for entry in inventory {
            self.handlers.on_inventory(&self.context, record, entry);
        }
    }

    fn on_script(&mut self, record: &TES3Object, script_text: &str, topic: &Dialogue) {
        for (code, comment) in iter_script(script_text) {
            self.handlers.on_scriptline(
                &self.context,
                record,
                &code.to_ascii_lowercase(),
                comment,
                topic,
            );
        }
    }
}
