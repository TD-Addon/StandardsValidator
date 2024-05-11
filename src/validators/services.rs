use std::collections::HashSet;

use super::Context;
use crate::{handlers::Handler, util::is_autocalc};
use serde::Deserialize;
use tes3::esp::{AiData, TES3Object};

const FLAG_SERVICE_WEAPON: u32 = 0x1;
const FLAG_SERVICE_ARMOR: u32 = 0x2;
const FLAG_SERVICE_CLOTHING: u32 = 0x4;
const FLAG_SERVICE_BOOKS: u32 = 0x8;
const FLAG_SERVICE_INGREDIENTS: u32 = 0x10;
const FLAG_SERVICE_LOCKPICKS: u32 = 0x20;
const FLAG_SERVICE_PROBES: u32 = 0x40;
const FLAG_SERVICE_LIGHTS: u32 = 0x80;
const FLAG_SERVICE_APPARATUS: u32 = 0x100;
const FLAG_SERVICE_REPAIR_ITEMS: u32 = 0x200;
const FLAG_SERVICE_MISC: u32 = 0x400;
// const FLAG_SERVICE_SPELLS: u32 = 0x0800;
const FLAG_SERVICE_MAGIC_ITEMS: u32 = 0x1000;
const FLAG_SERVICE_POTIONS: u32 = 0x2000;
// const FLAG_SERVICE_TRAINING: u32 = 0x4000;
// const FLAG_SERVICE_SPELLMAKING: u32 = 0x8000;
// const FLAG_SERVICE_ENCHANTING: u32 = 0x10000;
// const FLAG_SERVICE_REPAIR: u32 = 0x20000;
const FLAGS_BARTER: u32 = FLAG_SERVICE_WEAPON
    | FLAG_SERVICE_ARMOR
    | FLAG_SERVICE_CLOTHING
    | FLAG_SERVICE_BOOKS
    | FLAG_SERVICE_INGREDIENTS
    | FLAG_SERVICE_LOCKPICKS
    | FLAG_SERVICE_PROBES
    | FLAG_SERVICE_LIGHTS
    | FLAG_SERVICE_APPARATUS
    | FLAG_SERVICE_REPAIR_ITEMS
    | FLAG_SERVICE_MISC
    | FLAG_SERVICE_POTIONS;

fn barters(option: &Option<AiData>) -> bool {
    if let Some(data) = option {
        return (data.services & FLAGS_BARTER) != 0;
    }
    return false;
}

fn buy_magic_items(option: &Option<AiData>) -> bool {
    if let Some(data) = option {
        return (data.services & FLAG_SERVICE_MAGIC_ITEMS) != 0;
    }
    return false;
}

pub struct ServiceValidator {
    barter_classes: HashSet<String>,
}

impl Handler<'_> for ServiceValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object, _: &str, _: &String) {
        match record {
            TES3Object::Class(class) => {
                if let Some(data) = &class.data {
                    if data.auto_calc_flags & FLAGS_BARTER != 0 {
                        self.barter_classes.insert(class.id.to_ascii_lowercase());
                        return;
                    }
                }
                if self.barter_classes.contains(&class.id.to_ascii_lowercase()) {
                    println!("Class {} does not barter", class.id);
                }
            }
            TES3Object::Creature(creature) => {
                let has_gold = creature.data.as_ref().map(|d| d.gold != 0).unwrap_or(false);
                if barters(&creature.ai_data) {
                    if !has_gold {
                        println!("Creature {} does not have any barter gold", creature.id);
                    }
                } else {
                    if buy_magic_items(&creature.ai_data) {
                        println!(
                            "Creature {} buys magic items but does not have a barter menu",
                            creature.id
                        );
                    } else if has_gold {
                        println!(
                            "Creature {} has barter gold but does not barter",
                            creature.id
                        );
                    }
                }
            }
            TES3Object::Npc(npc) => {
                let mut barter_menu = false;
                if is_autocalc(npc) {
                    if let Some(class) = &npc.class {
                        barter_menu = self.barter_classes.contains(&class.to_ascii_lowercase());
                    }
                } else {
                    barter_menu = barters(&npc.ai_data);
                    if let Some(class) = &npc.class {
                        if !barter_menu && self.barter_classes.contains(&class.to_ascii_lowercase())
                        {
                            println!("Npc {} has class {} but does not barter", npc.id, class);
                        }
                    }
                }
                let has_gold = npc.data.as_ref().map(|d| d.gold != 0).unwrap_or(false);
                if barter_menu {
                    if !has_gold {
                        println!("Npc {} does not have any barter gold", npc.id);
                    }
                } else {
                    if buy_magic_items(&npc.ai_data) {
                        println!(
                            "Npc {} buys magic items but does not have a barter menu",
                            npc.id
                        );
                    } else if has_gold {
                        println!("Npc {} has barter gold but does not barter", npc.id);
                    }
                }
            }
            _ => {}
        }
    }
}

#[derive(Deserialize)]
pub struct Services {
    barter: Vec<String>,
}

impl ServiceValidator {
    pub fn new() -> Result<Self, serde_json::Error> {
        let services: Services = serde_json::from_str(include_str!("../../data/services.json"))?;
        let mut barter_classes = HashSet::new();
        for class in &services.barter {
            barter_classes.insert(class.to_ascii_lowercase());
        }
        return Ok(Self { barter_classes });
    }
}
