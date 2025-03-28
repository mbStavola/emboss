use emboss_common::EmbossingOptions;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use serde::Deserialize;

use crate::{codegen::emboss_token_multiple, macro_impl::Embossable};

#[derive(Deserialize)]
struct MultipleKeyValueEmbossing {
    items: Vec<KeyValueSpec>,

    #[serde(flatten)]
    options: EmbossingOptions,
}

#[derive(Deserialize)]
struct KeyValueSpec {
    key: String,

    value: String,

    variant_name: Option<String>,
}

pub(crate) fn emboss_many(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    let MultipleKeyValueEmbossing { items: p, options } =
        match serde_tokenstream::from_tokenstream::<MultipleKeyValueEmbossing>(&input) {
            Ok(val) => val,
            Err(err) => return err.to_compile_error().into(),
        };

    let mut pairs = Vec::with_capacity(p.len());
    for KeyValueSpec {
        key,
        value,
        variant_name,
    } in p
    {
        pairs.push(Embossable {
            key,
            value: quote! { #value },
            variant_name,
        });
    }

    emboss_token_multiple(pairs, options).into()
}
