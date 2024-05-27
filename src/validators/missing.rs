use super::Context;
use crate::{handlers::Handler, util::is_marker};
use tes3::esp::{EditorId, LightFlags, TES3Object, TypeInfo};

pub struct FieldValidator {}

fn check(record: &TES3Object, field: &str, value: &str) {
    if !value.is_empty() && !value.trim().is_empty() {
        if field != "name" && !value.contains('.') {
            println!(
                "{} {} has invalid {} {}",
                record.type_name(),
                record.editor_id(),
                field,
                value
            );
        }
        return;
    }
    println!(
        "{} {} has a missing {}",
        record.type_name(),
        record.editor_id(),
        field
    );
}

impl Handler<'_> for FieldValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object) {
        match record {
            TES3Object::Activator(r) => {
                check(record, "mesh", &r.mesh);
            }
            TES3Object::Alchemy(r) => {
                check(record, "icon", &r.icon);
                check(record, "mesh", &r.mesh);
                check(record, "name", &r.name);
            }
            TES3Object::Apparatus(r) => {
                check(record, "icon", &r.icon);
                check(record, "mesh", &r.mesh);
                check(record, "name", &r.name);
            }
            TES3Object::Armor(r) => {
                check(record, "icon", &r.icon);
                check(record, "mesh", &r.mesh);
                check(record, "name", &r.name);
            }
            TES3Object::Book(r) => {
                if !is_marker(r) {
                    check(record, "icon", &r.icon);
                    check(record, "mesh", &r.mesh);
                    check(record, "name", &r.name);
                }
            }
            TES3Object::Clothing(r) => {
                check(record, "icon", &r.icon);
                check(record, "mesh", &r.mesh);
                check(record, "name", &r.name);
            }
            TES3Object::Container(r) => {
                check(record, "mesh", &r.mesh);
            }
            TES3Object::Creature(r) => {
                check(record, "mesh", &r.mesh);
            }
            TES3Object::Door(r) => {
                check(record, "mesh", &r.mesh);
                check(record, "name", &r.name);
            }
            TES3Object::Ingredient(r) => {
                check(record, "icon", &r.icon);
                check(record, "mesh", &r.mesh);
                check(record, "name", &r.name);
            }
            TES3Object::Light(r) => {
                if r.data.flags.contains(LightFlags::CAN_CARRY) {
                    check(record, "icon", &r.icon);
                    check(record, "mesh", &r.mesh);
                    check(record, "name", &r.name);
                }
            }
            TES3Object::Lockpick(r) => {
                check(record, "icon", &r.icon);
                check(record, "mesh", &r.mesh);
                check(record, "name", &r.name);
            }
            TES3Object::MiscItem(r) => {
                check(record, "icon", &r.icon);
                check(record, "mesh", &r.mesh);
                check(record, "name", &r.name);
            }
            TES3Object::Npc(r) => {
                check(record, "name", &r.name);
            }
            TES3Object::Probe(r) => {
                check(record, "icon", &r.icon);
                check(record, "mesh", &r.mesh);
                check(record, "name", &r.name);
            }
            TES3Object::RepairItem(r) => {
                check(record, "icon", &r.icon);
                check(record, "mesh", &r.mesh);
                check(record, "name", &r.name);
            }
            TES3Object::Static(r) => {
                check(record, "mesh", &r.mesh);
            }
            TES3Object::Weapon(r) => {
                check(record, "icon", &r.icon);
                check(record, "mesh", &r.mesh);
                check(record, "name", &r.name);
            }
            _ => {}
        }
    }
}
