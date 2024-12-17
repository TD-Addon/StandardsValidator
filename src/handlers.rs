use crate::context::{Context, Mode};
use clap::ArgMatches;
use std::error::Error;
use tes3::esp::{Cell, Dialogue, DialogueInfo, FixedString, Reference, TES3Object};

#[allow(unused_variables)]
pub trait Handler<'a> {
    fn on_record(&mut self, context: &Context, record: &'a TES3Object) {}

    fn on_cellref(
        &mut self,
        context: &Context,
        record: &'a Cell,
        reference: &Reference,
        id: &str,
        refs: &[&Reference],
        i: usize,
    ) {
    }

    fn on_leveled(&mut self, context: &Context, record: &TES3Object, entry: &(String, u16)) {}

    fn on_inventory(
        &mut self,
        context: &Context,
        record: &TES3Object,
        entry: &(i32, FixedString<32>),
    ) {
    }

    fn on_info(&mut self, context: &Context, record: &'a DialogueInfo, topic: &Dialogue) {}

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
    pub fn new<'a>(context: &Context, args: &ArgMatches) -> Result<Handlers<'a>, Box<dyn Error>> {
        let npc_validator = Box::new(crate::validators::npc::NpcValidator::new()?);
        let unique_heads = npc_validator.get_unique_heads();
        let mut handlers: Vec<Box<dyn Handler<'a> + 'a>> = vec![
            Box::new(crate::validators::books::BookValidator {}),
            Box::new(crate::validators::cells::CellValidator::new()),
            Box::new(crate::validators::corpse::CorpseValidator {}),
            Box::new(crate::validators::duplicates::DuplicateRefValidator::new(
                args,
            )),
            Box::new(crate::validators::doors::DoorValidator {}),
            Box::new(crate::validators::keys::KeyValidator::new()),
            Box::new(crate::validators::leveled::LeveledValidator::new()),
            Box::new(crate::validators::lights::LightValidator {}),
            Box::new(crate::validators::dialogue::DialogueValidator::new()?),
            Box::new(crate::validators::magic::MagicValidator::new()),
            Box::new(crate::validators::missing::FieldValidator {}),
            npc_validator,
            Box::new(crate::validators::orphans::OrphanValidator::new()?),
            Box::new(crate::validators::persistent::PersistentValidator::new()),
            Box::new(crate::validators::scripts::ScriptValidator::new(
                context,
                unique_heads,
            )?),
            Box::new(crate::validators::services::ServiceValidator::new()),
            Box::new(crate::validators::soundgens::SoundGenValidator::new()),
            Box::new(crate::validators::supplies::SupplyChestValidator::new()),
            Box::new(crate::validators::todo::ToDoValidator::new()?),
            Box::new(crate::validators::travel::TravelValidator::new()),
            Box::new(crate::validators::unicode::UnicodeValidator::new()?),
        ];
        if context.mode.uses_td() {
            handlers.push(Box::new(crate::validators::classes::ClassValidator::new()));
        }
        if context.mode != Mode::Vanilla {
            handlers.push(Box::new(crate::validators::autocalc::AutoCalcValidator {}));
            handlers.push(Box::new(crate::validators::ids::IdValidator::new()));
            handlers.push(Box::new(
                crate::validators::uniques::UniquesValidator::new()?
            ));
        }
        Ok(Handlers { handlers })
    }
}

impl<'a> Handler<'a> for Handlers<'a> {
    fn on_record(&mut self, context: &Context, record: &'a TES3Object) {
        for handler in &mut self.handlers {
            handler.on_record(context, record);
        }
    }

    fn on_cellref(
        &mut self,
        context: &Context,
        record: &'a Cell,
        reference: &Reference,
        id: &str,
        refs: &[&Reference],
        i: usize,
    ) {
        for handler in &mut self.handlers {
            handler.on_cellref(context, record, reference, id, refs, i);
        }
    }

    fn on_info(&mut self, context: &Context, record: &'a DialogueInfo, topic: &Dialogue) {
        for handler in &mut self.handlers {
            handler.on_info(context, record, topic);
        }
    }

    fn on_leveled(&mut self, context: &Context, record: &TES3Object, entry: &(String, u16)) {
        for handler in &mut self.handlers {
            handler.on_leveled(context, record, entry);
        }
    }

    fn on_inventory(
        &mut self,
        context: &Context,
        record: &TES3Object,
        entry: &(i32, FixedString<32>),
    ) {
        for handler in &mut self.handlers {
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
        for handler in &mut self.handlers {
            handler.on_scriptline(context, record, code, comment, topic);
        }
    }

    fn on_end(&mut self, context: &Context) {
        for handler in &mut self.handlers {
            handler.on_end(context);
        }
    }
}
