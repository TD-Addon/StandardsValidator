use core::fmt;
use std::{collections::HashMap, error::Error};
use tes3::esp::{Cell, Npc, ObjectFlags, TES3Object};

pub const CELL_SIZE: i32 = 8192;
const FLAG_NPC_AUTO_CALC: u32 = 0x10;

#[derive(Debug)]
pub struct StringError {
    message: String,
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.message);
    }
}

impl Error for StringError {
    fn description(&self) -> &str {
        return self.message.as_str();
    }
}

impl StringError {
    pub fn new(message: String) -> Self {
        return Self { message };
    }
}

pub fn is_dead(record: &TES3Object) -> bool {
    match record {
        TES3Object::Creature(creature) => {
            if let Some(data) = &creature.data {
                return data.health == 0;
            }
        }
        TES3Object::Npc(npc) => {
            if let Some(data) = &npc.data {
                if let Some(stats) = &data.stats {
                    return stats.health == 0;
                }
            }
        }
        _ => {}
    }
    return false;
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
        f64::floor(x / CELL_SIZE as f64) as i32,
        f64::floor(y / CELL_SIZE as f64) as i32,
    );
}

pub fn ci_starts_with(s: &str, prefix: &str) -> bool {
    if s.len() >= prefix.len() {
        return s.as_bytes()[..prefix.len()].eq_ignore_ascii_case(prefix.as_bytes());
    }
    return false;
}

pub fn update_or_insert<V: Default, F>(map: &mut HashMap<String, V>, key: String, f: F)
where
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
