use std::{
    collections::HashSet,
    sync::{LazyLock, Mutex},
};

use emboss_common::{EmbossingOptions, LEADING_MAGIC_BYTES};
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;

use crate::{codegen::ident, macro_impl::Embossable};

static SECTION_SET: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

pub(crate) fn emit(items: &[Embossable], options: &EmbossingOptions) -> TokenStream2 {
    let EmbossingOptions {
        stored_in,
        #[cfg(target_os = "macos")]
        segment,
        ..
    } = options;

    if items.is_empty() {
        return quote! {
            const _: () = compile_error!("No data will be embossed.");
        };
    } else if items.len() > u8::MAX as usize {
        return quote! {
            const _: () = compile_error!("Too many items to emboss, consider breaking up the data into multiple segments/sections.");
        };
    }

    #[cfg(target_os = "macos")]
    let stored_in = format!("{},{}", segment, stored_in);
    {
        let mut section_set = SECTION_SET
            .lock()
            .expect("should be able to acquire section set lock");
        if section_set.contains(&stored_in) {
            let error_msg = format!(
                "There is already embossed data stored in '{}'. You probably meant to call this macro with a different segment/section.",
                stored_in
            );
            return quote! {
                const _: () = compile_error!(#error_msg);
            };
        }
        section_set.insert(stored_in.clone());
    }

    let items_len = items.len();
    let items_init_expr = items
        .iter()
        .enumerate()
        .map(|(index, _)| {
            let key_name = ident::key_const(index);
            let key_size_name = ident::key_size_const(index);

            let value_name = ident::value_const(index);
            let value_size_name = ident::value_size_const(index);

            let key_var_name = Ident::new(&format!("key_{}", index), Span::call_site());
            let value_var_name = Ident::new(&format!("value_{}", index), Span::call_site());

            quote! {
                let mut #key_var_name = [0u8; #key_size_name + 1];
                copy_to_array(#key_name.as_bytes(), &mut #key_var_name, #key_size_name);

                let mut #value_var_name = [0u8; #value_size_name + 1];
                copy_to_array(#value_name.as_bytes(), &mut #value_var_name, #value_size_name);
            }
        })
        .collect::<Vec<_>>();

    let items_field_init_expr = items
        .iter()
        .enumerate()
        .map(|(index, _)| {
            let key_name = Ident::new(&format!("key_{}", index), Span::call_site());
            let value_name = Ident::new(&format!("value_{}", index), Span::call_site());

            quote! {
                #key_name,
                #value_name,
            }
        })
        .collect::<Vec<_>>();

    quote! {
        #[used]
        #[unsafe(link_section = #stored_in)]
        pub static EMBOSSED: Embossed = {
            const fn copy_to_array(src: &[u8], dst: &mut [u8], len: usize) {
                unsafe {
                    let backing_ptr = dst.as_mut_ptr();
                    src.as_ptr().copy_to(backing_ptr, len);
                    *backing_ptr.add(len) = '\0' as u8;
                }
            }

            #(#items_init_expr)*

            Embossed {
                leading: #LEADING_MAGIC_BYTES,
                field_count: #items_len as u8,
                #(#items_field_init_expr)*
            }
        };
    }
}
