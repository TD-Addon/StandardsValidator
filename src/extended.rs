use tes3::esp::{Dialogue, Info, ObjectFlags, TES3Object};

use self::names::{NameValidator, QuestNameValidator};

mod names;

pub struct ExtendedValidator {
    handlers: Vec<Box<dyn ExtendedHandler>>,
}

#[allow(unused_variables)]
trait ExtendedHandler {
    fn on_record(&mut self, record: &TES3Object, typename: &'static str, id: &String, file: &str, last: bool) {}

    fn on_info(&mut self, record: &Info, topic: &Dialogue, file: &str, last: bool) {}

    fn on_end(&mut self) {}
}

impl ExtendedValidator {
    pub fn new(extended: bool, names: bool) -> Self {
        let mut handlers: Vec<Box<dyn ExtendedHandler>> = Vec::new();
        if extended {}
        if names {
            handlers.push(Box::new(NameValidator::new()));
            handlers.push(Box::new(QuestNameValidator::new()));
        }
        return Self { handlers };
    }

    pub fn validate(&mut self, records: &Vec<TES3Object>, file: &str, last: bool) {
        let dummy = Dialogue {
            flags: ObjectFlags::empty(),
            id: String::new(),
            kind: None,
        };
        let mut current_topic = &dummy;
        for record in records {
            match record {
                TES3Object::Activator(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Alchemy(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Apparatus(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Armor(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Birthsign(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Bodypart(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Book(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Cell(r) => {
                    self.on_record(record, r.type_name(), &r.id, file, last);
                }
                TES3Object::Class(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Clothing(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Container(r) => {
                    self.on_record(record, r.type_name(), &r.id, file, last);
                }
                TES3Object::Creature(r) => {
                    self.on_record(record, r.type_name(), &r.id, file, last);
                }
                TES3Object::Dialogue(r) => {
                    self.on_record(record, r.type_name(), &r.id, file, last);
                    current_topic = r;
                }
                TES3Object::Door(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Enchanting(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Faction(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::GameSetting(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::GlobalVariable(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Header(_) => {}
                TES3Object::Info(r) => {
                    self.on_record(record, r.type_name(), &r.id, file, last);
                    self.on_info(r, current_topic, file, last);
                }
                TES3Object::Ingredient(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Landscape(_) => {}
                TES3Object::LandscapeTexture(_) => {}
                TES3Object::LeveledCreature(r) => {
                    self.on_record(record, r.type_name(), &r.id, file, last);
                }
                TES3Object::LeveledItem(r) => {
                    self.on_record(record, r.type_name(), &r.id, file, last);
                }
                TES3Object::Light(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Lockpick(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::MagicEffect(r) => {
                    self.on_record(record, r.type_name(), &String::new(), file, last)
                }
                TES3Object::MiscItem(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Npc(r) => {
                    self.on_record(record, r.type_name(), &r.id, file, last);
                }
                TES3Object::PathGrid(r) => {
                    self.on_record(record, r.type_name(), &String::new(), file, last)
                }
                TES3Object::Probe(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Race(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Region(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::RepairItem(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Script(r) => {
                    self.on_record(record, r.type_name(), &r.id, file, last);
                }
                TES3Object::Skill(_) => {}
                TES3Object::Sound(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::SoundGen(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Spell(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::StartScript(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Static(r) => self.on_record(record, r.type_name(), &r.id, file, last),
                TES3Object::Weapon(r) => self.on_record(record, r.type_name(), &r.id, file, last),
            }
        }
        if last {
            for handler in &mut self.handlers {
                handler.on_end();
            }
        }
    }

    fn on_record(&mut self, record: &TES3Object, typename: &'static str, id: &String, file: &str, last: bool) {
        for handler in &mut self.handlers {
            handler.on_record(record, typename, id, file, last);
        }
    }

    fn on_info(&mut self, record: &Info, topic: &Dialogue, file: &str, last: bool) {
        for handler in &mut self.handlers {
            handler.on_info(record, topic, file, last);
        }
    }
}
