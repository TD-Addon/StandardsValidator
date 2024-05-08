use tes3::esp::{Cell, ObjectFlags, TES3Object};

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
