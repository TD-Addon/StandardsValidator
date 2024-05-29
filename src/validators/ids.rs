use std::collections::HashMap;

use super::Context;
use crate::{context::Mode, handlers::Handler, util::is_correct_vampire_head};
use tes3::esp::{Bodypart, BodypartFlags, BodypartId, EditorId, TES3Object, TypeInfo};

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
    part.data.flags.contains(BodypartFlags::FEMALE)
}

fn is_vampire_head(part: &Bodypart) -> bool {
    part.data.vampire && (part.data.part == BodypartId::Head)
}

fn check_id(context: &Context, record: &TES3Object) {
    let id = record.editor_id();
    let matching = context.projects.iter().find(|p| p.matches(&id));
    match matching {
        Some(project) => {
            if context.mode != Mode::TD && project.prefix == "T_" {
                println!("{} {} has a {} ID", record.type_name(), id, project.name);
            }
        }
        None => {
            println!(
                "{} {} does not match a known ID scheme",
                record.type_name(),
                id
            );
        }
    }
}

pub struct IdValidator {
    known: HashMap<String, &'static str>,
}

impl Handler<'_> for IdValidator {
    fn on_record(&mut self, context: &Context, record: &TES3Object) {
        match record {
            TES3Object::Bodypart(part) => {
                if is_vampire_head(part) {
                    if !is_correct_vampire_head(&part.id, &part.race, is_female(part)) {
                        println!(
                            "Bodypart {} should have id b_v_{}_{}_head_01",
                            part.id,
                            part.race,
                            if is_female(part) { "f" } else { "m" }
                        );
                    }
                } else {
                    check_id(context, record);
                }
                self.check_known(record);
            }
            TES3Object::Cell(_) => {}
            TES3Object::Dialogue(_) => {
                self.check_known(record);
            }
            TES3Object::Faction(_) => {
                let id: &str = &record.editor_id();
                if context.mode != Mode::TD || !VANILLA_FACTIONS.contains(&id) {
                    check_id(context, record);
                    self.check_known(record);
                }
            }
            TES3Object::GameSetting(_) => {
                println!("Found dirty {} {}", record.type_name(), record.editor_id());
            }
            TES3Object::DialogueInfo(_) => {}
            TES3Object::PathGrid(_) => {}
            TES3Object::Region(_) => {
                self.check_known(record);
            }
            TES3Object::SoundGen(_) => {}
            TES3Object::StartScript(_) => {}
            TES3Object::MagicEffect(mgef) => {
                println!("Found dirty {} {:?}", record.type_name(), mgef.effect_id);
            }
            _ => {
                check_id(context, record);
                self.check_known(record);
            }
        }
    }
}

impl IdValidator {
    pub fn new() -> Self {
        Self {
            known: HashMap::new(),
        }
    }

    fn check_known(&mut self, record: &TES3Object) {
        let typename = record.type_name();
        if let Some(prev) = self
            .known
            .insert(record.editor_id().to_ascii_lowercase(), typename)
        {
            println!(
                "{} {} shares its ID with a record of type {}",
                typename,
                record.editor_id(),
                prev
            );
        }
    }
}
