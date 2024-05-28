use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub fn generate() -> TokenStream {
    let data = include_str!("../data/uniques.txt");

    let uniques: HashSet<_> = data
        .lines()
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(str::to_ascii_lowercase)
        .collect();

    let len = uniques.len();
    let values = uniques.iter();

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
