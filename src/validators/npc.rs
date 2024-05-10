use std::{collections::HashMap, error::Error, rc::Rc};

use super::Context;
use crate::{
    context::Mode,
    handlers::Handler,
    util::{is_autocalc, is_dead, update_or_insert, StringError},
};
use serde::Deserialize;
use serde_json::Value;
use tes3::esp::{BodypartId, FixedString, Npc, TES3Object};

pub struct NpcValidator {
    slave_bracers: i32,
    uniques: HashMap<String, UniqueNpc>,
    heads: HashMap<String, AllRules>,
    hairs: HashMap<String, AllRules>,
}

const FLAG_NPC_FEMALE: u32 = 1;
const BOUNTY_ALARM: i8 = 100;
const HOSTILE: i8 = 70;
const KHAJIIT_ANIMATIONS: [&str; 2] = ["t_els_ohmes-raht", "t_els_suthay"];
const KHAJIIT_F: &str = "epos_kha_upr_anim_f.nif";
const KHAJIIT_M: &str = "epos_kha_upr_anim_m.nif";

fn check_khajiit_animations(npc: &Npc) {
    let requires_animations = npc
        .race
        .iter()
        .any(|r| KHAJIIT_ANIMATIONS.contains(&r.to_ascii_lowercase().as_str()));
    let mesh = npc.mesh.as_ref().map(&String::as_str).unwrap_or("");
    if requires_animations {
        let male = (npc.npc_flags.unwrap_or(0) & FLAG_NPC_FEMALE) == 0;
        let target = if male { KHAJIIT_M } else { KHAJIIT_F };
        if !mesh.eq_ignore_ascii_case(target) {
            println!("Npc {} is not using animation {}", npc.id, target);
        }
    } else if mesh.eq_ignore_ascii_case(KHAJIIT_F) || mesh.eq_ignore_ascii_case(KHAJIIT_M) {
        println!("Npc {} has animation {}", npc.id, mesh);
    }
}

impl Handler<'_> for NpcValidator {
    fn on_record(&mut self, context: &Context, record: &TES3Object, _: &'static str, _: &String) {
        self.slave_bracers = 0;
        if let TES3Object::Npc(npc) = record {
            self.check_bodyparts(npc);
            if context.mode == Mode::PT && is_autocalc(npc) {
                println!("Npc {} has auto calculated stats and spells", npc.id);
            }
            if !is_dead(record) {
                if let Some(ai) = &npc.ai_data {
                    if ai.fight >= HOSTILE && ai.alarm >= BOUNTY_ALARM {
                        println!(
                            "Npc {} reports crimes despite having {} fight",
                            npc.id, ai.fight
                        );
                    }
                    if ai.alarm < BOUNTY_ALARM
                        && npc.class.iter().any(|c| c.eq_ignore_ascii_case("guard"))
                    {
                        println!(
                            "Npc {} does not report crimes despite being a guard",
                            npc.id
                        );
                    }
                }
            }
            check_khajiit_animations(npc);
        }
    }

    fn on_inventory(&mut self, _: &Context, record: &TES3Object, entry: &(i32, FixedString<32>)) {
        if self.slave_bracers > 1 {
            return;
        }
        if let TES3Object::Npc(npc) = record {
            if entry.1.eq_ignore_ascii_case("slave_bracer_left")
                || entry.1.eq_ignore_ascii_case("slave_bracer_right")
            {
                self.slave_bracers += entry.0.abs();
                if self.slave_bracers > 1 {
                    println!("Npc {} has multiple slave bracers", npc.id);
                }
            }
        }
    }
}

enum Rule {
    Array(Vec<Rule>),
    Negation(Box<Rule>),
    Equality(String),
}

impl Rule {
    fn test(&self, value: &String) -> bool {
        match self {
            Rule::Array(rules) => rules.iter().all(|r| r.test(value)),
            Rule::Negation(rule) => !rule.test(value),
            Rule::Equality(v) => v.eq_ignore_ascii_case(value),
        }
    }
}

enum FieldRule {
    Class(Rule),
    Faction(Rule),
    Id(Rule),
}

trait Testable {
    fn test(&self, npc: &Npc) -> bool;
}

impl Testable for FieldRule {
    fn test(&self, npc: &Npc) -> bool {
        match self {
            FieldRule::Class(rule) => npc.class.as_ref().map(|c| rule.test(c)).unwrap_or(false),
            FieldRule::Faction(rule) => npc.faction.as_ref().map(|f| rule.test(f)).unwrap_or(false),
            FieldRule::Id(rule) => rule.test(&npc.id),
        }
    }
}

struct FieldRules {
    rules: Vec<FieldRule>,
}

impl Testable for FieldRules {
    fn test(&self, npc: &Npc) -> bool {
        return !self.rules.iter().any(|r| !r.test(npc));
    }
}
struct AllRules {
    rules: Vec<Rc<dyn Testable>>,
}

impl Testable for AllRules {
    fn test(&self, npc: &Npc) -> bool {
        return !self.rules.iter().any(|r| !r.test(npc));
    }
}

struct SomeRules {
    rules: Vec<FieldRules>,
}

impl Testable for SomeRules {
    fn test(&self, npc: &Npc) -> bool {
        return self.rules.iter().any(|r| r.test(npc));
    }
}

#[derive(Deserialize)]
struct Definition {
    model: String,
    ruleset: Option<String>,
    rules: Option<Vec<HashMap<String, Value>>>,
}

#[derive(Deserialize)]
struct BodyParts {
    rulesets: HashMap<String, Vec<HashMap<String, Value>>>,
    head: Vec<Definition>,
    hair: Vec<Definition>,
}

