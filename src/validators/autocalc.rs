use super::Context;
use crate::handlers::Handler;
use tes3::esp::{SpellFlags, TES3Object};

pub struct AutoCalcValidator {}

impl Handler<'_> for AutoCalcValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object) {
        if let TES3Object::Spell(spell) = record {
            if spell.data.flags.contains(SpellFlags::AUTO_CALCULATE) {
                println!("Spell {} is auto calculated", spell.id);
            }
        }
    }
}
