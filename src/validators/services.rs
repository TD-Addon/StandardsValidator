use std::collections::HashSet;

use super::Context;
use crate::{handlers::Handler, util::is_autocalc};
use tes3::esp::{AiData, ServiceFlags, TES3Object};

include!(concat!(env!("OUT_DIR"), "/gen_services.rs"));

const SERVICE_FLAGS_BARTERS_ANY: ServiceFlags = ServiceFlags::from_bits_truncate(
    ServiceFlags::BARTERS_WEAPONS.bits()
        | ServiceFlags::BARTERS_ARMOR.bits()
        | ServiceFlags::BARTERS_CLOTHING.bits()
        | ServiceFlags::BARTERS_BOOKS.bits()
        | ServiceFlags::BARTERS_INGREDIENTS.bits()
        | ServiceFlags::BARTERS_LOCKPICKS.bits()
        | ServiceFlags::BARTERS_PROBES.bits()
        | ServiceFlags::BARTERS_LIGHTS.bits()
        | ServiceFlags::BARTERS_APPARATUS.bits()
        | ServiceFlags::BARTERS_REPAIR_ITEMS.bits()
        | ServiceFlags::BARTERS_MISC_ITEMS.bits()
        | ServiceFlags::BARTERS_ALCHEMY.bits(),
);

fn barters(ai_data: &AiData) -> bool {
    ai_data.services.intersects(SERVICE_FLAGS_BARTERS_ANY)
}

fn buy_magic_items(ai_data: &AiData) -> bool {
    ai_data
        .services
        .contains(ServiceFlags::BARTERS_ENCHANTED_ITEMS)
}

pub struct ServiceValidator {
    barter_classes: HashSet<String>,
}

impl Handler<'_> for ServiceValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object) {
        match record {
            TES3Object::Class(class) => {
                if class.data.services.intersects(SERVICE_FLAGS_BARTERS_ANY) {
                    self.barter_classes.insert(class.id.to_ascii_lowercase());
                    return;
                }
                if self.barter_classes.contains(&class.id.to_ascii_lowercase()) {
                    println!("Class {} does not barter", class.id);
                }
            }
            TES3Object::Creature(creature) => {
                let has_gold = creature.data.gold != 0;
                if barters(&creature.ai_data) {
                    if !has_gold {
                        println!("Creature {} does not have any barter gold", creature.id);
                    }
                } else if buy_magic_items(&creature.ai_data) {
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
            TES3Object::Npc(npc) => {
                let mut barter_menu = false;
                if is_autocalc(npc) {
                    if !npc.class.is_empty() {
                        barter_menu = self
                            .barter_classes
                            .contains(&npc.class.to_ascii_lowercase());
                    }
                } else {
                    barter_menu = barters(&npc.ai_data);
                    if !npc.class.is_empty()
                        && !barter_menu
                        && self
                            .barter_classes
                            .contains(&npc.class.to_ascii_lowercase())
                    {
                        println!("Npc {} has class {} but does not barter", npc.id, npc.class);
                    }
                }
                let has_gold = npc.data.gold != 0;
                if barter_menu {
                    if !has_gold {
                        println!("Npc {} does not have any barter gold", npc.id);
                    }
                } else if buy_magic_items(&npc.ai_data) {
                    println!(
                        "Npc {} buys magic items but does not have a barter menu",
                        npc.id
                    );
                } else if has_gold {
                    println!("Npc {} has barter gold but does not barter", npc.id);
                }
            }
            _ => {}
        }
    }
}

impl ServiceValidator {
    pub fn new() -> Self {
        Self {
            barter_classes: get_barter_classes(),
        }
    }
}
