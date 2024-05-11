use levenshtein::levenshtein;
use rayon::prelude::*;
use tes3::esp::TES3Object;

use super::ExtendedHandler;

const DISTANCE_DIV: f32 = 7.;

pub struct NameValidator {
    names: Vec<(String, String)>,
}

impl ExtendedHandler for NameValidator {
    fn on_record(&mut self, record: &TES3Object, _: &str, _: &String, _: bool) {
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

impl NameValidator {
    pub fn new() -> Self {
        return Self { names: Vec::new() };
    }
}
