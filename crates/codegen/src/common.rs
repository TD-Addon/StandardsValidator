use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use serde::Deserialize;

// Transparent wrappers that implement `ToTokens` for common types .

#[derive(Default, Deserialize)]
#[serde(transparent)]
pub struct _Option<T>(pub Option<T>);

#[derive(Deserialize)]
#[serde(transparent)]
pub struct _Vec<T>(pub Vec<T>);

impl<T> ToTokens for _Option<T>
where
    T: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.0 {
            Some(inner) => quote! { Some(#inner) },
            None => quote! { None },
        }
        .to_tokens(tokens)
    }
}

impl<T> ToTokens for _Vec<T>
where
    T: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let inner = &self.0;
        quote! {
            vec![ #( #inner, )* ]
        }
        .to_tokens(tokens)
    }
}
