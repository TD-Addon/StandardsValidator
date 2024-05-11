use std::{collections::HashMap, mem::transmute};

use super::Context;
use crate::{
    context::Mode,
    handlers::Handler,
    util::{ci_starts_with, is_dead},
};
use regex::{Regex, RegexBuilder};
use tes3::esp::{Dialogue, Npc, Script, TES3Object};

pub struct ScriptValidator {
    scripts: HashMap<String, ScriptInfo>,
    npc: Regex,
    khajiit: Regex,
    nolore: Regex,
    commands: Regex,
    khajiit_script: Regex,
    projects: Vec<(String, Regex)>,
    set_khajiit_neg1: Regex,
    set_khajiit_var: Regex,
    position: Regex,
}

struct ScriptInfo {
    used: bool,
    used_by_khajiit: bool,
    npc: bool,
    khajiit: bool,
    nolore: bool,
    projects: Vec<&'static String>,
}

impl ScriptInfo {
    fn new(npc: bool, khajiit: bool, nolore: bool) -> Self {
        return Self {
            used: false,
            used_by_khajiit: false,
            npc,
            khajiit,
            nolore,
            projects: Vec::new(),
        };
    }
}

impl Handler<'_> for ScriptValidator {
    fn on_record(&mut self, context: &Context, record: &TES3Object, _: &str, _: &String) {
        if context.mode == Mode::Vanilla {
            return;
        }
        if let TES3Object::Script(script) = record {
            if let Some(text) = &script.script_text {
                let mut info = ScriptInfo::new(
                    self.npc.is_match(text),
                    self.khajiit.is_match(text),
                    self.nolore.is_match(text),
                );
                for (local, regex) in &self.projects {
                    if regex.is_match(text) {
                        unsafe {
                            info.projects
                                .push(transmute::<&String, &'static String>(local));
                        }
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
            }
        } else if let TES3Object::Npc(npc) = record {
            if !is_dead(record) {
                if let Some(script) = &npc.script {
                    if !script.is_empty() {
                        let id = script.to_ascii_lowercase();
                        self.check_npc_script(npc, id);
                        return;
                    }
                }
                println!("Npc {} does not have a script", npc.id);
            }
        }
    }

    fn on_scriptline(
        &mut self,
        _: &Context,
        record: &TES3Object,
        code: &str,
        _: &str,
        topic: &Dialogue,
    ) {
        if !code.is_empty() {
            if self.position.is_match(code) {
                if let TES3Object::Info(info) = record {
                    println!(
                        "Info {} in topic {} uses Position instead of PositionCell",
                        info.id, topic.id
                    );
                } else if let TES3Object::Script(script) = record {
                    println!("Script {} uses Position instead of PositionCell", script.id);
                }
            }
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
    pub fn new(context: &Context) -> Result<Self, regex::Error> {
        let npc = get_variable("T_Local_NPC", "short")?;
        let khajiit = get_variable("T_Local_Khajiit", "short")?;
        let nolore = get_variable("NoLore", "short")?;
        let joined_commands = include_str!("../../data/mwscript.returning.txt")
            .trim()
            .split_whitespace()
            .map(&String::from)
            .collect::<Vec<String>>()
            .join("|");
        let commands = get_variable(&joined_commands, "(short|long|float)")?;
        let mut khajiit_input = include_str!("../../data/khajiit.mwscript")
            .replace("(", r"\(")
            .replace(")", r"\)")
            .replace("\n", r"\s*((;.*)?\n)+\s*");
        khajiit_input = Regex::new(r"\s+")?
            .replace_all(&khajiit_input, r"\s+")
            .into_owned();
        let khajiit_script = RegexBuilder::new(&khajiit_input)
            .case_insensitive(true)
            .build()?;
        let mut projects = Vec::new();
        for project in &context.projects {
            if let Some(local) = &project.local {
                projects.push((local.clone(), get_variable(local, "short")?));
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
        return Ok(Self {
            scripts: HashMap::new(),
            npc,
            khajiit,
            nolore,
            commands,
            khajiit_script,
            projects,
            set_khajiit_neg1,
            set_khajiit_var,
            position,
        });
    }

    fn check_npc_script(&mut self, npc: &Npc, id: String) {
        if let Some(script) = self.scripts.get_mut(&id) {
            script.used = true;
            if !script.npc {
                println!(
                    "Npc {} uses script {} which does not define T_Local_NPC",
                    npc.id, id
                );
            }
            if !script.nolore {
                println!(
                    "Npc {} uses script {} which does not define NoLore",
                    npc.id, id
                );
            }
            if let Some(race) = &npc.race {
                if race.eq_ignore_ascii_case("khajiit") || ci_starts_with(race, "t_els_") {
                    script.used_by_khajiit = true;
                    if !script.khajiit {
                        println!(
                            "Npc {} uses script {} which does not define T_Local_Khajiit",
                            npc.id, id
                        );
                    }
                }
            }
            if script.projects.is_empty() {
                println!("Npc {} uses script {} which does not define any province specific local variables", npc.id, id);
            } else if script.projects.len() > 1 {
                println!(
                    "Npc {} uses script {} which defines {}",
                    npc.id,
                    id,
                    script
                        .projects
                        .iter()
                        .map(|s| (*s).clone())
                        .collect::<Vec<String>>()
                        .join(", ")
                );
            }
        } else if !ci_starts_with(&id, "t_scnpc_") {
            println!("Npc {} uses unknown script {}", npc.id, id);
        }
    }

    fn has_correct_khajiit_check(&self, record: &Script, text: &String) -> bool {
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
        return found;
    }
}
