use emboss_common::EmbossingOptions;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::{codegen::ident, macro_impl::Embossable};

pub(crate) fn emit(items: &[Embossable], _: &EmbossingOptions) -> TokenStream2 {
    let items_consts_expr = items
        .iter()
        .enumerate()
        .map(|(index, Embossable { key, value, .. })| {
            let key_name = ident::key_const(index);
            let key_size_name = ident::key_size_const(index);

            let value_name = ident::value_const(index);
            let value_size_name = ident::value_size_const(index);

            quote! {
                const #key_name: &str = #key;
                const #value_name: &str = #value;

                const #key_size_name: usize = #key_name.as_bytes().len();
                const #value_size_name: usize = #value_name.as_bytes().len();
            }
        })
        .collect::<Vec<_>>();

    quote! {
        #(#items_consts_expr)*
    }
}
