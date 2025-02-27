mod env;
mod kv;

pub(crate) use env::*;
pub(crate) use kv::*;
use proc_macro2::TokenStream as TokenStream2;

pub(crate) struct Embossable {
    pub key: String,
    pub value: TokenStream2,
    pub variant_name: Option<String>,
}
