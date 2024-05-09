use std::{collections::HashMap, rc::Rc};

use super::Context;
use crate::{
    context::Mode,
    handlers::Handler,
    util::{ci_starts_with, is_dead},
};
use serde::Deserialize;
use tes3::esp::{Effect, EffectId2, EnchantType, Npc, SpellType, TES3Object};

pub struct MagicValidator {
    spells: HashMap<String, (Rc<Rule>, Rc<Vec<String>>)>,
}

enum Duration {
    Integer(u32),
    Bool(bool),
}

fn get_effect_details(id: EffectId2) -> (bool, Duration, bool) {
    return match id {
        EffectId2::AbsorbAttribute => (false, Duration::Bool(true), true),
        EffectId2::AbsorbFatigue => (false, Duration::Bool(false), true),
        EffectId2::AbsorbHealth => (false, Duration::Bool(false), true),
        EffectId2::AbsorbMagicka => (false, Duration::Bool(false), true),
        EffectId2::AbsorbSkill => (false, Duration::Bool(true), true),
        EffectId2::AlmsiviIntervention => (false, Duration::Bool(false), false),
        EffectId2::Blind => (false, Duration::Bool(true), true),
        EffectId2::BoundBattleAxe => (false, Duration::Bool(true), false),
        EffectId2::BoundBoots => (false, Duration::Bool(true), false),
        EffectId2::BoundCuirass => (false, Duration::Bool(true), false),
        EffectId2::BoundDagger => (false, Duration::Bool(true), false),
        EffectId2::BoundGloves => (false, Duration::Bool(true), false),
        EffectId2::BoundHelm => (false, Duration::Bool(true), false),
        EffectId2::BoundLongbow => (false, Duration::Bool(true), false),
        EffectId2::BoundLongsword => (false, Duration::Bool(true), false),
        EffectId2::BoundMace => (false, Duration::Bool(true), false),
        EffectId2::BoundShield => (false, Duration::Bool(true), false),
        EffectId2::BoundSpear => (false, Duration::Bool(true), false),
        EffectId2::Burden => (false, Duration::Bool(true), true),
        EffectId2::CalmCreature => (false, Duration::Bool(false), true),
        EffectId2::CalmHumanoid => (false, Duration::Bool(false), true),
        EffectId2::Chameleon => (false, Duration::Bool(true), true),
        EffectId2::Charm => (false, Duration::Bool(true), true),
        EffectId2::CommandCreature => (false, Duration::Bool(true), true),
        EffectId2::CommandHumanoid => (false, Duration::Bool(true), true),
        EffectId2::Corprus => (true, Duration::Bool(false), false),
        EffectId2::CureBlightDisease => (false, Duration::Bool(false), false),
        EffectId2::CureCommonDisease => (false, Duration::Bool(false), false),
        EffectId2::CureCorprus => (true, Duration::Bool(false), false),
        EffectId2::CureParalyzation => (false, Duration::Bool(false), false),
        EffectId2::CurePoison => (false, Duration::Bool(false), false),
        EffectId2::DamageAttribute => (false, Duration::Bool(false), true),
        EffectId2::DamageFatigue => (false, Duration::Bool(false), true),
        EffectId2::DamageHealth => (false, Duration::Bool(false), true),
        EffectId2::DamageMagicka => (false, Duration::Bool(false), true),
        EffectId2::DamageSkill => (false, Duration::Bool(false), true),
        EffectId2::DemoralizeCreature => (false, Duration::Bool(true), true),
        EffectId2::DemoralizeHumanoid => (false, Duration::Bool(true), true),
        EffectId2::DetectAnimal => (false, Duration::Bool(true), true),
        EffectId2::DetectEnchantment => (false, Duration::Bool(true), true),
        EffectId2::DetectKey => (false, Duration::Bool(true), true),
        EffectId2::DisintegrateArmor => (false, Duration::Bool(false), true),
        EffectId2::DisintegrateWeapon => (false, Duration::Bool(false), true),
        EffectId2::Dispel => (false, Duration::Bool(false), true),
        EffectId2::DivineIntervention => (false, Duration::Bool(false), false),
        EffectId2::DrainAttribute => (false, Duration::Bool(true), true),
        EffectId2::DrainFatigue => (false, Duration::Bool(true), true),
        EffectId2::DrainHealth => (false, Duration::Bool(false), true),
        EffectId2::DrainMagicka => (false, Duration::Bool(true), true),
        EffectId2::DrainSkill => (false, Duration::Bool(true), true),
        EffectId2::Feather => (false, Duration::Bool(true), true),
        EffectId2::FireDamage => (false, Duration::Bool(false), true),
        EffectId2::FireShield => (false, Duration::Bool(true), true),
        EffectId2::FortifyAttackBonus => (false, Duration::Bool(true), true),
        EffectId2::FortifyAttribute => (false, Duration::Bool(true), true),
        EffectId2::FortifyFatigue => (false, Duration::Bool(true), true),
        EffectId2::FortifyHealth => (false, Duration::Bool(true), true),
        EffectId2::FortifyMagicka => (false, Duration::Bool(true), true),
        EffectId2::FortifyMagickaMultiplier => (false, Duration::Bool(true), true),
        EffectId2::FortifySkill => (false, Duration::Bool(true), true),
        EffectId2::FrenzyCreature => (false, Duration::Bool(true), true),
        EffectId2::FrenzyHumanoid => (false, Duration::Bool(true), true),
        EffectId2::FrostDamage => (false, Duration::Bool(false), true),
        EffectId2::FrostShield => (false, Duration::Bool(true), true),
        EffectId2::Invisibility => (false, Duration::Bool(true), true),
        EffectId2::Jump => (false, Duration::Bool(true), true),
        EffectId2::Levitate => (false, Duration::Bool(true), true),
        EffectId2::Light => (false, Duration::Bool(true), true),
        EffectId2::LightningShield => (false, Duration::Bool(true), true),
        EffectId2::Lock => (false, Duration::Bool(false), true),
        EffectId2::Mark => (false, Duration::Bool(false), false),
        EffectId2::NightEye => (false, Duration::Bool(true), true),
        EffectId2::Open => (false, Duration::Bool(false), true),
        EffectId2::Paralyze => (false, Duration::Integer(1), false),
        EffectId2::Poison => (false, Duration::Bool(false), true),
        EffectId2::RallyCreature => (false, Duration::Bool(true), true),
        EffectId2::RallyHumanoid => (false, Duration::Bool(true), true),
        EffectId2::Recall => (false, Duration::Bool(false), false),
        EffectId2::Reflect => (false, Duration::Bool(true), true),
        EffectId2::RemoveCurse => (true, Duration::Bool(false), false),
        EffectId2::ResistBlightDisease => (false, Duration::Bool(true), true),
        EffectId2::ResistCommonDisease => (false, Duration::Bool(true), true),
        EffectId2::ResistCorprus => (true, Duration::Bool(false), false),
        EffectId2::ResistFire => (false, Duration::Bool(true), true),
        EffectId2::ResistFrost => (false, Duration::Bool(true), true),
        EffectId2::ResistMagicka => (false, Duration::Bool(true), true),
        EffectId2::ResistNormalWeapons => (false, Duration::Bool(true), true),
        EffectId2::ResistParalysis => (false, Duration::Bool(true), true),
        EffectId2::ResistPoison => (false, Duration::Bool(true), true),
        EffectId2::ResistShock => (false, Duration::Bool(true), true),
        EffectId2::RestoreAttribute => (false, Duration::Bool(false), true),
        EffectId2::RestoreFatigue => (false, Duration::Bool(false), true),
        EffectId2::RestoreHealth => (false, Duration::Bool(false), true),
        EffectId2::RestoreMagicka => (false, Duration::Bool(false), true),
        EffectId2::RestoreSkill => (false, Duration::Bool(false), true),
        EffectId2::Sanctuary => (false, Duration::Bool(true), true),
        EffectId2::Shield => (false, Duration::Bool(true), true),
        EffectId2::ShockDamage => (false, Duration::Bool(false), true),
        EffectId2::Silence => (false, Duration::Bool(true), false),
        EffectId2::SlowFall => (false, Duration::Bool(true), true),
        EffectId2::SoulTrap => (false, Duration::Bool(true), true),
        EffectId2::Sound => (false, Duration::Bool(true), true),
        EffectId2::SpellAbsorption => (false, Duration::Bool(true), true),
        EffectId2::StuntedMagicka => (false, Duration::Bool(false), false),
        EffectId2::SummonBear => (false, Duration::Bool(true), false),
        EffectId2::SummonBoneWolf => (false, Duration::Bool(true), false),
        EffectId2::SummonBonelord => (false, Duration::Bool(true), false),
        EffectId2::SummonCenturionSphere => (false, Duration::Bool(true), false),
        EffectId2::SummonClannfear => (false, Duration::Bool(true), false),
        EffectId2::SummonDaedroth => (false, Duration::Bool(true), false),
        EffectId2::SummonDremora => (false, Duration::Bool(true), false),
        EffectId2::SummonFabricant => (false, Duration::Bool(true), false),
        EffectId2::SummonFlameAtronach => (false, Duration::Bool(true), false),
        EffectId2::SummonFrostAtronach => (false, Duration::Bool(true), false),
        EffectId2::SummonGhost => (false, Duration::Bool(true), false),
        EffectId2::SummonGoldenSaint => (false, Duration::Bool(true), false),
        EffectId2::SummonGreaterBonewalker => (false, Duration::Bool(true), false),
        EffectId2::SummonHunger => (false, Duration::Bool(true), false),
        EffectId2::SummonLeastBonewalker => (false, Duration::Bool(true), false),
        EffectId2::SummonScamp => (false, Duration::Bool(true), false),
        EffectId2::SummonSkeleton => (false, Duration::Bool(true), false),
        EffectId2::SummonStormAtronach => (false, Duration::Bool(true), false),
        EffectId2::SummonTwilight => (false, Duration::Bool(true), false),
        EffectId2::SummonWolf => (false, Duration::Bool(true), false),
        EffectId2::SunDamage => (false, Duration::Bool(false), false),
        EffectId2::SwiftSwim => (false, Duration::Bool(true), true),
        EffectId2::Telekinesis => (false, Duration::Bool(true), true),
        EffectId2::TurnUndead => (false, Duration::Bool(true), true),
        EffectId2::Vampirism => (false, Duration::Bool(false), false),
        EffectId2::WaterBreathing => (false, Duration::Bool(true), true),
        EffectId2::WaterWalking => (false, Duration::Bool(true), true),
        EffectId2::WeaknessToBlightDisease => (false, Duration::Bool(true), true),
        EffectId2::WeaknessToCommonDisease => (false, Duration::Bool(true), true),
        EffectId2::WeaknessToCorprus => (true, Duration::Bool(false), false),
        EffectId2::WeaknessToFire => (false, Duration::Integer(1), true),
        EffectId2::WeaknessToFrost => (false, Duration::Integer(1), true),
        EffectId2::WeaknessToMagicka => (false, Duration::Integer(1), true),
        EffectId2::WeaknessToNormalWeapons => (false, Duration::Bool(true), true),
        EffectId2::WeaknessToPoison => (false, Duration::Integer(1), true),
        EffectId2::WeaknessToShock => (false, Duration::Integer(1), true),
        _ => (true, Duration::Bool(false), false),
    };
}

