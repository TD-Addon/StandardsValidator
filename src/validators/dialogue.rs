use std::collections::HashSet;

use super::Context;
use crate::{context::Mode, handlers::Handler, util::is_khajiit};
use regex::{Error, Regex, RegexBuilder};
use tes3::esp::{
    Dialogue, DialogueInfo, DialogueType, FilterComparison, FilterFunction, FilterType,
    FilterValue, Sex, TES3Object,
};

const HIGH_RANK: i8 = 7;

pub struct DialogueValidator {
    blank: Regex,
    double_spaces: Regex,
    short_ellipsis: Regex,
    punctuation_whitespace: Regex,
    punctuation_double: Regex,
    article_pc: Regex,
    overrides: Regex,
    khajiit: HashSet<String>,
}

fn get_int(value: FilterValue) -> i32 {
    match value {
        FilterValue::Integer(i) => i,
        FilterValue::Float(f) => f as i32,
    }
}

fn to_op(comp: FilterComparison) -> &'static str {
    match comp {
        FilterComparison::Equal => "=",
        FilterComparison::Greater => ">",
        FilterComparison::GreaterEqual => ">=",
        FilterComparison::Less => "<",
        FilterComparison::LessEqual => "<=",
        FilterComparison::NotEqual => "!=",
    }
}

