use std::collections::{HashMap, HashSet};

use regex::{escape, Regex, RegexBuilder};
use tes3::esp::{Cell, Dialogue, DialogueInfo, EditorId, Reference, TES3Object, TypeInfo};

use crate::{context::Context, util::iter_script};

use super::ExtendedHandler;

const MODELS: [&str; 3] = [
    "td\\td_help_deprec_01.nif",
    "tr\\f\\tr_help_deprec_01.nif",
    "pc\\f\\pc_help_deprec_01.nif",
];

fn is_deprecated(mesh: &str, name: &str) -> (bool, bool) {
    if !mesh.is_empty() {
        if let Some((index, _)) = MODELS
            .iter()
            .enumerate()
            .find(|(_, m)| m.eq_ignore_ascii_case(mesh))
        {
            return (true, index != 0);
        }
    }
    (
        !name.is_empty() && name.to_ascii_lowercase().contains("deprecated"),
        false,
    )
}

fn check_script_line(regex_cache: &mut HashMap<String, Regex>, line: &str, id: &str) -> bool {
    if let Some(regex) = regex_cache.get(id) {
        return regex.is_match(line);
    }
    let escaped = r#"[ ,"]"#.to_owned() + &escape(id) + r#"($|[ ,"])"#;
    if let Ok(regex) = RegexBuilder::new(&escaped).case_insensitive(true).build() {
        let matches = regex.is_match(line);
        regex_cache.insert(id.to_string(), regex);
        return matches;
    }
    false
}

pub struct DeprecationValidator {
    deprecated: HashSet<String>,
    regex_cache: HashMap<String, Regex>,
}

impl ExtendedHandler for DeprecationValidator {
    fn on_record(&mut self, context: &Context, record: &TES3Object, _: &str, last: bool) {
        let (deprecated, wrong_model) = match record {
            TES3Object::Activator(r) => is_deprecated(&r.mesh, &r.name),
            TES3Object::Alchemy(r) => is_deprecated(&r.mesh, &r.name),
            TES3Object::Apparatus(r) => is_deprecated(&r.mesh, &r.name),
            TES3Object::Armor(r) => is_deprecated(&r.mesh, &r.name),
            TES3Object::Bodypart(r) => is_deprecated(&r.mesh, ""),
            TES3Object::Book(r) => is_deprecated(&r.mesh, &r.name),
            TES3Object::Class(r) => is_deprecated("", &r.name),
            TES3Object::Clothing(r) => is_deprecated(&r.mesh, &r.name),
            TES3Object::Container(r) => {
                if last {
                    for (_, id) in &r.inventory {
                        self.check(context, record, id);
                    }
                }
                is_deprecated(&r.mesh, &r.name)
            }
            TES3Object::Creature(r) => {
                if last {
                    for (_, id) in &r.inventory {
                        self.check(context, record, id);
                    }
                }
                is_deprecated(&r.mesh, &r.name)
            }
            TES3Object::Door(r) => is_deprecated(&r.mesh, &r.name),
            TES3Object::Faction(r) => is_deprecated("", &r.name),
            TES3Object::Ingredient(r) => is_deprecated(&r.mesh, &r.name),
            TES3Object::Light(r) => is_deprecated(&r.mesh, &r.name),
            TES3Object::Lockpick(r) => is_deprecated(&r.mesh, &r.name),
            TES3Object::MiscItem(r) => is_deprecated(&r.mesh, &r.name),
            TES3Object::Npc(r) => {
                if last {
                    for (_, id) in &r.inventory {
                        self.check(context, record, id);
                    }
                    self.check(context, record, &r.class);
                    self.check(context, record, &r.faction);
                }
                is_deprecated("", &r.name)
            }
            TES3Object::Probe(r) => is_deprecated(&r.mesh, &r.name),
            TES3Object::RepairItem(r) => is_deprecated(&r.mesh, &r.name),
            TES3Object::Static(r) => is_deprecated(&r.mesh, ""),
            TES3Object::Weapon(r) => is_deprecated(&r.mesh, &r.name),
            TES3Object::LeveledCreature(r) => {
                if last {
                    for (id, _) in &r.creatures {
                        self.check(context, record, id);
                    }
                }
                return;
            }
            TES3Object::LeveledItem(r) => {
                if last {
                    for (id, _) in &r.items {
                        self.check(context, record, id);
                    }
                }
                return;
            }
            TES3Object::Script(r) => {
                if last {
                    if let Some(id) = self.check_script(context, &r.text) {
                        println!(
                            "{} {} references {}",
                            record.type_name(),
                            record.editor_id(),
                            id
                        );
                    }
                }
                return;
            }
            _ => (false, false),
        };
        if wrong_model {
            println!(
                "{} {} is not using model {}",
                record.type_name(),
                record.editor_id(),
                MODELS[0]
            );
        }
        if deprecated {
            self.deprecated
                .insert(record.editor_id_ascii_lowercase().into_owned());
        }
    }

