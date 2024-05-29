use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use serde::Deserialize;

#[derive(Deserialize)]
struct ClassData {
    vanilla: String,
    data: String,
}

pub fn generate() -> TokenStream {
    let data: Vec<ClassData> = serde_json::from_str(include_str!("../data/classes.json")).unwrap();

    let mut tr_classes = HashMap::new();
    let mut classes = HashMap::new();

    for class in &data {
        let lower = class.vanilla.to_ascii_lowercase();
        if lower != "miner" {
            tr_classes.insert(class.data.to_ascii_lowercase(), &class.vanilla);
        }
        classes.insert(lower, &class.data);
    }

    let tr_len = tr_classes.len();
    let tr_keys = tr_classes.keys();
    let tr_values = tr_classes.values();

    let len = classes.len();
    let keys = classes.keys();
    let values = classes.values();

    quote! {
        {
            let mut tr_classes = std::collections::HashMap::with_capacity(#tr_len);
            #(
                tr_classes.insert(#tr_keys, #tr_values);
            )*
            let mut classes = std::collections::HashMap::with_capacity(#len);
            #(
                classes.insert(#keys, #values);
            )*
            (tr_classes, classes)
        }
    }
    .into_token_stream()
}
