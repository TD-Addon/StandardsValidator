use std::collections::{HashMap, HashSet};

use super::Context;
use crate::{
    context::Mode,
    handlers::Handler,
    util::{
        ci_ends_with, ci_starts_with, is_correct_vampire_head, is_khajiit, is_marker, Actor,
        NPC_MARKER,
    },
};
use codegen::{get_joined_commands, get_khajiit_script};
use regex::{Regex, RegexBuilder};
use tes3::esp::{Cell, Dialogue, Npc, NpcFlags, Reference, Script, TES3Object};

enum PositionMarkerType {
    Unknown,
    Book,
    Marker,
    NpcMarker,
}

pub struct ScriptValidator {
    unique_heads: HashSet<&'static str>,
    scripts: HashMap<String, ScriptInfo>,
    npc: Regex,
    khajiit: Regex,
    nolore: Regex,
    vampire: Regex,
    commands: Regex,
    khajiit_script: Regex,
    projects: Vec<(&'static str, Regex)>,
    set_khajiit_neg1: Regex,
    set_khajiit_var: Regex,
    position: Regex,
    markers: HashMap<String, (String, PositionMarkerType, bool, i32)>,
    aifollow: Regex,
    needs_marker: Regex,
    marker_id: Regex,
    mod_reputation: Regex,
    mod_facrep: Regex,
}

struct ScriptInfo {
    used: bool,
    used_by_khajiit: bool,
    npc: bool,
    khajiit: bool,
    nolore: bool,
    vampire: bool,
    projects: Vec<&'static str>,
}

impl ScriptInfo {
    fn new(npc: bool, khajiit: bool, nolore: bool, vampire: bool) -> Self {
        Self {
            used: false,
            used_by_khajiit: false,
            npc,
            khajiit,
            nolore,
            vampire,
            projects: Vec::new(),
        }
    }
}

impl Handler<'_> for ScriptValidator {
    fn on_record(&mut self, context: &Context, record: &TES3Object) {
        if context.mode == Mode::Vanilla {
            return;
        }
        if let TES3Object::Script(script) = record {
            let text = &script.text;
            let mut info = ScriptInfo::new(
                self.npc.is_match(text),
                self.khajiit.is_match(text),
                self.nolore.is_match(text),
                self.vampire.is_match(text),
            );
            for (local, regex) in &self.projects {
                if regex.is_match(text) {
                    info.projects.push(local);
                }
            }
            if info.khajiit && !self.has_correct_khajiit_check(script, text) {
                println!("Script {} contains non-standard khajiit check", script.id);
            }
            self.scripts.insert(script.id.to_ascii_lowercase(), info);
            if let Some(captures) = self.commands.captures(text) {
                println!(
                    "Script {} contains line {}",
                    script.id,
                    captures.get(0).unwrap().as_str()
                );
            }
        } else if let TES3Object::Npc(npc) = record {
            if !npc.is_dead() {
                if npc.script.is_empty() {
                    println!("Npc {} does not have a script", npc.id);
                } else {
                    self.check_npc_script(npc);
                }
            }
        } else if let TES3Object::Book(book) = record {
            let id = book.id.to_ascii_lowercase();
            let marker = if book.mesh.eq_ignore_ascii_case(NPC_MARKER) {
                PositionMarkerType::NpcMarker
            } else if is_marker(book) {
                PositionMarkerType::Marker
            } else {
                PositionMarkerType::Book
            };
            if let Some((_, marker_type, _, _)) = self.markers.get_mut(&id) {
                *marker_type = marker;
            } else if let Some(found) = self.marker_id.find(&book.id) {
                if found.len() == book.id.len() {
                    self.markers.insert(id, (String::new(), marker, false, 0));
                }
            }
        }
    }

