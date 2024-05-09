use std::collections::HashMap;

use super::Context;
use crate::{context::Mode, handlers::Handler};
use tes3::esp::{Bodypart, BodypartId, TES3Object};

const VANILLA_FACTIONS: [&str; 27] = [
    "Ashlanders",
    "Blades",
    "Camonna Tong",
    "Census and Excise",
    "Clan Aundae",
    "Clan Berne",
    "Clan Quarra",
    "Fighters Guild",
    "Hlaalu",
    "Imperial Cult",
    "Imperial Knights",
    "Imperial Legion",
    "Mages Guild",
    "Morag Tong",
    "Nerevarine",
    "Redoran",
    "Sixth House",
    "Talos Cult",
    "Telvanni",
    "Temple",
    "Thieves Guild",
    "Twin Lamps",
    "Dark Brotherhood",
    "Hands of Almalexia",
    "Royal Guard",
    "East Empire Company",
    "Skaal",
];

fn is_female(part: &Bodypart) -> bool {
    if let Some(data) = &part.data {
        return (data.female & 1) != 0;
    }
    return false;
}

fn is_vampire_head(part: &Bodypart) -> bool {
    if let Some(data) = &part.data {
        return data.vampire != 0 && data.part == BodypartId::Head;
    }
    return false;
}

fn check_id(context: &Context, t: &str, id: &String) {
    let matching = context.projects.iter().find(|p| p.matches(id));
    match matching {
        Some(project) => {
            if context.mode != Mode::TD && project.prefix == "T_" {
                println!("{} {} has a {} ID", t, id, project.name);
            }
        }
        None => {
            println!("{} {} does not match a known ID scheme", t, id);
        }
    }
}

pub struct IdValidator {
    known: HashMap<String, &'static str>,
}

impl Handler<'_> for IdValidator {
    fn on_record(
        &mut self,
        context: &Context,
        record: &TES3Object,
        typename: &'static str,
        id: &String,
    ) {
        match record {
            TES3Object::Bodypart(part) => {
                if part.name.is_some() && is_vampire_head(part) {
                    let id = format!(
                        "b_v_{}_{}_head_01",
                        part.name.as_ref().unwrap(),
                        if is_female(part) { "f" } else { "m" }
                    );
                    if part.id != id {
                        println!("Bodypart {} should have id {}", part.id, id);
                    }
                } else {
                    check_id(context, typename, &part.id);
                }
                self.check_known(typename, id);
            }
            TES3Object::Cell(_) => {}
            TES3Object::Dialogue(_) => {
                self.check_known(typename, id);
            }
            TES3Object::Faction(faction) => {
                if context.mode == Mode::TD && VANILLA_FACTIONS.contains(&id.as_str()) {
                    return;
                }
                check_id(context, typename, &faction.id);
                self.check_known(typename, id);
            }
            TES3Object::Info(_) => {}
            TES3Object::PathGrid(_) => {}
            TES3Object::Region(_) => {
                self.check_known(typename, id);
            }
            TES3Object::SoundGen(_) => {}
            TES3Object::StartScript(_) => {}
            _ => {
                check_id(context, typename, id);
                self.check_known(typename, id);
            }
        }
    }
}

impl IdValidator {
    pub fn new() -> Self {
        return IdValidator {
            known: HashMap::new(),
        };
    }

    fn check_known(&mut self, typename: &'static str, id: &String) {
        if let Some(prev) = self.known.insert(id.to_ascii_lowercase(), typename) {
            println!(
                "{} {} shares its ID with a record of type {}",
                typename, id, prev
            );
        }
    }
}
