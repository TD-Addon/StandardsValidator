use super::Context;
use crate::{
    handler_traits::Handler,
    util::{is_dead, is_persistent},
};
use tes3::esp::TES3Object;

pub struct CorpseValidator {}

impl Handler<'_> for CorpseValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object, _: &String) {
        if is_dead(record) && !is_persistent(record) {
            match record {
                TES3Object::Creature(creature) => {
                    println!(
                        "Creature {} is dead but does not have corpse persists checked",
                        creature.id
                    );
                }
                TES3Object::Npc(npc) => {
                    println!(
                        "Npc {} is dead but does not have corpse persists checked",
                        npc.id
                    );
                }
                _ => {}
            }
        }
    }
}