    fn on_scriptline(
        &mut self,
        context: &Context,
        record: &TES3Object,
        code: &str,
        comment: &str,
        topic: &Dialogue,
    ) {
        if !code.is_empty() && self.position.is_match(code) {
            if let TES3Object::DialogueInfo(info) = record {
                println!(
                    "Info {} in topic {} uses Position instead of PositionCell",
                    info.id, topic.id
                );
            } else if let TES3Object::Script(script) = record {
                println!("Script {} uses Position instead of PositionCell", script.id);
            }
        }
        if context.mode != Mode::Vanilla
            && self.needs_marker.is_match(code)
            && !self.aifollow.is_match(code)
        {
            if comment.is_empty() {
                if let TES3Object::DialogueInfo(info) = record {
                    println!(
                        "Info {} in topic {} lacks a comment for {}",
                        info.id, topic.id, code
                    );
                } else if let TES3Object::Script(script) = record {
                    println!("Script {} lacks a comment for {}", script.id, code);
                }
            } else if let Some(capture) = self.marker_id.captures(comment) {
                if let Some(group) = capture.get(2) {
                    let description = if let TES3Object::DialogueInfo(info) = record {
                        format!("Info {} in topic {}", info.id, topic.id)
                    } else if let TES3Object::Script(script) = record {
                        format!("Script {}", script.id)
                    } else {
                        String::new()
                    };
                    let id = group.as_str().to_ascii_lowercase();
                    if let Some((desc, _, used, _)) = self.markers.get_mut(&id) {
                        *desc = description;
                        *used = true;
                    } else {
                        self.markers
                            .insert(id, (description, PositionMarkerType::Unknown, false, 0));
                    }
                }
            }
        }
        if self.mod_reputation.is_match(code) {
            if let TES3Object::DialogueInfo(info) = record {
                println!(
                    "Info {} in topic {} uses ModReputation without an explicit target",
                    info.id, topic.id
                );
            } else if let TES3Object::Script(script) = record {
                println!(
                    "Script {} uses ModReputation without an explicit target",
                    script.id
                );
            }
        }
        if let Some(captures) = self.mod_facrep.captures(code) {
            if captures.get(3).is_none() {
                if let TES3Object::DialogueInfo(info) = record {
                    println!(
                        "Info {} in topic {} uses ModPCFacRep without specifying a faction",
                        info.id, topic.id
                    );
                } else if let TES3Object::Script(script) = record {
                    println!(
                        "Script {} uses ModReputation without specifying a faction",
                        script.id
                    );
                }
            }
            let mut garbage = code.contains(',');
            if !garbage {
                if let Some(capture) = captures.get(1) {
                    garbage = capture.as_str().contains('"');
                }
            }
            if garbage {
                if let TES3Object::DialogueInfo(info) = record {
                    println!(
                        "Info {} in topic {} contains superfluous characters in a ModPCFacRep call",
                        info.id, topic.id
                    );
                } else if let TES3Object::Script(script) = record {
                    println!(
                        "Script {} contains superfluous characters in a ModPCFacRep call",
                        script.id
                    );
                }
            }
        }
    }

    fn on_cellref(
        &mut self,
        _: &Context,
        _: &'_ Cell,
        _: &Reference,
        id: &str,
        _: &[&Reference],
        _: usize,
    ) {
        if let Some((_, _, _, count)) = self.markers.get_mut(id) {
            *count += 1;
        }
    }

    fn on_end(&mut self, context: &Context) {
        if context.mode != Mode::TD {
            for (id, script) in &self.scripts {
                if script.used && script.khajiit && !script.used_by_khajiit {
                    println!(
                        "Script {} defines T_Local_Khajiit but is not used by any khajiit",
                        id
                    );
                }
            }
        }
        for (id, (description, is_book, used, count)) in &self.markers {
            if !used {
                continue;
            }
            match *is_book {
                PositionMarkerType::Unknown => {
                    println!(
                        "{} refers to marker {} which is not a book",
                        description, id
                    );
                }
                PositionMarkerType::Book => {
                    println!(
                        "{} refers to book {} which is not a marker",
                        description, id
                    );
                }
                PositionMarkerType::Marker => {
                    println!(
                        "{} refers to book {} which is not an NPC marker",
                        description, id
                    );
                }
                _ => {}
            }
            if *count == 0 {
                println!(
                    "{} refers to marker {} which has no references",
                    description, id
                );
            }
        }
    }
}

fn get_variable(name: &str, types: &str) -> Result<Regex, regex::Error> {
    RegexBuilder::new(&format!(
        "\n[\\s,]*{}[\\s,]+({})[\\s,]*(;*.?)\n",
        types, name
    ))
    .case_insensitive(true)
    .build()
}

