use std::collections::HashMap;

use tes3::esp::{TES3Object, WeaponFlags};

use super::ExtendedHandler;

pub struct WeaponValidator {
    weapons: HashMap<String, BaseWeapon>,
}

struct BaseWeapon {
    ignores: Option<bool>,
    silver: bool,
    id: String,
}

impl ExtendedHandler for WeaponValidator {
    fn on_record(&mut self, record: &TES3Object, _: &str, _: &str, _: &str, last: bool) {
        if let TES3Object::Weapon(weapon) = record {
            if weapon.name.eq_ignore_ascii_case("<deprecated>") {
                return;
            }
            let mesh = &weapon.mesh;
            let flags = weapon.data.flags;

            let ignores = if weapon.enchanting.is_empty() {
                Some(flags.contains(WeaponFlags::IGNORES_NORMAL_WEAPON_RESISTANCE))
            } else {
                None
            };
            let silver = flags.contains(WeaponFlags::SILVER);
            let lower = mesh.to_ascii_lowercase();
            if let Some(base) = self.weapons.get_mut(&lower) {
                if base.id.eq_ignore_ascii_case(&weapon.id) {
                    base.silver = silver;
                    base.ignores = ignores;
                } else if base.ignores.is_none() && weapon.enchanting.is_empty() {
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

impl WeaponValidator {
    pub fn new() -> Self {
        Self {
            weapons: HashMap::new(),
        }
    }
}