    fn on_cellref(&mut self, context: &Context, record: &Cell, reference: &Reference, id: &str) {
        if self.is_deprecated(context, id) {
            println!(
                "{} {} references {}",
                record.type_name(),
                record.editor_id(),
                reference.id
            );
        }
        if let Some(owner) = &reference.owner {
            self.check(context, record, owner);
        }
        if let Some(faction) = &reference.owner_faction {
            self.check(context, record, faction);
        }
        if let Some(soul) = &reference.soul {
            self.check(context, record, soul);
        }
    }

    fn on_info(
        &mut self,
        context: &Context,
        record: &DialogueInfo,
        topic: &Dialogue,
        _: &str,
        last: bool,
    ) {
        if !last {
            return;
        }
        self.check_i(context, record, topic, &record.speaker_id);
        self.check_i(context, record, topic, &record.speaker_class);
        self.check_i(context, record, topic, &record.speaker_faction);
        for filter in &record.filters {
            self.check_i(context, record, topic, &filter.id);
        }
        if let Some(id) = self.check_script(context, &record.script_text) {
            println!(
                "{} {} in topic {} references {}",
                record.type_name(),
                record.id,
                topic.id,
                id
            );
        }
    }
}

impl DeprecationValidator {
    pub fn new() -> Self {
        Self {
            deprecated: HashSet::new(),
            regex_cache: HashMap::new(),
        }
    }

    fn is_deprecated(&self, context: &Context, lower: &str) -> bool {
        self.deprecated.contains(lower) || context.deprecated.contains(lower)
    }

    fn check_i(&self, context: &Context, record: &DialogueInfo, topic: &Dialogue, id: &str) {
        let lower = id.to_ascii_lowercase();
        if self.is_deprecated(context, &lower) {
            println!(
                "{} {} in topic {} references {}",
                record.type_name(),
                record.id,
                topic.id,
                id
            );
        }
    }

    fn check<T>(&self, context: &Context, record: &T, id: &str)
    where
        T: EditorId + TypeInfo,
    {
        if id.is_empty() {
            return;
        }
        let lower = id.to_ascii_lowercase();
        if self.is_deprecated(context, &lower) {
            println!(
                "{} {} references {}",
                record.type_name(),
                record.editor_id(),
                id
            );
        }
    }

    fn check_script<'a>(&'a mut self, context: &'a Context, script_text: &str) -> Option<&'a str> {
        for (line, _) in iter_script(script_text) {
            if line.is_empty() {
                continue;
            }
            for id in &context.deprecated {
                if check_script_line(&mut self.regex_cache, line, id) {
                    return Some(id);
                }
            }
            for id in &self.deprecated {
                if check_script_line(&mut self.regex_cache, line, id) {
                    return Some(id);
                }
            }
        }
        None
    }
}