impl ScriptValidator {
    pub fn new(
        context: &Context,
        unique_heads: HashSet<&'static str>,
    ) -> Result<Self, regex::Error> {
        let npc = get_variable("T_Local_NPC", "short")?;
        let khajiit = get_variable("T_Local_Khajiit", "short")?;
        let nolore = get_variable("NoLore", "short")?;
        let vampire = get_variable("T_Local_Vampire", "short")?;
        let commands = get_variable(get_joined_commands!(), "(short|long|float)")?;
        let khajiit_script = RegexBuilder::new(get_khajiit_script!())
            .case_insensitive(true)
            .build()?;
        let mut projects = Vec::new();
        for project in &context.projects {
            if let Some(local) = &project.local {
                projects.push((*local, get_variable(local, "short")?));
            }
        }
        let set_khajiit_neg1 =
            RegexBuilder::new(r"\n\s*set\s+T_Local_Khajiit\s+to\s+-1\s*(;.*)?\n")
                .case_insensitive(true)
                .build()?;
        let set_khajiit_var =
            RegexBuilder::new(r"\n\s*set\s+T_Local_Khajiit\s+to\s+([0-9-]+)\s*(;.*)?\n")
                .case_insensitive(true)
                .build()?;
        let position = Regex::new(r"^([,\s]*|.*?->[,\s]*)position[,\s]+")?;
        let needs_marker = Regex::new(
            r"^([,\s]*|.*?->[,\s]*)(position|aitravel|aiescort|placeitem|aifollow)(cell)?[,\s]",
        )?;
        let aifollow = Regex::new(
            r#"aifollow(cell[,\s]+("[^"]+"|[^,\s]+))?[,\s]+("[^"]+"|[^,\s]+)[,\s]+[0-9]+([,\s]+[0.]+){3,}"#,
        )?;
        let marker_id_pattern = r"(^|[,\s])((".to_string()
            + &context
                .projects
                .iter()
                .map(|project| project.prefix)
                .collect::<Vec<_>>()
                .join("|")
            + r#")[a-z0-9-_']*_mark_[a-z0-9-_']*)"#;
        let marker_id = RegexBuilder::new(&marker_id_pattern)
            .case_insensitive(true)
            .build()?;
        let mod_reputation = Regex::new(r"^[,\s]*modreputation[,\s]")?;
        let mod_facrep = Regex::new(r#"modpcfacrep[,\s]+([0-9"-]+)([,\s]+([^,\s]+))?[,\s]*$"#)?;
        Ok(Self {
            unique_heads,
            scripts: HashMap::new(),
            npc,
            khajiit,
            nolore,
            vampire,
            commands,
            khajiit_script,
            projects,
            set_khajiit_neg1,
            set_khajiit_var,
            position,
            needs_marker,
            aifollow,
            marker_id,
            markers: HashMap::new(),
            mod_reputation,
            mod_facrep,
        })
    }

    fn check_npc_script(&mut self, npc: &Npc) {
        let vampire;
        if let Some(script) = self.scripts.get_mut(&npc.script.to_ascii_lowercase()) {
            script.used = true;
            vampire = script.vampire;
            if !script.npc {
                println!(
                    "Npc {} uses script {} which does not define T_Local_NPC",
                    npc.id, npc.script
                );
            }
            if !script.nolore {
                println!(
                    "Npc {} uses script {} which does not define NoLore",
                    npc.id, npc.script
                );
            }
            if is_khajiit(&npc.race) {
                script.used_by_khajiit = true;
                if !script.khajiit {
                    println!(
                        "Npc {} uses script {} which does not define T_Local_Khajiit",
                        npc.id, npc.script
                    );
                }
            }
            if script.projects.is_empty() {
                println!("Npc {} uses script {} which does not define any province specific local variables", npc.id, npc.script);
            } else if script.projects.len() > 1 {
                println!(
                    "Npc {} uses script {} which defines {}",
                    npc.id,
                    npc.script,
                    script
                        .projects
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                );
            }
        } else if ci_starts_with(&npc.script, "t_scvamp_") && ci_ends_with(&npc.script, "_npc") {
            vampire = true;
        } else if !ci_starts_with(&npc.script, "t_scnpc_") {
            println!("Npc {} uses unknown script {}", npc.id, npc.script);
            return;
        } else {
            vampire = npc.script.contains("Vamp");
        }
        if vampire {
            let has_vampire_head = is_correct_vampire_head(
                &npc.head,
                &npc.race,
                npc.npc_flags.contains(NpcFlags::FEMALE),
            );
            let is_sneaky = npc.faction.eq_ignore_ascii_case("T_Cyr_VampirumOrder")
                || ci_starts_with(&npc.script, "T_ScNpc_Cyr_");
            if !has_vampire_head
                && !is_sneaky
                && !self
                    .unique_heads
                    .contains(npc.id.to_ascii_lowercase().as_str())
            {
                println!("Npc {} is a vampire but uses head {}", npc.id, npc.head);
            }
        }
    }

    fn has_correct_khajiit_check(&self, record: &Script, text: &str) -> bool {
        if self.set_khajiit_neg1.is_match(text) {
            return self.khajiit_script.is_match(text);
        }
        let mut found = false;
        for captures in self.set_khajiit_var.captures_iter(text) {
            if found {
                println!("Script {} sets T_Local_Khajiit multiple times", record.id);
                return false;
            }
            found = true;
            if captures.get(1).unwrap().as_str() != "1" {
                println!(
                    "Script {} contains unexpected line {}",
                    record.id,
                    captures.get(0).unwrap().as_str()
                );
                return false;
            }
        }
        found
    }
}
