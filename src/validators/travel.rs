use std::collections::{HashMap, HashSet};

use super::Context;
use crate::{
    handlers::Handler,
    util::{get_cell_grid, get_cell_name, is_dead},
};
use tes3::esp::{Cell, Dialogue, Info, Reference, TES3Object, TravelDestination};

pub struct TravelValidator<'a> {
    cells: HashMap<(i32, i32), &'a Cell>,
    classes: HashSet<String>,
    caravaners: HashMap<String, Caravaner<'a>>,
}

struct Caravaner<'a> {
    record: &'a TES3Object,
    cells: Vec<Location<'a>>,
    destination: Vec<&'a Info>,
}

fn matches_location(option: &Option<Vec<TravelDestination>>, location: &Location) -> bool {
    if let Some(destinations) = &option {
        return destinations.iter().any(|d| location.matches(d));
    }
    return false;
}

fn get_town_name(id: &String) -> &str {
    if let Some((prefix, _)) = id.split_once(",") {
        return prefix;
    }
    return id.as_str();
}

impl<'a> Caravaner<'a> {
    pub fn new(record: &'a TES3Object) -> Self {
        return Self {
            record,
            cells: Vec::new(),
            destination: Vec::new(),
        };
    }

    fn is_counterpart(&self, location: &Location) -> bool {
        if let TES3Object::Creature(r) = self.record {
            return matches_location(&r.travel_destinations, location);
        } else if let TES3Object::Npc(r) = self.record {
            return matches_location(&r.travel_destinations, location);
        }
        return false;
    }

    fn matches_class(&self, class: &String) -> bool {
        if let TES3Object::Creature(r) = self.record {
            return true;
        } else if let TES3Object::Npc(r) = self.record {
            if let Some(id) = &r.class {
                return id.eq_ignore_ascii_case(class);
            }
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
    fn on_record(&mut self, _: &Context, record: &'a TES3Object, _: &'static str, _: &String) {
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
                if let Some(destinations) = &creature.travel_destinations {
                    if !destinations.is_empty() {
                        self.caravaners
                            .insert(creature.id.to_ascii_lowercase(), Caravaner::new(record));
                    }
                }
            }
            TES3Object::Npc(npc) => {
                if let Some(destinations) = &npc.travel_destinations {
                    if !destinations.is_empty() {
                        self.caravaners
                            .insert(npc.id.to_ascii_lowercase(), Caravaner::new(record));
                        return;
                    }
                }
                if let Some(class) = &npc.class {
                    if self.classes.contains(&class.to_ascii_lowercase()) {
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
            if let TES3Object::Creature(r) = caravaner.record {
                self.check_caravaner(
                    caravaner,
                    r.type_name(),
                    &r.id,
                    r.travel_destinations.as_ref().unwrap(),
                    &None,
                );
            } else if let TES3Object::Npc(r) = caravaner.record {
                self.check_caravaner(
                    caravaner,
                    r.type_name(),
                    &r.id,
                    r.travel_destinations.as_ref().unwrap(),
                    &r.class,
                );
            }
        }
    }
}

impl TravelValidator<'_> {
    pub fn new() -> Result<Self, serde_json::Error> {
        let classes = serde_json::from_str(include_str!("../../data/travel.json"))?;
        return Ok(Self {
            cells: HashMap::new(),
            classes,
            caravaners: HashMap::new(),
        });
    }

    fn check_caravaner(
        &self,
        caravaner: &Caravaner,
        typename: &str,
        id: &String,
        destinations: &Vec<TravelDestination>,
        class: &Option<String>,
    ) {
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
            for dest in destinations {
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
                } else if let Some(class_id) = class {
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
