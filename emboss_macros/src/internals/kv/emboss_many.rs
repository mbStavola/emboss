use emboss_common::EmbossingOptions;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use serde::Deserialize;

use crate::emboss_token;

#[derive(Deserialize)]
struct MultipleKeyValueEmbossing {
    pairs: Vec<(String, String)>,

    #[serde(flatten)]
    options: EmbossingOptions,
}

pub fn emboss_many(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    let MultipleKeyValueEmbossing { pairs, options } =
        match serde_tokenstream::from_tokenstream::<MultipleKeyValueEmbossing>(&input) {
            Ok(val) => val,
            Err(err) => return err.to_compile_error().into(),
        };

    let mut blocks = Vec::with_capacity(pairs.len());
    for (key, value) in pairs {
        let tokens = emboss_token(&key, quote! { #value }, options.clone());
        blocks.push(tokens);
    }

    quote! {
        #(#blocks)*
    }.into()
}
