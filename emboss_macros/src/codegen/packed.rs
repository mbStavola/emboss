use emboss_common::EmbossingOptions;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;

use crate::{codegen::ident, macro_impl::Embossable};

pub(crate) fn emit(items: &[Embossable], _: &EmbossingOptions) -> TokenStream2 {
    let fields = items
        .iter()
        .enumerate()
        .map(|(index, _)| {
            let key_size_name = ident::key_size_const(index);
            let value_size_name = ident::value_size_const(index);

            let key_name = Ident::new(&format!("key_{}", index), Span::call_site());
            let value_name = Ident::new(&format!("value_{}", index), Span::call_site());

            quote! {
                #key_name: [u8; #key_size_name + 1],
                #value_name: [u8; #value_size_name + 1],
            }
        })
        .collect::<Vec<_>>();

    quote! {
        #[repr(C, packed)]
        pub struct Embossed {
            leading: u32,
            field_count: u8,
            #(#fields)*
        }
    }
}
