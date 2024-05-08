use super::Context;
use crate::{handler_traits::Handler, util::get_cell_name};
use tes3::esp::{Cell, Reference, TES3Object};

pub struct DoorValidator {}

impl Handler for DoorValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object, _: &String) {
        match record {
            TES3Object::Door(door) => {
                if let Some(mesh) = &door.mesh {
                    if mesh.eq_ignore_ascii_case("i\\in_lava_blacksquare.nif") {
                        println!("Door {} uses mesh {}", door.id, mesh);
                    }
                }
            }
            _ => {}
        }
    }

    fn on_cellref(&mut self, _: &Context, record: &Cell, reference: &Reference, id: &String) {
        if id == "prisonmarker" && reference.door_destination_cell.is_none() {
            println!(
                "Cell {} contains an unlinked {}",
                get_cell_name(record),
                reference.id
            );
        }
    }
}
