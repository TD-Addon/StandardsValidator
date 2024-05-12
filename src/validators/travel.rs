use std::collections::{HashMap, HashSet};

use super::Context;
use crate::{
    handlers::Handler,
    util::{get_cell_grid, get_cell_name, is_dead, Actor},
};
use tes3::esp::{Cell, Dialogue, Info, Reference, TES3Object, TravelDestination};

include!(concat!(env!("OUT_DIR"), "/gen_travel.rs"));

pub struct TravelValidator<'a> {
    cells: HashMap<(i32, i32), &'a Cell>,
    classes: HashSet<&'static str>,
    caravaners: HashMap<String, Caravaner<'a>>,
}

struct Caravaner<'a> {
    record: &'a dyn Actor,
    cells: Vec<Location<'a>>,
    destination: Vec<&'a Info>,
}

fn get_town_name(id: &String) -> &str {
    if let Some((prefix, _)) = id.split_once(",") {
        return prefix;
    }
    return id.as_str();
}

impl<'a> Caravaner<'a> {
    pub fn new(record: &'a dyn Actor) -> Self {
        return Self {
            record,
            cells: Vec::new(),
            destination: Vec::new(),
        };
    }

    fn is_counterpart(&self, location: &Location) -> bool {
        if let Some(destinations) = self.record.get_destinations() {
            return destinations.iter().any(|d| location.matches(d));
        }
        return false;
    }

    fn matches_class(&self, class: &String) -> bool {
        if let Some(id) = self.record.get_class() {
            return id.eq_ignore_ascii_case(class);
        }
        return false;
    }
}

struct Location<'a> {
    cell: &'a Cell,
}

impl Location<'_> {
    fn matches(&self, destination: &TravelDestination) -> bool {
        if self.cell.is_interior() {
            if let Some(id) = &destination.cell {
                return self.cell.id.eq_ignore_ascii_case(id);
            }
        } else {
            let [x, y, _] = destination.translation;
            let grid = get_cell_grid(x.into(), y.into());
            if self.cell.data.grid == grid {
                return true;
            }
            let dx = (grid.0 - self.cell.data.grid.0).abs();
            let dy = (grid.1 - self.cell.data.grid.1).abs();
            return dx <= 1 && dy <= 1;
        }
        return false;
    }
}

impl<'a> Handler<'a> for TravelValidator<'a> {
    fn on_record(&mut self, _: &Context, record: &'a TES3Object, _: &str, _: &String) {
        if is_dead(record) {
            return;
        }
        match record {
            TES3Object::Cell(cell) => {
                if cell.is_exterior() {
                    self.cells.insert(cell.data.grid, cell);
                }
            }
            TES3Object::Creature(creature) => {
                if let Some(destinations) = creature.get_destinations() {
                    if !destinations.is_empty() {
                        self.caravaners
                            .insert(creature.id.to_ascii_lowercase(), Caravaner::new(creature));
                    }
                }
            }
            TES3Object::Npc(npc) => {
                if let Some(destinations) = npc.get_destinations() {
                    if !destinations.is_empty() {
                        self.caravaners
                            .insert(npc.id.to_ascii_lowercase(), Caravaner::new(npc));
                        return;
                    }
                }
                if let Some(class) = &npc.class {
                    if self.classes.contains(&class.to_ascii_lowercase().as_str()) {
                        println!(
                            "Npc {} has class {} but does not offer travel services",
                            npc.id, class
                        );
                    }
                }
            }
            _ => {}
        }
    }

    fn on_cellref(
        &mut self,
        _: &Context,
        cell: &'a Cell,
        _: &Reference,
        id: &String,
        _: &Vec<&Reference>,
        _: usize,
    ) {
        if let Some(caravaner) = self.caravaners.get_mut(id) {
            caravaner.cells.push(Location { cell })
        }
    }

    fn on_info(&mut self, _: &Context, record: &'a Info, topic: &Dialogue) {
        if let Some(id) = &record.speaker_id {
            if topic.id.eq_ignore_ascii_case("destination") {
                if let Some(caravaner) = self.caravaners.get_mut(&id.to_ascii_lowercase()) {
                    caravaner.destination.push(record);
                }
            }
        }
    }

    fn on_end(&mut self, _: &Context) {
        for (_, caravaner) in &self.caravaners {
            self.check_caravaner(caravaner);
        }
    }
}

impl TravelValidator<'_> {
    pub fn new() -> Self {
        return Self {
            cells: HashMap::new(),
            classes: get_travel_classes(),
            caravaners: HashMap::new(),
        };
    }

    fn check_caravaner(&self, caravaner: &Caravaner) {
        let typename = caravaner.record.get_type();
        let id = caravaner.record.get_id();
        if caravaner.destination.is_empty() {
            println!(
                "{} {} offers travel services but does not have a reply to the destination topic",
                typename, id
            );
        }
        for cell in &caravaner.cells {
            let counterparts: Vec<&Caravaner> = self
                .caravaners
                .values()
                .filter(|c| c.is_counterpart(cell))
                .collect();
            for dest in caravaner.record.get_destinations().as_ref().unwrap() {
                let return_services: Vec<&Caravaner> = counterparts
                    .iter()
                    .filter(|c| c.cells.iter().any(|l| l.matches(dest)))
                    .map(|c| *c)
                    .collect();
                let (dest_name, town) = self.get_destination_name(dest);
                if return_services.is_empty() {
                    println!(
                        "{} {} in {} offers travel to {} but there is no return travel there",
                        typename,
                        id,
                        get_cell_name(cell.cell),
                        dest_name
                    )
                } else if let Some(class_id) = caravaner.record.get_class() {
                    if !return_services.iter().any(|c| c.matches_class(class_id)) {
                        println!("{} {} in {} offers {} travel to {} but there is no corresponding return travel there", typename, id, get_cell_name(cell.cell), class_id, dest_name);
                    }
                }
                if !town.is_empty()
                    && !caravaner.destination.is_empty()
                    && !caravaner
                        .destination
                        .iter()
                        .map(|i| &i.text)
                        .flatten()
                        .any(|t| t.contains(town))
                {
                    println!(
                        "{} {} does not mention {} in their destination response",
                        typename, id, town
                    );
                }
            }
        }
    }

    fn get_destination_name<'b>(&'b self, dest: &'b TravelDestination) -> (String, &'b str) {
        if let Some(cell) = &dest.cell {
            return (cell.clone(), get_town_name(cell));
        }
        let [x, y, _] = dest.translation;
        let grid = get_cell_grid(x.into(), y.into());
        if let Some(cell) = self.cells.get(&grid) {
            return (get_cell_name(cell), get_town_name(&cell.id));
        }
        return (format!("[{}, {}]", x, y), "");
    }
}
