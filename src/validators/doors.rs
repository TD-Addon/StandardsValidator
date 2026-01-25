use super::Context;
use crate::handlers::Handler;
use tes3::esp::{Cell, EditorId, Reference, TES3Object};

const SCRIPTED_DOORS: [&str; 8] = [
    "t_ayl_dngruin_doorirondbl_01",
    "t_ayl_dngruin_doorirondblopn_01",
    "t_ayl_dngruin_doorironsqr_01",
    "t_he_dngdirenni_doorin_01",
    "t_he_dngdirenni_doorin_02",
    "t_de_setind_doorrectin",
    "t_cyrimp_furnr_display1",
    "t_cyrimp_furnr_display2",
];

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
        } else if SCRIPTED_DOORS.contains(&id) {
            if reference.trap.is_some() {
                println!(
                    "Cell {} contains a trapped {}",
                    record.editor_id(),
                    reference.id
                );
            }
            if let Some(key) = &reference.key {
                println!(
                    "Cell {} contains {} unlocked with {}",
                    record.editor_id(),
                    reference.id,
                    key
                );
            }
        }
    }
}
