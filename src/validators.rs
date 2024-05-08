pub mod autocalc;
pub mod books;
pub mod corpse;
pub mod doors;
pub mod test;

use crate::handler_traits::Handler;
use tes3::esp::{Cell, Dialogue, FixedString, Info, ObjectFlags, Reference, TES3Object};

pub enum Mode {
    TR,
    PT,
    TD,
}

pub struct Context {
    pub mode: Mode,
}

pub struct Handlers {
    handlers: Vec<Box<dyn Handler>>,
}

impl Handlers {
    fn new() -> Handlers {
        return Handlers {
            handlers: vec![
                Box::new(test::TestValidator {}),
                Box::new(autocalc::AutoCalcValidator {}),
                Box::new(books::BookValidator {}),
                Box::new(corpse::CorpseValidator {}),
                Box::new(doors::DoorValidator {}),
            ],
        };
    }

    fn on_record(&mut self, context: &Context, record: &TES3Object, id: String) {
        for handler in self.handlers.iter_mut() {
            handler.on_record(context, record, &id);
        }
    }

    fn on_cellref(&mut self, context: &Context, record: &Cell, reference: &Reference, id: String) {
        for handler in self.handlers.iter_mut() {
            handler.on_cellref(context, record, reference, &id);
        }
    }

    fn on_info(&mut self, context: &Context, record: &Info, topic: &Dialogue) {
        for handler in self.handlers.iter_mut() {
            handler.on_info(context, record, topic);
        }
    }

    fn on_leveled(
        &mut self,
        context: &Context,
        record: &TES3Object,
        entry: &(String, u16),
        id: &String,
    ) {
        for handler in self.handlers.iter_mut() {
            handler.on_leveled(context, record, entry, id);
        }
    }

    fn on_inventory(
        &mut self,
        context: &Context,
        record: &TES3Object,
        entry: &(i32, FixedString<32>),
        id: &String,
    ) {
        for handler in self.handlers.iter_mut() {
            handler.on_inventory(context, record, entry, id);
        }
    }

    fn on_script(
        &mut self,
        context: &Context,
        record: &TES3Object,
        code: &str,
        comment: &str,
        topic: &Dialogue,
    ) {
        for handler in self.handlers.iter_mut() {
            handler.on_scriptline(context, record, code, comment, topic);
        }
    }
}

pub struct Validator {
    handlers: Handlers,
    context: Context,
}

impl Validator {
    pub fn new(context: Context) -> Validator {
        return Validator {
            handlers: Handlers::new(),
            context,
        };
    }

    pub fn validate(&mut self, records: &Vec<TES3Object>) {
        let dummy = Dialogue {
            flags: ObjectFlags::empty(),
            id: String::new(),
            kind: None,
        };
        let mut current_topic = &dummy;
        for record in records {
            match record {
                TES3Object::Activator(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Alchemy(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Apparatus(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Armor(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Birthsign(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Bodypart(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Book(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Cell(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase());
                    for reference in r.references.values() {
                        self.handlers.on_cellref(
                            &self.context,
                            r,
                            reference,
                            reference.id.to_ascii_lowercase(),
                        );
                    }
                }
                TES3Object::Class(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Clothing(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Container(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase());
                    self.on_inventory(record, &r.inventory);
                }
                TES3Object::Creature(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase());
                    self.on_inventory(record, &r.inventory);
                }
                TES3Object::Dialogue(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase());
                    current_topic = r;
                }
                TES3Object::Door(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Enchanting(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Faction(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::GameSetting(_) => {}
                TES3Object::GlobalVariable(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Header(_) => {}
                TES3Object::Info(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase());
                    self.handlers.on_info(&self.context, r, current_topic);
                    self.on_script(record, &r.script_text, current_topic);
                }
                TES3Object::Ingredient(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Landscape(_) => {}
                TES3Object::LandscapeTexture(_) => {}
                TES3Object::LeveledCreature(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase());
                    self.on_leveled(record, &r.creatures);
                }
                TES3Object::LeveledItem(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase());
                    self.on_leveled(record, &r.items);
                }
                TES3Object::Light(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Lockpick(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::MagicEffect(_) => {}
                TES3Object::MiscItem(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Npc(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase());
                    self.on_inventory(record, &r.inventory);
                }
                TES3Object::PathGrid(_) => {
                    self.handlers
                        .on_record(&self.context, record, String::new())
                }
                TES3Object::Probe(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Race(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Region(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::RepairItem(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Script(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase());
                    self.on_script(record, &r.script_text, &dummy);
                }
                TES3Object::Skill(_) => {}
                TES3Object::Sound(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::SoundGen(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Spell(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::StartScript(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Static(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
                TES3Object::Weapon(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.id.to_ascii_lowercase())
                }
            }
        }
    }

    fn on_leveled(&mut self, record: &TES3Object, entries: &Option<Vec<(String, u16)>>) {
        match entries {
            Some(list) => {
                for entry in list {
                    let id = entry.0.to_ascii_lowercase();
                    self.handlers.on_leveled(&self.context, record, entry, &id);
                }
            }
            None => {}
        }
    }

    fn on_inventory(
        &mut self,
        record: &TES3Object,
        inventory: &Option<Vec<(i32, FixedString<32>)>>,
    ) {
        match inventory {
            Some(list) => {
                for entry in list {
                    let id = entry.1.to_ascii_lowercase();
                    self.handlers
                        .on_inventory(&self.context, record, entry, &id);
                }
            }
            None => {}
        }
    }

    fn on_script(&mut self, record: &TES3Object, script: &Option<String>, topic: &Dialogue) {
        match script {
            Some(text) => {
                let empty = "";
                for line in text.trim().split('\n') {
                    let code: &str;
                    let comment: &str;
                    match line.split_once(';') {
                        Some(s) => {
                            code = s.0;
                            comment = s.1;
                        }
                        None => {
                            code = line;
                            comment = empty;
                        }
                    }
                    if !code.is_empty() || !comment.is_empty() {
                        self.handlers
                            .on_script(&self.context, record, code, comment, topic);
                    }
                }
            }
            None => {}
        }
    }
}
