use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

type TravelData = HashSet<String>;

pub fn generate() -> TokenStream {
    let data: TravelData = serde_json::from_str(include_str!("../data/travel.json")).unwrap();

    let len = data.len();
    let values = data.iter().map(|id| id.to_ascii_lowercase());

    quote! {
        {
            let mut set = std::collections::HashSet::with_capacity(#len);
            #(
                set.insert(#values);
            )*
            set
        }
    }
    .into_token_stream()
}