fn check_effects(
    typename: &'static str,
    id: &String,
    option: &Option<Vec<Effect>>,
    constant_effect: bool,
) {
    for effect in option.iter().flat_map(|v| v.iter()) {
        let (illegal, duration, magnitude) = get_effect_details(effect.magic_effect);
        if illegal {
            println!("{} {} uses {:?}", typename, id, effect.magic_effect);
        } else {
            if magnitude && (effect.min_magnitude <= 0 && effect.max_magnitude <= 0) {
                println!(
                    "{} {} uses {:?} without a magnitude",
                    typename, id, effect.magic_effect
                );
            }
            match duration {
                Duration::Bool(check) => {
                    if check && effect.duration <= 1 && !constant_effect {
                        println!(
                            "{} {} uses {:?} with duration {}",
                            typename, id, effect.magic_effect, effect.duration
                        );
                    }
                }
                Duration::Integer(value) => {
                    if effect.duration < value && !constant_effect {
                        println!(
                            "{} {} uses {:?} with duration {}",
                            typename, id, effect.magic_effect, effect.duration
                        );
                    }
                }
            }
        }
    }
}

impl Handler<'_> for MagicValidator {
    fn on_record(
        &mut self,
        context: &Context,
        record: &TES3Object,
        typename: &'static str,
        id: &String,
    ) {
        match record {
            TES3Object::Npc(npc) => {
                if !is_dead(record) {
                    for id in npc.spells.iter().flat_map(|s| s.iter()) {
                        if let Some((rule, alternatives)) =
                            self.spells.get(&id.to_ascii_lowercase())
                        {
                            if !rule.matches(npc) {
                                if context.mode == Mode::Vanilla {
                                    if alternatives.is_empty() {
                                        println!("Npc {} knows spell {}", npc.id, id);
                                    }
                                    return;
                                }
                                let valid_alternatives: Vec<String> = alternatives
                                    .iter()
                                    .filter(|a| {
                                        self.spells.get(*a).iter().any(|r| r.0.matches(npc))
                                    })
                                    .map(&String::clone)
                                    .collect();
                                if valid_alternatives.is_empty() {
                                    println!("Npc {} knows spell {}", npc.id, id);
                                } else {
                                    println!(
                                        "Npc {} knows spell {} which should probably be {}",
                                        npc.id,
                                        id,
                                        valid_alternatives.join(" or ")
                                    );
                                }
                            }
                        }
                    }
                }
            }
            TES3Object::Alchemy(potion) => {
                check_effects(typename, id, &potion.effects, false);
            }
            TES3Object::Enchanting(enchantment) => {
                let constant_effect = enchantment
                    .data
                    .iter()
                    .any(|d| d.kind == EnchantType::ConstantEffect);
                check_effects(typename, id, &enchantment.effects, constant_effect);
            }
            TES3Object::Spell(spell) => {
                let temporary = spell
                    .data
                    .iter()
                    .any(|d| d.kind == SpellType::Power || d.kind == SpellType::Spell);
                check_effects(typename, id, &spell.effects, !temporary);
            }
            _ => {}
        }
    }
}

