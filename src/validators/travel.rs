use std::collections::{HashMap, HashSet};

use super::Context;
use crate::{
    handlers::Handler,
    util::{get_cell_grid, is_dead, Actor},
};
use tes3::esp::{Cell, Dialogue, DialogueInfo, EditorId, Reference, TES3Object, TravelDestination};

include!(concat!(env!("OUT_DIR"), "/gen_travel.rs"));

pub struct TravelValidator<'a> {
    cells: HashMap<(i32, i32), &'a Cell>,
    classes: HashSet<&'static str>,
    caravaners: HashMap<String, Caravaner<'a>>,
}

struct Caravaner<'a> {
    record: &'a dyn Actor,
    cells: Vec<Location<'a>>,
    destination: Vec<&'a DialogueInfo>,
}

fn get_town_name(id: &str) -> &str {
    if let Some((prefix, _)) = id.split_once(",") {
        return prefix;
    }
    id
}

impl<'a> Caravaner<'a> {
    pub fn new(record: &'a dyn Actor) -> Self {
        Self {
            record,
            cells: Vec::new(),
            destination: Vec::new(),
        }
    }

    fn is_counterpart(&self, location: &Location) -> bool {
        self.record
            .get_destinations()
            .iter()
            .any(|destination| location.matches(destination))
    }

    fn matches_class(&self, class: &str) -> bool {
        self.record.get_class().eq_ignore_ascii_case(class)
    }
}

struct Location<'a> {
    cell: &'a Cell,
}

impl Location<'_> {
    fn matches(&self, destination: &TravelDestination) -> bool {
        if self.cell.is_interior() {
            self.cell.name.eq_ignore_ascii_case(&destination.cell)
        } else {
            let [x, y, _] = destination.translation;
            let grid = get_cell_grid(x.into(), y.into());
            if self.cell.data.grid == grid {
                return true;
            }
            let dx = (grid.0 - self.cell.data.grid.0).abs();
            let dy = (grid.1 - self.cell.data.grid.1).abs();
            dx <= 1 && dy <= 1
        }
    }
}

impl<'a> Handler<'a> for TravelValidator<'a> {
    fn on_record(&mut self, _: &Context, record: &'a TES3Object, _: &str, _: &str) {
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
                let destinations = creature.get_destinations();
                if !destinations.is_empty() {
                    self.caravaners
                        .insert(creature.id.to_ascii_lowercase(), Caravaner::new(creature));
                }
            }
            TES3Object::Npc(npc) => {
                let destinations = npc.get_destinations();
                if !destinations.is_empty() {
                    self.caravaners
                        .insert(npc.id.to_ascii_lowercase(), Caravaner::new(npc));
                    return;
                }
                // TODO: no alloc (use 'uncased' library?)
                if self
                    .classes
                    .contains(npc.class.to_ascii_lowercase().as_str())
                {
                    println!(
                        "Npc {} has class {} but does not offer travel services",
                        npc.id, npc.class
                    );
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
        id: &str,
        _: &[&Reference],
        _: usize,
    ) {
        if let Some(caravaner) = self.caravaners.get_mut(id) {
            caravaner.cells.push(Location { cell })
        }
    }

    fn on_info(&mut self, _: &Context, record: &'a DialogueInfo, topic: &Dialogue) {
        if !record.speaker_id.is_empty() && topic.id.eq_ignore_ascii_case("destination") {
            if let Some(caravaner) = self
                .caravaners
                .get_mut(&record.speaker_id.to_ascii_lowercase())
            {
                caravaner.destination.push(record);
            }
        }
    }

    fn on_end(&mut self, _: &Context) {
        for caravaner in self.caravaners.values() {
            self.check_caravaner(caravaner);
        }
    }
}

impl TravelValidator<'_> {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            classes: get_travel_classes(),
            caravaners: HashMap::new(),
        }
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
        for location in &caravaner.cells {
            let counterparts: Vec<&Caravaner> = self
                .caravaners
                .values()
                .filter(|c| c.is_counterpart(location))
                .collect();
            for dest in caravaner.record.get_destinations() {
                let return_services: Vec<&Caravaner> = counterparts
                    .iter()
                    .filter(|c| c.cells.iter().any(|l| l.matches(dest)))
                    .copied()
                    .collect();
                let (dest_name, town) = self.get_destination_name(dest);
                if return_services.is_empty() {
                    println!(
                        "{} {} in {} offers travel to {} but there is no return travel there",
                        typename,
                        id,
                        location.cell.editor_id(),
                        dest_name
                    )
                } else if !caravaner.record.get_class().is_empty() {
                    let class_id = caravaner.record.get_class();
                    if !return_services.iter().any(|c| c.matches_class(class_id)) {
                        println!("{} {} in {} offers {} travel to {} but there is no corresponding return travel there", typename, id, location.cell.editor_id(), class_id, dest_name);
                    }
                }
                if !town.is_empty()
                    && !caravaner.destination.is_empty()
                    && !caravaner
                        .destination
                        .iter()
                        .map(|i| &i.text)
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
        if !dest.cell.is_empty() {
            return (dest.cell.clone(), get_town_name(&dest.cell));
        }
        let [x, y, _] = dest.translation;
        let grid = get_cell_grid(x.into(), y.into());
        if let Some(cell) = self.cells.get(&grid) {
            return (cell.editor_id().into(), get_town_name(&cell.name));
        }
        (format!("[{}, {}]", x, y), "")
    }
}
