use std::collections::HashSet;

use tes3::esp::{EditorId, LightFlags, TES3Object};

use crate::util::cannot_sleep;

use super::ExtendedHandler;

pub struct OwnershipValidator {
    items: HashSet<String>,
    ownable: HashSet<String>,
}

impl ExtendedHandler for OwnershipValidator {
    fn on_record(&mut self, record: &TES3Object, _: &str, last: bool) {
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
                if !last {
                    return;
                }
                let mut owned = 0;
                let mut unowned = 0;
                let name = cell.editor_id();
                for reference in cell.references.values() {
                    let lower = reference.id.to_ascii_lowercase();
                    let scale = reference.scale.unwrap_or(1.);
                    if scale != 1. && self.items.contains(&lower) {
                        println!(
                            "Cell {} contains {} with scale {}",
                            name, reference.id, scale
                        );
                    }

                    let locked = reference.lock_level.is_some();
                    let has_trap = matches!(&reference.trap, Some(s) if !s.is_empty());
                    let has_owner = matches!(&reference.owner, Some(s) if !s.is_empty());
                    let has_owner_faction =
                        matches!(&reference.owner_faction, Some(s) if !s.is_empty());

                    if (locked || has_trap)
                        || self.items.contains(&lower)
                        || self.ownable.contains(&lower)
                    {
                        if has_owner || has_owner_faction {
                            owned += 1;
                        } else {
                            unowned += 1;
                        }
                    } else if has_owner || has_owner_faction {
                        println!(
                            "Cell {} contains incorrectly owned object {}",
                            name, reference.id
                        );
                    }
                }
                if cannot_sleep(cell) {
                    if unowned > 0 {
                        println!("Cell {} contains {} unowned items", name, unowned);
                    }
                } else if owned > 0 {
                    println!("Cell {} contains {} owned items", name, owned);
                }
            }
            _ => {}
        }
    }
}

impl OwnershipValidator {
    pub fn new() -> Self {
        Self {
            items: HashSet::new(),
            ownable: HashSet::new(),
        }
    }
}
