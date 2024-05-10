use std::collections::HashSet;

use super::Context;
use crate::handlers::Handler;
use tes3::esp::TES3Object;

pub struct SoundGenValidator {
    sound_gens: HashSet<String>,
    to_check: Vec<String>,
}

impl Handler<'_> for SoundGenValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object, _: &'static str, _: &String) {
        if let TES3Object::Creature(creature) = record {
            if creature.sound.is_none() {
                self.to_check.push(creature.id.to_ascii_lowercase());
            }
        } else if let TES3Object::SoundGen(soundgen) = record {
            if let Some(id) = &soundgen.creature {
                self.sound_gens.insert(id.to_ascii_lowercase());
            }
        }
    }

    fn on_end(&mut self, _: &Context) {
        for id in &self.to_check {
            if !self.sound_gens.contains(id) {
                println!("Creature {} is missing a sound gen", id);
            }
        }
    }
}

impl SoundGenValidator {
    pub fn new() -> Self {
        return Self {
            sound_gens: HashSet::new(),
            to_check: Vec::new(),
        };
    }
}
