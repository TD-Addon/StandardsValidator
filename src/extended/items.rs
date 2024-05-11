use std::collections::HashSet;

use tes3::esp::TES3Object;

use crate::util::{cannot_sleep, get_cell_name, is_empty};

use super::ExtendedHandler;

pub struct OwnershipValidator {
    items: HashSet<String>,
    ownable: HashSet<String>,
}

impl ExtendedHandler for OwnershipValidator {
    fn on_record(&mut self, record: &TES3Object, _: &str, id: &String, _: &str, last: bool) {
        match record {
            TES3Object::Activator(activator) => {
                if let Some(script) = &activator.script {
                    if script.eq_ignore_ascii_case("bed_standard") {
                        self.ownable.insert(id.to_ascii_lowercase());
                    }
                }
            }
            TES3Object::Alchemy(_) => {
                self.items.insert(id.to_ascii_lowercase());
            }
            TES3Object::Apparatus(_) => {
                self.items.insert(id.to_ascii_lowercase());
            }
            TES3Object::Armor(_) => {
                self.items.insert(id.to_ascii_lowercase());
            }
            TES3Object::Book(_) => {
                self.items.insert(id.to_ascii_lowercase());
            }
            TES3Object::Clothing(_) => {
                self.items.insert(id.to_ascii_lowercase());
            }
            TES3Object::Container(_) => {
                self.ownable.insert(id.to_ascii_lowercase());
            }
            TES3Object::Ingredient(_) => {
                self.items.insert(id.to_ascii_lowercase());
            }
            TES3Object::Light(light) => {
                if light.can_carry() {
                    self.items.insert(id.to_ascii_lowercase());
                }
            }
            TES3Object::Lockpick(_) => {
                self.items.insert(id.to_ascii_lowercase());
            }
            TES3Object::MiscItem(_) => {
                self.items.insert(id.to_ascii_lowercase());
            }
            TES3Object::Probe(_) => {
                self.items.insert(id.to_ascii_lowercase());
            }
            TES3Object::RepairItem(_) => {
                self.items.insert(id.to_ascii_lowercase());
            }
            TES3Object::Weapon(_) => {
                self.items.insert(id.to_ascii_lowercase());
            }
            TES3Object::Cell(cell) => {
                if !last {
                    return;
                }
                let name = get_cell_name(cell);
                let mut owned = 0;
                let mut unowned = 0;
                for reference in cell.references.values() {
                    let lower = reference.id.to_ascii_lowercase();
                    let scale = reference.scale.unwrap_or(1.);
                    if scale != 1. && self.items.contains(&lower) {
                        println!(
                            "Cell {} contains {} with scale {}",
                            name, reference.id, scale
                        );
                    }
                    if (reference.lock_level.is_some() || !is_empty(&reference.trap))
                        || self.items.contains(&lower)
                        || self.ownable.contains(&lower)
                    {
                        if !is_empty(&reference.owner) || !is_empty(&reference.owner_faction) {
                            owned += 1;
                        } else {
                            unowned += 1;
                        }
                    } else if !is_empty(&reference.owner) || !is_empty(&reference.owner_faction) {
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
        return Self {
            items: HashSet::new(),
            ownable: HashSet::new(),
        };
    }
}
