use crate::validators::Context;
use tes3::esp::{Cell, Dialogue, FixedString, Info, Reference, TES3Object};

pub trait RecordHandler {
    fn on_record(&mut self, context: &Context, record: &TES3Object, id: &String);
}

pub trait CellHandler {
    fn on_cellref(&mut self, context: &Context, record: &Cell, reference: &Reference, id: &String);
}

pub trait LeveledListHandler {
    fn on_leveled(
        &mut self,
        context: &Context,
        record: &TES3Object,
        entry: &(String, u16),
        id: &String,
    );
}

pub trait InventoryHandler {
    fn on_inventory(
        &mut self,
        context: &Context,
        record: &TES3Object,
        entry: &(i32, FixedString<32>),
        id: &String,
    );
}

pub trait DialogueHandler {
    fn on_info(&mut self, context: &Context, record: &Info, topic: &Dialogue);
}

pub trait ScriptHandler {
    fn on_scriptline(
        &mut self,
        context: &Context,
        record: &TES3Object,
        code: &str,
        comment: &str,
        topic: &Dialogue,
    );
}
