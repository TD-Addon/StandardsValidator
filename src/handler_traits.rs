use crate::validators::Context;
use tes3::esp::{Cell, Dialogue, FixedString, Info, Reference, TES3Object};

#[allow(unused_variables)]
pub trait Handler<'a> {
    fn on_record(&mut self, context: &Context, record: &'a TES3Object, id: &String) {}

    fn on_cellref(&mut self, context: &Context, record: &Cell, reference: &Reference, id: &String) {
    }

    fn on_leveled(
        &mut self,
        context: &Context,
        record: &TES3Object,
        entry: &(String, u16),
        id: &String,
    ) {
    }

    fn on_inventory(
        &mut self,
        context: &Context,
        record: &TES3Object,
        entry: &(i32, FixedString<32>),
        id: &String,
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