#[derive(Deserialize)]
struct Rule {
    prefix: Option<String>,
    race: Option<String>,
}

impl Rule {
    fn matches(&self, npc: &Npc) -> bool {
        if let Some(prefix) = &self.prefix {
            if ci_starts_with(&npc.id, prefix) {
                return true;
            }
        }
        if let Some(race) = &self.race {
            if let Some(id) = &npc.race {
                return race.eq_ignore_ascii_case(&id);
            }
        }
        return false;
    }
}

#[derive(Deserialize)]
struct SpellData {
    alternatives: Vec<HashMap<String, String>>,
    races: HashMap<String, Rule>,
    blacklist: Vec<String>,
}

impl MagicValidator {
    pub fn new() -> serde_json::Result<Self> {
        let data: SpellData = serde_json::from_str(include_str!("../../data/spells.json"))?;
        let mut rules: HashMap<String, Rc<Rule>> = HashMap::new();
        for (id, rule) in data.races {
            rules.insert(id, Rc::new(rule));
        }
        let mut spells = HashMap::new();
        for alternatives in &data.alternatives {
            let ids: Rc<Vec<String>> = Rc::new(
                alternatives
                    .values()
                    .map(&String::as_str)
                    .map(&str::to_ascii_lowercase)
                    .collect(),
            );
            for (rule, spell) in alternatives {
                if let Some(rule) = rules.get(rule) {
                    spells.insert(spell.to_ascii_lowercase(), (rule.clone(), ids.clone()));
                }
            }
        }
        let never = Rc::new(Rule {
            prefix: None,
            race: None,
        });
        let none = Rc::new(Vec::new());
        for id in &data.blacklist {
            spells.insert(id.to_ascii_lowercase(), (never.clone(), none.clone()));
        }
        return Ok(MagicValidator { spells });
    }
}
