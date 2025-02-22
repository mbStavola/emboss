use super::EnvVarFallback;

use emboss_common::EmbossingOptions;
use proc_macro::TokenStream;
use quote::quote;
use serde::Deserialize;

use crate::emboss_token;

#[derive(Deserialize)]
struct EnvVarEmbossing {
    env_var: String,

    key: Option<String>,
    
    #[serde(default)]
    fallback: EnvVarFallback,

    #[serde(flatten)]
    options: EmbossingOptions,
}


pub fn emboss_env(input: TokenStream) -> TokenStream {
    let EnvVarEmbossing {
        env_var,
        key,
        fallback,
        options,
    } = match serde_tokenstream::from_tokenstream::<EnvVarEmbossing>(&input.into()) {
        Ok(val) => val,
        Err(err) => return err.to_compile_error().into(),
    };

    let value = match fallback {
        EnvVarFallback::Fail => quote! { env!(#env_var) },
        EnvVarFallback::Empty => quote! {{ 
            match option_env!(#env_var) { 
                Some(val) => val, 
                None => "",
            } 
        }},
        EnvVarFallback::Value(fallback) => quote! {{ 
            match option_env!(#env_var) { 
                Some(val) => val, 
                None => #fallback,
            } 
        }},
    };

    let key = key.unwrap_or_else(|| env_var.clone());
    emboss_token(&key, value, options).into()
}
