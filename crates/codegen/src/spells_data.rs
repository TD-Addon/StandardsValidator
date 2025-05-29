use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use serde::Deserialize;

use crate::common::_Option;

#[derive(Default, Deserialize)]
struct SpellRule {
    prefix: _Option<String>,
    race: _Option<String>,
    vendor: _Option<bool>,
}

#[derive(Deserialize)]
struct SpellData {
    alternatives: Vec<HashMap<String, String>>,
    races: HashMap<String, SpellRule>,
    blacklist: Vec<String>,
    vendor_blacklist: Vec<String>,
}

impl ToTokens for SpellRule {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let prefix = &self.prefix;
        let race = &self.race;
        let vendor = &self.vendor;
        quote! {
            Rule {
                prefix: #prefix,
                race: #race,
                vendor: #vendor,
            }
        }
        .to_tokens(tokens)
    }
}

pub fn generate() -> TokenStream {
    let data: SpellData = serde_json::from_str(include_str!("../data/spells.json")).unwrap();

    let mut spells = HashMap::new();

    for entry in data.alternatives {
        let spell_ids: Vec<_> = entry.values().map(|id| id.to_ascii_lowercase()).collect();

        for (rule_id, spell_id) in entry {
            if let Some(rule) = data
                .races
                .iter()
                .find_map(|(id, rule)| rule_id.eq_ignore_ascii_case(id).then_some(rule))
            {
                spells.insert(spell_id.to_ascii_lowercase(), (rule, spell_ids.clone()));
            }
        }
    }

    let rule_never = SpellRule::default();
    for id in data.blacklist {
        spells.insert(id.to_ascii_lowercase(), (&rule_never, vec![]));
    }
    let rule_never_sold = SpellRule {
        prefix: _Option(Option::None),
        race: _Option(Option::None),
        vendor: _Option(Option::Some(false)),
    };
    for id in data.vendor_blacklist {
        spells.insert(id.to_ascii_lowercase(), (&rule_never_sold, vec![]));
    }

    let len = spells.len();
    let keys = spells.keys();
    let rules = spells.values().map(|(k, _)| k);
    let alternatives = spells.values().map(|(_, v)| v);

    quote! {
        {
            let mut map = std::collections::HashMap::with_capacity(#len);
            #(
                map.insert(
                    #keys,
                    (
                        #rules,
                        vec![ #( #alternatives, )* ]
                    )
                );
            )*
            map
        }
    }
    .into_token_stream()
}
