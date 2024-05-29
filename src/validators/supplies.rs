use std::collections::HashMap;

use super::Context;
use crate::handlers::Handler;
use codegen::get_supplies_data;
use tes3::esp::{Cell, EditorId, Reference};

pub struct SupplyChestValidator {
    chests: HashMap<&'static str, &'static str>,
}

const ALL_RANKS: u32 = 4294967295;

impl Handler<'_> for SupplyChestValidator {
    fn on_cellref(
        &mut self,
        _: &Context,
        record: &Cell,
        reference: &Reference,
        id: &str,
        _: &[&Reference],
        _: usize,
    ) {
        if let Some(faction) = self.chests.get(id) {
            if !reference
                .owner_faction
                .as_ref()
                .map(|f| f.eq_ignore_ascii_case(faction))
                .unwrap_or(false)
            {
                println!(
                    "Cell {} contains {} not owned by the {}",
                    record.editor_id(),
                    reference.id,
                    faction
                );
            } else {
                let rank = reference.owner_faction_rank.unwrap_or(0);
                if rank != 0 && rank != ALL_RANKS {
                    println!(
                        "Cell {} contains {} not available to all ranks",
                        record.editor_id(),
                        reference.id
                    );
                }
            }
        }
    }
}

impl SupplyChestValidator {
    pub fn new() -> Self {
        Self {
            chests: get_supplies_data!(),
        }
    }
}
