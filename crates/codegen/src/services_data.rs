use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Services {
    barter: Vec<String>,
    spells: Vec<String>,
}

fn get_class_tokens(classes: Vec<String>) -> TokenStream {
    let len = classes.len();
    let values = classes.iter().map(|id| id.to_ascii_lowercase());

    quote! {
        {
            let mut set = std::collections::HashSet::with_capacity(#len);
            #(
                set.insert(String::from(#values));
            )*
            set
        }
    }
    .into_token_stream()
}

pub fn generate_barter_classes() -> TokenStream {
    let data: Services = serde_json::from_str(include_str!("../data/services.json")).unwrap();
    get_class_tokens(data.barter)
}

pub fn generate_spell_vendor_classes() -> TokenStream {
    let data: Services = serde_json::from_str(include_str!("../data/services.json")).unwrap();
    get_class_tokens(data.spells)
}