impl Handler<'_> for DialogueValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object) {
        if let TES3Object::Npc(npc) = record {
            if is_khajiit(&npc.race) {
                self.khajiit.insert(npc.id.to_ascii_lowercase());
            }
        }
    }

    fn on_info(&mut self, context: &Context, record: &DialogueInfo, topic: &Dialogue) {
        if record.speaker_id.eq_ignore_ascii_case("dialog placeholder") {
            return;
        }
        if record.text.is_empty() {
            if record.data.dialogue_type != DialogueType::Journal
                && record.data.dialogue_type != DialogueType::Voice
                && !self.intentionally_left_blank(record)
            {
                println!("Info {} in topic {} has no text", record.id, topic.id);
            }
        } else {
            let text = &record.text;
            if self.double_spaces.is_match(text) {
                println!(
                    "Info {} in topic {} contains double spaces",
                    record.id, topic.id
                );
            }
            if text.contains(" - ") {
                println!(
                    "Info {} in topic {} contains a single hyphen",
                    record.id, topic.id
                );
            }
            if text.contains("....") {
                println!(
                    "Info {} in topic {} contains an overlong ellipsis",
                    record.id, topic.id
                );
            }
            if self.short_ellipsis.is_match(text) {
                println!(
                    "Info {} in topic {} contains a short ellipsis",
                    record.id, topic.id
                );
            }
            if self.punctuation_whitespace.is_match(text) {
                println!(
                    "Info {} in topic {} contains punctuation preceded by whitespace",
                    record.id, topic.id
                );
            }
            if self.punctuation_double.is_match(text) {
                println!(
                    "Info {} in topic {} contains doubled up punctuation",
                    record.id, topic.id
                );
            }
            if self.article_pc.is_match(text) {
                println!(
                    "Info {} in topic {} contains an indefinite article followed by a PC variable",
                    record.id, topic.id
                );
            }
            let start_trimmed = text.trim_start();
            if start_trimmed.len() != text.len() && !start_trimmed.is_empty() {
                println!(
                    "Info {} in topic {} contains leading whitespace",
                    record.id, topic.id
                );
            }
            let end_trimmed = start_trimmed.trim_end();
            if end_trimmed.len() != start_trimmed.len() {
                println!(
                    "Info {} in topic {} contains trailing whitespace",
                    record.id, topic.id
                );
            }
            if text.starts_with('*') {
                println!(
                    "Info {} in topic {} starts with an asterisk",
                    record.id, topic.id
                );
            }
        }
        for filter in &record.filters {
            let value = get_int(filter.value);
            if filter.filter_type == FilterType::Dead
                && filter.comparison == FilterComparison::Equal
                && value > 0
            {
                println!(
                    "Info {} in topic {} checks for Dead = {}",
                    record.id, topic.id, value
                );
            }
        }
        if !record.speaker_id.is_empty() {
            let speaker = &record.speaker_id;
            let is_player = speaker.eq_ignore_ascii_case("player");
            if !is_player {
                if !record.speaker_race.is_empty() {
                    println!(
                        "Info {} in topic {} has an unnecessary race filter",
                        record.id, topic.id
                    );
                }
                if !record.speaker_class.is_empty() {
                    println!(
                        "Info {} in topic {} has an unnecessary class filter",
                        record.id, topic.id
                    );
                }
                if !record.speaker_faction.is_empty() {
                    println!(
                        "Info {} in topic {} has an unnecessary faction filter",
                        record.id, topic.id
                    );
                }
                if record.data.speaker_sex != Sex::Any {
                    println!(
                        "Info {} in topic {} has an unnecessary sex filter",
                        record.id, topic.id
                    );
                }
            }
            let mut has_samerace_filter = false;
            for filter in &record.filters {
                if filter.filter_type == FilterType::Local
                    || filter.filter_type == FilterType::NotLocal
                {
                    if filter.id.eq_ignore_ascii_case("nolore")
                        || filter.id.eq_ignore_ascii_case("t_local_nolore")
                        || filter.id.eq_ignore_ascii_case("t_local_khajiit")
                        || filter.id.eq_ignore_ascii_case("t_local_npc")
                    {
                        println!(
                            "Info {} in topic {} has a {} filter",
                            record.id, topic.id, filter.id
                        );
                    }
                } else if filter.filter_type == FilterType::NotId {
                    println!(
                        "Info {} in topic {} has an unnecessary Not ID filter",
                        record.id, topic.id
                    );
                } else if filter.filter_type == FilterType::Function
                    && filter.function == FilterFunction::SameRace
                {
                    has_samerace_filter = true;
                } else if !is_player {
                    if filter.filter_type == FilterType::NotFaction {
                        println!(
                            "Info {} in topic {} has an unnecessary Not Faction filter",
                            record.id, topic.id
                        );
                    } else if filter.filter_type == FilterType::NotClass {
                        println!(
                            "Info {} in topic {} has an unnecessary Not Class filter",
                            record.id, topic.id
                        );
                    } else if filter.filter_type == FilterType::NotRace {
                        println!(
                            "Info {} in topic {} has an unnecessary Not Race filter",
                            record.id, topic.id
                        );
                    }
                }
            }
            if has_samerace_filter && self.khajiit.contains(&speaker.to_ascii_lowercase()) {
                println!(
                    "Info {} in topic {} has a Khajiit related Same Race filter",
                    record.id, topic.id
                );
            }
        } else if record.data.dialogue_type == DialogueType::Voice {
            if context.mode == Mode::TD {
                let race = &record.speaker_race;
                if !race.is_empty() && context.projects.iter().any(|p| p.matches(race)) {
                    return;
                }
            }
            if context.mode != Mode::Vanilla {
                let mut khajiit = is_khajiit(&record.speaker_race);
                let mut has_samerace_filter = false;
                let mut project = false;
                for filter in &record.filters {
                    if filter.filter_type == FilterType::Function
                        && filter.function == FilterFunction::SameRace
                    {
                        has_samerace_filter = true;
                    } else if filter.filter_type == FilterType::Local && !filter.id.is_empty() {
                        let khajiit_local = filter.id.eq_ignore_ascii_case("t_local_khajiit");
                        khajiit |= khajiit_local;
                        project |= khajiit_local
                            || filter.id.eq_ignore_ascii_case("t_local_npc")
                            || context
                                .projects
                                .iter()
                                .any(|p| p.has_local(&filter.id) || p.matches(&filter.id));
                    }
                }
                if khajiit && has_samerace_filter {
                    println!(
                        "Info {} in topic {} has a Khajiit related Same Race filter",
                        record.id, topic.id
                    );
                }
                if !project {
                    println!(
                        "Info {} in topic {} does not have a known project specific local filter",
                        record.id, topic.id
                    );
                }
            }
        } else if matches!(
            record.data.dialogue_type,
            DialogueType::Greeting | DialogueType::Topic | DialogueType::Persuasion
        ) {
            let is_service_refusal = topic.id == "Service Refusal";
            let mut project = false;
            let mut nolore = false;
            let mut vanilla_nolore = false;
            let mut choice = false;
            let mut khajiit = is_khajiit(&record.speaker_race);
            let mut has_samerace_filter = false;
            for filter in &record.filters {
                if filter.filter_type == FilterType::Local {
                    let khajiit_local = filter.id.eq_ignore_ascii_case("t_local_khajiit");
                    khajiit |= khajiit_local;
                    if filter.id.eq_ignore_ascii_case("t_local_nolore")
                        || filter.id.eq_ignore_ascii_case("nolore")
                    {
                        println!(
                            "Info {} in topic {} has a Local {} filter",
                            record.id, topic.id, filter.id
                        );
                    } else if !project || !nolore {
                        if filter.id.eq_ignore_ascii_case("t_local_npc")
                            || khajiit_local
                            || context.projects.iter().any(|p| p.has_local(&filter.id))
                        {
                            project = true;
                        } else if !filter.id.is_empty()
                            && context.projects.iter().any(|p| p.matches(&filter.id))
                        {
                            project = true;
                            nolore = true;
                        }
                    }
                    let value = get_int(filter.value);
                    if filter.id.eq_ignore_ascii_case("t_local_npc")
                        && (filter.comparison != FilterComparison::Equal || value != 0)
                        || khajiit_local
                            && (filter.comparison != FilterComparison::Equal || value != 1)
                    {
                        println!(
                            "Info {} in topic {} has a Local {} {} {:?} filter",
                            record.id,
                            topic.id,
                            filter.id,
                            to_op(filter.comparison),
                            value
                        );
                    }
                } else if filter.filter_type == FilterType::NotLocal {
                    let value = get_int(filter.value);
                    if filter.id.eq_ignore_ascii_case("t_local_nolore") {
                        nolore = true;
                        if filter.comparison != FilterComparison::Equal || value != 0 {
                            println!(
                                "Info {} in topic {} has a Not Local {} {} {:?} filter",
                                record.id,
                                topic.id,
                                filter.id,
                                to_op(filter.comparison),
                                value
                            );
                        }
                    } else if filter.id.eq_ignore_ascii_case("nolore")
                        && filter.comparison == FilterComparison::Equal
                        && value == 0
                    {
                        vanilla_nolore = true;
                    } else if (filter.id.eq_ignore_ascii_case("t_local_npc")
                        || filter.id.eq_ignore_ascii_case("t_local_khajiit"))
                        && (filter.comparison != FilterComparison::Equal && value != 1)
                    {
                        println!(
                            "Info {} in topic {} has a Not Local {} {} {:?} filter",
                            record.id,
                            topic.id,
                            filter.id,
                            to_op(filter.comparison),
                            value
                        );
                    }
                } else if filter.filter_type == FilterType::Function {
                    choice |= filter.function == FilterFunction::Choice;
                    has_samerace_filter = filter.function == FilterFunction::SameRace;
                }
            }
            if khajiit && has_samerace_filter {
                println!(
                    "Info {} in topic {} has a Khajiit related Same Race filter",
                    record.id, topic.id
                );
            }
            if !project {
                project = !record.speaker_faction.is_empty()
                    && context
                        .projects
                        .iter()
                        .any(|p| p.matches(&record.speaker_faction));
            }
            if vanilla_nolore {
                if project {
                    println!(
                        "Info {} in topic {} has a Not Local NoLore filter",
                        record.id, topic.id
                    );
                } else {
                    return;
                }
            } else if context.mode == Mode::Vanilla {
                println!(
                    "Info {} in topic {} does not have a NoLore filter",
                    record.id, topic.id
                );
                return;
            }
            if !(project
                || choice
                || self.overrides_vanilla(record)
                || is_service_refusal && context.mode == Mode::TD)
            {
                println!(
                    "Info {} in topic {} does not have a known project specific local filter",
                    record.id, topic.id
                );
            }
            if !nolore
                && !is_service_refusal
                && !choice
                && self.needs_nolore(record, topic, context)
            {
                println!(
                    "Info {} in topic {} does not have a T_Local_NoLore filter",
                    record.id, topic.id
                );
            }
        }
    }
}

