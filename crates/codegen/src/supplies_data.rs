use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

type SuppliesData = HashMap<String, String>;

pub fn generate() -> TokenStream {
    let data: SuppliesData = serde_json::from_str(include_str!("../data/supplies.json")).unwrap();

    let len = data.len();
    let keys = data.keys().map(|key| key.to_ascii_lowercase());
    let values = data.values();

    quote! {
        {
            let mut map = std::collections::HashMap::with_capacity(#len);
            #(
                map.insert(#keys, #values);
            )*
            map
        }
    }
    .into_token_stream()
}
