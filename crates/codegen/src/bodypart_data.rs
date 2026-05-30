use std::collections::HashMap;
use std::hash::Hash;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use serde::Deserialize;

use crate::common::{_Option, _Vec};

#[derive(Deserialize)]
#[serde(transparent)]
struct _HashMap<K: Eq + Hash, V>(HashMap<K, V>);

#[derive(Deserialize)]
struct NotRule {
    not: _Vec<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Rule {
    In(_Vec<String>),
    Equality(String),
    NotIn(Box<NotRule>),
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum FieldRule {
    Id(Rule),
    Class(Rule),
    Faction(Rule),
    Mode(Rule),
}

#[derive(Deserialize)]
struct BodyPartDefinition {
    model: String,
    ruleset: _Option<String>,
    rules: _Option<_Vec<_Vec<FieldRule>>>,
}

#[derive(Deserialize)]
struct ItemDefinition {
    id: String,
    ruleset: _Option<String>,
    rules: _Option<_Vec<_Vec<FieldRule>>>,
}

#[derive(Deserialize)]
struct BodyPartData {
    rulesets: _HashMap<String, _Vec<_Vec<FieldRule>>>,
    head: _Vec<BodyPartDefinition>,
    hair: _Vec<BodyPartDefinition>,
    items: _Vec<ItemDefinition>,
}

impl<K, V> ToTokens for _HashMap<K, V>
where
    K: ToTokens + Hash + Eq,
    V: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let keys = self.0.keys();
        let values = self.0.values();
        quote! {
            vec![ #( (#keys, #values), )* ]
        }
        .to_tokens(tokens);
    }
}

impl ToTokens for Rule {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Rule::In(rules) => quote! {
                Rule::In(#rules)
            },
            Rule::NotIn(rule) => {
                let rule = &rule.not;
                quote! {
                    Rule::NotIn(#rule)
                }
            }
            Rule::Equality(id) => {
                let id = id.to_ascii_lowercase();
                quote! {
                    Rule::Equality(#id)
                }
            }
        }
        .to_tokens(tokens)
    }
}

impl ToTokens for FieldRule {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            FieldRule::Class(rule) => quote! {
                FieldRule::Class(#rule)
            },
            FieldRule::Faction(rule) => quote! {
                FieldRule::Faction(#rule)
            },
            FieldRule::Id(rule) => quote! {
                FieldRule::Id(#rule)
            },
            FieldRule::Mode(rule) => quote! {
                FieldRule::Mode(#rule)
            },
        }
        .to_tokens(tokens)
    }
}

impl ToTokens for BodyPartDefinition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let model = self.model.to_ascii_lowercase();
        let ruleset = &self.ruleset;
        let rules = &self.rules;
        quote! {
            (
                #model,
                #ruleset,
                #rules,
            )
        }
        .to_tokens(tokens)
    }
}

impl ToTokens for ItemDefinition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = self.id.to_ascii_lowercase();
        let ruleset = &self.ruleset;
        let rules = &self.rules;
        quote! {
            (
                #id,
                #ruleset,
                #rules,
            )
        }
        .to_tokens(tokens)
    }
}

pub fn generate() -> TokenStream {
    let data = serde_json::from_str(include_str!("../data/bodyparts.json")).unwrap();

    let BodyPartData {
        rulesets,
        head,
        hair,
        items,
    } = data;

    quote! {
        (
            #rulesets,
            #head,
            #hair,
            #items,
        )
    }
    .into_token_stream()
}
