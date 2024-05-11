use std::collections::HashMap;

use tes3::esp::TES3Object;

use super::ExtendedHandler;

pub struct WeaponValidator {
    weapons: HashMap<String, BaseWeapon>,
}

const IGNORES_RESIST: u32 = 1;
const SILVER: u32 = 2;

struct BaseWeapon {
    ignores: Option<bool>,
    silver: bool,
    id: String,
}

impl ExtendedHandler for WeaponValidator {
    fn on_record(&mut self, record: &TES3Object, _: &str, _: &String, _: &str, last: bool) {
        if let TES3Object::Weapon(weapon) = record {
            if let Some(name) = &weapon.name {
                if name.eq_ignore_ascii_case("<deprecated>") {
                    return;
                }
            }
            if let Some(mesh) = &weapon.mesh {
                let flags = weapon.data.as_ref().map(|d| d.flags).unwrap_or(0);
                let ignores;
                if weapon.enchanting.is_none() {
                    ignores = Some((flags & IGNORES_RESIST) != 0);
                } else {
                    ignores = None;
                }
                let silver = (flags & SILVER) != 0;
                let lower = mesh.to_ascii_lowercase();
                if let Some(base) = self.weapons.get_mut(&lower) {
                    if base.id.eq_ignore_ascii_case(&weapon.id) {
                        base.silver = silver;
                        base.ignores = ignores;
                    } else if base.ignores.is_none() && weapon.enchanting.is_none() {
                        base.silver = silver;
                        base.ignores = ignores;
                        base.id = weapon.id.clone();
                    } else if last {
                        if base.silver != silver {
                            println!(
                                "Weapon {} has a different silver value than {}",
                                weapon.id, base.id
                            );
                        }
                        if base.ignores != ignores && base.ignores.is_some() && ignores.is_some() {
                            println!("Weapon {} has a different ignores normal weapon resistance value than {}", weapon.id, base.id);
                        }
                    }
                } else {
                    self.weapons.insert(
                        mesh.to_ascii_lowercase(),
                        BaseWeapon {
                            ignores,
                            silver,
                            id: weapon.id.clone(),
                        },
                    );
                }
            }
        }
    }
}

impl WeaponValidator {
    pub fn new() -> Self {
        return Self {
            weapons: HashMap::new(),
        };
    }
}
