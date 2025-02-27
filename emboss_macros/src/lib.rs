mod codegen;
mod macro_impl;

use proc_macro::TokenStream;

#[proc_macro]
pub fn emboss(input: TokenStream) -> TokenStream {
    macro_impl::emboss(input)
}

#[proc_macro]
pub fn emboss_many(input: TokenStream) -> TokenStream {
    macro_impl::emboss_many(input)
}

#[proc_macro]
pub fn emboss_env(input: TokenStream) -> TokenStream {
    macro_impl::emboss_env(input)
}

#[proc_macro]
pub fn emboss_envs(input: TokenStream) -> TokenStream {
    macro_impl::emboss_envs(input)
}
