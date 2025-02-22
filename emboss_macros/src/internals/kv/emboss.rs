use emboss_common::EmbossingOptions;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use serde::Deserialize;

use crate::emboss_token;

#[derive(Deserialize)]
struct SingleKeyValueEmbossing {
    key: String,

    value: String,

    #[serde(flatten)]
    options: EmbossingOptions,
}

pub fn emboss(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    let SingleKeyValueEmbossing {
        key,
        value,
        options,
    } = match serde_tokenstream::from_tokenstream::<SingleKeyValueEmbossing>(&input) {
        Ok(val) => val,
        Err(err) => return err.to_compile_error().into(),
    };

    emboss_token(&key, quote! { #value }, options).into()
}
