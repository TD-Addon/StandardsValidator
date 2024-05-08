use super::Context;
use crate::handler_traits::Handler;
use tes3::esp::TES3Object;

const FLAG_SPELL_AUTO_CALC: u32 = 1;

pub struct AutoCalcValidator {}

impl Handler for AutoCalcValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object, _: &String) {
        match record {
            TES3Object::Spell(spell) => {
                if let Some(data) = &spell.data {
                    if (data.flags & FLAG_SPELL_AUTO_CALC) != 0 {
                        println!("Spell {} is auto calculated", spell.id);
                    }
                }
            }
            _ => {}
        }
    }
}
