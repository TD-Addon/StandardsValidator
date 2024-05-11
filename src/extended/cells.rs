use std::collections::HashSet;

use clap::ArgMatches;
use tes3::esp::TES3Object;

use crate::util::{cannot_sleep, get_cell_name, Actor};

use super::ExtendedHandler;

pub struct CellValidator {
    inhabitants: HashSet<String>,
    pathgrids: HashSet<String>,
    cells: Vec<(String, String)>,
    min_inhabitants: usize,
}

impl ExtendedHandler for CellValidator {
    fn on_record(&mut self, record: &TES3Object, _: &str, id: &String, _: &str, last: bool) {
        match record {
            TES3Object::PathGrid(pathgrid) => {
                if let Some(cell) = &pathgrid.cell {
                    self.pathgrids.insert(cell.to_ascii_lowercase());
                }
            }
            TES3Object::Cell(cell) => {
                if last
                    && cell.is_interior()
                    && cell.references.len() > 1
                    && !cell.id.starts_with("T_")
                {
                    self.cells
                        .push((id.to_ascii_lowercase(), get_cell_name(cell)));
                    if !cannot_sleep(cell) {
                        let count = cell
                            .references
                            .iter()
                            .filter(|(_, r)| self.inhabitants.contains(&r.id.to_ascii_lowercase()))
                            .count();
                        if count < self.min_inhabitants {
                            println!(
                                "Cell {} contains {} NPCs or creatures",
                                get_cell_name(cell),
                                count
                            );
                        }
                    }
                }
            }
            TES3Object::LeveledCreature(_) => {
                self.inhabitants.insert(id.to_ascii_lowercase());
            }
            TES3Object::Creature(c) => {
                if !c.is_dead() {
                    self.inhabitants.insert(id.to_ascii_lowercase());
                }
            }
            TES3Object::Npc(n) => {
                if !n.is_dead() {
                    self.inhabitants.insert(id.to_ascii_lowercase());
                }
            }
            _ => {}
        }
    }

    fn on_end(&mut self) {
        for (id, name) in &self.cells {
            if !self.pathgrids.contains(id) {
                println!("Cell {} is missing a path grid", name);
            }
        }
    }
}

impl CellValidator {
    pub fn new(args: &ArgMatches) -> Self {
        let min_inhabitants = *args.get_one::<usize>("mininhabitants").unwrap();
        return Self {
            inhabitants: HashSet::new(),
            pathgrids: HashSet::new(),
            cells: Vec::new(),
            min_inhabitants,
        };
    }
}
