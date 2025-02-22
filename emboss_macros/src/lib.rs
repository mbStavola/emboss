mod internals;

use emboss_common::EmbossingOptions;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[proc_macro]
pub fn emboss(input: TokenStream) -> TokenStream {
    internals::emboss(input)
}

#[proc_macro]
pub fn emboss_many(input: TokenStream) -> TokenStream {
    internals::emboss_many(input)
}

#[proc_macro]
pub fn emboss_env(input: TokenStream) -> TokenStream {
    internals::emboss_env(input)
}

#[proc_macro]
pub fn emboss_envs(input: TokenStream) -> TokenStream {
    internals::emboss_envs(input)
}

pub(crate) fn emboss_token(
    key: &str,
    value: TokenStream2,
    options: EmbossingOptions,
) -> TokenStream2 {
    let EmbossingOptions {
        stored_in,
        separator,
        terminator,
        #[cfg(target_os = "macos")]
        segment,
    } = options;

    #[cfg(target_os = "macos")]
    let stored_in = format!("{},{}", segment, stored_in);

    // Some interesting things going on in this macro! See:
    //  On Transmuting: https://github.com/rust-lang/rust/issues/70239
    //  Disabling the transmute lint: https://rust-lang.github.io/rust-clippy/master/index.html#transmute_ptr_to_ref
    quote! {
        const _: () = {
            const KEY: &str = #key;
            const SEPARATOR: char = #separator;
            const VALUE: &str = #value;
            const TERMINATOR: char = #terminator;

            const KEY_SIZE: usize = KEY.as_bytes().len();
            const SEPARATOR_SIZE: usize = SEPARATOR.len_utf8();
            const VALUE_SIZE: usize = VALUE.as_bytes().len();
            const TERMINATOR_SIZE: usize = TERMINATOR.len_utf8();

            const DATA_LEN: usize = KEY_SIZE + SEPARATOR_SIZE + VALUE_SIZE + TERMINATOR_SIZE;

            type Data = [u8; DATA_LEN];

            const RAW_DATA: Data = {
                let mut backing_array: Data = [0u8; DATA_LEN];

                unsafe {
                    let mut backing_array_ptr = backing_array.as_mut_ptr();

                    KEY.as_bytes().as_ptr().copy_to(backing_array_ptr, KEY_SIZE);
                    backing_array_ptr = backing_array_ptr.add(KEY_SIZE);

                    SEPARATOR.encode_utf8(
                        ::core::slice::from_raw_parts_mut(backing_array_ptr, SEPARATOR_SIZE)
                    );
                    backing_array_ptr = backing_array_ptr.add(SEPARATOR_SIZE);

                    VALUE.as_bytes().as_ptr().copy_to(backing_array_ptr, VALUE_SIZE);
                    backing_array_ptr = backing_array_ptr.add(VALUE_SIZE);

                    TERMINATOR.encode_utf8(
                        ::core::slice::from_raw_parts_mut(backing_array_ptr, TERMINATOR_SIZE)
                    );
                    backing_array_ptr = backing_array_ptr.add(TERMINATOR_SIZE);
                }

                backing_array
            };

            #[used]
            #[unsafe(link_section = #stored_in)]
            static EMBOSSED: Data = RAW_DATA;
        };
    }
}
