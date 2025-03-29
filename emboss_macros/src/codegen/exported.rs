use emboss_common::EmbossingOptions;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::{codegen::ident, macro_impl::Embossable};

pub(crate) fn emit(items: &[Embossable], _: &EmbossingOptions) -> TokenStream2 {
    let enum_variant_expr = items
        .iter()
        .filter_map(
            |Embossable {
                 key, variant_name, ..
             }| {
                let enum_variant_name = ident::enum_variant(key, variant_name.as_ref())?;

                Some(quote! {
                    #enum_variant_name,
                })
            },
        )
        .collect::<Vec<_>>();

    let enum_decl = if !enum_variant_expr.is_empty() {
        quote! {
            #[repr(u8)]
            pub enum EmbossedKeyKind {
                #(#enum_variant_expr)*
            }
        }
    } else {
        quote! {}
    };

    let items_match_enum_expr = items
        .iter()
        .enumerate()
        .filter_map(
            |(
                index,
                Embossable {
                    key, variant_name, ..
                },
            )| {
                let enum_variant_name = ident::enum_variant(key, variant_name.as_ref())
                    .map(|variant_name| quote! { EmbossedKeyKind::#variant_name })?;
                let branch_body = get_branch_impl(index);

                Some(quote! {
                    #enum_variant_name => #branch_body,
                })
            },
        )
        .collect::<Vec<_>>();

    let get_enum_fn = if !items_match_enum_expr.is_empty() {
        quote! {
            pub fn get_by_kind(&self, kind: EmbossedKeyKind) -> (&str, &str) {
                match kind {
                    #(#items_match_enum_expr)*
                }
            }
        }
    } else {
        quote! {}
    };

    let items_match_index_expr = items
        .iter()
        .enumerate()
        .map(|(index, _)| {
            let branch_body = get_branch_impl(index);
            quote! {
                #index => #branch_body,
            }
        })
        .collect::<Vec<_>>();

    let items_match_name_expr = items
        .iter()
        .enumerate()
        .map(|(index, _)| {
            let key_name = ident::key_const(index);
            let branch_body = get_branch_impl(index);

            quote! {
                #key_name => #branch_body,
            }
        })
        .collect::<Vec<_>>();

    quote! {
        impl Embossed {
            pub fn get_by_index(&self, index: usize) -> Option<(&str, &str)> {
                let pair = match index {
                    #(#items_match_index_expr)*
                    _ => return None,
                };

                Some(pair)
            }

            pub fn get_by_key(&self, key: &str) -> Option<(&str, &str)> {
                let pair = match key {
                    #(#items_match_name_expr)*
                    _ => return None,
                };

                Some(pair)
            }

            #get_enum_fn
        }

        #enum_decl
    }
}

fn get_branch_impl(index: usize) -> TokenStream2 {
    let key_var_name = ident::key_field(index);
    let value_var_name = ident::value_field(index);

    quote! {
        unsafe {
            let key = core::str::from_utf8_unchecked(
                &self.#key_var_name[..self.#key_var_name.len() - 1]
            );
            let value = core::str::from_utf8_unchecked(
                &self.#value_var_name[..self.#value_var_name.len() - 1]
            );
            (key, value)
        }
    }
}
