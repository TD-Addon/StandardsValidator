use crate::{context::Context, context::Mode, handlers::Handler};
use std::collections::HashSet;
use tes3::esp::{Cell, EditorId, MiscItem, MiscItemFlags, Reference, TES3Object};

pub struct KeyValidator {
    miscs: HashSet<String>,
}

fn is_key(misc: &MiscItem) -> bool {
    misc.data.flags.contains(MiscItemFlags::KEY)
}

impl Handler<'_> for KeyValidator {
    fn on_record(&mut self, context: &Context, record: &TES3Object) {
        if let TES3Object::MiscItem(misc) = record {
            let lower = record.editor_id_ascii_lowercase();
            if context.mode != Mode::TD && !is_key(misc) && lower.contains("key") {
                println!("MiscItem {} is not a key", misc.id)
            }
            self.miscs.insert(lower.into_owned());
        }
    }

    fn on_cellref(
        &mut self,
        _: &Context,
        record: &Cell,
        reference: &Reference,
        _: &str,
        _: &[&Reference],
        _: usize,
    ) {
        if let Some(key) = &reference.key {
            if !self.miscs.contains(&key.to_ascii_lowercase()) {
                println!(
                    "Cell {} uses key {} to open {} which is not defined in this file",
                    record.editor_id(),
                    key,
                    reference.id
                );
            }
        }
    }
}

impl KeyValidator {
    pub fn new() -> Self {
        Self {
            miscs: HashSet::new(),
        }
    }
}
