use emboss_common::EmbossingOptions;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use serde::Deserialize;

use crate::{codegen::emboss_token, macro_impl::Embossable};

#[derive(Deserialize)]
struct SingleKeyValueEmbossing {
    key: String,

    value: String,

    variant_name: Option<String>,

    #[serde(flatten)]
    options: EmbossingOptions,
}

pub(crate) fn emboss(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    let SingleKeyValueEmbossing {
        key,
        value,
        variant_name,
        options,
    } = match serde_tokenstream::from_tokenstream::<SingleKeyValueEmbossing>(&input) {
        Ok(val) => val,
        Err(err) => return err.to_compile_error().into(),
    };

    emboss_token(
        Embossable {
            key,
            value: quote! { #value },
            variant_name,
        },
        options,
    )
    .into()
}
