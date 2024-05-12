use std::collections::HashMap;

use levenshtein::levenshtein;
use rayon::prelude::*;
use tes3::esp::{Dialogue, Info, TES3Object};

use super::ExtendedHandler;

const DISTANCE_DIV: f32 = 7.;

pub struct NameValidator {
    names: Vec<(String, String)>,
}

pub struct QuestNameValidator {
    names: HashMap<String, (String, String)>,
}

impl ExtendedHandler for NameValidator {
    fn on_record(&mut self, record: &TES3Object, _: &str, _: &String, _: &str, _: bool) {
        if let TES3Object::Npc(npc) = record {
            if let Some(name) = &npc.name {
                let min_distance = (name.len() as f32 / DISTANCE_DIV).round() as usize;
                if min_distance < 1 {
                    return;
                }
                let lower = name.to_ascii_lowercase();
                let found = self.names.par_iter().find_map_any(|element| {
                    let (other_name, _) = element;
                    if *other_name == *lower {
                        return None;
                    }
                    let distance = levenshtein(&lower, other_name);
                    if distance > min_distance {
                        return None;
                    }
                    return Some((element, distance));
                });
                if let Some(((other_name, id), distance)) = found {
                    println!(
                        "Npc {} ({}) has a name similar to {} ({}) {}",
                        npc.id, name, id, other_name, distance
                    );
                }
                self.names.push((lower, npc.id.clone()));
            }
        }
    }
}

impl ExtendedHandler for QuestNameValidator {
    fn on_info(&mut self, record: &Info, topic: &Dialogue, file: &str, last: bool) {
        if record.quest_name.is_some() {
            if let Some(name) = &record.text {
                if name.is_empty() {
                    return;
                }
                let lower = name.to_ascii_lowercase();
                if let Some((other_id, other_file)) = self.names.get(&lower) {
                    if last && other_file != file {
                        println!(
                            "Found quest {} ({}) in {} and {} ({})",
                            name, topic.id, file, other_file, other_id
                        );
                    }
                } else {
                    self.names
                        .insert(lower, (topic.id.clone(), file.to_string()));
                }
            }
        }
    }
}

impl NameValidator {
    pub fn new() -> Self {
        return Self { names: Vec::new() };
    }
}

impl QuestNameValidator {
    pub fn new() -> Self {
        return Self {
            names: HashMap::new(),
        };
    }
}
