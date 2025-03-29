mod const_decl;
mod exported;
mod ident;
mod packed;
mod static_value;

use emboss_common::EmbossingOptions;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;

use crate::macro_impl::Embossable;

pub(crate) fn emboss_token(item: Embossable, options: EmbossingOptions) -> TokenStream2 {
    emboss_token_multiple(vec![item], options)
}

pub(crate) fn emboss_token_multiple(
    items: Vec<Embossable>,
    options: EmbossingOptions,
) -> TokenStream2 {
    let const_decl = const_decl::emit(&items, &options);
    let packed_struct = packed::emit(&items, &options);
    let static_init = static_value::emit(&items, &options);

    let export_name = &options.export_name;
    let public_api = if options.export_name.is_some() {
        exported::emit(&items, &options)
    } else {
        quote! {}
    };

    let (top_level_fragment, terminator) = if let Some(export_name) = export_name {
        let export_name = Ident::new(export_name, Span::call_site());
        (quote! { mod #export_name }, quote! {})
    } else {
        (quote! { const _: () = }, quote! { ; })
    };

    quote! {
        #top_level_fragment {
            #const_decl

            #packed_struct

            #public_api

            #static_init
        }#terminator
    }
}
