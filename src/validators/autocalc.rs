use super::Context;
use crate::handlers::Handler;
use tes3::esp::TES3Object;

const FLAG_SPELL_AUTO_CALC: u32 = 1;

pub struct AutoCalcValidator {}

impl Handler<'_> for AutoCalcValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object, _: &str, _: &String) {
        if let TES3Object::Spell(spell) = record {
            if let Some(data) = &spell.data {
                if (data.flags & FLAG_SPELL_AUTO_CALC) != 0 {
                    println!("Spell {} is auto calculated", spell.id);
                }
            }
        }
    }
}
