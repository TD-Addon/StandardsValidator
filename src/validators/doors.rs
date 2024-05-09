use super::Context;
use crate::{handlers::Handler, util::get_cell_name};
use tes3::esp::{Cell, Reference, TES3Object};

pub struct DoorValidator {}

impl Handler<'_> for DoorValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object, _: &'static str, _: &String) {
        if let TES3Object::Door(door) = record {
            if let Some(mesh) = &door.mesh {
                if mesh.eq_ignore_ascii_case("i\\in_lava_blacksquare.nif") {
                    println!("Door {} uses mesh {}", door.id, mesh);
                }
            }
        }
    }

    fn on_cellref(&mut self, _: &Context, record: &Cell, reference: &Reference, _: &Vec<&Reference>, _: usize) {
        if reference.id.eq_ignore_ascii_case("prisonmarker")
            && reference.door_destination_cell.is_none()
        {
            println!(
                "Cell {} contains an unlinked {}",
                get_cell_name(record),
                reference.id
            );
        }
    }
}
