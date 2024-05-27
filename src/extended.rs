use clap::ArgMatches;
use tes3::esp::{Dialogue, DialogueInfo, TES3Object};

use self::{
    cells::CellValidator,
    items::OwnershipValidator,
    names::{NameValidator, QuestNameValidator},
    weapons::WeaponValidator,
};

mod cells;
mod items;
mod names;
mod weapons;

pub struct ExtendedValidator {
    handlers: Vec<Box<dyn ExtendedHandler>>,
}

#[allow(unused_variables)]
trait ExtendedHandler {
    fn on_record(&mut self, record: &TES3Object, file: &str, last: bool) {}

    fn on_info(&mut self, record: &DialogueInfo, topic: &Dialogue, file: &str, last: bool) {}

    fn on_end(&mut self) {}
}

impl ExtendedValidator {
    pub fn new(args: &ArgMatches) -> Self {
        let mut handlers: Vec<Box<dyn ExtendedHandler>> = Vec::new();
        let extended = args.get_flag("extended");
        let names = args.get_flag("names");
        if extended {
            handlers.push(Box::new(CellValidator::new(args)));
            handlers.push(Box::new(OwnershipValidator::new()));
            handlers.push(Box::new(WeaponValidator::new()));
        }
        if names {
            handlers.push(Box::new(NameValidator::new()));
            handlers.push(Box::new(QuestNameValidator::new()));
        }
        Self { handlers }
    }

    pub fn validate(&mut self, records: &Vec<TES3Object>, file: &str, last: bool) {
        let dummy = Dialogue::default();
        let mut current_topic = &dummy;
        for record in records {
            match record {
                TES3Object::Dialogue(r) => {
                    self.on_record(record, file, last);
                    current_topic = r;
                }
                TES3Object::Header(_) => {}
                TES3Object::DialogueInfo(r) => {
                    self.on_record(record, file, last);
                    self.on_info(r, current_topic, file, last);
                }
                TES3Object::Landscape(_) => {}
                TES3Object::LandscapeTexture(_) => {}
                TES3Object::Skill(_) => {}
                _ => self.on_record(record, file, last),
            }
        }
        if last {
            for handler in &mut self.handlers {
                handler.on_end();
            }
        }
    }

    fn on_record(&mut self, record: &TES3Object, file: &str, last: bool) {
        for handler in &mut self.handlers {
            handler.on_record(record, file, last);
        }
    }

    fn on_info(&mut self, record: &DialogueInfo, topic: &Dialogue, file: &str, last: bool) {
        for handler in &mut self.handlers {
            handler.on_info(record, topic, file, last);
        }
    }
}
