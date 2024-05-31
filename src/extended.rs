use clap::ArgMatches;
use deprecated::DeprecationValidator;
use tes3::esp::{Cell, Dialogue, DialogueInfo, Reference, TES3Object};

use crate::context::Context;

use self::{
    cells::CellValidator,
    items::OwnershipValidator,
    names::{NameValidator, QuestNameValidator},
    weapons::WeaponValidator,
};

mod cells;
mod deprecated;
mod items;
mod names;
mod weapons;

pub struct ExtendedValidator {
    handlers: Vec<Box<dyn ExtendedHandler>>,
}

#[allow(unused_variables)]
trait ExtendedHandler {
    fn on_record(&mut self, context: &Context, record: &TES3Object, file: &str, last: bool) {}

    fn on_cellref(&mut self, context: &Context, record: &Cell, reference: &Reference, id: &str) {}

    fn on_info(
        &mut self,
        context: &Context,
        record: &DialogueInfo,
        topic: &Dialogue,
        file: &str,
        last: bool,
    ) {
    }

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
            handlers.push(Box::new(DeprecationValidator::new()));
        }
        if names {
            handlers.push(Box::new(NameValidator::new()));
            handlers.push(Box::new(QuestNameValidator::new()));
        }
        Self { handlers }
    }

    pub fn validate(
        &mut self,
        records: &Vec<TES3Object>,
        file: &str,
        last: bool,
        context: &Context,
    ) {
        let dummy = Dialogue::default();
        let mut current_topic = &dummy;
        for record in records {
            match record {
                TES3Object::Cell(cell) => {
                    self.on_record(context, record, file, last);
                    if last {
                        for reference in cell.references.values() {
                            self.on_cellref(context, cell, reference);
                        }
                    }
                }
                TES3Object::Dialogue(r) => {
                    self.on_record(context, record, file, last);
                    current_topic = r;
                }
                TES3Object::Header(_) => {}
                TES3Object::DialogueInfo(r) => {
                    self.on_record(context, record, file, last);
                    self.on_info(context, r, current_topic, file, last);
                }
                TES3Object::Landscape(_) => {}
                TES3Object::LandscapeTexture(_) => {}
                TES3Object::Skill(_) => {}
                _ => self.on_record(context, record, file, last),
            }
        }
        if last {
            for handler in &mut self.handlers {
                handler.on_end();
            }
        }
    }

    fn on_record(&mut self, context: &Context, record: &TES3Object, file: &str, last: bool) {
        for handler in &mut self.handlers {
            handler.on_record(context, record, file, last);
        }
    }

    fn on_cellref(&mut self, context: &Context, record: &Cell, reference: &Reference) {
        let id = reference.id.to_ascii_lowercase();
        for handler in &mut self.handlers {
            handler.on_cellref(context, record, reference, &id);
        }
    }

    fn on_info(
        &mut self,
        context: &Context,
        record: &DialogueInfo,
        topic: &Dialogue,
        file: &str,
        last: bool,
    ) {
        for handler in &mut self.handlers {
            handler.on_info(context, record, topic, file, last);
        }
    }
}
