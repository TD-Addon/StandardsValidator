use proc_macro2::TokenStream;
use quote::ToTokens;
use regex::Regex;

pub fn generate_joined_commands() -> TokenStream {
    let data = include_str!("../data/mwscript.returning.txt");

    let commands = data //
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("|");

    commands.into_token_stream()
}

pub fn generate_khajiit_script() -> TokenStream {
    let khajiit_input = include_str!("../data/khajiit.mwscript")
        .replace('(', r"\(")
        .replace(')', r"\)")
        .replace('\n', r"\s*((;.*)?\n)+\s*");

    let khajiit_input = Regex::new(r"\s+")
        .unwrap()
        .replace_all(&khajiit_input, r"\s+")
        .to_string();

    khajiit_input.into_token_stream()
}
