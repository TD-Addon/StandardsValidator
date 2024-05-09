use crate::context::Context;
use std::error::Error;
use tes3::esp::{Cell, Dialogue, FixedString, Info, Reference, TES3Object};

#[allow(unused_variables)]
pub trait Handler<'a> {
    fn on_record(
        &mut self,
        context: &Context,
        record: &'a TES3Object,
        typename: &'static str,
        id: &String,
    ) {
    }

    fn on_cellref(&mut self, context: &Context, record: &Cell, reference: &Reference) {}

    fn on_leveled(&mut self, context: &Context, record: &TES3Object, entry: &(String, u16)) {}

    fn on_inventory(
        &mut self,
        context: &Context,
        record: &TES3Object,
        entry: &(i32, FixedString<32>),
    ) {
    }

    fn on_info(&mut self, context: &Context, record: &Info, topic: &Dialogue) {}

    fn on_scriptline(
        &mut self,
        context: &Context,
        record: &TES3Object,
        code: &str,
        comment: &str,
        topic: &Dialogue,
    ) {
    }

    fn on_end(&mut self, context: &Context) {}
}

pub struct Handlers<'a> {
    handlers: Vec<Box<dyn Handler<'a> + 'a>>,
}

impl Handlers<'_> {
    pub fn new<'a>() -> Result<Handlers<'a>, Box<dyn Error>> {
        return Ok(Handlers {
            handlers: vec![
                Box::new(crate::validators::test::TestValidator {}),
                Box::new(crate::validators::autocalc::AutoCalcValidator {}),
                Box::new(crate::validators::books::BookValidator {}),
                Box::new(crate::validators::cells::CellValidator::new()?),
                Box::new(crate::validators::classes::ClassValidator::new()?),
                Box::new(crate::validators::corpse::CorpseValidator {}),
                Box::new(crate::validators::dialogue::DialogueValidator::new()?),
                Box::new(crate::validators::doors::DoorValidator {}),
                Box::new(crate::validators::ids::IdValidator::new()),
                Box::new(crate::validators::keys::KeyValidator::new()),
                Box::new(crate::validators::leveled::LeveledValidator::new()),
            ],
        });
    }
}

impl<'a> Handler<'a> for Handlers<'a> {
    fn on_record(
        &mut self,
        context: &Context,
        record: &'a TES3Object,
        typename: &'static str,
        id: &String,
    ) {
        for handler in self.handlers.iter_mut() {
            handler.on_record(context, record, typename, &id);
        }
    }

    fn on_cellref(&mut self, context: &Context, record: &Cell, reference: &Reference) {
        for handler in self.handlers.iter_mut() {
            handler.on_cellref(context, record, reference);
        }
    }

    fn on_info(&mut self, context: &Context, record: &Info, topic: &Dialogue) {
        for handler in self.handlers.iter_mut() {
            handler.on_info(context, record, topic);
        }
    }

    fn on_leveled(&mut self, context: &Context, record: &TES3Object, entry: &(String, u16)) {
        for handler in self.handlers.iter_mut() {
            handler.on_leveled(context, record, entry);
        }
    }

    fn on_inventory(
        &mut self,
        context: &Context,
        record: &TES3Object,
        entry: &(i32, FixedString<32>),
    ) {
        for handler in self.handlers.iter_mut() {
            handler.on_inventory(context, record, entry);
        }
    }

    fn on_scriptline(
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

    fn on_end(&mut self, context: &Context) {
        for handler in self.handlers.iter_mut() {
            handler.on_end(context);
        }
    }
}