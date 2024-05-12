use std::{collections::HashMap, hash::Hash};
use tes3::esp::{Cell, Creature, Npc, ObjectFlags, TES3Object, TravelDestination};

pub const CELL_SIZE: f64 = 8192.;
const FLAG_NPC_AUTO_CALC: u32 = 0x10;
const FLAG_CELL_NO_SLEEP: u32 = 4;

pub trait Actor {
    fn is_dead(&self) -> bool;

    fn get_destinations(&self) -> &Option<Vec<TravelDestination>>;

    fn get_class(&self) -> Option<&String>;

    fn get_type(&self) -> &'static str;

    fn get_id(&self) -> &String;
}

impl Actor for Creature {
    fn is_dead(&self) -> bool {
        if let Some(data) = &self.data {
            return data.health == 0;
        }
        return false;
    }

    fn get_destinations(&self) -> &Option<Vec<TravelDestination>> {
        return &self.travel_destinations;
    }

    fn get_class(&self) -> Option<&String> {
        return None;
    }

    fn get_type(&self) -> &'static str {
        return self.type_name();
    }

    fn get_id(&self) -> &String {
        return &self.id;
    }
}

impl Actor for Npc {
    fn is_dead(&self) -> bool {
        if let Some(data) = &self.data {
            if let Some(stats) = &data.stats {
                return stats.health == 0;
            }
        }
        return false;
    }

    fn get_destinations(&self) -> &Option<Vec<TravelDestination>> {
        return &self.travel_destinations;
    }

    fn get_class(&self) -> Option<&String> {
        return self.class.as_ref();
    }

    fn get_type(&self) -> &'static str {
        return self.type_name();
    }

    fn get_id(&self) -> &String {
        return &self.id;
    }
}

pub fn is_dead(record: &TES3Object) -> bool {
    match record {
        TES3Object::Creature(creature) => creature.is_dead(),
        TES3Object::Npc(npc) => npc.is_dead(),
        _ => false,
    }
}

pub fn is_persistent(record: &TES3Object) -> bool {
    return match record {
        TES3Object::Activator(r) => r.flags.contains(ObjectFlags::PERSISTENT),
        TES3Object::Alchemy(r) => r.flags.contains(ObjectFlags::PERSISTENT),
        TES3Object::Apparatus(r) => r.flags.contains(ObjectFlags::PERSISTENT),
        TES3Object::Armor(r) => r.flags.contains(ObjectFlags::PERSISTENT),
        TES3Object::Book(r) => r.flags.contains(ObjectFlags::PERSISTENT),
        TES3Object::Clothing(r) => r.flags.contains(ObjectFlags::PERSISTENT),
        TES3Object::Container(r) => r.flags.contains(ObjectFlags::PERSISTENT),
        TES3Object::Creature(r) => r.flags.contains(ObjectFlags::PERSISTENT),
        TES3Object::Door(r) => r.flags.contains(ObjectFlags::PERSISTENT),
        TES3Object::Ingredient(r) => r.flags.contains(ObjectFlags::PERSISTENT),
        TES3Object::Light(r) => r.flags.contains(ObjectFlags::PERSISTENT),
        TES3Object::Lockpick(r) => r.flags.contains(ObjectFlags::PERSISTENT),
        TES3Object::MiscItem(r) => r.flags.contains(ObjectFlags::PERSISTENT),
        TES3Object::Npc(r) => r.flags.contains(ObjectFlags::PERSISTENT),
        TES3Object::Probe(r) => r.flags.contains(ObjectFlags::PERSISTENT),
        TES3Object::RepairItem(r) => r.flags.contains(ObjectFlags::PERSISTENT),
        TES3Object::Static(r) => r.flags.contains(ObjectFlags::PERSISTENT),
        TES3Object::Weapon(r) => r.flags.contains(ObjectFlags::PERSISTENT),
        _ => false,
    };
}

pub fn get_cell_name(cell: &Cell) -> String {
    if cell.is_interior() {
        return cell.id.clone();
    }
    let mut name = cell.id.as_str();
    if name.is_empty() {
        name = cell.region.as_ref().map_or("", &String::as_str);
    }
    if name.is_empty() {
        return format!("{},{}", cell.data.grid.0, cell.data.grid.1);
    }
    return format!("{} {},{}", name, cell.data.grid.0, cell.data.grid.1);
}

pub fn get_cell_grid(x: f64, y: f64) -> (i32, i32) {
    return (
        f64::floor(x / CELL_SIZE) as i32,
        f64::floor(y / CELL_SIZE) as i32,
    );
}

pub fn ci_starts_with(s: &str, prefix: &str) -> bool {
    if s.len() >= prefix.len() {
        return s.as_bytes()[..prefix.len()].eq_ignore_ascii_case(prefix.as_bytes());
    }
    return false;
}

pub fn update_or_insert<K, V: Default, F>(map: &mut HashMap<K, V>, key: K, f: F)
where
    K: PartialEq,
    K: Eq,
    K: Hash,
    F: FnOnce(&mut V),
{
    if let Some(entry) = map.get_mut(&key) {
        f(entry);
    } else {
        let mut v: V = Default::default();
        f(&mut v);
        map.insert(key, v);
    }
}

pub fn is_empty(option: &Option<String>) -> bool {
    return !option.iter().any(|v| !v.is_empty());
}

pub fn is_autocalc(npc: &Npc) -> bool {
    if let Some(flags) = npc.npc_flags {
        return (flags & FLAG_NPC_AUTO_CALC) != 0;
    }
    return false;
}

pub fn cannot_sleep(cell: &Cell) -> bool {
    return (cell.data.flags & FLAG_CELL_NO_SLEEP) != 0;
}
