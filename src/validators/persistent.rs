use std::collections::HashMap;

use super::Context;
use crate::{handlers::Handler, util::is_persistent};
use tes3::esp::{Cell, Reference, TES3Object};

pub struct PersistentValidator {
    counts: HashMap<String, u32>,
}

impl Handler<'_> for PersistentValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object, _: &str, id: &str) {
        if is_persistent(record) {
            match record {
                TES3Object::Creature(_) => {}
                TES3Object::Npc(_) => {}
                _ => {
                    self.counts.insert(id.to_ascii_lowercase(), 0);
                }
            }
        }
    }

    fn on_cellref(
        &mut self,
        _: &Context,
        _: &Cell,
        _: &Reference,
        id: &str,
        _: &[&Reference],
        _: usize,
    ) {
        if let Some(count) = self.counts.get_mut(id) {
            *count += 1;
            if *count > 1 {
                println!("Persistent object {} is used multiple times", id);
                self.counts.remove(id);
            }
        }
    }
}

impl PersistentValidator {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }
}
