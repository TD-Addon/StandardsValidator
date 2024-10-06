use std::collections::{HashMap, HashSet};

use clap::ArgMatches;
use tes3::esp::{EditorId, TES3Object};

use crate::{
    context::Context,
    util::{cannot_sleep, Actor},
};

use super::ExtendedHandler;

pub struct CellValidator {
    inhabitants: HashSet<String>,
    pathgrids: HashSet<String>,
    cells: Vec<(String, String)>,
    min_inhabitants: usize,
    regions: HashMap<(i32, i32), String>,
    changed: HashSet<(i32, i32)>,
}

fn is_region_change(x: i32, y: i32, changed: &HashSet<(i32, i32)>) -> bool {
    let mut surrounding = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if changed.contains(&(x + dx, y + dy)) {
                surrounding += 1;
                if surrounding > 1 || changed.contains(&(x + dx * 2, y + dy * 2)) {
                    return false;
                }
            }
        }
    }
    true
}

impl ExtendedHandler for CellValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object, _: &str, last: bool) {
        match record {
            TES3Object::PathGrid(pathgrid) => {
                if !pathgrid.cell.is_empty() {
                    self.pathgrids.insert(pathgrid.cell.to_ascii_lowercase());
                }
            }
            TES3Object::Cell(cell) => {
                if last
                    && cell.is_interior()
                    && cell.references.len() > 1
                    && !cell.name.starts_with("T_")
                {
                    self.cells
                        .push((cell.name.to_ascii_lowercase(), cell.editor_id().into()));
                    if !cannot_sleep(cell) {
                        let count = cell
                            .references
                            .iter()
                            .filter(|(_, r)| self.inhabitants.contains(&r.id.to_ascii_lowercase()))
                            .count();
                        if count < self.min_inhabitants {
                            println!(
                                "Cell {} contains {} NPCs or creatures",
                                cell.editor_id(),
                                count
                            );
                        }
                    }
                }
                if cell.is_exterior() {
                    if last {
                        let current = self.regions.get(&cell.data.grid);
                        if let Some(region) = current {
                            if !cell
                                .region
                                .as_ref()
                                .is_some_and(|r| r.eq_ignore_ascii_case(region))
                            {
                                self.changed.insert(cell.data.grid);
                            } else {
                                return;
                            }
                        } else {
                            return;
                        }
                    }
                    if let Some(region) = &cell.region {
                        self.regions.insert(cell.data.grid, region.clone());
                    } else {
                        self.regions.remove(&cell.data.grid);
                    }
                }
            }
            TES3Object::LeveledCreature(_) => {
                self.inhabitants
                    .insert(record.editor_id_ascii_lowercase().into_owned());
            }
            TES3Object::Creature(c) => {
                if !c.is_dead() {
                    self.inhabitants
                        .insert(record.editor_id_ascii_lowercase().into_owned());
                }
            }
            TES3Object::Npc(n) => {
                if !n.is_dead() {
                    self.inhabitants
                        .insert(record.editor_id_ascii_lowercase().into_owned());
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
        for grid in &self.changed {
            if is_region_change(grid.0, grid.1, &self.changed) {
                let region = self.regions.get(grid).map_or("None", String::as_ref);
                println!(
                    "Cell ({}, {}) had its region changed to {}",
                    grid.0, grid.1, region
                );
            }
        }
    }
}

impl CellValidator {
    pub fn new(args: &ArgMatches) -> Self {
        let min_inhabitants = *args.get_one::<usize>("mininhabitants").unwrap();
        Self {
            inhabitants: HashSet::new(),
            pathgrids: HashSet::new(),
            cells: Vec::new(),
            min_inhabitants,
            regions: HashMap::new(),
            changed: HashSet::new(),
        }
    }
}
