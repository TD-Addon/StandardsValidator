use super::Context;
use crate::{context::Mode, handlers::Handler, util::is_empty};
use regex::{Error, Regex, RegexBuilder};
use tes3::esp::{
    Dialogue, DialogueType, FilterComparison, FilterFunction, FilterType, FilterValue, Info, Sex,
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
}

fn get_int(value: &Option<FilterValue>) -> Option<i32> {
    if let Some(v) = value {
        match v {
            FilterValue::Integer(i) => return Some(*i),
            FilterValue::Float(f) => return Some(*f as i32),
        }
    }
    return None;
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
    fn on_info(&mut self, context: &Context, record: &Info, topic: &Dialogue) {
        if record
            .speaker_id
            .iter()
            .any(|id| id.eq_ignore_ascii_case("dialog placeholder"))
        {
            return;
        }
        if is_empty(&record.text) {
            if !record
                .data
                .iter()
                .any(|data| data.kind == DialogueType::Journal || data.kind == DialogueType::Voice)
                && !self.intentionally_left_blank(record)
            {
                println!("Info {} in topic {} has no text", record.id, topic.id);
            }
        } else {
            let text = record.text.as_ref().unwrap();
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
        }
        for filter in record.filters.iter().flat_map(|f| f.iter()) {
            if let Some(value) = get_int(&filter.value) {
                if filter.kind == FilterType::Dead
                    && filter.comparison == FilterComparison::Equal
                    && value > 0
                {
                    println!(
                        "Info {} in topic {} checks for Dead = {}",
                        record.id, topic.id, value
                    );
                }
            }
        }
        if let Some(speaker) = &record.speaker_id {
            let is_player = speaker.eq_ignore_ascii_case("player");
            if !is_player {
                if !is_empty(&record.speaker_rank) {
                    println!(
                        "Info {} in topic {} has an unnecessary race filter",
                        record.id, topic.id
                    );
                }
                if !is_empty(&record.speaker_class) {
                    println!(
                        "Info {} in topic {} has an unnecessary class filter",
                        record.id, topic.id
                    );
                }
                if !is_empty(&record.speaker_faction) {
                    println!(
                        "Info {} in topic {} has an unnecessary faction filter",
                        record.id, topic.id
                    );
                }
                if record.data.iter().any(|d| d.speaker_sex != Sex::Any) {
                    println!(
                        "Info {} in topic {} has an unnecessary sex filter",
                        record.id, topic.id
                    );
                }
            }
            for filter in record.filters.iter().flat_map(|f| f.iter()) {
                if filter.kind == FilterType::Local || filter.kind == FilterType::NotLocal {
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
                } else if filter.kind == FilterType::NotId {
                    println!(
                        "Info {} in topic {} has an unnecessary Not ID filter",
                        record.id, topic.id
                    );
                } else if !is_player {
                    if filter.kind == FilterType::NotFaction {
                        println!(
                            "Info {} in topic {} has an unnecessary Not Faction filter",
                            record.id, topic.id
                        );
                    } else if filter.kind == FilterType::NotClass {
                        println!(
                            "Info {} in topic {} has an unnecessary Not Class filter",
                            record.id, topic.id
                        );
                    } else if filter.kind == FilterType::NotRace {
                        println!(
                            "Info {} in topic {} has an unnecessary Not Race filter",
                            record.id, topic.id
                        );
                    }
                }
            }
        } else if record.data.iter().any(|d| d.kind == DialogueType::Voice) {
            if context.mode == Mode::TD {
                if let Some(race) = &record.speaker_rank {
                    if !race.is_empty() && context.projects.iter().any(|p| p.matches(race)) {
                        return;
                    }
                }
            }
            if context.mode != Mode::Vanilla {
                let project = record.filters.iter().flat_map(|f| f.iter()).any(|filter| {
                    if filter.kind == FilterType::Local && !filter.id.is_empty() {
                        return filter.id.eq_ignore_ascii_case("t_local_npc")
                            || filter.id.eq_ignore_ascii_case("t_local_khajiit")
                            || context
                                .projects
                                .iter()
                                .any(|p| p.has_local(&filter.id) || p.matches(&filter.id));
                    }
                    return false;
                });
                if !project {
                    println!(
                        "Info {} in topic {} does not have a known project specific local filter",
                        record.id, topic.id
                    );
                }
            }
        } else if record.data.iter().any(|d| {
            d.kind == DialogueType::Greeting
                || d.kind == DialogueType::Topic
                || d.kind == DialogueType::Persuasion
        }) {
            let is_service_refusal = topic.id == "Service Refusal";
            let mut project = false;
            let mut nolore = false;
            let mut vanilla_nolore = false;
            let mut choice = false;
            for filter in record.filters.iter().flat_map(|f| f.iter()) {
                if filter.kind == FilterType::Local {
                    if filter.id.eq_ignore_ascii_case("t_local_nolore")
                        || filter.id.eq_ignore_ascii_case("nolore")
                    {
                        println!(
                            "Info {} in topic {} has a Local {} filter",
                            record.id, topic.id, filter.id
                        );
                    } else if !project || !nolore {
                        if filter.id.eq_ignore_ascii_case("t_local_npc")
                            || filter.id.eq_ignore_ascii_case("t_local_khajiit")
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
                    let value = get_int(&filter.value);
                    if filter.id.eq_ignore_ascii_case("t_local_npc")
                        && (filter.comparison != FilterComparison::Equal || value != Some(0))
                        || filter.id.eq_ignore_ascii_case("t_local_khajiit")
                            && (filter.comparison != FilterComparison::Equal || value != Some(1))
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
                } else if filter.kind == FilterType::NotLocal {
                    let value = get_int(&filter.value);
                    if filter.id.eq_ignore_ascii_case("t_local_nolore") {
                        nolore = true;
                        if filter.comparison != FilterComparison::Equal || value != Some(0) {
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
                        && value == Some(0)
                    {
                        vanilla_nolore = true;
                    } else if (filter.id.eq_ignore_ascii_case("t_local_npc")
                        || filter.id.eq_ignore_ascii_case("t_local_khajiit"))
                        && (filter.comparison != FilterComparison::Equal && value != Some(1))
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
                } else if filter.kind == FilterType::Function
                    && filter.function == FilterFunction::Choice
                {
                    choice = true;
                }
            }
            if !project {
                if let Some(faction) = &record.speaker_faction {
                    project =
                        !faction.is_empty() && context.projects.iter().any(|p| p.matches(faction));
                }
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
            if !project
                && !(is_service_refusal && context.mode == Mode::TD)
                && !choice
                && !self.overrides_vanilla(record)
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
        return Ok(Self {
            blank,
            double_spaces,
            short_ellipsis,
            punctuation_whitespace,
            punctuation_double,
            article_pc,
            overrides,
        });
    }

    fn intentionally_left_blank(&self, record: &Info) -> bool {
        if let Some(text) = &record.script_text {
            return self.blank.is_match(text);
        }
        return false;
    }

    fn overrides_vanilla(&self, record: &Info) -> bool {
        if let Some(text) = &record.script_text {
            return self.overrides.is_match(text);
        }
        return false;
    }

    fn needs_nolore(&self, record: &Info, topic: &Dialogue, context: &Context) -> bool {
        if context.mode == Mode::TD {
            if record.data.iter().any(|d| d.kind == DialogueType::Greeting)
                || !is_empty(&record.speaker_faction)
            {
                return false;
            }
        }
        return !record.data.iter().any(|d| d.speaker_rank >= HIGH_RANK) // High-level faction dialogue
            && !record
                .speaker_class
                .iter()
                .any(|c| c.eq_ignore_ascii_case("slave")) // Slaves are generally NoLore
            && topic.id != "Greeting 9" // Has greetings for NoLore at the top
            && !self.overrides_vanilla(record);
    }
}
