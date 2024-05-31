use std::collections::HashSet;

use tes3::esp::{Cell, EditorId, LightFlags, Reference, TES3Object};

use crate::{context::Context, util::cannot_sleep};

use super::ExtendedHandler;

pub struct OwnershipValidator {
    items: HashSet<String>,
    ownable: HashSet<String>,
    owned: u32,
    unowned: u32,
    cell_name: String,
    is_dungeon: bool,
}

impl ExtendedHandler for OwnershipValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object, _: &str, last: bool) {
        self.end_cell();
        match record {
            TES3Object::Activator(activator) => {
                if !activator.script.is_empty()
                    && activator.script.eq_ignore_ascii_case("bed_standard")
                {
                    self.ownable
                        .insert(record.editor_id_ascii_lowercase().into_owned());
                }
            }
            TES3Object::Alchemy(_) => {
                self.items
                    .insert(record.editor_id_ascii_lowercase().into_owned());
            }
            TES3Object::Apparatus(_) => {
                self.items
                    .insert(record.editor_id_ascii_lowercase().into_owned());
            }
            TES3Object::Armor(_) => {
                self.items
                    .insert(record.editor_id_ascii_lowercase().into_owned());
            }
            TES3Object::Book(_) => {
                self.items
                    .insert(record.editor_id_ascii_lowercase().into_owned());
            }
            TES3Object::Clothing(_) => {
                self.items
                    .insert(record.editor_id_ascii_lowercase().into_owned());
            }
            TES3Object::Container(_) => {
                self.ownable
                    .insert(record.editor_id_ascii_lowercase().into_owned());
            }
            TES3Object::Ingredient(_) => {
                self.items
                    .insert(record.editor_id_ascii_lowercase().into_owned());
            }
            TES3Object::Light(light) => {
                if light.data.flags.contains(LightFlags::CAN_CARRY) {
                    self.items
                        .insert(record.editor_id_ascii_lowercase().into_owned());
                }
            }
            TES3Object::Lockpick(_) => {
                self.items
                    .insert(record.editor_id_ascii_lowercase().into_owned());
            }
            TES3Object::MiscItem(_) => {
                self.items
                    .insert(record.editor_id_ascii_lowercase().into_owned());
            }
            TES3Object::Probe(_) => {
                self.items
                    .insert(record.editor_id_ascii_lowercase().into_owned());
            }
            TES3Object::RepairItem(_) => {
                self.items
                    .insert(record.editor_id_ascii_lowercase().into_owned());
            }
            TES3Object::Weapon(_) => {
                self.items
                    .insert(record.editor_id_ascii_lowercase().into_owned());
            }
            TES3Object::Cell(cell) => {
                if last {
                    self.cell_name = cell.editor_id().into_owned();
                    self.is_dungeon = !cannot_sleep(cell);
                }
            }
            _ => {}
        }
    }

    fn on_cellref(&mut self, _: &Context, record: &Cell, reference: &Reference, lower: &str) {
        let scale = reference.scale.unwrap_or(1.);
        if scale != 1. && self.items.contains(lower) {
            println!(
                "Cell {} contains {} with scale {}",
                record.editor_id(),
                reference.id,
                scale
            );
        }

        let locked = reference.lock_level.is_some();
        let has_trap = matches!(&reference.trap, Some(s) if !s.is_empty());
        let has_owner = matches!(&reference.owner, Some(s) if !s.is_empty());
        let has_owner_faction = matches!(&reference.owner_faction, Some(s) if !s.is_empty());

        if (locked || has_trap) || self.items.contains(lower) || self.ownable.contains(lower) {
            if has_owner || has_owner_faction {
                self.owned += 1;
            } else {
                self.unowned += 1;
            }
        } else if has_owner || has_owner_faction {
            println!(
                "Cell {} contains incorrectly owned object {}",
                record.editor_id(),
                reference.id
            );
        }
    }

    fn on_end(&mut self) {
        self.end_cell();
    }
}

impl OwnershipValidator {
    pub fn new() -> Self {
        Self {
            items: HashSet::new(),
            ownable: HashSet::new(),
            owned: 0,
            unowned: 0,
            cell_name: String::new(),
            is_dungeon: false,
        }
    }

    fn end_cell(&mut self) {
        if !self.is_dungeon {
            if self.unowned > 0 {
                println!(
                    "Cell {} contains {} unowned items",
                    self.cell_name, self.unowned
                );
            }
        } else if self.owned > 0 {
            println!(
                "Cell {} contains {} owned items",
                self.cell_name, self.owned
            );
        }
        self.unowned = 0;
        self.owned = 0;
    }
}