fn build_rule(value: &Value) -> Option<Rule> {
    return match value {
        Value::Array(rule) => {
            let sub: Vec<Rule> = rule.iter().map(&build_rule).flatten().collect();
            if !sub.is_empty() {
                return Some(Rule::Array(sub));
            }
            None
        }
        Value::Object(rule) => {
            if let Some(v) = rule.get("not") {
                if let Some(sub) = build_rule(v) {
                    return Some(Rule::Negation(Box::new(sub)));
                }
            }
            None
        }
        Value::String(rule) => {
            return Some(Rule::Equality(rule.clone()));
        }
        _ => None,
    };
}

#[derive(Default)]
struct UniqueNpc {
    head: Option<String>,
    hair: Option<String>,
}

struct RulesParser {
    uniques: HashMap<String, UniqueNpc>,
    heads: HashMap<String, AllRules>,
    hairs: HashMap<String, AllRules>,
    rulesets: HashMap<String, Rc<SomeRules>>,
}

impl RulesParser {
    fn new() -> Self {
        return Self {
            uniques: HashMap::new(),
            heads: HashMap::new(),
            hairs: HashMap::new(),
            rulesets: HashMap::new(),
        };
    }

    fn parse_rules(
        &mut self,
        values: &Vec<HashMap<String, Value>>,
        params: Option<(BodypartId, &String)>,
    ) -> Result<SomeRules, StringError> {
        let mut out = SomeRules { rules: Vec::new() };
        for rule in values {
            let mut equality = Vec::new();
            for (key, value) in rule {
                if let Some(built) = build_rule(value) {
                    if key == "class" {
                        equality.push(FieldRule::Class(built));
                    } else if key == "faction" {
                        equality.push(FieldRule::Faction(built));
                    } else if key == "id" {
                        if let Some((part, model)) = params {
                            if let Rule::Equality(id) = &built {
                                update_or_insert(&mut self.uniques, id.to_ascii_lowercase(), |u| {
                                    if part == BodypartId::Hair {
                                        u.hair = Some(model.clone());
                                    } else if part == BodypartId::Head {
                                        u.head = Some(model.clone());
                                    }
                                });
                            }
                        }
                        equality.push(FieldRule::Id(built));
                    } else {
                        return Err(StringError::new(format!("Invalid key {}", key)));
                    }
                } else {
                    return Err(StringError::new(format!("Failed to parse {}", value)));
                }
            }
            if !equality.is_empty() {
                out.rules.push(FieldRules { rules: equality });
            }
        }
        return Ok(out);
    }

    fn parse_part(
        &mut self,
        definitions: &Vec<Definition>,
        part: BodypartId,
    ) -> Result<(), StringError> {
        for definition in definitions {
            let model = definition.model.to_ascii_lowercase();
            let mut predicate = AllRules { rules: Vec::new() };
            if let Some(rules) = &definition.rules {
                predicate
                    .rules
                    .push(Rc::new(self.parse_rules(rules, Some((part, &model)))?))
            }
            if let Some(name) = &definition.ruleset {
                if let Some(ruleset) = self.rulesets.get(name) {
                    predicate.rules.push(ruleset.clone());
                }
            }
            if part == BodypartId::Hair {
                self.hairs.insert(model, predicate);
            } else {
                self.heads.insert(model, predicate);
            }
        }
        Ok(())
    }

    fn parse_rulesets(
        &mut self,
        rulesets: &HashMap<String, Vec<HashMap<String, Value>>>,
    ) -> Result<(), StringError> {
        for (name, json) in rulesets {
            let parsed = self.parse_rules(json, None)?;
            self.rulesets.insert(name.clone(), Rc::new(parsed));
        }
        Ok(())
    }
}

impl NpcValidator {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let data: BodyParts = serde_json::from_str(include_str!("../../data/bodyparts.json"))?;
        let mut parser = RulesParser::new();
        parser.parse_rulesets(&data.rulesets)?;
        parser.parse_part(&data.head, BodypartId::Head)?;
        parser.parse_part(&data.hair, BodypartId::Hair)?;
        return Ok(Self {
            slave_bracers: 0,
            uniques: parser.uniques,
            heads: parser.heads,
            hairs: parser.hairs,
        });
    }

    fn check_bodyparts(&self, npc: &Npc) {
        self.check_part_rules(npc, &npc.hair, &self.hairs, "hair");
        self.check_part_rules(npc, &npc.head, &self.heads, "head");
        if let Some(unique) = self.uniques.get(&npc.id.to_ascii_lowercase()) {
            self.check_part(npc, &npc.hair, &unique.hair, "hair");
            self.check_part(npc, &npc.head, &unique.head, "head");
        }
    }

    fn check_part_rules(
        &self,
        npc: &Npc,
        part: &Option<String>,
        rules: &HashMap<String, AllRules>,
        name: &str,
    ) {
        if let Some(id) = part {
            let bodypart = id.to_lowercase();
            if let Some(rule) = rules.get(&bodypart) {
                if !rule.test(npc) {
                    println!("Npc {} is using {} {}", npc.id, name, id);
                }
            }
        }
    }

    fn check_part(
        &self,
        npc: &Npc,
        actual: &Option<String>,
        expected: &Option<String>,
        name: &str,
    ) {
        if let Some(expid) = expected {
            if let Some(actid) = actual {
                if expid.eq_ignore_ascii_case(&actid) {
                    return;
                }
            }
            println!("Npc {} is not using unique {} {}", npc.id, name, expid);
        }
    }
}
