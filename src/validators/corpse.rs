use super::Context;
use crate::{
    handlers::Handler,
    util::{is_dead, is_persistent},
};
use tes3::esp::TES3Object;

pub struct CorpseValidator {}

impl Handler<'_> for CorpseValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object, typename: &str, id: &String) {
        if is_dead(record) && !is_persistent(record) {
            println!(
                "{} {} is dead but does not have corpse persists checked",
                typename, id
            );
        }
    }
}
