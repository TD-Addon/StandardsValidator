use std::collections::HashMap;

use tes3::esp::{ArmorType, ClothingType, EditorId, FixedString, NpcFlags, TES3Object, TypeInfo};

use crate::context::Context;

use super::ExtendedHandler;

#[derive(Clone, PartialEq)]
enum EquipmentType {
    Head,
    Chest,
    Pauldron,
    Legs,
    Feet,
    Hand,
    Wrist,
    Shield,
    Robe,
    Skirt,
    Invisible,
}

#[derive(Clone, PartialEq)]
enum Rule {
    None,
    MaleImga,
    FemaleImga,
    Tsaesci,
    Naga,
}

pub struct EquipmentValidator {
    items: HashMap<String, EquipmentType>,
    rule: Rule,
}

fn armor_to_equipment(armor: ArmorType) -> EquipmentType {
    match armor {
        ArmorType::Helmet => EquipmentType::Head,
        ArmorType::Cuirass => EquipmentType::Chest,
        ArmorType::LeftPauldron => EquipmentType::Pauldron,
        ArmorType::RightPauldron => EquipmentType::Pauldron,
        ArmorType::Greaves => EquipmentType::Legs,
        ArmorType::Boots => EquipmentType::Feet,
        ArmorType::LeftGauntlet => EquipmentType::Hand,
        ArmorType::RightGauntlet => EquipmentType::Hand,
        ArmorType::Shield => EquipmentType::Shield,
        ArmorType::LeftBracer => EquipmentType::Wrist,
        ArmorType::RightBracer => EquipmentType::Wrist,
    }
}

fn clothing_to_equipment(clothing: ClothingType) -> EquipmentType {
    match clothing {
        ClothingType::Amulet => EquipmentType::Invisible,
        ClothingType::Belt => EquipmentType::Invisible,
        ClothingType::LeftGlove => EquipmentType::Hand,
        ClothingType::Pants => EquipmentType::Legs,
        ClothingType::RightGlove => EquipmentType::Hand,
        ClothingType::Ring => EquipmentType::Invisible,
        ClothingType::Robe => EquipmentType::Robe,
        ClothingType::Shirt => EquipmentType::Chest,
        ClothingType::Shoes => EquipmentType::Feet,
        ClothingType::Skirt => EquipmentType::Skirt,
    }
}

fn can_equip(slot: EquipmentType, rule: Rule) -> bool {
    match slot {
        EquipmentType::Feet => {
            rule != Rule::MaleImga && rule != Rule::FemaleImga && rule != Rule::Tsaesci
        }
        EquipmentType::Head => rule != Rule::MaleImga && rule != Rule::Naga,
        EquipmentType::Legs => rule != Rule::Tsaesci,
        _ => true,
    }
}

impl ExtendedHandler for EquipmentValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object, _: &str, last: bool) {
        self.rule = Rule::None;
        match record {
            TES3Object::Armor(r) => {
                self.items.insert(
                    record.editor_id_ascii_lowercase().to_string(),
                    armor_to_equipment(r.data.armor_type),
                );
            }
            TES3Object::Clothing(r) => {
                self.items.insert(
                    record.editor_id_ascii_lowercase().to_string(),
                    clothing_to_equipment(r.data.clothing_type),
                );
            }
            TES3Object::Npc(r) => {
                if last {
                    if r.race.eq_ignore_ascii_case("T_Val_Imga") {
                        if r.npc_flags.contains(NpcFlags::FEMALE) {
                            self.rule = Rule::FemaleImga;
                        } else {
                            self.rule = Rule::MaleImga;
                        }
                    } else if r.race.eq_ignore_ascii_case("T_Aka_Tsaesci") {
                        self.rule = Rule::Tsaesci;
                    } else if r.race.eq_ignore_ascii_case("T_Arg_Naga") {
                        self.rule = Rule::Naga;
                    }
                }
            }
            _ => {}
        }
    }

    fn on_inventory(
        &mut self,
        _: &Context,
        record: &TES3Object,
        entry: &(i32, FixedString<32>),
        _: &str,
    ) {
        if self.rule != Rule::None {
            if let Some(slot) = self.items.get(entry.1.as_str()) {
                if !can_equip(slot.clone(), self.rule.clone()) {
                    println!(
                        "{} {} has equipment {} they cannot wear",
                        record.type_name(),
                        record.editor_id(),
                        entry.1.as_str()
                    );
                }
            }
        }
    }
}

impl EquipmentValidator {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            rule: Rule::None,
        }
    }
}
