use super::Context;
use crate::handlers::Handler;
use tes3::esp::{Cell, EditorId, Reference, TES3Object};

pub struct DoorValidator {}

impl Handler<'_> for DoorValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object) {
        if let TES3Object::Door(door) = record {
            if door.mesh.eq_ignore_ascii_case("i\\in_lava_blacksquare.nif") {
                println!("Door {} uses mesh {}", door.id, door.mesh);
            }
        }
    }

    fn on_cellref(
        &mut self,
        _: &Context,
        record: &Cell,
        reference: &Reference,
        id: &str,
        _: &[&Reference],
        _: usize,
    ) {
        if id == "prisonmarker" && reference.destination.is_none() {
            println!(
                "Cell {} contains an unlinked {}",
                record.editor_id(),
                reference.id
            );
        }
    }
}
