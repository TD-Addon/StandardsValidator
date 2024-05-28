use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Services {
    barter: Vec<String>,
}

pub fn generate_barter_classes() -> TokenStream {
    let data: Services = serde_json::from_str(include_str!("../data/services.json")).unwrap();

    let len = data.barter.len();
    let values = data.barter.iter().map(|id| id.to_ascii_lowercase());

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
