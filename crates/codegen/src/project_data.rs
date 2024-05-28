use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use serde::Deserialize;

use crate::common::{_Option, _Vec};

#[derive(Deserialize)]
pub struct Project {
    pub name: String,
    pub prefix: String,
    pub local: _Option<String>,
}

impl ToTokens for Project {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let prefix = &self.prefix;
        let local = &self.local;
        quote! {
            Project {
                name: #name,
                prefix: #prefix,
                local: #local,
            }
        }
        .to_tokens(tokens);
    }
}

pub fn generate() -> TokenStream {
    let data: _Vec<Project> = serde_json::from_str(include_str!("../data/projects.json")).unwrap();
    data.into_token_stream()
}
