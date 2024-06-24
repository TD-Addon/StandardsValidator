use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use super::Context;
use crate::{
    context::Mode,
    handlers::Handler,
    util::{is_autocalc, is_dead, update_or_insert},
};
use codegen::get_bodypart_data;
use tes3::esp::{BodypartId, FixedString, Npc, NpcFlags, TES3Object};

pub struct NpcValidator {
    slave_bracers: i32,
    uniques: HashMap<&'static str, UniqueNpc>,
    heads: HashMap<&'static str, AllRules>,
    hairs: HashMap<&'static str, AllRules>,
}

const BOUNTY_ALARM: i8 = 100;
const HOSTILE: i8 = 70;
const KHAJIIT_ANIMATIONS: [&str; 2] = ["t_els_ohmes-raht", "t_els_suthay"];
const KHAJIIT_F: &str = "epos_kha_upr_anim_f.nif";
const KHAJIIT_M: &str = "epos_kha_upr_anim_m.nif";

fn check_khajiit_animations(npc: &Npc) {
    let requires_animations = KHAJIIT_ANIMATIONS.contains(&npc.race.to_ascii_lowercase().as_str());
    let mesh = &npc.mesh;
    if requires_animations {
        let male = !npc.npc_flags.contains(NpcFlags::FEMALE);
        let target = if male { KHAJIIT_M } else { KHAJIIT_F };
        if !mesh.eq_ignore_ascii_case(target) {
            println!("Npc {} is not using animation {}", npc.id, target);
        }
    } else if mesh.eq_ignore_ascii_case(KHAJIIT_F) || mesh.eq_ignore_ascii_case(KHAJIIT_M) {
        println!("Npc {} has animation {}", npc.id, mesh);
    }
}

impl Handler<'_> for NpcValidator {
    fn on_record(&mut self, context: &Context, record: &TES3Object) {
        self.slave_bracers = 0;
        if let TES3Object::Npc(npc) = record {
            self.check_bodyparts(npc);
            if context.mode == Mode::PT && is_autocalc(npc) {
                println!("Npc {} has auto calculated stats and spells", npc.id);
            }
            if !is_dead(record) {
                let ai = &npc.ai_data;
                if ai.fight >= HOSTILE && ai.alarm >= BOUNTY_ALARM {
                    println!(
                        "Npc {} reports crimes despite having {} fight",
                        npc.id, ai.fight
                    );
                }
                if (ai.alarm < BOUNTY_ALARM) && npc.class.eq_ignore_ascii_case("guard") {
                    println!(
                        "Npc {} does not report crimes despite being a guard",
                        npc.id
                    );
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
    Equality(&'static str),
}

impl Rule {
    fn test(&self, value: &str) -> bool {
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
            FieldRule::Class(rule) => rule.test(&npc.class),
            FieldRule::Faction(rule) => rule.test(&npc.faction),
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

#[derive(Default)]
struct UniqueNpc {
    head: Option<&'static str>,
    hair: Option<&'static str>,
}

struct RulesParser {
    uniques: HashMap<&'static str, UniqueNpc>,
    heads: HashMap<&'static str, AllRules>,
    hairs: HashMap<&'static str, AllRules>,
    rulesets: HashMap<&'static str, Rc<SomeRules>>,
}

impl RulesParser {
    fn new() -> Self {
        Self {
            uniques: HashMap::new(),
            heads: HashMap::new(),
            hairs: HashMap::new(),
            rulesets: HashMap::new(),
        }
    }

    fn parse_rules(
        &mut self,
        values: Vec<Vec<FieldRule>>,
        params: Option<(BodypartId, &'static str)>,
    ) -> Result<SomeRules, String> {
        let mut out = SomeRules { rules: Vec::new() };
        for equality in values {
            for rule in &equality {
                if let FieldRule::Id(id_rule) = rule {
                    if let Some((part, model)) = params {
                        if let Rule::Equality(id) = &id_rule {
                            update_or_insert(&mut self.uniques, id, |u| {
                                if part == BodypartId::Hair {
                                    u.hair = Some(model);
                                } else if part == BodypartId::Head {
                                    u.head = Some(model);
                                }
                            });
                        }
                    }
                    break;
                }
            }
            if !equality.is_empty() {
                out.rules.push(FieldRules { rules: equality });
            }
        }
        Ok(out)
    }

    #[allow(clippy::type_complexity)]
    fn parse_part(
        &mut self,
        definitions: Vec<(
            &'static str,
            Option<&'static str>,
            Option<Vec<Vec<FieldRule>>>,
        )>,
        part: BodypartId,
    ) -> Result<(), String> {
        for (model, ruleset, rules_opt) in definitions {
            let mut predicate = AllRules { rules: Vec::new() };
            if let Some(rules) = rules_opt {
                predicate
                    .rules
                    .push(Rc::new(self.parse_rules(rules, Some((part, model)))?))
            }
            if let Some(name) = ruleset {
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
        rulesets: Vec<(&'static str, Vec<Vec<FieldRule>>)>,
    ) -> Result<(), String> {
        for (name, rules) in rulesets {
            let parsed = self.parse_rules(rules, None)?;
            self.rulesets.insert(name, Rc::new(parsed));
        }
        Ok(())
    }
}

impl NpcValidator {
    pub fn new() -> Result<Self, String> {
        let (rulesets, head, hair) = get_bodypart_data!();
        let mut parser = RulesParser::new();
        parser.parse_rulesets(rulesets)?;
        parser.parse_part(head, BodypartId::Head)?;
        parser.parse_part(hair, BodypartId::Hair)?;
        Ok(Self {
            slave_bracers: 0,
            uniques: parser.uniques,
            heads: parser.heads,
            hairs: parser.hairs,
        })
    }

    fn check_bodyparts(&self, npc: &Npc) {
        self.check_part_rules(npc, &npc.hair, &self.hairs, "hair");
        self.check_part_rules(npc, &npc.head, &self.heads, "head");
        if let Some(unique) = self.uniques.get(npc.id.to_ascii_lowercase().as_str()) {
            self.check_part(npc, &npc.hair, &unique.hair, "hair");
            self.check_part(npc, &npc.head, &unique.head, "head");
        }
    }

    fn check_part_rules(
        &self,
        npc: &Npc,
        part_id: &str,
        rules: &HashMap<&'static str, AllRules>,
        name: &str,
    ) {
        let bodypart = part_id.to_lowercase();
        if let Some(rule) = rules.get(bodypart.as_str()) {
            if !rule.test(npc) {
                println!("Npc {} is using {} {}", npc.id, name, part_id);
            }
        }
    }

    fn check_part(&self, npc: &Npc, actual: &str, expected: &Option<&'static str>, name: &str) {
        if let Some(expid) = expected {
            if expid.eq_ignore_ascii_case(actual) {
                return;
            }
            println!("Npc {} is not using unique {} {}", npc.id, name, expid);
        }
    }

    pub fn get_unique_heads(&self) -> HashSet<&'static str> {
        let mut set = HashSet::new();
        for (id, unique) in &self.uniques {
            if unique.head.is_some() {
                set.insert(*id);
            }
        }
        set
    }
}
