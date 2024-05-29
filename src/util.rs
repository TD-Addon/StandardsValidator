use std::{collections::HashMap, hash::Hash};
use tes3::esp::{
    Book, Cell, CellFlags, Creature, Npc, NpcFlags, ObjectFlags, TES3Object, TravelDestination,
    TypeInfo,
};

pub const CELL_SIZE: f64 = 8192.;

pub trait Actor {
    fn is_dead(&self) -> bool;

    fn get_destinations(&self) -> &[TravelDestination];

    fn get_class(&self) -> &str;

    fn get_type(&self) -> &'static str;

    fn get_id(&self) -> &str;
}

impl Actor for Creature {
    fn is_dead(&self) -> bool {
        self.data.health == 0
    }

    fn get_destinations(&self) -> &[TravelDestination] {
        &self.travel_destinations
    }

    fn get_class(&self) -> &str {
        ""
    }

    fn get_type(&self) -> &'static str {
        self.type_name()
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}

impl Actor for Npc {
    fn is_dead(&self) -> bool {
        if let Some(stats) = &self.data.stats {
            return stats.health == 0;
        }
        false
    }

    fn get_destinations(&self) -> &[TravelDestination] {
        &self.travel_destinations
    }

    fn get_class(&self) -> &str {
        &self.class
    }

    fn get_type(&self) -> &'static str {
        self.type_name()
    }

    fn get_id(&self) -> &str {
        &self.id
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
    match record {
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
    }
}

pub fn get_cell_grid(x: f64, y: f64) -> (i32, i32) {
    (
        f64::floor(x / CELL_SIZE) as i32,
        f64::floor(y / CELL_SIZE) as i32,
    )
}

pub fn ci_starts_with(s: &str, prefix: &str) -> bool {
    if s.len() >= prefix.len() {
        return s.as_bytes()[..prefix.len()].eq_ignore_ascii_case(prefix.as_bytes());
    }
    false
}

pub fn ci_ends_with(s: &str, suffix: &str) -> bool {
    if s.len() >= suffix.len() {
        let start = s.len() - suffix.len();
        return s.as_bytes()[start..].eq_ignore_ascii_case(suffix.as_bytes());
    }
    false
}

pub fn is_correct_vampire_head(head: &str, race: &str, female: bool) -> bool {
    let prefix = "b_v_";
    if !ci_starts_with(head, prefix) {
        return false;
    }
    let without_prefix = &head[prefix.len()..];
    if !ci_starts_with(without_prefix, race) {
        return false;
    }
    let without_race = &without_prefix[race.len()..];
    if female {
        return "_f_head_01".eq_ignore_ascii_case(without_race);
    }
    "_m_head_01".eq_ignore_ascii_case(without_race)
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

pub fn is_autocalc(npc: &Npc) -> bool {
    npc.npc_flags.contains(NpcFlags::AUTO_CALCULATE)
}

pub fn cannot_sleep(cell: &Cell) -> bool {
    cell.data.flags.contains(CellFlags::RESTING_IS_ILLEGAL)
}

pub fn is_marker(book: &Book) -> bool {
    let mesh = &book.mesh;
    mesh.eq_ignore_ascii_case("tr\\tr_note_pin.nif")
        || mesh.eq_ignore_ascii_case("tr\\tr_editormarker_npc.nif")
        || mesh.eq_ignore_ascii_case("editormarker.nif")
}
