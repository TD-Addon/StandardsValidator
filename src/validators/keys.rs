use std::collections::HashSet;

use super::{Context, Mode};
use crate::{handler_traits::Handler, util::get_cell_name};
use tes3::esp::{MiscItem, TES3Object};

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
    fn on_record(&mut self, context: &Context, record: &TES3Object, id: &String) {
        if let TES3Object::MiscItem(misc) = record {
            if context.mode != Mode::TD && !is_key(misc) && id.contains("key") {
                println!("MiscItem {} is not a key", misc.id)
            }
            self.miscs.insert(id.clone());
        }
    }

    fn on_cellref(
        &mut self,
        _: &Context,
        record: &tes3::esp::Cell,
        reference: &tes3::esp::Reference,
        _: &String,
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