use crate::{context::Context, context::Mode, handlers::Handler, util::get_cell_name};
use std::collections::HashSet;
use tes3::esp::{Cell, MiscItem, Reference, TES3Object};

pub struct KeyValidator {
    miscs: HashSet<String>,
}

const FLAG_KEY: u32 = 1;

fn is_key(misc: &MiscItem) -> bool {
    if let Some(data) = &misc.data {
        return (data.flags & FLAG_KEY) != 0;
    }
    return false;
}

impl Handler<'_> for KeyValidator {
    fn on_record(&mut self, context: &Context, record: &TES3Object, _: &'static str, id: &String) {
        if let TES3Object::MiscItem(misc) = record {
            let lower = id.to_ascii_lowercase();
            if context.mode != Mode::TD && !is_key(misc) && lower.contains("key") {
                println!("MiscItem {} is not a key", misc.id)
            }
            self.miscs.insert(lower);
        }
    }

    fn on_cellref(
        &mut self,
        _: &Context,
        record: &Cell,
        reference: &Reference,
        _: &Vec<&Reference>,

        _: usize,
    ) {
        if let Some(key) = &reference.key {
            if !self.miscs.contains(&key.to_ascii_lowercase()) {
                println!(
                    "Cell {} uses key {} to open {} which is not defined in this file",
                    get_cell_name(record),
                    key,
                    reference.id
                );
            }
        }
    }
}

impl KeyValidator {
    pub fn new() -> KeyValidator {
        return KeyValidator {
            miscs: HashSet::new(),
        };
    }
}