impl DialogueValidator {
    pub fn new() -> Result<Self, Error> {
        let blank = RegexBuilder::new(r"(^|\n)\s*;\s*SV:\s*intentionally\s+left\s+blank\s*($|\n)")
            .case_insensitive(true)
            .build()?;
        let double_spaces = Regex::new(r"[^\S\r\n]{2,}")?;
        let short_ellipsis = Regex::new(r"[^.]\.{2}[^.?]")?;
        let punctuation_whitespace = Regex::new(r"\s[.,:;?]($|\s)")?;
        let punctuation_double = Regex::new(r"[,:;]{2,}")?;
        let article_pc = RegexBuilder::new(r"(^|\s)an?\s+%PC")
            .case_insensitive(true)
            .build()?;
        let overrides = RegexBuilder::new(r"(^|\n)\s*;\s*SV:\s*vanilla\s+override\s*($|\n)")
            .case_insensitive(true)
            .build()?;
        Ok(Self {
            blank,
            double_spaces,
            short_ellipsis,
            punctuation_whitespace,
            punctuation_double,
            article_pc,
            overrides,
            khajiit: HashSet::new(),
        })
    }

    fn intentionally_left_blank(&self, record: &DialogueInfo) -> bool {
        self.blank.is_match(&record.script_text)
    }

    fn overrides_vanilla(&self, record: &DialogueInfo) -> bool {
        self.overrides.is_match(&record.script_text)
    }

    fn needs_nolore(&self, record: &DialogueInfo, topic: &Dialogue, context: &Context) -> bool {
        if context.mode == Mode::TD
            && (record.data.dialogue_type == DialogueType::Greeting
                || record.data.dialogue_type == DialogueType::Voice
                || !record.speaker_faction.is_empty() && record.data.speaker_rank > 0)
        {
            return false;
        }
        record.data.speaker_rank < HIGH_RANK // High-level faction dialogue
            && !record
                .speaker_class
                .eq_ignore_ascii_case("slave") // Slaves are generally NoLore
            && topic.id != "Greeting 9" // Has greetings for NoLore at the top
            && !self.overrides_vanilla(record)
    }
}
